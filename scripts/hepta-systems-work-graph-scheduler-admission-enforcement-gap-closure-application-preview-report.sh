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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-scheduler-admission-gap-closure-application.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-scheduler-admission-enforcement-gap-closure-readback-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-readback-preview-report.sh" \
  >"$tmpdir/readback.json"

application_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_scheduler_admission_enforcement_gap_closure_application_preview.rs
)"
application_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-application-preview-report.sh
)"
application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-application-preview-gate.sh
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-readback-preview-gate.sh
)"

jq -n \
  --slurpfile readback "$tmpdir/readback.json" \
  --argjson application_rust_module_present "$application_rust_module_present" \
  --argjson application_report_script_present "$application_report_script_present" \
  --argjson application_gate_script_present "$application_gate_script_present" \
  --argjson readback_gate_script_present "$readback_gate_script_present" \
  '
  $readback[0] as $readback
  | def app_plan_id($source):
      "apply_" + $source + "_scheduler_admission_gap_closure_preview";
  def blocker_application_id($blocker):
      "apply_" + $blocker + "_scheduler_admission_blocker_mapping_preview";
  def app_plan_ids_for_sources($plans; $sources):
      $plans | map(select(.source_surface_id as $source | $sources | index($source)) | .application_plan_id);
  def application_plan($plan): {
      application_plan_id: app_plan_id($plan.source_surface_id),
      readback_plan_id: $plan.id,
      closure_plan_id: $plan.closure_plan_id,
      source_surface_id: $plan.source_surface_id,
      source_category: $plan.source_category,
      target_node_kind: $plan.target_node_kind,
      scheduler_blocker_id: $plan.scheduler_blocker_id,
      readback_probe_id: $plan.readback_probe_id,
      controller_adapter_blocker_ids: $plan.controller_adapter_blocker_ids,
      admission_check_ids: $plan.admission_check_ids,
      admission_decision_ids: $plan.admission_decision_ids,
      required_evidence_fields: $plan.required_evidence_fields,
      application_scope: "scheduler_admission_runtime_enforcement_binding",
      application_state: "preview_application_defined_scheduler_admission_not_attached",
      readback_verified_by_preview: true,
      applies_to_runtime: false,
      enforces_scheduler_admission: false,
      starts_work: false,
      acquires_lease: false,
      consumes_budget: false,
      records_approval: false,
      mutates_idempotency_index: false,
      writes_store: false,
      enables_append_only_store: false,
      enforces_role_manifest: false
    };
  def source_outcome($plan): {
      source_surface_id: $plan.source_surface_id,
      source_category: $plan.source_category,
      target_node_kind: $plan.target_node_kind,
      application_plan_id: $plan.application_plan_id,
      post_application_scheduler_admission_state: "scheduler_admission_contract_ready_preview_after_application",
      scheduler_admission_contract_ready_preview: true,
      ready_for_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview: true,
      ready_for_scheduler_admission_enforcement: false,
      applies_to_runtime: false
    };
  def blocker_application($assertion): {
      application_id: blocker_application_id($assertion.blocker_id),
      blocker_id: $assertion.blocker_id,
      category: $assertion.category,
      affected_source_surface_ids: $assertion.affected_source_surface_ids,
      affected_closure_plan_ids: $assertion.affected_closure_plan_ids,
      expected_blocker_state: "blocker_mapping_contract_ready_preview_after_application_runtime_still_blocked",
      blocker_contract_ready_preview: true,
      readback_verified_by_preview: true,
      clears_runtime_blocker: false,
      mutates_runtime: false
    };
  def app_group($id; $priority; $checks; $plans): {
      id: $id,
      priority: $priority,
      admission_check_ids: $checks,
      source_surface_ids: ($plans | map(.source_surface_id)),
      application_plan_ids: ($plans | map(.application_plan_id)),
      expected_scheduler_admission_ready_source_count_after_application: ($plans | length),
      mutates_runtime: false,
      enforces_scheduler_admission: false,
      starts_work: false
    };
  def app_guard($id; $severity; $scope): {
      id: $id,
      severity: $severity,
      guard_scope: $scope,
      required_before_scheduler_admission_enforcement: true,
      satisfied_by_preview: false
    };
  def app_blocker($id; $severity; $category; $sources; $plan_ids; $fix): {
      id: $id,
      severity: $severity,
      category: $category,
      affected_source_surface_ids: $sources,
      affected_application_plan_ids: $plan_ids,
      required_before_scheduler_admission_enforcement: true,
      recommended_fix: $fix
    };
  def app_blocker_from_readback($blocker; $plans):
      app_blocker(
        $blocker.id;
        $blocker.severity;
        $blocker.category;
        $blocker.affected_source_surface_ids;
        app_plan_ids_for_sources($plans; $blocker.affected_source_surface_ids);
        $blocker.recommended_fix
      );
  ($readback.readback_plans | map(application_plan(.))) as $application_plans
  | ($application_plans | map(source_outcome(.))) as $source_outcomes
  | ($readback.blocker_mapping_assertions | map(blocker_application(.))) as $blocker_applications
  | [
      app_group("dependency_and_task_contract_admission_application"; "p0"; ["dependencies_terminal_ready", "task_result_contract_preview_present"]; $application_plans),
      app_group("lease_budget_idempotency_admission_application"; "p0"; ["lane_lease_available_and_owned", "budget_and_timeout_available", "idempotency_replay_window_clear"]; $application_plans),
      app_group("approval_and_side_effect_lock_admission_application"; "p0"; ["approval_authority_present_when_required", "side_effect_boundary_locked"]; $application_plans),
      app_group("scheduler_source_adapter_binding_application"; "p0"; ($application_plans[0].admission_check_ids); $application_plans)
    ] as $application_groups
  | [
      app_guard("scheduler_admission_application_is_preview_only"; "medium"; "application_preview"),
      app_guard("readback_execution_disabled"; "critical"; "readback"),
      app_guard("scheduler_admission_enforcement_disabled"; "critical"; "scheduler_admission"),
      app_guard("lane_lease_acquisition_disabled"; "critical"; "lease"),
      app_guard("dependency_readback_not_executed"; "high"; "dependency_readback"),
      app_guard("approval_recording_disabled"; "critical"; "approval"),
      app_guard("idempotency_index_mutation_disabled"; "critical"; "idempotency"),
      app_guard("budget_consumption_disabled"; "high"; "budget"),
      app_guard("role_manifest_residuals_not_enforced"; "high"; "role_manifest"),
      app_guard("projection_timeline_runtime_residuals_not_promoted"; "high"; "projection_timeline"),
      app_guard("append_only_store_runtime_enablement_disabled"; "critical"; "append_only_store"),
      app_guard("operator_review_required"; "high"; "operator_review"),
      app_guard("enforcement_readiness_scheduler_admission_rerun_required"; "high"; "readiness_rerun")
    ] as $application_guards
  | ($application_plans | map(.source_surface_id)) as $all_sources
  | ($application_plans | map(.application_plan_id)) as $all_application_plan_ids
  | ([app_blocker("scheduler_admission_application_is_preview_only"; "medium"; "application_preview"; $all_sources; $all_application_plan_ids; "keep scheduler admission closure application as a no-mutation preview until readiness rerun proves the blocker moved")]
      + ($readback.blockers | map(select(.id != "scheduler_admission_closure_application_missing") | app_blocker_from_readback(. ; $application_plans)))
      + [app_blocker("scheduler_admission_readiness_rerun_missing"; "high"; "readiness_rerun"; $all_sources; $all_application_plan_ids; "rerun unified projection enforcement-readiness against the scheduler admission application preview outcomes")]) as $blockers
  | ($readback.required_prior_gates + [$readback.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_scheduler_admission_enforcement_gap_closure_application_preview_gate",
      schema_version: "work_graph_scheduler_admission_enforcement_gap_closure_application_preview_v1",
      preview_mode: "read_only_scheduler_admission_gap_closure_application_preview_no_runtime_mutation",
      readback_plan_count: $readback.readback_plan_count,
      application_plan_count: ($application_plans | length),
      source_outcome_count: ($source_outcomes | length),
      scheduler_admission_contract_ready_preview_count: ($source_outcomes | map(select(.scheduler_admission_contract_ready_preview)) | length),
      blocker_application_count: ($blocker_applications | length),
      application_group_count: ($application_groups | length),
      admission_check_ref_count: ($application_plans | map(.admission_check_ids | length) | add),
      admission_decision_ref_count: ($application_plans | map(.admission_decision_ids | length) | add),
      evidence_field_ref_count: ($application_plans | map(.required_evidence_fields | length) | add),
      application_guard_count: ($application_guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      application_plans: $application_plans,
      source_outcomes: $source_outcomes,
      blocker_applications: $blocker_applications,
      application_groups: $application_groups,
      application_guards: $application_guards,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview_gate",
      ready_for_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview: true,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_append_only_store_enablement: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_projection_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        scheduler_admission_gap_closure_application: {
          rust_module_present: $application_rust_module_present,
          report_script_present: $application_report_script_present,
          gate_script_present: $application_gate_script_present
        },
        scheduler_admission_gap_closure_readback: {
          upstream_gate: ($readback.gate == "hepta_work_graph_scheduler_admission_enforcement_gap_closure_readback_preview_gate"),
          gate_script_present: $readback_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        wal_written: false,
        checkpoint_written: false,
        readback_performed: false,
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
