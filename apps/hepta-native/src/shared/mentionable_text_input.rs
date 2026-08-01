//! Wrapper around a `TextInput` that shows an auto-complete popup upon trigger characters.
//!
//! Currently we use it for:
//! 1. Showing members in a room (upon pressing '@')
//! 2. Showing known rooms and spaces (upon pressing '#')
//! 3. Showing slash commands (upon pressing '/')

use std::{collections::BTreeSet, sync::Arc};
use makepad_widgets::{text::selection::Cursor, *};
use makepad_widgets::makepad_platform::event::finger::TouchState;
use matrix_sdk::{
    room::RoomMember,
    ruma::{
        events::{room::message::RoomMessageEventContent, Mentions},
        OwnedRoomId, OwnedUserId,
    },
};
use crate::{
    home::rooms_list::RoomsListRef,
    shared::{mention_popup::{MentionItem, MentionablePopupRef}, slash_commands},
    sliding_sync::{submit_async_request, MatrixRequest},
    utils::{self, MatchQuality},
};

pub const MENTION_PICKER_SEND_LOCAL_BOUNDARY_EVIDENCE: &str = "MentionableTextInput extracts compact @room and @user mentions from already loaded composer text and RoomScreen room_members cache. It exposes a minimal local suggestion row while the active token is an unfinished @mention: @room when power levels allow it and up to three cached-member matches from the loaded room_members cache. It also keeps a live local completed-mention pill tray after insertion: completed @room, literal Matrix user-id, loaded-member, and unmatched local @tokens are summarized as removable local pills, and clicking a pill only removes that completed token from the composer text before the existing send-time Mentions payload scan. The preview shows cached suggestion count, current selected token, loaded member identity preview, avatar MXC presence, no-match state, local tray count, or pill payload state. Clicking a suggestion, ArrowUp/ArrowDown selection plus Tab/Enter insertion, or primary Tab insertion only replaces the active @token and appends a space; after that trailing space the helper stops intercepting Enter so the existing composer send path can submit normally. The send path preserves markdown, /html, and /plain message creation, then add_mentions attaches Matrix Mentions to the existing RoomInputBar MatrixRequest::SendMessage payload for @room, literal Matrix user ids, and cached member display/localpart matches without submitting member lookup, room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests. The attachment review Send path now reuses mentions_for_text from the same loaded member cache and passes AttachmentConfig.mentions alongside the caption in MatrixRequest::SendAttachment. Full popup search, rich popup highlight styling, remote member lookup, rich attachment payload editing, and edit mention payload rewrites remain unwired.";
pub const MENTION_PICKER_SEND_LOCAL_BOUNDARY_LABEL: &str = "@mention picker: cached suggestions and completed-token pills edit composer text locally; Send attaches compact Matrix Mentions.";
pub const MENTION_PICKER_CACHED_SUGGESTION_LABEL: &str = "Cached mention suggestions use ArrowUp/ArrowDown, Tab, Enter, or click to insert local tokens; Send attaches Matrix Mentions.";
pub const MENTION_PICKER_CACHED_SELECTION_EVIDENCE: &str = "MentionableTextInput shows cached suggestion count, selected token, loaded member display name, Matrix user id, localpart, avatar MXC presence, and no-match state for the active @query from already loaded room_members only. This preview starts no remote member lookup, popup search, pill editor, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MENTION_PICKER_LOADED_IDENTITY_EVIDENCE: &str = "MentionableTextInput renders a selected suggestion identity preview from already loaded RoomMember data: display name availability, Matrix user id, localpart, and avatar MXC presence. The @room row reflects the already loaded power-level permission state. This preview does not query remote members, fetch avatars, open a popup search, build a pill editor, mutate membership, send room-state, touch account/profile, gateway/runtime/auth, or start live mutation.";
pub const MENTION_PICKER_LOADED_IDENTITY_LABEL: &str = "Loaded member identity preview: display name, user id, localpart, and avatar MXC status stay local.";
pub const MENTION_PICKER_LOCAL_CANDIDATE_ROWS_EVIDENCE: &str = "MentionableTextInput exposes local candidate rows for the active @query from @room power-level state plus up to three cached RoomMember matches. The row preview records rank, selected state, token, display name availability, Matrix user id, localpart, avatar MXC status, and cache source from already loaded room_members only. It starts no remote member lookup, server-side directory search, profile/avatar fetch, duplicate-name disambiguation, rich popup search, pill editor, attachment/edit mention payload, SendAttachment, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, retry automation, or live mutation request.";
pub const MENTION_PICKER_LOCAL_CANDIDATE_ROWS_LABEL: &str = "Local candidate rows: rank, selection, token, display/user id/localpart, avatar MXC, and cache source only.";
pub const MENTION_PICKER_LOCAL_DUPLICATE_HINTS_EVIDENCE: &str = "MentionableTextInput exposes local duplicate-name hints for the active @query by counting already loaded room_members display-name collisions inside the same cached suggestion pass. The hint reports cached candidate count, duplicate display-name group count, selected token, selected display collision count, and reminds the user that localpart and Matrix user id remain the only disambiguation clues before insertion. It starts no remote member lookup, server-side directory search, profile/avatar fetch, rich duplicate-name disambiguation UI, hover card, pill editor, attachment/edit mention payload, SendAttachment, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, retry automation, or live mutation request.";
pub const MENTION_PICKER_LOCAL_DUPLICATE_HINTS_LABEL: &str =
    "Local duplicate hints: cached display-name collisions use localpart/user-id clues only.";
