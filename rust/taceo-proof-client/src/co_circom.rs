use std::collections::{BTreeMap, HashMap};

use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use base64ct::{Base64, Encoding};
use circom_types::{
    Witness,
    groth16::CircomGroth16Proof,
    traits::{CircomArkworksPairingBridge, CircomArkworksPrimeFieldBridge},
};
use co_circom_types::{
    CompressedRep3SharedWitness, Compression, Input, ShamirSharedWitness, split_input,
};
use ed25519_dalek::Signature;
use eyre::Context;
use rand::rngs::OsRng;
use taceo_proof_api_client::{
    apis::{configuration::Configuration, job_api},
    models::MpcProtocol,
};
use uuid::Uuid;

use crate::{NodeProviders, StopStrategy};

/// Schedule a full REP3 job including witness extension.
///
/// This function schedules a job using the Rep3 Secret Sharing scheme. It takes an input,
/// splits it into shares using Rep3, encrypts the shares, and sends them to the respective
/// nodes for execution. This function does not require an extended witness to be provided
/// beforehand; instead, it computes the witness extension as part of the job execution,
/// followed by the proof generation.
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
/// # use uuid::Uuid;
/// # #[tokio::main]
/// # async fn main() -> eyre::Result<()> {
/// let config = Configuration {
///     base_path: "https://proof.taceo.network".to_string(),
///     ..Default::default()
/// };
/// let nodes = taceo_proof_client::get_random_node_providers(&config).await?;
/// let blueprint_id = Uuid::parse_str("54f9ee38-0160-44e2-a1d8-08d1b6771cbf")?;
/// let voucher = None; // only required for Restricted blueprints
/// let public_inputs = ["public_input_1".to_string(), "public_input_2".to_string()];
/// let input = serde_json::from_reader(std::fs::File::open("input.json")?)?;
/// let job_id = taceo_proof_client::co_circom::schedule_full_job::<ark_bn254::Fr>(
///     &config,
///     &nodes,
///     blueprint_id,
///     voucher,
///     &public_inputs,
///     input,
/// )
/// .await?;
/// # Ok(())
/// # }
/// ```
pub async fn schedule_full_job<F: PrimeField>(
    config: &Configuration,
    nodes: &NodeProviders,
    blueprint_id: Uuid,
    voucher: Option<&str>,
    public_inputs: &[String],
    input: Input,
) -> eyre::Result<Uuid> {
    tracing::debug!("schedule_full_job for blueprint {blueprint_id}");
    let keys = [
        &nodes.node0.enc_key,
        &nodes.node1.enc_key,
        &nodes.node2.enc_key,
    ];
    let [inputs0, inputs1, inputs2] = split_input::<F>(input, public_inputs)?
        .into_iter()
        .zip(keys.iter())
        .map(|(inputs, key)| {
            let sealed_input = inputs
                .into_iter()
                .map(|(name, input)| {
                    let input = bincode::serialize(&input).expect("can serialize");
                    let input = key
                        .seal(&mut OsRng, &input)
                        .context("while sealing share")?;
                    Ok((name, input))
                })
                .collect::<eyre::Result<BTreeMap<String, Vec<u8>>>>()?;
            Ok(bincode::serialize(&sealed_input).expect("can serialize"))
        })
        .collect::<eyre::Result<Vec<_>>>()?
        .try_into()
        .expect("len 3");
    let res = job_api::schedule_full_job(
        config,
        &blueprint_id.to_string(),
        inputs0,
        inputs1,
        inputs2,
        nodes.node0.id,
        nodes.node1.id,
        nodes.node2.id,
        voucher,
    )
    .await?;
    let job_id = res.job_id;
    tracing::debug!("job_id = {job_id}");
    Ok(job_id)
}

