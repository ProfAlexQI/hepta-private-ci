#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

path_exists() {
  local path="$1"
  [[ -e "$path" ]]
}

source_has() {
  local pattern="$1"
  local path="$2"
  rg -q "$pattern" "$path"
}

bool_for() {
  if "$@"; then
    printf 'true\n'
  else
    printf 'false\n'
  fi
}

SOURCE_REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-attachability-precondition-readiness-readback-report.sh"
source_report="$("$SOURCE_REPORT_SCRIPT")"

audit_index_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index.rs
)"
readback_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-live-attachment-attachability-precondition-readiness-readback-gate.sh
)"
readback_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback.rs
)"
readback_ready_present="$(
  bool_for source_has "ready_for_attachability_readback_audit_index: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback.rs
)"
readback_no_live_present="$(
  bool_for source_has "ready_for_live_execution: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback.rs
)"
readback_unpersisted_present="$(
  bool_for source_has "readback_persisted: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback.rs
)"

jq -n \
  --argjson source "$source_report" \
  --argjson audit_index_module_present "$audit_index_module_present" \
  --argjson readback_gate_present "$readback_gate_present" \
  --argjson readback_points_here "$readback_points_here" \
  --argjson readback_ready_present "$readback_ready_present" \
  --argjson readback_no_live_present "$readback_no_live_present" \
  --argjson readback_unpersisted_present "$readback_unpersisted_present" \
  '
  def entry($id; $key; $source; $category): {
    id: $id,
    stable_index_key: $key,
    source_readback_id: $source,
    audit_category: $category,
    indexed: true,
    recorded: false,
    persisted: false,
    authoritative: false,
    accepted: false,
    mutation_allowed: false,
    ready: true
  };
  def blocker($id; $action): {
    id: $id,
    blocked_action: $action,
    blocked: true,
    reason: "required before live attachment attachability readiness readback audit index can be recorded, accepted, enforced, or cut live",
    required_before_acceptance: true
  };
  {
    id: "agent_jobs_task_board_scheduler_guardrail_live_attachment_attachability_readback_audit_index_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.scheduler_guardrail.live_attachment_attachability_precondition_readiness_readback",
    index_mode: "live_attachment_attachability_precondition_readiness_readback_audit_index_report_only",
    stable_index_key: "work_graph.agent_jobs_task_board.scheduler_guardrail.live_attachment.attachability_precondition_readiness.readback.audit_index",
    index_visible: true,
    index_recorded: false,
    index_persisted: false,
    index_authoritative: false,
    index_accepted: false,
    live_acceptance_allowed: false
  } as $scope
  | [
    entry("attachability_readback_scope_audit_index"; "live_attachment_attachability_readback.audit_index.scope"; "agent_jobs_task_board_scheduler_guardrail_live_attachment_attachability_readiness_readback_scope"; "attachability_readback_scope"),
    entry("attachability_entrypoint_readbacks_audit_index"; "live_attachment_attachability_readback.audit_index.entrypoints"; "attachability_entrypoint_inventory_readback"; "entrypoint_readbacks"),
    entry("attachability_precondition_summary_audit_index"; "live_attachment_attachability_readback.audit_index.preconditions"; "attachability_precondition_summary_readback"; "precondition_summary"),
    entry("attachability_blocker_inventory_audit_index"; "live_attachment_attachability_readback.audit_index.blockers"; "attachability_blocker_inventory_readback"; "blocker_inventory"),
    entry("attachability_prior_chain_audit_index"; "live_attachment_attachability_readback.audit_index.prior_chain"; "attachability_prior_chain_readback"; "prior_chain"),
    entry("attachability_non_persistence_boundary_audit_index"; "live_attachment_attachability_readback.audit_index.non_persistence_boundary"; "attachability_non_persistence_boundary_readback"; "non_persistence_boundary"),
    entry("attachability_no_live_authority_audit_index"; "live_attachment_attachability_readback.audit_index.no_live_authority"; "attachability_no_live_authority_readback"; "no_live_authority"),
    entry("attachability_candidate_surface_audit_index"; "live_attachment_attachability_readback.audit_index.candidate_surface"; "attachability_readiness_surface_readback"; "candidate_surface"),
    entry("attachability_readiness_trace_evidence_audit_index"; "live_attachment_attachability_readback.audit_index.trace_evidence"; "live_attachment_attachability_trace_evidence_field"; "trace_evidence")
  ] as $entries
  | (
    [
      blocker("audit_index_record_blocked"; "record_live_attachment_attachability_readback_audit_index"),
      blocker("audit_index_persistence_blocked"; "persist_live_attachment_attachability_readback_audit_index"),
      blocker("audit_index_acceptance_blocked"; "accept_live_attachment_attachability_readback_audit_index")
    ]
    + ($source.readback_blockers | map(blocker(.id; .blocked_action)))
  ) as $blockers
  | ([$source.gate] + $source.required_prior_gates) as $required_priors
  | {
    product: "Hepta",
    runtime: "hepta",
    status: "ready",
    gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_gate",
    schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_v1",
    preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_report_only",
    source_attachability_readback_gate: $source.gate,
    source_readback_entry_count: $source.readback_entry_count,
    source_entrypoint_readback_count: $source.entrypoint_readback_count,
    source_readback_blocker_count: $source.readback_blocker_count,
    source_required_prior_gate_count: $source.required_prior_gate_count,
    audit_index_entry_count: ($entries | length),
    audit_index_blocker_count: ($blockers | length),
    required_prior_gate_count: ($required_priors | length),
    audit_index_scope: $scope,
    audit_index_entries: $entries,
    audit_index_blockers: $blockers,
    required_prior_gates: $required_priors,
    recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_audit_index_non_persistence_readback_gate",
    audit_index_visible: true,
    audit_index_recorded: false,
    audit_index_persisted: false,
    audit_index_authoritative: false,
    audit_index_accepted: false,
    source_readback_visible: $source.readback_visible,
    source_readback_recorded: false,
    source_readback_persisted: false,
    source_readback_authoritative: false,
    source_readback_accepted: false,
    audit_index_authorizes_attachability_readback_recording: false,
    audit_index_authorizes_attachability_readback_persistence: false,
    audit_index_authorizes_attachability_readiness_recording: false,
    audit_index_authorizes_attachability_readiness_persistence: false,
    audit_index_authorizes_live_attachment: false,
    audit_index_authorizes_live_blocking_hook: false,
    audit_index_authorizes_runtime_interception: false,
    audit_index_authorizes_scheduler_admission_enforcement: false,
    audit_index_authorizes_guardrail_enforcement: false,
    audit_index_authorizes_work_graph_persistence: false,
    audit_index_authorizes_projection_persistence: false,
    audit_index_authorizes_lease_or_work_start: false,
    audit_index_authorizes_agent_model_or_external_send: false,
    audit_index_authorizes_live_task_result: false,
    audit_index_authorizes_readback_replay_or_rollback: false,
    audit_index_authorizes_config_flag_or_traffic: false,
    audit_index_authorizes_operator_approval_or_live_cutover: false,
    ready_for_non_persistence_readback: true,
    ready_for_live_attachment: false,
    ready_for_live_execution: false,
    source_probes: {
      audit_index_module_present: $audit_index_module_present,
      readback_gate_present: $readback_gate_present,
      readback_points_here: $readback_points_here,
      readback_ready_present: $readback_ready_present,
      readback_no_live_present: $readback_no_live_present,
      readback_unpersisted_present: $readback_unpersisted_present
    },
    side_effects: {
      filesystem_written: false,
      audit_index_recorded: false,
      audit_index_persisted: false,
      audit_index_accepted: false,
      attachability_readback_recorded: false,
      attachability_readback_persisted: false,
      attachability_readback_accepted: false,
      attachability_readiness_recorded: false,
      attachability_readiness_persisted: false,
      attachability_readiness_accepted: false,
      live_attachment_enabled: false,
      live_blocking_hook_installed: false,
      runtime_interception_enabled: false,
      scheduler_admission_enforced: false,
      guardrail_enforcement_enabled: false,
      work_graph_event_persisted: false,
      projection_index_persisted: false,
      lease_acquired: false,
      work_started: false,
      hardening_decision_recorded: false,
      hardening_decision_persisted: false,
      live_task_result_emitted: false,
      readback_executed: false,
      replay_executed: false,
      replay_diff_recorded: false,
      replay_diff_persisted: false,
      rollback_executed: false,
      idempotency_index_mutated: false,
      config_written: false,
      feature_flag_mutated: false,
      canary_traffic_routed: false,
      operator_review_requested: false,
      approval_recorded: false,
      live_cutover_performed: false,
      runtime_mutation_performed: false,
      agent_spawn_performed: false,
      external_send_performed: false,
      model_invoked: false
    }
  }
  '
