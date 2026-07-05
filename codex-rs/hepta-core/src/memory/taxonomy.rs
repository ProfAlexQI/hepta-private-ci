use serde::Deserialize;
use serde::Serialize;

use super::ContextRecallInspection;
use super::ContextRecallSourceAvailability;

/// Coarse payload-light memory taxonomy class.
///
/// These buckets mirror common long-context systems without storing memory
/// contents, transcript text, ranked payloads, or source identifiers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryTaxonomyClass {
    Semantic,
    Episodic,
    Procedural,
    Control,
    Transcript,
    #[default]
    Unknown,
}

impl ContextMemoryTaxonomyClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Semantic => "semantic",
            Self::Episodic => "episodic",
            Self::Procedural => "procedural",
            Self::Control => "control",
            Self::Transcript => "transcript",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Payload-light returned-vs-available counts for one memory taxonomy class.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTaxonomyBucket {
    pub class: ContextMemoryTaxonomyClass,
    pub source_count: usize,
    pub returned_count: usize,
    pub available_count: usize,
    pub omitted_count: usize,
    pub provenance_span_count: usize,
}

impl ContextMemoryTaxonomyBucket {
    pub fn has_count_integrity(&self) -> bool {
        !self.class.is_unknown()
            && self.returned_count <= self.available_count
            && self.omitted_count == self.available_count.saturating_sub(self.returned_count)
            && (self.source_count > 0
                || (self.returned_count == 0
                    && self.available_count == 0
                    && self.omitted_count == 0
                    && self.provenance_span_count == 0))
    }

    pub fn is_empty(&self) -> bool {
        self.source_count == 0
            && self.returned_count == 0
            && self.available_count == 0
            && self.omitted_count == 0
            && self.provenance_span_count == 0
    }
}

/// Compact memory taxonomy report for recall diagnostics and future memory
/// formation planning.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTaxonomyReport {
    pub buckets: Vec<ContextMemoryTaxonomyBucket>,
}

impl ContextMemoryTaxonomyReport {
    pub fn from_recall_inspection(
        inspection: &ContextRecallInspection,
        source_availability: &ContextRecallSourceAvailability,
        memory_control_omitted_count: usize,
    ) -> Self {
        let mut buckets = Vec::new();
        let counts = &inspection.report.source_counts;

        push_nonempty_bucket(
            &mut buckets,
            ContextMemoryTaxonomyBucket {
                class: ContextMemoryTaxonomyClass::Semantic,
                source_count: usize::from(source_availability.durable_memory_match_count > 0),
                returned_count: counts.durable_memory_hit_count,
                available_count: source_availability.durable_memory_match_count,
                omitted_count: source_availability
                    .durable_memory_match_count
                    .saturating_sub(counts.durable_memory_hit_count),
                provenance_span_count: 0,
            },
        );

        push_nonempty_bucket(
            &mut buckets,
            ContextMemoryTaxonomyBucket {
                class: ContextMemoryTaxonomyClass::Episodic,
                source_count: usize::from(source_availability.summary_memory_match_count > 0),
                returned_count: counts.summary_hit_count,
                available_count: source_availability.summary_memory_match_count,
                omitted_count: source_availability
                    .summary_memory_match_count
                    .saturating_sub(counts.summary_hit_count),
                provenance_span_count: 0,
            },
        );

        push_nonempty_bucket(
            &mut buckets,
            ContextMemoryTaxonomyBucket {
                class: ContextMemoryTaxonomyClass::Control,
                source_count: usize::from(memory_control_omitted_count > 0),
                returned_count: 0,
                available_count: memory_control_omitted_count,
                omitted_count: memory_control_omitted_count,
                provenance_span_count: 0,
            },
        );

        let transcript_source_count = usize::from(source_availability.recent_entry_count > 0)
            + usize::from(source_availability.transcript_match_count > 0);
        let transcript_returned_count = counts.recent_entry_count + counts.transcript_hit_count;
        let transcript_available_count =
            source_availability.recent_entry_count + source_availability.transcript_match_count;
        let provenance_span_count = inspection.transcript_provenance_summary().span_count;
        push_nonempty_bucket(
            &mut buckets,
            ContextMemoryTaxonomyBucket {
                class: ContextMemoryTaxonomyClass::Transcript,
                source_count: transcript_source_count,
                returned_count: transcript_returned_count,
                available_count: transcript_available_count,
                omitted_count: transcript_available_count.saturating_sub(transcript_returned_count),
                provenance_span_count,
            },
        );

        Self { buckets }
    }

    pub fn has_count_integrity(&self) -> bool {
        self.buckets
            .iter()
            .all(ContextMemoryTaxonomyBucket::has_count_integrity)
    }

    pub fn is_empty(&self) -> bool {
        self.buckets.is_empty()
    }
}

fn push_nonempty_bucket(
    buckets: &mut Vec<ContextMemoryTaxonomyBucket>,
    bucket: ContextMemoryTaxonomyBucket,
) {
    if !bucket.is_empty() {
        buckets.push(bucket);
    }
}
