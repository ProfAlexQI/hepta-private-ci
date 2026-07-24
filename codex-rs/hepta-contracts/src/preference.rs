use crate::ContentHash;
use crate::ContractError;
use crate::PreferenceEvidenceId;
use crate::PreferenceId;
use crate::PreferenceTransitionId;
use crate::PrincipalId;
use crate::ReceiptRef;
use crate::Revision;

/// Closed explicit signal authenticated by one preference evidence envelope.
///
/// Execution success, failure, latency, and safety scores are deliberately not
/// variants, so system outcomes cannot be represented as human preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreferenceEvidenceSignal {
    /// The authenticated subject accepted the exact preference target.
    Accepted,
    /// The authenticated subject rejected the exact preference target.
    Rejected,
}

/// Exact reference to immutable evidence for one preference update.
///
/// The evidence digest is owned by the layer that authenticates and
/// canonicalizes feedback. This contract binds that digest to the exact
/// explicit signal, execution receipt, session, subject, preference, and
/// target. The reference is data, not a bearer credential; a consuming
/// authority must still resolve and validate its provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreferenceEvidenceRef {
    id: PreferenceEvidenceId,
    evidence_hash: ContentHash,
    signal: PreferenceEvidenceSignal,
    receipt: ReceiptRef,
    session_binding_hash: ContentHash,
    subject: PrincipalId,
    preference: PreferenceId,
    target_binding_hash: ContentHash,
}

impl PreferenceEvidenceRef {
    /// Creates an exact reference to canonical preference evidence.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: PreferenceEvidenceId,
        evidence_hash: ContentHash,
        signal: PreferenceEvidenceSignal,
        receipt: ReceiptRef,
        session_binding_hash: ContentHash,
        subject: PrincipalId,
        preference: PreferenceId,
        target_binding_hash: ContentHash,
    ) -> Self {
        Self {
            id,
            evidence_hash,
            signal,
            receipt,
            session_binding_hash,
            subject,
            preference,
            target_binding_hash,
        }
    }

    /// Returns the immutable evidence identity.
    pub fn id(&self) -> &PreferenceEvidenceId {
        &self.id
    }

    /// Returns the digest of the canonical evidence envelope.
    pub fn evidence_hash(&self) -> &ContentHash {
        &self.evidence_hash
    }

    /// Returns the explicit signal bound into the evidence envelope.
    pub const fn signal(&self) -> PreferenceEvidenceSignal {
        self.signal
    }

    /// Returns the exact execution receipt bound into the evidence.
    pub fn receipt(&self) -> &ReceiptRef {
        &self.receipt
    }

    /// Returns the digest binding the authenticated feedback session.
    pub fn session_binding_hash(&self) -> &ContentHash {
        &self.session_binding_hash
    }

    /// Returns the authenticated principal whose preference is evidenced.
    pub fn subject(&self) -> &PrincipalId {
        &self.subject
    }

    /// Returns the exact preference addressed by the evidence.
    pub fn preference(&self) -> &PreferenceId {
        &self.preference
    }

    /// Returns the digest binding the exact preference target.
    pub fn target_binding_hash(&self) -> &ContentHash {
        &self.target_binding_hash
    }
}

/// Opaque, versioned state of one preference.
///
/// Preference semantics and storage remain outside this crate; the content
/// hash lets learning layers refer to an exact state without sharing a model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreferenceState {
    revision: Revision,
    content_hash: ContentHash,
}

impl PreferenceState {
    /// Creates a reference to an exact preference state.
    pub fn new(revision: Revision, content_hash: ContentHash) -> Self {
        Self {
            revision,
            content_hash,
        }
    }

    /// Returns the preference state revision.
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns the digest of the canonical preference state.
    pub fn content_hash(&self) -> &ContentHash {
        &self.content_hash
    }
}

/// Auditable preference change supported by an execution outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreferenceTransition {
    id: PreferenceTransitionId,
    evidence: PreferenceEvidenceRef,
    preference: PreferenceId,
    subject: PrincipalId,
    cas_expected_previous: PreferenceState,
    committed_next: PreferenceState,
    caused_by: ReceiptRef,
}

impl PreferenceTransition {
    /// Creates a preference transition committed with revision CAS.
    ///
    /// `cas_expected_previous` is the exact state that memory compared before
    /// committing `committed_next`. Subject, preference, and receipt bindings
    /// are copied from the complete evidence reference, so callers cannot bind
    /// them independently. The committed revision must be the exact successor
    /// of the CAS expectation.
    pub fn try_new(
        id: PreferenceTransitionId,
        evidence: &PreferenceEvidenceRef,
        cas_expected_previous: PreferenceState,
        committed_next: PreferenceState,
    ) -> Result<Self, ContractError> {
        let expected = cas_expected_previous.revision();
        let committed = committed_next.revision();
        let required = expected
            .get()
            .checked_add(1)
            .map(Revision::new)
            .ok_or(ContractError::PreferenceRevisionOverflow { expected })?;
        if committed != required {
            return Err(ContractError::PreferenceRevisionNotAdvanced {
                expected,
                committed,
            });
        }

        Ok(Self {
            id,
            evidence: evidence.clone(),
            preference: evidence.preference().clone(),
            subject: evidence.subject().clone(),
            cas_expected_previous,
            committed_next,
            caused_by: evidence.receipt().clone(),
        })
    }

    /// Returns the transition identity.
    pub fn id(&self) -> &PreferenceTransitionId {
        &self.id
    }

    /// Returns the exact immutable evidence supporting this transition.
    pub fn evidence(&self) -> &PreferenceEvidenceRef {
        &self.evidence
    }

    /// Returns the evolving preference identity.
    pub fn preference(&self) -> &PreferenceId {
        &self.preference
    }

    /// Returns the principal whose preference changed.
    pub fn subject(&self) -> &PrincipalId {
        &self.subject
    }

    /// Returns the exact previous state used as the CAS expectation.
    pub fn cas_expected_previous(&self) -> &PreferenceState {
        &self.cas_expected_previous
    }

    /// Returns the next state committed after the CAS expectation matched.
    pub fn committed_next(&self) -> &PreferenceState {
        &self.committed_next
    }

    /// Returns the exact immutable outcome receipt supporting the transition.
    pub fn caused_by(&self) -> &ReceiptRef {
        &self.caused_by
    }
}
