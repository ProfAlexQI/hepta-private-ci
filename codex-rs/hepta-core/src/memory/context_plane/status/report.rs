use serde::Deserialize;
use serde::Serialize;

use super::entry::ContextPlaneStatusEntry;
use super::section::ContextPlaneStatusKind;
use super::section::ContextPlaneStatusSection;
use crate::memory::CONTEXT_PLANE_STATUS_SCHEMA_VERSION;
use crate::memory::ContextMemoryAdaptiveAllocatorEvalArm;
use crate::memory::ContextMemoryAdaptiveAllocatorEvalShadowReport;
use crate::memory::ContextMemoryEvalHarnessReport;
use crate::memory::ContextMemoryFormationQueueReport;
use crate::memory::ContextMemoryFormationReceiptReport;
use crate::memory::ContextMemoryNamespacePolicyReport;
use crate::memory::ContextMemoryRankedRecallShadowEvalReport;
use crate::memory::ContextMemoryRecallQualityGateReport;
use crate::memory::ContextMemoryShadowCanaryPromotionReadinessReport;
use crate::memory::ContextMemoryShadowQualityTrendSnapshotReport;
use crate::memory::ContextMemoryTaxonomyReport;
use crate::memory::ContextMemoryTemporalFactGraphReport;
use crate::memory::ContextMemoryTemporalFactReport;
use crate::memory::ContextMemoryTemporalGraphShadowEvalReport;
use crate::memory::ContextMemoryTemporalGraphShadowReplayReport;
use crate::memory::ContextMemoryTemporalGraphShadowRetrievalCanaryGuardReport;
use crate::memory::ContextMemoryTemporalGraphShadowStoreReport;
use crate::memory::ContextMemoryTemporalGraphShadowTraversalDiffReport;
use crate::memory::ContextMemoryTemporalGraphShadowTraversalQualityReport;
use crate::memory::ContextMemoryWriteChainReadinessReport;
use crate::memory::ContextMemoryWriteChainReceiptFreshnessReport;
use crate::memory::MemoryProviderReport;
use crate::memory::MemoryProviderV2AuditReport;

/// Unified, payload-light status surface for context-plane readiness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextPlaneStatusReport {
    pub schema_version: u32,
    pub sections: Vec<ContextPlaneStatusEntry>,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub adaptive_allocator_runtime_activation: bool,
    pub source_aware_runtime_activation: bool,
    pub prompt_assembly_change: bool,
    pub operator_activation_allowed: bool,
}

/// Typed input bundle for building a context-plane status report from the
/// payload-light context and memory diagnostic reports.
#[derive(Debug, Clone, Copy)]
pub struct ContextPlaneStatusReportInput<'a> {
    pub taxonomy: &'a ContextMemoryTaxonomyReport,
    pub formation_receipts: &'a ContextMemoryFormationReceiptReport,
    pub formation_queue: &'a ContextMemoryFormationQueueReport,
    pub namespace_policy: &'a ContextMemoryNamespacePolicyReport,
    pub write_chain_readiness: &'a ContextMemoryWriteChainReadinessReport,
    pub write_chain_receipt_freshness: &'a ContextMemoryWriteChainReceiptFreshnessReport,
    pub temporal_facts: &'a ContextMemoryTemporalFactReport,
    pub temporal_fact_graph: &'a ContextMemoryTemporalFactGraphReport,
    pub temporal_graph_shadow_eval: &'a ContextMemoryTemporalGraphShadowEvalReport,
    pub temporal_graph_shadow_store: &'a ContextMemoryTemporalGraphShadowStoreReport,
    pub temporal_graph_shadow_replay: &'a ContextMemoryTemporalGraphShadowReplayReport,
    pub temporal_graph_shadow_traversal_diff:
        &'a ContextMemoryTemporalGraphShadowTraversalDiffReport,
    pub temporal_graph_shadow_traversal_quality:
        &'a ContextMemoryTemporalGraphShadowTraversalQualityReport,
    pub temporal_graph_shadow_retrieval_canary_guard:
        &'a ContextMemoryTemporalGraphShadowRetrievalCanaryGuardReport,
    pub eval_seed: &'a ContextMemoryEvalHarnessReport,
    pub allocator_shadow: &'a ContextMemoryAdaptiveAllocatorEvalShadowReport,
    pub recall_quality_gate: &'a ContextMemoryRecallQualityGateReport,
    pub ranked_recall: &'a ContextMemoryRankedRecallShadowEvalReport,
    pub provider_report: &'a MemoryProviderReport,
    pub provider_v2_audit: &'a MemoryProviderV2AuditReport,
    pub shadow_quality_trend_snapshot: &'a ContextMemoryShadowQualityTrendSnapshotReport,
    pub shadow_canary_promotion_readiness: &'a ContextMemoryShadowCanaryPromotionReadinessReport,
}

