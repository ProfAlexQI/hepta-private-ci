use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use pretty_assertions::assert_eq;

use crate::AgentId;
use crate::AuthorityAction;
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

struct TestPhysicalVerifier {
    current_revision: RevocationRevision,
    valid_until_unix_seconds: u64,
    broad_calls: AtomicUsize,
    physical_calls: AtomicUsize,
    reject_broad: bool,
    reject_physical: bool,
}

impl TestPhysicalVerifier {
    fn allowing(current_revision: RevocationRevision, valid_until_unix_seconds: u64) -> Self {
        Self {
            current_revision,
            valid_until_unix_seconds,
            broad_calls: AtomicUsize::new(0),
            physical_calls: AtomicUsize::new(0),
            reject_broad: false,
            reject_physical: false,
        }
    }
}

impl CapabilityUseVerifier for TestPhysicalVerifier {
    fn verify_use(&self, _request: &CapabilityUseVerificationRequest<'_>) -> Result<(), String> {
        self.broad_calls.fetch_add(1, Ordering::SeqCst);
        if self.reject_broad {
            Err("broad capability revoked".to_string())
        } else {
            Ok(())
        }
    }
}

impl PhysicalUseVerifier for TestPhysicalVerifier {
    fn verify_physical_use(
        &self,
        request: &PhysicalUseVerificationRequest<'_>,
    ) -> Result<PhysicalUseVerification, String> {
        self.physical_calls.fetch_add(1, Ordering::SeqCst);
        if self.reject_physical {
            return Err("final payload denied".to_string());
        }
        if request.operation_id().as_str().is_empty()
            || request.final_payload_sha256().as_str().is_empty()
            || request.runtime_context().generation() != GENERATION
        {
            return Err("final request binding drift".to_string());
        }
        PhysicalUseVerification::new(
            self.current_revision,
            self.valid_until_unix_seconds,
            Sha256Digest::for_bytes(b"authority-verifier-receipt"),
        )
        .map_err(|error| error.to_string())
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

fn issue_external_token(
    capability: &Authorized<ExternalEffectCapability>,
    runtime_context: &RuntimeAuthorityContext,
    operation_id: &OperationId,
    payload_sha256: &Sha256Digest,
    kind: PhysicalCapabilityKind,
    expected_revision: RevocationRevision,
    verified_at_unix_seconds: u64,
    requested_expires_at_unix_seconds: u64,
    verifier: &TestPhysicalVerifier,
) -> Result<VerifiedUseToken<ExternalEffectCapability>, VerifiedUseError> {
    verify_physical_capability_use(
        capability,
        PhysicalUseVerificationRequest::new(
            kind,
            operation_id,
            payload_sha256,
            runtime_context,
            expected_revision,
            PhysicalUseWindow::new(
                verified_at_unix_seconds,
                requested_expires_at_unix_seconds,
            )?,
        ),
        verifier,
    )
}

#[test]
fn exact_final_payload_issues_and_consumes_one_stable_token() {
    let (capability, runtime_context) = external_fixture(200);
    let operation_id = operation("operation:verified-use:success");
    let payload_sha256 = Sha256Digest::for_bytes(b"final-provider-payload");
    let expected_revision = revision(4);
    let verifier = TestPhysicalVerifier::allowing(expected_revision, 180);

    let token = issue_external_token(
        &capability,
        &runtime_context,
        &operation_id,
        &payload_sha256,
        PhysicalCapabilityKind::ExternalEffect,
        expected_revision,
        110,
        190,
        &verifier,
    )
    .unwrap_or_else(|error| panic!("token must issue: {error}"));
    assert_eq!(token.kind(), PhysicalCapabilityKind::ExternalEffect);
    assert_eq!(token.action(), AuthorityAction::ExternalEffect);
    assert_eq!(token.expires_at_unix_seconds(), 180);
    let first_token_sha256 = token.token_sha256().clone();

    let witness = token
        .consume(PhysicalUseFinalCheck::new(
            PhysicalCapabilityKind::ExternalEffect,
            &operation_id,
            &payload_sha256,
            &runtime_context,
            expected_revision,
            150,
        ))
        .unwrap_or_else(|error| panic!("token must consume: {error}"));
    witness
        .validate()
        .unwrap_or_else(|error| panic!("witness must validate: {error}"));
    assert_eq!(witness.schema_version(), VERIFIED_USE_SCHEMA_VERSION);
    assert_eq!(witness.operation_id(), &operation_id);
    assert_eq!(witness.final_payload_sha256(), &payload_sha256);
    assert_eq!(witness.revocation_revision(), expected_revision);
    assert_eq!(witness.verified_at_unix_seconds(), 110);
    assert_eq!(witness.expires_at_unix_seconds(), 180);
    assert_eq!(witness.crossed_at_unix_seconds(), 150);
    assert_eq!(witness.token_sha256(), &first_token_sha256);
    assert!(!witness.witness_sha256().as_str().is_empty());
    assert!(!serde_json::to_vec(&witness)
        .unwrap_or_else(|error| panic!("witness must serialize: {error}"))
        .is_empty());
    assert_eq!(verifier.broad_calls.load(Ordering::SeqCst), 1);
    assert_eq!(verifier.physical_calls.load(Ordering::SeqCst), 1);

    let second = issue_external_token(
        &capability,
        &runtime_context,
        &operation_id,
        &payload_sha256,
        PhysicalCapabilityKind::ExternalEffect,
        expected_revision,
        110,
        190,
        &verifier,
    )
    .unwrap_or_else(|error| panic!("second token must issue: {error}"))
    .consume(PhysicalUseFinalCheck::new(
        PhysicalCapabilityKind::ExternalEffect,
        &operation_id,
        &payload_sha256,
        &runtime_context,
        expected_revision,
        150,
    ))
    .unwrap_or_else(|error| panic!("second token must consume: {error}"));
    assert_eq!(second, witness);
}

#[test]
fn kind_action_mismatch_is_rejected_before_any_verifier_call() {
    let (capability, runtime_context) = external_fixture(200);
    let operation_id = operation("operation:verified-use:wrong-kind");
    let payload_sha256 = Sha256Digest::for_bytes(b"payload");
    let verifier = TestPhysicalVerifier::allowing(revision(4), 180);

    let result = issue_external_token(
        &capability,
        &runtime_context,
        &operation_id,
        &payload_sha256,
        PhysicalCapabilityKind::ProviderDispatch,
        revision(4),
        110,
        150,
        &verifier,
    );
    assert!(matches!(
        result,
        Err(VerifiedUseError::CapabilityKindActionMismatch {
            kind: PhysicalCapabilityKind::ProviderDispatch,
            capability_action: AuthorityAction::ExternalEffect,
        })
    ));
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
    let operation_id = operation("operation:verified-use:local-write");
    let payload_sha256 = Sha256Digest::for_bytes(b"local-write-payload");
    let verifier = TestPhysicalVerifier::allowing(revision(1), 180);

    let result = verify_physical_capability_use(
        &capability,
        PhysicalUseVerificationRequest::new(
            PhysicalCapabilityKind::CognitiveStateWrite,
            &operation_id,
            &payload_sha256,
            &runtime_context,
            revision(1),
            PhysicalUseWindow::new(110, 150)
                .unwrap_or_else(|error| panic!("window must build: {error}")),
        ),
        &verifier,
    );
    assert!(matches!(
        result,
        Err(VerifiedUseError::ExternalAuthorityRequired(
            PhysicalCapabilityKind::CognitiveStateWrite
        ))
    ));
    assert_eq!(verifier.broad_calls.load(Ordering::SeqCst), 0);
    assert_eq!(verifier.physical_calls.load(Ordering::SeqCst), 0);
}

#[test]
fn broad_authority_context_drift_and_expiry_fail_before_physical_verification() {
    let (capability, runtime_context) = external_fixture(200);
    let operation_id = operation("operation:verified-use:context-drift");
    let payload_sha256 = Sha256Digest::for_bytes(b"payload");
    let verifier = TestPhysicalVerifier::allowing(revision(2), 180);
    let changed_context = RuntimeAuthorityContext::new(
        agent_id(),
        AUTHORITY_EPOCH,
        OWNER_EPOCH + 1,
        GENERATION,
        Sha256Digest::for_bytes(b"changed-fence"),
        runtime_context.authority_grant_sha256().clone(),
    )
    .unwrap_or_else(|error| panic!("changed context must build: {error}"));

    let drift = issue_external_token(
        &capability,
        &changed_context,
        &operation_id,
        &payload_sha256,
        PhysicalCapabilityKind::ExternalEffect,
        revision(2),
        110,
        150,
        &verifier,
    );
    assert!(matches!(
        drift,
        Err(VerifiedUseError::Authority(
            AuthorityError::VerificationRejected(_)
        ))
    ));
    assert_eq!(verifier.broad_calls.load(Ordering::SeqCst), 0);
    assert_eq!(verifier.physical_calls.load(Ordering::SeqCst), 0);

    let expired = issue_external_token(
        &capability,
        &runtime_context,
        &operation_id,
        &payload_sha256,
        PhysicalCapabilityKind::ExternalEffect,
        revision(2),
        200,
        201,
        &verifier,
    );
    assert!(matches!(
        expired,
        Err(VerifiedUseError::Authority(AuthorityError::LeaseExpired {
            deadline: 200
        }))
    ));
    assert_eq!(verifier.physical_calls.load(Ordering::SeqCst), 0);
}

#[test]
fn requested_window_and_current_revocation_revision_are_fail_closed() {
    let (capability, runtime_context) = external_fixture(200);
    let operation_id = operation("operation:verified-use:revision");
    let payload_sha256 = Sha256Digest::for_bytes(b"payload");
    let expected_revision = revision(4);
    let verifier = TestPhysicalVerifier::allowing(revision(5), 180);

    let oversized = issue_external_token(
        &capability,
        &runtime_context,
        &operation_id,
        &payload_sha256,
        PhysicalCapabilityKind::ExternalEffect,
        expected_revision,
        110,
        201,
        &verifier,
    );
    assert!(matches!(
        oversized,
        Err(
            VerifiedUseError::RequestedWindowExceedsAuthorityLease {
                requested_expires_at: 201,
                authority_expires_at: 200,
            }
        )
    ));
    assert_eq!(verifier.broad_calls.load(Ordering::SeqCst), 1);
    assert_eq!(verifier.physical_calls.load(Ordering::SeqCst), 0);

    let revision_drift = issue_external_token(
        &capability,
        &runtime_context,
        &operation_id,
        &payload_sha256,
        PhysicalCapabilityKind::ExternalEffect,
        expected_revision,
        110,
        150,
        &verifier,
    );
    assert!(matches!(
        revision_drift,
        Err(VerifiedUseError::RevocationRevisionDrift {
            expected,
            current,
        }) if expected == revision(4) && current == revision(5)
    ));
    assert_eq!(verifier.broad_calls.load(Ordering::SeqCst), 2);
    assert_eq!(verifier.physical_calls.load(Ordering::SeqCst), 1);
}

#[test]
fn verifier_denial_and_expired_verifier_window_are_distinct() {
    let (capability, runtime_context) = external_fixture(200);
    let operation_id = operation("operation:verified-use:verifier-denial");
    let payload_sha256 = Sha256Digest::for_bytes(b"payload");
    let mut denied = TestPhysicalVerifier::allowing(revision(1), 180);
    denied.reject_physical = true;
    assert!(matches!(
        issue_external_token(
            &capability,
            &runtime_context,
            &operation_id,
            &payload_sha256,
            PhysicalCapabilityKind::ExternalEffect,
            revision(1),
            110,
            150,
            &denied,
        ),
        Err(VerifiedUseError::PhysicalVerificationRejected(reason))
            if reason == "final payload denied"
    ));

    let expired = TestPhysicalVerifier::allowing(revision(1), 110);
    assert!(matches!(
        issue_external_token(
            &capability,
            &runtime_context,
            &operation_id,
            &payload_sha256,
            PhysicalCapabilityKind::ExternalEffect,
            revision(1),
            110,
            150,
            &expired,
        ),
        Err(VerifiedUseError::VerifierValidityExpired { valid_until: 110 })
    ));
}

#[test]
fn final_operation_payload_context_kind_and_revision_drift_are_rejected() {
    let (capability, runtime_context) = external_fixture(200);
    let operation_id = operation("operation:verified-use:final-check");
    let other_operation = operation("operation:verified-use:other");
    let payload_sha256 = Sha256Digest::for_bytes(b"payload");
    let other_payload_sha256 = Sha256Digest::for_bytes(b"other-payload");
    let current_revision = revision(8);
    let verifier = TestPhysicalVerifier::allowing(current_revision, 180);
    let token = || {
        issue_external_token(
            &capability,
            &runtime_context,
            &operation_id,
            &payload_sha256,
            PhysicalCapabilityKind::ExternalEffect,
            current_revision,
            110,
            170,
            &verifier,
        )
        .unwrap_or_else(|error| panic!("token must issue: {error}"))
    };

    assert!(matches!(
        token().consume(PhysicalUseFinalCheck::new(
            PhysicalCapabilityKind::ToolProcessSpawn,
            &operation_id,
            &payload_sha256,
            &runtime_context,
            current_revision,
            120,
        )),
        Err(VerifiedUseError::FinalCapabilityKindDrift { .. })
    ));
    assert!(matches!(
        token().consume(PhysicalUseFinalCheck::new(
            PhysicalCapabilityKind::ExternalEffect,
            &other_operation,
            &payload_sha256,
            &runtime_context,
            current_revision,
            120,
        )),
        Err(VerifiedUseError::FinalOperationDrift)
    ));
    assert!(matches!(
        token().consume(PhysicalUseFinalCheck::new(
            PhysicalCapabilityKind::ExternalEffect,
            &operation_id,
            &other_payload_sha256,
            &runtime_context,
            current_revision,
            120,
        )),
        Err(VerifiedUseError::FinalPayloadDrift)
    ));

    let changed_context = RuntimeAuthorityContext::new(
        agent_id(),
        AUTHORITY_EPOCH,
        OWNER_EPOCH + 1,
        GENERATION,
        Sha256Digest::for_bytes(b"other-fence"),
        runtime_context.authority_grant_sha256().clone(),
    )
    .unwrap_or_else(|error| panic!("changed context must build: {error}"));
    assert!(matches!(
        token().consume(PhysicalUseFinalCheck::new(
            PhysicalCapabilityKind::ExternalEffect,
            &operation_id,
            &payload_sha256,
            &changed_context,
            current_revision,
            120,
        )),
        Err(VerifiedUseError::FinalRuntimeContextDrift)
    ));
    assert!(matches!(
        token().consume(PhysicalUseFinalCheck::new(
            PhysicalCapabilityKind::ExternalEffect,
            &operation_id,
            &payload_sha256,
            &runtime_context,
            revision(9),
            120,
        )),
        Err(VerifiedUseError::FinalRevocationRevisionDrift { .. })
    ));
}

#[test]
fn final_crossing_time_must_be_inside_the_verified_window() {
    let (capability, runtime_context) = external_fixture(200);
    let operation_id = operation("operation:verified-use:time");
    let payload_sha256 = Sha256Digest::for_bytes(b"payload");
    let current_revision = revision(3);
    let verifier = TestPhysicalVerifier::allowing(current_revision, 160);
    let token = || {
        issue_external_token(
            &capability,
            &runtime_context,
            &operation_id,
            &payload_sha256,
            PhysicalCapabilityKind::ExternalEffect,
            current_revision,
            110,
            170,
            &verifier,
        )
        .unwrap_or_else(|error| panic!("token must issue: {error}"))
    };

    assert!(matches!(
        token().consume(PhysicalUseFinalCheck::new(
            PhysicalCapabilityKind::ExternalEffect,
            &operation_id,
            &payload_sha256,
            &runtime_context,
            current_revision,
            109,
        )),
        Err(VerifiedUseError::CrossedBeforeVerification {
            verified_at: 110,
            crossed_at: 109,
        })
    ));
    assert!(matches!(
        token().consume(PhysicalUseFinalCheck::new(
            PhysicalCapabilityKind::ExternalEffect,
            &operation_id,
            &payload_sha256,
            &runtime_context,
            current_revision,
            160,
        )),
        Err(VerifiedUseError::TokenExpired {
            expires_at: 160,
            crossed_at: 160,
        })
    ));
}

#[test]
fn revocation_revision_and_window_reject_zero_or_empty_values() {
    assert!(matches!(
        RevocationRevision::new(0),
        Err(VerifiedUseError::InvalidRevocationRevision)
    ));
    assert!(matches!(
        PhysicalUseWindow::new(0, 1),
        Err(VerifiedUseError::InvalidVerificationWindow)
    ));
    assert!(matches!(
        PhysicalUseWindow::new(10, 10),
        Err(VerifiedUseError::InvalidVerificationWindow)
    ));
}
