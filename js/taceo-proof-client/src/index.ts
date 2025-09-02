import { verify_proof_result_signature } from '../taceo-proof-wasm/pkg/taceo_proof_wasm.js';

export { JobApi, MpcProtocol, NodeApi, NodeProviders, NodeProvider, BlueprintCurve, Configuration, ConfigurationParameters } from '@taceo/proof-api-client';
export { type Groth16Proof, type PublicSignals } from "snarkjs";
export * as CoCircom from "./CoCircom";
export * as CoNoir from "./CoNoir";

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
