use serde::Serialize;

pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_PREVIEW_GATE:
    &str =
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_gate";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_SCHEMA_VERSION:
    &str =
    "work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_v1";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub activation_surface_count: usize,
    pub blocker_count: usize,
    pub required_enablement_count: usize,
    pub kill_switch_count: usize,
    pub invariant_count: usize,
    pub required_prior_gate_count: usize,
    pub activation_surfaces:
        Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationSurfacePreview>,
    pub blockers:
        Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPreview>,
    pub required_enablements:
        Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationEnablementPreview>,
    pub kill_switches:
        Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationKillSwitchPreview>,
    pub invariants:
        Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationInvariantPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview: bool,
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
        WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationSurfacePreview
{
    pub id: &'static str,
    pub risk_class: &'static str,
    pub source_target_id: &'static str,
    pub blocked_by_default: bool,
    pub required_blocker_ids: Vec<&'static str>,
    pub required_enablement_ids: Vec<&'static str>,
    pub activation_state: &'static str,
    pub enables_runtime_mutation: bool,
    pub enables_external_side_effect: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPreview
{
    pub id: &'static str,
    pub severity: &'static str,
    pub applies_to_surface_ids: Vec<&'static str>,
    pub denial_reason: &'static str,
    pub source_gate_id: &'static str,
    pub blocks_shadow_activation_blocker_activation_blocker_readback: bool,
    pub blocks_shadow_activation_blocker_activation_blocker_activation: bool,
    pub blocks_shadow_activation_blocker_activation_blocker_promotion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationEnablementPreview
{
    pub id: &'static str,
    pub source_gate_id: &'static str,
    pub required_for_surface_ids: Vec<&'static str>,
    pub required_evidence_fields: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationKillSwitchPreview
{
    pub id: &'static str,
    pub target_surface_ids: Vec<&'static str>,
    pub trigger: &'static str,
    pub armed_in_preview: bool,
    pub persists_switch_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationInvariantPreview
{
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPreviewSideEffects
{
    pub filesystem_written: bool,
    pub shadow_activation_blocker_activation_blocker_activation_blocker_persisted: bool,
    pub shadow_activation_blocker_activation_blocker_promotion_precondition_persisted: bool,
    pub audit_receipt_persisted: bool,
    pub shadow_activation_blocker_activation_blocker_readback_performed: bool,
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

pub fn hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_report()
-> WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPreviewReport
{
    let activation_surfaces =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_surfaces();
    let blockers =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blockers();
    let required_enablements =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_required_enablements();
    let kill_switches =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_kill_switches();
    let invariants =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_invariants();
    let required_prior_gates =
        work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_required_prior_gates();

    WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_SCHEMA_VERSION,
        preview_mode:
            "read_only_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_preview_no_activation",
        activation_surface_count: activation_surfaces.len(),
        blocker_count: blockers.len(),
        required_enablement_count: required_enablements.len(),
        kill_switch_count: kill_switches.len(),
        invariant_count: invariants.len(),
        required_prior_gate_count: required_prior_gates.len(),
        activation_surfaces,
        blockers,
        required_enablements,
        kill_switches,
        invariants,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_RECOMMENDED_NEXT_GATE,
        ready_for_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview: true,
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
            WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_surface_ids()
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

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_surfaces()
-> Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationSurfacePreview>{
    vec![
        surface(
            "wrapper_execution_activation",
            "runtime_execution",
            "shadow_activation_blocker_activation_blocker_promote_wrapper_execution_activation",
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
        surface(
            "readback_execution_activation",
            "readback_execution",
            "shadow_activation_blocker_activation_blocker_promote_readback_execution_activation",
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
        surface(
            "promotion_execution_activation",
            "state_promotion",
            "shadow_activation_blocker_activation_blocker_promote_promotion_execution_activation",
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
        surface(
            "task_result_enforcement_activation",
            "contract_enforcement",
            "shadow_activation_blocker_activation_blocker_promote_task_result_enforcement_activation",
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
        surface(
            "store_enablement_activation",
            "state_write",
            "shadow_activation_blocker_activation_blocker_promote_store_enablement_activation",
            vec![
                "shadow_activation_blocker_activation_blocker_promotion_preconditions_unsatisfied",
                "shadow_activation_blocker_activation_blocker_store_enablement_disabled",
                "shadow_activation_blocker_activation_blocker_audit_receipts_non_persistent",
                "shadow_activation_blocker_activation_blocker_activation_blocker_persistence_disabled",
            ],
            vec![
                "shadow_activation_blocker_activation_blocker_promotion_precondition_report",
                "shadow_activation_blocker_activation_blocker_non_persistent_audit_receipt_readback",
                "shadow_activation_blocker_activation_blocker_side_effect_lock_proof",
            ],
        ),
        surface(
            "live_execution_activation",
            "live_runtime",
            "shadow_activation_blocker_activation_blocker_promote_live_execution_activation",
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
        surface(
            "external_delivery_activation",
            "external_side_effect",
            "shadow_activation_blocker_activation_blocker_promote_external_delivery_activation",
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

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blockers()
-> Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPreview>{
    vec![
        blocker(
            "shadow_activation_blocker_activation_blocker_promotion_preconditions_unsatisfied",
            "critical",
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_surface_ids(),
            "shadow activation blocker activation-blocker promotion preconditions are preview-only and not satisfied",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_drift_budgets_not_executed",
            "critical",
            vec![
                "readback_execution_activation",
                "task_result_enforcement_activation",
                "live_execution_activation",
                "external_delivery_activation",
            ],
            "shadow activation blocker activation-blocker zero-tolerance drift budgets have not been proven by executed readback",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_operator_review_missing",
            "high",
            vec![
                "readback_execution_activation",
                "live_execution_activation",
                "external_delivery_activation",
            ],
            "shadow activation blocker activation-blocker operator review has not been performed",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_audit_receipts_non_persistent",
            "high",
            vec![
                "promotion_execution_activation",
                "store_enablement_activation",
            ],
            "shadow activation blocker activation-blocker audit receipts are non-persistent preview artifacts",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_side_effect_lock_not_proven",
            "critical",
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_surface_ids(),
            "shadow activation blocker activation-blocker side-effect lock has not been proven by executed readback",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_wrapper_execution_disabled",
            "medium",
            vec!["wrapper_execution_activation"],
            "shadow activation blocker activation-blocker wrapper execution attachment is not enabled",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_readback_execution_disabled",
            "medium",
            vec!["readback_execution_activation"],
            "shadow activation blocker activation-blocker readback probes are contract-only and do not execute",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_promotion_execution_disabled",
            "medium",
            vec!["promotion_execution_activation"],
            "shadow activation blocker activation-blocker promotion execution remains disabled after precondition preview",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_task_result_enforcement_disabled",
            "medium",
            vec!["task_result_enforcement_activation"],
            "shadow activation blocker activation-blocker TaskResult contract enforcement is not enabled",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_store_enablement_disabled",
            "medium",
            vec!["store_enablement_activation"],
            "shadow activation blocker activation-blocker store, WAL, checkpoint, and graph persistence remain disabled",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_live_execution_disabled",
            "critical",
            vec!["live_execution_activation"],
            "shadow activation blocker activation-blocker live execution is explicitly out of scope for this preview",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_external_delivery_disabled",
            "critical",
            vec!["external_delivery_activation"],
            "shadow activation blocker activation-blocker external delivery is explicitly disabled for activation",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_runtime_attachment_disabled",
            "medium",
            vec![
                "wrapper_execution_activation",
                "promotion_execution_activation",
                "task_result_enforcement_activation",
                "store_enablement_activation",
            ],
            "shadow activation blocker activation-blocker blockers are not attached to runtime paths",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_kill_switches_preview_only",
            "medium",
            vec![
                "wrapper_execution_activation",
                "live_execution_activation",
                "external_delivery_activation",
            ],
            "shadow activation blocker activation-blocker kill switches are declared but not wired to runtime activation",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_activation_blocker_persistence_disabled",
            "medium",
            vec![
                "store_enablement_activation",
                "promotion_execution_activation",
            ],
            "shadow activation blocker activation-blocker state cannot be persisted from this preview",
        ),
        blocker(
            "shadow_activation_blocker_activation_blocker_explicit_operator_approval_absent",
            "high",
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_surface_ids(),
            "no explicit future operator approval exists for shadow activation blocker activation-blocker activation or promotion",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_required_enablements()
-> Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationEnablementPreview>{
    vec![
        enablement(
            "shadow_activation_blocker_activation_blocker_promotion_precondition_report",
            "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_gate",
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_surface_ids(),
            vec!["targetCount", "preconditionBindingCount", "blockerCount"],
        ),
        enablement(
            "shadow_activation_blocker_activation_blocker_zero_tolerance_drift_budget_report",
            "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_gate",
            vec![
                "readback_execution_activation",
                "task_result_enforcement_activation",
                "live_execution_activation",
                "external_delivery_activation",
            ],
            vec![
                "maxAllowedMismatches",
                "maxAllowedUnreviewedFindings",
                "maxReplayLagMs",
            ],
        ),
        enablement(
            "shadow_activation_blocker_activation_blocker_operator_review_packet",
            "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_gate",
            vec![
                "readback_execution_activation",
                "live_execution_activation",
                "external_delivery_activation",
            ],
            vec!["reviewerIdHash", "reviewedAtUnixMs", "summaryHash"],
        ),
        enablement(
            "shadow_activation_blocker_activation_blocker_non_persistent_audit_receipt_readback",
            "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_gate",
            vec![
                "promotion_execution_activation",
                "store_enablement_activation",
            ],
            vec!["receiptHash", "redactedEvidenceRefs", "blockerIds"],
        ),
        enablement(
            "shadow_activation_blocker_activation_blocker_side_effect_lock_proof",
            "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_gate",
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_surface_ids(),
            vec![
                "activationPerformed",
                "taskResultEnforcementEnabled",
                "storePersistenceEnabled",
            ],
        ),
        enablement(
            "shadow_activation_blocker_activation_blocker_runtime_attachment_plan",
            "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_gate",
            vec![
                "wrapper_execution_activation",
                "promotion_execution_activation",
                "task_result_enforcement_activation",
                "store_enablement_activation",
            ],
            vec!["attachmentPoint", "rollbackPlanId", "killSwitchId"],
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_kill_switches()
-> Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationKillSwitchPreview>{
    vec![
        kill_switch(
            "kill_all_shadow_activation_blocker_activation_blocker_activation",
            work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_surface_ids(),
            "operator disables all shadow activation blocker activation-blocker activation surfaces",
        ),
        kill_switch(
            "kill_shadow_activation_blocker_activation_blocker_wrapper_execution_activation",
            vec!["wrapper_execution_activation"],
            "shadow activation blocker activation-blocker wrapper execution attachment diverges from preview contract",
        ),
        kill_switch(
            "kill_shadow_activation_blocker_activation_blocker_task_result_enforcement_activation",
            vec!["task_result_enforcement_activation"],
            "shadow activation blocker activation-blocker TaskResult enforcement rejects a terminal wrapper output",
        ),
        kill_switch(
            "kill_shadow_activation_blocker_activation_blocker_store_enablement_activation",
            vec!["store_enablement_activation"],
            "shadow activation blocker activation-blocker store or WAL write is attempted from preview-only path",
        ),
        kill_switch(
            "kill_shadow_activation_blocker_activation_blocker_live_execution_activation",
            vec!["live_execution_activation"],
            "shadow activation blocker activation-blocker live execution is requested from activation preview",
        ),
        kill_switch(
            "kill_shadow_activation_blocker_activation_blocker_external_delivery_activation",
            vec!["external_delivery_activation"],
            "shadow activation blocker activation-blocker external delivery is requested from activation preview",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_invariants()
-> Vec<WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationInvariantPreview>{
    vec![
        invariant(
            "shadow_activation_blocker_activation_blocker_activation_surfaces_blocked_by_default",
            "every shadow activation blocker activation-blocker activation surface remains blocked until explicit future enablement",
        ),
        invariant(
            "shadow_activation_blocker_activation_blocker_promotion_preconditions_do_not_authorize_activation",
            "shadow activation blocker activation-blocker promotion preconditions describe blockers but cannot execute, activate, or promote",
        ),
        invariant(
            "zero_tolerance_shadow_activation_blocker_activation_blocker_drift_must_be_executed_before_activation",
            "shadow activation blocker activation-blocker TaskResult enforcement and activation require executed readback proving zero critical drift",
        ),
        invariant(
            "shadow_activation_blocker_activation_blocker_audit_receipts_are_not_persistence_authority",
            "non-persistent shadow activation blocker activation-blocker audit receipts cannot authorize store, WAL, checkpoint, or graph writes",
        ),
        invariant(
            "shadow_activation_blocker_activation_blocker_side_effect_lock_blocks_live_store_and_external_surfaces",
            "shadow activation blocker activation-blocker side-effect lock must remain false before store, live, or external surfaces can activate",
        ),
        invariant(
            "shadow_activation_blocker_activation_blocker_activation_blocker_preview_has_no_side_effects",
            "this preview cannot activate, promote, execute wrappers, enforce TaskResult, persist state, or send externally",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_required_prior_gates()
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
    ]
}

impl WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            shadow_activation_blocker_activation_blocker_activation_blocker_persisted: false,
            shadow_activation_blocker_activation_blocker_promotion_precondition_persisted: false,
            audit_receipt_persisted: false,
            shadow_activation_blocker_activation_blocker_readback_performed: false,
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

fn surface(
    id: &'static str,
    risk_class: &'static str,
    source_target_id: &'static str,
    required_blocker_ids: Vec<&'static str>,
    required_enablement_ids: Vec<&'static str>,
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationSurfacePreview{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationSurfacePreview {
        id,
        risk_class,
        source_target_id,
        blocked_by_default: true,
        required_blocker_ids,
        required_enablement_ids,
        activation_state: "blocked_preview_only",
        enables_runtime_mutation: false,
        enables_external_side_effect: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    applies_to_surface_ids: Vec<&'static str>,
    denial_reason: &'static str,
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPreview{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPreview {
        id,
        severity,
        applies_to_surface_ids,
        denial_reason,
        source_gate_id: "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_gate",
        blocks_shadow_activation_blocker_activation_blocker_readback: true,
        blocks_shadow_activation_blocker_activation_blocker_activation: true,
        blocks_shadow_activation_blocker_activation_blocker_promotion: true,
    }
}

fn enablement(
    id: &'static str,
    source_gate_id: &'static str,
    required_for_surface_ids: Vec<&'static str>,
    required_evidence_fields: Vec<&'static str>,
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationEnablementPreview{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationEnablementPreview {
        id,
        source_gate_id,
        required_for_surface_ids,
        required_evidence_fields,
        currently_satisfied: false,
    }
}

fn kill_switch(
    id: &'static str,
    target_surface_ids: Vec<&'static str>,
    trigger: &'static str,
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationKillSwitchPreview{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationKillSwitchPreview {
        id,
        target_surface_ids,
        trigger,
        armed_in_preview: true,
        persists_switch_state: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationInvariantPreview{
    WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_activation_blocker_activation_blocker_activation_blocker_declares_blocked_surfaces() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_report();

        assert_eq!(report.activation_surface_count, 7);
        assert!(report.activation_surfaces.iter().all(|surface| {
            surface.blocked_by_default
                && surface.activation_state == "blocked_preview_only"
                && !surface.enables_runtime_mutation
                && !surface.enables_external_side_effect
                && surface.required_blocker_ids.len() == 4
                && surface.required_enablement_ids.len() == 3
        }));
    }

    #[test]
    fn shadow_activation_blocker_activation_blocker_activation_blocker_keeps_enablements_unsatisfied()
     {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_report();

        assert_eq!(report.blocker_count, 16);
        assert_eq!(report.required_enablement_count, 6);
        assert!(report.blockers.iter().all(|blocker| {
            !blocker.applies_to_surface_ids.is_empty()
                && blocker.blocks_shadow_activation_blocker_activation_blocker_readback
                && blocker.blocks_shadow_activation_blocker_activation_blocker_activation
                && blocker.blocks_shadow_activation_blocker_activation_blocker_promotion
        }));
        assert!(
            report
                .required_enablements
                .iter()
                .all(|enablement| !enablement.currently_satisfied)
        );
    }

    #[test]
    fn shadow_activation_blocker_activation_blocker_activation_blocker_declares_kill_switches_and_invariants()
     {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_report();

        assert_eq!(report.kill_switch_count, 6);
        assert_eq!(report.invariant_count, 6);
        assert!(report.kill_switches.iter().all(|kill_switch| {
            kill_switch.armed_in_preview && !kill_switch.persists_switch_state
        }));
        assert!(report.invariants.iter().all(|invariant| invariant.required));
    }

    #[test]
    fn shadow_activation_blocker_activation_blocker_activation_blocker_keeps_execution_store_and_live_disabled()
     {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_report();

        assert!(
            report
                .ready_for_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview
        );
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
            WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPreviewSideEffects::none()
        );
    }

    #[test]
    fn shadow_activation_blocker_activation_blocker_activation_blocker_requires_promotion_precondition_prior()
     {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_preview_report();

        assert_eq!(report.required_prior_gate_count, 49);
        assert_eq!(
            report.required_prior_gates.last(),
            Some(
                &"hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_gate"
            )
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_SHADOW_ACTIVATION_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_ACTIVATION_BLOCKER_RECOMMENDED_NEXT_GATE
        );
    }
}
