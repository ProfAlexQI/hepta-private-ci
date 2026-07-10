#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-live-attachment-tc-readback-ai-np-fc-readback-ai-np-fc-readback-ai-np-fc-readback-ai-np-final-closeout-readback-ai-np-final-closeout-readback-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-live-attachment-tc-readback-ai-np-fc-readback-ai-np-fc-readback-ai-np-fc-readback-ai-np-final-closeout-readback-ai-np-final-closeout-readback-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_gate"
  and .source_final_closeout_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_gate"
  and .source_final_closeout_entry_count == 8
  and .source_final_closeout_blocker_count == 134
  and .source_required_prior_gate_count == 49
  and .readback_entry_count == 6
  and .readback_blocker_count == 137
  and .required_prior_gate_count == 50
  and (.readback_entries | all(.visible == true and .ready == true and .recorded == false and .persisted == false and .authoritative == false and .accepted == false and .mutation_allowed == false))
  and (.readback_blockers | all(.blocked == true))
  and (.readback_blockers | map(.blocked_action) | index("record_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback") != null)
  and (.readback_blockers | map(.blocked_action) | index("persist_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback") != null)
  and (.readback_blockers | map(.blocked_action) | index("accept_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback") != null)
  and (.readback_blockers | map(.blocked_action) | index("record_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout") != null)
  and (.readback_blockers | map(.blocked_action) | index("enable_live_attachment") != null)
  and (.readback_blockers | map(.blocked_action) | index("install_live_blocking_hook") != null)
  and (.readback_blockers | map(.blocked_action) | index("enable_runtime_interception") != null)
  and (.readback_blockers | map(.blocked_action) | index("enforce_scheduler_admission") != null)
  and (.readback_blockers | map(.blocked_action) | index("enable_guardrail_enforcement") != null)
  and (.readback_blockers | map(.blocked_action) | index("persist_work_graph_event") != null)
  and (.readback_blockers | map(.blocked_action) | index("spawn_agent") != null)
  and (.readback_blockers | map(.blocked_action) | index("execute_readback") != null)
  and (.readback_blockers | map(.blocked_action) | index("execute_replay") != null)
  and (.readback_blockers | map(.blocked_action) | index("execute_rollback") != null)
  and (.readback_blockers | map(.blocked_action) | index("write_config") != null)
  and (.readback_blockers | map(.blocked_action) | index("mutate_feature_flag") != null)
  and (.readback_blockers | map(.blocked_action) | index("route_canary_traffic") != null)
  and (.readback_blockers | map(.blocked_action) | index("request_operator_review") != null)
  and (.readback_blockers | map(.blocked_action) | index("record_operator_approval") != null)
  and (.readback_blockers | map(.blocked_action) | index("perform_live_cutover") != null)
  and .required_prior_gates[0] == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_gate"
  and (.required_prior_gates | length == 50)
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_gate"
  and .source_final_closeout_visible == true
  and .source_final_closeout_recorded == false
  and .source_final_closeout_persisted == false
  and .source_final_closeout_authoritative == false
  and .source_final_closeout_accepted == false
  and .final_closeout_readback_visible == true
  and .final_closeout_readback_recorded == false
  and .final_closeout_readback_persisted == false
  and .final_closeout_readback_authoritative == false
  and .final_closeout_readback_accepted == false
  and .source_audit_index_visible == true
  and .source_audit_index_recorded == false
  and .source_audit_index_persisted == false
  and .source_audit_index_authoritative == false
  and .source_audit_index_accepted == false
  and .source_audit_index_readback_recorded == false
  and .source_audit_index_readback_persisted == false
  and .source_audit_index_readback_accepted == false
  and .source_prior_final_closeout_readback_visible == true
  and .source_prior_final_closeout_readback_recorded == false
  and .source_prior_final_closeout_readback_persisted == false
  and .source_prior_final_closeout_readback_authoritative == false
  and .source_prior_final_closeout_readback_accepted == false
  and .terminal_no_attachment_branch_closed == true
  and .live_attachment_allowed == false
  and .live_blocking_hook_install_allowed == false
  and .runtime_interception_allowed == false
  and .scheduler_admission_enforcement_allowed == false
  and .guardrail_enforcement_allowed == false
  and .work_graph_event_persistence_allowed == false
  and .projection_persistence_allowed == false
  and .lease_acquisition_allowed == false
  and .work_start_allowed == false
  and .agent_spawn_allowed == false
  and .model_invocation_allowed == false
  and .external_send_allowed == false
  and .live_task_result_emission_allowed == false
  and .readback_execution_allowed == false
  and .replay_execution_allowed == false
  and .replay_diff_recording_allowed == false
  and .replay_diff_persistence_allowed == false
  and .rollback_execution_allowed == false
  and .idempotency_mutation_allowed == false
  and .config_write_allowed == false
  and .feature_flag_mutation_allowed == false
  and .canary_traffic_allowed == false
  and .operator_review_request_allowed == false
  and .approval_recording_allowed == false
  and .live_cutover_allowed == false
  and .ready_for_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index == true
  and .ready_for_live_attachment == false
  and .ready_for_live_execution == false
  and .source_probes.final_closeout_readback_module_present == true
  and .source_probes.final_closeout_gate_present == true
  and .source_probes.final_closeout_points_here == true
  and .source_probes.final_closeout_ready_present == true
  and .source_probes.final_closeout_no_live_present == true
  and .source_probes.final_closeout_unpersisted_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  wg_sg_live_attach_tc_readback_ai_np_fc_readback_ai_np_final_closeout_readback_audit_index_np_final_closeout_readback_audit_index_np_final_closeout_readback_audit_index_np_final_closeout_readback --lib

echo "Hepta WorkGraph live attachment terminal closeout readback audit index non-persistence final closeout readback audit-index non-persistence final closeout readback audit-index non-persistence final closeout readback audit-index non-persistence final closeout readback audit-index non-persistence final closeout readback gate passed"
