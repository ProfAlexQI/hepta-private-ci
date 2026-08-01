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
        upload_progress::UploadProgressViewWidgetRefExt,
    },
    location::init_location_subscriber,
    settings::app_preferences::{AppPreferencesGlobal, AppPreferencesAction},
    shared::{
        avatar::AvatarWidgetRefExt,
        confirmation_modal::ConfirmationModalContent,
        file_upload_modal::{AttachmentUpload, FileUploadAttemptId},
        html_or_plaintext::HtmlOrPlaintextWidgetRefExt,
        mentionable_text_input::MentionableTextInputWidgetExt,
        popup_list::{PopupKind, enqueue_popup_notification},
        styles::*,
    },
    sliding_sync::{MatrixRequest, TimelineKind, UserPowerLevels, submit_async_request},
    utils,
};

mod attachments;
mod emoji_sticker;
mod implementation;
mod voice_message;

use attachments::*;
use emoji_sticker::*;
use voice_message::*;

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

        // Keep the composer visually detached from the room chrome while the
        // governed input/attachment behavior remains in the split modules.
        margin: Inset{left: 10, right: 10, bottom: 10}
        show_bg: true,
        draw_bg +: {
            color: (COLOR_HEPTA_GLASS_STRONG)
            border_radius: (HEPTA_RADIUS_FLOATING)
            border_color: (COLOR_HEPTA_HAIRLINE)
            border_size: 1.0
            shadow_color: (COLOR_HEPTA_SHADOW)
            shadow_radius: 8.0
            shadow_offset: vec2(0.0, 2.0)
        }

        // The top-most element is a preview of the message that the user is replying to, if any.
        replying_preview := ReplyingPreview { }

        // Below that, display a preview of the current location that a user is about to send.
        location_preview := LocationPreview { }

        // SDK-backed upload progress for the file-upload modal flow.
        upload_progress_view := UploadProgressView { }

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
                padding: 7,

                attachment_button := RobrixIconButton {
                    margin: 4
                    spacing: 0,
                    draw_icon +: {
                        svg: (ICON_UPLOAD)
                        color: (COLOR_TELEGRAM_MUTED)
                    },
                    draw_bg +: {
                        color: (COLOR_BG_PREVIEW)
                        color_hover: (COLOR_HEPTA_FOCUS_SURFACE_HOVER)
                        color_down: (COLOR_HEPTA_HAIRLINE)
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

fn current_time_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis().min(u64::MAX as u128) as u64)
        .unwrap_or_default()
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
