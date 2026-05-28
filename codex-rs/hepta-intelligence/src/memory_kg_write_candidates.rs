use std::collections::BTreeSet;

use hepta_core::{
    ContextRecallItem, ContextRecallScore, ContextRecallSource, MemoryConflict, MemoryLink,
    MemoryLinkKind, MemorySourceKind, MemorySourceSpan, MemoryUnit, MemoryUnitKind, SessionId,
    TranscriptRange, TranscriptSpanRef,
};
use hepta_kg::{
    KgConfidence, KgEntity, KgEntityKind, KgEpisode, KgEpisodeKind, KgExternalAdapterClientAudit,
    KgExternalAdapterClientBlocker, KgExternalAdapterClientRequest, KgExternalAdapterConfigEnvRead,
    KgExternalAdapterDryRunPlan, KgExternalAdapterKind, KgExternalAdapterStagingBlocker,
    KgExternalAdapterStagingConfig, KgExternalAdapterStagingPlan, KgOperatorReviewState,
    KgProvenance, KgReadQuery, KgRecallPlan, KgRedactionState, KgRelation, KgRelationKind,
    KgSourceKind, KgSourceSpan, KgTemporalValidity, KgWriteCandidate, KgWriteMode, KgWritePlan,
    KgWritePolicy, default_external_adapter_staging_configs, plan_external_adapter_dry_run,
    plan_external_adapter_staging_gate, plan_kg_recall, plan_kg_write,
    preview_disabled_external_adapter_write,
    read_all_external_adapter_staging_configs_from_env_pairs,
};
use serde::{Deserialize, Serialize};

pub const MEMORY_KG_WRITE_CANDIDATE_V0_CONTRACT: &str =
    "hepta-intelligence-memory-kg-write-candidate-v0";
pub const MEMORY_KG_CONTEXT_RECALL_BRIDGE_V0_CONTRACT: &str =
    "hepta-intelligence-memory-kg-context-recall-bridge-v0";
pub const MEMORY_KG_RECALL_EVALUATION_V0_CONTRACT: &str =
    "hepta-intelligence-memory-kg-recall-evaluation-v0";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgWriteCandidateChecks {
    pub candidate_count_nonzero: bool,
    pub all_candidates_have_provenance: bool,
    pub all_candidates_have_graph_payload: bool,
    pub all_plans_are_dry_run: bool,
    pub no_live_write_enabled: bool,
    pub no_external_side_effects: bool,
}

