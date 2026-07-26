//! Trusted explicit-preference source and reducer adapter.
//!
//! This module provides a transport-neutral HMAC source over the complete
//! memory-owned challenge plus a keyed durable ingress authority. A live
//! transport still has to provide strict parsing, secure key loading, and an
//! exact source binding; there is no allow-all source.

use std::fmt;
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
use hepta_memory::plan_preference_feedback_challenge;
use hmac::Hmac;
use hmac::Mac;
use sha2::Sha256;
use zeroize::Zeroizing;

use crate::EXPLICIT_PREFERENCE_REDUCER_VERSION;
use crate::ExplicitPreferenceSignal;
use crate::ExplicitPreferenceTarget;
use crate::PreferenceReductionError;
use crate::explicit_preference_genesis;
use crate::reduce_explicit_preference;

/// Stable identity of the deterministic explicit-preference reducer.
pub const EXPLICIT_PREFERENCE_REDUCER_ID: &str = "hepta.intelligence.explicit-preference.reducer";

const PREFERENCE_INGRESS_MAC_DOMAIN: &[u8] =
    b"hepta.intelligence.preference-ingress.hmac-sha256.v1";
const PREFERENCE_INGRESS_PLAN_MAC_DOMAIN: &[u8] =
    b"hepta.intelligence.preference-ingress.plan.hmac-sha256.v1";

type HmacSha256 = Hmac<Sha256>;

/// Non-cloneable authentication key for one trusted preference ingress.
pub struct PreferenceIngressAuthenticationKey(Zeroizing<[u8; 32]>);

impl PreferenceIngressAuthenticationKey {
    /// Constructs an exact 256-bit ingress authentication key.
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(Zeroizing::new(bytes))
    }
}

impl fmt::Debug for PreferenceIngressAuthenticationKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("PreferenceIngressAuthenticationKey")
            .field(&"[REDACTED]")
            .finish()
    }
}

/// Canonical HMAC proof over one memory-owned preference challenge.
#[derive(Clone, PartialEq, Eq)]
pub struct PreferenceIngressProof([u8; 32]);

impl PreferenceIngressProof {
    /// Parses one canonical lowercase hexadecimal proof.
    pub fn from_hex(encoded: &str) -> Result<Self, PreferenceFeedbackAuthenticationError> {
        if encoded.len() != 64 {
            return Err(PreferenceFeedbackAuthenticationError::new(
                "trusted_preference_ingress.proof_encoding_invalid",
            ));
        }
        let mut bytes = [0_u8; 32];
        for (index, pair) in encoded.as_bytes().chunks_exact(2).enumerate() {
            let Some(high) = decode_ingress_hex_nibble(pair[0]) else {
                return Err(PreferenceFeedbackAuthenticationError::new(
                    "trusted_preference_ingress.proof_encoding_invalid",
                ));
            };
            let Some(low) = decode_ingress_hex_nibble(pair[1]) else {
                return Err(PreferenceFeedbackAuthenticationError::new(
                    "trusted_preference_ingress.proof_encoding_invalid",
                ));
            };
            bytes[index] = (high << 4) | low;
        }
        Ok(Self(bytes))
    }

    /// Encodes the proof for an authenticated transport envelope.
    pub fn to_hex(&self) -> String {
        encode_ingress_hex(&self.0)
    }
}

impl fmt::Debug for PreferenceIngressProof {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("PreferenceIngressProof")
            .field(&"[REDACTED]")
            .finish()
    }
}

/// Per-request trusted source that verifies a proof inside memory authentication.
///
/// The HMAC is checked against the challenge evidence hash minted by Memory,
/// so it covers the source, reducer, transition, evidence, receipt, session,
/// subject, preference, target, and exact previous-state bindings.
pub struct HmacTrustedPreferenceFeedbackSource {
    source: PreferenceFeedbackSourceRef,
    key: PreferenceIngressAuthenticationKey,
    proof: PreferenceIngressProof,
}

impl HmacTrustedPreferenceFeedbackSource {
    /// Binds one source identity, secret key, and caller-supplied proof.
    pub fn new(
        source: PreferenceFeedbackSourceRef,
        key: PreferenceIngressAuthenticationKey,
        proof: PreferenceIngressProof,
    ) -> Self {
        Self { source, key, proof }
    }
}

