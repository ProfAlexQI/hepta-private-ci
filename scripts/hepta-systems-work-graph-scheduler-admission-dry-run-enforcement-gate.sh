#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-scheduler-admission-dry-run-enforcement-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-scheduler-admission-dry-run-enforcement-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  and .schema_version == "work_graph_scheduler_admission_dry_run_enforcement_v1"
  and .preview_mode == "dry_run_allow_deny_explanation_before_entrypoint_no_live_blocking"
  and .entrypoint_count == 4
  and (.entrypoints | map(.id) == [
    "spawn_agent",
    "spawn_agents_on_csv",
    "task_board_claim",
    "worker_task_run"
  ])
  and (.entrypoints | all(.dry_run_enforcement_enabled == true and .live_blocking_enforcement_enabled == false))
  and (.entrypoints | all((.applied_check_ids | length) == 7 and (.explanation_output_fields | index("traceId"))))
  and .check_count == 7
  and (.checks | map(.id) == [
    "dependencies_terminal_ready",
    "lane_lease_available_and_owned",
    "approval_authority_present_when_required",
    "idempotency_replay_window_clear",
    "budget_and_timeout_available",
    "task_result_contract_preview_present",
    "side_effect_boundary_locked"
  ])
  and (.checks | all(.blocks_live_execution == true and .explanation_required == true))
  and .decision_count == 8
  and (.decisions | map(.id) == [
    "allow_dry_run",
    "deny_dependencies_not_ready",
    "deny_lease_unavailable",
    "deny_approval_missing",
    "deny_idempotency_conflict",
    "deny_budget_exhausted",
    "deny_task_result_preview_missing",
    "deny_side_effect_boundary_open"
  ])
  and (.decisions[] | select(.id == "allow_dry_run") | .allow_entrypoint_to_continue == true and .live_execution_blocked == false)
  and (.decisions | map(select(.id | startswith("deny_"))) | all(.allow_entrypoint_to_continue == false and .live_execution_blocked == true))
  and .explanation_count == 4
  and (.explanations | map([.entrypoint_id, .decision_id]) == [
    ["spawn_agent", "allow_dry_run"],
    ["spawn_agents_on_csv", "allow_dry_run"],
    ["task_board_claim", "deny_lease_unavailable"],
    ["worker_task_run", "deny_side_effect_boundary_open"]
  ])
  and (.explanations | all((.trace_id | length) > 0))
  and (.required_prior_gates == [
    "hepta_work_graph_task_result_envelope_report_only_validator_gate",
    "hepta_work_graph_adapter_task_result_index_gate",
    "hepta_work_graph_terminal_envelope_readback_gate",
    "hepta_work_graph_source_id_alignment_readback_gate",
    "hepta_work_graph_task_result_contract_field_gap_readback_gate"
  ])
  and .recommended_next_gate == "hepta_work_graph_append_only_event_store_shadow_path_gate"
  and .dry_run_enforcement_enabled == true
  and .live_blocking_enforcement_enabled == false
  and .ready_for_append_only_event_store_shadow_path == true
  and .ready_for_live_execution == false
  and .source_probes.scheduler_admission_dry_run_enforcement.rust_module_present == true
  and .source_probes.scheduler_admission_dry_run_enforcement.report_script_present == true
  and .source_probes.scheduler_admission_dry_run_enforcement.gate_script_present == true
  and .source_probes.task_result_envelope_report_only_validator.gate_script_present == true
  and .source_probes.adapter_task_result_index.gate_script_present == true
  and .source_probes.terminal_envelope_readback.gate_script_present == true
  and .source_probes.source_id_alignment_readback.gate_script_present == true
  and .source_probes.task_result_contract_field_gap_readback.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_scheduler_admission_dry_run_enforcement --lib

echo "Hepta WorkGraph scheduler admission dry-run enforcement gate passed"
