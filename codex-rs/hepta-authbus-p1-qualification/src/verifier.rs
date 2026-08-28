use std::collections::BTreeMap;

use codex_hepta_contracts::Sha256Digest;
use ed25519_dalek::Signature;
use ed25519_dalek::VerifyingKey;

use crate::P11Error;
use crate::P11EvidenceState;
use crate::P11IdentityVerificationContext;
use crate::P11IdentityVerificationReceipt;
use crate::P11KeyPurpose;
use crate::P11ManualDecision;
use crate::P11ManualEvidenceDisposition;
use crate::P11ManualEvidenceReceipt;
use crate::P11OperationEvidenceBinding;
use crate::P11ProviderEvidenceDisposition;
use crate::P11ProviderStatusReceipt;
use crate::P11Result;
use crate::P11SignedIdentityEvidence;
use crate::P11SignedManualEvidence;
use crate::P11SignedProviderStatusEvidence;
use crate::P11VerificationKeyRecord;
use crate::P11WriteDisposition;
use crate::length_delimited_digest;
use crate::push_digest;
use crate::push_text;
use crate::push_u64;
use crate::validate_signature;
use crate::validate_text;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct P11Policy {
    pub identity_max_ttl_seconds: u64,
    pub allowed_clock_skew_seconds: u64,
    pub provider_max_age_seconds: u64,
    pub provider_future_skew_seconds: u64,
    pub max_nonce_entries: usize,
    pub max_operation_entries: usize,
}

impl Default for P11Policy {
    fn default() -> Self {
        Self {
            identity_max_ttl_seconds: 300,
            allowed_clock_skew_seconds: 5,
            provider_max_age_seconds: 600,
            provider_future_skew_seconds: 5,
            max_nonce_entries: 4_096,
            max_operation_entries: 4_096,
        }
    }
}

