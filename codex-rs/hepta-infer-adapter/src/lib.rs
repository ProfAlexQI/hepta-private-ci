//! Qualification-only adapter contracts for backends hosted behind `hepta-inferd`.
//!
//! This crate deliberately carries no raw prompt field, provider URL, network client,
//! model downloader, TCP listener, remote fallback, Memory/KG writer, or production
//! authority. It freezes exact-tuple capability and normalized-event semantics before
//! Ollama and LM Studio are wired behind the daemon.

use std::collections::HashMap;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use codex_hepta_infer_core::Digest;
use codex_hepta_infer_core::RequestId;

pub type AdapterFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, AdapterError>> + Send + 'a>>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum BackendId {
    Ollama = 1,
    LmStudio = 2,
}

impl BackendId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ollama => "ollama",
            Self::LmStudio => "lmstudio",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Capability {
    SemanticText = 1,
    NativeToolCall = 2,
    StrictSse = 3,
    ExplicitCancel = 4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityStatus {
    Qualified,
    UnsupportedFailClosed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapabilityMatrix {
    pub semantic_text: CapabilityStatus,
    pub native_tool_call: CapabilityStatus,
    pub strict_sse: CapabilityStatus,
    pub explicit_cancel: CapabilityStatus,
}

impl CapabilityMatrix {
    pub const fn status(self, capability: Capability) -> CapabilityStatus {
        match capability {
            Capability::SemanticText => self.semantic_text,
            Capability::NativeToolCall => self.native_tool_call,
            Capability::StrictSse => self.strict_sse,
            Capability::ExplicitCancel => self.explicit_cancel,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterProfile {
    pub backend_id: BackendId,
    pub model_tuple_digest: Digest,
    pub model_id_digest: Digest,
    pub backend_generation: u64,
    pub capabilities: CapabilityMatrix,
    pub loopback_only: bool,
    pub implicit_model_install: bool,
    pub remote_fallback: bool,
    pub text_fallback_for_tools: bool,
}

impl AdapterProfile {
    pub fn validate(&self) -> Result<(), AdapterError> {
        if self.backend_generation == 0 {
            return Err(AdapterError::InvalidGeneration);
        }
        if !self.loopback_only
            || self.implicit_model_install
            || self.remote_fallback
            || self.text_fallback_for_tools
        {
            return Err(AdapterError::UnsafeProfile);
        }
        if self.capabilities.semantic_text != CapabilityStatus::Qualified {
            return Err(AdapterError::SemanticTextNotQualified);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum QualificationFixture {
    SemanticHeptaOk = 1,
    NativeToolCallHeptaProbe = 2,
    StrictSseSequence = 3,
}

impl QualificationFixture {
    pub const fn required_capability(self) -> Capability {
        match self {
            Self::SemanticHeptaOk => Capability::SemanticText,
            Self::NativeToolCallHeptaProbe => Capability::NativeToolCall,
            Self::StrictSseSequence => Capability::StrictSse,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterRequest {
    pub request_id: RequestId,
    pub request_generation: u64,
    pub backend_generation: u64,
    pub model_tuple_digest: Digest,
    pub fixture: QualificationFixture,
}

impl AdapterRequest {
    pub fn validate(&self) -> Result<(), AdapterError> {
        if self.request_generation == 0 || self.backend_generation == 0 {
            return Err(AdapterError::InvalidGeneration);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum NormalizedEventKind {
    Running = 1,
    OutputDigest = 2,
    ToolCallDigest = 3,
    Completed = 100,
    Cancelled = 101,
    FailedClosed = 102,
}

impl NormalizedEventKind {
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Cancelled | Self::FailedClosed)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedEvent {
    pub request_id: RequestId,
    pub request_generation: u64,
    pub backend_generation: u64,
    pub sequence: u64,
    pub kind: NormalizedEventKind,
    pub payload_digest: Option<Digest>,
    pub payload_byte_length: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterExecutionReceipt {
    pub backend_id: BackendId,
    pub model_tuple_digest: Digest,
    pub request_id: RequestId,
    pub request_generation: u64,
    pub backend_generation: u64,
    pub terminal_kind: NormalizedEventKind,
    pub last_sequence: u64,
    pub result_digest: Option<Digest>,
    pub result_byte_length: u64,
    pub raw_prompt_persisted: bool,
    pub raw_output_persisted: bool,
    pub remote_fallback_used: bool,
    pub implicit_model_install_used: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterHealth {
    pub backend_id: BackendId,
    pub model_tuple_digest: Digest,
    pub backend_generation: u64,
    pub ready: bool,
}

pub trait LocalInferenceAdapter: Send + Sync {
    fn profile(&self) -> &AdapterProfile;

    fn probe(&self) -> AdapterFuture<'_, AdapterHealth>;

    fn execute_fixture<'a>(
        &'a self,
        request: &'a AdapterRequest,
    ) -> AdapterFuture<'a, Vec<NormalizedEvent>>;

    fn cancel<'a>(
        &'a self,
        request_id: &'a RequestId,
        request_generation: u64,
        cancel_generation: u64,
        backend_generation: u64,
    ) -> AdapterFuture<'a, NormalizedEvent>;
}

#[derive(Default)]
pub struct AdapterRegistry {
    adapters: HashMap<Digest, Arc<dyn LocalInferenceAdapter>>,
}

impl AdapterRegistry {
    pub fn register(
        &mut self,
        adapter: Arc<dyn LocalInferenceAdapter>,
    ) -> Result<(), AdapterError> {
        adapter.profile().validate()?;
        let key = adapter.profile().model_tuple_digest.clone();
        if self.adapters.insert(key, adapter).is_some() {
            return Err(AdapterError::DuplicateTuple);
        }
        Ok(())
    }

    pub async fn probe(&self, tuple: &Digest) -> Result<AdapterHealth, AdapterError> {
        let adapter = self.adapter(tuple)?;
        let health = adapter.probe().await?;
        validate_health(adapter.profile(), &health)?;
        Ok(health)
    }

    pub async fn execute(
        &self,
        request: &AdapterRequest,
    ) -> Result<AdapterExecutionReceipt, AdapterError> {
        request.validate()?;
        let adapter = self.adapter(&request.model_tuple_digest)?;
        let profile = adapter.profile();
        if request.backend_generation != profile.backend_generation {
            return Err(AdapterError::StaleBackendGeneration);
        }
        let capability = request.fixture.required_capability();
        if profile.capabilities.status(capability) != CapabilityStatus::Qualified {
            return Err(AdapterError::CapabilityUnsupported(capability));
        }
        let events = adapter.execute_fixture(request).await?;
        validate_events(profile, request, &events)
    }

    pub async fn cancel(
        &self,
        tuple: &Digest,
        request_id: &RequestId,
        request_generation: u64,
        cancel_generation: u64,
        backend_generation: u64,
    ) -> Result<NormalizedEvent, AdapterError> {
        if request_generation == 0 || cancel_generation == 0 || backend_generation == 0 {
            return Err(AdapterError::InvalidGeneration);
        }
        let adapter = self.adapter(tuple)?;
        let profile = adapter.profile();
        if backend_generation != profile.backend_generation {
            return Err(AdapterError::StaleBackendGeneration);
        }
        if profile.capabilities.explicit_cancel != CapabilityStatus::Qualified {
            return Err(AdapterError::CapabilityUnsupported(Capability::ExplicitCancel));
        }
        let event = adapter
            .cancel(
                request_id,
                request_generation,
                cancel_generation,
                backend_generation,
            )
            .await?;
        if event.request_id != *request_id
            || event.request_generation != request_generation
            || event.backend_generation != backend_generation
            || event.sequence == 0
            || event.kind != NormalizedEventKind::Cancelled
            || event.payload_digest.is_some()
            || event.payload_byte_length != 0
        {
            return Err(AdapterError::InvalidEvent);
        }
        Ok(event)
    }

    fn adapter(&self, tuple: &Digest) -> Result<&Arc<dyn LocalInferenceAdapter>, AdapterError> {
        self.adapters.get(tuple).ok_or(AdapterError::UnknownTuple)
    }
}

fn validate_health(
    profile: &AdapterProfile,
    health: &AdapterHealth,
) -> Result<(), AdapterError> {
    if health.backend_id != profile.backend_id
        || health.model_tuple_digest != profile.model_tuple_digest
        || health.backend_generation != profile.backend_generation
        || !health.ready
    {
        return Err(AdapterError::InvalidHealth);
    }
    Ok(())
}

fn validate_events(
    profile: &AdapterProfile,
    request: &AdapterRequest,
    events: &[NormalizedEvent],
) -> Result<AdapterExecutionReceipt, AdapterError> {
    if events.len() < 2 {
        return Err(AdapterError::InvalidEvent);
    }
    let mut previous = 0;
    let mut terminal = None;
    for event in events {
        if event.request_id != request.request_id
            || event.request_generation != request.request_generation
            || event.backend_generation != request.backend_generation
            || event.sequence != previous + 1
        {
            return Err(AdapterError::InvalidEvent);
        }
        if terminal.is_some() {
            return Err(AdapterError::EventAfterTerminal);
        }
        if event.kind.is_terminal() {
            terminal = Some(event);
        }
        previous = event.sequence;
    }
    let terminal = terminal.ok_or(AdapterError::MissingTerminal)?;
    if terminal.kind != NormalizedEventKind::Completed {
        return Err(AdapterError::BackendFailedClosed);
    }
    if terminal.payload_byte_length == 0 || terminal.payload_digest.is_none() {
        return Err(AdapterError::InvalidEvent);
    }
    Ok(AdapterExecutionReceipt {
        backend_id: profile.backend_id,
        model_tuple_digest: profile.model_tuple_digest.clone(),
        request_id: request.request_id.clone(),
        request_generation: request.request_generation,
        backend_generation: request.backend_generation,
        terminal_kind: terminal.kind,
        last_sequence: terminal.sequence,
        result_digest: terminal.payload_digest.clone(),
        result_byte_length: terminal.payload_byte_length,
        raw_prompt_persisted: false,
        raw_output_persisted: false,
        remote_fallback_used: false,
        implicit_model_install_used: false,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdapterError {
    BackendFailedClosed,
    CapabilityUnsupported(Capability),
    DuplicateTuple,
    EventAfterTerminal,
    InvalidEvent,
    InvalidGeneration,
    InvalidHealth,
    MissingTerminal,
    SemanticTextNotQualified,
    StaleBackendGeneration,
    UnknownTuple,
    UnsafeProfile,
}

impl AdapterError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::BackendFailedClosed => "INF_ADAPTER_BACKEND_FAILED_CLOSED",
            Self::CapabilityUnsupported(_) => "INF_ADAPTER_CAPABILITY_UNSUPPORTED",
            Self::DuplicateTuple => "INF_ADAPTER_DUPLICATE_TUPLE",
            Self::EventAfterTerminal => "INF_ADAPTER_EVENT_AFTER_TERMINAL",
            Self::InvalidEvent => "INF_ADAPTER_INVALID_EVENT",
            Self::InvalidGeneration => "INF_ADAPTER_INVALID_GENERATION",
            Self::InvalidHealth => "INF_ADAPTER_INVALID_HEALTH",
            Self::MissingTerminal => "INF_ADAPTER_MISSING_TERMINAL",
            Self::SemanticTextNotQualified => "INF_ADAPTER_SEMANTIC_TEXT_NOT_QUALIFIED",
            Self::StaleBackendGeneration => "INF_ADAPTER_STALE_BACKEND_GENERATION",
            Self::UnknownTuple => "INF_ADAPTER_UNKNOWN_TUPLE",
            Self::UnsafeProfile => "INF_ADAPTER_UNSAFE_PROFILE",
        }
    }
}

impl fmt::Display for AdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CapabilityUnsupported(capability) => {
                write!(formatter, "{}: {capability:?}", self.code())
            }
            _ => formatter.write_str(self.code()),
        }
    }
}

impl std::error::Error for AdapterError {}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    use super::*;

    fn digest(fill: char) -> Digest {
        match Digest::parse(&format!("sha256:{}", fill.to_string().repeat(64))) {
            Ok(value) => value,
            Err(error) => panic!("unexpected digest error: {error}"),
        }
    }

    fn request_id() -> RequestId {
        match RequestId::parse("request-a") {
            Ok(value) => value,
            Err(error) => panic!("unexpected request id error: {error}"),
        }
    }

    struct DeterministicAdapter {
        profile: AdapterProfile,
        executions: AtomicUsize,
    }

    impl DeterministicAdapter {
        fn new(capabilities: CapabilityMatrix) -> Self {
            Self {
                profile: AdapterProfile {
                    backend_id: BackendId::Ollama,
                    model_tuple_digest: digest('a'),
                    model_id_digest: digest('b'),
                    backend_generation: 7,
                    capabilities,
                    loopback_only: true,
                    implicit_model_install: false,
                    remote_fallback: false,
                    text_fallback_for_tools: false,
                },
                executions: AtomicUsize::new(0),
            }
        }
    }

    impl LocalInferenceAdapter for DeterministicAdapter {
        fn profile(&self) -> &AdapterProfile {
            &self.profile
        }

        fn probe(&self) -> AdapterFuture<'_, AdapterHealth> {
            Box::pin(async move {
                Ok(AdapterHealth {
                    backend_id: self.profile.backend_id,
                    model_tuple_digest: self.profile.model_tuple_digest.clone(),
                    backend_generation: self.profile.backend_generation,
                    ready: true,
                })
            })
        }

        fn execute_fixture<'a>(
            &'a self,
            request: &'a AdapterRequest,
        ) -> AdapterFuture<'a, Vec<NormalizedEvent>> {
            self.executions.fetch_add(1, Ordering::SeqCst);
            Box::pin(async move {
                let digest = match request.fixture {
                    QualificationFixture::SemanticHeptaOk => digest('c'),
                    QualificationFixture::NativeToolCallHeptaProbe => digest('d'),
                    QualificationFixture::StrictSseSequence => digest('e'),
                };
                Ok(vec![
                    NormalizedEvent {
                        request_id: request.request_id.clone(),
                        request_generation: request.request_generation,
                        backend_generation: request.backend_generation,
                        sequence: 1,
                        kind: NormalizedEventKind::Running,
                        payload_digest: None,
                        payload_byte_length: 0,
                    },
                    NormalizedEvent {
                        request_id: request.request_id.clone(),
                        request_generation: request.request_generation,
                        backend_generation: request.backend_generation,
                        sequence: 2,
                        kind: NormalizedEventKind::Completed,
                        payload_digest: Some(digest),
                        payload_byte_length: 14,
                    },
                ])
            })
        }

        fn cancel<'a>(
            &'a self,
            request_id: &'a RequestId,
            request_generation: u64,
            _cancel_generation: u64,
            backend_generation: u64,
        ) -> AdapterFuture<'a, NormalizedEvent> {
            Box::pin(async move {
                Ok(NormalizedEvent {
                    request_id: request_id.clone(),
                    request_generation,
                    backend_generation,
                    sequence: 3,
                    kind: NormalizedEventKind::Cancelled,
                    payload_digest: None,
                    payload_byte_length: 0,
                })
            })
        }
    }

    const QUALIFIED: CapabilityStatus = CapabilityStatus::Qualified;
    const UNSUPPORTED: CapabilityStatus = CapabilityStatus::UnsupportedFailClosed;

    fn matrix(
        tool: CapabilityStatus,
        sse: CapabilityStatus,
        cancel: CapabilityStatus,
    ) -> CapabilityMatrix {
        CapabilityMatrix {
            semantic_text: QUALIFIED,
            native_tool_call: tool,
            strict_sse: sse,
            explicit_cancel: cancel,
        }
    }

    fn request(fixture: QualificationFixture) -> AdapterRequest {
        AdapterRequest {
            request_id: request_id(),
            request_generation: 1,
            backend_generation: 7,
            model_tuple_digest: digest('a'),
            fixture,
        }
    }

    #[tokio::test]
    async fn exact_tuple_semantic_fixture_normalizes_terminal_receipt() {
        let adapter = Arc::new(DeterministicAdapter::new(matrix(
            QUALIFIED,
            UNSUPPORTED,
            UNSUPPORTED,
        )));
        let mut registry = AdapterRegistry::default();
        if let Err(error) = registry.register(adapter.clone()) {
            panic!("unexpected registration error: {error}");
        }
        let health = match registry.probe(&digest('a')).await {
            Ok(value) => value,
            Err(error) => panic!("unexpected probe error: {error}"),
        };
        assert!(health.ready);
        let receipt = match registry
            .execute(&request(QualificationFixture::SemanticHeptaOk))
            .await
        {
            Ok(value) => value,
            Err(error) => panic!("unexpected execution error: {error}"),
        };
        assert_eq!(receipt.terminal_kind, NormalizedEventKind::Completed);
        assert_eq!(receipt.last_sequence, 2);
        assert!(!receipt.raw_prompt_persisted);
        assert!(!receipt.raw_output_persisted);
        assert!(!receipt.remote_fallback_used);
        assert!(!receipt.implicit_model_install_used);
        assert_eq!(adapter.executions.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn unsupported_capability_rejects_before_backend_dispatch() {
        let adapter = Arc::new(DeterministicAdapter::new(matrix(
            UNSUPPORTED,
            UNSUPPORTED,
            UNSUPPORTED,
        )));
        let mut registry = AdapterRegistry::default();
        if let Err(error) = registry.register(adapter.clone()) {
            panic!("unexpected registration error: {error}");
        }
        let error = match registry
            .execute(&request(QualificationFixture::NativeToolCallHeptaProbe))
            .await
        {
            Ok(_) => panic!("unsupported tool call unexpectedly executed"),
            Err(error) => error,
        };
        assert_eq!(
            error,
            AdapterError::CapabilityUnsupported(Capability::NativeToolCall)
        );
        assert_eq!(adapter.executions.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn unknown_tuple_and_stale_generation_fail_before_dispatch() {
        let adapter = Arc::new(DeterministicAdapter::new(matrix(
            QUALIFIED,
            UNSUPPORTED,
            UNSUPPORTED,
        )));
        let mut registry = AdapterRegistry::default();
        if let Err(error) = registry.register(adapter.clone()) {
            panic!("unexpected registration error: {error}");
        }

        let mut unknown = request(QualificationFixture::SemanticHeptaOk);
        unknown.model_tuple_digest = digest('f');
        assert_eq!(
            registry.execute(&unknown).await,
            Err(AdapterError::UnknownTuple)
        );

        let mut stale = request(QualificationFixture::SemanticHeptaOk);
        stale.backend_generation = 6;
        assert_eq!(
            registry.execute(&stale).await,
            Err(AdapterError::StaleBackendGeneration)
        );
        assert_eq!(adapter.executions.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn explicit_cancel_is_capability_gated() {
        let unsupported = Arc::new(DeterministicAdapter::new(matrix(
            QUALIFIED,
            UNSUPPORTED,
            UNSUPPORTED,
        )));
        let mut registry = AdapterRegistry::default();
        if let Err(error) = registry.register(unsupported) {
            panic!("unexpected registration error: {error}");
        }
        assert_eq!(
            registry.cancel(&digest('a'), &request_id(), 1, 1, 7).await,
            Err(AdapterError::CapabilityUnsupported(
                Capability::ExplicitCancel
            ))
        );

        let supported = Arc::new(DeterministicAdapter::new(matrix(
            QUALIFIED,
            UNSUPPORTED,
            QUALIFIED,
        )));
        let mut registry = AdapterRegistry::default();
        if let Err(error) = registry.register(supported) {
            panic!("unexpected registration error: {error}");
        }
        let event = match registry.cancel(&digest('a'), &request_id(), 1, 1, 7).await {
            Ok(value) => value,
            Err(error) => panic!("unexpected cancellation error: {error}"),
        };
        assert_eq!(event.kind, NormalizedEventKind::Cancelled);
    }

    #[test]
    fn unsafe_profiles_are_rejected() {
        let mut profile = DeterministicAdapter::new(matrix(
            QUALIFIED,
            UNSUPPORTED,
            UNSUPPORTED,
        ))
        .profile;
        profile.remote_fallback = true;
        assert_eq!(profile.validate(), Err(AdapterError::UnsafeProfile));
        profile.remote_fallback = false;
        profile.implicit_model_install = true;
        assert_eq!(profile.validate(), Err(AdapterError::UnsafeProfile));
        profile.implicit_model_install = false;
        profile.text_fallback_for_tools = true;
        assert_eq!(profile.validate(), Err(AdapterError::UnsafeProfile));
        profile.text_fallback_for_tools = false;
        profile.loopback_only = false;
        assert_eq!(profile.validate(), Err(AdapterError::UnsafeProfile));
    }
}
