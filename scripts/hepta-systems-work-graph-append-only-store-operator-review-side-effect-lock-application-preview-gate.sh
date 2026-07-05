#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-append-only-store-operator-review-side-effect-lock-application-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-append-only-store-operator-review-side-effect-lock-application-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_append_only_store_operator_review_side_effect_lock_application_preview_gate"
  and .schema_version == "work_graph_append_only_store_operator_review_side_effect_lock_application_preview_v1"
  and .preview_mode == "read_only_append_only_store_operator_review_side_effect_lock_application_no_runtime_mutation"
  and .readback_plan_count == 7
  and .application_plan_count == 7
  and .source_outcome_count == 7
  and .operator_review_contract_ready_preview_count == 7
  and .side_effect_lock_contract_ready_preview_count == 7
  and .operator_review_packet_application_count == 7
  and .side_effect_lock_application_count == 7
  and .approval_boundary_application_count == 7
  and .readback_boundary_application_count == 7
  and .group_application_count == 4
  and .blocker_application_count == 9
  and .application_guard_count == 12
  and .blocker_count == 10
  and .required_prior_gate_count == 50
  and .evidence_field_ref_count == 56
  and .lock_scope_ref_count == 35
  and .group_source_ref_count == 7
  and .blocker_mapping_source_ref_count == 88
' >/dev/null <<<"$report"