impl Default for ContextPlaneStatusReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_PLANE_STATUS_SCHEMA_VERSION,
            sections: Vec::new(),
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            adaptive_allocator_runtime_activation: false,
            source_aware_runtime_activation: false,
            prompt_assembly_change: false,
            operator_activation_allowed: false,
        }
    }
}

impl ContextPlaneStatusReport {
    pub fn from_reports(input: ContextPlaneStatusReportInput<'_>) -> Self {
        let mut sections = vec![
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::SourceRegistry, 1),
            ContextPlaneStatusEntry::shadow(ContextPlaneStatusSection::AdaptiveBudgetAllocation, 1),
            ContextPlaneStatusEntry::from_integrity(
                ContextPlaneStatusSection::MemoryTaxonomy,
                input.taxonomy.has_count_integrity(),
                input.taxonomy.buckets.len(),
                taxonomy_total_omitted_count(input.taxonomy),
            ),
            ContextPlaneStatusEntry::from_integrity(
                ContextPlaneStatusSection::MemoryFormationReceipts,
                input.formation_receipts.has_receipt_integrity(),
                input.formation_receipts.receipts.len(),
                0,
            ),
            ContextPlaneStatusEntry::from_integrity(
                ContextPlaneStatusSection::MemoryFormationQueue,
                input.formation_queue.has_queue_integrity(),
                input.formation_queue.items.len(),
                0,
            ),
            ContextPlaneStatusEntry::from_memory_namespace_policy(input.namespace_policy),
            ContextPlaneStatusEntry::from_memory_write_chain_readiness(input.write_chain_readiness),
            ContextPlaneStatusEntry::from_memory_write_chain_receipt_freshness(
                input.write_chain_receipt_freshness,
            ),
            ContextPlaneStatusEntry::from_integrity(
                ContextPlaneStatusSection::MemoryTemporalFacts,
                input.temporal_facts.has_temporal_fact_integrity(),
                input.temporal_facts.facts.len(),
                0,
            ),
            ContextPlaneStatusEntry::from_integrity(
                ContextPlaneStatusSection::MemoryTemporalFactGraph,
                input.temporal_fact_graph.has_graph_integrity(),
                input.temporal_fact_graph.nodes.len(),
                0,
            ),
            ContextPlaneStatusEntry::from_temporal_graph_shadow_eval(
                input.temporal_graph_shadow_eval,
            ),
            ContextPlaneStatusEntry::from_temporal_graph_shadow_store(
                input.temporal_graph_shadow_store,
            ),
            ContextPlaneStatusEntry::from_temporal_graph_shadow_replay(
                input.temporal_graph_shadow_replay,
            ),
            ContextPlaneStatusEntry::from_temporal_graph_shadow_traversal_diff(
                input.temporal_graph_shadow_traversal_diff,
            ),
            ContextPlaneStatusEntry::from_temporal_graph_shadow_traversal_quality(
                input.temporal_graph_shadow_traversal_quality,
            ),
            ContextPlaneStatusEntry::from_temporal_graph_shadow_retrieval_canary_guard(
                input.temporal_graph_shadow_retrieval_canary_guard,
            ),
            ContextPlaneStatusEntry::from_integrity(
                ContextPlaneStatusSection::EvalHarnessSeed,
                input.eval_seed.has_eval_integrity(),
                input.eval_seed.fixture_count(),
                input.eval_seed.total_missing_critical_fact_count(),
            ),
            ContextPlaneStatusEntry::shadow_from_integrity(
                ContextPlaneStatusSection::AdaptiveAllocatorEvalShadow,
                input.allocator_shadow.has_eval_shadow_integrity(),
                input.allocator_shadow.shadow_results.len(),
                input
                    .allocator_shadow
                    .total_missing_critical_fact_count_for_arm(
                        ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive,
                    ),
            ),
            ContextPlaneStatusEntry::from_recall_quality_gate(input.recall_quality_gate),
            ContextPlaneStatusEntry::from_ranked_recall_shadow_eval(input.ranked_recall),
            ContextPlaneStatusEntry::from_memory_provider_report(input.provider_report),
            ContextPlaneStatusEntry::from_memory_provider_v2_audit(input.provider_v2_audit),
            ContextPlaneStatusEntry::from_memory_shadow_canary_readiness(
                input.shadow_quality_trend_snapshot,
            ),
            ContextPlaneStatusEntry::from_memory_shadow_canary_promotion_readiness(
                input.shadow_canary_promotion_readiness,
            ),
            ContextPlaneStatusEntry::disabled(ContextPlaneStatusSection::SourceAwareFrontDoor),
        ];
        sections.sort_by_key(|entry| match entry.section {
            ContextPlaneStatusSection::SourceRegistry => 0,
            ContextPlaneStatusSection::AdaptiveBudgetAllocation => 1,
            ContextPlaneStatusSection::MemoryTaxonomy => 2,
            ContextPlaneStatusSection::MemoryFormationReceipts => 3,
            ContextPlaneStatusSection::MemoryFormationQueue => 4,
            ContextPlaneStatusSection::MemoryNamespacePolicy => 5,
            ContextPlaneStatusSection::MemoryWriteChainReadiness => 6,
            ContextPlaneStatusSection::MemoryWriteChainReceiptFreshness => 7,
            ContextPlaneStatusSection::MemoryTemporalFacts => 8,
            ContextPlaneStatusSection::MemoryTemporalFactGraph => 9,
            ContextPlaneStatusSection::MemoryTemporalGraphShadowEval => 10,
            ContextPlaneStatusSection::MemoryTemporalGraphShadowStore => 11,
            ContextPlaneStatusSection::MemoryTemporalGraphShadowReplay => 12,
            ContextPlaneStatusSection::MemoryTemporalGraphShadowTraversalDiff => 13,
            ContextPlaneStatusSection::MemoryTemporalGraphShadowTraversalQuality => 14,
            ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalCanaryGuard => 15,
            ContextPlaneStatusSection::EvalHarnessSeed => 16,
            ContextPlaneStatusSection::AdaptiveAllocatorEvalShadow => 17,
            ContextPlaneStatusSection::RecallQualityGate => 18,
            ContextPlaneStatusSection::MemoryRankedRecallShadowEval => 19,
            ContextPlaneStatusSection::MemoryProviderBoundary => 20,
            ContextPlaneStatusSection::MemoryProviderV2Boundary => 21,
            ContextPlaneStatusSection::MemoryShadowCanaryReadiness => 22,
            ContextPlaneStatusSection::MemoryShadowCanaryPromotionReadiness => 23,
            ContextPlaneStatusSection::SourceAwareFrontDoor => 24,
            ContextPlaneStatusSection::Unknown => 25,
        });

