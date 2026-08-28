use super::*;

use crate::cognitive_test_support::agent_id;
use crate::cognitive_test_support::layout;
use tempfile::TempDir;

fn digest(label: &str) -> Sha256Digest {
    Sha256Digest::for_bytes(label.as_bytes())
}

fn binding(operation: &str) -> IntelligenceMutationBinding {
    IntelligenceMutationBinding {
        operation_id: operation.to_string(),
        lease_id: format!("lease:{operation}"),
        lease_epoch: 7,
        expected_revision: Some(3),
        starting_projection_generation: 11,
        causal_root_sha256: digest(&format!("root:{operation}")),
    }
}

async fn request_for(
    store: &CognitiveStore,
    binding: &IntelligenceMutationBinding,
    action: IntelligenceMutationAction,
) -> IntelligenceMutationTransitionRequest {
    let state = store
        .replay_intelligence_mutation_operation(&binding.operation_id)
        .await
        .expect("replay");
    IntelligenceMutationTransitionRequest {
        binding: binding.clone(),
        sequence: state.next_sequence(),
        causal_parent_sha256: state.causal_parent_sha256(),
        action,
    }
}

async fn append(
    store: &CognitiveStore,
    binding: &IntelligenceMutationBinding,
    action: IntelligenceMutationAction,
) -> IntelligenceMutationJournalAppend {
    let request = request_for(store, binding, action).await;
    store
        .append_intelligence_mutation_transition(
            request,
            IntelligenceMutationJournalFault::None,
        )
        .await
        .expect("append")
}

async fn drive_to_terminal(
    store: &CognitiveStore,
    binding: &IntelligenceMutationBinding,
) -> IntelligenceMutationJournalAppend {
    append(
        store,
        binding,
        IntelligenceMutationAction::WitnessSource {
            source_sha256: digest("source"),
        },
    )
    .await;
    append(
        store,
        binding,
        IntelligenceMutationAction::ValidateGrounding {
            grounding_receipt_sha256: digest("grounding"),
        },
    )
    .await;
    append(
        store,
        binding,
        IntelligenceMutationAction::AppendDurableIntent {
            intent_sha256: digest("intent"),
        },
    )
    .await;
    append(
        store,
        binding,
        IntelligenceMutationAction::CommitMemoryFacts {
            write_receipt_sha256: digest("write"),
        },
    )
    .await;
    append(
        store,
        binding,
        IntelligenceMutationAction::PublishProjection {
            expected_previous_generation: 11,
            new_generation: 12,
            projection_receipt_sha256: digest("projection"),
        },
    )
    .await;
    append(
        store,
        binding,
        IntelligenceMutationAction::SettleOutbox {
            outcome_sha256: digest("settled"),
        },
    )
    .await;
    append(store, binding, IntelligenceMutationAction::Terminalize).await
}

#[tokio::test]
async fn journal_replays_normal_terminal_path_after_reopen() {
    let temp = TempDir::new().expect("temp");
    let owner = agent_id(241);
    let layout = layout(&temp, &owner);
    let operation = binding("operation:normal");
    {
        let store = CognitiveStore::open_with_intelligence_mutation_journal(&layout)
            .await
            .expect("store");
        assert_eq!(
            store
                .begin_intelligence_mutation_journal(&operation)
                .await
                .expect("begin"),
            IntelligenceMutationJournalDisposition::Applied
        );
        assert_eq!(
            store
                .begin_intelligence_mutation_journal(&operation)
                .await
                .expect("begin replay"),
            IntelligenceMutationJournalDisposition::Replay
        );
        let terminal = drive_to_terminal(&store, &operation).await;
        assert_eq!(terminal.receipt.to_phase, IntelligenceMutationPhase::Terminal);
        assert_eq!(terminal.receipt.memory_write_count, 1);
        assert_eq!(terminal.receipt.projection_publish_count, 1);
        assert!(terminal.receipt.durable_intent_settled);
        assert!(terminal.sqlite_persistence);
        assert!(!terminal.runtime_wired);
        assert!(!terminal.default_open_wired);
        assert!(!terminal.production_authority);
    }
    let reopened = CognitiveStore::open_with_intelligence_mutation_journal(&layout)
        .await
        .expect("reopen");
    let state = reopened
        .replay_intelligence_mutation_operation(&operation.operation_id)
        .await
        .expect("replay after reopen");
    assert_eq!(state.phase(), IntelligenceMutationPhase::Terminal);
    reopened
        .verify_intelligence_mutation_journal()
        .await
        .expect("verify");
}

