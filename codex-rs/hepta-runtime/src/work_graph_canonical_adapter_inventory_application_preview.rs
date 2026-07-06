use serde::Serialize;

use crate::work_graph_canonical_adapter_inventory_readback_preview::WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_READBACK_PREVIEW_GATE;
use crate::work_graph_canonical_adapter_inventory_readback_preview::WorkGraphCanonicalAdapterInventoryBlockerMappingAssertionPreview;
use crate::work_graph_canonical_adapter_inventory_readback_preview::WorkGraphCanonicalAdapterInventoryCollectionBindingAssertionPreview;
use crate::work_graph_canonical_adapter_inventory_readback_preview::WorkGraphCanonicalAdapterInventoryEdgeKindAssertionPreview;
use crate::work_graph_canonical_adapter_inventory_readback_preview::WorkGraphCanonicalAdapterInventoryIdentityAssertionPreview;
use crate::work_graph_canonical_adapter_inventory_readback_preview::WorkGraphCanonicalAdapterInventoryReadbackPlanPreview;
use crate::work_graph_canonical_adapter_inventory_readback_preview::WorkGraphCanonicalAdapterInventoryTimelineAssertionPreview;
use crate::work_graph_canonical_adapter_inventory_readback_preview::hepta_work_graph_canonical_adapter_inventory_readback_preview_report;
use crate::work_graph_canonical_adapter_inventory_readback_preview::work_graph_canonical_adapter_inventory_readback_required_prior_gates;

pub const WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_APPLICATION_PREVIEW_GATE: &str =
    "hepta_work_graph_canonical_adapter_inventory_application_preview_gate";
pub const WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_APPLICATION_SCHEMA_VERSION: &str =
    "work_graph_canonical_adapter_inventory_application_preview_v1";
