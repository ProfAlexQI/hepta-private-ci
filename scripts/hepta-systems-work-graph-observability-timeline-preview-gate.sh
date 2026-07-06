#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-observability-timeline-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-observability-timeline-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_observability_timeline_preview_gate"
  and .schema_version == "work_graph_observability_timeline_preview_v1"
  and .preview_mode == "read_only_timeline_shape_preview_no_persistence"
  and .event_type_count == 9
  and (.event_types | length) == .event_type_count
  and (.event_types | map(.id) == [
    "plan_step_observed",
    "agent_task_spawned",
    "mailbox_progress_observed",
    "tool_invocation_observed",
    "artifact_produced",
    "verification_gate_observed",
    "task_result_observed",
    "scheduler_admission_decision_observed",
    "external_handoff_observed"
  ])
  and (.event_types | all((.required_evidence_fields | index("traceId")) and (.required_evidence_fields | index("nodeId"))))
  and .span_field_count == 11
  and (.span_fields | length) == .span_field_count
  and (.span_fields | map(.wire_name) == [
    "traceId",
    "parentTraceId",
    "nodeId",
    "sourceSurfaceId",
    "eventKind",
    "status",
    "startedAtUnixMs",
    "completedAtUnixMs",
    "evidenceRefs",
    "redactionState",
    "sideEffectClass"
  ])
  and (.span_fields | map(select(.required == true) | .wire_name) == [
    "traceId",
    "nodeId",
    "sourceSurfaceId",
    "eventKind",
    "status",
    "evidenceRefs",
    "redactionState",
    "sideEffectClass"
  ])
  and (.span_fields[] | select(.wire_name == "evidenceRefs") | .redaction_required == true)
  and .timeline_view_count == 5
  and (.timeline_views | length) == .timeline_view_count
  and (.timeline_views | map(.id) == [
    "trace_timeline",
    "node_history",
    "blocking_path",
    "artifact_lineage",
    "operator_audit_excerpt"
  ])
  and (.timeline_views | all(.required == true))
  and .adapter_preview_count == 7
  and (.adapter_previews | length) == .adapter_preview_count
  and (.adapter_previews | map(.source_surface_id) == [
    "app_server_turn_plan_notification",
    "multi_agent_v2_thread_spawn",
    "multi_agent_v2_mailbox_wait",
    "agent_jobs_batch_workers",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_scheduler_store",
    "hepta_runtime_agent_harness"
  ])
  and (.adapter_previews | all(.persistence_enabled == false))
  and (.adapter_previews | all((.required_join_fields | index("traceId")) and (.required_join_fields | index("nodeId"))))
  and .recommended_next_gate == "hepta_work_graph_role_manifest_contract_preview_gate"
  and .ready_for_role_manifest_preview == true
  and .ready_for_operator_dashboard == false
  and .ready_for_live_execution == false
  and .source_probes.observability_timeline.rust_module_present == true
  and .source_probes.observability_timeline.report_script_present == true
  and .source_probes.observability_timeline.gate_script_present == true
  and .source_probes.scheduler_admission_controller.rust_module_present == true
  and .source_probes.scheduler_admission_controller.report_script_present == true
  and .source_probes.scheduler_admission_controller.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_observability_timeline --lib

echo "Hepta WorkGraph observability timeline preview gate passed"