impl fmt::Debug for HmacTrustedPreferenceFeedbackSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HmacTrustedPreferenceFeedbackSource")
            .field("source", &self.source)
            .field("key", &"[REDACTED]")
            .field("proof", &"[REDACTED]")
            .finish()
    }
}

impl TrustedPreferenceFeedbackSource for HmacTrustedPreferenceFeedbackSource {
    fn source(&self) -> PreferenceFeedbackSourceRef {
        self.source.clone()
    }

    fn authenticate(
        &self,
        challenge: &TrustedPreferenceFeedbackChallenge<'_>,
    ) -> Result<(), PreferenceFeedbackAuthenticationError> {
        if challenge.authority().source() != &self.source {
            return Err(PreferenceFeedbackAuthenticationError::new(
                "trusted_preference_ingress.source_binding_mismatch",
            ));
        }
        verify_preference_ingress_proof(
            &self.key,
            &self.proof,
            challenge.authority().evidence_hash(),
        )
    }
}

/// Plans the exact evidence hash that a trusted ingress client must sign.
pub fn explicit_preference_feedback_challenge_hash(
    input: &ExplicitPreferenceFeedbackInput,
    source: PreferenceFeedbackSourceRef,
) -> Result<ContentHash, PreferenceAuthorityError> {
    let reducer = TrustedExplicitPreferenceReducer::try_new()?;
    let challenge = plan_preference_feedback_challenge(
        input.request.clone(),
        source,
        reducer.binding().clone(),
    )?;
    Ok(challenge.evidence_hash().clone())
}

/// Signs one planned challenge hash for a trusted transport envelope.
pub fn sign_preference_ingress_challenge(
    key: &PreferenceIngressAuthenticationKey,
    challenge_hash: &ContentHash,
) -> Result<PreferenceIngressProof, PreferenceFeedbackAuthenticationError> {
    let mut mac = preference_ingress_mac(key)?;
    update_ingress_mac_frame(&mut mac, challenge_hash.as_str().as_bytes());
    let mut proof = [0_u8; 32];
    proof.copy_from_slice(&mac.finalize().into_bytes());
    Ok(PreferenceIngressProof(proof))
}

/// Signs one no-write challenge-planning request before durable state is read.
///
/// This proof uses a separate HMAC domain from the memory-owned commit
/// challenge. It binds every caller field plus the exact source and reducer.
pub fn sign_preference_ingress_challenge_plan(
    key: &PreferenceIngressAuthenticationKey,
    parts: &ExplicitPreferenceFeedbackChallengeInputParts,
    source: &PreferenceFeedbackSourceRef,
    reducer: &PreferenceReducerRef,
) -> Result<PreferenceIngressProof, PreferenceFeedbackAuthenticationError> {
    let mac = preference_ingress_plan_mac(key, parts, source, reducer)?;
    let mut proof = [0_u8; 32];
    proof.copy_from_slice(&mac.finalize().into_bytes());
    Ok(PreferenceIngressProof(proof))
}

/// Typed denial from the durable HMAC ingress boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PreferenceIngressCommitError {
    /// Memory authority, reducer, authentication, or storage denied the call.
    Authority(PreferenceAuthorityError),
    /// The exact transition had already committed and live replay is denied.
    ReplayDenied,
}

impl PreferenceIngressCommitError {
    /// Returns the underlying authority error when this is not a replay denial.
    pub fn authority(&self) -> Option<&PreferenceAuthorityError> {
        match self {
            Self::Authority(error) => Some(error),
            Self::ReplayDenied => None,
        }
    }
}

impl fmt::Display for PreferenceIngressCommitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Authority(error) => error.fmt(formatter),
            Self::ReplayDenied => formatter.write_str("trusted preference ingress replay denied"),
        }
    }
}

impl std::error::Error for PreferenceIngressCommitError {}

impl From<PreferenceAuthorityError> for PreferenceIngressCommitError {
    fn from(error: PreferenceAuthorityError) -> Self {
        Self::Authority(error)
    }
}

/// Long-lived keyed authority used by an authenticated live transport.
///
/// Proof verification happens before genesis initialization or any CAS write.
/// The store is durable and keyed independently from the ingress HMAC key.
pub struct DurableHmacTrustedPreferenceIngress {
    store: DurablePreferenceStore,
    source: PreferenceFeedbackSourceRef,
    authentication_key: PreferenceIngressAuthenticationKey,
}