pub const MENTION_PICKER_LIFECYCLE_METADATA_EVIDENCE: &str = "MentionableTextInput lifecycle metadata reuses only the local active @query, cached suggestion count, selected token, @room power-level allowance, and already loaded room_members cache for active query, no-match, keyboard selection, Tab/Enter insertion, click insertion, and trailing-space send release states. ArrowUp/ArrowDown only changes selected_suggestion_index, Tab/Enter/click only replace the active @token and append a trailing space, and completed mentions no longer intercept Enter before the existing RoomInputBar SendMessage path. This sends no remote member lookup, profile fetch, avatar fetch, popup search, pill editor, disambiguation UI, attachment/edit mention payload request, extra SendMessage, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MENTION_PICKER_LIFECYCLE_METADATA_LABEL: &str =
    "@mention lifecycle metadata is local; keyboard/click insertion only edits the active token.";
pub const MENTION_PICKER_KEYBOARD_SELECTION_EVIDENCE: &str = "MentionableTextInput ArrowUp/ArrowDown selection and Tab/Enter insertion are local active-token controls. Arrow keys only update selected_suggestion_index over cached @room or already loaded RoomMember suggestions, and Tab/Enter only replace the unfinished @token plus trailing space before returning Enter to the existing RoomInputBar SendMessage path. They start no remote member lookup, server-side member directory search, profile/avatar fetch, rich popup search, pill editor, attachment/edit mention payload, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MENTION_PICKER_KEYBOARD_SELECTION_LABEL: &str = "Mention keyboard selection: ArrowUp/ArrowDown and Tab/Enter stay local to cached active-token insertion.";
pub const MENTION_PICKER_RICH_POPUP_BOUNDARY_EVIDENCE: &str = "MentionableTextInput keeps rich mention picker scope as boundary metadata only: the compact row is not a floating popup search, rich highlighted result list, pill editor, disambiguation UI, remote member lookup, profile/avatar fetch, attachment/edit mention payload editor, membership mutation, gateway/runtime/auth, or live mutation path. It only reports active @query, cached suggestion count, selected token or no-match state, and loaded identity from RoomScreen room_members before the existing SendMessage mention payload path.";
pub const MENTION_PICKER_RICH_POPUP_BOUNDARY_LABEL: &str = "Rich mention popup, pill editor, and remote lookup stay unwired; compact cache row stays local.";
pub const MENTION_PICKER_DIRECTORY_DISAMBIGUATION_BOUNDARY_EVIDENCE: &str = "MentionableTextInput directory/disambiguation boundary metadata keeps mention_picker_send UI-safe after cached room_members suggestions and send-time Matrix Mentions. Directory can now submit a read-only MatrixRequest::SearchUserDirectory request for the active @query, render client.search_users result/error metadata, and expose up to three visible directory result promotion buttons that replace only the active @token with a literal Matrix user id before the existing SendMessage add_mentions path. duplicate display-name disambiguation UI, remote profile hover cards, avatar/profile fetch beyond directory response fields, rich highlighted popup results beyond the bounded buttons, multi-select mention tray, pill editor, attachment/edit mention payload editor, room-state, membership, account/profile, gateway/runtime/auth, and live mutation remain local blocked controls. The boundary is derived only from the active @query, cached suggestion count, selected token, loaded room_members cache size, and optional user-directory result; Hover can render a local snapshot from those cached values without submitting SendAttachment, extra SendMessage, room-state, membership, gateway/runtime/auth, or live mutation request.";
pub const MENTION_PICKER_DIRECTORY_DISAMBIGUATION_BOUNDARY_LABEL: &str = "Directory live reads Matrix user-directory metadata and can promote result rows to local @tokens; Hover can render cached local snapshots; duplicate-name disambiguation, remote profile hover-card adapters, pill editor, attachment/edit mention payloads, and live mutation stay local blocked.";
pub const MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE: &str = "MentionableTextInput exposes Rich, Directory, Hover, Tray, and Pills as visible controls in the compact @mention preview. Rich and Pills render a local rich mention packet snapshot from the active @query, cached suggestion count, selected token, @room power-level allowance, loaded room_members cache size, and the existing SendMessage/add_mentions handoff metadata. Directory submits MatrixRequest::SearchUserDirectory for a non-empty active @query, SlidingSync calls client.search_users, UserDirectorySearchAction::Searched repaints read-only result/error metadata, and the bounded directory result promotion row can insert a literal Matrix user id into the composer without sending. Hover renders a local hover-card snapshot from already available directory result metadata or the selected cached RoomMember/@room suggestion. Tray only updates the local rich/directory boundary label and popup copy. It starts no floating popup search, duplicate display-name disambiguation, remote profile hover card fetch, avatar/profile fetch beyond directory response fields, rich highlighted result list beyond bounded directory buttons, multi-select mention tray, pill editor mutation, attachment/edit mention payload editor, SendAttachment, extra SendMessage, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MENTION_PICKER_RICH_DIRECTORY_CONTROLS_LABEL: &str = "Directory reads Matrix user-directory metadata and can promote result rows locally; Hover renders local cached/directory hover-card snapshots; Rich, Tray, and Pills stay local mention-picker metadata actions.";
pub const MENTION_PICKER_HOVER_CARD_SNAPSHOT_LIVE_EVIDENCE: &str = "MentionableTextInput Hover is a live local hover-card snapshot built only from already available @mention metadata. If a Matrix user-directory result is cached, Hover summarizes up to three returned rows with user id, display name availability, avatar MXC presence, result count, and limited flag. Otherwise it summarizes the selected cached RoomMember/@room suggestion from loaded room_members and power-level state. It submits no MatrixRequest::SearchUserDirectory, no profile/avatar fetch, no remote hover-card request, no duplicate-name disambiguation workflow, no rich popup search, no multi-select tray mutation, no pill editor mutation, no SendMessage, no SendAttachment, no room-state, membership, account/profile, gateway/runtime/auth, retry automation, or live mutation request.";
pub const MENTION_PICKER_HOVER_CARD_SNAPSHOT_LIVE_LABEL: &str =
    "Mention Hover card snapshot uses cached directory or loaded member metadata only.";
