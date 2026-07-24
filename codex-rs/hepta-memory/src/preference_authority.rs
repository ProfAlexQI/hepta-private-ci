//! Fail-closed preference authentication, reduction, and single-CAS authority.
//!
//! This module defines a domain seam only. It has no Telegram, HTTP, gateway,
//! runtime, or other live-ingress implementation. A caller must supply an
//! authenticator that validates the complete challenge and a reducer whose
//! exact identity and version are included in that challenge.

#[path = "preference_cas/preference_authority_canonical.rs"]
mod canonical;
#[path = "preference_cas/preference_authority_types.rs"]
mod types;

use hepta_contracts::PreferenceState;
use hepta_contracts::PreferenceTransition;

use crate::preference_cas::DurablePreferenceStore;
use crate::preference_cas::InMemoryPreferenceStore;
use crate::preference_cas::PreferenceCasError;
use crate::preference_cas::PreferenceDocumentCommitOutcome;
use crate::preference_cas::PreferenceStateDocument;

pub use types::AuthenticatedPreferenceFeedback;
pub use types::PreferenceAuthorityCommitOutcome;
pub use types::PreferenceAuthorityError;
pub use types::PreferenceDomainReducer;
pub use types::PreferenceDomainReducerError;
pub use types::PreferenceFeedbackAuthenticationError;
pub use types::PreferenceFeedbackAuthenticator;
pub use types::PreferenceFeedbackChallenge;
pub use types::PreferenceFeedbackRequest;
pub use types::PreferenceFeedbackRequestParts;
pub use types::PreferenceFeedbackSourceRef;
pub use types::PreferenceReducerRef;
pub use types::PreferenceReductionDraft;

/// Plans the exact memory-owned authentication challenge without mutating state.
///
/// Trusted ingress clients may use the returned evidence hash as the payload
/// for a transport authentication proof. Only a later authority advance can
/// authenticate the challenge and attempt the single CAS transition.
pub fn plan_preference_feedback_challenge(
    request: PreferenceFeedbackRequest,
    source: PreferenceFeedbackSourceRef,
    reducer: PreferenceReducerRef,
) -> Result<PreferenceFeedbackChallenge, PreferenceAuthorityError> {
    source.validate()?;
    reducer.validate()?;
    Ok(PreferenceFeedbackChallenge::new(request, source, reducer))
}

impl InMemoryPreferenceStore {
    /// Authenticates, reduces, and attempts exactly one in-memory CAS advance.
    pub fn advance_preference_with_authority<A, R>(
        &self,
        request: PreferenceFeedbackRequest,
        authenticator: &A,
        reducer: &R,
    ) -> Result<PreferenceAuthorityCommitOutcome, PreferenceAuthorityError>
    where
        A: PreferenceFeedbackAuthenticator + ?Sized,
        R: PreferenceDomainReducer + ?Sized,
    {
        let current = self
            .read_document(request.preference(), request.subject())?
            .ok_or_else(|| {
                PreferenceAuthorityError::Cas(
                    PreferenceCasError::PreferenceDocumentNotInitialized {
                        preference: request.preference().clone(),
                        subject: request.subject().clone(),
                    },
                )
            })?;
        ensure_expected_previous(&request, &current)?;
        let advance = prepare_advance(current, request, authenticator, reducer)?;
        let commit = self.commit_evidenced(advance.transition, advance.document)?;
        Ok(advance.audit.complete(commit))
    }
}

impl DurablePreferenceStore {
    /// Authenticates, reduces, and attempts exactly one durable CAS advance.
    pub async fn advance_preference_with_authority<A, R>(
        &self,
        request: PreferenceFeedbackRequest,
        authenticator: &A,
        reducer: &R,
    ) -> Result<PreferenceAuthorityCommitOutcome, PreferenceAuthorityError>
    where
        A: PreferenceFeedbackAuthenticator + ?Sized,
        R: PreferenceDomainReducer + ?Sized,
    {
        let current = self
            .read_document(request.preference(), request.subject())
            .await?
            .ok_or_else(|| {
                PreferenceAuthorityError::Cas(
                    PreferenceCasError::PreferenceDocumentNotInitialized {
                        preference: request.preference().clone(),
                        subject: request.subject().clone(),
                    },
                )
            })?;
        ensure_expected_previous(&request, &current)?;
        let advance = prepare_advance(current, request, authenticator, reducer)?;
        let commit = self
            .commit_evidenced(advance.transition, advance.document)
            .await?;
        Ok(advance.audit.complete(commit))
    }
}

