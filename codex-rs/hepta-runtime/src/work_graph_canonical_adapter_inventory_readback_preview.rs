use serde::Serialize;

use crate::work_graph_canonical_adapter_inventory_preview::WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_PREVIEW_GATE;
use crate::work_graph_canonical_adapter_inventory_preview::WorkGraphCanonicalAdapterInventorySourcePreview;
use crate::work_graph_canonical_adapter_inventory_preview::work_graph_canonical_adapter_inventory_blockers;
use crate::work_graph_canonical_adapter_inventory_preview::work_graph_canonical_adapter_inventory_required_prior_gates;
use crate::work_graph_canonical_adapter_inventory_preview::work_graph_canonical_adapter_inventory_sources;

pub const WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_READBACK_PREVIEW_GATE: &str =
    "hepta_work_graph_canonical_adapter_inventory_readback_preview_gate";
pub const WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_READBACK_SCHEMA_VERSION: &str =
    "work_graph_canonical_adapter_inventory_readback_preview_v1";
pub const WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_canonical_adapter_inventory_application_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryReadbackPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_surface_count: usize,
    pub readback_plan_count: usize,
    pub identity_assertion_count: usize,
    pub edge_kind_assertion_count: usize,
    pub collection_binding_assertion_count: usize,
    pub timeline_event_assertion_count: usize,
    pub blocker_mapping_assertion_count: usize,
    pub drift_detector_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_plans: Vec<WorkGraphCanonicalAdapterInventoryReadbackPlanPreview>,
    pub identity_assertions: Vec<WorkGraphCanonicalAdapterInventoryIdentityAssertionPreview>,
    pub edge_kind_assertions: Vec<WorkGraphCanonicalAdapterInventoryEdgeKindAssertionPreview>,
    pub collection_binding_assertions:
        Vec<WorkGraphCanonicalAdapterInventoryCollectionBindingAssertionPreview>,
    pub timeline_event_assertions: Vec<WorkGraphCanonicalAdapterInventoryTimelineAssertionPreview>,
    pub blocker_mapping_assertions:
        Vec<WorkGraphCanonicalAdapterInventoryBlockerMappingAssertionPreview>,
    pub drift_detectors: Vec<WorkGraphCanonicalAdapterInventoryDriftDetectorPreview>,
    pub blockers: Vec<WorkGraphCanonicalAdapterInventoryReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_canonical_adapter_inventory_application_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphCanonicalAdapterInventoryReadbackPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryReadbackPlanPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub canonical_inventory_state: &'static str,
    pub expected_identity_field_count: usize,
    pub expected_edge_kind_count: usize,
    pub expected_collection_binding_count: usize,
    pub expected_timeline_event_binding_count: usize,
    pub expected_inventory_blocker_count: usize,
    pub readback_status: &'static str,
    pub readback_execution_enabled: bool,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryIdentityAssertionPreview {
    pub source_surface_id: &'static str,
    pub canonical_node_kind: &'static str,
    pub required_identity_fields: Vec<&'static str>,
    pub deterministic_identity_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryEdgeKindAssertionPreview {
    pub source_surface_id: &'static str,
    pub canonical_edge_kinds: Vec<&'static str>,
    pub edge_namespace: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryCollectionBindingAssertionPreview {
    pub source_surface_id: &'static str,
    pub canonical_collection_ids: Vec<&'static str>,
    pub store_projection_persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryTimelineAssertionPreview {
    pub source_surface_id: &'static str,
    pub timeline_event_type_ids: Vec<&'static str>,
    pub timeline_persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryBlockerMappingAssertionPreview {
    pub blocker_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub blocks_append_only_fact_source: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryDriftDetectorPreview {
    pub id: &'static str,
    pub source_fields: Vec<&'static str>,
    pub drift_budget: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryReadbackBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalAdapterInventoryReadbackPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_events_persisted: bool,
    pub readback_executed: bool,
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

pub fn hepta_work_graph_canonical_adapter_inventory_readback_preview_report()
-> WorkGraphCanonicalAdapterInventoryReadbackPreviewReport {
    let adapters = work_graph_canonical_adapter_inventory_sources();
    let readback_plans = work_graph_canonical_adapter_inventory_readback_plans_from(&adapters);
    let identity_assertions =
        work_graph_canonical_adapter_inventory_identity_assertions_from(&adapters);
    let edge_kind_assertions =
        work_graph_canonical_adapter_inventory_edge_kind_assertions_from(&adapters);
    let collection_binding_assertions =
        work_graph_canonical_adapter_inventory_collection_binding_assertions_from(&adapters);
    let timeline_event_assertions =
        work_graph_canonical_adapter_inventory_timeline_assertions_from(&adapters);
    let blocker_mapping_assertions =
        work_graph_canonical_adapter_inventory_blocker_mapping_assertions();
    let drift_detectors = work_graph_canonical_adapter_inventory_drift_detectors();
    let blockers = work_graph_canonical_adapter_inventory_readback_blockers();
    let required_prior_gates =
        work_graph_canonical_adapter_inventory_readback_required_prior_gates();

    WorkGraphCanonicalAdapterInventoryReadbackPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_READBACK_PREVIEW_GATE,
        schema_version: WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_canonical_adapter_inventory_readback_preview_no_execution",
        source_surface_count: adapters.len(),
        readback_plan_count: readback_plans.len(),
        identity_assertion_count: identity_assertions.len(),
        edge_kind_assertion_count: edge_kind_assertions.len(),
        collection_binding_assertion_count: collection_binding_assertions.len(),
        timeline_event_assertion_count: timeline_event_assertions.len(),
        blocker_mapping_assertion_count: blocker_mapping_assertions.len(),
        drift_detector_count: drift_detectors.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_plans,
        identity_assertions,
        edge_kind_assertions,
        collection_binding_assertions,
        timeline_event_assertions,
        blocker_mapping_assertions,
        drift_detectors,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_canonical_adapter_inventory_application_preview: true,
        ready_for_append_only_work_graph_events: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphCanonicalAdapterInventoryReadbackPreviewSideEffects::none(),
    }
}

pub fn work_graph_canonical_adapter_inventory_readback_plans()
-> Vec<WorkGraphCanonicalAdapterInventoryReadbackPlanPreview> {
    work_graph_canonical_adapter_inventory_readback_plans_from(
        &work_graph_canonical_adapter_inventory_sources(),
    )
}

pub fn work_graph_canonical_adapter_inventory_blocker_mapping_assertions()
-> Vec<WorkGraphCanonicalAdapterInventoryBlockerMappingAssertionPreview> {
    work_graph_canonical_adapter_inventory_blockers()
        .into_iter()
        .map(
            |blocker| WorkGraphCanonicalAdapterInventoryBlockerMappingAssertionPreview {
                blocker_id: blocker.id,
                affected_source_surface_ids: blocker.affected_source_surface_ids,
                blocks_append_only_fact_source: blocker.blocks_append_only_fact_source,
            },
        )
        .collect()
}

pub fn work_graph_canonical_adapter_inventory_drift_detectors()
-> Vec<WorkGraphCanonicalAdapterInventoryDriftDetectorPreview> {
    vec![
        drift_detector("source_surface_order_drift", vec!["source_surface_id"]),
        drift_detector(
            "canonical_node_kind_drift",
            vec!["source_surface_id", "canonical_node_kind"],
        ),
        drift_detector(
            "canonical_identity_field_drift",
            vec!["source_surface_id", "canonical_identity_fields"],
        ),
        drift_detector(
            "canonical_edge_kind_drift",
            vec!["source_surface_id", "canonical_edge_kinds"],
        ),
        drift_detector(
            "canonical_collection_binding_drift",
            vec!["source_surface_id", "canonical_collection_ids"],
        ),
        drift_detector(
            "timeline_event_binding_drift",
            vec!["source_surface_id", "timeline_event_type_ids"],
        ),
        drift_detector(
            "inventory_blocker_mapping_drift",
            vec!["source_surface_id", "inventory_blocker_ids"],
        ),
    ]
}

pub fn work_graph_canonical_adapter_inventory_readback_blockers()
-> Vec<WorkGraphCanonicalAdapterInventoryReadbackBlockerPreview> {
    let source_surface_ids = work_graph_canonical_adapter_inventory_sources()
        .into_iter()
        .map(|adapter| adapter.source_surface_id)
        .collect::<Vec<_>>();
    vec![
        readback_blocker(
            "canonical_adapter_inventory_readback_not_executed",
            "high",
            source_surface_ids.clone(),
            "keep readback as a preview until the append-only event store can replay canonical adapter rows",
        ),
        readback_blocker(
            "canonical_adapter_inventory_application_missing",
            "high",
            source_surface_ids.clone(),
            "apply readback-verified adapter rows into a no-mutation application outcome before any enforcement",
        ),
        readback_blocker(
            "append_only_work_graph_events_disabled",
            "high",
            source_surface_ids,
            "do not persist WorkGraph events until readback and operator-review boundaries are promoted",
        ),
    ]
}

pub fn work_graph_canonical_adapter_inventory_readback_required_prior_gates() -> Vec<&'static str> {
    let mut gates = work_graph_canonical_adapter_inventory_required_prior_gates();
    gates.push(WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_PREVIEW_GATE);
    gates
}

impl WorkGraphCanonicalAdapterInventoryReadbackPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_events_persisted: false,
            readback_executed: false,
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

fn work_graph_canonical_adapter_inventory_readback_plans_from(
    adapters: &[WorkGraphCanonicalAdapterInventorySourcePreview],
) -> Vec<WorkGraphCanonicalAdapterInventoryReadbackPlanPreview> {
    adapters
        .iter()
        .map(
            |adapter| WorkGraphCanonicalAdapterInventoryReadbackPlanPreview {
                source_surface_id: adapter.source_surface_id,
                source_category: adapter.source_category,
                canonical_inventory_state: adapter.canonical_inventory_state,
                expected_identity_field_count: adapter.canonical_identity_fields.len(),
                expected_edge_kind_count: adapter.canonical_edge_kinds.len(),
                expected_collection_binding_count: adapter.canonical_collection_ids.len(),
                expected_timeline_event_binding_count: adapter.timeline_event_type_ids.len(),
                expected_inventory_blocker_count: adapter.inventory_blocker_ids.len(),
                readback_status: "readback_plan_ready",
                readback_execution_enabled: false,
                next_required_gate:
                    WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_READBACK_RECOMMENDED_NEXT_GATE,
            },
        )
        .collect()
}

fn work_graph_canonical_adapter_inventory_identity_assertions_from(
    adapters: &[WorkGraphCanonicalAdapterInventorySourcePreview],
) -> Vec<WorkGraphCanonicalAdapterInventoryIdentityAssertionPreview> {
    adapters
        .iter()
        .map(
            |adapter| WorkGraphCanonicalAdapterInventoryIdentityAssertionPreview {
                source_surface_id: adapter.source_surface_id,
                canonical_node_kind: adapter.canonical_node_kind,
                required_identity_fields: adapter.canonical_identity_fields.clone(),
                deterministic_identity_required: true,
            },
        )
        .collect()
}

fn work_graph_canonical_adapter_inventory_edge_kind_assertions_from(
    adapters: &[WorkGraphCanonicalAdapterInventorySourcePreview],
) -> Vec<WorkGraphCanonicalAdapterInventoryEdgeKindAssertionPreview> {
    adapters
        .iter()
        .map(
            |adapter| WorkGraphCanonicalAdapterInventoryEdgeKindAssertionPreview {
                source_surface_id: adapter.source_surface_id,
                canonical_edge_kinds: adapter.canonical_edge_kinds.clone(),
                edge_namespace: "work_graph_edge_kind",
            },
        )
        .collect()
}

fn work_graph_canonical_adapter_inventory_collection_binding_assertions_from(
    adapters: &[WorkGraphCanonicalAdapterInventorySourcePreview],
) -> Vec<WorkGraphCanonicalAdapterInventoryCollectionBindingAssertionPreview> {
    adapters
        .iter()
        .map(
            |adapter| WorkGraphCanonicalAdapterInventoryCollectionBindingAssertionPreview {
                source_surface_id: adapter.source_surface_id,
                canonical_collection_ids: adapter.canonical_collection_ids.clone(),
                store_projection_persisted: false,
            },
        )
        .collect()
}

fn work_graph_canonical_adapter_inventory_timeline_assertions_from(
    adapters: &[WorkGraphCanonicalAdapterInventorySourcePreview],
) -> Vec<WorkGraphCanonicalAdapterInventoryTimelineAssertionPreview> {
    adapters
        .iter()
        .map(
            |adapter| WorkGraphCanonicalAdapterInventoryTimelineAssertionPreview {
                source_surface_id: adapter.source_surface_id,
                timeline_event_type_ids: adapter.timeline_event_type_ids.clone(),
                timeline_persisted: false,
            },
        )
        .collect()
}

fn drift_detector(
    id: &'static str,
    source_fields: Vec<&'static str>,
) -> WorkGraphCanonicalAdapterInventoryDriftDetectorPreview {
    WorkGraphCanonicalAdapterInventoryDriftDetectorPreview {
        id,
        source_fields,
        drift_budget: 0,
    }
}

fn readback_blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphCanonicalAdapterInventoryReadbackBlockerPreview {
    WorkGraphCanonicalAdapterInventoryReadbackBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        recommended_fix,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_adapter_inventory_readback_covers_every_adapter_row() {
        let report = hepta_work_graph_canonical_adapter_inventory_readback_preview_report();
        let readback_source_ids = report
            .readback_plans
            .iter()
            .map(|plan| plan.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(report.source_surface_count, 12);
        assert_eq!(report.readback_plan_count, 12);
        assert_eq!(
            readback_source_ids,
            [
                "update_plan_tool",
                "plan_mode_proposed_plan_blocks",
                "app_server_turn_plan_notification",
                "multi_agent_v2_thread_spawn",
                "multi_agent_v2_mailbox_wait",
                "hepta_runtime_multi_agent_reducer",
                "agent_jobs_batch_workers",
                "hepta_runtime_task_board",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_scheduler_store",
                "hepta_runtime_approval_broker",
                "hepta_runtime_agent_harness",
            ]
        );
    }

    #[test]
    fn canonical_adapter_inventory_readback_declares_assertions_and_drift_detectors() {
        let report = hepta_work_graph_canonical_adapter_inventory_readback_preview_report();

        assert_eq!(report.identity_assertion_count, 12);
        assert_eq!(report.edge_kind_assertion_count, 12);
        assert_eq!(report.collection_binding_assertion_count, 12);
        assert_eq!(report.timeline_event_assertion_count, 12);
        assert_eq!(report.blocker_mapping_assertion_count, 8);
        assert_eq!(report.drift_detector_count, 7);
        assert!(
            report
                .identity_assertions
                .iter()
                .all(|assertion| assertion.deterministic_identity_required)
        );
        assert!(
            report
                .collection_binding_assertions
                .iter()
                .all(|assertion| !assertion.store_projection_persisted)
        );
    }

    #[test]
    fn canonical_adapter_inventory_readback_preserves_no_mutation_blockers() {
        let report = hepta_work_graph_canonical_adapter_inventory_readback_preview_report();
        let blocker_ids = report
            .blockers
            .iter()
            .map(|blocker| blocker.id)
            .collect::<Vec<_>>();

        assert_eq!(
            blocker_ids,
            [
                "canonical_adapter_inventory_readback_not_executed",
                "canonical_adapter_inventory_application_missing",
                "append_only_work_graph_events_disabled",
            ]
        );
        assert_eq!(report.blocker_count, 3);
        assert_eq!(report.required_prior_gate_count, 10);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_READBACK_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_canonical_adapter_inventory_application_preview);
        assert!(!report.ready_for_append_only_work_graph_events);
        assert!(!report.ready_for_runtime_adapter_enforcement);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn canonical_adapter_inventory_readback_remains_read_only() {
        let report = hepta_work_graph_canonical_adapter_inventory_readback_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphCanonicalAdapterInventoryReadbackPreviewSideEffects::none()
        );
        assert!(
            report
                .readback_plans
                .iter()
                .all(|plan| !plan.readback_execution_enabled)
        );
        assert!(
            report
                .timeline_event_assertions
                .iter()
                .all(|assertion| !assertion.timeline_persisted)
        );
    }
}
