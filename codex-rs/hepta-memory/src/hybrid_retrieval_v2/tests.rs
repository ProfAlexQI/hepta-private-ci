use super::*;

use serde_json::Value;
use tempfile::TempDir;

use crate::CognitiveAccess;
use crate::CognitiveScope;
use crate::KgFactSetDraft;
use crate::LedgerSourceKind;
use crate::MemoryDraft;
use crate::MemoryLifecycleState;
use crate::MemoryRevisionDraft;
use crate::MemoryVerification;
use crate::SourceDraft;
use crate::cognitive_test_support::agent_id;
use crate::cognitive_test_support::layout;

fn digest(label: &str) -> String {
    Sha256Digest::for_bytes(label.as_bytes())
        .as_str()
        .to_string()
}

fn semantic_batch(
    query: &str,
    memory_id: &StableMemoryId,
    revision: u64,
    similarity_ppm: u32,
) -> SemanticCandidateBatchDraft {
    let mut batch = SemanticCandidateBatchDraft {
        query_sha256: Sha256Digest::for_bytes(query.as_bytes())
            .as_str()
            .to_string(),
        model_sha256: digest("semantic-model"),
        tokenizer_sha256: digest("semantic-tokenizer"),
        index_sha256: digest("semantic-index"),
        index_generation: 1,
        embedding_dimensions: 768,
        metric: SemanticSimilarityMetric::CosineSimilarityPpm,
        batch_sha256: String::new(),
        candidates: vec![SemanticCandidateDraft {
            memory_id: memory_id.as_str().to_string(),
            revision,
            rank: 1,
            similarity_ppm,
        }],
    };
    batch.batch_sha256 = semantic_batch_digest(&batch)
        .expect("semantic digest")
        .as_str()
        .to_string();
    batch
}

fn source(text: &str, event_key: &str, observed_at: i64) -> SourceDraft {
    SourceDraft {
        scope: CognitiveScope::AgentPrivate,
        kind: LedgerSourceKind::ExplicitMemoryDirective,
        event_key: event_key.to_string(),
        content: text.as_bytes().to_vec(),
        observed_at_unix_seconds: observed_at,
    }
}

fn memory(text: &str, stable_key: &str, valid_from: i64) -> MemoryDraft {
    MemoryDraft {
        stable_key: stable_key.to_string(),
        revision: MemoryRevisionDraft {
            scope: CognitiveScope::AgentPrivate,
            content: text.to_string(),
            verification: MemoryVerification::Verified,
            lifecycle: MemoryLifecycleState::Active,
            valid_from_unix_seconds: valid_from,
            valid_to_unix_seconds: None,
            citations: Vec::new(),
        },
    }
}

#[test]
fn planner_is_deterministic_and_intent_aware_for_mixed_language() {
    let request = HybridRetrievalRequestDraft {
        query: "Why did 项目 Aurora change recently?".to_string(),
        now_unix_seconds: 100,
        max_results: Some(4),
        semantic: None,
    };
    let request_json = serde_json::to_string(&request).expect("request");
    let first = CognitiveStore::plan_shadow_hybrid_retrieval_v2(&request_json)
        .expect("first plan");
    let second = CognitiveStore::plan_shadow_hybrid_retrieval_v2(&request_json)
        .expect("second plan");
    assert_eq!(first, second);
    let plan: Value = serde_json::from_str(&first).expect("plan json");
    assert_eq!(plan["intent"], "causal");
    assert_eq!(plan["requested_temporal_scope"], "current");
    assert_eq!(plan["language_profile"], "mixed_latin_cjk");
    assert!(plan["deterministic"].as_bool().unwrap_or(false));
    assert!(!plan["model_called"].as_bool().unwrap_or(true));
    assert!(!plan["query_persisted"].as_bool().unwrap_or(true));
    assert_eq!(
        plan["planner_sha256"].as_str().map(str::len),
        Some(64)
    );
}

#[test]
fn semantic_batch_is_query_bound_and_tamper_evident() {
    let owner = agent_id(251);
    let memory_id = StableMemoryId::for_key(
        &owner,
        &CognitiveScope::AgentPrivate,
        "semantic-batch-memory",
    );
    let query = "Project Aurora";
    let batch = semantic_batch(query, &memory_id, 1, 900_000);
    assert!(validate_semantic_batch(Some(&batch), query).is_ok());

    let mut changed_query = batch.clone();
    changed_query.query_sha256 = digest("other-query");
    assert!(validate_semantic_batch(Some(&changed_query), query).is_err());

    let mut changed_candidate = batch;
    changed_candidate.candidates[0].similarity_ppm = 899_999;
    assert!(validate_semantic_batch(Some(&changed_candidate), query).is_err());
}

