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

rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_report_only_entrypoint_emission.rs
)"
report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-report-only-entrypoint-emission-report.sh
)"
gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-report-only-entrypoint-emission-gate.sh
)"
trace_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-trace-guardrail-span-report-only-gate.sh
)"
agent_jobs_core_hook_present="$(
  bool_for source_has "work_graph_report_only" codex-rs/core/src/tools/handlers/agent_jobs/report_agent_job_result.rs
)"
agent_jobs_output_field_present="$(
  bool_for source_has "work_graph_report_only" codex-rs/core/src/tools/handlers/agent_jobs.rs
)"
task_board_runtime_hook_present="$(
  bool_for source_has "task_board_work_graph_report_only_emission" codex-rs/hepta-runtime/src/task_board.rs
)"
task_board_output_field_present="$(
  bool_for source_has "workGraphReportOnly" codex-rs/hepta-runtime/src/task_board.rs
)"

jq -n \
  --argjson rust_module_present "$rust_module_present" \
  --argjson report_script_present "$report_script_present" \
  --argjson gate_script_present "$gate_script_present" \
  --argjson trace_gate_script_present "$trace_gate_script_present" \
  --argjson agent_jobs_core_hook_present "$agent_jobs_core_hook_present" \
  --argjson agent_jobs_output_field_present "$agent_jobs_output_field_present" \
  --argjson task_board_runtime_hook_present "$task_board_runtime_hook_present" \
  --argjson task_board_output_field_present "$task_board_output_field_present" \
  '
  def emission($source; $entrypoint; $mapping; $evidence): {
    source_surface_id: $source,
    entrypoint_id: $entrypoint,
    emission_field: "workGraphReportOnly",
    task_result_status_mapping: $mapping,
    trace_guardrail_join_fields: ["traceId", "spanId", "evidence", "blockingGuardrailPreview"],
    evidence_refs: $evidence,
    actual_runtime_hook_attached: true,
    report_only_attached: true,
    live_blocking_enabled: false,
    persistence_enabled: false
  };
  [
    "taskId",
    "status",
    "summary",
    "artifacts",
    "evidence",
    "risks",
    "nextActions",
    "verifier",
    "reducer",
    "usage",
    "traceId",
    "spanId",
    "blockingGuardrailPreview",
    "liveBlockingEnabled"
  ] as $fields
  | [
    emission("agent_jobs_batch_workers"; "report_agent_job_result"; "accepted=true -> succeeded; accepted=false -> blocked"; ["agent_job_id", "agent_job_item_id", "reporting_thread_id"]),
    emission("hepta_runtime_task_board"; "task_board_terminal_event"; "completed/failed/cancelled -> terminal TaskResultEnvelope preview"; ["task_board_event_id", "delivery_id", "readback_evidence_id"])
  ] as $emissions
  | [
    "hepta_work_graph_trace_guardrail_span_report_only_gate",
    "hepta_work_graph_task_result_envelope_report_only_validator_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  ] as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate",
      schema_version: "work_graph_agent_jobs_task_board_report_only_entrypoint_emission_v1",
      preview_mode: "report_only_entrypoint_emission_no_live_blocking",
      entrypoint_count: ($emissions | length),
      emission_count: ($emissions | length),
      required_prior_gate_count: ($required_prior_gates | length),
      canonical_wire_fields: $fields,
      emissions: $emissions,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate",
      agent_jobs_report_only_emission_attached: true,
      task_board_report_only_emission_attached: true,
      ready_for_canary_readback_replay_gate: true,
      ready_for_live_execution: false,
      source_probes: {
        agent_jobs_task_board_report_only_entrypoint_emission: {
          rust_module_present: $rust_module_present,
          report_script_present: $report_script_present,
          gate_script_present: $gate_script_present
        },
        trace_guardrail_span_report_only: {
          gate_script_present: $trace_gate_script_present
        },
        agent_jobs_batch_workers: {
          core_hook_present: $agent_jobs_core_hook_present,
          output_field_present: $agent_jobs_output_field_present
        },
        hepta_runtime_task_board: {
          runtime_hook_present: $task_board_runtime_hook_present,
          output_field_present: $task_board_output_field_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        task_result_persisted: false,
        scheduler_admission_enforced: false,
        guardrail_enforcement_enabled: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
