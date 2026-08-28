use super::*;
use crate::CognitiveScope;
use crate::KgEntityFactDraft;
use crate::KgFactSetDraft;
use crate::KgRelationFactDraft;
use crate::LedgerSourceKind;
use crate::cognitive_test_support::agent_id;
use crate::cognitive_test_support::layout;
use tempfile::TempDir;

fn source(text: &str, event_key: &str) -> SourceDraft {
    SourceDraft {
        scope: CognitiveScope::AgentPrivate,
        kind: LedgerSourceKind::ExplicitMemoryDirective,
        event_key: event_key.to_string(),
        content: text.as_bytes().to_vec(),
        observed_at_unix_seconds: 100,
    }
}

fn memory(text: &str, stable_key: &str) -> MemoryDraft {
    MemoryDraft {
        stable_key: stable_key.to_string(),
        revision: MemoryRevisionDraft {
            scope: CognitiveScope::AgentPrivate,
            content: text.to_string(),
            verification: MemoryVerification::Verified,
            lifecycle: MemoryLifecycleState::Active,
            valid_from_unix_seconds: 100,
            valid_to_unix_seconds: None,
            citations: Vec::new(),
        },
    }
}

fn facts() -> KgFactSetDraft {
    KgFactSetDraft {
        entities: vec![
            KgEntityFactDraft {
                key: "aurora".to_string(),
                entity_type: "project".to_string(),
                label: "Project Aurora".to_string(),
            },
            KgEntityFactDraft {
                key: "rust".to_string(),
                entity_type: "language".to_string(),
                label: "Rust".to_string(),
            },
        ],
        relations: vec![KgRelationFactDraft {
            key: "aurora-uses-rust".to_string(),
            from_entity_key: "aurora".to_string(),
            to_entity_key: "rust".to_string(),
            relation: "uses".to_string(),
        }],
    }
}

fn grounded(text: &str) -> GroundedKgFactSetDraft {
    let start = text.find("Project Aurora uses Rust").expect("evidence");
    let end = start + "Project Aurora uses Rust".len();
    GroundedKgFactSetDraft {
        facts: facts(),
        evidence: vec![
            crate::FactEvidenceSpanDraft::from_source_text(
                GroundedFactKind::Entity,
                "aurora",
                text,
                start,
                end,
            )
            .expect("entity evidence"),
            crate::FactEvidenceSpanDraft::from_source_text(
                GroundedFactKind::Entity,
                "rust",
                text,
                start,
                end,
            )
            .expect("entity evidence"),
            crate::FactEvidenceSpanDraft::from_source_text(
                GroundedFactKind::Relation,
                "aurora-uses-rust",
                text,
                start,
                end,
            )
            .expect("relation evidence"),
        ],
    }
}

#[tokio::test]
async fn durable_grounding_round_trips_and_reopens() {
    let temp = TempDir::new().expect("temp");
    let owner = agent_id(211);
    let agent_layout = layout(&temp, &owner);
    let store = CognitiveStore::open_with_durable_fact_grounding(&agent_layout)
        .await
        .expect("store");
    let text = "Project Aurora uses Rust for deployment.";
    let receipt = store
        .remember_with_durable_grounded_kg(
            &CognitiveAccess::agent_private(owner.clone()),
            &source(text, "durable:event:1"),
            &memory(text, "durable-memory-1"),
            &grounded(text),
        )
        .await
        .expect("durable write");
    assert_eq!(
        store
            .durable_fact_grounding_status(
                &receipt.memory.id.memory_id,
                receipt.memory.id.revision,
            )
            .await
            .expect("status"),
        "grounded_v1"
    );
    assert!(
        store
            .durable_fact_grounding_receipt_digest(
                &receipt.memory.id.memory_id,
                receipt.memory.id.revision,
            )
            .await
            .expect("digest")
            .is_some()
    );
    drop(store);
    let reopened = CognitiveStore::open_with_durable_fact_grounding(&agent_layout)
        .await
        .expect("reopen");
    reopened
        .verify_durable_fact_grounding_ledger()
        .await
        .expect("verify");
}

#[tokio::test]
async fn legacy_and_zero_fact_statuses_are_explicit() {
    let temp = TempDir::new().expect("temp");
    let owner = agent_id(212);
    let store = CognitiveStore::open_with_durable_fact_grounding(&layout(&temp, &owner))
        .await
        .expect("store");
    let text = "Project Aurora uses Rust.";
    let legacy = store
        .remember_with_kg(
            &CognitiveAccess::agent_private(owner.clone()),
            &source(text, "legacy:event"),
            &memory(text, "legacy-memory"),
            &facts(),
        )
        .await
        .expect("legacy write");
    assert_eq!(
        store
            .durable_fact_grounding_status(
                &legacy.memory.id.memory_id,
                legacy.memory.id.revision,
            )
            .await
            .expect("legacy status"),
        "legacy_unreviewed"
    );

    let zero_text = "No structured facts requested.";
    let zero = store
        .remember_with_kg(
            &CognitiveAccess::agent_private(owner),
            &source(zero_text, "zero:event"),
            &memory(zero_text, "zero-memory"),
            &KgFactSetDraft::default(),
        )
        .await
        .expect("zero write");
    assert_eq!(
        store
            .durable_fact_grounding_status(
                &zero.memory.id.memory_id,
                zero.memory.id.revision,
            )
            .await
            .expect("zero status"),
        "zero_fact"
    );
}

