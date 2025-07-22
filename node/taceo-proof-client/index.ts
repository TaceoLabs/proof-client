import { BlueprintCurve, Configuration, ConfigurationParameters, fetchJobResult, JobApi, JobType, NodeApi, scheduleFullJobRep3, scheduleProveJobRep3, scheduleProveJobShamir } from "@taceo/proof-client-node";
import * as fs from 'fs';
import { Command } from 'commander';

async function main() {
  const program = new Command();
  program
    .argument('<type>', 'The job type')
    .option('--api-url <url>', 'The API endpoint URL', 'https://proof.taceo.network')
    .requiredOption('--curve <curve>', 'The curve')
    .requiredOption('--input <path>', 'The path to the job input')
    .option('--voucher <voucher>', 'The voucher for a proof job')
    .requiredOption('--blueprint <uuid>', 'The job blueprint')
    .option('--r1cs <path>', 'The path to the r1cs file')
    .option('--num-inputs <number>', 'The number of inputs for the circuit', parseInt)
    .option('--public-inputs <values...>', 'The public inputs for witness extension')
    .option('--out <path>', 'The output file where the final proof is written to', 'proof.json')
    .option('--out-public-inputs <path>', 'The output JSON file where the public inputs are written to', 'public.json');

  program.parse();

  const options = program.opts();

  const jobType = options.job as JobType;
  const apiUrl = options.apiUrl;
  const curve = options.curve;
  const inputPath = options.input;
  const voucher = options.voucher;
  const blueprint = options.blueprint as BlueprintCurve;
  const numInputs = options.numInputs;
  const publicInputs = options.publicInputs;
  const outPath = options.out;
  const outPublicInputsPath = options.outPublicInputs;

  const configParams: ConfigurationParameters = {
    basePath: apiUrl,
  }
  const configuration = new Configuration(configParams)
  const jobInstance = new JobApi(configuration);
  const nodeInstance = new NodeApi(configuration);

  const nodes = await nodeInstance.randomNodeProviders();

  let jobId;

  if (jobType == "Rep3Full") {
    const input = new Uint8Array(fs.readFileSync(inputPath));
    jobId = await scheduleFullJobRep3(jobInstance, nodes, blueprint, voucher, curve, publicInputs, input);
  } else if (jobType == "Rep3Prove") {
    const witness = new Uint8Array(fs.readFileSync(inputPath));
    jobId = await scheduleProveJobRep3(jobInstance, nodes, blueprint, voucher, curve, numInputs, witness);
  } else {
    const witness = new Uint8Array(fs.readFileSync(inputPath));
    jobId = await scheduleProveJobShamir(jobInstance, nodes, blueprint, voucher, curve, numInputs, witness);
  }

  console.log("scheduled job %s", jobId);

  const jobResult = await fetchJobResult("wss://proof.taceo.network/api/v1/reports/subs", jobId);

  fs.writeFileSync(outPath, jobResult.proof);
  console.log("wrote proof to {%s}", outPath);
  fs.writeFileSync(outPublicInputsPath, jobResult.public_inputs);
  console.log("wrote public inputs to {%s}", outPublicInputsPath);
}

main()

