use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::AuthorityAction;
use codex_hepta_contracts::AuthorityCapability;
use codex_hepta_contracts::AuthorityLeaseBinding;
use codex_hepta_contracts::Authorized;
use codex_hepta_contracts::CapabilityUseVerificationRequest;
use codex_hepta_contracts::CapabilityUseVerifier;
use codex_hepta_contracts::CapabilityVerificationRequest;
use codex_hepta_contracts::CapabilityVerifier;
use codex_hepta_contracts::ExternalEffectCapability;
use codex_hepta_contracts::FleetMutationCapability;
use codex_hepta_contracts::OperationId;
use codex_hepta_contracts::OperatorAcceptanceCapability;
use codex_hepta_contracts::PhysicalCapabilityKind;
use codex_hepta_contracts::PhysicalUseVerification;
use codex_hepta_contracts::PhysicalUseVerificationRequest;
use codex_hepta_contracts::PhysicalUseVerifier;
use codex_hepta_contracts::PhysicalUseWindow;
use codex_hepta_contracts::ReleasePromotionCapability;
use codex_hepta_contracts::RevocationRevision;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::authorize_verified_capability;
use codex_hepta_contracts::physical_boundaries::governed::B3_GOVERNED_BOUNDARIES_FLEET_MUTATION;
use codex_hepta_contracts::physical_boundaries::governed::B3_GOVERNED_BOUNDARIES_MATRIX_SEND;
use codex_hepta_contracts::physical_boundaries::governed::B3_GOVERNED_BOUNDARIES_OPERATOR_ACCEPTANCE;
use codex_hepta_contracts::physical_boundaries::governed::B3_GOVERNED_BOUNDARIES_PRODUCTION_CALLER;
use codex_hepta_contracts::physical_boundaries::governed::B3_GOVERNED_BOUNDARIES_PRODUCTION_WRITER;
use codex_hepta_contracts::physical_boundaries::governed::B3_GOVERNED_BOUNDARIES_PROMOTION;
use codex_hepta_contracts::physical_boundaries::governed::B3_GOVERNED_BOUNDARIES_RELEASE;
use codex_hepta_contracts::physical_boundaries::governed::B3_GOVERNED_BOUNDARIES_RUNTIME_REGISTERED;
use codex_hepta_contracts::physical_boundaries::governed::B3_GOVERNED_BOUNDARIES_SECRET_OPERATION;
use codex_hepta_contracts::physical_boundaries::governed::CandidateIdentity;
use codex_hepta_contracts::physical_boundaries::governed::CheckedFleetMutation;
use codex_hepta_contracts::physical_boundaries::governed::CheckedMatrixSend;
use codex_hepta_contracts::physical_boundaries::governed::CheckedOperatorAcceptance;
use codex_hepta_contracts::physical_boundaries::governed::CheckedReleasePromotion;
use codex_hepta_contracts::physical_boundaries::governed::CheckedSecretOperation;
use codex_hepta_contracts::physical_boundaries::governed::FleetMutationIntent;
use codex_hepta_contracts::physical_boundaries::governed::GovernedBoundaryAdapter;
use codex_hepta_contracts::physical_boundaries::governed::GovernedBoundaryDispatch;
use codex_hepta_contracts::physical_boundaries::governed::GovernedBoundaryError;
use codex_hepta_contracts::physical_boundaries::governed::GovernedBoundaryFuture;
use codex_hepta_contracts::physical_boundaries::governed::GovernedBoundaryOutcome;
use codex_hepta_contracts::physical_boundaries::governed::MatrixSendIntent;
use codex_hepta_contracts::physical_boundaries::governed::OperatorAcceptanceIntent;
use codex_hepta_contracts::physical_boundaries::governed::ReleasePromotionIntent;
use codex_hepta_contracts::physical_boundaries::governed::SecretOperationIntent;

const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const GENERATION: u64 = 9;
const REVISION: u64 = 31;
const CANDIDATE_COMMIT: &str = "24a2c1b733cc1d0f1288b39ffd42057dc6ade8ba";
const CANDIDATE_TREE: &str = "7ed08fb76eb8f0a30f3be926b66ff0d81fa46336";

