use super::*;

pub(super) const SURFACE_AUDIT_RECORDED_SHADOW: &str =
    "work_graph_surface_audit_recorded_shadow_no_live_cutover";
pub(super) const SURFACE_AUDIT_BLOCKED_SHADOW: &str =
    "work_graph_surface_audit_blocked_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_RECORDED_SHADOW: &str =
    "work_graph_canonical_projection_recorded_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_BLOCKED_SHADOW: &str =
    "work_graph_canonical_projection_blocked_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_REPLAY_CONSISTENT_SHADOW: &str =
    "work_graph_canonical_projection_replay_consistent_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_REPLAY_MISMATCH_SHADOW: &str =
    "work_graph_canonical_projection_replay_mismatch_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_CLOSEOUT_RECORDED_SHADOW: &str =
    "work_graph_canonical_projection_closeout_recorded_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_CLOSEOUT_BLOCKED_SHADOW: &str =
    "work_graph_canonical_projection_closeout_blocked_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_CLOSEOUT_REPLAY_CONSISTENT_SHADOW: &str =
    "work_graph_canonical_projection_closeout_replay_consistent_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_CLOSEOUT_REPLAY_MISMATCH_SHADOW: &str =
    "work_graph_canonical_projection_closeout_replay_mismatch_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_RECORDED_SHADOW: &str =
    "work_graph_canonical_projection_audit_chain_closeout_recorded_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_BLOCKED_SHADOW: &str =
    "work_graph_canonical_projection_audit_chain_closeout_blocked_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_REPLAY_CONSISTENT_SHADOW: &str =
    "work_graph_canonical_projection_audit_chain_closeout_replay_consistent_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_REPLAY_MISMATCH_SHADOW: &str =
    "work_graph_canonical_projection_audit_chain_closeout_replay_mismatch_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_READY_SHADOW: &str =
    "work_graph_canonical_projection_enablement_operator_review_ready_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_BLOCKED_SHADOW: &str =
    "work_graph_canonical_projection_enablement_operator_review_blocked_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_REPLAY_CONSISTENT_SHADOW: &str = "work_graph_canonical_projection_enablement_operator_review_replay_consistent_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_REPLAY_MISMATCH_SHADOW: &str = "work_graph_canonical_projection_enablement_operator_review_replay_mismatch_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_RECORDED_SHADOW: &str = "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_recorded_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_BLOCKED_SHADOW: &str = "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_blocked_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_REPLAY_CONSISTENT_SHADOW: &str = "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistent_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_REPLAY_MISMATCH_SHADOW: &str = "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_mismatch_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_RECORDED_SHADOW: &str = "work_graph_canonical_projection_enablement_audit_chain_closeout_recorded_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_BLOCKED_SHADOW: &str = "work_graph_canonical_projection_enablement_audit_chain_closeout_blocked_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_REPLAY_CONSISTENT_SHADOW:
    &str = "work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistent_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_REPLAY_MISMATCH_SHADOW: &str = "work_graph_canonical_projection_enablement_audit_chain_closeout_replay_mismatch_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_READY_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_precondition_ready_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_BLOCKED_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_precondition_blocked_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_REPLAY_CONSISTENT_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_precondition_replay_consistent_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_REPLAY_MISMATCH_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_precondition_replay_mismatch_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_RECORDED_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_no_live_closeout_recorded_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_BLOCKED_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_no_live_closeout_blocked_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_REPLAY_CONSISTENT_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistent_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_REPLAY_MISMATCH_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_mismatch_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_RECORDED_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_recorded_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_BLOCKED_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_blocked_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_REPLAY_CONSISTENT_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistent_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_REPLAY_MISMATCH_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_mismatch_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_READY_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_BLOCKED_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_blocked_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_REPLAY_CONSISTENT_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistent_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_REPLAY_MISMATCH_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_mismatch_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_RECORDED_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_recorded_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_BLOCKED_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_blocked_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_REPLAY_CONSISTENT_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistent_shadow_no_live_cutover";
pub(super) const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_REPLAY_MISMATCH_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_mismatch_shadow_no_live_cutover";
pub(super) const WORK_GRAPH_SURFACE_AUDIT_CHAIN_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "admission_shadow_decision",
        event_type: "agent_job_admission_shadow_decision",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "promotion_readiness_matrix",
        event_type: "agent_job_promotion_readiness_matrix",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "operator_review_promotion_packet",
        event_type: "agent_job_operator_review_promotion_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "promotion_review_replay_consistency",
        event_type: "agent_job_promotion_review_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "promotion_closeout_receipt",
        event_type: "agent_job_promotion_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "promotion_closeout_replay_consistency",
        event_type: "agent_job_promotion_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "promotion_review_audit_chain_receipt",
        event_type: "agent_job_promotion_review_audit_chain_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "reviewed_flag_precondition_plan",
        event_type: "agent_job_reviewed_flag_precondition_plan",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "reviewed_flag_precondition_plan_replay_consistency",
        event_type: "agent_job_reviewed_flag_precondition_plan_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "reviewed_flag_readiness_closeout_receipt",
        event_type: "agent_job_reviewed_flag_readiness_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "reviewed_flag_readiness_closeout_replay_consistency",
        event_type: "agent_job_reviewed_flag_readiness_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "reviewed_flag_audit_chain_closeout_receipt",
        event_type: "agent_job_reviewed_flag_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_REPLAY_CHAIN_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_CLOSEOUT_CHAIN_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_CLOSEOUT_REPLAY_CHAIN_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_REPLAY_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_packet",
        replay_consistency_field: None,
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_REPLAY_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        replay_consistency_field: None,
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_REPLAY_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_REPLAY_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_operator_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_operator_packet",
        replay_consistency_field: None,
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_REPLAY_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_operator_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_operator_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_operator_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_operator_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_no_live_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt",
        replay_consistency_field: None,
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_REPLAY_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_operator_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_operator_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_no_live_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_no_live_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_operator_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_operator_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_no_live_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_no_live_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_REPLAY_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_operator_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_operator_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_no_live_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_no_live_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_operator_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_operator_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_no_live_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_no_live_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet",
        replay_consistency_field: None,
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_REPLAY_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_operator_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_operator_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_no_live_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_no_live_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_operator_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_operator_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_no_live_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_no_live_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet",
        replay_consistency_field: None,
    },
];
pub(super) const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_REPLAY_SEGMENTS:
    &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_receipt",
        event_type: "agent_job_work_graph_canonical_projection_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_operator_review_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_operator_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_operator_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_precondition_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_no_live_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_no_live_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_audit_chain_closeout_receipt",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet",
        replay_consistency_field: None,
    },
    AgentJobWorkGraphAuditChainSegmentSpec {
        segment_id: "canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency",
        event_type: "agent_job_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency",
        replay_consistency_field: Some("replayConsistent"),
    },
];
