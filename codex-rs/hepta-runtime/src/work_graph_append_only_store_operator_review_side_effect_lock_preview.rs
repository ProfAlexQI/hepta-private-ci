use serde::Serialize;

use crate::work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview::{
    WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_APPLICATION_PROMOTION_RERUN_PREVIEW_GATE,
    WorkGraphRuntimeApplicationPromotionRerunResidualBlockerPreview,
    WorkGraphRuntimeApplicationPromotionRerunSourceDecisionPreview,
    work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_required_prior_gates,
    work_graph_unified_projection_enforcement_runtime_application_promotion_rerun_residual_blockers,
    work_graph_unified_projection_enforcement_runtime_application_promotion_rerun_source_decisions,
};

pub const WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_store_operator_review_side_effect_lock_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_SCHEMA_VERSION: &str =
    "work_graph_append_only_store_operator_review_side_effect_lock_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_append_only_store_operator_review_side_effect_lock_readback_preview_gate";

const OPERATOR_REVIEW_EVIDENCE_FIELD_IDS: [&str; 8] = [
    "source_surface_id",
    "source_category",
    "runtime_application_promotion_rerun_decision",
    "operator_review_packet_id",
    "side_effect_lock_plan_id",
    "approval_evidence_boundary_id",
    "readback_boundary_id",
    "residual_source_blocker_ids",
];

const OPERATOR_REVIEW_PACKET_SECTION_IDS: [&str; 5] = [
    "runtime_application_contract_summary",
    "operator_review_scope",
    "side_effect_lock_scope",
    "write_boundary_residuals",
    "no_mutation_guard_evidence",
];

const SIDE_EFFECT_LOCK_SCOPE_IDS: [&str; 5] = [
    "runtime_application_promotion",
    "wal_write_boundary",
    "durable_store_runtime_switch",
    "idempotency_mutation_policy",
    "rollback_readback_execution",
];

