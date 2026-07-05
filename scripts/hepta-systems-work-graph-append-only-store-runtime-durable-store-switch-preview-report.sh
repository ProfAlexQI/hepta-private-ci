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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-runtime-durable-store-switch-preview.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$tmpdir/cache"
fi

capture_json_report \
  "hepta-work-graph-unified-projection-enforcement-readiness-runtime-write-boundary-rerun-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-write-boundary-rerun-preview-report.sh" \
  >"$tmpdir/runtime_write_boundary_rerun.json"

durable_store_switch_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_store_runtime_durable_store_switch_preview.rs
)"
durable_store_switch_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-runtime-durable-store-switch-preview-report.sh
)"
durable_store_switch_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-runtime-durable-store-switch-preview-gate.sh
)"
runtime_write_boundary_rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-write-boundary-rerun-preview-gate.sh
)"

jq -n \
  --slurpfile previous "$tmpdir/runtime_write_boundary_rerun.json" \
  --argjson durable_store_switch_rust_module_present "$durable_store_switch_rust_module_present" \
  --argjson durable_store_switch_report_script_present "$durable_store_switch_report_script_present" \
  --argjson durable_store_switch_gate_script_present "$durable_store_switch_gate_script_present" \
  --argjson runtime_write_boundary_rerun_gate_script_present "$runtime_write_boundary_rerun_gate_script_present" \
  '
  $previous[0] as $previous
  | def stage_ids: [
      "runtime_durable_store_switch_contract",
      "wal_replay_prerequisite_contract",
      "operator_review_rollback_guard",
      "durable_store_switch_no_mutation_guard",
      "durable_store_switch_blocker_mapping"
    ];
  def evidence_fields: [
      "source_surface_id",
      "source_category",
      "runtime_write_boundary_rerun_decision_ref",
      "durable_store_switch_contract_id",
      "wal_replay_prerequisite_id",
      "operator_review_rollback_guard_id",
      "no_mutation_guard_ref",
      "residual_source_blocker_ids",
      "next_required_gate"
    ];
  def unique_order: reduce .[] as $item ([]; if index($item) then . else . + [$item] end);
  def source_ids_for_blocker($id):
      (($previous.residual_blockers[] | select(.id == $id) | .affected_source_surface_ids) // []);
  def union_sources($ids):
      reduce $ids[] as $id ([]; . + source_ids_for_blocker($id) | unique_order);
  def plan_id($source):
      "append_only_store_runtime_durable_store_switch_" + $source + "_preview";
  def plan($decision): {
      source_surface_id: $decision.source_surface_id,
      source_category: $decision.source_category,
      durable_store_switch_plan_id: plan_id($decision.source_surface_id),
      previous_enforcement_decision: $decision.runtime_write_boundary_rerun_enforcement_decision,
      durable_store_switch_state: "durable_store_switch_contract_defined_preview_only",
      required_durable_store_switch_stage_ids: stage_ids,
      residual_source_blocker_ids: $decision.residual_source_blocker_ids,
      expected_evidence_field_ids: evidence_fields,
      durable_store_switch_contract_ready_preview: true,
      applies_to_runtime: false,
      writes_wal: false,
      writes_checkpoint: false,
      switches_durable_store: false,
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
      writes_wal: ($effect == "wal_replay"),
      writes_checkpoint: (($effect == "wal_replay") or ($effect == "rollback_readback")),
      switches_durable_store: ($effect == "switch"),
      mutates_idempotency_index: false,
      executes_replay: ($effect == "wal_replay"),
      executes_readback: ($effect == "rollback_readback"),
      executes_rollback: ($effect == "rollback_readback")
    };
  def guard($id; $severity; $scope): {
      id: $id,
      severity: $severity,
      guard_scope: $scope,
      required_before_durable_store_switch: true,
      satisfied_by_preview: false
    };
  def category_for($id):
      if $id == "durable_store_runtime_switch_disabled" then "durable_store_switch"
      elif $id == "wal_write_boundary_not_enabled" then "wal_replay_prerequisite"
      elif $id == "idempotency_index_mutation_disabled" then "idempotency_policy"
      elif ($id == "readback_execution_disabled" or $id == "rollback_readback_not_executed") then "rollback_readback"
      else "durable_store_switch"
      end;
  def stages_for($id):
      if $id == "durable_store_runtime_switch_disabled" then ["runtime_durable_store_switch_contract"]
      elif $id == "wal_write_boundary_not_enabled" then ["wal_replay_prerequisite_contract"]
      elif $id == "idempotency_index_mutation_disabled" then ["durable_store_switch_blocker_mapping"]
      elif ($id == "readback_execution_disabled" or $id == "rollback_readback_not_executed") then ["operator_review_rollback_guard"]
      else stage_ids
      end;
  def blocker($id; $severity; $category; $sources; $stages; $fix): {
      id: $id,
      severity: $severity,
      category: $category,
      affected_source_surface_ids: $sources,
      affected_durable_store_switch_stage_ids: $stages,
      affected_durable_store_switch_plan_ids: ($sources | map(plan_id(.))),
      required_before_durable_store_switch: true,
      recommended_fix: $fix
    };
  ($previous.decision_deltas
    | map(select(.runtime_write_boundary_rerun_enforcement_decision == "deny_runtime_durable_store_switch_disabled") | plan(.))) as $plans
  | ($plans | map(.source_surface_id)) as $source_ids
  | [
      stage("runtime_durable_store_switch_contract"; "p0"; "durable_store_switch"; $source_ids; [
        "durable_store_switch_contract_ready",
        "runtime_store_selector_contract_ready",
        "append_only_store_handle_contract_ready",
        "fallback_store_disable_contract_ready",
        "switch_state_readback_contract_ready",
        "switch_operator_gate_contract_ready"
      ]; "switch"),
      stage("wal_replay_prerequisite_contract"; "p0"; "wal_replay_prerequisite"; $source_ids; [
        "wal_ordering_contract_ready",
        "wal_replay_cursor_contract_ready",
        "checkpoint_replay_contract_ready",
        "durable_store_no_rewrite_contract_ready",
        "event_log_feature_gate_contract_ready",
        "replay_receipt_contract_ready"
      ]; "wal_replay"),
      stage("operator_review_rollback_guard"; "p0"; "operator_review_rollback"; $source_ids; [
        "operator_review_evidence_contract_ready",
        "side_effect_lock_contract_ready",
        "rollback_gate_contract_ready",
        "readback_execution_contract_ready",
        "switch_denial_receipt_contract_ready"
      ]; "rollback_readback"),
      stage("durable_store_switch_no_mutation_guard"; "p0"; "preview_no_mutation"; $source_ids; [
        "filesystem_no_write_guard_ready",
        "graph_state_no_persist_guard_ready",
        "durable_store_no_switch_guard_ready",
        "wal_no_write_guard_ready",
        "idempotency_no_mutation_guard_ready",
        "runtime_no_mutation_guard_ready"
      ]; "none"),
      stage("durable_store_switch_blocker_mapping"; "p0"; "blocker_mapping"; $source_ids; [
        "durable_store_blocker_mapping_ready",
        "wal_boundary_blocker_mapping_ready",
        "idempotency_blocker_mapping_ready",
        "rollback_readback_blocker_mapping_ready",
        "projection_enforcement_blocker_mapping_ready"
      ]; "none")
    ] as $stages
  | [
      guard("durable_store_switch_preview_only"; "medium"; "preview_boundary"),
      guard("durable_store_runtime_switch_disabled"; "critical"; "durable_store_switch"),
      guard("wal_write_boundary_disabled"; "critical"; "wal_boundary"),
      guard("checkpoint_write_disabled"; "critical"; "checkpoint"),
      guard("replay_execution_disabled"; "critical"; "replay"),
      guard("rollback_readback_execution_disabled"; "critical"; "rollback_readback"),
      guard("idempotency_index_mutation_disabled"; "critical"; "idempotency_index"),
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
          "durable_store_switch_readback_missing";
          "high";
          "readback_preview";
          $source_ids;
          stage_ids;
          "read back durable-store switch plans before any runtime store selector, WAL, replay, checkpoint, idempotency, or rollback promotion"
        )
      ]) as $blockers
  | ($previous.required_prior_gates + [$previous.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_append_only_store_runtime_durable_store_switch_preview_gate",
      schema_version: "work_graph_append_only_store_runtime_durable_store_switch_preview_v1",
      preview_mode: "read_only_append_only_store_runtime_durable_store_switch_preview_no_store_mutation",
      upstream_runtime_write_boundary_rerun_gate: "hepta_work_graph_unified_projection_enforcement_readiness_runtime_write_boundary_rerun_preview_gate",
      source_surface_count: $previous.source_surface_count,
      durable_store_switch_source_count: ($source_ids | length),
      durable_store_switch_plan_count: ($plans | length),
      durable_store_switch_stage_count: ($stages | length),
      durable_store_switch_stage_source_ref_count: ($stages | map(.affected_source_surface_ids | length) | add),
      durable_store_switch_stage_contract_ref_count: ($stages | map(.required_contract_ref_ids | length) | add),
      durable_store_switch_plan_stage_ref_count: ($plans | map(.required_durable_store_switch_stage_ids | length) | add),
      durable_store_switch_plan_evidence_field_ref_count: ($plans | map(.expected_evidence_field_ids | length) | add),
      durable_store_residual_source_count: (source_ids_for_blocker("durable_store_runtime_switch_disabled") | length),
      wal_boundary_residual_source_count: (source_ids_for_blocker("wal_write_boundary_not_enabled") | length),
      idempotency_residual_source_count: (source_ids_for_blocker("idempotency_index_mutation_disabled") | length),
      rollback_readback_residual_source_count: (union_sources(["readback_execution_disabled","rollback_readback_not_executed"]) | length),
      guard_count: ($guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      durable_store_switch_plans: $plans,
      durable_store_switch_stage_plans: $stages,
      guards: $guards,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_append_only_store_runtime_durable_store_switch_readback_preview_gate",
      ready_for_runtime_durable_store_switch_readback_preview: true,
      ready_for_runtime_durable_store_switch_application_preview: false,
      ready_for_wal_write: false,
      ready_for_checkpoint_write: false,
      ready_for_durable_store_switch: false,
      ready_for_idempotency_mutation: false,
      ready_for_rollback_readback_execution: false,
      ready_for_append_only_store_enablement: false,
      ready_for_projection_enforcement: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        durable_store_switch_preview: {
          rust_module_present: $durable_store_switch_rust_module_present,
          report_script_present: $durable_store_switch_report_script_present,
          gate_script_present: $durable_store_switch_gate_script_present
        },
        runtime_write_boundary_rerun: {
          upstream_gate: ($previous.gate == "hepta_work_graph_unified_projection_enforcement_readiness_runtime_write_boundary_rerun_preview_gate"),
          gate_script_present: $runtime_write_boundary_rerun_gate_script_present,
          recommended_next_matches: ($previous.recommended_next_gate == "hepta_work_graph_append_only_store_runtime_durable_store_switch_preview_gate")
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
