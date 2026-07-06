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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-append-only-store-precondition-readback.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-append-only-store-enablement-precondition-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-append-only-store-enablement-precondition-preview-report.sh" \
  >"$tmpdir/precondition.json"

readback_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_store_enablement_precondition_readback_preview.rs
)"
readback_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-enablement-precondition-readback-preview-report.sh
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-enablement-precondition-readback-preview-gate.sh
)"
precondition_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-enablement-precondition-preview-gate.sh
)"

jq -n \
  --slurpfile pre "$tmpdir/precondition.json" \
  --argjson readback_rust_module_present "$readback_rust_module_present" \
  --argjson readback_report_script_present "$readback_report_script_present" \
  --argjson readback_gate_script_present "$readback_gate_script_present" \
  --argjson precondition_gate_script_present "$precondition_gate_script_present" \
  '
  $pre[0] as $pre
  | def evidence_fields($category):
      if $category == "durable_store_switch" then
        ["preconditionId","sourceSurfaceIds","eventContractRefs","durableStoreSwitchState","operatorReviewRef"]
      elif $category == "wal_boundary" then
        ["preconditionId","walOperationIds","appendOrderingRule","walWriteBoundaryState","rollbackPlanRef"]
      elif $category == "idempotency_mutation_policy" then
        ["preconditionId","idempotencyGuardIds","collisionPolicyRefs","mutationPolicyState","replayProbeRef"]
      elif $category == "rollback_readback_gate" then
        ["preconditionId","checkpointContractRefs","readbackProbeRefs","rollbackPlanRef","replayDeterminismRef"]
      elif $category == "operator_review" then
        ["preconditionId","operatorReviewRef","sideEffectLockState","runtimeReceiptRefs","terminalTaskResultRefs"]
      elif $category == "scheduler_admission" then
        ["preconditionId","dependencyGateRef","leaseGateRef","budgetGateRef","approvalGateRef","idempotencyGateRef"]
      elif $category == "role_manifest" then
        ["preconditionId","roleCapabilityRef","toolPermissionRef","budgetLimitRef","laneBoundaryRef"]
      else ["preconditionId","unknownCategoryRef"]
      end;
  def scope_for($category):
      if $category == "durable_store_switch" then "durable_store_switch_state"
      elif $category == "wal_boundary" then "wal_boundary_contract_refs"
      elif $category == "idempotency_mutation_policy" then "idempotency_mutation_policy_refs"
      elif $category == "rollback_readback_gate" then "rollback_and_readback_probe_refs"
      elif $category == "operator_review" then "operator_review_and_side_effect_lock_refs"
      elif $category == "scheduler_admission" then "scheduler_admission_gate_refs"
      elif $category == "role_manifest" then "role_manifest_policy_refs"
      else "unknown_precondition_scope"
      end;
  def assertion_id($id; $suffix):
      if $suffix == "contract_refs" then ($id + "_contract_refs_readback")
      elif $suffix == "source_coverage" then ($id + "_source_coverage_readback")
      elif $suffix == "blocker_mapping" then ($id + "_blocker_mapping_readback")
      else ($id + "_unknown_readback")
      end;
  def plan($p): {
      precondition_id: $p.id,
      category: $p.category,
      severity: $p.severity,
      affected_source_surface_ids: $p.affected_source_surface_ids,
      expected_contract_ref_ids: $p.required_contract_refs,
      expected_blocker_id: $p.blocker_id,
      required_evidence_fields: evidence_fields($p.category),
      readback_scope: scope_for($p.category),
      expected_preview_state: (if $p.satisfied_by_preview_contracts then "preview_contract_ready_enablement_blocked" else "operator_or_enforcement_contract_missing_enablement_blocked" end),
      required_before_precondition_application: true,
      performs_readback: false,
      mutates_store: false,
      enables_append_only_store: false
    };
  def contract_assertion($p): {
      assertion_id: assertion_id($p.id; "contract_refs"),
      precondition_id: $p.id,
      category: $p.category,
      expected_contract_ref_ids: $p.required_contract_refs,
      expected_contract_ref_count: ($p.required_contract_refs | length),
      expected_contract_ref_state: (if $p.satisfied_by_preview_contracts then "preview_contract_refs_present_enablement_still_disabled" else "operator_or_enforcement_contract_refs_missing_for_enablement" end),
      performs_readback: false,
      mutates_store: false
    };
  def source_assertion($p): {
      assertion_id: assertion_id($p.id; "source_coverage"),
      precondition_id: $p.id,
      category: $p.category,
      expected_source_surface_ids: $p.affected_source_surface_ids,
      expected_source_surface_count: ($p.affected_source_surface_ids | length),
      expected_coverage_state: "source_coverage_declared_readback_not_executed",
      performs_readback: false,
      mutates_store: false
    };
  def blocker_assertion($b): {
      assertion_id: assertion_id($b.id; "blocker_mapping"),
      blocker_id: $b.id,
      category: $b.category,
      affected_precondition_ids: $b.affected_precondition_ids,
      affected_source_surface_ids: $b.affected_source_surface_ids,
      expected_blocker_state: "blocks_append_only_store_enablement_until_readback_and_application_preview",
      required_before_append_only_store_enablement: $b.required_before_append_only_store_enablement,
      performs_readback: false,
      mutates_store: false
    };
  def readback_blocker($id; $severity; $category; $preconditions; $sources; $fix): {
      id: $id,
      severity: $severity,
      category: $category,
      affected_precondition_ids: $preconditions,
      affected_source_surface_ids: $sources,
      required_before_precondition_application: true,
      recommended_fix: $fix
    };
  def blocker_from_precondition($b): readback_blocker($b.id; $b.severity; $b.category; $b.affected_precondition_ids; $b.affected_source_surface_ids; $b.recommended_fix);
  def drift($id; $fields; $severity): {
      id: $id,
      compared_field_ids: $fields,
      severity: $severity,
      blocks_precondition_application: true,
      performs_readback: false
    };
  ($pre.preconditions | map(plan(.))) as $plans
  | ($pre.preconditions | map(contract_assertion(.))) as $contract_assertions
  | ($pre.preconditions | map(source_assertion(.))) as $source_assertions
  | ($pre.blockers | map(blocker_assertion(.))) as $blocker_assertions
  | [
      drift("append_only_precondition_source_coverage_drift"; ["precondition_id","affected_source_surface_ids","source_precondition_decisions"]; "critical"),
      drift("append_only_precondition_contract_ref_drift"; ["required_contract_refs","append_only_event_contracts","wal_operations","idempotency_guards","readback_probes"]; "critical"),
      drift("append_only_precondition_blocker_mapping_drift"; ["blocker_id","affected_precondition_ids","affected_source_surface_ids"]; "high"),
      drift("append_only_precondition_side_effect_lock_drift"; ["side_effects","append_only_store_enabled","wal_written","readback_executed"]; "critical"),
      drift("append_only_precondition_decision_distribution_drift"; ["append_only_precondition_decision","scheduler_admission_not_enforced","role_manifest_not_enforced"]; "medium"),
      drift("append_only_precondition_prior_gate_drift"; ["required_prior_gates","precondition_gate","terminal_task_result_readiness_rerun_gate"]; "medium")
    ] as $drift_detectors
  | ($pre.preconditions | map(.id)) as $all_precondition_ids
  | ($pre.source_precondition_decisions | map(.source_surface_id)) as $all_sources
  | ([readback_blocker("readback_execution_disabled"; "critical"; "readback_execution"; $all_precondition_ids; $all_sources; "keep this gate preview-only until readback execution and rollback fixtures are explicitly promoted")]
     + ($pre.blockers | map(blocker_from_precondition(.)))) as $readback_blockers
  | ($pre.required_prior_gates + (if ($pre.required_prior_gates | index($pre.gate)) then [] else [$pre.gate] end)) as $required_priors
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_append_only_store_enablement_precondition_readback_preview_gate",
      schema_version: "work_graph_append_only_store_enablement_precondition_readback_preview_v1",
      preview_mode: "read_only_append_only_store_enablement_precondition_readback_preview_no_execution",
      precondition_count: ($pre.preconditions | length),
      source_precondition_decision_count: ($pre.source_precondition_decisions | length),
      readback_plan_count: ($plans | length),
      contract_ref_assertion_count: ($contract_assertions | length),
      source_coverage_assertion_count: ($source_assertions | length),
      blocker_mapping_assertion_count: ($blocker_assertions | length),
      readback_evidence_field_ref_count: ($plans | map(.required_evidence_fields | length) | add),
      contract_ref_count: ($contract_assertions | map(.expected_contract_ref_count) | add),
      precondition_source_ref_count: ($source_assertions | map(.expected_source_surface_count) | add),
      drift_detector_count: ($drift_detectors | length),
      blocker_count: ($readback_blockers | length),
      required_prior_gate_count: ($required_priors | length),
      readback_plans: $plans,
      contract_ref_assertions: $contract_assertions,
      source_coverage_assertions: $source_assertions,
      blocker_mapping_assertions: $blocker_assertions,
      drift_detectors: $drift_detectors,
      blockers: $readback_blockers,
      required_prior_gates: $required_priors,
      recommended_next_gate: "hepta_work_graph_append_only_store_enablement_precondition_application_preview_gate",
      ready_for_precondition_application_preview: true,
      ready_for_append_only_store_enablement: false,
      ready_for_projection_enforcement: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        append_only_store_enablement_precondition_readback: {
          rust_module_present: $readback_rust_module_present,
          report_script_present: $readback_report_script_present,
          gate_script_present: $readback_gate_script_present
        },
        append_only_store_enablement_precondition: {
          gate_script_present: $precondition_gate_script_present,
          upstream_gate: ($pre.gate == "hepta_work_graph_append_only_store_enablement_precondition_preview_gate")
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
