use codex_hepta_authbus_qualification::AdmissionDisposition;
use codex_hepta_authbus_qualification::DispatchObservation;
use codex_hepta_authbus_qualification::DurableOperationIntent;
use codex_hepta_authbus_qualification::LookupOutcome;
use codex_hepta_authbus_qualification::OperationState;
use codex_hepta_authbus_qualification::QualificationAdmission;
use codex_hepta_authbus_qualification::QualificationError;
use codex_hepta_authbus_qualification::QualificationFailpoint;
use codex_hepta_authbus_qualification::QualificationFence;
use codex_hepta_authbus_qualification::QualificationOperationKind;
use codex_hepta_authbus_qualification::QualificationPermit;
use codex_hepta_authbus_qualification::QualificationQuota;
use codex_hepta_authbus_qualification::QualificationStore;
use codex_hepta_authbus_qualification::QuotaReservationState;
use codex_hepta_authbus_qualification::RecoveryAction;
use codex_hepta_authbus_qualification::StatusObservation;
use codex_hepta_authbus_qualification::WriteDisposition;
use codex_hepta_authbus_qualification::WriterIdentity;
use codex_hepta_contracts::Sha256Digest;
use pretty_assertions::assert_eq;
use tempfile::TempDir;

fn digest(label: &str) -> Sha256Digest {
    Sha256Digest::for_bytes(label.as_bytes())
}

fn fence(generation: u64) -> QualificationFence {
    QualificationFence {
        authority_epoch: 3,
        owner_epoch: 7,
        generation,
        fencing_token_sha256: digest(&format!("fence-{generation}")),
    }
}

fn admission(label: &str, generation: u64, created_at_ms: u64) -> QualificationAdmission {
    let operation_fence = fence(generation);
    let intent = DurableOperationIntent {
        kind: QualificationOperationKind::Refresh,
        operation_id: format!("operation-{label}"),
        operation_key: format!("operation-key-{label}"),
        effect_key: format!("effect-key-{label}"),
        command_id: format!("command-{label}"),
        run_id: format!("run-{label}"),
        idempotency_key: format!("idempotency-{label}"),
        provider_id: "provider-a".to_string(),
        profile_id: "profile-a".to_string(),
        token_family_id: format!("family-{label}"),
        request_json: "{\"opaque\":true}".to_string(),
        request_sha256: digest(&format!("request-{label}")),
        payload_sha256: digest(&format!("payload-{label}")),
        policy_sha256: digest("policy"),
        scope_sha256: digest("scope"),
        purpose_sha256: digest("purpose"),
        fence: operation_fence.clone(),
        created_at_ms,
    };
    let permit = QualificationPermit {
        permit_id: format!("permit-{label}"),
        command_id: intent.command_id.clone(),
        idempotency_key: intent.idempotency_key.clone(),
        resource_id: "resource-a".to_string(),
        resource_sha256: digest("resource-a"),
        reserved: QualificationQuota {
            rpm: 1,
            tpm: 50,
            concurrency: 1,
            day_budget: 50,
            context: 128,
        },
        fence: operation_fence,
        issued_at_ms: created_at_ms - 1,
        expires_at_ms: created_at_ms + 10_000,
        authority: false,
    };
    QualificationAdmission::new(intent, permit).expect("valid admission")
}

async fn open_store(root: &TempDir, boot: &str, generation: u64) -> QualificationStore {
    QualificationStore::open(
        root.path(),
        WriterIdentity::new(boot, generation).expect("writer"),
        1_000 + generation,
    )
    .await
    .expect("open qualification store")
}

#[tokio::test]
async fn exact_admission_replays_and_changed_binding_conflicts() {
    let root = TempDir::new().expect("tempdir");
    let store = open_store(&root, "boot-a", 1).await;
    let original = admission("replay", 11, 2_000);

    let (first, snapshot) = store.admit(original.clone()).await.expect("admit");
    assert_eq!(first, AdmissionDisposition::Inserted);
    assert_eq!(snapshot.state, OperationState::IntentDurable);
    assert_eq!(snapshot.revision, 1);

    let (second, replay) = store.admit(original.clone()).await.expect("replay");
    assert_eq!(second, AdmissionDisposition::AlreadyPresent);
    assert_eq!(replay, snapshot);
    assert_eq!(store.operation_count().await.expect("count"), 1);

    let mut changed = original;
    changed.intent.payload_sha256 = digest("changed-payload");
    assert_eq!(
        store.admit(changed).await,
        Err(QualificationError::Conflict)
    );
    assert_eq!(store.operation_count().await.expect("count"), 1);
}

