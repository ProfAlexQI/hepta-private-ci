use serde_json::Value;

use super::{
    adapter::ValidatedBridgeUpdate,
    contract::{BridgeUpdateKind, HEPTA_BRIDGE_SCHEMA_VERSION},
};

pub const DEFAULT_PRESENTATION_PAYLOAD_CAP_BYTES: usize = 32 * 1024;
pub const MAX_PRESENTATION_PAYLOAD_CAP_BYTES: usize = 64 * 1024;
const MAX_PRESENTED_TEXT_CHARS: usize = 240;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresentationFallback {
    PayloadTooLarge,
    UnsupportedSchema,
    InvalidPayload,
    UnredactedPayload,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresentationDisposition {
    Ready,
    Fallback(PresentationFallback),
}

/// A bounded, display-safe projection. It intentionally contains no raw JSON.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PresentedBridgeUpdate {
    pub disposition: PresentationDisposition,
    pub title: String,
    pub summary: String,
}

impl PresentedBridgeUpdate {
    fn fallback(reason: PresentationFallback) -> Self {
        let summary = match reason {
            PresentationFallback::PayloadTooLarge => {
                "The activity payload exceeded this client's display limit."
            }
            PresentationFallback::UnsupportedSchema => {
                "This activity was produced by an unsupported bridge version."
            }
            PresentationFallback::InvalidPayload => "This activity could not be displayed safely.",
            PresentationFallback::UnredactedPayload => {
                "This activity is hidden because it was not marked as redacted."
            }
        };

        Self {
            disposition: PresentationDisposition::Fallback(reason),
            title: "Activity unavailable".into(),
            summary: summary.into(),
        }
    }
}

/// Parses and projects bridge updates without ever echoing rejected payload content.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BridgePresenter {
    payload_cap_bytes: usize,
}

impl Default for BridgePresenter {
    fn default() -> Self {
        Self::new(DEFAULT_PRESENTATION_PAYLOAD_CAP_BYTES)
    }
}

impl BridgePresenter {
    pub fn new(payload_cap_bytes: usize) -> Self {
        Self {
            payload_cap_bytes: payload_cap_bytes.clamp(1, MAX_PRESENTATION_PAYLOAD_CAP_BYTES),
        }
    }

    pub fn payload_cap_bytes(&self) -> usize {
        self.payload_cap_bytes
    }

    /// Untrusted JSON is inspected only far enough to choose a safe fallback.
    /// It can never self-assert that it is redacted and become displayable.
    pub fn present_json(&self, encoded: &[u8]) -> PresentedBridgeUpdate {
        if encoded.len() > self.payload_cap_bytes {
            return PresentedBridgeUpdate::fallback(PresentationFallback::PayloadTooLarge);
        }

        let Ok(value) = serde_json::from_slice::<Value>(encoded) else {
            return PresentedBridgeUpdate::fallback(PresentationFallback::InvalidPayload);
        };

        let schema_version = value
            .get("metadata")
            .and_then(|metadata| metadata.get("schema_version"))
            .and_then(Value::as_u64);
        if schema_version != Some(u64::from(HEPTA_BRIDGE_SCHEMA_VERSION)) {
            return PresentedBridgeUpdate::fallback(PresentationFallback::UnsupportedSchema);
        }

        PresentedBridgeUpdate::fallback(PresentationFallback::InvalidPayload)
    }

    pub(crate) fn present(&self, update: &ValidatedBridgeUpdate) -> PresentedBridgeUpdate {
        let Ok(encoded) = serde_json::to_vec(update.as_update()) else {
            return PresentedBridgeUpdate::fallback(PresentationFallback::InvalidPayload);
        };
        self.present_checked(update.clone(), encoded.len())
    }

    fn present_checked(
        &self,
        update: ValidatedBridgeUpdate,
        encoded_len: usize,
    ) -> PresentedBridgeUpdate {
        if encoded_len > self.payload_cap_bytes {
            return PresentedBridgeUpdate::fallback(PresentationFallback::PayloadTooLarge);
        }
        let update = update.into_update();
        if update.metadata.schema_version != HEPTA_BRIDGE_SCHEMA_VERSION {
            return PresentedBridgeUpdate::fallback(PresentationFallback::UnsupportedSchema);
        }
        if !update.is_contract_valid() {
            return PresentedBridgeUpdate::fallback(PresentationFallback::InvalidPayload);
        }
        if !update.is_presenter_safe() {
            return PresentedBridgeUpdate::fallback(PresentationFallback::UnredactedPayload);
        }

        let (title, summary) = match update.update {
            BridgeUpdateKind::Snapshot { snapshot } => (
                "Hepta activity".into(),
                format!(
                    "{} tasks, {} tools, {} approvals",
                    snapshot.tasks.len(),
                    snapshot.tool_invocations.len(),
                    snapshot.approvals.len()
                ),
            ),
            BridgeUpdateKind::RuntimeChanged { runtime } => (runtime.title, runtime.summary),
            BridgeUpdateKind::TaskUpsert { task } => (task.title, task.summary),
            BridgeUpdateKind::ToolInvocationUpsert { invocation } => {
                (invocation.title, invocation.summary)
            }
            BridgeUpdateKind::ApprovalUpsert { approval } => (approval.title, approval.summary),
            BridgeUpdateKind::ActivityUpsert { activity } => (activity.title, activity.summary),
            BridgeUpdateKind::Receipt { .. } => {
                ("Action update".into(), "The action state changed.".into())
            }
            BridgeUpdateKind::Error { problem } => {
                ("Action unavailable".into(), problem.user_safe_message)
            }
        };

        PresentedBridgeUpdate {
            disposition: PresentationDisposition::Ready,
            title: sanitize_and_truncate(&title),
            summary: sanitize_and_truncate(&summary),
        }
    }
}

