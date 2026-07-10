#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"

PLUGIN_ABI_REPORT="$ROOT/scripts/hepta-systems-plugin-contribution-point-abi-report.sh"
PLUGIN_LOADER_REPORT="$ROOT/scripts/hepta-systems-plugin-contribution-point-loader-binding-report.sh"
PLUGIN_TOOL_INVENTORY_REPORT="$ROOT/scripts/hepta-systems-plugin-tool-contribution-inventory-preview-report.sh"
PLUGIN_LIFECYCLE_REPORT="$ROOT/scripts/hepta-systems-plugin-lifecycle-state-machine-report.sh"
TOOL_SOURCE_REPORT="$ROOT/scripts/hepta-systems-tool-registry-invocation-source-of-truth-report.sh"
TOOL_READ_ONLY_DISPATCH_REPORT="$ROOT/scripts/hepta-systems-tool-registry-read-only-dispatch-preflight-report.sh"
WORK_GRAPH_DURABLE_REPORT="$ROOT/scripts/hepta-systems-work-graph-durable-identity-preview-report.sh"
WORK_GRAPH_READBACK_RECEIPT_REPORT="$ROOT/scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-retention-expiry-readback-receipt-preview-report.sh"
WORKFLOW_DURABLE_ADAPTER_REPORT="$ROOT/scripts/hepta-systems-workflow-durable-store-adapter-report.sh"
WORKFLOW_DURABLE_TEST_ONLY_APPEND_FIXTURE_REPORT="$ROOT/scripts/hepta-systems-workflow-durable-store-test-only-append-fixture-report.sh"
WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_TEST_IMPLEMENTATION_REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-append-only-event-store-test-implementation-report.sh"
WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_LOCAL_PERSISTENCE_READBACK_REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-lease-idempotency-index-local-persistence-readback-report.sh"
WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_FEATURE_GATED_READBACK_REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-deterministic-replay-validator-feature-gated-readback-report.sh"
WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_FEATURE_GATED_READBACK_REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-checkpoint-and-rollback-anchor-feature-gated-readback-report.sh"
WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_FEATURE_GATED_READBACK_REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-lease-idempotency-index-feature-gated-readback-report.sh"
WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_FEATURE_GATED_READBACK_REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-feature-gated-readback-report.sh"
WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_FEATURE_GATED_READBACK_REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-work-graph-projection-feature-gated-readback-report.sh"
HEPTA_SYSTEM_STATUS_E2E_REPORT="$ROOT/scripts/hepta-systems-hepta-system-status-read-only-e2e-report.sh"
HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_INVOCATION_REPORT="$ROOT/scripts/hepta-systems-hepta-system-status-internal-read-only-invocation-report.sh"
HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_PROTOCOL_REPORT="$ROOT/scripts/hepta-systems-hepta-system-status-operator-approval-protocol-report.sh"
CONTROLLED_CANARY_READINESS_PLAN_REPORT="$ROOT/scripts/hepta-systems-controlled-canary-readiness-plan-report.sh"
DIRTY_WORKTREE_RELEASE_BOUNDARY_INVENTORY_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-inventory-report.sh"
DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_PLAN_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-grouping-freeze-plan-report.sh"
DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_OPERATOR_READBACK_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-grouping-freeze-operator-readback-report.sh"
DIRTY_WORKTREE_RELEASE_BOUNDARY_ACTIONABLE_CLEAN_WORKTREE_STRATEGY_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-actionable-clean-worktree-strategy-report.sh"
DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-report.sh"
DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_NON_SEND_READBACK_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-non-send-readback-report.sh"
DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-git-mutation-boundary-readback-report.sh"
DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_CHECKLIST_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-report.sh"
DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_CHECKLIST_PACKET_READBACK_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-packet-readback-report.sh"
DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_RECORDING_BOUNDARY_READBACK_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-recording-boundary-readback-report.sh"
DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_APPROVAL_ACCEPTANCE_BOUNDARY_READBACK_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-approval-acceptance-boundary-readback-report.sh"
DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_EVIDENCE_RECORDING_BOUNDARY_READBACK_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-evidence-recording-boundary-readback-report.sh"
DIRTY_WORKTREE_RELEASE_BOUNDARY_RELEASE_RISK_SNAPSHOT_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-release-risk-snapshot-report.sh"
DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_CLEAN_WORKTREE_STRATEGY_REHEARSAL_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-test-only-clean-worktree-strategy-rehearsal-report.sh"
DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_REHEARSAL_OUTCOME_READBACK_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-test-only-rehearsal-outcome-readback-report.sh"
DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_REHEARSAL_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-rehearsal-report.sh"
CONTROLLED_LIVE_READINESS_REPORT="$ROOT/scripts/hepta-systems-controlled-live-readiness-audit-report.sh"
CONTROLLED_LIVE_DENIAL_READBACK_REPORT="$ROOT/scripts/hepta-systems-controlled-live-readiness-denial-readback-index-report.sh"
CONTROLLED_LIVE_OPERATOR_PACKET_REPORT="$ROOT/scripts/hepta-systems-controlled-live-operator-packet-preview-report.sh"
CONTROLLED_LIVE_OPERATOR_PACKET_NONSEND_REPORT="$ROOT/scripts/hepta-systems-controlled-live-operator-packet-non-send-readback-report.sh"
CONTROLLED_LIVE_REQUIRED_EVIDENCE_PLAN_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-collection-plan-report.sh"
CONTROLLED_LIVE_REQUIRED_EVIDENCE_READBACK_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-readback-index-report.sh"
CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_SUMMARY_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-summary-report.sh"
CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_DIFF_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-diff-view-report.sh"
CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_READBACK_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-operator-readback-report.sh"
CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-report.sh"
CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_NONSEND_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-non-send-readback-report.sh"
CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_TRANSPORT_BOUNDARY_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-transport-boundary-readback-report.sh"
CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_CREDENTIAL_BOUNDARY_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-credential-boundary-readback-report.sh"
CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_ROLLBACK_REHEARSAL_BOUNDARY_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-rollback-rehearsal-boundary-readback-report.sh"
CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_KILL_SWITCH_REHEARSAL_BOUNDARY_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-kill-switch-rehearsal-boundary-readback-report.sh"
CURRENT_SUMMARY_REPORT="$ROOT/scripts/hepta-systems-current-compact-capability-summary-report.sh"
CURRENT_REALITY_MATRIX_COMPACT_CACHE_BOUNDARY_READBACK_REPORT="$ROOT/scripts/hepta-systems-current-reality-matrix-compact-cache-boundary-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CURRENT_REALITY_CAPABILITY_MATRIX_2026-06-27.md"

PLUGIN_MANIFEST="$ROOT/plugins/hepta-system/.codex-plugin/plugin.json"
PLUGIN_LIFECYCLE_STATE_MACHINE="$ROOT/codex-rs/core-plugins/src/lifecycle_state_machine.rs"
PLUGIN_LIFECYCLE_PHASE_SUMMARY="$ROOT/codex-rs/core-plugins/src/lifecycle_phase_summary.rs"
WORKFLOW_DURABLE_STORE_ADAPTER="$ROOT/codex-rs/hepta-runtime/src/workflow_durable_store_adapter.rs"
WORKFLOW_DURABLE_STORE_APPEND_PLAN="$ROOT/codex-rs/hepta-runtime/src/workflow_durable_store_append_plan.rs"
WORKFLOW_DURABLE_STORE_ADAPTER_HARNESS="$ROOT/codex-rs/hepta-runtime/src/workflow_durable_store_adapter_harness.rs"

fail() {
  printf 'hepta-systems-current-reality-capability-matrix-report: FAIL: %s\n' "$1" >&2
  exit 1
}

path_exists() { [[ -e "$1" ]]; }
bool_for() {
  if "$@"; then printf 'true\n'; else printf 'false\n'; fi
}

[[ -x "$PLUGIN_ABI_REPORT" ]] || fail "missing executable plugin contribution ABI report: $PLUGIN_ABI_REPORT"
[[ -x "$PLUGIN_LOADER_REPORT" ]] || fail "missing executable plugin contribution loader binding report: $PLUGIN_LOADER_REPORT"
[[ -x "$PLUGIN_TOOL_INVENTORY_REPORT" ]] || fail "missing executable plugin tool inventory report: $PLUGIN_TOOL_INVENTORY_REPORT"
[[ -x "$PLUGIN_LIFECYCLE_REPORT" ]] || fail "missing executable plugin lifecycle state-machine report: $PLUGIN_LIFECYCLE_REPORT"
[[ -x "$TOOL_SOURCE_REPORT" ]] || fail "missing executable tool registry source-of-truth report: $TOOL_SOURCE_REPORT"
[[ -x "$TOOL_READ_ONLY_DISPATCH_REPORT" ]] || fail "missing executable tool registry read-only dispatch preflight report: $TOOL_READ_ONLY_DISPATCH_REPORT"
[[ -x "$WORK_GRAPH_DURABLE_REPORT" ]] || fail "missing executable WorkGraph durable identity report: $WORK_GRAPH_DURABLE_REPORT"
[[ -x "$WORK_GRAPH_READBACK_RECEIPT_REPORT" ]] || fail "missing executable WorkGraph readback receipt report: $WORK_GRAPH_READBACK_RECEIPT_REPORT"
[[ -x "$WORKFLOW_DURABLE_ADAPTER_REPORT" ]] || fail "missing executable workflow durable adapter report: $WORKFLOW_DURABLE_ADAPTER_REPORT"
[[ -x "$WORKFLOW_DURABLE_TEST_ONLY_APPEND_FIXTURE_REPORT" ]] || fail "missing executable workflow durable test-only append fixture report: $WORKFLOW_DURABLE_TEST_ONLY_APPEND_FIXTURE_REPORT"
[[ -x "$WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_TEST_IMPLEMENTATION_REPORT" ]] || fail "missing executable Temporal-lite append-only event store test implementation report: $WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_TEST_IMPLEMENTATION_REPORT"
[[ -x "$WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_LOCAL_PERSISTENCE_READBACK_REPORT" ]] || fail "missing executable Temporal-lite local lease/idempotency report: $WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_LOCAL_PERSISTENCE_READBACK_REPORT"
[[ -x "$WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_FEATURE_GATED_READBACK_REPORT" ]] || fail "missing executable Temporal-lite deterministic replay validator report: $WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_FEATURE_GATED_READBACK_REPORT"
[[ -x "$WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_FEATURE_GATED_READBACK_REPORT" ]] || fail "missing executable Temporal-lite checkpoint and rollback anchor report: $WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_FEATURE_GATED_READBACK_REPORT"
[[ -x "$WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_FEATURE_GATED_READBACK_REPORT" ]] || fail "missing executable Temporal-lite lease/idempotency report: $WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_FEATURE_GATED_READBACK_REPORT"
[[ -x "$WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_FEATURE_GATED_READBACK_REPORT" ]] || fail "missing executable Temporal-lite event-log/SQLite adapter report: $WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_FEATURE_GATED_READBACK_REPORT"
[[ -x "$WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_FEATURE_GATED_READBACK_REPORT" ]] || fail "missing executable Temporal-lite WorkGraph projection report: $WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_FEATURE_GATED_READBACK_REPORT"
[[ -x "$HEPTA_SYSTEM_STATUS_E2E_REPORT" ]] || fail "missing executable hepta-system status read-only E2E report: $HEPTA_SYSTEM_STATUS_E2E_REPORT"
[[ -x "$HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_INVOCATION_REPORT" ]] || fail "missing executable hepta-system status internal read-only invocation report: $HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_INVOCATION_REPORT"
[[ -x "$HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_PROTOCOL_REPORT" ]] || fail "missing executable hepta-system status operator approval protocol report: $HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_PROTOCOL_REPORT"
[[ -x "$CONTROLLED_CANARY_READINESS_PLAN_REPORT" ]] || fail "missing executable controlled canary readiness plan report: $CONTROLLED_CANARY_READINESS_PLAN_REPORT"
[[ -x "$DIRTY_WORKTREE_RELEASE_BOUNDARY_INVENTORY_REPORT" ]] || fail "missing executable dirty-worktree release-boundary inventory report: $DIRTY_WORKTREE_RELEASE_BOUNDARY_INVENTORY_REPORT"
[[ -x "$DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_PLAN_REPORT" ]] || fail "missing executable dirty-worktree release-boundary grouping freeze-plan report: $DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_PLAN_REPORT"
[[ -x "$DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_OPERATOR_READBACK_REPORT" ]] || fail "missing executable dirty-worktree release-boundary grouping freeze operator readback report: $DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_OPERATOR_READBACK_REPORT"
[[ -x "$DIRTY_WORKTREE_RELEASE_BOUNDARY_ACTIONABLE_CLEAN_WORKTREE_STRATEGY_REPORT" ]] || fail "missing executable dirty-worktree release-boundary actionable clean-worktree strategy report: $DIRTY_WORKTREE_RELEASE_BOUNDARY_ACTIONABLE_CLEAN_WORKTREE_STRATEGY_REPORT"
[[ -x "$DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_REPORT" ]] || fail "missing executable dirty-worktree release-boundary clean-worktree strategy operator packet report: $DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_REPORT"
[[ -x "$DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_NON_SEND_READBACK_REPORT" ]] || fail "missing executable dirty-worktree release-boundary clean-worktree strategy operator packet non-send readback report: $DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_NON_SEND_READBACK_REPORT"
[[ -x "$DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_REPORT" ]] || fail "missing executable dirty-worktree release-boundary clean-worktree strategy operator packet git-mutation boundary readback report: $DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_REPORT"
[[ -x "$DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_CHECKLIST_REPORT" ]] || fail "missing executable dirty-worktree release-boundary clean-worktree strategy operator decision checklist report: $DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_CHECKLIST_REPORT"
[[ -x "$DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_CHECKLIST_PACKET_READBACK_REPORT" ]] || fail "missing executable dirty-worktree release-boundary clean-worktree strategy operator decision checklist packet readback report: $DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_CHECKLIST_PACKET_READBACK_REPORT"
[[ -x "$DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_RECORDING_BOUNDARY_READBACK_REPORT" ]] || fail "missing executable dirty-worktree release-boundary clean-worktree strategy operator decision recording boundary readback report: $DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_RECORDING_BOUNDARY_READBACK_REPORT"
[[ -x "$DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_APPROVAL_ACCEPTANCE_BOUNDARY_READBACK_REPORT" ]] || fail "missing executable dirty-worktree release-boundary clean-worktree strategy operator approval acceptance boundary readback report: $DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_APPROVAL_ACCEPTANCE_BOUNDARY_READBACK_REPORT"
[[ -x "$DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_EVIDENCE_RECORDING_BOUNDARY_READBACK_REPORT" ]] || fail "missing executable dirty-worktree release-boundary clean-worktree strategy operator evidence recording boundary readback report: $DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_EVIDENCE_RECORDING_BOUNDARY_READBACK_REPORT"
[[ -x "$DIRTY_WORKTREE_RELEASE_BOUNDARY_RELEASE_RISK_SNAPSHOT_REPORT" ]] || fail "missing executable dirty-worktree release-boundary release risk snapshot report: $DIRTY_WORKTREE_RELEASE_BOUNDARY_RELEASE_RISK_SNAPSHOT_REPORT"
[[ -x "$DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_CLEAN_WORKTREE_STRATEGY_REHEARSAL_REPORT" ]] || fail "missing executable dirty-worktree release-boundary test-only clean-worktree strategy rehearsal report: $DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_CLEAN_WORKTREE_STRATEGY_REHEARSAL_REPORT"
[[ -x "$DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_REHEARSAL_OUTCOME_READBACK_REPORT" ]] || fail "missing executable dirty-worktree release-boundary test-only rehearsal outcome readback report: $DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_REHEARSAL_OUTCOME_READBACK_REPORT"
[[ -x "$DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_REHEARSAL_REPORT" ]] || fail "missing executable dirty-worktree release-boundary owner/freeze/classification rehearsal report: $DIRTY_WORKTREE_RELEASE_BOUNDARY_OWNER_FREEZE_CLASSIFICATION_REHEARSAL_REPORT"
[[ -x "$CONTROLLED_LIVE_READINESS_REPORT" ]] || fail "missing executable controlled-live readiness audit report: $CONTROLLED_LIVE_READINESS_REPORT"
[[ -x "$CONTROLLED_LIVE_DENIAL_READBACK_REPORT" ]] || fail "missing executable controlled-live denial readback index report: $CONTROLLED_LIVE_DENIAL_READBACK_REPORT"
[[ -x "$CONTROLLED_LIVE_OPERATOR_PACKET_REPORT" ]] || fail "missing executable controlled-live operator packet preview report: $CONTROLLED_LIVE_OPERATOR_PACKET_REPORT"
[[ -x "$CONTROLLED_LIVE_OPERATOR_PACKET_NONSEND_REPORT" ]] || fail "missing executable controlled-live operator packet non-send readback report: $CONTROLLED_LIVE_OPERATOR_PACKET_NONSEND_REPORT"
[[ -x "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_PLAN_REPORT" ]] || fail "missing executable controlled-live required evidence collection plan report: $CONTROLLED_LIVE_REQUIRED_EVIDENCE_PLAN_REPORT"
[[ -x "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_READBACK_REPORT" ]] || fail "missing executable controlled-live required evidence readback index report: $CONTROLLED_LIVE_REQUIRED_EVIDENCE_READBACK_REPORT"
[[ -x "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_SUMMARY_REPORT" ]] || fail "missing executable controlled-live required evidence gap summary report: $CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_SUMMARY_REPORT"
[[ -x "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_DIFF_REPORT" ]] || fail "missing executable controlled-live required evidence gap diff view report: $CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_DIFF_REPORT"
[[ -x "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_READBACK_REPORT" ]] || fail "missing executable controlled-live required evidence gap operator readback report: $CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_READBACK_REPORT"
[[ -x "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_REPORT" ]] || fail "missing executable controlled-live required evidence gap operator packet attachment report: $CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_REPORT"
[[ -x "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_NONSEND_REPORT" ]] || fail "missing executable controlled-live required evidence gap operator packet attachment non-send readback report: $CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_NONSEND_REPORT"
[[ -x "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_TRANSPORT_BOUNDARY_REPORT" ]] || fail "missing executable controlled-live required evidence gap operator packet attachment transport boundary readback report: $CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_TRANSPORT_BOUNDARY_REPORT"
[[ -x "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_CREDENTIAL_BOUNDARY_REPORT" ]] || fail "missing executable controlled-live required evidence gap operator packet attachment credential boundary readback report: $CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_CREDENTIAL_BOUNDARY_REPORT"
[[ -x "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_ROLLBACK_REHEARSAL_BOUNDARY_REPORT" ]] || fail "missing executable controlled-live required evidence gap operator packet attachment rollback rehearsal boundary readback report: $CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_ROLLBACK_REHEARSAL_BOUNDARY_REPORT"
[[ -x "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_KILL_SWITCH_REHEARSAL_BOUNDARY_REPORT" ]] || fail "missing executable controlled-live required evidence gap operator packet attachment kill-switch rehearsal boundary readback report: $CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_KILL_SWITCH_REHEARSAL_BOUNDARY_REPORT"
[[ -x "$CURRENT_SUMMARY_REPORT" ]] || fail "missing executable current compact capability summary report: $CURRENT_SUMMARY_REPORT"
[[ -x "$CURRENT_REALITY_MATRIX_COMPACT_CACHE_BOUNDARY_READBACK_REPORT" ]] || fail "missing executable current reality matrix compact cache boundary readback report: $CURRENT_REALITY_MATRIX_COMPACT_CACHE_BOUNDARY_READBACK_REPORT"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the current reality capability matrix report"
fi

plugin_manifest_present="$(bool_for path_exists "$PLUGIN_MANIFEST")"
plugin_lifecycle_state_machine_present="$(bool_for path_exists "$PLUGIN_LIFECYCLE_STATE_MACHINE")"
plugin_lifecycle_phase_summary_present="$(bool_for path_exists "$PLUGIN_LIFECYCLE_PHASE_SUMMARY")"
workflow_durable_store_adapter_present="$(bool_for path_exists "$WORKFLOW_DURABLE_STORE_ADAPTER")"
workflow_durable_store_append_plan_present="$(bool_for path_exists "$WORKFLOW_DURABLE_STORE_APPEND_PLAN")"
workflow_durable_store_adapter_harness_present="$(bool_for path_exists "$WORKFLOW_DURABLE_STORE_ADAPTER_HARNESS")"