#[tokio::test]
async fn durable_attempt_reopens_lookup_only_and_never_auto_dispatches() {
    let root = TempDir::new().expect("tempdir");
    let operation = admission("lookup-only", 12, 2_100);
    let operation_id = operation.intent.operation_id.clone();
    let store = open_store(&root, "boot-a", 1).await;
    let (_, intent) = store.admit(operation).await.expect("admit");
    let ticket = store
        .begin_dispatch(&operation_id, intent.revision, 2_200)
        .await
        .expect("durable attempt");
    assert_eq!(ticket.attempt, 1);
    store.close().await;

    let reopened = open_store(&root, "boot-a", 1).await;
    assert_eq!(
        reopened
            .recover_operation(&operation_id)
            .await
            .expect("recover"),
        RecoveryAction::LookupOnly {
            operation_id,
            attempt: 1,
            revision: 2,
        }
    );
}

#[tokio::test]
async fn unknown_marker_then_lookup_completion_settles_quota_and_claim_atomically() {
    let root = TempDir::new().expect("tempdir");
    let operation = admission("terminal", 13, 2_300);
    let operation_id = operation.intent.operation_id.clone();
    let operation_fence = operation.intent.fence.clone();
    let store = open_store(&root, "boot-a", 1).await;
    let (_, intent) = store.admit(operation).await.expect("admit");
    let ticket = store
        .begin_dispatch(&operation_id, intent.revision, 2_400)
        .await
        .expect("attempt");
    let (_, unknown) = store
        .record_dispatch_observation(
            &ticket,
            DispatchObservation::Unknown {
                reason_code: "transport_response_lost".to_string(),
                observed_at_ms: 2_500,
            },
        )
        .await
        .expect("unknown marker");
    assert_eq!(unknown.state, OperationState::Unknown);
    assert_eq!(store.active_claim_count().await.expect("claim count"), 1);
    assert_eq!(
        store
            .quota_snapshot(&operation_id)
            .await
            .expect("quota")
            .state,
        QuotaReservationState::Held
    );

    let binding = store
        .status_binding_sha256(&operation_id)
        .await
        .expect("binding");
    let observation = StatusObservation {
        operation_id: operation_id.clone(),
        status_revision: 1,
        observed_at_ms: 2_600,
        binding_sha256: binding,
        fence: operation_fence,
        outcome: LookupOutcome::Completed {
            provider_operation_sha256: digest("provider-operation"),
            actual: QualificationQuota {
                rpm: 1,
                tpm: 20,
                concurrency: 1,
                day_budget: 20,
                context: 64,
            },
        },
    };
    let (disposition, terminal) = store
        .record_status_observation(observation.clone())
        .await
        .expect("terminal lookup");
    assert_eq!(disposition, WriteDisposition::Applied);
    assert_eq!(terminal.state, OperationState::Completed);
    assert_eq!(store.active_claim_count().await.expect("claim count"), 0);
    let quota = store.quota_snapshot(&operation_id).await.expect("quota");
    assert_eq!(quota.state, QuotaReservationState::Completed);
    assert_eq!(quota.used.concurrency, 0);

    assert_eq!(
        store.record_status_observation(observation).await,
        Ok((WriteDisposition::AlreadyPresent, terminal))
    );
    assert!(store.pending_outbox(10).await.expect("outbox").len() >= 2);
    store.verify_integrity().await.expect("integrity");
}

#[tokio::test]
async fn stale_status_revision_changed_digest_and_time_rollback_fail_closed() {
    let root = TempDir::new().expect("tempdir");
    let operation = admission("anti-replay", 14, 3_000);
    let operation_id = operation.intent.operation_id.clone();
    let operation_fence = operation.intent.fence.clone();
    let store = open_store(&root, "boot-a", 1).await;
    let (_, intent) = store.admit(operation).await.expect("admit");
    let ticket = store
        .begin_dispatch(&operation_id, intent.revision, 3_100)
        .await
        .expect("attempt");
    store
        .record_dispatch_observation(
            &ticket,
            DispatchObservation::Unknown {
                reason_code: "response_lost".to_string(),
                observed_at_ms: 3_200,
            },
        )
        .await
        .expect("unknown");
    let binding = store
        .status_binding_sha256(&operation_id)
        .await
        .expect("binding");
    let first = StatusObservation {
        operation_id: operation_id.clone(),
        status_revision: 1,
        observed_at_ms: 3_300,
        binding_sha256: binding.clone(),
        fence: operation_fence.clone(),
        outcome: LookupOutcome::Indeterminate {
            reason_code: "provider_unknown".to_string(),
        },
    };
    store
        .record_status_observation(first.clone())
        .await
        .expect("first observation");

    let mut changed_same_revision = first.clone();
    changed_same_revision.outcome = LookupOutcome::Unknown {
        reason_code: "changed".to_string(),
    };
    assert_eq!(
        store.record_status_observation(changed_same_revision).await,
        Err(QualificationError::ObservationConflict)
    );

    let rollback = StatusObservation {
        status_revision: 2,
        observed_at_ms: 3_299,
        ..first
    };
    assert_eq!(
        store.record_status_observation(rollback).await,
        Err(QualificationError::StaleObservation)
    );
}

