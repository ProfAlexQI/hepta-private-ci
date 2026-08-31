use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::AuthorityAction;
use codex_hepta_contracts::AuthorityLeaseBinding;
use codex_hepta_contracts::CapabilityUseVerificationRequest;
use codex_hepta_contracts::CapabilityUseVerifier;
use codex_hepta_contracts::CapabilityVerificationRequest;
use codex_hepta_contracts::CapabilityVerifier;
use codex_hepta_contracts::CheckedModelInvocation;
use codex_hepta_contracts::ModelInvocationAdapter;
use codex_hepta_contracts::ModelInvocationCapability;
use codex_hepta_contracts::ModelInvocationDispatch;
use codex_hepta_contracts::ModelInvocationError;
use codex_hepta_contracts::ModelInvocationFuture;
use codex_hepta_contracts::ModelInvocationIntent;
use codex_hepta_contracts::ModelInvocationOutcome;
use codex_hepta_contracts::ModelInvocationRoute;
use codex_hepta_contracts::OperationId;
use codex_hepta_contracts::PhysicalCapabilityKind;
use codex_hepta_contracts::PhysicalUseVerification;
use codex_hepta_contracts::PhysicalUseVerificationRequest;
use codex_hepta_contracts::PhysicalUseVerifier;
use codex_hepta_contracts::PhysicalUseWindow;
use codex_hepta_contracts::RevocationRevision;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::authorize_verified_capability;

const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const GENERATION: u64 = 3;
const REVISION: u64 = 7;

fn agent_id() -> AgentId {
    AgentId::parse(AGENT_ID).unwrap_or_else(|error| panic!("AgentId must parse: {error}"))
}

fn revision() -> RevocationRevision {
    RevocationRevision::new(REVISION)
        .unwrap_or_else(|error| panic!("revision must build: {error}"))
}

struct MintVerifier;

impl CapabilityVerifier for MintVerifier {
    fn verify(&self, request: &CapabilityVerificationRequest<'_>) -> Result<(), String> {
        if request.action() != AuthorityAction::InvokeModel {
            return Err("unexpected model mint action".to_string());
        }
        Ok(())
    }
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
        if request.action() != AuthorityAction::InvokeModel {
            return Err("unexpected model use action".to_string());
        }
        Ok(())
    }
}

impl PhysicalUseVerifier for TestVerifier {
    fn verify_physical_use(
        &self,
        request: &PhysicalUseVerificationRequest<'_>,
    ) -> Result<PhysicalUseVerification, String> {
        let call = self.physical_calls.fetch_add(1, Ordering::SeqCst) + 1;
        if request.kind() != PhysicalCapabilityKind::ModelInvocation
            || request.action() != AuthorityAction::InvokeModel
            || request.operation_id().as_str().is_empty()
            || request.final_payload_sha256().as_str().is_empty()
        {
            return Err("model physical binding drift".to_string());
        }
        let current = RevocationRevision::new(self.current_revision.load(Ordering::SeqCst))
            .map_err(|error| error.to_string())?;
        PhysicalUseVerification::new(
            current,
            190,
            Sha256Digest::for_bytes(format!("model-verifier-{call}").as_bytes()),
        )
        .map_err(|error| error.to_string())
    }
}

struct RecordingAdapter {
    calls: Arc<AtomicUsize>,
    witness_persisted: Arc<AtomicBool>,
    fail_transport: bool,
}

impl ModelInvocationAdapter for RecordingAdapter {
    fn invoke<'a>(&'a mut self, dispatch: ModelInvocationDispatch<'a>) -> ModelInvocationFuture<'a> {
        let calls = Arc::clone(&self.calls);
        let witness_persisted = Arc::clone(&self.witness_persisted);
        let fail_transport = self.fail_transport;
        Box::pin(async move {
            if !witness_persisted.load(Ordering::SeqCst) {
                return Err("witness_not_persisted".to_string());
            }
            if dispatch.intent().wire_payload_sha256()
                != &Sha256Digest::for_bytes(dispatch.wire_payload())
            {
                return Err("payload_digest_drift".to_string());
            }
            if dispatch.verified_use_witness_sha256().as_str().is_empty() {
                return Err("missing_witness_digest".to_string());
            }
            calls.fetch_add(1, Ordering::SeqCst);
            if fail_transport {
                Err("timeout after write".to_string())
            } else {
                ModelInvocationOutcome::completed(b"bounded-model-response")
                    .map_err(|error| error.to_string())
            }
        })
    }
}

