#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-append-only-store-runtime-enablement-readback-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-append-only-store-runtime-enablement-readback-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_append_only_store_runtime_enablement_readback_preview_gate"
  and .schema_version == "work_graph_append_only_store_runtime_enablement_readback_preview_v1"
  and .preview_mode == "read_only_append_only_store_runtime_enablement_readback_preview_no_execution"
  and .runtime_enablement_plan_count == 12
  and .readback_plan_count == 12
  and .source_plan_assertion_count == 12
  and .stage_plan_assertion_count == 6
  and .evidence_field_assertion_count == 12
  and .guard_assertion_count == 10
  and .blocker_mapping_assertion_count == 13
  and .readback_evidence_field_ref_count == 96
  and .stage_contract_ref_count == 29
  and .stage_source_ref_count == 62
  and .blocker_mapping_source_ref_count == 113
  and .drift_detector_count == 6
  and .blocker_count == 14
  and .required_prior_gate_count == 41
' >/dev/null <<<"$report"

jq -e '
  (.readback_plans | all(
    (.expected_runtime_stage_ids | length) == 6
    and (.expected_evidence_field_ids | length) == 8
    and .readback_scope == "append_only_store_runtime_enablement_contract_refs"
    and .expected_preview_state == "runtime_enablement_contract_ready_readback_not_executed"
    and .required_before_runtime_enablement_application == true
    and .performs_readback == false
    and .mutates_store == false
    and .enables_append_only_store == false
    and .writes_wal == false
    and .mutates_idempotency_index == false
    and .executes_rollback == false
    and .records_approval == false
  ))
  and (.source_plan_assertions | all(
    .expected_runtime_stage_count == 6
    and .expected_plan_state == "runtime_enablement_plan_defined_runtime_disabled"
    and .performs_readback == false
    and .mutates_store == false
  ))
  and (.evidence_field_assertions | all(
    .expected_evidence_field_count == 8
    and .expected_evidence_state == "evidence_fields_declared_readback_not_executed"
    and .performs_readback == false
    and .mutates_store == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.stage_plan_assertions | map({id: .runtime_stage_id, sources: .expected_source_surface_count, contracts: .expected_contract_ref_count}) == [
    {"id": "durable_store_runtime_switch", "sources": 12, "contracts": 5},
    {"id": "wal_write_boundary", "sources": 12, "contracts": 6},
    {"id": "idempotency_mutation_policy", "sources": 12, "contracts": 5},
    {"id": "rollback_readback_execution_gate", "sources": 12, "contracts": 5},
    {"id": "operator_review_side_effect_lock", "sources": 7, "contracts": 3},
    {"id": "runtime_application_promotion", "sources": 7, "contracts": 5}
  ])
  and (.stage_plan_assertions | all(
    .expected_runtime_state == "contract_ready_preview_runtime_disabled_readback_not_executed"
    and .performs_readback == false
    and .mutates_store == false
    and .enables_append_only_store == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.guard_assertions | map(.guard_id) == [
    "runtime_enablement_preview_only",
    "durable_store_switch_not_enabled",
    "wal_write_boundary_not_enabled",
    "idempotency_index_mutation_disabled",
    "rollback_readback_execution_disabled",
    "operator_review_required",
    "runtime_application_promotion_disabled",
    "scheduler_role_runtime_application_disabled",
    "append_only_store_readback_required",
    "side_effect_lock_not_established"
  ])
  and (.guard_assertions | all(
    .expected_guard_state == "guard_required_not_satisfied_by_preview"
    and .required_before_runtime_enablement == true
    and .satisfied_by_preview == false
    and .performs_readback == false
    and .mutates_store == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.blocker_mapping_assertions | map({id: .blocker_id, count: (.affected_source_surface_ids | length)}) == [
    {"id": "durable_store_runtime_switch_disabled", "count": 12},
    {"id": "append_only_store_runtime_enablement_disabled", "count": 12},
    {"id": "wal_write_boundary_not_enabled", "count": 12},
    {"id": "idempotency_index_mutation_disabled", "count": 12},
    {"id": "rollback_readback_not_executed", "count": 12},
    {"id": "operator_review_required", "count": 7},
    {"id": "projection_adapter_runtime_closure_application_disabled", "count": 7},
    {"id": "store_guard_runtime_application_disabled", "count": 5},
    {"id": "terminal_task_result_runtime_application_disabled", "count": 6},
    {"id": "scheduler_admission_runtime_application_disabled", "count": 5},
    {"id": "role_manifest_runtime_application_disabled", "count": 4},
    {"id": "runtime_application_residuals_not_promoted", "count": 7},
    {"id": "append_only_store_runtime_enablement_readback_missing", "count": 12}
  ])
  and (.blocker_mapping_assertions | all(
    .expected_blocker_state == "blocks_runtime_enablement_until_readback_and_application_preview"
    and .required_before_runtime_enablement == true
    and .performs_readback == false
    and .mutates_store == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.blockers | map({id, count: (.affected_source_surface_ids | length)})[0] == {"id": "readback_execution_disabled", "count": 12})
  and (.blockers | all(.required_before_runtime_enablement_application == true))
  and (.drift_detectors | map(.id) == [
    "append_only_runtime_source_plan_drift",
    "append_only_runtime_stage_contract_drift",
    "append_only_runtime_evidence_field_drift",
    "append_only_runtime_blocker_mapping_drift",
    "append_only_runtime_side_effect_boundary_drift",
    "append_only_runtime_prior_gate_drift"
  ])
  and (.drift_detectors | all(.blocks_runtime_enablement_application == true and .performs_readback == false))
  and (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_append_only_store_runtime_enablement_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_append_only_store_runtime_enablement_application_preview_gate"
  and .ready_for_runtime_enablement_application_preview == true
  and .ready_for_append_only_store_enablement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_scheduler_admission_enforcement == false
  and .ready_for_role_manifest_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.append_only_store_runtime_enablement_readback.rust_module_present == true
  and .source_probes.append_only_store_runtime_enablement_readback.report_script_present == true
  and .source_probes.append_only_store_runtime_enablement_readback.gate_script_present == true
  and .source_probes.append_only_store_runtime_enablement_preview.upstream_gate == true
  and .source_probes.append_only_store_runtime_enablement_preview.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_append_only_store_runtime_enablement_readback --lib

echo "Hepta WorkGraph append-only store runtime enablement readback preview gate passed"
