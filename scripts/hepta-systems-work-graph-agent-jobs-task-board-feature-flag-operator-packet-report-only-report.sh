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

operator_packet_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only.rs
)"
config_wiring_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-config-wiring-report-only-gate.sh
)"
config_wiring_gate_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only.rs
)"
config_wiring_ready_for_operator_packet="$(
  bool_for source_has "ready_for_operator_packet_report_only: true" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only.rs
)"
agent_jobs_flag_metadata_present="$(
  bool_for source_has "work_graph_agent_jobs_non_blocking_canary" \
    codex-rs/core/src/tools/handlers/agent_jobs/report_agent_job_result.rs
)"
task_board_flag_metadata_present="$(
  bool_for source_has "work_graph_task_board_non_blocking_canary" codex-rs/hepta-runtime/src/task_board.rs
)"

jq -n \
  --argjson operator_packet_module_present "$operator_packet_module_present" \
  --argjson config_wiring_gate_present "$config_wiring_gate_present" \
  --argjson config_wiring_gate_points_here "$config_wiring_gate_points_here" \
  --argjson config_wiring_ready_for_operator_packet "$config_wiring_ready_for_operator_packet" \
  --argjson agent_jobs_flag_metadata_present "$agent_jobs_flag_metadata_present" \
  --argjson task_board_flag_metadata_present "$task_board_flag_metadata_present" \
  '
  def section($id; $title; $source): {
    id: $id,
    title: $title,
    source_gate: $source,
    required: true
  };
  def review($id; $flag): {
    id: $id,
    flag_id: $flag,
    review_surface_id: "work_graph_agent_jobs_task_board_feature_flag_operator_packet",
    decision_state: "pending_operator_review",
    required_before_enablement: true,
    config_write_authorized: false,
    canary_traffic_authorized: false,
    live_cutover_authorized: false
  };
  def evidence($id; $type; $source): {
    id: $id,
    evidence_type: $type,
    source_gate: $source,
    required: true,
    redacted: true,
    persisted: false
  };
  def blocked($id; $action; $reason): {
    id: $id,
    action: $action,
    blocked: true,
    reason: $reason
  };
  [
    section(
      "scope_and_canary_flags";
      "Scope and canary flags";
      "hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_gate"
    ),
    section(
      "config_contract_and_digest";
      "Config contract and digest preview";
      "hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_gate"
    ),
    section(
      "readback_replay_evidence";
      "Readback and replay evidence";
      "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate"
    ),
    section(
      "trace_guardrail_evidence";
      "Trace and guardrail evidence";
      "hepta_work_graph_trace_guardrail_span_report_only_gate"
    ),
    section(
      "scheduler_admission_evidence";
      "Scheduler admission dry-run evidence";
      "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
    )
  ] as $operator_packet_sections
  | [
    review("agent_jobs_canary_flag_operator_review"; "work_graph_agent_jobs_non_blocking_canary"),
    review("task_board_canary_flag_operator_review"; "work_graph_task_board_non_blocking_canary")
  ] as $review_items
  | [
    evidence(
      "feature_flag_non_blocking_canary_report";
      "canary_flag_shape";
      "hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_gate"
    ),
    evidence(
      "feature_flag_config_wiring_report";
      "config_contract_digest";
      "hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_gate"
    ),
    evidence(
      "canary_readback_replay_report";
      "readback_replay_diff";
      "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate"
    ),
    evidence(
      "trace_guardrail_span_report";
      "blocking_guardrail_preview";
      "hepta_work_graph_trace_guardrail_span_report_only_gate"
    ),
    evidence(
      "scheduler_admission_dry_run_report";
      "allow_deny_explanation";
      "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
    )
  ] as $evidence_refs
  | [
    blocked(
      "operator_packet_delivery_blocked";
      "send_operator_packet";
      "operator packet is only assembled as report-only evidence"
    ),
    blocked(
      "operator_packet_recording_blocked";
      "record_operator_approval";
      "no operator approval or packet acceptance is recorded by this gate"
    ),
    blocked(
      "feature_flag_config_write_blocked";
      "write_feature_flag_config";
      "config writing remains disabled until operator packet readback and approval are explicit"
    ),
    blocked(
      "feature_flag_enablement_blocked";
      "enable_feature_flag";
      "canary flags remain default/current off with zero traffic"
    ),
    blocked(
      "canary_traffic_blocked";
      "route_non_blocking_canary_traffic";
      "0ppm report-only observation remains the only allowed stage"
    ),
    blocked(
      "live_cutover_blocked";
      "perform_live_cutover";
      "live WorkGraph cutover is outside this report-only gate"
    )
  ] as $blocked_actions
  | [
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
      gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_gate",
      schema_version: "work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_v1",
      preview_mode: "feature_flag_operator_packet_report_only_no_approval_no_send_no_persistence",
      operator_packet_section_count: ($operator_packet_sections | length),
      review_item_count: ($review_items | length),
      evidence_ref_count: ($evidence_refs | length),
      blocked_action_count: ($blocked_actions | length),
      required_prior_gate_count: ($required_prior_gates | length),
      operator_packet_sections: $operator_packet_sections,
      review_items: $review_items,
      evidence_refs: $evidence_refs,
      blocked_actions: $blocked_actions,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_gate",
      operator_packet_visible: true,
      operator_packet_sent: false,
      operator_packet_recorded: false,
      operator_packet_persisted: false,
      operator_packet_authorizes_config_write: false,
      operator_packet_authorizes_canary_traffic: false,
      operator_packet_authorizes_live_cutover: false,
      ready_for_operator_packet_non_send_readback: true,
      ready_for_feature_flag_config_write: false,
      ready_for_feature_flag_enablement: false,
      ready_for_live_cutover: false,
      source_probes: {
        operator_packet_module_present: $operator_packet_module_present,
        config_wiring_gate_present: $config_wiring_gate_present,
        config_wiring_gate_points_here: $config_wiring_gate_points_here,
        config_wiring_ready_for_operator_packet: $config_wiring_ready_for_operator_packet,
        agent_jobs_flag_metadata_present: $agent_jobs_flag_metadata_present,
        task_board_flag_metadata_present: $task_board_flag_metadata_present
      },
      side_effects: {
        filesystem_written: false,
        operator_packet_sent: false,
        operator_packet_recorded: false,
        operator_packet_persisted: false,
        operator_packet_accepted: false,
        approval_recorded: false,
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
