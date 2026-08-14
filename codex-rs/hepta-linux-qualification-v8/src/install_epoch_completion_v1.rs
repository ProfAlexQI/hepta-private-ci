//! Model-only proof that the external watermark provider attested an exact
//! compare-and-swap and then freshly reported the resulting current tip.
//!
//! Production trust/profile bindings remain unpublished, so production entry
//! points fail closed. Even a verified model token grants no root installation,
//! daemon activation, trusted time, durable replay claim, trusted state root,
//! or qualification authority.

use serde::Deserialize;
use serde::Serialize;
use sha2::Digest as _;

use crate::AuthoritySignatureAlgorithmV8;
use crate::AuthoritySignerBindingV8;
use crate::CryptographicSignatureObservation;
use crate::EXTERNAL_WATERMARK_LEASE_CLAIM_SCOPE_V1;
use crate::ExternalWatermarkPredecessorV1;
use crate::INSTALL_EPOCH_AUTHORITY_CLAIM_SCOPE_V1;
use crate::InstallEpochBindingV1;
use crate::InstallEpochExactPhaseLookupV1;
use crate::InstallEpochReplayGuardV1;
use crate::QualificationError;
use crate::RootFileInstallIdentityV8;
use crate::SshsigTrustPurposeV8;
use crate::StateRootProfileBindingV1;
use crate::TargetHostBindingV8;
use crate::VerifiedInstallEpochPreparationV1;
use crate::VerifiedTrustPolicyBindingV8;
use crate::invalid;
use crate::required_frozen_trust_binding_v8;
use crate::verify_statement_sshsig_for_purpose_v8;

#[cfg(test)]
#[path = "install_epoch_completion_v1_tests.rs"]
mod tests;

#[cfg(test)]
pub(crate) fn test_only_completed_install_epoch_preparation_v1(
    model_now_unix_seconds: u64,
) -> VerifiedCommittedCurrentTipPreparationV1 {
    tests::complete_genesis(model_now_unix_seconds).0
}

#[cfg(test)]
pub(crate) fn test_only_completed_install_epoch_preparation_after_retry_v1(
    model_now_unix_seconds: u64,
) -> VerifiedCommittedCurrentTipPreparationV1 {
    tests::complete_genesis_after_one_retry(model_now_unix_seconds)
}

pub const EXTERNAL_WATERMARK_COMMIT_SCHEMA_V1: &str = "hepta_linux_v8_external_watermark_commit_v1";
pub const EXTERNAL_WATERMARK_COMMIT_NAMESPACE_V1: &str =
    "hepta-linux-v8-external-watermark-commit-v1";
pub const EXTERNAL_WATERMARK_CURRENT_TIP_SCHEMA_V1: &str =
    "hepta_linux_v8_external_watermark_current_tip_v1";
pub const EXTERNAL_WATERMARK_CURRENT_TIP_NAMESPACE_V1: &str =
    "hepta-linux-v8-external-watermark-current-tip-v1";
pub const EXTERNAL_WATERMARK_COMPLETION_PROFILE_ID_V1: &str =
    "hepta-linux-v8-external-watermark-completion-profile-v1";
pub const MAX_EXTERNAL_WATERMARK_CURRENT_TIP_LIFETIME_SECONDS_V1: u64 = 2 * 60;
pub const MAX_EXTERNAL_WATERMARK_CURRENT_TIP_RETRY_COUNT_V1: u64 = 8;

const EXTERNAL_WATERMARK_COMMIT_CLAIM_SCOPE_V1: &str =
    "hepta-linux-v8-external-watermark-commit-claim-v1";
const EXTERNAL_WATERMARK_QUERY_CLAIM_SCOPE_V1: &str =
    "hepta-linux-v8-external-watermark-query-claim-v1";
const EXTERNAL_WATERMARK_COMPLETION_CLAIM_DOMAIN_ID_V1: &str =
    "hepta-linux-v8-install-epoch-global-claim-domain-v1";