jq -e '
  (.application_plans | map(.source_surface_id) == [
    "multi_agent_v2_thread_spawn",
    "hepta_runtime_multi_agent_reducer",
    "agent_jobs_batch_workers",
    "hepta_runtime_task_board",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_scheduler_store",
    "hepta_runtime_agent_harness"
  ])
  and (.application_plans | all(
    .application_scope == "operator_review_side_effect_lock_application_binding"
    and .application_state == "preview_application_defined_operator_review_and_lock_not_recorded"
    and .readback_verified_by_preview == true
    and .operator_review_contract_ready_preview == true
    and .side_effect_lock_contract_ready_preview == true
    and .records_operator_review == false
    and .records_approval == false
    and .establishes_side_effect_lock == false
    and .executes_readback == false
    and .writes_store == false
    and .writes_wal == false
    and .mutates_runtime == false
    and (.expected_evidence_field_ids | length) == 8
    and (.lock_scope_ids | length) == 5
  ))
  and (.source_outcomes | all(
    .post_application_operator_review_state == "operator_review_side_effect_lock_contract_ready_preview_after_application"
    and .operator_review_contract_ready_preview == true
    and .side_effect_lock_contract_ready_preview == true
    and .ready_for_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview == true
    and .ready_for_operator_review_recording == false
    and .applies_to_runtime == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.packet_applications | all(
    .expected_packet_state == "packet_contract_ready_preview_after_application_not_recorded"
    and .packet_contract_ready_preview == true
    and .readback_verified_by_preview == true
    and .records_operator_review == false
    and .records_approval == false
    and (.required_section_ids | length) == 5
    and (.required_evidence_field_ids | length) == 8
  ))
  and (.side_effect_lock_applications | all(
    .expected_lock_state == "side_effect_lock_contract_ready_preview_after_application_not_established"
    and .lock_contract_ready_preview == true
    and .readback_verified_by_preview == true
    and .establishes_side_effect_lock == false
    and .mutates_runtime == false
    and (.lock_scope_ids | length) == 5
  ))
  and (.approval_boundary_applications | all(
    .expected_boundary_state == "approval_evidence_contract_ready_preview_after_application_not_recorded"
    and .boundary_contract_ready_preview == true
    and .readback_verified_by_preview == true
    and .records_operator_review == false
    and .records_approval == false
    and .persists_receipt == false
    and (.required_evidence_field_ids | length) == 8
  ))
  and (.readback_boundary_applications | all(
    .expected_boundary_state == "readback_boundary_contract_ready_preview_after_application_not_executed"
    and .boundary_contract_ready_preview == true
    and .readback_verified_by_preview == true
    and .executes_readback == false
    and .rollback_executed == false
    and .writes_checkpoint == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.group_applications | map({category: .source_category, count: (.affected_source_surface_ids | length), contracts: .expected_contract_count_after_application}) == [
    {"category": "multi_agent", "count": 2, "contracts": 2},
    {"category": "batch_agent_jobs", "count": 1, "contracts": 1},
    {"category": "runtime_scheduler", "count": 3, "contracts": 3},
    {"category": "external_handoff", "count": 1, "contracts": 1}
  ])
  and (.group_applications | all(
    .group_contract_ready_preview == true
    and .readback_verified_by_preview == true
    and .records_operator_review == false
    and .establishes_side_effect_lock == false
  ))
  and (.application_guards | map(.id) == [
    "operator_review_side_effect_lock_application_is_preview_only",
    "readback_execution_disabled",
    "operator_review_recording_disabled",
    "approval_recording_disabled",
    "side_effect_lock_establishment_disabled",
    "wal_write_boundary_disabled",
    "durable_store_runtime_switch_disabled",
    "idempotency_mutation_disabled",
    "rollback_readback_execution_disabled",
    "append_only_store_enablement_disabled",
    "runtime_mutation_disabled",
    "model_invocation_disabled"
  ])
  and (.application_guards | all(.required_before_operator_review_side_effect_lock == true and .satisfied_by_preview == false))
' >/dev/null <<<"$report"

jq -e '
  (.blocker_applications | map({id: .blocker_id, count: (.affected_source_surface_ids | length), plans: (.affected_application_plan_ids | length)}) == [
    {"id": "readback_execution_disabled", "count": 12, "plans": 7},
    {"id": "durable_store_runtime_switch_disabled", "count": 12, "plans": 7},
    {"id": "wal_write_boundary_not_enabled", "count": 12, "plans": 7},
    {"id": "idempotency_index_mutation_disabled", "count": 12, "plans": 7},
    {"id": "rollback_readback_not_executed", "count": 12, "plans": 7},
    {"id": "operator_review_required", "count": 7, "plans": 7},
    {"id": "side_effect_lock_not_established", "count": 7, "plans": 7},
    {"id": "operator_review_side_effect_lock_readback_missing", "count": 7, "plans": 7},
    {"id": "operator_review_side_effect_lock_application_missing", "count": 7, "plans": 7}
  ])
  and (.blocker_applications | all(
    .expected_blocker_state == "blocker_mapping_contract_ready_preview_after_application_runtime_still_blocked"
    and .blocker_contract_ready_preview == true
    and .readback_verified_by_preview == true
    and .clears_operator_review_blocker == false
    and .clears_side_effect_lock_blocker == false
    and .mutates_runtime == false
  ))
  and (.blockers | map({id, count: (.affected_source_surface_ids | length), plans: (.affected_application_plan_ids | length)}) == [
    {"id": "readback_execution_disabled", "count": 12, "plans": 7},
    {"id": "durable_store_runtime_switch_disabled", "count": 12, "plans": 7},
    {"id": "wal_write_boundary_not_enabled", "count": 12, "plans": 7},
    {"id": "idempotency_index_mutation_disabled", "count": 12, "plans": 7},
    {"id": "rollback_readback_not_executed", "count": 12, "plans": 7},
    {"id": "operator_review_required", "count": 7, "plans": 7},
    {"id": "side_effect_lock_not_established", "count": 7, "plans": 7},
    {"id": "operator_review_side_effect_lock_readback_missing", "count": 7, "plans": 7},
    {"id": "operator_review_side_effect_lock_application_missing", "count": 7, "plans": 7},
    {"id": "operator_review_side_effect_lock_readiness_rerun_missing", "count": 7, "plans": 7}
  ])
  and (.blockers | all(.required_before_operator_review_side_effect_lock == true))
' >/dev/null <<<"$report"

jq -e '
  (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_append_only_store_operator_review_side_effect_lock_readback_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview_gate"
  and .ready_for_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview == true
  and .ready_for_operator_review_recording == false
  and .ready_for_side_effect_lock_establishment == false
  and .ready_for_runtime_write_boundary_preview == false
  and .ready_for_append_only_store_enablement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.operator_review_side_effect_lock_application.rust_module_present == true
  and .source_probes.operator_review_side_effect_lock_application.report_script_present == true
  and .source_probes.operator_review_side_effect_lock_application.gate_script_present == true
  and .source_probes.operator_review_side_effect_lock_readback.upstream_gate == true
  and .source_probes.operator_review_side_effect_lock_readback.gate_script_present == true
  and .source_probes.operator_review_side_effect_lock_readback.recommended_next_matches == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_append_only_store_operator_review_side_effect_lock_application_preview --lib

echo "Hepta WorkGraph append-only store operator review side-effect lock application preview gate passed"
