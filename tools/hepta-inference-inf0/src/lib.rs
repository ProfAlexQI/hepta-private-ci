//! No-network reference contracts for Hepta inference INF-0.

#![forbid(unsafe_code)]
#![deny(clippy::expect_used, clippy::unwrap_used)]

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fmt;
use std::str::FromStr;

const DIGEST_PREFIX: &str = "sha256:";
const DIGEST_HEX_LEN: usize = 64;
const MAX_ID_LEN: usize = 128;
const MAX_FIELD_LEN: usize = 256;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digest([u8; 32]);

impl Digest {
    pub fn to_prefixed_hex(self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut output = String::with_capacity(DIGEST_PREFIX.len() + DIGEST_HEX_LEN);
        output.push_str(DIGEST_PREFIX);
        for byte in self.0 {
            output.push(char::from(HEX[usize::from(byte >> 4)]));
            output.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
        output
    }
}

impl fmt::Debug for Digest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&(*self).to_prefixed_hex())
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&(*self).to_prefixed_hex())
    }
}

impl FromStr for Digest {
    type Err = DigestError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let hex = value
            .strip_prefix(DIGEST_PREFIX)
            .ok_or(DigestError::UnsupportedAlgorithm)?;
        if hex.len() != DIGEST_HEX_LEN {
            return Err(DigestError::InvalidLength);
        }
        let mut output = [0_u8; 32];
        for (index, pair) in hex.as_bytes().chunks_exact(2).enumerate() {
            let high = lower_hex(pair[0]).ok_or(DigestError::InvalidLowerHex)?;
            let low = lower_hex(pair[1]).ok_or(DigestError::InvalidLowerHex)?;
            output[index] = (high << 4) | low;
        }
        Ok(Self(output))
    }
}

