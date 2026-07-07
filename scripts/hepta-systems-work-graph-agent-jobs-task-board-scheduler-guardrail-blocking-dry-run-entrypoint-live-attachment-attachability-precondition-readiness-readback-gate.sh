#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-attachability-precondition-readiness-readback-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-attachability-precondition-readiness-readback-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_v1"
  and .preview_mode == "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_report_only"
  and .source_attachability_readiness_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_gate"
  and .source_attachability_entrypoint_count == 4
  and .source_attachability_precondition_check_count == 16
  and .source_attachability_blocker_count == 50
  and .source_required_prior_gate_count == 21
  and .source_attachability_readiness_ready == true
  and .source_attachability_readiness_no_persistence_confirmed == true
  and .source_attachability_readiness_no_live_confirmed == true
  and .source_attachability_readiness_ready_for_readback == true
  and .readback_entry_count == 7
  and .entrypoint_readback_count == 4
  and .readback_blocker_count == 53
  and .required_prior_gate_count == 22
  and .readback_scope.source_surface_id == "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_attachability_precondition_readiness"
  and .readback_scope.readback_mode == "live_attachment_attachability_precondition_readiness_visible_only_readback"
  and .readback_scope.visible == true
  and .readback_scope.recorded == false
  and .readback_scope.persisted == false
  and .readback_scope.authoritative == false
  and .readback_scope.accepted == false
  and .readback_scope.mutation_allowed == false
  and (.readback_entries | map(.id) == [
    "attachability_readiness_surface_readback",
    "attachability_entrypoint_inventory_readback",
    "attachability_precondition_summary_readback",
    "attachability_blocker_inventory_readback",
    "attachability_prior_chain_readback",
    "attachability_non_persistence_boundary_readback",
    "attachability_no_live_authority_readback"
  ])
  and (.readback_entries | all(
    .visible == true
    and .recorded == false
    and .persisted == false
    and .accepted == false
    and .authoritative == false
    and .mutation_allowed == false
  ))
  and (.entrypoint_readbacks | map(.source_entrypoint_id) == [
    "spawn_agent",
    "spawn_agents_on_csv",
    "task_board_claim",
    "worker_task_run"
  ])
  and (.entrypoint_readbacks | all(
    .live_attachment_candidate == true
    and .live_attachment_allowed == false
    and .report_only == true
    and .readback_recorded == false
    and .readback_persisted == false
  ))
  and (.readback_blockers | all(.blocked == true))
  and (.readback_blockers | map(.blocked_action) | index("record_live_attachment_attachability_readback") != null)
  and (.readback_blockers | map(.blocked_action) | index("persist_live_attachment_attachability_readback") != null)
  and (.readback_blockers | map(.blocked_action) | index("accept_live_attachment_attachability_readback") != null)
  and (.readback_blockers | map(.blocked_action) | index("record_live_attachment_attachability_precondition_readiness") != null)
  and (.readback_blockers | map(.blocked_action) | index("attach_live_blocking_hook_to_entrypoints") != null)
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
  and (.readback_blockers | map(.blocked_action) | index("perform_live_cutover") != null)
  and .required_prior_gates[0] == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_gate"
  and (.required_prior_gates | length == 22)
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_gate"
  and .source_readiness_visible == true
  and .source_readiness_persisted == false
  and .readback_visible == true
  and .readback_recorded == false
  and .readback_persisted == false
  and .readback_authoritative == false
  and .readback_accepted == false
  and .readback_scope_visible_only_complete == true
  and .readback_entries_complete == true
  and .entrypoint_readbacks_complete == true
  and .readback_blockers_complete == true
  and .attachability_readback_preconditions_complete == true
  and .attachability_candidates_readback_ready == true
  and .attachability_preconditions_satisfied == false
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
  and .ready_for_attachability_readback_audit_index == true
  and .ready_for_live_attachment == false
  and .ready_for_live_execution == false
  and .source_readbacks.attachability_readiness_report_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_gate"
  and .source_readbacks.attachability_readiness_preconditions_complete == true
  and .source_readbacks.attachability_readiness_ready_for_readback == true
  and .source_readbacks.attachability_readiness_no_persistence_confirmed == true
  and .source_readbacks.attachability_readiness_no_live_confirmed == true
  and .source_readbacks.attachability_readiness_side_effects_all_false == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

tests=(
  live_attachment_attachability_readback_derives_from_readiness
  live_attachment_attachability_readback_is_visible_only
  live_attachment_attachability_readback_blocks_live_paths
  live_attachment_attachability_readback_links_priors_and_side_effects
)

for test_name in "${tests[@]}"; do
  cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
    "$test_name" --lib
done

echo "Hepta WorkGraph agent_jobs + task_board scheduler guardrail blocking dry-run entrypoint live attachment attachability precondition readiness readback gate passed"