impl fmt::Debug for DurableHmacTrustedPreferenceIngress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DurableHmacTrustedPreferenceIngress")
            .field("source", &self.source)
            .field("authentication_key", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl DurableHmacTrustedPreferenceIngress {
    /// Exclusively bootstraps a keyed durable live-ingress store.
    pub async fn bootstrap_new(
        path: impl AsRef<Path>,
        integrity_key: DurableIntegrityKey,
        authentication_key: PreferenceIngressAuthenticationKey,
        source: PreferenceFeedbackSourceRef,
    ) -> Result<Self, PreferenceAuthorityError> {
        validate_preference_source_binding(&source)?;
        let store = DurablePreferenceStore::bootstrap_new_keyed(path, integrity_key).await?;
        Ok(Self {
            store,
            source,
            authentication_key,
        })
    }

    /// Opens an existing keyed durable live-ingress store.
    pub async fn open_existing(
        path: impl AsRef<Path>,
        integrity_key: DurableIntegrityKey,
        authentication_key: PreferenceIngressAuthenticationKey,
        source: PreferenceFeedbackSourceRef,
    ) -> Result<Self, PreferenceAuthorityError> {
        validate_preference_source_binding(&source)?;
        let store = DurablePreferenceStore::open_existing_keyed(path, integrity_key).await?;
        Ok(Self {
            store,
            source,
            authentication_key,
        })
    }

    /// Returns the exact source identity pinned for the authority lifetime.
    pub fn source_binding(&self) -> &PreferenceFeedbackSourceRef {
        &self.source
    }

    /// Returns the exact deterministic reducer binding covered by each proof.
    pub fn reducer_binding(&self) -> Result<PreferenceReducerRef, PreferenceAuthorityError> {
        Ok(TrustedExplicitPreferenceReducer::try_new()?
            .binding()
            .clone())
    }

    /// Authenticates and plans a no-write challenge against exact durable state.
    ///
    /// The domain-separated planning proof is verified before any state read.
    pub async fn plan_challenge(
        &self,
        parts: ExplicitPreferenceFeedbackChallengeInputParts,
        proof: PreferenceIngressProof,
    ) -> Result<PlannedPreferenceIngressChallenge, PreferenceAuthorityError> {
        let reducer = self.reducer_binding()?;
        verify_preference_ingress_plan_proof(
            &self.authentication_key,
            &proof,
            &parts,
            &self.source,
            &reducer,
        )
        .map_err(PreferenceAuthorityError::Authentication)?;
        let genesis = explicit_preference_genesis(
            parts.subject.clone(),
            parts.preference.clone(),
            parts.target.clone(),
        );
        let expected_previous = self
            .store
            .read_document(&parts.preference, &parts.subject)
            .await?
            .map(|document| document.state().clone())
            .unwrap_or_else(|| genesis.state().clone());
        let input = parts.try_into_input(expected_previous)?;
        let challenge_hash =
            explicit_preference_feedback_challenge_hash(&input, self.source.clone())?;
        Ok(PlannedPreferenceIngressChallenge {
            input,
            challenge_hash,
        })
    }

    /// Verifies one proof, initializes exact genesis if needed, then attempts CAS.
    ///
    /// Invalid proofs are rejected before genesis initialization or any write.
    /// Exact transition replay is also denied rather than reported as success.
    pub async fn commit(
        &self,
        input: ExplicitPreferenceFeedbackInput,
        proof: PreferenceIngressProof,
    ) -> Result<PreferenceAuthorityCommitOutcome, PreferenceIngressCommitError> {
        let challenge_hash =
            explicit_preference_feedback_challenge_hash(&input, self.source.clone())?;
        verify_preference_ingress_proof(&self.authentication_key, &proof, &challenge_hash)
            .map_err(PreferenceAuthorityError::Authentication)?;

        let genesis = explicit_preference_genesis(
            input.request().subject().clone(),
            input.request().preference().clone(),
            input.target().clone(),
        );
        let current = self
            .store
            .read_document(input.request().preference(), input.request().subject())
            .await
            .map_err(PreferenceAuthorityError::from)?;
        let current_state = current
            .as_ref()
            .map(PreferenceStateDocument::state)
            .unwrap_or_else(|| genesis.state());
        if current_state != input.request().expected_previous() {
            return Err(PreferenceAuthorityError::from(
                hepta_memory::PreferenceCasError::StateConflict {
                    preference: input.request().preference().clone(),
                    subject: input.request().subject().clone(),
                    expected: input.request().expected_previous().clone(),
                    actual: current_state.clone(),
                },
            )
            .into());
        }
        self.store
            .get_or_init_genesis(
                input.request().preference().clone(),
                input.request().subject().clone(),
                PreferenceStateDocument::new(
                    genesis.state().clone(),
                    genesis.reducer_version(),
                    genesis.canonical_payload(),
                ),
            )
            .await
            .map_err(PreferenceAuthorityError::from)?;

        let source = BorrowedHmacTrustedPreferenceFeedbackSource {
            source: &self.source,
            key: &self.authentication_key,
            proof: &proof,
        };
        let outcome =
            advance_trusted_explicit_preference_durable(&self.store, &source, input).await?;
        if !outcome.committed_now() {
            return Err(PreferenceIngressCommitError::ReplayDenied);
        }
        Ok(outcome)
    }

    /// Reads the exact durable state for audit or challenge reconciliation.
    pub async fn read_document(
        &self,
        preference: &PreferenceId,
        subject: &PrincipalId,
    ) -> Result<Option<PreferenceStateDocument>, PreferenceAuthorityError> {
        self.store
            .read_document(preference, subject)
            .await
            .map_err(Into::into)
    }

    /// Returns the keyed durable store high-water projection for composition
    /// with an external monotonic anchor.
    pub async fn monotonic_state(
        &self,
    ) -> Result<hepta_memory::DurableMonotonicState, PreferenceAuthorityError> {
        self.store.monotonic_state().await.map_err(Into::into)
    }
}

struct BorrowedHmacTrustedPreferenceFeedbackSource<'a> {
    source: &'a PreferenceFeedbackSourceRef,
    key: &'a PreferenceIngressAuthenticationKey,
    proof: &'a PreferenceIngressProof,
}