        let production_write = sections.iter().any(|entry| entry.production_write);
        let graph_write = sections.iter().any(|entry| entry.graph_write);
        let runtime_activation = sections.iter().any(|entry| entry.runtime_activation);
        let prompt_assembly_change = sections.iter().any(|entry| entry.prompt_assembly_change);
        let operator_activation_allowed = sections
            .iter()
            .any(|entry| entry.operator_activation_allowed);

        Self {
            sections,
            production_write,
            graph_write,
            runtime_activation,
            prompt_assembly_change,
            operator_activation_allowed,
            ..Self::default()
        }
    }

    pub fn has_status_integrity(&self) -> bool {
        self.schema_version == CONTEXT_PLANE_STATUS_SCHEMA_VERSION
            && self.sections.len() == 25
            && self.has_required_sections()
            && self
                .sections
                .iter()
                .all(ContextPlaneStatusEntry::has_status_integrity)
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.adaptive_allocator_runtime_activation
            && !self.source_aware_runtime_activation
            && !self.prompt_assembly_change
            && !self.operator_activation_allowed
    }

    fn has_required_sections(&self) -> bool {
        [
            ContextPlaneStatusSection::SourceRegistry,
            ContextPlaneStatusSection::AdaptiveBudgetAllocation,
            ContextPlaneStatusSection::MemoryTaxonomy,
            ContextPlaneStatusSection::MemoryFormationReceipts,
            ContextPlaneStatusSection::MemoryFormationQueue,
            ContextPlaneStatusSection::MemoryNamespacePolicy,
            ContextPlaneStatusSection::MemoryWriteChainReadiness,
            ContextPlaneStatusSection::MemoryWriteChainReceiptFreshness,
            ContextPlaneStatusSection::MemoryTemporalFacts,
            ContextPlaneStatusSection::MemoryTemporalFactGraph,
            ContextPlaneStatusSection::MemoryTemporalGraphShadowEval,
            ContextPlaneStatusSection::MemoryTemporalGraphShadowStore,
            ContextPlaneStatusSection::MemoryTemporalGraphShadowReplay,
            ContextPlaneStatusSection::MemoryTemporalGraphShadowTraversalDiff,
            ContextPlaneStatusSection::MemoryTemporalGraphShadowTraversalQuality,
            ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalCanaryGuard,
            ContextPlaneStatusSection::EvalHarnessSeed,
            ContextPlaneStatusSection::AdaptiveAllocatorEvalShadow,
            ContextPlaneStatusSection::RecallQualityGate,
            ContextPlaneStatusSection::MemoryRankedRecallShadowEval,
            ContextPlaneStatusSection::MemoryProviderBoundary,
            ContextPlaneStatusSection::MemoryProviderV2Boundary,
            ContextPlaneStatusSection::MemoryShadowCanaryReadiness,
            ContextPlaneStatusSection::MemoryShadowCanaryPromotionReadiness,
            ContextPlaneStatusSection::SourceAwareFrontDoor,
        ]
        .into_iter()
        .all(|section| self.section_status(section).is_some())
    }

    pub fn section_status(
        &self,
        section: ContextPlaneStatusSection,
    ) -> Option<ContextPlaneStatusKind> {
        self.sections
            .iter()
            .find(|entry| entry.section == section)
            .map(|entry| entry.status)
    }

    pub fn blocker_count(&self) -> usize {
        self.sections.iter().map(|entry| entry.blocker_count).sum()
    }

    pub fn ready_section_count(&self) -> usize {
        self.sections
            .iter()
            .filter(|entry| entry.status == ContextPlaneStatusKind::Ready)
            .count()
    }

    pub fn shadow_section_count(&self) -> usize {
        self.sections
            .iter()
            .filter(|entry| entry.status == ContextPlaneStatusKind::Shadow)
            .count()
    }

    pub fn disabled_section_count(&self) -> usize {
        self.sections
            .iter()
            .filter(|entry| entry.status == ContextPlaneStatusKind::Disabled)
            .count()
    }
}

pub(in crate::memory::context_plane) fn status_entry_for_section(
    status: &ContextPlaneStatusReport,
    section: ContextPlaneStatusSection,
) -> Option<&ContextPlaneStatusEntry> {
    status
        .sections
        .iter()
        .find(|entry| entry.section == section)
}

pub(in crate::memory::context_plane) fn context_plane_status_report_has_side_effect_flag(
    status: &ContextPlaneStatusReport,
) -> bool {
    status.production_write
        || status.graph_write
        || status.runtime_activation
        || status.adaptive_allocator_runtime_activation
        || status.source_aware_runtime_activation
        || status.prompt_assembly_change
        || status.operator_activation_allowed
}

fn taxonomy_total_omitted_count(taxonomy: &ContextMemoryTaxonomyReport) -> usize {
    taxonomy
        .buckets
        .iter()
        .map(|bucket| bucket.omitted_count)
        .sum()
}
