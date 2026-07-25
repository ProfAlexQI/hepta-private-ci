use serde::Serialize;

use crate::work_graph_store_idempotency_guard_gap_closure_preview::WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_PREVIEW_GATE;
use crate::work_graph_store_idempotency_guard_gap_closure_preview::WorkGraphStoreIdempotencyCandidateGuardPreview;
use crate::work_graph_store_idempotency_guard_gap_closure_preview::WorkGraphStoreIdempotencyGuardClosurePlanPreview;
use crate::work_graph_store_idempotency_guard_gap_closure_preview::WorkGraphStoreIdempotencyGuardProbeBindingPreview;
use crate::work_graph_store_idempotency_guard_gap_closure_preview::work_graph_store_idempotency_guard_gap_bindings;
use crate::work_graph_store_idempotency_guard_gap_closure_preview::work_graph_store_idempotency_guard_gap_candidate_guards;
use crate::work_graph_store_idempotency_guard_gap_closure_preview::work_graph_store_idempotency_guard_gap_closure_plans;
use crate::work_graph_store_idempotency_guard_gap_closure_preview::work_graph_store_idempotency_guard_gap_closure_required_prior_gates;
use crate::work_graph_store_idempotency_guard_gap_closure_preview::work_graph_store_idempotency_guard_gap_probe_bindings;

pub const WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_store_idempotency_guard_gap_closure_readback_preview_gate";
pub const WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_READBACK_SCHEMA_VERSION: &str =
    "work_graph_store_idempotency_guard_gap_closure_readback_preview_v1";
pub const WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_store_idempotency_guard_gap_closure_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardGapClosureReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub closure_plan_count: usize,
    pub candidate_guard_count: usize,
    pub guard_binding_count: usize,
    pub guard_probe_binding_count: usize,
    pub readback_plan_count: usize,
    pub key_formula_assertion_count: usize,
    pub collision_policy_assertion_count: usize,
    pub probe_binding_assertion_count: usize,
    pub collection_ref_assertion_count: usize,
    pub expected_collection_ref_count: usize,
    pub readback_probe_contract_ref_count: usize,
    pub readback_evidence_field_ref_count: usize,
    pub drift_detector_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_plans: Vec<WorkGraphStoreIdempotencyGuardReadbackPlanPreview>,
    pub key_formula_assertions:
        Vec<WorkGraphStoreIdempotencyGuardKeyFormulaReadbackAssertionPreview>,
    pub collision_policy_assertions:
        Vec<WorkGraphStoreIdempotencyGuardCollisionPolicyReadbackAssertionPreview>,
    pub probe_binding_assertions:
        Vec<WorkGraphStoreIdempotencyGuardProbeBindingReadbackAssertionPreview>,
    pub collection_ref_assertions:
        Vec<WorkGraphStoreIdempotencyGuardCollectionRefReadbackAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphStoreIdempotencyGuardReadbackDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphStoreIdempotencyGuardReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_store_idempotency_guard_gap_closure_application_preview: bool,
    pub ready_for_runtime_guard_application: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphStoreIdempotencyGuardGapClosureReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardReadbackPlanPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub closure_plan_id: &'static str,
    pub candidate_guard_id: &'static str,
    pub key_formula_assertion_id: &'static str,
    pub collision_policy_assertion_id: &'static str,
    pub probe_binding_assertion_id: &'static str,
    pub collection_ref_assertion_id: &'static str,
    pub expected_key_fields: Vec<&'static str>,
    pub expected_collection_ids: Vec<&'static str>,
    pub readback_probe_contract_ids: Vec<&'static str>,
    pub required_before_runtime_guard_application: bool,
    pub readback_state: &'static str,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardKeyFormulaReadbackAssertionPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub candidate_guard_id: &'static str,
    pub key_fields: Vec<&'static str>,
    pub key_formula: &'static str,
    pub replay_scope: &'static str,
    pub redaction_policy: &'static str,
    pub requires_sha256_formula: bool,
    pub mutates_idempotency_index: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardCollisionPolicyReadbackAssertionPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub candidate_guard_id: &'static str,
    pub collision_policy: &'static str,
    pub required_before_append_only_intake: bool,
    pub expected_collision_state: &'static str,
    pub mutates_idempotency_index: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardProbeBindingReadbackAssertionPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub candidate_guard_id: &'static str,
    pub readback_probe_contract_ids: Vec<&'static str>,
    pub target_collection_ids: Vec<&'static str>,
    pub readback_evidence_fields: Vec<&'static str>,
    pub drift_detector_ids: Vec<&'static str>,
    pub expected_probe_binding_state: &'static str,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardCollectionRefReadbackAssertionPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub candidate_guard_id: &'static str,
    pub expected_collection_ids: Vec<&'static str>,
    pub required_collection_count: usize,
    pub required_readback_probe_contract_ids: Vec<&'static str>,
    pub expected_guard_binding_state: &'static str,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardReadbackDriftDetectorPreview {
    pub id: &'static str,
    pub compared_field_ids: Vec<&'static str>,
    pub severity: &'static str,
    pub blocks_runtime_guard_application: bool,
    pub performs_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardGapClosureReadbackPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub idempotency_index_mutated: bool,
    pub store_guard_attached: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub readback_performed: bool,
    pub task_result_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub approval_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_store_idempotency_guard_gap_closure_readback_preview_report()
