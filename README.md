# TACEO:Proof Client
> A client library for interacting with TACEO:Proof.

## Overview
This repository provides a simple and consistent client for communicating with **TACEO:Proof**.

We offer multiple client implementations to best fit your environment and use case.

TACEO:Proof distinguishes between two types of users: _Blueprint Creators_ and _End-Users_.
Blueprint Creators upload circuits and provide cryptographic material (circuit blueprints), requiring authentication with TACEO:Proof. End-Users can then generate proofs based on these blueprints.

In the primary use case, End-Users interact with services provided by Blueprint Creators and outsource proof generation to TACEO:Proof.

This repository focuses on helping Blueprint Creators integrate a client for TACEO:Proof, demonstrating the full workflow — from authenticating to computing a proof using TACEO’s coSNARK-engine.

It is designed to help you seamlessly integrate the API into your project and enable your users to leverage the power of outsourced proving.

## Features
 - Easy-to-use API for accessing TACEO:Proof.
 - Fully typed and documented.
 - Client implementations in different languages (if you want that we add a language please open an issue, we plan to support languages on-demand).

## Getting Started
### Installation

## How It Works
When interacting with **TACEO:Proof**, the process differs slightly for _Blueprint Creators_ and _End-Users_.

### For Blueprint Creators
(_These steps are typically performed through the web interface, not via this client library._)

1) **Authenticate via password-based login**
(_Note: This is subject to change. We plan to support OAuth in the future. Open an issue if you require a different authentication method._)
2) **Create a new blueprint**
    * Set a name for the blueprint.
    * Define the proof type (currently supported: Circom-Groth16, Groth16 with Libsnark reduction).
    * Receive a unique identifier for the blueprint. (_This ID is later used by End-Users to schedule coSNARKs._)
    * Receive the public keys of the Node Providers responsible for generating coSNARKs.
3) **Upload auxiliary data**
    * Upload the proving key (ZKey), verification key, Circom files, and any other necessary data.

Once these steps are complete, End-Users can begin scheduling coSNARK generation based on the uploaded blueprint.

### For End-Users
(_These steps are performed using this client library._)
1) **Authenticate through your service**
End-Users authenticate with your system (out-of-band from TACEO:Proof). (_Blueprint Creators are responsible for authenticating their users according to their own requirements._)
2) **Issue a Voucher**
If the End-User is eligible to request a coSNARK, your service issues a _Voucher_.
    * Specify a time limit and a maximum usage count for the Voucher.
3) **Request coSNARK generation**
The End-User submits the blueprint ID and their Voucher to TACEO:Proof.
    * In response, they receive a unique identifier for the coSNARK job.
4) **Secret-share and encrypt inputs**
The End-User secret-shares their private inputs, encrypts the shares using the Node Providers' public keys, and uploads the encrypted shares to TACEO:Proof.
5) **Poll for completion**
The End-User periodically polls TACEO:Proof to check the status of the coSNARK job.
Once complete, they retrieve the proof and the corresponding public inputs for verification.

Compared to generating the proof locally, the End-User simply:
* Performs two HTTP requests,
* Secret-shares their private input,
* Encrypts the shares — and lets TACEO:Proof handle the proving process.

## Supported Languages

We currently offer official client libraries for:

* Rust
* JavaScript/TypeScript

If you would like to see support for additional languages, please feel free to open an [https://github.com/TaceoLabs/proof-client/issues](issue) and let us know!
We’re happy to prioritize new language bindings based on community demand.
