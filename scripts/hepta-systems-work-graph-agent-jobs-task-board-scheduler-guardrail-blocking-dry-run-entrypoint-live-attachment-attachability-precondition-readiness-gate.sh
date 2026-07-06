#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-attachability-precondition-readiness-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-attachability-precondition-readiness-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_v1"
  and .preview_mode == "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_report_only"
  and .source_terminal_no_attachment_final_closeout_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_terminal_no_attachment_final_closeout_gate"
  and .source_final_closeout_entry_count == 9
  and .source_final_closeout_blocker_count == 45
  and .source_required_prior_gate_count == 20
  and .attachability_entrypoint_count == 4
  and .attachability_precondition_check_count == 16
  and .attachability_precondition_satisfied_count == 7
  and .attachability_precondition_unsatisfied_count == 9
  and .blocking_precondition_count == 9
  and .attachability_blocker_count == 50
  and .required_prior_gate_count == 21
  and (.attachability_entrypoints | map(.id) == [
    "spawn_agent",
    "spawn_agents_on_csv",
    "task_board_claim",
    "worker_task_run"
  ])
  and (.attachability_entrypoints | all(
    .attachability_status == "candidate_but_blocked"
    and .live_attachment_candidate == true
    and .live_attachment_allowed == false
    and .report_only == true
    and .runtime_interception_allowed == false
    and (.required_contracts | length == 5)
  ))
  and (.attachability_precondition_checks | map(.id) == [
    "terminal_no_attachment_final_closeout_ready",
    "entrypoint_inventory_stable",
    "hardening_contracts_visible",
    "denial_readback_chain_visible",
    "deterministic_decision_keys_visible",
    "trace_evidence_fields_visible",
    "shadow_replay_no_execution_closeout_visible",
    "live_hook_authorization_missing",
    "runtime_interception_authorization_missing",
    "scheduler_guardrail_enforcement_authorization_missing",
    "work_graph_persistence_authorization_missing",
    "task_result_live_acceptance_missing",
    "lease_work_start_authorization_missing",
    "config_flag_traffic_authorization_missing",
    "operator_approval_cutover_authorization_missing",
    "replay_rollback_rehearsal_execution_missing"
  ])
  and (.attachability_precondition_checks | all(.required == true and (.satisfied == true or .blocking == true)))
  and (.attachability_precondition_checks | map(select(.satisfied == true)) | length == 7)
  and (.attachability_precondition_checks | map(select(.blocking == true)) | length == 9)
  and (.attachability_blockers | all(.blocked == true))
  and (.attachability_blockers | map(.blocked_action) | index("record_live_attachment_attachability_precondition_readiness") != null)
  and (.attachability_blockers | map(.blocked_action) | index("persist_live_attachment_attachability_precondition_readiness") != null)
  and (.attachability_blockers | map(.blocked_action) | index("accept_live_attachment_attachability_precondition_readiness") != null)
  and (.attachability_blockers | map(.blocked_action) | index("attach_live_blocking_hook_to_entrypoints") != null)
  and (.attachability_blockers | map(.blocked_action) | index("promote_attachability_readiness_to_live") != null)
  and (.attachability_blockers | map(.blocked_action) | index("enable_live_attachment") != null)
  and (.attachability_blockers | map(.blocked_action) | index("install_live_blocking_hook") != null)
  and (.attachability_blockers | map(.blocked_action) | index("enable_runtime_interception") != null)
  and (.attachability_blockers | map(.blocked_action) | index("enforce_scheduler_admission") != null)
  and (.attachability_blockers | map(.blocked_action) | index("enable_guardrail_enforcement") != null)
  and (.attachability_blockers | map(.blocked_action) | index("persist_work_graph_event") != null)
  and (.attachability_blockers | map(.blocked_action) | index("spawn_agent") != null)
  and (.attachability_blockers | map(.blocked_action) | index("spawn_agents_on_csv") != null)
  and (.attachability_blockers | map(.blocked_action) | index("claim_task_board_work") != null)
  and (.attachability_blockers | map(.blocked_action) | index("run_worker_task") != null)
  and (.attachability_blockers | map(.blocked_action) | index("emit_live_task_result") != null)
  and (.attachability_blockers | map(.blocked_action) | index("perform_live_cutover") != null)
  and .required_prior_gates[0] == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_precondition_matrix_denial_readback_terminal_no_attachment_final_closeout_gate"
  and (.required_prior_gates | length == 21)
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_gate"
  and .readiness_mode == "identify_live_attachment_candidates_but_keep_attachment_blocked_until_all_priors_authorized"
  and .readiness_visible == true
  and .readiness_recorded == false
  and .readiness_persisted == false
  and .readiness_authoritative == false
  and .readiness_accepted == false
  and .terminal_no_attachment_branch_closed == true
  and .attachability_candidates_identified == true
  and .attachability_preconditions_satisfied == false
  and .source_final_closeout_visible == true
  and .source_final_closeout_persisted == false
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
  and .ready_for_attachability_precondition_readiness_readback == true
  and .ready_for_live_attachment == false
  and .ready_for_live_execution == false
  and .source_probes.readiness_module_present == true
  and .source_probes.terminal_final_closeout_gate_present == true
  and .source_probes.terminal_final_closeout_points_here == true
  and .source_probes.terminal_final_closeout_ready_present == true
  and .source_probes.terminal_final_closeout_no_attachment_present == true
  and .source_probes.terminal_final_closeout_no_live_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness --lib

echo "Hepta WorkGraph agent_jobs + task_board scheduler guardrail blocking dry-run entrypoint live attachment attachability precondition readiness gate passed"
