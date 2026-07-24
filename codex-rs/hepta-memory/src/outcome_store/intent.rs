use super::OutcomeRecord;

/// Durable producer-intent state recovered before terminal receipt ACK.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutcomeIntentState {
    /// Exact receipt material is durable but its outcome transaction is absent.
    Pending,
    /// The exact outcome committed, but producer acknowledgement did not finish.
    Committed,
}

impl OutcomeIntentState {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Committed => "committed",
        }
    }

    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "pending" => Some(Self::Pending),
            "committed" => Some(Self::Committed),
            _ => None,
        }
    }
}

/// Exact producer material retained until terminal receipt acknowledgement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutcomeIntent {
    state: OutcomeIntentState,
    record: OutcomeRecord,
}

impl OutcomeIntent {
    pub(crate) fn new(state: OutcomeIntentState, record: OutcomeRecord) -> Self {
        Self { state, record }
    }

    /// Returns whether the outcome is pending commit or pending acknowledgement.
    pub const fn state(&self) -> OutcomeIntentState {
        self.state
    }

    /// Returns the exact attempt, receipt, and evidence material.
    pub const fn record(&self) -> &OutcomeRecord {
        &self.record
    }
}

/// Result of durably staging exact producer material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutcomeIntentStageResult {
    /// A durable pending intent exists and still requires outcome commit.
    Pending,
    /// A durable committed intent exists and requires exact reconciliation.
    Committed,
    /// The exact outcome was previously acknowledged; no unresolved intent exists.
    AlreadyRecorded,
}
