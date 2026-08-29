use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::AuthorityAction;
use codex_hepta_contracts::AuthorityCapability;
use codex_hepta_contracts::AuthorityLeaseBinding;
use codex_hepta_contracts::Authorized;
use codex_hepta_contracts::CapabilityVerificationRequest;
use codex_hepta_contracts::CapabilityVerifier;
use codex_hepta_contracts::ExternalEffectCapability;
use codex_hepta_contracts::PhysicalCapabilityKind;
use codex_hepta_contracts::PhysicalUseVerification;
use codex_hepta_contracts::PhysicalUseVerificationRequest;
use codex_hepta_contracts::PhysicalUseVerifier;
use codex_hepta_contracts::PhysicalUseWindow;
use codex_hepta_contracts::ProviderDispatchCapability;
use codex_hepta_contracts::ProviderEffectAck;
use codex_hepta_contracts::ProviderEffectAckStatus;
use codex_hepta_contracts::ProviderEffectAdapter;
use codex_hepta_contracts::ProviderEffectDispatch;
use codex_hepta_contracts::ProviderEffectFuture;
use codex_hepta_contracts::ProviderEffectIdempotencyCapability;
use codex_hepta_contracts::ProviderEffectIntent;
use codex_hepta_contracts::ProviderEffectKey;
use codex_hepta_contracts::ProviderEffectLookup;
use codex_hepta_contracts::ProviderOperationCoordinator;
use codex_hepta_contracts::ProviderOperationRecord;
use codex_hepta_contracts::RevocationRevision;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::authorize_verified_capability;

const AGENT: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const REVISION: u64 = 19;

fn agent() -> AgentId {
    AgentId::parse(AGENT).expect("test agent must parse")
}

fn intent(payload: &[u8]) -> ProviderEffectIntent {
    ProviderEffectIntent::new(
        ProviderEffectKey::parse(format!(
            "provider-effect:v1:{}",
            Sha256Digest::for_bytes(b"b1-provider-occurrence").as_str()
        ))
        .expect("provider key must parse"),
        Sha256Digest::for_bytes(payload),
    )
}

fn operation(intent: &ProviderEffectIntent) -> ProviderOperationRecord {
    ProviderOperationRecord::new(
        agent(),
        intent,
        7,
        11,
        3,
        Sha256Digest::for_bytes(b"b1-provider-fence"),
        64,
    )
    .expect("provider operation must build")
}

#[derive(Clone, Copy)]
struct ExactVerifier;

impl CapabilityVerifier for ExactVerifier {
    fn verify(&self, request: &CapabilityVerificationRequest<'_>) -> Result<(), String> {
        if !matches!(
            request.action(),
            AuthorityAction::DispatchProvider | AuthorityAction::ExternalEffect
        ) {
            return Err("unexpected capability action".to_string());
        }
        Ok(())
    }
}

impl PhysicalUseVerifier for ExactVerifier {
    fn verify_physical_use(
        &self,
        request: &PhysicalUseVerificationRequest<'_>,
    ) -> Result<PhysicalUseVerification, String> {
        if !matches!(
            request.kind(),
            PhysicalCapabilityKind::ProviderDispatch | PhysicalCapabilityKind::ExternalEffect
        ) {
            return Err("unexpected physical capability kind".to_string());
        }
        PhysicalUseVerification::new(
            RevocationRevision::new(REVISION).map_err(|error| error.to_string())?,
            180,
            Sha256Digest::for_bytes(request.kind().as_str().as_bytes()),
        )
        .map_err(|error| error.to_string())
    }
}

fn capability<C>(grant: &[u8]) -> Authorized<C>
where
    C: AuthorityCapability,
{
    let binding = AuthorityLeaseBinding::new(
        agent(),
        Sha256Digest::for_bytes(grant),
        7,
        11,
        3,
        Sha256Digest::for_bytes(b"b1-provider-fence"),
        200,
    )
    .expect("binding must build");
    authorize_verified_capability::<C, _>(binding, &agent(), 3, 100, &ExactVerifier)
        .expect("capability must verify")
}

#[derive(Clone)]
struct CompleteAdapter {
    attempted: Arc<AtomicBool>,
}

