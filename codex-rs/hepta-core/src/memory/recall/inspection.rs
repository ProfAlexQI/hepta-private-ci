use serde::Deserialize;
use serde::Serialize;

use crate::TranscriptSpanRef;

use super::super::ContextMemoryFormationQueueReport;
use super::super::ContextMemoryFormationReceiptReport;
use super::super::ContextMemoryTaxonomyReport;
use super::super::ContextMemoryTemporalFactGraphReport;
use super::super::ContextMemoryTemporalFactReport;
use super::ContextRecallBundle;
use super::ContextRecallRequest;
use super::coverage::ContextRecallCoverage;
use super::coverage::ContextRecallLimitPressure;
use super::coverage::ContextRecallOmissionCounts;
use super::coverage::ContextRecallTranscriptProvenanceSummary;

/// Lightweight per-source item counts for a blended recall bundle.
///
/// This gives automation and tests a compact machine-readable summary without
/// needing to walk the full transcript and memory payloads.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextRecallSourceCounts {
    pub recent_entry_count: usize,
    pub transcript_hit_count: usize,
    pub durable_memory_hit_count: usize,
    pub summary_hit_count: usize,
}

impl ContextRecallSourceCounts {
    pub fn query_hit_count(&self) -> usize {
        self.transcript_hit_count + self.durable_memory_hit_count + self.summary_hit_count
    }

    pub fn total_item_count(&self) -> usize {
        self.recent_entry_count + self.query_hit_count()
    }

    pub fn has_query_matches(&self) -> bool {
        self.query_hit_count() > 0
    }

    pub fn is_empty(&self) -> bool {
        self.total_item_count() == 0
    }
}

/// Compact machine-readable report for blended transcript + memory recall.
///
/// This keeps the original request, per-source counts, and truncation state
/// without embedding the full transcript and memory payloads from the bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextRecallReport {
    pub request: ContextRecallRequest,
    pub source_counts: ContextRecallSourceCounts,
    pub truncated: bool,
}

impl ContextRecallReport {
    pub fn from_bundle(bundle: &ContextRecallBundle) -> Self {
        Self {
            request: bundle.request.clone(),
            source_counts: bundle.source_counts(),
            truncated: bundle.truncated,
        }
    }

    pub fn query_hit_count(&self) -> usize {
        self.source_counts.query_hit_count()
    }

    pub fn total_item_count(&self) -> usize {
        self.source_counts.total_item_count()
    }

    pub fn has_query_matches(&self) -> bool {
        self.source_counts.has_query_matches()
    }

    pub fn is_empty(&self) -> bool {
        self.source_counts.is_empty()
    }
}

/// Payload-light availability counts for blended recall sources before limits
/// are applied.
///
/// This complements [`ContextRecallReport`], whose counts describe returned
/// items only. Automation can use the availability view to detect which recall
/// sources were clipped by recent-window or query limits without loading the
/// full bundle payload. The counts stay pre-limit even when recent-window,
/// transcript, or memory caps return fewer items, which lets callers compare
/// compact availability data against returned counts without reconstructing the
/// full inspection payload.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextRecallAvailability {
    pub total_recent_entry_count: usize,
    pub total_transcript_match_count: usize,
    pub total_memory_match_count: usize,
}

impl ContextRecallAvailability {
    pub fn query_match_count(&self) -> usize {
        self.total_transcript_match_count + self.total_memory_match_count
    }

    pub fn total_item_count(&self) -> usize {
        self.total_recent_entry_count + self.query_match_count()
    }

    pub fn has_query_matches(&self) -> bool {
        self.query_match_count() > 0
    }

    pub fn is_empty(&self) -> bool {
        self.total_item_count() == 0
    }
}

/// Compact pre-limit recall counts that preserve the returned-source split.
///
/// Unlike [`ContextRecallAvailability`], this retains separate durable-memory
/// and session-summary match counts so automation can reason about which memory
/// lane contributed omitted hits without loading the full recall payload.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextRecallSourceAvailability {
    pub recent_entry_count: usize,
    pub transcript_match_count: usize,
    pub durable_memory_match_count: usize,
    pub summary_memory_match_count: usize,
}

impl ContextRecallSourceAvailability {
    pub fn memory_match_count(&self) -> usize {
        self.durable_memory_match_count + self.summary_memory_match_count
    }

    pub fn query_match_count(&self) -> usize {
        self.transcript_match_count + self.memory_match_count()
    }

    pub fn total_item_count(&self) -> usize {
        self.recent_entry_count + self.query_match_count()
    }

    pub fn has_query_matches(&self) -> bool {
        self.query_match_count() > 0
    }

    pub fn is_empty(&self) -> bool {
        self.total_item_count() == 0
    }
}

/// Compact inspection view for blended recall availability and returned items.
///
/// Unlike [`ContextRecallBundle`], this keeps only the report plus pre-limit
/// availability counts, which makes it suitable for doctor output, audit
/// trails, and automation that needs to distinguish between complete and
/// clipped recall results, including how many items were omitted by each
/// recall limit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextRecallInspection {
    pub report: ContextRecallReport,
    pub availability: ContextRecallAvailability,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_transcript_spans: Vec<TranscriptSpanRef>,
}

impl ContextRecallInspection {
    pub fn from_bundle(
        bundle: &ContextRecallBundle,
        availability: ContextRecallAvailability,
    ) -> Self {
        Self {
            report: bundle.report(),
            availability,
            source_transcript_spans: bundle.source_transcript_spans(),
        }
    }