pub const MENTION_PICKER_DIRECTORY_RESULT_PROMOTION_LIVE_EVIDENCE: &str = "MentionableTextInput renders up to three live user-directory result buttons after MatrixRequest::SearchUserDirectory returns. Clicking a directory result promotes only that result's literal Matrix user id into the active @token using the same local insert_mention_token path as cached suggestions, appends a trailing space, refreshes the completed mention pill tray, and relies on the existing SendMessage/add_mentions or attachment-caption AttachmentConfig.mentions scan later. It performs no automatic insertion on search completion, extra SendMessage, SendAttachment, profile/avatar fetch beyond directory response fields, duplicate-name disambiguation workflow, multi-select tray mutation, pill editor mutation, attachment/edit mention payload rewrite, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MENTION_PICKER_DIRECTORY_RESULT_PROMOTION_LIVE_LABEL: &str = "Directory result promotion: live directory rows can insert literal Matrix user-id @tokens locally.";
pub const MENTION_PICKER_LOCAL_PILL_TRAY_LIVE_EVIDENCE: &str = "MentionableTextInput exposes a live local completed-mention pill tray from composer text plus already loaded room_members cache. Completed @room, literal Matrix user-id, loaded-member display/localpart matches, and unmatched local @tokens are summarized after insertion; up to three visible pills can be clicked to remove that completed token from composer text. Removal only rewrites the local TextInput, recomputes cached suggestion/tray metadata, and updates the existing send-time Mentions payload preview. It submits no remote member lookup, profile/avatar fetch, server-side directory search, duplicate-name disambiguation, SendMessage, SendAttachment, edit/reply mention payload rewrite, room-state, membership, account/profile, gateway/runtime/auth, retry automation, or live mutation request.";
pub const MENTION_PICKER_LOCAL_PILL_TRAY_LIVE_LABEL: &str = "Mention pill tray: completed @tokens are removable local composer pills before SendMessage/add_mentions.";
pub const MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE: &str = "MentionableTextInput exposes Request, Result, Error, Retry, and Source as visible local preflight detail controls in the compact @mention preview. Clicking any control only updates local mention preflight metadata and popup copy from the active @query, cached suggestion count, selected token, @room power-level allowance, loaded room_members cache size, and existing SendMessage/add_mentions source. It starts no remote member lookup, server-side member directory search, duplicate-name disambiguation, profile/avatar fetch, rich popup search, pill editor, attachment/edit mention payload editor, SendAttachment, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, retry automation, or live mutation request.";
pub const MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_LABEL: &str = "Mention Request/Result/Error/Retry/Source detail stays local to cached suggestions and the existing SendMessage mention payload path.";
pub const MENTION_PICKER_SEND_PAYLOAD_METADATA_EVIDENCE: &str = "MentionableTextInput exposes send-time mention payload metadata for the existing RoomInputBar MatrixRequest::SendMessage and attachment-caption SendAttachment paths: text format, scanned @token count, deduped Matrix mention user count, literal Matrix user-id token count, cached RoomMember display/localpart match count, unmatched local token count, @room flag state, and loaded room_members cache size. The metadata is computed locally from the composer text, @room power-level allowance, and already loaded room_members cache before add_mentions attaches Matrix Mentions once for text sends or mentions_for_text provides AttachmentConfig.mentions for captioned media sends. It performs no remote member lookup, profile/avatar fetch, popup search, rich popup highlight styling, pill editor, disambiguation UI, rich attachment payload editor, edit mention payload request, extra SendMessage, extra SendAttachment, typing notice, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MENTION_PICKER_SEND_PAYLOAD_METADATA_LABEL: &str =
    "@mention send payload metadata: local token scan and loaded member-cache counts only.";
