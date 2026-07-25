use serde::Serialize;

use crate::work_graph_append_only_store_operator_review_side_effect_lock_preview::WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_PREVIEW_GATE;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_preview::WorkGraphApprovalEvidenceBoundaryPreview;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_preview::WorkGraphOperatorReviewGroupPreview;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_preview::WorkGraphOperatorReviewPacketPreview;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_preview::WorkGraphOperatorReviewReadbackBoundaryPreview;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_preview::WorkGraphOperatorReviewSideEffectLockBlockerPreview;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_preview::WorkGraphOperatorReviewSideEffectLockGuardPreview;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_preview::WorkGraphSideEffectLockPlanPreview;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_preview::work_graph_append_only_store_operator_review_approval_evidence_boundaries;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_preview::work_graph_append_only_store_operator_review_groups;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_preview::work_graph_append_only_store_operator_review_packets;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_preview::work_graph_append_only_store_operator_review_readback_boundaries;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_preview::work_graph_append_only_store_operator_review_side_effect_lock_blockers;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_preview::work_graph_append_only_store_operator_review_side_effect_lock_guards;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_preview::work_graph_append_only_store_operator_review_side_effect_lock_required_prior_gates;
use crate::work_graph_append_only_store_operator_review_side_effect_lock_preview::work_graph_append_only_store_side_effect_lock_plans;

