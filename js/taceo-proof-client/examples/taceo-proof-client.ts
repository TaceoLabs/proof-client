import { JobApi, Configuration, ConfigurationParameters } from "@taceo/proof-api-client";
import { scheduleProveJobRep3Bn254 } from "../index";
import { readFileSync } from 'fs';

const configParams: ConfigurationParameters = {
  basePath: 'http://localhost:3000',
  accessToken: '123456',
}

const congiuration = new Configuration(configParams)
const jobInstance = new JobApi(congiuration);

async function main() {
  const code = "foo";
  const blueprintId = "id";
  const numPublicInputs = 2;
  const witness = readFileSync("foo");

  const jobId = scheduleProveJobRep3Bn254(
    jobInstance,
    code,
    blueprintId,
    numPublicInputs,
    witness
  );

  console.log("job id:", jobId);
}

main();

