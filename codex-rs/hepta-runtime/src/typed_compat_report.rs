#[cfg(feature = "compat-report")]
use std::path::Path;

use serde_json::Value;
use thiserror::Error;

pub const TYPED_COMPAT_REPORT_IDS: &[&str] = &[
    "hepta-context-memory-ranked-recall-shadow-eval",
    "hepta-context-memory-shadow-quality-summary",
    "hepta-context-memory-shadow-quality-trend-snapshot",
    "hepta-context-memory-shadow-regression-dashboard",
    "hepta-context-memory-temporal-graph-shadow-eval",
    "hepta-context-memory-temporal-graph-shadow-replay",
    "hepta-context-memory-temporal-graph-shadow-retrieval-canary-guard",
    "hepta-context-memory-temporal-graph-shadow-retrieval-rollback-kill-switch",
    "hepta-context-memory-temporal-graph-shadow-store",
    "hepta-context-memory-temporal-graph-shadow-traversal-diff",
    "hepta-context-memory-temporal-graph-shadow-traversal-quality",
    "hepta-context-plane-activation-blocker-matrix",
    "hepta-context-plane-operator-approval-packet",
    "hepta-context-plane-operator-approval-packet-canonical-export-digest",
    "hepta-context-plane-operator-approval-packet-freshness",
    "hepta-context-plane-operator-approval-packet-freshness-dependency-chain",
    "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest",
    "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift",
    "hepta-context-plane-operator-approval-packet-negative-export",
    "hepta-systems-controlled-canary-readiness-plan",
    "hepta-systems-controlled-live-operator-packet-non-send-readback",
    "hepta-systems-controlled-live-operator-packet-preview",
    "hepta-systems-controlled-live-operator-readiness-dashboard",
    "hepta-systems-controlled-live-readiness-audit",
    "hepta-systems-controlled-live-readiness-denial-readback-index",
    "hepta-systems-controlled-live-required-evidence-collection-plan",
    "hepta-systems-controlled-live-required-evidence-gap-diff-view",
    "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment",
    "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-credential-boundary-readback",
    "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-kill-switch-rehearsal-boundary-readback",
    "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-non-send-readback",
    "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-rollback-rehearsal-boundary-readback",
    "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-transport-boundary-readback",
    "hepta-systems-controlled-live-required-evidence-gap-operator-readback",
    "hepta-systems-controlled-live-required-evidence-gap-summary",
    "hepta-systems-controlled-live-required-evidence-readback-index",
    "hepta-systems-current-reality-capability-matrix",
    "hepta-systems-dirty-worktree-release-boundary-actionable-clean-worktree-strategy",
    "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-approval-acceptance-boundary-readback",
    "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist",
    "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-packet-readback",
    "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-recording-boundary-readback",
    "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-evidence-recording-boundary-readback",
    "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet",
    "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-git-mutation-boundary-readback",
    "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-non-send-readback",
    "hepta-systems-dirty-worktree-release-boundary-grouping-freeze-operator-readback",
    "hepta-systems-dirty-worktree-release-boundary-grouping-freeze-plan",
    "hepta-systems-dirty-worktree-release-boundary-inventory",
    "hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-evidence-recording-boundary-readback-without-recording",
    "hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-rehearsal",
    "hepta-systems-dirty-worktree-release-boundary-release-risk-snapshot",
    "hepta-systems-dirty-worktree-release-boundary-test-only-clean-worktree-strategy-rehearsal",
    "hepta-systems-dirty-worktree-release-boundary-test-only-rehearsal-outcome-readback",
    "hepta-systems-plugin-contribution-point-abi",
    "hepta-systems-plugin-contribution-point-loader-binding",
    "hepta-systems-plugin-lifecycle-state-machine",
    "hepta-systems-plugin-tool-contribution-inventory-preview",
    "hepta-systems-plugin-tool-manifest-schema-cutover-preflight",
    "hepta-systems-work-graph-adapter-projection-fixture",
    "hepta-systems-work-graph-append-only-event-intake-preview",
    "hepta-systems-work-graph-append-only-store-enablement-precondition-preview",
    "hepta-systems-work-graph-append-only-store-enablement-precondition-readback-preview",
    "hepta-systems-work-graph-append-only-store-runtime-durable-store-switch-application-preview",
    "hepta-systems-work-graph-append-only-store-runtime-durable-store-switch-preview",
    "hepta-systems-work-graph-append-only-store-runtime-enablement-application-preview",
    "hepta-systems-work-graph-append-only-store-runtime-enablement-preview",
    "hepta-systems-work-graph-append-only-store-runtime-idempotency-mutation-application-preview",
    "hepta-systems-work-graph-append-only-store-runtime-rollback-readback-execution-application-preview",
    "hepta-systems-work-graph-append-only-store-runtime-rollback-readback-execution-preview",
    "hepta-systems-work-graph-append-only-store-runtime-rollback-readback-execution-readback-preview",
    "hepta-systems-work-graph-append-only-store-runtime-wal-write-boundary-execution-application-preview",
    "hepta-systems-work-graph-append-only-store-runtime-wal-write-boundary-execution-preview",
    "hepta-systems-work-graph-append-only-store-runtime-wal-write-boundary-execution-readback-preview",
    "hepta-systems-work-graph-append-only-store-runtime-write-boundary-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-adapter-projection-gap-closure-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-denial-evidence-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-denial-evidence-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-denial-evidence-readback-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-no-cutover-guard-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-operator-review-packet-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-operator-review-packet-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-readiness-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-readiness-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-readiness-readback-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-replay-readback-execution-closure-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-runtime-adapter-enforcement-closure-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-side-effect-lock-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-closeout-packet-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-readback-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-readback-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-replay-idempotency-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-replay-idempotency-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-replay-idempotency-readback-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-readback-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-readback-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-enablement-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-persistence-guard-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-replay-readback-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-shadow-write-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-shadow-write-preview",
    "hepta-systems-work-graph-canonical-adapter-inventory-application-preview",
    "hepta-systems-work-graph-canonical-adapter-inventory-preview",
    "hepta-systems-work-graph-canonical-adapter-inventory-readback-preview",
    "hepta-systems-work-graph-current-state-inventory",
    "hepta-systems-work-graph-idempotency-readback-adapter-preview",
    "hepta-systems-work-graph-observability-timeline-preview",
    "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-acknowledgement-preview",
    "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-acknowledgement-replay-idempotency-preview",
    "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-preview",
    "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-preview",
    "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-acknowledgement-replay-idempotency-preview",
    "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-preview",
    "hepta-systems-work-graph-projection-adapter-gap-closure-application-preview",
    "hepta-systems-work-graph-projection-adapter-gap-closure-preview",
    "hepta-systems-work-graph-projection-adapter-gap-closure-readback-preview",
    "hepta-systems-work-graph-role-manifest-enforcement-gap-closure-preview",
    "hepta-systems-work-graph-runtime-application-promotion-gap-closure-preview",
    "hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-preview",
    "hepta-systems-work-graph-state-store-persistence-preview",
    "hepta-systems-work-graph-store-idempotency-guard-gap-closure-preview",
    "hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-application-preview",
    "hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-preview",
    "hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-readback-preview",
    "hepta-systems-work-graph-terminal-task-result-wrapper-preview",
    "hepta-systems-work-graph-unified-projection-audit-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-append-only-store-runtime-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-canonical-adapter-inventory-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-operator-review-side-effect-lock-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-application-promotion-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-durable-store-switch-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-rollback-readback-execution-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-store-guard-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-adapter-projection-gap-closure-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-denial-evidence-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-final-no-enablement-readiness-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-no-cutover-guard-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-readiness-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-side-effect-lock-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-closeout-packet-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-operator-review-packet-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-enablement-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-persistence-guard-rerun-preview",
    "hepta-systems-workflow-durable-store-adapter",
    "hepta-systems-workflow-durable-store-test-only-append-fixture",
    "hepta-systems-workflow-temporal-lite-append-only-event-store-minimal-local-persistence",
    "hepta-systems-workflow-temporal-lite-append-only-event-store-test-implementation",
    "hepta-systems-workflow-temporal-lite-checkpoint-and-rollback-anchor-feature-gated-readback",
    "hepta-systems-workflow-temporal-lite-deterministic-replay-validator-feature-gated-readback",
    "hepta-systems-workflow-temporal-lite-deterministic-replay-validator-local-persistence-readback",
    "hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-feature-gated-readback",
    "hepta-systems-workflow-temporal-lite-lease-idempotency-index-feature-gated-readback",
    "hepta-systems-workflow-temporal-lite-lease-idempotency-index-local-persistence-readback",
    "hepta-systems-workflow-temporal-lite-replay-alignment-recovery-receipt-feature-gated-readback",
    "hepta-systems-workflow-temporal-lite-replay-alignment-recovery-receipt-local-persistence-readback",
    "hepta-systems-workflow-temporal-lite-replay-alignment-recovery-window-feature-gated-readback",
    "hepta-systems-workflow-temporal-lite-replay-alignment-recovery-window-local-persistence-readback",
    "hepta-systems-workflow-temporal-lite-work-graph-projection-feature-gated-readback",
];

#[derive(Debug, Error)]
pub enum TypedCompatReportError {
    #[error("unknown typed compatibility report: {0}")]
    UnknownReport(String),
    #[error("typed compatibility report contract violation: {0}")]
    ContractViolation(String),
    #[error("typed compatibility report serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
}

macro_rules! serialize_report {
    ($report:expr) => {
        serde_json::to_value($report).map_err(TypedCompatReportError::from)
    };
}

fn contract_object_mut<'a>(
    value: &'a mut Value,
    context: &str,
) -> Result<&'a mut serde_json::Map<String, Value>, TypedCompatReportError> {
    value.as_object_mut().ok_or_else(|| {
        TypedCompatReportError::ContractViolation(format!("{context} must be a JSON object"))
    })
}

fn take_contract_field(
    object: &mut serde_json::Map<String, Value>,
    field: &str,
    context: &str,
) -> Result<Value, TypedCompatReportError> {
    object.remove(field).ok_or_else(|| {
        TypedCompatReportError::ContractViolation(format!(
            "{context} must expose required field {field}"
        ))
    })
}