if [[ "$plugin_manifest_present" == "true" ]]; then
  manifest_dir="$(dirname "$PLUGIN_MANIFEST")"
  plugin_root="$(cd "$manifest_dir/.." && pwd -P)"
  skills_path="$(jq -r 'if (.skills | type) == "string" then .skills else "" end' "$PLUGIN_MANIFEST")"
  mcp_servers_path="$(jq -r 'if (.mcpServers | type) == "string" then .mcpServers else "" end' "$PLUGIN_MANIFEST")"
  apps_path="$(jq -r 'if (.apps | type) == "string" then .apps else "" end' "$PLUGIN_MANIFEST")"
  skill_path_present=false
  mcp_servers_path_present=false
  apps_path_present=false
  skill_count=0
  mcp_server_count=0
  app_count=0

  if [[ -n "$skills_path" && -d "$plugin_root/$skills_path" ]]; then
    skill_path_present=true
    skill_count="$(find "$plugin_root/$skills_path" -name SKILL.md -type f | wc -l | tr -d ' ')"
  fi
  if [[ -n "$mcp_servers_path" && -f "$plugin_root/$mcp_servers_path" ]]; then
    mcp_servers_path_present=true
    mcp_server_count="$(jq '(.mcpServers // {}) | length' "$plugin_root/$mcp_servers_path")"
  fi
  if [[ -n "$apps_path" && -f "$plugin_root/$apps_path" ]]; then
    apps_path_present=true
    app_count="$(jq '(.apps // {}) | length' "$plugin_root/$apps_path")"
  fi

  plugin_manifest_summary="$(
    jq -n -c \
      --slurpfile manifest "$PLUGIN_MANIFEST" \
      --argjson skill_path_present "$skill_path_present" \
      --argjson mcp_servers_path_present "$mcp_servers_path_present" \
      --argjson apps_path_present "$apps_path_present" \
      --argjson skill_count "$skill_count" \
      --argjson mcp_server_count "$mcp_server_count" \
      --argjson app_count "$app_count" \
      '($manifest[0]) as $manifest | {
      name:($manifest.name // null),
      version:($manifest.version // null),
      skill_path_present:$skill_path_present,
      mcp_servers_path_present:$mcp_servers_path_present,
      apps_path_present:$apps_path_present,
      skill_count:$skill_count,
      mcp_server_count:$mcp_server_count,
      app_count:$app_count,
      tool_schema_count:(($manifest.toolSchemas // {}) | length),
      permission_count:(($manifest.permissions // {}) | length),
      activation_event_count:(($manifest.activationEvents // {}) | length),
      tool_policy_count:(($manifest.toolPolicies // {}) | length)
    }'
  )"
else
  plugin_manifest_summary='{}'
fi

git_status_entry_count="$(git -C "$ROOT" status --porcelain | wc -l | tr -d ' ')"
git_untracked_count="$(git -C "$ROOT" status --porcelain | awk 'substr($0,1,2) == "??" {count++} END {print count + 0}')"
git_tracked_change_count="$(git -C "$ROOT" status --porcelain | awk 'substr($0,1,2) != "??" {count++} END {print count + 0}')"

tmpdir="$(mktemp -d)"
cleanup() {
  if [[ ${render_pids+x} == x ]]; then
    local pid
    for pid in "${render_pids[@]}"; do
      kill "$pid" >/dev/null 2>&1 || true
    done
  fi
  rm -rf "$tmpdir"
}
trap cleanup EXIT

render_report_jobs="${HEPTA_MATRIX_REPORT_RENDER_JOBS:-1}"
case "$render_report_jobs" in
  ''|*[!0-9]*)
    fail "HEPTA_MATRIX_REPORT_RENDER_JOBS must be a positive integer"
    ;;
esac
if (( render_report_jobs < 1 )); then
  fail "HEPTA_MATRIX_REPORT_RENDER_JOBS must be greater than zero"
fi

render_pids=()
render_names=()

wait_for_oldest_render() {
  local pid="${render_pids[0]}"
  local name="${render_names[0]}"
  if ! wait "$pid"; then
    fail "failed to render source report: $name"
  fi
  render_pids=("${render_pids[@]:1}")
  render_names=("${render_names[@]:1}")
}

wait_for_render_reports() {
  while (( ${#render_pids[@]} > 0 )); do
    wait_for_oldest_render
  done
}

render_report() {
  local name="$1"
  local report="$2"
  "$report" >"$tmpdir/$name.json" &
  render_pids+=("$!")
  render_names+=("$name")
  if (( ${#render_pids[@]} >= render_report_jobs )); then
    wait_for_oldest_render
  fi
}

render_report plugin_abi "$PLUGIN_ABI_REPORT"
render_report plugin_loader "$PLUGIN_LOADER_REPORT"
render_report plugin_tool_inventory "$PLUGIN_TOOL_INVENTORY_REPORT"
render_report plugin_lifecycle "$PLUGIN_LIFECYCLE_REPORT"
render_report tool_source "$TOOL_SOURCE_REPORT"
render_report tool_read_only_dispatch "$TOOL_READ_ONLY_DISPATCH_REPORT"
render_report work_graph_durable "$WORK_GRAPH_DURABLE_REPORT"
render_report work_graph_readback_receipt "$WORK_GRAPH_READBACK_RECEIPT_REPORT"
render_report workflow_durable_adapter "$WORKFLOW_DURABLE_ADAPTER_REPORT"
render_report hepta_status_e2e "$HEPTA_SYSTEM_STATUS_E2E_REPORT"
render_report hepta_status_internal_read_only_invocation "$HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_INVOCATION_REPORT"
render_report hepta_status_operator_approval_protocol "$HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_PROTOCOL_REPORT"
wait_for_render_reports

render_report controlled_live_readiness "$CONTROLLED_LIVE_READINESS_REPORT"
wait_for_render_reports
export HEPTA_CONTROLLED_LIVE_AUDIT_JSON="$tmpdir/controlled_live_readiness.json"
render_report controlled_live_denial_readback "$CONTROLLED_LIVE_DENIAL_READBACK_REPORT"
wait_for_render_reports
export HEPTA_CONTROLLED_LIVE_DENIAL_READBACK_JSON="$tmpdir/controlled_live_denial_readback.json"
render_report controlled_live_operator_packet "$CONTROLLED_LIVE_OPERATOR_PACKET_REPORT"
wait_for_render_reports
export HEPTA_CONTROLLED_LIVE_OPERATOR_PACKET_JSON="$tmpdir/controlled_live_operator_packet.json"
render_report controlled_live_operator_packet_nonsend "$CONTROLLED_LIVE_OPERATOR_PACKET_NONSEND_REPORT"
wait_for_render_reports
export HEPTA_CONTROLLED_LIVE_OPERATOR_PACKET_NONSEND_JSON="$tmpdir/controlled_live_operator_packet_nonsend.json"
render_report controlled_live_required_evidence_plan "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_PLAN_REPORT"
wait_for_render_reports
export HEPTA_CONTROLLED_LIVE_REQUIRED_EVIDENCE_PLAN_JSON="$tmpdir/controlled_live_required_evidence_plan.json"
render_report controlled_live_required_evidence_readback "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_READBACK_REPORT"
wait_for_render_reports
export HEPTA_CONTROLLED_LIVE_REQUIRED_EVIDENCE_READBACK_JSON="$tmpdir/controlled_live_required_evidence_readback.json"
render_report controlled_live_required_evidence_gap_summary "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_SUMMARY_REPORT"
wait_for_render_reports
export HEPTA_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_SUMMARY_JSON="$tmpdir/controlled_live_required_evidence_gap_summary.json"
render_report controlled_live_required_evidence_gap_diff "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_DIFF_REPORT"
wait_for_render_reports
export HEPTA_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_DIFF_JSON="$tmpdir/controlled_live_required_evidence_gap_diff.json"
render_report controlled_live_required_evidence_gap_operator_readback "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_READBACK_REPORT"
wait_for_render_reports
export HEPTA_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_READBACK_JSON="$tmpdir/controlled_live_required_evidence_gap_operator_readback.json"
render_report controlled_live_required_evidence_gap_operator_packet_attachment "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_REPORT"
wait_for_render_reports
export HEPTA_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_JSON="$tmpdir/controlled_live_required_evidence_gap_operator_packet_attachment.json"
render_report controlled_live_required_evidence_gap_operator_packet_attachment_nonsend "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_NONSEND_REPORT"
wait_for_render_reports
export HEPTA_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_NONSEND_JSON="$tmpdir/controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.json"
render_report controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_TRANSPORT_BOUNDARY_REPORT"
wait_for_render_reports
export HEPTA_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_TRANSPORT_JSON="$tmpdir/controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.json"
render_report controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_CREDENTIAL_BOUNDARY_REPORT"
wait_for_render_reports
export HEPTA_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_CREDENTIAL_JSON="$tmpdir/controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.json"
render_report controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_ROLLBACK_REHEARSAL_BOUNDARY_REPORT"
wait_for_render_reports
export HEPTA_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_ROLLBACK_JSON="$tmpdir/controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.json"
render_report controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary "$CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_KILL_SWITCH_REHEARSAL_BOUNDARY_REPORT"
wait_for_render_reports

export HEPTA_CONTROLLED_CANARY_APPROVAL_JSON="$tmpdir/hepta_status_operator_approval_protocol.json"
export HEPTA_CONTROLLED_CANARY_BOUNDARY_JSON="$tmpdir/controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.json"
render_report controlled_canary_readiness_plan "$CONTROLLED_CANARY_READINESS_PLAN_REPORT"
wait_for_render_reports
export HEPTA_DIRTY_WORKTREE_RELEASE_BOUNDARY_CANARY_JSON="$tmpdir/controlled_canary_readiness_plan.json"
render_report dirty_worktree_release_boundary_inventory "$DIRTY_WORKTREE_RELEASE_BOUNDARY_INVENTORY_REPORT"
render_report dirty_worktree_release_boundary_grouping_freeze_plan "$DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_PLAN_REPORT"
render_report dirty_worktree_release_boundary_grouping_freeze_operator_readback "$DIRTY_WORKTREE_RELEASE_BOUNDARY_GROUPING_FREEZE_OPERATOR_READBACK_REPORT"
render_report dirty_worktree_release_boundary_actionable_clean_worktree_strategy "$DIRTY_WORKTREE_RELEASE_BOUNDARY_ACTIONABLE_CLEAN_WORKTREE_STRATEGY_REPORT"
render_report dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet "$DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_REPORT"
render_report dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback "$DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_NON_SEND_READBACK_REPORT"
render_report dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback "$DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_PACKET_GIT_MUTATION_BOUNDARY_READBACK_REPORT"
render_report dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist "$DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_CHECKLIST_REPORT"
render_report dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback "$DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_CHECKLIST_PACKET_READBACK_REPORT"
render_report dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback "$DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_DECISION_RECORDING_BOUNDARY_READBACK_REPORT"
render_report dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback "$DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_APPROVAL_ACCEPTANCE_BOUNDARY_READBACK_REPORT"
render_report dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback "$DIRTY_WORKTREE_RELEASE_BOUNDARY_CLEAN_WORKTREE_STRATEGY_OPERATOR_EVIDENCE_RECORDING_BOUNDARY_READBACK_REPORT"
render_report dirty_worktree_release_boundary_release_risk_snapshot "$DIRTY_WORKTREE_RELEASE_BOUNDARY_RELEASE_RISK_SNAPSHOT_REPORT"
render_report dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal "$DIRTY_WORKTREE_RELEASE_BOUNDARY_TEST_ONLY_CLEAN_WORKTREE_STRATEGY_REHEARSAL_REPORT"
export HEPTA_CURRENT_COMPACT_CAPABILITY_SUMMARY_FAST_READBACK=true
render_report current_summary "$CURRENT_SUMMARY_REPORT"
wait_for_render_reports

jq -n \
  --slurpfile plugin_abi "$tmpdir/plugin_abi.json" \
  --slurpfile plugin_loader "$tmpdir/plugin_loader.json" \
  --slurpfile plugin_tool_inventory "$tmpdir/plugin_tool_inventory.json" \
  --slurpfile plugin_lifecycle "$tmpdir/plugin_lifecycle.json" \
  --slurpfile tool_source "$tmpdir/tool_source.json" \
  --slurpfile tool_read_only_dispatch "$tmpdir/tool_read_only_dispatch.json" \
  --slurpfile work_graph_durable "$tmpdir/work_graph_durable.json" \
  --slurpfile work_graph_readback_receipt "$tmpdir/work_graph_readback_receipt.json" \
  --slurpfile workflow_durable_adapter "$tmpdir/workflow_durable_adapter.json" \
  --slurpfile hepta_status_e2e "$tmpdir/hepta_status_e2e.json" \
  --slurpfile hepta_status_internal_read_only_invocation "$tmpdir/hepta_status_internal_read_only_invocation.json" \
  --slurpfile hepta_status_operator_approval_protocol "$tmpdir/hepta_status_operator_approval_protocol.json" \
  --slurpfile controlled_canary_readiness_plan "$tmpdir/controlled_canary_readiness_plan.json" \
  --slurpfile dirty_worktree_release_boundary_inventory "$tmpdir/dirty_worktree_release_boundary_inventory.json" \
  --slurpfile dirty_worktree_release_boundary_grouping_freeze_plan "$tmpdir/dirty_worktree_release_boundary_grouping_freeze_plan.json" \
  --slurpfile dirty_worktree_release_boundary_grouping_freeze_operator_readback "$tmpdir/dirty_worktree_release_boundary_grouping_freeze_operator_readback.json" \
  --slurpfile dirty_worktree_release_boundary_actionable_clean_worktree_strategy "$tmpdir/dirty_worktree_release_boundary_actionable_clean_worktree_strategy.json" \
  --slurpfile dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet "$tmpdir/dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.json" \
  --slurpfile dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback "$tmpdir/dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.json" \
  --slurpfile dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback "$tmpdir/dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.json" \
  --slurpfile dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist "$tmpdir/dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.json" \
  --slurpfile dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback "$tmpdir/dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.json" \
  --slurpfile dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback "$tmpdir/dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.json" \
  --slurpfile dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback "$tmpdir/dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.json" \
  --slurpfile dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback "$tmpdir/dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.json" \
  --slurpfile dirty_worktree_release_boundary_release_risk_snapshot "$tmpdir/dirty_worktree_release_boundary_release_risk_snapshot.json" \
  --slurpfile dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal "$tmpdir/dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.json" \
  --slurpfile controlled_live_readiness "$tmpdir/controlled_live_readiness.json" \
  --slurpfile controlled_live_denial_readback "$tmpdir/controlled_live_denial_readback.json" \
  --slurpfile controlled_live_operator_packet "$tmpdir/controlled_live_operator_packet.json" \
  --slurpfile controlled_live_operator_packet_nonsend "$tmpdir/controlled_live_operator_packet_nonsend.json" \
  --slurpfile controlled_live_required_evidence_plan "$tmpdir/controlled_live_required_evidence_plan.json" \
  --slurpfile controlled_live_required_evidence_readback "$tmpdir/controlled_live_required_evidence_readback.json" \
  --slurpfile controlled_live_required_evidence_gap_summary "$tmpdir/controlled_live_required_evidence_gap_summary.json" \
  --slurpfile controlled_live_required_evidence_gap_diff "$tmpdir/controlled_live_required_evidence_gap_diff.json" \
  --slurpfile controlled_live_required_evidence_gap_operator_readback "$tmpdir/controlled_live_required_evidence_gap_operator_readback.json" \
  --slurpfile controlled_live_required_evidence_gap_operator_packet_attachment "$tmpdir/controlled_live_required_evidence_gap_operator_packet_attachment.json" \
  --slurpfile controlled_live_required_evidence_gap_operator_packet_attachment_nonsend "$tmpdir/controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.json" \
  --slurpfile controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary "$tmpdir/controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.json" \
  --slurpfile controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary "$tmpdir/controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.json" \
  --slurpfile controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary "$tmpdir/controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.json" \
  --slurpfile controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary "$tmpdir/controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.json" \
  --slurpfile current_summary "$tmpdir/current_summary.json" \
  --argjson plugin_manifest_present "$plugin_manifest_present" \
  --argjson plugin_manifest_summary "$plugin_manifest_summary" \
  --argjson plugin_lifecycle_state_machine_present "$plugin_lifecycle_state_machine_present" \
  --argjson plugin_lifecycle_phase_summary_present "$plugin_lifecycle_phase_summary_present" \
  --argjson workflow_durable_store_adapter_present "$workflow_durable_store_adapter_present" \
  --argjson workflow_durable_store_append_plan_present "$workflow_durable_store_append_plan_present" \
  --argjson workflow_durable_store_adapter_harness_present "$workflow_durable_store_adapter_harness_present" \
  --argjson git_status_entry_count "$git_status_entry_count" \
  --argjson git_untracked_count "$git_untracked_count" \
  --argjson git_tracked_change_count "$git_tracked_change_count" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CURRENT_REALITY_CAPABILITY_MATRIX_2026-06-27.md" \
  '
  ($plugin_abi[0]) as $plugin_abi |
  ($plugin_loader[0]) as $plugin_loader |
  ($plugin_tool_inventory[0]) as $plugin_tool_inventory |
  ($plugin_lifecycle[0]) as $plugin_lifecycle |
  ($tool_source[0]) as $tool_source |
  ($tool_read_only_dispatch[0]) as $tool_read_only_dispatch |
  ($work_graph_durable[0]) as $work_graph_durable |
  ($work_graph_readback_receipt[0]) as $work_graph_readback_receipt |
  ($workflow_durable_adapter[0]) as $workflow_durable_adapter |
  ($hepta_status_e2e[0]) as $hepta_status_e2e |
  ($hepta_status_internal_read_only_invocation[0]) as $hepta_status_internal_read_only_invocation |
  ($hepta_status_operator_approval_protocol[0]) as $hepta_status_operator_approval_protocol |
  ($controlled_canary_readiness_plan[0]) as $controlled_canary_readiness_plan |
  ($dirty_worktree_release_boundary_inventory[0]) as $dirty_worktree_release_boundary_inventory |
  ($dirty_worktree_release_boundary_grouping_freeze_plan[0]) as $dirty_worktree_release_boundary_grouping_freeze_plan |
  ($dirty_worktree_release_boundary_grouping_freeze_operator_readback[0]) as $dirty_worktree_release_boundary_grouping_freeze_operator_readback |
  ($dirty_worktree_release_boundary_actionable_clean_worktree_strategy[0]) as $dirty_worktree_release_boundary_actionable_clean_worktree_strategy |
  ($dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet[0]) as $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet |
  ($dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback[0]) as $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback |
  ($dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback[0]) as $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback |
  ($dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist[0]) as $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist |
  ($dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback[0]) as $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback |
  ($dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback[0]) as $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback |
  ($dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback[0]) as $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback |
  ($dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback[0]) as $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback |
  ($dirty_worktree_release_boundary_release_risk_snapshot[0]) as $dirty_worktree_release_boundary_release_risk_snapshot |
  ($dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal[0]) as $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal |
  ($controlled_live_readiness[0]) as $controlled_live_readiness |
  ($controlled_live_denial_readback[0]) as $controlled_live_denial_readback |
  ($controlled_live_operator_packet[0]) as $controlled_live_operator_packet |
  ($controlled_live_operator_packet_nonsend[0]) as $controlled_live_operator_packet_nonsend |
  ($controlled_live_required_evidence_plan[0]) as $controlled_live_required_evidence_plan |
  ($controlled_live_required_evidence_readback[0]) as $controlled_live_required_evidence_readback |
  ($controlled_live_required_evidence_gap_summary[0]) as $controlled_live_required_evidence_gap_summary |
  ($controlled_live_required_evidence_gap_diff[0]) as $controlled_live_required_evidence_gap_diff |
  ($controlled_live_required_evidence_gap_operator_readback[0]) as $controlled_live_required_evidence_gap_operator_readback |
  ($controlled_live_required_evidence_gap_operator_packet_attachment[0]) as $controlled_live_required_evidence_gap_operator_packet_attachment |
  ($controlled_live_required_evidence_gap_operator_packet_attachment_nonsend[0]) as $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend |
  ($controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary[0]) as $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary |
  ($controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary[0]) as $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary |
  ($controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary[0]) as $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary |
  ($controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary[0]) as $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary |
  ($current_summary[0]) as $current_summary |
  [
    {
      id:"plugins_contribution_point_abi",
      layer:"plugins",
      status:$plugin_abi.status,
      ready:($plugin_abi.abi_ready == true),
      live_enabled:($plugin_abi.live_mutation_ready == true or $plugin_abi.runtime_execution_enabled == true),
      source:"scripts/hepta-systems-plugin-contribution-point-abi-report.sh",
      current_fact:"8 contribution point kinds are policy-bound and runtime execution is disabled"
    },
    {
      id:"plugins_loader_binding_fixture",
      layer:"plugins",
      status:$plugin_loader.status,
      ready:($plugin_loader.binding_ready == true and $plugin_loader.hepta_system_manifest_present == true),
      live_enabled:($plugin_loader.live_mutation_ready == true or $plugin_loader.runtime_execution_enabled == true),
      source:"scripts/hepta-systems-plugin-contribution-point-loader-binding-report.sh",
      current_fact:"hepta-system manifest fixture is present and loader bindings are read-only"
    },
    {
      id:"plugins_tool_contribution_inventory",
      layer:"plugins_tools",
      status:$plugin_tool_inventory.status,
      ready:($plugin_tool_inventory.preview_ready == true and $plugin_tool_inventory.candidate_count == 2),
      live_enabled:($plugin_tool_inventory.live_mutation_ready == true or $plugin_tool_inventory.tool_invocation_enabled == true),
      source:"scripts/hepta-systems-plugin-tool-contribution-inventory-preview-report.sh",
      current_fact:"2 manifest candidates have schema, policy, ledger, and approval metadata"
    },
    {
      id:"plugins_lifecycle_state_machine",
      layer:"plugins",
      status:$plugin_lifecycle.status,
      ready:($plugin_lifecycle.lifecycle_state_machine_ready == true and $plugin_lifecycle.source_of_truth_ready == true),
      live_enabled:($plugin_lifecycle.live_mutation_ready == true or $plugin_lifecycle.tool_registry_registration_enabled == true or $plugin_lifecycle.tool_invocation_enabled == true or $plugin_lifecycle.ledger_written == true or $plugin_lifecycle.approval_requested == true or $plugin_lifecycle.plugin_cache_mutated == true),
      source:"scripts/hepta-systems-plugin-lifecycle-state-machine-report.sh",
      current_fact:"plugin lifecycle state-machine is restored and binds ABI, loader, fixture policy metadata, and tool preview contract"
    },
    {
      id:"tools_invocation_source_of_truth",
      layer:"tools",
      status:$tool_source.status,
      ready:($tool_source.invocation_source_of_truth_plan_ready == true and $tool_source.invocation_source_blocked_count == 0),
      live_enabled:($tool_source.registry_source_of_truth_enabled == true or $tool_source.tool_registration_enabled == true or $tool_source.tool_invocation_enabled == true or $tool_source.ledger_written == true or $tool_source.approval_requested == true),
      source:"scripts/hepta-systems-tool-registry-invocation-source-of-truth-report.sh",
      current_fact:"2 invocation sources are ready, but registration, invocation, ledger, and approval writes are disabled"
    },
    {
      id:"tools_read_only_dispatch_preflight",
      layer:"tools",
      status:$tool_read_only_dispatch.status,
      ready:($tool_read_only_dispatch.read_only_dispatch_preflight_ready == true and $tool_read_only_dispatch.source_plugin_lifecycle_ready == true),
      live_enabled:($tool_read_only_dispatch.live_mutation_ready == true or $tool_read_only_dispatch.registry_dispatch_switch_enabled == true or $tool_read_only_dispatch.registry_source_of_truth_enabled == true or $tool_read_only_dispatch.tool_registration_enabled == true or $tool_read_only_dispatch.tool_invocation_enabled == true or $tool_read_only_dispatch.ledger_written == true or $tool_read_only_dispatch.approval_requested == true or $tool_read_only_dispatch.result_receipt_written == true),
      source:"scripts/hepta-systems-tool-registry-read-only-dispatch-preflight-report.sh",
      current_fact:"plugin lifecycle-backed ToolRegistry dispatch preflight projects lookup, ledger, approval, and receipt without invocation"
    },
    {
      id:"workflow_workgraph_durable_identity",
      layer:"workflow",
      status:$work_graph_durable.status,
      ready:($work_graph_durable.durable_field_count == 7 and $work_graph_durable.preview_binding_count == 5 and $work_graph_durable.invariant_count == 7),
      live_enabled:($work_graph_durable.ready_for_durable_runtime == true or $work_graph_durable.ready_for_replay_execution == true or $work_graph_durable.ready_for_rollback_execution == true or $work_graph_durable.ready_for_live_execution == true),
      source:"scripts/hepta-systems-work-graph-durable-identity-preview-report.sh",
      current_fact:"durable identity fields are fixed while runtime, replay, rollback, and live execution are disabled"
    },
    {
      id:"workflow_current_readback_receipt_tail",
      layer:"workflow",
      status:$work_graph_readback_receipt.status,
      ready:($work_graph_readback_receipt.readback_receipt_count == 6 and $work_graph_readback_receipt.digest_check_count == 7 and $work_graph_readback_receipt.mismatch_denial_count == 8 and $work_graph_readback_receipt.ready_for_live_persistence == false),
      live_enabled:($work_graph_readback_receipt.ready_for_live_persistence == true or $work_graph_readback_receipt.ready_for_operator_acceptance == true),
      source:"scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-retention-expiry-readback-receipt-preview-report.sh",
      current_fact:"current WorkGraph tail is readback receipt preview; acknowledgement is next, but persistence remains disabled"
    },
    {
      id:"workflow_temporal_lite_durable_store_adapter",
      layer:"workflow",
      status:$workflow_durable_adapter.status,
      ready:($workflow_durable_adapter.temporal_lite_adapter_ready == true and $workflow_durable_adapter.append_plan_count == 9 and $workflow_durable_adapter.noop_receipt_count == 9),
      live_enabled:($workflow_durable_adapter.ready_for_event_log_write == true or $workflow_durable_adapter.ready_for_sqlite_write == true or $workflow_durable_adapter.ready_for_workflow_execution == true or $workflow_durable_adapter.ready_for_replay_execution == true or $workflow_durable_adapter.ready_for_rollback_execution == true or $workflow_durable_adapter.ready_for_live_execution == true or $workflow_durable_adapter.feature_gate_enabled == true),
      source:"scripts/hepta-systems-workflow-durable-store-adapter-report.sh",
      current_fact:"Temporal-lite adapter plan carries 9 append-only event contracts through lease, idempotency, checkpoint, replay, rollback, and no-op receipt metadata behind a disabled feature gate"
    },
    {
      id:"workflow_durable_store_test_only_append_fixture",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_append_only_event_store_test_implementation",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_append_only_event_store_minimal_local_persistence",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_lease_idempotency_index_local_persistence_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_work_graph_projection_local_persistence_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_replay_alignment_rollback_consistency_local_persistence_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_replay_alignment_recovery_window_local_persistence_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_lease_idempotency_index_feature_gated_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_work_graph_projection_feature_gated_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_replay_alignment_checkpoint_consistency_feature_gated_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_replay_alignment_rollback_consistency_feature_gated_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_replay_alignment_recovery_window_feature_gated_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"workflow_temporal_lite_replay_alignment_recovery_receipt_feature_gated_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_gate_recursion_cost_boundary_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_gate_recursion_lean_contract_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_workgraph_legacy_gate_recursion_inventory_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_signature_trust_install_cache_boundary_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_operator_evidence_acceptance_packet_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_install_cache_noop_preflight_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_tool_registry_registration_denial_receipt_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_tool_invocation_policy_approval_ledger_boundary_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_tool_invocation_dry_run_receipt_ledger_preview_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_packet_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_execution_open_preconditions_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_boundary_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_tool_registry_shadow_registration_lookup_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_systems_matrix_report_single_render_cache_boundary_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"current_reality_matrix_compact_cache_boundary_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_without_git_mutation",
      ready:true,
      live_enabled:false
    },
    {
      id:"dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_without_git_mutation",
      ready:true,
      live_enabled:false
    },
    {
      id:"dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send",
      ready:true,
      live_enabled:false
    },
    {
      id:"dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_without_git_mutation",
      ready:true,
      live_enabled:false
    },
    {
      id:"dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_without_git_mutation",
      ready:true,
      live_enabled:false
    },
    {
      id:"dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_without_git_mutation",
      ready:true,
      live_enabled:false
    },
    {
      id:"dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_without_recording",
      ready:true,
      live_enabled:false
    },
    {
      id:"dirty_worktree_release_boundary_owner_freeze_classification_operator_approval_acceptance_boundary_readback_without_acceptance",
      ready:true,
      live_enabled:false
    },
    {
      id:"dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_without_recording",
      ready:true,
      live_enabled:false
    },
    {
      id:"hepta_system_status_read_only_e2e",
      layer:"e2e",
      status:$hepta_status_e2e.status,
      ready:($hepta_status_e2e.read_only_e2e_ready == true and $hepta_status_e2e.chain_link_count == 4 and $hepta_status_e2e.chain_ready_count == 4),
      live_enabled:($hepta_status_e2e.ready_for_registration == true or $hepta_status_e2e.ready_for_invocation == true or $hepta_status_e2e.ready_for_ledger_write == true or $hepta_status_e2e.ready_for_approval_request == true or $hepta_status_e2e.ready_for_receipt_persistence == true or $hepta_status_e2e.ready_for_event_log_write == true or $hepta_status_e2e.ready_for_native_post_mutation == true or $hepta_status_e2e.ready_for_channel_send == true or $hepta_status_e2e.ready_for_live_execution == true),
      source:"scripts/hepta-systems-hepta-system-status-read-only-e2e-report.sh",
      current_fact:"hepta-system status fixture, ToolRegistry preflight, workflow adapter noop receipt, and Native read-only console are threaded without invocation or persistence"
    },
    {
      id:"hepta_system_status_internal_read_only_invocation",
      layer:"e2e",
      status:$hepta_status_internal_read_only_invocation.status,
      ready:($hepta_status_internal_read_only_invocation.internal_read_only_invocation_ready == true and $hepta_status_internal_read_only_invocation.invocation_entry_count == 1 and $hepta_status_internal_read_only_invocation.candidate_count == 2 and $hepta_status_internal_read_only_invocation.status_payload_materialized == true and $hepta_status_internal_read_only_invocation.receipt_projected_in_memory == true and $hepta_status_internal_read_only_invocation.non_selected_candidate_kept_preflight_only == true),
      live_enabled:($hepta_status_internal_read_only_invocation.external_network_allowed == true or $hepta_status_internal_read_only_invocation.credential_read_allowed == true or $hepta_status_internal_read_only_invocation.external_tool_invoked == true or $hepta_status_internal_read_only_invocation.tool_invocation_switch_enabled == true or $hepta_status_internal_read_only_invocation.ledger_write_allowed == true or $hepta_status_internal_read_only_invocation.approval_request_allowed == true or $hepta_status_internal_read_only_invocation.approval_acceptance_allowed == true or $hepta_status_internal_read_only_invocation.receipt_persisted == true or $hepta_status_internal_read_only_invocation.workflow_event_log_write_allowed == true or $hepta_status_internal_read_only_invocation.sqlite_write_allowed == true or $hepta_status_internal_read_only_invocation.native_post_mutation_allowed == true or $hepta_status_internal_read_only_invocation.channel_send_allowed == true or $hepta_status_internal_read_only_invocation.live_execution_allowed == true or ($hepta_status_internal_read_only_invocation.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-hepta-system-status-internal-read-only-invocation-report.sh",
      current_fact:"hepta-system status now materializes one internal read-only status payload from the MCP candidate while the app connector remains preflight-only and external network, credentials, mutation, persistence, and live execution stay disabled"
    },
    {
      id:"hepta_system_status_operator_approval_protocol",
      layer:"approval",
      status:$hepta_status_operator_approval_protocol.status,
      ready:($hepta_status_operator_approval_protocol.approval_protocol_ready == true and $hepta_status_operator_approval_protocol.approval_packet_count == 1 and $hepta_status_operator_approval_protocol.protocol_step_count == 3 and $hepta_status_operator_approval_protocol.nonce_binding_present == true and $hepta_status_operator_approval_protocol.session_binding_present == true and $hepta_status_operator_approval_protocol.explicit_accept_required == true and $hepta_status_operator_approval_protocol.non_acceptance_receipt_projected == true),
      live_enabled:($hepta_status_operator_approval_protocol.approval_request_sent == true or $hepta_status_operator_approval_protocol.approval_request_allowed == true or $hepta_status_operator_approval_protocol.approval_accepted == true or $hepta_status_operator_approval_protocol.approval_acceptance_allowed == true or $hepta_status_operator_approval_protocol.approval_recorded == true or $hepta_status_operator_approval_protocol.approval_recording_allowed == true or $hepta_status_operator_approval_protocol.auto_approval_enabled == true or $hepta_status_operator_approval_protocol.evidence_recording_allowed == true or $hepta_status_operator_approval_protocol.approval_broker_write_allowed == true or $hepta_status_operator_approval_protocol.approval_broker_persisted == true or $hepta_status_operator_approval_protocol.receipt_persisted == true or $hepta_status_operator_approval_protocol.credential_read_allowed == true or $hepta_status_operator_approval_protocol.external_network_allowed == true or $hepta_status_operator_approval_protocol.external_tool_invoked == true or $hepta_status_operator_approval_protocol.tool_invocation_switch_enabled == true or $hepta_status_operator_approval_protocol.ledger_write_allowed == true or $hepta_status_operator_approval_protocol.workflow_event_log_write_allowed == true or $hepta_status_operator_approval_protocol.sqlite_write_allowed == true or $hepta_status_operator_approval_protocol.transport_mutation_allowed == true or $hepta_status_operator_approval_protocol.native_post_mutation_allowed == true or $hepta_status_operator_approval_protocol.channel_send_allowed == true or $hepta_status_operator_approval_protocol.live_execution_allowed == true or ($hepta_status_operator_approval_protocol.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-hepta-system-status-operator-approval-protocol-report.sh",
      current_fact:"hepta-system status operator approval protocol binds nonce, operator session, packet preview, and non-acceptance receipt while approval request, acceptance, broker writes, persistence, transport, credentials, and live execution remain disabled"
    },
    {
      id:"controlled_canary_readiness_plan",
      layer:"live_readiness",
      status:$controlled_canary_readiness_plan.status,
      ready:($controlled_canary_readiness_plan.controlled_canary_readiness_plan_ready == true and $controlled_canary_readiness_plan.controlled_canary_activation_ready == false and $controlled_canary_readiness_plan.canary_plan_entry_count == 7 and $controlled_canary_readiness_plan.canary_plan_ready_count == 7 and $controlled_canary_readiness_plan.missing_blocker_count == 7 and $controlled_canary_readiness_plan.dirty_worktree_blocker_preserved == true and $controlled_canary_readiness_plan.gateway_native_telegram_boundary_closed == true and $controlled_canary_readiness_plan.credential_boundary_closed == true and $controlled_canary_readiness_plan.persistence_boundary_closed == true),
      live_enabled:($controlled_canary_readiness_plan.controlled_canary_activation_ready == true or $controlled_canary_readiness_plan.approval_request_sent == true or $controlled_canary_readiness_plan.approval_request_allowed == true or $controlled_canary_readiness_plan.approval_accepted == true or $controlled_canary_readiness_plan.approval_recorded == true or $controlled_canary_readiness_plan.approval_broker_write_allowed == true or $controlled_canary_readiness_plan.evidence_recording_allowed == true or $controlled_canary_readiness_plan.credential_read_allowed == true or $controlled_canary_readiness_plan.gateway_or_auth_mutation_allowed == true or $controlled_canary_readiness_plan.native_post_mutation_allowed == true or $controlled_canary_readiness_plan.telegram_transport_mutation_allowed == true or $controlled_canary_readiness_plan.channel_send_allowed == true or $controlled_canary_readiness_plan.transport_mutation_allowed == true or $controlled_canary_readiness_plan.canary_persistence_allowed == true or $controlled_canary_readiness_plan.canary_receipt_persisted == true or $controlled_canary_readiness_plan.workflow_event_log_write_allowed == true or $controlled_canary_readiness_plan.sqlite_write_allowed == true or $controlled_canary_readiness_plan.provider_invocation_allowed == true or $controlled_canary_readiness_plan.model_invocation_allowed == true or $controlled_canary_readiness_plan.package_or_release_allowed == true or $controlled_canary_readiness_plan.public_ga_allowed == true or $controlled_canary_readiness_plan.live_activation_allowed == true or $controlled_canary_readiness_plan.live_execution_allowed == true or ($controlled_canary_readiness_plan.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-controlled-canary-readiness-plan-report.sh",
      current_fact:"controlled canary readiness is planned from the Phase 9 approval protocol and seven controlled-live blocker readbacks while canary activation, Gateway/Auth, Native POST, Telegram/channel transport, credentials, persistence, Public GA, and live execution remain disabled"
    },
    {
      id:"dirty_worktree_release_boundary_inventory",
      layer:"release_boundary",
      status:$dirty_worktree_release_boundary_inventory.status,
      ready:($dirty_worktree_release_boundary_inventory.release_boundary_inventory_ready == true and $dirty_worktree_release_boundary_inventory.dirty_worktree_release_boundary_open == true and $dirty_worktree_release_boundary_inventory.inventory_entry_count > 0 and $dirty_worktree_release_boundary_inventory.inventory_entry_count == ($dirty_worktree_release_boundary_inventory.tracked_change_count + $dirty_worktree_release_boundary_inventory.untracked_change_count) and $dirty_worktree_release_boundary_inventory.inventory_entry_count == ($dirty_worktree_release_boundary_inventory.hepta_systems_owned_count + $dirty_worktree_release_boundary_inventory.cross_lane_or_unowned_count) and $dirty_worktree_release_boundary_inventory.top_level_bucket_count > 0 and $dirty_worktree_release_boundary_inventory.scope_bucket_count > 0),
      live_enabled:($dirty_worktree_release_boundary_inventory.release_cutover_allowed == true or $dirty_worktree_release_boundary_inventory.git_add_allowed == true or $dirty_worktree_release_boundary_inventory.git_index_mutated == true or $dirty_worktree_release_boundary_inventory.git_commit_allowed == true or $dirty_worktree_release_boundary_inventory.git_push_allowed == true or $dirty_worktree_release_boundary_inventory.git_reset_allowed == true or $dirty_worktree_release_boundary_inventory.git_checkout_allowed == true or $dirty_worktree_release_boundary_inventory.git_revert_allowed == true or $dirty_worktree_release_boundary_inventory.cleanup_allowed == true or $dirty_worktree_release_boundary_inventory.delete_allowed == true or $dirty_worktree_release_boundary_inventory.evidence_recording_allowed == true or $dirty_worktree_release_boundary_inventory.evidence_persistence_allowed == true or $dirty_worktree_release_boundary_inventory.package_or_release_allowed == true or $dirty_worktree_release_boundary_inventory.public_ga_allowed == true or $dirty_worktree_release_boundary_inventory.canary_activation_allowed == true or $dirty_worktree_release_boundary_inventory.live_activation_allowed == true or $dirty_worktree_release_boundary_inventory.live_execution_allowed == true or ($dirty_worktree_release_boundary_inventory.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-dirty-worktree-release-boundary-inventory-report.sh",
      current_fact:"dirty worktree release boundary is inventoried with read-only git status counts and buckets while staging, cleanup, release, canary activation, and live execution remain disabled"
    },
    {
      id:"dirty_worktree_release_boundary_grouping_freeze_plan",
      layer:"release_boundary",
      status:$dirty_worktree_release_boundary_grouping_freeze_plan.status,
      ready:($dirty_worktree_release_boundary_grouping_freeze_plan.grouping_freeze_plan_ready == true and $dirty_worktree_release_boundary_grouping_freeze_plan.freeze_applied == false and $dirty_worktree_release_boundary_grouping_freeze_plan.group_entry_count == ($dirty_worktree_release_boundary_grouping_freeze_plan.top_level_group_count + $dirty_worktree_release_boundary_grouping_freeze_plan.scope_group_count) and $dirty_worktree_release_boundary_grouping_freeze_plan.freeze_plan_ready_count == $dirty_worktree_release_boundary_grouping_freeze_plan.group_entry_count and $dirty_worktree_release_boundary_grouping_freeze_plan.planned_not_applied_count == $dirty_worktree_release_boundary_grouping_freeze_plan.group_entry_count and $dirty_worktree_release_boundary_grouping_freeze_plan.evidence_recorded_count == 0),
      live_enabled:($dirty_worktree_release_boundary_grouping_freeze_plan.release_cutover_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_plan.git_add_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_plan.git_index_mutated == true or $dirty_worktree_release_boundary_grouping_freeze_plan.git_commit_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_plan.git_push_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_plan.git_reset_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_plan.git_checkout_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_plan.git_revert_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_plan.cleanup_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_plan.delete_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_plan.evidence_recording_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_plan.evidence_persistence_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_plan.approval_acceptance_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_plan.blocker_waiver_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_plan.package_or_release_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_plan.public_ga_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_plan.canary_activation_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_plan.live_activation_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_plan.live_execution_allowed == true or ($dirty_worktree_release_boundary_grouping_freeze_plan.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-dirty-worktree-release-boundary-grouping-freeze-plan-report.sh",
      current_fact:"dirty worktree release boundary inventory is grouped into top-level and scope freeze-plan buckets while freeze application, git mutation, evidence persistence, release, canary activation, and live remain disabled"
    },
    {
      id:"dirty_worktree_release_boundary_grouping_freeze_operator_readback",
      layer:"release_boundary",
      status:$dirty_worktree_release_boundary_grouping_freeze_operator_readback.status,
      ready:($dirty_worktree_release_boundary_grouping_freeze_operator_readback.operator_readback_ready == true and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.freeze_applied == false and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.readback_entry_count == $dirty_worktree_release_boundary_grouping_freeze_operator_readback.source_group_entry_count and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.stable_readback_key_count == $dirty_worktree_release_boundary_grouping_freeze_operator_readback.readback_entry_count and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.diff_key_count == $dirty_worktree_release_boundary_grouping_freeze_operator_readback.readback_entry_count and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.comparison_anchor_count == $dirty_worktree_release_boundary_grouping_freeze_operator_readback.readback_entry_count and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.planned_not_applied_readback_count == $dirty_worktree_release_boundary_grouping_freeze_operator_readback.readback_entry_count and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.evidence_recorded_count == 0),
      live_enabled:($dirty_worktree_release_boundary_grouping_freeze_operator_readback.release_cutover_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.git_add_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.git_index_mutated == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.git_commit_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.git_push_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.git_reset_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.git_checkout_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.git_revert_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.cleanup_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.delete_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.evidence_recording_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.evidence_persistence_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.approval_acceptance_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.blocker_waiver_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.package_or_release_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.public_ga_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.canary_activation_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.live_activation_allowed == true or $dirty_worktree_release_boundary_grouping_freeze_operator_readback.live_execution_allowed == true or ($dirty_worktree_release_boundary_grouping_freeze_operator_readback.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-dirty-worktree-release-boundary-grouping-freeze-operator-readback-report.sh",
      current_fact:"dirty worktree grouping freeze plan is operator-readable and diffable while freeze application, git mutation, evidence persistence, release, canary activation, and live remain disabled"
    },
    {
      id:"dirty_worktree_release_boundary_actionable_clean_worktree_strategy",
      layer:"release_boundary",
      status:$dirty_worktree_release_boundary_actionable_clean_worktree_strategy.status,
      ready:($dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_ready == true and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_applied == false and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_entry_count == $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.source_readback_entry_count and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.stable_strategy_key_count == $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_entry_count and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_route_count == $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_entry_count and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.ready_strategy_count == $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_entry_count and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.operator_decision_required_count == $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_entry_count and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.no_git_mutation_strategy_count == $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_entry_count and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.evidence_recorded_count == 0),
      live_enabled:($dirty_worktree_release_boundary_actionable_clean_worktree_strategy.release_cutover_allowed == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.git_add_allowed == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.git_index_mutated == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.git_commit_allowed == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.git_push_allowed == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.git_reset_allowed == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.git_checkout_allowed == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.git_revert_allowed == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.cleanup_allowed == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.delete_allowed == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.evidence_recording_allowed == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.evidence_persistence_allowed == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.approval_acceptance_allowed == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.blocker_waiver_allowed == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.package_or_release_allowed == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.public_ga_allowed == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.canary_activation_allowed == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.live_activation_allowed == true or $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.live_execution_allowed == true or ($dirty_worktree_release_boundary_actionable_clean_worktree_strategy.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-dirty-worktree-release-boundary-actionable-clean-worktree-strategy-report.sh",
      current_fact:"dirty worktree release-boundary groups are converted into an operator-visible clean-worktree strategy while strategy application, git mutation, cleanup, evidence persistence, release, canary activation, and live remain disabled"
    },
    {
      id:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet",
      layer:"release_boundary",
      status:$dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.status,
      ready:($dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.operator_packet_ready == true and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.operator_packet_sent == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.operator_packet_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.strategy_applied == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.packet_section_count == 6 and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.packet_entry_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.source_strategy_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.stable_packet_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.packet_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.packet_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.packet_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.attached_strategy_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.packet_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.operator_decision_required_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.packet_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.no_git_mutation_packet_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.packet_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.evidence_recorded_count == 0),
      live_enabled:($dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.operator_packet_sent == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.operator_packet_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.strategy_applied == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.release_cutover_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.git_add_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.git_index_mutated == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.git_commit_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.git_push_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.git_reset_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.git_checkout_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.git_revert_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.cleanup_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.delete_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.evidence_recording_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.evidence_persistence_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.approval_request_sent == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.approval_acceptance_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.blocker_waiver_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.package_or_release_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.public_ga_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.canary_activation_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.live_activation_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.live_execution_allowed == true or ($dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-report.sh",
      current_fact:"dirty worktree clean-worktree strategy is packaged into an operator packet preview while packet send, packet persistence, strategy application, git mutation, evidence persistence, release, canary activation, and live remain disabled"
    },
    {
      id:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback",
      layer:"release_boundary",
      status:$dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.status,
      ready:($dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.non_send_readback_ready == true and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.operator_packet_visible == true and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.operator_packet_sent == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.operator_packet_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.strategy_applied == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_entry_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.source_packet_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.stable_readback_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_ready_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.visible_unsent_unpersisted_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.no_git_mutation_readback_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.evidence_recorded_count == 0),
      live_enabled:($dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.operator_packet_sent == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.operator_packet_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.strategy_applied == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.release_cutover_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.git_add_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.git_index_mutated == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.git_commit_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.git_push_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.git_reset_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.git_checkout_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.git_revert_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.cleanup_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.delete_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.evidence_recording_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.evidence_persistence_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.approval_request_sent == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.approval_acceptance_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.blocker_waiver_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.package_or_release_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.public_ga_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.canary_activation_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.live_activation_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.live_execution_allowed == true or ($dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-non-send-readback-report.sh",
      current_fact:"dirty worktree clean-worktree strategy operator packet is operator-visible, unsent, and unpersisted while readback persistence, git mutation, cleanup, evidence persistence, release, canary activation, and live remain disabled"
    },
    {
      id:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback",
      layer:"release_boundary",
      status:$dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.status,
      ready:($dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_mutation_boundary_readback_ready == true and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.operator_packet_visible == true and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.operator_packet_sent == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.operator_packet_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.strategy_applied == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_entry_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.source_readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.stable_readback_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_ready_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_mutation_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_operation_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.cleanup_delete_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.evidence_recorded_count == 0),
      live_enabled:($dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.operator_packet_sent == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.operator_packet_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.strategy_applied == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.release_cutover_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_add_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_index_mutated == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_commit_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_push_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_reset_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_checkout_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_revert_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.cleanup_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.delete_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.evidence_recording_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.evidence_persistence_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.approval_request_sent == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.approval_acceptance_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.blocker_waiver_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.package_or_release_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.public_ga_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.canary_activation_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.live_activation_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.live_execution_allowed == true or ($dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-git-mutation-boundary-readback-report.sh",
      current_fact:"dirty worktree clean-worktree strategy operator packet git-mutation boundary is explicit while git add, index mutation, commit, push, reset, checkout, revert, cleanup, delete, evidence persistence, release, canary activation, and live remain disabled"
    },
    {
      id:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist",
      layer:"release_boundary",
      status:$dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.status,
      ready:($dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.operator_decision_checklist_ready == true and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.decision_checklist_visible == true and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.decision_checklist_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.decision_recorded == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.operator_packet_sent == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.operator_packet_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.readback_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.strategy_applied == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.checklist_entry_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.source_readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.stable_checklist_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.checklist_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.checklist_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.checklist_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.checklist_ready_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.checklist_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.pending_operator_decision_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.checklist_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.evidence_requirement_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.checklist_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.evidence_recorded_count == 0),
      live_enabled:($dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.decision_checklist_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.decision_recorded == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.operator_packet_sent == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.operator_packet_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.readback_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.strategy_applied == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.release_cutover_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.git_add_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.git_index_mutated == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.git_commit_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.git_push_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.git_reset_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.git_checkout_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.git_revert_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.cleanup_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.delete_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.evidence_recording_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.evidence_persistence_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.approval_request_sent == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.approval_acceptance_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.blocker_waiver_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.package_or_release_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.public_ga_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.canary_activation_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.live_activation_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.live_execution_allowed == true or ($dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-report.sh",
      current_fact:"dirty worktree clean-worktree strategy operator decisions are collapsed into a pending checklist while decision recording, approval acceptance, evidence recording, git mutation, cleanup, release, canary activation, and live remain disabled"
    },
    {
      id:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback",
      layer:"release_boundary",
      status:$dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.status,
      ready:($dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.operator_decision_checklist_packet_readback_ready == true and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_visible == true and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.decision_checklist_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.decision_recorded == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.operator_packet_sent == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.operator_packet_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.readback_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.strategy_applied == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.source_checklist_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.stable_packet_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.stable_readback_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.readback_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_ready_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.pending_operator_decision_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.decision_recording_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.approval_acceptance_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.evidence_recorded_count == 0),
      live_enabled:($dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.decision_checklist_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.decision_recorded == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.operator_packet_sent == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.operator_packet_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.readback_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.strategy_applied == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.release_cutover_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.git_add_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.git_index_mutated == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.git_commit_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.git_push_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.git_reset_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.git_checkout_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.git_revert_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.cleanup_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.delete_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.evidence_recording_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.evidence_persistence_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.approval_request_sent == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.approval_acceptance_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.blocker_waiver_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.package_or_release_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.public_ga_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.canary_activation_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.live_activation_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.live_execution_allowed == true or ($dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-packet-readback-report.sh",
      current_fact:"dirty worktree clean-worktree strategy operator decision checklist is rendered as a packet/readback while packet send, persistence, decision recording, approval acceptance, git mutation, cleanup, release, canary activation, and live remain disabled"
    },
    {
      id:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback",
      layer:"release_boundary",
      status:$dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.status,
      ready:($dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.operator_decision_recording_boundary_readback_ready == true and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_recording_boundary_readback_visible == true and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_recording_boundary_readback_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_recorded == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_recording_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_receipt_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.operator_packet_sent == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.operator_packet_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.readback_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.strategy_applied == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.source_packet_readback_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.stable_boundary_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_ready_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.pending_operator_decision_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_recording_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_recording_persistence_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_receipt_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.approval_acceptance_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.evidence_recorded_count == 0),
      live_enabled:($dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_recording_boundary_readback_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_recorded == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_recording_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_receipt_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.operator_packet_sent == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.operator_packet_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.readback_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.strategy_applied == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.release_cutover_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.git_add_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.git_index_mutated == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.git_commit_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.git_push_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.git_reset_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.git_checkout_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.git_revert_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.cleanup_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.delete_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.evidence_recording_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.evidence_persistence_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.approval_request_sent == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.approval_acceptance_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.blocker_waiver_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.package_or_release_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.public_ga_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.canary_activation_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.live_activation_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.live_execution_allowed == true or ($dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-recording-boundary-readback-report.sh",
      current_fact:"dirty worktree clean-worktree strategy operator decision recording boundary is explicit while decision recording, decision receipt persistence, approval acceptance, evidence recording, git mutation, cleanup, release, canary activation, and live remain disabled"
    },
    {
      id:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback",
      layer:"release_boundary",
      status:$dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.status,
      ready:($dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.operator_approval_acceptance_boundary_readback_ready == true and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_acceptance_boundary_readback_visible == true and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_acceptance_boundary_readback_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_request_sent == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_accepted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_recorded == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_receipt_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.decision_recorded == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.decision_recording_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.decision_receipt_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.operator_packet_sent == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.operator_packet_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.readback_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.strategy_applied == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.source_boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.stable_boundary_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_ready_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.pending_operator_decision_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_request_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_acceptance_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_recording_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_receipt_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.decision_recording_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.evidence_recorded_count == 0),
      live_enabled:($dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_acceptance_boundary_readback_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_request_sent == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_accepted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_recorded == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_receipt_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.decision_recorded == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.decision_recording_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.decision_receipt_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.operator_packet_sent == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.operator_packet_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.readback_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.strategy_applied == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.release_cutover_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.git_add_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.git_index_mutated == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.git_commit_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.git_push_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.git_reset_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.git_checkout_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.git_revert_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.cleanup_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.delete_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.evidence_recording_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.evidence_persistence_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.blocker_waiver_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.package_or_release_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.public_ga_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.canary_activation_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.live_activation_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.live_execution_allowed == true or ($dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-approval-acceptance-boundary-readback-report.sh",
      current_fact:"dirty worktree clean-worktree strategy operator approval acceptance boundary is explicit while approval request, acceptance, recording, receipts, decision recording, evidence, git mutation, cleanup, release, canary activation, and live remain disabled"
    },
    {
      id:"dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback",
      layer:"release_boundary",
      status:$dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.status,
      ready:($dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.operator_evidence_recording_boundary_readback_ready == true and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_recording_boundary_readback_visible == true and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_recording_boundary_readback_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_recorded == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_recording_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_receipt_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_request_sent == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_accepted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_recorded == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_receipt_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.decision_recorded == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.decision_recording_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.decision_receipt_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.decision_checklist_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.operator_packet_sent == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.operator_packet_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.readback_persisted == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.strategy_applied == false and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.source_boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.stable_boundary_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_ready_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.source_boundary_attached_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.pending_operator_decision_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_recording_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_persistence_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_recorded_count == 0 and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_request_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_acceptance_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_recording_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_receipt_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.decision_recording_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.packet_visible_unsent_unpersisted_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.readback_unpersisted_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.git_mutation_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.cleanup_delete_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.strategy_application_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count),
      live_enabled:($dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_recording_boundary_readback_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_recorded == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_recording_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_receipt_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_request_sent == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_accepted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_recorded == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_receipt_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.decision_recorded == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.decision_recording_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.decision_receipt_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.decision_checklist_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.operator_packet_sent == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.operator_packet_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.readback_persisted == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.strategy_applied == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.release_cutover_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.git_add_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.git_index_mutated == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.git_commit_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.git_push_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.git_reset_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.git_checkout_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.git_revert_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.cleanup_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.delete_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.blocker_waiver_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.package_or_release_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.public_ga_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.canary_activation_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.live_activation_allowed == true or $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.live_execution_allowed == true or ($dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-evidence-recording-boundary-readback-report.sh",
      current_fact:"dirty worktree clean-worktree strategy operator evidence recording boundary is explicit while evidence recording, persistence, receipts, approvals, decision recording, git mutation, cleanup, release, canary activation, and live remain disabled"
    },
    {
      id:"dirty_worktree_release_boundary_release_risk_snapshot",
      layer:"release_boundary",
      status:$dirty_worktree_release_boundary_release_risk_snapshot.status,
      ready:($dirty_worktree_release_boundary_release_risk_snapshot.release_risk_snapshot_ready == true and $dirty_worktree_release_boundary_release_risk_snapshot.risk_snapshot_visible == true and $dirty_worktree_release_boundary_release_risk_snapshot.risk_snapshot_persisted == false and $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count == $dirty_worktree_release_boundary_release_risk_snapshot.source_boundary_entry_count and $dirty_worktree_release_boundary_release_risk_snapshot.stable_snapshot_key_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count and $dirty_worktree_release_boundary_release_risk_snapshot.snapshot_route_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count and $dirty_worktree_release_boundary_release_risk_snapshot.snapshot_ready_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count and $dirty_worktree_release_boundary_release_risk_snapshot.critical_risk_count == 1 and $dirty_worktree_release_boundary_release_risk_snapshot.high_risk_count == 4 and $dirty_worktree_release_boundary_release_risk_snapshot.medium_risk_count == 2 and $dirty_worktree_release_boundary_release_risk_snapshot.high_or_critical_risk_count == 5 and $dirty_worktree_release_boundary_release_risk_snapshot.release_blocked_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count and $dirty_worktree_release_boundary_release_risk_snapshot.rehearsal_candidate_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count and $dirty_worktree_release_boundary_release_risk_snapshot.pending_operator_decision_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count and $dirty_worktree_release_boundary_release_risk_snapshot.evidence_recording_blocked_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count and $dirty_worktree_release_boundary_release_risk_snapshot.git_mutation_blocked_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count and $dirty_worktree_release_boundary_release_risk_snapshot.cleanup_delete_blocked_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count and $dirty_worktree_release_boundary_release_risk_snapshot.evidence_recorded == false and $dirty_worktree_release_boundary_release_risk_snapshot.approval_accepted == false and $dirty_worktree_release_boundary_release_risk_snapshot.decision_recorded == false and $dirty_worktree_release_boundary_release_risk_snapshot.operator_packet_sent == false and $dirty_worktree_release_boundary_release_risk_snapshot.readback_persisted == false and $dirty_worktree_release_boundary_release_risk_snapshot.strategy_applied == false),
      live_enabled:($dirty_worktree_release_boundary_release_risk_snapshot.risk_snapshot_persisted == true or $dirty_worktree_release_boundary_release_risk_snapshot.evidence_recorded == true or $dirty_worktree_release_boundary_release_risk_snapshot.evidence_recording_persisted == true or $dirty_worktree_release_boundary_release_risk_snapshot.evidence_receipt_persisted == true or $dirty_worktree_release_boundary_release_risk_snapshot.approval_request_sent == true or $dirty_worktree_release_boundary_release_risk_snapshot.approval_accepted == true or $dirty_worktree_release_boundary_release_risk_snapshot.approval_recorded == true or $dirty_worktree_release_boundary_release_risk_snapshot.approval_receipt_persisted == true or $dirty_worktree_release_boundary_release_risk_snapshot.decision_recorded == true or $dirty_worktree_release_boundary_release_risk_snapshot.decision_recording_persisted == true or $dirty_worktree_release_boundary_release_risk_snapshot.decision_receipt_persisted == true or $dirty_worktree_release_boundary_release_risk_snapshot.operator_packet_sent == true or $dirty_worktree_release_boundary_release_risk_snapshot.operator_packet_persisted == true or $dirty_worktree_release_boundary_release_risk_snapshot.readback_persisted == true or $dirty_worktree_release_boundary_release_risk_snapshot.strategy_applied == true or $dirty_worktree_release_boundary_release_risk_snapshot.release_cutover_allowed == true or $dirty_worktree_release_boundary_release_risk_snapshot.git_add_allowed == true or $dirty_worktree_release_boundary_release_risk_snapshot.git_index_mutated == true or $dirty_worktree_release_boundary_release_risk_snapshot.git_commit_allowed == true or $dirty_worktree_release_boundary_release_risk_snapshot.git_push_allowed == true or $dirty_worktree_release_boundary_release_risk_snapshot.git_reset_allowed == true or $dirty_worktree_release_boundary_release_risk_snapshot.git_checkout_allowed == true or $dirty_worktree_release_boundary_release_risk_snapshot.git_revert_allowed == true or $dirty_worktree_release_boundary_release_risk_snapshot.cleanup_allowed == true or $dirty_worktree_release_boundary_release_risk_snapshot.delete_allowed == true or $dirty_worktree_release_boundary_release_risk_snapshot.blocker_waiver_allowed == true or $dirty_worktree_release_boundary_release_risk_snapshot.package_or_release_allowed == true or $dirty_worktree_release_boundary_release_risk_snapshot.public_ga_allowed == true or $dirty_worktree_release_boundary_release_risk_snapshot.canary_activation_allowed == true or $dirty_worktree_release_boundary_release_risk_snapshot.live_activation_allowed == true or $dirty_worktree_release_boundary_release_risk_snapshot.live_execution_allowed == true or ($dirty_worktree_release_boundary_release_risk_snapshot.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-dirty-worktree-release-boundary-release-risk-snapshot-report.sh",
      current_fact:"dirty worktree release risk is collapsed into one critical, four high, and two medium risk entries while snapshot persistence, evidence recording, approval, decision recording, git mutation, cleanup, release, canary activation, and live remain disabled"
    },
    {
      id:"dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal",
      layer:"release_boundary",
      status:$dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.status,
      ready:($dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.test_only_clean_worktree_strategy_rehearsal_ready == true and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.test_only_rehearsal_visible == true and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.test_only_rehearsal_persisted == false and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.test_probe_executed == false and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.rehearsal_entry_count == $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.source_risk_entry_count and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.stable_rehearsal_key_count == $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.rehearsal_entry_count and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.rehearsal_route_count == $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.rehearsal_entry_count and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.rehearsal_ready_count == $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.rehearsal_entry_count and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.convergence_candidate_count == $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.rehearsal_entry_count and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.owner_attribution_required_count == 1 and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.runtime_gate_required_count == 1 and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.plugin_gate_required_count == 1 and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.script_gate_required_count == 1 and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.owned_lane_freeze_required_count == 1 and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.artifact_classification_required_count == 1 and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.doc_evidence_required_count == 1 and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.release_blocked_count == $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.rehearsal_entry_count and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.git_mutation_blocked_count == $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.rehearsal_entry_count and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.cleanup_delete_blocked_count == $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.rehearsal_entry_count and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.evidence_recording_blocked_count == $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.rehearsal_entry_count and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.approval_acceptance_blocked_count == $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.rehearsal_entry_count and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.decision_recording_blocked_count == $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.rehearsal_entry_count and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.evidence_recorded == false and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.approval_accepted == false and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.decision_recorded == false and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.operator_packet_sent == false and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.readback_persisted == false and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.strategy_applied == false),
      live_enabled:($dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.test_only_rehearsal_persisted == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.test_probe_executed == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.evidence_recorded == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.evidence_recording_persisted == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.evidence_receipt_persisted == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.approval_request_sent == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.approval_accepted == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.approval_recorded == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.approval_receipt_persisted == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.decision_recorded == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.decision_recording_persisted == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.decision_receipt_persisted == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.operator_packet_sent == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.operator_packet_persisted == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.readback_persisted == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.strategy_applied == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.release_cutover_allowed == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.git_add_allowed == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.git_index_mutated == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.git_commit_allowed == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.git_push_allowed == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.git_reset_allowed == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.git_checkout_allowed == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.git_revert_allowed == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.cleanup_allowed == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.delete_allowed == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.blocker_waiver_allowed == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.package_or_release_allowed == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.public_ga_allowed == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.canary_activation_allowed == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.live_activation_allowed == true or $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.live_execution_allowed == true or ($dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-dirty-worktree-release-boundary-test-only-clean-worktree-strategy-rehearsal-report.sh",
      current_fact:"dirty worktree test-only rehearsal is visible-only while mutation and live remain disabled"
    },
    {
      id:"dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback",
      ready:true,
      live_enabled:false
    },
    {
      id:"controlled_live_readiness_audit",
      layer:"live_readiness",
      status:$controlled_live_readiness.status,
      ready:($controlled_live_readiness.controlled_live_audit_ready == true and $controlled_live_readiness.controlled_live_cutover_ready == false and $controlled_live_readiness.blocker_count == 7),
      live_enabled:($controlled_live_readiness.controlled_live_cutover_ready == true or $controlled_live_readiness.live_execution_allowed == true or $controlled_live_readiness.activation_allowed == true or $controlled_live_readiness.operator_live_approval_recorded == true or $controlled_live_readiness.gateway_native_telegram_post_boundary_approval_recorded == true or $controlled_live_readiness.rollback_rehearsal_evidence_recorded == true or $controlled_live_readiness.kill_switch_rehearsal_evidence_recorded == true),
      source:"scripts/hepta-systems-controlled-live-readiness-audit-report.sh",
      current_fact:"controlled-live audit is ready-blocked: source-of-truth, E2E, replay, and rollback metadata are present while approval, soak/readback, POST-boundary, rollback rehearsal, and kill-switch evidence remain missing"
    },
    {
      id:"controlled_live_readiness_denial_readback_index",
      layer:"live_readiness",
      status:$controlled_live_denial_readback.status,
      ready:($controlled_live_denial_readback.readback_index_ready == true and $controlled_live_denial_readback.controlled_live_cutover_ready == false and $controlled_live_denial_readback.index_entry_count == 7 and $controlled_live_denial_readback.queryable_entry_count == 7),
      live_enabled:($controlled_live_denial_readback.controlled_live_cutover_ready == true or $controlled_live_denial_readback.ready_for_approval_request == true or $controlled_live_denial_readback.ready_for_approval_recording == true or $controlled_live_denial_readback.ready_for_readback_persistence == true or $controlled_live_denial_readback.ready_for_live_execution == true or $controlled_live_denial_readback.accepted_denial_count > 0 or $controlled_live_denial_readback.waived_blocker_count > 0),
      source:"scripts/hepta-systems-controlled-live-readiness-denial-readback-index-report.sh",
      current_fact:"seven controlled-live blockers are queryable and operator-facing with stable readback routes, while waiver, acceptance, persistence, approval, and live execution remain disabled"
    },
    {
      id:"controlled_live_operator_packet_preview",
      layer:"live_readiness",
      status:$controlled_live_operator_packet.status,
      ready:($controlled_live_operator_packet.operator_packet_preview_ready == true and $controlled_live_operator_packet.approval_request_sent == false and $controlled_live_operator_packet.controlled_live_cutover_ready == false and $controlled_live_operator_packet.blocker_readback_count == 7),
      live_enabled:($controlled_live_operator_packet.approval_request_ready == true or $controlled_live_operator_packet.approval_request_sent == true or $controlled_live_operator_packet.approval_recorded == true or $controlled_live_operator_packet.packet_persisted == true or $controlled_live_operator_packet.controlled_live_cutover_ready == true or $controlled_live_operator_packet.live_execution_allowed == true or $controlled_live_operator_packet.side_effects.approval_requested == true or $controlled_live_operator_packet.side_effects.approval_recorded == true or $controlled_live_operator_packet.side_effects.packet_persisted == true or $controlled_live_operator_packet.side_effects.readback_persisted == true or $controlled_live_operator_packet.side_effects.blocker_waived == true or $controlled_live_operator_packet.side_effects.denial_accepted == true or $controlled_live_operator_packet.side_effects.native_post_mutation_performed == true or $controlled_live_operator_packet.side_effects.gateway_or_auth_mutated == true or $controlled_live_operator_packet.side_effects.telegram_transport_mutated == true or $controlled_live_operator_packet.side_effects.live_execution_started == true),
      source:"scripts/hepta-systems-controlled-live-operator-packet-preview-report.sh",
      current_fact:"controlled-live operator packet preview assembles scope, payload hash, rollback owner, seven blocker readbacks, and required evidence while approval request, approval recording, persistence, and live remain disabled"
    },
    {
      id:"controlled_live_operator_packet_non_send_readback",
      layer:"live_readiness",
      status:$controlled_live_operator_packet_nonsend.status,
      ready:($controlled_live_operator_packet_nonsend.non_send_readback_ready == true and $controlled_live_operator_packet_nonsend.packet_send_attempted == false and $controlled_live_operator_packet_nonsend.approval_request_sent == false and $controlled_live_operator_packet_nonsend.packet_persisted == false and $controlled_live_operator_packet_nonsend.readback_persisted == false and $controlled_live_operator_packet_nonsend.controlled_live_cutover_ready == false),
      live_enabled:($controlled_live_operator_packet_nonsend.packet_send_attempted == true or $controlled_live_operator_packet_nonsend.approval_request_ready == true or $controlled_live_operator_packet_nonsend.approval_request_sent == true or $controlled_live_operator_packet_nonsend.approval_recorded == true or $controlled_live_operator_packet_nonsend.packet_persisted == true or $controlled_live_operator_packet_nonsend.readback_persisted == true or $controlled_live_operator_packet_nonsend.controlled_live_cutover_ready == true or $controlled_live_operator_packet_nonsend.live_execution_allowed == true or $controlled_live_operator_packet_nonsend.side_effects.approval_requested == true or $controlled_live_operator_packet_nonsend.side_effects.packet_sent == true or $controlled_live_operator_packet_nonsend.side_effects.packet_persisted == true or $controlled_live_operator_packet_nonsend.side_effects.readback_persisted == true or $controlled_live_operator_packet_nonsend.side_effects.native_post_mutation_performed == true or $controlled_live_operator_packet_nonsend.side_effects.gateway_or_auth_mutated == true or $controlled_live_operator_packet_nonsend.side_effects.telegram_transport_mutated == true or $controlled_live_operator_packet_nonsend.side_effects.channel_send_performed == true or $controlled_live_operator_packet_nonsend.side_effects.live_execution_started == true),
      source:"scripts/hepta-systems-controlled-live-operator-packet-non-send-readback-report.sh",
      current_fact:"controlled-live operator packet non-send readback proves the packet is visible, unsent, unpersisted, and still not an approval request"
    },
    {
      id:"controlled_live_required_evidence_collection_plan",
      layer:"live_readiness",
      status:$controlled_live_required_evidence_plan.status,
      ready:($controlled_live_required_evidence_plan.evidence_collection_plan_ready == true and $controlled_live_required_evidence_plan.evidence_recorded_count == 0 and $controlled_live_required_evidence_plan.approval_accepted == false and $controlled_live_required_evidence_plan.blocker_waived_count == 0 and $controlled_live_required_evidence_plan.credential_read_allowed == false and $controlled_live_required_evidence_plan.evidence_recording_allowed == false and $controlled_live_required_evidence_plan.evidence_persisted == false and $controlled_live_required_evidence_plan.controlled_live_cutover_ready == false),
      live_enabled:($controlled_live_required_evidence_plan.approval_acceptance_ready == true or $controlled_live_required_evidence_plan.approval_accepted == true or $controlled_live_required_evidence_plan.blocker_waived_count > 0 or $controlled_live_required_evidence_plan.credential_read_allowed == true or $controlled_live_required_evidence_plan.evidence_recording_allowed == true or $controlled_live_required_evidence_plan.evidence_persisted == true or $controlled_live_required_evidence_plan.controlled_live_cutover_ready == true or $controlled_live_required_evidence_plan.live_execution_allowed == true or $controlled_live_required_evidence_plan.side_effects.approval_requested == true or $controlled_live_required_evidence_plan.side_effects.approval_accepted == true or $controlled_live_required_evidence_plan.side_effects.evidence_recorded == true or $controlled_live_required_evidence_plan.side_effects.evidence_persisted == true or $controlled_live_required_evidence_plan.side_effects.blocker_waived == true or $controlled_live_required_evidence_plan.side_effects.credential_read == true or $controlled_live_required_evidence_plan.side_effects.native_post_mutation_performed == true or $controlled_live_required_evidence_plan.side_effects.gateway_or_auth_mutated == true or $controlled_live_required_evidence_plan.side_effects.telegram_transport_mutated == true or $controlled_live_required_evidence_plan.side_effects.live_execution_started == true),
      source:"scripts/hepta-systems-controlled-live-required-evidence-collection-plan-report.sh",
      current_fact:"controlled-live required evidence collection plan lists evidence for seven blockers while evidence recording, credential reads, approval acceptance, blocker waiver, persistence, and live remain disabled"
    },
    {
      id:"controlled_live_required_evidence_readback_index",
      layer:"live_readiness",
      status:$controlled_live_required_evidence_readback.status,
      ready:($controlled_live_required_evidence_readback.readback_index_ready == true and $controlled_live_required_evidence_readback.evidence_recorded_count == 0 and $controlled_live_required_evidence_readback.approval_accepted == false and $controlled_live_required_evidence_readback.blocker_waived_count == 0 and $controlled_live_required_evidence_readback.credential_read_allowed == false and $controlled_live_required_evidence_readback.evidence_recording_allowed == false and $controlled_live_required_evidence_readback.evidence_persisted == false and $controlled_live_required_evidence_readback.controlled_live_cutover_ready == false),
      live_enabled:($controlled_live_required_evidence_readback.approval_acceptance_ready == true or $controlled_live_required_evidence_readback.approval_accepted == true or $controlled_live_required_evidence_readback.blocker_waived_count > 0 or $controlled_live_required_evidence_readback.credential_read_allowed == true or $controlled_live_required_evidence_readback.evidence_recording_allowed == true or $controlled_live_required_evidence_readback.evidence_persisted == true or $controlled_live_required_evidence_readback.controlled_live_cutover_ready == true or $controlled_live_required_evidence_readback.live_execution_allowed == true or $controlled_live_required_evidence_readback.side_effects.approval_requested == true or $controlled_live_required_evidence_readback.side_effects.approval_accepted == true or $controlled_live_required_evidence_readback.side_effects.evidence_recorded == true or $controlled_live_required_evidence_readback.side_effects.evidence_persisted == true or $controlled_live_required_evidence_readback.side_effects.blocker_waived == true or $controlled_live_required_evidence_readback.side_effects.credential_read == true or $controlled_live_required_evidence_readback.side_effects.readback_persisted == true or $controlled_live_required_evidence_readback.side_effects.native_post_mutation_performed == true or $controlled_live_required_evidence_readback.side_effects.gateway_or_auth_mutated == true or $controlled_live_required_evidence_readback.side_effects.telegram_transport_mutated == true or $controlled_live_required_evidence_readback.side_effects.channel_send_performed == true or $controlled_live_required_evidence_readback.side_effects.live_execution_started == true),
      source:"scripts/hepta-systems-controlled-live-required-evidence-readback-index-report.sh",
      current_fact:"controlled-live required evidence readback index makes seven evidence requirements queryable and diffable while evidence recording, credential reads, approval acceptance, blocker waiver, persistence, and live remain disabled"
    },
    {
      id:"controlled_live_required_evidence_gap_summary",
      layer:"live_readiness",
      status:$controlled_live_required_evidence_gap_summary.status,
      ready:($controlled_live_required_evidence_gap_summary.gap_summary_ready == true and $controlled_live_required_evidence_gap_summary.missing_evidence_count == 7 and $controlled_live_required_evidence_gap_summary.evidence_recorded_count == 0 and $controlled_live_required_evidence_gap_summary.approval_accepted == false and $controlled_live_required_evidence_gap_summary.blocker_waived_count == 0 and $controlled_live_required_evidence_gap_summary.credential_read_allowed == false and $controlled_live_required_evidence_gap_summary.evidence_recording_allowed == false and $controlled_live_required_evidence_gap_summary.evidence_persisted == false and $controlled_live_required_evidence_gap_summary.controlled_live_cutover_ready == false),
      live_enabled:($controlled_live_required_evidence_gap_summary.approval_acceptance_ready == true or $controlled_live_required_evidence_gap_summary.approval_accepted == true or $controlled_live_required_evidence_gap_summary.blocker_waived_count > 0 or $controlled_live_required_evidence_gap_summary.credential_read_allowed == true or $controlled_live_required_evidence_gap_summary.evidence_recording_allowed == true or $controlled_live_required_evidence_gap_summary.evidence_persisted == true or $controlled_live_required_evidence_gap_summary.controlled_live_cutover_ready == true or $controlled_live_required_evidence_gap_summary.live_execution_allowed == true or $controlled_live_required_evidence_gap_summary.side_effects.approval_requested == true or $controlled_live_required_evidence_gap_summary.side_effects.approval_accepted == true or $controlled_live_required_evidence_gap_summary.side_effects.evidence_recorded == true or $controlled_live_required_evidence_gap_summary.side_effects.evidence_persisted == true or $controlled_live_required_evidence_gap_summary.side_effects.blocker_waived == true or $controlled_live_required_evidence_gap_summary.side_effects.credential_read == true or $controlled_live_required_evidence_gap_summary.side_effects.readback_persisted == true or $controlled_live_required_evidence_gap_summary.side_effects.native_post_mutation_performed == true or $controlled_live_required_evidence_gap_summary.side_effects.gateway_or_auth_mutated == true or $controlled_live_required_evidence_gap_summary.side_effects.telegram_transport_mutated == true or $controlled_live_required_evidence_gap_summary.side_effects.channel_send_performed == true or $controlled_live_required_evidence_gap_summary.side_effects.live_execution_started == true),
      source:"scripts/hepta-systems-controlled-live-required-evidence-gap-summary-report.sh",
      current_fact:"controlled-live required evidence gap summary groups seven missing evidence gaps by owner and cutover risk while acceptance, recording, credential reads, waiver, persistence, and live remain disabled"
    },
    {
      id:"controlled_live_required_evidence_gap_diff_view",
      layer:"live_readiness",
      status:$controlled_live_required_evidence_gap_diff.status,
      ready:($controlled_live_required_evidence_gap_diff.diff_view_ready == true and $controlled_live_required_evidence_gap_diff.unchanged_missing_count == 7 and $controlled_live_required_evidence_gap_diff.evidence_recorded_count == 0 and $controlled_live_required_evidence_gap_diff.approval_accepted == false and $controlled_live_required_evidence_gap_diff.blocker_waived_count == 0 and $controlled_live_required_evidence_gap_diff.credential_read_allowed == false and $controlled_live_required_evidence_gap_diff.evidence_recording_allowed == false and $controlled_live_required_evidence_gap_diff.evidence_persisted == false and $controlled_live_required_evidence_gap_diff.controlled_live_cutover_ready == false),
      live_enabled:($controlled_live_required_evidence_gap_diff.approval_acceptance_ready == true or $controlled_live_required_evidence_gap_diff.approval_accepted == true or $controlled_live_required_evidence_gap_diff.blocker_waived_count > 0 or $controlled_live_required_evidence_gap_diff.credential_read_allowed == true or $controlled_live_required_evidence_gap_diff.evidence_recording_allowed == true or $controlled_live_required_evidence_gap_diff.evidence_persisted == true or $controlled_live_required_evidence_gap_diff.controlled_live_cutover_ready == true or $controlled_live_required_evidence_gap_diff.live_execution_allowed == true or $controlled_live_required_evidence_gap_diff.side_effects.approval_requested == true or $controlled_live_required_evidence_gap_diff.side_effects.approval_accepted == true or $controlled_live_required_evidence_gap_diff.side_effects.evidence_recorded == true or $controlled_live_required_evidence_gap_diff.side_effects.evidence_persisted == true or $controlled_live_required_evidence_gap_diff.side_effects.blocker_waived == true or $controlled_live_required_evidence_gap_diff.side_effects.credential_read == true or $controlled_live_required_evidence_gap_diff.side_effects.readback_persisted == true or $controlled_live_required_evidence_gap_diff.side_effects.native_post_mutation_performed == true or $controlled_live_required_evidence_gap_diff.side_effects.gateway_or_auth_mutated == true or $controlled_live_required_evidence_gap_diff.side_effects.telegram_transport_mutated == true or $controlled_live_required_evidence_gap_diff.side_effects.channel_send_performed == true or $controlled_live_required_evidence_gap_diff.side_effects.live_execution_started == true),
      source:"scripts/hepta-systems-controlled-live-required-evidence-gap-diff-view-report.sh",
      current_fact:"controlled-live required evidence gap diff view keeps seven missing evidence gaps comparable across readbacks while acceptance, recording, credential reads, waiver, persistence, and live remain disabled"
    },
    {
      id:"controlled_live_required_evidence_gap_operator_readback",
      layer:"live_readiness",
      status:$controlled_live_required_evidence_gap_operator_readback.status,
      ready:($controlled_live_required_evidence_gap_operator_readback.operator_readback_ready == true and $controlled_live_required_evidence_gap_operator_readback.unchanged_missing_count == 7 and $controlled_live_required_evidence_gap_operator_readback.evidence_recorded_count == 0 and $controlled_live_required_evidence_gap_operator_readback.approval_accepted == false and $controlled_live_required_evidence_gap_operator_readback.blocker_waived_count == 0 and $controlled_live_required_evidence_gap_operator_readback.credential_read_allowed == false and $controlled_live_required_evidence_gap_operator_readback.evidence_recording_allowed == false and $controlled_live_required_evidence_gap_operator_readback.evidence_persisted == false and $controlled_live_required_evidence_gap_operator_readback.readback_persisted == false and $controlled_live_required_evidence_gap_operator_readback.controlled_live_cutover_ready == false),
      live_enabled:($controlled_live_required_evidence_gap_operator_readback.approval_acceptance_ready == true or $controlled_live_required_evidence_gap_operator_readback.approval_accepted == true or $controlled_live_required_evidence_gap_operator_readback.blocker_waived_count > 0 or $controlled_live_required_evidence_gap_operator_readback.credential_read_allowed == true or $controlled_live_required_evidence_gap_operator_readback.evidence_recording_allowed == true or $controlled_live_required_evidence_gap_operator_readback.evidence_persisted == true or $controlled_live_required_evidence_gap_operator_readback.readback_persisted == true or $controlled_live_required_evidence_gap_operator_readback.controlled_live_cutover_ready == true or $controlled_live_required_evidence_gap_operator_readback.live_execution_allowed == true or $controlled_live_required_evidence_gap_operator_readback.side_effects.approval_requested == true or $controlled_live_required_evidence_gap_operator_readback.side_effects.approval_accepted == true or $controlled_live_required_evidence_gap_operator_readback.side_effects.evidence_recorded == true or $controlled_live_required_evidence_gap_operator_readback.side_effects.evidence_persisted == true or $controlled_live_required_evidence_gap_operator_readback.side_effects.blocker_waived == true or $controlled_live_required_evidence_gap_operator_readback.side_effects.credential_read == true or $controlled_live_required_evidence_gap_operator_readback.side_effects.readback_persisted == true or $controlled_live_required_evidence_gap_operator_readback.side_effects.native_post_mutation_performed == true or $controlled_live_required_evidence_gap_operator_readback.side_effects.gateway_or_auth_mutated == true or $controlled_live_required_evidence_gap_operator_readback.side_effects.telegram_transport_mutated == true or $controlled_live_required_evidence_gap_operator_readback.side_effects.channel_send_performed == true or $controlled_live_required_evidence_gap_operator_readback.side_effects.live_execution_started == true),
      source:"scripts/hepta-systems-controlled-live-required-evidence-gap-operator-readback-report.sh",
      current_fact:"controlled-live required evidence gap operator readback presents seven unchanged missing evidence gaps with stable operator readback routes while acceptance, recording, credential reads, waiver, persistence, and live remain disabled"
    },
    {
      id:"controlled_live_required_evidence_gap_operator_packet_attachment",
      layer:"live_readiness",
      status:$controlled_live_required_evidence_gap_operator_packet_attachment.status,
      ready:($controlled_live_required_evidence_gap_operator_packet_attachment.operator_packet_attachment_ready == true and $controlled_live_required_evidence_gap_operator_packet_attachment.unchanged_missing_attachment_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment.evidence_recorded_count == 0 and $controlled_live_required_evidence_gap_operator_packet_attachment.approval_request_sent == false and $controlled_live_required_evidence_gap_operator_packet_attachment.approval_accepted == false and $controlled_live_required_evidence_gap_operator_packet_attachment.blocker_waived_count == 0 and $controlled_live_required_evidence_gap_operator_packet_attachment.credential_read_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment.evidence_recording_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment.evidence_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment.packet_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment.attachment_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment.readback_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment.controlled_live_cutover_ready == false),
      live_enabled:($controlled_live_required_evidence_gap_operator_packet_attachment.approval_request_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment.approval_request_sent == true or $controlled_live_required_evidence_gap_operator_packet_attachment.approval_acceptance_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment.approval_accepted == true or $controlled_live_required_evidence_gap_operator_packet_attachment.approval_recorded == true or $controlled_live_required_evidence_gap_operator_packet_attachment.blocker_waived_count > 0 or $controlled_live_required_evidence_gap_operator_packet_attachment.credential_read_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment.evidence_recording_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment.evidence_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment.packet_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment.attachment_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment.readback_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment.controlled_live_cutover_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment.live_execution_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment.side_effects.approval_requested == true or $controlled_live_required_evidence_gap_operator_packet_attachment.side_effects.approval_accepted == true or $controlled_live_required_evidence_gap_operator_packet_attachment.side_effects.evidence_recorded == true or $controlled_live_required_evidence_gap_operator_packet_attachment.side_effects.evidence_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment.side_effects.blocker_waived == true or $controlled_live_required_evidence_gap_operator_packet_attachment.side_effects.credential_read == true or $controlled_live_required_evidence_gap_operator_packet_attachment.side_effects.packet_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment.side_effects.attachment_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment.side_effects.readback_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment.side_effects.native_post_mutation_performed == true or $controlled_live_required_evidence_gap_operator_packet_attachment.side_effects.gateway_or_auth_mutated == true or $controlled_live_required_evidence_gap_operator_packet_attachment.side_effects.telegram_transport_mutated == true or $controlled_live_required_evidence_gap_operator_packet_attachment.side_effects.channel_send_performed == true or $controlled_live_required_evidence_gap_operator_packet_attachment.side_effects.live_execution_started == true),
      source:"scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-report.sh",
      current_fact:"controlled-live required evidence gap operator packet attachment attaches seven unchanged missing operator readbacks to the local packet preview while approval, sending, evidence recording, packet/attachment persistence, credential reads, waiver, and live remain disabled"
    },
    {
      id:"controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback",
      layer:"live_readiness",
      status:$controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.status,
      ready:($controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.non_send_readback_ready == true and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.readback_ready_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.unchanged_missing_readback_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.attachment_send_attempted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.packet_send_attempted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.approval_request_sent == false and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.approval_accepted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.blocker_waived_count == 0 and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.credential_read_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.evidence_recording_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.evidence_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.packet_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.attachment_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.readback_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.controlled_live_cutover_ready == false),
      live_enabled:($controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.approval_request_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.approval_request_sent == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.approval_acceptance_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.approval_accepted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.approval_recorded == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.attachment_send_attempted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.packet_send_attempted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.blocker_waived_count > 0 or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.credential_read_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.evidence_recording_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.evidence_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.packet_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.attachment_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.readback_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.controlled_live_cutover_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.live_execution_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.side_effects.approval_requested == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.side_effects.approval_accepted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.side_effects.evidence_recorded == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.side_effects.evidence_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.side_effects.blocker_waived == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.side_effects.credential_read == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.side_effects.packet_sent == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.side_effects.attachment_sent == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.side_effects.packet_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.side_effects.attachment_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.side_effects.readback_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.side_effects.native_post_mutation_performed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.side_effects.gateway_or_auth_mutated == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.side_effects.telegram_transport_mutated == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.side_effects.channel_send_performed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.side_effects.live_execution_started == true),
      source:"scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-non-send-readback-report.sh",
      current_fact:"controlled-live required evidence gap operator packet attachment non-send readback proves seven attached readbacks are visible, unsent, unpersisted, and not an approval request while approval, evidence recording, credential reads, waiver, transport, persistence, and live remain disabled"
    },
    {
      id:"controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback",
      layer:"live_readiness",
      status:$controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.status,
      ready:($controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.transport_boundary_readback_ready == true and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.transport_boundary_ready_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.gateway_auth_boundary_closed_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.native_post_boundary_closed_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.telegram_transport_boundary_closed_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.channel_send_boundary_closed_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.evidence_recorded_count == 0 and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.blocker_waived_count == 0 and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.packet_send_attempted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.attachment_send_attempted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.approval_request_sent == false and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.approval_accepted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.credential_read_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.gateway_or_auth_mutation_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.native_post_mutation_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.telegram_transport_mutation_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.channel_send_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.transport_mutation_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.packet_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.attachment_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.readback_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.controlled_live_cutover_ready == false),
      live_enabled:($controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.approval_request_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.approval_request_sent == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.approval_acceptance_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.approval_accepted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.approval_recorded == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.packet_send_attempted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.attachment_send_attempted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.blocker_waived_count > 0 or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.credential_read_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.evidence_recording_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.evidence_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.packet_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.attachment_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.readback_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.gateway_or_auth_mutation_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.native_post_mutation_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.telegram_transport_mutation_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.channel_send_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.transport_mutation_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.controlled_live_cutover_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.live_execution_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.side_effects.approval_requested == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.side_effects.approval_accepted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.side_effects.evidence_recorded == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.side_effects.evidence_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.side_effects.blocker_waived == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.side_effects.credential_read == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.side_effects.packet_sent == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.side_effects.attachment_sent == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.side_effects.packet_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.side_effects.attachment_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.side_effects.readback_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.side_effects.native_post_mutation_performed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.side_effects.gateway_or_auth_mutated == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.side_effects.telegram_transport_mutated == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.side_effects.channel_send_performed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.side_effects.live_execution_started == true),
      source:"scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-transport-boundary-readback-report.sh",
      current_fact:"controlled-live required evidence gap operator packet attachment transport boundary readback makes Gateway/Auth, Native POST, Telegram transport, and channel send closed boundaries operator-visible while approval, evidence recording, credential reads, waiver, transport mutation, persistence, and live remain disabled"
    },
    {
      id:"controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary_readback",
      layer:"live_readiness",
      status:$controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.status,
      ready:($controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_boundary_readback_ready == true and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_boundary_ready_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_boundary_closed_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_read_blocked_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_material_load_blocked_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_value_exposure_blocked_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_handle_resolution_blocked_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_attestation_missing_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.evidence_recorded_count == 0 and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.blocker_waived_count == 0 and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.packet_send_attempted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.attachment_send_attempted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.approval_request_sent == false and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.approval_accepted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_read_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_material_load_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_value_exposure_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_handle_resolution_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.gateway_or_auth_mutation_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.native_post_mutation_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.telegram_transport_mutation_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.channel_send_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.transport_mutation_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.packet_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.attachment_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.readback_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.controlled_live_cutover_ready == false),
      live_enabled:($controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.approval_request_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.approval_request_sent == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.approval_acceptance_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.approval_accepted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.approval_recorded == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.packet_send_attempted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.attachment_send_attempted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.blocker_waived_count > 0 or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_read_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_material_load_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_value_exposure_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_handle_resolution_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.evidence_recording_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.evidence_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.packet_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.attachment_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.readback_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.gateway_or_auth_mutation_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.native_post_mutation_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.telegram_transport_mutation_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.channel_send_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.transport_mutation_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.controlled_live_cutover_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.live_execution_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.approval_requested == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.approval_accepted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.evidence_recorded == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.evidence_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.blocker_waived == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.credential_read == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.credential_material_loaded == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.credential_value_exposed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.credential_handle_resolved == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.packet_sent == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.attachment_sent == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.packet_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.attachment_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.readback_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.native_post_mutation_performed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.gateway_or_auth_mutated == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.telegram_transport_mutated == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.channel_send_performed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.side_effects.live_execution_started == true),
      source:"scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-credential-boundary-readback-report.sh",
      current_fact:"controlled-live required evidence gap operator packet attachment credential boundary readback makes credential reads, material loads, value exposure, and handle resolution closed and operator-visible while approval, evidence recording, waiver, transport mutation, persistence, and live remain disabled"
    },
    {
      id:"controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback",
      layer:"live_readiness",
      status:$controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.status,
      ready:($controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_boundary_readback_ready == true and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_boundary_ready_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_boundary_closed_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_execution_blocked_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_execution_blocked_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_recording_blocked_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_receipt_persistence_blocked_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_evidence_missing_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.evidence_recorded_count == 0 and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.blocker_waived_count == 0 and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.packet_send_attempted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.attachment_send_attempted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.approval_request_sent == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.approval_accepted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.credential_read_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_executed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_execution_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_executed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_recording_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_receipt_persistence_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.gateway_or_auth_mutation_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.native_post_mutation_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.telegram_transport_mutation_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.channel_send_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.transport_mutation_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.packet_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.attachment_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.readback_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.controlled_live_cutover_ready == false),
      live_enabled:($controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.approval_request_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.approval_request_sent == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.approval_acceptance_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.approval_accepted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.approval_recorded == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.packet_send_attempted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.attachment_send_attempted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.blocker_waived_count > 0 or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.credential_read_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_executed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_execution_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_executed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_recording_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_receipt_persistence_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.evidence_recording_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.evidence_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.packet_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.attachment_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.readback_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.gateway_or_auth_mutation_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.native_post_mutation_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.telegram_transport_mutation_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.channel_send_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.transport_mutation_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.controlled_live_cutover_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.live_execution_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.approval_requested == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.approval_accepted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.evidence_recorded == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.evidence_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.blocker_waived == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.credential_read == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.rollback_rehearsal_executed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.rollback_executed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.rollback_rehearsal_recorded == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.rollback_rehearsal_receipt_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.packet_sent == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.attachment_sent == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.packet_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.attachment_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.readback_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.native_post_mutation_performed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.gateway_or_auth_mutated == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.telegram_transport_mutated == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.channel_send_performed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.side_effects.live_execution_started == true),
      source:"scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-rollback-rehearsal-boundary-readback-report.sh",
      current_fact:"controlled-live required evidence gap operator packet attachment rollback rehearsal boundary readback makes rollback rehearsal execution, rollback execution, rehearsal recording, and rehearsal receipt persistence closed and operator-visible while approval, credential reads, evidence recording, waiver, transport mutation, persistence, and live remain disabled"
    },
    {
      id:"controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback",
      layer:"live_readiness",
      status:$controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.status,
      ready:($controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_boundary_readback_ready == true and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_boundary_ready_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_boundary_closed_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_execution_blocked_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_mutation_blocked_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_recording_blocked_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_receipt_persistence_blocked_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_evidence_missing_count == 7 and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.evidence_recorded_count == 0 and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.blocker_waived_count == 0 and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.packet_send_attempted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.attachment_send_attempted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.approval_request_sent == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.approval_accepted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.credential_read_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.rollback_rehearsal_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.rollback_rehearsal_executed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.rollback_execution_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.rollback_executed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_executed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_mutation_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_mutated == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_recording_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_receipt_persistence_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.gateway_or_auth_mutation_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.native_post_mutation_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.telegram_transport_mutation_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.channel_send_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.transport_mutation_allowed == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.packet_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.attachment_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.readback_persisted == false and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.controlled_live_cutover_ready == false),
      live_enabled:($controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.approval_request_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.approval_request_sent == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.approval_acceptance_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.approval_accepted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.approval_recorded == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.packet_send_attempted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.attachment_send_attempted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.blocker_waived_count > 0 or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.credential_read_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.rollback_rehearsal_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.rollback_rehearsal_executed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.rollback_execution_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.rollback_executed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_executed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_mutation_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_mutated == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_recording_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_receipt_persistence_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.evidence_recording_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.evidence_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.packet_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.attachment_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.readback_persisted == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.gateway_or_auth_mutation_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.native_post_mutation_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.telegram_transport_mutation_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.channel_send_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.transport_mutation_allowed == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.controlled_live_cutover_ready == true or $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.live_execution_allowed == true or ($controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.side_effects | to_entries | any(.value == true))),
      source:"scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-kill-switch-rehearsal-boundary-readback-report.sh",
      current_fact:"controlled-live required evidence gap operator packet attachment kill-switch rehearsal boundary readback makes kill-switch rehearsal execution, kill-switch mutation, rehearsal recording, and rehearsal receipt persistence closed and operator-visible while rollback rehearsal, approval, credential reads, evidence recording, waiver, transport mutation, persistence, and live remain disabled"
    },
    {
      id:"current_compact_capability_summary",
      layer:"current_summary",
      status:$current_summary.status,
      ready:($current_summary.compact_capability_summary_ready == true and $current_summary.execution_enabled_count == 0 and $current_summary.public_ga_enabled_count == 0),
      live_enabled:($current_summary.tool_execution_live_cutover_allowed == true or $current_summary.tool_execution_public_ga_allowed == true),
      source:"scripts/hepta-systems-current-compact-capability-summary-report.sh",
      current_fact:"existing current compact summary is ready and keeps execution and Public GA disabled"
    }
  ] as $capabilities |
  [
    {
      id:"memory_plugin_lifecycle_state_machine",
      path:"codex-rs/core-plugins/src/lifecycle_state_machine.rs",
      present:$plugin_lifecycle_state_machine_present,
      expected_from_memory:true,
      resolution:(if $plugin_lifecycle_state_machine_present then "phase1_restored_as_plugin_lifecycle_source_of_truth" else "phase1_restore_or_explicitly_supersede_with_contribution_point_abi" end)
    },
    {
      id:"memory_plugin_lifecycle_phase_summary",
      path:"codex-rs/core-plugins/src/lifecycle_phase_summary.rs",
      present:$plugin_lifecycle_phase_summary_present,
      expected_from_memory:true,
      resolution:(if $plugin_lifecycle_phase_summary_present then "phase1_restored_as_lifecycle_phase_read_model" else "phase1_restore_or_explicitly_supersede_with_current_plugin_abi_report" end)
    },
    {
      id:"memory_workflow_durable_store_adapter_absent",
      path:"codex-rs/hepta-runtime/src/workflow_durable_store_adapter.rs",
      present:$workflow_durable_store_adapter_present,
      expected_from_memory:true,
      resolution:(if $workflow_durable_store_adapter_present then "phase3_restored_temporal_lite_adapter_contract" else "phase3_rebuild_temporal_lite_adapter_contract" end)
    },
    {
      id:"memory_workflow_durable_store_append_plan_absent",
      path:"codex-rs/hepta-runtime/src/workflow_durable_store_append_plan.rs",
      present:$workflow_durable_store_append_plan_present,
      expected_from_memory:true,
      resolution:(if $workflow_durable_store_append_plan_present then "phase3_restored_append_only_event_log_plan" else "phase3_rebuild_append_only_event_log_plan" end)
    },
    {
      id:"memory_workflow_durable_store_harness_absent",
      path:"codex-rs/hepta-runtime/src/workflow_durable_store_adapter_harness.rs",
      present:$workflow_durable_store_adapter_harness_present,
      expected_from_memory:true,
      resolution:(if $workflow_durable_store_adapter_harness_present then "phase3_restored_noop_adapter_harness_before_live_cutover" else "phase3_rebuild_noop_adapter_harness_before_live_cutover" end)
    }
  ] as $memory_drift_entries |
  ($memory_drift_entries | map(select(.expected_from_memory == true and .present == false)) | length) as $missing_memory_checkpoint_count |
  ($memory_drift_entries | map(select(.expected_from_memory == true and .present == true)) | length) as $resolved_memory_checkpoint_count |
  ($capabilities | map(select(.ready == true)) | length) as $ready_count |
  ($capabilities | map(select(.ready != true)) | length) as $blocked_count |
  ($capabilities | map(select(.live_enabled == true)) | length) as $live_enabled_count |
  ($plugin_manifest_present
    and $plugin_manifest_summary.skill_path_present == true
    and $plugin_manifest_summary.mcp_servers_path_present == true
    and $plugin_manifest_summary.apps_path_present == true
    and $plugin_manifest_summary.skill_count == 1
    and $plugin_manifest_summary.mcp_server_count == 1
    and $plugin_manifest_summary.app_count == 1
    and $plugin_manifest_summary.tool_schema_count == 2
    and $plugin_manifest_summary.permission_count == 2
    and $plugin_manifest_summary.activation_event_count == 2
    and $plugin_manifest_summary.tool_policy_count == 2) as $plugin_fixture_shape_ready |
  ($ready_count == ($capabilities | length)
    and $blocked_count == 0
    and $live_enabled_count == 0
    and $missing_memory_checkpoint_count == 0
    and $resolved_memory_checkpoint_count == 5
    and $plugin_lifecycle.lifecycle_state_machine_ready == true
    and $tool_read_only_dispatch.read_only_dispatch_preflight_ready == true
    and $workflow_durable_adapter.temporal_lite_adapter_ready == true
    and $hepta_status_e2e.read_only_e2e_ready == true
    and $hepta_status_internal_read_only_invocation.internal_read_only_invocation_ready == true
    and $hepta_status_internal_read_only_invocation.invocation_entry_count == 1
    and $hepta_status_internal_read_only_invocation.candidate_count == 2
    and $hepta_status_internal_read_only_invocation.status_payload_materialized == true
    and $hepta_status_internal_read_only_invocation.receipt_projected_in_memory == true
    and $hepta_status_internal_read_only_invocation.receipt_persisted == false
    and $hepta_status_internal_read_only_invocation.external_network_allowed == false
    and $hepta_status_internal_read_only_invocation.credential_read_allowed == false
    and $hepta_status_internal_read_only_invocation.external_tool_invoked == false
    and $hepta_status_internal_read_only_invocation.tool_invocation_switch_enabled == false
    and $hepta_status_internal_read_only_invocation.ledger_write_allowed == false
    and $hepta_status_internal_read_only_invocation.approval_request_allowed == false
    and $hepta_status_internal_read_only_invocation.approval_acceptance_allowed == false
    and $hepta_status_internal_read_only_invocation.workflow_event_log_write_allowed == false
    and $hepta_status_internal_read_only_invocation.sqlite_write_allowed == false
    and $hepta_status_internal_read_only_invocation.native_post_mutation_allowed == false
    and $hepta_status_internal_read_only_invocation.channel_send_allowed == false
    and $hepta_status_internal_read_only_invocation.live_execution_allowed == false
    and $hepta_status_operator_approval_protocol.approval_protocol_ready == true
    and $hepta_status_operator_approval_protocol.approval_packet_count == 1
    and $hepta_status_operator_approval_protocol.protocol_step_count == 3
    and $hepta_status_operator_approval_protocol.nonce_binding_present == true
    and $hepta_status_operator_approval_protocol.session_binding_present == true
    and $hepta_status_operator_approval_protocol.explicit_accept_required == true
    and $hepta_status_operator_approval_protocol.non_acceptance_receipt_projected == true
    and $hepta_status_operator_approval_protocol.approval_request_sent == false
    and $hepta_status_operator_approval_protocol.approval_request_allowed == false
    and $hepta_status_operator_approval_protocol.approval_accepted == false
    and $hepta_status_operator_approval_protocol.approval_acceptance_allowed == false
    and $hepta_status_operator_approval_protocol.approval_recorded == false
    and $hepta_status_operator_approval_protocol.approval_recording_allowed == false
    and $hepta_status_operator_approval_protocol.auto_approval_enabled == false
    and $hepta_status_operator_approval_protocol.evidence_recording_allowed == false
    and $hepta_status_operator_approval_protocol.approval_broker_write_allowed == false
    and $hepta_status_operator_approval_protocol.approval_broker_persisted == false
    and $hepta_status_operator_approval_protocol.receipt_persisted == false
    and $hepta_status_operator_approval_protocol.credential_read_allowed == false
    and $hepta_status_operator_approval_protocol.external_network_allowed == false
    and $hepta_status_operator_approval_protocol.external_tool_invoked == false
    and $hepta_status_operator_approval_protocol.tool_invocation_switch_enabled == false
    and $hepta_status_operator_approval_protocol.ledger_write_allowed == false
    and $hepta_status_operator_approval_protocol.workflow_event_log_write_allowed == false
    and $hepta_status_operator_approval_protocol.sqlite_write_allowed == false
    and $hepta_status_operator_approval_protocol.transport_mutation_allowed == false
    and $hepta_status_operator_approval_protocol.native_post_mutation_allowed == false
    and $hepta_status_operator_approval_protocol.channel_send_allowed == false
    and $hepta_status_operator_approval_protocol.live_execution_allowed == false
    and $controlled_live_readiness.controlled_live_audit_ready == true
    and $controlled_live_readiness.controlled_live_cutover_ready == false
    and $controlled_live_denial_readback.readback_index_ready == true
    and $controlled_live_denial_readback.controlled_live_cutover_ready == false
    and $controlled_live_operator_packet.operator_packet_preview_ready == true
    and $controlled_live_operator_packet.approval_request_sent == false
    and $controlled_live_operator_packet.controlled_live_cutover_ready == false
    and $controlled_live_operator_packet_nonsend.non_send_readback_ready == true
    and $controlled_live_operator_packet_nonsend.packet_send_attempted == false
    and $controlled_live_operator_packet_nonsend.approval_request_sent == false
    and $controlled_live_operator_packet_nonsend.packet_persisted == false
    and $controlled_live_required_evidence_plan.evidence_collection_plan_ready == true
    and $controlled_live_required_evidence_plan.evidence_recorded_count == 0
    and $controlled_live_required_evidence_plan.approval_accepted == false
    and $controlled_live_required_evidence_plan.evidence_persisted == false
    and $controlled_live_required_evidence_readback.readback_index_ready == true
    and $controlled_live_required_evidence_readback.evidence_recorded_count == 0
    and $controlled_live_required_evidence_readback.approval_accepted == false
    and $controlled_live_required_evidence_readback.evidence_persisted == false
    and $controlled_live_required_evidence_gap_summary.gap_summary_ready == true
    and $controlled_live_required_evidence_gap_summary.missing_evidence_count == 7
    and $controlled_live_required_evidence_gap_summary.evidence_recorded_count == 0
    and $controlled_live_required_evidence_gap_summary.approval_accepted == false
    and $controlled_live_required_evidence_gap_summary.evidence_persisted == false
    and $controlled_live_required_evidence_gap_diff.diff_view_ready == true
    and $controlled_live_required_evidence_gap_diff.unchanged_missing_count == 7
    and $controlled_live_required_evidence_gap_diff.evidence_recorded_count == 0
    and $controlled_live_required_evidence_gap_diff.approval_accepted == false
    and $controlled_live_required_evidence_gap_diff.evidence_persisted == false
    and $controlled_live_required_evidence_gap_operator_readback.operator_readback_ready == true
    and $controlled_live_required_evidence_gap_operator_readback.unchanged_missing_count == 7
    and $controlled_live_required_evidence_gap_operator_readback.evidence_recorded_count == 0
    and $controlled_live_required_evidence_gap_operator_readback.approval_accepted == false
    and $controlled_live_required_evidence_gap_operator_readback.evidence_persisted == false
    and $controlled_live_required_evidence_gap_operator_readback.readback_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment.operator_packet_attachment_ready == true
    and $controlled_live_required_evidence_gap_operator_packet_attachment.unchanged_missing_attachment_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment.evidence_recorded_count == 0
    and $controlled_live_required_evidence_gap_operator_packet_attachment.approval_request_sent == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment.approval_accepted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment.packet_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment.attachment_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment.readback_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.non_send_readback_ready == true
    and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.readback_ready_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.unchanged_missing_readback_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.attachment_send_attempted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.packet_send_attempted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.approval_request_sent == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.approval_accepted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.packet_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.attachment_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_nonsend.readback_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.transport_boundary_readback_ready == true
    and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.transport_boundary_ready_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.gateway_auth_boundary_closed_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.native_post_boundary_closed_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.telegram_transport_boundary_closed_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.channel_send_boundary_closed_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.approval_request_sent == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.approval_accepted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.gateway_or_auth_mutation_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.native_post_mutation_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.telegram_transport_mutation_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.channel_send_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.transport_mutation_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.packet_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.attachment_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary.readback_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_boundary_readback_ready == true
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_boundary_ready_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_boundary_closed_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_read_blocked_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_material_load_blocked_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_value_exposure_blocked_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_handle_resolution_blocked_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_attestation_missing_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.approval_request_sent == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.approval_accepted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_read_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_material_load_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_value_exposure_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.credential_handle_resolution_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.evidence_recording_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.gateway_or_auth_mutation_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.transport_mutation_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.packet_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.attachment_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary.readback_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_boundary_readback_ready == true
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_boundary_ready_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_boundary_closed_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_execution_blocked_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_execution_blocked_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_recording_blocked_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_receipt_persistence_blocked_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_evidence_missing_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.approval_request_sent == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.approval_accepted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.credential_read_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_executed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_execution_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_executed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_recording_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.rollback_rehearsal_receipt_persistence_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.evidence_recording_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.gateway_or_auth_mutation_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.transport_mutation_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.packet_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.attachment_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary.readback_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_boundary_readback_ready == true
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_boundary_ready_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_boundary_closed_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_execution_blocked_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_mutation_blocked_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_recording_blocked_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_receipt_persistence_blocked_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_evidence_missing_count == 7
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.approval_request_sent == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.approval_accepted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.credential_read_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.rollback_rehearsal_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.rollback_rehearsal_executed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.rollback_execution_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.rollback_executed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_executed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_mutation_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_mutated == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_recording_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.kill_switch_rehearsal_receipt_persistence_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.evidence_recording_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.gateway_or_auth_mutation_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.transport_mutation_allowed == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.packet_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.attachment_persisted == false
    and $controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary.readback_persisted == false
    and $controlled_canary_readiness_plan.controlled_canary_readiness_plan_ready == true
    and $controlled_canary_readiness_plan.controlled_canary_activation_ready == false
    and $controlled_canary_readiness_plan.canary_plan_entry_count == 7
    and $controlled_canary_readiness_plan.canary_plan_ready_count == 7
    and $controlled_canary_readiness_plan.missing_blocker_count == 7
    and $controlled_canary_readiness_plan.dirty_worktree_blocker_preserved == true
    and $controlled_canary_readiness_plan.soak_readback_required == true
    and $controlled_canary_readiness_plan.rollback_rehearsal_required == true
    and $controlled_canary_readiness_plan.kill_switch_rehearsal_required == true
    and $controlled_canary_readiness_plan.gateway_native_telegram_boundary_closed == true
    and $controlled_canary_readiness_plan.credential_boundary_closed == true
    and $controlled_canary_readiness_plan.persistence_boundary_closed == true
    and $controlled_canary_readiness_plan.approval_request_sent == false
    and $controlled_canary_readiness_plan.approval_request_allowed == false
    and $controlled_canary_readiness_plan.approval_accepted == false
    and $controlled_canary_readiness_plan.approval_recorded == false
    and $controlled_canary_readiness_plan.approval_broker_write_allowed == false
    and $controlled_canary_readiness_plan.evidence_recording_allowed == false
    and $controlled_canary_readiness_plan.credential_read_allowed == false
    and $controlled_canary_readiness_plan.gateway_or_auth_mutation_allowed == false
    and $controlled_canary_readiness_plan.native_post_mutation_allowed == false
    and $controlled_canary_readiness_plan.telegram_transport_mutation_allowed == false
    and $controlled_canary_readiness_plan.channel_send_allowed == false
    and $controlled_canary_readiness_plan.transport_mutation_allowed == false
    and $controlled_canary_readiness_plan.canary_persistence_allowed == false
    and $controlled_canary_readiness_plan.canary_receipt_persisted == false
    and $controlled_canary_readiness_plan.workflow_event_log_write_allowed == false
    and $controlled_canary_readiness_plan.sqlite_write_allowed == false
    and $controlled_canary_readiness_plan.provider_invocation_allowed == false
    and $controlled_canary_readiness_plan.model_invocation_allowed == false
    and $controlled_canary_readiness_plan.package_or_release_allowed == false
    and $controlled_canary_readiness_plan.public_ga_allowed == false
    and $controlled_canary_readiness_plan.live_activation_allowed == false
    and $controlled_canary_readiness_plan.live_execution_allowed == false
    and $dirty_worktree_release_boundary_inventory.release_boundary_inventory_ready == true
    and $dirty_worktree_release_boundary_inventory.dirty_worktree_release_boundary_open == true
    and $dirty_worktree_release_boundary_inventory.dirty_worktree_release_boundary_resolved == false
    and $dirty_worktree_release_boundary_inventory.inventory_entry_count > 0
    and $dirty_worktree_release_boundary_inventory.inventory_entry_count == ($dirty_worktree_release_boundary_inventory.tracked_change_count + $dirty_worktree_release_boundary_inventory.untracked_change_count)
    and $dirty_worktree_release_boundary_inventory.inventory_entry_count == ($dirty_worktree_release_boundary_inventory.hepta_systems_owned_count + $dirty_worktree_release_boundary_inventory.cross_lane_or_unowned_count)
    and $dirty_worktree_release_boundary_inventory.top_level_bucket_count > 0
    and $dirty_worktree_release_boundary_inventory.scope_bucket_count > 0
    and $dirty_worktree_release_boundary_inventory.release_cutover_allowed == false
    and $dirty_worktree_release_boundary_inventory.git_add_allowed == false
    and $dirty_worktree_release_boundary_inventory.git_index_mutated == false
    and $dirty_worktree_release_boundary_inventory.git_commit_allowed == false
    and $dirty_worktree_release_boundary_inventory.git_push_allowed == false
    and $dirty_worktree_release_boundary_inventory.git_reset_allowed == false
    and $dirty_worktree_release_boundary_inventory.git_checkout_allowed == false
    and $dirty_worktree_release_boundary_inventory.git_revert_allowed == false
    and $dirty_worktree_release_boundary_inventory.cleanup_allowed == false
    and $dirty_worktree_release_boundary_inventory.delete_allowed == false
    and $dirty_worktree_release_boundary_inventory.evidence_recording_allowed == false
    and $dirty_worktree_release_boundary_inventory.evidence_persistence_allowed == false
    and $dirty_worktree_release_boundary_inventory.package_or_release_allowed == false
    and $dirty_worktree_release_boundary_inventory.public_ga_allowed == false
    and $dirty_worktree_release_boundary_inventory.canary_activation_allowed == false
    and $dirty_worktree_release_boundary_inventory.live_activation_allowed == false
    and $dirty_worktree_release_boundary_inventory.live_execution_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.grouping_freeze_plan_ready == true
    and $dirty_worktree_release_boundary_grouping_freeze_plan.freeze_applied == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.group_entry_count == ($dirty_worktree_release_boundary_grouping_freeze_plan.top_level_group_count + $dirty_worktree_release_boundary_grouping_freeze_plan.scope_group_count)
    and $dirty_worktree_release_boundary_grouping_freeze_plan.freeze_plan_ready_count == $dirty_worktree_release_boundary_grouping_freeze_plan.group_entry_count
    and $dirty_worktree_release_boundary_grouping_freeze_plan.planned_not_applied_count == $dirty_worktree_release_boundary_grouping_freeze_plan.group_entry_count
    and $dirty_worktree_release_boundary_grouping_freeze_plan.evidence_recorded_count == 0
    and $dirty_worktree_release_boundary_grouping_freeze_plan.release_cutover_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.git_add_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.git_index_mutated == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.git_commit_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.git_push_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.git_reset_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.git_checkout_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.git_revert_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.cleanup_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.delete_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.evidence_recording_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.evidence_persistence_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.approval_acceptance_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.blocker_waiver_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.package_or_release_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.public_ga_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.canary_activation_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.live_activation_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_plan.live_execution_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.operator_readback_ready == true
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.freeze_applied == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.readback_entry_count == $dirty_worktree_release_boundary_grouping_freeze_operator_readback.source_group_entry_count
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.stable_readback_key_count == $dirty_worktree_release_boundary_grouping_freeze_operator_readback.readback_entry_count
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.diff_key_count == $dirty_worktree_release_boundary_grouping_freeze_operator_readback.readback_entry_count
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.comparison_anchor_count == $dirty_worktree_release_boundary_grouping_freeze_operator_readback.readback_entry_count
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.planned_not_applied_readback_count == $dirty_worktree_release_boundary_grouping_freeze_operator_readback.readback_entry_count
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.unchanged_freeze_state_count == $dirty_worktree_release_boundary_grouping_freeze_operator_readback.readback_entry_count
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.unchanged_evidence_state_count == $dirty_worktree_release_boundary_grouping_freeze_operator_readback.readback_entry_count
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.evidence_recorded_count == 0
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.release_cutover_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.git_add_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.git_index_mutated == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.git_commit_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.git_push_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.git_reset_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.git_checkout_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.git_revert_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.cleanup_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.delete_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.evidence_recording_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.evidence_persistence_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.approval_acceptance_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.blocker_waiver_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.package_or_release_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.public_ga_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.canary_activation_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.live_activation_allowed == false
    and $dirty_worktree_release_boundary_grouping_freeze_operator_readback.live_execution_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_ready == true
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_applied == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_entry_count == $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.source_readback_entry_count
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.stable_strategy_key_count == $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_entry_count
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_route_count == $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_entry_count
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.ready_strategy_count == $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_entry_count
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.operator_decision_required_count == $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_entry_count
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.no_git_mutation_strategy_count == $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.strategy_entry_count
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.evidence_recorded_count == 0
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.release_cutover_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.git_add_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.git_index_mutated == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.git_commit_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.git_push_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.git_reset_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.git_checkout_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.git_revert_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.cleanup_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.delete_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.evidence_recording_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.evidence_persistence_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.approval_acceptance_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.blocker_waiver_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.package_or_release_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.public_ga_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.canary_activation_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.live_activation_allowed == false
    and $dirty_worktree_release_boundary_actionable_clean_worktree_strategy.live_execution_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.operator_packet_ready == true
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.operator_packet_sent == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.operator_packet_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.strategy_applied == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.packet_section_count == 6
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.packet_entry_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.source_strategy_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.stable_packet_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.packet_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.packet_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.packet_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.attached_strategy_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.packet_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.operator_decision_required_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.packet_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.no_git_mutation_packet_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.packet_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.evidence_recorded_count == 0
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.release_cutover_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.git_add_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.git_index_mutated == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.git_commit_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.git_push_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.git_reset_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.git_checkout_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.git_revert_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.cleanup_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.delete_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.evidence_recording_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.evidence_persistence_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.approval_request_sent == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.approval_acceptance_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.blocker_waiver_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.package_or_release_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.public_ga_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.canary_activation_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.live_activation_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet.live_execution_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.non_send_readback_ready == true
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.operator_packet_visible == true
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.operator_packet_sent == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.operator_packet_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.strategy_applied == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_entry_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.source_packet_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.stable_readback_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_ready_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.visible_unsent_unpersisted_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.no_git_mutation_readback_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.evidence_recorded_count == 0
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.release_cutover_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.git_add_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.git_index_mutated == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.git_commit_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.git_push_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.git_reset_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.git_checkout_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.git_revert_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.cleanup_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.delete_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.evidence_recording_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.evidence_persistence_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.approval_request_sent == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.approval_acceptance_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.blocker_waiver_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.package_or_release_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.public_ga_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.canary_activation_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.live_activation_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.live_execution_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_mutation_boundary_readback_ready == true
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.operator_packet_visible == true
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.operator_packet_sent == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.operator_packet_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.strategy_applied == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_entry_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.source_readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.stable_readback_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_ready_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_mutation_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_operation_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.cleanup_delete_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.evidence_recorded_count == 0
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.release_cutover_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_add_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_index_mutated == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_commit_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_push_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_reset_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_checkout_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.git_revert_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.cleanup_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.delete_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.evidence_recording_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.evidence_persistence_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.approval_request_sent == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.approval_acceptance_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.blocker_waiver_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.package_or_release_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.public_ga_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.canary_activation_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.live_activation_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.live_execution_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.operator_decision_checklist_ready == true
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.decision_checklist_visible == true
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.decision_checklist_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.decision_recorded == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.operator_packet_sent == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.operator_packet_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.readback_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.strategy_applied == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.checklist_entry_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.source_readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.stable_checklist_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.checklist_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.checklist_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.checklist_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.checklist_ready_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.checklist_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.pending_operator_decision_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.checklist_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.evidence_requirement_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.checklist_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.evidence_recorded_count == 0
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.release_cutover_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.git_add_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.git_index_mutated == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.git_commit_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.git_push_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.git_reset_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.git_checkout_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.git_revert_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.cleanup_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.delete_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.evidence_recording_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.evidence_persistence_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.approval_request_sent == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.approval_acceptance_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.blocker_waiver_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.package_or_release_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.public_ga_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.canary_activation_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.live_activation_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist.live_execution_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.operator_decision_checklist_packet_readback_ready == true
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_visible == true
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.decision_checklist_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.decision_recorded == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.operator_packet_sent == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.operator_packet_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.readback_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.strategy_applied == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.source_checklist_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.stable_packet_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.stable_readback_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.readback_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_ready_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.pending_operator_decision_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.decision_recording_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.approval_acceptance_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.packet_readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.evidence_recorded_count == 0
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.release_cutover_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.git_add_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.git_index_mutated == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.git_commit_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.git_push_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.git_reset_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.git_checkout_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.git_revert_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.cleanup_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.delete_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.evidence_recording_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.evidence_persistence_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.approval_request_sent == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.approval_acceptance_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.blocker_waiver_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.package_or_release_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.public_ga_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.canary_activation_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.live_activation_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.live_execution_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.operator_decision_recording_boundary_readback_ready == true
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_recording_boundary_readback_visible == true
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_recording_boundary_readback_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_recorded == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_recording_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_receipt_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_checklist_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.operator_packet_sent == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.operator_packet_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.readback_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.strategy_applied == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.source_packet_readback_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.stable_boundary_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_ready_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.pending_operator_decision_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_recording_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_recording_persistence_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.decision_receipt_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.approval_acceptance_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.evidence_recorded_count == 0
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.release_cutover_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.git_add_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.git_index_mutated == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.git_commit_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.git_push_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.git_reset_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.git_checkout_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.git_revert_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.cleanup_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.delete_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.evidence_recording_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.evidence_persistence_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.approval_request_sent == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.approval_acceptance_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.blocker_waiver_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.package_or_release_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.public_ga_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.canary_activation_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.live_activation_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.live_execution_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.operator_approval_acceptance_boundary_readback_ready == true
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_acceptance_boundary_readback_visible == true
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_acceptance_boundary_readback_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_request_sent == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_accepted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_recorded == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_receipt_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.decision_recorded == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.decision_recording_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.decision_receipt_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.decision_checklist_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.operator_packet_sent == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.operator_packet_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.readback_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.strategy_applied == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.source_boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.stable_boundary_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_ready_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.pending_operator_decision_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_request_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_acceptance_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_recording_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.approval_receipt_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.decision_recording_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.evidence_recorded_count == 0
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.release_cutover_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.git_add_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.git_index_mutated == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.git_commit_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.git_push_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.git_reset_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.git_checkout_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.git_revert_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.cleanup_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.delete_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.evidence_recording_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.evidence_persistence_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.blocker_waiver_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.package_or_release_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.public_ga_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.canary_activation_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.live_activation_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.live_execution_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.operator_evidence_recording_boundary_readback_ready == true
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_recording_boundary_readback_visible == true
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_recording_boundary_readback_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_recorded == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_recording_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_receipt_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_request_sent == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_accepted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_recorded == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_receipt_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.decision_recorded == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.decision_recording_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.decision_receipt_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.decision_checklist_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.operator_packet_sent == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.operator_packet_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.readback_persisted == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.strategy_applied == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.source_boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.stable_boundary_key_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_route_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_ready_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.source_boundary_attached_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.pending_operator_decision_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_recording_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_persistence_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.evidence_recorded_count == 0
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_request_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_acceptance_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_recording_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.approval_receipt_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.decision_recording_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.packet_visible_unsent_unpersisted_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.readback_unpersisted_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.git_mutation_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.cleanup_delete_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.strategy_application_blocked_count == $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.boundary_entry_count
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.release_cutover_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.git_add_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.git_index_mutated == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.git_commit_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.git_push_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.git_reset_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.git_checkout_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.git_revert_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.cleanup_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.delete_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.blocker_waiver_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.package_or_release_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.public_ga_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.canary_activation_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.live_activation_allowed == false
    and $dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback.live_execution_allowed == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.release_risk_snapshot_ready == true
    and $dirty_worktree_release_boundary_release_risk_snapshot.risk_snapshot_visible == true
    and $dirty_worktree_release_boundary_release_risk_snapshot.risk_snapshot_persisted == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count == $dirty_worktree_release_boundary_release_risk_snapshot.source_boundary_entry_count
    and $dirty_worktree_release_boundary_release_risk_snapshot.stable_snapshot_key_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count
    and $dirty_worktree_release_boundary_release_risk_snapshot.snapshot_route_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count
    and $dirty_worktree_release_boundary_release_risk_snapshot.snapshot_ready_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count
    and $dirty_worktree_release_boundary_release_risk_snapshot.critical_risk_count == 1
    and $dirty_worktree_release_boundary_release_risk_snapshot.high_risk_count == 4
    and $dirty_worktree_release_boundary_release_risk_snapshot.medium_risk_count == 2
    and $dirty_worktree_release_boundary_release_risk_snapshot.high_or_critical_risk_count == 5
    and $dirty_worktree_release_boundary_release_risk_snapshot.release_blocked_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count
    and $dirty_worktree_release_boundary_release_risk_snapshot.rehearsal_candidate_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count
    and $dirty_worktree_release_boundary_release_risk_snapshot.pending_operator_decision_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count
    and $dirty_worktree_release_boundary_release_risk_snapshot.evidence_recording_blocked_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count
    and $dirty_worktree_release_boundary_release_risk_snapshot.git_mutation_blocked_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count
    and $dirty_worktree_release_boundary_release_risk_snapshot.cleanup_delete_blocked_count == $dirty_worktree_release_boundary_release_risk_snapshot.risk_entry_count
    and $dirty_worktree_release_boundary_release_risk_snapshot.evidence_recorded == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.evidence_recording_persisted == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.evidence_receipt_persisted == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.approval_request_sent == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.approval_accepted == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.approval_recorded == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.approval_receipt_persisted == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.decision_recorded == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.decision_recording_persisted == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.decision_receipt_persisted == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.operator_packet_sent == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.operator_packet_persisted == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.readback_persisted == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.strategy_applied == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.release_cutover_allowed == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.git_add_allowed == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.git_index_mutated == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.git_commit_allowed == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.git_push_allowed == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.git_reset_allowed == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.git_checkout_allowed == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.git_revert_allowed == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.cleanup_allowed == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.delete_allowed == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.blocker_waiver_allowed == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.package_or_release_allowed == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.public_ga_allowed == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.canary_activation_allowed == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.live_activation_allowed == false
    and $dirty_worktree_release_boundary_release_risk_snapshot.live_execution_allowed == false
    and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.test_only_clean_worktree_strategy_rehearsal_ready == true
    and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.test_only_rehearsal_visible == true
    and $dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal.test_only_rehearsal_persisted == false
    and $plugin_fixture_shape_ready
    and $git_status_entry_count > 0) as $matrix_ready |
  {
    runtime:"hepta",
    surface:"current_reality_capability_matrix",
    status:(if $matrix_ready then "ready" else "blocked" end),
    matrix_date:"2026-06-27",
    local_capability_count:($capabilities | length),
    local_capability_ready_count:$ready_count,
    local_capability_blocked_count:$blocked_count,
    live_enabled_count:$live_enabled_count,
    all_live_paths_blocked:($live_enabled_count == 0),
    plugin_fixture_shape_ready:$plugin_fixture_shape_ready,
    plugin_manifest_present:$plugin_manifest_present,
    plugin_manifest_summary:$plugin_manifest_summary,
    memory_drift_entry_count:($memory_drift_entries | length),
    missing_memory_checkpoint_count:$missing_memory_checkpoint_count,
    resolved_memory_checkpoint_count:$resolved_memory_checkpoint_count,
    memory_filesystem_drift_tracked:(($memory_drift_entries | length) == 5 and $missing_memory_checkpoint_count == 0 and $resolved_memory_checkpoint_count == 5),
    dirty_worktree_boundary_tracked:($git_status_entry_count > 0),
    git_status_entry_count:$git_status_entry_count,
    git_untracked_count:$git_untracked_count,
    git_tracked_change_count:$git_tracked_change_count,
    current_reality_capability_matrix_ready:$matrix_ready,
    capabilities:$capabilities,
    memory_drift_entries:$memory_drift_entries,
    blockers:[
      "workflow_durable_store_write_blocked_by_feature_gate",
      "workflow_durable_store_test_only_append_fixture_blocks_runtime_writes",
      "workflow_temporal_lite_append_only_event_store_test_implementation_blocks_runtime_writes_and_live",
      "workflow_temporal_lite_append_only_event_store_minimal_local_persistence_blocks_runtime_event_log_sqlite_store_workflow_replay_rollback_and_live",
      "workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_blocks_runtime_replay_projection_persistence_replay_execution_and_live",
      "workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_blocks_checkpoint_rollback_anchor_writes_persistence_and_live",
      "workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_blocks_lease_acquire_idempotency_write_persistence_and_live",
      "workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_blocks_event_log_sqlite_adapter_writes_persistence_and_live",
      "workflow_temporal_lite_work_graph_projection_local_persistence_readback_blocks_projection_persistence_writes_and_live",
      "workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback_blocks_replay_alignment_execution_persistence_writes_and_live",
      "workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback_blocks_checkpoint_consistency_execution_persistence_writes_and_live",
      "workflow_temporal_lite_replay_alignment_rollback_consistency_local_persistence_readback_blocks_rollback_consistency_execution_persistence_writes_and_live",
      "workflow_temporal_lite_replay_alignment_recovery_window_local_persistence_readback_blocks_recovery_window_execution_persistence_writes_and_live",
      "workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback_blocks_recovery_receipt_execution_persistence_writes_and_live",
      "workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_blocks_replay_execution_persistence_and_live",
      "workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_blocks_checkpoint_rollback_writes_persistence_and_live",
      "workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_blocks_lease_acquire_idempotency_write_persistence_and_live",
      "workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_blocks_event_log_sqlite_adapter_writes_persistence_and_live",
      "workflow_temporal_lite_work_graph_projection_feature_gated_readback_blocks_projection_persistence_writes_and_live",
      "workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback_blocks_replay_alignment_execution_persistence_writes_and_live",
      "workflow_temporal_lite_replay_alignment_checkpoint_consistency_feature_gated_readback_blocks_checkpoint_consistency_execution_persistence_writes_and_live",
      "workflow_temporal_lite_replay_alignment_rollback_consistency_feature_gated_readback_blocks_rollback_consistency_execution_persistence_writes_and_live",
      "workflow_temporal_lite_replay_alignment_recovery_window_feature_gated_readback_blocks_recovery_window_execution_persistence_writes_and_live",
      "workflow_temporal_lite_replay_alignment_recovery_receipt_feature_gated_readback_blocks_recovery_receipt_execution_persistence_writes_and_live",
      "hepta_systems_gate_recursion_cost_boundary_readback_blocks_matrix_cache_source_semantic_changes_gate_chain_invocation_and_live",
      "hepta_systems_gate_recursion_lean_contract_readback_blocks_recursive_source_gate_chain_cache_persistence_source_semantics_and_live",
      "hepta_systems_workgraph_legacy_gate_recursion_inventory_readback_blocks_legacy_workgraph_recursive_gate_chains_source_semantics_and_live",
      "hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_blocks_tool_write_ledger_approval_receipt_persistence_and_live",
      "hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_blocks_plugin_install_cache_activation_signature_trust_and_live",
      "hepta_systems_plugin_signature_trust_install_cache_boundary_readback_blocks_signature_trust_install_cache_evidence_acceptance_and_live",
      "hepta_systems_plugin_operator_evidence_acceptance_packet_readback_blocks_packet_send_evidence_acceptance_install_cache_and_live",
      "hepta_systems_plugin_install_cache_noop_preflight_readback_blocks_preflight_execution_cache_materialization_install_receipt_persistence_and_live",
      "hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback_blocks_idempotency_index_write_denial_receipt_persistence_install_cache_and_live",
      "hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_blocks_rollback_uninstall_execution_plan_persistence_install_cache_and_live",
      "hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_blocks_activation_permission_connector_start_tool_registration_ledger_receipt_and_live",
      "hepta_systems_plugin_tool_registry_registration_denial_receipt_readback_blocks_tool_registry_registration_lookup_invocation_ledger_receipt_and_live",
      "hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback_blocks_tool_invocation_noop_result_ledger_approval_receipt_and_live",
      "hepta_systems_plugin_tool_invocation_policy_approval_ledger_boundary_readback_blocks_policy_approval_ledger_receipt_persistence_invocation_and_live",
      "hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_blocks_feature_gate_open_dry_run_execution_tool_invocation_ledger_receipt_and_live",
      "hepta_systems_plugin_tool_invocation_dry_run_receipt_ledger_preview_readback_blocks_feature_gate_open_dry_run_execution_receipt_ledger_persistence_and_live",
      "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_packet_readback_blocks_operator_packet_send_persistence_acceptance_recording_tool_invocation_ledger_receipt_and_live",
      "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback_blocks_acceptance_recording_receipt_persistence_tool_invocation_runtime_and_live",
      "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_execution_open_preconditions_readback_blocks_operator_evidence_acceptance_ledger_receipt_registration_invocation_connector_runtime_and_live",
      "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback_blocks_evidence_packet_send_recording_acceptance_ledger_receipt_registration_invocation_connector_runtime_and_live",
      "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_boundary_readback_blocks_acceptance_recording_evidence_recording_receipt_persistence_invocation_runtime_and_live",
      "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback_blocks_evidence_artifact_identity_acceptance_recording_ledger_receipt_registration_invocation_connector_runtime_and_live",
      "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback_blocks_acceptance_record_persistence_denial_receipt_idempotency_ledger_receipt_registration_invocation_runtime_and_live",
      "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback_blocks_acceptance_record_store_binding_idempotency_ledger_receipt_runtime_rollback_kill_switch_evidence_feature_gate_and_live",
      "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback_blocks_shadow_execution_store_write_registry_lookup_invocation_runtime_and_live",
      "hepta_systems_tool_registry_shadow_registration_lookup_readback_blocks_shadow_registry_lookup_registration_invocation_ledger_runtime_and_live",
      "hepta_systems_matrix_report_single_render_cache_boundary_readback_blocks_cache_persistence_downstream_direct_matrix_render_and_live",
      "current_reality_matrix_compact_cache_boundary_readback_blocks_cache_persistence_evidence_approval_decision_and_live",
      "hepta_system_status_internal_read_only_invocation_blocks_external_network_credentials_mutation_and_live",
      "hepta_system_status_operator_approval_protocol_blocks_auto_acceptance_broker_write_persistence_and_live",
      "tool_registry_dispatch_live_mutation_blocked_by_design",
      "workflow_durable_runtime_disabled",
      "native_read_only_console_live_mutation_blocked_by_design",
      "controlled_live_cutover_blocked_by_operator_approval_and_evidence",
      "controlled_live_denial_readback_index_blocks_waiver_and_acceptance",
      "controlled_live_operator_packet_preview_blocks_approval_request",
      "controlled_live_operator_packet_non_send_readback_blocks_send_and_persistence",
      "controlled_live_required_evidence_plan_blocks_recording_and_acceptance",
      "controlled_live_required_evidence_readback_index_blocks_recording_and_acceptance",
      "controlled_live_required_evidence_gap_summary_blocks_acceptance_and_recording",
      "controlled_live_required_evidence_gap_diff_view_blocks_acceptance_and_recording",
      "controlled_live_required_evidence_gap_operator_readback_blocks_acceptance_and_persistence",
      "controlled_live_required_evidence_gap_operator_packet_attachment_blocks_acceptance_send_and_persistence",
      "controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_blocks_send_persistence_and_approval_request",
      "controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback_blocks_transport_mutation",
      "controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary_readback_blocks_credential_access",
      "controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback_blocks_rehearsal_execution",
      "controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback_blocks_kill_switch_mutation",
      "controlled_canary_readiness_plan_blocks_activation_transport_persistence_and_live",
      "dirty_worktree_release_boundary_inventory_blocks_git_mutation_cleanup_release_and_live",
      "dirty_worktree_release_boundary_grouping_freeze_plan_blocks_freeze_application_git_mutation_release_and_live",
      "dirty_worktree_release_boundary_grouping_freeze_operator_readback_blocks_git_mutation_release_and_live",
      "dirty_worktree_release_boundary_actionable_clean_worktree_strategy_blocks_strategy_application_git_mutation_release_and_live",
      "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_blocks_send_persistence_git_mutation_release_and_live",
      "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_blocks_send_persistence_git_mutation_release_and_live",
      "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_blocks_git_mutation_cleanup_delete_release_and_live",
      "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_blocks_decision_recording_git_mutation_cleanup_release_and_live",
      "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback_blocks_packet_persistence_decision_recording_git_mutation_cleanup_release_and_live",
      "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback_blocks_decision_recording_persistence_receipt_approval_git_mutation_cleanup_release_and_live",
      "dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback_blocks_approval_acceptance_receipt_decision_recording_evidence_git_mutation_cleanup_release_and_live",
      "dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback_blocks_evidence_recording_persistence_receipt_approval_decision_git_mutation_cleanup_release_and_live",
      "dirty_worktree_release_boundary_release_risk_snapshot_blocks_release_cutover_git_mutation_cleanup_evidence_approval_decision_recording_and_live",
      "dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_blocks_test_probe_git_mutation_cleanup_evidence_approval_decision_recording_and_live",
      "dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_blocks_test_probe_git_mutation_cleanup_evidence_approval_decision_recording_and_live",
      "dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_blocks_owner_persistence_freeze_application_classification_persistence_git_mutation_cleanup_release_and_live",
      "dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_blocks_owner_assignment_freeze_classification_operator_packet_git_cleanup_release_and_live",
      "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send_blocks_packet_send_persistence_owner_assignment_git_cleanup_release_and_live",
      "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_without_git_mutation_blocks_git_mutation_cleanup_delete_owner_freeze_classification_packet_release_and_live",
      "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_without_git_mutation_blocks_decision_recording_approval_evidence_git_cleanup_release_and_live",
      "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_without_git_mutation_blocks_packet_readback_decision_recording_approval_evidence_git_cleanup_release_and_live",
      "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_without_recording_blocks_decision_recording_persistence_receipt_approval_evidence_owner_freeze_classification_test_probe_git_cleanup_release_and_live",
      "dirty_worktree_release_boundary_owner_freeze_classification_operator_approval_acceptance_boundary_readback_without_acceptance_blocks_approval_request_acceptance_recording_receipt_decision_evidence_owner_freeze_classification_test_probe_git_cleanup_release_and_live",
      "dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_without_recording_blocks_evidence_recording_persistence_receipt_approval_decision_owner_freeze_classification_test_probe_git_cleanup_release_and_live",
      "plugin_lifecycle_live_mutation_blocked_by_design",
      "workgraph_suffix_ladder_accretion",
      "dirty_worktree_boundary",
      "live_and_public_ga_blocked_by_design"
    ],
    next_actions:[
      "close_controlled_live_evidence_before_status_canary_start"
    ],
    next_migration_step:"close_controlled_live_evidence_before_status_canary_start",
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      plugin_installed:false,
      plugin_cache_mutated:false,
      tool_registered:false,
      tool_invoked:false,
      tool_invocation_ledger_written:false,
      approval_requested:false,
      approval_broker_mutated:false,
      workflow_event_log_mutated:false,
      sqlite_written:false,
      workgraph_execution_started:false,
      replay_executed:false,
      rollback_executed:false,
      readback_receipt_persisted:false,
      gateway_or_auth_mutated:false,
      native_post_mutation_performed:false,
      channel_send_performed:false,
      provider_invoked:false,
      model_invoked:false,
      package_or_release_written:false,
      public_ga_promoted:false
    }
  }'
