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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-store-guard-gap-closure.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-unified-projection-enforcement-readiness-rerun-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-rerun-preview-report.sh" \
  >"$tmpdir/readiness_rerun.json"
capture_json_report \
  "hepta-work-graph-idempotency-readback-adapter-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-idempotency-readback-adapter-preview-report.sh" \
  >"$tmpdir/idempotency_adapter.json"
capture_json_report \
  "hepta-work-graph-state-store-persistence-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-state-store-persistence-preview-report.sh" \
  >"$tmpdir/state_store_persistence.json"

closure_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_store_idempotency_guard_gap_closure_preview.rs
)"
closure_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-store-idempotency-guard-gap-closure-preview-report.sh
)"
closure_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-store-idempotency-guard-gap-closure-preview-gate.sh
)"
rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-rerun-preview-gate.sh
)"
idempotency_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-idempotency-readback-adapter-preview-gate.sh
)"
state_store_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-state-store-persistence-preview-gate.sh
)"

jq -n \
  --slurpfile rerun "$tmpdir/readiness_rerun.json" \
  --slurpfile idempotency "$tmpdir/idempotency_adapter.json" \
  --slurpfile state "$tmpdir/state_store_persistence.json" \
  --argjson closure_rust_module_present "$closure_rust_module_present" \
  --argjson closure_report_script_present "$closure_report_script_present" \
  --argjson closure_gate_script_present "$closure_gate_script_present" \
  --argjson rerun_gate_script_present "$rerun_gate_script_present" \
  --argjson idempotency_gate_script_present "$idempotency_gate_script_present" \
  --argjson state_store_gate_script_present "$state_store_gate_script_present" \
  '
  $rerun[0] as $rerun
  | $idempotency[0] as $idempotency
  | $state[0] as $state
  | def unique_ordered($values):
      reduce $values[] as $value ([]; if index($value) then . else . + [$value] end);
  def adapter_for($source):
      [$idempotency.source_adapters[] | select(.source_surface_id == $source)][0];
  def replay_for($id):
      [$idempotency.replay_key_contracts[] | select(.id == $id)][0];
  def probes_for($source):
      [$idempotency.readback_probe_contracts[] | select(.source_surface_id == $source)];
  def closure_plan_id($source):
      "close_" + $source + "_store_idempotency_guard_gap";
  def candidate_guard_id($source):
      $source + "_store_idempotency_guard";
  def guard_binding_id($source):
      $source + "_store_guard_binding";
  def guard_probe_binding_id($source):
      $source + "_store_guard_readback_binding";
  def closure_plan($decision):
      (adapter_for($decision.source_surface_id)) as $adapter
      | (replay_for($adapter.replay_key_contract_id)) as $replay
      | {
          id: closure_plan_id($decision.source_surface_id),
          source_surface_id: $decision.source_surface_id,
          source_category: $decision.source_category,
          rerun_enforcement_decision: $decision.rerun_enforcement_decision,
          adapter_id: $adapter.id,
          replay_key_contract_id: $replay.id,
          candidate_guard_id: candidate_guard_id($decision.source_surface_id),
          key_fields: $replay.key_fields,
          expected_collection_ids: $adapter.expected_collection_ids,
          readback_probe_contract_ids: $adapter.readback_probe_contract_ids,
          collision_policy: $replay.collision_policy,
          closure_state: "candidate_guard_preview_only_runtime_not_attached",
          requires_task_result_wrapper: $adapter.requires_task_result_wrapper,
          runtime_guard_attached: false,
          mutates_idempotency_index: false,
          enables_store_write: false
        };
  def candidate_guard($plan):
      (adapter_for($plan.source_surface_id)) as $adapter
      | (replay_for($adapter.replay_key_contract_id)) as $replay
      | {
          id: $plan.candidate_guard_id,
          source_surface_id: $plan.source_surface_id,
          derived_from_adapter_id: $adapter.id,
          derived_from_replay_key_contract_id: $replay.id,
          key_fields: $replay.key_fields,
          key_formula: $replay.key_formula,
          replay_scope: $replay.replay_scope,
          collision_policy: $replay.collision_policy,
          redaction_policy: $replay.redaction_policy,
          required_before_append_only_intake: true,
          mutates_idempotency_index: false
        };
  def guard_binding($plan):
      (adapter_for($plan.source_surface_id)) as $adapter
      | (replay_for($adapter.replay_key_contract_id)) as $replay
      | (probes_for($plan.source_surface_id)) as $probes
      | {
          id: guard_binding_id($plan.source_surface_id),
          source_surface_id: $plan.source_surface_id,
          candidate_guard_id: $plan.candidate_guard_id,
          adapter_id: $adapter.id,
          replay_key_contract_id: $replay.id,
          existing_state_store_guard_present: any($state.idempotency_guards[]; .source_surface_id == $plan.source_surface_id),
          adapter_replay_key_contract_present: ($replay.id == $adapter.replay_key_contract_id),
          readback_probe_count: ($probes | length),
          expected_collection_ids: $adapter.expected_collection_ids,
          requires_task_result_wrapper: $adapter.requires_task_result_wrapper,
          closure_state: "candidate_guard_defined_state_store_binding_not_applied",
          no_runtime_application: true
        };
  def probe_binding($plan):
      (probes_for($plan.source_surface_id)) as $probes
      | {
          id: guard_probe_binding_id($plan.source_surface_id),
          source_surface_id: $plan.source_surface_id,
          candidate_guard_id: $plan.candidate_guard_id,
          readback_probe_contract_ids: ($probes | map(.id)),
          target_collection_ids: unique_ordered($probes | map(.collection_id)),
          readback_evidence_fields: unique_ordered($probes | map(.evidence_fields[]) ),
          drift_detector_ids: unique_ordered($probes | map(.drift_detector_ids[]) ),
          performs_readback: false,
          mutates_store: false
        };
  def blocker($id; $severity; $sources; $fix): {
      id: $id,
      severity: $severity,
      affected_source_surface_ids: $sources,
      required_before_projection_enforcement: true,
      recommended_fix: $fix
    };
  ($rerun.decision_deltas
    | map(select(.rerun_enforcement_decision == "deny_missing_store_idempotency_guard"))) as $gap_decisions
  | ($gap_decisions | map(closure_plan(.))) as $closure_plans
  | ($closure_plans | map(candidate_guard(.))) as $candidate_guards
  | ($closure_plans | map(guard_binding(.))) as $guard_bindings
  | ($closure_plans | map(probe_binding(.))) as $guard_probe_bindings
  | ($closure_plans | map(.source_surface_id)) as $source_ids
  | [
      blocker("runtime_guard_application_disabled"; "high"; $source_ids; "bind candidate guards to runtime adapters only after readback and operator review promote this preview"),
      blocker("state_store_guard_persistence_disabled"; "high"; $source_ids; "keep state-store guard rows as preview contracts until append-only intake and WAL replay gates are promoted"),
      blocker("append_only_store_enablement_disabled"; "high"; $source_ids; "do not allow append-only writes until guard collisions, replay, and readback are deterministic"),
      blocker("task_result_enforcement_disabled"; "high"; ["hepta_runtime_multi_agent_reducer", "hepta_runtime_task_board"]; "TaskResult-producing guard closures need terminal TaskResult enforcement before runtime use"),
      blocker("readback_execution_disabled"; "medium"; $source_ids; "verify guard readback contracts in a dedicated readback preview before any runtime application"),
      blocker("operator_review_required"; "medium"; $source_ids; "require operator review of guard formulas, collision policy, and redaction before promotion")
    ] as $blockers
  | ($rerun.required_prior_gates + [$rerun.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_store_idempotency_guard_gap_closure_preview_gate",
      schema_version: "work_graph_store_idempotency_guard_gap_closure_preview_v1",
      preview_mode: "read_only_store_idempotency_guard_gap_closure_no_index_write",
      rerun_store_guard_gap_count: ($gap_decisions | length),
      idempotency_adapter_count: $idempotency.source_adapter_count,
      existing_state_store_guard_count: $state.idempotency_guard_count,
      existing_guard_gap_count: ($guard_bindings | map(select(.existing_state_store_guard_present == false)) | length),
      closure_plan_count: ($closure_plans | length),
      candidate_guard_count: ($candidate_guards | length),
      guard_binding_count: ($guard_bindings | length),
      guard_probe_binding_count: ($guard_probe_bindings | length),
      expected_collection_ref_count: ($closure_plans | map(.expected_collection_ids | length) | add),
      readback_probe_contract_ref_count: ($closure_plans | map(.readback_probe_contract_ids | length) | add),
      task_result_guard_dependency_count: ($closure_plans | map(select(.requires_task_result_wrapper == true)) | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      closure_plans: $closure_plans,
      candidate_guards: $candidate_guards,
      guard_bindings: $guard_bindings,
      guard_probe_bindings: $guard_probe_bindings,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_store_idempotency_guard_gap_closure_readback_preview_gate",
      ready_for_store_idempotency_guard_gap_closure_readback_preview: true,
      ready_for_runtime_guard_application: false,
      ready_for_append_only_store_enablement: false,
      ready_for_projection_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        store_idempotency_guard_gap_closure: {
          rust_module_present: $closure_rust_module_present,
          report_script_present: $closure_report_script_present,
          gate_script_present: $closure_gate_script_present
        },
        readiness_rerun: {
          upstream_gate: ($rerun.gate == "hepta_work_graph_unified_projection_enforcement_readiness_rerun_preview_gate"),
          gate_script_present: $rerun_gate_script_present
        },
        idempotency_readback_adapter: {
          upstream_gate: ($idempotency.gate == "hepta_work_graph_idempotency_readback_adapter_preview_gate"),
          gate_script_present: $idempotency_gate_script_present
        },
        state_store_persistence: {
          upstream_gate: ($state.gate == "hepta_work_graph_state_store_persistence_preview_gate"),
          gate_script_present: $state_store_gate_script_present
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
