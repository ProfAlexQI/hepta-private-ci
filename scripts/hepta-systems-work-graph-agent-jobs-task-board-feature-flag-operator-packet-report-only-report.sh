#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  HEPTA_OPERATOR_PACKET_CAPTURE_CACHE_DIR="$(
    mktemp -d "${TMPDIR:-/tmp}/hepta-operator-packet-report-cache.XXXXXX"
  )"
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$HEPTA_OPERATOR_PACKET_CAPTURE_CACHE_DIR"
  trap 'rm -rf "$HEPTA_OPERATOR_PACKET_CAPTURE_CACHE_DIR"' EXIT
fi

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
agent_jobs_flag_metadata_present="$(
  bool_for source_has "work_graph_agent_jobs_non_blocking_canary" \
    codex-rs/core/src/tools/handlers/agent_jobs/report_agent_job_result.rs
)"
task_board_flag_metadata_present="$(
  bool_for source_has "work_graph_task_board_non_blocking_canary" codex-rs/hepta-runtime/src/task_board.rs
)"

config_wiring="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-feature-flag-config-wiring-report-only-report" \
    "$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-config-wiring-report-only-report.sh"
)"
feature_flag="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-feature-flag-non-blocking-canary-report" \
    "$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-non-blocking-canary-report.sh"
)"
canary_readback="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-canary-readback-replay-report" \
    "$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-canary-readback-replay-report.sh"
)"
entrypoint_emission="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-report-only-entrypoint-emission-report" \
    "$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-report-only-entrypoint-emission-report.sh"
)"
trace_guardrail="$(
  capture_json_report \
    "hepta-work-graph-trace-guardrail-span-report-only-report" \
    "$ROOT/scripts/hepta-systems-work-graph-trace-guardrail-span-report-only-report.sh"
)"
scheduler="$(
  capture_json_report \
    "hepta-work-graph-scheduler-admission-dry-run-enforcement-report" \
    "$ROOT/scripts/hepta-systems-work-graph-scheduler-admission-dry-run-enforcement-report.sh"
)"

