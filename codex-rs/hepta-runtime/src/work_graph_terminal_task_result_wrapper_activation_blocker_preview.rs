use serde::Serialize;

pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_ACTIVATION_BLOCKER_PREVIEW_GATE: &str =
    "hepta_work_graph_terminal_task_result_wrapper_activation_blocker_preview_gate";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_ACTIVATION_BLOCKER_SCHEMA_VERSION: &str =
    "work_graph_terminal_task_result_wrapper_activation_blocker_preview_v1";
pub const WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_ACTIVATION_BLOCKER_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_readback_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperActivationBlockerPreviewReport {
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
    pub activation_surfaces: Vec<WorkGraphTaskResultWrapperActivationSurfacePreview>,
    pub blockers: Vec<WorkGraphTaskResultWrapperActivationBlockerPreview>,
    pub required_enablements: Vec<WorkGraphTaskResultWrapperActivationEnablementPreview>,
    pub kill_switches: Vec<WorkGraphTaskResultWrapperActivationKillSwitchPreview>,
    pub invariants: Vec<WorkGraphTaskResultWrapperActivationInvariantPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_shadow_activation_readback_preview: bool,
    pub ready_for_activation: bool,
    pub ready_for_promotion_execution: bool,
    pub ready_for_wrapper_execution: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_store_enablement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphTerminalTaskResultWrapperActivationBlockerPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperActivationSurfacePreview {
    pub id: &'static str,
    pub risk_class: &'static str,
    pub blocked_by_default: bool,
    pub required_blocker_ids: Vec<&'static str>,
    pub activation_state: &'static str,
    pub enables_runtime_mutation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperActivationBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub applies_to_surface_ids: Vec<&'static str>,
    pub denial_reason: &'static str,
    pub blocks_activation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperActivationEnablementPreview {
    pub id: &'static str,
    pub source_gate_id: &'static str,
    pub required_for_surface_ids: Vec<&'static str>,
    pub required_evidence_fields: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperActivationKillSwitchPreview {
    pub id: &'static str,
    pub target_surface_ids: Vec<&'static str>,
    pub trigger: &'static str,
    pub armed_in_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultWrapperActivationInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalTaskResultWrapperActivationBlockerPreviewSideEffects {
    pub filesystem_written: bool,
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

pub fn hepta_work_graph_terminal_task_result_wrapper_activation_blocker_preview_report()
-> WorkGraphTerminalTaskResultWrapperActivationBlockerPreviewReport {
    let activation_surfaces = work_graph_terminal_task_result_wrapper_activation_surfaces();
    let blockers = work_graph_terminal_task_result_wrapper_activation_blockers();
    let required_enablements =
        work_graph_terminal_task_result_wrapper_activation_required_enablements();
    let kill_switches = work_graph_terminal_task_result_wrapper_activation_kill_switches();
    let invariants = work_graph_terminal_task_result_wrapper_activation_invariants();
    let required_prior_gates =
        work_graph_terminal_task_result_wrapper_activation_required_prior_gates();

    WorkGraphTerminalTaskResultWrapperActivationBlockerPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_ACTIVATION_BLOCKER_PREVIEW_GATE,
        schema_version: WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_ACTIVATION_BLOCKER_SCHEMA_VERSION,
        preview_mode: "read_only_terminal_task_result_wrapper_activation_blocker_preview_no_activation",
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
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_ACTIVATION_BLOCKER_RECOMMENDED_NEXT_GATE,
        ready_for_shadow_activation_readback_preview: true,
        ready_for_activation: false,
        ready_for_promotion_execution: false,
        ready_for_wrapper_execution: false,
        ready_for_task_result_enforcement: false,
        ready_for_store_enablement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphTerminalTaskResultWrapperActivationBlockerPreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_task_result_wrapper_activation_surface_ids() -> Vec<&'static str> {
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

pub fn work_graph_terminal_task_result_wrapper_activation_surfaces()
-> Vec<WorkGraphTaskResultWrapperActivationSurfacePreview> {
    vec![
        surface(
            "wrapper_execution_activation",
            "runtime_execution",
            vec![
                "promotion_preconditions_unsatisfied",
                "wrapper_execution_disabled",
                "runtime_attachment_disabled",
                "kill_switches_preview_only",
            ],
        ),
        surface(
            "readback_execution_activation",
            "readback_execution",
            vec![
                "promotion_preconditions_unsatisfied",
                "readback_execution_disabled",
                "zero_tolerance_drift_not_executed",
                "operator_review_missing",
            ],
        ),
        surface(
            "promotion_execution_activation",
            "state_promotion",
            vec![
                "promotion_preconditions_unsatisfied",
                "audit_receipt_persistence_disabled",
                "promotion_execution_disabled",
                "runtime_attachment_disabled",
            ],
        ),
        surface(
            "task_result_enforcement_activation",
            "contract_enforcement",
            vec![
                "promotion_preconditions_unsatisfied",
                "task_result_enforcement_disabled",
                "zero_tolerance_drift_not_executed",
                "redaction_precondition_unsatisfied",
            ],
        ),
        surface(
            "store_enablement_activation",
            "state_write",
            vec![
                "promotion_preconditions_unsatisfied",
                "store_enablement_disabled",
                "audit_receipt_persistence_disabled",
                "runtime_attachment_disabled",
            ],
        ),
        surface(
            "live_execution_activation",
            "live_runtime",
            vec![
                "promotion_preconditions_unsatisfied",
                "live_execution_disabled",
                "kill_switches_preview_only",
                "operator_review_missing",
            ],
        ),
        surface(
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

pub fn work_graph_terminal_task_result_wrapper_activation_blockers()
-> Vec<WorkGraphTaskResultWrapperActivationBlockerPreview> {
    vec![
        blocker(
            "promotion_preconditions_unsatisfied",
            "critical",
            work_graph_terminal_task_result_wrapper_activation_surface_ids(),
            "terminal wrapper promotion preconditions are preview-only and not satisfied",
        ),
        blocker(
            "zero_tolerance_drift_not_executed",
            "critical",
            vec![
                "readback_execution_activation",
                "task_result_enforcement_activation",
            ],
            "zero-tolerance drift budgets have not been proven by readback execution",
        ),
        blocker(
            "operator_review_missing",
            "high",
            vec![
                "readback_execution_activation",
                "live_execution_activation",
                "external_delivery_activation",
            ],
            "operator summary review has not been performed",
        ),
        blocker(
            "audit_receipt_persistence_disabled",
            "high",
            vec![
                "promotion_execution_activation",
                "store_enablement_activation",
            ],
            "promotion audit receipts are non-persistent preview artifacts",
        ),
        blocker(
            "wrapper_execution_disabled",
            "medium",
            vec!["wrapper_execution_activation"],
            "terminal TaskResult wrappers are not attached to runtime execution paths",
        ),
        blocker(
            "readback_execution_disabled",
            "medium",
            vec!["readback_execution_activation"],
            "readback probes are contract-only and do not execute",
        ),
        blocker(
            "promotion_execution_disabled",
            "medium",
            vec!["promotion_execution_activation"],
            "promotion execution remains disabled after precondition preview",
        ),
        blocker(
            "task_result_enforcement_disabled",
            "medium",
            vec!["task_result_enforcement_activation"],
            "TaskResult contract enforcement is not enabled",
        ),
        blocker(
            "store_enablement_disabled",
            "medium",
            vec!["store_enablement_activation"],
            "store, WAL, checkpoint, and graph persistence remain disabled",
        ),
        blocker(
            "live_execution_disabled",
            "critical",
            vec!["live_execution_activation"],
            "live execution is explicitly out of scope for this preview",
        ),
        blocker(
            "external_delivery_disabled",
            "critical",
            vec!["external_delivery_activation"],
            "external delivery is explicitly disabled for terminal wrapper activation",
        ),
        blocker(
            "redaction_precondition_unsatisfied",
            "critical",
            vec![
                "task_result_enforcement_activation",
                "external_delivery_activation",
            ],
            "redaction drift must remain zero before any enforcement or delivery surface",
        ),
        blocker(
            "runtime_attachment_disabled",
            "medium",
            vec![
                "wrapper_execution_activation",
                "promotion_execution_activation",
                "store_enablement_activation",
            ],
            "activation blockers are not attached to runtime paths",
        ),
        blocker(
            "kill_switches_preview_only",
            "medium",
            vec!["wrapper_execution_activation", "live_execution_activation"],
            "kill switches are declared but not wired to runtime activation",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_activation_required_enablements()
-> Vec<WorkGraphTaskResultWrapperActivationEnablementPreview> {
    vec![
        enablement(
            "promotion_precondition_report",
            "hepta_work_graph_terminal_task_result_wrapper_promotion_precondition_preview_gate",
            work_graph_terminal_task_result_wrapper_activation_surface_ids(),
            vec!["targetCount", "preconditionBindingCount", "blockerCount"],
        ),
        enablement(
            "zero_tolerance_drift_budget_report",
            "hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_gate",
            vec![
                "readback_execution_activation",
                "task_result_enforcement_activation",
                "external_delivery_activation",
            ],
            vec![
                "maxAllowedMismatches",
                "maxAllowedUnreviewedFindings",
                "maxReplayLagMs",
            ],
        ),
        enablement(
            "operator_review_packet",
            "hepta_work_graph_terminal_task_result_wrapper_drift_budget_preview_gate",
            vec![
                "readback_execution_activation",
                "live_execution_activation",
                "external_delivery_activation",
            ],
            vec!["reviewerIdHash", "reviewedAtUnixMs", "summaryHash"],
        ),
        enablement(
            "non_persistent_audit_receipt_readback",
            "hepta_work_graph_terminal_task_result_wrapper_promotion_precondition_preview_gate",
            vec![
                "promotion_execution_activation",
                "store_enablement_activation",
            ],
            vec!["receiptHash", "redactedEvidenceRefs", "blockerIds"],
        ),
        enablement(
            "runtime_attachment_plan",
            "hepta_work_graph_terminal_task_result_wrapper_promotion_precondition_preview_gate",
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

pub fn work_graph_terminal_task_result_wrapper_activation_kill_switches()
-> Vec<WorkGraphTaskResultWrapperActivationKillSwitchPreview> {
    vec![
        kill_switch(
            "kill_all_terminal_wrapper_activation",
            work_graph_terminal_task_result_wrapper_activation_surface_ids(),
            "operator disables terminal wrapper activation",
        ),
        kill_switch(
            "kill_wrapper_execution_activation",
            vec!["wrapper_execution_activation"],
            "wrapper execution attachment diverges from preview contract",
        ),
        kill_switch(
            "kill_task_result_enforcement_activation",
            vec!["task_result_enforcement_activation"],
            "TaskResult enforcement rejects a terminal wrapper output",
        ),
        kill_switch(
            "kill_store_enablement_activation",
            vec!["store_enablement_activation"],
            "store or WAL write is attempted from preview-only path",
        ),
        kill_switch(
            "kill_external_delivery_activation",
            vec!["external_delivery_activation"],
            "external delivery is requested from terminal wrapper activation",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_activation_invariants()
-> Vec<WorkGraphTaskResultWrapperActivationInvariantPreview> {
    vec![
        invariant(
            "activation_surfaces_blocked_by_default",
            "every terminal wrapper activation surface remains blocked until explicit future enablement",
        ),
        invariant(
            "promotion_preconditions_do_not_authorize_execution",
            "promotion precondition preview can describe blockers but cannot execute or promote",
        ),
        invariant(
            "zero_tolerance_drift_must_be_proven_before_enforcement",
            "TaskResult enforcement requires executed readback proving zero critical drift",
        ),
        invariant(
            "audit_receipts_are_not_persistence_authority",
            "non-persistent audit receipts cannot authorize store, WAL, checkpoint, or graph writes",
        ),
        invariant(
            "external_delivery_stays_disabled",
            "terminal wrapper activation cannot send externally or publish public claims",
        ),
        invariant(
            "activation_blocker_preview_has_no_side_effects",
            "this preview cannot activate, execute wrappers, enforce TaskResult, persist state, or send externally",
        ),
    ]
}

pub fn work_graph_terminal_task_result_wrapper_activation_required_prior_gates() -> Vec<&'static str>
{
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
    ]
}

impl WorkGraphTerminalTaskResultWrapperActivationBlockerPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
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
    required_blocker_ids: Vec<&'static str>,
) -> WorkGraphTaskResultWrapperActivationSurfacePreview {
    WorkGraphTaskResultWrapperActivationSurfacePreview {
        id,
        risk_class,
        blocked_by_default: true,
        required_blocker_ids,
        activation_state: "blocked_preview_only",
        enables_runtime_mutation: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    applies_to_surface_ids: Vec<&'static str>,
    denial_reason: &'static str,
) -> WorkGraphTaskResultWrapperActivationBlockerPreview {
    WorkGraphTaskResultWrapperActivationBlockerPreview {
        id,
        severity,
        applies_to_surface_ids,
        denial_reason,
        blocks_activation: true,
    }
}

fn enablement(
    id: &'static str,
    source_gate_id: &'static str,
    required_for_surface_ids: Vec<&'static str>,
    required_evidence_fields: Vec<&'static str>,
) -> WorkGraphTaskResultWrapperActivationEnablementPreview {
    WorkGraphTaskResultWrapperActivationEnablementPreview {
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
) -> WorkGraphTaskResultWrapperActivationKillSwitchPreview {
    WorkGraphTaskResultWrapperActivationKillSwitchPreview {
        id,
        target_surface_ids,
        trigger,
        armed_in_preview: true,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphTaskResultWrapperActivationInvariantPreview {
    WorkGraphTaskResultWrapperActivationInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapper_activation_blocker_declares_blocked_surfaces() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_activation_blocker_preview_report();

        assert_eq!(report.activation_surface_count, 7);
        assert!(report.activation_surfaces.iter().all(|surface| {
            surface.blocked_by_default
                && surface.activation_state == "blocked_preview_only"
                && !surface.enables_runtime_mutation
                && surface.required_blocker_ids.len() == 4
        }));
    }

    #[test]
    fn wrapper_activation_blocker_blocks_every_runtime_surface() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_activation_blocker_preview_report();

        assert_eq!(report.blocker_count, 14);
        assert!(report.blockers.iter().all(|blocker| {
            blocker.blocks_activation && !blocker.applies_to_surface_ids.is_empty()
        }));
        assert!(
            report
                .blockers
                .iter()
                .any(|blocker| blocker.id == "external_delivery_disabled")
        );
    }

    #[test]
    fn wrapper_activation_blocker_keeps_enablements_unsatisfied() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_activation_blocker_preview_report();

        assert_eq!(report.required_enablement_count, 5);
        assert!(report.required_enablements.iter().all(|enablement| {
            !enablement.currently_satisfied
                && !enablement.required_for_surface_ids.is_empty()
                && !enablement.required_evidence_fields.is_empty()
        }));
    }

    #[test]
    fn wrapper_activation_blocker_keeps_execution_and_persistence_disabled() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_activation_blocker_preview_report();

        assert!(report.ready_for_shadow_activation_readback_preview);
        assert!(!report.ready_for_activation);
        assert!(!report.ready_for_promotion_execution);
        assert!(!report.ready_for_wrapper_execution);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_store_enablement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphTerminalTaskResultWrapperActivationBlockerPreviewSideEffects::none()
        );
    }

    #[test]
    fn wrapper_activation_blocker_requires_promotion_precondition_prior() {
        let report =
            hepta_work_graph_terminal_task_result_wrapper_activation_blocker_preview_report();

        assert_eq!(report.kill_switch_count, 5);
        assert_eq!(report.invariant_count, 6);
        assert_eq!(report.required_prior_gate_count, 17);
        assert_eq!(
            report.required_prior_gates.last(),
            Some(
                &"hepta_work_graph_terminal_task_result_wrapper_promotion_precondition_preview_gate"
            )
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_TERMINAL_TASK_RESULT_WRAPPER_ACTIVATION_BLOCKER_RECOMMENDED_NEXT_GATE
        );
    }
}
