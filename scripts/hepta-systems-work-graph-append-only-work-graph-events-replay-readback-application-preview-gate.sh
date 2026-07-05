#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-append-only-work-graph-events-replay-readback-application-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-append-only-work-graph-events-replay-readback-application-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_append_only_work_graph_events_replay_readback_application_preview_gate"
  and .schema_version == "work_graph_append_only_work_graph_events_replay_readback_application_preview_v1"
  and .preview_mode == "read_only_append_only_work_graph_events_replay_readback_application_preview_no_mutation"
  and .readback_plan_count == 12
  and .application_plan_count == 12
  and .source_outcome_count == 12
  and .replay_readback_contract_ready_preview_count == 12
  and .stage_application_count == 8
  and .evidence_field_application_count == 12
  and .guard_application_count == 11
  and .blocker_application_count == 5
  and .application_guard_count == 11
  and .blocker_count == 5
  and .required_prior_gate_count == 19
' >/dev/null <<<"$report"

jq -e '
  (.application_plans | all(
    .application_state == "work_graph_events_replay_readback_contract_ready_preview_after_application"
    and .readback_verified_by_preview == true
    and .replay_readback_contract_ready_preview == true
    and .applies_to_runtime == false
    and .persists_work_graph_events == false
    and .writes_wal == false
    and .writes_checkpoint == false
    and .executes_replay == false
    and .executes_readback == false
    and .executes_rollback == false
    and .mutates_idempotency_index == false
    and .enforces_adapter_projection == false
  ))
  and (.source_outcomes | all(
    .replay_readback_contract_ready_preview == true
    and .ready_for_replay_readback_readiness_rerun_preview == true
    and .ready_for_append_only_work_graph_events == false
    and .ready_for_replay_readback_execution == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.stage_applications | length == 8 and all(
    .contract_ready_preview == true
    and .persists_work_graph_events == false
    and .executes_replay == false
    and .executes_readback == false
    and .executes_rollback == false
  ))
  and (.evidence_field_applications | length == 12 and all(
    .evidence_contract_ready_preview == true
    and .persists_evidence == false
  ))
  and (.guard_applications | length == 11 and all(
    .required_before_replay_readback_execution == true
    and .mutates_runtime == false
  ))
  and (.blocker_applications | length == 5 and all(
    .readback_verified_by_preview == true
    and .mutates_runtime == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.blocker_applications | map(select(.clears_application_missing_blocker == true)) | length) == 1
  and (.blockers | map(.id) == [
    "append_only_work_graph_events_disabled",
    "replay_readback_execution_disabled",
    "runtime_canonical_adapter_enforcement_disabled",
    "canonical_adapter_projection_partial_or_gap",
    "work_graph_events_replay_readback_readiness_rerun_missing"
  ])
  and (.blockers | map(select(.id == "canonical_adapter_projection_partial_or_gap"))[0].affected_source_surface_ids | length) == 7
  and (.application_guards | all(
    .required_before_append_only_events == true
    and .satisfied_by_preview == true
  ))
  and (.required_prior_gates[-1] == "hepta_work_graph_append_only_work_graph_events_replay_readback_readback_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_preview_gate"
  and .ready_for_replay_readback_readiness_rerun_preview == true
  and .ready_for_append_only_work_graph_events == false
  and .ready_for_replay_readback_execution == false
  and .ready_for_runtime_adapter_enforcement == false
  and .ready_for_live_execution == false
' >/dev/null <<<"$report"

jq -e '
  .source_probes.append_only_work_graph_events_replay_readback_application.rust_module_present == true
  and .source_probes.append_only_work_graph_events_replay_readback_application.report_script_present == true
  and .source_probes.append_only_work_graph_events_replay_readback_application.gate_script_present == true
  and .source_probes.append_only_work_graph_events_replay_readback_readback.rust_module_present == true
  and .source_probes.append_only_work_graph_events_replay_readback_readback.gate_script_present == true
  and .source_probes.append_only_work_graph_events_replay_readback_readback.upstream_gate == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_append_only_work_graph_events_replay_readback_application --lib

echo "Hepta WorkGraph append-only WorkGraph events replay/readback application preview gate passed"