#[test]
fn fusion_renormalizes_when_semantic_channel_is_absent() {
    let lexical_only = fused_score_ppm(1_000_000, 0, 500_000, 1_000_000, false);
    let semantic_present =
        fused_score_ppm(1_000_000, 1_000_000, 500_000, 1_000_000, true);
    assert!(lexical_only > 0);
    assert!(semantic_present > lexical_only);
    assert!(semantic_present <= SCORE_SCALE_PPM);
}

#[tokio::test]
async fn shadow_hybrid_union_reorders_with_semantic_evidence_without_product_mutation() {
    let temp = TempDir::new().expect("temp");
    let owner = agent_id(252);
    let store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("store");
    let access = CognitiveAccess::agent_private(owner);
    let empty_facts = KgFactSetDraft {
        entities: Vec::new(),
        relations: Vec::new(),
    };

    let aurora = "Project Aurora uses Rust for the local intelligence runtime.";
    let aurora_receipt = store
        .remember_with_kg(
            &access,
            &source(aurora, "hybrid:event:aurora", 100),
            &memory(aurora, "hybrid-memory-aurora", 100),
            &empty_facts,
        )
        .await
        .expect("aurora memory");
    let deployment = "The deployment plan and Friday release checklist are ready.";
    let deployment_receipt = store
        .remember_with_kg(
            &access,
            &source(deployment, "hybrid:event:deployment", 101),
            &memory(deployment, "hybrid-memory-deployment", 101),
            &empty_facts,
        )
        .await
        .expect("deployment memory");

    let source_count_before: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM source_ledger")
        .fetch_one(&store.pool)
        .await
        .expect("source count");
    let memory_count_before: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM memory_revisions")
            .fetch_one(&store.pool)
            .await
            .expect("memory count");
    let projection_count_before: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM kg_projection")
            .fetch_one(&store.pool)
            .await
            .expect("projection count");

    let query = "Project Aurora Rust";
    let request = HybridRetrievalRequestDraft {
        query: query.to_string(),
        now_unix_seconds: 200,
        max_results: Some(4),
        semantic: Some(semantic_batch(
            query,
            &deployment_receipt.memory.id.memory_id,
            deployment_receipt.memory.id.revision,
            1_000_000,
        )),
    };
    let receipt = store
        .shadow_hybrid_retrieve_v2(
            &access,
            &serde_json::to_string(&request).expect("request"),
        )
        .await
        .expect("hybrid receipt");
    let receipt: Value = serde_json::from_str(&receipt).expect("receipt json");
    assert_eq!(receipt["semantic_channel_present"], true);
    assert_eq!(receipt["semantic_host_supplied_untrusted"], true);
    assert_eq!(receipt["semantic_index_provenance_verified"], false);
    assert_eq!(receipt["local_embedding_executed"], false);
    assert_eq!(receipt["ann_index_executed"], false);
    assert_eq!(receipt["default_retrieval_changed"], false);
    assert_eq!(receipt["physical_send_changed"], false);
    assert_eq!(receipt["production_authority"], false);
    assert_eq!(
        receipt["candidates"][0]["memory_id"],
        deployment_receipt.memory.id.memory_id.as_str()
    );
    assert_ne!(
        receipt["candidates"][0]["memory_id"],
        aurora_receipt.memory.id.memory_id.as_str()
    );
    assert_eq!(
        receipt["receipt_sha256"].as_str().map(str::len),
        Some(64)
    );

    let source_count_after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM source_ledger")
        .fetch_one(&store.pool)
        .await
        .expect("source count after");
    let memory_count_after: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM memory_revisions")
            .fetch_one(&store.pool)
            .await
            .expect("memory count after");
    let projection_count_after: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM kg_projection")
            .fetch_one(&store.pool)
            .await
            .expect("projection count after");
    assert_eq!(source_count_after, source_count_before);
    assert_eq!(memory_count_after, memory_count_before);
    assert_eq!(projection_count_after, projection_count_before);
}
