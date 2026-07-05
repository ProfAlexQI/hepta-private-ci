use std::collections::BTreeSet;

use crate::TranscriptSpanRef;
use crate::intelligence::TopicSessionStatus;

use super::super::TranscriptEntry;
use super::super::TranscriptRange;
use super::ContextBudget;
use super::ContextRecallAvailability;
use super::ContextRecallBundle;
use super::ContextRecallInspection;
use super::ContextRecallReport;
use super::ContextRecallSourceCounts;
use super::ContextRecallTranscriptProvenanceSummary;

const DEFAULT_CONTEXT_RECALL_TRANSCRIPT_PROVENANCE_LIMIT: usize = 8;

impl ContextRecallBundle {
    /// Returns a bounded set of lightweight transcript provenance refs for the
    /// evidence used by this blended recall bundle.
    pub fn source_transcript_spans(&self) -> Vec<TranscriptSpanRef> {
        let mut refs = Vec::new();

        if let Some(range) = transcript_range_for_entries(&self.recent_entries) {
            upsert_context_recall_transcript_span_ref(
                &mut refs,
                TranscriptSpanRef {
                    session_id: self.recent_entries[0].session_id.clone(),
                    range,
                    reason: Some("recent_window".to_string()),
                },
            );
        }

        for hit in &self.transcript_hits {
            upsert_context_recall_transcript_span_ref(
                &mut refs,
                TranscriptSpanRef {
                    session_id: hit.session_id.clone(),
                    range: hit.range.clone(),
                    reason: Some("query_match".to_string()),
                },
            );
        }

        for topic_session in self
            .active_topic_sessions
            .iter()
            .filter(|topic_session| matches!(topic_session.status, TopicSessionStatus::Active))
        {
            for span in &topic_session.linked_transcript_spans {
                upsert_context_recall_transcript_span_ref(
                    &mut refs,
                    TranscriptSpanRef {
                        session_id: span.session_id.clone(),
                        range: span.range.clone(),
                        reason: merge_context_recall_transcript_span_reasons(
                            span.reason.as_deref(),
                            Some("active_topic_session"),
                        ),
                    },
                );
            }
        }

        refs.sort_by(|left, right| {
            right
                .range
                .end_sequence
                .cmp(&left.range.end_sequence)
                .then_with(|| right.range.start_sequence.cmp(&left.range.start_sequence))
                .then_with(|| left.session_id.0.cmp(&right.session_id.0))
        });
        refs.truncate(DEFAULT_CONTEXT_RECALL_TRANSCRIPT_PROVENANCE_LIMIT);
        refs
    }

    /// Returns a compact per-source count summary for this recall bundle.
    pub fn source_counts(&self) -> ContextRecallSourceCounts {
        ContextRecallSourceCounts {
            recent_entry_count: self.recent_entries.len(),
            transcript_hit_count: self.transcript_hits.len(),
            durable_memory_hit_count: self.durable_memory_hits.len(),
            summary_hit_count: self.summary_hits.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.source_counts().is_empty()
    }

    pub fn query_hit_count(&self) -> usize {
        self.source_counts().query_hit_count()
    }

    pub fn total_item_count(&self) -> usize {
        self.source_counts().total_item_count()
    }

    pub fn has_query_matches(&self) -> bool {
        self.source_counts().has_query_matches()
    }

    pub fn active_topic_session_count(&self) -> usize {
        self.active_topic_sessions.len()
    }

    pub fn active_neuron_count(&self) -> usize {
        self.active_neurons.len()
    }

    pub fn ensure_budget(&mut self) {
        if self.budget.max_items == 0 {
            self.budget = ContextBudget::from_request(&self.request);
        }
    }

    /// Returns a compact summary of transcript provenance attached to this
    /// recall bundle.
    pub fn transcript_provenance_summary(&self) -> ContextRecallTranscriptProvenanceSummary {
        ContextRecallTranscriptProvenanceSummary::from_span_refs(&self.source_transcript_spans())
    }

    /// Returns a payload-light report that preserves request, counts, and
    /// truncation state for diagnostics and automation.
    pub fn report(&self) -> ContextRecallReport {
        ContextRecallReport::from_bundle(self)
    }

    /// Returns a payload-light inspection view that pairs returned counts with
    /// pre-limit availability counts.
    pub fn inspection(&self, availability: ContextRecallAvailability) -> ContextRecallInspection {
        ContextRecallInspection::from_bundle(self, availability)
    }
}

fn transcript_range_for_entries(entries: &[TranscriptEntry]) -> Option<TranscriptRange> {
    let start = entries.first()?.sequence;
    let end = entries.last()?.sequence;
    Some(TranscriptRange {
        start_sequence: start,
        end_sequence: end,
    })
}

fn upsert_context_recall_transcript_span_ref(
    refs: &mut Vec<TranscriptSpanRef>,
    incoming: TranscriptSpanRef,
) {
    if let Some(existing) = refs.iter_mut().find(|existing| {
        existing.session_id == incoming.session_id && existing.range == incoming.range
    }) {
        existing.reason = merge_context_recall_transcript_span_reasons(
            existing.reason.as_deref(),
            incoming.reason.as_deref(),
        );
        return;
    }

    refs.push(incoming);
}

fn merge_context_recall_transcript_span_reasons(
    existing: Option<&str>,
    incoming: Option<&str>,
) -> Option<String> {
    let mut merged = Vec::new();
    let mut seen = BTreeSet::new();

    for reason in [existing, incoming].into_iter().flatten() {
        for part in reason
            .split(',')
            .map(str::trim)
            .filter(|part| !part.is_empty())
        {
            if seen.insert(part.to_string()) {
                merged.push(part.to_string());
            }
        }
    }

    (!merged.is_empty()).then(|| merged.join(", "))
}
