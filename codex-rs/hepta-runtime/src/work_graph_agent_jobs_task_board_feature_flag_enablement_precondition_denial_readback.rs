use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_canary_readback_replay::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DRY_RUN_GATE,
    hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_report,
};
use crate::work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_NON_SEND_READBACK_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_GATE;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_trace_guardrail_span_report_only::WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_READBACK_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_READBACK_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialReadbackReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_enablement_precondition_gate: &'static str,
    pub source_decision_count: usize,
    pub source_deny_reason_count: usize,
    pub source_allow_count: usize,
    pub source_deny_count: usize,
    pub denial_readback_entry_count: usize,
    pub denial_readback_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub denial_readback_scope: WorkGraphFeatureFlagEnablementPreconditionDenialReadbackScopePreview,
    pub denial_readback_entries:
        Vec<WorkGraphFeatureFlagEnablementPreconditionDenialReadbackEntryPreview>,
    pub denial_readback_blockers:
        Vec<WorkGraphFeatureFlagEnablementPreconditionDenialReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub dry_run_denial_visible: bool,
    pub dry_run_denial_recorded: bool,
    pub dry_run_denial_persisted: bool,
    pub dry_run_denial_accepted: bool,
    pub dry_run_denial_authoritative: bool,
    pub denial_readback_persisted: bool,
    pub denial_readback_authorizes_config_write: bool,
    pub denial_readback_authorizes_feature_flag_enablement: bool,
    pub denial_readback_authorizes_canary_traffic: bool,
    pub denial_readback_authorizes_live_cutover: bool,
    pub approval_recorded: bool,
    pub approval_acceptance_allowed: bool,
    pub ready_for_denial_audit_index: bool,
    pub ready_for_feature_flag_config_write: bool,
    pub ready_for_feature_flag_enablement: bool,
    pub ready_for_canary_traffic: bool,
    pub ready_for_live_cutover: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagEnablementPreconditionDenialReadbackScopePreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub readback_mode: &'static str,
    pub stable_readback_key: &'static str,
    pub denial_visible: bool,
    pub denial_recorded: bool,
    pub denial_persisted: bool,
    pub denial_accepted: bool,
    pub denial_authoritative: bool,
    pub readback_persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagEnablementPreconditionDenialReadbackEntryPreview {
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
pub struct WorkGraphFeatureFlagEnablementPreconditionDenialReadbackBlockerPreview {
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialReadbackSideEffects {
    pub filesystem_written: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_recorded: bool,
    pub operator_packet_persisted: bool,
    pub operator_packet_accepted: bool,
    pub approval_recorded: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_report()
-> WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialReadbackReport {
    let source =
        hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_report(
        );
    let denial_readback_scope =
        work_graph_agent_jobs_task_board_feature_flag_enablement_denial_readback_scope();
    let denial_readback_entries =
        work_graph_agent_jobs_task_board_feature_flag_enablement_denial_readback_entries();
    let denial_readback_blockers =
        work_graph_agent_jobs_task_board_feature_flag_enablement_denial_readback_blockers();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_feature_flag_enablement_denial_readback_required_prior_gates(
        );

    WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialReadbackReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_READBACK_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_READBACK_SCHEMA_VERSION,
        preview_mode: "enablement_precondition_denial_readback_only_no_accept_no_record_no_persistence",
        source_enablement_precondition_gate: source.gate,
        source_decision_count: source.decision_count,
        source_deny_reason_count: source.deny_reason_count,
        source_allow_count: source.allow_count,
        source_deny_count: source.deny_count,
        denial_readback_entry_count: denial_readback_entries.len(),
        denial_readback_blocker_count: denial_readback_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        denial_readback_scope,
        denial_readback_entries,
        denial_readback_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_READBACK_RECOMMENDED_NEXT_GATE,
        dry_run_denial_visible: true,
        dry_run_denial_recorded: false,
        dry_run_denial_persisted: false,
        dry_run_denial_accepted: false,
        dry_run_denial_authoritative: false,
        denial_readback_persisted: false,
        denial_readback_authorizes_config_write: false,
        denial_readback_authorizes_feature_flag_enablement: false,
        denial_readback_authorizes_canary_traffic: false,
        denial_readback_authorizes_live_cutover: false,
        approval_recorded: false,
        approval_acceptance_allowed: false,
        ready_for_denial_audit_index: true,
        ready_for_feature_flag_config_write: false,
        ready_for_feature_flag_enablement: false,
        ready_for_canary_traffic: false,
        ready_for_live_cutover: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialReadbackSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_enablement_denial_readback_scope()
-> WorkGraphFeatureFlagEnablementPreconditionDenialReadbackScopePreview {
    WorkGraphFeatureFlagEnablementPreconditionDenialReadbackScopePreview {
        id: "agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_scope",
        source_surface_id: "work_graph_agent_jobs_task_board.feature_flag.enablement_precondition_dry_run",
        readback_mode: "enablement_precondition_denial_readback_only",
        stable_readback_key: "work_graph.agent_jobs_task_board.feature_flag.enablement_precondition.denial_readback",
        denial_visible: true,
        denial_recorded: false,
        denial_persisted: false,
        denial_accepted: false,
        denial_authoritative: false,
        readback_persisted: false,
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_enablement_denial_readback_entries()
-> Vec<WorkGraphFeatureFlagEnablementPreconditionDenialReadbackEntryPreview> {
    vec![
        denial_readback_entry(
            "enablement_deny_decision_readback",
            "enablement_precondition_decisions_deny_both_canary_flags",
            "two_deny_decisions_visible_without_record_accept_or_persistence",
        ),
        denial_readback_entry(
            "enablement_deny_reason_catalog_readback",
            "enablement_precondition_deny_reasons_visible",
            "ten_deny_reasons_visible_unsatisfied_and_non_authoritative",
        ),
        denial_readback_entry(
            "feature_flag_current_off_readback",
            "feature_flag_current_off_zero_ppm_readback",
            "both_canary_flags_current_off_with_allowed_traffic_zero_ppm",
        ),
        denial_readback_entry(
            "scheduler_replay_rollback_boundary_readback",
            "scheduler_replay_rollback_still_preview_only",
            "scheduler_admission_replay_and_rollback_remain_unexecuted",
        ),
        denial_readback_entry(
            "live_cutover_denial_boundary_readback",
            "live_cutover_denied_by_precondition_readback",
            "denial_readback_does_not_authorize_config_write_enablement_traffic_or_cutover",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_enablement_denial_readback_blockers()
-> Vec<WorkGraphFeatureFlagEnablementPreconditionDenialReadbackBlockerPreview> {
    vec![
        denial_readback_blocker(
            "enablement_decision_acceptance_blocked",
            "accept_enablement_decision",
            "denial readback is not an operator approval or acceptance record",
        ),
        denial_readback_blocker(
            "approval_record_blocked",
            "record_operator_approval",
            "no approval recording is allowed by denial readback",
        ),
        denial_readback_blocker(
            "denial_readback_persistence_blocked",
            "persist_denial_readback",
            "denial readback remains stdout/report-only and unpersisted",
        ),
        denial_readback_blocker(
            "feature_flag_config_write_blocked",
            "write_feature_flag_config",
            "config writes remain disabled after denial readback",
        ),
        denial_readback_blocker(
            "feature_flag_enablement_blocked",
            "enable_feature_flag",
            "feature flags remain current off after denial readback",
        ),
        denial_readback_blocker(
            "canary_traffic_blocked",
            "route_canary_traffic",
            "canary traffic remains 0ppm after denial readback",
        ),
        denial_readback_blocker(
            "scheduler_enforcement_blocked",
            "enforce_scheduler_admission",
            "scheduler admission remains dry-run only",
        ),
        denial_readback_blocker(
            "replay_execution_blocked",
            "execute_replay",
            "replay remains a required future gate and is not executed",
        ),
        denial_readback_blocker(
            "rollback_execution_blocked",
            "execute_rollback",
            "rollback rehearsal remains a required future gate and is not executed",
        ),
        denial_readback_blocker(
            "live_cutover_blocked",
            "perform_live_cutover",
            "live cutover remains disabled after denial readback",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_enablement_denial_readback_required_prior_gates()
-> Vec<&'static str> {
    vec![
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

impl WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            operator_packet_sent: false,
            operator_packet_recorded: false,
            operator_packet_persisted: false,
            operator_packet_accepted: false,
            approval_recorded: false,
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

fn denial_readback_entry(
    id: &'static str,
    stable_readback_key: &'static str,
    observed_state: &'static str,
) -> WorkGraphFeatureFlagEnablementPreconditionDenialReadbackEntryPreview {
    WorkGraphFeatureFlagEnablementPreconditionDenialReadbackEntryPreview {
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

fn denial_readback_blocker(
    id: &'static str,
    blocked_action: &'static str,
    reason: &'static str,
) -> WorkGraphFeatureFlagEnablementPreconditionDenialReadbackBlockerPreview {
    WorkGraphFeatureFlagEnablementPreconditionDenialReadbackBlockerPreview {
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
    fn enablement_denial_readback_derives_from_deny_only_dry_run() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_report(
            );

        assert_eq!(
            report.source_enablement_precondition_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DRY_RUN_GATE
        );
        assert_eq!(report.source_decision_count, 2);
        assert_eq!(report.source_deny_reason_count, 10);
        assert_eq!(report.source_allow_count, 0);
        assert_eq!(report.source_deny_count, 2);
        assert!(report.dry_run_denial_visible);
        assert!(!report.dry_run_denial_recorded);
        assert!(!report.dry_run_denial_persisted);
        assert!(!report.dry_run_denial_accepted);
        assert!(!report.dry_run_denial_authoritative);
    }

    #[test]
    fn enablement_denial_readback_entries_are_visible_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_report(
            );

        assert_eq!(report.denial_readback_entry_count, 5);
        assert_eq!(
            report.denial_readback_scope.readback_mode,
            "enablement_precondition_denial_readback_only"
        );
        assert!(report.denial_readback_scope.denial_visible);
        assert!(!report.denial_readback_scope.denial_recorded);
        assert!(!report.denial_readback_scope.denial_persisted);
        assert!(!report.denial_readback_scope.denial_accepted);
        assert!(!report.denial_readback_scope.denial_authoritative);
        assert!(!report.denial_readback_scope.readback_persisted);
        assert!(report.denial_readback_entries.iter().all(|entry| {
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
    fn enablement_denial_readback_blocks_enablement_and_requires_priors() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_report(
            );

        assert_eq!(report.denial_readback_blocker_count, 10);
        assert!(
            report
                .denial_readback_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
        assert_eq!(
            report.required_prior_gates,
            vec![
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
        assert_eq!(report.required_prior_gate_count, 10);
        assert!(report.ready_for_denial_audit_index);
        assert!(!report.denial_readback_authorizes_config_write);
        assert!(!report.denial_readback_authorizes_feature_flag_enablement);
        assert!(!report.denial_readback_authorizes_canary_traffic);
        assert!(!report.denial_readback_authorizes_live_cutover);
        assert!(!report.approval_recorded);
        assert!(!report.approval_acceptance_allowed);
        assert!(!report.ready_for_feature_flag_config_write);
        assert!(!report.ready_for_feature_flag_enablement);
        assert!(!report.ready_for_canary_traffic);
        assert!(!report.ready_for_live_cutover);
    }

    #[test]
    fn enablement_denial_readback_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_report(
            );

        assert!(!report.denial_readback_persisted);
        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialReadbackSideEffects::none()
        );
    }
}