pub const MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_EVIDENCE: &str = "MentionableTextInput exposes Send, Attach, Edit, Reply, Source, Packet, Contract, and Taxonomy as visible local payload-scope controls in the compact @mention preview. Clicking any control only updates local payload-scope metadata and popup copy from the active @query, cached suggestion count, selected token, loaded room_members cache size, @room allowance, existing SendMessage/add_mentions source, and attachment-caption AttachmentConfig.mentions source. Packet persists the drilldown acceptance matrix; Contract maps it to typed rich-picker, directory, and Send/Attach/Edit/Reply payload contract slots; Taxonomy records remote hover/profile/disambiguation/edit-reply result slots as not-assigned or not-wired. It starts no rich attachment mention payload editor, edit-message mention payload rewrite, reply mention rewriting, remote member lookup, server-side member directory search beyond explicit Directory, profile/avatar fetch, remote hover-card fetch, rich popup search, pill editor, extra SendAttachment beyond the review-row handoff, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_LABEL: &str = "Mention Send/Attach/Edit/Reply/Source/Packet/Contract/Taxonomy scope: Send and attachment captions are live compact payloads; typed mention payload contract still gates rich/edit scopes.";
pub const MENTION_PICKER_PAYLOAD_DRILLDOWN_PACKET_EVIDENCE: &str = "MentionableTextInput exposes a visible Packet control that persists the mention payload drilldown acceptance matrix as local metadata while mention_picker_send remains a base gap. The packet combines rich picker, server directory, duplicate-name disambiguation, hover-card, tray, pills, SendMessage/add_mentions, attachment-caption AttachmentConfig.mentions, rich attachment/edit/reply payload scopes, and Request/Result/Error/Retry/Source preflight acceptance criteria from the active @query, cached suggestion count, selected token, loaded room_members cache size, and @room allowance. It starts no remote member lookup, server-side member directory search, duplicate-name disambiguation, profile/avatar fetch, rich popup search, hover-card fetch, pill editor mutation, rich attachment/edit/reply mention payload rewrite, extra SendAttachment beyond the review-row handoff, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, retry automation, or live mutation request.";
pub const MENTION_PICKER_PAYLOAD_DRILLDOWN_PACKET_LABEL: &str = "Mention payload drilldown packet: rich picker, directory, preflight, and Send/Attach/Edit/Reply payload-scope acceptance criteria stay local until typed mention contracts exist.";
pub const MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE: &str = "MentionableTextInput exposes a visible Contract control that maps the mention payload drilldown Packet to typed mention contract slots while mention_picker_send remains a base gap. The contract covers rich picker request/result/error/retry/source, server-directory lookup, duplicate-name disambiguation, hover-card source, tray state, pill draft, SendMessage/add_mentions handoff, attachment-caption AttachmentConfig.mentions handoff, rich attachment/edit/reply payload scopes, source-hash, stale-token handling, idempotency, and promotion blockers from the active @query, cached suggestion count, selected token, loaded room_members cache size, and @room allowance. It starts no remote member lookup, server-side member directory search, duplicate-name disambiguation, profile/avatar fetch, hover-card fetch, rich popup search, pill editor mutation, rich attachment/edit/reply mention payload rewrite, extra SendAttachment beyond the review-row handoff, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, retry automation, or live mutation request.";
pub const MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_LABEL: &str = "Mention Contract maps Packet to typed rich-picker, directory, and Send/Attach/Edit/Reply payload contracts locally.";
pub const MENTION_PICKER_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE: &str = "MentionableTextInput exposes a visible Taxonomy control that records remote hover/profile/disambiguation/edit-reply mention result slots as local metadata while mention_picker_send remains a base gap. The packet names the current live references as SendMessage/add_mentions, attachment-caption AttachmentConfig.mentions, read-only MatrixRequest::SearchUserDirectory/client.search_users, bounded directory result promotion to literal Matrix user-id tokens, local hover-card snapshots from cached directory or loaded member metadata, local completed-token pill tray removal, local rich packet snapshots, and local Packet/Contract metadata. Remote rich picker result promotion, richer directory result rendering, duplicate-name disambiguation, remote hover-card/profile fetch, avatar/profile fetch beyond directory response fields, rich attachment editors, edit/reply mention payload rewrites, multi-select tray, pill editor mutation, retry/cancel automation, source-hash reconciliation, and audit redaction remain not-assigned or not-wired. It performs no remote member lookup beyond explicit Directory, no profile/avatar fetch, no remote hover-card request, no duplicate-name disambiguation workflow, no rich popup search, no attachment/edit/reply mention payload rewrite, no extra SendMessage, no extra SendAttachment, no typing notice, no room-state, membership, account/profile, gateway/runtime/auth/provider, Telegram delivery, or live mutation request.";
pub const MENTION_PICKER_REMOTE_RESULT_TAXONOMY_PACKET_LABEL: &str = "Mention Taxonomy records remote hover/profile/disambiguation/edit-reply result slots as local not-wired metadata.";
const MAX_CACHED_MEMBER_SUGGESTIONS: usize = 3;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.MentionableTextInput = #(MentionableTextInput::register_widget(vm)) {
        width: Fill,
        height: Fit
        flow: Down

        text_input := RobrixTextInput {
            is_multiline: true,
        }
    }
}

