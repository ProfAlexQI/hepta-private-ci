#![cfg(unix)]

use std::path::PathBuf;

use codex_hepta_shadow_qualification::BrowserActorKind;
use codex_hepta_shadow_qualification::BrowserCommand;
use codex_hepta_shadow_qualification::BrowserOutcome;
use codex_hepta_shadow_qualification::BrowserRequest;
use codex_hepta_shadow_qualification::BrowserSessionId;
use codex_hepta_shadow_qualification::BrowserWorkerLaunchSpec;
use codex_hepta_shadow_qualification::BrowserWorkerShutdownReason;
use codex_hepta_shadow_qualification::UnixQualificationBrowserWorker;

#[tokio::test(flavor = "current_thread")]
async fn inherited_unix_socketpair_has_no_listener_and_preserves_worker_identity(
) -> Result<(), Box<dyn std::error::Error>> {
    let program = PathBuf::from(env!("CARGO_BIN_EXE_hepta-browser-worker-qualification"));
    let session_id = BrowserSessionId::from_seed("browser-worker-unix-socketpair-test")?;
    let spec = BrowserWorkerLaunchSpec::new(program, session_id.clone(), 17)?;
    let mut worker = UnixQualificationBrowserWorker::spawn(spec).await?;
    assert!(worker.is_ready());
    assert_ne!(worker.process_id(), 0);

    let navigate = BrowserRequest::new(
        1,
        session_id.clone(),
        BrowserActorKind::Agent,
        17,
        1,
        0,
        BrowserCommand::Navigate {
            url: "fixture://shared-form".to_string(),
        },
    );
    let navigate = worker.request(navigate).await?;
    assert!(matches!(
        navigate.outcome,
        BrowserOutcome::Applied { .. }
    ));
    assert_eq!(navigate.page_revision, 1);
    assert!(navigate.authority.is_closed());

    let observe = BrowserRequest::new(
        2,
        session_id,
        BrowserActorKind::Agent,
        17,
        1,
        1,
        BrowserCommand::Observe { max_nodes: 16 },
    );
    let observe = worker.request(observe).await?;
    assert!(matches!(
        observe.outcome,
        BrowserOutcome::Observed { .. }
    ));
    assert!(observe.authority.is_closed());

    worker
        .shutdown(BrowserWorkerShutdownReason::QualificationComplete)
        .await?;
    Ok(())
}
