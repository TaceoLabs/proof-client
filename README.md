# TACEO:Proof Client
> A client library for interacting with TACEO:Proof.

## Overview
This repository provides multiple **TACEO:Proof** client implementations for communicating with the **TACEO:Proof** network.
It is designed to help you seamlessly integrate the API into your project and enable you to leverage the power of outsourced proving.

## Getting Started
Currently we offer client libraries for the following languages:
- **Rust**
    * Simply add the client as a git dependency to your `Cargo.toml`.
        ```toml
        taceo-proof-client = { git = "https://github.com/TaceoLabs/proof-client.git" }
        ```
    * Checkout the [example](./rust/taceo-proof-client/examples/taceo-proof-client.rs) to see how to use the client.

- **JavaScript/TypeScript**
    * You can install the client using your favorite package manager.
    * We publish two versions of the client that correspond to the `nodejs` and `bundler` [targets](https://rustwasm.github.io/docs/wasm-pack/commands/build.html#target) of [wasm-pack](https://github.com/rustwasm/wasm-pack).
    * If you want to use nodejs:
        ```bash
        npm install @taceo/proof-client-node
        ```
    * If you want to use the client in a app that uses a bundler:
        ```bash
        npm install @taceo/proof-client-bundler
        ```
    * Checkout the nodejs [example](./node/taceo-proof-client) and the react + bundler [examples](./react).

If you would like to see support for additional languages, please feel free to open an [issue](https://github.com/TaceoLabs/proof-client/issues) and let us know!
We’re happy to prioritize new language bindings based on community demand.

## How It Works
1) **Request set of Node Providers**

    The client gives you the ability to request a set of 3 distinct Node Providers of **TACEO:Proof**.
    You receive the Node Providers with their ids, encryption keys and verification keys.
    You are encouraged to request a new set of Node Providers for each coSNARK job or quick burst of multiple jobs.
    We do not recommend to use the same Node Providers over a longer period of time.

2) **Schedule a coSNARK Job on the TACEO:Proof Network**

    You can schedule different coSNARK types
    * Witness Extension + Prove
    * Prove only

    We and the community offer different coSNARK blueprints that are defined by their circuits and proving system.
    To create your own blueprints just reach out to us!

    To schedule a job you must provided the unique identifier of the blueprint that you want to use.
    In the client, the private inputs (Extended Witness or Inputs to Witness Extension, depends on the job type) get secret-shared and encrypted using the Node Providers' public keys (encryption keys).
    The client then schedules the coSNARK job and uploads the encrypted shares to **TACEO:Proof** where the job gets send to nodes that compute the proof with Multi-Party Computation (MPC).
    In response you receive a unique identifier for this job.
    All of this is handled by the client libraries, you just need to call the provided functions.

3) **Fetch coSNARK Job Results**

    Using the received unique identifier, you can subscribe to receive the results of a coSNARK job via a WebSocket connection.
    By calling the provided functions you can wait for the results without needed to do any manual polling.
    The returned results include the proofs which are signed by the Node Providers.
    You can use the initially received verification keys to verify the signatures.
