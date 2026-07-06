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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-scheduler-admission-gap-closure.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-unified-projection-enforcement-readiness-append-only-store-rerun-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-append-only-store-rerun-preview-report.sh" \
  >"$tmpdir/append_only_store_rerun.json"
capture_json_report \
  "hepta-work-graph-scheduler-admission-controller-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-scheduler-admission-controller-preview-report.sh" \
  >"$tmpdir/scheduler_admission_controller.json"

scheduler_gap_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_scheduler_admission_enforcement_gap_closure_preview.rs
)"
scheduler_gap_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-preview-report.sh
)"
scheduler_gap_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-preview-gate.sh
)"
scheduler_controller_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_scheduler_admission_controller.rs
)"
scheduler_controller_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-controller-preview-gate.sh
)"

jq -n \
  --slurpfile rerun "$tmpdir/append_only_store_rerun.json" \
  --slurpfile controller "$tmpdir/scheduler_admission_controller.json" \
  --argjson scheduler_gap_rust_module_present "$scheduler_gap_rust_module_present" \
  --argjson scheduler_gap_report_script_present "$scheduler_gap_report_script_present" \
  --argjson scheduler_gap_gate_script_present "$scheduler_gap_gate_script_present" \
  --argjson scheduler_controller_rust_module_present "$scheduler_controller_rust_module_present" \
  --argjson scheduler_controller_gate_script_present "$scheduler_controller_gate_script_present" \
  '
  $rerun[0] as $rerun
  | $controller[0] as $controller
  | def unique_order: reduce .[] as $item ([]; if index($item) then . else . + [$item] end);
  def adapter_for($id): [$controller.adapter_previews[] | select(.source_surface_id == $id)][0];
  def has_contains($values; $needle): any($values[]?; contains($needle));
  def scheduler_blocker_for($decision): [$decision.residual_source_blocker_ids[] | select(endswith("_admission_not_enforced"))][0];
  def source_has($source; $needle): any($rerun.decision_deltas[]? | select(.source_surface_id == $source) | .residual_source_blocker_ids[]?; contains($needle));
  def evidence_fields: [$controller.checks[] | .required_evidence_fields[]] | unique_order;
  def plan_for($decision):
      (adapter_for($decision.source_surface_id)) as $adapter
      | {
          closure_plan_id: ("scheduler_admission_closure_plan:" + $decision.source_surface_id),
          source_surface_id: $decision.source_surface_id,
          source_category: $decision.source_category,
          target_node_kind: $adapter.target_node_kind,
          scheduler_blocker_id: scheduler_blocker_for($decision),
          source_fields: $adapter.source_fields,
          controller_adapter_blocker_ids: $adapter.blocker_ids,
          admission_check_ids: ($controller.checks | map(.id)),
          admission_decision_ids: ($controller.decisions | map(.id)),
          required_evidence_fields: evidence_fields,
          readback_probe_id: ("scheduler_admission_readback_probe:" + $decision.source_surface_id),
          closure_scope: "scheduler_admission_contract_preview_only",
          closure_state: "scheduler_admission_contract_ready_preview",
          ready_for_readback_preview: true,
          applies_to_runtime: false,
          enforces_scheduler_admission: false,
          starts_work: false,
          acquires_lease: false,
          writes_store: false,
          mutates_idempotency_index: false,
          records_approval: false
        };
  def group($id; $priority; $check_ids; $plans): {
      id: $id,
      priority: $priority,
      check_ids: $check_ids,
      closure_plan_ids: ($plans | map(.closure_plan_id)),
      source_surface_ids: ($plans | map(.source_surface_id)),
      mutates_runtime: false,
      enforces_scheduler_admission: false
    };
  def guard($id; $severity; $scope): {
      id: $id,
      severity: $severity,
      guard_scope: $scope,
      required_before_scheduler_admission_enforcement: true,
      satisfied_by_preview: false
    };
  def blocker($id; $severity; $category; $sources; $plans; $fix): {
      id: $id,
      severity: $severity,
      category: $category,
      affected_source_surface_ids: $sources,
      affected_closure_plan_ids: $plans,
      required_before_scheduler_admission_enforcement: true,
      recommended_fix: $fix
    };
  ($rerun.decision_deltas | map(select(.append_only_store_rerun_enforcement_decision == "deny_scheduler_admission_not_enforced"))) as $scheduler_sources
  | ($scheduler_sources | map(plan_for(.))) as $plans
  | ($plans | map(.source_surface_id)) as $all_sources
  | ($plans | map(.closure_plan_id)) as $all_plan_ids
  | ($plans | map(select(source_has(.source_surface_id; "role_manifest_not_enforced")) | .source_surface_id)) as $role_sources
  | ($plans | map(select(source_has(.source_surface_id; "role_manifest_not_enforced")) | .closure_plan_id)) as $role_plan_ids
  | ($plans | map(select(source_has(.source_surface_id; "store_projection_not_enforced") or source_has(.source_surface_id; "timeline_adapter_not_enforced")) | .source_surface_id)) as $projection_timeline_sources
  | ($plans | map(select(source_has(.source_surface_id; "store_projection_not_enforced") or source_has(.source_surface_id; "timeline_adapter_not_enforced")) | .closure_plan_id)) as $projection_timeline_plan_ids
  | [
      group("dependency_and_task_contract_admission_closure"; "p0"; ["dependencies_terminal_ready","task_result_contract_preview_present"]; $plans),
      group("lease_budget_idempotency_admission_closure"; "p0"; ["lane_lease_available_and_owned","budget_and_timeout_available","idempotency_replay_window_clear"]; $plans),
      group("approval_and_side_effect_lock_admission_closure"; "p0"; ["approval_authority_present_when_required","side_effect_boundary_locked"]; $plans),
      group("scheduler_source_adapter_binding_closure"; "p0"; ($controller.checks | map(.id)); $plans)
    ] as $groups
  | [
      guard("scheduler_admission_closure_is_preview_only"; "medium"; "closure_preview"),
      guard("controller_adapter_contract_required"; "high"; "controller_adapter"),
      guard("dependency_terminal_evidence_required"; "high"; "dependency_evidence"),
      guard("lane_lease_not_acquired"; "critical"; "lane_lease"),
      guard("approval_not_recorded"; "critical"; "approval"),
      guard("idempotency_index_not_mutated"; "critical"; "idempotency"),
      guard("budget_not_consumed"; "high"; "budget"),
      guard("scheduler_admission_not_enforced"; "critical"; "scheduler_admission"),
      guard("append_only_store_runtime_not_enabled"; "critical"; "append_only_store")
    ] as $guards
  | [
      blocker("scheduler_admission_enforcement_disabled"; "critical"; "scheduler_admission"; $all_sources; $all_plan_ids; "keep admission checks preview-only until readback, application, and operator-review gates are promoted"),
      blocker("lane_lease_acquisition_disabled"; "critical"; "lease"; $all_sources; $all_plan_ids; "do not acquire or mutate lane leases from the scheduler admission closure preview"),
      blocker("dependency_readback_not_executed"; "high"; "dependency_readback"; $all_sources; $all_plan_ids; "read back dependency terminal states before scheduler admission can become authoritative"),
      blocker("approval_recording_disabled"; "critical"; "approval"; $all_sources; $all_plan_ids; "approval evidence must be recorded by a later runtime boundary, not this preview"),
      blocker("idempotency_index_mutation_disabled"; "critical"; "idempotency"; $all_sources; $all_plan_ids; "idempotency index mutation remains blocked until append-only store and replay gates are promoted"),
      blocker("budget_consumption_disabled"; "high"; "budget"; $all_sources; $all_plan_ids; "budget checks are contract-only here and cannot consume resource or retry budget"),
      blocker("role_manifest_residuals_not_enforced"; "high"; "role_manifest"; $role_sources; $role_plan_ids; "role-manifest residuals must be handled by the role manifest enforcement frontier before scheduler cutover"),
      blocker("projection_timeline_runtime_residuals_not_promoted"; "high"; "projection_timeline"; $projection_timeline_sources; $projection_timeline_plan_ids; "store projection and timeline runtime residuals remain preview-only for scheduler-backed sources"),
      blocker("append_only_store_runtime_enablement_disabled"; "critical"; "append_only_store"; $all_sources; $all_plan_ids; "scheduler admission cannot become authoritative before append-only store runtime enablement is promoted"),
      blocker("scheduler_admission_closure_readback_missing"; "high"; "readback"; $all_sources; $all_plan_ids; "next gate must read back admission bindings, evidence fields, and no-mutation guards before application preview")
    ] as $blockers
  | (($rerun.required_prior_gates + [$controller.gate, $rerun.gate]) | unique_order) as $required_priors
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_scheduler_admission_enforcement_gap_closure_preview_gate",
      schema_version: "work_graph_scheduler_admission_enforcement_gap_closure_preview_v1",
      preview_mode: "read_only_scheduler_admission_gap_closure_no_enforcement",
      scheduler_blocked_source_count: ($scheduler_sources | length),
      controller_check_count: ($controller.checks | length),
      controller_decision_count: ($controller.decisions | length),
      controller_adapter_count: ($controller.adapter_previews | length),
      closure_plan_count: ($plans | length),
      admission_binding_count: ($plans | length),
      readback_probe_binding_count: ($plans | length),
      evidence_field_ref_count: ($plans | map(.required_evidence_fields | length) | add),
      closure_group_count: ($groups | length),
      guard_count: ($guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_priors | length),
      closure_plans: $plans,
      closure_groups: $groups,
      guards: $guards,
      blockers: $blockers,
      required_prior_gates: $required_priors,
      recommended_next_gate: "hepta_work_graph_scheduler_admission_enforcement_gap_closure_readback_preview_gate",
      ready_for_scheduler_admission_readback_preview: true,
      ready_for_scheduler_admission_application_preview: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        scheduler_admission_gap_closure: {
          rust_module_present: $scheduler_gap_rust_module_present,
          report_script_present: $scheduler_gap_report_script_present,
          gate_script_present: $scheduler_gap_gate_script_present
        },
        scheduler_admission_controller: {
          rust_module_present: $scheduler_controller_rust_module_present,
          gate_script_present: $scheduler_controller_gate_script_present,
          upstream_gate: ($controller.gate == "hepta_work_graph_scheduler_admission_controller_preview_gate")
        },
        append_only_store_rerun: {
          upstream_gate: ($rerun.gate == "hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview_gate")
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        wal_written: false,
        checkpoint_written: false,
        scheduler_admission_enforced: false,
        lease_acquired: false,
        work_started: false,
        budget_consumed: false,
        approval_recorded: false,
        idempotency_index_mutated: false,
        append_only_store_enabled: false,
        task_result_enforcement_enabled: false,
        role_manifest_enforcement_enabled: false,
        projection_enforcement_enabled: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