fn agent_id() -> AgentId {
    AgentId::parse(AGENT_ID).unwrap_or_else(|error| panic!("AgentId must parse: {error}"))
}

fn digest(value: &[u8]) -> Sha256Digest {
    Sha256Digest::for_bytes(value)
}

fn operation(suffix: &str) -> OperationId {
    OperationId::parse(format!("operation:governed:{suffix}"))
        .unwrap_or_else(|error| panic!("operation must parse: {error}"))
}

fn revision() -> RevocationRevision {
    RevocationRevision::new(REVISION)
        .unwrap_or_else(|error| panic!("revision must build: {error}"))
}

fn candidate() -> CandidateIdentity {
    CandidateIdentity::new(CANDIDATE_COMMIT, CANDIDATE_TREE)
        .unwrap_or_else(|error| panic!("candidate must build: {error}"))
}

struct MintVerifier {
    expected_action: AuthorityAction,
}

impl CapabilityVerifier for MintVerifier {
    fn verify(&self, request: &CapabilityVerificationRequest<'_>) -> Result<(), String> {
        if request.action() != self.expected_action {
            return Err("unexpected governed capability action".to_string());
        }
        Ok(())
    }
}

fn capability<C>() -> Authorized<C>
where
    C: AuthorityCapability,
{
    let binding = AuthorityLeaseBinding::new(
        agent_id(),
        digest(format!("signed-governed-grant:{}", C::ACTION.as_str()).as_bytes()),
        37,
        41,
        GENERATION,
        digest(b"governed-fence"),
        200,
    )
    .unwrap_or_else(|error| panic!("binding must build: {error}"));
    authorize_verified_capability::<C, _>(
        binding,
        &agent_id(),
        GENERATION,
        100,
        &MintVerifier {
            expected_action: C::ACTION,
        },
    )
    .unwrap_or_else(|error| panic!("capability must mint: {error}"))
}

struct TestVerifier {
    current_revision: AtomicU64,
    physical_calls: AtomicUsize,
}

impl TestVerifier {
    fn new() -> Self {
        Self {
            current_revision: AtomicU64::new(REVISION),
            physical_calls: AtomicUsize::new(0),
        }
    }
}

impl CapabilityUseVerifier for TestVerifier {
    fn verify_use(&self, request: &CapabilityUseVerificationRequest<'_>) -> Result<(), String> {
        if !matches!(
            request.action(),
            AuthorityAction::ExternalEffect
                | AuthorityAction::MutateFleet
                | AuthorityAction::AcceptOperator
                | AuthorityAction::PromoteRelease
        ) {
            return Err("unexpected governed use action".to_string());
        }
        Ok(())
    }
}

impl PhysicalUseVerifier for TestVerifier {
    fn verify_physical_use(
        &self,
        request: &PhysicalUseVerificationRequest<'_>,
    ) -> Result<PhysicalUseVerification, String> {
        if !matches!(
            request.kind(),
            PhysicalCapabilityKind::SecretOperation
                | PhysicalCapabilityKind::MatrixSend
                | PhysicalCapabilityKind::FleetMutation
                | PhysicalCapabilityKind::OperatorAcceptance
                | PhysicalCapabilityKind::ReleasePromotion
        ) || request.action() != request.kind().authority_action()
            || request.operation_id().as_str().is_empty()
            || request.final_payload_sha256().as_str().is_empty()
        {
            return Err("governed physical binding drift".to_string());
        }
        let call = self.physical_calls.fetch_add(1, Ordering::SeqCst) + 1;
        let current = RevocationRevision::new(self.current_revision.load(Ordering::SeqCst))
            .map_err(|error| error.to_string())?;
        PhysicalUseVerification::new(
            current,
            190,
            digest(format!("governed-verifier-{call}").as_bytes()),
        )
        .map_err(|error| error.to_string())
    }
}

struct RecordingAdapter {
    calls: Arc<AtomicUsize>,
    witnessed: Arc<AtomicBool>,
    kinds: Arc<Mutex<Vec<PhysicalCapabilityKind>>>,
    fail_transport: bool,
}

