#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-idempotency-mutation-rerun-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-unified-projection-enforcement-readiness-runtime-idempotency-mutation-rerun-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "blocked"
  and .gate == "hepta_work_graph_unified_projection_enforcement_readiness_runtime_idempotency_mutation_rerun_preview_gate"
  and .schema_version == "work_graph_unified_projection_enforcement_readiness_runtime_idempotency_mutation_rerun_preview_v1"
  and .preview_mode == "read_only_projection_enforcement_readiness_runtime_idempotency_mutation_rerun_no_enforcement"
  and .source_surface_count == 12
  and .idempotency_mutation_outcome_count == 12
  and .idempotency_mutation_application_covered_source_count == 12
  and .previous_contract_ready_surface_count == 12
  and .runtime_idempotency_mutation_rerun_contract_ready_surface_count == 12
  and .previous_idempotency_primary_blocked_surface_count == 12
  and .idempotency_primary_blocked_surface_count_after == 0
  and .rollback_readback_primary_blocked_surface_count == 12
  and .idempotency_mutation_contract_ready_source_count == 12
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
  (.decision_deltas | map(select(.covered_by_idempotency_mutation_application_preview == true)) | length) == 12
  and (.decision_deltas | map(select(.idempotency_mutation_primary_gap_closed_by_application_preview == true)) | length) == 12
  and (.decision_deltas | map(select(.runtime_idempotency_mutation_rerun_enforcement_decision == "deny_runtime_idempotency_mutation_disabled")) | length) == 0
  and (.decision_deltas | map(select(.runtime_idempotency_mutation_rerun_enforcement_decision == "deny_runtime_rollback_readback_execution_disabled")) | length) == 12
  and (.decision_deltas | all(.projection_contract_ready == true and .runtime_application_promotion_contract_ready == true and .operator_review_contract_ready == true and .side_effect_lock_contract_ready == true and .durable_store_switch_contract_ready == true and .idempotency_mutation_contract_ready == true))
  and (.decision_deltas | all(.idempotency_mutation_applied == false and .wal_write_enabled == false and .checkpoint_write_enabled == false and .durable_store_switch_enabled == false and .idempotency_mutation_enabled == false and .readback_execution_enabled == false and .rollback_execution_enabled == false))
  and (.decision_deltas | all((.residual_source_blocker_ids | index("idempotency_index_mutation_disabled")) | not))
  and (.decision_deltas | all((.residual_source_blocker_ids | index("idempotency_mutation_readback_missing")) | not))
  and (.decision_deltas | all((.residual_source_blocker_ids | index("idempotency_mutation_application_missing")) | not))
  and (.decision_deltas | all((.residual_source_blocker_ids | index("idempotency_mutation_readiness_rerun_missing")) | not))
  and (.decision_deltas | all(.next_required_gate == "hepta_work_graph_append_only_store_runtime_rollback_readback_execution_preview_gate"))
' >/dev/null <<<"$report"

jq -e '
  .cleared_blocker_count == 1
  and (.cleared_blockers[0].id == "idempotency_mutation_required_for_enforcement")
  and (.cleared_blockers[0].source_count_before == 12)
  and (.cleared_blockers[0].source_count_after == 0)
  and (.cleared_blockers[0].closure_gate_id == "hepta_work_graph_append_only_store_runtime_idempotency_mutation_application_preview_gate")
  and .residual_blocker_count == 3
  and (.residual_blockers | map({id, category, count: (.affected_source_surface_ids | length)}) == [
    {"id": "readback_execution_disabled", "category": "rollback_readback", "count": 12},
    {"id": "wal_write_boundary_not_enabled", "category": "wal_replay_prerequisite", "count": 12},
    {"id": "rollback_readback_not_executed", "category": "rollback_readback", "count": 12}
  ])
  and (.residual_blockers | all(.required_before_projection_enforcement == true))
' >/dev/null <<<"$report"

jq -e '
  .enforcement_stage_count == 5
  and (.enforcement_stages | map(.id) == [
    "idempotency_mutation_contracts",
    "wal_write_boundary_execution",
    "rollback_readback_execution_gate",
    "append_only_store_enablement_dry_run",
    "projection_enforcement_dry_run"
  ])
  and (.enforcement_stages | all(.enforcement_enabled == false and .next_gate == "hepta_work_graph_append_only_store_runtime_rollback_readback_execution_preview_gate"))
  and (.enforcement_stages | map(select(.id == "idempotency_mutation_contracts" and .observed_contract_count == 12 and .ready_contract_count_before == 0 and .ready_contract_count_after == 12)) | length) == 1
  and (.enforcement_stages | map(select(.id == "wal_write_boundary_execution" and .observed_contract_count == 12 and .ready_contract_count_after == 0)) | length) == 1
  and (.enforcement_stages | map(select(.id == "rollback_readback_execution_gate" and .observed_contract_count == 12 and .ready_contract_count_after == 0)) | length) == 1
  and (.enforcement_stages | map(select(.id == "append_only_store_enablement_dry_run" and .observed_contract_count == 12 and .ready_contract_count_after == 0)) | length) == 1
  and (.enforcement_stages | map(select(.id == "projection_enforcement_dry_run" and .observed_contract_count == 12 and .ready_contract_count_after == 0)) | length) == 1
' >/dev/null <<<"$report"

jq -e '
  .required_prior_gate_count == 63
  and (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_append_only_store_runtime_idempotency_mutation_application_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_append_only_store_runtime_rollback_readback_execution_preview_gate"
  and .ready_for_runtime_rollback_readback_execution_preview == true
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
  and .source_probes.runtime_idempotency_mutation_readiness_rerun.rust_module_present == true
  and .source_probes.runtime_idempotency_mutation_readiness_rerun.report_script_present == true
  and .source_probes.runtime_idempotency_mutation_readiness_rerun.gate_script_present == true
  and .source_probes.runtime_idempotency_mutation_application.rust_module_present == true
  and .source_probes.runtime_idempotency_mutation_application.gate_script_present == true
  and .source_probes.runtime_idempotency_mutation_application.upstream_gate == true
  and .source_probes.runtime_durable_store_switch_readiness_rerun.upstream_gate == true
  and .source_probes.runtime_durable_store_switch_readiness_rerun.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_unified_projection_enforcement_readiness_runtime_idempotency_mutation_rerun --lib

echo "Hepta WorkGraph unified projection enforcement readiness runtime idempotency mutation rerun preview gate passed"
