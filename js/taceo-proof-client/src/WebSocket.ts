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
interface SubscribeExecutionRequest {
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
type JobStatus =
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
interface FailedReason {
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
type WebSocketMessage =
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
export async function fetchJobResult(
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
