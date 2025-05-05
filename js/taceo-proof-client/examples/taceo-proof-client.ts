import { JobApi, Configuration, ConfigurationParameters, BlueprintCurve } from "@taceo/proof-api-client";
import { scheduleProveJobRep3 } from "../index";
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

  const jobId = scheduleProveJobRep3(
    jobInstance,
    code,
    blueprintId,
    BlueprintCurve.Bn254,
    numPublicInputs,
    witness
  );

  console.log("job id:", jobId);
}

main();

