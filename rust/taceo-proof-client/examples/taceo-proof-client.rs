use std::{fs::File, path::PathBuf, time::Instant};

use ark_bls12_377::Bls12_377;
use ark_bls12_381::Bls12_381;
use ark_bn254::Bn254;
use clap::{ArgGroup, Parser, Subcommand, ValueEnum};
use taceo_proof_client::{
    SignedResults, StopStrategy,
    apis::configuration::Configuration,
    ark_ec::pairing::Pairing,
    circom_types::{
        R1CS, Witness,
        traits::{CircomArkworksPairingBridge, CircomArkworksPrimeFieldBridge},
    },
    uuid::Uuid,
};

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

#[derive(Debug, Clone, Copy, ValueEnum)]
enum MpcProtocol {
    REP3,
    Shamir,
}

#[derive(Parser, Debug, Clone)]
struct FullProve {
    /// The API endpoint URL
    #[clap(
        long,
        env = "TACEO_PROOF_API_URL",
        default_value = "https://proof.taceo.network"
    )]
    pub api_url: String,

    /// The curve
    #[clap(long, env = "PROOF_CURVE")]
    pub curve: Curve,

    /// The voucher for a proof job
    #[clap(long, env = "PROOF_VOUCHER")]
    pub voucher: Option<String>,

    /// The job blueprint
    #[clap(long, env = "PROOF_BLUEPRINT")]
    pub blueprint: Uuid,

    /// The path to the job input
    #[clap(long, env = "PROOF_INPUT")]
    pub input: PathBuf,

    /// The public inputs for witness extension
    #[clap(long, env = "PROOF_PUBLIC_INPUTS")]
    pub public_inputs: Option<Vec<String>>,

    /// The output file where the final proof is written to
    #[arg(long, env = "PROOF_OUT", default_value = "proof.json")]
    pub out: PathBuf,

    /// The output JSON file where the public inputs are written to
    #[arg(long, env = "PROOF_OUT_PUBLIC_INPUTS", default_value = "public.json")]
    pub out_public_inputs: PathBuf,
}

#[derive(Parser, Debug, Clone)]
#[clap(group(
    ArgGroup::new("exclusive_args")
        .args(&["r1cs", "num_inputs"])
        .required(true)
))]
struct Prove {
    /// The API endpoint URL
    #[clap(
        long,
        env = "TACEO_PROOF_API_URL",
        default_value = "https://proof.taceo.network"
    )]
    pub api_url: String,

    /// The curve
    #[clap(long, env = "PROOF_CURVE")]
    pub curve: Curve,

    /// The voucher for a proof job
    #[clap(long, env = "PROOF_VOUCHER")]
    pub voucher: Option<String>,

    /// The job blueprint
    #[clap(long, env = "PROOF_BLUEPRINT")]
    pub blueprint: Uuid,

    /// The MPC protocol
    #[clap(long, env = "PROOF_MPC_PROTOCOL", default_value = "rep3")]
    pub protocol: MpcProtocol,

    /// The path to the witness file
    #[clap(long, env = "PROOF_WITNESS")]
    pub witness: PathBuf,

    /// The path to the r1cs file
    #[clap(long, env = "PROOF_R1CS")]
    pub r1cs: Option<PathBuf>,

    /// The number of inputs for the circuit
    #[clap(long, env = "PROOF_NUM_INPUTS")]
    pub num_inputs: Option<usize>,

    /// The output file where the final proof is written to
    #[arg(long, env = "PROOF_OUT", default_value = "proof.json")]
    pub out: PathBuf,

    /// The output JSON file where the public inputs are written to
    #[arg(long, env = "PROOF_OUT_PUBLIC_INPUTS", default_value = "public.json")]
    pub out_public_inputs: PathBuf,
}

#[derive(Debug, Clone, Subcommand)]
enum Commands {
    /// Schedule a full coSNARK job including witness extension
    FullProve(FullProve),
    /// Schedule a prove coSNARK job
    Prove(Prove),
}

