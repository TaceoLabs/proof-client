import { co_circom_split_input_bls12_381, co_circom_split_input_bn254, co_circom_split_witness_bls12_381, co_circom_split_witness_bn254, co_circom_split_witness_bls12_377 } from "../taceo-proof-wasm/pkg/taceo_proof_wasm.js";
import { BlueprintCurve, JobApi, MpcProtocol, NodeProviders } from '@taceo/proof-api-client';
import { StopStrategy, fetchJobResult as wsFetchResult } from "./WebSocket";
import { Groth16Proof, PublicSignals } from "snarkjs";
import { Base64 } from "js-base64";

/**
 * Schedule a full REP3 job including witness extension.
 *
 * This function schedules a job using the Rep3 Secret Sharing scheme. It takes an input,
 * splits it into shares using Rep3, encrypts the shares, and sends them to the respective
 * nodes for execution. This function does not require an extended witness to be provided
 * beforehand; instead, it computes the witness extension as part of the job execution,
 * followed by the proof generation.
 *
 * @param apiInstance - An instance of the JobApi client to interact with the job scheduling API.
 * @param nodes - A set of node providers that will execute the job. Each node must include an
 *                encryption key (`encKey`) and a unique identifier (`id`).
 * @param blueprintId - The unique identifier of the blueprint for the job.
 * @param voucher - An optional voucher for non-public blueprints. Pass `null` if not required.
 * @param curve - The elliptic curve used in the blueprint's circuit. Supported curves are `Bn254`
 *                and `Bls381`. `Bls377` is not supported for this function.
 * @param publicInputs - A list of public input names for the blueprint's circuit.
 * @param input - The input data to be shared and used in the witness extension. The data should
 *                be formatted as required by the blueprint's circuit.
 * 
 * @returns The id (`Uuid`) of the scheduled job on success.
 * 
 * @throws Will throw an error if:
 * - Input sharing fails due to invalid or incompatible input data.
 * - Encryption of shares fails due to issues with the encryption keys.
 * - The job scheduling API call fails due to server-side or network issues.
 * - The specified curve is unsupported.
 *
 * @example
 * ```ts
 * const configParams: ConfigurationParameters = {
 *   basePath: "https://proof.taceo.network",
 * }
 * const configuration = new Configuration(configParams)
 * const jobInstance = new JobApi(configuration);
 * const nodeInstance = new NodeApi(configuration);
 * const nodes = await nodeInstance.randomNodeProviders();
 * const blueprintId = "54f9ee38-0160-44e2-a1d8-08d1b6771cbf";
 * const voucher = null; // only required for Restricted blueprints
 * const publicInputs = ["public_input_1", "public_input_2"];
 * const input = JSON.parse(fs.readFileSync("input.json").toString());
 * const jobId = await CoCircom.scheduleFullJob(
 *   jobInstance,
 *   nodes,
 *   blueprintId,
 *   voucher,
 *   "Bn254",
 *   publicInputs,
 *   input
 * );
 * ```
 */
export async function scheduleFullJob(
  apiInstance: JobApi,
  nodes: NodeProviders,
  blueprintId: string,
  voucher: string | null,
  curve: BlueprintCurve,
  publicInputs: string[],
  input: any
): Promise<string> {
  const keys = [nodes.node0.encKey, nodes.node1.encKey, nodes.node2.encKey];
  let shares;
  switch (curve) {
    case BlueprintCurve.Bn254:
      shares = co_circom_split_input_bn254(keys, input, publicInputs);
      break;
    case BlueprintCurve.Bls381:
      shares = co_circom_split_input_bls12_381(keys, input, publicInputs);
      break;
    case BlueprintCurve.Bls377:
      throw "Bls377 is not supported for scheduleFullJob";
  }
  const scheduleJobResponse = await apiInstance.scheduleFullJob({
    blueprintId,
    node0: nodes.node0.id,
    node1: nodes.node1.id,
    node2: nodes.node2.id,
    voucher,
    inputs0: new Blob([shares.share0]),
    inputs1: new Blob([shares.share1]),
    inputs2: new Blob([shares.share2])
  });
  return scheduleJobResponse.jobId;
}

