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

mod constants;
mod helpers;
mod types;

use constants::*;
use helpers::*;
pub(crate) use types::*;

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

#[cfg(test)]
mod tests;
