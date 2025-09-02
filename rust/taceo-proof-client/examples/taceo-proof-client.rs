use std::{
    fs::File,
    path::{Path, PathBuf},
};

use ark_bls12_377::Bls12_377;
use ark_bls12_381::Bls12_381;
use ark_bn254::Bn254;
use ark_ec::pairing::Pairing;
use ark_ff::{PrimeField, Zero as _};
use circom_types::{
    groth16::CircomGroth16Proof,
    traits::{CircomArkworksPairingBridge, CircomArkworksPrimeFieldBridge},
};
use clap::{Parser, Subcommand, ValueEnum};
use noir_types::{HonkProof, SerializeF};
use taceo_proof_client::{
    NodeProviders, StopStrategy, apis::configuration::Configuration, circom_types::Witness,
    uuid::Uuid,
};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Curve {
    Bn254,
    Bls381,
    Bls377,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum MpcProtocol {
    Rep3,
    Shamir,
}

impl From<MpcProtocol> for taceo_proof_client::models::MpcProtocol {
    fn from(value: MpcProtocol) -> Self {
        match value {
            MpcProtocol::Rep3 => Self::Rep3,
            MpcProtocol::Shamir => Self::Shamir,
        }
    }
}

#[derive(Parser, Debug, Clone)]
struct CoCircomFullProve {
    /// The API endpoint URL
    #[clap(
        long,
        env = "PROOF_API_URL",
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
    pub public_inputs: Vec<String>,

    /// The output file where the final proof is written to
    #[arg(long, env = "PROOF_OUT", default_value = "proof.json")]
    pub out: PathBuf,

    /// The output JSON file where the public inputs are written to
    #[arg(long, env = "PROOF_OUT_PUBLIC_INPUTS", default_value = "public.json")]
    pub out_public_inputs: PathBuf,
}

#[derive(Parser, Debug, Clone)]
struct CoCircomFullMultipleInputs {
    /// The API endpoint URL
    #[clap(
        long,
        env = "PROOF_API_URL",
        default_value = "https://proof.taceo.network"
    )]
    pub api_url: String,

    /// The voucher for a proof job
    #[clap(long, env = "PROOF_VOUCHER")]
    pub voucher: Option<String>,

    /// The job blueprint
    #[clap(long, env = "PROOF_BLUEPRINT")]
    pub blueprint: Uuid,

    /// The job deadline
    #[clap(long, env = "PROOF_DEADLINE")]
    pub deadline: Option<chrono::DateTime<chrono::Local>>,
}

#[derive(Parser, Debug, Clone)]
struct CoCircomAddInputs {
    /// The API endpoint URL
    #[clap(
        long,
        env = "PROOF_API_URL",
        default_value = "https://proof.taceo.network"
    )]
    pub api_url: String,

    /// The curve
    #[clap(long, env = "PROOF_CURVE")]
    pub curve: Curve,

    /// The job id
    #[clap(long, env = "PROOF_JOB")]
    pub job: Uuid,

    /// The nodes used for the job
    #[clap(long, env = "PROOF_NODES")]
    pub nodes: Vec<i32>,

    /// The path to the job input
    #[clap(long, env = "PROOF_INPUT")]
    pub input: PathBuf,

    /// The public inputs for witness extension
    #[clap(long, env = "PROOF_PUBLIC_INPUTS")]
    pub public_inputs: Vec<String>,
}

#[derive(Parser, Debug, Clone)]
struct CoCircomProve {
    /// The API endpoint URL
    #[clap(
        long,
        env = "PROOF_API_URL",
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

    /// The number of inputs for the circuit
    #[clap(long, env = "PROOF_NUM_INPUTS")]
    pub num_inputs: usize,

    /// The output file where the final proof is written to
    #[arg(long, env = "PROOF_OUT", default_value = "proof.json")]
    pub out: PathBuf,

