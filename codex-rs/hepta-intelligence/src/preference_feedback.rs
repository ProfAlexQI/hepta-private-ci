//! Deterministic, non-live reduction of authority-bound explicit evidence.
//!
//! This module does not authenticate feedback, inspect execution status, write
//! memory, or change live routing. Its only signal is an explicit acceptance
//! or rejection asserted by a caller-supplied `PreferenceEvidenceRef`.

use std::error::Error;
use std::fmt;

use hepta_contracts::CapabilityManifestRef;
use hepta_contracts::ContentHash;
use hepta_contracts::PreferenceEvidenceRef;
use hepta_contracts::PreferenceId;
use hepta_contracts::PreferenceState;
use hepta_contracts::PrincipalId;
use hepta_contracts::Revision;

/// Public intelligence-layer name for the signal bound into preference evidence.
pub use hepta_contracts::PreferenceEvidenceSignal as ExplicitPreferenceSignal;

/// Version of the explicit-preference reduction semantics and payload.
pub const EXPLICIT_PREFERENCE_REDUCER_VERSION: &str =
    "hepta.intelligence.explicit-preference.reducer.v1";

mod canonical;

/// Closed V1 preference target.
///
/// V1 accepts only an exact capability manifest in an exact catalog snapshot.
/// The reference remains caller-untrusted until an external authority validates
/// its provenance; this reducer only binds and hashes it deterministically.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExplicitPreferenceTarget {
    /// One exact capability manifest and catalog snapshot.
    Capability(CapabilityManifestRef),
}

impl ExplicitPreferenceTarget {
    /// Returns the exact capability manifest addressed by this target.
    pub fn capability(&self) -> &CapabilityManifestRef {
        match self {
            Self::Capability(capability) => capability,
        }
    }

    /// Returns the reducer-owned exact target binding.
    pub fn binding_hash(&self) -> ContentHash {
        canonical::target_binding_hash(self)
    }
}

/// Versioned integer accumulator for one subject/preference/target tuple.
///
/// All fields are reducer-owned. There is no caller-provided weight, latency,
/// floating-point score, or safety score.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreferenceAccumulator {
    subject: PrincipalId,
    preference: PreferenceId,
    target: ExplicitPreferenceTarget,
    target_binding_hash: ContentHash,
    revision: Revision,
    accepted_count: u64,
    rejected_count: u64,
    canonical_payload: String,
    state: PreferenceState,
}

impl PreferenceAccumulator {
    /// Returns the preference subject bound into this state.
    pub fn subject(&self) -> &PrincipalId {
        &self.subject
    }
    /// Returns the exact preference identity bound into this state.
    pub fn preference(&self) -> &PreferenceId {
        &self.preference
    }
    /// Returns the closed exact target bound into this state.
    pub fn target(&self) -> &ExplicitPreferenceTarget {
        &self.target
    }
    /// Returns the reducer-owned target binding digest.
    pub fn target_binding_hash(&self) -> &ContentHash {
        &self.target_binding_hash
    }
    /// Returns the exact accumulator revision.
    pub const fn revision(&self) -> Revision {
        self.revision
    }
    /// Returns the explicit acceptance count.
    pub const fn accepted_count(&self) -> u64 {
        self.accepted_count
    }
    /// Returns the explicit rejection count.
    pub const fn rejected_count(&self) -> u64 {
        self.rejected_count
    }
    /// Returns the exact state derived from the canonical payload.
    pub fn state(&self) -> &PreferenceState {
        &self.state
    }
    /// Returns the canonical, versioned accumulator payload.
    pub fn canonical_payload(&self) -> &str {
        &self.canonical_payload
    }
    /// Returns the reducer version that owns this payload.
    pub const fn reducer_version(&self) -> &'static str {
        EXPLICIT_PREFERENCE_REDUCER_VERSION
    }
}

/// Auditable result of applying one explicit evidence signal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreferenceReduction {
    previous: PreferenceState,
    evidence: PreferenceEvidenceRef,
    signal: ExplicitPreferenceSignal,
    next: PreferenceAccumulator,
}

impl PreferenceReduction {
    /// Returns the exact previous state verified by the reducer.
    pub fn previous_state(&self) -> &PreferenceState {
        &self.previous
    }
    /// Returns the exact evidence reference whose bindings were checked.
    pub fn evidence(&self) -> &PreferenceEvidenceRef {
        &self.evidence
    }
    /// Returns the closed explicit signal that was reduced.
    pub const fn signal(&self) -> ExplicitPreferenceSignal {
        self.signal
    }
    /// Returns the next versioned accumulator.
    pub fn next(&self) -> &PreferenceAccumulator {
        &self.next
    }
    /// Returns the exact next preference state.
    pub fn next_state(&self) -> &PreferenceState {
        self.next.state()
    }
    /// Returns the canonical next-state payload.
    pub fn canonical_payload(&self) -> &str {
        self.next.canonical_payload()
    }
    /// Returns the reducer version that owns the next payload.
    pub const fn reducer_version(&self) -> &'static str {
        EXPLICIT_PREFERENCE_REDUCER_VERSION
    }
}

