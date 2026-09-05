use std::error::Error;
use std::fmt;

use codex_hepta_types::StableId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperationError {
    Missing(StableId),
    Conflict(StableId),
    InvalidTransition {
        from: &'static str,
        to: &'static str,
    },
    AuthorityRejected,
    StaleGeneration,
    Terminal,
    NotClaimed,
}

impl fmt::Display for OperationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(id) => write!(formatter, "operation is missing: {id}"),
            Self::Conflict(id) => write!(formatter, "operation binding conflict: {id}"),
            Self::InvalidTransition { from, to } => {
                write!(
                    formatter,
                    "invalid operation transition from {from} to {to}"
                )
            }
            Self::AuthorityRejected => formatter.write_str("operation authority witness rejected"),
            Self::StaleGeneration => formatter.write_str("operation generation fence is stale"),
            Self::Terminal => formatter.write_str("operation is already terminal"),
            Self::NotClaimed => formatter.write_str("outbox intent is not claimed"),
        }
    }
}

impl Error for OperationError {}
