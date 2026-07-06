//! A temporary lightweight MentionableTextInput that uses a simple TextInput
//! plus compact @mention extraction and cached-member suggestions for the
//! existing composer send path.
//!
//! This preserves the same external-facing API so that the real MentionableTextInput
//! can be slotted back in later without changing the code that depends on it.

use makepad_widgets::*;
use matrix_sdk::{
    room::RoomMember,
    ruma::{
        events::{room::message::RoomMessageEventContent, Mentions},
        OwnedRoomId, OwnedUserId, UserId,
    },
};

use crate::{
    shared::popup_list::{enqueue_popup_notification, PopupKind},
    sliding_sync::{
        sanitize_user_directory_search_query, submit_async_request, MatrixRequest,
        UserDirectorySearchAction, UserDirectorySearchEntry, UserDirectorySearchResult,
    },
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

        // Keep the same nested structure so that external DSL overrides
        // (e.g., `persistent.center.text_input.empty_text`) still work.
        persistent := RoundedView {
            width: Fill,
            height: Fit,
            flow: Down,
            top := View { height: 0 }
            center := RoundedView {
                width: Fill,
                height: Fit,
                text_input := RobrixTextInput {
                    empty_text: "Start typing..."
                    is_multiline: true,
                }
            }
            bottom := View {
                width: Fill,
                height: Fit,
                flow: Down,

                mention_preview := RoundedView {
                    visible: false
                    width: Fill,
                    height: Fit,
                    flow: Down,
                    spacing: 4.0,
                    margin: Inset{top: 6.0}
                    padding: Inset{top: 7.0, bottom: 7.0, left: 10.0, right: 10.0}
                    show_bg: true,
                    draw_bg +: {
                        color: #1F2C3A
                        border_color: #2AABEE66
                        border_size: 1.0
                        border_radius: 8.0
                    }

                    mention_preview_title := Label {
                        width: Fill,
                        height: Fit,
                        draw_text +: {
                            color: #2AABEE
                        }
                        text: "@mention helper"
                    }

                    mention_preview_summary := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: #D8F0FF
                        }
                        text: "Local @mention payload preview. Uses loaded member cache only."
                    }

                    mention_option_evidence := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: #A6CFE8
                        }
                        text: "@room and cached @user mentions attach Matrix Mentions on send; no remote member lookup or popup search."
                    }

                    mention_suggestion_row := View {
                        visible: false
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        padding: Inset{top: 2.0, bottom: 2.0}

                        mention_room_option_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "@room"
                        }
                        mention_member_option_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "@user"
                        }
                        mention_member_option_button_2 := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "@user"
                        }
                        mention_member_option_button_3 := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "@user"
                        }
                    }

                    mention_pill_tray := View {
                        visible: false
                        width: Fill,
                        height: Fit,
                        flow: Down,
                        spacing: 4.0,
                        padding: Inset{top: 2.0, bottom: 2.0}

                        mention_pill_tray_summary := Label {
                            width: Fill,
                            height: Fit,
                            flow: Flow.Right{wrap: true}
                            draw_text +: {
                                color: #D8F0FF
                            }
                            text: "Completed mention pills appear here."
                        }

                        mention_pill_button_row := View {
                            width: Fill,
                            height: Fit,
                            flow: Flow.Right{wrap: true},
                            spacing: 6.0

                            mention_pill_button_1 := RobrixNeutralIconButton {
                                width: Fit,
                                height: 32,
                                text: "Remove @"
                            }
                            mention_pill_button_2 := RobrixNeutralIconButton {
                                width: Fit,
                                height: 32,
                                text: "Remove @"
                            }
                            mention_pill_button_3 := RobrixNeutralIconButton {
                                width: Fit,
                                height: 32,
                                text: "Remove @"
                            }
                        }
                    }

                    mention_rich_directory_controls_row := View {
                        visible: false
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        padding: Inset{top: 2.0, bottom: 2.0}

                        mention_rich_control_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Rich"
                        }
                        mention_directory_control_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Directory"
                        }
                        mention_hover_control_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Hover"
                        }
                        mention_tray_control_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Tray"
                        }
                        mention_pills_control_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Pills"
                        }
                    }

                    mention_directory_result_promotion_row := View {
                        visible: false
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        padding: Inset{top: 2.0, bottom: 2.0}

                        mention_directory_result_button_1 := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "@user"
                        }
                        mention_directory_result_button_2 := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "@user"
                        }
                        mention_directory_result_button_3 := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "@user"
                        }
                    }

                    mention_preflight_detail_controls := View {
                        visible: false
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        padding: Inset{top: 2.0, bottom: 2.0}

                        mention_preflight_request_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Request"
                        }
                        mention_preflight_result_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Result"
                        }
                        mention_preflight_error_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Error"
                        }
                        mention_preflight_retry_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Retry"
                        }
                        mention_preflight_source_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Source"
                        }
                    }

                    mention_payload_scope_controls := View {
                        visible: false
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true},
                        spacing: 6.0,
                        padding: Inset{top: 2.0, bottom: 2.0}

                        mention_payload_send_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Send"
                        }
                        mention_payload_attach_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Attach"
                        }
                        mention_payload_edit_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Edit"
                        }
                        mention_payload_reply_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Reply"
                        }
                        mention_payload_source_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Source"
                        }
                        mention_payload_drilldown_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Packet"
                        }
                        mention_payload_contract_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Contract"
                        }
                        mention_payload_taxonomy_button := RobrixNeutralIconButton {
                            width: Fit,
                            height: 32,
                            text: "Taxonomy"
                        }
                    }

                    mention_identity_preview := Label {
                        visible: false
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: #D8F0FF
                        }
                        text: "Loaded member identity appears here without remote lookup."
                    }

                    mention_candidate_rows_preview := Label {
                        visible: false
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: #BEE7FF
                        }
                        text: "Local candidate rows appear here without remote lookup."
                    }

                    mention_duplicate_hints_preview := Label {
                        visible: false
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: #F4D38B
                        }
                        text: "Local duplicate-name hints appear here without directory lookup."
                    }

                    mention_rich_popup_boundary := Label {
                        visible: false
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: #F0C36A
                        }
                        text: "Rich picker boundary stays local."
                    }

                    mention_preflight_detail_metadata := Label {
                        visible: false
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: #9DECC9
                        }
                        text: "Mention preflight detail stays local."
                    }

                    mention_payload_scope_metadata := Label {
                        visible: false
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: #B6EAD9
                        }
                        text: "Mention payload scope stays local."
                    }

                    mention_send_boundary_evidence := Label {
                        width: Fill,
                        height: Fit,
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: #A6CFE8
                        }
                        text: "@mention payload: local cache extraction only; no remote member lookup, popup search, pill editor, room-state, membership, or live mutation."
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum MentionableTextInputAction {
    /// Notifies the MentionableTextInput about updated power levels for the room.
    PowerLevelsUpdated {
        room_id: OwnedRoomId,
        can_notify_room: bool,
    },
}

/// Lightweight widget that wraps a simple TextInput while preserving the same
/// external API as the original MentionableTextInput.
#[derive(Script, ScriptHook, Widget)]
pub struct MentionableTextInput {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    /// Whether the current user can notify everyone in the room (@room mention).
    #[rust]
    can_notify_room: bool,
    #[rust]
    mention_preview_visible: bool,
    #[rust]
    cached_member_suggestions: Vec<MentionSuggestion>,
    #[rust]
    cached_member_lookup: Vec<CachedMentionMember>,
    #[rust]
    local_mention_pills: Vec<MentionPill>,
    #[rust]
    selected_suggestion_index: usize,
    #[rust]
    active_mention_query_cache: Option<String>,
    #[rust]
    mention_rich_directory_last_action: Option<String>,
    #[rust]
    mention_directory_search_last_query: Option<String>,
    #[rust]
    mention_directory_search_result: Option<UserDirectorySearchResult>,
    #[rust]
    mention_directory_search_error: Option<String>,
    #[rust]
    mention_preflight_detail_last_action: Option<String>,
    #[rust]
    mention_payload_scope_last_action: Option<String>,
}

impl Widget for MentionableTextInput {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if cx.has_key_focus(
            self.text_input(cx, ids!(persistent.center.text_input))
                .area(),
        ) && self.has_insertable_suggestion()
        {
            if let Event::KeyDown(KeyEvent {
                key_code,
                modifiers:
                    KeyModifiers {
                        shift: false,
                        control: false,
                        alt: false,
                        logo: false,
                    },
                ..
            }) = event
            {
                match key_code {
                    KeyCode::ArrowDown => {
                        self.move_selected_suggestion(cx, 1);
                        return;
                    }
                    KeyCode::ArrowUp => {
                        self.move_selected_suggestion(cx, -1);
                        return;
                    }
                    KeyCode::Tab | KeyCode::ReturnKey => {
                        if let Some(token) = self.selected_suggestion_token() {
                            self.insert_mention_token(cx, &token);
                            return;
                        }
                    }
                    _ => {}
                }
            }
        }

        self.view.handle_event(cx, event, scope);

        // Handle MentionableTextInputAction for API compatibility.
        if let Event::Actions(actions) = event {
            for action in actions {
                if let Some(MentionableTextInputAction::PowerLevelsUpdated {
                    can_notify_room,
                    ..
                }) = action.downcast_ref()
                {
                    self.can_notify_room = *can_notify_room;
                    self.update_local_mention_preview(cx);
                }
                if let Some(UserDirectorySearchAction::Searched(result)) = action.downcast_ref() {
                    self.handle_user_directory_search_action(cx, result);
                }
            }

            if self
                .view
                .button(
                    cx,
                    ids!(
                        persistent
                            .bottom
                            .mention_preview
                            .mention_suggestion_row
                            .mention_room_option_button
                    ),
                )
                .clicked(actions)
            {
                self.insert_mention_token(cx, "@room");
            }

            self.handle_member_suggestion_click(
                cx,
                actions,
                0,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_suggestion_row
                        .mention_member_option_button
                ),
            );
            self.handle_member_suggestion_click(
                cx,
                actions,
                1,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_suggestion_row
                        .mention_member_option_button_2
                ),
            );
            self.handle_member_suggestion_click(
                cx,
                actions,
                2,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_suggestion_row
                        .mention_member_option_button_3
                ),
            );
            self.handle_completed_mention_pill_click(
                cx,
                actions,
                0,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_pill_tray
                        .mention_pill_button_row
                        .mention_pill_button_1
                ),
            );
            self.handle_completed_mention_pill_click(
                cx,
                actions,
                1,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_pill_tray
                        .mention_pill_button_row
                        .mention_pill_button_2
                ),
            );
            self.handle_completed_mention_pill_click(
                cx,
                actions,
                2,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_pill_tray
                        .mention_pill_button_row
                        .mention_pill_button_3
                ),
            );
            self.handle_rich_directory_control_click(
                cx,
                actions,
                "Rich",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_rich_directory_controls_row
                        .mention_rich_control_button
                ),
            );
            self.handle_rich_directory_control_click(
                cx,
                actions,
                "Directory",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_rich_directory_controls_row
                        .mention_directory_control_button
                ),
            );
            self.handle_rich_directory_control_click(
                cx,
                actions,
                "Hover",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_rich_directory_controls_row
                        .mention_hover_control_button
                ),
            );
            self.handle_rich_directory_control_click(
                cx,
                actions,
                "Tray",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_rich_directory_controls_row
                        .mention_tray_control_button
                ),
            );
            self.handle_rich_directory_control_click(
                cx,
                actions,
                "Pills",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_rich_directory_controls_row
                        .mention_pills_control_button
                ),
            );
            self.handle_directory_result_promotion_click(
                cx,
                actions,
                0,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_directory_result_promotion_row
                        .mention_directory_result_button_1
                ),
            );
            self.handle_directory_result_promotion_click(
                cx,
                actions,
                1,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_directory_result_promotion_row
                        .mention_directory_result_button_2
                ),
            );
            self.handle_directory_result_promotion_click(
                cx,
                actions,
                2,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_directory_result_promotion_row
                        .mention_directory_result_button_3
                ),
            );
            self.handle_mention_preflight_detail_control_click(
                cx,
                actions,
                "Request",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_preflight_detail_controls
                        .mention_preflight_request_button
                ),
            );
            self.handle_mention_preflight_detail_control_click(
                cx,
                actions,
                "Result",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_preflight_detail_controls
                        .mention_preflight_result_button
                ),
            );
            self.handle_mention_preflight_detail_control_click(
                cx,
                actions,
                "Error",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_preflight_detail_controls
                        .mention_preflight_error_button
                ),
            );
            self.handle_mention_preflight_detail_control_click(
                cx,
                actions,
                "Retry",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_preflight_detail_controls
                        .mention_preflight_retry_button
                ),
            );
            self.handle_mention_preflight_detail_control_click(
                cx,
                actions,
                "Source",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_preflight_detail_controls
                        .mention_preflight_source_button
                ),
            );
            self.handle_mention_payload_scope_control_click(
                cx,
                actions,
                "Send",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_payload_scope_controls
                        .mention_payload_send_button
                ),
            );
            self.handle_mention_payload_scope_control_click(
                cx,
                actions,
                "Attach",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_payload_scope_controls
                        .mention_payload_attach_button
                ),
            );
            self.handle_mention_payload_scope_control_click(
                cx,
                actions,
                "Edit",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_payload_scope_controls
                        .mention_payload_edit_button
                ),
            );
            self.handle_mention_payload_scope_control_click(
                cx,
                actions,
                "Reply",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_payload_scope_controls
                        .mention_payload_reply_button
                ),
            );
            self.handle_mention_payload_scope_control_click(
                cx,
                actions,
                "Source",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_payload_scope_controls
                        .mention_payload_source_button
                ),
            );
            self.handle_mention_payload_scope_control_click(
                cx,
                actions,
                "Packet",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_payload_scope_controls
                        .mention_payload_drilldown_button
                ),
            );
            self.handle_mention_payload_scope_control_click(
                cx,
                actions,
                "Contract",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_payload_scope_controls
                        .mention_payload_contract_button
                ),
            );
            self.handle_mention_payload_scope_control_click(
                cx,
                actions,
                "Taxonomy",
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_payload_scope_controls
                        .mention_payload_taxonomy_button
                ),
            );
        }
        self.update_local_mention_preview(cx);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }

    fn text(&self) -> String {
        self.child_by_path(ids!(text_input)).as_text_input().text()
    }

    fn set_text(&mut self, cx: &mut Cx, text: &str) {
        self.text_input(cx, ids!(persistent.center.text_input))
            .set_text(cx, text);
        self.update_local_mention_preview(cx);
        self.redraw(cx);
    }

    fn set_key_focus(&self, cx: &mut Cx) {
        self.text_input(cx, ids!(persistent.center.text_input))
            .set_key_focus(cx);
    }
}

impl MentionableTextInput {
    fn update_local_mention_preview(&mut self, cx: &mut Cx) {
        let current_text = self.text();
        let active_query = active_mention_query(&current_text).map(str::to_owned);
        self.local_mention_pills = completed_mention_pills_for_text(
            &current_text,
            &self.cached_member_lookup,
            self.can_notify_room,
        );
        if self.active_mention_query_cache != active_query {
            self.active_mention_query_cache = active_query.clone();
            self.selected_suggestion_index = 0;
        }
        let has_completed_pills = !self.local_mention_pills.is_empty();
        let should_show = active_query.is_some() || has_completed_pills;
        self.clamp_selected_suggestion_index();
        let status = if self.can_notify_room {
            "Local @mention picker: @room and cached @user choices insert composer tokens. Send attaches Matrix Mentions; no remote member lookup is started."
        } else {
            "Local @mention picker: cached @user choices insert composer tokens. @room is unavailable for this power level; no remote member lookup is started."
        };

        self.view
            .label(
                cx,
                ids!(persistent.bottom.mention_preview.mention_preview_summary),
            )
            .set_text(cx, status);
        self.view
            .label(
                cx,
                ids!(persistent.bottom.mention_preview.mention_option_evidence),
            )
            .set_text(
                cx,
                &self.cached_suggestion_status_text(active_query.as_deref()),
            );
        self.view
            .label(
                cx,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_send_boundary_evidence
                ),
            )
            .set_text(cx, MENTION_PICKER_SEND_LOCAL_BOUNDARY_LABEL);

