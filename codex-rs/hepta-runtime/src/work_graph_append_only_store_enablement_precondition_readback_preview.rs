use serde::Serialize;

use crate::work_graph_append_only_store_enablement_precondition_preview::WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_PREVIEW_GATE;
use crate::work_graph_append_only_store_enablement_precondition_preview::WorkGraphAppendOnlyStoreEnablementPreconditionBlockerPreview;
use crate::work_graph_append_only_store_enablement_precondition_preview::WorkGraphAppendOnlyStoreEnablementPreconditionPreview;
use crate::work_graph_append_only_store_enablement_precondition_preview::work_graph_append_only_store_enablement_precondition_blockers;
use crate::work_graph_append_only_store_enablement_precondition_preview::work_graph_append_only_store_enablement_precondition_required_prior_gates;
use crate::work_graph_append_only_store_enablement_precondition_preview::work_graph_append_only_store_enablement_preconditions;
use crate::work_graph_append_only_store_enablement_precondition_preview::work_graph_append_only_store_enablement_source_precondition_decisions;

pub const WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_store_enablement_precondition_readback_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_READBACK_SCHEMA_VERSION: &str =
    "work_graph_append_only_store_enablement_precondition_readback_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_append_only_store_enablement_precondition_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreEnablementPreconditionReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub precondition_count: usize,
    pub source_precondition_decision_count: usize,
    pub readback_plan_count: usize,
    pub contract_ref_assertion_count: usize,
    pub source_coverage_assertion_count: usize,
    pub blocker_mapping_assertion_count: usize,
    pub readback_evidence_field_ref_count: usize,
    pub contract_ref_count: usize,
    pub precondition_source_ref_count: usize,
    pub drift_detector_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_plans: Vec<WorkGraphAppendOnlyStorePreconditionReadbackPlanPreview>,
    pub contract_ref_assertions:
        Vec<WorkGraphAppendOnlyStorePreconditionContractRefReadbackAssertionPreview>,
    pub source_coverage_assertions:
        Vec<WorkGraphAppendOnlyStorePreconditionSourceCoverageReadbackAssertionPreview>,
    pub blocker_mapping_assertions:
        Vec<WorkGraphAppendOnlyStorePreconditionBlockerMappingReadbackAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphAppendOnlyStorePreconditionReadbackDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphAppendOnlyStorePreconditionReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_precondition_application_preview: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyStoreEnablementPreconditionReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStorePreconditionReadbackPlanPreview {
    pub precondition_id: &'static str,
    pub category: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub expected_contract_ref_ids: Vec<&'static str>,
    pub expected_blocker_id: &'static str,
    pub required_evidence_fields: Vec<&'static str>,
    pub readback_scope: &'static str,
    pub expected_preview_state: &'static str,
    pub required_before_precondition_application: bool,
    pub performs_readback: bool,
    pub mutates_store: bool,
    pub enables_append_only_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStorePreconditionContractRefReadbackAssertionPreview {
    pub assertion_id: &'static str,
    pub precondition_id: &'static str,
    pub category: &'static str,
    pub expected_contract_ref_ids: Vec<&'static str>,
    pub expected_contract_ref_count: usize,
    pub expected_contract_ref_state: &'static str,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStorePreconditionSourceCoverageReadbackAssertionPreview {
    pub assertion_id: &'static str,
    pub precondition_id: &'static str,
    pub category: &'static str,
    pub expected_source_surface_ids: Vec<&'static str>,
    pub expected_source_surface_count: usize,
    pub expected_coverage_state: &'static str,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStorePreconditionBlockerMappingReadbackAssertionPreview {
    pub assertion_id: &'static str,
    pub blocker_id: &'static str,
    pub category: &'static str,
    pub affected_precondition_ids: Vec<&'static str>,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub expected_blocker_state: &'static str,
    pub required_before_append_only_store_enablement: bool,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStorePreconditionReadbackDriftDetectorPreview {
    pub id: &'static str,
    pub compared_field_ids: Vec<&'static str>,
    pub severity: &'static str,
    pub blocks_precondition_application: bool,
    pub performs_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStorePreconditionReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_precondition_ids: Vec<&'static str>,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_precondition_application: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreEnablementPreconditionReadbackPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub idempotency_index_mutated: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub readback_executed: bool,
    pub rollback_executed: bool,
    pub scheduler_admission_enforced: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub task_result_enforcement_enabled: bool,
    pub runtime_wrapper_attached: bool,
    pub approval_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_store_enablement_precondition_readback_preview_report()
-> WorkGraphAppendOnlyStoreEnablementPreconditionReadbackPreviewReport {
    let preconditions = work_graph_append_only_store_enablement_preconditions();
    let source_precondition_decisions =
        work_graph_append_only_store_enablement_source_precondition_decisions();
    let readback_plans = work_graph_append_only_store_enablement_precondition_readback_plans();
    let contract_ref_assertions =
        work_graph_append_only_store_enablement_precondition_contract_ref_readback_assertions();
    let source_coverage_assertions =
        work_graph_append_only_store_enablement_precondition_source_coverage_readback_assertions();
    let blocker_mapping_assertions =
        work_graph_append_only_store_enablement_precondition_blocker_mapping_readback_assertions();
    let drift_detectors =
        work_graph_append_only_store_enablement_precondition_readback_drift_detectors();
    let blockers = work_graph_append_only_store_enablement_precondition_readback_blockers();
    let required_prior_gates =
        work_graph_append_only_store_enablement_precondition_readback_required_prior_gates();

    WorkGraphAppendOnlyStoreEnablementPreconditionReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_READBACK_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_append_only_store_enablement_precondition_readback_preview_no_execution",
        precondition_count: preconditions.len(),
        source_precondition_decision_count: source_precondition_decisions.len(),
        readback_plan_count: readback_plans.len(),
        contract_ref_assertion_count: contract_ref_assertions.len(),
        source_coverage_assertion_count: source_coverage_assertions.len(),
        blocker_mapping_assertion_count: blocker_mapping_assertions.len(),
        readback_evidence_field_ref_count: readback_plans
            .iter()
            .map(|plan| plan.required_evidence_fields.len())
            .sum(),
        contract_ref_count: contract_ref_assertions
            .iter()
            .map(|assertion| assertion.expected_contract_ref_count)
            .sum(),
        precondition_source_ref_count: source_coverage_assertions
            .iter()
            .map(|assertion| assertion.expected_source_surface_count)
            .sum(),
        drift_detector_count: drift_detectors.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_plans,
        contract_ref_assertions,
        source_coverage_assertions,
        blocker_mapping_assertions,
        drift_detectors,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_precondition_application_preview: true,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyStoreEnablementPreconditionReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_store_enablement_precondition_readback_plans()
-> Vec<WorkGraphAppendOnlyStorePreconditionReadbackPlanPreview> {
    work_graph_append_only_store_enablement_preconditions()
        .into_iter()
        .map(readback_plan)
        .collect()
}

pub fn work_graph_append_only_store_enablement_precondition_contract_ref_readback_assertions()
-> Vec<WorkGraphAppendOnlyStorePreconditionContractRefReadbackAssertionPreview> {
    work_graph_append_only_store_enablement_preconditions()
        .into_iter()
        .map(|precondition| {
            let assertion_id = assertion_id_for(precondition.id, "contract_refs");
            WorkGraphAppendOnlyStorePreconditionContractRefReadbackAssertionPreview {
                assertion_id,
                precondition_id: precondition.id,
                category: precondition.category,
                expected_contract_ref_count: precondition.required_contract_refs.len(),
                expected_contract_ref_ids: precondition.required_contract_refs,
                expected_contract_ref_state: if precondition.satisfied_by_preview_contracts {
                    "preview_contract_refs_present_enablement_still_disabled"
                } else {
                    "operator_or_enforcement_contract_refs_missing_for_enablement"
                },
                performs_readback: false,
                mutates_store: false,
            }
        })
        .collect()
}

pub fn work_graph_append_only_store_enablement_precondition_source_coverage_readback_assertions()
-> Vec<WorkGraphAppendOnlyStorePreconditionSourceCoverageReadbackAssertionPreview> {
    work_graph_append_only_store_enablement_preconditions()
        .into_iter()
        .map(|precondition| {
            let assertion_id = assertion_id_for(precondition.id, "source_coverage");
            WorkGraphAppendOnlyStorePreconditionSourceCoverageReadbackAssertionPreview {
                assertion_id,
                precondition_id: precondition.id,
                category: precondition.category,
                expected_source_surface_count: precondition.affected_source_surface_ids.len(),
                expected_source_surface_ids: precondition.affected_source_surface_ids,
                expected_coverage_state: "source_coverage_declared_readback_not_executed",
                performs_readback: false,
                mutates_store: false,
            }
        })
        .collect()
}

pub fn work_graph_append_only_store_enablement_precondition_blocker_mapping_readback_assertions()
-> Vec<WorkGraphAppendOnlyStorePreconditionBlockerMappingReadbackAssertionPreview> {
    work_graph_append_only_store_enablement_precondition_blockers()
        .into_iter()
        .map(|blocker| {
            let assertion_id = assertion_id_for(blocker.id, "blocker_mapping");
            WorkGraphAppendOnlyStorePreconditionBlockerMappingReadbackAssertionPreview {
                assertion_id,
                blocker_id: blocker.id,
                category: blocker.category,
                affected_precondition_ids: blocker.affected_precondition_ids,
                affected_source_surface_ids: blocker.affected_source_surface_ids,
                expected_blocker_state: "blocks_append_only_store_enablement_until_readback_and_application_preview",
                required_before_append_only_store_enablement: blocker
                    .required_before_append_only_store_enablement,
                performs_readback: false,
                mutates_store: false,
            }
        })
        .collect()
}

pub fn work_graph_append_only_store_enablement_precondition_readback_drift_detectors()
-> Vec<WorkGraphAppendOnlyStorePreconditionReadbackDriftDetectorPreview> {
    vec![
        drift_detector(
            "append_only_precondition_source_coverage_drift",
            vec![
                "precondition_id",
                "affected_source_surface_ids",
                "source_precondition_decisions",
            ],
            "critical",
        ),
        drift_detector(
            "append_only_precondition_contract_ref_drift",
            vec![
                "required_contract_refs",
                "append_only_event_contracts",
                "wal_operations",
                "idempotency_guards",
                "readback_probes",
            ],
            "critical",
        ),
        drift_detector(
            "append_only_precondition_blocker_mapping_drift",
            vec![
                "blocker_id",
                "affected_precondition_ids",
                "affected_source_surface_ids",
            ],
            "high",
        ),
        drift_detector(
            "append_only_precondition_side_effect_lock_drift",
            vec![
                "side_effects",
                "append_only_store_enabled",
                "wal_written",
                "readback_executed",
            ],
            "critical",
        ),
        drift_detector(
            "append_only_precondition_decision_distribution_drift",
            vec![
                "append_only_precondition_decision",
                "scheduler_admission_not_enforced",
                "role_manifest_not_enforced",
            ],
            "medium",
        ),
        drift_detector(
            "append_only_precondition_prior_gate_drift",
            vec![
                "required_prior_gates",
                "precondition_gate",
                "terminal_task_result_readiness_rerun_gate",
            ],
            "medium",
        ),
    ]
}

pub fn work_graph_append_only_store_enablement_precondition_readback_blockers()
-> Vec<WorkGraphAppendOnlyStorePreconditionReadbackBlockerPreview> {
    let preconditions = work_graph_append_only_store_enablement_preconditions();
    let all_precondition_ids = preconditions
        .iter()
        .map(|precondition| precondition.id)
        .collect::<Vec<_>>();
    let all_sources = work_graph_append_only_store_enablement_source_precondition_decisions()
        .into_iter()
        .map(|decision| decision.source_surface_id)
        .collect::<Vec<_>>();
    let mut blockers = vec![readback_blocker(
        "readback_execution_disabled",
        "critical",
        "readback_execution",
        all_precondition_ids,
        all_sources,
        "keep this gate preview-only until readback execution and rollback fixtures are explicitly promoted",
    )];
    blockers.extend(
        work_graph_append_only_store_enablement_precondition_blockers()
            .into_iter()
            .map(readback_blocker_from_precondition_blocker),
    );
    blockers
}

pub fn work_graph_append_only_store_enablement_precondition_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = work_graph_append_only_store_enablement_precondition_required_prior_gates();
    if !gates.contains(&WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_PREVIEW_GATE) {
        gates.push(WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_PREVIEW_GATE);
    }
    gates
}

impl WorkGraphAppendOnlyStoreEnablementPreconditionReadbackPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            idempotency_index_mutated: false,
            append_only_store_enabled: false,
            projection_enforcement_enabled: false,
            readback_executed: false,
            rollback_executed: false,
            scheduler_admission_enforced: false,
            role_manifest_enforcement_enabled: false,
            task_result_enforcement_enabled: false,
            runtime_wrapper_attached: false,
            approval_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn readback_plan(
    precondition: WorkGraphAppendOnlyStoreEnablementPreconditionPreview,
) -> WorkGraphAppendOnlyStorePreconditionReadbackPlanPreview {
    WorkGraphAppendOnlyStorePreconditionReadbackPlanPreview {
        precondition_id: precondition.id,
        category: precondition.category,
        severity: precondition.severity,
        affected_source_surface_ids: precondition.affected_source_surface_ids,
        expected_contract_ref_ids: precondition.required_contract_refs,
        expected_blocker_id: precondition.blocker_id,
        required_evidence_fields: evidence_fields_for_category(precondition.category),
        readback_scope: readback_scope_for_category(precondition.category),
        expected_preview_state: if precondition.satisfied_by_preview_contracts {
            "preview_contract_ready_enablement_blocked"
        } else {
            "operator_or_enforcement_contract_missing_enablement_blocked"
        },
        required_before_precondition_application: true,
        performs_readback: false,
        mutates_store: false,
        enables_append_only_store: false,
    }
}

fn evidence_fields_for_category(category: &'static str) -> Vec<&'static str> {
    match category {
        "durable_store_switch" => vec![
            "preconditionId",
            "sourceSurfaceIds",
            "eventContractRefs",
            "durableStoreSwitchState",
            "operatorReviewRef",
        ],
        "wal_boundary" => vec![
            "preconditionId",
            "walOperationIds",
            "appendOrderingRule",
            "walWriteBoundaryState",
            "rollbackPlanRef",
        ],
        "idempotency_mutation_policy" => vec![
            "preconditionId",
            "idempotencyGuardIds",
            "collisionPolicyRefs",
            "mutationPolicyState",
            "replayProbeRef",
        ],
        "rollback_readback_gate" => vec![
            "preconditionId",
            "checkpointContractRefs",
            "readbackProbeRefs",
            "rollbackPlanRef",
            "replayDeterminismRef",
        ],
        "operator_review" => vec![
            "preconditionId",
            "operatorReviewRef",
            "sideEffectLockState",
            "runtimeReceiptRefs",
            "terminalTaskResultRefs",
        ],
        "scheduler_admission" => vec![
            "preconditionId",
            "dependencyGateRef",
            "leaseGateRef",
            "budgetGateRef",
            "approvalGateRef",
            "idempotencyGateRef",
        ],
        "role_manifest" => vec![
            "preconditionId",
            "roleCapabilityRef",
            "toolPermissionRef",
            "budgetLimitRef",
            "laneBoundaryRef",
        ],
        _ => vec!["preconditionId", "unknownCategoryRef"],
    }
}

fn readback_scope_for_category(category: &'static str) -> &'static str {
    match category {
        "durable_store_switch" => "durable_store_switch_state",
        "wal_boundary" => "wal_boundary_contract_refs",
        "idempotency_mutation_policy" => "idempotency_mutation_policy_refs",
        "rollback_readback_gate" => "rollback_and_readback_probe_refs",
        "operator_review" => "operator_review_and_side_effect_lock_refs",
        "scheduler_admission" => "scheduler_admission_gate_refs",
        "role_manifest" => "role_manifest_policy_refs",
        _ => "unknown_precondition_scope",
    }
}

fn readback_blocker_from_precondition_blocker(
    blocker: WorkGraphAppendOnlyStoreEnablementPreconditionBlockerPreview,
) -> WorkGraphAppendOnlyStorePreconditionReadbackBlockerPreview {
    readback_blocker(
        blocker.id,
        blocker.severity,
        blocker.category,
        blocker.affected_precondition_ids,
        blocker.affected_source_surface_ids,
        blocker.recommended_fix,
    )
}

fn readback_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_precondition_ids: Vec<&'static str>,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphAppendOnlyStorePreconditionReadbackBlockerPreview {
    WorkGraphAppendOnlyStorePreconditionReadbackBlockerPreview {
        id,
        severity,
        category,
        affected_precondition_ids,
        affected_source_surface_ids,
        required_before_precondition_application: true,
        recommended_fix,
    }
}

fn drift_detector(
    id: &'static str,
    compared_field_ids: Vec<&'static str>,
    severity: &'static str,
) -> WorkGraphAppendOnlyStorePreconditionReadbackDriftDetectorPreview {
    WorkGraphAppendOnlyStorePreconditionReadbackDriftDetectorPreview {
        id,
        compared_field_ids,
        severity,
        blocks_precondition_application: true,
        performs_readback: false,
    }
}

fn assertion_id_for(id: &'static str, suffix: &str) -> &'static str {
    match (id, suffix) {
        ("durable_store_enablement_switch", "contract_refs") => {
            "durable_store_enablement_switch_contract_refs_readback"
        }
        ("wal_append_boundary_contract", "contract_refs") => {
            "wal_append_boundary_contract_refs_readback"
        }
        ("idempotency_mutation_policy", "contract_refs") => {
            "idempotency_mutation_policy_contract_refs_readback"
        }
        ("rollback_readback_gate", "contract_refs") => {
            "rollback_readback_gate_contract_refs_readback"
        }
        ("operator_review_and_side_effect_lock", "contract_refs") => {
            "operator_review_side_effect_lock_contract_refs_readback"
        }
        ("scheduler_admission_enforcement_precondition", "contract_refs") => {
            "scheduler_admission_enforcement_contract_refs_readback"
        }
        ("role_manifest_enforcement_precondition", "contract_refs") => {
            "role_manifest_enforcement_contract_refs_readback"
        }
        ("durable_store_enablement_switch", "source_coverage") => {
            "durable_store_enablement_switch_source_coverage_readback"
        }
        ("wal_append_boundary_contract", "source_coverage") => {
            "wal_append_boundary_source_coverage_readback"
        }
        ("idempotency_mutation_policy", "source_coverage") => {
            "idempotency_mutation_policy_source_coverage_readback"
        }
        ("rollback_readback_gate", "source_coverage") => {
            "rollback_readback_gate_source_coverage_readback"
        }
        ("operator_review_and_side_effect_lock", "source_coverage") => {
            "operator_review_side_effect_lock_source_coverage_readback"
        }
        ("scheduler_admission_enforcement_precondition", "source_coverage") => {
            "scheduler_admission_enforcement_source_coverage_readback"
        }
        ("role_manifest_enforcement_precondition", "source_coverage") => {
            "role_manifest_enforcement_source_coverage_readback"
        }
        ("durable_store_enablement_disabled", "blocker_mapping") => {
            "durable_store_enablement_disabled_blocker_mapping_readback"
        }
        ("wal_write_boundary_not_enabled", "blocker_mapping") => {
            "wal_write_boundary_not_enabled_blocker_mapping_readback"
        }
        ("idempotency_index_mutation_disabled", "blocker_mapping") => {
            "idempotency_index_mutation_disabled_blocker_mapping_readback"
        }
        ("rollback_readback_not_executed", "blocker_mapping") => {
            "rollback_readback_not_executed_blocker_mapping_readback"
        }
        ("operator_review_required", "blocker_mapping") => {
            "operator_review_required_blocker_mapping_readback"
        }
        ("scheduler_admission_not_enforced", "blocker_mapping") => {
            "scheduler_admission_not_enforced_blocker_mapping_readback"
        }
        ("role_manifest_not_enforced", "blocker_mapping") => {
            "role_manifest_not_enforced_blocker_mapping_readback"
        }
        ("runtime_application_residuals_not_promoted", "blocker_mapping") => {
            "runtime_application_residuals_not_promoted_blocker_mapping_readback"
        }
        _ => "unknown_append_only_store_precondition_readback_assertion",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_only_store_precondition_readback_summarizes_assertions() {
        let report =
            hepta_work_graph_append_only_store_enablement_precondition_readback_preview_report();

        assert_eq!(report.precondition_count, 7);
        assert_eq!(report.source_precondition_decision_count, 12);
        assert_eq!(report.readback_plan_count, 7);
        assert_eq!(report.contract_ref_assertion_count, 7);
        assert_eq!(report.source_coverage_assertion_count, 7);
        assert_eq!(report.blocker_mapping_assertion_count, 8);
        assert_eq!(report.readback_evidence_field_ref_count, 36);
        assert_eq!(report.contract_ref_count, 54);
        assert_eq!(report.precondition_source_ref_count, 63);
        assert_eq!(report.drift_detector_count, 6);
        assert_eq!(report.blocker_count, 9);
    }

    #[test]
    fn append_only_store_precondition_readback_declares_plans() {
        let plans = work_graph_append_only_store_enablement_precondition_readback_plans();
        let plan_summary = plans
            .iter()
            .map(|plan| {
                (
                    plan.precondition_id,
                    plan.expected_contract_ref_ids.len(),
                    plan.affected_source_surface_ids.len(),
                    plan.required_evidence_fields.len(),
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(
            plan_summary,
            [
                ("durable_store_enablement_switch", 9, 12, 5),
                ("wal_append_boundary_contract", 6, 12, 5),
                ("idempotency_mutation_policy", 17, 12, 5),
                ("rollback_readback_gate", 10, 12, 5),
                ("operator_review_and_side_effect_lock", 3, 6, 5),
                ("scheduler_admission_enforcement_precondition", 5, 5, 6),
                ("role_manifest_enforcement_precondition", 4, 4, 5),
            ]
        );
        assert!(
            plans
                .iter()
                .all(|plan| plan.required_before_precondition_application
                    && !plan.performs_readback
                    && !plan.mutates_store
                    && !plan.enables_append_only_store)
        );
    }

    #[test]
    fn append_only_store_precondition_readback_maps_blockers() {
        let report =
            hepta_work_graph_append_only_store_enablement_precondition_readback_preview_report();
        let blocker_counts = report
            .blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(
            blocker_counts,
            [
                ("readback_execution_disabled", 12),
                ("durable_store_enablement_disabled", 12),
                ("wal_write_boundary_not_enabled", 12),
                ("idempotency_index_mutation_disabled", 12),
                ("rollback_readback_not_executed", 12),
                ("operator_review_required", 6),
                ("scheduler_admission_not_enforced", 5),
                ("role_manifest_not_enforced", 4),
                ("runtime_application_residuals_not_promoted", 7),
            ]
        );
        assert_eq!(report.blocker_mapping_assertions.len(), 8);
        assert!(report.blocker_mapping_assertions.iter().all(|assertion| {
            assertion.required_before_append_only_store_enablement
                && !assertion.performs_readback
                && !assertion.mutates_store
        }));
    }

    #[test]
    fn append_only_store_precondition_readback_declares_drift_detectors() {
        let detectors =
            work_graph_append_only_store_enablement_precondition_readback_drift_detectors();
        let detector_ids = detectors
            .iter()
            .map(|detector| detector.id)
            .collect::<Vec<_>>();

        assert_eq!(
            detector_ids,
            [
                "append_only_precondition_source_coverage_drift",
                "append_only_precondition_contract_ref_drift",
                "append_only_precondition_blocker_mapping_drift",
                "append_only_precondition_side_effect_lock_drift",
                "append_only_precondition_decision_distribution_drift",
                "append_only_precondition_prior_gate_drift",
            ]
        );
        assert!(detectors.iter().all(
            |detector| detector.blocks_precondition_application && !detector.performs_readback
        ));
    }

    #[test]
    fn append_only_store_precondition_readback_keeps_side_effects_disabled() {
        let report =
            hepta_work_graph_append_only_store_enablement_precondition_readback_preview_report();

        assert_eq!(report.required_prior_gate_count, 29);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_READBACK_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_precondition_application_preview);
        assert!(!report.ready_for_append_only_store_enablement);
        assert!(!report.ready_for_projection_enforcement);
        assert!(!report.ready_for_scheduler_admission_enforcement);
        assert!(!report.ready_for_role_manifest_enforcement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphAppendOnlyStoreEnablementPreconditionReadbackPreviewSideEffects::none()
        );
    }
}
