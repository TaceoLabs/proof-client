import { seal_share, split_input_rep3_bls12_381, split_input_rep3_bn254, split_witness_rep3_bls12_381, split_witness_rep3_bn254, split_witness_shamir_bls12_381, split_witness_shamir_bn254, split_input_rep3_bls12_377, split_witness_rep3_bls12_377, split_witness_shamir_bls12_377, verify_proof_result_signature } from "./pkg/taceo_proof_wasm.js";
import { BlueprintCurve, JobApi, JobType, NodeProviders } from '@taceo/proof-api-client';

export { JobApi, JobType, NodeApi, NodeProviders, NodeProvider, BlueprintCurve, Configuration, ConfigurationParameters } from '@taceo/proof-api-client';

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
 *
 * This function ensures the integrity and authenticity of a proof result by verifying
 * its signature using the provided verifying key (`vk`). The signature is validated
 * against a prehashed digest that includes the job ID, proof, and public inputs.
 *
 * @param jobId - The unique identifier of the job.
 * @param proof - The proof string to be verified.
 * @param publicInputs - The public inputs associated with the proof.
 * @param signature - The digital signature to be verified.
 * @param verifyKey - The verifying key used to validate the signature.
 * @throws If the signature is invalid or the digest/signature verification process fails.
 */
export function verifyProofResultSignature(jobId: string, proof: string, publicInputs: string, signature: string,
  verifyKey: string) {
  verify_proof_result_signature(jobId, proof, publicInputs, verifyKey, signature)
}

/**
 * Schedule a full REP3 job including witness extension.
 *
 * This function schedules a job using the Rep3 Secret Sharing scheme. It takes an input,
 * splits it into shares using Rep3, encrypts the shares, and sends them to the respective
 * nodes for execution. Instead of providing the extended witness, this job will compute
 * witness extension first and then the proof afterwards.
 *
 * @param config - The configuration object for the API client.
 * @param nodes - A set of node providers that will execute the job.
 * @param blueprintId - The unique identifier of the blueprint for the job.
 * @param voucher - An optional voucher for non-public blueprints.
 * @param publicInputs - A list of public input names for the blueprint's circuit.
 * @param input - The input data to be shared and used in the witness extension.
 * 
 * @returns The id (`Uuid`) of the scheduled job on success.
 * 
 * @throws Will throw an error if:
 * - Input sharing fails.
 * - Encryption of shares fails.
 * - The job scheduling API call fails.
 */
export async function scheduleFullJobRep3(
  apiInstance: JobApi,
  nodes: NodeProviders,
  blueprintId: string,
  code: string | null,
  curve: BlueprintCurve,
  publicInputs: string[],
  input: any
): Promise<string> {
  let sharedInput;
  switch (curve) {
    case BlueprintCurve.Bn254:
      sharedInput = split_input_rep3_bn254(input, publicInputs);
      break;
    case BlueprintCurve.Bls381:
      sharedInput = split_input_rep3_bls12_381(input, publicInputs);
      break;
    case BlueprintCurve.Bls377:
      sharedInput = split_input_rep3_bls12_377(input, publicInputs);
      break;
  }
  const shares = [sharedInput.shares0, sharedInput.shares1, sharedInput.shares2];
  return await scheduleJob(apiInstance, nodes, blueprintId, JobType.Rep3Full, code, shares);
}

/**
 * Schedule a Rep3 prove job.
 *
 * This function schedules a proof job using the Rep3 Secret Sharing scheme.
 * It takes a witness, splits it into shares using Rep3, encrypts the shares,
 * and sends them to the respective nodes for execution.
 *
 * @param config - The configuration object for the API client.
 * @param nodes - A set of node providers that will execute the job.
 * @param blueprintId - The unique identifier of the blueprint for the proof job.
 * @param voucher - An optional voucher for non-public blueprints.
 * @param numPubInputs - The number of public inputs for the blueprint's circuit.
 * @param witness - The witness data to be used in the proof.
 * 
 * @returns The id (`Uuid`) of the scheduled job on success.
 * 
 * @throws Will throw an error if:
 * - Witness sharing fails.
 * - Encryption of shares fails.
 * - The job scheduling API call fails.
 */
export async function scheduleProveJobRep3(
  apiInstance: JobApi,
  nodes: NodeProviders,
  blueprintId: string,
  code: string | null,
  curve: BlueprintCurve,
  numPubInputs: number,
  witness: Uint8Array
): Promise<string> {
  let sharedInput;
  switch (curve) {
    case BlueprintCurve.Bn254:
      sharedInput = split_witness_rep3_bn254(witness, numPubInputs);
      break;
    case BlueprintCurve.Bls381:
      sharedInput = split_witness_rep3_bls12_381(witness, numPubInputs);
      break;
    case BlueprintCurve.Bls377:
      sharedInput = split_witness_rep3_bls12_377(witness, numPubInputs);
      break;
  }
  const shares = [sharedInput.shares0, sharedInput.shares1, sharedInput.shares2];
  return await scheduleJob(apiInstance, nodes, blueprintId, JobType.Rep3Prove, code, shares);
}

