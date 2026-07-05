#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

path_exists() {
  local path="$1"
  [[ -e "$path" ]]
}

bool_for() {
  if "$@"; then
    printf 'true\n'
  else
    printf 'false\n'
  fi
}

rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_trace_guardrail_span_report_only.rs
)"
report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-trace-guardrail-span-report-only-report.sh
)"
gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-trace-guardrail-span-report-only-gate.sh
)"
agent_card_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-role-agent-card-manifest-report-only-gate.sh
)"
shadow_path_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-event-store-shadow-path-gate.sh
)"
task_result_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-task-result-envelope-report-only-validator-gate.sh
)"
admission_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-dry-run-enforcement-gate.sh
)"

jq -n \
  --argjson rust_module_present "$rust_module_present" \
  --argjson report_script_present "$report_script_present" \
  --argjson gate_script_present "$gate_script_present" \
  --argjson agent_card_gate_script_present "$agent_card_gate_script_present" \
  --argjson shadow_path_gate_script_present "$shadow_path_gate_script_present" \
  --argjson task_result_gate_script_present "$task_result_gate_script_present" \
  --argjson admission_gate_script_present "$admission_gate_script_present" \
  '
  def span($id; $parent; $kind; $source; $entrypoint; $decision; $blocking; $guardrail; $evidence; $redaction; $hash): {
    trace_id: "trace-work-graph-report-only-001",
    span_id: $id,
    parent_span_id: $parent,
    kind: $kind,
    source_surface_id: $source,
    source_entrypoint: $entrypoint,
    decision: $decision,
    blocking_guardrail_required: $blocking,
    guardrail_span_id: $guardrail,
    evidence_ref: $evidence,
    redaction_policy: $redaction,
    payload_hash: $hash
  };
  def guardrail($guardrail; $span; $kind; $source; $evidence): {
    guardrail_id: $guardrail,
    span_id: $span,
    guardrail_kind: $kind,
    source_surface_id: $source,
    blocking_preview: true,
    required_for_live_promotion: true,
    decision: "block_live_execution_report_only",
    evidence_ref: $evidence
  };
  def binding($source; $entrypoint; $kinds; $guards): {
    source_surface_id: $source,
    entrypoint_id: $entrypoint,
    required_span_kinds: $kinds,
    required_guardrail_ids: $guards,
    trace_join_fields: ["traceId", "spanId", "parentSpanId", "evidenceRef"],
    report_only_attached: true,
    live_blocking_enabled: false
  };
  [
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
  ] as $fields
  | [
    span("span-plan-001"; null; "plan"; "planning_update_plan"; "update_plan"; "trace_only"; false; "guardrail-read-only-boundary"; "evidence:canonical-projection-readiness"; "redact_freeform_plan_text"; "sha256:trace-plan-001"),
    span("span-spawn-001"; "span-plan-001"; "spawn"; "multi_agent_v2_thread_spawn"; "spawn_agent"; "admission_trace_only"; true; "guardrail-spawn-admission"; "evidence:scheduler-admission-dry-run"; "redact_spawn_prompt_preview"; "sha256:trace-spawn-001"),
    span("span-handoff-001"; "span-spawn-001"; "handoff"; "hepta_runtime_agent_harness"; "handoff_event"; "external_handoff_preview_only"; true; "guardrail-external-handoff"; "evidence:persistent-mailbox-handoff-mapping"; "redact_handoff_payload_preview"; "sha256:trace-handoff-001"),
    span("span-mailbox-001"; "span-handoff-001"; "mailbox"; "multi_agent_mailbox"; "mailbox_message"; "ack_deadline_trace_only"; false; "guardrail-mailbox-deadline"; "evidence:mailbox-ack-deadline-contract"; "redact_mailbox_content"; "sha256:trace-mailbox-001"),
    span("span-tool-001"; "span-spawn-001"; "tool"; "tool_invocation"; "tool_call"; "tool_side_effect_preview_only"; true; "guardrail-tool-side-effect"; "evidence:side-effect-boundary"; "redact_tool_arguments"; "sha256:trace-tool-001"),
    span("span-result-001"; "span-tool-001"; "result"; "task_result_envelope"; "TaskResultEnvelope"; "result_validation_report_only"; false; "guardrail-result-contract"; "evidence:task-result-envelope-report-only"; "redact_result_summary"; "sha256:trace-result-001"),
    span("span-artifact-001"; "span-result-001"; "artifact"; "artifact_reference"; "artifact_ref"; "artifact_publication_blocked"; true; "guardrail-artifact-publication"; "evidence:artifact-ref-readback"; "redact_artifact_payload_to_ref"; "sha256:trace-artifact-001"),
    span("span-approval-001"; "span-handoff-001"; "approval"; "operator_approval"; "approval_readback"; "approval_required_but_not_recorded"; true; "guardrail-approval-authority"; "evidence:approval-absence-proof"; "redact_operator_payload"; "sha256:trace-approval-001"),
    span("span-guardrail-001"; "span-approval-001"; "guardrail"; "guardrail_policy"; "blocking_guardrail_preview"; "blocking_guardrail_preview_only"; true; "guardrail-trace-integrity"; "evidence:trace-guardrail-span"; "redact_guardrail_payload"; "sha256:trace-guardrail-001")
  ] as $spans
  | [
    guardrail("guardrail-spawn-admission"; "span-spawn-001"; "scheduler_admission"; "multi_agent_v2_thread_spawn"; "evidence:scheduler-admission-dry-run"),
    guardrail("guardrail-external-handoff"; "span-handoff-001"; "external_handoff"; "hepta_runtime_agent_harness"; "evidence:persistent-mailbox-handoff-mapping"),
    guardrail("guardrail-tool-side-effect"; "span-tool-001"; "side_effect_boundary"; "tool_invocation"; "evidence:side-effect-boundary"),
    guardrail("guardrail-artifact-publication"; "span-artifact-001"; "artifact_publication"; "artifact_reference"; "evidence:artifact-ref-readback"),
    guardrail("guardrail-approval-authority"; "span-approval-001"; "approval_authority"; "operator_approval"; "evidence:approval-absence-proof"),
    guardrail("guardrail-trace-integrity"; "span-guardrail-001"; "trace_integrity"; "guardrail_policy"; "evidence:trace-guardrail-span")
  ] as $guardrails
  | [
    binding("multi_agent_v2_thread_spawn"; "spawn_agent"; ["plan", "spawn", "mailbox", "guardrail"]; ["guardrail-spawn-admission", "guardrail-trace-integrity"]),
    binding("agent_jobs_batch_workers"; "report_agent_job_result"; ["tool", "result", "artifact", "guardrail"]; ["guardrail-tool-side-effect", "guardrail-trace-integrity"]),
    binding("hepta_runtime_task_board"; "task_board_terminal_event"; ["result", "artifact", "approval", "guardrail"]; ["guardrail-artifact-publication", "guardrail-approval-authority"]),
    binding("hepta_runtime_agent_harness"; "handoff_event"; ["handoff", "mailbox", "approval", "guardrail"]; ["guardrail-external-handoff", "guardrail-approval-authority"]),
    binding("hepta_runtime_worker_tasks"; "worker_task_run"; ["tool", "result", "artifact", "guardrail"]; ["guardrail-tool-side-effect", "guardrail-artifact-publication"])
  ] as $bindings
  | [
    "hepta_work_graph_agent_role_agent_card_manifest_report_only_gate",
    "hepta_work_graph_append_only_event_store_shadow_path_gate",
    "hepta_work_graph_task_result_envelope_report_only_validator_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  ] as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_trace_guardrail_span_report_only_gate",
      schema_version: "work_graph_trace_guardrail_span_report_only_v1",
      preview_mode: "report_only_trace_guardrail_span_no_live_blocking",
      required_wire_field_count: ($fields | length),
      span_count: ($spans | length),
      blocking_guardrail_count: ($spans | map(select(.blocking_guardrail_required == true)) | length),
      source_binding_count: ($bindings | length),
      required_prior_gate_count: ($required_prior_gates | length),
      required_wire_fields: $fields,
      spans: $spans,
      guardrail_bindings: $guardrails,
      source_bindings: $bindings,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate",
      trace_spine_complete: true,
      blocking_guardrail_preview_complete: true,
      report_only_guardrail_attached: true,
      live_guardrail_enforcement_enabled: false,
      ready_for_agent_jobs_task_board_report_only_emission: true,
      ready_for_live_execution: false,
      source_probes: {
        trace_guardrail_span_report_only: {
          rust_module_present: $rust_module_present,
          report_script_present: $report_script_present,
          gate_script_present: $gate_script_present
        },
        agent_role_agent_card_manifest_report_only: {
          gate_script_present: $agent_card_gate_script_present
        },
        append_only_event_store_shadow_path: {
          gate_script_present: $shadow_path_gate_script_present
        },
        task_result_envelope_report_only_validator: {
          gate_script_present: $task_result_gate_script_present
        },
        scheduler_admission_dry_run_enforcement: {
          gate_script_present: $admission_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        trace_persisted: false,
        guardrail_persisted: false,
        guardrail_enforcement_enabled: false,
        approval_recorded: false,
        side_effect_lock_established: false,
        scheduler_admission_enforced: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
