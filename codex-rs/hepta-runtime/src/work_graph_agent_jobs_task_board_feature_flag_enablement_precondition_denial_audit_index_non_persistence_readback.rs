use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_canary_readback_replay::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_GATE,
    hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_report,
};
use crate::work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_READBACK_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DRY_RUN_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_NON_SEND_READBACK_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_GATE;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_trace_guardrail_span_report_only::WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_NON_PERSISTENCE_READBACK_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_NON_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_denial_audit_index_gate: &'static str,
    pub source_audit_index_entry_count: usize,
    pub source_audit_index_blocker_count: usize,
    pub source_required_prior_gate_count: usize,
    pub readback_entry_count: usize,
    pub readback_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_scope:
        WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackScopePreview,
    pub readback_entries:
        Vec<WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackEntryPreview>,
    pub readback_blockers:
        Vec<WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub audit_index_visible: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub audit_index_authoritative: bool,
    pub audit_index_accepted: bool,
    pub readback_persisted: bool,
    pub operator_review_request_allowed: bool,
    pub approval_recorded: bool,
    pub config_write_allowed: bool,
    pub feature_flag_enablement_allowed: bool,
    pub canary_traffic_allowed: bool,
    pub scheduler_enforcement_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub live_cutover_allowed: bool,
    pub ready_for_operator_review_precondition_matrix: bool,
    pub ready_for_feature_flag_config_write: bool,
    pub ready_for_feature_flag_enablement: bool,
    pub ready_for_canary_traffic: bool,
    pub ready_for_live_cutover: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackScopePreview
{
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub readback_mode: &'static str,
    pub stable_readback_key: &'static str,
    pub audit_index_visible: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub audit_index_authoritative: bool,
    pub audit_index_accepted: bool,
    pub readback_persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackEntryPreview
{
    pub id: &'static str,
    pub stable_readback_key: &'static str,
    pub observed_state: &'static str,
    pub visible: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub accepted: bool,
    pub authoritative: bool,
    pub mutation_allowed: bool,
    pub ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackBlockerPreview
{
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackSideEffects
{
    pub filesystem_written: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_recorded: bool,
    pub operator_packet_persisted: bool,
    pub operator_packet_accepted: bool,
    pub approval_recorded: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub readback_persisted: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_report()
-> WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackReport{
    let source =
        hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_report(
        );
    let readback_scope =
        work_graph_agent_jobs_task_board_feature_flag_enablement_denial_audit_index_non_persistence_readback_scope();
    let readback_entries =
        work_graph_agent_jobs_task_board_feature_flag_enablement_denial_audit_index_non_persistence_readback_entries();
    let readback_blockers =
        work_graph_agent_jobs_task_board_feature_flag_enablement_denial_audit_index_non_persistence_readback_blockers();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_feature_flag_enablement_denial_audit_index_non_persistence_readback_required_prior_gates();

    WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_NON_PERSISTENCE_READBACK_SCHEMA_VERSION,
        preview_mode: "denial_audit_index_non_persistence_readback_no_record_no_acceptance_no_write",
        source_denial_audit_index_gate: source.gate,
        source_audit_index_entry_count: source.audit_index_entry_count,
        source_audit_index_blocker_count: source.audit_index_blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        readback_entry_count: readback_entries.len(),
        readback_blocker_count: readback_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_scope,
        readback_entries,
        readback_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_NON_PERSISTENCE_READBACK_RECOMMENDED_NEXT_GATE,
        audit_index_visible: true,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_authoritative: false,
        audit_index_accepted: false,
        readback_persisted: false,
        operator_review_request_allowed: false,
        approval_recorded: false,
        config_write_allowed: false,
        feature_flag_enablement_allowed: false,
        canary_traffic_allowed: false,
        scheduler_enforcement_allowed: false,
        replay_execution_allowed: false,
        rollback_execution_allowed: false,
        live_cutover_allowed: false,
        ready_for_operator_review_precondition_matrix: true,
        ready_for_feature_flag_config_write: false,
        ready_for_feature_flag_enablement: false,
        ready_for_canary_traffic: false,
        ready_for_live_cutover: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_enablement_denial_audit_index_non_persistence_readback_scope()
-> WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackScopePreview {
    WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackScopePreview {
        id: "agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_scope",
        source_surface_id: "work_graph_agent_jobs_task_board.feature_flag.enablement_precondition_denial_audit_index",
        readback_mode: "denial_audit_index_non_persistence_readback_only",
        stable_readback_key: "work_graph.agent_jobs_task_board.feature_flag.enablement_precondition.denial_audit_index.non_persistence_readback",
        audit_index_visible: true,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_authoritative: false,
        audit_index_accepted: false,
        readback_persisted: false,
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_enablement_denial_audit_index_non_persistence_readback_entries()
-> Vec<WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackEntryPreview>
{
    vec![
        readback_entry(
            "audit_index_surface_readback",
            "denial_audit_index_visible_unrecorded_unpersisted",
            "audit_index_visible_without_record_persist_accept_or_authority",
        ),
        readback_entry(
            "audit_index_prior_chain_readback",
            "denial_audit_index_required_priors_visible",
            "eleven_required_prior_gates_visible_but_not_persisted",
        ),
        readback_entry(
            "audit_index_blocker_readback",
            "denial_audit_index_blockers_visible",
            "eleven_blockers_visible_and_still_blocking",
        ),
        readback_entry(
            "audit_index_non_persistence_boundary_readback",
            "denial_audit_index_non_persistence_boundary",
            "audit_index_does_not_write_work_graph_projection_or_config_state",
        ),
        readback_entry(
            "audit_index_no_acceptance_boundary_readback",
            "denial_audit_index_no_acceptance_boundary",
            "audit_index_does_not_request_operator_review_or_acceptance",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_enablement_denial_audit_index_non_persistence_readback_blockers()
-> Vec<
    WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackBlockerPreview,
> {
    vec![
        readback_blocker("audit_index_record_blocked", "record_denial_audit_index"),
        readback_blocker(
            "audit_index_persistence_blocked",
            "persist_denial_audit_index",
        ),
        readback_blocker(
            "audit_index_acceptance_blocked",
            "accept_denial_audit_index",
        ),
        readback_blocker("operator_review_request_blocked", "request_operator_review"),
        readback_blocker("approval_record_blocked", "record_operator_approval"),
        readback_blocker(
            "feature_flag_config_write_blocked",
            "write_feature_flag_config",
        ),
        readback_blocker("feature_flag_enablement_blocked", "enable_feature_flag"),
        readback_blocker("canary_traffic_blocked", "route_canary_traffic"),
        readback_blocker(
            "scheduler_enforcement_blocked",
            "enforce_scheduler_admission",
        ),
        readback_blocker("replay_execution_blocked", "execute_replay"),
        readback_blocker("rollback_execution_blocked", "execute_rollback"),
        readback_blocker("live_cutover_blocked", "perform_live_cutover"),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_enablement_denial_audit_index_non_persistence_readback_required_prior_gates()
-> Vec<&'static str> {
    vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_GATE,
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

impl WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackSideEffects {
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
            readback_persisted: false,
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

fn readback_entry(
    id: &'static str,
    stable_readback_key: &'static str,
    observed_state: &'static str,
) -> WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackEntryPreview {
    WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackEntryPreview {
        id,
        stable_readback_key,
        observed_state,
        visible: true,
        recorded: false,
        persisted: false,
        accepted: false,
        authoritative: false,
        mutation_allowed: false,
        ready: true,
    }
}

fn readback_blocker(
    id: &'static str,
    blocked_action: &'static str,
) -> WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackBlockerPreview
{
    WorkGraphFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason: "non-persistence readback cannot authorize this action",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn denial_audit_index_non_persistence_readback_derives_from_audit_index() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_report();

        assert_eq!(
            report.source_denial_audit_index_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_GATE
        );
        assert_eq!(report.source_audit_index_entry_count, 6);
        assert_eq!(report.source_audit_index_blocker_count, 11);
        assert_eq!(report.source_required_prior_gate_count, 11);
        assert_eq!(report.readback_entry_count, 5);
        assert_eq!(report.readback_blocker_count, 12);
    }

    #[test]
    fn denial_audit_index_non_persistence_readback_entries_are_visible_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_report();

        assert_eq!(
            report.readback_scope.readback_mode,
            "denial_audit_index_non_persistence_readback_only"
        );
        assert!(report.readback_scope.audit_index_visible);
        assert!(!report.readback_scope.audit_index_recorded);
        assert!(!report.readback_scope.audit_index_persisted);
        assert!(!report.readback_scope.audit_index_authoritative);
        assert!(!report.readback_scope.audit_index_accepted);
        assert!(!report.readback_scope.readback_persisted);
        assert!(report.readback_entries.iter().all(|entry| {
            entry.visible
                && entry.ready
                && !entry.recorded
                && !entry.persisted
                && !entry.accepted
                && !entry.authoritative
                && !entry.mutation_allowed
        }));
    }

    #[test]
    fn denial_audit_index_non_persistence_readback_blocks_live_paths() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_report();

        assert_eq!(report.required_prior_gate_count, 12);
        assert!(
            report
                .readback_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
        assert!(report.ready_for_operator_review_precondition_matrix);
        assert!(!report.operator_review_request_allowed);
        assert!(!report.approval_recorded);
        assert!(!report.config_write_allowed);
        assert!(!report.feature_flag_enablement_allowed);
        assert!(!report.canary_traffic_allowed);
        assert!(!report.scheduler_enforcement_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_cutover_allowed);
        assert!(!report.ready_for_feature_flag_config_write);
        assert!(!report.ready_for_feature_flag_enablement);
        assert!(!report.ready_for_canary_traffic);
        assert!(!report.ready_for_live_cutover);
    }

    #[test]
    fn denial_audit_index_non_persistence_readback_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_report();

        assert!(!report.audit_index_recorded);
        assert!(!report.audit_index_persisted);
        assert!(!report.readback_persisted);
        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackSideEffects::none()
        );
    }
}