fn context_memory_shadow_fixture() -> (
    hepta_core::ContextMemoryRankedRecallShadowEvalReport,
    hepta_core::ContextMemoryShadowRegressionDashboardReport,
    hepta_core::ContextMemoryShadowQualitySummaryReport,
    hepta_core::ContextMemoryShadowQualityTrendSnapshotReport,
) {
    let snapshot = hepta_memory::StoreSnapshot {
        sessions: Vec::new(),
        memories: Vec::new(),
        transcripts: Vec::new(),
    };
    let request = hepta_core::ContextRecallRequest {
        session_id: hepta_core::SessionId("typed-compat-shadow".to_string()),
        query_text: None,
        recent_window_limit: 1,
        transcript_limit: 1,
        memory_limit: 1,
        allow_cross_session: false,
    };
    let ranked_recall = snapshot.context_memory_ranked_recall_shadow_eval_report();
    let dashboard = snapshot.context_memory_shadow_regression_dashboard_report(&request);
    let summary = snapshot.context_memory_shadow_quality_summary_report(&request);
    let trend = snapshot.context_memory_shadow_quality_trend_snapshot_report(&request);
    (ranked_recall, dashboard, summary, trend)
}

struct ContextMemoryTemporalGraphFixture {
    eval: hepta_core::ContextMemoryTemporalGraphShadowEvalReport,
    store: hepta_core::ContextMemoryTemporalGraphShadowStoreReport,
    replay: hepta_core::ContextMemoryTemporalGraphShadowReplayReport,
    traversal_diff: hepta_core::ContextMemoryTemporalGraphShadowTraversalDiffReport,
    traversal_quality: hepta_core::ContextMemoryTemporalGraphShadowTraversalQualityReport,
    retrieval_canary_guard: hepta_core::ContextMemoryTemporalGraphShadowRetrievalCanaryGuardReport,
    retrieval_rollback_kill_switch:
        hepta_core::ContextMemoryTemporalGraphShadowRetrievalRollbackKillSwitchReport,
}

fn context_memory_temporal_graph_fixture() -> ContextMemoryTemporalGraphFixture {
    let snapshot = hepta_memory::StoreSnapshot {
        sessions: Vec::new(),
        memories: vec![hepta_core::MemoryRecord {
            id: "typed-compat-memory".to_string(),
            scope: hepta_core::MemoryScope::LongTerm,
            content: "timeout retry guidance".to_string(),
        }],
        transcripts: vec![
            hepta_core::TranscriptEntry {
                entry_id: "typed-compat-session-1".to_string(),
                session_id: hepta_core::SessionId("typed-compat-session".to_string()),
                sequence: 1,
                kind: hepta_core::TranscriptEntryKind::Message,
                role: Some(hepta_core::MessageRole::Assistant),
                content: "timeout surfaced during tool run".to_string(),
                created_at_unix_ms: 101,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            },
            hepta_core::TranscriptEntry {
                entry_id: "typed-compat-session-2".to_string(),
                session_id: hepta_core::SessionId("typed-compat-session".to_string()),
                sequence: 2,
                kind: hepta_core::TranscriptEntryKind::Summary,
                role: Some(hepta_core::MessageRole::Assistant),
                content: "timeout retried successfully".to_string(),
                created_at_unix_ms: 102,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            },
        ],
    };
    let request = hepta_core::ContextRecallRequest {
        session_id: hepta_core::SessionId("typed-compat-session".to_string()),
        query_text: Some("timeout".to_string()),
        recent_window_limit: 1,
        transcript_limit: 1,
        memory_limit: 1,
        allow_cross_session: true,
    };
    ContextMemoryTemporalGraphFixture {
        eval: snapshot.context_memory_temporal_graph_shadow_eval_report(),
        store: snapshot.context_memory_temporal_graph_shadow_store_report(&request),
        replay: snapshot.context_memory_temporal_graph_shadow_replay_report(&request),
        traversal_diff: snapshot
            .context_memory_temporal_graph_shadow_traversal_diff_report(&request),
        traversal_quality: snapshot
            .context_memory_temporal_graph_shadow_traversal_quality_report(&request),
        retrieval_canary_guard: snapshot
            .context_memory_temporal_graph_shadow_retrieval_canary_guard_report(&request),
        retrieval_rollback_kill_switch: snapshot
            .context_memory_temporal_graph_shadow_retrieval_rollback_kill_switch_report(&request),
    }
}

fn state(value: bool, enabled: &'static str, disabled: &'static str) -> Value {
    Value::String(if value { enabled } else { disabled }.to_string())
}

fn context_memory_temporal_graph_typed_report<T: serde::Serialize>(
    report: &T,
    integrity: bool,
    gate: &str,
    schema: &str,
    legacy_business_fields: Value,
) -> Result<Value, TypedCompatReportError> {
    let mut value = context_memory_typed_report(report, integrity, gate, schema)?;
    let object = contract_object_mut(&mut value, "context-memory temporal-graph typed report")?;
    if !legacy_business_fields
        .as_object()
        .is_some_and(|fields| !fields.is_empty())
    {
        return Err(TypedCompatReportError::ContractViolation(format!(
            "{gate} must expose non-empty legacy business fields"
        )));
    }
    object.insert("legacy_business_fields".to_string(), legacy_business_fields);
    object.insert(
        "production_authority_granted".to_string(),
        Value::Bool(false),
    );
    object.insert("write_authority_granted".to_string(), Value::Bool(false));
    object.insert("ready_for_live_execution".to_string(), Value::Bool(false));
    object.insert("mutation_enabled".to_string(), Value::Bool(false));
    Ok(value)
}

fn temporal_graph_eval_legacy_business_fields(
    report: &hepta_core::ContextMemoryTemporalGraphShadowEvalReport,
) -> Value {
    serde_json::json!({
        "result": "pass",
        "payload_light": "pass",
        "schema": report.schema_version,
        "mode": match report.mode {
            hepta_core::ContextMemoryTemporalGraphShadowEvalMode::DeterministicShadow => "deterministic-shadow",
            hepta_core::ContextMemoryTemporalGraphShadowEvalMode::Unknown => "unknown",
        },
        "fixture_count": report.fixture_count(),
        "fixture_pass_count": report.fixture_pass_count(),
        "positive_fixture_count": report.positive_fixture_count(),
        "negative_fixture_count": report.negative_fixture_count(),
        "node_coverage_floor_basis_points": report.node_coverage_floor_basis_points,
        "edge_coverage_floor_basis_points": report.edge_coverage_floor_basis_points,
        "validity_window_floor_basis_points": report.validity_window_floor_basis_points,
        "supersedes_floor_basis_points": report.supersedes_floor_basis_points,
        "latency_max_ms": report.latency_max_ms,
        "regret_max_basis_points": report.regret_max_basis_points,
        "min_positive_node_coverage_basis_points": report.min_positive_node_coverage_basis_points(),
        "min_positive_edge_coverage_basis_points": report.min_positive_edge_coverage_basis_points(),
        "min_positive_validity_window_coverage_basis_points": report.min_positive_validity_window_coverage_basis_points(),
        "min_positive_supersedes_coverage_basis_points": report.min_positive_supersedes_coverage_basis_points(),
        "max_positive_latency_ms": report.max_positive_latency_ms(),
        "max_positive_regret_basis_points": report.max_positive_regret_basis_points(),
        "regression_fixture": state(report.regression_blocked_count() > 0, "blocked", "unblocked"),
        "operator_approval": state(report.operator_approval_required, "required", "not-required"),
        "production_route": state(report.production_route, "enabled", "disabled"),
        "graph_write": state(report.graph_write, "enabled", "disabled"),
        "runtime_activation": state(report.runtime_activation, "enabled", "disabled"),
    })
}

fn temporal_graph_store_legacy_business_fields(
    report: &hepta_core::ContextMemoryTemporalGraphShadowStoreReport,
) -> Value {
    serde_json::json!({
        "result": "pass",
        "payload_light": "pass",
        "schema": report.schema_version,
        "source_graph_schema": report.source_graph_schema_version,
        "mode": "approval-gated-shadow-store-skeleton",
        "node_count": report.node_count,
        "edge_count": report.edge_count,
        "provenance_edge_count": report.provenance_edge_count,
        "validity_window_edge_count": report.validity_window_edge_count,
        "supersedes_edge_count": report.supersedes_edge_count,
        "open_node_count": report.open_node_count,
        "invalidated_node_count": report.invalidated_node_count,
        "stage_required_count": report.readiness_stage_required_count(),
        "stage_projected_count": report.readiness_stage_projected_count(),
        "store_digest": state(!report.store_digest.is_empty(), "present", "missing"),
        "freshness_check": state(report.freshness_check_pass, "pass", "fail"),
        "replay_guard": state(report.replay_guard_pass, "pass", "fail"),
        "stale_replay_rejected": state(report.stale_replay_rejected, "pass", "fail"),
        "operator_approval": state(report.operator_approval_required, "required", "not-required"),
        "operator_approval_recorded_count": usize::from(report.operator_approval_recorded),
        "recorded_receipt_count": report.receipt_recorded_count(),
        "persisted_receipt_count": report.receipt_persisted_count(),
        "production_route": state(report.production_route, "enabled", "disabled"),
        "production_write_count": report.production_write_count(),
        "graph_write_count": report.graph_write_count(),
        "hot_path_write": state(report.hot_path_write, "enabled", "disabled"),
        "prompt_assembly_change": state(report.prompt_assembly_change, "enabled", "disabled"),
        "runtime_activation": state(report.runtime_activation, "enabled", "disabled"),
        "operator_activation": state(report.operator_activation_allowed, "enabled", "disabled"),
    })
}

