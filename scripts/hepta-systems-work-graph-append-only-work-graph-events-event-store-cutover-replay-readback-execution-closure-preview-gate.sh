#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-replay-readback-execution-closure-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-append-only-work-graph-events-event-store-cutover-replay-readback-execution-closure-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "blocked"
  and .gate == "hepta_work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_preview_gate"
  and .schema_version == "work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_preview_v1"
  and .preview_mode == "read_only_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_preview_no_persistence"
  and .source_surface_count == 7
  and .event_store_cutover_replay_readback_execution_closure_plan_count == 7
  and .event_store_cutover_replay_readback_execution_closure_stage_count == 6
  and .event_store_cutover_replay_readback_execution_closure_stage_source_ref_count == 42
  and .event_store_cutover_replay_readback_execution_closure_stage_contract_ref_count == 31
  and .event_store_cutover_replay_readback_execution_closure_plan_stage_ref_count == 42
  and .event_store_cutover_replay_readback_execution_closure_plan_evidence_field_ref_count == 70
  and .append_only_work_graph_events_primary_blocked_source_count == 7
  and .replay_readback_execution_blocked_source_count == 7
  and .runtime_adapter_enforcement_blocked_source_count == 0
  and .guard_count == 11
  and .blocker_count == 3
  and .required_prior_gate_count == 65
' >/dev/null <<<"$report"

jq -e '
  (.event_store_cutover_replay_readback_execution_closure_plans | all(
    .event_store_cutover_replay_readback_execution_closure_state == "work_graph_events_event_store_cutover_replay_readback_execution_closure_packet_ready_preview"
    and (.required_event_store_cutover_replay_readback_execution_closure_stage_ids | length == 6)
    and (.expected_evidence_field_ids | length == 10)
    and .event_store_cutover_replay_readback_execution_closure_contract_ready_preview == true
    and .append_only_event_store_prerequisite_ready_preview == true
    and .replay_execution_disabled_guard_ready_preview == true
    and .readback_execution_disabled_guard_ready_preview == true
    and .no_execution_proof_ready_preview == true
    and .applies_to_runtime == false
    and .persists_work_graph_events == false
    and .enables_event_store == false
    and .writes_wal == false
    and .writes_checkpoint == false
    and .executes_replay == false
    and .executes_readback == false
    and .enforces_adapter_projection == false
    and .mutates_runtime == false
  ))
  and (.event_store_cutover_replay_readback_execution_closure_stage_plans | all(
    .contract_ready_preview == true
    and .expected_runtime_state == "preview_only_no_event_store_cutover_replay_readback_execution_closure"
    and .persists_work_graph_events_after_preview == false
    and .enables_event_store_after_preview == false
    and .writes_wal_after_preview == false
    and .writes_checkpoint_after_preview == false
    and .executes_replay_after_preview == false
    and .executes_readback_after_preview == false
    and .enforces_adapter_projection_after_preview == false
    and .mutates_runtime_after_preview == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.event_store_cutover_replay_readback_execution_closure_stage_plans | map(.id) == [
    "work_graph_events_replay_readback_execution_closure_packet",
    "work_graph_events_replay_execution_disabled_guard",
    "work_graph_events_readback_execution_disabled_guard",
    "work_graph_events_append_only_event_store_prerequisite",
    "work_graph_events_event_store_cutover_no_execution_proof",
    "work_graph_events_replay_readback_execution_closure_blocker_mapping"
  ])
  and (.guards | all(
    .required_before_event_store_cutover_replay_readback_execution_closure == true
    and .satisfied_by_preview == false
  ))
  and (.blockers | map(.id) == [
    "append_only_work_graph_events_disabled",
    "replay_readback_execution_disabled",
    "append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_readback_missing"
  ])
  and (.blockers | map(select(.id == "replay_readback_execution_disabled"))[0].affected_source_surface_ids | length) == 7
  and (.blockers | all(.required_before_event_store_cutover_replay_readback_execution_closure == true))
' >/dev/null <<<"$report"

jq -e '
  (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_rerun_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_readback_preview_gate"
  and .ready_for_event_store_cutover_replay_readback_execution_closure_readback_preview == true
  and .ready_for_append_only_work_graph_events == false
  and .ready_for_event_store_cutover_replay_readback_execution_closure == false
  and .ready_for_replay_readback_execution == false
  and .ready_for_runtime_adapter_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_preview.rust_module_present == true
  and .source_probes.append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_preview.report_script_present == true
  and .source_probes.append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_preview.gate_script_present == true
  and .source_probes.work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_rerun.rust_module_present == true
  and .source_probes.work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_rerun.gate_script_present == true
  and .source_probes.work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_rerun.upstream_gate == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_preview --lib

echo "Hepta WorkGraph append-only WorkGraph events event-store cutover replay/readback execution closure preview gate passed"
