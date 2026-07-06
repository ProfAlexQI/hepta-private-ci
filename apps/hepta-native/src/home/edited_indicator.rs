//! An indicator that is shown nearby a message that has been edited.
//!
//! This widget is basically just a clickable label that shows the text "(edited)"
//! with an underline to indicate that it is clickable.
//! Upon hover, it shows a tooltip with the date and time when the message was edited.
//!
//! On click, this widget asks the room screen to fetch a compact Matrix edit
//! history summary. A full history modal/diff UI remains a separate product gap.

use chrono::{DateTime, Local};
use makepad_widgets::*;
use matrix_sdk::ruma::{EventId, OwnedEventId};
use matrix_sdk_ui::timeline::EventTimelineItem;

use crate::{
    event_preview::plaintext_body_of_timeline_item,
    shared::popup_list::{PopupKind, enqueue_popup_notification},
    utils::unix_time_millis_to_datetime,
};

pub const MESSAGE_EDIT_HISTORY_LOCAL_BOUNDARY_EVIDENCE: &str = "EditedIndicator now uses a compact Matrix m.replace relations read when clicked while message_edit_history remains a base gap. Hover still reads only the already loaded latest edit timestamp; click requests MatrixRequest::FetchEditHistory to count replacement events and preview the newest replacement, and RoomScreen compares it against the already loaded original plaintext preview as a local diff hint. RoomScreen can also open the existing EventSourceModal for the already loaded original event source when latest_json is still in the visible timeline cache. Full history modal UI, full diff rendering, event-context fetch, timeline pagination/reload, replacement event source fetch, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, and live mutation remain unwired.";
pub const MESSAGE_EDIT_HISTORY_LOCAL_BOUNDARY_LABEL: &str = "Edit history reads m.replace summary with loaded original preview; loaded original source can open locally; full modal, full diff, event context, reload, and mutation stay local.";
pub const MESSAGE_EDIT_HISTORY_LOADED_TARGET_METADATA_EVIDENCE: &str = "EditedIndicator caches loaded edit-history target metadata from the already loaded timeline row when set_latest_edit runs: loaded event-id availability, loaded original plaintext preview, character count, byte count, and latest edit timestamp availability. Hover and the click-start popup display this cached metadata before the existing MatrixRequest::FetchEditHistory read. They send no event-context fetch, timeline pagination/reload, event source open, full history modal request, full diff rendering, message send/edit/redact, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MESSAGE_EDIT_HISTORY_LOADED_TARGET_METADATA_LABEL: &str =
    "Loaded edit target metadata only; no event context, reload, or mutation.";
pub const MESSAGE_EDIT_HISTORY_COMPACT_LABEL: &str =
    "Edit history summary uses Matrix m.replace read plus loaded original preview.";

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*


    mod.widgets.EDITED_INDICATOR_FONT_SIZE = 9.5
    mod.widgets.EDITED_INDICATOR_FONT_COLOR = #666666

    mod.widgets.EditedIndicator = #(EditedIndicator::register_widget(vm)) {
        visible: false, // default to hidden
        width: Fit, height: Fit
        flow: Right,
        padding: 0,
        margin: Inset{ top: 5 }

        cursor: MouseCursor.Hand,

        edit_html := Html {
            width: Fit, height: Fit
            flow: Right, // do not wrap
            padding: 0,
            margin: 0,

            font_size: (mod.widgets.EDITED_INDICATOR_FONT_SIZE),
            font_color: (COLOR_ROBRIX_PURPLE),
            body: "(<u>edited</u>)",
        }
    }
}

/// A interactive label that indicates a message has been edited.
#[derive(Script, ScriptHook, Widget)]
pub struct EditedIndicator {
    #[deref]
    view: View,
    #[rust]
    latest_edit_ts: Option<DateTime<Local>>,
    #[rust]
    event_id: Option<OwnedEventId>,
    #[rust]
    loaded_target_metadata: Option<EditHistoryLoadedTargetMetadata>,
}