/// Typed failure from explicit preference reduction.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PreferenceReductionError {
    /// The previous canonical payload is malformed.
    MalformedPreviousPayload(&'static str),
    /// The previous payload uses a different reducer or schema version.
    UnsupportedVersion,
    /// The previous payload parses but is not in the one canonical form.
    NonCanonicalPreviousPayload,
    /// The exact state revision differs from the payload revision.
    PreviousRevisionMismatch { state: Revision, payload: Revision },
    /// The exact state content hash differs from the canonical payload hash.
    PreviousHashMismatch {
        state: ContentHash,
        computed: ContentHash,
    },
    /// The payload's declared target digest differs from its exact capability.
    PayloadTargetBindingMismatch {
        declared: ContentHash,
        computed: ContentHash,
    },
    /// Evidence names a different subject.
    SubjectBindingMismatch,
    /// Evidence names a different preference identity.
    PreferenceBindingMismatch,
    /// Evidence names a different exact target.
    TargetBindingMismatch,
    /// The next accumulator revision cannot be represented.
    RevisionOverflow,
    /// The selected explicit signal counter cannot be incremented.
    CounterOverflow(ExplicitPreferenceSignal),
}

impl fmt::Display for PreferenceReductionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for PreferenceReductionError {}

/// Creates deterministic revision-zero state for one exact preference tuple.
pub fn explicit_preference_genesis(
    subject: PrincipalId,
    preference: PreferenceId,
    target: ExplicitPreferenceTarget,
) -> PreferenceAccumulator {
    build_accumulator(AccumulatorData {
        subject,
        preference,
        target,
        revision: Revision::new(0),
        accepted_count: 0,
        rejected_count: 0,
    })
}

/// Applies one authority-bound explicit signal to an exact previous state.
///
/// The evidence reference remains caller-untrusted. This function verifies its
/// subject, preference, target, and closed signal bindings, but does not
/// authenticate its receipt, session, evidence hash, or human origin.
pub fn reduce_explicit_preference(
    previous: &PreferenceState,
    previous_canonical_payload: &str,
    evidence: &PreferenceEvidenceRef,
) -> Result<PreferenceReduction, PreferenceReductionError> {
    let current = parse_accumulator(previous_canonical_payload)?;
    if previous.revision() != current.revision {
        return Err(PreferenceReductionError::PreviousRevisionMismatch {
            state: previous.revision(),
            payload: current.revision,
        });
    }
    if previous.content_hash() != current.state.content_hash() {
        return Err(PreferenceReductionError::PreviousHashMismatch {
            state: previous.content_hash().clone(),
            computed: current.state.content_hash().clone(),
        });
    }
    if evidence.subject() != &current.subject {
        return Err(PreferenceReductionError::SubjectBindingMismatch);
    }
    if evidence.preference() != &current.preference {
        return Err(PreferenceReductionError::PreferenceBindingMismatch);
    }
    if evidence.target_binding_hash() != &current.target_binding_hash {
        return Err(PreferenceReductionError::TargetBindingMismatch);
    }

    let signal = evidence.signal();
    let revision = current
        .revision
        .get()
        .checked_add(1)
        .map(Revision::new)
        .ok_or(PreferenceReductionError::RevisionOverflow)?;
    let (accepted_count, rejected_count) = match signal {
        ExplicitPreferenceSignal::Accepted => (
            current
                .accepted_count
                .checked_add(1)
                .ok_or(PreferenceReductionError::CounterOverflow(signal))?,
            current.rejected_count,
        ),
        ExplicitPreferenceSignal::Rejected => (
            current.accepted_count,
            current
                .rejected_count
                .checked_add(1)
                .ok_or(PreferenceReductionError::CounterOverflow(signal))?,
        ),
    };
    let next = build_accumulator(AccumulatorData {
        subject: current.subject,
        preference: current.preference,
        target: current.target,
        revision,
        accepted_count,
        rejected_count,
    });
    Ok(PreferenceReduction {
        previous: previous.clone(),
        evidence: evidence.clone(),
        signal,
        next,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AccumulatorData {
    subject: PrincipalId,
    preference: PreferenceId,
    target: ExplicitPreferenceTarget,
    revision: Revision,
    accepted_count: u64,
    rejected_count: u64,
}

fn build_accumulator(data: AccumulatorData) -> PreferenceAccumulator {
    let target_binding_hash = canonical::target_binding_hash(&data.target);
    let canonical_payload = canonical::canonical_payload(&data, &target_binding_hash);
    let state = PreferenceState::new(data.revision, canonical::state_hash(&canonical_payload));
    PreferenceAccumulator {
        subject: data.subject,
        preference: data.preference,
        target: data.target,
        target_binding_hash,
        revision: data.revision,
        accepted_count: data.accepted_count,
        rejected_count: data.rejected_count,
        canonical_payload,
        state,
    }
}

fn parse_accumulator(payload: &str) -> Result<PreferenceAccumulator, PreferenceReductionError> {
    Ok(build_accumulator(canonical::parse_data(payload)?))
}

#[cfg(test)]
mod tests;
