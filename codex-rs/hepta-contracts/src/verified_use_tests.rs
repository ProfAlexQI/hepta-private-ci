use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use pretty_assertions::assert_eq;

use crate::AgentId;
use crate::AuthorityGrant;
use crate::AuthorityLeaseBinding;
use crate::Authorized;
use crate::CapabilityUseVerificationRequest;
use crate::CapabilityUseVerifier;
use crate::CapabilityVerificationRequest;
use crate::CapabilityVerifier;
use crate::CognitiveWriteCapability;
use crate::ExternalEffectCapability;
use crate::OperationId;
use crate::RuntimeAuthorityContext;
use crate::Sha256Digest;
use crate::authorize_verified_capability;

use super::*;

const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const GENERATION: u64 = 3;
const AUTHORITY_EPOCH: u64 = 7;
const OWNER_EPOCH: u64 = 11;

fn agent_id() -> AgentId {
    AgentId::parse(AGENT_ID).unwrap_or_else(|error| panic!("AgentId must parse: {error}"))
}

fn operation(value: &str) -> OperationId {
    OperationId::parse(value)
        .unwrap_or_else(|error| panic!("operation id must parse: {error}"))
}

fn revision(value: u64) -> RevocationRevision {
    RevocationRevision::new(value)
        .unwrap_or_else(|error| panic!("revision must be non-zero: {error}"))
}

struct MintVerifier;

impl CapabilityVerifier for MintVerifier {
    fn verify(&self, _request: &CapabilityVerificationRequest<'_>) -> Result<(), String> {
        Ok(())
    }
}

struct TestClock {
    now: AtomicU64,
    reject: AtomicBool,
    calls: AtomicUsize,
}

impl TestClock {
    fn at(now: u64) -> Self {
        Self {
            now: AtomicU64::new(now),
            reject: AtomicBool::new(false),
            calls: AtomicUsize::new(0),
        }
    }

    fn set(&self, now: u64) {
        self.now.store(now, Ordering::SeqCst);
    }
}

impl TrustedPhysicalClock for TestClock {
    fn now_unix_seconds(&self) -> Result<u64, String> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if self.reject.load(Ordering::SeqCst) {
            Err("clock unavailable".to_string())
        } else {
            Ok(self.now.load(Ordering::SeqCst))
        }
    }
}

struct TestPhysicalVerifier {
    revision: AtomicU64,
    valid_until: AtomicU64,
    broad_calls: AtomicUsize,
    physical_calls: AtomicUsize,
}

impl TestPhysicalVerifier {
    fn allowing(revision: RevocationRevision, valid_until: u64) -> Self {
        Self {
            revision: AtomicU64::new(revision.get()),
            valid_until: AtomicU64::new(valid_until),
            broad_calls: AtomicUsize::new(0),
            physical_calls: AtomicUsize::new(0),
        }
    }

    fn set_revision(&self, revision: RevocationRevision) {
        self.revision.store(revision.get(), Ordering::SeqCst);
    }
}

