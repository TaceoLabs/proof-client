import { seal_share, split_input_rep3_bls12_381, split_input_rep3_bn254, split_witness_rep3_bls12_381, split_witness_rep3_bn254, split_witness_shamir_bls12_381, split_witness_shamir_bn254, split_input_rep3_bls12_377, split_witness_rep3_bls12_377, split_witness_shamir_bls12_377 } from "./pkg/taceo_proof_wasm.js";
import { BlueprintApi, BlueprintCurve, JobApi, JobType } from '@taceo/proof-api-client';

/**
 * Download the base64 encoded encryption keys for the 3 nodes that will run the job.
 */
export async function getEncKeysB64(apiInstance: BlueprintApi, blueprintId: string): Promise<string[]> {
  const keyMaterial = await apiInstance.blueprintKeyMaterial({ id: blueprintId });
  return [keyMaterial[0].encKey, keyMaterial[1].encKey, keyMaterial[2].encKey];
}

async function scheduleJob(
  apiInstance: JobApi,
  blueprintId: string,
  jobType: JobType,
  code: string,
  shares: Uint8Array[],
  keys: string[],
): Promise<string> {
  const share0Ciphertext = seal_share(keys[0], shares[0]);
  const share1Ciphertext = seal_share(keys[1], shares[1]);
  const share2Ciphertext = seal_share(keys[2], shares[2]);

  const scheduleJobResponse = await apiInstance.scheduleJob({
    aBlueprintId: blueprintId,
    bJobType: jobType,
    cCode: code,
    inputParty0: new Blob([share0Ciphertext]),
    inputParty1: new Blob([share1Ciphertext]),
    inputParty2: new Blob([share2Ciphertext])
  });

  return scheduleJobResponse.jobId;
}

/**
 * Schedule a full job including witness extension. The retuned job id can be used to query the job status.
 */
export async function scheduleFullJobRep3(
  apiInstance: JobApi,
  blueprintId: string,
  code: string,
  curve: BlueprintCurve,
  keys: string[],
  public_inputs: string[],
  input: any
): Promise<string> {
  let sharedInput;
  switch (curve) {
    case BlueprintCurve.Bn254:
      sharedInput = split_input_rep3_bn254(input, public_inputs);
      break;
    case BlueprintCurve.Bls381:
      sharedInput = split_input_rep3_bls12_381(input, public_inputs);
      break;
    case BlueprintCurve.Bls377:
      sharedInput = split_input_rep3_bls12_377(input, public_inputs);
      break;
  }
  const shares = [sharedInput.shares0, sharedInput.shares1, sharedInput.shares2];
  return await scheduleJob(apiInstance, blueprintId, JobType.Rep3Full, code, shares, keys);
}

/**
 * Schedule a Rep3 prove job. The retuned job id can be used to query the job status.
 */
export async function scheduleProveJobRep3(
  apiInstance: JobApi,
  blueprintId: string,
  code: string,
  curve: BlueprintCurve,
  keys: string[],
  num_pub_inputs: number,
  witness: Uint8Array
): Promise<string> {
  let sharedInput;
  switch (curve) {
    case BlueprintCurve.Bn254:
      sharedInput = split_witness_rep3_bn254(witness, num_pub_inputs);
      break;
    case BlueprintCurve.Bls381:
      sharedInput = split_witness_rep3_bls12_381(witness, num_pub_inputs);
      break;
    case BlueprintCurve.Bls377:
      sharedInput = split_witness_rep3_bls12_377(witness, num_pub_inputs);
      break;
  }
  const shares = [sharedInput.shares0, sharedInput.shares1, sharedInput.shares2];
  return await scheduleJob(apiInstance, blueprintId, JobType.Rep3Prove, code, shares, keys);
}

/**
 * Schedule a Shamir prove job. The retuned job id can be used to query the job status.
 */
export async function scheduleProveJobShamir(apiInstance: JobApi,
  blueprintId: string,
  code: string,
  curve: BlueprintCurve,
  keys: string[],
  num_pub_inputs: number,
  witness: Uint8Array
): Promise<string> {
  let sharedInput;
  switch (curve) {
    case BlueprintCurve.Bn254:
      sharedInput = split_witness_shamir_bn254(witness, num_pub_inputs);
      break;
    case BlueprintCurve.Bls381:
      sharedInput = split_witness_shamir_bls12_381(witness, num_pub_inputs);
      break;
    case BlueprintCurve.Bls377:
      sharedInput = split_witness_shamir_bls12_377(witness, num_pub_inputs);
      break;
  }
  const shares = [sharedInput.shares0, sharedInput.shares1, sharedInput.shares2];
  return await scheduleJob(apiInstance, blueprintId, JobType.ShamirProve, code, shares, keys);
}

