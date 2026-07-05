use serde::Serialize;

pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_READBACK_PREVIEW_GATE:
    &str =
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_gate";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_READBACK_SCHEMA_VERSION:
    &str =
    "work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_v1";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_READBACK_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_plan_count: usize,
    pub blocker_assertion_count: usize,
    pub drift_detector_count: usize,
    pub operator_summary_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_plans:
        Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackPlanPreview>,
    pub blocker_assertions:
        Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerAssertionPreview>,
    pub drift_detectors:
        Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftDetectorPreview>,
    pub operator_summaries:
        Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerOperatorSummaryPreview>,
    pub blockers:
        Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview: bool,
    pub ready_for_shadow_activation_blocker_activation_blocker_readback_execution: bool,
    pub ready_for_shadow_activation_blocker_activation_blocker_activation_execution: bool,
    pub ready_for_shadow_activation_blocker_activation_blocker_promotion_execution: bool,
    pub ready_for_shadow_activation_execution: bool,
    pub ready_for_activation: bool,
    pub ready_for_wrapper_execution: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_store_enablement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackPlanPreview
{
    pub id: &'static str,
    pub activation_surface_id: &'static str,
    pub risk_class: &'static str,
    pub source_gate_id: &'static str,
    pub expected_activation_state: &'static str,
    pub expected_blocker_ids: Vec<&'static str>,
    pub expected_enablement_ids: Vec<&'static str>,
    pub required_assertion_ids: Vec<&'static str>,
    pub drift_detector_ids: Vec<&'static str>,
    pub readback_state: &'static str,
    pub performs_readback: bool,
    pub performs_shadow_activation_blocker_activation_blocker_activation: bool,
    pub performs_shadow_activation: bool,
    pub mutates_activation_state: bool,
    pub mutates_store: bool,
    pub enables_runtime_mutation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerAssertionPreview
{
    pub id: &'static str,
    pub assertion_scope: &'static str,
    pub required_inputs: Vec<&'static str>,
    pub evidence_fields: Vec<&'static str>,
    pub blocks_activation: bool,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftDetectorPreview
{
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub severity: &'static str,
    pub blocks_activation: bool,
    pub persists_drift: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerOperatorSummaryPreview
{
    pub id: &'static str,
    pub summary_scope: &'static str,
    pub required_fields: Vec<&'static str>,
    pub redacted: bool,
    pub persists_summary: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackBlockerPreview
{
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_activation_surface_ids: Vec<&'static str>,
    pub required_before_readback_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackPreviewSideEffects
{
    pub filesystem_written: bool,
    pub shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_performed:
        bool,
    pub shadow_activation_blocker_activation_blocker_activation_blocker_persisted: bool,
    pub drift_detector_persisted: bool,
    pub operator_summary_persisted: bool,
    pub shadow_activation_blocker_activation_blocker_activation_performed: bool,
    pub shadow_activation_blocker_activation_blocker_persisted: bool,
    pub shadow_readback_performed: bool,
    pub shadow_activation_performed: bool,
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

pub fn hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_report()
-> WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackPreviewReport
{
    let readback_plans =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_plans();
    let blocker_assertions =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_assertions();
    let drift_detectors =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_detectors();
    let operator_summaries =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_operator_summaries();
    let blockers =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_blockers();
    let required_prior_gates =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_required_prior_gates();

    WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_READBACK_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_READBACK_SCHEMA_VERSION,
        preview_mode:
            "read_only_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_no_execution",
        readback_plan_count: readback_plans.len(),
        blocker_assertion_count: blocker_assertions.len(),
        drift_detector_count: drift_detectors.len(),
        operator_summary_count: operator_summaries.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_plans,
        blocker_assertions,
        drift_detectors,
        operator_summaries,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview:
            true,
        ready_for_shadow_activation_blocker_activation_blocker_readback_execution: false,
        ready_for_shadow_activation_blocker_activation_blocker_activation_execution: false,
        ready_for_shadow_activation_blocker_activation_blocker_promotion_execution: false,
        ready_for_shadow_activation_execution: false,
        ready_for_activation: false,
        ready_for_wrapper_execution: false,
        ready_for_task_result_enforcement: false,
        ready_for_store_enablement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_surface_ids()
-> Vec<&'static str> {
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

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_assertion_ids()
-> Vec<&'static str> {
    vec![
        "assert_shadow_activation_blocker_activation_blocker_surface_stays_blocked",
        "assert_shadow_activation_blocker_activation_blocker_set_matches_preview",
        "assert_shadow_activation_blocker_activation_blocker_activation_required_enablements_remain_unsatisfied",
        "assert_shadow_activation_blocker_activation_blocker_activation_kill_switches_remain_preview_only",
        "assert_shadow_activation_blocker_activation_blocker_activation_side_effect_lock_remains_false",
        "assert_shadow_activation_blocker_activation_blocker_activation_prior_gate_chain_matches",
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_detector_ids()
-> Vec<&'static str> {
    vec![
        "detect_shadow_activation_blocker_activation_blocker_surface_state_drift",
        "detect_shadow_activation_blocker_activation_blocker_binding_drift",
        "detect_shadow_activation_blocker_activation_blocker_activation_enablement_satisfaction_drift",
        "detect_shadow_activation_blocker_activation_blocker_activation_kill_switch_armament_drift",
        "detect_shadow_activation_blocker_activation_blocker_activation_side_effect_lock_drift",
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_plans()
-> Vec<
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackPlanPreview,
>{
    vec![
        readback_plan(
            "shadow_activation_blocker_activation_blocker_readback_wrapper_execution_activation",
            "wrapper_execution_activation",
            "runtime_execution",
            vec![
                "shadow_activation_blocker_activation_blocker_promotion_preconditions_unsatisfied",
                "shadow_activation_blocker_activation_blocker_wrapper_execution_disabled",
                "shadow_activation_blocker_activation_blocker_runtime_attachment_disabled",
                "shadow_activation_blocker_activation_blocker_kill_switches_preview_only",
            ],
            vec![
                "shadow_activation_blocker_activation_blocker_promotion_precondition_report",
                "shadow_activation_blocker_activation_blocker_runtime_attachment_plan",
                "shadow_activation_blocker_activation_blocker_side_effect_lock_proof",
            ],
        ),
        readback_plan(
            "shadow_activation_blocker_activation_blocker_readback_readback_execution_activation",
            "readback_execution_activation",
            "readback_execution",
            vec![
                "shadow_activation_blocker_activation_blocker_promotion_preconditions_unsatisfied",
                "shadow_activation_blocker_activation_blocker_readback_execution_disabled",
                "shadow_activation_blocker_activation_blocker_drift_budgets_not_executed",
                "shadow_activation_blocker_activation_blocker_operator_review_missing",
            ],
            vec![
                "shadow_activation_blocker_activation_blocker_promotion_precondition_report",
                "shadow_activation_blocker_activation_blocker_zero_tolerance_drift_budget_report",
                "shadow_activation_blocker_activation_blocker_operator_review_packet",
            ],
        ),
        readback_plan(
            "shadow_activation_blocker_activation_blocker_readback_promotion_execution_activation",
            "promotion_execution_activation",
            "state_promotion",
            vec![
                "shadow_activation_blocker_activation_blocker_promotion_preconditions_unsatisfied",
                "shadow_activation_blocker_activation_blocker_audit_receipts_non_persistent",
                "shadow_activation_blocker_activation_blocker_promotion_execution_disabled",
                "shadow_activation_blocker_activation_blocker_runtime_attachment_disabled",
            ],
            vec![
                "shadow_activation_blocker_activation_blocker_promotion_precondition_report",
                "shadow_activation_blocker_activation_blocker_non_persistent_audit_receipt_readback",
                "shadow_activation_blocker_activation_blocker_runtime_attachment_plan",
            ],
        ),
        readback_plan(
            "shadow_activation_blocker_activation_blocker_readback_task_result_enforcement_activation",
            "task_result_enforcement_activation",
            "contract_enforcement",
            vec![
                "shadow_activation_blocker_activation_blocker_promotion_preconditions_unsatisfied",
                "shadow_activation_blocker_activation_blocker_task_result_enforcement_disabled",
                "shadow_activation_blocker_activation_blocker_drift_budgets_not_executed",
                "shadow_activation_blocker_activation_blocker_side_effect_lock_not_proven",
            ],
            vec![
                "shadow_activation_blocker_activation_blocker_promotion_precondition_report",
                "shadow_activation_blocker_activation_blocker_zero_tolerance_drift_budget_report",
                "shadow_activation_blocker_activation_blocker_side_effect_lock_proof",
            ],
        ),
        readback_plan(
            "shadow_activation_blocker_activation_blocker_readback_store_enablement_activation",
            "store_enablement_activation",
            "state_write",
            vec![
                "shadow_activation_blocker_activation_blocker_promotion_preconditions_unsatisfied",
                "shadow_activation_blocker_activation_blocker_store_enablement_disabled",
                "shadow_activation_blocker_activation_blocker_audit_receipts_non_persistent",
                "shadow_activation_blocker_activation_blocker_persistence_disabled",
            ],
            vec![
                "shadow_activation_blocker_activation_blocker_promotion_precondition_report",
                "shadow_activation_blocker_activation_blocker_non_persistent_audit_receipt_readback",
                "shadow_activation_blocker_activation_blocker_side_effect_lock_proof",
            ],
        ),
        readback_plan(
            "shadow_activation_blocker_activation_blocker_readback_live_execution_activation",
            "live_execution_activation",
            "live_runtime",
            vec![
                "shadow_activation_blocker_activation_blocker_promotion_preconditions_unsatisfied",
                "shadow_activation_blocker_activation_blocker_live_execution_disabled",
                "shadow_activation_blocker_activation_blocker_kill_switches_preview_only",
                "shadow_activation_blocker_activation_blocker_operator_review_missing",
            ],
            vec![
                "shadow_activation_blocker_activation_blocker_promotion_precondition_report",
                "shadow_activation_blocker_activation_blocker_operator_review_packet",
                "shadow_activation_blocker_activation_blocker_side_effect_lock_proof",
            ],
        ),
        readback_plan(
            "shadow_activation_blocker_activation_blocker_readback_external_delivery_activation",
            "external_delivery_activation",
            "external_side_effect",
            vec![
                "shadow_activation_blocker_activation_blocker_promotion_preconditions_unsatisfied",
                "shadow_activation_blocker_activation_blocker_external_delivery_disabled",
                "shadow_activation_blocker_activation_blocker_side_effect_lock_not_proven",
                "shadow_activation_blocker_activation_blocker_operator_review_missing",
            ],
            vec![
                "shadow_activation_blocker_activation_blocker_promotion_precondition_report",
                "shadow_activation_blocker_activation_blocker_operator_review_packet",
                "shadow_activation_blocker_activation_blocker_side_effect_lock_proof",
            ],
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_assertions()
-> Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerAssertionPreview>
{
    vec![
        assertion(
            "assert_shadow_activation_blocker_activation_blocker_surface_stays_blocked",
            "activation_surface",
            vec!["activationSurfaceId", "activationState", "blockedByDefault"],
            vec!["activationStateHash", "blockedSurfaceCount", "riskClass"],
        ),
        assertion(
            "assert_shadow_activation_blocker_activation_blocker_set_matches_preview",
            "blocker_set",
            vec![
                "activationSurfaceId",
                "requiredBlockerIds",
                "actualBlockerIds",
            ],
            vec!["blockerSetHash", "criticalBlockerCount", "denialReasons"],
        ),
        assertion(
            "assert_shadow_activation_blocker_activation_blocker_activation_required_enablements_remain_unsatisfied",
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
            "assert_shadow_activation_blocker_activation_blocker_activation_kill_switches_remain_preview_only",
            "kill_switch",
            vec![
                "killSwitchId",
                "targetSurfaceIds",
                "armedInPreview",
                "persistsSwitchState",
            ],
            vec!["killSwitchHash", "armedPreviewCount", "targetSurfaceCount"],
        ),
        assertion(
            "assert_shadow_activation_blocker_activation_blocker_activation_side_effect_lock_remains_false",
            "side_effects",
            vec![
                "sideEffects",
                "activationPerformed",
                "storePersistenceEnabled",
            ],
            vec!["sideEffectHash", "falseFieldCount", "mutationAttemptCount"],
        ),
        assertion(
            "assert_shadow_activation_blocker_activation_blocker_activation_prior_gate_chain_matches",
            "prior_gate_chain",
            vec!["requiredPriorGates", "lastPriorGate", "recommendedNextGate"],
            vec!["priorGateHash", "priorGateCount", "frontierGate"],
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_detectors()
-> Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftDetectorPreview>
{
    vec![
        detector(
            "detect_shadow_activation_blocker_activation_blocker_surface_state_drift",
            vec!["activationSurfaceId", "activationState", "blockedByDefault"],
        ),
        detector(
            "detect_shadow_activation_blocker_activation_blocker_binding_drift",
            vec![
                "activationSurfaceId",
                "requiredBlockerIds",
                "actualBlockerIds",
            ],
        ),
        detector(
            "detect_shadow_activation_blocker_activation_blocker_activation_enablement_satisfaction_drift",
            vec![
                "enablementId",
                "currentlySatisfied",
                "requiredEvidenceFields",
            ],
        ),
        detector(
            "detect_shadow_activation_blocker_activation_blocker_activation_kill_switch_armament_drift",
            vec!["killSwitchId", "armedInPreview", "persistsSwitchState"],
        ),
        detector(
            "detect_shadow_activation_blocker_activation_blocker_activation_side_effect_lock_drift",
            vec![
                "activationPerformed",
                "taskResultEnforcementEnabled",
                "storePersistenceEnabled",
            ],
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_operator_summaries()
-> Vec<
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerOperatorSummaryPreview,
>{
    vec![
        operator_summary(
            "shadow_activation_blocker_activation_blocker_surface_denial_summary",
            "activation_surfaces",
            vec![
                "blockedSurfaceCount",
                "criticalBlockerCount",
                "readyForActivation",
            ],
        ),
        operator_summary(
            "shadow_activation_blocker_activation_blocker_enablement_gap_summary",
            "required_enablements",
            vec![
                "unsatisfiedEnablementCount",
                "sourceGateIds",
                "requiredEvidenceFields",
            ],
        ),
        operator_summary(
            "shadow_activation_blocker_activation_blocker_kill_switch_summary",
            "kill_switches",
            vec!["killSwitchCount", "armedPreviewOnly", "persistsSwitchState"],
        ),
        operator_summary(
            "shadow_activation_blocker_activation_blocker_side_effect_lock_summary",
            "side_effect_lock",
            vec![
                "activationPerformed",
                "taskResultEnforcementEnabled",
                "storePersistenceEnabled",
            ],
        ),
        operator_summary(
            "shadow_activation_blocker_activation_blocker_external_delivery_denial_summary",
            "external_delivery",
            vec![
                "externalDeliveryDisabled",
                "operatorReviewMissing",
                "sideEffectLockNotProven",
            ],
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_blockers()
-> Vec<
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackBlockerPreview,
>{
    vec![
        blocker(
            "shadow_activation_blocker_activation_blocker_readback_execution_disabled",
            "high",
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_surface_ids(),
            "keep shadow activation blocker activation-blocker readback as contract-only until drift budget preview is reviewed",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_state_persistence_disabled",
            "high",
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_surface_ids(),
            "do not persist shadow activation blocker activation-blocker state before explicit future store enablement",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_operator_review_missing",
            "high",
            vec![
                "live_execution_activation",
                "external_delivery_activation",
                "readback_execution_activation",
            ],
            "attach operator-readable summaries before any shadow activation blocker activation-blocker readback execution",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_drift_budget_missing",
            "critical",
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_surface_ids(),
            "add a zero-tolerance drift budget for shadow activation blocker activation-blocker readback before any activation",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_required_prior_gates()
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
        "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_gate",
        "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_gate",
        "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_gate",
        "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_gate",
    ]
}

impl WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_performed: false,
            shadow_activation_blocker_activation_blocker_activation_blocker_persisted: false,
            drift_detector_persisted: false,
            operator_summary_persisted: false,
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
            model_invoked: false,
        }
    }
}

fn readback_plan(
    id: &'static str,
    activation_surface_id: &'static str,
    risk_class: &'static str,
    expected_blocker_ids: Vec<&'static str>,
    expected_enablement_ids: Vec<&'static str>,
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackPlanPreview
{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackPlanPreview {
        id,
        activation_surface_id,
        risk_class,
        source_gate_id:
            "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_gate",
        expected_activation_state: "blocked_preview_only",
        expected_blocker_ids,
        expected_enablement_ids,
        required_assertion_ids:
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_assertion_ids(),
        drift_detector_ids:
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_detector_ids(),
        readback_state:
            "preview_contract_defined_shadow_activation_blocker_activation_blocker_readback_execution_disabled",
        performs_readback: false,
        performs_shadow_activation_blocker_activation_blocker_activation: false,
        performs_shadow_activation: false,
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
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerAssertionPreview{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerAssertionPreview {
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
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftDetectorPreview
{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftDetectorPreview {
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
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerOperatorSummaryPreview
{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerOperatorSummaryPreview {
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
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackBlockerPreview
{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackBlockerPreview {
        id,
        severity,
        affected_activation_surface_ids,
        required_before_readback_execution: true,
        recommended_fix,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_activation_blocker_activation_blocker_readback_declares_contract_only_plans() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_report();

        assert_eq!(report.readback_plan_count, 7);
        assert!(report.readback_plans.iter().all(|plan| {
            plan.expected_activation_state == "blocked_preview_only"
                && plan.expected_blocker_ids.len() == 4
                && plan.expected_enablement_ids.len() == 3
                && plan.required_assertion_ids.len() == 6
                && plan.drift_detector_ids.len() == 5
                && !plan.performs_readback
                && !plan.performs_shadow_activation_blocker_activation_blocker_activation
                && !plan.performs_shadow_activation
                && !plan.mutates_activation_state
                && !plan.mutates_store
                && !plan.enables_runtime_mutation
        }));
    }

    #[test]
    fn shadow_activation_blocker_activation_blocker_readback_declares_assertions_and_drift_detectors()
     {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_report();

        assert_eq!(report.blocker_assertion_count, 6);
        assert_eq!(report.drift_detector_count, 5);
        assert!(report.blocker_assertions.iter().all(|assertion| {
            assertion.blocks_activation && !assertion.performs_readback && !assertion.mutates_store
        }));
        assert!(
            report
                .drift_detectors
                .iter()
                .all(|detector| detector.severity == "critical" && !detector.persists_drift)
        );
    }

    #[test]
    fn shadow_activation_blocker_activation_blocker_readback_keeps_summaries_redacted_and_non_persistent()
     {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_report();

        assert_eq!(report.operator_summary_count, 5);
        assert_eq!(report.blocker_count, 4);
        assert!(
            report
                .operator_summaries
                .iter()
                .all(|summary| summary.redacted && !summary.persists_summary)
        );
        assert!(
            report
                .blockers
                .iter()
                .all(|blocker| blocker.required_before_readback_execution)
        );
    }

    #[test]
    fn shadow_activation_blocker_activation_blocker_readback_keeps_execution_store_and_live_disabled()
     {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_report();

        assert!(report.ready_for_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview);
        assert!(!report.ready_for_shadow_activation_blocker_activation_blocker_readback_execution);
        assert!(
            !report.ready_for_shadow_activation_blocker_activation_blocker_activation_execution
        );
        assert!(!report.ready_for_shadow_activation_blocker_activation_blocker_promotion_execution);
        assert!(!report.ready_for_shadow_activation_execution);
        assert!(!report.ready_for_activation);
        assert!(!report.ready_for_wrapper_execution);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_store_enablement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackPreviewSideEffects::none()
        );
    }

    #[test]
    fn shadow_activation_blocker_activation_blocker_readback_requires_activation_blocker_prior() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_report();

        assert_eq!(report.required_prior_gate_count, 54);
        assert_eq!(
            report.required_prior_gates.last(),
            Some(
                &"hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_gate"
            )
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_READBACK_RECOMMENDED_NEXT_GATE
        );
    }
}