impl CapabilityUseVerifier for TestPhysicalVerifier {
    fn verify_use(&self, _request: &CapabilityUseVerificationRequest<'_>) -> Result<(), String> {
        self.broad_calls.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

impl PhysicalUseVerifier for TestPhysicalVerifier {
    fn verify_physical_use(
        &self,
        request: &PhysicalUseVerificationRequest<'_>,
    ) -> Result<PhysicalUseVerification, String> {
        let call = self.physical_calls.fetch_add(1, Ordering::SeqCst) + 1;
        if request.operation_id().as_str().is_empty()
            || request.final_payload_sha256().as_str().is_empty()
            || request.runtime_context().generation() != GENERATION
            || request.observed_at_unix_seconds() == 0
            || request.requested_expires_at_unix_seconds()
                <= request.observed_at_unix_seconds()
        {
            return Err("final request binding drift".to_string());
        }
        PhysicalUseVerification::new(
            revision(self.revision.load(Ordering::SeqCst)),
            self.valid_until.load(Ordering::SeqCst),
            Sha256Digest::for_bytes(format!("authority-receipt-{call}").as_bytes()),
        )
        .map_err(|error| error.to_string())
    }
}

#[derive(Default)]
struct InMemoryClaimStore {
    claims: Mutex<BTreeMap<String, String>>,
    next_revision: AtomicU64,
    unavailable: AtomicBool,
    corrupt_receipt: AtomicBool,
    calls: AtomicUsize,
}

impl PhysicalUseClaimStore for InMemoryClaimStore {
    fn claim_once(
        &self,
        request: &PhysicalUseClaimRequest<'_>,
    ) -> Result<PhysicalUseClaimReceipt, PhysicalUseClaimStoreError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if self.unavailable.load(Ordering::SeqCst) {
            return Err(PhysicalUseClaimStoreError::Unavailable(
                "disk unavailable".to_string(),
            ));
        }

        let scope = request
            .claim_key()
            .operation_scope_sha256()
            .as_str()
            .to_string();
        let claim = request.claim_key().claim_sha256().as_str().to_string();
        let mut claims = self
            .claims
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(existing) = claims.get(&scope) {
            if existing == &claim {
                return Err(PhysicalUseClaimStoreError::AlreadyClaimed);
            }
            return Err(PhysicalUseClaimStoreError::OperationPayloadConflict);
        }
        claims.insert(scope, claim);

        let claim_revision = self.next_revision.fetch_add(1, Ordering::SeqCst) + 1;
        let claimed_at = if self.corrupt_receipt.load(Ordering::SeqCst) {
            request.claimed_at_unix_seconds() + 1
        } else {
            request.claimed_at_unix_seconds()
        };
        PhysicalUseClaimReceipt::new(
            request.claim_key().clone(),
            claim_revision,
            claimed_at,
            Sha256Digest::for_bytes(
                format!(
                    "claim-receipt-{claim_revision}-{}",
                    request.token_sha256().as_str()
                )
                .as_bytes(),
            ),
        )
    }
}

fn external_fixture(
    expires_at_unix_seconds: u64,
) -> (
    Authorized<ExternalEffectCapability>,
    RuntimeAuthorityContext,
) {
    let binding = AuthorityLeaseBinding::new(
        agent_id(),
        Sha256Digest::for_bytes(b"signed-effect-grant"),
        AUTHORITY_EPOCH,
        OWNER_EPOCH,
        GENERATION,
        Sha256Digest::for_bytes(b"effect-fence"),
        expires_at_unix_seconds,
    )
    .unwrap_or_else(|error| panic!("binding must build: {error}"));
    let capability = authorize_verified_capability::<ExternalEffectCapability, _>(
        binding.clone(),
        &agent_id(),
        GENERATION,
        100,
        &MintVerifier,
    )
    .unwrap_or_else(|error| panic!("capability must mint: {error}"));
    let runtime_context = RuntimeAuthorityContext::from_external_binding(&binding)
        .unwrap_or_else(|error| panic!("runtime context must build: {error}"));
    (capability, runtime_context)
}

#[allow(clippy::too_many_arguments)]
fn issue(
    capability: &Authorized<ExternalEffectCapability>,
    runtime_context: &RuntimeAuthorityContext,
    operation_id: &OperationId,
    payload_sha256: &Sha256Digest,
    kind: PhysicalCapabilityKind,
    expected_revision: RevocationRevision,
    expires_at: u64,
    verifier: &TestPhysicalVerifier,
    clock: &TestClock,
) -> Result<VerifiedUseToken<ExternalEffectCapability>, VerifiedUseError> {
    verify_physical_capability_use(
        capability,
        kind,
        operation_id,
        payload_sha256,
        runtime_context,
        expected_revision,
        PhysicalUseWindow::new(expires_at)?,
        verifier,
        clock,
    )
}

fn final_check<'a>(
    kind: PhysicalCapabilityKind,
    operation_id: &'a OperationId,
    payload_sha256: &'a Sha256Digest,
    runtime_context: &'a RuntimeAuthorityContext,
) -> PhysicalUseFinalCheck<'a> {
    PhysicalUseFinalCheck::new(kind, operation_id, payload_sha256, runtime_context)
}

