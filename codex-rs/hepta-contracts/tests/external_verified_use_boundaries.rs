use std::sync::Arc;
use std::sync::Mutex;
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
use codex_hepta_contracts::ExternalEffectCapability;
use codex_hepta_contracts::OperationId;
use codex_hepta_contracts::PhysicalCapabilityKind;
use codex_hepta_contracts::PhysicalUseVerification;
use codex_hepta_contracts::PhysicalUseVerificationRequest;
use codex_hepta_contracts::PhysicalUseVerifier;
use codex_hepta_contracts::PhysicalUseWindow;
use codex_hepta_contracts::RevocationRevision;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::authorize_verified_capability;
use codex_hepta_contracts::physical_boundaries::external::CheckedExternalBoundary;
use codex_hepta_contracts::physical_boundaries::external::ExternalBoundaryAdapter;
use codex_hepta_contracts::physical_boundaries::external::ExternalBoundaryDispatch;
use codex_hepta_contracts::physical_boundaries::external::ExternalBoundaryError;
use codex_hepta_contracts::physical_boundaries::external::ExternalBoundaryFuture;
use codex_hepta_contracts::physical_boundaries::external::ExternalBoundaryIntent;
use codex_hepta_contracts::physical_boundaries::external::ExternalBoundaryOutcome;
use codex_hepta_contracts::physical_boundaries::external::ExternalFilesystemMutationIntent;
use codex_hepta_contracts::physical_boundaries::external::FilesystemMutationClass;
use codex_hepta_contracts::physical_boundaries::external::NetworkProtocol;
use codex_hepta_contracts::physical_boundaries::external::OutboundNetworkIntent;
use codex_hepta_contracts::physical_boundaries::external::ToolProcessIntent;

const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const GENERATION: u64 = 5;
const REVISION: u64 = 17;

fn agent_id() -> AgentId {
    AgentId::parse(AGENT_ID).unwrap_or_else(|error| panic!("AgentId must parse: {error}"))
}

fn revision() -> RevocationRevision {
    RevocationRevision::new(REVISION)
        .unwrap_or_else(|error| panic!("revision must build: {error}"))
}

fn digest(value: &[u8]) -> Sha256Digest {
    Sha256Digest::for_bytes(value)
}

fn operation(suffix: &str) -> OperationId {
    OperationId::parse(format!("operation:external:{suffix}"))
        .unwrap_or_else(|error| panic!("operation must parse: {error}"))
}

struct MintVerifier;

impl CapabilityVerifier for MintVerifier {
    fn verify(&self, request: &CapabilityVerificationRequest<'_>) -> Result<(), String> {
        if request.action() != AuthorityAction::ExternalEffect {
            return Err("unexpected external-effect mint action".to_string());
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
        if request.action() != AuthorityAction::ExternalEffect {
            return Err("unexpected external-effect use action".to_string());
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
            PhysicalCapabilityKind::ToolProcessSpawn
                | PhysicalCapabilityKind::OutboundNetworkConnect
                | PhysicalCapabilityKind::ExternalFilesystemMutation
        ) || request.action() != AuthorityAction::ExternalEffect
            || request.operation_id().as_str().is_empty()
            || request.final_payload_sha256().as_str().is_empty()
        {
            return Err("external physical binding drift".to_string());
        }
        let call = self.physical_calls.fetch_add(1, Ordering::SeqCst) + 1;
        let current = RevocationRevision::new(self.current_revision.load(Ordering::SeqCst))
            .map_err(|error| error.to_string())?;
        PhysicalUseVerification::new(
            current,
            190,
            digest(format!("external-verifier-{call}").as_bytes()),
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

impl ExternalBoundaryAdapter for RecordingAdapter {
    fn cross<'a>(
        &'a mut self,
        dispatch: ExternalBoundaryDispatch<'a>,
    ) -> ExternalBoundaryFuture<'a> {
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
                Err("ack lost after crossing".to_string())
            } else {
                ExternalBoundaryOutcome::completed(b"bounded-boundary-result")
                    .map_err(|error| error.to_string())
            }
        })
    }
}

fn coordinator(
    calls: Arc<AtomicUsize>,
    witnessed: Arc<AtomicBool>,
    kinds: Arc<Mutex<Vec<PhysicalCapabilityKind>>>,
    fail_transport: bool,
) -> CheckedExternalBoundary<RecordingAdapter, TestVerifier> {
    let binding = AuthorityLeaseBinding::new(
        agent_id(),
        digest(b"signed-external-effect-grant"),
        23,
        29,
        GENERATION,
        digest(b"external-effect-fence"),
        200,
    )
    .unwrap_or_else(|error| panic!("binding must build: {error}"));
    let capability = authorize_verified_capability::<ExternalEffectCapability, _>(
        binding,
        &agent_id(),
        GENERATION,
        100,
        &MintVerifier,
    )
    .unwrap_or_else(|error| panic!("capability must mint: {error}"));
    CheckedExternalBoundary::new(
        RecordingAdapter {
            calls,
            witnessed,
            kinds,
            fail_transport,
        },
        capability,
        TestVerifier::new(),
    )
    .unwrap_or_else(|error| panic!("coordinator must build: {error}"))
}

