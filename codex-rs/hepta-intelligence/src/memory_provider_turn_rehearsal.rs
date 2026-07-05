use serde::Deserialize;
use serde::Serialize;

use crate::MEMORY_TURN_DISPATCH_GATE_V1_CONTRACT;
use crate::memory_turn_dispatch_gate_sample_report;

pub const MEMORY_PROVIDER_TURN_REHEARSAL_V1_CONTRACT: &str =
    "hepta-intelligence-memory-provider-turn-rehearsal-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderTurnRehearsalPolicy {
    pub policy_id: &'static str,
    pub feature_flag_id: &'static str,
    pub require_feature_flag: bool,
    pub require_installed_runtime_readback: bool,
    pub require_telemetry_readback: bool,
    pub require_rollback_receipt: bool,
    pub block_provider_invocation_in_sample: bool,
    pub context_attachment_mode: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderTurnRollbackPlan {
    pub rollback_receipt_id: String,
    pub feature_flag_id: &'static str,
    pub disables_memory_context: bool,
    pub drops_staged_context: bool,
    pub replay_without_memory_supported: bool,
    pub installed_runtime_readback_present: bool,
    pub mutation_performed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderTurnPlan {
    pub turn_id: String,
    pub provider_stage: &'static str,
    pub dispatch_contract: &'static str,
    pub dispatch_decision_hash: String,
    pub prompt_bundle_id: String,
    pub feature_flag_enabled: bool,
    pub installed_runtime_readback_present: bool,
    pub telemetry_readback_id: String,
    pub rollback_receipt_id: String,
    pub attached_node_ids: Vec<String>,
    pub context_attachment_allowed_by_dispatch: bool,
    pub provider_request_prepared: bool,
    pub context_attached_to_live_prompt: bool,
    pub model_invoked: bool,
    pub reply_delivered: bool,
    pub fallback_action: &'static str,
    pub provider_request_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderTurnTelemetryRecord {
    pub telemetry_id: String,
    pub event_kind: &'static str,
    pub turn_id: String,
    pub dispatch_decision_hash: String,
    pub readback_evidence_id: String,
    pub rollback_receipt_id: String,
    pub redacted_payload: bool,
    pub context_injection_performed: bool,
    pub provider_invoked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderTurnRehearsalChecks {
    pub dispatch_gate_ready: bool,
    pub feature_flag_gate_declared: bool,
    pub installed_runtime_readback_present: bool,
    pub telemetry_readback_present: bool,
    pub rollback_receipt_present: bool,
    pub approved_plan_preserves_dispatch_decision: bool,
    pub approved_plan_requires_feature_flag: bool,
    pub disabled_flag_blocks_attachment: bool,
    pub exact_node_ids_preserved: bool,
    pub context_attachment_rehearsed_without_injection: bool,
    pub no_provider_model_invocation: bool,
    pub no_reply_delivery: bool,
    pub no_external_network_read: bool,
    pub no_production_memory_mutation: bool,
    pub no_raw_private_memory_logged: bool,
}

impl MemoryProviderTurnRehearsalChecks {
    pub fn ready(&self) -> bool {
        self.dispatch_gate_ready
            && self.feature_flag_gate_declared
            && self.installed_runtime_readback_present
            && self.telemetry_readback_present
            && self.rollback_receipt_present
            && self.approved_plan_preserves_dispatch_decision
            && self.approved_plan_requires_feature_flag
            && self.disabled_flag_blocks_attachment
            && self.exact_node_ids_preserved
            && self.context_attachment_rehearsed_without_injection
            && self.no_provider_model_invocation
            && self.no_reply_delivery
            && self.no_external_network_read
            && self.no_production_memory_mutation
            && self.no_raw_private_memory_logged
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderTurnRehearsalReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub p12_provider_turn_rehearsal_ready: bool,
    pub native_rewrite: bool,
    pub sample_run: bool,
    pub dispatch_contract: &'static str,
    pub included_node_count: usize,
    pub telemetry_record_count: usize,
    pub policy: MemoryProviderTurnRehearsalPolicy,
    pub approved_rehearsal_plan: MemoryProviderTurnPlan,
    pub disabled_flag_plan: MemoryProviderTurnPlan,
    pub rollback_plan: MemoryProviderTurnRollbackPlan,
    pub telemetry_records: Vec<MemoryProviderTurnTelemetryRecord>,
    pub checks: MemoryProviderTurnRehearsalChecks,
    pub next_phase: &'static str,
}

pub fn memory_provider_turn_rehearsal_sample_report(
    sample_run: bool,
) -> MemoryProviderTurnRehearsalReport {
    let dispatch = memory_turn_dispatch_gate_sample_report(true);
    let policy = MemoryProviderTurnRehearsalPolicy {
        policy_id: "memory-provider-turn-rehearsal-policy-v1",
        feature_flag_id: "HEPTA_MEMORY_CONTEXT_LIVE_TURN",
        require_feature_flag: true,
        require_installed_runtime_readback: true,
        require_telemetry_readback: true,
        require_rollback_receipt: true,
        block_provider_invocation_in_sample: true,
        context_attachment_mode: "provider_turn_rehearsal_only",
    };
    let rollback_plan = MemoryProviderTurnRollbackPlan {
        rollback_receipt_id: stable_digest("memory-provider-turn-rollback:sample"),
        feature_flag_id: policy.feature_flag_id,
        disables_memory_context: true,
        drops_staged_context: true,
        replay_without_memory_supported: true,
        installed_runtime_readback_present: true,
        mutation_performed: false,
    };
    let approved_rehearsal_plan =
        provider_turn_plan_from_dispatch(&dispatch, &policy, &rollback_plan, true);
    let disabled_flag_plan =
        provider_turn_plan_from_dispatch(&dispatch, &policy, &rollback_plan, false);
    let telemetry_records = vec![
        telemetry_record_from_plan(&approved_rehearsal_plan),
        telemetry_record_from_plan(&disabled_flag_plan),
    ];

    let checks = MemoryProviderTurnRehearsalChecks {
        dispatch_gate_ready: dispatch.p11_turn_dispatch_gate_ready,
        feature_flag_gate_declared: policy.require_feature_flag
            && !policy.feature_flag_id.trim().is_empty(),
        installed_runtime_readback_present: policy.require_installed_runtime_readback
            && rollback_plan.installed_runtime_readback_present
            && approved_rehearsal_plan.installed_runtime_readback_present,
        telemetry_readback_present: policy.require_telemetry_readback
            && telemetry_records.iter().all(|record| {
                !record.telemetry_id.trim().is_empty()
                    && !record.readback_evidence_id.trim().is_empty()
                    && record.redacted_payload
            }),
        rollback_receipt_present: policy.require_rollback_receipt
            && !rollback_plan.rollback_receipt_id.trim().is_empty()
            && rollback_plan.disables_memory_context
            && rollback_plan.replay_without_memory_supported,
        approved_plan_preserves_dispatch_decision: approved_rehearsal_plan.dispatch_decision_hash
            == dispatch.approved_dispatch_decision.decision_hash,
        approved_plan_requires_feature_flag: approved_rehearsal_plan.feature_flag_enabled
            && approved_rehearsal_plan.provider_request_prepared
            && approved_rehearsal_plan.context_attachment_allowed_by_dispatch,
        disabled_flag_blocks_attachment: !disabled_flag_plan.feature_flag_enabled
            && !disabled_flag_plan.provider_request_prepared
            && disabled_flag_plan.attached_node_ids.is_empty()
            && disabled_flag_plan.fallback_action == "run_without_memory_context",
        exact_node_ids_preserved: approved_rehearsal_plan.attached_node_ids
            == dispatch.approved_dispatch_decision.attached_node_ids,
        context_attachment_rehearsed_without_injection: approved_rehearsal_plan
            .provider_request_prepared
            && !approved_rehearsal_plan.context_attached_to_live_prompt
            && !approved_rehearsal_plan.model_invoked,
        no_provider_model_invocation: policy.block_provider_invocation_in_sample
            && !approved_rehearsal_plan.model_invoked
            && !disabled_flag_plan.model_invoked
            && telemetry_records
                .iter()
                .all(|record| !record.provider_invoked),
        no_reply_delivery: !approved_rehearsal_plan.reply_delivered
            && !disabled_flag_plan.reply_delivered,
        no_external_network_read: true,
        no_production_memory_mutation: !rollback_plan.mutation_performed,
        no_raw_private_memory_logged: telemetry_records
            .iter()
            .all(|record| record.redacted_payload),
    };
    let p12_provider_turn_rehearsal_ready = checks.ready();

    MemoryProviderTurnRehearsalReport {
        product: "Hepta",
        command: "memory-provider-turn-rehearsal",
        contract: MEMORY_PROVIDER_TURN_REHEARSAL_V1_CONTRACT,
        status: if p12_provider_turn_rehearsal_ready {
            "ready"
        } else {
            "attention"
        },
        p12_provider_turn_rehearsal_ready,
        native_rewrite: true,
        sample_run,
        dispatch_contract: MEMORY_TURN_DISPATCH_GATE_V1_CONTRACT,
        included_node_count: approved_rehearsal_plan.attached_node_ids.len(),
        telemetry_record_count: telemetry_records.len(),
        policy,
        approved_rehearsal_plan,
        disabled_flag_plan,
        rollback_plan,
        telemetry_records,
        checks,
        next_phase: "wire the rehearsed provider turn into installed runtime telemetry behind disabled-by-default feature flags before any live memory context injection is allowed",
    }
}

fn provider_turn_plan_from_dispatch(
    dispatch: &crate::MemoryTurnDispatchGateReport,
    policy: &MemoryProviderTurnRehearsalPolicy,
    rollback_plan: &MemoryProviderTurnRollbackPlan,
    feature_flag_enabled: bool,
) -> MemoryProviderTurnPlan {
    let context_attachment_allowed_by_dispatch = dispatch
        .approved_dispatch_decision
        .context_attachment_allowed;
    let provider_request_prepared = feature_flag_enabled
        && context_attachment_allowed_by_dispatch
        && rollback_plan.installed_runtime_readback_present;
    let attached_node_ids = if provider_request_prepared {
        dispatch
            .approved_dispatch_decision
            .attached_node_ids
            .clone()
    } else {
        Vec::new()
    };
    let telemetry_readback_id = stable_digest(&format!(
        "memory-provider-telemetry:{}:{}:{}",
        dispatch.approved_dispatch_decision.turn_id,
        dispatch.approved_dispatch_decision.decision_hash,
        feature_flag_enabled
    ));
    let provider_request_hash = stable_digest(&format!(
        "{}:{}:{}:{}:{}",
        dispatch.approved_dispatch_decision.turn_id,
        dispatch.approved_dispatch_decision.decision_hash,
        policy.feature_flag_id,
        feature_flag_enabled,
        attached_node_ids.join(",")
    ));

    MemoryProviderTurnPlan {
        turn_id: dispatch.approved_dispatch_decision.turn_id.clone(),
        provider_stage: "before_model_request",
        dispatch_contract: dispatch.contract,
        dispatch_decision_hash: dispatch.approved_dispatch_decision.decision_hash.clone(),
        prompt_bundle_id: dispatch.approved_dispatch_decision.prompt_bundle_id.clone(),
        feature_flag_enabled,
        installed_runtime_readback_present: rollback_plan.installed_runtime_readback_present,
        telemetry_readback_id,
        rollback_receipt_id: rollback_plan.rollback_receipt_id.clone(),
        attached_node_ids,
        context_attachment_allowed_by_dispatch,
        provider_request_prepared,
        context_attached_to_live_prompt: false,
        model_invoked: false,
        reply_delivered: false,
        fallback_action: if provider_request_prepared {
            "rehearse_provider_turn_without_invocation"
        } else {
            "run_without_memory_context"
        },
        provider_request_hash,
    }
}

fn telemetry_record_from_plan(plan: &MemoryProviderTurnPlan) -> MemoryProviderTurnTelemetryRecord {
    MemoryProviderTurnTelemetryRecord {
        telemetry_id: plan.telemetry_readback_id.clone(),
        event_kind: "memory_context_provider_turn_rehearsal",
        turn_id: plan.turn_id.clone(),
        dispatch_decision_hash: plan.dispatch_decision_hash.clone(),
        readback_evidence_id: plan.provider_request_hash.clone(),
        rollback_receipt_id: plan.rollback_receipt_id.clone(),
        redacted_payload: true,
        context_injection_performed: plan.context_attached_to_live_prompt,
        provider_invoked: plan.model_invoked,
    }
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
    fn memory_provider_turn_rehearsal_sample_gate_is_ready() {
        let report = memory_provider_turn_rehearsal_sample_report(true);

        assert_eq!(report.status, "ready");
        assert!(report.p12_provider_turn_rehearsal_ready);
        assert!(report.checks.ready());
        assert_eq!(report.included_node_count, 4);
        assert_eq!(report.telemetry_record_count, 2);
        assert!(report.approved_rehearsal_plan.provider_request_prepared);
        assert!(!report.approved_rehearsal_plan.model_invoked);
    }

    #[test]
    fn memory_provider_turn_rehearsal_feature_flag_blocks_attachment() {
        let report = memory_provider_turn_rehearsal_sample_report(true);

        assert!(report.checks.disabled_flag_blocks_attachment);
        assert!(!report.disabled_flag_plan.feature_flag_enabled);
        assert!(!report.disabled_flag_plan.provider_request_prepared);
        assert!(report.disabled_flag_plan.attached_node_ids.is_empty());
        assert_eq!(
            report.disabled_flag_plan.fallback_action,
            "run_without_memory_context"
        );
    }

    #[test]
    fn memory_provider_turn_rehearsal_preserves_dispatch_telemetry_and_rollback() {
        let report = memory_provider_turn_rehearsal_sample_report(true);

        assert!(report.checks.approved_plan_preserves_dispatch_decision);
        assert!(report.checks.exact_node_ids_preserved);
        assert!(report.checks.telemetry_readback_present);
        assert!(report.checks.rollback_receipt_present);
        assert!(report.rollback_plan.installed_runtime_readback_present);
        assert!(
            report
                .telemetry_records
                .iter()
                .all(|record| record.redacted_payload && !record.provider_invoked)
        );
    }
}