fn temporal_graph_replay_legacy_business_fields(
    report: &hepta_core::ContextMemoryTemporalGraphShadowReplayReport,
) -> Value {
    serde_json::json!({
        "result": "pass",
        "payload_light": "pass",
        "schema": report.schema_version,
        "source_store_schema": report.source_store_schema_version,
        "mode": "approval-gated-shadow-wal-replay",
        "node_count": report.node_count,
        "edge_count": report.edge_count,
        "provenance_replay_count": report.provenance_replay_count,
        "bitemporal_validity_replay_count": report.bitemporal_validity_replay_count,
        "fact_invalidation_replay_count": report.fact_invalidation_replay_count,
        "supersede_tombstone_replay_count": report.supersede_tombstone_replay_count,
        "stage_required_count": report.replay_stage_required_count(),
        "stage_projected_count": report.replay_stage_projected_count(),
        "replay_digest_count": report.replay_digest_count(),
        "freshness_pass_count": report.freshness_pass_count(),
        "replay_guard_pass_count": report.replay_guard_pass_count(),
        "stale_replay_rejected_count": report.stale_replay_rejected_count(),
        "operator_approval": state(report.operator_approval_required, "required", "not-required"),
        "operator_approval_recorded_count": usize::from(report.operator_approval_recorded),
        "recorded_receipt_count": report.receipt_recorded_count(),
        "persisted_receipt_count": report.receipt_persisted_count(),
        "production_route": state(report.production_route, "enabled", "disabled"),
        "production_write_count": report.production_write_count(),
        "graph_write_count": report.graph_write_count(),
        "hot_path_write": state(report.hot_path_write, "enabled", "disabled"),
        "prompt_assembly_change": state(report.prompt_assembly_change, "enabled", "disabled"),
        "runtime_activation": state(report.runtime_activation, "enabled", "disabled"),
        "operator_activation": state(report.operator_activation_allowed, "enabled", "disabled"),
    })
}

fn temporal_graph_traversal_diff_legacy_business_fields(
    report: &hepta_core::ContextMemoryTemporalGraphShadowTraversalDiffReport,
) -> Value {
    serde_json::json!({
        "result": "pass",
        "payload_light": "pass",
        "schema": report.schema_version,
        "source_replay_schema": report.source_replay_schema_version,
        "mode": "shadow-retrieval-traversal-diff",
        "production_selection_count": report.production_selection_count,
        "lexical_bm25_candidate_count": report.lexical_bm25_candidate_count,
        "semantic_candidate_count": report.semantic_candidate_count,
        "graph_traversal_candidate_count": report.graph_traversal_candidate_count,
        "hybrid_candidate_count": report.hybrid_candidate_count,
        "overlap_candidate_count": report.overlap_candidate_count,
        "graph_expansion_candidate_count": report.graph_expansion_candidate_count,
        "win_count": report.traversal_diff_win_count,
        "loss_count": report.traversal_diff_loss_count,
        "cost_count": report.traversal_diff_cost_count,
        "stage_required_count": report.traversal_stage_required_count(),
        "stage_projected_count": report.traversal_stage_projected_count(),
        "digest_count": report.traversal_digest_count(),
        "freshness_pass_count": report.freshness_pass_count(),
        "replay_guard_pass_count": report.replay_guard_pass_count(),
        "stale_replay_rejected_count": report.stale_replay_rejected_count(),
        "aggregate_counters_only": state(report.aggregate_counters_only, "pass", "fail"),
        "llm_rerank": state(report.llm_rerank, "enabled", "disabled"),
        "graph_persistence": state(report.graph_persistence, "enabled", "disabled"),
        "production_route": state(report.production_route, "enabled", "disabled"),
        "production_write_count": report.production_write_count(),
        "graph_write_count": report.graph_write_count(),
        "hot_path_write": state(report.hot_path_write, "enabled", "disabled"),
        "prompt_assembly_change": state(report.prompt_assembly_change, "enabled", "disabled"),
        "runtime_activation": state(report.runtime_activation, "enabled", "disabled"),
        "operator_activation": state(report.operator_activation_allowed, "enabled", "disabled"),
    })
}

fn temporal_graph_traversal_quality_legacy_business_fields(
    report: &hepta_core::ContextMemoryTemporalGraphShadowTraversalQualityReport,
) -> Value {
    serde_json::json!({
        "result": "pass",
        "payload_light": "pass",
        "schema": report.schema_version,
        "source_traversal_diff_schema": report.source_traversal_diff_schema_version,
        "mode": "shadow-traversal-quality-slo",
        "fixture_count": report.quality_fixture_count,
        "slo_required_count": report.quality_slo_required_count,
        "slo_pass_count": report.quality_slo_pass_count,
        "coverage_basis_points": report.coverage_basis_points,
        "precision_basis_points": report.precision_basis_points,
        "leak_rate_basis_points": report.leak_rate_basis_points,
        "latency_budget_ms": report.latency_budget_ms,
        "projected_latency_ms": report.projected_latency_ms,
        "token_saved_estimate": report.token_saved_estimate,
        "operator_review_required_count": report.operator_review_required_count,
        "win_count": report.traversal_win_count,
        "loss_count": report.traversal_loss_count,
        "cost_count": report.traversal_cost_count,
        "stage_required_count": report.traversal_quality_stage_required_count(),
        "stage_projected_count": report.traversal_quality_stage_projected_count(),
        "digest_count": report.traversal_quality_digest_count(),
        "freshness_pass_count": report.freshness_pass_count(),
        "replay_guard_pass_count": report.replay_guard_pass_count(),
        "stale_replay_rejected_count": report.stale_replay_rejected_count(),
        "aggregate_counters_only": state(report.aggregate_counters_only, "pass", "fail"),
        "llm_rerank": state(report.llm_rerank, "enabled", "disabled"),
        "graph_persistence": state(report.graph_persistence, "enabled", "disabled"),
        "production_route": state(report.production_route, "enabled", "disabled"),
        "production_write_count": report.production_write_count(),
        "graph_write_count": report.graph_write_count(),
        "hot_path_write": state(report.hot_path_write, "enabled", "disabled"),
        "prompt_assembly_change": state(report.prompt_assembly_change, "enabled", "disabled"),
        "runtime_activation": state(report.runtime_activation, "enabled", "disabled"),
        "operator_activation": state(report.operator_activation_allowed, "enabled", "disabled"),
    })
}

fn temporal_graph_retrieval_canary_guard_legacy_business_fields(
    report: &hepta_core::ContextMemoryTemporalGraphShadowRetrievalCanaryGuardReport,
) -> Value {
    serde_json::json!({
        "result": "pass",
        "payload_light": "pass",
        "schema": report.schema_version,
        "source_traversal_quality_schema": report.source_traversal_quality_schema_version,
        "mode": "shadow-retrieval-canary-guard",
        "fixture_count": report.guard_fixture_count,
        "stage_required_count": report.guard_stage_required_count,
        "stage_projected_count": report.guard_stage_projected_count,
        "quality_slo_pass_count": report.quality_slo_pass_count,
        "operator_approval_required_count": report.operator_approval_required_count,
        "operator_approval_recorded_count": report.operator_approval_recorded_count,
        "feature_flag_registered_count": report.feature_flag_registered_count,
        "feature_flag_enabled_count": report.feature_flag_enabled_count,
        "kill_switch_registered_count": report.kill_switch_registered_count,
        "kill_switch_ready_count": report.kill_switch_ready_count,
        "rollback_rehearsal_required_count": report.rollback_rehearsal_required_count,
        "rollback_rehearsal_pass_count": report.rollback_rehearsal_pass_count,
        "activation_denial_count": report.activation_denial_count,
        "canary_route_opened_count": report.canary_route_opened_count,
        "digest_count": report.retrieval_canary_guard_digest_count(),
        "freshness_pass_count": report.freshness_pass_count(),
        "replay_guard_pass_count": report.replay_guard_pass_count(),
        "stale_replay_rejected_count": report.stale_replay_rejected_count(),
        "aggregate_counters_only": state(report.aggregate_counters_only, "pass", "fail"),
        "llm_rerank": state(report.llm_rerank, "enabled", "disabled"),
        "graph_persistence": state(report.graph_persistence, "enabled", "disabled"),
        "production_route": state(report.production_route, "enabled", "disabled"),
        "production_write_count": report.production_write_count(),
        "graph_write_count": report.graph_write_count(),
        "rollback_write_count": report.rollback_write_count(),
        "hot_path_write": state(report.hot_path_write, "enabled", "disabled"),
        "prompt_assembly_change": state(report.prompt_assembly_change, "enabled", "disabled"),
        "runtime_activation": state(report.runtime_activation, "enabled", "disabled"),
        "operator_activation": state(report.operator_activation_allowed, "enabled", "disabled"),
    })
}

fn temporal_graph_retrieval_rollback_kill_switch_legacy_business_fields(
    report: &hepta_core::ContextMemoryTemporalGraphShadowRetrievalRollbackKillSwitchReport,
) -> Value {
    serde_json::json!({
        "result": "pass",
        "payload_light": "pass",
        "schema": report.schema_version,
        "source_retrieval_canary_guard_schema": report.source_retrieval_canary_guard_schema_version,
        "mode": "shadow-retrieval-rollback-kill-switch",
        "fixture_count": report.evidence_fixture_count,
        "stage_required_count": report.evidence_stage_required_count,
        "stage_projected_count": report.evidence_stage_projected_count,
        "canary_guard_pass_count": report.canary_guard_pass_count,
        "operator_approval_required_count": report.operator_approval_required_count,
        "operator_approval_recorded_count": report.operator_approval_recorded_count,
        "feature_flag_registered_count": report.feature_flag_registered_count,
        "feature_flag_enabled_count": report.feature_flag_enabled_count,
        "kill_switch_registered_count": report.kill_switch_registered_count,
        "kill_switch_readback_count": report.kill_switch_readback_count,
        "kill_switch_pass_count": report.kill_switch_pass_count,
        "rollback_rehearsal_required_count": report.rollback_rehearsal_required_count,
        "rollback_rehearsal_readback_count": report.rollback_rehearsal_readback_count,
        "rollback_rehearsal_pass_count": report.rollback_rehearsal_pass_count,
        "route_denial_count": report.route_denial_count,
        "rollback_write_denial_count": report.rollback_write_denial_count,
        "canary_route_opened_count": report.canary_route_opened_count,
        "digest_count": report.retrieval_rollback_kill_switch_digest_count(),
        "freshness_pass_count": report.freshness_pass_count(),
        "replay_guard_pass_count": report.replay_guard_pass_count(),
        "stale_replay_rejected_count": report.stale_replay_rejected_count(),
        "aggregate_counters_only": state(report.aggregate_counters_only, "pass", "fail"),
        "llm_rerank": state(report.llm_rerank, "enabled", "disabled"),
        "graph_persistence": state(report.graph_persistence, "enabled", "disabled"),
        "production_route": state(report.production_route, "enabled", "disabled"),
        "production_write_count": report.production_write_count(),
        "graph_write_count": report.graph_write_count(),
        "rollback_write_count": report.rollback_write_count(),
        "hot_path_write": state(report.hot_path_write, "enabled", "disabled"),
        "prompt_assembly_change": state(report.prompt_assembly_change, "enabled", "disabled"),
        "runtime_activation": state(report.runtime_activation, "enabled", "disabled"),
        "operator_activation": state(report.operator_activation_allowed, "enabled", "disabled"),
    })
}

