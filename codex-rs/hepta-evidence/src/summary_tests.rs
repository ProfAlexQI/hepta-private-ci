use codex_hepta_contracts::ActionId;
use codex_hepta_contracts::GOVERNANCE_SCHEMA_VERSION;
use codex_hepta_contracts::GovernanceDecision;
use codex_hepta_contracts::GovernanceDecisionRecord;
use codex_hepta_contracts::GovernanceMode;
use codex_hepta_contracts::GovernanceReceipt;
use codex_hepta_contracts::HandlerOutcome;
use codex_hepta_contracts::PolicyPhase;
use codex_hepta_contracts::PolicyStamp;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::ToolAction;
use codex_hepta_contracts::ToolActionSource;
use codex_state::SqliteConfig;
use codex_utils_absolute_path::AbsolutePathBuf;
use tempfile::TempDir;

use crate::AppendDisposition;
use crate::EvidenceSummary;
use crate::HeptaEvidenceStore;

fn sqlite_config(temp: &TempDir) -> SqliteConfig {
    SqliteConfig::new_for_testing(
        AbsolutePathBuf::try_from(temp.path().to_path_buf()).expect("absolute temp path"),
    )
}

fn decision() -> GovernanceDecisionRecord {
    GovernanceDecisionRecord::new(
        ToolAction {
            schema_version: GOVERNANCE_SCHEMA_VERSION,
            action_id: ActionId::for_tool_call("thread-1", "turn-1", "call-1"),
            thread_id: "thread-1".to_string(),
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: "exec_command".to_string(),
            source: ToolActionSource::Direct,
            payload_sha256: Sha256Digest::for_bytes(b"payload"),
        },
        PolicyPhase::Admission,
        GovernanceMode::Shadow,
        PolicyStamp::new("hepta.summary.test.v1", 1, b"allow"),
        GovernanceDecision::Allow,
    )
}

#[tokio::test]
async fn empty_summary_is_explicitly_zero_for_supported_families() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence");

    assert_eq!(
        store.summary().await.expect("summary"),
        EvidenceSummary::default()
    );
}

#[tokio::test]
async fn governance_summary_moves_from_pending_to_terminal() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence");
    let decision = decision();
    assert_eq!(
        store.append_decision(&decision).await.expect("decision"),
        AppendDisposition::Inserted
    );
    let pending = store.summary().await.expect("pending summary");
    assert_eq!(pending.governance.decisions, 1);
    assert_eq!(pending.governance.receipts, 0);
    assert_eq!(pending.governance.pending_actions, 1);

    let receipt = GovernanceReceipt::new(decision, None, false, HandlerOutcome::Blocked);
    assert_eq!(
        store.append_receipt(&receipt).await.expect("receipt"),
        AppendDisposition::Inserted
    );
    let terminal = store.summary().await.expect("terminal summary");
    assert_eq!(terminal.governance.decisions, 1);
    assert_eq!(terminal.governance.receipts, 1);
    assert_eq!(terminal.governance.pending_actions, 0);
}