    /// The output JSON file where the public inputs are written to
    #[arg(long, env = "PROOF_OUT_PUBLIC_INPUTS", default_value = "public.json")]
    pub out_public_inputs: PathBuf,
}

#[derive(Parser, Debug, Clone)]
struct CoCircomJobResult {
    /// The API endpoint URL
    #[clap(
        long,
        env = "PROOF_API_URL",
        default_value = "https://proof.taceo.network"
    )]
    pub api_url: String,

    /// The curve
    #[clap(long, env = "PROOF_CURVE")]
    pub curve: Curve,

    /// The job id
    #[clap(long, env = "PROOF_JOB")]
    pub job: Uuid,

    /// The output file where the final proof is written to
    #[arg(long, env = "PROOF_OUT", default_value = "proof")]
    pub out: PathBuf,

    /// The output JSON file where the public inputs are written to
    #[arg(long, env = "PROOF_OUT_PUBLIC_INPUTS", default_value = "public_inputs")]
    pub out_public_inputs: PathBuf,
}

#[derive(Parser, Debug, Clone)]
struct CoNoirFullProve {
    /// The API endpoint URL
    #[clap(
        long,
        env = "PROOF_API_URL",
        default_value = "https://proof.taceo.network"
    )]
    pub api_url: String,

    /// The voucher for a proof job
    #[clap(long, env = "PROOF_VOUCHER")]
    pub voucher: Option<String>,

    /// The job blueprint
    #[clap(long, env = "PROOF_BLUEPRINT")]
    pub blueprint: Uuid,

    /// The path to the job input
    #[clap(long, env = "PROOF_INPUT")]
    pub input: PathBuf,

    /// The path to abi.json of the circuit
    #[clap(long, env = "PROOF_ABI")]
    pub abi: PathBuf,

    /// The public inputs for witness extension
    #[clap(long, env = "PROOF_PUBLIC_INPUTS")]
    pub public_inputs: Vec<u32>,

    /// The output file where the final proof is written to
    #[arg(long, env = "PROOF_OUT", default_value = "proof")]
    pub out: PathBuf,

    /// The output JSON file where the public inputs are written to
    #[arg(long, env = "PROOF_OUT_PUBLIC_INPUTS", default_value = "public_inputs")]
    pub out_public_inputs: PathBuf,
}

#[derive(Parser, Debug, Clone)]
struct CoNoirFullMultipleInputs {
    /// The API endpoint URL
    #[clap(
        long,
        env = "PROOF_API_URL",
        default_value = "https://proof.taceo.network"
    )]
    pub api_url: String,

    /// The voucher for a proof job
    #[clap(long, env = "PROOF_VOUCHER")]
    pub voucher: Option<String>,

    /// The job blueprint
    #[clap(long, env = "PROOF_BLUEPRINT")]
    pub blueprint: Uuid,

    /// The job deadline
    #[clap(long, env = "PROOF_DEADLINE")]
    pub deadline: Option<chrono::DateTime<chrono::Local>>,
}

#[derive(Parser, Debug, Clone)]
struct CoNoirAddInputs {
    /// The API endpoint URL
    #[clap(
        long,
        env = "PROOF_API_URL",
        default_value = "https://proof.taceo.network"
    )]
    pub api_url: String,

    /// The job id
    #[clap(long, env = "PROOF_JOB")]
    pub job: Uuid,

    /// The nodes used for the job
    #[clap(long, env = "PROOF_NODES")]
    pub nodes: Vec<i32>,

    /// The path to the job input
    #[clap(long, env = "PROOF_INPUT")]
    pub input: PathBuf,

    /// The path to abi.json of the circuit
    #[clap(long, env = "PROOF_ABI")]
    pub abi: PathBuf,

    /// The public inputs for witness extension
    #[clap(long, env = "PROOF_PUBLIC_INPUTS")]
    pub public_inputs: Vec<u32>,
}

