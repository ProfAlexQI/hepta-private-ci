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

non_send_readback_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback.rs
)"
operator_packet_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-operator-packet-report-only-gate.sh
)"
operator_packet_gate_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only.rs
)"
operator_packet_visible_unsent_present="$(
  bool_for source_has "operator_packet_visible: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only.rs
)"
operator_packet_unrecorded_present="$(
  bool_for source_has "operator_packet_recorded: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only.rs
)"
operator_packet_unpersisted_present="$(
  bool_for source_has "operator_packet_persisted: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only.rs
)"

jq -n \
  --argjson non_send_readback_module_present "$non_send_readback_module_present" \
  --argjson operator_packet_gate_present "$operator_packet_gate_present" \
  --argjson operator_packet_gate_points_here "$operator_packet_gate_points_here" \
  --argjson operator_packet_visible_unsent_present "$operator_packet_visible_unsent_present" \
  --argjson operator_packet_unrecorded_present "$operator_packet_unrecorded_present" \
  --argjson operator_packet_unpersisted_present "$operator_packet_unpersisted_present" \
  '
  def entry($id; $key; $state): {
    id: $id,
    stable_readback_key: $key,
    observed_state: $state,
    visible: true,
    sent: false,
    recorded: false,
    persisted: false,
    accepted: false,
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
    id: "agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_scope",
    source_surface_id: "work_graph_agent_jobs_task_board_feature_flag_operator_packet",
    readback_mode: "operator_packet_non_send_readback_only",
    stable_readback_key: "work_graph.agent_jobs_task_board.feature_flag.operator_packet.non_send_readback",
    packet_visible: true,
    packet_sent: false,
    packet_recorded: false,
    packet_persisted: false,
    packet_accepted: false,
    packet_authoritative: false,
    readback_persisted: false
  } as $readback_scope
  | [
    entry(
      "operator_packet_surface_readback";
      "operator_packet_visible_unsent_unrecorded_unpersisted";
      "packet_visible_without_send_record_persist_or_acceptance"
    ),
    entry(
      "operator_packet_review_state_readback";
      "operator_packet_pending_review_non_authoritative";
      "review_items_pending_without_authorization"
    ),
    entry(
      "operator_packet_evidence_ref_readback";
      "operator_packet_evidence_redacted_unpersisted";
      "evidence_refs_visible_redacted_unpersisted"
    ),
    entry(
      "operator_packet_blocked_action_readback";
      "operator_packet_blocked_actions_still_blocked";
      "config_write_enablement_traffic_and_cutover_blocked"
    )
  ] as $readback_entries
  | [
    blocker(
      "operator_packet_send_blocked";
      "send_operator_packet";
      "non-send readback cannot deliver or request approval"
    ),
    blocker(
      "operator_packet_record_blocked";
      "record_operator_packet";
      "operator packet readback is not an acceptance record"
    ),
    blocker(
      "operator_packet_persistence_blocked";
      "persist_operator_packet";
      "readback remains stdout/report-only and unpersisted"
    ),
    blocker(
      "operator_packet_acceptance_blocked";
      "accept_operator_packet";
      "no approval acceptance is allowed by non-send readback"
    ),
    blocker(
      "feature_flag_config_write_blocked";
      "write_feature_flag_config";
      "config write requires explicit future approval beyond readback"
    ),
    blocker(
      "feature_flag_enablement_blocked";
      "enable_feature_flag";
      "feature flags remain current off after readback"
    ),
    blocker(
      "canary_traffic_blocked";
      "route_canary_traffic";
      "canary traffic stays 0ppm in report-only readback"
    ),
    blocker(
      "live_cutover_blocked";
      "perform_live_cutover";
      "live cutover remains outside non-send readback"
    )
  ] as $readback_blockers
  | [
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
      gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_gate",
      schema_version: "work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_v1",
      preview_mode: "operator_packet_non_send_readback_only_no_send_no_record_no_persistence",
      source_operator_packet_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_gate",
      source_operator_packet_section_count: 5,
      source_review_item_count: 2,
      source_evidence_ref_count: 5,
      source_blocked_action_count: 6,
      readback_entry_count: ($readback_entries | length),
      readback_blocker_count: ($readback_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_scope: $readback_scope,
      readback_entries: $readback_entries,
      readback_blockers: $readback_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_gate",
      operator_packet_visible: true,
      operator_packet_sent: false,
      operator_packet_recorded: false,
      operator_packet_persisted: false,
      operator_packet_accepted: false,
      operator_packet_authoritative: false,
      operator_packet_authorizes_config_write: false,
      operator_packet_authorizes_canary_traffic: false,
      operator_packet_authorizes_live_cutover: false,
      approval_recorded: false,
      approval_acceptance_allowed: false,
      readback_persisted: false,
      ready_for_rollback_replay_pre_enable_blocker_matrix: true,
      ready_for_operator_packet_acceptance: false,
      ready_for_feature_flag_config_write: false,
      ready_for_feature_flag_enablement: false,
      ready_for_live_cutover: false,
      source_probes: {
        non_send_readback_module_present: $non_send_readback_module_present,
        operator_packet_gate_present: $operator_packet_gate_present,
        operator_packet_gate_points_here: $operator_packet_gate_points_here,
        operator_packet_visible_unsent_present: $operator_packet_visible_unsent_present,
        operator_packet_unrecorded_present: $operator_packet_unrecorded_present,
        operator_packet_unpersisted_present: $operator_packet_unpersisted_present
      },
      side_effects: {
        filesystem_written: false,
        operator_packet_sent: false,
        operator_packet_recorded: false,
        operator_packet_persisted: false,
        operator_packet_accepted: false,
        approval_recorded: false,
        readback_persisted: false,
        config_written: false,
        feature_flag_mutated: false,
        non_blocking_canary_enabled: false,
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
