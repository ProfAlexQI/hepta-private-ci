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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-runtime-application-promotion-application.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-runtime-application-promotion-gap-closure-readback-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-runtime-application-promotion-gap-closure-readback-preview-report.sh" \
  >"$tmpdir/readback.json"

application_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_runtime_application_promotion_gap_closure_application_preview.rs
)"
application_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-runtime-application-promotion-gap-closure-application-preview-report.sh
)"
application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-runtime-application-promotion-gap-closure-application-preview-gate.sh
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-runtime-application-promotion-gap-closure-readback-preview-gate.sh
)"

jq -n \
  --slurpfile readback "$tmpdir/readback.json" \
  --argjson application_rust_module_present "$application_rust_module_present" \
  --argjson application_report_script_present "$application_report_script_present" \
  --argjson application_gate_script_present "$application_gate_script_present" \
  --argjson readback_gate_script_present "$readback_gate_script_present" \
  '
  $readback[0] as $readback
  | def app_plan_id($id): "apply_" + $id + "_runtime_application_promotion_preview";
  def binding_app_id($id): "apply_" + $id + "_promotion_binding_preview";
  def group_app_id($id): "apply_" + $id + "_promotion_group_preview";
  def blocker_app_id($id): "apply_" + $id + "_runtime_application_blocker_preview";
  def app_ids_for_readback($plans; $ids):
      $plans | map(select(.readback_plan_id as $readback_plan | $ids | index($readback_plan)) | .application_plan_id);
  def app_ids_for_sources($plans; $ids):
      $plans | map(select(.source_surface_id as $source | $ids | index($source)) | .application_plan_id);
  def unique_sources($plans):
      reduce ($plans[] | .source_surface_id) as $source ([]; if index($source) then . else . + [$source] end);
  def app_plan($plan): {
      application_plan_id: app_plan_id($plan.id),
      readback_plan_id: $plan.id,
      closure_plan_id: $plan.closure_plan_id,
      source_surface_id: $plan.source_surface_id,
      source_category: $plan.source_category,
      runtime_rerun_decision: $plan.runtime_rerun_decision,
      promotion_domain_ids: $plan.promotion_domain_ids,
      promotion_binding_ids: $plan.promotion_binding_ids,
      readback_probe_id: $plan.readback_probe_id,
      expected_evidence_field_ids: $plan.required_evidence_fields,
      residual_source_blocker_ids: $plan.residual_source_blocker_ids,
      application_scope: "runtime_application_promotion_gap_closure_application_binding",
      application_state: "preview_application_defined_runtime_application_not_promoted",
      readback_verified_by_preview: true,
      applies_to_runtime: false,
      promotes_runtime_application: false,
      attaches_runtime_wrapper: false,
      enforces_scheduler_admission: false,
      enforces_role_manifest: false,
      enables_task_result_enforcement: false,
      writes_store: false,
      writes_wal: false,
      records_approval: false,
      executes_readback: false,
      mutates_runtime: false
    };
  def source_outcome($plan): {
      source_surface_id: $plan.source_surface_id,
      source_category: $plan.source_category,
      application_plan_id: $plan.application_plan_id,
      post_application_runtime_promotion_state: "runtime_application_promotion_contract_ready_preview_after_application",
      runtime_application_contract_ready_preview: true,
      ready_for_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview: true,
      ready_for_runtime_application_promotion: false,
      applies_to_runtime: false
    };
  def binding_application($assertion): {
      application_id: binding_app_id($assertion.binding_id),
      binding_id: $assertion.binding_id,
      source_surface_id: $assertion.source_surface_id,
      promotion_domain_id: $assertion.promotion_domain_id,
      closes_blocker_id: $assertion.closes_blocker_id,
      required_evidence_field_ids: $assertion.required_evidence_field_ids,
      expected_binding_state: "binding_contract_ready_preview_after_application_runtime_still_blocked",
      binding_contract_ready_preview: true,
      readback_verified_by_preview: true,
      promotes_runtime_application: false,
      writes_store: false
    };
  def group_application($assertion; $plans): {
      application_id: group_app_id($assertion.group_id),
      group_id: $assertion.group_id,
      promotion_domain_id: $assertion.promotion_domain_id,
      affected_source_surface_ids: $assertion.affected_source_surface_ids,
      application_plan_ids: app_ids_for_sources($plans; $assertion.affected_source_surface_ids),
      promotion_binding_ids: $assertion.promotion_binding_ids,
      expected_contract_count_after_application: $assertion.expected_contract_count_after_closure,
      group_contract_ready_preview: true,
      readback_verified_by_preview: true,
      promotes_runtime_application: false,
      mutates_runtime: false
    };
  def blocker_application($assertion; $plans): {
      application_id: blocker_app_id($assertion.blocker_id),
      blocker_id: $assertion.blocker_id,
      severity: $assertion.severity,
      affected_source_surface_ids: $assertion.affected_source_surface_ids,
      affected_readback_plan_ids: $assertion.affected_readback_plan_ids,
      affected_application_plan_ids: app_ids_for_readback($plans; $assertion.affected_readback_plan_ids),
      blocked_promotion_domain_ids: $assertion.blocked_promotion_domain_ids,
      expected_blocker_state: "blocker_mapping_contract_ready_preview_after_application_runtime_still_blocked",
      blocker_contract_ready_preview: true,
      readback_verified_by_preview: true,
      clears_runtime_blocker: false,
      mutates_runtime: false
    };
  def app_guard($id; $severity; $scope): {
      id: $id,
      severity: $severity,
      guard_scope: $scope,
      required_before_runtime_application_promotion: true,
      satisfied_by_preview: false
    };
  def blocker($id; $severity; $category; $sources; $application_plan_ids; $domains; $fix): {
      id: $id,
      severity: $severity,
      category: $category,
      affected_source_surface_ids: $sources,
      affected_application_plan_ids: $application_plan_ids,
      blocked_promotion_domain_ids: $domains,
      required_before_runtime_application_promotion: true,
      recommended_fix: $fix
    };
  def blocker_from_readback($blocker; $plans):
      blocker(
        $blocker.id;
        $blocker.severity;
        "runtime_application_promotion";
        $blocker.affected_source_surface_ids;
        app_ids_for_sources($plans; $blocker.affected_source_surface_ids);
        $blocker.blocked_promotion_domain_ids;
        $blocker.recommended_fix
      );
  ($readback.readback_plans | map(app_plan(.))) as $application_plans
  | ($application_plans | map(source_outcome(.))) as $source_outcomes
  | ($readback.promotion_binding_assertions | map(binding_application(.))) as $binding_applications
  | ($readback.promotion_group_assertions | map(group_application(.; $application_plans))) as $group_applications
  | ($readback.blocker_mapping_assertions | map(blocker_application(.; $application_plans))) as $blocker_applications
  | [
      app_guard("runtime_application_promotion_application_is_preview_only"; "medium"; "application_preview"),
      app_guard("readback_execution_disabled"; "critical"; "readback"),
      app_guard("runtime_application_promotion_disabled"; "critical"; "runtime_application"),
      app_guard("runtime_wrapper_attachment_disabled"; "high"; "runtime_wrapper"),
      app_guard("task_result_enforcement_disabled"; "high"; "task_result"),
      app_guard("scheduler_admission_runtime_enforcement_disabled"; "high"; "scheduler"),
      app_guard("role_manifest_runtime_enforcement_disabled"; "high"; "role_manifest"),
      app_guard("operator_review_required"; "high"; "operator_review"),
      app_guard("side_effect_lock_not_established"; "critical"; "side_effect_lock"),
      app_guard("wal_write_boundary_disabled"; "critical"; "wal_boundary"),
      app_guard("durable_store_runtime_switch_disabled"; "critical"; "durable_store_switch"),
      app_guard("append_only_store_enablement_disabled"; "critical"; "append_only_store")
    ] as $application_guards
  | (($readback.blockers | map(blocker_from_readback(.; $application_plans))) + [
      blocker(
        "runtime_application_promotion_readiness_rerun_missing";
        "high";
        "readiness_rerun";
        unique_sources($application_plans);
        ($application_plans | map(.application_plan_id));
        [
          "projection_adapter_runtime_closure",
          "store_guard_runtime_application",
          "terminal_task_result_runtime_wrapper",
          "scheduler_admission_runtime_application",
          "role_manifest_runtime_application"
        ];
        "rerun unified projection enforcement-readiness against runtime application promotion application preview outcomes"
      )
    ]) as $blockers
  | ($readback.required_prior_gates + [$readback.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_runtime_application_promotion_gap_closure_application_preview_gate",
      schema_version: "work_graph_runtime_application_promotion_gap_closure_application_preview_v1",
      preview_mode: "read_only_runtime_application_promotion_gap_closure_application_no_runtime_mutation",
      readback_plan_count: $readback.readback_plan_count,
      application_plan_count: ($application_plans | length),
      source_outcome_count: ($source_outcomes | length),
      runtime_application_contract_ready_preview_count: ($source_outcomes | map(select(.runtime_application_contract_ready_preview)) | length),
      promotion_binding_application_count: ($binding_applications | length),
      promotion_group_application_count: ($group_applications | length),
      blocker_application_count: ($blocker_applications | length),
      application_guard_count: ($application_guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      promotion_domain_ref_count: ($application_plans | map(.promotion_domain_ids | length) | add),
      promotion_binding_ref_count: ($application_plans | map(.promotion_binding_ids | length) | add),
      evidence_field_ref_count: ($application_plans | map(.expected_evidence_field_ids | length) | add),
      group_source_ref_count: ($group_applications | map(.affected_source_surface_ids | length) | add),
      blocker_mapping_source_ref_count: ($blocker_applications | map(.affected_source_surface_ids | length) | add),
      application_plans: $application_plans,
      source_outcomes: $source_outcomes,
      promotion_binding_applications: $binding_applications,
      promotion_group_applications: $group_applications,
      blocker_applications: $blocker_applications,
      application_guards: $application_guards,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview_gate",
      ready_for_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview: true,
      ready_for_runtime_application_promotion: false,
      ready_for_operator_review_side_effect_lock: false,
      ready_for_append_only_store_enablement: false,
      ready_for_projection_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        runtime_application_promotion_gap_closure_application: {
          rust_module_present: $application_rust_module_present,
          report_script_present: $application_report_script_present,
          gate_script_present: $application_gate_script_present
        },
        runtime_application_promotion_gap_closure_readback: {
          upstream_gate: ($readback.gate == "hepta_work_graph_runtime_application_promotion_gap_closure_readback_preview_gate"),
          gate_script_present: $readback_gate_script_present,
          recommended_next_matches: ($readback.recommended_next_gate == "hepta_work_graph_runtime_application_promotion_gap_closure_application_preview_gate")
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
        readback_performed: false,
        runtime_application_promoted: false,
        runtime_wrapper_attached: false,
        scheduler_admission_enforced: false,
        role_manifest_enforced: false,
        task_result_enforcement_enabled: false,
        task_result_persisted: false,
        rollback_executed: false,
        approval_recorded: false,
        operator_review_recorded: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }
  '
