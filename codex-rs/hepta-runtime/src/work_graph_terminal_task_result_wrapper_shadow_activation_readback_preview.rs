use serde::Serialize;

pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_readback_preview_gate";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_READBACK_SCHEMA_VERSION: &str =
    "work_graph_terminal_task_result_wrapper_shadow_activation_readback_preview_v1";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_drift_budget_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperShadowActivationReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub shadow_readback_plan_count: usize,
    pub activation_surface_assertion_count: usize,
    pub drift_detector_count: usize,
    pub operator_summary_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub shadow_readback_plans: Vec<WorkGraphTaskResultWrapperShadowActivationReadbackPlanPreview>,
    pub activation_surface_assertions:
        Vec<WorkGraphTaskResultWrapperShadowActivationAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphTaskResultWrapperShadowActivationDriftDetectorPreview>,
    pub operator_summaries: Vec<WorkGraphTaskResultWrapperShadowActivationOperatorSummaryPreview>,
    pub blockers: Vec<WorkGraphTaskResultWrapperShadowActivationReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_shadow_activation_drift_budget_preview: bool,
    pub ready_for_shadow_readback_execution: bool,
    pub ready_for_activation: bool,
    pub ready_for_promotion_execution: bool,
    pub ready_for_wrapper_execution: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_store_enablement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphTerminalTaskResultWrapperShadowActivationReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationReadbackPlanPreview {
    pub id: &'static str,
    pub activation_surface_id: &'static str,
    pub risk_class: &'static str,
    pub expected_activation_state: &'static str,
    pub expected_blocker_ids: Vec<&'static str>,
    pub required_assertion_ids: Vec<&'static str>,
    pub drift_detector_ids: Vec<&'static str>,
    pub shadow_readback_state: &'static str,
    pub performs_shadow_activation: bool,
    pub performs_readback: bool,
    pub mutates_activation_state: bool,
    pub mutates_store: bool,
    pub enables_runtime_mutation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationAssertionPreview {
    pub id: &'static str,
    pub assertion_scope: &'static str,
    pub required_inputs: Vec<&'static str>,
    pub evidence_fields: Vec<&'static str>,
    pub blocks_activation: bool,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationDriftDetectorPreview {
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub severity: &'static str,
    pub blocks_activation: bool,
    pub persists_drift: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationOperatorSummaryPreview {
    pub id: &'static str,
    pub summary_scope: &'static str,
    pub required_fields: Vec<&'static str>,
    pub redacted: bool,
    pub persists_summary: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_activation_surface_ids: Vec<&'static str>,
    pub required_before_shadow_readback_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperShadowActivationReadbackPreviewSideEffects {
    pub filesystem_written: bool,
    pub shadow_activation_performed: bool,
    pub shadow_readback_performed: bool,
    pub activation_state_mutated: bool,
    pub activation_performed: bool,
    pub promotion_performed: bool,
    pub wrapper_executed: bool,
    pub readback_performed: bool,
    pub task_result_enforcement_enabled: bool,
    pub store_persistence_enabled: bool,
    pub event_record_persisted: bool,
    pub task_result_persisted: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub scheduler_admission_enforced: bool,
    pub replay_executed: bool,
    pub approval_recorded: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_terminal_task_result_wrapper_shadow_activation_readback_preview_report()
-> WorkGraphTerminalTaskResultWrapperShadowActivationReadbackPreviewReport {
    let shadow_readback_plans =
        work_graph_terminal_task_result_wrapper_shadow_activation_readback_plans();
    let activation_surface_assertions =
        work_graph_terminal_task_result_wrapper_shadow_activation_assertions();
    let drift_detectors =
        work_graph_terminal_task_result_wrapper_shadow_activation_drift_detectors();
    let operator_summaries =
        work_graph_terminal_task_result_wrapper_shadow_activation_operator_summaries();
    let blockers = work_graph_terminal_task_result_wrapper_shadow_activation_readback_blockers();
    let required_prior_gates =
        work_graph_terminal_task_result_wrapper_shadow_activation_required_prior_gates();

    WorkGraphTerminalTaskResultWrapperShadowActivationReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_READBACK_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_terminal_task_result_wrapper_shadow_activation_readback_preview_no_execution",
        shadow_readback_plan_count: shadow_readback_plans.len(),
        activation_surface_assertion_count: activation_surface_assertions.len(),
        drift_detector_count: drift_detectors.len(),
        operator_summary_count: operator_summaries.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        shadow_readback_plans,
        activation_surface_assertions,
        drift_detectors,
        operator_summaries,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_shadow_activation_drift_budget_preview: true,
        ready_for_shadow_readback_execution: false,
        ready_for_activation: false,
        ready_for_promotion_execution: false,
        ready_for_wrapper_execution: false,
        ready_for_task_result_enforcement: false,
        ready_for_store_enablement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphTerminalTaskResultWrapperShadowActivationReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_surface_ids() -> Vec<&'static str>
{
    vec![
        "wrapper_execution_activation",
        "readback_execution_activation",
        "promotion_execution_activation",
        "task_result_enforcement_activation",
        "store_enablement_activation",
        "live_execution_activation",
        "external_delivery_activation",
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_assertion_ids() -> Vec<&'static str>
{
    vec![
        "assert_shadow_surface_stays_blocked",
        "assert_shadow_blocker_set_matches_activation_gate",
        "assert_shadow_enablements_remain_unsatisfied",
        "assert_shadow_kill_switches_remain_preview_only",
        "assert_shadow_side_effect_lock_remains_false",
        "assert_shadow_prior_gate_chain_matches",
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_drift_detector_ids()
-> Vec<&'static str> {
    vec![
        "detect_shadow_activation_surface_state_drift",
        "detect_shadow_activation_blocker_binding_drift",
        "detect_shadow_enablement_satisfaction_drift",
        "detect_shadow_kill_switch_armament_drift",
        "detect_shadow_side_effect_lock_drift",
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_readback_plans()
-> Vec<WorkGraphTaskResultWrapperShadowActivationReadbackPlanPreview> {
    vec![
        shadow_plan(
            "shadow_readback_wrapper_execution_activation",
            "wrapper_execution_activation",
            "runtime_execution",
            vec![
                "promotion_preconditions_unsatisfied",
                "wrapper_execution_disabled",
                "runtime_attachment_disabled",
                "kill_switches_preview_only",
            ],
        ),
        shadow_plan(
            "shadow_readback_readback_execution_activation",
            "readback_execution_activation",
            "readback_execution",
            vec![
                "promotion_preconditions_unsatisfied",
                "readback_execution_disabled",
                "zero_tolerance_drift_not_executed",
                "operator_review_missing",
            ],
        ),
        shadow_plan(
            "shadow_readback_promotion_execution_activation",
            "promotion_execution_activation",
            "state_promotion",
            vec![
                "promotion_preconditions_unsatisfied",
                "audit_receipt_persistence_disabled",
                "promotion_execution_disabled",
                "runtime_attachment_disabled",
            ],
        ),
        shadow_plan(
            "shadow_readback_task_result_enforcement_activation",
            "task_result_enforcement_activation",
            "contract_enforcement",
            vec![
                "promotion_preconditions_unsatisfied",
                "task_result_enforcement_disabled",
                "zero_tolerance_drift_not_executed",
                "redaction_precondition_unsatisfied",
            ],
        ),
        shadow_plan(
            "shadow_readback_store_enablement_activation",
            "store_enablement_activation",
            "state_write",
            vec![
                "promotion_preconditions_unsatisfied",
                "store_enablement_disabled",
                "audit_receipt_persistence_disabled",
                "runtime_attachment_disabled",
            ],
        ),
        shadow_plan(
            "shadow_readback_live_execution_activation",
            "live_execution_activation",
            "live_runtime",
            vec![
                "promotion_preconditions_unsatisfied",
                "live_execution_disabled",
                "kill_switches_preview_only",
                "operator_review_missing",
            ],
        ),
        shadow_plan(
            "shadow_readback_external_delivery_activation",
            "external_delivery_activation",
            "external_side_effect",
            vec![
                "promotion_preconditions_unsatisfied",
                "external_delivery_disabled",
                "redaction_precondition_unsatisfied",
                "operator_review_missing",
            ],
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_assertions()
-> Vec<WorkGraphTaskResultWrapperShadowActivationAssertionPreview> {
    vec![
        assertion(
            "assert_shadow_surface_stays_blocked",
            "activation_surface",
            vec!["activationSurfaceId", "activationState", "blockedByDefault"],
            vec!["activationStateHash", "blockedSurfaceCount", "riskClass"],
        ),
        assertion(
            "assert_shadow_blocker_set_matches_activation_gate",
            "blocker_set",
            vec!["activationSurfaceId", "requiredBlockerIds", "blockerIds"],
            vec!["blockerSetHash", "criticalBlockerCount", "denialReasons"],
        ),
        assertion(
            "assert_shadow_enablements_remain_unsatisfied",
            "required_enablement",
            vec![
                "enablementId",
                "requiredEvidenceFields",
                "currentlySatisfied",
            ],
            vec![
                "enablementHash",
                "unsatisfiedEnablementCount",
                "sourceGateIds",
            ],
        ),
        assertion(
            "assert_shadow_kill_switches_remain_preview_only",
            "kill_switch",
            vec!["killSwitchId", "targetSurfaceIds", "armedInPreview"],
            vec!["killSwitchHash", "armedPreviewCount", "targetSurfaceCount"],
        ),
        assertion(
            "assert_shadow_side_effect_lock_remains_false",
            "side_effects",
            vec![
                "sideEffects",
                "activationPerformed",
                "storePersistenceEnabled",
            ],
            vec!["sideEffectHash", "falseFieldCount", "mutationAttemptCount"],
        ),
        assertion(
            "assert_shadow_prior_gate_chain_matches",
            "prior_gate_chain",
            vec!["requiredPriorGates", "lastPriorGate", "recommendedNextGate"],
            vec!["priorGateHash", "priorGateCount", "frontierGate"],
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_drift_detectors()
-> Vec<WorkGraphTaskResultWrapperShadowActivationDriftDetectorPreview> {
    vec![
        detector(
            "detect_shadow_activation_surface_state_drift",
            vec!["activationSurfaceId", "activationState", "blockedByDefault"],
        ),
        detector(
            "detect_shadow_activation_blocker_binding_drift",
            vec!["activationSurfaceId", "requiredBlockerIds", "blockerIds"],
        ),
        detector(
            "detect_shadow_enablement_satisfaction_drift",
            vec![
                "enablementId",
                "currentlySatisfied",
                "requiredEvidenceFields",
            ],
        ),
        detector(
            "detect_shadow_kill_switch_armament_drift",
            vec!["killSwitchId", "armedInPreview", "targetSurfaceIds"],
        ),
        detector(
            "detect_shadow_side_effect_lock_drift",
            vec![
                "activationPerformed",
                "taskResultEnforcementEnabled",
                "storePersistenceEnabled",
            ],
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_operator_summaries()
-> Vec<WorkGraphTaskResultWrapperShadowActivationOperatorSummaryPreview> {
    vec![
        operator_summary(
            "shadow_activation_denial_summary",
            "activation_surfaces",
            vec![
                "blockedSurfaceCount",
                "criticalBlockerCount",
                "readyForActivation",
            ],
        ),
        operator_summary(
            "shadow_enablement_gap_summary",
            "required_enablements",
            vec![
                "unsatisfiedEnablementCount",
                "sourceGateIds",
                "requiredEvidenceFields",
            ],
        ),
        operator_summary(
            "shadow_kill_switch_summary",
            "kill_switches",
            vec!["killSwitchCount", "armedPreviewOnly", "targetSurfaceIds"],
        ),
        operator_summary(
            "shadow_external_delivery_denial_summary",
            "external_delivery",
            vec![
                "externalDeliveryDisabled",
                "redactionPreconditionUnsatisfied",
                "operatorReviewMissing",
            ],
        ),
        operator_summary(
            "shadow_store_enforcement_denial_summary",
            "store_and_enforcement",
            vec![
                "storeEnablementDisabled",
                "taskResultEnforcementDisabled",
                "sideEffectsFalse",
            ],
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_readback_blockers()
-> Vec<WorkGraphTaskResultWrapperShadowActivationReadbackBlockerPreview> {
    vec![
        blocker(
            "shadow_readback_execution_disabled",
            "high",
            work_graph_terminal_task_result_wrapper_shadow_activation_surface_ids(),
            "keep shadow activation readback as contract-only until drift budget preview is reviewed",
        ),
        blocker(
            "activation_state_persistence_disabled",
            "high",
            work_graph_terminal_task_result_wrapper_shadow_activation_surface_ids(),
            "do not persist activation state before explicit future store enablement",
        ),
        blocker(
            "operator_review_missing",
            "high",
            vec![
                "live_execution_activation",
                "external_delivery_activation",
                "readback_execution_activation",
            ],
            "attach operator-readable summaries before any readback execution or live activation",
        ),
        blocker(
            "shadow_activation_drift_budget_missing",
            "critical",
            work_graph_terminal_task_result_wrapper_shadow_activation_surface_ids(),
            "add a zero-tolerance drift budget for shadow activation readback before any activation",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_required_prior_gates()
-> Vec<&'static str> {
    vec![
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
    ]
}

impl WorkGraphTerminalTaskResultWrapperShadowActivationReadbackPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            shadow_activation_performed: false,
            shadow_readback_performed: false,
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
            model_invoked: false,
        }
    }
}

fn shadow_plan(
    id: &'static str,
    activation_surface_id: &'static str,
    risk_class: &'static str,
    expected_blocker_ids: Vec<&'static str>,
) -> WorkGraphTaskResultWrapperShadowActivationReadbackPlanPreview {
    WorkGraphTaskResultWrapperShadowActivationReadbackPlanPreview {
        id,
        activation_surface_id,
        risk_class,
        expected_activation_state: "blocked_preview_only",
        expected_blocker_ids,
        required_assertion_ids:
            work_graph_terminal_task_result_wrapper_shadow_activation_assertion_ids(),
        drift_detector_ids:
            work_graph_terminal_task_result_wrapper_shadow_activation_drift_detector_ids(),
        shadow_readback_state: "preview_contract_defined_shadow_readback_execution_disabled",
        performs_shadow_activation: false,
        performs_readback: false,
        mutates_activation_state: false,
        mutates_store: false,
        enables_runtime_mutation: false,
    }
}

fn assertion(
    id: &'static str,
    assertion_scope: &'static str,
    required_inputs: Vec<&'static str>,
    evidence_fields: Vec<&'static str>,
) -> WorkGraphTaskResultWrapperShadowActivationAssertionPreview {
    WorkGraphTaskResultWrapperShadowActivationAssertionPreview {
        id,
        assertion_scope,
        required_inputs,
        evidence_fields,
        blocks_activation: true,
        performs_readback: false,
        mutates_store: false,
    }
}

fn detector(
    id: &'static str,
    compared_fields: Vec<&'static str>,
) -> WorkGraphTaskResultWrapperShadowActivationDriftDetectorPreview {
    WorkGraphTaskResultWrapperShadowActivationDriftDetectorPreview {
        id,
        compared_fields,
        severity: "critical",
        blocks_activation: true,
        persists_drift: false,
    }
}

fn operator_summary(
    id: &'static str,
    summary_scope: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphTaskResultWrapperShadowActivationOperatorSummaryPreview {
    WorkGraphTaskResultWrapperShadowActivationOperatorSummaryPreview {
        id,
        summary_scope,
        required_fields,
        redacted: true,
        persists_summary: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    affected_activation_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphTaskResultWrapperShadowActivationReadbackBlockerPreview {
    WorkGraphTaskResultWrapperShadowActivationReadbackBlockerPreview {
        id,
        severity,
        affected_activation_surface_ids,
        required_before_shadow_readback_execution: true,
        recommended_fix,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_activation_readback_declares_all_activation_surfaces() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_readback_preview_report(
            );

        assert_eq!(report.shadow_readback_plan_count, 7);
        assert_eq!(
            report
                .shadow_readback_plans
                .iter()
                .map(|plan| plan.activation_surface_id)
                .collect::<Vec<_>>(),
            work_graph_terminal_task_result_wrapper_shadow_activation_surface_ids()
        );
        assert!(report.shadow_readback_plans.iter().all(|plan| {
            plan.expected_activation_state == "blocked_preview_only"
                && !plan.performs_shadow_activation
                && !plan.performs_readback
                && !plan.mutates_activation_state
                && !plan.mutates_store
                && !plan.enables_runtime_mutation
        }));
    }

    #[test]
    fn shadow_activation_readback_keeps_assertions_and_drift_critical() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_readback_preview_report(
            );

        assert_eq!(report.activation_surface_assertion_count, 6);
        assert!(
            report
                .activation_surface_assertions
                .iter()
                .all(|assertion| {
                    assertion.blocks_activation
                        && !assertion.performs_readback
                        && !assertion.mutates_store
                })
        );
        assert_eq!(report.drift_detector_count, 5);
        assert!(
            report
                .drift_detectors
                .iter()
                .all(|detector| detector.severity == "critical"
                    && detector.blocks_activation
                    && !detector.persists_drift)
        );
    }

    #[test]
    fn shadow_activation_readback_summaries_are_non_persistent() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_readback_preview_report(
            );

        assert_eq!(report.operator_summary_count, 5);
        assert!(report.operator_summaries.iter().all(|summary| {
            summary.redacted && !summary.persists_summary && !summary.required_fields.is_empty()
        }));
    }

    #[test]
    fn shadow_activation_readback_blocks_execution_and_mutation() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_readback_preview_report(
            );

        assert!(report.ready_for_shadow_activation_drift_budget_preview);
        assert!(!report.ready_for_shadow_readback_execution);
        assert!(!report.ready_for_activation);
        assert!(!report.ready_for_promotion_execution);
        assert!(!report.ready_for_wrapper_execution);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_store_enablement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphTerminalTaskResultWrapperShadowActivationReadbackPreviewSideEffects::none()
        );
    }

    #[test]
    fn shadow_activation_readback_requires_activation_blocker_prior() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_readback_preview_report(
            );

        assert_eq!(report.blocker_count, 4);
        assert_eq!(report.required_prior_gate_count, 18);
        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_terminal_task_result_wrapper_activation_blocker_preview_gate")
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_READBACK_RECOMMENDED_NEXT_GATE
        );
    }
}