/// Schedule a job with multiple inputs and an optional deadline.
///
/// This function schedules a job that allows multiple input submissions. It is designed for
/// use cases where the inputs are not provided upfront but are added incrementally. Optionally,
/// a deadline can be set for the job, which specifies the latest time the job is allowed to run.
///
/// # Arguments
///
/// * `config` - The configuration object for the API client.
/// * `nodes` - A set of node providers that will execute the job.
/// * `blueprint_id` - The unique identifier of the blueprint for the job.
/// * `voucher` - An optional voucher for non-public blueprints.
/// * `deadline` - An optional deadline for the job in the local timezone.
///
/// # Returns
///
/// Returns the id (`Uuid`) of the scheduled job on success.
///
/// # Errors
///
/// Returns an error if:
/// - The job scheduling API call fails.
/// - The `blueprint_id` is invalid.
///
/// # Example
/// ```no_run
/// # use chrono::{Local, Duration};
/// # use taceo_proof_client::apis::configuration::Configuration;
/// # use uuid::Uuid;
/// # #[tokio::main]
/// # async fn main() -> eyre::Result<()> {
/// let config = Configuration {
///     base_path: "https://proof.taceo.network".to_string(),
///     ..Default::default()
/// };
/// let nodes = taceo_proof_client::get_random_node_providers(&config).await?;
/// let blueprint_id = Uuid::parse_str("54f9ee38-0160-44e2-a1d8-08d1b6771cbf")?;
/// let voucher = None; // only required for Restricted blueprints
/// let deadline = Some(Local::now() + Duration::days(1)); // Set a deadline 1 day from now
/// let job_id = taceo_proof_client::co_circom::schedule_full_multiple_inputs_job(
///     &config,
///     &nodes,
///     blueprint_id,
///     voucher,
///     deadline,
/// )
/// .await?;
/// # Ok(())
/// # }
/// ```
pub async fn schedule_full_multiple_inputs_job(
    config: &Configuration,
    nodes: &NodeProviders,
    blueprint_id: Uuid,
    voucher: Option<&str>,
    deadline: Option<chrono::DateTime<chrono::Local>>,
) -> eyre::Result<Uuid> {
    tracing::debug!("schedule_full_multiple_inputs_job for blueprint {blueprint_id}");
    let res = job_api::schedule_full_multiple_inputs_job(
        config,
        &blueprint_id.to_string(),
        nodes.node0.id,
        nodes.node1.id,
        nodes.node2.id,
        deadline.map(|date_time| date_time.to_string()),
        voucher,
    )
    .await?;
    let job_id = res.job_id;
    tracing::debug!("job_id = {job_id}");
    Ok(job_id)
}

/// Add inputs to an existing job.
///
/// This function adds inputs to a job that has already been scheduled. The inputs are split
/// into shares, encrypted, and sent to the respective nodes for processing.
///
/// # Arguments
///
/// * `config` - The configuration object for the API client.
/// * `nodes` - A set of node providers that will process the inputs.
/// * `job_id` - The unique identifier of the job to which inputs will be added.
/// * `public_inputs` - A list of public input names for the blueprint's circuit.
/// * `input` - The input data to be shared and added to the job.
///
/// # Returns
///
/// Returns `Ok(())` on success.
///
/// # Errors
///
/// Returns an error if:
/// - Input sharing fails.
/// - Encryption of shares fails.
/// - The API call to add inputs fails.
///
/// # Example
/// ```no_run
/// # use taceo_proof_client::apis::configuration::Configuration;
/// # use taceo_proof_client::NodeProviders;
/// # use uuid::Uuid;
/// # #[tokio::main]
/// # async fn main() -> eyre::Result<()> {
/// let config = Configuration {
///     base_path: "https://proof.taceo.network".to_string(),
///     ..Default::default()
/// };
/// // fetch the nodes selected when the job was scheduled
/// let (node0, node1, node2) = tokio::join!(
///     taceo_proof_client::get_node_provider(&config, 0),
///     taceo_proof_client::get_node_provider(&config, 1),
///     taceo_proof_client::get_node_provider(&config, 2)
/// );
/// let nodes = NodeProviders {
///     node0: node0?,
///     node1: node1?,
///     node2: node2?,
/// };
/// let job_id = Uuid::parse_str("9c2814d7-25d3-4de5-b61f-0a6e3bacbe99")?;
/// let public_inputs = ["public_input_1".to_string(), "public_input_2".to_string()];
/// let input = serde_json::from_reader(std::fs::File::open("input.json")?)?;
/// taceo_proof_client::co_circom::add_job_inputs::<ark_bn254::Fr>(
///     &config,
///     &nodes,
///     job_id,
///     &public_inputs,
///     input,
/// )
/// .await?;
/// # Ok(())
/// # }
/// ```
pub async fn add_job_inputs<F: PrimeField>(
    config: &Configuration,
    nodes: &NodeProviders,
    job_id: Uuid,
    public_inputs: &[String],
    input: Input,
) -> eyre::Result<()> {
    tracing::debug!("add_job_inputs for job {job_id}");
    let keys = [
        &nodes.node0.enc_key,
        &nodes.node1.enc_key,
        &nodes.node2.enc_key,
    ];
    let [inputs0, inputs1, inputs2] = split_input::<F>(input, public_inputs)?
        .into_iter()
        .zip(keys.iter())
        .map(|(inputs, key)| {
            let sealed_input = inputs
                .into_iter()
                .map(|(name, input)| {
                    let input = bincode::serialize(&input).expect("can serialize");
                    let input = key
                        .seal(&mut OsRng, &input)
                        .context("while sealing share")?;
                    Ok((name, input))
                })
                .collect::<eyre::Result<BTreeMap<String, Vec<u8>>>>()?;
            Ok(bincode::serialize(&sealed_input).expect("can serialize"))
        })
        .collect::<eyre::Result<Vec<_>>>()?
        .try_into()
        .expect("len 3");
    job_api::add_inputs(
        config,
        inputs0,
        inputs1,
        inputs2,
        &job_id.to_string(),
        nodes.node0.id,
        nodes.node1.id,
        nodes.node2.id,
    )
    .await?;
    tracing::debug!("added inputs");
    Ok(())
}

