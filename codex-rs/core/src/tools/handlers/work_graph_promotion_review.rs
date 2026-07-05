use crate::tools::handlers::work_graph_admission::WorkGraphAdmissionShadowCheck;
use crate::tools::handlers::work_graph_promotion_readiness::WorkGraphPromotionReadinessShadowMatrix;
use serde::Serialize;

const OPERATOR_REVIEW_PACKET_READY_SHADOW: &str =
    "operator_review_packet_ready_shadow_no_live_cutover";
const OPERATOR_REVIEW_PACKET_BLOCKED_SHADOW: &str =
    "operator_review_packet_blocked_shadow_no_live_cutover";
const PROMOTION_REVIEW_REPLAY_CONSISTENT_SHADOW: &str =
    "promotion_review_replay_consistent_shadow_no_live_cutover";
const PROMOTION_REVIEW_REPLAY_MISMATCH_SHADOW: &str =
    "promotion_review_replay_mismatch_shadow_no_live_cutover";
const PROMOTION_CLOSEOUT_RECEIPT_RECORDED_SHADOW: &str =
    "promotion_closeout_receipt_recorded_shadow_no_live_cutover";
const PROMOTION_CLOSEOUT_RECEIPT_BLOCKED_SHADOW: &str =
    "promotion_closeout_receipt_blocked_shadow_no_live_cutover";
const PROMOTION_CLOSEOUT_REPLAY_CONSISTENT_SHADOW: &str =
    "promotion_closeout_replay_consistent_shadow_no_live_cutover";
const PROMOTION_CLOSEOUT_REPLAY_MISMATCH_SHADOW: &str =
    "promotion_closeout_replay_mismatch_shadow_no_live_cutover";
const PROMOTION_REVIEW_AUDIT_CHAIN_RECORDED_SHADOW: &str =
    "promotion_review_audit_chain_recorded_shadow_no_live_cutover";
const PROMOTION_REVIEW_AUDIT_CHAIN_BLOCKED_SHADOW: &str =
    "promotion_review_audit_chain_blocked_shadow_no_live_cutover";
const REVIEWED_FLAG_PRECONDITION_PLAN_RECORDED_SHADOW: &str =
    "reviewed_flag_precondition_plan_recorded_shadow_no_live_cutover";
const REVIEWED_FLAG_PRECONDITION_PLAN_BLOCKED_SHADOW: &str =
    "reviewed_flag_precondition_plan_blocked_shadow_no_live_cutover";
const REVIEWED_FLAG_PRECONDITION_PLAN_REPLAY_CONSISTENT_SHADOW: &str =
    "reviewed_flag_precondition_plan_replay_consistent_shadow_no_live_cutover";
const REVIEWED_FLAG_PRECONDITION_PLAN_REPLAY_MISMATCH_SHADOW: &str =
    "reviewed_flag_precondition_plan_replay_mismatch_shadow_no_live_cutover";
const REVIEWED_FLAG_READINESS_CLOSEOUT_RECORDED_SHADOW: &str =
    "reviewed_flag_readiness_closeout_recorded_shadow_no_live_cutover";
const REVIEWED_FLAG_READINESS_CLOSEOUT_BLOCKED_SHADOW: &str =
    "reviewed_flag_readiness_closeout_blocked_shadow_no_live_cutover";
const REVIEWED_FLAG_READINESS_CLOSEOUT_REPLAY_CONSISTENT_SHADOW: &str =
    "reviewed_flag_readiness_closeout_replay_consistent_shadow_no_live_cutover";
const REVIEWED_FLAG_READINESS_CLOSEOUT_REPLAY_MISMATCH_SHADOW: &str =
    "reviewed_flag_readiness_closeout_replay_mismatch_shadow_no_live_cutover";
const REVIEWED_FLAG_AUDIT_CHAIN_CLOSEOUT_RECORDED_SHADOW: &str =
    "reviewed_flag_audit_chain_closeout_recorded_shadow_no_live_cutover";
const REVIEWED_FLAG_AUDIT_CHAIN_CLOSEOUT_BLOCKED_SHADOW: &str =
    "reviewed_flag_audit_chain_closeout_blocked_shadow_no_live_cutover";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphOperatorReviewPromotionPacket {
    pub(crate) decision: &'static str,
    pub(crate) promotion_stage: &'static str,
    pub(crate) matrix_decision: &'static str,
    pub(crate) expected_source_surface_count: usize,
    pub(crate) observed_source_surface_count: usize,
    pub(crate) promotion_ready_count: usize,
    pub(crate) promotion_not_ready_count: usize,
    pub(crate) coverage_ready: bool,
    pub(crate) all_promotion_ready: bool,
    pub(crate) no_live_guardrails_ready: bool,
    pub(crate) review_ready: bool,
    pub(crate) operator_review_required: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) promotion_allowed: bool,
    pub(crate) promotion_prohibited_reason: &'static str,
    pub(crate) missing_source_surface_ids: Vec<String>,
    pub(crate) unexpected_source_surface_ids: Vec<String>,
    pub(crate) duplicate_source_surface_ids: Vec<String>,
    pub(crate) not_ready_source_surface_ids: Vec<String>,
    pub(crate) review_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
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
pub(crate) struct WorkGraphPromotionReviewReplayConsistencyDecision {
    pub(crate) decision: &'static str,
    pub(crate) replay_stage: &'static str,
    pub(crate) job_id: String,
    pub(crate) readback_ready: bool,
    pub(crate) admission_shadow_decision_events: usize,
    pub(crate) promotion_readiness_matrix_events: usize,
    pub(crate) operator_review_promotion_packet_events: usize,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) admission_shadow_decision_matches: bool,
    pub(crate) promotion_readiness_matrix_matches: bool,
    pub(crate) operator_review_promotion_packet_matches: bool,
    pub(crate) replay_consistent: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) consistency_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
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
pub(crate) struct WorkGraphPromotionCloseoutReceipt {
    pub(crate) decision: &'static str,
    pub(crate) receipt_stage: &'static str,
    pub(crate) job_id: String,
    pub(crate) review_packet_decision: &'static str,
    pub(crate) replay_consistency_decision: &'static str,
    pub(crate) readback_ready: bool,
    pub(crate) replay_consistency_ready: bool,
    pub(crate) review_ready: bool,
    pub(crate) review_blocked: bool,
    pub(crate) replay_consistent: bool,
    pub(crate) closeout_ready: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) reviewed_but_not_promoted: bool,
    pub(crate) no_cutover_terminal_receipt: bool,
    pub(crate) operator_review_required: bool,
    pub(crate) operator_approval_recorded: bool,
    pub(crate) approval_record_mutation_enabled: bool,
    pub(crate) promotion_allowed: bool,
    pub(crate) promotion_prohibited_reason: &'static str,
    pub(crate) review_outcome: String,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) not_ready_source_surface_ids: Vec<String>,
    pub(crate) closeout_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
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
pub(crate) struct WorkGraphPromotionCloseoutReplayConsistencyDecision {
    pub(crate) decision: &'static str,
    pub(crate) replay_stage: &'static str,
    pub(crate) job_id: String,
    pub(crate) closeout_receipt_decision: &'static str,
    pub(crate) closeout_receipt_events: usize,
    pub(crate) closeout_receipt_ready: bool,
    pub(crate) closeout_receipt_matches: bool,
    pub(crate) closeout_ready: bool,
    pub(crate) reviewed_but_not_promoted: bool,
    pub(crate) no_cutover_terminal_receipt: bool,
    pub(crate) replay_consistent: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) consistency_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
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
pub(crate) struct WorkGraphPromotionReviewAuditChainReceipt {
    pub(crate) decision: &'static str,
    pub(crate) audit_stage: &'static str,
    pub(crate) job_id: String,
    pub(crate) admission_shadow_decision_events: usize,
    pub(crate) promotion_readiness_matrix_events: usize,
    pub(crate) operator_review_promotion_packet_events: usize,
    pub(crate) promotion_review_replay_consistency_events: usize,
    pub(crate) promotion_closeout_receipt_events: usize,
    pub(crate) promotion_closeout_replay_consistency_events: usize,
    pub(crate) admission_ready: bool,
    pub(crate) promotion_readiness_matrix_ready: bool,
    pub(crate) operator_review_packet_ready: bool,
    pub(crate) promotion_review_replay_ready: bool,
    pub(crate) promotion_closeout_receipt_ready: bool,
    pub(crate) promotion_closeout_replay_ready: bool,
    pub(crate) chain_readback_ready: bool,
    pub(crate) terminal_audit_ready: bool,
    pub(crate) review_replay_consistent: bool,
    pub(crate) closeout_replay_consistent: bool,
    pub(crate) reviewed_but_not_promoted: bool,
    pub(crate) no_cutover_terminal_receipt: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) admission_shadow_decision: String,
    pub(crate) promotion_readiness_matrix_decision: String,
    pub(crate) operator_review_packet_decision: String,
    pub(crate) promotion_review_replay_consistency_decision: String,
    pub(crate) promotion_closeout_receipt_decision: String,
    pub(crate) promotion_closeout_replay_consistency_decision: String,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) audit_chain_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
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
pub(crate) struct WorkGraphReviewedFlagPreconditionPlanPacket {
    pub(crate) decision: &'static str,
    pub(crate) plan_stage: &'static str,
    pub(crate) job_id: String,
    pub(crate) audit_chain_receipt_decision: &'static str,
    pub(crate) audit_chain_receipt_events: usize,
    pub(crate) audit_chain_receipt_ready: bool,
    pub(crate) terminal_audit_ready: bool,
    pub(crate) reviewed_but_not_promoted: bool,
    pub(crate) no_cutover_terminal_receipt: bool,
    pub(crate) dry_run_plan_ready: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) missing_live_promotion_prerequisites: Vec<String>,
    pub(crate) missing_live_promotion_prerequisite_count: usize,
    pub(crate) satisfied_shadow_prerequisites: Vec<String>,
    pub(crate) satisfied_shadow_prerequisite_count: usize,
    pub(crate) plan_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) reviewed_flag_id: &'static str,
    pub(crate) reviewed_flag_mutation_dry_run: bool,
    pub(crate) reviewed_flag_mutation_enabled: bool,
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
pub(crate) struct WorkGraphReviewedFlagPreconditionPlanReplayConsistencyDecision {
    pub(crate) decision: &'static str,
    pub(crate) replay_stage: &'static str,
    pub(crate) job_id: String,
    pub(crate) reviewed_flag_precondition_plan_decision: &'static str,
    pub(crate) reviewed_flag_precondition_plan_events: usize,
    pub(crate) reviewed_flag_precondition_plan_ready: bool,
    pub(crate) reviewed_flag_precondition_plan_matches: bool,
    pub(crate) dry_run_plan_ready: bool,
    pub(crate) missing_live_promotion_prerequisite_count: usize,
    pub(crate) replay_consistent: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) consistency_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) reviewed_flag_mutation_dry_run: bool,
    pub(crate) reviewed_flag_mutation_enabled: bool,
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
pub(crate) struct WorkGraphReviewedFlagReadinessCloseoutReceipt {
    pub(crate) decision: &'static str,
    pub(crate) receipt_stage: &'static str,
    pub(crate) job_id: String,
    pub(crate) reviewed_flag_precondition_plan_decision: &'static str,
    pub(crate) reviewed_flag_precondition_plan_replay_consistency_decision: &'static str,
    pub(crate) reviewed_flag_precondition_plan_events: usize,
    pub(crate) reviewed_flag_precondition_plan_replay_consistency_events: usize,
    pub(crate) reviewed_flag_precondition_plan_ready: bool,
    pub(crate) reviewed_flag_precondition_plan_replay_consistency_ready: bool,
    pub(crate) dry_run_plan_ready: bool,
    pub(crate) replay_consistent: bool,
    pub(crate) planned_but_not_mutable: bool,
    pub(crate) terminal_closeout_ready: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) missing_live_promotion_prerequisites: Vec<String>,
    pub(crate) missing_live_promotion_prerequisite_count: usize,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) closeout_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) reviewed_flag_mutation_dry_run: bool,
    pub(crate) reviewed_flag_mutation_enabled: bool,
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
pub(crate) struct WorkGraphReviewedFlagReadinessCloseoutReplayConsistencyDecision {
    pub(crate) decision: &'static str,
    pub(crate) replay_stage: &'static str,
    pub(crate) job_id: String,
    pub(crate) reviewed_flag_readiness_closeout_decision: &'static str,
    pub(crate) reviewed_flag_readiness_closeout_receipt_events: usize,
    pub(crate) reviewed_flag_readiness_closeout_receipt_ready: bool,
    pub(crate) reviewed_flag_readiness_closeout_receipt_matches: bool,
    pub(crate) planned_but_not_mutable: bool,
    pub(crate) terminal_closeout_ready: bool,
    pub(crate) missing_live_promotion_prerequisite_count: usize,
    pub(crate) replay_consistent: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) consistency_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) reviewed_flag_mutation_dry_run: bool,
    pub(crate) reviewed_flag_mutation_enabled: bool,
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
pub(crate) struct WorkGraphReviewedFlagAuditChainCloseoutReceipt {
    pub(crate) decision: &'static str,
    pub(crate) audit_stage: &'static str,
    pub(crate) job_id: String,
    pub(crate) reviewed_flag_precondition_plan_events: usize,
    pub(crate) reviewed_flag_precondition_plan_replay_consistency_events: usize,
    pub(crate) reviewed_flag_readiness_closeout_receipt_events: usize,
    pub(crate) reviewed_flag_readiness_closeout_replay_consistency_events: usize,
    pub(crate) reviewed_flag_precondition_plan_ready: bool,
    pub(crate) reviewed_flag_precondition_plan_replay_consistency_ready: bool,
    pub(crate) reviewed_flag_readiness_closeout_receipt_ready: bool,
    pub(crate) reviewed_flag_readiness_closeout_replay_consistency_ready: bool,
    pub(crate) reviewed_flag_chain_readback_ready: bool,
    pub(crate) terminal_reviewed_flag_audit_ready: bool,
    pub(crate) dry_run_plan_ready: bool,
    pub(crate) plan_replay_consistent: bool,
    pub(crate) readiness_closeout_ready: bool,
    pub(crate) readiness_closeout_replay_consistent: bool,
    pub(crate) planned_but_not_mutable: bool,
    pub(crate) shadow_readiness_failed: bool,
    pub(crate) missing_live_promotion_prerequisite_count: usize,
    pub(crate) reviewed_flag_precondition_plan_decision: String,
    pub(crate) reviewed_flag_precondition_plan_replay_consistency_decision: String,
    pub(crate) reviewed_flag_readiness_closeout_decision: String,
    pub(crate) reviewed_flag_readiness_closeout_replay_consistency_decision: String,
    pub(crate) live_blocking_event_count: usize,
    pub(crate) live_cutover_event_count: usize,
    pub(crate) audit_chain_blockers: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) reviewed_flag_mutation_dry_run: bool,
    pub(crate) reviewed_flag_mutation_enabled: bool,
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

