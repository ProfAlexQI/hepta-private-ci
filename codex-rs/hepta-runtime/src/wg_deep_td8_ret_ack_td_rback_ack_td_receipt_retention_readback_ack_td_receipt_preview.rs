use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: String,
    pub schema_version: String,
    pub preview_mode: &'static str,
    pub receipt_count: usize,
    pub digest_check_count: usize,
    pub mismatch_denial_count: usize,
    pub receipt_guard_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<String>,
    pub receipts: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptItem>,
    pub digest_checks: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptItem>,
    pub mismatch_denials: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptItem>,
    pub receipt_guards: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptItem>,
    pub local_views: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptItem>,
    pub invariants: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptItem>,
    pub recommended_next_gate: String,
    pub ready_for_terminal_decision_receipt_acknowledgement_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptItem {
    pub id: String,
    pub source_surface_ids: Vec<String>,
    pub required_fields: Vec<&'static str>,
    pub hash_only: bool,
    pub blocks_receipt_recording: bool,
    pub blocks_acceptance: bool,
    pub blocks_authority: bool,
    pub blocks_external_delivery: bool,
    pub required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_receipt_recorded: bool,
    pub terminal_decision_receipt_persisted: bool,
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

pub fn deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_acknowledgement_gate()
-> String {
    format!(
        "{}_receipt_acknowledgement_preview_gate",
        crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_base()
    )
}

pub fn hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_preview_report()
-> DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptPreviewReport {
    let receipts = receipt_items("receipt", 6);
    let digest_checks = receipt_items("digest_check", 6);
    let mismatch_denials = receipt_items("mismatch_denial", 7);
    let receipt_guards = receipt_items("receipt_guard", 5);
    let local_views = receipt_items("local_view", 4);
    let invariants = receipt_items("invariant", 6);
    let gate =
        crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_gate();

    DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        schema_version: crate::deep_td8_schema_for(&gate),
        gate,
        preview_mode: "read_only_deep_td8_ret_ack_td_rback_ack_terminal_decision_receipt_preview_hash_only_no_recording",
        receipt_count: receipts.len(),
        digest_check_count: digest_checks.len(),
        mismatch_denial_count: mismatch_denials.len(),
        receipt_guard_count: receipt_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates: deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_required_prior_gates(),
        receipts,
        digest_checks,
        mismatch_denials,
        receipt_guards,
        local_views,
        invariants,
        recommended_next_gate: deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_acknowledgement_gate(),
        ready_for_terminal_decision_receipt_acknowledgement_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptSideEffects::none(),
    }
}

pub fn deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_required_prior_gates()
-> Vec<String> {
    let mut gates = crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_required_prior_gates();
    gates.push(crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_terminal_decision_gate());
    gates
}

pub fn deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ids()
-> Vec<String> {
    (0..6)
        .map(|index| {
            format!(
                "deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_{index}"
            )
        })
        .collect()
}

fn receipt_items(
    prefix: &str,
    count: usize,
) -> Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptItem> {
    let source_surface_ids =
        crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_surface_ids();
    (0..count)
        .map(|index| DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptItem {
            id: format!("deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_{prefix}_{index}"),
            source_surface_ids: source_surface_ids.clone(),
            required_fields: vec!["terminalDecisionHash", "receiptHash", "zeroEffectHash"],
            hash_only: true,
            blocks_receipt_recording: true,
            blocks_acceptance: true,
            blocks_authority: true,
            blocks_external_delivery: true,
            required: true,
        })
        .collect()
}

impl DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptSideEffects {
    pub fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            terminal_decision_receipt_recorded: false,
            terminal_decision_receipt_persisted: false,
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
            && !self.terminal_decision_receipt_persisted
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
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_requires_terminal_decision_gate()
     {
        let report = hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_preview_report();

        assert_eq!(
            report.required_prior_gates.last(),
            Some(
                &crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_terminal_decision_gate()
            )
        );
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipts_are_hash_only()
     {
        let report = hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_preview_report();

        assert_eq!(report.receipt_count, 6);
        assert!(
            report
                .receipts
                .iter()
                .all(|receipt| receipt.hash_only && receipt.blocks_receipt_recording)
        );
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipts_check_digests_and_mismatches()
     {
        let report = hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_preview_report();

        assert_eq!(report.digest_check_count, 6);
        assert_eq!(report.mismatch_denial_count, 7);
        assert!(
            report
                .mismatch_denials
                .iter()
                .all(|denial| denial.blocks_acceptance && denial.blocks_authority)
        );
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipts_guard_recording_and_delivery()
     {
        let report = hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_preview_report();

        assert_eq!(report.receipt_guard_count, 5);
        assert!(
            report
                .receipt_guards
                .iter()
                .all(|guard| guard.blocks_receipt_recording && guard.blocks_external_delivery)
        );
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_points_to_ack_gate()
     {
        let report = hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_preview_report();

        assert_eq!(
            report.recommended_next_gate,
            deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_acknowledgement_gate()
        );
        assert!(report.ready_for_terminal_decision_receipt_acknowledgement_preview);
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_has_no_side_effects()
     {
        let report = hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_preview_report();

        assert!(report.side_effects.all_false());
    }
}
