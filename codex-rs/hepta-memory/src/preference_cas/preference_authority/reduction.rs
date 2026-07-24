use std::error::Error;
use std::fmt;

use hepta_contracts::PreferenceState;

/// Reducer-owned next state and canonical payload before memory constructs CAS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreferenceReductionDraft {
    next_state: PreferenceState,
    canonical_payload: String,
}

impl PreferenceReductionDraft {
    /// Creates reducer output for memory-side transition construction.
    pub fn new(next_state: PreferenceState, canonical_payload: impl Into<String>) -> Self {
        Self {
            next_state,
            canonical_payload: canonical_payload.into(),
        }
    }

    /// Returns the reducer-owned next state.
    pub fn next_state(&self) -> &PreferenceState {
        &self.next_state
    }

    /// Returns the reducer-owned canonical next-state payload.
    pub fn canonical_payload(&self) -> &str {
        &self.canonical_payload
    }
}

/// Typed failure returned by a preference reducer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreferenceDomainReducerError {
    code: String,
}

impl PreferenceDomainReducerError {
    /// Creates a reducer-owned stable failure code.
    pub fn new(code: impl Into<String>) -> Self {
        Self { code: code.into() }
    }

    /// Returns the reducer-owned failure code.
    pub fn code(&self) -> &str {
        &self.code
    }
}

impl fmt::Display for PreferenceDomainReducerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "preference reduction failed: {}", self.code)
    }
}

impl Error for PreferenceDomainReducerError {}
