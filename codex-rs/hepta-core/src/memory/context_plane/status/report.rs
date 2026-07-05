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
use crate::memory::ContextMemoryRecallQualityGateReport;
use crate::memory::ContextMemoryTaxonomyReport;
use crate::memory::ContextMemoryTemporalFactGraphReport;
use crate::memory::ContextMemoryTemporalFactReport;

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
    pub fn from_reports(
        taxonomy: &ContextMemoryTaxonomyReport,
        formation_receipts: &ContextMemoryFormationReceiptReport,
        formation_queue: &ContextMemoryFormationQueueReport,
        temporal_facts: &ContextMemoryTemporalFactReport,
        temporal_fact_graph: &ContextMemoryTemporalFactGraphReport,
        eval_seed: &ContextMemoryEvalHarnessReport,
        allocator_shadow: &ContextMemoryAdaptiveAllocatorEvalShadowReport,
        recall_quality_gate: &ContextMemoryRecallQualityGateReport,
    ) -> Self {
        let mut sections = vec![
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::SourceRegistry, 1),
            ContextPlaneStatusEntry::shadow(ContextPlaneStatusSection::AdaptiveBudgetAllocation, 1),
            ContextPlaneStatusEntry::from_integrity(
                ContextPlaneStatusSection::MemoryTaxonomy,
                taxonomy.has_count_integrity(),
                taxonomy.buckets.len(),
                taxonomy_total_omitted_count(taxonomy),
            ),
            ContextPlaneStatusEntry::from_integrity(
                ContextPlaneStatusSection::MemoryFormationReceipts,
                formation_receipts.has_receipt_integrity(),
                formation_receipts.receipts.len(),
                0,
            ),
            ContextPlaneStatusEntry::from_integrity(
                ContextPlaneStatusSection::MemoryFormationQueue,
                formation_queue.has_queue_integrity(),
                formation_queue.items.len(),
                0,
            ),
            ContextPlaneStatusEntry::from_integrity(
                ContextPlaneStatusSection::MemoryTemporalFacts,
                temporal_facts.has_temporal_fact_integrity(),
                temporal_facts.facts.len(),
                0,
            ),
            ContextPlaneStatusEntry::from_integrity(
                ContextPlaneStatusSection::MemoryTemporalFactGraph,
                temporal_fact_graph.has_graph_integrity(),
                temporal_fact_graph.nodes.len(),
                0,
            ),
            ContextPlaneStatusEntry::from_integrity(
                ContextPlaneStatusSection::EvalHarnessSeed,
                eval_seed.has_eval_integrity(),
                eval_seed.fixture_count(),
                eval_seed.total_missing_critical_fact_count(),
            ),
            ContextPlaneStatusEntry::shadow_from_integrity(
                ContextPlaneStatusSection::AdaptiveAllocatorEvalShadow,
                allocator_shadow.has_eval_shadow_integrity(),
                allocator_shadow.shadow_results.len(),
                allocator_shadow.total_missing_critical_fact_count_for_arm(
                    ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive,
                ),
            ),
            ContextPlaneStatusEntry::from_recall_quality_gate(recall_quality_gate),
            ContextPlaneStatusEntry::disabled(ContextPlaneStatusSection::SourceAwareFrontDoor),
        ];
        sections.sort_by_key(|entry| match entry.section {
            ContextPlaneStatusSection::SourceRegistry => 0,
            ContextPlaneStatusSection::AdaptiveBudgetAllocation => 1,
            ContextPlaneStatusSection::MemoryTaxonomy => 2,
            ContextPlaneStatusSection::MemoryFormationReceipts => 3,
            ContextPlaneStatusSection::MemoryFormationQueue => 4,
            ContextPlaneStatusSection::MemoryTemporalFacts => 5,
            ContextPlaneStatusSection::MemoryTemporalFactGraph => 6,
            ContextPlaneStatusSection::EvalHarnessSeed => 7,
            ContextPlaneStatusSection::AdaptiveAllocatorEvalShadow => 8,
            ContextPlaneStatusSection::RecallQualityGate => 9,
            ContextPlaneStatusSection::SourceAwareFrontDoor => 10,
            ContextPlaneStatusSection::Unknown => 11,
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
            && self.sections.len() == 11
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
            ContextPlaneStatusSection::MemoryTemporalFacts,
            ContextPlaneStatusSection::MemoryTemporalFactGraph,
            ContextPlaneStatusSection::EvalHarnessSeed,
            ContextPlaneStatusSection::AdaptiveAllocatorEvalShadow,
            ContextPlaneStatusSection::RecallQualityGate,
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
