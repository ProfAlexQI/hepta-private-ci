use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::Sha256Digest;

pub const CHANNEL_EVIDENCE_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct ChannelAdapterId(String);

impl ChannelAdapterId {
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        if value.is_empty()
            || value.len() > 64
            || !value.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'.' | b'_' | b'-')
            })
        {
            return Err(
                "channel adapter ids must contain 1..=64 lowercase ASCII identifier characters"
                    .to_string(),
            );
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
/// Opaque routing evidence binding. This is not an authenticated identity.
pub struct ChannelScope {
    pub adapter_id: ChannelAdapterId,
    pub installation_sha256: Sha256Digest,
    pub account_sha256: Sha256Digest,
    pub conversation_sha256: Sha256Digest,
    pub principal_sha256: Sha256Digest,
}

impl ChannelScope {
    pub fn binding_sha256(&self) -> Sha256Digest {
        digest_parts([
            self.adapter_id.as_str(),
            self.installation_sha256.as_str(),
            self.account_sha256.as_str(),
            self.conversation_sha256.as_str(),
            self.principal_sha256.as_str(),
        ])
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct ChannelIngressEventId(String);

impl ChannelIngressEventId {
    pub fn parse(value: impl Into<String>) -> Result<Self, String> {
        crate::stable_id::parse_prefixed_sha256_id(
            value,
            "channel-ingress:v1:",
            "channel ingress event",
        )
        .map(Self)
    }

    /// Identifies the external source event within one scope.
    ///
    /// Payload, target, cursor, and time remain full-record bindings so that a
    /// durable store can reject a substituted record under the same identity.
    pub fn for_event(scope: &ChannelScope, source_event_sha256: &Sha256Digest) -> Self {
        Self(format!(
            "channel-ingress:v1:{}",
            digest_parts([
                scope.binding_sha256().as_str(),
                source_event_sha256.as_str(),
            ])
            .as_str()
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct ChannelIngressReceiptId(String);

impl ChannelIngressReceiptId {
    pub fn for_event(event_id: &ChannelIngressEventId) -> Self {
        Self(format!(
            "channel-ingress-receipt:v1:{}",
            digest_parts([event_id.as_str()]).as_str()
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ChannelIngressEvent {
    pub schema_version: u32,
    pub event_id: ChannelIngressEventId,
    pub scope: ChannelScope,
    pub source_event_sha256: Sha256Digest,
    pub payload_sha256: Sha256Digest,
    pub target_thread_sha256: Sha256Digest,
    pub predecessor_cursor_sha256: Option<Sha256Digest>,
    pub next_cursor_sha256: Sha256Digest,
    /// Caller-observed evidence time; this is not a host freshness authority.
    pub received_at_unix_ms: u64,
}

impl ChannelIngressEvent {
    pub fn new(
        scope: ChannelScope,
        source_event_sha256: Sha256Digest,
        payload_sha256: Sha256Digest,
        target_thread_sha256: Sha256Digest,
        predecessor_cursor_sha256: Option<Sha256Digest>,
        next_cursor_sha256: Sha256Digest,
        received_at_unix_ms: u64,
    ) -> Result<Self, String> {
        if received_at_unix_ms == 0 {
            return Err("channel ingress receive time must be positive".to_string());
        }
        let event_id = ChannelIngressEventId::for_event(&scope, &source_event_sha256);
        Ok(Self {
            schema_version: CHANNEL_EVIDENCE_SCHEMA_VERSION,
            event_id,
            scope,
            source_event_sha256,
            payload_sha256,
            target_thread_sha256,
            predecessor_cursor_sha256,
            next_cursor_sha256,
            received_at_unix_ms,
        })
    }
}

pub fn channel_target_thread_sha256(thread_id: &str) -> Result<Sha256Digest, String> {
    if thread_id.trim().is_empty() {
        return Err("channel ingress target thread id must not be empty".to_string());
    }
    Ok(digest_parts(["hepta-channel-target-thread-v1", thread_id]))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "terminal", rename_all = "snake_case")]
pub enum ChannelIngressTerminal {
    Accepted { thread_id: String, turn_id: String },
    Rejected { reason_code: String },
    Indeterminate { reason_code: String },
}

impl ChannelIngressTerminal {
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Accepted { .. } => "accepted",
            Self::Rejected { .. } => "rejected",
            Self::Indeterminate { .. } => "indeterminate",
        }
    }

    pub const fn advances_cursor(&self) -> bool {
        matches!(self, Self::Accepted { .. } | Self::Rejected { .. })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ChannelIngressReceipt {
    pub schema_version: u32,
    pub receipt_id: ChannelIngressReceiptId,
    pub event_id: ChannelIngressEventId,
    pub event: ChannelIngressEvent,
    pub terminal: ChannelIngressTerminal,
}

impl ChannelIngressReceipt {
    pub fn new(event: ChannelIngressEvent, terminal: ChannelIngressTerminal) -> Self {
        let event_id = event.event_id.clone();
        let receipt_id = ChannelIngressReceiptId::for_event(&event_id);
        Self {
            schema_version: CHANNEL_EVIDENCE_SCHEMA_VERSION,
            receipt_id,
            event_id,
            event,
            terminal,
        }
    }
}

pub fn validate_ingress_event(event: &ChannelIngressEvent) -> Result<(), String> {
    if event.schema_version != CHANNEL_EVIDENCE_SCHEMA_VERSION {
        return Err("unsupported channel ingress schema version".to_string());
    }
    validate_channel_scope(&event.scope)?;
    ChannelIngressEventId::parse(event.event_id.as_str())?;
    validate_digest(&event.source_event_sha256)?;
    validate_digest(&event.payload_sha256)?;
    validate_digest(&event.target_thread_sha256)?;
    if let Some(predecessor) = event.predecessor_cursor_sha256.as_ref() {
        validate_digest(predecessor)?;
    }
    validate_digest(&event.next_cursor_sha256)?;
    if event.event_id != ChannelIngressEventId::for_event(&event.scope, &event.source_event_sha256)
    {
        return Err(
            "channel ingress event id does not bind its scope and source event".to_string(),
        );
    }
    if event.received_at_unix_ms == 0 {
        return Err("channel ingress receive time must be positive".to_string());
    }
    Ok(())
}

pub fn validate_ingress_receipt(receipt: &ChannelIngressReceipt) -> Result<(), String> {
    if receipt.schema_version != CHANNEL_EVIDENCE_SCHEMA_VERSION {
        return Err("unsupported channel ingress receipt schema version".to_string());
    }
    validate_ingress_event(&receipt.event)?;
    if receipt.event_id != receipt.event.event_id
        || receipt.receipt_id != ChannelIngressReceiptId::for_event(&receipt.event_id)
    {
        return Err("channel ingress receipt does not bind its event".to_string());
    }
    match &receipt.terminal {
        ChannelIngressTerminal::Accepted { thread_id, turn_id } => {
            if thread_id.trim().is_empty() || turn_id.trim().is_empty() {
                return Err("accepted channel ingress requires thread and turn ids".to_string());
            }
            if channel_target_thread_sha256(thread_id)? != receipt.event.target_thread_sha256 {
                return Err(
                    "accepted channel ingress thread does not match the claimed target".to_string(),
                );
            }
        }
        ChannelIngressTerminal::Rejected { reason_code }
        | ChannelIngressTerminal::Indeterminate { reason_code } => {
            validate_reason_code(reason_code)?;
        }
    }
    Ok(())
}

fn validate_channel_scope(scope: &ChannelScope) -> Result<(), String> {
    ChannelAdapterId::new(scope.adapter_id.as_str())?;
    validate_digest(&scope.installation_sha256)?;
    validate_digest(&scope.account_sha256)?;
    validate_digest(&scope.conversation_sha256)?;
    validate_digest(&scope.principal_sha256)?;
    Ok(())
}

fn validate_digest(digest: &Sha256Digest) -> Result<(), String> {
    Sha256Digest::parse(digest.as_str()).map(|_| ())
}
pub(crate) fn validate_reason_code(reason_code: &str) -> Result<(), String> {
    if reason_code.is_empty()
        || reason_code.len() > 128
        || !reason_code.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'.' | b'_' | b'-' | b':')
        })
    {
        return Err(
            "channel reason codes must contain 1..=128 lowercase ASCII identifier characters"
                .to_string(),
        );
    }
    Ok(())
}

pub(crate) fn digest_parts<'a>(parts: impl IntoIterator<Item = &'a str>) -> Sha256Digest {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_be_bytes());
        hasher.update(part.as_bytes());
    }
    Sha256Digest::from_sha256_output(hasher.finalize())
}

#[cfg(test)]
#[path = "channel_tests.rs"]
mod tests;