const MAX_SIGNATURE_BYTES_V1: usize = 16 * 1024;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalWatermarkCommitCapabilityV1 {
    ExactIdempotentCasByCommitNonceAndConsumeLeaseOnce,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalWatermarkCurrentTipCapabilityV1 {
    ExactIdempotentAuthenticatedCurrentTipByQueryNonceAfterCommit,
}

/// Deterministic value that the provider must store as the successor record.
/// The tip digest is the SHA-256 of this record's canonical bytes.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalWatermarkRecordV1 {
    pub completion_profile_sha256: String,
    pub prepared_epoch_binding_sha256: String,
    pub machine_id_sha256: String,
    pub predecessor: ExternalWatermarkPredecessorV1,
    pub preparation_binding_sha256: String,
    pub provider_profile_sha256: String,
    pub state_root_profile_sha256: String,
    pub stream_id_sha256: String,
    pub successor_revision: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalWatermarkCommitChallengeV1 {
    pub authority_nonce: String,
    pub capability: ExternalWatermarkCommitCapabilityV1,
    pub commit_nonce: String,
    pub committed_at_unix_seconds: u64,
    pub completion_operation_binding_sha256: String,
    pub completion_profile_sha256: String,
    pub lease_nonce: String,
    pub lease_signature_sha256: String,
    pub lease_statement_sha256: String,
    pub lease_trust_policy_sha256: String,
    pub namespace: String,
    pub predecessor: ExternalWatermarkPredecessorV1,
    pub preparation_binding_sha256: String,
    /// Opaque provider-assigned transaction identifier, represented by its
    /// non-zero SHA-256 digest. It is signed and cross-bound, not derived.
    pub provider_transaction_sha256: String,
    pub schema: String,
    pub signer: AuthoritySignerBindingV8,
    pub successor_record: ExternalWatermarkRecordV1,
    pub successor_tip_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignedExternalWatermarkCommitV1 {
    pub canonical_statement_sha256: String,
    pub challenge: ExternalWatermarkCommitChallengeV1,
    pub detached_signature_bytes: Vec<u8>,
    pub detached_signature_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalWatermarkCurrentTipChallengeV1 {
    pub capability: ExternalWatermarkCurrentTipCapabilityV1,
    pub commit_signature_sha256: String,
    pub commit_statement_sha256: String,
    pub commit_trust_policy_sha256: String,
    pub completion_profile_sha256: String,
    pub current_record: ExternalWatermarkRecordV1,
    pub current_revision: u64,
    pub current_tip_sha256: String,
    pub expires_at_unix_seconds: u64,
    pub issued_at_unix_seconds: u64,
    pub namespace: String,
    pub preparation_binding_sha256: String,
    pub provider_transaction_sha256: String,
    pub query_nonce: String,
    pub schema: String,
    pub signer: AuthoritySignerBindingV8,
    pub stream_id_sha256: String,
    pub target_host: TargetHostBindingV8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignedExternalWatermarkCurrentTipV1 {
    pub canonical_statement_sha256: String,
    pub challenge: ExternalWatermarkCurrentTipChallengeV1,
    pub detached_signature_bytes: Vec<u8>,
    pub detached_signature_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FrozenExternalWatermarkCompletionProfileV1 {
    commit_capability: ExternalWatermarkCommitCapabilityV1,
    commit_trust_policy_sha256: String,
    current_tip_capability: ExternalWatermarkCurrentTipCapabilityV1,
    current_tip_trust_policy_sha256: String,
    key_fingerprint: String,
    lease_trust_policy_sha256: String,
    maximum_current_tip_lifetime_seconds: u64,
    maximum_current_tip_retry_count: u64,
    profile_id: String,
    profile_revision: u64,
    profile_sha256: String,
    provider_profile_sha256: String,
    trust_root_id: String,
}

fn required_frozen_completion_profile_v1()
-> Result<FrozenExternalWatermarkCompletionProfileV1, QualificationError> {
    Err(invalid(
        "frozen external watermark completion profile is not independently published",
    ))
}

#[cfg(test)]
fn test_only_completion_profile_v1(
    preparation: &VerifiedInstallEpochPreparationV1,
    commit_trust: &VerifiedTrustPolicyBindingV8,
    current_tip_trust: &VerifiedTrustPolicyBindingV8,
) -> FrozenExternalWatermarkCompletionProfileV1 {
    let mut profile = FrozenExternalWatermarkCompletionProfileV1 {
        commit_capability:
            ExternalWatermarkCommitCapabilityV1::ExactIdempotentCasByCommitNonceAndConsumeLeaseOnce,
        commit_trust_policy_sha256: commit_trust.policy_sha256().to_string(),
        current_tip_capability:
            ExternalWatermarkCurrentTipCapabilityV1::ExactIdempotentAuthenticatedCurrentTipByQueryNonceAfterCommit,
        current_tip_trust_policy_sha256: current_tip_trust.policy_sha256().to_string(),
        key_fingerprint: commit_trust.key_fingerprint().to_string(),
        lease_trust_policy_sha256: preparation.lease_trust_policy_sha256().to_string(),
        maximum_current_tip_lifetime_seconds:
            MAX_EXTERNAL_WATERMARK_CURRENT_TIP_LIFETIME_SECONDS_V1,
        maximum_current_tip_retry_count: MAX_EXTERNAL_WATERMARK_CURRENT_TIP_RETRY_COUNT_V1,
        profile_id: EXTERNAL_WATERMARK_COMPLETION_PROFILE_ID_V1.to_string(),
        profile_revision: 1,
        profile_sha256: String::new(),
        provider_profile_sha256: predecessor_provider_profile_sha256(preparation.predecessor())
            .to_string(),
        trust_root_id: commit_trust.trust_root_id().to_string(),
    };
    profile.profile_sha256 = completion_profile_sha256(&profile);
    profile
}

/// A failed post-intent transition returns ownership of the exact pending
/// state. This prevents a failed commit or current-tip verification from
/// silently dropping the only recovery handle after an external effect may
/// already have happened.
#[derive(Debug)]
pub struct CompletionTransitionErrorV1<S> {
    error: QualificationError,
    pending: Box<S>,
}

impl<S> CompletionTransitionErrorV1<S> {
    fn new(error: QualificationError, pending: S) -> Self {
        Self {
            error,
            pending: Box::new(pending),
        }
    }

    pub fn error(&self) -> &QualificationError {
        &self.error
    }

    pub fn pending(&self) -> &S {
        &self.pending
    }

    pub fn into_pending(self) -> S {
        *self.pending
    }

    pub fn into_parts(self) -> (QualificationError, S) {
        (self.error, *self.pending)
    }
}

/// Opaque model of the durable CAS intent. Authority and lease claims already
/// exist and match the owned preparation; commit and initial-query claims were
/// added as one exact pair in the same global claim domain. Live code must
/// persist the intent atomically with those two claims before provider I/O.
#[derive(Debug)]
pub struct PendingExternalWatermarkCasIntentV1 {
    cas_intent_state_sha256: String,
    commit_claim_binding_sha256: String,
    commit_nonce: String,
    completion_operation_binding_sha256: String,
    completion_slot_id_sha256: String,
    initial_query_nonce: String,
    preparation: VerifiedInstallEpochPreparationV1,
    phase_state_sha256: String,
    profile: FrozenExternalWatermarkCompletionProfileV1,
    phase_revision: u64,
    query_claim_binding_sha256: String,
    phase_head_id_sha256: String,
    successor_record: ExternalWatermarkRecordV1,
    successor_tip_sha256: String,
}

impl PendingExternalWatermarkCasIntentV1 {
    pub fn preparation(&self) -> &VerifiedInstallEpochPreparationV1 {
        &self.preparation
    }

    pub fn commit_nonce(&self) -> &str {
        &self.commit_nonce
    }

    pub fn initial_query_nonce(&self) -> &str {
        &self.initial_query_nonce
    }

    pub fn completion_operation_binding_sha256(&self) -> &str {
        &self.completion_operation_binding_sha256
    }

    pub fn completion_slot_id_sha256(&self) -> &str {
        &self.completion_slot_id_sha256
    }

    pub fn cas_intent_state_sha256(&self) -> &str {
        &self.cas_intent_state_sha256
    }

    pub fn phase_head_id_sha256(&self) -> &str {
        &self.phase_head_id_sha256
    }

    pub fn phase_revision(&self) -> u64 {
        self.phase_revision
    }

    pub fn phase_state_sha256(&self) -> &str {
        &self.phase_state_sha256
    }
}

#[derive(Debug)]
pub struct FreshExternalWatermarkCasIntentV1(PendingExternalWatermarkCasIntentV1);

impl FreshExternalWatermarkCasIntentV1 {
    /// Consume the only fresh begin token and durably reserve exactly one CAS
    /// before any provider call. The returned permit must itself be consumed
    /// by the future provider executor.
    pub fn reserve_provider_cas_model(
        self,
        replay_guard: &mut InstallEpochReplayGuardV1,
    ) -> Result<
        ExternalWatermarkCasReservationOutcomeV1,
        CompletionTransitionErrorV1<FreshExternalWatermarkCasIntentV1>,
    > {
        let (fresh, intent) =
            advance_cas_issue_phase_v1(self.0, replay_guard).map_err(|error| {
                let (cause, intent) = error.into_parts();
                CompletionTransitionErrorV1::new(cause, FreshExternalWatermarkCasIntentV1(intent))
            })?;
        let issued = IssuedExternalWatermarkCasIntentV1(intent);
        if fresh {
            Ok(ExternalWatermarkCasReservationOutcomeV1::Fresh(
                ReservedExternalWatermarkCasV1(issued),
            ))
        } else {
            Ok(ExternalWatermarkCasReservationOutcomeV1::Recovered(
                RecoveredReservedExternalWatermarkCasV1(issued),
            ))
        }
    }
}

/// Opaque typestate proving the durable provider-CAS issue edge exists. Commit
/// verification accepts only this state, never the rev1 prepared intent.
#[derive(Debug)]
pub struct IssuedExternalWatermarkCasIntentV1(PendingExternalWatermarkCasIntentV1);

impl IssuedExternalWatermarkCasIntentV1 {
    pub fn preparation(&self) -> &VerifiedInstallEpochPreparationV1 {
        self.0.preparation()
    }

    pub fn commit_nonce(&self) -> &str {
        self.0.commit_nonce()
    }

    pub fn initial_query_nonce(&self) -> &str {
        self.0.initial_query_nonce()
    }

    pub fn completion_operation_binding_sha256(&self) -> &str {
        self.0.completion_operation_binding_sha256()
    }

    pub fn completion_slot_id_sha256(&self) -> &str {
        self.0.completion_slot_id_sha256()
    }

    pub fn cas_intent_state_sha256(&self) -> &str {
        self.0.cas_intent_state_sha256()
    }

    pub fn phase_head_id_sha256(&self) -> &str {
        self.0.phase_head_id_sha256()
    }

    pub fn phase_revision(&self) -> u64 {
        self.0.phase_revision()
    }

    pub fn phase_state_sha256(&self) -> &str {
        self.0.phase_state_sha256()
    }
}

/// Linear model permit for one exact provider CAS dispatch. The durable issue
/// edge is already committed; a future executor must consume this value and
/// use the frozen commit nonce as its idempotency key.
#[derive(Debug)]
pub struct ReservedExternalWatermarkCasV1(IssuedExternalWatermarkCasIntentV1);

impl ReservedExternalWatermarkCasV1 {
    pub fn into_pending_after_provider_call(self) -> IssuedExternalWatermarkCasIntentV1 {
        self.0
    }
}

/// Recovery view of the same durable outbox item. The provider contract is
/// explicitly idempotent by the frozen commit nonce, so a crash recovery may
/// re-dispatch only the byte-identical operation or reconcile a signed receipt.
#[derive(Debug)]
pub struct RecoveredReservedExternalWatermarkCasV1(IssuedExternalWatermarkCasIntentV1);

impl RecoveredReservedExternalWatermarkCasV1 {
    pub fn into_pending_after_exact_idempotent_provider_call(
        self,
    ) -> IssuedExternalWatermarkCasIntentV1 {
        self.0
    }

    pub fn into_pending_for_receipt_reconciliation(self) -> IssuedExternalWatermarkCasIntentV1 {
        self.0
    }
}

#[derive(Debug)]
pub enum ExternalWatermarkCasReservationOutcomeV1 {
    Fresh(ReservedExternalWatermarkCasV1),
    Recovered(RecoveredReservedExternalWatermarkCasV1),
}

#[derive(Debug)]
pub struct RecoveredExternalWatermarkCasIntentV1(PendingExternalWatermarkCasIntentV1);

impl RecoveredExternalWatermarkCasIntentV1 {
    /// A recovered genesis is not evidence that provider CAS was issued. It
    /// may atomically compete for the same durable issue edge. The creator
    /// receives the fresh dispatch token; exact recovery receives only the
    /// byte-identical idempotent outbox item bound to the same commit nonce.
    pub fn reserve_provider_cas_model(
        self,
        replay_guard: &mut InstallEpochReplayGuardV1,
    ) -> Result<
        ExternalWatermarkCasReservationOutcomeV1,
        CompletionTransitionErrorV1<RecoveredExternalWatermarkCasIntentV1>,
    > {
        let (fresh, intent) =
            advance_cas_issue_phase_v1(self.0, replay_guard).map_err(|error| {
                let (cause, intent) = error.into_parts();
                CompletionTransitionErrorV1::new(
                    cause,
                    RecoveredExternalWatermarkCasIntentV1(intent),
                )
            })?;
        let issued = IssuedExternalWatermarkCasIntentV1(intent);
        if fresh {
            Ok(ExternalWatermarkCasReservationOutcomeV1::Fresh(
                ReservedExternalWatermarkCasV1(issued),
            ))
        } else {
            Ok(ExternalWatermarkCasReservationOutcomeV1::Recovered(
                RecoveredReservedExternalWatermarkCasV1(issued),
            ))
        }
    }

    pub fn into_pending_for_reconciliation(
        self,
        replay_guard: &InstallEpochReplayGuardV1,
    ) -> Result<
        IssuedExternalWatermarkCasIntentV1,
        CompletionTransitionErrorV1<RecoveredExternalWatermarkCasIntentV1>,
    > {
        recover_cas_issue_phase_v1(self.0, replay_guard)
            .map(IssuedExternalWatermarkCasIntentV1)
            .map_err(|error| {
                let (cause, intent) = error.into_parts();
                CompletionTransitionErrorV1::new(
                    cause,
                    RecoveredExternalWatermarkCasIntentV1(intent),
                )
            })
    }
}

#[derive(Debug)]
pub enum ExternalWatermarkCasIntentOutcomeV1 {
    Fresh(FreshExternalWatermarkCasIntentV1),
    Recovered(RecoveredExternalWatermarkCasIntentV1),
}

struct CasIntentRequiredBindingsV1 {
    authority_claim: String,
    lease_claim: String,
    preparation_bundle_binding: String,
    preparation_bundle_id: String,
}

fn cas_intent_required_bindings_v1(
    intent: &PendingExternalWatermarkCasIntentV1,
) -> CasIntentRequiredBindingsV1 {
    CasIntentRequiredBindingsV1 {
        authority_claim: intent.preparation.authority_nonce_claim_binding_sha256(),
        lease_claim: intent.preparation.lease_nonce_claim_binding_sha256(),
        preparation_bundle_binding: intent.preparation.preparation_bundle_binding_sha256(),
        preparation_bundle_id: intent.preparation.preparation_bundle_id_sha256(),
    }
}

fn advance_cas_issue_phase_v1(
    mut intent: PendingExternalWatermarkCasIntentV1,
    replay_guard: &mut InstallEpochReplayGuardV1,
) -> Result<
    (bool, PendingExternalWatermarkCasIntentV1),
    CompletionTransitionErrorV1<PendingExternalWatermarkCasIntentV1>,
> {
    let next_revision = match intent.phase_revision.checked_add(1) {
        Some(value) => value,
        None => {
            return Err(CompletionTransitionErrorV1::new(
                invalid("provider CAS issue phase revision overflows"),
                intent,
            ));
        }
    };
    let next_state_sha256 = cas_issue_state_sha256_v1(
        &intent.phase_state_sha256,
        next_revision,
        &intent.completion_operation_binding_sha256,
    );
    let bindings = cas_intent_required_bindings_v1(&intent);
    let required_claims = [
        (
            intent.preparation.authority_nonce(),
            INSTALL_EPOCH_AUTHORITY_CLAIM_SCOPE_V1,
            bindings.authority_claim.as_str(),
        ),
        (
            intent.preparation.lease_nonce(),
            EXTERNAL_WATERMARK_LEASE_CLAIM_SCOPE_V1,
            bindings.lease_claim.as_str(),
        ),
        (
            intent.commit_nonce.as_str(),
            EXTERNAL_WATERMARK_COMMIT_CLAIM_SCOPE_V1,
            intent.commit_claim_binding_sha256.as_str(),
        ),
        (
            intent.initial_query_nonce.as_str(),
            EXTERNAL_WATERMARK_QUERY_CLAIM_SCOPE_V1,
            intent.query_claim_binding_sha256.as_str(),
        ),
    ];
    let required_bundles = [
        (
            bindings.preparation_bundle_id.as_str(),
            bindings.preparation_bundle_binding.as_str(),
        ),
        (
            intent.completion_slot_id_sha256.as_str(),
            intent.cas_intent_state_sha256.as_str(),
        ),
    ];
    let fresh = match replay_guard.advance_phase_or_exact_recovery(
        &required_claims,
        &required_bundles,
        &intent.phase_head_id_sha256,
        intent.phase_revision,
        &intent.phase_state_sha256,
        next_revision,
        &next_state_sha256,
    ) {
        Ok(value) => value,
        Err(error) => return Err(CompletionTransitionErrorV1::new(error, intent)),
    };
    intent.phase_revision = next_revision;
    intent.phase_state_sha256 = next_state_sha256;
    Ok((fresh, intent))
}

fn recover_cas_issue_phase_v1(
    mut intent: PendingExternalWatermarkCasIntentV1,
    replay_guard: &InstallEpochReplayGuardV1,
) -> Result<
    PendingExternalWatermarkCasIntentV1,
    CompletionTransitionErrorV1<PendingExternalWatermarkCasIntentV1>,
> {
    let next_revision = match intent.phase_revision.checked_add(1) {
        Some(value) => value,
        None => {
            return Err(CompletionTransitionErrorV1::new(
                invalid("provider CAS recovery phase revision overflows"),
                intent,
            ));
        }
    };
    let next_state_sha256 = cas_issue_state_sha256_v1(
        &intent.phase_state_sha256,
        next_revision,
        &intent.completion_operation_binding_sha256,
    );
    let bindings = cas_intent_required_bindings_v1(&intent);
    let required_claims = [
        (
            intent.preparation.authority_nonce(),
            INSTALL_EPOCH_AUTHORITY_CLAIM_SCOPE_V1,
            bindings.authority_claim.as_str(),
        ),
        (
            intent.preparation.lease_nonce(),
            EXTERNAL_WATERMARK_LEASE_CLAIM_SCOPE_V1,
            bindings.lease_claim.as_str(),
        ),
        (
            intent.commit_nonce.as_str(),
            EXTERNAL_WATERMARK_COMMIT_CLAIM_SCOPE_V1,
            intent.commit_claim_binding_sha256.as_str(),
        ),
        (
            intent.initial_query_nonce.as_str(),
            EXTERNAL_WATERMARK_QUERY_CLAIM_SCOPE_V1,
            intent.query_claim_binding_sha256.as_str(),
        ),
    ];
    let required_bundles = [
        (
            bindings.preparation_bundle_id.as_str(),
            bindings.preparation_bundle_binding.as_str(),
        ),
        (
            intent.completion_slot_id_sha256.as_str(),
            intent.cas_intent_state_sha256.as_str(),
        ),
    ];
    match replay_guard.lookup_exact_phase_transition(
        &required_claims,
        &required_bundles,
        &intent.phase_head_id_sha256,
        intent.phase_revision,
        &intent.phase_state_sha256,
        next_revision,
        &next_state_sha256,
    ) {
        Ok(InstallEpochExactPhaseLookupV1::Exact) => {}
        Ok(InstallEpochExactPhaseLookupV1::Absent) => {
            return Err(CompletionTransitionErrorV1::new(
                invalid("provider CAS issue edge is absent; reserve it before reconciliation"),
                intent,
            ));
        }
        Err(error) => return Err(CompletionTransitionErrorV1::new(error, intent)),
    }
    intent.phase_revision = next_revision;
    intent.phase_state_sha256 = next_state_sha256;
    Ok(intent)
}

#[derive(Debug)]
struct VerifiedProviderCasCommitV1 {
    committed_at_unix_seconds: u64,
    provider_transaction_sha256: String,
    signature_sha256: String,
    statement_sha256: String,
    trust_policy_sha256: String,
}

/// Provider CAS is authenticated and cross-bound, while a fresh current-tip
/// result is still pending. This token owns the original preparation and is
/// returned intact on every later verification failure.
#[derive(Debug)]
pub struct VerifiedCasCommittedPendingTipV1 {
    active_query_claim_binding_sha256: String,
    active_query_may_issue_model: bool,
    active_query_nonce: String,
    active_query_phase_durable: bool,
    active_query_sequence: u64,
    active_query_state_sha256: String,
    cas_receipt_state_sha256: String,
    commit: VerifiedProviderCasCommitV1,
    intent: PendingExternalWatermarkCasIntentV1,
    phase_revision: u64,
}

impl VerifiedCasCommittedPendingTipV1 {
    pub fn preparation(&self) -> &VerifiedInstallEpochPreparationV1 {
        &self.intent.preparation
    }

    pub fn commit_nonce(&self) -> &str {
        &self.intent.commit_nonce
    }

    pub fn active_query_nonce(&self) -> &str {
        &self.active_query_nonce
    }

    #[cfg(test)]
    pub(crate) fn active_query_may_issue_model(&self) -> bool {
        self.active_query_may_issue_model
    }

    pub fn active_query_state_sha256(&self) -> &str {
        &self.active_query_state_sha256
    }

    pub fn active_query_sequence(&self) -> u64 {
        self.active_query_sequence
    }

    pub fn phase_revision(&self) -> u64 {
        self.phase_revision
    }

    pub fn committed_at_unix_seconds(&self) -> u64 {
        self.commit.committed_at_unix_seconds
    }

    pub fn cas_receipt_state_sha256(&self) -> &str {
        &self.cas_receipt_state_sha256
    }

    /// Atomically reserve the active query in the durable phase chain before
    /// provider I/O. Exact replay returns a recovered outbox item bound to the
    /// same idempotent query nonce; it never creates a second query identity.
    pub fn reserve_current_tip_query_model(
        mut self,
        replay_guard: &mut InstallEpochReplayGuardV1,
    ) -> Result<
        ExternalWatermarkCurrentTipQueryReservationOutcomeV1,
        CompletionTransitionErrorV1<Self>,
    > {
        if !self.active_query_may_issue_model {
            return Err(CompletionTransitionErrorV1::new(
                invalid("current-tip query is recovery-only or was already issued"),
                self,
            ));
        }
        if self.active_query_phase_durable {
            self.active_query_may_issue_model = false;
            return Ok(ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Fresh(
                ReservedExternalWatermarkCurrentTipQueryV1(
                    IssuedExternalWatermarkCurrentTipQueryV1(self),
                ),
            ));
        }
        let next_revision = match self.phase_revision.checked_add(1) {
            Some(value) => value,
            None => {
                return Err(CompletionTransitionErrorV1::new(
                    invalid("current-tip query phase revision overflows"),
                    self,
                ));
            }
        };
        let authority_claim = self.preparation().authority_nonce_claim_binding_sha256();
        let lease_claim = self.preparation().lease_nonce_claim_binding_sha256();
        let preparation_bundle_id = self.preparation().preparation_bundle_id_sha256();
        let preparation_bundle_binding = self.preparation().preparation_bundle_binding_sha256();
        let required_claims = [
            (
                self.preparation().authority_nonce(),
                INSTALL_EPOCH_AUTHORITY_CLAIM_SCOPE_V1,
                authority_claim.as_str(),
            ),
            (
                self.preparation().lease_nonce(),
                EXTERNAL_WATERMARK_LEASE_CLAIM_SCOPE_V1,
                lease_claim.as_str(),
            ),
            (
                self.commit_nonce(),
                EXTERNAL_WATERMARK_COMMIT_CLAIM_SCOPE_V1,
                self.intent.commit_claim_binding_sha256.as_str(),
            ),
            (
                self.active_query_nonce(),
                EXTERNAL_WATERMARK_QUERY_CLAIM_SCOPE_V1,
                self.active_query_claim_binding_sha256.as_str(),
            ),
        ];
        let required_bundles = [
            (
                preparation_bundle_id.as_str(),
                preparation_bundle_binding.as_str(),
            ),
            (
                self.intent.completion_slot_id_sha256.as_str(),
                self.intent.cas_intent_state_sha256.as_str(),
            ),
        ];
        let fresh = match replay_guard.advance_phase_or_exact_recovery(
            &required_claims,
            &required_bundles,
            &self.intent.phase_head_id_sha256,
            self.phase_revision,
            &self.cas_receipt_state_sha256,
            next_revision,
            &self.active_query_state_sha256,
        ) {
            Ok(value) => value,
            Err(error) => return Err(CompletionTransitionErrorV1::new(error, self)),
        };
        self.phase_revision = next_revision;
        self.active_query_phase_durable = true;
        self.active_query_may_issue_model = false;
        let issued = IssuedExternalWatermarkCurrentTipQueryV1(self);
        if fresh {
            Ok(ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Fresh(
                ReservedExternalWatermarkCurrentTipQueryV1(issued),
            ))
        } else {
            Ok(
                ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Recovered(
                    RecoveredReservedExternalWatermarkCurrentTipQueryV1(issued),
                ),
            )
        }
    }
}

/// Typestate proving the active query phase is durable. Signed current-tip
/// verification and retry closure both require this state.
#[derive(Debug)]
pub struct IssuedExternalWatermarkCurrentTipQueryV1(VerifiedCasCommittedPendingTipV1);

impl IssuedExternalWatermarkCurrentTipQueryV1 {
    pub fn preparation(&self) -> &VerifiedInstallEpochPreparationV1 {
        self.0.preparation()
    }

    pub fn commit_nonce(&self) -> &str {
        self.0.commit_nonce()
    }

    pub fn active_query_nonce(&self) -> &str {
        self.0.active_query_nonce()
    }

    pub fn active_query_state_sha256(&self) -> &str {
        self.0.active_query_state_sha256()
    }

    pub fn active_query_sequence(&self) -> u64 {
        self.0.active_query_sequence()
    }

    pub fn phase_revision(&self) -> u64 {
        self.0.phase_revision()
    }

    pub fn committed_at_unix_seconds(&self) -> u64 {
        self.0.committed_at_unix_seconds()
    }

    pub fn cas_receipt_state_sha256(&self) -> &str {
        self.0.cas_receipt_state_sha256()
    }
}

#[derive(Debug)]
pub struct ReservedExternalWatermarkCurrentTipQueryV1(IssuedExternalWatermarkCurrentTipQueryV1);

impl ReservedExternalWatermarkCurrentTipQueryV1 {
    pub fn into_pending_after_provider_call(self) -> IssuedExternalWatermarkCurrentTipQueryV1 {
        self.0
    }
}

#[derive(Debug)]
pub struct RecoveredReservedExternalWatermarkCurrentTipQueryV1(
    IssuedExternalWatermarkCurrentTipQueryV1,
);

impl RecoveredReservedExternalWatermarkCurrentTipQueryV1 {
    pub fn into_pending_after_exact_idempotent_provider_call(
        self,
    ) -> IssuedExternalWatermarkCurrentTipQueryV1 {
        self.0
    }

    pub fn into_pending_for_receipt_reconciliation(
        self,
    ) -> IssuedExternalWatermarkCurrentTipQueryV1 {
        self.0
    }
}

#[derive(Debug)]
pub enum ExternalWatermarkCurrentTipQueryReservationOutcomeV1 {
    Fresh(ReservedExternalWatermarkCurrentTipQueryV1),
    Recovered(RecoveredReservedExternalWatermarkCurrentTipQueryV1),
}

/// Reconstruct an already-reserved initial current-tip query without writing
/// the phase ledger or granting provider I/O. This is the only recovery path
/// for a token rebuilt after the durable query edge was committed.
pub fn recover_external_watermark_current_tip_query_v1(
    mut pending: VerifiedCasCommittedPendingTipV1,
    replay_guard: &InstallEpochReplayGuardV1,
) -> Result<
    RecoveredReservedExternalWatermarkCurrentTipQueryV1,
    CompletionTransitionErrorV1<VerifiedCasCommittedPendingTipV1>,
> {
    if pending.active_query_phase_durable {
        return Err(CompletionTransitionErrorV1::new(
            invalid("current-tip query recovery requires a non-durable reconstructed token"),
            pending,
        ));
    }
    let next_revision = match pending.phase_revision.checked_add(1) {
        Some(value) => value,
        None => {
            return Err(CompletionTransitionErrorV1::new(
                invalid("current-tip query recovery phase revision overflows"),
                pending,
            ));
        }
    };
    let authority_claim = pending.preparation().authority_nonce_claim_binding_sha256();
    let lease_claim = pending.preparation().lease_nonce_claim_binding_sha256();
    let preparation_bundle_id = pending.preparation().preparation_bundle_id_sha256();
    let preparation_bundle_binding = pending.preparation().preparation_bundle_binding_sha256();
    let required_claims = [
        (
            pending.preparation().authority_nonce(),
            INSTALL_EPOCH_AUTHORITY_CLAIM_SCOPE_V1,
            authority_claim.as_str(),
        ),
        (
            pending.preparation().lease_nonce(),
            EXTERNAL_WATERMARK_LEASE_CLAIM_SCOPE_V1,
            lease_claim.as_str(),
        ),
        (
            pending.commit_nonce(),
            EXTERNAL_WATERMARK_COMMIT_CLAIM_SCOPE_V1,
            pending.intent.commit_claim_binding_sha256.as_str(),
        ),
        (
            pending.active_query_nonce(),
            EXTERNAL_WATERMARK_QUERY_CLAIM_SCOPE_V1,
            pending.active_query_claim_binding_sha256.as_str(),
        ),
    ];
    let required_bundles = [
        (
            preparation_bundle_id.as_str(),
            preparation_bundle_binding.as_str(),
        ),
        (
            pending.intent.completion_slot_id_sha256.as_str(),
            pending.intent.cas_intent_state_sha256.as_str(),
        ),
    ];
    match replay_guard.lookup_exact_phase_transition(
        &required_claims,
        &required_bundles,
        &pending.intent.phase_head_id_sha256,
        pending.phase_revision,
        &pending.cas_receipt_state_sha256,
        next_revision,
        &pending.active_query_state_sha256,
    ) {
        Ok(InstallEpochExactPhaseLookupV1::Exact) => {}
        Ok(InstallEpochExactPhaseLookupV1::Absent) => {
            return Err(CompletionTransitionErrorV1::new(
                invalid("current-tip query edge is absent; reserve it before recovery"),
                pending,
            ));
        }
        Err(error) => return Err(CompletionTransitionErrorV1::new(error, pending)),
    }
    pending.phase_revision = next_revision;
    pending.active_query_phase_durable = true;
    pending.active_query_may_issue_model = false;
    Ok(RecoveredReservedExternalWatermarkCurrentTipQueryV1(
        IssuedExternalWatermarkCurrentTipQueryV1(pending),
    ))
}

/// Opaque evidence that the exact active query reached a terminal provider
/// outcome without yielding an admissible current-tip receipt. There is no
/// production constructor until an independently frozen closure verifier and
/// profile are published; consequently live retry remains fail closed.
#[derive(Debug)]
pub struct VerifiedExternalWatermarkCurrentTipQueryClosureV1 {
    closure_binding_sha256: String,
    closure_evidence_sha256: String,
    closure_profile_sha256: String,
    completion_operation_binding_sha256: String,
    phase_head_id_sha256: String,
    provider_transaction_sha256: String,
    query_claim_binding_sha256: String,
    query_nonce: String,
    query_phase_revision: u64,
    query_sequence: u64,
    query_state_sha256: String,
}

fn verify_query_closure_matches_pending_v1(
    pending: &VerifiedCasCommittedPendingTipV1,
    closure: &VerifiedExternalWatermarkCurrentTipQueryClosureV1,
) -> Result<(), QualificationError> {
    for (label, value) in [
        (
            "query-closure binding",
            closure.closure_binding_sha256.as_str(),
        ),
        (
            "query-closure evidence",
            closure.closure_evidence_sha256.as_str(),
        ),
        (
            "query-closure profile",
            closure.closure_profile_sha256.as_str(),
        ),
    ] {
        validate_digest(label, value)?;
    }
    if closure.completion_operation_binding_sha256
        != pending.intent.completion_operation_binding_sha256
        || closure.phase_head_id_sha256 != pending.intent.phase_head_id_sha256
        || closure.provider_transaction_sha256 != pending.commit.provider_transaction_sha256
        || closure.query_claim_binding_sha256 != pending.active_query_claim_binding_sha256
        || closure.query_nonce != pending.active_query_nonce
        || closure.query_phase_revision != pending.phase_revision
        || closure.query_sequence != pending.active_query_sequence
        || closure.query_state_sha256 != pending.active_query_state_sha256
        || closure.closure_binding_sha256 != query_closure_binding_sha256_v1(closure)
    {
        return Err(invalid(
            "current-tip query closure does not exactly bind the active query",
        ));
    }
    Ok(())
}

#[cfg(test)]
fn verified_query_closure_for_test_v1(
    issued: &IssuedExternalWatermarkCurrentTipQueryV1,
    closure_evidence_sha256: String,
) -> VerifiedExternalWatermarkCurrentTipQueryClosureV1 {
    let pending = &issued.0;
    let closure_profile_sha256 =
        sha256(b"hepta_linux_v8_test_only_current_tip_query_closure_profile_v1");
    let mut closure = VerifiedExternalWatermarkCurrentTipQueryClosureV1 {
        closure_binding_sha256: String::new(),
        closure_evidence_sha256,
        closure_profile_sha256,
        completion_operation_binding_sha256: pending
            .intent
            .completion_operation_binding_sha256
            .clone(),
        phase_head_id_sha256: pending.intent.phase_head_id_sha256.clone(),
        provider_transaction_sha256: pending.commit.provider_transaction_sha256.clone(),
        query_claim_binding_sha256: pending.active_query_claim_binding_sha256.clone(),
        query_nonce: pending.active_query_nonce.clone(),
        query_phase_revision: pending.phase_revision,
        query_sequence: pending.active_query_sequence,
        query_state_sha256: pending.active_query_state_sha256.clone(),
    };
    closure.closure_binding_sha256 = query_closure_binding_sha256_v1(&closure);
    closure
}

/// Append a fresh query claim while retaining the one committed CAS intent.
/// Reusing an exact prior query yields a reconciliation-only state; a new
/// nonce yields one durable query outbox item. The opaque closure is consumed
/// by value, so a reservation-only or synthetic recovered query cannot mint a
/// retry. No path here can authorize another CAS.
#[derive(Debug)]
pub struct ExternalWatermarkCurrentTipRetryAttemptV1 {
    closure: VerifiedExternalWatermarkCurrentTipQueryClosureV1,
    issued: IssuedExternalWatermarkCurrentTipQueryV1,
    query_nonce: String,
}

impl ExternalWatermarkCurrentTipRetryAttemptV1 {
    pub fn issued(&self) -> &IssuedExternalWatermarkCurrentTipQueryV1 {
        &self.issued
    }

    pub fn query_nonce(&self) -> &str {
        &self.query_nonce
    }

    pub fn into_parts(
        self,
    ) -> (
        IssuedExternalWatermarkCurrentTipQueryV1,
        VerifiedExternalWatermarkCurrentTipQueryClosureV1,
        String,
    ) {
        (self.issued, self.closure, self.query_nonce)
    }
}

pub fn prepare_external_watermark_current_tip_retry_v1(
    issued: IssuedExternalWatermarkCurrentTipQueryV1,
    closure: VerifiedExternalWatermarkCurrentTipQueryClosureV1,
    query_nonce: String,
    replay_guard: &mut InstallEpochReplayGuardV1,
) -> Result<
    ExternalWatermarkCurrentTipQueryReservationOutcomeV1,
    CompletionTransitionErrorV1<ExternalWatermarkCurrentTipRetryAttemptV1>,
> {
    let mut pending = issued.0;
    let result = (|| {
        if !pending.active_query_phase_durable || pending.active_query_may_issue_model {
            return Err(invalid(
                "current-tip retry requires a durable, consumed predecessor query",
            ));
        }
        if pending.active_query_sequence > pending.intent.profile.maximum_current_tip_retry_count {
            return Err(invalid(
                "current-tip retry budget is exhausted and the terminal edge is reserved",
            ));
        }
        verify_query_closure_matches_pending_v1(&pending, &closure)?;
        validate_digest("current-tip retry nonce", &query_nonce)?;
        if query_nonce == pending.preparation().authority_nonce()
            || query_nonce == pending.preparation().lease_nonce()
            || query_nonce == pending.commit_nonce()
            || query_nonce == pending.active_query_nonce()
        {
            return Err(invalid(
                "current-tip retry nonce collides with the completion operation",
            ));
        }
        let authority_claim = pending.preparation().authority_nonce_claim_binding_sha256();
        let lease_claim = pending.preparation().lease_nonce_claim_binding_sha256();
        let preparation_bundle_id = pending.preparation().preparation_bundle_id_sha256();
        let preparation_bundle_binding = pending.preparation().preparation_bundle_binding_sha256();
        let required_claims = [
            (
                pending.preparation().authority_nonce(),
                INSTALL_EPOCH_AUTHORITY_CLAIM_SCOPE_V1,
                authority_claim.as_str(),
            ),
            (
                pending.preparation().lease_nonce(),
                EXTERNAL_WATERMARK_LEASE_CLAIM_SCOPE_V1,
                lease_claim.as_str(),
            ),
            (
                pending.commit_nonce(),
                EXTERNAL_WATERMARK_COMMIT_CLAIM_SCOPE_V1,
                pending.intent.commit_claim_binding_sha256.as_str(),
            ),
            (
                pending.active_query_nonce(),
                EXTERNAL_WATERMARK_QUERY_CLAIM_SCOPE_V1,
                pending.active_query_claim_binding_sha256.as_str(),
            ),
        ];
        let mut required_bundles = vec![
            (
                preparation_bundle_id.as_str(),
                preparation_bundle_binding.as_str(),
            ),
            (
                pending.intent.completion_slot_id_sha256.as_str(),
                pending.intent.cas_intent_state_sha256.as_str(),
            ),
        ];
        if pending.active_query_sequence > 1 {
            required_bundles.push((
                pending.active_query_claim_binding_sha256.as_str(),
                pending.active_query_state_sha256.as_str(),
            ));
        }
        let query_claim = completion_retry_nonce_claim_binding_sha256_v1(
            &query_nonce,
            pending.active_query_nonce(),
            pending.commit_nonce(),
            &pending.intent.completion_operation_binding_sha256,
            &closure.closure_binding_sha256,
        );
        let next_query_sequence = pending
            .active_query_sequence
            .checked_add(1)
            .ok_or_else(|| invalid("current-tip query sequence overflows"))?;
        let next_phase_revision = pending
            .phase_revision
            .checked_add(1)
            .ok_or_else(|| invalid("current-tip query phase revision overflows"))?;
        let query_state = current_tip_query_state_sha256_v1(
            &pending.active_query_state_sha256,
            next_phase_revision,
            next_query_sequence,
            &query_claim,
            Some(&closure.closure_binding_sha256),
        );
        let fresh = replay_guard.claim_bundle_and_advance_phase_or_exact_recovery(
            &required_claims,
            &required_bundles,
            (
                &query_nonce,
                EXTERNAL_WATERMARK_QUERY_CLAIM_SCOPE_V1,
                &query_claim,
            ),
            &query_claim,
            &query_state,
            &pending.intent.phase_head_id_sha256,
            pending.phase_revision,
            &pending.active_query_state_sha256,
            next_phase_revision,
            &query_state,
        )?;
        Ok((
            fresh,
            query_claim,
            query_state,
            next_query_sequence,
            next_phase_revision,
        ))
    })();
    match result {
        Ok((fresh, query_claim, query_state, query_sequence, phase_revision)) => {
            pending.active_query_claim_binding_sha256 = query_claim;
            pending.active_query_may_issue_model = false;
            pending.active_query_nonce = query_nonce;
            pending.active_query_phase_durable = true;
            pending.active_query_sequence = query_sequence;
            pending.active_query_state_sha256 = query_state;
            pending.phase_revision = phase_revision;
            let issued = IssuedExternalWatermarkCurrentTipQueryV1(pending);
            if fresh {
                Ok(ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Fresh(
                    ReservedExternalWatermarkCurrentTipQueryV1(issued),
                ))
            } else {
                Ok(
                    ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Recovered(
                        RecoveredReservedExternalWatermarkCurrentTipQueryV1(issued),
                    ),
                )
            }
        }
        Err(error) => Err(CompletionTransitionErrorV1::new(
            error,
            ExternalWatermarkCurrentTipRetryAttemptV1 {
                closure,
                issued: IssuedExternalWatermarkCurrentTipQueryV1(pending),
                query_nonce,
            },
        )),
    }
}

/// Opaque model proof. It owns and consumes the preparation token, but still
/// grants no live or privileged operation.
#[derive(Debug)]
pub struct VerifiedCommittedCurrentTipPreparationV1 {
    current_tip_expires_at_unix_seconds: u64,
    current_tip_issued_at_unix_seconds: u64,
    current_tip_signature_sha256: String,
    current_tip_statement_sha256: String,
    current_tip_trust_policy_sha256: String,
    final_phase_fresh_model: bool,
    final_phase_revision: u64,
    finalized_state_sha256: String,
    model_completed_at_unix_seconds: u64,
    pending: VerifiedCasCommittedPendingTipV1,
    query_nonce: String,
    successor_record: ExternalWatermarkRecordV1,
    successor_tip_sha256: String,
}

impl VerifiedCommittedCurrentTipPreparationV1 {
    pub(crate) fn durable_projection_source_v1(
        &self,
    ) -> crate::install_epoch_durable_projection_v1::InstallEpochDurableProjectionSourceV1 {
        let preparation = self.preparation();
        crate::install_epoch_durable_projection_v1::InstallEpochDurableProjectionSourceV1 {
            active_query_bundle: (self.pending.active_query_sequence > 1).then(|| {
                crate::install_epoch_durable_projection_v1::RawDurableBundleProjectionV1 {
                    binding_sha256: self.pending.active_query_state_sha256.clone(),
                    id_sha256: self.pending.active_query_claim_binding_sha256.clone(),
                }
            }),
            active_query_revision: self.pending.phase_revision,
            active_query_sequence: self.pending.active_query_sequence,
            active_query_state_sha256: self.pending.active_query_state_sha256.clone(),
            authority_claim:
                crate::install_epoch_durable_projection_v1::RawDurableNonceClaimProjectionV1 {
                    binding_sha256: preparation.authority_nonce_claim_binding_sha256(),
                    nonce: preparation.authority_nonce().to_string(),
                    scope: INSTALL_EPOCH_AUTHORITY_CLAIM_SCOPE_V1.to_string(),
                },
            cas_intent_revision: 1,
            cas_intent_state_sha256: self.pending.intent.cas_intent_state_sha256.clone(),
            cas_issue_revision: self.pending.intent.phase_revision,
            cas_issue_state_sha256: self.pending.intent.phase_state_sha256.clone(),
            cas_receipt_revision: 3,
            cas_receipt_state_sha256: self.pending.cas_receipt_state_sha256.clone(),
            claim_domain_id: EXTERNAL_WATERMARK_COMPLETION_CLAIM_DOMAIN_ID_V1.to_string(),
            commit_claim:
                crate::install_epoch_durable_projection_v1::RawDurableNonceClaimProjectionV1 {
                    binding_sha256: self.pending.intent.commit_claim_binding_sha256.clone(),
                    nonce: self.pending.intent.commit_nonce.clone(),
                    scope: EXTERNAL_WATERMARK_COMMIT_CLAIM_SCOPE_V1.to_string(),
                },
            commit_signature_sha256: self.pending.commit.signature_sha256.clone(),
            commit_statement_sha256: self.pending.commit.statement_sha256.clone(),
            commit_trust_policy_sha256: self.pending.commit.trust_policy_sha256.clone(),
            committed_at_unix_seconds: self.pending.commit.committed_at_unix_seconds,
            completion_bundle:
                crate::install_epoch_durable_projection_v1::RawDurableBundleProjectionV1 {
                    binding_sha256: self.pending.intent.cas_intent_state_sha256.clone(),
                    id_sha256: self.pending.intent.completion_slot_id_sha256.clone(),
                },
            completion_operation_binding_sha256: self
                .pending
                .intent
                .completion_operation_binding_sha256
                .clone(),
            completion_profile_sha256: self.pending.intent.profile.profile_sha256.clone(),
            current_tip_expires_at_unix_seconds: self.current_tip_expires_at_unix_seconds,
            current_tip_issued_at_unix_seconds: self.current_tip_issued_at_unix_seconds,
            current_tip_query_nonce: self.query_nonce.clone(),
            current_tip_signature_sha256: self.current_tip_signature_sha256.clone(),
            current_tip_statement_sha256: self.current_tip_statement_sha256.clone(),
            current_tip_trust_policy_sha256: self.current_tip_trust_policy_sha256.clone(),
            epoch: preparation.epoch().clone(),
            final_phase_revision: self.final_phase_revision,
            finalized_state_sha256: self.finalized_state_sha256.clone(),
            initial_query_nonce: self.pending.intent.initial_query_nonce.clone(),
            initial_query_claim:
                crate::install_epoch_durable_projection_v1::RawDurableNonceClaimProjectionV1 {
                    binding_sha256: self.pending.intent.query_claim_binding_sha256.clone(),
                    nonce: self.pending.intent.initial_query_nonce.clone(),
                    scope: EXTERNAL_WATERMARK_QUERY_CLAIM_SCOPE_V1.to_string(),
                },
            lease_claim:
                crate::install_epoch_durable_projection_v1::RawDurableNonceClaimProjectionV1 {
                    binding_sha256: preparation.lease_nonce_claim_binding_sha256(),
                    nonce: preparation.lease_nonce().to_string(),
                    scope: EXTERNAL_WATERMARK_LEASE_CLAIM_SCOPE_V1.to_string(),
                },
            machine_id_sha256: preparation.target_host().machine_id_sha256.clone(),
            phase_head_id_sha256: self.pending.intent.phase_head_id_sha256.clone(),
            prepared_epoch_binding_sha256: prepared_epoch_binding_sha256_v1(preparation),
            preparation_binding_sha256: install_epoch_preparation_binding_sha256_v1(preparation),
            preparation_bundle:
                crate::install_epoch_durable_projection_v1::RawDurableBundleProjectionV1 {
                    binding_sha256: preparation.preparation_bundle_binding_sha256(),
                    id_sha256: preparation.preparation_bundle_id_sha256(),
                },
            predecessor: preparation.predecessor().clone(),
            provider_transaction_sha256: self.pending.commit.provider_transaction_sha256.clone(),
            retry_query_claim: (self.pending.active_query_sequence > 1).then(|| {
                crate::install_epoch_durable_projection_v1::RawDurableNonceClaimProjectionV1 {
                    binding_sha256: self.pending.active_query_claim_binding_sha256.clone(),
                    nonce: self.pending.active_query_nonce.clone(),
                    scope: EXTERNAL_WATERMARK_QUERY_CLAIM_SCOPE_V1.to_string(),
                }
            }),
            state_root_profile_sha256: preparation.state_root_profile().profile_sha256.clone(),
            successor_record: self.successor_record.clone(),
            successor_tip_sha256: self.successor_tip_sha256.clone(),
        }
    }

    pub fn preparation(&self) -> &VerifiedInstallEpochPreparationV1 {
        self.pending.preparation()
    }

    pub fn committed_at_unix_seconds(&self) -> u64 {
        self.pending.commit.committed_at_unix_seconds
    }

    pub fn provider_transaction_sha256(&self) -> &str {
        &self.pending.commit.provider_transaction_sha256
    }

    pub fn commit_statement_sha256(&self) -> &str {
        &self.pending.commit.statement_sha256
    }

    pub fn commit_signature_sha256(&self) -> &str {
        &self.pending.commit.signature_sha256
    }

    pub fn commit_trust_policy_sha256(&self) -> &str {
        &self.pending.commit.trust_policy_sha256
    }

    pub fn commit_nonce(&self) -> &str {
        self.pending.commit_nonce()
    }

    pub fn current_tip_expires_at_unix_seconds(&self) -> u64 {
        self.current_tip_expires_at_unix_seconds
    }

    pub fn current_tip_issued_at_unix_seconds(&self) -> u64 {
        self.current_tip_issued_at_unix_seconds
    }

    pub fn current_tip_statement_sha256(&self) -> &str {
        &self.current_tip_statement_sha256
    }

    pub fn current_tip_signature_sha256(&self) -> &str {
        &self.current_tip_signature_sha256
    }

    pub fn current_tip_trust_policy_sha256(&self) -> &str {
        &self.current_tip_trust_policy_sha256
    }

    pub fn finalized_state_sha256(&self) -> &str {
        &self.finalized_state_sha256
    }

    pub fn final_phase_revision(&self) -> u64 {
        self.final_phase_revision
    }

    pub fn final_phase_was_fresh_model(&self) -> bool {
        self.final_phase_fresh_model
    }

    pub fn requires_read_only_reconciliation_model(&self) -> bool {
        !self.final_phase_fresh_model
    }

    pub fn query_nonce(&self) -> &str {
        &self.query_nonce
    }

    /// Caller-supplied model time used for the final half-open window check.
    /// It is an observation only: it is deliberately excluded from the
    /// durable final-state identity and remains distinct from future
    /// `TrustedTimeV8` evidence.
    pub fn model_completed_at_unix_seconds(&self) -> u64 {
        self.model_completed_at_unix_seconds
    }

    pub fn successor_record(&self) -> &ExternalWatermarkRecordV1 {
        &self.successor_record
    }

    pub fn successor_tip_sha256(&self) -> &str {
        &self.successor_tip_sha256
    }

    pub fn provider_exact_cas_committed_model(&self) -> bool {
        true
    }

    pub fn provider_current_tip_attested_model(&self) -> bool {
        true
    }

    pub fn actual_host_verified(&self) -> bool {
        false
    }

    pub fn trusted_time_verified(&self) -> bool {
        false
    }

    pub fn durable_global_nonce_claimed(&self) -> bool {
        false
    }

    pub fn root_install_execution_allowed(&self) -> bool {
        false
    }

    pub fn daemon_reload_enable_or_start_allowed(&self) -> bool {
        false
    }

    pub fn trusted_state_root_established(&self) -> bool {
        false
    }

    pub fn fresh_attempt_allowed(&self) -> bool {
        false
    }
}

pub fn install_epoch_preparation_binding_sha256_v1(
    preparation: &VerifiedInstallEpochPreparationV1,
) -> String {
    let mut bytes = b"hepta_linux_v8_install_epoch_preparation_binding_v1\0".to_vec();
    append_field(
        &mut bytes,
        "authority_nonce",
        preparation.authority_nonce().as_bytes(),
    );
    append_u64(
        &mut bytes,
        "authority_issued_at_unix_seconds",
        preparation.authority_issued_at_unix_seconds(),
    );
    append_u64(
        &mut bytes,
        "authority_expires_at_unix_seconds",
        preparation.authority_expires_at_unix_seconds(),
    );
    append_field(
        &mut bytes,
        "authority_statement_sha256",
        preparation.authority_statement_sha256().as_bytes(),
    );
    append_field(
        &mut bytes,
        "authority_signature_sha256",
        preparation.authority_signature_sha256().as_bytes(),
    );
    append_field(
        &mut bytes,
        "authority_trust_policy_sha256",
        preparation.authority_trust_policy_sha256().as_bytes(),
    );
    append_field(
        &mut bytes,
        "lease_nonce",
        preparation.lease_nonce().as_bytes(),
    );
    append_u64(
        &mut bytes,
        "lease_issued_at_unix_seconds",
        preparation.lease_issued_at_unix_seconds(),
    );
    append_u64(
        &mut bytes,
        "lease_expires_at_unix_seconds",
        preparation.lease_expires_at_unix_seconds(),
    );
    append_field(
        &mut bytes,
        "lease_statement_sha256",
        preparation.lease_statement_sha256().as_bytes(),
    );
    append_field(
        &mut bytes,
        "lease_signature_sha256",
        preparation.lease_signature_sha256().as_bytes(),
    );
    append_field(
        &mut bytes,
        "lease_trust_policy_sha256",
        preparation.lease_trust_policy_sha256().as_bytes(),
    );
    append_u64(
        &mut bytes,
        "model_verified_at_unix_seconds",
        preparation.model_verified_at_unix_seconds(),
    );
    append_epoch(&mut bytes, preparation.epoch());
    append_predecessor(&mut bytes, preparation.predecessor());
    append_u64(
        &mut bytes,
        "reserved_successor_revision",
        preparation.reserved_successor_revision(),
    );
    append_field(
        &mut bytes,
        "machine_id_sha256",
        preparation.target_host().machine_id_sha256.as_bytes(),
    );
    append_state_root_profile(&mut bytes, preparation.state_root_profile());
    append_inventory(&mut bytes, preparation.install_inventory());
    sha256(&bytes)
}

pub fn prepared_epoch_binding_sha256_v1(preparation: &VerifiedInstallEpochPreparationV1) -> String {
    let mut bytes = b"hepta_linux_v8_prepared_epoch_binding_v1\0".to_vec();
    append_field(
        &mut bytes,
        "preparation_binding_sha256",
        install_epoch_preparation_binding_sha256_v1(preparation).as_bytes(),
    );
    append_epoch(&mut bytes, preparation.epoch());
    append_field(
        &mut bytes,
        "machine_id_sha256",
        preparation.target_host().machine_id_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "state_root_profile_sha256",
        preparation.state_root_profile().profile_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "install_inventory_sha256",
        inventory_sha256(preparation.install_inventory()).as_bytes(),
    );
    sha256(&bytes)
}

pub fn external_watermark_record_sha256_v1(record: &ExternalWatermarkRecordV1) -> String {
    sha256(&canonical_external_watermark_record_v1(record))
}

struct PreparedCompletionIntentBindingsV1 {
    cas_intent_state_sha256: String,
    commit_claim_binding_sha256: String,
    completion_operation_binding_sha256: String,
    completion_slot_id_sha256: String,
    fresh: bool,
    phase_head_id_sha256: String,
    query_claim_binding_sha256: String,
    successor_record: ExternalWatermarkRecordV1,
    successor_tip_sha256: String,
}

pub fn prepare_external_watermark_cas_intent_v1(
    preparation: VerifiedInstallEpochPreparationV1,
    commit_nonce: String,
    initial_query_nonce: String,
    replay_guard: &mut InstallEpochReplayGuardV1,
) -> Result<
    ExternalWatermarkCasIntentOutcomeV1,
    CompletionTransitionErrorV1<VerifiedInstallEpochPreparationV1>,
> {
    let commit_trust =
        match required_frozen_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCommitV1) {
            Ok(value) => value,
            Err(error) => return Err(CompletionTransitionErrorV1::new(error, preparation)),
        };
    let current_tip_trust =
        match required_frozen_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCurrentTipV1)
        {
            Ok(value) => value,
            Err(error) => return Err(CompletionTransitionErrorV1::new(error, preparation)),
        };
    let profile = match required_frozen_completion_profile_v1() {
        Ok(value) => value,
        Err(error) => return Err(CompletionTransitionErrorV1::new(error, preparation)),
    };
    prepare_cas_intent_with_profile_v1(
        preparation,
        commit_nonce,
        initial_query_nonce,
        &commit_trust,
        &current_tip_trust,
        &profile,
        replay_guard,
    )
}

#[cfg(test)]
fn prepare_cas_intent_for_test_v1(
    preparation: VerifiedInstallEpochPreparationV1,
    commit_nonce: String,
    initial_query_nonce: String,
    replay_guard: &mut InstallEpochReplayGuardV1,
) -> Result<
    ExternalWatermarkCasIntentOutcomeV1,
    CompletionTransitionErrorV1<VerifiedInstallEpochPreparationV1>,
> {
    let commit_trust =
        crate::test_only_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCommitV1);
    let current_tip_trust =
        crate::test_only_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCurrentTipV1);
    let profile = test_only_completion_profile_v1(&preparation, &commit_trust, &current_tip_trust);
    prepare_cas_intent_with_profile_v1(
        preparation,
        commit_nonce,
        initial_query_nonce,
        &commit_trust,
        &current_tip_trust,
        &profile,
        replay_guard,
    )
}

fn prepare_cas_intent_with_profile_v1(
    preparation: VerifiedInstallEpochPreparationV1,
    commit_nonce: String,
    initial_query_nonce: String,
    commit_trust: &VerifiedTrustPolicyBindingV8,
    current_tip_trust: &VerifiedTrustPolicyBindingV8,
    profile: &FrozenExternalWatermarkCompletionProfileV1,
    replay_guard: &mut InstallEpochReplayGuardV1,
) -> Result<
    ExternalWatermarkCasIntentOutcomeV1,
    CompletionTransitionErrorV1<VerifiedInstallEpochPreparationV1>,
> {
    let bindings = (|| -> Result<PreparedCompletionIntentBindingsV1, QualificationError> {
        validate_completion_profile_v1(&preparation, commit_trust, current_tip_trust, profile)?;
        for (label, nonce) in [
            ("completion commit nonce", commit_nonce.as_str()),
            (
                "initial current-tip query nonce",
                initial_query_nonce.as_str(),
            ),
        ] {
            validate_digest(label, nonce)?;
        }
        let nonces = [
            preparation.authority_nonce(),
            preparation.lease_nonce(),
            commit_nonce.as_str(),
            initial_query_nonce.as_str(),
        ];
        for left in 0..nonces.len() {
            for right in (left + 1)..nonces.len() {
                if nonces[left] == nonces[right] {
                    return Err(invalid("completion nonces are not pairwise distinct"));
                }
            }
        }
        let authority_claim = preparation.authority_nonce_claim_binding_sha256();
        let lease_claim = preparation.lease_nonce_claim_binding_sha256();
        let preparation_bundle_id = preparation.preparation_bundle_id_sha256();
        let preparation_bundle_binding = preparation.preparation_bundle_binding_sha256();
        let successor_record = expected_external_watermark_record_v1(&preparation, profile);
        let successor_tip_sha256 = external_watermark_record_sha256_v1(&successor_record);
        let completion_slot_id_sha256 = completion_slot_id_sha256_v1(
            preparation.authority_nonce(),
            preparation.lease_nonce(),
            &authority_claim,
            &lease_claim,
        );
        let phase_head_id_sha256 = completion_phase_head_id_sha256_v1(&completion_slot_id_sha256);
        let completion_operation_binding_sha256 = completion_operation_binding_sha256_v1(
            &preparation,
            profile,
            &completion_slot_id_sha256,
            &authority_claim,
            &lease_claim,
            &commit_nonce,
            &initial_query_nonce,
            &successor_record,
            &successor_tip_sha256,
        );
        let commit_claim_binding_sha256 = completion_nonce_claim_binding_sha256_v1(
            EXTERNAL_WATERMARK_COMMIT_CLAIM_SCOPE_V1,
            &commit_nonce,
            &initial_query_nonce,
            &completion_operation_binding_sha256,
        );
        let query_claim_binding_sha256 = completion_nonce_claim_binding_sha256_v1(
            EXTERNAL_WATERMARK_QUERY_CLAIM_SCOPE_V1,
            &initial_query_nonce,
            &commit_nonce,
            &completion_operation_binding_sha256,
        );
        let cas_intent_state_sha256 = cas_intent_state_sha256_v1(
            &completion_operation_binding_sha256,
            &commit_claim_binding_sha256,
            &query_claim_binding_sha256,
        );
        let fresh = replay_guard.claim_pair_bundle_and_phase_or_exact_recovery(
            [
                (
                    preparation.authority_nonce(),
                    INSTALL_EPOCH_AUTHORITY_CLAIM_SCOPE_V1,
                    &authority_claim,
                ),
                (
                    preparation.lease_nonce(),
                    EXTERNAL_WATERMARK_LEASE_CLAIM_SCOPE_V1,
                    &lease_claim,
                ),
            ],
            (&preparation_bundle_id, &preparation_bundle_binding),
            (
                &commit_nonce,
                EXTERNAL_WATERMARK_COMMIT_CLAIM_SCOPE_V1,
                &commit_claim_binding_sha256,
            ),
            (
                &initial_query_nonce,
                EXTERNAL_WATERMARK_QUERY_CLAIM_SCOPE_V1,
                &query_claim_binding_sha256,
            ),
            &completion_slot_id_sha256,
            &cas_intent_state_sha256,
            &phase_head_id_sha256,
            1,
            &cas_intent_state_sha256,
        )?;
        Ok(PreparedCompletionIntentBindingsV1 {
            cas_intent_state_sha256,
            commit_claim_binding_sha256,
            completion_operation_binding_sha256,
            completion_slot_id_sha256,
            fresh,
            phase_head_id_sha256,
            query_claim_binding_sha256,
            successor_record,
            successor_tip_sha256,
        })
    })();
    let bindings = match bindings {
        Ok(value) => value,
        Err(error) => return Err(CompletionTransitionErrorV1::new(error, preparation)),
    };
    let intent = PendingExternalWatermarkCasIntentV1 {
        cas_intent_state_sha256: bindings.cas_intent_state_sha256.clone(),
        commit_claim_binding_sha256: bindings.commit_claim_binding_sha256,
        commit_nonce,
        completion_operation_binding_sha256: bindings.completion_operation_binding_sha256,
        completion_slot_id_sha256: bindings.completion_slot_id_sha256,
        initial_query_nonce,
        preparation,
        phase_state_sha256: bindings.cas_intent_state_sha256,
        profile: profile.clone(),
        phase_revision: 1,
        query_claim_binding_sha256: bindings.query_claim_binding_sha256,
        phase_head_id_sha256: bindings.phase_head_id_sha256,
        successor_record: bindings.successor_record,
        successor_tip_sha256: bindings.successor_tip_sha256,
    };
    if bindings.fresh {
        Ok(ExternalWatermarkCasIntentOutcomeV1::Fresh(
            FreshExternalWatermarkCasIntentV1(intent),
        ))
    } else {
        Ok(ExternalWatermarkCasIntentOutcomeV1::Recovered(
            RecoveredExternalWatermarkCasIntentV1(intent),
        ))
    }
}

pub fn canonical_external_watermark_commit_statement_v1(
    intent: &IssuedExternalWatermarkCasIntentV1,
    challenge: &ExternalWatermarkCommitChallengeV1,
) -> Result<Vec<u8>, QualificationError> {
    let commit_trust =
        required_frozen_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCommitV1)?;
    let current_tip_trust =
        required_frozen_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCurrentTipV1)?;
    canonical_commit_statement_with_profile_v1(
        &intent.0,
        challenge,
        &commit_trust,
        &current_tip_trust,
        &intent.0.profile,
    )
}

