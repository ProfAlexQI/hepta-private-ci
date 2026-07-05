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

shadow_promotion_precondition_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_terminal_task_result_wrapper_shadow_activation_promotion_precondition_preview.rs
)"
shadow_promotion_precondition_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-terminal-task-result-wrapper-shadow-activation-promotion-precondition-preview-report.sh
)"
shadow_promotion_precondition_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-terminal-task-result-wrapper-shadow-activation-promotion-precondition-preview-gate.sh
)"
shadow_drift_budget_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_terminal_task_result_wrapper_shadow_activation_drift_budget_preview.rs
)"
shadow_drift_budget_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-terminal-task-result-wrapper-shadow-activation-drift-budget-preview-gate.sh
)"

jq -n \
  --argjson shadow_promotion_precondition_rust_module_present "$shadow_promotion_precondition_rust_module_present" \
  --argjson shadow_promotion_precondition_report_script_present "$shadow_promotion_precondition_report_script_present" \
  --argjson shadow_promotion_precondition_gate_script_present "$shadow_promotion_precondition_gate_script_present" \
  --argjson shadow_drift_budget_rust_module_present "$shadow_drift_budget_rust_module_present" \
  --argjson shadow_drift_budget_gate_script_present "$shadow_drift_budget_gate_script_present" \
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
    "hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_promotion_precondition_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_activation_blocker_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_readback_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_drift_budget_preview_gate"
  ];
  def precondition_ids: [
    "all_shadow_activation_drift_budgets_zero_tolerance",
    "shadow_operator_summaries_reviewed",
    "shadow_side_effect_lock_zero_mutation_required",
    "shadow_activation_execution_remains_disabled_until_budget_review"
  ];
  def budget_ids: [
    "shadow_surface_state_drift_zero_tolerance_budget",
    "shadow_blocker_binding_drift_zero_tolerance_budget",
    "shadow_enablement_satisfaction_drift_zero_tolerance_budget",
    "shadow_kill_switch_armament_drift_zero_tolerance_budget",
    "shadow_side_effect_lock_drift_zero_tolerance_budget"
  ];
  def summary_ids: [
    "shadow_surface_state_drift_operator_summary",
    "shadow_blocker_binding_drift_operator_summary",
    "shadow_enablement_satisfaction_drift_operator_summary",
    "shadow_kill_switch_armament_drift_operator_summary",
    "shadow_side_effect_lock_drift_operator_summary"
  ];
  def target($id; $surface; $category; $receipt): {
    id: $id,
    activation_surface_id: $surface,
    activation_category: $category,
    required_precondition_ids: precondition_ids,
    required_drift_budget_ids: budget_ids,
    required_operator_summary_ids: summary_ids,
    audit_receipt_id: $receipt,
    promotion_state: "blocked_preview_only",
    blocks_activation: true,
    blocks_promotion_execution: true,
    promotes_state: false
  };
  def binding($id; $budget_ids; $summary_ids; $fields; $blocker): {
    id: $id,
    source_gate_id: "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_drift_budget_preview_gate",
    required_budget_ids: $budget_ids,
    required_summary_ids: $summary_ids,
    required_evidence_fields: $fields,
    failure_blocker_id: $blocker,
    blocks_activation: true,
    blocks_promotion: true,
    currently_satisfied: false
  };
  def blocker($id; $severity; $targets; $message): {
    id: $id,
    severity: $severity,
    blocks_target_ids: $targets,
    operator_message: $message,
    required_before_shadow_activation: true,
    required_before_shadow_promotion: true
  };
  def receipt($id; $target): {
    id: $id,
    target_id: $target,
    required_fields: [
      "activationSurfaceId",
      "activationCategory",
      "traceId",
      "budgetIds",
      "operatorSummaryIds",
      "preconditionIds",
      "blockerIds",
      "sideEffectLockHash",
      "redactedEvidenceRefs",
      "receiptHash"
    ],
    redaction_policy: "only ids, hashes, surface names, and blocker states are allowed",
    persists_receipt: false,
    authorizes_activation: false,
    external_delivery_enabled: false
  };
  def invariant($id; $reason): {
    id: $id,
    required: true,
    reason: $reason
  };
  [
    target("shadow_promote_wrapper_execution_activation"; "wrapper_execution_activation"; "runtime_execution"; "shadow_wrapper_execution_promotion_audit_receipt"),
    target("shadow_promote_readback_execution_activation"; "readback_execution_activation"; "readback_execution"; "shadow_readback_execution_promotion_audit_receipt"),
    target("shadow_promote_promotion_execution_activation"; "promotion_execution_activation"; "state_promotion"; "shadow_promotion_execution_promotion_audit_receipt"),
    target("shadow_promote_task_result_enforcement_activation"; "task_result_enforcement_activation"; "contract_enforcement"; "shadow_task_result_enforcement_promotion_audit_receipt"),
    target("shadow_promote_store_enablement_activation"; "store_enablement_activation"; "state_write"; "shadow_store_enablement_promotion_audit_receipt"),
    target("shadow_promote_live_execution_activation"; "live_execution_activation"; "live_runtime"; "shadow_live_execution_promotion_audit_receipt"),
    target("shadow_promote_external_delivery_activation"; "external_delivery_activation"; "external_side_effect"; "shadow_external_delivery_promotion_audit_receipt")
  ] as $targets
  | [
    binding("all_shadow_activation_drift_budgets_zero_tolerance"; budget_ids; summary_ids; ["maxAllowedMismatches", "maxAllowedUnreviewedFindings"]; "shadow_activation_drift_budgets_not_executed"),
    binding("shadow_operator_summaries_reviewed"; budget_ids; summary_ids; ["reviewerIdHash", "reviewedAtUnixMs", "summaryHash"]; "shadow_operator_review_missing"),
    binding("shadow_side_effect_lock_zero_mutation_required"; ["shadow_side_effect_lock_drift_zero_tolerance_budget"]; ["shadow_side_effect_lock_drift_operator_summary"]; ["activationPerformed", "taskResultEnforcementEnabled", "storePersistenceEnabled"]; "shadow_side_effect_lock_not_proven"),
    binding("shadow_activation_execution_remains_disabled_until_budget_review"; budget_ids; []; ["readyForShadowReadbackExecution", "readyForActivation", "readyForStoreEnablement"]; "shadow_promotion_execution_disabled")
  ] as $precondition_bindings
  | [
    blocker("shadow_activation_drift_budgets_not_executed"; "critical"; ($targets | map(.id)); "shadow activation drift budgets are declared but no shadow readback execution has produced findings"),
    blocker("shadow_operator_review_missing"; "high"; ($targets | map(.id)); "shadow activation operator summaries are preview-only and have not been reviewed"),
    blocker("shadow_side_effect_lock_not_proven"; "critical"; ($targets | map(.id)); "shadow side-effect lock evidence must remain zero before any activation or promotion"),
    blocker("shadow_runtime_attachment_disabled"; "medium"; ($targets | map(.id)); "shadow activation promotion preconditions are not attached to runtime activation paths"),
    blocker("shadow_promotion_execution_disabled"; "medium"; ($targets | map(.id)); "shadow promotion execution remains disabled until a later explicit activation cut")
  ] as $blockers
  | ($targets | map(receipt(.audit_receipt_id; .id))) as $audit_receipts
  | [
    invariant("shadow_preconditions_are_preview_only"; "shadow activation promotion preconditions cannot attach to runtime activation paths"),
    invariant("zero_tolerance_budgets_do_not_authorize_activation"; "zero-tolerance drift budgets block activation until readback evidence and review exist"),
    invariant("operator_summaries_do_not_record_approval"; "operator summaries remain non-persistent preview artifacts and do not imply approval"),
    invariant("audit_receipts_are_non_persistent"; "audit receipts are redacted preview contracts and cannot be stored or delivered"),
    invariant("side_effect_lock_stays_false"; "all side-effect, store, enforcement, activation, and delivery flags stay false"),
    invariant("shadow_activation_promotion_precondition_has_no_side_effects"; "this preview cannot promote, activate, execute wrappers, enforce TaskResult, write state, or send externally")
  ] as $invariants
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_promotion_precondition_preview_gate",
      schema_version: "work_graph_terminal_task_result_wrapper_shadow_activation_promotion_precondition_preview_v1",
      preview_mode: "read_only_terminal_task_result_wrapper_shadow_activation_promotion_precondition_preview_no_activation",
      target_count: ($targets | length),
      precondition_binding_count: ($precondition_bindings | length),
      blocker_count: ($blockers | length),
      audit_receipt_count: ($audit_receipts | length),
      invariant_count: ($invariants | length),
      required_prior_gate_count: (prior_gates | length),
      targets: $targets,
      precondition_bindings: $precondition_bindings,
      blockers: $blockers,
      audit_receipts: $audit_receipts,
      invariants: $invariants,
      required_prior_gates: prior_gates,
      recommended_next_gate: "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_preview_gate",
      ready_for_shadow_activation_activation_blocker_preview: true,
      ready_for_shadow_promotion_execution: false,
      ready_for_activation: false,
      ready_for_wrapper_execution: false,
      ready_for_task_result_enforcement: false,
      ready_for_store_enablement: false,
      ready_for_live_execution: false,
      source_probes: {
        terminal_task_result_wrapper_shadow_activation_promotion_precondition: {
          rust_module_present: $shadow_promotion_precondition_rust_module_present,
          report_script_present: $shadow_promotion_precondition_report_script_present,
          gate_script_present: $shadow_promotion_precondition_gate_script_present
        },
        terminal_task_result_wrapper_shadow_activation_drift_budget: {
          rust_module_present: $shadow_drift_budget_rust_module_present,
          gate_script_present: $shadow_drift_budget_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        shadow_promotion_precondition_persisted: false,
        audit_receipt_persisted: false,
        promotion_state_mutated: false,
        shadow_readback_performed: false,
        shadow_activation_performed: false,
        activation_state_mutated: false,
        activation_performed: false,
        promotion_performed: false,
        wrapper_executed: false,
        readback_performed: false,
        task_result_enforcement_enabled: false,
        store_persistence_enabled: false,
        event_record_persisted: false,
        task_result_persisted: false,
        graph_state_persisted: false,
        wal_written: false,
        checkpoint_written: false,
        scheduler_admission_enforced: false,
        replay_executed: false,
        approval_recorded: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