impl MemoryKgWriteCandidateChecks {
    pub fn ready(&self) -> bool {
        self.candidate_count_nonzero
            && self.all_candidates_have_provenance
            && self.all_candidates_have_graph_payload
            && self.all_plans_are_dry_run
            && self.no_live_write_enabled
            && self.no_external_side_effects
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgWriteCandidateReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub sample_run: bool,
    pub memory_unit_count: usize,
    pub candidate_count: usize,
    pub live_write_enabled_count: usize,
    pub external_side_effect_enabled_count: usize,
    pub candidates: Vec<KgWriteCandidate>,
    pub plans: Vec<KgWritePlan>,
    pub checks: MemoryKgWriteCandidateChecks,
    pub next_phase: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgAdapterDryRunChecks {
    pub candidate_count_nonzero: bool,
    pub all_supported_adapters_projected: bool,
    pub all_projections_have_records: bool,
    pub no_network_calls_enabled: bool,
    pub no_external_writes_enabled: bool,
    pub no_live_writes_enabled: bool,
}

impl MemoryKgAdapterDryRunChecks {
    pub fn ready(&self) -> bool {
        self.candidate_count_nonzero
            && self.all_supported_adapters_projected
            && self.all_projections_have_records
            && self.no_network_calls_enabled
            && self.no_external_writes_enabled
            && self.no_live_writes_enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgAdapterDryRunReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub sample_run: bool,
    pub candidate_count: usize,
    pub adapter_count: usize,
    pub projection_count: usize,
    pub network_call_enabled_count: usize,
    pub external_write_enabled_count: usize,
    pub live_write_enabled_count: usize,
    pub projections: Vec<KgExternalAdapterDryRunPlan>,
    pub checks: MemoryKgAdapterDryRunChecks,
    pub next_phase: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgAdapterStagingGateChecks {
    pub candidate_count_nonzero: bool,
    pub all_supported_adapters_gated: bool,
    pub all_staging_plans_closed_by_default: bool,
    pub operator_review_required: bool,
    pub rollback_plan_required: bool,
    pub post_write_validation_required: bool,
    pub no_network_calls_enabled: bool,
    pub no_external_writes_enabled: bool,
    pub no_live_writes_enabled: bool,
}

impl MemoryKgAdapterStagingGateChecks {
    pub fn ready(&self) -> bool {
        self.candidate_count_nonzero
            && self.all_supported_adapters_gated
            && self.all_staging_plans_closed_by_default
            && self.operator_review_required
            && self.rollback_plan_required
            && self.post_write_validation_required
            && self.no_network_calls_enabled
            && self.no_external_writes_enabled
            && self.no_live_writes_enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgAdapterStagingGateReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub sample_run: bool,
    pub candidate_count: usize,
    pub adapter_count: usize,
    pub staging_plan_count: usize,
    pub staging_ready_count: usize,
    pub network_call_enabled_count: usize,
    pub external_write_enabled_count: usize,
    pub live_write_enabled_count: usize,
    pub plans: Vec<KgExternalAdapterStagingPlan>,
    pub checks: MemoryKgAdapterStagingGateChecks,
    pub next_phase: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgAdapterClientChecks {
    pub candidate_count_nonzero: bool,
    pub all_supported_clients_present: bool,
    pub all_client_calls_denied_by_default: bool,
    pub no_network_calls_attempted: bool,
    pub no_external_writes_attempted: bool,
    pub no_live_writes_attempted: bool,
    pub no_records_persisted: bool,
}

impl MemoryKgAdapterClientChecks {
    pub fn ready(&self) -> bool {
        self.candidate_count_nonzero
            && self.all_supported_clients_present
            && self.all_client_calls_denied_by_default
            && self.no_network_calls_attempted
            && self.no_external_writes_attempted
            && self.no_live_writes_attempted
            && self.no_records_persisted
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgAdapterClientReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub sample_run: bool,
    pub candidate_count: usize,
    pub adapter_count: usize,
    pub client_audit_count: usize,
    pub denied_client_count: usize,
    pub network_call_attempted_count: usize,
    pub external_write_attempted_count: usize,
    pub live_write_attempted_count: usize,
    pub persisted_record_count: usize,
    pub audits: Vec<KgExternalAdapterClientAudit>,
    pub checks: MemoryKgAdapterClientChecks,
    pub next_phase: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgAdapterConfigEnvChecks {
    pub all_supported_adapters_read: bool,
    pub all_env_keys_present_in_report: bool,
    pub all_configs_closed_by_default: bool,
    pub no_credential_values_captured: bool,
    pub no_network_calls_attempted: bool,
    pub no_external_writes_attempted: bool,
    pub no_live_writes_attempted: bool,
}

impl MemoryKgAdapterConfigEnvChecks {
    pub fn ready(&self) -> bool {
        self.all_supported_adapters_read
            && self.all_env_keys_present_in_report
            && self.all_configs_closed_by_default
            && self.no_credential_values_captured
            && self.no_network_calls_attempted
            && self.no_external_writes_attempted
            && self.no_live_writes_attempted
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgAdapterConfigEnvReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub sample_run: bool,
    pub adapter_count: usize,
    pub config_read_count: usize,
    pub feature_enabled_count: usize,
    pub endpoint_configured_count: usize,
    pub credentials_configured_count: usize,
    pub network_allowlisted_count: usize,
    pub external_write_allowlisted_count: usize,
    pub operator_approved_count: usize,
    pub dry_run_sample_passed_count: usize,
    pub rollback_plan_ready_count: usize,
    pub post_write_validation_ready_count: usize,
    pub fully_configured_count: usize,
    pub live_write_requested_count: usize,
    pub credential_value_captured_count: usize,
    pub network_call_attempted_count: usize,
    pub external_write_attempted_count: usize,
    pub live_write_attempted_count: usize,
    pub reads: Vec<KgExternalAdapterConfigEnvRead>,
    pub checks: MemoryKgAdapterConfigEnvChecks,
    pub next_phase: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgRecallPlanChecks {
    pub candidate_count_nonzero: bool,
    pub entity_matches_nonzero: bool,
    pub relation_neighborhoods_nonzero: bool,
    pub timeline_slices_nonzero: bool,
    pub evidence_paths_nonzero: bool,
    pub all_plans_are_read_only: bool,
    pub no_external_reads_enabled: bool,
    pub no_network_calls_enabled: bool,
    pub no_live_writes_enabled: bool,
}

impl MemoryKgRecallPlanChecks {
    pub fn ready(&self) -> bool {
        self.candidate_count_nonzero
            && self.entity_matches_nonzero
            && self.relation_neighborhoods_nonzero
            && self.timeline_slices_nonzero
            && self.evidence_paths_nonzero
            && self.all_plans_are_read_only
            && self.no_external_reads_enabled
            && self.no_network_calls_enabled
            && self.no_live_writes_enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgRecallPlanReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub sample_run: bool,
    pub query_count: usize,
    pub candidate_count: usize,
    pub entity_match_count: usize,
    pub relation_neighborhood_count: usize,
    pub timeline_slice_count: usize,
    pub evidence_path_count: usize,
    pub external_read_enabled_count: usize,
    pub network_call_enabled_count: usize,
    pub live_write_enabled_count: usize,
    pub plans: Vec<KgRecallPlan>,
    pub checks: MemoryKgRecallPlanChecks,
    pub next_phase: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgContextRecallBridgeChecks {
    pub recall_plan_ready: bool,
    pub context_items_nonzero: bool,
    pub all_items_have_kg_source: bool,
    pub all_items_have_scores: bool,
    pub transcript_provenance_preserved: bool,
    pub no_external_reads_enabled: bool,
    pub no_network_calls_enabled: bool,
    pub no_live_writes_enabled: bool,
    pub no_model_invoked: bool,
    pub no_context_injection_performed: bool,
}

impl MemoryKgContextRecallBridgeChecks {
    pub fn ready(&self) -> bool {
        self.recall_plan_ready
            && self.context_items_nonzero
            && self.all_items_have_kg_source
            && self.all_items_have_scores
            && self.transcript_provenance_preserved
            && self.no_external_reads_enabled
            && self.no_network_calls_enabled
            && self.no_live_writes_enabled
            && self.no_model_invoked
            && self.no_context_injection_performed
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryKgContextRecallBridgeReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub sample_run: bool,
    pub kg_recall_contract: &'static str,
    pub query_count: usize,
    pub kg_plan_count: usize,
    pub kg_evidence_path_count: usize,
    pub context_item_count: usize,
    pub transcript_span_count: usize,
    pub external_read_enabled_count: usize,
    pub network_call_enabled_count: usize,
    pub live_write_enabled_count: usize,
    pub model_invoked: bool,
    pub context_injection_performed: bool,
    pub items: Vec<ContextRecallItem>,
    pub checks: MemoryKgContextRecallBridgeChecks,
    pub next_phase: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryKgRecallEvaluationBlocker {
    MissingEvidencePath,
    MissingEntityEvidence,
    MissingRelationEvidence,
    MissingTimelineSlice,
    MissingTranscriptProvenance,
    MissingSourceMemoryId,
    MissingScore,
    ScoreOrderViolation,
    DuplicateContextSourceId,
    DuplicateSourceMemoryId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgRecallEvaluationCase {
    pub query_id: String,
    pub candidate_id: String,
    pub context_source_id: String,
    pub entity_evidence_count: usize,
    pub relation_path_count: usize,
    pub timeline_slice_count: usize,
    pub transcript_span_count: usize,
    pub source_memory_id_count: usize,
    pub final_score_basis_points: u16,
    pub relevance_basis_points: u16,
    pub durability_basis_points: u16,
    pub confidence_basis_points: u16,
    pub passed: bool,
    #[serde(default)]
    pub blockers: Vec<MemoryKgRecallEvaluationBlocker>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgRecallEvaluationChecks {
    pub bridge_ready: bool,
    pub evaluation_cases_nonzero: bool,
    pub all_cases_passed: bool,
    pub entity_evidence_complete: bool,
    pub relation_path_complete: bool,
    pub timeline_slice_complete: bool,
    pub transcript_provenance_complete: bool,
    pub source_memory_ids_unique: bool,
    pub scores_stably_ordered: bool,
    pub no_external_reads_enabled: bool,
    pub no_network_calls_enabled: bool,
    pub no_live_writes_enabled: bool,
    pub no_model_invoked: bool,
    pub no_context_injection_performed: bool,
}

impl MemoryKgRecallEvaluationChecks {
    pub fn ready(&self) -> bool {
        self.bridge_ready
            && self.evaluation_cases_nonzero
            && self.all_cases_passed
            && self.entity_evidence_complete
            && self.relation_path_complete
            && self.timeline_slice_complete
            && self.transcript_provenance_complete
            && self.source_memory_ids_unique
            && self.scores_stably_ordered
            && self.no_external_reads_enabled
            && self.no_network_calls_enabled
            && self.no_live_writes_enabled
            && self.no_model_invoked
            && self.no_context_injection_performed
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgRecallEvaluationReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub sample_run: bool,
    pub kg_recall_contract: &'static str,
    pub kg_context_bridge_contract: &'static str,
    pub query_count: usize,
    pub context_item_count: usize,
    pub evaluation_case_count: usize,
    pub passed_case_count: usize,
    pub failed_case_count: usize,
    pub entity_evidence_case_count: usize,
    pub relation_path_case_count: usize,
    pub timeline_slice_case_count: usize,
    pub transcript_provenance_case_count: usize,
    pub duplicate_context_source_id_count: usize,
    pub duplicate_source_memory_id_count: usize,
    pub score_order_violation_count: usize,
    pub coverage_basis_points: u16,
    pub precision_proxy_basis_points: u16,
    pub score_stability_basis_points: u16,
    pub external_read_enabled_count: usize,
    pub network_call_enabled_count: usize,
    pub live_write_enabled_count: usize,
    pub model_invoked: bool,
    pub context_injection_performed: bool,
    pub cases: Vec<MemoryKgRecallEvaluationCase>,
    pub checks: MemoryKgRecallEvaluationChecks,
    pub next_phase: &'static str,
}

pub fn memory_kg_write_candidate_report(
    memory_units: &[MemoryUnit],
    sample_run: bool,
) -> MemoryKgWriteCandidateReport {
    let candidates = kg_write_candidates_from_memory_units(
        memory_units,
        "hepta-intelligence",
        "memory-kg-dry-run",
    );
    let policy = KgWritePolicy::default();
    let plans = candidates
        .iter()
        .map(|candidate| plan_kg_write(candidate, &policy))
        .collect::<Vec<_>>();
    let live_write_enabled_count = plans.iter().filter(|plan| plan.live_write_allowed).count();
    let external_side_effect_enabled_count = plans
        .iter()
        .filter(|plan| plan.external_side_effects_allowed)
        .count();
    let checks = MemoryKgWriteCandidateChecks {
        candidate_count_nonzero: !candidates.is_empty(),
        all_candidates_have_provenance: candidates
            .iter()
            .all(|candidate| candidate.provenance.has_source_evidence()),
        all_candidates_have_graph_payload: candidates
            .iter()
            .all(KgWriteCandidate::has_graph_payload),
        all_plans_are_dry_run: plans.iter().all(|plan| plan.mode == KgWriteMode::DryRun),
        no_live_write_enabled: live_write_enabled_count == 0,
        no_external_side_effects: external_side_effect_enabled_count == 0,
    };

    MemoryKgWriteCandidateReport {
        product: "Hepta",
        command: "memory-kg-write-candidates",
        contract: MEMORY_KG_WRITE_CANDIDATE_V0_CONTRACT,
        status: if checks.ready() { "ready" } else { "attention" },
        sample_run,
        memory_unit_count: memory_units.len(),
        candidate_count: candidates.len(),
        live_write_enabled_count,
        external_side_effect_enabled_count,
        candidates,
        plans,
        checks,
        next_phase: "wire reviewed KgWriteCandidate batches into a durable adapter such as Graphiti or Neo4j",
    }
}

pub fn memory_kg_adapter_dry_run_report(
    memory_units: &[MemoryUnit],
    sample_run: bool,
) -> MemoryKgAdapterDryRunReport {
    let candidate_report = memory_kg_write_candidate_report(memory_units, sample_run);
    let projections = candidate_report
        .candidates
        .iter()
        .zip(candidate_report.plans.iter())
        .flat_map(|(candidate, plan)| {
            KgExternalAdapterKind::ALL
                .into_iter()
                .map(move |adapter| plan_external_adapter_dry_run(candidate, plan, adapter))
        })
        .collect::<Vec<_>>();
    let network_call_enabled_count = projections
        .iter()
        .filter(|projection| projection.network_call_allowed)
        .count();
    let external_write_enabled_count = projections
        .iter()
        .filter(|projection| projection.external_write_allowed)
        .count();
    let live_write_enabled_count = projections
        .iter()
        .filter(|projection| projection.live_write_allowed)
        .count();
    let adapter_count = KgExternalAdapterKind::ALL.len();
    let checks = MemoryKgAdapterDryRunChecks {
        candidate_count_nonzero: candidate_report.candidate_count > 0,
        all_supported_adapters_projected: projections.len()
            == candidate_report.candidate_count * adapter_count,
        all_projections_have_records: projections
            .iter()
            .all(|projection| projection.projected_total_records > 0),
        no_network_calls_enabled: network_call_enabled_count == 0,
        no_external_writes_enabled: external_write_enabled_count == 0,
        no_live_writes_enabled: live_write_enabled_count == 0,
    };

    MemoryKgAdapterDryRunReport {
        product: "Hepta",
        command: "memory-kg-adapter-dry-run",
        contract: hepta_kg::KG_EXTERNAL_ADAPTER_DRY_RUN_CONTRACT,
        status: if checks.ready() { "ready" } else { "attention" },
        sample_run,
        candidate_count: candidate_report.candidate_count,
        adapter_count,
        projection_count: projections.len(),
        network_call_enabled_count,
        external_write_enabled_count,
        live_write_enabled_count,
        projections,
        checks,
        next_phase: "replace dry-run adapter projections with reviewed adapter-specific staging plans",
    }
}

pub fn memory_kg_adapter_staging_gate_report(
    memory_units: &[MemoryUnit],
    sample_run: bool,
) -> MemoryKgAdapterStagingGateReport {
    let dry_run_report = memory_kg_adapter_dry_run_report(memory_units, sample_run);
    let configs = default_external_adapter_staging_configs();
    let plans = dry_run_report
        .projections
        .iter()
        .map(|projection| {
            let config = configs
                .iter()
                .find(|config| config.adapter == projection.adapter)
                .expect("each supported adapter must have a default staging config");
            plan_external_adapter_staging_gate(projection, config)
        })
        .collect::<Vec<_>>();

    let staging_ready_count = plans.iter().filter(|plan| plan.staging_ready).count();
    let network_call_enabled_count = plans
        .iter()
        .filter(|plan| plan.network_call_allowed)
        .count();
    let external_write_enabled_count = plans
        .iter()
        .filter(|plan| plan.external_write_allowed)
        .count();
    let live_write_enabled_count = plans.iter().filter(|plan| plan.live_write_allowed).count();
    let adapter_count = KgExternalAdapterKind::ALL.len();
    let checks = MemoryKgAdapterStagingGateChecks {
        candidate_count_nonzero: dry_run_report.candidate_count > 0,
        all_supported_adapters_gated: configs.len() == adapter_count
            && KgExternalAdapterKind::ALL
                .into_iter()
                .all(|adapter| configs.iter().any(|config| config.adapter == adapter)),
        all_staging_plans_closed_by_default: staging_ready_count == 0,
        operator_review_required: plans.iter().all(|plan| {
            plan.blockers
                .contains(&KgExternalAdapterStagingBlocker::OperatorReviewMissing)
        }),
        rollback_plan_required: plans.iter().all(|plan| {
            plan.blockers
                .contains(&KgExternalAdapterStagingBlocker::RollbackPlanMissing)
        }),
        post_write_validation_required: plans.iter().all(|plan| {
            plan.blockers
                .contains(&KgExternalAdapterStagingBlocker::PostWriteValidationMissing)
        }),
        no_network_calls_enabled: network_call_enabled_count == 0,
        no_external_writes_enabled: external_write_enabled_count == 0,
        no_live_writes_enabled: live_write_enabled_count == 0,
    };

    MemoryKgAdapterStagingGateReport {
        product: "Hepta",
        command: "memory-kg-adapter-staging-gate",
        contract: hepta_kg::KG_EXTERNAL_ADAPTER_STAGING_GATE_CONTRACT,
        status: if checks.ready() { "ready" } else { "attention" },
        sample_run,
        candidate_count: dry_run_report.candidate_count,
        adapter_count,
        staging_plan_count: plans.len(),
        staging_ready_count,
        network_call_enabled_count,
        external_write_enabled_count,
        live_write_enabled_count,
        plans,
        checks,
        next_phase: "add disabled-by-default adapter clients behind the staging gate",
    }
}

pub fn memory_kg_adapter_client_report(
    memory_units: &[MemoryUnit],
    sample_run: bool,
) -> MemoryKgAdapterClientReport {
    let dry_run_report = memory_kg_adapter_dry_run_report(memory_units, sample_run);
    let staging_report = memory_kg_adapter_staging_gate_report(memory_units, sample_run);
    let audits = dry_run_report
        .projections
        .into_iter()
        .zip(staging_report.plans)
        .map(|(projection, staging_plan)| {
            let request = KgExternalAdapterClientRequest::from_plans(projection, staging_plan);
            preview_disabled_external_adapter_write(&request)
        })
        .collect::<Vec<_>>();

    let denied_client_count = audits
        .iter()
        .filter(|audit| {
            audit
                .blockers
                .contains(&KgExternalAdapterClientBlocker::DisabledClient)
        })
        .count();
    let network_call_attempted_count = audits
        .iter()
        .filter(|audit| audit.network_call_attempted)
        .count();
    let external_write_attempted_count = audits
        .iter()
        .filter(|audit| audit.external_write_attempted)
        .count();
    let live_write_attempted_count = audits
        .iter()
        .filter(|audit| audit.live_write_attempted)
        .count();
    let persisted_record_count = audits.iter().map(|audit| audit.persisted_records).sum();
    let adapter_count = KgExternalAdapterKind::ALL.len();
    let checks = MemoryKgAdapterClientChecks {
        candidate_count_nonzero: staging_report.candidate_count > 0,
        all_supported_clients_present: KgExternalAdapterKind::ALL.into_iter().all(|adapter| {
            audits
                .iter()
                .any(|audit| audit.adapter == adapter && audit.adapter_id == adapter.id())
        }),
        all_client_calls_denied_by_default: denied_client_count == audits.len(),
        no_network_calls_attempted: network_call_attempted_count == 0,
        no_external_writes_attempted: external_write_attempted_count == 0,
        no_live_writes_attempted: live_write_attempted_count == 0,
        no_records_persisted: persisted_record_count == 0,
    };

    MemoryKgAdapterClientReport {
        product: "Hepta",
        command: "memory-kg-adapter-client-denial",
        contract: hepta_kg::KG_EXTERNAL_ADAPTER_CLIENT_CONTRACT,
        status: if checks.ready() { "ready" } else { "attention" },
        sample_run,
        candidate_count: staging_report.candidate_count,
        adapter_count,
        client_audit_count: audits.len(),
        denied_client_count,
        network_call_attempted_count,
        external_write_attempted_count,
        live_write_attempted_count,
        persisted_record_count,
        audits,
        checks,
        next_phase: "replace disabled adapter clients with feature-gated real clients after staging approval",
    }
}

pub fn memory_kg_adapter_config_env_report(sample_run: bool) -> MemoryKgAdapterConfigEnvReport {
    memory_kg_adapter_config_env_report_from_env_pairs(sample_run, Vec::<(&str, &str)>::new())
}

pub fn memory_kg_adapter_config_env_report_from_env_pairs<I, K, V>(
    sample_run: bool,
    vars: I,
) -> MemoryKgAdapterConfigEnvReport
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    let reads = read_all_external_adapter_staging_configs_from_env_pairs(vars);
    let adapter_count = KgExternalAdapterKind::ALL.len();
    let feature_enabled_count = reads
        .iter()
        .filter(|read| read.staging_config.feature_enabled)
        .count();
    let endpoint_configured_count = reads
        .iter()
        .filter(|read| read.staging_config.endpoint_configured)
        .count();
    let credentials_configured_count = reads
        .iter()
        .filter(|read| read.staging_config.credentials_configured)
        .count();
    let network_allowlisted_count = reads
        .iter()
        .filter(|read| read.staging_config.network_allowlisted)
        .count();
    let external_write_allowlisted_count = reads
        .iter()
        .filter(|read| read.staging_config.external_write_allowlisted)
        .count();
    let operator_approved_count = reads
        .iter()
        .filter(|read| read.staging_config.operator_review == KgOperatorReviewState::Approved)
        .count();
    let dry_run_sample_passed_count = reads
        .iter()
        .filter(|read| read.staging_config.dry_run_sample_passed)
        .count();
    let rollback_plan_ready_count = reads
        .iter()
        .filter(|read| read.staging_config.rollback_plan_ready)
        .count();
    let post_write_validation_ready_count = reads
        .iter()
        .filter(|read| read.staging_config.post_write_validation_ready)
        .count();
    let fully_configured_count = reads
        .iter()
        .filter(|read| adapter_staging_config_is_fully_configured(&read.staging_config))
        .count();
    let live_write_requested_count = reads
        .iter()
        .filter(|read| read.staging_config.live_write_requested)
        .count();
    let credential_value_captured_count = reads
        .iter()
        .filter(|read| read.credential_value_captured)
        .count();
    let network_call_attempted_count = reads
        .iter()
        .filter(|read| read.network_call_attempted)
        .count();
    let external_write_attempted_count = reads
        .iter()
        .filter(|read| read.external_write_attempted)
        .count();
    let live_write_attempted_count = reads
        .iter()
        .filter(|read| read.live_write_attempted)
        .count();
    let checks = MemoryKgAdapterConfigEnvChecks {
        all_supported_adapters_read: reads.len() == adapter_count
            && KgExternalAdapterKind::ALL
                .into_iter()
                .all(|adapter| reads.iter().any(|read| read.adapter == adapter)),
        all_env_keys_present_in_report: reads.iter().all(|read| {
            !read.keys.feature_gate.trim().is_empty()
                && !read.keys.endpoint.trim().is_empty()
                && !read.keys.credential_ref.trim().is_empty()
                && !read.keys.network_allowlist.trim().is_empty()
                && !read.keys.external_write_allowlist.trim().is_empty()
                && !read.keys.operator_review.trim().is_empty()
                && !read.keys.dry_run_sample_passed.trim().is_empty()
                && !read.keys.rollback_plan_ready.trim().is_empty()
                && !read.keys.post_write_validation_ready.trim().is_empty()
                && !read.keys.live_write_requested.trim().is_empty()
        }),
        all_configs_closed_by_default: reads
            .iter()
            .all(|read| adapter_staging_config_is_closed(&read.staging_config)),
        no_credential_values_captured: credential_value_captured_count == 0,
        no_network_calls_attempted: network_call_attempted_count == 0,
        no_external_writes_attempted: external_write_attempted_count == 0,
        no_live_writes_attempted: live_write_attempted_count == 0,
    };

    MemoryKgAdapterConfigEnvReport {
        product: "Hepta",
        command: "memory-kg-adapter-config-env",
        contract: hepta_kg::KG_EXTERNAL_ADAPTER_CONFIG_ENV_CONTRACT,
        status: if checks.ready() { "ready" } else { "attention" },
        sample_run,
        adapter_count,
        config_read_count: reads.len(),
        feature_enabled_count,
        endpoint_configured_count,
        credentials_configured_count,
        network_allowlisted_count,
        external_write_allowlisted_count,
        operator_approved_count,
        dry_run_sample_passed_count,
        rollback_plan_ready_count,
        post_write_validation_ready_count,
        fully_configured_count,
        live_write_requested_count,
        credential_value_captured_count,
        network_call_attempted_count,
        external_write_attempted_count,
        live_write_attempted_count,
        reads,
        checks,
        next_phase: "bind approved env snapshots to staged adapter clients without reading credential values",
    }
}

pub fn memory_kg_recall_plan_report(
    memory_units: &[MemoryUnit],
    sample_run: bool,
) -> MemoryKgRecallPlanReport {
    let candidates = kg_write_candidates_from_memory_units(
        memory_units,
        "hepta-intelligence",
        "memory-kg-recall",
    );
    let queries = memory_kg_recall_queries_for_candidates(&candidates);
    let plans = queries
        .iter()
        .map(|query| plan_kg_recall(query, &candidates))
        .collect::<Vec<_>>();

    let entity_match_count = plans.iter().map(|plan| plan.entity_match_count).sum();
    let relation_neighborhood_count = plans
        .iter()
        .map(|plan| plan.relation_neighborhood_count)
        .sum();
    let timeline_slice_count = plans.iter().map(|plan| plan.timeline_slice_count).sum();
    let evidence_path_count = plans.iter().map(|plan| plan.evidence_path_count).sum();
    let external_read_enabled_count = plans
        .iter()
        .filter(|plan| plan.external_read_allowed)
        .count();
    let network_call_enabled_count = plans
        .iter()
        .filter(|plan| plan.network_call_allowed)
        .count();
    let live_write_enabled_count = plans.iter().filter(|plan| plan.live_write_allowed).count();
    let checks = MemoryKgRecallPlanChecks {
        candidate_count_nonzero: !candidates.is_empty(),
        entity_matches_nonzero: entity_match_count > 0,
        relation_neighborhoods_nonzero: relation_neighborhood_count > 0,
        timeline_slices_nonzero: timeline_slice_count > 0,
        evidence_paths_nonzero: evidence_path_count > 0,
        all_plans_are_read_only: plans.iter().all(|plan| plan.read_only),
        no_external_reads_enabled: external_read_enabled_count == 0,
        no_network_calls_enabled: network_call_enabled_count == 0,
        no_live_writes_enabled: live_write_enabled_count == 0,
    };

    MemoryKgRecallPlanReport {
        product: "Hepta",
        command: "memory-kg-recall-plan",
        contract: hepta_kg::KG_READ_RECALL_CONTRACT,
        status: if checks.ready() { "ready" } else { "attention" },
        sample_run,
        query_count: queries.len(),
        candidate_count: candidates.len(),
        entity_match_count,
        relation_neighborhood_count,
        timeline_slice_count,
        evidence_path_count,
        external_read_enabled_count,
        network_call_enabled_count,
        live_write_enabled_count,
        plans,
        checks,
        next_phase: "bind reviewed read plans to local recall surfaces before enabling external adapter reads",
    }
}

pub fn memory_kg_context_recall_bridge_report(
    memory_units: &[MemoryUnit],
    sample_run: bool,
) -> MemoryKgContextRecallBridgeReport {
    let recall_report = memory_kg_recall_plan_report(memory_units, sample_run);
    let items = context_recall_items_from_kg_recall_plans(&recall_report.plans);
    let transcript_span_count = items
        .iter()
        .map(|item| item.source_transcript_spans.len())
        .sum();
    let checks = MemoryKgContextRecallBridgeChecks {
        recall_plan_ready: recall_report.checks.ready(),
        context_items_nonzero: !items.is_empty(),
        all_items_have_kg_source: items
            .iter()
            .all(|item| item.source == ContextRecallSource::KnowledgeGraph),
        all_items_have_scores: items.iter().all(context_recall_item_score_present),
        transcript_provenance_preserved: !items.is_empty()
            && items
                .iter()
                .all(|item| !item.source_transcript_spans.is_empty()),
        no_external_reads_enabled: recall_report.external_read_enabled_count == 0,
        no_network_calls_enabled: recall_report.network_call_enabled_count == 0,
        no_live_writes_enabled: recall_report.live_write_enabled_count == 0,
        no_model_invoked: true,
        no_context_injection_performed: true,
    };

    MemoryKgContextRecallBridgeReport {
        product: "Hepta",
        command: "memory-kg-context-recall-bridge",
        contract: MEMORY_KG_CONTEXT_RECALL_BRIDGE_V0_CONTRACT,
        status: if checks.ready() { "ready" } else { "attention" },
        sample_run,
        kg_recall_contract: hepta_kg::KG_READ_RECALL_CONTRACT,
        query_count: recall_report.query_count,
        kg_plan_count: recall_report.plans.len(),
        kg_evidence_path_count: recall_report.evidence_path_count,
        context_item_count: items.len(),
        transcript_span_count,
        external_read_enabled_count: recall_report.external_read_enabled_count,
        network_call_enabled_count: recall_report.network_call_enabled_count,
        live_write_enabled_count: recall_report.live_write_enabled_count,
        model_invoked: false,
        context_injection_performed: false,
        items,
        checks,
        next_phase: "rank KG-backed recall beside live transcript and durable-memory recall without prompt injection",
    }
}

pub fn memory_kg_recall_evaluation_report(
    memory_units: &[MemoryUnit],
    sample_run: bool,
) -> MemoryKgRecallEvaluationReport {
    let recall_report = memory_kg_recall_plan_report(memory_units, sample_run);
    let bridge_report = memory_kg_context_recall_bridge_report(memory_units, sample_run);
    let cases = memory_kg_recall_evaluation_cases(&recall_report.plans, &bridge_report.items);
    let evaluation_case_count = cases.len();
    let passed_case_count = cases.iter().filter(|case| case.passed).count();
    let failed_case_count = evaluation_case_count.saturating_sub(passed_case_count);
    let entity_evidence_case_count = cases
        .iter()
        .filter(|case| case.entity_evidence_count > 0)
        .count();
    let relation_path_case_count = cases
        .iter()
        .filter(|case| case.relation_path_count > 0)
        .count();
    let timeline_slice_case_count = cases
        .iter()
        .filter(|case| case.timeline_slice_count > 0)
        .count();
    let transcript_provenance_case_count = cases
        .iter()
        .filter(|case| case.transcript_span_count > 0)
        .count();
    let duplicate_context_source_id_count = cases
        .iter()
        .filter(|case| {
            case.blockers
                .contains(&MemoryKgRecallEvaluationBlocker::DuplicateContextSourceId)
        })
        .count();
    let duplicate_source_memory_id_count = cases
        .iter()
        .filter(|case| {
            case.blockers
                .contains(&MemoryKgRecallEvaluationBlocker::DuplicateSourceMemoryId)
        })
        .count();
    let score_order_violation_count = cases
        .iter()
        .filter(|case| {
            case.blockers
                .contains(&MemoryKgRecallEvaluationBlocker::ScoreOrderViolation)
        })
        .count();
    let coverage_basis_points = recall_quality_basis_points(
        entity_evidence_case_count
            + relation_path_case_count
            + timeline_slice_case_count
            + transcript_provenance_case_count,
        evaluation_case_count * 4,
    );
    let precision_proxy_basis_points =
        recall_quality_basis_points(passed_case_count, evaluation_case_count);
    let score_stability_basis_points = recall_quality_basis_points(
        evaluation_case_count.saturating_sub(score_order_violation_count),
        evaluation_case_count,
    );

    let checks = MemoryKgRecallEvaluationChecks {
        bridge_ready: bridge_report.checks.ready(),
        evaluation_cases_nonzero: evaluation_case_count > 0,
        all_cases_passed: evaluation_case_count > 0 && failed_case_count == 0,
        entity_evidence_complete: evaluation_case_count > 0
            && entity_evidence_case_count == evaluation_case_count,
        relation_path_complete: evaluation_case_count > 0
            && relation_path_case_count == evaluation_case_count,
        timeline_slice_complete: evaluation_case_count > 0
            && timeline_slice_case_count == evaluation_case_count,
        transcript_provenance_complete: evaluation_case_count > 0
            && transcript_provenance_case_count == evaluation_case_count,
        source_memory_ids_unique: duplicate_source_memory_id_count == 0
            && duplicate_context_source_id_count == 0,
        scores_stably_ordered: score_order_violation_count == 0,
        no_external_reads_enabled: bridge_report.external_read_enabled_count == 0,
        no_network_calls_enabled: bridge_report.network_call_enabled_count == 0,
        no_live_writes_enabled: bridge_report.live_write_enabled_count == 0,
        no_model_invoked: !bridge_report.model_invoked,
        no_context_injection_performed: !bridge_report.context_injection_performed,
    };

    MemoryKgRecallEvaluationReport {
        product: "Hepta",
        command: "memory-kg-recall-evaluation",
        contract: MEMORY_KG_RECALL_EVALUATION_V0_CONTRACT,
        status: if checks.ready() { "ready" } else { "attention" },
        sample_run,
        kg_recall_contract: hepta_kg::KG_READ_RECALL_CONTRACT,
        kg_context_bridge_contract: MEMORY_KG_CONTEXT_RECALL_BRIDGE_V0_CONTRACT,
        query_count: recall_report.query_count,
        context_item_count: bridge_report.context_item_count,
        evaluation_case_count,
        passed_case_count,
        failed_case_count,
        entity_evidence_case_count,
        relation_path_case_count,
        timeline_slice_case_count,
        transcript_provenance_case_count,
        duplicate_context_source_id_count,
        duplicate_source_memory_id_count,
        score_order_violation_count,
        coverage_basis_points,
        precision_proxy_basis_points,
        score_stability_basis_points,
        external_read_enabled_count: bridge_report.external_read_enabled_count,
        network_call_enabled_count: bridge_report.network_call_enabled_count,
        live_write_enabled_count: bridge_report.live_write_enabled_count,
        model_invoked: bridge_report.model_invoked,
        context_injection_performed: bridge_report.context_injection_performed,
        cases,
        checks,
        next_phase: "shadow-rank KG-backed recall beside transcript and durable-memory recall without prompt injection",
    }
}

fn memory_kg_recall_queries_for_candidates(candidates: &[KgWriteCandidate]) -> Vec<KgReadQuery> {
    let focus_label = candidates
        .iter()
        .flat_map(|candidate| candidate.entities.iter())
        .find(|entity| !entity.label.trim().is_empty())
        .map(|entity| entity.label.clone())
        .unwrap_or_else(|| "memory".to_string());

    vec![
        KgReadQuery {
            id: "kg-recall-query:focus-entity".to_string(),
            contract: hepta_kg::KG_READ_RECALL_CONTRACT.to_string(),
            query_text: focus_label.clone(),
            focus_entity_label: Some(focus_label),
            relation_kinds: Vec::new(),
            max_entities: 8,
            max_relations: 8,
            max_evidence_paths: 4,
        },
        KgReadQuery {
            id: "kg-recall-query:memory-neighborhood".to_string(),
            contract: hepta_kg::KG_READ_RECALL_CONTRACT.to_string(),
            query_text: "memory".to_string(),
            focus_entity_label: None,
            relation_kinds: vec![
                KgRelationKind::Mentions,
                KgRelationKind::RelatedTo,
                KgRelationKind::DerivedFrom,
                KgRelationKind::TriggeredBy,
                KgRelationKind::TemporalContinuation,
            ],
            max_entities: 12,
            max_relations: 12,
            max_evidence_paths: 6,
        },
    ]
}

fn context_recall_items_from_kg_recall_plans(plans: &[KgRecallPlan]) -> Vec<ContextRecallItem> {
    let mut items = plans
        .iter()
        .flat_map(context_recall_items_from_kg_recall_plan)
        .collect::<Vec<_>>();
    items.sort_by(|left, right| {
        right
            .score
            .final_score
            .total_cmp(&left.score.final_score)
            .then_with(|| left.source_id.cmp(&right.source_id))
    });
    let mut seen_source_memory_ids = BTreeSet::new();
    items.retain(|item| {
        item.source_memory_ids
            .first()
            .map(|source_memory_id| seen_source_memory_ids.insert(source_memory_id.clone()))
            .unwrap_or(true)
    });
    items.truncate(8);
    items
}

fn context_recall_items_from_kg_recall_plan(plan: &KgRecallPlan) -> Vec<ContextRecallItem> {
    plan.evidence_paths
        .iter()
        .enumerate()
        .filter_map(|(idx, path)| {
            let spans = path
                .source_spans
                .iter()
                .filter_map(kg_source_span_to_transcript_span_ref)
                .collect::<Vec<_>>();
            if spans.is_empty() {
                return None;
            }

            Some(ContextRecallItem {
                source: ContextRecallSource::KnowledgeGraph,
                source_id: kg_context_source_id(plan, path, idx),
                summary: kg_context_recall_summary(plan, path),
                score: kg_context_recall_score(plan, path, spans.len()),
                source_transcript_spans: spans,
                source_memory_ids: vec![path.candidate_id.clone()],
                topic_session_ids: Vec::new(),
                neuron_ids: Vec::new(),
            })
        })
        .collect()
}

fn kg_context_source_id(
    plan: &KgRecallPlan,
    path: &hepta_kg::KgEvidencePath,
    path_index: usize,
) -> String {
    format!(
        "kg-context:{}:{}:{}",
        plan.query_id, path.candidate_id, path_index
    )
}

fn memory_kg_recall_evaluation_cases(
    plans: &[KgRecallPlan],
    items: &[ContextRecallItem],
) -> Vec<MemoryKgRecallEvaluationCase> {
    let mut cases = Vec::new();
    let mut seen_context_source_ids = BTreeSet::new();
    let mut seen_source_memory_ids = BTreeSet::new();
    let mut previous_score = None;

    for item in items {
        let duplicate_context_source_id = !seen_context_source_ids.insert(item.source_id.clone());
        let primary_source_memory_id = item.source_memory_ids.first().cloned();
        let duplicate_source_memory_id = primary_source_memory_id
            .as_ref()
            .map(|source_memory_id| !seen_source_memory_ids.insert(source_memory_id.clone()))
            .unwrap_or(false);
        let score_order_violation = previous_score
            .map(|score| item.score.final_score > score + f32::EPSILON)
            .unwrap_or(false);
        previous_score = Some(item.score.final_score);

        let matched_path = find_kg_recall_plan_path_for_context_source_id(plans, &item.source_id);
        let (
            query_id,
            candidate_id,
            entity_evidence_count,
            relation_path_count,
            timeline_slice_count,
        ) = matched_path
            .map(|(plan, path)| {
                (
                    plan.query_id.clone(),
                    path.candidate_id.clone(),
                    path.entity_ids.len(),
                    path.relation_ids.len(),
                    plan.timeline_slices
                        .iter()
                        .filter(|slice| slice.episode_id == path.episode_id)
                        .count(),
                )
            })
            .unwrap_or_else(|| {
                (
                    "missing-query".to_string(),
                    primary_source_memory_id
                        .clone()
                        .unwrap_or_else(|| "missing-candidate".to_string()),
                    0,
                    0,
                    0,
                )
            });

        let transcript_span_count = item.source_transcript_spans.len();
        let source_memory_id_count = item.source_memory_ids.len();
        let mut blockers = Vec::new();
        if matched_path.is_none() {
            blockers.push(MemoryKgRecallEvaluationBlocker::MissingEvidencePath);
        }
        if entity_evidence_count == 0 {
            blockers.push(MemoryKgRecallEvaluationBlocker::MissingEntityEvidence);
        }
        if relation_path_count == 0 {
            blockers.push(MemoryKgRecallEvaluationBlocker::MissingRelationEvidence);
        }
        if timeline_slice_count == 0 {
            blockers.push(MemoryKgRecallEvaluationBlocker::MissingTimelineSlice);
        }
        if transcript_span_count == 0 {
            blockers.push(MemoryKgRecallEvaluationBlocker::MissingTranscriptProvenance);
        }
        if source_memory_id_count == 0 {
            blockers.push(MemoryKgRecallEvaluationBlocker::MissingSourceMemoryId);
        }
        if !context_recall_item_score_present(item) {
            blockers.push(MemoryKgRecallEvaluationBlocker::MissingScore);
        }
        if score_order_violation {
            blockers.push(MemoryKgRecallEvaluationBlocker::ScoreOrderViolation);
        }
        if duplicate_context_source_id {
            blockers.push(MemoryKgRecallEvaluationBlocker::DuplicateContextSourceId);
        }
        if duplicate_source_memory_id {
            blockers.push(MemoryKgRecallEvaluationBlocker::DuplicateSourceMemoryId);
        }

        cases.push(MemoryKgRecallEvaluationCase {
            query_id,
            candidate_id,
            context_source_id: item.source_id.clone(),
            entity_evidence_count,
            relation_path_count,
            timeline_slice_count,
            transcript_span_count,
            source_memory_id_count,
            final_score_basis_points: score_to_basis_points(item.score.final_score),
            relevance_basis_points: score_to_basis_points(item.score.relevance),
            durability_basis_points: score_to_basis_points(item.score.durability),
            confidence_basis_points: score_to_basis_points(item.score.confidence),
            passed: blockers.is_empty(),
            blockers,
        });
    }

    cases
}

fn find_kg_recall_plan_path_for_context_source_id<'a>(
    plans: &'a [KgRecallPlan],
    context_source_id: &str,
) -> Option<(&'a KgRecallPlan, &'a hepta_kg::KgEvidencePath)> {
    plans.iter().find_map(|plan| {
        plan.evidence_paths
            .iter()
            .enumerate()
            .find(|(idx, path)| kg_context_source_id(plan, path, *idx) == context_source_id)
            .map(|(_, path)| (plan, path))
    })
}

fn context_recall_item_score_present(item: &ContextRecallItem) -> bool {
    item.score.final_score > 0.0
        && item.score.relevance > 0.0
        && item.score.durability > 0.0
        && item.score.confidence > 0.0
}

fn score_to_basis_points(score: f32) -> u16 {
    (score.clamp(0.0, 1.0) * 10_000.0).round() as u16
}

fn recall_quality_basis_points(numerator: usize, denominator: usize) -> u16 {
    if denominator == 0 {
        return 0;
    }
    (((numerator as f32 / denominator as f32).clamp(0.0, 1.0)) * 10_000.0).round() as u16
}

fn kg_context_recall_summary(plan: &KgRecallPlan, path: &hepta_kg::KgEvidencePath) -> String {
    let first_entity = path
        .entity_ids
        .first()
        .map(String::as_str)
        .unwrap_or("no-entity");
    let relation_count = path.relation_ids.len();
    format!(
        "KG recall query {} matched {} with {} relation evidence path(s)",
        plan.query_id, first_entity, relation_count
    )
}

fn kg_context_recall_score(
    plan: &KgRecallPlan,
    path: &hepta_kg::KgEvidencePath,
    transcript_span_count: usize,
) -> ContextRecallScore {
    let entity_signal = capped_ratio(path.entity_ids.len(), 4);
    let relation_signal = capped_ratio(path.relation_ids.len(), 4);
    let evidence_signal = capped_ratio(transcript_span_count, 3);
    let relevance = (0.46 + entity_signal * 0.24 + relation_signal * 0.20).clamp(0.0, 1.0);
    let durability = (0.68 + capped_ratio(plan.candidate_count, 6) * 0.22).clamp(0.0, 1.0);
    let confidence = (0.62 + evidence_signal * 0.28).clamp(0.0, 1.0);
    let recency = 0.56;
    let topic_activation = 0.0;
    let neuron_activation = 0.0;
    let final_score = ((recency * 0.18)
        + (relevance * 0.34)
        + (durability * 0.22)
        + (topic_activation * 0.08)
        + (neuron_activation * 0.06)
        + (confidence * 0.12))
        .clamp(0.0, 1.0);

    ContextRecallScore {
        recency,
        relevance,
        durability,
        topic_activation,
        neuron_activation,
        confidence,
        final_score,
        reason: Some("knowledge graph recall bridge".to_string()),
    }
}

fn capped_ratio(value: usize, cap: usize) -> f32 {
    if cap == 0 {
        return 0.0;
    }
    (value.min(cap) as f32 / cap as f32).clamp(0.0, 1.0)
}

fn kg_source_span_to_transcript_span_ref(span: &KgSourceSpan) -> Option<TranscriptSpanRef> {
    if span.source_kind != KgSourceKind::Transcript {
        return None;
    }
    let session_id = span.uri.as_deref()?.trim();
    if session_id.is_empty() {
        return None;
    }
    let start_sequence = span.start_offset? as u64;
    let end_sequence = span.end_offset.unwrap_or(span.start_offset?) as u64;

    Some(TranscriptSpanRef {
        session_id: SessionId(session_id.to_string()),
        range: TranscriptRange {
            start_sequence,
            end_sequence,
        },
        reason: Some(format!("kg_context_recall_bridge:{}", span.source_id)),
    })
}

fn adapter_staging_config_is_closed(config: &KgExternalAdapterStagingConfig) -> bool {
    !config.feature_enabled
        && !config.endpoint_configured
        && !config.credentials_configured
        && !config.network_allowlisted
        && !config.external_write_allowlisted
        && config.operator_review == KgOperatorReviewState::NotReviewed
        && !config.dry_run_sample_passed
        && !config.rollback_plan_ready
        && !config.post_write_validation_ready
        && !config.live_write_requested
}

fn adapter_staging_config_is_fully_configured(config: &KgExternalAdapterStagingConfig) -> bool {
    config.feature_enabled
        && config.endpoint_configured
        && config.credentials_configured
        && config.network_allowlisted
        && config.external_write_allowlisted
        && config.operator_review == KgOperatorReviewState::Approved
        && config.dry_run_sample_passed
        && config.rollback_plan_ready
        && config.post_write_validation_ready
        && !config.live_write_requested
}

pub fn kg_write_candidates_from_memory_units(
    memory_units: &[MemoryUnit],
    producer: &str,
    batch_id: &str,
) -> Vec<KgWriteCandidate> {
    memory_units
        .iter()
        .map(|unit| kg_write_candidate_from_memory_unit(unit, producer, batch_id))
        .collect()
}

pub fn kg_write_candidate_from_memory_unit(
    unit: &MemoryUnit,
    producer: &str,
    batch_id: &str,
) -> KgWriteCandidate {
    let source_spans = unit
        .source_spans
        .iter()
        .map(kg_source_span_from_memory_source_span)
        .collect::<Vec<_>>();
    let unit_entity_id = memory_unit_entity_id(unit);
    let mut entities = vec![KgEntity {
        id: unit_entity_id.clone(),
        kind: kg_entity_kind_for_memory_unit_kind(unit.kind),
        label: summarize_label(&unit.content, 80),
        aliases: unit.labels.iter().cloned().collect(),
        source_spans: source_spans.clone(),
    }];

    entities.extend(unit.entity_ids.iter().map(|entity_id| KgEntity {
        id: entity_id.clone(),
        kind: KgEntityKind::Other,
        label: entity_id.clone(),
        aliases: Vec::new(),
        source_spans: source_spans.clone(),
    }));

    let mut relations = unit
        .entity_ids
        .iter()
        .map(|entity_id| KgRelation {
            id: format!("kg-rel:{}:mentions:{}", unit.id, entity_id),
            kind: KgRelationKind::Mentions,
            from_entity_id: unit_entity_id.clone(),
            to_entity_id: entity_id.clone(),
            confidence: confidence_from_ppm(unit.confidence_ppm),
            temporal: kg_temporal_validity_from_memory_unit(unit),
            source_spans: source_spans.clone(),
        })
        .collect::<Vec<_>>();

    relations.extend(
        unit.links
            .iter()
            .map(|link| kg_relation_from_memory_link(unit, link, &source_spans)),
    );
    relations.extend(
        unit.conflicts
            .iter()
            .map(|conflict| kg_relation_from_memory_conflict(unit, conflict, &source_spans)),
    );

    KgWriteCandidate {
        id: format!("kg-candidate:{}:{}", batch_id, unit.id),
        schema_version: hepta_kg::DEFAULT_KG_SCHEMA_VERSION.to_string(),
        episode: KgEpisode {
            id: format!("kg-episode:{}:{}", batch_id, unit.id),
            kind: kg_episode_kind_for_memory_unit_kind(unit.kind),
            summary: summarize_label(&unit.content, 160),
            occurred_at_unix_ms: Some(u64_to_i64_saturating(unit.created_at_unix_ms)),
            source_spans: source_spans.clone(),
        },
        entities,
        relations,
        provenance: KgProvenance {
            producer: producer.to_string(),
            schema_version: hepta_kg::DEFAULT_KG_SCHEMA_VERSION.to_string(),
            source_spans,
            redaction: KgRedactionState::NotReviewed,
            operator_review: KgOperatorReviewState::NotReviewed,
        },
        idempotency_key: Some(format!("kg-idempotency:{}:{}", batch_id, unit.id)),
    }
}

fn kg_relation_from_memory_link(
    unit: &MemoryUnit,
    link: &MemoryLink,
    source_spans: &[KgSourceSpan],
) -> KgRelation {
    KgRelation {
        id: format!("kg-rel:{}:{:?}:{}", unit.id, link.kind, link.target_id),
        kind: kg_relation_kind_for_memory_link_kind(link.kind),
        from_entity_id: memory_unit_entity_id(unit),
        to_entity_id: format!("memory:{}", link.target_id),
        confidence: confidence_from_ppm(link.weight_ppm),
        temporal: kg_temporal_validity_from_memory_unit(unit),
        source_spans: source_spans.to_vec(),
    }
}

fn kg_relation_from_memory_conflict(
    unit: &MemoryUnit,
    conflict: &MemoryConflict,
    source_spans: &[KgSourceSpan],
) -> KgRelation {
    KgRelation {
        id: format!("kg-rel:{}:conflicts:{}", unit.id, conflict.other_unit_id),
        kind: KgRelationKind::ConflictsWith,
        from_entity_id: memory_unit_entity_id(unit),
        to_entity_id: format!("memory:{}", conflict.other_unit_id),
        confidence: confidence_from_ppm(unit.confidence_ppm),
        temporal: kg_temporal_validity_from_memory_unit(unit),
        source_spans: source_spans.to_vec(),
    }
}

fn kg_source_span_from_memory_source_span(span: &MemorySourceSpan) -> KgSourceSpan {
    KgSourceSpan {
        source_id: span.source_id.clone(),
        source_kind: kg_source_kind_from_memory_source_kind(span.source_kind),
        uri: span.session_id.as_ref().map(|session| session.0.clone()),
        start_offset: span
            .transcript_range
            .as_ref()
            .map(|range| range.start_sequence as usize),
        end_offset: span
            .transcript_range
            .as_ref()
            .map(|range| range.end_sequence as usize),
        excerpt_hash: Some(span.evidence_digest.clone()),
    }
}

fn kg_source_kind_from_memory_source_kind(kind: MemorySourceKind) -> KgSourceKind {
    match kind {
        MemorySourceKind::Transcript => KgSourceKind::Transcript,
        MemorySourceKind::ToolCall => KgSourceKind::ToolResult,
        MemorySourceKind::ToolResult => KgSourceKind::ToolResult,
        MemorySourceKind::Approval => KgSourceKind::OperatorInput,
        MemorySourceKind::Summary => KgSourceKind::MemoryRecord,
        MemorySourceKind::ImportedMemory => KgSourceKind::MemoryRecord,
        MemorySourceKind::OperatorFeedback => KgSourceKind::OperatorInput,
        MemorySourceKind::SyntheticSample => KgSourceKind::Other,
    }
}

fn kg_entity_kind_for_memory_unit_kind(kind: MemoryUnitKind) -> KgEntityKind {
    match kind {
        MemoryUnitKind::Preference => KgEntityKind::Preference,
        MemoryUnitKind::TaskFact => KgEntityKind::Task,
        MemoryUnitKind::Decision => KgEntityKind::Decision,
        MemoryUnitKind::EntityFact => KgEntityKind::Memory,
        MemoryUnitKind::Procedural => KgEntityKind::Capability,
        MemoryUnitKind::Profile => KgEntityKind::Person,
        MemoryUnitKind::CoreBlock => KgEntityKind::Memory,
        MemoryUnitKind::Scenario => KgEntityKind::Task,
        MemoryUnitKind::Semantic
        | MemoryUnitKind::Episodic
        | MemoryUnitKind::TemporalFact
        | MemoryUnitKind::SymbolicContext => KgEntityKind::Memory,
    }
}

fn kg_episode_kind_for_memory_unit_kind(kind: MemoryUnitKind) -> KgEpisodeKind {
    match kind {
        MemoryUnitKind::Decision => KgEpisodeKind::OperatorDecision,
        MemoryUnitKind::TaskFact | MemoryUnitKind::Procedural | MemoryUnitKind::Scenario => {
            KgEpisodeKind::TaskResult
        }
        MemoryUnitKind::EntityFact
        | MemoryUnitKind::Semantic
        | MemoryUnitKind::Episodic
        | MemoryUnitKind::TemporalFact
        | MemoryUnitKind::Profile
        | MemoryUnitKind::Preference
        | MemoryUnitKind::CoreBlock
        | MemoryUnitKind::SymbolicContext => KgEpisodeKind::ConversationTurn,
    }
}

fn kg_relation_kind_for_memory_link_kind(kind: MemoryLinkKind) -> KgRelationKind {
    match kind {
        MemoryLinkKind::Evidence => KgRelationKind::DerivedFrom,
        MemoryLinkKind::SemanticSimilarity => KgRelationKind::RelatedTo,
        MemoryLinkKind::EntityOverlap => KgRelationKind::RelatedTo,
        MemoryLinkKind::WorkflowAdjacency => KgRelationKind::TriggeredBy,
        MemoryLinkKind::CausalDependency => KgRelationKind::Causal,
        MemoryLinkKind::TemporalContinuation => KgRelationKind::TemporalContinuation,
        MemoryLinkKind::Supersedes => KgRelationKind::Supersedes,
        MemoryLinkKind::ConflictsWith => KgRelationKind::ConflictsWith,
        MemoryLinkKind::Inhibits => KgRelationKind::ConflictsWith,
    }
}

fn kg_temporal_validity_from_memory_unit(unit: &MemoryUnit) -> KgTemporalValidity {
    KgTemporalValidity {
        observed_at_unix_ms: unit.validity.observed_at_unix_ms.map(u64_to_i64_saturating),
        valid_from_unix_ms: unit.validity.valid_from_unix_ms.map(u64_to_i64_saturating),
        valid_to_unix_ms: unit.validity.valid_until_unix_ms.map(u64_to_i64_saturating),
        superseded_by: None,
    }
}

fn memory_unit_entity_id(unit: &MemoryUnit) -> String {
    format!("memory:{}", unit.id)
}

fn confidence_from_ppm(ppm: u32) -> KgConfidence {
    KgConfidence::new((ppm / 100).min(10_000) as u16)
}

fn u64_to_i64_saturating(value: u64) -> i64 {
    value.min(i64::MAX as u64) as i64
}

fn summarize_label(content: &str, limit: usize) -> String {
    let trimmed = content.trim();
    if trimmed.chars().count() <= limit {
        trimmed.to_string()
    } else {
        let prefix = trimmed
            .chars()
            .take(limit.saturating_sub(3))
            .collect::<String>();
        format!("{prefix}...")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_atom_pipeline_sample_report;

    #[test]
    fn memory_atoms_emit_dry_run_kg_candidates_without_live_write() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_write_candidate_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.memory_unit_count, atom_report.atoms.len());
        assert_eq!(report.candidate_count, atom_report.atoms.len());
        assert_eq!(report.live_write_enabled_count, 0);
        assert_eq!(report.external_side_effect_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(
            report
                .plans
                .iter()
                .all(|plan| plan.mode == KgWriteMode::DryRun && !plan.live_write_allowed)
        );
    }

    #[test]
    fn candidate_keeps_memory_provenance_and_requires_review_by_default() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let candidate = kg_write_candidate_from_memory_unit(
            &atom_report.atoms[0],
            "hepta-intelligence",
            "test-batch",
        );
        let plan = plan_kg_write(&candidate, &KgWritePolicy::default());

        assert!(candidate.provenance.has_source_evidence());
        assert!(candidate.has_graph_payload());
        assert_eq!(
            candidate.provenance.redaction,
            KgRedactionState::NotReviewed
        );
        assert_eq!(
            candidate.provenance.operator_review,
            KgOperatorReviewState::NotReviewed
        );
        assert!(!plan.live_write_allowed);
    }

    #[test]
    fn memory_atoms_emit_external_adapter_dry_run_projections_without_side_effects() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_adapter_dry_run_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.candidate_count, atom_report.atoms.len());
        assert_eq!(report.adapter_count, 3);
        assert_eq!(report.projection_count, report.candidate_count * 3);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.external_write_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(
            report
                .projections
                .iter()
                .any(|plan| plan.adapter_id == "graphiti")
        );
        assert!(
            report
                .projections
                .iter()
                .any(|plan| plan.adapter_id == "neo4j")
        );
        assert!(
            report
                .projections
                .iter()
                .any(|plan| plan.adapter_id == "cocoindex")
        );
    }

    #[test]
    fn memory_atoms_emit_closed_adapter_staging_gates_by_default() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_adapter_staging_gate_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.candidate_count, atom_report.atoms.len());
        assert_eq!(report.adapter_count, 3);
        assert_eq!(report.staging_plan_count, report.candidate_count * 3);
        assert_eq!(report.staging_ready_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.external_write_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.operator_review_required);
        assert!(report.checks.rollback_plan_required);
        assert!(report.checks.post_write_validation_required);
        assert!(report.plans.iter().all(|plan| !plan.staging_ready));
    }

    #[test]
    fn memory_atoms_emit_disabled_adapter_client_denials_without_side_effects() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_adapter_client_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.candidate_count, atom_report.atoms.len());
        assert_eq!(report.adapter_count, 3);
        assert_eq!(report.client_audit_count, report.candidate_count * 3);
        assert_eq!(report.denied_client_count, report.client_audit_count);
        assert_eq!(report.network_call_attempted_count, 0);
        assert_eq!(report.external_write_attempted_count, 0);
        assert_eq!(report.live_write_attempted_count, 0);
        assert_eq!(report.persisted_record_count, 0);
        assert!(report.checks.ready());
        assert!(
            report
                .audits
                .iter()
                .any(|audit| audit.client_name == "disabled-graphiti-adapter-client")
        );
        assert!(
            report
                .audits
                .iter()
                .any(|audit| audit.client_name == "disabled-neo4j-adapter-client")
        );
        assert!(
            report
                .audits
                .iter()
                .any(|audit| audit.client_name == "disabled-cocoindex-adapter-client")
        );
    }

    #[test]
    fn adapter_config_env_report_reads_all_supported_adapters_closed_by_default() {
        let report = memory_kg_adapter_config_env_report(true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.adapter_count, 3);
        assert_eq!(report.config_read_count, 3);
        assert_eq!(report.feature_enabled_count, 0);
        assert_eq!(report.endpoint_configured_count, 0);
        assert_eq!(report.credentials_configured_count, 0);
        assert_eq!(report.fully_configured_count, 0);
        assert_eq!(report.live_write_requested_count, 0);
        assert_eq!(report.credential_value_captured_count, 0);
        assert_eq!(report.network_call_attempted_count, 0);
        assert_eq!(report.external_write_attempted_count, 0);
        assert_eq!(report.live_write_attempted_count, 0);
        assert!(report.checks.ready());
        assert!(
            report
                .reads
                .iter()
                .any(|read| read.keys.feature_gate == "HEPTA_KG_GRAPHITI_STAGING")
        );
    }

    #[test]
    fn adapter_config_env_report_can_show_reviewed_config_without_secret_capture() {
        let report = memory_kg_adapter_config_env_report_from_env_pairs(
            true,
            [
                ("HEPTA_KG_GRAPHITI_STAGING", "true"),
                ("HEPTA_KG_GRAPHITI_ENDPOINT", "https://graphiti.local"),
                ("HEPTA_KG_GRAPHITI_CREDENTIAL_REF", "op://hepta/kg/graphiti"),
                ("HEPTA_KG_GRAPHITI_NETWORK_ALLOWLIST", "true"),
                ("HEPTA_KG_GRAPHITI_EXTERNAL_WRITE_ALLOWLIST", "true"),
                ("HEPTA_KG_GRAPHITI_OPERATOR_REVIEW", "approved"),
                ("HEPTA_KG_GRAPHITI_DRY_RUN_SAMPLE_PASSED", "true"),
                ("HEPTA_KG_GRAPHITI_ROLLBACK_PLAN_READY", "true"),
                ("HEPTA_KG_GRAPHITI_POST_WRITE_VALIDATION_READY", "true"),
            ],
        );

        assert_eq!(report.status, "attention");
        assert_eq!(report.config_read_count, 3);
        assert_eq!(report.feature_enabled_count, 1);
        assert_eq!(report.endpoint_configured_count, 1);
        assert_eq!(report.credentials_configured_count, 1);
        assert_eq!(report.operator_approved_count, 1);
        assert_eq!(report.dry_run_sample_passed_count, 1);
        assert_eq!(report.rollback_plan_ready_count, 1);
        assert_eq!(report.post_write_validation_ready_count, 1);
        assert_eq!(report.fully_configured_count, 1);
        assert_eq!(report.credential_value_captured_count, 0);
        assert_eq!(report.network_call_attempted_count, 0);
        assert_eq!(report.external_write_attempted_count, 0);
        assert_eq!(report.live_write_attempted_count, 0);
        assert!(!report.checks.all_configs_closed_by_default);
        assert!(report.checks.no_credential_values_captured);
    }

    #[test]
    fn memory_atoms_emit_read_only_kg_recall_plans() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_recall_plan_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.query_count, 2);
        assert_eq!(report.candidate_count, atom_report.atoms.len());
        assert!(report.entity_match_count > 0);
        assert!(report.relation_neighborhood_count > 0);
        assert!(report.timeline_slice_count > 0);
        assert!(report.evidence_path_count > 0);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.plans.iter().all(|plan| plan.read_only));
        assert!(
            report
                .plans
                .iter()
                .all(|plan| !plan.external_read_allowed && !plan.network_call_allowed)
        );
    }

    #[test]
    fn memory_kg_recall_plans_keep_evidence_paths_without_writes() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_recall_plan_report(&atom_report.atoms, true);

        assert!(
            report
                .plans
                .iter()
                .flat_map(|plan| plan.evidence_paths.iter())
                .any(|path| !path.source_spans.is_empty())
        );
        assert!(report.checks.no_live_writes_enabled);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
    }

    #[test]
    fn memory_kg_context_recall_bridge_emits_ranked_items_without_side_effects() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_context_recall_bridge_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.kg_recall_contract, hepta_kg::KG_READ_RECALL_CONTRACT);
        assert_eq!(report.query_count, 2);
        assert!(report.kg_plan_count > 0);
        assert!(report.kg_evidence_path_count > 0);
        assert!(report.context_item_count > 0);
        assert!(report.transcript_span_count > 0);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert!(report.checks.ready());
        assert!(
            report
                .items
                .iter()
                .all(|item| item.source == ContextRecallSource::KnowledgeGraph)
        );
        assert!(report.items.iter().all(|item| item.score.final_score > 0.0));
        assert!(
            report
                .items
                .iter()
                .all(|item| !item.source_transcript_spans.is_empty())
        );
    }

    #[test]
    fn memory_kg_context_recall_bridge_preserves_transcript_span_reasons() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_context_recall_bridge_report(&atom_report.atoms, true);

        assert!(
            report
                .items
                .iter()
                .flat_map(|item| item.source_transcript_spans.iter())
                .any(|span| span
                    .reason
                    .as_deref()
                    .is_some_and(|reason| reason.starts_with("kg_context_recall_bridge:")))
        );
        assert!(report.checks.transcript_provenance_preserved);
        assert!(report.checks.no_context_injection_performed);
    }

    #[test]
    fn memory_kg_recall_evaluation_report_marks_quality_gate_ready_without_side_effects() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_recall_evaluation_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.contract, MEMORY_KG_RECALL_EVALUATION_V0_CONTRACT);
        assert_eq!(
            report.kg_context_bridge_contract,
            MEMORY_KG_CONTEXT_RECALL_BRIDGE_V0_CONTRACT
        );
        assert_eq!(report.kg_recall_contract, hepta_kg::KG_READ_RECALL_CONTRACT);
        assert_eq!(report.query_count, 2);
        assert!(report.evaluation_case_count > 0);
        assert_eq!(report.context_item_count, report.evaluation_case_count);
        assert_eq!(report.passed_case_count, report.evaluation_case_count);
        assert_eq!(report.failed_case_count, 0);
        assert_eq!(report.coverage_basis_points, 10_000);
        assert_eq!(report.precision_proxy_basis_points, 10_000);
        assert_eq!(report.score_stability_basis_points, 10_000);
        assert_eq!(report.duplicate_context_source_id_count, 0);
        assert_eq!(report.duplicate_source_memory_id_count, 0);
        assert_eq!(report.score_order_violation_count, 0);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert!(report.checks.ready());
        assert!(report.cases.iter().all(|case| case.passed));
        assert!(report.cases.iter().all(|case| case.blockers.is_empty()));
    }

    #[test]
    fn memory_kg_recall_evaluation_cases_are_stably_sorted_and_deduplicated() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_recall_evaluation_report(&atom_report.atoms, true);

        let mut seen_context_source_ids = BTreeSet::new();
        let mut seen_candidate_ids = BTreeSet::new();
        let mut previous_score = None;
        for case in &report.cases {
            assert!(case.context_source_id.starts_with("kg-context:"));
            assert!(seen_context_source_ids.insert(case.context_source_id.clone()));
            assert!(seen_candidate_ids.insert(case.candidate_id.clone()));
            if let Some(score) = previous_score {
                assert!(case.final_score_basis_points <= score);
            }
            previous_score = Some(case.final_score_basis_points);
            assert!(case.entity_evidence_count > 0);
            assert!(case.relation_path_count > 0);
            assert!(case.timeline_slice_count > 0);
            assert!(case.transcript_span_count > 0);
            assert!(case.source_memory_id_count > 0);
        }

        assert!(report.checks.source_memory_ids_unique);
        assert!(report.checks.scores_stably_ordered);
    }
}
