#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-canonical-adapter-inventory-application-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-canonical-adapter-inventory-application-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_canonical_adapter_inventory_application_preview_gate"
  and .schema_version == "work_graph_canonical_adapter_inventory_application_preview_v1"
  and .preview_mode == "read_only_canonical_adapter_inventory_application_preview_no_mutation"
  and .readback_plan_count == 12
  and .application_plan_count == 12
  and .source_outcome_count == 12
  and .canonical_adapter_inventory_contract_ready_preview_count == 12
  and .identity_application_count == 12
  and .edge_kind_application_count == 12
  and .collection_binding_application_count == 12
  and .timeline_event_application_count == 12
  and .blocker_application_count == 8
  and .application_guard_count == 8
  and .blocker_count == 4
  and .required_prior_gate_count == 11
' >/dev/null <<<"$report"

jq -e '
  (.application_plans | all(
    .application_state == "canonical_adapter_inventory_contract_ready_preview_after_application"
    and .readback_verified_by_preview == true
    and .canonical_adapter_inventory_contract_ready_preview == true
    and .applies_to_runtime == false
    and .persists_work_graph_events == false
    and .enforces_adapter_projection == false
    and .mutates_scheduler_admission == false
    and .mutates_task_result_enforcement == false
    and .mutates_role_manifest_enforcement == false
  ))
  and (.source_outcomes | all(
    .canonical_adapter_inventory_contract_ready_preview == true
    and .ready_for_canonical_adapter_inventory_readiness_rerun_preview == true
    and .ready_for_append_only_work_graph_events == false
    and .applies_to_runtime == false
  ))
  and (.identity_applications | all(.persists_identity == false))
  and (.edge_kind_applications | all(.persists_edges == false))
  and (.collection_binding_applications | all(.persists_store_projection == false))
  and (.timeline_event_applications | all(.persists_timeline == false))
' >/dev/null <<<"$report"

jq -e '
  (.blockers | map(.id) == [
    "append_only_work_graph_events_disabled",
    "runtime_canonical_adapter_enforcement_disabled",
    "canonical_adapter_projection_partial_or_gap",
    "canonical_adapter_inventory_readiness_rerun_missing"
  ])
  and (.blockers | map(select(.id == "canonical_adapter_projection_partial_or_gap"))[0].affected_source_surface_ids | length) == 7
  and (.application_guards | all(.required_before_runtime_enforcement == true and .satisfied_by_preview == true))
  and (.required_prior_gates[-1] == "hepta_work_graph_canonical_adapter_inventory_readback_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview_gate"
  and .ready_for_canonical_adapter_inventory_readiness_rerun_preview == true
  and .ready_for_append_only_work_graph_events == false
  and .ready_for_runtime_adapter_enforcement == false
  and .ready_for_live_execution == false
' >/dev/null <<<"$report"

jq -e '
  .source_probes.canonical_adapter_inventory_application.rust_module_present == true
  and .source_probes.canonical_adapter_inventory_application.report_script_present == true
  and .source_probes.canonical_adapter_inventory_application.gate_script_present == true
  and .source_probes.canonical_adapter_inventory_readback.rust_module_present == true
  and .source_probes.canonical_adapter_inventory_readback.gate_script_present == true
  and .source_probes.canonical_adapter_inventory_readback.upstream_gate == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_canonical_adapter_inventory_application --lib

echo "Hepta WorkGraph canonical adapter inventory application preview gate passed"
