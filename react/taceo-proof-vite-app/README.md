# React + TypeScript + Vite + csn client

This repository contains a basic web client for the co-snarks-network.

# Setup

Install the depencies with.

```bash
npm install
```

Edit the API configuration [here](src/App.tsx) to the correct values.

```ts
const JOB_DEFINITION = "job-definition-id";
const ACCESS_TOKEN = "access-token";
const SERVER_URL = "https://csn.taceo.io"
```

## Build and run

> [!NOTE]  
> Currently `npm run dev` does not work, so just use `build` and `preview`

```bash
npm run build
npm run preview
```