#[test]
fn trusted_final_verification_and_durable_claim_issue_one_permit() {
    let (capability, runtime_context) = external_fixture(200);
    let operation_id = operation("operation:verified-use:success");
    let payload_sha256 = Sha256Digest::for_bytes(b"final-provider-payload");
    let expected_revision = revision(4);
    let verifier = TestPhysicalVerifier::allowing(expected_revision, 180);
    let clock = TestClock::at(110);
    let store = InMemoryClaimStore::default();

    let token = issue(
        &capability,
        &runtime_context,
        &operation_id,
        &payload_sha256,
        PhysicalCapabilityKind::ExternalEffect,
        expected_revision,
        190,
        &verifier,
        &clock,
    )
    .unwrap_or_else(|error| panic!("token must issue: {error}"));
    assert_eq!(token.verified_at_unix_seconds(), 110);
    assert_eq!(token.expires_at_unix_seconds(), 180);

    clock.set(150);
    let permit = token
        .consume_at_boundary(
            final_check(
                PhysicalCapabilityKind::ExternalEffect,
                &operation_id,
                &payload_sha256,
                &runtime_context,
            ),
            &verifier,
            &clock,
            &store,
        )
        .unwrap_or_else(|error| panic!("permit must issue: {error}"));
    let witness = permit.into_witness();
    witness
        .validate()
        .unwrap_or_else(|error| panic!("witness must validate: {error}"));
    assert_eq!(witness.schema_version(), VERIFIED_USE_SCHEMA_VERSION);
    assert_eq!(witness.operation_id(), &operation_id);
    assert_eq!(witness.final_payload_sha256(), &payload_sha256);
    assert_eq!(witness.revocation_revision(), expected_revision);
    assert_eq!(witness.verified_at_unix_seconds(), 110);
    assert_eq!(witness.crossed_at_unix_seconds(), 150);
    assert_eq!(witness.claim_revision(), 1);
    assert!(!witness.effect_completed());
    assert_eq!(clock.calls.load(Ordering::SeqCst), 2);
    assert_eq!(verifier.broad_calls.load(Ordering::SeqCst), 1);
    assert_eq!(verifier.physical_calls.load(Ordering::SeqCst), 2);
    assert_eq!(store.calls.load(Ordering::SeqCst), 1);
}

#[test]
fn durable_store_rejects_replay_and_same_operation_payload_conflict() {
    let (capability, runtime_context) = external_fixture(200);
    let operation_id = operation("operation:verified-use:single-use");
    let first_payload = Sha256Digest::for_bytes(b"payload-one");
    let second_payload = Sha256Digest::for_bytes(b"payload-two");
    let current_revision = revision(8);
    let verifier = TestPhysicalVerifier::allowing(current_revision, 190);
    let clock = TestClock::at(110);
    let store = InMemoryClaimStore::default();

    let first = issue(
        &capability,
        &runtime_context,
        &operation_id,
        &first_payload,
        PhysicalCapabilityKind::ExternalEffect,
        current_revision,
        180,
        &verifier,
        &clock,
    )
    .unwrap_or_else(|error| panic!("first token must issue: {error}"));
    let replay = issue(
        &capability,
        &runtime_context,
        &operation_id,
        &first_payload,
        PhysicalCapabilityKind::ExternalEffect,
        current_revision,
        180,
        &verifier,
        &clock,
    )
    .unwrap_or_else(|error| panic!("replay token must issue before claim: {error}"));
    let conflict = issue(
        &capability,
        &runtime_context,
        &operation_id,
        &second_payload,
        PhysicalCapabilityKind::ExternalEffect,
        current_revision,
        180,
        &verifier,
        &clock,
    )
    .unwrap_or_else(|error| panic!("conflict token must issue before claim: {error}"));

    clock.set(150);
    first
        .consume_at_boundary(
            final_check(
                PhysicalCapabilityKind::ExternalEffect,
                &operation_id,
                &first_payload,
                &runtime_context,
            ),
            &verifier,
            &clock,
            &store,
        )
        .unwrap_or_else(|error| panic!("first boundary claim must succeed: {error}"));
    assert!(matches!(
        replay.consume_at_boundary(
            final_check(
                PhysicalCapabilityKind::ExternalEffect,
                &operation_id,
                &first_payload,
                &runtime_context,
            ),
            &verifier,
            &clock,
            &store,
        ),
        Err(VerifiedUseError::ClaimStore(
            PhysicalUseClaimStoreError::AlreadyClaimed
        ))
    ));
    assert!(matches!(
        conflict.consume_at_boundary(
            final_check(
                PhysicalCapabilityKind::ExternalEffect,
                &operation_id,
                &second_payload,
                &runtime_context,
            ),
            &verifier,
            &clock,
            &store,
        ),
        Err(VerifiedUseError::ClaimStore(
            PhysicalUseClaimStoreError::OperationPayloadConflict
        ))
    ));
}