#[derive(Debug)]
pub enum MentionableTextInputAction {
    PowerLevelsUpdated {
        room_id: OwnedRoomId,
        can_notify_room: bool,
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct MentionableTextInput {
    #[source] source: ScriptObjectRef,
    #[deref] view: View,

    #[rust] room_id: Option<OwnedRoomId>,
    #[rust] room_members: Option<Arc<Vec<RoomMember>>>,
    /// Cached room display name, refreshed on room change (avoids a per-keystroke lookup).
    #[rust] room_name: String,
    #[rust] can_notify_room: bool,

    #[rust] active_trigger: Option<ActiveTrigger>,
    #[rust] request_id: u64,

    /// A superset of possible mentions that might be in the current textinput.
    /// Mentions may have been deleted after adding them, so we have to check for them
    /// before sending the message in the textinput.
    #[rust] possible_mentions: Mentions,
}

impl Widget for MentionableTextInput {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        let popup_ref = self.popup_ref(cx);
        // Handle events/actions that are relevant to a currently-open mention popup.
        if popup_ref.is_open_for(uid) {
            // On a window resize, the textinput moved, so re-anchor the popup to the cursor.
            if let Event::Actions(actions) = event {
                if actions.iter().any(|a| matches!(a.as_widget_action().cast(), WindowAction::WindowGeomChange(_))) {
                    let anchor = self.popup_anchor(cx);
                    popup_ref.set_anchor(cx, uid, anchor);
                }
            }

            // When the mention popup is open, key presses like arrows, return, and escape should be forwarded
            // to it so it can handle them (instead of treating them as regular TextInput navigation).
            // Obviously we can't just give key focus to the popup because we still need to let
            // the user type characters into the text input so they can filter the matches in the popup.
            //
            // While we typically don't want to match on "raw" events (see the comments i added to the
            // various `Event` variants in Makepad), here we don't really have a choice because we're
            // handling events for a different widget and delivering events to it.
            let text_input_area = self.text_input_ref().area();
            if cx.has_key_focus(text_input_area) {
                if let Event::KeyDown(ke) = event {
                    match ke.key_code {
                        KeyCode::ArrowDown => {
                            popup_ref.move_focus(cx, 1);
                            return;
                        }
                        KeyCode::ArrowUp => {
                            popup_ref.move_focus(cx, -1);
                            return;
                        }
                        KeyCode::ReturnKey => {
                            if let Some(item) = popup_ref.focused_item() {
                                self.selection_made(cx, item);
                                return;
                            }
                        }
                        KeyCode::Escape => {
                            self.close_popup(cx);
                            return;
                        }
                        _ => {}
                    }
                }
            }

            /// Returns true if the tap/click location was outside of the text input or the mention popup.
            fn is_outside(cx: &mut Cx, pref: &MentionablePopupRef, input_area: &Area, loc: DVec2) -> bool {
                !pref.content_rect(cx).contains(loc) && !input_area.rect(cx).contains(loc)
            }

            // Dismiss the mention popup on a click/touch outside it, or upon a go-back gesture.
            let should_dismiss = event.back_pressed()
                || match event {
                    Event::MouseDown(e) => is_outside(cx, &popup_ref, &text_input_area, e.abs),
                    Event::TouchUpdate(e) => e.touches.iter().any(
                        |t| t.state == TouchState::Start && is_outside(cx, &popup_ref, &text_input_area, t.abs)
                    ),
                    _ => false,
                };
            if should_dismiss {
                self.close_popup(cx);
            }
        }

        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            for action in actions {
                if let Some(MentionableTextInputAction::PowerLevelsUpdated {
                    room_id,
                    can_notify_room,
                }) = action.downcast_ref()
                {
                    if self.room_id.as_ref() == Some(room_id)
                        && self.can_notify_room != *can_notify_room
                    {
                        self.can_notify_room = *can_notify_room;
                        if self.active_trigger.is_some_and(|t| t.kind == TriggerKind::User) {
                            self.refresh_popup(cx);
                        }
                    }
                }
                // Handle updated matches for the current query, but only if it's ours.
                if let Some(results) = action.downcast_ref::<MentionMatches>() {
                    if results.owner == uid && results.request_id == self.request_id {
                        let empty = self.active_trigger.map_or("", |t| t.kind.empty_message());
                        popup_ref.set_results(cx, uid, results.items.clone(), false, empty);
                    }
                }
            }

            if popup_ref.is_open_for(uid) {
                if let Some(item) = popup_ref.clicked_item(actions) {
                    self.selection_made(cx, item);
                    return;
                }
            }

            let text_input = self.text_input_ref();
            if cx.has_key_focus(text_input.area()) && text_input.changed(actions).is_some() {
                self.refresh_popup(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }

    fn text(&self) -> String {
        self.text_input_ref().text()
    }

    fn set_text(&mut self, cx: &mut Cx, text: &str) {
        self.text_input_ref().set_text(cx, text);
        if text.trim().is_empty() {
            self.possible_mentions = Mentions::new();
        }
        self.redraw(cx);
    }

    fn set_key_focus(&self, cx: &mut Cx) {
        self.text_input_ref().set_key_focus(cx);
    }
}

impl MentionableTextInput {
    fn text_input_ref(&self) -> TextInputRef {
        self.child_by_path(ids!(text_input)).as_text_input()
    }

    fn popup_ref(&self, cx: &mut Cx) -> MentionablePopupRef {
        if cx.has_global::<MentionablePopupRef>() {
            cx.get_global::<MentionablePopupRef>().clone()
        } else {
            MentionablePopupRef::default()
        }
    }

    /// Returns the rectangle of the current line of text, in absolute window coordinates.
    ///
    /// If the text input cursor isn't set, it returns the rectangle of the text input itself.
    fn popup_anchor(&self, cx: &mut Cx) -> Rect {
        self.text_input_ref()
            .cursor_rect_in_absolute(cx)
            .unwrap_or_else(|| self.text_input_ref().area().rect(cx))
    }

    /// Re-detects the active trigger (e.g., when text changed) and starts or closes the matching.
    fn refresh_popup(&mut self, cx: &mut Cx) {
        let text = self.text_input_ref().text();
        let cursor = self.text_input_ref().cursor().index;

        match detect_trigger(&text, cursor) {
            Some((kind, start_byte, query)) => {
                self.active_trigger = Some(ActiveTrigger { kind, start_byte });
                self.start_matching(cx, kind, &query);
            }
            None => self.close_popup(cx),
        }
    }

    fn start_matching(&mut self, cx: &mut Cx, kind: TriggerKind, query: &str) {
        let uid = self.widget_uid();
        let anchor = self.popup_anchor(cx);
        let trigger_start = self.active_trigger.map_or(0, |t| t.start_byte);
        let popup_ref = self.popup_ref(cx);
        popup_ref.show(cx, uid, anchor, trigger_start, kind.header(), kind.loading_message());

        match kind {
            TriggerKind::User => self.match_members(cx, &popup_ref, query),
            TriggerKind::Room => self.match_rooms(cx, &popup_ref, query),
            TriggerKind::Command => {
                let items = slash_commands::matching_commands(query)
                    .map(MentionItem::Command)
                    .collect();
                popup_ref.set_results(cx, uid, Arc::new(items), false, kind.empty_message());
            }
        }
    }

    fn match_members(&mut self, cx: &mut Cx, popup_ref: &MentionablePopupRef, query: &str) {
        let uid = self.widget_uid();
        self.request_id = self.request_id.wrapping_add(1);
        // Show the loading spinner while we wait for the member list / ranking.
        popup_ref.set_results(cx, uid, Arc::new(Vec::new()), true, TriggerKind::User.empty_message());

        let (Some(_), Some(members)) = (self.room_id.as_ref(), self.room_members.clone()) else {
            // Room members not yet available, just keep showing the loading spinner for now
            return;
        };
        let request_id = self.request_id;
        let query = query.to_string();
        let can_notify_room = self.can_notify_room;
        let room_name = self.room_name.clone();

        std::thread::spawn(move || {
            let current_user = crate::sliding_sync::current_user_id();
            let items = rank_members(&query, &members, can_notify_room, current_user, room_name);
            Cx::post_action(MentionMatches::new(request_id, uid, items));
        });
    }

    /// Submits a request for the background matrix worker task to match & rank rooms/space.
    fn match_rooms(&mut self, cx: &mut Cx, popup_ref: &MentionablePopupRef, query: &str) {
        let uid = self.widget_uid();
        self.request_id = self.request_id.wrapping_add(1);
        // Show the loading spinner while the worker task does the rooms/spaces ranking.
        popup_ref.set_results(cx, uid, Arc::new(Vec::new()), true, TriggerKind::Room.empty_message());

        submit_async_request(MatrixRequest::GetMatchingRooms {
            query: query.to_string(),
            request_id: self.request_id,
            owner: uid,
        });
    }

    /// The user selected the given `item`, so insert that item's text/link at the trigger location.
    fn selection_made(&mut self, cx: &mut Cx, item: MentionItem) {
        let Some(trigger) = self.active_trigger else {
            self.close_popup(cx);
            return;
        };

        let text_to_insert = match &item {
            MentionItem::User { user_id, display_name, .. } => {
                self.possible_mentions.user_ids.insert(user_id.clone());
                format!("[{}]({}) ", display_name, user_id.matrix_to_uri())
            }
            MentionItem::NotifyRoom { .. } => {
                self.possible_mentions.room = true;
                "@room ".to_string()
            }
            MentionItem::Room(candidate) => {
                // Prefer the room alias so we don't need the `via` servers list.
                let (label, uri) = match candidate.alias.as_ref() {
                    Some(alias) => (alias.to_string(), alias.matrix_to_uri()),
                    None => (candidate.room_name_id.to_string(), candidate.room_name_id.room_id().matrix_to_uri()),
                };
                format!("[{label}]({uri}) ")
            }
            MentionItem::Command(cmd) => format!("/{} ", cmd.name),
        };

        let text_input = self.text_input_ref();
        let text = text_input.text();
        let start = trigger.start_byte.min(text.len());
        if !text.is_char_boundary(start) {
            return;
        }
        // Replace the whole trigger and query substring, up until the next whitespace.
        let end = text[start..]
            .find(char::is_whitespace)
            .map_or(text.len(), |i| start + i);
        let new_text = utils::safe_replace_by_byte_indices(&text, start, end, &text_to_insert);

        text_input.set_text(cx, &new_text);
        text_input.set_cursor(
            cx,
            Cursor { index: start + text_to_insert.len(), prefer_next_row: false },
            false,
        );

        self.close_popup(cx);
        // give key focus back to the text input so the user can keep typing
        text_input.set_key_focus(cx);
        self.redraw(cx);
    }

    fn close_popup(&mut self, cx: &mut Cx) {
        let uid = self.widget_uid();
        self.active_trigger = None;
        // Invalidate any in-flight background match.
        self.request_id = self.request_id.wrapping_add(1);
        self.popup_ref(cx).hide(cx, uid);
        self.redraw(cx);
    }
}

impl MentionableTextInputRef {
    pub fn text_input_ref(&self) -> TextInputRef {
        self.borrow()
            .map(|inner| inner.text_input_ref())
            .unwrap_or_default()
    }

    /// Updates whether the user can `@room`. Refreshes an open `@` popup so the
    /// "Notify the entire room" entry appears or disappears accordingly.
    pub fn set_can_notify_room(&self, cx: &mut Cx, can_notify: bool) {
        let Some(mut inner) = self.borrow_mut() else { return };
        if inner.can_notify_room != can_notify {
            inner.can_notify_room = can_notify;
            if inner.active_trigger.is_some_and(|t| t.kind == TriggerKind::User) {
                inner.refresh_popup(cx);
            }
        }
    }

    /// Updates the room context the input matches against. The RoomScreen calls this on
    /// room change / member-list fetch, so we don't poll for these rare changes every event.
    pub fn set_room_context(
        &self,
        cx: &mut Cx,
        room_id: OwnedRoomId,
        room_members: Option<Arc<Vec<RoomMember>>>,
    ) {
        let Some(mut inner) = self.borrow_mut() else { return };
        let uid = inner.widget_uid();
        let room_changed = inner.room_id.as_ref() != Some(&room_id);
        if room_changed {
            inner.room_name = cx.has_global::<RoomsListRef>()
                .then(|| cx.get_global::<RoomsListRef>().get_room_name(&room_id))
                .flatten()
                .map(|n| n.to_string())
                .unwrap_or_default();
        }
        inner.room_id = Some(room_id);
        let members_arrived = inner.room_members.is_none() && room_members.is_some();
        inner.room_members = room_members;

        // The input is reused across rooms, so reset @room capability (re-fetched with
        // the new room's power levels) and close a popup left open in the old one.
        if room_changed {
            inner.can_notify_room = false;
            if inner.popup_ref(cx).is_open_for(uid) {
                inner.close_popup(cx);
            }
        }
        // Repopulate a "loading members" popup once the members arrive.
        if members_arrived && inner.active_trigger.is_some_and(|t| t.kind == TriggerKind::User) {
            inner.refresh_popup(cx);
        }
    }

    /// Returns a saved instance of this widget's state.
    pub fn save_state(&self) -> MentionableTextInputState {
        self.borrow().map_or_else(
            MentionableTextInputState::default,
            |inner| MentionableTextInputState {
                text_input_state: inner.text_input_ref().save_state(),
                possible_mentions: inner.possible_mentions.clone(),
            }
        )
    }

    pub fn restore_state(&self, cx: &mut Cx, state: MentionableTextInputState) {
        let Some(mut inner) = self.borrow_mut() else { return };
        inner.text_input_ref().restore_state(cx, state.text_input_state);
        inner.possible_mentions = state.possible_mentions;
    }

    /// Clears the possible mentions, e.g., after we've sent the edit.
    pub fn clear_mentions(&self) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.possible_mentions = Mentions::new();
        }
    }

    /// Creates a message from the given entered text, handling slash commands and mentions.
    pub fn create_message_with_mentions(&self, entered_text: &str) -> RoomMessageEventContent {
        if let Some(message) = slash_commands::build_message_for_command(entered_text) {
            return message;
        }

        let message = RoomMessageEventContent::text_markdown(entered_text);
        match self.borrow() {
            Some(inner) => message.add_mentions(inner.real_mentions_in_markdown(entered_text)),
            None => message,
        }
    }

    /// Returns the mentions whose links still exist in the given `text`.
    pub fn get_mentions_in(&self, text: &str) -> Mentions {
        self.borrow().map_or_else(Mentions::new, |inner| inner.real_mentions_in_markdown(text))
    }

    /// Refreshes the popup's local member cache without performing a remote lookup.
    pub fn update_cached_member_suggestions(
        &self,
        cx: &mut Cx,
        room_members: Option<&[RoomMember]>,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.room_members = room_members.map(|members| Arc::new(members.to_vec()));
        if inner.active_trigger.is_some_and(|trigger| trigger.kind == TriggerKind::User) {
            inner.refresh_popup(cx);
        }
    }

    /// Returns the already-selected local mention payload for text or attachment captions.
    pub fn mentions_for_text(
        &self,
        entered_text: &str,
        _room_members: Option<&[RoomMember]>,
    ) -> Option<Mentions> {
        let mentions = self.get_mentions_in(entered_text);
        (!mentions.user_ids.is_empty() || mentions.room).then_some(mentions)
    }

    /// Summarizes the local send-time mention payload without issuing any Matrix request.
    pub fn send_payload_metadata_label(
        &self,
        entered_text: &str,
        room_members: Option<&[RoomMember]>,
    ) -> String {
        let mentions = self.mentions_for_text(entered_text, room_members);
        let user_count = mentions
            .as_ref()
            .map_or(0, |mentions| mentions.user_ids.len());
        let room_flag = mentions.as_ref().is_some_and(|mentions| mentions.room);
        let token_count = entered_text
            .split_whitespace()
            .filter(|token| token.starts_with('@'))
            .count();
        format!(
            "Mention send payload metadata: scanned @tokens {token_count}; Matrix mention users {user_count}; @room flag {room_flag}; loaded room_members {}.",
            room_members.map_or(0, <[RoomMember]>::len),
        )
    }
}

impl MentionableTextInput {
    /// The possible mentions whose link text is still present in `text`.
    fn real_mentions_in_markdown(&self, text: &str) -> Mentions {
        let mut user_ids = BTreeSet::new();
        for user_id in &self.possible_mentions.user_ids {
            // Match on the link's URI, not its label, so editing the displayed name
            // (while keeping the link) still counts as a mention.
            let by_uri = format!("]({})", user_id.matrix_to_uri());
            if text.contains(&by_uri) {
                user_ids.insert(user_id.clone());
            }
        }

        let mut mentions = Mentions::new();
        mentions.user_ids = user_ids;
        mentions.room = self.possible_mentions.room && contains_room_mention(text);
        mentions
    }
}


/// The saved state of a `MentionableTextInput`.
#[derive(Clone, Default)]
pub struct MentionableTextInputState {
    text_input_state: TextInputState,
    possible_mentions: Mentions,
}

/// Matched users or rooms/spaces, ranked on a background thread.
#[derive(Clone, Debug)]
pub struct MentionMatches {
    request_id: u64,
    owner: WidgetUid,
    items: Arc<Vec<MentionItem>>,
}
impl MentionMatches {
    pub fn new(request_id: u64, owner: WidgetUid, items: Vec<MentionItem>) -> Self {
        Self { request_id, owner, items: Arc::new(items) }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TriggerKind {
    User,
    Room,
    Command,
}

impl TriggerKind {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '@' => Some(TriggerKind::User),
            '#' => Some(TriggerKind::Room),
            '/' => Some(TriggerKind::Command),
            _ => None,
        }
    }

