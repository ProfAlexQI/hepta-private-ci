use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::AuthorityGrant;
use codex_hepta_contracts::AuthorityLeaseBinding;
use codex_hepta_contracts::Authorized;
use codex_hepta_contracts::CapabilityUseVerificationRequest;
use codex_hepta_contracts::CapabilityUseVerifier;
use codex_hepta_contracts::CapabilityVerificationRequest;
use codex_hepta_contracts::CapabilityVerifier;
use codex_hepta_contracts::CognitiveWriteCapability;
use codex_hepta_contracts::ExternalEffectCapability;
use codex_hepta_contracts::OperationId;
use codex_hepta_contracts::PhysicalCapabilityKind;
use codex_hepta_contracts::PhysicalUseClaimReceipt;
use codex_hepta_contracts::PhysicalUseClaimRequest;
use codex_hepta_contracts::PhysicalUseClaimStore;
use codex_hepta_contracts::PhysicalUseClaimStoreError;
use codex_hepta_contracts::PhysicalUseFinalCheck;
use codex_hepta_contracts::PhysicalUseVerification;
use codex_hepta_contracts::PhysicalUseVerificationRequest;
use codex_hepta_contracts::PhysicalUseVerifier;
use codex_hepta_contracts::PhysicalUseWindow;
use codex_hepta_contracts::RevocationRevision;
use codex_hepta_contracts::RuntimeAuthorityContext;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::TrustedPhysicalClock;
use codex_hepta_contracts::VerifiedUseError;
use codex_hepta_contracts::VerifiedUseToken;
use codex_hepta_contracts::authorize_verified_capability;
use codex_hepta_contracts::verify_physical_capability_use;

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
    current_revision: AtomicU64,
    valid_until: AtomicU64,
    reject_physical: AtomicBool,
    broad_calls: AtomicUsize,
    physical_calls: AtomicUsize,
}

impl TestPhysicalVerifier {
    fn allowing(current_revision: RevocationRevision, valid_until: u64) -> Self {
        Self {
            current_revision: AtomicU64::new(current_revision.get()),
            valid_until: AtomicU64::new(valid_until),
            reject_physical: AtomicBool::new(false),
            broad_calls: AtomicUsize::new(0),
            physical_calls: AtomicUsize::new(0),
        }
    }

    fn set_revision(&self, revision: RevocationRevision) {
        self.current_revision
            .store(revision.get(), Ordering::SeqCst);
    }

    fn set_reject_physical(&self, reject: bool) {
        self.reject_physical.store(reject, Ordering::SeqCst);
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
        if self.reject_physical.load(Ordering::SeqCst) {
            return Err("physical authority denied".to_string());
        }
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
            revision(self.current_revision.load(Ordering::SeqCst)),
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
    calls: AtomicUsize,
}

impl PhysicalUseClaimStore for InMemoryClaimStore {
    fn claim_once(
        &self,
        request: &PhysicalUseClaimRequest<'_>,
    ) -> Result<PhysicalUseClaimReceipt, PhysicalUseClaimStoreError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
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
            return if existing == &claim {
                Err(PhysicalUseClaimStoreError::AlreadyClaimed)
            } else {
                Err(PhysicalUseClaimStoreError::OperationPayloadConflict)
            };
        }
        claims.insert(scope, claim);

