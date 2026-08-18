use tempfile::TempDir;

use crate::CognitiveAccess;
use crate::CognitiveScope;
use crate::CognitiveStore;
use crate::CognitiveStoreError;
use crate::cognitive_test_support::agent_id;
use crate::cognitive_test_support::layout;
use crate::cognitive_test_support::source;
use crate::cognitive_test_support::workspace;

#[tokio::test]
async fn stores_are_per_agent_append_only_and_scope_fail_closed() {
    let temp = TempDir::new().expect("temp dir");
    let first_agent = agent_id(1);
    let second_agent = agent_id(2);
    let first = CognitiveStore::open(&layout(&temp, &first_agent))
        .await
        .expect("first store");
    let second = CognitiveStore::open(&layout(&temp, &second_agent))
        .await
        .expect("second store");
    assert_ne!(first.path(), second.path());

    let first_access = CognitiveAccess::agent_private(first_agent.clone());
    let source_draft = source(CognitiveScope::AgentPrivate, "explicit-1", "remember this");
    let appended = first
        .append_source(&first_access, &source_draft)
        .await
        .expect("append source");
    assert_eq!(
        first
            .append_source(&first_access, &source_draft)
            .await
            .expect("idempotent replay"),
        appended
    );

    let cross_agent = first
        .append_source(&CognitiveAccess::agent_private(second_agent), &source_draft)
        .await
        .expect_err("cross-agent write must fail");
    assert!(matches!(cross_agent, CognitiveStoreError::AccessDenied(_)));

    let scoped = source(
        CognitiveScope::WorkspacePrivate {
            workspace_sha256: workspace("alpha"),
        },
        "workspace-1",
        "private workspace fact",
    );
    let wrong_workspace = first
        .append_source(
            &CognitiveAccess::workspace_private(first_agent, workspace("beta")),
            &scoped,
        )
        .await
        .expect_err("workspace mismatch must fail");
    assert!(matches!(
        wrong_workspace,
        CognitiveStoreError::AccessDenied(_)
    ));

    let delete_error = sqlx::query("DELETE FROM source_ledger")
        .execute(&first.pool)
        .await
        .expect_err("source ledger must be immutable");
    assert!(
        delete_error
            .to_string()
            .contains("source ledger is immutable")
    );
}