/**
 * Schedule a full job with multiple inputs and an optional deadline.
 *
 * This function schedules a job using the Rep3 Secret Sharing scheme, allowing
 * for multiple inputs to be provided. It sends the job scheduling request to the
 * specified nodes, which will execute the job based on the provided blueprint.
 * Users can also specify an optional deadline for the job execution.
 *
 * @param apiInstance - An instance of the JobApi client to interact with the job scheduling API.
 * @param nodes - A set of node providers that will execute the job. Each node must include an
 *                encryption key (`encKey`) and a unique identifier (`id`).
 * @param blueprintId - The unique identifier of the blueprint for the job.
 * @param voucher - An optional voucher for non-public blueprints. Pass `null` if not required.
 * @param deadline - An optional deadline for the job execution. Pass `null` if no deadline is needed.
 * 
 * @returns The id (`Uuid`) of the scheduled job on success.
 * 
 * @throws Will throw an error if:
 * - The job scheduling API call fails due to server-side or network issues.
 *
 * @example
 * ```ts
 * const configParams: ConfigurationParameters = {
 *   basePath: "https://proof.taceo.network",
 * }
 * const configuration = new Configuration(configParams)
 * const jobInstance = new JobApi(configuration);
 * const nodeInstance = new NodeApi(configuration);
 * const nodes = await nodeInstance.randomNodeProviders();
 * const blueprintId = "54f9ee38-0160-44e2-a1d8-08d1b6771cbf";
 * const voucher = null; // only required for Restricted blueprints
 * const deadline = new Date(Date.now() + 3600 * 1000); // 1 hour from now
 * const jobId = await CoCircom.scheduleFullMultipleInputsJob(
 *   jobInstance,
 *   nodes,
 *   blueprintId,
 *   voucher,
 *   deadline
 * );
 * ```
 */
export async function scheduleFullMultipleInputsJob(
  apiInstance: JobApi,
  nodes: NodeProviders,
  blueprintId: string,
  voucher: string | null,
  deadline: Date | null
): Promise<string> {
  const scheduleJobResponse = await apiInstance.scheduleFullMultipleInputsJob({
    blueprintId,
    node0: nodes.node0.id,
    node1: nodes.node1.id,
    node2: nodes.node2.id,
    voucher,
    deadline,
  });
  return scheduleJobResponse.jobId;
}

/**
 * Add inputs to an existing job.
 *
 * This function adds inputs to a job using the Rep3 Secret Sharing scheme. It splits the input
 * into shares, encrypts the shares, and sends them to the respective nodes for the specified job.
 * This allows for dynamic addition of inputs to an already scheduled job, enabling more flexible
 * workflows and computations.
 *
 * @param apiInstance - An instance of the JobApi client to interact with the job scheduling API.
 * @param nodes - A set of node providers that will execute the job. Each node must include an
 *                encryption key (`encKey`) and a unique identifier (`id`).
 * @param jobId - The unique identifier of the job to which inputs will be added.
 * @param curve - The elliptic curve used in the blueprint's circuit. Supported curves are `Bn254`
 *                and `Bls381`. `Bls377` is not supported for this function.
 * @param publicInputs - A list of public input names for the blueprint's circuit. These names must
 *                       match the circuit's expected public input identifiers.
 * @param input - The input data to be shared and used in the job. The data should be formatted
 *                as required by the blueprint's circuit. It can include private inputs as well.
 * 
 * @returns A promise that resolves when the inputs have been successfully added to the job.
 * 
 * @throws Will throw an error if:
 * - Input sharing fails due to invalid or incompatible input data.
 * - Encryption of shares fails due to issues with the encryption keys.
 * - The specified curve is unsupported.
 *
 * @example
 * ```ts
 * const configParams: ConfigurationParameters = {
 *   basePath: "https://proof.taceo.network",
 * }
 * const configuration = new Configuration(configParams);
 * const jobInstance = new JobApi(configuration);
 * const nodeInstance = new NodeApi(configuration);
 * // fetch the nodes selected when the job was scheduled
 * const nodes = await Promise.all([0, 1, 2].map(async id => await nodeInstance.nodeProvider({ id })));
 * const jobId = "9c2814d7-25d3-4de5-b61f-0a6e3bacbe99";
 * const publicInputs = ["public_input_1", "public_input_2"];
 * const input = JSON.parse(fs.readFileSync("input.json").toString());
 * await CoCircom.addJobInputs(jobInstance, nodes, jobId, "Bn254", publicInputs, input);
 * ```
 */