        let show_suggestion_row = active_query.is_some() && self.has_insertable_suggestion();
        self.view
            .view(
                cx,
                ids!(persistent.bottom.mention_preview.mention_suggestion_row),
            )
            .set_visible(cx, show_suggestion_row);
        let room_button = self.view.button(
            cx,
            ids!(
                persistent
                    .bottom
                    .mention_preview
                    .mention_suggestion_row
                    .mention_room_option_button
            ),
        );
        room_button.set_visible(cx, active_query.is_some() && self.can_notify_room);
        room_button.set_text(cx, &self.suggestion_button_label(0, "@room"));
        self.update_member_suggestion_button(
            cx,
            ids!(
                persistent
                    .bottom
                    .mention_preview
                    .mention_suggestion_row
                    .mention_member_option_button
            ),
            0,
            active_query.is_some(),
        );
        self.update_member_suggestion_button(
            cx,
            ids!(
                persistent
                    .bottom
                    .mention_preview
                    .mention_suggestion_row
                    .mention_member_option_button_2
            ),
            1,
            active_query.is_some(),
        );
        self.update_member_suggestion_button(
            cx,
            ids!(
                persistent
                    .bottom
                    .mention_preview
                    .mention_suggestion_row
                    .mention_member_option_button_3
            ),
            2,
            active_query.is_some(),
        );
        self.update_local_pill_tray(cx, has_completed_pills);
        self.view
            .view(
                cx,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_rich_directory_controls_row
                ),
            )
            .set_visible(cx, active_query.is_some());
        self.update_directory_result_promotion_row(cx, active_query.is_some());
        self.view
            .view(
                cx,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_payload_scope_controls
                ),
            )
            .set_visible(cx, active_query.is_some());
        self.view
            .view(
                cx,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_preflight_detail_controls
                ),
            )
            .set_visible(cx, active_query.is_some());
        let identity_preview = self.view.label(
            cx,
            ids!(persistent.bottom.mention_preview.mention_identity_preview),
        );
        identity_preview.set_text(
            cx,
            &self.selected_suggestion_identity_preview(active_query.as_deref()),
        );
        identity_preview.set_visible(cx, active_query.is_some());
        let candidate_rows_preview = self.view.label(
            cx,
            ids!(
                persistent
                    .bottom
                    .mention_preview
                    .mention_candidate_rows_preview
            ),
        );
        candidate_rows_preview.set_text(
            cx,
            &self.local_candidate_rows_preview(active_query.as_deref()),
        );
        candidate_rows_preview.set_visible(cx, active_query.is_some());
        let duplicate_hints_preview = self.view.label(
            cx,
            ids!(
                persistent
                    .bottom
                    .mention_preview
                    .mention_duplicate_hints_preview
            ),
        );
        duplicate_hints_preview.set_text(
            cx,
            &self.local_duplicate_hints_preview(active_query.as_deref()),
        );
        duplicate_hints_preview.set_visible(cx, active_query.is_some());
        let selected_token = self.selected_suggestion_token();
        let rich_popup_boundary = self.view.label(
            cx,
            ids!(
                persistent
                    .bottom
                    .mention_preview
                    .mention_rich_popup_boundary
            ),
        );
        rich_popup_boundary.set_text(
            cx,
            &self.mention_rich_directory_status_label(
                self.mention_rich_directory_last_action.as_deref(),
                active_query.as_deref(),
                self.suggestion_count(),
                selected_token.as_deref(),
                self.cached_member_suggestions.len(),
            ),
        );
        rich_popup_boundary.set_visible(cx, active_query.is_some());
        let preflight_detail = self.view.label(
            cx,
            ids!(
                persistent
                    .bottom
                    .mention_preview
                    .mention_preflight_detail_metadata
            ),
        );
        preflight_detail.set_text(
            cx,
            &mention_picker_preflight_detail_controls_label(
                self.mention_preflight_detail_last_action.as_deref(),
                active_query.as_deref(),
                self.suggestion_count(),
                selected_token.as_deref(),
                self.cached_member_suggestions.len(),
                self.can_notify_room,
            ),
        );
        preflight_detail.set_visible(cx, active_query.is_some());
        let payload_scope = self.view.label(
            cx,
            ids!(
                persistent
                    .bottom
                    .mention_preview
                    .mention_payload_scope_metadata
            ),
        );
        payload_scope.set_text(
            cx,
            &mention_picker_payload_scope_controls_label(
                self.mention_payload_scope_last_action.as_deref(),
                active_query.as_deref(),
                self.suggestion_count(),
                selected_token.as_deref(),
                self.cached_member_suggestions.len(),
                self.can_notify_room,
            ),
        );
        payload_scope.set_visible(cx, active_query.is_some());

        if self.mention_preview_visible != should_show {
            self.mention_preview_visible = should_show;
            self.view
                .view(cx, ids!(persistent.bottom.mention_preview))
                .set_visible(cx, should_show);
            self.redraw(cx);
        }
    }

    fn mention_rich_directory_status_label(
        &self,
        action: Option<&str>,
        active_query: Option<&str>,
        suggestion_count: usize,
        selected_token: Option<&str>,
        loaded_member_cache_size: usize,
    ) -> String {
        if action.is_some_and(|action| action == "Directory") {
            if let Some(result) = self.mention_directory_search_result.as_ref() {
                return mention_picker_directory_search_result_label(
                    result,
                    suggestion_count,
                    selected_token,
                    loaded_member_cache_size,
                );
            }
            if let Some(error) = self.mention_directory_search_error.as_deref() {
                return mention_picker_directory_search_error_label(
                    self.mention_directory_search_last_query.as_deref(),
                    active_query,
                    error,
                    suggestion_count,
                    selected_token,
                    loaded_member_cache_size,
                );
            }
            if let Some(query) = self.mention_directory_search_last_query.as_deref() {
                return mention_picker_directory_search_request_label(
                    query,
                    active_query,
                    suggestion_count,
                    selected_token,
                    loaded_member_cache_size,
                );
            }
        }
        if action.is_some_and(|action| action == "Hover") {
            return mention_picker_hover_card_snapshot_label(
                active_query,
                suggestion_count,
                selected_token,
                loaded_member_cache_size,
                self.mention_directory_search_result.as_ref(),
                self.selected_cached_member_suggestion(),
                self.can_notify_room,
            );
        }
        mention_picker_rich_directory_controls_label(
            action,
            active_query,
            suggestion_count,
            selected_token,
            loaded_member_cache_size,
        )
    }

    fn handle_user_directory_search_action(
        &mut self,
        cx: &mut Cx,
        result: &Result<UserDirectorySearchResult, String>,
    ) {
        if self.mention_directory_search_last_query.is_none() {
            return;
        }
        match result {
            Ok(result) => {
                if self
                    .mention_directory_search_last_query
                    .as_deref()
                    .is_some_and(|query| query != result.query)
                {
                    return;
                }
                self.mention_directory_search_error = None;
                self.mention_directory_search_result = Some(result.clone());
                enqueue_popup_notification(
                    format!(
                        "Mention Directory live search returned {} Matrix user-directory rows for @{}. Result buttons can locally promote Matrix user-id tokens; SendMessage, SendAttachment, room-state, membership, gateway/runtime/auth, and live mutation were not submitted.",
                        result.results.len(),
                        result.query,
                    ),
                    PopupKind::Info,
                    Some(5.0),
                );
            }
            Err(error) => {
                self.mention_directory_search_result = None;
                self.mention_directory_search_error = Some(error.clone());
                enqueue_popup_notification(
                    format!(
                        "Mention Directory live search failed: {error}. Existing cached suggestions and send-time mentions stay available; no extra send or mutation was submitted."
                    ),
                    PopupKind::Error,
                    Some(5.0),
                );
            }
        }
        self.update_local_mention_preview(cx);
        self.redraw(cx);
    }

    fn insert_mention_token(&mut self, cx: &mut Cx, token: &str) {
        let current_text = self.text();
        let next_text = replace_active_mention_token(&current_text, token);
        self.text_input(cx, ids!(persistent.center.text_input))
            .set_text(cx, &next_text);
        self.update_local_mention_preview(cx);
        self.redraw(cx);
    }

    fn handle_member_suggestion_click(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
        index: usize,
        path: &[LiveId],
    ) {
        if self.view.button(cx, path).clicked(actions) {
            if let Some(token) = self
                .cached_member_suggestions
                .get(index)
                .map(|suggestion| suggestion.token.clone())
            {
                self.insert_mention_token(cx, &token);
            }
        }
    }

    fn handle_directory_result_promotion_click(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
        index: usize,
        path: &[LiveId],
    ) {
        if self.view.button(cx, path).clicked(actions) {
            let Some(token) = self.directory_result_promotion_token(index) else {
                return;
            };
            self.insert_mention_token(cx, &token);
            self.mention_rich_directory_last_action = Some("Directory result".to_string());
            enqueue_popup_notification(
                format!(
                    "Promoted Directory result {token} into the composer locally. SendMessage, SendAttachment, profile/avatar fetch, room-state, membership, gateway/runtime/auth, and live mutation were not submitted."
                ),
                PopupKind::Info,
                Some(4.0),
            );
            self.redraw(cx);
        }
    }

    fn handle_completed_mention_pill_click(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
        index: usize,
        path: &[LiveId],
    ) {
        if self.view.button(cx, path).clicked(actions) {
            let Some(pill) = self.local_mention_pills.get(index).cloned() else {
                return;
            };
            let current_text = self.text();
            let next_text = remove_completed_mention_token(&current_text, &pill.token);
            self.text_input(cx, ids!(persistent.center.text_input))
                .set_text(cx, &next_text);
            self.update_local_mention_preview(cx);
            enqueue_popup_notification(
                format!(
                    "Removed mention pill {} locally. No remote member lookup, SendMessage, SendAttachment, gateway/runtime/auth, or live mutation was emitted.",
                    pill.token
                ),
                PopupKind::Info,
                Some(4.0),
            );
            self.redraw(cx);
        }
    }

    fn handle_rich_directory_control_click(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
        action: &str,
        path: &[LiveId],
    ) {
        if self.view.button(cx, path).clicked(actions) {
            self.stage_mention_rich_directory_control(cx, action);
        }
    }

    fn handle_mention_preflight_detail_control_click(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
        action: &str,
        path: &[LiveId],
    ) {
        if self.view.button(cx, path).clicked(actions) {
            self.stage_telegram_mention_picker_preflight_detail_control(cx, action);
        }
    }

    fn handle_mention_payload_scope_control_click(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
        action: &str,
        path: &[LiveId],
    ) {
        if self.view.button(cx, path).clicked(actions) {
            self.stage_telegram_mention_payload_scope_control(cx, action);
        }
    }

    fn stage_mention_rich_directory_control(&mut self, cx: &mut Cx, action: &str) {
        let action = action.trim();
        if action.is_empty() {
            return;
        }
        self.mention_rich_directory_last_action = Some(action.to_string());
        let current_text = self.text();
        let active_query = active_mention_query(&current_text);
        let selected_token = self.selected_suggestion_token();
        let (label, action_detail) = if action == "Directory" {
            self.mention_directory_search_result = None;
            self.mention_directory_search_error = None;
            let directory_query = active_query
                .map(sanitize_user_directory_search_query)
                .unwrap_or_default();
            if directory_query.is_empty() {
                self.mention_directory_search_last_query = None;
                (
                    mention_picker_directory_search_unavailable_label(
                        active_query,
                        self.suggestion_count(),
                        selected_token.as_deref(),
                        self.cached_member_suggestions.len(),
                    ),
                    "needs an active @query; no request was submitted".to_string(),
                )
            } else {
                self.mention_directory_search_last_query = Some(directory_query.clone());
                submit_async_request(MatrixRequest::SearchUserDirectory {
                    query: directory_query.clone(),
                    limit: 8,
                });
                (
                    mention_picker_directory_search_request_label(
                        &directory_query,
                        active_query,
                        self.suggestion_count(),
                        selected_token.as_deref(),
                        self.cached_member_suggestions.len(),
                    ),
                    format!("submitted MatrixRequest::SearchUserDirectory for @{directory_query}"),
                )
            }
        } else if action == "Hover" {
            (
                mention_picker_hover_card_snapshot_label(
                    active_query,
                    self.suggestion_count(),
                    selected_token.as_deref(),
                    self.cached_member_suggestions.len(),
                    self.mention_directory_search_result.as_ref(),
                    self.selected_cached_member_suggestion(),
                    self.can_notify_room,
                ),
                "rendered a local hover-card snapshot from cached directory or loaded member metadata"
                    .to_string(),
            )
        } else if matches!(action, "Rich" | "Pills") {
            (
                mention_picker_rich_mention_packet_snapshot_label(
                    action,
                    active_query,
                    self.suggestion_count(),
                    selected_token.as_deref(),
                    self.cached_member_suggestions.len(),
                    self.can_notify_room,
                ),
                "rendered a local rich mention packet snapshot".to_string(),
            )
        } else {
            (
                mention_picker_rich_directory_controls_label(
                    Some(action),
                    active_query,
                    self.suggestion_count(),
                    selected_token.as_deref(),
                    self.cached_member_suggestions.len(),
                ),
                "stayed local".to_string(),
            )
        };
        self.view
            .label(
                cx,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_rich_popup_boundary
                ),
            )
            .set_text(cx, &label);
        enqueue_popup_notification(
            format!(
                "Mention {action} control {action_detail}. No profile/avatar fetch, pill editor mutation, attachment/edit payload, extra send, gateway/runtime/auth, or live mutation was emitted."
            ),
            PopupKind::Info,
            Some(4.0),
        );
        self.redraw(cx);
    }

    fn stage_telegram_mention_payload_scope_control(&mut self, cx: &mut Cx, action: &str) {
        let action = action.trim();
        if action.is_empty() {
            return;
        }
        self.mention_payload_scope_last_action = Some(action.to_string());
        let current_text = self.text();
        let active_query = active_mention_query(&current_text);
        let selected_token = self.selected_suggestion_token();
        let label = if action.eq_ignore_ascii_case("Packet") {
            mention_picker_payload_drilldown_packet_label(
                Some(action),
                active_query,
                self.suggestion_count(),
                selected_token.as_deref(),
                self.cached_member_suggestions.len(),
                self.can_notify_room,
            )
        } else if action.eq_ignore_ascii_case("Contract") {
            mention_picker_payload_typed_contract_packet_label(
                Some(action),
                active_query,
                self.suggestion_count(),
                selected_token.as_deref(),
                self.cached_member_suggestions.len(),
                self.can_notify_room,
            )
        } else if action.eq_ignore_ascii_case("Taxonomy") {
            mention_picker_remote_result_taxonomy_packet_label(
                Some(action),
                active_query,
                self.suggestion_count(),
                selected_token.as_deref(),
                self.cached_member_suggestions.len(),
                self.can_notify_room,
                self.mention_directory_search_result.as_ref(),
            )
        } else {
            mention_picker_payload_scope_controls_label(
                Some(action),
                active_query,
                self.suggestion_count(),
                selected_token.as_deref(),
                self.cached_member_suggestions.len(),
                self.can_notify_room,
            )
        };
        self.view
            .label(
                cx,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_payload_scope_metadata
                ),
            )
            .set_text(cx, &label);
        enqueue_popup_notification(
            format!(
                "Mention {action} payload scope stayed local. No attachment/edit/reply mention payload, remote directory lookup, extra SendMessage, SendAttachment, gateway/runtime/auth, or live mutation was emitted."
            ),
            PopupKind::Info,
            Some(4.0),
        );
        self.redraw(cx);
    }

    fn stage_telegram_mention_picker_preflight_detail_control(
        &mut self,
        cx: &mut Cx,
        action: &str,
    ) {
        let action = action.trim();
        if action.is_empty() {
            return;
        }
        self.mention_preflight_detail_last_action = Some(action.to_string());
        let current_text = self.text();
        let active_query = active_mention_query(&current_text);
        let selected_token = self.selected_suggestion_token();
        let label = mention_picker_preflight_detail_controls_label(
            Some(action),
            active_query,
            self.suggestion_count(),
            selected_token.as_deref(),
            self.cached_member_suggestions.len(),
            self.can_notify_room,
        );
        self.view
            .label(
                cx,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_preflight_detail_metadata
                ),
            )
            .set_text(cx, &label);
        enqueue_popup_notification(
            format!(
                "Mention {action} preflight detail stayed local. No remote directory search, profile/avatar fetch, attachment/edit payload, extra SendMessage, retry automation, gateway/runtime/auth, or live mutation was emitted."
            ),
            PopupKind::Info,
            Some(4.0),
        );
        self.redraw(cx);
    }

    fn update_member_suggestion_button(
        &mut self,
        cx: &mut Cx,
        path: &[LiveId],
        index: usize,
        should_show: bool,
    ) {
        let button = self.view.button(cx, path);
        let suggestion = self.cached_member_suggestions.get(index);
        button.set_visible(cx, should_show && suggestion.is_some());
        if let Some(suggestion) = suggestion {
            let global_index = self.cached_member_suggestion_global_index(index);
            button.set_text(
                cx,
                &self.suggestion_button_label(global_index, &suggestion.label),
            );
        }
    }

    fn update_directory_result_promotion_row(&mut self, cx: &mut Cx, active_query: bool) {
        let should_show = active_query
            && self
                .mention_directory_search_result
                .as_ref()
                .is_some_and(|result| !result.results.is_empty());
        self.view
            .view(
                cx,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_directory_result_promotion_row
                ),
            )
            .set_visible(cx, should_show);
        self.update_directory_result_promotion_button(
            cx,
            ids!(
                persistent
                    .bottom
                    .mention_preview
                    .mention_directory_result_promotion_row
                    .mention_directory_result_button_1
            ),
            0,
            should_show,
        );
        self.update_directory_result_promotion_button(
            cx,
            ids!(
                persistent
                    .bottom
                    .mention_preview
                    .mention_directory_result_promotion_row
                    .mention_directory_result_button_2
            ),
            1,
            should_show,
        );
        self.update_directory_result_promotion_button(
            cx,
            ids!(
                persistent
                    .bottom
                    .mention_preview
                    .mention_directory_result_promotion_row
                    .mention_directory_result_button_3
            ),
            2,
            should_show,
        );
    }

    fn update_directory_result_promotion_button(
        &mut self,
        cx: &mut Cx,
        path: &[LiveId],
        index: usize,
        should_show: bool,
    ) {
        let button = self.view.button(cx, path);
        let entry = self
            .mention_directory_search_result
            .as_ref()
            .and_then(|result| result.results.get(index));
        button.set_visible(cx, should_show && entry.is_some());
        if let Some(entry) = entry {
            button.set_text(
                cx,
                &mention_directory_result_promotion_button_label(index, entry),
            );
        }
    }

    fn update_local_pill_tray(&mut self, cx: &mut Cx, should_show: bool) {
        self.view
            .view(
                cx,
                ids!(persistent.bottom.mention_preview.mention_pill_tray),
            )
            .set_visible(cx, should_show);
        self.view
            .label(
                cx,
                ids!(
                    persistent
                        .bottom
                        .mention_preview
                        .mention_pill_tray
                        .mention_pill_tray_summary
                ),
            )
            .set_text(
                cx,
                &mention_local_pill_tray_label(&self.local_mention_pills),
            );
        self.update_local_pill_button(
            cx,
            ids!(
                persistent
                    .bottom
                    .mention_preview
                    .mention_pill_tray
                    .mention_pill_button_row
                    .mention_pill_button_1
            ),
            0,
            should_show,
        );
        self.update_local_pill_button(
            cx,
            ids!(
                persistent
                    .bottom
                    .mention_preview
                    .mention_pill_tray
                    .mention_pill_button_row
                    .mention_pill_button_2
            ),
            1,
            should_show,
        );
        self.update_local_pill_button(
            cx,
            ids!(
                persistent
                    .bottom
                    .mention_preview
                    .mention_pill_tray
                    .mention_pill_button_row
                    .mention_pill_button_3
            ),
            2,
            should_show,
        );
    }

    fn update_local_pill_button(
        &mut self,
        cx: &mut Cx,
        path: &[LiveId],
        index: usize,
        should_show: bool,
    ) {
        let button = self.view.button(cx, path);
        let pill = self.local_mention_pills.get(index);
        button.set_visible(cx, should_show && pill.is_some());
        if let Some(pill) = pill {
            button.set_text(cx, &pill.button_label);
        }
    }

    fn has_insertable_suggestion(&self) -> bool {
        active_mention_query(&self.text()).is_some() && self.suggestion_count() > 0
    }

    fn selected_suggestion_token(&self) -> Option<String> {
        let count = self.suggestion_count();
        if count == 0 {
            return None;
        }
        let selected_index = self.selected_suggestion_index.min(count - 1);
        if self.can_notify_room && selected_index == 0 {
            return Some("@room".to_string());
        }
        let member_index = selected_index.checked_sub(self.cached_member_suggestion_offset())?;
        self.cached_member_suggestions
            .get(member_index)
            .map(|suggestion| suggestion.token.clone())
    }

    fn directory_result_promotion_token(&self, index: usize) -> Option<String> {
        self.mention_directory_search_result
            .as_ref()?
            .results
            .get(index)
            .map(|entry| entry.user_id.as_str().to_string())
    }

    fn selected_cached_member_suggestion(&self) -> Option<&MentionSuggestion> {
        let count = self.suggestion_count();
        if count == 0 {
            return None;
        }
        let selected_index = self.selected_suggestion_index.min(count - 1);
        if self.can_notify_room && selected_index == 0 {
            return None;
        }
        let member_index = selected_index.checked_sub(self.cached_member_suggestion_offset())?;
        self.cached_member_suggestions.get(member_index)
    }

    fn cached_suggestion_status_text(&self, active_query: Option<&str>) -> String {
        let Some(query) = active_query else {
            return MENTION_PICKER_CACHED_SUGGESTION_LABEL.to_string();
        };
        let count = self.suggestion_count();
        if count == 0 {
            return mention_picker_lifecycle_metadata_label(
                "no cached match",
                Some(query),
                0,
                None,
                self.can_notify_room,
            );
        }
        let selected = self
            .selected_suggestion_token()
            .unwrap_or_else(|| "none".to_string());
        mention_picker_lifecycle_metadata_label(
            "active selection ready",
            Some(query),
            count,
            Some(&selected),
            self.can_notify_room,
        )
    }

    fn selected_suggestion_identity_preview(&self, active_query: Option<&str>) -> String {
        let Some(query) = active_query else {
            return MENTION_PICKER_LOADED_IDENTITY_LABEL.to_string();
        };
        let count = self.suggestion_count();
        let query_label = if query.is_empty() {
            "@".to_string()
        } else {
            format!("@{query}")
        };
        if count == 0 {
            return format!(
                "No loaded member identity for {query_label}; remote member lookup is not started."
            );
        }
        let selected_index = self.selected_suggestion_index.min(count - 1);
        if self.can_notify_room && selected_index == 0 {
            return "Selected @room uses loaded power-level allowance; no member identity lookup is started.".to_string();
        }
        let Some(member_index) = selected_index.checked_sub(self.cached_member_suggestion_offset())
        else {
            return MENTION_PICKER_LOADED_IDENTITY_LABEL.to_string();
        };
        self.cached_member_suggestions
            .get(member_index)
            .map(|suggestion| suggestion.identity_label.clone())
            .unwrap_or_else(|| MENTION_PICKER_LOADED_IDENTITY_LABEL.to_string())
    }

    fn local_candidate_rows_preview(&self, active_query: Option<&str>) -> String {
        let mut rows = Vec::new();
        if self.can_notify_room {
            rows.push(format_local_mention_candidate_row(
                0,
                self.selected_suggestion_index == 0,
                "@room",
                "room-wide mention - power level eligible - source power levels",
            ));
        }
        for (member_index, suggestion) in self.cached_member_suggestions.iter().enumerate() {
            let global_index = self.cached_member_suggestion_global_index(member_index);
            rows.push(format_local_mention_candidate_row(
                global_index,
                self.selected_suggestion_index == global_index,
                &suggestion.token,
                &suggestion.candidate_row_label,
            ));
        }
        mention_local_candidate_rows_label(
            active_query,
            self.suggestion_count(),
            self.selected_suggestion_token().as_deref(),
            self.can_notify_room,
            &rows,
        )
    }

    fn local_duplicate_hints_preview(&self, active_query: Option<&str>) -> String {
        mention_local_duplicate_hints_label(
            active_query,
            self.cached_member_suggestions.len(),
            self.selected_suggestion_token().as_deref(),
            &self.cached_member_suggestions,
        )
    }

    fn move_selected_suggestion(&mut self, cx: &mut Cx, delta: isize) {
        self.selected_suggestion_index = next_suggestion_index(
            self.selected_suggestion_index,
            self.suggestion_count(),
            delta,
        );
        self.update_local_mention_preview(cx);
        self.redraw(cx);
    }

    fn suggestion_count(&self) -> usize {
        self.cached_member_suggestion_offset() + self.cached_member_suggestions.len()
    }

    fn cached_member_suggestion_offset(&self) -> usize {
        if self.can_notify_room { 1 } else { 0 }
    }

    fn cached_member_suggestion_global_index(&self, member_index: usize) -> usize {
        self.cached_member_suggestion_offset() + member_index
    }

    fn clamp_selected_suggestion_index(&mut self) {
        let count = self.suggestion_count();
        self.selected_suggestion_index = if count == 0 {
            0
        } else {
            self.selected_suggestion_index.min(count - 1)
        };
    }

    fn suggestion_button_label(&self, global_index: usize, label: &str) -> String {
        if self.selected_suggestion_index == global_index {
            format!("> {label}")
        } else {
            label.to_string()
        }
    }

    /// Sets whether the current user can notify the entire room (@room mention).
    pub fn set_can_notify_room(&mut self, can_notify: bool) {
        self.can_notify_room = can_notify;
    }

    /// Gets whether the current user can notify the entire room (@room mention).
    pub fn can_notify_room(&self) -> bool {
        self.can_notify_room
    }
}

