use codex_hepta_contracts::Sha256Digest;
use pretty_assertions::assert_eq;
use tempfile::TempDir;

use crate::CognitiveAccess;
use crate::CognitiveScope;
use crate::CognitiveStore;
use crate::CognitiveStoreError;
use crate::KgEdge;
use crate::KgNode;
use crate::MemoryDraft;
use crate::RetrievalChannel;
use crate::RetrievalRequest;
use crate::RevalidationDrift;
use crate::RevalidationStatus;
use crate::cognitive_test_support::agent_id;
use crate::cognitive_test_support::layout;
use crate::cognitive_test_support::memory_revision;
use crate::cognitive_test_support::source;
use crate::cognitive_test_support::workspace;

#[tokio::test]
async fn retrieval_rrf_is_deterministic_explainable_and_revalidated() {
    let temp = TempDir::new().expect("temp dir");
    let agent_id = agent_id(5);
    let store = CognitiveStore::open(&layout(&temp, &agent_id))
        .await
        .expect("store");
    let access = CognitiveAccess::agent_private(agent_id);
    let ada_source = store
        .append_source(
            &access,
            &source(
                CognitiveScope::AgentPrivate,
                "ada-source",
                "Ada studied analytical engines",
            ),
        )
        .await
        .expect("Ada source");
    let charles_source = store
        .append_source(
            &access,
            &source(
                CognitiveScope::AgentPrivate,
                "charles-source",
                "Charles designed an engine",
            ),
        )
        .await
        .expect("Charles source");
    let ada_memory = store
        .remember_memory(
            &access,
            &MemoryDraft {
                stable_key: "ada-engine".to_string(),
                revision: memory_revision(
                    CognitiveScope::AgentPrivate,
                    "Ada studied the analytical engine.",
                    ada_source.clone(),
                ),
            },
        )
        .await
        .expect("Ada memory");
    let charles_memory = store
        .remember_memory(
            &access,
            &MemoryDraft {
                stable_key: "charles-engine".to_string(),
                revision: memory_revision(
                    CognitiveScope::AgentPrivate,
                    "Charles designed the difference engine.",
                    charles_source.clone(),
                ),
            },
        )
        .await
        .expect("Charles memory");
    let ada = KgNode {
        node_id: "person:ada".to_string(),
        entity_type: "person".to_string(),
        label: "Ada Lovelace".to_string(),
        valid_from_unix_seconds: 100,
        valid_to_unix_seconds: None,
        memory: ada_memory.id.clone(),
        source: ada_source.clone(),
    };
    let charles = KgNode {
        node_id: "person:charles".to_string(),
        entity_type: "person".to_string(),
        label: "Charles Babbage".to_string(),
        valid_from_unix_seconds: 100,
        valid_to_unix_seconds: None,
        memory: charles_memory.id,
        source: charles_source,
    };
    let edge = KgEdge {
        edge_id: "relation:collaborated".to_string(),
        from_node_id: ada.node_id.clone(),
        to_node_id: charles.node_id.clone(),
        relation: "collaborated_with".to_string(),
        valid_from_unix_seconds: 100,
        valid_to_unix_seconds: None,
        memory: ada_memory.id.clone(),
        source: ada_source.clone(),
    };
    store
        .rebuild_kg_projection(
            &access,
            &CognitiveScope::AgentPrivate,
            &[ada.clone(), charles],
            &[edge],
        )
        .await
        .expect("projection");

    let request = RetrievalRequest::new("Ada", 200);
    let first = store
        .retrieve_memory_candidates(&access, &request)
        .await
        .expect("first retrieval");
    let second = store
        .retrieve_memory_candidates(&access, &request)
        .await
        .expect("second retrieval");
    assert_eq!(second, first);
    assert_eq!(first.candidates[0].memory.id, ada_memory.id);
    assert_eq!(
        first.candidates[0].channels,
        vec![
            RetrievalChannel::MemoryFts,
            RetrievalChannel::EntityFts,
            RetrievalChannel::GraphOneHop,
            RetrievalChannel::Recency,
        ]
    );
    let explanation = store
        .explain_memory_head(&access, &ada_memory.id.memory_id)
        .await
        .expect("explanation");
    assert_eq!(
        explanation.citations[0].content,
        b"Ada studied analytical engines"
    );
    assert!(matches!(
        store
            .revalidate_memory_candidate(&access, &first.candidates[0].revalidation, 200,)
            .await
            .expect("revalidation"),
        RevalidationStatus::Current(_)
    ));

    let mut source_drift = first.candidates[0].revalidation.clone();
    source_drift.citations[0].content_sha256 = Sha256Digest::for_bytes(b"different source");
    assert_eq!(
        store
            .revalidate_memory_candidate(&access, &source_drift, 200)
            .await
            .expect("source drift"),
        RevalidationStatus::Stale(RevalidationDrift::SourceHash)
    );

    store
        .rebuild_kg_projection(&access, &CognitiveScope::AgentPrivate, &[ada], &[])
        .await
        .expect("replacement projection");
    assert_eq!(
        store
            .revalidate_memory_candidate(&access, &first.candidates[0].revalidation, 200,)
            .await
            .expect("generation drift"),
        RevalidationStatus::Stale(RevalidationDrift::KgProjectionGeneration)
    );

    let corrected = memory_revision(
        CognitiveScope::AgentPrivate,
        "Ada wrote the first published algorithm.",
        ada_source,
    );
    store
        .correct_memory(&access, &ada_memory.id.memory_id, 1, &corrected)
        .await
        .expect("correction");
    assert_eq!(
        store
            .revalidate_memory_candidate(&access, &first.candidates[0].revalidation, 200,)
            .await
            .expect("head drift"),
        RevalidationStatus::Stale(RevalidationDrift::HeadRevision)
    );
}

#[tokio::test]
async fn retrieval_fails_closed_across_agent_and_workspace_scope() {
    let temp = TempDir::new().expect("temp dir");
    let owner = agent_id(6);
    let store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("store");
    let allowed_workspace = workspace("allowed");
    let denied_workspace = workspace("denied");
    let scope = CognitiveScope::WorkspacePrivate {
        workspace_sha256: allowed_workspace.clone(),
    };
    let allowed = CognitiveAccess::workspace_private(owner.clone(), allowed_workspace);
    let citation = store
        .append_source(
            &allowed,
            &source(scope.clone(), "private-source", "private astronomy notes"),
        )
        .await
        .expect("source");
    let memory = store
        .remember_memory(
            &allowed,
            &MemoryDraft {
                stable_key: "private-notes".to_string(),
                revision: memory_revision(scope, "Private astronomy notes.", citation),
            },
        )
        .await
        .expect("memory");

    let denied = CognitiveAccess::workspace_private(owner, denied_workspace);
    assert!(
        store
            .retrieve_memory_candidates(&denied, &RetrievalRequest::new("astronomy", 200))
            .await
            .expect("scoped retrieval")
            .candidates
            .is_empty()
    );
    assert!(matches!(
        store.read_memory_head(&denied, &memory.id.memory_id).await,
        Err(CognitiveStoreError::AccessDenied(_))
    ));
    let other_agent = CognitiveAccess::agent_private(agent_id(7));
    assert!(matches!(
        store
            .retrieve_memory_candidates(&other_agent, &RetrievalRequest::new("astronomy", 200),)
            .await,
        Err(CognitiveStoreError::AccessDenied(_))
    ));
}