-> WorkGraphStoreIdempotencyGuardGapClosureReadbackPreviewReport {
    let closure_plans = work_graph_store_idempotency_guard_gap_closure_plans();
    let candidate_guards = work_graph_store_idempotency_guard_gap_candidate_guards();
    let guard_bindings = work_graph_store_idempotency_guard_gap_bindings();
    let guard_probe_bindings = work_graph_store_idempotency_guard_gap_probe_bindings();
    let readback_plans = work_graph_store_idempotency_guard_gap_closure_readback_plans();
    let key_formula_assertions =
        work_graph_store_idempotency_guard_key_formula_readback_assertions();
    let collision_policy_assertions =
        work_graph_store_idempotency_guard_collision_policy_readback_assertions();
    let probe_binding_assertions =
        work_graph_store_idempotency_guard_probe_binding_readback_assertions();
    let collection_ref_assertions =
        work_graph_store_idempotency_guard_collection_ref_readback_assertions();
    let drift_detectors = work_graph_store_idempotency_guard_readback_drift_detectors();
    let blockers = work_graph_store_idempotency_guard_gap_closure_readback_blockers();
    let required_prior_gates =
        work_graph_store_idempotency_guard_gap_closure_readback_required_prior_gates();

    WorkGraphStoreIdempotencyGuardGapClosureReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_READBACK_PREVIEW_GATE,
        schema_version: WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_store_idempotency_guard_gap_closure_readback_no_execution",
        closure_plan_count: closure_plans.len(),
        candidate_guard_count: candidate_guards.len(),
        guard_binding_count: guard_bindings.len(),
        guard_probe_binding_count: guard_probe_bindings.len(),
        readback_plan_count: readback_plans.len(),
        key_formula_assertion_count: key_formula_assertions.len(),
        collision_policy_assertion_count: collision_policy_assertions.len(),
        probe_binding_assertion_count: probe_binding_assertions.len(),
        collection_ref_assertion_count: collection_ref_assertions.len(),
        expected_collection_ref_count: collection_ref_assertions
            .iter()
            .map(|assertion| assertion.expected_collection_ids.len())
            .sum(),
        readback_probe_contract_ref_count: probe_binding_assertions
            .iter()
            .map(|assertion| assertion.readback_probe_contract_ids.len())
            .sum(),
        readback_evidence_field_ref_count: probe_binding_assertions
            .iter()
            .map(|assertion| assertion.readback_evidence_fields.len())
            .sum(),
        drift_detector_count: drift_detectors.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_plans,
        key_formula_assertions,
        collision_policy_assertions,
        probe_binding_assertions,
        collection_ref_assertions,
        drift_detectors,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_store_idempotency_guard_gap_closure_application_preview: true,
        ready_for_runtime_guard_application: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphStoreIdempotencyGuardGapClosureReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_store_idempotency_guard_gap_closure_readback_plans()
-> Vec<WorkGraphStoreIdempotencyGuardReadbackPlanPreview> {
    work_graph_store_idempotency_guard_gap_closure_plans()
        .into_iter()
        .filter_map(|plan| {
            let guard = guard_for_plan(&plan)?;
            Some(readback_plan(&plan, &guard))
        })
        .collect()
}

pub fn work_graph_store_idempotency_guard_key_formula_readback_assertions()
-> Vec<WorkGraphStoreIdempotencyGuardKeyFormulaReadbackAssertionPreview> {
    work_graph_store_idempotency_guard_gap_candidate_guards()
        .into_iter()
        .map(
            |guard| WorkGraphStoreIdempotencyGuardKeyFormulaReadbackAssertionPreview {
                id: key_formula_assertion_id_for_source(guard.source_surface_id),
                source_surface_id: guard.source_surface_id,
                candidate_guard_id: guard.id,
                key_fields: guard.key_fields,
                key_formula: guard.key_formula,
                replay_scope: guard.replay_scope,
                redaction_policy: guard.redaction_policy,
                requires_sha256_formula: guard.key_formula.starts_with("sha256("),
                mutates_idempotency_index: false,
            },
        )
        .collect()
}

pub fn work_graph_store_idempotency_guard_collision_policy_readback_assertions()
-> Vec<WorkGraphStoreIdempotencyGuardCollisionPolicyReadbackAssertionPreview> {
    work_graph_store_idempotency_guard_gap_candidate_guards()
        .into_iter()
        .map(
            |guard| WorkGraphStoreIdempotencyGuardCollisionPolicyReadbackAssertionPreview {
                id: collision_policy_assertion_id_for_source(guard.source_surface_id),
                source_surface_id: guard.source_surface_id,
                candidate_guard_id: guard.id,
                collision_policy: guard.collision_policy,
                required_before_append_only_intake: guard.required_before_append_only_intake,
                expected_collision_state: "collision_blocks_duplicate_projection_preview_only",
                mutates_idempotency_index: false,
            },
        )
        .collect()
}

pub fn work_graph_store_idempotency_guard_probe_binding_readback_assertions()
-> Vec<WorkGraphStoreIdempotencyGuardProbeBindingReadbackAssertionPreview> {
    work_graph_store_idempotency_guard_gap_probe_bindings()
        .into_iter()
        .map(
            |binding| WorkGraphStoreIdempotencyGuardProbeBindingReadbackAssertionPreview {
                id: probe_binding_assertion_id_for_source(binding.source_surface_id),
                source_surface_id: binding.source_surface_id,
                candidate_guard_id: binding.candidate_guard_id,
                readback_probe_contract_ids: binding.readback_probe_contract_ids,
                target_collection_ids: binding.target_collection_ids,
                readback_evidence_fields: binding.readback_evidence_fields,
                drift_detector_ids: binding.drift_detector_ids,
                expected_probe_binding_state: "probe_contract_shape_defined_readback_disabled",
                performs_readback: false,
                mutates_store: false,
            },
        )
        .collect()
}

pub fn work_graph_store_idempotency_guard_collection_ref_readback_assertions()
-> Vec<WorkGraphStoreIdempotencyGuardCollectionRefReadbackAssertionPreview> {
    work_graph_store_idempotency_guard_gap_bindings()
        .into_iter()
        .filter_map(|binding| {
            let probe_binding = probe_binding_for_source(binding.source_surface_id)?;
            Some(
                WorkGraphStoreIdempotencyGuardCollectionRefReadbackAssertionPreview {
                    id: collection_ref_assertion_id_for_source(binding.source_surface_id),
                    source_surface_id: binding.source_surface_id,
                    candidate_guard_id: binding.candidate_guard_id,
                    required_collection_count: binding.expected_collection_ids.len(),
                    expected_collection_ids: binding.expected_collection_ids,
                    required_readback_probe_contract_ids: probe_binding.readback_probe_contract_ids,
                    expected_guard_binding_state: binding.closure_state,
                    mutates_store: false,
                },
            )
        })
        .collect()
}

pub fn work_graph_store_idempotency_guard_readback_drift_detectors()
-> Vec<WorkGraphStoreIdempotencyGuardReadbackDriftDetectorPreview> {
    vec![
        drift_detector(
            "store_guard_key_formula_drift",
            vec!["keyFields", "keyFormula", "replayScope"],
            "high",
        ),
        drift_detector(
            "store_guard_collision_policy_drift",
            vec!["collisionPolicy", "requiredBeforeAppendOnlyIntake"],
            "high",
        ),
        drift_detector(
            "store_guard_probe_contract_drift",
            vec!["readbackProbeContractIds", "readbackEvidenceFields"],
            "high",
        ),
        drift_detector(
            "store_guard_collection_ref_drift",
            vec!["expectedCollectionIds", "targetCollectionIds"],
            "medium",
        ),
        drift_detector(
            "store_guard_redaction_policy_drift",
            vec!["redactionPolicy", "driftDetectorIds"],
            "medium",
        ),
    ]
}

pub fn work_graph_store_idempotency_guard_gap_closure_readback_blockers()
-> Vec<WorkGraphStoreIdempotencyGuardReadbackBlockerPreview> {
    let source_ids = store_guard_gap_source_surface_ids();
    vec![
        blocker(
            "readback_execution_disabled",
            "high",
            source_ids.clone(),
            "this preview defines readback assertions but does not query or mutate the WorkGraph store",
        ),
        blocker(
            "runtime_guard_application_disabled",
            "high",
            source_ids.clone(),
            "runtime guard application remains disabled until readback assertions are promoted",
        ),
        blocker(
            "state_store_guard_persistence_disabled",
            "high",
            source_ids.clone(),
            "candidate guard rows remain preview-only and are not persisted to state store",
        ),
        blocker(
            "append_only_store_enablement_disabled",
            "high",
            source_ids.clone(),
            "append-only store enablement remains blocked until guard readback and operator review pass",
        ),
        blocker(
            "task_result_enforcement_disabled",
            "high",
            vec![
                "hepta_runtime_multi_agent_reducer",
                "hepta_runtime_task_board",
            ],
            "TaskResult-producing guard assertions still need terminal TaskResult enforcement before runtime use",
        ),
        blocker(
            "operator_review_required",
            "medium",
            source_ids,
            "operator review must accept guard formulas, collision policy, and redaction before application preview promotion",
        ),
    ]
}

pub fn work_graph_store_idempotency_guard_gap_closure_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = work_graph_store_idempotency_guard_gap_closure_required_prior_gates();
    gates.push(WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_PREVIEW_GATE);
    gates
}

