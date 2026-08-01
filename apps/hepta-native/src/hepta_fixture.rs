//! Local Hepta fixture mode for Matrix-heart UI development.
//!
//! Robrix's Matrix heart is intentionally kept intact. This fixture layer gives
//! Hepta a safe local UI boot path before its native OpenClaw-parity runtime is live:
//! the app can show the desktop/mobile shell without requiring a homeserver
//! login, while tests still exercise the custom `m.hepta.*` event contract.

use crate::hepta_event::{
    event_type_for_event_kind, HeptaEventEnvelope, HeptaEventStatus, EVENT_AGENT_RUN,
    EVENT_APPROVAL_REQUEST, EVENT_MEMORY_CITATION, EVENT_RUNTIME_EVENT, EVENT_TASK,
    EVENT_TOOL_CALL, EVENT_TOOL_RESULT,
};
use crate::hepta_runtime_bridge::current_codex_runtime_bridge_event_input;
use crate::{
    home::rooms_list::{enqueue_rooms_list_update, JoinedRoomInfo, RoomsListUpdate},
    room::FetchedRoomAvatar,
    utils::RoomNameId,
};

use matrix_sdk::{
    ruma::{events::tag::Tags, MilliSecondsSinceUnixEpoch, OwnedRoomId},
    RoomDisplayName,
};
use serde_json::{json, Value};
use std::convert::TryFrom;

pub const HEPTA_NATIVE_FIXTURE_MODE_ENV: &str = "HEPTA_NATIVE_FIXTURE_MODE";
pub const HEPTA_FIXTURE_ROOM_ID: &str = "!hepta-runtime-fixture:local";

