use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

use crate::TranscriptSpanRef;

use super::ContextRecallInspection;

/// Compact transcript-provenance summary for blended recall.
///
/// This lets automation and doctor-style checks reason about how much
/// transcript evidence a recall result carries, how many sessions that
/// evidence spans, and whether provenance reasons were preserved, without
/// embedding the individual `TranscriptSpanRef` payloads.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextRecallTranscriptProvenanceSummary {
    pub span_count: usize,
    pub session_count: usize,
    pub spans_with_reason_count: usize,
    pub distinct_reason_count: usize,
}

impl ContextRecallTranscriptProvenanceSummary {
    pub fn from_span_refs(spans: &[TranscriptSpanRef]) -> Self {
        let mut session_ids = BTreeSet::new();
        let mut reasons = BTreeSet::new();
        let mut spans_with_reason_count = 0;

        for span in spans {
            let session_id = span.session_id.0.trim();
            if !session_id.is_empty() {
                session_ids.insert(session_id.to_string());
            }

            let mut has_reason = false;
            for reason in span.reason.as_deref().into_iter().flat_map(|reason| {
                reason
                    .split(',')
                    .map(str::trim)
                    .filter(|part| !part.is_empty())
                    .collect::<Vec<_>>()
            }) {
                has_reason = true;
                reasons.insert(reason.to_string());
            }

            if has_reason {
                spans_with_reason_count += 1;
            }
        }

        Self {
            span_count: spans.len(),
            session_count: session_ids.len(),
            spans_with_reason_count,
            distinct_reason_count: reasons.len(),
        }
    }

    pub fn has_spans(&self) -> bool {
        self.span_count > 0
    }

    pub fn has_reasons(&self) -> bool {
        self.distinct_reason_count > 0
    }

    pub fn is_empty(&self) -> bool {
        self.span_count == 0
    }
}

/// Compact returned-vs-available counts for one recall source.
///
/// This lets automation observe omission pressure and completeness without
/// diffing counts by hand.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextRecallCoverageCounts {
    pub returned_count: usize,
    pub available_count: usize,
}

impl ContextRecallCoverageCounts {
    pub fn omitted_count(&self) -> usize {
        self.available_count.saturating_sub(self.returned_count)
    }

    pub fn is_complete(&self) -> bool {
        self.returned_count == self.available_count
    }

    pub fn is_empty(&self) -> bool {
        self.available_count == 0
    }

    pub fn is_truncated(&self) -> bool {
        self.returned_count < self.available_count
    }
}

/// Compact omitted-item counts for blended recall sources and totals.
///
/// This gives automation a machine-readable summary of what recent-window or
/// query limits left behind without requiring callers to diff returned-vs-
/// available coverage counts by hand.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextRecallOmissionCounts {
    pub recent_entry_count: usize,
    pub transcript_hit_count: usize,
    pub memory_hit_count: usize,
    pub query_hit_count: usize,
    pub total_item_count: usize,
}

impl ContextRecallOmissionCounts {
    pub fn has_omissions(&self) -> bool {
        self.total_item_count > 0
    }

    pub fn is_empty(&self) -> bool {
        self.total_item_count == 0
    }
}

/// Compact summary of whether recall limits clipped any source.
///
/// Unlike [`ContextRecallCoverage`], this payload focuses on omission pressure
/// and truncation flags instead of returned-vs-available counts.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextRecallLimitPressure {
    pub recent_entries_truncated: bool,
    pub transcript_hits_truncated: bool,
    pub memory_hits_truncated: bool,
    pub omission_counts: ContextRecallOmissionCounts,
}

impl ContextRecallLimitPressure {
    pub fn from_inspection(inspection: &ContextRecallInspection) -> Self {
        Self {
            recent_entries_truncated: inspection.recent_entries_truncated(),
            transcript_hits_truncated: inspection.transcript_hits_truncated(),
            memory_hits_truncated: inspection.memory_hits_truncated(),
            omission_counts: inspection.omission_counts(),
        }
    }

