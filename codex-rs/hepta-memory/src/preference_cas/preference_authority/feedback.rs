use std::error::Error;
use std::fmt;

use hepta_contracts::ContentHash;
use hepta_contracts::PreferenceEvidenceId;
use hepta_contracts::PreferenceEvidenceRef;
use hepta_contracts::PreferenceEvidenceSignal;
use hepta_contracts::PreferenceId;
use hepta_contracts::PreferenceState;
use hepta_contracts::PreferenceTransitionId;
use hepta_contracts::PrincipalId;
use hepta_contracts::ReceiptRef;

use super::super::canonical::authority_evidence_hash;
use super::AuthenticatedPreferenceFeedback;
use super::PreferenceFeedbackChallenge;
use super::PreferenceFeedbackRequest;
use super::bindings::PreferenceFeedbackSourceRef;
use super::bindings::PreferenceReducerRef;
use super::error::PreferenceAuthorityError;
use super::error::require_nonempty;

/// Named inputs for one caller-untrusted preference feedback request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreferenceFeedbackRequestParts {
    /// Identity reserved for the resulting transition.
    pub transition_id: PreferenceTransitionId,
    /// Identity reserved for the immutable feedback evidence.
    pub evidence_id: PreferenceEvidenceId,
    /// Explicit accepted or rejected signal.
    pub signal: PreferenceEvidenceSignal,
    /// Exact execution receipt addressed by the feedback.
    pub receipt: ReceiptRef,
    /// Digest binding the authenticated feedback session.
    pub session_binding_hash: ContentHash,
    /// Subject whose preference may advance.
    pub subject: PrincipalId,
    /// Preference identity that may advance.
    pub preference: PreferenceId,
    /// Digest binding the exact target.
    pub target_binding_hash: ContentHash,
    /// Exact state required before authentication and reduction.
    pub expected_previous: PreferenceState,
}

impl PreferenceFeedbackRequest {
    /// Validates non-empty opaque bindings and creates an untrusted request.
    pub fn try_new(
        parts: PreferenceFeedbackRequestParts,
    ) -> Result<Self, PreferenceAuthorityError> {
        require_nonempty("transition.id", parts.transition_id.as_str())?;
        require_nonempty("evidence.id", parts.evidence_id.as_str())?;
        require_nonempty("receipt.id", parts.receipt.id().as_str())?;
        require_nonempty("receipt.hash", parts.receipt.receipt_hash().as_str())?;
        require_nonempty("session_binding_hash", parts.session_binding_hash.as_str())?;
        require_nonempty("subject", parts.subject.as_str())?;
        require_nonempty("preference", parts.preference.as_str())?;
        require_nonempty("target_binding_hash", parts.target_binding_hash.as_str())?;
        require_nonempty(
            "expected_previous.content_hash",
            parts.expected_previous.content_hash().as_str(),
        )?;
        Ok(Self {
            transition_id: parts.transition_id,
            evidence_id: parts.evidence_id,
            signal: parts.signal,
            receipt: parts.receipt,
            session_binding_hash: parts.session_binding_hash,
            subject: parts.subject,
            preference: parts.preference,
            target_binding_hash: parts.target_binding_hash,
            expected_previous: parts.expected_previous,
        })
    }

    /// Returns the reserved transition identity.
    pub fn transition_id(&self) -> &PreferenceTransitionId {
        &self.transition_id
    }

    /// Returns the reserved evidence identity.
    pub fn evidence_id(&self) -> &PreferenceEvidenceId {
        &self.evidence_id
    }

    /// Returns the explicit feedback signal.
    pub const fn signal(&self) -> PreferenceEvidenceSignal {
        self.signal
    }

    /// Returns the exact referenced outcome receipt.
    pub fn receipt(&self) -> &ReceiptRef {
        &self.receipt
    }

    /// Returns the authenticated-session binding.
    pub fn session_binding_hash(&self) -> &ContentHash {
        &self.session_binding_hash
    }

    /// Returns the expected authenticated subject.
    pub fn subject(&self) -> &PrincipalId {
        &self.subject
    }

    /// Returns the exact preference identity.
    pub fn preference(&self) -> &PreferenceId {
        &self.preference
    }

    /// Returns the exact target binding.
    pub fn target_binding_hash(&self) -> &ContentHash {
        &self.target_binding_hash
    }

    /// Returns the exact state required before this request may advance.
    pub fn expected_previous(&self) -> &PreferenceState {
        &self.expected_previous
    }
}

impl PreferenceFeedbackChallenge {
    pub(in crate::preference_authority) fn new(
        request: PreferenceFeedbackRequest,
        source: PreferenceFeedbackSourceRef,
        reducer: PreferenceReducerRef,
    ) -> Self {
        let evidence_hash = authority_evidence_hash(&request, &source, &reducer);
        Self {
            request,
            source,
            reducer,
            evidence_hash,
        }
    }

    /// Returns the complete untrusted request being authenticated.
    pub fn request(&self) -> &PreferenceFeedbackRequest {
        &self.request
    }

    /// Returns the exact source identity included in the evidence digest.
    pub fn source(&self) -> &PreferenceFeedbackSourceRef {
        &self.source
    }

    /// Returns the exact reducer identity included in the evidence digest.
    pub fn reducer(&self) -> &PreferenceReducerRef {
        &self.reducer
    }

    /// Returns the canonical digest that will become the evidence hash.
    pub fn evidence_hash(&self) -> &ContentHash {
        &self.evidence_hash
    }

    pub(in crate::preference_authority) fn into_authenticated(
        self,
    ) -> AuthenticatedPreferenceFeedback {
        let evidence = PreferenceEvidenceRef::new(
            self.request.evidence_id.clone(),
            self.evidence_hash.clone(),
            self.request.signal,
            self.request.receipt.clone(),
            self.request.session_binding_hash.clone(),
            self.request.subject.clone(),
            self.request.preference.clone(),
            self.request.target_binding_hash.clone(),
        );
        AuthenticatedPreferenceFeedback {
            challenge: self,
            evidence,
        }
    }
}

impl AuthenticatedPreferenceFeedback {
    /// Returns the canonical evidence reference minted by memory authority.
    pub fn evidence(&self) -> &PreferenceEvidenceRef {
        &self.evidence
    }

    /// Returns the exact expected prior state authenticated by the source.
    pub fn expected_previous(&self) -> &PreferenceState {
        self.challenge.request.expected_previous()
    }

    /// Returns the exact source binding authenticated for this feedback.
    pub fn source(&self) -> &PreferenceFeedbackSourceRef {
        self.challenge.source()
    }

    /// Returns the exact reducer binding authenticated for this feedback.
    pub fn reducer(&self) -> &PreferenceReducerRef {
        self.challenge.reducer()
    }

    /// Returns the complete authenticated request.
    pub fn request(&self) -> &PreferenceFeedbackRequest {
        self.challenge.request()
    }
}

/// Typed denial returned by a feedback authenticator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreferenceFeedbackAuthenticationError {
    code: String,
}

impl PreferenceFeedbackAuthenticationError {
    /// Creates a source-owned stable denial code.
    pub fn new(code: impl Into<String>) -> Self {
        Self { code: code.into() }
    }

    /// Returns the source-owned denial code.
    pub fn code(&self) -> &str {
        &self.code
    }
}

impl fmt::Display for PreferenceFeedbackAuthenticationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "preference feedback authentication denied: {}",
            self.code
        )
    }
}

impl Error for PreferenceFeedbackAuthenticationError {}