#[derive(Debug, Clone, PartialEq)]
pub struct HeptaFixtureConversation {
    pub room_id: &'static str,
    pub display_name: &'static str,
    pub topic: &'static str,
    pub events: Vec<HeptaEventEnvelope>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeptaFixtureMatrixEvent {
    pub room_id: &'static str,
    pub event_id: String,
    pub sender: &'static str,
    pub origin_server_ts_ms: u64,
    pub event_type: &'static str,
    pub content: Value,
}

impl HeptaFixtureMatrixEvent {
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

pub fn fixture_mode_enabled_from_value(value: Option<&str>) -> bool {
    value
        .map(str::trim)
        .map(str::to_ascii_lowercase)
        .is_some_and(|value| matches!(value.as_str(), "1" | "true" | "yes" | "on" | "fixture"))
}

pub fn is_fixture_mode_enabled() -> bool {
    fixture_mode_enabled_from_value(std::env::var(HEPTA_NATIVE_FIXTURE_MODE_ENV).ok().as_deref())
}

pub fn sample_conversation() -> HeptaFixtureConversation {
    let current_codex_runtime_bridge_event = current_codex_runtime_bridge_event_input()
        .and_then(|input| input.into_envelope())
        .expect("current codex-rs Hepta runtime bridge fixture must be valid");

    HeptaFixtureConversation {
        room_id: HEPTA_FIXTURE_ROOM_ID,
        display_name: "Hepta Runtime Cockpit",
        topic: "Local Matrix-heart fixture for task/tool/approval UI development",
        events: vec![
            HeptaEventEnvelope::new(
                EVENT_RUNTIME_EVENT,
                "evt-runtime-started",
                HeptaEventStatus::Started,
                "Runtime event received from Hepta native runtime",
            ),
            HeptaEventEnvelope::new(
                EVENT_TOOL_CALL,
                "evt-tool-call-exec",
                HeptaEventStatus::Running,
                "Tool call: exec cargo check for hepta-native",
            ),
            HeptaEventEnvelope::new(
                EVENT_TOOL_RESULT,
                "evt-tool-result-exec",
                HeptaEventStatus::Completed,
                "Tool result: hepta-native cargo check passed",
            ),
            HeptaEventEnvelope::new(
                EVENT_APPROVAL_REQUEST,
                "evt-approval-request",
                HeptaEventStatus::Waiting,
                "Approval request: elevated package install preview",
            ),
            HeptaEventEnvelope::new(
                EVENT_TASK,
                "evt-task-progress",
                HeptaEventStatus::Running,
                "Task update: Matrix-heart desktop/mobile UI milestone in progress",
            ),
            HeptaEventEnvelope::new(
                EVENT_AGENT_RUN,
                "evt-agent-run",
                HeptaEventStatus::Completed,
                "Agent run completed with attached verification evidence",
            ),
            {
                let mut envelope = HeptaEventEnvelope::new(
                    EVENT_TASK,
                    "evt-composer-draft",
                    HeptaEventStatus::Waiting,
                    "Composer dry-run preview staged locally",
                );
                envelope.payload = json!({
                    "title": "Draft task action",
                    "body": "A /task command is rendered as a local m.hepta.task preview before mutation gates are enabled.",
                    "mutation_class": "draft_task_plan",
                    "requires_confirmation": true,
                    "external_mutation_enabled": false,
                });
                envelope
            },
            HeptaEventEnvelope::new(
                EVENT_MEMORY_CITATION,
                "evt-memory-citation",
                HeptaEventStatus::Completed,
                "Memory citation attached for prior Hepta UI decision",
            ),
            current_codex_runtime_bridge_event,
        ],
    }
}

pub fn is_fixture_room_id(room_id: &str) -> bool {
    room_id == HEPTA_FIXTURE_ROOM_ID
}

pub fn sample_matrix_timeline_events() -> Vec<HeptaFixtureMatrixEvent> {
    let conversation = sample_conversation();
    conversation
        .events
        .into_iter()
        .enumerate()
        .map(|(index, envelope)| {
            let event_type = event_type_for_event_kind(&envelope.event_kind)
                .expect("fixture only contains known Hepta event kinds");
            let event_id = format!("${}:hepta.local", envelope.id);
            let content =
                serde_json::to_value(&envelope).expect("Hepta fixture envelopes are serializable");
            HeptaFixtureMatrixEvent {
                room_id: conversation.room_id,
                event_id,
                sender: "@hepta-runtime:local",
                origin_server_ts_ms: 1_764_000_000_000 + index as u64,
                event_type,
                content,
            }
        })
        .collect()
}

pub fn fixture_joined_room_info() -> JoinedRoomInfo {
    let conversation = sample_conversation();
    let room_id = OwnedRoomId::try_from(conversation.room_id)
        .expect("Hepta fixture room id must be a valid Matrix room id");
    let latest = conversation.events.last().map(|event| {
        (
            MilliSecondsSinceUnixEpoch::now(),
            format!("{} · {}", event.status.label(), event.fallback_body),
        )
    });
    JoinedRoomInfo {
        room_name_id: RoomNameId::new(
            RoomDisplayName::Named(conversation.display_name.into()),
            room_id,
        ),
        num_unread_messages: conversation.events.len() as u64,
        num_unread_mentions: 1,
        is_marked_unread: true,
        canonical_alias: None,
        alt_aliases: Vec::new(),
        tags: Tags::default(),
        latest,
        room_avatar: FetchedRoomAvatar::Text("H".to_string()),
        has_been_shown: true,
        has_been_paginated: true,
        is_selected: false,
        is_direct: false,
        is_tombstoned: false,
    }
}

pub fn enqueue_fixture_workspace_room() {
    let room = fixture_joined_room_info();
    enqueue_rooms_list_update(RoomsListUpdate::ClearRooms);
    enqueue_rooms_list_update(RoomsListUpdate::AddJoinedRoom(room));
    enqueue_rooms_list_update(RoomsListUpdate::LoadedRooms { max_rooms: Some(1) });
    enqueue_rooms_list_update(RoomsListUpdate::Status {
        status: "Local Hepta fixture workspace loaded without Matrix homeserver login.".to_string(),
    });
}

pub fn fixture_summary() -> String {
    let conversation = sample_conversation();
    format!(
        "{} · {} Hepta events · {}",
        conversation.display_name,
        conversation.events.len(),
        conversation.topic
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hepta_event::{is_hepta_event_type, HeptaEventEnvelope};

    #[test]
    fn fixture_mode_env_parser_is_explicit() {
        assert!(fixture_mode_enabled_from_value(Some("1")));
        assert!(fixture_mode_enabled_from_value(Some("true")));
        assert!(fixture_mode_enabled_from_value(Some("fixture")));
        assert!(fixture_mode_enabled_from_value(Some(" ON ")));
        assert!(!fixture_mode_enabled_from_value(Some("0")));
        assert!(!fixture_mode_enabled_from_value(Some("false")));
        assert!(!fixture_mode_enabled_from_value(None));
    }

    #[test]
    fn sample_conversation_only_uses_known_hepta_events() {
        let fixture = sample_conversation();
        assert_eq!(fixture.room_id, "!hepta-runtime-fixture:local");
        assert!(fixture.events.len() >= 9);
        for event in fixture.events {
            assert!(is_hepta_event_type(match event.event_kind.as_str() {
                "runtime_event" => EVENT_RUNTIME_EVENT,
                "tool_call" => EVENT_TOOL_CALL,
                "tool_result" => EVENT_TOOL_RESULT,
                "approval_request" => EVENT_APPROVAL_REQUEST,
                "task" => EVENT_TASK,
                "agent_run" => EVENT_AGENT_RUN,
                "memory_citation" => EVENT_MEMORY_CITATION,
                other => panic!("unknown fixture event kind: {other}"),
            }));
            assert!(event.redaction.secrets_redacted);
        }
    }

    #[test]
    fn sample_conversation_includes_current_codex_runtime_bridge_event() {
        let fixture = sample_conversation();
        let event = fixture
            .events
            .iter()
            .find(|event| event.id == "current-codex-runtime-bridge")
            .expect("fixture should include current codex runtime bridge event");

        assert_eq!(event.event_kind, "runtime_event");
        assert_eq!(
            event.conversation_id.as_deref(),
            Some("hepta-native-current-codex")
        );
        assert_eq!(
            event
                .payload
                .pointer("/bridge/source")
                .and_then(Value::as_str),
            Some("codex-rs/hepta-*"),
        );
        assert_eq!(
            event
                .payload
                .pointer("/bridge/gateway_called_by_bridge")
                .and_then(Value::as_bool),
            Some(false),
        );
        assert_eq!(
            event
                .payload
                .pointer("/bridge/channel_delivery_performed_by_bridge")
                .and_then(Value::as_bool),
            Some(false),
        );
    }

    #[test]
    fn fixture_summary_is_operator_readable() {
        let summary = fixture_summary();
        assert!(summary.contains("Hepta Runtime Cockpit"));
        assert!(summary.contains("Hepta events"));
    }

    #[test]
    fn fixture_room_info_is_selectable_workspace_shaped() {
        let room = fixture_joined_room_info();
        assert_eq!(
            room.room_name_id.room_id().as_str(),
            "!hepta-runtime-fixture:local"
        );
        assert_eq!(room.room_name_id.to_string(), "Hepta Runtime Cockpit");
        assert!(
            room.latest
                .as_ref()
                .is_some_and(|(_, latest)| { latest.contains("codex-rs/hepta-*") })
        );
        assert!(room.has_been_paginated);
        assert!(room.is_marked_unread);
        assert!(!room.is_direct);
    }

    #[test]
    fn matrix_timeline_fixture_is_custom_event_shaped() {
        let events = sample_matrix_timeline_events();
        assert!(events.len() >= 8);
        for event in events {
            assert_eq!(event.room_id, "!hepta-runtime-fixture:local");
            assert_eq!(event.sender, "@hepta-runtime:local");
            assert!(is_hepta_event_type(event.event_type));

            let raw = event.as_sync_timeline_json();
            assert_eq!(
                raw.get("type").and_then(Value::as_str),
                Some(event.event_type)
            );
            let content = raw.get("content").unwrap();
            let envelope = HeptaEventEnvelope::from_content_value(content).unwrap();
            assert_eq!(
                event_type_for_event_kind(&envelope.event_kind),
                Some(event.event_type)
            );
            assert!(envelope.redaction.secrets_redacted);
        }
    }
}
