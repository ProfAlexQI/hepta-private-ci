#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-terminal-task-result-wrapper-drift-budget-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-terminal-task-result-wrapper-drift-budget-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_gate"
  and .schema_version == "work_graph_terminal_task_result_wrapper_drift_budget_preview_v1"
  and .preview_mode == "read_only_terminal_task_result_wrapper_drift_budget_preview_no_execution"
  and .drift_budget_count == 5
  and (.drift_budgets | length) == .drift_budget_count
  and (.drift_budgets | map(.drift_detector_id) == [
    "detect_fixture_identity_drift",
    "detect_fixture_status_drift",
    "detect_fixture_evidence_drift",
    "detect_fixture_verifier_drift",
    "detect_fixture_redaction_drift"
  ])
  and (.drift_budgets | all(
    .max_allowed_mismatches == 0
    and .max_allowed_unreviewed_findings == 0
    and .max_replay_lag_ms == 0
    and .severity == "critical"
    and .block_level == "block_wrapper_execution_task_result_enforcement_and_promotion"
    and .allows_readback_execution == false
    and .allows_wrapper_execution == false
    and .allows_task_result_enforcement == false
    and .allows_store_promotion == false
  ))
  and .operator_summary_count == 5
  and (.operator_summaries | length) == .operator_summary_count
  and (.operator_summaries | all(
    .review_state == "preview_summary_defined_review_not_performed"
    and .persists_summary == false
    and .external_delivery_allowed == false
  ))
  and .promotion_precondition_count == 4
  and (.promotion_preconditions | length) == .promotion_precondition_count
  and (.promotion_preconditions | map(.id) == [
    "all_critical_drift_budgets_zero_tolerance",
    "operator_summaries_reviewed",
    "redaction_drift_zero_leak_required",
    "execution_remains_disabled_until_budget_review"
  ])
  and (.promotion_preconditions | all(.blocks_promotion == true and .currently_satisfied == false))
  and .blocker_count == 4
  and (.blockers | length) == .blocker_count
  and (.blockers | map(.id) == [
    "critical_drift_budget_not_executed",
    "operator_review_not_performed",
    "promotion_preconditions_not_attached",
    "drift_persistence_disabled"
  ])
  and (.blockers | map(select(.severity == "high")) | length) == 2
  and (.blockers | all(.required_before_readback_execution == true and .required_before_promotion == true))
  and .required_prior_gate_count == 15
  and (.required_prior_gates[-1] == "hepta_work_graph_terminal_task_result_wrapper_readback_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_terminal_task_result_wrapper_promotion_precondition_preview_gate"
  and .ready_for_promotion_precondition_preview == true
  and .ready_for_readback_execution == false
  and .ready_for_wrapper_execution == false
  and .ready_for_task_result_enforcement == false
  and .ready_for_store_enablement == false
  and .ready_for_live_execution == false
  and .source_probes.terminal_task_result_wrapper_drift_budget.rust_module_present == true
  and .source_probes.terminal_task_result_wrapper_drift_budget.report_script_present == true
  and .source_probes.terminal_task_result_wrapper_drift_budget.gate_script_present == true
  and .source_probes.terminal_task_result_wrapper_readback.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_terminal_task_result_wrapper_drift_budget --lib

echo "Hepta WorkGraph terminal TaskResult wrapper drift budget preview gate passed"
