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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-operator-review-side-effect-lock-application.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$tmpdir/cache"
fi

capture_json_report \
  "hepta-work-graph-append-only-store-operator-review-side-effect-lock-readback-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-append-only-store-operator-review-side-effect-lock-readback-preview-report.sh" \
  >"$tmpdir/readback.json"

application_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_store_operator_review_side_effect_lock_application_preview.rs
)"
application_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-operator-review-side-effect-lock-application-preview-report.sh
)"
application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-operator-review-side-effect-lock-application-preview-gate.sh
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-operator-review-side-effect-lock-readback-preview-gate.sh
)"

jq -n \
  --slurpfile readback "$tmpdir/readback.json" \
  --argjson application_rust_module_present "$application_rust_module_present" \
  --argjson application_report_script_present "$application_report_script_present" \
  --argjson application_gate_script_present "$application_gate_script_present" \
  --argjson readback_gate_script_present "$readback_gate_script_present" \
  '
  $readback[0] as $readback
  | def application_plan_id($readback_plan_id):
      "apply_" + $readback_plan_id + "_operator_review_side_effect_lock_preview";
  def application_plan_ids_for_sources($sources; $plans):
      [$plans[] as $plan | select($sources | index($plan.source_surface_id)) | $plan.application_plan_id];
  def application_plan_ids_for_readback_plans($readback_plan_ids; $plans):
      [$plans[] as $plan | select($readback_plan_ids | index($plan.readback_plan_id)) | $plan.application_plan_id];
  def application_guard($id; $severity; $scope): {
      id: $id,
      severity: $severity,
      guard_scope: $scope,
      required_before_operator_review_side_effect_lock: true,
      satisfied_by_preview: false
    };
  def application_blocker($id; $severity; $category; $sources; $plans; $fix): {
      id: $id,
      severity: $severity,
      category: $category,
      affected_source_surface_ids: $sources,
      affected_application_plan_ids: application_plan_ids_for_sources($sources; $plans),
      required_before_operator_review_side_effect_lock: true,
      recommended_fix: $fix
    };
  ($readback.readback_plans | map({
      application_plan_id: application_plan_id(.id),
      readback_plan_id: .id,
      source_surface_id: .source_surface_id,
      source_category: .source_category,
      operator_review_packet_id: .operator_review_packet_id,
      side_effect_lock_plan_id: .side_effect_lock_plan_id,
      approval_evidence_boundary_id: .approval_evidence_boundary_id,
      readback_boundary_id: .readback_boundary_id,
      expected_evidence_field_ids: .required_evidence_fields,
      lock_scope_ids: .lock_scope_ids,
      application_scope: "operator_review_side_effect_lock_application_binding",
      application_state: "preview_application_defined_operator_review_and_lock_not_recorded",
      readback_verified_by_preview: true,
      operator_review_contract_ready_preview: true,
      side_effect_lock_contract_ready_preview: true,
      records_operator_review: false,
      records_approval: false,
      establishes_side_effect_lock: false,
      executes_readback: false,
      writes_store: false,
      writes_wal: false,
      mutates_runtime: false
    })) as $application_plans
  | ($application_plans | map({
      source_surface_id: .source_surface_id,
      source_category: .source_category,
      application_plan_id: .application_plan_id,
      post_application_operator_review_state: "operator_review_side_effect_lock_contract_ready_preview_after_application",
      operator_review_contract_ready_preview: true,
      side_effect_lock_contract_ready_preview: true,
      ready_for_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview: true,
      ready_for_operator_review_recording: false,
      applies_to_runtime: false
    })) as $source_outcomes
  | ($readback.packet_assertions | map({
      application_id: ("apply_" + .packet_id + "_operator_review_packet_preview"),
      packet_id: .packet_id,
      source_surface_id: .source_surface_id,
      required_section_ids: .required_section_ids,
      required_evidence_field_ids: .required_evidence_field_ids,
      expected_packet_state: "packet_contract_ready_preview_after_application_not_recorded",
      packet_contract_ready_preview: true,
      readback_verified_by_preview: true,
      records_operator_review: false,
      records_approval: false
    })) as $packet_applications
  | ($readback.side_effect_lock_assertions | map({
      application_id: ("apply_" + .lock_plan_id + "_side_effect_lock_preview"),
      lock_plan_id: .lock_plan_id,
      source_surface_id: .source_surface_id,
      lock_scope_ids: .lock_scope_ids,
      expected_lock_state: "side_effect_lock_contract_ready_preview_after_application_not_established",
      lock_contract_ready_preview: true,
      readback_verified_by_preview: true,
      establishes_side_effect_lock: false,
      mutates_runtime: false
    })) as $lock_applications
  | ($readback.approval_boundary_assertions | map({
      application_id: ("apply_" + .boundary_id + "_approval_evidence_preview"),
      boundary_id: .boundary_id,
      source_surface_id: .source_surface_id,
      required_evidence_field_ids: .required_evidence_field_ids,
      expected_boundary_state: "approval_evidence_contract_ready_preview_after_application_not_recorded",
      boundary_contract_ready_preview: true,
      readback_verified_by_preview: true,
      records_operator_review: false,
      records_approval: false,
      persists_receipt: false
    })) as $approval_applications
  | ($readback.readback_boundary_assertions | map({
      application_id: ("apply_" + .boundary_id + "_readback_boundary_preview"),
      boundary_id: .boundary_id,
      readback_probe_id: .readback_probe_id,
      source_surface_id: .source_surface_id,
      expected_boundary_state: "readback_boundary_contract_ready_preview_after_application_not_executed",
      boundary_contract_ready_preview: true,
      readback_verified_by_preview: true,
      executes_readback: false,
      rollback_executed: false,
      writes_checkpoint: false
    })) as $readback_boundary_applications
  | ($readback.group_assertions | map({
      application_id: ("apply_" + .group_id + "_operator_review_group_preview"),
      group_id: .group_id,
      source_category: .source_category,
      affected_source_surface_ids: .affected_source_surface_ids,
      application_plan_ids: application_plan_ids_for_sources(.affected_source_surface_ids; $application_plans),
      operator_review_packet_ids: .operator_review_packet_ids,
      side_effect_lock_plan_ids: .side_effect_lock_plan_ids,
      expected_contract_count_after_application: .expected_review_packet_count,
      group_contract_ready_preview: true,
      readback_verified_by_preview: true,
      records_operator_review: false,
      establishes_side_effect_lock: false
    })) as $group_applications
  | ($readback.blocker_mapping_assertions | map({
      application_id: ("apply_" + .blocker_id + "_operator_review_side_effect_lock_blocker_preview"),
      blocker_id: .blocker_id,
      severity: .severity,
      affected_source_surface_ids: .affected_source_surface_ids,
      affected_readback_plan_ids: .affected_readback_plan_ids,
      affected_application_plan_ids: application_plan_ids_for_readback_plans(.affected_readback_plan_ids; $application_plans),
      expected_blocker_state: "blocker_mapping_contract_ready_preview_after_application_runtime_still_blocked",
      blocker_contract_ready_preview: true,
      readback_verified_by_preview: true,
      clears_operator_review_blocker: false,
      clears_side_effect_lock_blocker: false,
      mutates_runtime: false
    })) as $blocker_applications
  | ([
      application_guard("operator_review_side_effect_lock_application_is_preview_only"; "medium"; "application_preview"),
      application_guard("readback_execution_disabled"; "critical"; "readback"),
      application_guard("operator_review_recording_disabled"; "high"; "operator_review"),
      application_guard("approval_recording_disabled"; "high"; "approval"),
      application_guard("side_effect_lock_establishment_disabled"; "critical"; "side_effect_lock"),
      application_guard("wal_write_boundary_disabled"; "critical"; "wal_boundary"),
      application_guard("durable_store_runtime_switch_disabled"; "critical"; "durable_store_switch"),
      application_guard("idempotency_mutation_disabled"; "critical"; "idempotency"),
      application_guard("rollback_readback_execution_disabled"; "critical"; "rollback_readback"),
      application_guard("append_only_store_enablement_disabled"; "critical"; "append_only_store"),
      application_guard("runtime_mutation_disabled"; "critical"; "runtime_mutation"),
      application_guard("model_invocation_disabled"; "high"; "model_boundary")
    ]) as $application_guards
  | (($readback.blockers | map(application_blocker(
        .id;
        .severity;
        "operator_review_side_effect_lock";
        .affected_source_surface_ids;
        $application_plans;
        .recommended_fix
      ))) + [
      application_blocker(
        "operator_review_side_effect_lock_readiness_rerun_missing";
        "high";
        "readiness_rerun";
        ($application_plans | map(.source_surface_id));
        $application_plans;
        "rerun unified projection enforcement-readiness against operator-review side-effect lock application preview outcomes"
      )
    ]) as $blockers
  | ($readback.required_prior_gates + [$readback.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_append_only_store_operator_review_side_effect_lock_application_preview_gate",
      schema_version: "work_graph_append_only_store_operator_review_side_effect_lock_application_preview_v1",
      preview_mode: "read_only_append_only_store_operator_review_side_effect_lock_application_no_runtime_mutation",
      readback_plan_count: $readback.readback_plan_count,
      application_plan_count: ($application_plans | length),
      source_outcome_count: ($source_outcomes | length),
      operator_review_contract_ready_preview_count: ($source_outcomes | map(select(.operator_review_contract_ready_preview)) | length),
      side_effect_lock_contract_ready_preview_count: ($source_outcomes | map(select(.side_effect_lock_contract_ready_preview)) | length),
      operator_review_packet_application_count: ($packet_applications | length),
      side_effect_lock_application_count: ($lock_applications | length),
      approval_boundary_application_count: ($approval_applications | length),
      readback_boundary_application_count: ($readback_boundary_applications | length),
      group_application_count: ($group_applications | length),
      blocker_application_count: ($blocker_applications | length),
      application_guard_count: ($application_guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      evidence_field_ref_count: ($application_plans | map(.expected_evidence_field_ids | length) | add),
      lock_scope_ref_count: ($application_plans | map(.lock_scope_ids | length) | add),
      group_source_ref_count: ($group_applications | map(.affected_source_surface_ids | length) | add),
      blocker_mapping_source_ref_count: ($blocker_applications | map(.affected_source_surface_ids | length) | add),
      application_plans: $application_plans,
      source_outcomes: $source_outcomes,
      packet_applications: $packet_applications,
      side_effect_lock_applications: $lock_applications,
      approval_boundary_applications: $approval_applications,
      readback_boundary_applications: $readback_boundary_applications,
      group_applications: $group_applications,
      blocker_applications: $blocker_applications,
      application_guards: $application_guards,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview_gate",
      ready_for_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview: true,
      ready_for_operator_review_recording: false,
      ready_for_side_effect_lock_establishment: false,
      ready_for_runtime_write_boundary_preview: false,
      ready_for_append_only_store_enablement: false,
      ready_for_projection_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        operator_review_side_effect_lock_application: {
          rust_module_present: $application_rust_module_present,
          report_script_present: $application_report_script_present,
          gate_script_present: $application_gate_script_present
        },
        operator_review_side_effect_lock_readback: {
          upstream_gate: ($readback.gate == "hepta_work_graph_append_only_store_operator_review_side_effect_lock_readback_preview_gate"),
          gate_script_present: $readback_gate_script_present,
          recommended_next_matches: ($readback.recommended_next_gate == "hepta_work_graph_append_only_store_operator_review_side_effect_lock_application_preview_gate")
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
        role_manifest_enforced: false,
        task_result_enforcement_enabled: false,
        task_result_persisted: false,
        approval_recorded: false,
        operator_review_recorded: false,
        side_effect_lock_established: false,
        readback_executed: false,
        rollback_executed: false,
        runtime_mutation_performed: false,
        external_send_performed: false,
        model_invoked: false,
        agent_spawn_performed: false
      }
    }
  '
