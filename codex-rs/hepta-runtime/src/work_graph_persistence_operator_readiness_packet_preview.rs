use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_PACKET_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_operator_readiness_packet_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_PACKET_SCHEMA_VERSION: &str =
    "work_graph_persistence_operator_readiness_packet_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_PACKET_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_persistence_operator_readiness_receipt_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessPacketPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub packet_template_count: usize,
    pub packet_section_count: usize,
    pub validation_denial_count: usize,
    pub acceptance_guard_count: usize,
    pub expiry_revocation_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub packet_templates: Vec<WorkGraphPersistenceOperatorReadinessPacketTemplatePreview>,
    pub packet_sections: Vec<WorkGraphPersistenceOperatorReadinessSectionPreview>,
    pub validation_denials: Vec<WorkGraphPersistenceOperatorReadinessValidationDenialPreview>,
    pub acceptance_guards: Vec<WorkGraphPersistenceOperatorReadinessAcceptanceGuardPreview>,
    pub expiry_revocations: Vec<WorkGraphPersistenceOperatorReadinessExpiryRevocationPreview>,
    pub durable_identity_evidence:
        WorkGraphPersistenceOperatorReadinessDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphPersistenceOperatorReadinessPacketInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_operator_readiness_receipt_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: WorkGraphPersistenceOperatorReadinessPacketPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessPacketTemplatePreview {
    pub id: &'static str,
    pub target_rollout_stage_id: &'static str,
    pub required_section_ids: Vec<&'static str>,
    pub acceptance_allowed: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessSectionPreview {
    pub id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub redaction_state: &'static str,
    pub currently_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessValidationDenialPreview {
    pub id: &'static str,
    pub applies_to_section_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_acceptance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessAcceptanceGuardPreview {
    pub id: &'static str,
    pub applies_to_template_ids: Vec<&'static str>,
    pub required_evidence_fields: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessExpiryRevocationPreview {
    pub id: &'static str,
    pub applies_to_template_ids: Vec<&'static str>,
    pub trigger: &'static str,
    pub blocks_acceptance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessDurableIdentityEvidencePreview {
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_template_ids: Vec<&'static str>,
    pub required_section_id: &'static str,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessPacketInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessPacketPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub readiness_packet_persisted: bool,
    pub operator_acceptance_recorded: bool,
    pub approval_recorded: bool,
    pub enforcement_enabled: bool,
    pub rollout_started: bool,
    pub traffic_routed: bool,
    pub live_readback_executed: bool,
    pub release_published: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_persistence_operator_readiness_packet_preview_report()
-> WorkGraphPersistenceOperatorReadinessPacketPreviewReport {
    let packet_templates = work_graph_persistence_operator_readiness_packet_templates();
    let packet_sections = work_graph_persistence_operator_readiness_sections();
    let validation_denials = work_graph_persistence_operator_readiness_validation_denials();
    let acceptance_guards = work_graph_persistence_operator_readiness_acceptance_guards();
    let expiry_revocations = work_graph_persistence_operator_readiness_expiry_revocations();
    let durable_identity_evidence =
        work_graph_persistence_operator_readiness_durable_identity_evidence();
    let invariants = work_graph_persistence_operator_readiness_packet_invariants();

    WorkGraphPersistenceOperatorReadinessPacketPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_PACKET_PREVIEW_GATE,
        schema_version: WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_PACKET_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_operator_readiness_packet_preview_no_acceptance",
        packet_template_count: packet_templates.len(),
        packet_section_count: packet_sections.len(),
        validation_denial_count: validation_denials.len(),
        acceptance_guard_count: acceptance_guards.len(),
        expiry_revocation_count: expiry_revocations.len(),
        invariant_count: invariants.len(),
        required_prior_gates: work_graph_persistence_operator_readiness_packet_required_prior_gates(
        ),
        packet_templates,
        packet_sections,
        validation_denials,
        acceptance_guards,
        expiry_revocations,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_PACKET_RECOMMENDED_NEXT_GATE,
        ready_for_operator_readiness_receipt_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: WorkGraphPersistenceOperatorReadinessPacketPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_operator_readiness_packet_required_prior_gates() -> Vec<&'static str>
{
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
        "hepta_work_graph_activation_enforcement_blocker_preview_gate",
        "hepta_work_graph_shadow_adapter_readback_preview_gate",
        "hepta_work_graph_persistence_feature_flag_preview_gate",
        "hepta_work_graph_persistence_canary_dry_run_preview_gate",
        "hepta_work_graph_persistence_canary_readback_receipt_preview_gate",
        "hepta_work_graph_persistence_promotion_blocker_preview_gate",
        "hepta_work_graph_persistence_shadow_live_readback_comparison_preview_gate",
        "hepta_work_graph_persistence_enforcement_rollout_blocker_preview_gate",
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_operator_readiness_template_ids() -> Vec<&'static str> {
    vec![
        "store_persistence_readiness_packet",
        "wal_checkpoint_readiness_packet",
        "readback_receipt_readiness_packet",
        "replay_execution_readiness_packet",
        "external_publication_readiness_packet",
        "full_rollout_abort_readiness_packet",
    ]
}

pub fn work_graph_persistence_operator_readiness_durable_identity_field_ids() -> Vec<&'static str> {
    vec![
        "workflow_id",
        "run_id",
        "step_id",
        "checkpoint",
        "replay_key",
        "rollback_anchor",
        "receipt_hash",
    ]
}

pub fn work_graph_persistence_operator_readiness_durable_identity_section_id() -> &'static str {
    "durable_identity_section"
}

pub fn work_graph_persistence_operator_readiness_packet_templates()
-> Vec<WorkGraphPersistenceOperatorReadinessPacketTemplatePreview> {
    vec![
        packet_template(
            "store_persistence_readiness_packet",
            "store_persistence_enforcement_rollout",
            vec![
                "durable_identity_section",
                "operator_scope_section",
                "shadow_live_digest_section",
                "rollback_owner_section",
                "release_denial_section",
            ],
        ),
        packet_template(
            "wal_checkpoint_readiness_packet",
            "wal_append_enforcement_rollout",
            vec![
                "durable_identity_section",
                "operator_scope_section",
                "wal_checkpoint_schema_section",
                "traffic_ramp_blocker_section",
                "kill_switch_section",
            ],
        ),
        packet_template(
            "readback_receipt_readiness_packet",
            "readback_receipt_enforcement_rollout",
            vec![
                "durable_identity_section",
                "operator_scope_section",
                "receipt_redaction_section",
                "retention_expiry_section",
                "rollback_owner_section",
            ],
        ),
        packet_template(
            "replay_execution_readiness_packet",
            "replay_execution_enforcement_rollout",
            vec![
                "durable_identity_section",
                "operator_scope_section",
                "replay_drift_budget_section",
                "lane_lease_section",
                "kill_switch_section",
            ],
        ),
        packet_template(
            "external_publication_readiness_packet",
            "external_publication_enforcement_rollout",
            vec![
                "durable_identity_section",
                "operator_scope_section",
                "external_policy_section",
                "release_denial_section",
                "external_delivery_readback_section",
            ],
        ),
        packet_template(
            "full_rollout_abort_readiness_packet",
            "all_persistence_enforcement_rollouts",
            vec![
                "durable_identity_section",
                "abort_scope_section",
                "kill_switch_section",
                "rollback_owner_section",
                "retention_expiry_section",
            ],
        ),
    ]
}

pub fn work_graph_persistence_operator_readiness_sections()
-> Vec<WorkGraphPersistenceOperatorReadinessSectionPreview> {
    vec![
        section(
            work_graph_persistence_operator_readiness_durable_identity_section_id(),
            work_graph_persistence_operator_readiness_durable_identity_field_ids(),
        ),
        section(
            "operator_scope_section",
            vec!["operatorScopeHash", "operatorIdHash", "expiresAtUnixMs"],
        ),
        section(
            "shadow_live_digest_section",
            vec![
                "shadowDigestHash",
                "futureLiveProbeId",
                "mismatchClassifierIds",
            ],
        ),
        section(
            "rollback_owner_section",
            vec!["rollbackOwnerId", "quarantineScope", "killSwitchId"],
        ),
        section(
            "release_denial_section",
            vec!["releaseDenialIds", "publicationDenied", "targetSurfaceId"],
        ),
        section(
            "wal_checkpoint_schema_section",
            vec![
                "walSchemaDigest",
                "checkpointSchemaDigest",
                "diskBudgetHash",
            ],
        ),
        section(
            "traffic_ramp_blocker_section",
            vec!["trafficRampBlockerIds", "maxTrafficPpm", "rampDenied"],
        ),
        section(
            "receipt_redaction_section",
            vec!["receiptSchemaHash", "redactionState", "payloadHashOnly"],
        ),
        section(
            "retention_expiry_section",
            vec![
                "expiresAtUnixMs",
                "revocationReasonHash",
                "retentionPolicyHash",
            ],
        ),
        section(
            "replay_drift_budget_section",
            vec!["driftBudgetHash", "replayIdempotencyHash", "laneLeaseHash"],
        ),
        section(
            "lane_lease_section",
            vec!["laneId", "agentId", "leaseExpiresAtUnixMs"],
        ),
        section(
            "kill_switch_section",
            vec!["killSwitchIds", "armedInPreview", "rollbackOwnerIds"],
        ),
        section(
            "external_policy_section",
            vec![
                "deliveryPolicyHash",
                "externalTargetScope",
                "externalDeliveryDisabled",
            ],
        ),
        section(
            "external_delivery_readback_section",
            vec!["deliveryReadbackGate", "readbackHash", "publicationDenied"],
        ),
        section(
            "abort_scope_section",
            vec!["abortReasonHash", "affectedStageIds", "quarantineScopes"],
        ),
    ]
}

pub fn work_graph_persistence_operator_readiness_validation_denials()
-> Vec<WorkGraphPersistenceOperatorReadinessValidationDenialPreview> {
    vec![
        validation_denial(
            "deny_missing_durable_identity_evidence",
            vec![work_graph_persistence_operator_readiness_durable_identity_section_id()],
            "durable identity evidence packet is missing or unsatisfied",
        ),
        validation_denial(
            "deny_missing_operator_scope",
            vec!["operator_scope_section"],
            "operator scope or identity hash is missing",
        ),
        validation_denial(
            "deny_missing_shadow_live_digest",
            vec!["shadow_live_digest_section"],
            "shadow/live digest evidence is missing",
        ),
        validation_denial(
            "deny_missing_rollback_owner",
            vec!["rollback_owner_section"],
            "rollback owner or quarantine scope is missing",
        ),
        validation_denial(
            "deny_release_denial_matrix_missing",
            vec!["release_denial_section"],
            "release/publication denial matrix is missing",
        ),
        validation_denial(
            "deny_traffic_ramp_not_zero",
            vec!["traffic_ramp_blocker_section"],
            "traffic ramp is not locked to zero",
        ),
        validation_denial(
            "deny_receipt_redaction_missing",
            vec!["receipt_redaction_section"],
            "readiness packet is not redacted/hash-only",
        ),
        validation_denial(
            "deny_packet_expired_or_revoked",
            vec!["retention_expiry_section"],
            "readiness packet is expired, superseded, or revoked",
        ),
        validation_denial(
            "deny_external_policy_missing",
            vec![
                "external_policy_section",
                "external_delivery_readback_section",
            ],
            "external policy or delivery readback gate is missing",
        ),
    ]
}

pub fn work_graph_persistence_operator_readiness_acceptance_guards()
-> Vec<WorkGraphPersistenceOperatorReadinessAcceptanceGuardPreview> {
    let all_templates = work_graph_persistence_operator_readiness_template_ids();

    vec![
        acceptance_guard(
            "guard_durable_identity_evidence_declared",
            all_templates.clone(),
            work_graph_persistence_operator_readiness_durable_identity_field_ids(),
        ),
        acceptance_guard(
            "guard_non_recording_preview_acceptance",
            all_templates.clone(),
            vec!["previewMode", "approvalRecorded", "sideEffectHash"],
        ),
        acceptance_guard(
            "guard_all_sections_complete",
            all_templates.clone(),
            vec![
                "requiredSectionIds",
                "completeSectionIds",
                "validationDenialIds",
            ],
        ),
        acceptance_guard(
            "guard_release_publication_denied",
            all_templates.clone(),
            vec![
                "releaseDenialIds",
                "publicationDenied",
                "externalDeliveryDisabled",
            ],
        ),
        acceptance_guard(
            "guard_rollback_owners_declared",
            all_templates.clone(),
            vec!["rollbackOwnerIds", "quarantineScopes", "killSwitchIds"],
        ),
        acceptance_guard(
            "guard_expiry_and_revocation_current",
            all_templates,
            vec!["expiresAtUnixMs", "revocationStatus", "supersessionId"],
        ),
    ]
}

pub fn work_graph_persistence_operator_readiness_expiry_revocations()
-> Vec<WorkGraphPersistenceOperatorReadinessExpiryRevocationPreview> {
    let all_templates = work_graph_persistence_operator_readiness_template_ids();

    vec![
        expiry_revocation(
            "readiness_packet_expired",
            all_templates.clone(),
            "expiresAtUnixMs is in the past",
        ),
        expiry_revocation(
            "readiness_packet_superseded",
            all_templates.clone(),
            "newer packet digest supersedes this preview",
        ),
        expiry_revocation(
            "operator_scope_revoked",
            all_templates.clone(),
            "operator authority scope is revoked",
        ),
        expiry_revocation(
            "rollback_owner_revoked",
            all_templates,
            "rollback owner is unavailable or revoked",
        ),
    ]
}

pub fn work_graph_persistence_operator_readiness_durable_identity_evidence()
-> WorkGraphPersistenceOperatorReadinessDurableIdentityEvidencePreview {
    WorkGraphPersistenceOperatorReadinessDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: work_graph_persistence_operator_readiness_durable_identity_field_ids(),
        required_for_template_ids: work_graph_persistence_operator_readiness_template_ids(),
        required_section_id: work_graph_persistence_operator_readiness_durable_identity_section_id(
        ),
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_operator_readiness_packet_invariants()
-> Vec<WorkGraphPersistenceOperatorReadinessPacketInvariantPreview> {
    vec![
        invariant(
            "operator_readiness_requires_durable_identity_evidence",
            "operator readiness packets require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "readiness_packets_are_non_accepting",
            "packet templates describe requirements but cannot record operator acceptance",
        ),
        invariant(
            "every_packet_requires_operator_scope",
            "all readiness packet templates include an operator scope section",
        ),
        invariant(
            "release_and_publication_stay_denied",
            "operator readiness cannot override release or publication denial in preview",
        ),
        invariant(
            "expiry_revocation_blocks_acceptance",
            "expired, superseded, or revoked packets cannot become future acceptance receipts",
        ),
        invariant(
            "external_delivery_requires_separate_policy",
            "external publication readiness has its own policy and readback sections",
        ),
        invariant(
            "operator_readiness_packet_preview_has_no_side_effects",
            "this gate cannot persist packets, record approvals, enable enforcement, route traffic, publish releases, or send externally",
        ),
    ]
}

impl WorkGraphPersistenceOperatorReadinessPacketPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            readiness_packet_persisted: false,
            operator_acceptance_recorded: false,
            approval_recorded: false,
            enforcement_enabled: false,
            rollout_started: false,
            traffic_routed: false,
            live_readback_executed: false,
            release_published: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn packet_template(
    id: &'static str,
    target_rollout_stage_id: &'static str,
    required_section_ids: Vec<&'static str>,
) -> WorkGraphPersistenceOperatorReadinessPacketTemplatePreview {
    WorkGraphPersistenceOperatorReadinessPacketTemplatePreview {
        id,
        target_rollout_stage_id,
        required_section_ids,
        acceptance_allowed: false,
        external_delivery_enabled: false,
    }
}

fn section(
    id: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceOperatorReadinessSectionPreview {
    WorkGraphPersistenceOperatorReadinessSectionPreview {
        id,
        required_fields,
        redaction_state: "redacted_hash_only",
        currently_complete: false,
    }
}

fn validation_denial(
    id: &'static str,
    applies_to_section_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphPersistenceOperatorReadinessValidationDenialPreview {
    WorkGraphPersistenceOperatorReadinessValidationDenialPreview {
        id,
        applies_to_section_ids,
        reason,
        blocks_acceptance: true,
    }
}

fn acceptance_guard(
    id: &'static str,
    applies_to_template_ids: Vec<&'static str>,
    required_evidence_fields: Vec<&'static str>,
) -> WorkGraphPersistenceOperatorReadinessAcceptanceGuardPreview {
    WorkGraphPersistenceOperatorReadinessAcceptanceGuardPreview {
        id,
        applies_to_template_ids,
        required_evidence_fields,
        currently_satisfied: false,
    }
}

fn expiry_revocation(
    id: &'static str,
    applies_to_template_ids: Vec<&'static str>,
    trigger: &'static str,
) -> WorkGraphPersistenceOperatorReadinessExpiryRevocationPreview {
    WorkGraphPersistenceOperatorReadinessExpiryRevocationPreview {
        id,
        applies_to_template_ids,
        trigger,
        blocks_acceptance: true,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceOperatorReadinessPacketInvariantPreview {
    WorkGraphPersistenceOperatorReadinessPacketInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_readiness_packet_preview_declares_non_accepting_templates() {
        let report = hepta_work_graph_persistence_operator_readiness_packet_preview_report();
        let template_ids = report
            .packet_templates
            .iter()
            .map(|template| template.id)
            .collect::<Vec<_>>();

        assert_eq!(
            template_ids,
            [
                "store_persistence_readiness_packet",
                "wal_checkpoint_readiness_packet",
                "readback_receipt_readiness_packet",
                "replay_execution_readiness_packet",
                "external_publication_readiness_packet",
                "full_rollout_abort_readiness_packet",
            ]
        );
        assert_eq!(report.packet_template_count, 6);
        assert!(report.packet_templates.iter().all(|template| {
            !template.acceptance_allowed
                && !template.external_delivery_enabled
                && template.required_section_ids.len() >= 5
                && template
                    .required_section_ids
                    .contains(&"durable_identity_section")
        }));
    }

    #[test]
    fn operator_readiness_packet_preview_requires_redacted_incomplete_sections() {
        let report = hepta_work_graph_persistence_operator_readiness_packet_preview_report();

        assert_eq!(report.packet_section_count, 15);
        let durable_section = report
            .packet_sections
            .iter()
            .find(|section| section.id == "durable_identity_section")
            .expect("durable identity section present");
        assert_eq!(
            durable_section.required_fields,
            work_graph_persistence_operator_readiness_durable_identity_field_ids()
        );
        assert!(!durable_section.currently_complete);
        assert!(report.packet_sections.iter().all(|section| {
            section.redaction_state == "redacted_hash_only"
                && !section.currently_complete
                && section.required_fields.len() >= 3
        }));
    }

    #[test]
    fn operator_readiness_packet_preview_blocks_acceptance_on_validation_denials() {
        let report = hepta_work_graph_persistence_operator_readiness_packet_preview_report();
        let denial_ids = report
            .validation_denials
            .iter()
            .map(|denial| denial.id)
            .collect::<Vec<_>>();

        assert_eq!(
            denial_ids,
            [
                "deny_missing_durable_identity_evidence",
                "deny_missing_operator_scope",
                "deny_missing_shadow_live_digest",
                "deny_missing_rollback_owner",
                "deny_release_denial_matrix_missing",
                "deny_traffic_ramp_not_zero",
                "deny_receipt_redaction_missing",
                "deny_packet_expired_or_revoked",
                "deny_external_policy_missing",
            ]
        );
        assert_eq!(report.validation_denial_count, 9);
        assert!(
            report
                .validation_denials
                .iter()
                .all(|denial| denial.blocks_acceptance)
        );
    }

    #[test]
    fn operator_readiness_packet_preview_keeps_guards_unsatisfied() {
        let report = hepta_work_graph_persistence_operator_readiness_packet_preview_report();

        assert_eq!(report.acceptance_guard_count, 6);
        assert!(report.acceptance_guards.iter().all(|guard| {
            !guard.currently_satisfied
                && guard.applies_to_template_ids.len() == 6
                && guard.required_evidence_fields.len() >= 3
        }));
        let durable_guard = report
            .acceptance_guards
            .iter()
            .find(|guard| guard.id == "guard_durable_identity_evidence_declared")
            .expect("durable identity guard present");
        assert_eq!(
            durable_guard.required_evidence_fields,
            work_graph_persistence_operator_readiness_durable_identity_field_ids()
        );
        assert_eq!(report.expiry_revocation_count, 4);
        assert!(
            report
                .expiry_revocations
                .iter()
                .all(|revocation| revocation.blocks_acceptance)
        );
    }

    #[test]
    fn operator_readiness_packet_preview_requires_enforcement_rollout_blocker_gate() {
        let report = hepta_work_graph_persistence_operator_readiness_packet_preview_report();

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
                "hepta_work_graph_activation_enforcement_blocker_preview_gate",
                "hepta_work_graph_shadow_adapter_readback_preview_gate",
                "hepta_work_graph_persistence_feature_flag_preview_gate",
                "hepta_work_graph_persistence_canary_dry_run_preview_gate",
                "hepta_work_graph_persistence_canary_readback_receipt_preview_gate",
                "hepta_work_graph_persistence_promotion_blocker_preview_gate",
                "hepta_work_graph_persistence_shadow_live_readback_comparison_preview_gate",
                "hepta_work_graph_persistence_enforcement_rollout_blocker_preview_gate",
                "hepta_work_graph_durable_identity_preview_gate",
            ]
        );
        assert_eq!(
            report.durable_identity_evidence.schema_version,
            "work_graph_durable_identity_preview_v1"
        );
        assert_eq!(
            report.durable_identity_evidence.required_prior_gate,
            "hepta_work_graph_durable_identity_preview_gate"
        );
        assert_eq!(
            report.durable_identity_evidence.required_field_ids,
            work_graph_persistence_operator_readiness_durable_identity_field_ids()
        );
        assert_eq!(
            report.durable_identity_evidence.required_for_template_ids,
            work_graph_persistence_operator_readiness_template_ids()
        );
        assert_eq!(
            report.durable_identity_evidence.required_section_id,
            "durable_identity_section"
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_PACKET_RECOMMENDED_NEXT_GATE
        );
    }

    #[test]
    fn operator_readiness_packet_preview_has_no_side_effects() {
        let report = hepta_work_graph_persistence_operator_readiness_packet_preview_report();

        assert_eq!(report.invariant_count, 7);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceOperatorReadinessPacketPreviewSideEffects::none()
        );
        assert!(report.ready_for_operator_readiness_receipt_preview);
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
    }
}
