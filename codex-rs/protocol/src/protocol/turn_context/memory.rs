use super::common::TURN_CONTEXT_MEMORY_FORMATION_RECEIPT_SCHEMA_VERSION;
use super::common::TURN_CONTEXT_MEMORY_TAXONOMY_SCHEMA_VERSION;
use super::common::TURN_CONTEXT_MEMORY_TEMPORAL_FACT_SCHEMA_VERSION;
use super::common::compression_candidate_source_id_is_payload_light;
use super::common::is_false;
use super::common::is_stable_manifest_replay_hash;
use super::common::is_zero_u32;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextMemoryTaxonomyClass {
    Semantic,
    Episodic,
    Procedural,
    Control,
    Transcript,
    #[default]
    Unknown,
}

impl TurnContextMemoryTaxonomyClass {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextMemoryTaxonomyBucket {
    pub class: TurnContextMemoryTaxonomyClass,
    pub source_count: u32,
    pub returned_count: u32,
    pub available_count: u32,
    pub omitted_count: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub provenance_span_count: u32,
}

impl TurnContextMemoryTaxonomyBucket {
    pub fn schema_version(&self) -> Option<u32> {
        self.has_payload_light_integrity()
            .then_some(TURN_CONTEXT_MEMORY_TAXONOMY_SCHEMA_VERSION)
    }

    pub fn has_payload_light_integrity(&self) -> bool {
        !self.class.is_unknown()
            && self.returned_count <= self.available_count
            && self.omitted_count == self.available_count.saturating_sub(self.returned_count)
            && (self.source_count > 0
                || (self.returned_count == 0
                    && self.available_count == 0
                    && self.omitted_count == 0
                    && self.provenance_span_count == 0))
    }
}

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextMemoryFormationCandidateType {
    Fact,
    Task,
    Preference,
    Decision,
    Summary,
    #[default]
    Unknown,
}

impl TurnContextMemoryFormationCandidateType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fact => "fact",
            Self::Task => "task",
            Self::Preference => "preference",
            Self::Decision => "decision",
            Self::Summary => "summary",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextMemoryFormationReceipt {
    pub candidate_type: TurnContextMemoryFormationCandidateType,
    pub transcript_span_count: u32,
    pub provenance_span_count: u32,
    pub confidence_basis_points: u32,
    pub idempotency_key_hash: String,
    pub privacy_class: String,
    pub queued_for_background: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub production_write: bool,
}

impl TurnContextMemoryFormationReceipt {
    pub fn schema_version(&self) -> Option<u32> {
        self.has_payload_light_integrity()
            .then_some(TURN_CONTEXT_MEMORY_FORMATION_RECEIPT_SCHEMA_VERSION)
    }

    pub fn has_payload_light_integrity(&self) -> bool {
        !self.candidate_type.is_unknown()
            && self.transcript_span_count > 0
            && self.provenance_span_count > 0
            && self.provenance_span_count <= self.transcript_span_count
            && self.confidence_basis_points <= 10_000
            && is_stable_manifest_replay_hash(&self.idempotency_key_hash)
            && compression_candidate_source_id_is_payload_light(&self.privacy_class)
            && self.queued_for_background
            && !self.production_write
    }
}

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextMemoryTemporalFactType {
    Attribute,
    Preference,
    TaskState,
    Decision,
    Summary,
    #[default]
    Unknown,
}

impl TurnContextMemoryTemporalFactType {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextMemoryTemporalFact {
    pub fact_type: TurnContextMemoryTemporalFactType,
    pub entity_hash: String,
    pub provenance_span_count: u32,
    pub valid_from_sequence: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub invalid_at_sequence: Option<u32>,
    pub confidence_basis_points: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub supersedes_fact_hash: Option<String>,
    pub privacy_class: String,
    pub dry_run_only: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub production_write: bool,
}

impl TurnContextMemoryTemporalFact {
    pub fn schema_version(&self) -> Option<u32> {
        self.has_payload_light_integrity()
            .then_some(TURN_CONTEXT_MEMORY_TEMPORAL_FACT_SCHEMA_VERSION)
    }

    pub fn has_payload_light_integrity(&self) -> bool {
        !self.fact_type.is_unknown()
            && is_stable_manifest_replay_hash(&self.entity_hash)
            && self.provenance_span_count > 0
            && self.valid_from_sequence > 0
            && self
                .invalid_at_sequence
                .is_none_or(|sequence| sequence > self.valid_from_sequence)
            && self.confidence_basis_points <= 10_000
            && self
                .supersedes_fact_hash
                .as_deref()
                .is_none_or(is_stable_manifest_replay_hash)
            && compression_candidate_source_id_is_payload_light(&self.privacy_class)
            && self.dry_run_only
            && !self.production_write
    }
}
