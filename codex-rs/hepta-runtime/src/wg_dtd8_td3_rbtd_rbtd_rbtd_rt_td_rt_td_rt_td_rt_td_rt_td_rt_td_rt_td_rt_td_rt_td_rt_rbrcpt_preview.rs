use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8Td3RbackAckTd12ReceiptRetentionReadbackReceiptPreviewReport {
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
    pub receipts: Vec<DeepTd8Td3RbackAckTd12ReceiptRetentionReadbackItem>,
    pub digest_checks: Vec<DeepTd8Td3RbackAckTd12ReceiptRetentionReadbackItem>,
    pub mismatch_denials: Vec<DeepTd8Td3RbackAckTd12ReceiptRetentionReadbackItem>,
    pub receipt_guards: Vec<DeepTd8Td3RbackAckTd12ReceiptRetentionReadbackItem>,
    pub local_views: Vec<DeepTd8Td3RbackAckTd12ReceiptRetentionReadbackItem>,
    pub invariants: Vec<DeepTd8Td3RbackAckTd12ReceiptRetentionReadbackItem>,
    pub recommended_next_gate: String,
    pub ready_for_readback_acknowledgement_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: DeepTd8Td3RbackAckTd12ReceiptRetentionReadbackReceiptSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8Td3RbackAckTd12ReceiptRetentionReadbackItem {
    pub id: String,
    pub required_fields: Vec<&'static str>,
    pub hash_only: bool,
    pub blocks_recording: bool,
    pub blocks_external_delivery: bool,
    pub required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DeepTd8Td3RbackAckTd12ReceiptRetentionReadbackReceiptSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub readback_receipt_persisted: bool,
    pub receipt_recorded: bool,
    pub acknowledgement_recorded: bool,
    pub operator_acceptance_recorded: bool,
    pub authority_granted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub release_published: bool,
    pub public_claim_recorded: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_readback_ack_gate()
-> String {
    format!(
        "{}_receipt_retention_expiry_readback_acknowledgement_preview_gate",
        crate::deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_base()
    )
}

pub fn hepta_work_graph_deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbrcpt_preview_report()
-> DeepTd8Td3RbackAckTd12ReceiptRetentionReadbackReceiptPreviewReport {
    let receipts = readback_items("receipt", 6);
    let digest_checks = readback_items("digest_check", 6);
    let mismatch_denials = readback_items("mismatch_denial", 7);
    let receipt_guards = readback_items("receipt_guard", 5);
    let local_views = readback_items("local_view", 4);
    let invariants = readback_items("invariant", 6);
    let gate =
        crate::deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_readback_receipt_gate();

    DeepTd8Td3RbackAckTd12ReceiptRetentionReadbackReceiptPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        schema_version: crate::deep_td8_schema_for(&gate),
        gate,
        preview_mode: "read_only_deep_td8_td3_rbackack_td12_receipt_retention_readback_receipt_preview_hash_only_no_recording",
        receipt_count: receipts.len(),
        digest_check_count: digest_checks.len(),
        mismatch_denial_count: mismatch_denials.len(),
        receipt_guard_count: receipt_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbrcpt_required_prior_gates(),
        receipts,
        digest_checks,
        mismatch_denials,
        receipt_guards,
        local_views,
        invariants,
        recommended_next_gate:
            deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_readback_ack_gate(),
        ready_for_readback_acknowledgement_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: DeepTd8Td3RbackAckTd12ReceiptRetentionReadbackReceiptSideEffects::none(),
    }
}

pub fn deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbrcpt_required_prior_gates()
-> Vec<String> {
    let mut gates =
        crate::deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_expiry_required_prior_gates();
    gates.push(
        crate::deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_receipt_retention_expiry_gate(),
    );
    gates
}

fn readback_items(
    prefix: &str,
    count: usize,
) -> Vec<DeepTd8Td3RbackAckTd12ReceiptRetentionReadbackItem> {
    (0..count)
        .map(|index| DeepTd8Td3RbackAckTd12ReceiptRetentionReadbackItem {
            id: format!(
                "deep_td8_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbrcpt_{prefix}_{index}"
            ),
            required_fields: vec!["retentionGate", "readbackDigest", "zeroEffectHash"],
            hash_only: true,
            blocks_recording: true,
            blocks_external_delivery: true,
            required: true,
        })
        .collect()
}

impl DeepTd8Td3RbackAckTd12ReceiptRetentionReadbackReceiptSideEffects {
    pub fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            readback_receipt_persisted: false,
            receipt_recorded: false,
            acknowledgement_recorded: false,
            operator_acceptance_recorded: false,
            authority_granted: false,
            wal_written: false,
            checkpoint_written: false,
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
            && !self.receipt_recorded
            && !self.acknowledgement_recorded
            && !self.operator_acceptance_recorded
            && !self.authority_granted
            && !self.wal_written
            && !self.checkpoint_written
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
    fn work_graph_td3_rbackack_td12_rcptret_rbrcpt_requires_retention_gate() {
        let report =
            hepta_work_graph_deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbrcpt_preview_report();

        assert_eq!(
            report.required_prior_gates.last(),
            Some(
                &crate::deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_receipt_retention_expiry_gate()
            )
        );
    }

    #[test]
    fn work_graph_td3_rbackack_td12_rcptret_rbrcpt_declares_hash_only_receipts() {
        let report =
            hepta_work_graph_deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbrcpt_preview_report();

        assert_eq!(report.receipt_count, 6);
        assert!(
            report
                .receipts
                .iter()
                .all(|receipt| receipt.hash_only && receipt.blocks_recording)
        );
    }

    #[test]
    fn work_graph_td3_rbackack_td12_rcptret_rbrcpt_denies_mismatch_and_delivery() {
        let report =
            hepta_work_graph_deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbrcpt_preview_report();

        assert_eq!(report.digest_check_count, 6);
        assert_eq!(report.mismatch_denial_count, 7);
        assert!(
            report
                .mismatch_denials
                .iter()
                .all(|denial| denial.blocks_external_delivery && denial.blocks_recording)
        );
    }

    #[test]
    fn work_graph_td3_rbackack_td12_rcptret_rbrcpt_requires_non_recording_guards() {
        let report =
            hepta_work_graph_deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbrcpt_preview_report();

        assert_eq!(report.receipt_guard_count, 5);
        assert!(
            report
                .receipt_guards
                .iter()
                .all(|guard| guard.hash_only && guard.blocks_recording)
        );
    }

    #[test]
    fn work_graph_td3_rbackack_td12_rcptret_rbrcpt_keeps_views_local() {
        let report =
            hepta_work_graph_deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbrcpt_preview_report();

        assert_eq!(report.local_view_count, 4);
        assert_eq!(
            report.recommended_next_gate,
            deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_readback_ack_gate()
        );
        assert!(report.ready_for_readback_acknowledgement_preview);
    }

    #[test]
    fn work_graph_td3_rbackack_td12_rcptret_rbrcpt_has_no_side_effects() {
        let report =
            hepta_work_graph_deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbrcpt_preview_report();

        assert_eq!(report.invariant_count, 6);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
        assert!(report.side_effects.all_false());
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
    }
}
