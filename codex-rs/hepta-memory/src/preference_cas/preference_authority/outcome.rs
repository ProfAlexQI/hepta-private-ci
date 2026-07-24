use hepta_contracts::PreferenceEvidenceRef;
use hepta_contracts::PreferenceState;
use hepta_contracts::PreferenceTransitionId;

use super::bindings::PreferenceFeedbackSourceRef;
use super::bindings::PreferenceReducerRef;
use crate::preference_cas::PreferenceDocumentCommitOutcome;

/// Auditable result of one authenticated preference CAS attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreferenceAuthorityCommitOutcome {
    commit: PreferenceDocumentCommitOutcome,
    transition_id: PreferenceTransitionId,
    evidence: PreferenceEvidenceRef,
    source: PreferenceFeedbackSourceRef,
    reducer: PreferenceReducerRef,
    expected_previous: PreferenceState,
}

impl PreferenceAuthorityCommitOutcome {
    pub(in crate::preference_authority) fn new(
        commit: PreferenceDocumentCommitOutcome,
        transition_id: PreferenceTransitionId,
        evidence: PreferenceEvidenceRef,
        source: PreferenceFeedbackSourceRef,
        reducer: PreferenceReducerRef,
        expected_previous: PreferenceState,
    ) -> Self {
        Self {
            commit,
            transition_id,
            evidence,
            source,
            reducer,
            expected_previous,
        }
    }

    /// Returns the underlying exact CAS outcome.
    pub fn commit(&self) -> &PreferenceDocumentCommitOutcome {
        &self.commit
    }

    /// Reports whether this call performed the sole state advancement.
    pub const fn committed_now(&self) -> bool {
        self.commit.committed_now()
    }

    /// Returns the exact transition identity consumed by CAS.
    pub fn transition_id(&self) -> &PreferenceTransitionId {
        &self.transition_id
    }

    /// Returns the canonical authenticated evidence committed by CAS.
    pub fn evidence(&self) -> &PreferenceEvidenceRef {
        &self.evidence
    }

    /// Returns the exact authenticated source binding.
    pub fn source(&self) -> &PreferenceFeedbackSourceRef {
        &self.source
    }

    /// Returns the exact reducer binding.
    pub fn reducer(&self) -> &PreferenceReducerRef {
        &self.reducer
    }

    /// Returns the exact prior state bound by the challenge and CAS.
    pub fn expected_previous(&self) -> &PreferenceState {
        &self.expected_previous
    }
}
