use serde::Deserialize;
use serde::Serialize;

use crate::MEMORY_PROVIDER_TURN_REHEARSAL_V1_CONTRACT;
use crate::memory_provider_turn_rehearsal_sample_report;

pub const MEMORY_INSTALLED_TELEMETRY_GATE_V1_CONTRACT: &str =
    "hepta-intelligence-memory-installed-telemetry-gate-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryInstalledTelemetryPolicy {
    pub policy_id: &'static str,
    pub feature_flag_id: &'static str,
    pub require_disabled_by_default: bool,
    pub require_installed_service_witness: bool,
    pub require_redacted_provider_request_hash: bool,
    pub require_rollback_replay: bool,
    pub require_telemetry_hash_chain: bool,
    pub block_live_context_injection_in_sample: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryInstalledRuntimeWitness {
    pub witness_id: String,
    pub binary_path: &'static str,
    pub service_labels: Vec<&'static str>,
    pub feature_flag_default_enabled: bool,
    pub telemetry_sink: &'static str,
    pub local_readback_present: bool,
    pub external_network_read: bool,
    pub mutation_performed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderRequestRedactionProof {
    pub proof_id: String,
    pub provider_request_hash: String,
    pub dispatch_decision_hash: String,
    pub attached_node_count: usize,
    pub redacted_payload: bool,
    pub raw_memory_content_logged: bool,
    pub source_citations_retained: bool,
    pub provider_invoked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryRollbackReplayResult {
    pub replay_id: String,
    pub rollback_receipt_id: String,
    pub feature_flag_id: &'static str,
    pub replayed: bool,
    pub restored_feature_flag_default: bool,
    pub dropped_staged_context_node_count: usize,
    pub no_memory_provider_turn_hash: String,
    pub provider_invoked: bool,
    pub mutation_performed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryTelemetryHashChainRecord {
    pub sequence: u64,
    pub telemetry_id: String,
    pub previous_hash: String,
    pub record_hash: String,
    pub event_kind: &'static str,
    pub redacted_payload: bool,
    pub provider_invoked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryInstalledTelemetryChecks {
    pub provider_rehearsal_ready: bool,
    pub installed_service_witness_present: bool,
    pub feature_flag_disabled_by_default: bool,
    pub telemetry_hash_chain_complete: bool,
    pub provider_request_redacted: bool,
    pub rollback_replay_ready: bool,
    pub rollback_restores_no_memory_turn: bool,
    pub no_live_context_injection: bool,
    pub no_provider_model_invocation: bool,
    pub no_reply_delivery: bool,
    pub no_external_network_read: bool,
    pub no_production_memory_mutation: bool,
    pub no_raw_private_memory_logged: bool,
}

impl MemoryInstalledTelemetryChecks {
    pub fn ready(&self) -> bool {
        self.provider_rehearsal_ready
            && self.installed_service_witness_present
            && self.feature_flag_disabled_by_default
            && self.telemetry_hash_chain_complete
            && self.provider_request_redacted
            && self.rollback_replay_ready
            && self.rollback_restores_no_memory_turn
            && self.no_live_context_injection
            && self.no_provider_model_invocation
            && self.no_reply_delivery
            && self.no_external_network_read
            && self.no_production_memory_mutation
            && self.no_raw_private_memory_logged
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryInstalledTelemetryGateReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub p13_installed_telemetry_gate_ready: bool,
    pub native_rewrite: bool,
    pub sample_run: bool,
    pub provider_rehearsal_contract: &'static str,
    pub feature_flag_id: &'static str,
    pub telemetry_record_count: usize,
    pub redaction_proof_count: usize,
    pub policy: MemoryInstalledTelemetryPolicy,
    pub installed_runtime_witness: MemoryInstalledRuntimeWitness,
    pub redaction_proofs: Vec<MemoryProviderRequestRedactionProof>,
    pub rollback_replay: MemoryRollbackReplayResult,
    pub telemetry_hash_chain: Vec<MemoryTelemetryHashChainRecord>,
    pub checks: MemoryInstalledTelemetryChecks,
    pub next_phase: &'static str,
}

pub fn memory_installed_telemetry_gate_sample_report(
    sample_run: bool,
) -> MemoryInstalledTelemetryGateReport {
    let rehearsal = memory_provider_turn_rehearsal_sample_report(true);
    let policy = MemoryInstalledTelemetryPolicy {
        policy_id: "memory-installed-telemetry-policy-v1",
        feature_flag_id: rehearsal.policy.feature_flag_id,
        require_disabled_by_default: true,
        require_installed_service_witness: true,
        require_redacted_provider_request_hash: true,
        require_rollback_replay: true,
        require_telemetry_hash_chain: true,
        block_live_context_injection_in_sample: true,
    };
    let installed_runtime_witness = MemoryInstalledRuntimeWitness {
        witness_id: stable_digest("memory-installed-runtime-witness:sample"),
        binary_path: "/Users/qianqi/.local/opt/hepta/bin/hepta",
        service_labels: vec!["ai.hepta.gateway"],
        feature_flag_default_enabled: false,
        telemetry_sink: "local_redacted_memory_turn_telemetry",
        local_readback_present: true,
        external_network_read: false,
        mutation_performed: false,
    };
    let redaction_proofs = vec![
        redaction_proof_from_plan(&rehearsal.approved_rehearsal_plan),
        redaction_proof_from_plan(&rehearsal.disabled_flag_plan),
    ];
    let rollback_replay = MemoryRollbackReplayResult {
        replay_id: stable_digest(&format!(
            "memory-rollback-replay:{}:{}",
            rehearsal.rollback_plan.rollback_receipt_id, policy.feature_flag_id
        )),
        rollback_receipt_id: rehearsal.rollback_plan.rollback_receipt_id.clone(),
        feature_flag_id: policy.feature_flag_id,
        replayed: true,
        restored_feature_flag_default: true,
        dropped_staged_context_node_count: rehearsal.included_node_count,
        no_memory_provider_turn_hash: rehearsal.disabled_flag_plan.provider_request_hash.clone(),
        provider_invoked: false,
        mutation_performed: false,
    };
    let telemetry_hash_chain = telemetry_hash_chain_from_rehearsal(&rehearsal);

    let checks = MemoryInstalledTelemetryChecks {
        provider_rehearsal_ready: rehearsal.p12_provider_turn_rehearsal_ready,
        installed_service_witness_present: policy.require_installed_service_witness
            && installed_runtime_witness.local_readback_present
            && !installed_runtime_witness.binary_path.trim().is_empty()
            && installed_runtime_witness
                .service_labels
                .contains(&"ai.hepta.gateway"),
        feature_flag_disabled_by_default: policy.require_disabled_by_default
            && !installed_runtime_witness.feature_flag_default_enabled,
        telemetry_hash_chain_complete: policy.require_telemetry_hash_chain
            && telemetry_hash_chain.len() == rehearsal.telemetry_record_count
            && telemetry_hash_chain
                .iter()
                .all(|record| !record.record_hash.trim().is_empty()),
        provider_request_redacted: policy.require_redacted_provider_request_hash
            && redaction_proofs.iter().all(|proof| {
                proof.redacted_payload
                    && !proof.raw_memory_content_logged
                    && proof.source_citations_retained
                    && !proof.provider_invoked
            }),
        rollback_replay_ready: policy.require_rollback_replay
            && rollback_replay.replayed
            && rollback_replay.rollback_receipt_id == rehearsal.rollback_plan.rollback_receipt_id,
        rollback_restores_no_memory_turn: rollback_replay.restored_feature_flag_default
            && rollback_replay.dropped_staged_context_node_count == rehearsal.included_node_count
            && rollback_replay.no_memory_provider_turn_hash
                == rehearsal.disabled_flag_plan.provider_request_hash,
        no_live_context_injection: policy.block_live_context_injection_in_sample
            && !rehearsal
                .approved_rehearsal_plan
                .context_attached_to_live_prompt
            && !rehearsal.disabled_flag_plan.context_attached_to_live_prompt,
        no_provider_model_invocation: !rehearsal.approved_rehearsal_plan.model_invoked
            && !rehearsal.disabled_flag_plan.model_invoked
            && !rollback_replay.provider_invoked
            && telemetry_hash_chain
                .iter()
                .all(|record| !record.provider_invoked),
        no_reply_delivery: !rehearsal.approved_rehearsal_plan.reply_delivered
            && !rehearsal.disabled_flag_plan.reply_delivered,
        no_external_network_read: !installed_runtime_witness.external_network_read,
        no_production_memory_mutation: !installed_runtime_witness.mutation_performed
            && !rollback_replay.mutation_performed,
        no_raw_private_memory_logged: redaction_proofs
            .iter()
            .all(|proof| !proof.raw_memory_content_logged),
    };
    let p13_installed_telemetry_gate_ready = checks.ready();

    MemoryInstalledTelemetryGateReport {
        product: "Hepta",
        command: "memory-installed-telemetry-gate",
        contract: MEMORY_INSTALLED_TELEMETRY_GATE_V1_CONTRACT,
        status: if p13_installed_telemetry_gate_ready {
            "ready"
        } else {
            "attention"
        },
        p13_installed_telemetry_gate_ready,
        native_rewrite: true,
        sample_run,
        provider_rehearsal_contract: MEMORY_PROVIDER_TURN_REHEARSAL_V1_CONTRACT,
        feature_flag_id: policy.feature_flag_id,
        telemetry_record_count: telemetry_hash_chain.len(),
        redaction_proof_count: redaction_proofs.len(),
        policy,
        installed_runtime_witness,
        redaction_proofs,
        rollback_replay,
        telemetry_hash_chain,
        checks,
        next_phase: "connect installed memory telemetry to the live provider router only after disabled-by-default flag checks, redacted request hashes, and rollback replay are enforced by runtime code",
    }
}

fn redaction_proof_from_plan(
    plan: &crate::MemoryProviderTurnPlan,
) -> MemoryProviderRequestRedactionProof {
    MemoryProviderRequestRedactionProof {
        proof_id: stable_digest(&format!(
            "memory-provider-redaction:{}",
            plan.provider_request_hash
        )),
        provider_request_hash: plan.provider_request_hash.clone(),
        dispatch_decision_hash: plan.dispatch_decision_hash.clone(),
        attached_node_count: plan.attached_node_ids.len(),
        redacted_payload: true,
        raw_memory_content_logged: false,
        source_citations_retained: true,
        provider_invoked: plan.model_invoked,
    }
}

fn telemetry_hash_chain_from_rehearsal(
    rehearsal: &crate::MemoryProviderTurnRehearsalReport,
) -> Vec<MemoryTelemetryHashChainRecord> {
    let mut previous_hash = "genesis".to_string();
    rehearsal
        .telemetry_records
        .iter()
        .enumerate()
        .map(|(idx, record)| {
            let sequence = (idx + 1) as u64;
            let record_hash = stable_digest(&format!(
                "{}:{}:{}:{}:{}",
                sequence,
                previous_hash,
                record.telemetry_id,
                record.readback_evidence_id,
                record.rollback_receipt_id
            ));
            let chain_record = MemoryTelemetryHashChainRecord {
                sequence,
                telemetry_id: record.telemetry_id.clone(),
                previous_hash: previous_hash.clone(),
                record_hash: record_hash.clone(),
                event_kind: record.event_kind,
                redacted_payload: record.redacted_payload,
                provider_invoked: record.provider_invoked,
            };
            previous_hash = record_hash;
            chain_record
        })
        .collect()
}

fn stable_digest(value: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in value.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("sha256-sample-{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_installed_telemetry_gate_sample_gate_is_ready() {
        let report = memory_installed_telemetry_gate_sample_report(true);

        assert_eq!(report.status, "ready");
        assert!(report.p13_installed_telemetry_gate_ready);
        assert!(report.checks.ready());
        assert_eq!(report.telemetry_record_count, 2);
        assert_eq!(report.redaction_proof_count, 2);
        assert!(
            !report
                .installed_runtime_witness
                .feature_flag_default_enabled
        );
    }

    #[test]
    fn memory_installed_telemetry_gate_proves_redaction_and_hash_chain() {
        let report = memory_installed_telemetry_gate_sample_report(true);

        assert!(report.checks.provider_request_redacted);
        assert!(report.checks.telemetry_hash_chain_complete);
        assert_eq!(report.telemetry_hash_chain[0].previous_hash, "genesis");
        assert_eq!(
            report.telemetry_hash_chain[1].previous_hash,
            report.telemetry_hash_chain[0].record_hash
        );
        assert!(
            report
                .redaction_proofs
                .iter()
                .all(|proof| proof.redacted_payload && !proof.raw_memory_content_logged)
        );
    }

    #[test]
    fn memory_installed_telemetry_gate_replays_rollback_to_no_memory_turn() {
        let report = memory_installed_telemetry_gate_sample_report(true);

        assert!(report.checks.rollback_replay_ready);
        assert!(report.checks.rollback_restores_no_memory_turn);
        assert!(report.rollback_replay.replayed);
        assert!(report.rollback_replay.restored_feature_flag_default);
        assert_eq!(report.rollback_replay.dropped_staged_context_node_count, 4);
        assert!(!report.rollback_replay.provider_invoked);
    }
}
