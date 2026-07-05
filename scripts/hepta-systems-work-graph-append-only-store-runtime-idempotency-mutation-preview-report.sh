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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-runtime-idempotency-mutation-preview.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$tmpdir/cache"
fi

capture_json_report \
  "hepta-work-graph-unified-projection-enforcement-readiness-runtime-durable-store-switch-rerun-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-durable-store-switch-rerun-preview-report.sh" \
  >"$tmpdir/runtime_durable_store_switch_rerun.json"

idempotency_mutation_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_store_runtime_idempotency_mutation_preview.rs
)"
idempotency_mutation_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-runtime-idempotency-mutation-preview-report.sh
)"
idempotency_mutation_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-runtime-idempotency-mutation-preview-gate.sh
)"
runtime_durable_store_switch_rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-durable-store-switch-rerun-preview-gate.sh
)"

jq -n \
  --slurpfile previous "$tmpdir/runtime_durable_store_switch_rerun.json" \
  --argjson idempotency_mutation_rust_module_present "$idempotency_mutation_rust_module_present" \
  --argjson idempotency_mutation_report_script_present "$idempotency_mutation_report_script_present" \
  --argjson idempotency_mutation_gate_script_present "$idempotency_mutation_gate_script_present" \
  --argjson runtime_durable_store_switch_rerun_gate_script_present "$runtime_durable_store_switch_rerun_gate_script_present" \
  '
  $previous[0] as $previous
  | def stage_ids: [
      "idempotency_mutation_policy_contract",
      "idempotency_collision_replay_evidence_contract",
      "idempotency_index_no_mutation_guard",
      "rollback_readback_prerequisite_contract",
      "idempotency_blocker_mapping"
    ];
  def evidence_fields: [
      "source_surface_id",
      "source_category",
      "durable_store_switch_rerun_decision_ref",
      "idempotency_mutation_policy_id",
      "collision_replay_evidence_id",
      "no_mutation_guard_ref",
      "rollback_readback_prerequisite_id",
      "residual_source_blocker_ids",
      "next_required_gate"
    ];
  def unique_order: reduce .[] as $item ([]; if index($item) then . else . + [$item] end);
  def source_ids_for_blocker($id):
      (($previous.residual_blockers[] | select(.id == $id) | .affected_source_surface_ids) // []);
  def union_sources($ids):
      reduce $ids[] as $id ([]; . + source_ids_for_blocker($id) | unique_order);
  def plan_id($source):
      "append_only_store_runtime_idempotency_mutation_" + $source + "_preview";
  def plan($decision): {
      source_surface_id: $decision.source_surface_id,
      source_category: $decision.source_category,
      idempotency_mutation_plan_id: plan_id($decision.source_surface_id),
      previous_enforcement_decision: $decision.runtime_durable_store_switch_rerun_enforcement_decision,
      idempotency_mutation_state: "idempotency_mutation_contract_defined_preview_only",
      required_idempotency_mutation_stage_ids: stage_ids,
      residual_source_blocker_ids: $decision.residual_source_blocker_ids,
      expected_evidence_field_ids: evidence_fields,
      idempotency_mutation_policy_contract_ready_preview: true,
      collision_replay_evidence_contract_ready_preview: true,
      applies_to_runtime: false,
      writes_wal: false,
      writes_checkpoint: false,
      mutates_idempotency_index: false,
      executes_replay: false,
      executes_readback: false,
      executes_rollback: false,
      mutates_runtime: false
    };
  def stage($id; $priority; $category; $sources; $contracts; $effect): {
      id: $id,
      priority: $priority,
      category: $category,
      affected_source_surface_ids: $sources,
      required_contract_ref_ids: $contracts,
      expected_runtime_state: "contract_ready_preview_runtime_disabled",
      prerequisite_gate_ids: ($previous.required_prior_gates + [$previous.gate]),
      contract_ready_preview: true,
      runtime_enabled_after_preview: false,
      writes_wal: ($effect == "collision_replay"),
      writes_checkpoint: (($effect == "collision_replay") or ($effect == "rollback_readback")),
      mutates_idempotency_index: ($effect == "index_mutation"),
      executes_replay: ($effect == "collision_replay"),
      executes_readback: ($effect == "rollback_readback"),
      executes_rollback: ($effect == "rollback_readback")
    };
  def guard($id; $severity; $scope): {
      id: $id,
      severity: $severity,
      guard_scope: $scope,
      required_before_idempotency_mutation: true,
      satisfied_by_preview: false
    };
  def category_for($id):
      if $id == "idempotency_index_mutation_disabled" then "idempotency_policy"
      elif $id == "wal_write_boundary_not_enabled" then "wal_replay_prerequisite"
      elif ($id == "readback_execution_disabled" or $id == "rollback_readback_not_executed") then "rollback_readback"
      else "idempotency_policy"
      end;
  def stages_for($id):
      if $id == "idempotency_index_mutation_disabled" then ["idempotency_mutation_policy_contract"]
      elif $id == "wal_write_boundary_not_enabled" then ["idempotency_collision_replay_evidence_contract"]
      elif ($id == "readback_execution_disabled" or $id == "rollback_readback_not_executed") then ["rollback_readback_prerequisite_contract"]
      else stage_ids
      end;
  def blocker($id; $severity; $category; $sources; $stages; $fix): {
      id: $id,
      severity: $severity,
      category: $category,
      affected_source_surface_ids: $sources,
      affected_idempotency_mutation_stage_ids: $stages,
      affected_idempotency_mutation_plan_ids: ($sources | map(plan_id(.))),
      required_before_idempotency_mutation: true,
      recommended_fix: $fix
    };
  ($previous.decision_deltas
    | map(select(.runtime_durable_store_switch_rerun_enforcement_decision == "deny_runtime_idempotency_mutation_disabled") | plan(.))) as $plans
  | ($plans | map(.source_surface_id)) as $source_ids
  | [
      stage("idempotency_mutation_policy_contract"; "p0"; "idempotency_policy"; $source_ids; [
        "idempotency_index_key_contract_ready",
        "idempotency_collision_policy_contract_ready",
        "idempotency_replay_dedupe_contract_ready",
        "idempotency_mutation_order_contract_ready",
        "idempotency_redaction_contract_ready",
        "idempotency_operator_gate_contract_ready"
      ]; "index_mutation"),
      stage("idempotency_collision_replay_evidence_contract"; "p0"; "collision_replay_evidence"; $source_ids; [
        "collision_receipt_contract_ready",
        "replay_cursor_observation_contract_ready",
        "wal_order_reference_contract_ready",
        "checkpoint_reference_contract_ready",
        "duplicate_suppression_readback_contract_ready",
        "idempotency_evidence_hash_contract_ready"
      ]; "collision_replay"),
      stage("idempotency_index_no_mutation_guard"; "p0"; "preview_no_mutation"; $source_ids; [
        "filesystem_no_write_guard_ready",
        "graph_state_no_persist_guard_ready",
        "idempotency_index_no_mutation_guard_ready",
        "wal_no_write_guard_ready",
        "checkpoint_no_write_guard_ready",
        "runtime_no_mutation_guard_ready"
      ]; "none"),
      stage("rollback_readback_prerequisite_contract"; "p0"; "rollback_readback_prerequisite"; $source_ids; [
        "readback_execution_contract_ready",
        "rollback_execution_contract_ready",
        "mutation_receipt_readback_contract_ready",
        "operator_review_evidence_contract_ready",
        "side_effect_lock_contract_ready"
      ]; "rollback_readback"),
      stage("idempotency_blocker_mapping"; "p0"; "blocker_mapping"; $source_ids; [
        "idempotency_blocker_mapping_ready",
        "wal_boundary_blocker_mapping_ready",
        "rollback_readback_blocker_mapping_ready",
        "projection_enforcement_blocker_mapping_ready",
        "append_only_store_enablement_blocker_mapping_ready"
      ]; "none")
    ] as $stages
  | [
      guard("idempotency_mutation_preview_only"; "medium"; "preview_boundary"),
      guard("idempotency_index_mutation_disabled"; "critical"; "idempotency_index"),
      guard("wal_write_boundary_disabled"; "critical"; "wal_boundary"),
      guard("checkpoint_write_disabled"; "critical"; "checkpoint"),
      guard("replay_execution_disabled"; "critical"; "replay"),
      guard("rollback_readback_execution_disabled"; "critical"; "rollback_readback"),
      guard("durable_store_switch_disabled"; "critical"; "durable_store_switch"),
      guard("runtime_mutation_disabled"; "critical"; "runtime_mutation")
    ] as $guards
  | (($previous.residual_blockers | map(blocker(
        .id;
        .severity;
        category_for(.id);
        .affected_source_surface_ids;
        stages_for(.id);
        .recommended_fix
      ))) + [
        blocker(
          "idempotency_mutation_readback_missing";
          "high";
          "readback_preview";
          $source_ids;
          stage_ids;
          "read back idempotency mutation plans before any idempotency index mutation, WAL replay, rollback/readback, or projection enforcement promotion"
        )
      ]) as $blockers
  | ($previous.required_prior_gates + [$previous.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_append_only_store_runtime_idempotency_mutation_preview_gate",
      schema_version: "work_graph_append_only_store_runtime_idempotency_mutation_preview_v1",
      preview_mode: "read_only_append_only_store_runtime_idempotency_mutation_preview_no_index_mutation",
      upstream_runtime_durable_store_switch_rerun_gate: "hepta_work_graph_unified_projection_enforcement_readiness_runtime_durable_store_switch_rerun_preview_gate",
      source_surface_count: $previous.source_surface_count,
      idempotency_mutation_source_count: ($source_ids | length),
      idempotency_mutation_plan_count: ($plans | length),
      idempotency_mutation_stage_count: ($stages | length),
      idempotency_mutation_stage_source_ref_count: ($stages | map(.affected_source_surface_ids | length) | add),
      idempotency_mutation_stage_contract_ref_count: ($stages | map(.required_contract_ref_ids | length) | add),
      idempotency_mutation_plan_stage_ref_count: ($plans | map(.required_idempotency_mutation_stage_ids | length) | add),
      idempotency_mutation_plan_evidence_field_ref_count: ($plans | map(.expected_evidence_field_ids | length) | add),
      idempotency_residual_source_count: (source_ids_for_blocker("idempotency_index_mutation_disabled") | length),
      wal_boundary_residual_source_count: (source_ids_for_blocker("wal_write_boundary_not_enabled") | length),
      rollback_readback_residual_source_count: (union_sources(["readback_execution_disabled","rollback_readback_not_executed"]) | length),
      guard_count: ($guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      idempotency_mutation_plans: $plans,
      idempotency_mutation_stage_plans: $stages,
      guards: $guards,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_append_only_store_runtime_idempotency_mutation_readback_preview_gate",
      ready_for_runtime_idempotency_mutation_readback_preview: true,
      ready_for_runtime_idempotency_mutation_application_preview: false,
      ready_for_wal_write: false,
      ready_for_checkpoint_write: false,
      ready_for_idempotency_mutation: false,
      ready_for_readback_execution: false,
      ready_for_rollback_execution: false,
      ready_for_append_only_store_enablement: false,
      ready_for_projection_enforcement: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        idempotency_mutation_preview: {
          rust_module_present: $idempotency_mutation_rust_module_present,
          report_script_present: $idempotency_mutation_report_script_present,
          gate_script_present: $idempotency_mutation_gate_script_present
        },
        runtime_durable_store_switch_rerun: {
          upstream_gate: ($previous.gate == "hepta_work_graph_unified_projection_enforcement_readiness_runtime_durable_store_switch_rerun_preview_gate"),
          gate_script_present: $runtime_durable_store_switch_rerun_gate_script_present,
          recommended_next_matches: ($previous.recommended_next_gate == "hepta_work_graph_append_only_store_runtime_idempotency_mutation_preview_gate")
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
        lane_lease_acquired: false,
        work_started: false,
        budget_consumed: false,
        approval_recorded: false,
        operator_review_recorded: false,
        side_effect_lock_established: false,
        task_result_enforcement_enabled: false,
        task_result_persisted: false,
        role_manifest_enforcement_enabled: false,
        tool_permission_changed: false,
        role_budget_consumed: false,
        role_lane_binding_mutated: false,
        readback_executed: false,
        replay_executed: false,
        rollback_executed: false,
        runtime_application_promoted: false,
        runtime_wrapper_attached: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }
  '
