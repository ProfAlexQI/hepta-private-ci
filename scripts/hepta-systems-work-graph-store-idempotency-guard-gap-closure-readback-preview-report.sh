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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-store-guard-gap-closure-readback.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-store-idempotency-guard-gap-closure-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-store-idempotency-guard-gap-closure-preview-report.sh" \
  >"$tmpdir/store_guard_closure.json"

readback_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_store_idempotency_guard_gap_closure_readback_preview.rs
)"
readback_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-store-idempotency-guard-gap-closure-readback-preview-report.sh
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-store-idempotency-guard-gap-closure-readback-preview-gate.sh
)"
closure_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-store-idempotency-guard-gap-closure-preview-gate.sh
)"

jq -n \
  --slurpfile closure "$tmpdir/store_guard_closure.json" \
  --argjson readback_rust_module_present "$readback_rust_module_present" \
  --argjson readback_report_script_present "$readback_report_script_present" \
  --argjson readback_gate_script_present "$readback_gate_script_present" \
  --argjson closure_gate_script_present "$closure_gate_script_present" \
  '
  $closure[0] as $closure
  | def readback_plan_id($source):
      $source + "_store_guard_readback_plan";
  def key_formula_assertion_id($source):
      $source + "_key_formula_readback_assertion";
  def collision_policy_assertion_id($source):
      $source + "_collision_policy_readback_assertion";
  def probe_binding_assertion_id($source):
      $source + "_probe_binding_readback_assertion";
  def collection_ref_assertion_id($source):
      $source + "_collection_ref_readback_assertion";
  def guard_for($id):
      [$closure.candidate_guards[] | select(.id == $id)][0];
  def binding_for($source):
      [$closure.guard_bindings[] | select(.source_surface_id == $source)][0];
  def probe_binding_for($source):
      [$closure.guard_probe_bindings[] | select(.source_surface_id == $source)][0];
  def readback_plan($plan):
      (guard_for($plan.candidate_guard_id)) as $guard
      | {
          id: readback_plan_id($plan.source_surface_id),
          source_surface_id: $plan.source_surface_id,
          closure_plan_id: $plan.id,
          candidate_guard_id: $plan.candidate_guard_id,
          key_formula_assertion_id: key_formula_assertion_id($plan.source_surface_id),
          collision_policy_assertion_id: collision_policy_assertion_id($plan.source_surface_id),
          probe_binding_assertion_id: probe_binding_assertion_id($plan.source_surface_id),
          collection_ref_assertion_id: collection_ref_assertion_id($plan.source_surface_id),
          expected_key_fields: $guard.key_fields,
          expected_collection_ids: $plan.expected_collection_ids,
          readback_probe_contract_ids: $plan.readback_probe_contract_ids,
          required_before_runtime_guard_application: true,
          readback_state: "readback_assertions_defined_execution_disabled",
          performs_readback: false,
          mutates_store: false
        };
  def key_formula_assertion($guard): {
      id: key_formula_assertion_id($guard.source_surface_id),
      source_surface_id: $guard.source_surface_id,
      candidate_guard_id: $guard.id,
      key_fields: $guard.key_fields,
      key_formula: $guard.key_formula,
      replay_scope: $guard.replay_scope,
      redaction_policy: $guard.redaction_policy,
      requires_sha256_formula: ($guard.key_formula | startswith("sha256(")),
      mutates_idempotency_index: false
    };
  def collision_policy_assertion($guard): {
      id: collision_policy_assertion_id($guard.source_surface_id),
      source_surface_id: $guard.source_surface_id,
      candidate_guard_id: $guard.id,
      collision_policy: $guard.collision_policy,
      required_before_append_only_intake: $guard.required_before_append_only_intake,
      expected_collision_state: "collision_blocks_duplicate_projection_preview_only",
      mutates_idempotency_index: false
    };
  def probe_binding_assertion($binding): {
      id: probe_binding_assertion_id($binding.source_surface_id),
      source_surface_id: $binding.source_surface_id,
      candidate_guard_id: $binding.candidate_guard_id,
      readback_probe_contract_ids: $binding.readback_probe_contract_ids,
      target_collection_ids: $binding.target_collection_ids,
      readback_evidence_fields: $binding.readback_evidence_fields,
      drift_detector_ids: $binding.drift_detector_ids,
      expected_probe_binding_state: "probe_contract_shape_defined_readback_disabled",
      performs_readback: false,
      mutates_store: false
    };
  def collection_ref_assertion($binding):
      (probe_binding_for($binding.source_surface_id)) as $probe
      | {
          id: collection_ref_assertion_id($binding.source_surface_id),
          source_surface_id: $binding.source_surface_id,
          candidate_guard_id: $binding.candidate_guard_id,
          expected_collection_ids: $binding.expected_collection_ids,
          required_collection_count: ($binding.expected_collection_ids | length),
          required_readback_probe_contract_ids: $probe.readback_probe_contract_ids,
          expected_guard_binding_state: $binding.closure_state,
          mutates_store: false
        };
  def drift_detector($id; $fields; $severity): {
      id: $id,
      compared_field_ids: $fields,
      severity: $severity,
      blocks_runtime_guard_application: true,
      performs_readback: false
    };
  def blocker($id; $severity; $sources; $fix): {
      id: $id,
      severity: $severity,
      affected_source_surface_ids: $sources,
      required_before_projection_enforcement: true,
      recommended_fix: $fix
    };
  ($closure.closure_plans | map(readback_plan(.))) as $readback_plans
  | ($closure.candidate_guards | map(key_formula_assertion(.))) as $key_formula_assertions
  | ($closure.candidate_guards | map(collision_policy_assertion(.))) as $collision_policy_assertions
  | ($closure.guard_probe_bindings | map(probe_binding_assertion(.))) as $probe_binding_assertions
  | ($closure.guard_bindings | map(collection_ref_assertion(.))) as $collection_ref_assertions
  | [
      drift_detector("store_guard_key_formula_drift"; ["keyFields", "keyFormula", "replayScope"]; "high"),
      drift_detector("store_guard_collision_policy_drift"; ["collisionPolicy", "requiredBeforeAppendOnlyIntake"]; "high"),
      drift_detector("store_guard_probe_contract_drift"; ["readbackProbeContractIds", "readbackEvidenceFields"]; "high"),
      drift_detector("store_guard_collection_ref_drift"; ["expectedCollectionIds", "targetCollectionIds"]; "medium"),
      drift_detector("store_guard_redaction_policy_drift"; ["redactionPolicy", "driftDetectorIds"]; "medium")
    ] as $drift_detectors
  | ($closure.closure_plans | map(.source_surface_id)) as $source_ids
  | [
      blocker("readback_execution_disabled"; "high"; $source_ids; "this preview defines readback assertions but does not query or mutate the WorkGraph store"),
      blocker("runtime_guard_application_disabled"; "high"; $source_ids; "runtime guard application remains disabled until readback assertions are promoted"),
      blocker("state_store_guard_persistence_disabled"; "high"; $source_ids; "candidate guard rows remain preview-only and are not persisted to state store"),
      blocker("append_only_store_enablement_disabled"; "high"; $source_ids; "append-only store enablement remains blocked until guard readback and operator review pass"),
      blocker("task_result_enforcement_disabled"; "high"; ["hepta_runtime_multi_agent_reducer", "hepta_runtime_task_board"]; "TaskResult-producing guard assertions still need terminal TaskResult enforcement before runtime use"),
      blocker("operator_review_required"; "medium"; $source_ids; "operator review must accept guard formulas, collision policy, and redaction before application preview promotion")
    ] as $blockers
  | ($closure.required_prior_gates + [$closure.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_store_idempotency_guard_gap_closure_readback_preview_gate",
      schema_version: "work_graph_store_idempotency_guard_gap_closure_readback_preview_v1",
      preview_mode: "read_only_store_idempotency_guard_gap_closure_readback_no_execution",
      closure_plan_count: $closure.closure_plan_count,
      candidate_guard_count: $closure.candidate_guard_count,
      guard_binding_count: $closure.guard_binding_count,
      guard_probe_binding_count: $closure.guard_probe_binding_count,
      readback_plan_count: ($readback_plans | length),
      key_formula_assertion_count: ($key_formula_assertions | length),
      collision_policy_assertion_count: ($collision_policy_assertions | length),
      probe_binding_assertion_count: ($probe_binding_assertions | length),
      collection_ref_assertion_count: ($collection_ref_assertions | length),
      expected_collection_ref_count: ($collection_ref_assertions | map(.expected_collection_ids | length) | add),
      readback_probe_contract_ref_count: ($probe_binding_assertions | map(.readback_probe_contract_ids | length) | add),
      readback_evidence_field_ref_count: ($probe_binding_assertions | map(.readback_evidence_fields | length) | add),
      drift_detector_count: ($drift_detectors | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_plans: $readback_plans,
      key_formula_assertions: $key_formula_assertions,
      collision_policy_assertions: $collision_policy_assertions,
      probe_binding_assertions: $probe_binding_assertions,
      collection_ref_assertions: $collection_ref_assertions,
      drift_detectors: $drift_detectors,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_store_idempotency_guard_gap_closure_application_preview_gate",
      ready_for_store_idempotency_guard_gap_closure_application_preview: true,
      ready_for_runtime_guard_application: false,
      ready_for_append_only_store_enablement: false,
      ready_for_projection_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        store_idempotency_guard_gap_closure_readback: {
          rust_module_present: $readback_rust_module_present,
          report_script_present: $readback_report_script_present,
          gate_script_present: $readback_gate_script_present
        },
        store_idempotency_guard_gap_closure: {
          upstream_gate: ($closure.gate == "hepta_work_graph_store_idempotency_guard_gap_closure_preview_gate"),
          gate_script_present: $closure_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        wal_written: false,
        idempotency_index_mutated: false,
        store_guard_attached: false,
        append_only_store_enabled: false,
        projection_enforcement_enabled: false,
        readback_performed: false,
        task_result_enforcement_enabled: false,
        scheduler_admission_enforced: false,
        role_manifest_enforcement_enabled: false,
        approval_recorded: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