pub const WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_APPLICATION_RECOMMENDED_NEXT_GATE: &str = "hepta_work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryApplicationPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_plan_count: usize,
    pub application_plan_count: usize,
    pub source_outcome_count: usize,
    pub canonical_adapter_inventory_contract_ready_preview_count: usize,
    pub identity_application_count: usize,
    pub edge_kind_application_count: usize,
    pub collection_binding_application_count: usize,
    pub timeline_event_application_count: usize,
    pub blocker_application_count: usize,
    pub application_guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub application_plans: Vec<WorkGraphCanonicalAdapterInventoryApplicationPlanPreview>,
    pub source_outcomes: Vec<WorkGraphCanonicalAdapterInventoryApplicationSourceOutcomePreview>,
    pub identity_applications: Vec<WorkGraphCanonicalAdapterInventoryIdentityApplicationPreview>,
    pub edge_kind_applications: Vec<WorkGraphCanonicalAdapterInventoryEdgeKindApplicationPreview>,
    pub collection_binding_applications:
        Vec<WorkGraphCanonicalAdapterInventoryCollectionBindingApplicationPreview>,
    pub timeline_event_applications:
        Vec<WorkGraphCanonicalAdapterInventoryTimelineEventApplicationPreview>,
    pub blocker_applications: Vec<WorkGraphCanonicalAdapterInventoryBlockerApplicationPreview>,
    pub application_guards: Vec<WorkGraphCanonicalAdapterInventoryApplicationGuardPreview>,
    pub blockers: Vec<WorkGraphCanonicalAdapterInventoryApplicationBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_canonical_adapter_inventory_readiness_rerun_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphCanonicalAdapterInventoryApplicationPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryApplicationPlanPreview {
    pub application_plan_id: String,
    pub readback_source_surface_id: &'static str,
    pub source_category: &'static str,
    pub canonical_inventory_state: &'static str,
    pub application_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub canonical_adapter_inventory_contract_ready_preview: bool,
    pub applies_to_runtime: bool,
    pub persists_work_graph_events: bool,
    pub enforces_adapter_projection: bool,
    pub mutates_scheduler_admission: bool,
    pub mutates_task_result_enforcement: bool,
    pub mutates_role_manifest_enforcement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryApplicationSourceOutcomePreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub application_plan_id: String,
    pub post_application_canonical_inventory_state: &'static str,
    pub canonical_adapter_inventory_contract_ready_preview: bool,
    pub ready_for_canonical_adapter_inventory_readiness_rerun_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub applies_to_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryIdentityApplicationPreview {
    pub application_id: String,
    pub source_surface_id: &'static str,
    pub canonical_node_kind: &'static str,
    pub required_identity_fields: Vec<&'static str>,
    pub deterministic_identity_ready_preview: bool,
    pub persists_identity: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryEdgeKindApplicationPreview {
    pub application_id: String,
    pub source_surface_id: &'static str,
    pub canonical_edge_kinds: Vec<&'static str>,
    pub edge_namespace: &'static str,
    pub persists_edges: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryCollectionBindingApplicationPreview {
    pub application_id: String,
    pub source_surface_id: &'static str,
    pub canonical_collection_ids: Vec<&'static str>,
    pub persists_store_projection: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryTimelineEventApplicationPreview {
    pub application_id: String,
    pub source_surface_id: &'static str,
    pub timeline_event_type_ids: Vec<&'static str>,
    pub persists_timeline: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryBlockerApplicationPreview {
    pub application_id: String,
    pub blocker_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub expected_blocker_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub clears_application_missing_blocker: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryApplicationGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub required_before_runtime_enforcement: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryApplicationBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_application_plan_ids: Vec<String>,
    pub required_before_runtime_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryApplicationPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_events_persisted: bool,
    pub adapter_projection_enforced: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub approval_recorded: bool,
    pub side_effect_lock_established: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_canonical_adapter_inventory_application_preview_report()
-> WorkGraphCanonicalAdapterInventoryApplicationPreviewReport {
    let readback_report = hepta_work_graph_canonical_adapter_inventory_readback_preview_report();
    let application_plans = work_graph_canonical_adapter_inventory_application_plans_from(
        &readback_report.readback_plans,
    );
    let source_outcomes =
        work_graph_canonical_adapter_inventory_application_source_outcomes_from(&application_plans);
    let blockers =
        work_graph_canonical_adapter_inventory_application_blockers_from(&application_plans);
    let required_prior_gates =
        work_graph_canonical_adapter_inventory_application_required_prior_gates();

    WorkGraphCanonicalAdapterInventoryApplicationPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_APPLICATION_PREVIEW_GATE,
        schema_version: WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_APPLICATION_SCHEMA_VERSION,
        preview_mode: "read_only_canonical_adapter_inventory_application_preview_no_mutation",
        readback_plan_count: readback_report.readback_plan_count,
        application_plan_count: application_plans.len(),
        source_outcome_count: source_outcomes.len(),
        canonical_adapter_inventory_contract_ready_preview_count: source_outcomes
            .iter()
            .filter(|outcome| outcome.canonical_adapter_inventory_contract_ready_preview)
            .count(),
        identity_application_count: readback_report.identity_assertions.len(),
        edge_kind_application_count: readback_report.edge_kind_assertions.len(),
        collection_binding_application_count: readback_report.collection_binding_assertions.len(),
        timeline_event_application_count: readback_report.timeline_event_assertions.len(),
        blocker_application_count: readback_report.blocker_mapping_assertions.len(),
        application_guard_count: work_graph_canonical_adapter_inventory_application_guards().len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        identity_applications: work_graph_canonical_adapter_inventory_identity_applications_from(
            &readback_report.identity_assertions,
        ),
        edge_kind_applications: work_graph_canonical_adapter_inventory_edge_kind_applications_from(
            &readback_report.edge_kind_assertions,
        ),
        collection_binding_applications:
            work_graph_canonical_adapter_inventory_collection_binding_applications_from(
                &readback_report.collection_binding_assertions,
            ),
        timeline_event_applications:
            work_graph_canonical_adapter_inventory_timeline_event_applications_from(
                &readback_report.timeline_event_assertions,
            ),
        blocker_applications: work_graph_canonical_adapter_inventory_blocker_applications_from(
            &readback_report.blocker_mapping_assertions,
        ),
        application_guards: work_graph_canonical_adapter_inventory_application_guards(),
        application_plans,
        source_outcomes,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_APPLICATION_RECOMMENDED_NEXT_GATE,
        ready_for_canonical_adapter_inventory_readiness_rerun_preview: true,
        ready_for_append_only_work_graph_events: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphCanonicalAdapterInventoryApplicationPreviewSideEffects::none(),
    }
}

pub fn work_graph_canonical_adapter_inventory_application_plans()
-> Vec<WorkGraphCanonicalAdapterInventoryApplicationPlanPreview> {
    let readback_report = hepta_work_graph_canonical_adapter_inventory_readback_preview_report();
    work_graph_canonical_adapter_inventory_application_plans_from(&readback_report.readback_plans)
}

pub fn work_graph_canonical_adapter_inventory_application_source_outcomes()
-> Vec<WorkGraphCanonicalAdapterInventoryApplicationSourceOutcomePreview> {
    work_graph_canonical_adapter_inventory_application_source_outcomes_from(
        &work_graph_canonical_adapter_inventory_application_plans(),
    )
}

pub fn work_graph_canonical_adapter_inventory_application_blockers()
-> Vec<WorkGraphCanonicalAdapterInventoryApplicationBlockerPreview> {
    work_graph_canonical_adapter_inventory_application_blockers_from(
        &work_graph_canonical_adapter_inventory_application_plans(),
    )
}

pub fn work_graph_canonical_adapter_inventory_application_required_prior_gates() -> Vec<&'static str>
{
    let mut gates = work_graph_canonical_adapter_inventory_readback_required_prior_gates();
    gates.push(WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_READBACK_PREVIEW_GATE);
    gates
}

impl WorkGraphCanonicalAdapterInventoryApplicationPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_events_persisted: false,
            adapter_projection_enforced: false,
            runtime_mutation_performed: false,
            scheduler_admission_enforced: false,
            task_result_enforcement_enabled: false,
            role_manifest_enforcement_enabled: false,
            approval_recorded: false,
            side_effect_lock_established: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn work_graph_canonical_adapter_inventory_application_plans_from(
    readback_plans: &[WorkGraphCanonicalAdapterInventoryReadbackPlanPreview],
) -> Vec<WorkGraphCanonicalAdapterInventoryApplicationPlanPreview> {
    readback_plans
        .iter()
        .map(|plan| WorkGraphCanonicalAdapterInventoryApplicationPlanPreview {
            application_plan_id: format!(
                "{}_canonical_adapter_inventory_application",
                plan.source_surface_id
            ),
            readback_source_surface_id: plan.source_surface_id,
            source_category: plan.source_category,
            canonical_inventory_state: plan.canonical_inventory_state,
            application_state: "canonical_adapter_inventory_contract_ready_preview_after_application",
            readback_verified_by_preview: plan.readback_status == "readback_plan_ready",
            canonical_adapter_inventory_contract_ready_preview: true,
            applies_to_runtime: false,
            persists_work_graph_events: false,
            enforces_adapter_projection: false,
            mutates_scheduler_admission: false,
            mutates_task_result_enforcement: false,
            mutates_role_manifest_enforcement: false,
        })
        .collect()
}

fn work_graph_canonical_adapter_inventory_application_source_outcomes_from(
    application_plans: &[WorkGraphCanonicalAdapterInventoryApplicationPlanPreview],
) -> Vec<WorkGraphCanonicalAdapterInventoryApplicationSourceOutcomePreview> {
    application_plans
        .iter()
        .map(
            |plan| WorkGraphCanonicalAdapterInventoryApplicationSourceOutcomePreview {
                source_surface_id: plan.readback_source_surface_id,
                source_category: plan.source_category,
                application_plan_id: plan.application_plan_id.clone(),
                post_application_canonical_inventory_state: plan.application_state,
                canonical_adapter_inventory_contract_ready_preview: plan
                    .canonical_adapter_inventory_contract_ready_preview,
                ready_for_canonical_adapter_inventory_readiness_rerun_preview: true,
                ready_for_append_only_work_graph_events: false,
                applies_to_runtime: false,
            },
        )
        .collect()
}

fn work_graph_canonical_adapter_inventory_identity_applications_from(
    assertions: &[WorkGraphCanonicalAdapterInventoryIdentityAssertionPreview],
) -> Vec<WorkGraphCanonicalAdapterInventoryIdentityApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphCanonicalAdapterInventoryIdentityApplicationPreview {
                application_id: format!("{}_identity_application", assertion.source_surface_id),
                source_surface_id: assertion.source_surface_id,
                canonical_node_kind: assertion.canonical_node_kind,
                required_identity_fields: assertion.required_identity_fields.clone(),
                deterministic_identity_ready_preview: assertion.deterministic_identity_required,
                persists_identity: false,
            },
        )
        .collect()
}

fn work_graph_canonical_adapter_inventory_edge_kind_applications_from(
    assertions: &[WorkGraphCanonicalAdapterInventoryEdgeKindAssertionPreview],
) -> Vec<WorkGraphCanonicalAdapterInventoryEdgeKindApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphCanonicalAdapterInventoryEdgeKindApplicationPreview {
                application_id: format!("{}_edge_kind_application", assertion.source_surface_id),
                source_surface_id: assertion.source_surface_id,
                canonical_edge_kinds: assertion.canonical_edge_kinds.clone(),
                edge_namespace: assertion.edge_namespace,
                persists_edges: false,
            },
        )
        .collect()
}

fn work_graph_canonical_adapter_inventory_collection_binding_applications_from(
    assertions: &[WorkGraphCanonicalAdapterInventoryCollectionBindingAssertionPreview],
) -> Vec<WorkGraphCanonicalAdapterInventoryCollectionBindingApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphCanonicalAdapterInventoryCollectionBindingApplicationPreview {
                application_id: format!(
                    "{}_collection_binding_application",
                    assertion.source_surface_id
                ),
                source_surface_id: assertion.source_surface_id,
                canonical_collection_ids: assertion.canonical_collection_ids.clone(),
                persists_store_projection: false,
            },
        )
        .collect()
}

