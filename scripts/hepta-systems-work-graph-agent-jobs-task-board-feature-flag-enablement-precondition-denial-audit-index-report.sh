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

denial_audit_index_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index.rs
)"
denial_readback_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-enablement-precondition-denial-readback-gate.sh
)"
denial_readback_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback.rs
)"
denial_readback_ready_present="$(
  bool_for source_has "ready_for_denial_audit_index: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback.rs
)"
denial_readback_non_authoritative_present="$(
  bool_for source_has "dry_run_denial_authoritative: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback.rs
)"

jq -n \
  --argjson denial_audit_index_module_present "$denial_audit_index_module_present" \
  --argjson denial_readback_gate_present "$denial_readback_gate_present" \
  --argjson denial_readback_points_here "$denial_readback_points_here" \
  --argjson denial_readback_ready_present "$denial_readback_ready_present" \
  --argjson denial_readback_non_authoritative_present "$denial_readback_non_authoritative_present" \
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
    mutation_allowed: false,
    ready: true
  };
  def blocker($id; $action; $reason): {
    id: $id,
    blocked_action: $action,
    blocked: true,
    reason: $reason
  };
  {
    id: "agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_scope",
    source_surface_id: "work_graph_agent_jobs_task_board.feature_flag.enablement_precondition_denial_readback",
    index_mode: "denial_audit_index_report_only",
    stable_index_key: "work_graph.agent_jobs_task_board.feature_flag.enablement_precondition.denial_audit_index",
    index_visible: true,
    index_recorded: false,
    index_persisted: false,
    index_authoritative: false,
    acceptance_allowed: false
  } as $audit_index_scope
  | [
    entry(
      "deny_decision_audit_index";
      "denial_audit_index.deny_decisions";
      "enablement_deny_decision_readback";
      "enablement_decision"
    ),
    entry(
      "deny_reason_catalog_audit_index";
      "denial_audit_index.deny_reason_catalog";
      "enablement_deny_reason_catalog_readback";
      "deny_reason_catalog"
    ),
    entry(
      "feature_flag_current_off_audit_index";
      "denial_audit_index.feature_flag_current_off";
      "feature_flag_current_off_readback";
      "feature_flag_boundary"
    ),
    entry(
      "scheduler_replay_rollback_audit_index";
      "denial_audit_index.scheduler_replay_rollback_boundaries";
      "scheduler_replay_rollback_boundary_readback";
      "scheduler_replay_rollback_boundary"
    ),
    entry(
      "live_cutover_denial_audit_index";
      "denial_audit_index.live_cutover_denial";
      "live_cutover_denial_boundary_readback";
      "live_cutover_boundary"
    ),
    entry(
      "required_prior_chain_audit_index";
      "denial_audit_index.required_prior_chain";
      "enablement_precondition_denial_readback_gate";
      "required_prior_chain"
    )
  ] as $audit_index_entries
  | [
    blocker("audit_index_record_blocked"; "record_denial_audit_index"; "denial audit index remains report-only and unrecorded"),
    blocker("audit_index_persistence_blocked"; "persist_denial_audit_index"; "denial audit index is not written to WorkGraph or projection storage"),
    blocker("audit_index_acceptance_blocked"; "accept_denial_audit_index"; "audit index does not create operator acceptance"),
    blocker("approval_record_blocked"; "record_operator_approval"; "no approval record may be created from an audit index"),
    blocker("feature_flag_config_write_blocked"; "write_feature_flag_config"; "feature-flag config writes remain disabled"),
    blocker("feature_flag_enablement_blocked"; "enable_feature_flag"; "feature flags remain current off"),
    blocker("canary_traffic_blocked"; "route_canary_traffic"; "canary traffic remains 0ppm"),
    blocker("scheduler_enforcement_blocked"; "enforce_scheduler_admission"; "scheduler admission remains dry-run only"),
    blocker("replay_execution_blocked"; "execute_replay"; "replay remains unexecuted"),
    blocker("rollback_execution_blocked"; "execute_rollback"; "rollback remains unexecuted"),
    blocker("live_cutover_blocked"; "perform_live_cutover"; "live cutover remains disabled")
  ] as $audit_index_blockers
  | [
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
  ] as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_gate",
      schema_version: "work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_v1",
      preview_mode: "enablement_precondition_denial_audit_index_no_record_no_persistence_no_acceptance",
      source_denial_readback_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_gate",
      source_denial_readback_entry_count: 5,
      source_denial_readback_blocker_count: 10,
      source_required_prior_gate_count: 10,
      audit_index_entry_count: ($audit_index_entries | length),
      audit_index_blocker_count: ($audit_index_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      audit_index_scope: $audit_index_scope,
      audit_index_entries: $audit_index_entries,
      audit_index_blockers: $audit_index_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_gate",
      audit_index_visible: true,
      audit_index_recorded: false,
      audit_index_persisted: false,
      audit_index_authoritative: false,
      audit_index_acceptance_allowed: false,
      audit_index_authorizes_config_write: false,
      audit_index_authorizes_feature_flag_enablement: false,
      audit_index_authorizes_canary_traffic: false,
      audit_index_authorizes_scheduler_enforcement: false,
      audit_index_authorizes_replay_execution: false,
      audit_index_authorizes_rollback_execution: false,
      audit_index_authorizes_live_cutover: false,
      ready_for_non_persistence_readback: true,
      ready_for_feature_flag_config_write: false,
      ready_for_feature_flag_enablement: false,
      ready_for_canary_traffic: false,
      ready_for_live_cutover: false,
      source_probes: {
        denial_audit_index_module_present: $denial_audit_index_module_present,
        denial_readback_gate_present: $denial_readback_gate_present,
        denial_readback_points_here: $denial_readback_points_here,
        denial_readback_ready_present: $denial_readback_ready_present,
        denial_readback_non_authoritative_present: $denial_readback_non_authoritative_present
      },
      side_effects: {
        filesystem_written: false,
        operator_packet_sent: false,
        operator_packet_recorded: false,
        operator_packet_persisted: false,
        operator_packet_accepted: false,
        approval_recorded: false,
        audit_index_recorded: false,
        audit_index_persisted: false,
        config_written: false,
        feature_flag_mutated: false,
        non_blocking_canary_enabled: false,
        canary_traffic_routed: false,
        live_cutover_enabled: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        projection_index_persisted: false,
        config_digest_persisted: false,
        scheduler_admission_enforced: false,
        guardrail_enforcement_enabled: false,
        replay_executed: false,
        rollback_executed: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
