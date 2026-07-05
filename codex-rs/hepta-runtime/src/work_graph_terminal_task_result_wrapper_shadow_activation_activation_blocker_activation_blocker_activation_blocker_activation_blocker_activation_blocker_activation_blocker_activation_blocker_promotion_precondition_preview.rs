use serde::Serialize;

pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_PROMOTION_PRECONDITION_PREVIEW_GATE:
    &str =
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_gate";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_PROMOTION_PRECONDITION_SCHEMA_VERSION:
    &str =
    "work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_v1";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_PROMOTION_PRECONDITION_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionPreconditionPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub target_count: usize,
    pub precondition_binding_count: usize,
    pub blocker_count: usize,
    pub audit_receipt_count: usize,
    pub invariant_count: usize,
    pub required_prior_gate_count: usize,
    pub targets:
        Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionTargetPreview>,
    pub precondition_bindings: Vec<
        WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionPreconditionBindingPreview,
    >,
    pub blockers:
        Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionBlockerPreview>,
    pub audit_receipts: Vec<
        WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionAuditReceiptPreview,
    >,
    pub invariants:
        Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionInvariantPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview: bool,
    pub ready_for_shadow_activation_blocker_activation_blocker_promotion_execution: bool,
    pub ready_for_shadow_activation_blocker_activation_blocker_readback_execution: bool,
    pub ready_for_shadow_activation_execution: bool,
    pub ready_for_activation: bool,
    pub ready_for_wrapper_execution: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_store_enablement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionPreconditionPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionTargetPreview
{
    pub id: &'static str,
    pub activation_surface_id: &'static str,
    pub activation_category: &'static str,
    pub required_precondition_ids: Vec<&'static str>,
    pub required_drift_budget_ids: Vec<&'static str>,
    pub required_operator_summary_ids: Vec<&'static str>,
    pub audit_receipt_id: &'static str,
    pub promotion_state: &'static str,
    pub blocks_shadow_activation_blocker_activation_blocker_readback: bool,
    pub blocks_activation: bool,
    pub blocks_promotion_execution: bool,
    pub promotes_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionPreconditionBindingPreview
{
    pub id: &'static str,
    pub source_gate_id: &'static str,
    pub required_budget_ids: Vec<&'static str>,
    pub required_summary_ids: Vec<&'static str>,
    pub required_evidence_fields: Vec<&'static str>,
    pub failure_blocker_id: &'static str,
    pub blocks_shadow_activation_blocker_activation_blocker_readback: bool,
    pub blocks_activation: bool,
    pub blocks_promotion: bool,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionBlockerPreview
{
    pub id: &'static str,
    pub severity: &'static str,
    pub blocks_target_ids: Vec<&'static str>,
    pub operator_message: &'static str,
    pub required_before_shadow_activation_blocker_activation_blocker_readback: bool,
    pub required_before_shadow_activation: bool,
    pub required_before_shadow_promotion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionAuditReceiptPreview
{
    pub id: &'static str,
    pub target_id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub redaction_policy: &'static str,
    pub persists_receipt: bool,
    pub authorizes_activation: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionInvariantPreview
{
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionPreconditionPreviewSideEffects
{
    pub filesystem_written: bool,
    pub shadow_activation_blocker_activation_blocker_promotion_precondition_persisted: bool,
    pub audit_receipt_persisted: bool,
    pub promotion_state_mutated: bool,
    pub shadow_activation_blocker_activation_blocker_readback_performed: bool,
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

pub fn hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_report()
-> WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionPreconditionPreviewReport{
    let targets =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_targets();
    let precondition_bindings =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_bindings();
    let blockers =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_blockers();
    let audit_receipts =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_audit_receipts();
    let invariants =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_invariants();
    let required_prior_gates =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_required_prior_gates();

    WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionPreconditionPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_PROMOTION_PRECONDITION_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_PROMOTION_PRECONDITION_SCHEMA_VERSION,
        preview_mode:
            "read_only_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_no_activation",
        target_count: targets.len(),
        precondition_binding_count: precondition_bindings.len(),
        blocker_count: blockers.len(),
        audit_receipt_count: audit_receipts.len(),
        invariant_count: invariants.len(),
        required_prior_gate_count: required_prior_gates.len(),
        targets,
        precondition_bindings,
        blockers,
        audit_receipts,
        invariants,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_PROMOTION_PRECONDITION_RECOMMENDED_NEXT_GATE,
        ready_for_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview: true,
        ready_for_shadow_activation_blocker_activation_blocker_promotion_execution: false,
        ready_for_shadow_activation_blocker_activation_blocker_readback_execution: false,
        ready_for_shadow_activation_execution: false,
        ready_for_activation: false,
        ready_for_wrapper_execution: false,
        ready_for_task_result_enforcement: false,
        ready_for_store_enablement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionPreconditionPreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_targets()
-> Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionTargetPreview>{
    vec![
        target(
            "shadow_activation_blocker_activation_blocker_promote_wrapper_execution_activation",
            "wrapper_execution_activation",
            "runtime_execution",
            "shadow_activation_blocker_activation_blocker_wrapper_execution_promotion_audit_receipt",
        ),
        target(
            "shadow_activation_blocker_activation_blocker_promote_readback_execution_activation",
            "readback_execution_activation",
            "readback_execution",
            "shadow_activation_blocker_activation_blocker_readback_execution_promotion_audit_receipt",
        ),
        target(
            "shadow_activation_blocker_activation_blocker_promote_promotion_execution_activation",
            "promotion_execution_activation",
            "state_promotion",
            "shadow_activation_blocker_activation_blocker_promotion_execution_promotion_audit_receipt",
        ),
        target(
            "shadow_activation_blocker_activation_blocker_promote_task_result_enforcement_activation",
            "task_result_enforcement_activation",
            "contract_enforcement",
            "shadow_activation_blocker_activation_blocker_task_result_enforcement_promotion_audit_receipt",
        ),
        target(
            "shadow_activation_blocker_activation_blocker_promote_store_enablement_activation",
            "store_enablement_activation",
            "state_write",
            "shadow_activation_blocker_activation_blocker_store_enablement_promotion_audit_receipt",
        ),
        target(
            "shadow_activation_blocker_activation_blocker_promote_live_execution_activation",
            "live_execution_activation",
            "live_runtime",
            "shadow_activation_blocker_activation_blocker_live_execution_promotion_audit_receipt",
        ),
        target(
            "shadow_activation_blocker_activation_blocker_promote_external_delivery_activation",
            "external_delivery_activation",
            "external_side_effect",
            "shadow_activation_blocker_activation_blocker_external_delivery_promotion_audit_receipt",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_bindings()
-> Vec<
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionPreconditionBindingPreview,
>{
    vec![
        precondition_binding(
            "all_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets_zero_tolerance",
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_drift_budget_ids(),
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_drift_operator_summary_ids(),
            vec!["maxAllowedMismatches", "maxAllowedUnreviewedFindings"],
            "shadow_activation_blocker_activation_blocker_drift_budgets_not_executed",
        ),
        precondition_binding(
            "shadow_activation_blocker_activation_blocker_operator_summaries_reviewed",
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_drift_budget_ids(),
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_drift_operator_summary_ids(),
            vec!["reviewerIdHash", "reviewedAtUnixMs", "summaryHash"],
            "shadow_activation_blocker_activation_blocker_operator_review_missing",
        ),
        precondition_binding(
            "shadow_activation_blocker_activation_blocker_side_effect_lock_zero_mutation_required",
            vec!["shadow_activation_blocker_activation_side_effect_lock_drift_zero_tolerance_budget"],
            vec!["shadow_activation_blocker_activation_side_effect_lock_drift_operator_summary"],
            vec![
                "activationPerformed",
                "taskResultEnforcementEnabled",
                "storePersistenceEnabled",
            ],
            "shadow_activation_blocker_activation_blocker_side_effect_lock_not_proven",
        ),
        precondition_binding(
            "shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_remains_disabled_until_budget_review",
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_drift_budget_ids(),
            vec![],
            vec![
                "readyForShadowActivationBlockerActivationBlockerReadbackExecution",
                "readyForActivation",
                "readyForStoreEnablement",
            ],
            "shadow_activation_blocker_activation_blocker_promotion_execution_disabled",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_blockers()
-> Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionBlockerPreview>{
    vec![
        blocker(
            "shadow_activation_blocker_activation_blocker_drift_budgets_not_executed",
            "critical",
            "shadow activation blocker activation-blocker drift budgets are declared but no readback execution has produced findings",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_operator_review_missing",
            "high",
            "shadow activation blocker activation-blocker operator summaries are preview-only and have not been reviewed",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_side_effect_lock_not_proven",
            "critical",
            "shadow activation blocker activation-blocker side-effect lock evidence must remain zero before any activation or promotion",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_runtime_attachment_disabled",
            "medium",
            "shadow activation blocker activation-blocker promotion preconditions are not attached to runtime activation paths",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_promotion_execution_disabled",
            "medium",
            "shadow activation blocker activation-blocker promotion execution remains disabled until a later explicit activation cut",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_audit_receipts()
-> Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionAuditReceiptPreview>{
    work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_targets()
        .iter()
        .map(
            |target| WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionAuditReceiptPreview {
                id: target.audit_receipt_id,
                target_id: target.id,
                required_fields: vec![
                    "activationSurfaceId",
                    "activationCategory",
                    "traceId",
                    "budgetIds",
                    "operatorSummaryIds",
                    "preconditionIds",
                    "blockerIds",
                    "sideEffectLockHash",
                    "redactedEvidenceRefs",
                    "receiptHash",
                ],
                redaction_policy: "only ids, hashes, surface names, and blocker states are allowed",
                persists_receipt: false,
                authorizes_activation: false,
                external_delivery_enabled: false,
            },
        )
        .collect()
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_invariants()
-> Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionInvariantPreview>{
    vec![
        invariant(
            "shadow_activation_blocker_activation_blocker_preconditions_are_preview_only",
            "shadow activation blocker activation-blocker promotion preconditions cannot attach to runtime activation paths",
        ),
        invariant(
            "zero_tolerance_activation_blocker_budgets_do_not_authorize_activation",
            "zero-tolerance activation blocker drift budgets block activation until readback evidence and review exist",
        ),
        invariant(
            "shadow_activation_blocker_activation_blocker_operator_summaries_do_not_record_approval",
            "operator summaries remain non-persistent preview artifacts and do not imply approval",
        ),
        invariant(
            "shadow_activation_blocker_activation_blocker_audit_receipts_are_non_persistent",
            "audit receipts are redacted preview contracts and cannot be stored or delivered",
        ),
        invariant(
            "shadow_activation_blocker_activation_blocker_side_effect_lock_stays_false",
            "all side-effect, store, enforcement, activation, and delivery flags stay false",
        ),
        invariant(
            "shadow_activation_blocker_activation_blocker_promotion_precondition_has_no_side_effects",
            "this preview cannot promote, activate, execute wrappers, enforce TaskResult, write state, or send externally",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_required_prior_gates()
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
    ]
}

impl WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionPreconditionPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            shadow_activation_blocker_activation_blocker_promotion_precondition_persisted: false,
            audit_receipt_persisted: false,
            promotion_state_mutated: false,
            shadow_activation_blocker_activation_blocker_readback_performed: false,
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

fn target(
    id: &'static str,
    activation_surface_id: &'static str,
    activation_category: &'static str,
    audit_receipt_id: &'static str,
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionTargetPreview{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionTargetPreview {
        id,
        activation_surface_id,
        activation_category,
        required_precondition_ids:
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_ids(),
        required_drift_budget_ids:
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_drift_budget_ids(),
        required_operator_summary_ids:
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_drift_operator_summary_ids(),
        audit_receipt_id,
        promotion_state: "blocked_preview_only",
        blocks_shadow_activation_blocker_activation_blocker_readback: true,
        blocks_activation: true,
        blocks_promotion_execution: true,
        promotes_state: false,
    }
}

fn precondition_binding(
    id: &'static str,
    required_budget_ids: Vec<&'static str>,
    required_summary_ids: Vec<&'static str>,
    required_evidence_fields: Vec<&'static str>,
    failure_blocker_id: &'static str,
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionPreconditionBindingPreview
{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionPreconditionBindingPreview {
        id,
        source_gate_id: "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_gate",
        required_budget_ids,
        required_summary_ids,
        required_evidence_fields,
        failure_blocker_id,
        blocks_shadow_activation_blocker_activation_blocker_readback: true,
        blocks_activation: true,
        blocks_promotion: true,
        currently_satisfied: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    operator_message: &'static str,
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionBlockerPreview{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionBlockerPreview {
        id,
        severity,
        blocks_target_ids:
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_targets()
                .iter()
                .map(|target| target.id)
                .collect(),
        operator_message,
        required_before_shadow_activation_blocker_activation_blocker_readback: true,
        required_before_shadow_activation: true,
        required_before_shadow_promotion: true,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionInvariantPreview{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionInvariantPreview {
        id,
        required: true,
        reason,
    }
}

fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_ids()
-> Vec<&'static str> {
    vec![
        "all_shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budgets_zero_tolerance",
        "shadow_activation_blocker_activation_blocker_operator_summaries_reviewed",
        "shadow_activation_blocker_activation_blocker_side_effect_lock_zero_mutation_required",
        "shadow_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_remains_disabled_until_budget_review",
    ]
}

fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_drift_budget_ids()
-> Vec<&'static str> {
    vec![
        "shadow_activation_blocker_activation_blocker_surface_state_drift_zero_tolerance_budget",
        "shadow_activation_blocker_activation_blocker_binding_drift_zero_tolerance_budget",
        "shadow_activation_blocker_activation_enablement_satisfaction_drift_zero_tolerance_budget",
        "shadow_activation_blocker_activation_kill_switch_armament_drift_zero_tolerance_budget",
        "shadow_activation_blocker_activation_side_effect_lock_drift_zero_tolerance_budget",
    ]
}

fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_drift_operator_summary_ids()
-> Vec<&'static str> {
    vec![
        "shadow_activation_blocker_activation_blocker_surface_state_drift_operator_summary",
        "shadow_activation_blocker_activation_blocker_binding_drift_operator_summary",
        "shadow_activation_blocker_activation_enablement_satisfaction_drift_operator_summary",
        "shadow_activation_blocker_activation_kill_switch_armament_drift_operator_summary",
        "shadow_activation_blocker_activation_side_effect_lock_drift_operator_summary",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_activation_blocker_activation_blocker_promotion_precondition_declares_all_activation_targets()
     {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_report();

        assert_eq!(report.target_count, 7);
        assert!(report.targets.iter().all(|target| {
            target.required_precondition_ids.len() == 4
                && target.required_drift_budget_ids.len() == 5
                && target.required_operator_summary_ids.len() == 5
                && target.promotion_state == "blocked_preview_only"
                && target.blocks_shadow_activation_blocker_activation_blocker_readback
                && target.blocks_activation
                && target.blocks_promotion_execution
                && !target.promotes_state
        }));
    }

    #[test]
    fn shadow_activation_blocker_activation_blocker_promotion_precondition_binds_unsatisfied_budget_preconditions()
     {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_report();

        assert_eq!(report.precondition_binding_count, 4);
        assert!(report.precondition_bindings.iter().all(|binding| {
            binding.source_gate_id
                == "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_gate"
                && binding.blocks_shadow_activation_blocker_activation_blocker_readback
                && binding.blocks_activation
                && binding.blocks_promotion
                && !binding.currently_satisfied
        }));
    }

    #[test]
    fn shadow_activation_blocker_activation_blocker_promotion_precondition_declares_non_persistent_receipts()
     {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_report();

        assert_eq!(report.blocker_count, 5);
        assert_eq!(report.audit_receipt_count, 7);
        assert!(report.blockers.iter().all(|blocker| {
            blocker.blocks_target_ids.len() == 7
                && blocker.required_before_shadow_activation_blocker_activation_blocker_readback
                && blocker.required_before_shadow_activation
                && blocker.required_before_shadow_promotion
        }));
        assert!(report.audit_receipts.iter().all(|receipt| {
            receipt.required_fields.contains(&"receiptHash")
                && receipt.required_fields.contains(&"redactedEvidenceRefs")
                && !receipt.persists_receipt
                && !receipt.authorizes_activation
                && !receipt.external_delivery_enabled
        }));
    }

    #[test]
    fn shadow_activation_blocker_activation_blocker_promotion_precondition_keeps_execution_store_and_live_disabled()
     {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_report();

        assert!(report.ready_for_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview);
        assert!(!report.ready_for_shadow_activation_blocker_activation_blocker_promotion_execution);
        assert!(!report.ready_for_shadow_activation_blocker_activation_blocker_readback_execution);
        assert!(!report.ready_for_shadow_activation_execution);
        assert!(!report.ready_for_activation);
        assert!(!report.ready_for_wrapper_execution);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_store_enablement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPromotionPreconditionPreviewSideEffects::none()
        );
    }

    #[test]
    fn shadow_activation_blocker_activation_blocker_promotion_precondition_requires_drift_budget_prior()
     {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_report();

        assert_eq!(report.required_prior_gate_count, 48);
        assert_eq!(
            report.required_prior_gates.last(),
            Some(
                &"hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_gate"
            )
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_PROMOTION_PRECONDITION_RECOMMENDED_NEXT_GATE
        );
    }
}
