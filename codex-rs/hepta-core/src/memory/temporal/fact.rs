use serde::Deserialize;
use serde::Serialize;

use super::super::ContextRecallInspection;
use super::super::privacy_class_is_payload_light;
use super::super::stable_receipt_hash;
use super::super::stable_receipt_hash_is_valid;

/// Temporal fact type for future background memory formation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryTemporalFactType {
    Attribute,
    Preference,
    TaskState,
    Decision,
    Summary,
    #[default]
    Unknown,
}

impl ContextMemoryTemporalFactType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Attribute => "attribute",
            Self::Preference => "preference",
            Self::TaskState => "task_state",
            Self::Decision => "decision",
            Self::Summary => "summary",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Payload-light dry-run shape for future temporal memory facts.
///
/// This intentionally carries no entity text, fact text, transcript text,
/// source identifiers, or memory identifiers.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTemporalFact {
    pub fact_type: ContextMemoryTemporalFactType,
    pub entity_hash: String,
    pub provenance_span_count: usize,
    pub valid_from_sequence: u64,
    pub invalid_at_sequence: Option<u64>,
    pub confidence_basis_points: u32,
    pub supersedes_fact_hash: Option<String>,
    pub privacy_class: String,
    pub dry_run_only: bool,
    pub production_write: bool,
}

impl ContextMemoryTemporalFact {
    pub fn has_temporal_fact_integrity(&self) -> bool {
        !self.fact_type.is_unknown()
            && stable_receipt_hash_is_valid(&self.entity_hash)
            && self.provenance_span_count > 0
            && self.valid_from_sequence > 0
            && self
                .invalid_at_sequence
                .is_none_or(|sequence| sequence > self.valid_from_sequence)
            && self.confidence_basis_points <= 10_000
            && self
                .supersedes_fact_hash
                .as_deref()
                .is_none_or(stable_receipt_hash_is_valid)
            && privacy_class_is_payload_light(&self.privacy_class)
            && self.dry_run_only
            && !self.production_write
    }
}

/// Dry-run report for future temporal memory facts.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTemporalFactReport {
    pub facts: Vec<ContextMemoryTemporalFact>,
}

impl ContextMemoryTemporalFactReport {
    pub fn from_recall_inspection(inspection: &ContextRecallInspection) -> Self {
        let provenance = inspection.transcript_provenance_summary();
        if provenance.span_count == 0 {
            return Self::default();
        }

        let provenance_span_count = provenance.span_count;
        let valid_from_sequence = inspection
            .source_transcript_spans
            .iter()
            .map(|span| span.range.start_sequence)
            .min()
            .unwrap_or(1);
        let privacy_class = "user_private";
        let fact_specs = [
            (ContextMemoryTemporalFactType::Attribute, 6200),
            (ContextMemoryTemporalFactType::Preference, 5600),
            (ContextMemoryTemporalFactType::TaskState, 5400),
            (ContextMemoryTemporalFactType::Decision, 5800),
            (ContextMemoryTemporalFactType::Summary, 7000),
        ];
        let facts = fact_specs
            .into_iter()
            .map(|(fact_type, confidence_basis_points)| {
                let entity_hash = stable_receipt_hash(&[
                    "memory_temporal_fact_entity",
                    fact_type.as_str(),
                    &inspection.report.request.session_id.0,
                    &provenance_span_count.to_string(),
                    &valid_from_sequence.to_string(),
                ]);
                ContextMemoryTemporalFact {
                    fact_type,
                    entity_hash,
                    provenance_span_count,
                    valid_from_sequence,
                    invalid_at_sequence: None,
                    confidence_basis_points,
                    supersedes_fact_hash: None,
                    privacy_class: privacy_class.to_string(),
                    dry_run_only: true,
                    production_write: false,
                }
            })
            .collect();

        Self { facts }
    }

    pub fn has_temporal_fact_integrity(&self) -> bool {
        self.facts
            .iter()
            .all(ContextMemoryTemporalFact::has_temporal_fact_integrity)
    }

    pub fn is_empty(&self) -> bool {
        self.facts.is_empty()
    }
}
