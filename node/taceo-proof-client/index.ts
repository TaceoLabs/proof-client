import { CoCircom, CoNoir, BlueprintCurve, Configuration, ConfigurationParameters, JobApi, MpcProtocol, NodeApi } from "@taceo/proof-client-node";
import * as fs from 'fs';
import { Command } from 'commander';
import { exit } from "process";

type CoSnark = "co-circom" | "co-noir";
type JobType = "full" | "prove";

async function main() {
  const program = new Command();
  program
    .option('--api-url <url>', 'The API endpoint URL', 'https://proof.taceo.network')
    .requiredOption('--co-snark <co-snark>', 'The coSNARK (co-circom, co-noir)')
    .requiredOption('--job-type <type>', 'The job type (full, prove)')
    .option('--mpc-protocol <mpc-protocol>', 'The MPC protocol to use (rep3, shamir)', 'rep3')
    .requiredOption('--curve <curve>', 'The curve')
    .requiredOption('--input <path>', 'The path to the job input')
    .option('--voucher <voucher>', 'The voucher for a proof job')
    .requiredOption('--blueprint <uuid>', 'The job blueprint')
    .option('--abi <path>', 'Path to a json file with the Noir program ABI')
    .option('--num-inputs <number>', 'The number of inputs for the circuit', parseInt)
    .option('--public-inputs <values...>', 'The public inputs for witness extension')
    .option('--public-input-indices <values...>', 'The public input indices for witness extension')
    .option('--out <path>', 'The output file where the final proof is written to', 'proof')
    .option('--out-public-inputs <path>', 'The output file where the public inputs are written to', 'public_inputs');

  program.parse();

  const options = program.opts();

  const coSnark = options.coSnark as CoSnark;
  const jobType = options.jobType as JobType;
  const apiUrl = options.apiUrl;
  const mpcProtocol = options.mpcProtocol as MpcProtocol;
  const curve = options.curve as BlueprintCurve;
  const inputPath = options.input;
  const voucher = options.voucher;
  const blueprint = options.blueprint;
  const abiPath = options.abi;
  const numInputs = options.numInputs ? options.numInputs as number : 0;
  const publicInputs = options.publicInputs ? options.publicInputs as string[] : [];
  const publicInputIndices = options.publicInputIndices ? options.publicInputIndices as Uint32Array : new Uint32Array();
  const outPath = options.out;
  const outPublicInputsPath = options.outPublicInputs;

  const configParams: ConfigurationParameters = {
    basePath: apiUrl,
  }
  const configuration = new Configuration(configParams)
  const jobInstance = new JobApi(configuration);
  const nodeInstance = new NodeApi(configuration);

  const websocketUrl = apiUrl.replace(/^https:/, 'wss:').replace(/^http:/, 'ws:') + "/api/v1/reports/subs";
  const nodes = await nodeInstance.randomNodeProviders();

  let jobId;

  if (coSnark == "co-circom") {
    if (jobType == "full") {
      const input = JSON.parse(fs.readFileSync(inputPath).toString());
      jobId = await CoCircom.scheduleFullJob(jobInstance, nodes, blueprint, voucher, curve, publicInputs, input);
    } else if (jobType == "prove") {
      const witness = new Uint8Array(fs.readFileSync(inputPath));
      jobId = await CoCircom.scheduleProveJob(jobInstance, nodes, blueprint, mpcProtocol, voucher, curve, numInputs, witness);
    } else {
      console.error("invalid job type %s", jobType);
      exit(1);
    }
    console.log("scheduled job %s", jobId);
    const jobResult = await CoCircom.fetchJobResult(websocketUrl, jobId);
    fs.writeFileSync(outPath, JSON.stringify(jobResult.proof));
    console.log("wrote proof to %s", outPath);
    fs.writeFileSync(outPublicInputsPath, JSON.stringify(jobResult.public_inputs));
  } else if (coSnark == "co-noir") {
    if (jobType == "full") {
      const input = JSON.parse(fs.readFileSync(inputPath).toString());
      const abi = JSON.parse(fs.readFileSync(abiPath).toString());
      jobId = await CoNoir.scheduleFullJob(jobInstance, nodes, blueprint, voucher, abi, publicInputIndices, input);
    } else if (jobType == "prove") {
      const witness = new Uint8Array(fs.readFileSync(inputPath));
      jobId = await CoNoir.scheduleProveJob(jobInstance, nodes, blueprint, mpcProtocol, voucher, publicInputIndices, witness);
    } else {
      console.error("invalid job type %s", jobType);
      exit(1);
    }
    console.log("scheduled job %s", jobId);
    const jobResult = await CoNoir.fetchJobResult(websocketUrl, jobId);
    fs.writeFileSync(outPath, jobResult.proof);
    console.log("wrote proof to %s", outPath);
    fs.writeFileSync(outPublicInputsPath, jobResult.public_inputs);
    console.log("wrote public inputs to %s", outPublicInputsPath);
  } else {
    console.error("invalid coSNARK %s", coSnark);
    exit(1);
  }
}

main()

