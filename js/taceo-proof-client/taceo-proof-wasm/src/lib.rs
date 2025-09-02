use std::{collections::BTreeMap, io::Cursor};

use ark_ff::PrimeField;
use base64ct::{Base64, Encoding};
use circom_types::{traits::CircomArkworksPrimeFieldBridge, Witness};
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

/// The MPC protocol
#[wasm_bindgen]
pub enum MpcProtocol {
    Rep3 = "Rep3",
    Shamir = "Shamir",
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

fn co_circom_split_input<F: PrimeField>(
    keys: Vec<String>,
    input: JsValue,
    public_inputs: Vec<String>,
) -> Result<Shares, JsError> {
    let keys = base64_decode_keys(keys)?;
    let input = serde_wasm_bindgen::from_value::<co_circom_types::Input>(input)?;
    let [share0, share1, share2] = co_circom_types::split_input::<F>(input, &public_inputs)
        .map_err(|err| JsError::new(&format!("failed to share input: {err}")))?
        .into_iter()
        .zip(keys.iter())
        .map(|(inputs, key)| {
            let sealed_input = inputs
                .into_iter()
                .map(|(name, input)| {
                    let input = bincode::serialize(&input).expect("can serialize");
                    let input = key
                        .seal(&mut OsRng, &input)
                        .map_err(|_| JsError::new("encryption error"))?;
                    Ok((name, input))
                })
                .collect::<Result<BTreeMap<String, Vec<u8>>, JsError>>()?;
            Ok(bincode::serialize(&sealed_input).expect("can serialize"))
        })
        .collect::<Result<Vec<_>, JsError>>()?
        .try_into()
        .expect("len is 3");
    Ok(Shares {
        share0,
        share1,
        share2,
    })
}

/// Split the input into encrypted shares.
#[wasm_bindgen]
pub fn co_circom_split_input_bn254(
    keys: Vec<String>,
    input: JsValue,
    public_inputs: Vec<String>,
) -> Result<Shares, JsError> {
    co_circom_split_input::<ark_bn254::Fr>(keys, input, public_inputs)
}

/// Split the input into encrypted shares.
#[wasm_bindgen]
pub fn co_circom_split_input_bls12_381(
    keys: Vec<String>,
    input: JsValue,
    public_inputs: Vec<String>,
) -> Result<Shares, JsError> {
    co_circom_split_input::<ark_bls12_381::Fr>(keys, input, public_inputs)
}

fn co_circom_split_witness<F: PrimeField + CircomArkworksPrimeFieldBridge>(
    keys: Vec<String>,
    mpc_protocol: MpcProtocol,
    witness: Vec<u8>,
    num_pub_inputs: usize,
) -> Result<Shares, JsError> {
    let keys = base64_decode_keys(keys)?;
    let witness: Witness<F> = Witness::from_reader(Cursor::new(witness))?;
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
        MpcProtocol::__Invalid => unreachable!(),
    };
    let [share0, share1, share2] = shares
        .into_iter()
        .zip(keys)
        .map(|(share, key)| {
            key.seal(&mut OsRng, &share)
                .map_err(|_| JsError::new("encryption error"))
        })
        .collect::<Result<Vec<_>, JsError>>()?
        .try_into()
        .expect("len is 3");
    Ok(Shares {
        share0,
        share1,
        share2,
    })
}

/// Split the witness into encrypted shares.
#[wasm_bindgen]
pub fn co_circom_split_witness_bn254(
    keys: Vec<String>,
    mpc_protocol: MpcProtocol,
    witness: Vec<u8>,
    num_pub_inputs: usize,
) -> Result<Shares, JsError> {
    co_circom_split_witness::<ark_bn254::Fr>(keys, mpc_protocol, witness, num_pub_inputs)
}

/// Split the witness into encrypted shares.
#[wasm_bindgen]
pub fn co_circom_split_witness_bls12_381(
    keys: Vec<String>,
    mpc_protocol: MpcProtocol,
    witness: Vec<u8>,
    num_pub_inputs: usize,
) -> Result<Shares, JsError> {
    co_circom_split_witness::<ark_bls12_381::Fr>(keys, mpc_protocol, witness, num_pub_inputs)
}