pub fn verify_external_watermark_cas_commit_v1(
    intent: IssuedExternalWatermarkCasIntentV1,
    commit: &SignedExternalWatermarkCommitV1,
    replay_guard: &mut InstallEpochReplayGuardV1,
) -> Result<
    VerifiedCasCommittedPendingTipV1,
    CompletionTransitionErrorV1<IssuedExternalWatermarkCasIntentV1>,
> {
    let commit_trust =
        match required_frozen_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCommitV1) {
            Ok(trust) => trust,
            Err(error) => {
                return Err(CompletionTransitionErrorV1::new(error, intent));
            }
        };
    let current_tip_trust =
        match required_frozen_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCurrentTipV1)
        {
            Ok(trust) => trust,
            Err(error) => {
                return Err(CompletionTransitionErrorV1::new(error, intent));
            }
        };
    let statement = match canonical_commit_statement_with_profile_v1(
        &intent.0,
        &commit.challenge,
        &commit_trust,
        &current_tip_trust,
        &intent.0.profile,
    ) {
        Ok(statement) => statement,
        Err(error) => {
            return Err(CompletionTransitionErrorV1::new(error, intent));
        }
    };
    let observation = match verify_statement_sshsig_for_purpose_v8(
        &statement,
        &commit.detached_signature_bytes,
        SshsigTrustPurposeV8::ExternalWatermarkCommitV1,
    ) {
        Ok(observation) => observation,
        Err(error) => {
            return Err(CompletionTransitionErrorV1::new(error, intent));
        }
    };
    verify_cas_commit_with_evidence_v1(
        intent,
        commit,
        &statement,
        &observation,
        commit_trust,
        replay_guard,
    )
}

