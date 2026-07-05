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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-role-manifest-gap-closure-readback.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-role-manifest-enforcement-gap-closure-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-role-manifest-enforcement-gap-closure-preview-report.sh" \
  >"$tmpdir/closure.json"

readback_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_role_manifest_enforcement_gap_closure_readback_preview.rs
)"
readback_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-role-manifest-enforcement-gap-closure-readback-preview-report.sh
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-role-manifest-enforcement-gap-closure-readback-preview-gate.sh
)"
closure_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-role-manifest-enforcement-gap-closure-preview-gate.sh
)"

jq -n \
  --slurpfile closure "$tmpdir/closure.json" \
  --argjson readback_rust_module_present "$readback_rust_module_present" \
  --argjson readback_report_script_present "$readback_report_script_present" \
  --argjson readback_gate_script_present "$readback_gate_script_present" \
  --argjson closure_gate_script_present "$closure_gate_script_present" \
  '
  $closure[0] as $closure
  | def readback_id($source): "readback_" + $source + "_role_manifest_gap_closure";
  def capability_assertion_id($source): "assert_" + $source + "_role_capability_binding_readback";
  def permission_assertion_id($source): "assert_" + $source + "_role_tool_permission_readback";
  def budget_lane_assertion_id($source): "assert_" + $source + "_role_budget_lane_readback";
  def termination_output_assertion_id($source): "assert_" + $source + "_role_termination_output_readback";
  def guard_assertion_id($guard): "assert_" + $guard + "_readback";
  def blocker_assertion_id($blocker): "assert_" + $blocker + "_mapping_readback";
  def plan_readback($plan): {
      id: readback_id($plan.source_surface_id),
      source_surface_id: $plan.source_surface_id,
      closure_plan_id: $plan.closure_plan_id,
      source_category: $plan.source_category,
      projected_role_kind: $plan.projected_role_kind,
      role_blocker_id: $plan.role_blocker_id,
      covered_wire_fields: $plan.covered_wire_fields,
      capability_ids: $plan.capability_ids,
      tool_permission_mode_ids: $plan.tool_permission_mode_ids,
      role_binding_ids: $plan.role_binding_ids,
      readback_probe_id: $plan.readback_probe_id,
      required_before_closure_application: true,
      readback_state: "readback_assertions_defined_execution_disabled",
      performs_readback: false,
      enforces_role_manifest: false,
      changes_tool_permissions: false,
      consumes_budget: false,
      mutates_lane_binding: false,
      starts_work: false,
      spawns_agent: false,
      writes_store: false
    };
  def capability_assertion($plan): {
      id: capability_assertion_id($plan.source_surface_id),
      source_surface_id: $plan.source_surface_id,
      closure_plan_id: $plan.closure_plan_id,
      capability_binding_id: $plan.capability_binding_id,
      capability_ids: $plan.capability_ids,
      expected_binding_state: "role_capability_binding_defined_enforcement_disabled",
      enforces_role_manifest: false,
      changes_tool_permissions: false
    };
  def permission_assertion($plan): {
      id: permission_assertion_id($plan.source_surface_id),
      source_surface_id: $plan.source_surface_id,
      closure_plan_id: $plan.closure_plan_id,
      tool_permission_binding_id: $plan.tool_permission_binding_id,
      tool_permission_mode_ids: $plan.tool_permission_mode_ids,
      expected_permission_state: "role_tool_permission_defined_no_permission_change",
      changes_tool_permissions: false,
      mutates_runtime: false
    };
  def budget_lane_assertion($plan): {
      id: budget_lane_assertion_id($plan.source_surface_id),
      source_surface_id: $plan.source_surface_id,
      closure_plan_id: $plan.closure_plan_id,
      budget_binding_id: $plan.budget_binding_id,
      lane_binding_id: $plan.lane_binding_id,
      expected_budget_lane_state: "role_budget_lane_binding_defined_no_budget_or_lane_mutation",
      consumes_budget: false,
      mutates_lane_binding: false
    };
  def termination_output_assertion($plan): {
      id: termination_output_assertion_id($plan.source_surface_id),
      source_surface_id: $plan.source_surface_id,
      closure_plan_id: $plan.closure_plan_id,
      termination_binding_id: $plan.termination_binding_id,
      output_schema_binding_id: $plan.output_schema_binding_id,
      output_schema_ref_declared: ($plan.covered_wire_fields | index("outputSchemaRef") != null),
      verifier_ref_declared: ($plan.covered_wire_fields | index("verifierRef") != null),
      expected_terminal_contract_state: "role_terminal_contract_defined_no_work_or_agent_spawn",
      starts_work: false,
      spawns_agent: false
    };
  def guard_assertion($guard): {
      id: guard_assertion_id($guard.id),
      guard_id: $guard.id,
      severity: $guard.severity,
      guard_scope: $guard.guard_scope,
      expected_guard_state: "guard_declared_satisfied_by_runtime_false",
      required_before_role_manifest_enforcement: $guard.required_before_role_manifest_enforcement,
      satisfied_by_readback_preview: false,
      mutates_runtime: false
    };
  def blocker_assertion($blocker): {
      id: blocker_assertion_id($blocker.id),
      blocker_id: $blocker.id,
      category: $blocker.category,
      severity: $blocker.severity,
      affected_source_surface_ids: $blocker.affected_source_surface_ids,
      affected_closure_plan_ids: $blocker.affected_closure_plan_ids,
      expected_blocker_state: "blocks_role_manifest_until_readback_and_application_preview",
      required_before_role_manifest_enforcement: $blocker.required_before_role_manifest_enforcement,
      performs_readback: false,
      mutates_runtime: false
    };
  def drift($id; $fields; $severity): {
      id: $id,
      compared_field_ids: $fields,
      severity: $severity,
      blocks_closure_application: true,
      performs_readback: false
    };
  def readback_blocker($id; $severity; $category; $sources; $readback_ids; $fix): {
      id: $id,
      severity: $severity,
      category: $category,
      affected_source_surface_ids: $sources,
      affected_readback_plan_ids: $readback_ids,
      required_before_role_manifest_enforcement: true,
      recommended_fix: $fix
    };
  def readback_blocker_from_closure($blocker):
      readback_blocker(
        $blocker.id;
        $blocker.severity;
        $blocker.category;
        $blocker.affected_source_surface_ids;
        ($blocker.affected_source_surface_ids | map(readback_id(.)));
        $blocker.recommended_fix
      );
  ($closure.closure_plans | map(plan_readback(.))) as $readback_plans
  | ($closure.closure_plans | map(capability_assertion(.))) as $capability_assertions
  | ($closure.closure_plans | map(permission_assertion(.))) as $permission_assertions
  | ($closure.closure_plans | map(budget_lane_assertion(.))) as $budget_lane_assertions
  | ($closure.closure_plans | map(termination_output_assertion(.))) as $termination_output_assertions
  | ($closure.guards | map(guard_assertion(.))) as $guard_assertions
  | ($closure.blockers | map(blocker_assertion(.))) as $blocker_assertions
  | [
      drift("role_manifest_source_coverage_drift"; ["sourceSurfaceId", "closurePlanId", "projectedRoleKind"]; "critical"),
      drift("role_manifest_capability_binding_drift"; ["capabilityIds", "capabilityBindingId"]; "critical"),
      drift("role_manifest_tool_permission_binding_drift"; ["toolPermissionModeIds", "toolPermissionBindingId"]; "critical"),
      drift("role_manifest_budget_lane_binding_drift"; ["budgetBindingId", "laneBindingId"]; "high"),
      drift("role_manifest_termination_output_binding_drift"; ["terminationBindingId", "outputSchemaBindingId", "verifierRef"]; "high"),
      drift("role_manifest_no_mutation_blocker_mapping_drift"; ["performsReadback", "changesToolPermissions", "consumesBudget", "startsWork", "spawnsAgent", "blockerId"]; "critical")
    ] as $drift_detectors
  | ($readback_plans | map(.source_surface_id)) as $all_sources
  | ($readback_plans | map(.id)) as $all_readback_ids
  | ([readback_blocker("readback_execution_disabled"; "critical"; "readback_execution"; $all_sources; $all_readback_ids; "this preview defines role manifest readback assertions but does not execute readback")]
      + ($closure.blockers | map(select(.id != "role_manifest_closure_readback_missing") | readback_blocker_from_closure(.)))
      + [readback_blocker("role_manifest_closure_application_missing"; "high"; "application_preview"; $all_sources; $all_readback_ids; "run closure application preview after role manifest readback assertions are defined and reviewed")]) as $readback_blockers
  | ($closure.required_prior_gates + (if ($closure.required_prior_gates | index($closure.gate)) then [] else [$closure.gate] end)) as $required_priors
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_role_manifest_enforcement_gap_closure_readback_preview_gate",
      schema_version: "work_graph_role_manifest_enforcement_gap_closure_readback_preview_v1",
      preview_mode: "read_only_role_manifest_gap_closure_readback_no_execution",
      closure_plan_count: $closure.closure_plan_count,
      readback_plan_count: ($readback_plans | length),
      capability_binding_assertion_count: ($capability_assertions | length),
      tool_permission_assertion_count: ($permission_assertions | length),
      budget_lane_assertion_count: ($budget_lane_assertions | length),
      termination_output_assertion_count: ($termination_output_assertions | length),
      guard_assertion_count: ($guard_assertions | length),
      blocker_mapping_assertion_count: ($blocker_assertions | length),
      role_binding_ref_count: ($readback_plans | map(.role_binding_ids | length) | add),
      capability_ref_count: ($readback_plans | map(.capability_ids | length) | add),
      permission_mode_ref_count: ($readback_plans | map(.tool_permission_mode_ids | length) | add),
      manifest_field_ref_count: ($readback_plans | map(.covered_wire_fields | length) | add),
      drift_detector_count: ($drift_detectors | length),
      blocker_count: ($readback_blockers | length),
      required_prior_gate_count: ($required_priors | length),
      readback_plans: $readback_plans,
      capability_binding_assertions: $capability_assertions,
      tool_permission_assertions: $permission_assertions,
      budget_lane_assertions: $budget_lane_assertions,
      termination_output_assertions: $termination_output_assertions,
      guard_assertions: $guard_assertions,
      blocker_mapping_assertions: $blocker_assertions,
      drift_detectors: $drift_detectors,
      blockers: $readback_blockers,
      required_prior_gates: $required_priors,
      recommended_next_gate: "hepta_work_graph_role_manifest_enforcement_gap_closure_application_preview_gate",
      ready_for_role_manifest_closure_application_preview: true,
      ready_for_readback_execution: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_append_only_store_enablement: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_projection_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        role_manifest_gap_closure_readback: {
          rust_module_present: $readback_rust_module_present,
          report_script_present: $readback_report_script_present,
          gate_script_present: $readback_gate_script_present
        },
        role_manifest_gap_closure: {
          upstream_gate: ($closure.gate == "hepta_work_graph_role_manifest_enforcement_gap_closure_preview_gate"),
          gate_script_present: $closure_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        wal_written: false,
        checkpoint_written: false,
        readback_performed: false,
        role_manifest_enforced: false,
        tool_permission_changed: false,
        budget_consumed: false,
        lane_binding_mutated: false,
        work_started: false,
        agent_spawned: false,
        scheduler_admission_enforced: false,
        append_only_store_enabled: false,
        task_result_enforcement_enabled: false,
        projection_enforcement_enabled: false,
        runtime_mutation_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
