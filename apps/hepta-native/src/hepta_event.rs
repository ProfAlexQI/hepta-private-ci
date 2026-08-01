//! Hepta custom Matrix-style event helpers.
//!
//! The fast-path Hepta Native plan keeps Robrix's Matrix timeline heart intact and
//! layers Hepta runtime semantics over custom `m.hepta.*` event types. This module
//! is intentionally small and protocol-shaped: renderers can recognize Hepta
//! events without needing to de-Matrixify the Robrix timeline first.

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const HEPTA_SCHEMA_V1: &str = "hepta.event.v1";

pub const EVENT_RUNTIME_EVENT: &str = "m.hepta.runtime_event";
pub const EVENT_TOOL_CALL: &str = "m.hepta.tool_call";
pub const EVENT_TOOL_RESULT: &str = "m.hepta.tool_result";
pub const EVENT_APPROVAL_REQUEST: &str = "m.hepta.approval_request";
pub const EVENT_APPROVAL_RESULT: &str = "m.hepta.approval_result";
pub const EVENT_TASK: &str = "m.hepta.task";
pub const EVENT_AGENT_RUN: &str = "m.hepta.agent_run";
pub const EVENT_MEMORY_CITATION: &str = "m.hepta.memory_citation";
pub const EVENT_CONTEXT_SNAPSHOT: &str = "m.hepta.context_snapshot";
pub const EVENT_POLICY_NOTICE: &str = "m.hepta.policy_notice";
pub const EVENT_CHANNEL_STATUS: &str = "m.hepta.channel_status";
pub const EVENT_NODE_STATUS: &str = "m.hepta.node_status";

