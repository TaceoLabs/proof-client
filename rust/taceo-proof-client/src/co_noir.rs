use co_noir_types::PubPrivate;
use eyre::Context;
use noir_types::Abi;
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
/// * `abi` - The ABI for the Noir program.
/// * `public_inputs` - A list of public inputs witness indices for the blueprint's circuit.
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
pub async fn schedule_full_job_rep3(
    config: &Configuration,
    nodes: &NodeProviders,
    blueprint_id: Uuid,
    voucher: Option<&str>,
    abi: &Abi,
    public_inputs: &[u32],
    input: impl std::io::Read,
) -> eyre::Result<Uuid> {
    tracing::debug!("schedule_job Rep3Full for blueprint_id {blueprint_id}");
    tracing::debug!("sharing input...");
    let input = noir_types::partially_read_abi_bn254(input, abi, public_inputs)
        .context("while reading input")?;
    let [share0, share1, share2] = co_noir_types::split_input_rep3::<ark_bn254::Fr>(input);
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
/// * `public_inputs` - A list of public inputs witness indices for the blueprint's circuit.
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
pub async fn schedule_prove_job_rep3(
    config: &Configuration,
    nodes: &NodeProviders,
    blueprint_id: Uuid,
    voucher: Option<&str>,
    public_inputs: &[u32],
    witness: Vec<ark_bn254::Fr>,
) -> eyre::Result<Uuid> {
    tracing::debug!("schedule_job Rep3Prove for blueprint_id {blueprint_id}");
    tracing::debug!("sharing witness...");
    let witness = witness
        .into_iter()
        .enumerate()
        .map(|(idx, value)| {
            if public_inputs.contains(&(idx as u32)) {
                PubPrivate::Public(value)
            } else {
                PubPrivate::Private(value)
            }
        })
        .collect::<Vec<_>>();
    let [share0, share1, share2] = co_noir_types::split_witness_rep3::<ark_bn254::Fr>(witness);
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
/// * `public_inputs` - A list of public inputs witness indices for the blueprint's circuit.
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
pub async fn schedule_prove_job_shamir(
    config: &Configuration,
    nodes: &NodeProviders,
    blueprint_id: Uuid,
    voucher: Option<&str>,
    public_inputs: &[u32],
    witness: Vec<ark_bn254::Fr>,
) -> eyre::Result<Uuid> {
    tracing::debug!("schedule_job ShamirProve for blueprint_id {blueprint_id}");
    tracing::debug!("sharing witness...");
    let witness = witness
        .into_iter()
        .enumerate()
        .map(|(idx, value)| {
            if public_inputs.contains(&(idx as u32)) {
                PubPrivate::Public(value)
            } else {
                PubPrivate::Private(value)
            }
        })
        .collect::<Vec<_>>();
    let [share0, share1, share2] =
        co_noir_types::split_witness_shamir::<ark_bn254::Fr>(witness, 1, 3)
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
