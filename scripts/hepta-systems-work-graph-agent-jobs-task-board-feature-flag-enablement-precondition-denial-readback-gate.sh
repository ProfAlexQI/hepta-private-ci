#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-enablement-precondition-denial-readback-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-feature-flag-enablement-precondition-denial-readback-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_v1"
  and .preview_mode == "enablement_precondition_denial_readback_only_no_accept_no_record_no_persistence"
  and .source_enablement_precondition_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_gate"
  and .source_decision_count == 2
  and .source_deny_reason_count == 10
  and .source_allow_count == 0
  and .source_deny_count == 2
  and .denial_readback_entry_count == 5
  and .denial_readback_blocker_count == 10
  and .denial_readback_scope.readback_mode == "enablement_precondition_denial_readback_only"
  and .denial_readback_scope.denial_visible == true
  and .denial_readback_scope.denial_recorded == false
  and .denial_readback_scope.denial_persisted == false
  and .denial_readback_scope.denial_accepted == false
  and .denial_readback_scope.denial_authoritative == false
  and .denial_readback_scope.readback_persisted == false
  and (.denial_readback_entries | all(
    .visible == true
    and .recorded == false
    and .persisted == false
    and .accepted == false
    and .authoritative == false
    and .mutation_allowed == false
    and .ready == true
  ))
  and (.denial_readback_blockers | map(.blocked_action) == [
    "accept_enablement_decision",
    "record_operator_approval",
    "persist_denial_readback",
    "write_feature_flag_config",
    "enable_feature_flag",
    "route_canary_traffic",
    "enforce_scheduler_admission",
    "execute_replay",
    "execute_rollback",
    "perform_live_cutover"
  ])
  and (.denial_readback_blockers | all(.blocked == true))
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_gate",
    "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate",
    "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate",
    "hepta_work_graph_trace_guardrail_span_report_only_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  ]
  and .required_prior_gate_count == 10
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_gate"
  and .dry_run_denial_visible == true
  and .dry_run_denial_recorded == false
  and .dry_run_denial_persisted == false
  and .dry_run_denial_accepted == false
  and .dry_run_denial_authoritative == false
  and .denial_readback_persisted == false
  and .denial_readback_authorizes_config_write == false
  and .denial_readback_authorizes_feature_flag_enablement == false
  and .denial_readback_authorizes_canary_traffic == false
  and .denial_readback_authorizes_live_cutover == false
  and .approval_recorded == false
  and .approval_acceptance_allowed == false
  and .ready_for_denial_audit_index == true
  and .ready_for_feature_flag_config_write == false
  and .ready_for_feature_flag_enablement == false
  and .ready_for_canary_traffic == false
  and .ready_for_live_cutover == false
  and .source_probes.denial_readback_module_present == true
  and .source_probes.enablement_dry_run_gate_present == true
  and .source_probes.enablement_dry_run_points_here == true
  and .source_probes.enablement_dry_run_ready_present == true
  and .source_probes.enablement_dry_run_denies_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback --lib

echo "Hepta WorkGraph agent_jobs + task_board feature-flag enablement precondition denial readback gate passed"