impl WorkGraphStoreIdempotencyGuardGapClosureReadbackPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            idempotency_index_mutated: false,
            store_guard_attached: false,
            append_only_store_enabled: false,
            projection_enforcement_enabled: false,
            readback_performed: false,
            task_result_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            role_manifest_enforcement_enabled: false,
            approval_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn readback_plan(
    plan: &WorkGraphStoreIdempotencyGuardClosurePlanPreview,
    guard: &WorkGraphStoreIdempotencyCandidateGuardPreview,
) -> WorkGraphStoreIdempotencyGuardReadbackPlanPreview {
    WorkGraphStoreIdempotencyGuardReadbackPlanPreview {
        id: readback_plan_id_for_source(plan.source_surface_id),
        source_surface_id: plan.source_surface_id,
        closure_plan_id: plan.id,
        candidate_guard_id: plan.candidate_guard_id,
        key_formula_assertion_id: key_formula_assertion_id_for_source(plan.source_surface_id),
        collision_policy_assertion_id: collision_policy_assertion_id_for_source(
            plan.source_surface_id,
        ),
        probe_binding_assertion_id: probe_binding_assertion_id_for_source(plan.source_surface_id),
        collection_ref_assertion_id: collection_ref_assertion_id_for_source(plan.source_surface_id),
        expected_key_fields: guard.key_fields.clone(),
        expected_collection_ids: plan.expected_collection_ids.clone(),
        readback_probe_contract_ids: plan.readback_probe_contract_ids.clone(),
        required_before_runtime_guard_application: true,
        readback_state: "readback_assertions_defined_execution_disabled",
        performs_readback: false,
        mutates_store: false,
    }
}

