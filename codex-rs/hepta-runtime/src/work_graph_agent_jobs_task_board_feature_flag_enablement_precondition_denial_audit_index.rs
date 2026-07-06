use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_canary_readback_replay::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_READBACK_GATE,
    hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_report,
};
use crate::work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DRY_RUN_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_NON_SEND_READBACK_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_GATE;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_trace_guardrail_span_report_only::WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_denial_readback_gate: &'static str,
    pub source_denial_readback_entry_count: usize,
    pub source_denial_readback_blocker_count: usize,
    pub source_required_prior_gate_count: usize,
    pub audit_index_entry_count: usize,
    pub audit_index_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub audit_index_scope: WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexScopePreview,
    pub audit_index_entries:
        Vec<WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexEntryPreview>,
    pub audit_index_blockers:
        Vec<WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub audit_index_visible: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub audit_index_authoritative: bool,
    pub audit_index_acceptance_allowed: bool,
    pub audit_index_authorizes_config_write: bool,
    pub audit_index_authorizes_feature_flag_enablement: bool,
    pub audit_index_authorizes_canary_traffic: bool,
    pub audit_index_authorizes_scheduler_enforcement: bool,
    pub audit_index_authorizes_replay_execution: bool,
    pub audit_index_authorizes_rollback_execution: bool,
    pub audit_index_authorizes_live_cutover: bool,
    pub ready_for_non_persistence_readback: bool,
    pub ready_for_feature_flag_config_write: bool,
    pub ready_for_feature_flag_enablement: bool,
    pub ready_for_canary_traffic: bool,
    pub ready_for_live_cutover: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexScopePreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub index_mode: &'static str,
    pub stable_index_key: &'static str,
    pub index_visible: bool,
    pub index_recorded: bool,
    pub index_persisted: bool,
    pub index_authoritative: bool,
    pub acceptance_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexEntryPreview {
    pub id: &'static str,
    pub stable_index_key: &'static str,
    pub source_readback_id: &'static str,
    pub audit_category: &'static str,
    pub indexed: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub authoritative: bool,
    pub mutation_allowed: bool,
    pub ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexBlockerPreview {
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexSideEffects {
    pub filesystem_written: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_recorded: bool,
    pub operator_packet_persisted: bool,
    pub operator_packet_accepted: bool,
    pub approval_recorded: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub config_written: bool,
    pub feature_flag_mutated: bool,
    pub non_blocking_canary_enabled: bool,
    pub canary_traffic_routed: bool,
    pub live_cutover_enabled: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub projection_index_persisted: bool,
    pub config_digest_persisted: bool,
    pub scheduler_admission_enforced: bool,
    pub guardrail_enforcement_enabled: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_report()
-> WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexReport {
    let source =
        hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_report(
        );
    let audit_index_scope =
        work_graph_agent_jobs_task_board_feature_flag_enablement_denial_audit_index_scope();
    let audit_index_entries =
        work_graph_agent_jobs_task_board_feature_flag_enablement_denial_audit_index_entries();
    let audit_index_blockers =
        work_graph_agent_jobs_task_board_feature_flag_enablement_denial_audit_index_blockers();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_feature_flag_enablement_denial_audit_index_required_prior_gates(
        );

    WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_SCHEMA_VERSION,
        preview_mode: "enablement_precondition_denial_audit_index_no_record_no_persistence_no_acceptance",
        source_denial_readback_gate: source.gate,
        source_denial_readback_entry_count: source.denial_readback_entry_count,
        source_denial_readback_blocker_count: source.denial_readback_blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        audit_index_entry_count: audit_index_entries.len(),
        audit_index_blocker_count: audit_index_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        audit_index_scope,
        audit_index_entries,
        audit_index_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_RECOMMENDED_NEXT_GATE,
        audit_index_visible: true,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_authoritative: false,
        audit_index_acceptance_allowed: false,
        audit_index_authorizes_config_write: false,
        audit_index_authorizes_feature_flag_enablement: false,
        audit_index_authorizes_canary_traffic: false,
        audit_index_authorizes_scheduler_enforcement: false,
        audit_index_authorizes_replay_execution: false,
        audit_index_authorizes_rollback_execution: false,
        audit_index_authorizes_live_cutover: false,
        ready_for_non_persistence_readback: true,
        ready_for_feature_flag_config_write: false,
        ready_for_feature_flag_enablement: false,
        ready_for_canary_traffic: false,
        ready_for_live_cutover: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_enablement_denial_audit_index_scope()
-> WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexScopePreview {
    WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexScopePreview {
        id: "agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_scope",
        source_surface_id: "work_graph_agent_jobs_task_board.feature_flag.enablement_precondition_denial_readback",
        index_mode: "denial_audit_index_report_only",
        stable_index_key: "work_graph.agent_jobs_task_board.feature_flag.enablement_precondition.denial_audit_index",
        index_visible: true,
        index_recorded: false,
        index_persisted: false,
        index_authoritative: false,
        acceptance_allowed: false,
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_enablement_denial_audit_index_entries()
-> Vec<WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexEntryPreview> {
    vec![
        audit_index_entry(
            "deny_decision_audit_index",
            "denial_audit_index.deny_decisions",
            "enablement_deny_decision_readback",
            "enablement_decision",
        ),
        audit_index_entry(
            "deny_reason_catalog_audit_index",
            "denial_audit_index.deny_reason_catalog",
            "enablement_deny_reason_catalog_readback",
            "deny_reason_catalog",
        ),
        audit_index_entry(
            "feature_flag_current_off_audit_index",
            "denial_audit_index.feature_flag_current_off",
            "feature_flag_current_off_readback",
            "feature_flag_boundary",
        ),
        audit_index_entry(
            "scheduler_replay_rollback_audit_index",
            "denial_audit_index.scheduler_replay_rollback_boundaries",
            "scheduler_replay_rollback_boundary_readback",
            "scheduler_replay_rollback_boundary",
        ),
        audit_index_entry(
            "live_cutover_denial_audit_index",
            "denial_audit_index.live_cutover_denial",
            "live_cutover_denial_boundary_readback",
            "live_cutover_boundary",
        ),
        audit_index_entry(
            "required_prior_chain_audit_index",
            "denial_audit_index.required_prior_chain",
            "enablement_precondition_denial_readback_gate",
            "required_prior_chain",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_enablement_denial_audit_index_blockers()
-> Vec<WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexBlockerPreview> {
    vec![
        audit_index_blocker(
            "audit_index_record_blocked",
            "record_denial_audit_index",
            "denial audit index remains report-only and unrecorded",
        ),
        audit_index_blocker(
            "audit_index_persistence_blocked",
            "persist_denial_audit_index",
            "denial audit index is not written to WorkGraph or projection storage",
        ),
        audit_index_blocker(
            "audit_index_acceptance_blocked",
            "accept_denial_audit_index",
            "audit index does not create operator acceptance",
        ),
        audit_index_blocker(
            "approval_record_blocked",
            "record_operator_approval",
            "no approval record may be created from an audit index",
        ),
        audit_index_blocker(
            "feature_flag_config_write_blocked",
            "write_feature_flag_config",
            "feature-flag config writes remain disabled",
        ),
        audit_index_blocker(
            "feature_flag_enablement_blocked",
            "enable_feature_flag",
            "feature flags remain current off",
        ),
        audit_index_blocker(
            "canary_traffic_blocked",
            "route_canary_traffic",
            "canary traffic remains 0ppm",
        ),
        audit_index_blocker(
            "scheduler_enforcement_blocked",
            "enforce_scheduler_admission",
            "scheduler admission remains dry-run only",
        ),
        audit_index_blocker(
            "replay_execution_blocked",
            "execute_replay",
            "replay remains unexecuted",
        ),
        audit_index_blocker(
            "rollback_execution_blocked",
            "execute_rollback",
            "rollback remains unexecuted",
        ),
        audit_index_blocker(
            "live_cutover_blocked",
            "perform_live_cutover",
            "live cutover remains disabled",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_enablement_denial_audit_index_required_prior_gates()
-> Vec<&'static str> {
    vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_READBACK_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DRY_RUN_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_NON_SEND_READBACK_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE,
        WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
        WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
    ]
}

impl WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            operator_packet_sent: false,
            operator_packet_recorded: false,
            operator_packet_persisted: false,
            operator_packet_accepted: false,
            approval_recorded: false,
            audit_index_recorded: false,
            audit_index_persisted: false,
            config_written: false,
            feature_flag_mutated: false,
            non_blocking_canary_enabled: false,
            canary_traffic_routed: false,
            live_cutover_enabled: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            projection_index_persisted: false,
            config_digest_persisted: false,
            scheduler_admission_enforced: false,
            guardrail_enforcement_enabled: false,
            replay_executed: false,
            rollback_executed: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn audit_index_entry(
    id: &'static str,
    stable_index_key: &'static str,
    source_readback_id: &'static str,
    audit_category: &'static str,
) -> WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexEntryPreview {
    WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexEntryPreview {
        id,
        stable_index_key,
        source_readback_id,
        audit_category,
        indexed: true,
        recorded: false,
        persisted: false,
        authoritative: false,
        mutation_allowed: false,
        ready: true,
    }
}

fn audit_index_blocker(
    id: &'static str,
    blocked_action: &'static str,
    reason: &'static str,
) -> WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexBlockerPreview {
    WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn denial_audit_index_derives_from_denial_readback() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_report(
            );

        assert_eq!(
            report.source_denial_readback_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_READBACK_GATE
        );
        assert_eq!(report.source_denial_readback_entry_count, 5);
        assert_eq!(report.source_denial_readback_blocker_count, 10);
        assert_eq!(report.source_required_prior_gate_count, 10);
        assert_eq!(report.audit_index_entry_count, 6);
        assert_eq!(report.audit_index_blocker_count, 11);
    }

    #[test]
    fn denial_audit_index_entries_are_visible_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_report(
            );

        assert_eq!(
            report.audit_index_scope.index_mode,
            "denial_audit_index_report_only"
        );
        assert!(report.audit_index_scope.index_visible);
        assert!(!report.audit_index_scope.index_recorded);
        assert!(!report.audit_index_scope.index_persisted);
        assert!(!report.audit_index_scope.index_authoritative);
        assert!(!report.audit_index_scope.acceptance_allowed);
        assert!(report.audit_index_entries.iter().all(|entry| {
            entry.indexed
                && entry.ready
                && !entry.recorded
                && !entry.persisted
                && !entry.authoritative
                && !entry.mutation_allowed
        }));
    }

    #[test]
    fn denial_audit_index_blocks_live_paths_and_requires_priors() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_report(
            );

        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_READBACK_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DRY_RUN_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_NON_SEND_READBACK_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE,
                WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
                WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
            ]
        );
        assert_eq!(report.required_prior_gate_count, 11);
        assert!(
            report
                .audit_index_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
        assert!(report.ready_for_non_persistence_readback);
        assert!(!report.audit_index_authorizes_config_write);
        assert!(!report.audit_index_authorizes_feature_flag_enablement);
        assert!(!report.audit_index_authorizes_canary_traffic);
        assert!(!report.audit_index_authorizes_scheduler_enforcement);
        assert!(!report.audit_index_authorizes_replay_execution);
        assert!(!report.audit_index_authorizes_rollback_execution);
        assert!(!report.audit_index_authorizes_live_cutover);
        assert!(!report.ready_for_feature_flag_config_write);
        assert!(!report.ready_for_feature_flag_enablement);
        assert!(!report.ready_for_canary_traffic);
        assert!(!report.ready_for_live_cutover);
    }

    #[test]
    fn denial_audit_index_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_report(
            );

        assert!(!report.audit_index_recorded);
        assert!(!report.audit_index_persisted);
        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexSideEffects::none()
        );
    }
}
