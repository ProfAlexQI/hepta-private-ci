use hepta_core::{
    MemoryConflict, MemoryLink, MemoryLinkKind, MemorySourceKind, MemorySourceSpan, MemoryUnit,
    MemoryUnitKind,
};
use hepta_kg::{
    KgConfidence, KgEntity, KgEntityKind, KgEpisode, KgEpisodeKind, KgExternalAdapterClientAudit,
    KgExternalAdapterClientBlocker, KgExternalAdapterClientRequest, KgExternalAdapterConfigEnvRead,
    KgExternalAdapterDryRunPlan, KgExternalAdapterKind, KgExternalAdapterStagingBlocker,
    KgExternalAdapterStagingConfig, KgExternalAdapterStagingPlan, KgOperatorReviewState,
    KgProvenance, KgRedactionState, KgRelation, KgRelationKind, KgSourceKind, KgSourceSpan,
    KgTemporalValidity, KgWriteCandidate, KgWriteMode, KgWritePlan, KgWritePolicy,
    default_external_adapter_staging_configs, plan_external_adapter_dry_run,
    plan_external_adapter_staging_gate, plan_kg_write, preview_disabled_external_adapter_write,
    read_all_external_adapter_staging_configs_from_env_pairs,
};
use serde::{Deserialize, Serialize};

pub const MEMORY_KG_WRITE_CANDIDATE_V0_CONTRACT: &str =
    "hepta-intelligence-memory-kg-write-candidate-v0";

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
}