fn drift_detector(
    id: &'static str,
    compared_field_ids: Vec<&'static str>,
    severity: &'static str,
) -> WorkGraphStoreIdempotencyGuardReadbackDriftDetectorPreview {
    WorkGraphStoreIdempotencyGuardReadbackDriftDetectorPreview {
        id,
        compared_field_ids,
        severity,
        blocks_runtime_guard_application: true,
        performs_readback: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphStoreIdempotencyGuardReadbackBlockerPreview {
    WorkGraphStoreIdempotencyGuardReadbackBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        required_before_projection_enforcement: true,
        recommended_fix,
    }
}

fn guard_for_plan(
    plan: &WorkGraphStoreIdempotencyGuardClosurePlanPreview,
) -> Option<WorkGraphStoreIdempotencyCandidateGuardPreview> {
    work_graph_store_idempotency_guard_gap_candidate_guards()
        .into_iter()
        .find(|guard| guard.id == plan.candidate_guard_id)
}

fn probe_binding_for_source(
    source_surface_id: &str,
) -> Option<WorkGraphStoreIdempotencyGuardProbeBindingPreview> {
    work_graph_store_idempotency_guard_gap_probe_bindings()
        .into_iter()
        .find(|binding| binding.source_surface_id == source_surface_id)
}

fn store_guard_gap_source_surface_ids() -> Vec<&'static str> {
    work_graph_store_idempotency_guard_gap_closure_plans()
        .iter()
        .map(|plan| plan.source_surface_id)
        .collect()
}