fn work_graph_canonical_adapter_inventory_timeline_event_applications_from(
    assertions: &[WorkGraphCanonicalAdapterInventoryTimelineAssertionPreview],
) -> Vec<WorkGraphCanonicalAdapterInventoryTimelineEventApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphCanonicalAdapterInventoryTimelineEventApplicationPreview {
                application_id: format!(
                    "{}_timeline_event_application",
                    assertion.source_surface_id
                ),
                source_surface_id: assertion.source_surface_id,
                timeline_event_type_ids: assertion.timeline_event_type_ids.clone(),
                persists_timeline: false,
            },
        )
        .collect()
}

fn work_graph_canonical_adapter_inventory_blocker_applications_from(
    assertions: &[WorkGraphCanonicalAdapterInventoryBlockerMappingAssertionPreview],
) -> Vec<WorkGraphCanonicalAdapterInventoryBlockerApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphCanonicalAdapterInventoryBlockerApplicationPreview {
                application_id: format!("{}_application", assertion.blocker_id),
                blocker_id: assertion.blocker_id,
                affected_source_surface_ids: assertion.affected_source_surface_ids.clone(),
                expected_blocker_state: "mapped_for_canonical_adapter_inventory_rerun_preview",
                readback_verified_by_preview: true,
                clears_application_missing_blocker: assertion.blocker_id
                    == "canonical_adapter_inventory_readback_missing",
                mutates_runtime: false,
            },
        )
        .collect()
}

