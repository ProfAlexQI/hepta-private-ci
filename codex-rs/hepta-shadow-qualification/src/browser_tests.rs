use pretty_assertions::assert_eq;

use crate::QualificationError;
use crate::browser_contracts::BrowserAction;
use crate::browser_contracts::BrowserActorKind;
use crate::browser_contracts::BrowserCommand;
use crate::browser_contracts::BrowserCommandKind;
use crate::browser_contracts::BrowserControlMode;
use crate::browser_contracts::BrowserDenialCode;
use crate::browser_contracts::BrowserOutcome;
use crate::browser_contracts::BrowserRequest;
use crate::browser_contracts::BrowserSessionId;
use crate::browser_contracts::SemanticNode;
use crate::browser_contracts::SemanticRef;
use crate::browser_contracts::SemanticSnapshot;
use crate::browser_runtime::BrowserActor;
use crate::browser_runtime::FixtureBrowserEngine;

fn actor() -> Result<BrowserActor<FixtureBrowserEngine>, QualificationError> {
    BrowserActor::new(
        BrowserSessionId::from_seed("browser-c0-c3-qualification")?,
        7,
        FixtureBrowserEngine::default(),
    )
}

fn request(
    actor: &BrowserActor<FixtureBrowserEngine>,
    request_id: u64,
    actor_kind: BrowserActorKind,
    command: BrowserCommand,
) -> BrowserRequest {
    let status = actor.status();
    BrowserRequest::new(
        request_id,
        status.session_id,
        actor_kind,
        status.generation,
        status.owner_epoch,
        status.page_revision,
        command,
    )
}

fn snapshot(
    response: &crate::browser_contracts::BrowserResponse,
) -> Result<&SemanticSnapshot, QualificationError> {
    match &response.outcome {
        BrowserOutcome::Observed { snapshot } => Ok(snapshot),
        outcome => Err(QualificationError::State(format!(
            "expected observed browser outcome, found {outcome:?}"
        ))),
    }
}

fn node<'a>(
    snapshot: &'a SemanticSnapshot,
    role: &str,
) -> Result<&'a SemanticNode, QualificationError> {
    snapshot
        .nodes
        .iter()
        .find(|node| node.role == role)
        .ok_or_else(|| QualificationError::State(format!("missing browser fixture role {role}")))
}

#[test]
fn browser_shared_page_human_agent_round_trip() -> Result<(), QualificationError> {
    let mut actor = actor()?;

    let navigate = request(
        &actor,
        1,
        BrowserActorKind::Agent,
        BrowserCommand::Navigate {
            url: "fixture://shared-form".to_string(),
        },
    );
    let navigate_response = actor.handle(navigate, 100)?;
    assert!(matches!(
        navigate_response.outcome,
        BrowserOutcome::Applied {
            command: BrowserCommandKind::Navigate
        }
    ));
    assert_eq!(navigate_response.page_revision, 1);

    let observe = request(
        &actor,
        2,
        BrowserActorKind::Agent,
        BrowserCommand::Observe { max_nodes: 32 },
    );
    let observed_response = actor.handle(observe, 110)?;
    let first_snapshot = snapshot(&observed_response)?;
    let input_ref = node(first_snapshot, "textbox")?.semantic_ref.clone();

    let take_control = request(
        &actor,
        3,
        BrowserActorKind::Human,
        BrowserCommand::HumanTakeControl { lease_ms: 1_000 },
    );
    let take_response = actor.handle(take_control, 200)?;
    assert!(matches!(
        take_response.outcome,
        BrowserOutcome::ControlTransferred {
            mode: BrowserControlMode::HumanTurn
        }
    ));
    assert_eq!(take_response.owner_epoch, 2);
    assert_eq!(take_response.page_revision, 1);

    let type_name = request(
        &actor,
        4,
        BrowserActorKind::Human,
        BrowserCommand::HumanInput {
            target: input_ref,
            action: BrowserAction::TypeText {
                text: "Ada".to_string(),
            },
        },
    );
    let type_response = actor.handle(type_name, 300)?;
    assert!(matches!(
        type_response.outcome,
        BrowserOutcome::Applied {
            command: BrowserCommandKind::HumanInput
        }
    ));
    assert_eq!(type_response.page_revision, 2);

    let observe_during_human_turn = request(
        &actor,
        5,
        BrowserActorKind::Agent,
        BrowserCommand::Observe { max_nodes: 32 },
    );
    let shared_read_response = actor.handle(observe_during_human_turn, 310)?;
    let shared_snapshot = snapshot(&shared_read_response)?;
    assert_eq!(node(shared_snapshot, "textbox")?.value, "Ada");
    let button_ref = node(shared_snapshot, "button")?.semantic_ref.clone();

    let click = request(
        &actor,
        6,
        BrowserActorKind::Human,
        BrowserCommand::HumanInput {
            target: button_ref,
            action: BrowserAction::Click,
        },
    );
    let click_response = actor.handle(click, 400)?;
    assert_eq!(click_response.page_revision, 3);

    let release = request(
        &actor,
        7,
        BrowserActorKind::Human,
        BrowserCommand::HumanReleaseControl,
    );
    let release_response = actor.handle(release, 500)?;
    assert!(matches!(
        release_response.outcome,
        BrowserOutcome::ControlTransferred {
            mode: BrowserControlMode::AgentTurn
        }
    ));
    assert_eq!(release_response.owner_epoch, 3);
    assert_eq!(release_response.page_revision, 4);

    let final_observe = request(
        &actor,
        8,
        BrowserActorKind::Agent,
        BrowserCommand::Observe { max_nodes: 32 },
    );
    let final_response = actor.handle(final_observe, 510)?;
    let final_snapshot = snapshot(&final_response)?;
    assert_eq!(node(final_snapshot, "textbox")?.value, "Ada");
    assert_eq!(node(final_snapshot, "button")?.value, "1");
    assert_eq!(node(final_snapshot, "status")?.value, "submitted:1");

    let extract = request(
        &actor,
        9,
        BrowserActorKind::Agent,
        BrowserCommand::Extract {
            query: "storage.cookie_revision".to_string(),
            max_bytes: 128,
        },
    );
    let extract_response = actor.handle(extract, 520)?;
    match &extract_response.outcome {
        BrowserOutcome::Extracted {
            value, truncated, ..
        } => {
            assert_eq!(value, "1");
            assert!(!truncated);
        }
        outcome => {
            return Err(QualificationError::State(format!(
                "expected extracted browser outcome, found {outcome:?}"
            )));
        }
    }
    assert!(extract_response.authority.is_closed());
    assert!(extract_response.activity_receipt.authority.is_closed());
    let evidence = extract_response.evidence_receipt.ok_or_else(|| {
        QualificationError::State("missing browser evidence receipt".to_string())
    })?;
    assert!(!evidence.raw_secret_bytes_present);
    assert!(!evidence.cross_tenant_data_present);
    assert!(!evidence.external_effect);
    assert!(evidence.qualification_only);
    Ok(())
}