#[tokio::test]
async fn exact_retry_replays_and_changed_retry_fails_closed() {
    let temp = TempDir::new().expect("temp");
    let owner = agent_id(242);
    let store = CognitiveStore::open_with_intelligence_mutation_journal(&layout(&temp, &owner))
        .await
        .expect("store");
    let operation = binding("operation:replay");
    store
        .begin_intelligence_mutation_journal(&operation)
        .await
        .expect("begin");
    let request = request_for(
        &store,
        &operation,
        IntelligenceMutationAction::WitnessSource {
            source_sha256: digest("source-a"),
        },
    )
    .await;
    let first = store
        .append_intelligence_mutation_transition(
            request.clone(),
            IntelligenceMutationJournalFault::None,
        )
        .await
        .expect("first");
    let replay = store
        .append_intelligence_mutation_transition(
            request.clone(),
            IntelligenceMutationJournalFault::None,
        )
        .await
        .expect("replay");
    assert_eq!(first.receipt, replay.receipt);
    assert_eq!(replay.disposition, IntelligenceMutationJournalDisposition::Replay);

    let changed = IntelligenceMutationTransitionRequest {
        action: IntelligenceMutationAction::WitnessSource {
            source_sha256: digest("source-b"),
        },
        ..request
    };
    assert!(matches!(
        store
            .append_intelligence_mutation_transition(
                changed,
                IntelligenceMutationJournalFault::None,
            )
            .await,
        Err(IntelligenceMutationJournalError::State(
            IntelligenceMutationStateError::ReplayConflict
        ))
    ));
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM cognitive_intelligence_mutation_transitions
         WHERE operation_id = ?",
    )
    .bind(&operation.operation_id)
    .fetch_one(&store.pool)
    .await
    .expect("count");
    assert_eq!(count, 1);
}

#[tokio::test]
async fn precommit_failpoints_roll_back_without_a_transition() {
    let temp = TempDir::new().expect("temp");
    let owner = agent_id(243);
    let store = CognitiveStore::open_with_intelligence_mutation_journal(&layout(&temp, &owner))
        .await
        .expect("store");
    let operation = binding("operation:rollback");
    store
        .begin_intelligence_mutation_journal(&operation)
        .await
        .expect("begin");
    for fault in [
        IntelligenceMutationJournalFault::BeforeTransitionInsert,
        IntelligenceMutationJournalFault::AfterTransitionInsertBeforeCommit,
    ] {
        let request = request_for(
            &store,
            &operation,
            IntelligenceMutationAction::WitnessSource {
                source_sha256: digest("source"),
            },
        )
        .await;
        assert!(matches!(
            store
                .append_intelligence_mutation_transition(request, fault)
                .await,
            Err(IntelligenceMutationJournalError::Injected(_))
        ));
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM cognitive_intelligence_mutation_transitions
             WHERE operation_id = ?",
        )
        .bind(&operation.operation_id)
        .fetch_one(&store.pool)
        .await
        .expect("count");
        assert_eq!(count, 0);
        assert_eq!(
            store
                .replay_intelligence_mutation_operation(&operation.operation_id)
                .await
                .expect("replay")
                .phase(),
            IntelligenceMutationPhase::Planned
        );
    }
}

#[tokio::test]
async fn postcommit_ack_loss_is_adopted_by_exact_retry() {
    let temp = TempDir::new().expect("temp");
    let owner = agent_id(244);
    let store = CognitiveStore::open_with_intelligence_mutation_journal(&layout(&temp, &owner))
        .await
        .expect("store");
    let operation = binding("operation:ack-loss");
    store
        .begin_intelligence_mutation_journal(&operation)
        .await
        .expect("begin");
    let request = request_for(
        &store,
        &operation,
        IntelligenceMutationAction::WitnessSource {
            source_sha256: digest("source"),
        },
    )
    .await;
    let lost_digest = match store
        .append_intelligence_mutation_transition(
            request.clone(),
            IntelligenceMutationJournalFault::AfterCommitBeforeReturn,
        )
        .await
    {
        Err(IntelligenceMutationJournalError::Indeterminate(digest)) => digest,
        other => panic!("unexpected result: {other:?}"),
    };
    let retry = store
        .append_intelligence_mutation_transition(
            request,
            IntelligenceMutationJournalFault::None,
        )
        .await
        .expect("retry");
    assert_eq!(retry.disposition, IntelligenceMutationJournalDisposition::Replay);
    assert_eq!(retry.receipt.transition_sha256, lost_digest);
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM cognitive_intelligence_mutation_transitions
         WHERE operation_id = ?",
    )
    .bind(&operation.operation_id)
    .fetch_one(&store.pool)
    .await
    .expect("count");
    assert_eq!(count, 1);
}