impl Commands {
    pub fn curve(&self) -> Curve {
        match self {
            Commands::FullProve(full_prove) => full_prove.curve,
            Commands::Prove(prove) => prove.curve,
        }
    }

    pub fn api_url(&self) -> String {
        match self {
            Commands::FullProve(full_prove) => full_prove.api_url.clone(),
            Commands::Prove(prove) => prove.api_url.clone(),
        }
    }

    pub fn out(&self) -> PathBuf {
        match self {
            Commands::FullProve(full_prove) => full_prove.out.clone(),
            Commands::Prove(prove) => prove.out.clone(),
        }
    }

    pub fn out_public_inputs(&self) -> PathBuf {
        match self {
            Commands::FullProve(full_prove) => full_prove.out_public_inputs.clone(),
            Commands::Prove(prove) => prove.out_public_inputs.clone(),
        }
    }
}

#[derive(Parser, Debug, Clone)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

async fn run<P>(config: &Configuration, args: Args) -> eyre::Result<()>
where
    P: Pairing + CircomArkworksPairingBridge,
    P::ScalarField: CircomArkworksPrimeFieldBridge,
    P::BaseField: CircomArkworksPrimeFieldBridge,
{
    let nodes = taceo_proof_client::get_random_node_providers(config).await?;

    let start = Instant::now();
    let job_id = match args.command.clone() {
        Commands::FullProve(args) => {
            let input = serde_json::from_reader(File::open(args.input)?)?;
            taceo_proof_client::schedule_full_job_rep3::<P>(
                config,
                &nodes,
                args.blueprint,
                args.voucher.as_deref(),
                &args
                    .public_inputs
                    .expect("must be present if job is Rep3Full"),
                input,
            )
            .await?
        }
        Commands::Prove(args) => {
            let num_inputs = if let Some(r1cs) = args.r1cs {
                let r1cs = R1CS::<P>::from_reader(File::open(r1cs)?)?;
                r1cs.num_inputs
            } else {
                args.num_inputs.expect("must be present if r1cs is not")
            };
            let witness = Witness::from_reader(File::open(args.witness)?)?;
            match args.protocol {
                MpcProtocol::REP3 => {
                    taceo_proof_client::schedule_prove_job_rep3::<P>(
                        config,
                        &nodes,
                        args.blueprint,
                        args.voucher.as_deref(),
                        num_inputs,
                        witness,
                    )
                    .await?
                }
                MpcProtocol::Shamir => {
                    taceo_proof_client::schedule_prove_job_shamir::<P>(
                        config,
                        &nodes,
                        args.blueprint,
                        args.voucher.as_deref(),
                        num_inputs,
                        witness,
                    )
                    .await?
                }
            }
        }
    };
    tracing::info!("scheduled job {job_id}");

    let ws_url = args
        .command
        .api_url()
        .replace("http", "ws")
        .replace("https", "wss")
        + "/api/v1/reports/subs";
    // contains the proof and public_inputs as JSON strings for a CircomGroth16 proof
    // or as base64 encoded ark_serialize serialized bytes for a LibsnarkGroth16 proof
    let SignedResults {
        signatures: _,
        proof,
        public_inputs,
    } = taceo_proof_client::fetch_job_result(&ws_url, job_id, StopStrategy::default()).await?;
    tracing::info!("job took {}s", start.elapsed().as_secs_f64());

    let out = args.command.out();
    let out_public_inputs = args.command.out_public_inputs();

    std::fs::write(&out, proof)?;
    tracing::info!("wrote proof to {}", out.display());

    std::fs::write(&out_public_inputs, public_inputs)?;
    tracing::info!("wrote public inputs to {}", out_public_inputs.display());

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
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    let args = Args::parse();
    let config = Configuration {
        base_path: args.command.api_url(),
        ..Default::default()
    };

    match args.command.curve() {
        Curve::Bn254 => run::<Bn254>(&config, args).await?,
        Curve::Bls381 => run::<Bls12_381>(&config, args).await?,
        Curve::Bls377 => run::<Bls12_377>(&config, args).await?,
    };

    Ok(())
}
