use std::fmt;

use ark_ec::pairing::Pairing;
use base64ct::{Base64, Encoding};
use circom_types::{
    Witness,
    traits::{CircomArkworksPairingBridge, CircomArkworksPrimeFieldBridge},
};
use co_circom_types::{
    CompressedRep3SharedWitness, Compression, Input, ShamirSharedWitness, split_input,
};
use crypto_box::{PublicKey, aead::OsRng};
use ed25519_dalek::{Digest, Sha512, Signature, VerifyingKey};
use eyre::Context;
use futures::{SinkExt as _, StreamExt as _};
use intmap::IntMap;
use serde::{Deserialize, Serialize};
use taceo_proof_api_client::{
    apis::{configuration::Configuration, job_api, node_api},
    models::JobType,
};
use tokio_tungstenite::tungstenite::{self, Message};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct NodeProviders {
    node0: NodeProvider,
    node1: NodeProvider,
    node2: NodeProvider,
}

impl TryFrom<taceo_proof_api_client::models::NodeProviders> for NodeProviders {
    type Error = eyre::Report;
    fn try_from(value: taceo_proof_api_client::models::NodeProviders) -> Result<Self, Self::Error> {
        Ok(Self {
            node0: (*value.node0).try_into()?,
            node1: (*value.node1).try_into()?,
            node2: (*value.node2).try_into()?,
        })
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NodeProvider {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) enc_key: PublicKey,
    pub(crate) verify_key: VerifyingKey,
    pub(crate) online: bool,
}

impl TryFrom<taceo_proof_api_client::models::NodeProvider> for NodeProvider {
    type Error = eyre::Report;
    fn try_from(value: taceo_proof_api_client::models::NodeProvider) -> Result<Self, Self::Error> {
        Ok(NodeProvider {
            id: value.id,
            name: value.name,
            enc_key: PublicKey::from_bytes(
                Base64::decode_vec(&value.enc_key)
                    .context("invalid base64")?
                    .try_into()
                    .map_err(|_| eyre::eyre!("wrong len for PublicKey"))?,
            ),
            verify_key: VerifyingKey::from_bytes(
                &Base64::decode_vec(&value.verify_key)
                    .context("invalid base64")?
                    .try_into()
                    .map_err(|_| eyre::eyre!("wrong len for VerifyingKey"))?,
            )
            .context("failed to parse VerifyingKey")?,
            online: value.online,
        })
    }
}

/// Get 3 random nodes that can be used to run a job.
pub async fn get_random_node_providers(config: &Configuration) -> eyre::Result<NodeProviders> {
    let nodes = node_api::random_node_providers(config).await?;
    nodes.try_into()
}

/// Verify the signature of a proof result.
pub fn verify_proof_result_signature(
    job_id: Uuid,
    proof: &str,
    public_inputs: &str,
    signature: Signature,
    vk: VerifyingKey,
) -> eyre::Result<()> {
    tracing::debug!("verify result for job {job_id}");

    let mut digest = Sha512::new();
    digest.update(job_id.as_bytes());
    digest.update(proof);
    digest.update(public_inputs);

    vk.verify_prehashed_strict(
        digest,
        Some("taceo-proof-nps-reporting".as_bytes()),
        &signature,
    )
    .context("while verifying signature")?;
    tracing::debug!("signature ok for job {job_id}");

    Ok(())
}

fn seal_shares(nodes: &NodeProviders, shares: [Vec<u8>; 3]) -> eyre::Result<[Vec<u8>; 3]> {
    tracing::debug!("sealing shares...");
    let ct0 = nodes
        .node0
        .enc_key
        .seal(&mut OsRng, &shares[0])
        .context("while sealing share")?;
    let ct1 = nodes
        .node1
        .enc_key
        .seal(&mut OsRng, &shares[1])
        .context("while sealing share")?;
    let ct2 = nodes
        .node2
        .enc_key
        .seal(&mut OsRng, &shares[2])
        .context("while sealing share")?;
    Ok([ct0, ct1, ct2])
}

/// Schedule a full REP3 job including witness extension.
pub async fn schedule_full_job_rep3<P>(
    config: &Configuration,
    nodes: &NodeProviders,
    blueprint_id: Uuid,
    voucher: Option<&str>,
    input: Input,
    public_inputs: &[String],
) -> eyre::Result<Uuid>
where
    P: Pairing + CircomArkworksPairingBridge,
    P::ScalarField: CircomArkworksPrimeFieldBridge,
    P::BaseField: CircomArkworksPrimeFieldBridge,
{
    tracing::debug!("schedule_job Rep3Full for blueprint_id {blueprint_id}");
    tracing::debug!("sharing input...");
    let [share0, share1, share2] = split_input::<P::ScalarField>(input, public_inputs)?;
    let shares = [
        bincode::serialize(&share0)?,
        bincode::serialize(&share1)?,
        bincode::serialize(&share2)?,
    ];
    tracing::debug!("sealing shares...");
    let [ct0, ct1, ct2] = seal_shares(nodes, shares)?;
    tracing::debug!("scheduling job...");
    let res = job_api::schedule_job(
        config,
        &blueprint_id.to_string(),
        JobType::Rep3Full,
        nodes.node0.id,
        nodes.node1.id,
        nodes.node2.id,
        ct0,
        ct1,
        ct2,
        voucher,
    )
    .await?;
    let job_id = res.job_id;
    tracing::debug!("job_id = {job_id}");
    Ok(job_id)
}

/// Schedule a REP3 prove job.
pub async fn schedule_prove_job_rep3<P>(
    config: &Configuration,
    nodes: &NodeProviders,
    blueprint_id: Uuid,
    voucher: Option<&str>,
    witness: Witness<P::ScalarField>,
    num_pub_inputs: usize,
) -> eyre::Result<Uuid>
where
    P: Pairing + CircomArkworksPairingBridge,
    P::ScalarField: CircomArkworksPrimeFieldBridge,
    P::BaseField: CircomArkworksPrimeFieldBridge,
{
    tracing::debug!("schedule_job Rep3Prove for blueprint_id {blueprint_id}");
    let mut rng = rand::thread_rng();
    tracing::debug!("sharing witness...");
    let [share0, share1, share2] = CompressedRep3SharedWitness::<P::ScalarField>::share_rep3(
        witness,
        num_pub_inputs,
        &mut rng,
        Compression::SeededHalfShares,
    );
    let shares = [
        bincode::serialize(&share0)?,
        bincode::serialize(&share1)?,
        bincode::serialize(&share2)?,
    ];
    tracing::debug!("sealing shares...");
    let [ct0, ct1, ct2] = seal_shares(nodes, shares)?;
    tracing::debug!("scheduling job...");
    let res = job_api::schedule_job(
        config,
        &blueprint_id.to_string(),
        JobType::Rep3Prove,
        nodes.node0.id,
        nodes.node1.id,
        nodes.node2.id,
        ct0,
        ct1,
        ct2,
        voucher,
    )
    .await?;
    let job_id = res.job_id;
    tracing::debug!("job_id = {job_id}");
    Ok(job_id)
}

/// Schedule a Shamir prove job.
pub async fn schedule_prove_job_shamir<P>(
    config: &Configuration,
    nodes: &NodeProviders,
    blueprint_id: Uuid,
    voucher: Option<&str>,
    witness: Witness<P::ScalarField>,
    num_pub_inputs: usize,
) -> eyre::Result<Uuid>
where
    P: Pairing + CircomArkworksPairingBridge,
    P::ScalarField: CircomArkworksPrimeFieldBridge,
    P::BaseField: CircomArkworksPrimeFieldBridge,
{
    tracing::debug!("schedule_job ShamirProve for blueprint_id {blueprint_id}");
    let mut rng = rand::thread_rng();
    tracing::debug!("sharing witness...");
    tracing::debug!("sharing witness...");
    let [share0, share1, share2] = ShamirSharedWitness::<P::ScalarField>::share_shamir(
        witness,
        num_pub_inputs,
        1,
        3,
        &mut rng,
    )
    .try_into()
    .expect("correct len");
    let shares = [
        bincode::serialize(&share0)?,
        bincode::serialize(&share1)?,
        bincode::serialize(&share2)?,
    ];
    tracing::debug!("sealing shares...");
    let [ct0, ct1, ct2] = seal_shares(nodes, shares)?;
    tracing::debug!("scheduling job...");
    let res = job_api::schedule_job(
        config,
        &blueprint_id.to_string(),
        JobType::ShamirProve,
        nodes.node0.id,
        nodes.node1.id,
        nodes.node2.id,
        ct0,
        ct1,
        ct2,
        voucher,
    )
    .await?;
    let job_id = res.job_id;
    tracing::debug!("job_id = {job_id}");
    Ok(job_id)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Running,
    Finished,
    Cancelled,
}

#[derive(Serialize, Deserialize)]
pub struct SignedResults {
    pub signatures: IntMap<i32, String>,
    pub proof: String,
    pub public_inputs: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FailedReason {
    pub node_provider: i32,
    pub error: String,
    pub signature: String,
}

impl fmt::Debug for SignedResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node_providers = self
            .signatures
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<_>>();
        f.write_fmt(format_args!("Result from: {node_providers:?}"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WebSocketMessage {
    Success(SignedResults),
    Update(JobStatus),
    Failed(FailedReason),
    Cancelled,
    Err(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum StopStrategy {
    #[default]
    First,
    Majority,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeExecutionRequest {
    pub execution_id: Uuid,
    pub stop_on_finished_reports: StopStrategy,
    pub with_status_updates: Option<bool>,
}

pub async fn fetch_job_result(
    url: &str,
    job_id: Uuid,
    stop_strategy: StopStrategy,
) -> eyre::Result<SignedResults> {
    // subscribe with ws to get job updates
    let (mut ws_stream, _) = tokio_tungstenite::connect_async(url)
        .await
        .context("while connecting to endpoint")?;
    ws_stream
        .send(tungstenite::Message::text(
            serde_json::to_string(&SubscribeExecutionRequest {
                execution_id: job_id,
                stop_on_finished_reports: stop_strategy,
                with_status_updates: Some(false),
            })
            .expect("can serialize"),
        ))
        .await
        .context("while sending subscribe request")?;
    if let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let msg: WebSocketMessage =
                    serde_json::from_str(&text).context("invalid data from server")?;
                match msg {
                    WebSocketMessage::Success(signed_results) => Ok(signed_results),
                    WebSocketMessage::Failed(FailedReason {
                        node_provider,
                        error,
                        signature: _,
                    }) => eyre::bail!("node {node_provider} error: {error:?}"),
                    WebSocketMessage::Cancelled => eyre::bail!("job was cancelled"),
                    WebSocketMessage::Err(err) => eyre::bail!(err),
                    WebSocketMessage::Update(_) => eyre::bail!("unexpected update"),
                }
            }
            Ok(Message::Close(_)) => {
                eyre::bail!("server closed stream");
            }
            Ok(_) => {
                eyre::bail!("server sent invalid data");
            }
            Err(err) => Err(err.into()),
        }
    } else {
        eyre::bail!("server closed stream");
    }
}
