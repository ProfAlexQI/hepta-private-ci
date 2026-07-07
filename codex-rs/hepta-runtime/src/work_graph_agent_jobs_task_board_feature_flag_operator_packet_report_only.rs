use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_canary_readback_replay::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE;
use crate::work_graph_agent_jobs_task_board_canary_readback_replay::WorkGraphAgentJobsTaskBoardCanaryReadbackReplaySideEffects;
use crate::work_graph_agent_jobs_task_board_canary_readback_replay::hepta_work_graph_agent_jobs_task_board_canary_readback_replay_report;
use crate::work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only::WorkGraphAgentJobsTaskBoardFeatureFlagConfigWiringReportOnlySideEffects;
use crate::work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only::hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_report;
use crate::work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary::WorkGraphAgentJobsTaskBoardFeatureFlagNonBlockingCanarySideEffects;
use crate::work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary::hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_report;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WorkGraphAgentJobsTaskBoardReportOnlyEntrypointEmissionSideEffects;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_report;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WorkGraphSchedulerAdmissionDryRunEnforcementSideEffects;
use crate::work_graph_scheduler_admission_dry_run_enforcement::hepta_work_graph_scheduler_admission_dry_run_enforcement_report;
use crate::work_graph_trace_guardrail_span_report_only::WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE;
use crate::work_graph_trace_guardrail_span_report_only::WorkGraphTraceGuardrailSpanReportOnlySideEffects;
use crate::work_graph_trace_guardrail_span_report_only::hepta_work_graph_trace_guardrail_span_report_only_report;

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
    pub source_config_wiring_required_prior_gate_count: usize,
    pub source_config_wiring_config_contract_count: usize,
    pub source_config_wiring_config_digest_preview_count: usize,
    pub source_config_wiring_source_binding_count: usize,
    pub source_feature_flag_non_blocking_canary_required_prior_gate_count: usize,
    pub source_feature_flag_count: usize,
    pub source_feature_flag_safety_check_count: usize,
    pub source_canary_readback_replay_required_prior_gate_count: usize,
    pub source_canary_readback_replay_entrypoint_count: usize,
    pub source_canary_readback_replay_readback_evidence_count: usize,
    pub source_canary_readback_replay_replay_diff_count: usize,
    pub source_entrypoint_emission_entrypoint_count: usize,
    pub source_entrypoint_emission_emission_count: usize,
    pub source_trace_guardrail_span_count: usize,
    pub source_trace_guardrail_blocking_guardrail_count: usize,
    pub source_scheduler_admission_entrypoint_count: usize,
    pub source_scheduler_admission_required_prior_gate_count: usize,
    pub operator_packet_sections: Vec<WorkGraphFeatureFlagOperatorPacketSectionPreview>,
    pub review_items: Vec<WorkGraphFeatureFlagOperatorPacketReviewItemPreview>,
    pub evidence_refs: Vec<WorkGraphFeatureFlagOperatorPacketEvidenceRefPreview>,
    pub blocked_actions: Vec<WorkGraphFeatureFlagOperatorPacketBlockedActionPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub source_config_wiring_gate: &'static str,
    pub source_feature_flag_non_blocking_canary_gate: &'static str,
    pub source_canary_readback_replay_gate: &'static str,
    pub source_entrypoint_emission_gate: &'static str,
    pub source_trace_guardrail_gate: &'static str,
    pub source_scheduler_admission_dry_run_gate: &'static str,
    pub recommended_next_gate: &'static str,
    pub source_config_wiring_ready: bool,
    pub source_config_wiring_no_write_confirmed: bool,
    pub source_feature_flag_non_blocking_canary_ready: bool,
    pub source_feature_flag_non_blocking_canary_no_enablement_confirmed: bool,
    pub source_canary_readback_replay_ready: bool,
    pub source_canary_readback_replay_no_live_confirmed: bool,
    pub source_entrypoint_emission_readiness_complete: bool,
    pub source_entrypoint_emission_no_live_confirmed: bool,
    pub source_trace_guardrail_readiness_complete: bool,
    pub source_trace_guardrail_no_live_blocking_confirmed: bool,
    pub source_scheduler_admission_dry_run_ready: bool,
    pub source_scheduler_admission_no_live_blocking_confirmed: bool,
    pub operator_packet_prior_readbacks_complete: bool,
    pub operator_packet_sections_report_only_complete: bool,
    pub operator_packet_review_items_non_authorizing: bool,
    pub operator_packet_evidence_refs_report_only_complete: bool,
    pub operator_packet_blocked_actions_complete: bool,
    pub operator_packet_report_only_preconditions_complete: bool,
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
    let config_wiring =
        hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_report();
    let feature_flag_non_blocking_canary =
        hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_report();
    let canary_readback_replay =
        hepta_work_graph_agent_jobs_task_board_canary_readback_replay_report();
    let entrypoint_emission =
        hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_report();
    let trace_guardrail = hepta_work_graph_trace_guardrail_span_report_only_report();
    let scheduler_admission = hepta_work_graph_scheduler_admission_dry_run_enforcement_report();
    let source_config_wiring_no_write_confirmed = !config_wiring
        .ready_for_feature_flag_config_write
        && !config_wiring.ready_for_feature_flag_enablement
        && !config_wiring.ready_for_live_cutover
        && config_wiring.side_effects
            == WorkGraphAgentJobsTaskBoardFeatureFlagConfigWiringReportOnlySideEffects::none();
    let source_config_wiring_ready = config_wiring.gate
        == WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE
        && config_wiring.config_wiring_prior_readbacks_complete
        && config_wiring.config_wiring_report_only_preconditions_complete
        && config_wiring.ready_for_operator_packet_report_only
        && source_config_wiring_no_write_confirmed;
    let source_feature_flag_non_blocking_canary_no_enablement_confirmed =
        !feature_flag_non_blocking_canary.ready_for_feature_flag_enablement
            && !feature_flag_non_blocking_canary.ready_for_live_cutover
            && feature_flag_non_blocking_canary.side_effects
                == WorkGraphAgentJobsTaskBoardFeatureFlagNonBlockingCanarySideEffects::none();
    let source_feature_flag_non_blocking_canary_ready = feature_flag_non_blocking_canary.gate
        == WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE
        && feature_flag_non_blocking_canary.feature_flag_prior_readbacks_complete
        && feature_flag_non_blocking_canary
            .feature_flag_enablement_preconditions_report_only_complete
        && feature_flag_non_blocking_canary.ready_for_feature_flag_config_wiring
        && source_feature_flag_non_blocking_canary_no_enablement_confirmed;
    let source_canary_readback_replay_no_live_confirmed = !canary_readback_replay
        .feature_flag_enabled
        && !canary_readback_replay.ready_for_live_cutover
        && canary_readback_replay.side_effects
            == WorkGraphAgentJobsTaskBoardCanaryReadbackReplaySideEffects::none();
    let source_canary_readback_replay_ready = canary_readback_replay.gate
        == WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE
        && canary_readback_replay.canary_readback_replay_prior_readbacks_complete
        && canary_readback_replay.canary_projection_readback_replay_preview_complete
        && canary_readback_replay.ready_for_non_blocking_canary
        && source_canary_readback_replay_no_live_confirmed;
    let source_entrypoint_emission_no_live_confirmed = !entrypoint_emission
        .ready_for_live_execution
        && entrypoint_emission.side_effects
            == WorkGraphAgentJobsTaskBoardReportOnlyEntrypointEmissionSideEffects::none();
    let source_entrypoint_emission_readiness_complete = entrypoint_emission.gate
        == WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE
        && entrypoint_emission.entrypoint_emission_readiness_complete
        && entrypoint_emission.ready_for_canary_readback_replay_gate
        && source_entrypoint_emission_no_live_confirmed;
    let source_trace_guardrail_no_live_blocking_confirmed = !trace_guardrail
        .live_guardrail_enforcement_enabled
        && !trace_guardrail.ready_for_live_execution
        && trace_guardrail.side_effects == WorkGraphTraceGuardrailSpanReportOnlySideEffects::none();
    let source_trace_guardrail_readiness_complete = trace_guardrail.gate
        == WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE
        && trace_guardrail.trace_guardrail_prior_readbacks_complete
        && trace_guardrail.ready_for_agent_jobs_task_board_report_only_emission
        && source_trace_guardrail_no_live_blocking_confirmed;
    let source_scheduler_admission_no_live_blocking_confirmed = !scheduler_admission
        .live_blocking_enforcement_enabled
        && !scheduler_admission.ready_for_live_execution
        && scheduler_admission.side_effects
            == WorkGraphSchedulerAdmissionDryRunEnforcementSideEffects::none();
    let source_scheduler_admission_dry_run_ready = scheduler_admission.gate
        == WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE
        && scheduler_admission.dry_run_enforcement_enabled
        && scheduler_admission.ready_for_append_only_event_store_shadow_path
        && source_scheduler_admission_no_live_blocking_confirmed;
    let operator_packet_prior_readbacks_complete = source_config_wiring_ready
        && source_feature_flag_non_blocking_canary_ready
        && source_canary_readback_replay_ready
        && source_entrypoint_emission_readiness_complete
        && source_trace_guardrail_readiness_complete
        && source_scheduler_admission_dry_run_ready;
    let operator_packet_sections_report_only_complete = !operator_packet_sections.is_empty()
        && operator_packet_sections
            .iter()
            .all(|section| section.required);
    let operator_packet_review_items_non_authorizing = !review_items.is_empty()
        && review_items.iter().all(|item| {
            item.decision_state == "pending_operator_review"
                && item.required_before_enablement
                && !item.config_write_authorized
                && !item.canary_traffic_authorized
                && !item.live_cutover_authorized
        });
    let operator_packet_evidence_refs_report_only_complete = !evidence_refs.is_empty()
        && evidence_refs
            .iter()
            .all(|evidence| evidence.required && evidence.redacted && !evidence.persisted);
    let operator_packet_blocked_actions_complete =
        !blocked_actions.is_empty() && blocked_actions.iter().all(|action| action.blocked);
    let operator_packet_report_only_preconditions_complete =
        operator_packet_prior_readbacks_complete
            && operator_packet_sections_report_only_complete
            && operator_packet_review_items_non_authorizing
            && operator_packet_evidence_refs_report_only_complete
            && operator_packet_blocked_actions_complete;

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
        source_config_wiring_required_prior_gate_count: config_wiring.required_prior_gate_count,
        source_config_wiring_config_contract_count: config_wiring.config_contract_count,
        source_config_wiring_config_digest_preview_count: config_wiring
            .config_digest_preview_count,
        source_config_wiring_source_binding_count: config_wiring.source_binding_count,
        source_feature_flag_non_blocking_canary_required_prior_gate_count:
            feature_flag_non_blocking_canary.required_prior_gate_count,
        source_feature_flag_count: feature_flag_non_blocking_canary.feature_flag_count,
        source_feature_flag_safety_check_count: feature_flag_non_blocking_canary
            .safety_check_count,
        source_canary_readback_replay_required_prior_gate_count: canary_readback_replay
            .required_prior_gate_count,
        source_canary_readback_replay_entrypoint_count: canary_readback_replay
            .canary_entrypoint_count,
        source_canary_readback_replay_readback_evidence_count: canary_readback_replay
            .readback_evidence_count,
        source_canary_readback_replay_replay_diff_count: canary_readback_replay.replay_diff_count,
        source_entrypoint_emission_entrypoint_count: entrypoint_emission.entrypoint_count,
        source_entrypoint_emission_emission_count: entrypoint_emission.emission_count,
        source_trace_guardrail_span_count: trace_guardrail.span_count,
        source_trace_guardrail_blocking_guardrail_count: trace_guardrail.blocking_guardrail_count,
        source_scheduler_admission_entrypoint_count: scheduler_admission.entrypoint_count,
        source_scheduler_admission_required_prior_gate_count: scheduler_admission
            .required_prior_gates
            .len(),
        operator_packet_sections,
        review_items,
        evidence_refs,
        blocked_actions,
        required_prior_gates,
        source_config_wiring_gate: config_wiring.gate,
        source_feature_flag_non_blocking_canary_gate: feature_flag_non_blocking_canary.gate,
        source_canary_readback_replay_gate: canary_readback_replay.gate,
        source_entrypoint_emission_gate: entrypoint_emission.gate,
        source_trace_guardrail_gate: trace_guardrail.gate,
        source_scheduler_admission_dry_run_gate: scheduler_admission.gate,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_RECOMMENDED_NEXT_GATE,
        source_config_wiring_ready,
        source_config_wiring_no_write_confirmed,
        source_feature_flag_non_blocking_canary_ready,
        source_feature_flag_non_blocking_canary_no_enablement_confirmed,
        source_canary_readback_replay_ready,
        source_canary_readback_replay_no_live_confirmed,
        source_entrypoint_emission_readiness_complete,
        source_entrypoint_emission_no_live_confirmed,
        source_trace_guardrail_readiness_complete,
        source_trace_guardrail_no_live_blocking_confirmed,
        source_scheduler_admission_dry_run_ready,
        source_scheduler_admission_no_live_blocking_confirmed,
        operator_packet_prior_readbacks_complete,
        operator_packet_sections_report_only_complete,
        operator_packet_review_items_non_authorizing,
        operator_packet_evidence_refs_report_only_complete,
        operator_packet_blocked_actions_complete,
        operator_packet_report_only_preconditions_complete,
        operator_packet_visible: true,
        operator_packet_sent: false,
        operator_packet_recorded: false,
        operator_packet_persisted: false,
        operator_packet_authorizes_config_write: false,
        operator_packet_authorizes_canary_traffic: false,
        operator_packet_authorizes_live_cutover: false,
        ready_for_operator_packet_non_send_readback:
            operator_packet_report_only_preconditions_complete,
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
    fn feature_flag_operator_packet_consumes_prior_report_readbacks() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_report(
            );

        assert_eq!(
            report.source_config_wiring_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE
        );
        assert_eq!(
            report.source_feature_flag_non_blocking_canary_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE
        );
        assert_eq!(
            report.source_canary_readback_replay_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE
        );
        assert_eq!(
            report.source_entrypoint_emission_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE
        );
        assert_eq!(
            report.source_trace_guardrail_gate,
            WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE
        );
        assert_eq!(
            report.source_scheduler_admission_dry_run_gate,
            WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE
        );
        assert_eq!(report.source_config_wiring_required_prior_gate_count, 5);
        assert_eq!(report.source_config_wiring_config_contract_count, 2);
        assert_eq!(report.source_config_wiring_config_digest_preview_count, 2);
        assert_eq!(report.source_config_wiring_source_binding_count, 2);
        assert_eq!(
            report.source_feature_flag_non_blocking_canary_required_prior_gate_count,
            4
        );
        assert_eq!(report.source_feature_flag_count, 2);
        assert_eq!(report.source_feature_flag_safety_check_count, 4);
        assert_eq!(
            report.source_canary_readback_replay_required_prior_gate_count,
            4
        );
        assert_eq!(report.source_canary_readback_replay_entrypoint_count, 2);
        assert_eq!(
            report.source_canary_readback_replay_readback_evidence_count,
            2
        );
        assert_eq!(report.source_canary_readback_replay_replay_diff_count, 2);
        assert_eq!(report.source_entrypoint_emission_entrypoint_count, 2);
        assert_eq!(report.source_entrypoint_emission_emission_count, 2);
        assert_eq!(report.source_trace_guardrail_span_count, 9);
        assert_eq!(report.source_trace_guardrail_blocking_guardrail_count, 6);
        assert_eq!(report.source_scheduler_admission_entrypoint_count, 4);
        assert_eq!(
            report.source_scheduler_admission_required_prior_gate_count,
            5
        );
        assert!(report.source_config_wiring_ready);
        assert!(report.source_config_wiring_no_write_confirmed);
        assert!(report.source_feature_flag_non_blocking_canary_ready);
        assert!(report.source_feature_flag_non_blocking_canary_no_enablement_confirmed);
        assert!(report.source_canary_readback_replay_ready);
        assert!(report.source_canary_readback_replay_no_live_confirmed);
        assert!(report.source_entrypoint_emission_readiness_complete);
        assert!(report.source_entrypoint_emission_no_live_confirmed);
        assert!(report.source_trace_guardrail_readiness_complete);
        assert!(report.source_trace_guardrail_no_live_blocking_confirmed);
        assert!(report.source_scheduler_admission_dry_run_ready);
        assert!(report.source_scheduler_admission_no_live_blocking_confirmed);
        assert!(report.operator_packet_prior_readbacks_complete);
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
        assert!(report.operator_packet_sections_report_only_complete);
        assert!(report.operator_packet_review_items_non_authorizing);
        assert!(report.operator_packet_evidence_refs_report_only_complete);
        assert!(report.operator_packet_blocked_actions_complete);
        assert!(report.operator_packet_report_only_preconditions_complete);
        assert!(report.ready_for_operator_packet_non_send_readback);
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