#[tokio::test]
async fn invalid_grounding_rolls_back_without_rows() {
    let temp = TempDir::new().expect("temp");
    let owner = agent_id(213);
    let store = CognitiveStore::open_with_durable_fact_grounding(&layout(&temp, &owner))
        .await
        .expect("store");
    let text = "Project Aurora uses Rust.";
    let mut invalid = grounded(text);
    invalid.evidence.pop();
    let result = store
        .remember_with_durable_grounded_kg(
            &CognitiveAccess::agent_private(owner),
            &source(text, "invalid:event"),
            &memory(text, "invalid-memory"),
            &invalid,
        )
        .await;
    assert!(result.is_err());
    let sources: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM source_ledger")
        .fetch_one(&store.pool)
        .await
        .expect("source count");
    let memories: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM memory_revisions")
            .fetch_one(&store.pool)
            .await
            .expect("memory count");
    let grounding: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM kg_revision_fact_grounding_receipts",
    )
    .fetch_one(&store.pool)
    .await
    .expect("grounding count");
    assert_eq!((sources, memories, grounding), (0, 0, 0));
}

#[tokio::test]
async fn correction_persists_a_second_grounded_revision() {
    let temp = TempDir::new().expect("temp");
    let owner = agent_id(214);
    let store = CognitiveStore::open_with_durable_fact_grounding(&layout(&temp, &owner))
        .await
        .expect("store");
    let first_text = "Project Aurora uses Rust.";
    let first = store
        .remember_with_durable_grounded_kg(
            &CognitiveAccess::agent_private(owner.clone()),
            &source(first_text, "correct:event:1"),
            &memory(first_text, "correct-memory"),
            &grounded(first_text),
        )
        .await
        .expect("first");
    let second_text = "Project Aurora uses Rust for production.";
    let correction = MemoryRevisionDraft {
        scope: CognitiveScope::AgentPrivate,
        content: second_text.to_string(),
        verification: MemoryVerification::Verified,
        lifecycle: MemoryLifecycleState::Active,
        valid_from_unix_seconds: 200,
        valid_to_unix_seconds: None,
        citations: Vec::new(),
    };
    let second = store
        .correct_with_durable_grounded_kg(
            &CognitiveAccess::agent_private(owner),
            &first.memory.id.memory_id,
            first.memory.id.revision,
            &source(second_text, "correct:event:2"),
            &correction,
            &grounded(second_text),
        )
        .await
        .expect("second");
    assert_eq!(second.memory.id.revision, 2);
    assert_eq!(
        store
            .durable_fact_grounding_status(
                &second.memory.id.memory_id,
                second.memory.id.revision,
            )
            .await
            .expect("status"),
        "grounded_v1"
    );
}

#[tokio::test]
async fn tampered_evidence_digest_is_rejected_on_reopen() {
    let temp = TempDir::new().expect("temp");
    let owner = agent_id(215);
    let agent_layout = layout(&temp, &owner);
    let store = CognitiveStore::open_with_durable_fact_grounding(&agent_layout)
        .await
        .expect("store");
    let text = "Project Aurora uses Rust.";
    store
        .remember_with_durable_grounded_kg(
            &CognitiveAccess::agent_private(owner),
            &source(text, "tamper:event"),
            &memory(text, "tamper-memory"),
            &grounded(text),
        )
        .await
        .expect("write");

    sqlx::query("DROP TRIGGER kg_revision_fact_grounding_spans_no_update")
        .execute(&store.pool)
        .await
        .expect("drop guard");
    sqlx::query(
        "UPDATE kg_revision_fact_grounding_spans
         SET evidence_sha256 =
             '0000000000000000000000000000000000000000000000000000000000000000'
         WHERE rowid = (
             SELECT rowid FROM kg_revision_fact_grounding_spans LIMIT 1
         )",
    )
    .execute(&store.pool)
    .await
    .expect("tamper");
    sqlx::query(
        "CREATE TRIGGER kg_revision_fact_grounding_spans_no_update
         BEFORE UPDATE ON kg_revision_fact_grounding_spans BEGIN
             SELECT RAISE(ABORT, 'fact-grounding spans are immutable');
         END",
    )
    .execute(&store.pool)
    .await
    .expect("restore guard");
    drop(store);

    let error = match CognitiveStore::open_with_durable_fact_grounding(&agent_layout).await {
        Ok(_) => panic!("tampering must fail"),
        Err(error) => error,
    };
    assert!(matches!(error, CognitiveStoreError::Corrupt(_)));
}
