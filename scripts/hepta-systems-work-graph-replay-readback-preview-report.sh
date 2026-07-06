#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

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

replay_readback_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_replay_readback_preview.rs
)"
replay_readback_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-replay-readback-preview-report.sh
)"
replay_readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-replay-readback-preview-gate.sh
)"
append_only_event_intake_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_event_intake_preview.rs
)"
append_only_event_intake_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-event-intake-preview-report.sh
)"
append_only_event_intake_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-event-intake-preview-gate.sh
)"
unified_projection_audit_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_unified_projection_audit_preview.rs
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

jq -n \
  --argjson replay_readback_rust_module_present "$replay_readback_rust_module_present" \
  --argjson replay_readback_report_script_present "$replay_readback_report_script_present" \
  --argjson replay_readback_gate_script_present "$replay_readback_gate_script_present" \
  --argjson append_only_event_intake_rust_module_present "$append_only_event_intake_rust_module_present" \
  --argjson append_only_event_intake_report_script_present "$append_only_event_intake_report_script_present" \
  --argjson append_only_event_intake_gate_script_present "$append_only_event_intake_gate_script_present" \
  --argjson unified_projection_audit_rust_module_present "$unified_projection_audit_rust_module_present" \
  --argjson state_store_persistence_rust_module_present "$state_store_persistence_rust_module_present" \
  --argjson state_store_persistence_report_script_present "$state_store_persistence_report_script_present" \
  --argjson state_store_persistence_gate_script_present "$state_store_persistence_gate_script_present" \
  '
  def prior_gates: [
    "hepta_work_graph_contract_preview_gate",
    "hepta_work_graph_task_result_contract_preview_gate",
    "hepta_work_graph_scheduler_admission_controller_preview_gate",
    "hepta_work_graph_observability_timeline_preview_gate",
    "hepta_work_graph_role_manifest_contract_preview_gate",
    "hepta_work_graph_unified_state_store_preview_gate",
    "hepta_work_graph_adapter_projection_fixture_gate",
    "hepta_work_graph_unified_projection_audit_preview_gate",
    "hepta_work_graph_state_store_persistence_preview_gate",
    "hepta_work_graph_append_only_event_intake_preview_gate"
  ];
  def stage($id; $inputs; $outputs; $failure): {
    id: $id,
    input_contract_ids: $inputs,
    output_contract_ids: $outputs,
    failure_mode: $failure,
    executes_replay: false
  };
  def readback($id; $collection; $inputs; $evidence; $gate): {
    id: $id,
    collection_id: $collection,
    required_inputs: $inputs,
    evidence_fields: $evidence,
    promotion_gate: $gate,
    mutates_store: false
  };
  def drift($id; $fields; $severity): {
    id: $id,
    compared_fields: $fields,
    severity: $severity,
    blocks_promotion: true
  };
  def recovery($id; $triggers; $action): {
    id: $id,
    trigger_detector_ids: $triggers,
    recovery_action: $action,
    requires_operator_approval: true,
    executes_recovery: false
  };
  def invariant($id; $reason): {
    id: $id,
    required: true,
    reason: $reason
  };
  def event_plan($id; $event; $sources; $collections; $keys; $readbacks; $blocking_sources; $blocking_reasons): {
    id: $id,
    event_contract_id: $event,
    source_surface_ids: $sources,
    expected_collection_ids: $collections,
    deterministic_replay_key_fields: $keys,
    readback_assertion_ids: $readbacks,
    blocking_source_surface_ids: $blocking_sources,
    blocking_reason_ids: $blocking_reasons,
    executes_replay: false,
    performs_readback: false,
    mutates_store: false
  };
  def source_gap($id; $source; $missing; $events; $fix): {
    id: $id,
    source_surface_id: $source,
    missing_capability: $missing,
    affected_event_contract_ids: $events,
    required_before_replay_execution: true,
    recommended_fix: $fix
  };
  [
    stage("preview_load_wal_head"; ["walHeadHash", "walSegmentManifest"]; ["orderedWalSegmentRefs"]; "missing_or_untrusted_wal_manifest_blocks_replay"),
    stage("preview_validate_wal_hash_chain"; ["orderedWalSegmentRefs", "previousRecordHash"]; ["validatedWalHeadHash"]; "hash_mismatch_blocks_checkpoint_compare"),
    stage("preview_apply_idempotency_window"; ["validatedWalHeadHash", "sourceRecordKeys"]; ["dedupedRecordSet"]; "duplicate_key_collision_blocks_collection_materialization"),
    stage("preview_materialize_collections"; ["dedupedRecordSet", "collectionSchemas"]; ["materializedCollectionHashes"]; "schema_or_redaction_mismatch_blocks_readback"),
    stage("preview_compare_checkpoint"; ["materializedCollectionHashes", "checkpointHash"]; ["checkpointComparisonResult"]; "checkpoint_drift_blocks_promotion"),
    stage("preview_emit_readback_report"; ["checkpointComparisonResult", "readbackAssertions"]; ["redactedReadbackEvidenceRefs"]; "missing_readback_evidence_blocks_operator_summary")
  ] as $replay_stages
  | [
    readback("assert_nodes_readback_matches_wal"; "nodes"; ["traceId", "expectedNodeIds", "validatedWalHeadHash"]; ["nodeCount", "nodeHash", "missingNodeIds"]; "block_node_status_promotion_until_readback_matches"),
    readback("assert_edges_readback_matches_wal"; "edges"; ["traceId", "expectedEdgeIds", "validatedWalHeadHash"]; ["edgeCount", "edgeHash", "missingEdgeIds"]; "block_dependency_resolution_until_readback_matches"),
    readback("assert_task_results_readback_matches_wal"; "taskResults"; ["traceId", "taskId", "status", "validatedWalHeadHash"]; ["taskResultHash", "terminalStatusObserved", "evidenceRefs"]; "block_reducer_promotion_until_task_result_readback_matches"),
    readback("assert_artifacts_readback_matches_wal"; "artifacts"; ["producerNodeId", "artifactHash", "validatedWalHeadHash"]; ["artifactCount", "artifactHash", "redactionState"]; "block_handoff_until_artifact_readback_matches"),
    readback("assert_approvals_readback_matches_wal"; "approvals"; ["approvalId", "operatorScope", "expiresAtUnixMs"]; ["approvalHash", "approvalStatus", "operatorScopeHash"]; "block_scheduler_unblock_until_approval_readback_matches"),
    readback("assert_timeline_readback_matches_wal"; "timelineEvents"; ["traceId", "eventKind", "redactionState"]; ["timelineHash", "eventCount", "redactionState"]; "block_operator_audit_until_timeline_readback_matches")
  ] as $readback_assertions
  | [
    drift("detect_identity_drift"; ["nodeId", "edgeId", "taskId", "artifactId", "approvalId"]; "critical"),
    drift("detect_ordering_drift"; ["walOffset", "eventSequence", "parentTraceId"]; "critical"),
    drift("detect_status_drift"; ["status", "terminalStatusObserved", "promotionGate"]; "critical"),
    drift("detect_hash_drift"; ["walHeadHash", "checkpointHash", "collectionMerkleRoot"]; "critical"),
    drift("detect_redaction_drift"; ["redactionState", "payloadHash", "evidenceRefs"]; "high")
  ] as $drift_detectors
  | [
    recovery("preview_quarantine_checkpoint"; ["detect_hash_drift", "detect_ordering_drift"]; "mark checkpoint unusable and require WAL replay review"),
    recovery("preview_rebuild_projection_indexes"; ["detect_identity_drift", "detect_ordering_drift"]; "derive indexes from WAL again after operator review"),
    recovery("preview_hold_terminal_promotion"; ["detect_status_drift", "detect_hash_drift"]; "keep terminal status blocked until readback evidence is repaired"),
    recovery("preview_request_redaction_review"; ["detect_redaction_drift"]; "require privacy review before any replay evidence is exposed"),
    recovery(
      "preview_require_operator_replay_approval";
      ["detect_identity_drift", "detect_ordering_drift", "detect_status_drift", "detect_hash_drift", "detect_redaction_drift"];
      "operator must approve any future recovery execution path"
    )
  ] as $recovery_previews
  | [
    invariant("replay_is_deterministic"; "the same WAL and checkpoint inputs must yield the same materialized hashes"),
    invariant("readback_is_required_before_promotion"; "no terminal promotion, scheduler unblock, or handoff can proceed without readback"),
    invariant("drift_blocks_promotion"; "identity, ordering, status, hash, and redaction drift must block promotion"),
    invariant("recovery_requires_operator_approval"; "future recovery execution must be explicitly approved and traceable"),
    invariant("readback_evidence_is_redacted"; "readback evidence stores ids, hashes, and refs instead of raw private payloads"),
    invariant("replay_readback_preview_has_no_side_effects"; "this gate cannot replay WAL, read graph state, recover, promote, or persist drift")
  ] as $invariants
  | [
    event_plan("replay_plan_step_event_intake"; "plan_step_event_intake"; ["update_plan_tool", "plan_mode_proposed_plan_blocks", "app_server_turn_plan_notification"]; ["nodes", "edges", "timelineEvents"]; ["traceId", "turnId", "stepIndex", "proposalHash"]; ["assert_nodes_readback_matches_wal", "assert_edges_readback_matches_wal", "assert_timeline_readback_matches_wal"]; ["app_server_turn_plan_notification", "plan_mode_proposed_plan_blocks", "update_plan_tool"]; ["append_only_store_disabled_by_design", "event_intake_idempotency_guard_missing", "source_projection_not_contract_ready"]),
    event_plan("replay_agent_spawn_event_intake"; "agent_spawn_event_intake"; ["multi_agent_v2_thread_spawn"]; ["nodes", "edges", "timelineEvents"]; ["parentThreadId", "childThreadId", "roleId"]; ["assert_nodes_readback_matches_wal", "assert_edges_readback_matches_wal", "assert_timeline_readback_matches_wal"]; ["multi_agent_v2_thread_spawn"]; ["append_only_store_disabled_by_design", "terminal_task_result_enforcement_disabled"]),
    event_plan("replay_mailbox_delivery_event_intake"; "mailbox_delivery_event_intake"; ["multi_agent_v2_mailbox_wait"]; ["edges", "timelineEvents"]; ["traceId", "agentPath", "mailboxSeq"]; ["assert_edges_readback_matches_wal", "assert_timeline_readback_matches_wal"]; ["multi_agent_v2_mailbox_wait"]; ["append_only_store_disabled_by_design", "event_intake_idempotency_guard_missing", "source_projection_not_contract_ready"]),
    event_plan("replay_agent_job_item_event_intake"; "agent_job_item_event_intake"; ["agent_jobs_batch_workers"]; ["nodes", "taskResults", "timelineEvents"]; ["jobId", "itemId", "attempt"]; ["assert_nodes_readback_matches_wal", "assert_task_results_readback_matches_wal", "assert_timeline_readback_matches_wal"]; ["agent_jobs_batch_workers"]; ["append_only_store_disabled_by_design", "terminal_task_result_enforcement_disabled"]),
    event_plan("replay_worker_task_event_intake"; "worker_task_event_intake"; ["hepta_runtime_task_board", "hepta_runtime_worker_tasks"]; ["nodes", "taskResults", "artifacts", "timelineEvents"]; ["workerTaskId", "attempt", "artifactHash"]; ["assert_nodes_readback_matches_wal", "assert_task_results_readback_matches_wal", "assert_artifacts_readback_matches_wal", "assert_timeline_readback_matches_wal"]; ["hepta_runtime_task_board", "hepta_runtime_worker_tasks"]; ["append_only_store_disabled_by_design", "event_intake_idempotency_guard_missing", "source_projection_not_contract_ready", "terminal_task_result_enforcement_disabled"]),
    event_plan("replay_scheduler_run_event_intake"; "scheduler_run_event_intake"; ["hepta_runtime_scheduler_store"]; ["nodes", "edges", "taskResults", "timelineEvents"]; ["schedulerRunId", "leaseId", "admissionDecision"]; ["assert_nodes_readback_matches_wal", "assert_edges_readback_matches_wal", "assert_task_results_readback_matches_wal", "assert_timeline_readback_matches_wal"]; ["hepta_runtime_scheduler_store"]; ["append_only_store_disabled_by_design", "terminal_task_result_enforcement_disabled"]),
    event_plan("replay_artifact_event_intake"; "artifact_event_intake"; ["hepta_runtime_worker_tasks", "hepta_runtime_agent_harness"]; ["artifacts", "timelineEvents"]; ["artifactId", "producerNodeId", "artifactHash"]; ["assert_artifacts_readback_matches_wal", "assert_timeline_readback_matches_wal"]; ["hepta_runtime_agent_harness", "hepta_runtime_worker_tasks"]; ["append_only_store_disabled_by_design", "terminal_task_result_enforcement_disabled"]),
    event_plan("replay_approval_event_intake"; "approval_event_intake"; ["hepta_runtime_approval_broker"]; ["approvals", "timelineEvents"]; ["approvalId", "operatorScope", "requestHash"]; ["assert_approvals_readback_matches_wal", "assert_timeline_readback_matches_wal"]; ["hepta_runtime_approval_broker"]; ["append_only_store_disabled_by_design", "source_projection_not_contract_ready"]),
    event_plan("replay_task_result_event_intake"; "task_result_event_intake"; ["multi_agent_v2_thread_spawn", "hepta_runtime_multi_agent_reducer", "agent_jobs_batch_workers", "hepta_runtime_worker_tasks", "hepta_runtime_scheduler_store", "hepta_runtime_agent_harness"]; ["taskResults", "timelineEvents"]; ["traceId", "taskId", "status", "evidenceHash"]; ["assert_task_results_readback_matches_wal", "assert_timeline_readback_matches_wal"]; ["agent_jobs_batch_workers", "hepta_runtime_agent_harness", "hepta_runtime_multi_agent_reducer", "hepta_runtime_scheduler_store", "hepta_runtime_worker_tasks", "multi_agent_v2_thread_spawn"]; ["append_only_store_disabled_by_design", "event_intake_idempotency_guard_missing", "source_projection_not_contract_ready", "terminal_task_result_enforcement_disabled"])
  ] as $append_only_event_replay_plans
  | [
    source_gap("gap_plan_mode_proposed_plan_blocks_replay_key"; "plan_mode_proposed_plan_blocks"; "stable_plan_block_projection_idempotency_guard"; ["plan_step_event_intake"]; "derive replay key from traceId, turnId, stepIndex, and proposalHash before readback can materialize plan nodes"),
    source_gap("gap_app_server_turn_plan_notification_replay_key"; "app_server_turn_plan_notification"; "stable_turn_plan_notification_idempotency_guard"; ["plan_step_event_intake"]; "derive replay key from traceId, turnId, notification sequence, and proposalHash before app-server plan readback"),
    source_gap("gap_multi_agent_mailbox_delivery_replay_key"; "multi_agent_v2_mailbox_wait"; "mailbox_delivery_idempotency_guard_and_task_result_join"; ["mailbox_delivery_event_intake"]; "promote mailbox seq plus agentPath into a replay key and join wait results to timeline evidence refs"),
    source_gap("gap_multi_agent_reducer_task_result_replay_key"; "hepta_runtime_multi_agent_reducer"; "reducer_task_result_idempotency_guard"; ["task_result_event_intake"]; "derive reducer output keys from traceId, taskId, reducer strategy, status, and evidenceHash before terminal replay"),
    source_gap("gap_task_board_worker_task_replay_key"; "hepta_runtime_task_board"; "task_board_worker_task_projection_idempotency_guard"; ["worker_task_event_intake"]; "derive task board replay keys from workerTaskId, attempt, lane, leaseState, and artifactHash before scheduler readback")
  ] as $source_readback_gaps
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_replay_readback_preview_gate",
      schema_version: "work_graph_replay_readback_preview_v1",
      preview_mode: "read_only_replay_readback_contract_preview_no_replay",
      replay_stage_count: ($replay_stages | length),
      readback_assertion_count: ($readback_assertions | length),
      drift_detector_count: ($drift_detectors | length),
      recovery_preview_count: ($recovery_previews | length),
      invariant_count: ($invariants | length),
      append_only_event_contract_count: 9,
      append_only_source_route_count: 12,
      event_replay_plan_count: ($append_only_event_replay_plans | length),
      source_readback_gap_count: ($source_readback_gaps | length),
      required_prior_gates: prior_gates,
      replay_stages: $replay_stages,
      readback_assertions: $readback_assertions,
      drift_detectors: $drift_detectors,
      recovery_previews: $recovery_previews,
      invariants: $invariants,
      append_only_event_replay_plans: $append_only_event_replay_plans,
      source_readback_gaps: $source_readback_gaps,
      recommended_next_gate: "hepta_work_graph_idempotency_readback_adapter_preview_gate",
      ready_for_promotion_precondition_preview: true,
      ready_for_replay_execution: false,
      ready_for_live_execution: false,
      source_probes: {
        replay_readback: {
          rust_module_present: $replay_readback_rust_module_present,
          report_script_present: $replay_readback_report_script_present,
          gate_script_present: $replay_readback_gate_script_present
        },
        append_only_event_intake: {
          rust_module_present: $append_only_event_intake_rust_module_present,
          report_script_present: $append_only_event_intake_report_script_present,
          gate_script_present: $append_only_event_intake_gate_script_present
        },
        unified_projection_audit: {
          rust_module_present: $unified_projection_audit_rust_module_present
        },
        state_store_persistence: {
          rust_module_present: $state_store_persistence_rust_module_present,
          report_script_present: $state_store_persistence_report_script_present,
          gate_script_present: $state_store_persistence_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        event_record_persisted: false,
        graph_state_persisted: false,
        wal_replayed: false,
        checkpoint_loaded: false,
        idempotency_index_mutated: false,
        readback_performed: false,
        drift_state_persisted: false,
        recovery_performed: false,
        promotion_performed: false,
        runtime_mutation_performed: false,
        scheduler_cutover_performed: false,
        adapter_projection_enforced: false,
        approval_recorded: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
