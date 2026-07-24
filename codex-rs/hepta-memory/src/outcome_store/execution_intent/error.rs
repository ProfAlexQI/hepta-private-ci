use std::error::Error;
use std::fmt;

/// Invalid pre-dispatch execution-intent material.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionIntentError {
    EmptyField { field: &'static str },
    EmptyEffectPlan,
    PayloadBindingMismatch,
}

impl fmt::Display for ExecutionIntentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(
                    formatter,
                    "execution intent field {field} must not be empty"
                )
            }
            Self::EmptyEffectPlan => {
                formatter.write_str("execution intent effect plan must not be empty when present")
            }
            Self::PayloadBindingMismatch => formatter
                .write_str("execution intent payload hash differs from its payload-set binding"),
        }
    }
}

impl Error for ExecutionIntentError {}
