#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-live-attachment-terminal-closeout-readback-audit-index-non-persistence-final-closeout-readback-audit-index-non-persistence-readback-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-live-attachment-terminal-closeout-readback-audit-index-non-persistence-final-closeout-readback-audit-index-non-persistence-readback-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback_v1"
  and .preview_mode == "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback_only"
  and .source_audit_index_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_gate"
  and .source_audit_index_entry_count == 9
  and .source_audit_index_blocker_count == 92
  and .source_required_prior_gate_count == 35
  and .source_audit_index_ready == true
  and .source_audit_index_no_persistence_confirmed == true
  and .source_audit_index_no_live_confirmed == true
  and .source_audit_index_ready_for_non_persistence_readback == true
  and .readback_entry_count == 6
  and .readback_blocker_count == 95
  and .required_prior_gate_count == 36
  and .readback_scope.source_surface_id == "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index"
  and .readback_scope.readback_mode == "live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback_only"
  and .readback_scope.audit_index_visible == true
  and .readback_scope.audit_index_recorded == false
  and .readback_scope.audit_index_persisted == false
  and .readback_scope.audit_index_authoritative == false
  and .readback_scope.audit_index_accepted == false
  and .readback_scope.readback_recorded == false
  and .readback_scope.readback_persisted == false
  and .readback_scope.readback_accepted == false
  and (.readback_entries | map(.id) == [
    "terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_surface_non_persistence_readback",
    "terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_entry_inventory_non_persistence_readback",
    "terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_blocker_inventory_non_persistence_readback",
    "terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_prior_chain_non_persistence_readback",
    "terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_boundary_readback",
    "terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_no_live_authority_readback"
  ])
  and (.readback_entries | all(
    .visible == true
    and .recorded == false
    and .persisted == false
    and .authoritative == false
    and .accepted == false
    and .mutation_allowed == false
    and .ready == true
  ))
  and (.readback_blockers | map(.blocked_action) | index("record_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback") != null)
  and (.readback_blockers | map(.blocked_action) | index("persist_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback") != null)
  and (.readback_blockers | map(.blocked_action) | index("accept_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback") != null)
  and (.readback_blockers | map(.blocked_action) | index("record_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index") != null)
  and (.readback_blockers | map(.blocked_action) | index("persist_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index") != null)
  and (.readback_blockers | map(.blocked_action) | index("accept_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index") != null)
  and (.readback_blockers | map(.blocked_action) | index("record_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback") != null)
  and (.readback_blockers | map(.blocked_action) | index("persist_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback") != null)
  and (.readback_blockers | map(.blocked_action) | index("accept_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback") != null)
  and (.readback_blockers | map(.blocked_action) | index("enable_live_attachment") != null)
  and (.readback_blockers | map(.blocked_action) | index("install_live_blocking_hook") != null)
  and (.readback_blockers | map(.blocked_action) | index("enable_runtime_interception") != null)
  and (.readback_blockers | map(.blocked_action) | index("enforce_scheduler_admission") != null)
  and (.readback_blockers | map(.blocked_action) | index("enable_guardrail_enforcement") != null)
  and (.readback_blockers | map(.blocked_action) | index("persist_work_graph_event") != null)
  and (.readback_blockers | map(.blocked_action) | index("spawn_agent") != null)
  and (.readback_blockers | map(.blocked_action) | index("spawn_agents_on_csv") != null)
  and (.readback_blockers | map(.blocked_action) | index("claim_task_board_work") != null)
  and (.readback_blockers | map(.blocked_action) | index("run_worker_task") != null)
  and (.readback_blockers | map(.blocked_action) | index("emit_live_task_result") != null)
  and (.readback_blockers | map(.blocked_action) | index("execute_readback") != null)
  and (.readback_blockers | map(.blocked_action) | index("execute_replay") != null)
  and (.readback_blockers | map(.blocked_action) | index("execute_rollback") != null)
  and (.readback_blockers | map(.blocked_action) | index("write_config") != null)
  and (.readback_blockers | map(.blocked_action) | index("mutate_feature_flag") != null)
  and (.readback_blockers | map(.blocked_action) | index("route_canary_traffic") != null)
  and (.readback_blockers | map(.blocked_action) | index("request_operator_review") != null)
  and (.readback_blockers | map(.blocked_action) | index("record_operator_approval") != null)
  and (.readback_blockers | map(.blocked_action) | index("perform_live_cutover") != null)
  and (.readback_blockers | all(.blocked == true))
  and .required_prior_gates[0] == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_gate"
  and (.required_prior_gates | length == 36)
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_gate"
  and .readback_scope_complete == true
  and .readback_entries_complete == true
  and .readback_blockers_complete == true
  and .non_persistence_readback_preconditions_complete == true
  and .audit_index_visible == true
  and .audit_index_recorded == false
  and .audit_index_persisted == false
  and .audit_index_authoritative == false
  and .audit_index_accepted == false
  and .source_final_closeout_readback_visible == true
  and .source_final_closeout_readback_recorded == false
  and .source_final_closeout_readback_persisted == false
  and .source_final_closeout_readback_authoritative == false
  and .source_final_closeout_readback_accepted == false
  and .source_final_closeout_visible == true
  and .source_final_closeout_recorded == false
  and .source_final_closeout_persisted == false
  and .source_final_closeout_authoritative == false
  and .source_final_closeout_accepted == false
  and .terminal_no_attachment_branch_closed == true
  and .audit_index_readback_recorded == false
  and .audit_index_readback_persisted == false
  and .audit_index_readback_accepted == false
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
  and .ready_for_terminal_closeout_readback_audit_index_final_closeout == true
  and .ready_for_live_attachment == false
  and .ready_for_live_execution == false
  and .source_readbacks.audit_index_report_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_gate"
  and .source_readbacks.audit_index_preconditions_complete == true
  and .source_readbacks.audit_index_ready_for_non_persistence_readback == true
  and .source_readbacks.audit_index_no_persistence_confirmed == true
  and .source_readbacks.audit_index_no_live_confirmed == true
  and .source_readbacks.audit_index_side_effects_all_false == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

tests=(
  wg_sg_live_attach_tc_readback_ai_np_fc_readback_ai_np_readback::tests::final_closeout_readback_audit_index_non_persistence_readback_derives_from_index
  wg_sg_live_attach_tc_readback_ai_np_fc_readback_ai_np_readback::tests::final_closeout_readback_audit_index_non_persistence_readback_is_visible_only
  wg_sg_live_attach_tc_readback_ai_np_fc_readback_ai_np_readback::tests::final_closeout_readback_audit_index_non_persistence_readback_blocks_live_paths
  wg_sg_live_attach_tc_readback_ai_np_fc_readback_ai_np_readback::tests::final_closeout_readback_audit_index_non_persistence_readback_links_priors_and_side_effects
)

for test_name in "${tests[@]}"; do
  cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
    "$test_name" --lib
done

echo "Hepta WorkGraph live attachment terminal closeout readback audit index non-persistence final closeout readback audit-index non-persistence readback gate passed"