#[test]
fn final_revision_and_clock_are_rechecked_before_claim() {
    let (capability, runtime_context) = external_fixture(200);
    let operation_id = operation("operation:verified-use:final-recheck");
    let payload_sha256 = Sha256Digest::for_bytes(b"payload");
    let expected_revision = revision(4);
    let verifier = TestPhysicalVerifier::allowing(expected_revision, 180);
    let clock = TestClock::at(110);
    let store = InMemoryClaimStore::default();

    let token = issue(
        &capability,
        &runtime_context,
        &operation_id,
        &payload_sha256,
        PhysicalCapabilityKind::ExternalEffect,
        expected_revision,
        170,
        &verifier,
        &clock,
    )
    .unwrap_or_else(|error| panic!("token must issue: {error}"));
    verifier.set_revision(revision(5));
    clock.set(120);
    assert!(matches!(
        token.consume_at_boundary(
            final_check(
                PhysicalCapabilityKind::ExternalEffect,
                &operation_id,
                &payload_sha256,
                &runtime_context,
            ),
            &verifier,
            &clock,
            &store,
        ),
        Err(VerifiedUseError::FinalRevocationRevisionDrift {
            verified,
            current,
        }) if verified == revision(4) && current == revision(5)
    ));
    assert_eq!(store.calls.load(Ordering::SeqCst), 0);

    verifier.set_revision(expected_revision);
    clock.set(110);
    let rollback_operation = operation("operation:verified-use:rollback");
    let rollback = issue(
        &capability,
        &runtime_context,
        &rollback_operation,
        &payload_sha256,
        PhysicalCapabilityKind::ExternalEffect,
        expected_revision,
        170,
        &verifier,
        &clock,
    )
    .unwrap_or_else(|error| panic!("rollback token must issue: {error}"));
    clock.set(109);
    assert!(matches!(
        rollback.consume_at_boundary(
            final_check(
                PhysicalCapabilityKind::ExternalEffect,
                &rollback_operation,
                &payload_sha256,
                &runtime_context,
            ),
            &verifier,
            &clock,
            &store,
        ),
        Err(VerifiedUseError::ClockRollback {
            verified_at: 110,
            observed_at: 109,
        })
    ));
}

