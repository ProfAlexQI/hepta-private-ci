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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-enforcement-readiness-append-only-runtime-rerun.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-unified-projection-enforcement-readiness-role-manifest-rerun-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-role-manifest-rerun-preview-report.sh" \
  >"$tmpdir/role_manifest_rerun.json"
capture_json_report \
  "hepta-work-graph-append-only-store-runtime-enablement-application-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-append-only-store-runtime-enablement-application-preview-report.sh" \
  >"$tmpdir/runtime_application.json"

runtime_rerun_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview.rs
)"
runtime_rerun_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-append-only-store-runtime-rerun-preview-report.sh
)"
runtime_rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-append-only-store-runtime-rerun-preview-gate.sh
)"
runtime_application_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_store_runtime_enablement_application_preview.rs
)"
runtime_application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-runtime-enablement-application-preview-gate.sh
)"
role_manifest_rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-role-manifest-rerun-preview-gate.sh
)"

jq -n \
  --slurpfile role "$tmpdir/role_manifest_rerun.json" \
  --slurpfile application "$tmpdir/runtime_application.json" \
  --argjson runtime_rerun_rust_module_present "$runtime_rerun_rust_module_present" \
  --argjson runtime_rerun_report_script_present "$runtime_rerun_report_script_present" \
  --argjson runtime_rerun_gate_script_present "$runtime_rerun_gate_script_present" \
  --argjson runtime_application_rust_module_present "$runtime_application_rust_module_present" \
  --argjson runtime_application_gate_script_present "$runtime_application_gate_script_present" \
  --argjson role_manifest_rerun_gate_script_present "$role_manifest_rerun_gate_script_present" \
  '
  $role[0] as $role
  | $application[0] as $application
  | def unique_order: reduce .[] as $item ([]; if index($item) then . else . + [$item] end);
  def push_unique($values; $value): if ($values | index($value)) then $values else ($values + [$value]) end;
  def union_sources($left; $right): reduce $right[] as $item ($left; push_unique(.; $item));
  def blocker_sources($id): ($application.blockers[] | select(.id == $id) | .affected_source_surface_ids) // [];
  def source_blocker_ids($source):
      [$application.blockers[]
        | select((.affected_source_surface_ids | index($source)) and
                 (.id != "append_only_store_runtime_enablement_disabled") and
                 (.id != "append_only_store_runtime_enablement_readback_missing") and
                 (.id != "append_only_store_runtime_readiness_rerun_missing"))
        | .id];
  def runtime_decision_for($store; $timeline; $task_result; $route; $guard; $terminal_ready; $precondition_ready; $readback; $scheduler_ready; $role_ready; $runtime_ready; $route_blockers; $source_blockers):
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
      elif (($runtime_ready | not) or ($source_blockers | index("append_only_store_runtime_enablement_disabled"))) then "deny_runtime_append_only_store_enablement_disabled"
      elif ($source_blockers | index("runtime_application_residuals_not_promoted")) then "deny_runtime_application_residuals_not_promoted"
      elif ($source_blockers | index("operator_review_required")) then "deny_operator_review_required"
      elif (($source_blockers | index("wal_write_boundary_not_enabled")) or
            ($source_blockers | index("durable_store_runtime_switch_disabled")) or
            ($source_blockers | index("idempotency_index_mutation_disabled")) or
            ($source_blockers | index("rollback_readback_not_executed")) or
            ($source_blockers | index("readback_execution_disabled"))) then "deny_runtime_append_only_store_write_boundary_disabled"
      elif ($route_blockers | index("append_only_store_disabled_by_design")) then "deny_append_only_store_disabled"
      else "allow_preview_only"
      end;
  def next_gate_for($decision):
      if $decision == "deny_runtime_application_residuals_not_promoted" then "hepta_work_graph_runtime_application_promotion_gap_closure_preview_gate"
      elif $decision == "deny_operator_review_required" then "hepta_work_graph_append_only_store_operator_review_side_effect_lock_preview_gate"
      elif $decision == "deny_runtime_append_only_store_write_boundary_disabled" then "hepta_work_graph_append_only_store_runtime_write_boundary_preview_gate"
      elif $decision == "allow_preview_only" then "hepta_work_graph_projection_enforcement_dry_run_preview_gate"
      else "hepta_work_graph_runtime_application_promotion_gap_closure_preview_gate"
      end;
  def runtime_decision($decision):
      (any($application.source_outcomes[]?; .source_surface_id == $decision.source_surface_id and .runtime_enablement_contract_ready_preview == true and .applies_to_runtime == false)) as $covered
      | ($covered or (($decision.residual_source_blocker_ids | index("append_only_store_runtime_enablement_disabled")) | not)) as $runtime_ready
      | (($decision.role_manifest_rerun_enforcement_decision == "deny_runtime_append_only_store_enablement_disabled") and $runtime_ready) as $primary_gap_closed
      | (($decision.residual_source_blocker_ids | map(select(. != "append_only_store_runtime_enablement_disabled"))) + source_blocker_ids($decision.source_surface_id) | unique_order) as $source_blockers
      | (runtime_decision_for($decision.unified_store_projection_ready; $decision.timeline_projection_ready; $decision.task_result_projection_ready; $decision.append_only_route_ready; $decision.store_idempotency_guard_ready; $decision.terminal_task_result_contract_ready; $decision.append_only_store_precondition_ready; $decision.readback_probe_contract_ready; $decision.scheduler_admission_contract_ready; $decision.role_manifest_contract_ready; $runtime_ready; $decision.residual_route_blocker_ids; $source_blockers)) as $rerun_decision
      | {
          source_surface_id: $decision.source_surface_id,
          source_category: $decision.source_category,
          previous_role_manifest_rerun_state: $decision.role_manifest_rerun_state,
          append_only_store_runtime_rerun_state: (if $covered then "runtime_enablement_contract_ready_preview_after_application" else "runtime_enablement_not_required_for_source" end),
          covered_by_runtime_enablement_application_preview: $covered,
          previous_enforcement_decision: $decision.role_manifest_rerun_enforcement_decision,
          append_only_store_runtime_rerun_enforcement_decision: $rerun_decision,
          runtime_append_only_primary_gap_closed_by_application_preview: $primary_gap_closed,
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
          append_only_store_runtime_enablement_ready: $runtime_ready,
          runtime_application_promotion_ready: (($source_blockers | index("runtime_application_residuals_not_promoted")) | not),
          operator_review_ready: (($source_blockers | index("operator_review_required")) | not),
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
      next_gate: "hepta_work_graph_runtime_application_promotion_gap_closure_preview_gate"
    };
  def stage_blockers($id):
      if $id == "durable_store_runtime_switch" then ["durable_store_runtime_switch_disabled"]
      elif $id == "wal_write_boundary" then ["wal_write_boundary_not_enabled"]
      elif $id == "idempotency_mutation_policy" then ["idempotency_index_mutation_disabled"]
      elif $id == "rollback_readback_execution_gate" then ["readback_execution_disabled","rollback_readback_not_executed"]
      elif $id == "operator_review_side_effect_lock" then ["operator_review_required"]
      elif $id == "runtime_application_promotion" then ["runtime_application_residuals_not_promoted"]
      else []
      end;
  ($role.decision_deltas | map(runtime_decision(.))) as $decisions
  | ($role.decision_deltas | map(select(.role_manifest_rerun_enforcement_decision == "deny_runtime_append_only_store_enablement_disabled") | .source_surface_id)) as $before_runtime_gap_sources
  | ($decisions | map(select(.append_only_store_runtime_rerun_enforcement_decision == "deny_runtime_append_only_store_enablement_disabled") | .source_surface_id)) as $after_runtime_gap_sources
  | ($application.blockers | map(select((.id != "append_only_store_runtime_enablement_disabled") and (.id != "append_only_store_runtime_enablement_readback_missing") and (.id != "append_only_store_runtime_readiness_rerun_missing")) | residual_blocker(.))) as $residual_blockers
  | (union_sources(union_sources(blocker_sources("projection_adapter_runtime_closure_application_disabled"); blocker_sources("store_guard_runtime_application_disabled")); blocker_sources("terminal_task_result_runtime_application_disabled"))) as $projection_store_task_result_sources
  | (union_sources(blocker_sources("scheduler_admission_runtime_application_disabled"); blocker_sources("role_manifest_runtime_application_disabled"))) as $scheduler_role_sources
  | ([
      stage("append_only_store_runtime_enablement_contracts"; ($decisions | length); 0; ($application.source_outcomes | map(select(.runtime_enablement_contract_ready_preview)) | length); ["durable_store_runtime_switch_disabled","wal_write_boundary_not_enabled","idempotency_index_mutation_disabled","rollback_readback_not_executed"])
    ] + ($application.stage_applications | map(stage(.runtime_stage_id; (.affected_source_surface_ids | length); 0; (.affected_source_surface_ids | length); stage_blockers(.runtime_stage_id)))) + [
      stage("projection_store_task_result_runtime_application"; ($projection_store_task_result_sources | length); 0; 0; ["projection_adapter_runtime_closure_application_disabled","store_guard_runtime_application_disabled","terminal_task_result_runtime_application_disabled"]),
      stage("scheduler_role_runtime_application"; ($scheduler_role_sources | length); 0; 0; ["scheduler_admission_runtime_application_disabled","role_manifest_runtime_application_disabled"])
    ]) as $stages
  | ($application.required_prior_gates + [$application.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview_gate",
      schema_version: "work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview_v1",
      preview_mode: "read_only_projection_enforcement_readiness_append_only_store_runtime_rerun_no_enforcement",
      source_surface_count: ($decisions | length),
      runtime_application_outcome_count: ($application.source_outcomes | length),
      previous_contract_ready_surface_count: ($role.decision_deltas | map(select(.projection_contract_ready)) | length),
      runtime_rerun_contract_ready_surface_count: ($decisions | map(select(.projection_contract_ready)) | length),
      previous_runtime_append_only_primary_blocked_surface_count: ($before_runtime_gap_sources | length),
      runtime_append_only_primary_blocked_surface_count_after: ($after_runtime_gap_sources | length),
      runtime_enablement_contract_ready_surface_count: ($decisions | map(select(.append_only_store_runtime_enablement_ready)) | length),
      runtime_append_only_store_enabled_surface_count: ($decisions | map(select(.runtime_append_only_store_enabled)) | length),
      runtime_application_residual_source_count: ($decisions | map(select(.residual_source_blocker_ids | index("runtime_application_residuals_not_promoted"))) | length),
      operator_review_residual_source_count: ($decisions | map(select(.residual_source_blocker_ids | index("operator_review_required"))) | length),
      wal_boundary_residual_source_count: ($decisions | map(select(.residual_source_blocker_ids | index("wal_write_boundary_not_enabled"))) | length),
      rerun_ready_surface_count: ($decisions | map(select(.append_only_store_runtime_rerun_enforcement_decision == "allow_preview_only")) | length),
      rerun_blocked_surface_count: ($decisions | map(select(.append_only_store_runtime_rerun_enforcement_decision != "allow_preview_only")) | length),
      decision_delta_count: ($decisions | length),
      cleared_blocker_count: 1,
      residual_blocker_count: ($residual_blockers | length),
      enforcement_stage_count: ($stages | length),
      required_prior_gate_count: ($required_prior_gates | length),
      decision_deltas: $decisions,
      cleared_blockers: [{
        id: "append_only_store_runtime_enablement_disabled_for_enforcement",
        cleared_source_surface_ids: $before_runtime_gap_sources,
        source_count_before: ($before_runtime_gap_sources | length),
        source_count_after: ($after_runtime_gap_sources | length),
        closure_gate_id: "hepta_work_graph_append_only_store_runtime_enablement_application_preview_gate"
      }],
      residual_blockers: $residual_blockers,
      enforcement_stages: $stages,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_runtime_application_promotion_gap_closure_preview_gate",
      ready_for_runtime_application_promotion_preview: true,
      ready_for_append_only_store_enablement: false,
      ready_for_projection_enforcement: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        append_only_store_runtime_readiness_rerun: {
          rust_module_present: $runtime_rerun_rust_module_present,
          report_script_present: $runtime_rerun_report_script_present,
          gate_script_present: $runtime_rerun_gate_script_present
        },
        append_only_store_runtime_application: {
          rust_module_present: $runtime_application_rust_module_present,
          gate_script_present: $runtime_application_gate_script_present,
          upstream_gate: ($application.gate == "hepta_work_graph_append_only_store_runtime_enablement_application_preview_gate")
        },
        role_manifest_readiness_rerun: {
          upstream_gate: ($role.gate == "hepta_work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_preview_gate"),
          gate_script_present: $role_manifest_rerun_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        wal_written: false,
        checkpoint_written: false,
        idempotency_index_mutated: false,
        append_only_store_enabled: false,
        projection_enforcement_enabled: false,
        scheduler_admission_enforced: false,
        lane_lease_acquired: false,
        work_started: false,
        budget_consumed: false,
        approval_recorded: false,
        task_result_enforcement_enabled: false,
        task_result_persisted: false,
        role_manifest_enforcement_enabled: false,
        tool_permission_changed: false,
        role_budget_consumed: false,
        role_lane_binding_mutated: false,
        readback_executed: false,
        rollback_executed: false,
        runtime_application_promoted: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }
  '
