use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphSurfaceAuditPacket {
    pub(crate) decision: &'static str,
    pub(crate) audit_stage: &'static str,
    pub(crate) job_id: String,
    pub(crate) source_surface_count: usize,
    pub(crate) governed_source_surface_count: usize,
    pub(crate) planning_source_surface_count: usize,
    pub(crate) runtime_source_surface_count: usize,
    pub(crate) observed_this_run_count: usize,
    pub(crate) durable_fact_source_count: usize,
    pub(crate) canonical_write_enabled_count: usize,
    pub(crate) result_contract_gap_count: usize,
    pub(crate) verifier_reducer_gap_count: usize,
    pub(crate) canonical_readiness_failed: bool,
    pub(crate) audit_packet_ready: bool,
    pub(crate) surface_entries: Vec<WorkGraphSurfaceAuditEntry>,
    pub(crate) audit_chain: WorkGraphAuditChainSummary,
    pub(crate) audit_blockers: Vec<String>,
    pub(crate) optimization_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) recommended_next_action: &'static str,
    pub(crate) operator_review_required: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) promotion_allowed: bool,
    pub(crate) promotion_prohibited_reason: &'static str,
    pub(crate) feature_flag_id: &'static str,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) blocking_guardrail_preview: bool,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphSurfaceAuditPacketSummary {
    pub(crate) decision: &'static str,
    pub(crate) audit_stage: &'static str,
    pub(crate) job_id: String,
    pub(crate) source_surface_count: usize,
    pub(crate) governed_source_surface_count: usize,
    pub(crate) planning_source_surface_count: usize,
    pub(crate) runtime_source_surface_count: usize,
    pub(crate) observed_this_run_count: usize,
    pub(crate) durable_fact_source_count: usize,
    pub(crate) canonical_write_enabled_count: usize,
    pub(crate) result_contract_gap_count: usize,
    pub(crate) verifier_reducer_gap_count: usize,
    pub(crate) canonical_readiness_failed: bool,
    pub(crate) audit_packet_ready: bool,
    pub(crate) audit_chain_segment_count: usize,
    pub(crate) audit_chain_ready_segment_count: usize,
    pub(crate) audit_chain_missing_segment_ids: Vec<String>,
    pub(crate) audit_chain_inconsistent_segment_ids: Vec<String>,
    pub(crate) audit_chain_ready: bool,
    pub(crate) audit_blocker_count: usize,
    pub(crate) optimization_blocker_count: usize,
    pub(crate) optimization_blockers: Vec<String>,
    pub(crate) operator_matrix_row_count: usize,
    pub(crate) operator_matrix_ready_row_count: usize,
    pub(crate) operator_matrix_blocked_row_count: usize,
    pub(crate) operator_matrix_rows: Vec<WorkGraphOperatorMatrixRow>,
    pub(crate) recommended_next_action: &'static str,
    pub(crate) operator_review_required: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) promotion_allowed: bool,
    pub(crate) promotion_prohibited_reason: &'static str,
    pub(crate) feature_flag_id: &'static str,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) blocking_guardrail_preview: bool,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphOperatorMatrixRow {
    pub(crate) source_surface_id: String,
    pub(crate) family: &'static str,
    pub(crate) owner_lane: &'static str,
    #[serde(skip_serializing_if = "bool_is_false")]
    pub(crate) observed_this_run: bool,
    #[serde(skip_serializing_if = "bool_is_false")]
    pub(crate) durable_fact_source_present: bool,
    #[serde(skip_serializing_if = "bool_is_false")]
    pub(crate) canonical_work_graph_write_enabled: bool,
    #[serde(skip_serializing_if = "bool_is_true")]
    pub(crate) row_auditable: bool,
    #[serde(skip_serializing_if = "bool_is_false")]
    pub(crate) result_contract_ready: bool,
    #[serde(skip_serializing_if = "bool_is_false")]
    pub(crate) verifier_reducer_ready: bool,
    #[serde(skip_serializing_if = "bool_is_false")]
    pub(crate) promotion_ready: bool,
    #[serde(skip_serializing_if = "bool_is_true")]
    pub(crate) replay_consistent: bool,
    #[serde(skip_serializing_if = "bool_is_true")]
    pub(crate) no_live_guardrail_ready: bool,
    #[serde(skip_serializing_if = "bool_is_false")]
    pub(crate) canonical_promotion_ready: bool,
    pub(crate) readiness_status: &'static str,
    pub(crate) next_blocker: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) task_result_contract_plan_decision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) task_result_contract_plan_ready: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) task_result_contract_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) terminal_delivery_surface: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) missing_task_result_contract_parts: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) task_result_contract_next_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) task_result_contract_next_action_count: Option<usize>,
    pub(crate) next_action: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphSurfaceAuditEntry {
    pub(crate) source_surface_id: String,
    pub(crate) family: &'static str,
    pub(crate) owner_lane: &'static str,
    pub(crate) present_in_current_head: bool,
    pub(crate) observed_this_run: bool,
    pub(crate) durable_fact_source_present: bool,
    pub(crate) canonical_work_graph_write_enabled: bool,
    pub(crate) shadow_only: bool,
    pub(crate) result_contract_required: bool,
    pub(crate) result_contract_present: bool,
    pub(crate) verifier_present: bool,
    pub(crate) reducer_present: bool,
    pub(crate) role_manifest_decision: String,
    pub(crate) promotion_readiness_decision: String,
    pub(crate) promotion_ready: bool,
    pub(crate) task_result_contract_plan_decision: String,
    pub(crate) task_result_contract_plan_ready: bool,
    pub(crate) task_result_contract_id: String,
    pub(crate) terminal_delivery_surface: String,
    pub(crate) missing_task_result_contract_parts: Vec<String>,
    pub(crate) task_result_contract_next_actions: Vec<String>,
    pub(crate) next_action: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphAuditChainSummary {
    pub(crate) segment_count: usize,
    pub(crate) ready_segment_count: usize,
    pub(crate) missing_segment_ids: Vec<String>,
    pub(crate) inconsistent_segment_ids: Vec<String>,
    pub(crate) chain_readback_ready: bool,
    pub(crate) chain_replay_consistent: bool,
    pub(crate) no_live_guardrails_ready: bool,
    pub(crate) chain_ready: bool,
    pub(crate) segments: Vec<WorkGraphAuditChainSegment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphAuditChainSegment {
    pub(crate) segment_id: String,
    pub(crate) event_type: String,
    pub(crate) event_count: usize,
    pub(crate) latest_payload_present: bool,
    pub(crate) latest_decision: String,
    pub(crate) readback_ready: bool,
    pub(crate) replay_consistent: bool,
    pub(crate) no_live_guardrail_ready: bool,
    pub(crate) ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionShadowReceipt {
    pub(crate) decision: &'static str,
    pub(crate) projection_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) source_packet_decision: &'static str,
    pub(crate) source_surface_count: usize,
    pub(crate) operator_matrix_row_count: usize,
    pub(crate) projected_work_node_count: usize,
    pub(crate) projected_work_edge_count: usize,
    pub(crate) projected_task_result_count: usize,
    pub(crate) projected_timeline_event_count: usize,
    pub(crate) read_projection_ready: bool,
    pub(crate) write_projection_ready: bool,
    pub(crate) projection_receipt_ready: bool,
    pub(crate) projection_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) projection_rows: Vec<WorkGraphCanonicalProjectionRow>,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
    pub(crate) recommended_next_action: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionRow {
    pub(crate) source_surface_id: String,
    pub(crate) family: &'static str,
    pub(crate) node_kind: &'static str,
    pub(crate) row_auditable: bool,
    pub(crate) durable_fact_source_present: bool,
    pub(crate) result_contract_ready: bool,
    pub(crate) verifier_reducer_ready: bool,
    pub(crate) replay_consistent: bool,
    pub(crate) no_live_guardrail_ready: bool,
    pub(crate) read_projection_ready: bool,
    pub(crate) write_projection_ready: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) next_blocker: &'static str,
    pub(crate) next_action: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionReplayConsistencyDecision {
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) replay_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) projection_receipt_decision: &'static str,
    pub(crate) projection_receipt_events: usize,
    pub(crate) prior_projection_replay_consistency_events: usize,
    pub(crate) projection_receipt_readback_ready: bool,
    pub(crate) projection_receipt_matches_readback: bool,
    pub(crate) projection_receipt_ready: bool,
    pub(crate) read_projection_ready: bool,
    pub(crate) write_projection_ready: bool,
    pub(crate) no_live_guardrails_ready: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) replay_consistent: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) consistency_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionCloseoutReceipt {
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) closeout_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) projection_receipt_decision: &'static str,
    pub(crate) replay_consistency_decision: &'static str,
    pub(crate) projection_receipt_events: usize,
    pub(crate) projection_replay_consistency_events: usize,
    pub(crate) prior_projection_closeout_receipt_events: usize,
    pub(crate) projection_receipt_readback_ready: bool,
    pub(crate) projection_replay_consistency_ready: bool,
    pub(crate) projection_receipt_ready: bool,
    pub(crate) read_projection_ready: bool,
    pub(crate) write_projection_ready: bool,
    pub(crate) projection_replay_consistent: bool,
    pub(crate) projected_work_node_count: usize,
    pub(crate) projected_work_edge_count: usize,
    pub(crate) projected_task_result_count: usize,
    pub(crate) projected_timeline_event_count: usize,
    pub(crate) closeout_ready: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) no_cutover_terminal_receipt: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) closeout_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
    pub(crate) recommended_next_action: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionCloseoutReplayConsistencyDecision {
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) replay_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) closeout_receipt_decision: &'static str,
    pub(crate) closeout_receipt_events: usize,
    pub(crate) prior_closeout_replay_consistency_events: usize,
    pub(crate) closeout_receipt_readback_ready: bool,
    pub(crate) closeout_receipt_matches_readback: bool,
    pub(crate) closeout_receipt_ready: bool,
    pub(crate) no_cutover_terminal_receipt: bool,
    pub(crate) no_live_guardrails_ready: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) replay_consistent: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) consistency_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionAuditChainCloseoutReceipt {
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) audit_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) projection_receipt_decision: &'static str,
    pub(crate) projection_replay_consistency_decision: &'static str,
    pub(crate) closeout_receipt_decision: &'static str,
    pub(crate) closeout_replay_consistency_decision: &'static str,
    pub(crate) projection_receipt_events: usize,
    pub(crate) projection_replay_consistency_events: usize,
    pub(crate) closeout_receipt_events: usize,
    pub(crate) closeout_replay_consistency_events: usize,
    pub(crate) prior_audit_chain_closeout_receipt_events: usize,
    pub(crate) projection_receipt_readback_ready: bool,
    pub(crate) projection_replay_consistency_ready: bool,
    pub(crate) closeout_receipt_readback_ready: bool,
    pub(crate) closeout_replay_consistency_ready: bool,
    pub(crate) projection_receipt_ready: bool,
    pub(crate) projection_replay_consistent: bool,
    pub(crate) closeout_receipt_ready: bool,
    pub(crate) closeout_replay_consistent: bool,
    pub(crate) projected_work_node_count: usize,
    pub(crate) projected_work_edge_count: usize,
    pub(crate) projected_task_result_count: usize,
    pub(crate) projected_timeline_event_count: usize,
    pub(crate) no_cutover_terminal_receipt: bool,
    pub(crate) audit_chain_closeout_ready: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) audit_chain_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
    pub(crate) recommended_next_action: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionAuditChainCloseoutReplayConsistencyDecision {
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) replay_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) audit_chain_closeout_receipt_decision: &'static str,
    pub(crate) audit_chain_closeout_receipt_events: usize,
    pub(crate) prior_audit_chain_closeout_replay_consistency_events: usize,
    pub(crate) audit_chain_closeout_receipt_readback_ready: bool,
    pub(crate) audit_chain_closeout_receipt_matches_readback: bool,
    pub(crate) audit_chain_closeout_receipt_ready: bool,
    pub(crate) no_cutover_terminal_receipt: bool,
    pub(crate) no_live_guardrails_ready: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) replay_consistent: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) consistency_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionEnablementOperatorReviewPacket {
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) review_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) projection_receipt_decision: &'static str,
    pub(crate) projection_replay_consistency_decision: &'static str,
    pub(crate) closeout_receipt_decision: &'static str,
    pub(crate) closeout_replay_consistency_decision: &'static str,
    pub(crate) audit_chain_closeout_receipt_decision: &'static str,
    pub(crate) audit_chain_closeout_replay_consistency_decision: &'static str,
    pub(crate) projection_receipt_events: usize,
    pub(crate) projection_replay_consistency_events: usize,
    pub(crate) closeout_receipt_events: usize,
    pub(crate) closeout_replay_consistency_events: usize,
    pub(crate) audit_chain_closeout_receipt_events: usize,
    pub(crate) audit_chain_closeout_replay_consistency_events: usize,
    pub(crate) prior_enablement_operator_review_packet_events: usize,
    pub(crate) projection_receipt_readback_ready: bool,
    pub(crate) projection_replay_consistency_ready: bool,
    pub(crate) closeout_receipt_readback_ready: bool,
    pub(crate) closeout_replay_consistency_ready: bool,
    pub(crate) audit_chain_closeout_receipt_readback_ready: bool,
    pub(crate) audit_chain_closeout_replay_consistency_ready: bool,
    pub(crate) projection_receipt_ready: bool,
    pub(crate) projection_replay_consistent: bool,
    pub(crate) closeout_receipt_ready: bool,
    pub(crate) closeout_replay_consistent: bool,
    pub(crate) audit_chain_closeout_ready: bool,
    pub(crate) audit_chain_closeout_replay_consistent: bool,
    pub(crate) no_cutover_terminal_receipt: bool,
    pub(crate) no_live_enablement_rehearsal_ready: bool,
    pub(crate) enablement_operator_review_ready: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) projected_work_node_count: usize,
    pub(crate) projected_work_edge_count: usize,
    pub(crate) projected_task_result_count: usize,
    pub(crate) projected_timeline_event_count: usize,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) operator_review_required: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) reviewed_flag_enabled: bool,
    pub(crate) enablement_allowed: bool,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
    pub(crate) enablement_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) recommended_next_action: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyDecision {
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) replay_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) enablement_operator_review_decision: &'static str,
    pub(crate) enablement_operator_review_packet_events: usize,
    pub(crate) prior_enablement_operator_review_replay_consistency_events: usize,
    pub(crate) enablement_operator_review_packet_readback_ready: bool,
    pub(crate) enablement_operator_review_packet_matches_readback: bool,
    pub(crate) enablement_operator_review_ready: bool,
    pub(crate) no_live_enablement_rehearsal_ready: bool,
    pub(crate) enablement_allowed: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) reviewed_flag_enabled: bool,
    pub(crate) no_live_guardrails_ready: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) replay_consistent: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) consistency_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReceipt {
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) closeout_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) enablement_operator_review_decision: &'static str,
    pub(crate) enablement_operator_review_replay_consistency_decision: &'static str,
    pub(crate) enablement_operator_review_packet_events: usize,
    pub(crate) enablement_operator_review_replay_consistency_events: usize,
    pub(crate) prior_enablement_no_live_rehearsal_closeout_events: usize,
    pub(crate) enablement_operator_review_packet_readback_ready: bool,
    pub(crate) enablement_operator_review_replay_consistency_ready: bool,
    pub(crate) enablement_operator_review_ready: bool,
    pub(crate) enablement_operator_review_replay_consistent: bool,
    pub(crate) no_live_enablement_rehearsal_ready: bool,
    pub(crate) no_live_enablement_rehearsal_closeout_ready: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) enablement_allowed: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) reviewed_flag_enabled: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
    pub(crate) closeout_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) recommended_next_action: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyDecision
{
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) replay_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) no_live_rehearsal_closeout_decision: &'static str,
    pub(crate) no_live_rehearsal_closeout_events: usize,
    pub(crate) prior_no_live_rehearsal_closeout_replay_consistency_events: usize,
    pub(crate) no_live_rehearsal_closeout_readback_ready: bool,
    pub(crate) no_live_rehearsal_closeout_matches_readback: bool,
    pub(crate) no_live_enablement_rehearsal_ready: bool,
    pub(crate) no_live_enablement_rehearsal_closeout_ready: bool,
    pub(crate) enablement_allowed: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) reviewed_flag_enabled: bool,
    pub(crate) no_live_guardrails_ready: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) replay_consistent: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) consistency_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReceipt {
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) closeout_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) enablement_operator_review_decision: &'static str,
    pub(crate) enablement_operator_review_replay_consistency_decision: &'static str,
    pub(crate) no_live_rehearsal_closeout_decision: &'static str,
    pub(crate) no_live_rehearsal_closeout_replay_consistency_decision: &'static str,
    pub(crate) enablement_operator_review_packet_events: usize,
    pub(crate) enablement_operator_review_replay_consistency_events: usize,
    pub(crate) no_live_rehearsal_closeout_events: usize,
    pub(crate) no_live_rehearsal_closeout_replay_consistency_events: usize,
    pub(crate) prior_enablement_audit_chain_closeout_events: usize,
    pub(crate) enablement_operator_review_packet_readback_ready: bool,
    pub(crate) enablement_operator_review_replay_consistency_ready: bool,
    pub(crate) no_live_rehearsal_closeout_readback_ready: bool,
    pub(crate) no_live_rehearsal_closeout_replay_consistency_ready: bool,
    pub(crate) enablement_operator_review_ready: bool,
    pub(crate) enablement_operator_review_replay_consistent: bool,
    pub(crate) no_live_enablement_rehearsal_ready: bool,
    pub(crate) no_live_enablement_rehearsal_closeout_ready: bool,
    pub(crate) no_live_rehearsal_closeout_replay_consistent: bool,
    pub(crate) enablement_audit_chain_closeout_ready: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) enablement_allowed: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) reviewed_flag_enabled: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
    pub(crate) closeout_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) recommended_next_action: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyDecision
{
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) replay_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) enablement_audit_chain_closeout_decision: &'static str,
    pub(crate) enablement_audit_chain_closeout_events: usize,
    pub(crate) prior_enablement_audit_chain_closeout_replay_consistency_events: usize,
    pub(crate) enablement_audit_chain_closeout_readback_ready: bool,
    pub(crate) enablement_audit_chain_closeout_matches_readback: bool,
    pub(crate) enablement_audit_chain_closeout_ready: bool,
    pub(crate) no_live_enablement_rehearsal_ready: bool,
    pub(crate) enablement_allowed: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) reviewed_flag_enabled: bool,
    pub(crate) no_live_guardrails_ready: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) replay_consistent: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) consistency_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacket {
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) packet_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) enablement_audit_chain_closeout_decision: &'static str,
    pub(crate) enablement_audit_chain_closeout_replay_consistency_decision: &'static str,
    pub(crate) enablement_audit_chain_closeout_events: usize,
    pub(crate) enablement_audit_chain_closeout_replay_consistency_events: usize,
    pub(crate) prior_enablement_activation_precondition_operator_packet_events: usize,
    pub(crate) enablement_audit_chain_closeout_readback_ready: bool,
    pub(crate) enablement_audit_chain_closeout_replay_consistency_ready: bool,
    pub(crate) enablement_audit_chain_closeout_ready: bool,
    pub(crate) enablement_audit_chain_closeout_replay_consistent: bool,
    pub(crate) no_live_enablement_rehearsal_ready: bool,
    pub(crate) activation_precondition_ready: bool,
    pub(crate) activation_allowed: bool,
    pub(crate) enablement_allowed: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) reviewed_flag_enabled: bool,
    pub(crate) approval_record_required_before_activation: bool,
    pub(crate) reviewed_flag_required_before_activation: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) no_live_guardrails_ready: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) activation_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) recommended_next_action: &'static str,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyDecision
{
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) replay_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) activation_precondition_decision: &'static str,
    pub(crate) activation_precondition_operator_packet_events: usize,
    pub(crate) prior_activation_precondition_replay_consistency_events: usize,
    pub(crate) activation_precondition_operator_packet_readback_ready: bool,
    pub(crate) activation_precondition_operator_packet_matches_readback: bool,
    pub(crate) activation_precondition_ready: bool,
    pub(crate) activation_allowed: bool,
    pub(crate) enablement_allowed: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) reviewed_flag_enabled: bool,
    pub(crate) approval_record_required_before_activation: bool,
    pub(crate) reviewed_flag_required_before_activation: bool,
    pub(crate) no_live_guardrails_ready: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) replay_consistent: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) consistency_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReceipt {
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) closeout_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) activation_precondition_decision: &'static str,
    pub(crate) activation_precondition_replay_consistency_decision: &'static str,
    pub(crate) activation_precondition_operator_packet_events: usize,
    pub(crate) activation_precondition_replay_consistency_events: usize,
    pub(crate) prior_activation_no_live_closeout_events: usize,
    pub(crate) activation_precondition_operator_packet_readback_ready: bool,
    pub(crate) activation_precondition_replay_consistency_ready: bool,
    pub(crate) activation_precondition_ready: bool,
    pub(crate) activation_precondition_replay_consistent: bool,
    pub(crate) no_live_enablement_rehearsal_ready: bool,
    pub(crate) activation_no_live_closeout_ready: bool,
    pub(crate) activation_allowed: bool,
    pub(crate) enablement_allowed: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) reviewed_flag_enabled: bool,
    pub(crate) approval_record_required_before_activation: bool,
    pub(crate) reviewed_flag_required_before_activation: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
    pub(crate) closeout_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) recommended_next_action: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyDecision
{
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) replay_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) activation_no_live_closeout_decision: &'static str,
    pub(crate) activation_no_live_closeout_events: usize,
    pub(crate) prior_activation_no_live_closeout_replay_consistency_events: usize,
    pub(crate) activation_no_live_closeout_readback_ready: bool,
    pub(crate) activation_no_live_closeout_matches_readback: bool,
    pub(crate) activation_no_live_closeout_ready: bool,
    pub(crate) activation_precondition_ready: bool,
    pub(crate) activation_precondition_replay_consistent: bool,
    pub(crate) no_live_enablement_rehearsal_ready: bool,
    pub(crate) activation_allowed: bool,
    pub(crate) enablement_allowed: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) reviewed_flag_enabled: bool,
    pub(crate) approval_record_required_before_activation: bool,
    pub(crate) reviewed_flag_required_before_activation: bool,
    pub(crate) no_live_guardrails_ready: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) replay_consistent: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) consistency_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReceipt {
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) closeout_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) activation_precondition_decision: &'static str,
    pub(crate) activation_precondition_replay_consistency_decision: &'static str,
    pub(crate) activation_no_live_closeout_decision: &'static str,
    pub(crate) activation_no_live_closeout_replay_consistency_decision: &'static str,
    pub(crate) activation_precondition_operator_packet_events: usize,
    pub(crate) activation_precondition_replay_consistency_events: usize,
    pub(crate) activation_no_live_closeout_events: usize,
    pub(crate) activation_no_live_closeout_replay_consistency_events: usize,
    pub(crate) prior_activation_audit_chain_closeout_events: usize,
    pub(crate) activation_precondition_operator_packet_readback_ready: bool,
    pub(crate) activation_precondition_replay_consistency_ready: bool,
    pub(crate) activation_no_live_closeout_readback_ready: bool,
    pub(crate) activation_no_live_closeout_replay_consistency_ready: bool,
    pub(crate) activation_precondition_ready: bool,
    pub(crate) activation_precondition_replay_consistent: bool,
    pub(crate) activation_no_live_closeout_ready: bool,
    pub(crate) activation_no_live_closeout_replay_consistent: bool,
    pub(crate) no_live_enablement_rehearsal_ready: bool,
    pub(crate) no_live_guardrails_ready: bool,
    pub(crate) activation_audit_chain_closeout_ready: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) activation_allowed: bool,
    pub(crate) enablement_allowed: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) reviewed_flag_enabled: bool,
    pub(crate) approval_record_required_before_activation: bool,
    pub(crate) reviewed_flag_required_before_activation: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
    pub(crate) closeout_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) recommended_next_action: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyDecision
{
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) replay_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) activation_audit_chain_closeout_decision: &'static str,
    pub(crate) activation_audit_chain_closeout_events: usize,
    pub(crate) prior_activation_audit_chain_closeout_replay_consistency_events: usize,
    pub(crate) activation_audit_chain_closeout_readback_ready: bool,
    pub(crate) activation_audit_chain_closeout_matches_readback: bool,
    pub(crate) activation_audit_chain_closeout_ready: bool,
    pub(crate) activation_precondition_ready: bool,
    pub(crate) activation_precondition_replay_consistent: bool,
    pub(crate) activation_no_live_closeout_ready: bool,
    pub(crate) activation_no_live_closeout_replay_consistent: bool,
    pub(crate) no_live_enablement_rehearsal_ready: bool,
    pub(crate) activation_allowed: bool,
    pub(crate) enablement_allowed: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) reviewed_flag_enabled: bool,
    pub(crate) approval_record_required_before_activation: bool,
    pub(crate) reviewed_flag_required_before_activation: bool,
    pub(crate) no_live_guardrails_ready: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) replay_consistent: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) consistency_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacket
{
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) preflight_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) activation_audit_chain_closeout_decision: &'static str,
    pub(crate) activation_audit_chain_closeout_replay_consistency_decision: &'static str,
    pub(crate) activation_audit_chain_closeout_events: usize,
    pub(crate) activation_audit_chain_closeout_replay_consistency_events: usize,
    pub(crate) prior_activation_operator_approval_readiness_preflight_packet_events: usize,
    pub(crate) activation_audit_chain_closeout_readback_ready: bool,
    pub(crate) activation_audit_chain_closeout_replay_consistency_ready: bool,
    pub(crate) activation_audit_chain_closeout_ready: bool,
    pub(crate) activation_audit_chain_closeout_replay_consistent: bool,
    pub(crate) activation_operator_approval_readiness_preflight_ready: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) activation_allowed: bool,
    pub(crate) enablement_allowed: bool,
    pub(crate) operator_approval_required_before_activation: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_required_before_activation: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) reviewed_flag_required_before_activation: bool,
    pub(crate) reviewed_flag_enabled: bool,
    pub(crate) reviewed_flag_mutation_enabled: bool,
    pub(crate) no_live_guardrails_ready: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
    pub(crate) preflight_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) recommended_next_action: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyDecision
{
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) replay_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) activation_operator_approval_readiness_preflight_decision: &'static str,
    pub(crate) activation_operator_approval_readiness_preflight_packet_events: usize,
    pub(crate) prior_activation_operator_approval_readiness_preflight_replay_consistency_events:
        usize,
    pub(crate) activation_operator_approval_readiness_preflight_packet_readback_ready: bool,
    pub(crate) activation_operator_approval_readiness_preflight_packet_matches_readback: bool,
    pub(crate) activation_operator_approval_readiness_preflight_ready: bool,
    pub(crate) activation_audit_chain_closeout_ready: bool,
    pub(crate) activation_audit_chain_closeout_replay_consistent: bool,
    pub(crate) activation_allowed: bool,
    pub(crate) enablement_allowed: bool,
    pub(crate) operator_approval_required_before_activation: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_required_before_activation: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) reviewed_flag_required_before_activation: bool,
    pub(crate) reviewed_flag_enabled: bool,
    pub(crate) reviewed_flag_mutation_enabled: bool,
    pub(crate) no_live_guardrails_ready: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) replay_consistent: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) consistency_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutPacket
{
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) closeout_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) activation_operator_approval_readiness_preflight_decision: &'static str,
    pub(crate) activation_operator_approval_readiness_preflight_replay_consistency_decision:
        &'static str,
    pub(crate) activation_operator_approval_readiness_preflight_packet_events: usize,
    pub(crate) activation_operator_approval_readiness_preflight_replay_consistency_events: usize,
    pub(crate) prior_activation_approval_review_side_effect_lock_closeout_packet_events: usize,
    pub(crate) activation_operator_approval_readiness_preflight_packet_readback_ready: bool,
    pub(crate) activation_operator_approval_readiness_preflight_replay_consistency_ready: bool,
    pub(crate) activation_operator_approval_readiness_preflight_ready: bool,
    pub(crate) activation_operator_approval_readiness_preflight_replay_consistent: bool,
    pub(crate) activation_operator_approval_readiness_preflight_packet_matches_readback: bool,
    pub(crate) approval_review_side_effect_lock_closeout_ready: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) activation_allowed: bool,
    pub(crate) enablement_allowed: bool,
    pub(crate) operator_approval_required_before_activation: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_required_before_activation: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) reviewed_flag_required_before_activation: bool,
    pub(crate) reviewed_flag_enabled: bool,
    pub(crate) reviewed_flag_mutation_enabled: bool,
    pub(crate) approval_review_side_effects_locked: bool,
    pub(crate) no_live_guardrails_ready: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
    pub(crate) closeout_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) recommended_next_action: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutReplayConsistencyDecision
{
    pub(crate) source_surface_id: String,
    pub(crate) decision: &'static str,
    pub(crate) replay_stage: &'static str,
    pub(crate) source_packet_job_id: String,
    pub(crate) activation_approval_review_side_effect_lock_closeout_decision: &'static str,
    pub(crate) activation_approval_review_side_effect_lock_closeout_packet_events: usize,
    pub(crate) prior_activation_approval_review_side_effect_lock_closeout_replay_consistency_events:
        usize,
    pub(crate) activation_approval_review_side_effect_lock_closeout_packet_readback_ready: bool,
    pub(crate) activation_approval_review_side_effect_lock_closeout_packet_matches_readback: bool,
    pub(crate) approval_review_side_effect_lock_closeout_ready: bool,
    pub(crate) activation_operator_approval_readiness_preflight_ready: bool,
    pub(crate) activation_operator_approval_readiness_preflight_replay_consistent: bool,
    pub(crate) approval_review_side_effects_locked: bool,
    pub(crate) activation_allowed: bool,
    pub(crate) enablement_allowed: bool,
    pub(crate) operator_approval_required_before_activation: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_required_before_activation: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) reviewed_flag_required_before_activation: bool,
    pub(crate) reviewed_flag_enabled: bool,
    pub(crate) reviewed_flag_mutation_enabled: bool,
    pub(crate) no_live_guardrails_ready: bool,
    pub(crate) canonical_write_enabled: bool,
    pub(crate) canonical_read_enabled: bool,
    pub(crate) canonical_projection_persistence_enabled: bool,
    pub(crate) replay_consistent: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) consistency_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

