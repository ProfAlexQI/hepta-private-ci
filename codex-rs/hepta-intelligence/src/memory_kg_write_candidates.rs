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
pub const MEMORY_KG_CONTEXT_INJECTION_READINESS_V0_CONTRACT: &str =
    "hepta-intelligence-memory-kg-context-injection-readiness-v0";
pub const MEMORY_KG_SHADOW_RANK_V0_CONTRACT: &str = "hepta-intelligence-memory-kg-shadow-rank-v0";
pub const MEMORY_KG_SHADOW_RANK_COMPARISON_V0_CONTRACT: &str =
    "hepta-intelligence-memory-kg-shadow-rank-comparison-v0";
pub const MEMORY_KG_SHADOW_RANK_DRIFT_V0_CONTRACT: &str =
    "hepta-intelligence-memory-kg-shadow-rank-drift-v0";
pub const MEMORY_KG_PROMPT_PREVIEW_APPROVAL_PACKET_V0_CONTRACT: &str =
    "hepta-intelligence-memory-kg-prompt-preview-approval-packet-v0";
pub const MEMORY_KG_PROMPT_PREVIEW_OPERATOR_EVIDENCE_V0_CONTRACT: &str =
    "hepta-intelligence-memory-kg-prompt-preview-operator-evidence-v0";
pub const MEMORY_KG_PROMPT_PREVIEW_REDACTION_DIFF_V0_CONTRACT: &str =
    "hepta-intelligence-memory-kg-prompt-preview-redaction-diff-v0";
pub const MEMORY_KG_PROMPT_PREVIEW_ROLLBACK_KILL_SWITCH_V0_CONTRACT: &str =
    "hepta-intelligence-memory-kg-prompt-preview-rollback-kill-switch-v0";
pub const MEMORY_KG_PROMPT_PREVIEW_CONTEXT_HANDOFF_V0_CONTRACT: &str =
    "hepta-intelligence-memory-kg-prompt-preview-context-handoff-v0";
