use std::io::Cursor;

use ark_bls12_377::Bls12_377;
use ark_bls12_381::Bls12_381;
use ark_bn254::Bn254;
use ark_ec::pairing::Pairing;
use base64ct::{Base64, Encoding};
use circom_types::{
    traits::{CircomArkworksPairingBridge, CircomArkworksPrimeFieldBridge},
    Witness,
};
use co_circom_types::{CompressedRep3SharedWitness, Compression, ShamirSharedWitness};
use co_noir_types::PubPrivate;
use crypto_box::PublicKey;
use ed25519_dalek::{Signature, VerifyingKey};
use noir_types::Abi;
use rand::rngs::OsRng;
use sha2::{Digest as _, Sha512};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

/// The serialized input or witness shares
#[wasm_bindgen(getter_with_clone)]
pub struct Shares {
    pub share0: Vec<u8>,
    pub share1: Vec<u8>,
    pub share2: Vec<u8>,
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Seal the share with the given public key
#[wasm_bindgen]
pub fn seal_share(pk_b64: &str, share: Vec<u8>) -> Result<Vec<u8>, JsError> {
    let pk_bytes = Base64::decode_vec(pk_b64)?;
    let pk = PublicKey::from_bytes(
        pk_bytes
            .try_into()
            .map_err(|_| JsError::new("invalid key size"))?,
    );
    let ciphertext = pk
        .seal(&mut OsRng, &share)
        .map_err(|_| JsError::new("encryption error"))?;
    Ok(ciphertext)
}

/// Verify the signature with the given verifying key
#[wasm_bindgen]
pub fn verify_proof_result_signature(
    job_id: &str,
    proof: &str,
    public_inputs: &str,
    vk_b64: &str,
    signature_b64: &str,
) -> Result<(), JsError> {
    let job_id = job_id
        .parse::<Uuid>()
        .map_err(|_| JsError::new("invalid uuid"))?;
    let vk_bytes = Base64::decode_vec(vk_b64)?;
    let vk = VerifyingKey::from_bytes(
        &vk_bytes
            .try_into()
            .map_err(|_| JsError::new("invalid key size"))?,
    )?;
    let signature_bytes = Base64::decode_vec(signature_b64)?;
    let signature = Signature::from_bytes(
        &signature_bytes
            .try_into()
            .map_err(|_| JsError::new("invalid signature size"))?,
    );
    let mut digest = Sha512::new();
    digest.update(job_id.as_bytes());
    digest.update(proof);
    digest.update(public_inputs);

    vk.verify_prehashed_strict(
        digest,
        Some("taceo-proof-nps-reporting".as_bytes()),
        &signature,
    )
    .map_err(|_| JsError::new("signature verification failed"))?;

    Ok(())
}

fn co_circom_split_input_rep3<P>(
    input: JsValue,
    public_inputs: Vec<String>,
) -> Result<Shares, JsError>
where
    P: Pairing + CircomArkworksPairingBridge,
    P::ScalarField: CircomArkworksPrimeFieldBridge,
    P::BaseField: CircomArkworksPrimeFieldBridge,
{
    let input = serde_wasm_bindgen::from_value::<co_circom_types::Input>(input)?;
    let res = co_circom_types::split_input::<P::ScalarField>(input, &public_inputs)
        .map_err(|err| JsError::new(&format!("failed to parse and split input: {err}")))?;
    Ok(Shares {
        share0: bincode::serialize(&res[0]).map_err(|e| JsError::new(&e.to_string()))?,
        share1: bincode::serialize(&res[1]).map_err(|e| JsError::new(&e.to_string()))?,
        share2: bincode::serialize(&res[2]).map_err(|e| JsError::new(&e.to_string()))?,
    })
}

/// Split the input into REP3 shares.
#[wasm_bindgen]
pub fn co_circom_split_input_rep3_bn254(
    input: JsValue,
    public_inputs: Vec<String>,
) -> Result<Shares, JsError> {
    co_circom_split_input_rep3::<Bn254>(input, public_inputs)
}

/// Split the input into REP3 shares.
#[wasm_bindgen]
pub fn co_circom_split_input_rep3_bls12_381(
    input: JsValue,
    public_inputs: Vec<String>,
) -> Result<Shares, JsError> {
    co_circom_split_input_rep3::<Bls12_381>(input, public_inputs)
}

/// Split the input into REP3 shares.
#[wasm_bindgen]
pub fn co_circom_split_input_rep3_bls12_377(
    input: JsValue,
    public_inputs: Vec<String>,
) -> Result<Shares, JsError> {
    co_circom_split_input_rep3::<Bls12_377>(input, public_inputs)
}

fn co_circom_split_witness_rep3<P>(
    witness: Vec<u8>,
    num_pub_inputs: usize,
) -> Result<Shares, JsError>
where
    P: Pairing + CircomArkworksPairingBridge,
    P::ScalarField: CircomArkworksPrimeFieldBridge,
    P::BaseField: CircomArkworksPrimeFieldBridge,
{
    let witness: Witness<P::ScalarField> = Witness::from_reader(Cursor::new(witness))?;
    let mut rng = rand::thread_rng();
    let res = CompressedRep3SharedWitness::share_rep3(
        witness,
        num_pub_inputs,
        &mut rng,
        Compression::SeededHalfShares,
    );
    Ok(Shares {
        share0: bincode::serialize(&res[0]).map_err(|e| JsError::new(&e.to_string()))?,
        share1: bincode::serialize(&res[1]).map_err(|e| JsError::new(&e.to_string()))?,
        share2: bincode::serialize(&res[2]).map_err(|e| JsError::new(&e.to_string()))?,
    })
}

/// Split the witness into Shamir shares.
#[wasm_bindgen]
pub fn co_circom_split_witness_rep3_bn254(
    witness: Vec<u8>,
    num_pub_inputs: usize,
) -> Result<Shares, JsError> {
    co_circom_split_witness_rep3::<Bn254>(witness, num_pub_inputs)
}

/// Split the witness into Shamir shares.
#[wasm_bindgen]
pub fn co_circom_split_witness_rep3_bls12_381(
    witness: Vec<u8>,
    num_pub_inputs: usize,
) -> Result<Shares, JsError> {
    co_circom_split_witness_rep3::<Bls12_381>(witness, num_pub_inputs)
}

/// Split the witness into Shamir shares.
#[wasm_bindgen]
pub fn co_circom_split_witness_rep3_bls12_377(
    witness: Vec<u8>,
    num_pub_inputs: usize,
) -> Result<Shares, JsError> {
    co_circom_split_witness_rep3::<Bls12_377>(witness, num_pub_inputs)
}

fn co_circom_split_witness_shamir<P>(
    witness: Vec<u8>,
    num_pub_inputs: usize,
) -> Result<Shares, JsError>
where
    P: Pairing + CircomArkworksPairingBridge,
    P::ScalarField: CircomArkworksPrimeFieldBridge,
    P::BaseField: CircomArkworksPrimeFieldBridge,
{
    let witness: Witness<P::ScalarField> = Witness::from_reader(Cursor::new(witness))?;
    let mut rng = rand::thread_rng();
    let res = ShamirSharedWitness::share_shamir(witness, num_pub_inputs, 1, 3, &mut rng);
    Ok(Shares {
        share0: bincode::serialize(&res[0]).map_err(|e| JsError::new(&e.to_string()))?,
        share1: bincode::serialize(&res[1]).map_err(|e| JsError::new(&e.to_string()))?,
        share2: bincode::serialize(&res[2]).map_err(|e| JsError::new(&e.to_string()))?,
    })
}

/// Split the witness into Shamir shares.
#[wasm_bindgen]
pub fn co_circom_split_witness_shamir_bn254(
    witness: Vec<u8>,
    num_pub_inputs: usize,
) -> Result<Shares, JsError> {
    co_circom_split_witness_shamir::<Bn254>(witness, num_pub_inputs)
}

/// Split the witness into Shamir shares.
#[wasm_bindgen]
pub fn co_circom_split_witness_shamir_bls12_381(
    witness: Vec<u8>,
    num_pub_inputs: usize,
) -> Result<Shares, JsError> {
    co_circom_split_witness_shamir::<Bls12_381>(witness, num_pub_inputs)
}

/// Split the witness into Shamir shares.
#[wasm_bindgen]
pub fn co_circom_split_witness_shamir_bls12_377(
    witness: Vec<u8>,
    num_pub_inputs: usize,
) -> Result<Shares, JsError> {
    co_circom_split_witness_shamir::<Bls12_377>(witness, num_pub_inputs)
}

/// Split the input into Shamir shares.
#[wasm_bindgen]
pub fn co_noir_split_input_rep3_bn254(
    input: JsValue,
    abi: JsValue,
    public_inputs: Vec<u32>,
) -> Result<Shares, JsError> {
    let abi = serde_wasm_bindgen::from_value::<Abi>(abi)?;
    let input = serde_wasm_bindgen::from_value::<noir_types::Input>(input)?
        .into_iter()
        .collect();
    let input = noir_types::partial_abi_bn254_from_json(input, &abi, &public_inputs)
        .map_err(|e| JsError::new(&e.to_string()))?;
    let res = co_noir_types::split_input_rep3::<ark_bn254::Fr>(input);
    Ok(Shares {
        share0: bincode::serialize(&res[0]).map_err(|e| JsError::new(&e.to_string()))?,
        share1: bincode::serialize(&res[1]).map_err(|e| JsError::new(&e.to_string()))?,
        share2: bincode::serialize(&res[2]).map_err(|e| JsError::new(&e.to_string()))?,
    })
}

/// Split the witness into Shamir shares.
#[wasm_bindgen]
pub fn co_noir_split_witness_rep3_bn254(
    witness: Vec<u8>,
    public_inputs: Vec<u32>,
) -> Result<Shares, JsError> {
    let witness = noir_types::witness_from_reader(witness.as_slice())?;
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
    let res = co_noir_types::split_witness_rep3::<ark_bn254::Fr>(witness);
    Ok(Shares {
        share0: bincode::serialize(&res[0]).map_err(|e| JsError::new(&e.to_string()))?,
        share1: bincode::serialize(&res[1]).map_err(|e| JsError::new(&e.to_string()))?,
        share2: bincode::serialize(&res[2]).map_err(|e| JsError::new(&e.to_string()))?,
    })
}

/// Split the witness into Shamir shares.
#[wasm_bindgen]
pub fn co_noir_split_witness_shamir_bn254(
    witness: Vec<u8>,
    public_inputs: Vec<u32>,
) -> Result<Shares, JsError> {
    let witness = noir_types::witness_from_reader(witness.as_slice())?;
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
    let res = co_noir_types::split_witness_shamir::<ark_bn254::Fr>(witness, 1, 3);
    Ok(Shares {
        share0: bincode::serialize(&res[0]).map_err(|e| JsError::new(&e.to_string()))?,
        share1: bincode::serialize(&res[1]).map_err(|e| JsError::new(&e.to_string()))?,
        share2: bincode::serialize(&res[2]).map_err(|e| JsError::new(&e.to_string()))?,
    })
}