const OPERATOR_REVIEW_GROUPS: [(&str, &str, &str); 4] = [
    ("multi_agent", "multi_agent_operator_review_group", "p0"),
    (
        "batch_agent_jobs",
        "batch_agent_jobs_operator_review_group",
        "p1",
    ),
    (
        "runtime_scheduler",
        "runtime_scheduler_operator_review_group",
        "p0",
    ),
    (
        "external_handoff",
        "external_handoff_operator_review_group",
        "p1",
    ),
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub upstream_runtime_application_promotion_rerun_gate: &'static str,
    pub upstream_operator_review_residual_source_count: usize,
    pub upstream_side_effect_lock_residual_source_count: usize,
    pub upstream_write_boundary_primary_blocked_source_count: usize,
    pub operator_review_packet_count: usize,
    pub side_effect_lock_plan_count: usize,
    pub approval_evidence_boundary_count: usize,
    pub readback_boundary_count: usize,
    pub evidence_field_ref_count: usize,
    pub operator_review_group_count: usize,
    pub guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub operator_review_packets: Vec<WorkGraphOperatorReviewPacketPreview>,
    pub side_effect_lock_plans: Vec<WorkGraphSideEffectLockPlanPreview>,
    pub approval_evidence_boundaries: Vec<WorkGraphApprovalEvidenceBoundaryPreview>,
    pub readback_boundaries: Vec<WorkGraphOperatorReviewReadbackBoundaryPreview>,
    pub operator_review_groups: Vec<WorkGraphOperatorReviewGroupPreview>,
    pub guards: Vec<WorkGraphOperatorReviewSideEffectLockGuardPreview>,
    pub blockers: Vec<WorkGraphOperatorReviewSideEffectLockBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_operator_review_side_effect_lock_readback_preview: bool,
    pub ready_for_operator_review_side_effect_lock_application_preview: bool,
    pub ready_for_operator_review_recording: bool,
    pub ready_for_side_effect_lock_establishment: bool,
    pub ready_for_runtime_write_boundary_preview: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewPacketPreview {
    pub packet_id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub runtime_application_promotion_rerun_decision: &'static str,
    pub required_section_ids: Vec<&'static str>,
    pub evidence_field_ids: Vec<&'static str>,
    pub side_effect_lock_plan_id: String,
    pub approval_evidence_boundary_id: String,
    pub readback_boundary_id: String,
    pub packet_state: &'static str,
    pub ready_for_readback_preview: bool,
    pub external_delivery_enabled: bool,
    pub operator_review_recorded: bool,
    pub approval_recorded: bool,
    pub mutates_store: bool,
    pub writes_wal: bool,
    pub applies_to_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSideEffectLockPlanPreview {
    pub lock_plan_id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub lock_scope_ids: Vec<&'static str>,
    pub lock_state: &'static str,
    pub prevents_runtime_mutation: bool,
    pub side_effects_allowed: bool,
    pub lock_established: bool,
    pub writes_store: bool,
    pub writes_wal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphApprovalEvidenceBoundaryPreview {
    pub boundary_id: String,
    pub source_surface_id: &'static str,
    pub required_evidence_field_ids: Vec<&'static str>,
    pub redaction_state: &'static str,
    pub records_operator_review: bool,
    pub records_approval: bool,
    pub persists_receipt: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewReadbackBoundaryPreview {
    pub boundary_id: String,
    pub source_surface_id: &'static str,
    pub readback_probe_id: String,
    pub readback_state: &'static str,
    pub ready_for_readback_preview: bool,
    pub readback_executed: bool,
    pub rollback_executed: bool,
    pub writes_checkpoint: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewGroupPreview {
    pub id: &'static str,
    pub source_category: &'static str,
    pub priority: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub operator_review_packet_ids: Vec<String>,
    pub side_effect_lock_plan_ids: Vec<String>,
    pub expected_review_packet_count: usize,
    pub ready_for_application_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewSideEffectLockGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub scope: &'static str,
    pub enforced_in_preview: bool,
    pub prevents_runtime_mutation: bool,
    pub note: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewSideEffectLockBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub blocks_operator_review: bool,
    pub blocks_side_effect_lock: bool,
    pub blocks_runtime_write_boundary: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub durable_store_switch_enabled: bool,
    pub idempotency_index_mutated: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub role_manifest_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub task_result_persisted: bool,
    pub lane_lease_acquired: bool,
    pub work_started: bool,
    pub budget_consumed: bool,
    pub approval_recorded: bool,
    pub operator_review_recorded: bool,
    pub side_effect_lock_established: bool,
    pub readback_executed: bool,
    pub rollback_executed: bool,
    pub runtime_application_promoted: bool,
    pub runtime_mutation_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
    pub agent_spawn_performed: bool,
}

pub fn hepta_work_graph_append_only_store_operator_review_side_effect_lock_preview_report()
-> WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockPreviewReport {
    let decisions =
        work_graph_unified_projection_enforcement_runtime_application_promotion_rerun_source_decisions();
    let operator_review_decisions = operator_review_decisions_from(&decisions);
    let packets = work_graph_append_only_store_operator_review_packets_from(&decisions);
    let lock_plans = work_graph_append_only_store_side_effect_lock_plans_from(&decisions);
    let approval_boundaries =
        work_graph_append_only_store_operator_review_approval_evidence_boundaries_from(&decisions);
    let readback_boundaries =
        work_graph_append_only_store_operator_review_readback_boundaries_from(&decisions);
    let groups = work_graph_append_only_store_operator_review_groups_from(&decisions);
    let guards = work_graph_append_only_store_operator_review_side_effect_lock_guards();
    let blockers = work_graph_append_only_store_operator_review_side_effect_lock_blockers_from(
        &decisions,
        work_graph_unified_projection_enforcement_runtime_application_promotion_rerun_residual_blockers(),
    );
    let required_prior_gates =
        work_graph_append_only_store_operator_review_side_effect_lock_required_prior_gates();
    let upstream_side_effect_lock_residual_source_count =
        affected_sources(&decisions, "side_effect_lock_not_established").len();
    let upstream_write_boundary_primary_blocked_source_count = decisions
        .iter()
        .filter(|decision| {
            decision.runtime_application_promotion_rerun_enforcement_decision
                == "deny_runtime_append_only_store_write_boundary_disabled"
        })
        .count();
    let evidence_field_ref_count = packets
        .iter()
        .map(|packet| packet.evidence_field_ids.len())
        .sum();

    WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_PREVIEW_GATE,
        schema_version: WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_SCHEMA_VERSION,
        preview_mode:
            "read_only_append_only_store_operator_review_side_effect_lock_preview_no_approval",
        upstream_runtime_application_promotion_rerun_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_APPLICATION_PROMOTION_RERUN_PREVIEW_GATE,
        upstream_operator_review_residual_source_count: operator_review_decisions.len(),
        upstream_side_effect_lock_residual_source_count,
        upstream_write_boundary_primary_blocked_source_count,
        operator_review_packet_count: packets.len(),
        side_effect_lock_plan_count: lock_plans.len(),
        approval_evidence_boundary_count: approval_boundaries.len(),
        readback_boundary_count: readback_boundaries.len(),
        evidence_field_ref_count,
        operator_review_group_count: groups.len(),
        guard_count: guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        operator_review_packets: packets,
        side_effect_lock_plans: lock_plans,
        approval_evidence_boundaries: approval_boundaries,
        readback_boundaries,
        operator_review_groups: groups,
        guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RECOMMENDED_NEXT_GATE,
        ready_for_operator_review_side_effect_lock_readback_preview: true,
        ready_for_operator_review_side_effect_lock_application_preview: false,
        ready_for_operator_review_recording: false,
        ready_for_side_effect_lock_establishment: false,
        ready_for_runtime_write_boundary_preview: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockPreviewSideEffects::none(
        ),
    }
}

pub fn work_graph_append_only_store_operator_review_packets()
-> Vec<WorkGraphOperatorReviewPacketPreview> {
    let decisions =
        work_graph_unified_projection_enforcement_runtime_application_promotion_rerun_source_decisions();
    work_graph_append_only_store_operator_review_packets_from(&decisions)
}

pub fn work_graph_append_only_store_side_effect_lock_plans()
-> Vec<WorkGraphSideEffectLockPlanPreview> {
    let decisions =
        work_graph_unified_projection_enforcement_runtime_application_promotion_rerun_source_decisions();
    work_graph_append_only_store_side_effect_lock_plans_from(&decisions)
}

pub fn work_graph_append_only_store_operator_review_approval_evidence_boundaries()
-> Vec<WorkGraphApprovalEvidenceBoundaryPreview> {
    let decisions =
        work_graph_unified_projection_enforcement_runtime_application_promotion_rerun_source_decisions();
    work_graph_append_only_store_operator_review_approval_evidence_boundaries_from(&decisions)
}

pub fn work_graph_append_only_store_operator_review_readback_boundaries()
-> Vec<WorkGraphOperatorReviewReadbackBoundaryPreview> {
    let decisions =
        work_graph_unified_projection_enforcement_runtime_application_promotion_rerun_source_decisions();
    work_graph_append_only_store_operator_review_readback_boundaries_from(&decisions)
}

pub fn work_graph_append_only_store_operator_review_groups()
-> Vec<WorkGraphOperatorReviewGroupPreview> {
    let decisions =
        work_graph_unified_projection_enforcement_runtime_application_promotion_rerun_source_decisions();
    work_graph_append_only_store_operator_review_groups_from(&decisions)
}

pub fn work_graph_append_only_store_operator_review_side_effect_lock_guards()
-> Vec<WorkGraphOperatorReviewSideEffectLockGuardPreview> {
    vec![
        guard(
            "operator_review_side_effect_lock_preview_only",
            "critical",
            "preview",
        ),
        guard(
            "operator_review_recording_disabled",
            "critical",
            "operator_review",
        ),
        guard("approval_recording_disabled", "critical", "approval"),
        guard(
            "side_effect_lock_not_established",
            "critical",
            "side_effect_lock",
        ),
        guard("external_delivery_disabled", "critical", "delivery"),
        guard("runtime_mutation_disabled", "critical", "runtime"),
        guard("wal_write_boundary_disabled", "critical", "wal"),
        guard("durable_store_runtime_switch_disabled", "critical", "store"),
        guard("idempotency_mutation_disabled", "critical", "idempotency"),
        guard(
            "readback_rollback_execution_disabled",
            "critical",
            "readback",
        ),
        guard("model_invocation_disabled", "high", "model"),
    ]
}

pub fn work_graph_append_only_store_operator_review_side_effect_lock_blockers()
-> Vec<WorkGraphOperatorReviewSideEffectLockBlockerPreview> {
    let decisions =
        work_graph_unified_projection_enforcement_runtime_application_promotion_rerun_source_decisions();
    work_graph_append_only_store_operator_review_side_effect_lock_blockers_from(
        &decisions,
        work_graph_unified_projection_enforcement_runtime_application_promotion_rerun_residual_blockers(
        ),
    )
}

pub fn work_graph_append_only_store_operator_review_side_effect_lock_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_required_prior_gates();
    gates.push(
        WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_APPLICATION_PROMOTION_RERUN_PREVIEW_GATE,
    );
    gates
}

impl WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            durable_store_switch_enabled: false,
            idempotency_index_mutated: false,
            append_only_store_enabled: false,
            projection_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            role_manifest_enforced: false,
            task_result_enforcement_enabled: false,
            task_result_persisted: false,
            lane_lease_acquired: false,
            work_started: false,
            budget_consumed: false,
            approval_recorded: false,
            operator_review_recorded: false,
            side_effect_lock_established: false,
            readback_executed: false,
            rollback_executed: false,
            runtime_application_promoted: false,
            runtime_mutation_performed: false,
            external_send_performed: false,
            model_invoked: false,
            agent_spawn_performed: false,
        }
    }
}

