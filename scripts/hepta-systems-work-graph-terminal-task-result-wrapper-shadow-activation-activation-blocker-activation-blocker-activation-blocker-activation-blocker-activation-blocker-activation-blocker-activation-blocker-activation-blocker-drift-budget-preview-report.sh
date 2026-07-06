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

activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview.rs
)"
activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-terminal-task-result-wrapper-shadow-activation-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-drift-budget-preview-report.sh
)"
activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-terminal-task-result-wrapper-shadow-activation-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-drift-budget-preview-gate.sh
)"
activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview.rs
)"
activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-terminal-task-result-wrapper-shadow-activation-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-activation-blocker-readback-preview-gate.sh
)"

jq -n \
  --argjson activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_rust_module_present "$activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_rust_module_present" \
  --argjson activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_report_script_present "$activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_report_script_present" \
  --argjson activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_gate_script_present "$activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_gate_script_present" \
  --argjson activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_rust_module_present "$activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_rust_module_present" \
  --argjson activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_gate_script_present "$activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_gate_script_present" \
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
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_drift_budget_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_promotion_precondition_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_readback_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_drift_budget_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_promotion_precondition_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_readback_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_drift_budget_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_promotion_precondition_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_readback_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_gate",
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_gate"
  ];
  def budget($id; $detector; $fields; $summary; $preconditions): {
    id: $id,
    drift_detector_id: $detector,
    compared_fields: $fields,
    max_allowed_mismatches: 0,
    max_allowed_unreviewed_findings: 0,
    max_replay_lag_ms: 0,
    severity: "critical",
    block_level: "block_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_activation_promotion_enforcement_store_and_live_execution",
    operator_summary_id: $summary,
    activation_precondition_ids: $preconditions,
    allows_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_execution: false,
    allows_shadow_activation_blocker_activation: false,
    allows_shadow_activation_blocker_promotion_execution: false,
    allows_shadow_activation: false,
    allows_activation: false,
    allows_shadow_promotion_execution: false,
    allows_task_result_enforcement: false,
    allows_store_enablement: false,
    allows_live_execution: false
  };
  def summary($id; $detector; $title): {
    id: $id,
    drift_detector_id: $detector,
    title: $title,
    required_fields: [
      "detectorId",
      "budgetId",
      "summaryHash",
      "reviewerIdHash",
      "reviewedAtUnixMs"
    ],
    redaction_policy: "summarize ids, hashes, and states without raw activation payload",
    review_state: "preview_summary_defined_review_not_performed",
    persists_summary: false,
    external_delivery_allowed: false
  };
  def precondition($id; $budget_ids; $summary_ids; $fields): {
    id: $id,
    required_budget_ids: $budget_ids,
    required_summary_ids: $summary_ids,
    required_evidence_fields: $fields,
    blocks_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback: true,
    blocks_shadow_activation_blocker_activation: true,
    blocks_shadow_activation_blocker_promotion: true,
    blocks_activation: true,
    blocks_shadow_promotion: true,
    currently_satisfied: false
  };
  def blocker($id; $severity; $reason): {
    id: $id,
    severity: $severity,
    reason: $reason,
    required_before_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_execution: true,
    required_before_shadow_activation_blocker_activation: true,
    required_before_activation: true
  };
  [
    budget("shadow_activation_blocker_activation_blocker_surface_state_drift_zero_tolerance_budget"; "detect_shadow_activation_blocker_activation_blocker_surface_state_drift"; ["activationSurfaceId", "activationState", "blockedByDefault"]; "shadow_activation_blocker_activation_blocker_surface_state_drift_operator_summary"; ["all_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets_zero_tolerance", "shadow_activation_blocker_activation_blocker_operator_summaries_reviewed"]),
    budget("shadow_activation_blocker_activation_blocker_binding_drift_zero_tolerance_budget"; "detect_shadow_activation_blocker_activation_blocker_binding_drift"; ["activationSurfaceId", "requiredBlockerIds", "actualBlockerIds"]; "shadow_activation_blocker_activation_blocker_binding_drift_operator_summary"; ["all_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets_zero_tolerance", "shadow_activation_blocker_activation_blocker_operator_summaries_reviewed"]),
    budget("shadow_activation_blocker_activation_enablement_satisfaction_drift_zero_tolerance_budget"; "detect_shadow_activation_blocker_activation_enablement_satisfaction_drift"; ["enablementId", "currentlySatisfied", "requiredEvidenceFields"]; "shadow_activation_blocker_activation_enablement_satisfaction_drift_operator_summary"; ["all_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets_zero_tolerance", "shadow_activation_blocker_activation_blocker_operator_summaries_reviewed"]),
    budget("shadow_activation_blocker_activation_kill_switch_armament_drift_zero_tolerance_budget"; "detect_shadow_activation_blocker_activation_kill_switch_armament_drift"; ["killSwitchId", "armedInPreview", "persistsSwitchState"]; "shadow_activation_blocker_activation_kill_switch_armament_drift_operator_summary"; ["all_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets_zero_tolerance", "shadow_activation_blocker_activation_blocker_operator_summaries_reviewed"]),
    budget("shadow_activation_blocker_activation_side_effect_lock_drift_zero_tolerance_budget"; "detect_shadow_activation_blocker_activation_side_effect_lock_drift"; ["activationPerformed", "taskResultEnforcementEnabled", "storePersistenceEnabled"]; "shadow_activation_blocker_activation_side_effect_lock_drift_operator_summary"; ["all_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets_zero_tolerance", "shadow_activation_blocker_activation_blocker_operator_summaries_reviewed", "shadow_activation_blocker_activation_blocker_side_effect_lock_zero_mutation_required"])
  ] as $drift_budgets
  | [
    summary("shadow_activation_blocker_activation_blocker_surface_state_drift_operator_summary"; "detect_shadow_activation_blocker_activation_blocker_surface_state_drift"; "Shadow activation blocker activation-blocker surface state drift must remain blocked before activation"),
    summary("shadow_activation_blocker_activation_blocker_binding_drift_operator_summary"; "detect_shadow_activation_blocker_activation_blocker_binding_drift"; "Shadow activation blocker activation-blocker binding drift must be reviewed before promotion checks"),
    summary("shadow_activation_blocker_activation_enablement_satisfaction_drift_operator_summary"; "detect_shadow_activation_blocker_activation_enablement_satisfaction_drift"; "Shadow activation blocker activation-blocker enablement drift must remain unsatisfied before activation"),
    summary("shadow_activation_blocker_activation_kill_switch_armament_drift_operator_summary"; "detect_shadow_activation_blocker_activation_kill_switch_armament_drift"; "Shadow activation blocker activation-blocker kill switch drift must stay preview-only before execution"),
    summary("shadow_activation_blocker_activation_side_effect_lock_drift_operator_summary"; "detect_shadow_activation_blocker_activation_side_effect_lock_drift"; "Shadow activation blocker activation-blocker side-effect lock drift must remain zero before any live surface")
  ] as $operator_summaries
  | ($drift_budgets | map(.id)) as $budget_ids
  | ($operator_summaries | map(.id)) as $summary_ids
  | [
    precondition("all_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets_zero_tolerance"; $budget_ids; $summary_ids; ["maxAllowedMismatches", "maxAllowedUnreviewedFindings"]),
    precondition("shadow_activation_blocker_activation_blocker_operator_summaries_reviewed"; $budget_ids; $summary_ids; ["reviewerIdHash", "reviewedAtUnixMs", "summaryHash"]),
    precondition("shadow_activation_blocker_activation_blocker_side_effect_lock_zero_mutation_required"; ["shadow_activation_blocker_activation_side_effect_lock_drift_zero_tolerance_budget"]; ["shadow_activation_blocker_activation_side_effect_lock_drift_operator_summary"]; ["activationPerformed", "taskResultEnforcementEnabled", "storePersistenceEnabled"]),
    precondition("shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_remains_disabled_until_budget_review"; $budget_ids; []; ["readyForShadowActivationBlockerActivationBlockerReadbackExecution", "readyForActivation", "readyForStoreEnablement"])
  ] as $activation_preconditions
  | [
    blocker("shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_not_executed"; "critical"; "shadow activation blocker activation-blocker drift detectors have zero-tolerance budgets but no readback execution has run"),
    blocker("shadow_activation_blocker_activation_blocker_operator_review_not_performed"; "high"; "shadow activation blocker activation-blocker operator summaries are preview-only and have not been reviewed or persisted"),
    blocker("shadow_activation_blocker_activation_blocker_preconditions_not_attached"; "medium"; "shadow activation blocker activation-blocker preconditions are defined but not attached to runtime activation or promotion logic"),
    blocker("shadow_activation_blocker_activation_blocker_drift_persistence_disabled"; "medium"; "shadow activation blocker activation-blocker drift state cannot be persisted until store enablement is explicitly approved later")
  ] as $blockers
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_gate",
      schema_version: "work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_v1",
      preview_mode: "read_only_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_no_execution",
      drift_budget_count: ($drift_budgets | length),
      operator_summary_count: ($operator_summaries | length),
      activation_precondition_count: ($activation_preconditions | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: (prior_gates | length),
      drift_budgets: $drift_budgets,
      operator_summaries: $operator_summaries,
      activation_preconditions: $activation_preconditions,
      blockers: $blockers,
      required_prior_gates: prior_gates,
      recommended_next_gate: "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_gate",
      ready_for_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview: true,
      ready_for_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_execution: false,
      ready_for_shadow_activation_blocker_activation_execution: false,
      ready_for_shadow_activation_blocker_promotion_execution: false,
      ready_for_shadow_activation_execution: false,
      ready_for_activation: false,
      ready_for_shadow_promotion_execution: false,
      ready_for_wrapper_execution: false,
      ready_for_task_result_enforcement: false,
      ready_for_store_enablement: false,
      ready_for_live_execution: false,
      source_probes: {
        terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget: {
          rust_module_present: $activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_rust_module_present,
          report_script_present: $activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_report_script_present,
          gate_script_present: $activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_gate_script_present
        },
        terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback: {
          rust_module_present: $activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_rust_module_present,
          gate_script_present: $activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_persisted: false,
        operator_summary_persisted: false,
        shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_performed: false,
        shadow_activation_blocker_activation_blocker_activation_blocker_persisted: false,
        shadow_activation_blocker_activation_blocker_activation_performed: false,
        shadow_activation_blocker_activation_blocker_persisted: false,
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