fn readback_plan_id_for_source(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "plan_mode_proposed_plan_blocks" => {
            "plan_mode_proposed_plan_blocks_store_guard_readback_plan"
        }
        "app_server_turn_plan_notification" => {
            "app_server_turn_plan_notification_store_guard_readback_plan"
        }
        "multi_agent_v2_mailbox_wait" => "multi_agent_v2_mailbox_wait_store_guard_readback_plan",
        "hepta_runtime_multi_agent_reducer" => {
            "hepta_runtime_multi_agent_reducer_store_guard_readback_plan"
        }
        "hepta_runtime_task_board" => "hepta_runtime_task_board_store_guard_readback_plan",
        _ => "unknown_store_guard_readback_plan",
    }
}

fn key_formula_assertion_id_for_source(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "plan_mode_proposed_plan_blocks" => {
            "plan_mode_proposed_plan_blocks_key_formula_readback_assertion"
        }
        "app_server_turn_plan_notification" => {
            "app_server_turn_plan_notification_key_formula_readback_assertion"
        }
        "multi_agent_v2_mailbox_wait" => {
            "multi_agent_v2_mailbox_wait_key_formula_readback_assertion"
        }
        "hepta_runtime_multi_agent_reducer" => {
            "hepta_runtime_multi_agent_reducer_key_formula_readback_assertion"
        }
        "hepta_runtime_task_board" => "hepta_runtime_task_board_key_formula_readback_assertion",
        _ => "unknown_key_formula_readback_assertion",
    }
}