#[derive(Parser, Debug, Clone)]
struct CoNoirProve {
    /// The API endpoint URL
    #[clap(
        long,
        env = "PROOF_API_URL",
        default_value = "https://proof.taceo.network"
    )]
    pub api_url: String,

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

    /// The public inputs for witness extension
    #[clap(long, env = "PROOF_PUBLIC_INPUTS")]
    pub public_inputs: Vec<u32>,

    /// The output file where the final proof is written to
    #[arg(long, env = "PROOF_OUT", default_value = "proof")]
    pub out: PathBuf,

    /// The output JSON file where the public inputs are written to
    #[arg(long, env = "PROOF_OUT_PUBLIC_INPUTS", default_value = "public_inputs")]
    pub out_public_inputs: PathBuf,
}

#[derive(Parser, Debug, Clone)]
struct CoNoirJobResult {
    /// The API endpoint URL
    #[clap(
        long,
        env = "PROOF_API_URL",
        default_value = "https://proof.taceo.network"
    )]
    pub api_url: String,

    /// The job id
    #[clap(long, env = "PROOF_JOB")]
    pub job: Uuid,

    /// The output file where the final proof is written to
    #[arg(long, env = "PROOF_OUT", default_value = "proof")]
    pub out: PathBuf,

    /// The output JSON file where the public inputs are written to
    #[arg(long, env = "PROOF_OUT_PUBLIC_INPUTS", default_value = "public_inputs")]
    pub out_public_inputs: PathBuf,
}

#[derive(Debug, Clone, Subcommand)]
enum CoCircomCommands {
    /// Schedule a full coCircom job including witness extension
    Full(CoCircomFullProve),
    /// Schedule a full multiple input coCircom job including witness extension
    FullMultipleInputs(CoCircomFullMultipleInputs),
    /// Add inputs to a coCircom job
    AddInputs(CoCircomAddInputs),
    /// Schedule a prove coCircom job
    Prove(CoCircomProve),
    /// Fetch proof result
    GetResult(CoCircomJobResult),
}

#[derive(Debug, Clone, Subcommand)]
enum CoNoirCommands {
    /// Schedule a full coNoir job including witness extension
    Full(CoNoirFullProve),
    /// Schedule a full multiple input coNoir job including witness extension
    FullMultipleInputs(CoNoirFullMultipleInputs),
    /// Add inputs to a coNoir job
    AddInputs(CoNoirAddInputs),
    /// Schedule a prove coNoir job
    Prove(CoNoirProve),
    /// Fetch proof result
    GetResult(CoNoirJobResult),
}

#[derive(Debug, Clone, Subcommand)]
enum Commands {
    #[command(subcommand)]
    CoCircom(CoCircomCommands),
    #[command(subcommand)]
    CoNoir(CoNoirCommands),
}

#[derive(Parser, Debug, Clone)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

async fn run_co_circom_full<P: Pairing + CircomArkworksPairingBridge>(
    args: CoCircomFullProve,
) -> eyre::Result<()>
where
    P::BaseField: CircomArkworksPrimeFieldBridge,
    P::ScalarField: CircomArkworksPrimeFieldBridge,
{
    let config = Configuration {
        base_path: args.api_url,
        ..Default::default()
    };
    let ws_url = ws_url(&config.base_path);
    let nodes = taceo_proof_client::get_random_node_providers(&config).await?;
    let input = serde_json::from_reader(File::open(args.input)?)?;
    let job_id = taceo_proof_client::co_circom::schedule_full_job::<P::ScalarField>(
        &config,
        &nodes,
        args.blueprint,
        args.voucher.as_deref(),
        &args.public_inputs,
        input,
    )
    .await?;
    tracing::info!("scheduled job {job_id}");
    let (proof, public_inputs, _) = taceo_proof_client::co_circom::fetch_job_result::<P>(
        &ws_url,
        job_id,
        StopStrategy::default(),
    )
    .await?;
    write_co_circom_results(&args.out, &args.out_public_inputs, &proof, &public_inputs)?;
    Ok(())
}