fn lower_hex(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigestError {
    UnsupportedAlgorithm,
    InvalidLength,
    InvalidLowerHex,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoundedId(String);

impl BoundedId {
    pub fn new(value: impl Into<String>) -> Result<Self, IdentifierError> {
        let value = value.into();
        if value.is_empty() {
            return Err(IdentifierError::Empty);
        }
        if value.len() > MAX_ID_LEN {
            return Err(IdentifierError::TooLong);
        }
        if value.trim() != value {
            return Err(IdentifierError::SurroundingWhitespace);
        }
        if !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':' | b'/'))
        {
            return Err(IdentifierError::InvalidCharacter);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentifierError {
    Empty,
    TooLong,
    SurroundingWhitespace,
    InvalidCharacter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InferenceAuthority {
    pub qualification_only: bool,
    pub production_listener: bool,
    pub production_writer: bool,
    pub provider_effect: bool,
    pub external_effect: bool,
    pub shared_kg_write: bool,
    pub memory_write: bool,
    pub route_write: bool,
    pub fleet_write: bool,
    pub model_npu: bool,
    pub remote_inference: bool,
    pub automatic_model_install: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub release: bool,
}

impl Default for InferenceAuthority {
    fn default() -> Self {
        Self {
            qualification_only: true,
            production_listener: false,
            production_writer: false,
            provider_effect: false,
            external_effect: false,
            shared_kg_write: false,
            memory_write: false,
            route_write: false,
            fleet_write: false,
            model_npu: false,
            remote_inference: false,
            automatic_model_install: false,
            operator_acceptance: false,
            promotion: false,
            release: false,
        }
    }
}

impl InferenceAuthority {
    fn validate_closed(self) -> Result<(), AdmissionError> {
        let elevated = self.production_listener
            || self.production_writer
            || self.provider_effect
            || self.external_effect
            || self.shared_kg_write
            || self.memory_write
            || self.route_write
            || self.fleet_write
            || self.model_npu
            || self.remote_inference
            || self.automatic_model_install
            || self.operator_acceptance
            || self.promotion
            || self.release;
        if !self.qualification_only || elevated {
            return Err(AdmissionError::AuthorityEscalation);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelTuple {
    pub tuple_digest: Digest,
    pub model_digest: Digest,
    pub tokenizer_digest: Digest,
    pub backend_id: String,
    pub backend_commit: String,
    pub backend_abi: String,
    pub compiled_artifact_digest: Digest,
    pub compiler_id: String,
    pub driver_runtime: String,
    pub device_profile_digest: Digest,
    pub quantization: String,
    pub context_tokens: u32,
    pub batch_size: u32,
    pub kv_cache_policy: String,
    pub prefix_cache_policy: String,
    pub input_output_shape: String,
    pub sbom_digest: Digest,
    pub license_digest: Digest,
}

impl ModelTuple {
    fn validate(&self) -> Result<(), RegistryError> {
        for value in [
            self.backend_id.as_str(),
            self.backend_commit.as_str(),
            self.backend_abi.as_str(),
            self.compiler_id.as_str(),
            self.driver_runtime.as_str(),
            self.quantization.as_str(),
            self.kv_cache_policy.as_str(),
            self.prefix_cache_policy.as_str(),
            self.input_output_shape.as_str(),
        ] {
            if value.is_empty() {
                return Err(RegistryError::EmptyField);
            }
            if value.len() > MAX_FIELD_LEN {
                return Err(RegistryError::FieldTooLong);
            }
            if value.trim() != value || value.chars().any(char::is_control) {
                return Err(RegistryError::NonCanonicalField);
            }
        }
        if self.context_tokens == 0 || self.batch_size == 0 {
            return Err(RegistryError::ZeroCapacity);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedProfile {
    pub model_tuple: ModelTuple,
    pub policy_digest: Digest,
    pub resource_budget_id: BoundedId,
    pub max_output_tokens: u32,
    pub loopback_result_digest: Digest,
}

impl AdmittedProfile {
    fn validate(&self) -> Result<(), RegistryError> {
        self.model_tuple.validate()?;
        if self.max_output_tokens == 0 {
            return Err(RegistryError::ZeroCapacity);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryError {
    DuplicateTuple,
    EmptyField,
    FieldTooLong,
    NonCanonicalField,
    ZeroCapacity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestScope {
    pub tenant_id: BoundedId,
    pub workspace_id: BoundedId,
    pub agent_id: BoundedId,
    pub agent_generation: u64,
    pub task_id: BoundedId,
    pub request_id: BoundedId,
    pub request_generation: u64,
    pub cancel_generation: u64,
    pub policy_digest: Digest,
    pub resource_budget_id: BoundedId,
    pub deadline_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InferenceRequest {
    pub scope: RequestScope,
    pub model_tuple_digest: Digest,
    pub prompt_digest: Digest,
    pub prompt_byte_length: u64,
    pub output_token_limit: u32,
    pub privacy_class: BoundedId,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CacheScope {
    pub tenant_id: BoundedId,
    pub workspace_id: BoundedId,
    pub model_tuple_digest: Digest,
    pub policy_digest: Digest,
    pub privacy_class: BoundedId,
    pub backend_generation: u64,
}

impl CacheScope {
    pub fn may_reuse_with(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionTicket {
    pub request_id: BoundedId,
    pub request_generation: u64,
    pub backend_generation: u64,
    pub model_tuple_digest: Digest,
    pub cache_scope: CacheScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdmissionError {
    AuthorityEscalation,
    InvalidGeneration,
    DeadlineExpired,
    EmptyPrompt,
    EmptyOutputLimit,
    UnknownModelTuple,
    PolicyMismatch,
    ResourceBudgetMismatch,
    OutputLimitExceeded,
    BackendUnavailable,
}

#[derive(Debug, Default, Clone)]
pub struct AdmissionRegistry {
    profiles: BTreeMap<Digest, AdmittedProfile>,
}

impl AdmissionRegistry {
    pub fn register(&mut self, profile: AdmittedProfile) -> Result<(), RegistryError> {
        profile.validate()?;
        match self.profiles.entry(profile.model_tuple.tuple_digest) {
            Entry::Vacant(entry) => {
                entry.insert(profile);
                Ok(())
            }
            Entry::Occupied(_) => Err(RegistryError::DuplicateTuple),
        }
    }

    pub fn admit(
        &self,
        request: &InferenceRequest,
        authority: InferenceAuthority,
        now_unix_ms: u64,
        backend_generation: u64,
    ) -> Result<AdmissionTicket, AdmissionError> {
        authority.validate_closed()?;
        if request.scope.agent_generation == 0 || request.scope.request_generation == 0 {
            return Err(AdmissionError::InvalidGeneration);
        }
        if request.scope.deadline_unix_ms <= now_unix_ms {
            return Err(AdmissionError::DeadlineExpired);
        }
        if request.prompt_byte_length == 0 {
            return Err(AdmissionError::EmptyPrompt);
        }
        if request.output_token_limit == 0 {
            return Err(AdmissionError::EmptyOutputLimit);
        }
        if backend_generation == 0 {
            return Err(AdmissionError::BackendUnavailable);
        }
        let profile = self
            .profiles
            .get(&request.model_tuple_digest)
            .ok_or(AdmissionError::UnknownModelTuple)?;
        if request.scope.policy_digest != profile.policy_digest {
            return Err(AdmissionError::PolicyMismatch);
        }
        if request.scope.resource_budget_id != profile.resource_budget_id {
            return Err(AdmissionError::ResourceBudgetMismatch);
        }
        if request.output_token_limit > profile.max_output_tokens {
            return Err(AdmissionError::OutputLimitExceeded);
        }
        Ok(AdmissionTicket {
            request_id: request.scope.request_id.clone(),
            request_generation: request.scope.request_generation,
            backend_generation,
            model_tuple_digest: request.model_tuple_digest,
            cache_scope: CacheScope {
                tenant_id: request.scope.tenant_id.clone(),
                workspace_id: request.scope.workspace_id.clone(),
                model_tuple_digest: request.model_tuple_digest,
                policy_digest: request.scope.policy_digest,
                privacy_class: request.privacy_class.clone(),
                backend_generation,
            },
        })
    }

    fn profile(&self, digest: Digest) -> Option<&AdmittedProfile> {
        self.profiles.get(&digest)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecyclePhase {
    Admitted,
    Reserved,
    Queued,
    Loading,
    Warming,
    Running,
    Draining,
    Completed,
    Cancelled,
    Rejected,
    FailedClosed,
}

impl LifecyclePhase {
    fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Cancelled | Self::Rejected | Self::FailedClosed
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InferenceLifecycle {
    phase: LifecyclePhase,
}

impl InferenceLifecycle {
    pub fn new() -> Self {
        Self {
            phase: LifecyclePhase::Admitted,
        }
    }

    pub fn phase(self) -> LifecyclePhase {
        self.phase
    }

    pub fn transition(&mut self, next: LifecyclePhase) -> Result<(), LifecycleError> {
        if self.phase.is_terminal() {
            return Err(LifecycleError::TerminalState);
        }
        let normal = matches!(
            (self.phase, next),
            (LifecyclePhase::Admitted, LifecyclePhase::Reserved)
                | (LifecyclePhase::Admitted, LifecyclePhase::Rejected)
                | (LifecyclePhase::Reserved, LifecyclePhase::Queued)
                | (LifecyclePhase::Queued, LifecyclePhase::Loading)
                | (LifecyclePhase::Queued, LifecyclePhase::Running)
                | (LifecyclePhase::Loading, LifecyclePhase::Warming)
                | (LifecyclePhase::Loading, LifecyclePhase::Running)
                | (LifecyclePhase::Warming, LifecyclePhase::Running)
                | (LifecyclePhase::Running, LifecyclePhase::Draining)
                | (LifecyclePhase::Draining, LifecyclePhase::Completed)
        );
        let failure = matches!(next, LifecyclePhase::Cancelled | LifecyclePhase::FailedClosed);
        if !normal && !failure {
            return Err(LifecycleError::InvalidTransition {
                from: self.phase,
                to: next,
            });
        }
        self.phase = next;
        Ok(())
    }
}

impl Default for InferenceLifecycle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleError {
    TerminalState,
    InvalidTransition {
        from: LifecyclePhase,
        to: LifecyclePhase,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CancellationFence {
    request_generation: u64,
    backend_generation: u64,
    highest_cancel_generation: u64,
    last_sequence: u64,
    cancelled: bool,
}

impl CancellationFence {
    pub fn new(request_generation: u64, backend_generation: u64) -> Result<Self, FenceError> {
        if request_generation == 0 || backend_generation == 0 {
            return Err(FenceError::InvalidGeneration);
        }
        Ok(Self {
            request_generation,
            backend_generation,
            highest_cancel_generation: 0,
            last_sequence: 0,
            cancelled: false,
        })
    }

    pub fn publish(
        &mut self,
        request_generation: u64,
        backend_generation: u64,
        sequence: u64,
    ) -> Result<(), FenceError> {
        self.validate(request_generation, backend_generation)?;
        if self.cancelled {
            return Err(FenceError::Cancelled);
        }
        if sequence == 0 || sequence <= self.last_sequence {
            return Err(FenceError::NonMonotonicSequence);
        }
        self.last_sequence = sequence;
        Ok(())
    }

    pub fn cancel(
        &mut self,
        request_generation: u64,
        backend_generation: u64,
        cancel_generation: u64,
    ) -> Result<CancelReceipt, FenceError> {
        self.validate(request_generation, backend_generation)?;
        if cancel_generation == 0 || cancel_generation <= self.highest_cancel_generation {
            return Err(FenceError::StaleCancelGeneration);
        }
        self.highest_cancel_generation = cancel_generation;
        self.cancelled = true;
        Ok(CancelReceipt {
            request_generation,
            backend_generation,
            cancel_generation,
            last_published_sequence: self.last_sequence,
        })
    }

    fn validate(
        &self,
        request_generation: u64,
        backend_generation: u64,
    ) -> Result<(), FenceError> {
        if request_generation != self.request_generation {
            return Err(FenceError::StaleRequestGeneration);
        }
        if backend_generation != self.backend_generation {
            return Err(FenceError::StaleBackendGeneration);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CancelReceipt {
    pub request_generation: u64,
    pub backend_generation: u64,
    pub cancel_generation: u64,
    pub last_published_sequence: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceError {
    InvalidGeneration,
    StaleRequestGeneration,
    StaleBackendGeneration,
    StaleCancelGeneration,
    NonMonotonicSequence,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InferenceEventKind {
    Accepted,
    Reserved,
    Queued,
    Running,
    TokenDelta {
        token_index: u32,
        token_digest: Digest,
    },
    Draining,
    Completed {
        result_digest: Digest,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InferenceEvent {
    pub request_id: BoundedId,
    pub request_generation: u64,
    pub backend_generation: u64,
    pub sequence: u64,
    pub kind: InferenceEventKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionReceipt {
    pub schema: &'static str,
    pub request_id: BoundedId,
    pub request_generation: u64,
    pub backend_generation: u64,
    pub model_tuple_digest: Digest,
    pub policy_digest: Digest,
    pub resource_budget_id: BoundedId,
    pub prompt_digest: Digest,
    pub prompt_byte_length: u64,
    pub output_tokens: u32,
    pub result_digest: Digest,
    pub final_phase: LifecyclePhase,
    pub authority: InferenceAuthority,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopbackOutcome {
    pub events: Vec<InferenceEvent>,
    pub receipt: ExecutionReceipt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopbackError {
    Admission(AdmissionError),
    Lifecycle(LifecycleError),
    Fence(FenceError),
    MissingProfile,
}

impl From<AdmissionError> for LoopbackError {
    fn from(value: AdmissionError) -> Self {
        Self::Admission(value)
    }
}

impl From<LifecycleError> for LoopbackError {
    fn from(value: LifecycleError) -> Self {
        Self::Lifecycle(value)
    }
}

impl From<FenceError> for LoopbackError {
    fn from(value: FenceError) -> Self {
        Self::Fence(value)
    }
}

#[derive(Debug, Clone)]
pub struct ReferenceLoopbackBackend {
    registry: AdmissionRegistry,
    backend_generation: u64,
}

impl ReferenceLoopbackBackend {
    pub fn new(
        registry: AdmissionRegistry,
        backend_generation: u64,
    ) -> Result<Self, AdmissionError> {
        if backend_generation == 0 {
            return Err(AdmissionError::BackendUnavailable);
        }
        Ok(Self {
            registry,
            backend_generation,
        })
    }

    pub fn execute(
        &self,
        request: &InferenceRequest,
        authority: InferenceAuthority,
        now_unix_ms: u64,
    ) -> Result<LoopbackOutcome, LoopbackError> {
        let ticket = self
            .registry
            .admit(request, authority, now_unix_ms, self.backend_generation)?;
        let result_digest = self
            .registry
            .profile(ticket.model_tuple_digest)
            .ok_or(LoopbackError::MissingProfile)?
            .loopback_result_digest;
        let mut lifecycle = InferenceLifecycle::new();
        let mut fence = CancellationFence::new(
            ticket.request_generation,
            ticket.backend_generation,
        )?;
        let mut events = Vec::with_capacity(7);
        append_event(&mut events, &mut fence, &ticket, 1, InferenceEventKind::Accepted)?;
        lifecycle.transition(LifecyclePhase::Reserved)?;
        append_event(&mut events, &mut fence, &ticket, 2, InferenceEventKind::Reserved)?;
        lifecycle.transition(LifecyclePhase::Queued)?;
        append_event(&mut events, &mut fence, &ticket, 3, InferenceEventKind::Queued)?;
        lifecycle.transition(LifecyclePhase::Running)?;
        append_event(&mut events, &mut fence, &ticket, 4, InferenceEventKind::Running)?;
        append_event(
            &mut events,
            &mut fence,
            &ticket,
            5,
            InferenceEventKind::TokenDelta {
                token_index: 0,
                token_digest: result_digest,
            },
        )?;
        lifecycle.transition(LifecyclePhase::Draining)?;
        append_event(&mut events, &mut fence, &ticket, 6, InferenceEventKind::Draining)?;
        lifecycle.transition(LifecyclePhase::Completed)?;
        append_event(
            &mut events,
            &mut fence,
            &ticket,
            7,
            InferenceEventKind::Completed { result_digest },
        )?;
        Ok(LoopbackOutcome {
            events,
            receipt: ExecutionReceipt {
                schema: "hepta.inference.execution_receipt.v1",
                request_id: ticket.request_id,
                request_generation: ticket.request_generation,
                backend_generation: ticket.backend_generation,
                model_tuple_digest: ticket.model_tuple_digest,
                policy_digest: request.scope.policy_digest,
                resource_budget_id: request.scope.resource_budget_id.clone(),
                prompt_digest: request.prompt_digest,
                prompt_byte_length: request.prompt_byte_length,
                output_tokens: 1,
                result_digest,
                final_phase: lifecycle.phase(),
                authority,
            },
        })
    }
}

fn append_event(
    events: &mut Vec<InferenceEvent>,
    fence: &mut CancellationFence,
    ticket: &AdmissionTicket,
    sequence: u64,
    kind: InferenceEventKind,
) -> Result<(), FenceError> {
    fence.publish(
        ticket.request_generation,
        ticket.backend_generation,
        sequence,
    )?;
    events.push(InferenceEvent {
        request_id: ticket.request_id.clone(),
        request_generation: ticket.request_generation,
        backend_generation: ticket.backend_generation,
        sequence,
        kind,
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const D0: &str = "sha256:0000000000000000000000000000000000000000000000000000000000000000";
    const D1: &str = "sha256:1111111111111111111111111111111111111111111111111111111111111111";
    const D2: &str = "sha256:2222222222222222222222222222222222222222222222222222222222222222";
    const D3: &str = "sha256:3333333333333333333333333333333333333333333333333333333333333333";
    const D4: &str = "sha256:4444444444444444444444444444444444444444444444444444444444444444";
    const D5: &str = "sha256:5555555555555555555555555555555555555555555555555555555555555555";
    const D6: &str = "sha256:6666666666666666666666666666666666666666666666666666666666666666";
    const D7: &str = "sha256:7777777777777777777777777777777777777777777777777777777777777777";
    const D8: &str = "sha256:8888888888888888888888888888888888888888888888888888888888888888";
    const D9: &str = "sha256:9999999999999999999999999999999999999999999999999999999999999999";

    fn digest(value: &str) -> Digest {
        match value.parse() {
            Ok(value) => value,
            Err(error) => panic!("invalid test digest: {error:?}"),
        }
    }

    fn id(value: &str) -> BoundedId {
        match BoundedId::new(value) {
            Ok(value) => value,
            Err(error) => panic!("invalid test identifier: {error:?}"),
        }
    }

    fn profile() -> AdmittedProfile {
        AdmittedProfile {
            model_tuple: ModelTuple {
                tuple_digest: digest(D0),
                model_digest: digest(D1),
                tokenizer_digest: digest(D2),
                backend_id: "reference-loopback".to_string(),
                backend_commit: "locked-test-backend".to_string(),
                backend_abi: "hepta_backend_v1".to_string(),
                compiled_artifact_digest: digest(D3),
                compiler_id: "rustc-test".to_string(),
                driver_runtime: "none".to_string(),
                device_profile_digest: digest(D4),
                quantization: "none".to_string(),
                context_tokens: 4096,
                batch_size: 1,
                kv_cache_policy: "disabled".to_string(),
                prefix_cache_policy: "disabled".to_string(),
                input_output_shape: "digest-only-loopback".to_string(),
                sbom_digest: digest(D5),
                license_digest: digest(D6),
            },
            policy_digest: digest(D7),
            resource_budget_id: id("budget-a"),
            max_output_tokens: 16,
            loopback_result_digest: digest(D8),
        }
    }

    fn registry() -> AdmissionRegistry {
        let mut registry = AdmissionRegistry::default();
        if let Err(error) = registry.register(profile()) {
            panic!("profile registration failed: {error:?}");
        }
        registry
    }

    fn request() -> InferenceRequest {
        InferenceRequest {
            scope: RequestScope {
                tenant_id: id("tenant-a"),
                workspace_id: id("workspace-a"),
                agent_id: id("agent-a"),
                agent_generation: 1,
                task_id: id("task-a"),
                request_id: id("request-a"),
                request_generation: 1,
                cancel_generation: 0,
                policy_digest: digest(D7),
                resource_budget_id: id("budget-a"),
                deadline_unix_ms: 2_000,
            },
            model_tuple_digest: digest(D0),
            prompt_digest: digest(D9),
            prompt_byte_length: 32,
            output_token_limit: 8,
            privacy_class: id("private-local"),
        }
    }

    #[test]
    fn digest_is_strict() {
        assert_eq!(digest(D0).to_string(), D0);
        assert_eq!("sha256:00".parse::<Digest>(), Err(DigestError::InvalidLength));
        assert_eq!(
            "sha256:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
                .parse::<Digest>(),
            Err(DigestError::InvalidLowerHex)
        );
    }

    #[test]
    fn admission_fails_closed() {
        let authority = InferenceAuthority {
            production_writer: true,
            ..InferenceAuthority::default()
        };
        assert_eq!(
            registry().admit(&request(), authority, 1_000, 1),
            Err(AdmissionError::AuthorityEscalation)
        );
        let mut unknown = request();
        unknown.model_tuple_digest = digest(D1);
        assert_eq!(
            registry().admit(&unknown, InferenceAuthority::default(), 1_000, 1),
            Err(AdmissionError::UnknownModelTuple)
        );
        let mut wrong_policy = request();
        wrong_policy.scope.policy_digest = digest(D8);
        assert_eq!(
            registry().admit(
                &wrong_policy,
                InferenceAuthority::default(),
                1_000,
                1,
            ),
            Err(AdmissionError::PolicyMismatch)
        );
    }

    #[test]
    fn cache_scope_is_tenant_and_workspace_bound() {
        let ticket = match registry().admit(
            &request(),
            InferenceAuthority::default(),
            1_000,
            7,
        ) {
            Ok(ticket) => ticket,
            Err(error) => panic!("admission failed: {error:?}"),
        };
        let same = ticket.cache_scope.clone();
        assert!(ticket.cache_scope.may_reuse_with(&same));
        let mut other_tenant = same.clone();
        other_tenant.tenant_id = id("tenant-b");
        assert!(!ticket.cache_scope.may_reuse_with(&other_tenant));
        let mut other_workspace = same;
        other_workspace.workspace_id = id("workspace-b");
        assert!(!ticket.cache_scope.may_reuse_with(&other_workspace));
    }

    #[test]
    fn generation_and_cancel_fences_reject_stale_events() {
        let mut fence = match CancellationFence::new(4, 9) {
            Ok(fence) => fence,
            Err(error) => panic!("fence creation failed: {error:?}"),
        };
        assert_eq!(fence.publish(3, 9, 1), Err(FenceError::StaleRequestGeneration));
        assert_eq!(fence.publish(4, 8, 1), Err(FenceError::StaleBackendGeneration));
        assert_eq!(fence.publish(4, 9, 1), Ok(()));
        assert_eq!(fence.publish(4, 9, 1), Err(FenceError::NonMonotonicSequence));
        let receipt = match fence.cancel(4, 9, 1) {
            Ok(receipt) => receipt,
            Err(error) => panic!("cancel failed: {error:?}"),
        };
        assert_eq!(receipt.last_published_sequence, 1);
        assert_eq!(fence.publish(4, 9, 2), Err(FenceError::Cancelled));
        assert_eq!(fence.cancel(4, 9, 1), Err(FenceError::StaleCancelGeneration));
    }

    #[test]
    fn lifecycle_is_terminal_and_loopback_is_deterministic() {
        let mut lifecycle = InferenceLifecycle::new();
        assert!(matches!(
            lifecycle.transition(LifecyclePhase::Running),
            Err(LifecycleError::InvalidTransition { .. })
        ));
        assert_eq!(lifecycle.transition(LifecyclePhase::Reserved), Ok(()));
        assert_eq!(lifecycle.transition(LifecyclePhase::Queued), Ok(()));
        assert_eq!(lifecycle.transition(LifecyclePhase::Running), Ok(()));
        assert_eq!(lifecycle.transition(LifecyclePhase::Draining), Ok(()));
        assert_eq!(lifecycle.transition(LifecyclePhase::Completed), Ok(()));
        assert_eq!(
            lifecycle.transition(LifecyclePhase::FailedClosed),
            Err(LifecycleError::TerminalState)
        );

        let backend = match ReferenceLoopbackBackend::new(registry(), 11) {
            Ok(backend) => backend,
            Err(error) => panic!("backend creation failed: {error:?}"),
        };
        let first = match backend.execute(&request(), InferenceAuthority::default(), 1_000) {
            Ok(outcome) => outcome,
            Err(error) => panic!("loopback failed: {error:?}"),
        };
        let second = match backend.execute(&request(), InferenceAuthority::default(), 1_000) {
            Ok(outcome) => outcome,
            Err(error) => panic!("loopback failed: {error:?}"),
        };
        assert_eq!(first, second);
        assert_eq!(first.events.len(), 7);
        assert_eq!(first.receipt.final_phase, LifecyclePhase::Completed);
        assert_eq!(first.receipt.result_digest, digest(D8));
        assert_eq!(first.receipt.prompt_digest, digest(D9));
        assert_eq!(first.receipt.authority, InferenceAuthority::default());
    }
}
