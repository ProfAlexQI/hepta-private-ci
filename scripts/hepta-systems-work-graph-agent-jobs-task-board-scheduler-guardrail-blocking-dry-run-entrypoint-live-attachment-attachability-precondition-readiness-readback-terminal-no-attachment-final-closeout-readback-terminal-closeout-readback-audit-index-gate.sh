#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-attachability-precondition-readiness-readback-terminal-no-attachment-final-closeout-readback-terminal-closeout-readback-audit-index-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-attachability-precondition-readiness-readback-terminal-no-attachment-final-closeout-readback-terminal-closeout-readback-audit-index-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_v1"
  and .preview_mode == "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_report_only"
  and .source_terminal_closeout_readback_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_gate"
  and .source_readback_entry_count == 7
  and .source_readback_blocker_count == 77
  and .source_required_prior_gate_count == 30
  and .source_terminal_closeout_readback_ready == true
  and .source_terminal_closeout_readback_no_persistence_confirmed == true
  and .source_terminal_closeout_readback_no_live_confirmed == true
  and .source_terminal_closeout_readback_ready_for_audit_index == true
  and .audit_index_entry_count == 9
  and .audit_index_blocker_count == 80
  and .required_prior_gate_count == 31
  and .audit_index_scope.id == "agent_jobs_task_board_scheduler_guardrail_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_scope"
  and .audit_index_scope.source_surface_id == "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback"
  and .audit_index_scope.index_mode == "live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_report_only"
  and .audit_index_scope.index_visible == true
  and .audit_index_scope.index_recorded == false
  and .audit_index_scope.index_persisted == false
  and .audit_index_scope.index_authoritative == false
  and .audit_index_scope.index_accepted == false
  and .audit_index_scope.live_acceptance_allowed == false
  and (.audit_index_entries | map(.id) == [
    "terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_scope_audit_index",
    "terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_entries_audit_index",
    "terminal_no_attachment_final_closeout_readback_terminal_closeout_source_summary_audit_index",
    "terminal_no_attachment_final_closeout_readback_terminal_closeout_blocker_inventory_audit_index",
    "terminal_no_attachment_final_closeout_readback_terminal_closeout_prior_chain_audit_index",
    "terminal_no_attachment_final_closeout_readback_terminal_closeout_non_persistence_boundary_audit_index",
    "terminal_no_attachment_final_closeout_readback_terminal_closeout_no_live_authority_audit_index",
    "terminal_no_attachment_final_closeout_readback_terminal_closeout_branch_state_audit_index",
    "terminal_no_attachment_final_closeout_readback_terminal_closeout_trace_evidence_audit_index"
  ])
  and (.audit_index_entries | all(
    .indexed == true
    and .recorded == false
    and .persisted == false
    and .authoritative == false
    and .accepted == false
    and .mutation_allowed == false
    and .ready == true
  ))
  and (.audit_index_blockers | map(.blocked_action) | index("record_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("persist_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("accept_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("record_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("persist_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("accept_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("record_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("persist_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("accept_live_attachment_attachability_terminal_no_attachment_final_closeout_readback_terminal_closeout") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("enable_live_attachment") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("install_live_blocking_hook") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("enable_runtime_interception") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("enforce_scheduler_admission") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("enable_guardrail_enforcement") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("persist_work_graph_event") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("spawn_agent") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("spawn_agents_on_csv") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("claim_task_board_work") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("run_worker_task") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("emit_live_task_result") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("execute_readback") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("execute_replay") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("execute_rollback") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("write_config") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("mutate_feature_flag") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("route_canary_traffic") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("request_operator_review") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("record_operator_approval") != null)
  and (.audit_index_blockers | map(.blocked_action) | index("perform_live_cutover") != null)
  and (.audit_index_blockers | all(.blocked == true and .required_before_acceptance == true))
  and .required_prior_gates[0] == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_gate"
  and (.required_prior_gates | length == 31)
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_readback_gate"
  and .audit_index_scope_complete == true
  and .audit_index_entries_complete == true
  and .audit_index_blockers_complete == true
  and .audit_index_preconditions_complete == true
  and .audit_index_visible == true
  and .audit_index_recorded == false
  and .audit_index_persisted == false
  and .audit_index_authoritative == false
  and .audit_index_accepted == false
  and .source_terminal_closeout_readback_visible == true
  and .source_terminal_closeout_readback_recorded == false
  and .source_terminal_closeout_readback_persisted == false
  and .source_terminal_closeout_readback_authoritative == false
  and .source_terminal_closeout_readback_accepted == false
  and .source_terminal_closeout_visible == true
  and .source_terminal_closeout_recorded == false
  and .source_terminal_closeout_persisted == false
  and .source_terminal_closeout_authoritative == false
  and .source_terminal_closeout_accepted == false
  and .terminal_no_attachment_branch_closed == true
  and .audit_index_authorizes_terminal_closeout_readback_recording == false
  and .audit_index_authorizes_terminal_closeout_readback_persistence == false
  and .audit_index_authorizes_terminal_closeout_recording == false
  and .audit_index_authorizes_terminal_closeout_persistence == false
  and .audit_index_authorizes_attachability_readback_recording == false
  and .audit_index_authorizes_attachability_readback_persistence == false
  and .audit_index_authorizes_live_attachment == false
  and .audit_index_authorizes_live_blocking_hook == false
  and .audit_index_authorizes_runtime_interception == false
  and .audit_index_authorizes_scheduler_admission_enforcement == false
  and .audit_index_authorizes_guardrail_enforcement == false
  and .audit_index_authorizes_work_graph_persistence == false
  and .audit_index_authorizes_projection_persistence == false
  and .audit_index_authorizes_lease_or_work_start == false
  and .audit_index_authorizes_agent_model_or_external_send == false
  and .audit_index_authorizes_live_task_result == false
  and .audit_index_authorizes_readback_replay_or_rollback == false
  and .audit_index_authorizes_config_flag_or_traffic == false
  and .audit_index_authorizes_operator_approval_or_live_cutover == false
  and .ready_for_non_persistence_readback == true
  and .ready_for_live_attachment == false
  and .ready_for_live_execution == false
  and .source_readbacks.terminal_closeout_readback_report_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_gate"
  and .source_readbacks.terminal_closeout_readback_preconditions_complete == true
  and .source_readbacks.terminal_closeout_readback_ready_for_audit_index == true
  and .source_readbacks.terminal_closeout_readback_no_persistence_confirmed == true
  and .source_readbacks.terminal_closeout_readback_no_live_confirmed == true
  and .source_readbacks.terminal_closeout_readback_side_effects_all_false == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

tests=(
  terminal_closeout_readback_audit_index_derives_from_readback
  terminal_closeout_readback_audit_index_is_visible_only
  terminal_closeout_readback_audit_index_blocks_live_paths
  terminal_closeout_readback_audit_index_links_priors_and_side_effects
)

for test_name in "${tests[@]}"; do
  cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
    "$test_name" --lib
done

echo "Hepta WorkGraph agent_jobs + task_board scheduler guardrail blocking dry-run entrypoint live attachment attachability terminal no-attachment final closeout readback terminal closeout readback audit index gate passed"
