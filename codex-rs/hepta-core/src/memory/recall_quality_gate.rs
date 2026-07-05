mod fixture;
mod report;

use serde::Deserialize;
use serde::Serialize;

pub use fixture::ContextMemoryRecallQualityFixtureGateReport;
pub use fixture::ContextMemoryRecallQualityGateBlockerReason;
pub use report::ContextMemoryRecallQualityGateReport;

/// Payload-light verdict for the offline recall-quality gate.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryRecallQualityGateVerdict {
    GatePass,
    Blocked,
    #[default]
    Unknown,
}

impl ContextMemoryRecallQualityGateVerdict {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}