pub(crate) fn build_operator_review_promotion_packet(
    matrix: &WorkGraphPromotionReadinessShadowMatrix,
) -> WorkGraphOperatorReviewPromotionPacket {
    let no_live_guardrails_ready = !matrix.feature_flag_enabled
        && matrix.canary_stage == "off"
        && matrix.canary_traffic_ppm == 0
        && !matrix.live_blocking_enabled
        && !matrix.live_cutover_enabled;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "source_surface_coverage",
            passed: matrix.coverage_ready,
            detail: format!(
                "operator review requires {} observed governed source surfaces with no missing, unexpected, or duplicate entries",
                matrix.expected_source_surface_count
            ),
        },
        WorkGraphAdmissionShadowCheck {
            name: "all_source_surfaces_promotion_ready",
            passed: matrix.all_promotion_ready,
            detail: format!(
                "{} ready and {} not ready source surfaces",
                matrix.promotion_ready_count, matrix.promotion_not_ready_count
            ),
        },
        WorkGraphAdmissionShadowCheck {
            name: "aggregate_matrix_consistency",
            passed: matrix.observed_source_surface_count == matrix.entries.len()
                && matrix.promotion_ready_count + matrix.promotion_not_ready_count
                    == matrix.entries.len(),
            detail:
                "matrix observed count and ready/not-ready totals must match the entry inventory"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_guardrails",
            passed: no_live_guardrails_ready,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "separate_reviewed_flag_path_required",
            passed: true,
            detail: "promotion remains prohibited until a separate reviewed flag path exists"
                .to_string(),
        },
    ];
    let review_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let review_ready = review_blockers.is_empty();
    let decision = if review_ready {
        OPERATOR_REVIEW_PACKET_READY_SHADOW
    } else {
        OPERATOR_REVIEW_PACKET_BLOCKED_SHADOW
    };

    WorkGraphOperatorReviewPromotionPacket {
        decision,
        promotion_stage: "operator_review_shadow_only",
        matrix_decision: matrix.decision,
        expected_source_surface_count: matrix.expected_source_surface_count,
        observed_source_surface_count: matrix.observed_source_surface_count,
        promotion_ready_count: matrix.promotion_ready_count,
        promotion_not_ready_count: matrix.promotion_not_ready_count,
        coverage_ready: matrix.coverage_ready,
        all_promotion_ready: matrix.all_promotion_ready,
        no_live_guardrails_ready,
        review_ready,
        operator_review_required: true,
        operator_approval_recorded: false,
        approval_record_mutation_enabled: false,
        promotion_allowed: false,
        promotion_prohibited_reason: "operator review packet is report-only; separate reviewed flag path required",
        missing_source_surface_ids: matrix.missing_source_surface_ids.clone(),
        unexpected_source_surface_ids: matrix.unexpected_source_surface_ids.clone(),
        duplicate_source_surface_ids: matrix.duplicate_source_surface_ids.clone(),
        not_ready_source_surface_ids: matrix.not_ready_source_surface_ids.clone(),
        review_blockers,
        checks,
        feature_flag_id: "work_graph_operator_review_promotion_packet_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_promotion_review_replay_consistency_decision(
    admission_shadow_decision_payload: &serde_json::Value,
    promotion_readiness_shadow_matrix_payload: &serde_json::Value,
    operator_review_promotion_packet_payload: &serde_json::Value,
    readback: &codex_state::AgentJobWorkGraphPromotionReviewReadback,
) -> WorkGraphPromotionReviewReplayConsistencyDecision {
    let admission_shadow_decision_matches = readback.latest_admission_shadow_decision.as_ref()
        == Some(admission_shadow_decision_payload);
    let promotion_readiness_matrix_matches = readback.latest_promotion_readiness_matrix.as_ref()
        == Some(promotion_readiness_shadow_matrix_payload);
    let operator_review_promotion_packet_matches =
        readback.latest_operator_review_promotion_packet.as_ref()
            == Some(operator_review_promotion_packet_payload);
    let no_live_events =
        readback.live_blocking_event_count == 0 && readback.live_cutover_event_count == 0;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "promotion_review_readback_ready",
            passed: readback.readback_ready,
            detail:
                "durable readback must include admission, matrix, and operator review packet payloads"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "admission_shadow_decision_latest_payload_matches",
            passed: admission_shadow_decision_matches,
            detail: "latest durable admission shadow decision must match the tool result payload"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "promotion_readiness_matrix_latest_payload_matches",
            passed: promotion_readiness_matrix_matches,
            detail:
                "latest durable promotion readiness matrix must match the tool result payload"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "operator_review_packet_latest_payload_matches",
            passed: operator_review_promotion_packet_matches,
            detail:
                "latest durable operator review packet must match the tool result payload"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_replay_events",
            passed: no_live_events,
            detail:
                "replay consistency gate must observe zero live blocking and live cutover events"
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
        PROMOTION_REVIEW_REPLAY_CONSISTENT_SHADOW
    } else {
        PROMOTION_REVIEW_REPLAY_MISMATCH_SHADOW
    };

    WorkGraphPromotionReviewReplayConsistencyDecision {
        decision,
        replay_stage: "promotion_review_replay_shadow_only",
        job_id: readback.job_id.clone(),
        readback_ready: readback.readback_ready,
        admission_shadow_decision_events: readback.admission_shadow_decision_events,
        promotion_readiness_matrix_events: readback.promotion_readiness_matrix_events,
        operator_review_promotion_packet_events: readback.operator_review_promotion_packet_events,
        live_blocking_event_count: readback.live_blocking_event_count,
        live_cutover_event_count: readback.live_cutover_event_count,
        admission_shadow_decision_matches,
        promotion_readiness_matrix_matches,
        operator_review_promotion_packet_matches,
        replay_consistent,
        shadow_readiness_failed: !replay_consistent,
        consistency_blockers,
        checks,
        operator_approval_recorded: false,
        approval_record_mutation_enabled: false,
        promotion_allowed: false,
        promotion_prohibited_reason: "replay consistency is report-only; separate reviewed flag path required",
        feature_flag_id: "work_graph_promotion_review_replay_consistency_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_promotion_closeout_receipt(
    operator_review_promotion_packet: &WorkGraphOperatorReviewPromotionPacket,
    replay_consistency_decision: &WorkGraphPromotionReviewReplayConsistencyDecision,
    readback: &codex_state::AgentJobWorkGraphPromotionReviewReadback,
) -> WorkGraphPromotionCloseoutReceipt {
    let no_promotion_or_approval_mutation = !operator_review_promotion_packet.promotion_allowed
        && !operator_review_promotion_packet.operator_approval_recorded
        && !operator_review_promotion_packet.approval_record_mutation_enabled
        && !replay_consistency_decision.promotion_allowed
        && !replay_consistency_decision.operator_approval_recorded
        && !replay_consistency_decision.approval_record_mutation_enabled;
    let no_live_guardrails = !operator_review_promotion_packet.feature_flag_enabled
        && operator_review_promotion_packet.canary_stage == "off"
        && operator_review_promotion_packet.canary_traffic_ppm == 0
        && !operator_review_promotion_packet.live_blocking_enabled
        && !operator_review_promotion_packet.live_cutover_enabled
        && !replay_consistency_decision.feature_flag_enabled
        && replay_consistency_decision.canary_stage == "off"
        && replay_consistency_decision.canary_traffic_ppm == 0
        && !replay_consistency_decision.live_blocking_enabled
        && !replay_consistency_decision.live_cutover_enabled
        && readback.live_blocking_event_count == 0
        && readback.live_cutover_event_count == 0;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "operator_review_packet_present",
            passed: true,
            detail: format!(
                "operator review packet decision {} is included in closeout receipt",
                operator_review_promotion_packet.decision
            ),
        },
        WorkGraphAdmissionShadowCheck {
            name: "promotion_review_readback_ready",
            passed: readback.readback_ready,
            detail:
                "durable readback must include admission, matrix, and operator review packet payloads"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "replay_consistency_ready",
            passed: readback.replay_consistency_ready,
            detail: "durable readback must include the replay consistency gate payload"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "replay_consistent",
            passed: replay_consistency_decision.replay_consistent
                && !replay_consistency_decision.shadow_readiness_failed,
            detail: "replay consistency gate must match tool-result and durable latest payloads"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_promotion_or_approval_mutation",
            passed: no_promotion_or_approval_mutation,
            detail: "closeout receipt must not record approval, reviewed flag mutation, or promotion"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_guardrails",
            passed: no_live_guardrails,
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
    let review_blocked = !operator_review_promotion_packet.review_ready;
    let decision = if closeout_ready {
        PROMOTION_CLOSEOUT_RECEIPT_RECORDED_SHADOW
    } else {
        PROMOTION_CLOSEOUT_RECEIPT_BLOCKED_SHADOW
    };
    let review_outcome = if operator_review_promotion_packet.review_ready {
        "operator_review_ready_no_cutover"
    } else {
        "operator_review_blocked_no_cutover"
    }
    .to_string();

    WorkGraphPromotionCloseoutReceipt {
        decision,
        receipt_stage: "terminal_no_cutover_promotion_closeout_shadow_only",
        job_id: readback.job_id.clone(),
        review_packet_decision: operator_review_promotion_packet.decision,
        replay_consistency_decision: replay_consistency_decision.decision,
        readback_ready: readback.readback_ready,
        replay_consistency_ready: readback.replay_consistency_ready,
        review_ready: operator_review_promotion_packet.review_ready,
        review_blocked,
        replay_consistent: replay_consistency_decision.replay_consistent,
        closeout_ready,
        shadow_readiness_failed: !closeout_ready,
        reviewed_but_not_promoted: closeout_ready,
        no_cutover_terminal_receipt: closeout_ready,
        operator_review_required: operator_review_promotion_packet.operator_review_required,
        operator_approval_recorded: false,
        approval_record_mutation_enabled: false,
        promotion_allowed: false,
        promotion_prohibited_reason: "terminal no-cutover closeout receipt is audit-only; separate reviewed flag path required",
        review_outcome,
        live_blocking_event_count: readback.live_blocking_event_count,
        live_cutover_event_count: readback.live_cutover_event_count,
        not_ready_source_surface_ids: operator_review_promotion_packet
            .not_ready_source_surface_ids
            .clone(),
        closeout_blockers,
        checks,
        feature_flag_id: "work_graph_promotion_closeout_receipt_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_promotion_closeout_replay_consistency_decision(
    promotion_closeout_receipt: &WorkGraphPromotionCloseoutReceipt,
    promotion_closeout_receipt_payload: &serde_json::Value,
    readback: &codex_state::AgentJobWorkGraphPromotionReviewReadback,
) -> WorkGraphPromotionCloseoutReplayConsistencyDecision {
    let closeout_receipt_matches = readback.latest_promotion_closeout_receipt.as_ref()
        == Some(promotion_closeout_receipt_payload);
    let no_promotion_or_approval_mutation = !promotion_closeout_receipt.promotion_allowed
        && !promotion_closeout_receipt.operator_approval_recorded
        && !promotion_closeout_receipt.approval_record_mutation_enabled;
    let no_live_events =
        readback.live_blocking_event_count == 0 && readback.live_cutover_event_count == 0;
    let no_live_guardrails = !promotion_closeout_receipt.feature_flag_enabled
        && promotion_closeout_receipt.canary_stage == "off"
        && promotion_closeout_receipt.canary_traffic_ppm == 0
        && !promotion_closeout_receipt.live_blocking_enabled
        && !promotion_closeout_receipt.live_cutover_enabled
        && no_live_events;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "promotion_closeout_receipt_ready",
            passed: readback.closeout_receipt_ready,
            detail: "durable readback must include the terminal no-cutover closeout receipt"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "promotion_closeout_latest_payload_matches",
            passed: closeout_receipt_matches,
            detail: "latest durable closeout receipt must match the tool result payload"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "promotion_closeout_ready_no_cutover",
            passed: promotion_closeout_receipt.closeout_ready
                && promotion_closeout_receipt.reviewed_but_not_promoted
                && promotion_closeout_receipt.no_cutover_terminal_receipt,
            detail: "closeout receipt must be terminal no-cutover evidence".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_promotion_or_approval_mutation",
            passed: no_promotion_or_approval_mutation,
            detail: "closeout replay gate must not record approval, reviewed flag mutation, or promotion"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_closeout_replay_events",
            passed: no_live_guardrails,
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
        PROMOTION_CLOSEOUT_REPLAY_CONSISTENT_SHADOW
    } else {
        PROMOTION_CLOSEOUT_REPLAY_MISMATCH_SHADOW
    };

    WorkGraphPromotionCloseoutReplayConsistencyDecision {
        decision,
        replay_stage: "promotion_closeout_replay_shadow_only",
        job_id: readback.job_id.clone(),
        closeout_receipt_decision: promotion_closeout_receipt.decision,
        closeout_receipt_events: readback.promotion_closeout_receipt_events,
        closeout_receipt_ready: readback.closeout_receipt_ready,
        closeout_receipt_matches,
        closeout_ready: promotion_closeout_receipt.closeout_ready,
        reviewed_but_not_promoted: promotion_closeout_receipt.reviewed_but_not_promoted,
        no_cutover_terminal_receipt: promotion_closeout_receipt.no_cutover_terminal_receipt,
        replay_consistent,
        shadow_readiness_failed: !replay_consistent,
        live_blocking_event_count: readback.live_blocking_event_count,
        live_cutover_event_count: readback.live_cutover_event_count,
        consistency_blockers,
        checks,
        operator_approval_recorded: false,
        approval_record_mutation_enabled: false,
        promotion_allowed: false,
        promotion_prohibited_reason: "closeout replay consistency is report-only; separate reviewed flag path required",
        feature_flag_id: "work_graph_promotion_closeout_replay_consistency_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_promotion_review_audit_chain_receipt(
    closeout_replay_consistency_decision: &WorkGraphPromotionCloseoutReplayConsistencyDecision,
    readback: &codex_state::AgentJobWorkGraphPromotionReviewReadback,
) -> WorkGraphPromotionReviewAuditChainReceipt {
    let admission_ready = readback.admission_shadow_decision_events > 0
        && readback.latest_admission_shadow_decision.is_some();
    let promotion_readiness_matrix_ready = readback.promotion_readiness_matrix_events > 0
        && readback.latest_promotion_readiness_matrix.is_some();
    let operator_review_packet_ready = readback.operator_review_promotion_packet_events > 0
        && readback.latest_operator_review_promotion_packet.is_some();
    let promotion_review_replay_ready = readback.replay_consistency_ready
        && readback
            .latest_promotion_review_replay_consistency
            .is_some();
    let promotion_closeout_receipt_ready =
        readback.closeout_receipt_ready && readback.latest_promotion_closeout_receipt.is_some();
    let promotion_closeout_replay_ready = readback.closeout_replay_consistency_ready
        && readback
            .latest_promotion_closeout_replay_consistency
            .is_some();
    let review_replay_consistent = readback
        .latest_promotion_review_replay_consistency
        .as_ref()
        .and_then(|payload| payload.get("replayConsistent"))
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let closeout_replay_consistent = closeout_replay_consistency_decision.replay_consistent
        && !closeout_replay_consistency_decision.shadow_readiness_failed;
    let reviewed_but_not_promoted = closeout_replay_consistency_decision.reviewed_but_not_promoted
        && !closeout_replay_consistency_decision.promotion_allowed;
    let no_cutover_terminal_receipt = closeout_replay_consistency_decision
        .no_cutover_terminal_receipt
        && !closeout_replay_consistency_decision.live_cutover_enabled;
    let no_promotion_or_approval_mutation = !closeout_replay_consistency_decision.promotion_allowed
        && !closeout_replay_consistency_decision.operator_approval_recorded
        && !closeout_replay_consistency_decision.approval_record_mutation_enabled;
    let no_live_guardrails = !closeout_replay_consistency_decision.feature_flag_enabled
        && closeout_replay_consistency_decision.canary_stage == "off"
        && closeout_replay_consistency_decision.canary_traffic_ppm == 0
        && !closeout_replay_consistency_decision.live_blocking_enabled
        && !closeout_replay_consistency_decision.live_cutover_enabled
        && readback.live_blocking_event_count == 0
        && readback.live_cutover_event_count == 0;
    let chain_readback_ready = admission_ready
        && promotion_readiness_matrix_ready
        && operator_review_packet_ready
        && promotion_review_replay_ready
        && promotion_closeout_receipt_ready
        && promotion_closeout_replay_ready
        && no_live_guardrails;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "admission_shadow_decision_readback_ready",
            passed: admission_ready,
            detail: "audit chain requires durable admission shadow decision payload".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "promotion_readiness_matrix_readback_ready",
            passed: promotion_readiness_matrix_ready,
            detail: "audit chain requires durable promotion readiness matrix payload".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "operator_review_packet_readback_ready",
            passed: operator_review_packet_ready,
            detail: "audit chain requires durable operator review promotion packet payload"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "promotion_review_replay_readback_ready",
            passed: promotion_review_replay_ready,
            detail: "audit chain requires durable promotion review replay consistency payload"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "promotion_closeout_receipt_readback_ready",
            passed: promotion_closeout_receipt_ready,
            detail: "audit chain requires durable terminal no-cutover closeout receipt payload"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "promotion_closeout_replay_readback_ready",
            passed: promotion_closeout_replay_ready,
            detail: "audit chain requires durable closeout replay consistency payload".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "review_replay_consistent",
            passed: review_replay_consistent,
            detail: "promotion review replay consistency must be recorded as consistent"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "closeout_replay_consistent",
            passed: closeout_replay_consistent,
            detail: "promotion closeout replay consistency must be recorded as consistent"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_but_not_promoted_no_cutover",
            passed: reviewed_but_not_promoted && no_cutover_terminal_receipt,
            detail:
                "terminal audit chain must end in reviewed-but-not-promoted no-cutover evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_promotion_or_approval_mutation",
            passed: no_promotion_or_approval_mutation,
            detail: "audit chain must not record approval, reviewed flag mutation, or promotion"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_guardrails",
            passed: no_live_guardrails,
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
    let terminal_audit_ready = chain_readback_ready && audit_chain_blockers.is_empty();
    let decision = if terminal_audit_ready {
        PROMOTION_REVIEW_AUDIT_CHAIN_RECORDED_SHADOW
    } else {
        PROMOTION_REVIEW_AUDIT_CHAIN_BLOCKED_SHADOW
    };

    WorkGraphPromotionReviewAuditChainReceipt {
        decision,
        audit_stage: "terminal_promotion_review_audit_chain_shadow_only",
        job_id: readback.job_id.clone(),
        admission_shadow_decision_events: readback.admission_shadow_decision_events,
        promotion_readiness_matrix_events: readback.promotion_readiness_matrix_events,
        operator_review_promotion_packet_events: readback.operator_review_promotion_packet_events,
        promotion_review_replay_consistency_events: readback
            .promotion_review_replay_consistency_events,
        promotion_closeout_receipt_events: readback.promotion_closeout_receipt_events,
        promotion_closeout_replay_consistency_events: readback
            .promotion_closeout_replay_consistency_events,
        admission_ready,
        promotion_readiness_matrix_ready,
        operator_review_packet_ready,
        promotion_review_replay_ready,
        promotion_closeout_receipt_ready,
        promotion_closeout_replay_ready,
        chain_readback_ready,
        terminal_audit_ready,
        review_replay_consistent,
        closeout_replay_consistent,
        reviewed_but_not_promoted,
        no_cutover_terminal_receipt,
        shadow_readiness_failed: !terminal_audit_ready,
        admission_shadow_decision: payload_decision(
            readback.latest_admission_shadow_decision.as_ref(),
        ),
        promotion_readiness_matrix_decision: payload_decision(
            readback.latest_promotion_readiness_matrix.as_ref(),
        ),
        operator_review_packet_decision: payload_decision(
            readback.latest_operator_review_promotion_packet.as_ref(),
        ),
        promotion_review_replay_consistency_decision: payload_decision(
            readback.latest_promotion_review_replay_consistency.as_ref(),
        ),
        promotion_closeout_receipt_decision: payload_decision(
            readback.latest_promotion_closeout_receipt.as_ref(),
        ),
        promotion_closeout_replay_consistency_decision: payload_decision(
            readback
                .latest_promotion_closeout_replay_consistency
                .as_ref(),
        ),
        live_blocking_event_count: readback.live_blocking_event_count,
        live_cutover_event_count: readback.live_cutover_event_count,
        audit_chain_blockers,
        checks,
        operator_approval_recorded: false,
        approval_record_mutation_enabled: false,
        promotion_allowed: false,
        promotion_prohibited_reason: "terminal audit chain is report-only; separate reviewed flag path required",
        feature_flag_id: "work_graph_promotion_review_audit_chain_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_reviewed_flag_precondition_plan_packet(
    audit_chain_receipt: &WorkGraphPromotionReviewAuditChainReceipt,
    readback: &codex_state::AgentJobWorkGraphPromotionReviewReadback,
) -> WorkGraphReviewedFlagPreconditionPlanPacket {
    let no_promotion_or_approval_mutation = !audit_chain_receipt.promotion_allowed
        && !audit_chain_receipt.operator_approval_recorded
        && !audit_chain_receipt.approval_record_mutation_enabled;
    let no_live_guardrails = !audit_chain_receipt.feature_flag_enabled
        && audit_chain_receipt.canary_stage == "off"
        && audit_chain_receipt.canary_traffic_ppm == 0
        && !audit_chain_receipt.live_blocking_enabled
        && !audit_chain_receipt.live_cutover_enabled
        && readback.live_blocking_event_count == 0
        && readback.live_cutover_event_count == 0;
    let missing_live_promotion_prerequisites = vec![
        "operator_approval_recording_path".to_string(),
        "reviewed_flag_mutation_path".to_string(),
        "reviewed_flag_idempotency_key_contract".to_string(),
        "production_state_write_authorization".to_string(),
        "canary_traffic_plan".to_string(),
        "live_blocking_guardrail_enforcement".to_string(),
        "live_cutover_runbook".to_string(),
        "rollback_and_replay_recovery_plan".to_string(),
    ];
    let satisfied_shadow_prerequisites = vec![
        "terminal_audit_chain_receipt_present".to_string(),
        "admission_matrix_review_replay_closeout_chain_readback_ready".to_string(),
        "reviewed_but_not_promoted_terminal_evidence".to_string(),
        "no_live_guardrails_observed".to_string(),
        "no_approval_or_promotion_mutation_observed".to_string(),
    ];
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "terminal_audit_chain_receipt_ready",
            passed: readback.audit_chain_receipt_ready && audit_chain_receipt.terminal_audit_ready,
            detail: "dry-run plan requires a recorded terminal audit-chain receipt".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_but_not_promoted_evidence",
            passed: audit_chain_receipt.reviewed_but_not_promoted
                && audit_chain_receipt.no_cutover_terminal_receipt,
            detail:
                "dry-run plan requires reviewed-but-not-promoted no-cutover terminal evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_promotion_or_approval_mutation",
            passed: no_promotion_or_approval_mutation,
            detail:
                "dry-run plan must not record approval, reviewed flag mutation, or promotion"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_guardrails",
            passed: no_live_guardrails,
            detail:
                "feature flag, canary traffic, live blocking, and live cutover must remain disabled"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "live_prerequisites_are_plan_only",
            passed: !missing_live_promotion_prerequisites.is_empty(),
            detail:
                "live promotion prerequisites must be enumerated without enabling reviewed flag mutation"
                    .to_string(),
        },
    ];
    let plan_blockers = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let dry_run_plan_ready = plan_blockers.is_empty();
    let missing_live_promotion_prerequisite_count = missing_live_promotion_prerequisites.len();
    let satisfied_shadow_prerequisite_count = satisfied_shadow_prerequisites.len();
    let decision = if dry_run_plan_ready {
        REVIEWED_FLAG_PRECONDITION_PLAN_RECORDED_SHADOW
    } else {
        REVIEWED_FLAG_PRECONDITION_PLAN_BLOCKED_SHADOW
    };

    WorkGraphReviewedFlagPreconditionPlanPacket {
        decision,
        plan_stage: "reviewed_flag_precondition_plan_shadow_only",
        job_id: audit_chain_receipt.job_id.clone(),
        audit_chain_receipt_decision: audit_chain_receipt.decision,
        audit_chain_receipt_events: readback.promotion_review_audit_chain_receipt_events,
        audit_chain_receipt_ready: readback.audit_chain_receipt_ready,
        terminal_audit_ready: audit_chain_receipt.terminal_audit_ready,
        reviewed_but_not_promoted: audit_chain_receipt.reviewed_but_not_promoted,
        no_cutover_terminal_receipt: audit_chain_receipt.no_cutover_terminal_receipt,
        dry_run_plan_ready,
        shadow_readiness_failed: !dry_run_plan_ready,
        missing_live_promotion_prerequisites,
        missing_live_promotion_prerequisite_count,
        satisfied_shadow_prerequisites,
        satisfied_shadow_prerequisite_count,
        plan_blockers,
        checks,
        reviewed_flag_id: "work_graph_reviewed_flag_shadow_plan",
        reviewed_flag_mutation_dry_run: true,
        reviewed_flag_mutation_enabled: false,
        operator_review_required: true,
        operator_approval_recorded: false,
        approval_record_mutation_enabled: false,
        promotion_allowed: false,
        promotion_prohibited_reason: "reviewed flag precondition plan is dry-run only; reviewed flag mutation, canary, blocking, and cutover remain disabled",
        feature_flag_id: "work_graph_reviewed_flag_precondition_plan_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_reviewed_flag_precondition_plan_replay_consistency_decision(
    reviewed_flag_precondition_plan_packet: &WorkGraphReviewedFlagPreconditionPlanPacket,
    reviewed_flag_precondition_plan_packet_payload: &serde_json::Value,
    readback: &codex_state::AgentJobWorkGraphPromotionReviewReadback,
) -> WorkGraphReviewedFlagPreconditionPlanReplayConsistencyDecision {
    let reviewed_flag_precondition_plan_matches =
        readback.latest_reviewed_flag_precondition_plan.as_ref()
            == Some(reviewed_flag_precondition_plan_packet_payload);
    let no_promotion_or_approval_mutation = !reviewed_flag_precondition_plan_packet
        .promotion_allowed
        && !reviewed_flag_precondition_plan_packet.operator_approval_recorded
        && !reviewed_flag_precondition_plan_packet.approval_record_mutation_enabled
        && !reviewed_flag_precondition_plan_packet.reviewed_flag_mutation_enabled;
    let no_live_events =
        readback.live_blocking_event_count == 0 && readback.live_cutover_event_count == 0;
    let no_live_guardrails = !reviewed_flag_precondition_plan_packet.feature_flag_enabled
        && reviewed_flag_precondition_plan_packet.canary_stage == "off"
        && reviewed_flag_precondition_plan_packet.canary_traffic_ppm == 0
        && !reviewed_flag_precondition_plan_packet.live_blocking_enabled
        && !reviewed_flag_precondition_plan_packet.live_cutover_enabled
        && no_live_events;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_flag_precondition_plan_ready",
            passed: readback.reviewed_flag_precondition_plan_ready,
            detail: "durable readback must include the reviewed flag precondition plan packet"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_flag_precondition_plan_latest_payload_matches",
            passed: reviewed_flag_precondition_plan_matches,
            detail:
                "latest durable reviewed flag precondition plan must match the tool result payload"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_flag_precondition_plan_dry_run_ready",
            passed: reviewed_flag_precondition_plan_packet.dry_run_plan_ready
                && reviewed_flag_precondition_plan_packet.reviewed_flag_mutation_dry_run,
            detail:
                "reviewed flag precondition plan must remain a dry-run plan without mutation"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "live_promotion_prerequisites_still_missing",
            passed: reviewed_flag_precondition_plan_packet
                .missing_live_promotion_prerequisite_count
                > 0,
            detail:
                "reviewed flag replay gate must preserve live promotion prerequisites as plan-only"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_promotion_or_approval_mutation",
            passed: no_promotion_or_approval_mutation,
            detail:
                "reviewed flag replay gate must not record approval, reviewed flag mutation, or promotion"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_reviewed_flag_replay_events",
            passed: no_live_guardrails,
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
        REVIEWED_FLAG_PRECONDITION_PLAN_REPLAY_CONSISTENT_SHADOW
    } else {
        REVIEWED_FLAG_PRECONDITION_PLAN_REPLAY_MISMATCH_SHADOW
    };

    WorkGraphReviewedFlagPreconditionPlanReplayConsistencyDecision {
        decision,
        replay_stage: "reviewed_flag_precondition_plan_replay_shadow_only",
        job_id: readback.job_id.clone(),
        reviewed_flag_precondition_plan_decision: reviewed_flag_precondition_plan_packet.decision,
        reviewed_flag_precondition_plan_events: readback.reviewed_flag_precondition_plan_events,
        reviewed_flag_precondition_plan_ready: readback.reviewed_flag_precondition_plan_ready,
        reviewed_flag_precondition_plan_matches,
        dry_run_plan_ready: reviewed_flag_precondition_plan_packet.dry_run_plan_ready,
        missing_live_promotion_prerequisite_count: reviewed_flag_precondition_plan_packet
            .missing_live_promotion_prerequisite_count,
        replay_consistent,
        shadow_readiness_failed: !replay_consistent,
        live_blocking_event_count: readback.live_blocking_event_count,
        live_cutover_event_count: readback.live_cutover_event_count,
        consistency_blockers,
        checks,
        reviewed_flag_mutation_dry_run: true,
        reviewed_flag_mutation_enabled: false,
        operator_approval_recorded: false,
        approval_record_mutation_enabled: false,
        promotion_allowed: false,
        promotion_prohibited_reason: "reviewed flag precondition plan replay is report-only; reviewed flag mutation, canary, blocking, and cutover remain disabled",
        feature_flag_id: "work_graph_reviewed_flag_precondition_plan_replay_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_reviewed_flag_readiness_closeout_receipt(
    reviewed_flag_precondition_plan_packet: &WorkGraphReviewedFlagPreconditionPlanPacket,
    reviewed_flag_precondition_plan_replay_consistency_decision:
        &WorkGraphReviewedFlagPreconditionPlanReplayConsistencyDecision,
    readback: &codex_state::AgentJobWorkGraphPromotionReviewReadback,
) -> WorkGraphReviewedFlagReadinessCloseoutReceipt {
    let no_promotion_or_approval_mutation = !reviewed_flag_precondition_plan_packet
        .promotion_allowed
        && !reviewed_flag_precondition_plan_packet.operator_approval_recorded
        && !reviewed_flag_precondition_plan_packet.approval_record_mutation_enabled
        && !reviewed_flag_precondition_plan_packet.reviewed_flag_mutation_enabled
        && !reviewed_flag_precondition_plan_replay_consistency_decision.promotion_allowed
        && !reviewed_flag_precondition_plan_replay_consistency_decision.operator_approval_recorded
        && !reviewed_flag_precondition_plan_replay_consistency_decision
            .approval_record_mutation_enabled
        && !reviewed_flag_precondition_plan_replay_consistency_decision
            .reviewed_flag_mutation_enabled;
    let no_live_guardrails = !reviewed_flag_precondition_plan_packet.feature_flag_enabled
        && reviewed_flag_precondition_plan_packet.canary_stage == "off"
        && reviewed_flag_precondition_plan_packet.canary_traffic_ppm == 0
        && !reviewed_flag_precondition_plan_packet.live_blocking_enabled
        && !reviewed_flag_precondition_plan_packet.live_cutover_enabled
        && !reviewed_flag_precondition_plan_replay_consistency_decision.feature_flag_enabled
        && reviewed_flag_precondition_plan_replay_consistency_decision.canary_stage == "off"
        && reviewed_flag_precondition_plan_replay_consistency_decision.canary_traffic_ppm == 0
        && !reviewed_flag_precondition_plan_replay_consistency_decision.live_blocking_enabled
        && !reviewed_flag_precondition_plan_replay_consistency_decision.live_cutover_enabled
        && readback.live_blocking_event_count == 0
        && readback.live_cutover_event_count == 0;
    let dry_run_plan_ready = reviewed_flag_precondition_plan_packet.dry_run_plan_ready
        && reviewed_flag_precondition_plan_packet.reviewed_flag_mutation_dry_run
        && reviewed_flag_precondition_plan_packet.missing_live_promotion_prerequisite_count > 0;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_flag_precondition_plan_ready",
            passed: readback.reviewed_flag_precondition_plan_ready,
            detail: "durable readback must include the reviewed flag precondition plan packet"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_flag_precondition_plan_replay_ready",
            passed: readback.reviewed_flag_precondition_plan_replay_consistency_ready,
            detail: "durable readback must include the reviewed flag precondition plan replay gate"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_flag_precondition_plan_replay_consistent",
            passed: reviewed_flag_precondition_plan_replay_consistency_decision.replay_consistent
                && !reviewed_flag_precondition_plan_replay_consistency_decision
                    .shadow_readiness_failed,
            detail: "reviewed flag plan replay gate must match tool-result and durable latest payloads"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_flag_precondition_plan_is_dry_run_only",
            passed: dry_run_plan_ready,
            detail:
                "reviewed flag readiness closeout must preserve missing live prerequisites as plan-only"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_promotion_or_approval_mutation",
            passed: no_promotion_or_approval_mutation,
            detail: "readiness closeout must not record approval, reviewed flag mutation, or promotion"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_reviewed_flag_closeout_events",
            passed: no_live_guardrails,
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
    let terminal_closeout_ready = closeout_blockers.is_empty();
    let decision = if terminal_closeout_ready {
        REVIEWED_FLAG_READINESS_CLOSEOUT_RECORDED_SHADOW
    } else {
        REVIEWED_FLAG_READINESS_CLOSEOUT_BLOCKED_SHADOW
    };

    WorkGraphReviewedFlagReadinessCloseoutReceipt {
        decision,
        receipt_stage: "terminal_reviewed_flag_readiness_closeout_shadow_only",
        job_id: readback.job_id.clone(),
        reviewed_flag_precondition_plan_decision: reviewed_flag_precondition_plan_packet.decision,
        reviewed_flag_precondition_plan_replay_consistency_decision:
            reviewed_flag_precondition_plan_replay_consistency_decision.decision,
        reviewed_flag_precondition_plan_events: readback.reviewed_flag_precondition_plan_events,
        reviewed_flag_precondition_plan_replay_consistency_events: readback
            .reviewed_flag_precondition_plan_replay_consistency_events,
        reviewed_flag_precondition_plan_ready: readback.reviewed_flag_precondition_plan_ready,
        reviewed_flag_precondition_plan_replay_consistency_ready: readback
            .reviewed_flag_precondition_plan_replay_consistency_ready,
        dry_run_plan_ready,
        replay_consistent: reviewed_flag_precondition_plan_replay_consistency_decision
            .replay_consistent,
        planned_but_not_mutable: terminal_closeout_ready,
        terminal_closeout_ready,
        shadow_readiness_failed: !terminal_closeout_ready,
        missing_live_promotion_prerequisites: reviewed_flag_precondition_plan_packet
            .missing_live_promotion_prerequisites
            .clone(),
        missing_live_promotion_prerequisite_count: reviewed_flag_precondition_plan_packet
            .missing_live_promotion_prerequisite_count,
        live_blocking_event_count: readback.live_blocking_event_count,
        live_cutover_event_count: readback.live_cutover_event_count,
        closeout_blockers,
        checks,
        reviewed_flag_mutation_dry_run: true,
        reviewed_flag_mutation_enabled: false,
        operator_review_required: true,
        operator_approval_recorded: false,
        approval_record_mutation_enabled: false,
        promotion_allowed: false,
        promotion_prohibited_reason: "reviewed flag readiness closeout is terminal shadow evidence only; reviewed flag mutation, canary, blocking, and cutover remain disabled",
        feature_flag_id: "work_graph_reviewed_flag_readiness_closeout_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_reviewed_flag_readiness_closeout_replay_consistency_decision(
    reviewed_flag_readiness_closeout_receipt: &WorkGraphReviewedFlagReadinessCloseoutReceipt,
    reviewed_flag_readiness_closeout_receipt_payload: &serde_json::Value,
    readback: &codex_state::AgentJobWorkGraphPromotionReviewReadback,
) -> WorkGraphReviewedFlagReadinessCloseoutReplayConsistencyDecision {
    let reviewed_flag_readiness_closeout_receipt_matches = readback
        .latest_reviewed_flag_readiness_closeout_receipt
        .as_ref()
        == Some(reviewed_flag_readiness_closeout_receipt_payload);
    let no_promotion_or_approval_mutation = !reviewed_flag_readiness_closeout_receipt
        .promotion_allowed
        && !reviewed_flag_readiness_closeout_receipt.operator_approval_recorded
        && !reviewed_flag_readiness_closeout_receipt.approval_record_mutation_enabled
        && !reviewed_flag_readiness_closeout_receipt.reviewed_flag_mutation_enabled;
    let no_live_events =
        readback.live_blocking_event_count == 0 && readback.live_cutover_event_count == 0;
    let no_live_guardrails = !reviewed_flag_readiness_closeout_receipt.feature_flag_enabled
        && reviewed_flag_readiness_closeout_receipt.canary_stage == "off"
        && reviewed_flag_readiness_closeout_receipt.canary_traffic_ppm == 0
        && !reviewed_flag_readiness_closeout_receipt.live_blocking_enabled
        && !reviewed_flag_readiness_closeout_receipt.live_cutover_enabled
        && no_live_events;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_flag_readiness_closeout_receipt_ready",
            passed: readback.reviewed_flag_readiness_closeout_receipt_ready,
            detail: "durable readback must include the reviewed flag readiness closeout receipt"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_flag_readiness_closeout_latest_payload_matches",
            passed: reviewed_flag_readiness_closeout_receipt_matches,
            detail:
                "latest durable reviewed flag readiness closeout must match the tool result payload"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_flag_readiness_closeout_planned_but_not_mutable",
            passed: reviewed_flag_readiness_closeout_receipt.terminal_closeout_ready
                && reviewed_flag_readiness_closeout_receipt.planned_but_not_mutable
                && reviewed_flag_readiness_closeout_receipt.reviewed_flag_mutation_dry_run
                && !reviewed_flag_readiness_closeout_receipt.reviewed_flag_mutation_enabled,
            detail: "readiness closeout replay must preserve planned-but-not-mutable evidence"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "live_promotion_prerequisites_still_missing",
            passed: reviewed_flag_readiness_closeout_receipt
                .missing_live_promotion_prerequisite_count
                > 0,
            detail: "readiness closeout replay must preserve live promotion prerequisites as missing"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_promotion_or_approval_mutation",
            passed: no_promotion_or_approval_mutation,
            detail:
                "readiness closeout replay must not record approval, reviewed flag mutation, or promotion"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_reviewed_flag_closeout_replay_events",
            passed: no_live_guardrails,
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
        REVIEWED_FLAG_READINESS_CLOSEOUT_REPLAY_CONSISTENT_SHADOW
    } else {
        REVIEWED_FLAG_READINESS_CLOSEOUT_REPLAY_MISMATCH_SHADOW
    };

    WorkGraphReviewedFlagReadinessCloseoutReplayConsistencyDecision {
        decision,
        replay_stage: "reviewed_flag_readiness_closeout_replay_shadow_only",
        job_id: readback.job_id.clone(),
        reviewed_flag_readiness_closeout_decision: reviewed_flag_readiness_closeout_receipt
            .decision,
        reviewed_flag_readiness_closeout_receipt_events: readback
            .reviewed_flag_readiness_closeout_receipt_events,
        reviewed_flag_readiness_closeout_receipt_ready: readback
            .reviewed_flag_readiness_closeout_receipt_ready,
        reviewed_flag_readiness_closeout_receipt_matches,
        planned_but_not_mutable: reviewed_flag_readiness_closeout_receipt.planned_but_not_mutable,
        terminal_closeout_ready: reviewed_flag_readiness_closeout_receipt.terminal_closeout_ready,
        missing_live_promotion_prerequisite_count: reviewed_flag_readiness_closeout_receipt
            .missing_live_promotion_prerequisite_count,
        replay_consistent,
        shadow_readiness_failed: !replay_consistent,
        live_blocking_event_count: readback.live_blocking_event_count,
        live_cutover_event_count: readback.live_cutover_event_count,
        consistency_blockers,
        checks,
        reviewed_flag_mutation_dry_run: true,
        reviewed_flag_mutation_enabled: false,
        operator_approval_recorded: false,
        approval_record_mutation_enabled: false,
        promotion_allowed: false,
        promotion_prohibited_reason: "reviewed flag readiness closeout replay is report-only; reviewed flag mutation, canary, blocking, and cutover remain disabled",
        feature_flag_id: "work_graph_reviewed_flag_readiness_closeout_replay_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

pub(crate) fn build_reviewed_flag_audit_chain_closeout_receipt(
    reviewed_flag_readiness_closeout_replay_consistency_decision:
        &WorkGraphReviewedFlagReadinessCloseoutReplayConsistencyDecision,
    readback: &codex_state::AgentJobWorkGraphPromotionReviewReadback,
) -> WorkGraphReviewedFlagAuditChainCloseoutReceipt {
    let reviewed_flag_precondition_plan_ready = readback.reviewed_flag_precondition_plan_ready
        && readback.latest_reviewed_flag_precondition_plan.is_some();
    let reviewed_flag_precondition_plan_replay_consistency_ready = readback
        .reviewed_flag_precondition_plan_replay_consistency_ready
        && readback
            .latest_reviewed_flag_precondition_plan_replay_consistency
            .is_some();
    let reviewed_flag_readiness_closeout_receipt_ready = readback
        .reviewed_flag_readiness_closeout_receipt_ready
        && readback
            .latest_reviewed_flag_readiness_closeout_receipt
            .is_some();
    let reviewed_flag_readiness_closeout_replay_consistency_ready = readback
        .reviewed_flag_readiness_closeout_replay_consistency_ready
        && readback
            .latest_reviewed_flag_readiness_closeout_replay_consistency
            .is_some();
    let dry_run_plan_ready = payload_bool(
        readback.latest_reviewed_flag_precondition_plan.as_ref(),
        "dryRunPlanReady",
    );
    let plan_replay_consistent = payload_bool(
        readback
            .latest_reviewed_flag_precondition_plan_replay_consistency
            .as_ref(),
        "replayConsistent",
    );
    let readiness_closeout_ready = payload_bool(
        readback
            .latest_reviewed_flag_readiness_closeout_receipt
            .as_ref(),
        "terminalCloseoutReady",
    );
    let readiness_closeout_replay_consistent =
        reviewed_flag_readiness_closeout_replay_consistency_decision.replay_consistent
            && !reviewed_flag_readiness_closeout_replay_consistency_decision
                .shadow_readiness_failed;
    let planned_but_not_mutable = reviewed_flag_readiness_closeout_replay_consistency_decision
        .planned_but_not_mutable
        && !reviewed_flag_readiness_closeout_replay_consistency_decision
            .reviewed_flag_mutation_enabled
        && !reviewed_flag_readiness_closeout_replay_consistency_decision.promotion_allowed;
    let missing_live_promotion_prerequisite_count =
        reviewed_flag_readiness_closeout_replay_consistency_decision
            .missing_live_promotion_prerequisite_count;
    let no_promotion_or_approval_mutation =
        !reviewed_flag_readiness_closeout_replay_consistency_decision.promotion_allowed
            && !reviewed_flag_readiness_closeout_replay_consistency_decision
                .operator_approval_recorded
            && !reviewed_flag_readiness_closeout_replay_consistency_decision
                .approval_record_mutation_enabled
            && !reviewed_flag_readiness_closeout_replay_consistency_decision
                .reviewed_flag_mutation_enabled;
    let no_live_guardrails = !reviewed_flag_readiness_closeout_replay_consistency_decision
        .feature_flag_enabled
        && reviewed_flag_readiness_closeout_replay_consistency_decision.canary_stage == "off"
        && reviewed_flag_readiness_closeout_replay_consistency_decision.canary_traffic_ppm == 0
        && !reviewed_flag_readiness_closeout_replay_consistency_decision.live_blocking_enabled
        && !reviewed_flag_readiness_closeout_replay_consistency_decision.live_cutover_enabled
        && readback.live_blocking_event_count == 0
        && readback.live_cutover_event_count == 0;
    let reviewed_flag_chain_readback_ready = reviewed_flag_precondition_plan_ready
        && reviewed_flag_precondition_plan_replay_consistency_ready
        && reviewed_flag_readiness_closeout_receipt_ready
        && reviewed_flag_readiness_closeout_replay_consistency_ready
        && no_live_guardrails;
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_flag_precondition_plan_readback_ready",
            passed: reviewed_flag_precondition_plan_ready,
            detail: "reviewed flag audit-chain closeout requires durable precondition plan payload"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_flag_precondition_plan_replay_readback_ready",
            passed: reviewed_flag_precondition_plan_replay_consistency_ready,
            detail:
                "reviewed flag audit-chain closeout requires durable plan replay consistency payload"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_flag_readiness_closeout_readback_ready",
            passed: reviewed_flag_readiness_closeout_receipt_ready,
            detail: "reviewed flag audit-chain closeout requires durable readiness closeout payload"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_flag_readiness_closeout_replay_readback_ready",
            passed: reviewed_flag_readiness_closeout_replay_consistency_ready,
            detail:
                "reviewed flag audit-chain closeout requires durable readiness closeout replay payload"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_flag_precondition_plan_replay_consistent",
            passed: plan_replay_consistent,
            detail: "reviewed flag precondition plan replay must be recorded as consistent"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_flag_readiness_closeout_replay_consistent",
            passed: readiness_closeout_replay_consistent,
            detail: "reviewed flag readiness closeout replay must be recorded as consistent"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "reviewed_flag_planned_but_not_mutable_terminal_evidence",
            passed: dry_run_plan_ready
                && readiness_closeout_ready
                && planned_but_not_mutable
                && missing_live_promotion_prerequisite_count > 0,
            detail:
                "terminal reviewed flag audit-chain closeout must preserve planned-but-not-mutable evidence"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_promotion_or_approval_mutation",
            passed: no_promotion_or_approval_mutation,
            detail:
                "reviewed flag audit-chain closeout must not record approval, reviewed flag mutation, or promotion"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_reviewed_flag_audit_chain_closeout_events",
            passed: no_live_guardrails,
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
    let terminal_reviewed_flag_audit_ready =
        reviewed_flag_chain_readback_ready && audit_chain_blockers.is_empty();
    let decision = if terminal_reviewed_flag_audit_ready {
        REVIEWED_FLAG_AUDIT_CHAIN_CLOSEOUT_RECORDED_SHADOW
    } else {
        REVIEWED_FLAG_AUDIT_CHAIN_CLOSEOUT_BLOCKED_SHADOW
    };

    WorkGraphReviewedFlagAuditChainCloseoutReceipt {
        decision,
        audit_stage: "terminal_reviewed_flag_audit_chain_closeout_shadow_only",
        job_id: readback.job_id.clone(),
        reviewed_flag_precondition_plan_events: readback.reviewed_flag_precondition_plan_events,
        reviewed_flag_precondition_plan_replay_consistency_events: readback
            .reviewed_flag_precondition_plan_replay_consistency_events,
        reviewed_flag_readiness_closeout_receipt_events: readback
            .reviewed_flag_readiness_closeout_receipt_events,
        reviewed_flag_readiness_closeout_replay_consistency_events: readback
            .reviewed_flag_readiness_closeout_replay_consistency_events,
        reviewed_flag_precondition_plan_ready,
        reviewed_flag_precondition_plan_replay_consistency_ready,
        reviewed_flag_readiness_closeout_receipt_ready,
        reviewed_flag_readiness_closeout_replay_consistency_ready,
        reviewed_flag_chain_readback_ready,
        terminal_reviewed_flag_audit_ready,
        dry_run_plan_ready,
        plan_replay_consistent,
        readiness_closeout_ready,
        readiness_closeout_replay_consistent,
        planned_but_not_mutable,
        shadow_readiness_failed: !terminal_reviewed_flag_audit_ready,
        missing_live_promotion_prerequisite_count,
        reviewed_flag_precondition_plan_decision: payload_decision(
            readback.latest_reviewed_flag_precondition_plan.as_ref(),
        ),
        reviewed_flag_precondition_plan_replay_consistency_decision: payload_decision(
            readback
                .latest_reviewed_flag_precondition_plan_replay_consistency
                .as_ref(),
        ),
        reviewed_flag_readiness_closeout_decision: payload_decision(
            readback
                .latest_reviewed_flag_readiness_closeout_receipt
                .as_ref(),
        ),
        reviewed_flag_readiness_closeout_replay_consistency_decision: payload_decision(
            readback
                .latest_reviewed_flag_readiness_closeout_replay_consistency
                .as_ref(),
        ),
        live_blocking_event_count: readback.live_blocking_event_count,
        live_cutover_event_count: readback.live_cutover_event_count,
        audit_chain_blockers,
        checks,
        reviewed_flag_mutation_dry_run: true,
        reviewed_flag_mutation_enabled: false,
        operator_review_required: true,
        operator_approval_recorded: false,
        approval_record_mutation_enabled: false,
        promotion_allowed: false,
        promotion_prohibited_reason: "reviewed flag audit-chain closeout is terminal shadow evidence only; reviewed flag mutation, canary, blocking, and cutover remain disabled",
        feature_flag_id: "work_graph_reviewed_flag_audit_chain_closeout_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

fn payload_decision(payload: Option<&serde_json::Value>) -> String {
    payload
        .and_then(|payload| payload.get("decision"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or("missing")
        .to_string()
}

fn payload_bool(payload: Option<&serde_json::Value>, field: &str) -> bool {
    payload
        .and_then(|payload| payload.get(field))
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AgentCardManifestConfig;
    use crate::tools::handlers::work_graph_admission::WorkGraphAgentCardManifest;
    use crate::tools::handlers::work_graph_admission::WorkGraphRoleManifestShadowDecision;
    use crate::tools::handlers::work_graph_admission::WorkGraphRoleManifestShadowInput;
    use crate::tools::handlers::work_graph_admission::build_work_graph_role_manifest_shadow_decision;
    use crate::tools::handlers::work_graph_admission::default_agent_card_manifest_registry;
    use crate::tools::handlers::work_graph_promotion_readiness::build_default_governed_promotion_readiness_shadow_matrix;
    use codex_state::AgentJobWorkGraphPromotionReviewReadback;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    const AGENT_CARD_MANIFEST_VERSION: &str = "hepta.agent_card_manifest.v1";

    fn ready_decision(manifest: WorkGraphAgentCardManifest) -> WorkGraphRoleManifestShadowDecision {
        build_work_graph_role_manifest_shadow_decision(WorkGraphRoleManifestShadowInput {
            source_surface_id: manifest.source_surface_id,
            definition_source: "test_ready_agent_card_manifest",
            manifest_version: AGENT_CARD_MANIFEST_VERSION,
            role_name: manifest.role_name.map(str::to_string),
            description: manifest.description,
            role_declared: true,
            role_description_present: true,
            configured_manifest_source: Some(format!(
                "agent-card://{}",
                manifest.source_surface_id
            )),
            configured_manifest_version: Some(AGENT_CARD_MANIFEST_VERSION.to_string()),
            configured_manifest_overlay: Some(AgentCardManifestConfig {
                schema_version: Some(AGENT_CARD_MANIFEST_VERSION.to_string()),
                source_surface_id: Some(manifest.source_surface_id.to_string()),
                capabilities: manifest
                    .capabilities
                    .iter()
                    .map(|capability| (*capability).to_string())
                    .collect(),
                allowed_tools: manifest
                    .allowed_tools
                    .iter()
                    .map(|tool| (*tool).to_string())
                    .collect(),
                lane: Some(manifest.lane.to_string()),
                max_threads: Some(8),
                max_depth: None,
            }),
            capabilities: manifest
                .capabilities
                .iter()
                .map(|capability| (*capability).to_string())
                .collect(),
            allowed_tools: manifest
                .allowed_tools
                .iter()
                .map(|tool| (*tool).to_string())
                .collect(),
            attempted_tool: manifest.allowed_tools.first().copied(),
            budget_present: true,
            side_effect_class: manifest.side_effect_class,
            output_contract_required: manifest.output_contract_required,
            output_contract_present: true,
            result_contract_required: manifest.result_contract_required,
            result_contract_present: true,
            verifier_present: true,
            reducer_present: true,
            lane: manifest.lane,
            observed_lane: Some(manifest.lane),
        })
    }

    #[test]
    fn operator_review_packet_blocks_incomplete_matrix() {
        let registry = default_agent_card_manifest_registry();
        let decision = ready_decision(
            registry
                .manifest_for_source("spawn_agents_on_csv")
                .expect("manifest should resolve"),
        );
        let matrix = build_default_governed_promotion_readiness_shadow_matrix(&[decision]);

        let packet = build_operator_review_promotion_packet(&matrix);

        assert_eq!(
            packet.decision,
            "operator_review_packet_blocked_shadow_no_live_cutover"
        );
        assert!(!packet.review_ready);
        assert!(packet.operator_review_required);
        assert!(!packet.operator_approval_recorded);
        assert!(!packet.approval_record_mutation_enabled);
        assert!(!packet.promotion_allowed);
        assert_eq!(packet.expected_source_surface_count, 8);
        assert_eq!(packet.observed_source_surface_count, 1);
        assert_eq!(packet.missing_source_surface_ids.len(), 7);
        assert!(
            packet
                .review_blockers
                .iter()
                .any(|blocker| blocker.contains("source_surface_coverage"))
        );
        assert!(!packet.feature_flag_enabled);
        assert_eq!(packet.canary_stage, "off");
        assert_eq!(packet.canary_traffic_ppm, 0);
        assert!(!packet.live_blocking_enabled);
        assert!(!packet.live_cutover_enabled);
    }

    #[test]
    fn operator_review_packet_can_be_ready_but_never_promotes() {
        let registry = default_agent_card_manifest_registry();
        let decisions = registry
            .entries()
            .iter()
            .map(|entry| ready_decision(entry.manifest))
            .collect::<Vec<_>>();
        let matrix = build_default_governed_promotion_readiness_shadow_matrix(decisions.as_slice());

        let packet = build_operator_review_promotion_packet(&matrix);

        assert_eq!(
            packet.decision,
            "operator_review_packet_ready_shadow_no_live_cutover"
        );
        assert!(packet.review_ready);
        assert!(packet.coverage_ready);
        assert!(packet.all_promotion_ready);
        assert!(packet.no_live_guardrails_ready);
        assert_eq!(packet.promotion_ready_count, 8);
        assert_eq!(packet.promotion_not_ready_count, 0);
        assert!(packet.review_blockers.is_empty());
        assert!(!packet.promotion_allowed);
        assert_eq!(
            packet.promotion_prohibited_reason,
            "operator review packet is report-only; separate reviewed flag path required"
        );
        assert!(!packet.operator_approval_recorded);
        assert!(!packet.approval_record_mutation_enabled);

        let value = serde_json::to_value(&packet).expect("packet should serialize");
        assert_eq!(
            value["promotionStage"],
            json!("operator_review_shadow_only")
        );
        assert_eq!(value["promotionAllowed"], json!(false));
        assert_eq!(value["liveCutoverEnabled"], json!(false));
    }

    #[test]
    fn promotion_review_replay_consistency_accepts_matching_durable_payloads() {
        let admission_payload = json!({
            "decision": "admit_shadow",
            "sourceSurfaceId": "spawn_agents_on_csv",
        });
        let matrix_payload = json!({
            "decision": "promotion_matrix_ready_shadow_no_live_cutover",
            "coverageReady": true,
        });
        let packet_payload = json!({
            "decision": "operator_review_packet_ready_shadow_no_live_cutover",
            "promotionAllowed": false,
        });
        let readback = AgentJobWorkGraphPromotionReviewReadback {
            job_id: "job-123".to_string(),
            admission_shadow_decision_events: 1,
            promotion_readiness_matrix_events: 1,
            operator_review_promotion_packet_events: 1,
            promotion_review_replay_consistency_events: 0,
            promotion_closeout_receipt_events: 0,
            promotion_closeout_replay_consistency_events: 0,
            promotion_review_audit_chain_receipt_events: 0,
            reviewed_flag_precondition_plan_events: 0,
            reviewed_flag_precondition_plan_replay_consistency_events: 0,
            reviewed_flag_readiness_closeout_receipt_events: 0,
            reviewed_flag_readiness_closeout_replay_consistency_events: 0,
            reviewed_flag_audit_chain_closeout_receipt_events: 0,
            live_blocking_event_count: 0,
            live_cutover_event_count: 0,
            latest_admission_shadow_decision: Some(admission_payload.clone()),
            latest_promotion_readiness_matrix: Some(matrix_payload.clone()),
            latest_operator_review_promotion_packet: Some(packet_payload.clone()),
            latest_promotion_review_replay_consistency: None,
            latest_promotion_closeout_receipt: None,
            latest_promotion_closeout_replay_consistency: None,
            latest_promotion_review_audit_chain_receipt: None,
            latest_reviewed_flag_precondition_plan: None,
            latest_reviewed_flag_precondition_plan_replay_consistency: None,
            latest_reviewed_flag_readiness_closeout_receipt: None,
            latest_reviewed_flag_readiness_closeout_replay_consistency: None,
            latest_reviewed_flag_audit_chain_closeout_receipt: None,
            readback_ready: true,
            replay_consistency_ready: false,
            closeout_receipt_ready: false,
            closeout_replay_consistency_ready: false,
            audit_chain_receipt_ready: false,
            reviewed_flag_precondition_plan_ready: false,
            reviewed_flag_precondition_plan_replay_consistency_ready: false,
            reviewed_flag_readiness_closeout_receipt_ready: false,
            reviewed_flag_readiness_closeout_replay_consistency_ready: false,
            reviewed_flag_audit_chain_closeout_receipt_ready: false,
        };

        let decision = build_promotion_review_replay_consistency_decision(
            &admission_payload,
            &matrix_payload,
            &packet_payload,
            &readback,
        );

        assert_eq!(
            decision.decision,
            "promotion_review_replay_consistent_shadow_no_live_cutover"
        );
        assert!(decision.replay_consistent);
        assert!(!decision.shadow_readiness_failed);
        assert!(decision.consistency_blockers.is_empty());
        assert!(!decision.promotion_allowed);
        assert!(!decision.operator_approval_recorded);
        assert!(!decision.approval_record_mutation_enabled);
        assert!(!decision.feature_flag_enabled);
        assert_eq!(decision.canary_stage, "off");
        assert_eq!(decision.canary_traffic_ppm, 0);
        assert!(!decision.live_blocking_enabled);
        assert!(!decision.live_cutover_enabled);
    }

    #[test]
    fn promotion_review_replay_consistency_fails_shadow_readiness_on_mismatch() {
        let admission_payload = json!({ "decision": "admit_shadow" });
        let matrix_payload = json!({ "decision": "promotion_matrix_ready" });
        let packet_payload = json!({ "decision": "operator_review_packet_ready" });
        let readback = AgentJobWorkGraphPromotionReviewReadback {
            job_id: "job-456".to_string(),
            admission_shadow_decision_events: 1,
            promotion_readiness_matrix_events: 1,
            operator_review_promotion_packet_events: 1,
            promotion_review_replay_consistency_events: 0,
            promotion_closeout_receipt_events: 0,
            promotion_closeout_replay_consistency_events: 0,
            promotion_review_audit_chain_receipt_events: 0,
            reviewed_flag_precondition_plan_events: 0,
            reviewed_flag_precondition_plan_replay_consistency_events: 0,
            reviewed_flag_readiness_closeout_receipt_events: 0,
            reviewed_flag_readiness_closeout_replay_consistency_events: 0,
            reviewed_flag_audit_chain_closeout_receipt_events: 0,
            live_blocking_event_count: 0,
            live_cutover_event_count: 0,
            latest_admission_shadow_decision: Some(admission_payload.clone()),
            latest_promotion_readiness_matrix: Some(json!({
                "decision": "stale_matrix"
            })),
            latest_operator_review_promotion_packet: Some(packet_payload.clone()),
            latest_promotion_review_replay_consistency: None,
            latest_promotion_closeout_receipt: None,
            latest_promotion_closeout_replay_consistency: None,
            latest_promotion_review_audit_chain_receipt: None,
            latest_reviewed_flag_precondition_plan: None,
            latest_reviewed_flag_precondition_plan_replay_consistency: None,
            latest_reviewed_flag_readiness_closeout_receipt: None,
            latest_reviewed_flag_readiness_closeout_replay_consistency: None,
            latest_reviewed_flag_audit_chain_closeout_receipt: None,
            readback_ready: true,
            replay_consistency_ready: false,
            closeout_receipt_ready: false,
            closeout_replay_consistency_ready: false,
            audit_chain_receipt_ready: false,
            reviewed_flag_precondition_plan_ready: false,
            reviewed_flag_precondition_plan_replay_consistency_ready: false,
            reviewed_flag_readiness_closeout_receipt_ready: false,
            reviewed_flag_readiness_closeout_replay_consistency_ready: false,
            reviewed_flag_audit_chain_closeout_receipt_ready: false,
        };

        let decision = build_promotion_review_replay_consistency_decision(
            &admission_payload,
            &matrix_payload,
            &packet_payload,
            &readback,
        );

        assert_eq!(
            decision.decision,
            "promotion_review_replay_mismatch_shadow_no_live_cutover"
        );
        assert!(!decision.replay_consistent);
        assert!(decision.shadow_readiness_failed);
        assert!(decision.admission_shadow_decision_matches);
        assert!(!decision.promotion_readiness_matrix_matches);
        assert!(decision.operator_review_promotion_packet_matches);
        assert!(
            decision.consistency_blockers.iter().any(
                |blocker| blocker.contains("promotion_readiness_matrix_latest_payload_matches")
            )
        );
        assert!(!decision.promotion_allowed);
        assert!(!decision.feature_flag_enabled);
        assert_eq!(decision.canary_stage, "off");
        assert!(!decision.live_cutover_enabled);
    }

    #[test]
    fn promotion_closeout_receipt_records_blocked_review_without_promotion() {
        let registry = default_agent_card_manifest_registry();
        let decision = ready_decision(
            registry
                .manifest_for_source("spawn_agents_on_csv")
                .expect("manifest should resolve"),
        );
        let matrix = build_default_governed_promotion_readiness_shadow_matrix(&[decision]);
        let packet = build_operator_review_promotion_packet(&matrix);
        let admission_payload = json!({ "decision": "allow_shadow_no_live_blocking" });
        let matrix_payload = serde_json::to_value(&matrix).expect("matrix should serialize");
        let packet_payload = serde_json::to_value(&packet).expect("packet should serialize");
        let readback_before_replay = AgentJobWorkGraphPromotionReviewReadback {
            job_id: "job-closeout".to_string(),
            admission_shadow_decision_events: 1,
            promotion_readiness_matrix_events: 1,
            operator_review_promotion_packet_events: 1,
            promotion_review_replay_consistency_events: 0,
            promotion_closeout_receipt_events: 0,
            promotion_closeout_replay_consistency_events: 0,
            promotion_review_audit_chain_receipt_events: 0,
            reviewed_flag_precondition_plan_events: 0,
            reviewed_flag_precondition_plan_replay_consistency_events: 0,
            reviewed_flag_readiness_closeout_receipt_events: 0,
            reviewed_flag_readiness_closeout_replay_consistency_events: 0,
            reviewed_flag_audit_chain_closeout_receipt_events: 0,
            live_blocking_event_count: 0,
            live_cutover_event_count: 0,
            latest_admission_shadow_decision: Some(admission_payload.clone()),
            latest_promotion_readiness_matrix: Some(matrix_payload.clone()),
            latest_operator_review_promotion_packet: Some(packet_payload.clone()),
            latest_promotion_review_replay_consistency: None,
            latest_promotion_closeout_receipt: None,
            latest_promotion_closeout_replay_consistency: None,
            latest_promotion_review_audit_chain_receipt: None,
            latest_reviewed_flag_precondition_plan: None,
            latest_reviewed_flag_precondition_plan_replay_consistency: None,
            latest_reviewed_flag_readiness_closeout_receipt: None,
            latest_reviewed_flag_readiness_closeout_replay_consistency: None,
            latest_reviewed_flag_audit_chain_closeout_receipt: None,
            readback_ready: true,
            replay_consistency_ready: false,
            closeout_receipt_ready: false,
            closeout_replay_consistency_ready: false,
            audit_chain_receipt_ready: false,
            reviewed_flag_precondition_plan_ready: false,
            reviewed_flag_precondition_plan_replay_consistency_ready: false,
            reviewed_flag_readiness_closeout_receipt_ready: false,
            reviewed_flag_readiness_closeout_replay_consistency_ready: false,
            reviewed_flag_audit_chain_closeout_receipt_ready: false,
        };
        let replay_decision = build_promotion_review_replay_consistency_decision(
            &admission_payload,
            &matrix_payload,
            &packet_payload,
            &readback_before_replay,
        );
        let replay_payload =
            serde_json::to_value(&replay_decision).expect("replay decision should serialize");
        let closeout_readback = AgentJobWorkGraphPromotionReviewReadback {
            promotion_review_replay_consistency_events: 1,
            latest_promotion_review_replay_consistency: Some(replay_payload),
            replay_consistency_ready: true,
            ..readback_before_replay
        };

        let receipt =
            build_promotion_closeout_receipt(&packet, &replay_decision, &closeout_readback);

        assert_eq!(
            receipt.decision,
            "promotion_closeout_receipt_recorded_shadow_no_live_cutover"
        );
        assert!(receipt.closeout_ready);
        assert!(receipt.review_blocked);
        assert!(!receipt.review_ready);
        assert!(receipt.replay_consistent);
        assert!(receipt.reviewed_but_not_promoted);
        assert!(receipt.no_cutover_terminal_receipt);
        assert_eq!(receipt.review_outcome, "operator_review_blocked_no_cutover");
        assert!(receipt.closeout_blockers.is_empty());
        assert!(!receipt.promotion_allowed);
        assert!(!receipt.operator_approval_recorded);
        assert!(!receipt.approval_record_mutation_enabled);
        assert!(!receipt.feature_flag_enabled);
        assert_eq!(receipt.canary_stage, "off");
        assert_eq!(receipt.canary_traffic_ppm, 0);
        assert!(!receipt.live_blocking_enabled);
        assert!(!receipt.live_cutover_enabled);
    }

    #[test]
    fn promotion_closeout_receipt_blocks_on_replay_mismatch() {
        let registry = default_agent_card_manifest_registry();
        let decisions = registry
            .entries()
            .iter()
            .map(|entry| ready_decision(entry.manifest))
            .collect::<Vec<_>>();
        let matrix = build_default_governed_promotion_readiness_shadow_matrix(decisions.as_slice());
        let packet = build_operator_review_promotion_packet(&matrix);
        let admission_payload = json!({ "decision": "allow_shadow_no_live_blocking" });
        let matrix_payload = serde_json::to_value(&matrix).expect("matrix should serialize");
        let packet_payload = serde_json::to_value(&packet).expect("packet should serialize");
        let readback_before_replay = AgentJobWorkGraphPromotionReviewReadback {
            job_id: "job-closeout-mismatch".to_string(),
            admission_shadow_decision_events: 1,
            promotion_readiness_matrix_events: 1,
            operator_review_promotion_packet_events: 1,
            promotion_review_replay_consistency_events: 0,
            promotion_closeout_receipt_events: 0,
            promotion_closeout_replay_consistency_events: 0,
            promotion_review_audit_chain_receipt_events: 0,
            reviewed_flag_precondition_plan_events: 0,
            reviewed_flag_precondition_plan_replay_consistency_events: 0,
            reviewed_flag_readiness_closeout_receipt_events: 0,
            reviewed_flag_readiness_closeout_replay_consistency_events: 0,
            reviewed_flag_audit_chain_closeout_receipt_events: 0,
            live_blocking_event_count: 0,
            live_cutover_event_count: 0,
            latest_admission_shadow_decision: Some(admission_payload.clone()),
            latest_promotion_readiness_matrix: Some(json!({ "decision": "stale_matrix" })),
            latest_operator_review_promotion_packet: Some(packet_payload.clone()),
            latest_promotion_review_replay_consistency: None,
            latest_promotion_closeout_receipt: None,
            latest_promotion_closeout_replay_consistency: None,
            latest_promotion_review_audit_chain_receipt: None,
            latest_reviewed_flag_precondition_plan: None,
            latest_reviewed_flag_precondition_plan_replay_consistency: None,
            latest_reviewed_flag_readiness_closeout_receipt: None,
            latest_reviewed_flag_readiness_closeout_replay_consistency: None,
            latest_reviewed_flag_audit_chain_closeout_receipt: None,
            readback_ready: true,
            replay_consistency_ready: false,
            closeout_receipt_ready: false,
            closeout_replay_consistency_ready: false,
            audit_chain_receipt_ready: false,
            reviewed_flag_precondition_plan_ready: false,
            reviewed_flag_precondition_plan_replay_consistency_ready: false,
            reviewed_flag_readiness_closeout_receipt_ready: false,
            reviewed_flag_readiness_closeout_replay_consistency_ready: false,
            reviewed_flag_audit_chain_closeout_receipt_ready: false,
        };
        let replay_decision = build_promotion_review_replay_consistency_decision(
            &admission_payload,
            &matrix_payload,
            &packet_payload,
            &readback_before_replay,
        );
        let replay_payload =
            serde_json::to_value(&replay_decision).expect("replay decision should serialize");
        let closeout_readback = AgentJobWorkGraphPromotionReviewReadback {
            promotion_review_replay_consistency_events: 1,
            latest_promotion_review_replay_consistency: Some(replay_payload),
            replay_consistency_ready: true,
            ..readback_before_replay
        };

        let receipt =
            build_promotion_closeout_receipt(&packet, &replay_decision, &closeout_readback);

        assert_eq!(
            receipt.decision,
            "promotion_closeout_receipt_blocked_shadow_no_live_cutover"
        );
        assert!(!receipt.closeout_ready);
        assert!(receipt.shadow_readiness_failed);
        assert!(!receipt.reviewed_but_not_promoted);
        assert!(!receipt.no_cutover_terminal_receipt);
        assert!(receipt.review_ready);
        assert!(!receipt.replay_consistent);
        assert!(
            receipt
                .closeout_blockers
                .iter()
                .any(|blocker| blocker.contains("replay_consistent"))
        );
        assert!(!receipt.promotion_allowed);
        assert!(!receipt.feature_flag_enabled);
        assert!(!receipt.live_cutover_enabled);
    }

    #[test]
    fn promotion_closeout_replay_consistency_accepts_matching_receipt_payload() {
        let registry = default_agent_card_manifest_registry();
        let decision = ready_decision(
            registry
                .manifest_for_source("spawn_agents_on_csv")
                .expect("manifest should resolve"),
        );
        let matrix = build_default_governed_promotion_readiness_shadow_matrix(&[decision]);
        let packet = build_operator_review_promotion_packet(&matrix);
        let admission_payload = json!({ "decision": "allow_shadow_no_live_blocking" });
        let matrix_payload = serde_json::to_value(&matrix).expect("matrix should serialize");
        let packet_payload = serde_json::to_value(&packet).expect("packet should serialize");
        let readback_before_replay = AgentJobWorkGraphPromotionReviewReadback {
            job_id: "job-closeout-replay".to_string(),
            admission_shadow_decision_events: 1,
            promotion_readiness_matrix_events: 1,
            operator_review_promotion_packet_events: 1,
            promotion_review_replay_consistency_events: 0,
            promotion_closeout_receipt_events: 0,
            promotion_closeout_replay_consistency_events: 0,
            promotion_review_audit_chain_receipt_events: 0,
            reviewed_flag_precondition_plan_events: 0,
            reviewed_flag_precondition_plan_replay_consistency_events: 0,
            reviewed_flag_readiness_closeout_receipt_events: 0,
            reviewed_flag_readiness_closeout_replay_consistency_events: 0,
            reviewed_flag_audit_chain_closeout_receipt_events: 0,
            live_blocking_event_count: 0,
            live_cutover_event_count: 0,
            latest_admission_shadow_decision: Some(admission_payload.clone()),
            latest_promotion_readiness_matrix: Some(matrix_payload.clone()),
            latest_operator_review_promotion_packet: Some(packet_payload.clone()),
            latest_promotion_review_replay_consistency: None,
            latest_promotion_closeout_receipt: None,
            latest_promotion_closeout_replay_consistency: None,
            latest_promotion_review_audit_chain_receipt: None,
            latest_reviewed_flag_precondition_plan: None,
            latest_reviewed_flag_precondition_plan_replay_consistency: None,
            latest_reviewed_flag_readiness_closeout_receipt: None,
            latest_reviewed_flag_readiness_closeout_replay_consistency: None,
            latest_reviewed_flag_audit_chain_closeout_receipt: None,
            readback_ready: true,
            replay_consistency_ready: false,
            closeout_receipt_ready: false,
            closeout_replay_consistency_ready: false,
            audit_chain_receipt_ready: false,
            reviewed_flag_precondition_plan_ready: false,
            reviewed_flag_precondition_plan_replay_consistency_ready: false,
            reviewed_flag_readiness_closeout_receipt_ready: false,
            reviewed_flag_readiness_closeout_replay_consistency_ready: false,
            reviewed_flag_audit_chain_closeout_receipt_ready: false,
        };
        let replay_decision = build_promotion_review_replay_consistency_decision(
            &admission_payload,
            &matrix_payload,
            &packet_payload,
            &readback_before_replay,
        );
        let replay_payload =
            serde_json::to_value(&replay_decision).expect("replay decision should serialize");
        let closeout_readback = AgentJobWorkGraphPromotionReviewReadback {
            promotion_review_replay_consistency_events: 1,
            latest_promotion_review_replay_consistency: Some(replay_payload),
            replay_consistency_ready: true,
            ..readback_before_replay
        };
        let closeout_receipt =
            build_promotion_closeout_receipt(&packet, &replay_decision, &closeout_readback);
        let closeout_payload =
            serde_json::to_value(&closeout_receipt).expect("closeout receipt should serialize");
        let closeout_replay_readback = AgentJobWorkGraphPromotionReviewReadback {
            promotion_closeout_receipt_events: 1,
            latest_promotion_closeout_receipt: Some(closeout_payload.clone()),
            closeout_receipt_ready: true,
            ..closeout_readback
        };

        let decision = build_promotion_closeout_replay_consistency_decision(
            &closeout_receipt,
            &closeout_payload,
            &closeout_replay_readback,
        );

        assert_eq!(
            decision.decision,
            "promotion_closeout_replay_consistent_shadow_no_live_cutover"
        );
        assert!(decision.replay_consistent);
        assert!(!decision.shadow_readiness_failed);
        assert!(decision.closeout_receipt_matches);
        assert!(decision.closeout_ready);
        assert!(decision.reviewed_but_not_promoted);
        assert!(decision.no_cutover_terminal_receipt);
        assert!(decision.consistency_blockers.is_empty());
        assert!(!decision.promotion_allowed);
        assert!(!decision.operator_approval_recorded);
        assert!(!decision.approval_record_mutation_enabled);
        assert!(!decision.feature_flag_enabled);
        assert_eq!(decision.canary_stage, "off");
        assert!(!decision.live_cutover_enabled);
    }

    #[test]
    fn promotion_closeout_replay_consistency_fails_shadow_readiness_on_mismatch() {
        let registry = default_agent_card_manifest_registry();
        let decision = ready_decision(
            registry
                .manifest_for_source("spawn_agents_on_csv")
                .expect("manifest should resolve"),
        );
        let matrix = build_default_governed_promotion_readiness_shadow_matrix(&[decision]);
        let packet = build_operator_review_promotion_packet(&matrix);
        let admission_payload = json!({ "decision": "allow_shadow_no_live_blocking" });
        let matrix_payload = serde_json::to_value(&matrix).expect("matrix should serialize");
        let packet_payload = serde_json::to_value(&packet).expect("packet should serialize");
        let readback_before_replay = AgentJobWorkGraphPromotionReviewReadback {
            job_id: "job-closeout-replay-mismatch".to_string(),
            admission_shadow_decision_events: 1,
            promotion_readiness_matrix_events: 1,
            operator_review_promotion_packet_events: 1,
            promotion_review_replay_consistency_events: 0,
            promotion_closeout_receipt_events: 0,
            promotion_closeout_replay_consistency_events: 0,
            promotion_review_audit_chain_receipt_events: 0,
            reviewed_flag_precondition_plan_events: 0,
            reviewed_flag_precondition_plan_replay_consistency_events: 0,
            reviewed_flag_readiness_closeout_receipt_events: 0,
            reviewed_flag_readiness_closeout_replay_consistency_events: 0,
            reviewed_flag_audit_chain_closeout_receipt_events: 0,
            live_blocking_event_count: 0,
            live_cutover_event_count: 0,
            latest_admission_shadow_decision: Some(admission_payload.clone()),
            latest_promotion_readiness_matrix: Some(matrix_payload.clone()),
            latest_operator_review_promotion_packet: Some(packet_payload.clone()),
            latest_promotion_review_replay_consistency: None,
            latest_promotion_closeout_receipt: None,
            latest_promotion_closeout_replay_consistency: None,
            latest_promotion_review_audit_chain_receipt: None,
            latest_reviewed_flag_precondition_plan: None,
            latest_reviewed_flag_precondition_plan_replay_consistency: None,
            latest_reviewed_flag_readiness_closeout_receipt: None,
            latest_reviewed_flag_readiness_closeout_replay_consistency: None,
            latest_reviewed_flag_audit_chain_closeout_receipt: None,
            readback_ready: true,
            replay_consistency_ready: false,
            closeout_receipt_ready: false,
            closeout_replay_consistency_ready: false,
            audit_chain_receipt_ready: false,
            reviewed_flag_precondition_plan_ready: false,
            reviewed_flag_precondition_plan_replay_consistency_ready: false,
            reviewed_flag_readiness_closeout_receipt_ready: false,
            reviewed_flag_readiness_closeout_replay_consistency_ready: false,
            reviewed_flag_audit_chain_closeout_receipt_ready: false,
        };
        let replay_decision = build_promotion_review_replay_consistency_decision(
            &admission_payload,
            &matrix_payload,
            &packet_payload,
            &readback_before_replay,
        );
        let replay_payload =
            serde_json::to_value(&replay_decision).expect("replay decision should serialize");
        let closeout_readback = AgentJobWorkGraphPromotionReviewReadback {
            promotion_review_replay_consistency_events: 1,
            latest_promotion_review_replay_consistency: Some(replay_payload),
            replay_consistency_ready: true,
            ..readback_before_replay
        };
        let closeout_receipt =
            build_promotion_closeout_receipt(&packet, &replay_decision, &closeout_readback);
        let closeout_payload =
            serde_json::to_value(&closeout_receipt).expect("closeout receipt should serialize");
        let closeout_replay_readback = AgentJobWorkGraphPromotionReviewReadback {
            promotion_closeout_receipt_events: 1,
            latest_promotion_closeout_receipt: Some(json!({
                "decision": "stale_closeout_receipt"
            })),
            closeout_receipt_ready: true,
            ..closeout_readback
        };

        let decision = build_promotion_closeout_replay_consistency_decision(
            &closeout_receipt,
            &closeout_payload,
            &closeout_replay_readback,
        );

        assert_eq!(
            decision.decision,
            "promotion_closeout_replay_mismatch_shadow_no_live_cutover"
        );
        assert!(!decision.replay_consistent);
        assert!(decision.shadow_readiness_failed);
        assert!(!decision.closeout_receipt_matches);
        assert!(decision.closeout_ready);
        assert!(decision.reviewed_but_not_promoted);
        assert!(
            decision
                .consistency_blockers
                .iter()
                .any(|blocker| blocker.contains("promotion_closeout_latest_payload_matches"))
        );
        assert!(!decision.promotion_allowed);
        assert!(!decision.feature_flag_enabled);
        assert!(!decision.live_cutover_enabled);
    }

    #[test]
    fn promotion_review_audit_chain_receipt_records_complete_shadow_chain() {
        let registry = default_agent_card_manifest_registry();
        let decision = ready_decision(
            registry
                .manifest_for_source("spawn_agents_on_csv")
                .expect("manifest should resolve"),
        );
        let matrix = build_default_governed_promotion_readiness_shadow_matrix(&[decision]);
        let packet = build_operator_review_promotion_packet(&matrix);
        let admission_payload = json!({ "decision": "allow_shadow_no_live_blocking" });
        let matrix_payload = serde_json::to_value(&matrix).expect("matrix should serialize");
        let packet_payload = serde_json::to_value(&packet).expect("packet should serialize");
        let readback_before_replay = AgentJobWorkGraphPromotionReviewReadback {
            job_id: "job-audit-chain".to_string(),
            admission_shadow_decision_events: 1,
            promotion_readiness_matrix_events: 1,
            operator_review_promotion_packet_events: 1,
            promotion_review_replay_consistency_events: 0,
            promotion_closeout_receipt_events: 0,
            promotion_closeout_replay_consistency_events: 0,
            promotion_review_audit_chain_receipt_events: 0,
            reviewed_flag_precondition_plan_events: 0,
            reviewed_flag_precondition_plan_replay_consistency_events: 0,
            reviewed_flag_readiness_closeout_receipt_events: 0,
            reviewed_flag_readiness_closeout_replay_consistency_events: 0,
            reviewed_flag_audit_chain_closeout_receipt_events: 0,
            live_blocking_event_count: 0,
            live_cutover_event_count: 0,
            latest_admission_shadow_decision: Some(admission_payload.clone()),
            latest_promotion_readiness_matrix: Some(matrix_payload.clone()),
            latest_operator_review_promotion_packet: Some(packet_payload.clone()),
            latest_promotion_review_replay_consistency: None,
            latest_promotion_closeout_receipt: None,
            latest_promotion_closeout_replay_consistency: None,
            latest_promotion_review_audit_chain_receipt: None,
            latest_reviewed_flag_precondition_plan: None,
            latest_reviewed_flag_precondition_plan_replay_consistency: None,
            latest_reviewed_flag_readiness_closeout_receipt: None,
            latest_reviewed_flag_readiness_closeout_replay_consistency: None,
            latest_reviewed_flag_audit_chain_closeout_receipt: None,
            readback_ready: true,
            replay_consistency_ready: false,
            closeout_receipt_ready: false,
            closeout_replay_consistency_ready: false,
            audit_chain_receipt_ready: false,
            reviewed_flag_precondition_plan_ready: false,
            reviewed_flag_precondition_plan_replay_consistency_ready: false,
            reviewed_flag_readiness_closeout_receipt_ready: false,
            reviewed_flag_readiness_closeout_replay_consistency_ready: false,
            reviewed_flag_audit_chain_closeout_receipt_ready: false,
        };
        let replay_decision = build_promotion_review_replay_consistency_decision(
            &admission_payload,
            &matrix_payload,
            &packet_payload,
            &readback_before_replay,
        );
        let replay_payload =
            serde_json::to_value(&replay_decision).expect("replay decision should serialize");
        let closeout_readback = AgentJobWorkGraphPromotionReviewReadback {
            promotion_review_replay_consistency_events: 1,
            latest_promotion_review_replay_consistency: Some(replay_payload),
            replay_consistency_ready: true,
            ..readback_before_replay
        };
        let closeout_receipt =
            build_promotion_closeout_receipt(&packet, &replay_decision, &closeout_readback);
        let closeout_payload =
            serde_json::to_value(&closeout_receipt).expect("closeout receipt should serialize");
        let closeout_replay_readback = AgentJobWorkGraphPromotionReviewReadback {
            promotion_closeout_receipt_events: 1,
            latest_promotion_closeout_receipt: Some(closeout_payload.clone()),
            closeout_receipt_ready: true,
            ..closeout_readback
        };
        let closeout_replay_decision = build_promotion_closeout_replay_consistency_decision(
            &closeout_receipt,
            &closeout_payload,
            &closeout_replay_readback,
        );
        let closeout_replay_payload = serde_json::to_value(&closeout_replay_decision)
            .expect("closeout replay decision should serialize");
        let audit_chain_readback = AgentJobWorkGraphPromotionReviewReadback {
            promotion_closeout_replay_consistency_events: 1,
            latest_promotion_closeout_replay_consistency: Some(closeout_replay_payload),
            closeout_replay_consistency_ready: true,
            ..closeout_replay_readback
        };

        let receipt = build_promotion_review_audit_chain_receipt(
            &closeout_replay_decision,
            &audit_chain_readback,
        );

        assert_eq!(
            receipt.decision,
            "promotion_review_audit_chain_recorded_shadow_no_live_cutover"
        );
        assert!(receipt.terminal_audit_ready);
        assert!(receipt.chain_readback_ready);
        assert!(receipt.admission_ready);
        assert!(receipt.promotion_readiness_matrix_ready);
        assert!(receipt.operator_review_packet_ready);
        assert!(receipt.promotion_review_replay_ready);
        assert!(receipt.promotion_closeout_receipt_ready);
        assert!(receipt.promotion_closeout_replay_ready);
        assert!(receipt.review_replay_consistent);
        assert!(receipt.closeout_replay_consistent);
        assert!(receipt.reviewed_but_not_promoted);
        assert!(receipt.no_cutover_terminal_receipt);
        assert!(!receipt.shadow_readiness_failed);
        assert!(receipt.audit_chain_blockers.is_empty());
        assert!(!receipt.promotion_allowed);
        assert!(!receipt.operator_approval_recorded);
        assert!(!receipt.approval_record_mutation_enabled);
        assert!(!receipt.feature_flag_enabled);
        assert_eq!(receipt.canary_stage, "off");
        assert!(!receipt.live_cutover_enabled);
    }

    #[test]
    fn promotion_review_audit_chain_receipt_blocks_missing_closeout_replay_payload() {
        let closeout_replay_decision = WorkGraphPromotionCloseoutReplayConsistencyDecision {
            decision: "promotion_closeout_replay_consistent_shadow_no_live_cutover",
            replay_stage: "promotion_closeout_replay_shadow_only",
            job_id: "job-audit-chain-missing".to_string(),
            closeout_receipt_decision: "promotion_closeout_receipt_recorded_shadow_no_live_cutover",
            closeout_receipt_events: 1,
            closeout_receipt_ready: true,
            closeout_receipt_matches: true,
            closeout_ready: true,
            reviewed_but_not_promoted: true,
            no_cutover_terminal_receipt: true,
            replay_consistent: true,
            shadow_readiness_failed: false,
            live_blocking_event_count: 0,
            live_cutover_event_count: 0,
            consistency_blockers: Vec::new(),
            checks: Vec::new(),
            operator_approval_recorded: false,
            approval_record_mutation_enabled: false,
            promotion_allowed: false,
            promotion_prohibited_reason: "closeout replay consistency is report-only; separate reviewed flag path required",
            feature_flag_id: "work_graph_promotion_closeout_replay_consistency_shadow_only",
            feature_flag_enabled: false,
            canary_stage: "off",
            canary_traffic_ppm: 0,
            blocking_guardrail_preview: true,
            live_blocking_enabled: false,
            live_cutover_enabled: false,
        };
        let readback = AgentJobWorkGraphPromotionReviewReadback {
            job_id: "job-audit-chain-missing".to_string(),
            admission_shadow_decision_events: 1,
            promotion_readiness_matrix_events: 1,
            operator_review_promotion_packet_events: 1,
            promotion_review_replay_consistency_events: 1,
            promotion_closeout_receipt_events: 1,
            promotion_closeout_replay_consistency_events: 0,
            promotion_review_audit_chain_receipt_events: 0,
            reviewed_flag_precondition_plan_events: 0,
            reviewed_flag_precondition_plan_replay_consistency_events: 0,
            reviewed_flag_readiness_closeout_receipt_events: 0,
            reviewed_flag_readiness_closeout_replay_consistency_events: 0,
            reviewed_flag_audit_chain_closeout_receipt_events: 0,
            live_blocking_event_count: 0,
            live_cutover_event_count: 0,
            latest_admission_shadow_decision: Some(json!({
                "decision": "allow_shadow_no_live_blocking"
            })),
            latest_promotion_readiness_matrix: Some(json!({
                "decision": "promotion_matrix_not_ready_shadow_no_live_cutover"
            })),
            latest_operator_review_promotion_packet: Some(json!({
                "decision": "operator_review_packet_blocked_shadow_no_live_cutover"
            })),
            latest_promotion_review_replay_consistency: Some(json!({
                "decision": "promotion_review_replay_consistent_shadow_no_live_cutover",
                "replayConsistent": true
            })),
            latest_promotion_closeout_receipt: Some(json!({
                "decision": "promotion_closeout_receipt_recorded_shadow_no_live_cutover"
            })),
            latest_promotion_closeout_replay_consistency: None,
            latest_promotion_review_audit_chain_receipt: None,
            latest_reviewed_flag_precondition_plan: None,
            latest_reviewed_flag_precondition_plan_replay_consistency: None,
            latest_reviewed_flag_readiness_closeout_receipt: None,
            latest_reviewed_flag_readiness_closeout_replay_consistency: None,
            latest_reviewed_flag_audit_chain_closeout_receipt: None,
            readback_ready: true,
            replay_consistency_ready: true,
            closeout_receipt_ready: true,
            closeout_replay_consistency_ready: false,
            audit_chain_receipt_ready: false,
            reviewed_flag_precondition_plan_ready: false,
            reviewed_flag_precondition_plan_replay_consistency_ready: false,
            reviewed_flag_readiness_closeout_receipt_ready: false,
            reviewed_flag_readiness_closeout_replay_consistency_ready: false,
            reviewed_flag_audit_chain_closeout_receipt_ready: false,
        };

        let receipt =
            build_promotion_review_audit_chain_receipt(&closeout_replay_decision, &readback);

        assert_eq!(
            receipt.decision,
            "promotion_review_audit_chain_blocked_shadow_no_live_cutover"
        );
        assert!(!receipt.terminal_audit_ready);
        assert!(!receipt.chain_readback_ready);
        assert!(!receipt.promotion_closeout_replay_ready);
        assert!(receipt.shadow_readiness_failed);
        assert!(
            receipt
                .audit_chain_blockers
                .iter()
                .any(|blocker| blocker.contains("promotion_closeout_replay_readback_ready"))
        );
        assert!(!receipt.promotion_allowed);
        assert!(!receipt.feature_flag_enabled);
        assert!(!receipt.live_cutover_enabled);
    }

    fn ready_audit_chain_receipt() -> WorkGraphPromotionReviewAuditChainReceipt {
        WorkGraphPromotionReviewAuditChainReceipt {
            decision: "promotion_review_audit_chain_recorded_shadow_no_live_cutover",
            audit_stage: "terminal_promotion_review_audit_chain_shadow_only",
            job_id: "job-reviewed-flag-plan".to_string(),
            admission_shadow_decision_events: 1,
            promotion_readiness_matrix_events: 1,
            operator_review_promotion_packet_events: 1,
            promotion_review_replay_consistency_events: 1,
            promotion_closeout_receipt_events: 1,
            promotion_closeout_replay_consistency_events: 1,
            admission_ready: true,
            promotion_readiness_matrix_ready: true,
            operator_review_packet_ready: true,
            promotion_review_replay_ready: true,
            promotion_closeout_receipt_ready: true,
            promotion_closeout_replay_ready: true,
            chain_readback_ready: true,
            terminal_audit_ready: true,
            review_replay_consistent: true,
            closeout_replay_consistent: true,
            reviewed_but_not_promoted: true,
            no_cutover_terminal_receipt: true,
            shadow_readiness_failed: false,
            admission_shadow_decision: "allow_shadow_no_live_blocking".to_string(),
            promotion_readiness_matrix_decision:
                "promotion_matrix_not_ready_shadow_no_live_cutover".to_string(),
            operator_review_packet_decision:
                "operator_review_packet_blocked_shadow_no_live_cutover".to_string(),
            promotion_review_replay_consistency_decision:
                "promotion_review_replay_consistent_shadow_no_live_cutover".to_string(),
            promotion_closeout_receipt_decision:
                "promotion_closeout_receipt_recorded_shadow_no_live_cutover".to_string(),
            promotion_closeout_replay_consistency_decision:
                "promotion_closeout_replay_consistent_shadow_no_live_cutover".to_string(),
            live_blocking_event_count: 0,
            live_cutover_event_count: 0,
            audit_chain_blockers: Vec::new(),
            checks: Vec::new(),
            operator_approval_recorded: false,
            approval_record_mutation_enabled: false,
            promotion_allowed: false,
            promotion_prohibited_reason: "terminal audit chain is report-only; separate reviewed flag path required",
            feature_flag_id: "work_graph_promotion_review_audit_chain_shadow_only",
            feature_flag_enabled: false,
            canary_stage: "off",
            canary_traffic_ppm: 0,
            blocking_guardrail_preview: true,
            live_blocking_enabled: false,
            live_cutover_enabled: false,
        }
    }

    fn readback_with_audit_chain_receipt(
        audit_chain_receipt_ready: bool,
    ) -> AgentJobWorkGraphPromotionReviewReadback {
        AgentJobWorkGraphPromotionReviewReadback {
            job_id: "job-reviewed-flag-plan".to_string(),
            admission_shadow_decision_events: 1,
            promotion_readiness_matrix_events: 1,
            operator_review_promotion_packet_events: 1,
            promotion_review_replay_consistency_events: 1,
            promotion_closeout_receipt_events: 1,
            promotion_closeout_replay_consistency_events: 1,
            promotion_review_audit_chain_receipt_events: usize::from(audit_chain_receipt_ready),
            reviewed_flag_precondition_plan_events: 0,
            reviewed_flag_precondition_plan_replay_consistency_events: 0,
            reviewed_flag_readiness_closeout_receipt_events: 0,
            reviewed_flag_readiness_closeout_replay_consistency_events: 0,
            reviewed_flag_audit_chain_closeout_receipt_events: 0,
            live_blocking_event_count: 0,
            live_cutover_event_count: 0,
            latest_admission_shadow_decision: Some(json!({
                "decision": "allow_shadow_no_live_blocking"
            })),
            latest_promotion_readiness_matrix: Some(json!({
                "decision": "promotion_matrix_not_ready_shadow_no_live_cutover"
            })),
            latest_operator_review_promotion_packet: Some(json!({
                "decision": "operator_review_packet_blocked_shadow_no_live_cutover"
            })),
            latest_promotion_review_replay_consistency: Some(json!({
                "decision": "promotion_review_replay_consistent_shadow_no_live_cutover",
                "replayConsistent": true
            })),
            latest_promotion_closeout_receipt: Some(json!({
                "decision": "promotion_closeout_receipt_recorded_shadow_no_live_cutover"
            })),
            latest_promotion_closeout_replay_consistency: Some(json!({
                "decision": "promotion_closeout_replay_consistent_shadow_no_live_cutover"
            })),
            latest_promotion_review_audit_chain_receipt: audit_chain_receipt_ready.then(|| {
                json!({
                    "decision": "promotion_review_audit_chain_recorded_shadow_no_live_cutover",
                    "terminalAuditReady": true
                })
            }),
            latest_reviewed_flag_precondition_plan: None,
            latest_reviewed_flag_precondition_plan_replay_consistency: None,
            latest_reviewed_flag_readiness_closeout_receipt: None,
            latest_reviewed_flag_readiness_closeout_replay_consistency: None,
            latest_reviewed_flag_audit_chain_closeout_receipt: None,
            readback_ready: true,
            replay_consistency_ready: true,
            closeout_receipt_ready: true,
            closeout_replay_consistency_ready: true,
            audit_chain_receipt_ready,
            reviewed_flag_precondition_plan_ready: false,
            reviewed_flag_precondition_plan_replay_consistency_ready: false,
            reviewed_flag_readiness_closeout_receipt_ready: false,
            reviewed_flag_readiness_closeout_replay_consistency_ready: false,
            reviewed_flag_audit_chain_closeout_receipt_ready: false,
        }
    }

    fn ready_reviewed_flag_readiness_closeout_replay() -> (
        WorkGraphReviewedFlagReadinessCloseoutReplayConsistencyDecision,
        AgentJobWorkGraphPromotionReviewReadback,
    ) {
        let audit_chain_receipt = ready_audit_chain_receipt();
        let initial_readback = readback_with_audit_chain_receipt(true);
        let plan =
            build_reviewed_flag_precondition_plan_packet(&audit_chain_receipt, &initial_readback);
        let plan_payload = serde_json::to_value(&plan).expect("plan should serialize");
        let mut plan_replay_readback = readback_with_audit_chain_receipt(true);
        plan_replay_readback.reviewed_flag_precondition_plan_events = 1;
        plan_replay_readback.latest_reviewed_flag_precondition_plan = Some(plan_payload.clone());
        plan_replay_readback.reviewed_flag_precondition_plan_ready = true;
        let plan_replay = build_reviewed_flag_precondition_plan_replay_consistency_decision(
            &plan,
            &plan_payload,
            &plan_replay_readback,
        );
        let mut closeout_readback = plan_replay_readback;
        closeout_readback.reviewed_flag_precondition_plan_replay_consistency_events = 1;
        closeout_readback.latest_reviewed_flag_precondition_plan_replay_consistency =
            Some(serde_json::to_value(&plan_replay).expect("plan replay should serialize"));
        closeout_readback.reviewed_flag_precondition_plan_replay_consistency_ready = true;
        let closeout =
            build_reviewed_flag_readiness_closeout_receipt(&plan, &plan_replay, &closeout_readback);
        let closeout_payload = serde_json::to_value(&closeout).expect("closeout should serialize");
        let mut closeout_replay_readback = closeout_readback;
        closeout_replay_readback.reviewed_flag_readiness_closeout_receipt_events = 1;
        closeout_replay_readback.latest_reviewed_flag_readiness_closeout_receipt =
            Some(closeout_payload.clone());
        closeout_replay_readback.reviewed_flag_readiness_closeout_receipt_ready = true;
        let closeout_replay = build_reviewed_flag_readiness_closeout_replay_consistency_decision(
            &closeout,
            &closeout_payload,
            &closeout_replay_readback,
        );
        let mut audit_readback = closeout_replay_readback;
        audit_readback.reviewed_flag_readiness_closeout_replay_consistency_events = 1;
        audit_readback.latest_reviewed_flag_readiness_closeout_replay_consistency =
            Some(serde_json::to_value(&closeout_replay).expect("closeout replay should serialize"));
        audit_readback.reviewed_flag_readiness_closeout_replay_consistency_ready = true;

        (closeout_replay, audit_readback)
    }

    #[test]
    fn reviewed_flag_precondition_plan_records_shadow_only_missing_live_prerequisites() {
        let audit_chain_receipt = ready_audit_chain_receipt();
        let readback = readback_with_audit_chain_receipt(true);

        let plan = build_reviewed_flag_precondition_plan_packet(&audit_chain_receipt, &readback);

        assert_eq!(
            plan.decision,
            "reviewed_flag_precondition_plan_recorded_shadow_no_live_cutover"
        );
        assert!(plan.dry_run_plan_ready);
        assert!(!plan.shadow_readiness_failed);
        assert!(plan.reviewed_but_not_promoted);
        assert!(plan.no_cutover_terminal_receipt);
        assert!(plan.reviewed_flag_mutation_dry_run);
        assert!(!plan.reviewed_flag_mutation_enabled);
        assert!(!plan.operator_approval_recorded);
        assert!(!plan.approval_record_mutation_enabled);
        assert!(!plan.promotion_allowed);
        assert_eq!(plan.canary_stage, "off");
        assert_eq!(plan.canary_traffic_ppm, 0);
        assert!(!plan.live_blocking_enabled);
        assert!(!plan.live_cutover_enabled);
        assert!(
            plan.missing_live_promotion_prerequisites
                .contains(&"reviewed_flag_mutation_path".to_string())
        );
        assert!(
            plan.missing_live_promotion_prerequisites
                .contains(&"operator_approval_recording_path".to_string())
        );
        assert_eq!(
            plan.missing_live_promotion_prerequisite_count,
            plan.missing_live_promotion_prerequisites.len()
        );
        assert!(plan.plan_blockers.is_empty());
    }

    #[test]
    fn reviewed_flag_precondition_plan_blocks_without_terminal_audit_chain() {
        let audit_chain_receipt = WorkGraphPromotionReviewAuditChainReceipt {
            terminal_audit_ready: false,
            chain_readback_ready: false,
            shadow_readiness_failed: true,
            ..ready_audit_chain_receipt()
        };
        let readback = readback_with_audit_chain_receipt(false);

        let plan = build_reviewed_flag_precondition_plan_packet(&audit_chain_receipt, &readback);

        assert_eq!(
            plan.decision,
            "reviewed_flag_precondition_plan_blocked_shadow_no_live_cutover"
        );
        assert!(!plan.dry_run_plan_ready);
        assert!(plan.shadow_readiness_failed);
        assert!(!plan.audit_chain_receipt_ready);
        assert!(!plan.terminal_audit_ready);
        assert!(
            plan.plan_blockers
                .iter()
                .any(|blocker| blocker.contains("terminal_audit_chain_receipt_ready"))
        );
        assert!(!plan.reviewed_flag_mutation_enabled);
        assert!(!plan.promotion_allowed);
        assert!(!plan.live_cutover_enabled);
    }

    #[test]
    fn reviewed_flag_precondition_plan_replay_accepts_matching_plan_payload() {
        let audit_chain_receipt = ready_audit_chain_receipt();
        let initial_readback = readback_with_audit_chain_receipt(true);
        let plan =
            build_reviewed_flag_precondition_plan_packet(&audit_chain_receipt, &initial_readback);
        let plan_payload = serde_json::to_value(&plan).expect("plan should serialize");
        let mut replay_readback = readback_with_audit_chain_receipt(true);
        replay_readback.reviewed_flag_precondition_plan_events = 1;
        replay_readback.latest_reviewed_flag_precondition_plan = Some(plan_payload.clone());
        replay_readback.reviewed_flag_precondition_plan_ready = true;

        let replay = build_reviewed_flag_precondition_plan_replay_consistency_decision(
            &plan,
            &plan_payload,
            &replay_readback,
        );

        assert_eq!(
            replay.decision,
            "reviewed_flag_precondition_plan_replay_consistent_shadow_no_live_cutover"
        );
        assert!(replay.reviewed_flag_precondition_plan_ready);
        assert!(replay.reviewed_flag_precondition_plan_matches);
        assert!(replay.dry_run_plan_ready);
        assert!(replay.replay_consistent);
        assert!(!replay.shadow_readiness_failed);
        assert!(replay.consistency_blockers.is_empty());
        assert!(replay.reviewed_flag_mutation_dry_run);
        assert!(!replay.reviewed_flag_mutation_enabled);
        assert!(!replay.promotion_allowed);
        assert!(!replay.operator_approval_recorded);
        assert!(!replay.approval_record_mutation_enabled);
        assert_eq!(replay.canary_stage, "off");
        assert_eq!(replay.canary_traffic_ppm, 0);
        assert!(!replay.live_blocking_enabled);
        assert!(!replay.live_cutover_enabled);
    }

    #[test]
    fn reviewed_flag_precondition_plan_replay_fails_shadow_readiness_on_mismatch() {
        let audit_chain_receipt = ready_audit_chain_receipt();
        let initial_readback = readback_with_audit_chain_receipt(true);
        let plan =
            build_reviewed_flag_precondition_plan_packet(&audit_chain_receipt, &initial_readback);
        let plan_payload = serde_json::to_value(&plan).expect("plan should serialize");
        let mut replay_readback = readback_with_audit_chain_receipt(true);
        replay_readback.reviewed_flag_precondition_plan_events = 1;
        replay_readback.latest_reviewed_flag_precondition_plan = Some(json!({
            "decision": "reviewed_flag_precondition_plan_recorded_shadow_no_live_cutover",
            "dryRunPlanReady": false
        }));
        replay_readback.reviewed_flag_precondition_plan_ready = true;

        let replay = build_reviewed_flag_precondition_plan_replay_consistency_decision(
            &plan,
            &plan_payload,
            &replay_readback,
        );

        assert_eq!(
            replay.decision,
            "reviewed_flag_precondition_plan_replay_mismatch_shadow_no_live_cutover"
        );
        assert!(replay.reviewed_flag_precondition_plan_ready);
        assert!(!replay.reviewed_flag_precondition_plan_matches);
        assert!(!replay.replay_consistent);
        assert!(replay.shadow_readiness_failed);
        assert!(replay.consistency_blockers.iter().any(|blocker| {
            blocker.contains("reviewed_flag_precondition_plan_latest_payload_matches")
        }));
        assert!(!replay.reviewed_flag_mutation_enabled);
        assert!(!replay.promotion_allowed);
        assert!(!replay.live_cutover_enabled);
    }

    #[test]
    fn reviewed_flag_readiness_closeout_records_planned_but_not_mutable_receipt() {
        let audit_chain_receipt = ready_audit_chain_receipt();
        let initial_readback = readback_with_audit_chain_receipt(true);
        let plan =
            build_reviewed_flag_precondition_plan_packet(&audit_chain_receipt, &initial_readback);
        let plan_payload = serde_json::to_value(&plan).expect("plan should serialize");
        let mut replay_readback = readback_with_audit_chain_receipt(true);
        replay_readback.reviewed_flag_precondition_plan_events = 1;
        replay_readback.latest_reviewed_flag_precondition_plan = Some(plan_payload.clone());
        replay_readback.reviewed_flag_precondition_plan_ready = true;
        let replay = build_reviewed_flag_precondition_plan_replay_consistency_decision(
            &plan,
            &plan_payload,
            &replay_readback,
        );
        let mut closeout_readback = replay_readback;
        closeout_readback.reviewed_flag_precondition_plan_replay_consistency_events = 1;
        closeout_readback.latest_reviewed_flag_precondition_plan_replay_consistency =
            Some(serde_json::to_value(&replay).expect("replay should serialize"));
        closeout_readback.reviewed_flag_precondition_plan_replay_consistency_ready = true;

        let closeout =
            build_reviewed_flag_readiness_closeout_receipt(&plan, &replay, &closeout_readback);

        assert_eq!(
            closeout.decision,
            "reviewed_flag_readiness_closeout_recorded_shadow_no_live_cutover"
        );
        assert!(closeout.reviewed_flag_precondition_plan_ready);
        assert!(closeout.reviewed_flag_precondition_plan_replay_consistency_ready);
        assert!(closeout.dry_run_plan_ready);
        assert!(closeout.replay_consistent);
        assert!(closeout.planned_but_not_mutable);
        assert!(closeout.terminal_closeout_ready);
        assert!(!closeout.shadow_readiness_failed);
        assert!(closeout.closeout_blockers.is_empty());
        assert!(closeout.reviewed_flag_mutation_dry_run);
        assert!(!closeout.reviewed_flag_mutation_enabled);
        assert!(!closeout.promotion_allowed);
        assert!(!closeout.operator_approval_recorded);
        assert!(!closeout.approval_record_mutation_enabled);
        assert_eq!(closeout.canary_stage, "off");
        assert_eq!(closeout.canary_traffic_ppm, 0);
        assert!(!closeout.live_blocking_enabled);
        assert!(!closeout.live_cutover_enabled);
    }

    #[test]
    fn reviewed_flag_readiness_closeout_blocks_on_plan_replay_mismatch() {
        let audit_chain_receipt = ready_audit_chain_receipt();
        let initial_readback = readback_with_audit_chain_receipt(true);
        let plan =
            build_reviewed_flag_precondition_plan_packet(&audit_chain_receipt, &initial_readback);
        let plan_payload = serde_json::to_value(&plan).expect("plan should serialize");
        let mut replay_readback = readback_with_audit_chain_receipt(true);
        replay_readback.reviewed_flag_precondition_plan_events = 1;
        replay_readback.latest_reviewed_flag_precondition_plan = Some(json!({
            "decision": "reviewed_flag_precondition_plan_recorded_shadow_no_live_cutover",
            "dryRunPlanReady": false
        }));
        replay_readback.reviewed_flag_precondition_plan_ready = true;
        let replay = build_reviewed_flag_precondition_plan_replay_consistency_decision(
            &plan,
            &plan_payload,
            &replay_readback,
        );
        let mut closeout_readback = replay_readback;
        closeout_readback.reviewed_flag_precondition_plan_replay_consistency_events = 1;
        closeout_readback.latest_reviewed_flag_precondition_plan_replay_consistency =
            Some(serde_json::to_value(&replay).expect("replay should serialize"));
        closeout_readback.reviewed_flag_precondition_plan_replay_consistency_ready = true;

        let closeout =
            build_reviewed_flag_readiness_closeout_receipt(&plan, &replay, &closeout_readback);

        assert_eq!(
            closeout.decision,
            "reviewed_flag_readiness_closeout_blocked_shadow_no_live_cutover"
        );
        assert!(closeout.reviewed_flag_precondition_plan_replay_consistency_ready);
        assert!(!closeout.replay_consistent);
        assert!(!closeout.planned_but_not_mutable);
        assert!(!closeout.terminal_closeout_ready);
        assert!(closeout.shadow_readiness_failed);
        assert!(closeout.closeout_blockers.iter().any(|blocker| {
            blocker.contains("reviewed_flag_precondition_plan_replay_consistent")
        }));
        assert!(!closeout.reviewed_flag_mutation_enabled);
        assert!(!closeout.promotion_allowed);
        assert!(!closeout.live_cutover_enabled);
    }

    #[test]
    fn reviewed_flag_readiness_closeout_replay_accepts_matching_receipt_payload() {
        let audit_chain_receipt = ready_audit_chain_receipt();
        let initial_readback = readback_with_audit_chain_receipt(true);
        let plan =
            build_reviewed_flag_precondition_plan_packet(&audit_chain_receipt, &initial_readback);
        let plan_payload = serde_json::to_value(&plan).expect("plan should serialize");
        let mut plan_replay_readback = readback_with_audit_chain_receipt(true);
        plan_replay_readback.reviewed_flag_precondition_plan_events = 1;
        plan_replay_readback.latest_reviewed_flag_precondition_plan = Some(plan_payload.clone());
        plan_replay_readback.reviewed_flag_precondition_plan_ready = true;
        let plan_replay = build_reviewed_flag_precondition_plan_replay_consistency_decision(
            &plan,
            &plan_payload,
            &plan_replay_readback,
        );
        let mut closeout_readback = plan_replay_readback;
        closeout_readback.reviewed_flag_precondition_plan_replay_consistency_events = 1;
        closeout_readback.latest_reviewed_flag_precondition_plan_replay_consistency =
            Some(serde_json::to_value(&plan_replay).expect("plan replay should serialize"));
        closeout_readback.reviewed_flag_precondition_plan_replay_consistency_ready = true;
        let closeout =
            build_reviewed_flag_readiness_closeout_receipt(&plan, &plan_replay, &closeout_readback);
        let closeout_payload = serde_json::to_value(&closeout).expect("closeout should serialize");
        let mut replay_readback = closeout_readback;
        replay_readback.reviewed_flag_readiness_closeout_receipt_events = 1;
        replay_readback.latest_reviewed_flag_readiness_closeout_receipt =
            Some(closeout_payload.clone());
        replay_readback.reviewed_flag_readiness_closeout_receipt_ready = true;

        let replay = build_reviewed_flag_readiness_closeout_replay_consistency_decision(
            &closeout,
            &closeout_payload,
            &replay_readback,
        );

        assert_eq!(
            replay.decision,
            "reviewed_flag_readiness_closeout_replay_consistent_shadow_no_live_cutover"
        );
        assert!(replay.reviewed_flag_readiness_closeout_receipt_ready);
        assert!(replay.reviewed_flag_readiness_closeout_receipt_matches);
        assert!(replay.planned_but_not_mutable);
        assert!(replay.terminal_closeout_ready);
        assert!(replay.missing_live_promotion_prerequisite_count > 0);
        assert!(replay.replay_consistent);
        assert!(!replay.shadow_readiness_failed);
        assert!(replay.consistency_blockers.is_empty());
        assert!(replay.reviewed_flag_mutation_dry_run);
        assert!(!replay.reviewed_flag_mutation_enabled);
        assert!(!replay.promotion_allowed);
        assert!(!replay.operator_approval_recorded);
        assert!(!replay.approval_record_mutation_enabled);
        assert_eq!(replay.canary_stage, "off");
        assert_eq!(replay.canary_traffic_ppm, 0);
        assert!(!replay.live_blocking_enabled);
        assert!(!replay.live_cutover_enabled);
    }

    #[test]
    fn reviewed_flag_readiness_closeout_replay_fails_shadow_readiness_on_mismatch() {
        let audit_chain_receipt = ready_audit_chain_receipt();
        let initial_readback = readback_with_audit_chain_receipt(true);
        let plan =
            build_reviewed_flag_precondition_plan_packet(&audit_chain_receipt, &initial_readback);
        let plan_payload = serde_json::to_value(&plan).expect("plan should serialize");
        let mut plan_replay_readback = readback_with_audit_chain_receipt(true);
        plan_replay_readback.reviewed_flag_precondition_plan_events = 1;
        plan_replay_readback.latest_reviewed_flag_precondition_plan = Some(plan_payload.clone());
        plan_replay_readback.reviewed_flag_precondition_plan_ready = true;
        let plan_replay = build_reviewed_flag_precondition_plan_replay_consistency_decision(
            &plan,
            &plan_payload,
            &plan_replay_readback,
        );
        let mut closeout_readback = plan_replay_readback;
        closeout_readback.reviewed_flag_precondition_plan_replay_consistency_events = 1;
        closeout_readback.latest_reviewed_flag_precondition_plan_replay_consistency =
            Some(serde_json::to_value(&plan_replay).expect("plan replay should serialize"));
        closeout_readback.reviewed_flag_precondition_plan_replay_consistency_ready = true;
        let closeout =
            build_reviewed_flag_readiness_closeout_receipt(&plan, &plan_replay, &closeout_readback);
        let closeout_payload = serde_json::to_value(&closeout).expect("closeout should serialize");
        let mut replay_readback = closeout_readback;
        replay_readback.reviewed_flag_readiness_closeout_receipt_events = 1;
        replay_readback.latest_reviewed_flag_readiness_closeout_receipt = Some(json!({
            "decision": "reviewed_flag_readiness_closeout_recorded_shadow_no_live_cutover",
            "plannedButNotMutable": false
        }));
        replay_readback.reviewed_flag_readiness_closeout_receipt_ready = true;

        let replay = build_reviewed_flag_readiness_closeout_replay_consistency_decision(
            &closeout,
            &closeout_payload,
            &replay_readback,
        );

        assert_eq!(
            replay.decision,
            "reviewed_flag_readiness_closeout_replay_mismatch_shadow_no_live_cutover"
        );
        assert!(replay.reviewed_flag_readiness_closeout_receipt_ready);
        assert!(!replay.reviewed_flag_readiness_closeout_receipt_matches);
        assert!(!replay.replay_consistent);
        assert!(replay.shadow_readiness_failed);
        assert!(replay.consistency_blockers.iter().any(|blocker| {
            blocker.contains("reviewed_flag_readiness_closeout_latest_payload_matches")
        }));
        assert!(!replay.reviewed_flag_mutation_enabled);
        assert!(!replay.promotion_allowed);
        assert!(!replay.live_cutover_enabled);
    }

    #[test]
    fn reviewed_flag_audit_chain_closeout_records_terminal_shadow_bundle() {
        let (closeout_replay, readback) = ready_reviewed_flag_readiness_closeout_replay();

        let receipt = build_reviewed_flag_audit_chain_closeout_receipt(&closeout_replay, &readback);

        assert_eq!(
            receipt.decision,
            "reviewed_flag_audit_chain_closeout_recorded_shadow_no_live_cutover"
        );
        assert!(receipt.reviewed_flag_precondition_plan_ready);
        assert!(receipt.reviewed_flag_precondition_plan_replay_consistency_ready);
        assert!(receipt.reviewed_flag_readiness_closeout_receipt_ready);
        assert!(receipt.reviewed_flag_readiness_closeout_replay_consistency_ready);
        assert!(receipt.reviewed_flag_chain_readback_ready);
        assert!(receipt.terminal_reviewed_flag_audit_ready);
        assert!(receipt.dry_run_plan_ready);
        assert!(receipt.plan_replay_consistent);
        assert!(receipt.readiness_closeout_ready);
        assert!(receipt.readiness_closeout_replay_consistent);
        assert!(receipt.planned_but_not_mutable);
        assert!(receipt.missing_live_promotion_prerequisite_count > 0);
        assert!(!receipt.shadow_readiness_failed);
        assert!(receipt.audit_chain_blockers.is_empty());
        assert!(receipt.reviewed_flag_mutation_dry_run);
        assert!(!receipt.reviewed_flag_mutation_enabled);
        assert!(!receipt.promotion_allowed);
        assert!(!receipt.operator_approval_recorded);
        assert!(!receipt.approval_record_mutation_enabled);
        assert_eq!(receipt.canary_stage, "off");
        assert_eq!(receipt.canary_traffic_ppm, 0);
        assert!(!receipt.live_blocking_enabled);
        assert!(!receipt.live_cutover_enabled);
    }

    #[test]
    fn reviewed_flag_audit_chain_closeout_blocks_missing_readiness_closeout_replay() {
        let (closeout_replay, mut readback) = ready_reviewed_flag_readiness_closeout_replay();
        readback.reviewed_flag_readiness_closeout_replay_consistency_events = 0;
        readback.latest_reviewed_flag_readiness_closeout_replay_consistency = None;
        readback.reviewed_flag_readiness_closeout_replay_consistency_ready = false;

        let receipt = build_reviewed_flag_audit_chain_closeout_receipt(&closeout_replay, &readback);

        assert_eq!(
            receipt.decision,
            "reviewed_flag_audit_chain_closeout_blocked_shadow_no_live_cutover"
        );
        assert!(receipt.reviewed_flag_readiness_closeout_receipt_ready);
        assert!(!receipt.reviewed_flag_readiness_closeout_replay_consistency_ready);
        assert!(!receipt.reviewed_flag_chain_readback_ready);
        assert!(!receipt.terminal_reviewed_flag_audit_ready);
        assert!(receipt.readiness_closeout_replay_consistent);
        assert!(receipt.shadow_readiness_failed);
        assert!(receipt.audit_chain_blockers.iter().any(|blocker| {
            blocker.contains("reviewed_flag_readiness_closeout_replay_readback_ready")
        }));
        assert!(!receipt.reviewed_flag_mutation_enabled);
        assert!(!receipt.promotion_allowed);
        assert!(!receipt.live_cutover_enabled);
    }
}