#[test]
fn browser_stale_semantic_ref_is_rejected_after_mutation() -> Result<(), QualificationError> {
    let mut actor = actor()?;
    let navigate = request(
        &actor,
        1,
        BrowserActorKind::Agent,
        BrowserCommand::Navigate {
            url: "fixture://shared-form".to_string(),
        },
    );
    actor.handle(navigate, 10)?;
    let observe = request(
        &actor,
        2,
        BrowserActorKind::Agent,
        BrowserCommand::Observe { max_nodes: 32 },
    );
    let observed = actor.handle(observe, 20)?;
    let input_ref = node(snapshot(&observed)?, "textbox")?.semantic_ref.clone();

    let first_action = request(
        &actor,
        3,
        BrowserActorKind::Agent,
        BrowserCommand::Act {
            target: input_ref.clone(),
            action: BrowserAction::TypeText {
                text: "first".to_string(),
            },
        },
    );
    let applied = actor.handle(first_action, 30)?;
    assert_eq!(applied.page_revision, 2);

    let stale_action = request(
        &actor,
        4,
        BrowserActorKind::Agent,
        BrowserCommand::Act {
            target: input_ref,
            action: BrowserAction::Clear,
        },
    );
    let denied = actor.handle(stale_action, 40)?;
    assert!(matches!(
        denied.outcome,
        BrowserOutcome::Denied {
            code: BrowserDenialCode::StaleSemanticRef
        }
    ));
    assert_eq!(denied.page_revision, 2);
    assert!(denied.evidence_receipt.is_none());
    assert!(denied.activity_receipt.authority.is_closed());
    Ok(())
}

#[test]
fn browser_request_replay_is_stable_and_conflict_fails_closed() -> Result<(), QualificationError> {
    let mut actor = actor()?;
    let navigate = request(
        &actor,
        1,
        BrowserActorKind::Agent,
        BrowserCommand::Navigate {
            url: "fixture://shared-form".to_string(),
        },
    );
    let original = actor.handle(navigate.clone(), 10)?;
    let replay = actor.handle(navigate, 20)?;
    assert_eq!(replay, original);

    let status = actor.status();
    let conflict = BrowserRequest::new(
        1,
        status.session_id,
        BrowserActorKind::Agent,
        status.generation,
        status.owner_epoch,
        status.page_revision,
        BrowserCommand::Observe { max_nodes: 8 },
    );
    let denied = actor.handle(conflict, 30)?;
    assert!(matches!(
        denied.outcome,
        BrowserOutcome::Denied {
            code: BrowserDenialCode::RequestIdConflict
        }
    ));
    assert!(denied.evidence_receipt.is_none());
    assert_eq!(denied.activity_receipt.session_id, actor.status().session_id);
    assert_eq!(denied.activity_receipt.generation, actor.status().generation);
    Ok(())
}