#[tokio::test]
async fn changed_operation_binding_is_rejected() {
    let temp = TempDir::new().expect("temp");
    let owner = agent_id(245);
    let store = CognitiveStore::open_with_intelligence_mutation_journal(&layout(&temp, &owner))
        .await
        .expect("store");
    let original = binding("operation:binding");
    store
        .begin_intelligence_mutation_journal(&original)
        .await
        .expect("begin");
    let mut changed = original;
    changed.lease_epoch += 1;
    assert!(matches!(
        store.begin_intelligence_mutation_journal(&changed).await,
        Err(IntelligenceMutationJournalError::Store(
            CognitiveStoreError::Conflict(_)
        ))
    ));
}

#[tokio::test]
async fn raw_sequence_gap_and_immutable_rows_are_rejected() {
    let temp = TempDir::new().expect("temp");
    let owner = agent_id(246);
    let store = CognitiveStore::open_with_intelligence_mutation_journal(&layout(&temp, &owner))
        .await
        .expect("store");
    let operation = binding("operation:raw-guard");
    store
        .begin_intelligence_mutation_journal(&operation)
        .await
        .expect("begin");
    let fake = digest("fake");
    let insert = sqlx::query(
        "INSERT INTO cognitive_intelligence_mutation_transitions (
            operation_id, sequence, from_phase, to_phase, action,
            action_payload_json, request_sha256, causal_parent_sha256,
            transition_sha256, durable_intent_appended,
            durable_intent_settled, memory_write_count,
            projection_publish_count, last_published_generation,
            recorded_at_unix_seconds
         ) VALUES (?, 1, 'planned', 'source_witnessed', 'witness_source',
                   '{}', ?, ?, ?, 0, 0, 0, 0, 11, unixepoch())",
    )
    .bind(&operation.operation_id)
    .bind(fake.as_str())
    .bind(fake.as_str())
    .bind(fake.as_str())
    .execute(&store.pool)
    .await;
    assert!(insert.is_err());

    let request = request_for(
        &store,
        &operation,
        IntelligenceMutationAction::WitnessSource {
            source_sha256: digest("source"),
        },
    )
    .await;
    store
        .append_intelligence_mutation_transition(
            request,
            IntelligenceMutationJournalFault::None,
        )
        .await
        .expect("append");
    assert!(
        sqlx::query(
            "UPDATE cognitive_intelligence_mutation_transitions
             SET action = action WHERE operation_id = ? AND sequence = 0",
        )
        .bind(&operation.operation_id)
        .execute(&store.pool)
        .await
        .is_err()
    );
    assert!(
        sqlx::query(
            "DELETE FROM cognitive_intelligence_mutation_operations
             WHERE operation_id = ?",
        )
        .bind(&operation.operation_id)
        .execute(&store.pool)
        .await
        .is_err()
    );
}

#[tokio::test]
async fn schema_drift_is_rejected_by_reopen_verifier() {
    let temp = TempDir::new().expect("temp");
    let owner = agent_id(247);
    let layout = layout(&temp, &owner);
    let store = CognitiveStore::open_with_intelligence_mutation_journal(&layout)
        .await
        .expect("store");
    sqlx::query("DROP INDEX cognitive_intelligence_mutation_transitions_phase_lookup")
        .execute(&store.pool)
        .await
        .expect("drop index");
    assert!(matches!(
        store.verify_intelligence_mutation_journal().await,
        Err(IntelligenceMutationJournalError::Store(
            CognitiveStoreError::Corrupt(_)
        ))
    ));
}
