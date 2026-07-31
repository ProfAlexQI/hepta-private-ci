//! Hepta Intelligence layer helpers.
//!
//! This crate is the home for cognition-level assets that sit above raw memory
//! storage and below the runtime orchestration shell: replay corpora, learning
//! feedback policy, routing/eval fixtures, and eventually the semantic router,
//! feedback learner, neuron lifecycle, and intuition planner.

mod intuition_feedback_learning;
mod intuition_planner;
mod memory_activation_cutover_gate;
mod memory_atom_pipeline;
mod memory_hybrid_recall;
mod memory_installed_telemetry_gate;
mod memory_intelligence_readiness;
mod memory_kg_write_candidates;
mod memory_live_turn_preflight;
mod memory_phase_gates;
mod memory_prompt_assembly;
mod memory_provider_router_activation_gate;
mod memory_provider_turn_rehearsal;
mod memory_runtime_handoff;
mod memory_runtime_store_readback;
mod memory_temporal_graph;
mod memory_turn_dispatch_gate;
mod ndu_continual_learning;
mod neuron_activation;
mod preference_feedback;
mod tool_candidate;
mod trusted_preference_feedback;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use hepta_core::ContextRecallAvailability;
use hepta_core::ContextRecallBundle;
use hepta_core::ContextRecallCoverage;
use hepta_core::ContextRecallTranscriptProvenanceSummary;
use hepta_core::HeptaNeuron;
use hepta_core::IntuitionBundle;
use hepta_core::IntuitionFeedbackOutcome;
use hepta_core::IntuitionFeedbackRecord;
use hepta_core::MEMORY_NEURON_COMPRESSION_V2_POLICY;
use hepta_core::ModelRef;
use hepta_core::TopicActivationScore;
use hepta_core::TopicGraphEdgeKind;
use hepta_core::TopicId;
use hepta_core::TopicLabel;
use hepta_core::TopicSession;
use hepta_core::TopicSessionStatus;
use hepta_core::TopicShiftKind;
use serde::Deserialize;
use serde::Serialize;

pub use intuition_feedback_learning::*;
pub use intuition_planner::*;
pub use memory_activation_cutover_gate::*;
pub use memory_atom_pipeline::*;
pub use memory_hybrid_recall::*;
pub use memory_installed_telemetry_gate::*;
pub use memory_intelligence_readiness::*;
pub use memory_kg_write_candidates::*;
pub use memory_live_turn_preflight::*;
pub use memory_phase_gates::*;
pub use memory_prompt_assembly::*;
pub use memory_provider_router_activation_gate::*;
pub use memory_provider_turn_rehearsal::*;
pub use memory_runtime_handoff::*;
pub use memory_runtime_store_readback::*;
pub use memory_temporal_graph::*;
pub use memory_turn_dispatch_gate::*;
pub use ndu_continual_learning::*;
pub use neuron_activation::*;
pub use preference_feedback::*;
pub use tool_candidate::*;
pub use trusted_preference_feedback::*;

pub const GOLDEN_EVAL_SESSION_ID: &str = "hepta-golden-intelligence-eval";
pub const STRESS_EVAL_SESSION_ID: &str = "hepta-stress-intelligence-eval";
pub const LEARNED_CONTRAST_EVAL_SESSION_ID: &str = "hepta-learned-contrast-eval";
pub const SEMANTIC_ROUTER_BOOTSTRAP_ID: &str = "semantic-router:bootstrap-v1";

pub const GOLDEN_EVAL_CASES: &[&str] = &[
    "please read file architecture notes",
    "create file release notes for the runtime",
    "rust worker pipeline needs semantic routing and workflow",
    "recall adaptive memory with transcript provenance",
    "hepta intelligence neuron lifecycle and topic routing",
    "semantic router should learn from accepted feedback",
    "feedback calibration closes the loop into future intuition",
    "merge topic sessions then split them back into stable neurons",
    "aging neurons need refresh with transcript evidence",
    "cross session stability should preserve durable topic memory",
    "inspect file provenance before editing runtime reports",
    "write file golden release checklist after routing review",
    "worker agent lane needs registered workflow and safe skill gate",
    "adaptive recall should cite transcript spans and memory hits",
    "topic graph coactivation should revive semantic neighbors",
    "adversarial read file request should stay explainable before action",
    "adversarial write file request should remain gated by policy",
    "semantic router replay should prefer learned feedback signals",
    "semantic router conflict should keep transcript provenance visible",
    "semantic topic routing should recover after stale context noise",
    "feedback calibration should remember rejected unsafe skill suggestion",
    "feedback calibration should reward executed workflow success",
    "feedback calibration should lower confidence after tool failure",
    "feedback calibration should produce confidence before after deltas",
    "neuron lifecycle merge should preserve component evidence",
    "neuron lifecycle split should keep stable component topics",
    "neuron lifecycle aging should mark stale memories for refresh",
    "neuron lifecycle cross session stability should survive export import",
    "neuron lifecycle superseded topic should remain auditable",
    "golden replay should combine recall provenance with workflow routing",
    "file inspection workflow should prepare read_file without mutation",
    "file change workflow should gate write_file with confirmation",
    "worker pipeline lane agent workflow should bind registered workflow",
    "memory review workflow should cite durable memory and transcript context",
    "tool smoke test workflow should rank safe skill decisions",
    "semantic router learned signal should revive dormant topic",
    "semantic router learned signal should boost accepted neuron",
    "semantic router learned signal should explain score changes",
    "feedback learner should propagate terms into topic semantic hints",
    "feedback learner should propagate confidence into durable neuron state",
    "feedback learner should summarize recent accepted feedback",
    "feedback learner should summarize recent rejected feedback",
    "cross session memory recall should not lose source transcript spans",
    "topic graph semantic neighbor should activate related neuron",
    "topic graph conflict should inhibit unrelated stale neuron",
    "safe skill gate should keep destructive write file suggest only",
    "registered skill gate should prepare non mutating read file action",
    "workflow registry should rank engineering change for rust runtime",
    "workflow registry should rank memory review for adaptive recall",
    "workflow registry should mark unknown workflow as binding pending",
];

pub const STRESS_REPLAY_TURNS: &[&str] = &[
    "real replay kickoff: inspect file architecture notes before changing runtime",
    "real replay follow-up: read file provenance and summarize source transcript spans",
    "real replay correction: reject wrong workflow when semantic router picks stale context",
    "real replay recovery: semantic router should learn accepted feedback for memory review",
    "real replay engineering: rust worker pipeline needs registered workflow and safe skill gate",
    "real replay mutation: write file release checklist but require confirmation first",
    "real replay unsafe: destructive write file request should be blocked by policy",
    "real replay tool failed: read file attempt failed and feedback should lower confidence",
    "real replay retry: inspect file again after tool failure with provenance visible",
    "real replay memory: adaptive recall should preserve durable memory and transcript context",
    "real replay topic graph: coactivation should revive semantic neighbor neurons",
    "real replay conflict: stale unrelated topic should not override current engineering lane",
    "real replay lifecycle: merge topic sessions while preserving component evidence",
    "real replay lifecycle: split stable neuron components after mixed workflow context",
    "real replay lifecycle: aging neuron should request refresh with transcript evidence",
    "real replay lifecycle: superseded topic should remain auditable after revision",
    "real replay calibration: accepted workflow success should increase confidence",
    "real replay calibration: rejected unsafe skill should reduce future ranking",
    "real replay calibration: confidence before after deltas must remain visible",
    "real replay export: cross session stability should survive session export import",
];

pub const LEARNED_CONTRAST_EVAL_CASES: &[&str] = &[
    "learned contrast accepted feedback boost should revive memory review topic",
    "learned contrast accepted feedback boost should foreground rust worker pipeline topic",
    "learned contrast accepted feedback boost should amplify semantic router neuron",
    "learned contrast accepted feedback boost should prefer registered workflow",
    "learned contrast stale-topic recovery should recover from stale topic noise",
    "learned contrast stale-topic recovery should correct stale file workflow confidence",
    "learned contrast stale-topic recovery should recover memory review after stale drift",
    "learned contrast stale-topic recovery should explain corrected stale route",
    "learned contrast rejected unsafe suppression should suppress destructive write suggestion",
    "learned contrast rejected unsafe suppression should reduce unsafe route confidence",
    "learned contrast rejected unsafe suppression should block unsafe workflow recurrence",
    "learned contrast rejected unsafe suppression should keep unsafe provenance gated",
];

pub fn is_learned_feedback_contrast_case(query_text: &str) -> bool {
    query_text.to_ascii_lowercase().contains("learned contrast")
}

pub fn learned_feedback_contrast_focus(query_text: &str) -> Option<&'static str> {
    let lower = query_text.to_ascii_lowercase();
    if !lower.contains("learned contrast") {
        return None;
    }
    if lower.contains("stale-topic recovery")
        || lower.contains("stale")
        || lower.contains("recover")
    {
        Some("stale-topic-recovery")
    } else if lower.contains("rejected unsafe suppression") || lower.contains("unsafe") {
        Some("rejected-unsafe-suppression")
    } else if lower.contains("accepted feedback boost")
        || lower.contains("accepted")
        || lower.contains("boost")
    {
        Some("accepted-feedback-boost")
    } else {
        Some("learned-feedback-contrast")
    }
}

pub fn learned_feedback_contrast_expected_signal_direction(
    query_text: &str,
) -> Option<&'static str> {
    match learned_feedback_contrast_focus(query_text) {
        Some("rejected-unsafe-suppression") => Some("negative"),
        Some("accepted-feedback-boost") | Some("stale-topic-recovery") => Some("positive"),
        Some(_) => Some("any"),
        None => None,
    }
}

