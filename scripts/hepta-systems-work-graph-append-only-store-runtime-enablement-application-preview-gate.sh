#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-append-only-store-runtime-enablement-application-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-append-only-store-runtime-enablement-application-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_append_only_store_runtime_enablement_application_preview_gate"
  and .schema_version == "work_graph_append_only_store_runtime_enablement_application_preview_v1"
  and .preview_mode == "read_only_append_only_store_runtime_enablement_application_preview_no_runtime_mutation"
  and .readback_plan_count == 12
  and .application_plan_count == 12
  and .source_outcome_count == 12
  and .runtime_enablement_contract_ready_preview_count == 12
  and .stage_application_count == 6
  and .blocker_application_count == 13
  and .application_group_count == 4
  and .runtime_plan_stage_ref_count == 72
  and .evidence_field_ref_count == 96
  and .stage_contract_ref_count == 29
  and .stage_source_ref_count == 62
  and .blocker_mapping_source_ref_count == 113
  and .application_guard_count == 11
  and .blocker_count == 15
  and .required_prior_gate_count == 42
' >/dev/null <<<"$report"

jq -e '
  (.application_plans | all(
    (.expected_runtime_stage_ids | length) == 6
    and (.expected_evidence_field_ids | length) == 8
    and .application_scope == "append_only_store_runtime_enablement_application_binding"
    and .application_state == "preview_application_defined_runtime_enablement_not_applied"
    and .readback_verified_by_preview == true
    and .applies_to_runtime == false
    and .enables_append_only_store == false
    and .writes_wal == false
    and .writes_checkpoint == false
    and .mutates_idempotency_index == false
    and .executes_readback == false
    and .executes_rollback == false
    and .records_approval == false
    and .promotes_runtime_application == false
    and .mutates_store == false
  ))
  and (.source_outcomes | all(
    .post_application_runtime_enablement_state == "runtime_enablement_contract_ready_preview_after_application"
    and .runtime_enablement_contract_ready_preview == true
    and .ready_for_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview == true
    and .ready_for_append_only_store_enablement == false
    and .applies_to_runtime == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.stage_applications | map({id: .runtime_stage_id, sources: (.affected_source_surface_ids | length), contracts: (.expected_contract_ref_ids | length)}) == [
    {"id": "durable_store_runtime_switch", "sources": 12, "contracts": 5},
    {"id": "wal_write_boundary", "sources": 12, "contracts": 6},
    {"id": "idempotency_mutation_policy", "sources": 12, "contracts": 5},
    {"id": "rollback_readback_execution_gate", "sources": 12, "contracts": 5},
    {"id": "operator_review_side_effect_lock", "sources": 7, "contracts": 3},
    {"id": "runtime_application_promotion", "sources": 7, "contracts": 5}
  ])
  and (.stage_applications | all(
    .expected_stage_state == "stage_contract_ready_preview_after_application_runtime_disabled"
    and .stage_contract_ready_preview == true
    and .readback_verified_by_preview == true
    and .applies_to_runtime == false
    and .enables_append_only_store == false
    and .writes_wal == false
    and .mutates_idempotency_index == false
    and .executes_readback == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.blocker_applications | map({id: .blocker_id, count: (.affected_source_surface_ids | length)}) == [
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
  and (.blocker_applications | all(
    .expected_blocker_state == "blocker_mapping_contract_ready_preview_after_application_runtime_still_blocked"
    and .blocker_contract_ready_preview == true
    and .readback_verified_by_preview == true
    and .clears_runtime_blocker == false
    and .mutates_runtime == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.application_groups | map({id, count: (.stage_application_ids | length)}) == [
    {"id": "append_only_store_runtime_core_application", "count": 2},
    {"id": "append_only_store_runtime_replay_safety_application", "count": 2},
    {"id": "append_only_store_runtime_operator_lock_application", "count": 1},
    {"id": "append_only_store_runtime_application_promotion_preview", "count": 1}
  ])
  and (.application_groups | all(
    .priority == "p0"
    and .mutates_runtime == false
    and .enables_append_only_store == false
    and .writes_wal == false
  ))
  and (.application_guards | map(.id) == [
    "runtime_enablement_application_is_preview_only",
    "durable_store_runtime_switch_disabled",
    "wal_write_boundary_disabled",
    "idempotency_index_mutation_disabled",
    "rollback_readback_execution_disabled",
    "operator_review_required",
    "runtime_application_promotion_disabled",
    "scheduler_role_runtime_application_disabled",
    "append_only_store_runtime_readiness_rerun_required",
    "side_effect_lock_not_established",
    "graph_state_persistence_disabled"
  ])
  and (.application_guards | all(.required_before_append_only_store_runtime_enablement == true and .satisfied_by_preview == false))
' >/dev/null <<<"$report"

jq -e '
  (.blockers | map({id, count: (.affected_source_surface_ids | length), plans: (.affected_application_plan_ids | length)}) == [
    {"id": "readback_execution_disabled", "count": 12, "plans": 12},
    {"id": "durable_store_runtime_switch_disabled", "count": 12, "plans": 12},
    {"id": "append_only_store_runtime_enablement_disabled", "count": 12, "plans": 12},
    {"id": "wal_write_boundary_not_enabled", "count": 12, "plans": 12},
    {"id": "idempotency_index_mutation_disabled", "count": 12, "plans": 12},
    {"id": "rollback_readback_not_executed", "count": 12, "plans": 12},
    {"id": "operator_review_required", "count": 7, "plans": 7},
    {"id": "projection_adapter_runtime_closure_application_disabled", "count": 7, "plans": 7},
    {"id": "store_guard_runtime_application_disabled", "count": 5, "plans": 5},
    {"id": "terminal_task_result_runtime_application_disabled", "count": 6, "plans": 6},
    {"id": "scheduler_admission_runtime_application_disabled", "count": 5, "plans": 5},
    {"id": "role_manifest_runtime_application_disabled", "count": 4, "plans": 4},
    {"id": "runtime_application_residuals_not_promoted", "count": 7, "plans": 7},
    {"id": "append_only_store_runtime_enablement_readback_missing", "count": 12, "plans": 12},
    {"id": "append_only_store_runtime_readiness_rerun_missing", "count": 12, "plans": 12}
  ])
  and (.blockers | all(.required_before_append_only_store_runtime_enablement == true))
' >/dev/null <<<"$report"

jq -e '
  (.required_prior_gates | length == (unique | length))
  and .required_prior_gate_count == 42
  and (.required_prior_gates[-1] == "hepta_work_graph_append_only_store_runtime_enablement_readback_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview_gate"
  and .ready_for_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview == true
  and .ready_for_append_only_store_enablement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_scheduler_admission_enforcement == false
  and .ready_for_role_manifest_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.append_only_store_runtime_enablement_application.rust_module_present == true
  and .source_probes.append_only_store_runtime_enablement_application.report_script_present == true
  and .source_probes.append_only_store_runtime_enablement_application.gate_script_present == true
  and .source_probes.append_only_store_runtime_enablement_readback.upstream_gate == true
  and .source_probes.append_only_store_runtime_enablement_readback.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_append_only_store_runtime_enablement_application --lib

echo "Hepta WorkGraph append-only store runtime enablement application preview gate passed"