fn work_graph_append_only_store_operator_review_packets_from(
    decisions: &[WorkGraphRuntimeApplicationPromotionRerunSourceDecisionPreview],
) -> Vec<WorkGraphOperatorReviewPacketPreview> {
    operator_review_decisions_from(decisions)
        .into_iter()
        .map(|decision| WorkGraphOperatorReviewPacketPreview {
            packet_id: operator_review_packet_id(decision.source_surface_id),
            source_surface_id: decision.source_surface_id,
            source_category: decision.source_category,
            runtime_application_promotion_rerun_decision: decision
                .runtime_application_promotion_rerun_enforcement_decision,
            required_section_ids: OPERATOR_REVIEW_PACKET_SECTION_IDS.to_vec(),
            evidence_field_ids: OPERATOR_REVIEW_EVIDENCE_FIELD_IDS.to_vec(),
            side_effect_lock_plan_id: side_effect_lock_plan_id(decision.source_surface_id),
            approval_evidence_boundary_id: approval_evidence_boundary_id(
                decision.source_surface_id,
            ),
            readback_boundary_id: readback_boundary_id(decision.source_surface_id),
            packet_state: "preview_only_operator_review_not_recorded",
            ready_for_readback_preview: true,
            external_delivery_enabled: false,
            operator_review_recorded: false,
            approval_recorded: false,
            mutates_store: false,
            writes_wal: false,
            applies_to_runtime: false,
        })
        .collect()
}

