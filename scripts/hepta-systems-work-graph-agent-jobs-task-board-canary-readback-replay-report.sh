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

canary_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_canary_readback_replay.rs
)"
entrypoint_emission_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-report-only-entrypoint-emission-gate.sh
)"
shadow_path_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-event-store-shadow-path-gate.sh
)"
agent_jobs_report_only_hook_present="$(
  bool_for source_has "work_graph_report_only" codex-rs/core/src/tools/handlers/agent_jobs/report_agent_job_result.rs
)"
task_board_report_only_hook_present="$(
  bool_for source_has "task_board_work_graph_report_only_emission" codex-rs/hepta-runtime/src/task_board.rs
)"
agent_jobs_test_present="$(
  bool_for source_has "work_graph_report_only_emission_keeps_agent_job_result_non_blocking" codex-rs/core/src/tools/handlers/agent_jobs/report_agent_job_result.rs
)"
task_board_test_present="$(
  bool_for source_has "workGraphReportOnly" codex-rs/hepta-runtime/src/task_board.rs
)"

jq -n \
  --argjson canary_module_present "$canary_module_present" \
  --argjson entrypoint_emission_gate_present "$entrypoint_emission_gate_present" \
  --argjson shadow_path_gate_present "$shadow_path_gate_present" \
  --argjson agent_jobs_report_only_hook_present "$agent_jobs_report_only_hook_present" \
  --argjson task_board_report_only_hook_present "$task_board_report_only_hook_present" \
  --argjson agent_jobs_test_present "$agent_jobs_test_present" \
  --argjson task_board_test_present "$task_board_test_present" \
  '
  def entrypoint($source; $entrypoint; $trace_join; $rollback): {
    source_surface_id: $source,
    entrypoint_id: $entrypoint,
    report_only_field: "workGraphReportOnly",
    admission_decision: "allow_report_only_no_live_blocking",
    trace_join: $trace_join,
    task_result_preview: "TaskResultEnvelope report-only emission",
    rollback_anchor: $rollback,
    live_blocking_enabled: false,
    live_persistence_enabled: false
  };
  def index($id; $source; $keys): {
    index_id: $id,
    source_surface_id: $source,
    key_fields: $keys,
    deterministic_id_rule: "sha256(redacted source surface + taskId + traceId + spanId)",
    redaction_rule: "payload summaries only; no raw prompt, transcript, secret, or artifact body",
    persisted: false
  };
  def evidence($id; $source; $checks): {
    evidence_id: $id,
    source_surface_id: $source,
    checks: $checks,
    evidence_status: "preview_ready_not_persisted",
    evidence_persisted: false
  };
  def diff($id; $source; $scope): {
    diff_id: $id,
    source_surface_id: $source,
    replay_scope: $scope,
    expected_diff: "deterministic report-only envelope matches readback projection",
    replay_executed: false
  };
  [
    entrypoint("agent_jobs_batch_workers"; "report_agent_job_result"; "traceId + spanId + agent_job_id + agent_job_item_id"; "agent_job_state_db_item_status"),
    entrypoint("hepta_runtime_task_board"; "task_board_terminal_event"; "traceId + spanId + task_board_event_id + delivery_id"; "task_board_json_state_terminal_event")
  ] as $entrypoints
  | [
    index("agent_jobs_task_result_by_task_id"; "agent_jobs_batch_workers"; ["taskId", "traceId", "agent_job_id", "agent_job_item_id"]),
    index("task_board_terminal_event_by_task_id"; "hepta_runtime_task_board"; ["taskId", "traceId", "task_board_event_id", "delivery_id"])
  ] as $indexes
  | [
    evidence("agent_jobs_task_result_report_only_readback"; "agent_jobs_batch_workers"; [
      "workGraphReportOnly field present",
      "TaskResultEnvelope canonical fields present",
      "admission decision remains non-blocking",
      "no WorkGraph event persisted"
    ]),
    evidence("task_board_terminal_report_only_readback"; "hepta_runtime_task_board"; [
      "workGraphReportOnly field present",
      "terminal event id joins evidence",
      "delivery readback evidence joins trace",
      "no WorkGraph event persisted"
    ])
  ] as $evidence
  | [
    diff("agent_jobs_report_only_replay_diff"; "agent_jobs_batch_workers"; "report_agent_job_result dry-run envelope"),
    diff("task_board_terminal_report_only_replay_diff"; "hepta_runtime_task_board"; "task_board terminal event dry-run envelope")
  ] as $diffs
  | [
    "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate",
    "hepta_work_graph_append_only_event_store_shadow_path_gate",
    "hepta_work_graph_task_result_envelope_report_only_validator_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  ] as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate",
      schema_version: "work_graph_agent_jobs_task_board_canary_readback_replay_v1",
      preview_mode: "canary_readback_replay_report_only_no_live_cutover",
      canary_entrypoint_count: ($entrypoints | length),
      readback_evidence_count: ($evidence | length),
      replay_diff_count: ($diffs | length),
      required_prior_gate_count: ($required_prior_gates | length),
      canary_entrypoints: $entrypoints,
      projection_indexes: $indexes,
      readback_evidence: $evidence,
      replay_diffs: $diffs,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_gate",
      feature_flag_required: true,
      feature_flag_enabled: false,
      ready_for_non_blocking_canary: true,
      ready_for_live_cutover: false,
      source_probes: {
        canary_readback_replay_module_present: $canary_module_present,
        entrypoint_emission_gate_present: $entrypoint_emission_gate_present,
        shadow_path_gate_present: $shadow_path_gate_present,
        agent_jobs_report_only_hook_present: $agent_jobs_report_only_hook_present,
        task_board_report_only_hook_present: $task_board_report_only_hook_present,
        agent_jobs_test_present: $agent_jobs_test_present,
        task_board_test_present: $task_board_test_present
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        projection_index_persisted: false,
        readback_evidence_persisted: false,
        replay_executed: false,
        rollback_executed: false,
        feature_flag_enabled: false,
        scheduler_admission_enforced: false,
        guardrail_enforcement_enabled: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
