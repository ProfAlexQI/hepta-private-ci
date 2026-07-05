#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-append-only-work-graph-events-shadow-write-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-append-only-work-graph-events-shadow-write-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "blocked"
  and .gate == "hepta_work_graph_append_only_work_graph_events_shadow_write_preview_gate"
  and .schema_version == "work_graph_append_only_work_graph_events_shadow_write_preview_v1"
  and .preview_mode == "read_only_append_only_work_graph_events_shadow_write_preview_no_persistence"
  and .source_surface_count == 12
  and .shadow_write_plan_count == 12
  and .event_schema_count == 11
  and .source_event_binding_count == 27
  and .shadow_write_stage_count == 6
  and .shadow_write_stage_source_ref_count == 72
  and .shadow_write_stage_contract_ref_count == 34
  and .shadow_write_plan_stage_ref_count == 72
  and .shadow_write_plan_event_schema_ref_count == 27
  and .idempotency_key_field_ref_count == 60
  and .shadow_write_contract_ready_preview_count == 12
  and .append_only_work_graph_events_primary_blocked_source_count == 12
  and .partial_or_gap_blocked_source_count == 7
  and .shadow_write_enabled_source_count == 0
  and .guard_count == 10
  and .blocker_count == 4
  and .required_prior_gate_count == 13
' >/dev/null <<<"$report"

jq -e '
  (.shadow_write_plans | all(
    .shadow_write_state == "append_only_work_graph_events_shadow_write_contract_defined_preview_only"
    and .canonical_adapter_inventory_contract_ready == true
    and .shadow_write_contract_ready_preview == true
    and .applies_to_runtime == false
    and .persists_work_graph_events == false
    and .writes_wal == false
    and .writes_checkpoint == false
    and .executes_replay == false
    and .executes_readback == false
    and .mutates_runtime == false
    and (.required_shadow_write_stage_ids | length == 6)
    and (.idempotency_key_field_ids | length == 5)
  ))
  and (.shadow_write_plans | map(select(.source_surface_id == "multi_agent_v2_thread_spawn"))[0].event_schema_ids == ["AgentTaskSpawned","MessageLinked","TimelineEventAppended"])
  and (.shadow_write_plans | map(select(.source_surface_id == "hepta_runtime_worker_tasks"))[0].event_schema_ids == ["TaskResultReported","ArtifactProduced","TimelineEventAppended"])
  and (.shadow_write_plans | map(select(.source_surface_id == "hepta_runtime_approval_broker"))[0].event_schema_ids == ["ApprovalRequired","ApprovalRecorded"])
' >/dev/null <<<"$report"

jq -e '
  (.event_schemas | map(.id) == [
    "PlanStepCreated",
    "AgentTaskSpawned",
    "MessageLinked",
    "TaskResultReported",
    "ArtifactProduced",
    "ApprovalRequired",
    "ApprovalRecorded",
    "LeaseAcquired",
    "LeaseReleased",
    "GateEvaluated",
    "TimelineEventAppended"
  ])
  and (.event_schemas | all(
    .redaction_required == true
    and .payload_hash_required == true
    and .replay_readback_required == true
    and .shadow_write_only == true
    and .persists_event_after_preview == false
  ))
  and (.stage_plans | map(.id) == [
    "work_graph_event_schema_contract",
    "work_graph_event_source_surface_mapping",
    "work_graph_event_idempotency_key_contract",
    "work_graph_event_replay_readback_guard",
    "work_graph_event_no_persistence_guard",
    "work_graph_event_blocker_mapping"
  ])
  and (.stage_plans | all(
    .contract_ready_preview == true
    and .persists_work_graph_events_after_preview == false
    and .executes_replay_after_preview == false
    and .executes_readback_after_preview == false
    and .mutates_runtime_after_preview == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.blockers | map(.id) == [
    "append_only_work_graph_events_disabled",
    "runtime_canonical_adapter_enforcement_disabled",
    "canonical_adapter_projection_partial_or_gap",
    "append_only_work_graph_events_shadow_write_readback_missing"
  ])
  and (.blockers | map(select(.id == "append_only_work_graph_events_disabled"))[0].affected_source_surface_ids | length) == 12
  and (.blockers | map(select(.id == "canonical_adapter_projection_partial_or_gap"))[0].affected_source_surface_ids | length) == 7
  and (.blockers | map(select(.id == "append_only_work_graph_events_shadow_write_readback_missing"))[0].affected_source_surface_ids | length) == 12
  and (.guards | all(.required_before_shadow_write == true and .satisfied_by_preview == false))
' >/dev/null <<<"$report"

jq -e '
  (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_append_only_work_graph_events_shadow_write_readback_preview_gate"
  and .ready_for_shadow_write_readback_preview == true
  and .ready_for_shadow_write_application_preview == false
  and .ready_for_append_only_work_graph_events == false
  and .ready_for_replay_readback == false
  and .ready_for_runtime_adapter_enforcement == false
  and .ready_for_scheduler_admission_enforcement == false
  and .ready_for_task_result_enforcement == false
  and .ready_for_role_manifest_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.append_only_work_graph_events_shadow_write.rust_module_present == true
  and .source_probes.append_only_work_graph_events_shadow_write.report_script_present == true
  and .source_probes.append_only_work_graph_events_shadow_write.gate_script_present == true
  and .source_probes.canonical_adapter_inventory_rerun.upstream_gate == true
  and .source_probes.canonical_adapter_inventory_rerun.gate_script_present == true
  and .source_probes.canonical_adapter_inventory_application.upstream_gate == true
  and .source_probes.canonical_adapter_inventory_application.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_append_only_work_graph_events_shadow_write --lib

echo "Hepta WorkGraph append-only WorkGraph events shadow-write preview gate passed"
