use serde::Serialize;

pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_DRIFT_BUDGET_PREVIEW_GATE:
    &str =
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_gate";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_DRIFT_BUDGET_SCHEMA_VERSION:
    &str =
    "work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_v1";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_DRIFT_BUDGET_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub drift_budget_count: usize,
    pub operator_summary_count: usize,
    pub activation_precondition_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub drift_budgets:
        Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetPreview>,
    pub operator_summaries:
        Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftOperatorSummaryPreview>,
    pub activation_preconditions:
        Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftPreconditionPreview>,
    pub blockers:
        Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview: bool,
    pub ready_for_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_execution: bool,
    pub ready_for_shadow_activation_blocker_activation_execution: bool,
    pub ready_for_shadow_activation_blocker_promotion_execution: bool,
    pub ready_for_shadow_activation_execution: bool,
    pub ready_for_activation: bool,
    pub ready_for_shadow_promotion_execution: bool,
    pub ready_for_wrapper_execution: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_store_enablement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetPreview
{
    pub id: &'static str,
    pub drift_detector_id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub max_allowed_mismatches: u32,
    pub max_allowed_unreviewed_findings: u32,
    pub max_replay_lag_ms: u32,
    pub severity: &'static str,
    pub block_level: &'static str,
    pub operator_summary_id: &'static str,
    pub activation_precondition_ids: Vec<&'static str>,
    pub allows_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_execution:
        bool,
    pub allows_shadow_activation_blocker_activation: bool,
    pub allows_shadow_activation_blocker_promotion_execution: bool,
    pub allows_shadow_activation: bool,
    pub allows_activation: bool,
    pub allows_shadow_promotion_execution: bool,
    pub allows_task_result_enforcement: bool,
    pub allows_store_enablement: bool,
    pub allows_live_execution: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftOperatorSummaryPreview
{
    pub id: &'static str,
    pub drift_detector_id: &'static str,
    pub title: &'static str,
    pub required_fields: Vec<&'static str>,
    pub redaction_policy: &'static str,
    pub review_state: &'static str,
    pub persists_summary: bool,
    pub external_delivery_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftPreconditionPreview
{
    pub id: &'static str,
    pub required_budget_ids: Vec<&'static str>,
    pub required_summary_ids: Vec<&'static str>,
    pub required_evidence_fields: Vec<&'static str>,
    pub blocks_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback:
        bool,
    pub blocks_shadow_activation_blocker_activation: bool,
    pub blocks_shadow_activation_blocker_promotion: bool,
    pub blocks_activation: bool,
    pub blocks_shadow_promotion: bool,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetBlockerPreview
{
    pub id: &'static str,
    pub severity: &'static str,
    pub reason: &'static str,
    pub required_before_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_execution:
        bool,
    pub required_before_shadow_activation_blocker_activation: bool,
    pub required_before_activation: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetPreviewSideEffects
{
    pub filesystem_written: bool,
    pub shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_persisted:
        bool,
    pub operator_summary_persisted: bool,
    pub shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_performed:
        bool,
    pub shadow_activation_blocker_activation_blocker_activation_blocker_persisted: bool,
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

pub fn hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_report()
-> WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetPreviewReport{
    let drift_budgets =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets(
        );
    let operator_summaries =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_operator_summaries(
        );
    let activation_preconditions =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_preconditions();
    let blockers =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_blockers(
        );
    let required_prior_gates =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_required_prior_gates();

    WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_DRIFT_BUDGET_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_DRIFT_BUDGET_SCHEMA_VERSION,
        preview_mode:
            "read_only_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_no_execution",
        drift_budget_count: drift_budgets.len(),
        operator_summary_count: operator_summaries.len(),
        activation_precondition_count: activation_preconditions.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        drift_budgets,
        operator_summaries,
        activation_preconditions,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_DRIFT_BUDGET_RECOMMENDED_NEXT_GATE,
        ready_for_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview: true,
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
        side_effects:
            WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetPreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets()
-> Vec<
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetPreview,
>{
    vec![
        drift_budget(
            "shadow_activation_blocker_activation_blocker_surface_state_drift_zero_tolerance_budget",
            "detect_shadow_activation_blocker_activation_blocker_surface_state_drift",
            vec!["activationSurfaceId", "activationState", "blockedByDefault"],
            "shadow_activation_blocker_activation_blocker_surface_state_drift_operator_summary",
            vec![
                "all_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets_zero_tolerance",
                "shadow_activation_blocker_activation_blocker_operator_summaries_reviewed",
            ],
        ),
        drift_budget(
            "shadow_activation_blocker_activation_blocker_binding_drift_zero_tolerance_budget",
            "detect_shadow_activation_blocker_activation_blocker_binding_drift",
            vec![
                "activationSurfaceId",
                "requiredBlockerIds",
                "actualBlockerIds",
            ],
            "shadow_activation_blocker_activation_blocker_binding_drift_operator_summary",
            vec![
                "all_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets_zero_tolerance",
                "shadow_activation_blocker_activation_blocker_operator_summaries_reviewed",
            ],
        ),
        drift_budget(
            "shadow_activation_blocker_activation_enablement_satisfaction_drift_zero_tolerance_budget",
            "detect_shadow_activation_blocker_activation_enablement_satisfaction_drift",
            vec![
                "enablementId",
                "currentlySatisfied",
                "requiredEvidenceFields",
            ],
            "shadow_activation_blocker_activation_enablement_satisfaction_drift_operator_summary",
            vec![
                "all_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets_zero_tolerance",
                "shadow_activation_blocker_activation_blocker_operator_summaries_reviewed",
            ],
        ),
        drift_budget(
            "shadow_activation_blocker_activation_kill_switch_armament_drift_zero_tolerance_budget",
            "detect_shadow_activation_blocker_activation_kill_switch_armament_drift",
            vec!["killSwitchId", "armedInPreview", "persistsSwitchState"],
            "shadow_activation_blocker_activation_kill_switch_armament_drift_operator_summary",
            vec![
                "all_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets_zero_tolerance",
                "shadow_activation_blocker_activation_blocker_operator_summaries_reviewed",
            ],
        ),
        drift_budget(
            "shadow_activation_blocker_activation_side_effect_lock_drift_zero_tolerance_budget",
            "detect_shadow_activation_blocker_activation_side_effect_lock_drift",
            vec![
                "activationPerformed",
                "taskResultEnforcementEnabled",
                "storePersistenceEnabled",
            ],
            "shadow_activation_blocker_activation_side_effect_lock_drift_operator_summary",
            vec![
                "all_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets_zero_tolerance",
                "shadow_activation_blocker_activation_blocker_operator_summaries_reviewed",
                "shadow_activation_blocker_activation_blocker_side_effect_lock_zero_mutation_required",
            ],
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_operator_summaries()
-> Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftOperatorSummaryPreview>{
    vec![
        operator_summary(
            "shadow_activation_blocker_activation_blocker_surface_state_drift_operator_summary",
            "detect_shadow_activation_blocker_activation_blocker_surface_state_drift",
            "Shadow activation blocker activation-blocker surface state drift must remain blocked before activation",
        ),
        operator_summary(
            "shadow_activation_blocker_activation_blocker_binding_drift_operator_summary",
            "detect_shadow_activation_blocker_activation_blocker_binding_drift",
            "Shadow activation blocker activation-blocker binding drift must be reviewed before promotion checks",
        ),
        operator_summary(
            "shadow_activation_blocker_activation_enablement_satisfaction_drift_operator_summary",
            "detect_shadow_activation_blocker_activation_enablement_satisfaction_drift",
            "Shadow activation blocker activation-blocker enablement drift must remain unsatisfied before activation",
        ),
        operator_summary(
            "shadow_activation_blocker_activation_kill_switch_armament_drift_operator_summary",
            "detect_shadow_activation_blocker_activation_kill_switch_armament_drift",
            "Shadow activation blocker activation-blocker kill switch drift must stay preview-only before execution",
        ),
        operator_summary(
            "shadow_activation_blocker_activation_side_effect_lock_drift_operator_summary",
            "detect_shadow_activation_blocker_activation_side_effect_lock_drift",
            "Shadow activation blocker activation-blocker side-effect lock drift must remain zero before any live surface",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_preconditions()
-> Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftPreconditionPreview>{
    vec![
        activation_precondition(
            "all_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets_zero_tolerance",
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_ids(),
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_operator_summary_ids(),
            vec!["maxAllowedMismatches", "maxAllowedUnreviewedFindings"],
        ),
        activation_precondition(
            "shadow_activation_blocker_activation_blocker_operator_summaries_reviewed",
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_ids(),
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_operator_summary_ids(),
            vec!["reviewerIdHash", "reviewedAtUnixMs", "summaryHash"],
        ),
        activation_precondition(
            "shadow_activation_blocker_activation_blocker_side_effect_lock_zero_mutation_required",
            vec!["shadow_activation_blocker_activation_side_effect_lock_drift_zero_tolerance_budget"],
            vec!["shadow_activation_blocker_activation_side_effect_lock_drift_operator_summary"],
            vec![
                "activationPerformed",
                "taskResultEnforcementEnabled",
                "storePersistenceEnabled",
            ],
        ),
        activation_precondition(
            "shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_remains_disabled_until_budget_review",
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_ids(),
            vec![],
            vec![
                "readyForShadowActivationBlockerActivationBlockerReadbackExecution",
                "readyForActivation",
                "readyForStoreEnablement",
            ],
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_blockers()
-> Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetBlockerPreview>{
    vec![
        blocker(
            "shadow_activation_blocker_activation_blocker_drift_budget_not_executed",
            "critical",
            "shadow activation blocker activation-blocker drift detectors have zero-tolerance budgets but no readback execution has run",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_operator_review_not_performed",
            "high",
            "shadow activation blocker activation-blocker operator summaries are preview-only and have not been reviewed or persisted",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_preconditions_not_attached",
            "medium",
            "shadow activation blocker activation-blocker preconditions are defined but not attached to runtime activation or promotion logic",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_drift_persistence_disabled",
            "medium",
            "shadow activation blocker activation-blocker drift state cannot be persisted until store enablement is explicitly approved later",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        crate::work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_required_prior_gates();
    gates.push(
        "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_gate",
    );
    gates
}

impl
    WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetPreviewSideEffects
{
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_persisted: false,
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
            model_invoked: false,
        }
    }
}

fn drift_budget(
    id: &'static str,
    drift_detector_id: &'static str,
    compared_fields: Vec<&'static str>,
    operator_summary_id: &'static str,
    activation_precondition_ids: Vec<&'static str>,
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetPreview
{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetPreview {
        id,
        drift_detector_id,
        compared_fields,
        max_allowed_mismatches: 0,
        max_allowed_unreviewed_findings: 0,
        max_replay_lag_ms: 0,
        severity: "critical",
        block_level: "block_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_activation_promotion_enforcement_store_and_live_execution",
        operator_summary_id,
        activation_precondition_ids,
        allows_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_execution: false,
        allows_shadow_activation_blocker_activation: false,
        allows_shadow_activation_blocker_promotion_execution: false,
        allows_shadow_activation: false,
        allows_activation: false,
        allows_shadow_promotion_execution: false,
        allows_task_result_enforcement: false,
        allows_store_enablement: false,
        allows_live_execution: false,
    }
}

fn operator_summary(
    id: &'static str,
    drift_detector_id: &'static str,
    title: &'static str,
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftOperatorSummaryPreview{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftOperatorSummaryPreview {
        id,
        drift_detector_id,
        title,
        required_fields: vec![
            "detectorId",
            "budgetId",
            "summaryHash",
            "reviewerIdHash",
            "reviewedAtUnixMs",
        ],
        redaction_policy: "summarize ids, hashes, and states without raw activation payload",
        review_state: "preview_summary_defined_review_not_performed",
        persists_summary: false,
        external_delivery_allowed: false,
    }
}

fn activation_precondition(
    id: &'static str,
    required_budget_ids: Vec<&'static str>,
    required_summary_ids: Vec<&'static str>,
    required_evidence_fields: Vec<&'static str>,
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftPreconditionPreview{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftPreconditionPreview {
        id,
        required_budget_ids,
        required_summary_ids,
        required_evidence_fields,
        blocks_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback: true,
        blocks_shadow_activation_blocker_activation: true,
        blocks_shadow_activation_blocker_promotion: true,
        blocks_activation: true,
        blocks_shadow_promotion: true,
        currently_satisfied: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    reason: &'static str,
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetBlockerPreview{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetBlockerPreview {
        id,
        severity,
        reason,
        required_before_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_execution: true,
        required_before_shadow_activation_blocker_activation: true,
        required_before_activation: true,
    }
}

fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_ids()
-> Vec<&'static str> {
    work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets()
        .iter()
        .map(|budget| budget.id)
        .collect()
}

fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_operator_summary_ids()
-> Vec<&'static str> {
    work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_operator_summaries()
        .iter()
        .map(|summary| summary.id)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_activation_blocker_activation_blocker_drift_budget_declares_zero_tolerance_budgets() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_report();

        assert_eq!(report.drift_budget_count, 5);
        assert!(report.drift_budgets.iter().all(|budget| {
            budget.max_allowed_mismatches == 0
                && budget.max_allowed_unreviewed_findings == 0
                && budget.max_replay_lag_ms == 0
                && budget.severity == "critical"
                && !budget.allows_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_execution
                && !budget.allows_shadow_activation_blocker_activation
                && !budget.allows_shadow_activation_blocker_promotion_execution
                && !budget.allows_shadow_activation
                && !budget.allows_activation
                && !budget.allows_shadow_promotion_execution
                && !budget.allows_task_result_enforcement
                && !budget.allows_store_enablement
                && !budget.allows_live_execution
        }));
    }

    #[test]
    fn shadow_activation_blocker_activation_blocker_drift_budget_requires_operator_review() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_report();

        assert_eq!(report.operator_summary_count, 5);
        assert!(report.operator_summaries.iter().all(|summary| {
            summary.review_state == "preview_summary_defined_review_not_performed"
                && !summary.persists_summary
                && !summary.external_delivery_allowed
        }));
        assert!(
            report
                .drift_budgets
                .iter()
                .all(|budget| !budget.operator_summary_id.is_empty())
        );
    }

    #[test]
    fn shadow_activation_blocker_activation_blocker_drift_budget_blocks_activation_until_reviewed()
    {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_report();

        assert_eq!(report.activation_precondition_count, 4);
        assert!(report.activation_preconditions.iter().all(|precondition| {
            precondition
                .blocks_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback
                && precondition.blocks_shadow_activation_blocker_activation
                && precondition.blocks_shadow_activation_blocker_promotion
                && precondition.blocks_activation
                && precondition.blocks_shadow_promotion
                && !precondition.currently_satisfied
        }));
        assert!(
            report.ready_for_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_DRIFT_BUDGET_RECOMMENDED_NEXT_GATE
        );
    }

    #[test]
    fn shadow_activation_blocker_activation_blocker_drift_budget_keeps_execution_and_persistence_disabled()
     {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_report();

        assert_eq!(report.blocker_count, 4);
        assert!(!report.ready_for_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_execution);
        assert!(!report.ready_for_shadow_activation_blocker_activation_execution);
        assert!(!report.ready_for_shadow_activation_blocker_promotion_execution);
        assert!(!report.ready_for_shadow_activation_execution);
        assert!(!report.ready_for_activation);
        assert!(!report.ready_for_shadow_promotion_execution);
        assert!(!report.ready_for_wrapper_execution);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_store_enablement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftBudgetPreviewSideEffects::none()
        );
    }

    #[test]
    fn shadow_activation_blocker_activation_blocker_drift_budget_requires_readback_prior() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_report();

        assert_eq!(report.required_prior_gate_count, 67);
        assert_eq!(
            report.required_prior_gates.last(),
            Some(
                &"hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_gate"
            )
        );
    }
}
