#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-trace-guardrail-span-report-only-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-trace-guardrail-span-report-only-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_trace_guardrail_span_report_only_gate"
  and .schema_version == "work_graph_trace_guardrail_span_report_only_v1"
  and .preview_mode == "report_only_trace_guardrail_span_no_live_blocking"
  and .required_wire_field_count == 10
  and (.required_wire_fields == [
    "traceId",
    "spanId",
    "parentSpanId",
    "kind",
    "source",
    "decision",
    "blocking",
    "evidenceRef",
    "redaction",
    "hash"
  ])
  and .span_count == 9
  and (.spans | map(.kind) == [
    "plan",
    "spawn",
    "handoff",
    "mailbox",
    "tool",
    "result",
    "artifact",
    "approval",
    "guardrail"
  ])
  and (.spans | all(
    .trace_id == "trace-work-graph-report-only-001"
    and (.span_id | length) > 0
    and (.source_surface_id | length) > 0
    and (.source_entrypoint | length) > 0
    and (.decision | length) > 0
    and (.guardrail_span_id | length) > 0
    and (.evidence_ref | length) > 0
    and (.redaction_policy | length) > 0
    and (.payload_hash | startswith("sha256:"))
  ))
  and .blocking_guardrail_count == 6
  and ([.spans[] | select(.blocking_guardrail_required == true) | .span_id] == [
    "span-spawn-001",
    "span-handoff-001",
    "span-tool-001",
    "span-artifact-001",
    "span-approval-001",
    "span-guardrail-001"
  ])
  and (.guardrail_bindings | length) == 6
  and (.guardrail_bindings | all(
    .blocking_preview == true
    and .required_for_live_promotion == true
    and .decision == "block_live_execution_report_only"
    and (.evidence_ref | length) > 0
  ))
  and (([.spans[] | select(.blocking_guardrail_required == true) | .span_id] - [.guardrail_bindings[].span_id]) == [])
  and .source_binding_count == 5
  and (.source_bindings | map(.source_surface_id) == [
    "multi_agent_v2_thread_spawn",
    "agent_jobs_batch_workers",
    "hepta_runtime_task_board",
    "hepta_runtime_agent_harness",
    "hepta_runtime_worker_tasks"
  ])
  and (.source_bindings | all(
    (.required_span_kinds | length) >= 4
    and (.required_guardrail_ids | length) >= 2
    and .trace_join_fields == ["traceId", "spanId", "parentSpanId", "evidenceRef"]
    and .report_only_attached == true
    and .live_blocking_enabled == false
  ))
  and .required_prior_gates == [
    "hepta_work_graph_agent_role_agent_card_manifest_report_only_gate",
    "hepta_work_graph_append_only_event_store_shadow_path_gate",
    "hepta_work_graph_task_result_envelope_report_only_validator_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  ]
  and .required_prior_gate_count == 4
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate"
  and .trace_spine_complete == true
  and .blocking_guardrail_preview_complete == true
  and .report_only_guardrail_attached == true
  and .live_guardrail_enforcement_enabled == false
  and .ready_for_agent_jobs_task_board_report_only_emission == true
  and .ready_for_live_execution == false
  and .source_probes.trace_guardrail_span_report_only.rust_module_present == true
  and .source_probes.trace_guardrail_span_report_only.report_script_present == true
  and .source_probes.trace_guardrail_span_report_only.gate_script_present == true
  and .source_probes.agent_role_agent_card_manifest_report_only.gate_script_present == true
  and .source_probes.append_only_event_store_shadow_path.gate_script_present == true
  and .source_probes.task_result_envelope_report_only_validator.gate_script_present == true
  and .source_probes.scheduler_admission_dry_run_enforcement.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_trace_guardrail_span_report_only --lib

echo "Hepta WorkGraph trace guardrail span report-only gate passed"