async fn run_co_circom_full_multiple_inputs(args: CoCircomFullMultipleInputs) -> eyre::Result<()> {
    let config = Configuration {
        base_path: args.api_url,
        ..Default::default()
    };
    let nodes = taceo_proof_client::get_random_node_providers(&config).await?;
    tracing::info!("nodes for multiple inputs job {nodes:?}");
    let job_id = taceo_proof_client::co_circom::schedule_full_multiple_inputs_job(
        &config,
        &nodes,
        args.blueprint,
        args.voucher.as_deref(),
        args.deadline,
    )
    .await?;
    tracing::info!("scheduled job {job_id}");
    Ok(())
}

async fn run_co_circom_add_inputs<F: PrimeField>(args: CoCircomAddInputs) -> eyre::Result<()> {
    let config = Configuration {
        base_path: args.api_url,
        ..Default::default()
    };
    let job_id = args.job;
    let input = serde_json::from_reader(File::open(args.input)?)?;
    let (node0, node1, node2) = tokio::join!(
        taceo_proof_client::get_node_provider(&config, args.nodes[0]),
        taceo_proof_client::get_node_provider(&config, args.nodes[1]),
        taceo_proof_client::get_node_provider(&config, args.nodes[2])
    );
    let nodes = NodeProviders {
        node0: node0?,
        node1: node1?,
        node2: node2?,
    };
    taceo_proof_client::co_circom::add_job_inputs::<F>(
        &config,
        &nodes,
        job_id,
        &args.public_inputs,
        input,
    )
    .await?;
    tracing::info!("added inputs to job {job_id}");
    Ok(())
}

async fn run_co_circom_prove<P: Pairing + CircomArkworksPairingBridge>(
    args: CoCircomProve,
) -> eyre::Result<()>
where
    P::BaseField: CircomArkworksPrimeFieldBridge,
    P::ScalarField: CircomArkworksPrimeFieldBridge,
{
    let config = Configuration {
        base_path: args.api_url,
        ..Default::default()
    };
    let ws_url = ws_url(&config.base_path);
    let nodes = taceo_proof_client::get_random_node_providers(&config).await?;
    let witness = Witness::from_reader(File::open(args.witness)?)?;
    let job_id = taceo_proof_client::co_circom::schedule_prove_job::<ark_bn254::Fr>(
        &config,
        &nodes,
        args.blueprint,
        args.protocol.into(),
        args.voucher.as_deref(),
        args.num_inputs,
        witness,
    )
    .await?;
    tracing::info!("scheduled job {job_id}");
    let (proof, public_inputs, _) = taceo_proof_client::co_circom::fetch_job_result::<Bn254>(
        &ws_url,
        job_id,
        StopStrategy::default(),
    )
    .await?;
    write_co_circom_results(&args.out, &args.out_public_inputs, &proof, &public_inputs)?;
    Ok(())
}

async fn run_co_circom_fetch_results<P: Pairing + CircomArkworksPairingBridge>(
    args: CoCircomJobResult,
) -> eyre::Result<()>
where
    P::BaseField: CircomArkworksPrimeFieldBridge,
    P::ScalarField: CircomArkworksPrimeFieldBridge,
{
    let ws_url = ws_url(&args.api_url);
    let (proof, public_inputs, _) = taceo_proof_client::co_circom::fetch_job_result::<Bn254>(
        &ws_url,
        args.job,
        StopStrategy::default(),
    )
    .await?;
    write_co_circom_results(&args.out, &args.out_public_inputs, &proof, &public_inputs)?;
    Ok(())
}