fn coordinator(
    calls: Arc<AtomicUsize>,
    witness_persisted: Arc<AtomicBool>,
    fail_transport: bool,
) -> CheckedModelInvocation<RecordingAdapter, TestVerifier> {
    let binding = AuthorityLeaseBinding::new(
        agent_id(),
        Sha256Digest::for_bytes(b"signed-model-grant"),
        11,
        13,
        GENERATION,
        Sha256Digest::for_bytes(b"model-fence"),
        200,
    )
    .unwrap_or_else(|error| panic!("binding must build: {error}"));
    let capability = authorize_verified_capability::<ModelInvocationCapability, _>(
        binding,
        &agent_id(),
        GENERATION,
        100,
        &MintVerifier,
    )
    .unwrap_or_else(|error| panic!("capability must mint: {error}"));
    CheckedModelInvocation::new(
        RecordingAdapter {
            calls,
            witness_persisted,
            fail_transport,
        },
        capability,
        TestVerifier::new(),
    )
    .unwrap_or_else(|error| panic!("coordinator must build: {error}"))
}

fn intent(payload: &[u8], suffix: &str) -> ModelInvocationIntent {
    let route = ModelInvocationRoute::new(
        "provider-local",
        "model-fixed-v1",
        Sha256Digest::for_bytes(b"loopback-endpoint"),
        Sha256Digest::for_bytes(b"fixed-routing-policy"),
    )
    .unwrap_or_else(|error| panic!("route must build: {error}"));
    ModelInvocationIntent::new(
        OperationId::parse(format!("operation:model:{suffix}"))
            .unwrap_or_else(|error| panic!("operation must parse: {error}")),
        route,
        payload,
        "application/json",
        Sha256Digest::for_bytes(b"strict-response-contract"),
        Some(Sha256Digest::for_bytes(b"strict-tool-contract")),
        true,
    )
    .unwrap_or_else(|error| panic!("intent must build: {error}"))
}

#[tokio::test(flavor = "current_thread")]
async fn witness_is_committed_before_exact_adapter_call() {
    let calls = Arc::new(AtomicUsize::new(0));
    let persisted = Arc::new(AtomicBool::new(false));
    let mut coordinator = coordinator(Arc::clone(&calls), Arc::clone(&persisted), false);
    let payload = br#"{"model":"model-fixed-v1","input":"digest-only-test"}"#;
    let intent = intent(payload, "success");
    let claim_calls = AtomicUsize::new(0);

    let (outcome, witness) = coordinator
        .invoke_once(
            &intent,
            payload,
            revision(),
            PhysicalUseWindow::new(180)
                .unwrap_or_else(|error| panic!("window must build: {error}")),
            &|| Ok(120),
            &|kind, scope, claim, token, request, claimed_at| {
                assert_eq!(kind, PhysicalCapabilityKind::ModelInvocation);
                assert!(!scope.as_str().is_empty());
                assert!(!claim.as_str().is_empty());
                assert!(!token.as_str().is_empty());
                assert!(!request.as_str().is_empty());
                assert_eq!(claimed_at, 120);
                claim_calls.fetch_add(1, Ordering::SeqCst);
                Ok((1, Sha256Digest::for_bytes(b"model-claim-receipt")))
            },
            |witness| {
                witness
                    .validate()
                    .map_err(|error| error.to_string())?;
                persisted.store(true, Ordering::SeqCst);
                Ok(())
            },
        )
        .await
        .unwrap_or_else(|error| panic!("model invocation must succeed: {error}"));

    assert!(matches!(outcome, ModelInvocationOutcome::Completed { .. }));
    assert_eq!(witness.kind(), PhysicalCapabilityKind::ModelInvocation);
    assert!(!witness.effect_completed());
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(claim_calls.load(Ordering::SeqCst), 1);
    assert!(persisted.load(Ordering::SeqCst));
}

