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

durable_identity_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_durable_identity_preview.rs
)"
durable_identity_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-durable-identity-preview-report.sh
)"
durable_identity_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-durable-identity-preview-gate.sh
)"
state_store_persistence_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_state_store_persistence_preview.rs
)"
state_store_persistence_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-state-store-persistence-preview-report.sh
)"
state_store_persistence_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-state-store-persistence-preview-gate.sh
)"
replay_readback_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_replay_readback_preview.rs
)"
replay_readback_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-replay-readback-preview-report.sh
)"
replay_readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-replay-readback-preview-gate.sh
)"
promotion_precondition_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_promotion_precondition_preview.rs
)"

state_report="$(
  capture_json_report \
    "hepta-work-graph-state-store-persistence-preview-report" \
    "$ROOT/scripts/hepta-systems-work-graph-state-store-persistence-preview-report.sh"
)"
replay_report="$(
  capture_json_report \
    "hepta-work-graph-replay-readback-preview-report" \
    "$ROOT/scripts/hepta-systems-work-graph-replay-readback-preview-report.sh"
)"

jq -n \
  --argjson durable_identity_rust_module_present "$durable_identity_rust_module_present" \
  --argjson durable_identity_report_script_present "$durable_identity_report_script_present" \
  --argjson durable_identity_gate_script_present "$durable_identity_gate_script_present" \
  --argjson state_store_persistence_rust_module_present "$state_store_persistence_rust_module_present" \
  --argjson state_store_persistence_report_script_present "$state_store_persistence_report_script_present" \
  --argjson state_store_persistence_gate_script_present "$state_store_persistence_gate_script_present" \
  --argjson replay_readback_rust_module_present "$replay_readback_rust_module_present" \
  --argjson replay_readback_report_script_present "$replay_readback_report_script_present" \
  --argjson replay_readback_gate_script_present "$replay_readback_gate_script_present" \
  --argjson promotion_precondition_rust_module_present "$promotion_precondition_rust_module_present" \
  --argjson state_report "$state_report" \
  --argjson replay_report "$replay_report" \
  '
  def prior_gates: [
    "hepta_work_graph_contract_preview_gate",
    "hepta_work_graph_task_result_contract_preview_gate",
    "hepta_work_graph_scheduler_admission_controller_preview_gate",
    "hepta_work_graph_observability_timeline_preview_gate",
    "hepta_work_graph_role_manifest_contract_preview_gate",
    "hepta_work_graph_unified_state_store_preview_gate",
    "hepta_work_graph_adapter_projection_fixture_gate",
    "hepta_work_graph_state_store_persistence_preview_gate",
    "hepta_work_graph_replay_readback_preview_gate"
  ];
  def durable_field($id; $phase; $sources; $gate; $policy): {
    id: $id,
    phase: $phase,
    source_fields: $sources,
    required_prior_gate: $gate,
    persistence_policy: $policy,
    mutates_state: false
  };
  def preview_binding($id; $gate; $contracts; $fields): {
    id: $id,
    source_gate: $gate,
    source_contract_ids: $contracts,
    binds_fields: $fields,
    required: true,
    mutates_state: false
  };
  def invariant($id; $reason): {
    id: $id,
    required: true,
    reason: $reason
  };
  [
    durable_field(
      "workflow_id";
      "identity";
      ["traceId", "sourceThreadId", "sourceSurfaceId"];
      "hepta_work_graph_contract_preview_gate";
      "required_on_every_wal_record_before_store_persistence"
    ),
    durable_field(
      "run_id";
      "identity";
      ["schedulerRunId", "workerTaskId", "jobId", "attempt"];
      "hepta_work_graph_scheduler_admission_controller_preview_gate";
      "required_before_scheduler_or_worker_projection"
    ),
    durable_field(
      "step_id";
      "identity";
      ["stepIndex", "taskId", "nodeId", "edgeId"];
      "hepta_work_graph_task_result_contract_preview_gate";
      "required_before_task_result_or_edge_projection"
    ),
    durable_field(
      "checkpoint";
      "checkpoint";
      ["walHeadHash", "checkpointHash", "collectionMerkleRoot"];
      "hepta_work_graph_state_store_persistence_preview_gate";
      "derived_from_wal_and_disabled_until_replay_readback_passes"
    ),
    durable_field(
      "replay_key";
      "replay";
      ["validatedWalHeadHash", "sourceRecordKeys", "dedupedRecordSet"];
      "hepta_work_graph_replay_readback_preview_gate";
      "deterministic_key_required_before_any_replay_execution"
    ),
    durable_field(
      "rollback_anchor";
      "rollback";
      ["checkpointHash", "detectorIds", "operatorApprovalRef"];
      "hepta_work_graph_replay_readback_preview_gate";
      "required_before_recovery_canary_or_runtime_rollback"
    ),
    durable_field(
      "receipt_hash";
      "receipt";
      ["taskResultHash", "approvalHash", "timelineHash", "redactedReadbackEvidenceRefs"];
      "hepta_work_graph_replay_readback_preview_gate";
      "required_before_promotion_or_operator_audit_visibility"
    )
  ] as $durable_fields
  | [
    preview_binding(
      "state_store_wal_to_durable_identity";
      "hepta_work_graph_state_store_persistence_preview_gate";
      ["preview_append_node_record", "preview_append_task_result_record", "preview_append_timeline_event_record"];
      ["workflow_id", "run_id", "step_id", "receipt_hash"]
    ),
    preview_binding(
      "checkpoint_contract_to_checkpoint";
      "hepta_work_graph_state_store_persistence_preview_gate";
      ["preview_full_graph_checkpoint", "preview_trace_checkpoint"];
      ["checkpoint", "receipt_hash"]
    ),
    preview_binding(
      "replay_hash_chain_to_replay_key";
      "hepta_work_graph_replay_readback_preview_gate";
      ["preview_validate_wal_hash_chain", "preview_apply_idempotency_window"];
      ["checkpoint", "replay_key"]
    ),
    preview_binding(
      "recovery_preview_to_rollback_anchor";
      "hepta_work_graph_replay_readback_preview_gate";
      ["preview_quarantine_checkpoint", "preview_rebuild_projection_indexes", "preview_require_operator_replay_approval"];
      ["rollback_anchor", "receipt_hash"]
    ),
    preview_binding(
      "readback_evidence_to_receipt_hash";
      "hepta_work_graph_replay_readback_preview_gate";
      ["assert_task_results_readback_matches_wal", "assert_approvals_readback_matches_wal", "assert_timeline_readback_matches_wal"];
      ["workflow_id", "step_id", "receipt_hash"]
    )
  ] as $preview_bindings
  | [
    invariant("durable_identity_required_before_persistence"; "workflow, run, and step identity must be stable before WAL or checkpoint writes"),
    invariant("checkpoint_derived_from_wal"; "checkpoint is an evidence pointer derived from WAL hashes, not an authority source"),
    invariant("replay_key_is_deterministic"; "the same WAL head and source record keys must produce the same replay key"),
    invariant("rollback_anchor_precedes_recovery"; "future recovery or rollback cannot run without a named checkpoint anchor"),
    invariant("receipt_hash_precedes_promotion"; "promotion and operator audit require redacted receipt hash evidence first"),
    invariant("readback_evidence_is_redacted"; "receipt hashes and evidence refs must not expose raw prompts or credentials"),
    invariant("durable_identity_preview_has_no_side_effects"; "this preview cannot persist state, replay WAL, roll back, promote, or send externally")
  ] as $invariants
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_durable_identity_preview_gate",
      schema_version: "work_graph_durable_identity_preview_v1",
      preview_mode: "read_only_durable_identity_contract_preview_no_state_writes",
      durable_field_count: ($durable_fields | length),
      preview_binding_count: ($preview_bindings | length),
      invariant_count: ($invariants | length),
      required_prior_gates: prior_gates,
      durable_fields: $durable_fields,
      preview_bindings: $preview_bindings,
      invariants: $invariants,
      existing_preview_bindings: {
        state_store_schema_version: $state_report.schema_version,
        replay_readback_schema_version: $replay_report.schema_version,
        wal_operation_count: $state_report.wal_operation_count,
        checkpoint_contract_count: $state_report.checkpoint_contract_count,
        idempotency_guard_count: $state_report.idempotency_guard_count,
        readback_probe_count: $state_report.readback_probe_count,
        replay_stage_count: $replay_report.replay_stage_count,
        readback_assertion_count: $replay_report.readback_assertion_count,
        drift_detector_count: $replay_report.drift_detector_count,
        recovery_preview_count: $replay_report.recovery_preview_count
      },
      recommended_next_gate: "hepta_work_graph_promotion_precondition_preview_gate",
      ready_for_promotion_precondition_preview: true,
      ready_for_durable_runtime: false,
      ready_for_replay_execution: false,
      ready_for_rollback_execution: false,
      ready_for_live_execution: false,
      source_probes: {
        durable_identity: {
          rust_module_present: $durable_identity_rust_module_present,
          report_script_present: $durable_identity_report_script_present,
          gate_script_present: $durable_identity_gate_script_present
        },
        state_store_persistence: {
          rust_module_present: $state_store_persistence_rust_module_present,
          report_script_present: $state_store_persistence_report_script_present,
          gate_script_present: $state_store_persistence_gate_script_present
        },
        replay_readback: {
          rust_module_present: $replay_readback_rust_module_present,
          report_script_present: $replay_readback_report_script_present,
          gate_script_present: $replay_readback_gate_script_present
        },
        promotion_precondition: {
          rust_module_present: $promotion_precondition_rust_module_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        wal_written: false,
        checkpoint_written: false,
        replay_executed: false,
        rollback_performed: false,
        receipt_persisted: false,
        idempotency_index_mutated: false,
        promotion_performed: false,
        runtime_mutation_performed: false,
        scheduler_cutover_performed: false,
        approval_recorded: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