fn work_graph_canonical_adapter_inventory_application_guards()
-> Vec<WorkGraphCanonicalAdapterInventoryApplicationGuardPreview> {
    vec![
        application_guard("no_work_graph_event_persistence"),
        application_guard("no_adapter_projection_enforcement"),
        application_guard("no_scheduler_admission_enforcement"),
        application_guard("no_task_result_enforcement"),
        application_guard("no_role_manifest_enforcement"),
        application_guard("no_agent_spawn"),
        application_guard("no_external_send"),
        application_guard("no_model_invocation"),
    ]
}

fn work_graph_canonical_adapter_inventory_application_blockers_from(
    application_plans: &[WorkGraphCanonicalAdapterInventoryApplicationPlanPreview],
) -> Vec<WorkGraphCanonicalAdapterInventoryApplicationBlockerPreview> {
    let all_source_surface_ids = application_plans
        .iter()
        .map(|plan| plan.readback_source_surface_id)
        .collect::<Vec<_>>();
    let partial_or_gap_source_ids = application_plans
        .iter()
        .filter(|plan| plan.canonical_inventory_state != "canonical_contract_ready_preview")
        .map(|plan| plan.readback_source_surface_id)
        .collect::<Vec<_>>();
    let application_plan_ids = application_plans
        .iter()
        .map(|plan| plan.application_plan_id.clone())
        .collect::<Vec<_>>();
    vec![
        application_blocker(
            "append_only_work_graph_events_disabled",
            "high",
            "append_only_fact_source",
            all_source_surface_ids.clone(),
            application_plan_ids.clone(),
            "shadow-write canonical WorkGraph events with replay/readback before enforcement",
        ),
        application_blocker(
            "runtime_canonical_adapter_enforcement_disabled",
            "high",
            "runtime_adapter_enforcement",
            all_source_surface_ids.clone(),
            application_plan_ids.clone(),
            "keep canonical adapters preview-only until append-only events and operator-review boundaries are promoted",
        ),
        application_blocker(
            "canonical_adapter_projection_partial_or_gap",
            "high",
            "projection_coverage",
            partial_or_gap_source_ids,
            application_plan_ids.clone(),
            "close partial/gap source adapters before making the canonical projection authoritative",
        ),
        application_blocker(
            "canonical_adapter_inventory_readiness_rerun_missing",
            "medium",
            "readiness_rerun",
            all_source_surface_ids,
            application_plan_ids,
            "rerun enforcement readiness after no-mutation application outcomes are available",
        ),
    ]
}

