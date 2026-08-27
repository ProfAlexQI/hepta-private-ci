use std::fmt;

use crate::MAX_DENIAL_CODE_BYTES;
use crate::MAX_FIXTURE_ID_BYTES;
use crate::MAX_HUMAN_LEASE_MS;
use crate::MAX_OBSERVE_NODES;
use crate::MAX_REFERENCE_BYTES;
use crate::MAX_TEXT_BYTES;
use crate::ProtocolError;
use crate::constant_time_eq;
use crate::invalid;

const QUALIFICATION_ONLY_BIT: u16 = 1 << 0;
const PRODUCTION_CALLER_BIT: u16 = 1 << 1;
const PRODUCTION_WRITER_BIT: u16 = 1 << 2;
const EFFECT_AUTHORITY_BIT: u16 = 1 << 3;
const EXTERNAL_EFFECT_BIT: u16 = 1 << 4;
const OPERATOR_ACCEPTANCE_BIT: u16 = 1 << 5;
const PROMOTION_BIT: u16 = 1 << 6;
const G5_ALLOWED_BIT: u16 = 1 << 7;
const EXECUTE_ALLOWED_BIT: u16 = 1 << 8;
const EXTERNAL_NETWORK_BIT: u16 = 1 << 9;
const CREDENTIAL_EXPORT_BIT: u16 = 1 << 10;
const KNOWN_POSTURE_BITS: u16 = QUALIFICATION_ONLY_BIT
    | PRODUCTION_CALLER_BIT
    | PRODUCTION_WRITER_BIT
    | EFFECT_AUTHORITY_BIT
    | EXTERNAL_EFFECT_BIT
    | OPERATOR_ACCEPTANCE_BIT
    | PROMOTION_BIT
    | G5_ALLOWED_BIT
    | EXECUTE_ALLOWED_BIT
    | EXTERNAL_NETWORK_BIT
    | CREDENTIAL_EXPORT_BIT;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct BrowserSessionId(pub(crate) [u8; 32]);

impl BrowserSessionId {
    pub fn new(bytes: [u8; 32]) -> Result<Self, ProtocolError> {
        require_nonzero(&bytes, "browser session id must not be all zero")?;
        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for BrowserSessionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("BrowserSessionId")
            .field(&Hex(&self.0))
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct StartupCapability(pub(crate) [u8; 32]);

impl StartupCapability {
    pub fn new(bytes: [u8; 32]) -> Result<Self, ProtocolError> {
        require_nonzero(&bytes, "startup capability must not be all zero")?;
        Ok(Self(bytes))
    }

    pub fn matches(&self, other: &Self) -> bool {
        constant_time_eq(&self.0, &other.0)
    }
}

impl fmt::Debug for StartupCapability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("StartupCapability(<redacted>)")
    }
}