pub const WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_READBACK_PREVIEW_GATE:
    &str =
    "hepta_work_graph_append_only_store_operator_review_side_effect_lock_readback_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_READBACK_SCHEMA_VERSION:
    &str = "work_graph_append_only_store_operator_review_side_effect_lock_readback_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_store_operator_review_side_effect_lock_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub operator_review_packet_count: usize,
    pub side_effect_lock_plan_count: usize,
    pub approval_evidence_boundary_count: usize,
    pub readback_boundary_count: usize,
    pub readback_plan_count: usize,
    pub packet_assertion_count: usize,
    pub side_effect_lock_assertion_count: usize,
    pub approval_boundary_assertion_count: usize,
    pub readback_boundary_assertion_count: usize,
    pub evidence_field_assertion_count: usize,
    pub guard_assertion_count: usize,
    pub blocker_mapping_assertion_count: usize,
    pub evidence_field_ref_count: usize,
    pub lock_scope_ref_count: usize,
    pub group_source_ref_count: usize,
    pub drift_detector_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_plans: Vec<WorkGraphOperatorReviewSideEffectLockReadbackPlanPreview>,
    pub packet_assertions: Vec<WorkGraphOperatorReviewPacketReadbackAssertionPreview>,
    pub side_effect_lock_assertions: Vec<WorkGraphSideEffectLockReadbackAssertionPreview>,
    pub approval_boundary_assertions:
        Vec<WorkGraphApprovalEvidenceBoundaryReadbackAssertionPreview>,
    pub readback_boundary_assertions: Vec<WorkGraphOperatorReviewBoundaryReadbackAssertionPreview>,
    pub evidence_field_assertions:
        Vec<WorkGraphOperatorReviewEvidenceFieldReadbackAssertionPreview>,
    pub group_assertions: Vec<WorkGraphOperatorReviewGroupReadbackAssertionPreview>,
    pub guard_assertions: Vec<WorkGraphOperatorReviewGuardReadbackAssertionPreview>,
    pub blocker_mapping_assertions: Vec<WorkGraphOperatorReviewBlockerMappingAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphOperatorReviewReadbackDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphOperatorReviewReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_operator_review_side_effect_lock_application_preview: bool,
    pub ready_for_readback_execution: bool,
    pub ready_for_operator_review_recording: bool,
    pub ready_for_side_effect_lock_establishment: bool,
    pub ready_for_runtime_write_boundary_preview: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewSideEffectLockReadbackPlanPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub operator_review_packet_id: String,
    pub side_effect_lock_plan_id: String,
    pub approval_evidence_boundary_id: String,
    pub readback_boundary_id: String,
    pub required_evidence_fields: Vec<&'static str>,
    pub lock_scope_ids: Vec<&'static str>,
    pub readback_state: &'static str,
    pub required_before_application: bool,
    pub performs_readback: bool,
    pub records_operator_review: bool,
    pub records_approval: bool,
    pub establishes_side_effect_lock: bool,
    pub mutates_store: bool,
    pub writes_wal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewPacketReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub packet_id: String,
    pub expected_packet_state: &'static str,
    pub required_section_ids: Vec<&'static str>,
    pub required_evidence_field_ids: Vec<&'static str>,
    pub records_operator_review: bool,
    pub records_approval: bool,
    pub applies_to_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSideEffectLockReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub lock_plan_id: String,
    pub expected_lock_state: &'static str,
    pub lock_scope_ids: Vec<&'static str>,
    pub prevents_runtime_mutation: bool,
    pub side_effects_allowed: bool,
    pub lock_established: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphApprovalEvidenceBoundaryReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub boundary_id: String,
    pub expected_boundary_state: &'static str,
    pub required_evidence_field_ids: Vec<&'static str>,
    pub records_operator_review: bool,
    pub records_approval: bool,
    pub persists_receipt: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewBoundaryReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub boundary_id: String,
    pub readback_probe_id: String,
    pub expected_readback_state: &'static str,
    pub performs_readback: bool,
    pub rollback_executed: bool,
    pub writes_checkpoint: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewEvidenceFieldReadbackAssertionPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub required_evidence_fields: Vec<&'static str>,
    pub required_field_count: usize,
    pub expected_evidence_state: &'static str,
    pub performs_readback: bool,
    pub persists_evidence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewGroupReadbackAssertionPreview {
    pub id: String,
    pub group_id: &'static str,
    pub source_category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub operator_review_packet_ids: Vec<String>,
    pub side_effect_lock_plan_ids: Vec<String>,
    pub expected_review_packet_count: usize,
    pub expected_group_state: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewGuardReadbackAssertionPreview {
    pub id: String,
    pub guard_id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub expected_guard_state: &'static str,
    pub prevents_runtime_mutation: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewBlockerMappingAssertionPreview {
    pub id: String,
    pub blocker_id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_readback_plan_ids: Vec<String>,
    pub expected_blocker_state: &'static str,
    pub blocks_operator_review: bool,
    pub blocks_side_effect_lock: bool,
    pub blocks_runtime_write_boundary: bool,
    pub performs_readback: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewReadbackDriftDetectorPreview {
    pub id: &'static str,
    pub compared_field_ids: Vec<&'static str>,
    pub severity: &'static str,
    pub blocks_application_preview: bool,
    pub performs_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_readback_plan_ids: Vec<String>,
    pub blocks_operator_review: bool,
    pub blocks_side_effect_lock: bool,
    pub blocks_runtime_write_boundary: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockReadbackPreviewSideEffects {
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
    pub approval_recorded: bool,
    pub operator_review_recorded: bool,
    pub side_effect_lock_established: bool,
    pub readback_executed: bool,
    pub rollback_executed: bool,
    pub runtime_mutation_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
    pub agent_spawn_performed: bool,
}

pub fn hepta_work_graph_append_only_store_operator_review_side_effect_lock_readback_preview_report()
-> WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockReadbackPreviewReport {
    let packets = work_graph_append_only_store_operator_review_packets();
    let lock_plans = work_graph_append_only_store_side_effect_lock_plans();
    let approval_boundaries =
        work_graph_append_only_store_operator_review_approval_evidence_boundaries();
    let readback_boundaries = work_graph_append_only_store_operator_review_readback_boundaries();
    let groups = work_graph_append_only_store_operator_review_groups();
    let guards = work_graph_append_only_store_operator_review_side_effect_lock_guards();
    let preview_blockers = work_graph_append_only_store_operator_review_side_effect_lock_blockers();
    let readback_plans =
        work_graph_append_only_store_operator_review_side_effect_lock_readback_plans_from(
            &packets,
            &lock_plans,
            &approval_boundaries,
            &readback_boundaries,
        );
    let packet_assertions = packet_readback_assertions_from(&packets);
    let side_effect_lock_assertions = side_effect_lock_readback_assertions_from(&lock_plans);
    let approval_boundary_assertions =
        approval_boundary_readback_assertions_from(&approval_boundaries);
    let readback_boundary_assertions = readback_boundary_assertions_from(&readback_boundaries);
    let evidence_field_assertions = evidence_field_assertions_from(&readback_plans);
    let group_assertions = group_readback_assertions_from(&groups);
    let guard_assertions = guard_readback_assertions_from(&guards);
    let blockers = readback_blockers_from_preview_blockers(&preview_blockers, &readback_plans);
    let blocker_mapping_assertions = blocker_mapping_assertions_from(&blockers, &readback_plans);
    let drift_detectors =
        work_graph_append_only_store_operator_review_side_effect_lock_readback_drift_detectors();
    let required_prior_gates =
        work_graph_append_only_store_operator_review_side_effect_lock_readback_required_prior_gates(
        );

    WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_READBACK_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_READBACK_SCHEMA_VERSION,
        preview_mode:
            "read_only_append_only_store_operator_review_side_effect_lock_readback_no_execution",
        operator_review_packet_count: packets.len(),
        side_effect_lock_plan_count: lock_plans.len(),
        approval_evidence_boundary_count: approval_boundaries.len(),
        readback_boundary_count: readback_boundaries.len(),
        readback_plan_count: readback_plans.len(),
        packet_assertion_count: packet_assertions.len(),
        side_effect_lock_assertion_count: side_effect_lock_assertions.len(),
        approval_boundary_assertion_count: approval_boundary_assertions.len(),
        readback_boundary_assertion_count: readback_boundary_assertions.len(),
        evidence_field_assertion_count: evidence_field_assertions.len(),
        guard_assertion_count: guard_assertions.len(),
        blocker_mapping_assertion_count: blocker_mapping_assertions.len(),
        evidence_field_ref_count: evidence_field_assertions
            .iter()
            .map(|assertion| assertion.required_field_count)
            .sum(),
        lock_scope_ref_count: readback_plans
            .iter()
            .map(|plan| plan.lock_scope_ids.len())
            .sum(),
        group_source_ref_count: groups
            .iter()
            .map(|group| group.affected_source_surface_ids.len())
            .sum(),
        drift_detector_count: drift_detectors.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_plans,
        packet_assertions,
        side_effect_lock_assertions,
        approval_boundary_assertions,
        readback_boundary_assertions,
        evidence_field_assertions,
        group_assertions,
        guard_assertions,
        blocker_mapping_assertions,
        drift_detectors,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_operator_review_side_effect_lock_application_preview: true,
        ready_for_readback_execution: false,
        ready_for_operator_review_recording: false,
        ready_for_side_effect_lock_establishment: false,
        ready_for_runtime_write_boundary_preview: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_store_operator_review_side_effect_lock_readback_plans()
-> Vec<WorkGraphOperatorReviewSideEffectLockReadbackPlanPreview> {
    let packets = work_graph_append_only_store_operator_review_packets();
    let lock_plans = work_graph_append_only_store_side_effect_lock_plans();
    let approval_boundaries =
        work_graph_append_only_store_operator_review_approval_evidence_boundaries();
    let readback_boundaries = work_graph_append_only_store_operator_review_readback_boundaries();
    work_graph_append_only_store_operator_review_side_effect_lock_readback_plans_from(
        &packets,
        &lock_plans,
        &approval_boundaries,
        &readback_boundaries,
    )
}

pub fn work_graph_append_only_store_operator_review_side_effect_lock_readback_drift_detectors()
-> Vec<WorkGraphOperatorReviewReadbackDriftDetectorPreview> {
    vec![
        drift_detector(
            "operator_review_packet_alignment",
            vec!["packet_id", "required_section_ids"],
        ),
        drift_detector(
            "side_effect_lock_scope_alignment",
            vec!["lock_plan_id", "lock_scope_ids"],
        ),
        drift_detector(
            "approval_evidence_boundary_alignment",
            vec!["boundary_id", "required_evidence_field_ids"],
        ),
        drift_detector(
            "readback_boundary_alignment",
            vec!["readback_probe_id", "readback_state"],
        ),
        drift_detector(
            "guard_no_mutation_alignment",
            vec!["guard_id", "mutates_runtime"],
        ),
        drift_detector(
            "blocker_mapping_alignment",
            vec!["blocker_id", "affected_readback_plan_ids"],
        ),
        drift_detector(
            "side_effect_boundary_alignment",
            vec!["side_effects", "operator_review_recorded"],
        ),
    ]
}

pub fn work_graph_append_only_store_operator_review_side_effect_lock_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_store_operator_review_side_effect_lock_required_prior_gates();
    gates.push(WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_PREVIEW_GATE);
    gates
}

impl WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockReadbackPreviewSideEffects {
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
            approval_recorded: false,
            operator_review_recorded: false,
            side_effect_lock_established: false,
            readback_executed: false,
            rollback_executed: false,
            runtime_mutation_performed: false,
            external_send_performed: false,
            model_invoked: false,
            agent_spawn_performed: false,
        }
    }
}

fn work_graph_append_only_store_operator_review_side_effect_lock_readback_plans_from(
    packets: &[WorkGraphOperatorReviewPacketPreview],
    lock_plans: &[WorkGraphSideEffectLockPlanPreview],
    approval_boundaries: &[WorkGraphApprovalEvidenceBoundaryPreview],
    readback_boundaries: &[WorkGraphOperatorReviewReadbackBoundaryPreview],
) -> Vec<WorkGraphOperatorReviewSideEffectLockReadbackPlanPreview> {
    packets
        .iter()
        .filter_map(|packet| {
            let lock_plan = lock_plans
                .iter()
                .find(|plan| plan.source_surface_id == packet.source_surface_id)?;
            let approval_boundary = approval_boundaries
                .iter()
                .find(|boundary| boundary.source_surface_id == packet.source_surface_id)?;
            let readback_boundary = readback_boundaries
                .iter()
                .find(|boundary| boundary.source_surface_id == packet.source_surface_id)?;
            Some(WorkGraphOperatorReviewSideEffectLockReadbackPlanPreview {
                id: readback_plan_id(packet.source_surface_id),
                source_surface_id: packet.source_surface_id,
                source_category: packet.source_category,
                operator_review_packet_id: packet.packet_id.clone(),
                side_effect_lock_plan_id: lock_plan.lock_plan_id.clone(),
                approval_evidence_boundary_id: approval_boundary.boundary_id.clone(),
                readback_boundary_id: readback_boundary.boundary_id.clone(),
                required_evidence_fields: packet.evidence_field_ids.clone(),
                lock_scope_ids: lock_plan.lock_scope_ids.clone(),
                readback_state: "asserted_from_operator_review_preview_no_execution",
                required_before_application: true,
                performs_readback: false,
                records_operator_review: false,
                records_approval: false,
                establishes_side_effect_lock: false,
                mutates_store: false,
                writes_wal: false,
            })
        })
        .collect()
}

fn packet_readback_assertions_from(
    packets: &[WorkGraphOperatorReviewPacketPreview],
) -> Vec<WorkGraphOperatorReviewPacketReadbackAssertionPreview> {
    packets
        .iter()
        .map(
            |packet| WorkGraphOperatorReviewPacketReadbackAssertionPreview {
                id: format!(
                    "operator_review_packet_readback_assertion__{}",
                    packet.source_surface_id
                ),
                source_surface_id: packet.source_surface_id,
                packet_id: packet.packet_id.clone(),
                expected_packet_state: "readback_verified_no_mutation",
                required_section_ids: packet.required_section_ids.clone(),
                required_evidence_field_ids: packet.evidence_field_ids.clone(),
                records_operator_review: false,
                records_approval: false,
                applies_to_runtime: false,
            },
        )
        .collect()
}

fn side_effect_lock_readback_assertions_from(
    lock_plans: &[WorkGraphSideEffectLockPlanPreview],
) -> Vec<WorkGraphSideEffectLockReadbackAssertionPreview> {
    lock_plans
        .iter()
        .map(|plan| WorkGraphSideEffectLockReadbackAssertionPreview {
            id: format!(
                "side_effect_lock_readback_assertion__{}",
                plan.source_surface_id
            ),
            source_surface_id: plan.source_surface_id,
            lock_plan_id: plan.lock_plan_id.clone(),
            expected_lock_state: "readback_verified_not_established",
            lock_scope_ids: plan.lock_scope_ids.clone(),
            prevents_runtime_mutation: true,
            side_effects_allowed: false,
            lock_established: false,
        })
        .collect()
}

fn approval_boundary_readback_assertions_from(
    boundaries: &[WorkGraphApprovalEvidenceBoundaryPreview],
) -> Vec<WorkGraphApprovalEvidenceBoundaryReadbackAssertionPreview> {
    boundaries
        .iter()
        .map(
            |boundary| WorkGraphApprovalEvidenceBoundaryReadbackAssertionPreview {
                id: format!(
                    "approval_evidence_boundary_readback_assertion__{}",
                    boundary.source_surface_id
                ),
                source_surface_id: boundary.source_surface_id,
                boundary_id: boundary.boundary_id.clone(),
                expected_boundary_state: "readback_verified_not_recorded",
                required_evidence_field_ids: boundary.required_evidence_field_ids.clone(),
                records_operator_review: false,
                records_approval: false,
                persists_receipt: false,
            },
        )
        .collect()
}

fn readback_boundary_assertions_from(
    boundaries: &[WorkGraphOperatorReviewReadbackBoundaryPreview],
) -> Vec<WorkGraphOperatorReviewBoundaryReadbackAssertionPreview> {
    boundaries
        .iter()
        .map(
            |boundary| WorkGraphOperatorReviewBoundaryReadbackAssertionPreview {
                id: format!(
                    "operator_review_readback_boundary_assertion__{}",
                    boundary.source_surface_id
                ),
                source_surface_id: boundary.source_surface_id,
                boundary_id: boundary.boundary_id.clone(),
                readback_probe_id: boundary.readback_probe_id.clone(),
                expected_readback_state: "readback_contract_declared_not_executed",
                performs_readback: false,
                rollback_executed: false,
                writes_checkpoint: false,
            },
        )
        .collect()
}

fn evidence_field_assertions_from(
    plans: &[WorkGraphOperatorReviewSideEffectLockReadbackPlanPreview],
) -> Vec<WorkGraphOperatorReviewEvidenceFieldReadbackAssertionPreview> {
    plans
        .iter()
        .map(
            |plan| WorkGraphOperatorReviewEvidenceFieldReadbackAssertionPreview {
                id: format!(
                    "operator_review_evidence_field_readback_assertion__{}",
                    plan.source_surface_id
                ),
                source_surface_id: plan.source_surface_id,
                required_field_count: plan.required_evidence_fields.len(),
                required_evidence_fields: plan.required_evidence_fields.clone(),
                expected_evidence_state: "evidence_fields_declared_not_persisted",
                performs_readback: false,
                persists_evidence: false,
            },
        )
        .collect()
}

fn group_readback_assertions_from(
    groups: &[WorkGraphOperatorReviewGroupPreview],
) -> Vec<WorkGraphOperatorReviewGroupReadbackAssertionPreview> {
    groups
        .iter()
        .map(
            |group| WorkGraphOperatorReviewGroupReadbackAssertionPreview {
                id: format!("operator_review_group_readback_assertion__{}", group.id),
                group_id: group.id,
                source_category: group.source_category,
                affected_source_surface_ids: group.affected_source_surface_ids.clone(),
                operator_review_packet_ids: group.operator_review_packet_ids.clone(),
                side_effect_lock_plan_ids: group.side_effect_lock_plan_ids.clone(),
                expected_review_packet_count: group.expected_review_packet_count,
                expected_group_state: "readback_verified_no_mutation",
            },
        )
        .collect()
}

fn guard_readback_assertions_from(
    guards: &[WorkGraphOperatorReviewSideEffectLockGuardPreview],
) -> Vec<WorkGraphOperatorReviewGuardReadbackAssertionPreview> {
    guards
        .iter()
        .map(
            |guard| WorkGraphOperatorReviewGuardReadbackAssertionPreview {
                id: format!("operator_review_guard_readback_assertion__{}", guard.id),
                guard_id: guard.id,
                severity: guard.severity,
                guard_scope: guard.scope,
                expected_guard_state: "guard_declared_and_runtime_mutation_prevented",
                prevents_runtime_mutation: true,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn readback_blockers_from_preview_blockers(
    blockers: &[WorkGraphOperatorReviewSideEffectLockBlockerPreview],
    readback_plans: &[WorkGraphOperatorReviewSideEffectLockReadbackPlanPreview],
) -> Vec<WorkGraphOperatorReviewReadbackBlockerPreview> {
    let mut readback_blockers = blockers
        .iter()
        .map(|blocker| WorkGraphOperatorReviewReadbackBlockerPreview {
            id: blocker.id,
            severity: blocker.severity,
            affected_readback_plan_ids: affected_readback_plan_ids_for(
                &blocker.affected_source_surface_ids,
                readback_plans,
            ),
            affected_source_surface_ids: blocker.affected_source_surface_ids.clone(),
            blocks_operator_review: blocker.blocks_operator_review,
            blocks_side_effect_lock: blocker.blocks_side_effect_lock,
            blocks_runtime_write_boundary: blocker.blocks_runtime_write_boundary,
            recommended_fix: blocker.recommended_fix,
        })
        .collect::<Vec<_>>();
    readback_blockers.push(WorkGraphOperatorReviewReadbackBlockerPreview {
        id: "operator_review_side_effect_lock_application_missing",
        severity: "high",
        affected_source_surface_ids: readback_plans
            .iter()
            .map(|plan| plan.source_surface_id)
            .collect(),
        affected_readback_plan_ids: readback_plans
            .iter()
            .map(|plan| plan.id.clone())
            .collect(),
        blocks_operator_review: true,
        blocks_side_effect_lock: true,
        blocks_runtime_write_boundary: false,
        recommended_fix:
            "apply readback-verified operator review packets and side-effect lock plans before readiness rerun",
    });
    readback_blockers
}

fn blocker_mapping_assertions_from(
    blockers: &[WorkGraphOperatorReviewReadbackBlockerPreview],
    readback_plans: &[WorkGraphOperatorReviewSideEffectLockReadbackPlanPreview],
) -> Vec<WorkGraphOperatorReviewBlockerMappingAssertionPreview> {
    blockers
        .iter()
        .map(
            |blocker| WorkGraphOperatorReviewBlockerMappingAssertionPreview {
                id: format!(
                    "operator_review_blocker_mapping_readback_assertion__{}",
                    blocker.id
                ),
                blocker_id: blocker.id,
                severity: blocker.severity,
                affected_readback_plan_ids: affected_readback_plan_ids_for(
                    &blocker.affected_source_surface_ids,
                    readback_plans,
                ),
                affected_source_surface_ids: blocker.affected_source_surface_ids.clone(),
                expected_blocker_state: "blocker_mapping_readback_verified_no_mutation",
                blocks_operator_review: blocker.blocks_operator_review,
                blocks_side_effect_lock: blocker.blocks_side_effect_lock,
                blocks_runtime_write_boundary: blocker.blocks_runtime_write_boundary,
                performs_readback: false,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn affected_readback_plan_ids_for(
    affected_sources: &[&'static str],
    readback_plans: &[WorkGraphOperatorReviewSideEffectLockReadbackPlanPreview],
) -> Vec<String> {
    readback_plans
        .iter()
        .filter(|plan| affected_sources.contains(&plan.source_surface_id))
        .map(|plan| plan.id.clone())
        .collect()
}

fn drift_detector(
    id: &'static str,
    compared_field_ids: Vec<&'static str>,
) -> WorkGraphOperatorReviewReadbackDriftDetectorPreview {
    WorkGraphOperatorReviewReadbackDriftDetectorPreview {
        id,
        compared_field_ids,
        severity: "high",
        blocks_application_preview: true,
        performs_readback: false,
    }
}

fn readback_plan_id(source: &str) -> String {
    format!("operator_review_side_effect_lock_readback_plan__{source}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_review_side_effect_lock_readback_declares_no_execution_boundary() {
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_READBACK_PREVIEW_GATE,
            "hepta_work_graph_append_only_store_operator_review_side_effect_lock_readback_preview_gate"
        );
        assert_eq!(
            WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_READBACK_RECOMMENDED_NEXT_GATE,
            "hepta_work_graph_append_only_store_operator_review_side_effect_lock_application_preview_gate"
        );
        assert_eq!(
            WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockReadbackPreviewSideEffects::none(),
            WorkGraphAppendOnlyStoreOperatorReviewSideEffectLockReadbackPreviewSideEffects {
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
                approval_recorded: false,
                operator_review_recorded: false,
                side_effect_lock_established: false,
                readback_executed: false,
                rollback_executed: false,
                runtime_mutation_performed: false,
                external_send_performed: false,
                model_invoked: false,
                agent_spawn_performed: false,
            }
        );
    }

    #[test]
    fn operator_review_side_effect_lock_readback_tracks_priors_and_drift_detectors() {
        let required_prior_gates =
            work_graph_append_only_store_operator_review_side_effect_lock_readback_required_prior_gates();

        assert_eq!(required_prior_gates.len(), 49);
        assert_eq!(
            required_prior_gates.last().copied(),
            Some(WORK_GRAPH_APPEND_ONLY_STORE_OPERATOR_REVIEW_SIDE_EFFECT_LOCK_PREVIEW_GATE)
        );
        assert_eq!(
            work_graph_append_only_store_operator_review_side_effect_lock_readback_drift_detectors(
            )
            .len(),
            7
        );
    }
}