#[cfg(test)]
fn verify_cas_commit_for_test_v1(
    intent: IssuedExternalWatermarkCasIntentV1,
    commit: &SignedExternalWatermarkCommitV1,
    observation: &CryptographicSignatureObservation,
    replay_guard: &mut InstallEpochReplayGuardV1,
) -> Result<
    VerifiedCasCommittedPendingTipV1,
    CompletionTransitionErrorV1<IssuedExternalWatermarkCasIntentV1>,
> {
    let commit_trust =
        crate::test_only_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCommitV1);
    let current_tip_trust =
        crate::test_only_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCurrentTipV1);
    let statement = match canonical_commit_statement_with_profile_v1(
        &intent.0,
        &commit.challenge,
        &commit_trust,
        &current_tip_trust,
        &intent.0.profile,
    ) {
        Ok(statement) => statement,
        Err(error) => {
            return Err(CompletionTransitionErrorV1::new(error, intent));
        }
    };
    verify_cas_commit_with_evidence_v1(
        intent,
        commit,
        &statement,
        observation,
        commit_trust,
        replay_guard,
    )
}

fn verify_cas_commit_with_evidence_v1(
    issued_intent: IssuedExternalWatermarkCasIntentV1,
    commit: &SignedExternalWatermarkCommitV1,
    statement: &[u8],
    observation: &CryptographicSignatureObservation,
    commit_trust: VerifiedTrustPolicyBindingV8,
    replay_guard: &mut InstallEpochReplayGuardV1,
) -> Result<
    VerifiedCasCommittedPendingTipV1,
    CompletionTransitionErrorV1<IssuedExternalWatermarkCasIntentV1>,