pub(crate) struct WorkGraphSurfaceAuditPacketInput<'a> {
    pub(crate) job_id: &'a str,
    pub(crate) promotion_readiness_shadow_matrix: &'a WorkGraphPromotionReadinessShadowMatrix,
    pub(crate) role_manifest_shadow_decisions: &'a [WorkGraphRoleManifestShadowDecision],
    pub(crate) audit_chain_readback: &'a AgentJobWorkGraphAuditChainReadback,
}

pub(crate) struct DirectWaitWorkGraphSurfaceAuditPacketInput<'a> {
    pub(crate) thread_id: String,
    pub(crate) barrier_id: &'a str,
    pub(crate) wait_task_result_readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
    pub(crate) wait_operator_matrix_row: Option<&'a WorkGraphOperatorMatrixRow>,
}

pub(crate) struct WorkGraphCanonicalProjectionReplayConsistencyInput<'a> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) projection_receipt: &'a WorkGraphCanonicalProjectionShadowReceipt,
    pub(crate) projection_receipt_payload: &'a Value,
    pub(crate) latest_projection_receipt_payload: Option<&'a Value>,
    pub(crate) projection_receipt_events: usize,
    pub(crate) projection_receipt_readback_ready: bool,
    pub(crate) prior_projection_replay_consistency_events: usize,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionCloseoutReceiptInput<'a> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) projection_receipt: &'a WorkGraphCanonicalProjectionShadowReceipt,
    pub(crate) replay_consistency_decision:
        &'a WorkGraphCanonicalProjectionReplayConsistencyDecision,
    pub(crate) projection_receipt_events: usize,
    pub(crate) projection_replay_consistency_events: usize,
    pub(crate) prior_projection_closeout_receipt_events: usize,
    pub(crate) projection_receipt_readback_ready: bool,
    pub(crate) projection_replay_consistency_ready: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionCloseoutReplayConsistencyInput<'a> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) closeout_receipt: &'a WorkGraphCanonicalProjectionCloseoutReceipt,
    pub(crate) closeout_receipt_payload: &'a Value,
    pub(crate) latest_closeout_receipt_payload: Option<&'a Value>,
    pub(crate) closeout_receipt_events: usize,
    pub(crate) closeout_receipt_readback_ready: bool,
    pub(crate) prior_closeout_replay_consistency_events: usize,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionAuditChainCloseoutReceiptInput<'a> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) projection_receipt: &'a WorkGraphCanonicalProjectionShadowReceipt,
    pub(crate) projection_replay_consistency_decision:
        &'a WorkGraphCanonicalProjectionReplayConsistencyDecision,
    pub(crate) closeout_receipt: &'a WorkGraphCanonicalProjectionCloseoutReceipt,
    pub(crate) closeout_replay_consistency_decision:
        &'a WorkGraphCanonicalProjectionCloseoutReplayConsistencyDecision,
    pub(crate) projection_receipt_events: usize,
    pub(crate) projection_replay_consistency_events: usize,
    pub(crate) closeout_receipt_events: usize,
    pub(crate) closeout_replay_consistency_events: usize,
    pub(crate) prior_audit_chain_closeout_receipt_events: usize,
    pub(crate) projection_receipt_readback_ready: bool,
    pub(crate) projection_replay_consistency_ready: bool,
    pub(crate) closeout_receipt_readback_ready: bool,
    pub(crate) closeout_replay_consistency_ready: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionAuditChainCloseoutReplayConsistencyInput<'a> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) audit_chain_closeout_receipt:
        &'a WorkGraphCanonicalProjectionAuditChainCloseoutReceipt,
    pub(crate) audit_chain_closeout_receipt_payload: &'a Value,
    pub(crate) latest_audit_chain_closeout_receipt_payload: Option<&'a Value>,
    pub(crate) audit_chain_closeout_receipt_events: usize,
    pub(crate) audit_chain_closeout_receipt_readback_ready: bool,
    pub(crate) prior_audit_chain_closeout_replay_consistency_events: usize,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionEnablementOperatorReviewPacketInput<'a> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) projection_receipt: &'a WorkGraphCanonicalProjectionShadowReceipt,
    pub(crate) projection_replay_consistency_decision:
        &'a WorkGraphCanonicalProjectionReplayConsistencyDecision,
    pub(crate) closeout_receipt: &'a WorkGraphCanonicalProjectionCloseoutReceipt,
    pub(crate) closeout_replay_consistency_decision:
        &'a WorkGraphCanonicalProjectionCloseoutReplayConsistencyDecision,
    pub(crate) audit_chain_closeout_receipt:
        &'a WorkGraphCanonicalProjectionAuditChainCloseoutReceipt,
    pub(crate) audit_chain_closeout_replay_consistency_decision:
        &'a WorkGraphCanonicalProjectionAuditChainCloseoutReplayConsistencyDecision,
    pub(crate) projection_receipt_events: usize,
    pub(crate) projection_replay_consistency_events: usize,
    pub(crate) closeout_receipt_events: usize,
    pub(crate) closeout_replay_consistency_events: usize,
    pub(crate) audit_chain_closeout_receipt_events: usize,
    pub(crate) audit_chain_closeout_replay_consistency_events: usize,
    pub(crate) prior_enablement_operator_review_packet_events: usize,
    pub(crate) projection_receipt_readback_ready: bool,
    pub(crate) projection_replay_consistency_ready: bool,
    pub(crate) closeout_receipt_readback_ready: bool,
    pub(crate) closeout_replay_consistency_ready: bool,
    pub(crate) audit_chain_closeout_receipt_readback_ready: bool,
    pub(crate) audit_chain_closeout_replay_consistency_ready: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyInput<'a> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) enablement_operator_review_packet:
        &'a WorkGraphCanonicalProjectionEnablementOperatorReviewPacket,
    pub(crate) enablement_operator_review_packet_payload: &'a Value,
    pub(crate) latest_enablement_operator_review_packet_payload: Option<&'a Value>,
    pub(crate) enablement_operator_review_packet_events: usize,
    pub(crate) enablement_operator_review_packet_readback_ready: bool,
    pub(crate) prior_enablement_operator_review_replay_consistency_events: usize,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutInput<'a> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) enablement_operator_review_packet:
        &'a WorkGraphCanonicalProjectionEnablementOperatorReviewPacket,
    pub(crate) enablement_operator_review_replay_consistency_decision:
        &'a WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyDecision,
    pub(crate) enablement_operator_review_packet_events: usize,
    pub(crate) enablement_operator_review_replay_consistency_events: usize,
    pub(crate) prior_enablement_no_live_rehearsal_closeout_events: usize,
    pub(crate) enablement_operator_review_packet_readback_ready: bool,
    pub(crate) enablement_operator_review_replay_consistency_ready: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyInput<
    'a,
> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) no_live_rehearsal_closeout_receipt:
        &'a WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReceipt,
    pub(crate) no_live_rehearsal_closeout_receipt_payload: &'a Value,
    pub(crate) latest_no_live_rehearsal_closeout_receipt_payload: Option<&'a Value>,
    pub(crate) no_live_rehearsal_closeout_events: usize,
    pub(crate) no_live_rehearsal_closeout_readback_ready: bool,
    pub(crate) prior_no_live_rehearsal_closeout_replay_consistency_events: usize,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionEnablementAuditChainCloseoutInput<'a> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) enablement_operator_review_packet:
        &'a WorkGraphCanonicalProjectionEnablementOperatorReviewPacket,
    pub(crate) enablement_operator_review_replay_consistency_decision:
        &'a WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyDecision,
    pub(crate) no_live_rehearsal_closeout_receipt:
        &'a WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReceipt,
    pub(crate) no_live_rehearsal_closeout_replay_consistency_decision:
        &'a WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyDecision,
    pub(crate) enablement_operator_review_packet_events: usize,
    pub(crate) enablement_operator_review_replay_consistency_events: usize,
    pub(crate) no_live_rehearsal_closeout_events: usize,
    pub(crate) no_live_rehearsal_closeout_replay_consistency_events: usize,
    pub(crate) prior_enablement_audit_chain_closeout_events: usize,
    pub(crate) enablement_operator_review_packet_readback_ready: bool,
    pub(crate) enablement_operator_review_replay_consistency_ready: bool,
    pub(crate) no_live_rehearsal_closeout_readback_ready: bool,
    pub(crate) no_live_rehearsal_closeout_replay_consistency_ready: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyInput<'a>
{
    pub(crate) source_surface_id: &'a str,
    pub(crate) enablement_audit_chain_closeout_receipt:
        &'a WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReceipt,
    pub(crate) enablement_audit_chain_closeout_receipt_payload: &'a Value,
    pub(crate) latest_enablement_audit_chain_closeout_receipt_payload: Option<&'a Value>,
    pub(crate) enablement_audit_chain_closeout_events: usize,
    pub(crate) enablement_audit_chain_closeout_readback_ready: bool,
    pub(crate) prior_enablement_audit_chain_closeout_replay_consistency_events: usize,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacketInput<
    'a,
> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) enablement_audit_chain_closeout_receipt:
        &'a WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReceipt,
    pub(crate) enablement_audit_chain_closeout_replay_consistency_decision:
        &'a WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyDecision,
    pub(crate) enablement_audit_chain_closeout_events: usize,
    pub(crate) enablement_audit_chain_closeout_replay_consistency_events: usize,
    pub(crate) prior_enablement_activation_precondition_operator_packet_events: usize,
    pub(crate) enablement_audit_chain_closeout_readback_ready: bool,
    pub(crate) enablement_audit_chain_closeout_replay_consistency_ready: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyInput<
    'a,
> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) activation_precondition_operator_packet:
        &'a WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacket,
    pub(crate) activation_precondition_operator_packet_payload: &'a Value,
    pub(crate) latest_activation_precondition_operator_packet_payload: Option<&'a Value>,
    pub(crate) activation_precondition_operator_packet_events: usize,
    pub(crate) activation_precondition_operator_packet_readback_ready: bool,
    pub(crate) prior_activation_precondition_replay_consistency_events: usize,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutInput<'a> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) activation_precondition_operator_packet:
        &'a WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacket,
    pub(crate) activation_precondition_replay_consistency_decision:
        &'a WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyDecision,
    pub(crate) activation_precondition_operator_packet_events: usize,
    pub(crate) activation_precondition_replay_consistency_events: usize,
    pub(crate) prior_activation_no_live_closeout_events: usize,
    pub(crate) activation_precondition_operator_packet_readback_ready: bool,
    pub(crate) activation_precondition_replay_consistency_ready: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyInput<
    'a,
> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) activation_no_live_closeout_receipt:
        &'a WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReceipt,
    pub(crate) activation_no_live_closeout_receipt_payload: &'a Value,
    pub(crate) latest_activation_no_live_closeout_receipt_payload: Option<&'a Value>,
    pub(crate) activation_no_live_closeout_events: usize,
    pub(crate) activation_no_live_closeout_readback_ready: bool,
    pub(crate) prior_activation_no_live_closeout_replay_consistency_events: usize,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutInput<'a> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) activation_precondition_operator_packet:
        &'a WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacket,
    pub(crate) activation_precondition_replay_consistency_decision:
        &'a WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyDecision,
    pub(crate) activation_no_live_closeout_receipt:
        &'a WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReceipt,
    pub(crate) activation_no_live_closeout_replay_consistency_decision:
        &'a WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyDecision,
    pub(crate) activation_precondition_operator_packet_events: usize,
    pub(crate) activation_precondition_replay_consistency_events: usize,
    pub(crate) activation_no_live_closeout_events: usize,
    pub(crate) activation_no_live_closeout_replay_consistency_events: usize,
    pub(crate) prior_activation_audit_chain_closeout_events: usize,
    pub(crate) activation_precondition_operator_packet_readback_ready: bool,
    pub(crate) activation_precondition_replay_consistency_ready: bool,
    pub(crate) activation_no_live_closeout_readback_ready: bool,
    pub(crate) activation_no_live_closeout_replay_consistency_ready: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyInput<
    'a,
> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) activation_audit_chain_closeout_receipt:
        &'a WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReceipt,
    pub(crate) activation_audit_chain_closeout_receipt_payload: &'a Value,
    pub(crate) latest_activation_audit_chain_closeout_receipt_payload: Option<&'a Value>,
    pub(crate) activation_audit_chain_closeout_events: usize,
    pub(crate) activation_audit_chain_closeout_readback_ready: bool,
    pub(crate) prior_activation_audit_chain_closeout_replay_consistency_events: usize,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacketInput<
    'a,
> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) activation_audit_chain_closeout_receipt:
        &'a WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReceipt,
    pub(crate) activation_audit_chain_closeout_replay_consistency_decision:
        &'a WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyDecision,
    pub(crate) activation_audit_chain_closeout_events: usize,
    pub(crate) activation_audit_chain_closeout_replay_consistency_events: usize,
    pub(crate) prior_activation_operator_approval_readiness_preflight_packet_events: usize,
    pub(crate) activation_audit_chain_closeout_readback_ready: bool,
    pub(crate) activation_audit_chain_closeout_replay_consistency_ready: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyInput<
    'a,
> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) activation_operator_approval_readiness_preflight_packet:
        &'a WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacket,
    pub(crate) activation_operator_approval_readiness_preflight_packet_payload: &'a Value,
    pub(crate) latest_activation_operator_approval_readiness_preflight_packet_payload:
        Option<&'a Value>,
    pub(crate) activation_operator_approval_readiness_preflight_packet_events: usize,
    pub(crate) activation_operator_approval_readiness_preflight_packet_readback_ready: bool,
    pub(crate) prior_activation_operator_approval_readiness_preflight_replay_consistency_events:
        usize,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutPacketInput<
    'a,
> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) activation_operator_approval_readiness_preflight_packet:
        &'a WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacket,
    pub(crate) activation_operator_approval_readiness_preflight_replay_consistency_decision:
        &'a WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyDecision,
    pub(crate) activation_operator_approval_readiness_preflight_packet_events: usize,
    pub(crate) activation_operator_approval_readiness_preflight_replay_consistency_events:
        usize,
    pub(crate) prior_activation_approval_review_side_effect_lock_closeout_packet_events: usize,
    pub(crate) activation_operator_approval_readiness_preflight_packet_readback_ready: bool,
    pub(crate) activation_operator_approval_readiness_preflight_replay_consistency_ready: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}

pub(crate) struct WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutReplayConsistencyInput<
    'a,
> {
    pub(crate) source_surface_id: &'a str,
    pub(crate) activation_approval_review_side_effect_lock_closeout_packet:
        &'a WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutPacket,
    pub(crate) activation_approval_review_side_effect_lock_closeout_packet_payload: &'a Value,
    pub(crate) latest_activation_approval_review_side_effect_lock_closeout_packet_payload:
        Option<&'a Value>,
    pub(crate) activation_approval_review_side_effect_lock_closeout_packet_events: usize,
    pub(crate) activation_approval_review_side_effect_lock_closeout_packet_readback_ready: bool,
    pub(crate) prior_activation_approval_review_side_effect_lock_closeout_replay_consistency_events:
        usize,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
}
