//! Typed local contracts for one Hepta workspace Agent's Matrix sidecar.
//!
//! This crate deliberately contains no Matrix SDK, SQLite, App Server, or
//! supervisor implementation. Matrix timeline events never implement this
//! owner-local control protocol and therefore cannot approve tools or cancel a
//! turn.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt;

use codex_hepta_contracts::AgentId;
use serde::Deserialize;
use serde::Serialize;

pub const MATRIX_BINDING_SCHEMA_VERSION: u32 = 1;
pub const MATRIXD_CONTROL_SCHEMA_VERSION: u32 = 1;
pub const MAX_MATRIXD_CONTROL_FRAME_BYTES: u64 = 65_536;
pub const MAX_MATRIXD_EVENT_BATCH: u16 = 256;
const MAX_MATRIX_IDENTIFIER_BYTES: usize = 255;
const MAX_MATRIX_BINDING_ENTRIES: usize = 256;

macro_rules! matrix_server_identifier {
    ($name:ident, $prefix:literal, $label:literal) => {
        #[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, MatrixProtocolError> {
                let value = value.into();
                validate_matrix_server_identifier(&value, $prefix, $label)?;
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        impl TryFrom<String> for $name {
            type Error = MatrixProtocolError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

matrix_server_identifier!(MatrixRoomId, '!', "Matrix room ID");
matrix_server_identifier!(MatrixUserId, '@', "Matrix user ID");

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct MatrixEventId(String);

impl MatrixEventId {
    pub fn parse(value: impl Into<String>) -> Result<Self, MatrixProtocolError> {
        let value = value.into();
        validate_opaque_identifier(&value, "Matrix event ID")?;
        if !value.starts_with('$') || value.len() == 1 {
            return Err(MatrixProtocolError::Invalid(
                "Matrix event ID must begin with '$' and include an opaque event identifier"
                    .to_string(),
            ));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MatrixEventId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<String> for MatrixEventId {
    type Error = MatrixProtocolError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<MatrixEventId> for String {
    fn from(value: MatrixEventId) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct MatrixDeviceId(String);

impl MatrixDeviceId {
    pub fn parse(value: impl Into<String>) -> Result<Self, MatrixProtocolError> {
        let value = value.into();
        validate_opaque_identifier(&value, "Matrix device ID")?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for MatrixDeviceId {
    type Error = MatrixProtocolError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<MatrixDeviceId> for String {
    fn from(value: MatrixDeviceId) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct MatrixTransactionId(String);

impl MatrixTransactionId {
    pub fn parse(value: impl Into<String>) -> Result<Self, MatrixProtocolError> {
        let value = value.into();
        validate_opaque_identifier(&value, "Matrix transaction ID")?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MatrixTransactionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<String> for MatrixTransactionId {
    type Error = MatrixProtocolError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<MatrixTransactionId> for String {
    fn from(value: MatrixTransactionId) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct MatrixHomeserverUrl(String);

impl MatrixHomeserverUrl {
    pub fn parse(value: impl Into<String>) -> Result<Self, MatrixProtocolError> {
        let value = value.into();
        let parsed = url::Url::parse(&value).map_err(|_| {
            MatrixProtocolError::Invalid("Matrix homeserver URL is invalid".to_string())
        })?;
        let secure = parsed.scheme() == "https";
        let local_http = parsed.scheme() == "http"
            && match parsed.host() {
                Some(url::Host::Domain(host)) => host.eq_ignore_ascii_case("localhost"),
                Some(url::Host::Ipv4(ip)) => ip.is_loopback(),
                Some(url::Host::Ipv6(ip)) => ip.is_loopback(),
                None => false,
            };
        if !secure && !local_http {
            return Err(MatrixProtocolError::Invalid(
                "Matrix homeserver must use HTTPS, except loopback HTTP in local tests".to_string(),
            ));
        }
        if parsed.username() != ""
            || parsed.password().is_some()
            || parsed.query().is_some()
            || parsed.fragment().is_some()
        {
            return Err(MatrixProtocolError::Invalid(
                "Matrix homeserver URL must not contain credentials, query, or fragment"
                    .to_string(),
            ));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for MatrixHomeserverUrl {
    type Error = MatrixProtocolError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<MatrixHomeserverUrl> for String {
    fn from(value: MatrixHomeserverUrl) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MatrixBindingV1 {
    pub schema_version: u32,
    pub agent_id: AgentId,
    pub revision: u64,
    pub homeserver: MatrixHomeserverUrl,
    pub expected_mxid: MatrixUserId,
    pub expected_device_id: MatrixDeviceId,
    pub allowed_rooms: Vec<MatrixRoomId>,
    pub allowed_senders: Vec<MatrixUserId>,
    pub require_explicit_mention: bool,
}

impl MatrixBindingV1 {
    pub fn validate(&self) -> Result<(), MatrixProtocolError> {
        if self.schema_version != MATRIX_BINDING_SCHEMA_VERSION || self.revision == 0 {
            return Err(MatrixProtocolError::Invalid(
                "Matrix binding schema and revision must be current and non-zero".to_string(),
            ));
        }
        validate_unique_bounded(&self.allowed_rooms, "allowed Matrix rooms")?;
        validate_unique_bounded(&self.allowed_senders, "allowed Matrix senders")?;
        if self.allowed_rooms.is_empty() || self.allowed_senders.is_empty() {
            return Err(MatrixProtocolError::Invalid(
                "Matrix binding must allow at least one exact room and sender".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MatrixdRequest {
    pub schema_version: u32,
    pub request_id: u64,
    pub agent_id: AgentId,
    pub expected_binding_revision: u64,
    pub expected_agent_generation: u64,
    pub method: MatrixdMethod,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum MatrixdMethod {
    Health,
    Snapshot,
    Events {
        after_cursor: u64,
        limit: u16,
    },
    CancelTurn {
        thread_id: String,
        turn_id: String,
    },
    ResolveApproval {
        approval_key: String,
        decision: LocalApprovalDecision,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalApprovalDecision {
    Accept,
    AcceptForSession,
    Decline,
    Cancel,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MatrixdResponse {
    pub schema_version: u32,
    pub request_id: u64,
    pub agent_id: AgentId,
    pub binding_revision: u64,
    pub agent_generation: u64,
    pub connection_epoch: u64,
    pub payload: MatrixdPayload,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum MatrixdPayload {
    Health(MatrixdHealth),
    Snapshot(MatrixdSnapshot),
    Events(MatrixdEventBatch),
    Accepted,
    Error { code: String, message: String },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MatrixdLifecycle {
    Starting,
    Syncing,
    Ready,
    Degraded,
    Draining,
    Fenced,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MatrixdHealth {
    pub lifecycle: MatrixdLifecycle,
    pub process_id: u32,
    pub agentd_connected: bool,
    pub matrix_sync_connected: bool,
    pub fenced: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MatrixdSnapshot {
    pub lifecycle: MatrixdLifecycle,
    pub expected_mxid: MatrixUserId,
    pub active_rooms: Vec<MatrixRoomId>,
    pub inbox_depth: u32,
    pub outbox_depth: u32,
    pub oldest_inbox_age_seconds: Option<u64>,
    pub oldest_outbox_age_seconds: Option<u64>,
    pub active_thread_id: Option<String>,
    pub active_turn_id: Option<String>,
    pub pending_approvals: u16,
    pub resync_required: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MatrixdEvent {
    pub cursor: u64,
    pub kind: MatrixdEventKind,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum MatrixdEventKind {
    Lifecycle {
        lifecycle: MatrixdLifecycle,
    },
    AgentConnection {
        connected: bool,
        generation: u64,
    },
    MatrixConnection {
        connected: bool,
    },
    QueueDepth {
        inbox: u32,
        outbox: u32,
    },
    TurnStarted {
        thread_id: String,
        turn_id: String,
    },
    TurnCompleted {
        thread_id: String,
        turn_id: String,
    },
    ApprovalPending {
        approval_key: String,
        thread_id: String,
        turn_id: String,
        kind: String,
    },
    ApprovalResolved {
        approval_key: String,
    },
    ResyncRequired {
        reason_code: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MatrixdEventBatch {
    pub events: Vec<MatrixdEvent>,
    pub gap: bool,
    pub next_cursor: u64,
    pub latest_cursor: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum MatrixProtocolError {
    #[error("invalid Matrix protocol value: {0}")]
    Invalid(String),
}

fn validate_matrix_server_identifier(
    value: &str,
    prefix: char,
    label: &str,
) -> Result<(), MatrixProtocolError> {
    validate_opaque_identifier(value, label)?;
    let Some((localpart, server_name)) = value.split_once(':') else {
        return Err(MatrixProtocolError::Invalid(format!(
            "{label} must begin with {prefix:?} and contain a server-name separator"
        )));
    };
    if !localpart.starts_with(prefix) || localpart.len() == 1 || server_name.is_empty() {
        return Err(MatrixProtocolError::Invalid(format!(
            "{label} must contain non-empty local and server-name parts"
        )));
    }
    Ok(())
}

fn validate_opaque_identifier(value: &str, label: &str) -> Result<(), MatrixProtocolError> {
    if value.is_empty()
        || value.len() > MAX_MATRIX_IDENTIFIER_BYTES
        || value.chars().any(char::is_control)
        || value.chars().any(char::is_whitespace)
    {
        return Err(MatrixProtocolError::Invalid(format!(
            "{label} must contain 1..={MAX_MATRIX_IDENTIFIER_BYTES} non-control, non-whitespace bytes"
        )));
    }
    Ok(())
}

fn validate_unique_bounded<T: Ord + Clone>(
    values: &[T],
    label: &str,
) -> Result<(), MatrixProtocolError> {
    if values.len() > MAX_MATRIX_BINDING_ENTRIES
        || values.iter().cloned().collect::<BTreeSet<_>>().len() != values.len()
    {
        return Err(MatrixProtocolError::Invalid(format!(
            "{label} must be unique and contain at most {MAX_MATRIX_BINDING_ENTRIES} entries"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn agent_id() -> AgentId {
        AgentId::parse("018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12").expect("valid agent id")
    }

    #[test]
    fn binding_is_exact_unique_and_secret_free() {
        let binding = MatrixBindingV1 {
            schema_version: MATRIX_BINDING_SCHEMA_VERSION,
            agent_id: agent_id(),
            revision: 7,
            homeserver: MatrixHomeserverUrl::parse("https://matrix.example.test")
                .expect("homeserver"),
            expected_mxid: MatrixUserId::parse("@hepta-a:example.test").expect("mxid"),
            expected_device_id: MatrixDeviceId::parse("HEPTA_A_DEVICE").expect("device id"),
            allowed_rooms: vec![MatrixRoomId::parse("!room-a:example.test").expect("room")],
            allowed_senders: vec![MatrixUserId::parse("@owner:example.test").expect("sender")],
            require_explicit_mention: true,
        };
        binding.validate().expect("binding should be valid");
        let json = serde_json::to_string(&binding).expect("serialize binding");
        assert!(!json.contains("token"));
        assert!(!json.contains("passphrase"));
        assert_eq!(
            serde_json::from_str::<MatrixBindingV1>(&json).expect("parse binding"),
            binding
        );
    }

    #[test]
    fn insecure_remote_homeservers_and_identifier_drift_are_rejected() {
        assert!(MatrixHomeserverUrl::parse("http://matrix.example.test").is_err());
        assert!(MatrixHomeserverUrl::parse("http://127.0.0.1:8008").is_ok());
        assert!(MatrixHomeserverUrl::parse("https://user:secret@example.test").is_err());
        assert!(MatrixRoomId::parse("room-without-sigil:example.test").is_err());
        assert!(MatrixRoomId::parse("!:example.test").is_err());
        assert!(MatrixUserId::parse("@owner:").is_err());
        assert!(MatrixUserId::parse("@contains whitespace:example.test").is_err());
        assert!(MatrixEventId::parse("$modern-serverless-event-id").is_ok());
        assert!(MatrixEventId::parse("$").is_err());
        assert!(MatrixTransactionId::parse("hepta-v1-0123456789abcdef").is_ok());
    }

    #[test]
    fn control_wire_binds_agent_revision_generation_and_exact_turn() {
        let request = MatrixdRequest {
            schema_version: MATRIXD_CONTROL_SCHEMA_VERSION,
            request_id: 9,
            agent_id: agent_id(),
            expected_binding_revision: 4,
            expected_agent_generation: 11,
            method: MatrixdMethod::CancelTurn {
                thread_id: "thr-1".to_string(),
                turn_id: "turn-2".to_string(),
            },
        };
        let bytes = serde_json::to_vec(&request).expect("serialize request");
        assert!(bytes.len() as u64 <= MAX_MATRIXD_CONTROL_FRAME_BYTES);
        assert_eq!(
            serde_json::from_slice::<MatrixdRequest>(&bytes).expect("parse request"),
            request
        );
    }

    #[test]
    fn matrix_binding_rejects_duplicates_and_empty_authority_sets() {
        let room = MatrixRoomId::parse("!room-a:example.test").expect("room");
        let mut binding = MatrixBindingV1 {
            schema_version: MATRIX_BINDING_SCHEMA_VERSION,
            agent_id: agent_id(),
            revision: 1,
            homeserver: MatrixHomeserverUrl::parse("https://matrix.example.test")
                .expect("homeserver"),
            expected_mxid: MatrixUserId::parse("@hepta-a:example.test").expect("mxid"),
            expected_device_id: MatrixDeviceId::parse("DEVICE").expect("device"),
            allowed_rooms: vec![room.clone(), room],
            allowed_senders: vec![MatrixUserId::parse("@owner:example.test").expect("sender")],
            require_explicit_mention: false,
        };
        assert!(binding.validate().is_err());
        binding.allowed_rooms.clear();
        assert!(binding.validate().is_err());
    }
}