impl MentionableTextInputRef {
    /// Returns a reference to the inner `TextInput` widget.
    pub fn text_input_ref(&self) -> TextInputRef {
        self.child_by_path(ids!(persistent.center.text_input))
            .as_text_input()
    }

    /// Sets whether the current user can notify the entire room (@room mention).
    pub fn set_can_notify_room(&self, can_notify: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_can_notify_room(can_notify);
        }
    }

    /// Gets whether the current user can notify the entire room (@room mention).
    pub fn can_notify_room(&self) -> bool {
        self.borrow().is_some_and(|inner| inner.can_notify_room())
    }

    /// Updates the local suggestion row from already loaded room members.
    pub fn update_cached_member_suggestions(
        &self,
        cx: &mut Cx,
        room_members: Option<&[RoomMember]>,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.cached_member_lookup = cached_mention_member_lookup(room_members);
            inner.cached_member_suggestions =
                cached_member_suggestions(&inner.text(), room_members);
            inner.update_local_mention_preview(cx);
        }
    }

    /// Creates a message from the entered text and attaches compact Matrix mentions.
    ///
    /// This handles `/html` and `/plain` prefixes and only uses the already
    /// loaded member cache passed down from RoomScreen.
    pub fn create_message_with_mentions(
        &self,
        entered_text: &str,
        room_members: Option<&[RoomMember]>,
    ) -> RoomMessageEventContent {
        let mut message = if let Some(html_text) = entered_text.strip_prefix("/html") {
            RoomMessageEventContent::text_html(html_text, html_text)
        } else if let Some(plain_text) = entered_text.strip_prefix("/plain") {
            RoomMessageEventContent::text_plain(plain_text)
        } else {
            RoomMessageEventContent::text_markdown(entered_text)
        };

        if let Some(mentions) =
            extract_mentions_from_loaded_members(entered_text, room_members, self.can_notify_room())
        {
            message = message.add_mentions(mentions);
        }
        message
    }

    /// Builds the compact Matrix Mentions payload for non-text message paths
    /// that can carry a separate `m.mentions` field, such as media captions.
    pub fn mentions_for_text(
        &self,
        entered_text: &str,
        room_members: Option<&[RoomMember]>,
    ) -> Option<Mentions> {
        extract_mentions_from_loaded_members(entered_text, room_members, self.can_notify_room())
    }

    pub fn send_payload_metadata_label(
        &self,
        entered_text: &str,
        room_members: Option<&[RoomMember]>,
    ) -> String {
        mention_send_payload_metadata_label(entered_text, room_members, self.can_notify_room())
    }
}

#[derive(Clone, Debug)]
struct MentionSuggestion {
    label: String,
    token: String,
    identity_label: String,
    candidate_row_label: String,
    display_duplicate_count: usize,
    display_duplicate_key: Option<String>,
}

#[derive(Clone, Debug)]
struct CachedMentionMember {
    display_name: Option<String>,
    user_id: OwnedUserId,
    avatar_present: bool,
}

#[derive(Clone, Debug)]
struct MentionPill {
    token: String,
    button_label: String,
    detail_label: String,
}

#[derive(Debug, Default)]
struct MentionPayloadMetadata {
    mention_token_count: usize,
    literal_user_id_token_count: usize,
    cached_member_match_token_count: usize,
    unmatched_local_token_count: usize,
    room_token_requested: bool,
    room_flag_enabled: bool,
    user_ids: Vec<OwnedUserId>,
}

fn extract_mentions_from_loaded_members(
    entered_text: &str,
    room_members: Option<&[RoomMember]>,
    can_notify_room: bool,
) -> Option<Mentions> {
    let metadata = collect_mention_payload_metadata(entered_text, room_members, can_notify_room);

    if metadata.user_ids.is_empty() && !metadata.room_flag_enabled {
        return None;
    }

    let mut mentions = Mentions::with_user_ids(metadata.user_ids);
    mentions.room = metadata.room_flag_enabled;
    Some(mentions)
}

pub fn mention_send_payload_metadata_label(
    entered_text: &str,
    room_members: Option<&[RoomMember]>,
    can_notify_room: bool,
) -> String {
    let metadata = collect_mention_payload_metadata(entered_text, room_members, can_notify_room);
    let format_label = if entered_text.starts_with("/html") {
        "/html"
    } else if entered_text.starts_with("/plain") {
        "/plain"
    } else {
        "markdown"
    };
    let room_flag = if metadata.room_flag_enabled {
        "enabled"
    } else if metadata.room_token_requested {
        "blocked by power level"
    } else {
        "not requested"
    };
    let cache_source = room_members
        .map(|members| format!("loaded room_members: {}", members.len()))
        .unwrap_or_else(|| "loaded room_members: unavailable".to_string());

    format!(
        "Mention send payload metadata. Format: {format_label}. Scanned @tokens: {tokens}. Matrix mention users: {users}. Literal Matrix user-id tokens: {literal}. Cached member matches: {cached}. Unmatched local tokens: {unmatched}. @room flag: {room_flag}. Cache source: {cache_source}. Existing RoomInputBar SendMessage attaches add_mentions once; no remote member lookup, profile/avatar fetch, popup search, rich popup highlight styling, pill editor, disambiguation UI, attachment/edit mention payload, extra SendMessage, SendAttachment, typing notice, room-state, membership, account/profile, gateway/runtime/auth, or live mutation. {MENTION_PICKER_SEND_PAYLOAD_METADATA_LABEL}",
        tokens = metadata.mention_token_count,
        users = metadata.user_ids.len(),
        literal = metadata.literal_user_id_token_count,
        cached = metadata.cached_member_match_token_count,
        unmatched = metadata.unmatched_local_token_count,
    )
}

fn collect_mention_payload_metadata(
    entered_text: &str,
    room_members: Option<&[RoomMember]>,
    can_notify_room: bool,
) -> MentionPayloadMetadata {
    let mut metadata = MentionPayloadMetadata::default();

    for token in mention_tokens(entered_text) {
        metadata.mention_token_count += 1;
        let trimmed = trim_mention_token(token);
        if trimmed.is_empty() {
            metadata.unmatched_local_token_count += 1;
            continue;
        }
        if trimmed == "@room" {
            metadata.room_token_requested = true;
            metadata.room_flag_enabled |= can_notify_room;
            continue;
        }

        if let Ok(user_id) = UserId::parse(trimmed) {
            metadata.literal_user_id_token_count += 1;
            push_unique_user_id(&mut metadata.user_ids, user_id.to_owned());
            continue;
        }

        if let Some(member) = find_cached_member_for_mention(trimmed, room_members) {
            metadata.cached_member_match_token_count += 1;
            push_unique_user_id(&mut metadata.user_ids, member.user_id().to_owned());
            continue;
        }

        metadata.unmatched_local_token_count += 1;
    }

    metadata
}

fn trim_mention_token(token: &str) -> &str {
    token
        .trim_matches(|ch: char| {
            matches!(
                ch,
                ',' | '.'
                    | ';'
                    | ':'
                    | '!'
                    | '?'
                    | ')'
                    | '('
                    | '['
                    | ']'
                    | '{'
                    | '}'
                    | '<'
                    | '>'
                    | '"'
                    | '\''
            )
        })
        .trim()
}

fn mention_tokens(entered_text: &str) -> impl Iterator<Item = &str> {
    entered_text.split_whitespace().filter(|token| {
        token
            .trim_start_matches(|ch: char| matches!(ch, '(' | '[' | '{' | '<' | '"'))
            .starts_with('@')
    })
}

fn find_cached_member_for_mention<'a>(
    mention: &str,
    room_members: Option<&'a [RoomMember]>,
) -> Option<&'a RoomMember> {
    let members = room_members?;
    let wanted = mention.trim_start_matches('@');
    members.iter().find(|member| {
        member
            .display_name()
            .is_some_and(|display_name| display_name == wanted)
            || member.user_id().localpart() == wanted
    })
}