    pub fn from_coverage(coverage: &ContextRecallCoverage) -> Self {
        Self {
            recent_entries_truncated: coverage.recent_entries.is_truncated(),
            transcript_hits_truncated: coverage.transcript_hits.is_truncated(),
            memory_hits_truncated: coverage.memory_hits.is_truncated(),
            omission_counts: coverage.omission_counts(),
        }
    }

    pub fn query_hits_truncated(&self) -> bool {
        self.transcript_hits_truncated || self.memory_hits_truncated
    }

    pub fn has_omissions(&self) -> bool {
        self.omission_counts.has_omissions()
    }

    pub fn is_complete(&self) -> bool {
        !self.recent_entries_truncated
            && !self.transcript_hits_truncated
            && !self.memory_hits_truncated
    }

    pub fn is_empty(&self) -> bool {
        self.is_complete() && self.omission_counts.is_empty()
    }
}

/// Compact returned-vs-available coverage summary for blended recall.
///
/// Unlike [`ContextRecallInspection`], this payload omits the original request
/// and collapses source totals into machine-readable coverage counts that are
/// easy to ship through automation and audit trails.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextRecallCoverage {
    pub recent_entries: ContextRecallCoverageCounts,
    pub transcript_hits: ContextRecallCoverageCounts,
    pub memory_hits: ContextRecallCoverageCounts,
    pub query_hits: ContextRecallCoverageCounts,
    pub total_items: ContextRecallCoverageCounts,
}

impl ContextRecallCoverage {
    pub fn from_inspection(inspection: &ContextRecallInspection) -> Self {
        let recent_entries = ContextRecallCoverageCounts {
            returned_count: inspection.report.source_counts.recent_entry_count,
            available_count: inspection.availability.total_recent_entry_count,
        };
        let transcript_hits = ContextRecallCoverageCounts {
            returned_count: inspection.report.source_counts.transcript_hit_count,
            available_count: inspection.availability.total_transcript_match_count,
        };
        let memory_hits = ContextRecallCoverageCounts {
            returned_count: inspection.returned_memory_hit_count(),
            available_count: inspection.availability.total_memory_match_count,
        };
        let query_hits = ContextRecallCoverageCounts {
            returned_count: inspection.returned_query_hit_count(),
            available_count: inspection.matched_query_hit_count(),
        };
        let total_items = ContextRecallCoverageCounts {
            returned_count: inspection.returned_total_item_count(),
            available_count: inspection.matched_total_item_count(),
        };

        Self {
            recent_entries,
            transcript_hits,
            memory_hits,
            query_hits,
            total_items,
        }
    }

    pub fn omitted_total_item_count(&self) -> usize {
        self.total_items.omitted_count()
    }

    /// Returns a compact omitted-item summary for each recall source.
    pub fn omission_counts(&self) -> ContextRecallOmissionCounts {
        ContextRecallOmissionCounts {
            recent_entry_count: self.recent_entries.omitted_count(),
            transcript_hit_count: self.transcript_hits.omitted_count(),
            memory_hit_count: self.memory_hits.omitted_count(),
            query_hit_count: self.query_hits.omitted_count(),
            total_item_count: self.total_items.omitted_count(),
        }
    }

    pub fn has_omissions(&self) -> bool {
        self.omitted_total_item_count() > 0
    }

    pub fn is_complete(&self) -> bool {
        self.recent_entries.is_complete()
            && self.transcript_hits.is_complete()
            && self.memory_hits.is_complete()
            && self.query_hits.is_complete()
            && self.total_items.is_complete()
    }

    pub fn is_empty(&self) -> bool {
        self.total_items.is_empty()
    }

    /// Returns a compact limit-pressure summary for recall sources and totals.
    pub fn limit_pressure(&self) -> ContextRecallLimitPressure {
        ContextRecallLimitPressure::from_coverage(self)
    }
}