#[test]
fn browser_human_lease_expiry_advances_epoch_and_revision() -> Result<(), QualificationError> {
    let mut actor = actor()?;
    let navigate = request(
        &actor,
        1,
        BrowserActorKind::Agent,
        BrowserCommand::Navigate {
            url: "fixture://shared-form".to_string(),
        },
    );
    actor.handle(navigate, 10)?;
    let take = request(
        &actor,
        2,
        BrowserActorKind::Human,
        BrowserCommand::HumanTakeControl { lease_ms: 100 },
    );
    let taken = actor.handle(take, 20)?;
    assert_eq!(taken.owner_epoch, 2);
    assert_eq!(taken.page_revision, 1);

    let stale_read = request(
        &actor,
        3,
        BrowserActorKind::Agent,
        BrowserCommand::Observe { max_nodes: 8 },
    );
    let denied = actor.handle(stale_read, 120)?;
    assert!(matches!(
        denied.outcome,
        BrowserOutcome::Denied {
            code: BrowserDenialCode::StaleOwnerEpoch
        }
    ));
    let status = actor.status();
    assert_eq!(status.mode, BrowserControlMode::AgentTurn);
    assert_eq!(status.owner_epoch, 3);
    assert_eq!(status.page_revision, 2);
    assert_eq!(status.human_lease_expires_at_ms, None);

    let fresh_read = request(
        &actor,
        4,
        BrowserActorKind::Agent,
        BrowserCommand::Observe { max_nodes: 8 },
    );
    let observed = actor.handle(fresh_read, 121)?;
    assert!(matches!(observed.outcome, BrowserOutcome::Observed { .. }));
    Ok(())
}

#[test]
fn browser_external_navigation_is_denied_without_state_evidence() -> Result<(), QualificationError> {
    let mut actor = actor()?;
    let external = request(
        &actor,
        1,
        BrowserActorKind::Agent,
        BrowserCommand::Navigate {
            url: "https://example.com/".to_string(),
        },
    );
    let denied = actor.handle(external, 10)?;
    assert!(matches!(
        denied.outcome,
        BrowserOutcome::Denied {
            code: BrowserDenialCode::ExternalNavigationDisabled
        }
    ));
    assert_eq!(denied.page_revision, 0);
    assert!(denied.evidence_receipt.is_none());
    assert!(denied.authority.is_closed());
    Ok(())
}

#[test]
fn browser_wrong_session_receipt_binds_actual_actor_identity() -> Result<(), QualificationError> {
    let mut actor = actor()?;
    let actor_status = actor.status();
    let request = BrowserRequest::new(
        1,
        BrowserSessionId::from_seed("another-browser-session")?,
        BrowserActorKind::Agent,
        actor_status.generation,
        actor_status.owner_epoch,
        actor_status.page_revision,
        BrowserCommand::Observe { max_nodes: 8 },
    );
    let denied = actor.handle(request, 10)?;
    assert!(matches!(
        denied.outcome,
        BrowserOutcome::Denied {
            code: BrowserDenialCode::WrongSession
        }
    ));
    assert_eq!(denied.session_id, actor_status.session_id);
    assert_eq!(denied.activity_receipt.session_id, actor_status.session_id);
    assert_eq!(denied.activity_receipt.generation, actor_status.generation);
    assert!(denied.evidence_receipt.is_none());
    Ok(())
}

#[test]
fn browser_sensitive_extract_is_denied() -> Result<(), QualificationError> {
    let mut actor = actor()?;
    let navigate = request(
        &actor,
        1,
        BrowserActorKind::Agent,
        BrowserCommand::Navigate {
            url: "fixture://shared-form".to_string(),
        },
    );
    actor.handle(navigate, 10)?;
    let extract = request(
        &actor,
        2,
        BrowserActorKind::Agent,
        BrowserCommand::Extract {
            query: "document.cookie".to_string(),
            max_bytes: 128,
        },
    );
    let denied = actor.handle(extract, 20)?;
    assert!(matches!(
        denied.outcome,
        BrowserOutcome::Denied {
            code: BrowserDenialCode::SensitiveDataDenied
        }
    ));
    assert!(denied.evidence_receipt.is_none());
    assert!(denied.activity_receipt.authority.is_closed());
    Ok(())
}

#[test]
fn browser_wire_identifiers_require_canonical_shape() {
    let invalid_session = serde_json::from_str::<BrowserSessionId>("\"not-a-session\"");
    assert!(invalid_session.is_err());

    let leading_zero_revision = format!("browser-ref:v1:01:{}", "0".repeat(64));
    let invalid_ref =
        serde_json::from_value::<SemanticRef>(serde_json::json!(leading_zero_revision));
    assert!(invalid_ref.is_err());
}
