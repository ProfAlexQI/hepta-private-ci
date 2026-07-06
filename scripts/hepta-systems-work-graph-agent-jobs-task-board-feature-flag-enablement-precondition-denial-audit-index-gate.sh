#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-enablement-precondition-denial-audit-index-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-feature-flag-enablement-precondition-denial-audit-index-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_v1"
  and .preview_mode == "enablement_precondition_denial_audit_index_no_record_no_persistence_no_acceptance"
  and .source_denial_readback_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_gate"
  and .source_denial_readback_entry_count == 5
  and .source_denial_readback_blocker_count == 10
  and .source_required_prior_gate_count == 10
  and .audit_index_entry_count == 6
  and .audit_index_blocker_count == 11
  and .audit_index_scope.index_mode == "denial_audit_index_report_only"
  and .audit_index_scope.index_visible == true
  and .audit_index_scope.index_recorded == false
  and .audit_index_scope.index_persisted == false
  and .audit_index_scope.index_authoritative == false
  and .audit_index_scope.acceptance_allowed == false
  and (.audit_index_entries | all(
    .indexed == true
    and .recorded == false
    and .persisted == false
    and .authoritative == false
    and .mutation_allowed == false
    and .ready == true
  ))
  and (.audit_index_blockers | map(.blocked_action) == [
    "record_denial_audit_index",
    "persist_denial_audit_index",
    "accept_denial_audit_index",
    "record_operator_approval",
    "write_feature_flag_config",
    "enable_feature_flag",
    "route_canary_traffic",
    "enforce_scheduler_admission",
    "execute_replay",
    "execute_rollback",
    "perform_live_cutover"
  ])
  and (.audit_index_blockers | all(.blocked == true))
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_gate",
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
  and .required_prior_gate_count == 11
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_gate"
  and .audit_index_visible == true
  and .audit_index_recorded == false
  and .audit_index_persisted == false
  and .audit_index_authoritative == false
  and .audit_index_acceptance_allowed == false
  and .audit_index_authorizes_config_write == false
  and .audit_index_authorizes_feature_flag_enablement == false
  and .audit_index_authorizes_canary_traffic == false
  and .audit_index_authorizes_scheduler_enforcement == false
  and .audit_index_authorizes_replay_execution == false
  and .audit_index_authorizes_rollback_execution == false
  and .audit_index_authorizes_live_cutover == false
  and .ready_for_non_persistence_readback == true
  and .ready_for_feature_flag_config_write == false
  and .ready_for_feature_flag_enablement == false
  and .ready_for_canary_traffic == false
  and .ready_for_live_cutover == false
  and .source_probes.denial_audit_index_module_present == true
  and .source_probes.denial_readback_gate_present == true
  and .source_probes.denial_readback_points_here == true
  and .source_probes.denial_readback_ready_present == true
  and .source_probes.denial_readback_non_authoritative_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index --lib

echo "Hepta WorkGraph agent_jobs + task_board feature-flag enablement precondition denial audit index gate passed"
