# csn-client

this repository contains a typescript/js client that can be used to schedule jobs on our co-snarks-network.

## Usage

```ts
import { ConfigurationParameters, Configuration, JobApi, scheduleFullJob } from '@taceo/csn-client';

const configParams: ConfigurationParameters = {
  basePath: 'http://localhost:3000',
  accessToken: '123456',
}

const congiuration = new Configuration(configParams)
const apiInstance = new JobApi(congiuration);

const public_inputs: string[] = [];

const input = {
  a: "2",
  b: "3"
};

const jobId = await scheduleFullJob(apiInstance, "job-definition-id", public_inputs, input);
```