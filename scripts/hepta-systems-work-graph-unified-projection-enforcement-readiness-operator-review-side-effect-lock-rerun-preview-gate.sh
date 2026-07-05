#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-operator-review-side-effect-lock-rerun-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-unified-projection-enforcement-readiness-operator-review-side-effect-lock-rerun-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "blocked"
  and .gate == "hepta_work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview_gate"
  and .schema_version == "work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview_v1"
  and .preview_mode == "read_only_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_no_enforcement"
  and .source_surface_count == 12
  and .operator_review_side_effect_lock_outcome_count == 7
  and .operator_review_side_effect_lock_application_covered_source_count == 7
  and .previous_contract_ready_surface_count == 12
  and .operator_review_side_effect_lock_rerun_contract_ready_surface_count == 12
  and .previous_operator_review_primary_blocked_surface_count == 7
  and .operator_review_primary_blocked_surface_count_after == 0
  and .previous_write_boundary_primary_blocked_surface_count == 5
  and .write_boundary_primary_blocked_surface_count == 12
  and .operator_review_contract_ready_source_count == 12
  and .side_effect_lock_contract_ready_source_count == 12
  and .operator_review_recorded_source_count == 0
  and .side_effect_lock_established_source_count == 0
  and .wal_boundary_residual_source_count == 12
  and .rerun_ready_surface_count == 0
  and .rerun_blocked_surface_count == 12
  and .decision_delta_count == 12
  and (.decision_deltas | length) == .decision_delta_count
' >/dev/null <<<"$report"

jq -e '
  (.decision_deltas | map(select(.covered_by_operator_review_side_effect_lock_application_preview == true)) | length) == 7
  and (.decision_deltas | map(select(.operator_review_primary_gap_closed_by_application_preview == true)) | length) == 7
  and (.decision_deltas | map(select(.operator_review_side_effect_lock_rerun_enforcement_decision == "deny_operator_review_required")) | length) == 0
  and (.decision_deltas | map(select(.operator_review_side_effect_lock_rerun_enforcement_decision == "deny_runtime_append_only_store_write_boundary_disabled")) | length) == 12
  and (.decision_deltas | map(select(.source_surface_id == "multi_agent_v2_thread_spawn" and .previous_enforcement_decision == "deny_operator_review_required" and .operator_review_side_effect_lock_rerun_enforcement_decision == "deny_runtime_append_only_store_write_boundary_disabled")) | length) == 1
  and (.decision_deltas | map(select(.source_surface_id == "update_plan_tool" and .previous_enforcement_decision == "deny_runtime_append_only_store_write_boundary_disabled" and .operator_review_side_effect_lock_rerun_enforcement_decision == "deny_runtime_append_only_store_write_boundary_disabled")) | length) == 1
  and (.decision_deltas | all(.projection_contract_ready == true and .runtime_application_promotion_contract_ready == true and .operator_review_contract_ready == true and .side_effect_lock_contract_ready == true))
  and (.decision_deltas | all(.operator_review_recorded == false and .side_effect_lock_established == false and .runtime_append_only_store_enabled == false))
  and (.decision_deltas | all((.residual_source_blocker_ids | index("operator_review_required")) | not))
  and (.decision_deltas | all((.residual_source_blocker_ids | index("side_effect_lock_not_established")) | not))
  and (.decision_deltas | all((.residual_source_blocker_ids | index("operator_review_side_effect_lock_readiness_rerun_missing")) | not))
' >/dev/null <<<"$report"

jq -e '
  .cleared_blocker_count == 1
  and (.cleared_blockers[0].id == "operator_review_side_effect_lock_required_for_enforcement")
  and (.cleared_blockers[0].source_count_before == 7)
  and (.cleared_blockers[0].source_count_after == 0)
  and (.cleared_blockers[0].closure_gate_id == "hepta_work_graph_append_only_store_operator_review_side_effect_lock_application_preview_gate")
  and (.cleared_blockers[0].cleared_source_surface_ids | sort == [
    "agent_jobs_batch_workers",
    "hepta_runtime_agent_harness",
    "hepta_runtime_multi_agent_reducer",
    "hepta_runtime_scheduler_store",
    "hepta_runtime_task_board",
    "hepta_runtime_worker_tasks",
    "multi_agent_v2_thread_spawn"
  ])
  and .residual_blocker_count == 5
  and (.residual_blockers | map({id, count: (.affected_source_surface_ids | length)}) == [
    {"id": "readback_execution_disabled", "count": 12},
    {"id": "durable_store_runtime_switch_disabled", "count": 12},
    {"id": "wal_write_boundary_not_enabled", "count": 12},
    {"id": "idempotency_index_mutation_disabled", "count": 12},
    {"id": "rollback_readback_not_executed", "count": 12}
  ])
  and (.residual_blockers | all(.required_before_projection_enforcement == true))
' >/dev/null <<<"$report"

jq -e '
  .enforcement_stage_count == 6
  and (.enforcement_stages | map(.id) == [
    "operator_review_side_effect_lock_contracts",
    "durable_store_runtime_switch",
    "wal_write_boundary",
    "idempotency_mutation_policy",
    "rollback_readback_execution_gate",
    "projection_enforcement_dry_run"
  ])
  and (.enforcement_stages | all(.enforcement_enabled == false))
  and (.enforcement_stages | map(select(.id == "operator_review_side_effect_lock_contracts" and .observed_contract_count == 7 and .ready_contract_count_before == 0 and .ready_contract_count_after == 7)) | length) == 1
  and (.enforcement_stages | map(select(.id == "durable_store_runtime_switch" and .observed_contract_count == 12 and .ready_contract_count_after == 0)) | length) == 1
  and (.enforcement_stages | map(select(.id == "wal_write_boundary" and .observed_contract_count == 12 and .ready_contract_count_after == 0)) | length) == 1
  and (.enforcement_stages | map(select(.id == "rollback_readback_execution_gate" and .observed_contract_count == 12 and .ready_contract_count_after == 0)) | length) == 1
  and (.enforcement_stages | map(select(.id == "projection_enforcement_dry_run" and .observed_contract_count == 12 and .ready_contract_count_after == 0)) | length) == 1
' >/dev/null <<<"$report"

jq -e '
  .required_prior_gate_count == 51
  and (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_append_only_store_operator_review_side_effect_lock_application_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_append_only_store_runtime_write_boundary_preview_gate"
  and .ready_for_operator_review_recording == false
  and .ready_for_side_effect_lock_establishment == false
  and .ready_for_runtime_write_boundary_preview == true
  and .ready_for_append_only_store_enablement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_scheduler_admission_enforcement == false
  and .ready_for_role_manifest_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.operator_review_side_effect_lock_readiness_rerun.rust_module_present == true
  and .source_probes.operator_review_side_effect_lock_readiness_rerun.report_script_present == true
  and .source_probes.operator_review_side_effect_lock_readiness_rerun.gate_script_present == true
  and .source_probes.operator_review_side_effect_lock_application.rust_module_present == true
  and .source_probes.operator_review_side_effect_lock_application.gate_script_present == true
  and .source_probes.operator_review_side_effect_lock_application.upstream_gate == true
  and .source_probes.runtime_application_promotion_readiness_rerun.upstream_gate == true
  and .source_probes.runtime_application_promotion_readiness_rerun.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun --lib

echo "Hepta WorkGraph unified projection enforcement readiness operator review side-effect lock rerun preview gate passed"