    pub fn returned_memory_hit_count(&self) -> usize {
        self.report.source_counts.durable_memory_hit_count
            + self.report.source_counts.summary_hit_count
    }

    pub fn returned_query_hit_count(&self) -> usize {
        self.report.query_hit_count()
    }

    pub fn returned_total_item_count(&self) -> usize {
        self.report.total_item_count()
    }

    pub fn omitted_recent_entry_count(&self) -> usize {
        self.availability
            .total_recent_entry_count
            .saturating_sub(self.report.source_counts.recent_entry_count)
    }

    pub fn omitted_transcript_hit_count(&self) -> usize {
        self.availability
            .total_transcript_match_count
            .saturating_sub(self.report.source_counts.transcript_hit_count)
    }

    pub fn omitted_memory_hit_count(&self) -> usize {
        self.availability
            .total_memory_match_count
            .saturating_sub(self.returned_memory_hit_count())
    }

    pub fn omitted_query_hit_count(&self) -> usize {
        self.omitted_transcript_hit_count() + self.omitted_memory_hit_count()
    }

    pub fn omitted_total_item_count(&self) -> usize {
        self.omitted_recent_entry_count() + self.omitted_query_hit_count()
    }

    /// Returns a compact omitted-item summary for each recall source.
    pub fn omission_counts(&self) -> ContextRecallOmissionCounts {
        ContextRecallOmissionCounts {
            recent_entry_count: self.omitted_recent_entry_count(),
            transcript_hit_count: self.omitted_transcript_hit_count(),
            memory_hit_count: self.omitted_memory_hit_count(),
            query_hit_count: self.omitted_query_hit_count(),
            total_item_count: self.omitted_total_item_count(),
        }
    }

    pub fn matched_query_hit_count(&self) -> usize {
        self.availability.query_match_count()
    }

    pub fn matched_total_item_count(&self) -> usize {
        self.availability.total_item_count()
    }

    pub fn recent_entries_truncated(&self) -> bool {
        self.report.source_counts.recent_entry_count < self.availability.total_recent_entry_count
    }

    pub fn transcript_hits_truncated(&self) -> bool {
        self.report.source_counts.transcript_hit_count
            < self.availability.total_transcript_match_count
    }

    pub fn memory_hits_truncated(&self) -> bool {
        self.returned_memory_hit_count() < self.availability.total_memory_match_count
    }

    pub fn has_query_matches(&self) -> bool {
        self.availability.has_query_matches()
    }

    pub fn has_omissions(&self) -> bool {
        self.omitted_total_item_count() > 0
    }

    pub fn is_complete(&self) -> bool {
        !self.recent_entries_truncated()
            && !self.transcript_hits_truncated()
            && !self.memory_hits_truncated()
    }

    pub fn is_empty(&self) -> bool {
        self.report.is_empty() && self.availability.is_empty()
    }

    /// Returns a compact summary of transcript provenance attached to this
    /// inspection view.
    pub fn transcript_provenance_summary(&self) -> ContextRecallTranscriptProvenanceSummary {
        ContextRecallTranscriptProvenanceSummary::from_span_refs(&self.source_transcript_spans)
    }

    /// Returns a compact returned-vs-available coverage summary for recall
    /// sources and totals.
    pub fn coverage(&self) -> ContextRecallCoverage {
        ContextRecallCoverage::from_inspection(self)
    }

    /// Returns a compact limit-pressure summary for recall sources and totals.
    pub fn limit_pressure(&self) -> ContextRecallLimitPressure {
        ContextRecallLimitPressure::from_inspection(self)
    }

    /// Returns a payload-light semantic/episodic/control/transcript taxonomy
    /// report for this recall inspection.
    pub fn memory_taxonomy_report(
        &self,
        source_availability: &ContextRecallSourceAvailability,
        memory_control_omitted_count: usize,
    ) -> ContextMemoryTaxonomyReport {
        ContextMemoryTaxonomyReport::from_recall_inspection(
            self,
            source_availability,
            memory_control_omitted_count,
        )
    }

    /// Returns background-memory formation receipts for transcript evidence
    /// without producing memory candidates or writing durable memory.
    pub fn memory_formation_receipt_report(&self) -> ContextMemoryFormationReceiptReport {
        ContextMemoryFormationReceiptReport::from_recall_inspection(self)
    }

    /// Returns a payload-light background-memory formation queue dry-run
    /// derived from receipts without producing or writing durable memory.
    pub fn memory_formation_queue_report(&self) -> ContextMemoryFormationQueueReport {
        ContextMemoryFormationQueueReport::from_receipts(&self.memory_formation_receipt_report())
    }

    /// Returns payload-light temporal fact dry-run metadata for transcript
    /// evidence without writing graph facts or production memory.
    pub fn memory_temporal_fact_report(&self) -> ContextMemoryTemporalFactReport {
        ContextMemoryTemporalFactReport::from_recall_inspection(self)
    }

    /// Returns a payload-light temporal fact graph dry-run derived from
    /// temporal facts without writing graph facts or production memory.
    pub fn memory_temporal_fact_graph_report(&self) -> ContextMemoryTemporalFactGraphReport {
        ContextMemoryTemporalFactGraphReport::from_temporal_facts(
            &self.memory_temporal_fact_report(),
        )
    }
}