impl GovernedBoundaryAdapter for RecordingAdapter {
    fn cross<'a>(
        &'a mut self,
        dispatch: GovernedBoundaryDispatch<'a>,
    ) -> GovernedBoundaryFuture<'a> {
        let calls = Arc::clone(&self.calls);
        let witnessed = Arc::clone(&self.witnessed);
        let kinds = Arc::clone(&self.kinds);
        let fail_transport = self.fail_transport;
        Box::pin(async move {
            if !witnessed.load(Ordering::SeqCst) {
                return Err("witness_not_persisted".to_string());
            }
            if dispatch.intent().final_payload_sha256()
                != &Sha256Digest::for_bytes(dispatch.final_payload())
            {
                return Err("payload_digest_drift".to_string());
            }
            if dispatch.verified_use_witness_sha256().as_str().is_empty() {
                return Err("missing_witness_digest".to_string());
            }
            calls.fetch_add(1, Ordering::SeqCst);
            kinds
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .push(dispatch.kind());
            if fail_transport {
                Err("external receipt acknowledgement lost".to_string())
            } else {
                GovernedBoundaryOutcome::external_receipt_recorded(
                    b"independently-issued-external-receipt",
                )
                .map_err(|error| error.to_string())
            }
        })
    }
}

fn adapter(
    calls: &Arc<AtomicUsize>,
    witnessed: &Arc<AtomicBool>,
    kinds: &Arc<Mutex<Vec<PhysicalCapabilityKind>>>,
    fail_transport: bool,
) -> RecordingAdapter {
    RecordingAdapter {
        calls: Arc::clone(calls),
        witnessed: Arc::clone(witnessed),
        kinds: Arc::clone(kinds),
        fail_transport,
    }
}

fn secret(payload: &[u8], suffix: &str, revision: u64) -> SecretOperationIntent {
    SecretOperationIntent::new(
        operation(suffix),
        digest(b"opaque-secret-ref"),
        "heptabao",
        "production-profile",
        "token-family-a",
        "model-provider-auth",
        "agent-runtime",
        revision,
        180,
        payload,
    )
    .unwrap_or_else(|error| panic!("secret intent must build: {error}"))
}

fn matrix(payload: &[u8], suffix: &str, generation: u64) -> MatrixSendIntent {
    MatrixSendIntent::new(
        operation(suffix),
        "!room:example.invalid",
        "$event-example",
        generation,
        digest(b"matrix-sender-identity"),
        digest(b"durable-outbox-envelope"),
        digest(b"homeserver-route"),
        payload,
    )
    .unwrap_or_else(|error| panic!("Matrix intent must build: {error}"))
}

fn fleet(payload: &[u8], suffix: &str, registry_revision: u64) -> FleetMutationIntent {
    FleetMutationIntent::new(
        operation(suffix),
        registry_revision,
        "release-2026-08-31",
        43,
        47,
        digest(b"immutable-release-identity"),
        digest(b"expected-prior-registry"),
        payload,
    )
    .unwrap_or_else(|error| panic!("Fleet intent must build: {error}"))
}

fn operator(
    payload: &[u8],
    suffix: &str,
    reviewer_identity: &[u8],
) -> Result<OperatorAcceptanceIntent, GovernedBoundaryError> {
    OperatorAcceptanceIntent::new(
        operation(suffix),
        candidate(),
        digest(b"complete-evidence-manifest"),
        53,
        digest(b"implementer-identity"),
        digest(reviewer_identity),
        digest(b"review-challenge"),
        120,
        180,
        payload,
    )
}

fn release(payload: &[u8], suffix: &str, sbom: &[u8]) -> ReleasePromotionIntent {
    ReleasePromotionIntent::new(
        operation(suffix),
        candidate(),
        "release-2026-08-31",
        "qualification-to-canary",
        digest(b"release-manifest"),
        digest(b"artifact-set"),
        digest(sbom),
        digest(b"migration-compatibility"),
        digest(b"rollback-evidence"),
        digest(b"independent-review-receipt"),
        digest(b"operator-acceptance-receipt"),
        59,
        payload,
    )
    .unwrap_or_else(|error| panic!("release intent must build: {error}"))
}