fn context_memory_typed_report<T: serde::Serialize>(
    report: &T,
    integrity: bool,
    gate: &str,
    schema: &str,
) -> Result<Value, TypedCompatReportError> {
    if !integrity {
        return Err(TypedCompatReportError::ContractViolation(format!(
            "{gate} source report failed its read-only integrity contract"
        )));
    }
    let mut value = serde_json::to_value(report)?;
    let object = contract_object_mut(&mut value, "context-memory typed report")?;
    let context_schema_version =
        take_contract_field(object, "schema_version", "context-memory source report")?;
    object.insert("context_schema_version".to_string(), context_schema_version);
    object.insert("runtime".to_string(), Value::String("hepta".to_string()));
    object.insert("product".to_string(), Value::String("Hepta".to_string()));
    object.insert("status".to_string(), Value::String("pass".to_string()));
    object.insert("gate".to_string(), Value::String(gate.to_string()));
    object.insert(
        "schema_version".to_string(),
        Value::String(schema.to_string()),
    );
    object.insert(
        "side_effects".to_string(),
        serde_json::json!({
            "channel_send_performed": false,
            "external_send_performed": false,
            "filesystem_written": false,
            "graph_state_persisted": false,
            "model_invoked": false,
            "provider_invoked": false,
            "runtime_mutation_performed": false
        }),
    );
    Ok(value)
}

fn workflow_durable_store_adapter_compat_report() -> Result<Value, TypedCompatReportError> {
    let append_plan = crate::hepta_workflow_durable_store_append_plan_report();
    let harness = crate::hepta_workflow_durable_store_adapter_harness_report();
    let adapter = crate::workflow_durable_store_adapter_report(&append_plan, &harness);
    let mut report = serde_json::to_value(adapter)?;
    let object = contract_object_mut(&mut report, "workflow durable store adapter report")?;

    let source_harness_surface =
        take_contract_field(object, "source_harness_surface", "typed adapter report")?;
    let source_harness_ready =
        take_contract_field(object, "source_harness_ready", "typed adapter report")?;
    object.insert(
        "source_adapter_harness_surface".to_string(),
        source_harness_surface,
    );
    object.insert(
        "source_adapter_harness_ready".to_string(),
        source_harness_ready,
    );
    object.insert(
        "source_append_only_event_intake_surface".to_string(),
        Value::String(append_plan.source_gate.to_string()),
    );
    object.insert(
        "source_append_only_event_intake_ready".to_string(),
        Value::Bool(append_plan.source_append_only_event_intake_ready),
    );
    object.insert(
        "source_append_only_event_contract_count".to_string(),
        Value::from(append_plan.event_contract_count),
    );
    object.insert("lib_export_present".to_string(), Value::Bool(true));
    object.insert(
        "local_gate".to_string(),
        Value::String(
            "scripts/lib/hepta-gate-pair-compat-v1/hepta-systems-workflow-durable-store-adapter.gate"
                .to_string(),
        ),
    );
    object.insert(
        "architecture_note".to_string(),
        Value::String(
            "docs/architecture/HEPTA_SYSTEMS_WORKFLOW_DURABLE_STORE_ADAPTER_2026-06-27.md"
                .to_string(),
        ),
    );
    object.insert("side_effect_free".to_string(), Value::Bool(true));
    object.insert(
        "source_files".to_string(),
        serde_json::json!({
            "adapter": "codex-rs/hepta-runtime/src/workflow_durable_store_adapter.rs",
            "append_only_intake_report": "scripts/hepta-systems-work-graph-append-only-event-intake-preview-report.sh",
            "append_plan": "codex-rs/hepta-runtime/src/workflow_durable_store_append_plan.rs",
            "harness": "codex-rs/hepta-runtime/src/workflow_durable_store_adapter_harness.rs"
        }),
    );
    object.remove("recommended_next_gate");
    object.insert(
        "next_migration_step".to_string(),
        Value::String(crate::WORKFLOW_DURABLE_STORE_ADAPTER_RECOMMENDED_NEXT_GATE.to_string()),
    );
    object.insert(
        "next_actions".to_string(),
        serde_json::json!([
            crate::WORKFLOW_DURABLE_STORE_ADAPTER_RECOMMENDED_NEXT_GATE,
            "keep_event_log_sqlite_replay_rollback_and_live_execution_disabled_until_explicit_cutover"
        ]),
    );

    let entries = object
        .get_mut("entries")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| {
            TypedCompatReportError::ContractViolation(
                "typed adapter report must expose entries as a JSON array".to_string(),
            )
        })?;
    if entries.len() != append_plan.append_plans.len() || entries.len() != harness.receipts.len() {
        return Err(TypedCompatReportError::ContractViolation(format!(
            "typed adapter entry count {} does not match append-plan count {} and harness count {}",
            entries.len(),
            append_plan.append_plans.len(),
            harness.receipts.len()
        )));
    }
    for (index, ((entry, plan), receipt)) in entries
        .iter_mut()
        .zip(&append_plan.append_plans)
        .zip(&harness.receipts)
        .enumerate()
    {
        let entry_event_contract_id = entry
            .get("event_contract_id")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                TypedCompatReportError::ContractViolation(format!(
                    "typed adapter entry {index} must expose string event_contract_id"
                ))
            })?;
        if entry_event_contract_id != plan.event_contract_id
            || plan.event_contract_id != receipt.event_contract_id
        {
            return Err(TypedCompatReportError::ContractViolation(format!(
                "typed adapter entry {index} event contract mismatch: report={entry_event_contract_id}, append_plan={}, harness={}",
                plan.event_contract_id, receipt.event_contract_id
            )));
        }
        let entry = contract_object_mut(entry, "typed adapter entry")?;
        entry.insert(
            "target_collection_ids".to_string(),
            serde_json::to_value(&plan.target_collection_ids)?,
        );
        entry.insert(
            "required_fields".to_string(),
            serde_json::to_value(&plan.required_fields)?,
        );
        entry.insert(
            "idempotency_key_fields".to_string(),
            serde_json::to_value(&plan.idempotency_key_fields)?,
        );
        entry.insert(
            "append_policy".to_string(),
            Value::String(plan.append_policy.to_string()),
        );
        entry.insert(
            "append_suppressed_by_feature_gate".to_string(),
            Value::Bool(receipt.append_suppressed_by_feature_gate),
        );
        entry.insert(
            "noop_receipt_projected".to_string(),
            Value::Bool(receipt.noop_receipt_projected),
        );
        entry.insert(
            "checkpoint_write_enabled".to_string(),
            Value::Bool(plan.checkpoint_write_enabled),
        );
    }

    let side_effects = object
        .get_mut("side_effects")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| {
            TypedCompatReportError::ContractViolation(
                "typed adapter report must expose side_effects as a JSON object".to_string(),
            )
        })?;
    side_effects.remove("live_execution_started");
    for key in [
        "report_written",
        "git_index_mutated",
        "workflow_event_log_mutated",
        "provider_invoked",
        "model_invoked",
        "gateway_or_auth_mutated",
        "native_post_mutation_performed",
        "channel_send_performed",
        "package_or_release_written",
        "public_ga_promoted",
    ] {
        side_effects.insert(key.to_string(), Value::Bool(false));
    }

    Ok(report)
}

pub fn typed_compat_report_with_dirty_worktree_observation(
    id: &str,
    observation: &crate::DirtyWorktreeObservation,
) -> Result<Value, TypedCompatReportError> {
    if !crate::is_dirty_worktree_typed_compat_report(id) {
        return Err(TypedCompatReportError::UnknownReport(id.to_string()));
    }
    crate::dirty_worktree_typed_compat_report(id, observation)
        .map_err(TypedCompatReportError::ContractViolation)
}

pub fn typed_compat_report_with_controlled_live_worktree_observation(
    id: &str,
    observation: &crate::ControlledLiveWorktreeObservation,
) -> Result<Value, TypedCompatReportError> {
    if !crate::is_controlled_live_typed_compat_report(id) {
        return Err(TypedCompatReportError::UnknownReport(id.to_string()));
    }
    crate::controlled_live_typed_compat_report(id, observation)
        .map_err(TypedCompatReportError::ContractViolation)
}

pub fn is_current_reality_typed_compat_report(id: &str) -> bool {
    id == "hepta-systems-current-reality-capability-matrix"
}

#[cfg(feature = "compat-report")]
pub fn typed_compat_report_with_current_reality_sources(
    id: &str,
    sources: &crate::CurrentRealityCapabilityMatrixSources,
) -> Result<Value, TypedCompatReportError> {
    if !is_current_reality_typed_compat_report(id) {
        return Err(TypedCompatReportError::UnknownReport(id.to_string()));
    }
    let report = crate::current_reality_capability_matrix_report_from_sources(sources)
        .map_err(|error| TypedCompatReportError::ContractViolation(error.to_string()))?;
    if !report.has_current_reality_integrity(sources) {
        return Err(TypedCompatReportError::ContractViolation(
            "typed current-reality report failed source integrity".to_string(),
        ));
    }
    serialize_report!(&report)
}

pub fn is_plugin_typed_compat_report(id: &str) -> bool {
    matches!(
        id,
        "hepta-systems-plugin-contribution-point-abi"
            | "hepta-systems-plugin-contribution-point-loader-binding"
            | "hepta-systems-plugin-lifecycle-state-machine"
            | "hepta-systems-plugin-tool-contribution-inventory-preview"
            | "hepta-systems-plugin-tool-manifest-schema-cutover-preflight"
    )
}