/// Schedule a prove job using a specified MPC protocol.
///
/// This function schedules a proof job using either the Rep3 or Shamir Secret Sharing scheme,
/// depending on the specified `mpc_protocol`. It takes a witness, splits it into shares,
/// encrypts the shares, and sends them to the respective nodes for execution.
///
/// # Arguments
///
/// * `config` - The configuration object for the API client.
/// * `nodes` - A set of node providers that will execute the job.
/// * `blueprint_id` - The unique identifier of the blueprint for the proof job.
/// * `mpc_protocol` - The MPC protocol to use for the proof job (e.g., Rep3 or Shamir).
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
/// # use taceo_proof_client::models::MpcProtocol;
/// # #[tokio::main]
/// # async fn main() -> eyre::Result<()> {
/// let config = Configuration {
///     base_path: "https://proof.taceo.network".to_string(),
///     ..Default::default()
/// };
/// let nodes = taceo_proof_client::get_random_node_providers(&config).await?;
/// let blueprint_id = uuid::Uuid::parse_str("54f9ee38-0160-44e2-a1d8-08d1b6771cbf")?;
/// let mpc_protocol = MpcProtocol::Rep3; // specify the MPC protocol to use (Rep3 or Shamir)
/// let voucher = None; // only required for Restricted blueprints
/// let num_pub_inputs = 2;
/// let witness = circom_types::Witness::from_reader(std::fs::File::open("witness.wtns")?)?;
/// let job_id = taceo_proof_client::co_circom::schedule_prove_job::<ark_bn254::Fr>(
///     &config,
///     &nodes,
///     blueprint_id,
///     mpc_protocol,
///     voucher,
///     num_pub_inputs,
///     witness,
/// )
/// .await?;
/// # Ok(())
/// # }
/// ```
pub async fn schedule_prove_job<F: PrimeField>(
    config: &Configuration,
    nodes: &NodeProviders,
    blueprint_id: Uuid,
    mpc_protocol: MpcProtocol,
    voucher: Option<&str>,
    num_pub_inputs: usize,
    witness: Witness<F>,
) -> eyre::Result<Uuid> {
    tracing::debug!("schedule_prove_job for blueprint {blueprint_id}");
    let mut rng = rand::thread_rng();
    let shares = match mpc_protocol {
        MpcProtocol::Rep3 => CompressedRep3SharedWitness::share_rep3(
            witness,
            num_pub_inputs,
            &mut rng,
            Compression::SeededHalfShares,
        )
        .into_iter()
        .map(|share| bincode::serialize(&share).expect("can serialize"))
        .collect::<Vec<_>>(),
        MpcProtocol::Shamir => {
            ShamirSharedWitness::share_shamir(witness, num_pub_inputs, 1, 3, &mut rng)
                .into_iter()
                .map(|share| bincode::serialize(&share).expect("can serialize"))
                .collect::<Vec<_>>()
        }
    };
    let [witness0, witness1, witness2] = shares
        .into_iter()
        .zip([
            &nodes.node0.enc_key,
            &nodes.node1.enc_key,
            &nodes.node2.enc_key,
        ])
        .map(|(share, key)| key.seal(&mut OsRng, &share).context("while sealing share"))
        .collect::<eyre::Result<Vec<_>>>()?
        .try_into()
        .expect("len is 3");
    let res = job_api::schedule_prove_job(
        config,
        &blueprint_id.to_string(),
        mpc_protocol,
        nodes.node0.id,
        nodes.node1.id,
        nodes.node2.id,
        witness0,
        witness1,
        witness2,
        voucher,
    )
    .await?;
    let job_id = res.job_id;
    tracing::debug!("job_id = {job_id}");
    Ok(job_id)
}