#[tokio::test]
async fn writer_generation_rebind_fences_old_store_and_ticket() {
    let root = TempDir::new().expect("tempdir");
    let operation = admission("writer", 15, 4_000);
    let operation_id = operation.intent.operation_id.clone();
    let old = open_store(&root, "boot-old", 1).await;
    let (_, intent) = old.admit(operation).await.expect("admit");
    let ticket = old
        .begin_dispatch(&operation_id, intent.revision, 4_100)
        .await
        .expect("ticket");

    let current = open_store(&root, "boot-current", 2).await;
    assert_eq!(
        old.record_dispatch_observation(
            &ticket,
            DispatchObservation::Unknown {
                reason_code: "late-old-writer".to_string(),
                observed_at_ms: 4_200,
            },
        )
        .await,
        Err(QualificationError::StaleWriter)
    );
    assert_eq!(
        current
            .snapshot(&operation_id)
            .await
            .expect("snapshot")
            .state,
        OperationState::AttemptStarted
    );
}

#[tokio::test]
async fn commit_failpoints_rollback_entire_transaction() {
    let root = TempDir::new().expect("tempdir");
    let store = open_store(&root, "boot-a", 1).await;
    store.qualification_set_failpoint(QualificationFailpoint::AdmissionBeforeCommit);
    assert_eq!(
        store.admit(admission("failpoint", 16, 5_000)).await,
        Err(QualificationError::InjectedDiskFull)
    );
    assert_eq!(store.operation_count().await.expect("count"), 0);
    assert_eq!(store.active_claim_count().await.expect("claims"), 0);
    store.qualification_clear_failpoint(QualificationFailpoint::AdmissionBeforeCommit);
    store.verify_integrity().await.expect("integrity");
}

#[tokio::test]
async fn outbox_ack_uses_cursor_cas_and_exact_replay() {
    let root = TempDir::new().expect("tempdir");
    let operation = admission("outbox", 17, 6_000);
    let operation_id = operation.intent.operation_id.clone();
    let store = open_store(&root, "boot-a", 1).await;
    let (_, intent) = store.admit(operation).await.expect("admit");
    let ticket = store
        .begin_dispatch(&operation_id, intent.revision, 6_100)
        .await
        .expect("attempt");
    store
        .record_dispatch_observation(
            &ticket,
            DispatchObservation::Completed {
                provider_operation_sha256: digest("provider-complete"),
                actual: QualificationQuota {
                    rpm: 1,
                    tpm: 10,
                    concurrency: 1,
                    day_budget: 10,
                    context: 32,
                },
                observed_at_ms: 6_200,
            },
        )
        .await
        .expect("complete");
    let pending = store.pending_outbox(10).await.expect("outbox");
    let record = pending.first().expect("pending record");
    let ack = digest("receiver-ack");
    assert_eq!(store.outbox_cursor_revision().await.expect("cursor"), 0);
    assert_eq!(
        store
            .ack_outbox(&record.outbox_id, ack.clone(), 0, 6_300)
            .await,
        Ok(WriteDisposition::Applied)
    );
    assert_eq!(store.outbox_cursor_revision().await.expect("cursor"), 1);
    assert_eq!(
        store.ack_outbox(&record.outbox_id, ack, 1, 6_301).await,
        Ok(WriteDisposition::AlreadyPresent)
    );
}

#[tokio::test]
async fn corrupt_row_digest_is_detected_on_read_and_integrity_scan() {
    let root = TempDir::new().expect("tempdir");
    let operation = admission("corrupt", 18, 7_000);
    let operation_id = operation.intent.operation_id.clone();
    let store = open_store(&root, "boot-a", 1).await;
    store.admit(operation).await.expect("admit");
    store
        .qualification_inject_corrupt_operation_digest(&operation_id)
        .await
        .expect("inject");
    assert_eq!(
        store.snapshot(&operation_id).await,
        Err(QualificationError::Corrupt)
    );
    assert_eq!(
        store.verify_integrity().await,
        Err(QualificationError::Corrupt)
    );
}

#[tokio::test]
async fn schema_and_runtime_posture_exclude_raw_secret_columns() {
    let root = TempDir::new().expect("tempdir");
    let store = open_store(&root, "boot-a", 1).await;
    let columns = store
        .qualification_schema_columns()
        .await
        .expect("schema columns");
    for column in columns {
        let lower = column.to_ascii_lowercase();
        assert!(!lower.contains("access_token"));
        assert!(!lower.contains("refresh_token"));
        assert!(!lower.contains("client_secret"));
        assert!(!lower.contains("authorization"));
        assert!(!lower.contains("secret_bytes"));
    }
    assert!(
        !store
            .sqlite_runtime_version()
            .await
            .expect("sqlite")
            .is_empty()
    );
}