export async function addJobInputs(
  apiInstance: JobApi,
  nodes: NodeProviders,
  jobId: string,
  curve: BlueprintCurve,
  publicInputs: string[],
  input: any
): Promise<void> {
  const keys = [nodes.node0.encKey, nodes.node1.encKey, nodes.node2.encKey];
  let shares;
  switch (curve) {
    case BlueprintCurve.Bn254:
      shares = co_circom_split_input_bn254(keys, input, publicInputs);
      break;
    case BlueprintCurve.Bls381:
      shares = co_circom_split_input_bls12_381(keys, input, publicInputs);
      break;
    case BlueprintCurve.Bls377:
      throw "Bls377 is not supported for addJobInputs";
  }
  await apiInstance.addInputs({
    jobId,
    node0: nodes.node0.id,
    node1: nodes.node1.id,
    node2: nodes.node2.id,
    inputs0: new Blob([shares.share0]),
    inputs1: new Blob([shares.share1]),
    inputs2: new Blob([shares.share2])
  });
}

/**
 * Schedule a proof generation job.
 *
 * This function schedules a proof job using either the Rep3 or Shamir Secret Sharing scheme,
 * depending on the specified `mpcProtocol`. It takes a witness, splits it into shares,
 * encrypts the shares, and sends them to the respective nodes for execution.
 *
 * @param apiInstance - An instance of the JobApi client to interact with the job scheduling API.
 * @param nodes - A set of node providers that will execute the job. Each node must include an
 *                encryption key (`encKey`) and a unique identifier (`id`).
 * @param blueprintId - The unique identifier of the blueprint for the proof job.
 * @param mpcProtocol - The MPC protocol to be used for the proof generation. Supported protocols
 *                      are defined in the `MpcProtocol` enumeration.
 * @param voucher - An optional voucher for non-public blueprints. Pass `null` if not required.
 * @param curve - The elliptic curve used in the blueprint's circuit. Supported curves include `Bn254`, `Bls381`, and `Bls377`.
 * @param numPubInputs - The number of public inputs for the blueprint's circuit.
 * @param witness - The witness data to be used in the proof. This should be a `Uint8Array` formatted as required by the blueprint's circuit.
 * 
 * @returns The id (`Uuid`) of the scheduled job on success.
 * 
 * @throws Will throw an error if:
 * - Witness sharing fails due to invalid or incompatible witness data.
 * - Encryption of shares fails due to issues with the encryption keys.
 * - The job scheduling API call fails due to server-side or network issues.
 *
 * @example
 * ```ts
 * const configParams: ConfigurationParameters = {
 *   basePath: "https://proof.taceo.network",
 * };
 * const configuration = new Configuration(configParams);
 * const jobInstance = new JobApi(configuration);
 * const nodeInstance = new NodeApi(configuration);
 * const nodes = await nodeInstance.randomNodeProviders();
 * const blueprintId = "54f9ee38-0160-44e2-a1d8-08d1b6771cbf";
 * const mpcProtocol = "Rep3"; // specify the MPC protocol to use (Rep3 or Shamir)
 * const voucher = null; // only required for restricted blueprints
 * const numPubInputs = 2;
 * const witness = new Uint8Array(fs.readFileSync(inputPath));
 * const jobId = await CoCircom.scheduleProveJob(
 *   jobInstance,
 *   nodes,
 *   blueprintId,
 *   mpcProtocol, 
 *   voucher,
 *   "Bn254",
 *   numPubInputs,
 *   witness
 * );
 * ```
 */