fn cached_member_suggestions(
    entered_text: &str,
    room_members: Option<&[RoomMember]>,
) -> Vec<MentionSuggestion> {
    let Some(query) = active_mention_query(entered_text) else {
        return Vec::new();
    };
    let query_lower = query.to_ascii_lowercase();
    let Some(room_members) = room_members else {
        return Vec::new();
    };
    let duplicate_display_counts = duplicate_display_name_counts(room_members, &query_lower);
    room_members
        .iter()
        .filter_map(|member| {
            let display_name = member.display_name();
            let localpart = member.user_id().localpart();
            if !member_matches_mention_query(member, &query_lower) {
                return None;
            }

            let token_source = display_name
                .filter(|name| !name.trim().is_empty() && !name.chars().any(char::is_whitespace))
                .unwrap_or(localpart);
            let display_duplicate_key = display_name.and_then(normalized_display_name_key);
            let duplicate_count = display_duplicate_key
                .as_ref()
                .and_then(|display_key| {
                    duplicate_display_counts
                        .iter()
                        .find(|(candidate_key, _)| candidate_key == display_key)
                        .map(|(_, count)| *count)
                })
                .unwrap_or(0);
            Some(MentionSuggestion {
                label: format!("@{token_source}"),
                token: format!("@{token_source}"),
                identity_label: loaded_member_identity_label(member),
                candidate_row_label: loaded_member_candidate_row_label(
                    member,
                    token_source,
                    duplicate_count,
                ),
                display_duplicate_count: duplicate_count,
                display_duplicate_key,
            })
        })
        .take(MAX_CACHED_MEMBER_SUGGESTIONS)
        .collect()
}

fn cached_mention_member_lookup(room_members: Option<&[RoomMember]>) -> Vec<CachedMentionMember> {
    room_members
        .unwrap_or_default()
        .iter()
        .map(|member| CachedMentionMember {
            display_name: member.display_name().map(ToOwned::to_owned),
            user_id: member.user_id().to_owned(),
            avatar_present: member.avatar_url().is_some(),
        })
        .collect()
}

fn member_matches_mention_query(member: &RoomMember, query_lower: &str) -> bool {
    let display_matches = member
        .display_name()
        .is_some_and(|name| name.to_ascii_lowercase().contains(query_lower));
    let localpart_matches = member
        .user_id()
        .localpart()
        .to_ascii_lowercase()
        .contains(query_lower);
    display_matches || localpart_matches
}

fn duplicate_display_name_counts(
    room_members: &[RoomMember],
    query_lower: &str,
) -> Vec<(String, usize)> {
    let mut counts = Vec::<(String, usize)>::new();
    for display_key in room_members
        .iter()
        .filter(|member| member_matches_mention_query(member, query_lower))
        .filter_map(|member| member.display_name().and_then(normalized_display_name_key))
    {
        if let Some((_, count)) = counts
            .iter_mut()
            .find(|(candidate_key, _)| candidate_key == &display_key)
        {
            *count += 1;
        } else {
            counts.push((display_key, 1));
        }
    }
    counts
}

fn normalized_display_name_key(display_name: &str) -> Option<String> {
    let display_name = display_name.trim();
    if display_name.is_empty() {
        None
    } else {
        Some(display_name.to_ascii_lowercase())
    }
}

fn loaded_member_identity_label(member: &RoomMember) -> String {
    format_loaded_mention_identity(
        member.display_name(),
        member.user_id(),
        member.avatar_url().is_some(),
    )
}

fn loaded_member_candidate_row_label(
    member: &RoomMember,
    token_source: &str,
    display_duplicate_count: usize,
) -> String {
    format_local_mention_candidate_member(
        token_source,
        member.display_name(),
        member.user_id(),
        member.avatar_url().is_some(),
        display_duplicate_count,
    )
}

fn completed_mention_pills_for_text(
    entered_text: &str,
    cached_members: &[CachedMentionMember],
    can_notify_room: bool,
) -> Vec<MentionPill> {
    completed_mention_tokens(entered_text)
        .into_iter()
        .take(3)
        .map(|token| mention_pill_for_token(token, cached_members, can_notify_room))
        .collect()
}

fn completed_mention_tokens(entered_text: &str) -> Vec<&str> {
    let tokens: Vec<&str> = entered_text.split_whitespace().collect();
    let last_index = tokens.len().saturating_sub(1);
    let ends_with_whitespace = entered_text.chars().last().is_some_and(char::is_whitespace);

    tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            let trimmed = trim_mention_token(token);
            if trimmed.is_empty() || !trimmed.starts_with('@') {
                return None;
            }
            if !ends_with_whitespace && index == last_index {
                return None;
            }
            Some(trimmed)
        })
        .collect()
}

fn mention_pill_for_token(
    token: &str,
    cached_members: &[CachedMentionMember],
    can_notify_room: bool,
) -> MentionPill {
    let token = token.trim();
    let detail_label = if token == "@room" {
        if can_notify_room {
            "@room will set the Matrix room mention flag on send".to_string()
        } else {
            "@room is visible but blocked by the current power level".to_string()
        }
    } else if let Ok(user_id) = UserId::parse(token) {
        format!("literal Matrix user id {user_id} will be included in m.mentions")
    } else if let Some(member) = find_cached_mention_member_for_token(token, cached_members) {
        let display_label = member
            .display_name
            .as_deref()
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .unwrap_or("display name unavailable");
        let avatar_label = if member.avatar_present {
            "avatar mxc present"
        } else {
            "avatar mxc none"
        };
        format!(
            "loaded member {display_label} -> {} ({avatar_label}) will be included in m.mentions",
            member.user_id
        )
    } else {
        "unmatched local token; visible for cleanup but not included in m.mentions".to_string()
    };

    MentionPill {
        token: token.to_string(),
        button_label: format!("Remove {token}"),
        detail_label,
    }
}

fn find_cached_mention_member_for_token<'a>(
    token: &str,
    cached_members: &'a [CachedMentionMember],
) -> Option<&'a CachedMentionMember> {
    let wanted = token.trim_start_matches('@');
    cached_members.iter().find(|member| {
        member
            .display_name
            .as_deref()
            .is_some_and(|display_name| display_name == wanted)
            || member.user_id.localpart() == wanted
    })
}

fn mention_local_pill_tray_label(pills: &[MentionPill]) -> String {
    let tray_state = if pills.is_empty() {
        "no completed mention pills".to_string()
    } else {
        pills
            .iter()
            .enumerate()
            .map(|(index, pill)| {
                format!("pill {} {} - {}", index + 1, pill.token, pill.detail_label)
            })
            .collect::<Vec<_>>()
            .join(" | ")
    };

    format!(
        "Mention local pill tray. {tray_state}. Clicking a visible pill removes only that completed @token from composer text and recomputes the existing send-time Mentions payload preview. No remote member lookup, server-side directory search, profile/avatar fetch, duplicate-name disambiguation, SendMessage, SendAttachment, edit/reply mention payload rewrite, gateway/runtime/auth, or live mutation is emitted. {MENTION_PICKER_LOCAL_PILL_TRAY_LIVE_LABEL}"
    )
}

fn remove_completed_mention_token(entered_text: &str, token_to_remove: &str) -> String {
    let mut removed = false;
    let ends_with_whitespace = entered_text.chars().last().is_some_and(char::is_whitespace);
    let next_tokens = entered_text
        .split_whitespace()
        .filter(|token| {
            if removed {
                return true;
            }
            if trim_mention_token(token) == token_to_remove {
                removed = true;
                return false;
            }
            true
        })
        .collect::<Vec<_>>();

    let mut next_text = next_tokens.join(" ");
    if ends_with_whitespace && !next_text.is_empty() {
        next_text.push(' ');
    }
    next_text
}

fn format_loaded_mention_identity(
    display_name: Option<&str>,
    user_id: &UserId,
    avatar_present: bool,
) -> String {
    let display_name_label = display_name
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .unwrap_or("display name unavailable");
    let avatar_label = if avatar_present {
        "avatar mxc: present"
    } else {
        "avatar mxc: none"
    };
    format!(
        "Loaded member identity: display {display_name_label} | user id {user_id} | localpart @{} | {avatar_label}",
        user_id.localpart()
    )
}

fn format_local_mention_candidate_member(
    token_source: &str,
    display_name: Option<&str>,
    user_id: &UserId,
    avatar_present: bool,
    display_duplicate_count: usize,
) -> String {
    let display_name_label = display_name
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .unwrap_or("display name unavailable");
    let avatar_label = if avatar_present {
        "avatar mxc present"
    } else {
        "avatar mxc none"
    };
    let duplicate_label = if display_duplicate_count > 1 {
        format!("duplicate display rows {display_duplicate_count}")
    } else {
        "duplicate display rows unique".to_string()
    };
    format!(
        "token @{token_source} - display {display_name_label} - user id {user_id} - localpart @{} - {avatar_label} - {duplicate_label} - source loaded room_members",
        user_id.localpart()
    )
}

fn format_local_mention_candidate_row(
    zero_based_index: usize,
    selected: bool,
    token: &str,
    detail: &str,
) -> String {
    let selected_label = if selected { "selected" } else { "available" };
    format!(
        "row {} {selected_label} {token} - {detail}",
        zero_based_index + 1
    )
}

fn mention_local_candidate_rows_label(
    active_query: Option<&str>,
    suggestion_count: usize,
    selected_token: Option<&str>,
    can_notify_room: bool,
    rows: &[String],
) -> String {
    let query_label = match active_query.map(str::trim) {
        Some("") => "@".to_string(),
        Some(query) => format!("@{query}"),
        None => "no active @query".to_string(),
    };
    let selected_token = selected_token
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .unwrap_or("none");
    let room_status = if can_notify_room {
        "@room row eligible"
    } else {
        "@room row unavailable"
    };
    let rows_label = if rows.is_empty() {
        "no cached candidate rows".to_string()
    } else {
        rows.join(" | ")
    };

    format!(
        "Mention local candidate rows. Active query: {query_label}. Cached suggestion count: {suggestion_count}. Selected token: {selected_token}. {room_status}. Rows: {rows_label}. Candidate rows are built only from @room power-level state and up to three loaded room_members cache matches. No remote member lookup, server-side directory search, profile/avatar fetch, duplicate-name disambiguation, rich popup search, pill editor, attachment/edit mention payload, SendAttachment, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, retry automation, or live mutation. {MENTION_PICKER_LOCAL_CANDIDATE_ROWS_LABEL}"
    )
}

fn mention_local_duplicate_hints_label(
    active_query: Option<&str>,
    cached_candidate_count: usize,
    selected_token: Option<&str>,
    suggestions: &[MentionSuggestion],
) -> String {
    let query_label = match active_query.map(str::trim) {
        Some("") => "@".to_string(),
        Some(query) => format!("@{query}"),
        None => "no active @query".to_string(),
    };
    let selected_token = selected_token
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .unwrap_or("none");
    let mut duplicate_group_keys = Vec::<&str>::new();
    for suggestion in suggestions
        .iter()
        .filter(|suggestion| suggestion.display_duplicate_count > 1)
    {
        let Some(display_key) = suggestion.display_duplicate_key.as_deref() else {
            continue;
        };
        if !duplicate_group_keys.contains(&display_key) {
            duplicate_group_keys.push(display_key);
        }
    }
    let duplicate_group_count = duplicate_group_keys.len();
    let selected_duplicate_count = suggestions
        .iter()
        .find(|suggestion| suggestion.token == selected_token)
        .map(|suggestion| suggestion.display_duplicate_count)
        .unwrap_or(0);
    let selected_hint = if selected_duplicate_count > 1 {
        format!("selected display collision rows: {selected_duplicate_count}")
    } else if selected_token == "@room" {
        "selected token is @room; duplicate member display names do not apply".to_string()
    } else {
        "selected display collision rows: none".to_string()
    };
    let duplicate_hint = if duplicate_group_count == 0 {
        "duplicate display-name groups in cached rows: none".to_string()
    } else {
        format!("duplicate display-name groups in cached rows: {duplicate_group_count}")
    };

    format!(
        "Mention local duplicate hints. Active query: {query_label}. Cached member candidates: {cached_candidate_count}. Selected token: {selected_token}. {duplicate_hint}. {selected_hint}. Localpart and Matrix user id stay visible as the local disambiguation clues before insertion. No remote member lookup, server-side directory search, profile/avatar fetch, rich duplicate-name disambiguation UI, hover card, pill editor, attachment/edit mention payload, SendAttachment, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, retry automation, or live mutation. {MENTION_PICKER_LOCAL_DUPLICATE_HINTS_LABEL}"
    )
}

fn mention_picker_lifecycle_metadata_label(
    lifecycle_state: &str,
    active_query: Option<&str>,
    suggestion_count: usize,
    selected_token: Option<&str>,
    can_notify_room: bool,
) -> String {
    let lifecycle_state = lifecycle_state.trim();
    let lifecycle_state = if lifecycle_state.is_empty() {
        "status updated"
    } else {
        lifecycle_state
    };
    let query_label = match active_query.map(str::trim) {
        Some("") => "@".to_string(),
        Some(query) => format!("@{query}"),
        None => "no active @query".to_string(),
    };
    let selected_token = selected_token
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .unwrap_or("none");
    let room_status = if can_notify_room {
        "@room eligible"
    } else {
        "@room unavailable"
    };

    format!(
        "Mention lifecycle {lifecycle_state}. Active query: {query_label}. Cached suggestion count: {suggestion_count}. Selected token: {selected_token}. {room_status}. ArrowUp/ArrowDown, Tab, Enter, or click only edits the active token; trailing space releases Enter to SendMessage. {MENTION_PICKER_LIFECYCLE_METADATA_LABEL}"
    )
}

#[allow(dead_code)]
fn mention_picker_rich_popup_boundary_label(
    active_query: Option<&str>,
    suggestion_count: usize,
    selected_token: Option<&str>,
) -> String {
    let query_label = match active_query.map(str::trim) {
        Some("") => "@".to_string(),
        Some(query) => format!("@{query}"),
        None => "no active @query".to_string(),
    };
    let selected_token = selected_token
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .unwrap_or("none");
    format!(
        "Rich mention picker boundary. Active query: {query_label}. Cached suggestion count: {suggestion_count}. Selected token: {selected_token}. No floating popup search, rich highlight styling, pill editor, disambiguation UI, remote member lookup, profile/avatar fetch, attachment/edit mention payload editor, membership, gateway/runtime/auth, or live mutation. {MENTION_PICKER_RICH_POPUP_BOUNDARY_LABEL}"
    )
}

#[allow(dead_code)]
fn mention_picker_directory_disambiguation_boundary_label(
    active_query: Option<&str>,
    suggestion_count: usize,
    selected_token: Option<&str>,
    loaded_member_cache_size: usize,
) -> String {
    let query_label = match active_query.map(str::trim) {
        Some("") => "@".to_string(),
        Some(query) => format!("@{query}"),
        None => "no active @query".to_string(),
    };
    let selected_token = selected_token
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .unwrap_or("none");
    format!(
        "Mention directory/disambiguation boundary. Active query: {query_label}. Cached suggestion count: {suggestion_count}. Selected token: {selected_token}. Loaded member cache rows: {loaded_member_cache_size}. Directory can read Matrix user-directory metadata through MatrixRequest::SearchUserDirectory/client.search_users; duplicate display-name disambiguation UI, remote profile hover cards, avatar/profile fetch beyond directory response fields, rich highlighted popup results, multi-select mention tray, pill editor, attachment/edit mention payload editor, room-state, membership, gateway/runtime/auth, and live mutation stay local blocked. {MENTION_PICKER_DIRECTORY_DISAMBIGUATION_BOUNDARY_LABEL}"
    )
}

fn mention_picker_rich_directory_controls_label(
    action: Option<&str>,
    active_query: Option<&str>,
    suggestion_count: usize,
    selected_token: Option<&str>,
    loaded_member_cache_size: usize,
) -> String {
    let action = action
        .map(str::trim)
        .filter(|action| !action.is_empty())
        .unwrap_or("ready");
    let query_label = match active_query.map(str::trim) {
        Some("") => "@".to_string(),
        Some(query) => format!("@{query}"),
        None => "no active @query".to_string(),
    };
    let selected_token = selected_token
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .unwrap_or("none");
    format!(
        "Mention rich/directory control {action}. Active query: {query_label}. Cached suggestion count: {suggestion_count}. Selected token: {selected_token}. Loaded member cache rows: {loaded_member_cache_size}. Directory uses MatrixRequest::SearchUserDirectory/client.search_users when the active @query is non-empty, and Hover can render a local cached/directory hover-card snapshot; Rich popup search, duplicate-name disambiguation, remote profile hover cards, avatar/profile fetch beyond directory response fields, highlighted result list, multi-select tray, pill editor, attachment/edit mention payloads, extra SendMessage, SendAttachment, room-state, membership, account/profile, gateway/runtime/auth, and live mutation stay local blocked. {MENTION_PICKER_RICH_DIRECTORY_CONTROLS_LABEL}"
    )
}

