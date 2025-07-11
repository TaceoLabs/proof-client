import { seal_share, split_input_rep3_bls12_381, split_input_rep3_bn254, split_witness_rep3_bls12_381, split_witness_rep3_bn254, split_witness_shamir_bls12_381, split_witness_shamir_bn254, split_input_rep3_bls12_377, split_witness_rep3_bls12_377, split_witness_shamir_bls12_377, verify_proof_result_signature } from "./pkg/taceo_proof_wasm.js";
import { BlueprintCurve, JobApi, JobType, NodeProviders } from '@taceo/proof-api-client';

async function scheduleJob(
  apiInstance: JobApi,
  nodes: NodeProviders,
  blueprintId: string,
  jobType: JobType,
  code: string | null,
  shares: Uint8Array[],
): Promise<string> {
  const share0Ciphertext = seal_share(nodes.node0.encKey, shares[0]);
  const share1Ciphertext = seal_share(nodes.node1.encKey, shares[1]);
  const share2Ciphertext = seal_share(nodes.node2.encKey, shares[2]);

  const scheduleJobResponse = await apiInstance.scheduleJob({
    aBlueprintId: blueprintId,
    bJobType: jobType,
    cNode0: nodes.node0.id,
    cNode1: nodes.node1.id,
    cNode2: nodes.node2.id,
    dCode: code,
    inputParty0: new Blob([share0Ciphertext]),
    inputParty1: new Blob([share1Ciphertext]),
    inputParty2: new Blob([share2Ciphertext])
  });

  return scheduleJobResponse.jobId;
}

/**
 * Verify the signature of a proof result. Throws an error if the signature cannot be verified.
 */
export function verifyProofResultSignature(jobId: string, proof: string, publicInputs: string, signature: string,
  verifyKey: string) {
  verify_proof_result_signature(jobId, proof, publicInputs, verifyKey, signature)
}

/**
 * Schedule a full job including witness extension. The retuned job id can be used to query the job status.
 */
export async function scheduleFullJobRep3(
  apiInstance: JobApi,
  nodes: NodeProviders,
  blueprintId: string,
  code: string | null,
  curve: BlueprintCurve,
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
  return await scheduleJob(apiInstance, nodes, blueprintId, JobType.Rep3Full, code, shares);
}

/**
 * Schedule a Rep3 prove job. The retuned job id can be used to query the job status.
 */
export async function scheduleProveJobRep3(
  apiInstance: JobApi,
  nodes: NodeProviders,
  blueprintId: string,
  code: string | null,
  curve: BlueprintCurve,
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
  return await scheduleJob(apiInstance, nodes, blueprintId, JobType.Rep3Prove, code, shares);
}

/**
 * Schedule a Shamir prove job. The retuned job id can be used to query the job status.
 */
export async function scheduleProveJobShamir(
  apiInstance: JobApi,
  nodes: NodeProviders,
  blueprintId: string,
  code: string | null,
  curve: BlueprintCurve,
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
  return await scheduleJob(apiInstance, nodes, blueprintId, JobType.ShamirProve, code, shares);
}

export type StopStrategy = "First" | "All" | "Majority";

export interface SubscribeExecutionRequest {
  execution_id: string;
  stop_on_finished_reports: StopStrategy;
  with_status_updates?: boolean;
}

export type JobStatus =
  | "Pending"
  | "Running"
  | "Finished"
  | "Cancelled";

export interface SignedResults {
  signatures: { [key: number]: string };
  proof: string;
  public_inputs: string;
}

export interface FailedReason {
  node_provider: number;
  error: string;
  signature: string;
}

export type WebSocketMessage =
  | { Success: SignedResults }
  | { Update: JobStatus }
  | { Failed: FailedReason }
  | { Cancelled: null }
  | { Err: string };

/**
 * Fetch the result for the given jobId via a WebSocket connection.
 */
export function fetchJobResult(
  url: string,
  jobId: string,
  stopStrategy?: StopStrategy,
): Promise<SignedResults> {
  return new Promise((resolve, reject) => {
    const socket = new WebSocket(url);
    socket.onopen = () => {
      const request: SubscribeExecutionRequest = {
        execution_id: jobId,
        stop_on_finished_reports: stopStrategy ? stopStrategy! : "First",
        with_status_updates: false,
      };
      socket.send(JSON.stringify(request));
    };
    socket.onmessage = (event) => {
      const msg: WebSocketMessage = JSON.parse(event.data);
      if ("Success" in msg) {
        resolve(msg.Success);
      } else if ("Failed" in msg) {
        reject(new Error(msg.Failed.error))
      } else if ("Cancelled" in msg) {
        reject(new Error("job was cancelled!"))
      } else if ("Err" in msg) {
        reject(new Error(msg.Err))
      }
      socket.close();
    };
    socket.onerror = (event) => {
      reject(new Error(`ws error: ${event}`))
      socket.close();
    };
  });
}