/// Split the witness into encrypted shares.
#[wasm_bindgen]
pub fn co_circom_split_witness_bls12_377(
    keys: Vec<String>,
    mpc_protocol: MpcProtocol,
    witness: Vec<u8>,
    num_pub_inputs: usize,
) -> Result<Shares, JsError> {
    co_circom_split_witness::<ark_bls12_377::Fr>(keys, mpc_protocol, witness, num_pub_inputs)
}

/// Split the input into encrypted shares.
#[wasm_bindgen]
pub fn co_noir_split_input_bn254(
    keys: Vec<String>,
    input: JsValue,
    abi: JsValue,
    public_inputs: Vec<u32>,
) -> Result<Shares, JsError> {
    let keys = base64_decode_keys(keys)?;
    let abi = serde_wasm_bindgen::from_value::<Abi>(abi)?;
    let input = serde_wasm_bindgen::from_value::<noir_types::Input>(input)?
        .into_iter()
        .collect();
    let input = noir_types::partial_abi_bn254_from_json(input, &abi, &public_inputs)
        .map_err(|e| JsError::new(&e.to_string()))?;
    let [share0, share1, share2] = co_noir_types::split_input_rep3::<ark_bn254::Fr>(input)
        .into_iter()
        .zip(keys.iter())
        .map(|(inputs, key)| {
            let sealed_input = inputs
                .into_iter()
                .map(|(name, input)| {
                    let input = bincode::serialize(&input).expect("can serialize");
                    let input = key
                        .seal(&mut OsRng, &input)
                        .map_err(|_| JsError::new("encryption error"))?;
                    Ok((name, input))
                })
                .collect::<Result<BTreeMap<String, Vec<u8>>, JsError>>()?;
            Ok(bincode::serialize(&sealed_input).expect("can serialize"))
        })
        .collect::<Result<Vec<_>, JsError>>()?
        .try_into()
        .expect("len 3");
    Ok(Shares {
        share0,
        share1,
        share2,
    })
}

/// Split the witness into encrypted shares.
#[wasm_bindgen]
pub fn co_noir_split_witness_bn254(
    keys: Vec<String>,
    mpc_protocol: MpcProtocol,
    witness: Vec<u8>,
    public_inputs: Vec<u32>,
) -> Result<Shares, JsError> {
    let keys = base64_decode_keys(keys)?;
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
    let shares = match mpc_protocol {
        MpcProtocol::Rep3 => co_noir_types::split_witness_rep3(witness)
            .into_iter()
            .map(|share| bincode::serialize(&share).expect("can serialize"))
            .collect::<Vec<_>>(),
        MpcProtocol::Shamir => co_noir_types::split_witness_shamir(witness, 1, 3)
            .into_iter()
            .map(|share| bincode::serialize(&share).expect("can serialize"))
            .collect::<Vec<_>>(),
        MpcProtocol::__Invalid => unreachable!(),
    };
    let [share0, share1, share2] = shares
        .into_iter()
        .zip(keys)
        .map(|(share, key)| {
            key.seal(&mut OsRng, &share)
                .map_err(|_| JsError::new("encryption error"))
        })
        .collect::<Result<Vec<_>, JsError>>()?
        .try_into()
        .expect("len is 3");
    Ok(Shares {
        share0,
        share1,
        share2,
    })
}

fn base64_decode_keys(keys: Vec<String>) -> Result<[PublicKey; 3], JsError> {
    Ok(keys
        .into_iter()
        .map(|b64| {
            let bytes = Base64::decode_vec(&b64)?;
            let key = PublicKey::from_bytes(
                bytes
                    .try_into()
                    .map_err(|_| JsError::new("invalid key size"))?,
            );
            Ok(key)
        })
        .collect::<Result<Vec<_>, JsError>>()?
        .try_into()
        .expect("len is 3"))
}
