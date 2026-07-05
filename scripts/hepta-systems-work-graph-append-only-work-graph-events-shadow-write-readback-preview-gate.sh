#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-append-only-work-graph-events-shadow-write-readback-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-append-only-work-graph-events-shadow-write-readback-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_append_only_work_graph_events_shadow_write_readback_preview_gate"
  and .schema_version == "work_graph_append_only_work_graph_events_shadow_write_readback_preview_v1"
  and .preview_mode == "read_only_append_only_work_graph_events_shadow_write_readback_preview_no_execution"
  and .source_surface_count == 12
  and .preview_plan_count == 12
  and .readback_plan_count == 12
  and .event_schema_assertion_count == 11
  and .stage_assertion_count == 6
  and .source_mapping_assertion_count == 12
  and .event_binding_assertion_count == 27
  and .idempotency_key_assertion_count == 12
  and .guard_assertion_count == 10
  and .blocker_mapping_assertion_count == 4
  and .drift_detector_count == 7
  and .blocker_count == 5
  and .required_prior_gate_count == 14
' >/dev/null <<<"$report"

jq -e '
  (.readback_plans | all(
    .readback_status == "readback_plan_ready"
    and .readback_execution_enabled == false
    and .persists_work_graph_events == false
    and .next_required_gate == "hepta_work_graph_append_only_work_graph_events_shadow_write_application_preview_gate"
  ))
  and (.event_schema_assertions | all(.shadow_write_only == true and .persists_event_after_readback == false))
  and (.stage_assertions | all(.contract_ready_preview == true and .persistence_enabled_after_readback == false))
  and (.source_mapping_assertions | all(.source_mapping_ready_preview == true and .persists_mapping_after_readback == false))
  and (.event_binding_assertions | all(.binding_ready_preview == true and .persists_event_after_readback == false))
  and (.idempotency_key_assertions | all(.idempotency_key_ready_preview == true and .mutates_idempotency_index_after_readback == false))
  and (.guard_assertions | all(.required_before_shadow_write == true and .satisfied_by_preview == false))
  and (.blocker_mapping_assertions | all(.blocks_shadow_write_persistence == true))
  and (.drift_detectors | all(.drift_budget == 0))
' >/dev/null <<<"$report"

jq -e '
  (.blockers | map(.id) == [
    "append_only_work_graph_events_shadow_write_readback_not_executed",
    "append_only_work_graph_events_shadow_write_application_missing",
    "append_only_work_graph_events_disabled",
    "runtime_canonical_adapter_enforcement_disabled",
    "canonical_adapter_projection_partial_or_gap"
  ])
  and (.blockers | map(select(.id == "canonical_adapter_projection_partial_or_gap"))[0].affected_source_surface_ids | length) == 7
  and (.blockers | map(select(.id != "canonical_adapter_projection_partial_or_gap")) | all((.affected_source_surface_ids | length) == 12))
  and (.required_prior_gates[-1] == "hepta_work_graph_append_only_work_graph_events_shadow_write_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_append_only_work_graph_events_shadow_write_application_preview_gate"
  and .ready_for_shadow_write_application_preview == true
  and .ready_for_append_only_work_graph_events == false
  and .ready_for_replay_readback == false
  and .ready_for_runtime_adapter_enforcement == false
  and .ready_for_live_execution == false
' >/dev/null <<<"$report"

jq -e '
  .source_probes.append_only_work_graph_events_shadow_write_readback.rust_module_present == true
  and .source_probes.append_only_work_graph_events_shadow_write_readback.report_script_present == true
  and .source_probes.append_only_work_graph_events_shadow_write_readback.gate_script_present == true
  and .source_probes.append_only_work_graph_events_shadow_write_preview.rust_module_present == true
  and .source_probes.append_only_work_graph_events_shadow_write_preview.gate_script_present == true
  and .source_probes.append_only_work_graph_events_shadow_write_preview.upstream_gate == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_append_only_work_graph_events_shadow_write_readback --lib

echo "Hepta WorkGraph append-only WorkGraph events shadow-write readback preview gate passed"
