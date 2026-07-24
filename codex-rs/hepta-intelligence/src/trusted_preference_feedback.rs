//! Trusted explicit-preference source and reducer adapter.
//!
//! No source implementation is provided here. In particular, Telegram, HTTP,
//! gateway, and runtime input remain unattached. A future composition root must
//! supply a source that authenticates the complete memory-owned challenge.

use std::path::Path;

use hepta_contracts::ContentHash;
use hepta_contracts::PreferenceEvidenceId;
use hepta_contracts::PreferenceId;
use hepta_contracts::PreferenceState;
use hepta_contracts::PreferenceTransitionId;
use hepta_contracts::PrincipalId;
use hepta_contracts::ReceiptRef;
use hepta_memory::AuthenticatedPreferenceFeedback;
use hepta_memory::DurableIntegrityKey;
use hepta_memory::DurablePreferenceStore;
use hepta_memory::InMemoryPreferenceStore;
use hepta_memory::PreferenceAuthorityCommitOutcome;
use hepta_memory::PreferenceAuthorityError;
use hepta_memory::PreferenceDomainReducer;
use hepta_memory::PreferenceDomainReducerError;
use hepta_memory::PreferenceFeedbackAuthenticationError;
use hepta_memory::PreferenceFeedbackAuthenticator;
use hepta_memory::PreferenceFeedbackChallenge;
use hepta_memory::PreferenceFeedbackRequest;
use hepta_memory::PreferenceFeedbackRequestParts;
use hepta_memory::PreferenceFeedbackSourceRef;
use hepta_memory::PreferenceGenesisOutcome;
use hepta_memory::PreferenceReducerRef;
use hepta_memory::PreferenceReductionDraft;
use hepta_memory::PreferenceStateDocument;

use crate::EXPLICIT_PREFERENCE_REDUCER_VERSION;
use crate::ExplicitPreferenceSignal;
use crate::ExplicitPreferenceTarget;
use crate::PreferenceReductionError;
use crate::reduce_explicit_preference;

/// Stable identity of the deterministic explicit-preference reducer.
pub const EXPLICIT_PREFERENCE_REDUCER_ID: &str = "hepta.intelligence.explicit-preference.reducer";

/// Keyed, non-live composition root for trusted durable preference feedback.
///
/// This owns one caller-supplied trusted source and pins its exact source
/// binding for the authority lifetime. It provides no Telegram, HTTP, gateway,
/// runtime, or allow-all source implementation.
pub struct DurableTrustedPreferenceFeedbackAuthority<S> {
    store: DurablePreferenceStore,
    source: S,
    source_binding: PreferenceFeedbackSourceRef,
}

