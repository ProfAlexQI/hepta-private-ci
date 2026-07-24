use std::error::Error;
use std::fmt;

use hepta_contracts::ContractError;

use super::bindings::PreferenceFeedbackSourceRef;
use super::bindings::PreferenceReducerRef;
use super::feedback::PreferenceFeedbackAuthenticationError;
use super::reduction::PreferenceDomainReducerError;
use crate::preference_cas::PreferenceCasError;

/// Typed failure from authenticated preference authority.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PreferenceAuthorityError {
    /// A required opaque identity or digest is empty.
    EmptyBinding {
        /// Stable field name.
        field: &'static str,
    },
    /// The configured source changed while authenticating one challenge.
    SourceBindingChanged {
        /// Source bound before authentication.
        expected: PreferenceFeedbackSourceRef,
        /// Source reported after authentication.
        actual: PreferenceFeedbackSourceRef,
    },
    /// The configured reducer changed while reducing one authenticated signal.
    ReducerBindingChanged {
        /// Reducer bound before authentication.
        expected: PreferenceReducerRef,
        /// Reducer reported after reduction.
        actual: PreferenceReducerRef,
    },
    /// The current document belongs to a different reducer version.
    ReducerVersionConflict {
        /// Current document reducer version.
        current: String,
        /// Authority-bound reducer version.
        authority: String,
    },
    /// The trusted feedback source denied the challenge.
    Authentication(PreferenceFeedbackAuthenticationError),
    /// The exact reducer failed.
    Reduction(PreferenceDomainReducerError),
    /// The reducer output violated stable transition invariants.
    Contract(ContractError),
    /// The backing preference store denied or failed the exact CAS.
    Cas(PreferenceCasError),
}

impl fmt::Display for PreferenceAuthorityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyBinding { field } => {
                write!(formatter, "preference binding {field} is empty")
            }
            Self::SourceBindingChanged { .. } => {
                formatter.write_str("preference feedback source binding changed")
            }
            Self::ReducerBindingChanged { .. } => {
                formatter.write_str("preference reducer binding changed")
            }
            Self::ReducerVersionConflict { current, authority } => write!(
                formatter,
                "preference reducer version {authority} does not match current {current}"
            ),
            Self::Authentication(error) => error.fmt(formatter),
            Self::Reduction(error) => error.fmt(formatter),
            Self::Contract(error) => error.fmt(formatter),
            Self::Cas(error) => error.fmt(formatter),
        }
    }
}

impl Error for PreferenceAuthorityError {}

impl From<PreferenceCasError> for PreferenceAuthorityError {
    fn from(error: PreferenceCasError) -> Self {
        Self::Cas(error)
    }
}

impl From<ContractError> for PreferenceAuthorityError {
    fn from(error: ContractError) -> Self {
        Self::Contract(error)
    }
}

pub(super) fn require_nonempty(
    field: &'static str,
    value: &str,
) -> Result<(), PreferenceAuthorityError> {
    if value.is_empty() {
        Err(PreferenceAuthorityError::EmptyBinding { field })
    } else {
        Ok(())
    }
}
