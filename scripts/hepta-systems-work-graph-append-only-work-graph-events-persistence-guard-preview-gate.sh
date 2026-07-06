#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-append-only-work-graph-events-persistence-guard-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-append-only-work-graph-events-persistence-guard-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "blocked"
  and .gate == "hepta_work_graph_append_only_work_graph_events_persistence_guard_preview_gate"
  and .schema_version == "work_graph_append_only_work_graph_events_persistence_guard_preview_v1"
  and .preview_mode == "read_only_append_only_work_graph_events_persistence_guard_preview_no_persistence"
  and .source_surface_count == 12
  and .persistence_guard_plan_count == 12
  and .persistence_guard_stage_count == 6
  and .persistence_guard_stage_source_ref_count == 72
  and .persistence_guard_stage_contract_ref_count == 31
  and .persistence_guard_plan_stage_ref_count == 72
  and .persistence_guard_plan_evidence_field_ref_count == 120
  and .append_only_work_graph_events_primary_blocked_source_count == 12
  and .replay_readback_execution_blocked_source_count == 12
  and .partial_or_gap_blocked_source_count == 7
  and .guard_count == 11
  and .blocker_count == 5
  and .required_prior_gate_count == 21
' >/dev/null <<<"$report"

jq -e '
  (.persistence_guard_plans | all(
    .persistence_guard_state == "work_graph_events_persistence_guard_contract_ready_preview"
    and (.required_persistence_guard_stage_ids | length == 6)
    and (.expected_evidence_field_ids | length == 10)
    and .persistence_guard_contract_ready_preview == true
    and .event_store_enablement_contract_ready_preview == true
    and .replay_readback_prerequisite_ready_preview == true
    and .adapter_enforcement_guard_ready_preview == true
    and .no_persistence_guard_ready_preview == true
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
  and (.persistence_guard_stage_plans | all(
    .contract_ready_preview == true
    and .expected_runtime_state == "preview_only_no_event_persistence"
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
  (.persistence_guard_stage_plans | map(.id) == [
    "work_graph_events_persistence_guard_contract",
    "work_graph_events_event_store_enablement_contract",
    "work_graph_events_replay_readback_execution_prerequisite",
    "work_graph_events_adapter_enforcement_guard",
    "work_graph_events_no_persistence_guard",
    "work_graph_events_persistence_guard_blocker_mapping"
  ])
  and (.guards | all(
    .required_before_event_store_enablement == true
    and .satisfied_by_preview == false
  ))
  and (.blockers | map(.id) == [
    "append_only_work_graph_events_disabled",
    "replay_readback_execution_disabled",
    "runtime_canonical_adapter_enforcement_disabled",
    "canonical_adapter_projection_partial_or_gap",
    "append_only_work_graph_events_persistence_guard_readback_missing"
  ])
  and (.blockers | map(select(.id == "canonical_adapter_projection_partial_or_gap"))[0].affected_source_surface_ids | length) == 7
  and (.blockers | all(.required_before_event_store_enablement == true))
' >/dev/null <<<"$report"

jq -e '
  (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_append_only_work_graph_events_persistence_guard_readback_preview_gate"
  and .ready_for_persistence_guard_readback_preview == true
  and .ready_for_append_only_work_graph_events == false
  and .ready_for_event_store_enablement == false
  and .ready_for_replay_readback_execution == false
  and .ready_for_runtime_adapter_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.append_only_work_graph_events_persistence_guard_preview.rust_module_present == true
  and .source_probes.append_only_work_graph_events_persistence_guard_preview.report_script_present == true
  and .source_probes.append_only_work_graph_events_persistence_guard_preview.gate_script_present == true
  and .source_probes.work_graph_events_replay_readback_rerun.rust_module_present == true
  and .source_probes.work_graph_events_replay_readback_rerun.gate_script_present == true
  and .source_probes.work_graph_events_replay_readback_rerun.upstream_gate == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_append_only_work_graph_events_persistence_guard_preview --lib

echo "Hepta WorkGraph append-only WorkGraph events persistence guard preview gate passed"
