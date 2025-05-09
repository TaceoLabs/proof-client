import { JobApi, Configuration, ConfigurationParameters, BlueprintCurve, BlueprintApi } from "@taceo/proof-api-client";
import { getEncKeysB64, scheduleProveJobRep3 } from "../index";
import { readFileSync } from 'fs';

const configParams: ConfigurationParameters = {
  basePath: 'http://localhost:3000',
  accessToken: '123456',
}

const congiuration = new Configuration(configParams)
const jobInstance = new JobApi(congiuration);
const blueprintInstance = new BlueprintApi(congiuration);

async function main() {
  const code = "foo";
  const blueprintId = "id";
  const numPublicInputs = 2;
  const witness = readFileSync("foo");

  const keys = await getEncKeysB64(blueprintInstance, blueprintId);
  const jobId = scheduleProveJobRep3(
    jobInstance,
    blueprintId,
    code,
    BlueprintCurve.Bn254,
    keys,
    numPublicInputs,
    witness
  );

  console.log("job id:", jobId);
}

main();