fn work_graph_append_only_store_side_effect_lock_plans_from(
    decisions: &[WorkGraphRuntimeApplicationPromotionRerunSourceDecisionPreview],
) -> Vec<WorkGraphSideEffectLockPlanPreview> {
    operator_review_decisions_from(decisions)
        .into_iter()
        .map(|decision| WorkGraphSideEffectLockPlanPreview {
            lock_plan_id: side_effect_lock_plan_id(decision.source_surface_id),
            source_surface_id: decision.source_surface_id,
            source_category: decision.source_category,
            lock_scope_ids: SIDE_EFFECT_LOCK_SCOPE_IDS.to_vec(),
            lock_state: "planned_not_established",
            prevents_runtime_mutation: true,
            side_effects_allowed: false,
            lock_established: false,
            writes_store: false,
            writes_wal: false,
        })
        .collect()
}

fn work_graph_append_only_store_operator_review_approval_evidence_boundaries_from(
    decisions: &[WorkGraphRuntimeApplicationPromotionRerunSourceDecisionPreview],
) -> Vec<WorkGraphApprovalEvidenceBoundaryPreview> {
    operator_review_decisions_from(decisions)
        .into_iter()
        .map(|decision| WorkGraphApprovalEvidenceBoundaryPreview {
            boundary_id: approval_evidence_boundary_id(decision.source_surface_id),
            source_surface_id: decision.source_surface_id,
            required_evidence_field_ids: OPERATOR_REVIEW_EVIDENCE_FIELD_IDS.to_vec(),
            redaction_state: "redacted_preview_only",
            records_operator_review: false,
            records_approval: false,
            persists_receipt: false,
            external_delivery_enabled: false,
        })
        .collect()
}

