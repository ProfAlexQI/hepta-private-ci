//! Bounded owner-local client for the qualification-only Hepta inference controller.
//!
//! The client transports only canonical digest/identity/control messages. It has no
//! raw prompt field, HTTP fallback, model installer, remote route, or production
//! authority. Product callers must opt into a separately qualified shadow route.

use std::fmt;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use codex_hepta_infer_core::ClientMessage;
use codex_hepta_infer_core::ControllerSnapshot;
use codex_hepta_infer_core::Digest;
use codex_hepta_infer_core::InferenceRequest;
use codex_hepta_infer_core::MAX_FRAME_BYTES;
use codex_hepta_infer_core::ServerMessage;
use codex_uds::UnixStream;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::time::timeout;

pub type Result<T> = std::result::Result<T, ClientError>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InferenceCapability {
    SemanticText,
    NativeToolCall,
    StrictSse,
    DirectBackendCancelAcknowledgement,
}

impl InferenceCapability {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SemanticText => "semantic_text",
            Self::NativeToolCall => "native_tool_call",
            Self::StrictSse => "strict_sse",
            Self::DirectBackendCancelAcknowledgement => "direct_backend_cancel_acknowledgement",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityDisposition {
    Qualified,
    UnsupportedFailClosed,
    KnownGapNotRouted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteMode {
    ShadowCompareOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactCapabilityProfile {
    model_tuple_digest: Digest,
    semantic_text: CapabilityDisposition,
    native_tool_call: CapabilityDisposition,
    strict_sse: CapabilityDisposition,
    direct_backend_cancel_acknowledgement: CapabilityDisposition,
}

impl ExactCapabilityProfile {
    pub fn new(
        model_tuple_digest: Digest,
        semantic_text: CapabilityDisposition,
        native_tool_call: CapabilityDisposition,
        strict_sse: CapabilityDisposition,
        direct_backend_cancel_acknowledgement: CapabilityDisposition,
    ) -> Self {
        Self {
            model_tuple_digest,
            semantic_text,
            native_tool_call,
            strict_sse,
            direct_backend_cancel_acknowledgement,
        }
    }

    pub const fn model_tuple_digest(&self) -> &Digest {
        &self.model_tuple_digest
    }

    pub const fn route_mode(&self) -> RouteMode {
        RouteMode::ShadowCompareOnly
    }

    pub const fn disposition(&self, capability: InferenceCapability) -> CapabilityDisposition {
        match capability {
            InferenceCapability::SemanticText => self.semantic_text,
            InferenceCapability::NativeToolCall => self.native_tool_call,
            InferenceCapability::StrictSse => self.strict_sse,
            InferenceCapability::DirectBackendCancelAcknowledgement => {
                self.direct_backend_cancel_acknowledgement
            }
        }
    }

    pub fn require(&self, tuple: &Digest, capability: InferenceCapability) -> Result<()> {
        if tuple != &self.model_tuple_digest {
            return Err(ClientError::ModelTupleNotRouted);
        }
        match self.disposition(capability) {
            CapabilityDisposition::Qualified => Ok(()),
            CapabilityDisposition::UnsupportedFailClosed => {
                Err(ClientError::CapabilityUnsupported(capability))
            }
            CapabilityDisposition::KnownGapNotRouted => {
                Err(ClientError::CapabilityKnownGap(capability))
            }
        }
    }
}

#[derive(Debug)]
pub enum ClientError {
    Config(&'static str),
    ConnectTimeout,
    ExchangeTimeout,
    Io(io::Error),
    Protocol(codex_hepta_infer_core::InferError),
    Remote(String),
    UnexpectedResponse(&'static str),
    ModelTupleNotRouted,
    CapabilityUnsupported(InferenceCapability),
    CapabilityKnownGap(InferenceCapability),
}

impl ClientError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Config(_) => "INF_CLIENT_CONFIG",
            Self::ConnectTimeout => "INF_CLIENT_CONNECT_TIMEOUT",
            Self::ExchangeTimeout => "INF_CLIENT_EXCHANGE_TIMEOUT",
            Self::Io(_) => "INF_CLIENT_IO",
            Self::Protocol(_) => "INF_CLIENT_PROTOCOL",
            Self::Remote(_) => "INF_CLIENT_REMOTE",
            Self::UnexpectedResponse(_) => "INF_CLIENT_UNEXPECTED_RESPONSE",
            Self::ModelTupleNotRouted => "INF_CLIENT_MODEL_TUPLE_NOT_ROUTED",
            Self::CapabilityUnsupported(_) => "INF_CLIENT_CAPABILITY_UNSUPPORTED",
            Self::CapabilityKnownGap(_) => "INF_CLIENT_CAPABILITY_KNOWN_GAP_NOT_ROUTED",
        }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(detail) | Self::UnexpectedResponse(detail) => {
                write!(formatter, "{}: {detail}", self.code())
            }
            Self::Io(error) => write!(formatter, "{}: {error}", self.code()),
            Self::Protocol(error) => write!(formatter, "{}: {error}", self.code()),
            Self::Remote(code) => write!(formatter, "{}: {code}", self.code()),
            Self::CapabilityUnsupported(capability) | Self::CapabilityKnownGap(capability) => {
                write!(formatter, "{}: {}", self.code(), capability.as_str())
            }
            Self::ConnectTimeout | Self::ExchangeTimeout | Self::ModelTupleNotRouted => {
                formatter.write_str(self.code())
            }
        }
    }
}

impl std::error::Error for ClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Protocol(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for ClientError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<codex_hepta_infer_core::InferError> for ClientError {
    fn from(error: codex_hepta_infer_core::InferError) -> Self {
        Self::Protocol(error)
    }
}

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub socket_path: PathBuf,
    pub connect_timeout: Duration,
    pub exchange_timeout: Duration,
    pub max_frame_bytes: usize,
}

impl ClientConfig {
    pub fn qualification_only(socket_path: impl Into<PathBuf>) -> Self {
        Self {
            socket_path: socket_path.into(),
            connect_timeout: Duration::from_secs(2),
            exchange_timeout: Duration::from_secs(10),
            max_frame_bytes: MAX_FRAME_BYTES,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.socket_path.as_os_str().is_empty() {
            return Err(ClientError::Config("INF_CLIENT_SOCKET_PATH_EMPTY"));
        }
        if !self.socket_path.is_absolute() {
            return Err(ClientError::Config("INF_CLIENT_SOCKET_PATH_NOT_ABSOLUTE"));
        }
        socket_parent(&self.socket_path)?;
        if self.connect_timeout.is_zero() || self.exchange_timeout.is_zero() {
            return Err(ClientError::Config("INF_CLIENT_TIMEOUT_ZERO"));
        }
        if self.max_frame_bytes == 0 || self.max_frame_bytes > MAX_FRAME_BYTES {
            return Err(ClientError::Config("INF_CLIENT_FRAME_BOUND_INVALID"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct InferdClient {
    config: ClientConfig,
}

impl InferdClient {
    pub fn new(config: ClientConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    pub const fn config(&self) -> &ClientConfig {
        &self.config
    }

    pub async fn exchange(&self, message: ClientMessage) -> Result<ServerMessage> {
        let connect = UnixStream::connect(&self.config.socket_path);
        let mut stream = timeout(self.config.connect_timeout, connect)
            .await
            .map_err(|_| ClientError::ConnectTimeout)??;
        stream.ensure_current_user_peer()?;

        timeout(self.config.exchange_timeout, async {
            write_request(&mut stream, &message, self.config.max_frame_bytes).await?;
            let response = read_response(&mut stream, self.config.max_frame_bytes).await?;
            stream.shutdown().await?;
            Ok(response)
        })
        .await
        .map_err(|_| ClientError::ExchangeTimeout)?
    }

    pub async fn ping(&self, nonce: u64) -> Result<()> {
        match self.exchange(ClientMessage::Ping { nonce }).await? {
            ServerMessage::Pong { nonce: echoed } if echoed == nonce => Ok(()),
            ServerMessage::Error { code } => Err(ClientError::Remote(code)),
            _ => Err(ClientError::UnexpectedResponse("INF_CLIENT_PONG_MISMATCH")),
        }
    }

    pub async fn snapshot(&self) -> Result<ControllerSnapshot> {
        match self.exchange(ClientMessage::Snapshot).await? {
            ServerMessage::Snapshot(snapshot) => {
                snapshot.authority.validate_closed()?;
                Ok(snapshot)
            }
            ServerMessage::Error { code } => Err(ClientError::Remote(code)),
            _ => Err(ClientError::UnexpectedResponse(
                "INF_CLIENT_SNAPSHOT_RESPONSE_MISMATCH",
            )),
        }
    }

    pub async fn admit(&self, request: InferenceRequest) -> Result<ServerMessage> {
        self.expect_non_error(ClientMessage::Admit(request)).await
    }

    pub async fn cancel(
        &self,
        request_id: codex_hepta_infer_core::RequestId,
        request_generation: u64,
        cancel_generation: u64,
        backend_generation: u64,
    ) -> Result<ServerMessage> {
        self.expect_non_error(ClientMessage::Cancel {
            request_id,
            request_generation,
            cancel_generation,
            backend_generation,
        })
        .await
    }

    pub async fn restart_backend(&self, expected_generation: u64) -> Result<ServerMessage> {
        self.expect_non_error(ClientMessage::RestartBackend {
            expected_generation,
        })
        .await
    }

    async fn expect_non_error(&self, message: ClientMessage) -> Result<ServerMessage> {
        match self.exchange(message).await? {
            ServerMessage::Error { code } => Err(ClientError::Remote(code)),
            response => Ok(response),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShadowInferdClient {
    transport: InferdClient,
    capability_profile: ExactCapabilityProfile,
}

impl ShadowInferdClient {
    pub fn new(transport: InferdClient, capability_profile: ExactCapabilityProfile) -> Self {
        Self {
            transport,
            capability_profile,
        }
    }

    pub const fn route_mode(&self) -> RouteMode {
        RouteMode::ShadowCompareOnly
    }

    pub const fn capability_profile(&self) -> &ExactCapabilityProfile {
        &self.capability_profile
    }

    pub async fn admit(
        &self,
        capability: InferenceCapability,
        request: InferenceRequest,
    ) -> Result<ServerMessage> {
        self.capability_profile
            .require(&request.model_tuple_digest, capability)?;
        self.transport.admit(request).await
    }

    pub async fn ping(&self, nonce: u64) -> Result<()> {
        self.transport.ping(nonce).await
    }

    pub async fn snapshot(&self) -> Result<ControllerSnapshot> {
        self.transport.snapshot().await
    }

    pub async fn cancel_controller(
        &self,
        request_id: codex_hepta_infer_core::RequestId,
        request_generation: u64,
        cancel_generation: u64,
        backend_generation: u64,
    ) -> Result<ServerMessage> {
        self.transport
            .cancel(
                request_id,
                request_generation,
                cancel_generation,
                backend_generation,
            )
            .await
    }
}

async fn write_request(
    stream: &mut UnixStream,
    message: &ClientMessage,
    max_frame_bytes: usize,
) -> Result<()> {
    let bytes = message.encode_canonical()?;
    if bytes.is_empty() || bytes.len() > max_frame_bytes {
        return Err(ClientError::Config(
            "INF_CLIENT_REQUEST_FRAME_OUT_OF_BOUNDS",
        ));
    }
    let length = u32::try_from(bytes.len())
        .map_err(|_| ClientError::Config("INF_CLIENT_REQUEST_FRAME_TOO_LARGE"))?;
    stream.write_all(&length.to_be_bytes()).await?;
    stream.write_all(&bytes).await?;
    stream.flush().await?;
    Ok(())
}

async fn read_response(stream: &mut UnixStream, max_frame_bytes: usize) -> Result<ServerMessage> {
    let mut length_bytes = [0u8; 4];
    stream.read_exact(&mut length_bytes).await?;
    let length = usize::try_from(u32::from_be_bytes(length_bytes))
        .map_err(|_| io::Error::new(ErrorKind::InvalidData, "INF_CLIENT_FRAME_LENGTH_INVALID"))?;
    if length == 0 || length > max_frame_bytes {
        return Err(ClientError::Io(io::Error::new(
            ErrorKind::InvalidData,
            "INF_CLIENT_RESPONSE_FRAME_OUT_OF_BOUNDS",
        )));
    }
    let mut bytes = vec![0u8; length];
    stream.read_exact(&mut bytes).await?;
    ServerMessage::decode_canonical(&bytes).map_err(ClientError::Protocol)
}

pub fn socket_parent(path: &Path) -> Result<&Path> {
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .ok_or(ClientError::Config("INF_CLIENT_SOCKET_PARENT_MISSING"))
}

#[cfg(test)]
mod tests;
