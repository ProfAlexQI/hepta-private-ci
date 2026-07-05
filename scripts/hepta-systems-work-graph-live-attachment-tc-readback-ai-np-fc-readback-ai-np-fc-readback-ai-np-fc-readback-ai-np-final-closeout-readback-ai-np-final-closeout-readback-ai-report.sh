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

SOURCE_REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-live-attachment-tc-readback-ai-np-fc-readback-ai-np-fc-readback-ai-np-fc-readback-ai-np-final-closeout-readback-ai-np-final-closeout-readback-report.sh"
SOURCE_MODULE="codex-rs/hepta-runtime/src/wg_sg_live_attach_tc_readback_ai_np_fc_readback_ai_np_final_closeout_readback_audit_index_np_final_closeout_readback_audit_index_np_final_closeout_readback_audit_index_np_final_closeout_readback.rs"

source_report="$("$SOURCE_REPORT_SCRIPT")"

audit_index_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/wg_sg_live_attach_tc_readback_ai_np_fc_readback_ai_np_final_closeout_readback_audit_index_np_final_closeout_readback_audit_index_np_final_closeout_readback_audit_index_np_final_closeout_readback_audit_index.rs
)"
final_closeout_readback_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-live-attachment-tc-readback-ai-np-fc-readback-ai-np-fc-readback-ai-np-fc-readback-ai-np-final-closeout-readback-gate.sh
)"
final_closeout_readback_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_gate" \
    "$SOURCE_MODULE"
)"
final_closeout_readback_ready_present="$(
  bool_for source_has "ready_for_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index" "$SOURCE_MODULE"
)"
final_closeout_readback_no_live_present="$(
  bool_for source_has "ready_for_live_execution: false" "$SOURCE_MODULE"
)"
final_closeout_readback_unpersisted_present="$(
  bool_for source_has "final_closeout_readback_persisted: false" "$SOURCE_MODULE"
)"

