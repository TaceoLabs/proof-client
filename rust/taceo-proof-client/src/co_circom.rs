use ark_ec::pairing::Pairing;
use circom_types::{
    Witness,
    traits::{CircomArkworksPairingBridge, CircomArkworksPrimeFieldBridge},
};
use co_circom_types::{
    CompressedRep3SharedWitness, Compression, Input, ShamirSharedWitness, split_input,
};
use taceo_proof_api_client::{
    apis::{configuration::Configuration, job_api},
    models::JobType,
};
use uuid::Uuid;

use crate::{NodeProviders, seal_shares};

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
///     base_path: "https://proof.taceo.network".to_string(),
///     ..Default::default()
/// };
/// let nodes = taceo_proof_client::get_random_node_providers(&config).await?;
/// let input = serde_json::from_reader(std::fs::File::open("input.json")?)?;
/// let blueprint_id = uuid::Uuid::parse_str("54f9ee38-0160-44e2-a1d8-08d1b6771cbf")?;
/// let job_id = taceo_proof_client::co_circom::schedule_full_job_rep3::<ark_bn254::Bn254>(
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
///     base_path: "https://proof.taceo.network".to_string(),
///     ..Default::default()
/// };
/// let nodes = taceo_proof_client::get_random_node_providers(&config).await?;
/// let blueprint_id = uuid::Uuid::parse_str("54f9ee38-0160-44e2-a1d8-08d1b6771cbf")?;
/// let witness = circom_types::Witness::from_reader(std::fs::File::open("witness.wtns")?)?;
/// let job_id = taceo_proof_client::co_circom::schedule_prove_job_rep3::<ark_bn254::Bn254>(
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
///     base_path: "https://proof.taceo.network".to_string(),
///     ..Default::default()
/// };
/// let nodes = taceo_proof_client::get_random_node_providers(&config).await?;
/// let blueprint_id = uuid::Uuid::parse_str("54f9ee38-0160-44e2-a1d8-08d1b6771cbf")?;
/// let witness = circom_types::Witness::from_reader(std::fs::File::open("witness.wtns")?)?;
/// let job_id = taceo_proof_client::co_circom::schedule_prove_job_shamir::<ark_bn254::Bn254>(
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
