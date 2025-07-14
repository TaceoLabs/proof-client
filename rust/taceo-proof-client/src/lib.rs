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

pub use ark_ec;
pub use circom_types;
pub use ed25519_dalek;
pub use taceo_proof_api_client::{apis, models};
pub use uuid;

/// A collection of three `NodeProvider`s used to schedule jobs.
///
/// The `NodeProviders` struct is used to represent a group of three node providers
/// in a Multi-Party Computation (MPC) system.
#[derive(Debug, Clone)]
pub struct NodeProviders {
    /// The first node provider.
    node0: NodeProvider,
    /// The second node provider.
    node1: NodeProvider,
    /// The third node provider.
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

/// Represents a node provider in the network.
///
/// A `NodeProvider` contains information about a node, including its unique identifier,
/// name, encryption key, verification key, and online status. This struct is used to
/// interact with and manage the nodes participating in Multi-Party Computation (MPC).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NodeProvider {
    /// A unique identifier for the node provider.
    pub id: i32,
    /// The name of the node provider.
    pub name: String,
    /// The public encryption key of the node provider, used for secure communication.
    pub enc_key: PublicKey,
    /// The verifying key of the node provider, used to verify digital signatures.
    pub verify_key: VerifyingKey,
    /// A boolean indicating whether the node provider is currently online.
    pub online: bool,
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
///
/// This function ensures the integrity and authenticity of a proof result by verifying
/// its signature using the provided verifying key (`vk`). The signature is validated
/// against a prehashed digest that includes the job ID, proof, and public inputs.
///
/// # Arguments
///
/// * `job_id` - The unique identifier of the job.
/// * `proof` - The proof string to be verified.
/// * `public_inputs` - The public inputs associated with the proof.
/// * `signature` - The digital signature to be verified.
/// * `vk` - The verifying key used to validate the signature.
///
/// # Returns
///
/// Returns `Ok(())` if the signature is valid. Otherwise, it returns an error indicating
/// the reason for the failure.
///
/// # Errors
///
/// Returns an error if:
/// - The signature is invalid.
/// - The digest or signature verification process fails.
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
///
/// This function schedules a job using the Rep3 Secret Sharing scheme. It takes an input,
/// splits it into shares using Rep3, encrypts the shares, and sends them to the respective
/// nodes for execution. Instead of providing the extended witness, this job will compute
/// witness extension first and then the proof afterwards.
///
/// # Arguments
///
/// * `config` - The configuration object for the API client.
/// * `nodes` - A set of node providers that will execute the job.
/// * `blueprint_id` - The unique identifier of the blueprint for the job.
/// * `voucher` - An optional voucher for non-public blueprints.
/// * `public_inputs` - A list of public input names for the blueprint's circuit.
/// * `input` - The input data to be shared and used in the witness extension.
///
/// # Returns
///
/// Returns the id (`Uuid`) of the scheduled job on success.
///
/// # Errors
///
/// Returns an error if:
/// - Input sharing fails.
/// - Encryption of shares fails.
/// - The job scheduling API call fails.
///
/// # Example
/// ```no_run
/// # use taceo_proof_client::apis::configuration::Configuration;
/// # #[tokio::main]
/// # async fn main() -> eyre::Result<()> {
/// let config = Configuration {
///     base_path: "https://taceo.proof.network".to_string(),
///     ..Default::default()
/// };
/// let nodes = taceo_proof_client::get_random_node_providers(&config).await?;
/// let input = serde_json::from_reader(std::fs::File::open("input.json")?)?;
/// let blueprint_id = uuid::Uuid::parse_str("54f9ee38-0160-44e2-a1d8-08d1b6771cbf")?;
/// let job_id = taceo_proof_client::schedule_full_job_rep3::<ark_bn254::Bn254>(
///     &config,
///     &nodes,
///     blueprint_id,
///     None,
///     &["a_public_input_name".to_string()],
///     input
/// )
/// .await?;
/// # Ok(())
/// # }
/// ```
pub async fn schedule_full_job_rep3<P>(
    config: &Configuration,
    nodes: &NodeProviders,
    blueprint_id: Uuid,
    voucher: Option<&str>,
    public_inputs: &[String],
    input: Input,
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
///
/// This function schedules a proof job using the Rep3 Secret Sharing scheme.
/// It takes a witness, splits it into shares using Rep3, encrypts the shares,
/// and sends them to the respective nodes for execution.
///
/// # Arguments
///
/// * `config` - The configuration object for the API client.
/// * `nodes` - A set of node providers that will execute the job.
/// * `blueprint_id` - The unique identifier of the blueprint for the proof job.
/// * `voucher` - An optional voucher for non-public blueprints.
/// * `num_pub_inputs` - The number of public inputs for the blueprint's circuit.
/// * `witness` - The witness data to be used in the proof.
///
/// # Returns
///
/// Returns the id (`Uuid`) of the scheduled job on success.
///
/// # Errors
///
/// Returns an error if:
/// - Witness sharing fails.
/// - Encryption of shares fails.
/// - The job scheduling API call fails.
///
/// # Example
/// ```no_run
/// # use taceo_proof_client::apis::configuration::Configuration;
/// # #[tokio::main]
/// # async fn main() -> eyre::Result<()> {
/// let config = Configuration {
///     base_path: "https://taceo.proof.network".to_string(),
///     ..Default::default()
/// };
/// let nodes = taceo_proof_client::get_random_node_providers(&config).await?;
/// let blueprint_id = uuid::Uuid::parse_str("54f9ee38-0160-44e2-a1d8-08d1b6771cbf")?;
/// let witness = circom_types::Witness::from_reader(std::fs::File::open("witness.wtns")?)?;
/// let job_id = taceo_proof_client::schedule_prove_job_rep3::<ark_bn254::Bn254>(
///     &config,
///     &nodes,
///     blueprint_id,
///     None,
///     2, // the number of public inputs
///     witness,
/// )
/// .await?;
/// # Ok(())
/// # }
/// ```
pub async fn schedule_prove_job_rep3<P>(
    config: &Configuration,
    nodes: &NodeProviders,
    blueprint_id: Uuid,
    voucher: Option<&str>,
    num_pub_inputs: usize,
    witness: Witness<P::ScalarField>,
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
///
/// This function schedules a proof job using Shamir Secret Sharing scheme.
/// It takes a witness, splits it into shares using Shamir scheme, encrypts the shares,
/// and sends them to the respective nodes for execution.
///
/// # Arguments
///
/// * `config` - The configuration object for the API client.
/// * `nodes` - A set of node providers that will execute the job.
/// * `blueprint_id` - The unique identifier of the blueprint for the proof job.
/// * `voucher` - An optional voucher for non-public blueprints.
/// * `num_pub_inputs` - The number of public inputs for the blueprint's circuit.
/// * `witness` - The witness data to be used in the proof.
///
/// # Returns
///
/// Returns the id (`Uuid`) of the scheduled job on success.
///
/// # Errors
///
/// Returns an error if:
/// - Witness sharing fails.
/// - Encryption of shares fails.
/// - The job scheduling API call fails.
///
/// # Example
/// ```no_run
/// # use taceo_proof_client::apis::configuration::Configuration;
/// # #[tokio::main]
/// # async fn main() -> eyre::Result<()> {
/// let config = Configuration {
///     base_path: "https://taceo.proof.network".to_string(),
///     ..Default::default()
/// };
/// let nodes = taceo_proof_client::get_random_node_providers(&config).await?;
/// let blueprint_id = uuid::Uuid::parse_str("54f9ee38-0160-44e2-a1d8-08d1b6771cbf")?;
/// let witness = circom_types::Witness::from_reader(std::fs::File::open("witness.wtns")?)?;
/// let job_id = taceo_proof_client::schedule_prove_job_shamir::<ark_bn254::Bn254>(
///     &config,
///     &nodes,
///     blueprint_id,
///     None,
///     2, // the number of public inputs
///     witness,
/// )
/// .await?;
/// # Ok(())
/// # }
/// ```
pub async fn schedule_prove_job_shamir<P>(
    config: &Configuration,
    nodes: &NodeProviders,
    blueprint_id: Uuid,
    voucher: Option<&str>,
    num_pub_inputs: usize,
    witness: Witness<P::ScalarField>,
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
/// The status of a job.
pub enum JobStatus {
    Pending,
    Running,
    Finished,
    Cancelled,
}

#[derive(Serialize, Deserialize)]
/// The result of a job, including signatures of the nodes that handled the job.
pub struct SignedResults {
    /// The signatures of each node used to compute the job (identified by their id).
    pub signatures: IntMap<i32, String>,
    /// The proof as JSON (for CircomGroth16 proofs) or base64 encoded ark_serialized `ark_groth16::Proof` (for LibsnarkGroth16 proofs).
    pub proof: String,
    /// The array of public inputs as JSON (for CircomGroth16 proofs) pr base64 encoded ark_serialized `Vec<P::ScalarField>` (for LibsnarkGroth16 proofs).
    pub public_inputs: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// A error result of a job.
pub struct FailedReason {
    /// The node provider that encountered the error.
    pub node_provider: i32,
    /// The error string.
    pub error: String,
    /// The signature of the error and the node provider.
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
/// The different messages we receive over the ws connection.
pub enum WebSocketMessage {
    Success(SignedResults),
    Update(JobStatus),
    Failed(FailedReason),
    Cancelled,
    Err(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
/// The stop strategy when waiting for the job result.
pub enum StopStrategy {
    #[default]
    /// Stop if one node responded with a result.
    First,
    /// Stop if the majority of nodes responded with a result.
    Majority,
    /// Stop if the all nodes responded with a result.
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The ws subscribe request to wait for a job result.
pub struct SubscribeExecutionRequest {
    pub execution_id: Uuid,
    pub stop_on_finished_reports: StopStrategy,
    pub with_status_updates: Option<bool>,
}

/// Fetches the result of a job execution using a WebSocket connection.
///
/// This function connects to the specified WebSocket `url` and subscribes to updates
/// for the job identified by `job_id`. It listens for messages from the server and
/// processes them based on the provided `stop_strategy`. The function returns the
/// signed results of the job execution if successful, or an error if the job fails
/// or the connection is closed unexpectedly.
///
/// # Arguments
///
/// * `url` - The WebSocket URL to connect to for job updates.
/// * `job_id` - The id of the job whose results are being fetched.
/// * `stop_strategy` - The strategy to determine when to stop waiting for job results.
///
/// # Returns
///
/// Returns a `SignedResults` struct containing the proof and public inputs, along
/// with the signatures from the nodes that handled the job.
///
/// # Errors
///
/// This function returns an error if:
/// - The WebSocket connection fails to establish.
/// - The server sends invalid or unexpected data.
/// - The job fails, is cancelled, or encounters an error on a node.
/// - The server closes the WebSocket connection unexpectedly.
///
/// # Example
///
/// ```no_run
/// # use ark_serialize::CanonicalDeserialize;
/// # use base64ct::Encoding;
/// # #[tokio::main]
/// # async fn main() -> eyre::Result<()> {
/// let ws_url = "wss://taceo.proof.network/api/v1/reports/subs".to_string();
/// let job_id = uuid::Uuid::parse_str("9c2814d7-25d3-4de5-b61f-0a6e3bacbe99")?;
/// let res = taceo_proof_client::fetch_job_result(&ws_url, job_id, taceo_proof_client::StopStrategy::default()).await?;
///
/// // CircomGroth16 proofs compatible with circom
/// std::fs::write("proof.json", &res.proof)?;
/// std::fs::write("public.json", &res.public_inputs)?;
///
/// // LibsnarkGroth16 proofs
/// let proof = ark_groth16::Proof::<ark_bn254::Bn254>::deserialize_compressed(
///     base64ct::Base64::decode_vec(&res.proof)?.as_slice(),
/// );
/// let public_inputs = Vec::<ark_bn254::Fr>::deserialize_compressed(
///     base64ct::Base64::decode_vec(&res.public_inputs)?.as_slice(),
/// );
/// # Ok(())
/// # }
/// ```
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
