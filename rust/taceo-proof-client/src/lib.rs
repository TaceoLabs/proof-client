use base64ct::{Base64, Encoding};
use crypto_box::PublicKey;
use ed25519_dalek::{Digest, Sha512, Signature, VerifyingKey};
use eyre::Context;
use taceo_proof_api_client::apis::{configuration::Configuration, node_api};
use uuid::Uuid;

pub use ark_ec;
pub use chrono;
pub use circom_types;
pub use ed25519_dalek;
pub use taceo_proof_api_client::{apis, models};
pub use uuid;
pub use websocket::StopStrategy;

pub mod co_circom;
pub mod co_noir;
mod websocket;

/// A collection of three `NodeProvider`s used to schedule jobs.
///
/// The `NodeProviders` struct is used to represent a group of three node providers
/// in a Multi-Party Computation (MPC) system.
#[derive(Debug, Clone)]
pub struct NodeProviders {
    /// The first node provider.
    pub node0: NodeProvider,
    /// The second node provider.
    pub node1: NodeProvider,
    /// The third node provider.
    pub node2: NodeProvider,
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

/// Get a specific node provider by its id.
pub async fn get_node_provider(config: &Configuration, id: i32) -> eyre::Result<NodeProvider> {
    let node = node_api::node_provider(config, id).await?;
    node.try_into()
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