impl P11Policy {
    pub fn validate(self) -> P11Result<()> {
        if self.identity_max_ttl_seconds == 0
            || self.provider_max_age_seconds == 0
            || self.max_nonce_entries == 0
            || self.max_operation_entries == 0
        {
            return Err(P11Error::InvalidInput);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct P11KeyRegistry {
    records: BTreeMap<(String, String, u64), P11VerificationKeyRecord>,
    current: BTreeMap<(String, P11KeyPurpose), (u64, String)>,
}

impl P11KeyRegistry {
    pub fn register(&mut self, record: P11VerificationKeyRecord) -> P11Result<P11WriteDisposition> {
        record.validate()?;
        let record_key = (
            record.issuer_id.clone(),
            record.key_id.clone(),
            record.key_epoch,
        );
        if let Some(existing) = self.records.get(&record_key) {
            if existing == &record {
                return Ok(P11WriteDisposition::AlreadyPresent);
            }
            return Err(P11Error::KeyConflict);
        }

        let current_key = (record.issuer_id.clone(), record.purpose);
        if let Some((current_epoch, current_id)) = self.current.get(&current_key) {
            if record.key_epoch < *current_epoch {
                return Err(P11Error::StaleKeyEpoch);
            }
            if record.key_epoch == *current_epoch && record.key_id != *current_id {
                return Err(P11Error::KeyConflict);
            }
        }

        self.current
            .insert(current_key, (record.key_epoch, record.key_id.clone()));
        self.records.insert(record_key, record);
        Ok(P11WriteDisposition::Applied)
    }

    pub fn revoke(
        &mut self,
        issuer_id: &str,
        key_id: &str,
        key_epoch: u64,
        revoked_at_unix_seconds: u64,
    ) -> P11Result<P11WriteDisposition> {
        validate_text(issuer_id)?;
        validate_text(key_id)?;
        if key_epoch == 0 || revoked_at_unix_seconds == 0 {
            return Err(P11Error::InvalidInput);
        }
        let record = self
            .records
            .get_mut(&(issuer_id.to_owned(), key_id.to_owned(), key_epoch))
            .ok_or(P11Error::UnknownKey)?;
        if revoked_at_unix_seconds < record.valid_from_unix_seconds {
            return Err(P11Error::InvalidInput);
        }
        if let Some(existing) = record.revoked_at_unix_seconds {
            if existing == revoked_at_unix_seconds {
                return Ok(P11WriteDisposition::AlreadyPresent);
            }
            return Err(P11Error::KeyConflict);
        }
        record.revoked_at_unix_seconds = Some(revoked_at_unix_seconds);
        Ok(P11WriteDisposition::Applied)
    }

    fn resolve(
        &self,
        issuer_id: &str,
        key_id: &str,
        key_epoch: u64,
        purpose: P11KeyPurpose,
        at_unix_seconds: u64,
    ) -> P11Result<P11VerificationKeyRecord> {
        let record = self
            .records
            .get(&(issuer_id.to_owned(), key_id.to_owned(), key_epoch))
            .ok_or(P11Error::UnknownKey)?;
        if record.purpose != purpose {
            return Err(P11Error::KeyPurposeMismatch);
        }
        let current = self
            .current
            .get(&(issuer_id.to_owned(), purpose))
            .ok_or(P11Error::UnknownKey)?;
        if current.0 != key_epoch || current.1 != key_id {
            return Err(P11Error::StaleKeyEpoch);
        }
        if at_unix_seconds < record.valid_from_unix_seconds {
            return Err(P11Error::KeyNotYetValid);
        }
        if at_unix_seconds >= record.valid_until_unix_seconds {
            return Err(P11Error::KeyExpired);
        }
        if record
            .revoked_at_unix_seconds
            .is_some_and(|revoked_at| at_unix_seconds >= revoked_at)
        {
            return Err(P11Error::KeyRevoked);
        }
        Ok(record.clone())
    }
}

#[derive(Clone, Debug)]
struct NonceEntry {
    expires_at_unix_seconds: u64,
}

#[derive(Clone, Debug, Default)]
struct P11NonceReplayCache {
    entries: BTreeMap<String, NonceEntry>,
}

impl P11NonceReplayCache {
    fn observe(
        &mut self,
        nonce_key: Sha256Digest,
        expires_at_unix_seconds: u64,
        now_unix_seconds: u64,
        capacity: usize,
    ) -> P11Result<()> {
        self.entries
            .retain(|_, entry| entry.expires_at_unix_seconds > now_unix_seconds);
        let key = nonce_key.as_str().to_owned();
        if self.entries.contains_key(&key) {
            return Err(P11Error::NonceReplay);
        }
        if self.entries.len() >= capacity {
            return Err(P11Error::NonceCapacity);
        }
        self.entries.insert(
            key,
            NonceEntry {
                expires_at_unix_seconds,
            },
        );
        Ok(())
    }

    fn len(&self) -> usize {
        self.entries.len()
    }
}

#[derive(Clone, Debug)]
struct OperationLedger {
    binding: P11OperationEvidenceBinding,
    state: P11EvidenceState,
    last_status_revision: Option<u64>,
    last_manual_revision: Option<u64>,
    last_observed_at_unix_seconds: Option<u64>,
    last_status_digest: Option<Sha256Digest>,
    last_manual_digest: Option<Sha256Digest>,
    last_status_receipt: Option<P11ProviderStatusReceipt>,
    last_manual_receipt: Option<P11ManualEvidenceReceipt>,
}

impl OperationLedger {
    fn new(binding: P11OperationEvidenceBinding) -> Self {
        Self {
            binding,
            state: P11EvidenceState::Pending,
            last_status_revision: None,
            last_manual_revision: None,
            last_observed_at_unix_seconds: None,
            last_status_digest: None,
            last_manual_digest: None,
            last_status_receipt: None,
            last_manual_receipt: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct P11Verifier {
    policy: P11Policy,
    keys: P11KeyRegistry,
    nonces: P11NonceReplayCache,
    operations: BTreeMap<String, OperationLedger>,
}

impl P11Verifier {
    pub fn new(policy: P11Policy) -> P11Result<Self> {
        policy.validate()?;
        Ok(Self {
            policy,
            keys: P11KeyRegistry::default(),
            nonces: P11NonceReplayCache::default(),
            operations: BTreeMap::new(),
        })
    }

    pub fn register_key(
        &mut self,
        record: P11VerificationKeyRecord,
    ) -> P11Result<P11WriteDisposition> {
        self.keys.register(record)
    }

    pub fn revoke_key(
        &mut self,
        issuer_id: &str,
        key_id: &str,
        key_epoch: u64,
        revoked_at_unix_seconds: u64,
    ) -> P11Result<P11WriteDisposition> {
        self.keys
            .revoke(issuer_id, key_id, key_epoch, revoked_at_unix_seconds)
    }

    pub fn nonce_entry_count(&self) -> usize {
        self.nonces.len()
    }

    pub fn register_operation(
        &mut self,
        binding: P11OperationEvidenceBinding,
    ) -> P11Result<P11WriteDisposition> {
        binding.validate()?;
        if let Some(existing) = self.operations.get(&binding.operation_id) {
            if existing.binding == binding {
                return Ok(P11WriteDisposition::AlreadyPresent);
            }
            return Err(P11Error::OperationConflict);
        }
        if self.operations.len() >= self.policy.max_operation_entries {
            return Err(P11Error::OperationCapacity);
        }
        self.operations
            .insert(binding.operation_id.clone(), OperationLedger::new(binding));
        Ok(P11WriteDisposition::Applied)
    }

    pub fn operation_state(&self, operation_id: &str) -> P11Result<P11EvidenceState> {
        self.operations
            .get(operation_id)
            .map(|ledger| ledger.state)
            .ok_or(P11Error::UnknownOperation)
    }

    pub fn verify_identity(
        &mut self,
        evidence: &P11SignedIdentityEvidence,
        context: &P11IdentityVerificationContext,
    ) -> P11Result<P11IdentityVerificationReceipt> {
        context.validate()?;
        let signing_bytes = evidence.signing_bytes()?;
        validate_signature(&evidence.signature)?;
        let binding = &evidence.binding;

        if binding.audience != context.expected_audience {
            return Err(P11Error::AudienceMismatch);
        }
        if binding.service_identity_digest != context.expected_service_identity_sha256
            || binding.policy_digest != context.expected_policy_sha256
            || binding.peer != context.expected_peer
        {
            return Err(P11Error::BindingMismatch);
        }

        let now = context.now_unix_seconds;
        let skew = self.policy.allowed_clock_skew_seconds;
        if now.saturating_add(skew) < binding.not_before_unix_seconds {
            return Err(P11Error::NotYetValid);
        }
        if now >= binding.expires_at_unix_seconds.saturating_add(skew) {
            return Err(P11Error::Expired);
        }
        if binding.issued_at_unix_seconds > now.saturating_add(skew) {
            return Err(P11Error::FutureEvidence);
        }
        if binding
            .expires_at_unix_seconds
            .saturating_sub(binding.issued_at_unix_seconds)
            > self.policy.identity_max_ttl_seconds
        {
            return Err(P11Error::TtlExceeded);
        }

        let key = self.keys.resolve(
            &evidence.issuer_id,
            &evidence.key_id,
            evidence.key_epoch,
            P11KeyPurpose::IdentityIssuer,
            now,
        )?;
        verify_ed25519(&key, &signing_bytes, &evidence.signature)?;

        let binding_sha256 = binding.digest().map_err(|_| P11Error::ContractInvalid)?;
        let nonce_key = identity_nonce_key(evidence, &binding_sha256);
        self.nonces.observe(
            nonce_key,
            binding.expires_at_unix_seconds.saturating_add(skew),
            now,
            self.policy.max_nonce_entries,
        )?;

        Ok(P11IdentityVerificationReceipt {
            evidence_sha256: evidence.evidence_digest()?,
            binding_sha256,
            issuer_id: evidence.issuer_id.clone(),
            key_id: evidence.key_id.clone(),
            key_epoch: evidence.key_epoch,
            subject_sha256: binding.subject_digest.clone(),
            nonce_sha256: binding.nonce_sha256.clone(),
            launch_nonce_sha256: binding.launch_nonce_sha256.clone(),
            expires_at_unix_seconds: binding.expires_at_unix_seconds,
            authority: false,
        })
    }

    pub fn verify_provider_status(
        &mut self,
        evidence: &P11SignedProviderStatusEvidence,
        now_unix_seconds: u64,
    ) -> P11Result<P11ProviderEvidenceDisposition> {
        if now_unix_seconds == 0 {
            return Err(P11Error::InvalidInput);
        }
        let signing_bytes = evidence.signing_bytes()?;
        validate_signature(&evidence.signature)?;
        check_observation_time(
            evidence.observed_at_unix_seconds,
            now_unix_seconds,
            self.policy.provider_max_age_seconds,
            self.policy.provider_future_skew_seconds,
        )?;
        let binding = self
            .operations
            .get(&evidence.operation_id)
            .map(|ledger| ledger.binding.clone())
            .ok_or(P11Error::UnknownOperation)?;
        verify_provider_binding(&binding, evidence)?;

        let key = self.keys.resolve(
            &evidence.issuer_id,
            &evidence.key_id,
            evidence.key_epoch,
            P11KeyPurpose::ProviderStatusIssuer,
            now_unix_seconds,
        )?;
        verify_ed25519(&key, &signing_bytes, &evidence.signature)?;
        let evidence_sha256 = evidence.evidence_digest()?;

        let ledger = self
            .operations
            .get_mut(&evidence.operation_id)
            .ok_or(P11Error::UnknownOperation)?;
        if ledger.last_status_revision == Some(evidence.status_revision) {
            if ledger.last_status_digest.as_ref() == Some(&evidence_sha256) {
                return ledger
                    .last_status_receipt
                    .clone()
                    .map(P11ProviderEvidenceDisposition::AlreadyPresent)
                    .ok_or(P11Error::EvidenceConflict);
            }
            return Err(P11Error::EvidenceConflict);
        }
        if ledger.state.is_terminal() {
            return Err(P11Error::TerminalImmutable);
        }
        if ledger.state == P11EvidenceState::ManualRequired {
            return Err(P11Error::ManualEvidenceRequired);
        }
        match ledger.last_status_revision {
            Some(revision) if evidence.status_revision <= revision => {
                return Err(P11Error::StaleObservation);
            }
            None if evidence.status_revision != 1 => {
                return Err(P11Error::StaleObservation);
            }
            _ => {}
        }
        if ledger
            .last_observed_at_unix_seconds
            .is_some_and(|observed_at| evidence.observed_at_unix_seconds < observed_at)
        {
            return Err(P11Error::StaleObservation);
        }

        let receipt = P11ProviderStatusReceipt {
            evidence_sha256: evidence_sha256.clone(),
            operation_id: evidence.operation_id.clone(),
            status_revision: evidence.status_revision,
            observed_at_unix_seconds: evidence.observed_at_unix_seconds,
            state: evidence.outcome.target_state(),
            authority: false,
        };
        ledger.state = receipt.state;
        ledger.last_status_revision = Some(evidence.status_revision);
        ledger.last_observed_at_unix_seconds = Some(evidence.observed_at_unix_seconds);
        ledger.last_status_digest = Some(evidence_sha256);
        ledger.last_status_receipt = Some(receipt.clone());
        Ok(P11ProviderEvidenceDisposition::Applied(receipt))
    }

    pub fn verify_manual_evidence(
        &mut self,
        evidence: &P11SignedManualEvidence,
        now_unix_seconds: u64,
    ) -> P11Result<P11ManualEvidenceDisposition> {
        if now_unix_seconds == 0 {
            return Err(P11Error::InvalidInput);
        }
        let signing_bytes = evidence.signing_bytes()?;
        validate_signature(&evidence.signature)?;
        check_observation_time(
            evidence.observed_at_unix_seconds,
            now_unix_seconds,
            self.policy.provider_max_age_seconds,
            self.policy.provider_future_skew_seconds,
        )?;
        let binding = self
            .operations
            .get(&evidence.operation_id)
            .map(|ledger| ledger.binding.clone())
            .ok_or(P11Error::UnknownOperation)?;
        verify_manual_binding(&binding, evidence)?;

        let key = self.keys.resolve(
            &evidence.issuer_id,
            &evidence.key_id,
            evidence.key_epoch,
            P11KeyPurpose::OperatorEvidenceIssuer,
            now_unix_seconds,
        )?;
        verify_ed25519(&key, &signing_bytes, &evidence.signature)?;
        let evidence_sha256 = evidence.evidence_digest()?;

        let ledger = self
            .operations
            .get_mut(&evidence.operation_id)
            .ok_or(P11Error::UnknownOperation)?;
        if ledger.last_manual_revision == Some(evidence.manual_revision) {
            if ledger.last_manual_digest.as_ref() == Some(&evidence_sha256) {
                return ledger
                    .last_manual_receipt
                    .clone()
                    .map(P11ManualEvidenceDisposition::AlreadyPresent)
                    .ok_or(P11Error::EvidenceConflict);
            }
            return Err(P11Error::EvidenceConflict);
        }
        if ledger.state.is_terminal() {
            return Err(P11Error::TerminalImmutable);
        }
        if ledger.state != P11EvidenceState::ManualRequired {
            return Err(P11Error::InvalidManualTransition);
        }
        if ledger
            .last_manual_revision
            .is_some_and(|revision| evidence.manual_revision <= revision)
        {
            return Err(P11Error::StaleObservation);
        }
        if ledger
            .last_observed_at_unix_seconds
            .is_some_and(|observed_at| evidence.observed_at_unix_seconds < observed_at)
        {
            return Err(P11Error::StaleObservation);
        }

        let state = match evidence.decision {
            P11ManualDecision::ResumeLookupOnly => P11EvidenceState::LookupOnly,
            P11ManualDecision::KeepManualRequired => P11EvidenceState::ManualRequired,
            P11ManualDecision::Quarantine => P11EvidenceState::Quarantined,
        };
        let receipt = P11ManualEvidenceReceipt {
            evidence_sha256: evidence_sha256.clone(),
            operation_id: evidence.operation_id.clone(),
            manual_revision: evidence.manual_revision,
            observed_at_unix_seconds: evidence.observed_at_unix_seconds,
            state,
            authority: false,
        };
        ledger.state = state;
        ledger.last_manual_revision = Some(evidence.manual_revision);
        ledger.last_observed_at_unix_seconds = Some(evidence.observed_at_unix_seconds);
        ledger.last_manual_digest = Some(evidence_sha256);
        ledger.last_manual_receipt = Some(receipt.clone());
        Ok(P11ManualEvidenceDisposition::Applied(receipt))
    }
}

fn verify_ed25519(
    key: &P11VerificationKeyRecord,
    message: &[u8],
    signature: &[u8],
) -> P11Result<()> {
    let public_key: [u8; 32] = key
        .public_key
        .as_slice()
        .try_into()
        .map_err(|_| P11Error::InvalidInput)?;
    let signature_bytes: [u8; 64] = signature.try_into().map_err(|_| P11Error::InvalidInput)?;
    let verifying_key =
        VerifyingKey::from_bytes(&public_key).map_err(|_| P11Error::InvalidInput)?;
    let signature = Signature::from_bytes(&signature_bytes);
    verifying_key
        .verify_strict(message, &signature)
        .map_err(|_| P11Error::SignatureInvalid)
}

fn identity_nonce_key(
    evidence: &P11SignedIdentityEvidence,
    binding_sha256: &Sha256Digest,
) -> Sha256Digest {
    let binding = &evidence.binding;
    let mut bytes = Vec::new();
    push_text(&mut bytes, &evidence.issuer_id);
    push_text(&mut bytes, &evidence.key_id);
    push_u64(&mut bytes, evidence.key_epoch);
    push_digest(&mut bytes, &binding.subject_digest);
    push_text(&mut bytes, &binding.audience);
    push_digest(&mut bytes, &binding.nonce_sha256);
    push_digest(&mut bytes, &binding.launch_nonce_sha256);
    push_digest(&mut bytes, binding_sha256);
    length_delimited_digest(
        "hepta.authbus.p1.1.identity-nonce-key.v1",
        &[bytes.as_slice()],
    )
}

fn verify_provider_binding(
    binding: &P11OperationEvidenceBinding,
    evidence: &P11SignedProviderStatusEvidence,
) -> P11Result<()> {
    if binding.operation_id != evidence.operation_id
        || binding.provider_id != evidence.provider_id
        || binding.profile_id != evidence.profile_id
        || binding.token_family_id != evidence.token_family_id
        || binding.status_binding_sha256 != evidence.status_binding_sha256
        || binding.fence != evidence.fence
    {
        return Err(P11Error::BindingMismatch);
    }
    Ok(())
}

fn verify_manual_binding(
    binding: &P11OperationEvidenceBinding,
    evidence: &P11SignedManualEvidence,
) -> P11Result<()> {
    if binding.operation_id != evidence.operation_id
        || binding.status_binding_sha256 != evidence.status_binding_sha256
        || binding.fence != evidence.fence
    {
        return Err(P11Error::BindingMismatch);
    }
    Ok(())
}

fn check_observation_time(
    observed_at_unix_seconds: u64,
    now_unix_seconds: u64,
    max_age_seconds: u64,
    future_skew_seconds: u64,
) -> P11Result<()> {
    if observed_at_unix_seconds > now_unix_seconds.saturating_add(future_skew_seconds) {
        return Err(P11Error::FutureEvidence);
    }
    if now_unix_seconds.saturating_sub(observed_at_unix_seconds) > max_age_seconds {
        return Err(P11Error::EvidenceTooOld);
    }
    Ok(())
}
