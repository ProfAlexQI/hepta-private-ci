//! Bounded local smoke report for Hepta Native fixture mode.
//!
//! This stays inside the local Matrix-shaped fixture path. It checks that the
//! desktop/mobile client can render a current `codex-rs/hepta-*` runtime event
//! without entering any live Matrix, Gateway, Telegram, provider, or process path.

use crate::hepta_event::{event_type_for_event_kind, HeptaEventEnvelope, EVENT_RUNTIME_EVENT};
use crate::hepta_fixture::{sample_conversation, sample_matrix_timeline_events};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaFixtureSmokeReport {
    pub status: &'static str,
    pub event_count: usize,
    pub timeline_event_count: usize,
    pub contains_current_codex_runtime_bridge: bool,
    pub current_codex_runtime_bridge_event_type: Option<&'static str>,
    pub bridge_source: Option<String>,
    pub gateway_called_by_bridge: bool,
    pub provider_invoked_by_bridge: bool,
    pub channel_delivery_performed_by_bridge: bool,
    pub process_spawned_by_bridge: bool,
    pub all_events_have_known_matrix_types: bool,
    pub all_events_are_redacted: bool,
}

impl HeptaFixtureSmokeReport {
    pub fn ready(&self) -> bool {
        self.status == "ready"
    }
}

pub fn sample_current_codex_fixture_smoke_report() -> HeptaFixtureSmokeReport {
    let conversation = sample_conversation();
    let timeline_events = sample_matrix_timeline_events();
    let runtime_bridge_event = conversation
        .events
        .iter()
        .find(|event| event.id == "current-codex-runtime-bridge");
    let all_events_have_known_matrix_types = conversation
        .events
        .iter()
        .all(|event| event_type_for_event_kind(&event.event_kind).is_some());
    let all_events_are_redacted = conversation
        .events
        .iter()
        .all(|event| event.redaction.secrets_redacted);

    let bridge_source = runtime_bridge_event
        .and_then(|event| event.payload.pointer("/bridge/source"))
        .and_then(Value::as_str)
        .map(str::to_string);
    let gateway_called_by_bridge =
        bridge_flag(runtime_bridge_event, "/bridge/gateway_called_by_bridge");
    let provider_invoked_by_bridge =
        bridge_flag(runtime_bridge_event, "/bridge/provider_invoked_by_bridge");
    let channel_delivery_performed_by_bridge = bridge_flag(
        runtime_bridge_event,
        "/bridge/channel_delivery_performed_by_bridge",
    );
    let process_spawned_by_bridge =
        bridge_flag(runtime_bridge_event, "/bridge/process_spawned_by_bridge");
    let current_codex_runtime_bridge_event_type =
        runtime_bridge_event.and_then(matrix_event_type_for_envelope);
    let contains_current_codex_runtime_bridge = runtime_bridge_event.is_some();
    let side_effect_performed = gateway_called_by_bridge
        || provider_invoked_by_bridge
        || channel_delivery_performed_by_bridge
        || process_spawned_by_bridge;
    let status = if contains_current_codex_runtime_bridge
        && current_codex_runtime_bridge_event_type == Some(EVENT_RUNTIME_EVENT)
        && bridge_source.as_deref() == Some("codex-rs/hepta-*")
        && !side_effect_performed
        && all_events_have_known_matrix_types
        && all_events_are_redacted
        && conversation.events.len() == timeline_events.len()
    {
        "ready"
    } else {
        "attention"
    };

    HeptaFixtureSmokeReport {
        status,
        event_count: conversation.events.len(),
        timeline_event_count: timeline_events.len(),
        contains_current_codex_runtime_bridge,
        current_codex_runtime_bridge_event_type,
        bridge_source,
        gateway_called_by_bridge,
        provider_invoked_by_bridge,
        channel_delivery_performed_by_bridge,
        process_spawned_by_bridge,
        all_events_have_known_matrix_types,
        all_events_are_redacted,
    }
}

fn bridge_flag(event: Option<&HeptaEventEnvelope>, pointer: &str) -> bool {
    event
        .and_then(|event| event.payload.pointer(pointer))
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn matrix_event_type_for_envelope(envelope: &HeptaEventEnvelope) -> Option<&'static str> {
    event_type_for_event_kind(&envelope.event_kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_codex_fixture_smoke_is_ready_without_live_side_effects() {
        let report = sample_current_codex_fixture_smoke_report();

        assert!(report.ready(), "{report:?}");
        assert!(report.event_count >= 9);
        assert_eq!(report.event_count, report.timeline_event_count);
        assert!(report.contains_current_codex_runtime_bridge);
        assert_eq!(
            report.current_codex_runtime_bridge_event_type,
            Some(EVENT_RUNTIME_EVENT)
        );
        assert_eq!(report.bridge_source.as_deref(), Some("codex-rs/hepta-*"));
        assert!(!report.gateway_called_by_bridge);
        assert!(!report.provider_invoked_by_bridge);
        assert!(!report.channel_delivery_performed_by_bridge);
        assert!(!report.process_spawned_by_bridge);
        assert!(report.all_events_have_known_matrix_types);
        assert!(report.all_events_are_redacted);
    }
}