fn write_co_circom_results<P: Pairing + CircomArkworksPairingBridge>(
    out: &Path,
    out_public_inputs: &Path,
    proof: &CircomGroth16Proof<P>,
    public_inputs: &[P::ScalarField],
) -> eyre::Result<()>
where
    P::BaseField: CircomArkworksPrimeFieldBridge,
    P::ScalarField: CircomArkworksPrimeFieldBridge,
{
    let proof_bytes = serde_json::to_vec(proof)?;
    let public_inputs_strings = public_inputs
        .iter()
        .map(|f| {
            if f.is_zero() {
                "0".to_string()
            } else {
                f.to_string()
            }
        })
        .collect::<Vec<String>>();
    let public_inputs_bytes = serde_json::to_vec(&public_inputs_strings)?;

    std::fs::write(out, proof_bytes)?;
    tracing::info!("wrote proof to {}", out.display());
    std::fs::write(out_public_inputs, public_inputs_bytes)?;
    tracing::info!("wrote public inputs to {}", out_public_inputs.display());
    Ok(())
}

async fn run_co_noir_full(args: CoNoirFullProve) -> eyre::Result<()> {
    let config = Configuration {
        base_path: args.api_url,
        ..Default::default()
    };
    let ws_url = ws_url(&config.base_path);
    let nodes = taceo_proof_client::get_random_node_providers(&config).await?;
    let abi = serde_json::from_reader(File::open(args.abi)?)?;
    let input = File::open(args.input)?;
    let job_id = taceo_proof_client::co_noir::schedule_full_job(
        &config,
        &nodes,
        args.blueprint,
        args.voucher.as_deref(),
        &abi,
        &args.public_inputs,
        input,
    )
    .await?;
    tracing::info!("scheduled job {job_id}");
    let (proof, public_inputs, _) =
        taceo_proof_client::co_noir::fetch_job_result(&ws_url, job_id, StopStrategy::default())
            .await?;
    write_co_noir_results(&args.out, &args.out_public_inputs, &proof, &public_inputs)?;
    Ok(())
}

async fn run_co_noir_full_multiple_inputs(args: CoNoirFullMultipleInputs) -> eyre::Result<()> {
    let config = Configuration {
        base_path: args.api_url,
        ..Default::default()
    };
    let nodes = taceo_proof_client::get_random_node_providers(&config).await?;
    tracing::info!("nodes for multiple inputs job {nodes:?}");
    let job_id = taceo_proof_client::co_noir::schedule_full_multiple_inputs_job(
        &config,
        &nodes,
        args.blueprint,
        args.voucher.as_deref(),
        args.deadline,
    )
    .await?;
    tracing::info!("scheduled job {job_id}");
    Ok(())
}

async fn run_co_noir_add_inputs(args: CoNoirAddInputs) -> eyre::Result<()> {
    let config = Configuration {
        base_path: args.api_url,
        ..Default::default()
    };
    let job_id = args.job;
    let abi = serde_json::from_reader(File::open(args.abi)?)?;
    let input = File::open(args.input)?;
    let (node0, node1, node2) = tokio::join!(
        taceo_proof_client::get_node_provider(&config, args.nodes[0]),
        taceo_proof_client::get_node_provider(&config, args.nodes[1]),
        taceo_proof_client::get_node_provider(&config, args.nodes[2])
    );
    let nodes = NodeProviders {
        node0: node0?,
        node1: node1?,
        node2: node2?,
    };
    taceo_proof_client::co_noir::add_job_inputs(
        &config,
        &nodes,
        args.job,
        &abi,
        &args.public_inputs,
        input,
    )
    .await?;
    tracing::info!("added inputs to job {job_id}");
    Ok(())
}

async fn run_co_noir_prove(args: CoNoirProve) -> eyre::Result<()> {
    let config = Configuration {
        base_path: args.api_url,
        ..Default::default()
    };
    let ws_url = ws_url(&config.base_path);
    let nodes = taceo_proof_client::get_random_node_providers(&config).await?;
    let witness = noir_types::witness_from_reader(File::open(args.witness)?)?;
    let job_id = taceo_proof_client::co_noir::schedule_prove_job(
        &config,
        &nodes,
        args.blueprint,
        args.protocol.into(),
        args.voucher.as_deref(),
        &args.public_inputs,
        witness,
    )
    .await?;
    tracing::info!("scheduled job {job_id}");
    let (proof, public_inputs, _) =
        taceo_proof_client::co_noir::fetch_job_result(&ws_url, job_id, StopStrategy::default())
            .await?;
    write_co_noir_results(&args.out, &args.out_public_inputs, &proof, &public_inputs)?;
    Ok(())
}

