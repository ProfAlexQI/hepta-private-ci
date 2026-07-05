#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-runtime-application-promotion-gap-closure-application-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-runtime-application-promotion-gap-closure-application-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_runtime_application_promotion_gap_closure_application_preview_gate"
  and .schema_version == "work_graph_runtime_application_promotion_gap_closure_application_preview_v1"
  and .preview_mode == "read_only_runtime_application_promotion_gap_closure_application_no_runtime_mutation"
  and .readback_plan_count == 12
  and .application_plan_count == 12
  and .source_outcome_count == 12
  and .runtime_application_contract_ready_preview_count == 12
  and .promotion_binding_application_count == 27
  and .promotion_group_application_count == 5
  and .blocker_application_count == 14
  and .application_guard_count == 12
  and .blocker_count == 15
  and .required_prior_gate_count == 46
  and .promotion_domain_ref_count == 27
  and .promotion_binding_ref_count == 27
  and .evidence_field_ref_count == 96
  and .group_source_ref_count == 27
  and .blocker_mapping_source_ref_count == 125
' >/dev/null <<<"$report"

jq -e '
  (.application_plans | map(.source_surface_id) == [
    "update_plan_tool",
    "plan_mode_proposed_plan_blocks",
    "app_server_turn_plan_notification",
    "multi_agent_v2_thread_spawn",
    "multi_agent_v2_mailbox_wait",
    "hepta_runtime_multi_agent_reducer",
    "agent_jobs_batch_workers",
    "hepta_runtime_task_board",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_scheduler_store",
    "hepta_runtime_approval_broker",
    "hepta_runtime_agent_harness"
  ])
  and (.application_plans | all(
    .application_scope == "runtime_application_promotion_gap_closure_application_binding"
    and .application_state == "preview_application_defined_runtime_application_not_promoted"
    and .readback_verified_by_preview == true
    and .applies_to_runtime == false
    and .promotes_runtime_application == false
    and .attaches_runtime_wrapper == false
    and .enforces_scheduler_admission == false
    and .enforces_role_manifest == false
    and .enables_task_result_enforcement == false
    and .writes_store == false
    and .writes_wal == false
    and .records_approval == false
    and .executes_readback == false
    and .mutates_runtime == false
    and (.expected_evidence_field_ids | length) == 8
  ))
  and (.source_outcomes | all(
    .post_application_runtime_promotion_state == "runtime_application_promotion_contract_ready_preview_after_application"
    and .runtime_application_contract_ready_preview == true
    and .ready_for_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview == true
    and .ready_for_runtime_application_promotion == false
    and .applies_to_runtime == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.promotion_binding_applications | map(select(.promotion_domain_id == "projection_adapter_runtime_closure")) | length) == 7
  and (.promotion_binding_applications | map(select(.promotion_domain_id == "store_guard_runtime_application")) | length) == 5
  and (.promotion_binding_applications | map(select(.promotion_domain_id == "terminal_task_result_runtime_wrapper")) | length) == 6
  and (.promotion_binding_applications | map(select(.promotion_domain_id == "scheduler_admission_runtime_application")) | length) == 5
  and (.promotion_binding_applications | map(select(.promotion_domain_id == "role_manifest_runtime_application")) | length) == 4
  and (.promotion_binding_applications | all(
    .expected_binding_state == "binding_contract_ready_preview_after_application_runtime_still_blocked"
    and .binding_contract_ready_preview == true
    and .readback_verified_by_preview == true
    and .promotes_runtime_application == false
    and .writes_store == false
    and (.required_evidence_field_ids | length) == 8
  ))
  and (.promotion_group_applications | map({id: .promotion_domain_id, count: (.affected_source_surface_ids | length), contracts: .expected_contract_count_after_application}) == [
    {"id": "projection_adapter_runtime_closure", "count": 7, "contracts": 7},
    {"id": "store_guard_runtime_application", "count": 5, "contracts": 5},
    {"id": "terminal_task_result_runtime_wrapper", "count": 6, "contracts": 6},
    {"id": "scheduler_admission_runtime_application", "count": 5, "contracts": 5},
    {"id": "role_manifest_runtime_application", "count": 4, "contracts": 4}
  ])
  and (.promotion_group_applications | all(.group_contract_ready_preview == true and .readback_verified_by_preview == true and .promotes_runtime_application == false and .mutates_runtime == false))
' >/dev/null <<<"$report"

jq -e '
  (.blocker_applications | map({id: .blocker_id, count: (.affected_source_surface_ids | length), plans: (.affected_application_plan_ids | length)}) == [
    {"id": "readback_execution_disabled", "count": 12, "plans": 12},
    {"id": "durable_store_runtime_switch_disabled", "count": 12, "plans": 12},
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
    {"id": "runtime_application_promotion_readback_missing", "count": 12, "plans": 12},
    {"id": "runtime_application_promotion_closure_application_missing", "count": 12, "plans": 12}
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
  (.application_guards | map(.id) == [
    "runtime_application_promotion_application_is_preview_only",
    "readback_execution_disabled",
    "runtime_application_promotion_disabled",
    "runtime_wrapper_attachment_disabled",
    "task_result_enforcement_disabled",
    "scheduler_admission_runtime_enforcement_disabled",
    "role_manifest_runtime_enforcement_disabled",
    "operator_review_required",
    "side_effect_lock_not_established",
    "wal_write_boundary_disabled",
    "durable_store_runtime_switch_disabled",
    "append_only_store_enablement_disabled"
  ])
  and (.application_guards | all(.required_before_runtime_application_promotion == true and .satisfied_by_preview == false))
  and (.blockers | map({id, count: (.affected_source_surface_ids | length), plans: (.affected_application_plan_ids | length)}) == [
    {"id": "readback_execution_disabled", "count": 12, "plans": 12},
    {"id": "durable_store_runtime_switch_disabled", "count": 12, "plans": 12},
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
    {"id": "runtime_application_promotion_readback_missing", "count": 12, "plans": 12},
    {"id": "runtime_application_promotion_closure_application_missing", "count": 12, "plans": 12},
    {"id": "runtime_application_promotion_readiness_rerun_missing", "count": 12, "plans": 12}
  ])
  and (.blockers | all(.required_before_runtime_application_promotion == true))
' >/dev/null <<<"$report"

jq -e '
  (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_runtime_application_promotion_gap_closure_readback_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview_gate"
  and .ready_for_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview == true
  and .ready_for_runtime_application_promotion == false
  and .ready_for_operator_review_side_effect_lock == false
  and .ready_for_append_only_store_enablement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.runtime_application_promotion_gap_closure_application.rust_module_present == true
  and .source_probes.runtime_application_promotion_gap_closure_application.report_script_present == true
  and .source_probes.runtime_application_promotion_gap_closure_application.gate_script_present == true
  and .source_probes.runtime_application_promotion_gap_closure_readback.upstream_gate == true
  and .source_probes.runtime_application_promotion_gap_closure_readback.gate_script_present == true
  and .source_probes.runtime_application_promotion_gap_closure_readback.recommended_next_matches == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_runtime_application_promotion_gap_closure_application_preview --lib -- --test-threads=1

echo "Hepta WorkGraph runtime application promotion gap closure application preview gate passed"