impl ProviderEffectAdapter for CompleteAdapter {
    fn capability(&self) -> ProviderEffectIdempotencyCapability {
        ProviderEffectIdempotencyCapability::KeyAndStatusLookup
    }

    fn dispatch<'a>(
        &'a self,
        _intent: &'a ProviderEffectIntent,
    ) -> ProviderEffectFuture<'a, ProviderEffectDispatch> {
        Box::pin(async {
            ProviderEffectDispatch::NotDispatched {
                reason_code: "verified_wire_payload_required".to_string(),
            }
        })
    }

    fn dispatch_with_payload<'a>(
        &'a self,
        intent: &'a ProviderEffectIntent,
        _wire_payload: &'a [u8],
    ) -> ProviderEffectFuture<'a, ProviderEffectDispatch> {
        self.attempted.store(true, Ordering::SeqCst);
        Box::pin(async move {
            ProviderEffectDispatch::Ack(ProviderEffectAck::new(
                intent.key.clone(),
                intent.payload_sha256.clone(),
                Sha256Digest::for_bytes(b"b1-provider-operation"),
                ProviderEffectAckStatus::Completed,
            ))
        })
    }

    fn lookup<'a>(
        &'a self,
        _key: &'a ProviderEffectKey,
    ) -> ProviderEffectFuture<'a, ProviderEffectLookup> {
        Box::pin(async { ProviderEffectLookup::Unknown })
    }
}

#[tokio::test]
async fn dual_witnesses_are_persisted_before_provider_dispatch() {
    let payload = b"exact-final-provider-payload";
    let intent = intent(payload);
    let attempted = Arc::new(AtomicBool::new(false));
    let mut coordinator = ProviderOperationCoordinator::new(
        CompleteAdapter {
            attempted: Arc::clone(&attempted),
        },
        operation(&intent),
        capability::<ProviderDispatchCapability>(b"provider-dispatch-grant"),
        capability::<ExternalEffectCapability>(b"external-effect-grant"),
        100,
        ExactVerifier,
    )
    .expect("checked coordinator must build");

    let mut persisted = Vec::new();
    let (receipt, provider_witness, effect_witness) = coordinator
        .dispatch_once_with_payload(
            intent,
            payload,
            RevocationRevision::new(REVISION).expect("revision"),
            RevocationRevision::new(REVISION).expect("revision"),
            PhysicalUseWindow::new(100, 150).expect("window"),
            101,
            |witness| {
                persisted.push(witness.kind());
                Ok(())
            },
        )
        .await
        .expect("verified dispatch must settle");

    assert!(attempted.load(Ordering::SeqCst));
    assert!(receipt.provider.physical_dispatch_attempted);
    assert_eq!(provider_witness.kind(), PhysicalCapabilityKind::ProviderDispatch);
    assert_eq!(effect_witness.kind(), PhysicalCapabilityKind::ExternalEffect);
    assert_eq!(
        persisted,
        vec![
            PhysicalCapabilityKind::ProviderDispatch,
            PhysicalCapabilityKind::ExternalEffect,
        ]
    );
}

#[tokio::test]
async fn witness_sink_failure_prevents_the_physical_send() {
    let payload = b"exact-final-provider-payload";
    let intent = intent(payload);
    let attempted = Arc::new(AtomicBool::new(false));
    let mut coordinator = ProviderOperationCoordinator::new(
        CompleteAdapter {
            attempted: Arc::clone(&attempted),
        },
        operation(&intent),
        capability::<ProviderDispatchCapability>(b"provider-dispatch-grant"),
        capability::<ExternalEffectCapability>(b"external-effect-grant"),
        100,
        ExactVerifier,
    )
    .expect("checked coordinator must build");

    let result = coordinator
        .dispatch_once_with_payload(
            intent,
            payload,
            RevocationRevision::new(REVISION).expect("revision"),
            RevocationRevision::new(REVISION).expect("revision"),
            PhysicalUseWindow::new(100, 150).expect("window"),
            101,
            |_witness| Err("durable witness store unavailable".to_string()),
        )
        .await;

    assert!(result.is_err());
    assert!(!attempted.load(Ordering::SeqCst));
}