async fn run_co_noir_fetch_results(args: CoNoirJobResult) -> eyre::Result<()> {
    let ws_url = ws_url(&args.api_url);
    let (proof, public_inputs, _) =
        taceo_proof_client::co_noir::fetch_job_result(&ws_url, args.job, StopStrategy::default())
            .await?;
    write_co_noir_results(&args.out, &args.out_public_inputs, &proof, &public_inputs)?;
    Ok(())
}

fn write_co_noir_results(
    out: &Path,
    out_public_inputs: &Path,
    proof: &HonkProof<ark_bn254::Fr>,
    public_inputs: &[ark_bn254::Fr],
) -> eyre::Result<()> {
    let proof_bytes = proof.to_buffer();
    let public_inputs_bytes = SerializeF::to_buffer(public_inputs, false);
    std::fs::write(out, proof_bytes)?;
    tracing::info!("wrote proof to {}", out.display());
    std::fs::write(out_public_inputs, public_inputs_bytes)?;
    tracing::info!("wrote public inputs to {}", out_public_inputs.display());
    Ok(())
}

fn ws_url(api_url: &str) -> String {
    api_url.replace("http", "ws").replace("https", "wss") + "/api/v1/reports/subs"
}

async fn run(command: Commands) -> eyre::Result<()> {
    match command {
        Commands::CoCircom(command) => match command {
            CoCircomCommands::Full(args) => {
                match args.curve {
                    Curve::Bn254 => run_co_circom_full::<Bn254>(args).await?,
                    Curve::Bls381 => run_co_circom_full::<Bls12_381>(args).await?,
                    Curve::Bls377 => eyre::bail!("Bls377 is not supported for co-circom full"),
                };
            }
            CoCircomCommands::FullMultipleInputs(args) => {
                run_co_circom_full_multiple_inputs(args).await?
            }
            CoCircomCommands::AddInputs(args) => {
                match args.curve {
                    Curve::Bn254 => run_co_circom_add_inputs::<ark_bn254::Fr>(args).await?,
                    Curve::Bls381 => run_co_circom_add_inputs::<ark_bls12_381::Fr>(args).await?,
                    Curve::Bls377 => {
                        eyre::bail!("Bls377 is not supported for co-circom add-inputs")
                    }
                };
            }
            CoCircomCommands::Prove(args) => {
                match args.curve {
                    Curve::Bn254 => run_co_circom_prove::<Bn254>(args).await?,
                    Curve::Bls381 => run_co_circom_prove::<Bls12_381>(args).await?,
                    Curve::Bls377 => run_co_circom_prove::<Bls12_377>(args).await?,
                };
            }
            CoCircomCommands::GetResult(args) => {
                match args.curve {
                    Curve::Bn254 => run_co_circom_fetch_results::<Bn254>(args).await?,
                    Curve::Bls381 => run_co_circom_fetch_results::<Bls12_381>(args).await?,
                    Curve::Bls377 => run_co_circom_fetch_results::<Bls12_377>(args).await?,
                };
            }
        },
        Commands::CoNoir(command) => match command {
            CoNoirCommands::Full(args) => run_co_noir_full(args).await?,
            CoNoirCommands::FullMultipleInputs(args) => {
                run_co_noir_full_multiple_inputs(args).await?
            }
            CoNoirCommands::AddInputs(args) => run_co_noir_add_inputs(args).await?,
            CoNoirCommands::Prove(args) => run_co_noir_prove(args).await?,
            CoNoirCommands::GetResult(args) => run_co_noir_fetch_results(args).await?,
        },
    };
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
    run(args.command).await?;
    Ok(())
}
