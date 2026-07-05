use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_canary_readback_replay::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_trace_guardrail_span_report_only::WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_GATE: &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorPacketReportOnlyReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub operator_packet_section_count: usize,
    pub review_item_count: usize,
    pub evidence_ref_count: usize,
    pub blocked_action_count: usize,
    pub required_prior_gate_count: usize,
    pub operator_packet_sections: Vec<WorkGraphFeatureFlagOperatorPacketSectionPreview>,
    pub review_items: Vec<WorkGraphFeatureFlagOperatorPacketReviewItemPreview>,
    pub evidence_refs: Vec<WorkGraphFeatureFlagOperatorPacketEvidenceRefPreview>,
    pub blocked_actions: Vec<WorkGraphFeatureFlagOperatorPacketBlockedActionPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub operator_packet_visible: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_recorded: bool,
    pub operator_packet_persisted: bool,
    pub operator_packet_authorizes_config_write: bool,
    pub operator_packet_authorizes_canary_traffic: bool,
    pub operator_packet_authorizes_live_cutover: bool,
    pub ready_for_operator_packet_non_send_readback: bool,
    pub ready_for_feature_flag_config_write: bool,
    pub ready_for_feature_flag_enablement: bool,
    pub ready_for_live_cutover: bool,
    pub side_effects: WorkGraphAgentJobsTaskBoardFeatureFlagOperatorPacketReportOnlySideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagOperatorPacketSectionPreview {
    pub id: &'static str,
    pub title: &'static str,
    pub source_gate: &'static str,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagOperatorPacketReviewItemPreview {
    pub id: &'static str,
    pub flag_id: &'static str,
    pub review_surface_id: &'static str,
    pub decision_state: &'static str,
    pub required_before_enablement: bool,
    pub config_write_authorized: bool,
    pub canary_traffic_authorized: bool,
    pub live_cutover_authorized: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagOperatorPacketEvidenceRefPreview {
    pub id: &'static str,
    pub evidence_type: &'static str,
    pub source_gate: &'static str,
    pub required: bool,
    pub redacted: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagOperatorPacketBlockedActionPreview {
    pub id: &'static str,
    pub action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorPacketReportOnlySideEffects {
    pub filesystem_written: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_recorded: bool,
    pub operator_packet_persisted: bool,
    pub operator_packet_accepted: bool,
    pub approval_recorded: bool,
    pub config_written: bool,
    pub feature_flag_mutated: bool,
    pub non_blocking_canary_enabled: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_report()
-> WorkGraphAgentJobsTaskBoardFeatureFlagOperatorPacketReportOnlyReport {
    let operator_packet_sections =
        work_graph_agent_jobs_task_board_feature_flag_operator_packet_sections();
    let review_items = work_graph_agent_jobs_task_board_feature_flag_operator_packet_review_items();
    let evidence_refs =
        work_graph_agent_jobs_task_board_feature_flag_operator_packet_evidence_refs();
    let blocked_actions =
        work_graph_agent_jobs_task_board_feature_flag_operator_packet_blocked_actions();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_feature_flag_operator_packet_required_prior_gates();

    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorPacketReportOnlyReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_SCHEMA_VERSION,
        preview_mode: "feature_flag_operator_packet_report_only_no_approval_no_send_no_persistence",
        operator_packet_section_count: operator_packet_sections.len(),
        review_item_count: review_items.len(),
        evidence_ref_count: evidence_refs.len(),
        blocked_action_count: blocked_actions.len(),
        required_prior_gate_count: required_prior_gates.len(),
        operator_packet_sections,
        review_items,
        evidence_refs,
        blocked_actions,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_RECOMMENDED_NEXT_GATE,
        operator_packet_visible: true,
        operator_packet_sent: false,
        operator_packet_recorded: false,
        operator_packet_persisted: false,
        operator_packet_authorizes_config_write: false,
        operator_packet_authorizes_canary_traffic: false,
        operator_packet_authorizes_live_cutover: false,
        ready_for_operator_packet_non_send_readback: true,
        ready_for_feature_flag_config_write: false,
        ready_for_feature_flag_enablement: false,
        ready_for_live_cutover: false,
        side_effects: WorkGraphAgentJobsTaskBoardFeatureFlagOperatorPacketReportOnlySideEffects::none(
        ),
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_packet_sections()
-> Vec<WorkGraphFeatureFlagOperatorPacketSectionPreview> {
    vec![
        operator_packet_section(
            "scope_and_canary_flags",
            "Scope and canary flags",
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE,
        ),
        operator_packet_section(
            "config_contract_and_digest",
            "Config contract and digest preview",
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE,
        ),
        operator_packet_section(
            "readback_replay_evidence",
            "Readback and replay evidence",
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE,
        ),
        operator_packet_section(
            "trace_guardrail_evidence",
            "Trace and guardrail evidence",
            WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
        ),
        operator_packet_section(
            "scheduler_admission_evidence",
            "Scheduler admission dry-run evidence",
            WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_packet_review_items()
-> Vec<WorkGraphFeatureFlagOperatorPacketReviewItemPreview> {
    vec![
        review_item(
            "agent_jobs_canary_flag_operator_review",
            "work_graph_agent_jobs_non_blocking_canary",
        ),
        review_item(
            "task_board_canary_flag_operator_review",
            "work_graph_task_board_non_blocking_canary",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_packet_evidence_refs()
-> Vec<WorkGraphFeatureFlagOperatorPacketEvidenceRefPreview> {
    vec![
        evidence_ref(
            "feature_flag_non_blocking_canary_report",
            "canary_flag_shape",
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE,
        ),
        evidence_ref(
            "feature_flag_config_wiring_report",
            "config_contract_digest",
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE,
        ),
        evidence_ref(
            "canary_readback_replay_report",
            "readback_replay_diff",
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE,
        ),
        evidence_ref(
            "trace_guardrail_span_report",
            "blocking_guardrail_preview",
            WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
        ),
        evidence_ref(
            "scheduler_admission_dry_run_report",
            "allow_deny_explanation",
            WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_packet_blocked_actions()
-> Vec<WorkGraphFeatureFlagOperatorPacketBlockedActionPreview> {
    vec![
        blocked_action(
            "operator_packet_delivery_blocked",
            "send_operator_packet",
            "operator packet is only assembled as report-only evidence",
        ),
        blocked_action(
            "operator_packet_recording_blocked",
            "record_operator_approval",
            "no operator approval or packet acceptance is recorded by this gate",
        ),
        blocked_action(
            "feature_flag_config_write_blocked",
            "write_feature_flag_config",
            "config writing remains disabled until operator packet readback and approval are explicit",
        ),
        blocked_action(
            "feature_flag_enablement_blocked",
            "enable_feature_flag",
            "canary flags remain default/current off with zero traffic",
        ),
        blocked_action(
            "canary_traffic_blocked",
            "route_non_blocking_canary_traffic",
            "0ppm report-only observation remains the only allowed stage",
        ),
        blocked_action(
            "live_cutover_blocked",
            "perform_live_cutover",
            "live WorkGraph cutover is outside this report-only gate",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_packet_required_prior_gates()
-> Vec<&'static str> {
    vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE,
        WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
        WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
    ]
}

impl WorkGraphAgentJobsTaskBoardFeatureFlagOperatorPacketReportOnlySideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            operator_packet_sent: false,
            operator_packet_recorded: false,
            operator_packet_persisted: false,
            operator_packet_accepted: false,
            approval_recorded: false,
            config_written: false,
            feature_flag_mutated: false,
            non_blocking_canary_enabled: false,
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

fn operator_packet_section(
    id: &'static str,
    title: &'static str,
    source_gate: &'static str,
) -> WorkGraphFeatureFlagOperatorPacketSectionPreview {
    WorkGraphFeatureFlagOperatorPacketSectionPreview {
        id,
        title,
        source_gate,
        required: true,
    }
}

fn review_item(
    id: &'static str,
    flag_id: &'static str,
) -> WorkGraphFeatureFlagOperatorPacketReviewItemPreview {
    WorkGraphFeatureFlagOperatorPacketReviewItemPreview {
        id,
        flag_id,
        review_surface_id: "work_graph_agent_jobs_task_board_feature_flag_operator_packet",
        decision_state: "pending_operator_review",
        required_before_enablement: true,
        config_write_authorized: false,
        canary_traffic_authorized: false,
        live_cutover_authorized: false,
    }
}

fn evidence_ref(
    id: &'static str,
    evidence_type: &'static str,
    source_gate: &'static str,
) -> WorkGraphFeatureFlagOperatorPacketEvidenceRefPreview {
    WorkGraphFeatureFlagOperatorPacketEvidenceRefPreview {
        id,
        evidence_type,
        source_gate,
        required: true,
        redacted: true,
        persisted: false,
    }
}

fn blocked_action(
    id: &'static str,
    action: &'static str,
    reason: &'static str,
) -> WorkGraphFeatureFlagOperatorPacketBlockedActionPreview {
    WorkGraphFeatureFlagOperatorPacketBlockedActionPreview {
        id,
        action,
        blocked: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_flag_operator_packet_declares_sections_and_evidence() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_report(
            );

        assert_eq!(report.operator_packet_section_count, 5);
        assert_eq!(report.evidence_ref_count, 5);
        assert!(
            report
                .operator_packet_sections
                .iter()
                .all(|section| section.required)
        );
        assert!(
            report
                .evidence_refs
                .iter()
                .all(|evidence| { evidence.required && evidence.redacted && !evidence.persisted })
        );
        assert!(report.operator_packet_visible);
        assert!(report.ready_for_operator_packet_non_send_readback);
    }

    #[test]
    fn feature_flag_operator_packet_review_items_do_not_authorize_enablement() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_report(
            );
        let flag_ids = report
            .review_items
            .iter()
            .map(|item| item.flag_id)
            .collect::<Vec<_>>();

        assert_eq!(report.review_item_count, 2);
        assert!(flag_ids.contains(&"work_graph_agent_jobs_non_blocking_canary"));
        assert!(flag_ids.contains(&"work_graph_task_board_non_blocking_canary"));
        assert!(report.review_items.iter().all(|item| {
            item.decision_state == "pending_operator_review"
                && item.required_before_enablement
                && !item.config_write_authorized
                && !item.canary_traffic_authorized
                && !item.live_cutover_authorized
        }));
        assert!(!report.operator_packet_authorizes_config_write);
        assert!(!report.operator_packet_authorizes_canary_traffic);
        assert!(!report.operator_packet_authorizes_live_cutover);
        assert!(!report.ready_for_feature_flag_config_write);
        assert!(!report.ready_for_feature_flag_enablement);
        assert!(!report.ready_for_live_cutover);
    }

    #[test]
    fn feature_flag_operator_packet_blocks_mutating_actions_and_requires_priors() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_report(
            );

        assert_eq!(report.blocked_action_count, 6);
        assert!(report.blocked_actions.iter().all(|action| action.blocked));
        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE,
                WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
                WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
            ]
        );
        assert_eq!(report.required_prior_gate_count, 6);
    }

    #[test]
    fn feature_flag_operator_packet_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_report(
            );

        assert!(!report.operator_packet_sent);
        assert!(!report.operator_packet_recorded);
        assert!(!report.operator_packet_persisted);
        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorPacketReportOnlySideEffects::none()
        );
    }
}