fn collision_policy_assertion_id_for_source(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "plan_mode_proposed_plan_blocks" => {
            "plan_mode_proposed_plan_blocks_collision_policy_readback_assertion"
        }
        "app_server_turn_plan_notification" => {
            "app_server_turn_plan_notification_collision_policy_readback_assertion"
        }
        "multi_agent_v2_mailbox_wait" => {
            "multi_agent_v2_mailbox_wait_collision_policy_readback_assertion"
        }
        "hepta_runtime_multi_agent_reducer" => {
            "hepta_runtime_multi_agent_reducer_collision_policy_readback_assertion"
        }
        "hepta_runtime_task_board" => {
            "hepta_runtime_task_board_collision_policy_readback_assertion"
        }
        _ => "unknown_collision_policy_readback_assertion",
    }
}

fn probe_binding_assertion_id_for_source(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "plan_mode_proposed_plan_blocks" => {
            "plan_mode_proposed_plan_blocks_probe_binding_readback_assertion"
        }
        "app_server_turn_plan_notification" => {
            "app_server_turn_plan_notification_probe_binding_readback_assertion"
        }
        "multi_agent_v2_mailbox_wait" => {
            "multi_agent_v2_mailbox_wait_probe_binding_readback_assertion"
        }
        "hepta_runtime_multi_agent_reducer" => {
            "hepta_runtime_multi_agent_reducer_probe_binding_readback_assertion"
        }
        "hepta_runtime_task_board" => "hepta_runtime_task_board_probe_binding_readback_assertion",
        _ => "unknown_probe_binding_readback_assertion",
    }
}

