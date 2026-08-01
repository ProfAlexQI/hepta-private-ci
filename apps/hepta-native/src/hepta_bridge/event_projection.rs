//! Side-effect-free projection from governed Hepta runtime events into the
//! Matrix timeline shape consumed by the product shell.

use crate::hepta_event::{
    HEPTA_SCHEMA_V1, HeptaEventEnvelope, HeptaEventStatus, HeptaRedactionState,
    event_type_for_event_kind,
};

use serde_json::{Value, json};

#[derive(Debug, Clone, PartialEq)]
pub struct HeptaBridgeEventInput {
    pub event_kind: String,
    pub id: String,
    pub conversation_id: Option<String>,
    pub created_at_ms: u64,
    pub status: HeptaEventStatus,
    pub fallback_body: String,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeptaBridgeMatrixEvent {
    pub room_id: String,
    pub event_id: String,
    pub sender: String,
    pub origin_server_ts_ms: u64,
    pub event_type: &'static str,
    pub content: Value,
}

impl HeptaBridgeEventInput {
    pub fn new(
        event_kind: impl Into<String>,
        id: impl Into<String>,
        status: HeptaEventStatus,
        fallback_body: impl Into<String>,
    ) -> Self {
        Self {
            event_kind: event_kind.into(),
            id: id.into(),
            conversation_id: None,
            created_at_ms: 0,
            status,
            fallback_body: fallback_body.into(),
            payload: Value::Null,
        }
    }

    pub fn into_envelope(self) -> Result<HeptaEventEnvelope, String> {
        if event_type_for_event_kind(&self.event_kind).is_none() {
            return Err(format!("unknown Hepta event kind: {}", self.event_kind));
        }
        let envelope = HeptaEventEnvelope {
            hepta_schema: HEPTA_SCHEMA_V1.to_string(),
            event_kind: self.event_kind,
            id: self.id,
            conversation_id: self.conversation_id,
            created_at_ms: self.created_at_ms,
            status: self.status,
            redaction: HeptaRedactionState::default(),
            fallback_body: self.fallback_body,
            payload: self.payload,
        };
        envelope.validate()?;
        Ok(envelope)
    }
}

impl HeptaBridgeMatrixEvent {
    pub fn from_input(
        room_id: impl Into<String>,
        sender: impl Into<String>,
        input: HeptaBridgeEventInput,
    ) -> Result<Self, String> {
        let envelope = input.into_envelope()?;
        let event_type = event_type_for_event_kind(&envelope.event_kind)
            .expect("envelope validation already checked event kind");
        let event_id = format!("${}:hepta.local", envelope.id);
        let origin_server_ts_ms = envelope.created_at_ms;
        let content = serde_json::to_value(&envelope)
            .map_err(|err| format!("failed to encode Hepta bridge envelope: {err}"))?;
        Ok(Self {
            room_id: room_id.into(),
            event_id,
            sender: sender.into(),
            origin_server_ts_ms,
            event_type,
            content,
        })
    }

    pub fn as_sync_timeline_json(&self) -> Value {
        json!({
            "type": self.event_type,
            "room_id": self.room_id,
            "event_id": self.event_id,
            "sender": self.sender,
            "origin_server_ts": self.origin_server_ts_ms,
            "content": self.content,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hepta_event::{EVENT_APPROVAL_REQUEST, EVENT_TOOL_RESULT, HeptaEventEnvelope};

    #[test]
    fn converts_hepta_runtime_input_to_matrix_custom_event() {
        let mut input = HeptaBridgeEventInput::new(
            "tool_result",
            "tool-result-7",
            HeptaEventStatus::Completed,
            "cargo check passed",
        );
        input.conversation_id = Some("conv-runtime".to_string());
        input.created_at_ms = 1_764_123_456_000;
        input.payload = json!({
            "title": "Native check",
            "body": "hepta-native cargo check passed",
        });

        let event = HeptaBridgeMatrixEvent::from_input(
            "!hepta-runtime-fixture:local",
            "@hepta-runtime:local",
            input,
        )
        .unwrap();

        assert_eq!(event.event_type, EVENT_TOOL_RESULT);
        assert_eq!(event.event_id, "$tool-result-7:hepta.local");
        let raw = event.as_sync_timeline_json();
        assert_eq!(raw.get("type").and_then(Value::as_str), Some(EVENT_TOOL_RESULT));
        let envelope = HeptaEventEnvelope::from_content_value(raw.get("content").unwrap()).unwrap();
        assert_eq!(envelope.event_kind, "tool_result");
        assert_eq!(envelope.card_title(), "Native check");
        assert!(envelope.redaction.secrets_redacted);
    }

    #[test]
    fn rejects_unknown_event_kinds_before_matrix_injection() {
        let input = HeptaBridgeEventInput::new(
            "not_a_hepta_event",
            "bad-event",
            HeptaEventStatus::Failed,
            "bad",
        );
        let err = HeptaBridgeMatrixEvent::from_input(
            "!hepta-runtime-fixture:local",
            "@hepta-runtime:local",
            input,
        )
        .unwrap_err();
        assert!(err.contains("unknown Hepta event kind"));
    }

    #[test]
    fn approval_requests_keep_their_custom_event_type() {
        let input = HeptaBridgeEventInput::new(
            "approval_request",
            "approval-1",
            HeptaEventStatus::Waiting,
            "approval required",
        );
        let event = HeptaBridgeMatrixEvent::from_input(
            "!hepta-runtime-fixture:local",
            "@hepta-runtime:local",
            input,
        )
        .unwrap();
        assert_eq!(event.event_type, EVENT_APPROVAL_REQUEST);
    }
}
