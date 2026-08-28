use std::fmt;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;

use crate::browser_contracts::BrowserAuthorityStatus;
use crate::browser_contracts::BrowserRequest;
use crate::browser_contracts::BrowserResponse;
use crate::browser_contracts::BrowserSessionId;
use crate::digest::framed_digest;
use crate::request::canonical_json;

pub const BROWSER_WORKER_PROTOCOL_SCHEMA_VERSION: u32 = 1;
pub const MAX_BROWSER_WORKER_FRAME_BYTES: usize = 65_536;
pub const BROWSER_WORKER_STARTUP_CAPABILITY_BYTES: usize = 32;
const MAX_PROTOCOL_ERROR_MESSAGE_BYTES: usize = 512;
const STARTUP_CAPABILITY_DOMAIN: &[u8] = b"hepta.browser.worker-startup-capability:v1";

#[derive(Debug, thiserror::Error)]
pub enum BrowserWorkerProtocolError {
    #[error("browser worker transport I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("browser worker frame serialization failed: {0}")]
    Serialization(String),
    #[error("browser worker frame is empty")]
    EmptyFrame,
    #[error("browser worker frame exceeds the {MAX_BROWSER_WORKER_FRAME_BYTES}-byte bound")]
    FrameTooLarge,
    #[error("browser worker frame is not valid strict JSON: {0}")]
    InvalidJson(String),
    #[error("browser worker frame is not compact canonical JSON")]
    NonCanonicalFrame,
    #[error("browser worker frame uses an unsupported schema")]
    UnsupportedSchema,
    #[error("browser worker frame belongs to another session")]
    WrongSession,
    #[error("browser worker frame belongs to another generation")]
    WrongGeneration,
    #[error("browser worker startup capability is invalid")]
    WrongStartupCapability,
    #[error("browser worker frame sequence is stale, duplicated or skipped")]
    WrongSequence,
    #[error("browser worker received an unexpected frame for its channel state")]
    UnexpectedFrame,
    #[error("browser worker authority posture is not closed")]
    AuthorityOpen,
    #[error("browser worker transport kind does not match the launch contract")]
    WrongTransport,
    #[error("browser worker channel is closed")]
    ChannelClosed,
    #[error("browser worker protocol value is invalid: {0}")]
    Invalid(String),
}

pub struct BrowserWorkerStartupCapability([u8; BROWSER_WORKER_STARTUP_CAPABILITY_BYTES]);

impl BrowserWorkerStartupCapability {
    pub fn generate() -> Self {
        Self(rand::random())
    }

    pub fn from_bytes(bytes: [u8; BROWSER_WORKER_STARTUP_CAPABILITY_BYTES]) -> Self {
        Self(bytes)
    }

    pub fn digest(&self) -> String {
        framed_digest(STARTUP_CAPABILITY_DOMAIN, [self.0.as_slice()])
    }
}

impl Drop for BrowserWorkerStartupCapability {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

impl fmt::Debug for BrowserWorkerStartupCapability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("BrowserWorkerStartupCapability([REDACTED])")
    }
}

impl PartialEq for BrowserWorkerStartupCapability {
    fn eq(&self, other: &Self) -> bool {
        constant_time_bytes_eq(&self.0, &other.0)
    }
}

impl Eq for BrowserWorkerStartupCapability {}

impl Serialize for BrowserWorkerStartupCapability {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&encode_hex(&self.0))
    }
}

impl<'de> Deserialize<'de> for BrowserWorkerStartupCapability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded = String::deserialize(deserializer)?;
        decode_hex_32(&encoded)
            .map(Self)
            .ok_or_else(|| serde::de::Error::custom("invalid browser worker startup capability"))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserWorkerTransportKind {
    UnixInheritedSocketPair,
    WindowsSidNamedPipe,
    QualificationStdioPipe,
}

