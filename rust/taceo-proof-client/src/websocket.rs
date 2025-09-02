use std::fmt;

use eyre::Context as _;
use futures::{SinkExt as _, StreamExt as _};
use intmap::IntMap;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// The status of a job.
pub enum JobStatus {
    Pending,
    Running,
    Finished,
    Cancelled,
}

#[derive(Serialize, Deserialize)]
/// The result of a job, including signatures of the nodes that handled the job.
pub(crate) struct SignedResults {
    /// The signatures of each node used to compute the job (identified by their id).
    pub signatures: IntMap<i32, String>,
    /// The proof as JSON (for CircomGroth16 proofs) or base64 encoded ark_serialized `ark_groth16::Proof` (for LibsnarkGroth16 proofs).
    pub proof: String,
    /// The array of public inputs as JSON (for CircomGroth16 proofs) pr base64 encoded ark_serialized `Vec<P::ScalarField>` (for LibsnarkGroth16 proofs).
    pub public_inputs: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// A error result of a job.
struct FailedReason {
    /// The node provider that encountered the error.
    pub node_provider: i32,
    /// The error string.
    pub error: String,
    /// The signature of the error and the node provider.
    pub signature: String,
}

impl fmt::Debug for SignedResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node_providers = self
            .signatures
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<_>>();
        f.write_fmt(format_args!("Result from: {node_providers:?}"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// The different messages we receive over the ws connection.
enum WebSocketMessage {
    Success(SignedResults),
    Update(JobStatus),
    Failed(FailedReason),
    Cancelled,
    Err(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
/// The stop strategy when waiting for the job result.
pub enum StopStrategy {
    #[default]
    /// Stop if one node responded with a result.
    First,
    /// Stop if the majority of nodes responded with a result.
    Majority,
    /// Stop if the all nodes responded with a result.
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The ws subscribe request to wait for a job result.
struct SubscribeExecutionRequest {
    pub execution_id: Uuid,
    pub stop_on_finished_reports: StopStrategy,
    pub with_status_updates: Option<bool>,
}

pub(crate) async fn fetch_job_result(
    url: &str,
    job_id: Uuid,
    stop_strategy: StopStrategy,
) -> eyre::Result<SignedResults> {
    // subscribe with ws to get job updates
    let (mut ws_stream, _) = tokio_tungstenite::connect_async(url)
        .await
        .context("while connecting to endpoint")?;
    ws_stream
        .send(Message::text(
            serde_json::to_string(&SubscribeExecutionRequest {
                execution_id: job_id,
                stop_on_finished_reports: stop_strategy,
                with_status_updates: Some(false),
            })
            .expect("can serialize"),
        ))
        .await
        .context("while sending subscribe request")?;
    if let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let msg: WebSocketMessage =
                    serde_json::from_str(&text).context("invalid data from server")?;
                match msg {
                    WebSocketMessage::Success(signed_results) => Ok(signed_results),
                    WebSocketMessage::Failed(FailedReason {
                        node_provider,
                        error,
                        signature: _,
                    }) => eyre::bail!("node {node_provider} error: {error:?}"),
                    WebSocketMessage::Cancelled => eyre::bail!("job was cancelled"),
                    WebSocketMessage::Err(err) => eyre::bail!(err),
                    WebSocketMessage::Update(_) => eyre::bail!("unexpected update"),
                }
            }
            Ok(Message::Close(_)) => {
                eyre::bail!("server closed stream");
            }
            Ok(_) => {
                eyre::bail!("server sent invalid data");
            }
            Err(err) => Err(err.into()),
        }
    } else {
        eyre::bail!("server closed stream");
    }
}