fn work_graph_append_only_store_operator_review_readback_boundaries_from(
    decisions: &[WorkGraphRuntimeApplicationPromotionRerunSourceDecisionPreview],
) -> Vec<WorkGraphOperatorReviewReadbackBoundaryPreview> {
    operator_review_decisions_from(decisions)
        .into_iter()
        .map(|decision| WorkGraphOperatorReviewReadbackBoundaryPreview {
            boundary_id: readback_boundary_id(decision.source_surface_id),
            source_surface_id: decision.source_surface_id,
            readback_probe_id: readback_probe_id(decision.source_surface_id),
            readback_state: "planned_not_executed",
            ready_for_readback_preview: true,
            readback_executed: false,
            rollback_executed: false,
            writes_checkpoint: false,
        })
        .collect()
}

fn work_graph_append_only_store_operator_review_groups_from(
    decisions: &[WorkGraphRuntimeApplicationPromotionRerunSourceDecisionPreview],
) -> Vec<WorkGraphOperatorReviewGroupPreview> {
    let operator_review_decisions = operator_review_decisions_from(decisions);
    OPERATOR_REVIEW_GROUPS
        .iter()
        .map(|(category, group_id, priority)| {
            let affected_source_surface_ids = operator_review_decisions
                .iter()
                .filter(|decision| decision.source_category == *category)
                .map(|decision| decision.source_surface_id)
                .collect::<Vec<_>>();
            let operator_review_packet_ids = affected_source_surface_ids
                .iter()
                .map(|source| operator_review_packet_id(source))
                .collect::<Vec<_>>();
            let side_effect_lock_plan_ids = affected_source_surface_ids
                .iter()
                .map(|source| side_effect_lock_plan_id(source))
                .collect::<Vec<_>>();
            WorkGraphOperatorReviewGroupPreview {
                id: group_id,
                source_category: category,
                priority,
                expected_review_packet_count: affected_source_surface_ids.len(),
                affected_source_surface_ids,
                operator_review_packet_ids,
                side_effect_lock_plan_ids,
                ready_for_application_preview: false,
            }
        })
        .collect()
}

fn work_graph_append_only_store_operator_review_side_effect_lock_blockers_from(
    decisions: &[WorkGraphRuntimeApplicationPromotionRerunSourceDecisionPreview],
    upstream_blockers: Vec<WorkGraphRuntimeApplicationPromotionRerunResidualBlockerPreview>,
) -> Vec<WorkGraphOperatorReviewSideEffectLockBlockerPreview> {
    let mut blockers = upstream_blockers
        .into_iter()
        .map(
            |blocker| WorkGraphOperatorReviewSideEffectLockBlockerPreview {
                id: blocker.id,
                severity: blocker.severity,
                affected_source_surface_ids: blocker.affected_source_surface_ids,
                blocks_operator_review: blocks_operator_review(blocker.id),
                blocks_side_effect_lock: blocks_side_effect_lock(blocker.id),
                blocks_runtime_write_boundary: blocks_runtime_write_boundary(blocker.id),
                recommended_fix: blocker.recommended_fix,
            },
        )
        .collect::<Vec<_>>();
    blockers.push(WorkGraphOperatorReviewSideEffectLockBlockerPreview {
        id: "operator_review_side_effect_lock_readback_missing",
        severity: "high",
        affected_source_surface_ids: operator_review_decisions_from(decisions)
            .into_iter()
            .map(|decision| decision.source_surface_id)
            .collect(),
        blocks_operator_review: true,
        blocks_side_effect_lock: true,
        blocks_runtime_write_boundary: false,
        recommended_fix:
            "read back every operator-review packet and side-effect lock plan before application preview",
    });
    blockers
}

fn operator_review_decisions_from(
    decisions: &[WorkGraphRuntimeApplicationPromotionRerunSourceDecisionPreview],
) -> Vec<&WorkGraphRuntimeApplicationPromotionRerunSourceDecisionPreview> {
    decisions
        .iter()
        .filter(|decision| {
            decision.runtime_application_promotion_rerun_enforcement_decision
                == "deny_operator_review_required"
                || decision
                    .residual_source_blocker_ids
                    .contains(&"operator_review_required")
                || decision
                    .residual_source_blocker_ids
                    .contains(&"side_effect_lock_not_established")
        })
        .collect()
}

