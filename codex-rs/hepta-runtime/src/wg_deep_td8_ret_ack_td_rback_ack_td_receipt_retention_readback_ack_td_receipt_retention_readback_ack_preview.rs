use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: String,
    pub schema_version: String,
    pub preview_mode: &'static str,
    pub acknowledgement_contract_count: usize,
    pub non_acceptance_reason_count: usize,
    pub recording_denial_count: usize,
    pub expiry_replay_guard_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<String>,
    pub acknowledgement_contracts: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckItem>,
    pub non_acceptance_reasons: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckItem>,
    pub recording_denials: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckItem>,
    pub expiry_replay_guards: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckItem>,
    pub local_views: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckItem>,
    pub invariants: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckItem>,
    pub recommended_next_gate: String,
    pub ready_for_acknowledgement_replay_idempotency_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckItem {
    pub id: &'static str,
    pub source_receipt_ids: Vec<&'static str>,
    pub required_fields: Vec<&'static str>,
    pub hash_only: bool,
    pub blocks_recording: bool,
    pub blocks_acceptance: bool,
    pub blocks_authority: bool,
    pub blocks_external_delivery: bool,
    pub required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub readback_receipt_persisted: bool,
    pub acknowledgement_recorded: bool,
    pub receipt_recorded: bool,
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

pub fn deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_replay_gate()
-> String {
    format!(
        "{}_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate",
        crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_base()
    )
}

pub fn hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_preview_report()
-> DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckPreviewReport {
    let acknowledgement_contracts = ack_items("contract", 6);
    let non_acceptance_reasons = ack_items("non_acceptance", 7);
    let recording_denials = ack_items("recording_denial", 7);
    let expiry_replay_guards = ack_items("expiry_replay_guard", 5);
    let local_views = ack_items("local_view", 4);
    let invariants = ack_items("invariant", 6);
    let gate = crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_gate();

    DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        schema_version: crate::deep_td8_schema_for(&gate),
        gate,
        preview_mode: "read_only_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_acknowledgement_preview_no_recording",
        acknowledgement_contract_count: acknowledgement_contracts.len(),
        non_acceptance_reason_count: non_acceptance_reasons.len(),
        recording_denial_count: recording_denials.len(),
        expiry_replay_guard_count: expiry_replay_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_required_prior_gates(),
        acknowledgement_contracts,
        non_acceptance_reasons,
        recording_denials,
        expiry_replay_guards,
        local_views,
        invariants,
        recommended_next_gate:
            deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_replay_gate(),
        ready_for_acknowledgement_replay_idempotency_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckSideEffects::none(),
    }
}

pub fn deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_required_prior_gates()
-> Vec<String> {
    let mut gates =
        crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_receipt_required_prior_gates();
    gates.push(crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_receipt_gate());
    gates
}

pub fn deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_ids()
-> Vec<&'static str> {
    vec![
        "operator_deep_td8_ret_ack_td_readback_acknowledgement",
        "auditor_deep_td8_ret_ack_td_readback_acknowledgement",
        "release_owner_deep_td8_ret_ack_td_readback_acknowledgement",
        "authority_denial_deep_td8_ret_ack_td_readback_acknowledgement",
        "public_claim_denial_deep_td8_ret_ack_td_readback_acknowledgement",
        "external_delivery_denial_deep_td8_ret_ack_td_readback_acknowledgement",
    ]
}

