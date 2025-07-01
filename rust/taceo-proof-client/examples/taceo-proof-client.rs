use std::{fs::File, path::PathBuf, time::Duration};

use ark_bls12_377::Bls12_377;
use ark_bls12_381::Bls12_381;
use ark_bn254::Bn254;
use ark_ec::pairing::Pairing;
use circom_types::{
    R1CS, Witness,
    traits::{CircomArkworksPairingBridge, CircomArkworksPrimeFieldBridge},
};
use clap::{ArgGroup, Parser, ValueEnum};
use taceo_proof_api_client::{
    apis::{configuration::Configuration, job_api},
    models::JobStatus,
};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum JobType {
    Rep3Full,
    Rep3Prove,
    ShamirProve,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Curve {
    Bn254,
    Bls381,
    Bls377,
}

#[derive(Parser, Debug)]
#[clap(group(
    ArgGroup::new("exclusive_args")
        .args(&["r1cs", "num_inputs"])
        .required(true)
))]
struct Args {
    /// The job type
    pub job: JobType,

    /// The API endpoint URL
    #[clap(long, env = "PROOF_API_URL", default_value = "http://localhost:1234")]
    pub api_url: String,

    /// The curve
    #[clap(long, env = "PROOF_CURVE")]
    pub curve: Curve,

    /// The path to the job input
    #[clap(long, env = "PROOF_INPUT")]
    pub input: PathBuf,

    /// The voucher for a proof job
    #[clap(long, env = "PROOF_VOUCHER")]
    pub voucher: Option<String>,

    /// The job blueprint
    #[clap(long, env = "PROOF_BLUEPRINT")]
    pub blueprint: Uuid,

    /// The path to the r1cs file
    #[clap(long, env = "PROOF_R1CS")]
    pub r1cs: Option<PathBuf>,

    /// The number of inputs for the circuit
    #[clap(long, env = "PROOF_NUM_INPUTS")]
    pub num_inputs: Option<usize>,

    /// The public inputs for witness extension
    #[clap(long, env = "PROOF_PUBLIC_INPUTS", required_if_eq("job", "rep3-full"))]
    pub public_inputs: Option<Vec<String>>,

    /// The output file where the final proof is written to
    #[arg(long, env = "PROOF_OUT", default_value = "proof.json")]
    pub out: PathBuf,

    /// The output JSON file where the public inputs are written to
    #[arg(long, env = "PROOF_OUT_PUBLIC_INPUTS", default_value = "public.json")]
    pub out_public_inputs: PathBuf,
}

async fn run<P>(config: &Configuration, args: Args) -> eyre::Result<()>
where
    P: Pairing + CircomArkworksPairingBridge,
    P::ScalarField: CircomArkworksPrimeFieldBridge,
    P::BaseField: CircomArkworksPrimeFieldBridge,
{
    let num_inputs = if let Some(r1cs) = args.r1cs {
        let r1cs = R1CS::<P>::from_reader(File::open(r1cs)?)?;
        r1cs.num_inputs
    } else {
        args.num_inputs.expect("must be present if r1cs is not")
    };

    let keys = taceo_proof_client::get_nps_key_material(config, args.blueprint).await?;

    tracing::info!("scheduling job...");
    let job_id = match args.job {
        JobType::Rep3Full => {
            let input = serde_json::from_reader(File::open(args.input)?)?;
            taceo_proof_client::schedule_full_job_rep3::<P>(
                config,
                args.blueprint,
                args.voucher.as_deref(),
                &keys,
                input,
                &args
                    .public_inputs
                    .expect("must be present if job is Rep3Full"),
            )
            .await?
        }
        JobType::Rep3Prove => {
            let witness = Witness::from_reader(File::open(args.input)?)?;
            taceo_proof_client::schedule_prove_job_rep3::<P>(
                config,
                args.blueprint,
                args.voucher.as_deref(),
                &keys,
                witness,
                num_inputs,
            )
            .await?
        }
        JobType::ShamirProve => {
            let witness = Witness::from_reader(File::open(args.input)?)?;
            taceo_proof_client::schedule_prove_job_shamir::<P>(
                config,
                args.blueprint,
                args.voucher.as_deref(),
                &keys,
                witness,
                num_inputs,
            )
            .await?
        }
    };

    let (proof, public_inputs) = loop {
        let results = job_api::get_results(config, &job_id.to_string()).await?;
        tracing::debug!("result from api: {results:?}");
        match results.result0.status {
            JobStatus::Success => {
                // contains the proof and public_inputs as JSON strings for a CircomGroth16 proof
                // or as base64 encoded ark_serialize serialized bytes for a LibsnarkGroth16 proof
                let proof_res = results.result0.ok.unwrap().unwrap();
                break (proof_res.proof, proof_res.public_inputs);
            }
            JobStatus::Failed => eyre::bail!(results.result0.error.unwrap().unwrap()),
            _ => {
                tracing::info!("waiting for result...");
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    };

    std::fs::write(&args.out, proof)?;
    tracing::info!("Wrote proof to {}", args.out.display());

    std::fs::write(&args.out_public_inputs, public_inputs)?;
    tracing::info!(
        "Wrote public inputs to {}",
        args.out_public_inputs.display()
    );

    Ok(())
}

fn install_tracing() {
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{EnvFilter, fmt};

    let fmt_layer = fmt::layer().with_target(false).with_line_number(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    install_tracing();
    let args = Args::parse();
    let config = Configuration {
        base_path: args.api_url.clone(),
        ..Default::default()
    };

    match args.curve {
        Curve::Bn254 => run::<Bn254>(&config, args).await?,
        Curve::Bls381 => run::<Bls12_381>(&config, args).await?,
        Curve::Bls377 => run::<Bls12_377>(&config, args).await?,
    };

    Ok(())
}