/**
 * Schedule a Shamir prove job.
 *
 * This function schedules a proof job using Shamir Secret Sharing scheme.
 * It takes a witness, splits it into shares using Shamir, encrypts the shares,
 * and sends them to the respective nodes for execution.
 *
 * @param config - The configuration object for the API client.
 * @param nodes - A set of node providers that will execute the job.
 * @param blueprintId - The unique identifier of the blueprint for the proof job.
 * @param voucher - An optional voucher for non-public blueprints.
 * @param numPubInputs - The number of public inputs for the blueprint's circuit.
 * @param witness - The witness data to be used in the proof.
 * 
 * @returns The id (`Uuid`) of the scheduled job on success.
 * 
 * @throws Will throw an error if:
 * - Witness sharing fails.
 * - Encryption of shares fails.
 * - The job scheduling API call fails.
 */
export async function scheduleProveJobShamir(
  apiInstance: JobApi,
  nodes: NodeProviders,
  blueprintId: string,
  code: string | null,
  curve: BlueprintCurve,
  numPubInputs: number,
  witness: Uint8Array
): Promise<string> {
  let sharedInput;
  switch (curve) {
    case BlueprintCurve.Bn254:
      sharedInput = split_witness_shamir_bn254(witness, numPubInputs);
      break;
    case BlueprintCurve.Bls381:
      sharedInput = split_witness_shamir_bls12_381(witness, numPubInputs);
      break;
    case BlueprintCurve.Bls377:
      sharedInput = split_witness_shamir_bls12_377(witness, numPubInputs);
      break;
  }
  const shares = [sharedInput.shares0, sharedInput.shares1, sharedInput.shares2];
  return await scheduleJob(apiInstance, nodes, blueprintId, JobType.ShamirProve, code, shares);
}

/**
 * The stop strategy when waiting for the job result.
 */
export type StopStrategy =
  /** Stop if one node responded with a result. */
  | "First"
  /** Stop if the majority of nodes responded with a result. */
  | "Majority"
  /** Stop if all nodes responded with a result. */
  | "All";

/**
 * The WebSocket subscribe request to wait for a job result.
 *
 * This request is sent to the WebSocket server to subscribe to updates
 * for a specific job execution.
 */
export interface SubscribeExecutionRequest {
  /** The unique identifier of the job execution. */
  execution_id: string;

  /** The stop strategy to determine when to stop waiting for job results. */
  stop_on_finished_reports: StopStrategy;

  /** Whether to include status updates in the WebSocket messages. */
  with_status_updates?: boolean;
}

/**
 * The status of a job.
 */
export type JobStatus =
  | "Pending"
  | "Running"
  | "Finished"
  | "Cancelled";

/**
 * The result of a job, including signatures of the nodes that handled the job.
 */
export interface SignedResults {
  /** The signatures of each node used to compute the job (identified by their id). */
  signatures: { [key: number]: string };

  /** The proof as JSON (for CircomGroth16 proofs) or base64 encoded ark_serialized `ark_groth16::Proof` (for LibsnarkGroth16 proofs). */
  proof: string;

  /** The array of public inputs as JSON (for CircomGroth16 proofs) or base64 encoded ark_serialized `Vec<P::ScalarField>` (for LibsnarkGroth16 proofs). */
  public_inputs: string;
}

/**
 * An error result of a job.
 *
 * Provides details about a failure encountered during the execution of a job.
 */
export interface FailedReason {
  /** The node provider that encountered the error. */
  node_provider: number;
  /** The error string describing the failure. */
  error: string;
  /** The signature of the error and the node provider. */
  signature: string;
}

/**
 * The different messages received over the WebSocket connection.
 */
export type WebSocketMessage =
  /** The job completed successfully with signed results. */
  | { Success: SignedResults }
  /** A status update for the job. */
  | { Update: JobStatus }
  /** The job failed with a reason. */
  | { Failed: FailedReason }
  /** The job was cancelled. */
  | { Cancelled: null }
  /** An error occurred during the WebSocket communication. */
  | { Err: string };

/**
 * Fetches the result of a job execution using a WebSocket connection.
 *
 * This function connects to the specified WebSocket `url` and subscribes to updates
 * for the job identified by `jobId`. It listens for messages from the server and
 * processes them based on the provided `stopStrategy`. The function returns the
 * signed results of the job execution if successful, or an error if the job fails
 * or the connection is closed unexpectedly.
 *
 * @param url - The WebSocket URL to connect to for job updates.
 * @param jobId - The unique identifier of the job whose results are being fetched.
 * @param stopStrategy - The strategy to determine when to stop waiting for job results.
 *
 * @returns A promise that resolves to a `SignedResults` object containing the proof,
 * public inputs, and signatures from the nodes that handled the job.
 *
 * @throws This function throws an error if:
 * - The WebSocket connection fails to establish.
 * - The server sends invalid or unexpected data.
 * - The job fails, is cancelled, or encounters an error on a node.
 * - The server closes the WebSocket connection unexpectedly.
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
        reject(new Error(msg.Failed.error));
      } else if ("Cancelled" in msg) {
        reject(new Error("Job was cancelled!"));
      } else if ("Err" in msg) {
        reject(new Error(msg.Err));
      }
      socket.close();
    };
    socket.onerror = (event) => {
      reject(new Error(`WebSocket error: ${event}`));
      socket.close();
    };
  });
}
