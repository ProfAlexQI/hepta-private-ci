use std::collections::BTreeMap;
use std::fmt;

use crate::Digest;
use crate::RequestId;

pub const HEPTA_BACKEND_ABI_VERSION: u32 = 1;
pub const HEPTA_BACKEND_ABI_NAME: &str = "hepta_backend_v1";
pub const LLAMA_CPP_PINNED_COMMIT: &str = "cc83d7b4824f73cfdda4dfbb47ee39804f71b328";
pub const MAX_SHARED_REGION_BYTES: u64 = 256 * 1024 * 1024;
pub const MAX_NATIVE_WORKER_ACTIVE: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendOperation {
    Load,
    Warm,
    Submit,
    Poll,
    Cancel,
    Drain,
    Unload,
    Health,
    Stats,
}

pub const REQUIRED_BACKEND_OPERATIONS: [BackendOperation; 9] = [
    BackendOperation::Load,
    BackendOperation::Warm,
    BackendOperation::Submit,
    BackendOperation::Poll,
    BackendOperation::Cancel,
    BackendOperation::Drain,
    BackendOperation::Unload,
    BackendOperation::Health,
    BackendOperation::Stats,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkerTransport {
    InheritedPipe,
    OwnerLocalUds,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendAbiContract {
    pub abi_version: u32,
    pub abi_name: String,
    pub backend_id: String,
    pub upstream_commit: String,
    pub transport: WorkerTransport,
    pub operations: [BackendOperation; 9],
}

impl BackendAbiContract {
    pub fn pinned_llama_cpp() -> Self {
        Self {
            abi_version: HEPTA_BACKEND_ABI_VERSION,
            abi_name: HEPTA_BACKEND_ABI_NAME.to_owned(),
            backend_id: "llama.cpp".to_owned(),
            upstream_commit: LLAMA_CPP_PINNED_COMMIT.to_owned(),
            transport: WorkerTransport::InheritedPipe,
            operations: REQUIRED_BACKEND_OPERATIONS,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.abi_version != HEPTA_BACKEND_ABI_VERSION
            || self.abi_name != HEPTA_BACKEND_ABI_NAME
            || !valid_backend_id(&self.backend_id)
            || !is_lower_hex_commit(&self.upstream_commit)
            || self.operations != REQUIRED_BACKEND_OPERATIONS
        {
            return Err(WorkerError::AbiContractInvalid);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkerQualificationDisposition {
    Qualified,
    KnownGapNotRouted,
    UnsupportedFailClosed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GgufModelManifest {
    pub tuple_digest: Digest,
    pub model_digest: Digest,
    pub tokenizer_digest: Digest,
    pub gguf_artifact_digest: Digest,
    pub sbom_digest: Digest,
    pub license_digest: Digest,
    pub device_profile_digest: Digest,
    pub backend: BackendAbiContract,
    pub quantization: String,
    pub disposition: WorkerQualificationDisposition,
}

impl GgufModelManifest {
    pub fn validate(&self) -> Result<()> {
        self.backend.validate()?;
        if self.backend.backend_id != "llama.cpp"
            || self.backend.upstream_commit != LLAMA_CPP_PINNED_COMMIT
            || !is_allowed_quantization(&self.quantization)
        {
            return Err(WorkerError::ManifestInvalid);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct NativeWorkerRegistry {
    manifests: BTreeMap<Digest, GgufModelManifest>,
}

impl NativeWorkerRegistry {
    pub fn new(manifests: impl IntoIterator<Item = GgufModelManifest>) -> Result<Self> {
        let mut registry = BTreeMap::new();
        for manifest in manifests {
            manifest.validate()?;
            if registry
                .insert(manifest.tuple_digest.clone(), manifest)
                .is_some()
            {
                return Err(WorkerError::DuplicateTuple);
            }
        }
        if registry.is_empty() {
            return Err(WorkerError::RegistryEmpty);
        }
        Ok(Self {
            manifests: registry,
        })
    }

    pub fn manifest(&self, tuple_digest: &Digest) -> Result<&GgufModelManifest> {
        self.manifests
            .get(tuple_digest)
            .ok_or(WorkerError::UnknownTuple)
    }

    pub fn admit(&self, tuple_digest: &Digest) -> Result<&GgufModelManifest> {
        let manifest = self.manifest(tuple_digest)?;
        match manifest.disposition {
            WorkerQualificationDisposition::Qualified => Ok(manifest),
            WorkerQualificationDisposition::KnownGapNotRouted => {
                Err(WorkerError::KnownGapNotRouted)
            }
            WorkerQualificationDisposition::UnsupportedFailClosed => {
                Err(WorkerError::UnsupportedFailClosed)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct AbiByteSlice {
    pub region_handle: u64,
    pub offset: u64,
    pub length: u64,
}

impl AbiByteSlice {
    pub fn validate(self, region_length: u64) -> Result<()> {
        let end = self
            .offset
            .checked_add(self.length)
            .ok_or(WorkerError::SharedRegionOutOfBounds)?;
        if self.region_handle == 0
            || self.length == 0
            || region_length == 0
            || region_length > MAX_SHARED_REGION_BYTES
            || end > region_length
        {
            return Err(WorkerError::SharedRegionOutOfBounds);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct AbiOwnedBuffer {
    pub allocator_handle: u64,
    pub buffer_handle: u64,
    pub length: u64,
}

impl AbiOwnedBuffer {
    pub fn validate(self) -> Result<()> {
        if self.allocator_handle == 0
            || self.buffer_handle == 0
            || self.length == 0
            || self.length > MAX_SHARED_REGION_BYTES
        {
            return Err(WorkerError::OwnedBufferInvalid);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SharedRegionDescriptor {
    pub region_digest: Digest,
    pub region_length: u64,
    pub slice: AbiByteSlice,
    pub sealed_backend_generation: u64,
    pub read_only: bool,
}

impl SharedRegionDescriptor {
    pub fn validate(&self, backend_generation: u64) -> Result<()> {
        if self.sealed_backend_generation != backend_generation {
            return Err(WorkerError::StaleBackendGeneration);
        }
        if !self.read_only {
            return Err(WorkerError::SharedRegionWritable);
        }
        self.slice.validate(self.region_length)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeWorkerRequest {
    pub request_id: RequestId,
    pub request_generation: u64,
    pub backend_generation: u64,
    pub tuple_digest: Digest,
    pub prompt: SharedRegionDescriptor,
    pub output_token_limit: u32,
}

impl NativeWorkerRequest {
    pub fn validate(&self) -> Result<()> {
        if self.request_generation == 0 || self.backend_generation == 0 {
            return Err(WorkerError::InvalidGeneration);
        }
        if self.output_token_limit == 0 {
            return Err(WorkerError::OutputLimitEmpty);
        }
        self.prompt.validate(self.backend_generation)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkerHealth {
    Stopped,
    Ready,
    Draining,
    FailedClosed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkerFault {
    Crash,
    OutOfMemory,
    Deadlock,
    ProtocolViolation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkerFaultReceipt {
    pub fault: WorkerFault,
    pub previous_backend_generation: u64,
    pub backend_generation: u64,
    pub affected_requests: Vec<RequestId>,
    pub forced_worker_termination: bool,
    pub remote_fallback_attempted: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkerSnapshot {
    pub backend_generation: u64,
    pub health: WorkerHealth,
    pub active_requests: usize,
    pub max_active_requests: usize,
    pub tcp_listener: bool,
    pub remote_endpoint: bool,
    pub hepta_state_write: bool,
}

#[derive(Clone, Debug)]
struct ActiveRequest {
    request_generation: u64,
    backend_generation: u64,
}

#[derive(Debug)]
pub struct WorkerSupervisor {
    registry: NativeWorkerRegistry,
    backend_generation: u64,
    health: WorkerHealth,
    max_active_requests: usize,
    active: BTreeMap<RequestId, ActiveRequest>,
}

impl WorkerSupervisor {
    pub fn new(
        registry: NativeWorkerRegistry,
        backend_generation: u64,
        max_active_requests: usize,
    ) -> Result<Self> {
        if backend_generation == 0
            || max_active_requests == 0
            || max_active_requests > MAX_NATIVE_WORKER_ACTIVE
        {
            return Err(WorkerError::SupervisorConfigInvalid);
        }
        Ok(Self {
            registry,
            backend_generation,
            health: WorkerHealth::Stopped,
            max_active_requests,
            active: BTreeMap::new(),
        })
    }

    pub const fn backend_generation(&self) -> u64 {
        self.backend_generation
    }

    pub fn start(&mut self, expected_generation: u64) -> Result<()> {
        self.require_generation(expected_generation)?;
        if !matches!(
            self.health,
            WorkerHealth::Stopped | WorkerHealth::FailedClosed
        ) || !self.active.is_empty()
        {
            return Err(WorkerError::InvalidWorkerState);
        }
        self.health = WorkerHealth::Ready;
        Ok(())
    }

    pub fn submit(&mut self, request: NativeWorkerRequest) -> Result<()> {
        if self.health != WorkerHealth::Ready {
            return Err(WorkerError::WorkerNotReady);
        }
        self.require_generation(request.backend_generation)?;
        request.validate()?;
        self.registry.admit(&request.tuple_digest)?;
        if self.active.len() >= self.max_active_requests {
            return Err(WorkerError::WorkerQueueFull);
        }
        let request_id = request.request_id;
        if self.active.contains_key(&request_id) {
            return Err(WorkerError::DuplicateRequest);
        }
        let active = ActiveRequest {
            request_generation: request.request_generation,
            backend_generation: request.backend_generation,
        };
        self.active.insert(request_id, active);
        Ok(())
    }

    pub fn complete(
        &mut self,
        request_id: &RequestId,
        request_generation: u64,
        backend_generation: u64,
    ) -> Result<()> {
        self.remove_active(request_id, request_generation, backend_generation)
    }

    pub fn cancel(
        &mut self,
        request_id: &RequestId,
        request_generation: u64,
        backend_generation: u64,
    ) -> Result<()> {
        self.remove_active(request_id, request_generation, backend_generation)
    }

    pub fn begin_drain(&mut self, expected_generation: u64) -> Result<()> {
        self.require_generation(expected_generation)?;
        if self.health != WorkerHealth::Ready {
            return Err(WorkerError::InvalidWorkerState);
        }
        self.health = WorkerHealth::Draining;
        Ok(())
    }

    pub fn finish_drain(&mut self, expected_generation: u64) -> Result<()> {
        self.require_generation(expected_generation)?;
        if self.health != WorkerHealth::Draining || !self.active.is_empty() {
            return Err(WorkerError::InvalidWorkerState);
        }
        self.health = WorkerHealth::Stopped;
        Ok(())
    }

    pub fn fail_closed(&mut self, fault: WorkerFault) -> Result<WorkerFaultReceipt> {
        let previous = self.backend_generation;
        let next = previous
            .checked_add(1)
            .ok_or(WorkerError::GenerationOverflow)?;
        let affected_requests = self.active.keys().cloned().collect();
        self.active.clear();
        self.backend_generation = next;
        self.health = WorkerHealth::FailedClosed;
        Ok(WorkerFaultReceipt {
            fault,
            previous_backend_generation: previous,
            backend_generation: next,
            affected_requests,
            forced_worker_termination: true,
            remote_fallback_attempted: false,
        })
    }

    pub fn controlled_restart(&mut self, expected_generation: u64) -> Result<WorkerFaultReceipt> {
        self.require_generation(expected_generation)?;
        self.fail_closed(WorkerFault::ProtocolViolation)
    }

    pub fn snapshot(&self) -> WorkerSnapshot {
        WorkerSnapshot {
            backend_generation: self.backend_generation,
            health: self.health,
            active_requests: self.active.len(),
            max_active_requests: self.max_active_requests,
            tcp_listener: false,
            remote_endpoint: false,
            hepta_state_write: false,
        }
    }

    fn remove_active(
        &mut self,
        request_id: &RequestId,
        request_generation: u64,
        backend_generation: u64,
    ) -> Result<()> {
        self.require_generation(backend_generation)?;
        let active = self
            .active
            .get(request_id)
            .ok_or(WorkerError::UnknownRequest)?;
        if active.request_generation != request_generation {
            return Err(WorkerError::StaleRequestGeneration);
        }
        if active.backend_generation != backend_generation {
            return Err(WorkerError::StaleBackendGeneration);
        }
        self.active.remove(request_id);
        Ok(())
    }

    fn require_generation(&self, backend_generation: u64) -> Result<()> {
        if backend_generation == self.backend_generation {
            Ok(())
        } else {
            Err(WorkerError::StaleBackendGeneration)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkerError {
    AbiContractInvalid,
    ManifestInvalid,
    RegistryEmpty,
    DuplicateTuple,
    UnknownTuple,
    KnownGapNotRouted,
    UnsupportedFailClosed,
    SharedRegionOutOfBounds,
    SharedRegionWritable,
    OwnedBufferInvalid,
    InvalidGeneration,
    OutputLimitEmpty,
    SupervisorConfigInvalid,
    InvalidWorkerState,
    WorkerNotReady,
    WorkerQueueFull,
    DuplicateRequest,
    UnknownRequest,
    StaleRequestGeneration,
    StaleBackendGeneration,
    GenerationOverflow,
}

impl WorkerError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::AbiContractInvalid => "INF_WORKER_ABI_CONTRACT_INVALID",
            Self::ManifestInvalid => "INF_WORKER_MANIFEST_INVALID",
            Self::RegistryEmpty => "INF_WORKER_REGISTRY_EMPTY",
            Self::DuplicateTuple => "INF_WORKER_DUPLICATE_TUPLE",
            Self::UnknownTuple => "INF_WORKER_UNKNOWN_TUPLE",
            Self::KnownGapNotRouted => "INF_WORKER_KNOWN_GAP_NOT_ROUTED",
            Self::UnsupportedFailClosed => "INF_WORKER_UNSUPPORTED_FAIL_CLOSED",
            Self::SharedRegionOutOfBounds => "INF_WORKER_SHARED_REGION_OUT_OF_BOUNDS",
            Self::SharedRegionWritable => "INF_WORKER_SHARED_REGION_WRITABLE",
            Self::OwnedBufferInvalid => "INF_WORKER_OWNED_BUFFER_INVALID",
            Self::InvalidGeneration => "INF_WORKER_INVALID_GENERATION",
            Self::OutputLimitEmpty => "INF_WORKER_OUTPUT_LIMIT_EMPTY",
            Self::SupervisorConfigInvalid => "INF_WORKER_SUPERVISOR_CONFIG_INVALID",
            Self::InvalidWorkerState => "INF_WORKER_STATE_INVALID",
            Self::WorkerNotReady => "INF_WORKER_NOT_READY",
            Self::WorkerQueueFull => "INF_WORKER_QUEUE_FULL",
            Self::DuplicateRequest => "INF_WORKER_DUPLICATE_REQUEST",
            Self::UnknownRequest => "INF_WORKER_UNKNOWN_REQUEST",
            Self::StaleRequestGeneration => "INF_WORKER_STALE_REQUEST_GENERATION",
            Self::StaleBackendGeneration => "INF_WORKER_STALE_BACKEND_GENERATION",
            Self::GenerationOverflow => "INF_WORKER_GENERATION_OVERFLOW",
        }
    }
}

impl fmt::Display for WorkerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for WorkerError {}

pub type Result<T> = std::result::Result<T, WorkerError>;

fn valid_backend_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

fn is_lower_hex_commit(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn is_allowed_quantization(value: &str) -> bool {
    matches!(value, "Q4_K_M" | "Q5_K_M" | "Q6_K" | "Q8_0" | "F16")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn must<T, E: fmt::Display>(value: std::result::Result<T, E>) -> T {
        match value {
            Ok(value) => value,
            Err(error) => panic!("unexpected error: {error}"),
        }
    }

    fn digest(fill: char) -> Digest {
        must(Digest::parse(&format!(
            "sha256:{}",
            fill.to_string().repeat(64)
        )))
    }

    fn request_id(value: &str) -> RequestId {
        must(RequestId::parse(value))
    }

    fn manifest(disposition: WorkerQualificationDisposition) -> GgufModelManifest {
        GgufModelManifest {
            tuple_digest: digest('a'),
            model_digest: digest('b'),
            tokenizer_digest: digest('c'),
            gguf_artifact_digest: digest('d'),
            sbom_digest: digest('e'),
            license_digest: digest('f'),
            device_profile_digest: digest('1'),
            backend: BackendAbiContract::pinned_llama_cpp(),
            quantization: "Q4_K_M".to_owned(),
            disposition,
        }
    }

    fn descriptor(generation: u64) -> SharedRegionDescriptor {
        SharedRegionDescriptor {
            region_digest: digest('2'),
            region_length: 4096,
            slice: AbiByteSlice {
                region_handle: 1,
                offset: 128,
                length: 512,
            },
            sealed_backend_generation: generation,
            read_only: true,
        }
    }

    fn request(id: &str, generation: u64) -> NativeWorkerRequest {
        NativeWorkerRequest {
            request_id: request_id(id),
            request_generation: 1,
            backend_generation: generation,
            tuple_digest: digest('a'),
            prompt: descriptor(generation),
            output_token_limit: 32,
        }
    }

    #[test]
    fn pinned_abi_is_complete_and_rejects_commit_drift() {
        let mut contract = BackendAbiContract::pinned_llama_cpp();
        assert!(contract.validate().is_ok());
        assert_eq!(contract.operations, REQUIRED_BACKEND_OPERATIONS);
        contract.upstream_commit = "0000000000000000000000000000000000000000".to_owned();
        let mut drifted = manifest(WorkerQualificationDisposition::KnownGapNotRouted);
        drifted.backend = contract;
        assert_eq!(drifted.validate(), Err(WorkerError::ManifestInvalid));
    }

    #[test]
    fn known_gap_and_unsupported_tuples_never_dispatch() {
        let known_gap = must(NativeWorkerRegistry::new([manifest(
            WorkerQualificationDisposition::KnownGapNotRouted,
        )]));
        assert_eq!(
            known_gap.admit(&digest('a')),
            Err(WorkerError::KnownGapNotRouted)
        );
        let unsupported = must(NativeWorkerRegistry::new([manifest(
            WorkerQualificationDisposition::UnsupportedFailClosed,
        )]));
        assert_eq!(
            unsupported.admit(&digest('a')),
            Err(WorkerError::UnsupportedFailClosed)
        );
    }

    #[test]
    fn shared_region_is_read_only_generation_bound_and_overflow_safe() {
        assert!(descriptor(7).validate(7).is_ok());
        assert_eq!(
            descriptor(7).validate(8),
            Err(WorkerError::StaleBackendGeneration)
        );
        let mut writable = descriptor(7);
        writable.read_only = false;
        assert_eq!(writable.validate(7), Err(WorkerError::SharedRegionWritable));
        let overflowing = AbiByteSlice {
            region_handle: 1,
            offset: u64::MAX,
            length: 2,
        };
        assert_eq!(
            overflowing.validate(4096),
            Err(WorkerError::SharedRegionOutOfBounds)
        );
    }

    #[test]
    fn crash_oom_and_deadlock_increment_generation_and_fence_stale_work() {
        for fault in [
            WorkerFault::Crash,
            WorkerFault::OutOfMemory,
            WorkerFault::Deadlock,
        ] {
            let registry = must(NativeWorkerRegistry::new([manifest(
                WorkerQualificationDisposition::Qualified,
            )]));
            let mut supervisor = must(WorkerSupervisor::new(registry, 7, 4));
            must(supervisor.start(7));
            must(supervisor.submit(request("request-a", 7)));
            let receipt = must(supervisor.fail_closed(fault));
            assert_eq!(receipt.previous_backend_generation, 7);
            assert_eq!(receipt.backend_generation, 8);
            assert_eq!(receipt.affected_requests, vec![request_id("request-a")]);
            assert!(receipt.forced_worker_termination);
            assert!(!receipt.remote_fallback_attempted);
            assert_eq!(supervisor.snapshot().health, WorkerHealth::FailedClosed);
            must(supervisor.start(8));
            assert_eq!(
                supervisor.submit(request("request-stale", 7)),
                Err(WorkerError::StaleBackendGeneration)
            );
        }
    }

    #[test]
    fn draining_rejects_new_work_and_requires_empty_active_set() {
        let registry = must(NativeWorkerRegistry::new([manifest(
            WorkerQualificationDisposition::Qualified,
        )]));
        let mut supervisor = must(WorkerSupervisor::new(registry, 7, 4));
        must(supervisor.start(7));
        must(supervisor.submit(request("request-a", 7)));
        must(supervisor.begin_drain(7));
        assert_eq!(
            supervisor.submit(request("request-b", 7)),
            Err(WorkerError::WorkerNotReady)
        );
        assert_eq!(
            supervisor.finish_drain(7),
            Err(WorkerError::InvalidWorkerState)
        );
        must(supervisor.complete(&request_id("request-a"), 1, 7));
        must(supervisor.finish_drain(7));
        assert_eq!(supervisor.snapshot().health, WorkerHealth::Stopped);
    }
}
