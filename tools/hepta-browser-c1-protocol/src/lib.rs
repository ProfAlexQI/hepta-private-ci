//! Qualification-only private worker protocol for the Hepta Servo integration lane.
//!
//! This crate intentionally has no TCP listener, HTTP surface, Servo dependency,
//! credential export, production caller, external effect, or promotion authority.
//! It freezes the bounded inherited-channel handshake and command vocabulary that
//! a future pinned Servo worker must implement.

#![forbid(unsafe_code)]

mod codec;
mod protocol;
mod transport;

pub use codec::decode_message;
pub use codec::encode_message;
pub use codec::read_message;
pub use codec::write_message;
pub use protocol::AuthorityPosture;
pub use protocol::BrowserSessionId;
pub use protocol::CommandFrame;
pub use protocol::CommandKind;
pub use protocol::EstablishedBinding;
pub use protocol::HostAck;
pub use protocol::HostExpectedWorker;
pub use protocol::Message;
pub use protocol::OutcomeFrame;
pub use protocol::OutcomeStatus;
pub use protocol::SourcePin;
pub use protocol::StartupCapability;
pub use protocol::WorkerConfirm;
pub use protocol::WorkerHello;
pub use protocol::WorkerIdentity;
pub use transport::FramedChannel;
pub use transport::host_handshake;
pub use transport::worker_handshake;

pub const PROTOCOL_VERSION: u16 = 1;
pub const MAX_FRAME_BYTES: usize = 65_536;
pub const MAX_TEXT_BYTES: usize = 4_096;
pub const MAX_REFERENCE_BYTES: usize = 256;
pub const MAX_FIXTURE_ID_BYTES: usize = 128;
pub const MAX_DENIAL_CODE_BYTES: usize = 128;
pub const MAX_OBSERVE_NODES: u16 = 512;
pub const MAX_HUMAN_LEASE_MS: u32 = 300_000;

pub const QUALIFICATION_ONLY: bool = true;
pub const PRODUCTION_CALLER: bool = false;
pub const PRODUCTION_WRITER: bool = false;
pub const EFFECT_AUTHORITY: bool = false;
pub const EXTERNAL_EFFECT: bool = false;
pub const OPERATOR_ACCEPTANCE: bool = false;
pub const PROMOTION: bool = false;
pub const G5_ALLOWED: bool = false;
pub const EXECUTE_ALLOWED: bool = false;
pub const EXTERNAL_NETWORK_ALLOWED: bool = false;
pub const CREDENTIAL_EXPORT_ALLOWED: bool = false;

#[derive(Debug)]
pub enum ProtocolError {
    Invalid(&'static str),
    Io(std::io::Error),
    Utf8(std::str::Utf8Error),
    FrameTooLarge { length: usize, maximum: usize },
    UnexpectedMessage,
    AuthenticationFailed,
    StaleFence,
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(message) => formatter.write_str(message),
            Self::Io(error) => write!(formatter, "private worker protocol I/O failed: {error}"),
            Self::Utf8(error) => write!(formatter, "private worker protocol UTF-8 failed: {error}"),
            Self::FrameTooLarge { length, maximum } => write!(
                formatter,
                "private worker frame length {length} exceeds maximum {maximum}"
            ),
            Self::UnexpectedMessage => formatter.write_str("unexpected private worker message"),
            Self::AuthenticationFailed => {
                formatter.write_str("private worker startup authentication failed")
            }
            Self::StaleFence => formatter.write_str("private worker identity fence is stale"),
        }
    }
}

impl std::error::Error for ProtocolError {}

impl From<std::io::Error> for ProtocolError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<std::str::Utf8Error> for ProtocolError {
    fn from(error: std::str::Utf8Error) -> Self {
        Self::Utf8(error)
    }
}

pub(crate) fn invalid(message: &'static str) -> ProtocolError {
    ProtocolError::Invalid(message)
}

pub(crate) fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let mut difference = 0_u8;
    for (left_byte, right_byte) in left.iter().zip(right) {
        difference |= left_byte ^ right_byte;
    }
    difference == 0
}
