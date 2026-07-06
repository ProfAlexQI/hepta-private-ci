#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-ttrw-shadow-activation-layer13-blocker-drift-budget-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-ttrw-shadow-activation-layer13-blocker-drift-budget-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_gate"
  and .schema_version == "work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_v1"
  and .preview_mode == "read_only_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_no_execution"
  and .drift_budget_count == 5
  and (.drift_budgets | length) == .drift_budget_count
  and (.drift_budgets | map(.drift_detector_id) == [
    "detect_shadow_activation_blocker_activation_blocker_surface_state_drift",
    "detect_shadow_activation_blocker_activation_blocker_binding_drift",
    "detect_shadow_activation_blocker_activation_enablement_satisfaction_drift",
    "detect_shadow_activation_blocker_activation_kill_switch_armament_drift",
    "detect_shadow_activation_blocker_activation_side_effect_lock_drift"
  ])
  and (.drift_budgets | all(
    .max_allowed_mismatches == 0
    and .max_allowed_unreviewed_findings == 0
    and .max_replay_lag_ms == 0
    and .severity == "critical"
    and .block_level == "block_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_activation_promotion_enforcement_store_and_live_execution"
    and .allows_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_execution == false
    and .allows_shadow_activation_blocker_activation == false
    and .allows_shadow_activation_blocker_promotion_execution == false
    and .allows_shadow_activation == false
    and .allows_activation == false
    and .allows_shadow_promotion_execution == false
    and .allows_task_result_enforcement == false
    and .allows_store_enablement == false
    and .allows_live_execution == false
  ))
  and .operator_summary_count == 5
  and (.operator_summaries | length) == .operator_summary_count
  and (.operator_summaries | map(.id) == [
    "shadow_activation_blocker_activation_blocker_surface_state_drift_operator_summary",
    "shadow_activation_blocker_activation_blocker_binding_drift_operator_summary",
    "shadow_activation_blocker_activation_enablement_satisfaction_drift_operator_summary",
    "shadow_activation_blocker_activation_kill_switch_armament_drift_operator_summary",
    "shadow_activation_blocker_activation_side_effect_lock_drift_operator_summary"
  ])
  and (.operator_summaries | all(
    .review_state == "preview_summary_defined_review_not_performed"
    and .persists_summary == false
    and .external_delivery_allowed == false
    and (.required_fields | length) == 5
  ))
  and .activation_precondition_count == 4
  and (.activation_preconditions | length) == .activation_precondition_count
  and (.activation_preconditions | map(.id) == [
    "all_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets_zero_tolerance",
    "shadow_activation_blocker_activation_blocker_operator_summaries_reviewed",
    "shadow_activation_blocker_activation_blocker_side_effect_lock_zero_mutation_required",
    "shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_remains_disabled_until_budget_review"
  ])
  and (.activation_preconditions | all(
    .blocks_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback == true
    and .blocks_shadow_activation_blocker_activation == true
    and .blocks_shadow_activation_blocker_promotion == true
    and .blocks_activation == true
    and .blocks_shadow_promotion == true
    and .currently_satisfied == false
  ))
  and .blocker_count == 4
  and (.blockers | length) == .blocker_count
  and (.blockers | map(.id) == [
    "shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_not_executed",
    "shadow_activation_blocker_activation_blocker_operator_review_not_performed",
    "shadow_activation_blocker_activation_blocker_preconditions_not_attached",
    "shadow_activation_blocker_activation_blocker_drift_persistence_disabled"
  ])
  and (.blockers | map(select(.severity == "critical")) | length) == 1
  and (.blockers | map(select(.severity == "high")) | length) == 1
  and (.blockers | all(
    .required_before_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_execution == true
    and .required_before_shadow_activation_blocker_activation == true
    and .required_before_activation == true
  ))
  and .required_prior_gate_count == 71
  and (.required_prior_gates[-1] == "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_gate"
  and .ready_for_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview == true
  and .ready_for_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_execution == false
  and .ready_for_shadow_activation_blocker_activation_execution == false
  and .ready_for_shadow_activation_blocker_promotion_execution == false
  and .ready_for_shadow_activation_execution == false
  and .ready_for_activation == false
  and .ready_for_shadow_promotion_execution == false
  and .ready_for_wrapper_execution == false
  and .ready_for_task_result_enforcement == false
  and .ready_for_store_enablement == false
  and .ready_for_live_execution == false
  and .source_probes.terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget.rust_module_present == true
  and .source_probes.terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget.report_script_present == true
  and .source_probes.terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget.gate_script_present == true
  and .source_probes.terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback.rust_module_present == true
  and .source_probes.terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_ttrw_shadow_activation_layer13_blocker_drift_budget_preview --lib

echo "Hepta WorkGraph terminal TaskResult wrapper shadow activation activation blocker activation blocker activation blocker activation blocker activation blocker activation blocker drift budget preview gate passed"