> {
    let verified_commit =
        match verify_commit_envelope_v1(commit, statement, observation, &commit_trust) {
            Ok(commit) => commit,
            Err(error) => {
                return Err(CompletionTransitionErrorV1::new(error, issued_intent));
            }
        };
    let intent = &issued_intent.0;
    let receipt_phase_revision = match intent.phase_revision.checked_add(1) {
        Some(value) => value,
        None => {
            return Err(CompletionTransitionErrorV1::new(
                invalid("CAS receipt phase revision overflows"),
                issued_intent,
            ));
        }
    };
    let cas_receipt_state_sha256 = cas_receipt_state_sha256_v1(
        &intent.phase_state_sha256,
        receipt_phase_revision,
        &verified_commit,
    );
    let initial_query_phase_revision = match receipt_phase_revision.checked_add(1) {
        Some(value) => value,
        None => {
            return Err(CompletionTransitionErrorV1::new(
                invalid("initial current-tip query phase revision overflows"),
                issued_intent,
            ));
        }
    };
    let authority_claim = intent.preparation.authority_nonce_claim_binding_sha256();
    let lease_claim = intent.preparation.lease_nonce_claim_binding_sha256();
    let preparation_bundle_id = intent.preparation.preparation_bundle_id_sha256();
    let preparation_bundle_binding = intent.preparation.preparation_bundle_binding_sha256();
    let required_claims = [
        (
            intent.preparation.authority_nonce(),
            INSTALL_EPOCH_AUTHORITY_CLAIM_SCOPE_V1,
            authority_claim.as_str(),
        ),
        (
            intent.preparation.lease_nonce(),
            EXTERNAL_WATERMARK_LEASE_CLAIM_SCOPE_V1,
            lease_claim.as_str(),
        ),
        (
            intent.commit_nonce.as_str(),
            EXTERNAL_WATERMARK_COMMIT_CLAIM_SCOPE_V1,
            intent.commit_claim_binding_sha256.as_str(),
        ),
        (
            intent.initial_query_nonce.as_str(),
            EXTERNAL_WATERMARK_QUERY_CLAIM_SCOPE_V1,
            intent.query_claim_binding_sha256.as_str(),
        ),
    ];
    let required_bundles = [
        (
            preparation_bundle_id.as_str(),
            preparation_bundle_binding.as_str(),
        ),
        (
            intent.completion_slot_id_sha256.as_str(),
            intent.cas_intent_state_sha256.as_str(),
        ),
    ];
    let _phase_fresh = match replay_guard.advance_phase_or_exact_recovery(
        &required_claims,
        &required_bundles,
        &intent.phase_head_id_sha256,
        intent.phase_revision,
        &intent.phase_state_sha256,
        receipt_phase_revision,
        &cas_receipt_state_sha256,
    ) {
        Ok(value) => value,
        Err(error) => return Err(CompletionTransitionErrorV1::new(error, issued_intent)),
    };
    let active_query_state_sha256 = current_tip_query_state_sha256_v1(
        &cas_receipt_state_sha256,
        initial_query_phase_revision,
        1,
        &intent.query_claim_binding_sha256,
        None,
    );
    Ok(VerifiedCasCommittedPendingTipV1 {
        active_query_claim_binding_sha256: intent.query_claim_binding_sha256.clone(),
        // Any exact verified receipt may compete to reserve the next query;
        // the durable rev3 -> rev4 edge, not receipt freshness, selects the
        // sole provider-I/O winner.
        active_query_may_issue_model: true,
        active_query_nonce: intent.initial_query_nonce.clone(),
        active_query_phase_durable: false,
        active_query_sequence: 1,
        active_query_state_sha256,
        cas_receipt_state_sha256,
        commit: verified_commit,
        intent: issued_intent.0,
        phase_revision: receipt_phase_revision,
    })
}

pub fn canonical_external_watermark_current_tip_statement_v1(
    issued: &IssuedExternalWatermarkCurrentTipQueryV1,
    challenge: &ExternalWatermarkCurrentTipChallengeV1,
) -> Result<Vec<u8>, QualificationError> {
    let commit_trust =
        required_frozen_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCommitV1)?;
    let current_tip_trust =
        required_frozen_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCurrentTipV1)?;
    canonical_current_tip_statement_with_profile_v1(
        &issued.0,
        challenge,
        &commit_trust,
        &current_tip_trust,
        &issued.0.intent.profile,
    )
}

pub fn verify_external_watermark_current_tip_v1(
    issued: IssuedExternalWatermarkCurrentTipQueryV1,
    current_tip: &SignedExternalWatermarkCurrentTipV1,
    model_now_unix_seconds: u64,
    replay_guard: &mut InstallEpochReplayGuardV1,
) -> Result<
    VerifiedCommittedCurrentTipPreparationV1,
    CompletionTransitionErrorV1<IssuedExternalWatermarkCurrentTipQueryV1>,
> {
    let commit_trust =
        match required_frozen_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCommitV1) {
            Ok(trust) => trust,
            Err(error) => return Err(CompletionTransitionErrorV1::new(error, issued)),
        };
    let current_tip_trust =
        match required_frozen_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCurrentTipV1)
        {
            Ok(trust) => trust,
            Err(error) => return Err(CompletionTransitionErrorV1::new(error, issued)),
        };
    let statement = match canonical_current_tip_statement_with_profile_v1(
        &issued.0,
        &current_tip.challenge,
        &commit_trust,
        &current_tip_trust,
        &issued.0.intent.profile,
    ) {
        Ok(statement) => statement,
        Err(error) => return Err(CompletionTransitionErrorV1::new(error, issued)),
    };
    let observation = match verify_statement_sshsig_for_purpose_v8(
        &statement,
        &current_tip.detached_signature_bytes,
        SshsigTrustPurposeV8::ExternalWatermarkCurrentTipV1,
    ) {
        Ok(observation) => observation,
        Err(error) => return Err(CompletionTransitionErrorV1::new(error, issued)),
    };
    verify_current_tip_with_evidence_v1(
        issued,
        current_tip,
        &statement,
        &observation,
        current_tip_trust,
        model_now_unix_seconds,
        replay_guard,
    )
}

#[cfg(test)]
fn verify_current_tip_for_test_v1(
    issued: IssuedExternalWatermarkCurrentTipQueryV1,
    current_tip: &SignedExternalWatermarkCurrentTipV1,
    observation: &CryptographicSignatureObservation,
    model_now_unix_seconds: u64,
    replay_guard: &mut InstallEpochReplayGuardV1,
) -> Result<
    VerifiedCommittedCurrentTipPreparationV1,
    CompletionTransitionErrorV1<IssuedExternalWatermarkCurrentTipQueryV1>,
