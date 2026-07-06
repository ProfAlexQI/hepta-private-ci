use serde::Serialize;

use crate::work_graph_append_only_event_store_shadow_path::{
    WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE,
    WorkGraphAppendOnlyEventStoreShadowPathSideEffects,
    hepta_work_graph_append_only_event_store_shadow_path_report,
};

pub const WORK_GRAPH_PERSISTENT_MAILBOX_HANDOFF_EVENT_MAPPING_GATE: &str =
    "hepta_work_graph_persistent_mailbox_handoff_event_mapping_gate";
pub const WORK_GRAPH_PERSISTENT_MAILBOX_HANDOFF_EVENT_MAPPING_SCHEMA_VERSION: &str =
    "work_graph_persistent_mailbox_handoff_event_mapping_v1";
pub const WORK_GRAPH_PERSISTENT_MAILBOX_HANDOFF_EVENT_MAPPING_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_agent_role_agent_card_manifest_report_only_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistentMailboxHandoffEventMappingReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub mailbox_event_mapping_count: usize,
    pub handoff_event_mapping_count: usize,
    pub ack_deadline_contract_count: usize,
    pub wait_agent_target_count: usize,
    pub required_prior_gate_count: usize,
    pub source_shadow_path_scheduler_prior_gate_count: usize,
    pub source_shadow_path_required_prior_gate_count: usize,
    pub mailbox_event_mappings: Vec<WorkGraphMailboxEventMappingPreview>,
    pub handoff_event_mappings: Vec<WorkGraphHandoffEventMappingPreview>,
    pub ack_deadline_contracts: Vec<WorkGraphMailboxAckDeadlineContractPreview>,
    pub wait_agent_targets: Vec<WorkGraphWaitAgentTargetPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub source_shadow_path_gate: &'static str,
    pub recommended_next_gate: &'static str,
    pub source_shadow_path_readiness_complete: bool,
    pub source_shadow_path_ready_for_persistent_mailbox_handoff: bool,
    pub source_shadow_path_no_persistence_confirmed: bool,
    pub persistent_mailbox_handoff_mapping_readiness_complete: bool,
    pub mailbox_events_map_to_work_graph_events: bool,
    pub ack_deadline_parent_child_artifact_refs_ready: bool,
    pub wait_agent_named_task_result_barrier_ready: bool,
    pub persistent_mailbox_store_enabled: bool,
    pub live_wait_agent_behavior_changed: bool,
    pub ready_for_agent_role_agent_card_manifest: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphPersistentMailboxHandoffEventMappingSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphMailboxEventMappingPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub work_graph_event_kind: &'static str,
    pub required_fields: Vec<&'static str>,
    pub parent_child_task_ref_required: bool,
    pub ack_ref_required: bool,
    pub deadline_ref_required: bool,
    pub artifact_refs_required: bool,
    pub maps_to_shadow_event_store: bool,
    pub persists_mailbox_event: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphHandoffEventMappingPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub work_graph_event_kind: &'static str,
    pub required_fields: Vec<&'static str>,
    pub handoff_direction: &'static str,
    pub artifact_refs_required: bool,
    pub approval_ref_required: bool,
    pub maps_to_shadow_event_store: bool,
    pub persists_handoff_event: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphMailboxAckDeadlineContractPreview {
    pub id: &'static str,
    pub applies_to_event_kind: &'static str,
    pub required_fields: Vec<&'static str>,
    pub timeout_policy: &'static str,
    pub ack_state: &'static str,
    pub deadline_state: &'static str,
    pub mutates_mailbox_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphWaitAgentTargetPreview {
    pub id: &'static str,
    pub wait_agent_mode: &'static str,
    pub wait_target_type: &'static str,
    pub required_fields: Vec<&'static str>,
    pub success_condition: &'static str,
    pub timeout_condition: &'static str,
    pub returns_task_result_ref: bool,
    pub returns_barrier_ref: bool,
    pub live_wait_behavior_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistentMailboxHandoffEventMappingSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub mailbox_event_persisted: bool,
    pub handoff_event_persisted: bool,
    pub ack_recorded: bool,
    pub deadline_recorded: bool,
    pub wait_agent_runtime_changed: bool,
    pub barrier_state_mutated: bool,
    pub artifact_ref_persisted: bool,
    pub scheduler_admission_enforced: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_persistent_mailbox_handoff_event_mapping_report()
-> WorkGraphPersistentMailboxHandoffEventMappingReport {
    let mailbox_event_mappings = work_graph_persistent_mailbox_event_mappings();
    let handoff_event_mappings = work_graph_persistent_handoff_event_mappings();
    let ack_deadline_contracts = work_graph_persistent_mailbox_ack_deadline_contracts();
    let wait_agent_targets = work_graph_wait_agent_named_task_result_barrier_targets();
    let source_shadow_path = hepta_work_graph_append_only_event_store_shadow_path_report();
    let required_prior_gates =
        work_graph_persistent_mailbox_handoff_event_mapping_required_prior_gates();
    let source_shadow_path_no_persistence_confirmed = !source_shadow_path
        .shadow_store_write_enabled
        && !source_shadow_path.live_cutover_enabled
        && !source_shadow_path.ready_for_live_execution
        && source_shadow_path.side_effects
            == WorkGraphAppendOnlyEventStoreShadowPathSideEffects::none();
    let source_shadow_path_readiness_complete = source_shadow_path.gate
        == WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE
        && source_shadow_path.scheduler_prior_gate_count == 5
        && source_shadow_path.required_prior_gate_count == 9
        && source_shadow_path.scheduler_prior_chain_ready
        && source_shadow_path.task_result_contract_field_gap_readback_ready
        && source_shadow_path.append_only_shadow_path_readiness_complete
        && source_shadow_path.ready_for_persistent_mailbox_handoff
        && source_shadow_path_no_persistence_confirmed;
    let persistent_mailbox_handoff_mapping_readiness_complete =
        source_shadow_path_readiness_complete
            && !mailbox_event_mappings.is_empty()
            && !handoff_event_mappings.is_empty()
            && !ack_deadline_contracts.is_empty()
            && !wait_agent_targets.is_empty();

    WorkGraphPersistentMailboxHandoffEventMappingReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENT_MAILBOX_HANDOFF_EVENT_MAPPING_GATE,
        schema_version: WORK_GRAPH_PERSISTENT_MAILBOX_HANDOFF_EVENT_MAPPING_SCHEMA_VERSION,
        preview_mode: "report_only_persistent_mailbox_handoff_event_mapping_no_live_wait_change",
        mailbox_event_mapping_count: mailbox_event_mappings.len(),
        handoff_event_mapping_count: handoff_event_mappings.len(),
        ack_deadline_contract_count: ack_deadline_contracts.len(),
        wait_agent_target_count: wait_agent_targets.len(),
        required_prior_gate_count: required_prior_gates.len(),
        source_shadow_path_scheduler_prior_gate_count: source_shadow_path
            .scheduler_prior_gate_count,
        source_shadow_path_required_prior_gate_count: source_shadow_path.required_prior_gate_count,
        mailbox_event_mappings,
        handoff_event_mappings,
        ack_deadline_contracts,
        wait_agent_targets,
        required_prior_gates,
        source_shadow_path_gate: source_shadow_path.gate,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENT_MAILBOX_HANDOFF_EVENT_MAPPING_RECOMMENDED_NEXT_GATE,
        source_shadow_path_readiness_complete,
        source_shadow_path_ready_for_persistent_mailbox_handoff: source_shadow_path
            .ready_for_persistent_mailbox_handoff,
        source_shadow_path_no_persistence_confirmed,
        persistent_mailbox_handoff_mapping_readiness_complete,
        mailbox_events_map_to_work_graph_events: true,
        ack_deadline_parent_child_artifact_refs_ready: true,
        wait_agent_named_task_result_barrier_ready: true,
        persistent_mailbox_store_enabled: false,
        live_wait_agent_behavior_changed: false,
        ready_for_agent_role_agent_card_manifest:
            persistent_mailbox_handoff_mapping_readiness_complete,
        ready_for_live_execution: false,
        side_effects: WorkGraphPersistentMailboxHandoffEventMappingSideEffects::none(),
    }
}

pub fn work_graph_persistent_mailbox_event_mappings() -> Vec<WorkGraphMailboxEventMappingPreview> {
    vec![
        mailbox_event_mapping(
            "mailbox_message_queued_to_work_graph_event",
            "MailboxMessageQueued",
            true,
            true,
            false,
        ),
        mailbox_event_mapping(
            "mailbox_message_delivered_to_work_graph_event",
            "MailboxMessageDelivered",
            true,
            true,
            true,
        ),
        mailbox_event_mapping(
            "mailbox_ack_observed_to_work_graph_event",
            "MailboxAckObserved",
            true,
            false,
            false,
        ),
        mailbox_event_mapping(
            "mailbox_deadline_expired_to_work_graph_event",
            "MailboxDeadlineExpired",
            false,
            true,
            false,
        ),
    ]
}

pub fn work_graph_persistent_handoff_event_mappings() -> Vec<WorkGraphHandoffEventMappingPreview> {
    vec![
        handoff_event_mapping(
            "handoff_requested_to_work_graph_event",
            "HandoffRequested",
            "parent_to_child",
            true,
            true,
        ),
        handoff_event_mapping(
            "handoff_accepted_to_work_graph_event",
            "HandoffAccepted",
            "child_to_parent",
            true,
            false,
        ),
        handoff_event_mapping(
            "handoff_artifact_linked_to_work_graph_event",
            "HandoffArtifactLinked",
            "producer_to_consumer",
            true,
            false,
        ),
        handoff_event_mapping(
            "handoff_barrier_satisfied_to_work_graph_event",
            "TaskBarrierSatisfied",
            "barrier_to_waiter",
            false,
            false,
        ),
    ]
}

pub fn work_graph_persistent_mailbox_ack_deadline_contracts()
-> Vec<WorkGraphMailboxAckDeadlineContractPreview> {
    vec![
        ack_deadline_contract(
            "mailbox_delivery_ack_contract",
            "MailboxMessageDelivered",
            "ack_required_before_wait_success",
            "deadline_carried_from_wait_budget",
        ),
        ack_deadline_contract(
            "mailbox_wait_timeout_contract",
            "MailboxDeadlineExpired",
            "ack_absent_after_timeout",
            "deadline_required_for_timeout_result",
        ),
        ack_deadline_contract(
            "handoff_acceptance_ack_contract",
            "HandoffAccepted",
            "ack_required_before_parent_merge",
            "deadline_carried_from_handoff_policy",
        ),
    ]
}

pub fn work_graph_wait_agent_named_task_result_barrier_targets()
-> Vec<WorkGraphWaitAgentTargetPreview> {
    vec![
        wait_agent_target(
            "wait_agent_named_task_target",
            "named_task",
            vec!["taskName", "taskId", "traceId", "parentTaskId"],
            "named task reaches terminal TaskResultEnvelope",
            "deadline expires before named task terminal result",
            true,
            false,
        ),
        wait_agent_target(
            "wait_agent_task_result_target",
            "task_result",
            vec!["taskId", "expectedStatus", "traceId", "verifierRef"],
            "TaskResultEnvelope status satisfies expected status",
            "deadline expires before task result readback",
            true,
            false,
        ),
        wait_agent_target(
            "wait_agent_mailbox_barrier_target",
            "barrier",
            vec!["barrierId", "parentTaskId", "childTaskIds", "traceId"],
            "all child task barriers have acked or terminal results",
            "deadline expires before barrier quorum",
            false,
            true,
        ),
    ]
}

pub fn work_graph_persistent_mailbox_handoff_event_mapping_required_prior_gates()
-> Vec<&'static str> {
    vec![WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE]
}

