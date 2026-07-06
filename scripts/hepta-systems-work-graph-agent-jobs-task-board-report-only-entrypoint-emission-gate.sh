#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-report-only-entrypoint-emission-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-report-only-entrypoint-emission-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_report_only_entrypoint_emission_v1"
  and .preview_mode == "report_only_entrypoint_emission_no_live_blocking"
  and .entrypoint_count == 2
  and .emission_count == 2
  and (.canonical_wire_fields == [
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
  ])
  and (.emissions | map(.source_surface_id) == [
    "agent_jobs_batch_workers",
    "hepta_runtime_task_board"
  ])
  and (.emissions | all(
    .emission_field == "workGraphReportOnly"
    and (.trace_guardrail_join_fields == ["traceId", "spanId", "evidence", "blockingGuardrailPreview"])
    and (.evidence_refs | length) >= 3
    and .actual_runtime_hook_attached == true
    and .report_only_attached == true
    and .live_blocking_enabled == false
    and .persistence_enabled == false
  ))
  and .required_prior_gates == [
    "hepta_work_graph_trace_guardrail_span_report_only_gate",
    "hepta_work_graph_task_result_envelope_report_only_validator_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  ]
  and .required_prior_gate_count == 3
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate"
  and .agent_jobs_report_only_emission_attached == true
  and .task_board_report_only_emission_attached == true
  and .ready_for_canary_readback_replay_gate == true
  and .ready_for_live_execution == false
  and .source_probes.agent_jobs_task_board_report_only_entrypoint_emission.rust_module_present == true
  and .source_probes.agent_jobs_task_board_report_only_entrypoint_emission.report_script_present == true
  and .source_probes.agent_jobs_task_board_report_only_entrypoint_emission.gate_script_present == true
  and .source_probes.trace_guardrail_span_report_only.gate_script_present == true
  and .source_probes.agent_jobs_batch_workers.core_hook_present == true
  and .source_probes.agent_jobs_batch_workers.output_field_present == true
  and .source_probes.hepta_runtime_task_board.runtime_hook_present == true
  and .source_probes.hepta_runtime_task_board.output_field_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_report_only_entrypoint_emission --lib

echo "Hepta WorkGraph agent_jobs + task_board report-only entrypoint emission gate passed"
