use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8TerminalDecisionPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: String,
    pub schema_version: String,
    pub preview_mode: &'static str,
    pub terminal_decision_surface_count: usize,
    pub non_promotion_denial_count: usize,
    pub authority_guard_count: usize,
    pub release_delivery_guard_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<String>,
    pub terminal_decision_surfaces: Vec<DeepTd8TerminalDecisionSurface>,
    pub non_promotion_denials: Vec<DeepTd8TerminalDecisionDenial>,
    pub authority_guards: Vec<DeepTd8TerminalDecisionAuthorityGuard>,
    pub release_delivery_guards: Vec<DeepTd8TerminalDecisionReleaseDeliveryGuard>,
    pub local_views: Vec<DeepTd8TerminalDecisionLocalView>,
    pub invariants: Vec<DeepTd8TerminalDecisionInvariant>,
    pub recommended_next_gate: String,
    pub ready_for_terminal_decision_receipt_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: DeepTd8TerminalDecisionSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8TerminalDecisionSurface {
    pub id: &'static str,
    pub audience: &'static str,
    pub source_replay_scenario_ids: Vec<&'static str>,
    pub decision_visibility: &'static str,
    pub decision_recording_allowed: bool,
    pub promotion_allowed: bool,
    pub authority_grant_allowed: bool,
    pub public_claim_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8TerminalDecisionDenial {
    pub id: &'static str,
    pub applies_to_surface_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_persistence_promotion: bool,
    pub blocks_authority_grant: bool,
    pub blocks_rollout: bool,
    pub blocks_release_publication: bool,
    pub blocks_public_claim: bool,
    pub blocks_external_delivery: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8TerminalDecisionAuthorityGuard {
    pub id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub authority_grant_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8TerminalDecisionReleaseDeliveryGuard {
    pub id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub release_publication_allowed: bool,
    pub public_claim_allowed: bool,
    pub delivery_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8TerminalDecisionLocalView {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8TerminalDecisionInvariant {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DeepTd8TerminalDecisionSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_receipt_recorded: bool,
    pub acknowledgement_recorded: bool,
    pub operator_acceptance_recorded: bool,
    pub approval_recorded: bool,
    pub authority_granted: bool,
    pub live_persistence_enabled: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub rollout_started: bool,
    pub release_published: bool,
    pub public_claim_recorded: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn deep_td8_base() -> String {
    let gate = crate::deep_td7_receipt_retention_readback_ack_terminal_decision_gate();
    gate.strip_suffix("_preview_gate")
        .unwrap_or(gate.as_str())
        .to_owned()
}

pub fn deep_td8_terminal_decision_gate() -> String {
    crate::deep_td7_receipt_retention_readback_ack_terminal_decision_gate()
}

pub fn deep_td8_terminal_decision_receipt_gate() -> String {
    format!("{}_receipt_preview_gate", deep_td8_base())
}

pub fn deep_td8_schema_for(gate: &str) -> String {
    let without_prefix = gate.strip_prefix("hepta_").unwrap_or(gate);
    let without_suffix = without_prefix
        .strip_suffix("_gate")
        .unwrap_or(without_prefix);
    format!("{without_suffix}_v1")
}

pub fn hepta_work_graph_deep_td8_terminal_decision_preview_report()
-> DeepTd8TerminalDecisionPreviewReport {
    let terminal_decision_surfaces = deep_td8_terminal_decision_surfaces();
    let non_promotion_denials = deep_td8_terminal_decision_non_promotion_denials();
    let authority_guards = deep_td8_terminal_decision_authority_guards();
    let release_delivery_guards = deep_td8_terminal_decision_release_delivery_guards();
    let local_views = deep_td8_terminal_decision_local_views();
    let invariants = deep_td8_terminal_decision_invariants();
    let gate = deep_td8_terminal_decision_gate();

    DeepTd8TerminalDecisionPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        schema_version: deep_td8_schema_for(&gate),
        gate,
        preview_mode: "read_only_deep_td8_terminal_decision_non_promotion_preview_no_promotion",
        terminal_decision_surface_count: terminal_decision_surfaces.len(),
        non_promotion_denial_count: non_promotion_denials.len(),
        authority_guard_count: authority_guards.len(),
        release_delivery_guard_count: release_delivery_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates: deep_td8_terminal_decision_required_prior_gates(),
        terminal_decision_surfaces,
        non_promotion_denials,
        authority_guards,
        release_delivery_guards,
        local_views,
        invariants,
        recommended_next_gate: deep_td8_terminal_decision_receipt_gate(),
        ready_for_terminal_decision_receipt_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: DeepTd8TerminalDecisionSideEffects::none(),
    }
}

pub fn deep_td8_terminal_decision_required_prior_gates() -> Vec<String> {
    let mut gates = crate::deep_td7_receipt_retention_readback_ack_replay_required_prior_gates();
    gates.push(crate::deep_td7_receipt_retention_readback_ack_replay_gate());
    gates
}

pub fn deep_td8_terminal_decision_surface_ids() -> Vec<&'static str> {
    vec![
        "operator_deep_td8_terminal_decision_visibility",
        "release_owner_deep_td8_terminal_decision_visibility",
        "auditor_deep_td8_terminal_decision_visibility",
        "rollback_owner_deep_td8_terminal_decision_visibility",
        "runtime_deep_td8_terminal_decision_summary_visibility",
        "external_delivery_deep_td8_terminal_decision_echo",
    ]
}

pub fn deep_td8_source_replay_scenario_ids() -> Vec<&'static str> {
    vec![
        "duplicate_deep_td7_readback_receipt_replay",
        "duplicate_deep_td7_readback_ack_replay",
        "stale_deep_td7_readback_digest_replay",
        "superseded_deep_td7_readback_scope_replay",
        "cross_scope_deep_td7_readback_ack_replay",
        "out_of_order_deep_td7_readback_ack_replay",
    ]
}

pub fn deep_td8_terminal_decision_surfaces() -> Vec<DeepTd8TerminalDecisionSurface> {
    let replay_ids = deep_td8_source_replay_scenario_ids();
    vec![
        terminal_surface(
            "operator_deep_td8_terminal_decision_visibility",
            "operator",
            replay_ids.clone(),
            "local_operator_deep_td8_terminal_decision_read_only",
        ),
        terminal_surface(
            "release_owner_deep_td8_terminal_decision_visibility",
            "release_owner",
            replay_ids.clone(),
            "local_release_owner_deep_td8_terminal_decision_read_only",
        ),
        terminal_surface(
            "auditor_deep_td8_terminal_decision_visibility",
            "auditor",
            replay_ids.clone(),
            "local_auditor_deep_td8_terminal_decision_read_only",
        ),
        terminal_surface(
            "rollback_owner_deep_td8_terminal_decision_visibility",
            "rollback_owner",
            replay_ids.clone(),
            "local_rollback_owner_deep_td8_terminal_decision_read_only",
        ),
        terminal_surface(
            "runtime_deep_td8_terminal_decision_summary_visibility",
            "system",
            replay_ids.clone(),
            "local_runtime_deep_td8_terminal_decision_summary_read_only",
        ),
        terminal_surface(
            "external_delivery_deep_td8_terminal_decision_echo",
            "external_delivery",
            replay_ids,
            "external_delivery_echo_denied",
        ),
    ]
}

pub fn deep_td8_terminal_decision_non_promotion_denials() -> Vec<DeepTd8TerminalDecisionDenial> {
    let surface_ids = deep_td8_terminal_decision_surface_ids();
    vec![
        terminal_denial(
            "deep_td8_terminal_decision_cannot_promote_persistence",
            surface_ids.clone(),
            "terminal decision visibility after td7 receipt retention readback acknowledgement replay cannot promote persistence, WAL, or checkpoints",
        ),
        terminal_denial(
            "deep_td8_terminal_decision_cannot_grant_authority",
            surface_ids.clone(),
            "terminal decision visibility after td7 receipt retention readback acknowledgement replay cannot grant acceptance or approval authority",
        ),
        terminal_denial(
            "deep_td8_release_owner_terminal_decision_cannot_publish",
            surface_ids.clone(),
            "release-owner terminal decision visibility cannot publish release state or public claims",
        ),
        terminal_denial(
            "deep_td8_operator_terminal_decision_cannot_start_rollout",
            surface_ids.clone(),
            "operator terminal decision visibility cannot start rollout or route traffic",
        ),
        terminal_denial(
            "deep_td8_auditor_terminal_decision_cannot_record_approval",
            surface_ids.clone(),
            "auditor terminal decision visibility cannot record approval",
        ),
        terminal_denial(
            "deep_td8_rollback_owner_terminal_decision_cannot_mutate_quarantine",
            surface_ids.clone(),
            "rollback-owner terminal decision visibility cannot mutate rollback or quarantine state",
        ),
        terminal_denial(
            "deep_td8_external_delivery_terminal_decision_cannot_send",
            surface_ids.clone(),
            "external delivery echo of terminal decision must stay denied",
        ),
        terminal_denial(
            "deep_td8_terminal_decision_summary_cannot_claim_live_completion",
            surface_ids,
            "terminal decision summary cannot claim live persistence completion",
        ),
    ]
}

pub fn deep_td8_terminal_decision_authority_guards() -> Vec<DeepTd8TerminalDecisionAuthorityGuard> {
    vec![
        authority_guard(
            "deep_td8_accepted_record_authority_absent",
            vec![
                "acceptedRecordId",
                "acceptedRecordHash",
                "authoritySignature",
            ],
        ),
        authority_guard(
            "deep_td8_approval_recording_authority_absent",
            vec!["approvalRecordId", "approvalScope", "approvalSignature"],
        ),
        authority_guard(
            "deep_td8_operator_enablement_authority_absent",
            vec![
                "operatorEnablementPacketId",
                "enablementScope",
                "enablementSignature",
            ],
        ),
        authority_guard(
            "deep_td8_release_owner_authority_absent",
            vec!["releaseOwnerId", "releaseScope", "releaseSignature"],
        ),
        authority_guard(
            "deep_td8_live_persistence_authority_absent",
            vec![
                "livePersistenceFlag",
                "walAuthorityHash",
                "checkpointAuthorityHash",
            ],
        ),
        authority_guard(
            "deep_td8_external_delivery_authority_absent",
            vec![
                "externalDeliveryPolicyId",
                "deliveryScope",
                "deliverySignature",
            ],
        ),
    ]
}

pub fn deep_td8_terminal_decision_release_delivery_guards()
-> Vec<DeepTd8TerminalDecisionReleaseDeliveryGuard> {
    vec![
        release_delivery_guard(
            "deep_td8_release_publication_gate_denied",
            vec![
                "releasePublicationRequest",
                "releaseOwnerSignature",
                "releaseHash",
            ],
        ),
        release_delivery_guard(
            "deep_td8_public_claim_gate_denied",
            vec!["publicClaimRequest", "claimAudience", "claimHash"],
        ),
        release_delivery_guard(
            "deep_td8_traffic_ramp_gate_zero",
            vec!["trafficRampRequest", "trafficPercent", "killSwitchState"],
        ),
        release_delivery_guard(
            "deep_td8_external_delivery_gate_denied",
            vec![
                "externalDeliveryRequest",
                "destinationPolicy",
                "deliveryHash",
            ],
        ),
        release_delivery_guard(
            "deep_td8_artifact_publication_gate_denied",
            vec![
                "artifactPublicationRequest",
                "artifactHash",
                "redactionHash",
            ],
        ),
        release_delivery_guard(
            "deep_td8_completion_claim_local_only",
            vec!["terminalDecisionId", "completionClaimHash", "localViewHash"],
        ),
    ]
}

pub fn deep_td8_terminal_decision_local_views() -> Vec<DeepTd8TerminalDecisionLocalView> {
    vec![
        local_view(
            "operator_deep_td8_terminal_decision_non_promotion_view",
            "operator",
            vec![
                "terminalDecisionSurfaceId",
                "promotionAllowed",
                "authorityGranted",
                "nextGate",
            ],
        ),
        local_view(
            "release_owner_deep_td8_release_denial_view",
            "release_owner",
            vec![
                "releasePublished",
                "publicClaimRecorded",
                "trafficRouted",
                "externalDeliveryDenied",
            ],
        ),
        local_view(
            "auditor_deep_td8_digest_denial_view",
            "auditor",
            vec![
                "priorReplayGateDigest",
                "terminalDecisionHash",
                "authorityGuardId",
                "denialId",
            ],
        ),
        local_view(
            "runtime_deep_td8_zero_effect_view",
            "system",
            vec![
                "terminalDecisionRecorded",
                "livePersistenceEnabled",
                "releasePublished",
                "externalSendPerformed",
            ],
        ),
    ]
}

pub fn deep_td8_terminal_decision_invariants() -> Vec<DeepTd8TerminalDecisionInvariant> {
    vec![
        invariant(
            "deep_td8_terminal_decision_visibility_is_not_promotion",
            "terminal decision visibility cannot promote persistence",
        ),
        invariant(
            "deep_td8_terminal_decision_visibility_is_not_authority",
            "terminal decision visibility cannot grant acceptance, approval, or delivery authority",
        ),
        invariant(
            "deep_td8_terminal_decision_requires_td7_replay_gate",
            "terminal decision visibility requires td7 readback acknowledgement replay idempotency evidence first",
        ),
        invariant(
            "deep_td8_terminal_decision_keeps_release_and_rollout_denied",
            "release publication, public claim, rollout, and traffic routing remain denied",
        ),
        invariant(
            "deep_td8_terminal_decision_views_are_local_only",
            "operator, auditor, release-owner, rollback-owner, and runtime views cannot be sent externally",
        ),
        invariant(
            "deep_td8_terminal_decision_preview_has_no_side_effects",
            "this gate cannot record terminal decisions, persist state, grant authority, publish, or send externally",
        ),
    ]
}

fn terminal_surface(
    id: &'static str,
    audience: &'static str,
    source_replay_scenario_ids: Vec<&'static str>,
    decision_visibility: &'static str,
) -> DeepTd8TerminalDecisionSurface {
    DeepTd8TerminalDecisionSurface {
        id,
        audience,
        source_replay_scenario_ids,
        decision_visibility,
        decision_recording_allowed: false,
        promotion_allowed: false,
        authority_grant_allowed: false,
        public_claim_enabled: false,
        external_delivery_enabled: false,
    }
}

fn terminal_denial(
    id: &'static str,
    applies_to_surface_ids: Vec<&'static str>,
    reason: &'static str,
) -> DeepTd8TerminalDecisionDenial {
    DeepTd8TerminalDecisionDenial {
        id,
        applies_to_surface_ids,
        reason,
        blocks_persistence_promotion: true,
        blocks_authority_grant: true,
        blocks_rollout: true,
        blocks_release_publication: true,
        blocks_public_claim: true,
        blocks_external_delivery: true,
    }
}

fn authority_guard(
    id: &'static str,
    required_fields: Vec<&'static str>,
) -> DeepTd8TerminalDecisionAuthorityGuard {
    DeepTd8TerminalDecisionAuthorityGuard {
        id,
        required_fields,
        authority_grant_allowed: false,
    }
}

fn release_delivery_guard(
    id: &'static str,
    required_fields: Vec<&'static str>,
) -> DeepTd8TerminalDecisionReleaseDeliveryGuard {
    DeepTd8TerminalDecisionReleaseDeliveryGuard {
        id,
        required_fields,
        release_publication_allowed: false,
        public_claim_allowed: false,
        delivery_allowed: false,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> DeepTd8TerminalDecisionLocalView {
    DeepTd8TerminalDecisionLocalView {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(id: &'static str, reason: &'static str) -> DeepTd8TerminalDecisionInvariant {
    DeepTd8TerminalDecisionInvariant {
        id,
        required: true,
        reason,
    }
}

impl DeepTd8TerminalDecisionSideEffects {
    pub fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            terminal_decision_receipt_recorded: false,
            acknowledgement_recorded: false,
            operator_acceptance_recorded: false,
            approval_recorded: false,
            authority_granted: false,
            live_persistence_enabled: false,
            wal_written: false,
            checkpoint_written: false,
            rollout_started: false,
            release_published: false,
            public_claim_recorded: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }

    #[cfg(test)]
    fn all_false(self) -> bool {
        !self.filesystem_written
            && !self.graph_state_persisted
            && !self.terminal_decision_recorded
            && !self.terminal_decision_receipt_recorded
            && !self.acknowledgement_recorded
            && !self.operator_acceptance_recorded
            && !self.approval_recorded
            && !self.authority_granted
            && !self.live_persistence_enabled
            && !self.wal_written
            && !self.checkpoint_written
            && !self.rollout_started
            && !self.release_published
            && !self.public_claim_recorded
            && !self.external_send_performed
            && !self.model_invoked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_graph_deep_td8_terminal_decision_requires_td7_replay_gate() {
        let report = hepta_work_graph_deep_td8_terminal_decision_preview_report();

        assert_eq!(
            report.required_prior_gates.last(),
            Some(&crate::deep_td7_receipt_retention_readback_ack_replay_gate())
        );
    }

    #[test]
    fn work_graph_deep_td8_terminal_decision_declares_local_surfaces() {
        let report = hepta_work_graph_deep_td8_terminal_decision_preview_report();

        assert_eq!(report.terminal_decision_surface_count, 6);
        assert!(report.terminal_decision_surfaces.iter().all(|surface| {
            surface.source_replay_scenario_ids.len() == 6
                && !surface.decision_recording_allowed
                && !surface.promotion_allowed
                && !surface.authority_grant_allowed
                && !surface.public_claim_enabled
                && !surface.external_delivery_enabled
        }));
    }

    #[test]
    fn work_graph_deep_td8_terminal_decision_blocks_every_promotion_path() {
        let report = hepta_work_graph_deep_td8_terminal_decision_preview_report();

        assert_eq!(report.non_promotion_denial_count, 8);
        assert!(report.non_promotion_denials.iter().all(|denial| {
            denial.applies_to_surface_ids.len() == 6
                && denial.blocks_persistence_promotion
                && denial.blocks_authority_grant
                && denial.blocks_rollout
                && denial.blocks_release_publication
                && denial.blocks_public_claim
                && denial.blocks_external_delivery
        }));
    }

    #[test]
    fn work_graph_deep_td8_terminal_decision_requires_missing_authority() {
        let report = hepta_work_graph_deep_td8_terminal_decision_preview_report();

        assert_eq!(report.authority_guard_count, 6);
        assert!(
            report
                .authority_guards
                .iter()
                .all(|guard| !guard.authority_grant_allowed && guard.required_fields.len() >= 3)
        );
    }

    #[test]
    fn work_graph_deep_td8_terminal_decision_keeps_release_delivery_denied() {
        let report = hepta_work_graph_deep_td8_terminal_decision_preview_report();

        assert_eq!(report.release_delivery_guard_count, 6);
        assert!(report.release_delivery_guards.iter().all(|guard| {
            !guard.release_publication_allowed
                && !guard.public_claim_allowed
                && !guard.delivery_allowed
        }));
    }

    #[test]
    fn work_graph_deep_td8_terminal_decision_has_no_side_effects_and_points_to_receipt() {
        let report = hepta_work_graph_deep_td8_terminal_decision_preview_report();

        assert_eq!(report.local_view_count, 4);
        assert_eq!(report.invariant_count, 6);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
        assert_eq!(
            report.recommended_next_gate,
            deep_td8_terminal_decision_receipt_gate()
        );
        assert!(report.ready_for_terminal_decision_receipt_preview);
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert!(report.side_effects.all_false());
    }
}
