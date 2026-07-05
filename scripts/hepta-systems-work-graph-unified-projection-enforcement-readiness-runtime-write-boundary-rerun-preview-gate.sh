#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-write-boundary-rerun-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-unified-projection-enforcement-readiness-runtime-write-boundary-rerun-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "blocked"
  and .gate == "hepta_work_graph_unified_projection_enforcement_readiness_runtime_write_boundary_rerun_preview_gate"
  and .schema_version == "work_graph_unified_projection_enforcement_readiness_runtime_write_boundary_rerun_preview_v1"
  and .preview_mode == "read_only_projection_enforcement_readiness_runtime_write_boundary_rerun_no_enforcement"
  and .source_surface_count == 12
  and .runtime_write_boundary_outcome_count == 12
  and .runtime_write_boundary_application_covered_source_count == 12
  and .previous_contract_ready_surface_count == 12
  and .runtime_write_boundary_rerun_contract_ready_surface_count == 12
  and .previous_write_boundary_primary_blocked_surface_count == 12
  and .write_boundary_primary_blocked_surface_count_after == 0
  and .durable_store_primary_blocked_surface_count == 12
  and .runtime_write_boundary_contract_ready_source_count == 12
  and .wal_write_enabled_source_count == 0
  and .durable_store_switch_enabled_source_count == 0
  and .idempotency_mutation_enabled_source_count == 0
  and .rollback_readback_execution_enabled_source_count == 0
  and .rerun_ready_surface_count == 0
  and .rerun_blocked_surface_count == 12
  and .decision_delta_count == 12
  and (.decision_deltas | length) == .decision_delta_count
' >/dev/null <<<"$report"

jq -e '
  (.decision_deltas | map(select(.covered_by_runtime_write_boundary_application_preview == true)) | length) == 12
  and (.decision_deltas | map(select(.runtime_write_boundary_primary_gap_closed_by_application_preview == true)) | length) == 12
  and (.decision_deltas | map(select(.runtime_write_boundary_rerun_enforcement_decision == "deny_runtime_append_only_store_write_boundary_disabled")) | length) == 0
  and (.decision_deltas | map(select(.runtime_write_boundary_rerun_enforcement_decision == "deny_runtime_durable_store_switch_disabled")) | length) == 12
  and (.decision_deltas | all(.projection_contract_ready == true and .runtime_application_promotion_contract_ready == true and .operator_review_contract_ready == true and .side_effect_lock_contract_ready == true and .runtime_write_boundary_contract_ready == true))
  and (.decision_deltas | all(.runtime_write_boundary_applied == false and .wal_write_enabled == false and .checkpoint_write_enabled == false and .durable_store_switch_enabled == false and .idempotency_mutation_enabled == false and .readback_execution_enabled == false and .rollback_execution_enabled == false))
  and (.decision_deltas | all((.residual_source_blocker_ids | index("runtime_write_boundary_readback_missing")) | not))
  and (.decision_deltas | all((.residual_source_blocker_ids | index("runtime_write_boundary_application_missing")) | not))
  and (.decision_deltas | all((.residual_source_blocker_ids | index("runtime_write_boundary_readiness_rerun_missing")) | not))
  and (.decision_deltas | all(.next_required_gate == "hepta_work_graph_append_only_store_runtime_durable_store_switch_preview_gate"))
' >/dev/null <<<"$report"

