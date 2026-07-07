use serde::Serialize;

use crate::work_graph_persistent_mailbox_handoff_event_mapping::WORK_GRAPH_PERSISTENT_MAILBOX_HANDOFF_EVENT_MAPPING_GATE;
use crate::work_graph_persistent_mailbox_handoff_event_mapping::WorkGraphPersistentMailboxHandoffEventMappingSideEffects;
use crate::work_graph_persistent_mailbox_handoff_event_mapping::hepta_work_graph_persistent_mailbox_handoff_event_mapping_report;
use crate::work_graph_role_manifest_contract::WORK_GRAPH_ROLE_MANIFEST_CONTRACT_PREVIEW_GATE;
use crate::work_graph_role_manifest_contract::WorkGraphRoleManifestContractPreviewSideEffects;
use crate::work_graph_role_manifest_contract::hepta_work_graph_role_manifest_contract_preview_report;

pub const WORK_GRAPH_AGENT_ROLE_AGENT_CARD_MANIFEST_REPORT_ONLY_GATE: &str =
    "hepta_work_graph_agent_role_agent_card_manifest_report_only_gate";
pub const WORK_GRAPH_AGENT_ROLE_AGENT_CARD_MANIFEST_REPORT_ONLY_SCHEMA_VERSION: &str =
    "work_graph_agent_role_agent_card_manifest_report_only_v1";
