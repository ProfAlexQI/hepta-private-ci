use serde::Serialize;

use crate::work_graph_append_only_store_runtime_enablement_preview::WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_PREVIEW_GATE;
use crate::work_graph_append_only_store_runtime_enablement_preview::WorkGraphAppendOnlyStoreRuntimeEnablementBlockerPreview;
use crate::work_graph_append_only_store_runtime_enablement_preview::WorkGraphAppendOnlyStoreRuntimeEnablementGuardPreview;
use crate::work_graph_append_only_store_runtime_enablement_preview::WorkGraphAppendOnlyStoreRuntimeEnablementSourcePlanPreview;
use crate::work_graph_append_only_store_runtime_enablement_preview::WorkGraphAppendOnlyStoreRuntimeEnablementStagePlanPreview;
use crate::work_graph_append_only_store_runtime_enablement_preview::work_graph_append_only_store_runtime_enablement_blockers;
use crate::work_graph_append_only_store_runtime_enablement_preview::work_graph_append_only_store_runtime_enablement_guards;
use crate::work_graph_append_only_store_runtime_enablement_preview::work_graph_append_only_store_runtime_enablement_required_prior_gates;
use crate::work_graph_append_only_store_runtime_enablement_preview::work_graph_append_only_store_runtime_enablement_source_plans;
use crate::work_graph_append_only_store_runtime_enablement_preview::work_graph_append_only_store_runtime_enablement_stage_plans;

pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_enablement_readback_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_READBACK_SCHEMA_VERSION: &str =
    "work_graph_append_only_store_runtime_enablement_readback_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_enablement_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeEnablementReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub runtime_enablement_plan_count: usize,
    pub readback_plan_count: usize,
    pub source_plan_assertion_count: usize,
    pub stage_plan_assertion_count: usize,
    pub evidence_field_assertion_count: usize,
    pub guard_assertion_count: usize,
    pub blocker_mapping_assertion_count: usize,
    pub readback_evidence_field_ref_count: usize,
    pub stage_contract_ref_count: usize,
    pub stage_source_ref_count: usize,
    pub blocker_mapping_source_ref_count: usize,
    pub drift_detector_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_plans: Vec<WorkGraphAppendOnlyStoreRuntimeEnablementReadbackPlanPreview>,
    pub source_plan_assertions:
        Vec<WorkGraphAppendOnlyStoreRuntimeSourcePlanReadbackAssertionPreview>,
    pub stage_plan_assertions:
        Vec<WorkGraphAppendOnlyStoreRuntimeStagePlanReadbackAssertionPreview>,
    pub evidence_field_assertions:
        Vec<WorkGraphAppendOnlyStoreRuntimeEvidenceFieldReadbackAssertionPreview>,
    pub guard_assertions: Vec<WorkGraphAppendOnlyStoreRuntimeGuardReadbackAssertionPreview>,
    pub blocker_mapping_assertions:
        Vec<WorkGraphAppendOnlyStoreRuntimeBlockerMappingReadbackAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphAppendOnlyStoreRuntimeReadbackDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphAppendOnlyStoreRuntimeReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_runtime_enablement_application_preview: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyStoreRuntimeEnablementReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeEnablementReadbackPlanPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub runtime_enablement_plan_id: String,
    pub expected_runtime_stage_ids: Vec<&'static str>,
    pub expected_evidence_field_ids: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub readback_scope: &'static str,
    pub expected_preview_state: &'static str,
    pub required_before_runtime_enablement_application: bool,
    pub performs_readback: bool,
    pub mutates_store: bool,
    pub enables_append_only_store: bool,
    pub writes_wal: bool,
    pub mutates_idempotency_index: bool,
    pub executes_rollback: bool,
    pub records_approval: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeSourcePlanReadbackAssertionPreview {
    pub assertion_id: String,
    pub source_surface_id: &'static str,
    pub runtime_enablement_plan_id: String,
    pub expected_runtime_stage_ids: Vec<&'static str>,
    pub expected_runtime_stage_count: usize,
    pub expected_plan_state: &'static str,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeStagePlanReadbackAssertionPreview {
    pub assertion_id: String,
    pub runtime_stage_id: &'static str,
    pub category: &'static str,
    pub expected_source_surface_ids: Vec<&'static str>,
    pub expected_source_surface_count: usize,
    pub expected_contract_ref_ids: Vec<&'static str>,
    pub expected_contract_ref_count: usize,
    pub expected_runtime_state: &'static str,
    pub performs_readback: bool,
    pub mutates_store: bool,
    pub enables_append_only_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeEvidenceFieldReadbackAssertionPreview {
    pub assertion_id: String,
    pub source_surface_id: &'static str,
    pub runtime_enablement_plan_id: String,
    pub expected_evidence_field_ids: Vec<&'static str>,
    pub expected_evidence_field_count: usize,
    pub expected_evidence_state: &'static str,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeGuardReadbackAssertionPreview {
    pub assertion_id: String,
    pub guard_id: &'static str,
    pub guard_scope: &'static str,
    pub expected_guard_state: &'static str,
    pub required_before_runtime_enablement: bool,
    pub satisfied_by_preview: bool,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeBlockerMappingReadbackAssertionPreview {
    pub assertion_id: String,
    pub blocker_id: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_runtime_stage_ids: Vec<&'static str>,
    pub affected_runtime_enablement_plan_ids: Vec<String>,
    pub expected_blocker_state: &'static str,
    pub required_before_runtime_enablement: bool,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeReadbackDriftDetectorPreview {
    pub id: &'static str,
    pub compared_field_ids: Vec<&'static str>,
    pub severity: &'static str,
    pub blocks_runtime_enablement_application: bool,
    pub performs_readback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_runtime_enablement_plan_ids: Vec<String>,
    pub required_before_runtime_enablement_application: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeEnablementReadbackPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub idempotency_index_mutated: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub task_result_enforcement_enabled: bool,
    pub task_result_persisted: bool,
    pub readback_executed: bool,
    pub rollback_executed: bool,
    pub runtime_application_promoted: bool,
    pub approval_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_store_runtime_enablement_readback_preview_report()
-> WorkGraphAppendOnlyStoreRuntimeEnablementReadbackPreviewReport {
    let plans = work_graph_append_only_store_runtime_enablement_source_plans();
    let readback_plans = work_graph_append_only_store_runtime_enablement_readback_plans();
    let source_plan_assertions =
        work_graph_append_only_store_runtime_source_plan_readback_assertions();
    let stage_plan_assertions =
        work_graph_append_only_store_runtime_stage_plan_readback_assertions();
    let evidence_field_assertions =
        work_graph_append_only_store_runtime_evidence_field_readback_assertions();
    let guard_assertions = work_graph_append_only_store_runtime_guard_readback_assertions();
    let blocker_mapping_assertions =
        work_graph_append_only_store_runtime_blocker_mapping_readback_assertions();
    let drift_detectors = work_graph_append_only_store_runtime_readback_drift_detectors();
    let blockers = work_graph_append_only_store_runtime_readback_blockers();
    let required_prior_gates =
        work_graph_append_only_store_runtime_enablement_readback_required_prior_gates();

    WorkGraphAppendOnlyStoreRuntimeEnablementReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_READBACK_PREVIEW_GATE,
        schema_version: WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_append_only_store_runtime_enablement_readback_preview_no_execution",
        runtime_enablement_plan_count: plans.len(),
        readback_plan_count: readback_plans.len(),
        source_plan_assertion_count: source_plan_assertions.len(),
        stage_plan_assertion_count: stage_plan_assertions.len(),
        evidence_field_assertion_count: evidence_field_assertions.len(),
        guard_assertion_count: guard_assertions.len(),
        blocker_mapping_assertion_count: blocker_mapping_assertions.len(),
        readback_evidence_field_ref_count: readback_plans
            .iter()
            .map(|plan| plan.expected_evidence_field_ids.len())
            .sum(),
        stage_contract_ref_count: stage_plan_assertions
            .iter()
            .map(|assertion| assertion.expected_contract_ref_count)
            .sum(),
        stage_source_ref_count: stage_plan_assertions
            .iter()
            .map(|assertion| assertion.expected_source_surface_count)
            .sum(),
        blocker_mapping_source_ref_count: blocker_mapping_assertions
            .iter()
            .map(|assertion| assertion.affected_source_surface_ids.len())
            .sum(),
        drift_detector_count: drift_detectors.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_plans,
        source_plan_assertions,
        stage_plan_assertions,
        evidence_field_assertions,
        guard_assertions,
        blocker_mapping_assertions,
        drift_detectors,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_runtime_enablement_application_preview: true,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphAppendOnlyStoreRuntimeEnablementReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_store_runtime_enablement_readback_plans()
-> Vec<WorkGraphAppendOnlyStoreRuntimeEnablementReadbackPlanPreview> {
    work_graph_append_only_store_runtime_enablement_source_plans()
        .into_iter()
        .map(readback_plan)
        .collect()
}

pub fn work_graph_append_only_store_runtime_source_plan_readback_assertions()
-> Vec<WorkGraphAppendOnlyStoreRuntimeSourcePlanReadbackAssertionPreview> {
    work_graph_append_only_store_runtime_enablement_source_plans()
        .into_iter()
        .map(
            |plan| WorkGraphAppendOnlyStoreRuntimeSourcePlanReadbackAssertionPreview {
                assertion_id: assertion_id_for(&plan.runtime_enablement_plan_id, "source_plan"),
                source_surface_id: plan.source_surface_id,
                runtime_enablement_plan_id: plan.runtime_enablement_plan_id,
                expected_runtime_stage_count: plan.required_runtime_stage_ids.len(),
                expected_runtime_stage_ids: plan.required_runtime_stage_ids,
                expected_plan_state: "runtime_enablement_plan_defined_runtime_disabled",
                performs_readback: false,
                mutates_store: false,
            },
        )
        .collect()
}

pub fn work_graph_append_only_store_runtime_stage_plan_readback_assertions()
-> Vec<WorkGraphAppendOnlyStoreRuntimeStagePlanReadbackAssertionPreview> {
    work_graph_append_only_store_runtime_enablement_stage_plans()
        .into_iter()
        .map(stage_assertion)
        .collect()
}

pub fn work_graph_append_only_store_runtime_evidence_field_readback_assertions()
-> Vec<WorkGraphAppendOnlyStoreRuntimeEvidenceFieldReadbackAssertionPreview> {
    work_graph_append_only_store_runtime_enablement_source_plans()
        .into_iter()
        .map(
            |plan| WorkGraphAppendOnlyStoreRuntimeEvidenceFieldReadbackAssertionPreview {
                assertion_id: assertion_id_for(&plan.runtime_enablement_plan_id, "evidence_fields"),
                source_surface_id: plan.source_surface_id,
                runtime_enablement_plan_id: plan.runtime_enablement_plan_id,
                expected_evidence_field_count: plan.expected_evidence_field_ids.len(),
                expected_evidence_field_ids: plan.expected_evidence_field_ids,
                expected_evidence_state: "evidence_fields_declared_readback_not_executed",
                performs_readback: false,
                mutates_store: false,
            },
        )
        .collect()
}

pub fn work_graph_append_only_store_runtime_guard_readback_assertions()
-> Vec<WorkGraphAppendOnlyStoreRuntimeGuardReadbackAssertionPreview> {
    work_graph_append_only_store_runtime_enablement_guards()
        .into_iter()
        .map(guard_assertion)
        .collect()
}

pub fn work_graph_append_only_store_runtime_blocker_mapping_readback_assertions()
-> Vec<WorkGraphAppendOnlyStoreRuntimeBlockerMappingReadbackAssertionPreview> {
    work_graph_append_only_store_runtime_enablement_blockers()
        .into_iter()
        .map(blocker_mapping_assertion)
        .collect()
}

pub fn work_graph_append_only_store_runtime_readback_drift_detectors()
-> Vec<WorkGraphAppendOnlyStoreRuntimeReadbackDriftDetectorPreview> {
    vec![
        drift_detector(
            "append_only_runtime_source_plan_drift",
            vec![
                "source_surface_id",
                "runtime_enablement_plan_id",
                "residual_source_blocker_ids",
            ],
            "critical",
        ),
        drift_detector(
            "append_only_runtime_stage_contract_drift",
            vec![
                "runtime_stage_id",
                "required_contract_ref_ids",
                "affected_source_surface_ids",
            ],
            "critical",
        ),
        drift_detector(
            "append_only_runtime_evidence_field_drift",
            vec![
                "expected_evidence_field_ids",
                "runtime_store_switch_contract_ref",
                "no_mutation_guard_ref",
            ],
            "high",
        ),
        drift_detector(
            "append_only_runtime_blocker_mapping_drift",
            vec![
                "blocker_id",
                "affected_runtime_stage_ids",
                "affected_runtime_enablement_plan_ids",
            ],
            "critical",
        ),
        drift_detector(
            "append_only_runtime_side_effect_boundary_drift",
            vec![
                "side_effects",
                "append_only_store_enabled",
                "wal_written",
                "runtime_application_promoted",
            ],
            "critical",
        ),
        drift_detector(
            "append_only_runtime_prior_gate_drift",
            vec![
                "required_prior_gates",
                "runtime_enablement_preview_gate",
                "role_manifest_readiness_rerun_gate",
            ],
            "medium",
        ),
    ]
}

pub fn work_graph_append_only_store_runtime_readback_blockers()
-> Vec<WorkGraphAppendOnlyStoreRuntimeReadbackBlockerPreview> {
    let all_sources = work_graph_append_only_store_runtime_enablement_source_plans()
        .into_iter()
        .map(|plan| plan.source_surface_id)
        .collect::<Vec<_>>();
    let mut blockers = vec![readback_blocker(
        "readback_execution_disabled",
        "critical",
        "readback_execution",
        all_sources,
        "keep this gate preview-only until runtime enablement readback execution is explicitly promoted",
    )];
    blockers.extend(
        work_graph_append_only_store_runtime_enablement_blockers()
            .into_iter()
            .map(readback_blocker_from_runtime_blocker),
    );
    blockers
}

pub fn work_graph_append_only_store_runtime_enablement_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = work_graph_append_only_store_runtime_enablement_required_prior_gates();
    if !gates.contains(&WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_PREVIEW_GATE) {
        gates.push(WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_PREVIEW_GATE);
    }
    gates
}

impl WorkGraphAppendOnlyStoreRuntimeEnablementReadbackPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            idempotency_index_mutated: false,
            append_only_store_enabled: false,
            projection_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            role_manifest_enforcement_enabled: false,
            task_result_enforcement_enabled: false,
            task_result_persisted: false,
            readback_executed: false,
            rollback_executed: false,
            runtime_application_promoted: false,
            approval_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn readback_plan(
    plan: WorkGraphAppendOnlyStoreRuntimeEnablementSourcePlanPreview,
) -> WorkGraphAppendOnlyStoreRuntimeEnablementReadbackPlanPreview {
    WorkGraphAppendOnlyStoreRuntimeEnablementReadbackPlanPreview {
        source_surface_id: plan.source_surface_id,
        source_category: plan.source_category,
        runtime_enablement_plan_id: plan.runtime_enablement_plan_id,
        expected_runtime_stage_ids: plan.required_runtime_stage_ids,
        expected_evidence_field_ids: plan.expected_evidence_field_ids,
        residual_source_blocker_ids: plan.residual_source_blocker_ids,
        readback_scope: "append_only_store_runtime_enablement_contract_refs",
        expected_preview_state: "runtime_enablement_contract_ready_readback_not_executed",
        required_before_runtime_enablement_application: true,
        performs_readback: false,
        mutates_store: false,
        enables_append_only_store: false,
        writes_wal: false,
        mutates_idempotency_index: false,
        executes_rollback: false,
        records_approval: false,
    }
}

fn stage_assertion(
    stage: WorkGraphAppendOnlyStoreRuntimeEnablementStagePlanPreview,
) -> WorkGraphAppendOnlyStoreRuntimeStagePlanReadbackAssertionPreview {
    WorkGraphAppendOnlyStoreRuntimeStagePlanReadbackAssertionPreview {
        assertion_id: assertion_id_for(stage.id, "stage_plan"),
        runtime_stage_id: stage.id,
        category: stage.category,
        expected_source_surface_count: stage.affected_source_surface_ids.len(),
        expected_source_surface_ids: stage.affected_source_surface_ids,
        expected_contract_ref_count: stage.required_contract_ref_ids.len(),
        expected_contract_ref_ids: stage.required_contract_ref_ids,
        expected_runtime_state: "contract_ready_preview_runtime_disabled_readback_not_executed",
        performs_readback: false,
        mutates_store: false,
        enables_append_only_store: false,
    }
}

fn guard_assertion(
    guard: WorkGraphAppendOnlyStoreRuntimeEnablementGuardPreview,
) -> WorkGraphAppendOnlyStoreRuntimeGuardReadbackAssertionPreview {
    WorkGraphAppendOnlyStoreRuntimeGuardReadbackAssertionPreview {
        assertion_id: assertion_id_for(guard.id, "guard"),
        guard_id: guard.id,
        guard_scope: guard.guard_scope,
        expected_guard_state: "guard_required_not_satisfied_by_preview",
        required_before_runtime_enablement: guard.required_before_runtime_enablement,
        satisfied_by_preview: guard.satisfied_by_preview,
        performs_readback: false,
        mutates_store: false,
    }
}

fn blocker_mapping_assertion(
    blocker: WorkGraphAppendOnlyStoreRuntimeEnablementBlockerPreview,
) -> WorkGraphAppendOnlyStoreRuntimeBlockerMappingReadbackAssertionPreview {
    WorkGraphAppendOnlyStoreRuntimeBlockerMappingReadbackAssertionPreview {
        assertion_id: assertion_id_for(blocker.id, "blocker_mapping"),
        blocker_id: blocker.id,
        category: blocker.category,
        affected_source_surface_ids: blocker.affected_source_surface_ids,
        affected_runtime_stage_ids: blocker.affected_runtime_stage_ids,
        affected_runtime_enablement_plan_ids: blocker.affected_runtime_enablement_plan_ids,
        expected_blocker_state: "blocks_runtime_enablement_until_readback_and_application_preview",
        required_before_runtime_enablement: blocker.required_before_runtime_enablement,
        performs_readback: false,
        mutates_store: false,
    }
}

fn drift_detector(
    id: &'static str,
    compared_field_ids: Vec<&'static str>,
    severity: &'static str,
) -> WorkGraphAppendOnlyStoreRuntimeReadbackDriftDetectorPreview {
    WorkGraphAppendOnlyStoreRuntimeReadbackDriftDetectorPreview {
        id,
        compared_field_ids,
        severity,
        blocks_runtime_enablement_application: true,
        performs_readback: false,
    }
}

fn readback_blocker_from_runtime_blocker(
    blocker: WorkGraphAppendOnlyStoreRuntimeEnablementBlockerPreview,
) -> WorkGraphAppendOnlyStoreRuntimeReadbackBlockerPreview {
    WorkGraphAppendOnlyStoreRuntimeReadbackBlockerPreview {
        id: blocker.id,
        severity: blocker.severity,
        category: blocker.category,
        affected_source_surface_ids: blocker.affected_source_surface_ids,
        affected_runtime_enablement_plan_ids: blocker.affected_runtime_enablement_plan_ids,
        required_before_runtime_enablement_application: true,
        recommended_fix: blocker.recommended_fix,
    }
}

fn readback_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphAppendOnlyStoreRuntimeReadbackBlockerPreview {
    WorkGraphAppendOnlyStoreRuntimeReadbackBlockerPreview {
        id,
        severity,
        category,
        affected_runtime_enablement_plan_ids: affected_source_surface_ids
            .iter()
            .map(|source| format!("append_only_store_runtime_enablement_{source}_preview"))
            .collect(),
        affected_source_surface_ids,
        required_before_runtime_enablement_application: true,
        recommended_fix,
    }
}

fn assertion_id_for(base_id: &str, suffix: &str) -> String {
    format!("{base_id}_{suffix}_readback_assertion")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_enablement_readback_derives_expected_assertion_counts() {
        let report =
            hepta_work_graph_append_only_store_runtime_enablement_readback_preview_report();

        assert_eq!(report.status, "ready");
        assert_eq!(report.runtime_enablement_plan_count, 12);
        assert_eq!(report.readback_plan_count, 12);
        assert_eq!(report.source_plan_assertion_count, 12);
        assert_eq!(report.stage_plan_assertion_count, 6);
        assert_eq!(report.evidence_field_assertion_count, 12);
        assert_eq!(report.guard_assertion_count, 10);
        assert_eq!(report.blocker_mapping_assertion_count, 13);
        assert_eq!(report.readback_evidence_field_ref_count, 96);
        assert_eq!(report.stage_contract_ref_count, 29);
        assert_eq!(report.stage_source_ref_count, 62);
        assert_eq!(report.blocker_mapping_source_ref_count, 113);
        assert_eq!(report.required_prior_gate_count, 41);
    }

    #[test]
    fn runtime_enablement_readback_keeps_execution_disabled() {
        let report =
            hepta_work_graph_append_only_store_runtime_enablement_readback_preview_report();

        assert!(report.readback_plans.iter().all(|plan| {
            plan.expected_runtime_stage_ids.len() == 6
                && plan.expected_evidence_field_ids.len() == 8
                && plan.required_before_runtime_enablement_application
                && !plan.performs_readback
                && !plan.mutates_store
                && !plan.enables_append_only_store
                && !plan.writes_wal
                && !plan.mutates_idempotency_index
                && !plan.executes_rollback
                && !plan.records_approval
        }));
        assert!(
            report
                .stage_plan_assertions
                .iter()
                .all(|assertion| !assertion.performs_readback
                    && !assertion.mutates_store
                    && !assertion.enables_append_only_store)
        );
        assert!(
            report
                .guard_assertions
                .iter()
                .all(|assertion| !assertion.satisfied_by_preview && !assertion.performs_readback)
        );
    }

    #[test]
    fn runtime_enablement_readback_preserves_stage_and_blocker_mapping() {
        let report =
            hepta_work_graph_append_only_store_runtime_enablement_readback_preview_report();
        let stage_counts = report
            .stage_plan_assertions
            .iter()
            .map(|assertion| {
                (
                    assertion.runtime_stage_id,
                    assertion.expected_source_surface_count,
                    assertion.expected_contract_ref_count,
                )
            })
            .collect::<Vec<_>>();
        let blocker_counts = report
            .blocker_mapping_assertions
            .iter()
            .map(|assertion| {
                (
                    assertion.blocker_id,
                    assertion.affected_source_surface_ids.len(),
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(
            stage_counts,
            [
                ("durable_store_runtime_switch", 12, 5),
                ("wal_write_boundary", 12, 6),
                ("idempotency_mutation_policy", 12, 5),
                ("rollback_readback_execution_gate", 12, 5),
                ("operator_review_side_effect_lock", 7, 3),
                ("runtime_application_promotion", 7, 5),
            ]
        );
        assert_eq!(
            blocker_counts,
            [
                ("durable_store_runtime_switch_disabled", 12),
                ("append_only_store_runtime_enablement_disabled", 12),
                ("wal_write_boundary_not_enabled", 12),
                ("idempotency_index_mutation_disabled", 12),
                ("rollback_readback_not_executed", 12),
                ("operator_review_required", 7),
                ("projection_adapter_runtime_closure_application_disabled", 7),
                ("store_guard_runtime_application_disabled", 5),
                ("terminal_task_result_runtime_application_disabled", 6),
                ("scheduler_admission_runtime_application_disabled", 5),
                ("role_manifest_runtime_application_disabled", 4),
                ("runtime_application_residuals_not_promoted", 7),
                ("append_only_store_runtime_enablement_readback_missing", 12),
            ]
        );
    }

    #[test]
    fn runtime_enablement_readback_blockers_and_next_gate_are_stable() {
        let report =
            hepta_work_graph_append_only_store_runtime_enablement_readback_preview_report();
        let blocker_counts = report
            .blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(report.blocker_count, 14);
        assert_eq!(report.drift_detector_count, 6);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_READBACK_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_PREVIEW_GATE)
        );
        assert_eq!(blocker_counts[0], ("readback_execution_disabled", 12));
        assert!(
            report
                .blockers
                .iter()
                .all(|blocker| blocker.required_before_runtime_enablement_application)
        );
        assert!(report.ready_for_runtime_enablement_application_preview);
        assert!(!report.ready_for_append_only_store_enablement);
    }

    #[test]
    fn runtime_enablement_readback_preserves_no_side_effect_boundary() {
        let report =
            hepta_work_graph_append_only_store_runtime_enablement_readback_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAppendOnlyStoreRuntimeEnablementReadbackPreviewSideEffects::none()
        );
    }
}