> {
    let commit_trust =
        crate::test_only_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCommitV1);
    let current_tip_trust =
        crate::test_only_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCurrentTipV1);
    let statement = match canonical_current_tip_statement_with_profile_v1(
        &issued.0,
        &current_tip.challenge,
        &commit_trust,
        &current_tip_trust,
        &issued.0.intent.profile,
    ) {
        Ok(statement) => statement,
        Err(error) => return Err(CompletionTransitionErrorV1::new(error, issued)),
    };
    verify_current_tip_with_evidence_v1(
        issued,
        current_tip,
        &statement,
        observation,
        current_tip_trust,
        model_now_unix_seconds,
        replay_guard,
    )
}

fn verify_current_tip_with_evidence_v1(
    issued: IssuedExternalWatermarkCurrentTipQueryV1,
    current_tip: &SignedExternalWatermarkCurrentTipV1,
    statement: &[u8],
    observation: &CryptographicSignatureObservation,
    current_tip_trust: VerifiedTrustPolicyBindingV8,
    model_now_unix_seconds: u64,
    replay_guard: &mut InstallEpochReplayGuardV1,
) -> Result<
    VerifiedCommittedCurrentTipPreparationV1,
    CompletionTransitionErrorV1<IssuedExternalWatermarkCurrentTipQueryV1>,
> {
    let pending = issued.0;
    let result = (|| {
        if !pending.active_query_phase_durable || pending.active_query_may_issue_model {
            return Err(invalid(
                "current-tip verification requires a durable, consumed query phase",
            ));
        }
        let current_tip_statement_sha256 = sha256(statement);
        let current_tip_signature_sha256 = validate_signed_envelope(
            &current_tip.canonical_statement_sha256,
            statement,
            &current_tip.detached_signature_sha256,
            &current_tip.detached_signature_bytes,
            observation,
            &current_tip_trust,
        )?;
        validate_model_time_window(
            current_tip.challenge.issued_at_unix_seconds,
            current_tip.challenge.expires_at_unix_seconds,
            pending.intent.profile.maximum_current_tip_lifetime_seconds,
            model_now_unix_seconds,
        )?;
        Ok((current_tip_statement_sha256, current_tip_signature_sha256))
    })();
    let (current_tip_statement_sha256, current_tip_signature_sha256) = match result {
        Ok(value) => value,
        Err(error) => {
            return Err(CompletionTransitionErrorV1::new(
                error,
                IssuedExternalWatermarkCurrentTipQueryV1(pending),
            ));
        }
    };
    let final_phase_revision = match pending.phase_revision.checked_add(1) {
        Some(value) => value,
        None => {
            return Err(CompletionTransitionErrorV1::new(
                invalid("current-tip final phase revision overflows"),
                IssuedExternalWatermarkCurrentTipQueryV1(pending),
            ));
        }
    };
    let finalized_state_sha256 = current_tip_finalized_state_sha256_v1(
        &pending,
        current_tip,
        final_phase_revision,
        &current_tip_statement_sha256,
        &current_tip_signature_sha256,
        current_tip_trust.policy_sha256(),
    );
    let authority_claim = pending.preparation().authority_nonce_claim_binding_sha256();
    let lease_claim = pending.preparation().lease_nonce_claim_binding_sha256();
    let preparation_bundle_id = pending.preparation().preparation_bundle_id_sha256();
    let preparation_bundle_binding = pending.preparation().preparation_bundle_binding_sha256();
    let required_claims = [
        (
            pending.preparation().authority_nonce(),
            INSTALL_EPOCH_AUTHORITY_CLAIM_SCOPE_V1,
            authority_claim.as_str(),
        ),
        (
            pending.preparation().lease_nonce(),
            EXTERNAL_WATERMARK_LEASE_CLAIM_SCOPE_V1,
            lease_claim.as_str(),
        ),
        (
            pending.commit_nonce(),
            EXTERNAL_WATERMARK_COMMIT_CLAIM_SCOPE_V1,
            pending.intent.commit_claim_binding_sha256.as_str(),
        ),
        (
            pending.active_query_nonce(),
            EXTERNAL_WATERMARK_QUERY_CLAIM_SCOPE_V1,
            pending.active_query_claim_binding_sha256.as_str(),
        ),
    ];
    let mut required_bundles = vec![
        (
            preparation_bundle_id.as_str(),
            preparation_bundle_binding.as_str(),
        ),
        (
            pending.intent.completion_slot_id_sha256.as_str(),
            pending.intent.cas_intent_state_sha256.as_str(),
        ),
    ];
    if pending.active_query_sequence > 1 {
        required_bundles.push((
            pending.active_query_claim_binding_sha256.as_str(),
            pending.active_query_state_sha256.as_str(),
        ));
    }
    let final_phase_fresh_model = match replay_guard.advance_phase_or_exact_recovery(
        &required_claims,
        &required_bundles,
        &pending.intent.phase_head_id_sha256,
        pending.phase_revision,
        &pending.active_query_state_sha256,
        final_phase_revision,
        &finalized_state_sha256,
    ) {
        Ok(value) => value,
        Err(error) => {
            return Err(CompletionTransitionErrorV1::new(
                error,
                IssuedExternalWatermarkCurrentTipQueryV1(pending),
            ));
        }
    };
    Ok(VerifiedCommittedCurrentTipPreparationV1 {
        current_tip_expires_at_unix_seconds: current_tip.challenge.expires_at_unix_seconds,
        current_tip_issued_at_unix_seconds: current_tip.challenge.issued_at_unix_seconds,
        current_tip_signature_sha256,
        current_tip_statement_sha256,
        current_tip_trust_policy_sha256: current_tip_trust.policy_sha256().to_string(),
        final_phase_fresh_model,
        final_phase_revision,
        finalized_state_sha256,
        model_completed_at_unix_seconds: model_now_unix_seconds,
        pending,
        query_nonce: current_tip.challenge.query_nonce.clone(),
        successor_record: current_tip.challenge.current_record.clone(),
        successor_tip_sha256: current_tip.challenge.current_tip_sha256.clone(),
    })
}

fn verify_commit_envelope_v1(
    commit: &SignedExternalWatermarkCommitV1,
    statement: &[u8],
    observation: &CryptographicSignatureObservation,
    trust: &VerifiedTrustPolicyBindingV8,
) -> Result<VerifiedProviderCasCommitV1, QualificationError> {
    let statement_sha256 = sha256(statement);
    let signature_sha256 = validate_signed_envelope(
        &commit.canonical_statement_sha256,
        statement,
        &commit.detached_signature_sha256,
        &commit.detached_signature_bytes,
        observation,
        trust,
    )?;
    Ok(VerifiedProviderCasCommitV1 {
        committed_at_unix_seconds: commit.challenge.committed_at_unix_seconds,
        provider_transaction_sha256: commit.challenge.provider_transaction_sha256.clone(),
        signature_sha256,
        statement_sha256,
        trust_policy_sha256: trust.policy_sha256().to_string(),
    })
}

fn canonical_commit_statement_with_profile_v1(
    intent: &PendingExternalWatermarkCasIntentV1,
    challenge: &ExternalWatermarkCommitChallengeV1,
    commit_trust: &VerifiedTrustPolicyBindingV8,
    current_tip_trust: &VerifiedTrustPolicyBindingV8,
    profile: &FrozenExternalWatermarkCompletionProfileV1,
) -> Result<Vec<u8>, QualificationError> {
    let preparation = &intent.preparation;
    validate_completion_profile_v1(preparation, commit_trust, current_tip_trust, profile)?;
    if *profile != intent.profile {
        return Err(invalid(
            "external watermark commit profile differs from the durable intent",
        ));
    }
    let expected_record = &intent.successor_record;
    let expected_preparation = install_epoch_preparation_binding_sha256_v1(preparation);
    let expected_tip = &intent.successor_tip_sha256;
    if challenge.schema != EXTERNAL_WATERMARK_COMMIT_SCHEMA_V1
        || challenge.namespace != EXTERNAL_WATERMARK_COMMIT_NAMESPACE_V1
        || challenge.capability != profile.commit_capability
        || commit_trust.purpose() != SshsigTrustPurposeV8::ExternalWatermarkCommitV1
        || commit_trust.namespace() != EXTERNAL_WATERMARK_COMMIT_NAMESPACE_V1
        || !signer_matches_trust(&challenge.signer, commit_trust)?
    {
        return Err(invalid(
            "external watermark commit family or signer is not exact",
        ));
    }
    for (label, value) in [
        ("commit nonce", challenge.commit_nonce.as_str()),
        (
            "completion operation binding",
            challenge.completion_operation_binding_sha256.as_str(),
        ),
        (
            "provider transaction",
            challenge.provider_transaction_sha256.as_str(),
        ),
        (
            "completion profile",
            challenge.completion_profile_sha256.as_str(),
        ),
        (
            "preparation binding",
            challenge.preparation_binding_sha256.as_str(),
        ),
        ("successor tip", challenge.successor_tip_sha256.as_str()),
    ] {
        validate_digest(label, value)?;
    }
    if challenge.authority_nonce != preparation.authority_nonce()
        || challenge.commit_nonce != intent.commit_nonce
        || challenge.completion_operation_binding_sha256
            != intent.completion_operation_binding_sha256
        || challenge.lease_nonce != preparation.lease_nonce()
        || challenge.lease_statement_sha256 != preparation.lease_statement_sha256()
        || challenge.lease_signature_sha256 != preparation.lease_signature_sha256()
        || challenge.lease_trust_policy_sha256 != preparation.lease_trust_policy_sha256()
        || challenge.completion_profile_sha256 != profile.profile_sha256
        || challenge.preparation_binding_sha256 != expected_preparation
        || challenge.predecessor != *preparation.predecessor()
        || challenge.successor_record != *expected_record
        || challenge.successor_tip_sha256 != *expected_tip
    {
        return Err(invalid(
            "external watermark commit does not bind the exact preparation",
        ));
    }
    let committed = challenge.committed_at_unix_seconds;
    if committed < preparation.model_verified_at_unix_seconds()
        || committed < preparation.authority_issued_at_unix_seconds()
        || committed >= preparation.authority_expires_at_unix_seconds()
        || committed < preparation.lease_issued_at_unix_seconds()
        || committed >= preparation.lease_expires_at_unix_seconds()
    {
        return Err(invalid(
            "provider CAS commit is outside the verified preparation windows",
        ));
    }
    let mut statement = b"hepta_linux_v8_external_watermark_commit_statement_v1\0".to_vec();
    append_field(&mut statement, "schema", challenge.schema.as_bytes());
    append_field(&mut statement, "namespace", challenge.namespace.as_bytes());
    append_signer(&mut statement, &challenge.signer);
    append_field(
        &mut statement,
        "capability",
        b"exact_idempotent_cas_by_commit_nonce_and_consume_lease_once",
    );
    append_field(
        &mut statement,
        "authority_nonce",
        challenge.authority_nonce.as_bytes(),
    );
    append_field(
        &mut statement,
        "lease_nonce",
        challenge.lease_nonce.as_bytes(),
    );
    append_field(
        &mut statement,
        "commit_nonce",
        challenge.commit_nonce.as_bytes(),
    );
    append_u64(&mut statement, "committed_at_unix_seconds", committed);
    append_field(
        &mut statement,
        "completion_operation_binding_sha256",
        challenge.completion_operation_binding_sha256.as_bytes(),
    );
    append_field(
        &mut statement,
        "completion_profile_sha256",
        challenge.completion_profile_sha256.as_bytes(),
    );
    append_field(
        &mut statement,
        "lease_statement_sha256",
        challenge.lease_statement_sha256.as_bytes(),
    );
    append_field(
        &mut statement,
        "lease_signature_sha256",
        challenge.lease_signature_sha256.as_bytes(),
    );
    append_field(
        &mut statement,
        "lease_trust_policy_sha256",
        challenge.lease_trust_policy_sha256.as_bytes(),
    );
    append_field(
        &mut statement,
        "preparation_binding_sha256",
        challenge.preparation_binding_sha256.as_bytes(),
    );
    append_field(
        &mut statement,
        "provider_transaction_sha256",
        challenge.provider_transaction_sha256.as_bytes(),
    );
    append_predecessor(&mut statement, &challenge.predecessor);
    append_record(&mut statement, &challenge.successor_record);
    append_field(
        &mut statement,
        "successor_tip_sha256",
        challenge.successor_tip_sha256.as_bytes(),
    );
    Ok(statement)
}

fn canonical_current_tip_statement_with_profile_v1(
    pending: &VerifiedCasCommittedPendingTipV1,
    challenge: &ExternalWatermarkCurrentTipChallengeV1,
    commit_trust: &VerifiedTrustPolicyBindingV8,
    current_tip_trust: &VerifiedTrustPolicyBindingV8,
    profile: &FrozenExternalWatermarkCompletionProfileV1,
) -> Result<Vec<u8>, QualificationError> {
    let preparation = &pending.intent.preparation;
    validate_completion_profile_v1(preparation, commit_trust, current_tip_trust, profile)?;
    if *profile != pending.intent.profile {
        return Err(invalid(
            "external current-tip profile differs from the durable intent",
        ));
    }
    let expected_record = &pending.intent.successor_record;
    let expected_tip = &pending.intent.successor_tip_sha256;
    let expected_preparation = install_epoch_preparation_binding_sha256_v1(preparation);
    let commit_statement_sha256 = &pending.commit.statement_sha256;
    let commit_signature_sha256 = &pending.commit.signature_sha256;
    if challenge.schema != EXTERNAL_WATERMARK_CURRENT_TIP_SCHEMA_V1
        || challenge.namespace != EXTERNAL_WATERMARK_CURRENT_TIP_NAMESPACE_V1
        || pending.active_query_may_issue_model
        || !pending.active_query_phase_durable
        || challenge.capability != profile.current_tip_capability
        || current_tip_trust.purpose() != SshsigTrustPurposeV8::ExternalWatermarkCurrentTipV1
        || current_tip_trust.namespace() != EXTERNAL_WATERMARK_CURRENT_TIP_NAMESPACE_V1
        || !signer_matches_trust(&challenge.signer, current_tip_trust)?
    {
        return Err(invalid(
            "external current-tip family or signer is not exact",
        ));
    }
    for (label, value) in [
        ("query nonce", challenge.query_nonce.as_str()),
        (
            "commit statement",
            challenge.commit_statement_sha256.as_str(),
        ),
        (
            "commit signature",
            challenge.commit_signature_sha256.as_str(),
        ),
        (
            "commit trust policy",
            challenge.commit_trust_policy_sha256.as_str(),
        ),
        (
            "completion profile",
            challenge.completion_profile_sha256.as_str(),
        ),
        (
            "preparation binding",
            challenge.preparation_binding_sha256.as_str(),
        ),
        (
            "provider transaction",
            challenge.provider_transaction_sha256.as_str(),
        ),
        ("current tip", challenge.current_tip_sha256.as_str()),
        ("current stream", challenge.stream_id_sha256.as_str()),
    ] {
        validate_digest(label, value)?;
    }
    if challenge.completion_profile_sha256 != profile.profile_sha256
        || challenge.query_nonce != pending.active_query_nonce
        || challenge.commit_statement_sha256 != *commit_statement_sha256
        || challenge.commit_signature_sha256 != *commit_signature_sha256
        || challenge.commit_trust_policy_sha256 != commit_trust.policy_sha256()
        || challenge.preparation_binding_sha256 != expected_preparation
        || challenge.provider_transaction_sha256 != pending.commit.provider_transaction_sha256
        || challenge.current_record != *expected_record
        || challenge.current_revision != expected_record.successor_revision
        || challenge.current_tip_sha256 != *expected_tip
        || challenge.stream_id_sha256 != expected_record.stream_id_sha256
        || challenge.target_host != *preparation.target_host()
    {
        return Err(invalid(
            "external current tip does not equal the exact committed successor",
        ));
    }
    if challenge.issued_at_unix_seconds < pending.commit.committed_at_unix_seconds {
        return Err(invalid(
            "external current-tip statement predates the CAS commit",
        ));
    }
    validate_interval_shape(
        challenge.issued_at_unix_seconds,
        challenge.expires_at_unix_seconds,
        profile.maximum_current_tip_lifetime_seconds,
    )?;
    let mut statement = b"hepta_linux_v8_external_watermark_current_tip_statement_v1\0".to_vec();
    append_field(&mut statement, "schema", challenge.schema.as_bytes());
    append_field(&mut statement, "namespace", challenge.namespace.as_bytes());
    append_signer(&mut statement, &challenge.signer);
    append_field(
        &mut statement,
        "capability",
        b"exact_idempotent_authenticated_current_tip_by_query_nonce_after_commit",
    );
    append_field(
        &mut statement,
        "query_nonce",
        challenge.query_nonce.as_bytes(),
    );
    append_u64(
        &mut statement,
        "issued_at_unix_seconds",
        challenge.issued_at_unix_seconds,
    );
    append_u64(
        &mut statement,
        "expires_at_unix_seconds",
        challenge.expires_at_unix_seconds,
    );
    append_field(
        &mut statement,
        "completion_profile_sha256",
        challenge.completion_profile_sha256.as_bytes(),
    );
    append_field(
        &mut statement,
        "commit_statement_sha256",
        challenge.commit_statement_sha256.as_bytes(),
    );
    append_field(
        &mut statement,
        "commit_signature_sha256",
        challenge.commit_signature_sha256.as_bytes(),
    );
    append_field(
        &mut statement,
        "commit_trust_policy_sha256",
        challenge.commit_trust_policy_sha256.as_bytes(),
    );
    append_field(
        &mut statement,
        "preparation_binding_sha256",
        challenge.preparation_binding_sha256.as_bytes(),
    );
    append_field(
        &mut statement,
        "provider_transaction_sha256",
        challenge.provider_transaction_sha256.as_bytes(),
    );
    append_field(
        &mut statement,
        "stream_id_sha256",
        challenge.stream_id_sha256.as_bytes(),
    );
    append_u64(
        &mut statement,
        "current_revision",
        challenge.current_revision,
    );
    append_record(&mut statement, &challenge.current_record);
    append_field(
        &mut statement,
        "current_tip_sha256",
        challenge.current_tip_sha256.as_bytes(),
    );
    append_field(
        &mut statement,
        "machine_id_sha256",
        challenge.target_host.machine_id_sha256.as_bytes(),
    );
    Ok(statement)
}

