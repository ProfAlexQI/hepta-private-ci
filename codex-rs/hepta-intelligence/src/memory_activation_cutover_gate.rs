use serde::Deserialize;
use serde::Serialize;

use crate::MEMORY_INSTALLED_TELEMETRY_GATE_V1_CONTRACT;
use crate::memory_installed_telemetry_gate_sample_report;

pub const MEMORY_ACTIVATION_CUTOVER_GATE_V1_CONTRACT: &str =
    "hepta-intelligence-memory-activation-cutover-gate-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryActivationCutoverPolicy {
    pub policy_id: &'static str,
    pub feature_flag_id: &'static str,
    pub require_installed_telemetry_gate: bool,
    pub require_manual_release_approval: bool,
    pub require_release_checklist: bool,
    pub require_canary_stage_order: bool,
    pub require_kill_switch: bool,
    pub require_rollback_drill: bool,
    pub block_automatic_enablement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryReleaseChecklistItem {
    pub item_id: &'static str,
    pub description: &'static str,
    pub required: bool,
    pub satisfied: bool,
    pub evidence_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryCanaryStage {
    pub stage_id: &'static str,
    pub traffic_percent_ppm: u32,
    pub max_attached_node_count: usize,
    pub telemetry_hash_chain_required: bool,
    pub rollback_on_failure: bool,
    pub live_prompt_attachment_performed: bool,
    pub provider_invoked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKillSwitchPlan {
    pub kill_switch_id: String,
    pub feature_flag_id: &'static str,
    pub disables_context_attachment: bool,
    pub restores_no_memory_provider_turn_hash: String,
    pub rollback_receipt_id: String,
    pub telemetry_drain_required: bool,
    pub mutation_performed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryActivationCutoverDecision {
    pub decision_id: String,
    pub release_channel: &'static str,
    pub operator_release_approved: bool,
    pub activation_allowed: bool,
    pub staged_rollout_allowed: bool,
    pub feature_flag_currently_enabled: bool,
    pub feature_flag_mutated: bool,
    pub context_attached_to_live_prompt: bool,
    pub provider_invoked: bool,
    pub decision_reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryActivationCutoverChecks {
    pub installed_telemetry_gate_ready: bool,
    pub release_checklist_complete: bool,
    pub canary_stage_order_valid: bool,
    pub kill_switch_ready: bool,
    pub rollback_drill_ready: bool,
    pub approved_decision_allows_staged_rollout: bool,
    pub missing_approval_blocks_activation: bool,
    pub feature_flag_remains_disabled_in_sample: bool,
    pub no_live_context_attachment: bool,
    pub no_provider_model_invocation: bool,
    pub no_reply_delivery: bool,
    pub no_external_network_read: bool,
    pub no_production_memory_mutation: bool,
    pub no_raw_private_memory_logged: bool,
}

impl MemoryActivationCutoverChecks {
    pub fn ready(&self) -> bool {
        self.installed_telemetry_gate_ready
            && self.release_checklist_complete
            && self.canary_stage_order_valid
            && self.kill_switch_ready
            && self.rollback_drill_ready
            && self.approved_decision_allows_staged_rollout
            && self.missing_approval_blocks_activation
            && self.feature_flag_remains_disabled_in_sample
            && self.no_live_context_attachment
            && self.no_provider_model_invocation
            && self.no_reply_delivery
            && self.no_external_network_read
            && self.no_production_memory_mutation
            && self.no_raw_private_memory_logged
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryActivationCutoverGateReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub p14_activation_cutover_gate_ready: bool,
    pub native_rewrite: bool,
    pub sample_run: bool,
    pub installed_telemetry_contract: &'static str,
    pub feature_flag_id: &'static str,
    pub release_check_count: usize,
    pub canary_stage_count: usize,
    pub policy: MemoryActivationCutoverPolicy,
    pub release_checklist: Vec<MemoryReleaseChecklistItem>,
    pub canary_stages: Vec<MemoryCanaryStage>,
    pub kill_switch_plan: MemoryKillSwitchPlan,
    pub approved_cutover_decision: MemoryActivationCutoverDecision,
    pub missing_approval_decision: MemoryActivationCutoverDecision,
    pub checks: MemoryActivationCutoverChecks,
    pub next_phase: &'static str,
}

pub fn memory_activation_cutover_gate_sample_report(
    sample_run: bool,
) -> MemoryActivationCutoverGateReport {
    let telemetry = memory_installed_telemetry_gate_sample_report(true);
    let feature_flag_id = telemetry.feature_flag_id;
    let policy = MemoryActivationCutoverPolicy {
        policy_id: "memory-activation-cutover-policy-v1",
        feature_flag_id,
        require_installed_telemetry_gate: true,
        require_manual_release_approval: true,
        require_release_checklist: true,
        require_canary_stage_order: true,
        require_kill_switch: true,
        require_rollback_drill: true,
        block_automatic_enablement: true,
    };
    let release_checklist = vec![
        checklist_item(
            "p13-telemetry-ready",
            "P13 installed telemetry gate is ready",
            telemetry.p13_installed_telemetry_gate_ready,
            telemetry.contract,
        ),
        checklist_item(
            "flag-default-off",
            "live memory context feature flag remains disabled by default",
            !telemetry
                .installed_runtime_witness
                .feature_flag_default_enabled,
            feature_flag_id,
        ),
        checklist_item(
            "redacted-request-hashes",
            "provider request hashes are redacted and source-citation preserving",
            telemetry.checks.provider_request_redacted,
            "memory-provider-request-redaction",
        ),
        checklist_item(
            "telemetry-hash-chain",
            "telemetry records form a complete hash chain",
            telemetry.checks.telemetry_hash_chain_complete,
            last_hash(&telemetry.telemetry_hash_chain),
        ),
        checklist_item(
            "rollback-replay",
            "rollback replay restores the no-memory provider turn",
            telemetry.checks.rollback_restores_no_memory_turn,
            &telemetry.rollback_replay.rollback_receipt_id,
        ),
        checklist_item(
            "kill-switch-declared",
            "kill switch restores the no-memory provider request hash",
            true,
            &telemetry.rollback_replay.no_memory_provider_turn_hash,
        ),
        checklist_item(
            "operator-cutover-required",
            "manual operator release approval is required before any staged rollout",
            true,
            "operator-release-approval-required",
        ),
    ];
    let canary_stages = vec![
        MemoryCanaryStage {
            stage_id: "stage-0-shadow-readback",
            traffic_percent_ppm: 0,
            max_attached_node_count: telemetry.rollback_replay.dropped_staged_context_node_count,
            telemetry_hash_chain_required: true,
            rollback_on_failure: true,
            live_prompt_attachment_performed: false,
            provider_invoked: false,
        },
        MemoryCanaryStage {
            stage_id: "stage-1-operator-visible-canary",
            traffic_percent_ppm: 10_000,
            max_attached_node_count: telemetry.rollback_replay.dropped_staged_context_node_count,
            telemetry_hash_chain_required: true,
            rollback_on_failure: true,
            live_prompt_attachment_performed: false,
            provider_invoked: false,
        },
        MemoryCanaryStage {
            stage_id: "stage-2-opt-in-session-canary",
            traffic_percent_ppm: 100_000,
            max_attached_node_count: telemetry.rollback_replay.dropped_staged_context_node_count,
            telemetry_hash_chain_required: true,
            rollback_on_failure: true,
            live_prompt_attachment_performed: false,
            provider_invoked: false,
        },
    ];
    let kill_switch_plan = MemoryKillSwitchPlan {
        kill_switch_id: stable_digest(&format!(
            "memory-kill-switch:{}:{}",
            feature_flag_id, telemetry.rollback_replay.rollback_receipt_id
        )),
        feature_flag_id,
        disables_context_attachment: true,
        restores_no_memory_provider_turn_hash: telemetry
            .rollback_replay
            .no_memory_provider_turn_hash
            .clone(),
        rollback_receipt_id: telemetry.rollback_replay.rollback_receipt_id.clone(),
        telemetry_drain_required: true,
        mutation_performed: false,
    };
    let approved_cutover_decision = cutover_decision(
        "approved",
        true,
        true,
        false,
        false,
        "manual approval permits staged rollout planning but sample does not mutate the feature flag",
    );
    let missing_approval_decision = cutover_decision(
        "missing-approval",
        false,
        false,
        false,
        false,
        "manual operator approval is absent, so activation is blocked",
    );
    let checks = MemoryActivationCutoverChecks {
        installed_telemetry_gate_ready: policy.require_installed_telemetry_gate
            && telemetry.p13_installed_telemetry_gate_ready,
        release_checklist_complete: policy.require_release_checklist
            && release_checklist
                .iter()
                .all(|item| item.required && item.satisfied && !item.evidence_id.trim().is_empty()),
        canary_stage_order_valid: policy.require_canary_stage_order
            && canary_stages
                .windows(2)
                .all(|pair| pair[0].traffic_percent_ppm <= pair[1].traffic_percent_ppm)
            && canary_stages
                .iter()
                .all(|stage| stage.telemetry_hash_chain_required && stage.rollback_on_failure),
        kill_switch_ready: policy.require_kill_switch
            && kill_switch_plan.disables_context_attachment
            && kill_switch_plan.restores_no_memory_provider_turn_hash
                == telemetry.rollback_replay.no_memory_provider_turn_hash
            && !kill_switch_plan.mutation_performed,
        rollback_drill_ready: policy.require_rollback_drill
            && telemetry.rollback_replay.replayed
            && telemetry.rollback_replay.restored_feature_flag_default,
        approved_decision_allows_staged_rollout: approved_cutover_decision
            .operator_release_approved
            && approved_cutover_decision.activation_allowed
            && approved_cutover_decision.staged_rollout_allowed,
        missing_approval_blocks_activation: !missing_approval_decision.operator_release_approved
            && !missing_approval_decision.activation_allowed
            && !missing_approval_decision.staged_rollout_allowed,
        feature_flag_remains_disabled_in_sample: policy.block_automatic_enablement
            && !approved_cutover_decision.feature_flag_currently_enabled
            && !approved_cutover_decision.feature_flag_mutated
            && !missing_approval_decision.feature_flag_mutated,
        no_live_context_attachment: !approved_cutover_decision.context_attached_to_live_prompt
            && !missing_approval_decision.context_attached_to_live_prompt
            && canary_stages
                .iter()
                .all(|stage| !stage.live_prompt_attachment_performed),
        no_provider_model_invocation: !approved_cutover_decision.provider_invoked
            && !missing_approval_decision.provider_invoked
            && canary_stages.iter().all(|stage| !stage.provider_invoked),
        no_reply_delivery: telemetry.checks.no_reply_delivery,
        no_external_network_read: telemetry.checks.no_external_network_read,
        no_production_memory_mutation: telemetry.checks.no_production_memory_mutation
            && !kill_switch_plan.mutation_performed,
        no_raw_private_memory_logged: telemetry.checks.no_raw_private_memory_logged,
    };
    let p14_activation_cutover_gate_ready = checks.ready();

    MemoryActivationCutoverGateReport {
        product: "Hepta",
        command: "memory-activation-cutover-gate",
        contract: MEMORY_ACTIVATION_CUTOVER_GATE_V1_CONTRACT,
        status: if p14_activation_cutover_gate_ready {
            "ready"
        } else {
            "attention"
        },
        p14_activation_cutover_gate_ready,
        native_rewrite: true,
        sample_run,
        installed_telemetry_contract: MEMORY_INSTALLED_TELEMETRY_GATE_V1_CONTRACT,
        feature_flag_id,
        release_check_count: release_checklist.len(),
        canary_stage_count: canary_stages.len(),
        policy,
        release_checklist,
        canary_stages,
        kill_switch_plan,
        approved_cutover_decision,
        missing_approval_decision,
        checks,
        next_phase: "wire live memory context activation to the runtime provider router only behind this release gate, with staged canary telemetry and an immediate kill switch",
    }
}

fn checklist_item(
    item_id: &'static str,
    description: &'static str,
    satisfied: bool,
    evidence: &str,
) -> MemoryReleaseChecklistItem {
    MemoryReleaseChecklistItem {
        item_id,
        description,
        required: true,
        satisfied,
        evidence_id: evidence.to_string(),
    }
}

fn cutover_decision(
    suffix: &str,
    operator_release_approved: bool,
    activation_allowed: bool,
    feature_flag_currently_enabled: bool,
    feature_flag_mutated: bool,
    decision_reason: &'static str,
) -> MemoryActivationCutoverDecision {
    MemoryActivationCutoverDecision {
        decision_id: stable_digest(&format!("memory-cutover-decision:{suffix}")),
        release_channel: "local_operator_canary",
        operator_release_approved,
        activation_allowed,
        staged_rollout_allowed: operator_release_approved && activation_allowed,
        feature_flag_currently_enabled,
        feature_flag_mutated,
        context_attached_to_live_prompt: false,
        provider_invoked: false,
        decision_reason,
    }
}

fn last_hash(records: &[crate::MemoryTelemetryHashChainRecord]) -> &str {
    records
        .last()
        .map(|record| record.record_hash.as_str())
        .unwrap_or("missing-telemetry-hash")
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
    fn memory_activation_cutover_gate_sample_gate_is_ready() {
        let report = memory_activation_cutover_gate_sample_report(true);

        assert_eq!(report.status, "ready");
        assert!(report.p14_activation_cutover_gate_ready);
        assert!(report.checks.ready());
        assert_eq!(report.release_check_count, 7);
        assert_eq!(report.canary_stage_count, 3);
        assert_eq!(report.feature_flag_id, "HEPTA_MEMORY_CONTEXT_LIVE_TURN");
    }

    #[test]
    fn memory_activation_cutover_requires_operator_approval() {
        let report = memory_activation_cutover_gate_sample_report(true);

        assert!(report.approved_cutover_decision.activation_allowed);
        assert!(report.approved_cutover_decision.staged_rollout_allowed);
        assert!(!report.missing_approval_decision.activation_allowed);
        assert!(!report.missing_approval_decision.staged_rollout_allowed);
        assert!(report.checks.missing_approval_blocks_activation);
    }

    #[test]
    fn memory_activation_cutover_keeps_sample_disabled_and_rollback_ready() {
        let report = memory_activation_cutover_gate_sample_report(true);

        assert!(report.checks.feature_flag_remains_disabled_in_sample);
        assert!(report.checks.kill_switch_ready);
        assert!(report.checks.rollback_drill_ready);
        assert!(!report.approved_cutover_decision.feature_flag_mutated);
        assert!(!report.kill_switch_plan.mutation_performed);
        assert!(
            report
                .canary_stages
                .iter()
                .all(|stage| !stage.live_prompt_attachment_performed && !stage.provider_invoked)
        );
    }
}