impl WorkGraphPersistentMailboxHandoffEventMappingSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            mailbox_event_persisted: false,
            handoff_event_persisted: false,
            ack_recorded: false,
            deadline_recorded: false,
            wait_agent_runtime_changed: false,
            barrier_state_mutated: false,
            artifact_ref_persisted: false,
            scheduler_admission_enforced: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn mailbox_event_mapping(
    id: &'static str,
    work_graph_event_kind: &'static str,
    ack_ref_required: bool,
    deadline_ref_required: bool,
    artifact_refs_required: bool,
) -> WorkGraphMailboxEventMappingPreview {
    WorkGraphMailboxEventMappingPreview {
        id,
        source_surface_id: "multi_agent_v2_mailbox_wait",
        work_graph_event_kind,
        required_fields: vec![
            "traceId",
            "mailboxSeq",
            "agentPath",
            "parentTaskId",
            "childTaskId",
            "eventId",
        ],
        parent_child_task_ref_required: true,
        ack_ref_required,
        deadline_ref_required,
        artifact_refs_required,
        maps_to_shadow_event_store: true,
        persists_mailbox_event: false,
    }
}

fn handoff_event_mapping(
    id: &'static str,
    work_graph_event_kind: &'static str,
    handoff_direction: &'static str,
    artifact_refs_required: bool,
    approval_ref_required: bool,
) -> WorkGraphHandoffEventMappingPreview {
    WorkGraphHandoffEventMappingPreview {
        id,
        source_surface_id: "hepta_runtime_agent_harness",
        work_graph_event_kind,
        required_fields: vec![
            "traceId",
            "handoffId",
            "parentTaskId",
            "childTaskId",
            "artifactRefs",
            "deadlineRef",
        ],
        handoff_direction,
        artifact_refs_required,
        approval_ref_required,
        maps_to_shadow_event_store: true,
        persists_handoff_event: false,
    }
}

