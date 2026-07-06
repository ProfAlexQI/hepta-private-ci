use crate::tools::handlers::work_graph_admission::WorkGraphAdmissionShadowCheck;
use crate::tools::handlers::work_graph_admission::WorkGraphRoleManifestShadowDecision;
use crate::tools::handlers::work_graph_admission::build_default_task_result_contract_shadow_plan;
use crate::tools::handlers::work_graph_admission::default_agent_card_manifest_registry;
use crate::tools::handlers::work_graph_promotion_readiness::WorkGraphPromotionReadinessShadowMatrix;
use crate::tools::handlers::work_graph_promotion_readiness::build_default_governed_promotion_readiness_shadow_matrix;
use codex_state::AgentJobWorkGraphAuditChainReadback;
use codex_state::AgentJobWorkGraphAuditChainSegmentSpec;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeSet;

const SURFACE_AUDIT_RECORDED_SHADOW: &str =
    "work_graph_surface_audit_recorded_shadow_no_live_cutover";
const SURFACE_AUDIT_BLOCKED_SHADOW: &str =
    "work_graph_surface_audit_blocked_shadow_no_live_cutover";
const CANONICAL_PROJECTION_RECORDED_SHADOW: &str =
    "work_graph_canonical_projection_recorded_shadow_no_live_cutover";
const CANONICAL_PROJECTION_BLOCKED_SHADOW: &str =
    "work_graph_canonical_projection_blocked_shadow_no_live_cutover";
const CANONICAL_PROJECTION_REPLAY_CONSISTENT_SHADOW: &str =
    "work_graph_canonical_projection_replay_consistent_shadow_no_live_cutover";
const CANONICAL_PROJECTION_REPLAY_MISMATCH_SHADOW: &str =
    "work_graph_canonical_projection_replay_mismatch_shadow_no_live_cutover";
const CANONICAL_PROJECTION_CLOSEOUT_RECORDED_SHADOW: &str =
    "work_graph_canonical_projection_closeout_recorded_shadow_no_live_cutover";
const CANONICAL_PROJECTION_CLOSEOUT_BLOCKED_SHADOW: &str =
    "work_graph_canonical_projection_closeout_blocked_shadow_no_live_cutover";
const CANONICAL_PROJECTION_CLOSEOUT_REPLAY_CONSISTENT_SHADOW: &str =
    "work_graph_canonical_projection_closeout_replay_consistent_shadow_no_live_cutover";
const CANONICAL_PROJECTION_CLOSEOUT_REPLAY_MISMATCH_SHADOW: &str =
    "work_graph_canonical_projection_closeout_replay_mismatch_shadow_no_live_cutover";
const CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_RECORDED_SHADOW: &str =
    "work_graph_canonical_projection_audit_chain_closeout_recorded_shadow_no_live_cutover";
const CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_BLOCKED_SHADOW: &str =
    "work_graph_canonical_projection_audit_chain_closeout_blocked_shadow_no_live_cutover";
const CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_REPLAY_CONSISTENT_SHADOW: &str =
    "work_graph_canonical_projection_audit_chain_closeout_replay_consistent_shadow_no_live_cutover";
const CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_REPLAY_MISMATCH_SHADOW: &str =
    "work_graph_canonical_projection_audit_chain_closeout_replay_mismatch_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_READY_SHADOW: &str =
    "work_graph_canonical_projection_enablement_operator_review_ready_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_BLOCKED_SHADOW: &str =
    "work_graph_canonical_projection_enablement_operator_review_blocked_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_REPLAY_CONSISTENT_SHADOW: &str = "work_graph_canonical_projection_enablement_operator_review_replay_consistent_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_REPLAY_MISMATCH_SHADOW: &str = "work_graph_canonical_projection_enablement_operator_review_replay_mismatch_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_RECORDED_SHADOW: &str = "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_recorded_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_BLOCKED_SHADOW: &str = "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_blocked_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_REPLAY_CONSISTENT_SHADOW: &str = "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistent_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_REPLAY_MISMATCH_SHADOW: &str = "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_mismatch_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_RECORDED_SHADOW: &str = "work_graph_canonical_projection_enablement_audit_chain_closeout_recorded_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_BLOCKED_SHADOW: &str = "work_graph_canonical_projection_enablement_audit_chain_closeout_blocked_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_REPLAY_CONSISTENT_SHADOW: &str = "work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistent_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_REPLAY_MISMATCH_SHADOW: &str = "work_graph_canonical_projection_enablement_audit_chain_closeout_replay_mismatch_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_READY_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_precondition_ready_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_BLOCKED_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_precondition_blocked_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_REPLAY_CONSISTENT_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_precondition_replay_consistent_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_REPLAY_MISMATCH_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_precondition_replay_mismatch_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_RECORDED_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_no_live_closeout_recorded_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_BLOCKED_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_no_live_closeout_blocked_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_REPLAY_CONSISTENT_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistent_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_REPLAY_MISMATCH_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_mismatch_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_RECORDED_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_recorded_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_BLOCKED_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_blocked_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_REPLAY_CONSISTENT_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistent_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_REPLAY_MISMATCH_SHADOW: &str = "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_mismatch_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_READY_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_BLOCKED_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_blocked_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_REPLAY_CONSISTENT_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistent_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_REPLAY_MISMATCH_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_mismatch_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_RECORDED_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_recorded_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_BLOCKED_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_blocked_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_REPLAY_CONSISTENT_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistent_shadow_no_live_cutover";
const CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_REPLAY_MISMATCH_SHADOW:
    &str = "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_mismatch_shadow_no_live_cutover";