        let claim_revision = self.next_revision.fetch_add(1, Ordering::SeqCst) + 1;
        PhysicalUseClaimReceipt::new(
            request.claim_key().clone(),
            claim_revision,
            request.claimed_at_unix_seconds(),
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
fn exact_final_payload_issues_and_consumes_one_stable_token() {
    let (capability, runtime_context) = external_fixture(200);
    let operation_id = operation("operation:verified-use:public-success");
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
    let token_sha256 = token.token_sha256().clone();
    clock.set(150);
    let witness = token
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
        .unwrap_or_else(|error| panic!("permit must issue: {error}"))
        .into_witness();

    assert_eq!(witness.token_sha256(), &token_sha256);
    assert_eq!(witness.operation_id(), &operation_id);
    assert_eq!(witness.final_payload_sha256(), &payload_sha256);
    assert_eq!(witness.claim_revision(), 1);
    assert!(!witness.effect_completed());
    assert_eq!(verifier.broad_calls.load(Ordering::SeqCst), 1);
    assert_eq!(verifier.physical_calls.load(Ordering::SeqCst), 2);
    assert_eq!(store.calls.load(Ordering::SeqCst), 1);
}

#[test]
fn kind_action_mismatch_is_rejected_before_any_verifier_call() {
    let (capability, runtime_context) = external_fixture(200);
    let verifier = TestPhysicalVerifier::allowing(revision(1), 180);
    let clock = TestClock::at(110);
    let operation_id = operation("operation:verified-use:kind-action-mismatch");
    let payload_sha256 = Sha256Digest::for_bytes(b"payload");

    assert!(matches!(
        verify_physical_capability_use(
            &capability,
            PhysicalCapabilityKind::ModelInvocation,
            &operation_id,
            &payload_sha256,
            &runtime_context,
            revision(1),
            PhysicalUseWindow::new(170)
                .unwrap_or_else(|error| panic!("window must build: {error}")),
            &verifier,
            &clock,
        ),
        Err(VerifiedUseError::CapabilityKindActionMismatch {
            kind: PhysicalCapabilityKind::ModelInvocation,
            ..
        })
    ));
    assert_eq!(clock.calls.load(Ordering::SeqCst), 0);
    assert_eq!(verifier.broad_calls.load(Ordering::SeqCst), 0);
    assert_eq!(verifier.physical_calls.load(Ordering::SeqCst), 0);
}

#[test]
fn local_broad_capability_cannot_cross_a_physical_write_boundary() {
    let grant = AuthorityGrant::qualification_cognitive_write(agent_id(), GENERATION)
        .unwrap_or_else(|error| panic!("qualification grant must build: {error}"));
    let capability = grant
        .authorize::<CognitiveWriteCapability>()
        .unwrap_or_else(|error| panic!("local capability must authorize: {error}"));
    let runtime_context = RuntimeAuthorityContext::new(
        agent_id(),
        AUTHORITY_EPOCH,
        OWNER_EPOCH,
        GENERATION,
        Sha256Digest::for_bytes(b"local-fence"),
        grant.digest(),
    )
    .unwrap_or_else(|error| panic!("local context must build: {error}"));
    let verifier = TestPhysicalVerifier::allowing(revision(1), 180);
    let clock = TestClock::at(110);
    let operation_id = operation("operation:verified-use:local-explicit");
    let payload_sha256 = Sha256Digest::for_bytes(b"payload");

    assert!(matches!(
        verify_physical_capability_use(
            &capability,
            PhysicalCapabilityKind::CognitiveStateWrite,
            &operation_id,
            &payload_sha256,
            &runtime_context,
            revision(1),
            PhysicalUseWindow::new(170)
                .unwrap_or_else(|error| panic!("window must build: {error}")),
            &verifier,
            &clock,
        ),
        Err(VerifiedUseError::ExternalAuthorityRequired(
            PhysicalCapabilityKind::CognitiveStateWrite
        ))
    ));
    assert_eq!(clock.calls.load(Ordering::SeqCst), 0);
    assert_eq!(verifier.broad_calls.load(Ordering::SeqCst), 0);
    assert_eq!(verifier.physical_calls.load(Ordering::SeqCst), 0);
}

#[test]
fn broad_authority_context_drift_and_expiry_fail_before_physical_verification() {
    let (capability, _runtime_context) = external_fixture(200);
    let drifted_context = RuntimeAuthorityContext::new(
        agent_id(),
        AUTHORITY_EPOCH,
        OWNER_EPOCH,
        GENERATION,
        Sha256Digest::for_bytes(b"drifted-effect-fence"),
        Sha256Digest::for_bytes(b"signed-effect-grant"),
    )
    .unwrap_or_else(|error| panic!("drifted context must build: {error}"));
    let verifier = TestPhysicalVerifier::allowing(revision(1), 180);
    let clock = TestClock::at(110);
    let payload_sha256 = Sha256Digest::for_bytes(b"payload");
    assert!(matches!(
        issue(
            &capability,
            &drifted_context,
            &operation("operation:verified-use:context-drift"),
            &payload_sha256,
            PhysicalCapabilityKind::ExternalEffect,
            revision(1),
            170,
            &verifier,
            &clock,
        ),
        Err(VerifiedUseError::Authority(_))
    ));
    assert_eq!(verifier.physical_calls.load(Ordering::SeqCst), 0);

    let (expired_capability, expired_context) = external_fixture(120);
    let expired_verifier = TestPhysicalVerifier::allowing(revision(1), 180);
    let expired_clock = TestClock::at(121);
    assert!(matches!(
        issue(
            &expired_capability,
            &expired_context,
            &operation("operation:verified-use:expired-authority"),
            &payload_sha256,
            PhysicalCapabilityKind::ExternalEffect,
            revision(1),
            130,
            &expired_verifier,
            &expired_clock,
        ),
        Err(VerifiedUseError::Authority(_))
    ));
    assert_eq!(expired_verifier.physical_calls.load(Ordering::SeqCst), 0);
}

#[test]
fn requested_window_and_current_revocation_revision_are_fail_closed() {
    let (capability, runtime_context) = external_fixture(150);
    let verifier = TestPhysicalVerifier::allowing(revision(1), 180);
    let clock = TestClock::at(110);
    let payload_sha256 = Sha256Digest::for_bytes(b"payload");
    assert!(matches!(
        issue(
            &capability,
            &runtime_context,
            &operation("operation:verified-use:window-exceeds-lease"),
            &payload_sha256,
            PhysicalCapabilityKind::ExternalEffect,
            revision(1),
            151,
            &verifier,
            &clock,
        ),
        Err(VerifiedUseError::RequestedWindowExceedsAuthorityLease {
            requested_expires_at: 151,
            authority_expires_at: 150,
        })
    ));
    assert_eq!(verifier.physical_calls.load(Ordering::SeqCst), 0);

    let (capability, runtime_context) = external_fixture(200);
    let revision_verifier = TestPhysicalVerifier::allowing(revision(2), 180);
    assert!(matches!(
        issue(
            &capability,
            &runtime_context,
            &operation("operation:verified-use:revision-drift"),
            &payload_sha256,
            PhysicalCapabilityKind::ExternalEffect,
            revision(1),
            170,
            &revision_verifier,
            &clock,
        ),
        Err(VerifiedUseError::RevocationRevisionDrift { expected, current })
            if expected == revision(1) && current == revision(2)
    ));
}

#[test]
fn verifier_denial_and_expired_verifier_window_are_distinct() {
    let (capability, runtime_context) = external_fixture(200);
    let payload_sha256 = Sha256Digest::for_bytes(b"payload");
    let clock = TestClock::at(110);

    let denied = TestPhysicalVerifier::allowing(revision(1), 180);
    denied.set_reject_physical(true);
    assert!(matches!(
        issue(
            &capability,
            &runtime_context,
            &operation("operation:verified-use:physical-denied"),
            &payload_sha256,
            PhysicalCapabilityKind::ExternalEffect,
            revision(1),
            170,
            &denied,
            &clock,
        ),
        Err(VerifiedUseError::PhysicalVerificationRejected(reason))
            if reason == "physical authority denied"
    ));

    let expired = TestPhysicalVerifier::allowing(revision(1), 110);
    assert!(matches!(
        issue(
            &capability,
            &runtime_context,
            &operation("operation:verified-use:verifier-expired"),
            &payload_sha256,
            PhysicalCapabilityKind::ExternalEffect,
            revision(1),
            170,
            &expired,
            &clock,
        ),
        Err(VerifiedUseError::VerifierValidityExpired { valid_until: 110 })
    ));
}

#[test]
fn final_operation_payload_context_kind_and_revision_drift_are_rejected() {
    let (capability, runtime_context) = external_fixture(200);
    let operation_id = operation("operation:verified-use:final-drift");
    let other_operation = operation("operation:verified-use:final-drift-other");
    let payload_sha256 = Sha256Digest::for_bytes(b"payload");
    let other_payload = Sha256Digest::for_bytes(b"other-payload");
    let expected_revision = revision(1);
    let verifier = TestPhysicalVerifier::allowing(expected_revision, 180);
    let clock = TestClock::at(110);
    let store = InMemoryClaimStore::default();
    let issue_token = || {
        clock.set(110);
        issue(
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
        .unwrap_or_else(|error| panic!("token must issue: {error}"))
    };

    let kind_token = issue_token();
    clock.set(120);
    assert!(matches!(
        kind_token.consume_at_boundary(
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

    let operation_token = issue_token();
    clock.set(120);
    assert!(matches!(
        operation_token.consume_at_boundary(
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

    let payload_token = issue_token();
    clock.set(120);
    assert!(matches!(
        payload_token.consume_at_boundary(
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

    let drifted_context = RuntimeAuthorityContext::new(
        agent_id(),
        AUTHORITY_EPOCH,
        OWNER_EPOCH,
        GENERATION,
        Sha256Digest::for_bytes(b"drifted-effect-fence"),
        Sha256Digest::for_bytes(b"signed-effect-grant"),
    )
    .unwrap_or_else(|error| panic!("drifted context must build: {error}"));
    let context_token = issue_token();
    clock.set(120);
    assert!(matches!(
        context_token.consume_at_boundary(
            final_check(
                PhysicalCapabilityKind::ExternalEffect,
                &operation_id,
                &payload_sha256,
                &drifted_context,
            ),
            &verifier,
            &clock,
            &store,
        ),
        Err(VerifiedUseError::FinalRuntimeContextDrift)
    ));

    let revision_token = issue_token();
    verifier.set_revision(revision(2));
    clock.set(120);
    assert!(matches!(
        revision_token.consume_at_boundary(
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
        Err(VerifiedUseError::FinalRevocationRevisionDrift { verified, current })
            if verified == revision(1) && current == revision(2)
    ));
    assert_eq!(store.calls.load(Ordering::SeqCst), 0);
}

#[test]
fn final_crossing_time_must_be_inside_the_verified_window() {
    let (capability, runtime_context) = external_fixture(200);
    let operation_id = operation("operation:verified-use:final-expiry");
    let payload_sha256 = Sha256Digest::for_bytes(b"payload");
    let expected_revision = revision(1);
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
    clock.set(170);
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
        Err(VerifiedUseError::TokenExpired {
            expires_at: 170,
            crossed_at: 170,
        })
    ));
    assert_eq!(store.calls.load(Ordering::SeqCst), 0);
}

#[test]
fn revocation_revision_and_window_reject_zero_or_empty_values() {
    assert!(matches!(
        RevocationRevision::new(0),
        Err(VerifiedUseError::InvalidRevocationRevision)
    ));
    assert!(matches!(
        PhysicalUseWindow::new(0),
        Err(VerifiedUseError::InvalidVerificationWindow)
    ));
}