#[test]
fn caller_drift_local_authority_and_claim_failures_are_closed() {
    let (capability, runtime_context) = external_fixture(200);
    let operation_id = operation("operation:verified-use:drift");
    let other_operation = operation("operation:verified-use:other");
    let payload_sha256 = Sha256Digest::for_bytes(b"payload");
    let other_payload = Sha256Digest::for_bytes(b"other-payload");
    let current_revision = revision(8);
    let verifier = TestPhysicalVerifier::allowing(current_revision, 180);
    let clock = TestClock::at(110);
    let store = InMemoryClaimStore::default();
    let token = || {
        issue(
            &capability,
            &runtime_context,
            &operation_id,
            &payload_sha256,
            PhysicalCapabilityKind::ExternalEffect,
            current_revision,
            170,
            &verifier,
            &clock,
        )
        .unwrap_or_else(|error| panic!("token must issue: {error}"))
    };
    clock.set(120);

    assert!(matches!(
        token().consume_at_boundary(
            final_check(
                PhysicalCapabilityKind::ToolProcessSpawn,
                &operation_id,
                &payload_sha256,
                &runtime_context,
            ),
            &verifier,
            &clock,
            &store,
        ),
        Err(VerifiedUseError::FinalCapabilityKindDrift { .. })
    ));
    assert!(matches!(
        token().consume_at_boundary(
            final_check(
                PhysicalCapabilityKind::ExternalEffect,
                &other_operation,
                &payload_sha256,
                &runtime_context,
            ),
            &verifier,
            &clock,
            &store,
        ),
        Err(VerifiedUseError::FinalOperationDrift)
    ));
    assert!(matches!(
        token().consume_at_boundary(
            final_check(
                PhysicalCapabilityKind::ExternalEffect,
                &operation_id,
                &other_payload,
                &runtime_context,
            ),
            &verifier,
            &clock,
            &store,
        ),
        Err(VerifiedUseError::FinalPayloadDrift)
    ));

    let grant = AuthorityGrant::qualification_cognitive_write(agent_id(), GENERATION)
        .unwrap_or_else(|error| panic!("qualification grant must build: {error}"));
    let local_capability = grant
        .authorize::<CognitiveWriteCapability>()
        .unwrap_or_else(|error| panic!("local capability must authorize: {error}"));
    let local_context = RuntimeAuthorityContext::new(
        agent_id(),
        AUTHORITY_EPOCH,
        OWNER_EPOCH,
        GENERATION,
        Sha256Digest::for_bytes(b"local-fence"),
        grant.digest(),
    )
    .unwrap_or_else(|error| panic!("local context must build: {error}"));
    assert!(matches!(
        verify_physical_capability_use(
            &local_capability,
            PhysicalCapabilityKind::CognitiveStateWrite,
            &operation("operation:verified-use:local"),
            &payload_sha256,
            &local_context,
            revision(1),
            PhysicalUseWindow::new(150)
                .unwrap_or_else(|error| panic!("window must build: {error}")),
            &verifier,
            &clock,
        ),
        Err(VerifiedUseError::ExternalAuthorityRequired(
            PhysicalCapabilityKind::CognitiveStateWrite
        ))
    ));

    clock.set(110);
    let unavailable = InMemoryClaimStore::default();
    unavailable.unavailable.store(true, Ordering::SeqCst);
    let unavailable_operation = operation("operation:verified-use:unavailable");
    let unavailable_token = issue(
        &capability,
        &runtime_context,
        &unavailable_operation,
        &payload_sha256,
        PhysicalCapabilityKind::ExternalEffect,
        current_revision,
        170,
        &verifier,
        &clock,
    )
    .unwrap_or_else(|error| panic!("token must issue: {error}"));
    clock.set(120);
    assert!(matches!(
        unavailable_token.consume_at_boundary(
            final_check(
                PhysicalCapabilityKind::ExternalEffect,
                &unavailable_operation,
                &payload_sha256,
                &runtime_context,
            ),
            &verifier,
            &clock,
            &unavailable,
        ),
        Err(VerifiedUseError::ClaimStore(
            PhysicalUseClaimStoreError::Unavailable(reason)
        )) if reason == "disk unavailable"
    ));

    clock.set(110);
    let corrupt = InMemoryClaimStore::default();
    corrupt.corrupt_receipt.store(true, Ordering::SeqCst);
    let corrupt_operation = operation("operation:verified-use:corrupt");
    let corrupt_token = issue(
        &capability,
        &runtime_context,
        &corrupt_operation,
        &payload_sha256,
        PhysicalCapabilityKind::ExternalEffect,
        current_revision,
        170,
        &verifier,
        &clock,
    )
    .unwrap_or_else(|error| panic!("token must issue: {error}"));
    clock.set(120);
    assert!(matches!(
        corrupt_token.consume_at_boundary(
            final_check(
                PhysicalCapabilityKind::ExternalEffect,
                &corrupt_operation,
                &payload_sha256,
                &runtime_context,
            ),
            &verifier,
            &clock,
            &corrupt,
        ),
        Err(VerifiedUseError::ClaimReceiptIntegrityDrift)
    ));
}

#[test]
fn invalid_windows_revisions_and_trusted_clock_fail_closed() {
    assert!(matches!(
        RevocationRevision::new(0),
        Err(VerifiedUseError::InvalidRevocationRevision)
    ));
    assert!(matches!(
        PhysicalUseWindow::new(0),
        Err(VerifiedUseError::InvalidVerificationWindow)
    ));

    let (capability, runtime_context) = external_fixture(200);
    let verifier = TestPhysicalVerifier::allowing(revision(1), 180);
    let clock = TestClock::at(110);
    let payload_sha256 = Sha256Digest::for_bytes(b"payload");
    assert!(matches!(
        issue(
            &capability,
            &runtime_context,
            &operation("operation:verified-use:past"),
            &payload_sha256,
            PhysicalCapabilityKind::ExternalEffect,
            revision(1),
            110,
            &verifier,
            &clock,
        ),
        Err(VerifiedUseError::InvalidVerificationWindow)
    ));
    clock.reject.store(true, Ordering::SeqCst);
    assert!(matches!(
        issue(
            &capability,
            &runtime_context,
            &operation("operation:verified-use:clock"),
            &payload_sha256,
            PhysicalCapabilityKind::ExternalEffect,
            revision(1),
            170,
            &verifier,
            &clock,
        ),
        Err(VerifiedUseError::TrustedClockRejected(reason))
            if reason == "clock unavailable"
    ));
}
