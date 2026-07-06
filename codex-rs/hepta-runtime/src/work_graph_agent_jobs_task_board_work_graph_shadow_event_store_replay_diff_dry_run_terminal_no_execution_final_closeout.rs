use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_NON_EXECUTION_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
    hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_report,
};

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_TERMINAL_NO_EXECUTION_FINAL_CLOSEOUT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_TERMINAL_NO_EXECUTION_FINAL_CLOSEOUT_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_TERMINAL_NO_EXECUTION_FINAL_CLOSEOUT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_non_persistence_readback_gate: &'static str,
    pub source_readback_entry_count: usize,
    pub source_readback_blocker_count: usize,
    pub source_required_prior_gate_count: usize,
    pub final_closeout_entry_count: usize,
    pub final_closeout_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub final_closeout_scope:
        WorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutScopePreview,
    pub final_closeout_entries:
        Vec<WorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutEntryPreview>,
    pub final_closeout_blockers:
        Vec<WorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub terminal_no_execution_branch_closed: bool,
    pub final_closeout_visible: bool,
    pub final_closeout_recorded: bool,
    pub final_closeout_persisted: bool,
    pub final_closeout_authoritative: bool,
    pub final_closeout_accepted: bool,
    pub source_audit_index_visible: bool,
    pub source_audit_index_persisted: bool,
    pub source_readback_persisted: bool,
    pub readback_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub replay_diff_recording_allowed: bool,
    pub replay_diff_persistence_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub idempotency_mutation_allowed: bool,
    pub work_graph_event_persistence_allowed: bool,
    pub projection_persistence_allowed: bool,
    pub scheduler_guardrail_enforcement_allowed: bool,
    pub runtime_interception_allowed: bool,
    pub feature_flag_enablement_allowed: bool,
    pub canary_traffic_allowed: bool,
    pub operator_review_request_allowed: bool,
    pub approval_recording_allowed: bool,
    pub live_cutover_allowed: bool,
    pub ready_for_scheduler_guardrail_blocking_dry_run_entrypoint_hardening: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutScopePreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub closeout_mode: &'static str,
    pub stable_closeout_key: &'static str,
    pub visible: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub authoritative: bool,
    pub accepted: bool,
    pub terminal: bool,
    pub mutation_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutEntryPreview {
    pub id: &'static str,
    pub stable_closeout_key: &'static str,
    pub source_readback_id: &'static str,
    pub closeout_category: &'static str,
    pub visible: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub accepted: bool,
    pub authoritative: bool,
    pub mutation_allowed: bool,
    pub closed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutBlockerPreview {
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutSideEffects
{
    pub filesystem_written: bool,
    pub final_closeout_recorded: bool,
    pub final_closeout_persisted: bool,
    pub final_closeout_accepted: bool,
    pub audit_index_readback_recorded: bool,
    pub audit_index_readback_persisted: bool,
    pub audit_index_readback_accepted: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub audit_index_accepted: bool,
    pub non_execution_readback_executed: bool,
    pub non_execution_readback_recorded: bool,
    pub non_execution_readback_persisted: bool,
    pub replay_executed: bool,
    pub replay_diff_recorded: bool,
    pub replay_diff_persisted: bool,
    pub rollback_executed: bool,
    pub idempotency_index_mutated: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub shadow_event_persisted: bool,
    pub projection_index_persisted: bool,
    pub scheduler_admission_enforced: bool,
    pub guardrail_enforcement_enabled: bool,
    pub runtime_interception_enabled: bool,
    pub config_written: bool,
    pub feature_flag_mutated: bool,
    pub canary_traffic_routed: bool,
    pub operator_review_requested: bool,
    pub approval_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_report()
-> WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutReport{
    let source =
        hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_report();
    let final_closeout_scope =
        work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_scope();
    let final_closeout_entries =
        work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_entries();
    let final_closeout_blockers =
        work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_blockers();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_NON_EXECUTION_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
    ];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());

    WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_TERMINAL_NO_EXECUTION_FINAL_CLOSEOUT_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_TERMINAL_NO_EXECUTION_FINAL_CLOSEOUT_SCHEMA_VERSION,
        preview_mode:
            "work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_report_only",
        source_non_persistence_readback_gate: source.gate,
        source_readback_entry_count: source.readback_entry_count,
        source_readback_blocker_count: source.readback_blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        final_closeout_entry_count: final_closeout_entries.len(),
        final_closeout_blocker_count: final_closeout_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        final_closeout_scope,
        final_closeout_entries,
        final_closeout_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_TERMINAL_NO_EXECUTION_FINAL_CLOSEOUT_RECOMMENDED_NEXT_GATE,
        terminal_no_execution_branch_closed: true,
        final_closeout_visible: true,
        final_closeout_recorded: false,
        final_closeout_persisted: false,
        final_closeout_authoritative: false,
        final_closeout_accepted: false,
        source_audit_index_visible: source.audit_index_visible,
        source_audit_index_persisted: false,
        source_readback_persisted: false,
        readback_execution_allowed: false,
        replay_execution_allowed: false,
        replay_diff_recording_allowed: false,
        replay_diff_persistence_allowed: false,
        rollback_execution_allowed: false,
        idempotency_mutation_allowed: false,
        work_graph_event_persistence_allowed: false,
        projection_persistence_allowed: false,
        scheduler_guardrail_enforcement_allowed: false,
        runtime_interception_allowed: false,
        feature_flag_enablement_allowed: false,
        canary_traffic_allowed: false,
        operator_review_request_allowed: false,
        approval_recording_allowed: false,
        live_cutover_allowed: false,
        ready_for_scheduler_guardrail_blocking_dry_run_entrypoint_hardening: true,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_scope()
-> WorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutScopePreview {
    WorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutScopePreview {
        id: "agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_scope",
        source_surface_id: "work_graph_agent_jobs_task_board.work_graph.shadow_event_store.replay_diff_dry_run.non_execution_readback_audit_index_non_persistence_readback",
        closeout_mode: "work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_report_only",
        stable_closeout_key: "work_graph.agent_jobs_task_board.shadow_event_store.replay_diff_dry_run.terminal_no_execution.final_closeout",
        visible: true,
        recorded: false,
        persisted: false,
        authoritative: false,
        accepted: false,
        terminal: true,
        mutation_allowed: false,
    }
}

pub fn work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_entries()
-> Vec<WorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutEntryPreview> {
    vec![
        final_closeout_entry(
            "replay_diff_no_execution_branch_final_closeout",
            "replay_diff.terminal_no_execution.final_closeout.branch_closed",
            "replay_diff_audit_index_no_live_authority_readback",
            "terminal_no_execution_branch",
        ),
        final_closeout_entry(
            "replay_diff_audit_index_surface_final_closeout",
            "replay_diff.terminal_no_execution.final_closeout.audit_index_surface",
            "replay_diff_audit_index_surface_non_persistence_readback",
            "audit_index_surface",
        ),
        final_closeout_entry(
            "replay_diff_audit_index_entry_inventory_final_closeout",
            "replay_diff.terminal_no_execution.final_closeout.audit_index_entries",
            "replay_diff_audit_index_entry_inventory_non_persistence_readback",
            "audit_index_entry_inventory",
        ),
        final_closeout_entry(
            "replay_diff_audit_index_blocker_inventory_final_closeout",
            "replay_diff.terminal_no_execution.final_closeout.audit_index_blockers",
            "replay_diff_audit_index_blocker_inventory_non_persistence_readback",
            "audit_index_blocker_inventory",
        ),
        final_closeout_entry(
            "replay_diff_audit_index_prior_chain_final_closeout",
            "replay_diff.terminal_no_execution.final_closeout.required_priors",
            "replay_diff_audit_index_prior_chain_non_persistence_readback",
            "required_prior_chain",
        ),
        final_closeout_entry(
            "replay_diff_non_persistence_boundary_final_closeout",
            "replay_diff.terminal_no_execution.final_closeout.non_persistence_boundary",
            "replay_diff_audit_index_non_persistence_boundary_readback",
            "non_persistence_boundary",
        ),
        final_closeout_entry(
            "replay_diff_no_live_authority_final_closeout",
            "replay_diff.terminal_no_execution.final_closeout.no_live_authority",
            "replay_diff_audit_index_no_live_authority_readback",
            "no_live_authority",
        ),
        final_closeout_entry(
            "replay_diff_entrypoint_scope_final_closeout",
            "replay_diff.terminal_no_execution.final_closeout.entrypoint_scope",
            "replay_diff_audit_index_entry_inventory_non_persistence_readback",
            "entrypoint_scope",
        ),
        final_closeout_entry(
            "replay_diff_scheduler_guardrail_boundary_final_closeout",
            "replay_diff.terminal_no_execution.final_closeout.scheduler_guardrail_boundary",
            "scheduler_guardrail_live_enforcement_blocked",
            "scheduler_guardrail_boundary",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_blockers()
-> Vec<WorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutBlockerPreview> {
    vec![
        final_closeout_blocker(
            "final_closeout_record_blocked",
            "record_replay_diff_terminal_no_execution_final_closeout",
        ),
        final_closeout_blocker(
            "final_closeout_persistence_blocked",
            "persist_replay_diff_terminal_no_execution_final_closeout",
        ),
        final_closeout_blocker(
            "final_closeout_acceptance_blocked",
            "accept_replay_diff_terminal_no_execution_final_closeout",
        ),
        final_closeout_blocker(
            "audit_index_readback_record_blocked",
            "record_replay_diff_audit_index_non_persistence_readback",
        ),
        final_closeout_blocker(
            "audit_index_readback_persistence_blocked",
            "persist_replay_diff_audit_index_non_persistence_readback",
        ),
        final_closeout_blocker(
            "audit_index_readback_acceptance_blocked",
            "accept_replay_diff_audit_index_non_persistence_readback",
        ),
        final_closeout_blocker(
            "audit_index_record_blocked",
            "record_replay_diff_non_execution_readback_audit_index",
        ),
        final_closeout_blocker(
            "audit_index_persistence_blocked",
            "persist_replay_diff_non_execution_readback_audit_index",
        ),
        final_closeout_blocker(
            "audit_index_acceptance_blocked",
            "accept_replay_diff_non_execution_readback_audit_index",
        ),
        final_closeout_blocker(
            "readback_execution_blocked",
            "execute_non_execution_readback",
        ),
        final_closeout_blocker(
            "readback_recording_blocked",
            "record_non_execution_readback",
        ),
        final_closeout_blocker(
            "readback_persistence_blocked",
            "persist_non_execution_readback",
        ),
        final_closeout_blocker("replay_execution_blocked", "execute_replay"),
        final_closeout_blocker("replay_diff_recording_blocked", "record_replay_diff"),
        final_closeout_blocker("replay_diff_persistence_blocked", "persist_replay_diff"),
        final_closeout_blocker("rollback_execution_blocked", "execute_rollback"),
        final_closeout_blocker("idempotency_mutation_blocked", "mutate_idempotency_index"),
        final_closeout_blocker(
            "work_graph_event_persistence_blocked",
            "persist_work_graph_event",
        ),
        final_closeout_blocker(
            "projection_index_persistence_blocked",
            "persist_projection_index",
        ),
        final_closeout_blocker(
            "scheduler_guardrail_live_enforcement_blocked",
            "enable_scheduler_guardrail_live_enforcement",
        ),
        final_closeout_blocker(
            "runtime_interception_blocked",
            "enable_runtime_interception",
        ),
        final_closeout_blocker("feature_flag_enablement_blocked", "enable_feature_flag"),
        final_closeout_blocker("canary_traffic_blocked", "route_canary_traffic"),
        final_closeout_blocker("operator_review_request_blocked", "request_operator_review"),
        final_closeout_blocker("approval_recording_blocked", "record_operator_approval"),
        final_closeout_blocker("live_cutover_blocked", "perform_live_cutover"),
    ]
}

pub fn work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_required_prior_gates()
-> Vec<&'static str> {
    let source =
        hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_report();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_NON_EXECUTION_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
    ];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());
    required_prior_gates
}

impl
    WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutSideEffects
{
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            final_closeout_recorded: false,
            final_closeout_persisted: false,
            final_closeout_accepted: false,
            audit_index_readback_recorded: false,
            audit_index_readback_persisted: false,
            audit_index_readback_accepted: false,
            audit_index_recorded: false,
            audit_index_persisted: false,
            audit_index_accepted: false,
            non_execution_readback_executed: false,
            non_execution_readback_recorded: false,
            non_execution_readback_persisted: false,
            replay_executed: false,
            replay_diff_recorded: false,
            replay_diff_persisted: false,
            rollback_executed: false,
            idempotency_index_mutated: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            shadow_event_persisted: false,
            projection_index_persisted: false,
            scheduler_admission_enforced: false,
            guardrail_enforcement_enabled: false,
            runtime_interception_enabled: false,
            config_written: false,
            feature_flag_mutated: false,
            canary_traffic_routed: false,
            operator_review_requested: false,
            approval_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn final_closeout_entry(
    id: &'static str,
    stable_closeout_key: &'static str,
    source_readback_id: &'static str,
    closeout_category: &'static str,
) -> WorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutEntryPreview {
    WorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutEntryPreview {
        id,
        stable_closeout_key,
        source_readback_id,
        closeout_category,
        visible: true,
        recorded: false,
        persisted: false,
        accepted: false,
        authoritative: false,
        mutation_allowed: false,
        closed: true,
    }
}

fn final_closeout_blocker(
    id: &'static str,
    blocked_action: &'static str,
) -> WorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutBlockerPreview {
    WorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason: "replay/diff terminal no-execution final closeout cannot authorize this action",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_diff_terminal_no_execution_final_closeout_derives_from_non_persistence_readback() {
        let report =
            hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_report();

        assert_eq!(
            report.source_non_persistence_readback_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_NON_EXECUTION_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE
        );
        assert_eq!(report.source_readback_entry_count, 6);
        assert_eq!(report.source_readback_blocker_count, 23);
        assert_eq!(report.source_required_prior_gate_count, 4);
        assert_eq!(report.final_closeout_entry_count, 9);
        assert_eq!(report.final_closeout_blocker_count, 26);
        assert_eq!(report.required_prior_gate_count, 5);
    }

    #[test]
    fn replay_diff_terminal_no_execution_final_closeout_is_visible_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_report();

        assert!(report.final_closeout_scope.visible);
        assert!(!report.final_closeout_scope.recorded);
        assert!(!report.final_closeout_scope.persisted);
        assert!(!report.final_closeout_scope.authoritative);
        assert!(!report.final_closeout_scope.accepted);
        assert!(report.final_closeout_scope.terminal);
        assert!(!report.final_closeout_scope.mutation_allowed);
        assert!(report.final_closeout_entries.iter().all(|entry| {
            entry.visible
                && entry.closed
                && !entry.recorded
                && !entry.persisted
                && !entry.accepted
                && !entry.authoritative
                && !entry.mutation_allowed
        }));
    }

    #[test]
    fn replay_diff_terminal_no_execution_final_closeout_blocks_execution_and_live_paths() {
        let report =
            hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_report();

        assert_eq!(
            report.required_prior_gates[0],
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_NON_EXECUTION_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE
        );
        assert!(
            report
                .final_closeout_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
        assert!(report.terminal_no_execution_branch_closed);
        assert!(report.final_closeout_visible);
        assert!(!report.final_closeout_recorded);
        assert!(!report.final_closeout_persisted);
        assert!(!report.final_closeout_authoritative);
        assert!(!report.final_closeout_accepted);
        assert!(report.source_audit_index_visible);
        assert!(!report.source_audit_index_persisted);
        assert!(!report.source_readback_persisted);
        assert!(!report.readback_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.replay_diff_recording_allowed);
        assert!(!report.replay_diff_persistence_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.idempotency_mutation_allowed);
        assert!(!report.work_graph_event_persistence_allowed);
        assert!(!report.projection_persistence_allowed);
        assert!(!report.scheduler_guardrail_enforcement_allowed);
        assert!(!report.runtime_interception_allowed);
        assert!(!report.feature_flag_enablement_allowed);
        assert!(!report.canary_traffic_allowed);
        assert!(!report.operator_review_request_allowed);
        assert!(!report.approval_recording_allowed);
        assert!(!report.live_cutover_allowed);
        assert!(report.ready_for_scheduler_guardrail_blocking_dry_run_entrypoint_hardening);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn replay_diff_terminal_no_execution_final_closeout_links_priors_and_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_report();

        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_NON_EXECUTION_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
                "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_gate",
                "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_gate",
                "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_gate",
                "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate",
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_TERMINAL_NO_EXECUTION_FINAL_CLOSEOUT_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunTerminalNoExecutionFinalCloseoutSideEffects::none()
        );
    }
}