fn validate_completion_profile_v1(
    preparation: &VerifiedInstallEpochPreparationV1,
    commit_trust: &VerifiedTrustPolicyBindingV8,
    current_tip_trust: &VerifiedTrustPolicyBindingV8,
    profile: &FrozenExternalWatermarkCompletionProfileV1,
) -> Result<(), QualificationError> {
    if profile.profile_id != EXTERNAL_WATERMARK_COMPLETION_PROFILE_ID_V1
        || profile.profile_revision == 0
        || profile.commit_capability
            != ExternalWatermarkCommitCapabilityV1::ExactIdempotentCasByCommitNonceAndConsumeLeaseOnce
        || profile.current_tip_capability
            != ExternalWatermarkCurrentTipCapabilityV1::ExactIdempotentAuthenticatedCurrentTipByQueryNonceAfterCommit
        || profile.maximum_current_tip_lifetime_seconds
            != MAX_EXTERNAL_WATERMARK_CURRENT_TIP_LIFETIME_SECONDS_V1
        || profile.maximum_current_tip_retry_count
            != MAX_EXTERNAL_WATERMARK_CURRENT_TIP_RETRY_COUNT_V1
        || profile.profile_sha256 != completion_profile_sha256(profile)
        || profile.provider_profile_sha256
            != predecessor_provider_profile_sha256(preparation.predecessor())
        || profile.lease_trust_policy_sha256 != preparation.lease_trust_policy_sha256()
        || profile.commit_trust_policy_sha256 != commit_trust.policy_sha256()
        || profile.current_tip_trust_policy_sha256 != current_tip_trust.policy_sha256()
        || profile.trust_root_id != commit_trust.trust_root_id()
        || profile.trust_root_id != current_tip_trust.trust_root_id()
        || profile.key_fingerprint != commit_trust.key_fingerprint()
        || profile.key_fingerprint != current_tip_trust.key_fingerprint()
        || profile.lease_trust_policy_sha256 == profile.commit_trust_policy_sha256
        || profile.lease_trust_policy_sha256 == profile.current_tip_trust_policy_sha256
        || commit_trust.policy_sha256() == current_tip_trust.policy_sha256()
    {
        return Err(invalid(
            "external watermark completion profile is not exact",
        ));
    }
    Ok(())
}

fn expected_external_watermark_record_v1(
    preparation: &VerifiedInstallEpochPreparationV1,
    profile: &FrozenExternalWatermarkCompletionProfileV1,
) -> ExternalWatermarkRecordV1 {
    ExternalWatermarkRecordV1 {
        completion_profile_sha256: profile.profile_sha256.clone(),
        prepared_epoch_binding_sha256: prepared_epoch_binding_sha256_v1(preparation),
        machine_id_sha256: preparation.target_host().machine_id_sha256.clone(),
        predecessor: preparation.predecessor().clone(),
        preparation_binding_sha256: install_epoch_preparation_binding_sha256_v1(preparation),
        provider_profile_sha256: profile.provider_profile_sha256.clone(),
        state_root_profile_sha256: preparation.state_root_profile().profile_sha256.clone(),
        stream_id_sha256: predecessor_stream_id_sha256(preparation.predecessor()).to_string(),
        successor_revision: preparation.reserved_successor_revision(),
    }
}

fn completion_slot_id_sha256_v1(
    authority_nonce: &str,
    lease_nonce: &str,
    authority_claim_sha256: &str,
    lease_claim_sha256: &str,
) -> String {
    let mut bytes = b"hepta_linux_v8_external_watermark_completion_slot_v1\0".to_vec();
    append_field(
        &mut bytes,
        "claim_domain_id",
        EXTERNAL_WATERMARK_COMPLETION_CLAIM_DOMAIN_ID_V1.as_bytes(),
    );
    append_field(&mut bytes, "authority_nonce", authority_nonce.as_bytes());
    append_field(&mut bytes, "lease_nonce", lease_nonce.as_bytes());
    append_field(
        &mut bytes,
        "authority_claim_sha256",
        authority_claim_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "lease_claim_sha256",
        lease_claim_sha256.as_bytes(),
    );
    sha256(&bytes)
}

fn completion_phase_head_id_sha256_v1(completion_slot_id_sha256: &str) -> String {
    let mut bytes = b"hepta_linux_v8_external_watermark_completion_phase_head_v1\0".to_vec();
    append_field(
        &mut bytes,
        "completion_slot_id_sha256",
        completion_slot_id_sha256.as_bytes(),
    );
    sha256(&bytes)
}

#[allow(clippy::too_many_arguments)]
fn completion_operation_binding_sha256_v1(
    preparation: &VerifiedInstallEpochPreparationV1,
    profile: &FrozenExternalWatermarkCompletionProfileV1,
    completion_slot_id_sha256: &str,
    authority_claim_sha256: &str,
    lease_claim_sha256: &str,
    commit_nonce: &str,
    initial_query_nonce: &str,
    successor_record: &ExternalWatermarkRecordV1,
    successor_tip_sha256: &str,
) -> String {
    let mut bytes = b"hepta_linux_v8_external_watermark_completion_operation_v1\0".to_vec();
    append_field(
        &mut bytes,
        "claim_domain_id",
        EXTERNAL_WATERMARK_COMPLETION_CLAIM_DOMAIN_ID_V1.as_bytes(),
    );
    append_field(
        &mut bytes,
        "completion_slot_id_sha256",
        completion_slot_id_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "preparation_binding_sha256",
        install_epoch_preparation_binding_sha256_v1(preparation).as_bytes(),
    );
    append_field(
        &mut bytes,
        "machine_id_sha256",
        preparation.target_host().machine_id_sha256.as_bytes(),
    );
    append_epoch(&mut bytes, preparation.epoch());
    append_field(
        &mut bytes,
        "authority_claim_sha256",
        authority_claim_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "lease_claim_sha256",
        lease_claim_sha256.as_bytes(),
    );
    append_field(&mut bytes, "commit_nonce", commit_nonce.as_bytes());
    append_field(
        &mut bytes,
        "initial_query_nonce",
        initial_query_nonce.as_bytes(),
    );
    append_field(
        &mut bytes,
        "commit_namespace",
        EXTERNAL_WATERMARK_COMMIT_NAMESPACE_V1.as_bytes(),
    );
    append_field(
        &mut bytes,
        "query_namespace",
        EXTERNAL_WATERMARK_CURRENT_TIP_NAMESPACE_V1.as_bytes(),
    );
    append_field(
        &mut bytes,
        "completion_profile_sha256",
        profile.profile_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "provider_profile_sha256",
        profile.provider_profile_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "stream_id_sha256",
        predecessor_stream_id_sha256(preparation.predecessor()).as_bytes(),
    );
    append_predecessor(&mut bytes, preparation.predecessor());
    append_u64(
        &mut bytes,
        "reserved_successor_revision",
        preparation.reserved_successor_revision(),
    );
    append_record(&mut bytes, successor_record);
    append_field(
        &mut bytes,
        "successor_tip_sha256",
        successor_tip_sha256.as_bytes(),
    );
    sha256(&bytes)
}

fn completion_nonce_claim_binding_sha256_v1(
    scope: &str,
    nonce: &str,
    counterpart_nonce: &str,
    completion_operation_binding_sha256: &str,
) -> String {
    let mut bytes = b"hepta_linux_v8_external_watermark_completion_nonce_claim_v1\0".to_vec();
    append_field(
        &mut bytes,
        "claim_domain_id",
        EXTERNAL_WATERMARK_COMPLETION_CLAIM_DOMAIN_ID_V1.as_bytes(),
    );
    append_field(&mut bytes, "scope", scope.as_bytes());
    append_field(&mut bytes, "nonce", nonce.as_bytes());
    append_field(
        &mut bytes,
        "counterpart_nonce",
        counterpart_nonce.as_bytes(),
    );
    append_field(
        &mut bytes,
        "completion_operation_binding_sha256",
        completion_operation_binding_sha256.as_bytes(),
    );
    sha256(&bytes)
}

fn completion_retry_nonce_claim_binding_sha256_v1(
    nonce: &str,
    predecessor_query_nonce: &str,
    commit_nonce: &str,
    completion_operation_binding_sha256: &str,
    predecessor_query_closure_sha256: &str,
) -> String {
    let mut bytes = b"hepta_linux_v8_external_watermark_completion_retry_claim_v1\0".to_vec();
    append_field(
        &mut bytes,
        "claim_domain_id",
        EXTERNAL_WATERMARK_COMPLETION_CLAIM_DOMAIN_ID_V1.as_bytes(),
    );
    append_field(
        &mut bytes,
        "scope",
        EXTERNAL_WATERMARK_QUERY_CLAIM_SCOPE_V1.as_bytes(),
    );
    append_field(&mut bytes, "nonce", nonce.as_bytes());
    append_field(
        &mut bytes,
        "predecessor_query_nonce",
        predecessor_query_nonce.as_bytes(),
    );
    append_field(&mut bytes, "commit_nonce", commit_nonce.as_bytes());
    append_field(
        &mut bytes,
        "completion_operation_binding_sha256",
        completion_operation_binding_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "predecessor_query_closure_sha256",
        predecessor_query_closure_sha256.as_bytes(),
    );
    sha256(&bytes)
}

fn query_closure_binding_sha256_v1(
    closure: &VerifiedExternalWatermarkCurrentTipQueryClosureV1,
) -> String {
    let mut bytes = b"hepta_linux_v8_external_watermark_query_closure_v1\0".to_vec();
    append_field(
        &mut bytes,
        "closure_profile_sha256",
        closure.closure_profile_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "closure_evidence_sha256",
        closure.closure_evidence_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "completion_operation_binding_sha256",
        closure.completion_operation_binding_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "phase_head_id_sha256",
        closure.phase_head_id_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "provider_transaction_sha256",
        closure.provider_transaction_sha256.as_bytes(),
    );
    append_field(&mut bytes, "query_nonce", closure.query_nonce.as_bytes());
    append_field(
        &mut bytes,
        "query_claim_binding_sha256",
        closure.query_claim_binding_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "query_state_sha256",
        closure.query_state_sha256.as_bytes(),
    );
    append_u64(&mut bytes, "query_sequence", closure.query_sequence);
    append_u64(
        &mut bytes,
        "query_phase_revision",
        closure.query_phase_revision,
    );
    sha256(&bytes)
}

fn cas_intent_state_sha256_v1(
    operation_binding_sha256: &str,
    commit_claim_binding_sha256: &str,
    query_claim_binding_sha256: &str,
) -> String {
    let mut bytes = b"hepta_linux_v8_external_watermark_completion_phase_v1\0".to_vec();
    append_u64(&mut bytes, "phase_revision", 1);
    append_field(
        &mut bytes,
        "operation_binding_sha256",
        operation_binding_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "commit_claim_binding_sha256",
        commit_claim_binding_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "query_claim_binding_sha256",
        query_claim_binding_sha256.as_bytes(),
    );
    sha256(&bytes)
}

fn cas_issue_state_sha256_v1(
    prior_state_sha256: &str,
    phase_revision: u64,
    completion_operation_binding_sha256: &str,
) -> String {
    let mut bytes = b"hepta_linux_v8_external_watermark_completion_phase_v1\0".to_vec();
    append_u64(&mut bytes, "phase_revision", phase_revision);
    append_field(
        &mut bytes,
        "prior_state_sha256",
        prior_state_sha256.as_bytes(),
    );
    append_field(&mut bytes, "transition", b"provider_cas_issue_reserved");
    append_field(
        &mut bytes,
        "completion_operation_binding_sha256",
        completion_operation_binding_sha256.as_bytes(),
    );
    sha256(&bytes)
}

fn cas_receipt_state_sha256_v1(
    prior_state_sha256: &str,
    phase_revision: u64,
    commit: &VerifiedProviderCasCommitV1,
) -> String {
    let mut bytes = b"hepta_linux_v8_external_watermark_completion_phase_v1\0".to_vec();
    append_u64(&mut bytes, "phase_revision", phase_revision);
    append_field(
        &mut bytes,
        "prior_state_sha256",
        prior_state_sha256.as_bytes(),
    );
    append_u64(
        &mut bytes,
        "committed_at_unix_seconds",
        commit.committed_at_unix_seconds,
    );
    append_field(
        &mut bytes,
        "provider_transaction_sha256",
        commit.provider_transaction_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "commit_statement_sha256",
        commit.statement_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "commit_signature_sha256",
        commit.signature_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "commit_trust_policy_sha256",
        commit.trust_policy_sha256.as_bytes(),
    );
    sha256(&bytes)
}

fn current_tip_query_state_sha256_v1(
    prior_state_sha256: &str,
    phase_revision: u64,
    query_sequence: u64,
    query_claim_binding_sha256: &str,
    predecessor_query_closure_sha256: Option<&str>,
) -> String {
    let mut bytes = b"hepta_linux_v8_external_watermark_completion_query_state_v1\0".to_vec();
    append_u64(&mut bytes, "phase_revision", phase_revision);
    append_u64(&mut bytes, "query_sequence", query_sequence);
    append_field(
        &mut bytes,
        "prior_state_sha256",
        prior_state_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "query_claim_binding_sha256",
        query_claim_binding_sha256.as_bytes(),
    );
    match predecessor_query_closure_sha256 {
        Some(binding) => append_field(
            &mut bytes,
            "predecessor_query_closure_sha256",
            binding.as_bytes(),
        ),
        None => append_field(
            &mut bytes,
            "predecessor_query_closure",
            b"initial_query_no_predecessor",
        ),
    }
    sha256(&bytes)
}