impl<S> DurableTrustedPreferenceFeedbackAuthority<S>
where
    S: TrustedPreferenceFeedbackSource,
{
    /// Exclusively bootstraps keyed durable storage and pins the source.
    pub async fn bootstrap_new(
        path: impl AsRef<Path>,
        integrity_key: DurableIntegrityKey,
        source: S,
    ) -> Result<Self, PreferenceAuthorityError> {
        let source_binding = source.source();
        let store = DurablePreferenceStore::bootstrap_new_keyed(path, integrity_key).await?;
        Ok(Self {
            store,
            source,
            source_binding,
        })
    }

    /// Opens keyed durable storage and pins the source without live attachment.
    pub async fn open_existing(
        path: impl AsRef<Path>,
        integrity_key: DurableIntegrityKey,
        source: S,
    ) -> Result<Self, PreferenceAuthorityError> {
        let source_binding = source.source();
        let store = DurablePreferenceStore::open_existing_keyed(path, integrity_key).await?;
        Ok(Self {
            store,
            source,
            source_binding,
        })
    }

    /// Returns the exact source identity pinned at composition.
    pub fn source_binding(&self) -> &PreferenceFeedbackSourceRef {
        &self.source_binding
    }

    /// Initializes one exact revision-zero document without exposing CAS writes.
    pub async fn get_or_init_genesis(
        &self,
        preference: PreferenceId,
        subject: PrincipalId,
        document: PreferenceStateDocument,
    ) -> Result<PreferenceGenesisOutcome, PreferenceAuthorityError> {
        self.ensure_source_binding()?;
        self.store
            .get_or_init_genesis(preference, subject, document)
            .await
            .map_err(Into::into)
    }

    /// Reads one exact durable document for audit and reconciliation.
    pub async fn read_document(
        &self,
        preference: &PreferenceId,
        subject: &PrincipalId,
    ) -> Result<Option<PreferenceStateDocument>, PreferenceAuthorityError> {
        self.ensure_source_binding()?;
        self.store
            .read_document(preference, subject)
            .await
            .map_err(Into::into)
    }

    /// Authenticates and attempts one exact durable CAS through the pinned source.
    pub async fn advance(
        &self,
        input: ExplicitPreferenceFeedbackInput,
    ) -> Result<PreferenceAuthorityCommitOutcome, PreferenceAuthorityError> {
        self.ensure_source_binding()?;
        let reducer = TrustedExplicitPreferenceReducer::try_new()?;
        let adapter = TrustedSourceAdapter {
            source: &self.source,
            target: &input.target,
            pinned_source: Some(&self.source_binding),
        };
        self.store
            .advance_preference_with_authority(input.request, &adapter, &reducer)
            .await
    }

    fn ensure_source_binding(&self) -> Result<(), PreferenceAuthorityError> {
        let actual = self.source.source();
        if actual == self.source_binding {
            return Ok(());
        }
        Err(PreferenceAuthorityError::SourceBindingChanged {
            expected: self.source_binding.clone(),
            actual,
        })
    }
}

/// Caller-untrusted explicit feedback plus its exact semantic target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplicitPreferenceFeedbackInput {
    request: PreferenceFeedbackRequest,
    target: ExplicitPreferenceTarget,
}

/// Named inputs for caller-untrusted explicit preference feedback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplicitPreferenceFeedbackInputParts {
    /// Identity reserved for the resulting transition.
    pub transition_id: PreferenceTransitionId,
    /// Identity reserved for immutable feedback evidence.
    pub evidence_id: PreferenceEvidenceId,
    /// Explicit accepted or rejected signal.
    pub signal: ExplicitPreferenceSignal,
    /// Exact execution receipt addressed by the feedback.
    pub receipt: ReceiptRef,
    /// Digest binding the authenticated feedback session.
    pub session_binding_hash: ContentHash,
    /// Claimed subject that the source must authenticate.
    pub subject: PrincipalId,
    /// Exact preference identity.
    pub preference: PreferenceId,
    /// Exact closed target.
    pub target: ExplicitPreferenceTarget,
    /// Exact state required before the feedback may advance.
    pub expected_previous: PreferenceState,
}

impl ExplicitPreferenceFeedbackInput {
    /// Creates untrusted input while deriving its exact target binding.
    pub fn try_new(
        parts: ExplicitPreferenceFeedbackInputParts,
    ) -> Result<Self, PreferenceAuthorityError> {
        let target_binding_hash = parts.target.binding_hash();
        let request = PreferenceFeedbackRequest::try_new(PreferenceFeedbackRequestParts {
            transition_id: parts.transition_id,
            evidence_id: parts.evidence_id,
            signal: parts.signal,
            receipt: parts.receipt,
            session_binding_hash: parts.session_binding_hash,
            subject: parts.subject,
            preference: parts.preference,
            target_binding_hash,
            expected_previous: parts.expected_previous,
        })?;
        Ok(Self {
            request,
            target: parts.target,
        })
    }

    /// Returns the caller-untrusted authority request.
    pub fn request(&self) -> &PreferenceFeedbackRequest {
        &self.request
    }

    /// Returns the exact semantic target whose digest is in the request.
    pub fn target(&self) -> &ExplicitPreferenceTarget {
        &self.target
    }
}

