use serde::Deserialize;
use serde::Serialize;

use crate::intelligence::HeptaNeuron;
use crate::intelligence::TopicSession;

use super::MemoryRecord;
use super::TranscriptEntry;
use super::TranscriptSpan;

mod bundle;
mod coverage;
mod inspection;
mod ranked;
mod request;

pub use coverage::ContextRecallCoverage;
pub use coverage::ContextRecallCoverageCounts;
pub use coverage::ContextRecallLimitPressure;
pub use coverage::ContextRecallOmissionCounts;
pub use coverage::ContextRecallTranscriptProvenanceSummary;
pub use inspection::ContextRecallAvailability;
pub use inspection::ContextRecallInspection;
pub use inspection::ContextRecallReport;
pub use inspection::ContextRecallSourceAvailability;
pub use inspection::ContextRecallSourceCounts;
pub use ranked::ContextRecallItem;
pub use ranked::ContextRecallScore;
pub use ranked::ContextRecallSource;
pub use ranked::IntelligenceTurnFrame;
pub use request::ContextBudget;
pub use request::ContextRecallRequest;

/// Runtime-facing blended recall bundle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextRecallBundle {
    pub request: ContextRecallRequest,
    #[serde(default)]
    pub recent_entries: Vec<TranscriptEntry>,
    #[serde(default)]
    pub transcript_hits: Vec<TranscriptSpan>,
    #[serde(default)]
    pub durable_memory_hits: Vec<MemoryRecord>,
    #[serde(default)]
    pub summary_hits: Vec<MemoryRecord>,
    #[serde(default)]
    pub active_topic_sessions: Vec<TopicSession>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub active_neurons: Vec<HeptaNeuron>,
    #[serde(default)]
    pub budget: ContextBudget,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ranked_items: Vec<ContextRecallItem>,
    #[serde(default)]
    pub omitted_by_budget: usize,
    pub truncated: bool,
}

pub(super) fn stable_receipt_hash(parts: &[&str]) -> String {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;

    let mut hash = OFFSET;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(PRIME);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(PRIME);
    }
    format!("{hash:016x}")
}

pub(super) fn stable_receipt_hash_is_valid(value: &str) -> bool {
    value.len() == 16 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

pub(super) fn privacy_class_is_payload_light(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        && !["memory_id", "query", "source_id", "text", "transcript"]
            .iter()
            .any(|forbidden| value.contains(forbidden))
}

pub(super) fn basis_points(numerator: usize, denominator: usize) -> u32 {
    if denominator == 0 {
        return 0;
    }

    ((numerator.saturating_mul(10_000)) / denominator) as u32
}