fn collection_ref_assertion_id_for_source(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "plan_mode_proposed_plan_blocks" => {
            "plan_mode_proposed_plan_blocks_collection_ref_readback_assertion"
        }
        "app_server_turn_plan_notification" => {
            "app_server_turn_plan_notification_collection_ref_readback_assertion"
        }
        "multi_agent_v2_mailbox_wait" => {
            "multi_agent_v2_mailbox_wait_collection_ref_readback_assertion"
        }
        "hepta_runtime_multi_agent_reducer" => {
            "hepta_runtime_multi_agent_reducer_collection_ref_readback_assertion"
        }
        "hepta_runtime_task_board" => "hepta_runtime_task_board_collection_ref_readback_assertion",
        _ => "unknown_collection_ref_readback_assertion",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_idempotency_guard_readback_targets_all_candidate_guards() {
        let report = hepta_work_graph_store_idempotency_guard_gap_closure_readback_preview_report();
        let source_surface_ids = report
            .readback_plans
            .iter()
            .map(|plan| plan.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(
            source_surface_ids,
            [
                "plan_mode_proposed_plan_blocks",
                "app_server_turn_plan_notification",
                "multi_agent_v2_mailbox_wait",
                "hepta_runtime_multi_agent_reducer",
                "hepta_runtime_task_board",
            ]
        );
        assert_eq!(report.closure_plan_count, 5);
        assert_eq!(report.candidate_guard_count, 5);
        assert_eq!(report.readback_plan_count, 5);
        assert_eq!(report.key_formula_assertion_count, 5);
        assert_eq!(report.collision_policy_assertion_count, 5);
        assert_eq!(report.probe_binding_assertion_count, 5);
        assert_eq!(report.collection_ref_assertion_count, 5);
    }

    #[test]
    fn store_idempotency_guard_readback_checks_key_formula_and_collision_policy() {
        let report = hepta_work_graph_store_idempotency_guard_gap_closure_readback_preview_report();
        let guard_ids = report
            .key_formula_assertions
            .iter()
            .map(|assertion| assertion.candidate_guard_id)
            .collect::<Vec<_>>();

        assert_eq!(
            guard_ids,
            [
                "plan_mode_proposed_plan_blocks_store_idempotency_guard",
                "app_server_turn_plan_notification_store_idempotency_guard",
                "multi_agent_v2_mailbox_wait_store_idempotency_guard",
                "hepta_runtime_multi_agent_reducer_store_idempotency_guard",
                "hepta_runtime_task_board_store_idempotency_guard",
            ]
        );
        assert!(report.key_formula_assertions.iter().all(|assertion| {
            assertion.requires_sha256_formula
                && !assertion.key_fields.is_empty()
                && !assertion.mutates_idempotency_index
        }));
        assert!(report.collision_policy_assertions.iter().all(|assertion| {
            assertion.required_before_append_only_intake
                && assertion.expected_collision_state
                    == "collision_blocks_duplicate_projection_preview_only"
                && !assertion.mutates_idempotency_index
        }));
    }

    #[test]
    fn store_idempotency_guard_readback_preserves_probe_and_collection_refs() {
        let report = hepta_work_graph_store_idempotency_guard_gap_closure_readback_preview_report();
        let probe_counts = report
            .probe_binding_assertions
            .iter()
            .map(|assertion| assertion.readback_probe_contract_ids.len())
            .collect::<Vec<_>>();
        let collection_counts = report
            .collection_ref_assertions
            .iter()
            .map(|assertion| assertion.required_collection_count)
            .collect::<Vec<_>>();

        assert_eq!(probe_counts, [3, 3, 2, 2, 4]);
        assert_eq!(collection_counts, [3, 3, 2, 2, 4]);
        assert_eq!(report.expected_collection_ref_count, 14);
        assert_eq!(report.readback_probe_contract_ref_count, 14);
        assert_eq!(report.readback_evidence_field_ref_count, 39);
        assert_eq!(report.drift_detector_count, 5);
        assert!(
            report
                .probe_binding_assertions
                .iter()
                .all(|assertion| !assertion.performs_readback && !assertion.mutates_store)
        );
        assert!(
            report
                .collection_ref_assertions
                .iter()
                .all(|assertion| !assertion.mutates_store)
        );
    }

    #[test]
    fn store_idempotency_guard_readback_preserves_blockers_and_next_frontier() {
        let report = hepta_work_graph_store_idempotency_guard_gap_closure_readback_preview_report();
        let blocker_counts = report
            .blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(
            blocker_counts,
            [
                ("readback_execution_disabled", 5),
                ("runtime_guard_application_disabled", 5),
                ("state_store_guard_persistence_disabled", 5),
                ("append_only_store_enablement_disabled", 5),
                ("task_result_enforcement_disabled", 2),
                ("operator_review_required", 5),
            ]
        );
        assert_eq!(report.blocker_count, 6);
        assert_eq!(report.required_prior_gate_count, 18);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_READBACK_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_store_idempotency_guard_gap_closure_application_preview);
        assert!(!report.ready_for_runtime_guard_application);
    }

    #[test]
    fn store_idempotency_guard_readback_keeps_side_effects_disabled() {
        let side_effects =
            hepta_work_graph_store_idempotency_guard_gap_closure_readback_preview_report()
                .side_effects;

        assert_eq!(
            side_effects,
            WorkGraphStoreIdempotencyGuardGapClosureReadbackPreviewSideEffects {
                filesystem_written: false,
                graph_state_persisted: false,
                wal_written: false,
                idempotency_index_mutated: false,
                store_guard_attached: false,
                append_only_store_enabled: false,
                projection_enforcement_enabled: false,
                readback_performed: false,
                task_result_enforcement_enabled: false,
                scheduler_admission_enforced: false,
                role_manifest_enforcement_enabled: false,
                approval_recorded: false,
                runtime_mutation_performed: false,
                agent_spawn_performed: false,
                external_send_performed: false,
                model_invoked: false,
            }
        );
    }
}
