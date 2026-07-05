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

observability_timeline_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_observability_timeline.rs
)"
observability_timeline_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-observability-timeline-preview-report.sh
)"
observability_timeline_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-observability-timeline-preview-gate.sh
)"
scheduler_admission_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_scheduler_admission_controller.rs
)"
scheduler_admission_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-controller-preview-report.sh
)"
scheduler_admission_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-controller-preview-gate.sh
)"

jq -n \
  --argjson observability_timeline_rust_module_present "$observability_timeline_rust_module_present" \
  --argjson observability_timeline_report_script_present "$observability_timeline_report_script_present" \
  --argjson observability_timeline_gate_script_present "$observability_timeline_gate_script_present" \
  --argjson scheduler_admission_rust_module_present "$scheduler_admission_rust_module_present" \
  --argjson scheduler_admission_report_script_present "$scheduler_admission_report_script_present" \
  --argjson scheduler_admission_gate_script_present "$scheduler_admission_gate_script_present" \
  '
  def event($id; $node_kind; $fields; $purpose): {
    id: $id,
    node_kind: $node_kind,
    required_evidence_fields: $fields,
    purpose: $purpose
  };
  def field($wire; $required; $redaction; $purpose): {
    wire_name: $wire,
    required: $required,
    redaction_required: $redaction,
    purpose: $purpose
  };
  def view($id; $purpose): {
    id: $id,
    required: true,
    purpose: $purpose
  };
  def adapter($source; $events; $joins; $blockers): {
    source_surface_id: $source,
    event_type_ids: $events,
    required_join_fields: $joins,
    persistence_enabled: false,
    blocker_ids: $blockers
  };
  [
    event("plan_step_observed"; "plan_step"; ["traceId", "nodeId", "sourceSurfaceId", "status"]; "plan notifications become joinable timeline events without becoming execution authority"),
    event("agent_task_spawned"; "agent_task"; ["traceId", "nodeId", "parentTraceId", "status"]; "subagent lineage and parent authority are visible across thread boundaries"),
    event("mailbox_progress_observed"; "agent_task"; ["traceId", "nodeId", "eventKind", "evidenceRefs"]; "mailbox progress can be distinguished from terminal task status"),
    event("tool_invocation_observed"; "agent_task"; ["traceId", "nodeId", "eventKind", "sideEffectClass"]; "tool calls can be audited without embedding private tool payloads"),
    event("artifact_produced"; "artifact"; ["traceId", "nodeId", "evidenceRefs", "redactionState"]; "artifact lineage is traceable by id and hash rather than raw content"),
    event("verification_gate_observed"; "verification_gate"; ["traceId", "nodeId", "status", "evidenceRefs"]; "gate results can explain promotion, block, retry, and rollback decisions"),
    event("task_result_observed"; "worker_task"; ["traceId", "nodeId", "status", "evidenceRefs"]; "TaskResult projection becomes the terminal join point for work completion"),
    event("scheduler_admission_decision_observed"; "scheduler_run"; ["traceId", "nodeId", "status", "evidenceRefs"]; "scheduler allow or deny decisions are reviewable before enforcement"),
    event("external_handoff_observed"; "external_handoff"; ["traceId", "nodeId", "status", "redactionState"]; "external actions remain previewable without performing delivery")
  ] as $event_types
  | [
    field("traceId"; true; false; "stable join key for the full work graph"),
    field("parentTraceId"; false; false; "optional parent trace for spawned or retried work"),
    field("nodeId"; true; false; "stable WorkGraph node identity"),
    field("sourceSurfaceId"; true; false; "originating planning, agent, worker, scheduler, or harness surface"),
    field("eventKind"; true; false; "canonical event type for timeline ordering"),
    field("status"; true; false; "normalized node or event status"),
    field("startedAtUnixMs"; false; false; "start timestamp when known"),
    field("completedAtUnixMs"; false; false; "completion timestamp when known"),
    field("evidenceRefs"; true; true; "redacted evidence ids, hashes, or report references"),
    field("redactionState"; true; false; "whether sensitive payload was omitted, hashed, or unavailable"),
    field("sideEffectClass"; true; false; "declares none, preview, filesystem, runtime, gateway, or external impact")
  ] as $span_fields
  | [
    view("trace_timeline"; "events ordered by traceId and timestamp"),
    view("node_history"; "all events for a single WorkGraph node"),
    view("blocking_path"; "why a node is not runnable or not promotable"),
    view("artifact_lineage"; "producer, verifier, and consumer path for an artifact"),
    view("operator_audit_excerpt"; "redacted operator-facing explanation for a gate or handoff")
  ] as $timeline_views
  | [
    adapter("app_server_turn_plan_notification"; ["plan_step_observed"]; ["traceId", "nodeId", "sourceSurfaceId"]; ["turn_plan_notification_timeline_adapter_not_enforced"]),
    adapter("multi_agent_v2_thread_spawn"; ["agent_task_spawned"]; ["traceId", "parentTraceId", "nodeId"]; ["agent_task_timeline_adapter_not_enforced"]),
    adapter("multi_agent_v2_mailbox_wait"; ["mailbox_progress_observed"]; ["traceId", "nodeId", "eventKind"]; ["mailbox_progress_timeline_adapter_not_enforced"]),
    adapter("agent_jobs_batch_workers"; ["agent_task_spawned", "task_result_observed"]; ["traceId", "nodeId", "status"]; ["agent_job_timeline_adapter_not_enforced"]),
    adapter("hepta_runtime_worker_tasks"; ["tool_invocation_observed", "artifact_produced", "task_result_observed"]; ["traceId", "nodeId", "evidenceRefs"]; ["worker_task_timeline_adapter_not_enforced"]),
    adapter("hepta_runtime_scheduler_store"; ["scheduler_admission_decision_observed"]; ["traceId", "nodeId", "status"]; ["scheduler_timeline_adapter_not_enforced"]),
    adapter("hepta_runtime_agent_harness"; ["external_handoff_observed", "artifact_produced", "verification_gate_observed"]; ["traceId", "nodeId", "redactionState"]; ["agent_harness_timeline_adapter_not_enforced"])
  ] as $adapter_previews
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_observability_timeline_preview_gate",
      schema_version: "work_graph_observability_timeline_preview_v1",
      preview_mode: "read_only_timeline_shape_preview_no_persistence",
      event_type_count: ($event_types | length),
      span_field_count: ($span_fields | length),
      timeline_view_count: ($timeline_views | length),
      adapter_preview_count: ($adapter_previews | length),
      event_types: $event_types,
      span_fields: $span_fields,
      timeline_views: $timeline_views,
      adapter_previews: $adapter_previews,
      recommended_next_gate: "hepta_work_graph_role_manifest_contract_preview_gate",
      ready_for_role_manifest_preview: true,
      ready_for_operator_dashboard: false,
      ready_for_live_execution: false,
      source_probes: {
        observability_timeline: {
          rust_module_present: $observability_timeline_rust_module_present,
          report_script_present: $observability_timeline_report_script_present,
          gate_script_present: $observability_timeline_gate_script_present
        },
        scheduler_admission_controller: {
          rust_module_present: $scheduler_admission_rust_module_present,
          report_script_present: $scheduler_admission_report_script_present,
          gate_script_present: $scheduler_admission_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        timeline_persisted: false,
        runtime_mutation_performed: false,
        scheduler_cutover_performed: false,
        dashboard_exposed: false,
        external_export_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
