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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-runtime-application-promotion-readback.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-runtime-application-promotion-gap-closure-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-runtime-application-promotion-gap-closure-preview-report.sh" \
  >"$tmpdir/closure.json"

readback_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_runtime_application_promotion_gap_closure_readback_preview.rs
)"
readback_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-runtime-application-promotion-gap-closure-readback-preview-report.sh
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-runtime-application-promotion-gap-closure-readback-preview-gate.sh
)"
closure_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-runtime-application-promotion-gap-closure-preview-gate.sh
)"

jq -n \
  --slurpfile closure "$tmpdir/closure.json" \
  --argjson readback_rust_module_present "$readback_rust_module_present" \
  --argjson readback_report_script_present "$readback_report_script_present" \
  --argjson readback_gate_script_present "$readback_gate_script_present" \
  --argjson closure_gate_script_present "$closure_gate_script_present" \
  '
  $closure[0] as $closure
  | def readback_plan($plan): {
      id: ("runtime_application_promotion_closure_readback_plan__" + $plan.source_surface_id),
      source_surface_id: $plan.source_surface_id,
      closure_plan_id: $plan.closure_plan_id,
      source_category: $plan.source_category,
      runtime_rerun_decision: $plan.runtime_rerun_decision,
      promotion_domain_ids: $plan.promotion_domain_ids,
      promotion_binding_ids: $plan.promotion_binding_ids,
      readback_probe_id: $plan.readback_probe_id,
      required_evidence_fields: $plan.evidence_field_ids,
      residual_source_blocker_ids: $plan.residual_source_blocker_ids,
      required_before_closure_application: true,
      readback_state: "asserted_from_closure_preview_no_execution",
      performs_readback: false,
      promotes_runtime_application: false,
      attaches_runtime_wrapper: false,
      enforces_scheduler_admission: false,
      enforces_role_manifest: false,
      mutates_store: false,
      writes_wal: false,
      records_approval: false
    };
  def affected_readback_plan_ids($sources; $plans):
      [$plans[] as $plan | select($sources | index($plan.source_surface_id)) | $plan.id];
  def binding_assertion($binding): {
      id: ("runtime_application_promotion_binding_readback_assertion__" + $binding.id),
      source_surface_id: $binding.source_surface_id,
      binding_id: $binding.id,
      promotion_domain_id: $binding.promotion_domain_id,
      closes_blocker_id: $binding.closes_blocker_id,
      required_evidence_field_ids: $binding.required_evidence_field_ids,
      expected_binding_state: "readback_verified_no_mutation",
      promotes_runtime_application: false,
      writes_store: false
    };
  def group_assertion($group): {
      id: ("runtime_application_promotion_group_readback_assertion__" + $group.id),
      group_id: $group.id,
      promotion_domain_id: $group.promotion_domain_id,
      affected_source_surface_ids: $group.affected_source_surface_ids,
      closure_plan_ids: $group.closure_plan_ids,
      promotion_binding_ids: $group.promotion_binding_ids,
      expected_contract_count_after_closure: $group.expected_contract_count_after_closure,
      expected_group_state: "readback_verified_no_mutation",
      promotes_runtime_application: false
    };
  def probe_assertion($plan): {
      id: ("runtime_application_promotion_probe_readback_assertion__" + $plan.source_surface_id),
      source_surface_id: $plan.source_surface_id,
      closure_plan_id: $plan.closure_plan_id,
      readback_probe_id: $plan.readback_probe_id,
      required_evidence_fields: $plan.required_evidence_fields,
      expected_probe_state: "readback_contract_declared_not_executed",
      performs_readback: false,
      persists_evidence: false,
      promotes_runtime_application: false
    };
  def evidence_assertion($plan): {
      id: ("runtime_application_promotion_evidence_field_readback_assertion__" + $plan.source_surface_id),
      source_surface_id: $plan.source_surface_id,
      closure_plan_id: $plan.closure_plan_id,
      required_evidence_fields: $plan.required_evidence_fields,
      required_field_count: ($plan.required_evidence_fields | length),
      expected_evidence_state: "evidence_fields_declared_not_persisted",
      performs_readback: false,
      promotes_runtime_application: false
    };
  def guard_assertion($guard): {
      id: ("runtime_application_promotion_guard_readback_assertion__" + $guard.id),
      guard_id: $guard.id,
      severity: $guard.severity,
      guard_scope: $guard.scope,
      expected_guard_state: "guard_declared_and_runtime_mutation_prevented",
      required_before_runtime_application_promotion: true,
      satisfied_by_readback_preview: true,
      mutates_runtime: false
    };
  def blocker_assertion($blocker; $plans): {
      id: ("runtime_application_promotion_blocker_mapping_readback_assertion__" + $blocker.id),
      blocker_id: $blocker.id,
      severity: $blocker.severity,
      affected_source_surface_ids: $blocker.affected_source_surface_ids,
      affected_readback_plan_ids: affected_readback_plan_ids($blocker.affected_source_surface_ids; $plans),
      blocked_promotion_domain_ids: $blocker.blocked_promotion_domain_ids,
      expected_blocker_state: "blocker_mapping_readback_verified_no_mutation",
      required_before_runtime_application_promotion: true,
      performs_readback: false,
      mutates_runtime: false
    };
  def drift_detector($id; $fields): {
      id: $id,
      compared_field_ids: $fields,
      severity: "high",
      blocks_closure_application: true,
      performs_readback: false
    };
  ($closure.promotion_plans | map(readback_plan(.))) as $readback_plans
  | ($closure.blockers + [{
      id: "runtime_application_promotion_closure_application_missing",
      severity: "high",
      affected_source_surface_ids: ($closure.promotion_plans | map(.source_surface_id)),
      blocked_promotion_domain_ids: ($closure.promotion_groups | map(.promotion_domain_id)),
      required_before_runtime_application_promotion: true,
      recommended_fix: "apply readback-verified runtime application promotion plans before readiness rerun"
    }]) as $blockers
  | ($closure.required_prior_gates + [$closure.gate]) as $required_prior_gates
  | ($closure.promotion_bindings | map(binding_assertion(.))) as $binding_assertions
  | ($closure.promotion_groups | map(group_assertion(.))) as $group_assertions
  | ($readback_plans | map(probe_assertion(.))) as $probe_assertions
  | ($readback_plans | map(evidence_assertion(.))) as $evidence_assertions
  | ($closure.guards | map(guard_assertion(.))) as $guard_assertions
  | ($blockers | map(blocker_assertion(. ; $readback_plans))) as $blocker_assertions
  | ([
      drift_detector("source_surface_alignment"; ["source_surface_id", "source_category"]),
      drift_detector("promotion_binding_alignment"; ["promotion_binding_ids", "closes_blocker_id"]),
      drift_detector("promotion_group_alignment"; ["promotion_domain_id", "affected_source_surface_ids"]),
      drift_detector("evidence_field_alignment"; ["required_evidence_fields", "required_field_count"]),
      drift_detector("guard_no_mutation_alignment"; ["guard_id", "mutates_runtime"]),
      drift_detector("blocker_mapping_alignment"; ["blocker_id", "affected_readback_plan_ids"]),
      drift_detector("side_effect_boundary_alignment"; ["side_effects", "runtime_mutation_performed"])
    ]) as $drift_detectors
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_runtime_application_promotion_gap_closure_readback_preview_gate",
      schema_version: "work_graph_runtime_application_promotion_gap_closure_readback_preview_v1",
      preview_mode: "read_only_runtime_application_promotion_gap_closure_readback_no_execution",
      closure_plan_count: ($closure.promotion_plans | length),
      promotion_binding_count: ($closure.promotion_bindings | length),
      promotion_group_count: ($closure.promotion_groups | length),
      readback_probe_binding_count: ($closure.promotion_plans | length),
      readback_plan_count: ($readback_plans | length),
      promotion_binding_assertion_count: ($binding_assertions | length),
      promotion_group_assertion_count: ($group_assertions | length),
      readback_probe_assertion_count: ($probe_assertions | length),
      evidence_field_assertion_count: ($evidence_assertions | length),
      guard_assertion_count: ($guard_assertions | length),
      blocker_mapping_assertion_count: ($blocker_assertions | length),
      promotion_domain_ref_count: ($readback_plans | map(.promotion_domain_ids | length) | add),
      promotion_binding_ref_count: ($readback_plans | map(.promotion_binding_ids | length) | add),
      evidence_field_ref_count: ($evidence_assertions | map(.required_field_count) | add),
      group_source_ref_count: ($closure.promotion_groups | map(.affected_source_surface_ids | length) | add),
      drift_detector_count: ($drift_detectors | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_plans: $readback_plans,
      promotion_binding_assertions: $binding_assertions,
      promotion_group_assertions: $group_assertions,
      readback_probe_assertions: $probe_assertions,
      evidence_field_assertions: $evidence_assertions,
      guard_assertions: $guard_assertions,
      blocker_mapping_assertions: $blocker_assertions,
      drift_detectors: $drift_detectors,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_runtime_application_promotion_gap_closure_application_preview_gate",
      ready_for_runtime_application_promotion_closure_application_preview: true,
      ready_for_readback_execution: false,
      ready_for_runtime_application_promotion: false,
      ready_for_operator_review_side_effect_lock: false,
      ready_for_append_only_store_enablement: false,
      ready_for_projection_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        runtime_application_promotion_gap_closure_readback: {
          rust_module_present: $readback_rust_module_present,
          report_script_present: $readback_report_script_present,
          gate_script_present: $readback_gate_script_present
        },
        runtime_application_promotion_gap_closure: {
          upstream_gate: ($closure.gate == "hepta_work_graph_runtime_application_promotion_gap_closure_preview_gate"),
          gate_script_present: $closure_gate_script_present,
          recommended_next_matches: ($closure.recommended_next_gate == "hepta_work_graph_runtime_application_promotion_gap_closure_readback_preview_gate")
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