fn ack_items(
    prefix: &'static str,
    count: usize,
) -> Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckItem> {
    let receipt_ids = vec![
        "operator_deep_td8_ret_ack_td_retention_readback_receipt",
        "auditor_deep_td8_ret_ack_td_retention_readback_receipt",
        "release_owner_deep_td8_ret_ack_td_retention_readback_receipt",
        "authority_denial_deep_td8_ret_ack_td_retention_readback_receipt",
        "public_claim_denial_deep_td8_ret_ack_td_retention_readback_receipt",
        "external_delivery_denial_deep_td8_ret_ack_td_retention_readback_receipt",
    ];
    (0..count)
        .map(|index| {
            DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckItem {
                id: match (prefix, index) {
                    ("contract", 0) => "operator_deep_td8_ret_ack_td_readback_ack_contract",
                    ("contract", 1) => "auditor_deep_td8_ret_ack_td_readback_ack_contract",
                    ("contract", 2) => "release_owner_deep_td8_ret_ack_td_readback_ack_contract",
                    ("contract", 3) => "authority_denial_deep_td8_ret_ack_td_readback_ack_contract",
                    ("contract", 4) => {
                        "public_claim_denial_deep_td8_ret_ack_td_readback_ack_contract"
                    }
                    ("contract", _) => {
                        "external_delivery_denial_deep_td8_ret_ack_td_readback_ack_contract"
                    }
                    ("non_acceptance", 0) => "deep_td8_ret_ack_td_readback_ack_is_not_acceptance",
                    ("non_acceptance", 1) => {
                        "deep_td8_ret_ack_td_readback_ack_cannot_record_receipt"
                    }
                    ("non_acceptance", 2) => {
                        "deep_td8_ret_ack_td_readback_ack_cannot_record_approval"
                    }
                    ("non_acceptance", 3) => {
                        "deep_td8_ret_ack_td_readback_ack_cannot_grant_authority"
                    }
                    ("non_acceptance", 4) => {
                        "deep_td8_ret_ack_td_readback_ack_cannot_enable_persistence"
                    }
                    ("non_acceptance", 5) => {
                        "deep_td8_ret_ack_td_readback_ack_cannot_start_rollout"
                    }
                    ("non_acceptance", _) => {
                        "deep_td8_ret_ack_td_readback_ack_cannot_publish_or_send"
                    }
                    ("recording_denial", 0) => "deep_td8_ret_ack_td_ack_recording_denied",
                    ("recording_denial", 1) => "deep_td8_ret_ack_td_receipt_recording_denied",
                    ("recording_denial", 2) => "deep_td8_ret_ack_td_acceptance_recording_denied",
                    ("recording_denial", 3) => "deep_td8_ret_ack_td_approval_recording_denied",
                    ("recording_denial", 4) => "deep_td8_ret_ack_td_authority_recording_denied",
                    ("recording_denial", 5) => "deep_td8_ret_ack_td_public_claim_recording_denied",
                    ("recording_denial", _) => {
                        "deep_td8_ret_ack_td_external_delivery_recording_denied"
                    }
                    ("expiry_replay_guard", 0) => "deep_td8_ret_ack_td_ack_scope_unexpired",
                    ("expiry_replay_guard", 1) => "deep_td8_ret_ack_td_ack_scope_not_superseded",
                    ("expiry_replay_guard", 2) => "deep_td8_ret_ack_td_ack_digest_matches",
                    ("expiry_replay_guard", 3) => "deep_td8_ret_ack_td_ack_replay_marker_absent",
                    ("expiry_replay_guard", _) => {
                        "deep_td8_ret_ack_td_ack_zero_effect_digest_matches"
                    }
                    ("local_view", 0) => "operator_deep_td8_ret_ack_td_readback_ack_view",
                    ("local_view", 1) => "auditor_deep_td8_ret_ack_td_readback_ack_digest_view",
                    ("local_view", 2) => {
                        "release_owner_deep_td8_ret_ack_td_readback_ack_denial_view"
                    }
                    ("local_view", _) => {
                        "runtime_deep_td8_ret_ack_td_readback_ack_zero_effect_view"
                    }
                    ("invariant", 0) => {
                        "deep_td8_ret_ack_td_readback_acknowledgements_are_hash_only"
                    }
                    ("invariant", 1) => {
                        "deep_td8_ret_ack_td_readback_acknowledgements_are_non_accepting"
                    }
                    ("invariant", 2) => {
                        "deep_td8_ret_ack_td_readback_acknowledgements_are_non_recording"
                    }
                    ("invariant", 3) => {
                        "deep_td8_ret_ack_td_readback_acknowledgements_block_authority"
                    }
                    ("invariant", 4) => "deep_td8_ret_ack_td_readback_ack_views_are_local_only",
                    ("invariant", _) => {
                        "deep_td8_ret_ack_td_readback_ack_preview_has_no_side_effects"
                    }
                    _ => unreachable!(),
                },
                source_receipt_ids: receipt_ids.clone(),
                required_fields: vec!["readbackReceiptHash", "scopeEpoch", "zeroEffectHash"],
                hash_only: true,
                blocks_recording: true,
                blocks_acceptance: true,
                blocks_authority: true,
                blocks_external_delivery: true,
                required: true,
            }
        })
        .collect()
}

impl DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckSideEffects {
    pub fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            readback_receipt_persisted: false,
            acknowledgement_recorded: false,
            receipt_recorded: false,
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
            && !self.readback_receipt_persisted
            && !self.acknowledgement_recorded
            && !self.receipt_recorded
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
    fn work_graph_deep_td8_ret_ack_td_readback_ack_requires_readback_receipt_gate() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_preview_report();

        assert_eq!(
            report.required_prior_gates.last(),
            Some(
                &crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_receipt_gate()
            )
        );
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_readback_ack_declares_non_accepting_contracts() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_preview_report();

        assert_eq!(report.acknowledgement_contract_count, 6);
        assert!(report.acknowledgement_contracts.iter().all(|contract| {
            contract.hash_only
                && contract.blocks_acceptance
                && contract.source_receipt_ids.len() == 6
        }));
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_readback_ack_blocks_acceptance_and_authority() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_preview_report();

        assert_eq!(report.non_acceptance_reason_count, 7);
        assert!(
            report
                .non_acceptance_reasons
                .iter()
                .all(|reason| reason.blocks_acceptance && reason.blocks_authority)
        );
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_readback_ack_denies_recording_and_delivery() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_preview_report();

        assert_eq!(report.recording_denial_count, 7);
        assert!(
            report
                .recording_denials
                .iter()
                .all(|denial| denial.blocks_recording && denial.blocks_external_delivery)
        );
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_readback_ack_guards_expiry_scope_and_replay() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_preview_report();

        assert_eq!(report.expiry_replay_guard_count, 5);
        assert_eq!(
            report.recommended_next_gate,
            deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_replay_gate()
        );
        assert!(
            report
                .expiry_replay_guards
                .iter()
                .all(|guard| guard.required)
        );
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_readback_ack_has_no_side_effects() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_preview_report();

        assert_eq!(report.local_view_count, 4);
        assert_eq!(report.invariant_count, 6);
        assert!(report.side_effects.all_false());
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
    }
}