struct PreparedPreferenceAdvance {
    transition: PreferenceTransition,
    document: PreferenceStateDocument,
    audit: PreferenceAuthorityAudit,
}

struct PreferenceAuthorityAudit {
    transition_id: hepta_contracts::PreferenceTransitionId,
    evidence: hepta_contracts::PreferenceEvidenceRef,
    source: PreferenceFeedbackSourceRef,
    reducer: PreferenceReducerRef,
    expected_previous: PreferenceState,
}

impl PreferenceAuthorityAudit {
    fn complete(self, commit: PreferenceDocumentCommitOutcome) -> PreferenceAuthorityCommitOutcome {
        PreferenceAuthorityCommitOutcome::new(
            commit,
            self.transition_id,
            self.evidence,
            self.source,
            self.reducer,
            self.expected_previous,
        )
    }
}

fn ensure_expected_previous(
    request: &PreferenceFeedbackRequest,
    current: &PreferenceStateDocument,
) -> Result<(), PreferenceAuthorityError> {
    if current.state() == request.expected_previous() {
        return Ok(());
    }
    Err(PreferenceCasError::StateConflict {
        preference: request.preference().clone(),
        subject: request.subject().clone(),
        expected: request.expected_previous().clone(),
        actual: current.state().clone(),
    }
    .into())
}

fn prepare_advance<A, R>(
    current: PreferenceStateDocument,
    request: PreferenceFeedbackRequest,
    authenticator: &A,
    reducer: &R,
) -> Result<PreparedPreferenceAdvance, PreferenceAuthorityError>
where
    A: PreferenceFeedbackAuthenticator + ?Sized,
    R: PreferenceDomainReducer + ?Sized,
{
    let source = authenticator.source();
    source.validate()?;
    let reducer_ref = reducer.reducer();
    reducer_ref.validate()?;
    if current.reducer_version() != reducer_ref.version() {
        return Err(PreferenceAuthorityError::ReducerVersionConflict {
            current: current.reducer_version().to_owned(),
            authority: reducer_ref.version().to_owned(),
        });
    }

    let challenge = PreferenceFeedbackChallenge::new(request, source.clone(), reducer_ref.clone());
    authenticator
        .authenticate(&challenge)
        .map_err(PreferenceAuthorityError::Authentication)?;
    let actual_source = authenticator.source();
    actual_source.validate()?;
    if actual_source != source {
        return Err(PreferenceAuthorityError::SourceBindingChanged {
            expected: source,
            actual: actual_source,
        });
    }

    let feedback = challenge.into_authenticated();
    let draft = reducer
        .reduce(&current, &feedback)
        .map_err(PreferenceAuthorityError::Reduction)?;
    let actual_reducer = reducer.reducer();
    actual_reducer.validate()?;
    if actual_reducer != reducer_ref {
        return Err(PreferenceAuthorityError::ReducerBindingChanged {
            expected: reducer_ref,
            actual: actual_reducer,
        });
    }

    let transition = PreferenceTransition::try_new(
        feedback.request().transition_id().clone(),
        feedback.evidence(),
        feedback.expected_previous().clone(),
        draft.next_state().clone(),
    )?;
    let document = PreferenceStateDocument::new(
        draft.next_state().clone(),
        feedback.reducer().version(),
        draft.canonical_payload(),
    );
    let audit = PreferenceAuthorityAudit {
        transition_id: transition.id().clone(),
        evidence: feedback.evidence().clone(),
        source: feedback.source().clone(),
        reducer: feedback.reducer().clone(),
        expected_previous: feedback.expected_previous().clone(),
    };
    Ok(PreparedPreferenceAdvance {
        transition,
        document,
        audit,
    })
}

#[cfg(test)]
mod tests;
