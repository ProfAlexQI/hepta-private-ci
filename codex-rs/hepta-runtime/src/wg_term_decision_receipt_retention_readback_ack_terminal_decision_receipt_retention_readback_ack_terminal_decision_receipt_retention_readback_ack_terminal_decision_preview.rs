use serde::Serialize;

use crate::WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionAuthorityGuardPreview;
use crate::WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionDenialPreview;
use crate::WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionInvariantPreview;
use crate::WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionLocalViewPreview;
use crate::WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReleaseDeliveryGuardPreview;
use crate::WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionSurfacePreview;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_PREVIEW_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_SCHEMA_VERSION:
    &str = "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub terminal_decision_surface_count: usize,
    pub non_promotion_denial_count: usize,
    pub authority_guard_count: usize,
    pub release_delivery_guard_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub terminal_decision_surfaces:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionSurfacePreview>,
    pub non_promotion_denials:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionDenialPreview>,
    pub authority_guards:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionAuthorityGuardPreview>,
    pub release_delivery_guards: Vec<
        WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReleaseDeliveryGuardPreview,
    >,
    pub local_views:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionLocalViewPreview>,
    pub invariants:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview:
        bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionSideEffects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_receipt_recorded: bool,
    pub terminal_decision_receipt_retention_state_persisted: bool,
    pub readback_receipt_persisted: bool,
    pub readback_acknowledgement_recorded: bool,
    pub readback_acknowledgement_replay_recorded: bool,
    pub operator_acceptance_recorded: bool,
    pub approval_recorded: bool,
    pub authority_granted: bool,
    pub live_persistence_enabled: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub enforcement_enabled: bool,
    pub rollout_started: bool,
    pub traffic_routed: bool,
    pub release_published: bool,
    pub public_claim_recorded: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_report()
-> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionPreviewReport{
    let terminal_decision_surfaces =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_surfaces();
    let non_promotion_denials =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_non_promotion_denials();
    let authority_guards =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_authority_guards();
    let release_delivery_guards =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_release_delivery_guards();
    let local_views =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_local_views();
    let invariants =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_invariants();

    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_SCHEMA_VERSION,
        preview_mode: "read_only_terminal_decision_receipt_retention_readback_acknowledgement_terminal_decision_non_promotion_preview_no_promotion",
        terminal_decision_surface_count: terminal_decision_surfaces.len(),
        non_promotion_denial_count: non_promotion_denials.len(),
        authority_guard_count: authority_guards.len(),
        release_delivery_guard_count: release_delivery_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_required_prior_gates(),
        terminal_decision_surfaces,
        non_promotion_denials,
        authority_guards,
        release_delivery_guards,
        local_views,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionSideEffects::none(),
    }
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = crate::work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_required_prior_gates();
    gates.push("hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate");
    gates
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_surface_ids()
-> Vec<&'static str> {
    vec![
        "operator_terminal_decision_receipt_retention_readback_ack_decision_visibility",
        "release_owner_terminal_decision_receipt_retention_readback_ack_decision_visibility",
        "auditor_terminal_decision_receipt_retention_readback_ack_decision_visibility",
        "rollback_owner_terminal_decision_receipt_retention_readback_ack_decision_visibility",
        "runtime_terminal_decision_receipt_retention_readback_ack_summary_visibility",
        "external_delivery_terminal_decision_receipt_retention_readback_ack_echo",
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_surfaces()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionSurfacePreview> {
    let replay_ids =
        crate::work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_scenario_ids();
    vec![
        terminal_decision_surface(
            "operator_terminal_decision_receipt_retention_readback_ack_decision_visibility",
            "operator",
            replay_ids.clone(),
            "local_operator_terminal_decision_receipt_retention_readback_ack_decision_read_only",
        ),
        terminal_decision_surface(
            "release_owner_terminal_decision_receipt_retention_readback_ack_decision_visibility",
            "release_owner",
            replay_ids.clone(),
            "local_release_owner_terminal_decision_receipt_retention_readback_ack_decision_read_only",
        ),
        terminal_decision_surface(
            "auditor_terminal_decision_receipt_retention_readback_ack_decision_visibility",
            "auditor",
            replay_ids.clone(),
            "local_auditor_terminal_decision_receipt_retention_readback_ack_decision_read_only",
        ),
        terminal_decision_surface(
            "rollback_owner_terminal_decision_receipt_retention_readback_ack_decision_visibility",
            "rollback_owner",
            replay_ids.clone(),
            "local_rollback_owner_terminal_decision_receipt_retention_readback_ack_decision_read_only",
        ),
        terminal_decision_surface(
            "runtime_terminal_decision_receipt_retention_readback_ack_summary_visibility",
            "system",
            replay_ids.clone(),
            "local_runtime_terminal_decision_receipt_retention_readback_ack_summary_read_only",
        ),
        terminal_decision_surface(
            "external_delivery_terminal_decision_receipt_retention_readback_ack_echo",
            "external_delivery",
            replay_ids,
            "external_delivery_echo_denied",
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_non_promotion_denials()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionDenialPreview> {
    let surface_ids =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_surface_ids();
    vec![
        non_promotion_denial(
            "terminal_decision_receipt_retention_readback_ack_decision_cannot_promote_persistence",
            surface_ids.clone(),
            "terminal decision visibility after terminal decision receipt retention readback acknowledgement replay cannot promote persistence, WAL, or checkpoints",
        ),
        non_promotion_denial(
            "terminal_decision_receipt_retention_readback_ack_decision_cannot_grant_authority",
            surface_ids.clone(),
            "terminal decision visibility after terminal decision receipt retention readback acknowledgement replay cannot grant acceptance or approval authority",
        ),
        non_promotion_denial(
            "release_owner_terminal_decision_receipt_retention_readback_ack_cannot_publish",
            surface_ids.clone(),
            "release-owner terminal decision visibility cannot publish release state or public claims",
        ),
        non_promotion_denial(
            "operator_terminal_decision_receipt_retention_readback_ack_cannot_start_rollout",
            surface_ids.clone(),
            "operator terminal decision visibility cannot start rollout or route traffic",
        ),
        non_promotion_denial(
            "auditor_terminal_decision_receipt_retention_readback_ack_cannot_record_approval",
            surface_ids.clone(),
            "auditor terminal decision visibility cannot record approval",
        ),
        non_promotion_denial(
            "rollback_owner_terminal_decision_receipt_retention_readback_ack_cannot_mutate_quarantine",
            surface_ids.clone(),
            "rollback-owner terminal decision visibility cannot mutate rollback or quarantine state",
        ),
        non_promotion_denial(
            "external_delivery_terminal_decision_receipt_retention_readback_ack_cannot_send",
            surface_ids.clone(),
            "external delivery echo of terminal decision receipt retention readback acknowledgement decision must stay denied",
        ),
        non_promotion_denial(
            "terminal_decision_receipt_retention_readback_ack_summary_cannot_claim_live_completion",
            surface_ids,
            "terminal decision summary cannot claim live persistence completion",
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_authority_guards()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionAuthorityGuardPreview> {
    vec![
        authority_guard(
            "accepted_record_authority_absent_after_terminal_decision_receipt_retention_readback_ack",
            vec![
                "acceptedRecordId",
                "acceptedRecordHash",
                "authoritySignature",
            ],
        ),
        authority_guard(
            "approval_recording_authority_absent_after_terminal_decision_receipt_retention_readback_ack",
            vec!["approvalRecordId", "approvalScope", "approvalSignature"],
        ),
        authority_guard(
            "operator_enablement_authority_absent_after_terminal_decision_receipt_retention_readback_ack",
            vec![
                "operatorEnablementPacketId",
                "enablementScope",
                "enablementSignature",
            ],
        ),
        authority_guard(
            "release_owner_authority_absent_after_terminal_decision_receipt_retention_readback_ack",
            vec!["releaseOwnerId", "releaseScope", "releaseSignature"],
        ),
        authority_guard(
            "live_persistence_authority_absent_after_terminal_decision_receipt_retention_readback_ack",
            vec![
                "livePersistenceFlag",
                "walAuthorityHash",
                "checkpointAuthorityHash",
            ],
        ),
        authority_guard(
            "external_delivery_authority_absent_after_terminal_decision_receipt_retention_readback_ack",
            vec![
                "externalDeliveryPolicyId",
                "deliveryScope",
                "deliverySignature",
            ],
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_release_delivery_guards()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReleaseDeliveryGuardPreview> {
    vec![
        release_delivery_guard(
            "release_publication_gate_denied_after_terminal_decision_receipt_retention_readback_ack",
            vec![
                "releasePublicationRequest",
                "releaseOwnerSignature",
                "releaseHash",
            ],
        ),
        release_delivery_guard(
            "public_claim_gate_denied_after_terminal_decision_receipt_retention_readback_ack",
            vec!["publicClaimRequest", "claimAudience", "claimHash"],
        ),
        release_delivery_guard(
            "traffic_ramp_gate_zero_after_terminal_decision_receipt_retention_readback_ack",
            vec!["trafficRampRequest", "trafficPercent", "killSwitchState"],
        ),
        release_delivery_guard(
            "external_delivery_gate_denied_after_terminal_decision_receipt_retention_readback_ack",
            vec![
                "externalDeliveryRequest",
                "destinationPolicy",
                "deliveryHash",
            ],
        ),
        release_delivery_guard(
            "artifact_publication_gate_denied_after_terminal_decision_receipt_retention_readback_ack",
            vec![
                "artifactPublicationRequest",
                "artifactHash",
                "redactionHash",
            ],
        ),
        release_delivery_guard(
            "terminal_decision_receipt_retention_readback_ack_completion_claim_local_only",
            vec!["terminalDecisionId", "completionClaimHash", "localViewHash"],
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_local_views()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionLocalViewPreview> {
    vec![
        local_view(
            "operator_terminal_decision_receipt_retention_readback_ack_non_promotion_view",
            "operator",
            vec![
                "terminalDecisionSurfaceId",
                "promotionAllowed",
                "authorityGranted",
                "nextGate",
            ],
        ),
        local_view(
            "release_owner_terminal_decision_receipt_retention_readback_ack_release_denial_view",
            "release_owner",
            vec![
                "releasePublished",
                "publicClaimRecorded",
                "trafficRouted",
                "externalDeliveryDenied",
            ],
        ),
        local_view(
            "auditor_terminal_decision_receipt_retention_readback_ack_digest_denial_view",
            "auditor",
            vec![
                "priorReplayGateDigest",
                "terminalDecisionHash",
                "authorityGuardId",
                "denialId",
            ],
        ),
        local_view(
            "runtime_terminal_decision_receipt_retention_readback_ack_zero_effect_view",
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

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_invariants()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionInvariantPreview> {
    vec![
        invariant(
            "terminal_decision_receipt_retention_readback_ack_visibility_is_not_promotion",
            "terminal decision visibility cannot promote persistence",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_visibility_is_not_authority",
            "terminal decision visibility cannot grant acceptance, approval, or delivery authority",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_requires_replay_idempotency_gate",
            "terminal decision visibility requires terminal decision receipt retention readback acknowledgement replay idempotency evidence first",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_keeps_release_and_rollout_denied",
            "release publication, public claim, rollout, and traffic routing remain denied",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_views_are_local_only",
            "operator, auditor, release-owner, rollback-owner, and runtime views cannot be sent externally",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_preview_has_no_side_effects",
            "this gate cannot record terminal decisions, persist state, grant authority, publish, or send externally",
        ),
    ]
}

fn terminal_decision_surface(
    id: &'static str,
    audience: &'static str,
    source_replay_scenario_ids: Vec<&'static str>,
    decision_visibility: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionSurfacePreview {
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionSurfacePreview {
        id,
        audience,
        source_replay_scenario_ids,
        decision_visibility,
        required_fields: vec![
            "workflow_id",
            "run_id",
            "step_id",
            "checkpoint",
            "replay_key",
            "rollback_anchor",
            "receipt_hash",
            "terminalDecisionSurfaceId",
            "decisionVisibility",
            "sourceReplayScenarioIds",
            "nonPromotionProofHash",
        ],
        decision_recording_allowed: false,
        promotion_allowed: false,
        authority_grant_allowed: false,
        public_claim_enabled: false,
        external_delivery_enabled: false,
    }
}

fn non_promotion_denial(
    id: &'static str,
    applies_to_surface_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionDenialPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionDenialPreview {
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
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionAuthorityGuardPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionAuthorityGuardPreview {
        id,
        required_fields,
        authority_grant_allowed: false,
    }
}

fn release_delivery_guard(
    id: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReleaseDeliveryGuardPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReleaseDeliveryGuardPreview {
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
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionLocalViewPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionInvariantPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionInvariantPreview {
        id,
        required: true,
        reason,
    }
}

impl WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionSideEffects {
    pub fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            terminal_decision_receipt_recorded: false,
            terminal_decision_receipt_retention_state_persisted: false,
            readback_receipt_persisted: false,
            readback_acknowledgement_recorded: false,
            readback_acknowledgement_replay_recorded: false,
            operator_acceptance_recorded: false,
            approval_recorded: false,
            authority_granted: false,
            live_persistence_enabled: false,
            wal_written: false,
            checkpoint_written: false,
            enforcement_enabled: false,
            rollout_started: false,
            traffic_routed: false,
            release_published: false,
            public_claim_recorded: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_requires_replay_gate()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_report();

        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(
                "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate"
            )
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview);
    }

    #[test]
    fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_declares_local_surfaces()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_report();

        assert_eq!(report.terminal_decision_surface_count, 6);
        assert!(report.terminal_decision_surfaces.iter().all(|surface| {
            !surface.decision_recording_allowed
                && !surface.promotion_allowed
                && !surface.authority_grant_allowed
                && !surface.public_claim_enabled
                && !surface.external_delivery_enabled
                && surface.source_replay_scenario_ids.len() == 6
        }));
    }

    #[test]
    fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_blocks_every_promotion_path()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_report();

        assert_eq!(report.non_promotion_denial_count, 8);
        assert!(report.non_promotion_denials.iter().all(|denial| {
            denial.blocks_persistence_promotion
                && denial.blocks_authority_grant
                && denial.blocks_rollout
                && denial.blocks_release_publication
                && denial.blocks_public_claim
                && denial.blocks_external_delivery
                && denial.applies_to_surface_ids.len() == 6
        }));
    }

    #[test]
    fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_requires_missing_authority()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_report();

        assert_eq!(report.authority_guard_count, 6);
        assert!(
            report
                .authority_guards
                .iter()
                .all(|guard| !guard.authority_grant_allowed && guard.required_fields.len() >= 3)
        );
    }

    #[test]
    fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_keeps_release_delivery_denied()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_report();

        assert_eq!(report.release_delivery_guard_count, 6);
        assert!(report.release_delivery_guards.iter().all(|guard| {
            !guard.release_publication_allowed
                && !guard.public_claim_allowed
                && !guard.delivery_allowed
                && guard.required_fields.len() >= 3
        }));
        assert_eq!(report.local_view_count, 4);
        assert!(
            report
                .local_views
                .iter()
                .all(|view| !view.external_delivery_enabled && view.required_fields.len() >= 4)
        );
    }

    #[test]
    fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_has_no_side_effects()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_report();

        assert_eq!(report.invariant_count, 6);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert_eq!(
            report.side_effects,
            WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionSideEffects::none()
        );
    }
}
