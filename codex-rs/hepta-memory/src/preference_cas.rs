//! In-memory and durable, non-live preference document and CAS primitives.
//!
//! The store accepts caller-constructed contracts and canonical payloads. It
//! checks exact identity, replay, and compare-and-swap invariants, but does not
//! authenticate evidence, run a reducer, or project changes into runtime,
//! neurons, routing, KG, or live preferences. The durable implementation owns
//! SQLite-WAL persistence and a separate canonical-row integrity hash.

mod durable;
mod legacy;

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;

use hepta_contracts::PreferenceEvidenceId;
use hepta_contracts::PreferenceEvidenceRef;
use hepta_contracts::PreferenceId;
use hepta_contracts::PreferenceState;
use hepta_contracts::PreferenceTransition;
use hepta_contracts::PreferenceTransitionId;
use hepta_contracts::PrincipalId;
use hepta_contracts::ReceiptId;
use hepta_contracts::ReceiptRef;
use hepta_contracts::Revision;

pub use durable::DurablePreferenceStore;
pub use legacy::PreferenceCommitOutcome;
pub use legacy::PreferenceSeedOutcome;

/// Exact state plus the versioned canonical payload used to derive its hash.
///
/// This type does not calculate or validate `PreferenceState::content_hash`;
/// canonicalization and hashing remain the reducer's responsibility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreferenceStateDocument {
    state: PreferenceState,
    reducer_version: String,
    canonical_payload: String,
}

impl PreferenceStateDocument {
    /// Creates an opaque state document without interpreting its payload.
    pub fn new(
        state: PreferenceState,
        reducer_version: impl Into<String>,
        canonical_payload: impl Into<String>,
    ) -> Self {
        Self {
            state,
            reducer_version: reducer_version.into(),
            canonical_payload: canonical_payload.into(),
        }
    }

    /// Returns the exact state reference for this document.
    pub fn state(&self) -> &PreferenceState {
        &self.state
    }

    /// Returns the reducer version that owns the canonical payload.
    pub fn reducer_version(&self) -> &str {
        &self.reducer_version
    }

    /// Returns the uninterpreted canonical preference payload.
    pub fn canonical_payload(&self) -> &str {
        &self.canonical_payload
    }
}

/// Result of deterministic V2 genesis initialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreferenceGenesisOutcome {
    /// The exact revision-zero document was initialized.
    Initialized,
    /// The exact same revision-zero document already existed.
    AlreadyInitialized,
}

/// Result of committing an evidenced V2 preference document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreferenceDocumentCommitOutcome {
    /// The document was committed by this call.
    Committed {
        /// Exact committed document.
        document: PreferenceStateDocument,
    },
    /// The exact transition and document had already committed.
    AlreadyCommitted {
        /// Exact document committed by the original call.
        document: PreferenceStateDocument,
    },
}

impl PreferenceDocumentCommitOutcome {
    /// Returns the document committed by the original transition.
    pub fn document(&self) -> &PreferenceStateDocument {
        match self {
            Self::Committed { document } | Self::AlreadyCommitted { document } => document,
        }
    }

    /// Reports whether this call performed the commit.
    pub const fn committed_now(&self) -> bool {
        matches!(self, Self::Committed { .. })
    }
}

/// Non-durable, caller-untrusted implementation of preference CAS.
///
/// Clones share one mutex. Legacy state and V2 document/evidence indexes are
/// intentionally isolated so compatibility calls cannot become V2 authority.
#[derive(Clone, Default)]
pub struct InMemoryPreferenceStore {
    state: Arc<Mutex<PreferenceStoreState>>,
}

#[derive(Default)]
struct PreferenceStoreState {
    legacy: legacy::LegacyPreferenceStoreState,
    documents: BTreeMap<PreferenceKey, PreferenceStateDocument>,
    transitions: BTreeMap<PreferenceTransitionId, EvidencedTransitionUse>,
    evidences: BTreeMap<PreferenceEvidenceId, EvidenceUse>,
    receipts: BTreeMap<ReceiptId, ReceiptUse>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PreferenceKey {
    preference: PreferenceId,
    subject: PrincipalId,
}

impl PreferenceKey {
    fn new(preference: PreferenceId, subject: PrincipalId) -> Self {
        Self {
            preference,
            subject,
        }
    }

