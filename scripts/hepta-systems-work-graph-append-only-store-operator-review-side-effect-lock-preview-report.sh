#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

path_exists() {
  local path="$1"
  [[ -e "$path" ]]
}

bool_for() {
  if "$@"; then
    printf 'true\n'
  else
    printf 'false\n'
  fi
}

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-operator-review-side-effect-lock.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$tmpdir/cache"
fi

capture_json_report \
  "hepta-work-graph-unified-projection-enforcement-readiness-runtime-application-promotion-rerun-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-application-promotion-rerun-preview-report.sh" \
  >"$tmpdir/runtime_application_rerun.json"

operator_review_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_store_operator_review_side_effect_lock_preview.rs
)"
operator_review_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-operator-review-side-effect-lock-preview-report.sh
)"
operator_review_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-operator-review-side-effect-lock-preview-gate.sh
)"
runtime_application_rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-application-promotion-rerun-preview-gate.sh
)"

jq -n \
  --slurpfile rerun "$tmpdir/runtime_application_rerun.json" \
  --argjson operator_review_rust_module_present "$operator_review_rust_module_present" \
  --argjson operator_review_report_script_present "$operator_review_report_script_present" \
  --argjson operator_review_gate_script_present "$operator_review_gate_script_present" \
  --argjson runtime_application_rerun_gate_script_present "$runtime_application_rerun_gate_script_present" \
  '
  $rerun[0] as $rerun
  | def evidence_fields: [
      "source_surface_id",
      "source_category",
      "runtime_application_promotion_rerun_decision",
      "operator_review_packet_id",
      "side_effect_lock_plan_id",
      "approval_evidence_boundary_id",
      "readback_boundary_id",
      "residual_source_blocker_ids"
    ];
  def packet_sections: [
      "runtime_application_contract_summary",
      "operator_review_scope",
      "side_effect_lock_scope",
      "write_boundary_residuals",
      "no_mutation_guard_evidence"
    ];
  def lock_scopes: [
      "runtime_application_promotion",
      "wal_write_boundary",
      "durable_store_runtime_switch",
      "idempotency_mutation_policy",
      "rollback_readback_execution"
    ];
  def groups: [
      {category: "multi_agent", id: "multi_agent_operator_review_group", priority: "p0"},
      {category: "batch_agent_jobs", id: "batch_agent_jobs_operator_review_group", priority: "p1"},
      {category: "runtime_scheduler", id: "runtime_scheduler_operator_review_group", priority: "p0"},
      {category: "external_handoff", id: "external_handoff_operator_review_group", priority: "p1"}
    ];
  def packet_id($source): "operator_review_packet__" + $source;
  def lock_plan_id($source): "side_effect_lock_plan__" + $source;
  def approval_boundary_id($source): "approval_evidence_boundary__" + $source;
  def readback_boundary_id($source): "operator_review_readback_boundary__" + $source;
  def readback_probe_id($source): "operator_review_side_effect_lock_readback_probe__" + $source;
  def operator_review_decisions:
      [.decision_deltas[]
       | select(
          .runtime_application_promotion_rerun_enforcement_decision == "deny_operator_review_required"
          or (.residual_source_blocker_ids | index("operator_review_required"))
          or (.residual_source_blocker_ids | index("side_effect_lock_not_established"))
        )];
  def guard($id; $severity; $scope): {
      id: $id,
      severity: $severity,
      scope: $scope,
      enforced_in_preview: true,
      prevents_runtime_mutation: true,
      note: "preview records review and side-effect-lock contracts only; no approval or lock is applied"
    };
  def blocks_operator_review($id):
      $id == "operator_review_required"
      or $id == "side_effect_lock_not_established"
      or $id == "operator_review_side_effect_lock_readback_missing";
  def blocks_side_effect_lock($id):
      $id == "side_effect_lock_not_established"
      or $id == "operator_review_required"
      or $id == "operator_review_side_effect_lock_readback_missing";
  def blocks_runtime_write_boundary($id):
      $id == "readback_execution_disabled"
      or $id == "durable_store_runtime_switch_disabled"
      or $id == "wal_write_boundary_not_enabled"
      or $id == "idempotency_index_mutation_disabled"
      or $id == "rollback_readback_not_executed";
  ($rerun | operator_review_decisions) as $operator_review_decisions
  | ($operator_review_decisions | map({
      packet_id: packet_id(.source_surface_id),
      source_surface_id: .source_surface_id,
      source_category: .source_category,
      runtime_application_promotion_rerun_decision: .runtime_application_promotion_rerun_enforcement_decision,
      required_section_ids: packet_sections,
      evidence_field_ids: evidence_fields,
      side_effect_lock_plan_id: lock_plan_id(.source_surface_id),
      approval_evidence_boundary_id: approval_boundary_id(.source_surface_id),
      readback_boundary_id: readback_boundary_id(.source_surface_id),
      packet_state: "preview_only_operator_review_not_recorded",
      ready_for_readback_preview: true,
      external_delivery_enabled: false,
      operator_review_recorded: false,
      approval_recorded: false,
      mutates_store: false,
      writes_wal: false,
      applies_to_runtime: false
    })) as $packets
  | ($operator_review_decisions | map({
      lock_plan_id: lock_plan_id(.source_surface_id),
      source_surface_id: .source_surface_id,
      source_category: .source_category,
      lock_scope_ids: lock_scopes,
      lock_state: "planned_not_established",
      prevents_runtime_mutation: true,
      side_effects_allowed: false,
      lock_established: false,
      writes_store: false,
      writes_wal: false
    })) as $lock_plans
  | ($operator_review_decisions | map({
      boundary_id: approval_boundary_id(.source_surface_id),
      source_surface_id: .source_surface_id,
      required_evidence_field_ids: evidence_fields,
      redaction_state: "redacted_preview_only",
      records_operator_review: false,
      records_approval: false,
      persists_receipt: false,
      external_delivery_enabled: false
    })) as $approval_boundaries
  | ($operator_review_decisions | map({
      boundary_id: readback_boundary_id(.source_surface_id),
      source_surface_id: .source_surface_id,
      readback_probe_id: readback_probe_id(.source_surface_id),
      readback_state: "planned_not_executed",
      ready_for_readback_preview: true,
      readback_executed: false,
      rollback_executed: false,
      writes_checkpoint: false
    })) as $readback_boundaries
  | (groups | map(. as $group
      | ($operator_review_decisions | map(select(.source_category == $group.category) | .source_surface_id)) as $sources
      | {
          id: $group.id,
          source_category: $group.category,
          priority: $group.priority,
          affected_source_surface_ids: $sources,
          operator_review_packet_ids: ($sources | map(packet_id(.))),
          side_effect_lock_plan_ids: ($sources | map(lock_plan_id(.))),
          expected_review_packet_count: ($sources | length),
          ready_for_application_preview: false
        })) as $operator_review_groups
  | ([
      guard("operator_review_side_effect_lock_preview_only"; "critical"; "preview"),
      guard("operator_review_recording_disabled"; "critical"; "operator_review"),
      guard("approval_recording_disabled"; "critical"; "approval"),
      guard("side_effect_lock_not_established"; "critical"; "side_effect_lock"),
      guard("external_delivery_disabled"; "critical"; "delivery"),
      guard("runtime_mutation_disabled"; "critical"; "runtime"),
      guard("wal_write_boundary_disabled"; "critical"; "wal"),
      guard("durable_store_runtime_switch_disabled"; "critical"; "store"),
      guard("idempotency_mutation_disabled"; "critical"; "idempotency"),
      guard("readback_rollback_execution_disabled"; "critical"; "readback"),
      guard("model_invocation_disabled"; "high"; "model")
    ]) as $guards
  | (($rerun.residual_blockers | map({
      id,
      severity,
      affected_source_surface_ids,
      blocks_operator_review: blocks_operator_review(.id),
      blocks_side_effect_lock: blocks_side_effect_lock(.id),
      blocks_runtime_write_boundary: blocks_runtime_write_boundary(.id),
      recommended_fix
    })) + [{
      id: "operator_review_side_effect_lock_readback_missing",
      severity: "high",
      affected_source_surface_ids: ($operator_review_decisions | map(.source_surface_id)),
      blocks_operator_review: true,
      blocks_side_effect_lock: true,
      blocks_runtime_write_boundary: false,
      recommended_fix: "read back every operator-review packet and side-effect lock plan before application preview"
    }]) as $blockers
  | ($rerun.required_prior_gates + [$rerun.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_append_only_store_operator_review_side_effect_lock_preview_gate",
      schema_version: "work_graph_append_only_store_operator_review_side_effect_lock_preview_v1",
      preview_mode: "read_only_append_only_store_operator_review_side_effect_lock_preview_no_approval",
      upstream_runtime_application_promotion_rerun_gate: "hepta_work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview_gate",
      upstream_operator_review_residual_source_count: ($operator_review_decisions | length),
      upstream_side_effect_lock_residual_source_count: ($rerun.decision_deltas | map(select(.residual_source_blocker_ids | index("side_effect_lock_not_established"))) | length),
      upstream_write_boundary_primary_blocked_source_count: ($rerun.decision_deltas | map(select(.runtime_application_promotion_rerun_enforcement_decision == "deny_runtime_append_only_store_write_boundary_disabled")) | length),
      operator_review_packet_count: ($packets | length),
      side_effect_lock_plan_count: ($lock_plans | length),
      approval_evidence_boundary_count: ($approval_boundaries | length),
      readback_boundary_count: ($readback_boundaries | length),
      evidence_field_ref_count: ($packets | map(.evidence_field_ids | length) | add),
      operator_review_group_count: ($operator_review_groups | length),
      guard_count: ($guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      operator_review_packets: $packets,
      side_effect_lock_plans: $lock_plans,
      approval_evidence_boundaries: $approval_boundaries,
      readback_boundaries: $readback_boundaries,
      operator_review_groups: $operator_review_groups,
      guards: $guards,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_append_only_store_operator_review_side_effect_lock_readback_preview_gate",
      ready_for_operator_review_side_effect_lock_readback_preview: true,
      ready_for_operator_review_side_effect_lock_application_preview: false,
      ready_for_operator_review_recording: false,
      ready_for_side_effect_lock_establishment: false,
      ready_for_runtime_write_boundary_preview: false,
      ready_for_append_only_store_enablement: false,
      ready_for_projection_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        operator_review_side_effect_lock_preview: {
          rust_module_present: $operator_review_rust_module_present,
          report_script_present: $operator_review_report_script_present,
          gate_script_present: $operator_review_gate_script_present
        },
        runtime_application_promotion_rerun: {
          upstream_gate: ($rerun.gate == "hepta_work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview_gate"),
          gate_script_present: $runtime_application_rerun_gate_script_present,
          recommended_next_matches: ($rerun.recommended_next_gate == "hepta_work_graph_append_only_store_operator_review_side_effect_lock_preview_gate")
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        wal_written: false,
        checkpoint_written: false,
        durable_store_switch_enabled: false,
        idempotency_index_mutated: false,
        append_only_store_enabled: false,
        projection_enforcement_enabled: false,
        scheduler_admission_enforced: false,
        role_manifest_enforced: false,
        task_result_enforcement_enabled: false,
        task_result_persisted: false,
        lane_lease_acquired: false,
        work_started: false,
        budget_consumed: false,
        approval_recorded: false,
        operator_review_recorded: false,
        side_effect_lock_established: false,
        readback_executed: false,
        rollback_executed: false,
        runtime_application_promoted: false,
        runtime_mutation_performed: false,
        external_send_performed: false,
        model_invoked: false,
        agent_spawn_performed: false
      }
    }
  '