pub const WORK_GRAPH_AGENT_ROLE_AGENT_CARD_MANIFEST_REPORT_ONLY_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_trace_guardrail_span_report_only_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentRoleAgentCardManifestReportOnlyReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub required_wire_field_count: usize,
    pub agent_card_count: usize,
    pub source_binding_count: usize,
    pub required_prior_gate_count: usize,
    pub source_persistent_mailbox_handoff_required_prior_gate_count: usize,
    pub source_role_manifest_required_field_count: usize,
    pub source_role_manifest_capability_count: usize,
    pub source_role_manifest_permission_mode_count: usize,
    pub source_role_manifest_invariant_count: usize,
    pub source_role_manifest_adapter_preview_count: usize,
    pub required_wire_fields: Vec<&'static str>,
    pub agent_cards: Vec<WorkGraphAgentRoleAgentCardPreview>,
    pub source_bindings: Vec<WorkGraphAgentRoleSourceBindingPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub source_persistent_mailbox_handoff_gate: &'static str,
    pub source_role_manifest_contract_gate: &'static str,
    pub recommended_next_gate: &'static str,
    pub source_persistent_mailbox_handoff_readiness_complete: bool,
    pub source_persistent_mailbox_handoff_no_persistence_confirmed: bool,
    pub source_role_manifest_contract_ready: bool,
    pub source_role_manifest_no_enforcement_confirmed: bool,
    pub agent_role_agent_card_manifest_readiness_complete: bool,
    pub capability_tool_budget_lane_ready: bool,
    pub side_effect_class_ready: bool,
    pub handoff_output_verifier_ready: bool,
    pub report_only_manifest_attached: bool,
    pub role_enforcement_enabled: bool,
    pub ready_for_trace_guardrail_span: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAgentRoleAgentCardManifestReportOnlySideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentRoleAgentCardPreview {
    pub role_id: &'static str,
    pub agent_card_id: &'static str,
    pub capability_ids: Vec<&'static str>,
    pub allowed_tools: Vec<&'static str>,
    pub budget_policy_id: &'static str,
    pub side_effect_class: &'static str,
    pub handoff_description: &'static str,
    pub output_contract: &'static str,
    pub verifier_id: &'static str,
    pub reducer_id: &'static str,
    pub lane: &'static str,
    pub manifest_report_only: bool,
    pub role_enforcement_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentRoleSourceBindingPreview {
    pub source_surface_id: &'static str,
    pub entrypoint_id: &'static str,
    pub required_role_ids: Vec<&'static str>,
    pub default_lane: &'static str,
    pub output_contract: &'static str,
    pub verifier_required: bool,
    pub manifest_attached_report_only: bool,
    pub enforcement_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentRoleAgentCardManifestReportOnlySideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub role_manifest_persisted: bool,
    pub role_enforcement_enabled: bool,
    pub tool_permission_changed: bool,
    pub budget_debited: bool,
    pub scheduler_admission_enforced: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_agent_role_agent_card_manifest_report_only_report()
-> WorkGraphAgentRoleAgentCardManifestReportOnlyReport {
    let required_wire_fields = work_graph_agent_role_agent_card_required_wire_fields();
    let agent_cards = work_graph_agent_role_agent_cards();
    let source_bindings = work_graph_agent_role_source_bindings();
    let required_prior_gates =
        work_graph_agent_role_agent_card_manifest_report_only_required_prior_gates();
    let persistent_mailbox = hepta_work_graph_persistent_mailbox_handoff_event_mapping_report();
    let role_manifest_contract = hepta_work_graph_role_manifest_contract_preview_report();
    let source_persistent_mailbox_handoff_no_persistence_confirmed = !persistent_mailbox
        .persistent_mailbox_store_enabled
        && !persistent_mailbox.live_wait_agent_behavior_changed
        && !persistent_mailbox.ready_for_live_execution
        && persistent_mailbox.side_effects
            == WorkGraphPersistentMailboxHandoffEventMappingSideEffects::none();
    let source_persistent_mailbox_handoff_readiness_complete = persistent_mailbox.gate
        == WORK_GRAPH_PERSISTENT_MAILBOX_HANDOFF_EVENT_MAPPING_GATE
        && persistent_mailbox.persistent_mailbox_handoff_mapping_readiness_complete
        && persistent_mailbox.ready_for_agent_role_agent_card_manifest
        && source_persistent_mailbox_handoff_no_persistence_confirmed;
    let source_role_manifest_no_enforcement_confirmed = !role_manifest_contract
        .ready_for_role_enforcement
        && !role_manifest_contract.ready_for_live_execution
        && role_manifest_contract.side_effects
            == WorkGraphRoleManifestContractPreviewSideEffects::none();
    let source_role_manifest_contract_ready = role_manifest_contract.gate
        == WORK_GRAPH_ROLE_MANIFEST_CONTRACT_PREVIEW_GATE
        && role_manifest_contract.required_field_count == 12
        && role_manifest_contract.capability_count == 7
        && role_manifest_contract.permission_mode_count == 5
        && role_manifest_contract.invariant_count == 6
        && role_manifest_contract.adapter_preview_count == 4
        && role_manifest_contract.ready_for_unified_state_store_preview
        && source_role_manifest_no_enforcement_confirmed;
    let agent_role_agent_card_manifest_readiness_complete =
        source_persistent_mailbox_handoff_readiness_complete
            && source_role_manifest_contract_ready
            && !required_wire_fields.is_empty()
            && !agent_cards.is_empty()
            && !source_bindings.is_empty();

    WorkGraphAgentRoleAgentCardManifestReportOnlyReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_ROLE_AGENT_CARD_MANIFEST_REPORT_ONLY_GATE,
        schema_version: WORK_GRAPH_AGENT_ROLE_AGENT_CARD_MANIFEST_REPORT_ONLY_SCHEMA_VERSION,
        preview_mode: "report_only_agent_role_agent_card_manifest_no_enforcement",
        required_wire_field_count: required_wire_fields.len(),
        agent_card_count: agent_cards.len(),
        source_binding_count: source_bindings.len(),
        required_prior_gate_count: required_prior_gates.len(),
        source_persistent_mailbox_handoff_required_prior_gate_count: persistent_mailbox
            .required_prior_gate_count,
        source_role_manifest_required_field_count: role_manifest_contract.required_field_count,
        source_role_manifest_capability_count: role_manifest_contract.capability_count,
        source_role_manifest_permission_mode_count: role_manifest_contract.permission_mode_count,
        source_role_manifest_invariant_count: role_manifest_contract.invariant_count,
        source_role_manifest_adapter_preview_count: role_manifest_contract.adapter_preview_count,
        required_wire_fields,
        agent_cards,
        source_bindings,
        required_prior_gates,
        source_persistent_mailbox_handoff_gate: persistent_mailbox.gate,
        source_role_manifest_contract_gate: role_manifest_contract.gate,
        recommended_next_gate:
            WORK_GRAPH_AGENT_ROLE_AGENT_CARD_MANIFEST_REPORT_ONLY_RECOMMENDED_NEXT_GATE,
        source_persistent_mailbox_handoff_readiness_complete,
        source_persistent_mailbox_handoff_no_persistence_confirmed,
        source_role_manifest_contract_ready,
        source_role_manifest_no_enforcement_confirmed,
        agent_role_agent_card_manifest_readiness_complete,
        capability_tool_budget_lane_ready: true,
        side_effect_class_ready: true,
        handoff_output_verifier_ready: true,
        report_only_manifest_attached: agent_role_agent_card_manifest_readiness_complete,
        role_enforcement_enabled: false,
        ready_for_trace_guardrail_span: agent_role_agent_card_manifest_readiness_complete,
        ready_for_live_execution: false,
        side_effects: WorkGraphAgentRoleAgentCardManifestReportOnlySideEffects::none(),
    }
}

pub fn work_graph_agent_role_agent_card_required_wire_fields() -> Vec<&'static str> {
    vec![
        "roleId",
        "agentCardId",
        "capabilities",
        "allowedTools",
        "budget",
        "sideEffectClass",
        "handoffDescription",
        "outputContract",
        "verifier",
        "reducer",
        "lane",
    ]
}

