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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-enforcement-readiness-runtime-write-boundary-rerun.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$tmpdir/cache"
fi

capture_json_report \
  "hepta-work-graph-unified-projection-enforcement-readiness-operator-review-side-effect-lock-rerun-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-operator-review-side-effect-lock-rerun-preview-report.sh" \
  >"$tmpdir/previous.json"
capture_json_report \
  "hepta-work-graph-append-only-store-runtime-write-boundary-application-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-append-only-store-runtime-write-boundary-application-preview-report.sh" \
  >"$tmpdir/application.json"

runtime_write_boundary_rerun_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_unified_projection_enforcement_readiness_runtime_write_boundary_rerun_preview.rs
)"
runtime_write_boundary_rerun_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-write-boundary-rerun-preview-report.sh
)"
runtime_write_boundary_rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-write-boundary-rerun-preview-gate.sh
)"
runtime_write_boundary_application_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_store_runtime_write_boundary_application_preview.rs
)"
runtime_write_boundary_application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-runtime-write-boundary-application-preview-gate.sh
)"
operator_review_rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-operator-review-side-effect-lock-rerun-preview-gate.sh
)"

jq -n \
  --slurpfile previous "$tmpdir/previous.json" \
  --slurpfile application "$tmpdir/application.json" \
  --argjson runtime_write_boundary_rerun_rust_module_present "$runtime_write_boundary_rerun_rust_module_present" \
  --argjson runtime_write_boundary_rerun_report_script_present "$runtime_write_boundary_rerun_report_script_present" \
  --argjson runtime_write_boundary_rerun_gate_script_present "$runtime_write_boundary_rerun_gate_script_present" \
  --argjson runtime_write_boundary_application_rust_module_present "$runtime_write_boundary_application_rust_module_present" \
  --argjson runtime_write_boundary_application_gate_script_present "$runtime_write_boundary_application_gate_script_present" \
  --argjson operator_review_rerun_gate_script_present "$operator_review_rerun_gate_script_present" \
  '
  $previous[0] as $previous
  | $application[0] as $application
  | def unique_order: reduce .[] as $item ([]; if index($item) then . else . + [$item] end);
  def push_unique($values; $value): if ($values | index($value)) then $values else ($values + [$value]) end;
  def cleared_runtime_write_boundary_blocker($id):
      ($id == "runtime_write_boundary_readback_missing") or
      ($id == "runtime_write_boundary_application_missing") or
      ($id == "runtime_write_boundary_readiness_rerun_missing");
  def source_blocker_ids($source):
      [$application.blockers[]
        | select((.affected_source_surface_ids | index($source)) and (cleared_runtime_write_boundary_blocker(.id) | not))
        | .id];
  def runtime_decision_for($store; $timeline; $task_result; $route; $guard; $terminal_ready; $precondition_ready; $readback; $scheduler_ready; $role_ready; $runtime_enablement_ready; $runtime_application_ready; $operator_ready; $lock_ready; $write_boundary_ready; $route_blockers; $source_blockers):
      if ($store | not) then "deny_missing_unified_store_projection"
      elif ($timeline | not) then "deny_missing_timeline_projection"
      elif ($task_result | not) then "deny_missing_task_result_projection"
      elif ($route | not) then "deny_missing_append_only_route"
      elif ($guard | not) then "deny_missing_store_idempotency_guard"
      elif ($terminal_ready | not) then "deny_terminal_task_result_contract_missing"
      elif ($precondition_ready | not) then "deny_append_only_store_precondition_missing"
      elif ($readback | not) then "deny_missing_readback_probe"
      elif ($scheduler_ready | not) then "deny_scheduler_admission_not_enforced"
      elif ($role_ready | not) then "deny_role_manifest_not_enforced"
      elif (($runtime_enablement_ready | not) or ($source_blockers | index("append_only_store_runtime_enablement_disabled"))) then "deny_runtime_append_only_store_enablement_disabled"
      elif (($runtime_application_ready | not) or ($source_blockers | index("runtime_application_residuals_not_promoted"))) then "deny_runtime_application_residuals_not_promoted"
      elif (($operator_ready | not) or ($lock_ready | not) or ($source_blockers | index("operator_review_required")) or ($source_blockers | index("side_effect_lock_not_established"))) then "deny_operator_review_required"
      elif (($write_boundary_ready | not) or ($source_blockers | index("runtime_write_boundary_readback_missing")) or ($source_blockers | index("runtime_write_boundary_application_missing"))) then "deny_runtime_append_only_store_write_boundary_disabled"
      elif ($source_blockers | index("durable_store_runtime_switch_disabled")) then "deny_runtime_durable_store_switch_disabled"
      elif ($source_blockers | index("idempotency_index_mutation_disabled")) then "deny_runtime_idempotency_mutation_disabled"
      elif (($source_blockers | index("rollback_readback_not_executed")) or ($source_blockers | index("readback_execution_disabled"))) then "deny_runtime_rollback_readback_execution_disabled"
      elif ($source_blockers | index("wal_write_boundary_not_enabled")) then "deny_runtime_wal_write_boundary_not_enabled"
      elif ($route_blockers | index("append_only_store_disabled_by_design")) then "deny_append_only_store_disabled"
      else "allow_preview_only"
      end;
  def next_gate_for($decision):
      if $decision == "deny_runtime_durable_store_switch_disabled" then "hepta_work_graph_append_only_store_runtime_durable_store_switch_preview_gate"
      elif $decision == "deny_runtime_idempotency_mutation_disabled" then "hepta_work_graph_append_only_store_runtime_idempotency_mutation_preview_gate"
      elif $decision == "deny_runtime_rollback_readback_execution_disabled" then "hepta_work_graph_append_only_store_runtime_rollback_readback_execution_preview_gate"
      elif $decision == "allow_preview_only" then "hepta_work_graph_projection_enforcement_dry_run_preview_gate"
      else "hepta_work_graph_append_only_store_runtime_durable_store_switch_preview_gate"
      end;
  def rerun_decision($decision):
      (any($application.source_outcomes[]?; .source_surface_id == $decision.source_surface_id and .runtime_write_boundary_contract_ready_preview == true and .applies_to_runtime == false)) as $covered
      | ($covered or (($decision.residual_source_blocker_ids | index("runtime_write_boundary_application_missing")) | not)) as $write_boundary_ready
      | (($decision.operator_review_side_effect_lock_rerun_enforcement_decision == "deny_runtime_append_only_store_write_boundary_disabled") and $write_boundary_ready) as $primary_gap_closed
      | (($decision.residual_source_blocker_ids | map(select(cleared_runtime_write_boundary_blocker(.) | not))) + source_blocker_ids($decision.source_surface_id) | unique_order) as $source_blockers
      | (runtime_decision_for($decision.unified_store_projection_ready; $decision.timeline_projection_ready; $decision.task_result_projection_ready; $decision.append_only_route_ready; $decision.store_idempotency_guard_ready; $decision.terminal_task_result_contract_ready; $decision.append_only_store_precondition_ready; $decision.readback_probe_contract_ready; $decision.scheduler_admission_contract_ready; $decision.role_manifest_contract_ready; $decision.append_only_store_runtime_enablement_ready; $decision.runtime_application_promotion_contract_ready; $decision.operator_review_contract_ready; $decision.side_effect_lock_contract_ready; $write_boundary_ready; $decision.residual_route_blocker_ids; $source_blockers)) as $rerun_decision
      | {
          source_surface_id: $decision.source_surface_id,
          source_category: $decision.source_category,
          previous_operator_review_side_effect_lock_rerun_state: $decision.operator_review_side_effect_lock_rerun_state,
          runtime_write_boundary_rerun_state: (if $covered then "runtime_write_boundary_contract_ready_preview_after_application" else "runtime_write_boundary_application_not_required_for_source" end),
          covered_by_runtime_write_boundary_application_preview: $covered,
          previous_enforcement_decision: $decision.operator_review_side_effect_lock_rerun_enforcement_decision,
          runtime_write_boundary_rerun_enforcement_decision: $rerun_decision,
          runtime_write_boundary_primary_gap_closed_by_application_preview: $primary_gap_closed,
          projection_contract_ready: $decision.projection_contract_ready,
          unified_store_projection_ready: $decision.unified_store_projection_ready,
          timeline_projection_ready: $decision.timeline_projection_ready,
          task_result_projection_ready: $decision.task_result_projection_ready,
          store_idempotency_guard_ready: $decision.store_idempotency_guard_ready,
          terminal_task_result_contract_ready: $decision.terminal_task_result_contract_ready,
          append_only_route_ready: $decision.append_only_route_ready,
          append_only_store_precondition_ready: $decision.append_only_store_precondition_ready,
          readback_probe_contract_ready: $decision.readback_probe_contract_ready,
          scheduler_admission_contract_ready: $decision.scheduler_admission_contract_ready,
          role_manifest_contract_ready: $decision.role_manifest_contract_ready,
          append_only_store_runtime_enablement_ready: $decision.append_only_store_runtime_enablement_ready,
          runtime_application_promotion_contract_ready: $decision.runtime_application_promotion_contract_ready,
          operator_review_contract_ready: $decision.operator_review_contract_ready,
          side_effect_lock_contract_ready: $decision.side_effect_lock_contract_ready,
          runtime_write_boundary_contract_ready: $write_boundary_ready,
          runtime_write_boundary_applied: false,
          wal_write_enabled: false,
          checkpoint_write_enabled: false,
          durable_store_switch_enabled: false,
          idempotency_mutation_enabled: false,
          readback_execution_enabled: false,
          rollback_execution_enabled: false,
          runtime_append_only_store_enabled: false,
          scheduler_admission_enforcement_ready: false,
          role_manifest_enforcement_ready: false,
          residual_source_blocker_ids: $source_blockers,
          residual_route_blocker_ids: $decision.residual_route_blocker_ids,
          next_required_gate: next_gate_for($rerun_decision)
        };
  def residual_blocker($blocker): {
      id: $blocker.id,
      severity: $blocker.severity,
      category: $blocker.category,
      affected_source_surface_ids: $blocker.affected_source_surface_ids,
      required_before_projection_enforcement: true,
      recommended_fix: $blocker.recommended_fix
    };
  def stage($id; $observed; $before; $after; $blockers): {
      id: $id,
      observed_contract_count: $observed,
      ready_contract_count_before: $before,
      ready_contract_count_after: $after,
      hard_blocker_ids: $blockers,
      enforcement_enabled: false,
      next_gate: "hepta_work_graph_append_only_store_runtime_durable_store_switch_preview_gate"
    };
  def residual_sources($decisions; $ids):
      reduce $decisions[] as $decision ([]; if any($ids[]; . as $id | $decision.residual_source_blocker_ids | index($id)) then push_unique(.; $decision.source_surface_id) else . end);
  ($previous.decision_deltas | map(rerun_decision(.))) as $decisions
  | ($previous.decision_deltas | map(select(.operator_review_side_effect_lock_rerun_enforcement_decision == "deny_runtime_append_only_store_write_boundary_disabled") | .source_surface_id)) as $before_write_boundary_sources
  | ($decisions | map(select(.runtime_write_boundary_rerun_enforcement_decision == "deny_runtime_append_only_store_write_boundary_disabled") | .source_surface_id)) as $after_write_boundary_sources
  | ($application.blockers | map(select(cleared_runtime_write_boundary_blocker(.id) | not) | residual_blocker(.))) as $residual_blockers
  | (residual_sources($decisions; ["durable_store_runtime_switch_disabled"])) as $durable_sources
  | (residual_sources($decisions; ["wal_write_boundary_not_enabled"])) as $wal_sources
  | (residual_sources($decisions; ["idempotency_index_mutation_disabled"])) as $idempotency_sources
  | (residual_sources($decisions; ["readback_execution_disabled","rollback_readback_not_executed"])) as $rollback_sources
  | ([
      stage("runtime_write_boundary_contracts"; ($application.source_outcomes | length); 0; ($decisions | map(select(.covered_by_runtime_write_boundary_application_preview)) | length); ["durable_store_runtime_switch_disabled"]),
      stage("durable_store_runtime_switch"; ($durable_sources | length); 0; 0; ["durable_store_runtime_switch_disabled"]),
      stage("wal_write_boundary_execution"; ($wal_sources | length); 0; 0; ["wal_write_boundary_not_enabled"]),
      stage("idempotency_mutation_policy"; ($idempotency_sources | length); 0; 0; ["idempotency_index_mutation_disabled"]),
      stage("rollback_readback_execution_gate"; ($rollback_sources | length); 0; 0; ["readback_execution_disabled","rollback_readback_not_executed"]),
      stage("projection_enforcement_dry_run"; ($decisions | length); 0; 0; ["durable_store_runtime_switch_disabled","wal_write_boundary_not_enabled","idempotency_index_mutation_disabled","rollback_readback_not_executed"])
    ]) as $stages
  | ($application.required_prior_gates + [$application.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_unified_projection_enforcement_readiness_runtime_write_boundary_rerun_preview_gate",
      schema_version: "work_graph_unified_projection_enforcement_readiness_runtime_write_boundary_rerun_preview_v1",
      preview_mode: "read_only_projection_enforcement_readiness_runtime_write_boundary_rerun_no_enforcement",
      source_surface_count: ($decisions | length),
      runtime_write_boundary_outcome_count: ($application.source_outcomes | length),
      runtime_write_boundary_application_covered_source_count: ($decisions | map(select(.covered_by_runtime_write_boundary_application_preview)) | length),
      previous_contract_ready_surface_count: ($previous.decision_deltas | map(select(.projection_contract_ready)) | length),
      runtime_write_boundary_rerun_contract_ready_surface_count: ($decisions | map(select(.projection_contract_ready)) | length),
      previous_write_boundary_primary_blocked_surface_count: ($before_write_boundary_sources | length),
      write_boundary_primary_blocked_surface_count_after: ($after_write_boundary_sources | length),
      durable_store_primary_blocked_surface_count: ($decisions | map(select(.runtime_write_boundary_rerun_enforcement_decision == "deny_runtime_durable_store_switch_disabled")) | length),
      runtime_write_boundary_contract_ready_source_count: ($decisions | map(select(.runtime_write_boundary_contract_ready)) | length),
      wal_write_enabled_source_count: ($decisions | map(select(.wal_write_enabled)) | length),
      durable_store_switch_enabled_source_count: ($decisions | map(select(.durable_store_switch_enabled)) | length),
      idempotency_mutation_enabled_source_count: ($decisions | map(select(.idempotency_mutation_enabled)) | length),
      rollback_readback_execution_enabled_source_count: ($decisions | map(select(.readback_execution_enabled and .rollback_execution_enabled)) | length),
      rerun_ready_surface_count: ($decisions | map(select(.runtime_write_boundary_rerun_enforcement_decision == "allow_preview_only")) | length),
      rerun_blocked_surface_count: ($decisions | map(select(.runtime_write_boundary_rerun_enforcement_decision != "allow_preview_only")) | length),
      decision_delta_count: ($decisions | length),
      cleared_blocker_count: 1,
      residual_blocker_count: ($residual_blockers | length),
      enforcement_stage_count: ($stages | length),
      required_prior_gate_count: ($required_prior_gates | length),
      decision_deltas: $decisions,
      cleared_blockers: [{
        id: "runtime_write_boundary_required_for_enforcement",
        cleared_source_surface_ids: $before_write_boundary_sources,
        source_count_before: ($before_write_boundary_sources | length),
        source_count_after: ($after_write_boundary_sources | length),
        closure_gate_id: "hepta_work_graph_append_only_store_runtime_write_boundary_application_preview_gate"
      }],
      residual_blockers: $residual_blockers,
      enforcement_stages: $stages,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_append_only_store_runtime_durable_store_switch_preview_gate",
      ready_for_runtime_durable_store_switch_preview: true,
      ready_for_wal_write: false,
      ready_for_checkpoint_write: false,
      ready_for_idempotency_mutation: false,
      ready_for_readback_execution: false,
      ready_for_rollback_execution: false,
      ready_for_append_only_store_enablement: false,
      ready_for_projection_enforcement: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        runtime_write_boundary_readiness_rerun: {
          rust_module_present: $runtime_write_boundary_rerun_rust_module_present,
          report_script_present: $runtime_write_boundary_rerun_report_script_present,
          gate_script_present: $runtime_write_boundary_rerun_gate_script_present
        },
        runtime_write_boundary_application: {
          rust_module_present: $runtime_write_boundary_application_rust_module_present,
          gate_script_present: $runtime_write_boundary_application_gate_script_present,
          upstream_gate: ($application.gate == "hepta_work_graph_append_only_store_runtime_write_boundary_application_preview_gate")
        },
        operator_review_side_effect_lock_readiness_rerun: {
          upstream_gate: ($previous.gate == "hepta_work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview_gate"),
          gate_script_present: $operator_review_rerun_gate_script_present
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
        lane_lease_acquired: false,
        work_started: false,
        budget_consumed: false,
        approval_recorded: false,
        operator_review_recorded: false,
        side_effect_lock_established: false,
        task_result_enforcement_enabled: false,
        task_result_persisted: false,
        role_manifest_enforcement_enabled: false,
        tool_permission_changed: false,
        role_budget_consumed: false,
        role_lane_binding_mutated: false,
        readback_executed: false,
        rollback_executed: false,
        runtime_application_promoted: false,
        runtime_wrapper_attached: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }
  '
