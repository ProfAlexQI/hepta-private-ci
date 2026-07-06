use serde::Serialize;

pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_DRIFT_BUDGET_PREVIEW_GATE: &str =
    "hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_gate";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_DRIFT_BUDGET_SCHEMA_VERSION: &str =
    "work_graph_terminal_task_result_wrapper_drift_budget_preview_v1";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_DRIFT_BUDGET_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_terminal_task_result_wrapper_promotion_precondition_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperDriftBudgetPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub drift_budget_count: usize,
    pub operator_summary_count: usize,
    pub promotion_precondition_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub drift_budgets: Vec<WorkGraphTaskResultWrapperDriftBudgetPreview>,
    pub operator_summaries: Vec<WorkGraphTaskResultWrapperDriftOperatorSummaryPreview>,
    pub promotion_preconditions: Vec<WorkGraphTaskResultWrapperDriftPromotionPreconditionPreview>,
    pub blockers: Vec<WorkGraphTaskResultWrapperDriftBudgetBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_promotion_precondition_preview: bool,
    pub ready_for_readback_execution: bool,
    pub ready_for_wrapper_execution: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_store_enablement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphTerminalTaskResultWrapperDriftBudgetPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperDriftBudgetPreview {
    pub id: &'static str,
    pub drift_detector_id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub max_allowed_mismatches: u32,
    pub max_allowed_unreviewed_findings: u32,
    pub max_replay_lag_ms: u32,
    pub severity: &'static str,
    pub block_level: &'static str,
    pub human_summary_id: &'static str,
    pub promotion_precondition_ids: Vec<&'static str>,
    pub allows_readback_execution: bool,
    pub allows_wrapper_execution: bool,
    pub allows_task_result_enforcement: bool,
    pub allows_store_promotion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperDriftOperatorSummaryPreview {
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
pub struct WorkGraphTaskResultWrapperDriftPromotionPreconditionPreview {
    pub id: &'static str,
    pub required_budget_ids: Vec<&'static str>,
    pub required_summary_ids: Vec<&'static str>,
    pub required_evidence_fields: Vec<&'static str>,
    pub blocks_promotion: bool,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperDriftBudgetBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub reason: &'static str,
    pub required_before_readback_execution: bool,
    pub required_before_promotion: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperDriftBudgetPreviewSideEffects {
    pub filesystem_written: bool,
    pub drift_budget_persisted: bool,
    pub operator_summary_persisted: bool,
    pub promotion_state_mutated: bool,
    pub readback_performed: bool,
    pub wrapper_executed: bool,
    pub event_record_persisted: bool,
    pub task_result_persisted: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub task_result_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub replay_executed: bool,
    pub approval_recorded: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_report()
-> WorkGraphTerminalTaskResultWrapperDriftBudgetPreviewReport {
    let drift_budgets = work_graph_terminal_task_result_wrapper_drift_budgets();
    let operator_summaries = work_graph_terminal_task_result_wrapper_drift_operator_summaries();
    let promotion_preconditions =
        work_graph_terminal_task_result_wrapper_drift_promotion_preconditions();
    let blockers = work_graph_terminal_task_result_wrapper_drift_budget_blockers();
    let required_prior_gates =
        work_graph_terminal_task_result_wrapper_drift_budget_required_prior_gates();

    WorkGraphTerminalTaskResultWrapperDriftBudgetPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_DRIFT_BUDGET_PREVIEW_GATE,
        schema_version: WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_DRIFT_BUDGET_SCHEMA_VERSION,
        preview_mode: "read_only_terminal_task_result_wrapper_drift_budget_preview_no_execution",
        drift_budget_count: drift_budgets.len(),
        operator_summary_count: operator_summaries.len(),
        promotion_precondition_count: promotion_preconditions.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        drift_budgets,
        operator_summaries,
        promotion_preconditions,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_DRIFT_BUDGET_RECOMMENDED_NEXT_GATE,
        ready_for_promotion_precondition_preview: true,
        ready_for_readback_execution: false,
        ready_for_wrapper_execution: false,
        ready_for_task_result_enforcement: false,
        ready_for_store_enablement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphTerminalTaskResultWrapperDriftBudgetPreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_task_result_wrapper_drift_budgets()
-> Vec<WorkGraphTaskResultWrapperDriftBudgetPreview> {
    vec![
        drift_budget(
            "identity_drift_zero_tolerance_budget",
            "detect_fixture_identity_drift",
            vec!["taskId", "traceId", "wrapperId"],
            "identity_drift_operator_summary",
            vec![
                "all_critical_drift_budgets_zero_tolerance",
                "operator_summaries_reviewed",
            ],
        ),
        drift_budget(
            "status_drift_zero_tolerance_budget",
            "detect_fixture_status_drift",
            vec!["status", "terminalStatusObserved"],
            "status_drift_operator_summary",
            vec![
                "all_critical_drift_budgets_zero_tolerance",
                "operator_summaries_reviewed",
            ],
        ),
        drift_budget(
            "evidence_drift_zero_tolerance_budget",
            "detect_fixture_evidence_drift",
            vec!["evidenceHash", "evidenceRefs"],
            "evidence_drift_operator_summary",
            vec![
                "all_critical_drift_budgets_zero_tolerance",
                "operator_summaries_reviewed",
            ],
        ),
        drift_budget(
            "verifier_drift_zero_tolerance_budget",
            "detect_fixture_verifier_drift",
            vec!["verifierRef", "gateReportHash"],
            "verifier_drift_operator_summary",
            vec![
                "all_critical_drift_budgets_zero_tolerance",
                "operator_summaries_reviewed",
            ],
        ),
        drift_budget(
            "redaction_drift_zero_tolerance_budget",
            "detect_fixture_redaction_drift",
            vec!["summaryHash", "redactionState"],
            "redaction_drift_operator_summary",
            vec![
                "all_critical_drift_budgets_zero_tolerance",
                "operator_summaries_reviewed",
                "redaction_drift_zero_leak_required",
            ],
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_drift_operator_summaries()
-> Vec<WorkGraphTaskResultWrapperDriftOperatorSummaryPreview> {
    vec![
        operator_summary(
            "identity_drift_operator_summary",
            "detect_fixture_identity_drift",
            "Task identity drift must be reviewed before wrapper execution",
        ),
        operator_summary(
            "status_drift_operator_summary",
            "detect_fixture_status_drift",
            "Terminal status drift must be reviewed before TaskResult enforcement",
        ),
        operator_summary(
            "evidence_drift_operator_summary",
            "detect_fixture_evidence_drift",
            "Evidence reference drift must be reviewed before readback execution",
        ),
        operator_summary(
            "verifier_drift_operator_summary",
            "detect_fixture_verifier_drift",
            "Verifier reference drift must be reviewed before promotion checks",
        ),
        operator_summary(
            "redaction_drift_operator_summary",
            "detect_fixture_redaction_drift",
            "Redaction drift must remain zero before any live surface",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_drift_promotion_preconditions()
-> Vec<WorkGraphTaskResultWrapperDriftPromotionPreconditionPreview> {
    vec![
        promotion_precondition(
            "all_critical_drift_budgets_zero_tolerance",
            work_graph_terminal_task_result_wrapper_drift_budget_ids(),
            work_graph_terminal_task_result_wrapper_drift_operator_summary_ids(),
            vec!["maxAllowedMismatches", "maxAllowedUnreviewedFindings"],
        ),
        promotion_precondition(
            "operator_summaries_reviewed",
            work_graph_terminal_task_result_wrapper_drift_budget_ids(),
            work_graph_terminal_task_result_wrapper_drift_operator_summary_ids(),
            vec!["reviewerIdHash", "reviewedAtUnixMs", "summaryHash"],
        ),
        promotion_precondition(
            "redaction_drift_zero_leak_required",
            vec!["redaction_drift_zero_tolerance_budget"],
            vec!["redaction_drift_operator_summary"],
            vec!["redactionState", "summaryHash", "externalDeliveryAllowed"],
        ),
        promotion_precondition(
            "execution_remains_disabled_until_budget_review",
            work_graph_terminal_task_result_wrapper_drift_budget_ids(),
            vec![],
            vec![
                "readyForReadbackExecution",
                "readyForWrapperExecution",
                "readyForStoreEnablement",
            ],
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_drift_budget_blockers()
-> Vec<WorkGraphTaskResultWrapperDriftBudgetBlockerPreview> {
    vec![
        blocker(
            "critical_drift_budget_not_executed",
            "high",
            "critical drift detectors have a zero-tolerance budget but no readback execution has run",
        ),
        blocker(
            "operator_review_not_performed",
            "high",
            "operator summaries are preview-only and have not been reviewed or persisted",
        ),
        blocker(
            "promotion_preconditions_not_attached",
            "medium",
            "promotion precondition wiring is defined but not attached to runtime promotion logic",
        ),
        blocker(
            "drift_persistence_disabled",
            "medium",
            "drift state cannot be persisted until readback execution is explicitly enabled later",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_drift_budget_required_prior_gates()
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
    ]
}

impl WorkGraphTerminalTaskResultWrapperDriftBudgetPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            drift_budget_persisted: false,
            operator_summary_persisted: false,
            promotion_state_mutated: false,
            readback_performed: false,
            wrapper_executed: false,
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
            model_invoked: false,
        }
    }
}

fn drift_budget(
    id: &'static str,
    drift_detector_id: &'static str,
    compared_fields: Vec<&'static str>,
    human_summary_id: &'static str,
    promotion_precondition_ids: Vec<&'static str>,
) -> WorkGraphTaskResultWrapperDriftBudgetPreview {
    WorkGraphTaskResultWrapperDriftBudgetPreview {
        id,
        drift_detector_id,
        compared_fields,
        max_allowed_mismatches: 0,
        max_allowed_unreviewed_findings: 0,
        max_replay_lag_ms: 0,
        severity: "critical",
        block_level: "block_wrapper_execution_task_result_enforcement_and_promotion",
        human_summary_id,
        promotion_precondition_ids,
        allows_readback_execution: false,
        allows_wrapper_execution: false,
        allows_task_result_enforcement: false,
        allows_store_promotion: false,
    }
}

fn operator_summary(
    id: &'static str,
    drift_detector_id: &'static str,
    title: &'static str,
) -> WorkGraphTaskResultWrapperDriftOperatorSummaryPreview {
    WorkGraphTaskResultWrapperDriftOperatorSummaryPreview {
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
        redaction_policy: "summarize ids, hashes, and states without raw terminal payload",
        review_state: "preview_summary_defined_review_not_performed",
        persists_summary: false,
        external_delivery_allowed: false,
    }
}

fn promotion_precondition(
    id: &'static str,
    required_budget_ids: Vec<&'static str>,
    required_summary_ids: Vec<&'static str>,
    required_evidence_fields: Vec<&'static str>,
) -> WorkGraphTaskResultWrapperDriftPromotionPreconditionPreview {
    WorkGraphTaskResultWrapperDriftPromotionPreconditionPreview {
        id,
        required_budget_ids,
        required_summary_ids,
        required_evidence_fields,
        blocks_promotion: true,
        currently_satisfied: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    reason: &'static str,
) -> WorkGraphTaskResultWrapperDriftBudgetBlockerPreview {
    WorkGraphTaskResultWrapperDriftBudgetBlockerPreview {
        id,
        severity,
        reason,
        required_before_readback_execution: true,
        required_before_promotion: true,
    }
}

fn work_graph_terminal_task_result_wrapper_drift_budget_ids() -> Vec<&'static str> {
    work_graph_terminal_task_result_wrapper_drift_budgets()
        .iter()
        .map(|budget| budget.id)
        .collect()
}

fn work_graph_terminal_task_result_wrapper_drift_operator_summary_ids() -> Vec<&'static str> {
    work_graph_terminal_task_result_wrapper_drift_operator_summaries()
        .iter()
        .map(|summary| summary.id)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapper_drift_budget_declares_zero_tolerance_budgets() {
        let report = hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_report();

        assert_eq!(report.drift_budget_count, 5);
        assert!(report.drift_budgets.iter().all(|budget| {
            budget.max_allowed_mismatches == 0
                && budget.max_allowed_unreviewed_findings == 0
                && budget.max_replay_lag_ms == 0
                && budget.severity == "critical"
                && !budget.allows_readback_execution
                && !budget.allows_wrapper_execution
                && !budget.allows_task_result_enforcement
                && !budget.allows_store_promotion
        }));
    }

    #[test]
    fn wrapper_drift_budget_requires_operator_summaries() {
        let report = hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_report();

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
                .all(|budget| !budget.human_summary_id.is_empty())
        );
    }

    #[test]
    fn wrapper_drift_budget_blocks_promotion_until_preconditions_are_reviewed() {
        let report = hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_report();

        assert_eq!(report.promotion_precondition_count, 4);
        assert!(report.promotion_preconditions.iter().all(|precondition| {
            precondition.blocks_promotion && !precondition.currently_satisfied
        }));
        assert!(report.ready_for_promotion_precondition_preview);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_DRIFT_BUDGET_RECOMMENDED_NEXT_GATE
        );
    }

    #[test]
    fn wrapper_drift_budget_keeps_execution_and_persistence_disabled() {
        let report = hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_report();

        assert_eq!(report.blocker_count, 4);
        assert!(!report.ready_for_readback_execution);
        assert!(!report.ready_for_wrapper_execution);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_store_enablement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphTerminalTaskResultWrapperDriftBudgetPreviewSideEffects::none()
        );
    }

    #[test]
    fn wrapper_drift_budget_requires_readback_prior() {
        let report = hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_report();

        assert_eq!(report.required_prior_gate_count, 15);
        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_terminal_task_result_wrapper_readback_preview_gate")
        );
    }
}