jq -e '
  .cleared_blocker_count == 1
  and (.cleared_blockers[0].id == "runtime_write_boundary_required_for_enforcement")
  and (.cleared_blockers[0].source_count_before == 12)
  and (.cleared_blockers[0].source_count_after == 0)
  and (.cleared_blockers[0].closure_gate_id == "hepta_work_graph_append_only_store_runtime_write_boundary_application_preview_gate")
  and (.cleared_blockers[0].cleared_source_surface_ids | sort == [
    "agent_jobs_batch_workers",
    "app_server_turn_plan_notification",
    "hepta_runtime_agent_harness",
    "hepta_runtime_approval_broker",
    "hepta_runtime_multi_agent_reducer",
    "hepta_runtime_scheduler_store",
    "hepta_runtime_task_board",
    "hepta_runtime_worker_tasks",
    "multi_agent_v2_mailbox_wait",
    "multi_agent_v2_thread_spawn",
    "plan_mode_proposed_plan_blocks",
    "update_plan_tool"
  ])
  and .residual_blocker_count == 5
  and (.residual_blockers | map({id, category, count: (.affected_source_surface_ids | length)}) == [
    {"id": "readback_execution_disabled", "category": "rollback_readback", "count": 12},
    {"id": "durable_store_runtime_switch_disabled", "category": "durable_store_switch", "count": 12},
    {"id": "wal_write_boundary_not_enabled", "category": "wal_boundary", "count": 12},
    {"id": "idempotency_index_mutation_disabled", "category": "idempotency_policy", "count": 12},
    {"id": "rollback_readback_not_executed", "category": "rollback_readback", "count": 12}
  ])
  and (.residual_blockers | all(.required_before_projection_enforcement == true))
' >/dev/null <<<"$report"

jq -e '
  .enforcement_stage_count == 6
  and (.enforcement_stages | map(.id) == [
    "runtime_write_boundary_contracts",
    "durable_store_runtime_switch",
    "wal_write_boundary_execution",
    "idempotency_mutation_policy",
    "rollback_readback_execution_gate",
    "projection_enforcement_dry_run"
  ])
  and (.enforcement_stages | all(.enforcement_enabled == false and .next_gate == "hepta_work_graph_append_only_store_runtime_durable_store_switch_preview_gate"))
  and (.enforcement_stages | map(select(.id == "runtime_write_boundary_contracts" and .observed_contract_count == 12 and .ready_contract_count_before == 0 and .ready_contract_count_after == 12)) | length) == 1
  and (.enforcement_stages | map(select(.id == "durable_store_runtime_switch" and .observed_contract_count == 12 and .ready_contract_count_after == 0)) | length) == 1
  and (.enforcement_stages | map(select(.id == "wal_write_boundary_execution" and .observed_contract_count == 12 and .ready_contract_count_after == 0)) | length) == 1
  and (.enforcement_stages | map(select(.id == "idempotency_mutation_policy" and .observed_contract_count == 12 and .ready_contract_count_after == 0)) | length) == 1
  and (.enforcement_stages | map(select(.id == "rollback_readback_execution_gate" and .observed_contract_count == 12 and .ready_contract_count_after == 0)) | length) == 1
  and (.enforcement_stages | map(select(.id == "projection_enforcement_dry_run" and .observed_contract_count == 12 and .ready_contract_count_after == 0)) | length) == 1
' >/dev/null <<<"$report"

jq -e '
  .required_prior_gate_count == 55
  and (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_append_only_store_runtime_write_boundary_application_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_append_only_store_runtime_durable_store_switch_preview_gate"
  and .ready_for_runtime_durable_store_switch_preview == true
  and .ready_for_wal_write == false
  and .ready_for_checkpoint_write == false
  and .ready_for_idempotency_mutation == false
  and .ready_for_readback_execution == false
  and .ready_for_rollback_execution == false
  and .ready_for_append_only_store_enablement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_scheduler_admission_enforcement == false
  and .ready_for_role_manifest_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.runtime_write_boundary_readiness_rerun.rust_module_present == true
  and .source_probes.runtime_write_boundary_readiness_rerun.report_script_present == true
  and .source_probes.runtime_write_boundary_readiness_rerun.gate_script_present == true
  and .source_probes.runtime_write_boundary_application.rust_module_present == true
  and .source_probes.runtime_write_boundary_application.gate_script_present == true
  and .source_probes.runtime_write_boundary_application.upstream_gate == true
  and .source_probes.operator_review_side_effect_lock_readiness_rerun.upstream_gate == true
  and .source_probes.operator_review_side_effect_lock_readiness_rerun.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_unified_projection_enforcement_readiness_runtime_write_boundary_rerun --lib

echo "Hepta WorkGraph unified projection enforcement readiness runtime write-boundary rerun preview gate passed"
