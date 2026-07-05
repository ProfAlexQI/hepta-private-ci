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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-append-only-store-precondition-application.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-append-only-store-enablement-precondition-readback-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-append-only-store-enablement-precondition-readback-preview-report.sh" \
  >"$tmpdir/readback.json"

application_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_store_enablement_precondition_application_preview.rs
)"
application_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-enablement-precondition-application-preview-report.sh
)"
application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-enablement-precondition-application-preview-gate.sh
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-enablement-precondition-readback-preview-gate.sh
)"

jq -n \
  --slurpfile readback "$tmpdir/readback.json" \
  --argjson application_rust_module_present "$application_rust_module_present" \
  --argjson application_report_script_present "$application_report_script_present" \
  --argjson application_gate_script_present "$application_gate_script_present" \
  --argjson readback_gate_script_present "$readback_gate_script_present" \
  '
  $readback[0] as $readback
  | def app_plan_id($id): "apply_" + $id + "_append_only_store_precondition_preview";
  def blocker_app_id($id): "apply_" + $id + "_append_only_store_blocker_mapping_preview";
  def app_plan($plan): {
      application_plan_id: app_plan_id($plan.precondition_id),
      readback_precondition_id: $plan.precondition_id,
      category: $plan.category,
      severity: $plan.severity,
      affected_source_surface_ids: $plan.affected_source_surface_ids,
      expected_contract_ref_ids: $plan.expected_contract_ref_ids,
      expected_blocker_id: $plan.expected_blocker_id,
      required_evidence_fields: $plan.required_evidence_fields,
      application_scope: "append_only_store_enablement_precondition_runtime_binding",
      application_state: "preview_application_defined_precondition_not_applied_to_runtime",
      readback_verified_by_preview: true,
      applies_to_runtime: false,
      persists_precondition_state: false,
      enables_append_only_store: false,
      mutates_store: false,
      writes_wal: false,
      writes_checkpoint: false,
      mutates_idempotency_index: false,
      enforces_scheduler_admission: false,
      enforces_role_manifest: false
    };
  def outcome($plan): {
      precondition_id: $plan.readback_precondition_id,
      category: $plan.category,
      application_plan_id: $plan.application_plan_id,
      post_application_precondition_state: "precondition_contract_ready_preview_after_application",
      precondition_contract_ready_preview: true,
      ready_for_unified_projection_enforcement_readiness_append_only_store_rerun_preview: true,
      ready_for_append_only_store_enablement: false,
      applies_to_runtime: false
    };
  def blocker_application($assertion): {
      application_id: blocker_app_id($assertion.blocker_id),
      blocker_id: $assertion.blocker_id,
      category: $assertion.category,
      affected_precondition_ids: $assertion.affected_precondition_ids,
      affected_source_surface_ids: $assertion.affected_source_surface_ids,
      expected_blocker_state: "blocker_mapping_contract_ready_preview_after_application_runtime_still_blocked",
      blocker_contract_ready_preview: true,
      readback_verified_by_preview: true,
      clears_runtime_blocker: false,
      mutates_store: false
    };
  def app_group($id; $priority; $preconditions; $plans): {
      id: $id,
      priority: $priority,
      precondition_ids: $preconditions,
      application_plan_ids: ($plans | map(select(.readback_precondition_id as $precondition | $preconditions | index($precondition)) | .application_plan_id)),
      expected_precondition_contract_ready_count_after_application: ($preconditions | length),
      mutates_runtime: false,
      enables_append_only_store: false,
      writes_wal: false
    };
  def app_guard($id; $severity; $scope): {
      id: $id,
      severity: $severity,
      guard_scope: $scope,
      required_before_append_only_store_enablement: true,
      satisfied_by_preview: false
    };
  def affected_sources($plans; $predicate):
      reduce ($plans[] | select($predicate) | .affected_source_surface_ids[]) as $source ([]; if index($source) then . else . + [$source] end);
  def plan_ids($plans; $predicate):
      $plans | map(select($predicate) | .application_plan_id);
  def blocker($id; $severity; $category; $preconditions; $sources; $plan_ids; $fix): {
      id: $id,
      severity: $severity,
      category: $category,
      affected_precondition_ids: $preconditions,
      affected_source_surface_ids: $sources,
      affected_application_plan_ids: $plan_ids,
      required_before_append_only_store_enablement: true,
      recommended_fix: $fix
    };
  def blocker_from_readback($blocker; $plans):
      blocker(
        $blocker.id;
        $blocker.severity;
        $blocker.category;
        $blocker.affected_precondition_ids;
        $blocker.affected_source_surface_ids;
        ($plans | map(select(.readback_precondition_id as $precondition | $blocker.affected_precondition_ids | index($precondition)) | .application_plan_id));
        $blocker.recommended_fix
      );
  ($readback.readback_plans | map(app_plan(.))) as $application_plans
  | ($application_plans | map(outcome(.))) as $precondition_outcomes
  | ($readback.blocker_mapping_assertions | map(blocker_application(.))) as $blocker_applications
  | [
      app_group("append_only_store_core_precondition_application"; "p0"; ["durable_store_enablement_switch", "wal_append_boundary_contract"]; $application_plans),
      app_group("append_only_replay_safety_precondition_application"; "p0"; ["idempotency_mutation_policy", "rollback_readback_gate"]; $application_plans),
      app_group("append_only_operator_lock_precondition_application"; "p0"; ["operator_review_and_side_effect_lock"]; $application_plans),
      app_group("append_only_scheduler_role_precondition_application"; "p0"; ["scheduler_admission_enforcement_precondition", "role_manifest_enforcement_precondition"]; $application_plans)
    ] as $application_groups
  | [
      app_guard("precondition_application_is_preview_only"; "medium"; "application_preview"),
      app_guard("durable_store_runtime_switch_disabled"; "critical"; "durable_store_switch"),
      app_guard("wal_write_boundary_disabled"; "critical"; "wal_boundary"),
      app_guard("idempotency_index_mutation_disabled"; "critical"; "idempotency_index"),
      app_guard("rollback_readback_execution_disabled"; "critical"; "rollback_readback"),
      app_guard("operator_review_required"; "high"; "operator_review"),
      app_guard("scheduler_admission_not_enforced"; "high"; "scheduler_admission"),
      app_guard("role_manifest_not_enforced"; "high"; "role_manifest"),
      app_guard("runtime_application_residuals_not_promoted"; "high"; "runtime_application"),
      app_guard("append_only_store_readiness_rerun_required"; "high"; "readiness_rerun")
    ] as $application_guards
  | (($readback.blockers | map(blocker_from_readback(.; $application_plans))) + [
      blocker(
        "append_only_store_readiness_rerun_missing";
        "high";
        "readiness_rerun";
        ($application_plans | map(.readback_precondition_id));
        affected_sources($application_plans; true);
        plan_ids($application_plans; true);
        "rerun unified projection enforcement-readiness against the append-only store precondition application preview outcomes"
      )
    ]) as $blockers
  | ($readback.required_prior_gates + [$readback.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_append_only_store_enablement_precondition_application_preview_gate",
      schema_version: "work_graph_append_only_store_enablement_precondition_application_preview_v1",
      preview_mode: "read_only_append_only_store_enablement_precondition_application_preview_no_runtime_mutation",
      readback_plan_count: $readback.readback_plan_count,
      application_plan_count: ($application_plans | length),
      precondition_outcome_count: ($precondition_outcomes | length),
      precondition_contract_ready_preview_count: ($precondition_outcomes | map(select(.precondition_contract_ready_preview)) | length),
      blocker_application_count: ($blocker_applications | length),
      application_group_count: ($application_groups | length),
      contract_ref_count: ($application_plans | map(.expected_contract_ref_ids | length) | add),
      source_ref_count: ($application_plans | map(.affected_source_surface_ids | length) | add),
      evidence_field_ref_count: ($application_plans | map(.required_evidence_fields | length) | add),
      blocker_mapping_source_ref_count: ($blocker_applications | map(.affected_source_surface_ids | length) | add),
      application_guard_count: ($application_guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      application_plans: $application_plans,
      precondition_outcomes: $precondition_outcomes,
      blocker_applications: $blocker_applications,
      application_groups: $application_groups,
      application_guards: $application_guards,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview_gate",
      ready_for_unified_projection_enforcement_readiness_append_only_store_rerun_preview: true,
      ready_for_append_only_store_enablement: false,
      ready_for_projection_enforcement: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        append_only_store_enablement_precondition_application: {
          rust_module_present: $application_rust_module_present,
          report_script_present: $application_report_script_present,
          gate_script_present: $application_gate_script_present
        },
        append_only_store_enablement_precondition_readback: {
          upstream_gate: ($readback.gate == "hepta_work_graph_append_only_store_enablement_precondition_readback_preview_gate"),
          gate_script_present: $readback_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        wal_written: false,
        checkpoint_written: false,
        idempotency_index_mutated: false,
        precondition_state_persisted: false,
        append_only_store_enabled: false,
        projection_enforcement_enabled: false,
        readback_executed: false,
        rollback_executed: false,
        scheduler_admission_enforced: false,
        role_manifest_enforcement_enabled: false,
        task_result_enforcement_enabled: false,
        runtime_wrapper_attached: false,
        approval_recorded: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
