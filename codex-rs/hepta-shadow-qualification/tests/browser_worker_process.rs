use std::path::PathBuf;

use codex_hepta_shadow_qualification::BrowserActorKind;
use codex_hepta_shadow_qualification::BrowserCommand;
use codex_hepta_shadow_qualification::BrowserOutcome;
use codex_hepta_shadow_qualification::BrowserRequest;
use codex_hepta_shadow_qualification::BrowserSessionId;
use codex_hepta_shadow_qualification::BrowserWorkerLaunchSpec;
use codex_hepta_shadow_qualification::BrowserWorkerShutdownReason;
use codex_hepta_shadow_qualification::QualificationBrowserWorker;

#[tokio::test(flavor = "current_thread")]
async fn qualification_worker_uses_only_private_child_pipes_and_preserves_browser_fences(
) -> Result<(), Box<dyn std::error::Error>> {
    let program = PathBuf::from(env!("CARGO_BIN_EXE_hepta-browser-worker-qualification"));
    let session_id = BrowserSessionId::from_seed("browser-worker-process-test")?;
    let spec = BrowserWorkerLaunchSpec::new(program, session_id.clone(), 13)?;
    let mut worker = QualificationBrowserWorker::spawn(spec).await?;
    assert!(worker.is_ready());
    assert_ne!(worker.process_id(), 0);

    let navigate = BrowserRequest::new(
        1,
        session_id.clone(),
        BrowserActorKind::Agent,
        13,
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
        session_id.clone(),
        BrowserActorKind::Agent,
        13,
        1,
        1,
        BrowserCommand::Observe { max_nodes: 16 },
    );
    let observe = worker.request(observe).await?;
    let snapshot = match observe.outcome {
        BrowserOutcome::Observed { snapshot } => snapshot,
        other => panic!("expected semantic snapshot, got {other:?}"),
    };
    assert_eq!(snapshot.page_revision, 1);
    assert_eq!(snapshot.nodes.len(), 3);
    assert!(observe.authority.is_closed());

    worker
        .shutdown(BrowserWorkerShutdownReason::QualificationComplete)
        .await?;
    Ok(())
}
