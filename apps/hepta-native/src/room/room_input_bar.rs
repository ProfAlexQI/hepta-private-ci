//! The RoomInputBar widget contains all components related to sending messages/content to a room.
//!
//! The RoomInputBar is capped to a maximum height of 75% of the containing RoomScreen's height.
//!
//! The widgets included in the RoomInputBar are:
//! * a preview of the message the user is replying to.
//! * the location preview (which allows you to send your current location to the room),
//!   and a button to show the location preview.
//! * If TSP is enabled, a checkbox to enable TSP signing for the outgoing message.
//! * A MentionableTextInput, which allows the user to type a message
//!   and mention other users via the `@` key.
//! * Local Telegram-style attachment, emoji/sticker, and voice message affordances.
//! * A button to send the message.
//! * The editing pane, which is shown when the user is editing a previous message.
//! * A tombstone footer, which is shown if the room has been tombstoned (replaced).
//! * A "cannot-send-message" notice, which is shown if the user cannot send messages to the room.
//!

use std::{
    cell::RefCell,
    fs,
    io::Read,
    path::{Path, PathBuf},
};

use makepad_widgets::*;
use matrix_sdk::room::reply::{EnforceThread, Reply};
use matrix_sdk_ui::timeline::{EmbeddedEvent, EventTimelineItem, TimelineEventItemId};
use robius_location::Coordinates;
use ruma::{
    events::{
        Mentions,
        room::message::{
            AddMentions, LocationMessageEventContent, MessageType, ReplyWithinThread,
            RoomMessageEventContent, TextMessageEventContent,
        },
    },
    OwnedEventId, OwnedRoomId,
};
use crate::{
    app::PositiveConfirmationModalAction,
    hepta_composer::{looks_like_hepta_composer_command, plan_hepta_composer_command},
    home::{
        editing_pane::{EditingPaneState, EditingPaneWidgetExt, EditingPaneWidgetRefExt},
        location_preview::{LocationPreviewWidgetExt, LocationPreviewWidgetRefExt},
        room_screen::{MessageAction, RoomScreenProps, populate_preview_of_timeline_item},
        tombstone_footer::{SuccessorRoomDetails, TombstoneFooterWidgetExt},
    },
    location::init_location_subscriber,
    settings::app_preferences::{AppPreferencesGlobal, AppPreferencesAction},
    shared::{
        avatar::AvatarWidgetRefExt,
        confirmation_modal::ConfirmationModalContent,
        html_or_plaintext::HtmlOrPlaintextWidgetRefExt,
        mentionable_text_input::MentionableTextInputWidgetExt,
        popup_list::{PopupKind, enqueue_popup_notification},
        styles::*,
    },
    sliding_sync::{MatrixRequest, TimelineKind, UserPowerLevels, submit_async_request},
    utils,
};

pub const MESSAGE_SEND_OPERATION_STATUS_EVIDENCE: &str = "RoomInputBar submits text, reply, thread, confirmed location messages, and confirmed desktop attachments through the existing MatrixRequest send paths, while the Telegram send operation strip, queued/progress/failure labels, result bridge label, and Cancel controls update local evidence. Text sends attach compact Matrix Mentions through create_message_with_mentions, and attachment review-row Send now carries compact caption mentions through MatrixRequest::SendAttachment into AttachmentConfig.mentions. Attachment worker failure Retry is the one guarded exception: it reuses the cached last validated local file, caption, caption mentions, reply id, MIME, and TimelineKind only after PositiveConfirmationModal before resubmitting MatrixRequest::SendAttachment. Retry never auto-runs and Cancel does not abort or remove SDK send-queue work; neither emits room-state, membership, account, profile, gateway/runtime/auth, or live mutation request.";

pub const COMPOSER_TYPING_NOTICE_SEND_EVIDENCE: &str = "RoomInputBar submits the existing MatrixRequest::SendTypingNotice path only for plain composer text changes, while reserved Hepta command previews suppress Matrix typing notices and stay local preview state. The Telegram typing notice strip and set_typing_notice_status only update local labels; it emits no message send, room-state, retry, cancel, membership, account, profile, or extra Matrix request beyond the intended typing notice.";

pub const EMOJI_STICKER_SEND_LOCAL_BOUNDARY_EVIDENCE: &str = "RoomInputBar emoji/sticker choices only stage local preview status in the Telegram composer panel. Smile, Thumbs, Heart, Sticker, repeated selection, and Close do not insert composer text, attach sticker media, create sticker payloads, submit MatrixRequest::SendMessage, submit MatrixRequest::SendAttachment, upload media, start SDK send-queue work, send typing notices, or emit room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests.";
pub const EMOJI_STICKER_SEND_LOCAL_BOUNDARY_LABEL: &str =
    "Local emoji/sticker preview only; no composer insert, payload, upload, or send.";
pub const EMOJI_STICKER_LIFECYCLE_METADATA_EVIDENCE: &str = "RoomInputBar emoji/sticker lifecycle metadata stays local to the Telegram composer panel. Opening, repeated Smile/Thumbs/Heart/Sticker staging, Close, and reopen update only panel visibility, last staged choice, staged choice count, and close/reopen state. The metadata does not insert composer text, attach sticker media, create sticker payloads, submit MatrixRequest::SendMessage, submit MatrixRequest::SendAttachment, upload media, start SDK send-queue work, send typing notices, request a remote picker/search, or emit room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests.";
pub const EMOJI_STICKER_LIFECYCLE_METADATA_LABEL: &str = "Emoji/sticker lifecycle metadata: panel visibility, last choice, staged count, and close/reopen state stay local.";

pub const LOCATION_SEND_CONFIRMATION_EVIDENCE: &str = "RoomInputBar opens a local ConfirmationModal before any location message is submitted. The Location preview can fetch current coordinates, but opening the confirmation, Cancel, and guard display keep the location message unsent; only the confirmed accept handler emits LocationSendConfirmed and then submits the existing MatrixRequest::SendMessage location path. The guard sends no location SendMessage before confirmation, retry, cancel-location, extra message, room-state, membership, account, profile, or live mutation request.";

pub const ATTACHMENT_HANDOFF_CONFIRMATION_EVIDENCE: &str = "RoomInputBar opens a local ConfirmationModal before Photo or File attachments can open the native desktop file picker. Opening and canceling this guard stay local; accepting the guard can open rfd on desktop, and selecting a file only stages local pending attachment review until the user clicks Send. Camera, Contact, Close, picker cancel, and unsupported mobile picker states send no Matrix media, upload, message, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ATTACHMENT_CAMERA_CONTACT_LOCAL_BOUNDARY_EVIDENCE: &str = "RoomInputBar keeps Telegram Camera and Contact attachment choices as local-only placeholders while file_upload_send remains incomplete on cross-platform capture/share surfaces. Camera does not request camera or photo-library permission, capture media, create image/video payloads, write files, generate thumbnails, upload media, or submit MatrixRequest::SendAttachment. Contact does not request contacts permission, read an address book, create vCard/contact payloads, attach contact media, send a text fallback, or submit MatrixRequest::SendMessage. Camera, Contact, repeated selection, Close, and unsupported mobile picker states only update local preview/status/popup copy and emit no SDK send-queue work, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ATTACHMENT_CAMERA_CONTACT_LOCAL_BOUNDARY_LABEL: &str = "Camera/Contact placeholders: local preview only; no permissions, capture, contacts read, payload, upload, or send.";
pub const ATTACHMENT_MOBILE_PICKER_CONTROLS_EVIDENCE: &str = "RoomInputBar exposes Gallery, Camera, Files, Contact, Thumbnail, and Share as visible local mobile picker controls in the attachment picker while file_upload_send remains incomplete on cross-platform picker/capture/share surfaces. Clicking any control only updates local mobile picker boundary metadata and popup copy from current pending review state and local status. It does not request camera permission, photo-library permission, files provider permission, contacts permission, capture media, read contacts, open a mobile document picker, generate or decode thumbnails, open a system share sheet, create image/video/vCard/share payloads, upload media, submit MatrixRequest::SendAttachment, submit MatrixRequest::SendMessage, cancel pending review, mutate SDK send-queue work, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const ATTACHMENT_MOBILE_PICKER_CONTROLS_LABEL: &str = "Mobile picker controls: Gallery, Camera, Files, Contact, Thumbnail, and Share stay local; no permissions, picker, thumbnail decode, share sheet, payload, upload, or send.";
pub const ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_EVIDENCE: &str = "RoomInputBar exposes Share as a visible local attachment mobile picker control while file_upload_send still lacks a real mobile share-sheet handoff. Clicking Share only updates local attachment picker status, mobile picker boundary metadata, and popup copy from pending review state. It opens no system share sheet, invokes no platform share extension, reads no shared media item, creates no share payload, attaches no shared file, uploads nothing, submits no MatrixRequest::SendAttachment, submits no MatrixRequest::SendMessage, mutates no SDK send-queue work, and emits no gateway/runtime/auth or live mutation request.";
pub const ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_LABEL: &str = "Attachment Share stays a local mobile picker control; no system share sheet, share extension, payload, upload, send, or live mutation.";

pub const ATTACHMENT_PRE_SEND_REVIEW_EVIDENCE: &str = "RoomInputBar stages a selected desktop Photo/File attachment in local pending review state before MatrixRequest::SendAttachment is submitted. The selected filename, MIME type, file extension, local file size when available, caption preview, and reply context remain local until the user clicks Send in the attachment review row; Discard and Close clear the pending attachment locally. Selecting, reviewing, discarding, closing, picker cancel, and unsupported mobile picker states send no Matrix media, upload, message, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";

pub const ATTACHMENT_SELECTED_FILE_PREVIEW_EVIDENCE: &str = "RoomInputBar shows selected attachment filename, MIME type, file extension, local file size if available, caption preview, and reply-context status in the local attachment review surface before MatrixRequest::SendAttachment is submitted. This preview only uses the already selected local file path and composer state; it performs no upload, media decode, thumbnail generation, Matrix media send, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request. The final caption is still read from the composer only when the user clicks Send.";
pub const ATTACHMENT_SELECTED_IMAGE_METADATA_EVIDENCE: &str = "RoomInputBar shows selected desktop Photo image metadata in the local attachment review surface before MatrixRequest::SendAttachment is submitted. The preview uses the already selected local file path to show filename, MIME type, extension, local file size, and image dimensions status; lightweight PNG, JPEG, GIF, BMP, or WebP header dimensions can be displayed when available, otherwise dimensions stay unavailable. This performs no thumbnail decode, full image decode, media upload, Matrix media send, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ATTACHMENT_SELECTED_IMAGE_METADATA_LABEL: &str = "Selected image metadata: filename, MIME, extension, size, and dimensions status stay local until review Send.";

pub const ATTACHMENT_MAIN_SEND_GUARD_EVIDENCE: &str = "RoomInputBar guards the main composer Send button and Enter submit path while a selected attachment is pending review. Main Send/Enter only brings the local attachment review surface forward and tells the user to use the review-row Send; it does not send the caption as a plain text SendMessage, does not submit SendAttachment, does not clear the pending attachment, and emits no upload, media send, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";

pub const ATTACHMENT_SEND_RESULT_BRIDGE_EVIDENCE: &str = "RoomInputBar keeps the attachment send result bridge honest after review-row Send: MatrixRequest::SendAttachment hands the file to Timeline::send_attachment().use_send_queue(), async worker success returns a queued-only result to the composer operation strip and clears the cached retry attempt, and worker failure returns a failure-copy result plus the existing popup error path while retaining the cached last validated handoff. The composer operation strip does not claim delivery success, swallow failure, auto-resubmit SendAttachment, cancel SDK send-queue work, or emit room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ATTACHMENT_QUEUE_FAILURE_RECOVERY_COPY_EVIDENCE: &str = "RoomInputBar treats attachment queued, worker failure, Retry, and Cancel states as recovery copy around the existing SDK send queue handoff. After review-row Send submits MatrixRequest::SendAttachment, the worker reports only queued or immediate handoff failure back to the operation strip; SDK queue progress/error/sent state is rendered on the timeline local echo row, not wired into the composer recovery controls. Worker failure Retry reuses the cached last validated SendAttachment handoff only after PositiveConfirmationModal, preserving filename, MIME, file path, caption, caption mentions, reply id, and TimelineKind; it sends no caption-only SendMessage and never auto-runs. Cancel does not abort, remove, or cancel SDK send-queue work. Reopening Photo/File after a queued submit starts a new local review and never infers delivery success or failure for the earlier queue item; no room-state, membership, account/profile, gateway/runtime/auth, or live mutation request is emitted.";
pub const ATTACHMENT_SEND_FAILURE_RETRY_CONFIRMATION_EVIDENCE: &str = "RoomInputBar caches only the last validated attachment handoff after review-row Send submits MatrixRequest::SendAttachment. If the worker returns an immediate failure before SDK queue ownership, Retry opens PositiveConfirmationModal and only confirmed accept resubmits the same MatrixRequest::SendAttachment with cached TimelineKind, local file path, MIME type, caption content, compact caption mentions, and reply event id. Successful queued handoff clears the cache; missing cache and confirmation cancel stay local. This is not SDK queue retry/resume, upload abort, queue removal, delivery receipt mapping, caption-only SendMessage fallback, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const ATTACHMENT_SEND_FAILURE_RETRY_CONFIRMATION_LABEL: &str = "Attachment failure Retry confirms before resubmitting the cached SendAttachment handoff; SDK queue retry/resume/cancel stays unwired.";
pub const ATTACHMENT_TRUE_QUEUE_CONTROL_LOCAL_BOUNDARY_EVIDENCE: &str = "RoomInputBar makes the remaining file_upload_send queue-control gap explicit. Review-row Send already hands a desktop attachment to MatrixRequest::SendAttachment and Timeline::send_attachment().use_send_queue(), the worker returns queued or immediate failure to the operation strip, and RoomScreen renders SDK queue progress/error/sent state from the timeline local echo. The composer recovery strip can now confirm and resubmit only the cached immediate worker handoff failure, while accepted SDK queue cancel is available only from a timeline local echo row that still has a matrix-sdk-ui SendHandle. Timeline local echo Cancel submits MatrixRequest::AbortLocalSend and now returns TimelineUpdate::LocalSendAbortResult so the operation strip reflects canceled, already-sent/no-longer-cancellable, or failed abort results. Composer controls do not retry or resume accepted SDK queue uploads, abort uploads, remove queued media, or map SDK delivery receipts back into Retry/Cancel state. It does not retry or resume accepted SDK queue uploads from the composer. Composer Cancel remains local boundary evidence and emits no SDK queue abort/remove/cancel, caption-only SendMessage, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ATTACHMENT_TRUE_QUEUE_CONTROL_LOCAL_BOUNDARY_LABEL: &str = "Queue control gap: timeline shows SDK queue state; failed handoff Retry confirms before cached resubmit, and timeline local echo Cancel reports SendHandle abort results when a handle exists.";
pub const ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_EVIDENCE: &str = "RoomInputBar exposes visible accepted-SDK-queue action buttons for Pause, Resume, Reorder, Background, and Clear in the send operation strip, but every one of those controls is a local boundary state only. Background renders a local accepted attachment queue snapshot from the pending review summary, immediate handoff retry cache, and current local attachment status; Pause, Resume, Reorder, and Clear update local status/popup copy from the same already handed-off MatrixRequest::SendAttachment and Timeline::send_attachment().use_send_queue() boundary without retrying or resuming accepted SDK queue uploads, pausing uploads, aborting uploads, removing queued media, reordering SDK queue items, opening a background queue manager, clearing delivery receipts, resubmitting SendAttachment, sending a caption-only SendMessage, or emitting room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests.";
pub const ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_LABEL: &str = "Accepted SDK queue controls: Pause, Resume, Reorder, Background, and Clear stay local; only failed-handoff Retry can resubmit.";
pub const ATTACHMENT_ACCEPTED_QUEUE_TIMELINE_CANCEL_BRIDGE_EVIDENCE: &str = "RoomInputBar exposes Status, Handle, Timeline, Cancel, and Source as accepted-queue timeline-cancel bridge controls. These composer controls only explain whether the last operation is queued, whether pending review or failed-handoff retry metadata is loaded, and where real SDK abort can happen: the timeline local echo context menu exposes Cancel Send only while matrix-sdk-ui provides a local_echo_send_handle, and RoomScreen then submits MatrixRequest::AbortLocalSend for that exact SendHandle. SlidingSync returns TimelineUpdate::LocalSendAbortResult with canceled, already-sent/no-longer-cancellable, or failed status so the operation strip can reflect the real abort result. The bridge controls do not hold a SendHandle, do not abort uploads from the composer, do not remove queued media, do not retry/resume accepted queue items, do not resubmit SendAttachment, and emit no gateway/runtime/auth or live mutation.";
pub const ATTACHMENT_ACCEPTED_QUEUE_TIMELINE_CANCEL_BRIDGE_LABEL: &str = "Timeline cancel bridge: Status, Handle, Timeline, Cancel, and Source point to the real local-echo Cancel Send path when a SendHandle exists.";
pub const ATTACHMENT_LOCAL_SEND_ABORT_RESULT_EVIDENCE: &str = "RoomInputBar consumes TimelineUpdate::LocalSendAbortResult from the MatrixRequest::AbortLocalSend worker so the send operation strip reflects the actual SDK SendHandle::abort outcome from a timeline local echo: canceled, already sent/no longer cancellable, or failed. This result bridge is status-only in the composer; it does not hold a SendHandle, abort from composer controls, retry/resume accepted queue items, remove queued media, resubmit SendAttachment, send caption-only SendMessage, mutate room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const ATTACHMENT_LOCAL_SEND_ABORT_RESULT_LABEL: &str = "Local send abort result: timeline Cancel Send reports canceled/already-sent/failed back to the operation strip.";
pub const ATTACHMENT_PER_FILE_STATUS_CONTROLS_EVIDENCE: &str = "RoomInputBar exposes visible per-file Status, Progress, Pause, Resume, Cancel, Retry, Drilldown, Contract, and Taxonomy controls in the send operation strip after the existing SendAttachment/use_send_queue handoff. Each control only derives copy from the current local pending review state, cached immediate handoff retry availability, latest operation status label, and accepted-send queue acceptance fields. Contract renders a typed SDK queue control/progress/result/error/delivery receipt/background/multi-file acceptance contract from the same local drilldown state. Taxonomy records accepted queue/progress/result slots locally before real queue controls can be promoted. This does not inspect or mutate SDK queue entries, subscribe to upload progress, pause uploads, resume uploads, abort uploads, remove queued media, retry accepted SDK queue items, resubmit SendAttachment, send caption-only SendMessage, map delivery receipts, or emit room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests. The only real retry remains the failed-handoff Retry confirmation path.";
pub const ATTACHMENT_PER_FILE_STATUS_CONTROLS_LABEL: &str = "Per-file status controls: Status, Progress, Pause, Resume, Cancel, Retry, Drilldown, Contract, and Taxonomy stay local; failed-handoff Retry confirmation remains the only resubmit path.";
pub const ATTACHMENT_PER_FILE_QUEUE_DRILLDOWN_EVIDENCE: &str = "RoomInputBar per-file Drilldown renders a local accepted-send queue acceptance matrix from pending review metadata, cached immediate handoff retry readiness, and the latest local operation status. The matrix names queue item identity, file metadata, progress slot, pause/resume/cancel eligibility, retry eligibility, timeline local-echo cancel handle, result/error slot, delivery receipt mapping, background persistence, and reorder/grouping slots as acceptance fields only. It performs no SDK queue lookup, progress subscription, upload pause/resume/cancel, accepted queue retry, SendAttachment resubmit, caption-only SendMessage, gateway/runtime/auth call, or live mutation.";
pub const ATTACHMENT_PER_FILE_QUEUE_DRILLDOWN_LABEL: &str = "Per-file queue drilldown: accepted-send queue identity/progress/control/result acceptance fields stay local and do not touch SDK queue state.";
pub const ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_EVIDENCE: &str = "RoomInputBar per-file Contract renders a local typed SDK queue contract from the per-file drilldown, pending review metadata, cached immediate handoff retry readiness, and latest local operation status. The contract names queue item/local echo identity, file metadata, upload progress bytes/percent/speed/ETA slots, pause/resume/cancel/retry/reorder/remove eligibility, SendHandle and AbortLocalSend boundary, result states, error taxonomy, delivery receipt mapping, background persistence, multi-file album grouping, idempotency, stale-handle handling, and promotion blockers before real accepted SDK queue controls can be wired. It performs no SDK queue lookup, progress subscription, upload pause/resume/cancel, accepted queue retry, queue reorder/remove, delivery receipt read, SendAttachment resubmit, caption-only SendMessage, gateway/runtime/auth call, or live mutation.";
pub const ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_LABEL: &str = "Per-file SDK queue contract maps Drilldown to typed progress/control/result/error acceptance locally.";
pub const ATTACHMENT_QUEUE_PROGRESS_RESULT_TAXONOMY_PACKET_EVIDENCE: &str = "RoomInputBar per-file Taxonomy renders a local accepted queue/progress/result taxonomy packet from the pending review metadata, cached immediate handoff retry readiness, latest local operation status, and existing timeline local-echo cancel boundary. The packet names current live references as review-row MatrixRequest::SendAttachment, Timeline::send_attachment().use_send_queue(), timeline local echo progress/error/sent rendering, MatrixRequest::AbortLocalSend plus TimelineUpdate::LocalSendAbortResult, and confirmed failed-handoff Retry only. It records accepted queue operation id, queue item/local echo identity, progress subscription, delivery receipt, pause/resume, accepted-queue retry, cancel ownership, reorder/remove, background persistence, stale SendHandle, and audit redaction slots as not_wired before backend accepted queue promotion. It performs no SDK queue lookup, progress subscription, upload pause/resume/cancel, accepted queue retry, queue reorder/remove, delivery receipt read, SendAttachment resubmit, caption-only SendMessage, gateway/runtime/auth call, or live mutation.";
pub const ATTACHMENT_QUEUE_PROGRESS_RESULT_TAXONOMY_PACKET_LABEL: &str = "Per-file Taxonomy records accepted queue/progress/result slots locally; timeline local echo Cancel and failed-handoff Retry remain the only live recovery paths.";
pub const ATTACHMENT_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE: &str = "RoomInputBar exposes visible Request, Result, Error, Retry, and Source controls in the send operation strip around the existing attachment review-row SendAttachment handoff. Each detail control only summarizes current local pending review state, latest local operation status, cached immediate handoff failure text, retry-cache readiness, and existing result-bridge/source copy; it does not submit MatrixRequest::SendAttachment, retry accepted SDK queue items, subscribe to upload progress, inspect queue entries, abort uploads, remove queued media, cancel SDK send-queue work, send a caption-only SendMessage, duplicate upload, map delivery receipts, or emit room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests. The only real SendAttachment paths remain review-row Send and failed-handoff Retry after PositiveConfirmationModal.";
pub const ATTACHMENT_SEND_PREFLIGHT_DETAIL_CONTROLS_LABEL: &str = "Attachment send preflight detail controls: Request, Result, Error, Retry, and Source stay local; review Send and confirmed failed-handoff Retry remain the only SendAttachment paths.";
pub const ATTACHMENT_MULTI_FILE_QUEUE_BOUNDARY_EVIDENCE: &str = "RoomInputBar attachment multi-file/album queue boundary metadata makes the next file_upload_send queue gap explicit while preserving the existing single selected-file review SendAttachment path. Multiple-file selection, album grouping, per-file progress rows, background upload list, reorder/remove queued items, bulk retry, accepted SDK queue retry/resume/cancel, delivery receipt fan-in, and queue persistence across room switches remain local blocked controls. The metadata is derived only from local pending review state and the cached immediate handoff retry state. It sends no extra file picker request, no additional SendAttachment, no caption-only SendMessage, no SDK queue abort/remove/cancel, no room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ATTACHMENT_MULTI_FILE_QUEUE_BOUNDARY_LABEL: &str = "Multi-file, album, per-file progress, background queue, reorder/remove, bulk retry, and accepted SDK queue controls stay local blocked.";
pub const ATTACHMENT_QUEUE_FAILURE_RECOVERY_RESULT_BRIDGE_LABEL: &str = "Result bridge: worker reports queued or immediate failure after SendAttachment handoff; timeline local echo shows SDK queue state; failed handoff Retry confirms cached resubmit; Cancel never cancels SDK queue work.";
pub const ATTACHMENT_STATUS_TAXONOMY_LOCAL_EVIDENCE: &str = "RoomInputBar keeps attachment status labels stable across the local review, validation, handoff, queued, and recovery surfaces. The taxonomy is review-pending, review-replaced, review-preserved, validation-held, handoff-submitted, queued-only, failure-copy, retry-confirmation-open, retry-confirmed, empty-held, discarded-local, closed-local, retry-local, and cancel-local. Only handoff-submitted and retry-confirmed emit MatrixRequest::SendAttachment; queued-only is not delivery success, Retry never auto-runs, and every other taxonomy state updates local review/status/popup evidence without caption-only SendMessage, duplicate upload, SDK queue abort/remove/cancel, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ATTACHMENT_STATUS_TAXONOMY_LABEL: &str = "Attachment status taxonomy: review-pending -> validation-held -> handoff-submitted -> queued-only; failure-copy may open retry-confirmation-open then retry-confirmed; empty-held, retry-local, cancel-local, discarded-local, and closed-local stay local evidence.";
pub const ATTACHMENT_REVIEW_ROW_COMPACT_FIT_EVIDENCE: &str = "RoomInputBar keeps attachment review-row and operation-strip evidence compact on desktop and narrow mobile layouts. Attachment filename, metadata, caption/reply context, taxonomy, result bridge, retry-confirmation label, and validation warning labels use wrapping Fill/Fit text surfaces, while Send, Discard, Retry, and Cancel remain explicit button affordances in wrapped action rows. Compact fit never changes the send boundary: only review-row Send with pending state or confirmed failed-handoff Retry can submit MatrixRequest::SendAttachment; layout wrapping, overflow prevention, and mobile fit evidence do not emit caption-only SendMessage, duplicate upload, SDK queue abort/remove/cancel, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ATTACHMENT_REVIEW_ROW_COMPACT_FIT_LABEL: &str = "Compact fit: attachment review/status text wraps in Fill/Fit labels; Send, Discard, Retry, and Cancel stay explicit wrapped buttons with no extra send or queue-cancel request.";
pub const ATTACHMENT_MOBILE_ACTION_DENSITY_EVIDENCE: &str = "RoomInputBar keeps attachment review and operation recovery actions usable on narrow mobile layouts by routing Send, Discard, Retry, Cancel, Close, Photo, File, Camera, and Contact through the shared TelegramAttachmentOptionButton density. The shared button sets a stable 36px touch-height, 8/12 padding, wrapped action rows, icon+label affordances, and no hidden overflow send affordance. Action density evidence is visual/local only: it does not change the send boundary, does not emit SendAttachment outside pending review-row Send or confirmed failed-handoff Retry, does not send caption-only SendMessage, does not duplicate upload automatically, does not abort/remove/cancel SDK send-queue work, and emits no room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ATTACHMENT_MOBILE_ACTION_DENSITY_LABEL: &str = "Action density: Send/Discard/Retry/Cancel use shared 36px wrapped icon+label buttons on mobile; density changes no send, retry, upload, or SDK queue-cancel behavior.";

pub const ATTACHMENT_SELECTION_REPLACEMENT_PRESERVE_EVIDENCE: &str = "RoomInputBar treats repeated Photo/File selection while an attachment is pending review as local replacement of pending review state only, and picker cancel as preservation of the existing pending attachment. Replacing the selected file does not upload or send the previous file, does not submit a cancel for the previous pending review, does not clear caption/reply context unless review Send later consumes them, and emits no SendAttachment, SendMessage, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";

pub const ATTACHMENT_REVIEW_LIFECYCLE_METADATA_EVIDENCE: &str = "RoomInputBar shows attachment review lifecycle metadata for Select, Replace, picker cancel, Close, and Discard from local pending review state: kind, filename, MIME type, local size, caption preview, reply context, validation warning, and previous pending filename when replacing. This metadata only updates popup/status/review copy; it does not open a picker beyond the confirmed handoff, upload media, submit MatrixRequest::SendAttachment, send caption-only SendMessage, retry or cancel SDK send-queue work, mutate room-state or membership, touch account/profile, call gateway/runtime/auth, or perform live mutation.";
pub const ATTACHMENT_REVIEW_LIFECYCLE_METADATA_LABEL: &str = "Attachment review lifecycle metadata: local pending/replaced/closed/discarded state only; no upload, SendAttachment, SDK queue cancel, or live mutation.";

pub const ATTACHMENT_REVIEW_SEND_SINGLE_SUBMIT_EVIDENCE: &str = "RoomInputBar consumes telegram_pending_attachment_send with Option::take() before review-row Send submits MatrixRequest::SendAttachment, so a second click or empty review Send has no pending attachment to submit. Empty/duplicate review Send only updates local attachment status, popup, and operation strip; it does not submit duplicate SendAttachment, does not send the caption as SendMessage, does not upload media, does not cancel SDK send-queue work, and emits no room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";

pub const ATTACHMENT_REVIEW_DISCARD_CLOSE_IDEMPOTENT_EVIDENCE: &str = "RoomInputBar treats attachment review Discard and picker Close as idempotent local pending-review cleanup. Discard and Close consume telegram_pending_attachment_send with Option::take(), clear only the local pending attachment, preserve composer caption/reply text unless a later normal send path consumes it, and make later review-row Send fall into the empty local guard. Repeated Discard, empty Discard, empty Close, and review-row Send after Discard/Close only update local status, popup, and operation strip; they submit no SendAttachment, no caption-only SendMessage, no upload, no SDK send-queue cancel, and no room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ATTACHMENT_CAPTION_REPLY_CONTEXT_BOUNDARY_EVIDENCE: &str = "RoomInputBar keeps attachment caption and reply context local and explicit while a selected Photo/File waits in pending review. Caption preview live-updates from the composer text, main Send/Enter preserves the pending attachment plus composer caption/reply preview, and Discard/Close clear only telegram_pending_attachment_send while preserving composer caption/reply text. Empty or duplicate review-row Send after cleanup does not clear or send caption text. Review-row Send with a pending attachment is the only attachment path that consumes the current composer caption into MatrixRequest::SendAttachment, reuses MentionableTextInput::mentions_for_text to attach compact Matrix Mentions through AttachmentConfig.mentions, carries the captured reply/thread event id, then clears composer text and reply preview after submit; it does not emit caption-only SendMessage, extra SendAttachment, upload outside the SDK send queue, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const ATTACHMENT_FILE_VALIDATION_LOCAL_ERROR_EVIDENCE: &str = "RoomInputBar performs a final local attachment file validation before review-row Send submits MatrixRequest::SendAttachment. If the selected path is unreadable, not a regular file, or an empty file, the pending attachment remains in local review with an Attachment validation held locally status, composer caption/reply text is preserved, and no SendAttachment, caption-only SendMessage, upload, SDK send-queue cancel, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request is emitted. MIME fallback to application/octet-stream and size unavailable states remain visible local metadata in the review surface before any send.";
pub const ATTACHMENT_VALIDATION_ERROR_RECOVERY_EVIDENCE: &str = "RoomInputBar treats attachment validation errors as recoverable local review state. After an unreadable, non-file, or empty-file validation failure, the pending attachment stays visible with a validation warning; choosing Photo/File again replaces the local pending review and clears only that local warning, while Discard and Close clear the pending review plus warning locally. Retry/Cancel controls remain local evidence and do not revalidate, resubmit SendAttachment, send a caption-only SendMessage, upload media, cancel SDK send-queue work, or emit room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests.";

pub const VOICE_MESSAGE_SEND_LOCAL_BLOCKED_EVIDENCE: &str = "RoomInputBar keeps Telegram voice Record, Lock, Cancel, and Close as local blocked preview controls while voice_message_send remains a base gap. Voice Send can open a desktop audio-file confirmation and reuse the existing pending review plus MatrixRequest::SendAttachment handoff for an already selected local audio file, but it never requests microphone permission, starts recording, stores a captured audio payload, encodes media, sends a caption/text fallback, starts hidden SDK queue work before review Send, or emits room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests. Cancel, repeated Close, Record, Lock, and reopening the voice surface only update local status, labels, and popup copy.";
pub const VOICE_MESSAGE_SEND_LOCAL_BLOCKED_LABEL: &str = "Voice send: desktop audio file can use confirmed review SendAttachment; Record/Lock/mic capture stay local.";
pub const VOICE_MESSAGE_PERMISSION_RECORDING_LOCAL_BOUNDARY_EVIDENCE: &str = "RoomInputBar voice permission/recording controls keep Record, Lock, Cancel, Close, waveform, and timer as local-only preview evidence while voice_message_send remains a base gap. They never request microphone permission, privacy entitlement, audio session activation, platform recorder, captured local audio file creation, temporary file write for recording, waveform sampling, duration capture, opus/aac encoding, captured media upload, MatrixRequest::SendMessage text fallback, SDK send-queue work before confirmed review Send, room-state, membership, account/profile, gateway/runtime/auth, or live mutation requests. Voice Send is limited to confirmed desktop audio-file selection and pending review before MatrixRequest::SendAttachment.";
pub const VOICE_MESSAGE_PERMISSION_RECORDING_LOCAL_BOUNDARY_LABEL: &str = "Voice permission boundary: no mic permission, audio session, recorder, local file, waveform capture, encoder, upload, send queue, or live mutation.";
pub const VOICE_MESSAGE_SELECTED_AUDIO_METADATA_EVIDENCE: &str = "RoomInputBar shows selected desktop audio-file metadata in the local review surface before Voice Send can submit MatrixRequest::SendAttachment. The preview uses the already selected local file path to show filename, MIME type, extension, local file size, duration status, codec/container status, and bounded local WAV PCM waveform peaks when available. Simple WAV header duration and PCM peaks can be displayed from capped local bytes; otherwise duration, codec, or waveform stay visibly unavailable. This sends no microphone permission request, privacy entitlement change, audio session activation, platform recorder, captured local audio file creation, temporary recording write, recorder waveform capture, media decode/player startup, opus/aac encoding, upload, hidden SDK send-queue work before review Send, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const VOICE_MESSAGE_SELECTED_AUDIO_METADATA_LABEL: &str = "Selected audio metadata: filename, MIME, extension, size, duration, codec, and bounded WAV waveform peaks stay local until review Send.";
pub const VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_EVIDENCE: &str = "RoomInputBar promotes only the already selected desktop Voice attachment into local waveform/codec evidence. WAV files are parsed from capped local bytes for RIFF/fmt/data metadata, codec name, sample rate, channel count, bit depth, data bytes, and coarse PCM peak buckets; non-WAV or unsupported WAV codecs report a local unavailable waveform state. The path never requests microphone permission, starts a recorder, writes a captured file, decodes compressed media, starts playback, transcodes, uploads, submits SendAttachment before review Send, sends a text fallback, mutates SDK queue state, room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_LABEL: &str = "Selected-audio waveform/codec: capped local WAV header + PCM peaks only; no recorder, decode, playback, upload, or live mutation.";
pub const VOICE_MESSAGE_LIFECYCLE_METADATA_EVIDENCE: &str = "RoomInputBar voice lifecycle metadata reuses only local voice panel visibility, local Record/Lock/Cancel/Close status, confirmation and picker state copy, pending selected desktop audio filename, duration status, and reply-context state. Record, Lock, Cancel, Close, reopen, confirmation cancel, picker cancel, unsupported picker, selected audio review, and repeated status repaint update only local labels/popup copy. Send opens the existing confirmation before the desktop audio picker; selected audio still enters the existing attachment review row before MatrixRequest::SendAttachment. This sends no microphone permission request, privacy entitlement change, audio session activation, platform recorder, captured local audio file creation, temporary recording write, waveform sampling, duration capture from a recorder, media decode, player startup, opus/aac encoding, captured media upload, SendMessage text fallback, hidden SDK send-queue work before review Send, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const VOICE_MESSAGE_LIFECYCLE_METADATA_LABEL: &str = "Voice lifecycle metadata: local panel state, control status, confirmation/picker state, pending audio filename, duration status, and reply context stay local until review Send.";
pub const VOICE_MESSAGE_CONFIRMATION_CANCEL_METADATA_EVIDENCE: &str = "RoomInputBar voice Send confirmation cancel uses a local RoomInputBarAction to repaint only the voice panel and attachment review preview state after PositiveConfirmationModal cancel. If a pending attachment already exists it is preserved; otherwise the waiting picker preview is hidden. Cancel sends no desktop picker request, microphone permission request, recorder/audio-session work, local recording file creation, waveform sampling, encoder work, upload, SendAttachment, SendMessage fallback, SDK queue cancel, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.";
pub const VOICE_MESSAGE_CONFIRMATION_CANCEL_METADATA_LABEL: &str = "Voice confirmation cancel repaints local voice/picker state only; no picker, mic, upload, queue cancel, or live mutation.";
pub const VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE: &str = "RoomInputBar keeps the voice recorder, waveform capture/render, encoder/codec selection, opus/ogg/amr conversion, silence trimming, transcription, playback scrubber, upload progress, background recording, attachment/edit voice payload, and hidden SDK queue controls as local blocked evidence while voice_message_send remains a base gap. Record, Lock, Cancel, Close, confirmation cancel, and status repaint send no microphone permission prompt, privacy entitlement change, audio session activation, platform recorder, captured file write, media decode, codec/transcription service request, upload progress subscription, SendAttachment, SendMessage fallback, account/profile, room-state, membership, gateway/runtime/auth, or live mutation request. The existing positive Voice Send path still only chooses an already selected desktop audio file, stages attachment review, and can submit MatrixRequest::SendAttachment from that review row.";
pub const VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_LABEL: &str = "Voice recorder boundary: waveform, codec conversion, trimming, transcription, scrubber, upload progress, background recording, and SDK queue controls stay local blocked.";
pub const VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE: &str = "RoomInputBar exposes Timer, Waveform, Transcript, Progress, and Codec as visible local voice recorder status controls while voice_message_send remains a base gap. Clicking any of those controls only updates local voice panel status, static meter/timer copy, boundary metadata, and popup text derived from panel visibility and pending desktop audio review state. Waveform and Codec can additionally summarize the already selected desktop WAV file with capped local RIFF/fmt/data parsing and coarse PCM peak buckets. The controls request no microphone permission, privacy entitlement, audio session activation, platform recorder, captured local audio file, temporary recording write, recorder waveform sampling, transcript service, codec conversion, upload progress subscription, SDK queue control, SendAttachment, SendMessage fallback, account/profile, room-state, membership, gateway/runtime/auth, or live mutation.";
pub const VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_LABEL: &str =
    "Voice recorder status controls: Timer, Waveform, Transcript, Progress, and Codec stay local.";
pub const VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE: &str = "RoomInputBar exposes Permission, Capture, Encode, Review, Upload, Packet, Contract, and Taxonomy as visible local voice capture lifecycle controls while voice_message_send remains a base gap. Permission and Upload render a local voice capture/request packet snapshot from panel visibility, pending desktop audio review, retry state, local status, and source copy; Packet renders a recorder lifecycle drilldown packet with microphone/recorder acceptance criteria; Contract maps that drilldown to typed microphone permission, recorder session, capture file, waveform, codec, transcription, review, mobile picker, upload queue, and SendAttachment result contracts; Taxonomy records permission/capture/encode/review/upload result slots locally before recorder or captured-upload work can be promoted. Capture, Encode, and Review only update local voice panel status, capture lifecycle metadata, and popup copy from the same local state. Permission requests no microphone permission or privacy entitlement; Capture starts no platform recorder, audio session, captured local audio file, temporary recording write, waveform sampling, or duration capture; Encode performs no codec conversion, media decode, silence trimming, or transcription; Review does not create a captured voice payload or edit/attachment voice payload; Upload submits no SendAttachment, SendMessage fallback, SDK send-queue work, upload progress subscription, room-state, membership, account/profile, gateway/runtime/auth, or live mutation. The only positive voice path remains confirmed desktop audio-file review SendAttachment.";
pub const VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_LABEL: &str = "Voice capture lifecycle controls: Permission, Capture, Encode, Review, Upload, Packet, Contract, and Taxonomy stay local; Packet persists microphone/recorder acceptance criteria; Contract maps typed recorder/upload contracts; Taxonomy records permission/capture/upload result slots; confirmed desktop audio review Send remains the only positive path.";
pub const VOICE_MESSAGE_MOBILE_PICKER_CONTROLS_EVIDENCE: &str = "RoomInputBar exposes Mic, Files, Library, Retake, and Share as visible local voice mobile picker controls while voice_message_send remains a base gap. Clicking any control only updates local voice panel status, mobile picker metadata, and popup copy from panel visibility plus pending desktop audio review state. Mic requests no mobile microphone permission or privacy entitlement; Files opens no mobile document picker; Library opens no photo/audio library picker; Retake deletes no captured clip and starts no new capture session; Share opens no system share sheet or external handoff. The controls create no captured voice payload, read no mobile media, submit no SendAttachment, SendMessage fallback, SDK send-queue work, upload progress subscription, account/profile, room-state, membership, gateway/runtime/auth, or live mutation. The only positive voice path remains confirmed desktop audio-file review SendAttachment.";
pub const VOICE_MESSAGE_MOBILE_PICKER_CONTROLS_LABEL: &str = "Voice mobile picker controls: Mic, Files, Library, Retake, and Share stay local; no mobile permission, picker, capture, share sheet, upload, or live mutation.";
pub const VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_EVIDENCE: &str = "RoomInputBar exposes Play, Pause, Scrub, Speed, and Drop as visible voice review playback controls while voice_message_send remains a base gap. Play is a narrow local OS handoff: when a pending desktop Voice attachment review still has a readable regular local file, it converts the path to a file URL and asks the system opener to play it; missing, stale, or non-file pending audio stays warning-only. Pause, Scrub, and Speed only update local voice review metadata and popup copy from panel visibility plus pending desktop audio review filename, duration status, and latest voice status. Drop is a real local cleanup handoff: it consumes only a pending desktop Voice attachment review with Option::take(), clears voice failed-handoff retry metadata, preserves composer caption/reply text, leaves Photo/File pending attachments untouched, and deletes no local file. The controls start no inline audio player, media decode, waveform sampling, playback position subscription, speed transform, scrubber timeline, local file deletion, SendAttachment, SendMessage fallback, SDK send-queue work, upload progress subscription, room-state, membership, account/profile, gateway/runtime/auth, or live mutation. The only positive voice network path remains confirmed desktop audio-file review SendAttachment.";
pub const VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_LABEL: &str = "Voice review playback controls: Play opens pending desktop audio with the system opener; Pause/Scrub/Speed stay local metadata; Drop is a real pending-audio cleanup handoff.";
pub const VOICE_MESSAGE_REVIEW_DROP_PENDING_AUDIO_EVIDENCE: &str = "RoomInputBar voice review Drop now performs a narrow local pending-audio cleanup. If the current pending attachment is Voice, Drop consumes telegram_pending_attachment_send with Option::take(), clears only voice failed-handoff retry metadata, preserves composer caption/reply text, hides no file on disk, and makes repeated review-row Send fall into the existing empty local guard. If no pending Voice review exists, Drop only updates local voice status and popup copy. It does not discard Photo/File pending attachments, delete local files, open an audio player, decode media, submit SendAttachment, send a caption-only SendMessage, abort or remove SDK send-queue work, mutate room-state, membership, account/profile, gateway/runtime/auth, or live mutation.";
pub const VOICE_MESSAGE_REVIEW_DROP_PENDING_AUDIO_LABEL: &str = "Voice Drop is a real pending-audio cleanup handoff: it clears only pending voice review and voice retry metadata; no file deletion, SendAttachment, SDK queue cancel, or live mutation.";
pub const VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE: &str = "RoomInputBar exposes Request, Result, Error, Retry, and Source as visible local Voice SendAttachment preflight detail controls in the voice message panel. Request renders a local voice capture/request packet snapshot; Result, Error, Retry, and Source summarize only current voice panel visibility, pending desktop audio review filename and duration status, latest local voice status, cached immediate attachment handoff failure text, retry-cache readiness, and source/evidence copy from the existing attachment send bridge. The controls request no microphone permission, privacy entitlement, audio session activation, platform recorder, captured local audio file, temporary recording write, waveform sampling, transcription service, codec conversion, upload progress subscription, SDK queue control, extra MatrixRequest::SendAttachment, unconfirmed retry, SendMessage fallback, account/profile, room-state, membership, gateway/runtime/auth, or live mutation. The only real Voice SendAttachment paths remain confirmed desktop audio picker -> attachment review-row Send and failed-handoff Retry after PositiveConfirmationModal.";
pub const VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_LABEL: &str = "Voice Send preflight detail controls: Request snapshot, Result, Error, Retry, and Source stay local; confirmed desktop audio review Send and confirmed failed-handoff Retry remain the only Voice SendAttachment paths.";
pub const VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_EVIDENCE: &str = "RoomInputBar exposes a visible Packet control in the local voice capture lifecycle row while voice_message_send remains a base gap. Packet renders a recorder lifecycle drilldown from only voice panel visibility, pending desktop audio review filename and duration status, local voice status, retry-cache readiness, cached immediate handoff error text, and source/evidence copy. The packet persists microphone permission, privacy entitlement, audio session, recorder start/lock/cancel, temporary capture file lifecycle, waveform sampling/rendering, timer/duration capture, codec/encoding/transcription, review playback/drop cleanup, mobile picker/share sheet, upload queue, result/error/retry/source, and confirmed desktop audio review SendAttachment acceptance criteria as local metadata only. It requests no microphone permission, privacy entitlement, audio session activation, platform recorder, captured local audio file, temporary recording write, waveform sampling, duration capture, media decode, audio player, codec conversion, transcription service, upload progress subscription, SDK queue control, extra MatrixRequest::SendAttachment, unconfirmed retry, SendMessage fallback, account/profile, room-state, membership, gateway/runtime/auth, or live mutation.";
pub const VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_LABEL: &str = "Voice recorder lifecycle drilldown packet: microphone, recorder, waveform, codec, review, mobile picker, upload, retry, result, and source acceptance criteria stay local.";
pub const VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE: &str = "RoomInputBar exposes a visible Contract control in the local voice capture lifecycle row while voice_message_send remains a base gap. Contract renders a typed recorder/upload acceptance packet from the recorder lifecycle drilldown, voice panel visibility, pending desktop audio review filename and duration status, local voice status, retry-cache readiness, cached immediate handoff error text, and source/evidence copy. The contract names microphone permission and privacy entitlement request/result/error slots, audio session and recorder session lifecycle, capture file identity and cleanup, waveform/timer sampling, codec/encoding/transcription results, review playback/drop cleanup, mobile picker/share sheet handoff, upload queue progress/result/error/retry/source slots, confirmed desktop audio review SendAttachment result mapping, stale capture handling, idempotency, and adapter promotion blockers before microphone, recorder, waveform, codec, transcription, mobile picker, or captured upload work can be wired. It requests no microphone permission, privacy entitlement, audio session activation, platform recorder, captured local audio file, temporary recording write, waveform sampling, duration capture, media decode, audio player, codec conversion, transcription service, upload progress subscription, SDK queue control, extra MatrixRequest::SendAttachment, unconfirmed retry, SendMessage fallback, account/profile, room-state, membership, gateway/runtime/auth, or live mutation.";
pub const VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_LABEL: &str = "Voice recorder typed contract packet: microphone permission, recorder session, capture, waveform, codec, transcription, review, mobile picker, upload queue, and SendAttachment result contracts stay local.";
pub const VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE: &str = "RoomInputBar exposes a visible Taxonomy control in the local voice capture lifecycle row while voice_message_send remains a base gap. Taxonomy renders a local permission/capture/upload result taxonomy packet from voice panel visibility, pending desktop audio review filename and duration status, local voice status, retry-cache readiness, cached immediate handoff error text, and source/evidence copy. The packet names the only current live result references as confirmed desktop audio review MatrixRequest::SendAttachment, Timeline::send_attachment().use_send_queue(), local failed-handoff Retry after PositiveConfirmationModal, selected-audio bounded WAV duration/codec/waveform analysis, review Play local system-opener handoff, and Drop pending-audio local cleanup. It records microphone permission operation id, privacy entitlement result, audio session id, recorder session id, capture file identity, waveform/timer sampling, codec/transcription result, review player state, mobile picker/share sheet result, captured upload queue item, delivery result, stale capture, retry/cancel, and audit redaction slots as not_assigned or not_wired before recorder or captured-upload work can be promoted. It requests no microphone permission, privacy entitlement, audio session activation, platform recorder, captured local audio file, temporary recording write, waveform sampling, duration capture, media decode, inline audio player, codec conversion, transcription service, upload progress subscription, SDK queue control, extra MatrixRequest::SendAttachment, unconfirmed retry, SendMessage fallback, account/profile, room-state, membership, gateway/runtime/auth, or live mutation.";
pub const VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_LABEL: &str = "Voice recorder result taxonomy packet: permission, recorder, capture, waveform, codec, transcription, review, mobile picker, upload queue, delivery, retry, cancel, stale, and audit results stay local.";

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*


    mod.widgets.ICO_LOCATION_PERSON = crate_resource("self://resources/icons/location-person.svg")

    mod.widgets.TelegramAttachmentOptionButton = RobrixNeutralIconButton {
        width: Fit
        height: 36
        margin: 0
        spacing: 7
        padding: Inset{top: 8, bottom: 8, left: 12, right: 12}
        draw_bg +: {
            color: (COLOR_TELEGRAM_PANEL)
            color_hover: #xFFFFFF14
            color_down: (COLOR_TELEGRAM_DIALOG_ACTIVE)
            border_color: (COLOR_TELEGRAM_GLASS_HAIRLINE)
            border_color_hover: (COLOR_TELEGRAM_BLUE)
            border_color_down: (COLOR_TELEGRAM_BLUE)
            border_size: 1.0
            border_radius: 16.0
        }
        draw_icon.color: (COLOR_TELEGRAM_BLUE)
        icon_walk: Walk{width: 14, height: 14}
        draw_text +: {
            color: (COLOR_TELEGRAM_TEXT)
            color_hover: (COLOR_TELEGRAM_TEXT)
            color_down: (COLOR_TELEGRAM_TEXT)
            text_style: theme.font_bold { font_size: 10.5 }
        }
    }

    mod.widgets.RoomInputBar = set_type_default() do #(RoomInputBar::register_widget(vm)) {
        ..mod.widgets.RoundedView

        width: Fill,
        height: Fit{max: FitBound.Rel{base: Base.Full, factor: 0.75}}
        flow: Down,

        margin: Inset{left: 0, right: 0, bottom: 0}
        show_bg: true,
        draw_bg +: {
            color: (COLOR_TELEGRAM_PANEL)
            border_radius: 14.0
            border_color: (COLOR_TELEGRAM_GLASS_HAIRLINE)
            border_size: 1.0
        }

        // The top-most element is a preview of the message that the user is replying to, if any.
        replying_preview := ReplyingPreview { }

        // Below that, display a preview of the current location that a user is about to send.
        location_preview := LocationPreview { }

        hepta_command_preview := RoundedView {
            visible: false,
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 4.0,
            margin: Inset{top: 4, left: 8, right: 8, bottom: 2}
            padding: Inset{top: 8, bottom: 8, left: 10, right: 10}
            show_bg: true,
            draw_bg +: {
                color: (COLOR_TELEGRAM_INPUT)
                border_color: (COLOR_TELEGRAM_GLASS_HAIRLINE)
                border_size: 1.0
                border_radius: 6.0
            }

            title := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_TEXT),
                    text_style: theme.font_bold { font_size: 12.0 }
                }
                text: "Hepta dry-run preview"
            }

            body := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_TEXT),
                    text_style: theme.font_regular { font_size: 11.0 }
                }
                text: ""
            }

            meta := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_MUTED),
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: ""
            }
        }

        send_operation_status := RoundedView {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 3.0,
            margin: Inset{top: 4, left: 8, right: 8, bottom: 2}
            padding: Inset{top: 7, bottom: 7, left: 10, right: 10}
            show_bg: true,
            draw_bg +: {
                color: (COLOR_TELEGRAM_INPUT)
                border_color: (COLOR_TELEGRAM_GLASS_HAIRLINE)
                border_size: 1.0
                border_radius: 6.0
            }

            title := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_TEXT)
                    text_style: theme.font_bold { font_size: 11.0 }
                }
                text: "Send operation"
            }

            evidence := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Text, reply, thread, location, and confirmed desktop attachments submit through existing MatrixRequest paths. Queued/progress/failure labels plus Retry/Cancel controls are local evidence only; no extra send, retry, or cancel request is emitted."
            }

            result_bridge := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Result bridge: worker reports queued or immediate failure after SendAttachment handoff; timeline local echo shows SDK queue state; Retry/Cancel never retry or cancel SDK queue work."
            }

            taxonomy := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Attachment status taxonomy: review-pending -> validation-held -> handoff-submitted -> queued-only; failure-copy, empty-held, retry-local, cancel-local, discarded-local, and closed-local stay local evidence."
            }

            compact_fit := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Compact fit: attachment review/status text wraps in Fill/Fit labels; Send, Discard, Retry, and Cancel stay explicit wrapped buttons with no extra send or queue-cancel request."
            }

            action_density := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Action density: Send/Discard/Retry/Cancel use shared 36px wrapped icon+label buttons on mobile; density changes no send, retry, upload, or SDK queue-cancel behavior."
            }

            multi_file_queue_boundary := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Multi-file, album, per-file progress, background queue, reorder/remove, bulk retry, and accepted SDK queue controls stay local blocked."
            }

            accepted_queue_actions := View {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                spacing: 6.0,
                align: Align{y: 0.5}

                accepted_queue_label := Label {
                    width: Fit,
                    height: Fit,
                    draw_text +: {
                        color: (COLOR_TELEGRAM_MUTED)
                        text_style: theme.font_bold { font_size: 10.0 }
                    }
                    text: "accepted queue"
                }

                pause_attachment_queue_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Pause"
                }

                resume_attachment_queue_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_SEND)
                    text: "Resume"
                }

                reorder_attachment_queue_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Reorder"
                }

                background_attachment_queue_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_UPLOAD)
                    text: "Background"
                }

                clear_attachment_queue_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_CLOSE)
                    text: "Clear"
                }
            }

            accepted_queue_actions_label := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Accepted SDK queue controls: Pause, Resume, Reorder, Background, and Clear stay local; only failed-handoff Retry can resubmit."
            }

            accepted_queue_timeline_cancel_bridge := View {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                spacing: 6.0,
                align: Align{y: 0.5}

                accepted_queue_timeline_label := Label {
                    width: Fit,
                    height: Fit,
                    draw_text +: {
                        color: (COLOR_TELEGRAM_MUTED)
                        text_style: theme.font_bold { font_size: 10.0 }
                    }
                    text: "timeline cancel"
                }

                status_attachment_timeline_cancel_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Status"
                }

                handle_attachment_timeline_cancel_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Handle"
                }

                timeline_attachment_timeline_cancel_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_UPLOAD)
                    text: "Timeline"
                }

                cancel_attachment_timeline_cancel_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_CLOSE)
                    text: "Cancel"
                }

                source_attachment_timeline_cancel_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Source"
                }
            }

            accepted_queue_timeline_cancel_bridge_label := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Timeline cancel bridge: Status, Handle, Timeline, Cancel, and Source point to the real local-echo Cancel Send path when a SendHandle exists."
            }

            per_file_status_controls := View {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                spacing: 6.0,
                align: Align{y: 0.5}

                per_file_status_label := Label {
                    width: Fit,
                    height: Fit,
                    draw_text +: {
                        color: (COLOR_TELEGRAM_MUTED)
                        text_style: theme.font_bold { font_size: 10.0 }
                    }
                    text: "per-file"
                }

                status_attachment_file_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Status"
                }

                progress_attachment_file_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_UPLOAD)
                    text: "Progress"
                }

                pause_attachment_file_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Pause"
                }

                resume_attachment_file_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_SEND)
                    text: "Resume"
                }

                cancel_attachment_file_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_CLOSE)
                    text: "Cancel"
                }

                retry_attachment_file_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_SEND)
                    text: "Retry"
                }

                drilldown_attachment_file_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Drilldown"
                }

                contract_attachment_file_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Contract"
                }

                taxonomy_attachment_file_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Taxonomy"
                }
            }

            per_file_status_controls_label := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Per-file status controls: Status, Progress, Pause, Resume, Cancel, Retry, and Drilldown stay local; failed-handoff Retry confirmation remains the only resubmit path."
            }

            send_preflight_detail_controls := View {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                spacing: 6.0,
                align: Align{y: 0.5}

                send_preflight_label := Label {
                    width: Fit,
                    height: Fit,
                    draw_text +: {
                        color: (COLOR_TELEGRAM_MUTED)
                        text_style: theme.font_bold { font_size: 10.0 }
                    }
                    text: "preflight"
                }

                request_attachment_send_preflight_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Request"
                }

                result_attachment_send_preflight_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Result"
                }

                error_attachment_send_preflight_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Error"
                }

                retry_attachment_send_preflight_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_SEND)
                    text: "Retry"
                }

                source_attachment_send_preflight_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_UPLOAD)
                    text: "Source"
                }
            }

            attachment_send_preflight_detail_controls_label := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Attachment send preflight detail controls: Request, Result, Error, Retry, and Source stay local; review Send and confirmed failed-handoff Retry remain the only SendAttachment paths."
            }

            status_actions := View {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                spacing: 6.0,
                align: Align{y: 0.5}

                queue_status_label := Label {
                    width: Fit,
                    height: Fit,
                    draw_text +: {
                        color: (COLOR_TELEGRAM_MUTED)
                        text_style: theme.font_bold { font_size: 10.0 }
                    }
                    text: "local evidence"
                }

                retry_send_operation_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_SEND)
                    text: "Retry"
                }

                cancel_send_operation_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_CLOSE)
                    text: "Cancel"
                }
            }
        }

        typing_notice_status := RoundedView {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 3.0,
            margin: Inset{top: 4, left: 8, right: 8, bottom: 2}
            padding: Inset{top: 7, bottom: 7, left: 10, right: 10}
            show_bg: true,
            draw_bg +: {
                color: (COLOR_TELEGRAM_INPUT)
                border_color: (COLOR_TELEGRAM_BORDER)
                border_size: 1.0
                border_radius: 6.0
            }

            title := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_TEXT)
                    text_style: theme.font_bold { font_size: 11.0 }
                }
                text: "Typing notice"
            }

            evidence := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Plain composer text changes submit existing MatrixRequest::SendTypingNotice. Hepta command previews suppress Matrix typing notices; this strip emits no message, room-state, retry, or cancel request."
            }
        }

        telegram_attachment_picker := RoundedView {
            visible: false
            width: Fill,
            height: 258,
            flow: Down,
            spacing: 6.0,
            margin: Inset{top: 4, left: 8, right: 8, bottom: 2}
            padding: Inset{top: 8, bottom: 8, left: 10, right: 10}
            show_bg: true,
            draw_bg +: {
                color: (COLOR_TELEGRAM_INPUT)
                border_color: (COLOR_TELEGRAM_BORDER)
                border_size: 1.0
                border_radius: 6.0
            }

            attachment_header := View {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 8.0,
                align: Align{y: 0.5}

                attachment_title := Label {
                    width: Fill,
                    height: Fit,
                    draw_text +: {
                        color: (COLOR_TELEGRAM_TEXT)
                        text_style: theme.font_bold { font_size: 12.0 }
                    }
                    text: "Attach"
                }

                attachment_status := Label {
                    width: Fit,
                    height: Fit,
                    draw_text +: {
                        color: (COLOR_TELEGRAM_MUTED)
                        text_style: theme.font_bold { font_size: 10.0 }
                    }
                    text: "confirm + pick"
                }

                close_attachment_picker_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_CLOSE)
                    text: "Close"
                }
            }

            attachment_options := View {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                spacing: 6.0,
                align: Align{y: 0.5}

                photo_attachment_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_UPLOAD)
                    text: "Photo"
                }

                file_attachment_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_HTML_FILE)
                    text: "File"
                }

                camera_attachment_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Camera"
                }

                contact_attachment_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_ADD_USER)
                    text: "Contact"
                }
            }

            attachment_mobile_picker_controls := View {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                spacing: 6.0,
                align: Align{y: 0.5}

                attachment_mobile_picker_label := Label {
                    width: Fit,
                    height: Fit,
                    draw_text +: {
                        color: (COLOR_TELEGRAM_MUTED)
                        text_style: theme.font_bold { font_size: 10.0 }
                    }
                    text: "mobile"
                }

                gallery_attachment_mobile_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_UPLOAD)
                    text: "Gallery"
                }

                camera_attachment_mobile_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Camera"
                }

                files_attachment_mobile_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_HTML_FILE)
                    text: "Files"
                }

                contact_attachment_mobile_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_ADD_USER)
                    text: "Contact"
                }

                thumbnail_attachment_mobile_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Thumbnail"
                }

                share_attachment_mobile_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_SEND)
                    text: "Share"
                }
            }

            attachment_mobile_picker_controls_label := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Mobile picker controls: Gallery, Camera, Files, Contact, Thumbnail, and Share stay local; no permissions, picker, thumbnail decode, share sheet, payload, upload, or send."
            }

            attachment_review_actions := View {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                spacing: 6.0,
                align: Align{y: 0.5}

                send_selected_attachment_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_SEND)
                    text: "Send"
                }

                discard_selected_attachment_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_CLOSE)
                    text: "Discard"
                }
            }

            attachment_review_compact_fit := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Compact fit: review filename, metadata, validation warning, caption, and reply context wrap; Send and Discard stay explicit local review actions."
            }

            attachment_review_action_density := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Action density: review and recovery buttons share mobile 36px touch-height, wrapped rows, and icon+label affordances; no hidden send or queue-cancel action."
            }

            attachment_review_preview := View {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 2.0,

                attachment_review_title := Label {
                    width: Fill,
                    height: Fit,
                    flow: Flow.Right{wrap: true},
                    draw_text +: {
                        color: (COLOR_TELEGRAM_TEXT)
                        text_style: theme.font_bold { font_size: 10.5 }
                    }
                    text: "No selected attachment"
                }

                attachment_review_filename := Label {
                    width: Fill,
                    height: Fit,
                    flow: Flow.Right{wrap: true},
                    draw_text +: {
                        color: (COLOR_TELEGRAM_MUTED)
                        text_style: theme.font_bold { font_size: 10.0 }
                    }
                    text: "Pick Photo or File to review filename and MIME before send."
                }

                attachment_review_metadata := Label {
                    width: Fill,
                    height: Fit,
                    flow: Flow.Right{wrap: true},
                    draw_text +: {
                        color: (COLOR_TELEGRAM_DIM)
                        text_style: theme.font_regular { font_size: 10.0 }
                    }
                    text: "Local metadata appears after picker selection; no upload or media decode."
                }

                attachment_review_context := Label {
                    width: Fill,
                    height: Fit,
                    flow: Flow.Right{wrap: true},
                    draw_text +: {
                        color: (COLOR_TELEGRAM_DIM)
                        text_style: theme.font_regular { font_size: 10.0 }
                    }
                    text: "Caption and reply context stay local until review Send."
                }
            }

            attachment_summary := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.5 }
                }
                text: "Photo/File handoff requires confirmation. Selected desktop files enter local review before Matrix attachment send."
            }

            attachment_option_evidence := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Photo and File confirm before desktop rfd picker; selected files stage a local review row. Send submits MatrixRequest::SendAttachment via the Matrix send queue. Discard, Camera, Contact, and Close stay local."
            }
        }

        telegram_emoji_sticker_panel := RoundedView {
            visible: false
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 7.0,
            margin: Inset{top: 4, left: 8, right: 8, bottom: 2}
            padding: Inset{top: 8, bottom: 8, left: 10, right: 10}
            show_bg: true,
            draw_bg +: {
                color: (COLOR_TELEGRAM_INPUT)
                border_color: (COLOR_TELEGRAM_BORDER)
                border_size: 1.0
                border_radius: 6.0
            }

            emoji_header := View {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 8.0,
                align: Align{y: 0.5}

                emoji_title := Label {
                    width: Fill,
                    height: Fit,
                    draw_text +: {
                        color: (COLOR_TELEGRAM_TEXT)
                        text_style: theme.font_bold { font_size: 12.0 }
                    }
                    text: "Emoji / Sticker"
                }

                emoji_status := Label {
                    width: Fit,
                    height: Fit,
                    draw_text +: {
                        color: (COLOR_TELEGRAM_MUTED)
                        text_style: theme.font_bold { font_size: 10.0 }
                    }
                    text: "local only"
                }

                close_emoji_sticker_panel_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_CLOSE)
                    text: "Close"
                }
            }

            emoji_options := View {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                spacing: 6.0,
                align: Align{y: 0.5}

                smile_emoji_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_ADD_REACTION)
                    text: "Smile"
                }

                thumbs_emoji_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_ADD_REACTION)
                    text: "Thumbs"
                }

                heart_emoji_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_ADD_REACTION)
                    text: "Heart"
                }

                sticker_emoji_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Sticker"
                }
            }

            emoji_summary := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.5 }
                }
                text: "Local emoji/sticker preview only; no composer insert, payload, upload, or send."
            }

            emoji_lifecycle_metadata := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_MUTED)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Emoji/sticker lifecycle metadata: panel visibility, last choice, staged count, and close/reopen state stay local."
            }
        }

        telegram_voice_message_panel := RoundedView {
            visible: false
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 6.0,
            margin: Inset{top: 4, left: 8, right: 8, bottom: 2}
            padding: Inset{top: 8, bottom: 8, left: 10, right: 10}
            show_bg: true,
            draw_bg +: {
                color: (COLOR_TELEGRAM_INPUT)
                border_color: (COLOR_TELEGRAM_BORDER)
                border_size: 1.0
                border_radius: 6.0
            }

            voice_header := View {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 8.0,
                align: Align{y: 0.5}

                voice_title := Label {
                    width: Fill,
                    height: Fit,
                    draw_text +: {
                        color: (COLOR_TELEGRAM_TEXT)
                        text_style: theme.font_bold { font_size: 12.0 }
                    }
                    text: "Voice"
                }

                voice_status := Label {
                    width: Fit,
                    height: Fit,
                    draw_text +: {
                        color: (COLOR_TELEGRAM_MUTED)
                        text_style: theme.font_bold { font_size: 10.0 }
                    }
                    text: "local only"
                }

                close_voice_message_panel_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_CLOSE)
                    text: "Close"
                }
            }

            voice_preview := View {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 9.0,
                align: Align{y: 0.5}

                voice_meter := Label {
                    width: Fit,
                    height: Fit,
                    draw_text +: {
                        color: (COLOR_TELEGRAM_BLUE)
                        text_style: theme.font_bold { font_size: 11.0 }
                    }
                    text: "[|||| ||| ||]"
                }

                voice_duration := Label {
                    width: Fit,
                    height: Fit,
                    draw_text +: {
                        color: (COLOR_TELEGRAM_TEXT)
                        text_style: theme.font_bold { font_size: 11.0 }
                    }
                    text: "00:00"
                }

                voice_preview_mode := Label {
                    width: Fill,
                    height: Fit,
                    flow: Flow.Right{wrap: true},
                    draw_text +: {
                        color: (COLOR_TELEGRAM_MUTED)
                        text_style: theme.font_regular { font_size: 10.0 }
                    }
                    text: "preview only"
                }
            }

            voice_controls := View {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                spacing: 6.0,
                align: Align{y: 0.5}

                record_voice_preview_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_MIC)
                    text: "Record"
                }

                lock_voice_preview_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Lock"
                }

                cancel_voice_preview_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_CLOSE)
                    text: "Cancel"
                }

                send_voice_preview_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_SEND)
                    text: "Send"
                }
            }

            voice_recorder_status_controls := View {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                spacing: 6.0,
                align: Align{y: 0.5}

                timer_voice_status_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Timer"
                }

                waveform_voice_status_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Waveform"
                }

                transcript_voice_status_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Transcript"
                }

                progress_voice_status_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Progress"
                }

                codec_voice_status_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Codec"
                }
            }

            voice_capture_lifecycle_controls := View {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                spacing: 6.0,
                align: Align{y: 0.5}

                permission_voice_capture_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Permission"
                }

                capture_voice_capture_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_MIC)
                    text: "Capture"
                }

                encode_voice_capture_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Encode"
                }

                review_voice_capture_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Review"
                }

                upload_voice_capture_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_SEND)
                    text: "Upload"
                }

                packet_voice_capture_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Packet"
                }

                contract_voice_capture_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Contract"
                }

                taxonomy_voice_capture_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Taxonomy"
                }
            }

            voice_mobile_picker_controls := View {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                spacing: 6.0,
                align: Align{y: 0.5}

                mic_voice_mobile_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_MIC)
                    text: "Mic"
                }

                files_voice_mobile_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Files"
                }

                library_voice_mobile_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Library"
                }

                retake_voice_mobile_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_CLOSE)
                    text: "Retake"
                }

                share_voice_mobile_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_SEND)
                    text: "Share"
                }
            }

            voice_review_playback_controls := View {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                spacing: 6.0,
                align: Align{y: 0.5}

                play_voice_review_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Play"
                }

                pause_voice_review_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Pause"
                }

                scrub_voice_review_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Scrub"
                }

                speed_voice_review_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Speed"
                }

                drop_voice_review_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_CLOSE)
                    text: "Drop"
                }
            }

            voice_send_preflight_detail_controls := View {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                spacing: 6.0,
                align: Align{y: 0.5}

                request_voice_send_preflight_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Request"
                }

                result_voice_send_preflight_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Result"
                }

                error_voice_send_preflight_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Error"
                }

                retry_voice_send_preflight_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Retry"
                }

                source_voice_send_preflight_button := mod.widgets.TelegramAttachmentOptionButton {
                    draw_icon.svg: (ICON_INFO)
                    text: "Source"
                }
            }

            voice_recorder_status_metadata := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_MUTED)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Timer, waveform, transcript, progress, and codec controls stay local."
            }

            voice_capture_lifecycle_metadata := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_MUTED)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Permission, capture, encode, review, upload, Packet, Contract, and Taxonomy lifecycle controls stay local."
            }

            voice_mobile_picker_metadata := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_MUTED)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Mic, Files, Library, Retake, and Share mobile picker controls stay local."
            }

            voice_review_playback_metadata := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_MUTED)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Play, Pause, Scrub, Speed, and Drop review controls stay local."
            }

            voice_send_preflight_detail_metadata := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_MUTED)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Request, Result, Error, Retry, and Source voice send detail controls stay local."
            }

            voice_summary := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.5 }
                }
                text: "Voice permission boundary: no mic permission, audio session, recorder, local file, waveform capture, encoder, upload, send queue, or live mutation."
            }

            voice_option_evidence := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Record, Lock, Cancel, Send, and Close only stage this local voice preview; no permission, recorder, local file, waveform capture, encoder, upload, SendAttachment, text fallback, or SDK queue work is requested."
            }

            voice_send_blocked_evidence := Label {
                width: Fill,
                height: Fit,
                flow: Flow.Right{wrap: true},
                draw_text +: {
                    color: (COLOR_TELEGRAM_DIM)
                    text_style: theme.font_regular { font_size: 10.0 }
                }
                text: "Voice send blocked locally: Record/Lock/Send/Cancel/Close update preview copy only; no mic permission, audio payload, upload, SendAttachment, text fallback, or SDK queue work."
            }
        }

        // Below that, display one of multiple possible views:
        // * the message input bar (buttons and message TextInput).
        // * a notice that the user can't send messages to this room.
        // * if this room was tombstoned, a "footer" view showing the successor room info.
        // * the EditingPane, which slides up as an overlay in front of the other views below.
        overlay_wrapper := View {
            width: Fill,
            height: Fit{max: FitBound.Rel{base: Base.Full, factor: 0.75}}
            flow: Overlay,

            // Below that, display a view that holds the message input bar and send button.
            input_bar := View {
                width: Fill,
                height: Fit{max: FitBound.Rel{base: Base.Full, factor: 0.75}}
                flow: Right
                // Bottom-align everything to ensure that buttons always stick to the bottom
                // even when the mentionable_text_input box is very tall.
                align: Align{y: 1.0},
                padding: Inset{top: 8, bottom: 8, left: 12, right: 12},

                attachment_button := RobrixIconButton {
                    margin: 4
                    spacing: 0,
                    draw_icon +: {
                        svg: (ICON_UPLOAD)
                        color: (COLOR_TELEGRAM_MUTED)
                    },
                    draw_bg +: {
                        color: (COLOR_TELEGRAM_INPUT)
                        color_hover: #xFFFFFF14
                        color_down: (COLOR_TELEGRAM_DIALOG_ACTIVE)
                    }
                    icon_walk: Walk{width: 21, height: 21}
                    text: "",
                }

                location_button := RobrixIconButton {
                    margin: 4
                    spacing: 0,
                    draw_icon +: {
                        svg: (mod.widgets.ICO_LOCATION_PERSON)
                        color: (COLOR_TELEGRAM_BLUE)
                    },
                    draw_bg +: {
                        color: (COLOR_TELEGRAM_INPUT)
                        color_hover: #xFFFFFF14
                        color_down: (COLOR_TELEGRAM_DIALOG_ACTIVE)
                    }
                    icon_walk: Walk{width: 23, height: 23, margin: Inset{bottom: -1}}
                    text: "",
                }

                // A checkbox that enables TSP signing for the outgoing message.
                // If TSP is not enabled, this will be an empty invisible view.
                tsp_sign_checkbox := TspSignAnycastCheckbox {
                    margin: Inset{bottom: 9, left: 6, right: 0}
                }

                mentionable_text_input := MentionableTextInput {
                    width: Fill,
                    height: Fit
                    margin: Inset {
                        top: 3, // add some space between the top border of the text input and the top border of the room input bar
                        bottom: 5.75, // to line up the middle of the text input with the middle of the buttons
                        left: 3, right: 3 // to give a bit of breathing room between the text input and the buttons on the sides
                    },

                    persistent +: {
                        center +: {
                            text_input := RobrixTextInput {
                                empty_text: "Message Hepta"
                                is_multiline: true,
                                draw_bg +: {
                                    border_radius: 19.0
                                    border_size: 0.0
                                    color: (COLOR_TELEGRAM_INPUT)
                                    color_hover: (COLOR_TELEGRAM_INPUT)
                                    color_focus: (COLOR_TELEGRAM_INPUT)
                                    color_down: (COLOR_TELEGRAM_INPUT)
                                    color_empty: (COLOR_TELEGRAM_INPUT)
                                }
                                draw_text +: {
                                    color: (COLOR_TELEGRAM_TEXT)
                                    color_hover: (COLOR_TELEGRAM_TEXT)
                                    color_focus: (COLOR_TELEGRAM_TEXT)
                                    color_down: (COLOR_TELEGRAM_TEXT)
                                    color_empty: (COLOR_TELEGRAM_MUTED)
                                    color_empty_hover: (COLOR_TELEGRAM_MUTED)
                                    color_empty_focus: (COLOR_TELEGRAM_MUTED)
                                }
                                draw_cursor +: {
                                    color: (COLOR_TELEGRAM_TEXT)
                                }
                            }
                        }
                    }
                }

                emoji_button := RobrixIconButton {
                    margin: 4
                    spacing: 0,
                    draw_icon +: {
                        svg: (ICON_ADD_REACTION)
                        color: (COLOR_TELEGRAM_MUTED)
                    },
                    draw_bg +: {
                        color: (COLOR_TELEGRAM_INPUT)
                        color_hover: #xFFFFFF14
                        color_down: (COLOR_TELEGRAM_DIALOG_ACTIVE)
                    }
                    icon_walk: Walk{width: 21, height: 21}
                    text: "",
                }

                voice_message_button := RobrixIconButton {
                    margin: 4
                    spacing: 0,
                    draw_icon +: {
                        svg: (ICON_MIC)
                        color: (COLOR_TELEGRAM_MUTED)
                    },
                    draw_bg +: {
                        color: (COLOR_TELEGRAM_INPUT)
                        color_hover: #xFFFFFF14
                        color_down: (COLOR_TELEGRAM_DIALOG_ACTIVE)
                    }
                    icon_walk: Walk{width: 21, height: 21}
                    text: "",
                }

                send_message_button := RobrixPositiveIconButton {
                    // Disabled by default; enabled when text is inputted
                    enabled: false,
                    spacing: 0,
                    text: "",
                    margin: 4
                    draw_icon +: {
                        svg: (ICON_SEND)
                        color: (COLOR_TELEGRAM_TEXT)
                    }
                    draw_bg +: {
                        color: (COLOR_TELEGRAM_BLUE)
                        color_hover: #x319ED8
                        color_down: (COLOR_TELEGRAM_DIALOG_ACTIVE)
                    }
                    icon_walk: Walk{width: 21, height: 21},
                }
            }

            can_not_send_message_notice := SolidView {
                visible: false
                padding: 20
                align: Align{x: 0.5, y: 0.5}
                width: Fill, height: Fit

                show_bg: true
                draw_bg.color: (COLOR_SECONDARY)

                text := Label {
                    width: Fill,
                    flow: Flow.Right{wrap: true},
                    align: Align{x: 0.5, y: 0.5}
                    draw_text +: {
                        color: (COLOR_TEXT)
                        text_style: theme.font_italic {font_size: 12.2}
                    }
                    text: "You don't have permission to post to this room.",
                }
            }

            tombstone_footer := TombstoneFooter { }

            editing_pane := EditingPane { }
        }
    }
}

/// Main component for message input with @mention support
#[derive(Script, Widget)]
pub struct RoomInputBar {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,

    /// Whether the `ReplyingPreview` was visible when the `EditingPane` was shown.
    /// If true, when the `EditingPane` gets hidden, we need to re-show the `ReplyingPreview`.
    #[rust]
    was_replying_preview_visible: bool,
    /// Info about the message event that the user is currently replying to, if any.
    #[rust]
    replying_to: Option<(EventTimelineItem, EmbeddedEvent)>,
    /// Cached natural Fit height of the input_bar, used as the animation
    /// target when the editing pane is being hidden.
    #[rust]
    input_bar_natural_height: f64,
    #[rust]
    telegram_attachment_picker_visible: bool,
    #[rust]
    telegram_attachment_local_status: String,
    #[rust]
    telegram_pending_attachment_send: Option<PendingAttachmentSend>,
    #[rust]
    telegram_attachment_send_retry_attempt: Option<AttachmentSendRetryAttempt>,
    #[rust]
    telegram_attachment_send_preflight_detail: String,
    #[rust]
    telegram_attachment_send_cached_error: Option<String>,
    #[rust]
    telegram_emoji_sticker_panel_visible: bool,
    #[rust]
    telegram_emoji_sticker_local_status: String,
    #[rust]
    telegram_emoji_sticker_last_choice: Option<String>,
    #[rust]
    telegram_emoji_sticker_stage_count: usize,
    #[rust]
    telegram_emoji_sticker_last_lifecycle_action: String,
    #[rust]
    telegram_voice_message_panel_visible: bool,
    #[rust]
    telegram_voice_local_status: String,
    #[rust]
    telegram_voice_recorder_last_control: Option<String>,
    #[rust]
    telegram_voice_capture_lifecycle_last_control: Option<String>,
    #[rust]
    telegram_voice_mobile_picker_last_control: Option<String>,
    #[rust]
    telegram_voice_review_playback_last_control: Option<String>,
    #[rust]
    telegram_voice_send_preflight_detail: String,
}

#[derive(Clone, Debug)]
enum RoomInputBarAction {
    AttachmentHandoffConfirmed {
        kind: AttachmentHandoffKind,
        timeline_kind: TimelineKind,
        in_reply_to: Option<OwnedEventId>,
    },
    AttachmentHandoffCanceled {
        kind: AttachmentHandoffKind,
    },
    AttachmentSendRetryConfirmed {
        attempt: AttachmentSendRetryAttempt,
    },
    LocationSendConfirmed {
        timeline_kind: TimelineKind,
        coords: Coordinates,
        replied_to: Option<RoomInputBarReplyTarget>,
        #[cfg(feature = "tsp")]
        sign_with_tsp: bool,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AttachmentHandoffKind {
    Photo,
    File,
    Voice,
}

impl AttachmentHandoffKind {
    fn label(self) -> &'static str {
        match self {
            Self::Photo => "Photo",
            Self::File => "File",
            Self::Voice => "Voice",
        }
    }
}

#[allow(dead_code)]
enum AttachmentFilePickResult {
    Picked(PathBuf),
    Canceled,
    Unsupported,
}

#[derive(Clone)]
struct PendingAttachmentSend {
    kind: AttachmentHandoffKind,
    timeline_kind: TimelineKind,
    file_path: PathBuf,
    mime_type: mime::Mime,
    filename: String,
    file_extension: String,
    file_size_bytes: Option<u64>,
    image_dimensions_label: Option<String>,
    audio_duration_label: Option<String>,
    audio_waveform_codec_label: Option<String>,
    caption_preview: String,
    in_reply_to: Option<OwnedEventId>,
    validation_error: Option<String>,
}

#[derive(Clone, Debug)]
struct AttachmentSendRetryAttempt {
    kind: AttachmentHandoffKind,
    timeline_kind: TimelineKind,
    file_path: PathBuf,
    mime_type: mime::Mime,
    filename: String,
    caption: Option<TextMessageEventContent>,
    mentions: Option<Mentions>,
    in_reply_to: Option<OwnedEventId>,
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
fn pick_telegram_attachment_file(kind: AttachmentHandoffKind) -> AttachmentFilePickResult {
    let mut dialog = rfd::FileDialog::new();
    if kind == AttachmentHandoffKind::Photo {
        dialog = dialog.add_filter("Images", &["png", "jpg", "jpeg", "gif", "webp", "bmp"]);
    } else if kind == AttachmentHandoffKind::Voice {
        dialog = dialog.add_filter(
            "Audio",
            &["ogg", "opus", "m4a", "mp3", "wav", "aac", "flac", "webm"],
        );
    }
    dialog
        .pick_file()
        .map(AttachmentFilePickResult::Picked)
        .unwrap_or(AttachmentFilePickResult::Canceled)
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
fn pick_telegram_attachment_file(_kind: AttachmentHandoffKind) -> AttachmentFilePickResult {
    AttachmentFilePickResult::Unsupported
}

fn telegram_attachment_mime_type(path: &Path) -> mime::Mime {
    mime_guess::from_path(path).first_or_octet_stream()
}

fn telegram_attachment_file_size(path: &Path) -> Option<u64> {
    fs::metadata(path).ok().map(|metadata| metadata.len())
}

fn validate_telegram_attachment_file_for_review_send(path: &Path) -> Result<(), &'static str> {
    let metadata = fs::metadata(path).map_err(|_| "selected path is unreadable")?;
    if !metadata.is_file() {
        return Err("selected path is not a regular file");
    }
    if metadata.len() == 0 {
        return Err("selected file is empty");
    }
    Ok(())
}

fn display_attachment_filename(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("attachment")
        .to_string()
}

fn display_attachment_extension(path: &Path) -> String {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(str::trim)
        .filter(|extension| !extension.is_empty())
        .map(|extension| extension.to_ascii_lowercase())
        .unwrap_or_else(|| "no extension".to_string())
}

fn format_attachment_file_size(size: Option<u64>) -> String {
    let Some(size) = size else {
        return "size unavailable".to_string();
    };
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;
    if size < 1024 {
        format!("{size} B")
    } else if size < 1024 * 1024 {
        format!("{:.1} KB", size as f64 / KB)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.1} MB", size as f64 / MB)
    } else {
        format!("{:.1} GB", size as f64 / GB)
    }
}

fn is_header_dimension_image_file(path: &Path, mime_type: &mime::Mime) -> bool {
    matches!(
        mime_type.essence_str(),
        "image/png" | "image/jpeg" | "image/gif" | "image/bmp" | "image/webp"
    ) || path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp"
            )
        })
        .unwrap_or(false)
}

fn read_image_header_bytes(path: &Path) -> Option<Vec<u8>> {
    let mut file = fs::File::open(path).ok()?;
    let mut bytes = Vec::new();
    file.by_ref()
        .take(512 * 1024)
        .read_to_end(&mut bytes)
        .ok()?;
    Some(bytes)
}

fn parse_png_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 24 || &bytes[0..8] != b"\x89PNG\r\n\x1a\n" || &bytes[12..16] != b"IHDR" {
        return None;
    }
    Some((
        u32::from_be_bytes(bytes[16..20].try_into().ok()?),
        u32::from_be_bytes(bytes[20..24].try_into().ok()?),
    ))
}

fn parse_gif_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 10 || !matches!(&bytes[0..6], b"GIF87a" | b"GIF89a") {
        return None;
    }
    Some((
        u16::from_le_bytes(bytes[6..8].try_into().ok()?) as u32,
        u16::from_le_bytes(bytes[8..10].try_into().ok()?) as u32,
    ))
}

fn parse_bmp_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 26 || &bytes[0..2] != b"BM" {
        return None;
    }
    let width = i32::from_le_bytes(bytes[18..22].try_into().ok()?);
    let height = i32::from_le_bytes(bytes[22..26].try_into().ok()?);
    Some((width.unsigned_abs(), height.unsigned_abs()))
}

fn parse_jpeg_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 4 || bytes[0] != 0xff || bytes[1] != 0xd8 {
        return None;
    }

    let mut offset = 2usize;
    while offset + 3 < bytes.len() {
        while offset < bytes.len() && bytes[offset] == 0xff {
            offset += 1;
        }
        if offset >= bytes.len() {
            break;
        }
        let marker = bytes[offset];
        offset += 1;
        if marker == 0xd9 || marker == 0xda {
            break;
        }
        if offset + 2 > bytes.len() {
            break;
        }
        let segment_len = u16::from_be_bytes(bytes[offset..offset + 2].try_into().ok()?) as usize;
        if segment_len < 2 {
            break;
        }
        let segment_start = offset + 2;
        let segment_end = offset.checked_add(segment_len)?;
        if segment_end > bytes.len() {
            break;
        }
        if matches!(
            marker,
            0xc0 | 0xc1
                | 0xc2
                | 0xc3
                | 0xc5
                | 0xc6
                | 0xc7
                | 0xc9
                | 0xca
                | 0xcb
                | 0xcd
                | 0xce
                | 0xcf
        ) && segment_start + 5 <= segment_end
        {
            let height = u16::from_be_bytes(
                bytes[segment_start + 1..segment_start + 3]
                    .try_into()
                    .ok()?,
            ) as u32;
            let width = u16::from_be_bytes(
                bytes[segment_start + 3..segment_start + 5]
                    .try_into()
                    .ok()?,
            ) as u32;
            return Some((width, height));
        }
        offset = segment_end;
    }

    None
}

fn parse_webp_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 20 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WEBP" {
        return None;
    }

    let mut offset = 12usize;
    while offset + 8 <= bytes.len() {
        let chunk_id = &bytes[offset..offset + 4];
        let chunk_size =
            u32::from_le_bytes(bytes[offset + 4..offset + 8].try_into().ok()?) as usize;
        let data_start = offset + 8;
        let Some(data_end) = data_start.checked_add(chunk_size) else {
            break;
        };
        if data_end > bytes.len() {
            break;
        }

        if chunk_id == b"VP8X" && chunk_size >= 10 {
            let width = 1 + u32::from_le_bytes([
                bytes[data_start + 4],
                bytes[data_start + 5],
                bytes[data_start + 6],
                0,
            ]);
            let height = 1 + u32::from_le_bytes([
                bytes[data_start + 7],
                bytes[data_start + 8],
                bytes[data_start + 9],
                0,
            ]);
            return Some((width, height));
        } else if chunk_id == b"VP8L" && chunk_size >= 5 && bytes[data_start] == 0x2f {
            let b1 = bytes[data_start + 1] as u32;
            let b2 = bytes[data_start + 2] as u32;
            let b3 = bytes[data_start + 3] as u32;
            let b4 = bytes[data_start + 4] as u32;
            let width = 1 + (((b2 & 0x3f) << 8) | b1);
            let height = 1 + (((b4 & 0x0f) << 10) | (b3 << 2) | ((b2 & 0xc0) >> 6));
            return Some((width, height));
        } else if chunk_id == b"VP8 "
            && chunk_size >= 10
            && &bytes[data_start + 3..data_start + 6] == b"\x9d\x01\x2a"
        {
            let width = u16::from_le_bytes(bytes[data_start + 6..data_start + 8].try_into().ok()?)
                as u32
                & 0x3fff;
            let height = u16::from_le_bytes(bytes[data_start + 8..data_start + 10].try_into().ok()?)
                as u32
                & 0x3fff;
            return Some((width, height));
        }

        offset = data_end + (chunk_size % 2);
    }

    None
}

fn image_dimensions_from_header(bytes: &[u8]) -> Option<(u32, u32, &'static str)> {
    if let Some((width, height)) = parse_png_dimensions(bytes) {
        return Some((width, height, "PNG"));
    }
    if let Some((width, height)) = parse_jpeg_dimensions(bytes) {
        return Some((width, height, "JPEG"));
    }
    if let Some((width, height)) = parse_gif_dimensions(bytes) {
        return Some((width, height, "GIF"));
    }
    if let Some((width, height)) = parse_bmp_dimensions(bytes) {
        return Some((width, height, "BMP"));
    }
    if let Some((width, height)) = parse_webp_dimensions(bytes) {
        return Some((width, height, "WebP"));
    }
    None
}

fn selected_image_dimensions_label(path: &Path, mime_type: &mime::Mime) -> String {
    if !is_header_dimension_image_file(path, mime_type) {
        return "dimensions: unavailable for this Photo file type".to_string();
    }
    let Some(bytes) = read_image_header_bytes(path) else {
        return "dimensions: unavailable from unreadable image header".to_string();
    };
    image_dimensions_from_header(&bytes)
        .map(|(width, height, format)| format!("dimensions: {width}x{height} from {format} header"))
        .unwrap_or_else(|| "dimensions: unavailable from image header".to_string())
}

fn pending_attachment_image_metadata_label(pending: &PendingAttachmentSend) -> Option<String> {
    pending
        .image_dimensions_label
        .as_ref()
        .map(|dimensions_label| {
            format!(
                "image metadata: filename {} | MIME {} | ext {} | size {} | {}",
                pending.filename,
                pending.mime_type,
                pending.file_extension,
                format_attachment_file_size(pending.file_size_bytes),
                dimensions_label
            )
        })
}

fn format_audio_duration_millis(duration_millis: u64) -> String {
    let total_seconds = (duration_millis + 500) / 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{minutes}:{seconds:02}")
}

fn is_wav_audio_file(path: &Path, mime_type: &mime::Mime) -> bool {
    matches!(mime_type.essence_str(), "audio/wav" | "audio/x-wav")
        || path
            .extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| extension.eq_ignore_ascii_case("wav"))
            .unwrap_or(false)
}

const VOICE_SELECTED_AUDIO_WAVEFORM_MAX_BYTES: u64 = 1024 * 1024;
const VOICE_SELECTED_AUDIO_WAVEFORM_BUCKETS: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WavAudioHeader {
    audio_format: u16,
    channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    bits_per_sample: u16,
    data_offset: usize,
    data_size: usize,
}

fn read_wav_probe_bytes(path: &Path, byte_limit: u64) -> Option<Vec<u8>> {
    let mut file = fs::File::open(path).ok()?;
    let mut bytes = Vec::new();
    file.by_ref()
        .take(byte_limit)
        .read_to_end(&mut bytes)
        .ok()?;
    Some(bytes)
}

fn parse_wav_audio_header(bytes: &[u8]) -> Option<WavAudioHeader> {
    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return None;
    }

    let mut offset = 12usize;
    let mut fmt: Option<(u16, u16, u32, u32, u16)> = None;
    let mut data: Option<(usize, usize)> = None;
    while offset + 8 <= bytes.len() {
        let chunk_id = &bytes[offset..offset + 4];
        let chunk_size =
            u32::from_le_bytes(bytes[offset + 4..offset + 8].try_into().ok()?) as usize;
        let chunk_data_start = offset + 8;
        let Some(chunk_data_end) = chunk_data_start.checked_add(chunk_size) else {
            break;
        };

        if chunk_id == b"fmt " && chunk_size >= 16 && chunk_data_start + 16 <= bytes.len() {
            let audio_format = u16::from_le_bytes(
                bytes[chunk_data_start..chunk_data_start + 2]
                    .try_into()
                    .ok()?,
            );
            let channels = u16::from_le_bytes(
                bytes[chunk_data_start + 2..chunk_data_start + 4]
                    .try_into()
                    .ok()?,
            );
            let sample_rate = u32::from_le_bytes(
                bytes[chunk_data_start + 4..chunk_data_start + 8]
                    .try_into()
                    .ok()?,
            );
            let byte_rate = u32::from_le_bytes(
                bytes[chunk_data_start + 8..chunk_data_start + 12]
                    .try_into()
                    .ok()?,
            );
            let bits_per_sample = u16::from_le_bytes(
                bytes[chunk_data_start + 14..chunk_data_start + 16]
                    .try_into()
                    .ok()?,
            );
            fmt = Some((
                audio_format,
                channels,
                sample_rate,
                byte_rate,
                bits_per_sample,
            ));
        } else if chunk_id == b"data" {
            data = Some((chunk_data_start, chunk_size));
        }

        if let (
            Some((audio_format, channels, sample_rate, byte_rate, bits_per_sample)),
            Some((data_offset, data_size)),
        ) = (fmt, data)
        {
            return Some(WavAudioHeader {
                audio_format,
                channels,
                sample_rate,
                byte_rate,
                bits_per_sample,
                data_offset,
                data_size,
            });
        }

        if chunk_data_end > bytes.len() {
            break;
        }
        offset = chunk_data_end + (chunk_size % 2);
    }

    None
}

fn wav_duration_millis(path: &Path) -> Option<u64> {
    let bytes = read_wav_probe_bytes(path, 64 * 1024)?;
    let header = parse_wav_audio_header(&bytes)?;
    (header.byte_rate > 0).then(|| header.data_size as u64 * 1000 / header.byte_rate as u64)
}

fn voice_audio_duration_label(path: &Path, mime_type: &mime::Mime) -> String {
    if is_wav_audio_file(path, mime_type) {
        if let Some(duration_millis) = wav_duration_millis(path) {
            return format!(
                "duration: {} from WAV header",
                format_audio_duration_millis(duration_millis)
            );
        }
        return "duration: unavailable from WAV header".to_string();
    }

    "duration: unavailable before recorder/player metadata".to_string()
}

fn wav_codec_name(audio_format: u16) -> &'static str {
    match audio_format {
        1 => "PCM",
        3 => "IEEE float",
        6 => "A-law",
        7 => "mu-law",
        0xfffe => "WAVE extensible",
        _ => "unknown WAV codec",
    }
}

fn pcm_sample_peak(sample: &[u8], bits_per_sample: u16) -> Option<f64> {
    match bits_per_sample {
        8 => sample.first().map(|value| {
            let centered = *value as i16 - 128;
            (centered.unsigned_abs() as f64 / 128.0).min(1.0)
        }),
        16 if sample.len() >= 2 => {
            let value = i16::from_le_bytes(sample[0..2].try_into().ok()?) as i32;
            Some((value.unsigned_abs() as f64 / 32768.0).min(1.0))
        }
        24 if sample.len() >= 3 => {
            let raw = sample[0] as i32 | ((sample[1] as i32) << 8) | ((sample[2] as i32) << 16);
            let signed = (raw << 8) >> 8;
            Some((signed.unsigned_abs() as f64 / 8_388_608.0).min(1.0))
        }
        32 if sample.len() >= 4 => {
            let value = i32::from_le_bytes(sample[0..4].try_into().ok()?) as i64;
            Some(((value.unsigned_abs() as f64) / 2_147_483_648.0).min(1.0))
        }
        _ => None,
    }
}

fn wav_pcm_peak_buckets(
    bytes: &[u8],
    header: &WavAudioHeader,
    bucket_count: usize,
) -> Option<Vec<u8>> {
    if header.audio_format != 1 || header.channels == 0 || bucket_count == 0 {
        return None;
    }
    if header.bits_per_sample == 0 || header.bits_per_sample % 8 != 0 {
        return None;
    }
    let bytes_per_sample = usize::from(header.bits_per_sample / 8);
    let frame_size = bytes_per_sample.checked_mul(usize::from(header.channels))?;
    if frame_size == 0 || header.data_offset >= bytes.len() {
        return None;
    }
    let available_data_size = header
        .data_size
        .min(bytes.len().saturating_sub(header.data_offset));
    let frame_count = available_data_size / frame_size;
    if frame_count == 0 {
        return None;
    }

    let mut peaks = Vec::with_capacity(bucket_count);
    for bucket in 0..bucket_count {
        let start_frame = bucket * frame_count / bucket_count;
        let mut end_frame = (bucket + 1) * frame_count / bucket_count;
        if end_frame <= start_frame {
            end_frame = (start_frame + 1).min(frame_count);
        }
        let mut peak = 0.0f64;
        for frame in start_frame..end_frame {
            let frame_offset = header.data_offset + frame * frame_size;
            for channel in 0..usize::from(header.channels) {
                let sample_offset = frame_offset + channel * bytes_per_sample;
                let sample_end = sample_offset + bytes_per_sample;
                if sample_end > bytes.len() {
                    continue;
                }
                if let Some(sample_peak) =
                    pcm_sample_peak(&bytes[sample_offset..sample_end], header.bits_per_sample)
                {
                    peak = peak.max(sample_peak);
                }
            }
        }
        peaks.push((peak * 100.0).round().clamp(0.0, 100.0) as u8);
    }
    Some(peaks)
}

fn format_waveform_peaks(peaks: &[u8]) -> String {
    peaks
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn voice_audio_waveform_codec_label(path: &Path, mime_type: &mime::Mime) -> String {
    let extension = display_attachment_extension(path);
    if !is_wav_audio_file(path, mime_type) {
        return format!(
            "codec: {} / ext {}; waveform: unavailable for non-WAV selected audio until decoder adapter",
            mime_type, extension
        );
    }
    let Some(bytes) = read_wav_probe_bytes(path, VOICE_SELECTED_AUDIO_WAVEFORM_MAX_BYTES) else {
        return "codec: unavailable from unreadable WAV file; waveform: unavailable".to_string();
    };
    let Some(header) = parse_wav_audio_header(&bytes) else {
        return format!(
            "codec: unavailable from WAV header; waveform: unavailable; probe bytes {}",
            bytes.len()
        );
    };
    let codec = wav_codec_name(header.audio_format);
    let duration = if header.byte_rate > 0 {
        format_audio_duration_millis(header.data_size as u64 * 1000 / header.byte_rate as u64)
    } else {
        "unavailable".to_string()
    };
    let waveform = wav_pcm_peak_buckets(&bytes, &header, VOICE_SELECTED_AUDIO_WAVEFORM_BUCKETS)
        .map(|peaks| {
            format!(
                "waveform: PCM peak buckets 16x={}",
                format_waveform_peaks(&peaks)
            )
        })
        .unwrap_or_else(|| {
            format!(
                "waveform: unavailable for {} format {}-bit",
                codec, header.bits_per_sample
            )
        });
    format!(
        "codec: {codec} format={} channels={} sample_rate={}Hz bits={} data={} bytes duration={} from WAV header; {waveform}; probe bytes {} capped at {}",
        header.audio_format,
        header.channels,
        header.sample_rate,
        header.bits_per_sample,
        header.data_size,
        duration,
        bytes.len(),
        format_attachment_file_size(Some(VOICE_SELECTED_AUDIO_WAVEFORM_MAX_BYTES)),
    )
}

fn pending_attachment_audio_metadata_label(pending: &PendingAttachmentSend) -> Option<String> {
    pending.audio_duration_label.as_ref().map(|duration_label| {
        let waveform_codec = pending
            .audio_waveform_codec_label
            .as_deref()
            .unwrap_or("codec/waveform: unavailable before selected audio analysis");
        format!(
            "audio metadata: filename {} | MIME {} | ext {} | size {} | {} | {}",
            pending.filename,
            pending.mime_type,
            pending.file_extension,
            format_attachment_file_size(pending.file_size_bytes),
            duration_label,
            waveform_codec
        )
    })
}

fn summarize_attachment_caption(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return "caption: none".to_string();
    }
    let char_count = trimmed.chars().count();
    let mut preview = trimmed.chars().take(48).collect::<String>();
    if char_count > 48 {
        preview.push_str("...");
    }
    format!("caption: {preview}")
}

fn attachment_review_lifecycle_metadata_label(
    action: &str,
    kind_label: &str,
    filename: Option<&str>,
    mime_label: Option<&str>,
    file_size_bytes: Option<u64>,
    caption_preview: Option<&str>,
    reply_context_loaded: bool,
    validation_error: Option<&str>,
    replaced_previous_filename: Option<&str>,
) -> String {
    let file_state = filename
        .filter(|filename| !filename.trim().is_empty())
        .map(|filename| {
            let mime_state = mime_label
                .filter(|mime| !mime.trim().is_empty())
                .unwrap_or("mime unavailable");
            format!(
                "file {filename}, {mime_state}, {}",
                format_attachment_file_size(file_size_bytes)
            )
        })
        .unwrap_or_else(|| "no pending attachment loaded".to_string());
    let caption_state = caption_preview
        .filter(|caption| !caption.trim().is_empty())
        .unwrap_or("caption: none");
    let reply_state = if reply_context_loaded {
        "reply context loaded"
    } else {
        "reply context none"
    };
    let validation_state = validation_error
        .filter(|reason| !reason.trim().is_empty())
        .map(|reason| format!("validation warning loaded: {reason}"))
        .unwrap_or_else(|| "validation ready".to_string());
    let replacement_state = replaced_previous_filename
        .filter(|filename| !filename.trim().is_empty())
        .map(|filename| format!("previous pending replaced: {filename}"))
        .unwrap_or_else(|| "no previous pending replacement".to_string());
    format!(
        "Attachment {action} metadata: {kind_label}; {file_state}; {caption_state}; {reply_state}; {validation_state}; {replacement_state}. {ATTACHMENT_REVIEW_LIFECYCLE_METADATA_LABEL}"
    )
}

fn attachment_send_failure_retry_confirmation_label(
    filename: &str,
    kind_label: &str,
    has_caption: bool,
    has_reply: bool,
) -> String {
    let filename = if filename.trim().is_empty() {
        "attachment"
    } else {
        filename.trim()
    };
    let caption_state = if has_caption {
        "caption cached"
    } else {
        "caption none"
    };
    let reply_state = if has_reply {
        "reply event id cached"
    } else {
        "reply none"
    };
    format!(
        "Attachment failed-handoff Retry confirmation: {kind_label} {filename}; {caption_state}; {reply_state}; cached TimelineKind, local file path, and MIME reused only after PositiveConfirmationModal. {ATTACHMENT_SEND_FAILURE_RETRY_CONFIRMATION_LABEL}"
    )
}

fn attachment_multi_file_queue_boundary_label(
    pending_review: Option<&str>,
    retry_cache_ready: bool,
) -> String {
    let pending_state = pending_review
        .filter(|pending| !pending.trim().is_empty())
        .map(|pending| format!("pending review {pending}"))
        .unwrap_or_else(|| "no pending review loaded".to_string());
    let retry_state = if retry_cache_ready {
        "immediate handoff retry cache ready"
    } else {
        "immediate handoff retry cache empty"
    };
    format!(
        "Attachment multi-file/album queue boundary: {pending_state}; {retry_state}. Multiple-file selection, album grouping, per-file progress rows, background upload list, reorder/remove queued items, bulk retry, accepted SDK queue retry/resume/cancel, delivery receipt fan-in, and queue persistence across room switches stay local blocked controls. {ATTACHMENT_MULTI_FILE_QUEUE_BOUNDARY_LABEL} No extra picker, extra SendAttachment, caption-only SendMessage, SDK queue abort/remove/cancel, gateway/runtime/auth, or live mutation."
    )
}

fn attachment_accepted_queue_actions_row_label(
    action: &str,
    pending_review: Option<&str>,
    retry_cache_ready: bool,
) -> String {
    let action = action.trim();
    let action_label = if action.is_empty() {
        "Queue action"
    } else {
        action
    };
    let pending_state = pending_review
        .filter(|pending| !pending.trim().is_empty())
        .map(|pending| format!("pending review {pending}"))
        .unwrap_or_else(|| "no pending review loaded".to_string());
    let retry_state = if retry_cache_ready {
        "immediate handoff retry cache ready"
    } else {
        "immediate handoff retry cache empty"
    };
    format!(
        "Accepted SDK queue {action_label} stayed local: {pending_state}; {retry_state}. Pause, Resume, Reorder, Background, and Clear only update this composer boundary copy after the existing SendAttachment/use_send_queue handoff. They do not retry or resume accepted SDK queue uploads, pause uploads, abort uploads, remove queued media, reorder SDK queue items, open a background queue manager, clear delivery receipts, resubmit SendAttachment, send caption-only SendMessage, gateway/runtime/auth, or live mutation. {ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_LABEL}"
    )
}

fn attachment_accepted_queue_background_snapshot_label(
    pending_review: Option<&str>,
    retry_cache_ready: bool,
    latest_status: &str,
) -> String {
    let pending_state = pending_review
        .filter(|pending| !pending.trim().is_empty())
        .map(|pending| format!("pending review {pending}"))
        .unwrap_or_else(|| "no pending review loaded".to_string());
    let retry_state = if retry_cache_ready {
        "immediate handoff retry cache ready"
    } else {
        "immediate handoff retry cache empty"
    };
    let latest_status = latest_status.trim();
    let latest_status = if latest_status.is_empty() {
        "local attachment status empty"
    } else {
        latest_status
    };
    format!(
        "Local accepted attachment queue snapshot: {pending_state}; {retry_state}; latest status {latest_status}; accepted SDK queue handle unavailable in composer; background queue manager not opened. Background renders this local queue snapshot only after the existing SendAttachment/use_send_queue handoff. It does not retry or resume accepted SDK queue uploads, pause uploads, abort uploads, remove queued media, reorder SDK queue items, open a background queue manager, clear delivery receipts, resubmit SendAttachment, send caption-only SendMessage, gateway/runtime/auth, or live mutation. {ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_LABEL}"
    )
}

fn attachment_accepted_queue_timeline_cancel_bridge_label(
    control: &str,
    pending_review: Option<&str>,
    retry_cache_ready: bool,
    latest_status: &str,
) -> String {
    let control = control.trim();
    let control_label = if control.is_empty() {
        "Status"
    } else {
        control
    };
    let pending_state = pending_review
        .filter(|pending| !pending.trim().is_empty())
        .map(|pending| format!("pending review {pending}"))
        .unwrap_or_else(|| "no pending review loaded".to_string());
    let retry_state = if retry_cache_ready {
        "immediate handoff retry cache ready"
    } else {
        "immediate handoff retry cache empty"
    };
    let latest_status = if latest_status.trim().is_empty() {
        "local evidence"
    } else {
        latest_status.trim()
    };
    format!(
        "Accepted queue timeline-cancel {control_label} stayed local: {pending_state}; {retry_state}; latest status {latest_status}. Status, Handle, Timeline, Cancel, and Source only bridge composer queue copy to the real timeline local echo cancel surface. The composer bridge does not hold a SendHandle; a real abort is available only from the timeline local echo context menu while local_echo_send_handle exists, where RoomScreen submits MatrixRequest::AbortLocalSend for that exact SendHandle. The bridge does not abort uploads from the composer, remove queued media, retry/resume accepted queue items, resubmit SendAttachment, gateway/runtime/auth, or live mutation. {ATTACHMENT_ACCEPTED_QUEUE_TIMELINE_CANCEL_BRIDGE_LABEL}"
    )
}

fn attachment_local_send_abort_result_label(result: &Result<bool, String>) -> String {
    match result {
        Ok(true) => format!(
            "Timeline local echo Cancel Send result: SDK SendHandle::abort returned canceled. {ATTACHMENT_LOCAL_SEND_ABORT_RESULT_LABEL} No composer-held SendHandle, accepted queue retry/resume, queue removal, SendAttachment resubmit, caption-only SendMessage, gateway/runtime/auth, or live mutation."
        ),
        Ok(false) => format!(
            "Timeline local echo Cancel Send result: SDK SendHandle::abort reported the item was already sent or no longer cancellable. {ATTACHMENT_LOCAL_SEND_ABORT_RESULT_LABEL} No composer-held SendHandle, accepted queue retry/resume, queue removal, SendAttachment resubmit, caption-only SendMessage, gateway/runtime/auth, or live mutation."
        ),
        Err(error) => format!(
            "Timeline local echo Cancel Send failed: {error}. {ATTACHMENT_LOCAL_SEND_ABORT_RESULT_LABEL} No automatic retry, composer-held SendHandle, accepted queue retry/resume, queue removal, SendAttachment resubmit, caption-only SendMessage, gateway/runtime/auth, or live mutation."
        ),
    }
}

fn attachment_per_file_queue_drilldown_label(
    pending_review: Option<&str>,
    retry_cache_ready: bool,
    latest_status: &str,
) -> String {
    let pending_state = pending_review
        .filter(|pending| !pending.trim().is_empty())
        .map(|pending| format!("pending review {pending}"))
        .unwrap_or_else(|| "no pending review loaded".to_string());
    let retry_state = if retry_cache_ready {
        "immediate handoff retry cache ready"
    } else {
        "immediate handoff retry cache empty"
    };
    let latest_status = if latest_status.trim().is_empty() {
        "local evidence"
    } else {
        latest_status.trim()
    };
    format!(
        "Per-file attachment queue drilldown stayed local: {pending_state}; {retry_state}; latest status {latest_status}. Accepted-send queue acceptance criteria are represented as local fields only: queue item identity, stable file metadata, progress slot, pause eligibility, resume eligibility, cancel eligibility, retry eligibility, timeline local-echo cancel handle, result slot, error slot, delivery receipt mapping, background persistence, and reorder/grouping slots. This drilldown does not inspect SDK queue entries, subscribe to upload progress, pause/resume/cancel uploads, retry accepted queue items, resubmit SendAttachment, send caption-only SendMessage, map delivery receipts, gateway/runtime/auth, or live mutation. {ATTACHMENT_PER_FILE_QUEUE_DRILLDOWN_LABEL}"
    )
}

fn attachment_sdk_queue_contract_packet_label(
    pending_review: Option<&str>,
    retry_cache_ready: bool,
    latest_status: &str,
) -> String {
    let pending_state = pending_review
        .filter(|pending| !pending.trim().is_empty())
        .map(|pending| format!("pending review {pending}"))
        .unwrap_or_else(|| "no pending review loaded".to_string());
    let retry_state = if retry_cache_ready {
        "immediate handoff retry cache ready"
    } else {
        "immediate handoff retry cache empty"
    };
    let latest_status = if latest_status.trim().is_empty() {
        "local evidence"
    } else {
        latest_status.trim()
    };
    format!(
        "Per-file SDK queue contract stayed local: {pending_state}; {retry_state}; latest status {latest_status}. Typed accepted-queue contract slots are represented as local fields only: queue item identity, local echo id, stable file metadata, upload progress bytes, upload percent, speed, ETA, pause eligibility, resume eligibility, cancel eligibility, retry eligibility, reorder/remove eligibility, SendHandle availability, AbortLocalSend boundary, queued/uploading/sent/failed/canceled result states, error taxonomy, delivery receipt mapping, background persistence, multi-file album grouping, idempotency, stale-handle handling, and adapter promotion blockers. This contract does not inspect SDK queue entries, subscribe to upload progress, pause/resume/cancel uploads, retry accepted queue items, reorder/remove queued media, read delivery receipts, resubmit SendAttachment, send caption-only SendMessage, gateway/runtime/auth, or live mutation. {ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_LABEL}"
    )
}

fn attachment_queue_progress_result_taxonomy_packet_label(
    pending_review: Option<&str>,
    retry_cache_ready: bool,
    latest_status: &str,
) -> String {
    let pending_state = pending_review
        .filter(|pending| !pending.trim().is_empty())
        .map(|pending| format!("pending review {pending}"))
        .unwrap_or_else(|| "no pending review loaded".to_string());
    let retry_state = if retry_cache_ready {
        "immediate handoff retry cache ready"
    } else {
        "immediate handoff retry cache empty"
    };
    let latest_status = if latest_status.trim().is_empty() {
        "local evidence"
    } else {
        latest_status.trim()
    };
    format!(
        "Per-file accepted queue/progress/result taxonomy stayed local: {pending_state}; {retry_state}; latest status {latest_status}. Live references: review-row MatrixRequest::SendAttachment, Timeline::send_attachment().use_send_queue(), timeline local echo progress/error/sent rendering, MatrixRequest::AbortLocalSend with TimelineUpdate::LocalSendAbortResult, and confirmed failed-handoff Retry only. Blocked accepted_queue_operation_id: not_assigned. Blocked queue_item_id/local_echo_id identity: timeline-owned, not available in composer controls. Blocked progress_subscription_result: bytes_sent, bytes_total, percent, speed, and ETA not_subscribed_in_composer. Blocked queue_result: queued, uploading, sent, failed, cancelled, stale not_wired_to_composer_recovery. Blocked delivery_receipt_result: delivered, failed, unknown not_wired. Blocked pause_result and resume_result: not_wired. Blocked accepted_queue_retry_result: not_wired; only immediate worker handoff Retry is confirmed. Blocked cancel_result: timeline local echo SendHandle only; composer accepted-queue cancel not_wired. Blocked reorder_remove_result and background_persistence_result: not_wired. Stale policy: SendHandle generation, source hash, queue item id, and local echo id required before accepted queue promotion. Audit redaction: no raw file path, access token, room secret, caption body, full mention payload, or delivery receipt secret in local packet. This taxonomy performs no SDK queue lookup, progress subscription, upload pause/resume/cancel, accepted queue retry, queue reorder/remove, delivery receipt read, SendAttachment resubmit, caption-only SendMessage, gateway/runtime/auth, or live mutation. {ATTACHMENT_QUEUE_PROGRESS_RESULT_TAXONOMY_PACKET_LABEL}"
    )
}

fn attachment_per_file_status_controls_label(
    control: &str,
    pending_review: Option<&str>,
    retry_cache_ready: bool,
    latest_status: &str,
) -> String {
    let control = control.trim();
    let control_label = if control.is_empty() {
        "Status"
    } else {
        control
    };
    if control_label.eq_ignore_ascii_case("Drilldown") {
        return attachment_per_file_queue_drilldown_label(
            pending_review,
            retry_cache_ready,
            latest_status,
        );
    }
    if control_label.eq_ignore_ascii_case("Contract") {
        return attachment_sdk_queue_contract_packet_label(
            pending_review,
            retry_cache_ready,
            latest_status,
        );
    }
    if control_label.eq_ignore_ascii_case("Taxonomy") {
        return attachment_queue_progress_result_taxonomy_packet_label(
            pending_review,
            retry_cache_ready,
            latest_status,
        );
    }
    let pending_state = pending_review
        .filter(|pending| !pending.trim().is_empty())
        .map(|pending| format!("pending review {pending}"))
        .unwrap_or_else(|| "no pending review loaded".to_string());
    let retry_state = if retry_cache_ready {
        "immediate handoff retry cache ready"
    } else {
        "immediate handoff retry cache empty"
    };
    let latest_status = if latest_status.trim().is_empty() {
        "local evidence"
    } else {
        latest_status.trim()
    };
    format!(
        "Per-file {control_label} control stayed local: {pending_state}; {retry_state}; latest status {latest_status}. Status, Progress, Pause, Resume, Cancel, Retry, Drilldown, Contract, and Taxonomy only update this local file-status copy around the existing SendAttachment/use_send_queue handoff. They do not inspect SDK queue entries, subscribe to upload progress, pause/resume/abort/remove accepted uploads, retry accepted queue items, resubmit SendAttachment, send caption-only SendMessage, map delivery receipts, gateway/runtime/auth, or live mutation. {ATTACHMENT_PER_FILE_STATUS_CONTROLS_LABEL}"
    )
}

fn attachment_mobile_picker_controls_label(
    control: &str,
    pending_review: Option<&str>,
    latest_status: &str,
) -> String {
    let control = control.trim();
    let control_label = if control.is_empty() {
        "Gallery"
    } else {
        control
    };
    let pending_state = pending_review
        .filter(|pending| !pending.trim().is_empty())
        .map(|pending| format!("pending review preserved: {pending}"))
        .unwrap_or_else(|| "no pending review loaded".to_string());
    let latest_status = if latest_status.trim().is_empty() {
        "local evidence"
    } else {
        latest_status.trim()
    };
    format!(
        "Mobile attachment {control_label} control stayed local: {pending_state}; latest status {latest_status}. Gallery, Camera, Files, Contact, Thumbnail, and Share only update mobile picker boundary copy in the attachment picker. Share follows the same local share-sheet boundary. They do not request camera, photo-library, files, or contacts permission; open a mobile picker or system share sheet; invoke a platform share extension; capture media; read contacts or shared media; generate thumbnails; decode full media; create image/video/vCard/share payloads; upload media; submit SendAttachment or SendMessage; clear pending review; cancel SDK queue work; gateway/runtime/auth; or live mutation. {ATTACHMENT_MOBILE_PICKER_CONTROLS_LABEL} {ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_LABEL}"
    )
}

fn attachment_send_preflight_control_from_status(status: &str) -> &'static str {
    let status = status.to_ascii_lowercase();
    if status.contains("request") {
        "Request"
    } else if status.contains("error") || status.contains("failure") {
        "Error"
    } else if status.contains("retry") {
        "Retry"
    } else if status.contains("source") {
        "Source"
    } else {
        "Result"
    }
}

fn attachment_send_preflight_detail_controls_label(
    control: &str,
    pending_review: Option<&str>,
    retry_cache_ready: bool,
    latest_status: &str,
    cached_error: Option<&str>,
    source_copy: &str,
) -> String {
    let control = control.trim();
    let control_label = if control.is_empty() {
        "Result"
    } else {
        control
    };
    let pending_state = pending_review
        .filter(|pending| !pending.trim().is_empty())
        .map(|pending| format!("pending review {pending}"))
        .unwrap_or_else(|| "no pending review loaded".to_string());
    let retry_state = if retry_cache_ready {
        "immediate handoff retry cache ready"
    } else {
        "immediate handoff retry cache empty"
    };
    let latest_status = if latest_status.trim().is_empty() {
        "local evidence"
    } else {
        latest_status.trim()
    };
    let error_state = cached_error
        .filter(|error| !error.trim().is_empty())
        .map(|error| {
            let char_count = error.chars().count();
            let mut preview = error.chars().take(54).collect::<String>();
            if char_count > 54 {
                preview.push_str("...");
            }
            format!("cached error {char_count} chars: {preview}")
        })
        .unwrap_or_else(|| "cached error empty".to_string());
    let source_char_count = source_copy.trim().chars().count();
    format!(
        "Attachment send preflight {control_label} stayed local: {pending_state}; {retry_state}; latest status {latest_status}; {error_state}; source copy {source_char_count} chars. Request, Result, Error, Retry, and Source only summarize pending review, latest local operation status, cached immediate handoff failure, retry readiness, and result-bridge/source evidence around the existing SendAttachment/use_send_queue handoff. They do not submit SendAttachment, retry accepted SDK queue items, subscribe to upload progress, abort/remove/cancel SDK queue work, send caption-only SendMessage, duplicate upload, map delivery receipts, gateway/runtime/auth, or live mutation. {ATTACHMENT_SEND_PREFLIGHT_DETAIL_CONTROLS_LABEL}"
    )
}

#[cfg(test)]
mod attachment_review_lifecycle_metadata_tests {
    use super::*;

    #[test]
    fn attachment_review_lifecycle_metadata_label_summarizes_replacement() {
        let label = attachment_review_lifecycle_metadata_label(
            "selected",
            "Photo",
            Some("new.png"),
            Some("image/png"),
            Some(2048),
            Some("caption: launch image"),
            true,
            None,
            Some("old.png"),
        );

        assert!(label.contains("Attachment selected metadata: Photo"));
        assert!(label.contains("file new.png, image/png, 2.0 KB"));
        assert!(label.contains("caption: launch image"));
        assert!(label.contains("reply context loaded"));
        assert!(label.contains("validation ready"));
        assert!(label.contains("previous pending replaced: old.png"));
        assert!(label.contains(ATTACHMENT_REVIEW_LIFECYCLE_METADATA_LABEL));
    }

    #[test]
    fn attachment_review_lifecycle_metadata_label_reports_empty_close() {
        let label = attachment_review_lifecycle_metadata_label(
            "empty close",
            "Attachment",
            None,
            None,
            None,
            None,
            false,
            Some("selected file is empty"),
            None,
        );

        assert!(label.contains("Attachment empty close metadata: Attachment"));
        assert!(label.contains("no pending attachment loaded"));
        assert!(label.contains("caption: none"));
        assert!(label.contains("reply context none"));
        assert!(label.contains("validation warning loaded: selected file is empty"));
        assert!(label.contains("no upload"));
        assert!(label.contains("SDK queue cancel"));
    }

    #[test]
    fn attachment_send_failure_retry_confirmation_label_summarizes_cached_attempt() {
        let label =
            attachment_send_failure_retry_confirmation_label("launch.png", "Photo", true, true);

        assert!(label.contains("Attachment failed-handoff Retry confirmation"));
        assert!(label.contains("Photo launch.png"));
        assert!(label.contains("caption cached"));
        assert!(label.contains("reply event id cached"));
        assert!(label.contains("PositiveConfirmationModal"));
        assert!(label.contains(ATTACHMENT_SEND_FAILURE_RETRY_CONFIRMATION_LABEL));
        assert!(ATTACHMENT_SEND_FAILURE_RETRY_CONFIRMATION_EVIDENCE.contains("last validated"));
        assert!(
            ATTACHMENT_SEND_FAILURE_RETRY_CONFIRMATION_EVIDENCE
                .contains("MatrixRequest::SendAttachment")
        );
        assert!(
            ATTACHMENT_SEND_FAILURE_RETRY_CONFIRMATION_EVIDENCE.contains("SDK queue retry/resume")
        );
        assert!(
            ATTACHMENT_SEND_FAILURE_RETRY_CONFIRMATION_EVIDENCE.contains("gateway/runtime/auth")
        );
        assert!(ATTACHMENT_SEND_FAILURE_RETRY_CONFIRMATION_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn attachment_send_failure_retry_confirmation_label_uses_empty_fallbacks() {
        let label = attachment_send_failure_retry_confirmation_label("", "File", false, false);

        assert!(label.contains("File attachment"));
        assert!(label.contains("caption none"));
        assert!(label.contains("reply none"));
    }

    #[test]
    fn attachment_multi_file_queue_boundary_label_lists_blocked_controls() {
        let label = attachment_multi_file_queue_boundary_label(Some("Photo launch.png"), true);

        assert!(label.contains("Attachment multi-file/album queue boundary"));
        assert!(label.contains("pending review Photo launch.png"));
        assert!(label.contains("immediate handoff retry cache ready"));
        assert!(label.contains("Multiple-file selection"));
        assert!(label.contains("album grouping"));
        assert!(label.contains("per-file progress rows"));
        assert!(label.contains("background upload list"));
        assert!(label.contains("reorder/remove queued items"));
        assert!(label.contains("bulk retry"));
        assert!(label.contains("accepted SDK queue retry/resume/cancel"));
        assert!(label.contains("delivery receipt fan-in"));
        assert!(label.contains("queue persistence across room switches"));
        assert!(label.contains(ATTACHMENT_MULTI_FILE_QUEUE_BOUNDARY_LABEL));
        assert!(label.contains("extra SendAttachment"));
        assert!(label.contains("caption-only SendMessage"));
        assert!(label.contains("SDK queue abort/remove/cancel"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn attachment_multi_file_queue_boundary_label_reports_empty_state() {
        let label = attachment_multi_file_queue_boundary_label(None, false);

        assert!(label.contains("no pending review loaded"));
        assert!(label.contains("immediate handoff retry cache empty"));
        assert!(label.contains("stay local blocked controls"));
    }

    #[test]
    fn attachment_accepted_queue_actions_row_label_lists_local_controls() {
        let label =
            attachment_accepted_queue_actions_row_label("Pause", Some("Photo launch.png"), true);

        assert!(label.contains("Accepted SDK queue Pause stayed local"));
        assert!(label.contains("pending review Photo launch.png"));
        assert!(label.contains("immediate handoff retry cache ready"));
        assert!(label.contains("Pause, Resume, Reorder, Background, and Clear"));
        assert!(label.contains("SendAttachment/use_send_queue handoff"));
        assert!(label.contains("do not retry or resume accepted SDK queue uploads"));
        assert!(label.contains("abort uploads"));
        assert!(label.contains("remove queued media"));
        assert!(label.contains("reorder SDK queue items"));
        assert!(label.contains("caption-only SendMessage"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_LABEL));
        assert!(ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_EVIDENCE.contains("Pause"));
        assert!(ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_EVIDENCE.contains("Resume"));
        assert!(ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_EVIDENCE.contains("Background"));
        assert!(
            ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_EVIDENCE
                .contains("local accepted attachment queue snapshot")
        );
    }

    #[test]
    fn attachment_accepted_queue_background_snapshot_label_summarizes_local_queue_state() {
        let label = attachment_accepted_queue_background_snapshot_label(
            Some("File launch.pdf"),
            true,
            "Queued after SendAttachment handoff",
        );

        assert!(label.contains("Local accepted attachment queue snapshot"));
        assert!(label.contains("pending review File launch.pdf"));
        assert!(label.contains("immediate handoff retry cache ready"));
        assert!(label.contains("latest status Queued after SendAttachment handoff"));
        assert!(label.contains("accepted SDK queue handle unavailable in composer"));
        assert!(label.contains("background queue manager not opened"));
        assert!(label.contains("SendAttachment/use_send_queue handoff"));
        assert!(label.contains("retry or resume accepted SDK queue uploads"));
        assert!(label.contains("abort uploads"));
        assert!(label.contains("remove queued media"));
        assert!(label.contains("reorder SDK queue items"));
        assert!(label.contains("caption-only SendMessage"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_LABEL));
    }

    #[test]
    fn attachment_accepted_queue_actions_row_label_uses_empty_fallbacks() {
        let label = attachment_accepted_queue_actions_row_label("", None, false);

        assert!(label.contains("Accepted SDK queue Queue action stayed local"));
        assert!(label.contains("no pending review loaded"));
        assert!(label.contains("immediate handoff retry cache empty"));
    }

    #[test]
    fn attachment_accepted_queue_timeline_cancel_bridge_label_points_to_real_handle_path() {
        let label = attachment_accepted_queue_timeline_cancel_bridge_label(
            "Cancel",
            Some("File agenda.pdf"),
            false,
            "queued-confirmed",
        );

        assert!(label.contains("Accepted queue timeline-cancel Cancel stayed local"));
        assert!(label.contains("pending review File agenda.pdf"));
        assert!(label.contains("immediate handoff retry cache empty"));
        assert!(label.contains("latest status queued-confirmed"));
        assert!(label.contains("Status, Handle, Timeline, Cancel, and Source"));
        assert!(label.contains("timeline local echo context menu"));
        assert!(label.contains("local_echo_send_handle"));
        assert!(label.contains("MatrixRequest::AbortLocalSend"));
        assert!(label.contains("does not abort uploads from the composer"));
        assert!(label.contains("resubmit SendAttachment"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ATTACHMENT_ACCEPTED_QUEUE_TIMELINE_CANCEL_BRIDGE_LABEL));
        assert!(
            ATTACHMENT_ACCEPTED_QUEUE_TIMELINE_CANCEL_BRIDGE_EVIDENCE
                .contains("local_echo_send_handle")
        );
        assert!(
            ATTACHMENT_ACCEPTED_QUEUE_TIMELINE_CANCEL_BRIDGE_EVIDENCE
                .contains("MatrixRequest::AbortLocalSend")
        );
    }

    #[test]
    fn attachment_accepted_queue_timeline_cancel_bridge_label_uses_empty_fallbacks() {
        let label = attachment_accepted_queue_timeline_cancel_bridge_label("", None, false, "");

        assert!(label.contains("Accepted queue timeline-cancel Status stayed local"));
        assert!(label.contains("no pending review loaded"));
        assert!(label.contains("latest status local evidence"));
    }

    #[test]
    fn attachment_local_send_abort_result_label_covers_sdk_outcomes() {
        let canceled = attachment_local_send_abort_result_label(&Ok(true));
        assert!(canceled.contains("SDK SendHandle::abort returned canceled"));
        assert!(canceled.contains(ATTACHMENT_LOCAL_SEND_ABORT_RESULT_LABEL));
        assert!(canceled.contains("No composer-held SendHandle"));
        assert!(canceled.contains("SendAttachment resubmit"));

        let not_cancellable = attachment_local_send_abort_result_label(&Ok(false));
        assert!(not_cancellable.contains("already sent or no longer cancellable"));
        assert!(not_cancellable.contains(ATTACHMENT_LOCAL_SEND_ABORT_RESULT_LABEL));
        assert!(not_cancellable.contains("accepted queue retry/resume"));
        assert!(not_cancellable.contains("gateway/runtime/auth"));

        let failed = attachment_local_send_abort_result_label(&Err("stale handle".to_string()));
        assert!(failed.contains("Timeline local echo Cancel Send failed: stale handle"));
        assert!(failed.contains("No automatic retry"));
        assert!(failed.contains("caption-only SendMessage"));
        assert!(failed.contains("live mutation"));

        assert!(
            ATTACHMENT_LOCAL_SEND_ABORT_RESULT_EVIDENCE
                .contains("TimelineUpdate::LocalSendAbortResult")
        );
        assert!(ATTACHMENT_LOCAL_SEND_ABORT_RESULT_EVIDENCE.contains("SendHandle::abort"));
    }

    #[test]
    fn attachment_per_file_status_controls_label_lists_local_controls() {
        let label = attachment_per_file_status_controls_label(
            "Progress",
            Some("File agenda.pdf"),
            true,
            "queued-confirmed",
        );

        assert!(label.contains("Per-file Progress control stayed local"));
        assert!(label.contains("pending review File agenda.pdf"));
        assert!(label.contains("immediate handoff retry cache ready"));
        assert!(label.contains("latest status queued-confirmed"));
        assert!(label.contains(
            "Status, Progress, Pause, Resume, Cancel, Retry, Drilldown, Contract, and Taxonomy"
        ));
        assert!(label.contains("SendAttachment/use_send_queue handoff"));
        assert!(label.contains("do not inspect SDK queue entries"));
        assert!(label.contains("pause/resume/abort/remove accepted uploads"));
        assert!(label.contains("retry accepted queue items"));
        assert!(label.contains("caption-only SendMessage"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ATTACHMENT_PER_FILE_STATUS_CONTROLS_LABEL));
        assert!(ATTACHMENT_PER_FILE_STATUS_CONTROLS_EVIDENCE.contains("Progress"));
        assert!(ATTACHMENT_PER_FILE_STATUS_CONTROLS_EVIDENCE.contains("Drilldown"));
        assert!(ATTACHMENT_PER_FILE_STATUS_CONTROLS_EVIDENCE.contains("Contract"));
        assert!(ATTACHMENT_PER_FILE_STATUS_CONTROLS_EVIDENCE.contains("Taxonomy"));
        assert!(ATTACHMENT_PER_FILE_STATUS_CONTROLS_EVIDENCE.contains("delivery receipts"));
    }

    #[test]
    fn attachment_per_file_queue_drilldown_lists_accepted_send_criteria() {
        let label = attachment_per_file_status_controls_label(
            "Drilldown",
            Some("Photo launch.png"),
            true,
            "queued-only",
        );

        assert!(label.contains("Per-file attachment queue drilldown stayed local"));
        assert!(label.contains("pending review Photo launch.png"));
        assert!(label.contains("immediate handoff retry cache ready"));
        assert!(label.contains("latest status queued-only"));
        assert!(label.contains("Accepted-send queue acceptance criteria"));
        assert!(label.contains("queue item identity"));
        assert!(label.contains("stable file metadata"));
        assert!(label.contains("progress slot"));
        assert!(label.contains("pause eligibility"));
        assert!(label.contains("resume eligibility"));
        assert!(label.contains("cancel eligibility"));
        assert!(label.contains("retry eligibility"));
        assert!(label.contains("timeline local-echo cancel handle"));
        assert!(label.contains("delivery receipt mapping"));
        assert!(label.contains("background persistence"));
        assert!(label.contains("reorder/grouping slots"));
        assert!(label.contains("does not inspect SDK queue entries"));
        assert!(label.contains("subscribe to upload progress"));
        assert!(label.contains("resubmit SendAttachment"));
        assert!(label.contains("caption-only SendMessage"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ATTACHMENT_PER_FILE_QUEUE_DRILLDOWN_LABEL));
        assert!(
            ATTACHMENT_PER_FILE_QUEUE_DRILLDOWN_EVIDENCE
                .contains("accepted-send queue acceptance matrix")
        );
        assert!(ATTACHMENT_PER_FILE_QUEUE_DRILLDOWN_EVIDENCE.contains("SDK queue lookup"));
    }

    #[test]
    fn attachment_sdk_queue_contract_packet_lists_typed_queue_contract() {
        let label = attachment_per_file_status_controls_label(
            "Contract",
            Some("File agenda.pdf"),
            true,
            "queued-only",
        );

        assert!(label.contains("Per-file SDK queue contract stayed local"));
        assert!(label.contains("pending review File agenda.pdf"));
        assert!(label.contains("immediate handoff retry cache ready"));
        assert!(label.contains("latest status queued-only"));
        assert!(label.contains("Typed accepted-queue contract slots"));
        assert!(label.contains("queue item identity"));
        assert!(label.contains("local echo id"));
        assert!(label.contains("upload progress bytes"));
        assert!(label.contains("upload percent"));
        assert!(label.contains("speed"));
        assert!(label.contains("ETA"));
        assert!(label.contains("pause eligibility"));
        assert!(label.contains("resume eligibility"));
        assert!(label.contains("cancel eligibility"));
        assert!(label.contains("retry eligibility"));
        assert!(label.contains("reorder/remove eligibility"));
        assert!(label.contains("SendHandle availability"));
        assert!(label.contains("AbortLocalSend boundary"));
        assert!(label.contains("queued/uploading/sent/failed/canceled"));
        assert!(label.contains("error taxonomy"));
        assert!(label.contains("delivery receipt mapping"));
        assert!(label.contains("background persistence"));
        assert!(label.contains("multi-file album grouping"));
        assert!(label.contains("idempotency"));
        assert!(label.contains("stale-handle handling"));
        assert!(label.contains("adapter promotion blockers"));
        assert!(label.contains("does not inspect SDK queue entries"));
        assert!(label.contains("subscribe to upload progress"));
        assert!(label.contains("reorder/remove queued media"));
        assert!(label.contains("resubmit SendAttachment"));
        assert!(label.contains("caption-only SendMessage"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_LABEL));
    }

    #[test]
    fn attachment_queue_progress_result_taxonomy_packet_lists_blocked_results() {
        let label = attachment_per_file_status_controls_label(
            "Taxonomy",
            Some("Photo launch.png"),
            true,
            "queued-only",
        );

        assert!(label.contains("accepted queue/progress/result taxonomy"));
        assert!(label.contains("pending review Photo launch.png"));
        assert!(label.contains("immediate handoff retry cache ready"));
        assert!(label.contains("latest status queued-only"));
        assert!(label.contains("MatrixRequest::SendAttachment"));
        assert!(label.contains("Timeline::send_attachment().use_send_queue()"));
        assert!(label.contains("TimelineUpdate::LocalSendAbortResult"));
        assert!(label.contains("accepted_queue_operation_id: not_assigned"));
        assert!(label.contains("queue_item_id/local_echo_id identity"));
        assert!(label.contains("progress_subscription_result"));
        assert!(label.contains("queue_result: queued, uploading, sent, failed, cancelled, stale"));
        assert!(label.contains("delivery_receipt_result"));
        assert!(label.contains("pause_result and resume_result"));
        assert!(label.contains("accepted_queue_retry_result: not_wired"));
        assert!(label.contains("cancel_result: timeline local echo SendHandle only"));
        assert!(label.contains("reorder_remove_result"));
        assert!(label.contains("background_persistence_result"));
        assert!(label.contains("Stale policy"));
        assert!(label.contains("Audit redaction"));
        assert!(label.contains(ATTACHMENT_QUEUE_PROGRESS_RESULT_TAXONOMY_PACKET_LABEL));
        assert!(
            ATTACHMENT_QUEUE_PROGRESS_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("accepted queue/progress/result taxonomy packet")
        );
        assert!(
            ATTACHMENT_QUEUE_PROGRESS_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("TimelineUpdate::LocalSendAbortResult")
        );
        assert!(label.contains("no SDK queue lookup"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn attachment_sdk_queue_contract_evidence_names_boundaries() {
        assert!(ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_EVIDENCE.contains("typed SDK queue contract"));
        assert!(ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_EVIDENCE.contains("upload progress bytes"));
        assert!(
            ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_EVIDENCE
                .contains("pause/resume/cancel/retry/reorder/remove eligibility")
        );
        assert!(ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_EVIDENCE.contains("SendHandle"));
        assert!(ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_EVIDENCE.contains("AbortLocalSend"));
        assert!(ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_EVIDENCE.contains("delivery receipt mapping"));
        assert!(
            ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_EVIDENCE.contains("multi-file album grouping")
        );
        assert!(ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_EVIDENCE.contains("idempotency"));
        assert!(
            ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_EVIDENCE.contains("performs no SDK queue lookup")
        );
        assert!(ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn attachment_per_file_status_controls_label_uses_empty_fallbacks() {
        let label = attachment_per_file_status_controls_label("", None, false, "");

        assert!(label.contains("Per-file Status control stayed local"));
        assert!(label.contains("no pending review loaded"));
        assert!(label.contains("immediate handoff retry cache empty"));
        assert!(label.contains("latest status local evidence"));
    }

    #[test]
    fn attachment_mobile_picker_controls_label_keeps_mobile_entries_local() {
        let label = attachment_mobile_picker_controls_label(
            "Thumbnail",
            Some("Photo launch.png"),
            "review-pending",
        );

        assert!(label.contains("Mobile attachment Thumbnail control stayed local"));
        assert!(label.contains("pending review preserved: Photo launch.png"));
        assert!(label.contains("latest status review-pending"));
        assert!(label.contains("Gallery, Camera, Files, Contact, Thumbnail, and Share"));
        assert!(label.contains("do not request camera"));
        assert!(label.contains("photo-library"));
        assert!(label.contains("files"));
        assert!(label.contains("contacts permission"));
        assert!(label.contains("system share sheet"));
        assert!(label.contains("platform share extension"));
        assert!(label.contains("generate thumbnails"));
        assert!(label.contains("decode full media"));
        assert!(label.contains("vCard/share payloads"));
        assert!(label.contains("SendAttachment"));
        assert!(label.contains("SendMessage"));
        assert!(label.contains("clear pending review"));
        assert!(label.contains("cancel SDK queue work"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ATTACHMENT_MOBILE_PICKER_CONTROLS_LABEL));
        assert!(
            ATTACHMENT_MOBILE_PICKER_CONTROLS_EVIDENCE
                .contains("visible local mobile picker controls")
        );
        assert!(ATTACHMENT_MOBILE_PICKER_CONTROLS_EVIDENCE.contains("Share"));
        assert!(ATTACHMENT_MOBILE_PICKER_CONTROLS_EVIDENCE.contains("system share sheet"));
        assert!(
            ATTACHMENT_MOBILE_PICKER_CONTROLS_EVIDENCE
                .contains("does not request camera permission")
        );
        assert!(
            ATTACHMENT_MOBILE_PICKER_CONTROLS_EVIDENCE
                .contains("submit MatrixRequest::SendAttachment")
        );
    }

    #[test]
    fn attachment_mobile_share_sheet_boundary_label_is_local_only() {
        let label = attachment_mobile_picker_controls_label(
            "Share",
            Some("File launch.pdf"),
            "review-pending",
        );

        assert!(label.contains("Mobile attachment Share control stayed local"));
        assert!(label.contains("pending review preserved: File launch.pdf"));
        assert!(label.contains("system share sheet"));
        assert!(label.contains("platform share extension"));
        assert!(label.contains("shared media"));
        assert!(label.contains("share payloads"));
        assert!(label.contains("upload media"));
        assert!(label.contains("SendAttachment"));
        assert!(label.contains("SendMessage"));
        assert!(label.contains(ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_LABEL));
        assert!(ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_EVIDENCE.contains("Share"));
        assert!(
            ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_EVIDENCE.contains("opens no system share sheet")
        );
        assert!(
            ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_EVIDENCE
                .contains("invokes no platform share extension")
        );
        assert!(
            ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_EVIDENCE
                .contains("submits no MatrixRequest::SendAttachment")
        );
        assert!(ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_EVIDENCE.contains("gateway/runtime/auth"));
        assert!(ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn attachment_mobile_picker_controls_label_uses_empty_fallbacks() {
        let label = attachment_mobile_picker_controls_label("", None, "");

        assert!(label.contains("Mobile attachment Gallery control stayed local"));
        assert!(label.contains("no pending review loaded"));
        assert!(label.contains("latest status local evidence"));
    }

    #[test]
    fn attachment_send_preflight_detail_controls_label_summarizes_cached_failure() {
        let label = attachment_send_preflight_detail_controls_label(
            "Error",
            Some("Photo launch.png"),
            true,
            "failure-copy",
            Some("upload worker failed before SDK queue ownership"),
            "SendAttachment worker failure evidence",
        );

        assert!(label.contains("Attachment send preflight Error stayed local"));
        assert!(label.contains("pending review Photo launch.png"));
        assert!(label.contains("immediate handoff retry cache ready"));
        assert!(label.contains("latest status failure-copy"));
        assert!(label.contains("cached error"));
        assert!(label.contains("upload worker failed before SDK queue ownership"));
        assert!(label.contains("source copy 38 chars"));
        assert!(label.contains("Request, Result, Error, Retry, and Source"));
        assert!(label.contains("SendAttachment/use_send_queue handoff"));
        assert!(label.contains("do not submit SendAttachment"));
        assert!(label.contains("retry accepted SDK queue items"));
        assert!(label.contains("caption-only SendMessage"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ATTACHMENT_SEND_PREFLIGHT_DETAIL_CONTROLS_LABEL));
        assert!(ATTACHMENT_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("visible Request"));
        assert!(
            ATTACHMENT_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("cached immediate handoff failure text")
        );
    }

    #[test]
    fn attachment_send_preflight_detail_controls_label_reports_empty_state() {
        let label = attachment_send_preflight_detail_controls_label("", None, false, "", None, "");

        assert!(label.contains("Attachment send preflight Result stayed local"));
        assert!(label.contains("no pending review loaded"));
        assert!(label.contains("immediate handoff retry cache empty"));
        assert!(label.contains("latest status local evidence"));
        assert!(label.contains("cached error empty"));
        assert!(label.contains("source copy 0 chars"));
    }

    #[test]
    fn attachment_send_preflight_control_from_status_maps_local_status() {
        assert_eq!(
            attachment_send_preflight_control_from_status("send-preflight-request-local"),
            "Request"
        );
        assert_eq!(
            attachment_send_preflight_control_from_status("failure-copy"),
            "Error"
        );
        assert_eq!(
            attachment_send_preflight_control_from_status("retry-confirmed"),
            "Retry"
        );
        assert_eq!(
            attachment_send_preflight_control_from_status("queued-confirmed"),
            "Result"
        );
    }
}

fn emoji_sticker_lifecycle_metadata_label(
    action: &str,
    panel_visible: bool,
    last_choice: Option<&str>,
    stage_count: usize,
    local_status: Option<&str>,
) -> String {
    let panel_state = if panel_visible {
        "panel visible"
    } else {
        "panel hidden"
    };
    let choice_state = last_choice
        .filter(|choice| !choice.trim().is_empty())
        .map(|choice| format!("last choice {choice}"))
        .unwrap_or_else(|| "no staged emoji/sticker choice".to_string());
    let status_state = local_status
        .filter(|status| !status.trim().is_empty())
        .unwrap_or("no local emoji/sticker status");
    format!(
        "Emoji/sticker lifecycle {action}: {panel_state}; {choice_state}; staged count {stage_count}; status: {status_state}. {EMOJI_STICKER_LIFECYCLE_METADATA_LABEL}"
    )
}

#[cfg(test)]
mod emoji_sticker_lifecycle_metadata_tests {
    use super::*;

    #[test]
    fn emoji_sticker_lifecycle_metadata_label_summarizes_repeated_selection() {
        let label = emoji_sticker_lifecycle_metadata_label(
            "staged Heart",
            true,
            Some("Heart"),
            3,
            Some("Heart emoji/sticker preview staged locally"),
        );

        assert!(label.contains("Emoji/sticker lifecycle staged Heart"));
        assert!(label.contains("panel visible"));
        assert!(label.contains("last choice Heart"));
        assert!(label.contains("staged count 3"));
        assert!(label.contains("Heart emoji/sticker preview staged locally"));
        assert!(label.contains(EMOJI_STICKER_LIFECYCLE_METADATA_LABEL));
        assert!(
            EMOJI_STICKER_LIFECYCLE_METADATA_EVIDENCE
                .contains("repeated Smile/Thumbs/Heart/Sticker staging")
        );
        assert!(EMOJI_STICKER_LIFECYCLE_METADATA_EVIDENCE.contains("last staged choice"));
        assert!(EMOJI_STICKER_LIFECYCLE_METADATA_EVIDENCE.contains("staged choice count"));
        assert!(EMOJI_STICKER_LIFECYCLE_METADATA_EVIDENCE.contains("remote picker/search"));
        assert!(EMOJI_STICKER_LIFECYCLE_METADATA_EVIDENCE.contains("gateway/runtime/auth"));
        assert!(EMOJI_STICKER_LIFECYCLE_METADATA_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn emoji_sticker_lifecycle_metadata_label_uses_empty_fallbacks() {
        let label = emoji_sticker_lifecycle_metadata_label("closed", false, None, 0, Some(""));

        assert!(label.contains("Emoji/sticker lifecycle closed"));
        assert!(label.contains("panel hidden"));
        assert!(label.contains("no staged emoji/sticker choice"));
        assert!(label.contains("staged count 0"));
        assert!(label.contains("no local emoji/sticker status"));
    }
}

fn voice_message_lifecycle_metadata_label(
    action: &str,
    panel_visible: bool,
    local_status: Option<&str>,
    pending_audio_filename: Option<&str>,
    pending_audio_duration: Option<&str>,
    reply_context_loaded: bool,
    picker_state: &str,
) -> String {
    let panel_state = if panel_visible {
        "panel visible"
    } else {
        "panel hidden"
    };
    let status_state = local_status
        .filter(|status| !status.trim().is_empty())
        .unwrap_or("no local voice control staged");
    let audio_state = pending_audio_filename
        .filter(|filename| !filename.trim().is_empty())
        .map(|filename| {
            let duration = pending_audio_duration
                .filter(|duration| !duration.trim().is_empty())
                .unwrap_or("duration: unavailable before recorder/player metadata");
            format!("pending audio {filename}; {duration}")
        })
        .unwrap_or_else(|| "no pending audio review loaded".to_string());
    let reply_state = if reply_context_loaded {
        "reply context loaded"
    } else {
        "reply context none"
    };
    let picker_state = if picker_state.trim().is_empty() {
        "no confirmation or picker pending"
    } else {
        picker_state.trim()
    };
    format!(
        "Voice lifecycle {action}: {panel_state}; status: {status_state}; {audio_state}; {reply_state}; picker state: {picker_state}. {VOICE_MESSAGE_LIFECYCLE_METADATA_LABEL}"
    )
}

fn voice_confirmation_cancel_metadata_label(
    pending_audio_filename: Option<&str>,
    reply_context_loaded: bool,
) -> String {
    let pending_state = pending_audio_filename
        .filter(|filename| !filename.trim().is_empty())
        .map(|filename| format!("pending audio review preserved: {filename}"))
        .unwrap_or_else(|| "no pending audio review; picker preview hidden".to_string());
    let reply_state = if reply_context_loaded {
        "reply context preserved"
    } else {
        "reply context none"
    };
    format!(
        "Voice confirmation canceled locally: {pending_state}; {reply_state}. {VOICE_MESSAGE_CONFIRMATION_CANCEL_METADATA_LABEL}"
    )
}

fn voice_message_recorder_waveform_codec_boundary_label(
    action: &str,
    panel_visible: bool,
    pending_audio_filename: Option<&str>,
) -> String {
    let panel_state = if panel_visible {
        "recorder panel visible"
    } else {
        "recorder panel hidden"
    };
    let audio_state = pending_audio_filename
        .filter(|filename| !filename.trim().is_empty())
        .map(|filename| format!("desktop audio review preserved: {filename}"))
        .unwrap_or_else(|| "no captured recorder payload".to_string());

    format!(
        "Voice recorder boundary {action}: {panel_state}; {audio_state}. {VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_LABEL}"
    )
}

fn voice_message_recorder_status_controls_label(
    control: Option<&str>,
    panel_visible: bool,
    pending_audio_filename: Option<&str>,
    pending_audio_duration: Option<&str>,
    pending_audio_waveform_codec: Option<&str>,
) -> String {
    let control_state = control
        .filter(|control| !control.trim().is_empty())
        .map(|control| format!("{control} selected"))
        .unwrap_or_else(|| "no recorder status control selected".to_string());
    let panel_state = if panel_visible {
        "voice panel visible"
    } else {
        "voice panel hidden"
    };
    let audio_state = pending_audio_filename
        .filter(|filename| !filename.trim().is_empty())
        .map(|filename| {
            let duration = pending_audio_duration
                .filter(|duration| !duration.trim().is_empty())
                .unwrap_or("duration unavailable before player metadata");
            let waveform_codec = pending_audio_waveform_codec
                .filter(|label| !label.trim().is_empty())
                .unwrap_or("selected-audio waveform/codec unavailable");
            format!("desktop audio review visible: {filename}; {duration}; {waveform_codec}")
        })
        .unwrap_or_else(|| "no captured recorder payload or pending audio upload".to_string());

    format!(
        "Voice recorder status controls {control_state}: {panel_state}; {audio_state}. Timer, Transcript, and Progress update local status only; Waveform and Codec summarize already selected desktop WAV files with capped local RIFF/fmt/data parsing when available. {VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_LABEL} {VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_LABEL}"
    )
}

fn voice_message_capture_lifecycle_controls_label(
    control: Option<&str>,
    panel_visible: bool,
    pending_audio_filename: Option<&str>,
    pending_audio_duration: Option<&str>,
) -> String {
    if matches!(control.map(str::trim), Some("Packet")) {
        return voice_message_recorder_lifecycle_drilldown_packet_label(
            panel_visible,
            pending_audio_filename,
            pending_audio_duration,
            false,
            "local recorder lifecycle drilldown evidence",
            None,
            VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE,
        );
    }
    if matches!(control.map(str::trim), Some("Contract")) {
        return voice_message_recorder_typed_contract_packet_label(
            panel_visible,
            pending_audio_filename,
            pending_audio_duration,
            false,
            "local recorder typed contract evidence",
            None,
            VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE,
        );
    }
    if matches!(control.map(str::trim), Some("Taxonomy")) {
        return voice_message_recorder_result_taxonomy_packet_label(
            panel_visible,
            pending_audio_filename,
            pending_audio_duration,
            false,
            "local recorder result taxonomy evidence",
            None,
            VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE,
        );
    }
    let control_state = control
        .filter(|control| !control.trim().is_empty())
        .map(|control| format!("{control} selected"))
        .unwrap_or_else(|| "no capture lifecycle control selected".to_string());
    if matches!(control.map(str::trim), Some("Permission" | "Upload")) {
        return voice_message_capture_request_packet_snapshot_label(
            control.unwrap_or("Request"),
            panel_visible,
            pending_audio_filename,
            pending_audio_duration,
            false,
            "local capture lifecycle evidence",
            None,
            VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE,
        );
    }
    let panel_state = if panel_visible {
        "voice panel visible"
    } else {
        "voice panel hidden"
    };
    let audio_state = pending_audio_filename
        .filter(|filename| !filename.trim().is_empty())
        .map(|filename| {
            let duration = pending_audio_duration
                .filter(|duration| !duration.trim().is_empty())
                .unwrap_or("duration unavailable before captured recorder metadata");
            format!("desktop audio review preserved: {filename}; {duration}")
        })
        .unwrap_or_else(|| "no captured recorder payload or pending upload".to_string());

    format!(
        "Voice capture lifecycle controls {control_state}: {panel_state}; {audio_state}. Permission, Capture, Encode, Review, Upload, Packet, Contract, and Taxonomy update local capture lifecycle metadata only. No microphone permission, privacy entitlement, audio session activation, platform recorder, captured local audio file, temporary recording write, waveform sampling, duration capture, codec conversion, transcription, SendAttachment, SendMessage fallback, SDK send-queue work, upload progress subscription, gateway/runtime/auth, or live mutation. {VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_LABEL}"
    )
}

fn voice_message_capture_request_packet_snapshot_label(
    control: &str,
    panel_visible: bool,
    pending_audio_filename: Option<&str>,
    pending_audio_duration: Option<&str>,
    retry_cache_ready: bool,
    latest_status: &str,
    cached_error: Option<&str>,
    source_copy: &str,
) -> String {
    let control = control.trim();
    let control_label = if control.is_empty() {
        "Request"
    } else {
        control
    };
    let panel_state = if panel_visible {
        "voice panel visible"
    } else {
        "voice panel hidden"
    };
    let audio_state = pending_audio_filename
        .filter(|filename| !filename.trim().is_empty())
        .map(|filename| {
            let duration = pending_audio_duration
                .filter(|duration| !duration.trim().is_empty())
                .unwrap_or("duration unavailable before recorder/player metadata");
            format!("pending desktop audio review {filename}; {duration}")
        })
        .unwrap_or_else(|| "no captured recorder payload or pending audio review".to_string());
    let retry_state = if retry_cache_ready {
        "voice retry cache ready"
    } else {
        "voice retry cache empty"
    };
    let latest_status = if latest_status.trim().is_empty() {
        "local voice capture/request evidence"
    } else {
        latest_status.trim()
    };
    let error_state = cached_error
        .filter(|error| !error.trim().is_empty())
        .map(|error| {
            let char_count = error.chars().count();
            let mut preview = error.chars().take(54).collect::<String>();
            if char_count > 54 {
                preview.push_str("...");
            }
            format!("cached error {char_count} chars: {preview}")
        })
        .unwrap_or_else(|| "cached error empty".to_string());
    let source_char_count = source_copy.trim().chars().count();

    format!(
        "Local voice capture/request packet snapshot: {control_label} selected. {panel_state}; {audio_state}; {retry_state}; latest status {latest_status}; {error_state}; source copy {source_char_count} chars. Proposed microphone permission request, capture session slot, encoder job slot, review payload slot, upload request body, result slot, error slot, retry eligibility, source summary, voice-message contract target, attachment handoff target, and mobile picker target are represented as local metadata only. No microphone permission, privacy entitlement, audio session activation, platform recorder, captured local audio file, temporary recording write, waveform sampling, duration capture, transcription, codec conversion, upload progress subscription, MatrixRequest::SendAttachment, SendMessage fallback, SDK send-queue work, gateway/runtime/auth, or live mutation was submitted. {VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_LABEL} {VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_LABEL}"
    )
}

fn voice_message_recorder_lifecycle_drilldown_packet_label(
    panel_visible: bool,
    pending_audio_filename: Option<&str>,
    pending_audio_duration: Option<&str>,
    retry_cache_ready: bool,
    latest_status: &str,
    cached_error: Option<&str>,
    source_copy: &str,
) -> String {
    let panel_state = if panel_visible {
        "voice panel visible"
    } else {
        "voice panel hidden"
    };
    let audio_state = pending_audio_filename
        .filter(|filename| !filename.trim().is_empty())
        .map(|filename| {
            let duration = pending_audio_duration
                .filter(|duration| !duration.trim().is_empty())
                .unwrap_or("duration unavailable before recorder/player metadata");
            format!("pending desktop audio review {filename}; {duration}")
        })
        .unwrap_or_else(|| "no captured recorder payload or pending audio review".to_string());
    let retry_state = if retry_cache_ready {
        "voice retry cache ready"
    } else {
        "voice retry cache empty"
    };
    let latest_status = if latest_status.trim().is_empty() {
        "local recorder lifecycle evidence"
    } else {
        latest_status.trim()
    };
    let error_state = cached_error
        .filter(|error| !error.trim().is_empty())
        .map(|error| {
            let char_count = error.chars().count();
            let mut preview = error.chars().take(54).collect::<String>();
            if char_count > 54 {
                preview.push_str("...");
            }
            format!("cached error {char_count} chars: {preview}")
        })
        .unwrap_or_else(|| "cached error empty".to_string());
    let source_char_count = source_copy.trim().chars().count();

    format!(
        "Voice recorder lifecycle drilldown packet: {panel_state}; {audio_state}; {retry_state}; latest status {latest_status}; {error_state}; source copy {source_char_count} chars. Acceptance matrix keeps microphone permission, privacy entitlement, audio session activation, recorder start/lock/cancel, temporary capture file lifecycle, waveform sampling/rendering, timer/duration capture, codec/encoding/transcription, review playback/drop cleanup, mobile picker/share sheet, upload queue, result/error/retry/source slots, and confirmed desktop audio review SendAttachment as local metadata only. No microphone permission, privacy entitlement, audio session activation, platform recorder, captured local audio file, temporary recording write, waveform sampling, duration capture, media decode, audio player, codec conversion, transcription service, upload progress subscription, SDK queue control, extra MatrixRequest::SendAttachment, unconfirmed retry, SendMessage fallback, room-state, membership, account/profile, gateway/runtime/auth, or live mutation was submitted. {VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_LABEL} {VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_LABEL} {VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_LABEL}"
    )
}

fn voice_message_recorder_typed_contract_packet_label(
    panel_visible: bool,
    pending_audio_filename: Option<&str>,
    pending_audio_duration: Option<&str>,
    retry_cache_ready: bool,
    latest_status: &str,
    cached_error: Option<&str>,
    source_copy: &str,
) -> String {
    let panel_state = if panel_visible {
        "voice panel visible"
    } else {
        "voice panel hidden"
    };
    let audio_state = pending_audio_filename
        .filter(|filename| !filename.trim().is_empty())
        .map(|filename| {
            let duration = pending_audio_duration
                .filter(|duration| !duration.trim().is_empty())
                .unwrap_or("duration unavailable before recorder/player metadata");
            format!("pending desktop audio review {filename}; {duration}")
        })
        .unwrap_or_else(|| "no captured recorder payload or pending audio review".to_string());
    let retry_state = if retry_cache_ready {
        "voice retry cache ready"
    } else {
        "voice retry cache empty"
    };
    let latest_status = if latest_status.trim().is_empty() {
        "local recorder typed contract evidence"
    } else {
        latest_status.trim()
    };
    let error_state = cached_error
        .filter(|error| !error.trim().is_empty())
        .map(|error| {
            let char_count = error.chars().count();
            let mut preview = error.chars().take(54).collect::<String>();
            if char_count > 54 {
                preview.push_str("...");
            }
            format!("cached error {char_count} chars: {preview}")
        })
        .unwrap_or_else(|| "cached error empty".to_string());
    let source_char_count = source_copy.trim().chars().count();

    format!(
        "Voice recorder typed contract packet: {panel_state}; {audio_state}; {retry_state}; latest status {latest_status}; {error_state}; source copy {source_char_count} chars. Typed contracts cover microphone permission request/result/error, privacy entitlement, audio session lifecycle, recorder session start/lock/cancel, capture file identity and cleanup, waveform sampling/rendering, timer/duration capture, codec/encoding/transcription result taxonomy, review playback/drop cleanup, mobile picker/share sheet handoff, upload queue progress/result/error/retry/source slots, confirmed desktop audio review SendAttachment result mapping, stale capture handling, idempotency, and adapter promotion blockers before recorder or captured-upload work can be promoted. No microphone permission, privacy entitlement, audio session activation, platform recorder, captured local audio file, temporary recording write, waveform sampling, duration capture, media decode, audio player, codec conversion, transcription service, upload progress subscription, SDK queue control, extra MatrixRequest::SendAttachment, unconfirmed retry, SendMessage fallback, room-state, membership, account/profile, gateway/runtime/auth, or live mutation was submitted. {VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_LABEL} {VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_LABEL} {VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_LABEL}"
    )
}

fn voice_message_recorder_result_taxonomy_packet_label(
    panel_visible: bool,
    pending_audio_filename: Option<&str>,
    pending_audio_duration: Option<&str>,
    retry_cache_ready: bool,
    latest_status: &str,
    cached_error: Option<&str>,
    source_copy: &str,
) -> String {
    let panel_state = if panel_visible {
        "voice panel visible"
    } else {
        "voice panel hidden"
    };
    let audio_state = pending_audio_filename
        .filter(|filename| !filename.trim().is_empty())
        .map(|filename| {
            let duration = pending_audio_duration
                .filter(|duration| !duration.trim().is_empty())
                .unwrap_or("duration unavailable before recorder/player metadata");
            format!("pending desktop audio review {filename}; {duration}")
        })
        .unwrap_or_else(|| "no captured recorder payload or pending audio review".to_string());
    let retry_state = if retry_cache_ready {
        "voice retry cache ready"
    } else {
        "voice retry cache empty"
    };
    let latest_status = if latest_status.trim().is_empty() {
        "local recorder result taxonomy evidence"
    } else {
        latest_status.trim()
    };
    let error_state = cached_error
        .filter(|error| !error.trim().is_empty())
        .map(|error| {
            let char_count = error.chars().count();
            let mut preview = error.chars().take(54).collect::<String>();
            if char_count > 54 {
                preview.push_str("...");
            }
            format!("cached error {char_count} chars: {preview}")
        })
        .unwrap_or_else(|| "cached error empty".to_string());
    let source_char_count = source_copy.trim().chars().count();

    format!(
        "Voice recorder result taxonomy packet: {panel_state}; {audio_state}; {retry_state}; latest status {latest_status}; {error_state}; source copy {source_char_count} chars. Live references remain confirmed desktop audio review MatrixRequest::SendAttachment, Timeline::send_attachment().use_send_queue(), confirmed failed-handoff Retry, selected-audio bounded WAV metadata/waveform analysis, review Play local system-opener handoff, and Drop pending-audio local cleanup. microphone_permission_operation_id not_assigned; privacy_entitlement_result not_wired; audio_session_id not_assigned; recorder_session_id not_assigned; capture_file_identity not_assigned; waveform_timer_result not_wired; codec_transcription_result not_wired; review_player_result not_wired; mobile_picker_share_result not_wired; captured_upload_queue_item_id not_assigned; delivery_result not_wired; stale_capture_result not_wired; retry_cancel_result not_wired; audit_redaction raw_path_microphone_buffer_transcript_redacted. No microphone permission, privacy entitlement, audio session activation, platform recorder, captured local audio file, temporary recording write, waveform sampling, duration capture, media decode, inline audio player, codec conversion, transcription service, upload progress subscription, SDK queue control, extra MatrixRequest::SendAttachment, unconfirmed retry, SendMessage fallback, room-state, membership, account/profile, gateway/runtime/auth, or live mutation was submitted. {VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_LABEL} {VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_LABEL} {VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_LABEL}"
    )
}

fn voice_message_mobile_picker_controls_label(
    control: Option<&str>,
    panel_visible: bool,
    pending_audio_filename: Option<&str>,
    pending_audio_duration: Option<&str>,
    latest_status: &str,
) -> String {
    let control_state = control
        .filter(|control| !control.trim().is_empty())
        .map(|control| format!("{control} selected"))
        .unwrap_or_else(|| "no mobile picker control selected".to_string());
    let panel_state = if panel_visible {
        "voice panel visible"
    } else {
        "voice panel hidden"
    };
    let audio_state = pending_audio_filename
        .filter(|filename| !filename.trim().is_empty())
        .map(|filename| {
            let duration = pending_audio_duration
                .filter(|duration| !duration.trim().is_empty())
                .unwrap_or("duration unavailable before mobile picker metadata");
            format!("desktop audio review preserved: {filename}; {duration}")
        })
        .unwrap_or_else(|| "no mobile voice picker payload or pending upload".to_string());
    let latest_status = if latest_status.trim().is_empty() {
        "local mobile picker evidence"
    } else {
        latest_status.trim()
    };

    format!(
        "Voice mobile picker controls {control_state}: {panel_state}; {audio_state}; latest status {latest_status}. Mic, Files, Library, Retake, and Share update local mobile picker metadata only. No mobile microphone permission, privacy entitlement, mobile document picker, photo/audio library picker, capture session, captured local audio file, retake deletion, system share sheet, external handoff, SendAttachment, SendMessage fallback, SDK send-queue work, upload progress subscription, gateway/runtime/auth, or live mutation. {VOICE_MESSAGE_MOBILE_PICKER_CONTROLS_LABEL}"
    )
}

fn voice_message_review_playback_controls_label(
    control: Option<&str>,
    panel_visible: bool,
    pending_audio_filename: Option<&str>,
    pending_audio_duration: Option<&str>,
    latest_status: &str,
) -> String {
    let control_state = control
        .filter(|control| !control.trim().is_empty())
        .map(|control| format!("{control} selected"))
        .unwrap_or_else(|| "no review playback control selected".to_string());
    let panel_state = if panel_visible {
        "voice panel visible"
    } else {
        "voice panel hidden"
    };
    let audio_state = pending_audio_filename
        .filter(|filename| !filename.trim().is_empty())
        .map(|filename| {
            let duration = pending_audio_duration
                .filter(|duration| !duration.trim().is_empty())
                .unwrap_or("duration unavailable before local player metadata");
            format!("desktop audio review available: {filename}; {duration}")
        })
        .unwrap_or_else(|| "no pending voice review audio loaded".to_string());
    let latest_status = if latest_status.trim().is_empty() {
        "local voice review evidence"
    } else {
        latest_status.trim()
    };

    format!(
        "Voice review playback controls {control_state}: {panel_state}; {audio_state}; latest status {latest_status}. Play opens the pending desktop audio review with the system opener when a readable local file exists; Pause, Scrub, and Speed update local review metadata only; Drop performs the real pending-audio cleanup handoff when one exists. No inline audio player, media decode, waveform sampling, playback position subscription, speed transform, scrubber timeline, local file deletion, SendAttachment, SendMessage fallback, SDK send-queue work, upload progress subscription, gateway/runtime/auth, or live mutation. {VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_LABEL}"
    )
}

fn open_voice_review_audio_file(path: &Path) -> Result<(), String> {
    let metadata = fs::metadata(path)
        .map_err(|error| format!("selected audio file is unreadable: {error}"))?;
    if !metadata.is_file() {
        return Err("selected audio path is not a regular file".to_string());
    }
    let file_url = url::Url::from_file_path(path)
        .map_err(|_| "selected audio path cannot be converted to a file URL".to_string())?;
    robius_open::Uri::new(file_url.as_str())
        .open()
        .map_err(|error| format!("system opener failed: {error:?}"))
}

fn voice_message_review_playback_open_result_label(
    filename: &str,
    duration_label: Option<&str>,
    result_state: &str,
) -> String {
    let filename = if filename.trim().is_empty() {
        "pending audio"
    } else {
        filename.trim()
    };
    let duration = duration_label
        .filter(|duration| !duration.trim().is_empty())
        .unwrap_or("duration unavailable before local opener playback");
    let result_state = if result_state.trim().is_empty() {
        "system opener result unavailable"
    } else {
        result_state.trim()
    };

    format!(
        "Voice review Play local opener: {filename}; {duration}; {result_state}. Play uses only the pending desktop Voice attachment local file path and the system opener. It submits no SendAttachment, SendMessage fallback, SDK queue work, recorder request, media decode, inline player, waveform sampling, file deletion, gateway/runtime/auth, or live mutation. {VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_LABEL}"
    )
}

fn voice_message_review_drop_pending_audio_label(
    pending_audio_filename: Option<&str>,
    pending_audio_duration: Option<&str>,
    retry_cache_cleared: bool,
) -> String {
    let audio_state = pending_audio_filename
        .filter(|filename| !filename.trim().is_empty())
        .map(|filename| {
            let duration = pending_audio_duration
                .filter(|duration| !duration.trim().is_empty())
                .unwrap_or("duration unavailable before local player metadata");
            format!("dropped pending desktop audio review {filename}; {duration}")
        })
        .unwrap_or_else(|| "no pending voice review audio loaded".to_string());
    let retry_state = if retry_cache_cleared {
        "voice failed-handoff retry cache cleared"
    } else {
        "no voice failed-handoff retry cache loaded"
    };

    format!(
        "Voice review Drop: {audio_state}; {retry_state}. Drop consumes only local pending Voice review state, preserves composer caption/reply text, deletes no local file, and submits no SendAttachment, SendMessage fallback, SDK queue cancel, gateway/runtime/auth, or live mutation. {VOICE_MESSAGE_REVIEW_DROP_PENDING_AUDIO_LABEL}"
    )
}

fn voice_message_send_preflight_control_from_status(status: &str) -> &'static str {
    let status = status.to_ascii_lowercase();
    if status.contains("request") || status.contains("confirmation") || status.contains("picker") {
        "Request"
    } else if status.contains("error") || status.contains("failure") {
        "Error"
    } else if status.contains("retry") {
        "Retry"
    } else if status.contains("source") {
        "Source"
    } else {
        "Result"
    }
}

fn voice_message_send_preflight_detail_controls_label(
    control: &str,
    panel_visible: bool,
    pending_audio_filename: Option<&str>,
    pending_audio_duration: Option<&str>,
    retry_cache_ready: bool,
    latest_status: &str,
    cached_error: Option<&str>,
    source_copy: &str,
) -> String {
    let control = control.trim();
    let control_label = if control.is_empty() {
        "Result"
    } else {
        control
    };
    if control_label.eq_ignore_ascii_case("Request") {
        return voice_message_capture_request_packet_snapshot_label(
            control_label,
            panel_visible,
            pending_audio_filename,
            pending_audio_duration,
            retry_cache_ready,
            latest_status,
            cached_error,
            source_copy,
        );
    }
    let panel_state = if panel_visible {
        "voice panel visible"
    } else {
        "voice panel hidden"
    };
    let audio_state = pending_audio_filename
        .filter(|filename| !filename.trim().is_empty())
        .map(|filename| {
            let duration = pending_audio_duration
                .filter(|duration| !duration.trim().is_empty())
                .unwrap_or("duration unavailable before player metadata");
            format!("pending desktop audio review {filename}; {duration}")
        })
        .unwrap_or_else(|| "no pending desktop audio review loaded".to_string());
    let retry_state = if retry_cache_ready {
        "immediate handoff retry cache ready"
    } else {
        "immediate handoff retry cache empty"
    };
    let latest_status = if latest_status.trim().is_empty() {
        "local voice evidence"
    } else {
        latest_status.trim()
    };
    let error_state = cached_error
        .filter(|error| !error.trim().is_empty())
        .map(|error| {
            let char_count = error.chars().count();
            let mut preview = error.chars().take(54).collect::<String>();
            if char_count > 54 {
                preview.push_str("...");
            }
            format!("cached error {char_count} chars: {preview}")
        })
        .unwrap_or_else(|| "cached error empty".to_string());
    let source_char_count = source_copy.trim().chars().count();

    format!(
        "Voice Send preflight {control_label} stayed local: {panel_state}; {audio_state}; {retry_state}; latest status {latest_status}; {error_state}; source copy {source_char_count} chars. Request, Result, Error, Retry, and Source only summarize the confirmed desktop audio picker, pending voice attachment review, cached immediate SendAttachment handoff failure, retry readiness, and source evidence. They do not request microphone permission, start a recorder, create a captured audio file, sample waveform, transcribe, convert codec, subscribe to upload progress, submit extra SendAttachment, run unconfirmed retry, send SendMessage fallback, gateway/runtime/auth, or live mutation. {VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_LABEL}"
    )
}

#[cfg(test)]
mod voice_message_lifecycle_metadata_tests {
    use super::*;

    fn test_pcm_wav_bytes(samples: &[i16], sample_rate: u32) -> Vec<u8> {
        let data_size = samples.len() as u32 * 2;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&(36 + data_size).to_le_bytes());
        bytes.extend_from_slice(b"WAVE");
        bytes.extend_from_slice(b"fmt ");
        bytes.extend_from_slice(&16u32.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&sample_rate.to_le_bytes());
        bytes.extend_from_slice(&(sample_rate * 2).to_le_bytes());
        bytes.extend_from_slice(&2u16.to_le_bytes());
        bytes.extend_from_slice(&16u16.to_le_bytes());
        bytes.extend_from_slice(b"data");
        bytes.extend_from_slice(&data_size.to_le_bytes());
        for sample in samples {
            bytes.extend_from_slice(&sample.to_le_bytes());
        }
        bytes
    }

    #[test]
    fn selected_audio_waveform_codec_label_reads_bounded_wav_pcm_peaks() {
        let path = std::env::temp_dir().join(format!(
            "hepta-selected-audio-waveform-{}.wav",
            current_time_ms()
        ));
        let samples = [
            0, 2048, -4096, 8192, -12_000, 16_000, -20_000, 24_000, -28_000, 32_000,
        ];
        fs::write(&path, test_pcm_wav_bytes(&samples, 8000)).unwrap();
        let mime_type: mime::Mime = "audio/wav".parse().unwrap();

        let label = voice_audio_waveform_codec_label(&path, &mime_type);
        let _ = fs::remove_file(&path);

        assert!(label.contains("codec: PCM"));
        assert!(label.contains("format=1"));
        assert!(label.contains("channels=1"));
        assert!(label.contains("sample_rate=8000Hz"));
        assert!(label.contains("bits=16"));
        assert!(label.contains("duration=0:00 from WAV header"));
        assert!(label.contains("waveform: PCM peak buckets 16x="));
        assert!(label.contains("probe bytes"));
        assert!(!label.contains("waveform: unavailable"));
        assert!(
            VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_EVIDENCE.contains("capped local bytes")
        );
        assert!(
            VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_EVIDENCE
                .contains("coarse PCM peak buckets")
        );
        assert!(
            VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_EVIDENCE
                .contains("submits SendAttachment before review Send")
        );
        assert!(
            VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_EVIDENCE.contains("gateway/runtime/auth")
        );
        assert!(VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn selected_audio_waveform_codec_label_reports_non_wav_boundary() {
        let path = PathBuf::from("clip.mp3");
        let mime_type: mime::Mime = "audio/mpeg".parse().unwrap();

        let label = voice_audio_waveform_codec_label(&path, &mime_type);

        assert!(label.contains("codec: audio/mpeg / ext mp3"));
        assert!(label.contains("waveform: unavailable for non-WAV selected audio"));
        assert!(
            VOICE_MESSAGE_SELECTED_AUDIO_METADATA_EVIDENCE
                .contains("duration status, codec/container status")
        );
        assert!(
            VOICE_MESSAGE_SELECTED_AUDIO_METADATA_EVIDENCE
                .contains("bounded local WAV PCM waveform peaks")
        );
        assert!(VOICE_MESSAGE_SELECTED_AUDIO_METADATA_LABEL.contains("codec"));
    }

    #[test]
    fn voice_message_lifecycle_metadata_label_summarizes_selected_audio() {
        let label = voice_message_lifecycle_metadata_label(
            "audio selected",
            false,
            Some("Selected audio file staged locally"),
            Some("note.wav"),
            Some("duration: 0:03 from WAV header"),
            true,
            "desktop audio picker accepted",
        );

        assert!(label.contains("Voice lifecycle audio selected"));
        assert!(label.contains("panel hidden"));
        assert!(label.contains("Selected audio file staged locally"));
        assert!(label.contains("pending audio note.wav"));
        assert!(label.contains("duration: 0:03 from WAV header"));
        assert!(label.contains("reply context loaded"));
        assert!(label.contains("desktop audio picker accepted"));
        assert!(label.contains(VOICE_MESSAGE_LIFECYCLE_METADATA_LABEL));
        assert!(VOICE_MESSAGE_LIFECYCLE_METADATA_EVIDENCE.contains("Record, Lock, Cancel"));
        assert!(
            VOICE_MESSAGE_LIFECYCLE_METADATA_EVIDENCE.contains("existing attachment review row")
        );
        assert!(VOICE_MESSAGE_LIFECYCLE_METADATA_EVIDENCE.contains("microphone permission"));
        assert!(VOICE_MESSAGE_LIFECYCLE_METADATA_EVIDENCE.contains("hidden SDK send-queue"));
        assert!(VOICE_MESSAGE_LIFECYCLE_METADATA_EVIDENCE.contains("gateway/runtime/auth"));
        assert!(VOICE_MESSAGE_LIFECYCLE_METADATA_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn voice_message_lifecycle_metadata_label_uses_empty_fallbacks() {
        let label = voice_message_lifecycle_metadata_label(
            "closed",
            false,
            Some(""),
            None,
            None,
            false,
            "",
        );

        assert!(label.contains("Voice lifecycle closed"));
        assert!(label.contains("panel hidden"));
        assert!(label.contains("no local voice control staged"));
        assert!(label.contains("no pending audio review loaded"));
        assert!(label.contains("reply context none"));
        assert!(label.contains("no confirmation or picker pending"));
        assert!(label.contains(VOICE_MESSAGE_LIFECYCLE_METADATA_LABEL));
    }

    #[test]
    fn voice_confirmation_cancel_metadata_label_preserves_pending_audio() {
        let label = voice_confirmation_cancel_metadata_label(Some("note.wav"), true);

        assert!(label.contains("Voice confirmation canceled locally"));
        assert!(label.contains("pending audio review preserved: note.wav"));
        assert!(label.contains("reply context preserved"));
        assert!(label.contains(VOICE_MESSAGE_CONFIRMATION_CANCEL_METADATA_LABEL));
        assert!(
            VOICE_MESSAGE_CONFIRMATION_CANCEL_METADATA_EVIDENCE
                .contains("PositiveConfirmationModal cancel")
        );
        assert!(
            VOICE_MESSAGE_CONFIRMATION_CANCEL_METADATA_EVIDENCE
                .contains("pending attachment already exists")
        );
        assert!(VOICE_MESSAGE_CONFIRMATION_CANCEL_METADATA_EVIDENCE.contains("SendAttachment"));
        assert!(
            VOICE_MESSAGE_CONFIRMATION_CANCEL_METADATA_EVIDENCE.contains("gateway/runtime/auth")
        );
        assert!(VOICE_MESSAGE_CONFIRMATION_CANCEL_METADATA_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn voice_confirmation_cancel_metadata_label_reports_empty_state() {
        let label = voice_confirmation_cancel_metadata_label(None, false);

        assert!(label.contains("no pending audio review"));
        assert!(label.contains("picker preview hidden"));
        assert!(label.contains("reply context none"));
        assert!(label.contains(VOICE_MESSAGE_CONFIRMATION_CANCEL_METADATA_LABEL));
    }

    #[test]
    fn voice_message_recorder_waveform_codec_boundary_label_preserves_review_handoff() {
        let label = voice_message_recorder_waveform_codec_boundary_label(
            "record staged",
            true,
            Some("clip.ogg"),
        );

        assert!(label.contains("Voice recorder boundary record staged"));
        assert!(label.contains("recorder panel visible"));
        assert!(label.contains("desktop audio review preserved: clip.ogg"));
        assert!(label.contains(VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_LABEL));
        assert!(
            VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE
                .contains("voice_message_send remains a base gap")
        );
        assert!(
            VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE
                .contains("waveform capture/render")
        );
        assert!(
            VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE
                .contains("opus/ogg/amr conversion")
        );
        assert!(VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE.contains("transcription"));
        assert!(
            VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE.contains("upload progress")
        );
        assert!(
            VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE
                .contains("MatrixRequest::SendAttachment")
        );
        assert!(
            VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE
                .contains("gateway/runtime/auth")
        );
        assert!(VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn voice_message_recorder_waveform_codec_boundary_label_reports_empty_state() {
        let label = voice_message_recorder_waveform_codec_boundary_label("closed", false, None);

        assert!(label.contains("Voice recorder boundary closed"));
        assert!(label.contains("recorder panel hidden"));
        assert!(label.contains("no captured recorder payload"));
        assert!(label.contains(VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_LABEL));
    }

    #[test]
    fn voice_message_recorder_status_controls_label_keeps_controls_local() {
        let label = voice_message_recorder_status_controls_label(
            Some("Waveform"),
            true,
            Some("clip.wav"),
            Some("duration: 0:04 from WAV header"),
            Some(
                "codec: PCM format=1 channels=1 sample_rate=8000Hz bits=16 data=64 bytes duration=0:04 from WAV header; waveform: PCM peak buckets 16x=0,10",
            ),
        );

        assert!(label.contains("Voice recorder status controls Waveform selected"));
        assert!(label.contains("voice panel visible"));
        assert!(label.contains("desktop audio review visible: clip.wav"));
        assert!(label.contains("duration: 0:04 from WAV header"));
        assert!(label.contains("codec: PCM"));
        assert!(label.contains("waveform: PCM peak buckets"));
        assert!(label.contains("Timer, Waveform, Transcript, Progress, and Codec"));
        assert!(label.contains(VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_LABEL));
        assert!(label.contains(VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_LABEL));
        assert!(
            VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE
                .contains("visible local voice recorder status controls")
        );
        assert!(VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE.contains("microphone permission"));
        assert!(
            VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE.contains("audio session activation")
        );
        assert!(VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE.contains("platform recorder"));
        assert!(
            VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE.contains("recorder waveform sampling")
        );
        assert!(
            VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE
                .contains("capped local RIFF/fmt/data parsing")
        );
        assert!(VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE.contains("transcript service"));
        assert!(VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE.contains("codec conversion"));
        assert!(
            VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE
                .contains("upload progress subscription")
        );
        assert!(VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE.contains("SendAttachment"));
        assert!(VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE.contains("gateway/runtime/auth"));
        assert!(VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn voice_message_recorder_status_controls_label_reports_empty_state() {
        let label = voice_message_recorder_status_controls_label(None, false, None, None, None);

        assert!(label.contains("no recorder status control selected"));
        assert!(label.contains("voice panel hidden"));
        assert!(label.contains("no captured recorder payload or pending audio upload"));
        assert!(label.contains(VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_LABEL));
    }

    #[test]
    fn voice_message_capture_lifecycle_controls_label_keeps_capture_chain_local() {
        let label = voice_message_capture_lifecycle_controls_label(
            Some("Capture"),
            true,
            Some("clip.wav"),
            Some("duration: 0:04 from WAV header"),
        );

        assert!(label.contains("Voice capture lifecycle controls Capture selected"));
        assert!(label.contains("voice panel visible"));
        assert!(label.contains("desktop audio review preserved: clip.wav"));
        assert!(label.contains("duration: 0:04 from WAV header"));
        assert!(label.contains(
            "Permission, Capture, Encode, Review, Upload, Packet, Contract, and Taxonomy"
        ));
        assert!(label.contains("microphone permission"));
        assert!(label.contains("privacy entitlement"));
        assert!(label.contains("audio session activation"));
        assert!(label.contains("platform recorder"));
        assert!(label.contains("captured local audio file"));
        assert!(label.contains("temporary recording write"));
        assert!(label.contains("waveform sampling"));
        assert!(label.contains("duration capture"));
        assert!(label.contains("codec conversion"));
        assert!(label.contains("SendAttachment"));
        assert!(label.contains("SendMessage fallback"));
        assert!(label.contains("SDK send-queue work"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_LABEL));
        assert!(
            VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE
                .contains("visible local voice capture lifecycle controls")
        );
        assert!(
            VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE
                .contains("Permission requests no microphone permission")
        );
        assert!(
            VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE
                .contains("Capture starts no platform recorder")
        );
        assert!(
            VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE
                .contains("Encode performs no codec conversion")
        );
        assert!(
            VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE
                .contains("Upload submits no SendAttachment")
        );
        assert!(
            VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE
                .contains("confirmed desktop audio-file review SendAttachment")
        );
    }

    #[test]
    fn voice_message_capture_lifecycle_controls_label_reports_empty_state() {
        let label = voice_message_capture_lifecycle_controls_label(None, false, None, None);

        assert!(label.contains("no capture lifecycle control selected"));
        assert!(label.contains("voice panel hidden"));
        assert!(label.contains("no captured recorder payload or pending upload"));
        assert!(label.contains(VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_LABEL));
    }

    #[test]
    fn voice_message_capture_request_packet_snapshot_label_summarizes_local_packet() {
        let label = voice_message_capture_request_packet_snapshot_label(
            "Upload",
            true,
            Some("voice.m4a"),
            Some("duration: 0:07 from WAV header"),
            true,
            "Upload capture lifecycle stayed local",
            Some("upload worker unavailable before recorder contract"),
            VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE,
        );

        assert!(label.contains("Local voice capture/request packet snapshot"));
        assert!(label.contains("Upload selected"));
        assert!(label.contains("voice panel visible"));
        assert!(label.contains("pending desktop audio review voice.m4a"));
        assert!(label.contains("duration: 0:07 from WAV header"));
        assert!(label.contains("voice retry cache ready"));
        assert!(label.contains("Upload capture lifecycle stayed local"));
        assert!(label.contains("cached error"));
        assert!(label.contains("source copy"));
        assert!(label.contains("microphone permission request"));
        assert!(label.contains("capture session slot"));
        assert!(label.contains("upload request body"));
        assert!(label.contains("voice-message contract target"));
        assert!(label.contains("MatrixRequest::SendAttachment"));
        assert!(label.contains("SendMessage fallback"));
        assert!(label.contains("SDK send-queue work"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_LABEL));
        assert!(label.contains(VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_LABEL));
        assert!(
            VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE
                .contains("local voice capture/request packet snapshot")
        );
        assert!(
            VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE
                .contains("Contract maps that drilldown to typed microphone permission")
        );
        assert!(
            VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE
                .contains("Taxonomy records permission/capture/encode/review/upload result slots")
        );
        assert!(
            VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("local voice capture/request packet snapshot")
        );

        let permission =
            voice_message_capture_lifecycle_controls_label(Some("Permission"), true, None, None);
        assert!(permission.contains("Local voice capture/request packet snapshot"));
        assert!(permission.contains("Permission selected"));
    }

    #[test]
    fn voice_message_recorder_lifecycle_drilldown_packet_label_persists_acceptance_matrix() {
        let label = voice_message_recorder_lifecycle_drilldown_packet_label(
            true,
            Some("voice.m4a"),
            Some("duration: 0:07 from WAV header"),
            true,
            "Packet capture lifecycle stayed local",
            Some("recorder contract missing before upload queue"),
            VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_EVIDENCE,
        );

        assert!(label.contains("Voice recorder lifecycle drilldown packet"));
        assert!(label.contains("voice panel visible"));
        assert!(label.contains("pending desktop audio review voice.m4a"));
        assert!(label.contains("duration: 0:07 from WAV header"));
        assert!(label.contains("voice retry cache ready"));
        assert!(label.contains("Packet capture lifecycle stayed local"));
        assert!(label.contains("cached error"));
        assert!(label.contains("source copy"));
        assert!(label.contains("Acceptance matrix"));
        assert!(label.contains("microphone permission"));
        assert!(label.contains("privacy entitlement"));
        assert!(label.contains("audio session activation"));
        assert!(label.contains("recorder start/lock/cancel"));
        assert!(label.contains("temporary capture file lifecycle"));
        assert!(label.contains("waveform sampling/rendering"));
        assert!(label.contains("timer/duration capture"));
        assert!(label.contains("codec/encoding/transcription"));
        assert!(label.contains("review playback/drop cleanup"));
        assert!(label.contains("mobile picker/share sheet"));
        assert!(label.contains("upload queue"));
        assert!(label.contains("MatrixRequest::SendAttachment"));
        assert!(label.contains("SendMessage fallback"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_LABEL));
        assert!(label.contains(VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_LABEL));
        assert!(label.contains(VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_LABEL));
        assert!(
            VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_EVIDENCE
                .contains("visible Packet control")
        );
        assert!(
            VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_EVIDENCE
                .contains("microphone permission, privacy entitlement, audio session")
        );
        assert!(
            VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_EVIDENCE
                .contains("confirmed desktop audio review SendAttachment acceptance criteria")
        );
        assert!(
            VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_EVIDENCE
                .contains("gateway/runtime/auth")
        );

        let packet =
            voice_message_capture_lifecycle_controls_label(Some("Packet"), true, None, None);
        assert!(packet.contains("Voice recorder lifecycle drilldown packet"));
        assert!(packet.contains("no captured recorder payload or pending audio review"));
    }

    #[test]
    fn voice_message_recorder_lifecycle_drilldown_packet_label_reports_empty_state() {
        let label = voice_message_recorder_lifecycle_drilldown_packet_label(
            false, None, None, false, "", None, "",
        );

        assert!(label.contains("voice panel hidden"));
        assert!(label.contains("no captured recorder payload or pending audio review"));
        assert!(label.contains("voice retry cache empty"));
        assert!(label.contains("local recorder lifecycle evidence"));
        assert!(label.contains("cached error empty"));
        assert!(label.contains("source copy 0 chars"));
        assert!(label.contains(VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_LABEL));
    }

    #[test]
    fn voice_message_recorder_typed_contract_packet_label_maps_drilldown_to_contracts() {
        let label = voice_message_recorder_typed_contract_packet_label(
            true,
            Some("voice.m4a"),
            Some("duration: 0:07 from WAV header"),
            true,
            "Contract capture lifecycle stayed local",
            Some("recorder session missing before captured upload"),
            VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE,
        );

        assert!(label.contains("Voice recorder typed contract packet"));
        assert!(label.contains("voice panel visible"));
        assert!(label.contains("pending desktop audio review voice.m4a"));
        assert!(label.contains("duration: 0:07 from WAV header"));
        assert!(label.contains("voice retry cache ready"));
        assert!(label.contains("Contract capture lifecycle stayed local"));
        assert!(label.contains("cached error"));
        assert!(label.contains("source copy"));
        assert!(label.contains("Typed contracts cover"));
        assert!(label.contains("microphone permission request/result/error"));
        assert!(label.contains("privacy entitlement"));
        assert!(label.contains("audio session lifecycle"));
        assert!(label.contains("recorder session start/lock/cancel"));
        assert!(label.contains("capture file identity and cleanup"));
        assert!(label.contains("waveform sampling/rendering"));
        assert!(label.contains("codec/encoding/transcription result taxonomy"));
        assert!(label.contains("mobile picker/share sheet handoff"));
        assert!(label.contains("upload queue progress/result/error/retry/source"));
        assert!(label.contains("SendAttachment result mapping"));
        assert!(label.contains("stale capture handling"));
        assert!(label.contains("idempotency"));
        assert!(label.contains("adapter promotion blockers"));
        assert!(label.contains("extra MatrixRequest::SendAttachment"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_LABEL));
        assert!(label.contains(VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_LABEL));
        assert!(label.contains(VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_LABEL));
        assert!(
            VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE
                .contains("visible Contract control")
        );
        assert!(
            VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE.contains(
                "microphone permission and privacy entitlement request/result/error slots"
            )
        );
        assert!(
            VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE
                .contains("adapter promotion blockers")
        );

        let contract =
            voice_message_capture_lifecycle_controls_label(Some("Contract"), true, None, None);
        assert!(contract.contains("Voice recorder typed contract packet"));
        assert!(contract.contains("no captured recorder payload or pending audio review"));
    }

    #[test]
    fn voice_message_recorder_typed_contract_packet_label_reports_empty_state() {
        let label = voice_message_recorder_typed_contract_packet_label(
            false, None, None, false, "", None, "",
        );

        assert!(label.contains("voice panel hidden"));
        assert!(label.contains("no captured recorder payload or pending audio review"));
        assert!(label.contains("voice retry cache empty"));
        assert!(label.contains("local recorder typed contract evidence"));
        assert!(label.contains("cached error empty"));
        assert!(label.contains("source copy 0 chars"));
        assert!(label.contains(VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_LABEL));
    }

    #[test]
    fn voice_message_recorder_result_taxonomy_packet_label_names_blocked_result_slots() {
        let label = voice_message_recorder_result_taxonomy_packet_label(
            true,
            Some("voice.m4a"),
            Some("duration: 0:07 from WAV header"),
            true,
            "Taxonomy capture lifecycle stayed local",
            Some("microphone denied before recorder contract"),
            VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE,
        );

        assert!(label.contains("Voice recorder result taxonomy packet"));
        assert!(label.contains("voice panel visible"));
        assert!(label.contains("pending desktop audio review voice.m4a"));
        assert!(label.contains("duration: 0:07 from WAV header"));
        assert!(label.contains("voice retry cache ready"));
        assert!(label.contains("Taxonomy capture lifecycle stayed local"));
        assert!(label.contains("cached error"));
        assert!(label.contains("Live references remain confirmed desktop audio review"));
        assert!(label.contains("MatrixRequest::SendAttachment"));
        assert!(label.contains("Timeline::send_attachment().use_send_queue()"));
        assert!(label.contains("confirmed failed-handoff Retry"));
        assert!(label.contains("selected-audio bounded WAV metadata"));
        assert!(label.contains("review Play local system-opener handoff"));
        assert!(label.contains("Drop pending-audio local cleanup"));
        assert!(label.contains("microphone_permission_operation_id not_assigned"));
        assert!(label.contains("privacy_entitlement_result not_wired"));
        assert!(label.contains("audio_session_id not_assigned"));
        assert!(label.contains("recorder_session_id not_assigned"));
        assert!(label.contains("capture_file_identity not_assigned"));
        assert!(label.contains("waveform_timer_result not_wired"));
        assert!(label.contains("codec_transcription_result not_wired"));
        assert!(label.contains("review_player_result not_wired"));
        assert!(label.contains("mobile_picker_share_result not_wired"));
        assert!(label.contains("captured_upload_queue_item_id not_assigned"));
        assert!(label.contains("delivery_result not_wired"));
        assert!(label.contains("stale_capture_result not_wired"));
        assert!(label.contains("retry_cancel_result not_wired"));
        assert!(label.contains("audit_redaction raw_path_microphone_buffer_transcript_redacted"));
        assert!(label.contains("extra MatrixRequest::SendAttachment"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_LABEL));
        assert!(label.contains(VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_LABEL));
        assert!(label.contains(VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_LABEL));
        assert!(
            VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("visible Taxonomy control")
        );
        assert!(
            VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("confirmed desktop audio review MatrixRequest::SendAttachment")
        );
        assert!(
            VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("microphone permission operation id")
        );
        assert!(VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE.contains("audit redaction"));

        let taxonomy =
            voice_message_capture_lifecycle_controls_label(Some("Taxonomy"), true, None, None);
        assert!(taxonomy.contains("Voice recorder result taxonomy packet"));
        assert!(taxonomy.contains("no captured recorder payload or pending audio review"));
    }

    #[test]
    fn voice_message_recorder_result_taxonomy_packet_label_reports_empty_state() {
        let label = voice_message_recorder_result_taxonomy_packet_label(
            false, None, None, false, "", None, "",
        );

        assert!(label.contains("voice panel hidden"));
        assert!(label.contains("no captured recorder payload or pending audio review"));
        assert!(label.contains("voice retry cache empty"));
        assert!(label.contains("local recorder result taxonomy evidence"));
        assert!(label.contains("cached error empty"));
        assert!(label.contains("source copy 0 chars"));
        assert!(label.contains(VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_LABEL));
    }

    #[test]
    fn voice_message_mobile_picker_controls_label_keeps_mobile_paths_local() {
        let label = voice_message_mobile_picker_controls_label(
            Some("Library"),
            true,
            Some("voice.m4a"),
            Some("duration: unavailable"),
            "Voice mobile picker open",
        );

        assert!(label.contains("Voice mobile picker controls Library selected"));
        assert!(label.contains("voice panel visible"));
        assert!(label.contains("desktop audio review preserved: voice.m4a"));
        assert!(label.contains("latest status Voice mobile picker open"));
        assert!(label.contains("Mic, Files, Library, Retake, and Share"));
        assert!(label.contains("mobile microphone permission"));
        assert!(label.contains("mobile document picker"));
        assert!(label.contains("photo/audio library picker"));
        assert!(label.contains("capture session"));
        assert!(label.contains("retake deletion"));
        assert!(label.contains("system share sheet"));
        assert!(label.contains("SendAttachment"));
        assert!(label.contains("SDK send-queue work"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(VOICE_MESSAGE_MOBILE_PICKER_CONTROLS_LABEL));
        assert!(
            VOICE_MESSAGE_MOBILE_PICKER_CONTROLS_EVIDENCE
                .contains("visible local voice mobile picker controls")
        );
        assert!(
            VOICE_MESSAGE_MOBILE_PICKER_CONTROLS_EVIDENCE
                .contains("Files opens no mobile document picker")
        );
        assert!(
            VOICE_MESSAGE_MOBILE_PICKER_CONTROLS_EVIDENCE
                .contains("Share opens no system share sheet")
        );
    }

    #[test]
    fn voice_message_mobile_picker_controls_label_reports_empty_state() {
        let label = voice_message_mobile_picker_controls_label(None, false, None, None, "");

        assert!(label.contains("no mobile picker control selected"));
        assert!(label.contains("voice panel hidden"));
        assert!(label.contains("no mobile voice picker payload or pending upload"));
        assert!(label.contains("local mobile picker evidence"));
        assert!(label.contains(VOICE_MESSAGE_MOBILE_PICKER_CONTROLS_LABEL));
    }

    #[test]
    fn voice_message_review_playback_controls_label_keeps_review_local() {
        let label = voice_message_review_playback_controls_label(
            Some("Scrub"),
            true,
            Some("clip.wav"),
            Some("duration: 0:04 from WAV header"),
            "Voice review open",
        );

        assert!(label.contains("Voice review playback controls Scrub selected"));
        assert!(label.contains("voice panel visible"));
        assert!(label.contains("desktop audio review available: clip.wav"));
        assert!(label.contains("duration: 0:04 from WAV header"));
        assert!(label.contains("latest status Voice review open"));
        assert!(
            label.contains("Play opens the pending desktop audio review with the system opener")
        );
        assert!(label.contains("Pause, Scrub, and Speed update local review metadata"));
        assert!(label.contains("Drop performs the real pending-audio cleanup handoff"));
        assert!(label.contains("inline audio player"));
        assert!(label.contains("media decode"));
        assert!(label.contains("playback position subscription"));
        assert!(label.contains("speed transform"));
        assert!(label.contains("local file deletion"));
        assert!(label.contains("SendAttachment"));
        assert!(label.contains("SendMessage fallback"));
        assert!(label.contains("SDK send-queue work"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_LABEL));
        assert!(
            VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_EVIDENCE
                .contains("visible voice review playback controls")
        );
        assert!(VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_EVIDENCE.contains("system opener"));
        assert!(
            VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_EVIDENCE.contains("readable regular local file")
        );
        assert!(
            VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_EVIDENCE
                .contains("start no inline audio player")
        );
        assert!(VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_EVIDENCE.contains("local file deletion"));
        assert!(
            VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_EVIDENCE
                .contains("Drop is a real local cleanup handoff")
        );
    }

    #[test]
    fn voice_message_review_playback_open_result_label_names_system_opener() {
        let label = voice_message_review_playback_open_result_label(
            "clip.wav",
            Some("duration: 0:04 from WAV header"),
            "opened with system opener",
        );

        assert!(label.contains("Voice review Play local opener"));
        assert!(label.contains("clip.wav"));
        assert!(label.contains("duration: 0:04 from WAV header"));
        assert!(label.contains("opened with system opener"));
        assert!(label.contains("pending desktop Voice attachment local file path"));
        assert!(label.contains("system opener"));
        assert!(label.contains("submits no SendAttachment"));
        assert!(label.contains("inline player"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_LABEL));
    }

    #[test]
    fn voice_message_review_playback_controls_label_reports_empty_state() {
        let label = voice_message_review_playback_controls_label(None, false, None, None, "");

        assert!(label.contains("no review playback control selected"));
        assert!(label.contains("voice panel hidden"));
        assert!(label.contains("no pending voice review audio loaded"));
        assert!(label.contains("local voice review evidence"));
        assert!(label.contains(VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_LABEL));
    }

    #[test]
    fn voice_message_review_drop_pending_audio_label_summarizes_local_cleanup() {
        let label = voice_message_review_drop_pending_audio_label(
            Some("clip.wav"),
            Some("duration: 0:04 from WAV header"),
            true,
        );

        assert!(label.contains("Voice review Drop"));
        assert!(label.contains("dropped pending desktop audio review clip.wav"));
        assert!(label.contains("duration: 0:04 from WAV header"));
        assert!(label.contains("voice failed-handoff retry cache cleared"));
        assert!(label.contains("deletes no local file"));
        assert!(label.contains("submits no SendAttachment"));
        assert!(label.contains("SDK queue cancel"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(VOICE_MESSAGE_REVIEW_DROP_PENDING_AUDIO_LABEL));
        assert!(VOICE_MESSAGE_REVIEW_DROP_PENDING_AUDIO_EVIDENCE.contains("Option::take()"));
        assert!(
            VOICE_MESSAGE_REVIEW_DROP_PENDING_AUDIO_EVIDENCE
                .contains("does not discard Photo/File pending attachments")
        );
    }

    #[test]
    fn voice_message_review_drop_pending_audio_label_reports_empty_state() {
        let label = voice_message_review_drop_pending_audio_label(None, None, false);

        assert!(label.contains("no pending voice review audio loaded"));
        assert!(label.contains("no voice failed-handoff retry cache loaded"));
        assert!(label.contains(VOICE_MESSAGE_REVIEW_DROP_PENDING_AUDIO_LABEL));
    }

    #[test]
    fn voice_message_send_preflight_detail_controls_label_summarizes_review_state() {
        let label = voice_message_send_preflight_detail_controls_label(
            "Error",
            true,
            Some("clip.wav"),
            Some("duration: 0:04 from WAV header"),
            true,
            "Voice attachment handoff failed before SDK queue",
            Some("network unavailable before SDK queue ownership"),
            VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE,
        );

        assert!(label.contains("Voice Send preflight Error stayed local"));
        assert!(label.contains("voice panel visible"));
        assert!(label.contains("pending desktop audio review clip.wav"));
        assert!(label.contains("duration: 0:04 from WAV header"));
        assert!(label.contains("immediate handoff retry cache ready"));
        assert!(label.contains("cached error"));
        assert!(label.contains("source copy"));
        assert!(label.contains(VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_LABEL));
        assert!(
            VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("Request, Result, Error, Retry, and Source")
        );
        assert!(
            VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("pending desktop audio review")
        );
        assert!(
            VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("extra MatrixRequest::SendAttachment")
        );
        assert!(
            VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("unconfirmed retry")
        );
        assert!(
            VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("gateway/runtime/auth")
        );
        assert!(VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn voice_message_send_preflight_request_renders_local_packet_snapshot() {
        let label = voice_message_send_preflight_detail_controls_label(
            "Request",
            true,
            Some("clip.wav"),
            Some("duration: 0:04 from WAV header"),
            false,
            "Voice Send preflight Request detail stayed local",
            None,
            VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE,
        );

        assert!(label.contains("Local voice capture/request packet snapshot"));
        assert!(label.contains("Request selected"));
        assert!(label.contains("pending desktop audio review clip.wav"));
        assert!(label.contains("voice retry cache empty"));
        assert!(label.contains("Voice Send preflight Request detail stayed local"));
        assert!(label.contains("upload request body"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn voice_message_send_preflight_detail_controls_label_reports_empty_state() {
        let label = voice_message_send_preflight_detail_controls_label(
            "", false, None, None, false, "", None, "",
        );

        assert!(label.contains("Voice Send preflight Result stayed local"));
        assert!(label.contains("voice panel hidden"));
        assert!(label.contains("no pending desktop audio review loaded"));
        assert!(label.contains("immediate handoff retry cache empty"));
        assert!(label.contains("local voice evidence"));
        assert!(label.contains("cached error empty"));
        assert!(label.contains(VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_LABEL));
    }

    #[test]
    fn voice_message_send_preflight_control_from_status_maps_status() {
        assert_eq!(
            voice_message_send_preflight_control_from_status("voice picker confirmation opened"),
            "Request"
        );
        assert_eq!(
            voice_message_send_preflight_control_from_status("worker failure-copy"),
            "Error"
        );
        assert_eq!(
            voice_message_send_preflight_control_from_status("retry-confirmed"),
            "Retry"
        );
        assert_eq!(
            voice_message_send_preflight_control_from_status("source metadata"),
            "Source"
        );
        assert_eq!(
            voice_message_send_preflight_control_from_status("queued-only"),
            "Result"
        );
    }
}

#[derive(Clone, Debug)]
struct RoomInputBarReplyTarget {
    event_id: OwnedEventId,
    enforce_thread: RoomInputBarReplyThread,
    add_mentions: bool,
}

#[derive(Clone, Copy, Debug)]
enum RoomInputBarReplyThread {
    ThreadedYes,
    ThreadedNo,
    MaybeThreaded,
}

impl RoomInputBarReplyTarget {
    fn into_reply(self) -> Reply {
        Reply {
            event_id: self.event_id,
            enforce_thread: match self.enforce_thread {
                RoomInputBarReplyThread::ThreadedYes => {
                    EnforceThread::Threaded(ReplyWithinThread::Yes)
                }
                RoomInputBarReplyThread::ThreadedNo => {
                    EnforceThread::Threaded(ReplyWithinThread::No)
                }
                RoomInputBarReplyThread::MaybeThreaded => EnforceThread::MaybeThreaded,
            },
            add_mentions: if self.add_mentions {
                AddMentions::Yes
            } else {
                AddMentions::No
            },
        }
    }
}

impl ScriptHook for RoomInputBar {
    fn on_after_new(&mut self, vm: &mut ScriptVm) {
        vm.with_cx_mut(|cx| {
            let send_on_enter = cx.global::<AppPreferencesGlobal>().0.send_on_enter;
            self.mentionable_text_input(cx, ids!(mentionable_text_input))
                .text_input_ref()
                .set_submit_on_enter(send_on_enter);
        });
    }
}

impl Widget for RoomInputBar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let room_screen_props = scope
            .props
            .get::<RoomScreenProps>()
            .expect("BUG: RoomScreenProps should be available in Scope::props for RoomInputBar");

        match event.hits(
            cx,
            self.view
                .view(cx, ids!(replying_preview.reply_preview_content))
                .area(),
        ) {
            // If the hit occurred on the replying message preview, jump to it.
            Hit::FingerUp(fe) if fe.is_over && fe.is_primary_hit() && fe.was_tap() => {
                if let Some(event_id) = self
                    .replying_to
                    .as_ref()
                    .and_then(|(event_tl_item, _)| event_tl_item.event_id().map(ToOwned::to_owned))
                {
                    cx.widget_action(
                        room_screen_props.room_screen_widget_uid,
                        MessageAction::JumpToEvent(event_id),
                    );
                } else {
                    enqueue_popup_notification(
                        "BUG: couldn't find the message you're replying to.",
                        PopupKind::Error,
                        None,
                    );
                }
            }
            _ => {}
        }

        if let Event::Actions(actions) = event {
            // Handle changes to the `send_on_enter` preference.
            for action in actions {
                if let Some(AppPreferencesAction::SendOnEnterChanged(v)) = action.downcast_ref() {
                    self.mentionable_text_input(cx, ids!(mentionable_text_input))
                        .text_input_ref()
                        .set_submit_on_enter(*v);
                }
            }

            self.handle_actions(cx, actions, room_screen_props);
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Shrink the input_bar's height as the editing pane slides in,
        // and grow it back as the editing pane slides out.
        // slide=1.0 → editing pane hidden → input_bar at full Fit height.
        // slide=0.0 → editing pane shown → input_bar at zero height.
        let slide = self.editing_pane(cx, ids!(editing_pane)).slide();
        let input_bar = self.view.view(cx, ids!(input_bar));

        // Remap slide through a steeper curve so the input_bar reaches
        // its full target height before the ExpDecay tail.
        let remapped = (slide as f64 * 1.25).min(1.0);
        if remapped >= 1.0 {
            // Input_bar has reached its full natural height: switch to Fit
            // so it can respond to content changes normally.
            // Update the cached height for future animations.
            let h = input_bar.area().rect(cx).size.y;
            if h > 0.0 {
                self.input_bar_natural_height = h;
            }
            if let Some(mut inner) = input_bar.borrow_mut() {
                inner.walk.height = Size::fit();
            }
        } else {
            let target = self.input_bar_natural_height;
            if let Some(mut inner) = input_bar.borrow_mut() {
                inner.walk.height = Size::Fixed((target * remapped).max(0.0));
            }
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl RoomInputBar {
    fn handle_actions(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
        room_screen_props: &RoomScreenProps,
    ) {
        let mentionable_text_input = self.mentionable_text_input(cx, ids!(mentionable_text_input));
        let text_input = mentionable_text_input.text_input_ref();
        mentionable_text_input.update_cached_member_suggestions(
            cx,
            room_screen_props
                .room_members
                .as_ref()
                .map(|members| members.as_slice()),
        );

        for action in actions {
            if let Some(RoomInputBarAction::AttachmentHandoffConfirmed {
                kind,
                timeline_kind,
                in_reply_to,
            }) = action.downcast_ref()
            {
                let kind = *kind;
                let label = kind.label();
                match pick_telegram_attachment_file(kind) {
                    AttachmentFilePickResult::Picked(file_path) => {
                        self.telegram_attachment_send_retry_attempt = None;
                        let replaced_attachment = self
                            .telegram_pending_attachment_send
                            .as_ref()
                            .map(|pending| {
                                (pending.filename.clone(), pending.validation_error.clone())
                            });
                        let filename = display_attachment_filename(&file_path);
                        let file_extension = display_attachment_extension(&file_path);
                        let file_size_bytes = telegram_attachment_file_size(&file_path);
                        let mime_type = telegram_attachment_mime_type(&file_path);
                        let mime_label = mime_type.to_string();
                        let image_dimensions_label = (kind == AttachmentHandoffKind::Photo)
                            .then(|| selected_image_dimensions_label(&file_path, &mime_type));
                        let audio_duration_label = (kind == AttachmentHandoffKind::Voice)
                            .then(|| voice_audio_duration_label(&file_path, &mime_type));
                        let audio_waveform_codec_label = (kind == AttachmentHandoffKind::Voice)
                            .then(|| voice_audio_waveform_codec_label(&file_path, &mime_type));
                        let caption_preview =
                            summarize_attachment_caption(&mentionable_text_input.text());
                        self.telegram_pending_attachment_send = Some(PendingAttachmentSend {
                            kind,
                            timeline_kind: timeline_kind.clone(),
                            file_path,
                            mime_type: mime_type.clone(),
                            filename: filename.clone(),
                            file_extension,
                            file_size_bytes,
                            image_dimensions_label: image_dimensions_label.clone(),
                            audio_duration_label: audio_duration_label.clone(),
                            audio_waveform_codec_label: audio_waveform_codec_label.clone(),
                            caption_preview: caption_preview.clone(),
                            in_reply_to: in_reply_to.clone(),
                            validation_error: None,
                        });
                        let image_note = image_dimensions_label
                            .as_deref()
                            .map(|label| format!("; {label}"))
                            .unwrap_or_default();
                        let audio_note = audio_duration_label
                            .as_deref()
                            .map(|label| format!("; {label}"))
                            .unwrap_or_default();
                        let audio_waveform_note = audio_waveform_codec_label
                            .as_deref()
                            .map(|label| format!("; {label}"))
                            .unwrap_or_default();
                        let metadata_note =
                            format!("{image_note}{audio_note}{audio_waveform_note}");
                        let lifecycle_metadata = attachment_review_lifecycle_metadata_label(
                            "selected",
                            label,
                            Some(&filename),
                            Some(&mime_label),
                            file_size_bytes,
                            Some(&caption_preview),
                            in_reply_to.is_some(),
                            None,
                            replaced_attachment
                                .as_ref()
                                .map(|(previous_filename, _)| previous_filename.as_str()),
                        );
                        self.telegram_attachment_local_status = if let Some((
                            previous_filename,
                            previous_validation_error,
                        )) = &replaced_attachment
                        {
                            let recovery_note = previous_validation_error
                                .as_ref()
                                .map(|reason| {
                                    format!("; cleared previous local validation warning: {reason}")
                                })
                                .unwrap_or_default();
                            format!(
                                "{label} selected for review: {filename} ({mime_label}{metadata_note}); replaced previous pending attachment locally: {previous_filename}{recovery_note}"
                            )
                        } else {
                            format!(
                                "{label} selected for review: {filename} ({mime_label}{metadata_note})"
                            )
                        };
                        if kind == AttachmentHandoffKind::Voice {
                            self.telegram_voice_local_status = format!(
                                "Selected audio file staged locally: {filename}; {}; {}",
                                audio_duration_label.as_deref().unwrap_or(
                                    "duration: unavailable before recorder/player metadata"
                                ),
                                audio_waveform_codec_label.as_deref().unwrap_or(
                                    "codec/waveform: unavailable before selected audio analysis"
                                )
                            );
                            self.update_telegram_voice_message_panel(cx);
                            self.set_telegram_voice_message_panel_visible(cx, false);
                        }
                        let voice_lifecycle_metadata =
                            (kind == AttachmentHandoffKind::Voice).then(|| {
                                self.current_voice_lifecycle_metadata_label(
                                    "audio file selected",
                                    "desktop audio picker accepted; pending review loaded",
                                )
                            });
                        let voice_lifecycle_note = voice_lifecycle_metadata
                            .as_deref()
                            .map(|metadata| format!(" {metadata}"))
                            .unwrap_or_default();
                        self.update_telegram_attachment_picker(cx);
                        self.set_telegram_attachment_picker_visible(cx, true);
                        if replaced_attachment.is_some() {
                            self.set_message_send_operation_status(
                                cx,
                                "review-replaced",
                                "Attachment selection replaced locally",
                                &format!(
                                    "A newly selected desktop attachment replaced only the local pending review state and clears any previous local validation warning. The previous selected file was not uploaded, sent, canceled through SDK send queue, or mutated on Matrix; composer caption/reply context stays local and review-row Send is still the only SendAttachment submit path. {lifecycle_metadata}{voice_lifecycle_note}"
                                ),
                            );
                        } else {
                            self.set_message_send_operation_status(
                                cx,
                                "review-pending",
                                "Attachment selected locally",
                                &format!(
                                    "Selected desktop attachment is staged in local review state. Caption preview live-updates from composer text and reply context remains local. No MatrixRequest::SendAttachment, upload, or media send is submitted until Send is clicked; review-row Send is the only attachment path that consumes caption/reply context. Discard and Close clear only the pending attachment locally. {lifecycle_metadata}{voice_lifecycle_note}"
                                ),
                            );
                        }
                        let replacement_note = replaced_attachment
                            .as_ref()
                            .map(|(previous_filename, previous_validation_error)| {
                                let recovery_note = previous_validation_error
                                    .as_ref()
                                    .map(|reason| {
                                        format!(" Cleared previous local validation warning: {reason}.")
                                    })
                                    .unwrap_or_default();
                                format!(" Replaced previous pending attachment locally: {previous_filename}.{recovery_note}")
                            })
                            .unwrap_or_default();
                        enqueue_popup_notification(
                            format!(
                                "{label} attachment selected for local review: {filename} ({mime_label}{metadata_note}).{replacement_note} {lifecycle_metadata}{voice_lifecycle_note} Click Send to submit or Discard to clear it."
                            ),
                            PopupKind::Info,
                            Some(4.0),
                        );
                    }
                    AttachmentFilePickResult::Canceled => {
                        let preserved_metadata = self
                            .telegram_pending_attachment_send
                            .as_ref()
                            .map(|pending| {
                                let mime_label = pending.mime_type.to_string();
                                attachment_review_lifecycle_metadata_label(
                                    "picker cancel preserved",
                                    pending.kind.label(),
                                    Some(&pending.filename),
                                    Some(&mime_label),
                                    pending.file_size_bytes,
                                    Some(&pending.caption_preview),
                                    pending.in_reply_to.is_some(),
                                    pending.validation_error.as_deref(),
                                    None,
                                )
                            })
                            .unwrap_or_else(|| {
                                attachment_review_lifecycle_metadata_label(
                                    "picker canceled",
                                    label,
                                    None,
                                    None,
                                    None,
                                    None,
                                    false,
                                    None,
                                    None,
                                )
                            });
                        let preserved_attachment = self
                            .telegram_pending_attachment_send
                            .as_ref()
                            .map(|pending| pending.filename.clone());
                        self.telegram_attachment_local_status = if let Some(filename) =
                            &preserved_attachment
                        {
                            format!(
                                "{label} attachment picker canceled locally; still reviewing existing pending attachment: {filename}"
                            )
                        } else {
                            format!("{label} attachment picker canceled locally")
                        };
                        if kind == AttachmentHandoffKind::Voice {
                            self.telegram_voice_local_status =
                                "Voice audio picker canceled locally".to_string();
                            self.update_telegram_voice_message_panel(cx);
                        }
                        let voice_lifecycle_note = (kind == AttachmentHandoffKind::Voice)
                            .then(|| {
                                format!(
                                    " {}",
                                    self.current_voice_lifecycle_metadata_label(
                                        "picker canceled",
                                        "desktop audio picker canceled"
                                    )
                                )
                            })
                            .unwrap_or_default();
                        self.update_telegram_attachment_picker(cx);
                        self.set_telegram_attachment_picker_visible(cx, true);
                        if preserved_attachment.is_some() {
                            self.set_message_send_operation_status(
                                cx,
                                "review-preserved",
                                "Picker cancel preserved pending attachment",
                                &format!(
                                    "Canceling a new desktop picker leaves the existing pending attachment review, composer caption, and reply preview intact. No pending attachment was cleared, uploaded, sent, canceled through SDK send queue, or mutated on Matrix. {preserved_metadata}{voice_lifecycle_note}"
                                ),
                            );
                        }
                        enqueue_popup_notification(
                            format!(
                                "{label} attachment picker canceled. Existing pending review is preserved if one was already selected; no upload or Matrix media send was started. {preserved_metadata}{voice_lifecycle_note}"
                            ),
                            PopupKind::Info,
                            Some(3.0),
                        );
                    }
                    AttachmentFilePickResult::Unsupported => {
                        self.telegram_attachment_local_status =
                            format!("{label} attachment picker is not available on this platform");
                        if kind == AttachmentHandoffKind::Voice {
                            self.telegram_voice_local_status =
                                "Voice audio picker unsupported on this platform".to_string();
                            self.update_telegram_voice_message_panel(cx);
                        }
                        let voice_lifecycle_note = (kind == AttachmentHandoffKind::Voice)
                            .then(|| {
                                format!(
                                    " {}",
                                    self.current_voice_lifecycle_metadata_label(
                                        "picker unsupported",
                                        "desktop audio picker unavailable"
                                    )
                                )
                            })
                            .unwrap_or_default();
                        self.update_telegram_attachment_picker(cx);
                        self.set_telegram_attachment_picker_visible(cx, true);
                        enqueue_popup_notification(
                            format!(
                                "{label} attachment picker is not available on this platform yet. No upload or Matrix media send was started.{voice_lifecycle_note}"
                            ),
                            PopupKind::Warning,
                            Some(4.0),
                        );
                    }
                }
                continue;
            }

            if let Some(RoomInputBarAction::AttachmentHandoffCanceled { kind }) =
                action.downcast_ref()
            {
                let kind = *kind;
                let label = kind.label();
                let pending_attachment = self.telegram_pending_attachment_send.as_ref();
                let pending_filename = pending_attachment.map(|pending| pending.filename.clone());
                let pending_voice_filename = pending_attachment
                    .filter(|pending| pending.kind == AttachmentHandoffKind::Voice)
                    .map(|pending| pending.filename.clone());
                let reply_context_loaded = pending_attachment
                    .map(|pending| pending.in_reply_to.is_some())
                    .unwrap_or_else(|| self.replying_to.is_some());
                let has_pending_attachment = pending_filename.is_some();
                self.telegram_attachment_local_status = if let Some(filename) = &pending_filename {
                    format!(
                        "{label} attachment send confirmation canceled locally; still reviewing existing pending attachment: {filename}"
                    )
                } else {
                    format!("{label} attachment send confirmation canceled before picker")
                };
                self.update_telegram_attachment_picker(cx);
                self.set_telegram_attachment_picker_visible(cx, has_pending_attachment);
                if kind == AttachmentHandoffKind::Voice {
                    self.telegram_voice_local_status =
                        "Voice Send confirmation canceled locally".to_string();
                    self.update_telegram_voice_message_panel(cx);
                    self.set_telegram_voice_message_panel_visible(cx, true);
                    let cancel_metadata = voice_confirmation_cancel_metadata_label(
                        pending_voice_filename.as_deref(),
                        reply_context_loaded,
                    );
                    self.set_message_send_operation_status(
                            cx,
                            "voice-confirmation-canceled-local",
                            "Voice confirmation canceled locally",
                        &format!(
                            "{cancel_metadata} No desktop picker, microphone permission, recorder, upload, SendAttachment, SDK queue cancel, room-state, membership, gateway/runtime/auth, or live mutation request was emitted."
                        ),
                    );
                    enqueue_popup_notification(
                        format!(
                            "Voice Send confirmation canceled before the desktop audio picker. {cancel_metadata}"
                        ),
                        PopupKind::Info,
                        Some(3.0),
                    );
                }
                continue;
            }

            if let Some(RoomInputBarAction::AttachmentSendRetryConfirmed { attempt }) =
                action.downcast_ref()
            {
                let attempt = attempt.clone();
                let label = attempt.kind.label();
                let filename = attempt.filename.clone();
                let mime_label = attempt.mime_type.to_string();
                let retry_metadata = attachment_send_failure_retry_confirmation_label(
                    &filename,
                    label,
                    attempt.caption.is_some(),
                    attempt.in_reply_to.is_some(),
                );
                self.telegram_attachment_send_retry_attempt = Some(attempt.clone());
                self.telegram_attachment_send_cached_error = None;
                self.telegram_attachment_local_status = format!(
                    "{label} attachment retry submitted after confirmation: {filename} ({mime_label})"
                );
                submit_async_request(MatrixRequest::SendAttachment {
                    timeline_kind: attempt.timeline_kind,
                    file_path: attempt.file_path,
                    mime_type: attempt.mime_type,
                    caption: attempt.caption,
                    mentions: attempt.mentions,
                    in_reply_to: attempt.in_reply_to,
                });
                self.update_telegram_attachment_picker(cx);
                self.set_telegram_attachment_picker_visible(cx, true);
                self.set_message_send_operation_status(
                    cx,
                    "retry-confirmed",
                    "Attachment Retry confirmed",
                    &format!(
                        "PositiveConfirmationModal accepted the failed attachment handoff Retry. The cached MatrixRequest::SendAttachment was resubmitted with the same TimelineKind, local file path, MIME type, caption, compact caption mentions, and reply id. This does not retry or resume accepted SDK queue uploads, abort uploads, remove queued media, send a caption-only SendMessage, mutate room-state or membership, touch account/profile, call gateway/runtime/auth, or perform live mutation. {retry_metadata}"
                    ),
                );
                enqueue_popup_notification(
                    format!(
                        "{label} attachment retry confirmed and resubmitted to the existing SendAttachment handoff: {filename} ({mime_label})."
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
                continue;
            }

            if let Some(RoomInputBarAction::LocationSendConfirmed {
                timeline_kind,
                coords,
                replied_to,
                #[cfg(feature = "tsp")]
                sign_with_tsp,
            }) = action.downcast_ref()
            {
                let geo_uri = format!(
                    "{}{},{}",
                    utils::GEO_URI_SCHEME,
                    coords.latitude,
                    coords.longitude
                );
                let message = RoomMessageEventContent::new(MessageType::Location(
                    LocationMessageEventContent::new(geo_uri.clone(), geo_uri),
                ));
                // Location confirmation evidence: only this confirmed action submits the
                // existing Matrix location SendMessage path.
                submit_async_request(MatrixRequest::SendMessage {
                    timeline_kind: timeline_kind.clone(),
                    message,
                    replied_to: replied_to.clone().map(RoomInputBarReplyTarget::into_reply),
                    #[cfg(feature = "tsp")]
                    sign_with_tsp: *sign_with_tsp,
                });

                self.clear_replying_to(cx);
                self.telegram_pending_attachment_send = None;
                self.telegram_attachment_send_retry_attempt = None;
                let location_preview = self.location_preview(cx, ids!(location_preview));
                location_preview.clear();
                location_preview.redraw(cx);
                enqueue_popup_notification(
                    "Location send confirmed. Existing Matrix location message path was requested.",
                    PopupKind::Info,
                    Some(4.0),
                );
                self.set_message_send_operation_status(
                    cx,
                    "location submitted",
                    "Location SendMessage submitted",
                    "Existing MatrixRequest::SendMessage was submitted for this location. Queued/progress/failure labels plus Retry/Cancel controls are local evidence only; no retry or cancel request was emitted from the evidence strip.",
                );
                return;
            }
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_actions
                        .pause_attachment_queue_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_accepted_queue_action(cx, "Pause");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_actions
                        .resume_attachment_queue_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_accepted_queue_action(cx, "Resume");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_actions
                        .reorder_attachment_queue_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_accepted_queue_action(cx, "Reorder");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_actions
                        .background_attachment_queue_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_accepted_queue_action(cx, "Background");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_actions
                        .clear_attachment_queue_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_accepted_queue_action(cx, "Clear");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_timeline_cancel_bridge
                        .status_attachment_timeline_cancel_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_timeline_cancel_bridge_control(cx, "Status");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_timeline_cancel_bridge
                        .handle_attachment_timeline_cancel_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_timeline_cancel_bridge_control(cx, "Handle");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_timeline_cancel_bridge
                        .timeline_attachment_timeline_cancel_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_timeline_cancel_bridge_control(cx, "Timeline");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_timeline_cancel_bridge
                        .cancel_attachment_timeline_cancel_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_timeline_cancel_bridge_control(cx, "Cancel");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .accepted_queue_timeline_cancel_bridge
                        .source_attachment_timeline_cancel_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_timeline_cancel_bridge_control(cx, "Source");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .status_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Status");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .progress_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Progress");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .pause_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Pause");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .resume_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Resume");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .cancel_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Cancel");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .retry_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Retry");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .drilldown_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Drilldown");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .contract_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Contract");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .per_file_status_controls
                        .taxonomy_attachment_file_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_per_file_status_control(cx, "Taxonomy");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .send_preflight_detail_controls
                        .request_attachment_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_send_preflight_detail_control(cx, "Request");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .send_preflight_detail_controls
                        .result_attachment_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_send_preflight_detail_control(cx, "Result");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .send_preflight_detail_controls
                        .error_attachment_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_send_preflight_detail_control(cx, "Error");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .send_preflight_detail_controls
                        .retry_attachment_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_send_preflight_detail_control(cx, "Retry");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .send_preflight_detail_controls
                        .source_attachment_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_send_preflight_detail_control(cx, "Source");
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .status_actions
                        .retry_send_operation_button
                ),
            )
            .clicked(actions)
        {
            if let Some(attempt) = self.telegram_attachment_send_retry_attempt.clone() {
                let label = attempt.kind.label();
                let filename = attempt.filename.clone();
                let retry_metadata = attachment_send_failure_retry_confirmation_label(
                    &filename,
                    label,
                    attempt.caption.is_some(),
                    attempt.in_reply_to.is_some(),
                );
                self.set_message_send_operation_status(
                    cx,
                    "retry-confirmation-open",
                    "Confirm attachment Retry",
                    &format!(
                        "Attachment Retry is available only for the cached immediate SendAttachment handoff failure. Confirming will resubmit the same local file path, MIME type, caption, reply id, and TimelineKind; canceling the confirmation stays local. This is not SDK queue retry/resume, upload abort, queue removal, delivery receipt mapping, caption-only SendMessage fallback, room-state, membership, gateway/runtime/auth, or live mutation. {retry_metadata}"
                    ),
                );
                let attempt_for_accept = attempt.clone();
                let content = ConfirmationModalContent {
                    title_text: "Retry attachment handoff".into(),
                    body_text: format!(
                        "Retry sending {label} attachment {filename}? This reuses the cached local file, MIME type, caption, reply id, and timeline after the worker failed before SDK queue ownership. It does not retry or cancel accepted SDK queue uploads."
                    )
                    .into(),
                    accept_button_text: Some("Retry".into()),
                    cancel_button_text: Some("Keep Failed".into()),
                    on_accept_clicked: Some(Box::new(move |cx| {
                        cx.action(RoomInputBarAction::AttachmentSendRetryConfirmed {
                            attempt: attempt_for_accept.clone(),
                        });
                    })),
                    on_cancel_clicked: Some(Box::new(move |_cx| {
                        enqueue_popup_notification(
                            format!(
                                "{label} attachment Retry confirmation canceled locally. No SendAttachment resubmit, SDK queue retry, upload abort, or live mutation was emitted."
                            ),
                            PopupKind::Info,
                            Some(4.0),
                        );
                    })),
                };
                enqueue_popup_notification(
                    format!(
                        "Attachment Retry confirmation opened for {filename}. No SendAttachment resubmit occurs before confirmation."
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
                cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
                    content,
                ))));
            } else {
                self.set_message_send_operation_status(
                    cx,
                    "retry-local",
                    "Retry staged locally",
                    "Retry had no cached failed SendAttachment handoff to reuse, so it only updates this local recovery copy. It does not submit SendMessage or SendAttachment, does not send the caption as a plain message, does not duplicate media upload, does not replace the SDK send queue item, and sends no room-state, membership, gateway/runtime/auth, account/profile, or live mutation request.",
                );
                enqueue_popup_notification(
                    "Retry has no cached failed attachment handoff. No Matrix retry, duplicate SendAttachment, caption-only SendMessage, or upload was requested.",
                    PopupKind::Info,
                    Some(4.0),
                );
            }
        }

        if self
            .button(
                cx,
                ids!(
                    send_operation_status
                        .status_actions
                        .cancel_send_operation_button
                ),
            )
            .clicked(actions)
        {
            self.telegram_attachment_send_retry_attempt = None;
            self.set_message_send_operation_status(
                cx,
                "cancel-local",
                "Cancel staged locally",
                "Cancel clears only the local cached failed-handoff retry attempt and updates this recovery copy after a queued attachment handoff or popup failure. It does not abort SDK send-queue work, remove queued attachments, cancel upload, clear Matrix queue state, or send a Matrix cancel, room-state, membership, gateway/runtime/auth, account/profile, or live mutation request.",
            );
            enqueue_popup_notification(
                "Cancel cleared local attachment retry cache only. No SDK send-queue cancel, Matrix cancel, upload abort, or queue removal was emitted.",
                PopupKind::Info,
                Some(4.0),
            );
        }

        // Clear the replying-to preview pane if the "cancel reply" button was clicked
        // or if the `Escape` key was pressed within the message input box.
        if self.button(cx, ids!(cancel_reply_button)).clicked(actions)
            || text_input.escaped(actions)
        {
            self.clear_replying_to(cx);
            self.redraw(cx);
        }

        if self.button(cx, ids!(attachment_button)).clicked(actions) {
            self.show_telegram_attachment_picker(cx);
            enqueue_popup_notification(
                "File attachments require confirmation first. On desktop, choosing Photo or File opens the native picker, then selected files enter local review before Matrix attachment send.",
                PopupKind::Info,
                Some(4.0),
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_header
                        .close_attachment_picker_button
                ),
            )
            .clicked(actions)
        {
            let discarded = self.telegram_pending_attachment_send.take();
            self.telegram_attachment_send_retry_attempt = None;
            self.set_telegram_attachment_picker_visible(cx, false);
            if let Some(pending) = discarded {
                let mime_label = pending.mime_type.to_string();
                let lifecycle_metadata = attachment_review_lifecycle_metadata_label(
                    "closed",
                    pending.kind.label(),
                    Some(&pending.filename),
                    Some(&mime_label),
                    pending.file_size_bytes,
                    Some(&pending.caption_preview),
                    pending.in_reply_to.is_some(),
                    pending.validation_error.as_deref(),
                    None,
                );
                let validation_note = pending
                    .validation_error
                    .as_deref()
                    .map(|reason| format!("; cleared local validation warning: {reason}"))
                    .unwrap_or_default();
                self.telegram_attachment_local_status = format!(
                    "{} attachment closed and discarded locally: {}{}",
                    pending.kind.label(),
                    pending.filename,
                    validation_note
                );
                self.set_message_send_operation_status(
                    cx,
                    "closed-local",
                    "Attachment review closed locally",
                    &format!(
                        "Close consumed and cleared the pending selected attachment plus any validation warning locally while preserving composer caption/reply text. Repeated Close or review-row Send after Close has no pending attachment to submit. No MatrixRequest::SendAttachment, caption-only SendMessage, upload, SDK send-queue cancel, room-state, membership, gateway/runtime/auth, account/profile, or live mutation request was emitted. {lifecycle_metadata}"
                    ),
                );
                enqueue_popup_notification(
                    format!(
                        "{} attachment review closed and discarded locally. No upload or Matrix media send was started. {lifecycle_metadata}",
                        pending.kind.label()
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            } else {
                let lifecycle_metadata = attachment_review_lifecycle_metadata_label(
                    "empty close",
                    "Attachment",
                    None,
                    None,
                    None,
                    None,
                    false,
                    None,
                    None,
                );
                self.telegram_attachment_local_status =
                    "Attachment picker closed locally with no pending attachment".to_string();
                self.set_message_send_operation_status(
                    cx,
                    "empty-held",
                    "Close held locally",
                    &format!(
                        "Close found no pending attachment review state. This empty Close stays local, preserves composer caption/reply text, and does not submit SendAttachment, send a caption-only SendMessage, upload media, cancel SDK send-queue work, or mutate room-state, membership, account/profile, gateway/runtime/auth, or live state. {lifecycle_metadata}"
                    ),
                );
                enqueue_popup_notification(
                    format!(
                        "Attachment picker closed locally. No native picker, upload, or Matrix media send was started. {lifecycle_metadata}"
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            }
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_review_actions
                        .send_selected_attachment_button
                ),
            )
            .clicked(actions)
        {
            if let Some(mut pending) = self.telegram_pending_attachment_send.take() {
                if let Err(validation_reason) =
                    validate_telegram_attachment_file_for_review_send(&pending.file_path)
                {
                    let label = pending.kind.label();
                    let filename = pending.filename.clone();
                    let mime_label = pending.mime_type.to_string();
                    pending.file_size_bytes = telegram_attachment_file_size(&pending.file_path);
                    if pending.kind == AttachmentHandoffKind::Photo {
                        pending.image_dimensions_label = Some(selected_image_dimensions_label(
                            &pending.file_path,
                            &pending.mime_type,
                        ));
                    }
                    if pending.kind == AttachmentHandoffKind::Voice {
                        pending.audio_duration_label = Some(voice_audio_duration_label(
                            &pending.file_path,
                            &pending.mime_type,
                        ));
                    }
                    pending.validation_error = Some(validation_reason.to_string());
                    self.telegram_pending_attachment_send = Some(pending);
                    self.telegram_attachment_local_status = format!(
                        "{label} attachment validation held locally: {filename} ({mime_label}); {validation_reason}"
                    );
                    self.update_telegram_attachment_picker(cx);
                    self.set_telegram_attachment_picker_visible(cx, true);
                    self.set_message_send_operation_status(
                        cx,
                        "validation-held",
                        "Attachment validation held locally",
                        "Review-row Send revalidated the selected file before MatrixRequest::SendAttachment. The selected path was unreadable, not a regular file, or an empty file, so pending review stayed local with a visible validation warning, composer caption/reply text was preserved, and no SendAttachment, caption-only SendMessage, upload, SDK send-queue cancel, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request was emitted. Replace, Discard, and Close are the local recovery paths; Retry/Cancel controls remain local evidence only.",
                    );
                    enqueue_popup_notification(
                        format!(
                            "{label} attachment stayed in local review because {validation_reason}. Choose another file or discard it."
                        ),
                        PopupKind::Warning,
                        Some(4.0),
                    );
                    return;
                }
                let PendingAttachmentSend {
                    kind,
                    timeline_kind,
                    file_path,
                    mime_type,
                    filename,
                    in_reply_to,
                    ..
                } = pending;
                let label = kind.label();
                let mime_label = mime_type.to_string();
                let caption_text = mentionable_text_input.text().trim().to_string();
                let mentions = (!caption_text.is_empty())
                    .then(|| {
                        mentionable_text_input.mentions_for_text(
                            &caption_text,
                            room_screen_props
                                .room_members
                                .as_ref()
                                .map(|members| members.as_slice()),
                        )
                    })
                    .flatten();
                let caption = (!caption_text.is_empty())
                    .then(|| TextMessageEventContent::plain(caption_text));
                let retry_attempt = AttachmentSendRetryAttempt {
                    kind,
                    timeline_kind: timeline_kind.clone(),
                    file_path: file_path.clone(),
                    mime_type: mime_type.clone(),
                    filename: filename.clone(),
                    caption: caption.clone(),
                    mentions: mentions.clone(),
                    in_reply_to: in_reply_to.clone(),
                };
                self.telegram_attachment_send_retry_attempt = Some(retry_attempt);
                self.telegram_attachment_send_cached_error = None;

                // Consume pending state before submit so duplicate/second clicks fall
                // into the local empty review guard instead of resubmitting.
                submit_async_request(MatrixRequest::SendAttachment {
                    timeline_kind,
                    file_path,
                    mime_type,
                    caption,
                    mentions,
                    in_reply_to,
                });

                self.clear_replying_to(cx);
                mentionable_text_input.set_text(cx, "");
                self.update_hepta_command_preview(cx, "");
                self.telegram_attachment_local_status =
                    format!("{label} attachment send queued: {filename} ({mime_label})");
                self.update_telegram_attachment_picker(cx);
                self.set_telegram_attachment_picker_visible(cx, true);
                self.set_message_send_operation_status(
                    cx,
                    "queued-only",
                    "Attachment SendAttachment submitted",
                    "MatrixRequest::SendAttachment was submitted only after local attachment review Send consumed pending state before submit; this is the handoff-submitted taxonomy boundary and caches the last validated handoff for a possible confirmed worker-failure Retry. The UI labels this as queued-only until the worker returns a queued-confirmed or failure-copy handoff result: the existing matrix-sdk-ui Timeline::send_attachment().use_send_queue() path owns upload/media send. Review-row Send is the only attachment path that consumes the current composer caption into SendAttachment, carries compact caption mentions through AttachmentConfig.mentions, carries the captured reply/thread event id, then clears composer text and reply preview after submit. Retry never auto-runs; Cancel does not abort, remove, or cancel SDK send-queue work.",
                );
                enqueue_popup_notification(
                    format!(
                        "{label} attachment queued for Matrix media send after review: {filename} ({mime_label})."
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            } else {
                self.telegram_attachment_local_status =
                    "No selected attachment is waiting for review".to_string();
                self.update_telegram_attachment_picker(cx);
                self.set_telegram_attachment_picker_visible(cx, true);
                self.set_message_send_operation_status(
                    cx,
                    "empty-held",
                    "Review Send held locally",
                    "No pending attachment was available for review-row Send. This empty or duplicate Send stays local and preserves composer caption/reply text: no duplicate MatrixRequest::SendAttachment, no caption-only SendMessage, no upload, no SDK send-queue cancel, and no room-state, membership, account/profile, gateway/runtime/auth, or live mutation request.",
                );
                enqueue_popup_notification(
                    "No selected attachment is waiting for review. Choose Photo or File first.",
                    PopupKind::Info,
                    Some(3.0),
                );
            }
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_review_actions
                        .discard_selected_attachment_button
                ),
            )
            .clicked(actions)
        {
            self.telegram_attachment_send_retry_attempt = None;
            if let Some(pending) = self.telegram_pending_attachment_send.take() {
                let mime_label = pending.mime_type.to_string();
                let lifecycle_metadata = attachment_review_lifecycle_metadata_label(
                    "discarded",
                    pending.kind.label(),
                    Some(&pending.filename),
                    Some(&mime_label),
                    pending.file_size_bytes,
                    Some(&pending.caption_preview),
                    pending.in_reply_to.is_some(),
                    pending.validation_error.as_deref(),
                    None,
                );
                let validation_note = pending
                    .validation_error
                    .as_deref()
                    .map(|reason| format!("; cleared local validation warning: {reason}"))
                    .unwrap_or_default();
                self.telegram_attachment_local_status = format!(
                    "{} attachment discarded locally: {}{}",
                    pending.kind.label(),
                    pending.filename,
                    validation_note
                );
                self.update_telegram_attachment_picker(cx);
                self.set_telegram_attachment_picker_visible(cx, true);
                self.set_message_send_operation_status(
                    cx,
                    "discarded-local",
                    "Attachment discarded locally",
                    &format!(
                        "Discard cleared the pending selected attachment plus any validation warning locally after consuming it with Option::take() while preserving composer caption/reply text. Repeated Discard or review-row Send after Discard has no pending attachment to submit. No MatrixRequest::SendAttachment, caption-only SendMessage, upload, SDK send-queue cancel, room-state, membership, gateway/runtime/auth, account/profile, or live mutation request was emitted. {lifecycle_metadata}"
                    ),
                );
                enqueue_popup_notification(
                    format!(
                        "{} attachment discarded locally. No upload or Matrix media send was started. {lifecycle_metadata}",
                        pending.kind.label()
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            } else {
                let lifecycle_metadata = attachment_review_lifecycle_metadata_label(
                    "empty discard",
                    "Attachment",
                    None,
                    None,
                    None,
                    None,
                    false,
                    None,
                    None,
                );
                self.telegram_attachment_local_status =
                    "No selected attachment to discard".to_string();
                self.update_telegram_attachment_picker(cx);
                self.set_telegram_attachment_picker_visible(cx, true);
                self.set_message_send_operation_status(
                    cx,
                    "empty-held",
                    "Discard held locally",
                    &format!(
                        "Discard found no pending attachment review state. This empty or repeated Discard stays local, preserves composer caption/reply text, and does not submit SendAttachment, send a caption-only SendMessage, upload media, cancel SDK send-queue work, or mutate room-state, membership, account/profile, gateway/runtime/auth, or live state. {lifecycle_metadata}"
                    ),
                );
                enqueue_popup_notification(
                    format!(
                        "No selected attachment to discard. No Matrix media request was emitted. {lifecycle_metadata}"
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            }
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_options
                        .photo_attachment_button
                ),
            )
            .clicked(actions)
        {
            self.open_telegram_attachment_handoff_confirmation(
                cx,
                AttachmentHandoffKind::Photo,
                room_screen_props.timeline_kind.clone(),
                self.replied_to_for_send(&room_screen_props.timeline_kind)
                    .map(|target| target.event_id),
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_options
                        .file_attachment_button
                ),
            )
            .clicked(actions)
        {
            self.open_telegram_attachment_handoff_confirmation(
                cx,
                AttachmentHandoffKind::File,
                room_screen_props.timeline_kind.clone(),
                self.replied_to_for_send(&room_screen_props.timeline_kind)
                    .map(|target| target.event_id),
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_options
                        .camera_attachment_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_choice(cx, "Camera");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_options
                        .contact_attachment_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_choice(cx, "Contact");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_mobile_picker_controls
                        .gallery_attachment_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_mobile_picker_control(cx, "Gallery");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_mobile_picker_controls
                        .camera_attachment_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_mobile_picker_control(cx, "Camera");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_mobile_picker_controls
                        .files_attachment_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_mobile_picker_control(cx, "Files");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_mobile_picker_controls
                        .contact_attachment_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_mobile_picker_control(cx, "Contact");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_mobile_picker_controls
                        .thumbnail_attachment_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_mobile_picker_control(cx, "Thumbnail");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_mobile_picker_controls
                        .share_attachment_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_attachment_mobile_picker_control(cx, "Share");
        }

        if self.button(cx, ids!(emoji_button)).clicked(actions) {
            self.show_telegram_emoji_sticker_panel(cx);
            let lifecycle_metadata = self.current_emoji_sticker_lifecycle_metadata_label("opened");
            enqueue_popup_notification(
                format!(
                    "Emoji and sticker picking is staged in the Telegram composer emoji/sticker surface. This local preview does not open a picker, upload stickers, or send Matrix content. {lifecycle_metadata}"
                ),
                PopupKind::Info,
                Some(4.0),
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_emoji_sticker_panel
                        .emoji_header
                        .close_emoji_sticker_panel_button
                ),
            )
            .clicked(actions)
        {
            self.telegram_emoji_sticker_local_status =
                "Emoji/sticker picker closed locally".to_string();
            self.telegram_emoji_sticker_last_lifecycle_action = "closed".to_string();
            self.set_telegram_emoji_sticker_panel_visible(cx, false);
            self.update_telegram_emoji_sticker_panel(cx);
            let lifecycle_metadata = self.current_emoji_sticker_lifecycle_metadata_label("closed");
            enqueue_popup_notification(
                format!(
                    "Emoji/sticker picker closed locally. No picker, sticker upload, or Matrix media send was started. {lifecycle_metadata}"
                ),
                PopupKind::Info,
                Some(3.0),
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_emoji_sticker_panel
                        .emoji_options
                        .smile_emoji_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_emoji_sticker_choice(cx, "Smile");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_emoji_sticker_panel
                        .emoji_options
                        .thumbs_emoji_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_emoji_sticker_choice(cx, "Thumbs");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_emoji_sticker_panel
                        .emoji_options
                        .heart_emoji_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_emoji_sticker_choice(cx, "Heart");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_emoji_sticker_panel
                        .emoji_options
                        .sticker_emoji_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_emoji_sticker_choice(cx, "Sticker");
        }

        if self.button(cx, ids!(voice_message_button)).clicked(actions) {
            self.show_telegram_voice_message_panel(cx);
            let lifecycle_metadata =
                self.current_voice_lifecycle_metadata_label("opened", "voice panel opened");
            enqueue_popup_notification(
                format!(
                    "Voice messages open a guarded composer surface. Record and Lock stay local; Send can choose an existing desktop audio file for review before Matrix attachment send. {lifecycle_metadata}"
                ),
                PopupKind::Info,
                Some(4.0),
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_header
                        .close_voice_message_panel_button
                ),
            )
            .clicked(actions)
        {
            self.telegram_voice_local_status = "Voice preview closed locally".to_string();
            self.update_telegram_voice_message_panel(cx);
            self.set_telegram_voice_message_panel_visible(cx, false);
            let lifecycle_metadata =
                self.current_voice_lifecycle_metadata_label("closed", "close control staged");
            enqueue_popup_notification(
                format!(
                    "Voice preview closed locally. No microphone permission, recording, upload, or Matrix media send was started. {lifecycle_metadata}"
                ),
                PopupKind::Info,
                Some(3.0),
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_controls
                        .record_voice_preview_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_choice(cx, "Record");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_controls
                        .lock_voice_preview_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_choice(cx, "Lock");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_controls
                        .cancel_voice_preview_button
                ),
            )
            .clicked(actions)
        {
            self.telegram_voice_local_status = "Voice preview cancelled locally".to_string();
            self.update_telegram_voice_message_panel(cx);
            self.set_telegram_voice_message_panel_visible(cx, false);
            let lifecycle_metadata =
                self.current_voice_lifecycle_metadata_label("cancelled", "cancel control staged");
            enqueue_popup_notification(
                format!(
                    "Voice preview was cancelled locally. No microphone permission, recording, upload, or Matrix media send was started. {lifecycle_metadata}"
                ),
                PopupKind::Info,
                Some(4.0),
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_controls
                        .send_voice_preview_button
                ),
            )
            .clicked(actions)
        {
            self.telegram_voice_local_status = "Voice Send confirmation opened locally".to_string();
            self.update_telegram_voice_message_panel(cx);
            let in_reply_to = self.replied_to_event_id();
            self.open_telegram_attachment_handoff_confirmation(
                cx,
                AttachmentHandoffKind::Voice,
                room_screen_props.timeline_kind.clone(),
                in_reply_to,
            );
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_recorder_status_controls
                        .timer_voice_status_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_recorder_status_control(cx, "Timer");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_recorder_status_controls
                        .waveform_voice_status_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_recorder_status_control(cx, "Waveform");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_recorder_status_controls
                        .transcript_voice_status_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_recorder_status_control(cx, "Transcript");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_recorder_status_controls
                        .progress_voice_status_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_recorder_status_control(cx, "Progress");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_recorder_status_controls
                        .codec_voice_status_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_recorder_status_control(cx, "Codec");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_capture_lifecycle_controls
                        .permission_voice_capture_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_capture_lifecycle_control(cx, "Permission");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_capture_lifecycle_controls
                        .capture_voice_capture_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_capture_lifecycle_control(cx, "Capture");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_capture_lifecycle_controls
                        .encode_voice_capture_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_capture_lifecycle_control(cx, "Encode");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_capture_lifecycle_controls
                        .review_voice_capture_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_capture_lifecycle_control(cx, "Review");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_capture_lifecycle_controls
                        .upload_voice_capture_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_capture_lifecycle_control(cx, "Upload");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_capture_lifecycle_controls
                        .packet_voice_capture_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_capture_lifecycle_control(cx, "Packet");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_capture_lifecycle_controls
                        .contract_voice_capture_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_capture_lifecycle_control(cx, "Contract");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_capture_lifecycle_controls
                        .taxonomy_voice_capture_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_capture_lifecycle_control(cx, "Taxonomy");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_mobile_picker_controls
                        .mic_voice_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_mobile_picker_control(cx, "Mic");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_mobile_picker_controls
                        .files_voice_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_mobile_picker_control(cx, "Files");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_mobile_picker_controls
                        .library_voice_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_mobile_picker_control(cx, "Library");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_mobile_picker_controls
                        .retake_voice_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_mobile_picker_control(cx, "Retake");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_mobile_picker_controls
                        .share_voice_mobile_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_mobile_picker_control(cx, "Share");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_review_playback_controls
                        .play_voice_review_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_review_playback_control(cx, "Play");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_review_playback_controls
                        .pause_voice_review_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_review_playback_control(cx, "Pause");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_review_playback_controls
                        .scrub_voice_review_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_review_playback_control(cx, "Scrub");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_review_playback_controls
                        .speed_voice_review_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_review_playback_control(cx, "Speed");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_review_playback_controls
                        .drop_voice_review_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_review_playback_control(cx, "Drop");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_send_preflight_detail_controls
                        .request_voice_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_send_preflight_detail_control(cx, "Request");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_send_preflight_detail_controls
                        .result_voice_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_send_preflight_detail_control(cx, "Result");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_send_preflight_detail_controls
                        .error_voice_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_send_preflight_detail_control(cx, "Error");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_send_preflight_detail_controls
                        .retry_voice_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_send_preflight_detail_control(cx, "Retry");
        }

        if self
            .button(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_send_preflight_detail_controls
                        .source_voice_send_preflight_button
                ),
            )
            .clicked(actions)
        {
            self.stage_telegram_voice_send_preflight_detail_control(cx, "Source");
        }

        // Handle the add location button being clicked.
        if self.button(cx, ids!(location_button)).clicked(actions) {
            self.set_telegram_attachment_picker_visible(cx, false);
            self.set_telegram_emoji_sticker_panel_visible(cx, false);
            self.set_telegram_voice_message_panel_visible(cx, false);
            log!("Add location button clicked; requesting current location...");
            if let Err(_e) = init_location_subscriber(cx) {
                error!("Failed to initialize location subscriber");
                enqueue_popup_notification(
                    "Failed to initialize location services.",
                    PopupKind::Error,
                    None,
                );
            }
            self.view
                .location_preview(cx, ids!(location_preview))
                .show();
            self.redraw(cx);
        }

        // Handle the send location button being clicked.
        if self
            .button(cx, ids!(location_preview.send_location_button))
            .clicked(actions)
        {
            let location_preview = self.location_preview(cx, ids!(location_preview));
            if let Some((coords, _system_time_opt)) = location_preview.get_current_data() {
                let timeline_kind = room_screen_props.timeline_kind.clone();
                let replied_to = self.replied_to_for_send(&room_screen_props.timeline_kind);
                #[cfg(feature = "tsp")]
                let sign_with_tsp = self.is_tsp_signing_enabled(cx);
                // Location confirmation evidence: opening/canceling this guard keeps the
                // location message unsent until the accept handler emits LocationSendConfirmed.
                let content = ConfirmationModalContent {
                    title_text: "Send Location".into(),
                    body_text: "Send your current location to this room? The existing Matrix location message path will only be requested after this confirmation.".into(),
                    accept_button_text: Some("Send Location".into()),
                    cancel_button_text: Some("Cancel".into()),
                    on_accept_clicked: Some(Box::new(move |cx| {
                        cx.action(RoomInputBarAction::LocationSendConfirmed {
                            timeline_kind,
                            coords,
                            replied_to,
                            #[cfg(feature = "tsp")]
                            sign_with_tsp,
                        });
                    })),
                    on_cancel_clicked: Some(Box::new(|_cx| {
                        enqueue_popup_notification(
                            "Location send canceled. No Matrix location message was sent.",
                            PopupKind::Info,
                            Some(3.0),
                        );
                    })),
                };
                enqueue_popup_notification(
                    "Location send confirmation opened. No Matrix location message was sent before confirmation.",
                    PopupKind::Info,
                    Some(4.0),
                );
                cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
                    content,
                ))));
            }
        }

        // Handle the send message button being clicked, or a `Returned` action
        // from the message text input. The text input only emits `Returned`
        // for the key combination chosen by the user in App Settings (plus
        // Cmd/Ctrl+Enter, which always submits).
        if self.button(cx, ids!(send_message_button)).clicked(actions)
            || text_input.returned(actions).is_some()
        {
            let pending_attachment_summary = self
                .telegram_pending_attachment_send
                .as_ref()
                .map(|pending| (pending.kind.label(), pending.filename.clone()));
            if let Some((label, filename)) = pending_attachment_summary {
                self.telegram_attachment_local_status = format!(
                    "{label} attachment still waiting for review: {filename}. Use attachment review Send."
                );
                self.update_telegram_attachment_picker(cx);
                self.set_telegram_attachment_picker_visible(cx, true);
                self.set_message_send_operation_status(
                    cx,
                    "attachment review required",
                    "Main Send held locally",
                    "A selected attachment is pending review. Main composer Send/Enter preserved the pending attachment plus composer caption/reply preview, did not send the caption as plain text, did not submit SendAttachment, and did not clear the pending attachment. Use the attachment review row Send to submit it.",
                );
                enqueue_popup_notification(
                    format!(
                        "{label} attachment is waiting for review: {filename}. Use the attachment review Send button to submit it."
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
                return;
            }

            let entered_text = mentionable_text_input.text().trim().to_string();
            if !entered_text.is_empty() {
                if let Some(plan) = plan_hepta_composer_command(&entered_text, current_time_ms()) {
                    let preview = plan.to_bridge_input();
                    enqueue_popup_notification(
                        format!(
                            "Hepta dry-run staged locally: {}\nPreview event: m.hepta.{} / {}\n\nHepta native execution adapters are pending; no external mutation was sent.",
                            plan.operator_summary(),
                            preview.event_kind,
                            preview.id,
                        ),
                        PopupKind::Warning,
                        Some(6.0),
                    );
                    mentionable_text_input.set_text(cx, "");
                    self.update_hepta_command_preview(cx, "");
                    self.set_telegram_attachment_picker_visible(cx, false);
                    self.set_telegram_emoji_sticker_panel_visible(cx, false);
                    self.set_telegram_voice_message_panel_visible(cx, false);
                    self.enable_send_message_button(cx, false);
                    return;
                }

                let message = mentionable_text_input.create_message_with_mentions(
                    &entered_text,
                    room_screen_props
                        .room_members
                        .as_ref()
                        .map(|members| members.as_slice()),
                );
                let mention_payload_metadata = mentionable_text_input.send_payload_metadata_label(
                    &entered_text,
                    room_screen_props
                        .room_members
                        .as_ref()
                        .map(|members| members.as_slice()),
                );
                let replied_to = self
                    .replying_to
                    .take()
                    .and_then(|(event_tl_item, _emb)| {
                        event_tl_item.event_id().map(|event_id| {
                            let enforce_thread = if room_screen_props
                                .timeline_kind
                                .thread_root_event_id()
                                .is_some()
                            {
                                EnforceThread::Threaded(ReplyWithinThread::Yes)
                            } else {
                                EnforceThread::MaybeThreaded
                            };
                            Reply {
                                event_id: event_id.to_owned(),
                                enforce_thread,
                                add_mentions: AddMentions::Yes,
                            }
                        })
                    })
                    .or_else(|| {
                        room_screen_props.timeline_kind.thread_root_event_id().map(
                            |thread_root_event_id| Reply {
                                event_id: thread_root_event_id.clone(),
                                enforce_thread: EnforceThread::Threaded(ReplyWithinThread::No),
                                add_mentions: AddMentions::No,
                            },
                        )
                    });
                submit_async_request(MatrixRequest::SendMessage {
                    timeline_kind: room_screen_props.timeline_kind.clone(),
                    message,
                    replied_to,
                    #[cfg(feature = "tsp")]
                    sign_with_tsp: self.is_tsp_signing_enabled(cx),
                });

                self.clear_replying_to(cx);
                mentionable_text_input.set_text(cx, "");
                self.update_hepta_command_preview(cx, "");
                self.set_telegram_attachment_picker_visible(cx, false);
                self.set_telegram_emoji_sticker_panel_visible(cx, false);
                self.set_telegram_voice_message_panel_visible(cx, false);
                self.telegram_pending_attachment_send = None;
                self.telegram_attachment_send_retry_attempt = None;
                self.enable_send_message_button(cx, false);
                self.set_message_send_operation_status(
                    cx,
                    "text submitted",
                    "Text SendMessage submitted",
                    &format!(
                        "Existing MatrixRequest::SendMessage was submitted for this text/reply/thread send. Queued/progress/failure labels plus Retry/Cancel controls are local evidence only; no retry or cancel request was emitted from the evidence strip. {mention_payload_metadata}"
                    ),
                );
            }
        }

        // If the user starts/stops typing in the message input box,
        // send a typing notice to the room and update the send_message_button state.
        let is_text_input_empty = if let Some(new_text) = text_input.changed(actions) {
            self.update_hepta_command_preview(cx, &new_text);
            mentionable_text_input.update_cached_member_suggestions(
                cx,
                room_screen_props
                    .room_members
                    .as_ref()
                    .map(|members| members.as_slice()),
            );
            if let Some(pending) = self.telegram_pending_attachment_send.as_mut() {
                pending.caption_preview = summarize_attachment_caption(&new_text);
                self.update_telegram_attachment_picker(cx);
            }
            let is_empty = new_text.is_empty();
            if !looks_like_hepta_composer_command(&new_text) {
                submit_async_request(MatrixRequest::SendTypingNotice {
                    room_id: room_screen_props.timeline_kind.room_id().clone(),
                    typing: !is_empty,
                });
                self.set_typing_notice_status(
                    cx,
                    if is_empty {
                        "Typing notice cleared"
                    } else {
                        "Typing notice submitted"
                    },
                    "Existing MatrixRequest::SendTypingNotice was submitted for plain composer text. No message send, room-state, retry, or cancel request was emitted from the typing evidence strip.",
                );
            } else {
                self.set_typing_notice_status(
                    cx,
                    "Hepta command preview suppressed Matrix typing notice",
                    "Reserved Hepta command previews stay local and do not submit MatrixRequest::SendTypingNotice. No message send, room-state, retry, or cancel request was emitted from the typing evidence strip.",
                );
            }
            is_empty
        } else {
            text_input.text().is_empty()
        };
        self.enable_send_message_button(cx, !is_text_input_empty);

        // Handle the user pressing the up arrow in an empty message input box
        // to edit their latest sent message.
        if is_text_input_empty {
            if let Some(KeyEvent {
                key_code: KeyCode::ArrowUp,
                modifiers:
                    KeyModifiers {
                        shift: false,
                        control: false,
                        alt: false,
                        logo: false,
                    },
                ..
            }) = text_input.key_down_unhandled(actions)
            {
                cx.widget_action(
                    room_screen_props.room_screen_widget_uid,
                    MessageAction::EditLatest,
                );
            }
        }

        // When the hide animation fully completes, restore the replying preview.
        if self
            .view
            .editing_pane(cx, ids!(editing_pane))
            .was_hidden(actions)
        {
            self.on_editing_pane_hidden(cx);
        }
    }

    fn replied_to_for_send(&self, timeline_kind: &TimelineKind) -> Option<RoomInputBarReplyTarget> {
        self.replying_to
            .as_ref()
            .and_then(|(event_tl_item, _emb)| {
                event_tl_item.event_id().map(|event_id| {
                    let enforce_thread = if timeline_kind.thread_root_event_id().is_some() {
                        RoomInputBarReplyThread::ThreadedYes
                    } else {
                        RoomInputBarReplyThread::MaybeThreaded
                    };
                    RoomInputBarReplyTarget {
                        event_id: event_id.to_owned(),
                        enforce_thread,
                        add_mentions: true,
                    }
                })
            })
            .or_else(|| {
                timeline_kind
                    .thread_root_event_id()
                    .map(|thread_root_event_id| RoomInputBarReplyTarget {
                        event_id: thread_root_event_id.clone(),
                        enforce_thread: RoomInputBarReplyThread::ThreadedNo,
                        add_mentions: false,
                    })
            })
    }

    /// Shows a preview of the given event that the user is currently replying to
    /// above the message input bar.
    ///
    /// If `grab_key_focus` is true, this will also automatically focus the keyboard
    /// on the message input box so that the user can immediately start typing their reply.
    fn show_replying_to(
        &mut self,
        cx: &mut Cx,
        replying_to: (EventTimelineItem, EmbeddedEvent),
        timeline_kind: &TimelineKind,
        grab_key_focus: bool,
    ) {
        // When the user clicks the reply button next to a message, we need to:
        // 1. Populate and show the ReplyingPreview, of course.
        let replying_preview = self.view(cx, ids!(replying_preview));
        let (replying_preview_username, _) = replying_preview
            .avatar(cx, ids!(reply_preview_content.reply_preview_avatar))
            .set_avatar_and_get_username(
                cx,
                timeline_kind,
                replying_to.0.sender(),
                Some(replying_to.0.sender_profile()),
                replying_to.0.event_id(),
                true,
            );

        replying_preview
            .label(cx, ids!(reply_preview_content.reply_preview_username))
            .set_text(cx, replying_preview_username.as_str());

        populate_preview_of_timeline_item(
            cx,
            &replying_preview.html_or_plaintext(cx, ids!(reply_preview_content.reply_preview_body)),
            replying_to.0.content(),
            replying_to.0.sender(),
            &replying_preview_username,
        );

        replying_preview.set_visible(cx, true);
        self.replying_to = Some(replying_to);

        // 2. Hide other views that are irrelevant to a reply, e.g.,
        //    the `EditingPane` would improperly cover up the ReplyPreview.
        self.editing_pane(cx, ids!(editing_pane))
            .force_reset_hide(cx);
        self.on_editing_pane_hidden(cx);
        // 3. Automatically focus the keyboard on the message input box
        //    so that the user can immediately start typing their reply
        //    without having to manually click on the message input box.
        if grab_key_focus {
            self.text_input(cx, ids!(input_bar.mentionable_text_input.text_input))
                .set_key_focus(cx);
        }
        self.button(cx, ids!(cancel_reply_button)).reset_hover(cx);
        self.redraw(cx);
    }

    /// Clears (and makes invisible) the preview of the message
    /// that the user is currently replying to.
    fn clear_replying_to(&mut self, cx: &mut Cx) {
        self.view(cx, ids!(replying_preview)).set_visible(cx, false);
        self.replying_to = None;
    }

    /// Shows the editing pane to allow the user to edit the given event.
    fn show_editing_pane(
        &mut self,
        cx: &mut Cx,
        behavior: ShowEditingPaneBehavior,
        timeline_kind: TimelineKind,
    ) {
        // Cache the input_bar's natural height before the animation shrinks it.
        let input_bar_height = self.view.view(cx, ids!(input_bar)).area().rect(cx).size.y;
        if input_bar_height > 0.0 {
            self.input_bar_natural_height = input_bar_height;
        }

        // Hide the replying preview and location preview while the editing
        // pane is shown. The input_bar is not hidden; instead it is slid out
        // of view in draw_walk using the EditingPane's slide value.
        self.set_telegram_attachment_picker_visible(cx, false);
        self.telegram_pending_attachment_send = None;
        self.set_telegram_emoji_sticker_panel_visible(cx, false);
        self.set_telegram_voice_message_panel_visible(cx, false);
        let replying_preview = self.view.view(cx, ids!(replying_preview));
        self.was_replying_preview_visible = replying_preview.visible();
        replying_preview.set_visible(cx, false);
        self.view
            .location_preview(cx, ids!(location_preview))
            .clear();

        let editing_pane = self.view.editing_pane(cx, ids!(editing_pane));
        match behavior {
            ShowEditingPaneBehavior::ShowNew { event_tl_item } => {
                editing_pane.show(cx, event_tl_item, timeline_kind);
            }
            ShowEditingPaneBehavior::RestoreExisting { editing_pane_state } => {
                editing_pane.restore_state(cx, editing_pane_state, timeline_kind);
            }
        };

        self.redraw(cx);
    }

    /// This should be invoked after the EditingPane has been fully hidden.
    fn on_editing_pane_hidden(&mut self, cx: &mut Cx) {
        // Restore the replying_preview.
        if self.was_replying_preview_visible && self.replying_to.is_some() {
            self.view
                .view(cx, ids!(replying_preview))
                .set_visible(cx, true);
        }
        self.redraw(cx);
        // We don't need to do anything with the editing pane itself here,
        // because it has already been hidden by the time this function gets called.
    }

    fn update_telegram_attachment_picker(&mut self, cx: &mut Cx) {
        let status = if let Some(pending) = &self.telegram_pending_attachment_send {
            let validation_context = pending
                .validation_error
                .as_deref()
                .map(|reason| format!("; validation warning: {reason}"))
                .unwrap_or_default();
            let taxonomy_status = if pending.validation_error.is_some() {
                "validation-held"
            } else {
                "review-pending"
            };
            format!(
                "{taxonomy_status}: {} selected for review: {} ({}, {}){}",
                pending.kind.label(),
                pending.filename,
                pending.mime_type,
                format_attachment_file_size(pending.file_size_bytes),
                validation_context
            )
        } else if self.telegram_attachment_local_status.trim().is_empty() {
            "Choose Photo or File to confirm desktop picker + local review before Matrix send; Camera and Contact stage local previews"
                .to_string()
        } else {
            self.telegram_attachment_local_status.clone()
        };
        let header_status = if self.telegram_pending_attachment_send.is_some() {
            "review"
        } else {
            "confirm + review"
        };
        self.view
            .label(
                cx,
                ids!(
                    telegram_attachment_picker
                        .attachment_header
                        .attachment_status
                ),
            )
            .set_text(cx, header_status);
        self.view
            .label(cx, ids!(telegram_attachment_picker.attachment_summary))
            .set_text(
                cx,
                &format!(
                    "{status}. Send submits selected attachments; Discard, Close, picker cancel, and unsupported picker states send no upload or Matrix media request."
                ),
            );
        self.view
            .label(cx, ids!(telegram_attachment_picker.attachment_option_evidence))
            .set_text(
                cx,
                    "Photo and File confirm before desktop rfd picker; Voice Send confirms before a desktop audio picker. Selected files stage local review with filename, MIME, extension, size, caption preview, and reply context; selected Photo image files also show dimensions status from lightweight PNG/JPEG/GIF/BMP/WebP headers when available, and selected audio also shows duration status, codec/container status, and bounded WAV PCM waveform peaks when available. Attachment status taxonomy stays stable: review-pending, review-replaced, review-preserved, validation-held, handoff-submitted, queued-only, failure-copy, retry-confirmation-open, retry-confirmed, empty-held, discarded-local, closed-local, retry-local, and cancel-local. Caption preview live-updates from composer text. Main Send, picker cancel, Discard, Close, and empty review Send preserve composer caption/reply text. Choosing another file replaces only local pending review state, clears any local validation warning, and review Send consumes pending once before MatrixRequest::SendAttachment via Timeline::send_attachment().use_send_queue(). Review-row Send revalidates the selected path before submit; unreadable, non-file, or empty-file paths stay local with validation evidence. Replace, Discard, and Close recover from validation warnings locally; worker-failure Retry confirms before resubmitting only the cached last handoff, while Cancel clears local retry cache only and never cancels SDK queue work. MIME fallback to application/octet-stream, size unavailable, image dimensions unavailable, audio duration/codec/waveform unavailable states stay visible metadata before Send. Review-row Send is the only attachment path that consumes caption/reply context; empty or duplicate review Send stays local. Discard and Close are idempotent local cleanup; repeated Discard/Close and review Send after cleanup stay local. Camera, Contact, and Share stay local with no permissions, capture, share sheet, thumbnail decode, full image decode, contacts or shared-media read, payload, upload, or send.",
            );
        self.view
            .label(
                cx,
                ids!(telegram_attachment_picker.attachment_review_compact_fit),
            )
            .set_text(cx, ATTACHMENT_REVIEW_ROW_COMPACT_FIT_LABEL);
        self.view
            .label(
                cx,
                ids!(telegram_attachment_picker.attachment_review_action_density),
            )
            .set_text(cx, ATTACHMENT_MOBILE_ACTION_DENSITY_LABEL);
        let pending_review = self
            .telegram_pending_attachment_send
            .as_ref()
            .map(|pending| format!("{} {}", pending.kind.label(), pending.filename));
        let mobile_picker_controls_label = attachment_mobile_picker_controls_label(
            "Status",
            pending_review.as_deref(),
            self.telegram_attachment_local_status.as_str(),
        );
        self.view
            .label(
                cx,
                ids!(telegram_attachment_picker.attachment_mobile_picker_controls_label),
            )
            .set_text(cx, &mobile_picker_controls_label);
        if let Some(pending) = &self.telegram_pending_attachment_send {
            let reply_context = if pending.in_reply_to.is_some() {
                "reply: included"
            } else {
                "reply: none"
            };
            let validation_context = pending
                .validation_error
                .as_deref()
                .map(|reason| {
                    format!("validation: {reason}; recover with Replace, Discard, or Close")
                })
                .unwrap_or_else(|| "validation: ready".to_string());
            self.view
                .label(
                    cx,
                    ids!(
                        telegram_attachment_picker
                            .attachment_review_preview
                            .attachment_review_title
                    ),
                )
                .set_text(cx, &format!("Review {} before Send", pending.kind.label()));
            self.view
                .label(
                    cx,
                    ids!(
                        telegram_attachment_picker
                            .attachment_review_preview
                            .attachment_review_filename
                    ),
                )
                .set_text(cx, &pending.filename);
            self.view
                .label(
                    cx,
                    ids!(
                        telegram_attachment_picker
                            .attachment_review_preview
                            .attachment_review_metadata
                    ),
                )
                .set_text(
                    cx,
                    &pending_attachment_image_metadata_label(pending)
                        .or_else(|| pending_attachment_audio_metadata_label(pending))
                        .unwrap_or_else(|| {
                            format!(
                                "MIME: {} | ext: {} | size: {}",
                                pending.mime_type,
                                pending.file_extension,
                                format_attachment_file_size(pending.file_size_bytes)
                            )
                        }),
                );
            let local_metadata_label = match pending.kind {
                AttachmentHandoffKind::Photo => ATTACHMENT_SELECTED_IMAGE_METADATA_LABEL,
                AttachmentHandoffKind::Voice => VOICE_MESSAGE_SELECTED_AUDIO_METADATA_LABEL,
                AttachmentHandoffKind::File => {
                    "Selected-file metadata stays local until review Send."
                }
            };
            self.view
                .label(
                    cx,
                    ids!(
                        telegram_attachment_picker
                            .attachment_review_preview
                            .attachment_review_context
                    ),
                )
                .set_text(
                    cx,
                    &format!(
                        "{} | {} | {} | local preview only until Send. {}",
                        pending.caption_preview,
                        reply_context,
                        validation_context,
                        local_metadata_label
                    ),
                );
        } else {
            self.view
                .label(
                    cx,
                    ids!(
                        telegram_attachment_picker
                            .attachment_review_preview
                            .attachment_review_title
                    ),
                )
                .set_text(cx, "No selected attachment");
            self.view
                .label(
                    cx,
                    ids!(
                        telegram_attachment_picker
                            .attachment_review_preview
                            .attachment_review_filename
                    ),
                )
                .set_text(
                    cx,
                    "Pick Photo or File to review filename and MIME before send.",
                );
            self.view
                .label(
                    cx,
                    ids!(
                        telegram_attachment_picker
                            .attachment_review_preview
                            .attachment_review_metadata
                    ),
                )
                .set_text(
                    cx,
                    "Local metadata appears after picker selection; no upload or media decode.",
                );
            self.view
                .label(
                    cx,
                    ids!(
                        telegram_attachment_picker
                            .attachment_review_preview
                            .attachment_review_context
                    ),
                )
                .set_text(
                    cx,
                    "Caption/reply stays in the composer until a pending review Send consumes it.",
                );
        }
    }

    fn set_telegram_attachment_picker_visible(&mut self, cx: &mut Cx, visible: bool) {
        if self.telegram_attachment_picker_visible == visible {
            return;
        }
        self.telegram_attachment_picker_visible = visible;
        self.view
            .view(cx, ids!(telegram_attachment_picker))
            .set_visible(cx, visible);
        self.redraw(cx);
    }

    fn show_telegram_attachment_picker(&mut self, cx: &mut Cx) {
        self.set_telegram_emoji_sticker_panel_visible(cx, false);
        self.set_telegram_voice_message_panel_visible(cx, false);
        self.update_telegram_attachment_picker(cx);
        self.set_telegram_attachment_picker_visible(cx, true);
    }

    fn stage_telegram_attachment_choice(&mut self, cx: &mut Cx, label: &str) {
        self.telegram_pending_attachment_send = None;
        self.telegram_attachment_send_retry_attempt = None;
        self.telegram_attachment_local_status = if matches!(label, "Camera" | "Contact") {
            format!("{label} attachment placeholder staged locally")
        } else {
            format!("{label} attachment preview staged locally")
        };
        self.update_telegram_attachment_picker(cx);
        self.set_telegram_attachment_picker_visible(cx, true);
        enqueue_popup_notification(
            format!(
                "{label} attachment was staged in the local Telegram composer preview. No permission prompt, native picker, capture/contact read, payload, upload, or Matrix send was started."
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_attachment_mobile_picker_control(&mut self, cx: &mut Cx, control: &str) {
        let control = control.trim();
        if control.is_empty() {
            return;
        }
        let pending_review = self
            .telegram_pending_attachment_send
            .as_ref()
            .map(|pending| format!("{} {}", pending.kind.label(), pending.filename));
        let label = attachment_mobile_picker_controls_label(
            control,
            pending_review.as_deref(),
            self.telegram_attachment_local_status.as_str(),
        );
        self.telegram_attachment_local_status =
            format!("Mobile attachment {control} control stayed local");
        self.update_telegram_attachment_picker(cx);
        self.set_telegram_attachment_picker_visible(cx, true);
        enqueue_popup_notification(
            format!(
                "Attachment mobile {control} stayed local. No camera/photos/files/contacts permission, picker, capture, contact or shared-media read, thumbnail decode, system share sheet, share extension, payload, upload, SendAttachment, SendMessage, SDK queue mutation, gateway/runtime/auth, or live mutation was emitted. {label}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn open_telegram_attachment_handoff_confirmation(
        &mut self,
        cx: &mut Cx,
        kind: AttachmentHandoffKind,
        timeline_kind: TimelineKind,
        in_reply_to: Option<OwnedEventId>,
    ) {
        let label = kind.label();
        self.telegram_attachment_local_status =
            format!("{label} attachment send waiting for confirmation");
        self.update_telegram_attachment_picker(cx);
        self.set_telegram_attachment_picker_visible(cx, true);
        let voice_lifecycle_note = (kind == AttachmentHandoffKind::Voice)
            .then(|| {
                format!(
                    " {}",
                    self.current_voice_lifecycle_metadata_label(
                        "send confirmation opened",
                        "confirmation opened before desktop audio picker"
                    )
                )
            })
            .unwrap_or_default();
        let content = ConfirmationModalContent {
            title_text: format!("Send {label} attachment").into(),
            body_text: format!("{label} attachments open the native desktop picker after this confirmation. Choosing a file stages local review first; only the review row Send button submits MatrixRequest::SendAttachment through the Matrix attachment send queue. Cancel sends no upload or Matrix media request.{voice_lifecycle_note}").into(),
            accept_button_text: Some("Choose File".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |cx| {
                cx.action(RoomInputBarAction::AttachmentHandoffConfirmed {
                    kind,
                    timeline_kind: timeline_kind.clone(),
                    in_reply_to: in_reply_to.clone(),
                });
            })),
            on_cancel_clicked: Some(Box::new(move |cx| {
                cx.action(RoomInputBarAction::AttachmentHandoffCanceled { kind });
                enqueue_popup_notification(
                    format!(
                        "{label} attachment send canceled before picker. No upload or Matrix media send was started."
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
        };
        enqueue_popup_notification(
            format!(
                "{label} attachment send confirmation opened. No native picker, upload, or Matrix media send was started before confirmation.{voice_lifecycle_note}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
    }

    fn current_emoji_sticker_lifecycle_metadata_label(&self, action: &str) -> String {
        let resolved_action = if action.trim().is_empty() {
            self.telegram_emoji_sticker_last_lifecycle_action.as_str()
        } else {
            action
        };
        emoji_sticker_lifecycle_metadata_label(
            resolved_action,
            self.telegram_emoji_sticker_panel_visible,
            self.telegram_emoji_sticker_last_choice.as_deref(),
            self.telegram_emoji_sticker_stage_count,
            Some(self.telegram_emoji_sticker_local_status.as_str()),
        )
    }

    fn update_telegram_emoji_sticker_panel(&mut self, cx: &mut Cx) {
        let status = if self.telegram_emoji_sticker_local_status.trim().is_empty() {
            "Choose Smile, Thumbs, Heart, or Sticker to stage a local-only emoji/sticker preview"
        } else {
            self.telegram_emoji_sticker_local_status.as_str()
        };
        self.view
            .label(cx, ids!(telegram_emoji_sticker_panel.emoji_summary))
            .set_text(
                cx,
                &format!("{status}. {EMOJI_STICKER_SEND_LOCAL_BOUNDARY_LABEL}"),
            );
        let lifecycle_metadata =
            self.current_emoji_sticker_lifecycle_metadata_label("panel update");
        self.view
            .label(
                cx,
                ids!(telegram_emoji_sticker_panel.emoji_lifecycle_metadata),
            )
            .set_text(cx, &lifecycle_metadata);
    }

    fn set_telegram_emoji_sticker_panel_visible(&mut self, cx: &mut Cx, visible: bool) {
        if self.telegram_emoji_sticker_panel_visible == visible {
            return;
        }
        self.telegram_emoji_sticker_panel_visible = visible;
        self.view
            .view(cx, ids!(telegram_emoji_sticker_panel))
            .set_visible(cx, visible);
        self.redraw(cx);
    }

    fn show_telegram_emoji_sticker_panel(&mut self, cx: &mut Cx) {
        self.set_telegram_attachment_picker_visible(cx, false);
        self.set_telegram_voice_message_panel_visible(cx, false);
        self.telegram_emoji_sticker_last_lifecycle_action =
            if self.telegram_emoji_sticker_stage_count == 0 {
                "opened"
            } else {
                "reopened"
            }
            .to_string();
        self.set_telegram_emoji_sticker_panel_visible(cx, true);
        self.update_telegram_emoji_sticker_panel(cx);
    }

    fn stage_telegram_emoji_sticker_choice(&mut self, cx: &mut Cx, label: &str) {
        self.telegram_emoji_sticker_last_choice = Some(label.to_string());
        self.telegram_emoji_sticker_stage_count =
            self.telegram_emoji_sticker_stage_count.saturating_add(1);
        self.telegram_emoji_sticker_last_lifecycle_action = format!("staged {label}");
        self.telegram_emoji_sticker_local_status =
            format!("{label} emoji/sticker preview staged locally");
        self.set_telegram_emoji_sticker_panel_visible(cx, true);
        self.update_telegram_emoji_sticker_panel(cx);
        let lifecycle_metadata = self.current_emoji_sticker_lifecycle_metadata_label("");
        enqueue_popup_notification(
            format!(
                "{label} emoji/sticker preview was staged in the local Telegram composer preview. {EMOJI_STICKER_SEND_LOCAL_BOUNDARY_LABEL} {lifecycle_metadata}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn current_voice_lifecycle_metadata_label(&self, action: &str, picker_state: &str) -> String {
        let pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .filter(|pending| pending.kind == AttachmentHandoffKind::Voice);
        voice_message_lifecycle_metadata_label(
            action,
            self.telegram_voice_message_panel_visible,
            Some(self.telegram_voice_local_status.as_str()),
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            pending_voice
                .map(|pending| pending.in_reply_to.is_some())
                .unwrap_or_else(|| self.replying_to.is_some()),
            picker_state,
        )
    }

    fn update_telegram_voice_message_panel(&mut self, cx: &mut Cx) {
        let status = if self.telegram_voice_local_status.trim().is_empty() {
            "Use Send to choose a desktop audio file for review; Record and Lock stay local"
        } else {
            self.telegram_voice_local_status.as_str()
        };
        let pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .filter(|pending| pending.kind == AttachmentHandoffKind::Voice);
        let lifecycle_metadata =
            self.current_voice_lifecycle_metadata_label("panel update", "status repaint only");
        let recorder_status_metadata = voice_message_recorder_status_controls_label(
            self.telegram_voice_recorder_last_control.as_deref(),
            self.telegram_voice_message_panel_visible,
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            pending_voice.and_then(|pending| pending.audio_waveform_codec_label.as_deref()),
        );
        let capture_lifecycle_metadata = voice_message_capture_lifecycle_controls_label(
            self.telegram_voice_capture_lifecycle_last_control
                .as_deref(),
            self.telegram_voice_message_panel_visible,
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
        );
        let mobile_picker_metadata = voice_message_mobile_picker_controls_label(
            self.telegram_voice_mobile_picker_last_control.as_deref(),
            self.telegram_voice_message_panel_visible,
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            status,
        );
        let review_playback_metadata = voice_message_review_playback_controls_label(
            self.telegram_voice_review_playback_last_control.as_deref(),
            self.telegram_voice_message_panel_visible,
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            status,
        );
        let voice_preflight_control = voice_message_send_preflight_control_from_status(
            self.telegram_voice_local_status.as_str(),
        );
        let voice_preflight_source = if self.telegram_voice_send_preflight_detail.trim().is_empty()
        {
            VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
        } else {
            self.telegram_voice_send_preflight_detail.as_str()
        };
        let voice_preflight_detail = voice_message_send_preflight_detail_controls_label(
            voice_preflight_control,
            self.telegram_voice_message_panel_visible,
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            self.telegram_attachment_send_retry_attempt
                .as_ref()
                .map(|attempt| attempt.kind == AttachmentHandoffKind::Voice)
                .unwrap_or(false),
            status,
            self.telegram_attachment_send_cached_error.as_deref(),
            voice_preflight_source,
        );
        self.telegram_voice_send_preflight_detail = voice_preflight_detail.clone();
        self.view
            .label(
                cx,
                ids!(
                    telegram_voice_message_panel
                        .voice_preview
                        .voice_preview_mode
                ),
            )
            .set_text(cx, &recorder_status_metadata);
        self.view
            .label(
                cx,
                ids!(telegram_voice_message_panel.voice_recorder_status_metadata),
            )
            .set_text(cx, &recorder_status_metadata);
        self.view
            .label(
                cx,
                ids!(telegram_voice_message_panel.voice_capture_lifecycle_metadata),
            )
            .set_text(cx, &capture_lifecycle_metadata);
        self.view
            .label(
                cx,
                ids!(telegram_voice_message_panel.voice_mobile_picker_metadata),
            )
            .set_text(cx, &mobile_picker_metadata);
        self.view
            .label(
                cx,
                ids!(telegram_voice_message_panel.voice_review_playback_metadata),
            )
            .set_text(cx, &review_playback_metadata);
        self.view
            .label(
                cx,
                ids!(telegram_voice_message_panel.voice_send_preflight_detail_metadata),
            )
            .set_text(cx, &voice_preflight_detail);
        self.view
            .label(cx, ids!(telegram_voice_message_panel.voice_summary))
            .set_text(
                cx,
                &format!(
                    "{status}. {VOICE_MESSAGE_PERMISSION_RECORDING_LOCAL_BOUNDARY_LABEL} {} {capture_lifecycle_metadata} {mobile_picker_metadata} {review_playback_metadata} {lifecycle_metadata}",
                    voice_message_recorder_waveform_codec_boundary_label(
                        "panel update",
                        self.telegram_voice_message_panel_visible,
                        None,
                    )
                ),
            );
        self.view
            .label(cx, ids!(telegram_voice_message_panel.voice_option_evidence))
            .set_text(
                cx,
                &format!(
                    "Voice Send reuses the confirmed desktop file picker and attachment review row for local audio files. Selected audio review shows filename, MIME, extension, size, duration, codec/container status, and bounded WAV waveform peaks before SendAttachment. Confirmation cancel repaints only local voice/picker status. Play can open the pending desktop audio review through the system opener; Record, Lock, Cancel, Permission, Capture, Encode, Review, Upload, Packet, Contract, Taxonomy, Mic, Files, Library, Retake, Share, Pause, Scrub, Speed, and Close stay local; Packet records recorder lifecycle acceptance criteria, Contract maps typed recorder/upload contracts, Taxonomy records recorder result slots, and Drop only clears pending voice review state. No microphone permission, mobile picker, share sheet, recorder, inline player, recorder waveform capture, encoder, codec conversion, transcription, upload progress, text fallback, room-state, membership, or live mutation is requested. {VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_LABEL} {VOICE_MESSAGE_LIFECYCLE_METADATA_LABEL}"
                ),
            );
        self.view
            .label(
                cx,
                ids!(telegram_voice_message_panel.voice_send_blocked_evidence),
            )
            .set_text(
                cx,
                &format!(
                    "{VOICE_MESSAGE_SEND_LOCAL_BLOCKED_LABEL} {VOICE_MESSAGE_SELECTED_AUDIO_METADATA_LABEL} {VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_LABEL} {VOICE_MESSAGE_LIFECYCLE_METADATA_LABEL} {VOICE_MESSAGE_CONFIRMATION_CANCEL_METADATA_LABEL} {VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_LABEL} {VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_LABEL} {VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_LABEL} {VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_LABEL} {VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_LABEL} {VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_LABEL} {VOICE_MESSAGE_MOBILE_PICKER_CONTROLS_LABEL} {VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_LABEL} {VOICE_MESSAGE_REVIEW_DROP_PENDING_AUDIO_LABEL} {VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_LABEL}"
                ),
            );
    }

    fn set_telegram_voice_message_panel_visible(&mut self, cx: &mut Cx, visible: bool) {
        if self.telegram_voice_message_panel_visible == visible {
            return;
        }
        self.telegram_voice_message_panel_visible = visible;
        self.view
            .view(cx, ids!(telegram_voice_message_panel))
            .set_visible(cx, visible);
        self.redraw(cx);
    }

    fn show_telegram_voice_message_panel(&mut self, cx: &mut Cx) {
        self.set_telegram_attachment_picker_visible(cx, false);
        self.set_telegram_emoji_sticker_panel_visible(cx, false);
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
    }

    fn stage_telegram_voice_choice(&mut self, cx: &mut Cx, label: &str) {
        self.telegram_voice_local_status = match label {
            "Record" => "Record control stayed local without microphone permission".to_string(),
            "Lock" => "Lock control stayed local without starting hands-free recording".to_string(),
            "Send" => "Send opens a confirmed desktop audio-file picker; mic capture stays local"
                .to_string(),
            _ => format!("{label} control staged a local voice preview"),
        };
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
        let lifecycle_metadata =
            self.current_voice_lifecycle_metadata_label(label, "local control staged");
        enqueue_popup_notification(
            format!(
                "{label} voice control was staged in the local Telegram composer preview. {VOICE_MESSAGE_PERMISSION_RECORDING_LOCAL_BOUNDARY_LABEL} {} {lifecycle_metadata}",
                voice_message_recorder_waveform_codec_boundary_label(
                    label,
                    self.telegram_voice_message_panel_visible,
                    None,
                )
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_voice_recorder_status_control(&mut self, cx: &mut Cx, control: &str) {
        let control = control.trim();
        if control.is_empty() {
            return;
        }
        self.telegram_voice_recorder_last_control = Some(control.to_string());
        let pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .filter(|pending| pending.kind == AttachmentHandoffKind::Voice);
        let pending_voice_filename = pending_voice.map(|pending| pending.filename.clone());
        let pending_voice_duration =
            pending_voice.and_then(|pending| pending.audio_duration_label.clone());
        let pending_voice_waveform_codec =
            pending_voice.and_then(|pending| pending.audio_waveform_codec_label.clone());
        self.telegram_voice_local_status = if matches!(control, "Waveform" | "Codec") {
            if let Some(filename) = pending_voice_filename.as_deref() {
                let analysis = pending_voice_waveform_codec
                    .as_deref()
                    .unwrap_or("selected-audio waveform/codec unavailable");
                format!("{control} selected-audio analysis stayed local: {filename}; {analysis}")
            } else {
                format!(
                    "{control} recorder status stayed local; no pending selected audio analysis"
                )
            }
        } else {
            format!(
                "{control} recorder status stayed local without mic permission, recording, transcription, upload progress, or codec work"
            )
        };
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
        let recorder_status_metadata = voice_message_recorder_status_controls_label(
            Some(control),
            self.telegram_voice_message_panel_visible,
            pending_voice_filename.as_deref(),
            pending_voice_duration.as_deref(),
            pending_voice_waveform_codec.as_deref(),
        );
        enqueue_popup_notification(
            format!(
                "{control} recorder status control stayed local. Waveform/Codec can read only capped bytes from the already selected desktop audio review when present; no microphone permission, audio session, recorder, recorder waveform sampling, transcription service, upload progress subscription, SendAttachment, gateway/runtime/auth, or live mutation was emitted. {recorder_status_metadata}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_voice_capture_lifecycle_control(&mut self, cx: &mut Cx, control: &str) {
        let control = control.trim();
        if control.is_empty() {
            return;
        }
        self.telegram_voice_capture_lifecycle_last_control = Some(control.to_string());
        self.telegram_voice_local_status = format!(
            "{control} capture lifecycle stayed local without mic permission, recording, encoding, upload, or SendAttachment"
        );
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
        let pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .filter(|pending| pending.kind == AttachmentHandoffKind::Voice);
        let capture_lifecycle_metadata = if control.eq_ignore_ascii_case("Packet") {
            let source_copy = if self.telegram_voice_send_preflight_detail.trim().is_empty() {
                VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_EVIDENCE
            } else {
                self.telegram_voice_send_preflight_detail.as_str()
            };
            voice_message_recorder_lifecycle_drilldown_packet_label(
                self.telegram_voice_message_panel_visible,
                pending_voice.map(|pending| pending.filename.as_str()),
                pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
                self.telegram_attachment_send_retry_attempt
                    .as_ref()
                    .map(|attempt| attempt.kind == AttachmentHandoffKind::Voice)
                    .unwrap_or(false),
                self.telegram_voice_local_status.as_str(),
                self.telegram_attachment_send_cached_error.as_deref(),
                source_copy,
            )
        } else if control.eq_ignore_ascii_case("Contract") {
            let source_copy = if self.telegram_voice_send_preflight_detail.trim().is_empty() {
                VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE
            } else {
                self.telegram_voice_send_preflight_detail.as_str()
            };
            voice_message_recorder_typed_contract_packet_label(
                self.telegram_voice_message_panel_visible,
                pending_voice.map(|pending| pending.filename.as_str()),
                pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
                self.telegram_attachment_send_retry_attempt
                    .as_ref()
                    .map(|attempt| attempt.kind == AttachmentHandoffKind::Voice)
                    .unwrap_or(false),
                self.telegram_voice_local_status.as_str(),
                self.telegram_attachment_send_cached_error.as_deref(),
                source_copy,
            )
        } else if control.eq_ignore_ascii_case("Taxonomy") {
            let source_copy = if self.telegram_voice_send_preflight_detail.trim().is_empty() {
                VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE
            } else {
                self.telegram_voice_send_preflight_detail.as_str()
            };
            voice_message_recorder_result_taxonomy_packet_label(
                self.telegram_voice_message_panel_visible,
                pending_voice.map(|pending| pending.filename.as_str()),
                pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
                self.telegram_attachment_send_retry_attempt
                    .as_ref()
                    .map(|attempt| attempt.kind == AttachmentHandoffKind::Voice)
                    .unwrap_or(false),
                self.telegram_voice_local_status.as_str(),
                self.telegram_attachment_send_cached_error.as_deref(),
                source_copy,
            )
        } else {
            voice_message_capture_lifecycle_controls_label(
                Some(control),
                self.telegram_voice_message_panel_visible,
                pending_voice.map(|pending| pending.filename.as_str()),
                pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            )
        };
        enqueue_popup_notification(
            format!(
                "Voice {control} capture lifecycle stayed local. No microphone permission, audio session, platform recorder, captured file, waveform sampling, codec conversion, upload progress subscription, SendAttachment, SendMessage fallback, gateway/runtime/auth, or live mutation was emitted. {capture_lifecycle_metadata}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_voice_mobile_picker_control(&mut self, cx: &mut Cx, control: &str) {
        let control = control.trim();
        if control.is_empty() {
            return;
        }
        let pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .filter(|pending| pending.kind == AttachmentHandoffKind::Voice);
        let latest_status = if self.telegram_voice_local_status.trim().is_empty() {
            "voice mobile picker local"
        } else {
            self.telegram_voice_local_status.as_str()
        };
        let mobile_picker_metadata = voice_message_mobile_picker_controls_label(
            Some(control),
            self.telegram_voice_message_panel_visible,
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            latest_status,
        );
        self.telegram_voice_mobile_picker_last_control = Some(control.to_string());
        self.telegram_voice_local_status = format!(
            "{control} mobile voice picker stayed local without mobile permission, picker, capture, share sheet, upload, or SendAttachment"
        );
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
        enqueue_popup_notification(
            format!(
                "Voice mobile {control} picker control stayed local. No mobile microphone permission, document picker, library picker, capture session, retake deletion, share sheet, SendAttachment, SendMessage fallback, gateway/runtime/auth, or live mutation was emitted. {mobile_picker_metadata}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_voice_review_playback_control(&mut self, cx: &mut Cx, control: &str) {
        let control = control.trim();
        if control.is_empty() {
            return;
        }
        if control.eq_ignore_ascii_case("Drop") {
            self.drop_telegram_voice_review_audio(cx);
            return;
        }
        if control.eq_ignore_ascii_case("Play") {
            self.play_telegram_voice_review_audio(cx);
            return;
        }
        let pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .filter(|pending| pending.kind == AttachmentHandoffKind::Voice);
        let latest_status = if self.telegram_voice_local_status.trim().is_empty() {
            "voice review playback local"
        } else {
            self.telegram_voice_local_status.as_str()
        };
        let review_playback_metadata = voice_message_review_playback_controls_label(
            Some(control),
            self.telegram_voice_message_panel_visible,
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            latest_status,
        );
        self.telegram_voice_review_playback_last_control = Some(control.to_string());
        self.telegram_voice_local_status = format!(
            "{control} review playback stayed local without player, decode, scrubber, deletion, upload, or SendAttachment"
        );
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
        enqueue_popup_notification(
            format!(
                "Voice review {control} stayed local. No inline audio player, media decode, waveform sampling, playback subscription, speed transform, scrubber timeline, local file deletion, SendAttachment, SendMessage fallback, gateway/runtime/auth, or live mutation was emitted. Play is the only review control that opens the pending local audio file with the system opener; Drop clears pending voice review state. {review_playback_metadata}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn play_telegram_voice_review_audio(&mut self, cx: &mut Cx) {
        let pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .filter(|pending| pending.kind == AttachmentHandoffKind::Voice)
            .cloned();
        let Some(pending) = pending_voice else {
            let metadata = voice_message_review_playback_open_result_label(
                "",
                None,
                "unavailable; no pending Voice attachment review is loaded",
            );
            self.telegram_voice_review_playback_last_control = Some("Play".to_string());
            self.telegram_voice_local_status =
                "Play needs a pending desktop audio review before opener handoff".to_string();
            self.update_telegram_voice_message_panel(cx);
            self.set_telegram_voice_message_panel_visible(cx, true);
            self.set_message_send_operation_status(
                cx,
                "voice-review-play-empty",
                "Voice Play held locally",
                &metadata,
            );
            enqueue_popup_notification(metadata, PopupKind::Warning, Some(4.0));
            return;
        };

        let filename = pending.filename.clone();
        let duration_label = pending.audio_duration_label.clone();
        let open_result = open_voice_review_audio_file(&pending.file_path);
        self.telegram_voice_review_playback_last_control = Some("Play".to_string());
        let result_state = match &open_result {
            Ok(()) => {
                self.telegram_voice_local_status = format!(
                    "Play opened pending voice review audio with system opener: {filename}"
                );
                "opened with system opener".to_string()
            }
            Err(error) => {
                self.telegram_voice_local_status =
                    format!("Play could not open pending voice review audio: {filename}; {error}");
                format!("failed: {error}")
            }
        };
        let metadata = voice_message_review_playback_open_result_label(
            &filename,
            duration_label.as_deref(),
            &result_state,
        );
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
        self.set_message_send_operation_status(
            cx,
            if open_result.is_ok() {
                "voice-review-opened-local"
            } else {
                "voice-review-open-failed"
            },
            if open_result.is_ok() {
                "Voice review Play opened locally"
            } else {
                "Voice review Play failed locally"
            },
            &metadata,
        );
        enqueue_popup_notification(
            metadata,
            if open_result.is_ok() {
                PopupKind::Info
            } else {
                PopupKind::Warning
            },
            Some(4.0),
        );
    }

    fn drop_telegram_voice_review_audio(&mut self, cx: &mut Cx) {
        let has_pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .map(|pending| pending.kind == AttachmentHandoffKind::Voice)
            .unwrap_or(false);
        let dropped_voice = if has_pending_voice {
            self.telegram_pending_attachment_send.take()
        } else {
            None
        };
        let retry_cache_cleared = self
            .telegram_attachment_send_retry_attempt
            .as_ref()
            .map(|attempt| attempt.kind == AttachmentHandoffKind::Voice)
            .unwrap_or(false);
        if retry_cache_cleared {
            self.telegram_attachment_send_retry_attempt = None;
            self.telegram_attachment_send_cached_error = None;
        }

        let drop_label = voice_message_review_drop_pending_audio_label(
            dropped_voice
                .as_ref()
                .map(|pending| pending.filename.as_str()),
            dropped_voice
                .as_ref()
                .and_then(|pending| pending.audio_duration_label.as_deref()),
            retry_cache_cleared,
        );
        self.telegram_voice_review_playback_last_control = Some("Drop".to_string());
        if let Some(pending) = dropped_voice.as_ref() {
            let duration_note = pending
                .audio_duration_label
                .as_deref()
                .unwrap_or("duration unavailable");
            self.telegram_voice_local_status = format!(
                "Drop cleared pending voice review locally: {} ({duration_note})",
                pending.filename
            );
            self.telegram_attachment_local_status = format!(
                "Voice attachment review dropped locally: {}; no local file deletion or upload",
                pending.filename
            );
            self.set_message_send_operation_status(
                cx,
                "discarded-local",
                "Voice review dropped locally",
                &format!(
                    "Voice review Drop consumed the pending selected audio review with Option::take() while preserving composer caption/reply text. Repeated review-row Send now has no pending voice attachment to submit. No local file was deleted, no MatrixRequest::SendAttachment or caption-only SendMessage was emitted, and no SDK send-queue cancel, room-state, membership, account/profile, gateway/runtime/auth, or live mutation request was sent. {drop_label}"
                ),
            );
        } else {
            self.telegram_voice_local_status =
                "Drop found no pending voice review audio and stayed local".to_string();
            self.set_message_send_operation_status(
                cx,
                "empty-held",
                "Voice Drop held locally",
                &format!(
                    "Voice review Drop found no pending Voice attachment review to clear. Existing Photo/File pending review state, if any, was left untouched. No local file deletion, SendAttachment, SendMessage fallback, SDK queue cancel, gateway/runtime/auth, or live mutation request was emitted. {drop_label}"
                ),
            );
        }

        self.update_telegram_attachment_picker(cx);
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
        enqueue_popup_notification(
            format!("Voice review Drop completed as local cleanup. {drop_label}"),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_voice_send_preflight_detail_control(&mut self, cx: &mut Cx, control: &str) {
        let control = control.trim();
        if control.is_empty() {
            return;
        }
        let pending_voice = self
            .telegram_pending_attachment_send
            .as_ref()
            .filter(|pending| pending.kind == AttachmentHandoffKind::Voice);
        let status = format!("voice-send-preflight-{control}-local").to_ascii_lowercase();
        let latest_status = if self.telegram_voice_local_status.trim().is_empty() {
            status.as_str()
        } else {
            self.telegram_voice_local_status.as_str()
        };
        let source_copy = if self.telegram_voice_send_preflight_detail.trim().is_empty() {
            VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
        } else {
            self.telegram_voice_send_preflight_detail.as_str()
        };
        let label = voice_message_send_preflight_detail_controls_label(
            control,
            self.telegram_voice_message_panel_visible,
            pending_voice.map(|pending| pending.filename.as_str()),
            pending_voice.and_then(|pending| pending.audio_duration_label.as_deref()),
            self.telegram_attachment_send_retry_attempt
                .as_ref()
                .map(|attempt| attempt.kind == AttachmentHandoffKind::Voice)
                .unwrap_or(false),
            latest_status,
            self.telegram_attachment_send_cached_error.as_deref(),
            source_copy,
        );
        self.telegram_voice_send_preflight_detail = label.clone();
        self.telegram_voice_local_status =
            format!("Voice Send preflight {control} detail stayed local");
        self.update_telegram_voice_message_panel(cx);
        self.set_telegram_voice_message_panel_visible(cx, true);
        enqueue_popup_notification(
            format!(
                "Voice Send {control} detail stayed local. No microphone permission, recorder, captured audio file, extra SendAttachment, unconfirmed retry, SendMessage fallback, gateway/runtime/auth, or live mutation was emitted. {label}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn replied_to_event_id(&self) -> Option<OwnedEventId> {
        self.replying_to
            .as_ref()
            .and_then(|(event_tl_item, _)| event_tl_item.event_id().map(ToOwned::to_owned))
    }

    /// Updates (populates and shows or hides) this room's tombstone footer
    /// based on the given successor room details.
    fn update_tombstone_footer(
        &mut self,
        cx: &mut Cx,
        tombstoned_room_id: &OwnedRoomId,
        successor_room_details: Option<&SuccessorRoomDetails>,
    ) {
        let tombstone_footer = self.tombstone_footer(cx, ids!(tombstone_footer));
        let input_bar = self.view(cx, ids!(input_bar));

        if let Some(srd) = successor_room_details {
            tombstone_footer.show(cx, tombstoned_room_id, srd);
            input_bar.set_visible(cx, false);
            self.set_telegram_attachment_picker_visible(cx, false);
            self.set_telegram_emoji_sticker_panel_visible(cx, false);
            self.set_telegram_voice_message_panel_visible(cx, false);
        } else {
            tombstone_footer.hide(cx);
            input_bar.set_visible(cx, true);
        }
    }

    fn set_message_send_operation_status(
        &mut self,
        cx: &mut Cx,
        status: &str,
        title: &str,
        evidence: &str,
    ) {
        let pending_review = self
            .telegram_pending_attachment_send
            .as_ref()
            .map(|pending| format!("{} {}", pending.kind.label(), pending.filename));
        let multi_file_queue_boundary = attachment_multi_file_queue_boundary_label(
            pending_review.as_deref(),
            self.telegram_attachment_send_retry_attempt.is_some(),
        );
        let per_file_status_controls = attachment_per_file_status_controls_label(
            "Status",
            pending_review.as_deref(),
            self.telegram_attachment_send_retry_attempt.is_some(),
            status,
        );
        let timeline_cancel_bridge = attachment_accepted_queue_timeline_cancel_bridge_label(
            "Status",
            pending_review.as_deref(),
            self.telegram_attachment_send_retry_attempt.is_some(),
            status,
        );
        let send_preflight_control = attachment_send_preflight_control_from_status(status);
        let send_preflight_detail = attachment_send_preflight_detail_controls_label(
            send_preflight_control,
            pending_review.as_deref(),
            self.telegram_attachment_send_retry_attempt.is_some(),
            status,
            self.telegram_attachment_send_cached_error.as_deref(),
            evidence,
        );
        self.telegram_attachment_send_preflight_detail = send_preflight_detail.clone();
        // Message send operation evidence: this only updates local status labels after
        // existing SendMessage/SendAttachment submit paths or local Retry/Cancel clicks.
        self.view
            .label(
                cx,
                ids!(send_operation_status.status_actions.queue_status_label),
            )
            .set_text(cx, status);
        self.view
            .label(cx, ids!(send_operation_status.title))
            .set_text(cx, title);
        self.view
            .label(cx, ids!(send_operation_status.evidence))
            .set_text(cx, evidence);
        self.view
            .label(cx, ids!(send_operation_status.result_bridge))
            .set_text(cx, ATTACHMENT_TRUE_QUEUE_CONTROL_LOCAL_BOUNDARY_LABEL);
        self.view
            .label(cx, ids!(send_operation_status.taxonomy))
            .set_text(cx, ATTACHMENT_STATUS_TAXONOMY_LABEL);
        self.view
            .label(cx, ids!(send_operation_status.compact_fit))
            .set_text(cx, ATTACHMENT_REVIEW_ROW_COMPACT_FIT_LABEL);
        self.view
            .label(cx, ids!(send_operation_status.action_density))
            .set_text(cx, ATTACHMENT_MOBILE_ACTION_DENSITY_LABEL);
        self.view
            .label(cx, ids!(send_operation_status.multi_file_queue_boundary))
            .set_text(cx, &multi_file_queue_boundary);
        self.view
            .label(cx, ids!(send_operation_status.accepted_queue_actions_label))
            .set_text(cx, ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_LABEL);
        self.view
            .label(
                cx,
                ids!(send_operation_status.accepted_queue_timeline_cancel_bridge_label),
            )
            .set_text(cx, &timeline_cancel_bridge);
        self.view
            .label(
                cx,
                ids!(send_operation_status.per_file_status_controls_label),
            )
            .set_text(cx, &per_file_status_controls);
        self.view
            .label(
                cx,
                ids!(send_operation_status.attachment_send_preflight_detail_controls_label),
            )
            .set_text(cx, &send_preflight_detail);
        self.redraw(cx);
    }

    fn stage_telegram_attachment_timeline_cancel_bridge_control(
        &mut self,
        cx: &mut Cx,
        control: &str,
    ) {
        let pending_review = self
            .telegram_pending_attachment_send
            .as_ref()
            .map(|pending| format!("{} {}", pending.kind.label(), pending.filename));
        let latest_status = if self.telegram_attachment_local_status.trim().is_empty() {
            "local evidence"
        } else {
            self.telegram_attachment_local_status.as_str()
        };
        let label = attachment_accepted_queue_timeline_cancel_bridge_label(
            control,
            pending_review.as_deref(),
            self.telegram_attachment_send_retry_attempt.is_some(),
            latest_status,
        );
        self.telegram_attachment_local_status =
            format!("Accepted queue timeline-cancel {control} bridge stayed local");
        self.set_message_send_operation_status(
            cx,
            &format!("timeline-cancel-{control}-local").to_ascii_lowercase(),
            &format!("Timeline cancel {control} stayed local"),
            &label,
        );
        enqueue_popup_notification(
            format!(
                "Attachment timeline-cancel {control} bridge stayed local. Use the timeline local echo context menu's Cancel Send when a SendHandle exists; no composer SDK queue abort, SendAttachment resubmit, gateway/runtime/auth, or live mutation was emitted."
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_attachment_accepted_queue_action(&mut self, cx: &mut Cx, action: &str) {
        let pending_review = self
            .telegram_pending_attachment_send
            .as_ref()
            .map(|pending| format!("{} {}", pending.kind.label(), pending.filename));
        let retry_cache_ready = self.telegram_attachment_send_retry_attempt.is_some();
        let label = if action.trim().eq_ignore_ascii_case("Background") {
            attachment_accepted_queue_background_snapshot_label(
                pending_review.as_deref(),
                retry_cache_ready,
                &self.telegram_attachment_local_status,
            )
        } else {
            attachment_accepted_queue_actions_row_label(
                action,
                pending_review.as_deref(),
                retry_cache_ready,
            )
        };
        self.telegram_attachment_local_status =
            format!("Accepted SDK queue {action} control stayed local");
        self.set_message_send_operation_status(
            cx,
            &format!("queue-{action}-local").to_ascii_lowercase(),
            &format!("Queue {action} stayed local"),
            &label,
        );
        enqueue_popup_notification(
            format!(
                "Attachment queue {action} stayed local. No SDK queue retry/resume/abort/remove/reorder, SendAttachment resubmit, gateway/runtime/auth, or live mutation was emitted."
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_attachment_per_file_status_control(&mut self, cx: &mut Cx, control: &str) {
        let pending_review = self
            .telegram_pending_attachment_send
            .as_ref()
            .map(|pending| format!("{} {}", pending.kind.label(), pending.filename));
        let status = format!("per-file-{control}-local").to_ascii_lowercase();
        let label = attachment_per_file_status_controls_label(
            control,
            pending_review.as_deref(),
            self.telegram_attachment_send_retry_attempt.is_some(),
            &status,
        );
        self.telegram_attachment_local_status =
            format!("Per-file attachment {control} control stayed local");
        self.set_message_send_operation_status(
            cx,
            &status,
            &format!("Per-file {control} stayed local"),
            &label,
        );
        enqueue_popup_notification(
            format!(
                "Attachment per-file {control} stayed local. No SDK progress subscription, queue pause/resume/cancel/retry, SendAttachment resubmit, gateway/runtime/auth, or live mutation was emitted."
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_telegram_attachment_send_preflight_detail_control(
        &mut self,
        cx: &mut Cx,
        control: &str,
    ) {
        let pending_review = self
            .telegram_pending_attachment_send
            .as_ref()
            .map(|pending| format!("{} {}", pending.kind.label(), pending.filename));
        let status = format!("send-preflight-{control}-local").to_ascii_lowercase();
        let latest_status = if self.telegram_attachment_local_status.trim().is_empty() {
            status.as_str()
        } else {
            self.telegram_attachment_local_status.as_str()
        };
        let source_copy = if self
            .telegram_attachment_send_preflight_detail
            .trim()
            .is_empty()
        {
            ATTACHMENT_SEND_RESULT_BRIDGE_EVIDENCE
        } else {
            self.telegram_attachment_send_preflight_detail.as_str()
        };
        let label = attachment_send_preflight_detail_controls_label(
            control,
            pending_review.as_deref(),
            self.telegram_attachment_send_retry_attempt.is_some(),
            latest_status,
            self.telegram_attachment_send_cached_error.as_deref(),
            source_copy,
        );
        self.telegram_attachment_send_preflight_detail = label.clone();
        self.telegram_attachment_local_status =
            format!("Attachment send preflight {control} detail stayed local");
        self.set_message_send_operation_status(
            cx,
            &status,
            &format!("Attachment {control} detail stayed local"),
            &label,
        );
        enqueue_popup_notification(
            format!(
                "Attachment send {control} detail stayed local. No SendAttachment, SDK queue retry/cancel, upload abort, caption-only SendMessage, gateway/runtime/auth, or live mutation was emitted."
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn handle_attachment_send_result_inner(
        &mut self,
        cx: &mut Cx,
        filename: String,
        result: Result<(), String>,
    ) {
        match result {
            Ok(()) => {
                if self
                    .telegram_attachment_send_retry_attempt
                    .as_ref()
                    .map(|attempt| attempt.filename == filename)
                    .unwrap_or(false)
                {
                    self.telegram_attachment_send_retry_attempt = None;
                }
                self.telegram_attachment_send_cached_error = None;
                self.telegram_attachment_local_status =
                    format!("Attachment queued in Matrix send queue: {filename}");
                self.set_message_send_operation_status(
                    cx,
                    "queued-confirmed",
                    "Attachment queued by SDK",
                    "The SendAttachment worker confirmed that Timeline::send_attachment().use_send_queue() accepted this attachment into the SDK send queue and the cached failed-handoff Retry attempt for this filename was cleared. This is still not delivery success: RoomScreen renders SDK queue progress/error/sent state on the timeline local echo, while composer Cancel still does not abort or remove SDK queue work and Retry has no cached failure to reuse.",
                );
                enqueue_popup_notification(
                    format!("Attachment queued by Matrix send queue: {filename}."),
                    PopupKind::Success,
                    Some(4.0),
                );
            }
            Err(error) => {
                self.telegram_attachment_send_cached_error = Some(error.clone());
                self.telegram_attachment_local_status =
                    format!("Attachment handoff failed before SDK queue: {filename}");
                self.set_message_send_operation_status(
                    cx,
                    "failure-copy",
                    "Attachment handoff failed",
                    &format!(
                        "The SendAttachment worker returned an immediate handoff failure before SDK queue ownership for {filename}: {error}. Retry now requires PositiveConfirmationModal before reusing the cached last validated SendAttachment handoff; Cancel clears only local retry cache and does not abort or remove SDK queue work. No automatic retry, caption-only SendMessage, SDK queue retry/resume, room-state, membership, gateway/runtime/auth, or live mutation request is emitted."
                    ),
                );
            }
        }
        self.update_telegram_attachment_picker(cx);
    }

    fn handle_local_send_abort_result_inner(&mut self, cx: &mut Cx, result: Result<bool, String>) {
        let status = match &result {
            Ok(true) => "timeline-cancel-canceled",
            Ok(false) => "timeline-cancel-not-cancellable",
            Err(_) => "timeline-cancel-failed",
        };
        let title = match &result {
            Ok(true) => "Timeline local send canceled",
            Ok(false) => "Timeline local send already sent",
            Err(_) => "Timeline local send cancel failed",
        };
        let label = attachment_local_send_abort_result_label(&result);
        self.telegram_attachment_local_status = title.to_string();
        self.set_message_send_operation_status(cx, status, title, &label);
        self.update_telegram_attachment_picker(cx);
    }

    fn set_typing_notice_status(&mut self, cx: &mut Cx, title: &str, evidence: &str) {
        // Typing notice evidence: this only updates local labels around the existing
        // SendTypingNotice path or the local Hepta command preview suppression path.
        self.view
            .label(cx, ids!(typing_notice_status.title))
            .set_text(cx, title);
        self.view
            .label(cx, ids!(typing_notice_status.evidence))
            .set_text(cx, evidence);
        self.redraw(cx);
    }

    /// Sets the send_message_button to be enabled and green, or disabled and gray.
    ///
    /// This should be called to update the button state when the message TextInput content changes.
    fn enable_send_message_button(&mut self, cx: &mut Cx, enable: bool) {
        let mut send_message_button = self.view.button(cx, ids!(send_message_button));
        let (fg_color, bg_color) = if enable {
            (COLOR_FG_ACCEPT_GREEN, COLOR_BG_ACCEPT_GREEN)
        } else {
            (COLOR_FG_DISABLED, COLOR_BG_DISABLED)
        };
        script_apply_eval!(cx, send_message_button, {
            enabled: #(enable),
            draw_icon.color: #(fg_color),
            draw_bg.color: #(bg_color),
        });
    }

    fn update_hepta_command_preview(&mut self, cx: &mut Cx, input: &str) {
        let preview_view = self.view.view(cx, ids!(hepta_command_preview));
        let Some(plan) = plan_hepta_composer_command(input, 0) else {
            preview_view.set_visible(cx, false);
            return;
        };
        let preview = plan.to_bridge_input();
        preview_view.set_visible(cx, true);
        self.view
            .label(cx, ids!(hepta_command_preview.title))
            .set_text(
                cx,
                &format!("Hepta dry-run · m.hepta.{}", preview.event_kind),
            );
        self.view
            .label(cx, ids!(hepta_command_preview.body))
            .set_text(cx, &plan.operator_summary());
        self.view.label(cx, ids!(hepta_command_preview.meta)).set_text(
            cx,
            &format!(
                "preview={} · confirmation={} · external_mutation_enabled=false · Matrix typing notice suppressed",
                preview.id,
                plan.requires_confirmation(),
            ),
        );
    }

    /// Updates the visibility of select views based on the user's new power levels.
    ///
    /// This will show/hide the `input_bar` and the `can_not_send_message_notice` views.
    fn update_user_power_levels(&mut self, cx: &mut Cx, user_power_levels: UserPowerLevels) {
        let can_send = user_power_levels.can_send_message();
        self.view
            .view(cx, ids!(input_bar))
            .set_visible(cx, can_send);
        self.view
            .view(cx, ids!(can_not_send_message_notice))
            .set_visible(cx, !can_send);
        if !can_send {
            self.set_telegram_attachment_picker_visible(cx, false);
            self.set_telegram_emoji_sticker_panel_visible(cx, false);
            self.set_telegram_voice_message_panel_visible(cx, false);
        }
    }

    /// Returns true if the TSP signing checkbox is checked, false otherwise.
    ///
    /// If TSP is not enabled, this will always return false.
    #[cfg(feature = "tsp")]
    fn is_tsp_signing_enabled(&self, cx: &mut Cx) -> bool {
        self.view.check_box(cx, ids!(tsp_sign_checkbox)).active(cx)
    }
}

fn current_time_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis().min(u64::MAX as u128) as u64)
        .unwrap_or_default()
}

impl RoomInputBarRef {
    /// Shows a preview of the given event that the user is currently replying to
    /// above the message input bar.
    pub fn show_replying_to(
        &self,
        cx: &mut Cx,
        replying_to: (EventTimelineItem, EmbeddedEvent),
        timeline_kind: &TimelineKind,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.show_replying_to(cx, replying_to, timeline_kind, true);
    }

    /// Shows the editing pane to allow the user to edit the given event.
    pub fn show_editing_pane(
        &self,
        cx: &mut Cx,
        event_tl_item: EventTimelineItem,
        timeline_kind: TimelineKind,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.show_editing_pane(
            cx,
            ShowEditingPaneBehavior::ShowNew { event_tl_item },
            timeline_kind,
        );
    }

    /// Updates the visibility of select views based on the user's new power levels.
    ///
    /// This will show/hide the `input_bar` and the `can_not_send_message_notice` views.
    pub fn update_user_power_levels(&self, cx: &mut Cx, user_power_levels: UserPowerLevels) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.update_user_power_levels(cx, user_power_levels);
    }

    /// Updates this room's tombstone footer based on the given `tombstone_state`.
    pub fn update_tombstone_footer(
        &self,
        cx: &mut Cx,
        tombstoned_room_id: &OwnedRoomId,
        successor_room_details: Option<&SuccessorRoomDetails>,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.update_tombstone_footer(cx, tombstoned_room_id, successor_room_details);
    }

    /// Forwards the result of an edit request to the `EditingPane` widget
    /// within this `RoomInputBar`.
    pub fn handle_edit_result(
        &self,
        cx: &mut Cx,
        timeline_event_item_id: TimelineEventItemId,
        edit_result: Result<(), matrix_sdk_ui::timeline::Error>,
    ) {
        let Some(inner) = self.borrow_mut() else {
            return;
        };
        inner
            .editing_pane(cx, ids!(editing_pane))
            .handle_edit_result(cx, timeline_event_item_id, edit_result);
    }

    /// Forwards the worker result of an attachment send handoff to the Telegram
    /// operation strip.
    pub fn handle_attachment_send_result(
        &self,
        cx: &mut Cx,
        filename: String,
        result: Result<(), String>,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.handle_attachment_send_result_inner(cx, filename, result);
    }

    /// Forwards the worker result of a timeline local echo cancel request to the
    /// Telegram operation strip.
    pub fn handle_local_send_abort_result(&self, cx: &mut Cx, result: Result<bool, String>) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.handle_local_send_abort_result_inner(cx, result);
    }

    /// Save a snapshot of the UI state of this `RoomInputBar`.
    pub fn save_state(&self) -> RoomInputBarState {
        let Some(inner) = self.borrow() else {
            return Default::default();
        };
        // Clear the location preview. We don't save this state because the
        // current location might change by the next time the user opens this same room.
        inner
            .child_by_path(ids!(location_preview))
            .as_location_preview()
            .clear();
        RoomInputBarState {
            was_replying_preview_visible: inner.was_replying_preview_visible,
            replying_to: inner.replying_to.clone(),
            editing_pane_state: inner
                .child_by_path(ids!(editing_pane))
                .as_editing_pane()
                .save_state(),
            text_input_state: inner
                .child_by_path(ids!(input_bar.mentionable_text_input.text_input))
                .as_text_input()
                .save_state(),
        }
    }

    /// Restore the UI state of this `RoomInputBar` from the given state snapshot.
    pub fn restore_state(
        &self,
        cx: &mut Cx,
        timeline_kind: TimelineKind,
        saved_state: RoomInputBarState,
        user_power_levels: UserPowerLevels,
        tombstone_info: Option<&SuccessorRoomDetails>,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        let RoomInputBarState {
            was_replying_preview_visible,
            text_input_state,
            replying_to,
            editing_pane_state,
        } = saved_state;

        // Note: we do *not* restore the location preview state here; see `save_state()`.
        inner.set_telegram_attachment_picker_visible(cx, false);
        inner.set_telegram_emoji_sticker_panel_visible(cx, false);
        inner.set_telegram_voice_message_panel_visible(cx, false);

        // 0. Update select views based on user power levels from the RoomScreen (the `TimelineUiState`).
        //    This must happen before we restore the state of the `EditingPane`,
        //    because the call to `show_editing_pane()` might re-update the `input_bar`'s visibility.
        inner.update_user_power_levels(cx, user_power_levels);

        // 1. Restore the state of the TextInput within the MentionableTextInput.
        inner
            .text_input(cx, ids!(input_bar.mentionable_text_input.text_input))
            .restore_state(cx, text_input_state);

        // 2. Restore the state of the replying-to preview.
        if let Some(replying_to) = replying_to {
            inner.show_replying_to(cx, replying_to, &timeline_kind, false);
        } else {
            inner.clear_replying_to(cx);
        }
        inner.was_replying_preview_visible = was_replying_preview_visible;

        // 3. Restore the state of the editing pane.
        if let Some(editing_pane_state) = editing_pane_state {
            inner.show_editing_pane(
                cx,
                ShowEditingPaneBehavior::RestoreExisting { editing_pane_state },
                timeline_kind.clone(),
            );
        } else {
            inner
                .editing_pane(cx, ids!(editing_pane))
                .force_reset_hide(cx);
            inner.on_editing_pane_hidden(cx);
        }

        // 4. Restore the state of the tombstone footer.
        //    This depends on the `EditingPane` state, so it must be done after Step 3.
        inner.update_tombstone_footer(cx, timeline_kind.room_id(), tombstone_info);
    }
}

/// The saved UI state of a `RoomInputBar` widget.
#[derive(Default)]
pub struct RoomInputBarState {
    /// Whether or not the `replying_preview` widget was shown.
    was_replying_preview_visible: bool,
    /// The state of the `TextInput` within the `mentionable_text_input`.
    text_input_state: TextInputState,
    /// The event that the user is currently replying to, if any.
    replying_to: Option<(EventTimelineItem, EmbeddedEvent)>,
    /// The state of the `EditingPane`, if any message was being edited.
    editing_pane_state: Option<EditingPaneState>,
}

/// Defines what to do when showing the `EditingPane` from the `RoomInputBar`.
enum ShowEditingPaneBehavior {
    /// Show a new edit session, e.g., when first clicking "edit" on a message.
    ShowNew { event_tl_item: EventTimelineItem },
    /// Restore the state of an `EditingPane` that already existed, e.g., when
    /// reopening a room that had an `EditingPane` open when it was closed.
    RestoreExisting {
        editing_pane_state: EditingPaneState,
    },
}
