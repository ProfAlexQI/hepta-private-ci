use std::collections::BTreeMap;

use hepta_contracts::PreferenceState;
use hepta_contracts::PreferenceTransition;
use hepta_contracts::PreferenceTransitionId;
use hepta_contracts::ReceiptId;
use hepta_contracts::ReceiptRef;

use super::InMemoryPreferenceStore;
use super::PreferenceCasError;
use super::PreferenceKey;

#[derive(Default)]
pub(super) struct LegacyPreferenceStoreState {
    preferences: BTreeMap<PreferenceKey, PreferenceState>,
    transitions: BTreeMap<PreferenceTransitionId, PreferenceTransition>,
    receipts: BTreeMap<ReceiptId, LegacyReceiptUse>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LegacyReceiptUse {
    receipt: ReceiptRef,
    transition: PreferenceTransitionId,
}

/// Result of seeding the compatibility-only preference state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreferenceSeedOutcome {
    /// No state existed for the subject/preference key, so it was inserted.
    Seeded,
    /// The exact state was already present; no mutation was needed.
    AlreadySeeded,
}

/// Result of committing through the compatibility-only CAS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreferenceCommitOutcome {
    /// The exact expected state matched and the next state was committed.
    Committed {
        /// State committed by the transition.
        state: PreferenceState,
    },
    /// The exact same transition had already committed.
    AlreadyCommitted {
        /// State originally committed by the transition.
        state: PreferenceState,
    },
}

impl PreferenceCommitOutcome {
    /// Returns the state committed by the original transition.
    pub fn state(&self) -> &PreferenceState {
        match self {
            Self::Committed { state } | Self::AlreadyCommitted { state } => state,
        }
    }

    /// Reports whether this call performed the commit.
    pub const fn committed_now(&self) -> bool {
        matches!(self, Self::Committed { .. })
    }
}

impl InMemoryPreferenceStore {
    /// Seeds untrusted compatibility state outside the V2 document index.
    ///
    /// This method exists for older tests and callers. It neither initializes
    /// a V2 genesis document nor grants authority to update live preferences.
    pub fn seed(
        &self,
        preference: hepta_contracts::PreferenceId,
        subject: hepta_contracts::PrincipalId,
        state: PreferenceState,
    ) -> Result<PreferenceSeedOutcome, PreferenceCasError> {
        let mut guard = self.lock_state()?;
        let key = PreferenceKey::new(preference, subject);
        match guard.legacy.preferences.get(&key) {
            Some(existing) if existing == &state => Ok(PreferenceSeedOutcome::AlreadySeeded),
            Some(existing) => Err(PreferenceCasError::SeedConflict {
                preference: key.preference,
                subject: key.subject,
                existing: existing.clone(),
                attempted: state,
            }),
            None => {
                guard.legacy.preferences.insert(key, state);
                Ok(PreferenceSeedOutcome::Seeded)
            }
        }
    }

    /// Reads untrusted compatibility state outside the V2 document index.
    pub fn read(
        &self,
        preference: &hepta_contracts::PreferenceId,
        subject: &hepta_contracts::PrincipalId,
    ) -> Result<Option<PreferenceState>, PreferenceCasError> {
        let guard = self.lock_state()?;
        let key = PreferenceKey::new(preference.clone(), subject.clone());
        Ok(guard.legacy.preferences.get(&key).cloned())
    }

    /// Commits through the isolated, untrusted compatibility CAS.
    ///
    /// This method does not populate the V2 evidence, document, transition, or
    /// receipt indexes. Use `commit_evidenced` for the stricter reference model.
    pub fn commit(
        &self,
        transition: PreferenceTransition,
    ) -> Result<PreferenceCommitOutcome, PreferenceCasError> {
        let mut guard = self.lock_state()?;
        if let Some(existing) = guard.legacy.transitions.get(transition.id()) {
            return if existing == &transition {
                Ok(PreferenceCommitOutcome::AlreadyCommitted {
                    state: existing.committed_next().clone(),
                })
            } else {
                Err(PreferenceCasError::TransitionReuseConflict {
                    transition: transition.id().clone(),
                })
            };
        }
        if let Some(existing) = guard.legacy.receipts.get(transition.caused_by().id()) {
            return Err(PreferenceCasError::ReceiptReuseConflict {
                receipt: transition.caused_by().id().clone(),
                existing_receipt: Box::new(existing.receipt.clone()),
                attempted_receipt: Box::new(transition.caused_by().clone()),
                existing_transition: existing.transition.clone(),
                attempted_transition: transition.id().clone(),
            });
        }

        let key = PreferenceKey::from_transition(&transition);
        let Some(current) = guard.legacy.preferences.get(&key) else {
            return Err(PreferenceCasError::PreferenceNotSeeded {
                preference: key.preference,
                subject: key.subject,
            });
        };
        if current != transition.cas_expected_previous() {
            return Err(PreferenceCasError::StateConflict {
                preference: key.preference,
                subject: key.subject,
                expected: transition.cas_expected_previous().clone(),
                actual: current.clone(),
            });
        }

        let committed = transition.committed_next().clone();
        let receipt = transition.caused_by().clone();
        let transition_id = transition.id().clone();
        guard.legacy.preferences.insert(key, committed.clone());
        guard.legacy.receipts.insert(
            receipt.id().clone(),
            LegacyReceiptUse {
                receipt,
                transition: transition_id.clone(),
            },
        );
        guard.legacy.transitions.insert(transition_id, transition);
        Ok(PreferenceCommitOutcome::Committed { state: committed })
    }
}
