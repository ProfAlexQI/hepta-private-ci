#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-terminal-task-result-wrapper-shadow-activation-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-readback-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-terminal-task-result-wrapper-shadow-activation-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-readback-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_gate"
  and .schema_version == "work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_v1"
  and .preview_mode == "read_only_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_no_execution"
  and .readback_plan_count == 7
  and (.readback_plans | length) == .readback_plan_count
  and (.readback_plans | map(.activation_surface_id) == [
    "wrapper_execution_activation",
    "readback_execution_activation",
    "promotion_execution_activation",
    "task_result_enforcement_activation",
    "store_enablement_activation",
    "live_execution_activation",
    "external_delivery_activation"
  ])
  and (.readback_plans | all(
    .source_gate_id == "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_gate"
    and .expected_activation_state == "blocked_preview_only"
    and .readback_state == "preview_contract_defined_shadow_activation_blocker_activation_blocker_readback_execution_disabled"
    and (.expected_blocker_ids | length) == 4
    and (.expected_enablement_ids | length) == 3
    and (.required_assertion_ids | length) == 6
    and (.drift_detector_ids | length) == 5
    and .performs_readback == false
    and .performs_shadow_activation_blocker_activation_blocker_activation == false
    and .performs_shadow_activation == false
    and .mutates_activation_state == false
    and .mutates_store == false
    and .enables_runtime_mutation == false
  ))
  and .blocker_assertion_count == 6
  and (.blocker_assertions | length) == .blocker_assertion_count
  and (.blocker_assertions | map(.id) == [
    "assert_shadow_activation_blocker_activation_blocker_surface_stays_blocked",
    "assert_shadow_activation_blocker_activation_blocker_set_matches_preview",
    "assert_shadow_activation_blocker_activation_blocker_activation_required_enablements_remain_unsatisfied",
    "assert_shadow_activation_blocker_activation_blocker_activation_kill_switches_remain_preview_only",
    "assert_shadow_activation_blocker_activation_blocker_activation_side_effect_lock_remains_false",
    "assert_shadow_activation_blocker_activation_blocker_activation_prior_gate_chain_matches"
  ])
  and (.blocker_assertions | all(
    .blocks_activation == true
    and .performs_readback == false
    and .mutates_store == false
    and (.required_inputs | length) >= 3
    and (.evidence_fields | length) >= 3
  ))
  and .drift_detector_count == 5
  and (.drift_detectors | length) == .drift_detector_count
  and (.drift_detectors | map(.id) == [
    "detect_shadow_activation_blocker_activation_blocker_surface_state_drift",
    "detect_shadow_activation_blocker_activation_blocker_binding_drift",
    "detect_shadow_activation_blocker_activation_blocker_activation_enablement_satisfaction_drift",
    "detect_shadow_activation_blocker_activation_blocker_activation_kill_switch_armament_drift",
    "detect_shadow_activation_blocker_activation_blocker_activation_side_effect_lock_drift"
  ])
  and (.drift_detectors | all(.severity == "critical" and .blocks_activation == true and .persists_drift == false))
  and .operator_summary_count == 5
  and (.operator_summaries | length) == .operator_summary_count
  and (.operator_summaries | map(.id) == [
    "shadow_activation_blocker_activation_blocker_surface_denial_summary",
    "shadow_activation_blocker_activation_blocker_enablement_gap_summary",
    "shadow_activation_blocker_activation_blocker_kill_switch_summary",
    "shadow_activation_blocker_activation_blocker_side_effect_lock_summary",
    "shadow_activation_blocker_activation_blocker_external_delivery_denial_summary"
  ])
  and (.operator_summaries | all(.redacted == true and .persists_summary == false and (.required_fields | length) >= 3))
  and .blocker_count == 4
  and (.blockers | length) == .blocker_count
  and (.blockers | map(.id) == [
    "shadow_activation_blocker_activation_blocker_readback_execution_disabled",
    "shadow_activation_blocker_activation_blocker_state_persistence_disabled",
    "shadow_activation_blocker_activation_blocker_operator_review_missing",
    "shadow_activation_blocker_activation_blocker_drift_budget_missing"
  ])
  and (.blockers | map(select(.severity == "critical")) | length) == 1
  and (.blockers | map(select(.severity == "high")) | length) == 3
  and (.blockers | all(.required_before_readback_execution == true and (.affected_activation_surface_ids | length) >= 1))
  and .required_prior_gate_count == 46
  and (.required_prior_gates[-1] == "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_gate"
  and .ready_for_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview == true
  and .ready_for_shadow_activation_blocker_activation_blocker_readback_execution == false
  and .ready_for_shadow_activation_blocker_activation_blocker_activation_execution == false
  and .ready_for_shadow_activation_blocker_activation_blocker_promotion_execution == false
  and .ready_for_shadow_activation_execution == false
  and .ready_for_activation == false
  and .ready_for_wrapper_execution == false
  and .ready_for_task_result_enforcement == false
  and .ready_for_store_enablement == false
  and .ready_for_live_execution == false
  and .source_probes.terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback.rust_module_present == true
  and .source_probes.terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback.report_script_present == true
  and .source_probes.terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback.gate_script_present == true
  and .source_probes.terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker.rust_module_present == true
  and .source_probes.terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback --lib

echo "Hepta WorkGraph terminal TaskResult wrapper shadow activation activation blocker activation blocker activation blocker activation blocker activation blocker activation blocker readback preview gate passed"
