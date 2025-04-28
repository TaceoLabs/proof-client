import { seal_share, split_input_rep3_bls12_381, split_input_rep3_bn254, split_witness_rep3_bls12_381, split_witness_rep3_bn254, split_witness_shamir_bls12_381, split_witness_shamir_bn254, split_input_rep3_bls12_377, split_witness_rep3_bls12_377, split_witness_shamir_bls12_377 } from "@taceo/proof-wasm";
import type { ScheduleJobRequest, ScheduleJobResponse } from '@taceo/proof-api-client';
import { JobApi, JobType } from "@taceo/proof-api-client";

export { JobApi, Configuration, ConfigurationParameters } from "@taceo/proof-api-client";

async function scheduleJob(apiInstance: JobApi, code: string, blueprintId: string, jobType: JobType): Promise<ScheduleJobResponse> {
  const request: ScheduleJobRequest = {
    blueprintId,
    jobType,
    code,
  };
  return apiInstance.scheduleJob({ scheduleJobRequest: request })
}

async function addInput(apiInstance: JobApi, scheduleJobResponse: ScheduleJobResponse, shares: { shares0: Uint8Array, shares1: Uint8Array, shares2: Uint8Array }): Promise<void> {
  const pk0 = scheduleJobResponse.keyMaterial[0].encKey;
  const pk1 = scheduleJobResponse.keyMaterial[1].encKey;
  const pk2 = scheduleJobResponse.keyMaterial[2].encKey;

  const share0Ciphertext = seal_share(pk0, shares.shares0);
  const share1Ciphertext = seal_share(pk1, shares.shares1);
  const share2Ciphertext = seal_share(pk2, shares.shares2);

  await apiInstance.addInput({
    id: scheduleJobResponse.jobId,
    inputParty0: new Blob([share0Ciphertext]),
    inputParty1: new Blob([share1Ciphertext]),
    inputParty2: new Blob([share2Ciphertext])
  });
}

/**
 * Schedule a full job including witness extension. The retuned job id can be used to query the job status.
 */
export async function scheduleFullJobRep3Bn254(apiInstance: JobApi, code: string, blueprintId: string, public_inputs: string[], input: any): Promise<string> {
  const scheduleJobResponse = await scheduleJob(apiInstance, code, blueprintId, JobType.Rep3Full);
  const sharedInput = split_input_rep3_bn254(input, public_inputs);
  addInput(apiInstance, scheduleJobResponse, sharedInput)
  return scheduleJobResponse.jobId;
}

/**
 * Schedule a Rep3 prove job. The retuned job id can be used to query the job status.
 */
export async function scheduleProveJobRep3Bn254(apiInstance: JobApi, code: string, blueprintId: string, num_pub_inputs: number, witness: Uint8Array): Promise<string> {
  const scheduleJobResponse = await scheduleJob(apiInstance, code, blueprintId, JobType.Rep3Prove);
  const sharedInput = split_witness_rep3_bn254(witness, num_pub_inputs);
  addInput(apiInstance, scheduleJobResponse, sharedInput)
  return scheduleJobResponse.jobId;
}

/**
 * Schedule a Shamir prove job. The retuned job id can be used to query the job status.
 */
export async function scheduleProveJobShamirBn254(apiInstance: JobApi, code: string, blueprintId: string, num_pub_inputs: number, witness: Uint8Array): Promise<string> {
  const scheduleJobResponse = await scheduleJob(apiInstance, code, blueprintId, JobType.ShamirProve);
  const sharedInput = split_witness_shamir_bn254(witness, num_pub_inputs);
  addInput(apiInstance, scheduleJobResponse, sharedInput)
  return scheduleJobResponse.jobId;
}

/**
 * Schedule a full job including witness extension. The retuned job id can be used to query the job status.
 */
export async function scheduleFullJobRep3Bls381(apiInstance: JobApi, code: string, blueprintId: string, public_inputs: string[], input: any): Promise<string> {
  const scheduleJobResponse = await scheduleJob(apiInstance, code, blueprintId, JobType.Rep3Full);
  const sharedInput = split_input_rep3_bls12_381(input, public_inputs);
  addInput(apiInstance, scheduleJobResponse, sharedInput)
  return scheduleJobResponse.jobId;
}

/**
 * Schedule a Rep3 prove job. The retuned job id can be used to query the job status.
 */
export async function scheduleProveJobRep3Bls381(apiInstance: JobApi, code: string, blueprintId: string, num_pub_inputs: number, witness: Uint8Array): Promise<string> {
  const scheduleJobResponse = await scheduleJob(apiInstance, code, blueprintId, JobType.Rep3Prove);
  const sharedInput = split_witness_rep3_bls12_381(witness, num_pub_inputs);
  addInput(apiInstance, scheduleJobResponse, sharedInput)
  return scheduleJobResponse.jobId;
}

/**
 * Schedule a Shamir prove job. The retuned job id can be used to query the job status.
 */
export async function scheduleProveJobShamirBls381(apiInstance: JobApi, code: string, blueprintId: string, num_pub_inputs: number, witness: Uint8Array): Promise<string> {
  const scheduleJobResponse = await scheduleJob(apiInstance, code, blueprintId, JobType.ShamirProve);
  const sharedInput = split_witness_shamir_bls12_381(witness, num_pub_inputs);
  addInput(apiInstance, scheduleJobResponse, sharedInput)
  return scheduleJobResponse.jobId;
}

/**
 * Schedule a full job including witness extension. The retuned job id can be used to query the job status.
 */
export async function scheduleFullJobRep3Bls377(apiInstance: JobApi, code: string, blueprintId: string, public_inputs: string[], input: any): Promise<string> {
  const scheduleJobResponse = await scheduleJob(apiInstance, code, blueprintId, JobType.Rep3Full);
  const sharedInput = split_input_rep3_bls12_377(input, public_inputs);
  addInput(apiInstance, scheduleJobResponse, sharedInput)
  return scheduleJobResponse.jobId;
}

/**
 * Schedule a Rep3 prove job. The retuned job id can be used to query the job status.
 */
export async function scheduleProveJobRep3Bls377(apiInstance: JobApi, code: string, blueprintId: string, num_pub_inputs: number, witness: Uint8Array): Promise<string> {
  const scheduleJobResponse = await scheduleJob(apiInstance, code, blueprintId, JobType.Rep3Prove);
  const sharedInput = split_witness_rep3_bls12_377(witness, num_pub_inputs);
  addInput(apiInstance, scheduleJobResponse, sharedInput)
  return scheduleJobResponse.jobId;
}

/**
 * Schedule a Shamir prove job. The retuned job id can be used to query the job status.
 */
export async function scheduleProveJobShamirBls377(apiInstance: JobApi, code: string, blueprintId: string, num_pub_inputs: number, witness: Uint8Array): Promise<string> {
  const scheduleJobResponse = await scheduleJob(apiInstance, code, blueprintId, JobType.ShamirProve);
  const sharedInput = split_witness_shamir_bls12_377(witness, num_pub_inputs);
  addInput(apiInstance, scheduleJobResponse, sharedInput)
  return scheduleJobResponse.jobId;
}

