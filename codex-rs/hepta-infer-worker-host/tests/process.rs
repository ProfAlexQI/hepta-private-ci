use std::path::Path;
use std::time::Duration;

use codex_hepta_infer_core::Digest;
use codex_hepta_infer_core::RequestId;
use codex_hepta_infer_worker_host::MAX_PRIVATE_WORKER_FRAME_BYTES;
use codex_hepta_infer_worker_host::WorkerCancelOutcome;
use codex_hepta_infer_worker_host::WorkerHostError;
use codex_hepta_infer_worker_host::WorkerHostProcess;

const WORKER: &str = env!("CARGO_BIN_EXE_hepta-infer-worker-host");

fn must<T, E: std::fmt::Display>(result: std::result::Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => panic!("unexpected error: {error}"),
    }
}

fn digest(fill: char) -> Digest {
    must(Digest::parse(&format!(
        "sha256:{}",
        fill.to_string().repeat(64)
    )))
}

async fn spawn(mode: &str, timeout: Duration) -> WorkerHostProcess {
    must(
        WorkerHostProcess::spawn_fixture(
            Path::new(WORKER),
            mode,
            7,
            digest('a'),
            MAX_PRIVATE_WORKER_FRAME_BYTES,
            timeout,
        )
        .await,
    )
}

#[tokio::test]
async fn isolated_fixture_process_emits_fenced_token_and_non_real_receipt() {
    let mut worker = spawn("success", Duration::from_secs(2)).await;
    assert!(worker.child_id().is_some());
    let receipt = must(
        worker
            .submit_fixture(
                must(RequestId::parse("request-worker-success")),
                2,
                3,
                digest('b'),
                digest('c'),
                8,
            )
            .await,
    );
    assert_eq!(receipt.backend_generation, 7);
    assert_eq!(receipt.output_tokens, 1);
    assert!(receipt.fixture_only);
    assert!(!receipt.real_native_model_executed);
    assert!(!receipt.remote_fallback_attempted);
    must(worker.shutdown().await);
}

#[tokio::test]
async fn stale_generation_and_oom_frames_fail_closed() {
    let mut stale = spawn("stale", Duration::from_secs(2)).await;
    assert_eq!(
        stale
            .submit_fixture(
                must(RequestId::parse("request-worker-stale")),
                2,
                3,
                digest('b'),
                digest('c'),
                8,
            )
            .await,
        Err(WorkerHostError::ProtocolFence)
    );

    let mut oom = spawn("oom", Duration::from_secs(2)).await;
    assert_eq!(
        oom.submit_fixture(
            must(RequestId::parse("request-worker-oom")),
            2,
            3,
            digest('b'),
            digest('c'),
            8,
        )
        .await,
        Err(WorkerHostError::WorkerFailure(
            "INF_WORKER_OOM".to_owned()
        ))
    );
}

#[tokio::test]
async fn cancel_ack_timeout_kill_and_crash_are_distinct() {
    let request_id = must(RequestId::parse("request-worker-cancel"));

    let mut acknowledged = spawn("cancel-ack", Duration::from_secs(2)).await;
    assert_eq!(
        must(acknowledged.cancel(request_id.clone(), 2, 4).await),
        WorkerCancelOutcome::Acknowledged
    );
    must(acknowledged.shutdown().await);

    let mut hanging = spawn("hang", Duration::from_millis(50)).await;
    assert_eq!(
        must(hanging.cancel(request_id.clone(), 2, 4).await),
        WorkerCancelOutcome::ForcedKill
    );

    let mut crashed = spawn("crash", Duration::from_secs(2)).await;
    assert_eq!(
        must(crashed.cancel(request_id, 2, 4).await),
        WorkerCancelOutcome::WorkerExited
    );
}