jq -n \
  --argjson source "$source_report" \
  --argjson audit_index_module_present "$audit_index_module_present" \
  --argjson final_closeout_readback_gate_present "$final_closeout_readback_gate_present" \
  --argjson final_closeout_readback_points_here "$final_closeout_readback_points_here" \
  --argjson final_closeout_readback_ready_present "$final_closeout_readback_ready_present" \
  --argjson final_closeout_readback_no_live_present "$final_closeout_readback_no_live_present" \
  --argjson final_closeout_readback_unpersisted_present "$final_closeout_readback_unpersisted_present" \
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
    reason: "required before final closeout readback audit index can be recorded, accepted, enforced, or cut live",
    required_before_acceptance: true
  };
  [
    entry("final_closeout_readback_audit_index_scope"; "final_closeout_readback.audit_index.scope"; "final_closeout_readback_surface"; "final_closeout_readback_scope"),
    entry("final_closeout_readback_audit_index_entries"; "final_closeout_readback.audit_index.entries"; "final_closeout_readback_entries"; "readback_entries"),
    entry("final_closeout_readback_audit_index_source_summary"; "final_closeout_readback.audit_index.source_summary"; "final_closeout_readback_surface"; "source_final_closeout_summary"),
    entry("final_closeout_readback_audit_index_blockers"; "final_closeout_readback.audit_index.blockers"; "final_closeout_readback_blockers"; "blocker_inventory"),
    entry("final_closeout_readback_audit_index_priors"; "final_closeout_readback.audit_index.priors"; "final_closeout_readback_priors"; "prior_chain"),
    entry("final_closeout_readback_audit_index_non_persistence"; "final_closeout_readback.audit_index.non_persistence"; "final_closeout_readback_boundary"; "non_persistence_boundary"),
    entry("final_closeout_readback_audit_index_no_live"; "final_closeout_readback.audit_index.no_live"; "final_closeout_readback_no_live_authority"; "no_live_authority"),
    entry("final_closeout_readback_audit_index_branch_state"; "final_closeout_readback.audit_index.branch_state"; "final_closeout_readback_surface"; "terminal_branch_state"),
    entry("final_closeout_readback_audit_index_trace"; "final_closeout_readback.audit_index.trace"; "record_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback"; "trace_evidence")
  ] as $entries
  | (
    [
      blocker("final_closeout_readback_audit_index_record_blocked"; "record_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index"),
      blocker("final_closeout_readback_audit_index_persistence_blocked"; "persist_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index"),
      blocker("final_closeout_readback_audit_index_acceptance_blocked"; "accept_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index")
    ]
    + ($source.readback_blockers | map(blocker(.id; .blocked_action)))
  ) as $blockers
  | ([$source.gate] + $source.required_prior_gates) as $required_priors
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_gate",
      schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_v1",
      preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_report_only",
      source_final_closeout_readback_gate: $source.gate,
      source_readback_entry_count: $source.readback_entry_count,
      source_readback_blocker_count: $source.readback_blocker_count,
      source_required_prior_gate_count: $source.required_prior_gate_count,
      audit_index_entry_count: ($entries | length),
      audit_index_blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_priors | length),
      audit_index_entries: $entries,
      audit_index_blockers: $blockers,
      required_prior_gates: $required_priors,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_live_attachment_attachability_precondition_readiness_readback_terminal_no_attachment_final_closeout_readback_terminal_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_final_closeout_readback_audit_index_non_persistence_readback_gate",
      audit_index_visible: true,
      audit_index_recorded: false,
      audit_index_persisted: false,
      audit_index_authoritative: false,
      audit_index_accepted: false,
      source_final_closeout_readback_visible: $source.final_closeout_readback_visible,
      source_final_closeout_readback_recorded: $source.final_closeout_readback_recorded,
      source_final_closeout_readback_persisted: $source.final_closeout_readback_persisted,
      source_final_closeout_readback_authoritative: $source.final_closeout_readback_authoritative,
      source_final_closeout_readback_accepted: $source.final_closeout_readback_accepted,
      source_final_closeout_visible: $source.source_final_closeout_visible,
      source_final_closeout_recorded: $source.source_final_closeout_recorded,
      source_final_closeout_persisted: $source.source_final_closeout_persisted,
      source_final_closeout_authoritative: $source.source_final_closeout_authoritative,
      source_final_closeout_accepted: $source.source_final_closeout_accepted,
      source_prior_audit_index_visible: $source.source_audit_index_visible,
      source_prior_audit_index_recorded: $source.source_audit_index_recorded,
      source_prior_audit_index_persisted: $source.source_audit_index_persisted,
      source_prior_audit_index_authoritative: $source.source_audit_index_authoritative,
      source_prior_audit_index_accepted: $source.source_audit_index_accepted,
      source_prior_audit_index_readback_recorded: $source.source_audit_index_readback_recorded,
      source_prior_audit_index_readback_persisted: $source.source_audit_index_readback_persisted,
      source_prior_audit_index_readback_accepted: $source.source_audit_index_readback_accepted,
      terminal_no_attachment_branch_closed: $source.terminal_no_attachment_branch_closed,
      audit_index_authorizes_final_closeout_readback_recording: false,
      audit_index_authorizes_final_closeout_readback_persistence: false,
      audit_index_authorizes_final_closeout_recording: false,
      audit_index_authorizes_final_closeout_persistence: false,
      audit_index_authorizes_prior_audit_index_recording: false,
      audit_index_authorizes_prior_audit_index_persistence: false,
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
        final_closeout_readback_gate_present: $final_closeout_readback_gate_present,
        final_closeout_readback_points_here: $final_closeout_readback_points_here,
        final_closeout_readback_ready_present: $final_closeout_readback_ready_present,
        final_closeout_readback_no_live_present: $final_closeout_readback_no_live_present,
        final_closeout_readback_unpersisted_present: $final_closeout_readback_unpersisted_present
      },
      side_effects: {
        filesystem_written: false,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_accepted: false,
        final_closeout_readback_recorded: false,
        final_closeout_readback_persisted: false,
        final_closeout_readback_accepted: false,
        final_closeout_recorded: false,
        final_closeout_persisted: false,
        final_closeout_accepted: false,
        prior_audit_index_recorded: false,
        prior_audit_index_persisted: false,
        prior_audit_index_accepted: false,
        prior_audit_index_readback_recorded: false,
        prior_audit_index_readback_persisted: false,
        prior_audit_index_readback_accepted: false,
        live_attachment_enabled: false,
        live_blocking_hook_installed: false,
        runtime_interception_enabled: false,
        scheduler_admission_enforced: false,
        guardrail_enforcement_enabled: false,
        work_graph_event_persisted: false,
        projection_index_persisted: false,
        lease_acquired: false,
        work_started: false,
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