macro_rules! execute_success {
    ($boundary:expr, $intent:expr, $payload:expr, $kind:expr, $witnessed:expr) => {{
        $witnessed.store(false, Ordering::SeqCst);
        let (outcome, witness) = $boundary
            .execute_once(
                &$intent,
                $payload,
                revision(),
                PhysicalUseWindow::new(180)
                    .unwrap_or_else(|error| panic!("window must build: {error}")),
                &|| Ok(120),
                &|kind, scope, claim, token, request, claimed_at| {
                    assert_eq!(kind, $kind);
                    assert!(!scope.as_str().is_empty());
                    assert!(!claim.as_str().is_empty());
                    assert!(!token.as_str().is_empty());
                    assert!(!request.as_str().is_empty());
                    assert_eq!(claimed_at, 120);
                    Ok((1, digest(format!("claim-{}", kind.as_str()).as_bytes())))
                },
                |value| {
                    value.validate().map_err(|error| error.to_string())?;
                    $witnessed.store(true, Ordering::SeqCst);
                    Ok(())
                },
            )
            .await
            .unwrap_or_else(|error| panic!("governed boundary must succeed: {error}"));
        assert_eq!(witness.kind(), $kind);
        assert!(!witness.effect_completed());
        assert!(matches!(
            outcome,
            GovernedBoundaryOutcome::ExternalReceiptRecorded { .. }
        ));
    }};
}

#[tokio::test(flavor = "current_thread")]
async fn all_five_governed_boundaries_persist_witness_before_adapter() {
    let calls = Arc::new(AtomicUsize::new(0));
    let witnessed = Arc::new(AtomicBool::new(false));
    let kinds = Arc::new(Mutex::new(Vec::new()));

    let secret_payload = b"secret-provider-envelope";
    let secret_intent = secret(secret_payload, "secret-success", 61);
    let mut secret_boundary = CheckedSecretOperation::new(
        adapter(&calls, &witnessed, &kinds, false),
        capability::<ExternalEffectCapability>(),
        TestVerifier::new(),
    )
    .unwrap_or_else(|error| panic!("secret boundary must build: {error}"));
    execute_success!(
        secret_boundary,
        secret_intent,
        secret_payload,
        PhysicalCapabilityKind::SecretOperation,
        witnessed
    );

    let matrix_payload = b"matrix-event-envelope";
    let matrix_intent = matrix(matrix_payload, "matrix-success", 67);
    let mut matrix_boundary = CheckedMatrixSend::new(
        adapter(&calls, &witnessed, &kinds, false),
        capability::<ExternalEffectCapability>(),
        TestVerifier::new(),
    )
    .unwrap_or_else(|error| panic!("Matrix boundary must build: {error}"));
    execute_success!(
        matrix_boundary,
        matrix_intent,
        matrix_payload,
        PhysicalCapabilityKind::MatrixSend,
        witnessed
    );

    let fleet_payload = b"fleet-registry-mutation";
    let fleet_intent = fleet(fleet_payload, "fleet-success", 71);
    let mut fleet_boundary = CheckedFleetMutation::new(
        adapter(&calls, &witnessed, &kinds, false),
        capability::<FleetMutationCapability>(),
        TestVerifier::new(),
    )
    .unwrap_or_else(|error| panic!("Fleet boundary must build: {error}"));
    execute_success!(
        fleet_boundary,
        fleet_intent,
        fleet_payload,
        PhysicalCapabilityKind::FleetMutation,
        witnessed
    );

    let operator_payload = b"operator-acceptance-envelope";
    let operator_intent = operator(
        operator_payload,
        "operator-success",
        b"independent-reviewer-identity",
    )
    .unwrap_or_else(|error| panic!("operator intent must build: {error}"));
    let mut operator_boundary = CheckedOperatorAcceptance::new(
        adapter(&calls, &witnessed, &kinds, false),
        capability::<OperatorAcceptanceCapability>(),
        TestVerifier::new(),
    )
    .unwrap_or_else(|error| panic!("operator boundary must build: {error}"));
    execute_success!(
        operator_boundary,
        operator_intent,
        operator_payload,
        PhysicalCapabilityKind::OperatorAcceptance,
        witnessed
    );

    let release_payload = b"release-promotion-envelope";
    let release_intent = release(release_payload, "release-success", b"release-sbom");
    let mut release_boundary = CheckedReleasePromotion::new(
        adapter(&calls, &witnessed, &kinds, false),
        capability::<ReleasePromotionCapability>(),
        TestVerifier::new(),
    )
    .unwrap_or_else(|error| panic!("release boundary must build: {error}"));
    execute_success!(
        release_boundary,
        release_intent,
        release_payload,
        PhysicalCapabilityKind::ReleasePromotion,
        witnessed
    );

    assert_eq!(calls.load(Ordering::SeqCst), 5);
    assert_eq!(
        *kinds.lock().unwrap_or_else(|error| error.into_inner()),
        vec![
            PhysicalCapabilityKind::SecretOperation,
            PhysicalCapabilityKind::MatrixSend,
            PhysicalCapabilityKind::FleetMutation,
            PhysicalCapabilityKind::OperatorAcceptance,
            PhysicalCapabilityKind::ReleasePromotion,
        ]
    );
}