jq -n \
  --argjson operator_packet_module_present "$operator_packet_module_present" \
  --argjson config_wiring_gate_present "$config_wiring_gate_present" \
  --argjson config_wiring_gate_points_here "$config_wiring_gate_points_here" \
  --argjson agent_jobs_flag_metadata_present "$agent_jobs_flag_metadata_present" \
  --argjson task_board_flag_metadata_present "$task_board_flag_metadata_present" \
  --argjson config_wiring "$config_wiring" \
  --argjson feature_flag "$feature_flag" \
  --argjson canary_readback "$canary_readback" \
  --argjson entrypoint_emission "$entrypoint_emission" \
  --argjson trace_guardrail "$trace_guardrail" \
  --argjson scheduler "$scheduler" \
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
  | ($config_wiring.ready_for_feature_flag_config_write == false
      and $config_wiring.ready_for_feature_flag_enablement == false
      and $config_wiring.ready_for_live_cutover == false
      and ($config_wiring.side_effects | to_entries | all(.value == false))) as $source_config_wiring_no_write_confirmed
  | ($config_wiring.gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_gate"
      and $config_wiring.config_wiring_prior_readbacks_complete == true
      and $config_wiring.config_wiring_report_only_preconditions_complete == true
      and $config_wiring.ready_for_operator_packet_report_only == true
      and $source_config_wiring_no_write_confirmed) as $source_config_wiring_ready
  | ($feature_flag.ready_for_feature_flag_enablement == false
      and $feature_flag.ready_for_live_cutover == false
      and ($feature_flag.side_effects | to_entries | all(.value == false))) as $source_feature_flag_non_blocking_canary_no_enablement_confirmed
  | ($feature_flag.gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_gate"
      and $feature_flag.feature_flag_prior_readbacks_complete == true
      and $feature_flag.feature_flag_enablement_preconditions_report_only_complete == true
      and $feature_flag.ready_for_feature_flag_config_wiring == true
      and $source_feature_flag_non_blocking_canary_no_enablement_confirmed) as $source_feature_flag_non_blocking_canary_ready
  | ($canary_readback.feature_flag_enabled == false
      and $canary_readback.ready_for_live_cutover == false
      and ($canary_readback.side_effects | to_entries | all(.value == false))) as $source_canary_readback_replay_no_live_confirmed
  | ($canary_readback.gate == "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate"
      and $canary_readback.canary_readback_replay_prior_readbacks_complete == true
      and $canary_readback.canary_projection_readback_replay_preview_complete == true
      and $canary_readback.ready_for_non_blocking_canary == true
      and $source_canary_readback_replay_no_live_confirmed) as $source_canary_readback_replay_ready
  | ($entrypoint_emission.ready_for_live_execution == false
      and ($entrypoint_emission.side_effects | to_entries | all(.value == false))) as $source_entrypoint_emission_no_live_confirmed
  | ($entrypoint_emission.gate == "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate"
      and $entrypoint_emission.entrypoint_emission_readiness_complete == true
      and $entrypoint_emission.ready_for_canary_readback_replay_gate == true
      and $source_entrypoint_emission_no_live_confirmed) as $source_entrypoint_emission_readiness_complete
  | ($trace_guardrail.live_guardrail_enforcement_enabled == false
      and $trace_guardrail.ready_for_live_execution == false
      and ($trace_guardrail.side_effects | to_entries | all(.value == false))) as $source_trace_guardrail_no_live_blocking_confirmed
  | ($trace_guardrail.gate == "hepta_work_graph_trace_guardrail_span_report_only_gate"
      and $trace_guardrail.trace_guardrail_prior_readbacks_complete == true
      and $trace_guardrail.ready_for_agent_jobs_task_board_report_only_emission == true
      and $source_trace_guardrail_no_live_blocking_confirmed) as $source_trace_guardrail_readiness_complete
  | ($scheduler.live_blocking_enforcement_enabled == false
      and $scheduler.ready_for_live_execution == false
      and ($scheduler.side_effects | to_entries | all(.value == false))) as $source_scheduler_admission_no_live_blocking_confirmed
  | ($scheduler.gate == "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
      and $scheduler.dry_run_enforcement_enabled == true
      and $scheduler.ready_for_append_only_event_store_shadow_path == true
      and $source_scheduler_admission_no_live_blocking_confirmed) as $source_scheduler_admission_dry_run_ready
  | ($source_config_wiring_ready
      and $source_feature_flag_non_blocking_canary_ready
      and $source_canary_readback_replay_ready
      and $source_entrypoint_emission_readiness_complete
      and $source_trace_guardrail_readiness_complete
      and $source_scheduler_admission_dry_run_ready) as $operator_packet_prior_readbacks_complete
  | (($operator_packet_sections | length) > 0
      and ($operator_packet_sections | all(.required == true))) as $operator_packet_sections_report_only_complete
  | (($review_items | length) > 0
      and ($review_items | all(
        .decision_state == "pending_operator_review"
        and .required_before_enablement == true
        and .config_write_authorized == false
        and .canary_traffic_authorized == false
        and .live_cutover_authorized == false
      ))) as $operator_packet_review_items_non_authorizing
  | (($evidence_refs | length) > 0
      and ($evidence_refs | all(
        .required == true
        and .redacted == true
        and .persisted == false
      ))) as $operator_packet_evidence_refs_report_only_complete
  | (($blocked_actions | length) > 0
      and ($blocked_actions | all(.blocked == true))) as $operator_packet_blocked_actions_complete
  | ($operator_packet_prior_readbacks_complete
      and $operator_packet_sections_report_only_complete
      and $operator_packet_review_items_non_authorizing
      and $operator_packet_evidence_refs_report_only_complete
      and $operator_packet_blocked_actions_complete) as $operator_packet_report_only_preconditions_complete
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
      source_config_wiring_required_prior_gate_count: $config_wiring.required_prior_gate_count,
      source_config_wiring_config_contract_count: $config_wiring.config_contract_count,
      source_config_wiring_config_digest_preview_count: $config_wiring.config_digest_preview_count,
      source_config_wiring_source_binding_count: $config_wiring.source_binding_count,
      source_feature_flag_non_blocking_canary_required_prior_gate_count: $feature_flag.required_prior_gate_count,
      source_feature_flag_count: $feature_flag.feature_flag_count,
      source_feature_flag_safety_check_count: $feature_flag.safety_check_count,
      source_canary_readback_replay_required_prior_gate_count: $canary_readback.required_prior_gate_count,
      source_canary_readback_replay_entrypoint_count: $canary_readback.canary_entrypoint_count,
      source_canary_readback_replay_readback_evidence_count: $canary_readback.readback_evidence_count,
      source_canary_readback_replay_replay_diff_count: $canary_readback.replay_diff_count,
      source_entrypoint_emission_entrypoint_count: $entrypoint_emission.entrypoint_count,
      source_entrypoint_emission_emission_count: $entrypoint_emission.emission_count,
      source_trace_guardrail_span_count: $trace_guardrail.span_count,
      source_trace_guardrail_blocking_guardrail_count: $trace_guardrail.blocking_guardrail_count,
      source_scheduler_admission_entrypoint_count: $scheduler.entrypoint_count,
      source_scheduler_admission_required_prior_gate_count: ($scheduler.required_prior_gates | length),
      operator_packet_sections: $operator_packet_sections,
      review_items: $review_items,
      evidence_refs: $evidence_refs,
      blocked_actions: $blocked_actions,
      required_prior_gates: $required_prior_gates,
      source_config_wiring_gate: $config_wiring.gate,
      source_feature_flag_non_blocking_canary_gate: $feature_flag.gate,
      source_canary_readback_replay_gate: $canary_readback.gate,
      source_entrypoint_emission_gate: $entrypoint_emission.gate,
      source_trace_guardrail_gate: $trace_guardrail.gate,
      source_scheduler_admission_dry_run_gate: $scheduler.gate,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_gate",
      source_config_wiring_ready: $source_config_wiring_ready,
      source_config_wiring_no_write_confirmed: $source_config_wiring_no_write_confirmed,
      source_feature_flag_non_blocking_canary_ready: $source_feature_flag_non_blocking_canary_ready,
      source_feature_flag_non_blocking_canary_no_enablement_confirmed: $source_feature_flag_non_blocking_canary_no_enablement_confirmed,
      source_canary_readback_replay_ready: $source_canary_readback_replay_ready,
      source_canary_readback_replay_no_live_confirmed: $source_canary_readback_replay_no_live_confirmed,
      source_entrypoint_emission_readiness_complete: $source_entrypoint_emission_readiness_complete,
      source_entrypoint_emission_no_live_confirmed: $source_entrypoint_emission_no_live_confirmed,
      source_trace_guardrail_readiness_complete: $source_trace_guardrail_readiness_complete,
      source_trace_guardrail_no_live_blocking_confirmed: $source_trace_guardrail_no_live_blocking_confirmed,
      source_scheduler_admission_dry_run_ready: $source_scheduler_admission_dry_run_ready,
      source_scheduler_admission_no_live_blocking_confirmed: $source_scheduler_admission_no_live_blocking_confirmed,
      operator_packet_prior_readbacks_complete: $operator_packet_prior_readbacks_complete,
      operator_packet_sections_report_only_complete: $operator_packet_sections_report_only_complete,
      operator_packet_review_items_non_authorizing: $operator_packet_review_items_non_authorizing,
      operator_packet_evidence_refs_report_only_complete: $operator_packet_evidence_refs_report_only_complete,
      operator_packet_blocked_actions_complete: $operator_packet_blocked_actions_complete,
      operator_packet_report_only_preconditions_complete: $operator_packet_report_only_preconditions_complete,
      operator_packet_visible: true,
      operator_packet_sent: false,
      operator_packet_recorded: false,
      operator_packet_persisted: false,
      operator_packet_authorizes_config_write: false,
      operator_packet_authorizes_canary_traffic: false,
      operator_packet_authorizes_live_cutover: false,
      ready_for_operator_packet_non_send_readback: $operator_packet_report_only_preconditions_complete,
      ready_for_feature_flag_config_write: false,
      ready_for_feature_flag_enablement: false,
      ready_for_live_cutover: false,
      source_probes: {
        operator_packet_module_present: $operator_packet_module_present,
        config_wiring_gate_present: $config_wiring_gate_present,
        config_wiring_gate_points_here: $config_wiring_gate_points_here,
        config_wiring_report_gate: $config_wiring.gate,
        config_wiring_ready_for_operator_packet: $config_wiring.ready_for_operator_packet_report_only,
        config_wiring_ready_for_config_write: $config_wiring.ready_for_feature_flag_config_write,
        config_wiring_ready_for_enablement: $config_wiring.ready_for_feature_flag_enablement,
        config_wiring_side_effects_all_false: ($config_wiring.side_effects | to_entries | all(.value == false)),
        feature_flag_non_blocking_canary_report_gate: $feature_flag.gate,
        feature_flag_non_blocking_canary_ready_for_config_wiring: $feature_flag.ready_for_feature_flag_config_wiring,
        feature_flag_non_blocking_canary_ready_for_enablement: $feature_flag.ready_for_feature_flag_enablement,
        feature_flag_non_blocking_canary_side_effects_all_false: ($feature_flag.side_effects | to_entries | all(.value == false)),
        canary_readback_replay_report_gate: $canary_readback.gate,
        canary_readback_replay_ready_for_non_blocking_canary: $canary_readback.ready_for_non_blocking_canary,
        canary_readback_replay_ready_for_live_cutover: $canary_readback.ready_for_live_cutover,
        canary_readback_replay_feature_flag_enabled: $canary_readback.feature_flag_enabled,
        canary_readback_replay_side_effects_all_false: ($canary_readback.side_effects | to_entries | all(.value == false)),
        entrypoint_emission_report_gate: $entrypoint_emission.gate,
        entrypoint_emission_readiness_complete: $entrypoint_emission.entrypoint_emission_readiness_complete,
        entrypoint_emission_side_effects_all_false: ($entrypoint_emission.side_effects | to_entries | all(.value == false)),
        trace_guardrail_report_gate: $trace_guardrail.gate,
        trace_guardrail_prior_readbacks_complete: $trace_guardrail.trace_guardrail_prior_readbacks_complete,
        trace_guardrail_side_effects_all_false: ($trace_guardrail.side_effects | to_entries | all(.value == false)),
        scheduler_admission_dry_run_report_gate: $scheduler.gate,
        scheduler_admission_dry_run_ready: $scheduler.ready_for_append_only_event_store_shadow_path,
        scheduler_admission_dry_run_side_effects_all_false: ($scheduler.side_effects | to_entries | all(.value == false)),
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
