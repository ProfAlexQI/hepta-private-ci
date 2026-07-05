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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-role-manifest-gap-closure.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-unified-projection-enforcement-readiness-scheduler-admission-rerun-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-scheduler-admission-rerun-preview-report.sh" \
  >"$tmpdir/scheduler_rerun.json"
capture_json_report \
  "hepta-work-graph-role-manifest-contract-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-role-manifest-contract-preview-report.sh" \
  >"$tmpdir/role_manifest_contract.json"

role_gap_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_role_manifest_enforcement_gap_closure_preview.rs
)"
role_gap_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-role-manifest-enforcement-gap-closure-preview-report.sh
)"
role_gap_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-role-manifest-enforcement-gap-closure-preview-gate.sh
)"
role_contract_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_role_manifest_contract.rs
)"
role_contract_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-role-manifest-contract-preview-gate.sh
)"
scheduler_rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-scheduler-admission-rerun-preview-gate.sh
)"

jq -n \
  --slurpfile rerun "$tmpdir/scheduler_rerun.json" \
  --slurpfile contract "$tmpdir/role_manifest_contract.json" \
  --argjson role_gap_rust_module_present "$role_gap_rust_module_present" \
  --argjson role_gap_report_script_present "$role_gap_report_script_present" \
  --argjson role_gap_gate_script_present "$role_gap_gate_script_present" \
  --argjson role_contract_rust_module_present "$role_contract_rust_module_present" \
  --argjson role_contract_gate_script_present "$role_contract_gate_script_present" \
  --argjson scheduler_rerun_gate_script_present "$scheduler_rerun_gate_script_present" \
  '
  $rerun[0] as $rerun
  | $contract[0] as $contract
  | def unique_order: reduce .[] as $item ([]; if index($item) then . else . + [$item] end);
  def has_contains($values; $needle): any($values[]?; contains($needle));
  def adapter_for($source): [$contract.adapter_previews[] | select(.source_surface_id == $source)][0];
  def role_blocker_for($decision): [$decision.residual_source_blocker_ids[] | select(contains("role_manifest_not_enforced"))][0];
  def capability_ids_for($role_kind):
      if $role_kind == "agent_task_role" then ["planning", "agent_delegation", "research", "verification"]
      elif $role_kind == "batch_worker_role" then ["agent_delegation", "code_editing", "verification"]
      elif $role_kind == "runtime_worker_role" then ["code_editing", "verification", "scheduler_control"]
      elif $role_kind == "external_handoff_role" then ["external_handoff_proposal", "research", "verification"]
      else error("unknown projected role kind: " + $role_kind)
      end;
  def permission_mode_ids_for($role_kind):
      if $role_kind == "runtime_worker_role" then ["read_only", "write_scoped", "approval_required"]
      else ["preview", "read_only", "approval_required"]
      end;
  def binding_ids_for($source): [
      "role_capability_binding:" + $source,
      "role_tool_permission_binding:" + $source,
      "role_budget_binding:" + $source,
      "role_lane_binding:" + $source,
      "role_termination_binding:" + $source,
      "role_output_schema_binding:" + $source
    ];
  def plan_for($decision):
      (adapter_for($decision.source_surface_id)) as $adapter
      | {
          closure_plan_id: ("role_manifest_closure_plan:" + $decision.source_surface_id),
          source_surface_id: $decision.source_surface_id,
          source_category: $decision.source_category,
          projected_role_kind: $adapter.projected_role_kind,
          role_blocker_id: role_blocker_for($decision),
          covered_wire_fields: $adapter.covered_wire_fields,
          capability_ids: capability_ids_for($adapter.projected_role_kind),
          tool_permission_mode_ids: permission_mode_ids_for($adapter.projected_role_kind),
          role_binding_ids: binding_ids_for($decision.source_surface_id),
          capability_binding_id: ("role_capability_binding:" + $decision.source_surface_id),
          tool_permission_binding_id: ("role_tool_permission_binding:" + $decision.source_surface_id),
          budget_binding_id: ("role_budget_binding:" + $decision.source_surface_id),
          lane_binding_id: ("role_lane_binding:" + $decision.source_surface_id),
          termination_binding_id: ("role_termination_binding:" + $decision.source_surface_id),
          output_schema_binding_id: ("role_output_schema_binding:" + $decision.source_surface_id),
          readback_probe_id: ("role_manifest_readback_probe:" + $decision.source_surface_id),
          closure_scope: "role_manifest_contract_preview_only",
          closure_state: "role_manifest_contract_ready_preview",
          ready_for_readback_preview: true,
          applies_to_runtime: false,
          enforces_role_manifest: false,
          changes_tool_permissions: false,
          consumes_budget: false,
          starts_work: false,
          spawns_agent: false,
          writes_store: false
        };
  def group($id; $priority; $binding_type; $plans): {
      id: $id,
      priority: $priority,
      binding_type: $binding_type,
      closure_plan_ids: ($plans | map(.closure_plan_id)),
      source_surface_ids: ($plans | map(.source_surface_id)),
      mutates_runtime: false,
      enforces_role_manifest: false
    };
  def guard($id; $severity; $scope): {
      id: $id,
      severity: $severity,
      guard_scope: $scope,
      required_before_role_manifest_enforcement: true,
      satisfied_by_preview: false
    };
  def blocker($id; $severity; $category; $sources; $plans; $fix): {
      id: $id,
      severity: $severity,
      category: $category,
      affected_source_surface_ids: $sources,
      affected_closure_plan_ids: $plans,
      required_before_role_manifest_enforcement: true,
      recommended_fix: $fix
    };
  ($rerun.decision_deltas | map(select(.scheduler_admission_rerun_enforcement_decision == "deny_role_manifest_not_enforced"))) as $role_sources
  | ($role_sources | map(plan_for(.))) as $plans
  | ($plans | map(.source_surface_id)) as $all_sources
  | ($plans | map(.closure_plan_id)) as $all_plan_ids
  | ($role_sources | map(select(has_contains(.residual_source_blocker_ids; "scheduler_admission_runtime_application_disabled")) | .source_surface_id)) as $scheduler_sources
  | ($plans | map(select(.source_surface_id as $source | $scheduler_sources | index($source)) | .closure_plan_id)) as $scheduler_plan_ids
  | ($role_sources | map(select(has_contains(.residual_source_blocker_ids; "store_projection_not_enforced") or has_contains(.residual_source_blocker_ids; "timeline_adapter_not_enforced")) | .source_surface_id)) as $projection_timeline_sources
  | ($plans | map(select(.source_surface_id as $source | $projection_timeline_sources | index($source)) | .closure_plan_id)) as $projection_timeline_plan_ids
  | [
      group("role_capability_binding_closure"; "p0"; "capability"; $plans),
      group("tool_permission_binding_closure"; "p0"; "tool_permission"; $plans),
      group("budget_lane_concurrency_binding_closure"; "p0"; "budget_lane_concurrency"; $plans),
      group("termination_output_schema_binding_closure"; "p0"; "termination_output_schema"; $plans),
      group("trace_approval_readback_binding_closure"; "p0"; "trace_approval_readback"; $plans)
    ] as $groups
  | [
      guard("role_manifest_closure_is_preview_only"; "medium"; "closure_preview"),
      guard("role_contract_adapter_required"; "high"; "role_contract"),
      guard("capability_permission_binding_required"; "high"; "capability_permission"),
      guard("output_schema_verifier_required"; "high"; "output_schema"),
      guard("budget_concurrency_lane_required"; "high"; "budget_lane"),
      guard("trace_policy_required"; "high"; "trace_policy"),
      guard("role_manifest_not_enforced"; "critical"; "role_manifest"),
      guard("tool_permissions_not_changed"; "critical"; "tool_permission"),
      guard("scheduler_admission_runtime_not_applied"; "high"; "scheduler_admission"),
      guard("append_only_store_runtime_not_enabled"; "critical"; "append_only_store")
    ] as $guards
  | [
      blocker("role_manifest_enforcement_disabled"; "critical"; "role_manifest"; $all_sources; $all_plan_ids; "keep role manifest enforcement preview-only until readback, application, and operator-review gates are promoted"),
      blocker("role_capability_binding_not_enforced"; "high"; "capability"; $all_sources; $all_plan_ids; "bind each role source to declared capabilities before role admission can be authoritative"),
      blocker("tool_permission_binding_not_enforced"; "critical"; "tool_permission"; $all_sources; $all_plan_ids; "do not change or enforce tool permission modes from this preview"),
      blocker("budget_lane_concurrency_not_enforced"; "high"; "budget_lane"; $all_sources; $all_plan_ids; "role budgets, concurrency, and lane bindings remain contract-only until runtime promotion"),
      blocker("termination_output_schema_not_enforced"; "high"; "termination_output"; $all_sources; $all_plan_ids; "terminal output schema, verifier, and termination contracts need readback before enforcement"),
      blocker("role_manifest_closure_readback_missing"; "high"; "readback"; $all_sources; $all_plan_ids; "next gate must read back role capability, tool permission, budget, lane, termination, and output-schema bindings"),
      blocker("scheduler_admission_runtime_application_disabled"; "high"; "scheduler_admission"; $scheduler_sources; $scheduler_plan_ids; "role manifests must stay separate from scheduler runtime application until lease, approval, and budget gates are promoted"),
      blocker("projection_timeline_runtime_residuals_not_promoted"; "high"; "projection_timeline"; $projection_timeline_sources; $projection_timeline_plan_ids; "store projection and timeline runtime residuals remain preview-only for role-manifest sources"),
      blocker("append_only_store_runtime_enablement_disabled"; "critical"; "append_only_store"; $all_sources; $all_plan_ids; "role enforcement cannot become authoritative before append-only store runtime enablement is promoted"),
      blocker("operator_review_required"; "high"; "operator_review"; $all_sources; $all_plan_ids; "operator review must accept role capability, permission, budget, lane, termination, and output-schema bindings before promotion")
    ] as $blockers
  | (($rerun.required_prior_gates + [$contract.gate, $rerun.gate]) | unique_order) as $required_priors
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_role_manifest_enforcement_gap_closure_preview_gate",
      schema_version: "work_graph_role_manifest_enforcement_gap_closure_preview_v1",
      preview_mode: "read_only_role_manifest_gap_closure_no_enforcement",
      role_blocked_source_count: ($role_sources | length),
      contract_adapter_count: ($contract.adapter_preview_count),
      manifest_required_field_count: ($contract.required_field_count),
      capability_count: ($contract.capability_count),
      permission_mode_count: ($contract.permission_mode_count),
      closure_plan_count: ($plans | length),
      role_binding_count: ($plans | map(.role_binding_ids | length) | add),
      capability_ref_count: ($plans | map(.capability_ids | length) | add),
      permission_mode_ref_count: ($plans | map(.tool_permission_mode_ids | length) | add),
      manifest_field_ref_count: ($plans | map(.covered_wire_fields | length) | add),
      closure_group_count: ($groups | length),
      guard_count: ($guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_priors | length),
      closure_plans: $plans,
      closure_groups: $groups,
      guards: $guards,
      blockers: $blockers,
      required_prior_gates: $required_priors,
      recommended_next_gate: "hepta_work_graph_role_manifest_enforcement_gap_closure_readback_preview_gate",
      ready_for_role_manifest_readback_preview: true,
      ready_for_role_manifest_application_preview: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        role_manifest_gap_closure: {
          rust_module_present: $role_gap_rust_module_present,
          report_script_present: $role_gap_report_script_present,
          gate_script_present: $role_gap_gate_script_present
        },
        role_manifest_contract: {
          rust_module_present: $role_contract_rust_module_present,
          gate_script_present: $role_contract_gate_script_present,
          upstream_gate: ($contract.gate == "hepta_work_graph_role_manifest_contract_preview_gate")
        },
        scheduler_admission_rerun: {
          gate_script_present: $scheduler_rerun_gate_script_present,
          upstream_gate: ($rerun.gate == "hepta_work_graph_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview_gate")
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        wal_written: false,
        checkpoint_written: false,
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