/// Intelligence-layer view of the exact challenge a source must authenticate.
pub struct TrustedPreferenceFeedbackChallenge<'a> {
    authority: &'a PreferenceFeedbackChallenge,
    target: &'a ExplicitPreferenceTarget,
}

impl<'a> TrustedPreferenceFeedbackChallenge<'a> {
    fn new(
        authority: &'a PreferenceFeedbackChallenge,
        target: &'a ExplicitPreferenceTarget,
    ) -> Self {
        Self { authority, target }
    }

    /// Returns the memory-owned challenge containing every CAS binding.
    pub fn authority(&self) -> &PreferenceFeedbackChallenge {
        self.authority
    }

    /// Returns the exact semantic target matching the challenge digest.
    pub fn target(&self) -> &ExplicitPreferenceTarget {
        self.target
    }
}

/// Trusted source capability for explicit human preference feedback.
///
/// Implementations must authenticate the claimed subject and immutable
/// feedback provenance, then validate the source, target, evidence, receipt,
/// session, exact prior state, transition, and reducer bindings. There is no
/// default or allow-all implementation.
pub trait TrustedPreferenceFeedbackSource {
    /// Returns the exact source identity to bind before authentication.
    fn source(&self) -> PreferenceFeedbackSourceRef;

    /// Authenticates the complete challenge or denies it.
    fn authenticate(
        &self,
        challenge: &TrustedPreferenceFeedbackChallenge<'_>,
    ) -> Result<(), PreferenceFeedbackAuthenticationError>;
}

/// Exact deterministic reducer used by trusted explicit feedback authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustedExplicitPreferenceReducer {
    binding: PreferenceReducerRef,
}

impl TrustedExplicitPreferenceReducer {
    /// Creates the fixed reducer identity and version binding.
    pub fn try_new() -> Result<Self, PreferenceAuthorityError> {
        Ok(Self {
            binding: PreferenceReducerRef::try_new(
                EXPLICIT_PREFERENCE_REDUCER_ID,
                EXPLICIT_PREFERENCE_REDUCER_VERSION,
            )?,
        })
    }

    /// Returns the exact reducer binding.
    pub fn binding(&self) -> &PreferenceReducerRef {
        &self.binding
    }
}

impl PreferenceDomainReducer for TrustedExplicitPreferenceReducer {
    fn reducer(&self) -> PreferenceReducerRef {
        self.binding.clone()
    }

    fn reduce(
        &self,
        current: &PreferenceStateDocument,
        feedback: &AuthenticatedPreferenceFeedback,
    ) -> Result<PreferenceReductionDraft, PreferenceDomainReducerError> {
        let reduction = reduce_explicit_preference(
            current.state(),
            current.canonical_payload(),
            feedback.evidence(),
        )
        .map_err(|error| PreferenceDomainReducerError::new(reduction_error_code(&error)))?;
        Ok(PreferenceReductionDraft::new(
            reduction.next_state().clone(),
            reduction.canonical_payload(),
        ))
    }
}

/// Authenticates and attempts one exact in-memory explicit-preference CAS.
pub fn advance_trusted_explicit_preference<S>(
    store: &InMemoryPreferenceStore,
    source: &S,
    input: ExplicitPreferenceFeedbackInput,
) -> Result<PreferenceAuthorityCommitOutcome, PreferenceAuthorityError>
where
    S: TrustedPreferenceFeedbackSource + ?Sized,
{
    let reducer = TrustedExplicitPreferenceReducer::try_new()?;
    let adapter = TrustedSourceAdapter {
        source,
        target: &input.target,
        pinned_source: None,
    };
    store.advance_preference_with_authority(input.request, &adapter, &reducer)
}

