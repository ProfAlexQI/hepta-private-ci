#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-readback-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-readback-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_readback_preview_gate"
  and .schema_version == "work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_readback_preview_v1"
  and .preview_mode == "read_only_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_readback_preview_no_execution"
  and .source_surface_count == 7
  and .preview_plan_count == 7
  and .readback_plan_count == 7
  and .stage_assertion_count == 6
  and .evidence_field_assertion_count == 7
  and .guard_assertion_count == 11
  and .blocker_mapping_assertion_count == 1
  and .drift_detector_count == 7
  and .blocker_count == 2
  and .required_prior_gate_count == 90
' >/dev/null <<<"$report"

jq -e '
  (.readback_plans | all(
    .readback_status == "readback_plan_ready"
    and .expected_stage_count == 6
    and .expected_evidence_field_count == 10
    and .readback_execution_enabled == false
    and .replay_execution_enabled == false
    and .event_store_cutover_terminal_no_cutover_receipt_enabled == false
    and .persists_work_graph_events == false
  ))
  and (.stage_assertions | all(
    .contract_ready_preview == true
    and .event_store_enabled_after_readback == false
    and .execution_enabled_after_readback == false
    and .persistence_enabled_after_readback == false
  ))
  and (.evidence_field_assertions | all(
    .evidence_contract_ready_preview == true
    and .persists_evidence_after_readback == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.guard_assertions | all(
    .required_before_event_store_cutover_terminal_no_cutover_receipt == true
    and .satisfied_by_preview == false
  ))
  and (.blocker_mapping_assertions | all(.blocks_event_store_cutover_terminal_no_cutover_receipt == true))
  and (.blockers | map(.id) == [
    "append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_readback_not_executed",
    "append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_application_missing"
  ])
  and (.drift_detectors | all(.drift_budget == 0))
' >/dev/null <<<"$report"

jq -e '
  (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_application_preview_gate"
  and .ready_for_event_store_cutover_terminal_no_cutover_receipt_application_preview == true
  and .ready_for_append_only_work_graph_events == false
  and .ready_for_event_store_cutover_terminal_no_cutover_receipt == false
  and .ready_for_replay_readback_execution == false
  and .ready_for_runtime_adapter_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_readback.rust_module_present == true
  and .source_probes.append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_readback.report_script_present == true
  and .source_probes.append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_readback.gate_script_present == true
  and .source_probes.append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_preview.rust_module_present == true
  and .source_probes.append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_preview.gate_script_present == true
  and .source_probes.append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_preview.upstream_gate == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_readback_preview --lib

echo "Hepta WorkGraph append-only WorkGraph events event-store cutover terminal no-cutover receipt readback preview gate passed"
