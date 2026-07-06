#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-final-no-enablement-readiness-rerun-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-final-no-enablement-readiness-rerun-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_final_no_enablement_readiness_rerun_preview_gate"
  and .schema_version == "work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_final_no_enablement_readiness_rerun_preview_v1"
  and .preview_mode == "read_only_projection_enforcement_readiness_work_graph_events_event_store_cutover_final_no_enablement_readiness_rerun_no_execution"
  and .source_surface_count == 7
  and .event_store_cutover_final_no_enablement_readiness_outcome_count == 7
  and .event_store_cutover_final_no_enablement_readiness_application_covered_source_count == 7
  and .previous_ready_surface_count == 7
  and .event_store_cutover_final_no_enablement_readiness_contract_ready_source_count == 7
  and .previous_append_only_work_graph_events_primary_blocked_surface_count == 0
  and .event_store_cutover_final_no_enablement_readiness_application_missing_surface_count_after == 0
  and .append_only_work_graph_events_primary_blocked_surface_count == 0
  and .replay_readback_execution_blocked_surface_count == 0
  and .runtime_adapter_enforcement_blocked_surface_count == 0
  and .append_only_work_graph_events_enabled_source_count == 0
  and .event_store_enabled_source_count == 0
  and .replay_readback_enabled_source_count == 0
  and .runtime_canonical_adapter_enforcement_enabled_source_count == 0
  and .rerun_ready_surface_count == 7
  and .rerun_blocked_surface_count == 0
  and .decision_delta_count == 7
  and .cleared_blocker_count == 1
  and .residual_blocker_count == 0
  and .enforcement_stage_count == 6
  and .required_prior_gate_count == 76
' >/dev/null <<<"$report"

jq -e '
  (.decision_deltas | all(
    .covered_by_event_store_cutover_final_no_enablement_readiness_application_preview == true
    and .event_store_cutover_final_no_enablement_readiness_contract_ready == true
    and .event_store_cutover_final_no_enablement_readiness_application_applied == false
    and .append_only_work_graph_events_enabled == false
    and .event_store_enabled == false
    and .replay_readback_execution_enabled == false
    and .runtime_canonical_adapter_enforcement_enabled == false
    and .work_graph_events_event_store_cutover_final_no_enablement_readiness_rerun_enforcement_decision == "allow_preview_only"
    and .next_required_gate == "hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_operator_review_packet_preview_gate"
  ))
  and (.cleared_blockers | map(.id) == [
    "work_graph_events_event_store_cutover_final_no_enablement_readiness_application_required_for_terminal_review"
  ])
  and .cleared_blockers[0].source_count_before == 7
  and .cleared_blockers[0].source_count_after == 0
' >/dev/null <<<"$report"

jq -e '
  (.residual_blockers | length == 0)
  and (.enforcement_stages | map(.id) == [
    "work_graph_events_event_store_cutover_final_no_enablement_readiness_packets",
    "final_side_effect_false_readback",
    "operator_approval_absence_proof",
    "event_store_switch_disabled_proof",
    "append_only_persistence_disabled_proof",
    "terminal_operator_review_packet_frontier"
  ])
  and (.enforcement_stages | all(.enforcement_enabled == false))
' >/dev/null <<<"$report"

jq -e '
  (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_append_only_work_graph_events_event_store_cutover_final_no_enablement_readiness_application_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_operator_review_packet_preview_gate"
  and .ready_for_event_store_cutover_final_no_enablement_readiness_preview == true
  and .ready_for_append_only_work_graph_events == false
  and .ready_for_event_store_cutover_final_no_enablement_readiness == false
  and .ready_for_replay_readback == false
  and .ready_for_runtime_adapter_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_final_no_enablement_readiness_rerun.rust_module_present == true
  and .source_probes.unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_final_no_enablement_readiness_rerun.report_script_present == true
  and .source_probes.unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_final_no_enablement_readiness_rerun.gate_script_present == true
  and .source_probes.append_only_work_graph_events_event_store_cutover_final_no_enablement_readiness_application.rust_module_present == true
  and .source_probes.append_only_work_graph_events_event_store_cutover_final_no_enablement_readiness_application.gate_script_present == true
  and .source_probes.append_only_work_graph_events_event_store_cutover_final_no_enablement_readiness_application.upstream_gate == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_final_no_enablement_readiness_rerun_preview --lib

echo "Hepta WorkGraph unified projection enforcement readiness WorkGraph events event-store cutover final no-enablement readiness rerun preview gate passed"
