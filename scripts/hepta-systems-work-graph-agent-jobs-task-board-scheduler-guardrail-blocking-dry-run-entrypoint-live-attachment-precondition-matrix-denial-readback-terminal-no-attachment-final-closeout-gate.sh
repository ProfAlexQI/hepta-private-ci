#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-precondition-matrix-denial-readback-terminal-no-attachment-final-closeout-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-precondition-matrix-denial-readback-terminal-no-attachment-final-closeout-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_terminal_no_attachment_final_closeout_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_terminal_no_attachment_final_closeout_v1"
  and .preview_mode == "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_terminal_no_attachment_final_closeout_report_only"
  and .source_non_persistence_readback_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_non_persistence_readback_gate"
  and .source_readback_entry_count == 6
  and .source_readback_blocker_count == 42
  and .source_required_prior_gate_count == 19
  and .source_non_persistence_readback_ready == true
  and .source_non_persistence_readback_no_persistence_confirmed == true
  and .source_non_persistence_readback_no_live_confirmed == true
  and .source_non_persistence_readback_ready_for_terminal_closeout == true
  and .final_closeout_entry_count == 9
  and .final_closeout_blocker_count == 45
  and .required_prior_gate_count == 20
  and .final_closeout_scope.source_surface_id == "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_precondition_matrix_denial_readback_audit_index_non_persistence_readback"
  and .final_closeout_scope.closeout_mode == "live_attachment_precondition_matrix_denial_readback_terminal_no_attachment_final_closeout_report_only"
  and .final_closeout_scope.visible == true
  and .final_closeout_scope.recorded == false
  and .final_closeout_scope.persisted == false
  and .final_closeout_scope.authoritative == false
  and .final_closeout_scope.accepted == false
  and .final_closeout_scope.terminal == true
  and .final_closeout_scope.mutation_allowed == false
  and (.final_closeout_entries | map(.id) == [
    "live_attachment_no_attachment_branch_final_closeout",
    "live_attachment_audit_index_surface_final_closeout",
    "live_attachment_audit_index_entry_inventory_final_closeout",
    "live_attachment_audit_index_blocker_inventory_final_closeout",
    "live_attachment_audit_index_prior_chain_final_closeout",
    "live_attachment_non_persistence_boundary_final_closeout",
    "live_attachment_no_live_authority_final_closeout",
    "live_attachment_entrypoint_scope_final_closeout",
    "live_attachment_scheduler_guardrail_boundary_final_closeout"
  ])
  and (.final_closeout_entries | all(
    .visible == true
    and .recorded == false
    and .persisted == false
    and .accepted == false
    and .authoritative == false
    and .mutation_allowed == false
    and .closed == true
  ))
  and (.final_closeout_blockers | all(.blocked == true))
  and (.final_closeout_blockers | map(.blocked_action) | index("record_live_attachment_terminal_no_attachment_final_closeout") != null)
  and (.final_closeout_blockers | map(.blocked_action) | index("enable_live_attachment") != null)
  and (.final_closeout_blockers | map(.blocked_action) | index("install_live_blocking_hook") != null)
  and (.final_closeout_blockers | map(.blocked_action) | index("enable_runtime_interception") != null)
  and (.final_closeout_blockers | map(.blocked_action) | index("enforce_scheduler_admission") != null)
  and (.final_closeout_blockers | map(.blocked_action) | index("enable_guardrail_enforcement") != null)
  and (.final_closeout_blockers | map(.blocked_action) | index("persist_work_graph_event") != null)
  and (.final_closeout_blockers | map(.blocked_action) | index("spawn_agent") != null)
  and (.final_closeout_blockers | map(.blocked_action) | index("spawn_agents_on_csv") != null)
  and (.final_closeout_blockers | map(.blocked_action) | index("claim_task_board_work") != null)
  and (.final_closeout_blockers | map(.blocked_action) | index("run_worker_task") != null)
  and (.final_closeout_blockers | map(.blocked_action) | index("emit_live_task_result") != null)
  and (.final_closeout_blockers | map(.blocked_action) | index("perform_live_cutover") != null)
  and .required_prior_gates[0] == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_non_persistence_readback_gate"
  and (.required_prior_gates | length == 20)
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_gate"
  and .terminal_no_attachment_branch_closed == true
  and .final_closeout_visible == true
  and .final_closeout_recorded == false
  and .final_closeout_persisted == false
  and .final_closeout_authoritative == false
  and .final_closeout_accepted == false
  and .source_audit_index_visible == true
  and .source_audit_index_persisted == false
  and .source_readback_persisted == false
  and .denial_readback_persisted == false
  and .final_closeout_scope_visible_only_complete == true
  and .final_closeout_entries_complete == true
  and .final_closeout_blockers_complete == true
  and .terminal_no_attachment_final_closeout_preconditions_complete == true
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
  and .hardening_decision_recording_allowed == false
  and .hardening_decision_persistence_allowed == false
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
  and .ready_for_live_attachment_attachability_precondition_readiness == true
  and .ready_for_live_attachment == false
  and .ready_for_live_execution == false
  and .source_readbacks.non_persistence_readback_report_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_audit_index_non_persistence_readback_gate"
  and .source_readbacks.non_persistence_readback_preconditions_complete == true
  and .source_readbacks.non_persistence_readback_ready_for_terminal_closeout == true
  and .source_readbacks.non_persistence_readback_no_persistence_confirmed == true
  and .source_readbacks.non_persistence_readback_no_live_confirmed == true
  and .source_readbacks.non_persistence_readback_side_effects_all_false == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

tests=(
  live_attachment_terminal_no_attachment_final_closeout_derives_from_readback
  live_attachment_terminal_no_attachment_final_closeout_is_visible_only
  live_attachment_terminal_no_attachment_final_closeout_blocks_live_paths
  live_attachment_terminal_no_attachment_final_closeout_links_priors_and_side_effects
)

for test_name in "${tests[@]}"; do
  cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
    "$test_name" --lib
done

echo "Hepta WorkGraph agent_jobs + task_board scheduler guardrail blocking dry-run entrypoint live attachment denial readback terminal no-attachment final closeout gate passed"