    fn header(self) -> &'static str {
        match self {
            TriggerKind::User => "Mention a user in this room",
            TriggerKind::Room => "Link to a room or space",
            TriggerKind::Command => "Special Commands",
        }
    }

    fn empty_message(self) -> &'static str {
        match self {
            TriggerKind::User => "No matching users",
            TriggerKind::Room => "No matching rooms or spaces",
            TriggerKind::Command => "No matching commands",
        }
    }

    fn loading_message(self) -> &'static str {
        match self {
            TriggerKind::User => "Loading user members…",
            TriggerKind::Room => "Loading rooms…",
            TriggerKind::Command => "Loading commands…",
        }
    }
}

#[derive(Clone, Copy)]
struct ActiveTrigger {
    kind: TriggerKind,
    /// The byte-wise index of the trigger character within the text.
    start_byte: usize,
}

/// Returns true if `text` contains a standalone `@room` word.
fn contains_room_mention(text: &str) -> bool {
    const ROOM_MENTION: &str = "@room";
    text.match_indices(ROOM_MENTION).any(|(i, _)| {
        let has_whitespace_before = text[..i].chars().next_back().is_none_or(|c| c.is_whitespace());
        let has_whitespace_after  = text[i + ROOM_MENTION.len()..].chars().next().is_none_or(|c| c.is_whitespace());
        has_whitespace_before && has_whitespace_after
    })
}