    fn from_transition(transition: &PreferenceTransition) -> Self {
        Self::new(
            transition.evidence().preference().clone(),
            transition.evidence().subject().clone(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EvidencedTransitionUse {
    transition: PreferenceTransition,
    document: PreferenceStateDocument,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EvidenceUse {
    evidence: PreferenceEvidenceRef,
    transition: PreferenceTransitionId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReceiptUse {
    receipt: ReceiptRef,
    transition: PreferenceTransitionId,
}

/// Typed failure returned by a preference CAS store.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PreferenceCasError {
    /// The store mutex was poisoned by a panicking holder.
    StorePoisoned,
    /// Durable storage could not complete an I/O or SQLite operation.
    Persistence {
        /// Stable operation label.
        operation: &'static str,
        /// Backend error detail.
        detail: String,
    },
    /// Durable bytes, WAL replay, or the head projection are invalid.
    Corrupt {
        /// Fail-closed corruption detail.
        detail: String,
    },
    /// Compatibility-only seed state conflicts with an earlier seed.
    SeedConflict {
        /// Conflicting preference identity.
        preference: PreferenceId,
        /// Conflicting subject identity.
        subject: PrincipalId,
        /// Existing compatibility state.
        existing: PreferenceState,
        /// Attempted compatibility state.
        attempted: PreferenceState,
    },
    /// Compatibility-only state was not seeded for the exact key.
    PreferenceNotSeeded {
        /// Requested preference.
        preference: PreferenceId,
        /// Requested subject.
        subject: PrincipalId,
    },
    /// An exact CAS expectation did not match current state.
    StateConflict {
        /// Requested preference.
        preference: PreferenceId,
        /// Requested subject.
        subject: PrincipalId,
        /// Expected exact state.
        expected: PreferenceState,
        /// Actual exact state.
        actual: PreferenceState,
    },
    /// A V2 genesis document did not use revision zero.
    NonZeroGenesis {
        /// Attempted genesis revision.
        attempted: Revision,
    },
    /// A different V2 genesis payload or reducer version already exists.
    GenesisConflict {
        /// Existing immutable revision-zero genesis document.
        existing: Box<PreferenceStateDocument>,
        /// Different attempted genesis document.
        attempted: Box<PreferenceStateDocument>,
    },
    /// No V2 document was initialized for the exact key.
    PreferenceDocumentNotInitialized {
        /// Requested preference.
        preference: PreferenceId,
        /// Requested subject.
        subject: PrincipalId,
    },
    /// The supplied next document does not equal the transition's next state.
    CommittedDocumentStateMismatch {
        /// State committed by the transition contract.
        expected: PreferenceState,
        /// State carried by the supplied document.
        attempted: PreferenceState,
    },
    /// A commit attempted an implicit reducer-version migration.
    ReducerVersionConflict {
        /// Reducer version of the current document.
        existing: String,
        /// Reducer version of the attempted document.
        attempted: String,
    },
    /// A transition identity was reused with different content.
    TransitionReuseConflict {
        /// Reused transition identity.
        transition: PreferenceTransitionId,
    },
    /// An evidence identity was reused or drifted.
    EvidenceReuseConflict {
        /// Reused evidence identity.
        evidence: PreferenceEvidenceId,
        /// Exact evidence recorded first.
        existing_evidence: Box<PreferenceEvidenceRef>,
        /// Exact evidence supplied later.
        attempted_evidence: Box<PreferenceEvidenceRef>,
        /// Transition that first consumed the evidence.
        existing_transition: PreferenceTransitionId,
        /// Transition attempting the reuse.
        attempted_transition: PreferenceTransitionId,
    },
    /// A receipt identity was reused by a different transition.
    ReceiptReuseConflict {
        /// Reused receipt identity.
        receipt: ReceiptId,
        /// Exact receipt recorded first.
        existing_receipt: Box<ReceiptRef>,
        /// Exact receipt supplied later.
        attempted_receipt: Box<ReceiptRef>,
        /// Transition that first consumed the receipt.
        existing_transition: PreferenceTransitionId,
        /// Transition attempting the reuse.
        attempted_transition: PreferenceTransitionId,
    },
}

impl fmt::Display for PreferenceCasError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StorePoisoned => formatter.write_str("preference store mutex poisoned"),
            Self::Persistence { operation, detail } => {
                write!(formatter, "preference store {operation} failed: {detail}")
            }
            Self::Corrupt { detail } => {
                write!(formatter, "preference store is corrupt: {detail}")
            }
            Self::SeedConflict { .. } => formatter.write_str("compatibility seed conflicts"),
            Self::PreferenceNotSeeded { .. } => formatter.write_str("preference not seeded"),
            Self::StateConflict { .. } => formatter.write_str("preference CAS state conflicts"),
            Self::NonZeroGenesis { attempted } => {
                write!(
                    formatter,
                    "preference genesis revision {attempted} is not zero"
                )
            }
            Self::GenesisConflict { .. } => formatter.write_str("preference genesis conflicts"),
            Self::PreferenceDocumentNotInitialized { .. } => {
                formatter.write_str("preference document is not initialized")
            }
            Self::CommittedDocumentStateMismatch { .. } => {
                formatter.write_str("committed document does not match transition")
            }
            Self::ReducerVersionConflict {
                existing,
                attempted,
            } => write!(
                formatter,
                "reducer version {attempted} does not match current {existing}"
            ),
            Self::TransitionReuseConflict { transition } => {
                write!(formatter, "preference transition {transition} was reused")
            }
            Self::EvidenceReuseConflict { evidence, .. } => {
                write!(formatter, "preference evidence {evidence} was reused")
            }
            Self::ReceiptReuseConflict { receipt, .. } => {
                write!(formatter, "receipt {receipt} was reused")
            }
        }
    }
}

impl Error for PreferenceCasError {}

impl InMemoryPreferenceStore {
    /// Initializes an exact revision-zero document or returns exact replay.
    pub fn get_or_init_genesis(
        &self,
        preference: PreferenceId,
        subject: PrincipalId,
        document: PreferenceStateDocument,
    ) -> Result<PreferenceGenesisOutcome, PreferenceCasError> {
        if document.state().revision() != Revision::new(0) {
            return Err(PreferenceCasError::NonZeroGenesis {
                attempted: document.state().revision(),
            });
        }
        let mut guard = self.lock_state()?;
        let key = PreferenceKey::new(preference, subject);
        match guard.documents.get(&key) {
            Some(existing) if existing == &document => {
                Ok(PreferenceGenesisOutcome::AlreadyInitialized)
            }
            Some(existing) => Err(PreferenceCasError::GenesisConflict {
                existing: Box::new(existing.clone()),
                attempted: Box::new(document),
            }),
            None => {
                guard.documents.insert(key, document);
                Ok(PreferenceGenesisOutcome::Initialized)
            }
        }
    }

    /// Reads the exact V2 document for one subject/preference key.
    pub fn read_document(
        &self,
        preference: &PreferenceId,
        subject: &PrincipalId,
    ) -> Result<Option<PreferenceStateDocument>, PreferenceCasError> {
        let guard = self.lock_state()?;
        let key = PreferenceKey::new(preference.clone(), subject.clone());
        Ok(guard.documents.get(&key).cloned())
    }

    /// Atomically commits a caller-supplied document under exact evidence.
    ///
    /// The key and receipt are copied from `transition.evidence()`. Exact
    /// replay is idempotent. This method does not authenticate that evidence or
    /// validate the canonical payload hash, and therefore remains non-live.
    pub fn commit_evidenced(
        &self,
        transition: PreferenceTransition,
        document: PreferenceStateDocument,
    ) -> Result<PreferenceDocumentCommitOutcome, PreferenceCasError> {
        let mut guard = self.lock_state()?;
        if let Some(existing) = guard.transitions.get(transition.id()) {
            if existing.transition == transition && existing.document == document {
                return Ok(PreferenceDocumentCommitOutcome::AlreadyCommitted {
                    document: existing.document.clone(),
                });
            }
            return Err(transition_reuse_error(existing, &transition));
        }
        if document.state() != transition.committed_next() {
            return Err(PreferenceCasError::CommittedDocumentStateMismatch {
                expected: transition.committed_next().clone(),
                attempted: document.state().clone(),
            });
        }
        if let Some(existing) = guard.evidences.get(transition.evidence().id()) {
            return Err(evidence_reuse_error(existing, &transition));
        }
        if let Some(existing) = guard.receipts.get(transition.caused_by().id()) {
            return Err(PreferenceCasError::ReceiptReuseConflict {
                receipt: transition.caused_by().id().clone(),
                existing_receipt: Box::new(existing.receipt.clone()),
                attempted_receipt: Box::new(transition.caused_by().clone()),
                existing_transition: existing.transition.clone(),
                attempted_transition: transition.id().clone(),
            });
        }

        let key = PreferenceKey::from_transition(&transition);
        let Some(current) = guard.documents.get(&key) else {
            return Err(PreferenceCasError::PreferenceDocumentNotInitialized {
                preference: key.preference,
                subject: key.subject,
            });
        };
        if current.state() != transition.cas_expected_previous() {
            return Err(PreferenceCasError::StateConflict {
                preference: key.preference,
                subject: key.subject,
                expected: transition.cas_expected_previous().clone(),
                actual: current.state().clone(),
            });
        }
        if current.reducer_version() != document.reducer_version() {
            return Err(PreferenceCasError::ReducerVersionConflict {
                existing: current.reducer_version().to_owned(),
                attempted: document.reducer_version().to_owned(),
            });
        }

        let transition_id = transition.id().clone();
        guard.documents.insert(key, document.clone());
        guard.evidences.insert(
            transition.evidence().id().clone(),
            EvidenceUse {
                evidence: transition.evidence().clone(),
                transition: transition_id.clone(),
            },
        );
        guard.receipts.insert(
            transition.caused_by().id().clone(),
            ReceiptUse {
                receipt: transition.caused_by().clone(),
                transition: transition_id.clone(),
            },
        );
        guard.transitions.insert(
            transition_id,
            EvidencedTransitionUse {
                transition,
                document: document.clone(),
            },
        );
        Ok(PreferenceDocumentCommitOutcome::Committed { document })
    }

    fn lock_state(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, PreferenceStoreState>, PreferenceCasError> {
        self.state
            .lock()
            .map_err(|_| PreferenceCasError::StorePoisoned)
    }
}

fn transition_reuse_error(
    existing: &EvidencedTransitionUse,
    attempted: &PreferenceTransition,
) -> PreferenceCasError {
    if existing.transition.evidence().id() == attempted.evidence().id()
        && existing.transition.evidence() != attempted.evidence()
    {
        return evidence_reuse_error(
            &EvidenceUse {
                evidence: existing.transition.evidence().clone(),
                transition: existing.transition.id().clone(),
            },
            attempted,
        );
    }
    PreferenceCasError::TransitionReuseConflict {
        transition: attempted.id().clone(),
    }
}

fn evidence_reuse_error(
    existing: &EvidenceUse,
    attempted: &PreferenceTransition,
) -> PreferenceCasError {
    PreferenceCasError::EvidenceReuseConflict {
        evidence: attempted.evidence().id().clone(),
        existing_evidence: Box::new(existing.evidence.clone()),
        attempted_evidence: Box::new(attempted.evidence().clone()),
        existing_transition: existing.transition.clone(),
        attempted_transition: attempted.id().clone(),
    }
}

fn map_durable_error(error: crate::durable::DurableStorageError) -> PreferenceCasError {
    match error {
        crate::durable::DurableStorageError::Persistence { operation, detail } => {
            PreferenceCasError::Persistence { operation, detail }
        }
        crate::durable::DurableStorageError::Corrupt { detail } => {
            PreferenceCasError::Corrupt { detail }
        }
    }
}