fn tool(payload: &[u8], suffix: &str, argv: &[u8]) -> ExternalBoundaryIntent {
    ToolProcessIntent::new(
        operation(suffix),
        digest(b"canonical-executable-path"),
        digest(b"executable-file-bytes"),
        digest(argv),
        3,
        digest(b"canonical-cwd"),
        digest(b"cleared-environment-policy"),
        digest(b"sandbox-policy"),
        digest(b"operator-approval-envelope"),
        payload,
    )
    .unwrap_or_else(|error| panic!("tool intent must build: {error}"))
    .into()
}

fn network(payload: &[u8], suffix: &str, ip_set: &[u8]) -> ExternalBoundaryIntent {
    OutboundNetworkIntent::new(
        operation(suffix),
        NetworkProtocol::Https,
        "example.invalid:443",
        digest(ip_set),
        digest(b"dns-pin-policy"),
        digest(b"no-proxy-policy"),
        digest(b"tls-policy"),
        digest(b"redacted-request-headers"),
        payload,
    )
    .unwrap_or_else(|error| panic!("network intent must build: {error}"))
    .into()
}

fn filesystem(
    payload: &[u8],
    suffix: &str,
    prior_state: &[u8],
    no_follow: bool,
) -> Result<ExternalBoundaryIntent, ExternalBoundaryError> {
    ExternalFilesystemMutationIntent::new(
        operation(suffix),
        digest(b"canonical-target"),
        digest(b"device-and-mount-identity"),
        no_follow,
        FilesystemMutationClass::ReplaceFile,
        digest(prior_state),
        payload,
    )
    .map(Into::into)
}

async fn execute_success(
    coordinator: &mut CheckedExternalBoundary<RecordingAdapter, TestVerifier>,
    intent: &ExternalBoundaryIntent,
    payload: &[u8],
    witnessed: &Arc<AtomicBool>,
) -> ExternalBoundaryOutcome {
    witnessed.store(false, Ordering::SeqCst);
    let (outcome, witness) = coordinator
        .cross_once(
            intent,
            payload,
            revision(),
            PhysicalUseWindow::new(180)
                .unwrap_or_else(|error| panic!("window must build: {error}")),
            &|| Ok(120),
            &|kind, scope, claim, token, request, claimed_at| {
                assert_eq!(kind, intent.kind());
                assert!(!scope.as_str().is_empty());
                assert!(!claim.as_str().is_empty());
                assert!(!token.as_str().is_empty());
                assert!(!request.as_str().is_empty());
                assert_eq!(claimed_at, 120);
                Ok((1, digest(format!("claim-{}", kind.as_str()).as_bytes())))
            },
            |value| {
                value.validate().map_err(|error| error.to_string())?;
                witnessed.store(true, Ordering::SeqCst);
                Ok(())
            },
        )
        .await
        .unwrap_or_else(|error| panic!("boundary must succeed: {error}"));
    assert_eq!(witness.kind(), intent.kind());
    assert!(!witness.effect_completed());
    outcome
}

#[tokio::test(flavor = "current_thread")]
async fn all_three_boundaries_claim_and_persist_before_adapter() {
    let calls = Arc::new(AtomicUsize::new(0));
    let witnessed = Arc::new(AtomicBool::new(false));
    let kinds = Arc::new(Mutex::new(Vec::new()));
    let mut coordinator = coordinator(
        Arc::clone(&calls),
        Arc::clone(&witnessed),
        Arc::clone(&kinds),
        false,
    );

    let cases = [
        (tool(b"tool-launch", "tool-success", b"argv-v1"), b"tool-launch".as_slice()),
        (
            network(b"network-request", "network-success", b"203.0.113.7"),
            b"network-request".as_slice(),
        ),
        (
            filesystem(b"replacement-bytes", "fs-success", b"prior-v1", true)
                .unwrap_or_else(|error| panic!("filesystem intent must build: {error}")),
            b"replacement-bytes".as_slice(),
        ),
    ];

    for (intent, payload) in &cases {
        let outcome = execute_success(&mut coordinator, intent, payload, &witnessed).await;
        assert!(matches!(outcome, ExternalBoundaryOutcome::Completed { .. }));
    }

    assert_eq!(calls.load(Ordering::SeqCst), 3);
    assert_eq!(
        *kinds.lock().unwrap_or_else(|error| error.into_inner()),
        vec![
            PhysicalCapabilityKind::ToolProcessSpawn,
            PhysicalCapabilityKind::OutboundNetworkConnect,
            PhysicalCapabilityKind::ExternalFilesystemMutation,
        ]
    );
}