#[test]
fn governed_boundary_specific_facts_change_physical_digests() {
    let secret_a = secret(b"secret", "secret-digest", 1);
    let secret_b = secret(b"secret", "secret-digest", 2);
    assert_ne!(
        secret_a
            .physical_payload_sha256()
            .unwrap_or_else(|error| panic!("secret digest must build: {error}")),
        secret_b
            .physical_payload_sha256()
            .unwrap_or_else(|error| panic!("secret digest must build: {error}"))
    );

    let matrix_a = matrix(b"matrix", "matrix-digest", 1);
    let matrix_b = matrix(b"matrix", "matrix-digest", 2);
    assert_ne!(
        matrix_a
            .physical_payload_sha256()
            .unwrap_or_else(|error| panic!("Matrix digest must build: {error}")),
        matrix_b
            .physical_payload_sha256()
            .unwrap_or_else(|error| panic!("Matrix digest must build: {error}"))
    );

    let fleet_a = fleet(b"fleet", "fleet-digest", 1);
    let fleet_b = fleet(b"fleet", "fleet-digest", 2);
    assert_ne!(
        fleet_a
            .physical_payload_sha256()
            .unwrap_or_else(|error| panic!("Fleet digest must build: {error}")),
        fleet_b
            .physical_payload_sha256()
            .unwrap_or_else(|error| panic!("Fleet digest must build: {error}"))
    );

    let operator_a = operator(b"operator", "operator-digest", b"reviewer-a")
        .unwrap_or_else(|error| panic!("operator intent must build: {error}"));
    let operator_b = operator(b"operator", "operator-digest", b"reviewer-b")
        .unwrap_or_else(|error| panic!("operator intent must build: {error}"));
    assert_ne!(
        operator_a
            .physical_payload_sha256()
            .unwrap_or_else(|error| panic!("operator digest must build: {error}")),
        operator_b
            .physical_payload_sha256()
            .unwrap_or_else(|error| panic!("operator digest must build: {error}"))
    );

    let release_a = release(b"release", "release-digest", b"sbom-a");
    let release_b = release(b"release", "release-digest", b"sbom-b");
    assert_ne!(
        release_a
            .physical_payload_sha256()
            .unwrap_or_else(|error| panic!("release digest must build: {error}")),
        release_b
            .physical_payload_sha256()
            .unwrap_or_else(|error| panic!("release digest must build: {error}"))
    );
}

#[test]
fn operator_reviewer_identity_must_be_independent() {
    let identity = digest(b"implementer-identity");
    let result = OperatorAcceptanceIntent::new(
        operation("operator-self-review"),
        candidate(),
        digest(b"complete-evidence-manifest"),
        1,
        identity.clone(),
        identity,
        digest(b"review-challenge"),
        120,
        180,
        b"operator-envelope",
    );
    assert!(matches!(
        result,
        Err(GovernedBoundaryError::InvalidIdentity(
            "independent operator reviewer identity"
        ))
    ));
}

#[test]
fn secret_intent_serialization_contains_no_raw_secret_material() {
    let intent = secret(b"provider-envelope-without-secret", "secret-redaction", 1);
    let encoded = serde_json::to_string(&intent)
        .unwrap_or_else(|error| panic!("secret intent must serialize: {error}"));
    assert!(!encoded.contains("raw-secret-material"));
    assert!(!encoded.contains("private-key-bytes"));
    assert!(encoded.contains("opaque_secret_ref_sha256"));
}

