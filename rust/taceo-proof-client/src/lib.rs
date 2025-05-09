use ark_ec::pairing::Pairing;
use ark_serialize::CanonicalDeserialize;
use base64ct::{Base64, Encoding};
use circom_types::{
    Witness,
    traits::{CircomArkworksPairingBridge, CircomArkworksPrimeFieldBridge},
};
use co_circom_types::{
    CompressedRep3SharedWitness, Compression, Input, ShamirSharedWitness, split_input,
};
use co_groth16::Proof;
use crypto_box::{PublicKey, aead::OsRng};
use eyre::Context;
use taceo_proof_api_client::{
    apis::{blueprint_api, configuration::Configuration, job_api},
    models::{JobStatus, JobType},
};
use uuid::Uuid;

/// Download the encryption keys for the 3 nodes that will run the job.
pub async fn get_enc_keys(
    config: &Configuration,
    blueprint_id: Uuid,
) -> eyre::Result<[PublicKey; 3]> {
    tracing::debug!("fetching key material for blueprint {blueprint_id}");
    let key_material =
        blueprint_api::blueprint_key_material(config, &blueprint_id.to_string()).await?;
    if key_material.len() != 3 {
        eyre::bail!("got wrong number of key_material");
    }
    tracing::debug!("decode pub key 0");
    let pk0 = PublicKey::from_bytes(
        Base64::decode_vec(&key_material[0].enc_key)?
            .try_into()
            .expect("correct len"),
    );
    tracing::debug!("decode pub key 0");
    let pk1 = PublicKey::from_bytes(
        Base64::decode_vec(&key_material[1].enc_key)?
            .try_into()
            .expect("correct len"),
    );
    tracing::debug!("decode pub key 0");
    let pk2 = PublicKey::from_bytes(
        Base64::decode_vec(&key_material[2].enc_key)?
            .try_into()
            .expect("correct len"),
    );
    Ok([pk0, pk1, pk2])
}

fn seal_shares(keys: &[PublicKey; 3], shares: [Vec<u8>; 3]) -> eyre::Result<[Vec<u8>; 3]> {
    tracing::debug!("sealing shares...");
    let ct0 = keys[0]
        .seal(&mut OsRng, &shares[0])
        .context("while sealing share")?;
    let ct1 = keys[1]
        .seal(&mut OsRng, &shares[1])
        .context("while sealing share")?;
    let ct2 = keys[2]
        .seal(&mut OsRng, &shares[2])
        .context("while sealing share")?;
    Ok([ct0, ct1, ct2])
}

/// Schedule a full REP3 job including witness extension.
pub async fn schedule_full_job_rep3<P>(
    config: &Configuration,
    blueprint_id: Uuid,
    code: &str,
    keys: &[PublicKey; 3],
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
    let [ct0, ct1, ct2] = seal_shares(keys, shares)?;
    tracing::debug!("scheduling job...");
    let res = job_api::schedule_job(
        config,
        &blueprint_id.to_string(),
        JobType::Rep3Full,
        code,
        ct0,
        ct1,
        ct2,
    )
    .await?;
    let job_id = res.job_id;
    tracing::debug!("job_id = {job_id}");
    Ok(job_id)
}

/// Schedule a REP3 prove job.
pub async fn schedule_prove_job_rep3<P>(
    config: &Configuration,
    blueprint_id: Uuid,
    code: &str,
    keys: &[PublicKey; 3],
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
    let [ct0, ct1, ct2] = seal_shares(keys, shares)?;
    tracing::debug!("scheduling job...");
    let res = job_api::schedule_job(
        config,
        &blueprint_id.to_string(),
        JobType::Rep3Prove,
        code,
        ct0,
        ct1,
        ct2,
    )
    .await?;
    let job_id = res.job_id;
    tracing::debug!("job_id = {job_id}");
    Ok(job_id)
}

/// Schedule a Shamir prove job.
pub async fn schedule_prove_job_shamir<P>(
    config: &Configuration,
    blueprint_id: Uuid,
    code: &str,
    keys: &[PublicKey; 3],
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
    let [ct0, ct1, ct2] = seal_shares(keys, shares)?;
    tracing::debug!("scheduling job...");
    let res = job_api::schedule_job(
        config,
        &blueprint_id.to_string(),
        JobType::ShamirProve,
        code,
        ct0,
        ct1,
        ct2,
    )
    .await?;
    let job_id = res.job_id;
    tracing::debug!("job_id = {job_id}");
    Ok(job_id)
}

/// The result of a scheduled job, represents either a successful proof, a error, or the status of a still running job
#[derive(Debug, Clone)]
pub enum JobResult<P: Pairing> {
    Ok((Proof<P>, Vec<P::ScalarField>)),
    Running(JobStatus),
    Err(String),
}

pub async fn get_job_result<P: Pairing>(
    config: &Configuration,
    id: Uuid,
) -> eyre::Result<JobResult<P>> {
    let res = job_api::get_result(config, &id.to_string()).await?;
    tracing::debug!("result from api: {res:?}");
    match res.status {
        JobStatus::Success => {
            let proof_res = res.ok.unwrap().unwrap();
            tracing::debug!("deser proof...");
            let proof = Proof::<P>::deserialize_uncompressed_unchecked(
                Base64::decode_vec(&proof_res.proof)?.as_slice(),
            )?;
            tracing::debug!("deser public_inputs...");
            let public_inputs = Vec::<P::ScalarField>::deserialize_uncompressed_unchecked(
                Base64::decode_vec(&proof_res.public_inputs)?.as_slice(),
            )?;
            tracing::debug!("done");
            Ok(JobResult::Ok((proof, public_inputs)))
        }
        JobStatus::Failed => Ok(JobResult::Err(res.error.unwrap().unwrap())),
        status => Ok(JobResult::Running(status)),
    }
}
