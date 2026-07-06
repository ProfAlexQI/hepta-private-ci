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

promotion_precondition_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_terminal_task_result_wrapper_promotion_precondition_preview.rs
)"
promotion_precondition_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-terminal-task-result-wrapper-promotion-precondition-preview-report.sh
)"
promotion_precondition_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-terminal-task-result-wrapper-promotion-precondition-preview-gate.sh
)"
drift_budget_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-terminal-task-result-wrapper-drift-budget-preview-gate.sh
)"

jq -n \
  --argjson promotion_precondition_rust_module_present "$promotion_precondition_rust_module_present" \
  --argjson promotion_precondition_report_script_present "$promotion_precondition_report_script_present" \
  --argjson promotion_precondition_gate_script_present "$promotion_precondition_gate_script_present" \
  --argjson drift_budget_gate_script_present "$drift_budget_gate_script_present" \
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
    "hepta_work_graph_append_only_event_intake_preview_gate",
    "hepta_work_graph_replay_readback_preview_gate",
    "hepta_work_graph_idempotency_readback_adapter_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_fixture_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_readback_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_gate"
  ];
  def precondition_ids: [
    "all_critical_drift_budgets_zero_tolerance",
    "operator_summaries_reviewed",
    "redaction_drift_zero_leak_required",
    "execution_remains_disabled_until_budget_review"
  ];
  def budget_ids: [
    "identity_drift_zero_tolerance_budget",
    "status_drift_zero_tolerance_budget",
    "evidence_drift_zero_tolerance_budget",
    "verifier_drift_zero_tolerance_budget",
    "redaction_drift_zero_tolerance_budget"
  ];
  def summary_ids: [
    "identity_drift_operator_summary",
    "status_drift_operator_summary",
    "evidence_drift_operator_summary",
    "verifier_drift_operator_summary",
    "redaction_drift_operator_summary"
  ];
  def target($id; $fixture; $wrapper; $source; $receipt): {
    id: $id,
    fixture_id: $fixture,
    wrapper_id: $wrapper,
    source_surface_id: $source,
    target_collection_id: "taskResults",
    required_precondition_ids: precondition_ids,
    required_budget_ids: budget_ids,
    required_operator_summary_ids: summary_ids,
    audit_receipt_id: $receipt,
    promotion_state: "blocked_preview_only",
    blocks_task_result_enforcement: true,
    promotes_state: false
  };
  def binding($id; $fields; $blocker): {
    id: $id,
    source_gate_id: "hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_gate",
    required_evidence_fields: $fields,
    failure_blocker_id: $blocker,
    blocks_promotion: true,
    currently_satisfied: false
  };
  def blocker($id; $severity; $targets; $message): {
    id: $id,
    severity: $severity,
    blocks_target_ids: $targets,
    operator_message: $message,
    required_before_promotion: true
  };
  def receipt($id; $target): {
    id: $id,
    target_id: $target,
    required_fields: [
      "taskId",
      "traceId",
      "wrapperId",
      "fixtureId",
      "budgetIds",
      "operatorSummaryIds",
      "preconditionIds",
      "blockerIds",
      "redactedEvidenceRefs",
      "receiptHash"
    ],
    persistence_enabled: false,
    external_delivery_enabled: false
  };
  def invariant($id; $reason): {
    id: $id,
    required: true,
    reason: $reason
  };
  [
    target("promote_fixture_multi_agent_thread_spawn_success"; "fixture_multi_agent_thread_spawn_success"; "multi_agent_thread_spawn_terminal_task_result_wrapper"; "multi_agent_v2_thread_spawn"; "thread_spawn_promotion_audit_receipt"),
    target("promote_fixture_multi_agent_mailbox_wait_success"; "fixture_multi_agent_mailbox_wait_success"; "multi_agent_mailbox_wait_terminal_task_result_wrapper"; "multi_agent_v2_mailbox_wait"; "mailbox_wait_promotion_audit_receipt"),
    target("promote_fixture_multi_agent_reducer_ok"; "fixture_multi_agent_reducer_ok"; "multi_agent_reducer_terminal_task_result_wrapper"; "hepta_runtime_multi_agent_reducer"; "reducer_promotion_audit_receipt"),
    target("promote_fixture_agent_job_item_failed"; "fixture_agent_job_item_failed"; "agent_job_item_terminal_task_result_wrapper"; "agent_jobs_batch_workers"; "agent_job_item_promotion_audit_receipt"),
    target("promote_fixture_worker_task_blocked"; "fixture_worker_task_blocked"; "worker_task_terminal_task_result_wrapper"; "hepta_runtime_worker_tasks"; "worker_task_promotion_audit_receipt"),
    target("promote_fixture_task_board_success"; "fixture_task_board_success"; "task_board_terminal_task_result_wrapper"; "hepta_runtime_task_board"; "task_board_promotion_audit_receipt"),
    target("promote_fixture_scheduler_run_superseded"; "fixture_scheduler_run_superseded"; "scheduler_run_terminal_task_result_wrapper"; "hepta_runtime_scheduler_store"; "scheduler_run_promotion_audit_receipt"),
    target("promote_fixture_agent_harness_cancelled"; "fixture_agent_harness_cancelled"; "agent_harness_terminal_task_result_wrapper"; "hepta_runtime_agent_harness"; "agent_harness_promotion_audit_receipt")
  ] as $promotion_targets
  | [
    binding("all_critical_drift_budgets_zero_tolerance"; ["maxAllowedMismatches", "maxAllowedUnreviewedFindings"]; "critical_drift_budget_not_executed"),
    binding("operator_summaries_reviewed"; ["reviewerIdHash", "reviewedAtUnixMs", "summaryHash"]; "operator_review_not_performed"),
    binding("redaction_drift_zero_leak_required"; ["redactionState", "summaryHash", "externalDeliveryAllowed"]; "redaction_precondition_unsatisfied"),
    binding("execution_remains_disabled_until_budget_review"; ["readyForReadbackExecution", "readyForWrapperExecution", "readyForStoreEnablement"]; "runtime_promotion_attachment_disabled")
  ] as $precondition_bindings
  | [
    blocker("critical_drift_budget_not_executed"; "high"; ($promotion_targets | map(.id)); "zero-tolerance drift budgets are declared but no readback execution has produced findings"),
    blocker("operator_review_not_performed"; "high"; ($promotion_targets | map(.id)); "operator summaries are preview-only and have not been reviewed"),
    blocker("redaction_precondition_unsatisfied"; "critical"; ($promotion_targets | map(.id)); "redaction drift must remain zero before terminal TaskResult promotion can be considered"),
    blocker("runtime_promotion_attachment_disabled"; "medium"; ($promotion_targets | map(.id)); "promotion preconditions are not attached to runtime promotion or enforcement paths")
  ] as $blockers
  | ($promotion_targets | map(receipt(.audit_receipt_id; .id))) as $audit_receipts
  | [
    invariant("every_wrapper_target_requires_zero_tolerance_drift_budget"; "each terminal wrapper target must carry all five zero-tolerance drift budgets"),
    invariant("promotion_preconditions_block_without_operator_summaries"; "operator summaries must be reviewed before any future promotion path can proceed"),
    invariant("redaction_drift_blocks_all_live_surfaces"; "redaction drift blocks terminal TaskResult enforcement and every live surface"),
    invariant("audit_receipts_are_preview_only"; "promotion audit receipts carry hashes and refs but cannot be persisted or delivered"),
    invariant("promotion_precondition_preview_has_no_side_effects"; "this preview cannot promote, execute wrappers, enforce TaskResult, write state, or send externally")
  ] as $invariants
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_terminal_task_result_wrapper_promotion_precondition_preview_gate",
      schema_version: "work_graph_terminal_task_result_wrapper_promotion_precondition_preview_v1",
      preview_mode: "read_only_terminal_task_result_wrapper_promotion_precondition_preview_no_promotion",
      promotion_target_count: ($promotion_targets | length),
      precondition_binding_count: ($precondition_bindings | length),
      blocker_count: ($blockers | length),
      audit_receipt_count: ($audit_receipts | length),
      invariant_count: ($invariants | length),
      required_prior_gate_count: (prior_gates | length),
      promotion_targets: $promotion_targets,
      precondition_bindings: $precondition_bindings,
      blockers: $blockers,
      audit_receipts: $audit_receipts,
      invariants: $invariants,
      required_prior_gates: prior_gates,
      recommended_next_gate: "hepta_work_graph_terminal_task_result_wrapper_activation_blocker_preview_gate",
      ready_for_activation_blocker_preview: true,
      ready_for_promotion_execution: false,
      ready_for_wrapper_execution: false,
      ready_for_task_result_enforcement: false,
      ready_for_store_enablement: false,
      ready_for_live_execution: false,
      source_probes: {
        terminal_task_result_wrapper_promotion_precondition: {
          rust_module_present: $promotion_precondition_rust_module_present,
          report_script_present: $promotion_precondition_report_script_present,
          gate_script_present: $promotion_precondition_gate_script_present
        },
        terminal_task_result_wrapper_drift_budget: {
          gate_script_present: $drift_budget_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        promotion_state_mutated: false,
        promotion_performed: false,
        wrapper_executed: false,
        readback_performed: false,
        drift_budget_persisted: false,
        audit_receipt_persisted: false,
        event_record_persisted: false,
        task_result_persisted: false,
        graph_state_persisted: false,
        wal_written: false,
        checkpoint_written: false,
        task_result_enforcement_enabled: false,
        scheduler_admission_enforced: false,
        replay_executed: false,
        approval_recorded: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
