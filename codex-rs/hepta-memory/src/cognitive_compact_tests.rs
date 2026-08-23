use codex_hepta_contracts::Sha256Digest;
use pretty_assertions::assert_eq;
use sqlx::Executor;
use sqlx::Row;
use tempfile::TempDir;

use crate::COGNITIVE_COMPACT_HOOK_NAMESPACE;
use crate::CognitiveCompactError;
use crate::CognitiveRuntime;
use crate::CognitiveStore;
use crate::CognitiveUnavailableReason;
use crate::CompactCheckpoint;
use crate::CompactCommitDecision;
use crate::CompactConflictReason;
use crate::CompactFence;
use crate::CompactLease;
use crate::CompactLossReport;
use crate::CompactParentSnapshot;
use crate::CompactProtectedRef;
use crate::CompactSummaryReceipt;
use crate::RehydrationStatus;
use crate::cognitive_test_support::agent_id;
use crate::cognitive_test_support::layout;

fn fence() -> CompactFence {
    CompactFence::new(3, 8, 19, "fence:19").expect("valid fence")
}

fn snapshot() -> CompactParentSnapshot {
    CompactParentSnapshot::new(
        "ctx:local-development",
        20,
        30,
        7,
        Sha256Digest::for_bytes(b"parent-state"),
        fence(),
    )
    .expect("valid parent snapshot")
}

fn summary() -> CompactSummaryReceipt {
    CompactSummaryReceipt::new(
        Sha256Digest::for_bytes(b"summary"),
        Sha256Digest::for_bytes(b"model-receipt"),
        Sha256Digest::for_bytes(b"compact-policy:v1"),
    )
}

fn loss_report() -> CompactLossReport {
    CompactLossReport::new(vec!["event:29".to_string()], 1, Vec::new(), 0)
        .expect("valid loss report")
}

fn checkpoint() -> CompactCheckpoint {
    let protected = CompactProtectedRef::new("approval:1", "approval", true)
        .expect("valid protected reference");
    CompactCheckpoint::new(
        "ctxcp:local-development",
        CompactLease::from_snapshot(snapshot()),
        vec![protected],
        summary(),
        loss_report(),
        0,
    )
    .expect("valid checkpoint")
}

#[test]
fn pre_hook_is_deterministic_and_explicitly_local_only() {
    let first = CompactLease::from_snapshot(snapshot());
    let second = CompactLease::from_snapshot(snapshot());
    assert_eq!(first, second);
    assert_eq!(first.namespace, COGNITIVE_COMPACT_HOOK_NAMESPACE);
    assert_eq!(
        first.lease_id,
        format!("ctxlease:v1:{}", first.lease_sha256.as_str())
    );
    assert_eq!(
        CognitiveRuntime::Absent.pre_compact(snapshot()),
        Err(CognitiveCompactError::RuntimeAbsent)
    );
}

#[test]
fn deserialized_lease_identity_is_recomputed_before_compact_use() {
    let mut digest_tampered = checkpoint();
    digest_tampered.lease.lease_sha256 = Sha256Digest::for_bytes(b"tampered");
    assert!(matches!(
        digest_tampered.rehydration_plan(0),
        Err(CognitiveCompactError::Invalid { message })
            if message.contains("lease digest")
    ));

    let mut id_tampered = checkpoint();
    id_tampered.lease.lease_id.push_str(":tampered");
    assert!(matches!(
        id_tampered.rehydration_plan(0),
        Err(CognitiveCompactError::Invalid { message })
            if message.contains("lease id")
    ));
}

