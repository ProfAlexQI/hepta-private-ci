use std::collections::BTreeMap;
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
use codex_hepta_contracts::OperationPhase;
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
use codex_hepta_contracts::ProviderOperationError;
use codex_hepta_contracts::ProviderOperationRecord;
use codex_hepta_contracts::RevocationRevision;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::VerifiedUseWitness;
use codex_hepta_contracts::authorize_verified_capability;

const AGENT: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const GENERATION: u64 = 3;
const AUTHORITY_EPOCH: u64 = 7;
const OWNER_EPOCH: u64 = 11;
const REVISION: u64 = 19;
const EXPIRES_AT: u64 = 200;

fn agent() -> AgentId {
    AgentId::parse(AGENT).expect("test agent must parse")
}

fn intent(payload: &[u8]) -> ProviderEffectIntent {
    ProviderEffectIntent::new(
        ProviderEffectKey::parse(format!(
            "provider-effect:v1:{}",
            Sha256Digest::for_bytes(b"b1a-provider-occurrence").as_str()
        ))
        .expect("provider key must parse"),
        Sha256Digest::for_bytes(payload),
    )
}

fn operation(intent: &ProviderEffectIntent) -> ProviderOperationRecord {
    ProviderOperationRecord::new(
        agent(),
        intent,
        AUTHORITY_EPOCH,
        OWNER_EPOCH,
        GENERATION,
        Sha256Digest::for_bytes(b"b1a-provider-fence"),
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

impl CapabilityUseVerifier for ExactVerifier {
    fn verify_use(&self, request: &CapabilityUseVerificationRequest<'_>) -> Result<(), String> {
        if !matches!(
            request.action(),
            AuthorityAction::DispatchProvider | AuthorityAction::ExternalEffect
        ) || request.subject_agent_id() != &agent()
            || request.generation() != GENERATION
            || request.observed_at_unix_seconds() == 0
        {
            return Err("current capability-use binding drift".to_string());
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
        ) || request.action() != request.kind().authority_action()
            || request.runtime_context().generation() != GENERATION
            || request.expected_revocation_revision().get() != REVISION
            || request.observed_at_unix_seconds() == 0
            || request.requested_expires_at_unix_seconds()
                <= request.observed_at_unix_seconds()
        {
            return Err("physical-use request binding drift".to_string());
        }
        PhysicalUseVerification::new(
            RevocationRevision::new(REVISION).map_err(|error| error.to_string())?,
            190,
            Sha256Digest::for_bytes(request.kind().as_str().as_bytes()),
        )
        .map_err(|error| error.to_string())
    }
}

struct TestClock {
    now: AtomicU64,
}

impl TestClock {
    fn at(now: u64) -> Self {
        Self {
            now: AtomicU64::new(now),
        }
    }
}

impl TestClock {
    fn read(&self) -> Result<u64, String> {
        Ok(self.now.load(Ordering::SeqCst))
    }
}

#[derive(Default)]
struct InMemoryClaimStore {
    claims: Mutex<BTreeMap<String, String>>,
    next_revision: AtomicU64,
    calls: AtomicUsize,
    reject_kind: Mutex<Option<PhysicalCapabilityKind>>,
}

impl InMemoryClaimStore {
    fn reject(&self, kind: PhysicalCapabilityKind) {
        *self
            .reject_kind
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(kind);
    }

    fn count(&self) -> usize {
        self.claims
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .len()
    }
}