export async function scheduleProveJob(
  apiInstance: JobApi,
  nodes: NodeProviders,
  blueprintId: string,
  mpcProtocol: MpcProtocol,
  voucher: string | null,
  curve: BlueprintCurve,
  numPubInputs: number,
  witness: Uint8Array
): Promise<string> {
  const keys = [nodes.node0.encKey, nodes.node1.encKey, nodes.node2.encKey];
  let shares;
  switch (curve) {
    case BlueprintCurve.Bn254:
      shares = co_circom_split_witness_bn254(keys, mpcProtocol, witness, numPubInputs);
      break;
    case BlueprintCurve.Bls381:
      shares = co_circom_split_witness_bls12_381(keys, mpcProtocol, witness, numPubInputs);
      break;
    case BlueprintCurve.Bls377:
      shares = co_circom_split_witness_bls12_377(keys, mpcProtocol, witness, numPubInputs);
      break;
  }
  const scheduleJobResponse = await apiInstance.scheduleProveJob({
    blueprintId: blueprintId,
    mpcProtocol,
    node0: nodes.node0.id,
    node1: nodes.node1.id,
    node2: nodes.node2.id,
    voucher,
    witness0: new Blob([shares.share0]),
    witness1: new Blob([shares.share1]),
    witness2: new Blob([shares.share2])
  });
  return scheduleJobResponse.jobId;
}

/**
 * Fetch the result of a job execution using a WebSocket connection.
 *
 * This function connects to the specified WebSocket `url` and subscribes to updates
 * for the job identified by `jobId`. It listens for messages from the server and
 * processes them based on the provided `stopStrategy`. The function returns the
 * proof, public inputs, and signatures of the job execution if successful, or an error
 * if the job fails or the connection is closed unexpectedly.
 *
 * @param url - The WebSocket URL to connect to for job updates.
 * @param jobId - The UUID of the job whose results are being fetched.
 * @param stopStrategy - The strategy to determine when to stop waiting for job results.
 *
 * @returns A promise that resolves to an object containing:
 * - `proof`: A `Groth16Proof` object representing the proof of the job.
 * - `public_inputs`: An array of scalar field elements representing the public inputs.
 * - `signatures`: A record mapping node IDs to their respective `Uint8Array` signatures.
 *
 * @throws Will throw an error if:
 * - The WebSocket connection fails to establish.
 * - The server sends invalid or unexpected data.
 * - The job fails, is cancelled, or encounters an error on a node.
 * - The server closes the WebSocket connection unexpectedly.
 * - Deserialization of the proof or public inputs fails.
 *
 * @example
 * ```ts
 * const wsUrl = "wss://proof.taceo.network/api/v1/reports/subs";
 * const jobId = "9c2814d7-25d3-4de5-b61f-0a6e3bacbe99";
 * const { proof, public_inputs, signatures } = await CoCircom.fetchJobResult(wsUrl, jobId);
 * ```
 */
export async function fetchJobResult(
  url: string,
  jobId: string,
  stopStrategy?: StopStrategy,
): Promise<{ proof: Groth16Proof, public_inputs: PublicSignals, signatures: Record<number, Uint8Array> }> {
  const result = await wsFetchResult(url, jobId, stopStrategy);
  const proof = JSON.parse(result.proof);
  const public_inputs = JSON.parse(result.public_inputs);
  const signatures: Record<number, Uint8Array> = Object.fromEntries(
    Object.entries(result.signatures).map(([key, value]) => [key, Base64.toUint8Array(value)])
  );
  return { proof, public_inputs, signatures };
}