impl BrowserWorkerTransportKind {
    pub fn is_private(self) -> bool {
        matches!(
            self,
            Self::UnixInheritedSocketPair
                | Self::WindowsSidNamedPipe
                | Self::QualificationStdioPipe
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserWorkerShutdownReason {
    Requested,
    GenerationFenced,
    ParentDraining,
    QualificationComplete,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserWorkerProtocolErrorCode {
    InvalidEnvelope,
    WrongCapability,
    WrongSequence,
    UnexpectedFrame,
    WorkerFailure,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum BrowserWorkerPayload {
    ParentHello {
        startup_capability: BrowserWorkerStartupCapability,
        parent_pid: u32,
    },
    WorkerReady {
        startup_capability_sha256: String,
        worker_pid: u32,
        transport: BrowserWorkerTransportKind,
        authority: BrowserAuthorityStatus,
    },
    Request {
        request: BrowserRequest,
    },
    Response {
        response: BrowserResponse,
    },
    Shutdown {
        reason: BrowserWorkerShutdownReason,
    },
    ShutdownAck,
    ProtocolError {
        code: BrowserWorkerProtocolErrorCode,
        message: String,
    },
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BrowserWorkerFrame {
    pub schema_version: u32,
    pub session_id: BrowserSessionId,
    pub generation: u64,
    pub sequence: u64,
    pub payload: BrowserWorkerPayload,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BrowserWorkerChannelState {
    AwaitingHello,
    AwaitingReady,
    Ready,
    Closing,
    Closed,
}

#[derive(Debug)]
pub enum BrowserWorkerParentEvent {
    Ready {
        worker_pid: u32,
        transport: BrowserWorkerTransportKind,
    },
    Response(BrowserResponse),
    ProtocolError {
        code: BrowserWorkerProtocolErrorCode,
        message: String,
    },
    ShutdownAck,
}

#[derive(Debug)]
pub enum BrowserWorkerServerEvent {
    HandshakeAccepted {
        ready: BrowserWorkerFrame,
        parent_pid: u32,
    },
    Request(BrowserRequest),
    Shutdown(BrowserWorkerShutdownReason),
}

#[derive(Debug)]
pub struct BrowserWorkerParentSession {
    session_id: BrowserSessionId,
    generation: u64,
    expected_capability_sha256: String,
    expected_transport: BrowserWorkerTransportKind,
    state: BrowserWorkerChannelState,
    next_outbound_sequence: u64,
    next_inbound_sequence: u64,
}

impl BrowserWorkerParentSession {
    pub fn begin(
        session_id: BrowserSessionId,
        generation: u64,
        transport: BrowserWorkerTransportKind,
        startup_capability: BrowserWorkerStartupCapability,
    ) -> Result<(Self, BrowserWorkerFrame), BrowserWorkerProtocolError> {
        if generation == 0 || !transport.is_private() {
            return Err(BrowserWorkerProtocolError::Invalid(
                "browser worker launch requires a nonzero generation and private transport"
                    .to_string(),
            ));
        }
        let expected_capability_sha256 = startup_capability.digest();
        let hello = BrowserWorkerFrame {
            schema_version: BROWSER_WORKER_PROTOCOL_SCHEMA_VERSION,
            session_id: session_id.clone(),
            generation,
            sequence: 0,
            payload: BrowserWorkerPayload::ParentHello {
                startup_capability,
                parent_pid: std::process::id(),
            },
        };
        Ok((
            Self {
                session_id,
                generation,
                expected_capability_sha256,
                expected_transport: transport,
                state: BrowserWorkerChannelState::AwaitingReady,
                next_outbound_sequence: 1,
                next_inbound_sequence: 1,
            },
            hello,
        ))
    }

    pub fn expected_capability_sha256(&self) -> &str {
        &self.expected_capability_sha256
    }

    pub fn is_ready(&self) -> bool {
        self.state == BrowserWorkerChannelState::Ready
    }

    pub fn is_closed(&self) -> bool {
        self.state == BrowserWorkerChannelState::Closed
    }

    pub fn next_request(
        &mut self,
        request: BrowserRequest,
    ) -> Result<BrowserWorkerFrame, BrowserWorkerProtocolError> {
        if self.state != BrowserWorkerChannelState::Ready {
            return self.fail(BrowserWorkerProtocolError::ChannelClosed);
        }
        if request.session_id != self.session_id {
            return self.fail(BrowserWorkerProtocolError::WrongSession);
        }
        if request.generation != self.generation {
            return self.fail(BrowserWorkerProtocolError::WrongGeneration);
        }
        let sequence = self.take_outbound_sequence()?;
        Ok(BrowserWorkerFrame {
            schema_version: BROWSER_WORKER_PROTOCOL_SCHEMA_VERSION,
            session_id: self.session_id.clone(),
            generation: self.generation,
            sequence,
            payload: BrowserWorkerPayload::Request { request },
        })
    }

    pub fn next_shutdown(
        &mut self,
        reason: BrowserWorkerShutdownReason,
    ) -> Result<BrowserWorkerFrame, BrowserWorkerProtocolError> {
        if self.state != BrowserWorkerChannelState::Ready {
            return self.fail(BrowserWorkerProtocolError::ChannelClosed);
        }
        let sequence = self.take_outbound_sequence()?;
        self.state = BrowserWorkerChannelState::Closing;
        Ok(BrowserWorkerFrame {
            schema_version: BROWSER_WORKER_PROTOCOL_SCHEMA_VERSION,
            session_id: self.session_id.clone(),
            generation: self.generation,
            sequence,
            payload: BrowserWorkerPayload::Shutdown { reason },
        })
    }

    pub fn accept(
        &mut self,
        frame: BrowserWorkerFrame,
    ) -> Result<BrowserWorkerParentEvent, BrowserWorkerProtocolError> {
        if self.state == BrowserWorkerChannelState::Closed {
            return Err(BrowserWorkerProtocolError::ChannelClosed);
        }
        if let Err(error) = validate_envelope(&self.session_id, self.generation, &frame) {
            return self.fail(error);
        }
        match self.state {
            BrowserWorkerChannelState::AwaitingReady => {
                if frame.sequence != 0 {
                    return self.fail(BrowserWorkerProtocolError::WrongSequence);
                }
                let BrowserWorkerPayload::WorkerReady {
                    startup_capability_sha256,
                    worker_pid,
                    transport,
                    authority,
                } = frame.payload
                else {
                    return self.fail(BrowserWorkerProtocolError::UnexpectedFrame);
                };
                if worker_pid == 0
                    || !valid_sha256(&startup_capability_sha256)
                    || !constant_time_text_eq(
                        &startup_capability_sha256,
                        &self.expected_capability_sha256,
                    )
                {
                    return self.fail(BrowserWorkerProtocolError::WrongStartupCapability);
                }
                if transport != self.expected_transport || !transport.is_private() {
                    return self.fail(BrowserWorkerProtocolError::WrongTransport);
                }
                if !authority.is_closed() {
                    return self.fail(BrowserWorkerProtocolError::AuthorityOpen);
                }
                self.state = BrowserWorkerChannelState::Ready;
                Ok(BrowserWorkerParentEvent::Ready {
                    worker_pid,
                    transport,
                })
            }
            BrowserWorkerChannelState::Ready => {
                self.require_inbound_sequence(frame.sequence)?;
                match frame.payload {
                    BrowserWorkerPayload::Response { response } => {
                        if response.session_id != self.session_id {
                            return self.fail(BrowserWorkerProtocolError::WrongSession);
                        }
                        if response.generation != self.generation {
                            return self.fail(BrowserWorkerProtocolError::WrongGeneration);
                        }
                        if !response.authority.is_closed() {
                            return self.fail(BrowserWorkerProtocolError::AuthorityOpen);
                        }
                        Ok(BrowserWorkerParentEvent::Response(response))
                    }
                    BrowserWorkerPayload::ProtocolError { code, message } => {
                        self.state = BrowserWorkerChannelState::Closed;
                        Ok(BrowserWorkerParentEvent::ProtocolError {
                            code,
                            message: bounded_message(message),
                        })
                    }
                    _ => self.fail(BrowserWorkerProtocolError::UnexpectedFrame),
                }
            }
            BrowserWorkerChannelState::Closing => {
                self.require_inbound_sequence(frame.sequence)?;
                if !matches!(frame.payload, BrowserWorkerPayload::ShutdownAck) {
                    return self.fail(BrowserWorkerProtocolError::UnexpectedFrame);
                }
                self.state = BrowserWorkerChannelState::Closed;
                Ok(BrowserWorkerParentEvent::ShutdownAck)
            }
            BrowserWorkerChannelState::AwaitingHello | BrowserWorkerChannelState::Closed => {
                self.fail(BrowserWorkerProtocolError::UnexpectedFrame)
            }
        }
    }

    fn take_outbound_sequence(&mut self) -> Result<u64, BrowserWorkerProtocolError> {
        let sequence = self.next_outbound_sequence;
        self.next_outbound_sequence = sequence.checked_add(1).ok_or_else(|| {
            self.state = BrowserWorkerChannelState::Closed;
            BrowserWorkerProtocolError::WrongSequence
        })?;
        Ok(sequence)
    }

    fn require_inbound_sequence(
        &mut self,
        sequence: u64,
    ) -> Result<(), BrowserWorkerProtocolError> {
        if sequence != self.next_inbound_sequence {
            return self.fail(BrowserWorkerProtocolError::WrongSequence);
        }
        self.next_inbound_sequence = sequence.checked_add(1).ok_or_else(|| {
            self.state = BrowserWorkerChannelState::Closed;
            BrowserWorkerProtocolError::WrongSequence
        })?;
        Ok(())
    }

    fn fail<T>(
        &mut self,
        error: BrowserWorkerProtocolError,
    ) -> Result<T, BrowserWorkerProtocolError> {
        self.state = BrowserWorkerChannelState::Closed;
        Err(error)
    }
}

#[derive(Debug)]
pub struct BrowserWorkerServerSession {
    session_id: BrowserSessionId,
    generation: u64,
    expected_capability_sha256: String,
    transport: BrowserWorkerTransportKind,
    state: BrowserWorkerChannelState,
    next_outbound_sequence: u64,
    next_inbound_sequence: u64,
}

impl BrowserWorkerServerSession {
    pub fn new(
        session_id: BrowserSessionId,
        generation: u64,
        expected_capability_sha256: String,
        transport: BrowserWorkerTransportKind,
    ) -> Result<Self, BrowserWorkerProtocolError> {
        if generation == 0 || !valid_sha256(&expected_capability_sha256) || !transport.is_private()
        {
            return Err(BrowserWorkerProtocolError::Invalid(
                "browser worker server launch contract is invalid".to_string(),
            ));
        }
        Ok(Self {
            session_id,
            generation,
            expected_capability_sha256,
            transport,
            state: BrowserWorkerChannelState::AwaitingHello,
            next_outbound_sequence: 1,
            next_inbound_sequence: 1,
        })
    }

    pub fn is_ready(&self) -> bool {
        self.state == BrowserWorkerChannelState::Ready
    }

    pub fn is_closed(&self) -> bool {
        self.state == BrowserWorkerChannelState::Closed
    }

    pub fn accept(
        &mut self,
        frame: BrowserWorkerFrame,
    ) -> Result<BrowserWorkerServerEvent, BrowserWorkerProtocolError> {
        if self.state == BrowserWorkerChannelState::Closed {
            return Err(BrowserWorkerProtocolError::ChannelClosed);
        }
        if let Err(error) = validate_envelope(&self.session_id, self.generation, &frame) {
            return self.fail(error);
        }
        match self.state {
            BrowserWorkerChannelState::AwaitingHello => {
                if frame.sequence != 0 {
                    return self.fail(BrowserWorkerProtocolError::WrongSequence);
                }
                let BrowserWorkerPayload::ParentHello {
                    startup_capability,
                    parent_pid,
                } = frame.payload
                else {
                    return self.fail(BrowserWorkerProtocolError::UnexpectedFrame);
                };
                let actual_digest = startup_capability.digest();
                if parent_pid == 0
                    || !constant_time_text_eq(&actual_digest, &self.expected_capability_sha256)
                {
                    return self.fail(BrowserWorkerProtocolError::WrongStartupCapability);
                }
                self.state = BrowserWorkerChannelState::Ready;
                let ready = BrowserWorkerFrame {
                    schema_version: BROWSER_WORKER_PROTOCOL_SCHEMA_VERSION,
                    session_id: self.session_id.clone(),
                    generation: self.generation,
                    sequence: 0,
                    payload: BrowserWorkerPayload::WorkerReady {
                        startup_capability_sha256: actual_digest,
                        worker_pid: std::process::id(),
                        transport: self.transport,
                        authority: BrowserAuthorityStatus::default(),
                    },
                };
                Ok(BrowserWorkerServerEvent::HandshakeAccepted { ready, parent_pid })
            }
            BrowserWorkerChannelState::Ready => {
                self.require_inbound_sequence(frame.sequence)?;
                match frame.payload {
                    BrowserWorkerPayload::Request { request } => {
                        if request.session_id != self.session_id {
                            return self.fail(BrowserWorkerProtocolError::WrongSession);
                        }
                        if request.generation != self.generation {
                            return self.fail(BrowserWorkerProtocolError::WrongGeneration);
                        }
                        Ok(BrowserWorkerServerEvent::Request(request))
                    }
                    BrowserWorkerPayload::Shutdown { reason } => {
                        self.state = BrowserWorkerChannelState::Closing;
                        Ok(BrowserWorkerServerEvent::Shutdown(reason))
                    }
                    _ => self.fail(BrowserWorkerProtocolError::UnexpectedFrame),
                }
            }
            BrowserWorkerChannelState::AwaitingReady
            | BrowserWorkerChannelState::Closing
            | BrowserWorkerChannelState::Closed => {
                self.fail(BrowserWorkerProtocolError::UnexpectedFrame)
            }
        }
    }

    pub fn next_response(
        &mut self,
        response: BrowserResponse,
    ) -> Result<BrowserWorkerFrame, BrowserWorkerProtocolError> {
        if self.state != BrowserWorkerChannelState::Ready {
            return self.fail(BrowserWorkerProtocolError::ChannelClosed);
        }
        if response.session_id != self.session_id {
            return self.fail(BrowserWorkerProtocolError::WrongSession);
        }
        if response.generation != self.generation {
            return self.fail(BrowserWorkerProtocolError::WrongGeneration);
        }
        if !response.authority.is_closed() {
            return self.fail(BrowserWorkerProtocolError::AuthorityOpen);
        }
        let sequence = self.take_outbound_sequence()?;
        Ok(BrowserWorkerFrame {
            schema_version: BROWSER_WORKER_PROTOCOL_SCHEMA_VERSION,
            session_id: self.session_id.clone(),
            generation: self.generation,
            sequence,
            payload: BrowserWorkerPayload::Response { response },
        })
    }

    pub fn next_protocol_error(
        &mut self,
        code: BrowserWorkerProtocolErrorCode,
        message: impl Into<String>,
    ) -> Result<BrowserWorkerFrame, BrowserWorkerProtocolError> {
        if self.state == BrowserWorkerChannelState::Closed {
            return Err(BrowserWorkerProtocolError::ChannelClosed);
        }
        let sequence = self.take_outbound_sequence()?;
        self.state = BrowserWorkerChannelState::Closed;
        Ok(BrowserWorkerFrame {
            schema_version: BROWSER_WORKER_PROTOCOL_SCHEMA_VERSION,
            session_id: self.session_id.clone(),
            generation: self.generation,
            sequence,
            payload: BrowserWorkerPayload::ProtocolError {
                code,
                message: bounded_message(message.into()),
            },
        })
    }

    pub fn next_shutdown_ack(&mut self) -> Result<BrowserWorkerFrame, BrowserWorkerProtocolError> {
        if self.state != BrowserWorkerChannelState::Closing {
            return self.fail(BrowserWorkerProtocolError::UnexpectedFrame);
        }
        let sequence = self.take_outbound_sequence()?;
        self.state = BrowserWorkerChannelState::Closed;
        Ok(BrowserWorkerFrame {
            schema_version: BROWSER_WORKER_PROTOCOL_SCHEMA_VERSION,
            session_id: self.session_id.clone(),
            generation: self.generation,
            sequence,
            payload: BrowserWorkerPayload::ShutdownAck,
        })
    }

    fn take_outbound_sequence(&mut self) -> Result<u64, BrowserWorkerProtocolError> {
        let sequence = self.next_outbound_sequence;
        self.next_outbound_sequence = sequence.checked_add(1).ok_or_else(|| {
            self.state = BrowserWorkerChannelState::Closed;
            BrowserWorkerProtocolError::WrongSequence
        })?;
        Ok(sequence)
    }

    fn require_inbound_sequence(
        &mut self,
        sequence: u64,
    ) -> Result<(), BrowserWorkerProtocolError> {
        if sequence != self.next_inbound_sequence {
            return self.fail(BrowserWorkerProtocolError::WrongSequence);
        }
        self.next_inbound_sequence = sequence.checked_add(1).ok_or_else(|| {
            self.state = BrowserWorkerChannelState::Closed;
            BrowserWorkerProtocolError::WrongSequence
        })?;
        Ok(())
    }

    fn fail<T>(
        &mut self,
        error: BrowserWorkerProtocolError,
    ) -> Result<T, BrowserWorkerProtocolError> {
        self.state = BrowserWorkerChannelState::Closed;
        Err(error)
    }
}

pub async fn write_browser_worker_frame<W>(
    writer: &mut W,
    frame: &BrowserWorkerFrame,
) -> Result<(), BrowserWorkerProtocolError>
where
    W: AsyncWrite + Unpin,
{
    let mut bytes = canonical_json(frame)
        .map_err(|error| BrowserWorkerProtocolError::Serialization(error.to_string()))?;
    if bytes.is_empty() {
        return Err(BrowserWorkerProtocolError::EmptyFrame);
    }
    if bytes.len() > MAX_BROWSER_WORKER_FRAME_BYTES {
        bytes.fill(0);
        return Err(BrowserWorkerProtocolError::FrameTooLarge);
    }
    let length = u32::try_from(bytes.len())
        .map_err(|_| BrowserWorkerProtocolError::FrameTooLarge)?
        .to_be_bytes();
    let result = async {
        writer.write_all(&length).await?;
        writer.write_all(&bytes).await?;
        writer.flush().await
    }
    .await;
    bytes.fill(0);
    result.map_err(BrowserWorkerProtocolError::Io)
}

pub async fn read_browser_worker_frame<R>(
    reader: &mut R,
) -> Result<BrowserWorkerFrame, BrowserWorkerProtocolError>
where
    R: AsyncRead + Unpin,
{
    let mut length = [0_u8; 4];
    reader.read_exact(&mut length).await?;
    let length = u32::from_be_bytes(length) as usize;
    if length == 0 {
        return Err(BrowserWorkerProtocolError::EmptyFrame);
    }
    if length > MAX_BROWSER_WORKER_FRAME_BYTES {
        return Err(BrowserWorkerProtocolError::FrameTooLarge);
    }
    let mut bytes = vec![0_u8; length];
    reader.read_exact(&mut bytes).await?;
    let frame: BrowserWorkerFrame = match serde_json::from_slice(&bytes) {
        Ok(frame) => frame,
        Err(error) => {
            bytes.fill(0);
            return Err(BrowserWorkerProtocolError::InvalidJson(error.to_string()));
        }
    };
    let mut canonical = canonical_json(&frame)
        .map_err(|error| BrowserWorkerProtocolError::Serialization(error.to_string()))?;
    let canonical_matches = constant_time_bytes_eq(&bytes, &canonical);
    bytes.fill(0);
    canonical.fill(0);
    if !canonical_matches {
        return Err(BrowserWorkerProtocolError::NonCanonicalFrame);
    }
    Ok(frame)
}

fn validate_envelope(
    expected_session_id: &BrowserSessionId,
    expected_generation: u64,
    frame: &BrowserWorkerFrame,
) -> Result<(), BrowserWorkerProtocolError> {
    if frame.schema_version != BROWSER_WORKER_PROTOCOL_SCHEMA_VERSION {
        Err(BrowserWorkerProtocolError::UnsupportedSchema)
    } else if &frame.session_id != expected_session_id {
        Err(BrowserWorkerProtocolError::WrongSession)
    } else if frame.generation != expected_generation || frame.generation == 0 {
        Err(BrowserWorkerProtocolError::WrongGeneration)
    } else {
        Ok(())
    }
}

fn bounded_message(message: String) -> String {
    message
        .chars()
        .take(MAX_PROTOCOL_ERROR_MESSAGE_BYTES)
        .collect()
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn constant_time_text_eq(left: &str, right: &str) -> bool {
    constant_time_bytes_eq(left.as_bytes(), right.as_bytes())
}

fn constant_time_bytes_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    left.iter()
        .zip(right)
        .fold(0_u8, |difference, (left, right)| {
            difference | (left ^ right)
        })
        == 0
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

fn decode_hex_32(encoded: &str) -> Option<[u8; BROWSER_WORKER_STARTUP_CAPABILITY_BYTES]> {
    if encoded.len() != BROWSER_WORKER_STARTUP_CAPABILITY_BYTES * 2 {
        return None;
    }
    let mut decoded = [0_u8; BROWSER_WORKER_STARTUP_CAPABILITY_BYTES];
    for (index, pair) in encoded.as_bytes().chunks_exact(2).enumerate() {
        let high = decode_hex_nibble(pair[0])?;
        let low = decode_hex_nibble(pair[1])?;
        decoded[index] = (high << 4) | low;
    }
    Some(decoded)
}

fn decode_hex_nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        _ => None,
    }
}
