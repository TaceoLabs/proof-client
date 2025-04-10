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
use eyre::Context;
use ppd_api_client::{
    apis::{configuration::Configuration, job_api},
    models::{JobType, ScheduleJobRequest, ScheduleJobResponse},
};
use uuid::Uuid;

async fn schedule_job(
    config: &Configuration,
    code: &str,
    blueprint_id: i32,
    job_type: JobType,
) -> eyre::Result<ScheduleJobResponse> {
    Ok(job_api::schedule_job(
        config,
        ScheduleJobRequest::new(blueprint_id, code.to_string(), job_type),
    )
    .await?)
}

async fn add_input(
    config: &Configuration,
    res: &ScheduleJobResponse,
    shares: [Vec<u8>; 3],
) -> eyre::Result<()> {
    let pk0 = PublicKey::from_bytes(
        Base64::decode_vec(&res.key_material[0].enc_key)?
            .try_into()
            .expect("correct len"),
    );
    let pk1 = PublicKey::from_bytes(
        Base64::decode_vec(&res.key_material[1].enc_key)?
            .try_into()
            .expect("correct len"),
    );
    let pk2 = PublicKey::from_bytes(
        Base64::decode_vec(&res.key_material[2].enc_key)?
            .try_into()
            .expect("correct len"),
    );
    let ct0 = pk0
        .seal(&mut OsRng, &shares[0])
        .context("while sealing share")?;
    let ct1 = pk1
        .seal(&mut OsRng, &shares[1])
        .context("while sealing share")?;
    let ct2 = pk2
        .seal(&mut OsRng, &shares[2])
        .context("while sealing share")?;
    job_api::add_input(config, &res.job_id.to_string(), ct0, ct1, ct2).await?;
    Ok(())
}

/// Schedule a full REP3 job including witness extenesion.
pub async fn schedule_full_job_rep3<P>(
    config: &Configuration,
    code: &str,
    blueprint_id: i32,
    input: Input,
    public_inputs: &[String],
) -> eyre::Result<Uuid>
where
    P: Pairing + CircomArkworksPairingBridge,
    P::ScalarField: CircomArkworksPrimeFieldBridge,
    P::BaseField: CircomArkworksPrimeFieldBridge,
{
    let res = schedule_job(config, code, blueprint_id, JobType::Rep3Full).await?;
    let [share0, share1, share2] = split_input::<P::ScalarField>(input, public_inputs)?;
    let shares = [
        bincode::serialize(&share0)?,
        bincode::serialize(&share1)?,
        bincode::serialize(&share2)?,
    ];
    add_input(config, &res, shares).await?;
    Ok(res.job_id)
}

/// Schedule a REP3 prove job.
pub async fn schedule_prove_job_rep3<P>(
    config: &Configuration,
    code: &str,
    blueprint_id: i32,
    witness: Witness<P::ScalarField>,
    num_pub_inputs: usize,
) -> eyre::Result<Uuid>
where
    P: Pairing + CircomArkworksPairingBridge,
    P::ScalarField: CircomArkworksPrimeFieldBridge,
    P::BaseField: CircomArkworksPrimeFieldBridge,
{
    let res = schedule_job(config, code, blueprint_id, JobType::Rep3Prove).await?;
    let mut rng = rand::thread_rng();
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
    add_input(config, &res, shares).await?;
    Ok(res.job_id)
}

/// Schedule a Shamir prove job.
pub async fn schedule_prove_job_shamir<P>(
    config: &Configuration,
    code: &str,
    blueprint_id: i32,
    witness: Witness<P::ScalarField>,
    num_pub_inputs: usize,
) -> eyre::Result<Uuid>
where
    P: Pairing + CircomArkworksPairingBridge,
    P::ScalarField: CircomArkworksPrimeFieldBridge,
    P::BaseField: CircomArkworksPrimeFieldBridge,
{
    let res = schedule_job(config, code, blueprint_id, JobType::Rep3Prove).await?;
    let mut rng = rand::thread_rng();
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
    add_input(config, &res, shares).await?;
    Ok(res.job_id)
}
