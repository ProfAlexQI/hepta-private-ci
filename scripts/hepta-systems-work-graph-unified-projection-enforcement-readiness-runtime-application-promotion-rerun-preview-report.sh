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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-enforcement-readiness-runtime-application-rerun.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-unified-projection-enforcement-readiness-append-only-store-runtime-rerun-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-append-only-store-runtime-rerun-preview-report.sh" \
  >"$tmpdir/append_only_runtime_rerun.json"
capture_json_report \
  "hepta-work-graph-runtime-application-promotion-gap-closure-application-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-runtime-application-promotion-gap-closure-application-preview-report.sh" \
  >"$tmpdir/runtime_application_promotion_application.json"

runtime_application_rerun_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview.rs
)"
runtime_application_rerun_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-application-promotion-rerun-preview-report.sh
)"
runtime_application_rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-application-promotion-rerun-preview-gate.sh
)"
runtime_application_application_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_runtime_application_promotion_gap_closure_application_preview.rs
)"
runtime_application_application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-runtime-application-promotion-gap-closure-application-preview-gate.sh
)"
append_only_runtime_rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-append-only-store-runtime-rerun-preview-gate.sh
)"

jq -n \
  --slurpfile previous "$tmpdir/append_only_runtime_rerun.json" \
  --slurpfile application "$tmpdir/runtime_application_promotion_application.json" \
  --argjson runtime_application_rerun_rust_module_present "$runtime_application_rerun_rust_module_present" \
  --argjson runtime_application_rerun_report_script_present "$runtime_application_rerun_report_script_present" \
  --argjson runtime_application_rerun_gate_script_present "$runtime_application_rerun_gate_script_present" \
  --argjson runtime_application_application_rust_module_present "$runtime_application_application_rust_module_present" \
  --argjson runtime_application_application_gate_script_present "$runtime_application_application_gate_script_present" \
  --argjson append_only_runtime_rerun_gate_script_present "$append_only_runtime_rerun_gate_script_present" \
  '
  $previous[0] as $previous
  | $application[0] as $application
  | def unique_order: reduce .[] as $item ([]; if index($item) then . else . + [$item] end);
  def push_unique($values; $value): if ($values | index($value)) then $values else ($values + [$value]) end;
  def cleared_runtime_application_blocker($id):
      ($id == "runtime_application_residuals_not_promoted") or
      ($id == "projection_adapter_runtime_closure_application_disabled") or
      ($id == "store_guard_runtime_application_disabled") or
      ($id == "terminal_task_result_runtime_application_disabled") or
      ($id == "scheduler_admission_runtime_application_disabled") or
      ($id == "role_manifest_runtime_application_disabled") or
      ($id == "runtime_application_promotion_readback_missing") or
      ($id == "runtime_application_promotion_closure_application_missing") or
      ($id == "runtime_application_promotion_readiness_rerun_missing");
  def blocker_sources($id): ($application.blockers[] | select(.id == $id) | .affected_source_surface_ids) // [];
  def source_blocker_ids($source):
      ([$application.blockers[]
        | select((.affected_source_surface_ids | index($source)) and (cleared_runtime_application_blocker(.id) | not))
        | .id]) as $ids
      | if ($ids | index("operator_review_required")) then ($ids + ["side_effect_lock_not_established"]) else $ids end;
  def runtime_decision_for($store; $timeline; $task_result; $route; $guard; $terminal_ready; $precondition_ready; $readback; $scheduler_ready; $role_ready; $runtime_enablement_ready; $runtime_application_ready; $route_blockers; $source_blockers):
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
      elif (($source_blockers | index("operator_review_required")) or ($source_blockers | index("side_effect_lock_not_established"))) then "deny_operator_review_required"
      elif (($source_blockers | index("wal_write_boundary_not_enabled")) or
            ($source_blockers | index("durable_store_runtime_switch_disabled")) or
            ($source_blockers | index("idempotency_index_mutation_disabled")) or
            ($source_blockers | index("rollback_readback_not_executed")) or
            ($source_blockers | index("readback_execution_disabled"))) then "deny_runtime_append_only_store_write_boundary_disabled"
      elif ($route_blockers | index("append_only_store_disabled_by_design")) then "deny_append_only_store_disabled"
      else "allow_preview_only"
      end;
  def next_gate_for($decision):
      if $decision == "deny_operator_review_required" then "hepta_work_graph_append_only_store_operator_review_side_effect_lock_preview_gate"
      elif $decision == "deny_runtime_append_only_store_write_boundary_disabled" then "hepta_work_graph_append_only_store_runtime_write_boundary_preview_gate"
      elif $decision == "allow_preview_only" then "hepta_work_graph_projection_enforcement_dry_run_preview_gate"
      else "hepta_work_graph_append_only_store_operator_review_side_effect_lock_preview_gate"
      end;
  def runtime_decision($decision):
      (any($application.source_outcomes[]?; .source_surface_id == $decision.source_surface_id and .runtime_application_contract_ready_preview == true and .applies_to_runtime == false)) as $covered
      | ($covered or (($decision.residual_source_blocker_ids | index("runtime_application_residuals_not_promoted")) | not)) as $runtime_application_ready
      | (($decision.append_only_store_runtime_rerun_enforcement_decision == "deny_runtime_application_residuals_not_promoted") and $runtime_application_ready) as $primary_gap_closed
      | (($decision.residual_source_blocker_ids | map(select(cleared_runtime_application_blocker(.) | not))) + source_blocker_ids($decision.source_surface_id) | unique_order) as $source_blockers
      | (runtime_decision_for($decision.unified_store_projection_ready; $decision.timeline_projection_ready; $decision.task_result_projection_ready; $decision.append_only_route_ready; $decision.store_idempotency_guard_ready; $decision.terminal_task_result_contract_ready; $decision.append_only_store_precondition_ready; $decision.readback_probe_contract_ready; $decision.scheduler_admission_contract_ready; $decision.role_manifest_contract_ready; $decision.append_only_store_runtime_enablement_ready; $runtime_application_ready; $decision.residual_route_blocker_ids; $source_blockers)) as $rerun_decision
      | {
          source_surface_id: $decision.source_surface_id,
          source_category: $decision.source_category,
          previous_append_only_store_runtime_rerun_state: $decision.append_only_store_runtime_rerun_state,
          runtime_application_promotion_rerun_state: (if $covered then "runtime_application_promotion_contract_ready_preview_after_application" else "runtime_application_promotion_not_required_for_source" end),
          covered_by_runtime_application_promotion_application_preview: $covered,
          previous_enforcement_decision: $decision.append_only_store_runtime_rerun_enforcement_decision,
          runtime_application_promotion_rerun_enforcement_decision: $rerun_decision,
          runtime_application_primary_gap_closed_by_application_preview: $primary_gap_closed,
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
          runtime_application_promotion_contract_ready: $runtime_application_ready,
          runtime_application_promoted: false,
          operator_review_ready: (($source_blockers | index("operator_review_required")) | not),
          side_effect_lock_ready: (($source_blockers | index("side_effect_lock_not_established")) | not),
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
      next_gate: "hepta_work_graph_append_only_store_operator_review_side_effect_lock_preview_gate"
    };
  def residual_sources($decisions; $ids):
      reduce $decisions[] as $decision ([]; if any($ids[]; . as $id | $decision.residual_source_blocker_ids | index($id)) then push_unique(.; $decision.source_surface_id) else . end);
  ($previous.decision_deltas | map(runtime_decision(.))) as $decisions
  | ($previous.decision_deltas | map(select(.append_only_store_runtime_rerun_enforcement_decision == "deny_runtime_application_residuals_not_promoted") | .source_surface_id)) as $before_runtime_application_gap_sources
  | ($decisions | map(select(.runtime_application_promotion_rerun_enforcement_decision == "deny_runtime_application_residuals_not_promoted") | .source_surface_id)) as $after_runtime_application_gap_sources
  | ($application.blockers | map(select(cleared_runtime_application_blocker(.id) | not) | residual_blocker(.))) as $application_residual_blockers
  | (residual_sources($decisions; ["operator_review_required"])) as $operator_review_sources
  | ($application_residual_blockers + [{
      id: "side_effect_lock_not_established",
      severity: "critical",
      affected_source_surface_ids: $operator_review_sources,
      required_before_projection_enforcement: true,
      recommended_fix: "establish a no-mutation side-effect lock before operator approval or runtime promotion can proceed"
    }]) as $residual_blockers
  | (residual_sources($decisions; ["wal_write_boundary_not_enabled"])) as $wal_sources
  | (residual_sources($decisions; ["durable_store_runtime_switch_disabled"])) as $durable_sources
  | (residual_sources($decisions; ["idempotency_index_mutation_disabled"])) as $idempotency_sources
  | (residual_sources($decisions; ["readback_execution_disabled","rollback_readback_not_executed"])) as $rollback_sources
  | ([
      stage("runtime_application_promotion_contracts"; ($decisions | length); 0; ($decisions | map(select(.runtime_application_promotion_contract_ready)) | length); ["operator_review_required","side_effect_lock_not_established","wal_write_boundary_not_enabled"]),
      stage("operator_review_side_effect_lock"; ($operator_review_sources | length); 0; 0; ["operator_review_required","side_effect_lock_not_established"]),
      stage("durable_store_runtime_switch"; ($durable_sources | length); 0; 0; ["durable_store_runtime_switch_disabled"]),
      stage("wal_write_boundary"; ($wal_sources | length); 0; 0; ["wal_write_boundary_not_enabled"]),
      stage("idempotency_mutation_policy"; ($idempotency_sources | length); 0; 0; ["idempotency_index_mutation_disabled"]),
      stage("rollback_readback_execution_gate"; ($rollback_sources | length); 0; 0; ["readback_execution_disabled","rollback_readback_not_executed"]),
      stage("projection_enforcement_dry_run"; ($decisions | length); 0; 0; ["operator_review_required","side_effect_lock_not_established","wal_write_boundary_not_enabled","durable_store_runtime_switch_disabled","idempotency_index_mutation_disabled","rollback_readback_not_executed"])
    ]) as $stages
  | ($application.required_prior_gates + [$application.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview_gate",
      schema_version: "work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview_v1",
      preview_mode: "read_only_projection_enforcement_readiness_runtime_application_promotion_rerun_no_enforcement",
      source_surface_count: ($decisions | length),
      runtime_application_promotion_outcome_count: ($application.source_outcomes | length),
      previous_contract_ready_surface_count: ($previous.decision_deltas | map(select(.projection_contract_ready)) | length),
      runtime_application_rerun_contract_ready_surface_count: ($decisions | map(select(.projection_contract_ready)) | length),
      previous_runtime_application_primary_blocked_surface_count: ($before_runtime_application_gap_sources | length),
      runtime_application_primary_blocked_surface_count_after: ($after_runtime_application_gap_sources | length),
      runtime_application_contract_ready_surface_count: ($decisions | map(select(.runtime_application_promotion_contract_ready)) | length),
      runtime_application_promoted_surface_count: ($decisions | map(select(.runtime_application_promoted)) | length),
      operator_review_residual_source_count: ($operator_review_sources | length),
      side_effect_lock_residual_source_count: (residual_sources($decisions; ["side_effect_lock_not_established"]) | length),
      wal_boundary_residual_source_count: ($wal_sources | length),
      write_boundary_primary_blocked_surface_count: ($decisions | map(select(.runtime_application_promotion_rerun_enforcement_decision == "deny_runtime_append_only_store_write_boundary_disabled")) | length),
      rerun_ready_surface_count: ($decisions | map(select(.runtime_application_promotion_rerun_enforcement_decision == "allow_preview_only")) | length),
      rerun_blocked_surface_count: ($decisions | map(select(.runtime_application_promotion_rerun_enforcement_decision != "allow_preview_only")) | length),
      decision_delta_count: ($decisions | length),
      cleared_blocker_count: 1,
      residual_blocker_count: ($residual_blockers | length),
      enforcement_stage_count: ($stages | length),
      required_prior_gate_count: ($required_prior_gates | length),
      decision_deltas: $decisions,
      cleared_blockers: [{
        id: "runtime_application_residuals_not_promoted_for_enforcement",
        cleared_source_surface_ids: $before_runtime_application_gap_sources,
        source_count_before: ($before_runtime_application_gap_sources | length),
        source_count_after: ($after_runtime_application_gap_sources | length),
        closure_gate_id: "hepta_work_graph_runtime_application_promotion_gap_closure_application_preview_gate"
      }],
      residual_blockers: $residual_blockers,
      enforcement_stages: $stages,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_append_only_store_operator_review_side_effect_lock_preview_gate",
      ready_for_operator_review_side_effect_lock_preview: true,
      ready_for_runtime_write_boundary_preview: false,
      ready_for_append_only_store_enablement: false,
      ready_for_projection_enforcement: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        runtime_application_promotion_readiness_rerun: {
          rust_module_present: $runtime_application_rerun_rust_module_present,
          report_script_present: $runtime_application_rerun_report_script_present,
          gate_script_present: $runtime_application_rerun_gate_script_present
        },
        runtime_application_promotion_application: {
          rust_module_present: $runtime_application_application_rust_module_present,
          gate_script_present: $runtime_application_application_gate_script_present,
          upstream_gate: ($application.gate == "hepta_work_graph_runtime_application_promotion_gap_closure_application_preview_gate")
        },
        append_only_store_runtime_readiness_rerun: {
          upstream_gate: ($previous.gate == "hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview_gate"),
          gate_script_present: $append_only_runtime_rerun_gate_script_present
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
