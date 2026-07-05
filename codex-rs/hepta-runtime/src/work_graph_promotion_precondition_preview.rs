use serde::Serialize;

pub const WORK_GRAPH_PROMOTION_PRECONDITION_PREVIEW_GATE: &str =
    "hepta_work_graph_promotion_precondition_preview_gate";
pub const WORK_GRAPH_PROMOTION_PRECONDITION_SCHEMA_VERSION: &str =
    "work_graph_promotion_precondition_preview_v1";
pub const WORK_GRAPH_PROMOTION_PRECONDITION_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_activation_enforcement_blocker_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPromotionPreconditionPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub promotion_target_count: usize,
    pub required_check_count: usize,
    pub denial_reason_count: usize,
    pub audit_receipt_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub promotion_targets: Vec<WorkGraphPromotionTargetPreview>,
    pub required_checks: Vec<WorkGraphPromotionCheckPreview>,
    pub denial_reasons: Vec<WorkGraphPromotionDenialPreview>,
    pub audit_receipts: Vec<WorkGraphPromotionAuditReceiptPreview>,
    pub invariants: Vec<WorkGraphPromotionPreconditionInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_activation_enforcement_blocker_preview: bool,
    pub ready_for_promotion_execution: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphPromotionPreconditionPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPromotionTargetPreview {
    pub id: &'static str,
    pub node_kind: &'static str,
    pub target_status: &'static str,
    pub required_check_ids: Vec<&'static str>,
    pub blocked_without_readback: bool,
    pub promotes_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPromotionCheckPreview {
    pub id: &'static str,
    pub applies_to_target_ids: Vec<&'static str>,
    pub required_evidence_fields: Vec<&'static str>,
    pub failure_denial_id: &'static str,
    pub required_before_promotion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPromotionDenialPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub blocks_target_ids: Vec<&'static str>,
    pub operator_message: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPromotionAuditReceiptPreview {
    pub id: &'static str,
    pub target_id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub persistence_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPromotionPreconditionInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPromotionPreconditionPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub promotion_performed: bool,
    pub scheduler_unblock_performed: bool,
    pub handoff_promotion_performed: bool,
    pub approval_recorded: bool,
    pub audit_receipt_persisted: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_cutover_performed: bool,
    pub adapter_projection_enforced: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_promotion_precondition_preview_report()
-> WorkGraphPromotionPreconditionPreviewReport {
    let promotion_targets = work_graph_promotion_precondition_targets();
    let required_checks = work_graph_promotion_precondition_checks();
    let denial_reasons = work_graph_promotion_precondition_denials();
    let audit_receipts = work_graph_promotion_precondition_audit_receipts();
    let invariants = work_graph_promotion_precondition_invariants();

    WorkGraphPromotionPreconditionPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PROMOTION_PRECONDITION_PREVIEW_GATE,
        schema_version: WORK_GRAPH_PROMOTION_PRECONDITION_SCHEMA_VERSION,
        preview_mode: "read_only_promotion_precondition_preview_no_promotion",
        promotion_target_count: promotion_targets.len(),
        required_check_count: required_checks.len(),
        denial_reason_count: denial_reasons.len(),
        audit_receipt_count: audit_receipts.len(),
        invariant_count: invariants.len(),
        required_prior_gates: work_graph_promotion_precondition_required_prior_gates(),
        promotion_targets,
        required_checks,
        denial_reasons,
        audit_receipts,
        invariants,
        recommended_next_gate: WORK_GRAPH_PROMOTION_PRECONDITION_RECOMMENDED_NEXT_GATE,
        ready_for_activation_enforcement_blocker_preview: true,
        ready_for_promotion_execution: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphPromotionPreconditionPreviewSideEffects::none(),
    }
}

pub fn work_graph_promotion_precondition_required_prior_gates() -> Vec<&'static str> {
    vec![
        "hepta_work_graph_contract_preview_gate",
        "hepta_work_graph_task_result_contract_preview_gate",
        "hepta_work_graph_scheduler_admission_controller_preview_gate",
        "hepta_work_graph_observability_timeline_preview_gate",
        "hepta_work_graph_role_manifest_contract_preview_gate",
        "hepta_work_graph_unified_state_store_preview_gate",
        "hepta_work_graph_adapter_projection_fixture_gate",
        "hepta_work_graph_state_store_persistence_preview_gate",
        "hepta_work_graph_replay_readback_preview_gate",
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_promotion_precondition_target_ids() -> Vec<&'static str> {
    vec![
        "terminal_task_result_promotion",
        "scheduler_unblock_promotion",
        "artifact_handoff_promotion",
        "external_handoff_promotion",
        "approval_resolution_promotion",
        "timeline_operator_summary_promotion",
    ]
}

pub fn work_graph_promotion_precondition_targets() -> Vec<WorkGraphPromotionTargetPreview> {
    vec![
        promotion_target(
            "terminal_task_result_promotion",
            "worker_task",
            "terminal_result_visible",
            vec![
                "durable_identity_evidence_ready",
                "task_result_schema_valid",
                "task_result_readback_clean",
                "no_replay_drift_detected",
                "artifact_redaction_verified",
                "operator_audit_receipt_ready",
            ],
        ),
        promotion_target(
            "scheduler_unblock_promotion",
            "scheduler_run",
            "runnable",
            vec![
                "durable_identity_evidence_ready",
                "dependency_closure_satisfied",
                "approval_readback_clean",
                "lease_and_budget_current",
                "no_replay_drift_detected",
                "operator_audit_receipt_ready",
            ],
        ),
        promotion_target(
            "artifact_handoff_promotion",
            "artifact",
            "handoff_ready",
            vec![
                "durable_identity_evidence_ready",
                "artifact_readback_clean",
                "artifact_redaction_verified",
                "handoff_scope_authorized",
                "no_external_delivery_enabled",
                "operator_audit_receipt_ready",
            ],
        ),
        promotion_target(
            "external_handoff_promotion",
            "external_handoff",
            "blocked_pending_operator",
            vec![
                "durable_identity_evidence_ready",
                "handoff_scope_authorized",
                "approval_readback_clean",
                "no_external_delivery_enabled",
                "operator_audit_receipt_ready",
            ],
        ),
        promotion_target(
            "approval_resolution_promotion",
            "human_approval",
            "approval_visible_not_recorded",
            vec![
                "durable_identity_evidence_ready",
                "approval_readback_clean",
                "operator_authority_scope_valid",
                "approval_expiry_current",
                "operator_audit_receipt_ready",
            ],
        ),
        promotion_target(
            "timeline_operator_summary_promotion",
            "timeline_event",
            "operator_summary_ready",
            vec![
                "durable_identity_evidence_ready",
                "timeline_readback_clean",
                "artifact_redaction_verified",
                "operator_audit_receipt_ready",
            ],
        ),
    ]
}

pub fn work_graph_promotion_precondition_checks() -> Vec<WorkGraphPromotionCheckPreview> {
    vec![
        check(
            "durable_identity_evidence_ready",
            work_graph_promotion_precondition_target_ids(),
            vec![
                "workflow_id",
                "run_id",
                "step_id",
                "checkpoint",
                "replay_key",
                "rollback_anchor",
                "receipt_hash",
            ],
            "deny_durable_identity_evidence_missing",
        ),
        check(
            "task_result_schema_valid",
            vec!["terminal_task_result_promotion"],
            vec!["taskId", "status", "summaryHash", "evidenceRefs", "traceId"],
            "deny_task_result_schema_missing",
        ),
        check(
            "task_result_readback_clean",
            vec!["terminal_task_result_promotion"],
            vec![
                "taskResultHash",
                "terminalStatusObserved",
                "validatedWalHeadHash",
            ],
            "deny_task_result_readback_missing",
        ),
        check(
            "artifact_readback_clean",
            vec!["artifact_handoff_promotion"],
            vec!["artifactHash", "producerNodeId", "redactionState"],
            "deny_artifact_readback_missing",
        ),
        check(
            "approval_readback_clean",
            vec![
                "scheduler_unblock_promotion",
                "external_handoff_promotion",
                "approval_resolution_promotion",
            ],
            vec!["approvalId", "operatorScopeHash", "expiresAtUnixMs"],
            "deny_approval_readback_missing",
        ),
        check(
            "timeline_readback_clean",
            vec!["timeline_operator_summary_promotion"],
            vec!["timelineHash", "eventCount", "redactionState"],
            "deny_timeline_readback_missing",
        ),
        check(
            "dependency_closure_satisfied",
            vec!["scheduler_unblock_promotion"],
            vec![
                "dependencyNodeIds",
                "blockingEdgeIds",
                "terminalStatusObserved",
            ],
            "deny_dependency_closure_unsatisfied",
        ),
        check(
            "lease_and_budget_current",
            vec!["scheduler_unblock_promotion"],
            vec!["leaseId", "budgetState", "checkedAtUnixMs"],
            "deny_lease_or_budget_stale",
        ),
        check(
            "operator_authority_scope_valid",
            vec!["approval_resolution_promotion"],
            vec!["operatorScope", "authorityHash", "expiresAtUnixMs"],
            "deny_operator_authority_invalid",
        ),
        check(
            "approval_expiry_current",
            vec!["approval_resolution_promotion"],
            vec!["approvalId", "expiresAtUnixMs", "checkedAtUnixMs"],
            "deny_approval_expired",
        ),
        check(
            "handoff_scope_authorized",
            vec!["artifact_handoff_promotion", "external_handoff_promotion"],
            vec!["handoffId", "target", "operatorScopeHash"],
            "deny_handoff_scope_unauthorized",
        ),
        check(
            "no_external_delivery_enabled",
            vec!["artifact_handoff_promotion", "external_handoff_promotion"],
            vec!["deliveryEnabled", "externalSendEnabled", "previewMode"],
            "deny_external_delivery_enabled",
        ),
        check(
            "artifact_redaction_verified",
            vec![
                "terminal_task_result_promotion",
                "artifact_handoff_promotion",
                "timeline_operator_summary_promotion",
            ],
            vec!["redactionState", "payloadHash", "evidenceRefs"],
            "deny_redaction_unverified",
        ),
        check(
            "no_replay_drift_detected",
            vec![
                "terminal_task_result_promotion",
                "scheduler_unblock_promotion",
            ],
            vec!["driftDetectorIds", "driftStatus", "validatedWalHeadHash"],
            "deny_replay_drift_detected",
        ),
        check(
            "operator_audit_receipt_ready",
            vec![
                "terminal_task_result_promotion",
                "scheduler_unblock_promotion",
                "artifact_handoff_promotion",
                "external_handoff_promotion",
                "approval_resolution_promotion",
                "timeline_operator_summary_promotion",
            ],
            vec!["traceId", "nodeId", "redactedEvidenceRefs", "receiptHash"],
            "deny_audit_receipt_missing",
        ),
    ]
}

pub fn work_graph_promotion_precondition_denials() -> Vec<WorkGraphPromotionDenialPreview> {
    vec![
        denial(
            "deny_durable_identity_evidence_missing",
            "critical",
            work_graph_promotion_precondition_target_ids(),
            "promotion requires workflow, run, step, checkpoint, replay, rollback, and receipt hash evidence",
        ),
        denial(
            "deny_task_result_schema_missing",
            "critical",
            vec!["terminal_task_result_promotion"],
            "TaskResult cannot be promoted without canonical schema evidence",
        ),
        denial(
            "deny_task_result_readback_missing",
            "critical",
            vec!["terminal_task_result_promotion"],
            "terminal TaskResult readback must match replay evidence first",
        ),
        denial(
            "deny_artifact_readback_missing",
            "critical",
            vec!["artifact_handoff_promotion"],
            "artifact handoff cannot proceed without artifact readback",
        ),
        denial(
            "deny_approval_readback_missing",
            "critical",
            vec![
                "scheduler_unblock_promotion",
                "external_handoff_promotion",
                "approval_resolution_promotion",
            ],
            "approval-dependent promotion requires visible approval readback",
        ),
        denial(
            "deny_timeline_readback_missing",
            "high",
            vec!["timeline_operator_summary_promotion"],
            "operator summary requires replayable timeline readback",
        ),
        denial(
            "deny_dependency_closure_unsatisfied",
            "critical",
            vec!["scheduler_unblock_promotion"],
            "scheduler unblock requires every dependency edge to be satisfied",
        ),
        denial(
            "deny_lease_or_budget_stale",
            "critical",
            vec!["scheduler_unblock_promotion"],
            "scheduler unblock requires current lease and budget evidence",
        ),
        denial(
            "deny_operator_authority_invalid",
            "critical",
            vec!["approval_resolution_promotion"],
            "approval resolution requires valid operator authority scope",
        ),
        denial(
            "deny_approval_expired",
            "critical",
            vec!["approval_resolution_promotion"],
            "expired approval cannot unblock or promote work",
        ),
        denial(
            "deny_handoff_scope_unauthorized",
            "critical",
            vec!["artifact_handoff_promotion", "external_handoff_promotion"],
            "handoff target is outside authorized scope",
        ),
        denial(
            "deny_external_delivery_enabled",
            "critical",
            vec!["artifact_handoff_promotion", "external_handoff_promotion"],
            "preview promotion cannot enable external delivery",
        ),
        denial(
            "deny_redaction_unverified",
            "critical",
            vec![
                "terminal_task_result_promotion",
                "artifact_handoff_promotion",
                "timeline_operator_summary_promotion",
            ],
            "promotion evidence must be redacted before it can be surfaced",
        ),
        denial(
            "deny_replay_drift_detected",
            "critical",
            vec![
                "terminal_task_result_promotion",
                "scheduler_unblock_promotion",
            ],
            "replay/readback drift blocks promotion",
        ),
        denial(
            "deny_audit_receipt_missing",
            "critical",
            vec![
                "terminal_task_result_promotion",
                "scheduler_unblock_promotion",
                "artifact_handoff_promotion",
                "external_handoff_promotion",
                "approval_resolution_promotion",
                "timeline_operator_summary_promotion",
            ],
            "promotion needs a redacted audit receipt before any future execution",
        ),
    ]
}

pub fn work_graph_promotion_precondition_audit_receipts()
-> Vec<WorkGraphPromotionAuditReceiptPreview> {
    vec![
        audit_receipt(
            "terminal_task_result_receipt",
            "terminal_task_result_promotion",
        ),
        audit_receipt("scheduler_unblock_receipt", "scheduler_unblock_promotion"),
        audit_receipt("artifact_handoff_receipt", "artifact_handoff_promotion"),
        audit_receipt("external_handoff_receipt", "external_handoff_promotion"),
        audit_receipt(
            "approval_resolution_receipt",
            "approval_resolution_promotion",
        ),
        audit_receipt(
            "timeline_operator_summary_receipt",
            "timeline_operator_summary_promotion",
        ),
    ]
}

pub fn work_graph_promotion_precondition_invariants()
-> Vec<WorkGraphPromotionPreconditionInvariantPreview> {
    vec![
        invariant(
            "promotion_requires_durable_identity",
            "promotion preconditions must consume workflow_id, run_id, step_id, checkpoint, replay_key, rollback_anchor, and receipt_hash",
        ),
        invariant(
            "promotion_requires_readback",
            "every promotable target must have readback evidence from replay/readback preview",
        ),
        invariant(
            "scheduler_unblock_requires_dependency_closure",
            "scheduler work cannot become runnable until dependencies, lease, budget, and approvals are current",
        ),
        invariant(
            "handoff_promotion_cannot_deliver",
            "handoff readiness cannot perform external delivery in preview mode",
        ),
        invariant(
            "approval_resolution_is_visible_not_recorded",
            "approval visibility can be previewed without recording an operator decision",
        ),
        invariant(
            "audit_receipts_are_redacted_and_non_persistent",
            "preview audit receipts contain hashes and refs and cannot be persisted",
        ),
        invariant(
            "promotion_precondition_preview_has_no_side_effects",
            "this gate cannot promote state, unblock schedulers, record approvals, or send externally",
        ),
    ]
}

impl WorkGraphPromotionPreconditionPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            promotion_performed: false,
            scheduler_unblock_performed: false,
            handoff_promotion_performed: false,
            approval_recorded: false,
            audit_receipt_persisted: false,
            runtime_mutation_performed: false,
            scheduler_cutover_performed: false,
            adapter_projection_enforced: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn promotion_target(
    id: &'static str,
    node_kind: &'static str,
    target_status: &'static str,
    required_check_ids: Vec<&'static str>,
) -> WorkGraphPromotionTargetPreview {
    WorkGraphPromotionTargetPreview {
        id,
        node_kind,
        target_status,
        required_check_ids,
        blocked_without_readback: true,
        promotes_state: false,
    }
}

fn check(
    id: &'static str,
    applies_to_target_ids: Vec<&'static str>,
    required_evidence_fields: Vec<&'static str>,
    failure_denial_id: &'static str,
) -> WorkGraphPromotionCheckPreview {
    WorkGraphPromotionCheckPreview {
        id,
        applies_to_target_ids,
        required_evidence_fields,
        failure_denial_id,
        required_before_promotion: true,
    }
}

fn denial(
    id: &'static str,
    severity: &'static str,
    blocks_target_ids: Vec<&'static str>,
    operator_message: &'static str,
) -> WorkGraphPromotionDenialPreview {
    WorkGraphPromotionDenialPreview {
        id,
        severity,
        blocks_target_ids,
        operator_message,
    }
}

fn audit_receipt(
    id: &'static str,
    target_id: &'static str,
) -> WorkGraphPromotionAuditReceiptPreview {
    WorkGraphPromotionAuditReceiptPreview {
        id,
        target_id,
        required_fields: vec![
            "workflow_id",
            "run_id",
            "step_id",
            "checkpoint",
            "replay_key",
            "rollback_anchor",
            "receipt_hash",
            "traceId",
            "nodeId",
            "targetStatus",
            "requiredCheckIds",
            "denialIds",
            "redactedEvidenceRefs",
            "receiptHash",
        ],
        persistence_enabled: false,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPromotionPreconditionInvariantPreview {
    WorkGraphPromotionPreconditionInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn promotion_precondition_preview_declares_targets() {
        let report = hepta_work_graph_promotion_precondition_preview_report();
        let target_ids = report
            .promotion_targets
            .iter()
            .map(|target| target.id)
            .collect::<Vec<_>>();

        assert_eq!(
            target_ids,
            [
                "terminal_task_result_promotion",
                "scheduler_unblock_promotion",
                "artifact_handoff_promotion",
                "external_handoff_promotion",
                "approval_resolution_promotion",
                "timeline_operator_summary_promotion",
            ]
        );
        assert_eq!(report.promotion_target_count, 6);
        assert!(
            report
                .promotion_targets
                .iter()
                .all(|target| { target.blocked_without_readback && !target.promotes_state })
        );
    }

    #[test]
    fn promotion_precondition_preview_declares_required_checks_and_denials() {
        let report = hepta_work_graph_promotion_precondition_preview_report();
        let check_ids = report
            .required_checks
            .iter()
            .map(|check| check.id)
            .collect::<Vec<_>>();

        assert_eq!(
            check_ids,
            [
                "durable_identity_evidence_ready",
                "task_result_schema_valid",
                "task_result_readback_clean",
                "artifact_readback_clean",
                "approval_readback_clean",
                "timeline_readback_clean",
                "dependency_closure_satisfied",
                "lease_and_budget_current",
                "operator_authority_scope_valid",
                "approval_expiry_current",
                "handoff_scope_authorized",
                "no_external_delivery_enabled",
                "artifact_redaction_verified",
                "no_replay_drift_detected",
                "operator_audit_receipt_ready",
            ]
        );
        assert_eq!(report.required_check_count, 15);
        assert_eq!(report.denial_reason_count, 15);
        assert!(
            report
                .required_checks
                .iter()
                .all(|check| check.required_before_promotion)
        );
    }

    #[test]
    fn promotion_precondition_preview_requires_audit_receipts_without_persistence() {
        let report = hepta_work_graph_promotion_precondition_preview_report();
        let receipt_targets = report
            .audit_receipts
            .iter()
            .map(|receipt| receipt.target_id)
            .collect::<Vec<_>>();

        assert_eq!(
            receipt_targets,
            [
                "terminal_task_result_promotion",
                "scheduler_unblock_promotion",
                "artifact_handoff_promotion",
                "external_handoff_promotion",
                "approval_resolution_promotion",
                "timeline_operator_summary_promotion",
            ]
        );
        assert_eq!(report.audit_receipt_count, 6);
        assert!(report.audit_receipts.iter().all(|receipt| {
            [
                "workflow_id",
                "run_id",
                "step_id",
                "checkpoint",
                "replay_key",
                "rollback_anchor",
                "receipt_hash",
            ]
            .iter()
            .all(|field| receipt.required_fields.contains(field))
        }));
        assert!(
            report.audit_receipts.iter().all(|receipt| {
                !receipt.persistence_enabled && !receipt.external_delivery_enabled
            })
        );
    }

    #[test]
    fn promotion_precondition_preview_keeps_execution_disabled() {
        let report = hepta_work_graph_promotion_precondition_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphPromotionPreconditionPreviewSideEffects::none()
        );
        assert!(report.ready_for_activation_enforcement_blocker_preview);
        assert!(!report.ready_for_promotion_execution);
        assert!(!report.ready_for_live_execution);
        assert_eq!(report.invariant_count, 7);
    }

    #[test]
    fn promotion_precondition_preview_requires_prior_gates() {
        let report = hepta_work_graph_promotion_precondition_preview_report();

        assert_eq!(
            report.required_prior_gates,
            [
                "hepta_work_graph_contract_preview_gate",
                "hepta_work_graph_task_result_contract_preview_gate",
                "hepta_work_graph_scheduler_admission_controller_preview_gate",
                "hepta_work_graph_observability_timeline_preview_gate",
                "hepta_work_graph_role_manifest_contract_preview_gate",
                "hepta_work_graph_unified_state_store_preview_gate",
                "hepta_work_graph_adapter_projection_fixture_gate",
                "hepta_work_graph_state_store_persistence_preview_gate",
                "hepta_work_graph_replay_readback_preview_gate",
                "hepta_work_graph_durable_identity_preview_gate",
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PROMOTION_PRECONDITION_RECOMMENDED_NEXT_GATE
        );
    }
}