#[cfg(feature = "compat-report")]
pub fn typed_compat_report_with_plugin_repo_root(
    id: &str,
    repo_root: &Path,
) -> Result<Value, TypedCompatReportError> {
    if !is_plugin_typed_compat_report(id) {
        return Err(TypedCompatReportError::UnknownReport(id.to_string()));
    }
    let manifest_path = repo_root.join("plugins/hepta-system/.codex-plugin/plugin.json");
    let manifest_bytes = std::fs::read(&manifest_path).map_err(|error| {
        TypedCompatReportError::ContractViolation(format!(
            "cannot read validated plugin compatibility manifest: {error}"
        ))
    })?;
    let reports =
        crate::plugin_compat_report::build_plugin_compat_reports(repo_root, &manifest_bytes)
            .map_err(TypedCompatReportError::ContractViolation)?;
    let report = reports.report(id).ok_or_else(|| {
        TypedCompatReportError::ContractViolation(format!(
            "plugin compatibility report set omitted {id}"
        ))
    })?;
    serialize_report!(report)
}

pub fn typed_compat_report(id: &str) -> Result<Value, TypedCompatReportError> {
    if crate::context_plane_compat_report::is_context_plane_typed_compat_report(id) {
        return crate::context_plane_compat_report::context_plane_typed_compat_report(id)
            .map_err(TypedCompatReportError::ContractViolation);
    }
    if crate::is_controlled_live_typed_compat_report(id) {
        return Err(TypedCompatReportError::ContractViolation(format!(
            "controlled-live typed compatibility report requires an explicit repository observation: {id}"
        )));
    }
    if crate::is_dirty_worktree_typed_compat_report(id) {
        return Err(TypedCompatReportError::ContractViolation(format!(
            "dirty-worktree typed compatibility report requires an explicit repository observation: {id}"
        )));
    }
    if is_current_reality_typed_compat_report(id) {
        return Err(TypedCompatReportError::ContractViolation(format!(
            "current-reality typed compatibility report requires explicit repository sources: {id}"
        )));
    }
    if is_plugin_typed_compat_report(id) {
        return Err(TypedCompatReportError::ContractViolation(format!(
            "plugin typed compatibility report requires an explicit repository root: {id}"
        )));
    }
    match id {
        "hepta-context-memory-ranked-recall-shadow-eval" => {
            let (report, _, _, _) = context_memory_shadow_fixture();
            context_memory_typed_report(
                &report,
                report.has_ranked_recall_shadow_integrity(),
                "hepta_context_memory_ranked_recall_shadow_eval_gate",
                "context_memory_ranked_recall_shadow_eval_v1",
            )
        }
        "hepta-context-memory-shadow-regression-dashboard" => {
            let (_, report, _, _) = context_memory_shadow_fixture();
            context_memory_typed_report(
                &report,
                report.has_shadow_regression_dashboard_integrity(),
                "hepta_context_memory_shadow_regression_dashboard_gate",
                "context_memory_shadow_regression_dashboard_v1",
            )
        }
        "hepta-context-memory-shadow-quality-summary" => {
            let (_, _, report, _) = context_memory_shadow_fixture();
            context_memory_typed_report(
                &report,
                report.has_shadow_quality_summary_integrity(),
                "hepta_context_memory_shadow_quality_summary_gate",
                "context_memory_shadow_quality_summary_v1",
            )
        }
        "hepta-context-memory-shadow-quality-trend-snapshot" => {
            let (_, _, _, report) = context_memory_shadow_fixture();
            context_memory_typed_report(
                &report,
                report.has_shadow_quality_trend_snapshot_integrity(),
                "hepta_context_memory_shadow_quality_trend_snapshot_gate",
                "context_memory_shadow_quality_trend_snapshot_v1",
            )
        }
        "hepta-context-memory-temporal-graph-shadow-eval" => {
            let report = context_memory_temporal_graph_fixture().eval;
            let legacy_business_fields = temporal_graph_eval_legacy_business_fields(&report);
            context_memory_temporal_graph_typed_report(
                &report,
                report.has_temporal_graph_shadow_integrity(),
                "hepta_context_memory_temporal_graph_shadow_eval_gate",
                "context_memory_temporal_graph_shadow_eval_v1",
                legacy_business_fields,
            )
        }
        "hepta-context-memory-temporal-graph-shadow-store" => {
            let report = context_memory_temporal_graph_fixture().store;
            let legacy_business_fields = temporal_graph_store_legacy_business_fields(&report);
            context_memory_temporal_graph_typed_report(
                &report,
                report.has_shadow_store_integrity(),
                "hepta_context_memory_temporal_graph_shadow_store_gate",
                "context_memory_temporal_graph_shadow_store_v1",
                legacy_business_fields,
            )
        }
        "hepta-context-memory-temporal-graph-shadow-replay" => {
            let report = context_memory_temporal_graph_fixture().replay;
            let legacy_business_fields = temporal_graph_replay_legacy_business_fields(&report);
            context_memory_temporal_graph_typed_report(
                &report,
                report.has_shadow_replay_integrity(),
                "hepta_context_memory_temporal_graph_shadow_replay_gate",
                "context_memory_temporal_graph_shadow_replay_v1",
                legacy_business_fields,
            )
        }
        "hepta-context-memory-temporal-graph-shadow-traversal-diff" => {
            let report = context_memory_temporal_graph_fixture().traversal_diff;
            let legacy_business_fields =
                temporal_graph_traversal_diff_legacy_business_fields(&report);
            context_memory_temporal_graph_typed_report(
                &report,
                report.has_traversal_diff_integrity(),
                "hepta_context_memory_temporal_graph_shadow_traversal_diff_gate",
                "context_memory_temporal_graph_shadow_traversal_diff_v1",
                legacy_business_fields,
            )
        }
        "hepta-context-memory-temporal-graph-shadow-traversal-quality" => {
            let report = context_memory_temporal_graph_fixture().traversal_quality;
            let legacy_business_fields =
                temporal_graph_traversal_quality_legacy_business_fields(&report);
            context_memory_temporal_graph_typed_report(
                &report,
                report.has_traversal_quality_integrity(),
                "hepta_context_memory_temporal_graph_shadow_traversal_quality_gate",
                "context_memory_temporal_graph_shadow_traversal_quality_v1",
                legacy_business_fields,
            )
        }
        "hepta-context-memory-temporal-graph-shadow-retrieval-canary-guard" => {
            let report = context_memory_temporal_graph_fixture().retrieval_canary_guard;
            let legacy_business_fields =
                temporal_graph_retrieval_canary_guard_legacy_business_fields(&report);
            context_memory_temporal_graph_typed_report(
                &report,
                report.has_retrieval_canary_guard_integrity(),
                "hepta_context_memory_temporal_graph_shadow_retrieval_canary_guard_gate",
                "context_memory_temporal_graph_shadow_retrieval_canary_guard_v1",
                legacy_business_fields,
            )
        }
        "hepta-context-memory-temporal-graph-shadow-retrieval-rollback-kill-switch" => {
            let report = context_memory_temporal_graph_fixture()
                .retrieval_rollback_kill_switch;
            let legacy_business_fields =
                temporal_graph_retrieval_rollback_kill_switch_legacy_business_fields(&report);
            context_memory_temporal_graph_typed_report(
                &report,
                report.has_retrieval_rollback_kill_switch_integrity(),
                "hepta_context_memory_temporal_graph_shadow_retrieval_rollback_kill_switch_gate",
                "context_memory_temporal_graph_shadow_retrieval_rollback_kill_switch_v1",
                legacy_business_fields,
            )
        }
        "hepta-systems-current-reality-matrix-compact-cache-boundary-readback" => serialize_report!(crate::hepta_current_reality_matrix_compact_cache_boundary_readback_report()),
        "hepta-systems-work-graph-adapter-projection-fixture" => serialize_report!(crate::hepta_work_graph_adapter_projection_fixture_report()),
        "hepta-systems-work-graph-append-only-event-intake-preview" => serialize_report!(crate::hepta_work_graph_append_only_event_intake_preview_report()),
        "hepta-systems-work-graph-append-only-store-enablement-precondition-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_enablement_precondition_preview_report()),
        "hepta-systems-work-graph-append-only-store-enablement-precondition-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_enablement_precondition_readback_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-durable-store-switch-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_durable_store_switch_application_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-durable-store-switch-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_durable_store_switch_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-enablement-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_enablement_application_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-enablement-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_enablement_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-idempotency-mutation-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_idempotency_mutation_application_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-rollback-readback-execution-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_rollback_readback_execution_application_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-rollback-readback-execution-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_rollback_readback_execution_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-rollback-readback-execution-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_rollback_readback_execution_readback_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-wal-write-boundary-execution-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_application_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-wal-write-boundary-execution-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-wal-write-boundary-execution-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-write-boundary-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_write_boundary_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-adapter-projection-gap-closure-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-denial-evidence-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_denial_evidence_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-denial-evidence-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_denial_evidence_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-denial-evidence-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_denial_evidence_readback_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-no-cutover-guard-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_no_cutover_guard_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-operator-review-packet-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_operator_review_packet_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-operator-review-packet-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_operator_review_packet_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-readiness-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_readiness_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-readiness-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_readiness_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-readiness-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_readiness_readback_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-replay-readback-execution-closure-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-runtime-adapter-enforcement-closure-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-side-effect-lock-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_side_effect_lock_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-closeout-packet-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_closeout_packet_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_readback_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_readback_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-replay-idempotency-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-replay-idempotency-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-replay-idempotency-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_readback_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_readback_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_readback_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-enablement-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_enablement_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-persistence-guard-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_persistence_guard_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-replay-readback-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_replay_readback_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-shadow-write-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_shadow_write_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-shadow-write-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_shadow_write_preview_report()),
        "hepta-systems-work-graph-canonical-adapter-inventory-application-preview" => serialize_report!(crate::hepta_work_graph_canonical_adapter_inventory_application_preview_report()),
        "hepta-systems-work-graph-canonical-adapter-inventory-preview" => serialize_report!(crate::hepta_work_graph_canonical_adapter_inventory_preview_report()),
        "hepta-systems-work-graph-canonical-adapter-inventory-readback-preview" => serialize_report!(crate::hepta_work_graph_canonical_adapter_inventory_readback_preview_report()),
        "hepta-systems-work-graph-current-state-inventory" => serialize_report!(crate::hepta_work_graph_current_state_inventory_report()),
        "hepta-systems-work-graph-idempotency-readback-adapter-preview" => serialize_report!(crate::hepta_work_graph_idempotency_readback_adapter_preview_report()),
        "hepta-systems-work-graph-observability-timeline-preview" => serialize_report!(crate::hepta_work_graph_observability_timeline_preview_report()),
        "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-acknowledgement-preview" => serialize_report!(crate::hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_report()),
        "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-acknowledgement-replay-idempotency-preview" => serialize_report!(crate::hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_report()),
        "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-preview" => serialize_report!(crate::hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report()),
        "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-preview" => serialize_report!(crate::hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_report()),
        "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-acknowledgement-replay-idempotency-preview" => serialize_report!(crate::hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report()),
        "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-preview" => serialize_report!(crate::hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_report()),
        "hepta-systems-work-graph-projection-adapter-gap-closure-application-preview" => serialize_report!(crate::hepta_work_graph_projection_adapter_gap_closure_application_preview_report()),
        "hepta-systems-work-graph-projection-adapter-gap-closure-preview" => serialize_report!(crate::hepta_work_graph_projection_adapter_gap_closure_preview_report()),
        "hepta-systems-work-graph-projection-adapter-gap-closure-readback-preview" => serialize_report!(crate::hepta_work_graph_projection_adapter_gap_closure_readback_preview_report()),
        "hepta-systems-work-graph-role-manifest-enforcement-gap-closure-preview" => serialize_report!(crate::hepta_work_graph_role_manifest_enforcement_gap_closure_preview_report()),
        "hepta-systems-work-graph-runtime-application-promotion-gap-closure-preview" => serialize_report!(crate::hepta_work_graph_runtime_application_promotion_gap_closure_preview_report()),
        "hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-preview" => serialize_report!(crate::hepta_work_graph_scheduler_admission_enforcement_gap_closure_preview_report()),
        "hepta-systems-work-graph-state-store-persistence-preview" => serialize_report!(crate::hepta_work_graph_state_store_persistence_preview_report()),
        "hepta-systems-work-graph-store-idempotency-guard-gap-closure-preview" => serialize_report!(crate::hepta_work_graph_store_idempotency_guard_gap_closure_preview_report()),
        "hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-application-preview" => serialize_report!(crate::hepta_work_graph_terminal_task_result_enforcement_gap_closure_application_preview_report()),
        "hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-preview" => serialize_report!(crate::hepta_work_graph_terminal_task_result_enforcement_gap_closure_preview_report()),
        "hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-readback-preview" => serialize_report!(crate::hepta_work_graph_terminal_task_result_enforcement_gap_closure_readback_preview_report()),
        "hepta-systems-work-graph-terminal-task-result-wrapper-preview" => serialize_report!(crate::hepta_work_graph_terminal_task_result_wrapper_preview_report()),
        "hepta-systems-work-graph-unified-projection-audit-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_audit_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-append-only-store-runtime-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-canonical-adapter-inventory-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-operator-review-side-effect-lock-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-application-promotion-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-durable-store-switch-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_runtime_durable_store_switch_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-rollback-readback-execution-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-store-guard-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-adapter-projection-gap-closure-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_adapter_projection_gap_closure_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-denial-evidence-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_denial_evidence_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-final-no-enablement-readiness-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_final_no_enablement_readiness_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-no-cutover-guard-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_no_cutover_guard_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-readiness-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_readiness_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-side-effect-lock-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_side_effect_lock_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-closeout-packet-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_closeout_packet_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-operator-review-packet-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_operator_review_packet_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-enablement-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_enablement_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-persistence-guard-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_persistence_guard_rerun_preview_report()),
        "hepta-systems-workflow-durable-store-adapter" => {
            workflow_durable_store_adapter_compat_report()
        }
        "hepta-systems-workflow-durable-store-test-only-append-fixture" => serialize_report!(crate::hepta_workflow_durable_store_test_only_append_fixture_report()),
        "hepta-systems-workflow-temporal-lite-append-only-event-store-minimal-local-persistence" => serialize_report!(crate::hepta_workflow_temporal_lite_append_only_event_store_minimal_local_persistence_report()),
        "hepta-systems-workflow-temporal-lite-append-only-event-store-test-implementation" => serialize_report!(crate::hepta_workflow_temporal_lite_append_only_event_store_test_implementation_report()),
        "hepta-systems-workflow-temporal-lite-checkpoint-and-rollback-anchor-feature-gated-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_report()),
        "hepta-systems-workflow-temporal-lite-deterministic-replay-validator-feature-gated-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_report()),
        "hepta-systems-workflow-temporal-lite-deterministic-replay-validator-local-persistence-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_report()),
        "hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-feature-gated-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_report()),
        "hepta-systems-workflow-temporal-lite-lease-idempotency-index-feature-gated-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_report()),
        "hepta-systems-workflow-temporal-lite-lease-idempotency-index-local-persistence-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_report()),
        "hepta-systems-workflow-temporal-lite-replay-alignment-recovery-receipt-feature-gated-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_replay_alignment_recovery_receipt_feature_gated_readback_report()),
        "hepta-systems-workflow-temporal-lite-replay-alignment-recovery-receipt-local-persistence-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback_report()),
        "hepta-systems-workflow-temporal-lite-replay-alignment-recovery-window-feature-gated-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_replay_alignment_recovery_window_feature_gated_readback_report()),
        "hepta-systems-workflow-temporal-lite-replay-alignment-recovery-window-local-persistence-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_replay_alignment_recovery_window_local_persistence_readback_report()),
        "hepta-systems-workflow-temporal-lite-work-graph-projection-feature-gated-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_work_graph_projection_feature_gated_readback_report()),
        _ => Err(TypedCompatReportError::UnknownReport(id.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn typed_compatibility_report_registry_is_unique() {
        assert!(TYPED_COMPAT_REPORT_IDS.len() >= 100);
        assert_eq!(
            TYPED_COMPAT_REPORT_IDS
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .len(),
            TYPED_COMPAT_REPORT_IDS.len()
        );
    }

    #[test]
    fn representative_typed_compatibility_report_is_read_only() {
        let id = "hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-preview";
        let report = typed_compat_report(id).expect("typed report should render");
        let object = report
            .as_object()
            .expect("typed report should be an object");
        assert!(!object.is_empty());
        let side_effects = object
            .get("side_effects")
            .and_then(Value::as_object)
            .expect("typed report should declare side effects");
        assert!(
            side_effects
                .values()
                .all(|value| value == &Value::Bool(false))
        );
    }

    #[test]
    fn context_memory_shadow_reports_are_typed_and_read_only() {
        for id in [
            "hepta-context-memory-ranked-recall-shadow-eval",
            "hepta-context-memory-shadow-regression-dashboard",
            "hepta-context-memory-shadow-quality-summary",
            "hepta-context-memory-shadow-quality-trend-snapshot",
        ] {
            let report = typed_compat_report(id).expect("context-memory report should render");
            let object = report
                .as_object()
                .expect("context-memory report should be an object");
            assert_eq!(object.get("runtime"), Some(&Value::String("hepta".into())));
            assert_eq!(object.get("status"), Some(&Value::String("pass".into())));
            assert!(object.get("context_schema_version").is_some());
            assert!(
                object
                    .get("side_effects")
                    .and_then(Value::as_object)
                    .is_some_and(|effects| effects
                        .values()
                        .all(|value| value == &Value::Bool(false)))
            );
            for field in [
                "production_route",
                "production_write",
                "graph_write",
                "runtime_activation",
                "prompt_assembly_change",
                "operator_activation_allowed",
            ] {
                assert_eq!(
                    object.get(field),
                    Some(&Value::Bool(false)),
                    "{id}: {field}"
                );
            }
        }
    }

    #[test]
    fn context_plane_reports_are_typed_source_bound_and_read_only() {
        for id in crate::context_plane_compat_report::CONTEXT_PLANE_COMPAT_REPORT_IDS {
            let report = typed_compat_report(id).expect("context-plane report should render");
            let object = report
                .as_object()
                .expect("context-plane report should be an object");
            assert_eq!(object.get("runtime"), Some(&Value::String("hepta".into())));
            assert_eq!(object.get("product"), Some(&Value::String("Hepta".into())));
            assert_eq!(object.get("status"), Some(&Value::String("pass".into())));
            assert_eq!(object.get("gate"), Some(&Value::String((*id).into())));
            assert_eq!(
                object.get("production_authority_granted"),
                Some(&Value::Bool(false))
            );
            assert_eq!(
                object.get("write_authority_granted"),
                Some(&Value::Bool(false))
            );
            assert!(
                object
                    .get("legacy_business_fields")
                    .and_then(Value::as_object)
                    .is_some_and(|fields| !fields.is_empty())
            );
            assert!(
                object
                    .get("side_effects")
                    .and_then(Value::as_object)
                    .is_some_and(|effects| effects
                        .values()
                        .all(|value| value == &Value::Bool(false)))
            );
        }
    }

    fn legacy_shell_business_fields(prefix: &str, baseline: &str) -> Value {
        let mut fields = serde_json::Map::new();
        for line in baseline.lines().filter(|line| !line.is_empty()) {
            let (key, raw_value) = line
                .split_once('=')
                .expect("legacy baseline line should contain equals");
            let suffix = key
                .strip_prefix(prefix)
                .expect("legacy baseline line should use the expected prefix");
            let key = if suffix.is_empty() {
                "result".to_string()
            } else {
                suffix
                    .strip_prefix('.')
                    .expect("legacy baseline suffix should start with a dot")
                    .replace('-', "_")
            };
            let value = raw_value
                .parse::<u64>()
                .map(Value::from)
                .unwrap_or_else(|_| Value::String(raw_value.to_string()));
            assert!(fields.insert(key, value).is_none());
        }
        Value::Object(fields)
    }

    #[test]
    fn temporal_graph_typed_reports_recursively_preserve_legacy_business_fields() {
        let baselines = [
            (
                "hepta-context-memory-temporal-graph-shadow-eval",
                "temporal-graph-shadow-eval",
                r#"temporal-graph-shadow-eval=pass
temporal-graph-shadow-eval.payload-light=pass
temporal-graph-shadow-eval.schema=1
temporal-graph-shadow-eval.mode=deterministic-shadow
temporal-graph-shadow-eval.fixture-count=4
temporal-graph-shadow-eval.fixture-pass-count=4
temporal-graph-shadow-eval.positive-fixture-count=3
temporal-graph-shadow-eval.negative-fixture-count=1
temporal-graph-shadow-eval.node-coverage-floor-basis-points=10000
temporal-graph-shadow-eval.edge-coverage-floor-basis-points=10000
temporal-graph-shadow-eval.validity-window-floor-basis-points=10000
temporal-graph-shadow-eval.supersedes-floor-basis-points=10000
temporal-graph-shadow-eval.latency-max-ms=100
temporal-graph-shadow-eval.regret-max-basis-points=0
temporal-graph-shadow-eval.min-positive-node-coverage-basis-points=10000
temporal-graph-shadow-eval.min-positive-edge-coverage-basis-points=10000
temporal-graph-shadow-eval.min-positive-validity-window-coverage-basis-points=10000
temporal-graph-shadow-eval.min-positive-supersedes-coverage-basis-points=10000
temporal-graph-shadow-eval.max-positive-latency-ms=47
temporal-graph-shadow-eval.max-positive-regret-basis-points=0
temporal-graph-shadow-eval.regression-fixture=blocked
temporal-graph-shadow-eval.operator-approval=required
temporal-graph-shadow-eval.production-route=disabled
temporal-graph-shadow-eval.graph-write=disabled
temporal-graph-shadow-eval.runtime-activation=disabled"#,
            ),
            (
                "hepta-context-memory-temporal-graph-shadow-store",
                "temporal-graph-shadow-store",
                r#"temporal-graph-shadow-store=pass
temporal-graph-shadow-store.payload-light=pass
temporal-graph-shadow-store.schema=1
temporal-graph-shadow-store.source-graph-schema=1
temporal-graph-shadow-store.mode=approval-gated-shadow-store-skeleton
temporal-graph-shadow-store.node-count=5
temporal-graph-shadow-store.edge-count=10
temporal-graph-shadow-store.provenance-edge-count=5
temporal-graph-shadow-store.validity-window-edge-count=5
temporal-graph-shadow-store.supersedes-edge-count=0
temporal-graph-shadow-store.open-node-count=5
temporal-graph-shadow-store.invalidated-node-count=0
temporal-graph-shadow-store.stage-required-count=6
temporal-graph-shadow-store.stage-projected-count=6
temporal-graph-shadow-store.store-digest=present
temporal-graph-shadow-store.freshness-check=pass
temporal-graph-shadow-store.replay-guard=pass
temporal-graph-shadow-store.stale-replay-rejected=pass
temporal-graph-shadow-store.operator-approval=required
temporal-graph-shadow-store.operator-approval-recorded-count=0
temporal-graph-shadow-store.recorded-receipt-count=0
temporal-graph-shadow-store.persisted-receipt-count=0
temporal-graph-shadow-store.production-route=disabled
temporal-graph-shadow-store.production-write-count=0
temporal-graph-shadow-store.graph-write-count=0
temporal-graph-shadow-store.hot-path-write=disabled
temporal-graph-shadow-store.prompt-assembly-change=disabled
temporal-graph-shadow-store.runtime-activation=disabled
temporal-graph-shadow-store.operator-activation=disabled"#,
            ),
            (
                "hepta-context-memory-temporal-graph-shadow-replay",
                "temporal-graph-shadow-replay",
                r#"temporal-graph-shadow-replay=pass
temporal-graph-shadow-replay.payload-light=pass
temporal-graph-shadow-replay.schema=1
temporal-graph-shadow-replay.source-store-schema=1
temporal-graph-shadow-replay.mode=approval-gated-shadow-wal-replay
temporal-graph-shadow-replay.node-count=5
temporal-graph-shadow-replay.edge-count=10
temporal-graph-shadow-replay.provenance-replay-count=5
temporal-graph-shadow-replay.bitemporal-validity-replay-count=5
temporal-graph-shadow-replay.fact-invalidation-replay-count=0
temporal-graph-shadow-replay.supersede-tombstone-replay-count=0
temporal-graph-shadow-replay.stage-required-count=6
temporal-graph-shadow-replay.stage-projected-count=6
temporal-graph-shadow-replay.replay-digest-count=6
temporal-graph-shadow-replay.freshness-pass-count=6
temporal-graph-shadow-replay.replay-guard-pass-count=6
temporal-graph-shadow-replay.stale-replay-rejected-count=6
temporal-graph-shadow-replay.operator-approval=required
temporal-graph-shadow-replay.operator-approval-recorded-count=0
temporal-graph-shadow-replay.recorded-receipt-count=0
temporal-graph-shadow-replay.persisted-receipt-count=0
temporal-graph-shadow-replay.production-route=disabled
temporal-graph-shadow-replay.production-write-count=0
temporal-graph-shadow-replay.graph-write-count=0
temporal-graph-shadow-replay.hot-path-write=disabled
temporal-graph-shadow-replay.prompt-assembly-change=disabled
temporal-graph-shadow-replay.runtime-activation=disabled
temporal-graph-shadow-replay.operator-activation=disabled"#,
            ),
            (
                "hepta-context-memory-temporal-graph-shadow-traversal-diff",
                "temporal-graph-shadow-traversal-diff",
                r#"temporal-graph-shadow-traversal-diff=pass
temporal-graph-shadow-traversal-diff.payload-light=pass
temporal-graph-shadow-traversal-diff.schema=1
temporal-graph-shadow-traversal-diff.source-replay-schema=1
temporal-graph-shadow-traversal-diff.mode=shadow-retrieval-traversal-diff
temporal-graph-shadow-traversal-diff.production-selection-count=5
temporal-graph-shadow-traversal-diff.lexical-bm25-candidate-count=5
temporal-graph-shadow-traversal-diff.semantic-candidate-count=5
temporal-graph-shadow-traversal-diff.graph-traversal-candidate-count=10
temporal-graph-shadow-traversal-diff.hybrid-candidate-count=10
temporal-graph-shadow-traversal-diff.overlap-candidate-count=5
temporal-graph-shadow-traversal-diff.graph-expansion-candidate-count=5
temporal-graph-shadow-traversal-diff.win-count=1
temporal-graph-shadow-traversal-diff.loss-count=0
temporal-graph-shadow-traversal-diff.cost-count=5
temporal-graph-shadow-traversal-diff.stage-required-count=5
temporal-graph-shadow-traversal-diff.stage-projected-count=5
temporal-graph-shadow-traversal-diff.digest-count=5
temporal-graph-shadow-traversal-diff.freshness-pass-count=5
temporal-graph-shadow-traversal-diff.replay-guard-pass-count=5
temporal-graph-shadow-traversal-diff.stale-replay-rejected-count=5
temporal-graph-shadow-traversal-diff.aggregate-counters-only=pass
temporal-graph-shadow-traversal-diff.llm-rerank=disabled
temporal-graph-shadow-traversal-diff.graph-persistence=disabled
temporal-graph-shadow-traversal-diff.production-route=disabled
temporal-graph-shadow-traversal-diff.production-write-count=0
temporal-graph-shadow-traversal-diff.graph-write-count=0
temporal-graph-shadow-traversal-diff.hot-path-write=disabled
temporal-graph-shadow-traversal-diff.prompt-assembly-change=disabled
temporal-graph-shadow-traversal-diff.runtime-activation=disabled
temporal-graph-shadow-traversal-diff.operator-activation=disabled"#,
            ),
            (
                "hepta-context-memory-temporal-graph-shadow-traversal-quality",
                "temporal-graph-shadow-traversal-quality",
                r#"temporal-graph-shadow-traversal-quality=pass
temporal-graph-shadow-traversal-quality.payload-light=pass
temporal-graph-shadow-traversal-quality.schema=1
temporal-graph-shadow-traversal-quality.source-traversal-diff-schema=1
temporal-graph-shadow-traversal-quality.mode=shadow-traversal-quality-slo
temporal-graph-shadow-traversal-quality.fixture-count=5
temporal-graph-shadow-traversal-quality.slo-required-count=5
temporal-graph-shadow-traversal-quality.slo-pass-count=5
temporal-graph-shadow-traversal-quality.coverage-basis-points=10000
temporal-graph-shadow-traversal-quality.precision-basis-points=10000
temporal-graph-shadow-traversal-quality.leak-rate-basis-points=0
temporal-graph-shadow-traversal-quality.latency-budget-ms=20
temporal-graph-shadow-traversal-quality.projected-latency-ms=5
temporal-graph-shadow-traversal-quality.token-saved-estimate=768
temporal-graph-shadow-traversal-quality.operator-review-required-count=5
temporal-graph-shadow-traversal-quality.win-count=1
temporal-graph-shadow-traversal-quality.loss-count=0
temporal-graph-shadow-traversal-quality.cost-count=5
temporal-graph-shadow-traversal-quality.stage-required-count=5
temporal-graph-shadow-traversal-quality.stage-projected-count=5
temporal-graph-shadow-traversal-quality.digest-count=5
temporal-graph-shadow-traversal-quality.freshness-pass-count=5
temporal-graph-shadow-traversal-quality.replay-guard-pass-count=5
temporal-graph-shadow-traversal-quality.stale-replay-rejected-count=5
temporal-graph-shadow-traversal-quality.aggregate-counters-only=pass
temporal-graph-shadow-traversal-quality.llm-rerank=disabled
temporal-graph-shadow-traversal-quality.graph-persistence=disabled
temporal-graph-shadow-traversal-quality.production-route=disabled
temporal-graph-shadow-traversal-quality.production-write-count=0
temporal-graph-shadow-traversal-quality.graph-write-count=0
temporal-graph-shadow-traversal-quality.hot-path-write=disabled
temporal-graph-shadow-traversal-quality.prompt-assembly-change=disabled
temporal-graph-shadow-traversal-quality.runtime-activation=disabled
temporal-graph-shadow-traversal-quality.operator-activation=disabled"#,
            ),
            (
                "hepta-context-memory-temporal-graph-shadow-retrieval-canary-guard",
                "temporal-graph-shadow-retrieval-canary-guard",
                r#"temporal-graph-shadow-retrieval-canary-guard=pass
temporal-graph-shadow-retrieval-canary-guard.payload-light=pass
temporal-graph-shadow-retrieval-canary-guard.schema=1
temporal-graph-shadow-retrieval-canary-guard.source-traversal-quality-schema=1
temporal-graph-shadow-retrieval-canary-guard.mode=shadow-retrieval-canary-guard
temporal-graph-shadow-retrieval-canary-guard.fixture-count=5
temporal-graph-shadow-retrieval-canary-guard.stage-required-count=5
temporal-graph-shadow-retrieval-canary-guard.stage-projected-count=5
temporal-graph-shadow-retrieval-canary-guard.quality-slo-pass-count=5
temporal-graph-shadow-retrieval-canary-guard.operator-approval-required-count=5
temporal-graph-shadow-retrieval-canary-guard.operator-approval-recorded-count=0
temporal-graph-shadow-retrieval-canary-guard.feature-flag-registered-count=5
temporal-graph-shadow-retrieval-canary-guard.feature-flag-enabled-count=0
temporal-graph-shadow-retrieval-canary-guard.kill-switch-registered-count=5
temporal-graph-shadow-retrieval-canary-guard.kill-switch-ready-count=5
temporal-graph-shadow-retrieval-canary-guard.rollback-rehearsal-required-count=5
temporal-graph-shadow-retrieval-canary-guard.rollback-rehearsal-pass-count=5
temporal-graph-shadow-retrieval-canary-guard.activation-denial-count=5
temporal-graph-shadow-retrieval-canary-guard.canary-route-opened-count=0
temporal-graph-shadow-retrieval-canary-guard.digest-count=5
temporal-graph-shadow-retrieval-canary-guard.freshness-pass-count=5
temporal-graph-shadow-retrieval-canary-guard.replay-guard-pass-count=5
temporal-graph-shadow-retrieval-canary-guard.stale-replay-rejected-count=5
temporal-graph-shadow-retrieval-canary-guard.aggregate-counters-only=pass
temporal-graph-shadow-retrieval-canary-guard.llm-rerank=disabled
temporal-graph-shadow-retrieval-canary-guard.graph-persistence=disabled
temporal-graph-shadow-retrieval-canary-guard.production-route=disabled
temporal-graph-shadow-retrieval-canary-guard.production-write-count=0
temporal-graph-shadow-retrieval-canary-guard.graph-write-count=0
temporal-graph-shadow-retrieval-canary-guard.rollback-write-count=0
temporal-graph-shadow-retrieval-canary-guard.hot-path-write=disabled
temporal-graph-shadow-retrieval-canary-guard.prompt-assembly-change=disabled
temporal-graph-shadow-retrieval-canary-guard.runtime-activation=disabled
temporal-graph-shadow-retrieval-canary-guard.operator-activation=disabled"#,
            ),
            (
                "hepta-context-memory-temporal-graph-shadow-retrieval-rollback-kill-switch",
                "temporal-graph-shadow-retrieval-rollback-kill-switch",
                r#"temporal-graph-shadow-retrieval-rollback-kill-switch=pass
temporal-graph-shadow-retrieval-rollback-kill-switch.payload-light=pass
temporal-graph-shadow-retrieval-rollback-kill-switch.schema=1
temporal-graph-shadow-retrieval-rollback-kill-switch.source-retrieval-canary-guard-schema=1
temporal-graph-shadow-retrieval-rollback-kill-switch.mode=shadow-retrieval-rollback-kill-switch
temporal-graph-shadow-retrieval-rollback-kill-switch.fixture-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.stage-required-count=6
temporal-graph-shadow-retrieval-rollback-kill-switch.stage-projected-count=6
temporal-graph-shadow-retrieval-rollback-kill-switch.canary-guard-pass-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.operator-approval-required-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.operator-approval-recorded-count=0
temporal-graph-shadow-retrieval-rollback-kill-switch.feature-flag-registered-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.feature-flag-enabled-count=0
temporal-graph-shadow-retrieval-rollback-kill-switch.kill-switch-registered-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.kill-switch-readback-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.kill-switch-pass-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-rehearsal-required-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-rehearsal-readback-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-rehearsal-pass-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.route-denial-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-write-denial-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.canary-route-opened-count=0
temporal-graph-shadow-retrieval-rollback-kill-switch.digest-count=6
temporal-graph-shadow-retrieval-rollback-kill-switch.freshness-pass-count=6
temporal-graph-shadow-retrieval-rollback-kill-switch.replay-guard-pass-count=6
temporal-graph-shadow-retrieval-rollback-kill-switch.stale-replay-rejected-count=6
temporal-graph-shadow-retrieval-rollback-kill-switch.aggregate-counters-only=pass
temporal-graph-shadow-retrieval-rollback-kill-switch.llm-rerank=disabled
temporal-graph-shadow-retrieval-rollback-kill-switch.graph-persistence=disabled
temporal-graph-shadow-retrieval-rollback-kill-switch.production-route=disabled
temporal-graph-shadow-retrieval-rollback-kill-switch.production-write-count=0
temporal-graph-shadow-retrieval-rollback-kill-switch.graph-write-count=0
temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-write-count=0
temporal-graph-shadow-retrieval-rollback-kill-switch.hot-path-write=disabled
temporal-graph-shadow-retrieval-rollback-kill-switch.prompt-assembly-change=disabled
temporal-graph-shadow-retrieval-rollback-kill-switch.runtime-activation=disabled
temporal-graph-shadow-retrieval-rollback-kill-switch.operator-activation=disabled"#,
            ),
        ];

        for (id, prefix, baseline) in baselines {
            let expected = legacy_shell_business_fields(prefix, baseline);
            let report = typed_compat_report(id).expect("temporal graph report should render");
            assert_eq!(
                report.get("legacy_business_fields"),
                Some(&expected),
                "{id} legacy business projection drifted"
            );
            for field in [
                "production_authority_granted",
                "write_authority_granted",
                "ready_for_live_execution",
                "mutation_enabled",
            ] {
                assert_eq!(
                    report.get(field),
                    Some(&Value::Bool(false)),
                    "{id}: {field}"
                );
            }
        }
    }

    #[test]
    fn temporal_graph_typed_report_integrity_fails_closed() {
        let error = context_memory_temporal_graph_typed_report(
            &serde_json::json!({"production_write": false}),
            false,
            "hepta_context_memory_temporal_graph_shadow_eval_gate",
            "context_memory_temporal_graph_shadow_eval_v1",
            serde_json::json!({"result": "pass"}),
        )
        .expect_err("invalid source integrity must not render a compatibility report");
        assert!(matches!(
            error,
            TypedCompatReportError::ContractViolation(message)
                if message.contains("failed its read-only integrity contract")
        ));
    }

    #[test]
    fn durable_store_compatibility_report_preserves_legacy_business_fields() {
        let report = typed_compat_report("hepta-systems-workflow-durable-store-adapter")
            .expect("durable store compatibility report should render");
        let object = report
            .as_object()
            .expect("durable store compatibility report should be an object");
        for field in [
            "source_append_only_event_intake_surface",
            "source_append_only_event_intake_ready",
            "source_append_only_event_contract_count",
            "source_append_plan_surface",
            "source_append_plan_ready",
            "source_adapter_harness_surface",
            "source_adapter_harness_ready",
            "adapter_contract_ready",
            "temporal_lite_adapter_ready",
            "next_actions",
            "next_migration_step",
            "local_gate",
            "architecture_note",
            "source_files",
            "side_effect_free",
        ] {
            assert!(object.contains_key(field), "missing legacy field {field}");
        }
        assert!(!object.contains_key("source_harness_surface"));
        assert!(!object.contains_key("source_harness_ready"));
        assert!(!object.contains_key("recommended_next_gate"));

        let entries = object
            .get("entries")
            .and_then(Value::as_array)
            .expect("durable store compatibility report should expose entries");
        assert_eq!(entries.len(), 9);
        for entry in entries {
            let entry = entry
                .as_object()
                .expect("durable store compatibility entry should be an object");
            for field in [
                "target_collection_ids",
                "required_fields",
                "idempotency_key_fields",
                "append_policy",
                "append_suppressed_by_feature_gate",
                "noop_receipt_projected",
                "checkpoint_write_enabled",
            ] {
                assert!(entry.contains_key(field), "missing entry field {field}");
            }
        }

        let side_effects = object
            .get("side_effects")
            .and_then(Value::as_object)
            .expect("durable store compatibility report should expose side effects");
        for field in [
            "report_written",
            "git_index_mutated",
            "workflow_event_log_mutated",
            "provider_invoked",
            "model_invoked",
            "gateway_or_auth_mutated",
            "native_post_mutation_performed",
            "channel_send_performed",
            "package_or_release_written",
            "public_ga_promoted",
        ] {
            assert_eq!(side_effects.get(field), Some(&Value::Bool(false)));
        }
        assert!(!side_effects.contains_key("live_execution_started"));
    }

    #[test]
    fn durable_store_contract_helpers_fail_closed_without_panicking() {
        let mut non_object = Value::Null;
        assert!(matches!(
            contract_object_mut(&mut non_object, "fixture"),
            Err(TypedCompatReportError::ContractViolation(message))
                if message == "fixture must be a JSON object"
        ));

        let mut object = serde_json::Map::new();
        assert!(matches!(
            take_contract_field(&mut object, "required", "fixture"),
            Err(TypedCompatReportError::ContractViolation(message))
                if message == "fixture must expose required field required"
        ));
    }

    #[test]
    fn unknown_typed_compatibility_report_fails_closed() {
        assert!(matches!(
            typed_compat_report("unknown"),
            Err(TypedCompatReportError::UnknownReport(id)) if id == "unknown"
        ));
    }
}