#[tokio::test(flavor = "current_thread")]
async fn payload_drift_fails_before_claim_and_adapter() {
    let calls = Arc::new(AtomicUsize::new(0));
    let persisted = Arc::new(AtomicBool::new(false));
    let mut coordinator = coordinator(Arc::clone(&calls), Arc::clone(&persisted), false);
    let intent = intent(b"payload-a", "payload-drift");
    let claim_calls = AtomicUsize::new(0);

    let result = coordinator
        .invoke_once(
            &intent,
            b"payload-b",
            revision(),
            PhysicalUseWindow::new(180)
                .unwrap_or_else(|error| panic!("window must build: {error}")),
            &|| Ok(120),
            &|_, _, _, _, _, _| {
                claim_calls.fetch_add(1, Ordering::SeqCst);
                Ok((1, Sha256Digest::for_bytes(b"unreachable")))
            },
            |_| Ok(()),
        )
        .await;

    assert!(matches!(result, Err(ModelInvocationError::WirePayloadDrift)));
    assert_eq!(claim_calls.load(Ordering::SeqCst), 0);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert!(!persisted.load(Ordering::SeqCst));
}

#[tokio::test(flavor = "current_thread")]
async fn witness_persistence_failure_blocks_adapter_and_consumes_claim() {
    let calls = Arc::new(AtomicUsize::new(0));
    let persisted = Arc::new(AtomicBool::new(false));
    let mut coordinator = coordinator(Arc::clone(&calls), Arc::clone(&persisted), false);
    let payload = b"final-payload";
    let intent = intent(payload, "witness-failure");
    let claim_calls = AtomicUsize::new(0);

    let result = coordinator
        .invoke_once(
            &intent,
            payload,
            revision(),
            PhysicalUseWindow::new(180)
                .unwrap_or_else(|error| panic!("window must build: {error}")),
            &|| Ok(120),
            &|_, _, _, _, _, _| {
                claim_calls.fetch_add(1, Ordering::SeqCst);
                Ok((1, Sha256Digest::for_bytes(b"claim-committed")))
            },
            |_| Err("sqlite_commit_failed".to_string()),
        )
        .await;

    assert!(matches!(
        result,
        Err(ModelInvocationError::WitnessPersistence(reason))
            if reason == "sqlite_commit_failed"
    ));
    assert_eq!(claim_calls.load(Ordering::SeqCst), 1);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}

#[tokio::test(flavor = "current_thread")]
async fn transport_failure_is_indeterminate_without_retry() {
    let calls = Arc::new(AtomicUsize::new(0));
    let persisted = Arc::new(AtomicBool::new(false));
    let mut coordinator = coordinator(Arc::clone(&calls), Arc::clone(&persisted), true);
    let payload = b"final-payload";
    let intent = intent(payload, "indeterminate");

    let (outcome, _) = coordinator
        .invoke_once(
            &intent,
            payload,
            revision(),
            PhysicalUseWindow::new(180)
                .unwrap_or_else(|error| panic!("window must build: {error}")),
            &|| Ok(120),
            &|_, _, _, _, _, _| Ok((1, Sha256Digest::for_bytes(b"claim"))),
            |_| {
                persisted.store(true, Ordering::SeqCst);
                Ok(())
            },
        )
        .await
        .unwrap_or_else(|error| panic!("transport uncertainty must be classified: {error}"));

    assert!(matches!(
        outcome,
        ModelInvocationOutcome::Indeterminate { ref reason_code }
            if reason_code == "timeout_after_write"
    ));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test(flavor = "current_thread")]
async fn claim_rejection_blocks_the_physical_adapter() {
    let calls = Arc::new(AtomicUsize::new(0));
    let persisted = Arc::new(AtomicBool::new(false));
    let mut coordinator = coordinator(Arc::clone(&calls), Arc::clone(&persisted), false);
    let payload = b"final-payload";
    let intent = intent(payload, "claim-rejected");

    let result = coordinator
        .invoke_once(
            &intent,
            payload,
            revision(),
            PhysicalUseWindow::new(180)
                .unwrap_or_else(|error| panic!("window must build: {error}")),
            &|| Ok(120),
            &|_, _, _, _, _, _| Err("already_claimed".to_string()),
            |_| Ok(()),
        )
        .await;

    assert!(matches!(result, Err(ModelInvocationError::VerifiedUse(_))));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert!(!persisted.load(Ordering::SeqCst));
}