fn current_tip_finalized_state_sha256_v1(
    pending: &VerifiedCasCommittedPendingTipV1,
    current_tip: &SignedExternalWatermarkCurrentTipV1,
    phase_revision: u64,
    statement_sha256: &str,
    signature_sha256: &str,
    trust_policy_sha256: &str,
) -> String {
    let mut bytes = b"hepta_linux_v8_external_watermark_completion_phase_v1\0".to_vec();
    append_u64(&mut bytes, "phase_revision", phase_revision);
    append_field(
        &mut bytes,
        "prior_state_sha256",
        pending.active_query_state_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "query_claim_binding_sha256",
        pending.active_query_claim_binding_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "query_nonce",
        current_tip.challenge.query_nonce.as_bytes(),
    );
    append_u64(
        &mut bytes,
        "issued_at_unix_seconds",
        current_tip.challenge.issued_at_unix_seconds,
    );
    append_u64(
        &mut bytes,
        "expires_at_unix_seconds",
        current_tip.challenge.expires_at_unix_seconds,
    );
    append_field(
        &mut bytes,
        "current_tip_statement_sha256",
        statement_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "current_tip_signature_sha256",
        signature_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "current_tip_trust_policy_sha256",
        trust_policy_sha256.as_bytes(),
    );
    append_record(&mut bytes, &current_tip.challenge.current_record);
    append_field(
        &mut bytes,
        "current_tip_sha256",
        current_tip.challenge.current_tip_sha256.as_bytes(),
    );
    sha256(&bytes)
}

fn completion_profile_sha256(profile: &FrozenExternalWatermarkCompletionProfileV1) -> String {
    let mut bytes = b"hepta_linux_v8_external_watermark_completion_profile_v1\0".to_vec();
    append_field(&mut bytes, "profile_id", profile.profile_id.as_bytes());
    append_u64(&mut bytes, "profile_revision", profile.profile_revision);
    append_field(
        &mut bytes,
        "commit_capability",
        b"exact_idempotent_cas_by_commit_nonce_and_consume_lease_once",
    );
    append_field(
        &mut bytes,
        "current_tip_capability",
        b"exact_idempotent_authenticated_current_tip_by_query_nonce_after_commit",
    );
    append_field(
        &mut bytes,
        "provider_profile_sha256",
        profile.provider_profile_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "lease_trust_policy_sha256",
        profile.lease_trust_policy_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "commit_trust_policy_sha256",
        profile.commit_trust_policy_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "current_tip_trust_policy_sha256",
        profile.current_tip_trust_policy_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "trust_root_id",
        profile.trust_root_id.as_bytes(),
    );
    append_field(
        &mut bytes,
        "key_fingerprint",
        profile.key_fingerprint.as_bytes(),
    );
    append_u64(
        &mut bytes,
        "maximum_current_tip_lifetime_seconds",
        profile.maximum_current_tip_lifetime_seconds,
    );
    append_u64(
        &mut bytes,
        "maximum_current_tip_retry_count",
        profile.maximum_current_tip_retry_count,
    );
    sha256(&bytes)
}

fn canonical_external_watermark_record_v1(record: &ExternalWatermarkRecordV1) -> Vec<u8> {
    let mut bytes = b"hepta_linux_v8_external_watermark_record_v1\0".to_vec();
    append_field(
        &mut bytes,
        "completion_profile_sha256",
        record.completion_profile_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "prepared_epoch_binding_sha256",
        record.prepared_epoch_binding_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "machine_id_sha256",
        record.machine_id_sha256.as_bytes(),
    );
    append_predecessor(&mut bytes, &record.predecessor);
    append_field(
        &mut bytes,
        "preparation_binding_sha256",
        record.preparation_binding_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "provider_profile_sha256",
        record.provider_profile_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "state_root_profile_sha256",
        record.state_root_profile_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "stream_id_sha256",
        record.stream_id_sha256.as_bytes(),
    );
    append_u64(&mut bytes, "successor_revision", record.successor_revision);
    bytes
}

fn validate_signed_envelope(
    declared_statement_sha256: &str,
    statement: &[u8],
    declared_signature_sha256: &str,
    signature_bytes: &[u8],
    observation: &CryptographicSignatureObservation,
    trust: &VerifiedTrustPolicyBindingV8,
) -> Result<String, QualificationError> {
    let statement_sha256 = sha256(statement);
    if declared_statement_sha256 != statement_sha256 {
        return Err(invalid("completion statement digest is not canonical"));
    }
    if signature_bytes.is_empty() || signature_bytes.len() > MAX_SIGNATURE_BYTES_V1 {
        return Err(invalid("completion signature bytes are empty or oversized"));
    }
    let signature_sha256 = sha256(signature_bytes);
    if declared_signature_sha256 != signature_sha256
        || !observation.exactly_matches(&signature_sha256, &statement_sha256, trust)
    {
        return Err(invalid("completion signature observation is not exact"));
    }
    Ok(signature_sha256)
}

fn validate_model_time_window(
    issued: u64,
    expires: u64,
    maximum_lifetime: u64,
    now: u64,
) -> Result<(), QualificationError> {
    validate_interval_shape(issued, expires, maximum_lifetime)?;
    if now < issued || now >= expires {
        return Err(invalid(
            "current-tip model time is outside the signed window",
        ));
    }
    Ok(())
}

fn validate_interval_shape(
    issued: u64,
    expires: u64,
    maximum_lifetime: u64,
) -> Result<(), QualificationError> {
    if issued >= expires || expires - issued > maximum_lifetime {
        return Err(invalid("current-tip validity interval is invalid"));
    }
    Ok(())
}

fn signer_matches_trust(
    signer: &AuthoritySignerBindingV8,
    trust: &VerifiedTrustPolicyBindingV8,
) -> Result<bool, QualificationError> {
    signer.validate()?;
    Ok(
        signer.allowed_signers_sha256 == trust.allowed_signers_sha256()
            && signer.key_fingerprint == trust.key_fingerprint()
            && signer.principal == trust.principal()
            && signer.signature_algorithm == trust.signature_algorithm(),
    )
}

fn predecessor_provider_profile_sha256(predecessor: &ExternalWatermarkPredecessorV1) -> &str {
    match predecessor {
        ExternalWatermarkPredecessorV1::GenesisPinnedSentinel {
            provider_profile_sha256,
            ..
        }
        | ExternalWatermarkPredecessorV1::Successor {
            provider_profile_sha256,
            ..
        } => provider_profile_sha256,
    }
}

fn predecessor_stream_id_sha256(predecessor: &ExternalWatermarkPredecessorV1) -> &str {
    match predecessor {
        ExternalWatermarkPredecessorV1::GenesisPinnedSentinel {
            stream_id_sha256, ..
        }
        | ExternalWatermarkPredecessorV1::Successor {
            stream_id_sha256, ..
        } => stream_id_sha256,
    }
}

fn predecessor_revision(predecessor: &ExternalWatermarkPredecessorV1) -> u64 {
    match predecessor {
        ExternalWatermarkPredecessorV1::GenesisPinnedSentinel { revision, .. }
        | ExternalWatermarkPredecessorV1::Successor { revision, .. } => *revision,
    }
}

fn predecessor_tip_sha256(predecessor: &ExternalWatermarkPredecessorV1) -> &str {
    match predecessor {
        ExternalWatermarkPredecessorV1::GenesisPinnedSentinel { tip_sha256, .. }
        | ExternalWatermarkPredecessorV1::Successor { tip_sha256, .. } => tip_sha256,
    }
}

fn predecessor_epoch_binding_sha256(predecessor: &ExternalWatermarkPredecessorV1) -> &str {
    match predecessor {
        ExternalWatermarkPredecessorV1::GenesisPinnedSentinel {
            genesis_epoch_binding_sha256,
            ..
        } => genesis_epoch_binding_sha256,
        ExternalWatermarkPredecessorV1::Successor {
            installed_epoch_binding_sha256,
            ..
        } => installed_epoch_binding_sha256,
    }
}

fn predecessor_epoch_sequence(predecessor: &ExternalWatermarkPredecessorV1) -> u64 {
    match predecessor {
        ExternalWatermarkPredecessorV1::GenesisPinnedSentinel { .. } => 0,
        ExternalWatermarkPredecessorV1::Successor {
            installed_epoch_sequence,
            ..
        } => *installed_epoch_sequence,
    }
}

fn inventory_sha256(inventory: &crate::ExactRootInstallInventoryV8) -> String {
    let mut bytes = b"hepta_linux_v8_install_inventory_binding_v1\0".to_vec();
    append_inventory(&mut bytes, inventory);
    sha256(&bytes)
}

fn append_inventory(statement: &mut Vec<u8>, inventory: &crate::ExactRootInstallInventoryV8) {
    append_root_file(statement, "admissiond_binary", &inventory.admissiond_binary);
    append_root_file(statement, "admissiond_unit", &inventory.admissiond_unit);
    append_root_file(statement, "recovery_binary", &inventory.recovery_binary);
    append_root_file(statement, "recovery_unit", &inventory.recovery_unit);
    append_field(
        statement,
        "state_root.path",
        inventory.state_root.path.as_bytes(),
    );
    append_u64(
        statement,
        "state_root.uid",
        u64::from(inventory.state_root.uid),
    );
    append_u64(
        statement,
        "state_root.gid",
        u64::from(inventory.state_root.gid),
    );
    append_u64(
        statement,
        "state_root.mode",
        u64::from(inventory.state_root.mode),
    );
    append_field(
        statement,
        "state_root.layout_manifest_sha256",
        inventory.state_root.layout_manifest_sha256.as_bytes(),
    );
}

fn append_root_file(statement: &mut Vec<u8>, label: &str, file: &RootFileInstallIdentityV8) {
    append_field(statement, &format!("{label}.path"), file.path.as_bytes());
    append_field(
        statement,
        &format!("{label}.content_sha256"),
        file.content_sha256.as_bytes(),
    );
    append_u64(statement, &format!("{label}.size_bytes"), file.size_bytes);
    append_u64(statement, &format!("{label}.uid"), u64::from(file.uid));
    append_u64(statement, &format!("{label}.gid"), u64::from(file.gid));
    append_u64(statement, &format!("{label}.mode"), u64::from(file.mode));
}

fn append_state_root_profile(statement: &mut Vec<u8>, profile: &StateRootProfileBindingV1) {
    append_field(statement, "state_profile.id", profile.profile_id.as_bytes());
    append_u64(
        statement,
        "state_profile.revision",
        profile.profile_revision,
    );
    append_field(
        statement,
        "state_profile.sha256",
        profile.profile_sha256.as_bytes(),
    );
    append_field(statement, "state_profile.path", profile.path.as_bytes());
    append_u64(statement, "state_profile.uid", u64::from(profile.uid));
    append_u64(statement, "state_profile.gid", u64::from(profile.gid));
    append_u64(statement, "state_profile.mode", u64::from(profile.mode));
    append_field(
        statement,
        "state_profile.layout_manifest_sha256",
        profile.layout_manifest_sha256.as_bytes(),
    );
}

fn append_epoch(statement: &mut Vec<u8>, epoch: &InstallEpochBindingV1) {
    append_u64(statement, "epoch.sequence", epoch.epoch_sequence);
    append_field(
        statement,
        "epoch.nonce_sha256",
        epoch.epoch_nonce_sha256.as_bytes(),
    );
}

fn append_predecessor(statement: &mut Vec<u8>, predecessor: &ExternalWatermarkPredecessorV1) {
    append_field(
        statement,
        "predecessor.kind",
        match predecessor {
            ExternalWatermarkPredecessorV1::GenesisPinnedSentinel { .. } => {
                b"genesis_pinned_sentinel"
            }
            ExternalWatermarkPredecessorV1::Successor { .. } => b"successor",
        },
    );
    append_u64(
        statement,
        "predecessor.revision",
        predecessor_revision(predecessor),
    );
    append_u64(
        statement,
        "predecessor.epoch_sequence",
        predecessor_epoch_sequence(predecessor),
    );
    append_field(
        statement,
        "predecessor.epoch_binding_sha256",
        predecessor_epoch_binding_sha256(predecessor).as_bytes(),
    );
    append_field(
        statement,
        "predecessor.provider_profile_sha256",
        predecessor_provider_profile_sha256(predecessor).as_bytes(),
    );
    append_field(
        statement,
        "predecessor.stream_id_sha256",
        predecessor_stream_id_sha256(predecessor).as_bytes(),
    );
    append_field(
        statement,
        "predecessor.tip_sha256",
        predecessor_tip_sha256(predecessor).as_bytes(),
    );
}

fn append_record(statement: &mut Vec<u8>, record: &ExternalWatermarkRecordV1) {
    append_field(
        statement,
        "record.canonical_sha256",
        external_watermark_record_sha256_v1(record).as_bytes(),
    );
    append_field(
        statement,
        "record.completion_profile_sha256",
        record.completion_profile_sha256.as_bytes(),
    );
    append_field(
        statement,
        "record.prepared_epoch_binding_sha256",
        record.prepared_epoch_binding_sha256.as_bytes(),
    );
    append_field(
        statement,
        "record.machine_id_sha256",
        record.machine_id_sha256.as_bytes(),
    );
    append_predecessor(statement, &record.predecessor);
    append_field(
        statement,
        "record.preparation_binding_sha256",
        record.preparation_binding_sha256.as_bytes(),
    );
    append_field(
        statement,
        "record.provider_profile_sha256",
        record.provider_profile_sha256.as_bytes(),
    );
    append_field(
        statement,
        "record.state_root_profile_sha256",
        record.state_root_profile_sha256.as_bytes(),
    );
    append_field(
        statement,
        "record.stream_id_sha256",
        record.stream_id_sha256.as_bytes(),
    );
    append_u64(
        statement,
        "record.successor_revision",
        record.successor_revision,
    );
}

fn append_signer(statement: &mut Vec<u8>, signer: &AuthoritySignerBindingV8) {
    append_field(
        statement,
        "allowed_signers_sha256",
        signer.allowed_signers_sha256.as_bytes(),
    );
    append_field(
        statement,
        "key_fingerprint",
        signer.key_fingerprint.as_bytes(),
    );
    append_field(statement, "principal", signer.principal.as_bytes());
    append_field(
        statement,
        "signature_algorithm",
        match signer.signature_algorithm {
            AuthoritySignatureAlgorithmV8::OpenSshSshsigEd25519 => b"openssh_sshsig_ed25519_sha256",
        },
    );
}

fn append_field(statement: &mut Vec<u8>, label: &str, value: &[u8]) {
    statement.extend_from_slice(&(label.len() as u64).to_be_bytes());
    statement.extend_from_slice(label.as_bytes());
    statement.extend_from_slice(&(value.len() as u64).to_be_bytes());
    statement.extend_from_slice(value);
}

fn append_u64(statement: &mut Vec<u8>, label: &str, value: u64) {
    append_field(statement, label, &value.to_be_bytes());
}

fn validate_digest(label: &str, value: &str) -> Result<(), QualificationError> {
    if !digest_shape(value) {
        return Err(invalid(format!(
            "{label} must be one non-zero lowercase SHA-256 digest"
        )));
    }
    Ok(())
}

fn digest_shape(value: &str) -> bool {
    value.len() == 64
        && !value.bytes().all(|byte| byte == b'0')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}