impl TrustedPreferenceFeedbackSource for BorrowedHmacTrustedPreferenceFeedbackSource<'_> {
    fn source(&self) -> PreferenceFeedbackSourceRef {
        self.source.clone()
    }

    fn authenticate(
        &self,
        challenge: &TrustedPreferenceFeedbackChallenge<'_>,
    ) -> Result<(), PreferenceFeedbackAuthenticationError> {
        if challenge.authority().source() != self.source {
            return Err(PreferenceFeedbackAuthenticationError::new(
                "trusted_preference_ingress.source_binding_mismatch",
            ));
        }
        verify_preference_ingress_proof(self.key, self.proof, challenge.authority().evidence_hash())
    }
}

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

/// No-write challenge inputs whose exact prior state is resolved by the store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplicitPreferenceFeedbackChallengeInputParts {
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
}

impl ExplicitPreferenceFeedbackChallengeInputParts {
    fn try_into_input(
        self,
        expected_previous: PreferenceState,
    ) -> Result<ExplicitPreferenceFeedbackInput, PreferenceAuthorityError> {
        ExplicitPreferenceFeedbackInput::try_new(ExplicitPreferenceFeedbackInputParts {
            transition_id: self.transition_id,
            evidence_id: self.evidence_id,
            signal: self.signal,
            receipt: self.receipt,
            session_binding_hash: self.session_binding_hash,
            subject: self.subject,
            preference: self.preference,
            target: self.target,
            expected_previous,
        })
    }
}

/// Exact no-write challenge planned from the current keyed durable state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedPreferenceIngressChallenge {
    input: ExplicitPreferenceFeedbackInput,
    challenge_hash: ContentHash,
}

impl PlannedPreferenceIngressChallenge {
    /// Returns the complete input whose bindings were hashed by Memory.
    pub fn input(&self) -> &ExplicitPreferenceFeedbackInput {
        &self.input
    }

    /// Returns the Memory-owned exact challenge digest to authenticate.
    pub fn challenge_hash(&self) -> &ContentHash {
        &self.challenge_hash
    }

