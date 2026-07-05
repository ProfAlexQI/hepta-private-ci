use serde::Serialize;

pub const WORK_GRAPH_ACTIVATION_ENFORCEMENT_BLOCKER_PREVIEW_GATE: &str =
    "hepta_work_graph_activation_enforcement_blocker_preview_gate";
pub const WORK_GRAPH_ACTIVATION_ENFORCEMENT_BLOCKER_SCHEMA_VERSION: &str =
    "work_graph_activation_enforcement_blocker_preview_v1";
pub const WORK_GRAPH_ACTIVATION_ENFORCEMENT_BLOCKER_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_shadow_adapter_readback_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphActivationEnforcementBlockerPreviewReport {
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
    pub required_prior_gates: Vec<&'static str>,
    pub activation_surfaces: Vec<WorkGraphActivationSurfacePreview>,
    pub blockers: Vec<WorkGraphActivationBlockerPreview>,
    pub required_enablements: Vec<WorkGraphActivationEnablementPreview>,
    pub kill_switches: Vec<WorkGraphActivationKillSwitchPreview>,
    pub durable_identity_evidence: WorkGraphActivationDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphActivationEnforcementInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_shadow_adapter_readback_preview: bool,
    pub ready_for_activation: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphActivationEnforcementBlockerPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphActivationSurfacePreview {
    pub id: &'static str,
    pub risk_class: &'static str,
    pub blocked_by_default: bool,
    pub required_blocker_ids: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphActivationBlockerPreview {
    pub id: &'static str,
    pub applies_to_surface_ids: Vec<&'static str>,
    pub denial_reason: &'static str,
    pub blocks_activation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphActivationEnablementPreview {
    pub id: &'static str,
    pub required_for_surface_ids: Vec<&'static str>,
    pub required_evidence_fields: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphActivationKillSwitchPreview {
    pub id: &'static str,
    pub target_surface_ids: Vec<&'static str>,
    pub trigger: &'static str,
    pub armed_in_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphActivationDurableIdentityEvidencePreview {
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_surface_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphActivationEnforcementInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphActivationEnforcementBlockerPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub activation_performed: bool,
    pub enforcement_enabled: bool,
    pub store_persistence_enabled: bool,
    pub replay_execution_enabled: bool,
    pub promotion_execution_enabled: bool,
    pub scheduler_cutover_performed: bool,
    pub adapter_projection_enforced: bool,
    pub approval_recorded: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_activation_enforcement_blocker_preview_report()
-> WorkGraphActivationEnforcementBlockerPreviewReport {
    let activation_surfaces = work_graph_activation_surfaces();
    let blockers = work_graph_activation_blockers();
    let required_enablements = work_graph_activation_required_enablements();
    let kill_switches = work_graph_activation_kill_switches();
    let durable_identity_evidence = work_graph_activation_durable_identity_evidence();
    let invariants = work_graph_activation_enforcement_invariants();

    WorkGraphActivationEnforcementBlockerPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_ACTIVATION_ENFORCEMENT_BLOCKER_PREVIEW_GATE,
        schema_version: WORK_GRAPH_ACTIVATION_ENFORCEMENT_BLOCKER_SCHEMA_VERSION,
        preview_mode: "read_only_activation_enforcement_blocker_preview_no_activation",
        activation_surface_count: activation_surfaces.len(),
        blocker_count: blockers.len(),
        required_enablement_count: required_enablements.len(),
        kill_switch_count: kill_switches.len(),
        invariant_count: invariants.len(),
        required_prior_gates: work_graph_activation_enforcement_required_prior_gates(),
        activation_surfaces,
        blockers,
        required_enablements,
        kill_switches,
        durable_identity_evidence,
        invariants,
        recommended_next_gate: WORK_GRAPH_ACTIVATION_ENFORCEMENT_BLOCKER_RECOMMENDED_NEXT_GATE,
        ready_for_shadow_adapter_readback_preview: true,
        ready_for_activation: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphActivationEnforcementBlockerPreviewSideEffects::none(),
    }
}

pub fn work_graph_activation_enforcement_required_prior_gates() -> Vec<&'static str> {
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
        "hepta_work_graph_promotion_precondition_preview_gate",
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_activation_surface_ids() -> Vec<&'static str> {
    vec![
        "store_persistence_activation",
        "wal_replay_execution_activation",
        "promotion_execution_activation",
        "scheduler_cutover_activation",
        "adapter_projection_enforcement_activation",
        "approval_recording_activation",
        "external_delivery_activation",
        "operator_dashboard_publication_activation",
    ]
}

pub fn work_graph_activation_surfaces() -> Vec<WorkGraphActivationSurfacePreview> {
    vec![
        surface(
            "store_persistence_activation",
            "state_write",
            vec![
                "durable_identity_evidence_missing",
                "feature_flag_not_enabled",
                "operator_activation_approval_missing",
                "shadow_readback_gate_missing",
            ],
        ),
        surface(
            "wal_replay_execution_activation",
            "replay_execution",
            vec![
                "durable_identity_evidence_missing",
                "feature_flag_not_enabled",
                "operator_activation_approval_missing",
                "drift_budget_not_configured",
            ],
        ),
        surface(
            "promotion_execution_activation",
            "state_promotion",
            vec![
                "durable_identity_evidence_missing",
                "feature_flag_not_enabled",
                "promotion_receipt_persistence_missing",
                "rollback_plan_missing",
            ],
        ),
        surface(
            "scheduler_cutover_activation",
            "scheduler_runtime",
            vec![
                "durable_identity_evidence_missing",
                "feature_flag_not_enabled",
                "lane_lease_guard_missing",
                "backpressure_policy_missing",
            ],
        ),
        surface(
            "adapter_projection_enforcement_activation",
            "adapter_enforcement",
            vec![
                "durable_identity_evidence_missing",
                "feature_flag_not_enabled",
                "shadow_adapter_readback_missing",
                "source_adapter_opt_in_missing",
            ],
        ),
        surface(
            "approval_recording_activation",
            "operator_decision_write",
            vec![
                "durable_identity_evidence_missing",
                "feature_flag_not_enabled",
                "operator_activation_approval_missing",
                "approval_receipt_store_missing",
            ],
        ),
        surface(
            "external_delivery_activation",
            "external_side_effect",
            vec![
                "durable_identity_evidence_missing",
                "feature_flag_not_enabled",
                "operator_activation_approval_missing",
                "external_delivery_policy_missing",
            ],
        ),
        surface(
            "operator_dashboard_publication_activation",
            "operator_surface",
            vec![
                "durable_identity_evidence_missing",
                "feature_flag_not_enabled",
                "redaction_review_missing",
                "public_claim_promotion_disabled",
            ],
        ),
    ]
}

pub fn work_graph_activation_blockers() -> Vec<WorkGraphActivationBlockerPreview> {
    vec![
        blocker(
            "durable_identity_evidence_missing",
            work_graph_activation_surface_ids(),
            "activation requires workflow, run, step, checkpoint, replay, rollback, and receipt hash evidence",
        ),
        blocker(
            "feature_flag_not_enabled",
            work_graph_activation_surface_ids(),
            "WorkGraph live activation feature flag is absent",
        ),
        blocker(
            "operator_activation_approval_missing",
            vec![
                "store_persistence_activation",
                "wal_replay_execution_activation",
                "approval_recording_activation",
                "external_delivery_activation",
            ],
            "operator has not approved this activation class",
        ),
        blocker(
            "shadow_readback_gate_missing",
            vec!["store_persistence_activation"],
            "state writes require shadow readback evidence first",
        ),
        blocker(
            "drift_budget_not_configured",
            vec!["wal_replay_execution_activation"],
            "replay execution requires an explicit drift budget",
        ),
        blocker(
            "promotion_receipt_persistence_missing",
            vec!["promotion_execution_activation"],
            "promotion execution requires durable audit receipt storage",
        ),
        blocker(
            "rollback_plan_missing",
            vec!["promotion_execution_activation"],
            "promotion execution requires rollback and quarantine plan evidence",
        ),
        blocker(
            "lane_lease_guard_missing",
            vec!["scheduler_cutover_activation"],
            "scheduler cutover requires lane lease and backpressure guards",
        ),
        blocker(
            "backpressure_policy_missing",
            vec!["scheduler_cutover_activation"],
            "scheduler cutover requires bounded queue and retry policy",
        ),
        blocker(
            "shadow_adapter_readback_missing",
            vec!["adapter_projection_enforcement_activation"],
            "adapter enforcement requires shadow adapter readback match evidence",
        ),
        blocker(
            "source_adapter_opt_in_missing",
            vec!["adapter_projection_enforcement_activation"],
            "source adapters must explicitly opt in before enforcement",
        ),
        blocker(
            "approval_receipt_store_missing",
            vec!["approval_recording_activation"],
            "approval recording requires durable receipt and expiry storage",
        ),
        blocker(
            "external_delivery_policy_missing",
            vec!["external_delivery_activation"],
            "external delivery requires policy, scope, and readback gates",
        ),
        blocker(
            "redaction_review_missing",
            vec!["operator_dashboard_publication_activation"],
            "operator dashboard publication requires redaction review",
        ),
        blocker(
            "public_claim_promotion_disabled",
            vec!["operator_dashboard_publication_activation"],
            "preview reports cannot be promoted to public claims",
        ),
    ]
}

pub fn work_graph_activation_required_enablements() -> Vec<WorkGraphActivationEnablementPreview> {
    vec![
        enablement(
            "durable_identity_evidence_packet",
            work_graph_activation_surface_ids(),
            vec![
                "workflow_id",
                "run_id",
                "step_id",
                "checkpoint",
                "replay_key",
                "rollback_anchor",
                "receipt_hash",
            ],
        ),
        enablement(
            "explicit_feature_flag",
            work_graph_activation_surface_ids(),
            vec!["featureFlagName", "enabledAtUnixMs", "operatorIdHash"],
        ),
        enablement(
            "operator_activation_packet",
            vec![
                "store_persistence_activation",
                "wal_replay_execution_activation",
                "approval_recording_activation",
                "external_delivery_activation",
            ],
            vec!["operatorScope", "approvalHash", "expiresAtUnixMs"],
        ),
        enablement(
            "shadow_readback_match",
            vec![
                "store_persistence_activation",
                "adapter_projection_enforcement_activation",
            ],
            vec!["shadowTraceId", "readbackHash", "driftStatus"],
        ),
        enablement(
            "rollback_quarantine_plan",
            vec![
                "promotion_execution_activation",
                "scheduler_cutover_activation",
            ],
            vec!["rollbackPlanId", "quarantineTarget", "recoveryOwner"],
        ),
        enablement(
            "external_scope_policy",
            vec!["external_delivery_activation"],
            vec!["target", "operatorScopeHash", "deliveryReadbackGate"],
        ),
        enablement(
            "redaction_review_packet",
            vec!["operator_dashboard_publication_activation"],
            vec!["redactionState", "reviewerHash", "payloadHash"],
        ),
    ]
}

pub fn work_graph_activation_kill_switches() -> Vec<WorkGraphActivationKillSwitchPreview> {
    vec![
        kill_switch(
            "kill_all_work_graph_activation",
            work_graph_activation_surface_ids(),
            "operator disables WorkGraph live activation",
        ),
        kill_switch(
            "kill_external_delivery_activation",
            vec!["external_delivery_activation"],
            "external delivery readback or policy mismatch is observed",
        ),
        kill_switch(
            "kill_scheduler_cutover_activation",
            vec!["scheduler_cutover_activation"],
            "lane lease, heartbeat, or backpressure evidence fails",
        ),
        kill_switch(
            "kill_adapter_enforcement_activation",
            vec!["adapter_projection_enforcement_activation"],
            "shadow adapter readback diverges from preview projection",
        ),
    ]
}

pub fn work_graph_activation_durable_identity_evidence()
-> WorkGraphActivationDurableIdentityEvidencePreview {
    WorkGraphActivationDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: vec![
            "workflow_id",
            "run_id",
            "step_id",
            "checkpoint",
            "replay_key",
            "rollback_anchor",
            "receipt_hash",
        ],
        required_for_surface_ids: work_graph_activation_surface_ids(),
        currently_satisfied: false,
    }
}

pub fn work_graph_activation_enforcement_invariants()
-> Vec<WorkGraphActivationEnforcementInvariantPreview> {
    vec![
        invariant(
            "activation_requires_durable_identity_evidence",
            "every activation surface requires durable workflow, replay, rollback, and receipt hash evidence first",
        ),
        invariant(
            "activation_is_blocked_by_default",
            "every live or persistent surface remains blocked until explicit enablement evidence exists",
        ),
        invariant(
            "feature_flag_and_operator_packet_required",
            "a feature flag alone cannot authorize high-risk activation without operator evidence",
        ),
        invariant(
            "shadow_readback_precedes_enforcement",
            "state persistence and adapter enforcement require shadow readback match evidence",
        ),
        invariant(
            "external_delivery_has_separate_policy",
            "external delivery cannot inherit approval from state promotion or scheduler cutover",
        ),
        invariant(
            "kill_switches_are_defined_before_activation",
            "every activation class has a rollback or kill-switch path before future execution",
        ),
        invariant(
            "activation_blocker_preview_has_no_side_effects",
            "this gate cannot enable enforcement, write state, cut over schedulers, or send externally",
        ),
    ]
}

impl WorkGraphActivationEnforcementBlockerPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            activation_performed: false,
            enforcement_enabled: false,
            store_persistence_enabled: false,
            replay_execution_enabled: false,
            promotion_execution_enabled: false,
            scheduler_cutover_performed: false,
            adapter_projection_enforced: false,
            approval_recorded: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn surface(
    id: &'static str,
    risk_class: &'static str,
    required_blocker_ids: Vec<&'static str>,
) -> WorkGraphActivationSurfacePreview {
    WorkGraphActivationSurfacePreview {
        id,
        risk_class,
        blocked_by_default: true,
        required_blocker_ids,
    }
}

fn blocker(
    id: &'static str,
    applies_to_surface_ids: Vec<&'static str>,
    denial_reason: &'static str,
) -> WorkGraphActivationBlockerPreview {
    WorkGraphActivationBlockerPreview {
        id,
        applies_to_surface_ids,
        denial_reason,
        blocks_activation: true,
    }
}

fn enablement(
    id: &'static str,
    required_for_surface_ids: Vec<&'static str>,
    required_evidence_fields: Vec<&'static str>,
) -> WorkGraphActivationEnablementPreview {
    WorkGraphActivationEnablementPreview {
        id,
        required_for_surface_ids,
        required_evidence_fields,
        currently_satisfied: false,
    }
}

fn kill_switch(
    id: &'static str,
    target_surface_ids: Vec<&'static str>,
    trigger: &'static str,
) -> WorkGraphActivationKillSwitchPreview {
    WorkGraphActivationKillSwitchPreview {
        id,
        target_surface_ids,
        trigger,
        armed_in_preview: true,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphActivationEnforcementInvariantPreview {
    WorkGraphActivationEnforcementInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activation_enforcement_blocker_preview_declares_surfaces() {
        let report = hepta_work_graph_activation_enforcement_blocker_preview_report();
        let surface_ids = report
            .activation_surfaces
            .iter()
            .map(|surface| surface.id)
            .collect::<Vec<_>>();

        assert_eq!(
            surface_ids,
            [
                "store_persistence_activation",
                "wal_replay_execution_activation",
                "promotion_execution_activation",
                "scheduler_cutover_activation",
                "adapter_projection_enforcement_activation",
                "approval_recording_activation",
                "external_delivery_activation",
                "operator_dashboard_publication_activation",
            ]
        );
        assert_eq!(report.activation_surface_count, 8);
        assert!(
            report
                .activation_surfaces
                .iter()
                .all(|surface| surface.blocked_by_default)
        );
    }

    #[test]
    fn activation_enforcement_blocker_preview_blocks_every_surface() {
        let report = hepta_work_graph_activation_enforcement_blocker_preview_report();
        let blocker_ids = report
            .blockers
            .iter()
            .map(|blocker| blocker.id)
            .collect::<Vec<_>>();

        assert_eq!(report.blocker_count, 15);
        assert!(blocker_ids.contains(&"durable_identity_evidence_missing"));
        assert!(blocker_ids.contains(&"feature_flag_not_enabled"));
        assert!(blocker_ids.contains(&"external_delivery_policy_missing"));
        assert!(
            report
                .blockers
                .iter()
                .all(|blocker| blocker.blocks_activation)
        );
    }

    #[test]
    fn activation_enforcement_blocker_preview_requires_unsatisfied_enablements() {
        let report = hepta_work_graph_activation_enforcement_blocker_preview_report();
        let enablement_ids = report
            .required_enablements
            .iter()
            .map(|enablement| enablement.id)
            .collect::<Vec<_>>();

        assert_eq!(
            enablement_ids,
            [
                "durable_identity_evidence_packet",
                "explicit_feature_flag",
                "operator_activation_packet",
                "shadow_readback_match",
                "rollback_quarantine_plan",
                "external_scope_policy",
                "redaction_review_packet",
            ]
        );
        assert_eq!(report.required_enablement_count, 7);
        assert_eq!(
            report.durable_identity_evidence.required_field_ids,
            [
                "workflow_id",
                "run_id",
                "step_id",
                "checkpoint",
                "replay_key",
                "rollback_anchor",
                "receipt_hash",
            ]
        );
        assert_eq!(
            report.durable_identity_evidence.required_for_surface_ids,
            work_graph_activation_surface_ids()
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert!(
            report
                .required_enablements
                .iter()
                .all(|enablement| !enablement.currently_satisfied)
        );
    }

    #[test]
    fn activation_enforcement_blocker_preview_keeps_execution_disabled() {
        let report = hepta_work_graph_activation_enforcement_blocker_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphActivationEnforcementBlockerPreviewSideEffects::none()
        );
        assert!(report.ready_for_shadow_adapter_readback_preview);
        assert!(!report.ready_for_activation);
        assert!(!report.ready_for_live_execution);
        assert_eq!(report.kill_switch_count, 4);
        assert_eq!(report.invariant_count, 7);
    }

    #[test]
    fn activation_enforcement_blocker_preview_requires_prior_gates() {
        let report = hepta_work_graph_activation_enforcement_blocker_preview_report();

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
                "hepta_work_graph_promotion_precondition_preview_gate",
                "hepta_work_graph_durable_identity_preview_gate",
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_ACTIVATION_ENFORCEMENT_BLOCKER_RECOMMENDED_NEXT_GATE
        );
    }
}