/// Authenticates and attempts one exact durable explicit-preference CAS.
pub async fn advance_trusted_explicit_preference_durable<S>(
    store: &DurablePreferenceStore,
    source: &S,
    input: ExplicitPreferenceFeedbackInput,
) -> Result<PreferenceAuthorityCommitOutcome, PreferenceAuthorityError>
where
    S: TrustedPreferenceFeedbackSource + ?Sized,
{
    let reducer = TrustedExplicitPreferenceReducer::try_new()?;
    let adapter = TrustedSourceAdapter {
        source,
        target: &input.target,
        pinned_source: None,
    };
    store
        .advance_preference_with_authority(input.request, &adapter, &reducer)
        .await
}

struct TrustedSourceAdapter<'a, S>
where
    S: TrustedPreferenceFeedbackSource + ?Sized,
{
    source: &'a S,
    target: &'a ExplicitPreferenceTarget,
    pinned_source: Option<&'a PreferenceFeedbackSourceRef>,
}

impl<S> TrustedSourceAdapter<'_, S>
where
    S: TrustedPreferenceFeedbackSource + ?Sized,
{
    fn ensure_pinned_source(&self) -> Result<(), PreferenceFeedbackAuthenticationError> {
        let Some(expected) = self.pinned_source else {
            return Ok(());
        };
        if self.source.source() == *expected {
            return Ok(());
        }
        Err(PreferenceFeedbackAuthenticationError::new(
            "trusted_preference_feedback.source_binding_changed",
        ))
    }
}

impl<S> PreferenceFeedbackAuthenticator for TrustedSourceAdapter<'_, S>
where
    S: TrustedPreferenceFeedbackSource + ?Sized,
{
    fn source(&self) -> PreferenceFeedbackSourceRef {
        self.pinned_source
            .cloned()
            .unwrap_or_else(|| self.source.source())
    }

    fn authenticate(
        &self,
        challenge: &PreferenceFeedbackChallenge,
    ) -> Result<(), PreferenceFeedbackAuthenticationError> {
        self.ensure_pinned_source()?;
        if challenge.request().target_binding_hash() != &self.target.binding_hash() {
            return Err(PreferenceFeedbackAuthenticationError::new(
                "trusted_preference_feedback.target_binding_mismatch",
            ));
        }
        self.source
            .authenticate(&TrustedPreferenceFeedbackChallenge::new(
                challenge,
                self.target,
            ))?;
        self.ensure_pinned_source()
    }
}

fn reduction_error_code(error: &PreferenceReductionError) -> &'static str {
    match error {
        PreferenceReductionError::MalformedPreviousPayload(_) => {
            "explicit_preference.malformed_previous_payload"
        }
        PreferenceReductionError::UnsupportedVersion => "explicit_preference.unsupported_version",
        PreferenceReductionError::NonCanonicalPreviousPayload => {
            "explicit_preference.noncanonical_previous_payload"
        }
        PreferenceReductionError::PreviousRevisionMismatch { .. } => {
            "explicit_preference.previous_revision_mismatch"
        }
        PreferenceReductionError::PreviousHashMismatch { .. } => {
            "explicit_preference.previous_hash_mismatch"
        }
        PreferenceReductionError::PayloadTargetBindingMismatch { .. } => {
            "explicit_preference.payload_target_binding_mismatch"
        }
        PreferenceReductionError::SubjectBindingMismatch => {
            "explicit_preference.subject_binding_mismatch"
        }
        PreferenceReductionError::PreferenceBindingMismatch => {
            "explicit_preference.preference_binding_mismatch"
        }
        PreferenceReductionError::TargetBindingMismatch => {
            "explicit_preference.target_binding_mismatch"
        }
        PreferenceReductionError::RevisionOverflow => "explicit_preference.revision_overflow",
        PreferenceReductionError::CounterOverflow(ExplicitPreferenceSignal::Accepted) => {
            "explicit_preference.accepted_counter_overflow"
        }
        PreferenceReductionError::CounterOverflow(ExplicitPreferenceSignal::Rejected) => {
            "explicit_preference.rejected_counter_overflow"
        }
    }
}

#[cfg(test)]
mod tests;
