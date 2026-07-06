use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_SHADOW_LIVE_READBACK_COMPARISON_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_shadow_live_readback_comparison_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_SHADOW_LIVE_READBACK_COMPARISON_SCHEMA_VERSION: &str =
    "work_graph_persistence_shadow_live_readback_comparison_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_SHADOW_LIVE_READBACK_COMPARISON_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_persistence_enforcement_rollout_blocker_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceShadowLiveReadbackComparisonPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub comparison_surface_count: usize,
    pub readback_pair_count: usize,
    pub mismatch_classifier_count: usize,
    pub promotion_denial_count: usize,
    pub operator_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub comparison_surfaces: Vec<WorkGraphPersistenceComparisonSurfacePreview>,
    pub readback_pairs: Vec<WorkGraphPersistenceReadbackPairPreview>,
    pub mismatch_classifiers: Vec<WorkGraphPersistenceMismatchClassifierPreview>,
    pub promotion_denials: Vec<WorkGraphPersistenceShadowLivePromotionDenialPreview>,
    pub operator_views: Vec<WorkGraphPersistenceShadowLiveOperatorViewPreview>,
    pub durable_identity_evidence: WorkGraphPersistenceShadowLiveDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphPersistenceShadowLiveReadbackInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_enforcement_rollout_blocker_preview: bool,
    pub ready_for_live_readback: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: WorkGraphPersistenceShadowLiveReadbackComparisonPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceComparisonSurfacePreview {
    pub id: &'static str,
    pub source_collection: &'static str,
    pub shadow_probe_id: &'static str,
    pub future_live_probe_id: &'static str,
    pub comparison_mode: &'static str,
    pub live_read_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceReadbackPairPreview {
    pub id: &'static str,
    pub surface_id: &'static str,
    pub required_digest_fields: Vec<&'static str>,
    pub tolerance: &'static str,
    pub blocks_promotion_on_mismatch: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceMismatchClassifierPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub applies_to_surface_ids: Vec<&'static str>,
    pub trigger: &'static str,
    pub quarantine_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceShadowLivePromotionDenialPreview {
    pub id: &'static str,
    pub applies_to_classifier_ids: Vec<&'static str>,
    pub operator_message: &'static str,
    pub blocks_promotion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceShadowLiveOperatorViewPreview {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceShadowLiveDurableIdentityEvidencePreview {
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_readback_pair_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceShadowLiveReadbackInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceShadowLiveReadbackComparisonPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub shadow_readback_executed: bool,
    pub live_readback_executed: bool,
    pub comparison_executed: bool,
    pub promotion_performed: bool,
    pub enforcement_enabled: bool,
    pub feature_flag_mutated: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub approval_recorded: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_persistence_shadow_live_readback_comparison_preview_report()
-> WorkGraphPersistenceShadowLiveReadbackComparisonPreviewReport {
    let comparison_surfaces = work_graph_persistence_shadow_live_comparison_surfaces();
    let readback_pairs = work_graph_persistence_shadow_live_readback_pairs();
    let mismatch_classifiers = work_graph_persistence_shadow_live_mismatch_classifiers();
    let promotion_denials = work_graph_persistence_shadow_live_promotion_denials();
    let operator_views = work_graph_persistence_shadow_live_operator_views();
    let durable_identity_evidence = work_graph_persistence_shadow_live_durable_identity_evidence();
    let invariants = work_graph_persistence_shadow_live_readback_invariants();

    WorkGraphPersistenceShadowLiveReadbackComparisonPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_SHADOW_LIVE_READBACK_COMPARISON_PREVIEW_GATE,
        schema_version: WORK_GRAPH_PERSISTENCE_SHADOW_LIVE_READBACK_COMPARISON_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_shadow_live_readback_comparison_preview_no_live_read",
        comparison_surface_count: comparison_surfaces.len(),
        readback_pair_count: readback_pairs.len(),
        mismatch_classifier_count: mismatch_classifiers.len(),
        promotion_denial_count: promotion_denials.len(),
        operator_view_count: operator_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_persistence_shadow_live_readback_comparison_required_prior_gates(),
        comparison_surfaces,
        readback_pairs,
        mismatch_classifiers,
        promotion_denials,
        operator_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_SHADOW_LIVE_READBACK_COMPARISON_RECOMMENDED_NEXT_GATE,
        ready_for_enforcement_rollout_blocker_preview: true,
        ready_for_live_readback: false,
        ready_for_live_persistence: false,
        side_effects: WorkGraphPersistenceShadowLiveReadbackComparisonPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_shadow_live_readback_comparison_required_prior_gates()
-> Vec<&'static str> {
    vec![
        "hepta_work_graph_contract_preview_gate",
        "hepta_work_graph_task_result_contract_preview_gate",
        "hepta_work_graph_scheduler_admission_controller_preview_gate",
        "hepta_work_graph_observability_timeline_preview_gate",
        "hepta_work_graph_role_manifest_contract_preview_gate",
        "hepta_work_graph_unified_state_store_preview_gate",
        "hepta_work_graph_adapter_projection_fixture_gate",
        "hepta_work_graph_state_store_persistence_preview_gate",
        "hepta_work_graph_replay_readback_preview_gate",
        "hepta_work_graph_promotion_precondition_preview_gate",
        "hepta_work_graph_activation_enforcement_blocker_preview_gate",
        "hepta_work_graph_shadow_adapter_readback_preview_gate",
        "hepta_work_graph_persistence_feature_flag_preview_gate",
        "hepta_work_graph_persistence_canary_dry_run_preview_gate",
        "hepta_work_graph_persistence_canary_readback_receipt_preview_gate",
        "hepta_work_graph_persistence_promotion_blocker_preview_gate",
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_shadow_live_surface_ids() -> Vec<&'static str> {
    vec![
        "work_graph_node_collection_comparison",
        "work_graph_edge_collection_comparison",
        "work_graph_task_result_collection_comparison",
        "work_graph_artifact_collection_comparison",
        "work_graph_approval_collection_comparison",
        "work_graph_timeline_collection_comparison",
    ]
}

pub fn work_graph_persistence_shadow_live_readback_pair_ids() -> Vec<&'static str> {
    vec![
        "compare_node_shadow_to_future_live_digest",
        "compare_edge_shadow_to_future_live_digest",
        "compare_task_result_shadow_to_future_live_digest",
        "compare_artifact_shadow_to_future_live_digest",
        "compare_approval_shadow_to_future_live_digest",
        "compare_timeline_shadow_to_future_live_digest",
    ]
}

pub fn work_graph_persistence_shadow_live_durable_identity_field_ids() -> Vec<&'static str> {
    vec![
        "workflow_id",
        "run_id",
        "step_id",
        "checkpoint",
        "replay_key",
        "rollback_anchor",
        "receipt_hash",
    ]
}

pub fn work_graph_persistence_shadow_live_comparison_surfaces()
-> Vec<WorkGraphPersistenceComparisonSurfacePreview> {
    vec![
        comparison_surface(
            "work_graph_node_collection_comparison",
            "nodes",
            "shadow_nodes_readback_probe",
            "future_live_nodes_readback_probe",
        ),
        comparison_surface(
            "work_graph_edge_collection_comparison",
            "edges",
            "shadow_edges_readback_probe",
            "future_live_edges_readback_probe",
        ),
        comparison_surface(
            "work_graph_task_result_collection_comparison",
            "task_results",
            "shadow_task_results_readback_probe",
            "future_live_task_results_readback_probe",
        ),
        comparison_surface(
            "work_graph_artifact_collection_comparison",
            "artifacts",
            "shadow_artifacts_readback_probe",
            "future_live_artifacts_readback_probe",
        ),
        comparison_surface(
            "work_graph_approval_collection_comparison",
            "approvals",
            "shadow_approvals_readback_probe",
            "future_live_approvals_readback_probe",
        ),
        comparison_surface(
            "work_graph_timeline_collection_comparison",
            "timeline_events",
            "shadow_timeline_readback_probe",
            "future_live_timeline_readback_probe",
        ),
    ]
}

pub fn work_graph_persistence_shadow_live_readback_pairs()
-> Vec<WorkGraphPersistenceReadbackPairPreview> {
    vec![
        readback_pair(
            "compare_node_shadow_to_future_live_digest",
            "work_graph_node_collection_comparison",
            with_shadow_live_durable_identity_fields(vec![
                "nodeId",
                "nodeKind",
                "statusHash",
                "versionHash",
            ]),
        ),
        readback_pair(
            "compare_edge_shadow_to_future_live_digest",
            "work_graph_edge_collection_comparison",
            with_shadow_live_durable_identity_fields(vec![
                "edgeId",
                "fromNodeHash",
                "toNodeHash",
                "dependencyKind",
            ]),
        ),
        readback_pair(
            "compare_task_result_shadow_to_future_live_digest",
            "work_graph_task_result_collection_comparison",
            with_shadow_live_durable_identity_fields(vec![
                "taskId",
                "terminalStatus",
                "summaryHash",
                "evidenceHash",
            ]),
        ),
        readback_pair(
            "compare_artifact_shadow_to_future_live_digest",
            "work_graph_artifact_collection_comparison",
            with_shadow_live_durable_identity_fields(vec![
                "artifactId",
                "producerNodeHash",
                "payloadHash",
                "redactionState",
            ]),
        ),
        readback_pair(
            "compare_approval_shadow_to_future_live_digest",
            "work_graph_approval_collection_comparison",
            with_shadow_live_durable_identity_fields(vec![
                "approvalId",
                "operatorScopeHash",
                "expiryHash",
                "recordingState",
            ]),
        ),
        readback_pair(
            "compare_timeline_shadow_to_future_live_digest",
            "work_graph_timeline_collection_comparison",
            with_shadow_live_durable_identity_fields(vec![
                "eventId",
                "spanHash",
                "operatorSummaryHash",
                "redactionState",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_shadow_live_mismatch_classifiers()
-> Vec<WorkGraphPersistenceMismatchClassifierPreview> {
    let all_surfaces = work_graph_persistence_shadow_live_comparison_surfaces()
        .iter()
        .map(|surface| surface.id)
        .collect::<Vec<_>>();

    vec![
        mismatch_classifier(
            "missing_shadow_digest",
            "critical",
            all_surfaces.clone(),
            "shadow digest packet is absent",
        ),
        mismatch_classifier(
            "durable_identity_digest_missing",
            "critical",
            all_surfaces.clone(),
            "workflow, run, step, checkpoint, replay, rollback, or receipt identity digest is absent",
        ),
        mismatch_classifier(
            "future_live_probe_not_authorized",
            "critical",
            all_surfaces.clone(),
            "future live probe lacks feature flag and operator packet",
        ),
        mismatch_classifier(
            "schema_version_drift",
            "high",
            all_surfaces.clone(),
            "shadow schema version differs from future live schema version",
        ),
        mismatch_classifier(
            "collection_count_drift",
            "high",
            all_surfaces,
            "shadow and future live collection counts differ",
        ),
        mismatch_classifier(
            "redaction_state_drift",
            "critical",
            vec![
                "work_graph_artifact_collection_comparison",
                "work_graph_approval_collection_comparison",
                "work_graph_timeline_collection_comparison",
            ],
            "redaction state differs across compared digests",
        ),
        mismatch_classifier(
            "operator_scope_drift",
            "critical",
            vec!["work_graph_approval_collection_comparison"],
            "operator authority scope differs across compared approval digests",
        ),
    ]
}

pub fn work_graph_persistence_shadow_live_promotion_denials()
-> Vec<WorkGraphPersistenceShadowLivePromotionDenialPreview> {
    vec![
        promotion_denial(
            "deny_missing_shadow_digest",
            vec!["missing_shadow_digest"],
            "shadow digest must exist before any live readback comparison or promotion",
        ),
        promotion_denial(
            "deny_durable_identity_digest_missing",
            vec!["durable_identity_digest_missing"],
            "durable identity digest must exist before shadow/live comparison or promotion",
        ),
        promotion_denial(
            "deny_future_live_probe_without_authorization",
            vec!["future_live_probe_not_authorized"],
            "future live probe cannot run without feature flag and operator enablement",
        ),
        promotion_denial(
            "deny_schema_version_drift",
            vec!["schema_version_drift"],
            "schema drift blocks persistence promotion",
        ),
        promotion_denial(
            "deny_collection_count_drift",
            vec!["collection_count_drift"],
            "collection count drift blocks persistence promotion",
        ),
        promotion_denial(
            "deny_redaction_state_drift",
            vec!["redaction_state_drift"],
            "redaction drift blocks release and external publication",
        ),
        promotion_denial(
            "deny_operator_scope_drift",
            vec!["operator_scope_drift"],
            "operator scope drift blocks approval recording and external delivery",
        ),
    ]
}

pub fn work_graph_persistence_shadow_live_operator_views()
-> Vec<WorkGraphPersistenceShadowLiveOperatorViewPreview> {
    vec![
        operator_view(
            "operator_shadow_live_comparison_summary",
            "operator",
            with_shadow_live_durable_identity_fields(vec![
                "surfaceIds",
                "readbackPairIds",
                "mismatchClassifierIds",
                "promotionDenied",
            ]),
        ),
        operator_view(
            "auditor_mismatch_classifier_view",
            "auditor",
            with_shadow_live_durable_identity_fields(vec![
                "classifierId",
                "severity",
                "surfaceId",
                "quarantineRequired",
            ]),
        ),
        operator_view(
            "rollback_quarantine_preview_view",
            "rollback_owner",
            with_shadow_live_durable_identity_fields(vec![
                "surfaceId",
                "classifierId",
                "killSwitchId",
                "quarantineScope",
            ]),
        ),
        operator_view(
            "enforcement_rollout_blocker_view",
            "system",
            with_shadow_live_durable_identity_fields(vec![
                "denialIds",
                "nextGate",
                "liveReadEnabled",
                "sideEffectHash",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_shadow_live_durable_identity_evidence()
-> WorkGraphPersistenceShadowLiveDurableIdentityEvidencePreview {
    WorkGraphPersistenceShadowLiveDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: work_graph_persistence_shadow_live_durable_identity_field_ids(),
        required_for_readback_pair_ids: work_graph_persistence_shadow_live_readback_pair_ids(),
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_shadow_live_readback_invariants()
-> Vec<WorkGraphPersistenceShadowLiveReadbackInvariantPreview> {
    vec![
        invariant(
            "shadow_live_comparison_requires_durable_identity_evidence",
            "shadow/live comparison pairs require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "shadow_live_comparison_is_digest_only",
            "comparison surfaces describe hash-only readback pairs and never expose payloads",
        ),
        invariant(
            "live_readback_is_disabled_in_preview",
            "future live probe ids are declared, but no live readback can execute in this gate",
        ),
        invariant(
            "any_mismatch_blocks_promotion",
            "all mismatch classifiers have promotion denial records before future execution",
        ),
        invariant(
            "redaction_and_scope_drift_are_critical",
            "redaction and operator-scope mismatch must block release and approval recording",
        ),
        invariant(
            "operator_views_are_local_only",
            "operator comparison views are local preview shapes and cannot be externally delivered",
        ),
        invariant(
            "shadow_live_readback_comparison_preview_has_no_side_effects",
            "this gate cannot run readbacks, compare live state, enable enforcement, promote, or send externally",
        ),
    ]
}

impl WorkGraphPersistenceShadowLiveReadbackComparisonPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            shadow_readback_executed: false,
            live_readback_executed: false,
            comparison_executed: false,
            promotion_performed: false,
            enforcement_enabled: false,
            feature_flag_mutated: false,
            wal_written: false,
            checkpoint_written: false,
            approval_recorded: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn comparison_surface(
    id: &'static str,
    source_collection: &'static str,
    shadow_probe_id: &'static str,
    future_live_probe_id: &'static str,
) -> WorkGraphPersistenceComparisonSurfacePreview {
    WorkGraphPersistenceComparisonSurfacePreview {
        id,
        source_collection,
        shadow_probe_id,
        future_live_probe_id,
        comparison_mode: "hash_only_shadow_to_future_live_digest",
        live_read_enabled: false,
    }
}

fn readback_pair(
    id: &'static str,
    surface_id: &'static str,
    required_digest_fields: Vec<&'static str>,
) -> WorkGraphPersistenceReadbackPairPreview {
    WorkGraphPersistenceReadbackPairPreview {
        id,
        surface_id,
        required_digest_fields,
        tolerance: "exact_digest_match_required",
        blocks_promotion_on_mismatch: true,
    }
}

fn with_shadow_live_durable_identity_fields(fields: Vec<&'static str>) -> Vec<&'static str> {
    let mut merged = work_graph_persistence_shadow_live_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn mismatch_classifier(
    id: &'static str,
    severity: &'static str,
    applies_to_surface_ids: Vec<&'static str>,
    trigger: &'static str,
) -> WorkGraphPersistenceMismatchClassifierPreview {
    WorkGraphPersistenceMismatchClassifierPreview {
        id,
        severity,
        applies_to_surface_ids,
        trigger,
        quarantine_required: true,
    }
}

fn promotion_denial(
    id: &'static str,
    applies_to_classifier_ids: Vec<&'static str>,
    operator_message: &'static str,
) -> WorkGraphPersistenceShadowLivePromotionDenialPreview {
    WorkGraphPersistenceShadowLivePromotionDenialPreview {
        id,
        applies_to_classifier_ids,
        operator_message,
        blocks_promotion: true,
    }
}

fn operator_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceShadowLiveOperatorViewPreview {
    WorkGraphPersistenceShadowLiveOperatorViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceShadowLiveReadbackInvariantPreview {
    WorkGraphPersistenceShadowLiveReadbackInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_live_readback_comparison_preview_declares_surfaces_and_pairs() {
        let report = hepta_work_graph_persistence_shadow_live_readback_comparison_preview_report();
        let surface_ids = report
            .comparison_surfaces
            .iter()
            .map(|surface| surface.id)
            .collect::<Vec<_>>();

        assert_eq!(
            surface_ids,
            [
                "work_graph_node_collection_comparison",
                "work_graph_edge_collection_comparison",
                "work_graph_task_result_collection_comparison",
                "work_graph_artifact_collection_comparison",
                "work_graph_approval_collection_comparison",
                "work_graph_timeline_collection_comparison",
            ]
        );
        assert_eq!(report.comparison_surface_count, 6);
        assert_eq!(report.readback_pair_count, 6);
        assert!(
            report
                .comparison_surfaces
                .iter()
                .all(|surface| !surface.live_read_enabled)
        );
        assert!(report.readback_pairs.iter().all(|pair| {
            pair.tolerance == "exact_digest_match_required"
                && pair.blocks_promotion_on_mismatch
                && pair.required_digest_fields.len() >= 4
                && work_graph_persistence_shadow_live_durable_identity_field_ids()
                    .iter()
                    .all(|field| pair.required_digest_fields.contains(field))
        }));
    }

    #[test]
    fn shadow_live_readback_comparison_preview_classifies_mismatches() {
        let report = hepta_work_graph_persistence_shadow_live_readback_comparison_preview_report();
        let classifier_ids = report
            .mismatch_classifiers
            .iter()
            .map(|classifier| classifier.id)
            .collect::<Vec<_>>();

        assert_eq!(
            classifier_ids,
            [
                "missing_shadow_digest",
                "durable_identity_digest_missing",
                "future_live_probe_not_authorized",
                "schema_version_drift",
                "collection_count_drift",
                "redaction_state_drift",
                "operator_scope_drift",
            ]
        );
        assert_eq!(report.mismatch_classifier_count, 7);
        assert!(
            report
                .mismatch_classifiers
                .iter()
                .all(|classifier| classifier.quarantine_required)
        );
    }

    #[test]
    fn shadow_live_readback_comparison_preview_denies_promotion_for_each_classifier() {
        let report = hepta_work_graph_persistence_shadow_live_readback_comparison_preview_report();
        let denial_ids = report
            .promotion_denials
            .iter()
            .map(|denial| denial.id)
            .collect::<Vec<_>>();

        assert_eq!(
            denial_ids,
            [
                "deny_missing_shadow_digest",
                "deny_durable_identity_digest_missing",
                "deny_future_live_probe_without_authorization",
                "deny_schema_version_drift",
                "deny_collection_count_drift",
                "deny_redaction_state_drift",
                "deny_operator_scope_drift",
            ]
        );
        assert_eq!(report.promotion_denial_count, 7);
        assert!(
            report
                .promotion_denials
                .iter()
                .all(|denial| denial.blocks_promotion)
        );
    }

    #[test]
    fn shadow_live_readback_comparison_preview_keeps_views_local_and_no_side_effects() {
        let report = hepta_work_graph_persistence_shadow_live_readback_comparison_preview_report();
        let view_ids = report
            .operator_views
            .iter()
            .map(|view| view.id)
            .collect::<Vec<_>>();

        assert_eq!(
            view_ids,
            [
                "operator_shadow_live_comparison_summary",
                "auditor_mismatch_classifier_view",
                "rollback_quarantine_preview_view",
                "enforcement_rollout_blocker_view",
            ]
        );
        assert_eq!(report.operator_view_count, 4);
        assert!(report.operator_views.iter().all(|view| {
            !view.external_delivery_enabled
                && view.required_fields.contains(&"workflow_id")
                && view.required_fields.contains(&"receipt_hash")
        }));
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceShadowLiveReadbackComparisonPreviewSideEffects::none()
        );
        assert_eq!(
            report.durable_identity_evidence.required_field_ids,
            work_graph_persistence_shadow_live_durable_identity_field_ids()
        );
        assert_eq!(
            report
                .durable_identity_evidence
                .required_for_readback_pair_ids,
            work_graph_persistence_shadow_live_readback_pair_ids()
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert!(report.ready_for_enforcement_rollout_blocker_preview);
        assert!(!report.ready_for_live_readback);
        assert!(!report.ready_for_live_persistence);
    }

    #[test]
    fn shadow_live_readback_comparison_preview_requires_promotion_blocker_gate() {
        let report = hepta_work_graph_persistence_shadow_live_readback_comparison_preview_report();

        assert_eq!(
            report.required_prior_gates,
            [
                "hepta_work_graph_contract_preview_gate",
                "hepta_work_graph_task_result_contract_preview_gate",
                "hepta_work_graph_scheduler_admission_controller_preview_gate",
                "hepta_work_graph_observability_timeline_preview_gate",
                "hepta_work_graph_role_manifest_contract_preview_gate",
                "hepta_work_graph_unified_state_store_preview_gate",
                "hepta_work_graph_adapter_projection_fixture_gate",
                "hepta_work_graph_state_store_persistence_preview_gate",
                "hepta_work_graph_replay_readback_preview_gate",
                "hepta_work_graph_promotion_precondition_preview_gate",
                "hepta_work_graph_activation_enforcement_blocker_preview_gate",
                "hepta_work_graph_shadow_adapter_readback_preview_gate",
                "hepta_work_graph_persistence_feature_flag_preview_gate",
                "hepta_work_graph_persistence_canary_dry_run_preview_gate",
                "hepta_work_graph_persistence_canary_readback_receipt_preview_gate",
                "hepta_work_graph_persistence_promotion_blocker_preview_gate",
                "hepta_work_graph_durable_identity_preview_gate",
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_SHADOW_LIVE_READBACK_COMPARISON_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(report.invariant_count, 7);
    }
}