fn affected_sources(
    decisions: &[WorkGraphRuntimeApplicationPromotionRerunSourceDecisionPreview],
    blocker_id: &'static str,
) -> Vec<&'static str> {
    decisions
        .iter()
        .filter(|decision| decision.residual_source_blocker_ids.contains(&blocker_id))
        .map(|decision| decision.source_surface_id)
        .collect()
}

fn blocks_operator_review(blocker_id: &'static str) -> bool {
    matches!(
        blocker_id,
        "operator_review_required"
            | "side_effect_lock_not_established"
            | "operator_review_side_effect_lock_readback_missing"
    )
}

fn blocks_side_effect_lock(blocker_id: &'static str) -> bool {
    matches!(
        blocker_id,
        "side_effect_lock_not_established"
            | "operator_review_required"
            | "operator_review_side_effect_lock_readback_missing"
    )
}

fn blocks_runtime_write_boundary(blocker_id: &'static str) -> bool {
    matches!(
        blocker_id,
        "readback_execution_disabled"
            | "durable_store_runtime_switch_disabled"
            | "wal_write_boundary_not_enabled"
            | "idempotency_index_mutation_disabled"
            | "rollback_readback_not_executed"
    )
}

fn guard(
    id: &'static str,
    severity: &'static str,
    scope: &'static str,
) -> WorkGraphOperatorReviewSideEffectLockGuardPreview {
    WorkGraphOperatorReviewSideEffectLockGuardPreview {
        id,
        severity,
        scope,
        enforced_in_preview: true,
        prevents_runtime_mutation: true,
        note: "preview records review and side-effect-lock contracts only; no approval or lock is applied",
    }
}

fn operator_review_packet_id(source: &str) -> String {
    format!("operator_review_packet__{source}")
}

fn side_effect_lock_plan_id(source: &str) -> String {
    format!("side_effect_lock_plan__{source}")
}

fn approval_evidence_boundary_id(source: &str) -> String {
    format!("approval_evidence_boundary__{source}")
}

fn readback_boundary_id(source: &str) -> String {
    format!("operator_review_readback_boundary__{source}")
}

fn readback_probe_id(source: &str) -> String {
    format!("operator_review_side_effect_lock_readback_probe__{source}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_review_side_effect_lock_declares_no_mutation_boundary() {
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_PREVIEW_GATE,
            "hepta_work_graph_append_only_store_operator_review_side_effect_lock_preview_gate"
        );
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_RECOMMENDED_NEXT_GATE,
            "hepta_work_graph_append_only_store_operator_review_side_effect_lock_readback_preview_gate"
        );
        assert_eq!(
            WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockPreviewSideEffects::none(),
            WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockPreviewSideEffects {
                filesystem_written: false,
                graph_state_persisted: false,
                wal_written: false,
                checkpoint_written: false,
                durable_store_switch_enabled: false,
                idempotency_index_mutated: false,
                append_only_store_enabled: false,
                projection_enforcement_enabled: false,
                scheduler_admission_enforced: false,
                role_manifest_enforced: false,
                task_result_enforcement_enabled: false,
                task_result_persisted: false,
                lane_lease_acquired: false,
                work_started: false,
                budget_consumed: false,
                approval_recorded: false,
                operator_review_recorded: false,
                side_effect_lock_established: false,
                readback_executed: false,
                rollback_executed: false,
                runtime_application_promoted: false,
                runtime_mutation_performed: false,
                external_send_performed: false,
                model_invoked: false,
                agent_spawn_performed: false,
            }
        );
    }

    #[test]
    fn operator_review_side_effect_lock_classifies_boundaries() {
        let required_prior_gates =
            work_graph_append_only_store_operator_review_side_effect_lock_required_prior_gates();

        assert_eq!(required_prior_gates.len(), 48);
        assert_eq!(
            required_prior_gates.last().copied(),
            Some(WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RUNTIME_APPLICATION_PROMOTION_RERUN_PREVIEW_GATE)
        );
        assert!(blocks_operator_review("operator_review_required"));
        assert!(blocks_operator_review(
            "operator_review_side_effect_lock_readback_missing"
        ));
        assert!(blocks_side_effect_lock("side_effect_lock_not_established"));
        assert!(blocks_runtime_write_boundary(
            "wal_write_boundary_not_enabled"
        ));
        assert!(!blocks_operator_review("wal_write_boundary_not_enabled"));
    }
}