fn mention_picker_hover_card_snapshot_label(
    active_query: Option<&str>,
    suggestion_count: usize,
    selected_token: Option<&str>,
    loaded_member_cache_size: usize,
    directory_result: Option<&UserDirectorySearchResult>,
    cached_suggestion: Option<&MentionSuggestion>,
    can_notify_room: bool,
) -> String {
    let query_label = match active_query.map(str::trim) {
        Some("") => "@".to_string(),
        Some(query) => format!("@{query}"),
        None => "no active @query".to_string(),
    };
    let selected_token = selected_token
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .unwrap_or("none");
    let snapshot = if let Some(result) = directory_result {
        if result.results.is_empty() {
            format!(
                "directory result source for @{} returned no rows; limited {}",
                result.query, result.limited
            )
        } else {
            let rows = result
                .results
                .iter()
                .take(3)
                .enumerate()
                .map(|(index, entry)| {
                    let display_name = entry
                        .display_name
                        .as_deref()
                        .map(str::trim)
                        .filter(|name| !name.is_empty())
                        .unwrap_or("display name unavailable");
                    let avatar_state = if entry.avatar_url.is_some() {
                        "avatar mxc present"
                    } else {
                        "avatar mxc missing"
                    };
                    format!(
                        "directory row {} user id {} display {} {}",
                        index + 1,
                        entry.user_id,
                        display_name,
                        avatar_state
                    )
                })
                .collect::<Vec<_>>()
                .join(" | ");
            format!(
                "directory result source for @{} rows {} limited {}: {}",
                result.query,
                result.results.len(),
                result.limited,
                rows
            )
        }
    } else if let Some(suggestion) = cached_suggestion {
        format!(
            "loaded member source selected token {} identity [{}] candidate [{}]",
            suggestion.token, suggestion.identity_label, suggestion.candidate_row_label
        )
    } else if selected_token == "@room" {
        let room_state = if can_notify_room {
            "@room power-level hover snapshot eligible"
        } else {
            "@room power-level hover snapshot blocked"
        };
        format!("{room_state}; no member row needed")
    } else {
        "no cached directory or loaded member source for hover snapshot".to_string()
    };

    format!(
        "Mention Hover local card snapshot. Active query: {query_label}. Cached suggestion count: {suggestion_count}. Selected token: {selected_token}. Loaded member cache rows: {loaded_member_cache_size}. Snapshot: {snapshot}. This uses only cached user-directory result metadata, loaded RoomMember suggestion metadata, or @room power-level state. No MatrixRequest::SearchUserDirectory, profile/avatar fetch, remote hover-card request, duplicate-name disambiguation workflow, rich popup search, multi-select tray mutation, pill editor mutation, SendMessage, SendAttachment, room-state, membership, account/profile, gateway/runtime/auth, retry automation, or live mutation. {MENTION_PICKER_HOVER_CARD_SNAPSHOT_LIVE_LABEL}"
    )
}

fn mention_picker_directory_search_unavailable_label(
    active_query: Option<&str>,
    suggestion_count: usize,
    selected_token: Option<&str>,
    loaded_member_cache_size: usize,
) -> String {
    let query_label = match active_query.map(str::trim) {
        Some("") => "@".to_string(),
        Some(query) => format!("@{query}"),
        None => "no active @query".to_string(),
    };
    let selected_token = selected_token
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .unwrap_or("none");
    format!(
        "Mention Directory live Matrix user-directory search needs an active @query. Active query: {query_label}. Cached suggestion count: {suggestion_count}. Selected token: {selected_token}. Loaded member cache rows: {loaded_member_cache_size}. No MatrixRequest::SearchUserDirectory, SendMessage, SendAttachment, room-state, membership, gateway/runtime/auth, or live mutation was submitted. {MENTION_PICKER_RICH_DIRECTORY_CONTROLS_LABEL}"
    )
}

fn mention_picker_directory_search_request_label(
    query: &str,
    active_query: Option<&str>,
    suggestion_count: usize,
    selected_token: Option<&str>,
    loaded_member_cache_size: usize,
) -> String {
    let active_query_label = match active_query.map(str::trim) {
        Some("") => "@".to_string(),
        Some(query) => format!("@{query}"),
        None => "no active @query".to_string(),
    };
    let selected_token = selected_token
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .unwrap_or("none");
    format!(
        "Mention Directory live Matrix user-directory search requested for @{query}. Active query: {active_query_label}. Cached suggestion count: {suggestion_count}. Selected token: {selected_token}. Loaded member cache rows: {loaded_member_cache_size}. Submitted MatrixRequest::SearchUserDirectory as a read-only client.search_users request; result rows can later expose bounded local promotion buttons without auto-insert. No SendMessage, SendAttachment, profile/avatar fetch beyond directory response fields, pill editor mutation, attachment/edit payload, room-state, membership, gateway/runtime/auth, or live mutation was submitted. {MENTION_PICKER_RICH_DIRECTORY_CONTROLS_LABEL}"
    )
}

fn mention_picker_directory_search_error_label(
    requested_query: Option<&str>,
    active_query: Option<&str>,
    error: &str,
    suggestion_count: usize,
    selected_token: Option<&str>,
    loaded_member_cache_size: usize,
) -> String {
    let requested_query = requested_query
        .map(str::trim)
        .filter(|query| !query.is_empty())
        .unwrap_or("unknown");
    let active_query_label = match active_query.map(str::trim) {
        Some("") => "@".to_string(),
        Some(query) => format!("@{query}"),
        None => "no active @query".to_string(),
    };
    let selected_token = selected_token
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .unwrap_or("none");
    format!(
        "Mention Directory live Matrix user-directory search failed for @{requested_query}. Active query: {active_query_label}. Cached suggestion count: {suggestion_count}. Selected token: {selected_token}. Loaded member cache rows: {loaded_member_cache_size}. Error: {error}. Existing cached suggestions and SendMessage/add_mentions handoff stay available; no SendMessage, SendAttachment, room-state, membership, gateway/runtime/auth, retry automation, or live mutation was submitted. {MENTION_PICKER_RICH_DIRECTORY_CONTROLS_LABEL}"
    )
}

fn mention_picker_directory_search_result_label(
    result: &UserDirectorySearchResult,
    suggestion_count: usize,
    selected_token: Option<&str>,
    loaded_member_cache_size: usize,
) -> String {
    let selected_token = selected_token
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .unwrap_or("none");
    let result_state = if result.results.is_empty() {
        "no remote rows".to_string()
    } else {
        result
            .results
            .iter()
            .take(3)
            .map(|entry| {
                let display_name = entry
                    .display_name
                    .as_deref()
                    .map(str::trim)
                    .filter(|name| !name.is_empty())
                    .unwrap_or("no display name");
                let avatar_state = if entry.avatar_url.is_some() {
                    "avatar mxc present"
                } else {
                    "avatar mxc missing"
                };
                format!("{} ({display_name}, {avatar_state})", entry.user_id)
            })
            .collect::<Vec<_>>()
            .join("; ")
    };
    format!(
        "Mention Directory live Matrix user-directory result for @{}. Result count: {}. Limited: {}. First rows: {result_state}. Cached suggestion count: {suggestion_count}. Selected token: {selected_token}. Loaded member cache rows: {loaded_member_cache_size}. Results are read-only metadata from client.search_users; visible Directory result buttons can promote a literal Matrix user-id token locally, but search completion does not insert tokens automatically. No SendMessage, SendAttachment, profile/avatar fetch beyond directory response fields, pill editor mutation, attachment/edit payload, room-state, membership, gateway/runtime/auth, or live mutation was submitted. {MENTION_PICKER_RICH_DIRECTORY_CONTROLS_LABEL}",
        result.query,
        result.results.len(),
        result.limited,
    )
}

fn mention_directory_result_promotion_button_label(
    index: usize,
    entry: &UserDirectorySearchEntry,
) -> String {
    let display_name = entry
        .display_name
        .as_deref()
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .unwrap_or("Directory user");
    format!("{} {} {}", index + 1, display_name, entry.user_id)
}

fn mention_picker_rich_mention_packet_snapshot_label(
    action: &str,
    active_query: Option<&str>,
    suggestion_count: usize,
    selected_token: Option<&str>,
    loaded_member_cache_size: usize,
    can_notify_room: bool,
) -> String {
    let action = action.trim();
    let action = if action.is_empty() { "Rich" } else { action };
    let query_label = match active_query.map(str::trim) {
        Some("") => "@".to_string(),
        Some(query) => format!("@{query}"),
        None => "no active @query".to_string(),
    };
    let selected_token = selected_token
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .unwrap_or("none");
    let room_status = if can_notify_room {
        "@room packet eligible"
    } else {
        "@room packet blocked by power level"
    };
    let suggestion_state = if suggestion_count == 0 {
        "no cached rich rows"
    } else {
        "cached rich rows ready"
    };

    format!(
        "Local rich mention packet snapshot: {action} selected. Active query: {query_label}. Cached suggestion count: {suggestion_count}; {suggestion_state}. Selected token: {selected_token}. Loaded room_members rows: {loaded_member_cache_size}. {room_status}. Rich popup model, pill draft, highlighted result rows, hover-card source, tray selection state, rich attachment/edit/reply payload scopes, SendMessage/add_mentions handoff, and attachment-caption AttachmentConfig.mentions handoff are represented as local metadata only. No floating popup search, server-side member directory search, duplicate-name disambiguation, profile/avatar fetch, profile hover card, pill editor mutation, rich attachment/edit mention payload, extra SendMessage, extra SendAttachment beyond the review-row handoff, typing notice, room-state, membership, account/profile, gateway/runtime/auth, or live mutation was submitted. {MENTION_PICKER_RICH_DIRECTORY_CONTROLS_LABEL}"
    )
}

fn mention_picker_preflight_detail_controls_label(
    action: Option<&str>,
    active_query: Option<&str>,
    suggestion_count: usize,
    selected_token: Option<&str>,
    loaded_member_cache_size: usize,
    can_notify_room: bool,
) -> String {
    let action = action
        .map(str::trim)
        .filter(|action| !action.is_empty())
        .unwrap_or("ready");
    let query_label = match active_query.map(str::trim) {
        Some("") => "@".to_string(),
        Some(query) => format!("@{query}"),
        None => "no active @query".to_string(),
    };
    let selected_token = selected_token
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .unwrap_or("none");
    let result_state = if suggestion_count == 0 {
        "no cached result"
    } else {
        "cached result ready"
    };
    let retry_cache = if active_query.is_some() {
        "active query cached"
    } else {
        "no active retry target"
    };
    let room_status = if can_notify_room {
        "@room eligible"
    } else {
        "@room unavailable"
    };

    format!(
        "Mention preflight detail {action}. Request: {query_label}. Result: {result_state} with {suggestion_count} cached suggestions. Error: local no-match/status only, no remote error channel. Retry: {retry_cache}. Source: loaded room_members rows {loaded_member_cache_size}; selected token: {selected_token}; {room_status}. Remote member lookup, server-side directory search, duplicate-name disambiguation, profile/avatar fetch, rich popup search, pill editor, attachment/edit mention payload, SendAttachment, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, retry automation, and live mutation stay local blocked. {MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_LABEL}"
    )
}

fn mention_picker_payload_scope_controls_label(
    action: Option<&str>,
    active_query: Option<&str>,
    suggestion_count: usize,
    selected_token: Option<&str>,
    loaded_member_cache_size: usize,
    can_notify_room: bool,
) -> String {
    let action = action
        .map(str::trim)
        .filter(|action| !action.is_empty())
        .unwrap_or("ready");
    let query_label = match active_query.map(str::trim) {
        Some("") => "@".to_string(),
        Some(query) => format!("@{query}"),
        None => "no active @query".to_string(),
    };
    let selected_token = selected_token
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .unwrap_or("none");
    let room_status = if can_notify_room {
        "@room SendMessage payload eligible"
    } else {
        "@room SendMessage payload blocked by power level"
    };

    format!(
        "Mention payload-scope control {action}. Active query: {query_label}. Cached suggestion count: {suggestion_count}. Selected token: {selected_token}. Loaded room_members rows: {loaded_member_cache_size}. {room_status}. Send uses SendMessage/add_mentions and Attach captions use AttachmentConfig.mentions from the same loaded cache; rich Attach editors, Edit, Reply, and Source remain local scope metadata; Packet records drilldown acceptance, Contract records typed mention payload contract slots, and Taxonomy records remote hover/profile/disambiguation/edit-reply result slots. No rich attachment mention payload editor, edit-message mention payload rewrite, reply mention rewriting, remote member lookup, server-side member directory search beyond explicit Directory, profile/avatar fetch, remote hover-card fetch, rich popup search, pill editor, extra SendAttachment beyond the review-row handoff, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, or live mutation. {MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_LABEL}"
    )
}

fn mention_picker_payload_drilldown_packet_label(
    action: Option<&str>,
    active_query: Option<&str>,
    suggestion_count: usize,
    selected_token: Option<&str>,
    loaded_member_cache_size: usize,
    can_notify_room: bool,
) -> String {
    let action = action
        .map(str::trim)
        .filter(|action| !action.is_empty())
        .unwrap_or("Packet");
    let query_label = match active_query.map(str::trim) {
        Some("") => "@".to_string(),
        Some(query) => format!("@{query}"),
        None => "no active @query".to_string(),
    };
    let selected_token = selected_token
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .unwrap_or("none");
    let room_status = if can_notify_room {
        "@room payload allowed"
    } else {
        "@room payload blocked by power level"
    };
    let rich_result_state = if suggestion_count == 0 {
        "no cached rich rows"
    } else {
        "cached rich rows ready"
    };

    format!(
        "Mention payload drilldown packet {action}. Active query: {query_label}. Cached suggestion count: {suggestion_count}; {rich_result_state}. Selected token: {selected_token}. Loaded room_members rows: {loaded_member_cache_size}. {room_status}. Rich picker acceptance: compact cached rows, highlighted popup model, hover-card source, tray selection, and pill draft remain metadata. Directory acceptance: remote member lookup, server-side member directory search, duplicate-name disambiguation, profile/avatar fetch, and hover-card fetch remain blocked. Payload scopes: Send uses SendMessage/add_mentions and Attach captions use AttachmentConfig.mentions; rich Attach editors, Edit, Reply, and Source wait for typed mention payload contracts. Preflight acceptance: Request, Result, Error, Retry, and Source stay local cached metadata with no retry automation. No rich popup search, pill editor mutation, rich attachment/edit/reply mention payload rewrite, extra SendAttachment beyond the review-row handoff, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, or live mutation. {MENTION_PICKER_PAYLOAD_DRILLDOWN_PACKET_LABEL}"
    )
}

fn mention_picker_payload_typed_contract_packet_label(
    action: Option<&str>,
    active_query: Option<&str>,
    suggestion_count: usize,
    selected_token: Option<&str>,
    loaded_member_cache_size: usize,
    can_notify_room: bool,
) -> String {
    let action = action
        .map(str::trim)
        .filter(|action| !action.is_empty())
        .unwrap_or("Contract");
    let query_label = match active_query.map(str::trim) {
        Some("") => "@".to_string(),
        Some(query) => format!("@{query}"),
        None => "no active @query".to_string(),
    };
    let selected_token = selected_token
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .unwrap_or("none");
    let room_status = if can_notify_room {
        "@room contract eligible"
    } else {
        "@room contract blocked by power level"
    };
    let cached_result_state = if suggestion_count == 0 {
        "typed result waits for cached rows"
    } else {
        "typed result has cached rows"
    };

    format!(
        "Mention payload typed contract {action}. Active query: {query_label}. Cached suggestion count: {suggestion_count}; {cached_result_state}. Selected token: {selected_token}. Loaded room_members rows: {loaded_member_cache_size}. {room_status}. Rich picker contract: request/result/error/retry/source slots stay local. Directory contract: server directory lookup, duplicate-name disambiguation, profile/avatar, and hover-card source stay blocked. Payload contracts: SendMessage/add_mentions and attachment-caption AttachmentConfig.mentions are the current sources while rich Attach editors, Edit, Reply, and Source wait for typed payload handoff. Control contracts: tray state, pill draft, source-hash, stale-token handling, idempotency, and promotion blockers are metadata only. No remote member lookup, server-side member directory search, duplicate-name disambiguation, profile/avatar fetch, hover-card fetch, rich popup search, pill editor mutation, rich attachment/edit/reply mention payload rewrite, extra SendAttachment beyond the review-row handoff, extra SendMessage, typing notice, room-state, membership, account/profile, gateway/runtime/auth, retry automation, or live mutation. {MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_LABEL}"
    )
}

