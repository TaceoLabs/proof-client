import { JobApi } from "@taceo/csn-api-client";
export { JobApi, Configuration, ConfigurationParameters, JobResult } from "@taceo/csn-api-client";
/**
 * Schedule a full job including witness extension. The retuned job id can be used to query the job status.
 */
export declare function scheduleFullJob(apiInstance: JobApi, id: string, public_inputs: string[], input: any): Promise<string>;
/**
 * Schedule a proof job with a provided witness. The retuned job id can be used to query the job status.
 */
export declare function scheduleProofJob(apiInstance: JobApi, id: string, num_pub_inputs: number, witness: Uint8Array): Promise<string>;