fn member_display_name(member: &RoomMember) -> &str {
    member.display_name().unwrap_or_else(|| member.user_id().as_str())
}

/// Ranks and builds all matching members.
///
/// Note: run this on a bg thread, as it can be computationally expensive.
fn rank_members(
    query: &str,
    members: &[RoomMember],
    can_notify_room: bool,
    current_user: Option<OwnedUserId>,
    room_name: String,
) -> Vec<MentionItem> {
    let query_lower = query.to_lowercase();
    let mut ranked: Vec<((MatchQuality, u8), String, usize)> = members
        .iter()
        .enumerate()
        .filter(|(_, m)| current_user.as_deref() != Some(m.user_id()))
        .filter_map(|(i, m)| {
            let display_lower = member_display_name(m).to_lowercase();
            let localpart_lower = m.user_id().localpart().to_lowercase();
            user_match_priority(&display_lower, &localpart_lower, &query_lower).map(|p| (p, display_lower, i))
        })
        .collect();
    ranked.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    let mut items = Vec::with_capacity(ranked.len() + 1);
    if can_notify_room && (query_lower.is_empty() || "room".starts_with(&query_lower)) {
        items.push(MentionItem::NotifyRoom { room_name });
    }
    for (_, _, i) in ranked {
        let member = &members[i];
        items.push(MentionItem::User {
            user_id: member.user_id().to_owned(),
            display_name: member_display_name(member).to_owned(),
            avatar_url: member.avatar_url().map(ToOwned::to_owned),
        });
    }
    items
}

