//! Typed preference authority challenges, capabilities, and outcomes.

#[path = "preference_authority/bindings.rs"]
mod bindings;
#[path = "preference_authority/error.rs"]
mod error;
#[path = "preference_authority/feedback.rs"]
mod feedback;
#[path = "preference_authority/outcome.rs"]
mod outcome;
#[path = "preference_authority/reduction.rs"]
mod reduction;

use hepta_contracts::ContentHash;
use hepta_contracts::PreferenceEvidenceId;
use hepta_contracts::PreferenceEvidenceRef;
use hepta_contracts::PreferenceEvidenceSignal;
use hepta_contracts::PreferenceId;
use hepta_contracts::PreferenceState;
use hepta_contracts::PreferenceTransitionId;
use hepta_contracts::PrincipalId;
use hepta_contracts::ReceiptRef;

use crate::preference_cas::PreferenceStateDocument;

pub use bindings::PreferenceFeedbackSourceRef;
pub use bindings::PreferenceReducerRef;
pub use error::PreferenceAuthorityError;
pub use feedback::PreferenceFeedbackAuthenticationError;
pub use feedback::PreferenceFeedbackRequestParts;
pub use outcome::PreferenceAuthorityCommitOutcome;
pub use reduction::PreferenceDomainReducerError;
pub use reduction::PreferenceReductionDraft;

/// Complete caller-untrusted feedback material before source authentication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreferenceFeedbackRequest {
    transition_id: PreferenceTransitionId,
    evidence_id: PreferenceEvidenceId,
    signal: PreferenceEvidenceSignal,
    receipt: ReceiptRef,
    session_binding_hash: ContentHash,
    subject: PrincipalId,
    preference: PreferenceId,
    target_binding_hash: ContentHash,
    expected_previous: PreferenceState,
}

/// Immutable authentication challenge over every preference authority binding.
#[derive(Debug, PartialEq, Eq)]
pub struct PreferenceFeedbackChallenge {
    request: PreferenceFeedbackRequest,
    source: PreferenceFeedbackSourceRef,
    reducer: PreferenceReducerRef,
    evidence_hash: ContentHash,
}

/// Non-cloneable feedback minted only after an authenticator accepts a challenge.
#[derive(Debug, PartialEq, Eq)]
pub struct AuthenticatedPreferenceFeedback {
    challenge: PreferenceFeedbackChallenge,
    evidence: PreferenceEvidenceRef,
}

/// Trusted capability that authenticates a complete preference challenge.
///
/// Implementations must validate human subject authentication and evidence
/// provenance against every challenge field. This crate provides no default,
/// allow-all, transport, or live-ingress implementation.
pub trait PreferenceFeedbackAuthenticator {
    /// Returns the exact source identity to bind before authentication.
    fn source(&self) -> PreferenceFeedbackSourceRef;

    /// Authenticates the complete immutable challenge or denies it.
    fn authenticate(
        &self,
        challenge: &PreferenceFeedbackChallenge,
    ) -> Result<(), PreferenceFeedbackAuthenticationError>;
}

/// Exact reducer capability used after source authentication.
///
/// Implementations receive only memory-minted authenticated feedback and must
/// deterministically derive one successor state under their declared version.
pub trait PreferenceDomainReducer {
    /// Returns the exact reducer identity to bind before authentication.
    fn reducer(&self) -> PreferenceReducerRef;

    /// Reduces authenticated feedback against the exact current document.
    fn reduce(
        &self,
        current: &PreferenceStateDocument,
        feedback: &AuthenticatedPreferenceFeedback,
    ) -> Result<PreferenceReductionDraft, PreferenceDomainReducerError>;
}