impl Widget for EditedIndicator {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let area = self.view.area();
        let should_hover_in = match event.hits(cx, area) {
            Hit::FingerLongPress(_) | Hit::FingerHoverIn(..) => true,
            Hit::FingerUp(fue) if fue.is_over && fue.is_primary_hit() => {
                if let Some(event_id) = self.event_id.clone() {
                    let loaded_target_metadata = self.loaded_target_metadata.clone();
                    cx.widget_action(
                        self.widget_uid(),
                        EditedIndicatorAction::ShowEditHistory {
                            event_id,
                            loaded_target_metadata,
                        },
                    );
                    enqueue_popup_notification(
                        MESSAGE_EDIT_HISTORY_COMPACT_LABEL,
                        PopupKind::Info,
                        Some(3.0),
                    );
                } else {
                    enqueue_popup_notification(
                        "Edit history unavailable: loaded event id is missing.",
                        PopupKind::Warning,
                        Some(3.0),
                    );
                }
                false
            }
            Hit::FingerHoverOut(_) => {
                cx.widget_action(self.widget_uid(), TooltipAction::HoverOut);
                false
            }
            _ => false,
        };
        if should_hover_in {
            // TODO: use pure_rust_locales crate to format the time based on the chosen Locale.
            let locale_extended_fmt_en_us = "%a %b %-d, %Y, %r";
            let mut text = if let Some(ts) = self.latest_edit_ts {
                format!("Last edited {}", ts.format(locale_extended_fmt_en_us))
            } else {
                "Last edit time unknown".to_string()
            };
            if let Some(metadata) = self.loaded_target_metadata.as_ref() {
                text.push_str(". ");
                text.push_str(&loaded_edit_history_target_metadata_tooltip_label(metadata));
            }
            cx.widget_action(
                self.widget_uid(),
                TooltipAction::HoverIn {
                    text,
                    widget_rect: area.rect(cx),
                    options: CalloutTooltipOptions {
                        position: TooltipPosition::Right,
                        ..Default::default()
                    },
                },
            );
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl EditedIndicator {
    /// Sets this indicator to show the timestamp of the latest edit of the given `EventTimelineItem`.
    pub fn set_latest_edit(&mut self, cx: &mut Cx, event_tl_item: &EventTimelineItem) {
        self.event_id = event_tl_item.event_id().map(EventId::to_owned);
        self.latest_edit_ts = None;
        if let Some(aste) = event_tl_item
            .latest_edit_json()
            .and_then(|json| json.deserialize().ok())
        {
            self.latest_edit_ts = unix_time_millis_to_datetime(aste.origin_server_ts());
        }
        self.loaded_target_metadata = Some(EditHistoryLoadedTargetMetadata::from_loaded_body(
            &plaintext_body_of_timeline_item(event_tl_item),
            self.event_id.is_some(),
            self.latest_edit_ts.is_some(),
        ));
        self.visible = true;
        self.redraw(cx);
    }
}

#[derive(Clone, Debug)]
pub struct EditHistoryLoadedTargetMetadata {
    pub body_preview: String,
    pub body_char_count: usize,
    pub body_byte_count: usize,
    pub event_id_loaded: bool,
    pub latest_edit_timestamp_loaded: bool,
}

impl EditHistoryLoadedTargetMetadata {
    pub fn from_loaded_body(
        body: &str,
        event_id_loaded: bool,
        latest_edit_timestamp_loaded: bool,
    ) -> Self {
        Self {
            body_preview: compact_edit_history_target_preview(body),
            body_char_count: body.chars().count(),
            body_byte_count: body.len(),
            event_id_loaded,
            latest_edit_timestamp_loaded,
        }
    }
}

fn compact_edit_history_target_preview(body: &str) -> String {
    let compact = body.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.is_empty() {
        return "empty loaded original".to_string();
    }

    let mut preview = compact.chars().take(72).collect::<String>();
    if compact.chars().count() > 72 {
        preview.push_str("...");
    }
    preview
}

pub(crate) fn loaded_edit_history_target_metadata_label(
    metadata: &EditHistoryLoadedTargetMetadata,
) -> String {
    let event_state = if metadata.event_id_loaded {
        "event id loaded"
    } else {
        "event id missing"
    };
    let timestamp_state = if metadata.latest_edit_timestamp_loaded {
        "latest edit timestamp loaded"
    } else {
        "latest edit timestamp missing"
    };
    format!(
        "Loaded edit target: {}, {}, {} chars, {} bytes. Preview: {}. {}",
        event_state,
        timestamp_state,
        metadata.body_char_count,
        metadata.body_byte_count,
        metadata.body_preview,
        MESSAGE_EDIT_HISTORY_LOADED_TARGET_METADATA_LABEL
    )
}

fn loaded_edit_history_target_metadata_tooltip_label(
    metadata: &EditHistoryLoadedTargetMetadata,
) -> String {
    let event_state = if metadata.event_id_loaded {
        "event id loaded"
    } else {
        "event id missing"
    };
    format!(
        "Loaded target: {event_state}, {} chars, {} bytes.",
        metadata.body_char_count, metadata.body_byte_count
    )
}

#[cfg(test)]
mod edit_history_loaded_target_metadata_tests {
    use super::*;

    #[test]
    fn loaded_edit_history_target_metadata_label_summarizes_loaded_target() {
        let metadata =
            EditHistoryLoadedTargetMetadata::from_loaded_body("Original edit body", true, true);

        let label = loaded_edit_history_target_metadata_label(&metadata);

        assert!(label.contains("Loaded edit target: event id loaded"));
        assert!(label.contains("latest edit timestamp loaded"));
        assert!(label.contains("18 chars"));
        assert!(label.contains("18 bytes"));
        assert!(label.contains("Preview: Original edit body"));
        assert!(label.contains("no event context, reload, or mutation"));
    }

    #[test]
    fn loaded_edit_history_target_metadata_label_handles_missing_loaded_bits() {
        let metadata = EditHistoryLoadedTargetMetadata::from_loaded_body("", false, false);

        let label = loaded_edit_history_target_metadata_label(&metadata);

        assert!(label.contains("event id missing"));
        assert!(label.contains("latest edit timestamp missing"));
        assert!(label.contains("0 chars"));
        assert!(label.contains("0 bytes"));
        assert!(label.contains("Preview: empty loaded original"));
    }
}

impl EditedIndicatorRef {
    /// See [`EditedIndicator::set_latest_edit()`].
    pub fn set_latest_edit(&self, cx: &mut Cx, event_tl_item: &EventTimelineItem) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_latest_edit(cx, event_tl_item);
        }
    }
}

/// Actions emitted by an `EditedIndicator` widget.
#[derive(Clone, Debug, Default)]
pub enum EditedIndicatorAction {
    /// The indicator was clicked, so the UI should fetch a compact Matrix edit history summary.
    ShowEditHistory {
        event_id: OwnedEventId,
        loaded_target_metadata: Option<EditHistoryLoadedTargetMetadata>,
    },
    #[default]
    None,
}