fn user_match_priority(display_lower: &str, localpart_lower: &str, query_lc: &str) -> Option<(MatchQuality, u8)> {
    if query_lc.is_empty() {
        return Some((MatchQuality::Substring, u8::MAX));
    }
    [ (MatchQuality::of(display_lower, query_lc), 0u8), (MatchQuality::of(localpart_lower, query_lc), 1u8) ]
        .into_iter()
        .filter(|(q, _)| q.is_match())
        .min()
}

/// Finds the active trigger "token" that ends at the current cursor, if any.
///
/// Returns a tuple of: (the detected trigger, the trigger's byte location, the query string).
///
/// We only accept '@' and '#' if there's leading whitespace before it (or they're at the beginning),
/// and '/' only if it's at the beginning.
fn detect_trigger(text: &str, cursor_byte: usize) -> Option<(TriggerKind, usize, String)> {
    if cursor_byte == 0 {
        return None;
    }
    // Start of the whitespace-delimited token the cursor sits in.
    let token_start = text[..cursor_byte]
        .char_indices()
        .rev()
        .find(|(_, c)| c.is_whitespace())
        .map_or(0, |(i, c)| i + c.len_utf8());
    if token_start >= cursor_byte {
        return None; // cursor sits right after whitespace: empty token
    }

    // The trigger is the token's first char, so a second trigger char (like "@@" or "#@")
    // should be treated as part of the query text, not another trigger.
    let trigger_char = text[token_start..cursor_byte].chars().next()?;
    let kind = TriggerKind::from_char(trigger_char)?;
    if kind == TriggerKind::Command && token_start != 0 {
        return None;
    }

    let query = text[token_start + trigger_char.len_utf8()..cursor_byte].to_string();
    Some((kind, token_start, query))
}