fn ack_deadline_contract(
    id: &'static str,
    applies_to_event_kind: &'static str,
    ack_state: &'static str,
    deadline_state: &'static str,
) -> WorkGraphMailboxAckDeadlineContractPreview {
    WorkGraphMailboxAckDeadlineContractPreview {
        id,
        applies_to_event_kind,
        required_fields: vec![
            "traceId",
            "ackId",
            "deadlineUnixMs",
            "parentTaskId",
            "childTaskId",
        ],
        timeout_policy: "deadline_required_no_unbounded_wait",
        ack_state,
        deadline_state,
        mutates_mailbox_state: false,
    }
}

fn wait_agent_target(
    id: &'static str,
    wait_target_type: &'static str,
    required_fields: Vec<&'static str>,
    success_condition: &'static str,
    timeout_condition: &'static str,
    returns_task_result_ref: bool,
    returns_barrier_ref: bool,
) -> WorkGraphWaitAgentTargetPreview {
    WorkGraphWaitAgentTargetPreview {
        id,
        wait_agent_mode: "named_task_result_barrier_preview",
        wait_target_type,
        required_fields,
        success_condition,
        timeout_condition,
        returns_task_result_ref,
        returns_barrier_ref,
        live_wait_behavior_enabled: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persistent_mailbox_handoff_maps_mailbox_events_to_work_graph_events() {
        let report = hepta_work_graph_persistent_mailbox_handoff_event_mapping_report();
        let event_kinds = report
            .mailbox_event_mappings
            .iter()
            .map(|mapping| mapping.work_graph_event_kind)
            .collect::<Vec<_>>();

        assert_eq!(report.mailbox_event_mapping_count, 4);
        assert!(event_kinds.contains(&"MailboxMessageQueued"));
        assert!(event_kinds.contains(&"MailboxMessageDelivered"));
        assert!(event_kinds.contains(&"MailboxAckObserved"));
        assert!(event_kinds.contains(&"MailboxDeadlineExpired"));
        assert!(report.mailbox_event_mappings.iter().all(|mapping| {
            mapping.maps_to_shadow_event_store
                && mapping.parent_child_task_ref_required
                && !mapping.persists_mailbox_event
        }));
    }

    #[test]
    fn persistent_mailbox_handoff_declares_handoff_ack_deadline_and_artifact_refs() {
        let report = hepta_work_graph_persistent_mailbox_handoff_event_mapping_report();

        assert_eq!(report.handoff_event_mapping_count, 4);
        assert_eq!(report.ack_deadline_contract_count, 3);
        assert!(
            report
                .handoff_event_mappings
                .iter()
                .any(|mapping| mapping.artifact_refs_required)
        );
        assert!(
            report
                .ack_deadline_contracts
                .iter()
                .all(|contract| !contract.mutates_mailbox_state)
        );
        assert!(report.ack_deadline_parent_child_artifact_refs_ready);
    }

    #[test]
    fn persistent_mailbox_handoff_upgrades_wait_agent_targets_in_preview() {
        let report = hepta_work_graph_persistent_mailbox_handoff_event_mapping_report();
        let target_types = report
            .wait_agent_targets
            .iter()
            .map(|target| target.wait_target_type)
            .collect::<Vec<_>>();

        assert_eq!(report.wait_agent_target_count, 3);
        assert!(target_types.contains(&"named_task"));
        assert!(target_types.contains(&"task_result"));
        assert!(target_types.contains(&"barrier"));
        assert!(
            report
                .wait_agent_targets
                .iter()
                .all(|target| !target.live_wait_behavior_enabled)
        );
        assert!(report.wait_agent_named_task_result_barrier_ready);
    }

    #[test]
    fn persistent_mailbox_handoff_remains_report_only_and_links_prior_gate() {
        let report = hepta_work_graph_persistent_mailbox_handoff_event_mapping_report();

        assert_eq!(
            report.required_prior_gates,
            vec![WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE]
        );
        assert_eq!(
            report.source_shadow_path_gate,
            WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE
        );
        assert_eq!(report.source_shadow_path_scheduler_prior_gate_count, 5);
        assert_eq!(report.source_shadow_path_required_prior_gate_count, 9);
        assert!(report.source_shadow_path_readiness_complete);
        assert!(report.source_shadow_path_ready_for_persistent_mailbox_handoff);
        assert!(report.source_shadow_path_no_persistence_confirmed);
        assert!(report.persistent_mailbox_handoff_mapping_readiness_complete);
        assert!(!report.persistent_mailbox_store_enabled);
        assert!(!report.live_wait_agent_behavior_changed);
        assert!(report.ready_for_agent_role_agent_card_manifest);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistentMailboxHandoffEventMappingSideEffects::none()
        );
    }
}
