#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-application-promotion-rerun-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-unified-projection-enforcement-readiness-runtime-application-promotion-rerun-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "blocked"
  and .gate == "hepta_work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview_gate"
  and .schema_version == "work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview_v1"
  and .preview_mode == "read_only_projection_enforcement_readiness_runtime_application_promotion_rerun_no_enforcement"
  and .source_surface_count == 12
  and .runtime_application_promotion_outcome_count == 12
  and .previous_contract_ready_surface_count == 12
  and .runtime_application_rerun_contract_ready_surface_count == 12
  and .previous_runtime_application_primary_blocked_surface_count == 7
  and .runtime_application_primary_blocked_surface_count_after == 0
  and .runtime_application_contract_ready_surface_count == 12
  and .runtime_application_promoted_surface_count == 0
  and .operator_review_residual_source_count == 7
  and .side_effect_lock_residual_source_count == 7
  and .wal_boundary_residual_source_count == 12
  and .write_boundary_primary_blocked_surface_count == 5
  and .rerun_ready_surface_count == 0
  and .rerun_blocked_surface_count == 12
  and .decision_delta_count == 12
  and (.decision_deltas | length) == .decision_delta_count
  and (.decision_deltas | all(.projection_contract_ready == true and .runtime_application_promotion_contract_ready == true and .runtime_application_promoted == false))
' >/dev/null <<<"$report"

jq -e '
  (.decision_deltas | map(select(.covered_by_runtime_application_promotion_application_preview == true)) | length) == 12
  and (.decision_deltas | map(select(.runtime_application_primary_gap_closed_by_application_preview == true)) | length) == 7
  and (.decision_deltas | map(select(.runtime_application_promotion_rerun_enforcement_decision == "deny_runtime_application_residuals_not_promoted")) | length) == 0
  and (.decision_deltas | map(select(.runtime_application_promotion_rerun_enforcement_decision == "deny_operator_review_required")) | length) == 7
  and (.decision_deltas | map(select(.runtime_application_promotion_rerun_enforcement_decision == "deny_runtime_append_only_store_write_boundary_disabled")) | length) == 5
  and (.decision_deltas | map(select(.source_surface_id == "update_plan_tool" and .previous_enforcement_decision == "deny_runtime_application_residuals_not_promoted" and .runtime_application_promotion_rerun_enforcement_decision == "deny_runtime_append_only_store_write_boundary_disabled")) | length) == 1
  and (.decision_deltas | map(select(.source_surface_id == "multi_agent_v2_thread_spawn" and .previous_enforcement_decision == "deny_operator_review_required" and .runtime_application_promotion_rerun_enforcement_decision == "deny_operator_review_required")) | length) == 1
  and (.decision_deltas | all((.residual_source_blocker_ids | index("runtime_application_residuals_not_promoted")) | not))
  and (.decision_deltas | all((.residual_source_blocker_ids | index("append_only_store_runtime_enablement_disabled")) | not))
  and (.decision_deltas | all(.scheduler_admission_enforcement_ready == false and .role_manifest_enforcement_ready == false and .runtime_append_only_store_enabled == false))
' >/dev/null <<<"$report"

jq -e '
  .cleared_blocker_count == 1
  and (.cleared_blockers[0].id == "runtime_application_residuals_not_promoted_for_enforcement")
  and (.cleared_blockers[0].source_count_before == 7)
  and (.cleared_blockers[0].source_count_after == 0)
  and (.cleared_blockers[0].closure_gate_id == "hepta_work_graph_runtime_application_promotion_gap_closure_application_preview_gate")
  and (.cleared_blockers[0].cleared_source_surface_ids == [
    "update_plan_tool",
    "plan_mode_proposed_plan_blocks",
    "app_server_turn_plan_notification",
    "multi_agent_v2_mailbox_wait",
    "hepta_runtime_multi_agent_reducer",
    "hepta_runtime_task_board",
    "hepta_runtime_approval_broker"
  ])
  and .residual_blocker_count == 7
  and (.residual_blockers | map({id, count: (.affected_source_surface_ids | length)}) == [
    {"id": "readback_execution_disabled", "count": 12},
    {"id": "durable_store_runtime_switch_disabled", "count": 12},
    {"id": "wal_write_boundary_not_enabled", "count": 12},
    {"id": "idempotency_index_mutation_disabled", "count": 12},
    {"id": "rollback_readback_not_executed", "count": 12},
    {"id": "operator_review_required", "count": 7},
    {"id": "side_effect_lock_not_established", "count": 7}
  ])
  and (.residual_blockers | all(.required_before_projection_enforcement == true))
' >/dev/null <<<"$report"

jq -e '
  .enforcement_stage_count == 7
  and (.enforcement_stages | map(.id) == [
    "runtime_application_promotion_contracts",
    "operator_review_side_effect_lock",
    "durable_store_runtime_switch",
    "wal_write_boundary",
    "idempotency_mutation_policy",
    "rollback_readback_execution_gate",
    "projection_enforcement_dry_run"
  ])
  and (.enforcement_stages | all(.enforcement_enabled == false))
  and (.enforcement_stages | map(select(.id == "runtime_application_promotion_contracts" and .observed_contract_count == 12 and .ready_contract_count_before == 0 and .ready_contract_count_after == 12)) | length) == 1
  and (.enforcement_stages | map(select(.id == "operator_review_side_effect_lock" and .observed_contract_count == 7 and .ready_contract_count_after == 0)) | length) == 1
  and (.enforcement_stages | map(select(.id == "durable_store_runtime_switch" and .observed_contract_count == 12 and .ready_contract_count_after == 0)) | length) == 1
  and (.enforcement_stages | map(select(.id == "wal_write_boundary" and .observed_contract_count == 12 and .ready_contract_count_after == 0)) | length) == 1
  and (.enforcement_stages | map(select(.id == "projection_enforcement_dry_run" and .observed_contract_count == 12 and .ready_contract_count_after == 0)) | length) == 1
' >/dev/null <<<"$report"

jq -e '
  .required_prior_gate_count == 47
  and (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_runtime_application_promotion_gap_closure_application_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_append_only_store_operator_review_side_effect_lock_preview_gate"
  and .ready_for_operator_review_side_effect_lock_preview == true
  and .ready_for_runtime_write_boundary_preview == false
  and .ready_for_append_only_store_enablement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_scheduler_admission_enforcement == false
  and .ready_for_role_manifest_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.runtime_application_promotion_readiness_rerun.rust_module_present == true
  and .source_probes.runtime_application_promotion_readiness_rerun.report_script_present == true
  and .source_probes.runtime_application_promotion_readiness_rerun.gate_script_present == true
  and .source_probes.runtime_application_promotion_application.rust_module_present == true
  and .source_probes.runtime_application_promotion_application.gate_script_present == true
  and .source_probes.runtime_application_promotion_application.upstream_gate == true
  and .source_probes.append_only_store_runtime_readiness_rerun.upstream_gate == true
  and .source_probes.append_only_store_runtime_readiness_rerun.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun --lib

echo "Hepta WorkGraph unified projection enforcement readiness runtime application promotion rerun preview gate passed"
