use crate::live_readiness::evaluate_runtime_readiness;
use crate::live_readiness::live_adapter_activation_discipline_sample;
use serde::Serialize;

pub const CONTROLLED_LIVE_READINESS_AUDIT_GATE: &str = "controlled_live_readiness_audit_gate";
pub const CONTROLLED_LIVE_READINESS_AUDIT_SCHEMA_VERSION: &str =
    "controlled_live_readiness_audit_v1";
pub const CONTROLLED_LIVE_READINESS_AUDIT_RECOMMENDED_NEXT_GATE: &str =
    "phase5a_controlled_live_readiness_denial_readback_index_without_cutover";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveReadinessAuditReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub runtime_readiness_stage: String,
    pub audit_precondition_count: usize,
    pub satisfied_precondition_count: usize,
    pub blocking_precondition_count: usize,
    pub blocker_count: usize,
    pub dry_run_adapter_count: usize,
    pub dry_run_only_adapter_count: usize,
    pub manual_operator_live_cutover_approval_required: bool,
    pub controlled_live_audit_ready: bool,
    pub controlled_live_cutover_ready: bool,
    pub live_execution_allowed: bool,
    pub activation_allowed: bool,
    pub preconditions: Vec<ControlledLiveReadinessPrecondition>,
    pub blockers: Vec<ControlledLiveReadinessBlocker>,
    pub recommended_next_gate: &'static str,
    pub side_effects: ControlledLiveReadinessAuditSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveReadinessPrecondition {
    pub id: &'static str,
    pub layer: &'static str,
    pub evidence: &'static str,
    pub satisfied: bool,
    pub required_for_live: bool,
    pub blocks_cutover: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveReadinessBlocker {
    pub id: &'static str,
    pub reason: &'static str,
    pub operator_recoverable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveReadinessAuditSideEffects {
    pub plugin_installed: bool,
    pub plugin_cache_mutated: bool,
    pub tool_registered: bool,
    pub tool_invoked: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub approval_recorded: bool,
    pub receipt_persisted: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub lease_acquired: bool,
    pub idempotency_index_mutated: bool,
    pub checkpoint_written: bool,
    pub workflow_execution_started: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub rollback_rehearsal_recorded: bool,
    pub kill_switch_mutated: bool,
    pub credential_read: bool,
    pub native_post_mutation_performed: bool,
    pub gateway_or_auth_mutated: bool,
    pub telegram_transport_mutated: bool,
    pub channel_send_performed: bool,
    pub provider_invoked: bool,
    pub model_invoked: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub live_execution_started: bool,
}

pub fn controlled_live_readiness_audit_report() -> ControlledLiveReadinessAuditReport {
    let runtime_readiness = evaluate_runtime_readiness(
        "hepta-system-controlled-live-readiness",
        true,
        true,
        false,
        true,
        true,
        false,
    )
    .expect("static controlled-live readiness sample is valid");
    let dry_run_adapters = live_adapter_activation_discipline_sample()
        .expect("static live adapter discipline sample is valid");
    let dry_run_adapter_count = dry_run_adapters.len();
    let dry_run_only_adapter_count = dry_run_adapters
        .iter()
        .filter(|report| report.discipline_ready && !report.activation_permitted && report.dry_run)
        .count();
    let preconditions = controlled_live_readiness_preconditions();
    let blockers = controlled_live_readiness_blockers();
    let satisfied_precondition_count = preconditions
        .iter()
        .filter(|condition| condition.satisfied)
        .count();
    let blocking_precondition_count = preconditions
        .iter()
        .filter(|condition| condition.blocks_cutover)
        .count();
    let controlled_live_audit_ready = preconditions.len() == 12
        && satisfied_precondition_count == 5
        && blocking_precondition_count == 7
        && blockers.len() == 7
        && dry_run_adapter_count == 4
        && dry_run_only_adapter_count == 4;

    ControlledLiveReadinessAuditReport {
        runtime: "hepta",
        surface: "controlled_live_readiness_audit",
        status: if controlled_live_audit_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: CONTROLLED_LIVE_READINESS_AUDIT_GATE,
        schema_version: CONTROLLED_LIVE_READINESS_AUDIT_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        runtime_readiness_stage: runtime_readiness.stage.label().to_string(),
        audit_precondition_count: preconditions.len(),
        satisfied_precondition_count,
        blocking_precondition_count,
        blocker_count: blockers.len(),
        dry_run_adapter_count,
        dry_run_only_adapter_count,
        manual_operator_live_cutover_approval_required: true,
        controlled_live_audit_ready,
        controlled_live_cutover_ready: false,
        live_execution_allowed: false,
        activation_allowed: false,
        preconditions,
        blockers,
        recommended_next_gate: CONTROLLED_LIVE_READINESS_AUDIT_RECOMMENDED_NEXT_GATE,
        side_effects: ControlledLiveReadinessAuditSideEffects::none(),
    }
}

pub fn controlled_live_readiness_preconditions() -> Vec<ControlledLiveReadinessPrecondition> {
    vec![
        precondition(
            "single_source_of_truth_ready",
            "systems",
            "current reality matrix is ready and all live paths are blocked",
            true,
            true,
        ),
        precondition(
            "read_only_e2e_chain_ready",
            "e2e",
            "hepta-system status read-only E2E has four ready links",
            true,
            true,
        ),
        precondition(
            "temporal_lite_adapter_feature_gate_closed",
            "workflow",
            "workflow durable-store adapter is ready while the event-log feature gate is disabled",
            true,
            true,
        ),
        precondition(
            "replay_validation_metadata_present",
            "workflow",
            "Temporal-lite adapter carries replay validation metadata",
            true,
            true,
        ),
        precondition(
            "rollback_metadata_present",
            "workflow",
            "Temporal-lite adapter carries rollback anchor metadata",
            true,
            true,
        ),
        precondition(
            "clean_worktree_required",
            "release",
            "controlled live requires a clean worktree boundary before mutation",
            false,
            true,
        ),
        precondition(
            "explicit_operator_live_approval_recorded",
            "operator",
            "no explicit operator live approval packet is recorded",
            false,
            true,
        ),
        precondition(
            "fresh_soak_readback_evidence_recorded",
            "observability",
            "no fresh soak and readback evidence is recorded for this cutover",
            false,
            true,
        ),
        precondition(
            "credential_boundary_attestation_recorded",
            "security",
            "no credential boundary attestation is recorded",
            false,
            true,
        ),
        precondition(
            "gateway_native_telegram_post_boundary_approval_recorded",
            "transport",
            "Gateway, Native POST, and Telegram live mutation boundary approval is absent",
            false,
            true,
        ),
        precondition(
            "rollback_rehearsal_evidence_recorded",
            "rollback",
            "no rollback rehearsal evidence is recorded",
            false,
            true,
        ),
        precondition(
            "kill_switch_rehearsal_evidence_recorded",
            "rollback",
            "no kill-switch rehearsal evidence is recorded",
            false,
            true,
        ),
    ]
}

pub fn controlled_live_readiness_blockers() -> Vec<ControlledLiveReadinessBlocker> {
    vec![
        blocker(
            "dirty_worktree_boundary",
            "controlled live requires a clean scoped worktree boundary",
        ),
        blocker(
            "operator_live_approval_missing",
            "explicit operator live approval packet is missing",
        ),
        blocker(
            "fresh_soak_readback_missing",
            "fresh soak and readback evidence is missing",
        ),
        blocker(
            "credential_boundary_attestation_missing",
            "credential boundary attestation is missing",
        ),
        blocker(
            "gateway_native_telegram_post_boundary_approval_missing",
            "Gateway, Native POST, and Telegram live mutation approval is missing",
        ),
        blocker(
            "rollback_rehearsal_missing",
            "rollback rehearsal evidence is missing",
        ),
        blocker(
            "kill_switch_rehearsal_missing",
            "kill-switch rehearsal evidence is missing",
        ),
    ]
}

fn precondition(
    id: &'static str,
    layer: &'static str,
    evidence: &'static str,
    satisfied: bool,
    required_for_live: bool,
) -> ControlledLiveReadinessPrecondition {
    ControlledLiveReadinessPrecondition {
        id,
        layer,
        evidence,
        satisfied,
        required_for_live,
        blocks_cutover: required_for_live && !satisfied,
    }
}

fn blocker(id: &'static str, reason: &'static str) -> ControlledLiveReadinessBlocker {
    ControlledLiveReadinessBlocker {
        id,
        reason,
        operator_recoverable: true,
    }
}

impl ControlledLiveReadinessAuditSideEffects {
    pub const fn none() -> Self {
        Self {
            plugin_installed: false,
            plugin_cache_mutated: false,
            tool_registered: false,
            tool_invoked: false,
            ledger_written: false,
            approval_requested: false,
            approval_recorded: false,
            receipt_persisted: false,
            workflow_event_log_written: false,
            sqlite_written: false,
            lease_acquired: false,
            idempotency_index_mutated: false,
            checkpoint_written: false,
            workflow_execution_started: false,
            replay_executed: false,
            rollback_executed: false,
            rollback_rehearsal_recorded: false,
            kill_switch_mutated: false,
            credential_read: false,
            native_post_mutation_performed: false,
            gateway_or_auth_mutated: false,
            telegram_transport_mutated: false,
            channel_send_performed: false,
            provider_invoked: false,
            model_invoked: false,
            package_or_release_written: false,
            public_ga_promoted: false,
            live_execution_started: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controlled_live_readiness_audit_is_ready_but_cutover_blocked() {
        let report = controlled_live_readiness_audit_report();

        assert_eq!(report.status, "ready_blocked");
        assert_eq!(report.runtime_readiness_stage, "M2-local-adapter");
        assert_eq!(report.audit_precondition_count, 12);
        assert_eq!(report.satisfied_precondition_count, 5);
        assert_eq!(report.blocking_precondition_count, 7);
        assert_eq!(report.blocker_count, 7);
        assert!(report.controlled_live_audit_ready);
        assert!(!report.controlled_live_cutover_ready);
        assert!(!report.live_execution_allowed);
        assert!(!report.activation_allowed);
        assert!(report.manual_operator_live_cutover_approval_required);
    }

    #[test]
    fn controlled_live_readiness_keeps_live_adapters_dry_run_only() {
        let report = controlled_live_readiness_audit_report();

        assert_eq!(report.dry_run_adapter_count, 4);
        assert_eq!(report.dry_run_only_adapter_count, 4);
        assert_eq!(
            report.side_effects,
            ControlledLiveReadinessAuditSideEffects::none()
        );
    }

    #[test]
    fn controlled_live_readiness_names_required_blockers() {
        let report = controlled_live_readiness_audit_report();
        let blocker_ids = report
            .blockers
            .iter()
            .map(|blocker| blocker.id)
            .collect::<Vec<_>>();

        assert!(blocker_ids.contains(&"operator_live_approval_missing"));
        assert!(blocker_ids.contains(&"fresh_soak_readback_missing"));
        assert!(blocker_ids.contains(&"gateway_native_telegram_post_boundary_approval_missing"));
        assert!(blocker_ids.contains(&"rollback_rehearsal_missing"));
        assert!(blocker_ids.contains(&"kill_switch_rehearsal_missing"));
    }
}