fn sanitize_and_truncate(value: &str) -> String {
    let mut sanitized = String::with_capacity(value.len().min(MAX_PRESENTED_TEXT_CHARS));
    let mut pending_space = false;
    for character in value.chars() {
        if is_bidi_control(character) || (character.is_control() && !character.is_whitespace()) {
            continue;
        }
        if character.is_whitespace() {
            pending_space = !sanitized.is_empty();
            continue;
        }
        if pending_space {
            sanitized.push(' ');
            pending_space = false;
        }
        sanitized.push(character);
    }

    let mut chars = sanitized.chars();
    let prefix: String = chars.by_ref().take(MAX_PRESENTED_TEXT_CHARS).collect();
    if chars.next().is_some() {
        format!("{prefix}…")
    } else {
        prefix
    }
}

fn is_bidi_control(character: char) -> bool {
    matches!(
        character,
        '\u{061C}'
            | '\u{200E}'
            | '\u{200F}'
            | '\u{202A}'..='\u{202E}'
            | '\u{2066}'..='\u{2069}'
            | '\u{206A}'..='\u{206F}'
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hepta_bridge::{
        adapter::ValidatedBridgeUpdate,
        contract::{BridgeUpdateKind, Redaction, tests_support::update},
    };

    fn validated(redaction: Redaction) -> ValidatedBridgeUpdate {
        ValidatedBridgeUpdate::for_test(update(redaction))
    }

    #[test]
    fn unknown_schema_degrades_without_echoing_payload() {
        let encoded = br#"{
            "metadata": { "schema_version": 999 },
            "secret": "never echo this"
        }"#;

        let presented = BridgePresenter::default().present_json(encoded);

        assert_eq!(
            presented.disposition,
            PresentationDisposition::Fallback(PresentationFallback::UnsupportedSchema)
        );
        assert!(!presented.summary.contains("never echo this"));
    }

    #[test]
    fn oversized_payload_degrades_before_parsing() {
        let presenter = BridgePresenter::new(64);
        let encoded = vec![b'x'; 65];

        assert_eq!(
            presenter.present_json(&encoded).disposition,
            PresentationDisposition::Fallback(PresentationFallback::PayloadTooLarge)
        );
    }

    #[test]
    fn unredacted_nested_record_is_never_presented() {
        let update = validated(Redaction::unredacted());
        let presented = BridgePresenter::default().present(&update);

        assert_eq!(
            presented.disposition,
            PresentationDisposition::Fallback(PresentationFallback::UnredactedPayload)
        );
        assert!(!presented.summary.contains("safe summary"));
    }

    #[test]
    fn redacted_record_is_presented_with_bounded_text() {
        let mut update = validated(Redaction::redacted("test-policy"));
        let BridgeUpdateKind::TaskUpsert { task } = &mut update.as_update_mut().update else {
            unreachable!();
        };
        task.summary = "a".repeat(MAX_PRESENTED_TEXT_CHARS + 10);

        let presented = BridgePresenter::default().present(&update);

        assert_eq!(presented.disposition, PresentationDisposition::Ready);
        assert_eq!(
            presented.summary.chars().count(),
            MAX_PRESENTED_TEXT_CHARS + 1
        );
        assert!(presented.summary.ends_with('…'));
    }

    #[test]
    fn self_asserted_redacted_json_is_never_rendered() {
        let encoded = serde_json::to_vec(&update(Redaction::redacted("self-asserted"))).unwrap();
        let presented = BridgePresenter::default().present_json(&encoded);

        assert_eq!(
            presented.disposition,
            PresentationDisposition::Fallback(PresentationFallback::InvalidPayload)
        );
        assert!(!presented.title.contains("Task"));
        assert!(!presented.summary.contains("safe summary"));
    }

    #[test]
    fn display_text_removes_line_and_direction_spoofing_controls() {
        let mut update = validated(Redaction::redacted("test-policy"));
        let BridgeUpdateKind::TaskUpsert { task } = &mut update.as_update_mut().update else {
            unreachable!();
        };
        task.title = "Approval\nAccepted\0\u{202E}denied".into();
        task.summary = "line one\r\nline two\u{2066}hidden\u{2069}".into();

        let presented = BridgePresenter::default().present(&update);

        assert_eq!(presented.title, "Approval Accepteddenied");
        assert_eq!(presented.summary, "line one line twohidden");
    }
}