fn mention_picker_remote_result_taxonomy_packet_label(
    action: Option<&str>,
    active_query: Option<&str>,
    suggestion_count: usize,
    selected_token: Option<&str>,
    loaded_member_cache_size: usize,
    can_notify_room: bool,
    directory_result: Option<&UserDirectorySearchResult>,
) -> String {
    let action = action
        .map(str::trim)
        .filter(|action| !action.is_empty())
        .unwrap_or("Taxonomy");
    let query_label = match active_query.map(str::trim) {
        Some("") => "@".to_string(),
        Some(query) => format!("@{query}"),
        None => "no active @query".to_string(),
    };
    let selected_token = selected_token
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .unwrap_or("none");
    let room_status = if can_notify_room {
        "@room mention source eligible"
    } else {
        "@room mention source blocked by power level"
    };
    let directory_state = directory_result
        .map(|result| {
            format!(
                "directory_result cached query @{} rows {} limited {}",
                result.query,
                result.results.len(),
                result.limited
            )
        })
        .unwrap_or_else(|| "directory_result not_cached".to_string());

    format!(
        "Mention remote result taxonomy packet {action}. Active query: {query_label}. Cached suggestion count: {suggestion_count}. Selected token: {selected_token}. Loaded room_members rows: {loaded_member_cache_size}. {room_status}. Live references: create_message_with_mentions SendMessage/add_mentions, mentions_for_text attachment-caption AttachmentConfig.mentions, MatrixRequest::SearchUserDirectory/client.search_users Directory read, UserDirectorySearchAction::Searched result/error metadata, bounded Directory result promotion into literal Matrix user-id tokens, local Hover snapshot from cached directory or loaded member metadata, local completed-token pill removal, local rich packet snapshot, and local Packet/Contract copy. Directory cache: {directory_state}. Blocked rich_picker_operation_id: not_assigned. Blocked richer_directory_result_ui: not_wired beyond bounded literal user-id token promotion. Blocked duplicate_disambiguation_operation_id: not_assigned; duplicate_name_result resolved/ambiguous/forbidden/failed/stale not_wired. Blocked remote_hover_profile_operation_id: not_assigned; hover_profile_result loaded/unavailable/forbidden/redacted/failed/stale not_wired. Blocked avatar_profile_fetch_result: not_wired beyond directory response avatar_url metadata. Blocked rich_attachment_editor_operation_id, edit_payload_operation_id, and reply_payload_operation_id: not_assigned; rich attachment/edit/reply rewrite results not_wired. Blocked multi_select_tray_result and pill_editor_result: not_wired; current pill remove is local composer rewrite only. Retry/cancel policy: no retry automation; cancel is local dismiss/text edit only. Source-hash policy before promotion: token hash, query hash, room id, member cache generation, directory result generation, composer revision, and payload scope hash required. Audit redaction: no raw message body, access token, room secret, profile secret, avatar bytes, or full mention payload in local packet. No remote member lookup beyond explicit Directory, profile/avatar fetch, remote hover-card request, duplicate-name disambiguation workflow, rich popup search, rich attachment/edit/reply mention payload rewrite, extra SendMessage, extra SendAttachment, typing notice, room-state, membership, account/profile, gateway/runtime/auth/provider, Telegram delivery, or live mutation. {MENTION_PICKER_REMOTE_RESULT_TAXONOMY_PACKET_LABEL}"
    )
}

fn active_mention_query(entered_text: &str) -> Option<&str> {
    if entered_text.chars().last().is_some_and(char::is_whitespace) {
        return None;
    }
    let last_token = entered_text.split_whitespace().last()?;
    let query = last_token.strip_prefix('@')?;
    Some(query)
}

fn replace_active_mention_token(entered_text: &str, replacement: &str) -> String {
    if active_mention_query(entered_text).is_none() {
        return entered_text.to_string();
    }
    let trimmed_end_len = entered_text.trim_end_matches(char::is_whitespace).len();
    let trailing = &entered_text[trimmed_end_len..];
    let trimmed = &entered_text[..trimmed_end_len];
    let Some(last_space_index) = trimmed.rfind(char::is_whitespace) else {
        return format!("{replacement} {trailing}");
    };
    let prefix_end = last_space_index
        + trimmed[last_space_index..]
            .chars()
            .next()
            .map(char::len_utf8)
            .unwrap_or(1);
    let prefix = &trimmed[..prefix_end];
    format!("{prefix}{replacement} {trailing}")
}

fn next_suggestion_index(current: usize, count: usize, delta: isize) -> usize {
    if count == 0 {
        return 0;
    }
    (current as isize + delta).rem_euclid(count as isize) as usize
}

