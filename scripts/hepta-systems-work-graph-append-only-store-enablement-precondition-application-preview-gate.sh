#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-append-only-store-enablement-precondition-application-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-append-only-store-enablement-precondition-application-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_append_only_store_enablement_precondition_application_preview_gate"
  and .schema_version == "work_graph_append_only_store_enablement_precondition_application_preview_v1"
  and .preview_mode == "read_only_append_only_store_enablement_precondition_application_preview_no_runtime_mutation"
  and .readback_plan_count == 7
  and .application_plan_count == 7
  and .precondition_outcome_count == 7
  and .precondition_contract_ready_preview_count == 7
  and .blocker_application_count == 8
  and .application_group_count == 4
  and .contract_ref_count == 49
  and .source_ref_count == 63
  and .evidence_field_ref_count == 36
  and .blocker_mapping_source_ref_count == 70
  and .application_guard_count == 10
  and .blocker_count == 10
' >/dev/null <<<"$report"

jq -e '
  (.application_plans | map({id: .readback_precondition_id, refs: (.expected_contract_ref_ids | length), sources: (.affected_source_surface_ids | length), evidence: (.required_evidence_fields | length)}) == [
    {"id": "durable_store_enablement_switch", "refs": 9, "sources": 12, "evidence": 5},
    {"id": "wal_append_boundary_contract", "refs": 6, "sources": 12, "evidence": 5},
    {"id": "idempotency_mutation_policy", "refs": 12, "sources": 12, "evidence": 5},
    {"id": "rollback_readback_gate", "refs": 10, "sources": 12, "evidence": 5},
    {"id": "operator_review_and_side_effect_lock", "refs": 3, "sources": 6, "evidence": 5},
    {"id": "scheduler_admission_enforcement_precondition", "refs": 5, "sources": 5, "evidence": 6},
    {"id": "role_manifest_enforcement_precondition", "refs": 4, "sources": 4, "evidence": 5}
  ])
  and (.application_plans | all(
    .application_scope == "append_only_store_enablement_precondition_runtime_binding"
    and .application_state == "preview_application_defined_precondition_not_applied_to_runtime"
    and .readback_verified_by_preview == true
    and .applies_to_runtime == false
    and .persists_precondition_state == false
    and .enables_append_only_store == false
    and .mutates_store == false
    and .writes_wal == false
    and .writes_checkpoint == false
    and .mutates_idempotency_index == false
    and .enforces_scheduler_admission == false
    and .enforces_role_manifest == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.precondition_outcomes | all(
    .post_application_precondition_state == "precondition_contract_ready_preview_after_application"
    and .precondition_contract_ready_preview == true
    and .ready_for_unified_projection_enforcement_readiness_append_only_store_rerun_preview == true
    and .ready_for_append_only_store_enablement == false
    and .applies_to_runtime == false
  ))
  and (.blocker_applications | length) == 8
  and (.blocker_applications | all(
    .expected_blocker_state == "blocker_mapping_contract_ready_preview_after_application_runtime_still_blocked"
    and .blocker_contract_ready_preview == true
    and .readback_verified_by_preview == true
    and .clears_runtime_blocker == false
    and .mutates_store == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.application_groups | map({id, count: (.application_plan_ids | length)}) == [
    {"id": "append_only_store_core_precondition_application", "count": 2},
    {"id": "append_only_replay_safety_precondition_application", "count": 2},
    {"id": "append_only_operator_lock_precondition_application", "count": 1},
    {"id": "append_only_scheduler_role_precondition_application", "count": 2}
  ])
  and (.application_groups | all(
    .priority == "p0"
    and .mutates_runtime == false
    and .enables_append_only_store == false
    and .writes_wal == false
  ))
  and (.application_guards | map(.id) == [
    "precondition_application_is_preview_only",
    "durable_store_runtime_switch_disabled",
    "wal_write_boundary_disabled",
    "idempotency_index_mutation_disabled",
    "rollback_readback_execution_disabled",
    "operator_review_required",
    "scheduler_admission_not_enforced",
    "role_manifest_not_enforced",
    "runtime_application_residuals_not_promoted",
    "append_only_store_readiness_rerun_required"
  ])
  and (.application_guards | all(.required_before_append_only_store_enablement == true and .satisfied_by_preview == false))
' >/dev/null <<<"$report"

jq -e '
  (.blockers | map({id, count: (.affected_source_surface_ids | length), plans: (.affected_application_plan_ids | length)}) == [
    {"id": "readback_execution_disabled", "count": 12, "plans": 7},
    {"id": "durable_store_enablement_disabled", "count": 12, "plans": 1},
    {"id": "wal_write_boundary_not_enabled", "count": 12, "plans": 1},
    {"id": "idempotency_index_mutation_disabled", "count": 12, "plans": 1},
    {"id": "rollback_readback_not_executed", "count": 12, "plans": 1},
    {"id": "operator_review_required", "count": 6, "plans": 1},
    {"id": "scheduler_admission_not_enforced", "count": 5, "plans": 1},
    {"id": "role_manifest_not_enforced", "count": 4, "plans": 1},
    {"id": "runtime_application_residuals_not_promoted", "count": 7, "plans": 7},
    {"id": "append_only_store_readiness_rerun_missing", "count": 12, "plans": 7}
  ])
  and (.blockers | all(.required_before_append_only_store_enablement == true))
' >/dev/null <<<"$report"

jq -e '
  .required_prior_gate_count == 30
  and (.required_prior_gates[-1] == "hepta_work_graph_append_only_store_enablement_precondition_readback_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview_gate"
  and .ready_for_unified_projection_enforcement_readiness_append_only_store_rerun_preview == true
  and .ready_for_append_only_store_enablement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_scheduler_admission_enforcement == false
  and .ready_for_role_manifest_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.append_only_store_enablement_precondition_application.rust_module_present == true
  and .source_probes.append_only_store_enablement_precondition_application.report_script_present == true
  and .source_probes.append_only_store_enablement_precondition_application.gate_script_present == true
  and .source_probes.append_only_store_enablement_precondition_readback.upstream_gate == true
  and .source_probes.append_only_store_enablement_precondition_readback.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_append_only_store_enablement_precondition_application --lib

echo "Hepta WorkGraph append-only store enablement precondition application preview gate passed"