pub fn work_graph_agent_role_agent_cards() -> Vec<WorkGraphAgentRoleAgentCardPreview> {
    vec![
        agent_card(
            "planner_role",
            "agent_card_planner_report_only",
            vec!["planning", "research"],
            vec!["update_plan", "read_repo", "web_research_preview"],
            "budget_planner_small",
            "read_only",
            "may create plan nodes and hand off named tasks without spawning work directly",
            "TaskResultEnvelope",
            "planner_output_verifier",
            "plan_step_reducer",
        ),
        agent_card(
            "builder_role",
            "agent_card_builder_report_only",
            vec!["code_editing", "verification"],
            vec!["apply_patch", "cargo_test", "bash_gate"],
            "budget_builder_medium",
            "local_write_preview",
            "may produce scoped patches and return artifacts with evidence refs",
            "TaskResultEnvelope",
            "builder_patch_verifier",
            "artifact_reducer",
        ),
        agent_card(
            "reviewer_role",
            "agent_card_reviewer_report_only",
            vec!["verification", "research"],
            vec!["read_repo", "cargo_test", "diff_review"],
            "budget_reviewer_small",
            "read_only",
            "may review outputs and attach risks, evidence, and next actions",
            "TaskResultEnvelope",
            "reviewer_gate_verifier",
            "review_summary_reducer",
        ),
        agent_card(
            "scheduler_role",
            "agent_card_scheduler_report_only",
            vec!["scheduler_control", "verification"],
            vec!["admission_dry_run", "lease_readback", "budget_readback"],
            "budget_scheduler_small",
            "scheduler_dry_run",
            "may produce allow or deny explanations without acquiring a live lease",
            "TaskResultEnvelope",
            "scheduler_admission_verifier",
            "scheduler_decision_reducer",
        ),
        agent_card(
            "handoff_role",
            "agent_card_handoff_report_only",
            vec!["external_handoff_proposal", "verification"],
            vec!["artifact_link", "approval_readback", "handoff_preview"],
            "budget_handoff_small",
            "external_handoff_preview",
            "may prepare handoff previews with artifact refs and approval requirements only",
            "TaskResultEnvelope",
            "handoff_scope_verifier",
            "handoff_barrier_reducer",
        ),
    ]
}

pub fn work_graph_agent_role_source_bindings() -> Vec<WorkGraphAgentRoleSourceBindingPreview> {
    vec![
        source_binding(
            "multi_agent_v2_thread_spawn",
            "spawn_agent",
            vec!["planner_role", "builder_role", "reviewer_role"],
        ),
        source_binding(
            "agent_jobs_batch_workers",
            "spawn_agents_on_csv",
            vec!["builder_role", "reviewer_role"],
        ),
        source_binding(
            "hepta_runtime_worker_tasks",
            "worker_task_run",
            vec!["builder_role", "scheduler_role"],
        ),
        source_binding(
            "hepta_runtime_agent_harness",
            "handoff_event",
            vec!["handoff_role", "reviewer_role"],
        ),
        source_binding(
            "hepta_runtime_task_board",
            "task_board_claim",
            vec!["scheduler_role", "reviewer_role"],
        ),
    ]
}

pub fn work_graph_agent_role_agent_card_manifest_report_only_required_prior_gates()
-> Vec<&'static str> {
    vec![
        WORK_GRAPH_PERSISTENT_MAILBOX_HANDOFF_EVENT_MAPPING_GATE,
        WORK_GRAPH_ROLE_MANIFEST_CONTRACT_PREVIEW_GATE,
    ]
}

impl WorkGraphAgentRoleAgentCardManifestReportOnlySideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            role_manifest_persisted: false,
            role_enforcement_enabled: false,
            tool_permission_changed: false,
            budget_debited: false,
            scheduler_admission_enforced: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn agent_card(
    role_id: &'static str,
    agent_card_id: &'static str,
    capability_ids: Vec<&'static str>,
    allowed_tools: Vec<&'static str>,
    budget_policy_id: &'static str,
    side_effect_class: &'static str,
    handoff_description: &'static str,
    output_contract: &'static str,
    verifier_id: &'static str,
    reducer_id: &'static str,
) -> WorkGraphAgentRoleAgentCardPreview {
    WorkGraphAgentRoleAgentCardPreview {
        role_id,
        agent_card_id,
        capability_ids,
        allowed_tools,
        budget_policy_id,
        side_effect_class,
        handoff_description,
        output_contract,
        verifier_id,
        reducer_id,
        lane: "hepta-backend",
        manifest_report_only: true,
        role_enforcement_enabled: false,
    }
}