fn push_unique_user_id(user_ids: &mut Vec<OwnedUserId>, user_id: OwnedUserId) {
    if !user_ids.iter().any(|existing| existing == &user_id) {
        user_ids.push(user_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mention_extraction_adds_literal_user_ids_and_room_mentions() {
        let mentions =
            extract_mentions_from_loaded_members("hello @alice:example.org and @room", None, true)
                .expect("expected mentions");

        assert!(mentions.room);
        assert!(
            mentions
                .user_ids
                .contains(&UserId::parse("@alice:example.org").unwrap().to_owned())
        );
    }

    #[test]
    fn mention_extraction_ignores_room_when_power_level_disallows_it() {
        let mentions = extract_mentions_from_loaded_members("hello @room", None, false);

        assert!(mentions.is_none());
    }

    #[test]
    fn mention_extraction_deduplicates_literal_user_ids() {
        let mentions = extract_mentions_from_loaded_members(
            "@alice:example.org @alice:example.org",
            None,
            false,
        )
        .expect("expected mentions");

        assert_eq!(mentions.user_ids.len(), 1);
    }

    #[test]
    fn mention_send_payload_metadata_label_summarizes_local_send_counts() {
        let label = mention_send_payload_metadata_label(
            "hello @alice:example.org @room @ghost",
            None,
            true,
        );

        assert!(label.contains("Mention send payload metadata"));
        assert!(label.contains("Format: markdown"));
        assert!(label.contains("Scanned @tokens: 3"));
        assert!(label.contains("Matrix mention users: 1"));
        assert!(label.contains("Literal Matrix user-id tokens: 1"));
        assert!(label.contains("Cached member matches: 0"));
        assert!(label.contains("Unmatched local tokens: 1"));
        assert!(label.contains("@room flag: enabled"));
        assert!(label.contains("loaded room_members: unavailable"));
        assert!(label.contains(MENTION_PICKER_SEND_PAYLOAD_METADATA_LABEL));
        assert!(
            MENTION_PICKER_SEND_PAYLOAD_METADATA_EVIDENCE
                .contains("send-time mention payload metadata")
        );
        assert!(
            MENTION_PICKER_SEND_PAYLOAD_METADATA_EVIDENCE
                .contains("deduped Matrix mention user count")
        );
        assert!(
            MENTION_PICKER_SEND_PAYLOAD_METADATA_EVIDENCE
                .contains("cached RoomMember display/localpart match count")
        );
        assert!(
            MENTION_PICKER_SEND_PAYLOAD_METADATA_EVIDENCE
                .contains("loaded room_members cache size")
        );
        assert!(
            MENTION_PICKER_SEND_PAYLOAD_METADATA_EVIDENCE
                .contains("add_mentions attaches Matrix Mentions once")
        );
        assert!(MENTION_PICKER_SEND_PAYLOAD_METADATA_EVIDENCE.contains("remote member lookup"));
        assert!(MENTION_PICKER_SEND_PAYLOAD_METADATA_EVIDENCE.contains("extra SendMessage"));
        assert!(MENTION_PICKER_SEND_PAYLOAD_METADATA_EVIDENCE.contains("gateway/runtime/auth"));
        assert!(MENTION_PICKER_SEND_PAYLOAD_METADATA_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn mention_send_payload_metadata_label_reports_room_power_level_block() {
        let label = mention_send_payload_metadata_label("/plain hi @room", Some(&[]), false);

        assert!(label.contains("Format: /plain"));
        assert!(label.contains("Scanned @tokens: 1"));
        assert!(label.contains("Matrix mention users: 0"));
        assert!(label.contains("@room flag: blocked by power level"));
        assert!(label.contains("loaded room_members: 0"));
    }

    #[test]
    fn mention_payload_scope_controls_label_keeps_attachment_and_edit_payloads_local() {
        let label = mention_picker_payload_scope_controls_label(
            Some("Attach"),
            Some("ali"),
            2,
            Some("@alice"),
            3,
            true,
        );

        assert!(label.contains("Mention payload-scope control Attach"));
        assert!(label.contains("Active query: @ali"));
        assert!(label.contains("Cached suggestion count: 2"));
        assert!(label.contains("Selected token: @alice"));
        assert!(label.contains("Loaded room_members rows: 3"));
        assert!(label.contains("@room SendMessage payload eligible"));
        assert!(label.contains("Send uses SendMessage/add_mentions"));
        assert!(label.contains("Attach captions use AttachmentConfig.mentions"));
        assert!(
            label.contains(
                "rich Attach editors, Edit, Reply, and Source remain local scope metadata"
            )
        );
        assert!(label.contains("Packet records drilldown acceptance"));
        assert!(label.contains("Contract records typed mention payload contract slots"));
        assert!(label.contains("Taxonomy records remote hover/profile/disambiguation/edit-reply"));
        assert!(label.contains("rich attachment mention payload editor"));
        assert!(label.contains("edit-message mention payload rewrite"));
        assert!(label.contains("reply mention rewriting"));
        assert!(label.contains("server-side member directory search"));
        assert!(label.contains("extra SendAttachment beyond the review-row handoff"));
        assert!(label.contains("extra SendMessage"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_LABEL));
        assert!(
            MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_EVIDENCE
                .contains("Send, Attach, Edit, Reply, Source, Packet, Contract, and Taxonomy")
        );
        assert!(
            MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_EVIDENCE
                .contains("existing SendMessage/add_mentions source")
        );
        assert!(
            MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_EVIDENCE
                .contains("attachment-caption AttachmentConfig.mentions source")
        );
        assert!(
            MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_EVIDENCE
                .contains("Contract maps it to typed rich-picker")
        );
        assert!(
            MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_EVIDENCE
                .contains("Taxonomy records remote hover/profile/disambiguation/edit-reply")
        );
        assert!(
            MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_EVIDENCE
                .contains("rich attachment mention payload editor")
        );
        assert!(
            MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_EVIDENCE
                .contains("edit-message mention payload rewrite")
        );
        assert!(MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_EVIDENCE.contains("live mutation request"));
    }

    #[test]
    fn mention_payload_scope_controls_label_reports_empty_state() {
        let label = mention_picker_payload_scope_controls_label(None, None, 0, Some(""), 0, false);

        assert!(label.contains("Mention payload-scope control ready"));
        assert!(label.contains("Active query: no active @query"));
        assert!(label.contains("Selected token: none"));
        assert!(label.contains("@room SendMessage payload blocked by power level"));
    }

    #[test]
    fn mention_payload_drilldown_packet_label_persists_acceptance_matrix() {
        let label = mention_picker_payload_drilldown_packet_label(
            Some("Packet"),
            Some("ali"),
            2,
            Some("@alice"),
            5,
            true,
        );

        assert!(label.contains("Mention payload drilldown packet Packet"));
        assert!(label.contains("Active query: @ali"));
        assert!(label.contains("Cached suggestion count: 2"));
        assert!(label.contains("cached rich rows ready"));
        assert!(label.contains("Selected token: @alice"));
        assert!(label.contains("Loaded room_members rows: 5"));
        assert!(label.contains("@room payload allowed"));
        assert!(label.contains("Rich picker acceptance"));
        assert!(label.contains("hover-card source"));
        assert!(label.contains("tray selection"));
        assert!(label.contains("pill draft"));
        assert!(label.contains("Directory acceptance"));
        assert!(label.contains("server-side member directory search"));
        assert!(label.contains("duplicate-name disambiguation"));
        assert!(label.contains("Payload scopes"));
        assert!(label.contains("Send uses SendMessage/add_mentions"));
        assert!(label.contains("Attach captions use AttachmentConfig.mentions"));
        assert!(label.contains("rich Attach editors, Edit, Reply, and Source"));
        assert!(label.contains("typed mention payload contracts"));
        assert!(label.contains("Preflight acceptance"));
        assert!(label.contains("Request, Result, Error, Retry, and Source"));
        assert!(label.contains("retry automation"));
        assert!(label.contains("rich attachment/edit/reply mention payload rewrite"));
        assert!(label.contains("extra SendAttachment beyond the review-row handoff"));
        assert!(label.contains("extra SendMessage"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MENTION_PICKER_PAYLOAD_DRILLDOWN_PACKET_LABEL));
        assert!(
            MENTION_PICKER_PAYLOAD_DRILLDOWN_PACKET_EVIDENCE.contains("visible Packet control")
        );
        assert!(
            MENTION_PICKER_PAYLOAD_DRILLDOWN_PACKET_EVIDENCE
                .contains("mention payload drilldown acceptance matrix")
        );
        assert!(
            MENTION_PICKER_PAYLOAD_DRILLDOWN_PACKET_EVIDENCE
                .contains("rich picker, server directory")
        );
        assert!(
            MENTION_PICKER_PAYLOAD_DRILLDOWN_PACKET_EVIDENCE
                .contains("attachment-caption AttachmentConfig.mentions")
        );
        assert!(
            MENTION_PICKER_PAYLOAD_DRILLDOWN_PACKET_EVIDENCE
                .contains("rich attachment/edit/reply payload scopes")
        );
        assert!(MENTION_PICKER_PAYLOAD_DRILLDOWN_PACKET_EVIDENCE.contains("live mutation request"));
    }

    #[test]
    fn mention_payload_drilldown_packet_label_reports_empty_state() {
        let label =
            mention_picker_payload_drilldown_packet_label(None, None, 0, Some(""), 0, false);

        assert!(label.contains("Mention payload drilldown packet Packet"));
        assert!(label.contains("Active query: no active @query"));
        assert!(label.contains("no cached rich rows"));
        assert!(label.contains("Selected token: none"));
        assert!(label.contains("@room payload blocked by power level"));
    }

    #[test]
    fn mention_payload_typed_contract_packet_label_maps_packet_to_contracts() {
        let label = mention_picker_payload_typed_contract_packet_label(
            Some("Contract"),
            Some("ali"),
            2,
            Some("@alice"),
            5,
            true,
        );

        assert!(label.contains("Mention payload typed contract Contract"));
        assert!(label.contains("Active query: @ali"));
        assert!(label.contains("Cached suggestion count: 2"));
        assert!(label.contains("typed result has cached rows"));
        assert!(label.contains("Selected token: @alice"));
        assert!(label.contains("Loaded room_members rows: 5"));
        assert!(label.contains("@room contract eligible"));
        assert!(label.contains("Rich picker contract"));
        assert!(label.contains("request/result/error/retry/source"));
        assert!(label.contains("Directory contract"));
        assert!(label.contains("server directory lookup"));
        assert!(label.contains("duplicate-name disambiguation"));
        assert!(label.contains("hover-card source"));
        assert!(label.contains("Payload contracts"));
        assert!(label.contains("SendMessage/add_mentions"));
        assert!(label.contains("attachment-caption AttachmentConfig.mentions"));
        assert!(label.contains("rich Attach editors, Edit, Reply, and Source"));
        assert!(label.contains("Control contracts"));
        assert!(label.contains("source-hash"));
        assert!(label.contains("stale-token handling"));
        assert!(label.contains("idempotency"));
        assert!(label.contains("promotion blockers"));
        assert!(label.contains("rich attachment/edit/reply mention payload rewrite"));
        assert!(label.contains("extra SendAttachment beyond the review-row handoff"));
        assert!(label.contains("extra SendMessage"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_LABEL));
        assert!(
            MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE
                .contains("visible Contract control")
        );
        assert!(
            MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE
                .contains("typed mention contract slots")
        );
        assert!(
            MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE
                .contains("server-directory lookup")
        );
        assert!(
            MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE
                .contains("attachment-caption AttachmentConfig.mentions handoff")
        );
        assert!(
            MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE
                .contains("rich attachment/edit/reply payload scopes")
        );
        assert!(
            MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE.contains("live mutation request")
        );
    }

    #[test]
    fn mention_payload_typed_contract_packet_label_reports_empty_state() {
        let label =
            mention_picker_payload_typed_contract_packet_label(None, None, 0, Some(""), 0, false);

        assert!(label.contains("Mention payload typed contract Contract"));
        assert!(label.contains("Active query: no active @query"));
        assert!(label.contains("typed result waits for cached rows"));
        assert!(label.contains("Selected token: none"));
        assert!(label.contains("@room contract blocked by power level"));
    }

    #[test]
    fn mention_remote_result_taxonomy_packet_label_lists_blocked_slots() {
        let result = UserDirectorySearchResult {
            query: "ali".to_string(),
            limited: true,
            results: vec![crate::sliding_sync::UserDirectorySearchEntry {
                user_id: UserId::parse("@alice:example.org").unwrap().to_owned(),
                display_name: Some("Alice".to_string()),
                avatar_url: Some("mxc://example.org/avatar".try_into().unwrap()),
            }],
        };
        let label = mention_picker_remote_result_taxonomy_packet_label(
            Some("Taxonomy"),
            Some("ali"),
            2,
            Some("@alice"),
            5,
            true,
            Some(&result),
        );

        assert!(label.contains("Mention remote result taxonomy packet Taxonomy"));
        assert!(label.contains("Active query: @ali"));
        assert!(label.contains("Cached suggestion count: 2"));
        assert!(label.contains("Selected token: @alice"));
        assert!(label.contains("Loaded room_members rows: 5"));
        assert!(label.contains("@room mention source eligible"));
        assert!(label.contains("SendMessage/add_mentions"));
        assert!(label.contains("AttachmentConfig.mentions"));
        assert!(label.contains("MatrixRequest::SearchUserDirectory/client.search_users"));
        assert!(label.contains("bounded Directory result promotion"));
        assert!(label.contains("local Hover snapshot"));
        assert!(label.contains("directory_result cached query @ali rows 1 limited true"));
        assert!(label.contains("rich_picker_operation_id: not_assigned"));
        assert!(label.contains("richer_directory_result_ui: not_wired"));
        assert!(label.contains("duplicate_disambiguation_operation_id: not_assigned"));
        assert!(label.contains(
            "hover_profile_result loaded/unavailable/forbidden/redacted/failed/stale not_wired"
        ));
        assert!(label.contains("edit_payload_operation_id"));
        assert!(label.contains("reply_payload_operation_id"));
        assert!(label.contains("Source-hash policy"));
        assert!(label.contains("Audit redaction"));
        assert!(label.contains("No remote member lookup beyond explicit Directory"));
        assert!(label.contains("profile/avatar fetch"));
        assert!(label.contains("extra SendMessage"));
        assert!(label.contains("extra SendAttachment"));
        assert!(label.contains("Telegram delivery"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MENTION_PICKER_REMOTE_RESULT_TAXONOMY_PACKET_LABEL));
        assert!(
            MENTION_PICKER_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("visible Taxonomy control")
        );
        assert!(
            MENTION_PICKER_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("remote hover/profile/disambiguation/edit-reply")
        );
        assert!(
            MENTION_PICKER_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("not-assigned or not-wired")
        );
    }

    #[test]
    fn mention_remote_result_taxonomy_packet_label_reports_empty_state() {
        let label = mention_picker_remote_result_taxonomy_packet_label(
            None,
            None,
            0,
            Some(""),
            0,
            false,
            None,
        );

        assert!(label.contains("Mention remote result taxonomy packet Taxonomy"));
        assert!(label.contains("Active query: no active @query"));
        assert!(label.contains("Selected token: none"));
        assert!(label.contains("@room mention source blocked by power level"));
        assert!(label.contains("directory_result not_cached"));
    }

    #[test]
    fn active_mention_query_reads_last_at_token() {
        assert_eq!(active_mention_query("hello @ali"), Some("ali"));
        assert_eq!(active_mention_query("hello world"), None);
    }

    #[test]
    fn active_mention_query_ignores_completed_tokens_with_trailing_space() {
        assert_eq!(active_mention_query("hello @alice "), None);
        assert_eq!(active_mention_query("@room\n"), None);
    }

    #[test]
    fn replace_active_mention_token_preserves_prefix_and_adds_space() {
        assert_eq!(
            replace_active_mention_token("hello @ali", "@alice"),
            "hello @alice "
        );
        assert_eq!(replace_active_mention_token("@ro", "@room"), "@room ");
    }

    #[test]
    fn replace_active_mention_token_leaves_completed_mentions_alone() {
        assert_eq!(
            replace_active_mention_token("hello @alice ", "@room"),
            "hello @alice "
        );
    }

    #[test]
    fn completed_mention_tokens_skip_active_unfinished_token() {
        assert_eq!(completed_mention_tokens("hello @alice @bo"), vec!["@alice"]);
        assert_eq!(
            completed_mention_tokens("hello @alice @bob "),
            vec!["@alice", "@bob"]
        );
    }

    #[test]
    fn completed_mention_pills_map_room_literal_loaded_and_unmatched_tokens() {
        let members = vec![CachedMentionMember {
            display_name: Some("Alice".to_string()),
            user_id: UserId::parse("@alice:example.org").unwrap().to_owned(),
            avatar_present: true,
        }];
        let pills = completed_mention_pills_for_text(
            "hello @room @alice @bob:example.org @ghost ",
            &members,
            true,
        );

        assert_eq!(pills.len(), 3);
        assert_eq!(pills[0].token, "@room");
        assert!(pills[0].detail_label.contains("Matrix room mention flag"));
        assert_eq!(pills[1].token, "@alice");
        assert!(pills[1].detail_label.contains("loaded member Alice"));
        assert!(pills[1].detail_label.contains("@alice:example.org"));
        assert_eq!(pills[2].token, "@bob:example.org");
        assert!(pills[2].detail_label.contains("literal Matrix user id"));
        assert!(pills[2].button_label.contains("Remove @bob:example.org"));
    }

    #[test]
    fn completed_mention_pills_report_power_level_block_and_unmatched_tokens() {
        let pills = completed_mention_pills_for_text("hello @room @ghost ", &[], false);

        assert_eq!(pills.len(), 2);
        assert!(
            pills[0]
                .detail_label
                .contains("blocked by the current power level")
        );
        assert!(pills[1].detail_label.contains("unmatched local token"));
    }

    #[test]
    fn mention_local_pill_tray_label_summarizes_removable_local_pills() {
        let pills = vec![MentionPill {
            token: "@alice".to_string(),
            button_label: "Remove @alice".to_string(),
            detail_label:
                "loaded member Alice -> @alice:example.org will be included in m.mentions"
                    .to_string(),
        }];
        let label = mention_local_pill_tray_label(&pills);

        assert!(label.contains("Mention local pill tray"));
        assert!(label.contains("pill 1 @alice"));
        assert!(label.contains("loaded member Alice"));
        assert!(label.contains("removes only that completed @token"));
        assert!(label.contains("SendMessage"));
        assert!(label.contains("SendAttachment"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MENTION_PICKER_LOCAL_PILL_TRAY_LIVE_LABEL));
        assert!(
            MENTION_PICKER_LOCAL_PILL_TRAY_LIVE_EVIDENCE
                .contains("live local completed-mention pill tray")
        );
        assert!(
            MENTION_PICKER_LOCAL_PILL_TRAY_LIVE_EVIDENCE
                .contains("remove that completed token from composer text")
        );
        assert!(MENTION_PICKER_LOCAL_PILL_TRAY_LIVE_EVIDENCE.contains("SendMessage"));
        assert!(MENTION_PICKER_LOCAL_PILL_TRAY_LIVE_EVIDENCE.contains("live mutation request"));
    }

    #[test]
    fn remove_completed_mention_token_removes_first_matching_completed_token() {
        assert_eq!(
            remove_completed_mention_token("hello @alice and @alice ", "@alice"),
            "hello and @alice "
        );
        assert_eq!(
            remove_completed_mention_token("@room @alice", "@room"),
            "@alice"
        );
    }

    #[test]
    fn next_suggestion_index_wraps_keyboard_selection() {
        assert_eq!(next_suggestion_index(0, 3, 1), 1);
        assert_eq!(next_suggestion_index(2, 3, 1), 0);
        assert_eq!(next_suggestion_index(0, 3, -1), 2);
        assert_eq!(next_suggestion_index(3, 0, -1), 0);
        assert!(MENTION_PICKER_KEYBOARD_SELECTION_EVIDENCE.contains("ArrowUp/ArrowDown selection"));
        assert!(MENTION_PICKER_KEYBOARD_SELECTION_EVIDENCE.contains("Tab/Enter insertion"));
        assert!(MENTION_PICKER_KEYBOARD_SELECTION_EVIDENCE.contains("selected_suggestion_index"));
        assert!(MENTION_PICKER_KEYBOARD_SELECTION_EVIDENCE.contains("trailing space"));
        assert!(MENTION_PICKER_KEYBOARD_SELECTION_EVIDENCE.contains("extra SendMessage"));
        assert!(MENTION_PICKER_KEYBOARD_SELECTION_EVIDENCE.contains("gateway/runtime/auth"));
        assert!(MENTION_PICKER_KEYBOARD_SELECTION_EVIDENCE.contains("live mutation"));
        assert!(MENTION_PICKER_KEYBOARD_SELECTION_LABEL.contains("ArrowUp/ArrowDown"));
    }

    #[test]
    fn loaded_mention_identity_label_includes_display_user_localpart_and_avatar_status() {
        let user_id = UserId::parse("@alice:example.org").unwrap();
        let label = format_loaded_mention_identity(Some("Alice"), &user_id, true);

        assert!(label.contains("display Alice"));
        assert!(label.contains("user id @alice:example.org"));
        assert!(label.contains("localpart @alice"));
        assert!(label.contains("avatar mxc: present"));
    }

    #[test]
    fn loaded_mention_identity_label_handles_missing_display_and_avatar() {
        let user_id = UserId::parse("@bob:example.org").unwrap();
        let label = format_loaded_mention_identity(None, &user_id, false);

        assert!(label.contains("display name unavailable"));
        assert!(label.contains("user id @bob:example.org"));
        assert!(label.contains("localpart @bob"));
        assert!(label.contains("avatar mxc: none"));
    }

    #[test]
    fn mention_local_candidate_rows_label_summarizes_cache_rows() {
        let rows = vec![
            format_local_mention_candidate_row(
                0,
                true,
                "@room",
                "room-wide mention - power level eligible - source power levels",
            ),
            "row 2 available @alice - token @alice - display Alice - user id @alice:example.org - localpart @alice - avatar mxc present - source loaded room_members".to_string(),
        ];
        let label = mention_local_candidate_rows_label(Some("ali"), 2, Some("@room"), true, &rows);

        assert!(label.contains("Mention local candidate rows"));
        assert!(label.contains("Active query: @ali"));
        assert!(label.contains("Cached suggestion count: 2"));
        assert!(label.contains("Selected token: @room"));
        assert!(label.contains("@room row eligible"));
        assert!(label.contains("row 1 selected @room"));
        assert!(label.contains("row 2 available @alice"));
        assert!(label.contains("display Alice"));
        assert!(label.contains("user id @alice:example.org"));
        assert!(label.contains("avatar mxc present"));
        assert!(label.contains("source loaded room_members"));
        assert!(label.contains("up to three loaded room_members cache matches"));
        assert!(label.contains("server-side directory search"));
        assert!(label.contains("extra SendMessage"));
        assert!(label.contains("retry automation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MENTION_PICKER_LOCAL_CANDIDATE_ROWS_LABEL));
        assert!(MENTION_PICKER_LOCAL_CANDIDATE_ROWS_EVIDENCE.contains("rank"));
        assert!(MENTION_PICKER_LOCAL_CANDIDATE_ROWS_EVIDENCE.contains("selected state"));
        assert!(MENTION_PICKER_LOCAL_CANDIDATE_ROWS_EVIDENCE.contains("Matrix user id"));
        assert!(MENTION_PICKER_LOCAL_CANDIDATE_ROWS_EVIDENCE.contains("avatar MXC status"));
        assert!(MENTION_PICKER_LOCAL_CANDIDATE_ROWS_EVIDENCE.contains("extra SendMessage"));
        assert!(MENTION_PICKER_LOCAL_CANDIDATE_ROWS_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn mention_local_candidate_rows_label_handles_empty_rows() {
        let label = mention_local_candidate_rows_label(None, 0, Some(""), false, &[]);

        assert!(label.contains("Active query: no active @query"));
        assert!(label.contains("Cached suggestion count: 0"));
        assert!(label.contains("Selected token: none"));
        assert!(label.contains("@room row unavailable"));
        assert!(label.contains("Rows: no cached candidate rows"));
        assert!(label.contains(MENTION_PICKER_LOCAL_CANDIDATE_ROWS_LABEL));
    }

    #[test]
    fn mention_local_duplicate_hints_label_reports_cached_collisions() {
        let suggestions = vec![
            MentionSuggestion {
                label: "@alice".to_string(),
                token: "@alice".to_string(),
                identity_label: "Loaded member identity: Alice".to_string(),
                candidate_row_label: "row 1 Alice".to_string(),
                display_duplicate_count: 2,
                display_duplicate_key: Some("alice".to_string()),
            },
            MentionSuggestion {
                label: "@alice2".to_string(),
                token: "@alice2".to_string(),
                identity_label: "Loaded member identity: Alice".to_string(),
                candidate_row_label: "row 2 Alice".to_string(),
                display_duplicate_count: 2,
                display_duplicate_key: Some("alice".to_string()),
            },
        ];
        let label =
            mention_local_duplicate_hints_label(Some("ali"), 2, Some("@alice"), &suggestions);

        assert!(label.contains("Mention local duplicate hints"));
        assert!(label.contains("Active query: @ali"));
        assert!(label.contains("Cached member candidates: 2"));
        assert!(label.contains("Selected token: @alice"));
        assert!(label.contains("duplicate display-name groups in cached rows: 1"));
        assert!(label.contains("selected display collision rows: 2"));
        assert!(label.contains("Localpart and Matrix user id"));
        assert!(label.contains("rich duplicate-name disambiguation UI"));
        assert!(label.contains("server-side directory search"));
        assert!(label.contains("extra SendMessage"));
        assert!(label.contains("retry automation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MENTION_PICKER_LOCAL_DUPLICATE_HINTS_LABEL));
        assert!(
            MENTION_PICKER_LOCAL_DUPLICATE_HINTS_EVIDENCE.contains("local duplicate-name hints")
        );
        assert!(
            MENTION_PICKER_LOCAL_DUPLICATE_HINTS_EVIDENCE
                .contains("selected display collision count")
        );
        assert!(MENTION_PICKER_LOCAL_DUPLICATE_HINTS_EVIDENCE.contains("localpart"));
        assert!(MENTION_PICKER_LOCAL_DUPLICATE_HINTS_EVIDENCE.contains("Matrix user id"));
        assert!(MENTION_PICKER_LOCAL_DUPLICATE_HINTS_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn mention_local_duplicate_hints_label_handles_room_and_unique_rows() {
        let suggestions = vec![MentionSuggestion {
            label: "@bob".to_string(),
            token: "@bob".to_string(),
            identity_label: "Loaded member identity: Bob".to_string(),
            candidate_row_label: "row 1 Bob".to_string(),
            display_duplicate_count: 1,
            display_duplicate_key: Some("bob".to_string()),
        }];
        let label = mention_local_duplicate_hints_label(Some(""), 1, Some("@room"), &suggestions);

        assert!(label.contains("Active query: @"));
        assert!(label.contains("Cached member candidates: 1"));
        assert!(label.contains("Selected token: @room"));
        assert!(label.contains("duplicate display-name groups in cached rows: none"));
        assert!(label.contains("selected token is @room"));
        assert!(label.contains(MENTION_PICKER_LOCAL_DUPLICATE_HINTS_LABEL));
    }

    #[test]
    fn mention_picker_lifecycle_metadata_label_summarizes_active_selection() {
        let label = mention_picker_lifecycle_metadata_label(
            "active selection ready",
            Some("ali"),
            3,
            Some("@alice"),
            true,
        );

        assert!(label.contains("Mention lifecycle active selection ready"));
        assert!(label.contains("Active query: @ali"));
        assert!(label.contains("Cached suggestion count: 3"));
        assert!(label.contains("Selected token: @alice"));
        assert!(label.contains("@room eligible"));
        assert!(label.contains("trailing space releases Enter to SendMessage"));
        assert!(label.contains(MENTION_PICKER_LIFECYCLE_METADATA_LABEL));
        assert!(MENTION_PICKER_LIFECYCLE_METADATA_EVIDENCE.contains("active @query"));
        assert!(MENTION_PICKER_LIFECYCLE_METADATA_EVIDENCE.contains("cached suggestion count"));
        assert!(MENTION_PICKER_LIFECYCLE_METADATA_EVIDENCE.contains("selected token"));
        assert!(MENTION_PICKER_LIFECYCLE_METADATA_EVIDENCE.contains("selected_suggestion_index"));
        assert!(MENTION_PICKER_LIFECYCLE_METADATA_EVIDENCE.contains("extra SendMessage"));
        assert!(MENTION_PICKER_LIFECYCLE_METADATA_EVIDENCE.contains("gateway/runtime/auth"));
        assert!(MENTION_PICKER_LIFECYCLE_METADATA_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn mention_picker_lifecycle_metadata_label_uses_empty_fallbacks() {
        let label = mention_picker_lifecycle_metadata_label("", None, 0, Some(""), false);

        assert!(label.contains("Mention lifecycle status updated"));
        assert!(label.contains("Active query: no active @query"));
        assert!(label.contains("Cached suggestion count: 0"));
        assert!(label.contains("Selected token: none"));
        assert!(label.contains("@room unavailable"));
        assert!(label.contains(MENTION_PICKER_LIFECYCLE_METADATA_LABEL));
    }

    #[test]
    fn mention_picker_rich_popup_boundary_label_keeps_popup_scope_unwired() {
        let label = mention_picker_rich_popup_boundary_label(Some("ali"), 2, Some("@alice"));

        assert!(label.contains("Rich mention picker boundary"));
        assert!(label.contains("Active query: @ali"));
        assert!(label.contains("Cached suggestion count: 2"));
        assert!(label.contains("Selected token: @alice"));
        assert!(label.contains("No floating popup search"));
        assert!(label.contains("rich highlight styling"));
        assert!(label.contains("pill editor"));
        assert!(label.contains("remote member lookup"));
        assert!(label.contains("attachment/edit mention payload editor"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MENTION_PICKER_RICH_POPUP_BOUNDARY_LABEL));
        assert!(MENTION_PICKER_RICH_POPUP_BOUNDARY_EVIDENCE.contains("boundary metadata"));
        assert!(MENTION_PICKER_RICH_POPUP_BOUNDARY_EVIDENCE.contains("floating popup search"));
        assert!(
            MENTION_PICKER_RICH_POPUP_BOUNDARY_EVIDENCE.contains("rich highlighted result list")
        );
        assert!(MENTION_PICKER_RICH_POPUP_BOUNDARY_EVIDENCE.contains("pill editor"));
        assert!(MENTION_PICKER_RICH_POPUP_BOUNDARY_EVIDENCE.contains("remote member lookup"));
        assert!(MENTION_PICKER_RICH_POPUP_BOUNDARY_EVIDENCE.contains("profile/avatar fetch"));
        assert!(MENTION_PICKER_RICH_POPUP_BOUNDARY_EVIDENCE.contains("membership mutation"));
        assert!(MENTION_PICKER_RICH_POPUP_BOUNDARY_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn mention_picker_rich_popup_boundary_label_uses_empty_fallbacks() {
        let label = mention_picker_rich_popup_boundary_label(None, 0, Some(""));

        assert!(label.contains("Active query: no active @query"));
        assert!(label.contains("Cached suggestion count: 0"));
        assert!(label.contains("Selected token: none"));
        assert!(label.contains(MENTION_PICKER_RICH_POPUP_BOUNDARY_LABEL));
    }

    #[test]
    fn mention_picker_directory_disambiguation_boundary_label_keeps_server_lookup_unwired() {
        let label = mention_picker_directory_disambiguation_boundary_label(
            Some("ali"),
            3,
            Some("@alice"),
            2,
        );

        assert!(label.contains("Mention directory/disambiguation boundary"));
        assert!(label.contains("Active query: @ali"));
        assert!(label.contains("Cached suggestion count: 3"));
        assert!(label.contains("Selected token: @alice"));
        assert!(label.contains("Loaded member cache rows: 2"));
        assert!(label.contains("Directory can read Matrix user-directory metadata"));
        assert!(label.contains("MatrixRequest::SearchUserDirectory"));
        assert!(label.contains("client.search_users"));
        assert!(label.contains("duplicate display-name disambiguation UI"));
        assert!(label.contains("profile hover cards"));
        assert!(label.contains("avatar/profile fetch beyond directory response fields"));
        assert!(label.contains("multi-select mention tray"));
        assert!(label.contains("pill editor"));
        assert!(label.contains("attachment/edit mention payload editor"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MENTION_PICKER_DIRECTORY_DISAMBIGUATION_BOUNDARY_LABEL));
        assert!(
            MENTION_PICKER_DIRECTORY_DISAMBIGUATION_BOUNDARY_EVIDENCE.contains("active @query")
        );
        assert!(
            MENTION_PICKER_DIRECTORY_DISAMBIGUATION_BOUNDARY_EVIDENCE
                .contains("existing SendMessage add_mentions path")
        );
    }

    #[test]
    fn mention_picker_directory_disambiguation_boundary_label_uses_empty_fallbacks() {
        let label = mention_picker_directory_disambiguation_boundary_label(None, 0, Some(""), 0);

        assert!(label.contains("Active query: no active @query"));
        assert!(label.contains("Cached suggestion count: 0"));
        assert!(label.contains("Selected token: none"));
        assert!(label.contains("Loaded member cache rows: 0"));
        assert!(label.contains(MENTION_PICKER_DIRECTORY_DISAMBIGUATION_BOUNDARY_LABEL));
    }

    #[test]
    fn mention_picker_rich_directory_controls_label_wires_directory_live_read() {
        let label = mention_picker_rich_directory_controls_label(
            Some("Directory"),
            Some("ali"),
            3,
            Some("@alice"),
            2,
        );

        assert!(label.contains("Mention rich/directory control Directory"));
        assert!(label.contains("Active query: @ali"));
        assert!(label.contains("Cached suggestion count: 3"));
        assert!(label.contains("Selected token: @alice"));
        assert!(label.contains("Loaded member cache rows: 2"));
        assert!(label.contains("MatrixRequest::SearchUserDirectory"));
        assert!(label.contains("client.search_users"));
        assert!(label.contains("Rich popup search"));
        assert!(label.contains("duplicate-name disambiguation"));
        assert!(label.contains("profile hover cards"));
        assert!(label.contains("multi-select tray"));
        assert!(label.contains("pill editor"));
        assert!(label.contains("attachment/edit mention payloads"));
        assert!(label.contains("extra SendMessage"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MENTION_PICKER_RICH_DIRECTORY_CONTROLS_LABEL));
        assert!(
            MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE
                .contains("Rich, Directory, Hover, Tray, and Pills")
        );
        assert!(
            MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE
                .contains("MatrixRequest::SearchUserDirectory")
        );
        assert!(MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE.contains("client.search_users"));
        assert!(MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE.contains("extra SendMessage"));
        assert!(MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE.contains("live mutation"));
        assert!(
            MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE
                .contains("local rich mention packet snapshot")
        );
        assert!(
            MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE.contains("local hover-card snapshot")
        );
        assert!(
            MENTION_PICKER_RICH_DIRECTORY_CONTROLS_LABEL
                .contains("Hover renders local cached/directory hover-card snapshots")
        );
    }

    #[test]
    fn mention_picker_hover_card_snapshot_label_uses_directory_result_metadata() {
        let result = UserDirectorySearchResult {
            query: "ali".to_string(),
            limited: true,
            results: vec![
                crate::sliding_sync::UserDirectorySearchEntry {
                    user_id: UserId::parse("@alice:example.org").unwrap().to_owned(),
                    display_name: Some("Alice".to_string()),
                    avatar_url: Some("mxc://example.org/avatar".try_into().unwrap()),
                },
                crate::sliding_sync::UserDirectorySearchEntry {
                    user_id: UserId::parse("@alina:example.org").unwrap().to_owned(),
                    display_name: None,
                    avatar_url: None,
                },
            ],
        };
        let label = mention_picker_hover_card_snapshot_label(
            Some("ali"),
            2,
            Some("@alice"),
            5,
            Some(&result),
            None,
            true,
        );

        assert!(label.contains("Mention Hover local card snapshot"));
        assert!(label.contains("Active query: @ali"));
        assert!(label.contains("Cached suggestion count: 2"));
        assert!(label.contains("Selected token: @alice"));
        assert!(label.contains("Loaded member cache rows: 5"));
        assert!(label.contains("directory result source for @ali rows 2 limited true"));
        assert!(label.contains(
            "directory row 1 user id @alice:example.org display Alice avatar mxc present"
        ));
        assert!(label.contains("directory row 2 user id @alina:example.org display display name unavailable avatar mxc missing"));
        assert!(label.contains("cached user-directory result metadata"));
        assert!(label.contains("No MatrixRequest::SearchUserDirectory"));
        assert!(label.contains("profile/avatar fetch"));
        assert!(label.contains("remote hover-card request"));
        assert!(label.contains("SendMessage"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MENTION_PICKER_HOVER_CARD_SNAPSHOT_LIVE_LABEL));
        assert!(
            MENTION_PICKER_HOVER_CARD_SNAPSHOT_LIVE_EVIDENCE
                .contains("live local hover-card snapshot")
        );
        assert!(
            MENTION_PICKER_HOVER_CARD_SNAPSHOT_LIVE_EVIDENCE
                .contains("already available @mention metadata")
        );
        assert!(
            MENTION_PICKER_HOVER_CARD_SNAPSHOT_LIVE_EVIDENCE
                .contains("no remote hover-card request")
        );
    }

    #[test]
    fn mention_picker_hover_card_snapshot_label_uses_cached_member_or_room() {
        let suggestion = MentionSuggestion {
            label: "@Alice".to_string(),
            token: "@Alice".to_string(),
            identity_label:
                "Loaded member identity: display Alice | user id @alice:example.org | localpart @alice | avatar mxc: present"
                    .to_string(),
            candidate_row_label:
                "token @Alice - display Alice - user id @alice:example.org - localpart @alice - avatar mxc present - duplicate display rows unique - source loaded room_members"
                    .to_string(),
            display_duplicate_count: 1,
            display_duplicate_key: Some("alice".to_string()),
        };
        let member_label = mention_picker_hover_card_snapshot_label(
            Some("ali"),
            1,
            Some("@Alice"),
            3,
            None,
            Some(&suggestion),
            false,
        );

        assert!(member_label.contains("loaded member source selected token @Alice"));
        assert!(member_label.contains("Loaded member identity: display Alice"));
        assert!(member_label.contains("source loaded room_members"));
        assert!(member_label.contains("No MatrixRequest::SearchUserDirectory"));
        assert!(member_label.contains("remote hover-card request"));

        let room_label = mention_picker_hover_card_snapshot_label(
            Some("room"),
            1,
            Some("@room"),
            3,
            None,
            None,
            true,
        );
        assert!(room_label.contains("@room power-level hover snapshot eligible"));
        assert!(room_label.contains("no member row needed"));

        let empty_label =
            mention_picker_hover_card_snapshot_label(None, 0, Some(""), 0, None, None, false);
        assert!(empty_label.contains("no cached directory or loaded member source"));
    }

    #[test]
    fn mention_picker_directory_search_helpers_render_request_result_and_error() {
        assert_eq!(sanitize_user_directory_search_query("  @alice  "), "alice");
        assert_eq!(sanitize_user_directory_search_query("@@bob"), "bob");

        let request_label =
            mention_picker_directory_search_request_label("ali", Some("ali"), 2, Some("@alice"), 3);
        assert!(request_label.contains("requested for @ali"));
        assert!(request_label.contains("MatrixRequest::SearchUserDirectory"));
        assert!(request_label.contains("client.search_users"));
        assert!(request_label.contains("read-only"));
        assert!(request_label.contains("No SendMessage"));

        let result = UserDirectorySearchResult {
            query: "ali".to_string(),
            limited: true,
            results: vec![
                crate::sliding_sync::UserDirectorySearchEntry {
                    user_id: UserId::parse("@alice:example.org").unwrap().to_owned(),
                    display_name: Some("Alice".to_string()),
                    avatar_url: None,
                },
                crate::sliding_sync::UserDirectorySearchEntry {
                    user_id: UserId::parse("@alina:example.org").unwrap().to_owned(),
                    display_name: None,
                    avatar_url: Some("mxc://example.org/avatar".try_into().unwrap()),
                },
            ],
        };
        let result_label =
            mention_picker_directory_search_result_label(&result, 2, Some("@alice"), 3);
        assert!(result_label.contains("result for @ali"));
        assert!(result_label.contains("Result count: 2"));
        assert!(result_label.contains("Limited: true"));
        assert!(result_label.contains("@alice:example.org (Alice, avatar mxc missing)"));
        assert!(result_label.contains("@alina:example.org (no display name, avatar mxc present)"));
        assert!(result_label.contains("read-only metadata from client.search_users"));
        assert!(result_label.contains("visible Directory result buttons"));
        assert!(result_label.contains("does not insert tokens automatically"));
        assert!(result_label.contains("No SendMessage"));

        assert_eq!(
            mention_directory_result_promotion_button_label(0, &result.results[0]),
            "1 Alice @alice:example.org"
        );
        assert_eq!(
            mention_directory_result_promotion_button_label(1, &result.results[1]),
            "2 Directory user @alina:example.org"
        );
        assert!(
            MENTION_PICKER_DIRECTORY_RESULT_PROMOTION_LIVE_EVIDENCE
                .contains("insert_mention_token path")
        );
        assert!(
            MENTION_PICKER_DIRECTORY_RESULT_PROMOTION_LIVE_EVIDENCE
                .contains("no automatic insertion")
        );
        assert!(
            MENTION_PICKER_DIRECTORY_RESULT_PROMOTION_LIVE_LABEL
                .contains("literal Matrix user-id @tokens")
        );

        let error_label = mention_picker_directory_search_error_label(
            Some("ali"),
            Some("ali"),
            "M_LIMIT_EXCEEDED",
            2,
            Some("@alice"),
            3,
        );
        assert!(error_label.contains("failed for @ali"));
        assert!(error_label.contains("Error: M_LIMIT_EXCEEDED"));
        assert!(error_label.contains("retry automation"));
        assert!(error_label.contains("no SendMessage"));

        let unavailable_label =
            mention_picker_directory_search_unavailable_label(Some(""), 0, None, 0);
        assert!(unavailable_label.contains("needs an active @query"));
        assert!(unavailable_label.contains("No MatrixRequest::SearchUserDirectory"));
    }

    #[test]
    fn mention_picker_rich_directory_controls_label_uses_empty_fallbacks() {
        let label = mention_picker_rich_directory_controls_label(None, None, 0, Some(""), 0);

        assert!(label.contains("Mention rich/directory control ready"));
        assert!(label.contains("Active query: no active @query"));
        assert!(label.contains("Cached suggestion count: 0"));
        assert!(label.contains("Selected token: none"));
        assert!(label.contains("Loaded member cache rows: 0"));
        assert!(label.contains(MENTION_PICKER_RICH_DIRECTORY_CONTROLS_LABEL));
    }

    #[test]
    fn mention_picker_rich_mention_packet_snapshot_label_summarizes_local_packet() {
        let label = mention_picker_rich_mention_packet_snapshot_label(
            "Pills",
            Some("ali"),
            3,
            Some("@alice"),
            2,
            true,
        );

        assert!(label.contains("Local rich mention packet snapshot"));
        assert!(label.contains("Pills selected"));
        assert!(label.contains("Active query: @ali"));
        assert!(label.contains("Cached suggestion count: 3"));
        assert!(label.contains("cached rich rows ready"));
        assert!(label.contains("Selected token: @alice"));
        assert!(label.contains("Loaded room_members rows: 2"));
        assert!(label.contains("@room packet eligible"));
        assert!(label.contains("Rich popup model"));
        assert!(label.contains("pill draft"));
        assert!(label.contains("SendMessage/add_mentions handoff"));
        assert!(label.contains("attachment-caption AttachmentConfig.mentions handoff"));
        assert!(label.contains("No floating popup search"));
        assert!(label.contains("server-side member directory search"));
        assert!(label.contains("profile/avatar fetch"));
        assert!(label.contains("rich attachment/edit mention payload"));
        assert!(label.contains("extra SendMessage"));
        assert!(label.contains("extra SendAttachment beyond the review-row handoff"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MENTION_PICKER_RICH_DIRECTORY_CONTROLS_LABEL));
    }

    #[test]
    fn mention_picker_preflight_detail_controls_label_keeps_buttons_local() {
        let label = mention_picker_preflight_detail_controls_label(
            Some("Request"),
            Some("ali"),
            3,
            Some("@alice"),
            2,
            true,
        );

        assert!(label.contains("Mention preflight detail Request"));
        assert!(label.contains("Request: @ali"));
        assert!(label.contains("Result: cached result ready with 3 cached suggestions"));
        assert!(label.contains("Error: local no-match/status only"));
        assert!(label.contains("Retry: active query cached"));
        assert!(label.contains("Source: loaded room_members rows 2"));
        assert!(label.contains("selected token: @alice"));
        assert!(label.contains("@room eligible"));
        assert!(label.contains("Remote member lookup"));
        assert!(label.contains("server-side directory search"));
        assert!(label.contains("profile/avatar fetch"));
        assert!(label.contains("attachment/edit mention payload"));
        assert!(label.contains("extra SendMessage"));
        assert!(label.contains("retry automation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_LABEL));
        assert!(
            MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("Request, Result, Error, Retry, and Source")
        );
        assert!(MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("active @query"));
        assert!(
            MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("existing SendMessage/add_mentions source")
        );
        assert!(MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("extra SendMessage"));
        assert!(MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("retry automation"));
        assert!(MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn mention_picker_preflight_detail_controls_label_uses_empty_fallbacks() {
        let label =
            mention_picker_preflight_detail_controls_label(None, None, 0, Some(""), 0, false);

        assert!(label.contains("Mention preflight detail ready"));
        assert!(label.contains("Request: no active @query"));
        assert!(label.contains("Result: no cached result with 0 cached suggestions"));
        assert!(label.contains("Retry: no active retry target"));
        assert!(label.contains("Source: loaded room_members rows 0"));
        assert!(label.contains("selected token: none"));
        assert!(label.contains("@room unavailable"));
        assert!(label.contains(MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_LABEL));
    }

    #[test]
    fn cached_member_suggestions_stay_empty_without_loaded_members() {
        assert!(cached_member_suggestions("hello @ali", None).is_empty());
    }
}
