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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-append-only-store-runtime-application.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-append-only-store-runtime-enablement-readback-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-append-only-store-runtime-enablement-readback-preview-report.sh" \
  >"$tmpdir/readback.json"

application_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_store_runtime_enablement_application_preview.rs
)"
application_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-runtime-enablement-application-preview-report.sh
)"
application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-runtime-enablement-application-preview-gate.sh
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-runtime-enablement-readback-preview-gate.sh
)"

jq -n \
  --slurpfile readback "$tmpdir/readback.json" \
  --argjson application_rust_module_present "$application_rust_module_present" \
  --argjson application_report_script_present "$application_report_script_present" \
  --argjson application_gate_script_present "$application_gate_script_present" \
  --argjson readback_gate_script_present "$readback_gate_script_present" \
  '
  $readback[0] as $readback
  | def app_plan_id($id): "apply_" + $id + "_runtime_enablement_preview";
  def stage_app_id($id): "apply_" + $id + "_runtime_stage_preview";
  def blocker_app_id($id): "apply_" + $id + "_runtime_blocker_mapping_preview";
  def app_plan($plan): {
      application_plan_id: app_plan_id($plan.runtime_enablement_plan_id),
      readback_runtime_enablement_plan_id: $plan.runtime_enablement_plan_id,
      source_surface_id: $plan.source_surface_id,
      source_category: $plan.source_category,
      expected_runtime_stage_ids: $plan.expected_runtime_stage_ids,
      expected_evidence_field_ids: $plan.expected_evidence_field_ids,
      residual_source_blocker_ids: $plan.residual_source_blocker_ids,
      application_scope: "append_only_store_runtime_enablement_application_binding",
      application_state: "preview_application_defined_runtime_enablement_not_applied",
      readback_verified_by_preview: true,
      applies_to_runtime: false,
      enables_append_only_store: false,
      writes_wal: false,
      writes_checkpoint: false,
      mutates_idempotency_index: false,
      executes_readback: false,
      executes_rollback: false,
      records_approval: false,
      promotes_runtime_application: false,
      mutates_store: false
    };
  def source_outcome($plan): {
      source_surface_id: $plan.source_surface_id,
      source_category: $plan.source_category,
      application_plan_id: $plan.application_plan_id,
      post_application_runtime_enablement_state: "runtime_enablement_contract_ready_preview_after_application",
      runtime_enablement_contract_ready_preview: true,
      ready_for_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview: true,
      ready_for_append_only_store_enablement: false,
      applies_to_runtime: false
    };
  def stage_application($assertion): {
      application_id: stage_app_id($assertion.runtime_stage_id),
      runtime_stage_id: $assertion.runtime_stage_id,
      category: $assertion.category,
      affected_source_surface_ids: $assertion.expected_source_surface_ids,
      expected_contract_ref_ids: $assertion.expected_contract_ref_ids,
      expected_stage_state: "stage_contract_ready_preview_after_application_runtime_disabled",
      stage_contract_ready_preview: true,
      readback_verified_by_preview: true,
      applies_to_runtime: false,
      enables_append_only_store: false,
      writes_wal: false,
      mutates_idempotency_index: false,
      executes_readback: false
    };
  def app_ids_for($plans; $ids):
      $plans | map(select(.readback_runtime_enablement_plan_id as $runtime_plan | $ids | index($runtime_plan)) | .application_plan_id);
  def blocker_application($assertion; $plans): {
      application_id: blocker_app_id($assertion.blocker_id),
      blocker_id: $assertion.blocker_id,
      category: $assertion.category,
      affected_source_surface_ids: $assertion.affected_source_surface_ids,
      affected_runtime_stage_ids: $assertion.affected_runtime_stage_ids,
      affected_runtime_enablement_plan_ids: $assertion.affected_runtime_enablement_plan_ids,
      affected_application_plan_ids: app_ids_for($plans; $assertion.affected_runtime_enablement_plan_ids),
      expected_blocker_state: "blocker_mapping_contract_ready_preview_after_application_runtime_still_blocked",
      blocker_contract_ready_preview: true,
      readback_verified_by_preview: true,
      clears_runtime_blocker: false,
      mutates_runtime: false
    };
  def app_group($id; $priority; $stage_ids; $stages): {
      id: $id,
      priority: $priority,
      runtime_stage_ids: $stage_ids,
      stage_application_ids: ($stages | map(select(.runtime_stage_id as $stage | $stage_ids | index($stage)) | .application_id)),
      expected_stage_contract_ready_count_after_application: ($stage_ids | length),
      mutates_runtime: false,
      enables_append_only_store: false,
      writes_wal: false
    };
  def app_guard($id; $severity; $scope): {
      id: $id,
      severity: $severity,
      guard_scope: $scope,
      required_before_append_only_store_runtime_enablement: true,
      satisfied_by_preview: false
    };
  def affected_sources($plans):
      reduce ($plans[] | .source_surface_id) as $source ([]; if index($source) then . else . + [$source] end);
  def blocker($id; $severity; $category; $sources; $runtime_plan_ids; $application_plan_ids; $fix): {
      id: $id,
      severity: $severity,
      category: $category,
      affected_source_surface_ids: $sources,
      affected_runtime_enablement_plan_ids: $runtime_plan_ids,
      affected_application_plan_ids: $application_plan_ids,
      required_before_append_only_store_runtime_enablement: true,
      recommended_fix: $fix
    };
  def blocker_from_readback($blocker; $plans):
      blocker(
        $blocker.id;
        $blocker.severity;
        $blocker.category;
        $blocker.affected_source_surface_ids;
        $blocker.affected_runtime_enablement_plan_ids;
        app_ids_for($plans; $blocker.affected_runtime_enablement_plan_ids);
        $blocker.recommended_fix
      );
  ($readback.readback_plans | map(app_plan(.))) as $application_plans
  | ($application_plans | map(source_outcome(.))) as $source_outcomes
  | ($readback.stage_plan_assertions | map(stage_application(.))) as $stage_applications
  | ($readback.blocker_mapping_assertions | map(blocker_application(.; $application_plans))) as $blocker_applications
  | [
      app_group("append_only_store_runtime_core_application"; "p0"; ["durable_store_runtime_switch", "wal_write_boundary"]; $stage_applications),
      app_group("append_only_store_runtime_replay_safety_application"; "p0"; ["idempotency_mutation_policy", "rollback_readback_execution_gate"]; $stage_applications),
      app_group("append_only_store_runtime_operator_lock_application"; "p0"; ["operator_review_side_effect_lock"]; $stage_applications),
      app_group("append_only_store_runtime_application_promotion_preview"; "p0"; ["runtime_application_promotion"]; $stage_applications)
    ] as $application_groups
  | [
      app_guard("runtime_enablement_application_is_preview_only"; "medium"; "application_preview"),
      app_guard("durable_store_runtime_switch_disabled"; "critical"; "durable_store_switch"),
      app_guard("wal_write_boundary_disabled"; "critical"; "wal_boundary"),
      app_guard("idempotency_index_mutation_disabled"; "critical"; "idempotency_index"),
      app_guard("rollback_readback_execution_disabled"; "critical"; "rollback_readback"),
      app_guard("operator_review_required"; "high"; "operator_review"),
      app_guard("runtime_application_promotion_disabled"; "high"; "runtime_application"),
      app_guard("scheduler_role_runtime_application_disabled"; "high"; "scheduler_role"),
      app_guard("append_only_store_runtime_readiness_rerun_required"; "high"; "readiness_rerun"),
      app_guard("side_effect_lock_not_established"; "critical"; "side_effect_lock"),
      app_guard("graph_state_persistence_disabled"; "critical"; "graph_state_persistence")
    ] as $application_guards
  | (($readback.blockers | map(blocker_from_readback(.; $application_plans))) + [
      blocker(
        "append_only_store_runtime_readiness_rerun_missing";
        "high";
        "readiness_rerun";
        affected_sources($application_plans);
        ($application_plans | map(.readback_runtime_enablement_plan_id));
        ($application_plans | map(.application_plan_id));
        "rerun unified projection enforcement-readiness against the append-only store runtime enablement application preview outcomes"
      )
    ]) as $blockers
  | ($readback.required_prior_gates + [$readback.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_append_only_store_runtime_enablement_application_preview_gate",
      schema_version: "work_graph_append_only_store_runtime_enablement_application_preview_v1",
      preview_mode: "read_only_append_only_store_runtime_enablement_application_preview_no_runtime_mutation",
      readback_plan_count: $readback.readback_plan_count,
      application_plan_count: ($application_plans | length),
      source_outcome_count: ($source_outcomes | length),
      runtime_enablement_contract_ready_preview_count: ($source_outcomes | map(select(.runtime_enablement_contract_ready_preview)) | length),
      stage_application_count: ($stage_applications | length),
      blocker_application_count: ($blocker_applications | length),
      application_group_count: ($application_groups | length),
      runtime_plan_stage_ref_count: ($application_plans | map(.expected_runtime_stage_ids | length) | add),
      evidence_field_ref_count: ($application_plans | map(.expected_evidence_field_ids | length) | add),
      stage_contract_ref_count: ($stage_applications | map(.expected_contract_ref_ids | length) | add),
      stage_source_ref_count: ($stage_applications | map(.affected_source_surface_ids | length) | add),
      blocker_mapping_source_ref_count: ($blocker_applications | map(.affected_source_surface_ids | length) | add),
      application_guard_count: ($application_guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      application_plans: $application_plans,
      source_outcomes: $source_outcomes,
      stage_applications: $stage_applications,
      blocker_applications: $blocker_applications,
      application_groups: $application_groups,
      application_guards: $application_guards,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview_gate",
      ready_for_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview: true,
      ready_for_append_only_store_enablement: false,
      ready_for_projection_enforcement: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        append_only_store_runtime_enablement_application: {
          rust_module_present: $application_rust_module_present,
          report_script_present: $application_report_script_present,
          gate_script_present: $application_gate_script_present
        },
        append_only_store_runtime_enablement_readback: {
          upstream_gate: ($readback.gate == "hepta_work_graph_append_only_store_runtime_enablement_readback_preview_gate"),
          gate_script_present: $readback_gate_script_present
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
        role_manifest_enforcement_enabled: false,
        task_result_enforcement_enabled: false,
        task_result_persisted: false,
        readback_executed: false,
        rollback_executed: false,
        runtime_application_promoted: false,
        approval_recorded: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