#[test]
fn boundary_specific_facts_change_physical_payload_digests() {
    let tool_a = tool(b"tool", "tool-a", b"argv-a");
    let tool_b = tool(b"tool", "tool-a", b"argv-b");
    assert_ne!(
        tool_a
            .physical_payload_sha256()
            .unwrap_or_else(|error| panic!("tool digest must build: {error}")),
        tool_b
            .physical_payload_sha256()
            .unwrap_or_else(|error| panic!("tool digest must build: {error}"))
    );

    let network_a = network(b"request", "network-a", b"203.0.113.7");
    let network_b = network(b"request", "network-a", b"203.0.113.8");
    assert_ne!(
        network_a
            .physical_payload_sha256()
            .unwrap_or_else(|error| panic!("network digest must build: {error}")),
        network_b
            .physical_payload_sha256()
            .unwrap_or_else(|error| panic!("network digest must build: {error}"))
    );

    let filesystem_a = filesystem(b"replace", "fs-a", b"prior-a", true)
        .unwrap_or_else(|error| panic!("filesystem intent must build: {error}"));
    let filesystem_b = filesystem(b"replace", "fs-a", b"prior-b", true)
        .unwrap_or_else(|error| panic!("filesystem intent must build: {error}"));
    assert_ne!(
        filesystem_a
            .physical_payload_sha256()
            .unwrap_or_else(|error| panic!("filesystem digest must build: {error}")),
        filesystem_b
            .physical_payload_sha256()
            .unwrap_or_else(|error| panic!("filesystem digest must build: {error}"))
    );
}

#[test]
fn external_filesystem_requires_no_follow() {
    assert!(matches!(
        filesystem(b"replace", "fs-follow", b"prior", false),
        Err(ExternalBoundaryError::NoFollowRequired)
    ));
}

#[tokio::test(flavor = "current_thread")]
async fn final_payload_drift_fails_before_claim_and_adapter() {
    let calls = Arc::new(AtomicUsize::new(0));
    let witnessed = Arc::new(AtomicBool::new(false));
    let kinds = Arc::new(Mutex::new(Vec::new()));
    let mut coordinator = coordinator(
        Arc::clone(&calls),
        Arc::clone(&witnessed),
        kinds,
        false,
    );
    let intent = network(b"request-a", "payload-drift", b"203.0.113.7");
    let claim_calls = AtomicUsize::new(0);

    let result = coordinator
        .cross_once(
            &intent,
            b"request-b",
            revision(),
            PhysicalUseWindow::new(180)
                .unwrap_or_else(|error| panic!("window must build: {error}")),
            &|| Ok(120),
            &|_, _, _, _, _, _| {
                claim_calls.fetch_add(1, Ordering::SeqCst);
                Ok((1, digest(b"unreachable")))
            },
            |_| Ok(()),
        )
        .await;

    assert!(matches!(result, Err(ExternalBoundaryError::FinalPayloadDrift)));
    assert_eq!(claim_calls.load(Ordering::SeqCst), 0);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert!(!witnessed.load(Ordering::SeqCst));
}

#[tokio::test(flavor = "current_thread")]
async fn claim_and_witness_failures_block_adapter() {
    let calls = Arc::new(AtomicUsize::new(0));
    let witnessed = Arc::new(AtomicBool::new(false));
    let kinds = Arc::new(Mutex::new(Vec::new()));
    let payload = b"tool-launch";
    let intent = tool(payload, "claim-failure", b"argv");
    let mut claim_failure = coordinator(
        Arc::clone(&calls),
        Arc::clone(&witnessed),
        Arc::clone(&kinds),
        false,
    );

    let result = claim_failure
        .cross_once(
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
    assert!(matches!(result, Err(ExternalBoundaryError::VerifiedUse(_))));
    assert_eq!(calls.load(Ordering::SeqCst), 0);

    let intent = tool(payload, "witness-failure", b"argv");
    let mut witness_failure = coordinator(
        Arc::clone(&calls),
        Arc::clone(&witnessed),
        kinds,
        false,
    );
    let result = witness_failure
        .cross_once(
            &intent,
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
        Err(ExternalBoundaryError::WitnessPersistence(reason))
            if reason == "witness_commit_failed"
    ));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}

#[tokio::test(flavor = "current_thread")]
async fn post_crossing_transport_failure_is_indeterminate_without_retry() {
    let calls = Arc::new(AtomicUsize::new(0));
    let witnessed = Arc::new(AtomicBool::new(false));
    let kinds = Arc::new(Mutex::new(Vec::new()));
    let mut coordinator = coordinator(
        Arc::clone(&calls),
        Arc::clone(&witnessed),
        kinds,
        true,
    );
    let payload = b"network-request";
    let intent = network(payload, "indeterminate", b"203.0.113.7");

    let (outcome, _) = coordinator
        .cross_once(
            &intent,
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
        ExternalBoundaryOutcome::Indeterminate { ref reason_code }
            if reason_code == "ack_lost_after_crossing"
    ));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}