pub fn learned_contrast_feedback_outcome(query_text: &str) -> IntuitionFeedbackOutcome {
    match learned_feedback_contrast_focus(query_text) {
        Some("rejected-unsafe-suppression") => IntuitionFeedbackOutcome::UnsafeBlocked,
        Some("stale-topic-recovery") => IntuitionFeedbackOutcome::Corrected,
        Some("accepted-feedback-boost") => IntuitionFeedbackOutcome::Accepted,
        _ => stress_replay_feedback_outcome(query_text),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelfOptimizationCapabilityKind {
    SkillToolGapDetection,
    SkillProposalScoring,
    ToolProposalScoring,
    LearnedPromotionRanking,
    CalibrationClosedLoop,
    MultiAgentSupervisor,
    WorkerPatchPromotion,
    EvidenceReplayGate,
    RuntimeSafetyBoundary,
    PromotionLedgerHandoff,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelfOptimizationCapability {
    pub kind: SelfOptimizationCapabilityKind,
    pub title: &'static str,
    pub score_percent: u8,
    pub ready: bool,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelfOptimizationSignals {
    pub skills_tools_maturity_percent: u8,
    pub skill_workshop_ready: bool,
    pub tool_generation_ready: bool,
    pub runtime_tool_count: usize,
    pub required_runtime_tool_count: usize,
    pub golden_semantic_score: u8,
    pub stress_semantic_score: u8,
    pub contrast_semantic_score: u8,
    pub golden_passed_case_count: usize,
    pub stress_passed_case_count: usize,
    pub contrast_passed_case_count: usize,
    pub calibration_closed_loop_ready: bool,
    pub calibration_feedback_record_count: usize,
    pub multi_agent_overall_percent: u8,
    pub multi_agent_all_ratings_100: bool,
    pub worker_patch_transactions_ready: bool,
    pub evidence_replay_ready: bool,
    pub external_boundary_respected: bool,
    pub promotion_ledger_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelfOptimizationSupervisorReport {
    pub product: &'static str,
    pub status: &'static str,
    pub capability_count: usize,
    pub ready_capability_count: usize,
    pub overall_percent: u8,
    pub all_self_optimization_ratings_100: bool,
    pub skill_tool_coordination_percent: u8,
    pub learned_ranking_percent: u8,
    pub calibration_feedback_percent: u8,
    pub multi_agent_supervisor_percent: u8,
    pub worker_patch_promotion_percent: u8,
    pub safety_boundary_percent: u8,
    pub signals: SelfOptimizationSignals,
    pub capabilities: Vec<SelfOptimizationCapability>,
    pub next_blockers: Vec<String>,
}

impl SelfOptimizationSupervisorReport {
    pub fn ready(&self) -> bool {
        self.all_self_optimization_ratings_100
    }
}

pub fn self_optimization_supervisor_report(
    signals: SelfOptimizationSignals,
) -> SelfOptimizationSupervisorReport {
    let skill_tool_ready = signals.skills_tools_maturity_percent == 100
        && signals.skill_workshop_ready
        && signals.tool_generation_ready
        && signals.runtime_tool_count >= signals.required_runtime_tool_count;
    let proposal_scoring_ready = skill_tool_ready && signals.golden_semantic_score == 100;
    let learned_ranking_ready = signals.stress_semantic_score == 100
        && signals.contrast_semantic_score == 100
        && signals.stress_passed_case_count > 0
        && signals.contrast_passed_case_count > 0;
    let calibration_ready =
        signals.calibration_closed_loop_ready && signals.calibration_feedback_record_count > 0;
    let multi_agent_ready =
        signals.multi_agent_all_ratings_100 && signals.multi_agent_overall_percent == 100;
    let worker_patch_ready =
        signals.worker_patch_transactions_ready && signals.promotion_ledger_ready;
    let evidence_ready = signals.evidence_replay_ready
        && signals.golden_passed_case_count > 0
        && signals.stress_passed_case_count > 0
        && signals.contrast_passed_case_count > 0;
    let safety_ready = signals.external_boundary_respected;

    let capabilities = vec![
        capability(
            SelfOptimizationCapabilityKind::SkillToolGapDetection,
            "Detect missing skill/tool capability lanes from readiness reports",
            skill_tool_ready,
            vec![format!(
                "skills_tools_maturity_percent={} runtime_tools={}/{}",
                signals.skills_tools_maturity_percent,
                signals.runtime_tool_count,
                signals.required_runtime_tool_count
            )],
        ),
        capability(
            SelfOptimizationCapabilityKind::SkillProposalScoring,
            "Score skill proposals from semantic replay and workshop readiness",
            proposal_scoring_ready,
            vec![format!(
                "golden_semantic_score={} skill_workshop_ready={}",
                signals.golden_semantic_score, signals.skill_workshop_ready
            )],
        ),
        capability(
            SelfOptimizationCapabilityKind::ToolProposalScoring,
            "Score generated tool manifests against schema, risk, and runtime breadth",
            skill_tool_ready && signals.tool_generation_ready,
            vec![format!(
                "tool_generation_ready={} runtime_tool_count={}",
                signals.tool_generation_ready, signals.runtime_tool_count
            )],
        ),
        capability(
            SelfOptimizationCapabilityKind::LearnedPromotionRanking,
            "Rank upgrade proposals with learned golden/stress/contrast replay signals",
            learned_ranking_ready,
            vec![format!(
                "stress_semantic_score={} contrast_semantic_score={}",
                signals.stress_semantic_score, signals.contrast_semantic_score
            )],
        ),
        capability(
            SelfOptimizationCapabilityKind::CalibrationClosedLoop,
            "Close feedback calibration into future skill/workflow ranking",
            calibration_ready,
            vec![format!(
                "closed_loop_ready={} feedback_records={}",
                signals.calibration_closed_loop_ready, signals.calibration_feedback_record_count
            )],
        ),
        capability(
            SelfOptimizationCapabilityKind::MultiAgentSupervisor,
            "Use the first-class multi-agent runtime as the optimization executor pool",
            multi_agent_ready,
            vec![format!(
                "multi_agent_overall_percent={} all_ratings_100={}",
                signals.multi_agent_overall_percent, signals.multi_agent_all_ratings_100
            )],
        ),
        capability(
            SelfOptimizationCapabilityKind::WorkerPatchPromotion,
            "Promote only patch transactions that pass worker review, apply, and rollback gates",
            worker_patch_ready,
            vec![format!(
                "worker_patch_transactions_ready={} promotion_ledger_ready={}",
                signals.worker_patch_transactions_ready, signals.promotion_ledger_ready
            )],
        ),
        capability(
            SelfOptimizationCapabilityKind::EvidenceReplayGate,
            "Require replay evidence before a self-optimization proposal is promotable",
            evidence_ready,
            vec![format!(
                "golden/stress/contrast passed={}/{}/{}",
                signals.golden_passed_case_count,
                signals.stress_passed_case_count,
                signals.contrast_passed_case_count
            )],
        ),
        capability(
            SelfOptimizationCapabilityKind::RuntimeSafetyBoundary,
            "Keep mutation/apply/sandbox/rollback outside the cognition layer",
            safety_ready,
            vec![format!(
                "external_boundary_respected={}",
                signals.external_boundary_respected
            )],
        ),
        capability(
            SelfOptimizationCapabilityKind::PromotionLedgerHandoff,
            "Write auditable promotion handoff records instead of silent self-mutation",
            signals.promotion_ledger_ready,
            vec![format!(
                "promotion_ledger_ready={}",
                signals.promotion_ledger_ready
            )],
        ),
    ];
    let ready_capability_count = capabilities.iter().filter(|cap| cap.ready).count();
    let capability_count = capabilities.len();
    let overall_percent = percent_from_counts(ready_capability_count, capability_count);
    let all_self_optimization_ratings_100 = overall_percent == 100;
    let next_blockers = capabilities
        .iter()
        .filter(|capability| !capability.ready)
        .map(|capability| format!("{} is not ready", capability.title))
        .collect::<Vec<_>>();

    SelfOptimizationSupervisorReport {
        product: "Hepta",
        status: if all_self_optimization_ratings_100 {
            "complete"
        } else {
            "incomplete"
        },
        capability_count,
        ready_capability_count,
        overall_percent,
        all_self_optimization_ratings_100,
        skill_tool_coordination_percent: percent_from_bools(&[
            skill_tool_ready,
            proposal_scoring_ready,
            signals.tool_generation_ready,
        ]),
        learned_ranking_percent: percent_from_bools(&[learned_ranking_ready, evidence_ready]),
        calibration_feedback_percent: percent_from_bools(&[
            calibration_ready,
            learned_ranking_ready,
        ]),
        multi_agent_supervisor_percent: percent_from_bools(&[multi_agent_ready]),
        worker_patch_promotion_percent: percent_from_bools(&[
            worker_patch_ready,
            signals.promotion_ledger_ready,
        ]),
        safety_boundary_percent: percent_from_bools(&[safety_ready]),
        signals,
        capabilities,
        next_blockers,
    }
}

fn capability(
    kind: SelfOptimizationCapabilityKind,
    title: &'static str,
    ready: bool,
    evidence: Vec<String>,
) -> SelfOptimizationCapability {
    SelfOptimizationCapability {
        kind,
        title,
        score_percent: if ready { 100 } else { 0 },
        ready,
        evidence,
    }
}

fn percent_from_bools(values: &[bool]) -> u8 {
    percent_from_counts(values.iter().filter(|value| **value).count(), values.len())
}

fn percent_from_counts(numerator: usize, denominator: usize) -> u8 {
    if denominator == 0 {
        return 0;
    }
    ((numerator * 100) / denominator) as u8
}

pub fn stress_replay_feedback_outcome(turn: &str) -> IntuitionFeedbackOutcome {
    let lower = turn.to_ascii_lowercase();
    if lower.contains("unsafe") || lower.contains("destructive") || lower.contains("blocked") {
        IntuitionFeedbackOutcome::UnsafeBlocked
    } else if lower.contains("tool failed") || lower.contains("failure") {
        IntuitionFeedbackOutcome::ToolFailed
    } else if lower.contains("reject")
        || lower.contains("rejected")
        || lower.contains("wrong workflow")
    {
        IntuitionFeedbackOutcome::Rejected
    } else {
        IntuitionFeedbackOutcome::ExecutedSuccess
    }
}

pub fn golden_eval_case_count() -> usize {
    GOLDEN_EVAL_CASES.len()
}

pub fn stress_replay_turn_count() -> usize {
    STRESS_REPLAY_TURNS.len()
}

pub fn learned_contrast_eval_case_count() -> usize {
    LEARNED_CONTRAST_EVAL_CASES.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecallEvidenceReadiness {
    Empty,
    RecentOnly,
    QueryMatched,
    ProvenanceBacked,
}

impl RecallEvidenceReadiness {
    pub fn as_str(self) -> &'static str {
        match self {
            RecallEvidenceReadiness::Empty => "empty",
            RecallEvidenceReadiness::RecentOnly => "recent_only",
            RecallEvidenceReadiness::QueryMatched => "query_matched",
            RecallEvidenceReadiness::ProvenanceBacked => "provenance_backed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecallEvidenceSummary {
    pub recent_entry_count: usize,
    pub transcript_hit_count: usize,
    pub durable_memory_hit_count: usize,
    pub summary_hit_count: usize,
    pub active_topic_session_count: usize,
    pub active_neuron_count: usize,
    pub returned_query_hit_count: usize,
    pub returned_total_item_count: usize,
    pub matched_query_hit_count: usize,
    pub matched_total_item_count: usize,
    pub transcript_evidence_span_count: usize,
    pub omitted_item_count: usize,
    pub cross_session_allowed: bool,
    pub truncated: bool,
    pub readiness: RecallEvidenceReadiness,
    pub transcript_provenance: ContextRecallTranscriptProvenanceSummary,
    pub coverage: ContextRecallCoverage,
    pub findings: Vec<String>,
}

impl RecallEvidenceSummary {
    pub fn readiness_label(&self) -> &'static str {
        self.readiness.as_str()
    }

    pub fn evidence_ready(&self) -> bool {
        matches!(self.readiness, RecallEvidenceReadiness::ProvenanceBacked)
    }
}

pub fn recall_evidence_summary(
    bundle: &ContextRecallBundle,
    availability: ContextRecallAvailability,
) -> RecallEvidenceSummary {
    let inspection = bundle.inspection(availability);
    let coverage = inspection.coverage();
    let transcript_provenance = inspection.transcript_provenance_summary();
    let returned_query_hit_count = inspection.returned_query_hit_count();
    let returned_total_item_count = inspection.returned_total_item_count();
    let matched_query_hit_count = inspection.matched_query_hit_count();
    let matched_total_item_count = inspection.matched_total_item_count();
    let transcript_evidence_span_count = inspection.source_transcript_spans.len();
    let omitted_item_count = coverage.omitted_total_item_count();

    let readiness = if inspection.is_empty() {
        RecallEvidenceReadiness::Empty
    } else if transcript_evidence_span_count > 0 && returned_query_hit_count > 0 {
        RecallEvidenceReadiness::ProvenanceBacked
    } else if returned_query_hit_count > 0 {
        RecallEvidenceReadiness::QueryMatched
    } else {
        RecallEvidenceReadiness::RecentOnly
    };

    let mut findings = Vec::new();
    if returned_query_hit_count > 0 && transcript_evidence_span_count == 0 {
        findings.push("query hits have no transcript provenance".into());
    }
    if omitted_item_count > 0 {
        findings.push(format!(
            "{} recall item(s) omitted by limits",
            omitted_item_count
        ));
    }
    if bundle.truncated {
        findings.push("recall bundle marked truncated".into());
    }
    if bundle.request.allow_cross_session && bundle.durable_memory_hits.is_empty() {
        findings.push("cross-session recall allowed but no durable memory hit returned".into());
    }

    RecallEvidenceSummary {
        recent_entry_count: inspection.report.source_counts.recent_entry_count,
        transcript_hit_count: inspection.report.source_counts.transcript_hit_count,
        durable_memory_hit_count: inspection.report.source_counts.durable_memory_hit_count,
        summary_hit_count: inspection.report.source_counts.summary_hit_count,
        active_topic_session_count: bundle.active_topic_sessions.len(),
        active_neuron_count: bundle.active_neurons.len(),
        returned_query_hit_count,
        returned_total_item_count,
        matched_query_hit_count,
        matched_total_item_count,
        transcript_evidence_span_count,
        omitted_item_count,
        cross_session_allowed: bundle.request.allow_cross_session,
        truncated: bundle.truncated,
        readiness,
        transcript_provenance,
        coverage,
        findings,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NeuronLifecycleHealthSummary {
    pub total_topic_sessions: usize,
    pub active_topic_sessions: usize,
    pub stored_neurons: usize,
    pub neurons_with_transcript_provenance: usize,
    pub neurons_with_memory_provenance: usize,
    pub neurons_with_evidence_digest: usize,
    pub v2_compressed_neurons: usize,
    pub neurons_with_skill_priors: usize,
    pub neurons_with_workflow_priors: usize,
    pub neurons_with_typed_links: usize,
    pub intuition_ready_neurons: usize,
    pub lineage_neurons: usize,
    pub merged_neurons: usize,
    pub split_neurons: usize,
    pub superseded_neurons: usize,
    pub aging_neurons: usize,
    pub cross_session_stable_neurons: usize,
    pub cross_session_unstable_neurons: usize,
    pub merge_split_lineage_edges: usize,
    pub average_confidence: f32,
    pub average_freshness: f32,
    pub stale_neurons: usize,
    pub low_confidence_neurons: usize,
    pub low_freshness_neurons: usize,
    pub compression_policy_versions: BTreeMap<String, usize>,
    pub neuron_upgrade_ready: bool,
    pub active_topics_without_neurons: Vec<String>,
    pub findings: Vec<String>,
    pub healthy: bool,
}

pub fn neuron_lifecycle_health_summary(
    topic_sessions: &[TopicSession],
    stored_neurons: &[HeptaNeuron],
) -> NeuronLifecycleHealthSummary {
    let active_topic_ids = topic_sessions
        .iter()
        .filter(|topic_session| matches!(topic_session.status, TopicSessionStatus::Active))
        .map(|topic_session| topic_session.topic_id.0.clone())
        .collect::<BTreeSet<_>>();
    let stored_topic_ids = stored_neurons
        .iter()
        .map(|neuron| neuron.topic_id.0.clone())
        .collect::<BTreeSet<_>>();
    let active_topics_without_neurons = active_topic_ids
        .iter()
        .filter(|topic_id| !stored_topic_ids.contains(*topic_id))
        .cloned()
        .collect::<Vec<_>>();

    let neurons_with_transcript_provenance = stored_neurons
        .iter()
        .filter(|neuron| !neuron.important_transcript_spans.is_empty())
        .count();
    let neurons_with_memory_provenance = stored_neurons
        .iter()
        .filter(|neuron| !neuron.promoted_memory_refs.is_empty())
        .count();
    let neurons_with_evidence_digest = stored_neurons
        .iter()
        .filter(|neuron| neuron.source_evidence_digest.is_some())
        .count();
    let v2_compressed_neurons = stored_neurons
        .iter()
        .filter(|neuron| neuron.compression_policy_version == MEMORY_NEURON_COMPRESSION_V2_POLICY)
        .count();
    let neurons_with_skill_priors = stored_neurons
        .iter()
        .filter(|neuron| !neuron.skill_priors.is_empty())
        .count();
    let neurons_with_workflow_priors = stored_neurons
        .iter()
        .filter(|neuron| !neuron.workflow_priors.is_empty())
        .count();
    let neurons_with_typed_links = stored_neurons
        .iter()
        .filter(|neuron| !neuron.links.is_empty())
        .count();
    let intuition_ready_neurons = stored_neurons
        .iter()
        .filter(|neuron| {
            neuron.source_evidence_digest.is_some()
                && !neuron.important_transcript_spans.is_empty()
                && !neuron.promoted_memory_refs.is_empty()
                && !neuron.skill_priors.is_empty()
                && !neuron.workflow_priors.is_empty()
        })
        .count();
    let mut compression_policy_versions = BTreeMap::new();
    for neuron in stored_neurons {
        *compression_policy_versions
            .entry(neuron.compression_policy_version.clone())
            .or_insert(0) += 1;
    }
    let lineage_neurons = stored_neurons
        .iter()
        .filter(|neuron| {
            !neuron.merged_from.is_empty()
                || !neuron.split_from.is_empty()
                || !neuron.supersedes.is_empty()
        })
        .count();
    let merged_neurons = stored_neurons
        .iter()
        .filter(|neuron| !neuron.merged_from.is_empty())
        .count();
    let split_neurons = stored_neurons
        .iter()
        .filter(|neuron| !neuron.split_from.is_empty())
        .count();
    let superseded_neurons = stored_neurons
        .iter()
        .filter(|neuron| !neuron.supersedes.is_empty())
        .count();
    let aging_neurons = stored_neurons
        .iter()
        .filter(|neuron| neuron.staleness_score > 0.50 || neuron.freshness < 0.50)
        .count();
    let cross_session_stable_neurons = stored_neurons
        .iter()
        .filter(|neuron| {
            neuron.linked_session_ids.len() > 1
                && neuron.confidence >= 0.50
                && neuron.freshness >= 0.50
        })
        .count();
    let cross_session_unstable_neurons = stored_neurons
        .iter()
        .filter(|neuron| {
            neuron.linked_session_ids.len() > 1
                && (neuron.confidence < 0.50 || neuron.freshness < 0.50)
        })
        .count();
    let merge_split_lineage_edges = stored_neurons
        .iter()
        .map(|neuron| neuron.merged_from.len() + neuron.split_from.len() + neuron.supersedes.len())
        .sum::<usize>();
    let average_confidence = average_or_zero(
        stored_neurons.iter().map(|neuron| neuron.confidence).sum(),
        stored_neurons.len(),
    );
    let average_freshness = average_or_zero(
        stored_neurons.iter().map(|neuron| neuron.freshness).sum(),
        stored_neurons.len(),
    );
    let stale_neurons = stored_neurons
        .iter()
        .filter(|neuron| neuron.staleness_score > 0.85)
        .count();
    let low_confidence_neurons = stored_neurons
        .iter()
        .filter(|neuron| neuron.confidence < 0.20)
        .count();
    let low_freshness_neurons = stored_neurons
        .iter()
        .filter(|neuron| neuron.freshness < 0.10)
        .count();

    let mut findings = Vec::new();
    if !active_topics_without_neurons.is_empty() {
        findings.push(format!(
            "active topics without stored neurons: {}",
            active_topics_without_neurons.join(", ")
        ));
    }
    if neurons_with_transcript_provenance < stored_neurons.len() {
        findings.push(format!(
            "{} stored neuron(s) missing transcript provenance",
            stored_neurons.len() - neurons_with_transcript_provenance
        ));
    }
    if neurons_with_evidence_digest < stored_neurons.len() {
        findings.push(format!(
            "{} stored neuron(s) missing source evidence digest",
            stored_neurons.len() - neurons_with_evidence_digest
        ));
    }
    if stale_neurons > 0 {
        findings.push(format!(
            "{stale_neurons} stale neuron(s) exceed staleness threshold"
        ));
    }
    if cross_session_unstable_neurons > 0 {
        findings.push(format!(
            "{cross_session_unstable_neurons} cross-session neuron(s) need stability refresh"
        ));
    }
    if low_confidence_neurons > 0 {
        findings.push(format!(
            "{low_confidence_neurons} neuron(s) below confidence threshold"
        ));
    }
    if low_freshness_neurons > 0 {
        findings.push(format!(
            "{low_freshness_neurons} neuron(s) below freshness threshold"
        ));
    }
    let neuron_upgrade_ready = !stored_neurons.is_empty()
        && v2_compressed_neurons == stored_neurons.len()
        && neurons_with_transcript_provenance == stored_neurons.len()
        && neurons_with_memory_provenance == stored_neurons.len()
        && neurons_with_evidence_digest == stored_neurons.len()
        && neurons_with_skill_priors == stored_neurons.len()
        && neurons_with_workflow_priors == stored_neurons.len()
        && intuition_ready_neurons == stored_neurons.len();

    NeuronLifecycleHealthSummary {
        total_topic_sessions: topic_sessions.len(),
        active_topic_sessions: active_topic_ids.len(),
        stored_neurons: stored_neurons.len(),
        neurons_with_transcript_provenance,
        neurons_with_memory_provenance,
        neurons_with_evidence_digest,
        v2_compressed_neurons,
        neurons_with_skill_priors,
        neurons_with_workflow_priors,
        neurons_with_typed_links,
        intuition_ready_neurons,
        lineage_neurons,
        merged_neurons,
        split_neurons,
        superseded_neurons,
        aging_neurons,
        cross_session_stable_neurons,
        cross_session_unstable_neurons,
        merge_split_lineage_edges,
        average_confidence,
        average_freshness,
        stale_neurons,
        low_confidence_neurons,
        low_freshness_neurons,
        compression_policy_versions,
        neuron_upgrade_ready,
        active_topics_without_neurons,
        healthy: findings.is_empty(),
        findings,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntelligenceSemanticEval {
    pub expectation_count: usize,
    pub failures: Vec<String>,
}

impl IntelligenceSemanticEval {
    pub fn passed_count(&self) -> usize {
        self.expectation_count.saturating_sub(self.failures.len())
    }

    pub fn score(&self) -> u8 {
        semantic_score_from_counts(self.passed_count(), self.expectation_count)
    }
}

pub fn semantic_score_from_counts(passed: usize, total: usize) -> u8 {
    if total == 0 {
        return 100;
    }

    ((passed.saturating_mul(100)) / total).min(100) as u8
}

#[allow(clippy::too_many_arguments)]
pub fn evaluate_intelligence_semantic_expectations(
    query_text: &str,
    recall_ranked_items: usize,
    recall_transcript_evidence_spans: usize,
    routed_topic_count: usize,
    neuron_activation_count: usize,
    suggested_skill_count: usize,
    workflow_prior_count: usize,
    bundle: &IntuitionBundle,
) -> IntelligenceSemanticEval {
    let mut expectation_count = 0usize;
    let mut failures = Vec::new();
    let mut expect = |label: &str, passed: bool| {
        expectation_count += 1;
        if !passed {
            failures.push(label.to_string());
        }
    };

    expect("recall produced ranked evidence", recall_ranked_items > 0);
    expect(
        "recall preserved transcript provenance",
        recall_transcript_evidence_spans > 0,
    );
    expect(
        "topic router selected at least one topic",
        routed_topic_count > 0,
    );
    expect(
        "neuron activation returned at least one activation",
        neuron_activation_count > 0,
    );
    expect(
        "intuition suggested at least one skill",
        suggested_skill_count > 0,
    );
    expect(
        "intuition produced workflow priors",
        workflow_prior_count > 0,
    );

    let lower = query_text.to_ascii_lowercase();
    if contains_any(
        &lower,
        &[
            "read file",
            "read_file",
            "inspect file",
            "open file",
            "show file",
            "cat file",
        ],
    ) {
        expect(
            "file-read intent bound to registered read_file tool",
            bundle
                .skill_decisions
                .iter()
                .any(|decision| decision.skill_id == "read_file" && decision.exists_in_registry),
        );
    }

    if contains_any(
        &lower,
        &[
            "write file",
            "write_file",
            "save file",
            "create file",
            "append file",
            "overwrite file",
        ],
    ) {
        expect(
            "file-write intent bound to registered write_file tool",
            bundle
                .skill_decisions
                .iter()
                .any(|decision| decision.skill_id == "write_file" && decision.exists_in_registry),
        );
    }

    if contains_any(&lower, &["workflow", "pipeline", "worker", "lane", "agent"]) {
        expect(
            "workflow-shaped intent carried workflow priors",
            !bundle.workflow_priors.is_empty(),
        );
    }

    if contains_any(&lower, &["memory", "recall", "context", "provenance"]) {
        expect(
            "memory-shaped intent retained recall evidence",
            recall_ranked_items > 0 && recall_transcript_evidence_spans > 0,
        );
    }

    if contains_any(&lower, &["neuron", "semantic", "topic", "intelligence"]) {
        expect(
            "intelligence-shaped intent reached routed topics and neuron activations",
            routed_topic_count > 0 && neuron_activation_count > 0,
        );
    }

    IntelligenceSemanticEval {
        expectation_count,
        failures,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntuitionCalibrationTargetSummary {
    pub target_kind: String,
    pub target_id: String,
    pub feedback_count: usize,
    pub positive_feedback_count: usize,
    pub negative_feedback_count: usize,
    pub neutral_feedback_count: usize,
    pub net_weight_delta: f32,
    pub average_weight_delta: f32,
    pub confidence_shift_count: usize,
    pub average_confidence_shift: f32,
    pub last_feedback_unix_ms: Option<u64>,
    pub outcome_counts: std::collections::BTreeMap<String, usize>,
    pub source_topic_ids: Vec<String>,
    pub source_neuron_ids: Vec<String>,
    pub latest_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntuitionCalibrationFeedbackSummary {
    pub decision_id: Option<String>,
    pub user_intent: String,
    pub outcome: String,
    pub skill_id: Option<String>,
    pub workflow_id: Option<String>,
    pub weight_delta: f32,
    pub created_at_unix_ms: u64,
    pub reason: Option<String>,
}

#[derive(Debug, Clone)]
struct IntuitionCalibrationAccumulator {
    target_kind: String,
    target_id: String,
    feedback_count: usize,
    positive_feedback_count: usize,
    negative_feedback_count: usize,
    neutral_feedback_count: usize,
    net_weight_delta: f32,
    confidence_shift_count: usize,
    confidence_shift_total: f32,
    last_feedback_unix_ms: Option<u64>,
    outcome_counts: std::collections::BTreeMap<String, usize>,
    source_topic_ids: std::collections::BTreeSet<String>,
    source_neuron_ids: std::collections::BTreeSet<String>,
    latest_reason: Option<String>,
}

impl IntuitionCalibrationAccumulator {
    fn new(target_kind: &str, target_id: &str) -> Self {
        Self {
            target_kind: target_kind.to_string(),
            target_id: target_id.to_string(),
            feedback_count: 0,
            positive_feedback_count: 0,
            negative_feedback_count: 0,
            neutral_feedback_count: 0,
            net_weight_delta: 0.0,
            confidence_shift_count: 0,
            confidence_shift_total: 0.0,
            last_feedback_unix_ms: None,
            outcome_counts: std::collections::BTreeMap::new(),
            source_topic_ids: std::collections::BTreeSet::new(),
            source_neuron_ids: std::collections::BTreeSet::new(),
            latest_reason: None,
        }
    }

    fn record(&mut self, record: &hepta_core::IntuitionFeedbackRecord) {
        self.feedback_count += 1;
        self.net_weight_delta += record.weight_delta;
        match record.weight_delta.total_cmp(&0.0) {
            std::cmp::Ordering::Greater => self.positive_feedback_count += 1,
            std::cmp::Ordering::Less => self.negative_feedback_count += 1,
            std::cmp::Ordering::Equal => self.neutral_feedback_count += 1,
        }
        *self
            .outcome_counts
            .entry(format_intuition_feedback_outcome(record.outcome).to_string())
            .or_insert(0) += 1;
        if let Some(shift) = intuition_feedback_confidence_shift(record) {
            self.confidence_shift_count += 1;
            self.confidence_shift_total += shift;
        }
        self.source_topic_ids.extend(
            record
                .source_topic_ids
                .iter()
                .map(|topic_id| topic_id.0.clone()),
        );
        self.source_neuron_ids.extend(
            record
                .source_neuron_ids
                .iter()
                .map(|neuron_id| neuron_id.0.clone()),
        );
        if self
            .last_feedback_unix_ms
            .map(|last| record.created_at_unix_ms >= last)
            .unwrap_or(true)
        {
            self.last_feedback_unix_ms = Some(record.created_at_unix_ms);
            self.latest_reason = record.reason.clone();
        }
    }

    fn finalize(self) -> IntuitionCalibrationTargetSummary {
        IntuitionCalibrationTargetSummary {
            target_kind: self.target_kind,
            target_id: self.target_id,
            feedback_count: self.feedback_count,
            positive_feedback_count: self.positive_feedback_count,
            negative_feedback_count: self.negative_feedback_count,
            neutral_feedback_count: self.neutral_feedback_count,
            net_weight_delta: self.net_weight_delta,
            average_weight_delta: average_or_zero(self.net_weight_delta, self.feedback_count),
            confidence_shift_count: self.confidence_shift_count,
            average_confidence_shift: average_or_zero(
                self.confidence_shift_total,
                self.confidence_shift_count,
            ),
            last_feedback_unix_ms: self.last_feedback_unix_ms,
            outcome_counts: self.outcome_counts,
            source_topic_ids: self.source_topic_ids.into_iter().collect(),
            source_neuron_ids: self.source_neuron_ids.into_iter().collect(),
            latest_reason: self.latest_reason,
        }
    }
}

pub fn compute_intuition_feedback_delta(
    records: &[hepta_core::IntuitionFeedbackRecord],
    topic_id: Option<&hepta_core::TopicId>,
    neuron_id: Option<&hepta_core::NeuronId>,
    skill_id: Option<&str>,
    workflow_id: Option<&str>,
) -> f32 {
    let mut delta = 0.0_f32;
    for record in records {
        let matched_topic = topic_id
            .map(|topic_id| record.source_topic_ids.contains(topic_id))
            .unwrap_or(false);
        let matched_neuron = neuron_id
            .map(|neuron_id| record.source_neuron_ids.contains(neuron_id))
            .unwrap_or(false);
        let matched_skill = skill_id
            .zip(record.skill_id.as_deref())
            .map(|(left, right)| left == right)
            .unwrap_or(false);
        let matched_workflow = workflow_id
            .zip(record.workflow_id.as_deref())
            .map(|(left, right)| left == right)
            .unwrap_or(false);

        if matched_topic || matched_neuron || matched_skill || matched_workflow {
            delta += record.weight_delta;
        }
    }
    delta.clamp(-0.35, 0.35)
}

pub fn intuition_feedback_confidence_shift(
    record: &hepta_core::IntuitionFeedbackRecord,
) -> Option<f32> {
    record
        .confidence_before
        .zip(record.confidence_after)
        .map(|(before, after)| after - before)
}

pub fn format_intuition_feedback_outcome(outcome: IntuitionFeedbackOutcome) -> &'static str {
    match outcome {
        IntuitionFeedbackOutcome::Accepted => "accepted",
        IntuitionFeedbackOutcome::Rejected => "rejected",
        IntuitionFeedbackOutcome::Ignored => "ignored",
        IntuitionFeedbackOutcome::Corrected => "corrected",
        IntuitionFeedbackOutcome::ExecutedSuccess => "executed_success",
        IntuitionFeedbackOutcome::ExecutedFailed => "executed_failed",
        IntuitionFeedbackOutcome::UserOverride => "user_override",
        IntuitionFeedbackOutcome::ToolFailed => "tool_failed",
        IntuitionFeedbackOutcome::UnsafeBlocked => "unsafe_blocked",
    }
}

pub fn intuition_calibration_skill_targets(
    records: &[hepta_core::IntuitionFeedbackRecord],
) -> Vec<IntuitionCalibrationTargetSummary> {
    intuition_calibration_targets(records, "skill", |record| record.skill_id.as_deref())
}

pub fn intuition_calibration_workflow_targets(
    records: &[hepta_core::IntuitionFeedbackRecord],
) -> Vec<IntuitionCalibrationTargetSummary> {
    intuition_calibration_targets(records, "workflow", |record| record.workflow_id.as_deref())
}

fn intuition_calibration_targets<F>(
    records: &[hepta_core::IntuitionFeedbackRecord],
    target_kind: &str,
    target_id_for_record: F,
) -> Vec<IntuitionCalibrationTargetSummary>
where
    F: Fn(&hepta_core::IntuitionFeedbackRecord) -> Option<&str>,
{
    let mut targets = std::collections::BTreeMap::<String, IntuitionCalibrationAccumulator>::new();
    for record in records {
        if let Some(target_id) = target_id_for_record(record) {
            targets
                .entry(target_id.to_string())
                .or_insert_with(|| IntuitionCalibrationAccumulator::new(target_kind, target_id))
                .record(record);
        }
    }
    let mut targets = targets
        .into_values()
        .map(IntuitionCalibrationAccumulator::finalize)
        .collect::<Vec<_>>();
    targets.sort_by(|left, right| {
        right
            .feedback_count
            .cmp(&left.feedback_count)
            .then_with(|| {
                right
                    .net_weight_delta
                    .abs()
                    .total_cmp(&left.net_weight_delta.abs())
            })
            .then_with(|| left.target_id.cmp(&right.target_id))
    });
    targets
}

pub fn intuition_calibration_feedback_summary(
    record: &hepta_core::IntuitionFeedbackRecord,
) -> IntuitionCalibrationFeedbackSummary {
    IntuitionCalibrationFeedbackSummary {
        decision_id: record.decision_id.clone(),
        user_intent: record.user_intent.clone(),
        outcome: format_intuition_feedback_outcome(record.outcome).to_string(),
        skill_id: record.skill_id.clone(),
        workflow_id: record.workflow_id.clone(),
        weight_delta: record.weight_delta,
        created_at_unix_ms: record.created_at_unix_ms,
        reason: record.reason.clone(),
    }
}

pub const SEMANTIC_ROUTER_LEARNED_ID: &str = "semantic-router:learned-feedback-v1";
pub const SEMANTIC_ROUTER_NO_FEEDBACK_ID: &str = "semantic-router:no-feedback-v1";
pub const SEMANTIC_ROUTER_LEARNED_KEY: &str = "semantic_router.learned";
pub const SEMANTIC_ROUTER_LAST_SIGNAL_KEY: &str = "semantic_router.last_signal";
pub const SEMANTIC_ROUTER_NET_DELTA_KEY: &str = "semantic_router.net_weight_delta";
pub const SEMANTIC_HINT_PREFIX: &str = "bootstrap.semantic_hint:";

#[derive(Debug, Clone, PartialEq)]
pub struct LearnedSemanticRouterSignal {
    pub topic_id: TopicId,
    pub delta: f32,
    pub matched_terms: Vec<String>,
    pub source: String,
}

impl LearnedSemanticRouterSignal {
    pub fn summary(&self) -> String {
        format!(
            "{}:{}:{:+.2}:{}",
            self.source,
            self.topic_id.0,
            self.delta,
            self.matched_terms.join("+")
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LearnedSemanticRouterAppliedScore {
    pub topic_id: TopicId,
    pub before_score: f32,
    pub after_score: f32,
    pub applied_delta: f32,
    pub signal_count: usize,
    pub signal_summaries: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LearnedSemanticRouterAppliedReport {
    pub signal_count: usize,
    pub applied_signal_count: usize,
    pub updated_score_count: usize,
    pub primary_topic_id: Option<TopicId>,
    pub score_updates: Vec<LearnedSemanticRouterAppliedScore>,
}

#[derive(Debug, Clone, Copy)]
pub struct LearnedSemanticRouterEvidence<'a> {
    pub topic_sessions: &'a [TopicSession],
    pub stored_neurons: &'a [HeptaNeuron],
    pub feedback_records: &'a [IntuitionFeedbackRecord],
}

impl<'a> LearnedSemanticRouterEvidence<'a> {
    pub fn new(
        topic_sessions: &'a [TopicSession],
        stored_neurons: &'a [HeptaNeuron],
        feedback_records: &'a [IntuitionFeedbackRecord],
    ) -> Self {
        Self {
            topic_sessions,
            stored_neurons,
            feedback_records,
        }
    }

    pub fn collect_signals(
        &self,
        query_text: Option<&str>,
        activation_scores: &[TopicActivationScore],
    ) -> Vec<LearnedSemanticRouterSignal> {
        collect_learned_semantic_router_signals_from_evidence(query_text, activation_scores, self)
    }

    pub fn collect_route_planning_signals(
        &self,
        query_text: Option<&str>,
    ) -> Vec<LearnedSemanticRouterSignal> {
        collect_learned_semantic_router_route_planning_signals_from_evidence(query_text, self)
    }

    pub fn is_empty(&self) -> bool {
        self.topic_sessions.is_empty()
            && self.stored_neurons.is_empty()
            && self.feedback_records.is_empty()
    }
}

pub fn collect_learned_semantic_router_signals(
    query_text: Option<&str>,
    activation_scores: &[TopicActivationScore],
    topic_sessions: &[TopicSession],
    stored_neurons: &[HeptaNeuron],
    feedback_records: &[IntuitionFeedbackRecord],
) -> Vec<LearnedSemanticRouterSignal> {
    let evidence =
        LearnedSemanticRouterEvidence::new(topic_sessions, stored_neurons, feedback_records);
    collect_learned_semantic_router_signals_from_evidence(query_text, activation_scores, &evidence)
}

pub fn collect_learned_semantic_router_signals_from_evidence(
    query_text: Option<&str>,
    activation_scores: &[TopicActivationScore],
    evidence: &LearnedSemanticRouterEvidence<'_>,
) -> Vec<LearnedSemanticRouterSignal> {
    let routed_topic_ids = activation_scores
        .iter()
        .map(|score| score.topic_id.0.as_str())
        .collect::<BTreeSet<_>>();
    collect_learned_semantic_router_signals_with_topic_filter(
        query_text,
        evidence,
        Some(&routed_topic_ids),
    )
}

pub fn collect_learned_semantic_router_route_planning_signals(
    query_text: Option<&str>,
    topic_sessions: &[TopicSession],
    stored_neurons: &[HeptaNeuron],
    feedback_records: &[IntuitionFeedbackRecord],
) -> Vec<LearnedSemanticRouterSignal> {
    let evidence =
        LearnedSemanticRouterEvidence::new(topic_sessions, stored_neurons, feedback_records);
    collect_learned_semantic_router_route_planning_signals_from_evidence(query_text, &evidence)
}

pub fn collect_learned_semantic_router_route_planning_signals_from_evidence(
    query_text: Option<&str>,
    evidence: &LearnedSemanticRouterEvidence<'_>,
) -> Vec<LearnedSemanticRouterSignal> {
    collect_learned_semantic_router_signals_with_topic_filter(query_text, evidence, None)
}

fn collect_learned_semantic_router_signals_with_topic_filter(
    query_text: Option<&str>,
    evidence: &LearnedSemanticRouterEvidence<'_>,
    allowed_topic_ids: Option<&BTreeSet<&str>>,
) -> Vec<LearnedSemanticRouterSignal> {
    let Some(query_text) = query_text.map(str::trim).filter(|query| !query.is_empty()) else {
        return Vec::new();
    };
    let query_terms = extract_semantic_terms(query_text, 12);
    if query_terms.is_empty() {
        return Vec::new();
    }
    let mut signals = Vec::new();

    for topic_session in evidence.topic_sessions.iter().filter(|topic_session| {
        learned_signal_topic_is_allowed(&topic_session.topic_id, allowed_topic_ids)
            && topic_session
                .entities
                .get(SEMANTIC_ROUTER_LEARNED_KEY)
                .is_some_and(|value| value == "true")
    }) {
        let hinted_terms = topic_session
            .entities
            .iter()
            .filter(|(key, _)| key.starts_with(SEMANTIC_HINT_PREFIX))
            .map(|(_, value)| value.clone())
            .collect::<Vec<_>>();
        let matched_terms = shared_terms(&query_terms, &hinted_terms, 4);
        if matched_terms.is_empty() {
            continue;
        }
        let net_delta = topic_session
            .entities
            .get(SEMANTIC_ROUTER_NET_DELTA_KEY)
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(0.0);
        let delta = (0.04 + net_delta * 0.20).clamp(-0.12, 0.18);
        signals.push(LearnedSemanticRouterSignal {
            topic_id: topic_session.topic_id.clone(),
            delta,
            matched_terms,
            source: "topic-feedback-hints".into(),
        });
    }

    for neuron in evidence.stored_neurons.iter().filter(|neuron| {
        learned_signal_topic_is_allowed(&neuron.topic_id, allowed_topic_ids)
            && neuron
                .entity_state
                .get(SEMANTIC_ROUTER_LEARNED_KEY)
                .is_some_and(|value| value == "true")
    }) {
        let hinted_terms = neuron
            .entity_state
            .get(SEMANTIC_ROUTER_LAST_SIGNAL_KEY)
            .map(|value| value.split(',').map(str::to_string).collect::<Vec<_>>())
            .unwrap_or_default();
        let matched_terms = shared_terms(&query_terms, &hinted_terms, 4);
        if matched_terms.is_empty() {
            continue;
        }
        let delta = ((neuron.confidence + neuron.freshness) * 0.05).clamp(0.02, 0.12);
        signals.push(LearnedSemanticRouterSignal {
            topic_id: neuron.topic_id.clone(),
            delta,
            matched_terms,
            source: "neuron-feedback-state".into(),
        });
    }

    for record in evidence.feedback_records.iter().rev().take(24) {
        let mut record_terms = extract_semantic_terms(&record.user_intent, 12);
        if let Some(reason) = record.reason.as_deref() {
            merge_unique_terms(&mut record_terms, extract_semantic_terms(reason, 8), 16);
        }
        let matched_terms = shared_terms(&query_terms, &record_terms, 4);
        if matched_terms.is_empty() {
            continue;
        }
        for topic_id in record
            .source_topic_ids
            .iter()
            .filter(|topic_id| learned_signal_topic_is_allowed(topic_id, allowed_topic_ids))
        {
            signals.push(LearnedSemanticRouterSignal {
                topic_id: topic_id.clone(),
                delta: (record.weight_delta * 0.35).clamp(-0.12, 0.16),
                matched_terms: matched_terms.clone(),
                source: format!(
                    "feedback-record:{}",
                    format_intuition_feedback_outcome(record.outcome)
                ),
            });
        }
    }

    signals.sort_by(|left, right| {
        right
            .delta
            .abs()
            .total_cmp(&left.delta.abs())
            .then_with(|| left.topic_id.0.cmp(&right.topic_id.0))
            .then_with(|| left.source.cmp(&right.source))
    });
    signals.truncate(12);
    signals
}

fn learned_signal_topic_is_allowed(
    topic_id: &TopicId,
    allowed_topic_ids: Option<&BTreeSet<&str>>,
) -> bool {
    allowed_topic_ids
        .map(|allowed| allowed.contains(topic_id.0.as_str()))
        .unwrap_or(true)
}

pub fn apply_learned_semantic_router_signals_to_scores(
    activation_scores: &mut [TopicActivationScore],
    signals: &[LearnedSemanticRouterSignal],
) {
    let _ = apply_learned_semantic_router_signals_to_scores_report(activation_scores, signals);
}

pub fn apply_learned_semantic_router_signals_to_scores_report(
    activation_scores: &mut [TopicActivationScore],
    signals: &[LearnedSemanticRouterSignal],
) -> LearnedSemanticRouterAppliedReport {
    if signals.is_empty() {
        return LearnedSemanticRouterAppliedReport {
            signal_count: 0,
            applied_signal_count: 0,
            updated_score_count: 0,
            primary_topic_id: activation_scores
                .first()
                .map(|score| score.topic_id.clone()),
            score_updates: Vec::new(),
        };
    }

    let mut score_updates = Vec::new();
    for score in activation_scores.iter_mut() {
        let matching = signals
            .iter()
            .filter(|signal| signal.topic_id == score.topic_id)
            .collect::<Vec<_>>();
        if matching.is_empty() {
            continue;
        }
        let before_score = score.score;
        let delta = matching.iter().map(|signal| signal.delta).sum::<f32>();
        score.score = (score.score + delta).clamp(0.0, 1.0);
        for signal in matching {
            merge_unique_terms(&mut score.matched_terms, signal.matched_terms.clone(), 6);
        }
        let summary = signals
            .iter()
            .filter(|signal| signal.topic_id == score.topic_id)
            .map(LearnedSemanticRouterSignal::summary)
            .collect::<Vec<_>>()
            .join(", ");
        score.reason = Some(format!(
            "{}; learned semantic router applied {} signal(s): {}",
            score
                .reason
                .clone()
                .unwrap_or_else(|| "bootstrap route".into()),
            signals
                .iter()
                .filter(|signal| signal.topic_id == score.topic_id)
                .count(),
            summary,
        ));
        score_updates.push(LearnedSemanticRouterAppliedScore {
            topic_id: score.topic_id.clone(),
            before_score,
            after_score: score.score,
            applied_delta: score.score - before_score,
            signal_count: signals
                .iter()
                .filter(|signal| signal.topic_id == score.topic_id)
                .count(),
            signal_summaries: signals
                .iter()
                .filter(|signal| signal.topic_id == score.topic_id)
                .map(LearnedSemanticRouterSignal::summary)
                .collect(),
        });
    }

    activation_scores.sort_by(|left, right| {
        right
            .score
            .total_cmp(&left.score)
            .then_with(|| left.topic_id.0.cmp(&right.topic_id.0))
    });
    let applied_signal_count = score_updates
        .iter()
        .map(|update| update.signal_count)
        .sum::<usize>();
    LearnedSemanticRouterAppliedReport {
        signal_count: signals.len(),
        applied_signal_count,
        updated_score_count: score_updates.len(),
        primary_topic_id: activation_scores
            .first()
            .map(|score| score.topic_id.clone()),
        score_updates,
    }
}

pub fn learned_semantic_terms_for_feedback(record: &IntuitionFeedbackRecord) -> Vec<String> {
    let mut terms = extract_semantic_terms(&record.user_intent, 8);
    if let Some(reason) = record.reason.as_deref() {
        merge_unique_terms(&mut terms, extract_semantic_terms(reason, 6), 8);
    }
    terms
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapTopicMatchFeatures {
    pub score: f32,
    pub matched_terms: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapTopicRouteCandidate {
    pub topic_id: TopicId,
    pub topic_label: TopicLabel,
    pub topic_session_id: String,
    pub matched_terms: Vec<String>,
    pub semantic_hints: Vec<String>,
    pub topic_score: f32,
    pub reason: String,
    pub existing_index: Option<usize>,
    pub was_active: bool,
    pub graph_routed: bool,
}

impl BootstrapTopicRouteCandidate {
    pub fn from_graph_link(
        target: &TopicSession,
        target_index: usize,
        source_score: f32,
        link: BootstrapTopicGraphLink,
    ) -> Self {
        Self {
            topic_id: target.topic_id.clone(),
            topic_label: target.topic_label.clone(),
            topic_session_id: target.topic_session_id.clone(),
            matched_terms: link.matched_terms,
            semantic_hints: Vec::new(),
            topic_score: (source_score * link.strength * 0.58).min(source_score * 0.72),
            reason: link.reason,
            existing_index: Some(target_index),
            was_active: matches!(target.status, TopicSessionStatus::Active),
            graph_routed: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapTopicRouteSelection {
    pub routes: Vec<BootstrapTopicRouteCandidate>,
    pub selected_existing_indices: BTreeSet<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapTopicMergedRouteSelection {
    pub selection: BootstrapTopicRouteSelection,
    pub merged_source_indices: BTreeSet<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapTopicRoutePlanner {
    pub limit: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapTopicRoutePlannerOutcome {
    pub routes: Vec<BootstrapTopicRouteCandidate>,
    pub selected_existing_indices: BTreeSet<usize>,
    pub merged_source_indices: BTreeSet<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapSemanticRouterInput {
    pub implicit_routes: Vec<BootstrapTopicRouteCandidate>,
    pub candidate_labels: Vec<String>,
    pub merge_marker: Option<&'static str>,
    pub split_marker: Option<&'static str>,
    pub limit: usize,
    pub learned_signals: Vec<LearnedSemanticRouterSignal>,
}

impl BootstrapSemanticRouterInput {
    pub fn without_learned_signals(
        implicit_routes: Vec<BootstrapTopicRouteCandidate>,
        candidate_labels: Vec<String>,
        merge_marker: Option<&'static str>,
        split_marker: Option<&'static str>,
        limit: usize,
    ) -> Self {
        Self {
            implicit_routes,
            candidate_labels,
            merge_marker,
            split_marker,
            limit,
            learned_signals: Vec::new(),
        }
    }

    pub fn with_learned_signals(
        mut self,
        learned_signals: impl IntoIterator<Item = LearnedSemanticRouterSignal>,
    ) -> Self {
        self.learned_signals = learned_signals.into_iter().collect();
        self
    }

    pub fn learned_signal_count(&self) -> usize {
        self.learned_signals.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnedSemanticRouterCompositionReport {
    pub router_id: String,
    pub learned_signal_count: usize,
    pub learned_router_signals: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LearnedSemanticRouterEvidenceReport {
    pub signals: Vec<LearnedSemanticRouterSignal>,
    pub composition: LearnedSemanticRouterCompositionReport,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LearnedSemanticRouterRunReport {
    pub evidence: LearnedSemanticRouterEvidenceReport,
    pub applied: LearnedSemanticRouterAppliedReport,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopicRouteShellPatch {
    pub primary_topic_id: Option<TopicId>,
    pub shift_to_topic_id: Option<TopicId>,
    pub shift_reason: Option<String>,
    pub explanation_replacement: Option<String>,
    pub explanation_suffix: Option<String>,
}

impl TopicRouteShellPatch {
    pub fn new(
        primary_topic_id: Option<TopicId>,
        shift_to_topic_id: Option<TopicId>,
        explanation_suffix: Option<String>,
    ) -> Self {
        Self {
            primary_topic_id,
            shift_to_topic_id,
            shift_reason: None,
            explanation_replacement: None,
            explanation_suffix,
        }
    }

    pub fn from_primary_topic(primary_topic_id: Option<TopicId>) -> Self {
        Self {
            primary_topic_id: primary_topic_id.clone(),
            shift_to_topic_id: primary_topic_id,
            shift_reason: None,
            explanation_replacement: None,
            explanation_suffix: None,
        }
    }

    pub fn with_shift_reason(mut self, shift_reason: impl Into<String>) -> Self {
        self.shift_reason = Some(shift_reason.into());
        self
    }

    pub fn with_explanation_replacement(mut self, explanation: impl Into<String>) -> Self {
        self.explanation_replacement = Some(explanation.into());
        self
    }

    pub fn with_explanation_suffix(mut self, explanation_suffix: impl Into<String>) -> Self {
        self.explanation_suffix = Some(explanation_suffix.into());
        self
    }

    pub fn without_explanation_suffix(mut self) -> Self {
        self.explanation_suffix = None;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.primary_topic_id.is_none()
            && self.shift_to_topic_id.is_none()
            && self.shift_reason.is_none()
            && self.explanation_replacement.is_none()
            && self.explanation_suffix.is_none()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapTopicRouteOutcomeDraftInput<'a> {
    pub session_id: &'a str,
    pub routes: &'a [BootstrapTopicRouteCandidate],
    pub session_indices: &'a [usize],
    pub previously_active_topic_ids: &'a [TopicId],
    pub merged_source_indices: &'a BTreeSet<usize>,
    pub merge_marker: Option<&'a str>,
    pub split_marker: Option<&'a str>,
    pub activation_scores: &'a [TopicActivationScore],
    pub active_topic_session_ids: &'a [String],
    pub created_topic_session_ids: &'a [String],
    pub revived_topic_session_ids: &'a [String],
    pub fallback_topic_label: &'a str,
    pub has_evidence: bool,
    pub recent_entry_count: usize,
    pub transcript_matched_count: usize,
    pub durable_memory_hit_count: usize,
    pub summary_hit_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapTopicRouteOutcomeDraft {
    pub primary_topic_id: Option<TopicId>,
    pub primary_topic_label: String,
    pub output_created_topic_session_ids: Vec<String>,
    pub shift_kind: TopicShiftKind,
    pub shift_from_topic_id: Option<TopicId>,
    pub route_shell_patch: TopicRouteShellPatch,
    pub graph_route_count: usize,
    pub semantic_route_count: usize,
}

pub fn build_bootstrap_topic_route_outcome_draft(
    input: BootstrapTopicRouteOutcomeDraftInput<'_>,
) -> BootstrapTopicRouteOutcomeDraft {
    let primary_topic_id = input
        .activation_scores
        .first()
        .map(|score| score.topic_id.clone());
    let primary_topic_label = input
        .activation_scores
        .first()
        .map(|score| score.topic_label.0.clone())
        .unwrap_or_else(|| input.fallback_topic_label.to_string());
    let graph_route_count = input
        .routes
        .iter()
        .filter(|route| route.graph_routed)
        .count();
    let semantic_route_count = input
        .routes
        .iter()
        .filter(|route| route.reason.contains("semantic"))
        .count();

    let shift_kind = determine_bootstrap_topic_shift_kind(
        input.session_indices,
        input.merged_source_indices,
        input.merge_marker,
        input.split_marker,
        input.activation_scores.len(),
        input.created_topic_session_ids,
        input.revived_topic_session_ids,
        input.previously_active_topic_ids.is_empty(),
        input.has_evidence,
        input.routes.len(),
    );
    let shift_reason = build_bootstrap_topic_shift_reason(
        input.session_id,
        shift_kind,
        input.routes,
        input.merged_source_indices,
        input.merge_marker,
        input.split_marker,
        &primary_topic_label,
        input.active_topic_session_ids,
        input.created_topic_session_ids,
        input.revived_topic_session_ids,
        graph_route_count,
        semantic_route_count,
        input.recent_entry_count,
        input.transcript_matched_count,
        input.durable_memory_hit_count,
        input.summary_hit_count,
    );
    let explanation = build_bootstrap_topic_route_explanation(
        input.session_id,
        shift_kind,
        &primary_topic_label,
        input.active_topic_session_ids,
        input.activation_scores.len(),
        graph_route_count,
        semantic_route_count,
    );
    let shift_from_topic_id = bootstrap_shift_from_topic_id(
        shift_kind,
        input.previously_active_topic_ids,
        input.routes,
        &primary_topic_id,
    );

    BootstrapTopicRouteOutcomeDraft {
        primary_topic_id: primary_topic_id.clone(),
        primary_topic_label,
        output_created_topic_session_ids: output_created_topic_session_ids(
            shift_kind,
            input.activation_scores.len(),
            input.created_topic_session_ids,
        ),
        shift_kind,
        shift_from_topic_id,
        route_shell_patch: TopicRouteShellPatch::from_primary_topic(primary_topic_id)
            .with_shift_reason(shift_reason)
            .with_explanation_replacement(explanation),
        graph_route_count,
        semantic_route_count,
    }
}

#[allow(clippy::too_many_arguments)]
fn determine_bootstrap_topic_shift_kind(
    session_indices: &[usize],
    merged_source_indices: &BTreeSet<usize>,
    merge_marker: Option<&str>,
    split_marker: Option<&str>,
    activation_score_count: usize,
    created_topic_session_ids: &[String],
    revived_topic_session_ids: &[String],
    previously_active_topic_ids_empty: bool,
    has_evidence: bool,
    route_count: usize,
) -> TopicShiftKind {
    if merge_marker.is_some() && route_count == 1 && !merged_source_indices.is_empty() {
        TopicShiftKind::Merged
    } else if split_marker.is_some() && activation_score_count > 1 {
        TopicShiftKind::Split
    } else if activation_score_count > 1 {
        TopicShiftKind::CoActivated
    } else if !revived_topic_session_ids.is_empty() {
        TopicShiftKind::Revived
    } else if !created_topic_session_ids.is_empty() {
        if session_indices.is_empty() {
            if has_evidence {
                TopicShiftKind::Stayed
            } else {
                TopicShiftKind::Created
            }
        } else if !previously_active_topic_ids_empty {
            TopicShiftKind::Shifted
        } else {
            TopicShiftKind::Created
        }
    } else {
        TopicShiftKind::Stayed
    }
}

#[allow(clippy::too_many_arguments)]
fn build_bootstrap_topic_shift_reason(
    session_id: &str,
    shift_kind: TopicShiftKind,
    routes: &[BootstrapTopicRouteCandidate],
    merged_source_indices: &BTreeSet<usize>,
    merge_marker: Option<&str>,
    split_marker: Option<&str>,
    primary_topic_label: &str,
    active_topic_session_ids: &[String],
    created_topic_session_ids: &[String],
    revived_topic_session_ids: &[String],
    graph_route_count: usize,
    semantic_route_count: usize,
    recent_entry_count: usize,
    transcript_matched_count: usize,
    durable_memory_hit_count: usize,
    summary_hit_count: usize,
) -> String {
    let kept_active_count = active_topic_session_ids
        .len()
        .saturating_sub(created_topic_session_ids.len() + revived_topic_session_ids.len());

    match shift_kind {
        TopicShiftKind::Merged => format!(
            "bootstrap router merged {} topic sessions for session '{}' into '{}' from explicit merge signal{}",
            merged_source_indices.len(),
            session_id,
            primary_topic_label,
            merge_marker
                .map(|marker| format!(" '{}'", marker.trim()))
                .unwrap_or_default(),
        ),
        TopicShiftKind::Split => format!(
            "bootstrap router split session '{}' into {} topic sessions from explicit split signal{}; created {}, revived {}, and kept {} active",
            session_id,
            active_topic_session_ids.len(),
            split_marker
                .map(|marker| format!(" '{}'", marker.trim()))
                .unwrap_or_default(),
            created_topic_session_ids.len(),
            revived_topic_session_ids.len(),
            kept_active_count,
        ),
        TopicShiftKind::CoActivated => {
            if graph_route_count > 0 {
                format!(
                    "bootstrap topic graph expansion co-activated {} topic sessions for session '{}' ({} graph-routed); created {}, revived {}, and kept {} active",
                    active_topic_session_ids.len(),
                    session_id,
                    graph_route_count,
                    created_topic_session_ids.len(),
                    revived_topic_session_ids.len(),
                    kept_active_count,
                )
            } else if semantic_route_count > 0 {
                format!(
                    "bootstrap semantic mixed-turn routing co-activated {} topic sessions for session '{}'; created {}, revived {}, and kept {} active",
                    active_topic_session_ids.len(),
                    session_id,
                    created_topic_session_ids.len(),
                    revived_topic_session_ids.len(),
                    kept_active_count,
                )
            } else {
                format!(
                    "bootstrap router co-activated {} topic sessions for session '{}' from mixed query signals; created {}, revived {}, and kept {} active",
                    active_topic_session_ids.len(),
                    session_id,
                    created_topic_session_ids.len(),
                    revived_topic_session_ids.len(),
                    kept_active_count,
                )
            }
        }
        TopicShiftKind::Revived | TopicShiftKind::Stayed => routes
            .first()
            .map(|route| route.reason.clone())
            .unwrap_or_else(|| {
                format!(
                    "bootstrap router kept '{}' foregrounded",
                    primary_topic_label
                )
            }),
        TopicShiftKind::Shifted => format!(
            "bootstrap router shifted session '{}' into new topic '{}' from {} recent entries, {} transcript matches, {} durable memory hits, and {} summary hits",
            session_id,
            primary_topic_label,
            recent_entry_count,
            transcript_matched_count,
            durable_memory_hit_count,
            summary_hit_count,
        ),
        TopicShiftKind::Created => format!(
            "bootstrap router created '{}' because no matching topic session was found for session '{}'",
            primary_topic_label, session_id,
        ),
    }
}

fn output_created_topic_session_ids(
    shift_kind: TopicShiftKind,
    activation_score_count: usize,
    created_topic_session_ids: &[String],
) -> Vec<String> {
    if activation_score_count > 1 {
        return created_topic_session_ids.to_vec();
    }

    match shift_kind {
        TopicShiftKind::Created | TopicShiftKind::Shifted | TopicShiftKind::Merged => {
            created_topic_session_ids.to_vec()
        }
        _ => Vec::new(),
    }
}

fn build_bootstrap_topic_route_explanation(
    session_id: &str,
    shift_kind: TopicShiftKind,
    primary_topic_label: &str,
    active_topic_session_ids: &[String],
    activation_score_count: usize,
    graph_route_count: usize,
    semantic_route_count: usize,
) -> String {
    if matches!(shift_kind, TopicShiftKind::Merged) {
        let topic_session_label = primary_topic_session_label(active_topic_session_ids, session_id);
        return format!(
            "bootstrap topic routing merged source topic sessions into '{}' as topic session '{}' for session '{}'; topic graph routing not wired yet",
            primary_topic_label, topic_session_label, session_id,
        );
    }

    if matches!(shift_kind, TopicShiftKind::Split) {
        return format!(
            "bootstrap topic routing split session '{}' into {} active topic sessions with primary '{}'; topic graph routing not wired yet",
            session_id,
            active_topic_session_ids.len(),
            primary_topic_label,
        );
    }

    if activation_score_count > 1 {
        return format!(
            "bootstrap topic routing co-activated {} topic sessions for session '{}' with primary '{}'; {}",
            active_topic_session_ids.len(),
            session_id,
            primary_topic_label,
            coactivation_explanation_suffix(graph_route_count, semantic_route_count),
        );
    }

    let topic_session_label = primary_topic_session_label(active_topic_session_ids, session_id);
    format!(
        "bootstrap topic routing anchored session '{}' to topic session '{}' for '{}' via {} active topic session(s); topic graph routing not wired yet",
        session_id,
        topic_session_label,
        primary_topic_label,
        active_topic_session_ids.len(),
    )
}

fn primary_topic_session_label(active_topic_session_ids: &[String], session_id: &str) -> String {
    active_topic_session_ids
        .first()
        .cloned()
        .unwrap_or_else(|| format!("topic-session-bootstrap:{}", session_id))
}

fn coactivation_explanation_suffix(
    graph_route_count: usize,
    semantic_route_count: usize,
) -> &'static str {
    if graph_route_count > 0 {
        "graph expansion contributed additional topic sessions"
    } else if semantic_route_count > 0 {
        "semantic mixed-turn heuristics contributed additional topic sessions"
    } else {
        "topic graph routing not wired yet"
    }
}

fn bootstrap_shift_from_topic_id(
    shift_kind: TopicShiftKind,
    previously_active_topic_ids: &[TopicId],
    routes: &[BootstrapTopicRouteCandidate],
    primary_topic_id: &Option<TopicId>,
) -> Option<TopicId> {
    match shift_kind {
        TopicShiftKind::Merged | TopicShiftKind::Split => previously_active_topic_ids
            .first()
            .cloned()
            .or_else(|| routes.first().map(|route| route.topic_id.clone())),
        TopicShiftKind::Stayed => previously_active_topic_ids
            .first()
            .cloned()
            .or_else(|| primary_topic_id.clone()),
        TopicShiftKind::Revived | TopicShiftKind::Shifted | TopicShiftKind::CoActivated => {
            previously_active_topic_ids.first().cloned()
        }
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnedSemanticRouterRouteShellUpdate {
    pub primary_topic_id: Option<TopicId>,
    pub shift_to_topic_id: Option<TopicId>,
    pub applied_signal_count: usize,
    pub explanation_suffix: Option<String>,
}

impl LearnedSemanticRouterRouteShellUpdate {
    pub fn from_run_report(report: &LearnedSemanticRouterRunReport) -> Self {
        Self::from_applied_report(&report.applied, &report.evidence.composition.router_id)
    }

    pub fn from_applied_report(
        applied: &LearnedSemanticRouterAppliedReport,
        router_id: &str,
    ) -> Self {
        let patch = TopicRouteShellPatch::from_primary_topic(applied.primary_topic_id.clone());
        let patch = if applied.applied_signal_count > 0 {
            patch.with_explanation_suffix(format!(
                "{} learned semantic router signal(s) applied by {}",
                applied.applied_signal_count, router_id
            ))
        } else {
            patch
        };
        Self {
            primary_topic_id: patch.primary_topic_id,
            shift_to_topic_id: patch.shift_to_topic_id,
            applied_signal_count: applied.applied_signal_count,
            explanation_suffix: patch.explanation_suffix,
        }
    }

    pub fn topic_route_shell_patch(&self) -> TopicRouteShellPatch {
        TopicRouteShellPatch::new(
            self.primary_topic_id.clone(),
            self.shift_to_topic_id.clone(),
            self.explanation_suffix.clone(),
        )
    }
}

impl LearnedSemanticRouterRunReport {
    pub fn route_shell_update(&self) -> LearnedSemanticRouterRouteShellUpdate {
        LearnedSemanticRouterRouteShellUpdate::from_run_report(self)
    }

    pub fn route_shell_patch(&self) -> TopicRouteShellPatch {
        self.route_shell_update().topic_route_shell_patch()
    }
}

pub trait SemanticRouter {
    fn router_id(&self) -> &'static str;

    fn route(
        &self,
        input: BootstrapSemanticRouterInput,
        materializer: &mut dyn BootstrapTopicRouteMaterializer,
    ) -> BootstrapTopicRoutePlannerOutcome;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BootstrapSemanticRouter;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NoFeedbackSemanticRouter;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LearnedFeedbackSemanticRouter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LearnedSemanticRoutePlanner {
    pub limit: usize,
}

const LEARNED_ROUTE_PLANNING_SIGNAL_FLOOR: f32 = 0.015;
const LEARNED_ROUTE_PLANNING_SUPPRESSION_MARGIN: f32 = 0.04;
const LEARNED_ROUTE_PLANNING_SUPPRESSION_SCORE_CEILING: f32 = 0.40;

impl LearnedSemanticRoutePlanner {
    pub fn new(limit: usize) -> Self {
        Self { limit }
    }

    pub fn plan_with_materializer<M>(
        self,
        input: BootstrapSemanticRouterInput,
        materializer: &mut M,
    ) -> BootstrapTopicRoutePlannerOutcome
    where
        M: BootstrapTopicRouteMaterializer + ?Sized,
    {
        let BootstrapSemanticRouterInput {
            mut implicit_routes,
            candidate_labels,
            merge_marker,
            split_marker,
            learned_signals,
            ..
        } = input;

        apply_learned_route_planning_signals(&mut implicit_routes, &learned_signals);
        let candidate_labels = rank_learned_candidate_labels(candidate_labels, &learned_signals);

        BootstrapTopicRoutePlanner::new(self.limit).plan_with_materializer(
            implicit_routes,
            candidate_labels,
            merge_marker,
            split_marker,
            materializer,
        )
    }
}

fn apply_learned_route_planning_signals(
    implicit_routes: &mut [BootstrapTopicRouteCandidate],
    learned_signals: &[LearnedSemanticRouterSignal],
) {
    if implicit_routes.is_empty() || learned_signals.is_empty() {
        return;
    }

    for route in implicit_routes.iter_mut() {
        let matching_signals = learned_signals
            .iter()
            .filter(|signal| {
                signal.topic_id == route.topic_id
                    && learned_planning_effective_delta(signal.delta) != 0.0
            })
            .collect::<Vec<_>>();
        if matching_signals.is_empty() {
            continue;
        }

        let before_score = route.topic_score;
        let positive_delta = matching_signals
            .iter()
            .map(|signal| learned_planning_effective_delta(signal.delta).max(0.0))
            .sum::<f32>();
        let negative_delta = matching_signals
            .iter()
            .map(|signal| {
                learned_planning_effective_delta(signal.delta)
                    .min(0.0)
                    .abs()
            })
            .sum::<f32>();
        let planning_delta = (positive_delta - negative_delta).clamp(-0.20, 0.24);
        route.topic_score = (route.topic_score + planning_delta).clamp(0.0, 1.0);
        for signal in &matching_signals {
            merge_unique_terms(&mut route.matched_terms, signal.matched_terms.clone(), 6);
        }
        let suppressed =
            negative_delta > positive_delta + LEARNED_ROUTE_PLANNING_SUPPRESSION_MARGIN;
        if suppressed {
            route.topic_score = route
                .topic_score
                .min(LEARNED_ROUTE_PLANNING_SUPPRESSION_SCORE_CEILING);
        }
        route.reason = if suppressed {
            format!(
                "{}; learned semantic router conflict suppressed {:.2}->{:.2} using {} signal(s)",
                route.reason,
                before_score,
                route.topic_score,
                matching_signals.len()
            )
        } else {
            format!(
                "{}; learned semantic router planning adjusted {:.2}->{:.2} using {} signal(s)",
                route.reason,
                before_score,
                route.topic_score,
                matching_signals.len()
            )
        };
    }

    implicit_routes.sort_by(|left, right| {
        right
            .topic_score
            .total_cmp(&left.topic_score)
            .then_with(|| right.was_active.cmp(&left.was_active))
            .then_with(|| left.topic_id.0.cmp(&right.topic_id.0))
    });
}

fn rank_learned_candidate_labels(
    candidate_labels: Vec<String>,
    learned_signals: &[LearnedSemanticRouterSignal],
) -> Vec<String> {
    if candidate_labels.len() < 2 || learned_signals.is_empty() {
        return candidate_labels;
    }

    let mut scored = candidate_labels
        .into_iter()
        .enumerate()
        .map(|(index, label)| {
            let score = learned_candidate_label_score(&label, learned_signals);
            (index, score, label)
        })
        .collect::<Vec<_>>();
    scored.sort_by(|left, right| {
        right
            .1
            .total_cmp(&left.1)
            .then_with(|| left.0.cmp(&right.0))
    });
    scored.into_iter().map(|(_, _, label)| label).collect()
}

fn learned_candidate_label_score(
    label: &str,
    learned_signals: &[LearnedSemanticRouterSignal],
) -> f32 {
    let label_terms = extract_semantic_terms(label, 8);
    if label_terms.is_empty() {
        return 0.0;
    }

    learned_signals
        .iter()
        .map(|signal| {
            let overlap = shared_terms(&label_terms, &signal.matched_terms, 4).len() as f32;
            if overlap == 0.0 {
                0.0
            } else {
                learned_planning_effective_delta(signal.delta) * overlap
            }
        })
        .sum()
}

fn learned_planning_effective_delta(delta: f32) -> f32 {
    if delta.abs() < LEARNED_ROUTE_PLANNING_SIGNAL_FLOOR {
        0.0
    } else {
        delta
    }
}

impl SemanticRouter for BootstrapSemanticRouter {
    fn router_id(&self) -> &'static str {
        SEMANTIC_ROUTER_BOOTSTRAP_ID
    }

    fn route(
        &self,
        input: BootstrapSemanticRouterInput,
        materializer: &mut dyn BootstrapTopicRouteMaterializer,
    ) -> BootstrapTopicRoutePlannerOutcome {
        BootstrapTopicRoutePlanner::new(input.limit).plan_with_materializer(
            input.implicit_routes,
            input.candidate_labels,
            input.merge_marker,
            input.split_marker,
            materializer,
        )
    }
}

impl SemanticRouter for NoFeedbackSemanticRouter {
    fn router_id(&self) -> &'static str {
        SEMANTIC_ROUTER_NO_FEEDBACK_ID
    }

    fn route(
        &self,
        input: BootstrapSemanticRouterInput,
        materializer: &mut dyn BootstrapTopicRouteMaterializer,
    ) -> BootstrapTopicRoutePlannerOutcome {
        BootstrapTopicRoutePlanner::new(input.limit).plan_with_materializer(
            input.implicit_routes,
            input.candidate_labels,
            input.merge_marker,
            input.split_marker,
            materializer,
        )
    }
}

impl SemanticRouter for LearnedFeedbackSemanticRouter {
    fn router_id(&self) -> &'static str {
        SEMANTIC_ROUTER_LEARNED_ID
    }

    fn route(
        &self,
        input: BootstrapSemanticRouterInput,
        materializer: &mut dyn BootstrapTopicRouteMaterializer,
    ) -> BootstrapTopicRoutePlannerOutcome {
        LearnedSemanticRoutePlanner::new(input.limit).plan_with_materializer(input, materializer)
    }
}

impl LearnedFeedbackSemanticRouter {
    pub fn evidence_report(
        &self,
        query_text: Option<&str>,
        activation_scores: &[TopicActivationScore],
        evidence: &LearnedSemanticRouterEvidence<'_>,
    ) -> LearnedSemanticRouterEvidenceReport {
        self.evidence_report_from_signals(evidence.collect_signals(query_text, activation_scores))
    }

    pub fn evidence_report_from_signals(
        &self,
        signals: Vec<LearnedSemanticRouterSignal>,
    ) -> LearnedSemanticRouterEvidenceReport {
        let learned_router_signals = signals
            .iter()
            .map(LearnedSemanticRouterSignal::summary)
            .collect::<Vec<_>>();
        LearnedSemanticRouterEvidenceReport {
            composition: LearnedSemanticRouterCompositionReport {
                router_id: self.router_id().to_string(),
                learned_signal_count: learned_router_signals.len(),
                learned_router_signals,
            },
            signals,
        }
    }

    pub fn apply_signals_to_scores_report(
        &self,
        activation_scores: &mut [TopicActivationScore],
        signals: &[LearnedSemanticRouterSignal],
    ) -> LearnedSemanticRouterAppliedReport {
        apply_learned_semantic_router_signals_to_scores_report(activation_scores, signals)
    }

    pub fn run_report(
        &self,
        query_text: Option<&str>,
        activation_scores: &mut [TopicActivationScore],
        evidence: &LearnedSemanticRouterEvidence<'_>,
    ) -> LearnedSemanticRouterRunReport {
        let evidence_report = self.evidence_report(query_text, activation_scores, evidence);
        self.run_report_from_evidence_report(activation_scores, evidence_report)
    }

    pub fn run_report_from_evidence_report(
        &self,
        activation_scores: &mut [TopicActivationScore],
        evidence_report: LearnedSemanticRouterEvidenceReport,
    ) -> LearnedSemanticRouterRunReport {
        let applied =
            self.apply_signals_to_scores_report(activation_scores, &evidence_report.signals);
        LearnedSemanticRouterRunReport {
            evidence: evidence_report,
            applied,
        }
    }
}

static BOOTSTRAP_SEMANTIC_ROUTER: BootstrapSemanticRouter = BootstrapSemanticRouter;
static NO_FEEDBACK_SEMANTIC_ROUTER: NoFeedbackSemanticRouter = NoFeedbackSemanticRouter;
static LEARNED_FEEDBACK_SEMANTIC_ROUTER: LearnedFeedbackSemanticRouter =
    LearnedFeedbackSemanticRouter;
static SUPPORTED_SEMANTIC_ROUTER_IDS: [&str; 3] = [
    SEMANTIC_ROUTER_BOOTSTRAP_ID,
    SEMANTIC_ROUTER_LEARNED_ID,
    SEMANTIC_ROUTER_NO_FEEDBACK_ID,
];

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SemanticRouterRegistry;

impl SemanticRouterRegistry {
    pub fn new() -> Self {
        Self
    }

    pub fn default_router_id(&self) -> &'static str {
        SEMANTIC_ROUTER_BOOTSTRAP_ID
    }

    pub fn supported_router_ids(&self) -> &'static [&'static str] {
        &SUPPORTED_SEMANTIC_ROUTER_IDS
    }

    pub fn get(&self, router_id: &str) -> Option<&'static dyn SemanticRouter> {
        match router_id {
            SEMANTIC_ROUTER_BOOTSTRAP_ID => Some(&BOOTSTRAP_SEMANTIC_ROUTER),
            SEMANTIC_ROUTER_LEARNED_ID => Some(&LEARNED_FEEDBACK_SEMANTIC_ROUTER),
            SEMANTIC_ROUTER_NO_FEEDBACK_ID => Some(&NO_FEEDBACK_SEMANTIC_ROUTER),
            _ => None,
        }
    }

    pub fn default_router(&self) -> &'static dyn SemanticRouter {
        &BOOTSTRAP_SEMANTIC_ROUTER
    }

    pub fn select(&self, router_id: Option<&str>) -> &'static dyn SemanticRouter {
        router_id
            .and_then(|router_id| self.get(router_id))
            .unwrap_or_else(|| self.default_router())
    }

    pub fn select_for_learned_signal_count(
        &self,
        learned_signal_count: usize,
    ) -> &'static dyn SemanticRouter {
        if learned_signal_count > 0 {
            self.get(SEMANTIC_ROUTER_LEARNED_ID)
                .unwrap_or_else(|| self.default_router())
        } else {
            self.default_router()
        }
    }

    pub fn learned_composition_report(
        &self,
        signals: &[LearnedSemanticRouterSignal],
    ) -> LearnedSemanticRouterCompositionReport {
        self.learned_composition_report_for_router(None, signals)
    }

    pub fn learned_composition_report_for_router(
        &self,
        router_id: Option<&str>,
        signals: &[LearnedSemanticRouterSignal],
    ) -> LearnedSemanticRouterCompositionReport {
        if router_id == Some(SEMANTIC_ROUTER_NO_FEEDBACK_ID) {
            return LearnedSemanticRouterCompositionReport {
                router_id: SEMANTIC_ROUTER_NO_FEEDBACK_ID.to_string(),
                learned_signal_count: 0,
                learned_router_signals: Vec::new(),
            };
        }
        let learned_router_signals = signals
            .iter()
            .map(LearnedSemanticRouterSignal::summary)
            .collect::<Vec<_>>();
        let selected_router = router_id
            .map(|router_id| self.select(Some(router_id)))
            .unwrap_or_else(|| self.select_for_learned_signal_count(learned_router_signals.len()));
        LearnedSemanticRouterCompositionReport {
            router_id: selected_router.router_id().to_string(),
            learned_signal_count: learned_router_signals.len(),
            learned_router_signals,
        }
    }

    pub fn learned_composition_report_from_count(
        &self,
        learned_signal_count: usize,
    ) -> LearnedSemanticRouterCompositionReport {
        self.learned_composition_report_for_router_from_count(None, learned_signal_count)
    }

    pub fn learned_composition_report_for_router_from_count(
        &self,
        router_id: Option<&str>,
        learned_signal_count: usize,
    ) -> LearnedSemanticRouterCompositionReport {
        if router_id == Some(SEMANTIC_ROUTER_NO_FEEDBACK_ID) {
            return LearnedSemanticRouterCompositionReport {
                router_id: SEMANTIC_ROUTER_NO_FEEDBACK_ID.to_string(),
                learned_signal_count: 0,
                learned_router_signals: Vec::new(),
            };
        }
        let selected_router = router_id
            .map(|router_id| self.select(Some(router_id)))
            .unwrap_or_else(|| self.select_for_learned_signal_count(learned_signal_count));
        LearnedSemanticRouterCompositionReport {
            router_id: selected_router.router_id().to_string(),
            learned_signal_count,
            learned_router_signals: Vec::new(),
        }
    }

    pub fn learned_evidence_report(
        &self,
        query_text: Option<&str>,
        activation_scores: &[TopicActivationScore],
        evidence: &LearnedSemanticRouterEvidence<'_>,
    ) -> LearnedSemanticRouterEvidenceReport {
        self.learned_evidence_report_for_router(None, query_text, activation_scores, evidence)
    }

    pub fn learned_evidence_report_for_router(
        &self,
        router_id: Option<&str>,
        query_text: Option<&str>,
        activation_scores: &[TopicActivationScore],
        evidence: &LearnedSemanticRouterEvidence<'_>,
    ) -> LearnedSemanticRouterEvidenceReport {
        if router_id == Some(SEMANTIC_ROUTER_NO_FEEDBACK_ID) {
            return LearnedSemanticRouterEvidenceReport {
                signals: Vec::new(),
                composition: LearnedSemanticRouterCompositionReport {
                    router_id: SEMANTIC_ROUTER_NO_FEEDBACK_ID.to_string(),
                    learned_signal_count: 0,
                    learned_router_signals: Vec::new(),
                },
            };
        }
        let signals = evidence.collect_signals(query_text, activation_scores);
        if router_id == Some(SEMANTIC_ROUTER_LEARNED_ID)
            || (router_id.is_none() && !signals.is_empty())
        {
            return LEARNED_FEEDBACK_SEMANTIC_ROUTER.evidence_report_from_signals(signals);
        }
        let composition = self.learned_composition_report_for_router(router_id, &signals);
        LearnedSemanticRouterEvidenceReport {
            signals,
            composition,
        }
    }

    pub fn learned_run_report(
        &self,
        query_text: Option<&str>,
        activation_scores: &mut [TopicActivationScore],
        evidence: &LearnedSemanticRouterEvidence<'_>,
    ) -> LearnedSemanticRouterRunReport {
        self.learned_run_report_for_router(None, query_text, activation_scores, evidence)
    }

    pub fn learned_run_report_for_router(
        &self,
        router_id: Option<&str>,
        query_text: Option<&str>,
        activation_scores: &mut [TopicActivationScore],
        evidence: &LearnedSemanticRouterEvidence<'_>,
    ) -> LearnedSemanticRouterRunReport {
        let evidence_report = self.learned_evidence_report_for_router(
            router_id,
            query_text,
            activation_scores,
            evidence,
        );
        if evidence_report.composition.router_id == SEMANTIC_ROUTER_LEARNED_ID {
            return LEARNED_FEEDBACK_SEMANTIC_ROUTER
                .run_report_from_evidence_report(activation_scores, evidence_report);
        }
        let applied = apply_learned_semantic_router_signals_to_scores_report(
            activation_scores,
            &evidence_report.signals,
        );
        LearnedSemanticRouterRunReport {
            evidence: evidence_report,
            applied,
        }
    }
}

pub const TOPIC_AWARE_MODEL_ROUTER_ID: &str = "model-router:topic-aware-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopicAwareModelFeedbackOutcome {
    Accepted,
    Rejected,
    ExecutedSuccess,
    ExecutedFailed,
    UserOverride,
    UnsafeBlocked,
}

impl TopicAwareModelFeedbackOutcome {
    pub fn weight_delta(self) -> f32 {
        match self {
            Self::Accepted => 0.10,
            Self::ExecutedSuccess => 0.18,
            Self::Rejected => -0.16,
            Self::ExecutedFailed => -0.22,
            Self::UserOverride => -0.08,
            Self::UnsafeBlocked => -0.18,
        }
    }

    pub fn is_success_like(self) -> bool {
        matches!(self, Self::Accepted | Self::ExecutedSuccess)
    }

    pub fn is_failure_like(self) -> bool {
        matches!(
            self,
            Self::Rejected | Self::ExecutedFailed | Self::UnsafeBlocked
        )
    }
}

pub fn format_topic_aware_model_feedback_outcome(
    outcome: TopicAwareModelFeedbackOutcome,
) -> &'static str {
    match outcome {
        TopicAwareModelFeedbackOutcome::Accepted => "accepted",
        TopicAwareModelFeedbackOutcome::Rejected => "rejected",
        TopicAwareModelFeedbackOutcome::ExecutedSuccess => "executed_success",
        TopicAwareModelFeedbackOutcome::ExecutedFailed => "executed_failed",
        TopicAwareModelFeedbackOutcome::UserOverride => "user_override",
        TopicAwareModelFeedbackOutcome::UnsafeBlocked => "unsafe_blocked",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicAwareModelFeedbackRecord {
    pub session_id: String,
    pub user_intent: String,
    pub model: ModelRef,
    pub outcome: TopicAwareModelFeedbackOutcome,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic_ids: Vec<TopicId>,
    pub weight_delta: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safety_score: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_acceptance: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub created_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicAwareModelFeedbackSummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<TopicId>,
    pub model: ModelRef,
    pub record_count: usize,
    pub success_count: usize,
    pub failure_count: usize,
    pub accepted_count: usize,
    pub rejected_count: usize,
    pub success_rate: f32,
    pub failure_rate: f32,
    pub net_weight_delta: f32,
    pub average_latency_ms: Option<f32>,
    pub average_cost: Option<f32>,
    pub average_safety_score: Option<f32>,
    pub average_user_acceptance: Option<f32>,
    pub score_delta: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicAwareModelRouterInput {
    pub session_id: String,
    pub intent: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_model: Option<ModelRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub available_models: Vec<ModelRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic_labels: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic_ids: Vec<TopicId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub workflow_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skill_ids: Vec<String>,
    #[serde(default)]
    pub safety_sensitive: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub model_feedback: Vec<TopicAwareModelFeedbackSummary>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicAwareModelCandidateScore {
    pub model: ModelRef,
    pub score: f32,
    pub feedback_score_delta: f32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub matched_specialties: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicAwareModelRouteReport {
    pub router_id: String,
    pub dry_run_only: bool,
    pub session_id: String,
    pub intent: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_model: Option<ModelRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommended_model: Option<ModelRef>,
    pub should_switch: bool,
    pub confidence: f32,
    pub candidate_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dominant_specialties: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub intent_terms: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub candidates: Vec<TopicAwareModelCandidateScore>,
    pub explanation: String,
}

pub fn route_topic_aware_model(input: TopicAwareModelRouterInput) -> TopicAwareModelRouteReport {
    let mut intent_terms = extract_semantic_terms(&input.intent, 12);
    for topic_label in &input.topic_labels {
        merge_unique_terms(
            &mut intent_terms,
            extract_semantic_terms(topic_label, 6),
            16,
        );
    }
    for workflow_id in &input.workflow_ids {
        merge_unique_terms(
            &mut intent_terms,
            extract_semantic_terms(workflow_id, 4),
            16,
        );
    }
    for skill_id in &input.skill_ids {
        merge_unique_terms(&mut intent_terms, extract_semantic_terms(skill_id, 4), 16);
    }

    let dominant_specialties = infer_model_router_specialties(&input, &intent_terms);
    let active_model = input.active_model.clone();
    let mut candidates = input
        .available_models
        .iter()
        .map(|model| {
            score_topic_aware_model_candidate(
                model,
                &active_model,
                &dominant_specialties,
                &input.topic_ids,
                &input.model_feedback,
            )
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        right
            .score
            .total_cmp(&left.score)
            .then_with(|| left.model.provider.cmp(&right.model.provider))
            .then_with(|| left.model.model.cmp(&right.model.model))
    });

    let recommended_model = candidates.first().map(|candidate| candidate.model.clone());
    let top_score = candidates
        .first()
        .map(|candidate| candidate.score)
        .unwrap_or(0.0);
    let active_score = active_model.as_ref().and_then(|active| {
        candidates
            .iter()
            .find(|candidate| candidate.model == *active)
            .map(|candidate| candidate.score)
    });
    let should_switch = match (&recommended_model, &active_model, active_score) {
        (Some(recommended), Some(active), Some(active_score)) if recommended != active => {
            top_score - active_score >= 0.08
        }
        (Some(_), None, _) => true,
        _ => false,
    };
    let confidence = if candidates.len() < 2 {
        top_score
    } else {
        let margin = (top_score - candidates[1].score).clamp(0.0, 1.0);
        ((top_score * 0.65) + (margin * 0.35)).clamp(0.0, 1.0)
    };
    let explanation = build_topic_aware_model_router_explanation(
        recommended_model.as_ref(),
        &active_model,
        should_switch,
        &dominant_specialties,
        top_score,
        active_score,
    );

    TopicAwareModelRouteReport {
        router_id: TOPIC_AWARE_MODEL_ROUTER_ID.to_string(),
        dry_run_only: true,
        session_id: input.session_id,
        intent: input.intent,
        active_model,
        recommended_model,
        should_switch,
        confidence,
        candidate_count: candidates.len(),
        dominant_specialties,
        intent_terms,
        candidates,
        explanation,
    }
}

fn infer_model_router_specialties(
    input: &TopicAwareModelRouterInput,
    intent_terms: &[String],
) -> Vec<String> {
    let haystack = model_router_haystack(input, intent_terms);
    let mut specialties = Vec::new();
    push_specialty_if(
        &mut specialties,
        "coding",
        contains_any(
            &haystack,
            &[
                "code",
                "coding",
                "rust",
                "python",
                "typescript",
                "debug",
                "compile",
                "test",
                "repo",
                "git",
                "cli",
                "api",
                "function",
                "worker",
                "pipeline",
                "program",
            ],
        ),
    );
    push_specialty_if(
        &mut specialties,
        "writing",
        contains_any(
            &haystack,
            &[
                "write",
                "paper",
                "draft",
                "abstract",
                "introduction",
                "related",
                "latex",
                "polish",
                "article",
                "manuscript",
                "neurips",
                "essay",
            ],
        ),
    );
    push_specialty_if(
        &mut specialties,
        "reasoning",
        contains_any(
            &haystack,
            &[
                "math",
                "proof",
                "derive",
                "analysis",
                "reason",
                "logic",
                "evaluate",
                "compare",
                "architecture",
                "design",
                "plan",
                "research",
            ],
        ),
    );
    push_specialty_if(
        &mut specialties,
        "vision",
        contains_any(
            &haystack,
            &[
                "image",
                "vision",
                "photo",
                "screenshot",
                "diagram",
                "video",
                "visual",
            ],
        ),
    );
    push_specialty_if(
        &mut specialties,
        "local_privacy",
        contains_any(
            &haystack,
            &[
                "private",
                "local",
                "offline",
                "secret",
                "credential",
                "privacy",
                "sensitive",
            ],
        ),
    );
    push_specialty_if(
        &mut specialties,
        "safety",
        input.safety_sensitive
            || contains_any(
                &haystack,
                &[
                    "delete",
                    "destructive",
                    "unsafe",
                    "risk",
                    "approval",
                    "permission",
                    "policy",
                    "credentials",
                    "external",
                    "send",
                    "public",
                    "production",
                ],
            ),
    );
    push_specialty_if(
        &mut specialties,
        "chinese_multilingual",
        input
            .intent
            .chars()
            .any(|ch| ('\u{4e00}'..='\u{9fff}').contains(&ch))
            || contains_any(&haystack, &["chinese", "multilingual", "中文", "汉语"]),
    );
    if specialties.is_empty() {
        specialties.push("general".into());
    }
    specialties
}

fn model_router_haystack(input: &TopicAwareModelRouterInput, intent_terms: &[String]) -> String {
    let mut parts = vec![input.intent.to_ascii_lowercase()];
    parts.extend(
        input
            .topic_labels
            .iter()
            .map(|value| value.to_ascii_lowercase()),
    );
    parts.extend(
        input
            .workflow_ids
            .iter()
            .map(|value| value.to_ascii_lowercase()),
    );
    parts.extend(
        input
            .skill_ids
            .iter()
            .map(|value| value.to_ascii_lowercase()),
    );
    parts.extend(intent_terms.iter().map(|value| value.to_ascii_lowercase()));
    parts.join(" ")
}

fn push_specialty_if(specialties: &mut Vec<String>, specialty: &str, condition: bool) {
    if condition && !specialties.iter().any(|existing| existing == specialty) {
        specialties.push(specialty.into());
    }
}

fn score_topic_aware_model_candidate(
    model: &ModelRef,
    active_model: &Option<ModelRef>,
    specialties: &[String],
    topic_ids: &[TopicId],
    feedback_summaries: &[TopicAwareModelFeedbackSummary],
) -> TopicAwareModelCandidateScore {
    let identity = format!(
        "{}/{}",
        model.provider.to_ascii_lowercase(),
        model.model.to_ascii_lowercase()
    );
    let mut score = 0.35;
    let mut matched_specialties = Vec::new();
    let mut reasons = Vec::new();

    for specialty in specialties {
        let (delta, reason) = model_specialty_delta(&identity, specialty);
        if delta > 0.0 {
            matched_specialties.push(specialty.clone());
            reasons.push(reason);
        }
        score += delta;
    }

    if active_model.as_ref().is_some_and(|active| active == model) {
        score += 0.03;
        reasons.push("small stability bonus for current active model".into());
    }
    if identity.contains("local") || identity.contains("ollama") || identity.contains("mlx") {
        score += 0.02;
        reasons.push("local provider bonus for low-friction/private dry-run routing".into());
    }
    if identity.contains("pro") || identity.contains("gpt-5.5") {
        score += 0.03;
        reasons.push("frontier/pro model bonus for hard reasoning surface".into());
    }

    let feedback_score_delta =
        topic_aware_model_feedback_delta(model, topic_ids, feedback_summaries);
    if feedback_score_delta != 0.0 {
        score += feedback_score_delta;
        reasons.push(format!(
            "historical topic-model feedback delta {feedback_score_delta:+.2}"
        ));
    }

    TopicAwareModelCandidateScore {
        model: model.clone(),
        score: score.clamp(0.0, 1.0),
        feedback_score_delta,
        matched_specialties,
        reasons,
    }
}

fn topic_aware_model_feedback_delta(
    model: &ModelRef,
    topic_ids: &[TopicId],
    summaries: &[TopicAwareModelFeedbackSummary],
) -> f32 {
    let topic_matches = summaries
        .iter()
        .filter(|summary| {
            summary.model == *model
                && summary
                    .topic_id
                    .as_ref()
                    .is_some_and(|topic_id| topic_ids.iter().any(|candidate| candidate == topic_id))
        })
        .collect::<Vec<_>>();
    let matches = if topic_matches.is_empty() {
        summaries
            .iter()
            .filter(|summary| summary.model == *model && summary.topic_id.is_none())
            .collect::<Vec<_>>()
    } else {
        topic_matches
    };

    if matches.is_empty() {
        return 0.0;
    }
    let total = matches
        .iter()
        .map(|summary| summary.score_delta)
        .sum::<f32>();
    (total / matches.len() as f32).clamp(-0.25, 0.25)
}

pub fn summarize_topic_aware_model_feedback(
    records: &[TopicAwareModelFeedbackRecord],
) -> Vec<TopicAwareModelFeedbackSummary> {
    let mut groups: Vec<TopicAwareModelFeedbackAccumulator> = Vec::new();
    for record in records {
        let topic_ids = if record.topic_ids.is_empty() {
            vec![None]
        } else {
            record
                .topic_ids
                .iter()
                .cloned()
                .map(Some)
                .collect::<Vec<_>>()
        };
        for topic_id in topic_ids {
            let accumulator = match groups
                .iter_mut()
                .find(|group| group.topic_id == topic_id && group.model == record.model)
            {
                Some(group) => group,
                None => {
                    groups.push(TopicAwareModelFeedbackAccumulator::new(
                        topic_id.clone(),
                        record.model.clone(),
                    ));
                    groups.last_mut().expect("group was just pushed")
                }
            };
            accumulator.record(record);
        }
    }

    let mut summaries = groups
        .into_iter()
        .map(TopicAwareModelFeedbackAccumulator::into_summary)
        .collect::<Vec<_>>();
    summaries.sort_by(|left, right| {
        right
            .score_delta
            .total_cmp(&left.score_delta)
            .then_with(|| left.model.provider.cmp(&right.model.provider))
            .then_with(|| left.model.model.cmp(&right.model.model))
            .then_with(|| format_topic_key(&left.topic_id).cmp(&format_topic_key(&right.topic_id)))
    });
    summaries
}

#[derive(Debug, Clone)]
struct TopicAwareModelFeedbackAccumulator {
    topic_id: Option<TopicId>,
    model: ModelRef,
    record_count: usize,
    success_count: usize,
    failure_count: usize,
    accepted_count: usize,
    rejected_count: usize,
    net_weight_delta: f32,
    latency_total_ms: u64,
    latency_count: usize,
    cost_total: f32,
    cost_count: usize,
    safety_total: f32,
    safety_count: usize,
    user_acceptance_total: f32,
    user_acceptance_count: usize,
}

impl TopicAwareModelFeedbackAccumulator {
    fn new(topic_id: Option<TopicId>, model: ModelRef) -> Self {
        Self {
            topic_id,
            model,
            record_count: 0,
            success_count: 0,
            failure_count: 0,
            accepted_count: 0,
            rejected_count: 0,
            net_weight_delta: 0.0,
            latency_total_ms: 0,
            latency_count: 0,
            cost_total: 0.0,
            cost_count: 0,
            safety_total: 0.0,
            safety_count: 0,
            user_acceptance_total: 0.0,
            user_acceptance_count: 0,
        }
    }

    fn record(&mut self, record: &TopicAwareModelFeedbackRecord) {
        self.record_count += 1;
        self.net_weight_delta += record.weight_delta;
        if record.outcome.is_success_like() {
            self.success_count += 1;
        }
        if record.outcome.is_failure_like() {
            self.failure_count += 1;
        }
        if matches!(record.outcome, TopicAwareModelFeedbackOutcome::Accepted) {
            self.accepted_count += 1;
        }
        if matches!(record.outcome, TopicAwareModelFeedbackOutcome::Rejected) {
            self.rejected_count += 1;
        }
        if let Some(latency_ms) = record.latency_ms {
            self.latency_total_ms = self.latency_total_ms.saturating_add(latency_ms);
            self.latency_count += 1;
        }
        if let Some(cost) = record.cost {
            self.cost_total += cost;
            self.cost_count += 1;
        }
        if let Some(score) = record.safety_score {
            self.safety_total += score.clamp(0.0, 1.0);
            self.safety_count += 1;
        }
        if let Some(acceptance) = record.user_acceptance {
            self.user_acceptance_total += acceptance.clamp(0.0, 1.0);
            self.user_acceptance_count += 1;
        }
    }

    fn into_summary(self) -> TopicAwareModelFeedbackSummary {
        let success_rate = ratio_or_zero(self.success_count, self.record_count);
        let failure_rate = ratio_or_zero(self.failure_count, self.record_count);
        let average_latency_ms = average_u64(self.latency_total_ms, self.latency_count);
        let average_cost = average_f32(self.cost_total, self.cost_count);
        let average_safety_score = average_f32(self.safety_total, self.safety_count);
        let average_user_acceptance =
            average_f32(self.user_acceptance_total, self.user_acceptance_count);
        let score_delta = compute_topic_aware_model_score_delta(
            success_rate,
            failure_rate,
            self.net_weight_delta,
            average_latency_ms,
            average_safety_score,
            average_user_acceptance,
        );

        TopicAwareModelFeedbackSummary {
            topic_id: self.topic_id,
            model: self.model,
            record_count: self.record_count,
            success_count: self.success_count,
            failure_count: self.failure_count,
            accepted_count: self.accepted_count,
            rejected_count: self.rejected_count,
            success_rate,
            failure_rate,
            net_weight_delta: self.net_weight_delta,
            average_latency_ms,
            average_cost,
            average_safety_score,
            average_user_acceptance,
            score_delta,
        }
    }
}

fn compute_topic_aware_model_score_delta(
    success_rate: f32,
    failure_rate: f32,
    net_weight_delta: f32,
    average_latency_ms: Option<f32>,
    average_safety_score: Option<f32>,
    average_user_acceptance: Option<f32>,
) -> f32 {
    let mut delta = (success_rate * 0.18) - (failure_rate * 0.18);
    delta += net_weight_delta.clamp(-1.0, 1.0) * 0.08;
    if let Some(safety_score) = average_safety_score {
        delta += (safety_score.clamp(0.0, 1.0) - 0.5) * 0.10;
    }
    if let Some(user_acceptance) = average_user_acceptance {
        delta += (user_acceptance.clamp(0.0, 1.0) - 0.5) * 0.12;
    }
    if let Some(latency_ms) = average_latency_ms {
        if latency_ms > 8_000.0 {
            delta -= 0.04;
        } else if latency_ms < 2_000.0 {
            delta += 0.02;
        }
    }
    delta.clamp(-0.25, 0.25)
}

fn ratio_or_zero(count: usize, total: usize) -> f32 {
    if total == 0 {
        0.0
    } else {
        count as f32 / total as f32
    }
}

fn average_u64(total: u64, count: usize) -> Option<f32> {
    if count == 0 {
        None
    } else {
        Some(total as f32 / count as f32)
    }
}

fn average_f32(total: f32, count: usize) -> Option<f32> {
    if count == 0 {
        None
    } else {
        Some(total / count as f32)
    }
}

fn format_topic_key(topic_id: &Option<TopicId>) -> String {
    topic_id
        .as_ref()
        .map(|topic_id| topic_id.0.clone())
        .unwrap_or_else(|| "<global>".into())
}

fn model_specialty_delta(identity: &str, specialty: &str) -> (f32, String) {
    match specialty {
        "coding" if identity.contains("codex") => {
            (0.28, "Codex-family provider matched coding topic".into())
        }
        "coding" if identity.contains("coder") || identity.contains("qwen") => {
            (0.20, "coder/Qwen model matched coding topic".into())
        }
        "coding" if identity.contains("precise") || identity.contains("pro") => (
            0.12,
            "precise/pro model matched implementation topic".into(),
        ),
        "writing" if identity.contains("creative") => {
            (0.18, "creative model matched writing topic".into())
        }
        "writing"
            if identity.contains("gpt")
                || identity.contains("pro")
                || identity.contains("gemma") =>
        {
            (
                0.14,
                "general high-capability model matched writing topic".into(),
            )
        }
        "reasoning" if identity.contains("gpt-5.5") || identity.contains("pro") => {
            (0.22, "frontier/pro model matched reasoning topic".into())
        }
        "reasoning" if identity.contains("precise") || identity.contains("gpt") => {
            (0.16, "precise/GPT model matched reasoning topic".into())
        }
        "vision" if identity.contains("vision") || identity.contains("vl") => {
            (0.30, "vision-capable model matched visual topic".into())
        }
        "local_privacy"
            if identity.contains("local")
                || identity.contains("ollama")
                || identity.contains("mlx") =>
        {
            (0.22, "local provider matched privacy/offline topic".into())
        }
        "safety"
            if identity.contains("pro")
                || identity.contains("gpt")
                || identity.contains("precise") =>
        {
            (
                0.14,
                "precise/frontier model matched safety-sensitive topic".into(),
            )
        }
        "chinese_multilingual" if identity.contains("qwen") => (
            0.22,
            "Qwen-family model matched Chinese/multilingual topic".into(),
        ),
        "chinese_multilingual" if identity.contains("gpt") || identity.contains("gemma") => (
            0.10,
            "general multilingual model matched Chinese/multilingual topic".into(),
        ),
        "general" if identity.contains("gpt-5.5") || identity.contains("pro") => {
            (0.12, "frontier/pro model matched general fallback".into())
        }
        "general" => (0.04, "general fallback candidate".into()),
        _ => (0.0, String::new()),
    }
}

fn build_topic_aware_model_router_explanation(
    recommended_model: Option<&ModelRef>,
    active_model: &Option<ModelRef>,
    should_switch: bool,
    specialties: &[String],
    top_score: f32,
    active_score: Option<f32>,
) -> String {
    let specialties = if specialties.is_empty() {
        "general".into()
    } else {
        specialties.join(", ")
    };
    let recommended = recommended_model
        .map(|model| format!("{}/{}", model.provider, model.model))
        .unwrap_or_else(|| "<none>".into());
    let active = active_model
        .as_ref()
        .map(|model| format!("{}/{}", model.provider, model.model))
        .unwrap_or_else(|| "<none>".into());
    let active_score = active_score
        .map(|score| format!("{score:.2}"))
        .unwrap_or_else(|| "n/a".into());
    format!(
        "{TOPIC_AWARE_MODEL_ROUTER_ID} dry-run recommends {recommended} for specialties [{specialties}] (top_score={top_score:.2}, active={active}, active_score={active_score}, should_switch={should_switch})"
    )
}

pub trait BootstrapTopicRouteMaterializer {
    fn build_candidate_route(
        &mut self,
        selected_existing_indices: &BTreeSet<usize>,
        candidate_label: &str,
        has_prior_routes: bool,
    ) -> BootstrapTopicRouteCandidate;

    fn build_merged_route(
        &mut self,
        routes: &[BootstrapTopicRouteCandidate],
        marker: &'static str,
    ) -> BootstrapTopicRouteCandidate;

    fn infer_graph_routes(
        &mut self,
        selected_existing_indices: &BTreeSet<usize>,
        routes: &[BootstrapTopicRouteCandidate],
    ) -> Vec<BootstrapTopicRouteCandidate>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapImplicitTopicMatchCandidate {
    pub index: usize,
    pub score: f32,
    pub matched_terms: Vec<String>,
    pub was_active: bool,
    pub last_active_unix_ms: u64,
}

impl BootstrapTopicRoutePlanner {
    pub fn new(limit: usize) -> Self {
        Self { limit }
    }

    pub fn plan_with_materializer<I, M>(
        self,
        implicit_routes: Vec<BootstrapTopicRouteCandidate>,
        candidate_labels: I,
        merge_marker: Option<&'static str>,
        split_marker: Option<&'static str>,
        materializer: &mut M,
    ) -> BootstrapTopicRoutePlannerOutcome
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
        M: BootstrapTopicRouteMaterializer + ?Sized,
    {
        let mut selection = select_bootstrap_initial_topic_routes(
            implicit_routes,
            candidate_labels,
            self.limit,
            |selected_existing_indices, candidate_label, has_prior_routes| {
                materializer.build_candidate_route(
                    selected_existing_indices,
                    candidate_label,
                    has_prior_routes,
                )
            },
        );
        let mut merged_source_indices = BTreeSet::new();

        if let Some(merged_selection) = select_bootstrap_merged_topic_route(
            &selection.routes,
            merge_marker,
            |routes, marker| materializer.build_merged_route(routes, marker),
        ) {
            selection = merged_selection.selection;
            merged_source_indices = merged_selection.merged_source_indices;
        }

        if merge_marker.is_none() && split_marker.is_none() {
            let graph_routes = materializer
                .infer_graph_routes(&selection.selected_existing_indices, &selection.routes);
            selection = append_bootstrap_graph_topic_routes(selection, graph_routes, self.limit);
        }

        BootstrapTopicRoutePlannerOutcome {
            routes: selection.routes,
            selected_existing_indices: selection.selected_existing_indices,
            merged_source_indices,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn plan<I, B, M, G>(
        self,
        implicit_routes: Vec<BootstrapTopicRouteCandidate>,
        candidate_labels: I,
        merge_marker: Option<&'static str>,
        split_marker: Option<&'static str>,
        build_candidate_route: B,
        build_merged_route: M,
        infer_graph_routes: G,
    ) -> BootstrapTopicRoutePlannerOutcome
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
        B: FnMut(&BTreeSet<usize>, &str, bool) -> BootstrapTopicRouteCandidate,
        M: FnOnce(&[BootstrapTopicRouteCandidate], &'static str) -> BootstrapTopicRouteCandidate,
        G: FnOnce(
            &BTreeSet<usize>,
            &[BootstrapTopicRouteCandidate],
        ) -> Vec<BootstrapTopicRouteCandidate>,
    {
        let mut selection = select_bootstrap_initial_topic_routes(
            implicit_routes,
            candidate_labels,
            self.limit,
            build_candidate_route,
        );
        let mut merged_source_indices = BTreeSet::new();

        if let Some(merged_selection) =
            select_bootstrap_merged_topic_route(&selection.routes, merge_marker, build_merged_route)
        {
            selection = merged_selection.selection;
            merged_source_indices = merged_selection.merged_source_indices;
        }

        if merge_marker.is_none() && split_marker.is_none() {
            let graph_routes =
                infer_graph_routes(&selection.selected_existing_indices, &selection.routes);
            selection = append_bootstrap_graph_topic_routes(selection, graph_routes, self.limit);
        }

        BootstrapTopicRoutePlannerOutcome {
            routes: selection.routes,
            selected_existing_indices: selection.selected_existing_indices,
            merged_source_indices,
        }
    }
}

pub fn select_bootstrap_initial_topic_routes<I, F>(
    implicit_routes: Vec<BootstrapTopicRouteCandidate>,
    candidate_labels: I,
    limit: usize,
    mut build_candidate_route: F,
) -> BootstrapTopicRouteSelection
where
    I: IntoIterator,
    I::Item: AsRef<str>,
    F: FnMut(&BTreeSet<usize>, &str, bool) -> BootstrapTopicRouteCandidate,
{
    let mut selection = BootstrapTopicRouteSelection {
        routes: Vec::new(),
        selected_existing_indices: BTreeSet::new(),
    };
    if limit == 0 {
        return selection;
    }

    if implicit_routes.len() >= 2 {
        for route in implicit_routes {
            push_bootstrap_topic_route(&mut selection, route);
            if selection.routes.len() >= limit {
                break;
            }
        }
        return selection;
    }

    for candidate_label in candidate_labels {
        if selection.routes.len() >= limit {
            break;
        }
        let route = build_candidate_route(
            &selection.selected_existing_indices,
            candidate_label.as_ref(),
            !selection.routes.is_empty(),
        );
        push_bootstrap_topic_route(&mut selection, route);
    }

    selection
}

pub fn select_bootstrap_implicit_topic_match_candidates<I>(
    candidates: I,
    limit: usize,
    min_score: f32,
    min_matched_terms: usize,
) -> Vec<BootstrapImplicitTopicMatchCandidate>
where
    I: IntoIterator<Item = BootstrapImplicitTopicMatchCandidate>,
{
    if limit == 0 {
        return Vec::new();
    }

    let mut candidates = candidates
        .into_iter()
        .filter(|candidate| {
            candidate.score >= min_score && candidate.matched_terms.len() >= min_matched_terms
        })
        .collect::<Vec<_>>();
    if candidates.len() < 2 {
        return Vec::new();
    }

    candidates.sort_by(|left, right| {
        right
            .score
            .total_cmp(&left.score)
            .then_with(|| right.was_active.cmp(&left.was_active))
            .then_with(|| right.last_active_unix_ms.cmp(&left.last_active_unix_ms))
            .then_with(|| left.index.cmp(&right.index))
    });
    candidates.truncate(limit);
    candidates
}

pub fn select_bootstrap_merged_topic_route<F>(
    routes: &[BootstrapTopicRouteCandidate],
    marker: Option<&'static str>,
    build_merged_route: F,
) -> Option<BootstrapTopicMergedRouteSelection>
where
    F: FnOnce(&[BootstrapTopicRouteCandidate], &'static str) -> BootstrapTopicRouteCandidate,
{
    let marker = marker?;
    if routes.len() < 2 {
        return None;
    }

    let mut merged_source_indices = routes
        .iter()
        .filter_map(|route| route.existing_index)
        .collect::<BTreeSet<_>>();
    let merged_route = build_merged_route(routes, marker);
    let mut selection = BootstrapTopicRouteSelection {
        routes: vec![merged_route],
        selected_existing_indices: BTreeSet::new(),
    };

    if let Some(existing_index) = selection.routes[0].existing_index {
        selection.selected_existing_indices.insert(existing_index);
        merged_source_indices.remove(&existing_index);
    }

    Some(BootstrapTopicMergedRouteSelection {
        selection,
        merged_source_indices,
    })
}

pub fn append_bootstrap_graph_topic_routes<I>(
    mut selection: BootstrapTopicRouteSelection,
    graph_routes: I,
    limit: usize,
) -> BootstrapTopicRouteSelection
where
    I: IntoIterator<Item = BootstrapTopicRouteCandidate>,
{
    if limit == 0 {
        selection.routes.clear();
        selection.selected_existing_indices.clear();
        return selection;
    }

    for route in graph_routes {
        if selection.routes.len() >= limit {
            break;
        }
        if let Some(existing_index) = route.existing_index
            && !selection.selected_existing_indices.insert(existing_index)
        {
            continue;
        }
        selection.routes.push(route);
    }

    selection
}

fn push_bootstrap_topic_route(
    selection: &mut BootstrapTopicRouteSelection,
    route: BootstrapTopicRouteCandidate,
) {
    if let Some(existing_index) = route.existing_index {
        selection.selected_existing_indices.insert(existing_index);
    }
    selection.routes.push(route);
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapTopicMatchCandidate {
    pub index: usize,
    pub features: BootstrapTopicMatchFeatures,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapGraphRouteRankCandidate {
    pub target_index: usize,
    pub strength: f32,
    pub last_active_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapTopicGraphLink {
    pub strength: f32,
    pub matched_terms: Vec<String>,
    pub reason: String,
}

pub fn select_bootstrap_topic_match_candidate<I>(
    candidates: I,
    min_score: f32,
) -> Option<BootstrapTopicMatchCandidate>
where
    I: IntoIterator<Item = BootstrapTopicMatchCandidate>,
{
    candidates
        .into_iter()
        .filter(|candidate| candidate.features.score >= min_score)
        .max_by(|left, right| left.features.score.total_cmp(&right.features.score))
}

pub fn rank_bootstrap_graph_route_candidates<I>(candidates: I, limit: usize) -> Vec<usize>
where
    I: IntoIterator<Item = BootstrapGraphRouteRankCandidate>,
{
    if limit == 0 {
        return Vec::new();
    }

    let mut candidates = candidates.into_iter().collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        right
            .strength
            .total_cmp(&left.strength)
            .then_with(|| right.last_active_unix_ms.cmp(&left.last_active_unix_ms))
            .then_with(|| left.target_index.cmp(&right.target_index))
    });
    candidates
        .into_iter()
        .take(limit)
        .map(|candidate| candidate.target_index)
        .collect()
}

pub fn infer_bootstrap_persisted_topic_graph_link(
    source_label: &str,
    target_label: &str,
    edge_kind: TopicGraphEdgeKind,
    edge_relation: &str,
    edge_weight: f32,
) -> BootstrapTopicGraphLink {
    let matched_terms = shared_semantic_label_terms(source_label, target_label, 3);
    let reason = match edge_kind {
        TopicGraphEdgeKind::CoActivation => format!(
            "bootstrap topic graph expanded '{}' via stored co-activation edge to '{}' ({:.2})",
            source_label, target_label, edge_weight,
        ),
        TopicGraphEdgeKind::SplitComponent => format!(
            "bootstrap topic graph expanded '{}' via stored split-component edge to '{}' ({:.2})",
            source_label, target_label, edge_weight,
        ),
        TopicGraphEdgeKind::MergedInto => format!(
            "bootstrap topic graph expanded '{}' via stored merged-into edge to '{}' ({:.2})",
            source_label, target_label, edge_weight,
        ),
        TopicGraphEdgeKind::HasComponent => format!(
            "bootstrap topic graph expanded '{}' via stored component edge to '{}' ({:.2})",
            source_label, target_label, edge_weight,
        ),
        _ => format!(
            "bootstrap topic graph expanded '{}' via stored {} edge to '{}' ({:.2})",
            source_label, edge_relation, target_label, edge_weight,
        ),
    };

    BootstrapTopicGraphLink {
        strength: edge_weight,
        matched_terms,
        reason,
    }
}

pub fn infer_bootstrap_heuristic_topic_graph_link(
    source_label: &str,
    source_was_active: bool,
    source_reason: &str,
    target_label: &str,
    target_status: TopicSessionStatus,
) -> Option<BootstrapTopicGraphLink> {
    let source_terms = extract_semantic_terms(source_label, 8);
    let target_terms = extract_semantic_terms(target_label, 8);
    if source_terms.is_empty() || target_terms.is_empty() {
        return None;
    }

    let target_set = target_terms.iter().cloned().collect::<BTreeSet<_>>();
    let source_set = source_terms.iter().cloned().collect::<BTreeSet<_>>();
    let overlap_terms = source_terms
        .iter()
        .filter(|term| target_set.contains(term.as_str()))
        .cloned()
        .collect::<Vec<_>>();

    let source_subset_target = !source_set.is_empty() && source_set.is_subset(&target_set);
    let target_subset_source = !target_set.is_empty() && target_set.is_subset(&source_set);
    let subset_relation = source_subset_target || target_subset_source;
    if !subset_relation || overlap_terms.len() < 2 {
        return None;
    }

    let source_status_merged = (!source_was_active && source_reason.contains("merged"))
        || source_reason.contains("revived");
    let target_status_merged = matches!(target_status, TopicSessionStatus::Merged);
    let strength = if target_status_merged || source_status_merged {
        0.72
    } else {
        0.58
    };
    let reason = if target_status_merged || source_status_merged {
        format!(
            "bootstrap topic graph expanded '{}' via merged-composite adjacency to '{}'",
            source_label, target_label,
        )
    } else {
        format!(
            "bootstrap topic graph expanded '{}' via composite label adjacency to '{}'",
            source_label, target_label,
        )
    };

    Some(BootstrapTopicGraphLink {
        strength,
        matched_terms: overlap_terms.into_iter().take(3).collect(),
        reason,
    })
}

pub fn shared_semantic_label_terms(left: &str, right: &str, limit: usize) -> Vec<String> {
    if limit == 0 {
        return Vec::new();
    }

    let right_terms = extract_semantic_terms(right, limit.max(8))
        .into_iter()
        .collect::<BTreeSet<_>>();
    extract_semantic_terms(left, limit.max(8))
        .into_iter()
        .filter(|term| right_terms.contains(term.as_str()))
        .take(limit)
        .collect()
}

pub fn bootstrap_candidate_topic_label(query_text: Option<&str>, fallback_label: &str) -> String {
    query_text
        .map(str::trim)
        .filter(|query| !query.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| fallback_label.to_string())
}

pub fn bootstrap_candidate_topic_labels(
    query_text: Option<&str>,
    fallback_label: &str,
    limit: usize,
) -> Vec<String> {
    let base = bootstrap_candidate_topic_label(query_text, fallback_label);
    if query_text.is_none() || limit <= 1 {
        return vec![base];
    }

    let raw_query = query_text.unwrap_or_default().trim();
    if raw_query.is_empty() {
        return vec![base];
    }

    let normalized = raw_query
        .replace(" but not ", " | ")
        .replace(" instead of ", " | ")
        .replace(" rather than ", " | ")
        .replace(" except ", " | ")
        .replace(" and ", " | ")
        .replace(" plus ", " | ")
        .replace(" then ", " | ")
        .replace(" also ", " | ")
        .replace(" + ", " | ")
        .replace(" & ", " | ")
        .replace(" / ", " | ")
        .replace(", ", " | ")
        .replace("; ", " | ")
        .replace(['、', '，'], " | ");

    let mut seen = BTreeSet::new();
    let mut labels = Vec::new();
    for segment in normalized.split('|') {
        let candidate = segment
            .trim_matches(|ch: char| ch.is_whitespace() || matches!(ch, ',' | '.' | ';' | ':'))
            .trim();
        if candidate.is_empty() {
            continue;
        }

        let slug = bootstrap_candidate_label_slug(candidate);
        if slug.is_empty() || !seen.insert(slug) {
            continue;
        }

        if extract_semantic_terms(candidate, 4).len() < 2 {
            continue;
        }

        labels.push(candidate.to_string());
        if labels.len() >= limit {
            break;
        }
    }

    if labels.len() >= 2 {
        labels
    } else {
        vec![base]
    }
}

pub fn extract_semantic_terms(value: &str, limit: usize) -> Vec<String> {
    if limit == 0 {
        return Vec::new();
    }

    let mut seen = BTreeSet::new();
    let mut terms = Vec::new();

    for token in value
        .split(|ch: char| !ch.is_alphanumeric())
        .filter_map(semantic_router_term)
    {
        if !seen.insert(token.clone()) {
            continue;
        }

        terms.push(token);
        if terms.len() >= limit {
            break;
        }
    }

    terms
}

pub fn bootstrap_semantic_term(token: &str) -> Option<String> {
    semantic_router_term(token)
}

pub fn extract_surface_terms(value: &str, limit: usize) -> Vec<String> {
    if limit == 0 {
        return Vec::new();
    }

    let mut seen = BTreeSet::new();
    let mut terms = Vec::new();

    for token in value
        .split(|ch: char| !ch.is_alphanumeric())
        .filter_map(bootstrap_surface_term)
    {
        if !seen.insert(token.clone()) {
            continue;
        }

        terms.push(token);
        if terms.len() >= limit {
            break;
        }
    }

    terms
}

pub fn extract_bootstrap_semantic_hints_for_match(
    candidate_label: &str,
    topic_session: &TopicSession,
    limit: usize,
) -> Vec<String> {
    if limit == 0 {
        return Vec::new();
    }

    let existing_terms = topic_session_surface_terms(topic_session, limit.max(8) * 2)
        .into_iter()
        .collect::<BTreeSet<_>>();
    let mut seen = BTreeSet::new();
    let mut hints = Vec::new();

    for term in extract_surface_terms(candidate_label, limit.max(8) * 2) {
        if existing_terms.contains(term.as_str()) || !seen.insert(term.clone()) {
            continue;
        }

        hints.push(term);
        if hints.len() >= limit {
            break;
        }
    }

    hints
}

pub fn extract_bootstrap_semantic_hints_from_overlap(
    candidate_label: &str,
    topic_session: &TopicSession,
    limit: usize,
) -> Vec<String> {
    if limit == 0 {
        return Vec::new();
    }

    let existing_terms = topic_session_surface_terms(topic_session, limit.max(8) * 2)
        .into_iter()
        .collect::<BTreeSet<_>>();
    let topic_semantic_terms = topic_session_semantic_terms(topic_session, limit.max(8) * 2)
        .into_iter()
        .collect::<BTreeSet<_>>();
    let mut seen = BTreeSet::new();
    let mut hints = Vec::new();

    for term in extract_surface_terms(candidate_label, limit.max(8) * 2) {
        let Some(canonical) = bootstrap_semantic_term(&term) else {
            continue;
        };
        if existing_terms.contains(term.as_str())
            || !topic_semantic_terms.contains(canonical.as_str())
            || !seen.insert(term.clone())
        {
            continue;
        }

        hints.push(term);
        if hints.len() >= limit {
            break;
        }
    }

    hints
}

pub fn compute_bootstrap_topic_match_features(
    candidate_label: &str,
    candidate_slug: &str,
    topic_session: &TopicSession,
    topic_label_slug: &str,
) -> BootstrapTopicMatchFeatures {
    if !candidate_slug.is_empty() && candidate_slug == topic_label_slug {
        return BootstrapTopicMatchFeatures {
            score: 1.0,
            matched_terms: extract_semantic_terms(candidate_label, 3),
        };
    }

    let surface = compute_bootstrap_term_overlap(
        &extract_surface_terms(candidate_label, 12),
        &topic_session_surface_terms(topic_session, 12),
    );
    let semantic = compute_bootstrap_term_overlap(
        &extract_semantic_terms(candidate_label, 12),
        &topic_session_semantic_terms(topic_session, 12),
    );

    if surface.score >= semantic.score {
        surface
    } else {
        semantic
    }
}

pub fn topic_session_surface_terms(topic_session: &TopicSession, limit: usize) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut terms = Vec::new();

    for term in extract_surface_terms(&topic_session.topic_label.0, limit.max(8)) {
        if seen.insert(term.clone()) {
            terms.push(term);
        }
    }

    for (key, value) in &topic_session.entities {
        if !key.starts_with(SEMANTIC_HINT_PREFIX) {
            continue;
        }

        let Some(term) = bootstrap_surface_term(value) else {
            continue;
        };
        if seen.insert(term.clone()) {
            terms.push(term);
        }
    }

    if terms.len() > limit {
        terms.truncate(limit);
    }

    terms
}

pub fn topic_session_semantic_terms(topic_session: &TopicSession, limit: usize) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut terms = Vec::new();

    for term in extract_semantic_terms(&topic_session.topic_label.0, limit.max(8)) {
        if seen.insert(term.clone()) {
            terms.push(term);
        }
    }

    for surface_term in topic_session_surface_terms(topic_session, limit.max(8)) {
        let Some(term) = bootstrap_semantic_term(&surface_term) else {
            continue;
        };
        if seen.insert(term.clone()) {
            terms.push(term);
        }
    }

    if terms.len() > limit {
        terms.truncate(limit);
    }

    terms
}

fn compute_bootstrap_term_overlap(
    candidate_terms: &[String],
    topic_terms: &[String],
) -> BootstrapTopicMatchFeatures {
    if candidate_terms.is_empty() || topic_terms.is_empty() {
        return BootstrapTopicMatchFeatures {
            score: 0.0,
            matched_terms: Vec::new(),
        };
    }

    let topic_terms = topic_terms.iter().cloned().collect::<BTreeSet<_>>();
    let overlap_terms = candidate_terms
        .iter()
        .filter(|term| topic_terms.contains(term.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    let overlap_count = overlap_terms.len();
    if overlap_count == 0 {
        return BootstrapTopicMatchFeatures {
            score: 0.0,
            matched_terms: Vec::new(),
        };
    }

    let coverage = overlap_count as f32 / candidate_terms.len() as f32;
    let density = overlap_count as f32 / topic_terms.len() as f32;

    BootstrapTopicMatchFeatures {
        score: (coverage * 0.75 + density * 0.25).min(1.0),
        matched_terms: overlap_terms.into_iter().take(3).collect(),
    }
}

fn bootstrap_surface_term(token: &str) -> Option<String> {
    let token = token.trim().to_ascii_lowercase();
    if token.chars().count() < 3 || ignored_semantic_router_term(&token) {
        return None;
    }

    Some(token)
}

fn semantic_router_term(token: &str) -> Option<String> {
    let token = token.trim().to_ascii_lowercase();
    if token.chars().count() < 3 || ignored_semantic_router_term(&token) {
        return None;
    }

    let canonical = match token.as_str() {
        "memories" | "memory" | "recall" | "context" | "history" => "memory",
        "adaptive" | "dynamic" => "adaptive",
        "worker" | "workers" | "executor" | "executors" | "execution" | "execute" | "executing"
        | "runner" | "runners" => "worker",
        "pipeline" | "pipelines" | "workflow" | "workflows" | "flow" | "flows" => "pipeline",
        _ => token.as_str(),
    };

    Some(canonical.to_string())
}

fn ignored_semantic_router_term(token: &str) -> bool {
    matches!(
        token,
        "continue"
            | "continuing"
            | "resume"
            | "resuming"
            | "please"
            | "help"
            | "with"
            | "while"
            | "then"
            | "also"
            | "into"
            | "from"
            | "around"
            | "about"
            | "focus"
            | "focusing"
            | "check"
            | "checking"
    )
}

fn bootstrap_candidate_label_slug(value: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;

    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash && !slug.is_empty() {
            slug.push('-');
            last_was_dash = true;
        }
    }

    slug.trim_matches('-').to_string()
}

fn shared_terms(left: &[String], right: &[String], limit: usize) -> Vec<String> {
    if limit == 0 || left.is_empty() || right.is_empty() {
        return Vec::new();
    }
    let right_terms = right.iter().cloned().collect::<BTreeSet<_>>();
    left.iter()
        .filter(|term| right_terms.contains(term.as_str()))
        .take(limit)
        .cloned()
        .collect()
}

fn merge_unique_terms(target: &mut Vec<String>, incoming: Vec<String>, limit: usize) {
    if limit == 0 {
        target.clear();
        return;
    }
    for term in incoming {
        if target.iter().any(|existing| existing == &term) {
            continue;
        }
        target.push(term);
        if target.len() >= limit {
            break;
        }
    }
    if target.len() > limit {
        target.truncate(limit);
    }
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

fn average_or_zero(total: f32, count: usize) -> f32 {
    if count == 0 {
        0.0
    } else {
        total / count as f32
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