fn application_guard(
    id: &'static str,
) -> WorkGraphCanonicalAdapterInventoryApplicationGuardPreview {
    WorkGraphCanonicalAdapterInventoryApplicationGuardPreview {
        id,
        severity: "high",
        required_before_runtime_enforcement: true,
        satisfied_by_preview: true,
    }
}

fn application_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_application_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphCanonicalAdapterInventoryApplicationBlockerPreview {
    WorkGraphCanonicalAdapterInventoryApplicationBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_application_plan_ids,
        required_before_runtime_enforcement: true,
        recommended_fix,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_adapter_inventory_application_covers_readback_rows() {
        let report = hepta_work_graph_canonical_adapter_inventory_application_preview_report();

        assert_eq!(report.readback_plan_count, 12);
        assert_eq!(report.application_plan_count, 12);
        assert_eq!(report.source_outcome_count, 12);
        assert_eq!(
            report.canonical_adapter_inventory_contract_ready_preview_count,
            12
        );
        assert!(
            report
                .application_plans
                .iter()
                .all(|plan| plan.readback_verified_by_preview)
        );
    }

    #[test]
    fn canonical_adapter_inventory_application_preserves_no_mutation_boundaries() {
        let report = hepta_work_graph_canonical_adapter_inventory_application_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphCanonicalAdapterInventoryApplicationPreviewSideEffects::none()
        );
        assert!(
            report
                .application_plans
                .iter()
                .all(|plan| !plan.applies_to_runtime && !plan.persists_work_graph_events)
        );
        assert!(
            report
                .source_outcomes
                .iter()
                .all(|outcome| !outcome.ready_for_append_only_work_graph_events)
        );
    }

    #[test]
    fn canonical_adapter_inventory_application_keeps_next_blockers_explicit() {
        let report = hepta_work_graph_canonical_adapter_inventory_application_preview_report();
        let blocker_ids = report
            .blockers
            .iter()
            .map(|blocker| blocker.id)
            .collect::<Vec<_>>();

        assert_eq!(
            blocker_ids,
            [
                "append_only_work_graph_events_disabled",
                "runtime_canonical_adapter_enforcement_disabled",
                "canonical_adapter_projection_partial_or_gap",
                "canonical_adapter_inventory_readiness_rerun_missing",
            ]
        );
        assert_eq!(report.blocker_count, 4);
        assert_eq!(report.required_prior_gate_count, 11);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_APPLICATION_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_canonical_adapter_inventory_readiness_rerun_preview);
        assert!(!report.ready_for_runtime_adapter_enforcement);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn canonical_adapter_inventory_application_projects_all_assertion_types() {
        let report = hepta_work_graph_canonical_adapter_inventory_application_preview_report();

        assert_eq!(report.identity_application_count, 12);
        assert_eq!(report.edge_kind_application_count, 12);
        assert_eq!(report.collection_binding_application_count, 12);
        assert_eq!(report.timeline_event_application_count, 12);
        assert_eq!(report.blocker_application_count, 8);
        assert_eq!(report.application_guard_count, 8);
    }
}
