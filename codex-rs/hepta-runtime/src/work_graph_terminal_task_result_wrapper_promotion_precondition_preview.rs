use serde::Serialize;

pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_PROMOTION_PRECONDITION_PREVIEW_GATE: &str =
    "hepta_work_graph_terminal_task_result_wrapper_promotion_precondition_preview_gate";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_PROMOTION_PRECONDITION_SCHEMA_VERSION: &str =
    "work_graph_terminal_task_result_wrapper_promotion_precondition_preview_v1";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_PROMOTION_PRECONDITION_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_terminal_task_result_wrapper_activation_blocker_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperPromotionPreconditionPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub promotion_target_count: usize,
    pub precondition_binding_count: usize,
    pub blocker_count: usize,
    pub audit_receipt_count: usize,
    pub invariant_count: usize,
    pub required_prior_gate_count: usize,
    pub promotion_targets: Vec<WorkGraphTaskResultWrapperPromotionTargetPreview>,
    pub precondition_bindings: Vec<WorkGraphTaskResultWrapperPromotionPreconditionBindingPreview>,
    pub blockers: Vec<WorkGraphTaskResultWrapperPromotionBlockerPreview>,
    pub audit_receipts: Vec<WorkGraphTaskResultWrapperPromotionAuditReceiptPreview>,
    pub invariants: Vec<WorkGraphTaskResultWrapperPromotionPreconditionInvariantPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_activation_blocker_preview: bool,
    pub ready_for_promotion_execution: bool,
    pub ready_for_wrapper_execution: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_store_enablement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphTerminalTaskResultWrapperPromotionPreconditionPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperPromotionTargetPreview {
    pub id: &'static str,
    pub fixture_id: &'static str,
    pub wrapper_id: &'static str,
    pub source_surface_id: &'static str,
    pub target_collection_id: &'static str,
    pub required_precondition_ids: Vec<&'static str>,
    pub required_budget_ids: Vec<&'static str>,
    pub required_operator_summary_ids: Vec<&'static str>,
    pub audit_receipt_id: &'static str,
    pub promotion_state: &'static str,
    pub blocks_task_result_enforcement: bool,
    pub promotes_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperPromotionPreconditionBindingPreview {
    pub id: &'static str,
    pub source_gate_id: &'static str,
    pub required_evidence_fields: Vec<&'static str>,
    pub failure_blocker_id: &'static str,
    pub blocks_promotion: bool,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperPromotionBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub blocks_target_ids: Vec<&'static str>,
    pub operator_message: &'static str,
    pub required_before_promotion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperPromotionAuditReceiptPreview {
    pub id: &'static str,
    pub target_id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub persistence_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperPromotionPreconditionInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperPromotionPreconditionPreviewSideEffects {
    pub filesystem_written: bool,
    pub promotion_state_mutated: bool,
    pub promotion_performed: bool,
    pub wrapper_executed: bool,
    pub readback_performed: bool,
    pub drift_budget_persisted: bool,
    pub audit_receipt_persisted: bool,
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

pub fn hepta_work_graph_terminal_task_result_wrapper_promotion_precondition_preview_report()
-> WorkGraphTerminalTaskResultWrapperPromotionPreconditionPreviewReport {
    let promotion_targets = work_graph_terminal_task_result_wrapper_promotion_targets();
    let precondition_bindings =
        work_graph_terminal_task_result_wrapper_promotion_precondition_bindings();
    let blockers = work_graph_terminal_task_result_wrapper_promotion_blockers();
    let audit_receipts = work_graph_terminal_task_result_wrapper_promotion_audit_receipts();
    let invariants = work_graph_terminal_task_result_wrapper_promotion_precondition_invariants();
    let required_prior_gates =
        work_graph_terminal_task_result_wrapper_promotion_precondition_required_prior_gates();

    WorkGraphTerminalTaskResultWrapperPromotionPreconditionPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_PROMOTION_PRECONDITION_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_PROMOTION_PRECONDITION_SCHEMA_VERSION,
        preview_mode: "read_only_terminal_task_result_wrapper_promotion_precondition_preview_no_promotion",
        promotion_target_count: promotion_targets.len(),
        precondition_binding_count: precondition_bindings.len(),
        blocker_count: blockers.len(),
        audit_receipt_count: audit_receipts.len(),
        invariant_count: invariants.len(),
        required_prior_gate_count: required_prior_gates.len(),
        promotion_targets,
        precondition_bindings,
        blockers,
        audit_receipts,
        invariants,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_PROMOTION_PRECONDITION_RECOMMENDED_NEXT_GATE,
        ready_for_activation_blocker_preview: true,
        ready_for_promotion_execution: false,
        ready_for_wrapper_execution: false,
        ready_for_task_result_enforcement: false,
        ready_for_store_enablement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphTerminalTaskResultWrapperPromotionPreconditionPreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_task_result_wrapper_promotion_targets()
-> Vec<WorkGraphTaskResultWrapperPromotionTargetPreview> {
    vec![
        promotion_target(
            "promote_fixture_multi_agent_thread_spawn_success",
            "fixture_multi_agent_thread_spawn_success",
            "multi_agent_thread_spawn_terminal_task_result_wrapper",
            "multi_agent_v2_thread_spawn",
            "thread_spawn_promotion_audit_receipt",
        ),
        promotion_target(
            "promote_fixture_multi_agent_mailbox_wait_success",
            "fixture_multi_agent_mailbox_wait_success",
            "multi_agent_mailbox_wait_terminal_task_result_wrapper",
            "multi_agent_v2_mailbox_wait",
            "mailbox_wait_promotion_audit_receipt",
        ),
        promotion_target(
            "promote_fixture_multi_agent_reducer_ok",
            "fixture_multi_agent_reducer_ok",
            "multi_agent_reducer_terminal_task_result_wrapper",
            "hepta_runtime_multi_agent_reducer",
            "reducer_promotion_audit_receipt",
        ),
        promotion_target(
            "promote_fixture_agent_job_item_failed",
            "fixture_agent_job_item_failed",
            "agent_job_item_terminal_task_result_wrapper",
            "agent_jobs_batch_workers",
            "agent_job_item_promotion_audit_receipt",
        ),
        promotion_target(
            "promote_fixture_worker_task_blocked",
            "fixture_worker_task_blocked",
            "worker_task_terminal_task_result_wrapper",
            "hepta_runtime_worker_tasks",
            "worker_task_promotion_audit_receipt",
        ),
        promotion_target(
            "promote_fixture_task_board_success",
            "fixture_task_board_success",
            "task_board_terminal_task_result_wrapper",
            "hepta_runtime_task_board",
            "task_board_promotion_audit_receipt",
        ),
        promotion_target(
            "promote_fixture_scheduler_run_superseded",
            "fixture_scheduler_run_superseded",
            "scheduler_run_terminal_task_result_wrapper",
            "hepta_runtime_scheduler_store",
            "scheduler_run_promotion_audit_receipt",
        ),
        promotion_target(
            "promote_fixture_agent_harness_cancelled",
            "fixture_agent_harness_cancelled",
            "agent_harness_terminal_task_result_wrapper",
            "hepta_runtime_agent_harness",
            "agent_harness_promotion_audit_receipt",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_promotion_precondition_bindings()
-> Vec<WorkGraphTaskResultWrapperPromotionPreconditionBindingPreview> {
    vec![
        precondition_binding(
            "all_critical_drift_budgets_zero_tolerance",
            vec!["maxAllowedMismatches", "maxAllowedUnreviewedFindings"],
            "critical_drift_budget_not_executed",
        ),
        precondition_binding(
            "operator_summaries_reviewed",
            vec!["reviewerIdHash", "reviewedAtUnixMs", "summaryHash"],
            "operator_review_not_performed",
        ),
        precondition_binding(
            "redaction_drift_zero_leak_required",
            vec!["redactionState", "summaryHash", "externalDeliveryAllowed"],
            "redaction_precondition_unsatisfied",
        ),
        precondition_binding(
            "execution_remains_disabled_until_budget_review",
            vec![
                "readyForReadbackExecution",
                "readyForWrapperExecution",
                "readyForStoreEnablement",
            ],
            "runtime_promotion_attachment_disabled",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_promotion_blockers()
-> Vec<WorkGraphTaskResultWrapperPromotionBlockerPreview> {
    vec![
        blocker(
            "critical_drift_budget_not_executed",
            "high",
            "zero-tolerance drift budgets are declared but no readback execution has produced findings",
        ),
        blocker(
            "operator_review_not_performed",
            "high",
            "operator summaries are preview-only and have not been reviewed",
        ),
        blocker(
            "redaction_precondition_unsatisfied",
            "critical",
            "redaction drift must remain zero before terminal TaskResult promotion can be considered",
        ),
        blocker(
            "runtime_promotion_attachment_disabled",
            "medium",
            "promotion preconditions are not attached to runtime promotion or enforcement paths",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_promotion_audit_receipts()
-> Vec<WorkGraphTaskResultWrapperPromotionAuditReceiptPreview> {
    work_graph_terminal_task_result_wrapper_promotion_targets()
        .iter()
        .map(
            |target| WorkGraphTaskResultWrapperPromotionAuditReceiptPreview {
                id: target.audit_receipt_id,
                target_id: target.id,
                required_fields: vec![
                    "taskId",
                    "traceId",
                    "wrapperId",
                    "fixtureId",
                    "budgetIds",
                    "operatorSummaryIds",
                    "preconditionIds",
                    "blockerIds",
                    "redactedEvidenceRefs",
                    "receiptHash",
                ],
                persistence_enabled: false,
                external_delivery_enabled: false,
            },
        )
        .collect()
}

pub fn work_graph_terminal_task_result_wrapper_promotion_precondition_invariants()
-> Vec<WorkGraphTaskResultWrapperPromotionPreconditionInvariantPreview> {
    vec![
        invariant(
            "every_wrapper_target_requires_zero_tolerance_drift_budget",
            "each terminal wrapper target must carry all five zero-tolerance drift budgets",
        ),
        invariant(
            "promotion_preconditions_block_without_operator_summaries",
            "operator summaries must be reviewed before any future promotion path can proceed",
        ),
        invariant(
            "redaction_drift_blocks_all_live_surfaces",
            "redaction drift blocks terminal TaskResult enforcement and every live surface",
        ),
        invariant(
            "audit_receipts_are_preview_only",
            "promotion audit receipts carry hashes and refs but cannot be persisted or delivered",
        ),
        invariant(
            "promotion_precondition_preview_has_no_side_effects",
            "this preview cannot promote, execute wrappers, enforce TaskResult, write state, or send externally",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_promotion_precondition_required_prior_gates()
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
    ]
}

impl WorkGraphTerminalTaskResultWrapperPromotionPreconditionPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
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
            model_invoked: false,
        }
    }
}

fn promotion_target(
    id: &'static str,
    fixture_id: &'static str,
    wrapper_id: &'static str,
    source_surface_id: &'static str,
    audit_receipt_id: &'static str,
) -> WorkGraphTaskResultWrapperPromotionTargetPreview {
    WorkGraphTaskResultWrapperPromotionTargetPreview {
        id,
        fixture_id,
        wrapper_id,
        source_surface_id,
        target_collection_id: "taskResults",
        required_precondition_ids:
            work_graph_terminal_task_result_wrapper_promotion_precondition_ids(),
        required_budget_ids: work_graph_terminal_task_result_wrapper_drift_budget_ids(),
        required_operator_summary_ids: work_graph_terminal_task_result_wrapper_operator_summary_ids(
        ),
        audit_receipt_id,
        promotion_state: "blocked_preview_only",
        blocks_task_result_enforcement: true,
        promotes_state: false,
    }
}

fn precondition_binding(
    id: &'static str,
    required_evidence_fields: Vec<&'static str>,
    failure_blocker_id: &'static str,
) -> WorkGraphTaskResultWrapperPromotionPreconditionBindingPreview {
    WorkGraphTaskResultWrapperPromotionPreconditionBindingPreview {
        id,
        source_gate_id: "hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_gate",
        required_evidence_fields,
        failure_blocker_id,
        blocks_promotion: true,
        currently_satisfied: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    operator_message: &'static str,
) -> WorkGraphTaskResultWrapperPromotionBlockerPreview {
    WorkGraphTaskResultWrapperPromotionBlockerPreview {
        id,
        severity,
        blocks_target_ids: work_graph_terminal_task_result_wrapper_promotion_targets()
            .iter()
            .map(|target| target.id)
            .collect(),
        operator_message,
        required_before_promotion: true,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphTaskResultWrapperPromotionPreconditionInvariantPreview {
    WorkGraphTaskResultWrapperPromotionPreconditionInvariantPreview {
        id,
        required: true,
        reason,
    }
}

fn work_graph_terminal_task_result_wrapper_promotion_precondition_ids() -> Vec<&'static str> {
    vec![
        "all_critical_drift_budgets_zero_tolerance",
        "operator_summaries_reviewed",
        "redaction_drift_zero_leak_required",
        "execution_remains_disabled_until_budget_review",
    ]
}

fn work_graph_terminal_task_result_wrapper_drift_budget_ids() -> Vec<&'static str> {
    vec![
        "identity_drift_zero_tolerance_budget",
        "status_drift_zero_tolerance_budget",
        "evidence_drift_zero_tolerance_budget",
        "verifier_drift_zero_tolerance_budget",
        "redaction_drift_zero_tolerance_budget",
    ]
}

fn work_graph_terminal_task_result_wrapper_operator_summary_ids() -> Vec<&'static str> {
    vec![
        "identity_drift_operator_summary",
        "status_drift_operator_summary",
        "evidence_drift_operator_summary",
        "verifier_drift_operator_summary",
        "redaction_drift_operator_summary",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapper_promotion_precondition_declares_all_terminal_targets() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_promotion_precondition_preview_report();

        assert_eq!(report.promotion_target_count, 8);
        assert!(report.promotion_targets.iter().all(|target| {
            target.target_collection_id == "taskResults"
                && target.required_precondition_ids.len() == 4
                && target.required_budget_ids.len() == 5
                && target.required_operator_summary_ids.len() == 5
                && target.promotion_state == "blocked_preview_only"
                && target.blocks_task_result_enforcement
                && !target.promotes_state
        }));
    }

    #[test]
    fn wrapper_promotion_precondition_binds_drift_budget_preconditions() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_promotion_precondition_preview_report();

        assert_eq!(report.precondition_binding_count, 4);
        assert!(report.precondition_bindings.iter().all(|binding| {
            binding.source_gate_id
                == "hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_gate"
                && binding.blocks_promotion
                && !binding.currently_satisfied
        }));
    }

    #[test]
    fn wrapper_promotion_precondition_declares_blockers_and_receipts() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_promotion_precondition_preview_report();

        assert_eq!(report.blocker_count, 4);
        assert_eq!(report.audit_receipt_count, 8);
        assert!(report.blockers.iter().all(|blocker| {
            blocker.blocks_target_ids.len() == 8 && blocker.required_before_promotion
        }));
        assert!(report.audit_receipts.iter().all(|receipt| {
            receipt.required_fields.contains(&"receiptHash")
                && receipt.required_fields.contains(&"redactedEvidenceRefs")
                && !receipt.persistence_enabled
                && !receipt.external_delivery_enabled
        }));
    }

    #[test]
    fn wrapper_promotion_precondition_keeps_execution_and_persistence_disabled() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_promotion_precondition_preview_report();

        assert!(report.ready_for_activation_blocker_preview);
        assert!(!report.ready_for_promotion_execution);
        assert!(!report.ready_for_wrapper_execution);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_store_enablement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphTerminalTaskResultWrapperPromotionPreconditionPreviewSideEffects::none()
        );
    }

    #[test]
    fn wrapper_promotion_precondition_requires_drift_budget_prior() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_promotion_precondition_preview_report();

        assert_eq!(report.required_prior_gate_count, 16);
        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_gate")
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_PROMOTION_PRECONDITION_RECOMMENDED_NEXT_GATE
        );
    }
}