#[test]
fn compact_commit_validation_fences_parent_cas_and_generation() {
    let checkpoint = checkpoint();
    assert_eq!(
        checkpoint.validate_against(&snapshot()),
        CompactCommitDecision::Accepted {
            checkpoint_id: "ctxcp:local-development".to_string(),
            checkpoint_revision: 0,
        }
    );

    let mut revision_changed = snapshot();
    revision_changed.expected_parent_revision = 8;
    assert_eq!(
        checkpoint.validate_against(&revision_changed),
        CompactCommitDecision::Conflict {
            reason: CompactConflictReason::ParentRevisionChanged,
        }
    );

    let mut state_changed = snapshot();
    state_changed.expected_state_sha256 = Sha256Digest::for_bytes(b"new-state");
    assert_eq!(
        checkpoint.validate_against(&state_changed),
        CompactCommitDecision::Conflict {
            reason: CompactConflictReason::ParentStateChanged,
        }
    );

    let mut generation_changed = snapshot();
    generation_changed.fence.generation = 20;
    assert_eq!(
        checkpoint.validate_against(&generation_changed),
        CompactCommitDecision::StaleGeneration
    );
}

#[test]
fn post_hook_returns_visible_rehydration_plan_and_checks_revision() {
    let checkpoint = checkpoint();
    let plan = checkpoint
        .rehydration_plan(0)
        .expect("matching checkpoint revision");
    assert_eq!(plan.checkpoint_id, "ctxcp:local-development");
    assert_eq!(plan.protected_refs.len(), 1);
    assert_eq!(plan.status, RehydrationStatus::NotStarted);
    assert_eq!(
        checkpoint.rehydration_plan(1),
        Err(CognitiveCompactError::RevisionMismatch {
            expected: 1,
            actual: 0,
        })
    );
}

#[test]
fn summary_fact_admission_and_protected_loss_are_rejected() {
    let mut encoded = serde_json::to_value(summary()).expect("serialize summary");
    encoded["fact_admission"] = serde_json::Value::Bool(true);
    let fact_admitting_summary: CompactSummaryReceipt =
        serde_json::from_value(encoded).expect("decode summary");
    assert_eq!(
        CompactCheckpoint::new(
            "ctxcp:fact-admission",
            CompactLease::from_snapshot(snapshot()),
            Vec::new(),
            fact_admitting_summary,
            loss_report(),
            0,
        ),
        Err(CognitiveCompactError::SummaryFactAdmission)
    );

    let lost_refs = CompactLossReport::new(Vec::new(), 0, vec!["approval:1".to_string()], 0)
        .expect("loss report is structurally valid");
    assert_eq!(
        CompactCheckpoint::new(
            "ctxcp:lost-ref",
            CompactLease::from_snapshot(snapshot()),
            Vec::new(),
            summary(),
            lost_refs,
            0,
        ),
        Err(CognitiveCompactError::ProtectedReferenceLoss)
    );
}

#[tokio::test]
async fn available_runtime_hooks_are_read_only_for_the_cognitive_store() {
    let temp = TempDir::new().expect("temp dir");
    let owner = agent_id(91);
    let store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("open cognitive store");
    let before: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM memory_revisions")
        .fetch_one(&store.pool)
        .await
        .expect("memory count before hooks");
    let runtime = CognitiveRuntime::from_open_result(Ok(store));
    let lease = runtime.pre_compact(snapshot()).expect("pre hook");
    let mut checkpoint = checkpoint();
    checkpoint.lease = lease;
    let plan = runtime.post_compact(&checkpoint, 0).expect("post hook");
    assert_eq!(plan.status, RehydrationStatus::NotStarted);
    assert_eq!(
        runtime
            .validate_compact_commit(&checkpoint, &snapshot())
            .expect("CAS validation"),
        CompactCommitDecision::Accepted {
            checkpoint_id: checkpoint.checkpoint_id.clone(),
            checkpoint_revision: 0,
        }
    );
    let after: i64 = runtime
        .available_store()
        .expect("available store")
        .pool
        .clone()
        .fetch_one(sqlx::query("SELECT COUNT(*) FROM memory_revisions"))
        .await
        .expect("memory count after hooks")
        .try_get(0)
        .expect("memory count column");
    assert_eq!(before, after);
}

#[test]
fn unavailable_runtime_keeps_hook_error_sanitized() {
    let runtime = CognitiveRuntime::Unavailable(CognitiveUnavailableReason::StorageUnavailable);
    assert_eq!(
        runtime.post_compact(&checkpoint(), 0),
        Err(CognitiveCompactError::RuntimeUnavailable {
            reason: CognitiveUnavailableReason::StorageUnavailable,
        })
    );
}
