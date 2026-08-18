use pretty_assertions::assert_eq;
use tempfile::TempDir;

use crate::CognitiveAccess;
use crate::CognitiveScope;
use crate::CognitiveStore;
use crate::CognitiveStoreError;
use crate::KgEdge;
use crate::KgNode;
use crate::MemoryDraft;
use crate::cognitive_test_support::agent_id;
use crate::cognitive_test_support::layout;
use crate::cognitive_test_support::memory_revision;
use crate::cognitive_test_support::source;
use crate::cognitive_test_support::workspace;

#[tokio::test]
async fn temporal_kg_rebuild_is_scoped_cited_and_fts_backed() {
    let temp = TempDir::new().expect("temp dir");
    let agent_id = agent_id(4);
    let store = CognitiveStore::open(&layout(&temp, &agent_id))
        .await
        .expect("store");
    let workspace_sha256 = workspace("research");
    let scope = CognitiveScope::WorkspacePrivate {
        workspace_sha256: workspace_sha256.clone(),
    };
    let access = CognitiveAccess::workspace_private(agent_id, workspace_sha256);
    let citation = store
        .append_source(
            &access,
            &source(scope.clone(), "kg-source", "Ada collaborated with Charles"),
        )
        .await
        .expect("source");
    let memory = store
        .create_memory(
            &access,
            &MemoryDraft {
                stable_key: "ada-collaboration".to_string(),
                revision: memory_revision(
                    scope.clone(),
                    "Ada collaborated with Charles.",
                    citation.clone(),
                ),
            },
        )
        .await
        .expect("memory");
    let ada = KgNode {
        node_id: "person:ada".to_string(),
        entity_type: "person".to_string(),
        label: "Ada Lovelace".to_string(),
        valid_from_unix_seconds: 100,
        valid_to_unix_seconds: None,
        memory: memory.id.clone(),
        source: citation.clone(),
    };
    let charles = KgNode {
        node_id: "person:charles".to_string(),
        entity_type: "person".to_string(),
        label: "Charles Babbage".to_string(),
        valid_from_unix_seconds: 100,
        valid_to_unix_seconds: None,
        memory: memory.id.clone(),
        source: citation.clone(),
    };
    let edge = KgEdge {
        edge_id: "relation:collaborated".to_string(),
        from_node_id: ada.node_id.clone(),
        to_node_id: charles.node_id.clone(),
        relation: "collaborated_with".to_string(),
        valid_from_unix_seconds: 100,
        valid_to_unix_seconds: None,
        memory: memory.id.clone(),
        source: citation.clone(),
    };
    assert_eq!(
        store
            .rebuild_kg_projection(&access, &scope, &[ada.clone(), charles], &[edge])
            .await
            .expect("first projection")
            .get(),
        1
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM kg_entity_fts WHERE kg_entity_fts MATCH 'Ada'"
        )
        .fetch_one(&store.pool)
        .await
        .expect("FTS5 query"),
        1
    );
    assert_eq!(
        store
            .rebuild_kg_projection(&access, &scope, &[ada], &[])
            .await
            .expect("second projection")
            .get(),
        2
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM kg_edges")
            .fetch_one(&store.pool)
            .await
            .expect("edge count"),
        0
    );

    let uncited_source = store
        .append_source(
            &access,
            &source(scope.clone(), "uncited", "unrelated source"),
        )
        .await
        .expect("uncited source");
    let bad_node = KgNode {
        node_id: "person:bad".to_string(),
        entity_type: "person".to_string(),
        label: "Uncited projection".to_string(),
        valid_from_unix_seconds: 100,
        valid_to_unix_seconds: None,
        memory: memory.id,
        source: uncited_source,
    };
    let error = store
        .rebuild_kg_projection(&access, &scope, &[bad_node], &[])
        .await
        .expect_err("uncited KG provenance must fail");
    assert!(matches!(error, CognitiveStoreError::Invalid(_)));
}
