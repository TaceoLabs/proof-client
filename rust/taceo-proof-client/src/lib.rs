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
use taceo_proof_api_client::{
    apis::{blueprint_api, configuration::Configuration, job_api},
    models::{JobType, ProofResult},
};
use uuid::Uuid;

/// The encryption and verification keys for a NPS
#[derive(Debug, Clone)]
pub struct NpsKeyMaterial {
    pub enc_key: PublicKey,
    pub verify_key: VerifyingKey,
}

/// Download the encryption and verify keys for the 3 nodes that will run the job.
pub async fn get_nps_key_material(
    config: &Configuration,
    blueprint_id: Uuid,
) -> eyre::Result<[NpsKeyMaterial; 3]> {
    tracing::debug!("fetching key material for blueprint {blueprint_id}");
    let key_material =
        blueprint_api::blueprint_key_material(config, &blueprint_id.to_string()).await?;
    if key_material.len() != 3 {
        eyre::bail!("got wrong number of key_material");
    }
    // we checked len above, we can unwrap here
    Ok(key_material
        .iter()
        .map(|nps| {
            tracing::debug!("decode pub key");
            let enc_key = PublicKey::from_bytes(
                Base64::decode_vec(&nps.enc_key)?
                    .try_into()
                    .expect("correct len"),
            );
            let verify_key = VerifyingKey::from_bytes(
                &Base64::decode_vec(&nps.verify_key)?
                    .try_into()
                    .expect("correct len"),
            )?;

            Ok(NpsKeyMaterial {
                enc_key,
                verify_key,
            })
        })
        .collect::<eyre::Result<Vec<NpsKeyMaterial>>>()?
        .try_into()
        .unwrap())
}

/// Verify the signature of a proof result.
pub fn verify_proof_result_signature(
    job_id: Uuid,
    result: &ProofResult,
    signature: Signature,
    vk: VerifyingKey,
) -> eyre::Result<()> {
    tracing::debug!("verify result for job {job_id}");
    let proof_bytes = Base64::decode_vec(&result.proof)?;
    let public_inputs_bytes = Base64::decode_vec(&result.public_inputs)?;

    let mut digest = Sha512::new();
    digest.update(job_id.as_bytes());
    digest.update(proof_bytes);
    digest.update(public_inputs_bytes);

    vk.verify_prehashed_strict(
        digest,
        Some("taceo-proof-nps-reporting".as_bytes()),
        &signature,
    )
    .context("while verifying signature")?;
    tracing::debug!("signature ok for job {job_id}");

    Ok(())
}

fn seal_shares(keys: &[NpsKeyMaterial; 3], shares: [Vec<u8>; 3]) -> eyre::Result<[Vec<u8>; 3]> {
    tracing::debug!("sealing shares...");
    let ct0 = keys[0]
        .enc_key
        .seal(&mut OsRng, &shares[0])
        .context("while sealing share")?;
    let ct1 = keys[1]
        .enc_key
        .seal(&mut OsRng, &shares[1])
        .context("while sealing share")?;
    let ct2 = keys[2]
        .enc_key
        .seal(&mut OsRng, &shares[2])
        .context("while sealing share")?;
    Ok([ct0, ct1, ct2])
}

/// Schedule a full REP3 job including witness extension.
pub async fn schedule_full_job_rep3<P>(
    config: &Configuration,
    blueprint_id: Uuid,
    voucher: Option<&str>,
    keys: &[NpsKeyMaterial; 3],
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
    blueprint_id: Uuid,
    voucher: Option<&str>,
    keys: &[NpsKeyMaterial; 3],
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
    blueprint_id: Uuid,
    voucher: Option<&str>,
    keys: &[NpsKeyMaterial; 3],
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