pub const HEPTA_EVENT_TYPES: &[&str] = &[
    EVENT_RUNTIME_EVENT,
    EVENT_TOOL_CALL,
    EVENT_TOOL_RESULT,
    EVENT_APPROVAL_REQUEST,
    EVENT_APPROVAL_RESULT,
    EVENT_TASK,
    EVENT_AGENT_RUN,
    EVENT_MEMORY_CITATION,
    EVENT_CONTEXT_SNAPSHOT,
    EVENT_POLICY_NOTICE,
    EVENT_CHANNEL_STATUS,
    EVENT_NODE_STATUS,
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeptaEventStatus {
    Started,
    Running,
    Completed,
    Failed,
    Blocked,
    Cancelled,
    Waiting,
}

impl HeptaEventStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Started => "started",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Blocked => "blocked",
            Self::Cancelled => "cancelled",
            Self::Waiting => "waiting",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Started => "●",
            Self::Running => "↻",
            Self::Completed => "✓",
            Self::Failed => "!",
            Self::Blocked => "⏸",
            Self::Cancelled => "×",
            Self::Waiting => "…",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaRedactionState {
    pub secrets_redacted: bool,
    #[serde(default)]
    pub raw_secret_fields: Vec<String>,
}

impl Default for HeptaRedactionState {
    fn default() -> Self {
        Self {
            secrets_redacted: true,
            raw_secret_fields: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeptaEventEnvelope {
    pub hepta_schema: String,
    pub event_kind: String,
    pub id: String,
    #[serde(default)]
    pub conversation_id: Option<String>,
    #[serde(default)]
    pub created_at_ms: u64,
    pub status: HeptaEventStatus,
    #[serde(default)]
    pub redaction: HeptaRedactionState,
    pub fallback_body: String,
    #[serde(default)]
    pub payload: serde_json::Value,
}

impl HeptaEventEnvelope {
    pub fn new(
        event_type: &str,
        id: impl Into<String>,
        status: HeptaEventStatus,
        fallback_body: impl Into<String>,
    ) -> Self {
        Self {
            hepta_schema: HEPTA_SCHEMA_V1.to_string(),
            event_kind: event_kind_for_event_type(event_type)
                .unwrap_or("unknown")
                .to_string(),
            id: id.into(),
            conversation_id: None,
            created_at_ms: 0,
            status,
            redaction: HeptaRedactionState::default(),
            fallback_body: fallback_body.into(),
            payload: serde_json::Value::Null,
        }
    }

    pub fn from_content_value(value: &Value) -> Result<Self, String> {
        let envelope: Self = serde_json::from_value(value.clone())
            .map_err(|err| format!("invalid Hepta event envelope: {err}"))?;
        envelope.validate()?;
        Ok(envelope)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.hepta_schema != HEPTA_SCHEMA_V1 {
            return Err(format!("unsupported Hepta schema: {}", self.hepta_schema));
        }
        if self.id.trim().is_empty() {
            return Err("Hepta event id is empty".to_string());
        }
        if self.event_kind.trim().is_empty() {
            return Err("Hepta event kind is empty".to_string());
        }
        Ok(())
    }

    pub fn payload_str(&self, key: &str) -> Option<&str> {
        self.payload
            .get(key)
            .and_then(Value::as_str)
            .filter(|s| !s.trim().is_empty())
    }

    pub fn card_title(&self) -> String {
        self.payload_str("title")
            .or_else(|| self.payload_str("summary"))
            .map(str::to_string)
            .unwrap_or_else(|| label_for_event_kind(&self.event_kind).to_string())
    }

    pub fn card_body(&self) -> String {
        self.payload_str("body")
            .or_else(|| self.payload_str("message"))
            .or_else(|| self.payload_str("description"))
            .map(str::to_string)
            .unwrap_or_else(|| self.fallback_body.clone())
    }

    pub fn card_meta(&self) -> String {
        let redaction = if self.redaction.secrets_redacted {
            "secrets redacted"
        } else {
            "redaction pending"
        };
        match &self.conversation_id {
            Some(conversation_id) => format!("{} · {} · {}", self.id, conversation_id, redaction),
            None => format!("{} · {}", self.id, redaction),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaEventCardText {
    pub eyebrow: String,
    pub status: String,
    pub title: String,
    pub body: String,
    pub meta: String,
}

pub fn card_text_for_event(
    event_type: &str,
    envelope: Option<&HeptaEventEnvelope>,
) -> HeptaEventCardText {
    let eyebrow = preview_label_for_event_type(event_type)
        .unwrap_or("Hepta event")
        .to_string();
    let Some(envelope) = envelope else {
        return HeptaEventCardText {
            eyebrow,
            status: "custom event".to_string(),
            title: event_type.to_string(),
            body: "Matrix custom event received before a typed Hepta envelope was available."
                .to_string(),
            meta: "fallback renderer".to_string(),
        };
    };
    HeptaEventCardText {
        eyebrow,
        status: format!("{} {}", envelope.status.icon(), envelope.status.label()),
        title: envelope.card_title(),
        body: envelope.card_body(),
        meta: envelope.card_meta(),
    }
}

pub fn is_hepta_event_type(event_type: &str) -> bool {
    HEPTA_EVENT_TYPES.contains(&event_type)
}

pub fn event_kind_for_event_type(event_type: &str) -> Option<&'static str> {
    match event_type {
        EVENT_RUNTIME_EVENT => Some("runtime_event"),
        EVENT_TOOL_CALL => Some("tool_call"),
        EVENT_TOOL_RESULT => Some("tool_result"),
        EVENT_APPROVAL_REQUEST => Some("approval_request"),
        EVENT_APPROVAL_RESULT => Some("approval_result"),
        EVENT_TASK => Some("task"),
        EVENT_AGENT_RUN => Some("agent_run"),
        EVENT_MEMORY_CITATION => Some("memory_citation"),
        EVENT_CONTEXT_SNAPSHOT => Some("context_snapshot"),
        EVENT_POLICY_NOTICE => Some("policy_notice"),
        EVENT_CHANNEL_STATUS => Some("channel_status"),
        EVENT_NODE_STATUS => Some("node_status"),
        _ => None,
    }
}

pub fn event_type_for_event_kind(event_kind: &str) -> Option<&'static str> {
    match event_kind {
        "runtime_event" => Some(EVENT_RUNTIME_EVENT),
        "tool_call" => Some(EVENT_TOOL_CALL),
        "tool_result" => Some(EVENT_TOOL_RESULT),
        "approval_request" => Some(EVENT_APPROVAL_REQUEST),
        "approval_result" => Some(EVENT_APPROVAL_RESULT),
        "task" => Some(EVENT_TASK),
        "agent_run" => Some(EVENT_AGENT_RUN),
        "memory_citation" => Some(EVENT_MEMORY_CITATION),
        "context_snapshot" => Some(EVENT_CONTEXT_SNAPSHOT),
        "policy_notice" => Some(EVENT_POLICY_NOTICE),
        "channel_status" => Some(EVENT_CHANNEL_STATUS),
        "node_status" => Some(EVENT_NODE_STATUS),
        _ => None,
    }
}

pub fn preview_label_for_event_type(event_type: &str) -> Option<&'static str> {
    match event_type {
        EVENT_RUNTIME_EVENT => Some("Hepta runtime event"),
        EVENT_TOOL_CALL => Some("Hepta tool call"),
        EVENT_TOOL_RESULT => Some("Hepta tool result"),
        EVENT_APPROVAL_REQUEST => Some("Hepta approval request"),
        EVENT_APPROVAL_RESULT => Some("Hepta approval result"),
        EVENT_TASK => Some("Hepta task update"),
        EVENT_AGENT_RUN => Some("Hepta agent run"),
        EVENT_MEMORY_CITATION => Some("Hepta memory citation"),
        EVENT_CONTEXT_SNAPSHOT => Some("Hepta context snapshot"),
        EVENT_POLICY_NOTICE => Some("Hepta policy notice"),
        EVENT_CHANNEL_STATUS => Some("Hepta channel status"),
        EVENT_NODE_STATUS => Some("Hepta node status"),
        _ => None,
    }
}

pub fn label_for_event_kind(event_kind: &str) -> &'static str {
    match event_kind {
        "runtime_event" => "Runtime event",
        "tool_call" => "Tool call",
        "tool_result" => "Tool result",
        "approval_request" => "Approval request",
        "approval_result" => "Approval result",
        "task" => "Task update",
        "agent_run" => "Agent run",
        "memory_citation" => "Memory citation",
        "context_snapshot" => "Context snapshot",
        "policy_notice" => "Policy notice",
        "channel_status" => "Channel status",
        "node_status" => "Node status",
        _ => "Hepta event",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_all_hepta_event_types() {
        for event_type in HEPTA_EVENT_TYPES {
            assert!(is_hepta_event_type(event_type));
            let event_kind = event_kind_for_event_type(event_type).unwrap();
            assert_eq!(event_type_for_event_kind(event_kind), Some(*event_type));
            assert!(preview_label_for_event_type(event_type).is_some());
        }
        assert!(!is_hepta_event_type("m.room.message"));
        assert!(event_kind_for_event_type("m.room.message").is_none());
        assert!(event_type_for_event_kind("room_message").is_none());
    }

    #[test]
    fn envelope_defaults_to_redacted_schema_v1() {
        let envelope = HeptaEventEnvelope::new(
            EVENT_TOOL_CALL,
            "tool-call-1",
            HeptaEventStatus::Started,
            "exec requested",
        );

        assert_eq!(envelope.hepta_schema, HEPTA_SCHEMA_V1);
        assert_eq!(envelope.event_kind, "tool_call");
        assert_eq!(envelope.id, "tool-call-1");
        assert_eq!(envelope.status, HeptaEventStatus::Started);
        assert!(envelope.redaction.secrets_redacted);
        assert!(envelope.redaction.raw_secret_fields.is_empty());
    }

    #[test]
    fn parses_content_value_into_card_text() {
        let content = serde_json::json!({
            "hepta_schema": HEPTA_SCHEMA_V1,
            "event_kind": "tool_result",
            "id": "tool-result-1",
            "conversation_id": "conv-1",
            "status": "completed",
            "redaction": { "secrets_redacted": true, "raw_secret_fields": [] },
            "fallback_body": "cargo check passed",
            "payload": { "title": "Native check", "body": "hepta-native compiled" }
        });

        let envelope = HeptaEventEnvelope::from_content_value(&content).unwrap();
        let card = card_text_for_event(EVENT_TOOL_RESULT, Some(&envelope));

        assert_eq!(card.eyebrow, "Hepta tool result");
        assert_eq!(card.status, "✓ completed");
        assert_eq!(card.title, "Native check");
        assert_eq!(card.body, "hepta-native compiled");
        assert!(card.meta.contains("tool-result-1"));
        assert!(card.meta.contains("conv-1"));
        assert!(card.meta.contains("secrets redacted"));
    }

    #[test]
    fn rejects_wrong_schema() {
        let content = serde_json::json!({
            "hepta_schema": "hepta.event.v0",
            "event_kind": "runtime_event",
            "id": "evt-1",
            "status": "running",
            "fallback_body": "old event"
        });
        assert!(HeptaEventEnvelope::from_content_value(&content).is_err());
    }
}