#[tokio::test(flavor = "current_thread")]
async fn claim_witness_and_transport_fail_closed() {
    let calls = Arc::new(AtomicUsize::new(0));
    let witnessed = Arc::new(AtomicBool::new(false));
    let kinds = Arc::new(Mutex::new(Vec::new()));
    let payload = b"secret-provider-envelope";

    let mut claim_failure = CheckedSecretOperation::new(
        adapter(&calls, &witnessed, &kinds, false),
        capability::<ExternalEffectCapability>(),
        TestVerifier::new(),
    )
    .unwrap_or_else(|error| panic!("secret boundary must build: {error}"));
    let claim_intent = secret(payload, "claim-failure", 1);
    let result = claim_failure
        .execute_once(
            &claim_intent,
            payload,
            revision(),
            PhysicalUseWindow::new(180)
                .unwrap_or_else(|error| panic!("window must build: {error}")),
            &|| Ok(120),
            &|_, _, _, _, _, _| Err("already_claimed".to_string()),
            |_| Ok(()),
        )
        .await;
    assert!(matches!(result, Err(GovernedBoundaryError::VerifiedUse(_))));
    assert_eq!(calls.load(Ordering::SeqCst), 0);

    let mut witness_failure = CheckedSecretOperation::new(
        adapter(&calls, &witnessed, &kinds, false),
        capability::<ExternalEffectCapability>(),
        TestVerifier::new(),
    )
    .unwrap_or_else(|error| panic!("secret boundary must build: {error}"));
    let witness_intent = secret(payload, "witness-failure", 1);
    let result = witness_failure
        .execute_once(
            &witness_intent,
            payload,
            revision(),
            PhysicalUseWindow::new(180)
                .unwrap_or_else(|error| panic!("window must build: {error}")),
            &|| Ok(120),
            &|_, _, _, _, _, _| Ok((1, digest(b"claim-committed"))),
            |_| Err("witness_commit_failed".to_string()),
        )
        .await;
    assert!(matches!(
        result,
        Err(GovernedBoundaryError::WitnessPersistence(reason))
            if reason == "witness_commit_failed"
    ));
    assert_eq!(calls.load(Ordering::SeqCst), 0);

    let mut transport_failure = CheckedSecretOperation::new(
        adapter(&calls, &witnessed, &kinds, true),
        capability::<ExternalEffectCapability>(),
        TestVerifier::new(),
    )
    .unwrap_or_else(|error| panic!("secret boundary must build: {error}"));
    let transport_intent = secret(payload, "transport-failure", 1);
    let (outcome, _) = transport_failure
        .execute_once(
            &transport_intent,
            payload,
            revision(),
            PhysicalUseWindow::new(180)
                .unwrap_or_else(|error| panic!("window must build: {error}")),
            &|| Ok(120),
            &|_, _, _, _, _, _| Ok((1, digest(b"claim"))),
            |_| {
                witnessed.store(true, Ordering::SeqCst);
                Ok(())
            },
        )
        .await
        .unwrap_or_else(|error| panic!("uncertainty must be classified: {error}"));
    assert!(matches!(
        outcome,
        GovernedBoundaryOutcome::Indeterminate { ref reason_code }
            if reason_code == "external_receipt_acknowledgement_lost"
    ));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[test]
fn repository_source_grants_no_governed_authority() {
    assert!(!B3_GOVERNED_BOUNDARIES_RUNTIME_REGISTERED);
    assert!(!B3_GOVERNED_BOUNDARIES_PRODUCTION_CALLER);
    assert!(!B3_GOVERNED_BOUNDARIES_PRODUCTION_WRITER);
    assert!(!B3_GOVERNED_BOUNDARIES_SECRET_OPERATION);
    assert!(!B3_GOVERNED_BOUNDARIES_MATRIX_SEND);
    assert!(!B3_GOVERNED_BOUNDARIES_FLEET_MUTATION);
    assert!(!B3_GOVERNED_BOUNDARIES_OPERATOR_ACCEPTANCE);
    assert!(!B3_GOVERNED_BOUNDARIES_PROMOTION);
    assert!(!B3_GOVERNED_BOUNDARIES_RELEASE);
}