fn source_binding(
    source_surface_id: &'static str,
    entrypoint_id: &'static str,
    required_role_ids: Vec<&'static str>,
) -> WorkGraphAgentRoleSourceBindingPreview {
    WorkGraphAgentRoleSourceBindingPreview {
        source_surface_id,
        entrypoint_id,
        required_role_ids,
        default_lane: "hepta-backend",
        output_contract: "TaskResultEnvelope",
        verifier_required: true,
        manifest_attached_report_only: true,
        enforcement_enabled: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_role_agent_card_manifest_declares_required_wire_fields() {
        let fields = work_graph_agent_role_agent_card_required_wire_fields();

        assert_eq!(fields.len(), 11);
        assert!(fields.contains(&"capabilities"));
        assert!(fields.contains(&"allowedTools"));
        assert!(fields.contains(&"budget"));
        assert!(fields.contains(&"sideEffectClass"));
        assert!(fields.contains(&"handoffDescription"));
        assert!(fields.contains(&"outputContract"));
        assert!(fields.contains(&"verifier"));
        assert!(fields.contains(&"lane"));
    }

    #[test]
    fn agent_role_agent_card_manifest_covers_role_cards() {
        let report = hepta_work_graph_agent_role_agent_card_manifest_report_only_report();
        let role_ids = report
            .agent_cards
            .iter()
            .map(|card| card.role_id)
            .collect::<Vec<_>>();

        assert_eq!(report.agent_card_count, 5);
        assert!(role_ids.contains(&"planner_role"));
        assert!(role_ids.contains(&"builder_role"));
        assert!(role_ids.contains(&"scheduler_role"));
        assert!(role_ids.contains(&"handoff_role"));
        assert!(report.agent_cards.iter().all(|card| {
            !card.capability_ids.is_empty()
                && !card.allowed_tools.is_empty()
                && card.output_contract == "TaskResultEnvelope"
                && card.lane == "hepta-backend"
                && card.manifest_report_only
                && !card.role_enforcement_enabled
        }));
    }

    #[test]
    fn agent_role_agent_card_manifest_binds_requested_sources() {
        let report = hepta_work_graph_agent_role_agent_card_manifest_report_only_report();
        let source_ids = report
            .source_bindings
            .iter()
            .map(|binding| binding.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(report.source_binding_count, 5);
        assert!(source_ids.contains(&"multi_agent_v2_thread_spawn"));
        assert!(source_ids.contains(&"agent_jobs_batch_workers"));
        assert!(source_ids.contains(&"hepta_runtime_worker_tasks"));
        assert!(source_ids.contains(&"hepta_runtime_agent_harness"));
        assert!(
            report.source_bindings.iter().all(
                |binding| binding.manifest_attached_report_only && !binding.enforcement_enabled
            )
        );
    }

    #[test]
    fn agent_role_agent_card_manifest_remains_report_only() {
        let report = hepta_work_graph_agent_role_agent_card_manifest_report_only_report();

        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_PERSISTENT_MAILBOX_HANDOFF_EVENT_MAPPING_GATE,
                WORK_GRAPH_ROLE_MANIFEST_CONTRACT_PREVIEW_GATE,
            ]
        );
        assert_eq!(
            report.source_persistent_mailbox_handoff_gate,
            WORK_GRAPH_PERSISTENT_MAILBOX_HANDOFF_EVENT_MAPPING_GATE
        );
        assert_eq!(
            report.source_role_manifest_contract_gate,
            WORK_GRAPH_ROLE_MANIFEST_CONTRACT_PREVIEW_GATE
        );
        assert_eq!(
            report.source_persistent_mailbox_handoff_required_prior_gate_count,
            1
        );
        assert_eq!(report.source_role_manifest_required_field_count, 12);
        assert_eq!(report.source_role_manifest_capability_count, 7);
        assert_eq!(report.source_role_manifest_permission_mode_count, 5);
        assert_eq!(report.source_role_manifest_invariant_count, 6);
        assert_eq!(report.source_role_manifest_adapter_preview_count, 4);
        assert!(report.source_persistent_mailbox_handoff_readiness_complete);
        assert!(report.source_persistent_mailbox_handoff_no_persistence_confirmed);
        assert!(report.source_role_manifest_contract_ready);
        assert!(report.source_role_manifest_no_enforcement_confirmed);
        assert!(report.agent_role_agent_card_manifest_readiness_complete);
        assert!(report.report_only_manifest_attached);
        assert!(!report.role_enforcement_enabled);
        assert!(report.ready_for_trace_guardrail_span);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphAgentRoleAgentCardManifestReportOnlySideEffects::none()
        );
    }
}