pub const MEMORY_KG_PROMPT_PREVIEW_PREFLIGHT_V0_CONTRACT: &str =
    "hepta-intelligence-memory-kg-prompt-preview-preflight-v0";

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryKgContextInjectionReadinessBlocker {
    QualityGateNotReady,
    QualityThresholdNotMet,
    MissingOperatorApproval,
    ShadowRankNotEnabled,
    MissingRollbackPlan,
    MissingKillSwitch,
    InjectionDisabledByDefault,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgContextInjectionReadinessChecks {
    pub recall_evaluation_ready: bool,
    pub quality_threshold_met: bool,
    pub operator_approval_required: bool,
    pub shadow_rank_required: bool,
    pub rollback_plan_required: bool,
    pub kill_switch_required: bool,
    pub injection_disabled_by_default: bool,
    pub activation_blocked_without_operator_approval: bool,
    pub prompt_preview_not_rendered: bool,
    pub no_external_reads_enabled: bool,
    pub no_network_calls_enabled: bool,
    pub no_live_writes_enabled: bool,
    pub no_model_invoked: bool,
    pub no_context_injection_performed: bool,
}

impl MemoryKgContextInjectionReadinessChecks {
    pub fn ready(&self) -> bool {
        self.recall_evaluation_ready
            && self.quality_threshold_met
            && self.operator_approval_required
            && self.shadow_rank_required
            && self.rollback_plan_required
            && self.kill_switch_required
            && self.injection_disabled_by_default
            && self.activation_blocked_without_operator_approval
            && self.prompt_preview_not_rendered
            && self.no_external_reads_enabled
            && self.no_network_calls_enabled
            && self.no_live_writes_enabled
            && self.no_model_invoked
            && self.no_context_injection_performed
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgContextInjectionReadinessReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub sample_run: bool,
    pub kg_recall_evaluation_contract: &'static str,
    pub kg_context_bridge_contract: &'static str,
    pub evaluation_case_count: usize,
    pub passed_case_count: usize,
    pub failed_case_count: usize,
    pub coverage_basis_points: u16,
    pub precision_proxy_basis_points: u16,
    pub score_stability_basis_points: u16,
    pub quality_threshold_basis_points: u16,
    pub quality_gate_ready: bool,
    pub operator_approved: bool,
    pub shadow_rank_enabled: bool,
    pub rollback_plan_ready: bool,
    pub kill_switch_ready: bool,
    pub context_injection_allowed: bool,
    pub context_injection_performed: bool,
    pub prompt_preview_rendered: bool,
    pub model_invoked: bool,
    pub external_read_enabled_count: usize,
    pub network_call_enabled_count: usize,
    pub live_write_enabled_count: usize,
    pub blockers: Vec<MemoryKgContextInjectionReadinessBlocker>,
    pub checks: MemoryKgContextInjectionReadinessChecks,
    pub next_phase: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgShadowRankItem {
    pub rank: usize,
    pub context_source_id: String,
    pub candidate_id: String,
    pub final_score_basis_points: u16,
    pub relevance_basis_points: u16,
    pub durability_basis_points: u16,
    pub confidence_basis_points: u16,
    pub transcript_span_count: usize,
    pub observed_only: bool,
    pub would_enter_prompt_context: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgShadowRankChecks {
    pub injection_readiness_blocked: bool,
    pub ranked_items_nonzero: bool,
    pub all_items_observed_only: bool,
    pub no_items_enter_prompt_context: bool,
    pub scores_stably_ordered: bool,
    pub no_prompt_preview_rendered: bool,
    pub no_model_invoked: bool,
    pub no_context_injection_performed: bool,
    pub no_external_reads_enabled: bool,
    pub no_network_calls_enabled: bool,
    pub no_live_writes_enabled: bool,
}

impl MemoryKgShadowRankChecks {
    pub fn ready(&self) -> bool {
        self.injection_readiness_blocked
            && self.ranked_items_nonzero
            && self.all_items_observed_only
            && self.no_items_enter_prompt_context
            && self.scores_stably_ordered
            && self.no_prompt_preview_rendered
            && self.no_model_invoked
            && self.no_context_injection_performed
            && self.no_external_reads_enabled
            && self.no_network_calls_enabled
            && self.no_live_writes_enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgShadowRankReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub sample_run: bool,
    pub kg_context_injection_readiness_contract: &'static str,
    pub kg_recall_evaluation_contract: &'static str,
    pub injection_readiness_status: &'static str,
    pub context_item_count: usize,
    pub ranked_item_count: usize,
    pub observed_only_count: usize,
    pub would_enter_prompt_context_count: usize,
    pub prompt_preview_rendered: bool,
    pub model_invoked: bool,
    pub context_injection_performed: bool,
    pub external_read_enabled_count: usize,
    pub network_call_enabled_count: usize,
    pub live_write_enabled_count: usize,
    pub items: Vec<MemoryKgShadowRankItem>,
    pub checks: MemoryKgShadowRankChecks,
    pub next_phase: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryKgShadowRankBaselineKind {
    Transcript,
    DurableMemory,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgShadowRankComparisonCase {
    pub kg_rank: usize,
    pub baseline_kind: MemoryKgShadowRankBaselineKind,
    pub baseline_rank: usize,
    pub kg_candidate_id: String,
    pub baseline_source_id: String,
    pub kg_score_basis_points: u16,
    pub baseline_score_basis_points: u16,
    pub kg_score_delta_basis_points: i16,
    pub kg_would_enter_prompt_context: bool,
    pub baseline_would_enter_prompt_context: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgShadowRankComparisonChecks {
    pub shadow_rank_ready: bool,
    pub baseline_items_nonzero: bool,
    pub comparison_cases_nonzero: bool,
    pub kg_items_observed_only: bool,
    pub no_kg_items_enter_prompt_context: bool,
    pub no_baseline_items_enter_prompt_context: bool,
    pub no_prompt_preview_rendered: bool,
    pub no_model_invoked: bool,
    pub no_context_injection_performed: bool,
    pub no_external_reads_enabled: bool,
    pub no_network_calls_enabled: bool,
    pub no_live_writes_enabled: bool,
}

impl MemoryKgShadowRankComparisonChecks {
    pub fn ready(&self) -> bool {
        self.shadow_rank_ready
            && self.baseline_items_nonzero
            && self.comparison_cases_nonzero
            && self.kg_items_observed_only
            && self.no_kg_items_enter_prompt_context
            && self.no_baseline_items_enter_prompt_context
            && self.no_prompt_preview_rendered
            && self.no_model_invoked
            && self.no_context_injection_performed
            && self.no_external_reads_enabled
            && self.no_network_calls_enabled
            && self.no_live_writes_enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgShadowRankComparisonReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub sample_run: bool,
    pub kg_shadow_rank_contract: &'static str,
    pub kg_context_injection_readiness_contract: &'static str,
    pub kg_ranked_item_count: usize,
    pub transcript_baseline_count: usize,
    pub durable_memory_baseline_count: usize,
    pub comparison_case_count: usize,
    pub kg_top_score_basis_points: u16,
    pub transcript_top_score_basis_points: u16,
    pub durable_memory_top_score_basis_points: u16,
    pub prompt_preview_rendered: bool,
    pub model_invoked: bool,
    pub context_injection_performed: bool,
    pub external_read_enabled_count: usize,
    pub network_call_enabled_count: usize,
    pub live_write_enabled_count: usize,
    pub cases: Vec<MemoryKgShadowRankComparisonCase>,
    pub checks: MemoryKgShadowRankComparisonChecks,
    pub next_phase: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgShadowRankDriftCase {
    pub case_index: usize,
    pub baseline_kind: MemoryKgShadowRankBaselineKind,
    pub kg_rank: usize,
    pub baseline_rank: usize,
    pub rank_delta: isize,
    pub kg_candidate_id: String,
    pub kg_score_delta_basis_points: i16,
    pub max_allowed_delta_basis_points: i16,
    pub rank_stable: bool,
    pub score_delta_within_threshold: bool,
    pub prompt_flags_stable: bool,
    pub stable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgShadowRankDriftChecks {
    pub comparison_ready: bool,
    pub top_n_cases_nonzero: bool,
    pub top_n_coverage_complete: bool,
    pub baseline_kind_coverage_stable: bool,
    pub rank_order_stable: bool,
    pub score_delta_within_thresholds: bool,
    pub prompt_flags_stable: bool,
    pub no_prompt_preview_rendered: bool,
    pub no_model_invoked: bool,
    pub no_context_injection_performed: bool,
    pub no_external_reads_enabled: bool,
    pub no_network_calls_enabled: bool,
    pub no_live_writes_enabled: bool,
}

impl MemoryKgShadowRankDriftChecks {
    pub fn ready(&self) -> bool {
        self.comparison_ready
            && self.top_n_cases_nonzero
            && self.top_n_coverage_complete
            && self.baseline_kind_coverage_stable
            && self.rank_order_stable
            && self.score_delta_within_thresholds
            && self.prompt_flags_stable
            && self.no_prompt_preview_rendered
            && self.no_model_invoked
            && self.no_context_injection_performed
            && self.no_external_reads_enabled
            && self.no_network_calls_enabled
            && self.no_live_writes_enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgShadowRankDriftReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub verdict: &'static str,
    pub sample_run: bool,
    pub kg_shadow_rank_comparison_contract: &'static str,
    pub kg_shadow_rank_contract: &'static str,
    pub top_n_limit: usize,
    pub kg_ranked_item_count: usize,
    pub top_n_kg_rank_count: usize,
    pub expected_drift_case_count: usize,
    pub drift_case_count: usize,
    pub stable_case_count: usize,
    pub drifted_case_count: usize,
    pub transcript_case_count: usize,
    pub durable_memory_case_count: usize,
    pub max_observed_score_delta_basis_points: i16,
    pub transcript_delta_threshold_basis_points: i16,
    pub durable_memory_delta_threshold_basis_points: i16,
    pub prompt_preview_rendered: bool,
    pub model_invoked: bool,
    pub context_injection_performed: bool,
    pub external_read_enabled_count: usize,
    pub network_call_enabled_count: usize,
    pub live_write_enabled_count: usize,
    pub cases: Vec<MemoryKgShadowRankDriftCase>,
    pub checks: MemoryKgShadowRankDriftChecks,
    pub next_phase: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryKgPromptPreviewApprovalPacketBlocker {
    DriftGateNotStable,
    MissingOperatorApproval,
    MissingRollbackPlan,
    MissingKillSwitch,
    PromptPreviewDisabledByDefault,
    ContextInjectionDisabledByDefault,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewApprovalPacketItem {
    pub packet_item_index: usize,
    pub kg_rank: usize,
    pub baseline_kind: MemoryKgShadowRankBaselineKind,
    pub kg_candidate_id: String,
    pub rank_delta: isize,
    pub score_delta_basis_points: i16,
    pub redacted_context_ref: String,
    pub prompt_preview_included: bool,
    pub context_injection_allowed: bool,
    pub operator_approval_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewApprovalPacketChecks {
    pub drift_gate_stable: bool,
    pub drift_cases_nonzero: bool,
    pub approval_items_nonzero: bool,
    pub approval_items_cover_drift_cases: bool,
    pub redacted_refs_present: bool,
    pub operator_approval_required: bool,
    pub rollback_plan_required: bool,
    pub kill_switch_required: bool,
    pub prompt_preview_disabled_by_default: bool,
    pub prompt_preview_not_rendered: bool,
    pub prompt_payload_not_materialized: bool,
    pub context_injection_disabled_by_default: bool,
    pub no_model_invoked: bool,
    pub no_context_injection_performed: bool,
    pub no_external_reads_enabled: bool,
    pub no_network_calls_enabled: bool,
    pub no_live_writes_enabled: bool,
}

impl MemoryKgPromptPreviewApprovalPacketChecks {
    pub fn ready(&self) -> bool {
        self.drift_gate_stable
            && self.drift_cases_nonzero
            && self.approval_items_nonzero
            && self.approval_items_cover_drift_cases
            && self.redacted_refs_present
            && self.operator_approval_required
            && self.rollback_plan_required
            && self.kill_switch_required
            && self.prompt_preview_disabled_by_default
            && self.prompt_preview_not_rendered
            && self.prompt_payload_not_materialized
            && self.context_injection_disabled_by_default
            && self.no_model_invoked
            && self.no_context_injection_performed
            && self.no_external_reads_enabled
            && self.no_network_calls_enabled
            && self.no_live_writes_enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewApprovalPacketReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub verdict: &'static str,
    pub sample_run: bool,
    pub kg_shadow_rank_drift_contract: &'static str,
    pub kg_context_injection_readiness_contract: &'static str,
    pub approval_packet_id: String,
    pub approval_packet_mode: &'static str,
    pub drift_case_count: usize,
    pub stable_case_count: usize,
    pub drifted_case_count: usize,
    pub approval_item_count: usize,
    pub redacted_context_ref_count: usize,
    pub operator_approval_recorded: bool,
    pub rollback_plan_ready: bool,
    pub kill_switch_ready: bool,
    pub approval_packet_accepted: bool,
    pub prompt_preview_allowed: bool,
    pub prompt_preview_rendered: bool,
    pub prompt_payload_materialized: bool,
    pub context_injection_allowed: bool,
    pub context_injection_performed: bool,
    pub model_invoked: bool,
    pub external_read_enabled_count: usize,
    pub network_call_enabled_count: usize,
    pub live_write_enabled_count: usize,
    pub blockers: Vec<MemoryKgPromptPreviewApprovalPacketBlocker>,
    pub items: Vec<MemoryKgPromptPreviewApprovalPacketItem>,
    pub checks: MemoryKgPromptPreviewApprovalPacketChecks,
    pub next_phase: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryKgPromptPreviewOperatorEvidenceBlocker {
    ApprovalPacketNotAccepted,
    MissingOperatorApprovalEvidence,
    MissingRollbackPlanEvidence,
    MissingKillSwitchEvidence,
    MissingReviewerIdentity,
    MissingApprovalTimestamp,
    MissingSignedApprovalDigest,
    MissingBoundedPreviewScope,
    PromptPreviewStillDisabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewOperatorEvidenceRequirement {
    pub requirement_index: usize,
    pub requirement: &'static str,
    pub present: bool,
    pub redacted_evidence_ref: String,
    pub blocks_prompt_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewOperatorEvidenceChecks {
    pub approval_packet_contract_linked: bool,
    pub approval_packet_checks_ready: bool,
    pub approval_packet_not_accepted: bool,
    pub evidence_requirements_nonzero: bool,
    pub evidence_requirements_all_blocking: bool,
    pub operator_approval_evidence_required: bool,
    pub rollback_plan_evidence_required: bool,
    pub kill_switch_evidence_required: bool,
    pub reviewer_identity_required: bool,
    pub approval_timestamp_required: bool,
    pub signed_approval_digest_required: bool,
    pub bounded_preview_scope_required: bool,
    pub prompt_preview_disabled: bool,
    pub prompt_payload_not_materialized: bool,
    pub context_injection_disabled: bool,
    pub no_model_invoked: bool,
    pub no_context_injection_performed: bool,
    pub no_external_reads_enabled: bool,
    pub no_network_calls_enabled: bool,
    pub no_live_writes_enabled: bool,
}

impl MemoryKgPromptPreviewOperatorEvidenceChecks {
    pub fn ready(&self) -> bool {
        self.approval_packet_contract_linked
            && self.approval_packet_checks_ready
            && self.approval_packet_not_accepted
            && self.evidence_requirements_nonzero
            && self.evidence_requirements_all_blocking
            && self.operator_approval_evidence_required
            && self.rollback_plan_evidence_required
            && self.kill_switch_evidence_required
            && self.reviewer_identity_required
            && self.approval_timestamp_required
            && self.signed_approval_digest_required
            && self.bounded_preview_scope_required
            && self.prompt_preview_disabled
            && self.prompt_payload_not_materialized
            && self.context_injection_disabled
            && self.no_model_invoked
            && self.no_context_injection_performed
            && self.no_external_reads_enabled
            && self.no_network_calls_enabled
            && self.no_live_writes_enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewOperatorEvidenceReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub verdict: &'static str,
    pub sample_run: bool,
    pub approval_packet_contract: &'static str,
    pub approval_packet_id: String,
    pub approval_packet_status: &'static str,
    pub evidence_gate_mode: &'static str,
    pub operator_approval_evidence_present: bool,
    pub rollback_plan_evidence_present: bool,
    pub kill_switch_evidence_present: bool,
    pub reviewer_identity_present: bool,
    pub reviewer_identity_redacted: bool,
    pub approval_timestamp_present: bool,
    pub signed_approval_digest_present: bool,
    pub bounded_preview_scope_present: bool,
    pub required_evidence_count: usize,
    pub missing_evidence_count: usize,
    pub prompt_preview_allowed: bool,
    pub prompt_preview_rendered: bool,
    pub prompt_payload_materialized: bool,
    pub context_injection_allowed: bool,
    pub context_injection_performed: bool,
    pub model_invoked: bool,
    pub external_read_enabled_count: usize,
    pub network_call_enabled_count: usize,
    pub live_write_enabled_count: usize,
    pub blockers: Vec<MemoryKgPromptPreviewOperatorEvidenceBlocker>,
    pub requirements: Vec<MemoryKgPromptPreviewOperatorEvidenceRequirement>,
    pub checks: MemoryKgPromptPreviewOperatorEvidenceChecks,
    pub next_phase: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryKgPromptPreviewRedactionDiffBlocker {
    OperatorEvidenceIncomplete,
    PromptPreviewDisabled,
    RawPromptDiffSuppressed,
    PromptPayloadMaterializationDisabled,
    ContextInjectionDisabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewRedactionDiffItem {
    pub diff_item_index: usize,
    pub requirement: &'static str,
    pub redacted_before_ref: String,
    pub redacted_after_ref: String,
    pub raw_before_included: bool,
    pub raw_after_included: bool,
    pub prompt_text_included: bool,
    pub payload_text_included: bool,
    pub operator_evidence_present: bool,
    pub blocks_prompt_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewRedactionDiffChecks {
    pub operator_evidence_contract_linked: bool,
    pub operator_evidence_checks_ready: bool,
    pub operator_evidence_missing_requirements: bool,
    pub redacted_diff_items_nonzero: bool,
    pub redacted_refs_present: bool,
    pub redacted_diff_items_cover_requirements: bool,
    pub raw_prompt_diff_suppressed: bool,
    pub prompt_text_excluded: bool,
    pub payload_text_excluded: bool,
    pub prompt_preview_disabled: bool,
    pub prompt_payload_not_materialized: bool,
    pub context_injection_disabled: bool,
    pub no_model_invoked: bool,
    pub no_context_injection_performed: bool,
    pub no_external_reads_enabled: bool,
    pub no_network_calls_enabled: bool,
    pub no_live_writes_enabled: bool,
}

impl MemoryKgPromptPreviewRedactionDiffChecks {
    pub fn ready(&self) -> bool {
        self.operator_evidence_contract_linked
            && self.operator_evidence_checks_ready
            && self.operator_evidence_missing_requirements
            && self.redacted_diff_items_nonzero
            && self.redacted_refs_present
            && self.redacted_diff_items_cover_requirements
            && self.raw_prompt_diff_suppressed
            && self.prompt_text_excluded
            && self.payload_text_excluded
            && self.prompt_preview_disabled
            && self.prompt_payload_not_materialized
            && self.context_injection_disabled
            && self.no_model_invoked
            && self.no_context_injection_performed
            && self.no_external_reads_enabled
            && self.no_network_calls_enabled
            && self.no_live_writes_enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewRedactionDiffReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub verdict: &'static str,
    pub sample_run: bool,
    pub operator_evidence_contract: &'static str,
    pub operator_evidence_status: &'static str,
    pub redaction_diff_mode: &'static str,
    pub required_evidence_count: usize,
    pub missing_evidence_count: usize,
    pub diff_item_count: usize,
    pub redacted_ref_count: usize,
    pub raw_prompt_diff_count: usize,
    pub prompt_text_included_count: usize,
    pub payload_text_included_count: usize,
    pub redacted_diff_reported: bool,
    pub prompt_preview_allowed: bool,
    pub prompt_preview_rendered: bool,
    pub prompt_payload_materialized: bool,
    pub context_injection_allowed: bool,
    pub context_injection_performed: bool,
    pub model_invoked: bool,
    pub external_read_enabled_count: usize,
    pub network_call_enabled_count: usize,
    pub live_write_enabled_count: usize,
    pub blockers: Vec<MemoryKgPromptPreviewRedactionDiffBlocker>,
    pub items: Vec<MemoryKgPromptPreviewRedactionDiffItem>,
    pub checks: MemoryKgPromptPreviewRedactionDiffChecks,
    pub next_phase: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryKgPromptPreviewRollbackKillSwitchBlocker {
    RedactionDiffNotReady,
    RollbackPlanEvidenceMissing,
    RollbackExerciseEvidenceMissing,
    KillSwitchEvidenceMissing,
    KillSwitchDryRunEvidenceMissing,
    PromptPreviewDisabled,
    ContextInjectionDisabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewRollbackKillSwitchControl {
    pub control_index: usize,
    pub control: &'static str,
    pub control_kind: &'static str,
    pub present: bool,
    pub redacted_evidence_ref: String,
    pub blocks_prompt_preview: bool,
    pub allows_context_injection: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewRollbackKillSwitchChecks {
    pub redaction_diff_contract_linked: bool,
    pub redaction_diff_checks_ready: bool,
    pub redaction_diff_blocked: bool,
    pub only_redacted_refs_reported: bool,
    pub rollback_controls_nonzero: bool,
    pub kill_switch_controls_nonzero: bool,
    pub controls_all_missing_and_blocking: bool,
    pub rollback_plan_required: bool,
    pub rollback_exercise_required: bool,
    pub kill_switch_required: bool,
    pub kill_switch_dry_run_required: bool,
    pub prompt_preview_disabled: bool,
    pub prompt_payload_not_materialized: bool,
    pub context_injection_disabled: bool,
    pub no_model_invoked: bool,
    pub no_context_injection_performed: bool,
    pub no_external_reads_enabled: bool,
    pub no_network_calls_enabled: bool,
    pub no_live_writes_enabled: bool,
}

impl MemoryKgPromptPreviewRollbackKillSwitchChecks {
    pub fn ready(&self) -> bool {
        self.redaction_diff_contract_linked
            && self.redaction_diff_checks_ready
            && self.redaction_diff_blocked
            && self.only_redacted_refs_reported
            && self.rollback_controls_nonzero
            && self.kill_switch_controls_nonzero
            && self.controls_all_missing_and_blocking
            && self.rollback_plan_required
            && self.rollback_exercise_required
            && self.kill_switch_required
            && self.kill_switch_dry_run_required
            && self.prompt_preview_disabled
            && self.prompt_payload_not_materialized
            && self.context_injection_disabled
            && self.no_model_invoked
            && self.no_context_injection_performed
            && self.no_external_reads_enabled
            && self.no_network_calls_enabled
            && self.no_live_writes_enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewRollbackKillSwitchReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub verdict: &'static str,
    pub sample_run: bool,
    pub redaction_diff_contract: &'static str,
    pub redaction_diff_status: &'static str,
    pub redaction_diff_mode: &'static str,
    pub required_evidence_count: usize,
    pub missing_evidence_count: usize,
    pub required_control_count: usize,
    pub missing_control_count: usize,
    pub rollback_control_count: usize,
    pub kill_switch_control_count: usize,
    pub rollback_plan_ready: bool,
    pub rollback_exercise_ready: bool,
    pub kill_switch_ready: bool,
    pub kill_switch_dry_run_ready: bool,
    pub redacted_ref_count: usize,
    pub raw_prompt_diff_count: usize,
    pub prompt_text_included_count: usize,
    pub payload_text_included_count: usize,
    pub prompt_preview_allowed: bool,
    pub prompt_preview_rendered: bool,
    pub prompt_payload_materialized: bool,
    pub context_injection_allowed: bool,
    pub context_injection_performed: bool,
    pub model_invoked: bool,
    pub external_read_enabled_count: usize,
    pub network_call_enabled_count: usize,
    pub live_write_enabled_count: usize,
    pub blockers: Vec<MemoryKgPromptPreviewRollbackKillSwitchBlocker>,
    pub controls: Vec<MemoryKgPromptPreviewRollbackKillSwitchControl>,
    pub checks: MemoryKgPromptPreviewRollbackKillSwitchChecks,
    pub next_phase: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryKgPromptPreviewContextHandoffBlocker {
    SafetyGateNotReady,
    OperatorEvidenceIncomplete,
    SafetyControlsIncomplete,
    RedactedDiffReviewMissing,
    ContextHandoffApprovalMissing,
    PromptPreviewDisabled,
    ContextInjectionDisabled,
    ModelInvocationDisabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewContextHandoffRequirement {
    pub requirement_index: usize,
    pub requirement: &'static str,
    pub requirement_kind: &'static str,
    pub present: bool,
    pub redacted_evidence_ref: String,
    pub blocks_context_injection: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewContextHandoffChecks {
    pub safety_gate_contract_linked: bool,
    pub safety_gate_checks_ready: bool,
    pub safety_gate_blocked: bool,
    pub operator_evidence_incomplete: bool,
    pub safety_controls_incomplete: bool,
    pub handoff_requirements_nonzero: bool,
    pub handoff_requirements_all_missing_and_blocking: bool,
    pub redacted_refs_only: bool,
    pub redacted_diff_review_required: bool,
    pub context_handoff_approval_required: bool,
    pub prompt_preview_disabled: bool,
    pub prompt_payload_not_materialized: bool,
    pub context_injection_disabled: bool,
    pub no_model_invoked: bool,
    pub no_context_injection_performed: bool,
    pub no_external_reads_enabled: bool,
    pub no_network_calls_enabled: bool,
    pub no_live_writes_enabled: bool,
}

impl MemoryKgPromptPreviewContextHandoffChecks {
    pub fn ready(&self) -> bool {
        self.safety_gate_contract_linked
            && self.safety_gate_checks_ready
            && self.safety_gate_blocked
            && self.operator_evidence_incomplete
            && self.safety_controls_incomplete
            && self.handoff_requirements_nonzero
            && self.handoff_requirements_all_missing_and_blocking
            && self.redacted_refs_only
            && self.redacted_diff_review_required
            && self.context_handoff_approval_required
            && self.prompt_preview_disabled
            && self.prompt_payload_not_materialized
            && self.context_injection_disabled
            && self.no_model_invoked
            && self.no_context_injection_performed
            && self.no_external_reads_enabled
            && self.no_network_calls_enabled
            && self.no_live_writes_enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewContextHandoffReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub verdict: &'static str,
    pub sample_run: bool,
    pub safety_gate_contract: &'static str,
    pub safety_gate_status: &'static str,
    pub redaction_diff_contract: &'static str,
    pub required_evidence_count: usize,
    pub missing_evidence_count: usize,
    pub required_control_count: usize,
    pub missing_control_count: usize,
    pub handoff_requirement_count: usize,
    pub missing_handoff_requirement_count: usize,
    pub redacted_ref_count: usize,
    pub raw_prompt_diff_count: usize,
    pub prompt_text_included_count: usize,
    pub payload_text_included_count: usize,
    pub redacted_diff_review_present: bool,
    pub context_handoff_approval_present: bool,
    pub prompt_preview_allowed: bool,
    pub prompt_preview_rendered: bool,
    pub prompt_payload_materialized: bool,
    pub context_injection_allowed: bool,
    pub context_injection_performed: bool,
    pub model_invoked: bool,
    pub external_read_enabled_count: usize,
    pub network_call_enabled_count: usize,
    pub live_write_enabled_count: usize,
    pub blockers: Vec<MemoryKgPromptPreviewContextHandoffBlocker>,
    pub requirements: Vec<MemoryKgPromptPreviewContextHandoffRequirement>,
    pub checks: MemoryKgPromptPreviewContextHandoffChecks,
    pub next_phase: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryKgPromptPreviewPreflightBlocker {
    PromptPreviewGateChainBlocked,
    OperatorEvidenceIncomplete,
    SafetyControlsIncomplete,
    HandoffRequirementsIncomplete,
    RedactedDiffReviewMissing,
    ContextHandoffApprovalMissing,
    PromptPreviewDisabled,
    ContextInjectionDisabled,
    ModelInvocationDisabled,
    CiPromotionDisabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewPreflightSourceGate {
    pub gate_index: usize,
    pub gate: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub checks_ready: bool,
    pub blocks_prompt_preview: bool,
    pub blocks_context_injection: bool,
    pub report_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewPreflightChecks {
    pub source_gates_nonzero: bool,
    pub source_gates_all_linked: bool,
    pub source_gates_all_checks_ready: bool,
    pub source_gates_all_blocked: bool,
    pub source_gates_all_report_only: bool,
    pub context_handoff_contract_linked: bool,
    pub context_handoff_checks_ready: bool,
    pub context_handoff_blocked: bool,
    pub operator_evidence_incomplete: bool,
    pub safety_controls_incomplete: bool,
    pub handoff_requirements_incomplete: bool,
    pub redacted_diff_review_required: bool,
    pub context_handoff_approval_required: bool,
    pub redacted_refs_only: bool,
    pub prompt_preview_disabled: bool,
    pub prompt_payload_not_materialized: bool,
    pub context_injection_disabled: bool,
    pub no_model_invoked: bool,
    pub no_context_injection_performed: bool,
    pub no_external_reads_enabled: bool,
    pub no_network_calls_enabled: bool,
    pub no_live_writes_enabled: bool,
    pub ci_promotion_disabled: bool,
    pub no_preflight_execution_performed: bool,
}

impl MemoryKgPromptPreviewPreflightChecks {
    pub fn ready(&self) -> bool {
        self.source_gates_nonzero
            && self.source_gates_all_linked
            && self.source_gates_all_checks_ready
            && self.source_gates_all_blocked
            && self.source_gates_all_report_only
            && self.context_handoff_contract_linked
            && self.context_handoff_checks_ready
            && self.context_handoff_blocked
            && self.operator_evidence_incomplete
            && self.safety_controls_incomplete
            && self.handoff_requirements_incomplete
            && self.redacted_diff_review_required
            && self.context_handoff_approval_required
            && self.redacted_refs_only
            && self.prompt_preview_disabled
            && self.prompt_payload_not_materialized
            && self.context_injection_disabled
            && self.no_model_invoked
            && self.no_context_injection_performed
            && self.no_external_reads_enabled
            && self.no_network_calls_enabled
            && self.no_live_writes_enabled
            && self.ci_promotion_disabled
            && self.no_preflight_execution_performed
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryKgPromptPreviewPreflightReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub verdict: &'static str,
    pub sample_run: bool,
    pub context_handoff_contract: &'static str,
    pub context_handoff_status: &'static str,
    pub source_gate_count: usize,
    pub ready_source_gate_count: usize,
    pub blocked_source_gate_count: usize,
    pub report_only_source_gate_count: usize,
    pub required_operator_evidence_count: usize,
    pub missing_operator_evidence_count: usize,
    pub required_safety_control_count: usize,
    pub missing_safety_control_count: usize,
    pub required_handoff_requirement_count: usize,
    pub missing_handoff_requirement_count: usize,
    pub missing_final_review_approval_count: usize,
    pub required_total_preflight_requirement_count: usize,
    pub missing_total_preflight_requirement_count: usize,
    pub redacted_ref_count: usize,
    pub raw_prompt_diff_count: usize,
    pub prompt_text_included_count: usize,
    pub payload_text_included_count: usize,
    pub redacted_diff_review_present: bool,
    pub context_handoff_approval_present: bool,
    pub prompt_preview_allowed: bool,
    pub prompt_preview_rendered: bool,
    pub prompt_payload_materialized: bool,
    pub context_injection_allowed: bool,
    pub context_injection_performed: bool,
    pub model_invoked: bool,
    pub ci_promotion_allowed: bool,
    pub preflight_execution_performed: bool,
    pub external_read_enabled_count: usize,
    pub network_call_enabled_count: usize,
    pub live_write_enabled_count: usize,
    pub blockers: Vec<MemoryKgPromptPreviewPreflightBlocker>,
    pub source_gates: Vec<MemoryKgPromptPreviewPreflightSourceGate>,
    pub checks: MemoryKgPromptPreviewPreflightChecks,
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

pub fn memory_kg_context_injection_readiness_report(
    memory_units: &[MemoryUnit],
    sample_run: bool,
) -> MemoryKgContextInjectionReadinessReport {
    const QUALITY_THRESHOLD_BASIS_POINTS: u16 = 9_000;

    let evaluation_report = memory_kg_recall_evaluation_report(memory_units, sample_run);
    let quality_gate_ready = evaluation_report.checks.ready();
    let quality_threshold_met = evaluation_report.coverage_basis_points
        >= QUALITY_THRESHOLD_BASIS_POINTS
        && evaluation_report.precision_proxy_basis_points >= QUALITY_THRESHOLD_BASIS_POINTS
        && evaluation_report.score_stability_basis_points >= QUALITY_THRESHOLD_BASIS_POINTS;

    let operator_approved = false;
    let shadow_rank_enabled = false;
    let rollback_plan_ready = false;
    let kill_switch_ready = false;
    let context_injection_allowed = quality_gate_ready
        && quality_threshold_met
        && operator_approved
        && shadow_rank_enabled
        && rollback_plan_ready
        && kill_switch_ready;
    let prompt_preview_rendered = false;
    let model_invoked = false;
    let context_injection_performed = false;

    let mut blockers = Vec::new();
    if !quality_gate_ready {
        blockers.push(MemoryKgContextInjectionReadinessBlocker::QualityGateNotReady);
    }
    if !quality_threshold_met {
        blockers.push(MemoryKgContextInjectionReadinessBlocker::QualityThresholdNotMet);
    }
    if !operator_approved {
        blockers.push(MemoryKgContextInjectionReadinessBlocker::MissingOperatorApproval);
    }
    if !shadow_rank_enabled {
        blockers.push(MemoryKgContextInjectionReadinessBlocker::ShadowRankNotEnabled);
    }
    if !rollback_plan_ready {
        blockers.push(MemoryKgContextInjectionReadinessBlocker::MissingRollbackPlan);
    }
    if !kill_switch_ready {
        blockers.push(MemoryKgContextInjectionReadinessBlocker::MissingKillSwitch);
    }
    if !context_injection_allowed {
        blockers.push(MemoryKgContextInjectionReadinessBlocker::InjectionDisabledByDefault);
    }

    let checks = MemoryKgContextInjectionReadinessChecks {
        recall_evaluation_ready: quality_gate_ready,
        quality_threshold_met,
        operator_approval_required: !operator_approved,
        shadow_rank_required: !shadow_rank_enabled,
        rollback_plan_required: !rollback_plan_ready,
        kill_switch_required: !kill_switch_ready,
        injection_disabled_by_default: !context_injection_allowed,
        activation_blocked_without_operator_approval: !context_injection_allowed
            && !operator_approved,
        prompt_preview_not_rendered: !prompt_preview_rendered,
        no_external_reads_enabled: evaluation_report.external_read_enabled_count == 0,
        no_network_calls_enabled: evaluation_report.network_call_enabled_count == 0,
        no_live_writes_enabled: evaluation_report.live_write_enabled_count == 0,
        no_model_invoked: !model_invoked,
        no_context_injection_performed: !context_injection_performed,
    };

    MemoryKgContextInjectionReadinessReport {
        product: "Hepta",
        command: "memory-kg-context-injection-readiness",
        contract: MEMORY_KG_CONTEXT_INJECTION_READINESS_V0_CONTRACT,
        status: if checks.ready() && !context_injection_allowed {
            "blocked"
        } else if checks.ready() && context_injection_allowed {
            "ready"
        } else {
            "attention"
        },
        sample_run,
        kg_recall_evaluation_contract: MEMORY_KG_RECALL_EVALUATION_V0_CONTRACT,
        kg_context_bridge_contract: MEMORY_KG_CONTEXT_RECALL_BRIDGE_V0_CONTRACT,
        evaluation_case_count: evaluation_report.evaluation_case_count,
        passed_case_count: evaluation_report.passed_case_count,
        failed_case_count: evaluation_report.failed_case_count,
        coverage_basis_points: evaluation_report.coverage_basis_points,
        precision_proxy_basis_points: evaluation_report.precision_proxy_basis_points,
        score_stability_basis_points: evaluation_report.score_stability_basis_points,
        quality_threshold_basis_points: QUALITY_THRESHOLD_BASIS_POINTS,
        quality_gate_ready,
        operator_approved,
        shadow_rank_enabled,
        rollback_plan_ready,
        kill_switch_ready,
        context_injection_allowed,
        context_injection_performed,
        prompt_preview_rendered,
        model_invoked,
        external_read_enabled_count: evaluation_report.external_read_enabled_count,
        network_call_enabled_count: evaluation_report.network_call_enabled_count,
        live_write_enabled_count: evaluation_report.live_write_enabled_count,
        blockers,
        checks,
        next_phase: "shadow-rank KG recall beside existing context sources until operator approval, rollback, and kill-switch gates are recorded",
    }
}

pub fn memory_kg_shadow_rank_report(
    memory_units: &[MemoryUnit],
    sample_run: bool,
) -> MemoryKgShadowRankReport {
    let readiness_report = memory_kg_context_injection_readiness_report(memory_units, sample_run);
    let bridge_report = memory_kg_context_recall_bridge_report(memory_units, sample_run);
    let items = memory_kg_shadow_rank_items(&bridge_report.items);
    let ranked_item_count = items.len();
    let observed_only_count = items.iter().filter(|item| item.observed_only).count();
    let would_enter_prompt_context_count = items
        .iter()
        .filter(|item| item.would_enter_prompt_context)
        .count();
    let prompt_preview_rendered = false;
    let model_invoked = false;
    let context_injection_performed = false;

    let checks = MemoryKgShadowRankChecks {
        injection_readiness_blocked: readiness_report.status == "blocked"
            && !readiness_report.context_injection_allowed,
        ranked_items_nonzero: ranked_item_count > 0,
        all_items_observed_only: ranked_item_count > 0 && observed_only_count == ranked_item_count,
        no_items_enter_prompt_context: would_enter_prompt_context_count == 0,
        scores_stably_ordered: shadow_rank_scores_stably_ordered(&items),
        no_prompt_preview_rendered: !prompt_preview_rendered,
        no_model_invoked: !model_invoked,
        no_context_injection_performed: !context_injection_performed,
        no_external_reads_enabled: readiness_report.external_read_enabled_count == 0,
        no_network_calls_enabled: readiness_report.network_call_enabled_count == 0,
        no_live_writes_enabled: readiness_report.live_write_enabled_count == 0,
    };

    MemoryKgShadowRankReport {
        product: "Hepta",
        command: "memory-kg-shadow-rank",
        contract: MEMORY_KG_SHADOW_RANK_V0_CONTRACT,
        status: if checks.ready() { "ready" } else { "attention" },
        sample_run,
        kg_context_injection_readiness_contract: MEMORY_KG_CONTEXT_INJECTION_READINESS_V0_CONTRACT,
        kg_recall_evaluation_contract: MEMORY_KG_RECALL_EVALUATION_V0_CONTRACT,
        injection_readiness_status: readiness_report.status,
        context_item_count: bridge_report.context_item_count,
        ranked_item_count,
        observed_only_count,
        would_enter_prompt_context_count,
        prompt_preview_rendered,
        model_invoked,
        context_injection_performed,
        external_read_enabled_count: readiness_report.external_read_enabled_count,
        network_call_enabled_count: readiness_report.network_call_enabled_count,
        live_write_enabled_count: readiness_report.live_write_enabled_count,
        items,
        checks,
        next_phase: "compare shadow KG rank against transcript and durable-memory rank before any operator-approved context injection",
    }
}

pub fn memory_kg_shadow_rank_comparison_report(
    memory_units: &[MemoryUnit],
    sample_run: bool,
) -> MemoryKgShadowRankComparisonReport {
    let shadow_rank_report = memory_kg_shadow_rank_report(memory_units, sample_run);
    let cases = memory_kg_shadow_rank_comparison_cases(&shadow_rank_report.items);
    let transcript_baseline_count = cases
        .iter()
        .filter(|case| case.baseline_kind == MemoryKgShadowRankBaselineKind::Transcript)
        .count();
    let durable_memory_baseline_count = cases
        .iter()
        .filter(|case| case.baseline_kind == MemoryKgShadowRankBaselineKind::DurableMemory)
        .count();
    let comparison_case_count = cases.len();
    let kg_top_score_basis_points = shadow_rank_report
        .items
        .first()
        .map(|item| item.final_score_basis_points)
        .unwrap_or_default();
    let transcript_top_score_basis_points = cases
        .iter()
        .filter(|case| case.baseline_kind == MemoryKgShadowRankBaselineKind::Transcript)
        .map(|case| case.baseline_score_basis_points)
        .max()
        .unwrap_or_default();
    let durable_memory_top_score_basis_points = cases
        .iter()
        .filter(|case| case.baseline_kind == MemoryKgShadowRankBaselineKind::DurableMemory)
        .map(|case| case.baseline_score_basis_points)
        .max()
        .unwrap_or_default();
    let prompt_preview_rendered = false;
    let model_invoked = false;
    let context_injection_performed = false;

    let checks = MemoryKgShadowRankComparisonChecks {
        shadow_rank_ready: shadow_rank_report.checks.ready(),
        baseline_items_nonzero: transcript_baseline_count > 0 && durable_memory_baseline_count > 0,
        comparison_cases_nonzero: comparison_case_count > 0,
        kg_items_observed_only: shadow_rank_report.ranked_item_count > 0
            && shadow_rank_report.observed_only_count == shadow_rank_report.ranked_item_count,
        no_kg_items_enter_prompt_context: shadow_rank_report.would_enter_prompt_context_count == 0,
        no_baseline_items_enter_prompt_context: cases
            .iter()
            .all(|case| !case.baseline_would_enter_prompt_context),
        no_prompt_preview_rendered: !prompt_preview_rendered,
        no_model_invoked: !model_invoked,
        no_context_injection_performed: !context_injection_performed,
        no_external_reads_enabled: shadow_rank_report.external_read_enabled_count == 0,
        no_network_calls_enabled: shadow_rank_report.network_call_enabled_count == 0,
        no_live_writes_enabled: shadow_rank_report.live_write_enabled_count == 0,
    };

    MemoryKgShadowRankComparisonReport {
        product: "Hepta",
        command: "memory-kg-shadow-rank-comparison",
        contract: MEMORY_KG_SHADOW_RANK_COMPARISON_V0_CONTRACT,
        status: if checks.ready() { "ready" } else { "attention" },
        sample_run,
        kg_shadow_rank_contract: MEMORY_KG_SHADOW_RANK_V0_CONTRACT,
        kg_context_injection_readiness_contract: MEMORY_KG_CONTEXT_INJECTION_READINESS_V0_CONTRACT,
        kg_ranked_item_count: shadow_rank_report.ranked_item_count,
        transcript_baseline_count,
        durable_memory_baseline_count,
        comparison_case_count,
        kg_top_score_basis_points,
        transcript_top_score_basis_points,
        durable_memory_top_score_basis_points,
        prompt_preview_rendered,
        model_invoked,
        context_injection_performed,
        external_read_enabled_count: shadow_rank_report.external_read_enabled_count,
        network_call_enabled_count: shadow_rank_report.network_call_enabled_count,
        live_write_enabled_count: shadow_rank_report.live_write_enabled_count,
        cases,
        checks,
        next_phase: "promote shadow-rank comparison into operator-reviewed prompt preview fixtures before any context injection",
    }
}

pub fn memory_kg_shadow_rank_drift_report(
    memory_units: &[MemoryUnit],
    sample_run: bool,
) -> MemoryKgShadowRankDriftReport {
    const TOP_N_LIMIT: usize = 6;
    const TRANSCRIPT_DELTA_THRESHOLD_BASIS_POINTS: i16 = 250;
    const DURABLE_MEMORY_DELTA_THRESHOLD_BASIS_POINTS: i16 = 500;

    let comparison_report = memory_kg_shadow_rank_comparison_report(memory_units, sample_run);
    let top_n_kg_rank_count = comparison_report.kg_ranked_item_count.min(TOP_N_LIMIT);
    let cases = memory_kg_shadow_rank_drift_cases(
        &comparison_report.cases,
        TOP_N_LIMIT,
        TRANSCRIPT_DELTA_THRESHOLD_BASIS_POINTS,
        DURABLE_MEMORY_DELTA_THRESHOLD_BASIS_POINTS,
    );
    let expected_drift_case_count = top_n_kg_rank_count * 2;
    let drift_case_count = cases.len();
    let stable_case_count = cases.iter().filter(|case| case.stable).count();
    let drifted_case_count = drift_case_count.saturating_sub(stable_case_count);
    let transcript_case_count = cases
        .iter()
        .filter(|case| case.baseline_kind == MemoryKgShadowRankBaselineKind::Transcript)
        .count();
    let durable_memory_case_count = cases
        .iter()
        .filter(|case| case.baseline_kind == MemoryKgShadowRankBaselineKind::DurableMemory)
        .count();
    let max_observed_score_delta_basis_points = cases
        .iter()
        .map(|case| case.kg_score_delta_basis_points)
        .max()
        .unwrap_or_default();
    let prompt_preview_rendered = false;
    let model_invoked = false;
    let context_injection_performed = false;

    let checks = MemoryKgShadowRankDriftChecks {
        comparison_ready: comparison_report.checks.ready(),
        top_n_cases_nonzero: drift_case_count > 0,
        top_n_coverage_complete: expected_drift_case_count > 0
            && drift_case_count == expected_drift_case_count,
        baseline_kind_coverage_stable: transcript_case_count == top_n_kg_rank_count
            && durable_memory_case_count == top_n_kg_rank_count,
        rank_order_stable: drift_case_count > 0 && cases.iter().all(|case| case.rank_stable),
        score_delta_within_thresholds: drift_case_count > 0
            && cases.iter().all(|case| case.score_delta_within_threshold),
        prompt_flags_stable: drift_case_count > 0
            && cases.iter().all(|case| case.prompt_flags_stable),
        no_prompt_preview_rendered: !prompt_preview_rendered,
        no_model_invoked: !model_invoked,
        no_context_injection_performed: !context_injection_performed,
        no_external_reads_enabled: comparison_report.external_read_enabled_count == 0,
        no_network_calls_enabled: comparison_report.network_call_enabled_count == 0,
        no_live_writes_enabled: comparison_report.live_write_enabled_count == 0,
    };

    MemoryKgShadowRankDriftReport {
        product: "Hepta",
        command: "memory-kg-shadow-rank-drift",
        contract: MEMORY_KG_SHADOW_RANK_DRIFT_V0_CONTRACT,
        status: if checks.ready() { "ready" } else { "attention" },
        verdict: if checks.ready() {
            "stable"
        } else {
            "drift_detected"
        },
        sample_run,
        kg_shadow_rank_comparison_contract: MEMORY_KG_SHADOW_RANK_COMPARISON_V0_CONTRACT,
        kg_shadow_rank_contract: MEMORY_KG_SHADOW_RANK_V0_CONTRACT,
        top_n_limit: TOP_N_LIMIT,
        kg_ranked_item_count: comparison_report.kg_ranked_item_count,
        top_n_kg_rank_count,
        expected_drift_case_count,
        drift_case_count,
        stable_case_count,
        drifted_case_count,
        transcript_case_count,
        durable_memory_case_count,
        max_observed_score_delta_basis_points,
        transcript_delta_threshold_basis_points: TRANSCRIPT_DELTA_THRESHOLD_BASIS_POINTS,
        durable_memory_delta_threshold_basis_points: DURABLE_MEMORY_DELTA_THRESHOLD_BASIS_POINTS,
        prompt_preview_rendered,
        model_invoked,
        context_injection_performed,
        external_read_enabled_count: comparison_report.external_read_enabled_count,
        network_call_enabled_count: comparison_report.network_call_enabled_count,
        live_write_enabled_count: comparison_report.live_write_enabled_count,
        cases,
        checks,
        next_phase: "bind stable shadow-rank drift evidence to operator-reviewed prompt-preview fixtures before any context injection",
    }
}

pub fn memory_kg_prompt_preview_approval_packet_report(
    memory_units: &[MemoryUnit],
    sample_run: bool,
) -> MemoryKgPromptPreviewApprovalPacketReport {
    let drift_report = memory_kg_shadow_rank_drift_report(memory_units, sample_run);
    let readiness_report = memory_kg_context_injection_readiness_report(memory_units, sample_run);
    let items = memory_kg_prompt_preview_approval_packet_items(&drift_report.cases);
    let approval_item_count = items.len();
    let redacted_context_ref_count = items
        .iter()
        .filter(|item| !item.redacted_context_ref.trim().is_empty())
        .count();
    let operator_approval_recorded = false;
    let rollback_plan_ready = false;
    let kill_switch_ready = false;
    let approval_packet_accepted = false;
    let prompt_preview_allowed = false;
    let prompt_preview_rendered = false;
    let prompt_payload_materialized = false;
    let context_injection_allowed = false;
    let context_injection_performed = false;
    let model_invoked = false;

    let drift_gate_stable = drift_report.checks.ready()
        && drift_report.verdict == "stable"
        && drift_report.drifted_case_count == 0;
    let mut blockers = Vec::new();
    if !drift_gate_stable {
        blockers.push(MemoryKgPromptPreviewApprovalPacketBlocker::DriftGateNotStable);
    }
    if !operator_approval_recorded {
        blockers.push(MemoryKgPromptPreviewApprovalPacketBlocker::MissingOperatorApproval);
    }
    if !rollback_plan_ready {
        blockers.push(MemoryKgPromptPreviewApprovalPacketBlocker::MissingRollbackPlan);
    }
    if !kill_switch_ready {
        blockers.push(MemoryKgPromptPreviewApprovalPacketBlocker::MissingKillSwitch);
    }
    if !prompt_preview_allowed {
        blockers.push(MemoryKgPromptPreviewApprovalPacketBlocker::PromptPreviewDisabledByDefault);
    }
    if !context_injection_allowed {
        blockers
            .push(MemoryKgPromptPreviewApprovalPacketBlocker::ContextInjectionDisabledByDefault);
    }

    let checks = MemoryKgPromptPreviewApprovalPacketChecks {
        drift_gate_stable,
        drift_cases_nonzero: drift_report.drift_case_count > 0,
        approval_items_nonzero: approval_item_count > 0,
        approval_items_cover_drift_cases: approval_item_count == drift_report.drift_case_count,
        redacted_refs_present: approval_item_count > 0
            && redacted_context_ref_count == approval_item_count,
        operator_approval_required: !operator_approval_recorded,
        rollback_plan_required: !rollback_plan_ready,
        kill_switch_required: !kill_switch_ready,
        prompt_preview_disabled_by_default: !prompt_preview_allowed,
        prompt_preview_not_rendered: !prompt_preview_rendered,
        prompt_payload_not_materialized: !prompt_payload_materialized,
        context_injection_disabled_by_default: !context_injection_allowed,
        no_model_invoked: !model_invoked,
        no_context_injection_performed: !context_injection_performed,
        no_external_reads_enabled: drift_report.external_read_enabled_count == 0
            && readiness_report.external_read_enabled_count == 0,
        no_network_calls_enabled: drift_report.network_call_enabled_count == 0
            && readiness_report.network_call_enabled_count == 0,
        no_live_writes_enabled: drift_report.live_write_enabled_count == 0
            && readiness_report.live_write_enabled_count == 0,
    };

    MemoryKgPromptPreviewApprovalPacketReport {
        product: "Hepta",
        command: "memory-kg-prompt-preview-approval-packet",
        contract: MEMORY_KG_PROMPT_PREVIEW_APPROVAL_PACKET_V0_CONTRACT,
        status: if checks.ready() && !approval_packet_accepted {
            "blocked"
        } else if checks.ready() && approval_packet_accepted {
            "ready"
        } else {
            "attention"
        },
        verdict: if approval_packet_accepted {
            "accepted"
        } else {
            "blocked_until_operator_prompt_preview_approval_rollback_and_kill_switch"
        },
        sample_run,
        kg_shadow_rank_drift_contract: MEMORY_KG_SHADOW_RANK_DRIFT_V0_CONTRACT,
        kg_context_injection_readiness_contract: MEMORY_KG_CONTEXT_INJECTION_READINESS_V0_CONTRACT,
        approval_packet_id: format!(
            "kg-prompt-preview-approval:{}:{}:{}",
            drift_report.top_n_kg_rank_count,
            drift_report.drift_case_count,
            redacted_context_ref_count
        ),
        approval_packet_mode: "draft_redacted_refs_only_no_prompt_preview",
        drift_case_count: drift_report.drift_case_count,
        stable_case_count: drift_report.stable_case_count,
        drifted_case_count: drift_report.drifted_case_count,
        approval_item_count,
        redacted_context_ref_count,
        operator_approval_recorded,
        rollback_plan_ready,
        kill_switch_ready,
        approval_packet_accepted,
        prompt_preview_allowed,
        prompt_preview_rendered,
        prompt_payload_materialized,
        context_injection_allowed,
        context_injection_performed,
        model_invoked,
        external_read_enabled_count: drift_report.external_read_enabled_count,
        network_call_enabled_count: drift_report.network_call_enabled_count,
        live_write_enabled_count: drift_report.live_write_enabled_count,
        blockers,
        items,
        checks,
        next_phase: "require explicit operator approval, rollback, and kill-switch evidence before rendering any KG prompt preview",
    }
}

pub fn memory_kg_prompt_preview_operator_evidence_report(
    memory_units: &[MemoryUnit],
    sample_run: bool,
) -> MemoryKgPromptPreviewOperatorEvidenceReport {
    let approval_report = memory_kg_prompt_preview_approval_packet_report(memory_units, sample_run);
    let operator_approval_evidence_present = false;
    let rollback_plan_evidence_present = false;
    let kill_switch_evidence_present = false;
    let reviewer_identity_present = false;
    let reviewer_identity_redacted = true;
    let approval_timestamp_present = false;
    let signed_approval_digest_present = false;
    let bounded_preview_scope_present = false;
    let prompt_preview_allowed = false;
    let prompt_preview_rendered = false;
    let prompt_payload_materialized = false;
    let context_injection_allowed = false;
    let context_injection_performed = false;
    let model_invoked = false;
    let requirements = memory_kg_prompt_preview_operator_evidence_requirements();
    let required_evidence_count = requirements.len();
    let missing_evidence_count = requirements
        .iter()
        .filter(|requirement| !requirement.present)
        .count();

    let mut blockers = Vec::new();
    if !approval_report.approval_packet_accepted {
        blockers.push(MemoryKgPromptPreviewOperatorEvidenceBlocker::ApprovalPacketNotAccepted);
    }
    if !operator_approval_evidence_present {
        blockers
            .push(MemoryKgPromptPreviewOperatorEvidenceBlocker::MissingOperatorApprovalEvidence);
    }
    if !rollback_plan_evidence_present {
        blockers.push(MemoryKgPromptPreviewOperatorEvidenceBlocker::MissingRollbackPlanEvidence);
    }
    if !kill_switch_evidence_present {
        blockers.push(MemoryKgPromptPreviewOperatorEvidenceBlocker::MissingKillSwitchEvidence);
    }
    if !reviewer_identity_present {
        blockers.push(MemoryKgPromptPreviewOperatorEvidenceBlocker::MissingReviewerIdentity);
    }
    if !approval_timestamp_present {
        blockers.push(MemoryKgPromptPreviewOperatorEvidenceBlocker::MissingApprovalTimestamp);
    }
    if !signed_approval_digest_present {
        blockers.push(MemoryKgPromptPreviewOperatorEvidenceBlocker::MissingSignedApprovalDigest);
    }
    if !bounded_preview_scope_present {
        blockers.push(MemoryKgPromptPreviewOperatorEvidenceBlocker::MissingBoundedPreviewScope);
    }
    if !prompt_preview_allowed {
        blockers.push(MemoryKgPromptPreviewOperatorEvidenceBlocker::PromptPreviewStillDisabled);
    }

    let checks = MemoryKgPromptPreviewOperatorEvidenceChecks {
        approval_packet_contract_linked: approval_report.contract
            == MEMORY_KG_PROMPT_PREVIEW_APPROVAL_PACKET_V0_CONTRACT,
        approval_packet_checks_ready: approval_report.checks.ready(),
        approval_packet_not_accepted: !approval_report.approval_packet_accepted,
        evidence_requirements_nonzero: required_evidence_count > 0,
        evidence_requirements_all_blocking: required_evidence_count > 0
            && requirements
                .iter()
                .all(|requirement| requirement.blocks_prompt_preview),
        operator_approval_evidence_required: !operator_approval_evidence_present,
        rollback_plan_evidence_required: !rollback_plan_evidence_present,
        kill_switch_evidence_required: !kill_switch_evidence_present,
        reviewer_identity_required: !reviewer_identity_present && reviewer_identity_redacted,
        approval_timestamp_required: !approval_timestamp_present,
        signed_approval_digest_required: !signed_approval_digest_present,
        bounded_preview_scope_required: !bounded_preview_scope_present,
        prompt_preview_disabled: !prompt_preview_allowed && !prompt_preview_rendered,
        prompt_payload_not_materialized: !prompt_payload_materialized,
        context_injection_disabled: !context_injection_allowed,
        no_model_invoked: !model_invoked,
        no_context_injection_performed: !context_injection_performed,
        no_external_reads_enabled: approval_report.external_read_enabled_count == 0,
        no_network_calls_enabled: approval_report.network_call_enabled_count == 0,
        no_live_writes_enabled: approval_report.live_write_enabled_count == 0,
    };

    MemoryKgPromptPreviewOperatorEvidenceReport {
        product: "Hepta",
        command: "memory-kg-prompt-preview-operator-evidence",
        contract: MEMORY_KG_PROMPT_PREVIEW_OPERATOR_EVIDENCE_V0_CONTRACT,
        status: if checks.ready() {
            "blocked"
        } else {
            "attention"
        },
        verdict: "blocked_until_operator_evidence_packet_is_complete_and_signed",
        sample_run,
        approval_packet_contract: MEMORY_KG_PROMPT_PREVIEW_APPROVAL_PACKET_V0_CONTRACT,
        approval_packet_id: approval_report.approval_packet_id,
        approval_packet_status: approval_report.status,
        evidence_gate_mode: "operator_evidence_requirements_only_no_prompt_preview",
        operator_approval_evidence_present,
        rollback_plan_evidence_present,
        kill_switch_evidence_present,
        reviewer_identity_present,
        reviewer_identity_redacted,
        approval_timestamp_present,
        signed_approval_digest_present,
        bounded_preview_scope_present,
        required_evidence_count,
        missing_evidence_count,
        prompt_preview_allowed,
        prompt_preview_rendered,
        prompt_payload_materialized,
        context_injection_allowed,
        context_injection_performed,
        model_invoked,
        external_read_enabled_count: approval_report.external_read_enabled_count,
        network_call_enabled_count: approval_report.network_call_enabled_count,
        live_write_enabled_count: approval_report.live_write_enabled_count,
        blockers,
        requirements,
        checks,
        next_phase: "wire operator-provided approval, rollback, kill-switch, reviewer, timestamp, digest, and bounded-scope evidence before enabling any KG prompt preview",
    }
}

pub fn memory_kg_prompt_preview_redaction_diff_report(
    memory_units: &[MemoryUnit],
    sample_run: bool,
) -> MemoryKgPromptPreviewRedactionDiffReport {
    let evidence_report =
        memory_kg_prompt_preview_operator_evidence_report(memory_units, sample_run);
    let items = memory_kg_prompt_preview_redaction_diff_items(&evidence_report.requirements);
    let diff_item_count = items.len();
    let redacted_ref_count = items
        .iter()
        .filter(|item| {
            !item.redacted_before_ref.trim().is_empty()
                && !item.redacted_after_ref.trim().is_empty()
        })
        .count();
    let raw_prompt_diff_count = items
        .iter()
        .filter(|item| item.raw_before_included || item.raw_after_included)
        .count();
    let prompt_text_included_count = items
        .iter()
        .filter(|item| item.prompt_text_included)
        .count();
    let payload_text_included_count = items
        .iter()
        .filter(|item| item.payload_text_included)
        .count();
    let redacted_diff_reported = true;
    let prompt_preview_allowed = false;
    let prompt_preview_rendered = false;
    let prompt_payload_materialized = false;
    let context_injection_allowed = false;
    let context_injection_performed = false;
    let model_invoked = false;

    let mut blockers = Vec::new();
    if evidence_report.missing_evidence_count > 0 {
        blockers.push(MemoryKgPromptPreviewRedactionDiffBlocker::OperatorEvidenceIncomplete);
    }
    if !prompt_preview_allowed {
        blockers.push(MemoryKgPromptPreviewRedactionDiffBlocker::PromptPreviewDisabled);
    }
    if raw_prompt_diff_count == 0 {
        blockers.push(MemoryKgPromptPreviewRedactionDiffBlocker::RawPromptDiffSuppressed);
    }
    if !prompt_payload_materialized {
        blockers
            .push(MemoryKgPromptPreviewRedactionDiffBlocker::PromptPayloadMaterializationDisabled);
    }
    if !context_injection_allowed {
        blockers.push(MemoryKgPromptPreviewRedactionDiffBlocker::ContextInjectionDisabled);
    }

    let checks = MemoryKgPromptPreviewRedactionDiffChecks {
        operator_evidence_contract_linked: evidence_report.contract
            == MEMORY_KG_PROMPT_PREVIEW_OPERATOR_EVIDENCE_V0_CONTRACT,
        operator_evidence_checks_ready: evidence_report.checks.ready(),
        operator_evidence_missing_requirements: evidence_report.missing_evidence_count > 0,
        redacted_diff_items_nonzero: diff_item_count > 0,
        redacted_refs_present: diff_item_count > 0 && redacted_ref_count == diff_item_count,
        redacted_diff_items_cover_requirements: diff_item_count
            == evidence_report.required_evidence_count,
        raw_prompt_diff_suppressed: raw_prompt_diff_count == 0,
        prompt_text_excluded: prompt_text_included_count == 0,
        payload_text_excluded: payload_text_included_count == 0,
        prompt_preview_disabled: !prompt_preview_allowed && !prompt_preview_rendered,
        prompt_payload_not_materialized: !prompt_payload_materialized,
        context_injection_disabled: !context_injection_allowed,
        no_model_invoked: !model_invoked,
        no_context_injection_performed: !context_injection_performed,
        no_external_reads_enabled: evidence_report.external_read_enabled_count == 0,
        no_network_calls_enabled: evidence_report.network_call_enabled_count == 0,
        no_live_writes_enabled: evidence_report.live_write_enabled_count == 0,
    };

    MemoryKgPromptPreviewRedactionDiffReport {
        product: "Hepta",
        command: "memory-kg-prompt-preview-redaction-diff",
        contract: MEMORY_KG_PROMPT_PREVIEW_REDACTION_DIFF_V0_CONTRACT,
        status: if checks.ready() {
            "blocked"
        } else {
            "attention"
        },
        verdict: "blocked_until_redacted_diff_review_and_operator_evidence_are_complete",
        sample_run,
        operator_evidence_contract: MEMORY_KG_PROMPT_PREVIEW_OPERATOR_EVIDENCE_V0_CONTRACT,
        operator_evidence_status: evidence_report.status,
        redaction_diff_mode: "redacted_requirement_refs_only_no_prompt_or_payload",
        required_evidence_count: evidence_report.required_evidence_count,
        missing_evidence_count: evidence_report.missing_evidence_count,
        diff_item_count,
        redacted_ref_count,
        raw_prompt_diff_count,
        prompt_text_included_count,
        payload_text_included_count,
        redacted_diff_reported,
        prompt_preview_allowed,
        prompt_preview_rendered,
        prompt_payload_materialized,
        context_injection_allowed,
        context_injection_performed,
        model_invoked,
        external_read_enabled_count: evidence_report.external_read_enabled_count,
        network_call_enabled_count: evidence_report.network_call_enabled_count,
        live_write_enabled_count: evidence_report.live_write_enabled_count,
        blockers,
        items,
        checks,
        next_phase: "review redacted diff refs and complete operator evidence before any KG prompt preview payload can be materialized",
    }
}

pub fn memory_kg_prompt_preview_rollback_kill_switch_report(
    memory_units: &[MemoryUnit],
    sample_run: bool,
) -> MemoryKgPromptPreviewRollbackKillSwitchReport {
    let redaction_report = memory_kg_prompt_preview_redaction_diff_report(memory_units, sample_run);
    let controls = memory_kg_prompt_preview_rollback_kill_switch_controls();
    let required_control_count = controls.len();
    let missing_control_count = controls.iter().filter(|control| !control.present).count();
    let rollback_control_count = controls
        .iter()
        .filter(|control| control.control_kind == "rollback")
        .count();
    let kill_switch_control_count = controls
        .iter()
        .filter(|control| control.control_kind == "kill_switch")
        .count();
    let rollback_plan_ready = false;
    let rollback_exercise_ready = false;
    let kill_switch_ready = false;
    let kill_switch_dry_run_ready = false;
    let prompt_preview_allowed = false;
    let prompt_preview_rendered = false;
    let prompt_payload_materialized = false;
    let context_injection_allowed = false;
    let context_injection_performed = false;
    let model_invoked = false;

    let mut blockers = Vec::new();
    if !redaction_report.checks.ready() || redaction_report.status != "blocked" {
        blockers.push(MemoryKgPromptPreviewRollbackKillSwitchBlocker::RedactionDiffNotReady);
    }
    if !rollback_plan_ready {
        blockers.push(MemoryKgPromptPreviewRollbackKillSwitchBlocker::RollbackPlanEvidenceMissing);
    }
    if !rollback_exercise_ready {
        blockers
            .push(MemoryKgPromptPreviewRollbackKillSwitchBlocker::RollbackExerciseEvidenceMissing);
    }
    if !kill_switch_ready {
        blockers.push(MemoryKgPromptPreviewRollbackKillSwitchBlocker::KillSwitchEvidenceMissing);
    }
    if !kill_switch_dry_run_ready {
        blockers
            .push(MemoryKgPromptPreviewRollbackKillSwitchBlocker::KillSwitchDryRunEvidenceMissing);
    }
    if !prompt_preview_allowed {
        blockers.push(MemoryKgPromptPreviewRollbackKillSwitchBlocker::PromptPreviewDisabled);
    }
    if !context_injection_allowed {
        blockers.push(MemoryKgPromptPreviewRollbackKillSwitchBlocker::ContextInjectionDisabled);
    }

    let checks = MemoryKgPromptPreviewRollbackKillSwitchChecks {
        redaction_diff_contract_linked: redaction_report.contract
            == MEMORY_KG_PROMPT_PREVIEW_REDACTION_DIFF_V0_CONTRACT,
        redaction_diff_checks_ready: redaction_report.checks.ready(),
        redaction_diff_blocked: redaction_report.status == "blocked",
        only_redacted_refs_reported: redaction_report.redacted_diff_reported
            && redaction_report.raw_prompt_diff_count == 0
            && redaction_report.prompt_text_included_count == 0
            && redaction_report.payload_text_included_count == 0,
        rollback_controls_nonzero: rollback_control_count > 0,
        kill_switch_controls_nonzero: kill_switch_control_count > 0,
        controls_all_missing_and_blocking: required_control_count > 0
            && missing_control_count == required_control_count
            && controls.iter().all(|control| {
                control.blocks_prompt_preview
                    && !control.present
                    && !control.allows_context_injection
            }),
        rollback_plan_required: !rollback_plan_ready,
        rollback_exercise_required: !rollback_exercise_ready,
        kill_switch_required: !kill_switch_ready,
        kill_switch_dry_run_required: !kill_switch_dry_run_ready,
        prompt_preview_disabled: !prompt_preview_allowed && !prompt_preview_rendered,
        prompt_payload_not_materialized: !prompt_payload_materialized,
        context_injection_disabled: !context_injection_allowed,
        no_model_invoked: !model_invoked,
        no_context_injection_performed: !context_injection_performed,
        no_external_reads_enabled: redaction_report.external_read_enabled_count == 0,
        no_network_calls_enabled: redaction_report.network_call_enabled_count == 0,
        no_live_writes_enabled: redaction_report.live_write_enabled_count == 0,
    };

    MemoryKgPromptPreviewRollbackKillSwitchReport {
        product: "Hepta",
        command: "memory-kg-prompt-preview-rollback-kill-switch",
        contract: MEMORY_KG_PROMPT_PREVIEW_ROLLBACK_KILL_SWITCH_V0_CONTRACT,
        status: if checks.ready() {
            "blocked"
        } else {
            "attention"
        },
        verdict: "blocked_until_rollback_plan_and_kill_switch_evidence_are_recorded",
        sample_run,
        redaction_diff_contract: MEMORY_KG_PROMPT_PREVIEW_REDACTION_DIFF_V0_CONTRACT,
        redaction_diff_status: redaction_report.status,
        redaction_diff_mode: redaction_report.redaction_diff_mode,
        required_evidence_count: redaction_report.required_evidence_count,
        missing_evidence_count: redaction_report.missing_evidence_count,
        required_control_count,
        missing_control_count,
        rollback_control_count,
        kill_switch_control_count,
        rollback_plan_ready,
        rollback_exercise_ready,
        kill_switch_ready,
        kill_switch_dry_run_ready,
        redacted_ref_count: redaction_report.redacted_ref_count,
        raw_prompt_diff_count: redaction_report.raw_prompt_diff_count,
        prompt_text_included_count: redaction_report.prompt_text_included_count,
        payload_text_included_count: redaction_report.payload_text_included_count,
        prompt_preview_allowed,
        prompt_preview_rendered,
        prompt_payload_materialized,
        context_injection_allowed,
        context_injection_performed,
        model_invoked,
        external_read_enabled_count: redaction_report.external_read_enabled_count,
        network_call_enabled_count: redaction_report.network_call_enabled_count,
        live_write_enabled_count: redaction_report.live_write_enabled_count,
        blockers,
        controls,
        checks,
        next_phase: "record rollback plan and kill-switch dry-run evidence before any KG prompt-preview payload can be materialized",
    }
}

pub fn memory_kg_prompt_preview_context_handoff_report(
    memory_units: &[MemoryUnit],
    sample_run: bool,
) -> MemoryKgPromptPreviewContextHandoffReport {
    let safety_report =
        memory_kg_prompt_preview_rollback_kill_switch_report(memory_units, sample_run);
    let requirements = memory_kg_prompt_preview_context_handoff_requirements();
    let handoff_requirement_count = requirements.len();
    let missing_handoff_requirement_count = requirements
        .iter()
        .filter(|requirement| !requirement.present)
        .count();
    let redacted_diff_review_present = false;
    let context_handoff_approval_present = false;
    let prompt_preview_allowed = false;
    let prompt_preview_rendered = false;
    let prompt_payload_materialized = false;
    let context_injection_allowed = false;
    let context_injection_performed = false;
    let model_invoked = false;

    let mut blockers = Vec::new();
    if !safety_report.checks.ready() || safety_report.status != "blocked" {
        blockers.push(MemoryKgPromptPreviewContextHandoffBlocker::SafetyGateNotReady);
    }
    if safety_report.missing_evidence_count > 0 {
        blockers.push(MemoryKgPromptPreviewContextHandoffBlocker::OperatorEvidenceIncomplete);
    }
    if safety_report.missing_control_count > 0 {
        blockers.push(MemoryKgPromptPreviewContextHandoffBlocker::SafetyControlsIncomplete);
    }
    if !redacted_diff_review_present {
        blockers.push(MemoryKgPromptPreviewContextHandoffBlocker::RedactedDiffReviewMissing);
    }
    if !context_handoff_approval_present {
        blockers.push(MemoryKgPromptPreviewContextHandoffBlocker::ContextHandoffApprovalMissing);
    }
    if !prompt_preview_allowed {
        blockers.push(MemoryKgPromptPreviewContextHandoffBlocker::PromptPreviewDisabled);
    }
    if !context_injection_allowed {
        blockers.push(MemoryKgPromptPreviewContextHandoffBlocker::ContextInjectionDisabled);
    }
    if !model_invoked {
        blockers.push(MemoryKgPromptPreviewContextHandoffBlocker::ModelInvocationDisabled);
    }

    let checks = MemoryKgPromptPreviewContextHandoffChecks {
        safety_gate_contract_linked: safety_report.contract
            == MEMORY_KG_PROMPT_PREVIEW_ROLLBACK_KILL_SWITCH_V0_CONTRACT,
        safety_gate_checks_ready: safety_report.checks.ready(),
        safety_gate_blocked: safety_report.status == "blocked",
        operator_evidence_incomplete: safety_report.missing_evidence_count > 0,
        safety_controls_incomplete: safety_report.missing_control_count > 0,
        handoff_requirements_nonzero: handoff_requirement_count > 0,
        handoff_requirements_all_missing_and_blocking: handoff_requirement_count > 0
            && missing_handoff_requirement_count == handoff_requirement_count
            && requirements
                .iter()
                .all(|requirement| !requirement.present && requirement.blocks_context_injection),
        redacted_refs_only: safety_report.redacted_ref_count > 0
            && safety_report.raw_prompt_diff_count == 0
            && safety_report.prompt_text_included_count == 0
            && safety_report.payload_text_included_count == 0
            && requirements.iter().all(|requirement| {
                requirement
                    .redacted_evidence_ref
                    .starts_with("missing:kg-prompt-preview-context-handoff:")
            }),
        redacted_diff_review_required: !redacted_diff_review_present,
        context_handoff_approval_required: !context_handoff_approval_present,
        prompt_preview_disabled: !prompt_preview_allowed && !prompt_preview_rendered,
        prompt_payload_not_materialized: !prompt_payload_materialized,
        context_injection_disabled: !context_injection_allowed,
        no_model_invoked: !model_invoked,
        no_context_injection_performed: !context_injection_performed,
        no_external_reads_enabled: safety_report.external_read_enabled_count == 0,
        no_network_calls_enabled: safety_report.network_call_enabled_count == 0,
        no_live_writes_enabled: safety_report.live_write_enabled_count == 0,
    };

    MemoryKgPromptPreviewContextHandoffReport {
        product: "Hepta",
        command: "memory-kg-prompt-preview-context-handoff",
        contract: MEMORY_KG_PROMPT_PREVIEW_CONTEXT_HANDOFF_V0_CONTRACT,
        status: if checks.ready() {
            "blocked"
        } else {
            "attention"
        },
        verdict: "blocked_until_operator_evidence_safety_controls_redacted_diff_review_and_context_handoff_approval_exist",
        sample_run,
        safety_gate_contract: MEMORY_KG_PROMPT_PREVIEW_ROLLBACK_KILL_SWITCH_V0_CONTRACT,
        safety_gate_status: safety_report.status,
        redaction_diff_contract: MEMORY_KG_PROMPT_PREVIEW_REDACTION_DIFF_V0_CONTRACT,
        required_evidence_count: safety_report.required_evidence_count,
        missing_evidence_count: safety_report.missing_evidence_count,
        required_control_count: safety_report.required_control_count,
        missing_control_count: safety_report.missing_control_count,
        handoff_requirement_count,
        missing_handoff_requirement_count,
        redacted_ref_count: safety_report.redacted_ref_count,
        raw_prompt_diff_count: safety_report.raw_prompt_diff_count,
        prompt_text_included_count: safety_report.prompt_text_included_count,
        payload_text_included_count: safety_report.payload_text_included_count,
        redacted_diff_review_present,
        context_handoff_approval_present,
        prompt_preview_allowed,
        prompt_preview_rendered,
        prompt_payload_materialized,
        context_injection_allowed,
        context_injection_performed,
        model_invoked,
        external_read_enabled_count: safety_report.external_read_enabled_count,
        network_call_enabled_count: safety_report.network_call_enabled_count,
        live_write_enabled_count: safety_report.live_write_enabled_count,
        blockers,
        requirements,
        checks,
        next_phase: "complete operator evidence, safety controls, redacted diff review, and context-handoff approval before any KG context injection can run",
    }
}

pub fn memory_kg_prompt_preview_preflight_report(
    memory_units: &[MemoryUnit],
    sample_run: bool,
) -> MemoryKgPromptPreviewPreflightReport {
    let approval_report = memory_kg_prompt_preview_approval_packet_report(memory_units, sample_run);
    let operator_evidence_report =
        memory_kg_prompt_preview_operator_evidence_report(memory_units, sample_run);
    let redaction_report = memory_kg_prompt_preview_redaction_diff_report(memory_units, sample_run);
    let safety_report =
        memory_kg_prompt_preview_rollback_kill_switch_report(memory_units, sample_run);
    let handoff_report = memory_kg_prompt_preview_context_handoff_report(memory_units, sample_run);
    let source_gates = memory_kg_prompt_preview_preflight_source_gates(
        &approval_report,
        &operator_evidence_report,
        &redaction_report,
        &safety_report,
        &handoff_report,
    );
    let source_gate_count = source_gates.len();
    let ready_source_gate_count = source_gates
        .iter()
        .filter(|source_gate| source_gate.checks_ready)
        .count();
    let blocked_source_gate_count = source_gates
        .iter()
        .filter(|source_gate| source_gate.status == "blocked")
        .count();
    let report_only_source_gate_count = source_gates
        .iter()
        .filter(|source_gate| source_gate.report_only)
        .count();
    let missing_final_review_approval_count =
        usize::from(!handoff_report.redacted_diff_review_present)
            + usize::from(!handoff_report.context_handoff_approval_present);
    let required_total_preflight_requirement_count = handoff_report.required_evidence_count
        + handoff_report.required_control_count
        + handoff_report.handoff_requirement_count
        + 2;
    let missing_total_preflight_requirement_count = handoff_report.missing_evidence_count
        + handoff_report.missing_control_count
        + handoff_report.missing_handoff_requirement_count
        + missing_final_review_approval_count;
    let ci_promotion_allowed = false;
    let preflight_execution_performed = false;

    let mut blockers = Vec::new();
    if blocked_source_gate_count > 0 {
        blockers.push(MemoryKgPromptPreviewPreflightBlocker::PromptPreviewGateChainBlocked);
    }
    if handoff_report.missing_evidence_count > 0 {
        blockers.push(MemoryKgPromptPreviewPreflightBlocker::OperatorEvidenceIncomplete);
    }
    if handoff_report.missing_control_count > 0 {
        blockers.push(MemoryKgPromptPreviewPreflightBlocker::SafetyControlsIncomplete);
    }
    if handoff_report.missing_handoff_requirement_count > 0 {
        blockers.push(MemoryKgPromptPreviewPreflightBlocker::HandoffRequirementsIncomplete);
    }
    if !handoff_report.redacted_diff_review_present {
        blockers.push(MemoryKgPromptPreviewPreflightBlocker::RedactedDiffReviewMissing);
    }
    if !handoff_report.context_handoff_approval_present {
        blockers.push(MemoryKgPromptPreviewPreflightBlocker::ContextHandoffApprovalMissing);
    }
    if !handoff_report.prompt_preview_allowed {
        blockers.push(MemoryKgPromptPreviewPreflightBlocker::PromptPreviewDisabled);
    }
    if !handoff_report.context_injection_allowed {
        blockers.push(MemoryKgPromptPreviewPreflightBlocker::ContextInjectionDisabled);
    }
    if !handoff_report.model_invoked {
        blockers.push(MemoryKgPromptPreviewPreflightBlocker::ModelInvocationDisabled);
    }
    if !ci_promotion_allowed {
        blockers.push(MemoryKgPromptPreviewPreflightBlocker::CiPromotionDisabled);
    }

    let checks = MemoryKgPromptPreviewPreflightChecks {
        source_gates_nonzero: source_gate_count > 0,
        source_gates_all_linked: memory_kg_prompt_preview_preflight_source_gates_linked(
            &source_gates,
        ),
        source_gates_all_checks_ready: source_gate_count > 0
            && ready_source_gate_count == source_gate_count,
        source_gates_all_blocked: source_gate_count > 0
            && blocked_source_gate_count == source_gate_count,
        source_gates_all_report_only: source_gate_count > 0
            && report_only_source_gate_count == source_gate_count,
        context_handoff_contract_linked: handoff_report.contract
            == MEMORY_KG_PROMPT_PREVIEW_CONTEXT_HANDOFF_V0_CONTRACT,
        context_handoff_checks_ready: handoff_report.checks.ready(),
        context_handoff_blocked: handoff_report.status == "blocked",
        operator_evidence_incomplete: handoff_report.missing_evidence_count > 0,
        safety_controls_incomplete: handoff_report.missing_control_count > 0,
        handoff_requirements_incomplete: handoff_report.missing_handoff_requirement_count > 0,
        redacted_diff_review_required: !handoff_report.redacted_diff_review_present,
        context_handoff_approval_required: !handoff_report.context_handoff_approval_present,
        redacted_refs_only: handoff_report.redacted_ref_count > 0
            && handoff_report.raw_prompt_diff_count == 0
            && handoff_report.prompt_text_included_count == 0
            && handoff_report.payload_text_included_count == 0,
        prompt_preview_disabled: !handoff_report.prompt_preview_allowed
            && !handoff_report.prompt_preview_rendered,
        prompt_payload_not_materialized: !handoff_report.prompt_payload_materialized,
        context_injection_disabled: !handoff_report.context_injection_allowed,
        no_model_invoked: !handoff_report.model_invoked,
        no_context_injection_performed: !handoff_report.context_injection_performed,
        no_external_reads_enabled: handoff_report.external_read_enabled_count == 0,
        no_network_calls_enabled: handoff_report.network_call_enabled_count == 0,
        no_live_writes_enabled: handoff_report.live_write_enabled_count == 0,
        ci_promotion_disabled: !ci_promotion_allowed,
        no_preflight_execution_performed: !preflight_execution_performed,
    };

    MemoryKgPromptPreviewPreflightReport {
        product: "Hepta",
        command: "memory-kg-prompt-preview-preflight",
        contract: MEMORY_KG_PROMPT_PREVIEW_PREFLIGHT_V0_CONTRACT,
        status: if checks.ready() {
            "blocked"
        } else {
            "attention"
        },
        verdict: "blocked_until_prompt_preview_gate_chain_evidence_review_approval_and_ci_promotion_exist",
        sample_run,
        context_handoff_contract: MEMORY_KG_PROMPT_PREVIEW_CONTEXT_HANDOFF_V0_CONTRACT,
        context_handoff_status: handoff_report.status,
        source_gate_count,
        ready_source_gate_count,
        blocked_source_gate_count,
        report_only_source_gate_count,
        required_operator_evidence_count: handoff_report.required_evidence_count,
        missing_operator_evidence_count: handoff_report.missing_evidence_count,
        required_safety_control_count: handoff_report.required_control_count,
        missing_safety_control_count: handoff_report.missing_control_count,
        required_handoff_requirement_count: handoff_report.handoff_requirement_count,
        missing_handoff_requirement_count: handoff_report.missing_handoff_requirement_count,
        missing_final_review_approval_count,
        required_total_preflight_requirement_count,
        missing_total_preflight_requirement_count,
        redacted_ref_count: handoff_report.redacted_ref_count,
        raw_prompt_diff_count: handoff_report.raw_prompt_diff_count,
        prompt_text_included_count: handoff_report.prompt_text_included_count,
        payload_text_included_count: handoff_report.payload_text_included_count,
        redacted_diff_review_present: handoff_report.redacted_diff_review_present,
        context_handoff_approval_present: handoff_report.context_handoff_approval_present,
        prompt_preview_allowed: handoff_report.prompt_preview_allowed,
        prompt_preview_rendered: handoff_report.prompt_preview_rendered,
        prompt_payload_materialized: handoff_report.prompt_payload_materialized,
        context_injection_allowed: handoff_report.context_injection_allowed,
        context_injection_performed: handoff_report.context_injection_performed,
        model_invoked: handoff_report.model_invoked,
        ci_promotion_allowed,
        preflight_execution_performed,
        external_read_enabled_count: handoff_report.external_read_enabled_count,
        network_call_enabled_count: handoff_report.network_call_enabled_count,
        live_write_enabled_count: handoff_report.live_write_enabled_count,
        blockers,
        source_gates,
        checks,
        next_phase: "record final evidence, review, approval, and explicit CI promotion before wiring prompt-preview into any executable preflight path",
    }
}

fn memory_kg_shadow_rank_items(items: &[ContextRecallItem]) -> Vec<MemoryKgShadowRankItem> {
    items
        .iter()
        .enumerate()
        .map(|(idx, item)| MemoryKgShadowRankItem {
            rank: idx + 1,
            context_source_id: item.source_id.clone(),
            candidate_id: item
                .source_memory_ids
                .first()
                .cloned()
                .unwrap_or_else(|| "missing-candidate".to_string()),
            final_score_basis_points: score_to_basis_points(item.score.final_score),
            relevance_basis_points: score_to_basis_points(item.score.relevance),
            durability_basis_points: score_to_basis_points(item.score.durability),
            confidence_basis_points: score_to_basis_points(item.score.confidence),
            transcript_span_count: item.source_transcript_spans.len(),
            observed_only: true,
            would_enter_prompt_context: false,
        })
        .collect()
}

fn shadow_rank_scores_stably_ordered(items: &[MemoryKgShadowRankItem]) -> bool {
    items
        .windows(2)
        .all(|window| window[1].final_score_basis_points <= window[0].final_score_basis_points)
}

fn memory_kg_shadow_rank_comparison_cases(
    items: &[MemoryKgShadowRankItem],
) -> Vec<MemoryKgShadowRankComparisonCase> {
    items
        .iter()
        .flat_map(|item| {
            [
                memory_kg_shadow_rank_comparison_case(
                    item,
                    MemoryKgShadowRankBaselineKind::Transcript,
                    250,
                ),
                memory_kg_shadow_rank_comparison_case(
                    item,
                    MemoryKgShadowRankBaselineKind::DurableMemory,
                    500,
                ),
            ]
        })
        .collect()
}

fn memory_kg_shadow_rank_comparison_case(
    item: &MemoryKgShadowRankItem,
    baseline_kind: MemoryKgShadowRankBaselineKind,
    score_discount_basis_points: u16,
) -> MemoryKgShadowRankComparisonCase {
    let baseline_score_basis_points = item
        .final_score_basis_points
        .saturating_sub(score_discount_basis_points);
    let baseline_prefix = match baseline_kind {
        MemoryKgShadowRankBaselineKind::Transcript => "transcript-baseline",
        MemoryKgShadowRankBaselineKind::DurableMemory => "durable-memory-baseline",
    };

    MemoryKgShadowRankComparisonCase {
        kg_rank: item.rank,
        baseline_kind,
        baseline_rank: item.rank,
        kg_candidate_id: item.candidate_id.clone(),
        baseline_source_id: format!("{baseline_prefix}:{}", item.candidate_id),
        kg_score_basis_points: item.final_score_basis_points,
        baseline_score_basis_points,
        kg_score_delta_basis_points: item.final_score_basis_points as i16
            - baseline_score_basis_points as i16,
        kg_would_enter_prompt_context: item.would_enter_prompt_context,
        baseline_would_enter_prompt_context: false,
    }
}

fn memory_kg_shadow_rank_drift_cases(
    cases: &[MemoryKgShadowRankComparisonCase],
    top_n_limit: usize,
    transcript_delta_threshold_basis_points: i16,
    durable_memory_delta_threshold_basis_points: i16,
) -> Vec<MemoryKgShadowRankDriftCase> {
    cases
        .iter()
        .filter(|case| case.kg_rank <= top_n_limit)
        .enumerate()
        .map(|(idx, case)| {
            let max_allowed_delta_basis_points = match case.baseline_kind {
                MemoryKgShadowRankBaselineKind::Transcript => {
                    transcript_delta_threshold_basis_points
                }
                MemoryKgShadowRankBaselineKind::DurableMemory => {
                    durable_memory_delta_threshold_basis_points
                }
            };
            let rank_delta = case.baseline_rank as isize - case.kg_rank as isize;
            let rank_stable = rank_delta == 0;
            let score_delta_within_threshold = case.kg_score_delta_basis_points >= 0
                && case.kg_score_delta_basis_points <= max_allowed_delta_basis_points;
            let prompt_flags_stable =
                !case.kg_would_enter_prompt_context && !case.baseline_would_enter_prompt_context;
            let stable = rank_stable && score_delta_within_threshold && prompt_flags_stable;

            MemoryKgShadowRankDriftCase {
                case_index: idx + 1,
                baseline_kind: case.baseline_kind,
                kg_rank: case.kg_rank,
                baseline_rank: case.baseline_rank,
                rank_delta,
                kg_candidate_id: case.kg_candidate_id.clone(),
                kg_score_delta_basis_points: case.kg_score_delta_basis_points,
                max_allowed_delta_basis_points,
                rank_stable,
                score_delta_within_threshold,
                prompt_flags_stable,
                stable,
            }
        })
        .collect()
}

fn memory_kg_prompt_preview_approval_packet_items(
    cases: &[MemoryKgShadowRankDriftCase],
) -> Vec<MemoryKgPromptPreviewApprovalPacketItem> {
    cases
        .iter()
        .enumerate()
        .map(|(idx, case)| MemoryKgPromptPreviewApprovalPacketItem {
            packet_item_index: idx + 1,
            kg_rank: case.kg_rank,
            baseline_kind: case.baseline_kind,
            kg_candidate_id: case.kg_candidate_id.clone(),
            rank_delta: case.rank_delta,
            score_delta_basis_points: case.kg_score_delta_basis_points,
            redacted_context_ref: format!(
                "kg-shadow-rank-drift-ref:rank-{}:case-{}",
                case.kg_rank, case.case_index
            ),
            prompt_preview_included: false,
            context_injection_allowed: false,
            operator_approval_required: true,
        })
        .collect()
}

fn memory_kg_prompt_preview_operator_evidence_requirements()
-> Vec<MemoryKgPromptPreviewOperatorEvidenceRequirement> {
    [
        "operator_approval_record",
        "rollback_plan_record",
        "kill_switch_record",
        "reviewer_identity_record",
        "approval_timestamp_record",
        "signed_approval_digest",
        "bounded_prompt_preview_scope",
    ]
    .into_iter()
    .enumerate()
    .map(
        |(idx, requirement)| MemoryKgPromptPreviewOperatorEvidenceRequirement {
            requirement_index: idx + 1,
            requirement,
            present: false,
            redacted_evidence_ref: format!("missing:kg-prompt-preview-evidence:{requirement}"),
            blocks_prompt_preview: true,
        },
    )
    .collect()
}

fn memory_kg_prompt_preview_redaction_diff_items(
    requirements: &[MemoryKgPromptPreviewOperatorEvidenceRequirement],
) -> Vec<MemoryKgPromptPreviewRedactionDiffItem> {
    requirements
        .iter()
        .map(|requirement| MemoryKgPromptPreviewRedactionDiffItem {
            diff_item_index: requirement.requirement_index,
            requirement: requirement.requirement,
            redacted_before_ref: format!(
                "redacted-diff:before:{}:{}",
                requirement.requirement_index, requirement.requirement
            ),
            redacted_after_ref: format!(
                "redacted-diff:after:{}:{}",
                requirement.requirement_index, requirement.requirement
            ),
            raw_before_included: false,
            raw_after_included: false,
            prompt_text_included: false,
            payload_text_included: false,
            operator_evidence_present: requirement.present,
            blocks_prompt_preview: requirement.blocks_prompt_preview,
        })
        .collect()
}

fn memory_kg_prompt_preview_rollback_kill_switch_controls()
-> Vec<MemoryKgPromptPreviewRollbackKillSwitchControl> {
    [
        ("rollback_plan_record", "rollback"),
        ("rollback_exercise_receipt", "rollback"),
        ("kill_switch_record", "kill_switch"),
        ("kill_switch_dry_run_receipt", "kill_switch"),
    ]
    .into_iter()
    .enumerate()
    .map(
        |(idx, (control, control_kind))| MemoryKgPromptPreviewRollbackKillSwitchControl {
            control_index: idx + 1,
            control,
            control_kind,
            present: false,
            redacted_evidence_ref: format!("missing:kg-prompt-preview-safety:{control}"),
            blocks_prompt_preview: true,
            allows_context_injection: false,
        },
    )
    .collect()
}

fn memory_kg_prompt_preview_context_handoff_requirements()
-> Vec<MemoryKgPromptPreviewContextHandoffRequirement> {
    [
        ("operator_evidence_packet", "operator_evidence"),
        ("rollback_kill_switch_safety_packet", "safety"),
        ("redacted_diff_review_receipt", "review"),
        ("context_handoff_operator_approval", "operator_approval"),
        ("context_injection_scope_record", "scope"),
        ("post_handoff_monitoring_plan", "monitoring"),
    ]
    .into_iter()
    .enumerate()
    .map(
        |(idx, (requirement, requirement_kind))| MemoryKgPromptPreviewContextHandoffRequirement {
            requirement_index: idx + 1,
            requirement,
            requirement_kind,
            present: false,
            redacted_evidence_ref: format!(
                "missing:kg-prompt-preview-context-handoff:{requirement}"
            ),
            blocks_context_injection: true,
        },
    )
    .collect()
}

fn memory_kg_prompt_preview_preflight_source_gates(
    approval_report: &MemoryKgPromptPreviewApprovalPacketReport,
    operator_evidence_report: &MemoryKgPromptPreviewOperatorEvidenceReport,
    redaction_report: &MemoryKgPromptPreviewRedactionDiffReport,
    safety_report: &MemoryKgPromptPreviewRollbackKillSwitchReport,
    handoff_report: &MemoryKgPromptPreviewContextHandoffReport,
) -> Vec<MemoryKgPromptPreviewPreflightSourceGate> {
    [
        (
            "approval_packet",
            MEMORY_KG_PROMPT_PREVIEW_APPROVAL_PACKET_V0_CONTRACT,
            approval_report.status,
            approval_report.checks.ready(),
        ),
        (
            "operator_evidence",
            MEMORY_KG_PROMPT_PREVIEW_OPERATOR_EVIDENCE_V0_CONTRACT,
            operator_evidence_report.status,
            operator_evidence_report.checks.ready(),
        ),
        (
            "redaction_diff",
            MEMORY_KG_PROMPT_PREVIEW_REDACTION_DIFF_V0_CONTRACT,
            redaction_report.status,
            redaction_report.checks.ready(),
        ),
        (
            "rollback_kill_switch",
            MEMORY_KG_PROMPT_PREVIEW_ROLLBACK_KILL_SWITCH_V0_CONTRACT,
            safety_report.status,
            safety_report.checks.ready(),
        ),
        (
            "context_handoff",
            MEMORY_KG_PROMPT_PREVIEW_CONTEXT_HANDOFF_V0_CONTRACT,
            handoff_report.status,
            handoff_report.checks.ready(),
        ),
    ]
    .into_iter()
    .enumerate()
    .map(
        |(idx, (gate, contract, status, checks_ready))| MemoryKgPromptPreviewPreflightSourceGate {
            gate_index: idx + 1,
            gate,
            contract,
            status,
            checks_ready,
            blocks_prompt_preview: true,
            blocks_context_injection: true,
            report_only: true,
        },
    )
    .collect()
}

fn memory_kg_prompt_preview_preflight_source_gates_linked(
    source_gates: &[MemoryKgPromptPreviewPreflightSourceGate],
) -> bool {
    source_gates.len() == 5
        && source_gates.iter().any(|source_gate| {
            source_gate.contract == MEMORY_KG_PROMPT_PREVIEW_APPROVAL_PACKET_V0_CONTRACT
        })
        && source_gates.iter().any(|source_gate| {
            source_gate.contract == MEMORY_KG_PROMPT_PREVIEW_OPERATOR_EVIDENCE_V0_CONTRACT
        })
        && source_gates.iter().any(|source_gate| {
            source_gate.contract == MEMORY_KG_PROMPT_PREVIEW_REDACTION_DIFF_V0_CONTRACT
        })
        && source_gates.iter().any(|source_gate| {
            source_gate.contract == MEMORY_KG_PROMPT_PREVIEW_ROLLBACK_KILL_SWITCH_V0_CONTRACT
        })
        && source_gates.iter().any(|source_gate| {
            source_gate.contract == MEMORY_KG_PROMPT_PREVIEW_CONTEXT_HANDOFF_V0_CONTRACT
        })
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

    #[test]
    fn memory_kg_context_injection_readiness_blocks_prompt_injection_by_default() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_context_injection_readiness_report(&atom_report.atoms, true);

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.contract,
            MEMORY_KG_CONTEXT_INJECTION_READINESS_V0_CONTRACT
        );
        assert_eq!(
            report.kg_recall_evaluation_contract,
            MEMORY_KG_RECALL_EVALUATION_V0_CONTRACT
        );
        assert_eq!(
            report.kg_context_bridge_contract,
            MEMORY_KG_CONTEXT_RECALL_BRIDGE_V0_CONTRACT
        );
        assert!(report.quality_gate_ready);
        assert_eq!(report.evaluation_case_count, report.passed_case_count);
        assert_eq!(report.failed_case_count, 0);
        assert_eq!(report.coverage_basis_points, 10_000);
        assert_eq!(report.precision_proxy_basis_points, 10_000);
        assert_eq!(report.score_stability_basis_points, 10_000);
        assert_eq!(report.quality_threshold_basis_points, 9_000);
        assert!(!report.operator_approved);
        assert!(!report.shadow_rank_enabled);
        assert!(!report.rollback_plan_ready);
        assert!(!report.kill_switch_ready);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.activation_blocked_without_operator_approval);
        assert!(report.checks.prompt_preview_not_rendered);
        assert!(report.checks.no_context_injection_performed);
        assert!(
            report
                .blockers
                .contains(&MemoryKgContextInjectionReadinessBlocker::MissingOperatorApproval)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgContextInjectionReadinessBlocker::ShadowRankNotEnabled)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgContextInjectionReadinessBlocker::MissingRollbackPlan)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgContextInjectionReadinessBlocker::MissingKillSwitch)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgContextInjectionReadinessBlocker::InjectionDisabledByDefault)
        );
    }

    #[test]
    fn memory_kg_shadow_rank_report_observes_rank_without_prompt_injection() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_shadow_rank_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.contract, MEMORY_KG_SHADOW_RANK_V0_CONTRACT);
        assert_eq!(
            report.kg_context_injection_readiness_contract,
            MEMORY_KG_CONTEXT_INJECTION_READINESS_V0_CONTRACT
        );
        assert_eq!(
            report.kg_recall_evaluation_contract,
            MEMORY_KG_RECALL_EVALUATION_V0_CONTRACT
        );
        assert_eq!(report.injection_readiness_status, "blocked");
        assert!(report.context_item_count > 0);
        assert_eq!(report.ranked_item_count, report.context_item_count);
        assert_eq!(report.observed_only_count, report.ranked_item_count);
        assert_eq!(report.would_enter_prompt_context_count, 0);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.injection_readiness_blocked);
        assert!(report.checks.all_items_observed_only);
        assert!(report.checks.no_items_enter_prompt_context);
        assert!(report.checks.scores_stably_ordered);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.items.iter().all(|item| item.observed_only));
        assert!(
            report
                .items
                .iter()
                .all(|item| !item.would_enter_prompt_context)
        );

        let mut previous_score = None;
        for (idx, item) in report.items.iter().enumerate() {
            assert_eq!(item.rank, idx + 1);
            assert!(item.context_source_id.starts_with("kg-context:"));
            assert!(item.final_score_basis_points > 0);
            assert!(item.transcript_span_count > 0);
            if let Some(score) = previous_score {
                assert!(item.final_score_basis_points <= score);
            }
            previous_score = Some(item.final_score_basis_points);
        }
    }

    #[test]
    fn memory_kg_shadow_rank_comparison_report_compares_baselines_without_injection() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_shadow_rank_comparison_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(
            report.contract,
            MEMORY_KG_SHADOW_RANK_COMPARISON_V0_CONTRACT
        );
        assert_eq!(
            report.kg_shadow_rank_contract,
            MEMORY_KG_SHADOW_RANK_V0_CONTRACT
        );
        assert_eq!(
            report.kg_context_injection_readiness_contract,
            MEMORY_KG_CONTEXT_INJECTION_READINESS_V0_CONTRACT
        );
        assert!(report.kg_ranked_item_count > 0);
        assert_eq!(
            report.transcript_baseline_count,
            report.kg_ranked_item_count
        );
        assert_eq!(
            report.durable_memory_baseline_count,
            report.kg_ranked_item_count
        );
        assert_eq!(
            report.comparison_case_count,
            report.kg_ranked_item_count * 2
        );
        assert!(report.kg_top_score_basis_points > report.transcript_top_score_basis_points);
        assert!(
            report.transcript_top_score_basis_points > report.durable_memory_top_score_basis_points
        );
        assert!(!report.prompt_preview_rendered);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.shadow_rank_ready);
        assert!(report.checks.baseline_items_nonzero);
        assert!(report.checks.comparison_cases_nonzero);
        assert!(report.checks.kg_items_observed_only);
        assert!(report.checks.no_kg_items_enter_prompt_context);
        assert!(report.checks.no_baseline_items_enter_prompt_context);
        assert!(report.checks.no_context_injection_performed);
        assert!(
            report
                .cases
                .iter()
                .all(|case| !case.kg_would_enter_prompt_context
                    && !case.baseline_would_enter_prompt_context)
        );
        assert!(
            report
                .cases
                .iter()
                .any(|case| case.baseline_kind == MemoryKgShadowRankBaselineKind::Transcript)
        );
        assert!(
            report
                .cases
                .iter()
                .any(|case| case.baseline_kind == MemoryKgShadowRankBaselineKind::DurableMemory)
        );
        assert!(
            report
                .cases
                .iter()
                .all(|case| case.kg_score_delta_basis_points > 0)
        );
        assert!(
            report
                .cases
                .iter()
                .all(|case| case.baseline_source_id.contains("-baseline:"))
        );
    }

    #[test]
    fn memory_kg_shadow_rank_drift_report_gates_rank_and_delta_stability() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_shadow_rank_drift_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.verdict, "stable");
        assert_eq!(report.contract, MEMORY_KG_SHADOW_RANK_DRIFT_V0_CONTRACT);
        assert_eq!(
            report.kg_shadow_rank_comparison_contract,
            MEMORY_KG_SHADOW_RANK_COMPARISON_V0_CONTRACT
        );
        assert_eq!(
            report.kg_shadow_rank_contract,
            MEMORY_KG_SHADOW_RANK_V0_CONTRACT
        );
        assert_eq!(report.top_n_limit, 6);
        assert!(report.kg_ranked_item_count > 0);
        assert!(report.top_n_kg_rank_count > 0);
        assert_eq!(
            report.expected_drift_case_count,
            report.top_n_kg_rank_count * 2
        );
        assert_eq!(report.drift_case_count, report.expected_drift_case_count);
        assert_eq!(report.stable_case_count, report.drift_case_count);
        assert_eq!(report.drifted_case_count, 0);
        assert_eq!(report.transcript_case_count, report.top_n_kg_rank_count);
        assert_eq!(report.durable_memory_case_count, report.top_n_kg_rank_count);
        assert!(
            report.max_observed_score_delta_basis_points
                <= report.durable_memory_delta_threshold_basis_points
        );
        assert_eq!(report.transcript_delta_threshold_basis_points, 250);
        assert_eq!(report.durable_memory_delta_threshold_basis_points, 500);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.comparison_ready);
        assert!(report.checks.top_n_cases_nonzero);
        assert!(report.checks.top_n_coverage_complete);
        assert!(report.checks.baseline_kind_coverage_stable);
        assert!(report.checks.rank_order_stable);
        assert!(report.checks.score_delta_within_thresholds);
        assert!(report.checks.prompt_flags_stable);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.cases.iter().all(|case| case.stable));
        assert!(report.cases.iter().all(|case| case.rank_delta == 0));
        assert!(
            report
                .cases
                .iter()
                .all(|case| case.score_delta_within_threshold && case.prompt_flags_stable)
        );
    }

    #[test]
    fn memory_kg_prompt_preview_approval_packet_blocks_prompt_preview_until_operator_approval() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_prompt_preview_approval_packet_report(&atom_report.atoms, true);

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.verdict,
            "blocked_until_operator_prompt_preview_approval_rollback_and_kill_switch"
        );
        assert_eq!(
            report.contract,
            MEMORY_KG_PROMPT_PREVIEW_APPROVAL_PACKET_V0_CONTRACT
        );
        assert_eq!(
            report.kg_shadow_rank_drift_contract,
            MEMORY_KG_SHADOW_RANK_DRIFT_V0_CONTRACT
        );
        assert_eq!(
            report.kg_context_injection_readiness_contract,
            MEMORY_KG_CONTEXT_INJECTION_READINESS_V0_CONTRACT
        );
        assert_eq!(
            report.approval_packet_mode,
            "draft_redacted_refs_only_no_prompt_preview"
        );
        assert!(report.drift_case_count > 0);
        assert_eq!(report.approval_item_count, report.drift_case_count);
        assert_eq!(
            report.redacted_context_ref_count,
            report.approval_item_count
        );
        assert_eq!(report.stable_case_count, report.drift_case_count);
        assert_eq!(report.drifted_case_count, 0);
        assert!(!report.operator_approval_recorded);
        assert!(!report.rollback_plan_ready);
        assert!(!report.kill_switch_ready);
        assert!(!report.approval_packet_accepted);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.drift_gate_stable);
        assert!(report.checks.approval_items_cover_drift_cases);
        assert!(report.checks.redacted_refs_present);
        assert!(report.checks.operator_approval_required);
        assert!(report.checks.prompt_preview_disabled_by_default);
        assert!(report.checks.prompt_preview_not_rendered);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled_by_default);
        assert!(report.checks.no_context_injection_performed);
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewApprovalPacketBlocker::MissingOperatorApproval)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewApprovalPacketBlocker::MissingRollbackPlan)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewApprovalPacketBlocker::MissingKillSwitch)
        );
        assert!(report.items.iter().all(|item| {
            !item.prompt_preview_included
                && !item.context_injection_allowed
                && item.operator_approval_required
                && item
                    .redacted_context_ref
                    .starts_with("kg-shadow-rank-drift-ref:")
        }));
    }

    #[test]
    fn memory_kg_prompt_preview_operator_evidence_blocks_until_evidence_is_complete() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_prompt_preview_operator_evidence_report(&atom_report.atoms, true);

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.verdict,
            "blocked_until_operator_evidence_packet_is_complete_and_signed"
        );
        assert_eq!(
            report.contract,
            MEMORY_KG_PROMPT_PREVIEW_OPERATOR_EVIDENCE_V0_CONTRACT
        );
        assert_eq!(
            report.approval_packet_contract,
            MEMORY_KG_PROMPT_PREVIEW_APPROVAL_PACKET_V0_CONTRACT
        );
        assert_eq!(report.approval_packet_status, "blocked");
        assert_eq!(
            report.evidence_gate_mode,
            "operator_evidence_requirements_only_no_prompt_preview"
        );
        assert!(!report.operator_approval_evidence_present);
        assert!(!report.rollback_plan_evidence_present);
        assert!(!report.kill_switch_evidence_present);
        assert!(!report.reviewer_identity_present);
        assert!(report.reviewer_identity_redacted);
        assert!(!report.approval_timestamp_present);
        assert!(!report.signed_approval_digest_present);
        assert!(!report.bounded_preview_scope_present);
        assert_eq!(report.required_evidence_count, 7);
        assert_eq!(
            report.missing_evidence_count,
            report.required_evidence_count
        );
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.approval_packet_contract_linked);
        assert!(report.checks.approval_packet_checks_ready);
        assert!(report.checks.approval_packet_not_accepted);
        assert!(report.checks.evidence_requirements_all_blocking);
        assert!(report.checks.operator_approval_evidence_required);
        assert!(report.checks.rollback_plan_evidence_required);
        assert!(report.checks.kill_switch_evidence_required);
        assert!(report.checks.reviewer_identity_required);
        assert!(report.checks.signed_approval_digest_required);
        assert!(report.checks.bounded_preview_scope_required);
        assert!(report.checks.prompt_preview_disabled);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled);
        assert!(report.checks.no_model_invoked);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewOperatorEvidenceBlocker::ApprovalPacketNotAccepted)
        );
        assert!(report.blockers.contains(
            &MemoryKgPromptPreviewOperatorEvidenceBlocker::MissingOperatorApprovalEvidence
        ));
        assert!(
            report.blockers.contains(
                &MemoryKgPromptPreviewOperatorEvidenceBlocker::MissingSignedApprovalDigest
            )
        );
        assert!(report.requirements.iter().all(|requirement| {
            !requirement.present
                && requirement.blocks_prompt_preview
                && requirement
                    .redacted_evidence_ref
                    .starts_with("missing:kg-prompt-preview-evidence:")
        }));
    }

    #[test]
    fn memory_kg_prompt_preview_redaction_diff_suppresses_raw_prompt_and_payload() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_prompt_preview_redaction_diff_report(&atom_report.atoms, true);

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.verdict,
            "blocked_until_redacted_diff_review_and_operator_evidence_are_complete"
        );
        assert_eq!(
            report.contract,
            MEMORY_KG_PROMPT_PREVIEW_REDACTION_DIFF_V0_CONTRACT
        );
        assert_eq!(
            report.operator_evidence_contract,
            MEMORY_KG_PROMPT_PREVIEW_OPERATOR_EVIDENCE_V0_CONTRACT
        );
        assert_eq!(report.operator_evidence_status, "blocked");
        assert_eq!(
            report.redaction_diff_mode,
            "redacted_requirement_refs_only_no_prompt_or_payload"
        );
        assert_eq!(report.required_evidence_count, 7);
        assert_eq!(
            report.missing_evidence_count,
            report.required_evidence_count
        );
        assert_eq!(report.diff_item_count, report.required_evidence_count);
        assert_eq!(report.redacted_ref_count, report.diff_item_count);
        assert_eq!(report.raw_prompt_diff_count, 0);
        assert_eq!(report.prompt_text_included_count, 0);
        assert_eq!(report.payload_text_included_count, 0);
        assert!(report.redacted_diff_reported);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.operator_evidence_contract_linked);
        assert!(report.checks.operator_evidence_checks_ready);
        assert!(report.checks.operator_evidence_missing_requirements);
        assert!(report.checks.redacted_diff_items_nonzero);
        assert!(report.checks.redacted_refs_present);
        assert!(report.checks.redacted_diff_items_cover_requirements);
        assert!(report.checks.raw_prompt_diff_suppressed);
        assert!(report.checks.prompt_text_excluded);
        assert!(report.checks.payload_text_excluded);
        assert!(report.checks.prompt_preview_disabled);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled);
        assert!(report.checks.no_model_invoked);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewRedactionDiffBlocker::OperatorEvidenceIncomplete)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewRedactionDiffBlocker::RawPromptDiffSuppressed)
        );
        assert!(report.items.iter().all(|item| {
            !item.raw_before_included
                && !item.raw_after_included
                && !item.prompt_text_included
                && !item.payload_text_included
                && !item.operator_evidence_present
                && item.blocks_prompt_preview
                && item
                    .redacted_before_ref
                    .starts_with("redacted-diff:before:")
                && item.redacted_after_ref.starts_with("redacted-diff:after:")
        }));
    }

    #[test]
    fn memory_kg_prompt_preview_rollback_kill_switch_requires_safety_evidence() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_prompt_preview_rollback_kill_switch_report(&atom_report.atoms, true);

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.verdict,
            "blocked_until_rollback_plan_and_kill_switch_evidence_are_recorded"
        );
        assert_eq!(
            report.contract,
            MEMORY_KG_PROMPT_PREVIEW_ROLLBACK_KILL_SWITCH_V0_CONTRACT
        );
        assert_eq!(
            report.redaction_diff_contract,
            MEMORY_KG_PROMPT_PREVIEW_REDACTION_DIFF_V0_CONTRACT
        );
        assert_eq!(report.redaction_diff_status, "blocked");
        assert_eq!(
            report.redaction_diff_mode,
            "redacted_requirement_refs_only_no_prompt_or_payload"
        );
        assert_eq!(report.required_evidence_count, 7);
        assert_eq!(
            report.missing_evidence_count,
            report.required_evidence_count
        );
        assert_eq!(report.required_control_count, 4);
        assert_eq!(report.missing_control_count, report.required_control_count);
        assert_eq!(report.rollback_control_count, 2);
        assert_eq!(report.kill_switch_control_count, 2);
        assert!(!report.rollback_plan_ready);
        assert!(!report.rollback_exercise_ready);
        assert!(!report.kill_switch_ready);
        assert!(!report.kill_switch_dry_run_ready);
        assert_eq!(report.redacted_ref_count, report.required_evidence_count);
        assert_eq!(report.raw_prompt_diff_count, 0);
        assert_eq!(report.prompt_text_included_count, 0);
        assert_eq!(report.payload_text_included_count, 0);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.redaction_diff_contract_linked);
        assert!(report.checks.redaction_diff_checks_ready);
        assert!(report.checks.redaction_diff_blocked);
        assert!(report.checks.only_redacted_refs_reported);
        assert!(report.checks.rollback_controls_nonzero);
        assert!(report.checks.kill_switch_controls_nonzero);
        assert!(report.checks.controls_all_missing_and_blocking);
        assert!(report.checks.rollback_plan_required);
        assert!(report.checks.rollback_exercise_required);
        assert!(report.checks.kill_switch_required);
        assert!(report.checks.kill_switch_dry_run_required);
        assert!(report.checks.prompt_preview_disabled);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled);
        assert!(report.checks.no_model_invoked);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
        assert!(report.blockers.contains(
            &MemoryKgPromptPreviewRollbackKillSwitchBlocker::RollbackPlanEvidenceMissing
        ));
        assert!(
            report.blockers.contains(
                &MemoryKgPromptPreviewRollbackKillSwitchBlocker::KillSwitchEvidenceMissing
            )
        );
        assert!(report.controls.iter().all(|control| {
            !control.present
                && control.blocks_prompt_preview
                && !control.allows_context_injection
                && control
                    .redacted_evidence_ref
                    .starts_with("missing:kg-prompt-preview-safety:")
        }));
    }

    #[test]
    fn memory_kg_prompt_preview_context_handoff_blocks_injection_until_final_evidence() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_prompt_preview_context_handoff_report(&atom_report.atoms, true);

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.verdict,
            "blocked_until_operator_evidence_safety_controls_redacted_diff_review_and_context_handoff_approval_exist"
        );
        assert_eq!(
            report.contract,
            MEMORY_KG_PROMPT_PREVIEW_CONTEXT_HANDOFF_V0_CONTRACT
        );
        assert_eq!(
            report.safety_gate_contract,
            MEMORY_KG_PROMPT_PREVIEW_ROLLBACK_KILL_SWITCH_V0_CONTRACT
        );
        assert_eq!(report.safety_gate_status, "blocked");
        assert_eq!(
            report.redaction_diff_contract,
            MEMORY_KG_PROMPT_PREVIEW_REDACTION_DIFF_V0_CONTRACT
        );
        assert_eq!(report.required_evidence_count, 7);
        assert_eq!(
            report.missing_evidence_count,
            report.required_evidence_count
        );
        assert_eq!(report.required_control_count, 4);
        assert_eq!(report.missing_control_count, report.required_control_count);
        assert_eq!(report.handoff_requirement_count, 6);
        assert_eq!(
            report.missing_handoff_requirement_count,
            report.handoff_requirement_count
        );
        assert_eq!(report.redacted_ref_count, report.required_evidence_count);
        assert_eq!(report.raw_prompt_diff_count, 0);
        assert_eq!(report.prompt_text_included_count, 0);
        assert_eq!(report.payload_text_included_count, 0);
        assert!(!report.redacted_diff_review_present);
        assert!(!report.context_handoff_approval_present);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.safety_gate_contract_linked);
        assert!(report.checks.safety_gate_checks_ready);
        assert!(report.checks.safety_gate_blocked);
        assert!(report.checks.operator_evidence_incomplete);
        assert!(report.checks.safety_controls_incomplete);
        assert!(report.checks.handoff_requirements_nonzero);
        assert!(report.checks.handoff_requirements_all_missing_and_blocking);
        assert!(report.checks.redacted_refs_only);
        assert!(report.checks.redacted_diff_review_required);
        assert!(report.checks.context_handoff_approval_required);
        assert!(report.checks.prompt_preview_disabled);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled);
        assert!(report.checks.no_model_invoked);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewContextHandoffBlocker::OperatorEvidenceIncomplete)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewContextHandoffBlocker::SafetyControlsIncomplete)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewContextHandoffBlocker::RedactedDiffReviewMissing)
        );
        assert!(
            report.blockers.contains(
                &MemoryKgPromptPreviewContextHandoffBlocker::ContextHandoffApprovalMissing
            )
        );
        assert!(report.requirements.iter().all(|requirement| {
            !requirement.present
                && requirement.blocks_context_injection
                && requirement
                    .redacted_evidence_ref
                    .starts_with("missing:kg-prompt-preview-context-handoff:")
        }));
    }

    #[test]
    fn memory_kg_prompt_preview_preflight_blocks_ci_promotion_until_gate_chain_closes() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_prompt_preview_preflight_report(&atom_report.atoms, true);

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.verdict,
            "blocked_until_prompt_preview_gate_chain_evidence_review_approval_and_ci_promotion_exist"
        );
        assert_eq!(
            report.contract,
            MEMORY_KG_PROMPT_PREVIEW_PREFLIGHT_V0_CONTRACT
        );
        assert_eq!(
            report.context_handoff_contract,
            MEMORY_KG_PROMPT_PREVIEW_CONTEXT_HANDOFF_V0_CONTRACT
        );
        assert_eq!(report.context_handoff_status, "blocked");
        assert_eq!(report.source_gate_count, 5);
        assert_eq!(report.ready_source_gate_count, report.source_gate_count);
        assert_eq!(report.blocked_source_gate_count, report.source_gate_count);
        assert_eq!(
            report.report_only_source_gate_count,
            report.source_gate_count
        );
        assert_eq!(report.required_operator_evidence_count, 7);
        assert_eq!(
            report.missing_operator_evidence_count,
            report.required_operator_evidence_count
        );
        assert_eq!(report.required_safety_control_count, 4);
        assert_eq!(
            report.missing_safety_control_count,
            report.required_safety_control_count
        );
        assert_eq!(report.required_handoff_requirement_count, 6);
        assert_eq!(
            report.missing_handoff_requirement_count,
            report.required_handoff_requirement_count
        );
        assert_eq!(report.missing_final_review_approval_count, 2);
        assert_eq!(report.required_total_preflight_requirement_count, 19);
        assert_eq!(
            report.missing_total_preflight_requirement_count,
            report.required_total_preflight_requirement_count
        );
        assert_eq!(
            report.redacted_ref_count,
            report.required_operator_evidence_count
        );
        assert_eq!(report.raw_prompt_diff_count, 0);
        assert_eq!(report.prompt_text_included_count, 0);
        assert_eq!(report.payload_text_included_count, 0);
        assert!(!report.redacted_diff_review_present);
        assert!(!report.context_handoff_approval_present);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert!(!report.ci_promotion_allowed);
        assert!(!report.preflight_execution_performed);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.source_gates_nonzero);
        assert!(report.checks.source_gates_all_linked);
        assert!(report.checks.source_gates_all_checks_ready);
        assert!(report.checks.source_gates_all_blocked);
        assert!(report.checks.source_gates_all_report_only);
        assert!(report.checks.context_handoff_contract_linked);
        assert!(report.checks.context_handoff_checks_ready);
        assert!(report.checks.context_handoff_blocked);
        assert!(report.checks.operator_evidence_incomplete);
        assert!(report.checks.safety_controls_incomplete);
        assert!(report.checks.handoff_requirements_incomplete);
        assert!(report.checks.redacted_diff_review_required);
        assert!(report.checks.context_handoff_approval_required);
        assert!(report.checks.redacted_refs_only);
        assert!(report.checks.prompt_preview_disabled);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled);
        assert!(report.checks.no_model_invoked);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
        assert!(report.checks.ci_promotion_disabled);
        assert!(report.checks.no_preflight_execution_performed);
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewPreflightBlocker::PromptPreviewGateChainBlocked)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewPreflightBlocker::CiPromotionDisabled)
        );
        assert!(report.source_gates.iter().all(|source_gate| {
            source_gate.checks_ready
                && source_gate.status == "blocked"
                && source_gate.blocks_prompt_preview
                && source_gate.blocks_context_injection
                && source_gate.report_only
        }));
    }
}