    /// Consumes the plan and returns its complete commit input.
    pub fn into_input(self) -> ExplicitPreferenceFeedbackInput {
        self.input
    }
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

fn preference_ingress_mac(
    key: &PreferenceIngressAuthenticationKey,
) -> Result<HmacSha256, PreferenceFeedbackAuthenticationError> {
    let mut mac = HmacSha256::new_from_slice(key.0.as_ref()).map_err(|_| {
        PreferenceFeedbackAuthenticationError::new(
            "trusted_preference_ingress.key_length_unsupported",
        )
    })?;
    update_ingress_mac_frame(&mut mac, PREFERENCE_INGRESS_MAC_DOMAIN);
    Ok(mac)
}

fn preference_ingress_plan_mac(
    key: &PreferenceIngressAuthenticationKey,
    parts: &ExplicitPreferenceFeedbackChallengeInputParts,
    source: &PreferenceFeedbackSourceRef,
    reducer: &PreferenceReducerRef,
) -> Result<HmacSha256, PreferenceFeedbackAuthenticationError> {
    let mut mac = HmacSha256::new_from_slice(key.0.as_ref()).map_err(|_| {
        PreferenceFeedbackAuthenticationError::new(
            "trusted_preference_ingress.key_length_unsupported",
        )
    })?;
    update_ingress_mac_frame(&mut mac, PREFERENCE_INGRESS_PLAN_MAC_DOMAIN);
    update_ingress_mac_frame(&mut mac, source.identity().as_str().as_bytes());
    update_ingress_mac_frame(&mut mac, &source.revision().get().to_be_bytes());
    update_ingress_mac_frame(&mut mac, source.content_hash().as_str().as_bytes());
    update_ingress_mac_frame(&mut mac, reducer.identity().as_bytes());
    update_ingress_mac_frame(&mut mac, reducer.version().as_bytes());
    update_ingress_mac_frame(&mut mac, parts.transition_id.as_str().as_bytes());
    update_ingress_mac_frame(&mut mac, parts.evidence_id.as_str().as_bytes());
    update_ingress_mac_frame(
        &mut mac,
        match parts.signal {
            ExplicitPreferenceSignal::Accepted => b"accepted",
            ExplicitPreferenceSignal::Rejected => b"rejected",
        },
    );
    update_ingress_mac_frame(&mut mac, parts.receipt.id().as_str().as_bytes());
    update_ingress_mac_frame(&mut mac, parts.receipt.receipt_hash().as_str().as_bytes());
    update_ingress_mac_frame(&mut mac, parts.session_binding_hash.as_str().as_bytes());
    update_ingress_mac_frame(&mut mac, parts.subject.as_str().as_bytes());
    update_ingress_mac_frame(&mut mac, parts.preference.as_str().as_bytes());
    update_ingress_mac_frame(&mut mac, parts.target.binding_hash().as_str().as_bytes());
    Ok(mac)
}

fn verify_preference_ingress_plan_proof(
    key: &PreferenceIngressAuthenticationKey,
    proof: &PreferenceIngressProof,
    parts: &ExplicitPreferenceFeedbackChallengeInputParts,
    source: &PreferenceFeedbackSourceRef,
    reducer: &PreferenceReducerRef,
) -> Result<(), PreferenceFeedbackAuthenticationError> {
    preference_ingress_plan_mac(key, parts, source, reducer)?
        .verify_slice(&proof.0)
        .map_err(|_| {
            PreferenceFeedbackAuthenticationError::new(
                "trusted_preference_ingress.plan_proof_verification_failed",
            )
        })
}

fn verify_preference_ingress_proof(
    key: &PreferenceIngressAuthenticationKey,
    proof: &PreferenceIngressProof,
    challenge_hash: &ContentHash,
) -> Result<(), PreferenceFeedbackAuthenticationError> {
    let mut mac = preference_ingress_mac(key)?;
    update_ingress_mac_frame(&mut mac, challenge_hash.as_str().as_bytes());
    mac.verify_slice(&proof.0).map_err(|_| {
        PreferenceFeedbackAuthenticationError::new(
            "trusted_preference_ingress.proof_verification_failed",
        )
    })
}

fn validate_preference_source_binding(
    source: &PreferenceFeedbackSourceRef,
) -> Result<(), PreferenceAuthorityError> {
    if source.identity().as_str().is_empty() {
        return Err(PreferenceAuthorityError::EmptyBinding {
            field: "feedback_source.identity",
        });
    }
    if source.content_hash().as_str().is_empty() {
        return Err(PreferenceAuthorityError::EmptyBinding {
            field: "feedback_source.content_hash",
        });
    }
    Ok(())
}

fn update_ingress_mac_frame(mac: &mut HmacSha256, value: &[u8]) {
    mac.update(&(value.len() as u64).to_be_bytes());
    mac.update(value);
}

fn encode_ingress_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

fn decode_ingress_hex_nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests;