impl Drop for StartupCapability {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkerIdentity {
    pub session_id: BrowserSessionId,
    pub generation: u64,
    pub owner_epoch: u64,
}

impl WorkerIdentity {
    pub fn new(
        session_id: BrowserSessionId,
        generation: u64,
        owner_epoch: u64,
    ) -> Result<Self, ProtocolError> {
        if generation == 0 || owner_epoch == 0 {
            return Err(invalid(
                "worker generation and owner epoch must both be nonzero",
            ));
        }
        Ok(Self {
            session_id,
            generation,
            owner_epoch,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourcePin {
    pub(crate) servo_commit: [u8; 40],
    pub(crate) servo_tree: [u8; 40],
}

impl SourcePin {
    pub fn new(servo_commit: &str, servo_tree: &str) -> Result<Self, ProtocolError> {
        Ok(Self {
            servo_commit: parse_lower_hex_40(servo_commit)?,
            servo_tree: parse_lower_hex_40(servo_tree)?,
        })
    }

    pub fn servo_commit_hex(&self) -> String {
        String::from_utf8_lossy(&self.servo_commit).into_owned()
    }

    pub fn servo_tree_hex(&self) -> String {
        String::from_utf8_lossy(&self.servo_tree).into_owned()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthorityPosture {
    pub qualification_only: bool,
    pub production_caller: bool,
    pub production_writer: bool,
    pub effect_authority: bool,
    pub external_effect: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub g5_allowed: bool,
    pub execute_allowed: bool,
    pub external_network: bool,
    pub credential_export: bool,
}

impl AuthorityPosture {
    pub const fn qualification_only() -> Self {
        Self {
            qualification_only: true,
            production_caller: false,
            production_writer: false,
            effect_authority: false,
            external_effect: false,
            operator_acceptance: false,
            promotion: false,
            g5_allowed: false,
            execute_allowed: false,
            external_network: false,
            credential_export: false,
        }
    }

    pub fn validate(self) -> Result<(), ProtocolError> {
        if self != Self::qualification_only() {
            return Err(invalid(
                "C1 protocol accepts only the qualification-only negative-authority posture",
            ));
        }
        Ok(())
    }

    pub fn wire_bits(self) -> Result<u16, ProtocolError> {
        self.validate()?;
        Ok(QUALIFICATION_ONLY_BIT)
    }

    pub fn from_wire_bits(bits: u16) -> Result<Self, ProtocolError> {
        if bits & !KNOWN_POSTURE_BITS != 0 || bits != QUALIFICATION_ONLY_BIT {
            return Err(invalid(
                "worker authority posture contains an unknown or enabled authority bit",
            ));
        }
        Ok(Self::qualification_only())
    }
}

impl Default for AuthorityPosture {
    fn default() -> Self {
        Self::qualification_only()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkerHello {
    pub identity: WorkerIdentity,
    pub source_pin: SourcePin,
    pub authority: AuthorityPosture,
    pub(crate) startup_capability: StartupCapability,
}

impl WorkerHello {
    pub fn new(
        identity: WorkerIdentity,
        source_pin: SourcePin,
        startup_capability: StartupCapability,
    ) -> Result<Self, ProtocolError> {
        let authority = AuthorityPosture::qualification_only();
        authority.validate()?;
        Ok(Self {
            identity,
            source_pin,
            authority,
            startup_capability,
        })
    }

    pub fn startup_capability_matches(&self, expected: &StartupCapability) -> bool {
        self.startup_capability.matches(expected)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct HostExpectedWorker {
    pub identity: WorkerIdentity,
    pub source_pin: SourcePin,
    pub startup_capability: StartupCapability,
    pub(crate) host_nonce: [u8; 32],
}

impl HostExpectedWorker {
    pub fn new(
        identity: WorkerIdentity,
        source_pin: SourcePin,
        startup_capability: StartupCapability,
        host_nonce: [u8; 32],
    ) -> Result<Self, ProtocolError> {
        require_nonzero(&host_nonce, "host nonce must not be all zero")?;
        Ok(Self {
            identity,
            source_pin,
            startup_capability,
            host_nonce,
        })
    }

    pub fn worker_hello(&self) -> Result<WorkerHello, ProtocolError> {
        WorkerHello::new(
            self.identity,
            self.source_pin,
            self.startup_capability.clone(),
        )
    }

    pub fn host_nonce(&self) -> &[u8; 32] {
        &self.host_nonce
    }
}

impl fmt::Debug for HostExpectedWorker {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostExpectedWorker")
            .field("identity", &self.identity)
            .field("source_pin", &self.source_pin)
            .field("startup_capability", &"<redacted>")
            .field("host_nonce", &"<redacted>")
            .finish()
    }
}

impl Drop for HostExpectedWorker {
    fn drop(&mut self) {
        self.host_nonce.fill(0);
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct HostAck {
    pub identity: WorkerIdentity,
    pub accepted: bool,
    pub(crate) host_nonce: [u8; 32],
}

impl HostAck {
    pub fn accepted(identity: WorkerIdentity, host_nonce: [u8; 32]) -> Result<Self, ProtocolError> {
        require_nonzero(&host_nonce, "host acknowledgement nonce must not be all zero")?;
        Ok(Self {
            identity,
            accepted: true,
            host_nonce,
        })
    }

    pub fn host_nonce_matches(&self, expected: &[u8; 32]) -> bool {
        constant_time_eq(&self.host_nonce, expected)
    }
}

impl fmt::Debug for HostAck {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostAck")
            .field("identity", &self.identity)
            .field("accepted", &self.accepted)
            .field("host_nonce", &"<redacted>")
            .finish()
    }
}

impl Drop for HostAck {
    fn drop(&mut self) {
        self.host_nonce.fill(0);
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct WorkerConfirm {
    pub identity: WorkerIdentity,
    pub(crate) host_nonce: [u8; 32],
}

impl WorkerConfirm {
    pub fn new(identity: WorkerIdentity, host_nonce: [u8; 32]) -> Result<Self, ProtocolError> {
        require_nonzero(&host_nonce, "worker confirmation nonce must not be all zero")?;
        Ok(Self {
            identity,
            host_nonce,
        })
    }

    pub fn host_nonce_matches(&self, expected: &[u8; 32]) -> bool {
        constant_time_eq(&self.host_nonce, expected)
    }
}

impl fmt::Debug for WorkerConfirm {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkerConfirm")
            .field("identity", &self.identity)
            .field("host_nonce", &"<redacted>")
            .finish()
    }
}

impl Drop for WorkerConfirm {
    fn drop(&mut self) {
        self.host_nonce.fill(0);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstablishedBinding {
    pub identity: WorkerIdentity,
    pub source_pin: SourcePin,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandFrame {
    pub request_id: u64,
    pub identity: WorkerIdentity,
    pub page_revision: u64,
    pub command: CommandKind,
}

impl CommandFrame {
    pub fn new(
        request_id: u64,
        identity: WorkerIdentity,
        page_revision: u64,
        command: CommandKind,
    ) -> Result<Self, ProtocolError> {
        if request_id == 0 || page_revision == 0 {
            return Err(invalid(
                "worker command request id and page revision must be nonzero",
            ));
        }
        command.validate()?;
        Ok(Self {
            request_id,
            identity,
            page_revision,
            command,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommandKind {
    Ping,
    NavigateLocal { fixture_id: String },
    Observe { limit: u16 },
    Click { semantic_ref: String },
    TypeText { semantic_ref: String, text: String },
    HumanTakeover { lease_ms: u32 },
    HumanRelease,
    Shutdown,
}

impl CommandKind {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        match self {
            Self::Ping | Self::HumanRelease | Self::Shutdown => Ok(()),
            Self::NavigateLocal { fixture_id } => validate_fixture_id(fixture_id),
            Self::Observe { limit } => {
                if !(1..=MAX_OBSERVE_NODES).contains(limit) {
                    return Err(invalid("observe node limit is outside the C1 bound"));
                }
                Ok(())
            }
            Self::Click { semantic_ref } => validate_reference(semantic_ref),
            Self::TypeText {
                semantic_ref,
                text,
            } => {
                validate_reference(semantic_ref)?;
                if text.is_empty() || text.len() > MAX_TEXT_BYTES || text.contains('\0') {
                    return Err(invalid("typed text is empty, oversized, or contains NUL"));
                }
                Ok(())
            }
            Self::HumanTakeover { lease_ms } => {
                if !(1..=MAX_HUMAN_LEASE_MS).contains(lease_ms) {
                    return Err(invalid("human takeover lease is outside the C1 bound"));
                }
                Ok(())
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutcomeStatus {
    Completed,
    Denied,
    Stale,
    Invalid,
    Indeterminate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutcomeFrame {
    pub request_id: u64,
    pub identity: WorkerIdentity,
    pub page_revision: u64,
    pub status: OutcomeStatus,
    pub code: String,
}

impl OutcomeFrame {
    pub fn new(
        request_id: u64,
        identity: WorkerIdentity,
        page_revision: u64,
        status: OutcomeStatus,
        code: impl Into<String>,
    ) -> Result<Self, ProtocolError> {
        let code = code.into();
        if request_id == 0
            || page_revision == 0
            || code.is_empty()
            || code.len() > MAX_DENIAL_CODE_BYTES
            || !code.bytes().all(is_safe_identifier_byte)
        {
            return Err(invalid("worker outcome identity or code is invalid"));
        }
        Ok(Self {
            request_id,
            identity,
            page_revision,
            status,
            code,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    WorkerHello(WorkerHello),
    HostAck(HostAck),
    WorkerConfirm(WorkerConfirm),
    Command(CommandFrame),
    Outcome(OutcomeFrame),
}

fn validate_fixture_id(value: &str) -> Result<(), ProtocolError> {
    if value.is_empty()
        || value.len() > MAX_FIXTURE_ID_BYTES
        || value.contains("://")
        || value.starts_with('/')
        || !value.bytes().all(is_safe_identifier_byte)
    {
        return Err(invalid(
            "local fixture id is empty, external, absolute, oversized, or noncanonical",
        ));
    }
    Ok(())
}

fn validate_reference(value: &str) -> Result<(), ProtocolError> {
    if value.is_empty()
        || value.len() > MAX_REFERENCE_BYTES
        || !value.bytes().all(is_safe_identifier_byte)
    {
        return Err(invalid("semantic reference is empty, oversized, or noncanonical"));
    }
    Ok(())
}

fn is_safe_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':' | b'/')
}

fn parse_lower_hex_40(value: &str) -> Result<[u8; 40], ProtocolError> {
    let bytes = value.as_bytes();
    if bytes.len() != 40
        || !bytes
            .iter()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(invalid("source pin must be 40 lowercase hexadecimal bytes"));
    }
    let mut output = [0_u8; 40];
    output.copy_from_slice(bytes);
    Ok(output)
}

fn require_nonzero(bytes: &[u8], message: &'static str) -> Result<(), ProtocolError> {
    if bytes.iter().all(|byte| *byte == 0) {
        return Err(invalid(message));
    }
    Ok(())
}

struct Hex<'a>(&'a [u8]);

impl fmt::Debug for Hex<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(formatter, "{byte:02x}")?;
        }
        Ok(())
    }
}