const WORK_GRAPH_SURFACE_AUDIT_CHAIN_SEGMENTS: &[AgentJobWorkGraphAuditChainSegmentSpec] = &[
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
const WORK_GRAPH_CANONICAL_PROJECTION_REPLAY_CHAIN_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_CLOSEOUT_CHAIN_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_CLOSEOUT_REPLAY_CHAIN_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_REPLAY_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_REPLAY_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_REPLAY_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_REPLAY_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_REPLAY_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_REPLAY_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_REPLAY_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_REPLAY_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_SEGMENTS:
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
const WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_REPLAY_SEGMENTS:
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

struct WorkGraphSurfaceAuditPacketPartsInput {
    job_id: String,
    surface_entries: Vec<WorkGraphSurfaceAuditEntry>,
    audit_chain: WorkGraphAuditChainSummary,
    no_live_guardrails_ready: bool,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn bool_is_false(value: &bool) -> bool {
    !*value
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn bool_is_true(value: &bool) -> bool {
    *value
}

pub(crate) fn work_graph_surface_audit_chain_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_SURFACE_AUDIT_CHAIN_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_replay_chain_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_REPLAY_CHAIN_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_closeout_chain_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_CLOSEOUT_CHAIN_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_closeout_replay_chain_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_CLOSEOUT_REPLAY_CHAIN_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_audit_chain_closeout_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_audit_chain_closeout_replay_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_REPLAY_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_enablement_operator_review_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_enablement_operator_review_replay_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_REPLAY_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_REPLAY_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_enablement_audit_chain_closeout_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_enablement_audit_chain_closeout_replay_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_REPLAY_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_enablement_activation_precondition_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_enablement_activation_precondition_replay_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_REPLAY_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_enablement_activation_no_live_closeout_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_REPLAY_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_enablement_activation_audit_chain_closeout_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_REPLAY_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_REPLAY_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_SEGMENTS
}

pub(crate) fn work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_segment_specs()
-> &'static [AgentJobWorkGraphAuditChainSegmentSpec] {
    WORK_GRAPH_CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_REPLAY_SEGMENTS
}

pub(crate) fn build_work_graph_surface_audit_packet(
    input: WorkGraphSurfaceAuditPacketInput<'_>,
) -> WorkGraphSurfaceAuditPacket {
    let surface_entries = build_surface_entries(
        input.promotion_readiness_shadow_matrix,
        input.role_manifest_shadow_decisions,
    );
    let audit_chain = build_audit_chain_summary(input.audit_chain_readback);
    let no_live_guardrails_ready = input.audit_chain_readback.live_blocking_event_count == 0
        && input.audit_chain_readback.live_cutover_event_count == 0
        && !input.promotion_readiness_shadow_matrix.feature_flag_enabled
        && input.promotion_readiness_shadow_matrix.canary_stage == "off"
        && input.promotion_readiness_shadow_matrix.canary_traffic_ppm == 0
        && !input
            .promotion_readiness_shadow_matrix
            .live_blocking_enabled
        && !input.promotion_readiness_shadow_matrix.live_cutover_enabled;

    build_work_graph_surface_audit_packet_from_parts(WorkGraphSurfaceAuditPacketPartsInput {
        job_id: input.job_id.to_string(),
        surface_entries,
        audit_chain,
        no_live_guardrails_ready,
    })
}

pub(crate) fn build_direct_wait_work_graph_surface_audit_packet(
    input: DirectWaitWorkGraphSurfaceAuditPacketInput<'_>,
) -> WorkGraphSurfaceAuditPacket {
    let matrix = build_default_governed_promotion_readiness_shadow_matrix(&[]);
    let mut surface_entries = build_surface_entries(&matrix, &[]);
    let direct_wait_entry = build_direct_wait_surface_audit_entry(
        input.wait_task_result_readback,
        input.wait_operator_matrix_row,
    );
    if let Some(entry) = surface_entries
        .iter_mut()
        .find(|entry| entry.source_surface_id == "wait_agent")
    {
        *entry = direct_wait_entry;
    } else {
        surface_entries.push(direct_wait_entry);
    }
    let audit_chain = build_direct_wait_global_audit_chain_summary(input.wait_task_result_readback);
    let no_live_guardrails_ready = audit_chain.no_live_guardrails_ready;
    build_work_graph_surface_audit_packet_from_parts(WorkGraphSurfaceAuditPacketPartsInput {
        job_id: format!("direct-wait:{}:{}", input.thread_id, input.barrier_id),
        surface_entries,
        audit_chain,
        no_live_guardrails_ready,
    })
}

pub(crate) fn build_work_graph_canonical_projection_replay_consistency_decision(
    input: WorkGraphCanonicalProjectionReplayConsistencyInput<'_>,
) -> WorkGraphCanonicalProjectionReplayConsistencyDecision {
    let projection_receipt_matches_readback =
        input.latest_projection_receipt_payload == Some(input.projection_receipt_payload);
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input.projection_receipt.canonical_write_enabled
        && !input.projection_receipt.canonical_read_enabled
        && !input
            .projection_receipt
            .canonical_projection_persistence_enabled
        && !input.projection_receipt.feature_flag_enabled
        && input.projection_receipt.canary_stage == "off"
        && !input.projection_receipt.live_blocking_enabled
        && !input.projection_receipt.live_cutover_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_receipt_readback_ready",
            passed: input.projection_receipt_readback_ready,
            detail: "durable readback must include the canonical projection receipt payload"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_receipt_latest_payload_matches",
            passed: projection_receipt_matches_readback,
            detail: "latest durable canonical projection receipt must match the tool result payload"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_receipt_ready",
            passed: input.projection_receipt.projection_receipt_ready,
            detail: "canonical projection receipt must be ready as shadow-only read projection evidence"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_disabled",
            passed: !input.projection_receipt.write_projection_ready
                && !input.projection_receipt.canonical_write_enabled
                && !input.projection_receipt.canonical_read_enabled,
            detail: "canonical projection replay must not enable canonical WorkGraph write or read paths"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_replay_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let consistency_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let replay_consistent = consistency_blockers.is_empty();
    let decision = if replay_consistent {
        CANONICAL_PROJECTION_REPLAY_CONSISTENT_SHADOW
    } else {
        CANONICAL_PROJECTION_REPLAY_MISMATCH_SHADOW
    };

    WorkGraphCanonicalProjectionReplayConsistencyDecision {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        replay_stage: "canonical_work_graph_projection_receipt_replay_shadow_only",
        source_packet_job_id: input.projection_receipt.source_packet_job_id.clone(),
        projection_receipt_decision: input.projection_receipt.decision,
        projection_receipt_events: input.projection_receipt_events,
        prior_projection_replay_consistency_events: input
            .prior_projection_replay_consistency_events,
        projection_receipt_readback_ready: input.projection_receipt_readback_ready,
        projection_receipt_matches_readback,
        projection_receipt_ready: input.projection_receipt.projection_receipt_ready,
        read_projection_ready: input.projection_receipt.read_projection_ready,
        write_projection_ready: input.projection_receipt.write_projection_ready,
        no_live_guardrails_ready,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        replay_consistent,
        shadow_readiness_failed: !replay_consistent,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        consistency_blockers,
        checks,
        feature_flag_enabled: false,
        canary_stage: "off",
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_work_graph_canonical_projection_closeout_receipt(
    input: WorkGraphCanonicalProjectionCloseoutReceiptInput<'_>,
) -> WorkGraphCanonicalProjectionCloseoutReceipt {
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input.projection_receipt.canonical_write_enabled
        && !input.projection_receipt.canonical_read_enabled
        && !input
            .projection_receipt
            .canonical_projection_persistence_enabled
        && !input.projection_receipt.feature_flag_enabled
        && input.projection_receipt.canary_stage == "off"
        && !input.projection_receipt.live_blocking_enabled
        && !input.projection_receipt.live_cutover_enabled
        && !input.replay_consistency_decision.feature_flag_enabled
        && input.replay_consistency_decision.canary_stage == "off"
        && !input.replay_consistency_decision.live_blocking_enabled
        && !input.replay_consistency_decision.live_cutover_enabled;
    let projection_write_disabled = !input.projection_receipt.write_projection_ready
        && !input.projection_receipt.canonical_write_enabled
        && !input.projection_receipt.canonical_read_enabled
        && !input
            .projection_receipt
            .canonical_projection_persistence_enabled
        && !input.replay_consistency_decision.canonical_write_enabled
        && !input.replay_consistency_decision.canonical_read_enabled;
    let projection_replay_consistent = input.replay_consistency_decision.replay_consistent
        && !input.replay_consistency_decision.shadow_readiness_failed;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_receipt_readback_ready",
            passed: input.projection_receipt_events > 0 && input.projection_receipt_readback_ready,
            detail: "durable readback must include the canonical projection receipt".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_replay_readback_ready",
            passed: input.projection_replay_consistency_events > 0
                && input.projection_replay_consistency_ready,
            detail:
                "durable readback must include canonical projection replay consistency evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_receipt_ready",
            passed: input.projection_receipt.projection_receipt_ready
                && input.projection_receipt.read_projection_ready,
            detail: "canonical projection receipt must be ready read-projection evidence"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_replay_consistent",
            passed: projection_replay_consistent,
            detail: "canonical projection replay consistency must match durable latest payloads"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_disabled",
            passed: projection_write_disabled,
            detail:
                "terminal closeout must not enable canonical WorkGraph write/read/persistence paths"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_closeout_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let closeout_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let closeout_ready = closeout_blockers.is_empty();
    let decision = if closeout_ready {
        CANONICAL_PROJECTION_CLOSEOUT_RECORDED_SHADOW
    } else {
        CANONICAL_PROJECTION_CLOSEOUT_BLOCKED_SHADOW
    };

    WorkGraphCanonicalProjectionCloseoutReceipt {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        closeout_stage: "terminal_no_cutover_canonical_projection_closeout_shadow_only",
        source_packet_job_id: input.projection_receipt.source_packet_job_id.clone(),
        projection_receipt_decision: input.projection_receipt.decision,
        replay_consistency_decision: input.replay_consistency_decision.decision,
        projection_receipt_events: input.projection_receipt_events,
        projection_replay_consistency_events: input.projection_replay_consistency_events,
        prior_projection_closeout_receipt_events: input.prior_projection_closeout_receipt_events,
        projection_receipt_readback_ready: input.projection_receipt_readback_ready,
        projection_replay_consistency_ready: input.projection_replay_consistency_ready,
        projection_receipt_ready: input.projection_receipt.projection_receipt_ready,
        read_projection_ready: input.projection_receipt.read_projection_ready,
        write_projection_ready: input.projection_receipt.write_projection_ready,
        projection_replay_consistent,
        projected_work_node_count: input.projection_receipt.projected_work_node_count,
        projected_work_edge_count: input.projection_receipt.projected_work_edge_count,
        projected_task_result_count: input.projection_receipt.projected_task_result_count,
        projected_timeline_event_count: input.projection_receipt.projected_timeline_event_count,
        closeout_ready,
        shadow_readiness_failed: !closeout_ready,
        no_cutover_terminal_receipt: closeout_ready,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        closeout_blockers,
        checks,
        feature_flag_enabled: false,
        canary_stage: "off",
        live_blocking_enabled: false,
        live_cutover_enabled: false,
        recommended_next_action: "persist replay/readback consistency for this no-cutover closeout before any canonical WorkGraph read/write activation",
    }
}

pub(crate) fn build_work_graph_canonical_projection_closeout_replay_consistency_decision(
    input: WorkGraphCanonicalProjectionCloseoutReplayConsistencyInput<'_>,
) -> WorkGraphCanonicalProjectionCloseoutReplayConsistencyDecision {
    let closeout_receipt_matches_readback =
        input.latest_closeout_receipt_payload == Some(input.closeout_receipt_payload);
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input.closeout_receipt.canonical_write_enabled
        && !input.closeout_receipt.canonical_read_enabled
        && !input
            .closeout_receipt
            .canonical_projection_persistence_enabled
        && !input.closeout_receipt.feature_flag_enabled
        && input.closeout_receipt.canary_stage == "off"
        && !input.closeout_receipt.live_blocking_enabled
        && !input.closeout_receipt.live_cutover_enabled;
    let projection_write_disabled = !input.closeout_receipt.write_projection_ready
        && !input.closeout_receipt.canonical_write_enabled
        && !input.closeout_receipt.canonical_read_enabled
        && !input
            .closeout_receipt
            .canonical_projection_persistence_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_closeout_receipt_readback_ready",
            passed: input.closeout_receipt_events > 0 && input.closeout_receipt_readback_ready,
            detail: "durable readback must include the canonical projection closeout receipt payload"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_closeout_receipt_latest_payload_matches",
            passed: closeout_receipt_matches_readback,
            detail:
                "latest durable canonical projection closeout receipt must match the tool result payload"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_closeout_receipt_ready",
            passed: input.closeout_receipt.closeout_ready
                && !input.closeout_receipt.shadow_readiness_failed,
            detail: "canonical projection closeout receipt must be ready shadow-only evidence"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_closeout_no_cutover",
            passed: input.closeout_receipt.no_cutover_terminal_receipt,
            detail: "canonical projection closeout must remain a terminal no-cutover receipt"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_disabled",
            passed: projection_write_disabled,
            detail:
                "canonical projection closeout replay must not enable WorkGraph write/read/persistence paths"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_closeout_replay_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let consistency_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let replay_consistent = consistency_blockers.is_empty();
    let decision = if replay_consistent {
        CANONICAL_PROJECTION_CLOSEOUT_REPLAY_CONSISTENT_SHADOW
    } else {
        CANONICAL_PROJECTION_CLOSEOUT_REPLAY_MISMATCH_SHADOW
    };

    WorkGraphCanonicalProjectionCloseoutReplayConsistencyDecision {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        replay_stage: "canonical_work_graph_projection_closeout_replay_shadow_only",
        source_packet_job_id: input.closeout_receipt.source_packet_job_id.clone(),
        closeout_receipt_decision: input.closeout_receipt.decision,
        closeout_receipt_events: input.closeout_receipt_events,
        prior_closeout_replay_consistency_events: input.prior_closeout_replay_consistency_events,
        closeout_receipt_readback_ready: input.closeout_receipt_readback_ready,
        closeout_receipt_matches_readback,
        closeout_receipt_ready: input.closeout_receipt.closeout_ready,
        no_cutover_terminal_receipt: input.closeout_receipt.no_cutover_terminal_receipt,
        no_live_guardrails_ready,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        replay_consistent,
        shadow_readiness_failed: !replay_consistent,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        consistency_blockers,
        checks,
        feature_flag_enabled: false,
        canary_stage: "off",
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_work_graph_canonical_projection_audit_chain_closeout_receipt(
    input: WorkGraphCanonicalProjectionAuditChainCloseoutReceiptInput<'_>,
) -> WorkGraphCanonicalProjectionAuditChainCloseoutReceipt {
    let projection_replay_consistent = input
        .projection_replay_consistency_decision
        .replay_consistent
        && !input
            .projection_replay_consistency_decision
            .shadow_readiness_failed;
    let closeout_replay_consistent = input.closeout_replay_consistency_decision.replay_consistent
        && !input
            .closeout_replay_consistency_decision
            .shadow_readiness_failed;
    let no_cutover_terminal_receipt = input.closeout_receipt.no_cutover_terminal_receipt
        && input.closeout_receipt.closeout_ready
        && !input.closeout_receipt.shadow_readiness_failed
        && closeout_replay_consistent;
    let canonical_projection_disabled = !input.projection_receipt.write_projection_ready
        && !input.projection_receipt.canonical_write_enabled
        && !input.projection_receipt.canonical_read_enabled
        && !input
            .projection_receipt
            .canonical_projection_persistence_enabled
        && !input
            .projection_replay_consistency_decision
            .canonical_write_enabled
        && !input
            .projection_replay_consistency_decision
            .canonical_read_enabled
        && !input.closeout_receipt.canonical_write_enabled
        && !input.closeout_receipt.canonical_read_enabled
        && !input
            .closeout_receipt
            .canonical_projection_persistence_enabled
        && !input
            .closeout_replay_consistency_decision
            .canonical_write_enabled
        && !input
            .closeout_replay_consistency_decision
            .canonical_read_enabled;
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input.projection_receipt.feature_flag_enabled
        && input.projection_receipt.canary_stage == "off"
        && !input.projection_receipt.live_blocking_enabled
        && !input.projection_receipt.live_cutover_enabled
        && !input
            .projection_replay_consistency_decision
            .feature_flag_enabled
        && input.projection_replay_consistency_decision.canary_stage == "off"
        && !input
            .projection_replay_consistency_decision
            .live_blocking_enabled
        && !input
            .projection_replay_consistency_decision
            .live_cutover_enabled
        && !input.closeout_receipt.feature_flag_enabled
        && input.closeout_receipt.canary_stage == "off"
        && !input.closeout_receipt.live_blocking_enabled
        && !input.closeout_receipt.live_cutover_enabled
        && !input
            .closeout_replay_consistency_decision
            .feature_flag_enabled
        && input.closeout_replay_consistency_decision.canary_stage == "off"
        && !input
            .closeout_replay_consistency_decision
            .live_blocking_enabled
        && !input
            .closeout_replay_consistency_decision
            .live_cutover_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_receipt_readback_ready",
            passed: input.projection_receipt_events > 0
                && input.projection_receipt_readback_ready,
            detail: "durable readback must include canonical projection receipt evidence"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_replay_readback_ready",
            passed: input.projection_replay_consistency_events > 0
                && input.projection_replay_consistency_ready,
            detail:
                "durable readback must include canonical projection replay consistency evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_closeout_receipt_readback_ready",
            passed: input.closeout_receipt_events > 0 && input.closeout_receipt_readback_ready,
            detail: "durable readback must include canonical projection closeout receipt evidence"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_closeout_replay_readback_ready",
            passed: input.closeout_replay_consistency_events > 0
                && input.closeout_replay_consistency_ready,
            detail:
                "durable readback must include canonical projection closeout replay consistency evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_receipt_ready",
            passed: input.projection_receipt.projection_receipt_ready
                && input.projection_receipt.read_projection_ready,
            detail: "canonical projection receipt must be ready read-projection evidence"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_replay_consistent",
            passed: projection_replay_consistent,
            detail: "canonical projection replay consistency must be ready and matched"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_closeout_receipt_ready",
            passed: input.closeout_receipt.closeout_ready
                && !input.closeout_receipt.shadow_readiness_failed,
            detail: "canonical projection closeout receipt must be ready".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_closeout_replay_consistent",
            passed: closeout_replay_consistent,
            detail: "canonical projection closeout replay consistency must be ready and matched"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_terminal_no_cutover",
            passed: no_cutover_terminal_receipt,
            detail: "canonical projection audit chain must end in a terminal no-cutover receipt"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical WorkGraph write, read, and projection persistence must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_audit_chain_closeout_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let audit_chain_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let audit_chain_closeout_ready = audit_chain_blockers.is_empty();
    let decision = if audit_chain_closeout_ready {
        CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_RECORDED_SHADOW
    } else {
        CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_BLOCKED_SHADOW
    };

    WorkGraphCanonicalProjectionAuditChainCloseoutReceipt {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        audit_stage: "terminal_canonical_projection_audit_chain_closeout_shadow_only",
        source_packet_job_id: input.projection_receipt.source_packet_job_id.clone(),
        projection_receipt_decision: input.projection_receipt.decision,
        projection_replay_consistency_decision: input
            .projection_replay_consistency_decision
            .decision,
        closeout_receipt_decision: input.closeout_receipt.decision,
        closeout_replay_consistency_decision: input.closeout_replay_consistency_decision.decision,
        projection_receipt_events: input.projection_receipt_events,
        projection_replay_consistency_events: input.projection_replay_consistency_events,
        closeout_receipt_events: input.closeout_receipt_events,
        closeout_replay_consistency_events: input.closeout_replay_consistency_events,
        prior_audit_chain_closeout_receipt_events: input.prior_audit_chain_closeout_receipt_events,
        projection_receipt_readback_ready: input.projection_receipt_readback_ready,
        projection_replay_consistency_ready: input.projection_replay_consistency_ready,
        closeout_receipt_readback_ready: input.closeout_receipt_readback_ready,
        closeout_replay_consistency_ready: input.closeout_replay_consistency_ready,
        projection_receipt_ready: input.projection_receipt.projection_receipt_ready,
        projection_replay_consistent,
        closeout_receipt_ready: input.closeout_receipt.closeout_ready,
        closeout_replay_consistent,
        projected_work_node_count: input.projection_receipt.projected_work_node_count,
        projected_work_edge_count: input.projection_receipt.projected_work_edge_count,
        projected_task_result_count: input.projection_receipt.projected_task_result_count,
        projected_timeline_event_count: input.projection_receipt.projected_timeline_event_count,
        no_cutover_terminal_receipt,
        audit_chain_closeout_ready,
        shadow_readiness_failed: !audit_chain_closeout_ready,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        audit_chain_blockers,
        checks,
        feature_flag_enabled: false,
        canary_stage: "off",
        live_blocking_enabled: false,
        live_cutover_enabled: false,
        recommended_next_action: "keep canonical WorkGraph write/read disabled until operator-reviewed audit-chain closeout is replayed from durable evidence",
    }
}

pub(crate) fn build_work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision(
    input: WorkGraphCanonicalProjectionAuditChainCloseoutReplayConsistencyInput<'_>,
) -> WorkGraphCanonicalProjectionAuditChainCloseoutReplayConsistencyDecision {
    let audit_chain_closeout_receipt_matches_readback = input
        .latest_audit_chain_closeout_receipt_payload
        == Some(input.audit_chain_closeout_receipt_payload);
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input.audit_chain_closeout_receipt.feature_flag_enabled
        && input.audit_chain_closeout_receipt.canary_stage == "off"
        && !input.audit_chain_closeout_receipt.live_blocking_enabled
        && !input.audit_chain_closeout_receipt.live_cutover_enabled;
    let canonical_projection_disabled = !input.audit_chain_closeout_receipt.canonical_write_enabled
        && !input.audit_chain_closeout_receipt.canonical_read_enabled
        && !input
            .audit_chain_closeout_receipt
            .canonical_projection_persistence_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_audit_chain_closeout_receipt_readback_ready",
            passed: input.audit_chain_closeout_receipt_events > 0
                && input.audit_chain_closeout_receipt_readback_ready,
            detail:
                "durable readback must include the canonical projection audit-chain closeout receipt payload"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_audit_chain_closeout_latest_payload_matches",
            passed: audit_chain_closeout_receipt_matches_readback,
            detail:
                "latest durable canonical projection audit-chain closeout receipt must match the tool result payload"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_audit_chain_closeout_receipt_ready",
            passed: input
                .audit_chain_closeout_receipt
                .audit_chain_closeout_ready
                && !input.audit_chain_closeout_receipt.shadow_readiness_failed,
            detail:
                "canonical projection audit-chain closeout receipt must be ready shadow-only evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_audit_chain_closeout_no_cutover",
            passed: input
                .audit_chain_closeout_receipt
                .no_cutover_terminal_receipt,
            detail:
                "canonical projection audit-chain closeout must remain terminal no-cutover"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical projection audit-chain closeout replay must not enable WorkGraph write/read/persistence paths"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_audit_chain_closeout_replay_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let consistency_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let replay_consistent = consistency_blockers.is_empty();
    let decision = if replay_consistent {
        CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_REPLAY_CONSISTENT_SHADOW
    } else {
        CANONICAL_PROJECTION_AUDIT_CHAIN_CLOSEOUT_REPLAY_MISMATCH_SHADOW
    };

    WorkGraphCanonicalProjectionAuditChainCloseoutReplayConsistencyDecision {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        replay_stage: "canonical_work_graph_projection_audit_chain_closeout_replay_shadow_only",
        source_packet_job_id: input
            .audit_chain_closeout_receipt
            .source_packet_job_id
            .clone(),
        audit_chain_closeout_receipt_decision: input.audit_chain_closeout_receipt.decision,
        audit_chain_closeout_receipt_events: input.audit_chain_closeout_receipt_events,
        prior_audit_chain_closeout_replay_consistency_events: input
            .prior_audit_chain_closeout_replay_consistency_events,
        audit_chain_closeout_receipt_readback_ready: input
            .audit_chain_closeout_receipt_readback_ready,
        audit_chain_closeout_receipt_matches_readback,
        audit_chain_closeout_receipt_ready: input
            .audit_chain_closeout_receipt
            .audit_chain_closeout_ready,
        no_cutover_terminal_receipt: input
            .audit_chain_closeout_receipt
            .no_cutover_terminal_receipt,
        no_live_guardrails_ready,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        replay_consistent,
        shadow_readiness_failed: !replay_consistent,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        consistency_blockers,
        checks,
        feature_flag_enabled: false,
        canary_stage: "off",
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_work_graph_canonical_projection_enablement_operator_review_packet(
    input: WorkGraphCanonicalProjectionEnablementOperatorReviewPacketInput<'_>,
) -> WorkGraphCanonicalProjectionEnablementOperatorReviewPacket {
    let projection_replay_consistent = input
        .projection_replay_consistency_decision
        .replay_consistent
        && !input
            .projection_replay_consistency_decision
            .shadow_readiness_failed;
    let closeout_replay_consistent = input.closeout_replay_consistency_decision.replay_consistent
        && !input
            .closeout_replay_consistency_decision
            .shadow_readiness_failed;
    let audit_chain_closeout_replay_consistent = input
        .audit_chain_closeout_replay_consistency_decision
        .replay_consistent
        && !input
            .audit_chain_closeout_replay_consistency_decision
            .shadow_readiness_failed;
    let no_cutover_terminal_receipt = input.closeout_receipt.no_cutover_terminal_receipt
        && input
            .audit_chain_closeout_receipt
            .no_cutover_terminal_receipt
        && input
            .audit_chain_closeout_replay_consistency_decision
            .no_cutover_terminal_receipt;
    let canonical_projection_disabled = !input.projection_receipt.write_projection_ready
        && !input.projection_receipt.canonical_write_enabled
        && !input.projection_receipt.canonical_read_enabled
        && !input
            .projection_receipt
            .canonical_projection_persistence_enabled
        && !input
            .projection_replay_consistency_decision
            .canonical_write_enabled
        && !input
            .projection_replay_consistency_decision
            .canonical_read_enabled
        && !input.closeout_receipt.canonical_write_enabled
        && !input.closeout_receipt.canonical_read_enabled
        && !input
            .closeout_receipt
            .canonical_projection_persistence_enabled
        && !input
            .closeout_replay_consistency_decision
            .canonical_write_enabled
        && !input
            .closeout_replay_consistency_decision
            .canonical_read_enabled
        && !input.audit_chain_closeout_receipt.canonical_write_enabled
        && !input.audit_chain_closeout_receipt.canonical_read_enabled
        && !input
            .audit_chain_closeout_receipt
            .canonical_projection_persistence_enabled
        && !input
            .audit_chain_closeout_replay_consistency_decision
            .canonical_write_enabled
        && !input
            .audit_chain_closeout_replay_consistency_decision
            .canonical_read_enabled
        && !input
            .audit_chain_closeout_replay_consistency_decision
            .canonical_projection_persistence_enabled;
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input.projection_receipt.feature_flag_enabled
        && input.projection_receipt.canary_stage == "off"
        && !input.projection_receipt.live_blocking_enabled
        && !input.projection_receipt.live_cutover_enabled
        && !input
            .projection_replay_consistency_decision
            .feature_flag_enabled
        && input.projection_replay_consistency_decision.canary_stage == "off"
        && !input
            .projection_replay_consistency_decision
            .live_blocking_enabled
        && !input
            .projection_replay_consistency_decision
            .live_cutover_enabled
        && !input.closeout_receipt.feature_flag_enabled
        && input.closeout_receipt.canary_stage == "off"
        && !input.closeout_receipt.live_blocking_enabled
        && !input.closeout_receipt.live_cutover_enabled
        && !input
            .closeout_replay_consistency_decision
            .feature_flag_enabled
        && input.closeout_replay_consistency_decision.canary_stage == "off"
        && !input
            .closeout_replay_consistency_decision
            .live_blocking_enabled
        && !input
            .closeout_replay_consistency_decision
            .live_cutover_enabled
        && !input.audit_chain_closeout_receipt.feature_flag_enabled
        && input.audit_chain_closeout_receipt.canary_stage == "off"
        && !input.audit_chain_closeout_receipt.live_blocking_enabled
        && !input.audit_chain_closeout_receipt.live_cutover_enabled
        && !input
            .audit_chain_closeout_replay_consistency_decision
            .feature_flag_enabled
        && input
            .audit_chain_closeout_replay_consistency_decision
            .canary_stage
            == "off"
        && !input
            .audit_chain_closeout_replay_consistency_decision
            .live_blocking_enabled
        && !input
            .audit_chain_closeout_replay_consistency_decision
            .live_cutover_enabled;
    let no_live_enablement_rehearsal_ready =
        canonical_projection_disabled && no_live_guardrails_ready && no_cutover_terminal_receipt;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_receipt_readback_ready",
            passed: input.projection_receipt_events > 0
                && input.projection_receipt_readback_ready,
            detail: "durable readback must include canonical projection receipt evidence"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_replay_consistency_ready",
            passed: input.projection_replay_consistency_events > 0
                && input.projection_replay_consistency_ready
                && projection_replay_consistent,
            detail: "canonical projection receipt replay must be durable and consistent"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_closeout_receipt_ready",
            passed: input.closeout_receipt_events > 0
                && input.closeout_receipt_readback_ready
                && input.closeout_receipt.closeout_ready
                && !input.closeout_receipt.shadow_readiness_failed,
            detail: "terminal no-cutover canonical projection closeout receipt must be ready"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_closeout_replay_consistency_ready",
            passed: input.closeout_replay_consistency_events > 0
                && input.closeout_replay_consistency_ready
                && closeout_replay_consistent,
            detail: "canonical projection closeout replay must be durable and consistent"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_audit_chain_closeout_receipt_ready",
            passed: input.audit_chain_closeout_receipt_events > 0
                && input.audit_chain_closeout_receipt_readback_ready
                && input
                    .audit_chain_closeout_receipt
                    .audit_chain_closeout_ready
                && !input.audit_chain_closeout_receipt.shadow_readiness_failed,
            detail: "final canonical projection audit-chain closeout receipt must be ready"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_audit_chain_closeout_replay_ready",
            passed: input.audit_chain_closeout_replay_consistency_events > 0
                && input.audit_chain_closeout_replay_consistency_ready
                && audit_chain_closeout_replay_consistent,
            detail: "final canonical projection audit-chain closeout replay must be durable and consistent"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_terminal_no_cutover",
            passed: no_cutover_terminal_receipt,
            detail: "operator review packet must consume a terminal no-cutover closeout chain"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical WorkGraph write, read, and projection persistence must still be disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "operator_review_not_recorded",
            passed: true,
            detail: "this packet prepares review evidence without recording approval".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_enablement_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let enablement_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let enablement_operator_review_ready = enablement_blockers.is_empty();
    let decision = if enablement_operator_review_ready {
        CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_READY_SHADOW
    } else {
        CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_BLOCKED_SHADOW
    };

    WorkGraphCanonicalProjectionEnablementOperatorReviewPacket {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        review_stage: "canonical_projection_enablement_operator_review_shadow_only",
        source_packet_job_id: input.projection_receipt.source_packet_job_id.clone(),
        projection_receipt_decision: input.projection_receipt.decision,
        projection_replay_consistency_decision: input
            .projection_replay_consistency_decision
            .decision,
        closeout_receipt_decision: input.closeout_receipt.decision,
        closeout_replay_consistency_decision: input.closeout_replay_consistency_decision.decision,
        audit_chain_closeout_receipt_decision: input.audit_chain_closeout_receipt.decision,
        audit_chain_closeout_replay_consistency_decision: input
            .audit_chain_closeout_replay_consistency_decision
            .decision,
        projection_receipt_events: input.projection_receipt_events,
        projection_replay_consistency_events: input.projection_replay_consistency_events,
        closeout_receipt_events: input.closeout_receipt_events,
        closeout_replay_consistency_events: input.closeout_replay_consistency_events,
        audit_chain_closeout_receipt_events: input.audit_chain_closeout_receipt_events,
        audit_chain_closeout_replay_consistency_events: input
            .audit_chain_closeout_replay_consistency_events,
        prior_enablement_operator_review_packet_events: input
            .prior_enablement_operator_review_packet_events,
        projection_receipt_readback_ready: input.projection_receipt_readback_ready,
        projection_replay_consistency_ready: input.projection_replay_consistency_ready,
        closeout_receipt_readback_ready: input.closeout_receipt_readback_ready,
        closeout_replay_consistency_ready: input.closeout_replay_consistency_ready,
        audit_chain_closeout_receipt_readback_ready: input
            .audit_chain_closeout_receipt_readback_ready,
        audit_chain_closeout_replay_consistency_ready: input
            .audit_chain_closeout_replay_consistency_ready,
        projection_receipt_ready: input.projection_receipt.projection_receipt_ready,
        projection_replay_consistent,
        closeout_receipt_ready: input.closeout_receipt.closeout_ready,
        closeout_replay_consistent,
        audit_chain_closeout_ready: input
            .audit_chain_closeout_receipt
            .audit_chain_closeout_ready,
        audit_chain_closeout_replay_consistent,
        no_cutover_terminal_receipt,
        no_live_enablement_rehearsal_ready,
        enablement_operator_review_ready,
        shadow_readiness_failed: !enablement_operator_review_ready,
        projected_work_node_count: input.projection_receipt.projected_work_node_count,
        projected_work_edge_count: input.projection_receipt.projected_work_edge_count,
        projected_task_result_count: input.projection_receipt.projected_task_result_count,
        projected_timeline_event_count: input.projection_receipt.projected_timeline_event_count,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        operator_review_required: true,
        operator_approval_recorded: false,
        approval_record_mutation_enabled: false,
        reviewed_flag_enabled: false,
        enablement_allowed: false,
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
        enablement_blockers,
        checks,
        recommended_next_action: "run a separate no-live enablement rehearsal gate before any operator approval recording, reviewed flag, canary, blocking, or canonical WorkGraph cutover",
    }
}

pub(crate) fn build_work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision(
    input: WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyInput<'_>,
) -> WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyDecision {
    let enablement_operator_review_packet_matches_readback = input
        .latest_enablement_operator_review_packet_payload
        == Some(input.enablement_operator_review_packet_payload);
    let packet_keeps_enablement_disabled =
        !input.enablement_operator_review_packet.enablement_allowed
            && !input
                .enablement_operator_review_packet
                .operator_approval_recorded
            && !input
                .enablement_operator_review_packet
                .approval_record_mutation_enabled
            && !input
                .enablement_operator_review_packet
                .reviewed_flag_enabled;
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input.enablement_operator_review_packet.feature_flag_enabled
        && input.enablement_operator_review_packet.canary_stage == "off"
        && !input
            .enablement_operator_review_packet
            .live_blocking_enabled
        && !input.enablement_operator_review_packet.live_cutover_enabled;
    let canonical_projection_disabled = !input
        .enablement_operator_review_packet
        .canonical_write_enabled
        && !input
            .enablement_operator_review_packet
            .canonical_read_enabled
        && !input
            .enablement_operator_review_packet
            .canonical_projection_persistence_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_operator_review_packet_readback_ready",
            passed: input.enablement_operator_review_packet_events > 0
                && input.enablement_operator_review_packet_readback_ready,
            detail:
                "durable readback must include the canonical projection enablement operator-review packet"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_operator_review_latest_payload_matches",
            passed: enablement_operator_review_packet_matches_readback,
            detail:
                "latest durable canonical projection enablement operator-review packet must match the tool result payload"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_operator_review_ready",
            passed: input
                .enablement_operator_review_packet
                .enablement_operator_review_ready
                && !input
                    .enablement_operator_review_packet
                    .shadow_readiness_failed,
            detail:
                "canonical projection enablement operator-review packet must be ready shadow-only evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_no_live_enablement_rehearsal_ready",
            passed: input
                .enablement_operator_review_packet
                .no_live_enablement_rehearsal_ready,
            detail:
                "operator-review packet must remain a no-live enablement rehearsal only"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_still_disabled",
            passed: packet_keeps_enablement_disabled,
            detail:
                "enablementAllowed, approval recording, approval mutation, and reviewed flag must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical WorkGraph write, read, and projection persistence must still be disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_enablement_replay_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let consistency_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let replay_consistent = consistency_blockers.is_empty();
    let decision = if replay_consistent {
        CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_REPLAY_CONSISTENT_SHADOW
    } else {
        CANONICAL_PROJECTION_ENABLEMENT_OPERATOR_REVIEW_REPLAY_MISMATCH_SHADOW
    };

    WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyDecision {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        replay_stage: "canonical_work_graph_projection_enablement_operator_review_replay_shadow_only",
        source_packet_job_id: input
            .enablement_operator_review_packet
            .source_packet_job_id
            .clone(),
        enablement_operator_review_decision: input.enablement_operator_review_packet.decision,
        enablement_operator_review_packet_events: input.enablement_operator_review_packet_events,
        prior_enablement_operator_review_replay_consistency_events: input
            .prior_enablement_operator_review_replay_consistency_events,
        enablement_operator_review_packet_readback_ready: input
            .enablement_operator_review_packet_readback_ready,
        enablement_operator_review_packet_matches_readback,
        enablement_operator_review_ready: input
            .enablement_operator_review_packet
            .enablement_operator_review_ready,
        no_live_enablement_rehearsal_ready: input
            .enablement_operator_review_packet
            .no_live_enablement_rehearsal_ready,
        enablement_allowed: input.enablement_operator_review_packet.enablement_allowed,
        operator_approval_recorded: input
            .enablement_operator_review_packet
            .operator_approval_recorded,
        approval_record_mutation_enabled: input
            .enablement_operator_review_packet
            .approval_record_mutation_enabled,
        reviewed_flag_enabled: input
            .enablement_operator_review_packet
            .reviewed_flag_enabled,
        no_live_guardrails_ready,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        replay_consistent,
        shadow_readiness_failed: !replay_consistent,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        consistency_blockers,
        checks,
        feature_flag_enabled: false,
        canary_stage: "off",
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt(
    input: WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutInput<'_>,
) -> WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReceipt {
    let enablement_operator_review_replay_consistent = input
        .enablement_operator_review_replay_consistency_decision
        .replay_consistent
        && !input
            .enablement_operator_review_replay_consistency_decision
            .shadow_readiness_failed;
    let no_live_enablement_rehearsal_ready = input
        .enablement_operator_review_packet
        .no_live_enablement_rehearsal_ready
        && input
            .enablement_operator_review_replay_consistency_decision
            .no_live_enablement_rehearsal_ready;
    let enablement_still_disabled = !input.enablement_operator_review_packet.enablement_allowed
        && !input
            .enablement_operator_review_packet
            .operator_approval_recorded
        && !input
            .enablement_operator_review_packet
            .approval_record_mutation_enabled
        && !input
            .enablement_operator_review_packet
            .reviewed_flag_enabled
        && !input
            .enablement_operator_review_replay_consistency_decision
            .enablement_allowed
        && !input
            .enablement_operator_review_replay_consistency_decision
            .operator_approval_recorded
        && !input
            .enablement_operator_review_replay_consistency_decision
            .approval_record_mutation_enabled
        && !input
            .enablement_operator_review_replay_consistency_decision
            .reviewed_flag_enabled;
    let canonical_projection_disabled = !input
        .enablement_operator_review_packet
        .canonical_write_enabled
        && !input
            .enablement_operator_review_packet
            .canonical_read_enabled
        && !input
            .enablement_operator_review_packet
            .canonical_projection_persistence_enabled
        && !input
            .enablement_operator_review_replay_consistency_decision
            .canonical_write_enabled
        && !input
            .enablement_operator_review_replay_consistency_decision
            .canonical_read_enabled
        && !input
            .enablement_operator_review_replay_consistency_decision
            .canonical_projection_persistence_enabled;
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input.enablement_operator_review_packet.feature_flag_enabled
        && input.enablement_operator_review_packet.canary_stage == "off"
        && input.enablement_operator_review_packet.canary_traffic_ppm == 0
        && !input
            .enablement_operator_review_packet
            .live_blocking_enabled
        && !input.enablement_operator_review_packet.live_cutover_enabled
        && !input
            .enablement_operator_review_replay_consistency_decision
            .feature_flag_enabled
        && input
            .enablement_operator_review_replay_consistency_decision
            .canary_stage
            == "off"
        && !input
            .enablement_operator_review_replay_consistency_decision
            .live_blocking_enabled
        && !input
            .enablement_operator_review_replay_consistency_decision
            .live_cutover_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_operator_review_packet_readback_ready",
            passed: input.enablement_operator_review_packet_events > 0
                && input.enablement_operator_review_packet_readback_ready,
            detail:
                "durable readback must include canonical projection enablement operator-review packet evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_operator_review_replay_ready",
            passed: input.enablement_operator_review_replay_consistency_events > 0
                && input.enablement_operator_review_replay_consistency_ready
                && enablement_operator_review_replay_consistent,
            detail:
                "canonical projection enablement operator-review replay must be durable and consistent"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_operator_review_ready",
            passed: input
                .enablement_operator_review_packet
                .enablement_operator_review_ready
                && !input
                    .enablement_operator_review_packet
                    .shadow_readiness_failed,
            detail:
                "canonical projection enablement operator-review packet must be ready shadow-only evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_no_live_enablement_rehearsal_ready",
            passed: no_live_enablement_rehearsal_ready,
            detail:
                "enablement rehearsal must remain no-live and derived from replay-consistent review evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_still_disabled",
            passed: enablement_still_disabled,
            detail:
                "enablementAllowed, approval recording, approval mutation, and reviewed flag must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical WorkGraph write, read, and projection persistence must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_enablement_rehearsal_closeout_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let closeout_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let no_live_enablement_rehearsal_closeout_ready = closeout_blockers.is_empty();
    let decision = if no_live_enablement_rehearsal_closeout_ready {
        CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_RECORDED_SHADOW
    } else {
        CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_BLOCKED_SHADOW
    };

    WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReceipt {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        closeout_stage: "canonical_projection_enablement_no_live_rehearsal_closeout_shadow_only",
        source_packet_job_id: input
            .enablement_operator_review_packet
            .source_packet_job_id
            .clone(),
        enablement_operator_review_decision: input.enablement_operator_review_packet.decision,
        enablement_operator_review_replay_consistency_decision: input
            .enablement_operator_review_replay_consistency_decision
            .decision,
        enablement_operator_review_packet_events: input.enablement_operator_review_packet_events,
        enablement_operator_review_replay_consistency_events: input
            .enablement_operator_review_replay_consistency_events,
        prior_enablement_no_live_rehearsal_closeout_events: input
            .prior_enablement_no_live_rehearsal_closeout_events,
        enablement_operator_review_packet_readback_ready: input
            .enablement_operator_review_packet_readback_ready,
        enablement_operator_review_replay_consistency_ready: input
            .enablement_operator_review_replay_consistency_ready,
        enablement_operator_review_ready: input
            .enablement_operator_review_packet
            .enablement_operator_review_ready,
        enablement_operator_review_replay_consistent,
        no_live_enablement_rehearsal_ready,
        no_live_enablement_rehearsal_closeout_ready,
        shadow_readiness_failed: !no_live_enablement_rehearsal_closeout_ready,
        enablement_allowed: false,
        operator_approval_recorded: false,
        approval_record_mutation_enabled: false,
        reviewed_flag_enabled: false,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
        closeout_blockers,
        checks,
        recommended_next_action: "persist replay/readback consistency for this no-live enablement rehearsal closeout before any approval recording, reviewed flag, canary, blocking, or cutover",
    }
}

pub(crate) fn build_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision(
    input: WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyInput<'_>,
) -> WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyDecision {
    let no_live_rehearsal_closeout_matches_readback = input
        .latest_no_live_rehearsal_closeout_receipt_payload
        == Some(input.no_live_rehearsal_closeout_receipt_payload);
    let closeout_keeps_enablement_disabled =
        !input.no_live_rehearsal_closeout_receipt.enablement_allowed
            && !input
                .no_live_rehearsal_closeout_receipt
                .operator_approval_recorded
            && !input
                .no_live_rehearsal_closeout_receipt
                .approval_record_mutation_enabled
            && !input
                .no_live_rehearsal_closeout_receipt
                .reviewed_flag_enabled;
    let canonical_projection_disabled = !input
        .no_live_rehearsal_closeout_receipt
        .canonical_write_enabled
        && !input
            .no_live_rehearsal_closeout_receipt
            .canonical_read_enabled
        && !input
            .no_live_rehearsal_closeout_receipt
            .canonical_projection_persistence_enabled;
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input
            .no_live_rehearsal_closeout_receipt
            .feature_flag_enabled
        && input.no_live_rehearsal_closeout_receipt.canary_stage == "off"
        && input.no_live_rehearsal_closeout_receipt.canary_traffic_ppm == 0
        && !input
            .no_live_rehearsal_closeout_receipt
            .live_blocking_enabled
        && !input
            .no_live_rehearsal_closeout_receipt
            .live_cutover_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_no_live_rehearsal_closeout_readback_ready",
            passed: input.no_live_rehearsal_closeout_events > 0
                && input.no_live_rehearsal_closeout_readback_ready,
            detail:
                "durable readback must include the canonical projection enablement no-live rehearsal closeout receipt"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_no_live_rehearsal_closeout_latest_payload_matches",
            passed: no_live_rehearsal_closeout_matches_readback,
            detail:
                "latest durable canonical projection enablement no-live rehearsal closeout receipt must match the tool result payload"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_no_live_rehearsal_closeout_ready",
            passed: input
                .no_live_rehearsal_closeout_receipt
                .no_live_enablement_rehearsal_closeout_ready
                && !input
                    .no_live_rehearsal_closeout_receipt
                    .shadow_readiness_failed,
            detail:
                "canonical projection enablement no-live rehearsal closeout receipt must be ready shadow-only evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_no_live_enablement_rehearsal_ready",
            passed: input
                .no_live_rehearsal_closeout_receipt
                .no_live_enablement_rehearsal_ready,
            detail: "enablement rehearsal closeout must remain no-live".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_still_disabled",
            passed: closeout_keeps_enablement_disabled,
            detail:
                "enablementAllowed, approval recording, approval mutation, and reviewed flag must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical WorkGraph write, read, and projection persistence must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_enablement_no_live_rehearsal_closeout_replay_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let consistency_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let replay_consistent = consistency_blockers.is_empty();
    let decision = if replay_consistent {
        CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_REPLAY_CONSISTENT_SHADOW
    } else {
        CANONICAL_PROJECTION_ENABLEMENT_NO_LIVE_REHEARSAL_CLOSEOUT_REPLAY_MISMATCH_SHADOW
    };

    WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyDecision {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        replay_stage: "canonical_projection_enablement_no_live_rehearsal_closeout_replay_shadow_only",
        source_packet_job_id: input
            .no_live_rehearsal_closeout_receipt
            .source_packet_job_id
            .clone(),
        no_live_rehearsal_closeout_decision: input.no_live_rehearsal_closeout_receipt.decision,
        no_live_rehearsal_closeout_events: input.no_live_rehearsal_closeout_events,
        prior_no_live_rehearsal_closeout_replay_consistency_events: input
            .prior_no_live_rehearsal_closeout_replay_consistency_events,
        no_live_rehearsal_closeout_readback_ready: input.no_live_rehearsal_closeout_readback_ready,
        no_live_rehearsal_closeout_matches_readback,
        no_live_enablement_rehearsal_ready: input
            .no_live_rehearsal_closeout_receipt
            .no_live_enablement_rehearsal_ready,
        no_live_enablement_rehearsal_closeout_ready: input
            .no_live_rehearsal_closeout_receipt
            .no_live_enablement_rehearsal_closeout_ready,
        enablement_allowed: input.no_live_rehearsal_closeout_receipt.enablement_allowed,
        operator_approval_recorded: input
            .no_live_rehearsal_closeout_receipt
            .operator_approval_recorded,
        approval_record_mutation_enabled: input
            .no_live_rehearsal_closeout_receipt
            .approval_record_mutation_enabled,
        reviewed_flag_enabled: input
            .no_live_rehearsal_closeout_receipt
            .reviewed_flag_enabled,
        no_live_guardrails_ready,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        replay_consistent,
        shadow_readiness_failed: !replay_consistent,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        consistency_blockers,
        checks,
        feature_flag_enabled: false,
        canary_stage: "off",
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt(
    input: WorkGraphCanonicalProjectionEnablementAuditChainCloseoutInput<'_>,
) -> WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReceipt {
    let enablement_operator_review_replay_consistent = input
        .enablement_operator_review_replay_consistency_decision
        .replay_consistent
        && !input
            .enablement_operator_review_replay_consistency_decision
            .shadow_readiness_failed;
    let no_live_rehearsal_closeout_replay_consistent = input
        .no_live_rehearsal_closeout_replay_consistency_decision
        .replay_consistent
        && !input
            .no_live_rehearsal_closeout_replay_consistency_decision
            .shadow_readiness_failed;
    let no_live_enablement_rehearsal_ready = input
        .enablement_operator_review_packet
        .no_live_enablement_rehearsal_ready
        && input
            .enablement_operator_review_replay_consistency_decision
            .no_live_enablement_rehearsal_ready
        && input
            .no_live_rehearsal_closeout_receipt
            .no_live_enablement_rehearsal_ready
        && input
            .no_live_rehearsal_closeout_replay_consistency_decision
            .no_live_enablement_rehearsal_ready;
    let no_live_enablement_rehearsal_closeout_ready = input
        .no_live_rehearsal_closeout_receipt
        .no_live_enablement_rehearsal_closeout_ready
        && !input
            .no_live_rehearsal_closeout_receipt
            .shadow_readiness_failed
        && input
            .no_live_rehearsal_closeout_replay_consistency_decision
            .no_live_enablement_rehearsal_closeout_ready;
    let enablement_still_disabled = !input.enablement_operator_review_packet.enablement_allowed
        && !input
            .enablement_operator_review_packet
            .operator_approval_recorded
        && !input
            .enablement_operator_review_packet
            .approval_record_mutation_enabled
        && !input
            .enablement_operator_review_packet
            .reviewed_flag_enabled
        && !input
            .enablement_operator_review_replay_consistency_decision
            .enablement_allowed
        && !input
            .enablement_operator_review_replay_consistency_decision
            .operator_approval_recorded
        && !input
            .enablement_operator_review_replay_consistency_decision
            .approval_record_mutation_enabled
        && !input
            .enablement_operator_review_replay_consistency_decision
            .reviewed_flag_enabled
        && !input.no_live_rehearsal_closeout_receipt.enablement_allowed
        && !input
            .no_live_rehearsal_closeout_receipt
            .operator_approval_recorded
        && !input
            .no_live_rehearsal_closeout_receipt
            .approval_record_mutation_enabled
        && !input
            .no_live_rehearsal_closeout_receipt
            .reviewed_flag_enabled
        && !input
            .no_live_rehearsal_closeout_replay_consistency_decision
            .enablement_allowed
        && !input
            .no_live_rehearsal_closeout_replay_consistency_decision
            .operator_approval_recorded
        && !input
            .no_live_rehearsal_closeout_replay_consistency_decision
            .approval_record_mutation_enabled
        && !input
            .no_live_rehearsal_closeout_replay_consistency_decision
            .reviewed_flag_enabled;
    let canonical_projection_disabled = !input
        .enablement_operator_review_packet
        .canonical_write_enabled
        && !input
            .enablement_operator_review_packet
            .canonical_read_enabled
        && !input
            .enablement_operator_review_packet
            .canonical_projection_persistence_enabled
        && !input
            .enablement_operator_review_replay_consistency_decision
            .canonical_write_enabled
        && !input
            .enablement_operator_review_replay_consistency_decision
            .canonical_read_enabled
        && !input
            .enablement_operator_review_replay_consistency_decision
            .canonical_projection_persistence_enabled
        && !input
            .no_live_rehearsal_closeout_receipt
            .canonical_write_enabled
        && !input
            .no_live_rehearsal_closeout_receipt
            .canonical_read_enabled
        && !input
            .no_live_rehearsal_closeout_receipt
            .canonical_projection_persistence_enabled
        && !input
            .no_live_rehearsal_closeout_replay_consistency_decision
            .canonical_write_enabled
        && !input
            .no_live_rehearsal_closeout_replay_consistency_decision
            .canonical_read_enabled
        && !input
            .no_live_rehearsal_closeout_replay_consistency_decision
            .canonical_projection_persistence_enabled;
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input.enablement_operator_review_packet.feature_flag_enabled
        && input.enablement_operator_review_packet.canary_stage == "off"
        && input.enablement_operator_review_packet.canary_traffic_ppm == 0
        && !input
            .enablement_operator_review_packet
            .live_blocking_enabled
        && !input.enablement_operator_review_packet.live_cutover_enabled
        && !input
            .enablement_operator_review_replay_consistency_decision
            .feature_flag_enabled
        && input
            .enablement_operator_review_replay_consistency_decision
            .canary_stage
            == "off"
        && !input
            .enablement_operator_review_replay_consistency_decision
            .live_blocking_enabled
        && !input
            .enablement_operator_review_replay_consistency_decision
            .live_cutover_enabled
        && !input
            .no_live_rehearsal_closeout_receipt
            .feature_flag_enabled
        && input.no_live_rehearsal_closeout_receipt.canary_stage == "off"
        && input.no_live_rehearsal_closeout_receipt.canary_traffic_ppm == 0
        && !input
            .no_live_rehearsal_closeout_receipt
            .live_blocking_enabled
        && !input
            .no_live_rehearsal_closeout_receipt
            .live_cutover_enabled
        && !input
            .no_live_rehearsal_closeout_replay_consistency_decision
            .feature_flag_enabled
        && input
            .no_live_rehearsal_closeout_replay_consistency_decision
            .canary_stage
            == "off"
        && !input
            .no_live_rehearsal_closeout_replay_consistency_decision
            .live_blocking_enabled
        && !input
            .no_live_rehearsal_closeout_replay_consistency_decision
            .live_cutover_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_operator_review_packet_readback_ready",
            passed: input.enablement_operator_review_packet_events > 0
                && input.enablement_operator_review_packet_readback_ready,
            detail:
                "durable readback must include canonical projection enablement operator-review packet evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_operator_review_replay_ready",
            passed: input.enablement_operator_review_replay_consistency_events > 0
                && input.enablement_operator_review_replay_consistency_ready
                && enablement_operator_review_replay_consistent,
            detail:
                "canonical projection enablement operator-review replay must be durable and consistent"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_no_live_rehearsal_closeout_readback_ready",
            passed: input.no_live_rehearsal_closeout_events > 0
                && input.no_live_rehearsal_closeout_readback_ready
                && no_live_enablement_rehearsal_closeout_ready,
            detail:
                "durable readback must include ready no-live rehearsal closeout evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_no_live_rehearsal_closeout_replay_ready",
            passed: input.no_live_rehearsal_closeout_replay_consistency_events > 0
                && input.no_live_rehearsal_closeout_replay_consistency_ready
                && no_live_rehearsal_closeout_replay_consistent,
            detail:
                "canonical projection enablement no-live rehearsal closeout replay must be durable and consistent"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_no_live_enablement_rehearsal_ready",
            passed: no_live_enablement_rehearsal_ready,
            detail: "enablement chain must remain a no-live rehearsal".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_still_disabled",
            passed: enablement_still_disabled,
            detail:
                "enablementAllowed, approval recording, approval mutation, and reviewed flag must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical WorkGraph write, read, and projection persistence must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_enablement_audit_chain_closeout_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let closeout_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let enablement_audit_chain_closeout_ready = closeout_blockers.is_empty();
    let decision = if enablement_audit_chain_closeout_ready {
        CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_RECORDED_SHADOW
    } else {
        CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_BLOCKED_SHADOW
    };

    WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReceipt {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        closeout_stage: "canonical_projection_enablement_audit_chain_closeout_shadow_only",
        source_packet_job_id: input
            .enablement_operator_review_packet
            .source_packet_job_id
            .clone(),
        enablement_operator_review_decision: input.enablement_operator_review_packet.decision,
        enablement_operator_review_replay_consistency_decision: input
            .enablement_operator_review_replay_consistency_decision
            .decision,
        no_live_rehearsal_closeout_decision: input.no_live_rehearsal_closeout_receipt.decision,
        no_live_rehearsal_closeout_replay_consistency_decision: input
            .no_live_rehearsal_closeout_replay_consistency_decision
            .decision,
        enablement_operator_review_packet_events: input.enablement_operator_review_packet_events,
        enablement_operator_review_replay_consistency_events: input
            .enablement_operator_review_replay_consistency_events,
        no_live_rehearsal_closeout_events: input.no_live_rehearsal_closeout_events,
        no_live_rehearsal_closeout_replay_consistency_events: input
            .no_live_rehearsal_closeout_replay_consistency_events,
        prior_enablement_audit_chain_closeout_events: input
            .prior_enablement_audit_chain_closeout_events,
        enablement_operator_review_packet_readback_ready: input
            .enablement_operator_review_packet_readback_ready,
        enablement_operator_review_replay_consistency_ready: input
            .enablement_operator_review_replay_consistency_ready,
        no_live_rehearsal_closeout_readback_ready: input.no_live_rehearsal_closeout_readback_ready,
        no_live_rehearsal_closeout_replay_consistency_ready: input
            .no_live_rehearsal_closeout_replay_consistency_ready,
        enablement_operator_review_ready: input
            .enablement_operator_review_packet
            .enablement_operator_review_ready,
        enablement_operator_review_replay_consistent,
        no_live_enablement_rehearsal_ready,
        no_live_enablement_rehearsal_closeout_ready,
        no_live_rehearsal_closeout_replay_consistent,
        enablement_audit_chain_closeout_ready,
        shadow_readiness_failed: !enablement_audit_chain_closeout_ready,
        enablement_allowed: false,
        operator_approval_recorded: false,
        approval_record_mutation_enabled: false,
        reviewed_flag_enabled: false,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
        closeout_blockers,
        checks,
        recommended_next_action: "persist replay/readback consistency for this final enablement audit-chain closeout before any approval recording, reviewed flag, canary, blocking, or cutover",
    }
}

pub(crate) fn build_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision(
    input: WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyInput<'_>,
) -> WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyDecision {
    let enablement_audit_chain_closeout_matches_readback = input
        .latest_enablement_audit_chain_closeout_receipt_payload
        == Some(input.enablement_audit_chain_closeout_receipt_payload);
    let closeout_keeps_enablement_disabled = !input
        .enablement_audit_chain_closeout_receipt
        .enablement_allowed
        && !input
            .enablement_audit_chain_closeout_receipt
            .operator_approval_recorded
        && !input
            .enablement_audit_chain_closeout_receipt
            .approval_record_mutation_enabled
        && !input
            .enablement_audit_chain_closeout_receipt
            .reviewed_flag_enabled;
    let canonical_projection_disabled = !input
        .enablement_audit_chain_closeout_receipt
        .canonical_write_enabled
        && !input
            .enablement_audit_chain_closeout_receipt
            .canonical_read_enabled
        && !input
            .enablement_audit_chain_closeout_receipt
            .canonical_projection_persistence_enabled;
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input
            .enablement_audit_chain_closeout_receipt
            .feature_flag_enabled
        && input.enablement_audit_chain_closeout_receipt.canary_stage == "off"
        && input
            .enablement_audit_chain_closeout_receipt
            .canary_traffic_ppm
            == 0
        && !input
            .enablement_audit_chain_closeout_receipt
            .live_blocking_enabled
        && !input
            .enablement_audit_chain_closeout_receipt
            .live_cutover_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_audit_chain_closeout_readback_ready",
            passed: input.enablement_audit_chain_closeout_events > 0
                && input.enablement_audit_chain_closeout_readback_ready,
            detail:
                "durable readback must include the canonical projection enablement audit-chain closeout receipt"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_audit_chain_closeout_latest_payload_matches",
            passed: enablement_audit_chain_closeout_matches_readback,
            detail:
                "latest durable canonical projection enablement audit-chain closeout receipt must match the tool result payload"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_audit_chain_closeout_ready",
            passed: input
                .enablement_audit_chain_closeout_receipt
                .enablement_audit_chain_closeout_ready
                && !input
                    .enablement_audit_chain_closeout_receipt
                    .shadow_readiness_failed,
            detail:
                "canonical projection enablement audit-chain closeout receipt must be ready shadow-only evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_no_live_enablement_rehearsal_ready",
            passed: input
                .enablement_audit_chain_closeout_receipt
                .no_live_enablement_rehearsal_ready,
            detail: "enablement audit-chain closeout must remain no-live".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_still_disabled",
            passed: closeout_keeps_enablement_disabled,
            detail:
                "enablementAllowed, approval recording, approval mutation, and reviewed flag must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical WorkGraph write, read, and projection persistence must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_enablement_audit_chain_closeout_replay_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let consistency_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let replay_consistent = consistency_blockers.is_empty();
    let decision = if replay_consistent {
        CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_REPLAY_CONSISTENT_SHADOW
    } else {
        CANONICAL_PROJECTION_ENABLEMENT_AUDIT_CHAIN_CLOSEOUT_REPLAY_MISMATCH_SHADOW
    };

    WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyDecision {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        replay_stage: "canonical_projection_enablement_audit_chain_closeout_replay_shadow_only",
        source_packet_job_id: input
            .enablement_audit_chain_closeout_receipt
            .source_packet_job_id
            .clone(),
        enablement_audit_chain_closeout_decision: input
            .enablement_audit_chain_closeout_receipt
            .decision,
        enablement_audit_chain_closeout_events: input.enablement_audit_chain_closeout_events,
        prior_enablement_audit_chain_closeout_replay_consistency_events: input
            .prior_enablement_audit_chain_closeout_replay_consistency_events,
        enablement_audit_chain_closeout_readback_ready: input
            .enablement_audit_chain_closeout_readback_ready,
        enablement_audit_chain_closeout_matches_readback,
        enablement_audit_chain_closeout_ready: input
            .enablement_audit_chain_closeout_receipt
            .enablement_audit_chain_closeout_ready,
        no_live_enablement_rehearsal_ready: input
            .enablement_audit_chain_closeout_receipt
            .no_live_enablement_rehearsal_ready,
        enablement_allowed: input
            .enablement_audit_chain_closeout_receipt
            .enablement_allowed,
        operator_approval_recorded: input
            .enablement_audit_chain_closeout_receipt
            .operator_approval_recorded,
        approval_record_mutation_enabled: input
            .enablement_audit_chain_closeout_receipt
            .approval_record_mutation_enabled,
        reviewed_flag_enabled: input
            .enablement_audit_chain_closeout_receipt
            .reviewed_flag_enabled,
        no_live_guardrails_ready,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        replay_consistent,
        shadow_readiness_failed: !replay_consistent,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        consistency_blockers,
        checks,
        feature_flag_enabled: false,
        canary_stage: "off",
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_work_graph_canonical_projection_enablement_activation_precondition_operator_packet(
    input: WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacketInput<'_>,
) -> WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacket {
    let enablement_audit_chain_closeout_ready = input
        .enablement_audit_chain_closeout_receipt
        .enablement_audit_chain_closeout_ready
        && !input
            .enablement_audit_chain_closeout_receipt
            .shadow_readiness_failed;
    let enablement_audit_chain_closeout_replay_consistent = input
        .enablement_audit_chain_closeout_replay_consistency_decision
        .replay_consistent
        && !input
            .enablement_audit_chain_closeout_replay_consistency_decision
            .shadow_readiness_failed;
    let enablement_still_disabled = !input
        .enablement_audit_chain_closeout_receipt
        .enablement_allowed
        && !input
            .enablement_audit_chain_closeout_receipt
            .operator_approval_recorded
        && !input
            .enablement_audit_chain_closeout_receipt
            .approval_record_mutation_enabled
        && !input
            .enablement_audit_chain_closeout_receipt
            .reviewed_flag_enabled
        && !input
            .enablement_audit_chain_closeout_replay_consistency_decision
            .enablement_allowed
        && !input
            .enablement_audit_chain_closeout_replay_consistency_decision
            .operator_approval_recorded
        && !input
            .enablement_audit_chain_closeout_replay_consistency_decision
            .approval_record_mutation_enabled
        && !input
            .enablement_audit_chain_closeout_replay_consistency_decision
            .reviewed_flag_enabled;
    let canonical_projection_disabled = !input
        .enablement_audit_chain_closeout_receipt
        .canonical_write_enabled
        && !input
            .enablement_audit_chain_closeout_receipt
            .canonical_read_enabled
        && !input
            .enablement_audit_chain_closeout_receipt
            .canonical_projection_persistence_enabled
        && !input
            .enablement_audit_chain_closeout_replay_consistency_decision
            .canonical_write_enabled
        && !input
            .enablement_audit_chain_closeout_replay_consistency_decision
            .canonical_read_enabled
        && !input
            .enablement_audit_chain_closeout_replay_consistency_decision
            .canonical_projection_persistence_enabled;
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input
            .enablement_audit_chain_closeout_receipt
            .feature_flag_enabled
        && input.enablement_audit_chain_closeout_receipt.canary_stage == "off"
        && input
            .enablement_audit_chain_closeout_receipt
            .canary_traffic_ppm
            == 0
        && !input
            .enablement_audit_chain_closeout_receipt
            .live_blocking_enabled
        && !input
            .enablement_audit_chain_closeout_receipt
            .live_cutover_enabled
        && !input
            .enablement_audit_chain_closeout_replay_consistency_decision
            .feature_flag_enabled
        && input
            .enablement_audit_chain_closeout_replay_consistency_decision
            .canary_stage
            == "off"
        && !input
            .enablement_audit_chain_closeout_replay_consistency_decision
            .live_blocking_enabled
        && !input
            .enablement_audit_chain_closeout_replay_consistency_decision
            .live_cutover_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_audit_chain_closeout_readback_ready",
            passed: input.enablement_audit_chain_closeout_events > 0
                && input.enablement_audit_chain_closeout_readback_ready,
            detail:
                "final enablement audit-chain closeout receipt must be durably readable"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_audit_chain_closeout_replay_ready",
            passed: input.enablement_audit_chain_closeout_replay_consistency_events > 0
                && input.enablement_audit_chain_closeout_replay_consistency_ready,
            detail:
                "final enablement audit-chain closeout replay consistency must be durably readable"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_audit_chain_closeout_ready",
            passed: enablement_audit_chain_closeout_ready,
            detail:
                "final enablement audit-chain closeout must be ready shadow-only evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_audit_chain_closeout_replay_consistent",
            passed: enablement_audit_chain_closeout_replay_consistent,
            detail:
                "final enablement audit-chain closeout replay consistency must match durable readback"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_still_disabled",
            passed: enablement_still_disabled,
            detail:
                "enablementAllowed, approval recording, approval mutation, and reviewed flag must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical WorkGraph write, read, and projection persistence must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_activation_precondition_guardrails_ready",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let activation_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let activation_precondition_ready = activation_blockers.is_empty();
    let decision = if activation_precondition_ready {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_READY_SHADOW
    } else {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_BLOCKED_SHADOW
    };

    WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacket {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        packet_stage: "canonical_projection_enablement_activation_precondition_shadow_only",
        source_packet_job_id: input
            .enablement_audit_chain_closeout_receipt
            .source_packet_job_id
            .clone(),
        enablement_audit_chain_closeout_decision: input
            .enablement_audit_chain_closeout_receipt
            .decision,
        enablement_audit_chain_closeout_replay_consistency_decision: input
            .enablement_audit_chain_closeout_replay_consistency_decision
            .decision,
        enablement_audit_chain_closeout_events: input.enablement_audit_chain_closeout_events,
        enablement_audit_chain_closeout_replay_consistency_events: input
            .enablement_audit_chain_closeout_replay_consistency_events,
        prior_enablement_activation_precondition_operator_packet_events: input
            .prior_enablement_activation_precondition_operator_packet_events,
        enablement_audit_chain_closeout_readback_ready: input
            .enablement_audit_chain_closeout_readback_ready,
        enablement_audit_chain_closeout_replay_consistency_ready: input
            .enablement_audit_chain_closeout_replay_consistency_ready,
        enablement_audit_chain_closeout_ready,
        enablement_audit_chain_closeout_replay_consistent,
        no_live_enablement_rehearsal_ready: input
            .enablement_audit_chain_closeout_receipt
            .no_live_enablement_rehearsal_ready,
        activation_precondition_ready,
        activation_allowed: false,
        enablement_allowed: input
            .enablement_audit_chain_closeout_receipt
            .enablement_allowed,
        operator_approval_recorded: input
            .enablement_audit_chain_closeout_receipt
            .operator_approval_recorded,
        approval_record_mutation_enabled: input
            .enablement_audit_chain_closeout_receipt
            .approval_record_mutation_enabled,
        reviewed_flag_enabled: input
            .enablement_audit_chain_closeout_receipt
            .reviewed_flag_enabled,
        approval_record_required_before_activation: true,
        reviewed_flag_required_before_activation: true,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        no_live_guardrails_ready,
        shadow_readiness_failed: !activation_precondition_ready,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        activation_blockers,
        checks,
        recommended_next_action: "prepare operator-reviewed approval recording preflight while keeping activationAllowed=false, reviewed flag disabled, and canonical WorkGraph read/write disabled",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision(
    input: WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyInput<'_>,
) -> WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyDecision {
    let activation_precondition_operator_packet_matches_readback = input
        .latest_activation_precondition_operator_packet_payload
        == Some(input.activation_precondition_operator_packet_payload);
    let activation_precondition_ready = input
        .activation_precondition_operator_packet
        .activation_precondition_ready
        && !input
            .activation_precondition_operator_packet
            .shadow_readiness_failed;
    let activation_still_denied = !input
        .activation_precondition_operator_packet
        .activation_allowed
        && !input
            .activation_precondition_operator_packet
            .enablement_allowed
        && !input
            .activation_precondition_operator_packet
            .operator_approval_recorded
        && !input
            .activation_precondition_operator_packet
            .approval_record_mutation_enabled
        && !input
            .activation_precondition_operator_packet
            .reviewed_flag_enabled
        && input
            .activation_precondition_operator_packet
            .approval_record_required_before_activation
        && input
            .activation_precondition_operator_packet
            .reviewed_flag_required_before_activation;
    let canonical_projection_disabled = !input
        .activation_precondition_operator_packet
        .canonical_write_enabled
        && !input
            .activation_precondition_operator_packet
            .canonical_read_enabled
        && !input
            .activation_precondition_operator_packet
            .canonical_projection_persistence_enabled;
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input
            .activation_precondition_operator_packet
            .feature_flag_enabled
        && input.activation_precondition_operator_packet.canary_stage == "off"
        && input
            .activation_precondition_operator_packet
            .canary_traffic_ppm
            == 0
        && !input
            .activation_precondition_operator_packet
            .live_blocking_enabled
        && !input
            .activation_precondition_operator_packet
            .live_cutover_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_precondition_readback_ready",
            passed: input.activation_precondition_operator_packet_events > 0
                && input.activation_precondition_operator_packet_readback_ready,
            detail:
                "durable readback must include the canonical projection enablement activation-precondition operator packet"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_precondition_latest_payload_matches",
            passed: activation_precondition_operator_packet_matches_readback,
            detail:
                "latest durable canonical projection enablement activation-precondition operator packet must match the tool result payload"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_precondition_ready",
            passed: activation_precondition_ready,
            detail:
                "canonical projection enablement activation-precondition packet must be ready shadow-only evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_still_denied",
            passed: activation_still_denied,
            detail:
                "activationAllowed, enablementAllowed, approval recording, approval mutation, and reviewed flag must remain disabled while approval/review prerequisites stay required"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical WorkGraph write, read, and projection persistence must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_activation_precondition_replay_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let consistency_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let replay_consistent = consistency_blockers.is_empty();
    let decision = if replay_consistent {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_REPLAY_CONSISTENT_SHADOW
    } else {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_PRECONDITION_REPLAY_MISMATCH_SHADOW
    };

    WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyDecision {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        replay_stage: "canonical_projection_enablement_activation_precondition_replay_shadow_only",
        source_packet_job_id: input
            .activation_precondition_operator_packet
            .source_packet_job_id
            .clone(),
        activation_precondition_decision: input.activation_precondition_operator_packet.decision,
        activation_precondition_operator_packet_events: input
            .activation_precondition_operator_packet_events,
        prior_activation_precondition_replay_consistency_events: input
            .prior_activation_precondition_replay_consistency_events,
        activation_precondition_operator_packet_readback_ready: input
            .activation_precondition_operator_packet_readback_ready,
        activation_precondition_operator_packet_matches_readback,
        activation_precondition_ready: input
            .activation_precondition_operator_packet
            .activation_precondition_ready,
        activation_allowed: input
            .activation_precondition_operator_packet
            .activation_allowed,
        enablement_allowed: input
            .activation_precondition_operator_packet
            .enablement_allowed,
        operator_approval_recorded: input
            .activation_precondition_operator_packet
            .operator_approval_recorded,
        approval_record_mutation_enabled: input
            .activation_precondition_operator_packet
            .approval_record_mutation_enabled,
        reviewed_flag_enabled: input
            .activation_precondition_operator_packet
            .reviewed_flag_enabled,
        approval_record_required_before_activation: input
            .activation_precondition_operator_packet
            .approval_record_required_before_activation,
        reviewed_flag_required_before_activation: input
            .activation_precondition_operator_packet
            .reviewed_flag_required_before_activation,
        no_live_guardrails_ready,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        replay_consistent,
        shadow_readiness_failed: !replay_consistent,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        consistency_blockers,
        checks,
        feature_flag_enabled: false,
        canary_stage: "off",
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt(
    input: WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutInput<'_>,
) -> WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReceipt {
    let activation_precondition_ready = input
        .activation_precondition_operator_packet
        .activation_precondition_ready
        && !input
            .activation_precondition_operator_packet
            .shadow_readiness_failed;
    let activation_precondition_replay_consistent = input
        .activation_precondition_replay_consistency_decision
        .replay_consistent
        && !input
            .activation_precondition_replay_consistency_decision
            .shadow_readiness_failed;
    let no_live_enablement_rehearsal_ready = input
        .activation_precondition_operator_packet
        .no_live_enablement_rehearsal_ready
        && input
            .activation_precondition_replay_consistency_decision
            .no_live_guardrails_ready;
    let activation_still_denied = !input
        .activation_precondition_operator_packet
        .activation_allowed
        && !input
            .activation_precondition_replay_consistency_decision
            .activation_allowed
        && !input
            .activation_precondition_operator_packet
            .enablement_allowed
        && !input
            .activation_precondition_replay_consistency_decision
            .enablement_allowed
        && !input
            .activation_precondition_operator_packet
            .operator_approval_recorded
        && !input
            .activation_precondition_replay_consistency_decision
            .operator_approval_recorded
        && !input
            .activation_precondition_operator_packet
            .approval_record_mutation_enabled
        && !input
            .activation_precondition_replay_consistency_decision
            .approval_record_mutation_enabled
        && !input
            .activation_precondition_operator_packet
            .reviewed_flag_enabled
        && !input
            .activation_precondition_replay_consistency_decision
            .reviewed_flag_enabled
        && input
            .activation_precondition_operator_packet
            .approval_record_required_before_activation
        && input
            .activation_precondition_replay_consistency_decision
            .approval_record_required_before_activation
        && input
            .activation_precondition_operator_packet
            .reviewed_flag_required_before_activation
        && input
            .activation_precondition_replay_consistency_decision
            .reviewed_flag_required_before_activation;
    let canonical_projection_disabled = !input
        .activation_precondition_operator_packet
        .canonical_write_enabled
        && !input
            .activation_precondition_operator_packet
            .canonical_read_enabled
        && !input
            .activation_precondition_operator_packet
            .canonical_projection_persistence_enabled
        && !input
            .activation_precondition_replay_consistency_decision
            .canonical_write_enabled
        && !input
            .activation_precondition_replay_consistency_decision
            .canonical_read_enabled
        && !input
            .activation_precondition_replay_consistency_decision
            .canonical_projection_persistence_enabled;
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input
            .activation_precondition_operator_packet
            .feature_flag_enabled
        && input.activation_precondition_operator_packet.canary_stage == "off"
        && input
            .activation_precondition_operator_packet
            .canary_traffic_ppm
            == 0
        && !input
            .activation_precondition_operator_packet
            .live_blocking_enabled
        && !input
            .activation_precondition_operator_packet
            .live_cutover_enabled
        && !input
            .activation_precondition_replay_consistency_decision
            .feature_flag_enabled
        && input
            .activation_precondition_replay_consistency_decision
            .canary_stage
            == "off"
        && !input
            .activation_precondition_replay_consistency_decision
            .live_blocking_enabled
        && !input
            .activation_precondition_replay_consistency_decision
            .live_cutover_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_precondition_packet_readback_ready",
            passed: input.activation_precondition_operator_packet_events > 0
                && input.activation_precondition_operator_packet_readback_ready,
            detail:
                "durable readback must include canonical projection enablement activation-precondition packet evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_precondition_replay_ready",
            passed: input.activation_precondition_replay_consistency_events > 0
                && input.activation_precondition_replay_consistency_ready
                && activation_precondition_replay_consistent,
            detail:
                "canonical projection enablement activation-precondition replay must be durable and consistent"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_precondition_ready",
            passed: activation_precondition_ready,
            detail:
                "canonical projection enablement activation-precondition packet must be ready shadow-only evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_still_denied",
            passed: activation_still_denied,
            detail:
                "activationAllowed, enablementAllowed, approval recording, approval mutation, and reviewed flag must remain disabled while approval/review prerequisites stay required"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical WorkGraph write, read, and projection persistence must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_enablement_activation_closeout_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let closeout_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let activation_no_live_closeout_ready = closeout_blockers.is_empty();
    let decision = if activation_no_live_closeout_ready {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_RECORDED_SHADOW
    } else {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_BLOCKED_SHADOW
    };

    WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReceipt {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        closeout_stage: "canonical_projection_enablement_activation_no_live_closeout_shadow_only",
        source_packet_job_id: input
            .activation_precondition_operator_packet
            .source_packet_job_id
            .clone(),
        activation_precondition_decision: input.activation_precondition_operator_packet.decision,
        activation_precondition_replay_consistency_decision: input
            .activation_precondition_replay_consistency_decision
            .decision,
        activation_precondition_operator_packet_events: input
            .activation_precondition_operator_packet_events,
        activation_precondition_replay_consistency_events: input
            .activation_precondition_replay_consistency_events,
        prior_activation_no_live_closeout_events: input.prior_activation_no_live_closeout_events,
        activation_precondition_operator_packet_readback_ready: input
            .activation_precondition_operator_packet_readback_ready,
        activation_precondition_replay_consistency_ready: input
            .activation_precondition_replay_consistency_ready,
        activation_precondition_ready,
        activation_precondition_replay_consistent,
        no_live_enablement_rehearsal_ready,
        activation_no_live_closeout_ready,
        activation_allowed: false,
        enablement_allowed: false,
        operator_approval_recorded: false,
        approval_record_mutation_enabled: false,
        reviewed_flag_enabled: false,
        approval_record_required_before_activation: true,
        reviewed_flag_required_before_activation: true,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
        closeout_blockers,
        checks,
        recommended_next_action: "persist replay/readback consistency for this activation no-live closeout before any approval recording, reviewed flag, canary, blocking, or cutover",
    }
}

pub(crate) fn build_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision(
    input: WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyInput<'_>,
) -> WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyDecision {
    let activation_no_live_closeout_matches_readback = input
        .latest_activation_no_live_closeout_receipt_payload
        == Some(input.activation_no_live_closeout_receipt_payload);
    let activation_precondition_ready = input
        .activation_no_live_closeout_receipt
        .activation_precondition_ready
        && input
            .activation_no_live_closeout_receipt
            .activation_precondition_replay_consistent;
    let activation_still_denied = !input.activation_no_live_closeout_receipt.activation_allowed
        && !input.activation_no_live_closeout_receipt.enablement_allowed
        && !input
            .activation_no_live_closeout_receipt
            .operator_approval_recorded
        && !input
            .activation_no_live_closeout_receipt
            .approval_record_mutation_enabled
        && !input
            .activation_no_live_closeout_receipt
            .reviewed_flag_enabled
        && input
            .activation_no_live_closeout_receipt
            .approval_record_required_before_activation
        && input
            .activation_no_live_closeout_receipt
            .reviewed_flag_required_before_activation;
    let canonical_projection_disabled = !input
        .activation_no_live_closeout_receipt
        .canonical_write_enabled
        && !input
            .activation_no_live_closeout_receipt
            .canonical_read_enabled
        && !input
            .activation_no_live_closeout_receipt
            .canonical_projection_persistence_enabled;
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input
            .activation_no_live_closeout_receipt
            .feature_flag_enabled
        && input.activation_no_live_closeout_receipt.canary_stage == "off"
        && input.activation_no_live_closeout_receipt.canary_traffic_ppm == 0
        && !input
            .activation_no_live_closeout_receipt
            .live_blocking_enabled
        && !input
            .activation_no_live_closeout_receipt
            .live_cutover_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_no_live_closeout_readback_ready",
            passed: input.activation_no_live_closeout_events > 0
                && input.activation_no_live_closeout_readback_ready,
            detail:
                "durable readback must include the canonical projection enablement activation no-live closeout receipt"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_no_live_closeout_latest_payload_matches",
            passed: activation_no_live_closeout_matches_readback,
            detail:
                "latest durable canonical projection enablement activation no-live closeout receipt must match the tool result payload"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_no_live_closeout_ready",
            passed: input
                .activation_no_live_closeout_receipt
                .activation_no_live_closeout_ready,
            detail:
                "canonical projection enablement activation no-live closeout receipt must be ready shadow-only evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_precondition_ready",
            passed: activation_precondition_ready,
            detail:
                "canonical projection enablement activation precondition must remain ready and replay-consistent"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_still_denied",
            passed: activation_still_denied,
            detail:
                "activationAllowed, enablementAllowed, approval recording, approval mutation, and reviewed flag must remain disabled while approval/review prerequisites stay required"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical WorkGraph write, read, and projection persistence must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_enablement_activation_no_live_closeout_replay_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let consistency_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let replay_consistent = consistency_blockers.is_empty();
    let decision = if replay_consistent {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_REPLAY_CONSISTENT_SHADOW
    } else {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_NO_LIVE_CLOSEOUT_REPLAY_MISMATCH_SHADOW
    };

    WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyDecision {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        replay_stage: "canonical_projection_enablement_activation_no_live_closeout_replay_shadow_only",
        source_packet_job_id: input
            .activation_no_live_closeout_receipt
            .source_packet_job_id
            .clone(),
        activation_no_live_closeout_decision: input.activation_no_live_closeout_receipt.decision,
        activation_no_live_closeout_events: input.activation_no_live_closeout_events,
        prior_activation_no_live_closeout_replay_consistency_events: input
            .prior_activation_no_live_closeout_replay_consistency_events,
        activation_no_live_closeout_readback_ready: input
            .activation_no_live_closeout_readback_ready,
        activation_no_live_closeout_matches_readback,
        activation_no_live_closeout_ready: input
            .activation_no_live_closeout_receipt
            .activation_no_live_closeout_ready,
        activation_precondition_ready: input
            .activation_no_live_closeout_receipt
            .activation_precondition_ready,
        activation_precondition_replay_consistent: input
            .activation_no_live_closeout_receipt
            .activation_precondition_replay_consistent,
        no_live_enablement_rehearsal_ready: input
            .activation_no_live_closeout_receipt
            .no_live_enablement_rehearsal_ready,
        activation_allowed: input.activation_no_live_closeout_receipt.activation_allowed,
        enablement_allowed: input.activation_no_live_closeout_receipt.enablement_allowed,
        operator_approval_recorded: input
            .activation_no_live_closeout_receipt
            .operator_approval_recorded,
        approval_record_mutation_enabled: input
            .activation_no_live_closeout_receipt
            .approval_record_mutation_enabled,
        reviewed_flag_enabled: input
            .activation_no_live_closeout_receipt
            .reviewed_flag_enabled,
        approval_record_required_before_activation: input
            .activation_no_live_closeout_receipt
            .approval_record_required_before_activation,
        reviewed_flag_required_before_activation: input
            .activation_no_live_closeout_receipt
            .reviewed_flag_required_before_activation,
        no_live_guardrails_ready,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        replay_consistent,
        shadow_readiness_failed: !replay_consistent,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        consistency_blockers,
        checks,
        feature_flag_enabled: false,
        canary_stage: "off",
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt(
    input: WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutInput<'_>,
) -> WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReceipt {
    let activation_precondition_ready = input
        .activation_precondition_operator_packet
        .activation_precondition_ready
        && !input
            .activation_precondition_operator_packet
            .shadow_readiness_failed;
    let activation_precondition_replay_consistent = input
        .activation_precondition_replay_consistency_decision
        .replay_consistent
        && !input
            .activation_precondition_replay_consistency_decision
            .shadow_readiness_failed;
    let activation_no_live_closeout_ready = input
        .activation_no_live_closeout_receipt
        .activation_no_live_closeout_ready;
    let activation_no_live_closeout_replay_consistent = input
        .activation_no_live_closeout_replay_consistency_decision
        .replay_consistent
        && !input
            .activation_no_live_closeout_replay_consistency_decision
            .shadow_readiness_failed;
    let no_live_enablement_rehearsal_ready = input
        .activation_precondition_operator_packet
        .no_live_enablement_rehearsal_ready
        && input
            .activation_precondition_replay_consistency_decision
            .no_live_guardrails_ready
        && input
            .activation_no_live_closeout_receipt
            .no_live_enablement_rehearsal_ready
        && input
            .activation_no_live_closeout_replay_consistency_decision
            .no_live_enablement_rehearsal_ready
        && input
            .activation_no_live_closeout_replay_consistency_decision
            .no_live_guardrails_ready;
    let activation_still_denied = !input
        .activation_precondition_operator_packet
        .activation_allowed
        && !input
            .activation_precondition_replay_consistency_decision
            .activation_allowed
        && !input.activation_no_live_closeout_receipt.activation_allowed
        && !input
            .activation_no_live_closeout_replay_consistency_decision
            .activation_allowed
        && !input
            .activation_precondition_operator_packet
            .enablement_allowed
        && !input
            .activation_precondition_replay_consistency_decision
            .enablement_allowed
        && !input.activation_no_live_closeout_receipt.enablement_allowed
        && !input
            .activation_no_live_closeout_replay_consistency_decision
            .enablement_allowed
        && !input
            .activation_precondition_operator_packet
            .operator_approval_recorded
        && !input
            .activation_precondition_replay_consistency_decision
            .operator_approval_recorded
        && !input
            .activation_no_live_closeout_receipt
            .operator_approval_recorded
        && !input
            .activation_no_live_closeout_replay_consistency_decision
            .operator_approval_recorded
        && !input
            .activation_precondition_operator_packet
            .approval_record_mutation_enabled
        && !input
            .activation_precondition_replay_consistency_decision
            .approval_record_mutation_enabled
        && !input
            .activation_no_live_closeout_receipt
            .approval_record_mutation_enabled
        && !input
            .activation_no_live_closeout_replay_consistency_decision
            .approval_record_mutation_enabled
        && !input
            .activation_precondition_operator_packet
            .reviewed_flag_enabled
        && !input
            .activation_precondition_replay_consistency_decision
            .reviewed_flag_enabled
        && !input
            .activation_no_live_closeout_receipt
            .reviewed_flag_enabled
        && !input
            .activation_no_live_closeout_replay_consistency_decision
            .reviewed_flag_enabled
        && input
            .activation_precondition_operator_packet
            .approval_record_required_before_activation
        && input
            .activation_precondition_replay_consistency_decision
            .approval_record_required_before_activation
        && input
            .activation_no_live_closeout_receipt
            .approval_record_required_before_activation
        && input
            .activation_no_live_closeout_replay_consistency_decision
            .approval_record_required_before_activation
        && input
            .activation_precondition_operator_packet
            .reviewed_flag_required_before_activation
        && input
            .activation_precondition_replay_consistency_decision
            .reviewed_flag_required_before_activation
        && input
            .activation_no_live_closeout_receipt
            .reviewed_flag_required_before_activation
        && input
            .activation_no_live_closeout_replay_consistency_decision
            .reviewed_flag_required_before_activation;
    let canonical_projection_disabled = !input
        .activation_precondition_operator_packet
        .canonical_write_enabled
        && !input
            .activation_precondition_operator_packet
            .canonical_read_enabled
        && !input
            .activation_precondition_operator_packet
            .canonical_projection_persistence_enabled
        && !input
            .activation_precondition_replay_consistency_decision
            .canonical_write_enabled
        && !input
            .activation_precondition_replay_consistency_decision
            .canonical_read_enabled
        && !input
            .activation_precondition_replay_consistency_decision
            .canonical_projection_persistence_enabled
        && !input
            .activation_no_live_closeout_receipt
            .canonical_write_enabled
        && !input
            .activation_no_live_closeout_receipt
            .canonical_read_enabled
        && !input
            .activation_no_live_closeout_receipt
            .canonical_projection_persistence_enabled
        && !input
            .activation_no_live_closeout_replay_consistency_decision
            .canonical_write_enabled
        && !input
            .activation_no_live_closeout_replay_consistency_decision
            .canonical_read_enabled
        && !input
            .activation_no_live_closeout_replay_consistency_decision
            .canonical_projection_persistence_enabled;
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input
            .activation_precondition_operator_packet
            .feature_flag_enabled
        && input.activation_precondition_operator_packet.canary_stage == "off"
        && input
            .activation_precondition_operator_packet
            .canary_traffic_ppm
            == 0
        && !input
            .activation_precondition_operator_packet
            .live_blocking_enabled
        && !input
            .activation_precondition_operator_packet
            .live_cutover_enabled
        && !input
            .activation_precondition_replay_consistency_decision
            .feature_flag_enabled
        && input
            .activation_precondition_replay_consistency_decision
            .canary_stage
            == "off"
        && !input
            .activation_precondition_replay_consistency_decision
            .live_blocking_enabled
        && !input
            .activation_precondition_replay_consistency_decision
            .live_cutover_enabled
        && !input
            .activation_no_live_closeout_receipt
            .feature_flag_enabled
        && input.activation_no_live_closeout_receipt.canary_stage == "off"
        && input.activation_no_live_closeout_receipt.canary_traffic_ppm == 0
        && !input
            .activation_no_live_closeout_receipt
            .live_blocking_enabled
        && !input
            .activation_no_live_closeout_receipt
            .live_cutover_enabled
        && !input
            .activation_no_live_closeout_replay_consistency_decision
            .feature_flag_enabled
        && input
            .activation_no_live_closeout_replay_consistency_decision
            .canary_stage
            == "off"
        && !input
            .activation_no_live_closeout_replay_consistency_decision
            .live_blocking_enabled
        && !input
            .activation_no_live_closeout_replay_consistency_decision
            .live_cutover_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_precondition_packet_readback_ready",
            passed: input.activation_precondition_operator_packet_events > 0
                && input.activation_precondition_operator_packet_readback_ready
                && activation_precondition_ready,
            detail:
                "durable readback must include ready canonical projection enablement activation-precondition packet evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_precondition_replay_ready",
            passed: input.activation_precondition_replay_consistency_events > 0
                && input.activation_precondition_replay_consistency_ready
                && activation_precondition_replay_consistent,
            detail:
                "canonical projection enablement activation-precondition replay must be durable and consistent"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_no_live_closeout_readback_ready",
            passed: input.activation_no_live_closeout_events > 0
                && input.activation_no_live_closeout_readback_ready
                && activation_no_live_closeout_ready,
            detail:
                "durable readback must include ready canonical projection enablement activation no-live closeout evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_no_live_closeout_replay_ready",
            passed: input.activation_no_live_closeout_replay_consistency_events > 0
                && input.activation_no_live_closeout_replay_consistency_ready
                && activation_no_live_closeout_replay_consistent,
            detail:
                "canonical projection enablement activation no-live closeout replay must be durable and consistent"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_no_live_activation_rehearsal_ready",
            passed: no_live_enablement_rehearsal_ready,
            detail: "activation chain must remain a no-live rehearsal".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_still_denied",
            passed: activation_still_denied,
            detail:
                "activationAllowed, enablementAllowed, approval recording, approval mutation, and reviewed flag must remain disabled while approval/review prerequisites stay required"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical WorkGraph write, read, and projection persistence must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_enablement_activation_audit_chain_closeout_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let closeout_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let activation_audit_chain_closeout_ready = closeout_blockers.is_empty();
    let decision = if activation_audit_chain_closeout_ready {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_RECORDED_SHADOW
    } else {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_BLOCKED_SHADOW
    };

    WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReceipt {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        closeout_stage: "canonical_projection_enablement_activation_audit_chain_closeout_shadow_only",
        source_packet_job_id: input
            .activation_precondition_operator_packet
            .source_packet_job_id
            .clone(),
        activation_precondition_decision: input.activation_precondition_operator_packet.decision,
        activation_precondition_replay_consistency_decision: input
            .activation_precondition_replay_consistency_decision
            .decision,
        activation_no_live_closeout_decision: input.activation_no_live_closeout_receipt.decision,
        activation_no_live_closeout_replay_consistency_decision: input
            .activation_no_live_closeout_replay_consistency_decision
            .decision,
        activation_precondition_operator_packet_events: input
            .activation_precondition_operator_packet_events,
        activation_precondition_replay_consistency_events: input
            .activation_precondition_replay_consistency_events,
        activation_no_live_closeout_events: input.activation_no_live_closeout_events,
        activation_no_live_closeout_replay_consistency_events: input
            .activation_no_live_closeout_replay_consistency_events,
        prior_activation_audit_chain_closeout_events: input
            .prior_activation_audit_chain_closeout_events,
        activation_precondition_operator_packet_readback_ready: input
            .activation_precondition_operator_packet_readback_ready,
        activation_precondition_replay_consistency_ready: input
            .activation_precondition_replay_consistency_ready,
        activation_no_live_closeout_readback_ready: input
            .activation_no_live_closeout_readback_ready,
        activation_no_live_closeout_replay_consistency_ready: input
            .activation_no_live_closeout_replay_consistency_ready,
        activation_precondition_ready,
        activation_precondition_replay_consistent,
        activation_no_live_closeout_ready,
        activation_no_live_closeout_replay_consistent,
        no_live_enablement_rehearsal_ready,
        no_live_guardrails_ready,
        activation_audit_chain_closeout_ready,
        shadow_readiness_failed: !activation_audit_chain_closeout_ready,
        activation_allowed: false,
        enablement_allowed: false,
        operator_approval_recorded: false,
        approval_record_mutation_enabled: false,
        reviewed_flag_enabled: false,
        approval_record_required_before_activation: true,
        reviewed_flag_required_before_activation: true,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
        closeout_blockers,
        checks,
        recommended_next_action: "persist replay/readback consistency for this final activation audit-chain closeout before any approval recording, reviewed flag, canary, blocking, or cutover",
    }
}

pub(crate) fn build_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision(
    input: WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyInput<
        '_,
    >,
) -> WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyDecision {
    let activation_audit_chain_closeout_matches_readback = input
        .latest_activation_audit_chain_closeout_receipt_payload
        == Some(input.activation_audit_chain_closeout_receipt_payload);
    let closeout_keeps_activation_denied = !input
        .activation_audit_chain_closeout_receipt
        .activation_allowed
        && !input
            .activation_audit_chain_closeout_receipt
            .enablement_allowed
        && !input
            .activation_audit_chain_closeout_receipt
            .operator_approval_recorded
        && !input
            .activation_audit_chain_closeout_receipt
            .approval_record_mutation_enabled
        && !input
            .activation_audit_chain_closeout_receipt
            .reviewed_flag_enabled
        && input
            .activation_audit_chain_closeout_receipt
            .approval_record_required_before_activation
        && input
            .activation_audit_chain_closeout_receipt
            .reviewed_flag_required_before_activation;
    let canonical_projection_disabled = !input
        .activation_audit_chain_closeout_receipt
        .canonical_write_enabled
        && !input
            .activation_audit_chain_closeout_receipt
            .canonical_read_enabled
        && !input
            .activation_audit_chain_closeout_receipt
            .canonical_projection_persistence_enabled;
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input
            .activation_audit_chain_closeout_receipt
            .feature_flag_enabled
        && input.activation_audit_chain_closeout_receipt.canary_stage == "off"
        && input
            .activation_audit_chain_closeout_receipt
            .canary_traffic_ppm
            == 0
        && !input
            .activation_audit_chain_closeout_receipt
            .live_blocking_enabled
        && !input
            .activation_audit_chain_closeout_receipt
            .live_cutover_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_audit_chain_closeout_readback_ready",
            passed: input.activation_audit_chain_closeout_events > 0
                && input.activation_audit_chain_closeout_readback_ready,
            detail:
                "durable readback must include the canonical projection enablement activation audit-chain closeout receipt"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_audit_chain_closeout_latest_payload_matches",
            passed: activation_audit_chain_closeout_matches_readback,
            detail:
                "latest durable canonical projection enablement activation audit-chain closeout receipt must match the tool result payload"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_audit_chain_closeout_ready",
            passed: input
                .activation_audit_chain_closeout_receipt
                .activation_audit_chain_closeout_ready
                && !input
                    .activation_audit_chain_closeout_receipt
                    .shadow_readiness_failed,
            detail:
                "canonical projection enablement activation audit-chain closeout receipt must be ready shadow-only evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_no_live_activation_rehearsal_ready",
            passed: input
                .activation_audit_chain_closeout_receipt
                .no_live_enablement_rehearsal_ready,
            detail: "activation audit-chain closeout must remain no-live".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_activation_still_denied",
            passed: closeout_keeps_activation_denied,
            detail:
                "activationAllowed, enablementAllowed, approval recording, approval mutation, and reviewed flag must remain disabled while approval/review prerequisites stay required"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical WorkGraph write, read, and projection persistence must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_enablement_activation_audit_chain_closeout_replay_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let consistency_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let replay_consistent = consistency_blockers.is_empty();
    let decision = if replay_consistent {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_REPLAY_CONSISTENT_SHADOW
    } else {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_AUDIT_CHAIN_CLOSEOUT_REPLAY_MISMATCH_SHADOW
    };

    WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyDecision {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        replay_stage: "canonical_projection_enablement_activation_audit_chain_closeout_replay_shadow_only",
        source_packet_job_id: input
            .activation_audit_chain_closeout_receipt
            .source_packet_job_id
            .clone(),
        activation_audit_chain_closeout_decision: input
            .activation_audit_chain_closeout_receipt
            .decision,
        activation_audit_chain_closeout_events: input.activation_audit_chain_closeout_events,
        prior_activation_audit_chain_closeout_replay_consistency_events: input
            .prior_activation_audit_chain_closeout_replay_consistency_events,
        activation_audit_chain_closeout_readback_ready: input
            .activation_audit_chain_closeout_readback_ready,
        activation_audit_chain_closeout_matches_readback,
        activation_audit_chain_closeout_ready: input
            .activation_audit_chain_closeout_receipt
            .activation_audit_chain_closeout_ready,
        activation_precondition_ready: input
            .activation_audit_chain_closeout_receipt
            .activation_precondition_ready,
        activation_precondition_replay_consistent: input
            .activation_audit_chain_closeout_receipt
            .activation_precondition_replay_consistent,
        activation_no_live_closeout_ready: input
            .activation_audit_chain_closeout_receipt
            .activation_no_live_closeout_ready,
        activation_no_live_closeout_replay_consistent: input
            .activation_audit_chain_closeout_receipt
            .activation_no_live_closeout_replay_consistent,
        no_live_enablement_rehearsal_ready: input
            .activation_audit_chain_closeout_receipt
            .no_live_enablement_rehearsal_ready,
        activation_allowed: input
            .activation_audit_chain_closeout_receipt
            .activation_allowed,
        enablement_allowed: input
            .activation_audit_chain_closeout_receipt
            .enablement_allowed,
        operator_approval_recorded: input
            .activation_audit_chain_closeout_receipt
            .operator_approval_recorded,
        approval_record_mutation_enabled: input
            .activation_audit_chain_closeout_receipt
            .approval_record_mutation_enabled,
        reviewed_flag_enabled: input
            .activation_audit_chain_closeout_receipt
            .reviewed_flag_enabled,
        approval_record_required_before_activation: input
            .activation_audit_chain_closeout_receipt
            .approval_record_required_before_activation,
        reviewed_flag_required_before_activation: input
            .activation_audit_chain_closeout_receipt
            .reviewed_flag_required_before_activation,
        no_live_guardrails_ready,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        replay_consistent,
        shadow_readiness_failed: !replay_consistent,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        consistency_blockers,
        checks,
        feature_flag_enabled: false,
        canary_stage: "off",
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet(
    input: WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacketInput<
        '_,
    >,
) -> WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacket {
    let activation_audit_chain_closeout_ready = input
        .activation_audit_chain_closeout_receipt
        .activation_audit_chain_closeout_ready
        && !input
            .activation_audit_chain_closeout_receipt
            .shadow_readiness_failed;
    let activation_audit_chain_closeout_replay_consistent = input
        .activation_audit_chain_closeout_replay_consistency_decision
        .replay_consistent
        && !input
            .activation_audit_chain_closeout_replay_consistency_decision
            .shadow_readiness_failed
        && input
            .activation_audit_chain_closeout_replay_consistency_decision
            .activation_audit_chain_closeout_matches_readback;
    let activation_still_denied = !input
        .activation_audit_chain_closeout_receipt
        .activation_allowed
        && !input
            .activation_audit_chain_closeout_replay_consistency_decision
            .activation_allowed
        && !input
            .activation_audit_chain_closeout_receipt
            .enablement_allowed
        && !input
            .activation_audit_chain_closeout_replay_consistency_decision
            .enablement_allowed
        && !input
            .activation_audit_chain_closeout_receipt
            .operator_approval_recorded
        && !input
            .activation_audit_chain_closeout_replay_consistency_decision
            .operator_approval_recorded
        && !input
            .activation_audit_chain_closeout_receipt
            .approval_record_mutation_enabled
        && !input
            .activation_audit_chain_closeout_replay_consistency_decision
            .approval_record_mutation_enabled
        && !input
            .activation_audit_chain_closeout_receipt
            .reviewed_flag_enabled
        && !input
            .activation_audit_chain_closeout_replay_consistency_decision
            .reviewed_flag_enabled;
    let activation_prerequisites_required = input
        .activation_audit_chain_closeout_receipt
        .approval_record_required_before_activation
        && input
            .activation_audit_chain_closeout_replay_consistency_decision
            .approval_record_required_before_activation
        && input
            .activation_audit_chain_closeout_receipt
            .reviewed_flag_required_before_activation
        && input
            .activation_audit_chain_closeout_replay_consistency_decision
            .reviewed_flag_required_before_activation;
    let canonical_projection_disabled = !input
        .activation_audit_chain_closeout_receipt
        .canonical_write_enabled
        && !input
            .activation_audit_chain_closeout_receipt
            .canonical_read_enabled
        && !input
            .activation_audit_chain_closeout_receipt
            .canonical_projection_persistence_enabled
        && !input
            .activation_audit_chain_closeout_replay_consistency_decision
            .canonical_write_enabled
        && !input
            .activation_audit_chain_closeout_replay_consistency_decision
            .canonical_read_enabled
        && !input
            .activation_audit_chain_closeout_replay_consistency_decision
            .canonical_projection_persistence_enabled;
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input
            .activation_audit_chain_closeout_receipt
            .feature_flag_enabled
        && input.activation_audit_chain_closeout_receipt.canary_stage == "off"
        && input
            .activation_audit_chain_closeout_receipt
            .canary_traffic_ppm
            == 0
        && !input
            .activation_audit_chain_closeout_receipt
            .live_blocking_enabled
        && !input
            .activation_audit_chain_closeout_receipt
            .live_cutover_enabled
        && !input
            .activation_audit_chain_closeout_replay_consistency_decision
            .feature_flag_enabled
        && input
            .activation_audit_chain_closeout_replay_consistency_decision
            .canary_stage
            == "off"
        && !input
            .activation_audit_chain_closeout_replay_consistency_decision
            .live_blocking_enabled
        && !input
            .activation_audit_chain_closeout_replay_consistency_decision
            .live_cutover_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_audit_chain_closeout_readback_ready",
            passed: input.activation_audit_chain_closeout_events > 0
                && input.activation_audit_chain_closeout_readback_ready
                && activation_audit_chain_closeout_ready,
            detail:
                "durable readback must include ready canonical projection enablement activation audit-chain closeout evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_audit_chain_closeout_replay_ready",
            passed: input.activation_audit_chain_closeout_replay_consistency_events > 0
                && input.activation_audit_chain_closeout_replay_consistency_ready
                && activation_audit_chain_closeout_replay_consistent,
            detail:
                "canonical projection enablement activation audit-chain closeout replay must be durable and consistent"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_activation_operator_approval_prerequisites_required",
            passed: activation_prerequisites_required,
            detail:
                "operator approval, approval record, and reviewed flag must remain required before activation"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_activation_still_denied",
            passed: activation_still_denied,
            detail:
                "activationAllowed, enablementAllowed, approval recording, approval mutation, and reviewed flag mutation must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical WorkGraph write, read, and projection persistence must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_enablement_activation_operator_approval_preflight_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let preflight_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let activation_operator_approval_readiness_preflight_ready = preflight_blockers.is_empty();
    let decision = if activation_operator_approval_readiness_preflight_ready {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_READY_SHADOW
    } else {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_BLOCKED_SHADOW
    };

    WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacket {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        preflight_stage: "canonical_projection_enablement_activation_operator_approval_readiness_preflight_shadow_only",
        source_packet_job_id: input
            .activation_audit_chain_closeout_receipt
            .source_packet_job_id
            .clone(),
        activation_audit_chain_closeout_decision: input
            .activation_audit_chain_closeout_receipt
            .decision,
        activation_audit_chain_closeout_replay_consistency_decision: input
            .activation_audit_chain_closeout_replay_consistency_decision
            .decision,
        activation_audit_chain_closeout_events: input.activation_audit_chain_closeout_events,
        activation_audit_chain_closeout_replay_consistency_events: input
            .activation_audit_chain_closeout_replay_consistency_events,
        prior_activation_operator_approval_readiness_preflight_packet_events: input
            .prior_activation_operator_approval_readiness_preflight_packet_events,
        activation_audit_chain_closeout_readback_ready: input
            .activation_audit_chain_closeout_readback_ready,
        activation_audit_chain_closeout_replay_consistency_ready: input
            .activation_audit_chain_closeout_replay_consistency_ready,
        activation_audit_chain_closeout_ready,
        activation_audit_chain_closeout_replay_consistent,
        activation_operator_approval_readiness_preflight_ready,
        shadow_readiness_failed: !activation_operator_approval_readiness_preflight_ready,
        activation_allowed: false,
        enablement_allowed: false,
        operator_approval_required_before_activation: true,
        operator_approval_recorded: false,
        approval_record_required_before_activation: true,
        approval_record_mutation_enabled: false,
        reviewed_flag_required_before_activation: true,
        reviewed_flag_enabled: false,
        reviewed_flag_mutation_enabled: false,
        no_live_guardrails_ready,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
        preflight_blockers,
        checks,
        recommended_next_action: "persist replay/readback consistency for this activation operator-approval readiness preflight before any approval recording, reviewed flag mutation, canary, blocking, or cutover",
    }
}

pub(crate) fn build_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision(
    input: WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyInput<
        '_,
    >,
) -> WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyDecision{
    let activation_operator_approval_readiness_preflight_packet_matches_readback = input
        .latest_activation_operator_approval_readiness_preflight_packet_payload
        == Some(input.activation_operator_approval_readiness_preflight_packet_payload);
    let preflight_keeps_activation_denied = !input
        .activation_operator_approval_readiness_preflight_packet
        .activation_allowed
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .enablement_allowed
        && input
            .activation_operator_approval_readiness_preflight_packet
            .operator_approval_required_before_activation
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .operator_approval_recorded
        && input
            .activation_operator_approval_readiness_preflight_packet
            .approval_record_required_before_activation
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .approval_record_mutation_enabled
        && input
            .activation_operator_approval_readiness_preflight_packet
            .reviewed_flag_required_before_activation
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .reviewed_flag_enabled
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .reviewed_flag_mutation_enabled;
    let canonical_projection_disabled = !input
        .activation_operator_approval_readiness_preflight_packet
        .canonical_write_enabled
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .canonical_read_enabled
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .canonical_projection_persistence_enabled;
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .feature_flag_enabled
        && input
            .activation_operator_approval_readiness_preflight_packet
            .canary_stage
            == "off"
        && input
            .activation_operator_approval_readiness_preflight_packet
            .canary_traffic_ppm
            == 0
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .live_blocking_enabled
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .live_cutover_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_readback_ready",
            passed: input.activation_operator_approval_readiness_preflight_packet_events > 0
                && input
                    .activation_operator_approval_readiness_preflight_packet_readback_ready,
            detail:
                "durable readback must include the canonical projection enablement activation operator-approval readiness preflight packet"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_operator_approval_readiness_preflight_latest_payload_matches",
            passed: activation_operator_approval_readiness_preflight_packet_matches_readback,
            detail:
                "latest durable canonical projection enablement activation operator-approval readiness preflight packet must match the tool result payload"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready",
            passed: input
                .activation_operator_approval_readiness_preflight_packet
                .activation_operator_approval_readiness_preflight_ready
                && !input
                    .activation_operator_approval_readiness_preflight_packet
                    .shadow_readiness_failed,
            detail:
                "canonical projection enablement activation operator-approval readiness preflight packet must be ready shadow-only evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_activation_audit_chain_closeout_replay_ready",
            passed: input
                .activation_operator_approval_readiness_preflight_packet
                .activation_audit_chain_closeout_ready
                && input
                    .activation_operator_approval_readiness_preflight_packet
                    .activation_audit_chain_closeout_replay_consistent,
            detail:
                "activation operator-approval readiness preflight packet must consume ready final activation closeout replay evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_activation_operator_approval_prerequisites_still_required",
            passed: preflight_keeps_activation_denied,
            detail:
                "activationAllowed, enablementAllowed, approval recording, approval mutation, reviewed flag, and reviewed flag mutation must remain disabled while operator approval, approval record, and reviewed flag stay required"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical WorkGraph write, read, and projection persistence must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_enablement_activation_operator_approval_readiness_preflight_replay_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let consistency_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let replay_consistent = consistency_blockers.is_empty();
    let decision = if replay_consistent {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_REPLAY_CONSISTENT_SHADOW
    } else {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_OPERATOR_APPROVAL_READINESS_PREFLIGHT_REPLAY_MISMATCH_SHADOW
    };

    WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyDecision {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        replay_stage:
            "canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_shadow_only",
        source_packet_job_id: input
            .activation_operator_approval_readiness_preflight_packet
            .source_packet_job_id
            .clone(),
        activation_operator_approval_readiness_preflight_decision: input
            .activation_operator_approval_readiness_preflight_packet
            .decision,
        activation_operator_approval_readiness_preflight_packet_events: input
            .activation_operator_approval_readiness_preflight_packet_events,
        prior_activation_operator_approval_readiness_preflight_replay_consistency_events: input
            .prior_activation_operator_approval_readiness_preflight_replay_consistency_events,
        activation_operator_approval_readiness_preflight_packet_readback_ready: input
            .activation_operator_approval_readiness_preflight_packet_readback_ready,
        activation_operator_approval_readiness_preflight_packet_matches_readback,
        activation_operator_approval_readiness_preflight_ready: input
            .activation_operator_approval_readiness_preflight_packet
            .activation_operator_approval_readiness_preflight_ready,
        activation_audit_chain_closeout_ready: input
            .activation_operator_approval_readiness_preflight_packet
            .activation_audit_chain_closeout_ready,
        activation_audit_chain_closeout_replay_consistent: input
            .activation_operator_approval_readiness_preflight_packet
            .activation_audit_chain_closeout_replay_consistent,
        activation_allowed: input
            .activation_operator_approval_readiness_preflight_packet
            .activation_allowed,
        enablement_allowed: input
            .activation_operator_approval_readiness_preflight_packet
            .enablement_allowed,
        operator_approval_required_before_activation: input
            .activation_operator_approval_readiness_preflight_packet
            .operator_approval_required_before_activation,
        operator_approval_recorded: input
            .activation_operator_approval_readiness_preflight_packet
            .operator_approval_recorded,
        approval_record_required_before_activation: input
            .activation_operator_approval_readiness_preflight_packet
            .approval_record_required_before_activation,
        approval_record_mutation_enabled: input
            .activation_operator_approval_readiness_preflight_packet
            .approval_record_mutation_enabled,
        reviewed_flag_required_before_activation: input
            .activation_operator_approval_readiness_preflight_packet
            .reviewed_flag_required_before_activation,
        reviewed_flag_enabled: input
            .activation_operator_approval_readiness_preflight_packet
            .reviewed_flag_enabled,
        reviewed_flag_mutation_enabled: input
            .activation_operator_approval_readiness_preflight_packet
            .reviewed_flag_mutation_enabled,
        no_live_guardrails_ready,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        replay_consistent,
        shadow_readiness_failed: !replay_consistent,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        consistency_blockers,
        checks,
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet(
    input: WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutPacketInput<
        '_,
    >,
) -> WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutPacket {
    let activation_operator_approval_readiness_preflight_ready = input
        .activation_operator_approval_readiness_preflight_packet
        .activation_operator_approval_readiness_preflight_ready
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .shadow_readiness_failed
        && input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .activation_operator_approval_readiness_preflight_ready;
    let activation_operator_approval_readiness_preflight_replay_consistent = input
        .activation_operator_approval_readiness_preflight_replay_consistency_decision
        .replay_consistent
        && !input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .shadow_readiness_failed
        && input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .activation_operator_approval_readiness_preflight_packet_matches_readback;
    let approval_review_side_effects_locked = !input
        .activation_operator_approval_readiness_preflight_packet
        .activation_allowed
        && !input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .activation_allowed
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .enablement_allowed
        && !input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .enablement_allowed
        && input
            .activation_operator_approval_readiness_preflight_packet
            .operator_approval_required_before_activation
        && input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .operator_approval_required_before_activation
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .operator_approval_recorded
        && !input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .operator_approval_recorded
        && input
            .activation_operator_approval_readiness_preflight_packet
            .approval_record_required_before_activation
        && input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .approval_record_required_before_activation
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .approval_record_mutation_enabled
        && !input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .approval_record_mutation_enabled
        && input
            .activation_operator_approval_readiness_preflight_packet
            .reviewed_flag_required_before_activation
        && input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .reviewed_flag_required_before_activation
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .reviewed_flag_enabled
        && !input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .reviewed_flag_enabled
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .reviewed_flag_mutation_enabled
        && !input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .reviewed_flag_mutation_enabled;
    let canonical_projection_disabled = !input
        .activation_operator_approval_readiness_preflight_packet
        .canonical_write_enabled
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .canonical_read_enabled
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .canonical_projection_persistence_enabled
        && !input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .canonical_write_enabled
        && !input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .canonical_read_enabled
        && !input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .canonical_projection_persistence_enabled;
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .feature_flag_enabled
        && input
            .activation_operator_approval_readiness_preflight_packet
            .canary_stage
            == "off"
        && input
            .activation_operator_approval_readiness_preflight_packet
            .canary_traffic_ppm
            == 0
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .live_blocking_enabled
        && !input
            .activation_operator_approval_readiness_preflight_packet
            .live_cutover_enabled
        && !input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .feature_flag_enabled
        && input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .canary_stage
            == "off"
        && input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .canary_traffic_ppm
            == 0
        && !input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .live_blocking_enabled
        && !input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .live_cutover_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_readback_ready",
            passed: input.activation_operator_approval_readiness_preflight_packet_events > 0
                && input
                    .activation_operator_approval_readiness_preflight_packet_readback_ready
                && activation_operator_approval_readiness_preflight_ready,
            detail:
                "durable readback must include ready activation operator-approval/readiness preflight evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_ready",
            passed: input
                .activation_operator_approval_readiness_preflight_replay_consistency_events
                > 0
                && input
                    .activation_operator_approval_readiness_preflight_replay_consistency_ready
                && activation_operator_approval_readiness_preflight_replay_consistent,
            detail:
                "activation operator-approval/readiness preflight replay must be durable and consistent"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_activation_approval_review_side_effects_locked",
            passed: approval_review_side_effects_locked,
            detail:
                "approval recording, approval record mutation, reviewed flag, and reviewed flag mutation must remain disabled while activation prerequisites stay required"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical WorkGraph write, read, and projection persistence must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_enablement_activation_approval_review_side_effect_lock_closeout_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let closeout_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let approval_review_side_effect_lock_closeout_ready = closeout_blockers.is_empty();
    let decision = if approval_review_side_effect_lock_closeout_ready {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_RECORDED_SHADOW
    } else {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_BLOCKED_SHADOW
    };

    WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutPacket {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        closeout_stage: "canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_shadow_only",
        source_packet_job_id: input
            .activation_operator_approval_readiness_preflight_packet
            .source_packet_job_id
            .clone(),
        activation_operator_approval_readiness_preflight_decision: input
            .activation_operator_approval_readiness_preflight_packet
            .decision,
        activation_operator_approval_readiness_preflight_replay_consistency_decision: input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .decision,
        activation_operator_approval_readiness_preflight_packet_events: input
            .activation_operator_approval_readiness_preflight_packet_events,
        activation_operator_approval_readiness_preflight_replay_consistency_events: input
            .activation_operator_approval_readiness_preflight_replay_consistency_events,
        prior_activation_approval_review_side_effect_lock_closeout_packet_events: input
            .prior_activation_approval_review_side_effect_lock_closeout_packet_events,
        activation_operator_approval_readiness_preflight_packet_readback_ready: input
            .activation_operator_approval_readiness_preflight_packet_readback_ready,
        activation_operator_approval_readiness_preflight_replay_consistency_ready: input
            .activation_operator_approval_readiness_preflight_replay_consistency_ready,
        activation_operator_approval_readiness_preflight_ready,
        activation_operator_approval_readiness_preflight_replay_consistent,
        activation_operator_approval_readiness_preflight_packet_matches_readback: input
            .activation_operator_approval_readiness_preflight_replay_consistency_decision
            .activation_operator_approval_readiness_preflight_packet_matches_readback,
        approval_review_side_effect_lock_closeout_ready,
        shadow_readiness_failed: !approval_review_side_effect_lock_closeout_ready,
        activation_allowed: false,
        enablement_allowed: false,
        operator_approval_required_before_activation: true,
        operator_approval_recorded: false,
        approval_record_required_before_activation: true,
        approval_record_mutation_enabled: false,
        reviewed_flag_required_before_activation: true,
        reviewed_flag_enabled: false,
        reviewed_flag_mutation_enabled: false,
        approval_review_side_effects_locked,
        no_live_guardrails_ready,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
        closeout_blockers,
        checks,
        recommended_next_action: "persist replay/readback consistency for this activation approval/review side-effect lock closeout before any approval recording, reviewed flag mutation, canonical WorkGraph write/read, canary, blocking, or cutover",
    }
}

pub(crate) fn build_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision(
    input: WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutReplayConsistencyInput<
        '_,
    >,
) -> WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutReplayConsistencyDecision{
    let closeout_packet = input.activation_approval_review_side_effect_lock_closeout_packet;
    let activation_approval_review_side_effect_lock_closeout_packet_matches_readback = input
        .latest_activation_approval_review_side_effect_lock_closeout_packet_payload
        == Some(input.activation_approval_review_side_effect_lock_closeout_packet_payload);
    let closeout_packet_ready = closeout_packet.approval_review_side_effect_lock_closeout_ready
        && !closeout_packet.shadow_readiness_failed
        && closeout_packet.activation_operator_approval_readiness_preflight_ready
        && closeout_packet.activation_operator_approval_readiness_preflight_replay_consistent;
    let approval_review_side_effects_locked = closeout_packet.approval_review_side_effects_locked
        && !closeout_packet.activation_allowed
        && !closeout_packet.enablement_allowed
        && closeout_packet.operator_approval_required_before_activation
        && !closeout_packet.operator_approval_recorded
        && closeout_packet.approval_record_required_before_activation
        && !closeout_packet.approval_record_mutation_enabled
        && closeout_packet.reviewed_flag_required_before_activation
        && !closeout_packet.reviewed_flag_enabled
        && !closeout_packet.reviewed_flag_mutation_enabled;
    let canonical_projection_disabled = !closeout_packet.canonical_write_enabled
        && !closeout_packet.canonical_read_enabled
        && !closeout_packet.canonical_projection_persistence_enabled;
    let no_live_guardrails_ready = input.live_blocking_event_count == 0
        && input.live_cutover_event_count == 0
        && closeout_packet.no_live_guardrails_ready
        && !closeout_packet.feature_flag_enabled
        && closeout_packet.canary_stage == "off"
        && closeout_packet.canary_traffic_ppm == 0
        && !closeout_packet.live_blocking_enabled
        && !closeout_packet.live_cutover_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_readback_ready",
            passed: input.activation_approval_review_side_effect_lock_closeout_packet_events > 0
                && input
                    .activation_approval_review_side_effect_lock_closeout_packet_readback_ready,
            detail:
                "durable readback must include the canonical projection enablement activation approval/review side-effect lock closeout packet"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_latest_payload_matches",
            passed: activation_approval_review_side_effect_lock_closeout_packet_matches_readback,
            detail:
                "latest durable canonical projection enablement activation approval/review side-effect lock closeout packet must match the tool result payload"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_ready",
            passed: closeout_packet_ready,
            detail:
                "approval/review side-effect lock closeout packet must be ready shadow-only evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_activation_approval_review_side_effects_locked",
            passed: approval_review_side_effects_locked,
            detail:
                "approval recording, approval record mutation, reviewed flag, and reviewed flag mutation must remain disabled while activation prerequisites stay required"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_projection_write_read_persistence_disabled",
            passed: canonical_projection_disabled,
            detail:
                "canonical WorkGraph write, read, and projection persistence must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_events",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
    ];
    let consistency_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let replay_consistent = consistency_blockers.is_empty();
    let decision = if replay_consistent {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_REPLAY_CONSISTENT_SHADOW
    } else {
        CANONICAL_PROJECTION_ENABLEMENT_ACTIVATION_APPROVAL_REVIEW_SIDE_EFFECT_LOCK_CLOSEOUT_REPLAY_MISMATCH_SHADOW
    };

    WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutReplayConsistencyDecision {
        source_surface_id: input.source_surface_id.to_string(),
        decision,
        replay_stage:
            "canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_shadow_only",
        source_packet_job_id: closeout_packet.source_packet_job_id.clone(),
        activation_approval_review_side_effect_lock_closeout_decision: closeout_packet.decision,
        activation_approval_review_side_effect_lock_closeout_packet_events: input
            .activation_approval_review_side_effect_lock_closeout_packet_events,
        prior_activation_approval_review_side_effect_lock_closeout_replay_consistency_events: input
            .prior_activation_approval_review_side_effect_lock_closeout_replay_consistency_events,
        activation_approval_review_side_effect_lock_closeout_packet_readback_ready: input
            .activation_approval_review_side_effect_lock_closeout_packet_readback_ready,
        activation_approval_review_side_effect_lock_closeout_packet_matches_readback,
        approval_review_side_effect_lock_closeout_ready: closeout_packet
            .approval_review_side_effect_lock_closeout_ready,
        activation_operator_approval_readiness_preflight_ready: closeout_packet
            .activation_operator_approval_readiness_preflight_ready,
        activation_operator_approval_readiness_preflight_replay_consistent: closeout_packet
            .activation_operator_approval_readiness_preflight_replay_consistent,
        approval_review_side_effects_locked,
        activation_allowed: closeout_packet.activation_allowed,
        enablement_allowed: closeout_packet.enablement_allowed,
        operator_approval_required_before_activation: closeout_packet
            .operator_approval_required_before_activation,
        operator_approval_recorded: closeout_packet.operator_approval_recorded,
        approval_record_required_before_activation: closeout_packet
            .approval_record_required_before_activation,
        approval_record_mutation_enabled: closeout_packet.approval_record_mutation_enabled,
        reviewed_flag_required_before_activation: closeout_packet
            .reviewed_flag_required_before_activation,
        reviewed_flag_enabled: closeout_packet.reviewed_flag_enabled,
        reviewed_flag_mutation_enabled: closeout_packet.reviewed_flag_mutation_enabled,
        no_live_guardrails_ready,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        replay_consistent,
        shadow_readiness_failed: !replay_consistent,
        live_blocking_event_count: input.live_blocking_event_count,
        live_cutover_event_count: input.live_cutover_event_count,
        consistency_blockers,
        checks,
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

fn build_work_graph_surface_audit_packet_from_parts(
    input: WorkGraphSurfaceAuditPacketPartsInput,
) -> WorkGraphSurfaceAuditPacket {
    let surface_entries = input.surface_entries;
    let audit_chain = input.audit_chain;
    let no_live_guardrails_ready = input.no_live_guardrails_ready;
    let governed_source_surface_count = default_agent_card_manifest_registry().entries().len();
    let planning_source_surface_count = surface_entries
        .iter()
        .filter(|entry| entry.family == "planning")
        .count();
    let runtime_source_surface_count = surface_entries
        .iter()
        .filter(|entry| entry.family == "hepta_runtime")
        .count();
    let observed_this_run_count = surface_entries
        .iter()
        .filter(|entry| entry.observed_this_run)
        .count();
    let durable_fact_source_count = surface_entries
        .iter()
        .filter(|entry| entry.durable_fact_source_present)
        .count();
    let canonical_write_enabled_count = surface_entries
        .iter()
        .filter(|entry| entry.canonical_work_graph_write_enabled)
        .count();
    let result_contract_gap_count = surface_entries
        .iter()
        .filter(|entry| entry.result_contract_required && !entry.result_contract_present)
        .count();
    let verifier_reducer_gap_count = surface_entries
        .iter()
        .filter(|entry| {
            entry.result_contract_required && (!entry.verifier_present || !entry.reducer_present)
        })
        .count();
    let canonical_readiness_failed = canonical_write_enabled_count == 0
        || result_contract_gap_count > 0
        || verifier_reducer_gap_count > 0;
    let audit_packet_ready = audit_chain.chain_ready && no_live_guardrails_ready;
    let mut audit_blockers = Vec::new();
    if !audit_chain.chain_ready {
        audit_blockers.push(
            "generic audit chain readback is incomplete or contains replay inconsistencies"
                .to_string(),
        );
    }
    if !no_live_guardrails_ready {
        audit_blockers.push(
            "surface audit requires feature flag, canary, live blocking, and live cutover to remain disabled"
                .to_string(),
        );
    }
    let optimization_blockers = build_optimization_blockers(
        canonical_write_enabled_count,
        result_contract_gap_count,
        verifier_reducer_gap_count,
        &surface_entries,
    );
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "generic_audit_chain_ready",
            passed: audit_chain.chain_ready,
            detail: format!(
                "{} of {} audit-chain segments are ready",
                audit_chain.ready_segment_count, audit_chain.segment_count
            ),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_guardrails",
            passed: no_live_guardrails_ready,
            detail: "surface audit does not enable feature flags, canary traffic, live blocking, or cutover".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_surface_gap_inventory",
            passed: true,
            detail: format!(
                "{result_contract_gap_count} result contract gap(s), {verifier_reducer_gap_count} verifier/reducer gap(s), and {canonical_write_enabled_count} canonical write-enabled surface(s)"
            ),
        },
        WorkGraphAdmissionShadowCheck {
            name: "surface_inventory_coverage",
            passed: surface_entries.len() >= governed_source_surface_count,
            detail: format!(
                "{} source surfaces audited, including {} governed registry surfaces",
                surface_entries.len(),
                governed_source_surface_count
            ),
        },
    ];
    let decision = if audit_packet_ready {
        SURFACE_AUDIT_RECORDED_SHADOW
    } else {
        SURFACE_AUDIT_BLOCKED_SHADOW
    };

    WorkGraphSurfaceAuditPacket {
        decision,
        audit_stage: "surface_audit_shadow_only",
        job_id: input.job_id,
        source_surface_count: surface_entries.len(),
        governed_source_surface_count,
        planning_source_surface_count,
        runtime_source_surface_count,
        observed_this_run_count,
        durable_fact_source_count,
        canonical_write_enabled_count,
        result_contract_gap_count,
        verifier_reducer_gap_count,
        canonical_readiness_failed,
        audit_packet_ready,
        surface_entries,
        audit_chain,
        audit_blockers,
        optimization_blockers,
        checks,
        recommended_next_action: "promote this packet into the operator WorkGraph matrix, then migrate bespoke receipt/replay code to the generic audit-chain primitive before any live guardrail",
        operator_review_required: true,
        operator_approval_recorded: false,
        approval_record_mutation_enabled: false,
        promotion_allowed: false,
        promotion_prohibited_reason: "surface audit is shadow-only planning evidence; reviewed flag mutation, canary, blocking, and cutover remain disabled",
        feature_flag_id: "work_graph_surface_audit_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn summarize_work_graph_surface_audit_packet(
    packet: &WorkGraphSurfaceAuditPacket,
) -> WorkGraphSurfaceAuditPacketSummary {
    let operator_matrix_rows = build_operator_matrix_rows(packet);
    let operator_matrix_ready_row_count = operator_matrix_rows
        .iter()
        .filter(|row| row.canonical_promotion_ready)
        .count();
    let operator_matrix_row_count = operator_matrix_rows.len();
    WorkGraphSurfaceAuditPacketSummary {
        decision: packet.decision,
        audit_stage: packet.audit_stage,
        job_id: packet.job_id.clone(),
        source_surface_count: packet.source_surface_count,
        governed_source_surface_count: packet.governed_source_surface_count,
        planning_source_surface_count: packet.planning_source_surface_count,
        runtime_source_surface_count: packet.runtime_source_surface_count,
        observed_this_run_count: packet.observed_this_run_count,
        durable_fact_source_count: packet.durable_fact_source_count,
        canonical_write_enabled_count: packet.canonical_write_enabled_count,
        result_contract_gap_count: packet.result_contract_gap_count,
        verifier_reducer_gap_count: packet.verifier_reducer_gap_count,
        canonical_readiness_failed: packet.canonical_readiness_failed,
        audit_packet_ready: packet.audit_packet_ready,
        audit_chain_segment_count: packet.audit_chain.segment_count,
        audit_chain_ready_segment_count: packet.audit_chain.ready_segment_count,
        audit_chain_missing_segment_ids: packet.audit_chain.missing_segment_ids.clone(),
        audit_chain_inconsistent_segment_ids: packet.audit_chain.inconsistent_segment_ids.clone(),
        audit_chain_ready: packet.audit_chain.chain_ready,
        audit_blocker_count: packet.audit_blockers.len(),
        optimization_blocker_count: packet.optimization_blockers.len(),
        optimization_blockers: packet.optimization_blockers.clone(),
        operator_matrix_row_count,
        operator_matrix_ready_row_count,
        operator_matrix_blocked_row_count: operator_matrix_row_count
            .saturating_sub(operator_matrix_ready_row_count),
        operator_matrix_rows,
        recommended_next_action: packet.recommended_next_action,
        operator_review_required: packet.operator_review_required,
        operator_approval_recorded: packet.operator_approval_recorded,
        approval_record_mutation_enabled: packet.approval_record_mutation_enabled,
        promotion_allowed: packet.promotion_allowed,
        promotion_prohibited_reason: packet.promotion_prohibited_reason,
        feature_flag_id: packet.feature_flag_id,
        feature_flag_enabled: packet.feature_flag_enabled,
        canary_stage: packet.canary_stage,
        canary_traffic_ppm: packet.canary_traffic_ppm,
        blocking_guardrail_preview: packet.blocking_guardrail_preview,
        live_blocking_enabled: packet.live_blocking_enabled,
        live_cutover_enabled: packet.live_cutover_enabled,
    }
}

pub(crate) fn build_work_graph_canonical_projection_shadow_receipt(
    summary: &WorkGraphSurfaceAuditPacketSummary,
) -> WorkGraphCanonicalProjectionShadowReceipt {
    let projection_rows = summary
        .operator_matrix_rows
        .iter()
        .map(build_canonical_projection_row)
        .collect::<Vec<_>>();
    let projected_work_node_count = projection_rows
        .iter()
        .filter(|row| row.read_projection_ready)
        .count();
    let projected_work_edge_count = projection_rows
        .iter()
        .filter(|row| {
            row.read_projection_ready && row.result_contract_ready && row.verifier_reducer_ready
        })
        .count();
    let projected_task_result_count = projection_rows
        .iter()
        .filter(|row| {
            row.read_projection_ready && row.result_contract_ready && row.verifier_reducer_ready
        })
        .count();
    let projected_timeline_event_count = projected_work_node_count;
    let no_live_guardrails_ready = !summary.feature_flag_enabled
        && summary.canary_stage == "off"
        && summary.canary_traffic_ppm == 0
        && !summary.live_blocking_enabled
        && !summary.live_cutover_enabled;
    let read_projection_ready = summary.audit_chain_ready
        && summary.operator_matrix_row_count > 0
        && projected_work_node_count > 0
        && no_live_guardrails_ready;
    let write_projection_ready = false;
    let projection_receipt_ready = read_projection_ready
        && !write_projection_ready
        && !summary.promotion_allowed
        && !summary.operator_approval_recorded;
    let mut projection_blockers = Vec::new();
    if !summary.audit_chain_ready {
        projection_blockers.push(
            "surface audit chain must be ready before canonical projection can be read".to_string(),
        );
    }
    if summary.operator_matrix_row_count == 0 {
        projection_blockers.push("operator matrix has no rows to project".to_string());
    }
    if projected_work_node_count == 0 {
        projection_blockers.push(
            "no operator matrix row has durable, auditable evidence for read projection"
                .to_string(),
        );
    }
    if !no_live_guardrails_ready {
        projection_blockers.push(
            "canonical projection requires feature flag, canary, live blocking, and cutover to stay disabled"
                .to_string(),
        );
    }
    projection_blockers.push(
        "canonical WorkGraph write/read model remains disabled; this receipt is shadow-only projection evidence"
            .to_string(),
    );
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "surface_audit_chain_ready",
            passed: summary.audit_chain_ready,
            detail: format!(
                "{} of {} surface-audit segments are ready",
                summary.audit_chain_ready_segment_count, summary.audit_chain_segment_count
            ),
        },
        WorkGraphAdmissionShadowCheck {
            name: "operator_matrix_rows_projectable",
            passed: projected_work_node_count > 0,
            detail: format!(
                "{projected_work_node_count} of {} operator matrix row(s) have durable read-projection evidence",
                summary.operator_matrix_row_count
            ),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canonical_write_disabled",
            passed: true,
            detail: "canonical WorkGraph write path remains disabled in this shadow receipt"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_projection_guardrails",
            passed: no_live_guardrails_ready,
            detail: "projection receipt does not enable feature flags, canary traffic, live blocking, or cutover".to_string(),
        },
    ];
    let decision = if projection_receipt_ready {
        CANONICAL_PROJECTION_RECORDED_SHADOW
    } else {
        CANONICAL_PROJECTION_BLOCKED_SHADOW
    };

    WorkGraphCanonicalProjectionShadowReceipt {
        decision,
        projection_stage: "canonical_work_graph_write_read_projection_shadow",
        source_packet_job_id: summary.job_id.clone(),
        source_packet_decision: summary.decision,
        source_surface_count: summary.source_surface_count,
        operator_matrix_row_count: summary.operator_matrix_row_count,
        projected_work_node_count,
        projected_work_edge_count,
        projected_task_result_count,
        projected_timeline_event_count,
        read_projection_ready,
        write_projection_ready,
        projection_receipt_ready,
        projection_blockers,
        checks,
        projection_rows,
        canonical_write_enabled: false,
        canonical_read_enabled: false,
        canonical_projection_persistence_enabled: false,
        feature_flag_enabled: false,
        canary_stage: "off",
        live_blocking_enabled: false,
        live_cutover_enabled: false,
        recommended_next_action: "add durable readback/replay for this canonical projection receipt before any canonical write or read-model cutover",
    }
}

fn build_canonical_projection_row(
    row: &WorkGraphOperatorMatrixRow,
) -> WorkGraphCanonicalProjectionRow {
    let read_projection_ready = row.row_auditable
        && row.durable_fact_source_present
        && row.replay_consistent
        && row.no_live_guardrail_ready;
    WorkGraphCanonicalProjectionRow {
        source_surface_id: row.source_surface_id.clone(),
        family: row.family,
        node_kind: canonical_projection_node_kind(row),
        row_auditable: row.row_auditable,
        durable_fact_source_present: row.durable_fact_source_present,
        result_contract_ready: row.result_contract_ready,
        verifier_reducer_ready: row.verifier_reducer_ready,
        replay_consistent: row.replay_consistent,
        no_live_guardrail_ready: row.no_live_guardrail_ready,
        read_projection_ready,
        write_projection_ready: false,
        canonical_write_enabled: false,
        next_blocker: if read_projection_ready {
            "canonical_work_graph_write_disabled"
        } else {
            row.next_blocker
        },
        next_action: row.next_action,
    }
}

fn canonical_projection_node_kind(row: &WorkGraphOperatorMatrixRow) -> &'static str {
    match row.source_surface_id.as_str() {
        "spawn_agents_on_csv" => "work_batch",
        "report_agent_job_result" => "task_result",
        "spawn_agent" | "spawn_agent_v2" => "subagent_task",
        "send_message" | "followup_task" => "handoff_edge",
        "close_agent" => "lifecycle_close",
        "wait_agent" => "wait_barrier",
        "update_plan_tool" => "plan_step",
        "plan_mode_proposed_plan" => "plan_proposal",
        "hepta_runtime_task_board" => "runtime_task",
        "hepta_runtime_worker_tasks" => "runtime_worker_task",
        "hepta_runtime_multi_agent_reducer" => "runtime_reducer",
        "hepta_runtime_scheduler_store" => "scheduler_admission",
        _ => "work_node",
    }
}

fn build_operator_matrix_rows(
    packet: &WorkGraphSurfaceAuditPacket,
) -> Vec<WorkGraphOperatorMatrixRow> {
    packet
        .surface_entries
        .iter()
        .map(|entry| build_operator_matrix_row(packet, entry))
        .collect()
}

fn build_operator_matrix_row(
    packet: &WorkGraphSurfaceAuditPacket,
    entry: &WorkGraphSurfaceAuditEntry,
) -> WorkGraphOperatorMatrixRow {
    let row_auditable = packet.audit_chain.chain_ready
        && packet.audit_chain.chain_replay_consistent
        && packet.audit_chain.no_live_guardrails_ready
        && !packet.feature_flag_enabled
        && packet.canary_stage == "off"
        && packet.canary_traffic_ppm == 0
        && !packet.live_blocking_enabled
        && !packet.live_cutover_enabled;
    let result_contract_ready = !entry.result_contract_required || entry.result_contract_present;
    let verifier_reducer_ready =
        !entry.result_contract_required || (entry.verifier_present && entry.reducer_present);
    let canonical_promotion_ready = row_auditable
        && entry.durable_fact_source_present
        && entry.canonical_work_graph_write_enabled
        && result_contract_ready
        && verifier_reducer_ready
        && entry.promotion_ready;
    let (readiness_status, next_blocker) = operator_matrix_row_readiness(
        row_auditable,
        entry.durable_fact_source_present,
        entry.canonical_work_graph_write_enabled,
        result_contract_ready,
        verifier_reducer_ready,
        entry.promotion_ready,
    );
    let include_task_result_plan = entry.result_contract_required
        || entry.task_result_contract_id != "task_result_contract_not_required";
    let (
        task_result_contract_plan_decision,
        task_result_contract_plan_ready,
        task_result_contract_id,
        terminal_delivery_surface,
        missing_task_result_contract_parts,
        task_result_contract_next_action,
        task_result_contract_next_action_count,
    ) = if include_task_result_plan {
        (
            Some(entry.task_result_contract_plan_decision.clone()),
            Some(entry.task_result_contract_plan_ready),
            Some(entry.task_result_contract_id.clone()),
            Some(entry.terminal_delivery_surface.clone()),
            entry.missing_task_result_contract_parts.clone(),
            entry.task_result_contract_next_actions.first().cloned(),
            Some(entry.task_result_contract_next_actions.len()),
        )
    } else {
        (None, None, None, None, Vec::new(), None, None)
    };

    WorkGraphOperatorMatrixRow {
        source_surface_id: entry.source_surface_id.clone(),
        family: entry.family,
        owner_lane: entry.owner_lane,
        observed_this_run: entry.observed_this_run,
        durable_fact_source_present: entry.durable_fact_source_present,
        canonical_work_graph_write_enabled: entry.canonical_work_graph_write_enabled,
        row_auditable,
        result_contract_ready,
        verifier_reducer_ready,
        promotion_ready: entry.promotion_ready,
        replay_consistent: packet.audit_chain.chain_replay_consistent,
        no_live_guardrail_ready: packet.audit_chain.no_live_guardrails_ready,
        canonical_promotion_ready,
        readiness_status,
        next_blocker,
        task_result_contract_plan_decision,
        task_result_contract_plan_ready,
        task_result_contract_id,
        terminal_delivery_surface,
        missing_task_result_contract_parts,
        task_result_contract_next_action,
        task_result_contract_next_action_count,
        next_action: entry.next_action,
    }
}

fn operator_matrix_row_readiness(
    row_auditable: bool,
    durable_fact_source_present: bool,
    canonical_work_graph_write_enabled: bool,
    result_contract_ready: bool,
    verifier_reducer_ready: bool,
    promotion_ready: bool,
) -> (&'static str, &'static str) {
    if !row_auditable {
        return (
            "blocked_audit_chain_or_no_live_guardrail_not_ready",
            "audit_chain_or_no_live_guardrail_not_ready",
        );
    }
    if !result_contract_ready {
        return ("blocked_missing_result_contract", "missing_result_contract");
    }
    if !verifier_reducer_ready {
        return (
            "blocked_missing_verifier_reducer",
            "missing_verifier_reducer",
        );
    }
    if !durable_fact_source_present {
        return (
            "blocked_missing_durable_fact_source",
            "missing_durable_fact_source",
        );
    }
    if !promotion_ready {
        return ("blocked_promotion_not_ready", "promotion_not_ready");
    }
    if !canonical_work_graph_write_enabled {
        return (
            "blocked_canonical_work_graph_write_disabled",
            "canonical_work_graph_write_disabled",
        );
    }
    (
        "ready_shadow_operator_matrix_no_live_cutover",
        "none_shadow_only",
    )
}

fn build_surface_entries(
    matrix: &WorkGraphPromotionReadinessShadowMatrix,
    role_decisions: &[WorkGraphRoleManifestShadowDecision],
) -> Vec<WorkGraphSurfaceAuditEntry> {
    let observed_surface_ids = role_decisions
        .iter()
        .map(|decision| decision.source_surface_id)
        .collect::<BTreeSet<_>>();
    let mut entries = default_agent_card_manifest_registry()
        .entries()
        .iter()
        .map(|registry_entry| {
            let role_decision = role_decisions
                .iter()
                .find(|decision| decision.source_surface_id == registry_entry.source_surface_id);
            let task_result_contract_shadow_plan = role_decision
                .map(|decision| decision.task_result_contract_shadow_plan.clone())
                .unwrap_or_else(|| {
                    build_default_task_result_contract_shadow_plan(registry_entry.manifest)
                });
            let matrix_entry = matrix
                .entries
                .iter()
                .find(|entry| entry.source_surface_id == registry_entry.source_surface_id);
            WorkGraphSurfaceAuditEntry {
                source_surface_id: registry_entry.source_surface_id.to_string(),
                family: governed_surface_family(registry_entry.source_surface_id),
                owner_lane: registry_entry.manifest.lane,
                present_in_current_head: true,
                observed_this_run: observed_surface_ids.contains(registry_entry.source_surface_id),
                durable_fact_source_present: governed_surface_durable_fact_source_present(
                    registry_entry.source_surface_id,
                ),
                canonical_work_graph_write_enabled: false,
                shadow_only: true,
                result_contract_required: registry_entry.manifest.result_contract_required,
                result_contract_present: role_decision.map_or(
                    registry_entry.manifest.result_contract_present,
                    |decision| decision.result_contract_present,
                ),
                verifier_present: role_decision
                    .map_or(registry_entry.manifest.verifier_present, |decision| {
                        decision.verifier_present
                    }),
                reducer_present: role_decision
                    .map_or(registry_entry.manifest.reducer_present, |decision| {
                        decision.reducer_present
                    }),
                role_manifest_decision: role_decision
                    .map(|decision| decision.decision.to_string())
                    .unwrap_or_else(|| "not_observed_this_run".to_string()),
                promotion_readiness_decision: matrix_entry
                    .map(|entry| entry.promotion_readiness_decision.to_string())
                    .unwrap_or_else(|| "not_observed_this_run".to_string()),
                promotion_ready: matrix_entry.is_some_and(|entry| entry.promotion_ready),
                task_result_contract_plan_decision: task_result_contract_shadow_plan
                    .decision
                    .to_string(),
                task_result_contract_plan_ready: task_result_contract_shadow_plan
                    .contract_plan_ready,
                task_result_contract_id: task_result_contract_shadow_plan
                    .task_result_contract_id
                    .to_string(),
                terminal_delivery_surface: task_result_contract_shadow_plan
                    .terminal_delivery_surface
                    .to_string(),
                missing_task_result_contract_parts: task_result_contract_shadow_plan
                    .missing_contract_parts,
                task_result_contract_next_actions: task_result_contract_shadow_plan.next_actions,
                next_action: governed_surface_next_action(registry_entry.source_surface_id),
            }
        })
        .collect::<Vec<_>>();
    entries.extend(non_governed_surface_entries());
    entries
}

fn build_audit_chain_summary(
    readback: &AgentJobWorkGraphAuditChainReadback,
) -> WorkGraphAuditChainSummary {
    let segments = readback
        .segments
        .iter()
        .map(|segment| WorkGraphAuditChainSegment {
            segment_id: segment.segment_id.clone(),
            event_type: segment.event_type.clone(),
            event_count: segment.event_count,
            latest_payload_present: segment.latest_payload.is_some(),
            latest_decision: segment.latest_decision.clone(),
            readback_ready: segment.readback_ready,
            replay_consistent: segment.replay_consistent,
            no_live_guardrail_ready: segment.no_live_guardrail_ready,
            ready: segment.ready,
        })
        .collect::<Vec<_>>();
    let ready_segment_count = segments.iter().filter(|segment| segment.ready).count();
    let missing_segment_ids = segments
        .iter()
        .filter(|segment| !segment.readback_ready)
        .map(|segment| segment.segment_id.clone())
        .collect::<Vec<_>>();
    let inconsistent_segment_ids = segments
        .iter()
        .filter(|segment| segment.readback_ready && !segment.replay_consistent)
        .map(|segment| segment.segment_id.clone())
        .collect::<Vec<_>>();

    WorkGraphAuditChainSummary {
        segment_count: segments.len(),
        ready_segment_count,
        missing_segment_ids,
        inconsistent_segment_ids,
        chain_readback_ready: readback.chain_readback_ready,
        chain_replay_consistent: readback.chain_replay_consistent,
        no_live_guardrails_ready: readback.no_live_guardrails_ready,
        chain_ready: readback.chain_ready,
        segments,
    }
}

fn build_direct_wait_surface_audit_entry(
    readback: Option<&codex_state::InterAgentWaitTaskResultReadback>,
    operator_row: Option<&WorkGraphOperatorMatrixRow>,
) -> WorkGraphSurfaceAuditEntry {
    let result_contract_present =
        readback.is_some_and(|readback| readback.task_result_delivery_ready);
    let verifier_reducer_present =
        readback.is_some_and(|readback| readback.parent_reducer_receipt_ready);
    let direct_wait_surface_audit_ready =
        readback.is_some_and(|readback| readback.direct_wait_surface_audit_ready);
    let task_result_contract_plan_ready = operator_row
        .and_then(|row| row.task_result_contract_plan_ready)
        .unwrap_or(result_contract_present && verifier_reducer_present);
    let mut missing_task_result_contract_parts = operator_row
        .map(|row| row.missing_task_result_contract_parts.clone())
        .unwrap_or_default();
    if operator_row.is_none() {
        if !result_contract_present {
            missing_task_result_contract_parts.push("task_result_delivery_shadow".to_string());
        }
        if !verifier_reducer_present {
            missing_task_result_contract_parts.push("parent_reducer_shadow_receipt".to_string());
        }
    }
    missing_task_result_contract_parts.sort();
    missing_task_result_contract_parts.dedup();
    let next_action = if direct_wait_surface_audit_ready {
        "project direct wait surface-audit evidence into canonical WorkGraph write/read projection"
    } else {
        "complete direct wait delivery, reducer, replay, and surface-audit shadow readback"
    };
    let task_result_contract_next_actions = operator_row
        .and_then(|row| row.task_result_contract_next_action.clone())
        .map_or_else(|| vec![next_action.to_string()], |action| vec![action]);

    WorkGraphSurfaceAuditEntry {
        source_surface_id: "wait_agent".to_string(),
        family: "subagent_lifecycle",
        owner_lane: "subagent_lifecycle",
        present_in_current_head: true,
        observed_this_run: readback.is_some(),
        durable_fact_source_present: readback.is_some_and(|readback| readback.readback_ready),
        canonical_work_graph_write_enabled: false,
        shadow_only: true,
        result_contract_required: true,
        result_contract_present,
        verifier_present: verifier_reducer_present,
        reducer_present: verifier_reducer_present,
        role_manifest_decision: if readback.is_some() {
            "direct_wait_task_result_readback_observed_shadow_no_live_cutover".to_string()
        } else {
            "direct_wait_task_result_readback_missing_shadow_no_live_cutover".to_string()
        },
        promotion_readiness_decision: if direct_wait_surface_audit_ready {
            "direct_wait_surface_audit_ready_shadow_no_live_cutover".to_string()
        } else {
            "direct_wait_surface_audit_blocked_shadow_no_live_cutover".to_string()
        },
        promotion_ready: direct_wait_surface_audit_ready,
        task_result_contract_plan_decision: operator_row
            .and_then(|row| row.task_result_contract_plan_decision.clone())
            .unwrap_or_else(|| {
                if task_result_contract_plan_ready {
                    "task_result_delivery_readback_ready_shadow_no_live_cutover".to_string()
                } else {
                    "task_result_delivery_readback_blocked_shadow_no_live_cutover".to_string()
                }
            }),
        task_result_contract_plan_ready,
        task_result_contract_id: operator_row
            .and_then(|row| row.task_result_contract_id.clone())
            .unwrap_or_else(|| "subagent_task_result_contract_v1".to_string()),
        terminal_delivery_surface: operator_row
            .and_then(|row| row.terminal_delivery_surface.clone())
            .unwrap_or_else(|| "wait_agent(result_required=true)".to_string()),
        missing_task_result_contract_parts,
        task_result_contract_next_actions,
        next_action,
    }
}

fn build_direct_wait_global_audit_chain_summary(
    readback: Option<&codex_state::InterAgentWaitTaskResultReadback>,
) -> WorkGraphAuditChainSummary {
    let no_live_guardrails_ready = readback.is_none_or(|readback| {
        readback.live_blocking_event_count == 0 && readback.live_cutover_event_count == 0
    });
    let segments = vec![
        direct_wait_global_audit_chain_segment(DirectWaitGlobalAuditChainSegmentInput {
            segment_id: "wait_task_result_delivery_shadow",
            event_type: "wait_task_result_delivery_shadow",
            event_count: readback
                .map(|readback| readback.task_result_delivery_shadow_events)
                .unwrap_or_default(),
            latest_payload_present: readback
                .is_some_and(|readback| readback.latest_task_result_delivery_shadow.is_some()),
            latest_decision: readback
                .map(|readback| readback.latest_task_result_delivery_decision.clone())
                .unwrap_or_else(|| "missing".to_string()),
            readback_ready: readback
                .is_some_and(|readback| readback.task_result_delivery_readback_ready),
            replay_consistent: true,
            no_live_guardrail_ready: no_live_guardrails_ready,
        }),
        direct_wait_global_audit_chain_segment(DirectWaitGlobalAuditChainSegmentInput {
            segment_id: "wait_parent_reducer_shadow_receipt",
            event_type: "wait_parent_reducer_shadow_receipt",
            event_count: readback
                .map(|readback| readback.parent_reducer_shadow_receipt_events)
                .unwrap_or_default(),
            latest_payload_present: readback
                .is_some_and(|readback| readback.latest_parent_reducer_shadow_receipt.is_some()),
            latest_decision: readback
                .map(|readback| readback.latest_parent_reducer_decision.clone())
                .unwrap_or_else(|| "missing".to_string()),
            readback_ready: readback.is_some_and(|readback| readback.parent_reducer_readback_ready),
            replay_consistent: true,
            no_live_guardrail_ready: no_live_guardrails_ready,
        }),
        direct_wait_global_audit_chain_segment(DirectWaitGlobalAuditChainSegmentInput {
            segment_id: "wait_task_result_replay_consistency",
            event_type: "wait_task_result_replay_consistency",
            event_count: readback
                .map(|readback| readback.task_result_replay_consistency_events)
                .unwrap_or_default(),
            latest_payload_present: readback
                .is_some_and(|readback| readback.latest_task_result_replay_consistency.is_some()),
            latest_decision: readback
                .map(|readback| {
                    readback
                        .latest_task_result_replay_consistency_decision
                        .clone()
                })
                .unwrap_or_else(|| "missing".to_string()),
            readback_ready: readback.is_some_and(|readback| readback.replay_consistency_ready),
            replay_consistent: readback.is_some_and(|readback| readback.replay_consistent),
            no_live_guardrail_ready: no_live_guardrails_ready,
        }),
        direct_wait_global_audit_chain_segment(DirectWaitGlobalAuditChainSegmentInput {
            segment_id: "wait_surface_audit_packet",
            event_type: "wait_surface_audit_packet",
            event_count: readback
                .map(|readback| readback.wait_surface_audit_packet_events)
                .unwrap_or_default(),
            latest_payload_present: readback
                .is_some_and(|readback| readback.latest_wait_surface_audit_packet.is_some()),
            latest_decision: readback
                .map(|readback| readback.latest_wait_surface_audit_decision.clone())
                .unwrap_or_else(|| "missing".to_string()),
            readback_ready: readback
                .is_some_and(|readback| readback.wait_surface_audit_packet_readback_ready),
            replay_consistent: readback
                .is_some_and(|readback| readback.wait_surface_audit_packet_ready),
            no_live_guardrail_ready: no_live_guardrails_ready,
        }),
        direct_wait_global_audit_chain_segment(DirectWaitGlobalAuditChainSegmentInput {
            segment_id: "wait_surface_audit_replay_consistency",
            event_type: "wait_surface_audit_replay_consistency",
            event_count: readback
                .map(|readback| readback.wait_surface_audit_replay_consistency_events)
                .unwrap_or_default(),
            latest_payload_present: readback.is_some_and(|readback| {
                readback
                    .latest_wait_surface_audit_replay_consistency
                    .is_some()
            }),
            latest_decision: readback
                .map(|readback| {
                    readback
                        .latest_wait_surface_audit_replay_consistency_decision
                        .clone()
                })
                .unwrap_or_else(|| "missing".to_string()),
            readback_ready: readback
                .is_some_and(|readback| readback.wait_surface_audit_replay_consistency_ready),
            replay_consistent: readback
                .is_some_and(|readback| readback.wait_surface_audit_replay_consistent),
            no_live_guardrail_ready: no_live_guardrails_ready,
        }),
    ];
    let ready_segment_count = segments.iter().filter(|segment| segment.ready).count();
    let missing_segment_ids = segments
        .iter()
        .filter(|segment| !segment.readback_ready)
        .map(|segment| segment.segment_id.clone())
        .collect::<Vec<_>>();
    let inconsistent_segment_ids = segments
        .iter()
        .filter(|segment| segment.readback_ready && !segment.replay_consistent)
        .map(|segment| segment.segment_id.clone())
        .collect::<Vec<_>>();
    let chain_readback_ready = segments.iter().all(|segment| segment.readback_ready);
    let chain_replay_consistent = segments.iter().all(|segment| segment.replay_consistent);
    let chain_ready = chain_readback_ready && chain_replay_consistent && no_live_guardrails_ready;

    WorkGraphAuditChainSummary {
        segment_count: segments.len(),
        ready_segment_count,
        missing_segment_ids,
        inconsistent_segment_ids,
        chain_readback_ready,
        chain_replay_consistent,
        no_live_guardrails_ready,
        chain_ready,
        segments,
    }
}

struct DirectWaitGlobalAuditChainSegmentInput<'a> {
    segment_id: &'a str,
    event_type: &'a str,
    event_count: usize,
    latest_payload_present: bool,
    latest_decision: String,
    readback_ready: bool,
    replay_consistent: bool,
    no_live_guardrail_ready: bool,
}

fn direct_wait_global_audit_chain_segment(
    input: DirectWaitGlobalAuditChainSegmentInput<'_>,
) -> WorkGraphAuditChainSegment {
    WorkGraphAuditChainSegment {
        segment_id: input.segment_id.to_string(),
        event_type: input.event_type.to_string(),
        event_count: input.event_count,
        latest_payload_present: input.latest_payload_present,
        latest_decision: input.latest_decision,
        readback_ready: input.readback_ready,
        replay_consistent: input.replay_consistent,
        no_live_guardrail_ready: input.no_live_guardrail_ready,
        ready: input.readback_ready && input.replay_consistent && input.no_live_guardrail_ready,
    }
}

fn governed_surface_family(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "spawn_agents_on_csv" | "report_agent_job_result" => "agent_jobs",
        "spawn_agent" | "spawn_agent_v2" => "subagent_spawn",
        "send_message" | "followup_task" => "subagent_handoff",
        "close_agent" | "wait_agent" => "subagent_lifecycle",
        _ => "governed_tool",
    }
}

fn governed_surface_durable_fact_source_present(source_surface_id: &str) -> bool {
    matches!(
        source_surface_id,
        "spawn_agents_on_csv" | "report_agent_job_result" | "wait_agent"
    )
}

fn governed_surface_next_action(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "spawn_agents_on_csv" | "report_agent_job_result" => {
            "project agent_jobs TaskResult events into canonical WorkGraph nodes"
        }
        "spawn_agent" | "spawn_agent_v2" => {
            "require TaskResult, verifier, reducer, and parent-close policy before promotion"
        }
        "send_message" | "followup_task" => {
            "attach durable handoff edges and input schemas to canonical WorkGraph"
        }
        "close_agent" | "wait_agent" => {
            "project lifecycle barriers and terminal waits into canonical WorkGraph events"
        }
        _ => "add canonical WorkGraph projection adapter",
    }
}

fn non_governed_surface_entries() -> Vec<WorkGraphSurfaceAuditEntry> {
    vec![
        non_governed_surface(
            "update_plan_tool",
            "planning",
            "planning",
            false,
            false,
            "project checklist steps as durable PlanStep nodes with stable ids",
        ),
        non_governed_surface(
            "plan_mode_proposed_plan",
            "planning",
            "planning",
            false,
            false,
            "project proposed plan blocks into non-mutating graph proposals",
        ),
        non_governed_surface(
            "hepta_runtime_task_board",
            "hepta_runtime",
            "runtime_task_board",
            true,
            false,
            "adapt task_board tasks, dependencies, leases, and terminal events into WorkGraph",
        ),
        non_governed_surface(
            "hepta_runtime_worker_tasks",
            "hepta_runtime",
            "runtime_worker_tasks",
            false,
            true,
            "wrap worker task results, artifacts, patches, and command evidence in TaskResultEnvelope",
        ),
        non_governed_surface(
            "hepta_runtime_multi_agent_reducer",
            "hepta_runtime",
            "runtime_multi_agent",
            false,
            true,
            "map reducer consensus and agent runtime pool evidence into canonical TaskResult reducers",
        ),
        non_governed_surface(
            "hepta_runtime_scheduler_store",
            "hepta_runtime",
            "runtime_scheduler",
            true,
            false,
            "route scheduler admission decisions through the canonical WorkGraph admission controller",
        ),
    ]
}

fn non_governed_surface(
    source_surface_id: &'static str,
    family: &'static str,
    owner_lane: &'static str,
    durable_fact_source_present: bool,
    result_contract_required: bool,
    next_action: &'static str,
) -> WorkGraphSurfaceAuditEntry {
    let missing_task_result_contract_parts = if result_contract_required {
        vec![
            "task_result_contract".to_string(),
            "verifier".to_string(),
            "reducer".to_string(),
        ]
    } else {
        Vec::new()
    };
    WorkGraphSurfaceAuditEntry {
        source_surface_id: source_surface_id.to_string(),
        family,
        owner_lane,
        present_in_current_head: true,
        observed_this_run: false,
        durable_fact_source_present,
        canonical_work_graph_write_enabled: false,
        shadow_only: true,
        result_contract_required,
        result_contract_present: false,
        verifier_present: false,
        reducer_present: false,
        role_manifest_decision: "not_governed_by_agent_card_registry".to_string(),
        promotion_readiness_decision: "not_observed_this_run".to_string(),
        promotion_ready: false,
        task_result_contract_plan_decision:
            "task_result_contract_plan_not_governed_shadow_no_live_cutover".to_string(),
        task_result_contract_plan_ready: !result_contract_required,
        task_result_contract_id: "not_governed_by_agent_card_registry".to_string(),
        terminal_delivery_surface: "canonical_work_graph_adapter_required".to_string(),
        missing_task_result_contract_parts,
        task_result_contract_next_actions: vec![next_action.to_string()],
        next_action,
    }
}

fn build_optimization_blockers(
    canonical_write_enabled_count: usize,
    result_contract_gap_count: usize,
    verifier_reducer_gap_count: usize,
    entries: &[WorkGraphSurfaceAuditEntry],
) -> Vec<String> {
    let mut blockers = Vec::new();
    if canonical_write_enabled_count == 0 {
        blockers.push(
            "no audited source surface writes canonical WorkGraph nodes yet; all entries remain shadow/projection-only"
                .to_string(),
        );
    }
    if result_contract_gap_count > 0 {
        blockers.push(format!(
            "{result_contract_gap_count} audited source surface(s) still require TaskResult contract coverage"
        ));
    }
    if verifier_reducer_gap_count > 0 {
        blockers.push(format!(
            "{verifier_reducer_gap_count} audited source surface(s) still lack verifier/reducer coverage"
        ));
    }
    blockers.extend(
        entries
            .iter()
            .filter(|entry| entry.result_contract_required && !entry.result_contract_present)
            .map(|entry| format!("{}: {}", entry.source_surface_id, entry.next_action)),
    );
    blockers
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::handlers::work_graph_admission::WorkGraphAdmissionShadowCheck;
    use crate::tools::handlers::work_graph_promotion_readiness::WorkGraphPromotionReadinessShadowMatrix;
    use serde_json::json;

    fn ready_readback() -> codex_state::AgentJobWorkGraphAuditChainReadback {
        let segments = work_graph_surface_audit_chain_segment_specs()
            .iter()
            .map(|spec| {
                let latest_decision =
                    format!("{}_recorded_shadow_no_live_cutover", spec.segment_id);
                codex_state::AgentJobWorkGraphAuditChainSegmentReadback {
                    segment_id: spec.segment_id.to_string(),
                    event_type: spec.event_type.to_string(),
                    event_count: 1,
                    latest_payload: Some(json!({
                        "decision": latest_decision,
                        "replayConsistent": true
                    })),
                    latest_decision,
                    readback_ready: true,
                    replay_consistent: true,
                    no_live_guardrail_ready: true,
                    ready: true,
                }
            })
            .collect::<Vec<_>>();
        codex_state::AgentJobWorkGraphAuditChainReadback {
            job_id: "job-1".to_string(),
            segments,
            live_blocking_event_count: 0,
            live_cutover_event_count: 0,
            chain_readback_ready: true,
            chain_replay_consistent: true,
            no_live_guardrails_ready: true,
            chain_ready: true,
        }
    }

    fn sample_matrix() -> WorkGraphPromotionReadinessShadowMatrix {
        WorkGraphPromotionReadinessShadowMatrix {
            decision: "promotion_matrix_not_ready_shadow_no_live_cutover",
            promotion_stage: "shadow_only",
            expected_source_surface_count: 8,
            observed_source_surface_count: 0,
            promotion_ready_count: 0,
            promotion_not_ready_count: 0,
            coverage_ready: false,
            all_promotion_ready: false,
            ready_source_surface_ids: Vec::new(),
            not_ready_source_surface_ids: Vec::new(),
            missing_source_surface_ids: Vec::new(),
            unexpected_source_surface_ids: Vec::new(),
            duplicate_source_surface_ids: Vec::new(),
            entries: Vec::new(),
            checks: vec![WorkGraphAdmissionShadowCheck {
                name: "sample",
                passed: true,
                detail: "sample".to_string(),
            }],
            feature_flag_id: "work_graph_promotion_readiness_matrix_shadow_only",
            feature_flag_enabled: false,
            canary_stage: "off",
            canary_traffic_ppm: 0,
            blocking_guardrail_preview: true,
            live_blocking_enabled: false,
            live_cutover_enabled: false,
        }
    }

    #[test]
    fn surface_audit_records_generic_chain_without_live_cutover() {
        let matrix = sample_matrix();
        let readback = ready_readback();

        let packet = build_work_graph_surface_audit_packet(WorkGraphSurfaceAuditPacketInput {
            job_id: "job-1",
            promotion_readiness_shadow_matrix: &matrix,
            role_manifest_shadow_decisions: &[],
            audit_chain_readback: &readback,
        });

        assert_eq!(
            packet.decision,
            "work_graph_surface_audit_recorded_shadow_no_live_cutover"
        );
        assert!(packet.audit_packet_ready);
        assert!(packet.audit_chain.chain_ready);
        assert_eq!(work_graph_surface_audit_chain_segment_specs().len(), 12);
        assert_eq!(packet.audit_chain.segment_count, 12);
        assert_eq!(packet.audit_chain.ready_segment_count, 12);
        assert_eq!(packet.governed_source_surface_count, 8);
        assert_eq!(packet.planning_source_surface_count, 2);
        assert_eq!(packet.runtime_source_surface_count, 4);
        assert_eq!(packet.canonical_write_enabled_count, 0);
        assert!(packet.canonical_readiness_failed);
        assert!(!packet.feature_flag_enabled);
        assert!(!packet.promotion_allowed);
        assert!(!packet.live_cutover_enabled);
        assert!(
            packet
                .optimization_blockers
                .iter()
                .any(|blocker| blocker.contains("canonical WorkGraph nodes"))
        );
        let summary = summarize_work_graph_surface_audit_packet(&packet);
        assert_eq!(
            summary.operator_matrix_row_count,
            packet.source_surface_count
        );
        assert_eq!(
            summary.operator_matrix_blocked_row_count,
            packet.source_surface_count
        );
        assert_eq!(summary.operator_matrix_ready_row_count, 0);
        let direct_spawn_row = summary
            .operator_matrix_rows
            .iter()
            .find(|row| row.source_surface_id == "spawn_agent_v2")
            .expect("operator matrix should include direct subagent spawn row");
        assert_eq!(
            direct_spawn_row.readiness_status,
            "blocked_missing_result_contract"
        );
        assert_eq!(direct_spawn_row.next_blocker, "missing_result_contract");
        assert!(direct_spawn_row.row_auditable);
        assert!(!direct_spawn_row.canonical_promotion_ready);
        assert_eq!(
            direct_spawn_row
                .task_result_contract_plan_decision
                .as_deref(),
            Some("task_result_contract_plan_blocked_shadow_no_live_cutover")
        );
        assert_eq!(
            direct_spawn_row.task_result_contract_plan_ready,
            Some(false)
        );
        assert_eq!(
            direct_spawn_row.task_result_contract_id.as_deref(),
            Some("subagent_task_result_contract_v1")
        );
        assert_eq!(
            direct_spawn_row.terminal_delivery_surface.as_deref(),
            Some("wait_agent(result_required=true)")
        );
        assert_eq!(
            direct_spawn_row.missing_task_result_contract_parts,
            vec![
                "task_result_contract".to_string(),
                "verifier".to_string(),
                "reducer".to_string()
            ]
        );
        assert!(
            direct_spawn_row
                .task_result_contract_next_action
                .as_deref()
                .is_some_and(|action| action.contains("TaskResultEnvelope"))
        );
        assert_eq!(
            direct_spawn_row.task_result_contract_next_action_count,
            Some(3)
        );
        let agent_jobs_row = summary
            .operator_matrix_rows
            .iter()
            .find(|row| row.source_surface_id == "spawn_agents_on_csv")
            .expect("operator matrix should include agent job worker row");
        assert_eq!(agent_jobs_row.task_result_contract_plan_ready, Some(true));
        assert_eq!(
            agent_jobs_row.task_result_contract_id.as_deref(),
            Some("agent_job_task_result_contract_v1")
        );
        let projection_receipt = build_work_graph_canonical_projection_shadow_receipt(&summary);
        assert_eq!(
            projection_receipt.decision,
            "work_graph_canonical_projection_recorded_shadow_no_live_cutover"
        );
        assert!(projection_receipt.read_projection_ready);
        assert!(!projection_receipt.write_projection_ready);
        assert!(!projection_receipt.canonical_write_enabled);
        assert_eq!(projection_receipt.projected_work_node_count, 5);
        assert!(
            projection_receipt
                .projection_rows
                .iter()
                .any(|row| row.source_surface_id == "wait_agent"
                    && row.node_kind == "wait_barrier")
        );
        let projection_receipt_payload =
            serde_json::to_value(&projection_receipt).expect("projection receipt serializes");
        let replay_decision = build_work_graph_canonical_projection_replay_consistency_decision(
            WorkGraphCanonicalProjectionReplayConsistencyInput {
                source_surface_id: "agent_jobs",
                projection_receipt: &projection_receipt,
                projection_receipt_payload: &projection_receipt_payload,
                latest_projection_receipt_payload: Some(&projection_receipt_payload),
                projection_receipt_events: 1,
                projection_receipt_readback_ready: true,
                prior_projection_replay_consistency_events: 0,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
        assert_eq!(
            replay_decision.decision,
            "work_graph_canonical_projection_replay_consistent_shadow_no_live_cutover"
        );
        assert!(replay_decision.replay_consistent);
        assert!(replay_decision.projection_receipt_matches_readback);
        assert!(!replay_decision.shadow_readiness_failed);
        let closeout_receipt = build_work_graph_canonical_projection_closeout_receipt(
            WorkGraphCanonicalProjectionCloseoutReceiptInput {
                source_surface_id: "agent_jobs",
                projection_receipt: &projection_receipt,
                replay_consistency_decision: &replay_decision,
                projection_receipt_events: 1,
                projection_replay_consistency_events: 1,
                prior_projection_closeout_receipt_events: 0,
                projection_receipt_readback_ready: true,
                projection_replay_consistency_ready: true,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
        assert_eq!(
            closeout_receipt.decision,
            "work_graph_canonical_projection_closeout_recorded_shadow_no_live_cutover"
        );
        assert!(closeout_receipt.closeout_ready);
        assert!(closeout_receipt.no_cutover_terminal_receipt);
        assert!(!closeout_receipt.shadow_readiness_failed);
        assert!(!closeout_receipt.canonical_write_enabled);
        assert!(!closeout_receipt.canonical_read_enabled);
        let closeout_receipt_payload =
            serde_json::to_value(&closeout_receipt).expect("closeout receipt serializes");
        let closeout_replay_decision =
            build_work_graph_canonical_projection_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    closeout_receipt: &closeout_receipt,
                    closeout_receipt_payload: &closeout_receipt_payload,
                    latest_closeout_receipt_payload: Some(&closeout_receipt_payload),
                    closeout_receipt_events: 1,
                    closeout_receipt_readback_ready: true,
                    prior_closeout_replay_consistency_events: 0,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            closeout_replay_decision.decision,
            "work_graph_canonical_projection_closeout_replay_consistent_shadow_no_live_cutover"
        );
        assert!(closeout_replay_decision.replay_consistent);
        assert!(closeout_replay_decision.closeout_receipt_matches_readback);
        assert!(!closeout_replay_decision.shadow_readiness_failed);
        let audit_chain_closeout_receipt =
            build_work_graph_canonical_projection_audit_chain_closeout_receipt(
                WorkGraphCanonicalProjectionAuditChainCloseoutReceiptInput {
                    source_surface_id: "agent_jobs",
                    projection_receipt: &projection_receipt,
                    projection_replay_consistency_decision: &replay_decision,
                    closeout_receipt: &closeout_receipt,
                    closeout_replay_consistency_decision: &closeout_replay_decision,
                    projection_receipt_events: 1,
                    projection_replay_consistency_events: 1,
                    closeout_receipt_events: 1,
                    closeout_replay_consistency_events: 1,
                    prior_audit_chain_closeout_receipt_events: 0,
                    projection_receipt_readback_ready: true,
                    projection_replay_consistency_ready: true,
                    closeout_receipt_readback_ready: true,
                    closeout_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            audit_chain_closeout_receipt.decision,
            "work_graph_canonical_projection_audit_chain_closeout_recorded_shadow_no_live_cutover"
        );
        assert!(audit_chain_closeout_receipt.audit_chain_closeout_ready);
        assert!(audit_chain_closeout_receipt.no_cutover_terminal_receipt);
        assert!(!audit_chain_closeout_receipt.shadow_readiness_failed);
        assert!(!audit_chain_closeout_receipt.canonical_write_enabled);
        assert!(!audit_chain_closeout_receipt.canonical_read_enabled);
        let audit_chain_closeout_receipt_payload =
            serde_json::to_value(&audit_chain_closeout_receipt)
                .expect("audit-chain closeout receipt serializes");
        let audit_chain_closeout_replay_decision =
            build_work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionAuditChainCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    audit_chain_closeout_receipt: &audit_chain_closeout_receipt,
                    audit_chain_closeout_receipt_payload: &audit_chain_closeout_receipt_payload,
                    latest_audit_chain_closeout_receipt_payload: Some(
                        &audit_chain_closeout_receipt_payload,
                    ),
                    audit_chain_closeout_receipt_events: 1,
                    audit_chain_closeout_receipt_readback_ready: true,
                    prior_audit_chain_closeout_replay_consistency_events: 0,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            audit_chain_closeout_replay_decision.decision,
            "work_graph_canonical_projection_audit_chain_closeout_replay_consistent_shadow_no_live_cutover"
        );
        assert!(audit_chain_closeout_replay_decision.replay_consistent);
        assert!(audit_chain_closeout_replay_decision.audit_chain_closeout_receipt_matches_readback);
        assert!(!audit_chain_closeout_replay_decision.shadow_readiness_failed);
        let enablement_operator_review_packet =
            build_work_graph_canonical_projection_enablement_operator_review_packet(
                WorkGraphCanonicalProjectionEnablementOperatorReviewPacketInput {
                    source_surface_id: "agent_jobs",
                    projection_receipt: &projection_receipt,
                    projection_replay_consistency_decision: &replay_decision,
                    closeout_receipt: &closeout_receipt,
                    closeout_replay_consistency_decision: &closeout_replay_decision,
                    audit_chain_closeout_receipt: &audit_chain_closeout_receipt,
                    audit_chain_closeout_replay_consistency_decision:
                        &audit_chain_closeout_replay_decision,
                    projection_receipt_events: 1,
                    projection_replay_consistency_events: 1,
                    closeout_receipt_events: 1,
                    closeout_replay_consistency_events: 1,
                    audit_chain_closeout_receipt_events: 1,
                    audit_chain_closeout_replay_consistency_events: 1,
                    prior_enablement_operator_review_packet_events: 0,
                    projection_receipt_readback_ready: true,
                    projection_replay_consistency_ready: true,
                    closeout_receipt_readback_ready: true,
                    closeout_replay_consistency_ready: true,
                    audit_chain_closeout_receipt_readback_ready: true,
                    audit_chain_closeout_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            enablement_operator_review_packet.decision,
            "work_graph_canonical_projection_enablement_operator_review_ready_shadow_no_live_cutover"
        );
        assert!(enablement_operator_review_packet.enablement_operator_review_ready);
        assert!(enablement_operator_review_packet.no_live_enablement_rehearsal_ready);
        assert!(!enablement_operator_review_packet.shadow_readiness_failed);
        assert!(!enablement_operator_review_packet.enablement_allowed);
        assert!(!enablement_operator_review_packet.operator_approval_recorded);
        assert!(!enablement_operator_review_packet.reviewed_flag_enabled);
        assert!(!enablement_operator_review_packet.canonical_write_enabled);
        assert!(!enablement_operator_review_packet.live_cutover_enabled);
        let enablement_operator_review_packet_payload =
            serde_json::to_value(&enablement_operator_review_packet)
                .expect("enablement operator-review packet serializes");
        let enablement_operator_review_replay_decision =
            build_work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    enablement_operator_review_packet: &enablement_operator_review_packet,
                    enablement_operator_review_packet_payload: &enablement_operator_review_packet_payload,
                    latest_enablement_operator_review_packet_payload: Some(
                        &enablement_operator_review_packet_payload,
                    ),
                    enablement_operator_review_packet_events: 1,
                    enablement_operator_review_packet_readback_ready: true,
                    prior_enablement_operator_review_replay_consistency_events: 0,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            enablement_operator_review_replay_decision.decision,
            "work_graph_canonical_projection_enablement_operator_review_replay_consistent_shadow_no_live_cutover"
        );
        assert!(enablement_operator_review_replay_decision.replay_consistent);
        assert!(
            enablement_operator_review_replay_decision
                .enablement_operator_review_packet_matches_readback
        );
        assert!(!enablement_operator_review_replay_decision.shadow_readiness_failed);
        assert!(!enablement_operator_review_replay_decision.enablement_allowed);
        assert!(!enablement_operator_review_replay_decision.operator_approval_recorded);
        assert!(!enablement_operator_review_replay_decision.reviewed_flag_enabled);
        let enablement_no_live_rehearsal_closeout_receipt =
            build_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt(
                WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutInput {
                    source_surface_id: "agent_jobs",
                    enablement_operator_review_packet: &enablement_operator_review_packet,
                    enablement_operator_review_replay_consistency_decision:
                        &enablement_operator_review_replay_decision,
                    enablement_operator_review_packet_events: 1,
                    enablement_operator_review_replay_consistency_events: 1,
                    prior_enablement_no_live_rehearsal_closeout_events: 0,
                    enablement_operator_review_packet_readback_ready: true,
                    enablement_operator_review_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            enablement_no_live_rehearsal_closeout_receipt.decision,
            "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_recorded_shadow_no_live_cutover"
        );
        assert!(
            enablement_no_live_rehearsal_closeout_receipt
                .no_live_enablement_rehearsal_closeout_ready
        );
        assert!(!enablement_no_live_rehearsal_closeout_receipt.shadow_readiness_failed);
        assert!(!enablement_no_live_rehearsal_closeout_receipt.enablement_allowed);
        assert!(!enablement_no_live_rehearsal_closeout_receipt.operator_approval_recorded);
        assert!(!enablement_no_live_rehearsal_closeout_receipt.reviewed_flag_enabled);
        assert!(!enablement_no_live_rehearsal_closeout_receipt.canonical_write_enabled);
        let enablement_no_live_rehearsal_closeout_receipt_payload =
            serde_json::to_value(&enablement_no_live_rehearsal_closeout_receipt)
                .expect("enablement no-live rehearsal closeout receipt serializes");
        let enablement_no_live_rehearsal_closeout_replay_decision =
            build_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    no_live_rehearsal_closeout_receipt:
                        &enablement_no_live_rehearsal_closeout_receipt,
                    no_live_rehearsal_closeout_receipt_payload:
                        &enablement_no_live_rehearsal_closeout_receipt_payload,
                    latest_no_live_rehearsal_closeout_receipt_payload: Some(
                        &enablement_no_live_rehearsal_closeout_receipt_payload,
                    ),
                    no_live_rehearsal_closeout_events: 1,
                    no_live_rehearsal_closeout_readback_ready: true,
                    prior_no_live_rehearsal_closeout_replay_consistency_events: 0,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            enablement_no_live_rehearsal_closeout_replay_decision.decision,
            "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistent_shadow_no_live_cutover"
        );
        assert!(enablement_no_live_rehearsal_closeout_replay_decision.replay_consistent);
        assert!(
            enablement_no_live_rehearsal_closeout_replay_decision
                .no_live_rehearsal_closeout_matches_readback
        );
        assert!(
            enablement_no_live_rehearsal_closeout_replay_decision
                .no_live_enablement_rehearsal_closeout_ready
        );
        assert!(!enablement_no_live_rehearsal_closeout_replay_decision.shadow_readiness_failed);
        assert!(!enablement_no_live_rehearsal_closeout_replay_decision.enablement_allowed);
        assert!(!enablement_no_live_rehearsal_closeout_replay_decision.reviewed_flag_enabled);
        let enablement_audit_chain_closeout_receipt =
            build_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt(
                WorkGraphCanonicalProjectionEnablementAuditChainCloseoutInput {
                    source_surface_id: "agent_jobs",
                    enablement_operator_review_packet: &enablement_operator_review_packet,
                    enablement_operator_review_replay_consistency_decision:
                        &enablement_operator_review_replay_decision,
                    no_live_rehearsal_closeout_receipt:
                        &enablement_no_live_rehearsal_closeout_receipt,
                    no_live_rehearsal_closeout_replay_consistency_decision:
                        &enablement_no_live_rehearsal_closeout_replay_decision,
                    enablement_operator_review_packet_events: 1,
                    enablement_operator_review_replay_consistency_events: 1,
                    no_live_rehearsal_closeout_events: 1,
                    no_live_rehearsal_closeout_replay_consistency_events: 1,
                    prior_enablement_audit_chain_closeout_events: 0,
                    enablement_operator_review_packet_readback_ready: true,
                    enablement_operator_review_replay_consistency_ready: true,
                    no_live_rehearsal_closeout_readback_ready: true,
                    no_live_rehearsal_closeout_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            enablement_audit_chain_closeout_receipt.decision,
            "work_graph_canonical_projection_enablement_audit_chain_closeout_recorded_shadow_no_live_cutover"
        );
        assert!(enablement_audit_chain_closeout_receipt.enablement_audit_chain_closeout_ready);
        assert!(!enablement_audit_chain_closeout_receipt.shadow_readiness_failed);
        assert!(!enablement_audit_chain_closeout_receipt.enablement_allowed);
        assert!(!enablement_audit_chain_closeout_receipt.operator_approval_recorded);
        assert!(!enablement_audit_chain_closeout_receipt.reviewed_flag_enabled);
        let enablement_audit_chain_closeout_receipt_payload =
            serde_json::to_value(&enablement_audit_chain_closeout_receipt)
                .expect("enablement audit-chain closeout receipt serializes");
        let enablement_audit_chain_closeout_replay_decision =
            build_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    enablement_audit_chain_closeout_receipt:
                        &enablement_audit_chain_closeout_receipt,
                    enablement_audit_chain_closeout_receipt_payload:
                        &enablement_audit_chain_closeout_receipt_payload,
                    latest_enablement_audit_chain_closeout_receipt_payload: Some(
                        &enablement_audit_chain_closeout_receipt_payload,
                    ),
                    enablement_audit_chain_closeout_events: 1,
                    enablement_audit_chain_closeout_readback_ready: true,
                    prior_enablement_audit_chain_closeout_replay_consistency_events: 0,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            enablement_audit_chain_closeout_replay_decision.decision,
            "work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistent_shadow_no_live_cutover"
        );
        assert!(enablement_audit_chain_closeout_replay_decision.replay_consistent);
        assert!(
            enablement_audit_chain_closeout_replay_decision
                .enablement_audit_chain_closeout_matches_readback
        );
        assert!(
            enablement_audit_chain_closeout_replay_decision.enablement_audit_chain_closeout_ready
        );
        assert!(!enablement_audit_chain_closeout_replay_decision.shadow_readiness_failed);
        assert!(!enablement_audit_chain_closeout_replay_decision.enablement_allowed);
        assert!(!enablement_audit_chain_closeout_replay_decision.reviewed_flag_enabled);
        let activation_precondition_packet =
            build_work_graph_canonical_projection_enablement_activation_precondition_operator_packet(
                WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacketInput {
                    source_surface_id: "agent_jobs",
                    enablement_audit_chain_closeout_receipt:
                        &enablement_audit_chain_closeout_receipt,
                    enablement_audit_chain_closeout_replay_consistency_decision:
                        &enablement_audit_chain_closeout_replay_decision,
                    enablement_audit_chain_closeout_events: 1,
                    enablement_audit_chain_closeout_replay_consistency_events: 1,
                    prior_enablement_activation_precondition_operator_packet_events: 0,
                    enablement_audit_chain_closeout_readback_ready: true,
                    enablement_audit_chain_closeout_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            activation_precondition_packet.decision,
            "work_graph_canonical_projection_enablement_activation_precondition_ready_shadow_no_live_cutover"
        );
        assert!(activation_precondition_packet.activation_precondition_ready);
        assert!(!activation_precondition_packet.activation_allowed);
        assert!(!activation_precondition_packet.shadow_readiness_failed);
        assert!(!activation_precondition_packet.enablement_allowed);
        assert!(!activation_precondition_packet.operator_approval_recorded);
        assert!(!activation_precondition_packet.reviewed_flag_enabled);
        assert!(!activation_precondition_packet.canonical_write_enabled);
        assert!(activation_precondition_packet.approval_record_required_before_activation);
        assert!(activation_precondition_packet.reviewed_flag_required_before_activation);
        let activation_precondition_packet_payload =
            serde_json::to_value(&activation_precondition_packet)
                .expect("activation precondition packet serializes");
        let activation_precondition_replay_decision =
            build_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    activation_precondition_operator_packet: &activation_precondition_packet,
                    activation_precondition_operator_packet_payload:
                        &activation_precondition_packet_payload,
                    latest_activation_precondition_operator_packet_payload: Some(
                        &activation_precondition_packet_payload,
                    ),
                    activation_precondition_operator_packet_events: 1,
                    activation_precondition_operator_packet_readback_ready: true,
                    prior_activation_precondition_replay_consistency_events: 0,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            activation_precondition_replay_decision.decision,
            "work_graph_canonical_projection_enablement_activation_precondition_replay_consistent_shadow_no_live_cutover"
        );
        assert!(activation_precondition_replay_decision.replay_consistent);
        assert!(
            activation_precondition_replay_decision
                .activation_precondition_operator_packet_matches_readback
        );
        assert!(activation_precondition_replay_decision.activation_precondition_ready);
        assert!(!activation_precondition_replay_decision.activation_allowed);
        assert!(!activation_precondition_replay_decision.shadow_readiness_failed);
        let activation_no_live_closeout_receipt =
            build_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt(
                WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutInput {
                    source_surface_id: "agent_jobs",
                    activation_precondition_operator_packet: &activation_precondition_packet,
                    activation_precondition_replay_consistency_decision:
                        &activation_precondition_replay_decision,
                    activation_precondition_operator_packet_events: 1,
                    activation_precondition_replay_consistency_events: 1,
                    prior_activation_no_live_closeout_events: 0,
                    activation_precondition_operator_packet_readback_ready: true,
                    activation_precondition_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            activation_no_live_closeout_receipt.decision,
            "work_graph_canonical_projection_enablement_activation_no_live_closeout_recorded_shadow_no_live_cutover"
        );
        assert!(activation_no_live_closeout_receipt.activation_no_live_closeout_ready);
        assert!(activation_no_live_closeout_receipt.activation_precondition_ready);
        assert!(activation_no_live_closeout_receipt.activation_precondition_replay_consistent);
        assert!(activation_no_live_closeout_receipt.no_live_enablement_rehearsal_ready);
        assert!(!activation_no_live_closeout_receipt.activation_allowed);
        assert!(!activation_no_live_closeout_receipt.enablement_allowed);
        assert!(!activation_no_live_closeout_receipt.operator_approval_recorded);
        assert!(!activation_no_live_closeout_receipt.approval_record_mutation_enabled);
        assert!(!activation_no_live_closeout_receipt.reviewed_flag_enabled);
        assert!(activation_no_live_closeout_receipt.approval_record_required_before_activation);
        assert!(activation_no_live_closeout_receipt.reviewed_flag_required_before_activation);
        assert!(!activation_no_live_closeout_receipt.canonical_write_enabled);
        assert!(!activation_no_live_closeout_receipt.canonical_read_enabled);
        assert!(!activation_no_live_closeout_receipt.canonical_projection_persistence_enabled);
        assert!(!activation_no_live_closeout_receipt.feature_flag_enabled);
        assert_eq!(activation_no_live_closeout_receipt.canary_stage, "off");
        assert_eq!(activation_no_live_closeout_receipt.canary_traffic_ppm, 0);
        assert!(!activation_no_live_closeout_receipt.live_blocking_enabled);
        assert!(!activation_no_live_closeout_receipt.live_cutover_enabled);
        assert!(
            activation_no_live_closeout_receipt
                .closeout_blockers
                .is_empty()
        );
        let activation_no_live_closeout_receipt_payload =
            serde_json::to_value(&activation_no_live_closeout_receipt)
                .expect("activation no-live closeout receipt serializes");
        let activation_no_live_closeout_replay_decision =
            build_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    activation_no_live_closeout_receipt: &activation_no_live_closeout_receipt,
                    activation_no_live_closeout_receipt_payload:
                        &activation_no_live_closeout_receipt_payload,
                    latest_activation_no_live_closeout_receipt_payload: Some(
                        &activation_no_live_closeout_receipt_payload,
                    ),
                    activation_no_live_closeout_events: 1,
                    activation_no_live_closeout_readback_ready: true,
                    prior_activation_no_live_closeout_replay_consistency_events: 0,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            activation_no_live_closeout_replay_decision.decision,
            "work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistent_shadow_no_live_cutover"
        );
        assert!(activation_no_live_closeout_replay_decision.replay_consistent);
        assert!(
            activation_no_live_closeout_replay_decision
                .activation_no_live_closeout_matches_readback
        );
        assert!(activation_no_live_closeout_replay_decision.activation_no_live_closeout_ready);
        assert!(!activation_no_live_closeout_replay_decision.activation_allowed);
        assert!(!activation_no_live_closeout_replay_decision.reviewed_flag_enabled);
        assert!(!activation_no_live_closeout_replay_decision.canonical_write_enabled);
        assert!(!activation_no_live_closeout_replay_decision.shadow_readiness_failed);
        let activation_audit_chain_closeout_receipt =
            build_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt(
                WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutInput {
                    source_surface_id: "agent_jobs",
                    activation_precondition_operator_packet: &activation_precondition_packet,
                    activation_precondition_replay_consistency_decision:
                        &activation_precondition_replay_decision,
                    activation_no_live_closeout_receipt: &activation_no_live_closeout_receipt,
                    activation_no_live_closeout_replay_consistency_decision:
                        &activation_no_live_closeout_replay_decision,
                    activation_precondition_operator_packet_events: 1,
                    activation_precondition_replay_consistency_events: 1,
                    activation_no_live_closeout_events: 1,
                    activation_no_live_closeout_replay_consistency_events: 1,
                    prior_activation_audit_chain_closeout_events: 0,
                    activation_precondition_operator_packet_readback_ready: true,
                    activation_precondition_replay_consistency_ready: true,
                    activation_no_live_closeout_readback_ready: true,
                    activation_no_live_closeout_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            activation_audit_chain_closeout_receipt.decision,
            "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_recorded_shadow_no_live_cutover"
        );
        assert!(activation_audit_chain_closeout_receipt.activation_audit_chain_closeout_ready);
        assert!(activation_audit_chain_closeout_receipt.activation_precondition_ready);
        assert!(activation_audit_chain_closeout_receipt.activation_precondition_replay_consistent);
        assert!(activation_audit_chain_closeout_receipt.activation_no_live_closeout_ready);
        assert!(
            activation_audit_chain_closeout_receipt.activation_no_live_closeout_replay_consistent
        );
        assert!(activation_audit_chain_closeout_receipt.no_live_guardrails_ready);
        assert!(!activation_audit_chain_closeout_receipt.activation_allowed);
        assert!(!activation_audit_chain_closeout_receipt.enablement_allowed);
        assert!(!activation_audit_chain_closeout_receipt.operator_approval_recorded);
        assert!(!activation_audit_chain_closeout_receipt.approval_record_mutation_enabled);
        assert!(!activation_audit_chain_closeout_receipt.reviewed_flag_enabled);
        assert!(activation_audit_chain_closeout_receipt.approval_record_required_before_activation);
        assert!(activation_audit_chain_closeout_receipt.reviewed_flag_required_before_activation);
        assert!(!activation_audit_chain_closeout_receipt.canonical_write_enabled);
        assert!(!activation_audit_chain_closeout_receipt.canonical_read_enabled);
        assert!(!activation_audit_chain_closeout_receipt.canonical_projection_persistence_enabled);
        assert!(!activation_audit_chain_closeout_receipt.live_blocking_enabled);
        assert!(!activation_audit_chain_closeout_receipt.live_cutover_enabled);
        assert!(!activation_audit_chain_closeout_receipt.shadow_readiness_failed);
        assert!(
            activation_audit_chain_closeout_receipt
                .closeout_blockers
                .is_empty()
        );
        let activation_audit_chain_closeout_payload =
            serde_json::to_value(&activation_audit_chain_closeout_receipt)
                .expect("activation audit-chain closeout receipt serializes");
        let activation_audit_chain_closeout_replay_decision =
            build_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    activation_audit_chain_closeout_receipt:
                        &activation_audit_chain_closeout_receipt,
                    activation_audit_chain_closeout_receipt_payload:
                        &activation_audit_chain_closeout_payload,
                    latest_activation_audit_chain_closeout_receipt_payload: Some(
                        &activation_audit_chain_closeout_payload,
                    ),
                    activation_audit_chain_closeout_events: 1,
                    activation_audit_chain_closeout_readback_ready: true,
                    prior_activation_audit_chain_closeout_replay_consistency_events: 0,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            activation_audit_chain_closeout_replay_decision.decision,
            "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistent_shadow_no_live_cutover"
        );
        assert!(activation_audit_chain_closeout_replay_decision.replay_consistent);
        assert!(
            activation_audit_chain_closeout_replay_decision
                .activation_audit_chain_closeout_matches_readback
        );
        assert!(
            activation_audit_chain_closeout_replay_decision.activation_audit_chain_closeout_ready
        );
        assert!(!activation_audit_chain_closeout_replay_decision.activation_allowed);
        assert!(!activation_audit_chain_closeout_replay_decision.reviewed_flag_enabled);
        assert!(!activation_audit_chain_closeout_replay_decision.canonical_write_enabled);
        assert!(!activation_audit_chain_closeout_replay_decision.shadow_readiness_failed);
        let activation_operator_approval_readiness_preflight_packet =
            build_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet(
                WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacketInput {
                    source_surface_id: "agent_jobs",
                    activation_audit_chain_closeout_receipt:
                        &activation_audit_chain_closeout_receipt,
                    activation_audit_chain_closeout_replay_consistency_decision:
                        &activation_audit_chain_closeout_replay_decision,
                    activation_audit_chain_closeout_events: 1,
                    activation_audit_chain_closeout_replay_consistency_events: 1,
                    prior_activation_operator_approval_readiness_preflight_packet_events: 0,
                    activation_audit_chain_closeout_readback_ready: true,
                    activation_audit_chain_closeout_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            activation_operator_approval_readiness_preflight_packet.decision,
            "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready_shadow_no_live_cutover"
        );
        assert!(
            activation_operator_approval_readiness_preflight_packet
                .activation_operator_approval_readiness_preflight_ready
        );
        assert!(
            activation_operator_approval_readiness_preflight_packet
                .activation_audit_chain_closeout_ready
        );
        assert!(
            activation_operator_approval_readiness_preflight_packet
                .activation_audit_chain_closeout_replay_consistent
        );
        assert!(
            activation_operator_approval_readiness_preflight_packet
                .operator_approval_required_before_activation
        );
        assert!(
            activation_operator_approval_readiness_preflight_packet
                .approval_record_required_before_activation
        );
        assert!(
            activation_operator_approval_readiness_preflight_packet
                .reviewed_flag_required_before_activation
        );
        assert!(!activation_operator_approval_readiness_preflight_packet.activation_allowed);
        assert!(
            !activation_operator_approval_readiness_preflight_packet.operator_approval_recorded
        );
        assert!(
            !activation_operator_approval_readiness_preflight_packet
                .approval_record_mutation_enabled
        );
        assert!(!activation_operator_approval_readiness_preflight_packet.reviewed_flag_enabled);
        assert!(
            !activation_operator_approval_readiness_preflight_packet.reviewed_flag_mutation_enabled
        );
        assert!(!activation_operator_approval_readiness_preflight_packet.canonical_write_enabled);
        assert!(!activation_operator_approval_readiness_preflight_packet.shadow_readiness_failed);
        assert!(
            activation_operator_approval_readiness_preflight_packet
                .preflight_blockers
                .is_empty()
        );
        let activation_operator_approval_readiness_preflight_payload =
            serde_json::to_value(&activation_operator_approval_readiness_preflight_packet)
                .expect("preflight packet should serialize");
        let activation_operator_approval_readiness_preflight_replay_decision =
            build_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    activation_operator_approval_readiness_preflight_packet:
                        &activation_operator_approval_readiness_preflight_packet,
                    activation_operator_approval_readiness_preflight_packet_payload:
                        &activation_operator_approval_readiness_preflight_payload,
                    latest_activation_operator_approval_readiness_preflight_packet_payload:
                        Some(&activation_operator_approval_readiness_preflight_payload),
                    activation_operator_approval_readiness_preflight_packet_events: 1,
                    activation_operator_approval_readiness_preflight_packet_readback_ready: true,
                    prior_activation_operator_approval_readiness_preflight_replay_consistency_events: 0,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            activation_operator_approval_readiness_preflight_replay_decision.decision,
            "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistent_shadow_no_live_cutover"
        );
        assert!(activation_operator_approval_readiness_preflight_replay_decision.replay_consistent);
        assert!(
            activation_operator_approval_readiness_preflight_replay_decision
                .activation_operator_approval_readiness_preflight_packet_matches_readback
        );
        assert!(
            activation_operator_approval_readiness_preflight_replay_decision
                .activation_operator_approval_readiness_preflight_ready
        );
        assert!(
            activation_operator_approval_readiness_preflight_replay_decision
                .operator_approval_required_before_activation
        );
        assert!(
            activation_operator_approval_readiness_preflight_replay_decision
                .reviewed_flag_required_before_activation
        );
        assert!(
            !activation_operator_approval_readiness_preflight_replay_decision.activation_allowed
        );
        assert!(
            !activation_operator_approval_readiness_preflight_replay_decision
                .approval_record_mutation_enabled
        );
        assert!(
            !activation_operator_approval_readiness_preflight_replay_decision
                .reviewed_flag_mutation_enabled
        );
        assert!(
            !activation_operator_approval_readiness_preflight_replay_decision
                .canonical_write_enabled
        );
        assert!(
            !activation_operator_approval_readiness_preflight_replay_decision
                .shadow_readiness_failed
        );
    }

    #[test]
    fn surface_audit_blocks_when_audit_chain_missing_segment() {
        let matrix = sample_matrix();
        let mut readback = ready_readback();
        let missing_segment = readback
            .segments
            .iter_mut()
            .find(|segment| segment.segment_id == "reviewed_flag_audit_chain_closeout_receipt")
            .expect("reviewed flag audit-chain closeout segment should exist");
        missing_segment.event_count = 0;
        missing_segment.latest_payload = None;
        missing_segment.latest_decision = "missing".to_string();
        missing_segment.readback_ready = false;
        missing_segment.ready = false;
        readback.chain_readback_ready = false;
        readback.chain_ready = false;

        let packet = build_work_graph_surface_audit_packet(WorkGraphSurfaceAuditPacketInput {
            job_id: "job-1",
            promotion_readiness_shadow_matrix: &matrix,
            role_manifest_shadow_decisions: &[],
            audit_chain_readback: &readback,
        });

        assert_eq!(
            packet.decision,
            "work_graph_surface_audit_blocked_shadow_no_live_cutover"
        );
        assert!(!packet.audit_packet_ready);
        assert!(!packet.audit_chain.chain_ready);
        assert!(
            packet
                .audit_chain
                .missing_segment_ids
                .contains(&"reviewed_flag_audit_chain_closeout_receipt".to_string())
        );
        let summary = summarize_work_graph_surface_audit_packet(&packet);
        assert!(
            summary
                .operator_matrix_rows
                .iter()
                .all(|row| row.next_blocker == "audit_chain_or_no_live_guardrail_not_ready")
        );
        let projection_receipt = build_work_graph_canonical_projection_shadow_receipt(&summary);
        assert_eq!(
            projection_receipt.decision,
            "work_graph_canonical_projection_blocked_shadow_no_live_cutover"
        );
        assert!(!projection_receipt.read_projection_ready);
        assert!(!projection_receipt.canonical_write_enabled);
        assert!(!packet.live_cutover_enabled);
        let projection_receipt_payload =
            serde_json::to_value(&projection_receipt).expect("projection receipt serializes");
        let replay_decision = build_work_graph_canonical_projection_replay_consistency_decision(
            WorkGraphCanonicalProjectionReplayConsistencyInput {
                source_surface_id: "agent_jobs",
                projection_receipt: &projection_receipt,
                projection_receipt_payload: &projection_receipt_payload,
                latest_projection_receipt_payload: Some(&json!({
                    "decision": "stale_projection_receipt"
                })),
                projection_receipt_events: 1,
                projection_receipt_readback_ready: true,
                prior_projection_replay_consistency_events: 1,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
        assert_eq!(
            replay_decision.decision,
            "work_graph_canonical_projection_replay_mismatch_shadow_no_live_cutover"
        );
        assert!(!replay_decision.replay_consistent);
        assert!(!replay_decision.projection_receipt_matches_readback);
        assert!(replay_decision.shadow_readiness_failed);
        let closeout_receipt = build_work_graph_canonical_projection_closeout_receipt(
            WorkGraphCanonicalProjectionCloseoutReceiptInput {
                source_surface_id: "agent_jobs",
                projection_receipt: &projection_receipt,
                replay_consistency_decision: &replay_decision,
                projection_receipt_events: 1,
                projection_replay_consistency_events: 1,
                prior_projection_closeout_receipt_events: 1,
                projection_receipt_readback_ready: true,
                projection_replay_consistency_ready: true,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
        assert_eq!(
            closeout_receipt.decision,
            "work_graph_canonical_projection_closeout_blocked_shadow_no_live_cutover"
        );
        assert!(!closeout_receipt.closeout_ready);
        assert!(closeout_receipt.shadow_readiness_failed);
        assert!(!closeout_receipt.no_cutover_terminal_receipt);
        assert!(
            closeout_receipt
                .closeout_blockers
                .iter()
                .any(|blocker| { blocker.starts_with("canonical_projection_replay_consistent") })
        );
        let closeout_receipt_payload =
            serde_json::to_value(&closeout_receipt).expect("closeout receipt serializes");
        let closeout_replay_decision =
            build_work_graph_canonical_projection_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    closeout_receipt: &closeout_receipt,
                    closeout_receipt_payload: &closeout_receipt_payload,
                    latest_closeout_receipt_payload: Some(&json!({
                        "decision": "stale_closeout_receipt"
                    })),
                    closeout_receipt_events: 1,
                    closeout_receipt_readback_ready: true,
                    prior_closeout_replay_consistency_events: 1,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            closeout_replay_decision.decision,
            "work_graph_canonical_projection_closeout_replay_mismatch_shadow_no_live_cutover"
        );
        assert!(!closeout_replay_decision.replay_consistent);
        assert!(!closeout_replay_decision.closeout_receipt_matches_readback);
        assert!(closeout_replay_decision.shadow_readiness_failed);
        let audit_chain_closeout_receipt =
            build_work_graph_canonical_projection_audit_chain_closeout_receipt(
                WorkGraphCanonicalProjectionAuditChainCloseoutReceiptInput {
                    source_surface_id: "agent_jobs",
                    projection_receipt: &projection_receipt,
                    projection_replay_consistency_decision: &replay_decision,
                    closeout_receipt: &closeout_receipt,
                    closeout_replay_consistency_decision: &closeout_replay_decision,
                    projection_receipt_events: 1,
                    projection_replay_consistency_events: 1,
                    closeout_receipt_events: 1,
                    closeout_replay_consistency_events: 1,
                    prior_audit_chain_closeout_receipt_events: 1,
                    projection_receipt_readback_ready: true,
                    projection_replay_consistency_ready: true,
                    closeout_receipt_readback_ready: true,
                    closeout_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            audit_chain_closeout_receipt.decision,
            "work_graph_canonical_projection_audit_chain_closeout_blocked_shadow_no_live_cutover"
        );
        assert!(!audit_chain_closeout_receipt.audit_chain_closeout_ready);
        assert!(!audit_chain_closeout_receipt.no_cutover_terminal_receipt);
        assert!(audit_chain_closeout_receipt.shadow_readiness_failed);
        assert!(
            audit_chain_closeout_receipt
                .audit_chain_blockers
                .iter()
                .any(|blocker| blocker
                    .starts_with("canonical_projection_closeout_replay_consistent"))
        );
        let audit_chain_closeout_receipt_payload =
            serde_json::to_value(&audit_chain_closeout_receipt)
                .expect("audit-chain closeout receipt serializes");
        let audit_chain_closeout_replay_decision =
            build_work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionAuditChainCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    audit_chain_closeout_receipt: &audit_chain_closeout_receipt,
                    audit_chain_closeout_receipt_payload: &audit_chain_closeout_receipt_payload,
                    latest_audit_chain_closeout_receipt_payload: Some(&json!({
                        "decision": "stale_audit_chain_closeout_receipt"
                    })),
                    audit_chain_closeout_receipt_events: 1,
                    audit_chain_closeout_receipt_readback_ready: true,
                    prior_audit_chain_closeout_replay_consistency_events: 1,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            audit_chain_closeout_replay_decision.decision,
            "work_graph_canonical_projection_audit_chain_closeout_replay_mismatch_shadow_no_live_cutover"
        );
        assert!(!audit_chain_closeout_replay_decision.replay_consistent);
        assert!(
            !audit_chain_closeout_replay_decision.audit_chain_closeout_receipt_matches_readback
        );
        assert!(audit_chain_closeout_replay_decision.shadow_readiness_failed);
        let enablement_operator_review_packet =
            build_work_graph_canonical_projection_enablement_operator_review_packet(
                WorkGraphCanonicalProjectionEnablementOperatorReviewPacketInput {
                    source_surface_id: "agent_jobs",
                    projection_receipt: &projection_receipt,
                    projection_replay_consistency_decision: &replay_decision,
                    closeout_receipt: &closeout_receipt,
                    closeout_replay_consistency_decision: &closeout_replay_decision,
                    audit_chain_closeout_receipt: &audit_chain_closeout_receipt,
                    audit_chain_closeout_replay_consistency_decision:
                        &audit_chain_closeout_replay_decision,
                    projection_receipt_events: 1,
                    projection_replay_consistency_events: 1,
                    closeout_receipt_events: 1,
                    closeout_replay_consistency_events: 1,
                    audit_chain_closeout_receipt_events: 1,
                    audit_chain_closeout_replay_consistency_events: 1,
                    prior_enablement_operator_review_packet_events: 1,
                    projection_receipt_readback_ready: true,
                    projection_replay_consistency_ready: true,
                    closeout_receipt_readback_ready: true,
                    closeout_replay_consistency_ready: true,
                    audit_chain_closeout_receipt_readback_ready: true,
                    audit_chain_closeout_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            enablement_operator_review_packet.decision,
            "work_graph_canonical_projection_enablement_operator_review_blocked_shadow_no_live_cutover"
        );
        assert!(!enablement_operator_review_packet.enablement_operator_review_ready);
        assert!(!enablement_operator_review_packet.no_cutover_terminal_receipt);
        assert!(enablement_operator_review_packet.shadow_readiness_failed);
        assert!(!enablement_operator_review_packet.enablement_allowed);
        assert!(
            enablement_operator_review_packet
                .enablement_blockers
                .iter()
                .any(|blocker| blocker
                    .starts_with("canonical_projection_audit_chain_closeout_replay_ready"))
        );
        let enablement_operator_review_packet_payload =
            serde_json::to_value(&enablement_operator_review_packet)
                .expect("enablement operator-review packet serializes");
        let enablement_operator_review_replay_decision =
            build_work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    enablement_operator_review_packet: &enablement_operator_review_packet,
                    enablement_operator_review_packet_payload: &enablement_operator_review_packet_payload,
                    latest_enablement_operator_review_packet_payload: Some(&json!({
                        "decision": "stale_enablement_operator_review_packet"
                    })),
                    enablement_operator_review_packet_events: 1,
                    enablement_operator_review_packet_readback_ready: true,
                    prior_enablement_operator_review_replay_consistency_events: 1,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            enablement_operator_review_replay_decision.decision,
            "work_graph_canonical_projection_enablement_operator_review_replay_mismatch_shadow_no_live_cutover"
        );
        assert!(!enablement_operator_review_replay_decision.replay_consistent);
        assert!(
            !enablement_operator_review_replay_decision
                .enablement_operator_review_packet_matches_readback
        );
        assert!(enablement_operator_review_replay_decision.shadow_readiness_failed);
        assert!(!enablement_operator_review_replay_decision.enablement_allowed);
        let enablement_no_live_rehearsal_closeout_receipt =
            build_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt(
                WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutInput {
                    source_surface_id: "agent_jobs",
                    enablement_operator_review_packet: &enablement_operator_review_packet,
                    enablement_operator_review_replay_consistency_decision:
                        &enablement_operator_review_replay_decision,
                    enablement_operator_review_packet_events: 1,
                    enablement_operator_review_replay_consistency_events: 1,
                    prior_enablement_no_live_rehearsal_closeout_events: 1,
                    enablement_operator_review_packet_readback_ready: true,
                    enablement_operator_review_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            enablement_no_live_rehearsal_closeout_receipt.decision,
            "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_blocked_shadow_no_live_cutover"
        );
        assert!(
            !enablement_no_live_rehearsal_closeout_receipt
                .no_live_enablement_rehearsal_closeout_ready
        );
        assert!(enablement_no_live_rehearsal_closeout_receipt.shadow_readiness_failed);
        assert!(!enablement_no_live_rehearsal_closeout_receipt.enablement_allowed);
        let enablement_no_live_rehearsal_closeout_receipt_payload =
            serde_json::to_value(&enablement_no_live_rehearsal_closeout_receipt)
                .expect("enablement no-live rehearsal closeout receipt serializes");
        let enablement_no_live_rehearsal_closeout_replay_decision =
            build_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    no_live_rehearsal_closeout_receipt:
                        &enablement_no_live_rehearsal_closeout_receipt,
                    no_live_rehearsal_closeout_receipt_payload:
                        &enablement_no_live_rehearsal_closeout_receipt_payload,
                    latest_no_live_rehearsal_closeout_receipt_payload: Some(&json!({
                        "decision": "stale_no_live_rehearsal_closeout_receipt"
                    })),
                    no_live_rehearsal_closeout_events: 1,
                    no_live_rehearsal_closeout_readback_ready: true,
                    prior_no_live_rehearsal_closeout_replay_consistency_events: 1,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            enablement_no_live_rehearsal_closeout_replay_decision.decision,
            "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_mismatch_shadow_no_live_cutover"
        );
        assert!(!enablement_no_live_rehearsal_closeout_replay_decision.replay_consistent);
        assert!(
            !enablement_no_live_rehearsal_closeout_replay_decision
                .no_live_rehearsal_closeout_matches_readback
        );
        assert!(enablement_no_live_rehearsal_closeout_replay_decision.shadow_readiness_failed);
        assert!(!enablement_no_live_rehearsal_closeout_replay_decision.enablement_allowed);
        let enablement_audit_chain_closeout_receipt =
            build_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt(
                WorkGraphCanonicalProjectionEnablementAuditChainCloseoutInput {
                    source_surface_id: "agent_jobs",
                    enablement_operator_review_packet: &enablement_operator_review_packet,
                    enablement_operator_review_replay_consistency_decision:
                        &enablement_operator_review_replay_decision,
                    no_live_rehearsal_closeout_receipt:
                        &enablement_no_live_rehearsal_closeout_receipt,
                    no_live_rehearsal_closeout_replay_consistency_decision:
                        &enablement_no_live_rehearsal_closeout_replay_decision,
                    enablement_operator_review_packet_events: 1,
                    enablement_operator_review_replay_consistency_events: 1,
                    no_live_rehearsal_closeout_events: 1,
                    no_live_rehearsal_closeout_replay_consistency_events: 1,
                    prior_enablement_audit_chain_closeout_events: 1,
                    enablement_operator_review_packet_readback_ready: true,
                    enablement_operator_review_replay_consistency_ready: true,
                    no_live_rehearsal_closeout_readback_ready: true,
                    no_live_rehearsal_closeout_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            enablement_audit_chain_closeout_receipt.decision,
            "work_graph_canonical_projection_enablement_audit_chain_closeout_blocked_shadow_no_live_cutover"
        );
        assert!(!enablement_audit_chain_closeout_receipt.enablement_audit_chain_closeout_ready);
        assert!(enablement_audit_chain_closeout_receipt.shadow_readiness_failed);
        assert!(
            enablement_audit_chain_closeout_receipt
                .closeout_blockers
                .iter()
                .any(|blocker| blocker.starts_with(
                    "canonical_projection_enablement_no_live_rehearsal_closeout_replay_ready"
                ))
        );
        let enablement_audit_chain_closeout_receipt_payload =
            serde_json::to_value(&enablement_audit_chain_closeout_receipt)
                .expect("enablement audit-chain closeout receipt serializes");
        let enablement_audit_chain_closeout_replay_decision =
            build_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    enablement_audit_chain_closeout_receipt:
                        &enablement_audit_chain_closeout_receipt,
                    enablement_audit_chain_closeout_receipt_payload:
                        &enablement_audit_chain_closeout_receipt_payload,
                    latest_enablement_audit_chain_closeout_receipt_payload: Some(&json!({
                        "decision": "stale_enablement_audit_chain_closeout_receipt"
                    })),
                    enablement_audit_chain_closeout_events: 1,
                    enablement_audit_chain_closeout_readback_ready: true,
                    prior_enablement_audit_chain_closeout_replay_consistency_events: 1,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            enablement_audit_chain_closeout_replay_decision.decision,
            "work_graph_canonical_projection_enablement_audit_chain_closeout_replay_mismatch_shadow_no_live_cutover"
        );
        assert!(!enablement_audit_chain_closeout_replay_decision.replay_consistent);
        assert!(
            !enablement_audit_chain_closeout_replay_decision
                .enablement_audit_chain_closeout_matches_readback
        );
        assert!(enablement_audit_chain_closeout_replay_decision.shadow_readiness_failed);
        assert!(!enablement_audit_chain_closeout_replay_decision.enablement_allowed);
        let activation_precondition_packet =
            build_work_graph_canonical_projection_enablement_activation_precondition_operator_packet(
                WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacketInput {
                    source_surface_id: "agent_jobs",
                    enablement_audit_chain_closeout_receipt:
                        &enablement_audit_chain_closeout_receipt,
                    enablement_audit_chain_closeout_replay_consistency_decision:
                        &enablement_audit_chain_closeout_replay_decision,
                    enablement_audit_chain_closeout_events: 1,
                    enablement_audit_chain_closeout_replay_consistency_events: 1,
                    prior_enablement_activation_precondition_operator_packet_events: 1,
                    enablement_audit_chain_closeout_readback_ready: true,
                    enablement_audit_chain_closeout_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            activation_precondition_packet.decision,
            "work_graph_canonical_projection_enablement_activation_precondition_blocked_shadow_no_live_cutover"
        );
        assert!(!activation_precondition_packet.activation_precondition_ready);
        assert!(activation_precondition_packet.shadow_readiness_failed);
        assert!(!activation_precondition_packet.activation_allowed);
        assert!(
            activation_precondition_packet
                .activation_blockers
                .iter()
                .any(|blocker| blocker.starts_with(
                    "canonical_projection_enablement_audit_chain_closeout_replay_consistent"
                ))
        );
        let activation_precondition_packet_payload =
            serde_json::to_value(&activation_precondition_packet)
                .expect("activation precondition packet serializes");
        let activation_precondition_replay_decision =
            build_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    activation_precondition_operator_packet: &activation_precondition_packet,
                    activation_precondition_operator_packet_payload:
                        &activation_precondition_packet_payload,
                    latest_activation_precondition_operator_packet_payload: Some(&json!({
                        "decision": "stale_activation_precondition_packet"
                    })),
                    activation_precondition_operator_packet_events: 1,
                    activation_precondition_operator_packet_readback_ready: true,
                    prior_activation_precondition_replay_consistency_events: 1,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            activation_precondition_replay_decision.decision,
            "work_graph_canonical_projection_enablement_activation_precondition_replay_mismatch_shadow_no_live_cutover"
        );
        assert!(!activation_precondition_replay_decision.replay_consistent);
        assert!(
            !activation_precondition_replay_decision
                .activation_precondition_operator_packet_matches_readback
        );
        assert!(activation_precondition_replay_decision.shadow_readiness_failed);
        assert!(!activation_precondition_replay_decision.activation_allowed);
        let activation_no_live_closeout_receipt =
            build_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt(
                WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutInput {
                    source_surface_id: "agent_jobs",
                    activation_precondition_operator_packet: &activation_precondition_packet,
                    activation_precondition_replay_consistency_decision:
                        &activation_precondition_replay_decision,
                    activation_precondition_operator_packet_events: 1,
                    activation_precondition_replay_consistency_events: 1,
                    prior_activation_no_live_closeout_events: 1,
                    activation_precondition_operator_packet_readback_ready: true,
                    activation_precondition_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            activation_no_live_closeout_receipt.decision,
            "work_graph_canonical_projection_enablement_activation_no_live_closeout_blocked_shadow_no_live_cutover"
        );
        assert!(!activation_no_live_closeout_receipt.activation_no_live_closeout_ready);
        assert!(!activation_no_live_closeout_receipt.activation_precondition_ready);
        assert!(!activation_no_live_closeout_receipt.activation_precondition_replay_consistent);
        assert!(!activation_no_live_closeout_receipt.activation_allowed);
        assert!(!activation_no_live_closeout_receipt.enablement_allowed);
        assert!(!activation_no_live_closeout_receipt.operator_approval_recorded);
        assert!(!activation_no_live_closeout_receipt.reviewed_flag_enabled);
        assert!(
            activation_no_live_closeout_receipt
                .closeout_blockers
                .iter()
                .any(|blocker| blocker.starts_with(
                    "canonical_projection_enablement_activation_precondition_replay_ready"
                ))
        );
        let activation_no_live_closeout_receipt_payload =
            serde_json::to_value(&activation_no_live_closeout_receipt)
                .expect("activation no-live closeout receipt serializes");
        let activation_no_live_closeout_replay_decision =
            build_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    activation_no_live_closeout_receipt: &activation_no_live_closeout_receipt,
                    activation_no_live_closeout_receipt_payload:
                        &activation_no_live_closeout_receipt_payload,
                    latest_activation_no_live_closeout_receipt_payload: Some(&json!({
                        "decision": "stale_activation_no_live_closeout_receipt"
                    })),
                    activation_no_live_closeout_events: 1,
                    activation_no_live_closeout_readback_ready: true,
                    prior_activation_no_live_closeout_replay_consistency_events: 1,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            activation_no_live_closeout_replay_decision.decision,
            "work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_mismatch_shadow_no_live_cutover"
        );
        assert!(!activation_no_live_closeout_replay_decision.replay_consistent);
        assert!(
            !activation_no_live_closeout_replay_decision
                .activation_no_live_closeout_matches_readback
        );
        assert!(activation_no_live_closeout_replay_decision.shadow_readiness_failed);
        assert!(!activation_no_live_closeout_replay_decision.activation_allowed);
        assert!(
            activation_no_live_closeout_replay_decision
                .consistency_blockers
                .iter()
                .any(|blocker| blocker.starts_with(
                    "canonical_projection_enablement_activation_no_live_closeout_latest_payload_matches"
                ))
        );
        let activation_audit_chain_closeout_receipt =
            build_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt(
                WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutInput {
                    source_surface_id: "agent_jobs",
                    activation_precondition_operator_packet: &activation_precondition_packet,
                    activation_precondition_replay_consistency_decision:
                        &activation_precondition_replay_decision,
                    activation_no_live_closeout_receipt: &activation_no_live_closeout_receipt,
                    activation_no_live_closeout_replay_consistency_decision:
                        &activation_no_live_closeout_replay_decision,
                    activation_precondition_operator_packet_events: 1,
                    activation_precondition_replay_consistency_events: 1,
                    activation_no_live_closeout_events: 1,
                    activation_no_live_closeout_replay_consistency_events: 1,
                    prior_activation_audit_chain_closeout_events: 1,
                    activation_precondition_operator_packet_readback_ready: true,
                    activation_precondition_replay_consistency_ready: true,
                    activation_no_live_closeout_readback_ready: true,
                    activation_no_live_closeout_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            activation_audit_chain_closeout_receipt.decision,
            "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_blocked_shadow_no_live_cutover"
        );
        assert!(!activation_audit_chain_closeout_receipt.activation_audit_chain_closeout_ready);
        assert!(!activation_audit_chain_closeout_receipt.activation_precondition_ready);
        assert!(!activation_audit_chain_closeout_receipt.activation_precondition_replay_consistent);
        assert!(!activation_audit_chain_closeout_receipt.activation_no_live_closeout_ready);
        assert!(
            !activation_audit_chain_closeout_receipt.activation_no_live_closeout_replay_consistent
        );
        assert!(!activation_audit_chain_closeout_receipt.activation_allowed);
        assert!(!activation_audit_chain_closeout_receipt.enablement_allowed);
        assert!(!activation_audit_chain_closeout_receipt.operator_approval_recorded);
        assert!(!activation_audit_chain_closeout_receipt.reviewed_flag_enabled);
        assert!(activation_audit_chain_closeout_receipt.shadow_readiness_failed);
        assert!(
            activation_audit_chain_closeout_receipt
                .closeout_blockers
                .iter()
                .any(|blocker| blocker.starts_with(
                    "canonical_projection_enablement_activation_no_live_closeout_replay_ready"
                ))
        );
        let activation_audit_chain_closeout_payload =
            serde_json::to_value(&activation_audit_chain_closeout_receipt)
                .expect("activation audit-chain closeout receipt serializes");
        let activation_audit_chain_closeout_replay_decision =
            build_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    activation_audit_chain_closeout_receipt:
                        &activation_audit_chain_closeout_receipt,
                    activation_audit_chain_closeout_receipt_payload:
                        &activation_audit_chain_closeout_payload,
                    latest_activation_audit_chain_closeout_receipt_payload: Some(&json!({
                        "decision": "stale_activation_audit_chain_closeout_receipt"
                    })),
                    activation_audit_chain_closeout_events: 1,
                    activation_audit_chain_closeout_readback_ready: true,
                    prior_activation_audit_chain_closeout_replay_consistency_events: 1,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            activation_audit_chain_closeout_replay_decision.decision,
            "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_mismatch_shadow_no_live_cutover"
        );
        assert!(!activation_audit_chain_closeout_replay_decision.replay_consistent);
        assert!(
            !activation_audit_chain_closeout_replay_decision
                .activation_audit_chain_closeout_matches_readback
        );
        assert!(activation_audit_chain_closeout_replay_decision.shadow_readiness_failed);
        assert!(!activation_audit_chain_closeout_replay_decision.activation_allowed);
        assert!(
            activation_audit_chain_closeout_replay_decision
                .consistency_blockers
                .iter()
                .any(|blocker| blocker.starts_with(
                    "canonical_projection_enablement_activation_audit_chain_closeout_latest_payload_matches"
                ))
        );
        let activation_operator_approval_readiness_preflight_packet =
            build_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet(
                WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacketInput {
                    source_surface_id: "agent_jobs",
                    activation_audit_chain_closeout_receipt:
                        &activation_audit_chain_closeout_receipt,
                    activation_audit_chain_closeout_replay_consistency_decision:
                        &activation_audit_chain_closeout_replay_decision,
                    activation_audit_chain_closeout_events: 1,
                    activation_audit_chain_closeout_replay_consistency_events: 1,
                    prior_activation_operator_approval_readiness_preflight_packet_events: 1,
                    activation_audit_chain_closeout_readback_ready: true,
                    activation_audit_chain_closeout_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            activation_operator_approval_readiness_preflight_packet.decision,
            "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_blocked_shadow_no_live_cutover"
        );
        assert!(
            !activation_operator_approval_readiness_preflight_packet
                .activation_operator_approval_readiness_preflight_ready
        );
        assert!(
            !activation_operator_approval_readiness_preflight_packet
                .activation_audit_chain_closeout_ready
        );
        assert!(
            !activation_operator_approval_readiness_preflight_packet
                .activation_audit_chain_closeout_replay_consistent
        );
        assert!(!activation_operator_approval_readiness_preflight_packet.activation_allowed);
        assert!(
            !activation_operator_approval_readiness_preflight_packet.operator_approval_recorded
        );
        assert!(!activation_operator_approval_readiness_preflight_packet.reviewed_flag_enabled);
        assert!(activation_operator_approval_readiness_preflight_packet.shadow_readiness_failed);
        assert!(
            activation_operator_approval_readiness_preflight_packet
                .preflight_blockers
                .iter()
                .any(|blocker| blocker.starts_with(
                    "canonical_projection_enablement_activation_audit_chain_closeout_replay_ready"
                ))
        );
        let activation_operator_approval_readiness_preflight_payload =
            serde_json::to_value(&activation_operator_approval_readiness_preflight_packet)
                .expect("preflight packet should serialize");
        let mismatched_activation_operator_approval_readiness_preflight_payload =
            serde_json::json!({"mismatch": true});
        let activation_operator_approval_readiness_preflight_replay_decision =
            build_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    activation_operator_approval_readiness_preflight_packet:
                        &activation_operator_approval_readiness_preflight_packet,
                    activation_operator_approval_readiness_preflight_packet_payload:
                        &activation_operator_approval_readiness_preflight_payload,
                    latest_activation_operator_approval_readiness_preflight_packet_payload:
                        Some(&mismatched_activation_operator_approval_readiness_preflight_payload),
                    activation_operator_approval_readiness_preflight_packet_events: 1,
                    activation_operator_approval_readiness_preflight_packet_readback_ready: true,
                    prior_activation_operator_approval_readiness_preflight_replay_consistency_events: 1,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
        assert_eq!(
            activation_operator_approval_readiness_preflight_replay_decision.decision,
            "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_mismatch_shadow_no_live_cutover"
        );
        assert!(
            !activation_operator_approval_readiness_preflight_replay_decision.replay_consistent
        );
        assert!(
            !activation_operator_approval_readiness_preflight_replay_decision
                .activation_operator_approval_readiness_preflight_packet_matches_readback
        );
        assert!(
            activation_operator_approval_readiness_preflight_replay_decision
                .shadow_readiness_failed
        );
        assert!(
            activation_operator_approval_readiness_preflight_replay_decision
                .consistency_blockers
                .iter()
                .any(|blocker| blocker.starts_with(
                    "canonical_projection_enablement_activation_operator_approval_readiness_preflight_latest_payload_matches"
                ))
        );
    }
}