/// Fetches the result of a job execution using a WebSocket connection.
///
/// This function connects to the specified WebSocket `url` and subscribes to updates
/// for the job identified by `job_id`. It listens for messages from the server and
/// processes them based on the provided `stop_strategy`. The function returns the
/// proof, public inputs, and signatures of the job execution if successful, or an error
/// if the job fails or the connection is closed unexpectedly.
///
/// # Arguments
///
/// * `url` - The WebSocket URL to connect to for job updates.
/// * `job_id` - The UUID of the job whose results are being fetched.
/// * `stop_strategy` - The strategy to determine when to stop waiting for job results.
///
/// # Returns
///
/// Returns a tuple containing:
/// - A `CircomGroth16Proof` object representing the proof of the job.
/// - A vector of scalar field elements representing the public inputs.
/// - A `HashMap` mapping node IDs to their respective `Signature`.
///
/// # Errors
///
/// This function returns an error if:
/// - The WebSocket connection fails to establish.
/// - The server sends invalid or unexpected data.
/// - The job fails, is cancelled, or encounters an error on a node.
/// - The server closes the WebSocket connection unexpectedly.
/// - Deserialization of the proof or public inputs fails.
///
/// # Example
///
/// ```no_run
/// # use uuid::Uuid;
/// # use taceo_proof_client::StopStrategy;
/// # use circom_types::groth16::CircomGroth16Proof;
/// # use ark_bn254::Bn254;
/// # #[tokio::main]
/// # async fn main() -> eyre::Result<()> {
/// let ws_url = "wss://proof.taceo.network/api/v1/reports/subs".to_string();
/// let job_id = Uuid::parse_str("9c2814d7-25d3-4de5-b61f-0a6e3bacbe99")?;
/// let (proof, public_inputs, signatures) = taceo_proof_client::co_circom::fetch_job_result::<Bn254>(
///     &ws_url,
///     job_id,
///     StopStrategy::default(),
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub async fn fetch_job_result<P: Pairing + CircomArkworksPairingBridge>(
    url: &str,
    job_id: Uuid,
    stop_strategy: StopStrategy,
) -> eyre::Result<(
    CircomGroth16Proof<P>,
    Vec<P::ScalarField>,
    HashMap<i32, Signature>,
)>
where
    P::BaseField: CircomArkworksPrimeFieldBridge,
    P::ScalarField: CircomArkworksPrimeFieldBridge,
{
    let result = crate::websocket::fetch_job_result(url, job_id, stop_strategy)
        .await
        .context("while fetching job result")?;
    let proof = serde_json::from_str::<CircomGroth16Proof<P>>(&result.proof)?;
    let public_as_strings = serde_json::from_str::<Vec<String>>(&result.public_inputs)?;
    let public = public_as_strings
        .into_iter()
        .map(|s| {
            s.parse::<P::ScalarField>()
                .map_err(|_| eyre::eyre!("could not parse as field element: {}", s))
        })
        .collect::<Result<Vec<P::ScalarField>, _>>()
        .context("while converting public input strings to field elements")?;
    let signatures = result
        .signatures
        .into_iter()
        .map(|(id, b64)| {
            let bytes = Base64::decode_vec(&b64)?;
            let signature = Signature::from_bytes(
                &bytes
                    .try_into()
                    .map_err(|_| eyre::eyre!("invalid signature size"))?,
            );
            Ok((id, signature))
        })
        .collect::<eyre::Result<HashMap<i32, Signature>>>()?;
    Ok((proof, public, signatures))
}
