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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-scheduler-admission-gap-closure-readback.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-scheduler-admission-enforcement-gap-closure-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-preview-report.sh" \
  >"$tmpdir/closure.json"

readback_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_scheduler_admission_enforcement_gap_closure_readback_preview.rs
)"
readback_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-readback-preview-report.sh
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-readback-preview-gate.sh
)"
closure_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-preview-gate.sh
)"

jq -n \
  --slurpfile closure "$tmpdir/closure.json" \
  --argjson readback_rust_module_present "$readback_rust_module_present" \
  --argjson readback_report_script_present "$readback_report_script_present" \
  --argjson readback_gate_script_present "$readback_gate_script_present" \
  --argjson closure_gate_script_present "$closure_gate_script_present" \
  '
  $closure[0] as $closure
  | def readback_id($source):
      "readback_" + $source + "_scheduler_admission_gap_closure";
  def binding_assertion_id($source):
      "assert_" + $source + "_scheduler_admission_binding_readback";
  def probe_assertion_id($source):
      "assert_" + $source + "_scheduler_admission_probe_readback";
  def evidence_assertion_id($source):
      "assert_" + $source + "_scheduler_admission_evidence_fields_readback";
  def guard_assertion_id($guard):
      "assert_" + $guard + "_readback";
  def blocker_assertion_id($blocker):
      "assert_" + $blocker + "_mapping_readback";
  def plan_readback($plan): {
      id: readback_id($plan.source_surface_id),
      source_surface_id: $plan.source_surface_id,
      closure_plan_id: $plan.closure_plan_id,
      source_category: $plan.source_category,
      target_node_kind: $plan.target_node_kind,
      scheduler_blocker_id: $plan.scheduler_blocker_id,
      readback_probe_id: $plan.readback_probe_id,
      source_fields: $plan.source_fields,
      controller_adapter_blocker_ids: $plan.controller_adapter_blocker_ids,
      admission_check_ids: $plan.admission_check_ids,
      admission_decision_ids: $plan.admission_decision_ids,
      required_evidence_fields: $plan.required_evidence_fields,
      required_before_closure_application: true,
      readback_state: "readback_assertions_defined_execution_disabled",
      performs_readback: false,
      enforces_scheduler_admission: false,
      starts_work: false,
      acquires_lease: false,
      consumes_budget: false,
      records_approval: false,
      mutates_idempotency_index: false,
      writes_store: false
    };
  def binding_assertion($plan): {
      id: binding_assertion_id($plan.source_surface_id),
      source_surface_id: $plan.source_surface_id,
      closure_plan_id: $plan.closure_plan_id,
      target_node_kind: $plan.target_node_kind,
      scheduler_blocker_id: $plan.scheduler_blocker_id,
      controller_adapter_blocker_ids: $plan.controller_adapter_blocker_ids,
      admission_check_ids: $plan.admission_check_ids,
      admission_decision_ids: $plan.admission_decision_ids,
      expected_binding_state: "scheduler_admission_binding_defined_enforcement_disabled",
      enforces_scheduler_admission: false,
      starts_work: false
    };
  def probe_assertion($plan): {
      id: probe_assertion_id($plan.source_surface_id),
      source_surface_id: $plan.source_surface_id,
      closure_plan_id: $plan.closure_plan_id,
      readback_probe_id: $plan.readback_probe_id,
      required_evidence_fields: $plan.required_evidence_fields,
      expected_probe_state: "scheduler_admission_readback_probe_defined_execution_disabled",
      performs_readback: false,
      persists_evidence: false,
      enforces_scheduler_admission: false
    };
  def evidence_assertion($plan): {
      id: evidence_assertion_id($plan.source_surface_id),
      source_surface_id: $plan.source_surface_id,
      closure_plan_id: $plan.closure_plan_id,
      required_evidence_fields: $plan.required_evidence_fields,
      required_field_count: ($plan.required_evidence_fields | length),
      expected_evidence_state: "scheduler_admission_evidence_contract_defined_no_readback_execution",
      performs_readback: false,
      enforces_scheduler_admission: false
    };
  def guard_assertion($guard): {
      id: guard_assertion_id($guard.id),
      guard_id: $guard.id,
      severity: $guard.severity,
      guard_scope: $guard.guard_scope,
      expected_guard_state: "guard_declared_satisfied_by_runtime_false",
      required_before_scheduler_admission_enforcement: $guard.required_before_scheduler_admission_enforcement,
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
      expected_blocker_state: "blocks_scheduler_admission_until_readback_and_application_preview",
      required_before_scheduler_admission_enforcement: $blocker.required_before_scheduler_admission_enforcement,
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
      required_before_scheduler_admission_enforcement: true,
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
  | ($closure.closure_plans | map(binding_assertion(.))) as $binding_assertions
  | ($closure.closure_plans | map(probe_assertion(.))) as $probe_assertions
  | ($closure.closure_plans | map(evidence_assertion(.))) as $evidence_assertions
  | ($closure.guards | map(guard_assertion(.))) as $guard_assertions
  | ($closure.blockers | map(blocker_assertion(.))) as $blocker_assertions
  | [
      drift("scheduler_admission_source_coverage_drift"; ["sourceSurfaceId","closurePlanId","targetNodeKind"]; "critical"),
      drift("scheduler_admission_check_binding_drift"; ["admissionCheckIds","admissionDecisionIds"]; "critical"),
      drift("scheduler_admission_evidence_field_drift"; ["requiredEvidenceFields","readbackProbeId"]; "critical"),
      drift("scheduler_admission_no_mutation_guard_drift"; ["performsReadback","acquiresLease","startsWork","recordsApproval","mutatesIdempotencyIndex"]; "critical"),
      drift("scheduler_admission_blocker_mapping_drift"; ["blockerId","affectedSourceSurfaceIds","affectedClosurePlanIds"]; "high"),
      drift("scheduler_admission_prior_gate_drift"; ["requiredPriorGates","closurePreviewGate"]; "medium")
    ] as $drift_detectors
  | ($readback_plans | map(.source_surface_id)) as $all_sources
  | ($readback_plans | map(.id)) as $all_readback_ids
  | ([readback_blocker("readback_execution_disabled"; "critical"; "readback_execution"; $all_sources; $all_readback_ids; "this preview defines scheduler admission readback assertions but does not execute readback")]
      + ($closure.blockers | map(select(.id != "scheduler_admission_closure_readback_missing") | readback_blocker_from_closure(.)))
      + [
          readback_blocker("scheduler_admission_closure_application_missing"; "high"; "application_preview"; $all_sources; $all_readback_ids; "run closure application preview after readback assertions are defined and reviewed"),
          readback_blocker("operator_review_required"; "medium"; "operator_review"; $all_sources; $all_readback_ids; "operator review must accept scheduler admission bindings, evidence fields, guards, and blockers before promotion")
        ]) as $readback_blockers
  | ($closure.required_prior_gates + (if ($closure.required_prior_gates | index($closure.gate)) then [] else [$closure.gate] end)) as $required_priors
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_scheduler_admission_enforcement_gap_closure_readback_preview_gate",
      schema_version: "work_graph_scheduler_admission_enforcement_gap_closure_readback_preview_v1",
      preview_mode: "read_only_scheduler_admission_gap_closure_readback_no_execution",
      closure_plan_count: $closure.closure_plan_count,
      admission_binding_count: $closure.admission_binding_count,
      readback_probe_binding_count: $closure.readback_probe_binding_count,
      readback_plan_count: ($readback_plans | length),
      admission_binding_assertion_count: ($binding_assertions | length),
      readback_probe_assertion_count: ($probe_assertions | length),
      evidence_field_assertion_count: ($evidence_assertions | length),
      guard_assertion_count: ($guard_assertions | length),
      blocker_mapping_assertion_count: ($blocker_assertions | length),
      admission_check_ref_count: ($readback_plans | map(.admission_check_ids | length) | add),
      admission_decision_ref_count: ($readback_plans | map(.admission_decision_ids | length) | add),
      evidence_field_ref_count: ($evidence_assertions | map(.required_field_count) | add),
      drift_detector_count: ($drift_detectors | length),
      blocker_count: ($readback_blockers | length),
      required_prior_gate_count: ($required_priors | length),
      readback_plans: $readback_plans,
      admission_binding_assertions: $binding_assertions,
      readback_probe_assertions: $probe_assertions,
      evidence_field_assertions: $evidence_assertions,
      guard_assertions: $guard_assertions,
      blocker_mapping_assertions: $blocker_assertions,
      drift_detectors: $drift_detectors,
      blockers: $readback_blockers,
      required_prior_gates: $required_priors,
      recommended_next_gate: "hepta_work_graph_scheduler_admission_enforcement_gap_closure_application_preview_gate",
      ready_for_scheduler_admission_closure_application_preview: true,
      ready_for_readback_execution: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_append_only_store_enablement: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_projection_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        scheduler_admission_gap_closure_readback: {
          rust_module_present: $readback_rust_module_present,
          report_script_present: $readback_report_script_present,
          gate_script_present: $readback_gate_script_present
        },
        scheduler_admission_gap_closure: {
          upstream_gate: ($closure.gate == "hepta_work_graph_scheduler_admission_enforcement_gap_closure_preview_gate"),
          gate_script_present: $closure_gate_script_present
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
