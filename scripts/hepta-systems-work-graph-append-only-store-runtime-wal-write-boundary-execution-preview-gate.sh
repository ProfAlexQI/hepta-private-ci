#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-append-only-store-runtime-wal-write-boundary-execution-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-append-only-store-runtime-wal-write-boundary-execution-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "blocked"
  and .gate == "hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_preview_gate"
  and .schema_version == "work_graph_append_only_store_runtime_wal_write_boundary_execution_preview_v1"
  and .preview_mode == "read_only_append_only_store_runtime_wal_write_boundary_execution_preview_no_index_mutation"
  and .upstream_runtime_rollback_readback_execution_rerun_gate == "hepta_work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_preview_gate"
  and .source_surface_count == 12
  and .wal_write_boundary_execution_plan_count == 12
  and .wal_write_boundary_execution_stage_count == 5
  and .wal_write_boundary_execution_stage_source_ref_count == 60
  and .wal_write_boundary_execution_stage_contract_ref_count == 28
  and .wal_write_boundary_execution_plan_stage_ref_count == 60
  and .wal_write_boundary_execution_plan_evidence_field_ref_count == 108
  and .idempotency_residual_source_count == 0
  and .wal_boundary_residual_source_count == 12
  and .rollback_readback_residual_source_count == 0
  and .guard_count == 8
  and .blocker_count == 2
  and .required_prior_gate_count == 68
' >/dev/null <<<"$report"

jq -e '
  (.wal_write_boundary_execution_plans | length) == 12
  and (.wal_write_boundary_execution_plans | all(
    .previous_enforcement_decision == "deny_runtime_wal_write_boundary_not_enabled"
    and .wal_write_boundary_execution_state == "wal_write_boundary_execution_contract_defined_preview_only"
    and (.required_wal_write_boundary_execution_stage_ids | length) == 5
    and (.expected_evidence_field_ids | length) == 9
    and .wal_write_boundary_execution_policy_contract_ready_preview == true
    and .collision_replay_evidence_contract_ready_preview == true
    and .applies_to_runtime == false
    and .writes_wal == false
    and .writes_checkpoint == false
    and .mutates_idempotency_index == false
    and .executes_replay == false
    and .executes_readback == false
    and .executes_rollback == false
    and .mutates_runtime == false
  ))
  and (.wal_write_boundary_execution_stage_plans | all(
    .priority == "p0"
    and .expected_runtime_state == "contract_ready_preview_runtime_disabled"
    and .contract_ready_preview == true
    and .runtime_enabled_after_preview == false
    and (.prerequisite_gate_ids[-1] == "hepta_work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_preview_gate")
  ))
' >/dev/null <<<"$report"

jq -e '
  (.blockers | map({id, count: (.affected_source_surface_ids | length)}) == [
    {"id": "wal_write_boundary_not_enabled", "count": 12},
    {"id": "wal_write_boundary_execution_readback_missing", "count": 12}
  ])
  and (.blockers | all(.required_before_wal_write_boundary_execution == true))
  and (.guards | all(.required_before_wal_write_boundary_execution == true and .satisfied_by_preview == false))
  and (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_preview_gate"
  and .ready_for_runtime_wal_write_boundary_execution_readback_preview == true
  and .ready_for_runtime_wal_write_boundary_execution_application_preview == false
  and .ready_for_wal_write == false
  and .ready_for_checkpoint_write == false
  and .ready_for_readback_execution == false
  and .ready_for_rollback_execution == false
  and .ready_for_live_execution == false
  and .source_probes.wal_write_boundary_execution_preview.rust_module_present == true
  and .source_probes.wal_write_boundary_execution_preview.report_script_present == true
  and .source_probes.wal_write_boundary_execution_preview.gate_script_present == true
  and .source_probes.runtime_rollback_readback_execution_rerun.upstream_gate == true
  and .source_probes.runtime_rollback_readback_execution_rerun.gate_script_present == true
  and .source_probes.runtime_rollback_readback_execution_rerun.recommended_next_matches == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_append_only_store_runtime_wal_write_boundary_execution_preview --lib

echo "Hepta WorkGraph append-only store runtime WAL write-boundary execution preview gate passed"
