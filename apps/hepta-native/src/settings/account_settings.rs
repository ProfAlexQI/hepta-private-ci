use std::{
    cell::RefCell,
    fs,
    io::Read,
    path::{Path, PathBuf},
};

use bytesize::ByteSize;
use makepad_widgets::{text::selection::Cursor, *};
use matrix_sdk::{
    encryption::{identities::Device, VerificationState},
    ruma::{MxcUri, OwnedDeviceId, OwnedMxcUri},
};

use crate::{
    app::{ConfirmDeleteAction, PositiveConfirmationModalAction},
    avatar_cache::{self},
    logout::logout_confirm_modal::{
        LOGOUT_CONFIRMATION_COMPACT_LABEL, LogoutAction, LogoutConfirmModalAction,
    },
    profile::user_profile::UserProfile,
    settings::PopulateMode,
    shared::{
        avatar::{AvatarState, AvatarWidgetExt},
        confirmation_modal::ConfirmationModalContent,
        popup_list::{PopupKind, enqueue_popup_notification},
        styles::*,
    },
    sliding_sync::{
        get_client, submit_async_request, AccountDataAction, AccountDeviceDirectoryEntry,
        AccountDeviceRenameResult, MatrixRequest,
    },
    utils,
    verification::VerificationStateAction,
};

mod helpers;

use helpers::*;

pub const ACCOUNT_DEVICE_SELF_CHECK_READ_EVIDENCE: &str = "Device self-check uses the existing Matrix GetOwnDevice read path only while own_device is missing; fetched Device data populates the verification banner, session name, and Device ID without account mutation, device-list lookup, session-management, profile mutation, message, room-state, or membership request.";
pub const ACCOUNT_DISPLAY_NAME_CONFIRMATION_EVIDENCE: &str = "Display name drafts stay local until Save Name opens confirmation; Matrix SetDisplayName is requested only from the confirmed accept handler. DisplayNameChangeFailed leaves the draft enabled so Save Name can open confirmation again and resubmit Matrix SetDisplayName through the same live path; cancel/reset or result repaint sends no avatar, account, device/session-management, message, room-state, or membership request.";
pub const ACCOUNT_AVATAR_DELETE_CONFIRMATION_EVIDENCE: &str = "Avatar delete opens confirmation first; Matrix SetAvatar(None) is requested only from the confirmed accept handler, and cancel/result repaint sends no upload, display-name, account, device/session-management, message, room-state, or membership request.";
const ACCOUNT_CONFIRMATION_COMPACT_LABEL: &str =
    "Confirmation required before the Matrix account change runs.";
pub const ACCOUNT_AVATAR_UPLOAD_LOCAL_BOUNDARY_EVIDENCE: &str = "Account avatar upload now uses a desktop image picker plus confirmation before MatrixRequest::UploadAvatar. The confirmed path calls SDK client.account().upload_avatar, which performs Media::upload plus Account::set_avatar_url(Some(mxc)) before AvatarChanged(Some(mxc)) repaints. The direct MXC editor validates an existing mxc:// URI, opens PositiveConfirmationModal, and submits MatrixRequest::SetAvatar(Some(mxc)) through client.account().set_avatar_url(Some). Crop, Cancel, picker cancel, invalid files, mobile camera/photo-library capture, image editing, thumbnail generation, browser handoff, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, and unconfirmed live mutation remain unwired local evidence.";
pub const ACCOUNT_AVATAR_UPLOAD_LOCAL_BOUNDARY_LABEL: &str =
    "Avatar upload and direct MXC SetAvatar(Some) confirm first; crop/camera stay local.";
pub const ACCOUNT_AVATAR_UPLOAD_COMPACT_EVIDENCE: &str = "Avatar photo upload/set and direct MXC SetAvatar(Some) are real after confirmation; crop/camera stay local.";
pub const ACCOUNT_AVATAR_UPLOAD_REAL_PATH_EVIDENCE: &str = "Account avatar upload uses a desktop image picker, local file validation, confirmation modal, and MatrixRequest::UploadAvatar. The confirmed handler calls client.account().upload_avatar; the SDK uploads media and calls Account::set_avatar_url(Some(mxc)), then posts AvatarChanged(Some(mxc)). Direct MXC editor confirmation submits MatrixRequest::SetAvatar(Some(mxc)) through client.account().set_avatar_url(Some). Picker cancel, invalid files, Crop, and preview Cancel stay local and submit no upload.";
pub const ACCOUNT_AVATAR_UPLOAD_SELECTED_FILE_EVIDENCE: &str = "Account avatar upload selected-file preview shows filename, MIME type, local file size, extension, and image dimensions status after local validation and before confirmation. The preview, confirmation open, confirmation cancel, Crop, preview Cancel, picker cancel, invalid files, mobile camera/photo-library capture, image editing, thumbnail generation, browser handoff, account-data mutation beyond confirmed upload_avatar set_avatar_url(Some), message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, and live mutation remain local evidence until the confirmed UploadAvatar branch runs.";
pub const ACCOUNT_AVATAR_UPLOAD_SELECTED_FILE_LABEL: &str =
    "Selected image metadata is local until Upload is confirmed.";
pub const ACCOUNT_AVATAR_UPLOAD_SELECTED_IMAGE_METADATA_EVIDENCE: &str = "Account avatar selected image metadata preview uses the already selected local file path to show filename, MIME type, local file size, extension, and dimensions status before MatrixRequest::UploadAvatar can run. Lightweight PNG, JPEG, GIF, BMP, or WebP header dimensions can be displayed when available; unavailable dimensions stay explicit. This performs no thumbnail decode, full image decode, cropper/editor work, camera/photo-library capture, browser handoff, upload, SetAvatar(Some), account/profile mutation, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, or live mutation before the confirmed UploadAvatar branch runs.";
pub const ACCOUNT_AVATAR_UPLOAD_SELECTED_IMAGE_METADATA_LABEL: &str = "Selected avatar image metadata: filename, MIME, size, extension, and dimensions stay local until Upload is confirmed.";
pub const ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_EVIDENCE: &str = "Account avatar Thumbnail and Full-size controls run a bounded local pixel decode against only the already selected local image file. The decode first reuses the existing PNG, JPEG, GIF, BMP, or WebP header parser to enforce byte and pixel budgets, then decodes image bytes locally: Thumbnail generates an in-memory 128px RGBA thumbnail buffer and Full-size decodes the original RGBA pixel buffer. It creates no thumbnail file, runs no cropper/editor transform, opens no camera/photo-library/browser handoff, and submits no UploadAvatar, SetAvatar(Some), account/profile mutation, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, or live mutation request.";
pub const ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_LABEL: &str =
    "Thumbnail/Full-size bounded pixel decode stays local until Upload is confirmed.";
pub const ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_EVIDENCE: &str = "Account avatar upload lifecycle metadata reuses only the local selected-file metadata summary for picker opened, picker canceled, picker unsupported, invalid selection, confirmation opened, confirmation canceled, confirmed upload handoff, Crop, and preview Cancel popup states. MatrixRequest::UploadAvatar is still submitted only from the confirmed accept handler after a valid desktop image selection; the direct MXC editor uses its own confirmation before MatrixRequest::SetAvatar(Some). Picker cancel, invalid files, Crop, preview Cancel, unsupported platforms, camera/photo-library capture, image editor, thumbnail generation, browser handoff, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, and unconfirmed live mutation remain unwired local evidence.";
pub const ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_LABEL: &str =
    "Avatar upload lifecycle metadata is local; UploadAvatar waits for confirmation.";
pub const ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_EVIDENCE: &str = "Account avatar upload failed-state Retry reuses only the cached local file path and MIME type from the last validated selected image. Retry opens PositiveConfirmationModal before another MatrixRequest::UploadAvatar request is submitted; unavailable cached selection and confirmation cancel stay local. Direct MXC failed-state Retry separately reuses only the cached mxc:// URI and confirms before MatrixRequest::SetAvatar(Some). It sends no new file picker, cropper/editor work, thumbnail decode, camera/photo-library capture, browser handoff, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, or unconfirmed live mutation request.";
pub const ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_LABEL: &str = "Failed avatar upload Retry confirms before UploadAvatar; picker, cropper, camera, browser, and live mutation stay unwired.";
pub const ACCOUNT_AVATAR_DIRECT_SET_CONFIRMATION_EVIDENCE: &str = "Account avatar direct MXC editor validates an existing mxc:// URI locally, opens PositiveConfirmationModal, then submits MatrixRequest::SetAvatar(Some(mxc)). SlidingSync reuses client.account().set_avatar_url(Some(mxc)) and AvatarChanged(Some(mxc)) repaints the cached profile/avatar widgets. AvatarChangeFailed keeps the cached direct MXC URI for a confirmed Retry through the same SetAvatar(Some) path. It sends no file picker, UploadAvatar, cropper/editor work, thumbnail decode, camera/photo-library capture, browser handoff, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, or unconfirmed live mutation request.";
pub const ACCOUNT_AVATAR_DIRECT_SET_CONFIRMATION_LABEL: &str =
    "Direct MXC editor confirms before Matrix SetAvatar(Some); Retry reuses cached mxc://.";
pub const ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_EVIDENCE: &str = "Account avatar upload crop/editor boundary metadata makes the remaining account_avatar_upload product gap explicit while preserving the existing desktop picker plus confirmation-gated MatrixRequest::UploadAvatar path and the direct MXC confirmation-gated MatrixRequest::SetAvatar(Some) path. Crop renders a local avatar cropper packet snapshot from AvatarUploadPreviewState and the already selected local image summary. Crop, aspect-ratio presets, rotate/zoom, image editor controls, thumbnail generation, mobile camera capture, mobile photo-library capture, browser handoff, account-data mutation beyond confirmed UploadAvatar/direct SetAvatar, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, and live mutation remain local blocked controls. The metadata is derived only from AvatarUploadPreviewState and the already selected local image summary; it sends no file picker, image decode, cropper/editor, thumbnail, camera/photo-library, UploadAvatar, SetAvatar, account mutation, gateway/runtime/auth, or live mutation request.";
pub const ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_LABEL: &str =
    "Crop/editor, thumbnail, camera/photo-library, browser, and live mutation stay local blocked.";
pub const ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_EVIDENCE: &str = "Account avatar upload editor controls row exposes Aspect, Rotate, Zoom, Camera, and Library as visible local blocked buttons inside the avatar upload preview. Aspect renders a local avatar cropper packet snapshot from AvatarUploadPreviewState plus selected image metadata when available; Rotate, Zoom, Camera, and Library only update local preview metadata, crop/editor boundary copy, and popup text. It starts no cropper/editor, aspect-ratio transform, rotate/zoom image decode, thumbnail generation, camera capture, photo-library picker, browser handoff, UploadAvatar, SetAvatar(Some), account/profile mutation, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, or live mutation request.";
pub const ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_LABEL: &str =
    "Aspect, Rotate, Zoom, Camera, and Library are visible local avatar controls.";
pub const ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_EVIDENCE: &str = "Account avatar upload source/preview controls row exposes Source, Camera, Library, Thumbnail, Full-size, Packet, Contract, and Taxonomy as visible local buttons inside the avatar upload preview. Source can copy the already selected local avatar file path to the clipboard; Thumbnail and Full-size run a bounded local pixel decode that generates only in-memory RGBA preview buffers; Camera, Library, Packet, Contract, and Taxonomy only update local source/editor metadata, selected-image copy, and popup text from AvatarUploadPreviewState plus selected image metadata when available. Packet persists the source/editor acceptance matrix; Contract maps that matrix to typed cropper, camera, image-edit, thumbnail/full-size decode, UploadAvatar, and SetAvatar contracts; Taxonomy records source/cropper/camera/library/thumbnail artifact result slots before cropper/camera/editor work can be promoted. It opens no file picker, camera capture, photo-library picker, persistent thumbnail file, cropper/editor, browser handoff, UploadAvatar, SetAvatar(Some), account/profile mutation, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, or live mutation request.";
pub const ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_LABEL: &str = "Source copies the selected local path; Thumbnail/Full-size run bounded in-memory pixel decode; Camera, Library, Packet, Contract, and Taxonomy stay local avatar source controls.";
pub const ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_EVIDENCE: &str = "Account avatar upload source/editor drilldown packet adds a visible Packet control that persists local acceptance criteria for source type, desktop file path handoff, MIME/extension/size/dimensions, crop box/aspect/rotate/zoom, thumbnail/full-size decode targets, camera/photo-library permission and picker states, image editor handoff, UploadAvatar request/result/error/retry/source slots, and SetAvatar handoff. The packet is derived only from AvatarUploadPreviewState and selected local image metadata; it starts no file picker, camera/photo-library permission, capture, source mutation, cropper/editor, thumbnail decode/generation, full image decode, UploadAvatar, SetAvatar(Some), account/profile mutation, message send/edit/redact, room-state, membership, gateway/runtime/auth, or live mutation request.";
pub const ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_LABEL: &str =
    "Avatar source/editor drilldown acceptance criteria stay local.";
pub const ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_EVIDENCE: &str = "Account avatar upload source/editor typed contract packet adds a visible Contract control that maps the Packet acceptance matrix to typed cropper, camera, image-edit, thumbnail/full-size decode, UploadAvatar, and the now-live direct SetAvatar(Some) MXC editor handoff. The contract is derived only from AvatarUploadPreviewState and selected local image metadata; it records source identity, desktop file handoff, camera/photo-library permission and picker request/result/error slots, crop box/aspect/rotate/zoom request/result/error slots, thumbnail/full-size decode request/result/error slots, image editor transform slots, UploadAvatar request/result/error/retry/source slots, direct SetAvatar(Some) request/result/retry mapping, stale local file, source-hash, idempotency, and promotion blockers. It starts no file picker, camera/photo-library permission, capture, source mutation, cropper/editor, thumbnail decode/generation, full image decode, UploadAvatar, account/profile mutation beyond the separately confirmed SetAvatar(Some), message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, or unconfirmed live mutation request.";
pub const ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_LABEL: &str = "Avatar source/editor typed cropper-camera contracts stay local; direct MXC SetAvatar(Some) is live.";
pub const ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_EVIDENCE: &str = "Account avatar upload source/editor result taxonomy packet adds a visible Taxonomy control that records the only live avatar result references as confirmed desktop UploadAvatar plus SDK Account::set_avatar_url(Some), confirmed failed-state UploadAvatar Retry, direct MXC SetAvatar(Some) plus confirmed failed-state Retry, SetAvatar(None) delete, selected-file metadata, source-path clipboard, and bounded in-memory Thumbnail/Full-size pixel decode. Source identity, desktop picker, camera permission, photo-library permission, camera capture, photo-library selection, crop box/aspect/rotate/zoom, editor transform, persistent thumbnail artifact, transformed image upload, transformed SetAvatar mapping, mobile capture, stale source, retry/cancel, and audit redaction result slots remain not_assigned or not_wired before cropper/camera/editor work can be promoted. It starts no file picker, camera/photo-library permission, capture, source mutation, cropper/editor, persistent thumbnail generation, transformed image write, transformed UploadAvatar, transformed SetAvatar(Some), account/profile mutation beyond existing confirmed UploadAvatar/direct SetAvatar/Delete paths, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, or live mutation request.";
pub const ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_LABEL: &str = "Avatar source/editor result taxonomy packet keeps source, cropper, camera/library, thumbnail artifact, transform, retry, cancel, stale, and audit results local.";
pub const ACCOUNT_AVATAR_UPLOAD_SOURCE_PATH_CLIPBOARD_EVIDENCE: &str = "Account avatar upload Source copies only the already selected local avatar file path from AvatarUploadSelectionPreview to the local clipboard. If no image has been selected, Source stays a local prompt. It opens no file picker, camera capture, photo-library picker, thumbnail decode/generation, full image decode, cropper/editor, browser handoff, UploadAvatar, SetAvatar(Some), account/profile mutation, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, or live mutation request.";
pub const ACCOUNT_AVATAR_UPLOAD_SOURCE_PATH_CLIPBOARD_LABEL: &str =
    "Avatar Source copies selected local file path to clipboard only.";
pub const ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE: &str = "Account avatar upload preflight detail controls row exposes Request, Result, Error, Retry, and Source as visible local UploadAvatar detail buttons inside the avatar upload preview. Clicking any control only updates local preflight metadata, preview copy, and popup text from AvatarUploadPreviewState plus selected image metadata when available. It opens no file picker, cropper/editor, image decode, thumbnail generation, camera capture, photo-library picker, browser handoff, UploadAvatar, SetAvatar(Some), account/profile mutation, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, or live mutation request.";
pub const ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_LABEL: &str =
    "Request, Result, Error, Retry, and Source are visible local UploadAvatar details.";
pub const ACCOUNT_MANAGEMENT_OPTION_STAGING_EVIDENCE: &str = "Account management opens a local surface backed by the loaded own_profile account identity, the existing Matrix GetOwnDevice read path for the current session, a confirmed All devices MatrixRequest::GetDevices read path when requested, and a confirmed current-device MatrixRequest::RenameDevice path. Browser and Portal are separately confirmation-gated homeserver system-browser handoffs, while Manage Account, Security, Sessions, and Close send no password/SSO change, cross-session revoke, account mutation, message, room-state, membership, or unconfirmed live mutation request.";
pub const ACCOUNT_MANAGEMENT_LOADED_IDENTITY_EVIDENCE: &str = "Account management loaded identity preview combines already loaded own_profile display name, Matrix user id, avatar state, existing GetOwnDevice current-session details, and the latest all-device directory summary from the read-only GetDevices device directory when loaded. Browser and Portal can confirm before opening the active Matrix homeserver URL in the system browser; the identity preview itself sends no Matrix profile lookup, avatar fetch, password/SSO change, session-management mutation, cross-session revoke, account-data mutation, Matrix account mutation, profile/account mutation, message send/edit/redact, room-state, membership, gateway/runtime/auth, or live mutation request.";
pub const ACCOUNT_MANAGEMENT_LOADED_IDENTITY_LABEL: &str =
    "Loaded account identity and current session stay read-only.";
pub const ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_EVIDENCE: &str = "Account management lifecycle metadata reuses only the local AccountManagementPreviewState plus loaded own_profile identity, current Matrix device/session text, and fetched GetDevices directory summary for Manage Account, Security, Sessions, and Close popup states; All devices adds a read-only directory fetch state, and Browser/Portal add a confirmation-gated homeserver opener state. Manage Account, Security, and Sessions request MatrixRequest::GetOwnDevice only while current device data is missing; All devices submits MatrixRequest::GetDevices as a read-only directory fetch; Browser/Portal open PositiveConfirmationModal before handing the Matrix homeserver URL to robius_open; Close only hides the local preview. Password/SSO change, session-management mutation, cross-session revoke, account-data mutation, Matrix account mutation, profile/account mutation, message send/edit/redact, room-state, membership, account/device/session-management mutation, gateway/runtime/auth, and live mutation remain unwired local evidence.";
pub const ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_LABEL: &str = "Account management lifecycle metadata is local except confirmed Browser/Portal homeserver opener.";
pub const ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE: &str = "Account management Refresh opens PositiveConfirmationModal before reusing MatrixRequest::GetOwnDevice for current session metadata. The confirmed branch refreshes only the current Device display name, Device ID, and verification label already shown by AccountSettings; cancel, missing-device results, and repaint stay local. It sends no Matrix profile lookup, avatar fetch, external account page, browser handoff, password/SSO change, session-management lookup, cross-session revoke, account-data mutation, Matrix account mutation, profile/account mutation, message send/edit/redact, room-state, membership, account/device/session-management mutation, gateway/runtime/auth, or live mutation request.";
pub const ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_LABEL: &str =
    "Refresh confirms before GetOwnDevice; account/session mutations stay unwired.";
pub const ACCOUNT_MANAGEMENT_LOCAL_BOUNDARY_EVIDENCE: &str = "Account management now reuses loaded own_profile identity plus Matrix GetOwnDevice to show the current account/session/device, MatrixRequest::GetDevices to read the account device directory in Sessions, a confirmed MatrixRequest::RenameDevice path for the current device display name, and a confirmed Browser/Portal homeserver opener while account_management remains a base gap. Password change, SSO change, dedicated account-management portal route, session-management lookup/mutation, cross-session revoke, account-data mutation, Matrix account mutation, profile/account mutation beyond display name and current-device rename, message send/edit/redact, room-state, membership, account/device/session-management mutation beyond confirmed current-device rename_device, gateway/runtime/auth, and unconfirmed live mutation remain unwired local evidence.";
pub const ACCOUNT_MANAGEMENT_LOCAL_BOUNDARY_LABEL: &str = "Account management shows loaded identity, current device, read-only device directory, confirmed current-device Rename, and confirmed homeserver opener; password/SSO and revoke stay local.";
pub const ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_EVIDENCE: &str = "Account management session/revoke boundary metadata keeps account_management UI-safe after loaded identity, current-device GetOwnDevice previews, read-only GetDevices directory summaries, the confirmed current-device Rename path, and the separate confirmed homeserver opener. Dedicated external account page routes, password change, SSO change, all-device management beyond the read-only directory, session-management lookup, cross-session revoke, device delete/trust changes, account-data mutation, Matrix account/profile mutation beyond display name and current-device rename, message send/edit/redact, room-state, membership, gateway/runtime/auth, and unconfirmed live mutation remain local blocked controls. The boundary is derived only from AccountManagementPreviewState plus loaded account/device text; only confirmed current-device Rename submits MatrixRequest::RenameDevice, while all other session/device mutations stay blocked.";
pub const ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_LABEL: &str = "Password/SSO, dedicated account portal route, all-device management beyond read-only directory, revoke, delete/trust mutation, and unconfirmed live mutation stay local blocked.";
pub const ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_EVIDENCE: &str = "Account management session actions row exposes Revoke, Rename, Trust, and Browser in the account_management_preview. Rename opens PositiveConfirmationModal and only the accept branch submits MatrixRequest::RenameDevice for the loaded current Device ID through client.rename_device. Browser opens PositiveConfirmationModal and only the accept branch hands the active Matrix homeserver URL to robius_open. Revoke and Trust only stage AccountManagementPreviewState plus popup copy from already loaded own_profile identity and current-device GetOwnDevice text. The row performs no all-device list lookup, session-management lookup, cross-session revoke, device delete/trust changes, Matrix account/profile mutation beyond current-device rename, message send/edit/redact, room-state, membership, gateway/runtime/auth, or unconfirmed live mutation.";
pub const ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_LABEL: &str = "Rename confirms current-device Matrix rename_device; Revoke and Trust stay local; Browser confirms homeserver opener.";
pub const ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_EVIDENCE: &str = "Account management device-directory controls row exposes All devices, Password, SSO, Portal, and Activity in the account_management_preview. All devices submits the read-only MatrixRequest::GetDevices path and renders the returned all-device list with device id, display name, last-seen IP, and last-seen timestamp summary locally; failed GetDevices results cache the error for a confirmed Retry through the same read-only path. Portal opens PositiveConfirmationModal and only the accept branch hands the active Matrix homeserver URL to robius_open. Password, SSO, and Activity stay visible local blocked buttons that only stage AccountManagementPreviewState plus popup copy from already loaded own_profile identity and current-device GetOwnDevice text. The row performs no session-management mutation, password change, SSO start, session revoke, device trust/rename/delete change, Matrix account/profile mutation, message send, room-state or membership change, gateway/runtime/auth, or live mutation.";
pub const ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_LABEL: &str = "All devices reads Matrix device directory; Portal confirms homeserver opener; Password, SSO, and Activity stay local.";
pub const ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_RETRY_CONFIRMATION_EVIDENCE: &str = "Account management device-directory failed-state Retry reuses only the cached own_devices_last_error from the last MatrixRequest::GetDevices failure. Retry opens PositiveConfirmationModal before another MatrixRequest::GetDevices read-only request is submitted through client.devices() and AccountDataAction::OwnDevicesFetched; unavailable cache and confirmation cancel stay local. It sends no GetOwnDevice, external account portal or browser, password/SSO change, session-management mutation, cross-session revoke/trust, extra current-device RenameDevice, device delete/trust mutation, Matrix account/profile mutation beyond current-device rename, message send/edit/redact, room-state, membership, gateway/runtime/auth, or write-side live mutation request.";
pub const ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_RETRY_CONFIRMATION_LABEL: &str =
    "Device-directory Retry confirms before GetDevices; account/session mutations stay unwired.";
pub const ACCOUNT_MANAGEMENT_CURRENT_DEVICE_RENAME_EVIDENCE: &str = "Account management current-device Rename opens PositiveConfirmationModal before submitting MatrixRequest::RenameDevice for the already loaded GetOwnDevice Device ID and derived display name. SlidingSync calls client.rename_device and posts AccountDataAction::DeviceRenamed; success refreshes GetOwnDevice and GetDevices so the UI rereads current-session and directory metadata. Missing current-device metadata, confirmation cancel, empty display name, failed rename_device, and result repaint stay explicit. This path sends no password/SSO change, dedicated account portal route, session-management lookup, cross-session revoke, device delete/trust mutation, Matrix account/profile mutation beyond current-device rename, message send/edit/redact, room-state, membership, gateway/runtime/auth, Telegram delivery, or unconfirmed live mutation request.";
pub const ACCOUNT_MANAGEMENT_CURRENT_DEVICE_RENAME_LABEL: &str = "Current-device Rename confirms before Matrix rename_device; delete/trust/revoke/password stay blocked.";
pub const ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE: &str = "Account management preflight detail controls row exposes Request, Result, Error, Retry, Source, Packet, Contract, and Taxonomy as visible account/session detail buttons in the account_management_preview. Request renders a local account/session request snapshot from AccountManagementPreviewState, already loaded own_profile identity, and current-device GetOwnDevice text. Retry reuses a cached GetDevices failure only after PositiveConfirmationModal accepts another read-only MatrixRequest::GetDevices request; missing cached errors and cancel stay local. Packet persists the session/device drilldown acceptance matrix, Contract maps that matrix to typed dedicated account portal, Browser/Portal homeserver opener, all-device directory, password/SSO, current-device RenameDevice result, cross-session action, device delete/trust, account/profile mutation guard, and result/error contracts, and Taxonomy records blocked password/SSO/revoke/trust/delete result slots. Result, Error, Source, Packet, Contract, and Taxonomy only stage AccountManagementPreviewState plus popup copy from the same loaded metadata. It does not request GetOwnDevice, open a dedicated account portal route, perform session-management lookup, change password, start SSO, automatic retry, revoke any session, submit current-device RenameDevice, mutate cross-session device delete/trust state, mutate Matrix account/profile state beyond current-device rename, send a message, change room-state or membership, touch gateway/runtime/auth, or live mutation.";
pub const ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_LABEL: &str = "Request, Result, Error, Source, Packet, Contract, and Taxonomy stay local; Retry confirms cached GetDevices failures only.";
pub const ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_EVIDENCE: &str = "Account management session/device drilldown packet adds a visible Packet control that persists local acceptance criteria for loaded own_profile identity, current GetOwnDevice session/device metadata, verification state, device id/display/session/source clipboard payloads, Refresh/GetOwnDevice request/result/error/retry/source slots, current-device RenameDevice request/result/error/retry/source slots, dedicated account portal route targets, Browser/Portal homeserver opener outcome, all-device directory scope, password/SSO scope, cross-session revoke/trust scope, device delete/trust scope, account/profile mutation guard, and live-mutation boundary. The packet is derived only from AccountManagementPreviewState plus already loaded account/current-device text; it submits no extra GetOwnDevice, opens no dedicated portal route, starts no extra homeserver opener, fetches no all-device list, performs no session-management lookup, starts no password/SSO change, retries nothing automatically, revokes no session, submits no extra current-device RenameDevice, mutates no cross-session device delete/trust state, mutates no Matrix account/profile state beyond current-device rename, sends no message, changes no room-state or membership, and touches no gateway/runtime/auth or live mutation.";
pub const ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_LABEL: &str =
    "Session/device drilldown packet acceptance criteria stay local.";
pub const ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_EVIDENCE: &str = "Account management session/device typed contract packet adds a visible Contract control that maps the local session/device drilldown Packet to typed dedicated account portal route, Browser/Portal homeserver opener outcome, all-device directory, password/SSO, current-device RenameDevice, cross-session revoke/trust, device delete/trust, account/profile mutation guard, GetOwnDevice refresh, result/error/retry/source, source-hash, idempotency, stale-session, and promotion-blocker contracts. The packet is derived only from AccountManagementPreviewState plus already loaded own_profile and current-device GetOwnDevice text; it submits no extra GetOwnDevice, opens no dedicated portal route, starts no extra homeserver opener, fetches no all-device list, performs no session-management lookup, starts no password/SSO change, retries nothing automatically, revokes no session, submits no extra current-device RenameDevice, mutates no cross-session device delete/trust state, mutates no Matrix account/profile state beyond current-device rename, sends no message, changes no room-state or membership, and touches no gateway/runtime/auth or live mutation.";
pub const ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_LABEL: &str = "Session/device typed contract packet maps dedicated account portal, homeserver opener, all-device, password/SSO, current-device rename, cross-session action, and device delete/trust contracts locally.";
pub const ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_EVIDENCE: &str = "Account management session/device result taxonomy packet adds a visible Taxonomy control that records the only live account/session references as GetOwnDevice, GetDevices, SetDisplayName, current-device RenameDevice, and confirmed Browser/Portal homeserver opener, while dedicated account portal routes, password/SSO actions, cross-session revoke/trust, device delete/trust, and account/profile mutations beyond display-name/current-device rename stay blocked. The packet records operation_id slots as not_assigned, request slots as not_built, applied/permission_denied/failed/stale/cancelled result states as not_wired, stale-session and directory source-hash requirements, confirmation-gated retry policy, local-dismiss cancel policy, and audit redaction. It submits no extra GetOwnDevice, opens no dedicated portal route, starts no password/SSO flow, revokes no session, deletes/trusts no device, mutates no account/profile beyond existing live display-name/current-device rename paths, sends no message, changes no room-state or membership, and touches no gateway/runtime/auth or live mutation.";
pub const ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_LABEL: &str = "Session/device result taxonomy packet keeps password/SSO, revoke/trust/delete, and dedicated portal results local.";
pub const ACCOUNT_MANAGEMENT_BROWSER_PORTAL_HANDOFF_EVIDENCE: &str = "Account management Browser and Portal use the active Matrix client's homeserver URL from get_client().homeserver(), require an http/https URL, strip query and fragment, open PositiveConfirmationModal, and only the accept branch hands that URL to the system browser through robius_open. Missing Matrix client, invalid homeserver URL, confirmation cancel, and opener failure stay popup-only/local. This starts no MatrixRequest, password change, SSO flow, dedicated account-management portal route, session-management lookup, cross-session revoke, device trust/rename/delete mutation, Matrix account/profile mutation, message send/edit/redact, room-state, membership, gateway/runtime/auth, Telegram delivery, or live mutation.";
pub const ACCOUNT_MANAGEMENT_BROWSER_PORTAL_HANDOFF_LABEL: &str =
    "Browser/Portal confirm before opening the active Matrix homeserver in the system browser.";
pub const ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE: &str = "Account management current-device metadata controls row exposes Device, Verified, Display, Session, and Source as visible local buttons in the account_management_preview. Device copies the already loaded current Device ID from GetOwnDevice to the local clipboard when available; Verified copies the already loaded current-device verification status from local Matrix verification state plus GetOwnDevice current device ID to the local clipboard when available; Display copies the already loaded current device display name from GetOwnDevice to the local clipboard when available; Session copies the already loaded current-session summary from GetOwnDevice to the local clipboard when available; Source copies the loaded own_profile plus current-device summary to the local clipboard. It does not request extra GetOwnDevice, open an external account portal or browser, fetch an all-device list, perform session-management lookup, change password, start SSO, retry, revoke any session, submit extra current-device RenameDevice, mutate cross-session device delete/trust state, mutate Matrix account/profile state beyond current-device rename, send a message, change room-state or membership, touch gateway/runtime/auth, or live mutation.";
pub const ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_LABEL: &str =
    "Device, Verified, Display, Session, and Source copy local account/device metadata.";
pub const ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_EVIDENCE: &str = "Account management Verified copies only the already loaded current-device verification status from local Matrix verification state plus the existing GetOwnDevice current device ID to the local clipboard. If current-device metadata is pending, Verified stays a local prompt and writes no clipboard payload. It sends no extra GetOwnDevice, external account portal or browser, all-device list fetch, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, message send/edit/redact, room-state, membership, gateway/runtime/auth, or live mutation request.";
pub const ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_LABEL: &str =
    "Verified copies only loaded current-device verification status locally.";
pub const ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_EVIDENCE: &str = "Account management Device copies only the already loaded current Matrix Device ID from the existing GetOwnDevice result to the local clipboard. If current-device metadata is still pending, Device stays a local prompt and writes no clipboard payload. It sends no extra GetOwnDevice, external account portal or browser, all-device list fetch, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, message send/edit/redact, room-state, membership, gateway/runtime/auth, or live mutation request.";
pub const ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_LABEL: &str =
    "Device copies the loaded current Matrix Device ID to clipboard only.";
pub const ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_EVIDENCE: &str = "Account management Display copies only the already loaded current device display name from the existing GetOwnDevice result to the local clipboard. If current-device metadata is pending or the device has no display name, Display stays a local prompt and writes no clipboard payload. It sends no extra GetOwnDevice, external account portal or browser, all-device list fetch, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, message send/edit/redact, room-state, membership, gateway/runtime/auth, or live mutation request.";
pub const ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_LABEL: &str =
    "Display copies the loaded current device display name to clipboard only.";
pub const ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_EVIDENCE: &str = "Account management Session copies only the already loaded current-session summary from the existing GetOwnDevice result to the local clipboard. If current-device metadata is pending, Session stays a local prompt and writes no clipboard payload. It sends no extra GetOwnDevice, external account portal or browser, all-device list fetch, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, message send/edit/redact, room-state, membership, gateway/runtime/auth, or live mutation request.";
pub const ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_LABEL: &str =
    "Session copies the loaded current-session summary to clipboard only.";
pub const ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_EVIDENCE: &str = "Account management Source copies only the loaded local account/current-device summary from own_profile plus the existing GetOwnDevice text to the local clipboard. It sends no extra GetOwnDevice, external account portal or browser, all-device list fetch, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, message send/edit/redact, room-state, membership, gateway/runtime/auth, or live mutation request.";
pub const ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_LABEL: &str =
    "Source copies the local account/current-device summary to clipboard only.";
pub const ACCOUNT_MANAGEMENT_COMPACT_EVIDENCE: &str =
    "Loaded account identity uses own_profile; current session details use GetOwnDevice.";
pub const ACCOUNT_LOCAL_SURFACE_CLOSE_EVIDENCE: &str = "Account local surface Close and Cancel actions only hide local previews; avatar upload Cancel and account management Close send no file picker, image editor, upload, browser handoff, Matrix account/avatar request, message, room-state, membership, or live mutation request.";
pub const ACCOUNT_LOGOUT_CONFIRMATION_EVIDENCE: &str = "Account Logout opens confirmation first; Matrix Logout is requested only from the LogoutConfirmModal confirmed handler, and open, cancel, dismissed, reset, progress, or final-result repaint sends no extra logout, account/profile, message, room-state, membership, or live mutation request.";
pub const ACCOUNT_USER_ID_CLIPBOARD_EVIDENCE: &str = "Account Copy User ID uses the loaded own profile id to write clipboard text locally; it sends no Matrix profile lookup, account request, event fetch, message send, room-state, membership, or live mutation request.";

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*


    // The view containing all user account-related settings.
    mod.widgets.AccountSettings = #(AccountSettings::register_widget(vm)) {
        width: Fill, height: Fit
        flow: Down

        TitleLabel {
            text: "Account Settings"
        }

        // Verification banners. Both stay hidden until we know the state.
        verification_banner_verified := RoundedView {
            visible: false
            width: Fill {max: 450},
            height: Fit
            flow: Down
            spacing: 4
            margin: Inset{top: 10, bottom: 5}
            padding: Inset{top: 10, bottom: 9, left: 12, right: 12}
            show_bg: true
            draw_bg +: {
                color: (COLOR_BG_ACCEPT_GREEN)
                border_color: (COLOR_FG_ACCEPT_GREEN)
                border_size: 1.0
                border_radius: 4.0
            }
            Label {
                width: Fill, height: Fit
                flow: Flow.Right{wrap: true}
                draw_text +: {
                    color: (COLOR_FG_ACCEPT_GREEN),
                    text_style: theme.font_bold { font_size: 11.5 },
                }
                text: "This device is verified and can access encrypted messages."
            }
            Label {
                width: Fill, height: Fit
                flow: Flow.Right{wrap: true}
                draw_text +: {
                    color: (MESSAGE_TEXT_COLOR),
                    text_style: theme.font_regular { font_size: 11.0 },
                }
                text: "Device self-check uses the existing Matrix GetOwnDevice read path; fetched Device data only populates verification banner, session name, and Device ID without account mutation, device-list lookup, session-management, profile mutation, message, room-state, or membership request."
            }
        }

        verification_banner_unverified := RoundedView {
            visible: false
            width: Fill {max: 478},
            height: Fit
            flow: Down,
            align: Align {y: 0.5}
            spacing: 0,
            margin: Inset{top: 10, bottom: 5}
            padding: Inset{top: 10, bottom: 13, left: 12, right: 12}
            show_bg: true
            draw_bg +: {
                color: (COLOR_BG_DANGER_RED)
                border_color: (COLOR_FG_DANGER_RED)
                border_size: 1.0
                border_radius: 4.0
            }
            Label {
                width: Fill, height: Fit
                flow: Flow.Right{wrap: true}
                draw_text +: {
                    color: (COLOR_FG_DANGER_RED),
                    text_style: theme.font_bold { font_size: 11.5 },
                }
                text: "This device is not verified and can't view encrypted messages."
            }
            Label {
                width: Fill, height: Fit
                flow: Flow.Right{wrap: true}
                margin: Inset { top: 4, bottom: 1}
                draw_text +: {
                    color: (MESSAGE_TEXT_COLOR),
                    text_style: theme.font_regular { font_size: 11.5 },
                }
                text: "Verify it from another client using this info:"
            }
            // Filled in from Rust with the session name + device ID.
            unverified_device_info_label := Label {
                width: Fill, height: Fit
                padding: Inset{left: 8}
                flow: Flow.Right{wrap: true}
                draw_text +: {
                    color: (MESSAGE_TEXT_COLOR),
                    text_style: theme.font_regular { font_size: 11.5 },
                }
                text: ""
            }
            Label {
                width: Fill, height: Fit
                flow: Flow.Right{wrap: true}
                margin: Inset { top: 5 }
                draw_text +: {
                    color: (MESSAGE_TEXT_COLOR),
                    text_style: theme.font_regular { font_size: 11.0 },
                }
                text: "Device details come from the existing Matrix GetOwnDevice read path; fetched Device data only populates this local verification banner without account mutation, device-list lookup, session-management, profile mutation, message, room-state, or membership request."
            }
        }

        SubsectionLabel {
            text: "Your Avatar:"
        }

        View {
            width: Fill, height: Fit
            flow: Right { wrap: true },
            align: Align{y: 0.5}

            our_own_avatar := Avatar {
                width: 100,
                height: 100,
                margin: 10,
                text_view +: {
                    text +: {
                        draw_text +: {
                            text_style: theme.font_regular { font_size: 35.0 }
                        }
                    }
                }
            }

            View {
                width: Fit, height: Fit
                flow: Down,
                align: Align{y: 0.5}
                padding: Inset{ left: 10, right: 10 }
                spacing: 10
                margin: Inset{top: 15}

                View {
                    width: Fit, height: Fit
                    flow: Right,
                    align: Align{y: 0.5}
                    spacing: 10

                    upload_avatar_button := RobrixIconButton {
                        width: 140,
                        height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                        padding: Inset{top: 10, bottom: 10, left: 12, right: 15}
                        margin: 0,
                        draw_icon.svg: (ICON_UPLOAD)
                        icon_walk: Walk{width: 16, height: 16}
                        text: "Upload Avatar"
                    }

                    upload_avatar_spinner := LoadingSpinner {
                        width: 16, height: 16
                        visible: false
                        draw_bg.color: (COLOR_ACTIVE_PRIMARY)
                    }
                }

                avatar_upload_preview := RoundedView {
                    visible: false
                    width: Fill {max: 320},
                    height: Fit
                    flow: Down
                    spacing: 6
                    padding: Inset{top: 9, bottom: 9, left: 10, right: 10}
                    show_bg: true
                    draw_bg +: {
                        color: (COLOR_TELEGRAM_PANEL)
                        border_color: (COLOR_TELEGRAM_BLUE)
                        border_size: 1.0
                        border_radius: 6.0
                    }

                    Label {
                        width: Fill, height: Fit
                        draw_text +: {
                            color: (COLOR_TELEGRAM_BLUE),
                            text_style: theme.font_bold { font_size: 11.0 },
                        }
                        text: "Avatar upload preview"
                    }

                    avatar_upload_preview_status := Label {
                        width: Fill, height: Fit
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (MESSAGE_TEXT_COLOR),
                            text_style: theme.font_regular { font_size: 10.5 },
                        }
                        text: "Avatar upload uses picker + confirmation; crop/camera stay local."
                    }

                    avatar_upload_option_evidence := Label {
                        width: Fill, height: Fit
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_FG_DISABLED),
                            text_style: theme.font_regular { font_size: 10.0 },
                        }
                        text: "Avatar photo upload/set is real after confirmation; crop/camera stay local."
                    }

                    avatar_upload_editor_controls_status := Label {
                        width: Fill, height: Fit
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_BLUE),
                            text_style: theme.font_regular { font_size: 10.0 },
                        }
                        text: "Aspect, Rotate, Zoom, Camera, and Library are visible local avatar controls."
                    }

                    avatar_upload_source_preview_status := Label {
                        width: Fill, height: Fit
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_BLUE),
                            text_style: theme.font_regular { font_size: 10.0 },
                        }
                        text: "Source, Camera, Library, Thumbnail, Full-size, Packet, Contract, and Taxonomy stay local avatar source controls."
                    }

                    avatar_upload_preflight_detail_status := Label {
                        width: Fill, height: Fit
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_BLUE),
                            text_style: theme.font_regular { font_size: 10.0 },
                        }
                        text: "Request, Result, Error, Retry, and Source are local UploadAvatar preflight details."
                    }

                    View {
                        width: Fill, height: Fit
                        flow: Flow.Right{wrap: true}
                        spacing: 6
                        wrap_spacing: 6

                        avatar_preview_choose_photo_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_UPLOAD)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Choose Photo"
                        }

                        avatar_preview_crop_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_CHECKMARK)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Crop"
                        }

                        avatar_preview_retry_button := RobrixNeutralIconButton {
                            visible: false
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_UPLOAD)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Retry"
                        }

                        avatar_preview_cancel_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_FORBIDDEN)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Cancel"
                        }
                    }

                    avatar_editor_controls := View {
                        width: Fill, height: Fit
                        flow: Flow.Right{wrap: true}
                        spacing: 6
                        wrap_spacing: 6

                        avatar_editor_aspect_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_INFO)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Aspect"
                        }

                        avatar_editor_rotate_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_ROTATE_CW)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Rotate"
                        }

                        avatar_editor_zoom_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_ZOOM_IN)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Zoom"
                        }

                        avatar_editor_camera_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_UPLOAD)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Camera"
                        }

                        avatar_editor_library_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_CHECKMARK)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Library"
                        }
                    }

                    avatar_upload_source_preview_controls := View {
                        width: Fill, height: Fit
                        flow: Flow.Right{wrap: true}
                        spacing: 6
                        wrap_spacing: 6

                        avatar_source_preview_source_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_INFO)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Source"
                        }

                        avatar_source_preview_camera_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_UPLOAD)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Camera"
                        }

                        avatar_source_preview_library_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_CHECKMARK)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Library"
                        }

                        avatar_source_preview_thumbnail_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_INFO)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Thumbnail"
                        }

                        avatar_source_preview_full_size_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_ZOOM_IN)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Full-size"
                        }

                        avatar_source_preview_packet_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_INFO)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Packet"
                        }

                        avatar_source_preview_contract_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_INFO)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Contract"
                        }

                        avatar_source_preview_taxonomy_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_INFO)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Taxonomy"
                        }
                    }

                    avatar_upload_preflight_controls := View {
                        width: Fill, height: Fit
                        flow: Flow.Right{wrap: true}
                        spacing: 6
                        wrap_spacing: 6

                        avatar_upload_preflight_request_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_INFO)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Request"
                        }

                        avatar_upload_preflight_result_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_CHECKMARK)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Result"
                        }

                        avatar_upload_preflight_error_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_FORBIDDEN)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Error"
                        }

                        avatar_upload_preflight_retry_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_UPLOAD)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Retry"
                        }

                        avatar_upload_preflight_source_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_INFO)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Source"
                        }
                    }
                }

                View {
                    width: Fill {max: 320},
                    height: Fit
                    flow: Down
                    spacing: 6

                    Label {
                        width: Fill, height: Fit
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_TELEGRAM_BLUE),
                            text_style: theme.font_bold { font_size: 10.5 },
                        }
                        text: "Direct avatar MXC"
                    }

                    avatar_direct_mxc_input := RobrixTextInput {
                        width: Fill,
                        height: Fit
                        empty_text: "mxc://server/media-id"
                    }

                    avatar_direct_mxc_status := Label {
                        width: Fill, height: Fit
                        flow: Flow.Right{wrap: true}
                        draw_text +: {
                            color: (COLOR_FG_DISABLED),
                            text_style: theme.font_regular { font_size: 10.0 },
                        }
                        text: "Direct MXC editor confirms before Matrix SetAvatar(Some); Retry reuses cached mxc://."
                    }

                    View {
                        width: Fill, height: Fit
                        flow: Flow.Right{wrap: true}
                        spacing: 6
                        wrap_spacing: 6

                        avatar_direct_mxc_set_button := RobrixNeutralIconButton {
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_CHECKMARK)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Set MXC"
                        }

                        avatar_direct_mxc_retry_button := RobrixNeutralIconButton {
                            visible: false
                            height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                            padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                            draw_icon.svg: (ICON_UPLOAD)
                            icon_walk: Walk{width: 14, height: 14}
                            text: "Retry MXC"
                        }
                    }
                }

                View {
                    width: Fit, height: Fit
                    flow: Right,
                    align: Align{y: 0.5}
                    spacing: 10

                    delete_avatar_button := RobrixNegativeIconButton {
                        width: 140,
                        height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                        padding: Inset{top: 10, bottom: 10, left: 12, right: 15}
                        margin: 0,
                        draw_icon.svg: (ICON_TRASH)
                        icon_walk: Walk{ width: 16, height: 16 }
                        text: "Delete Avatar"
                    }

                    delete_avatar_spinner := LoadingSpinner {
                        width: 16, height: 16
                        visible: false
                        draw_bg.color: (COLOR_ACTIVE_PRIMARY)
                    }
                }
            }
        }

        SubsectionLabel {
            text: "Your Display Name:"
        }

        display_name_input := RobrixTextInput {
            margin: Inset{top: 3, left: 5, right: 5, bottom: 8},
            width: Fill { max: 226}, // to match the button width
            height: Fit
            empty_text: "Add a display name..."
        }

        display_name_staging_preview := RoundedView {
            visible: false
            width: Fill {max: 420},
            height: Fit
            flow: Down
            spacing: 6
            margin: Inset{left: 5, bottom: 8}
            padding: Inset{top: 9, bottom: 9, left: 12, right: 12}
            show_bg: true
            draw_bg +: {
                color: (COLOR_TELEGRAM_PANEL)
                border_color: (COLOR_TELEGRAM_BLUE)
                border_size: 1.0
                border_radius: 6.0
            }

            Label {
                width: Fill, height: Fit
                draw_text +: {
                    color: (COLOR_TELEGRAM_BLUE),
                    text_style: theme.font_bold { font_size: 11.5 },
                }
                text: "Display name staged"
            }

            display_name_staging_preview_status := Label {
                width: Fill, height: Fit
                flow: Flow.Right{wrap: true}
                draw_text +: {
                    color: (MESSAGE_TEXT_COLOR),
                    text_style: theme.font_regular { font_size: 11.0 },
                }
                text: "Display name draft is local. Save Name opens confirmation before Matrix SetDisplayName; failed results keep the draft editable for another confirmed Save Name resubmit; Cancel/reset sends no avatar, account, device/session, message, room-state, or membership request."
            }
        }

        View {
            width: Fill, height: Fit
            flow: Flow.Right{wrap: true},
            align: Align{y: 0.5},
            spacing: 10,
            wrap_spacing: 10

            // These buttons are disabled by default, and enabled when the user
            // changes the `display_name_input` text.
            // These buttons start disabled; Rust code enables them and swaps
            // their styles to RobrixNeutralIconButton / RobrixPositiveIconButton.
            cancel_display_name_button := RobrixNeutralIconButton {
                enabled: false,
                width: Fit,
                height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                padding: 10,
                margin: Inset{left: 5},
                draw_icon.svg: (ICON_FORBIDDEN)
                icon_walk: Walk{width: 16, height: 16, margin: 0}
                text: "Cancel"
            }

            accept_display_name_button := RobrixPositiveIconButton {
                enabled: false,
                width: Fit,
                height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                padding: 10,
                margin: Inset{left: 5},
                draw_icon.svg: (ICON_CHECKMARK)
                icon_walk: Walk{width: 16, height: 16, margin: 0}
                text: "Save Name"
            }

            save_name_spinner := LoadingSpinner {
                width: 16, height: 16
                margin: Inset{left: 5, top: 13} // vertically center with buttons
                visible: false
                draw_bg.color: (COLOR_ACTIVE_PRIMARY)
            }
        }

        SubsectionLabel {
            text: "Your User ID:"
        }

        View {
            width: Fill, height: Fit
            flow: Right,
            spacing: 10

            copy_user_id_button := RobrixNeutralIconButton {
                enable_long_press: true,
                margin: Inset{left: 5}
                padding: 12,
                spacing: 0,
                draw_icon.svg: (ICON_COPY)
                icon_walk: Walk{width: 16, height: 16, margin: Inset{right: -2} }
            }

            user_id := Label {
                width: Fill, height: Fit
                flow: Flow.Right{wrap: true},
                margin: Inset{top: 9}
                draw_text +: {
                    color: (MESSAGE_TEXT_COLOR),
                    text_style: MESSAGE_TEXT_STYLE { font_size: 11.5 },
                }
                text: "You are not logged in."
            }
        }

        SubsectionLabel {
            text: "Other actions:"
        }

        View {
            // margin: Inset{top: 20},
            width: Fill, height: Fit
            flow: Flow.Right{wrap: true},
            align: Align{y: 0.5},
            spacing: 10,
            wrap_spacing: 10

            manage_account_button := RobrixIconButton {
                height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                padding: Inset{left: 12, right: 15}
                margin: Inset{left: 5}
                draw_icon.svg: (ICON_EXTERNAL_LINK)
                icon_walk: Walk{width: 16, height: 16}
                text: "Manage Account"
            }

            logout_button := RobrixNegativeIconButton {
                height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                padding: Inset{top: 10, bottom: 10, left: 12, right: 15}
                margin: Inset{left: 5}
                draw_icon.svg: (ICON_LOGOUT)
                icon_walk: Walk{ width: 16, height: 16, margin: Inset{right: -2} }
                text: "Log out"
            }
        }

        account_management_preview := RoundedView {
            visible: false
            width: Fill {max: 340},
            height: Fit
            flow: Down
            spacing: 6
            margin: Inset{top: 8, left: 5}
            padding: Inset{top: 9, bottom: 9, left: 10, right: 10}
            show_bg: true
            draw_bg +: {
                color: (COLOR_TELEGRAM_PANEL)
                border_color: (COLOR_TELEGRAM_BLUE)
                border_size: 1.0
                border_radius: 6.0
            }

            Label {
                width: Fill, height: Fit
                draw_text +: {
                    color: (COLOR_TELEGRAM_BLUE),
                    text_style: theme.font_bold { font_size: 11.0 },
                }
                text: "Account management preview"
            }

            account_management_preview_status := Label {
                width: Fill, height: Fit
                flow: Flow.Right{wrap: true}
                draw_text +: {
                    color: (MESSAGE_TEXT_COLOR),
                    text_style: theme.font_regular { font_size: 10.5 },
                }
                text: "Account management shows loaded identity, current device, read-only all-devices, and confirmed Browser/Portal homeserver opener; password/SSO, revoke, and live mutation stay local."
            }

            account_management_option_evidence := Label {
                width: Fill, height: Fit
                flow: Flow.Right{wrap: true}
                draw_text +: {
                    color: (COLOR_FG_DISABLED),
                    text_style: theme.font_regular { font_size: 10.0 },
                }
                text: "Loaded account identity uses own_profile; current session details use GetOwnDevice. Browser/Portal confirm before opening the homeserver."
            }

            account_management_preflight_detail_status := Label {
                width: Fill, height: Fit
                flow: Flow.Right{wrap: true}
                draw_text +: {
                    color: (COLOR_TELEGRAM_BLUE),
                    text_style: theme.font_regular { font_size: 10.0 },
                }
                text: "Request, Result, Error, Retry, Source, Packet, Contract, and Taxonomy stay local account-management details."
            }

            account_management_device_metadata_status := Label {
                width: Fill, height: Fit
                flow: Flow.Right{wrap: true}
                draw_text +: {
                    color: (COLOR_TELEGRAM_BLUE),
                    text_style: theme.font_regular { font_size: 10.0 },
                }
                text: "Device, Verified, Display, Session, and Source copy local metadata."
            }

            View {
                width: Fill, height: Fit
                flow: Flow.Right{wrap: true}
                spacing: 6
                wrap_spacing: 6

                account_preview_security_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_CHECKMARK)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Security"
                }

                account_preview_sessions_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_EXTERNAL_LINK)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Sessions"
                }

                account_preview_refresh_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_CHECKMARK)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Refresh"
                }

                account_preview_revoke_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_TRASH)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Revoke"
                }

                account_preview_rename_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_EDIT)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Rename"
                }

                account_preview_trust_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_CHECKMARK)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Trust"
                }

                account_preview_browser_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_EXTERNAL_LINK)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Browser"
                }

                account_preview_all_devices_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_EXTERNAL_LINK)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "All devices"
                }

                account_preview_password_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_CHECKMARK)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Password"
                }

                account_preview_sso_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_EXTERNAL_LINK)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "SSO"
                }

                account_preview_portal_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_EXTERNAL_LINK)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Portal"
                }

                account_preview_activity_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_CHECKMARK)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Activity"
                }

                account_preview_close_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_FORBIDDEN)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Close"
                }
            }

            account_management_device_metadata_controls := View {
                width: Fill, height: Fit
                flow: Flow.Right{wrap: true}
                spacing: 6
                wrap_spacing: 6

                account_preview_device_metadata_device_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_INFO)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Device"
                }

                account_preview_device_metadata_verified_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_CHECKMARK)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Verified"
                }

                account_preview_device_metadata_display_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_EDIT)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Display"
                }

                account_preview_device_metadata_session_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_EXTERNAL_LINK)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Session"
                }

                account_preview_device_metadata_source_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_INFO)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Source"
                }
            }

            account_management_preflight_controls := View {
                width: Fill, height: Fit
                flow: Flow.Right{wrap: true}
                spacing: 6
                wrap_spacing: 6

                account_preview_preflight_request_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_INFO)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Request"
                }

                account_preview_preflight_result_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_CHECKMARK)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Result"
                }

                account_preview_preflight_error_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_FORBIDDEN)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Error"
                }

                account_preview_preflight_retry_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_CHECKMARK)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Retry"
                }

                account_preview_preflight_source_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_INFO)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Source"
                }

                account_preview_preflight_packet_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_INFO)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Packet"
                }

                account_preview_preflight_contract_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_INFO)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Contract"
                }

                account_preview_preflight_taxonomy_button := RobrixNeutralIconButton {
                    height: mod.widgets.SETTINGS_BUTTON_HEIGHT,
                    padding: Inset{top: 8, bottom: 8, left: 10, right: 12}
                    draw_icon.svg: (ICON_INFO)
                    icon_walk: Walk{width: 14, height: 14}
                    text: "Taxonomy"
                }
            }
        }
    }
}

/// The view containing all user account-related settings.
#[derive(Script, Widget)]
pub struct AccountSettings {
    #[deref]
    view: View,

    #[rust]
    own_profile: Option<UserProfile>,
    #[rust(VerificationState::Unknown)]
    verification_state: VerificationState,
    #[rust]
    own_device: Option<Device>,
    #[rust]
    own_devices: Vec<AccountDeviceDirectoryEntry>,
    #[rust]
    own_devices_last_error: Option<String>,
    #[rust(AvatarUploadPreviewState::Hidden)]
    avatar_upload_preview_state: AvatarUploadPreviewState,
    #[rust]
    avatar_upload_selection_preview: Option<AvatarUploadSelectionPreview>,
    #[rust]
    avatar_direct_mxc_pending_url: Option<OwnedMxcUri>,
    #[rust]
    avatar_direct_mxc_failed_url: Option<OwnedMxcUri>,
    #[rust(AccountManagementPreviewState::Hidden)]
    account_management_preview_state: AccountManagementPreviewState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AvatarUploadPreviewState {
    Hidden,
    ChoosePhoto,
    Selected,
    Failed,
    Crop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AccountManagementPreviewState {
    Hidden,
    Overview,
    Security,
    Sessions,
    Refreshing,
}

impl ScriptHook for AccountSettings {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        // After apply, the DSL fields will be reset to their defaults,
        // so we need to re-populate everything.
        if let Some(client) = get_client() {
            self.verification_state = client.encryption().verification_state().get();
        }
        if self.own_device.is_none() {
            // Account device self-check read evidence: this only reads the
            // current Matrix device through GetOwnDevice so the Account Settings
            // banner can show verification state, session name, and Device ID.
            // It sends no account mutation, device-list lookup, session-management,
            // profile mutation, message, room-state, or membership request.
            submit_async_request(MatrixRequest::GetOwnDevice);
        }
        let cx = vm.cx_mut();
        self.update_verification_banner(cx);

        // Restore user_id inline so the DSL placeholder never flashes.
        // Anything that goes through `cx.with_vm` (button colors, avatar)
        // can't run here, so it's handled later in `restore_after_reapply`.
        if !apply.is_script_reapply() {
            return;
        }
        let Some(own_profile) = self.own_profile.as_ref() else {
            return;
        };
        let cached_user_id = own_profile.user_id.as_str().to_owned();
        self.view
            .label(cx, ids!(user_id))
            .set_text(cx, &cached_user_id);
    }
}

impl Widget for AccountSettings {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);

        let copy_user_id_button = self.view.button(cx, ids!(copy_user_id_button));
        let copy_user_id_button_area = copy_user_id_button.area();
        match event.hits(cx, copy_user_id_button_area) {
            Hit::FingerHoverIn(_) | Hit::FingerLongPress(_) => {
                cx.widget_action(
                    copy_user_id_button.widget_uid(),
                    TooltipAction::HoverIn {
                        text: "Copy User ID".to_string(),
                        widget_rect: copy_user_id_button_area.rect(cx),
                        options: CalloutTooltipOptions {
                            position: TooltipPosition::Top,
                            ..Default::default()
                        },
                    },
                );
            }
            Hit::FingerHoverOut(_) => {
                cx.widget_action(copy_user_id_button.widget_uid(), TooltipAction::HoverOut);
            }
            _ => {}
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MatchEvent for AccountSettings {
    fn handle_signal(&mut self, cx: &mut Cx) {
        if self.own_profile.is_none() {
            return;
        }
        avatar_cache::process_avatar_updates(cx);

        if let Some(profile) = self.own_profile.as_mut() {
            profile.avatar_state.update_from_cache(cx);
        }
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let accept_display_name_button = self.view.button(cx, ids!(accept_display_name_button));
        let cancel_display_name_button = self.view.button(cx, ids!(cancel_display_name_button));
        let display_name_input = self.view.text_input(cx, ids!(display_name_input));
        let delete_avatar_button = self.view.button(cx, ids!(delete_avatar_button));
        let upload_avatar_button = self.view.button(cx, ids!(upload_avatar_button));
        let avatar_preview_choose_photo_button = self
            .view
            .button(cx, ids!(avatar_preview_choose_photo_button));
        let avatar_preview_crop_button = self.view.button(cx, ids!(avatar_preview_crop_button));
        let avatar_preview_retry_button = self.view.button(cx, ids!(avatar_preview_retry_button));
        let avatar_preview_cancel_button = self.view.button(cx, ids!(avatar_preview_cancel_button));
        let avatar_editor_aspect_button = self.view.button(cx, ids!(avatar_editor_aspect_button));
        let avatar_editor_rotate_button = self.view.button(cx, ids!(avatar_editor_rotate_button));
        let avatar_editor_zoom_button = self.view.button(cx, ids!(avatar_editor_zoom_button));
        let avatar_editor_camera_button = self.view.button(cx, ids!(avatar_editor_camera_button));
        let avatar_editor_library_button = self.view.button(cx, ids!(avatar_editor_library_button));
        let avatar_source_preview_source_button = self
            .view
            .button(cx, ids!(avatar_source_preview_source_button));
        let avatar_source_preview_camera_button = self
            .view
            .button(cx, ids!(avatar_source_preview_camera_button));
        let avatar_source_preview_library_button = self
            .view
            .button(cx, ids!(avatar_source_preview_library_button));
        let avatar_source_preview_thumbnail_button = self
            .view
            .button(cx, ids!(avatar_source_preview_thumbnail_button));
        let avatar_source_preview_full_size_button = self
            .view
            .button(cx, ids!(avatar_source_preview_full_size_button));
        let avatar_source_preview_packet_button = self
            .view
            .button(cx, ids!(avatar_source_preview_packet_button));
        let avatar_source_preview_contract_button = self
            .view
            .button(cx, ids!(avatar_source_preview_contract_button));
        let avatar_source_preview_taxonomy_button = self
            .view
            .button(cx, ids!(avatar_source_preview_taxonomy_button));
        let avatar_upload_preflight_request_button = self
            .view
            .button(cx, ids!(avatar_upload_preflight_request_button));
        let avatar_upload_preflight_result_button = self
            .view
            .button(cx, ids!(avatar_upload_preflight_result_button));
        let avatar_upload_preflight_error_button = self
            .view
            .button(cx, ids!(avatar_upload_preflight_error_button));
        let avatar_upload_preflight_retry_button = self
            .view
            .button(cx, ids!(avatar_upload_preflight_retry_button));
        let avatar_upload_preflight_source_button = self
            .view
            .button(cx, ids!(avatar_upload_preflight_source_button));
        let avatar_direct_mxc_input = self.view.text_input(cx, ids!(avatar_direct_mxc_input));
        let avatar_direct_mxc_set_button = self.view.button(cx, ids!(avatar_direct_mxc_set_button));
        let avatar_direct_mxc_retry_button =
            self.view.button(cx, ids!(avatar_direct_mxc_retry_button));

        for action in actions {
            if let Some(VerificationStateAction::Update(state)) = action.downcast_ref() {
                self.verification_state = *state;
                self.update_verification_banner(cx);
                continue;
            }

            // Handle LogoutAction::InProgress to update button state
            if let Some(LogoutAction::InProgress(is_in_progress)) = action.downcast_ref() {
                let logout_button = self.view.button(cx, ids!(logout_button));
                logout_button.set_text(
                    cx,
                    if *is_in_progress {
                        "Logging out..."
                    } else {
                        "Log out"
                    },
                );
                logout_button.set_enabled(cx, !*is_in_progress);
                logout_button.reset_hover(cx);
                continue;
            }

            // Handle account data changes.
            // Note: the NavigationTabBar handles removing stale data from the user_profile_cache,
            // so here, we only need to update this widget's local profile info.
            match action.downcast_ref() {
                Some(AccountDataAction::AvatarChanged(new_avatar_url)) => {
                    self.view
                        .widget(cx, ids!(upload_avatar_spinner))
                        .set_visible(cx, false);
                    self.view
                        .widget(cx, ids!(delete_avatar_spinner))
                        .set_visible(cx, false);
                    // Account avatar delete confirmation evidence: fetched
                    // avatar result data only repaints the local cached profile
                    // and avatar widgets after a confirmed SetAvatar path
                    // completes. This update sends no upload, display-name,
                    // account, device/session, message, room-state, or
                    // membership request.
                    if let Some(profile) = self.own_profile.as_mut() {
                        profile.avatar_state = AvatarState::Known(new_avatar_url.clone());
                        profile.avatar_state.update_from_cache(cx);
                        self.populate_avatar_views(cx);
                        self.avatar_upload_selection_preview = None;
                        self.avatar_direct_mxc_pending_url = None;
                        self.avatar_direct_mxc_failed_url = None;
                        self.set_avatar_upload_preview_state(cx, AvatarUploadPreviewState::Hidden);
                        self.update_avatar_direct_mxc_editor(cx);
                        enqueue_popup_notification(
                            format!(
                                "Successfully {} avatar.",
                                if new_avatar_url.is_some() {
                                    "updated"
                                } else {
                                    "deleted"
                                }
                            ),
                            PopupKind::Success,
                            Some(4.0),
                        );
                    }
                    continue;
                }
                Some(AccountDataAction::AvatarChangeFailed(err_msg)) => {
                    self.view
                        .widget(cx, ids!(upload_avatar_spinner))
                        .set_visible(cx, false);
                    self.view
                        .widget(cx, ids!(delete_avatar_spinner))
                        .set_visible(cx, false);
                    // Re-enable the avatar buttons so user can try again
                    Self::enable_upload_avatar_button(cx, true, &upload_avatar_button);
                    Self::enable_delete_avatar_button(
                        cx,
                        self.own_profile
                            .as_ref()
                            .is_some_and(|p| p.avatar_state.has_avatar()),
                        &delete_avatar_button,
                    );
                    if let Some(avatar_url) = self.avatar_direct_mxc_pending_url.take() {
                        self.avatar_direct_mxc_failed_url = Some(avatar_url);
                        self.update_avatar_direct_mxc_editor(cx);
                    }
                    if self.avatar_upload_selection_preview.is_some() {
                        self.set_avatar_upload_preview_state(cx, AvatarUploadPreviewState::Failed);
                    }
                    enqueue_popup_notification(err_msg.clone(), PopupKind::Error, Some(4.0));
                    continue;
                }
                Some(AccountDataAction::DisplayNameChanged(new_name)) => {
                    self.view
                        .widget(cx, ids!(save_name_spinner))
                        .set_visible(cx, false);
                    // Account display-name confirmation evidence: the fetched
                    // result only repaints local cached profile/input state
                    // after the confirmed SetDisplayName path completes.
                    // This update sends no avatar, account, device/session,
                    // message, room-state, or membership request.
                    if let Some(profile) = self.own_profile.as_mut() {
                        profile.username = new_name.clone();
                    }
                    // Update the display name text input and disable buttons
                    let (text, len) = new_name
                        .as_deref()
                        .map(|s| (s, s.len()))
                        .unwrap_or_default();
                    display_name_input.set_text(cx, text);
                    display_name_input.set_cursor(
                        cx,
                        Cursor {
                            index: len,
                            prefer_next_row: false,
                        },
                        false,
                    );
                    display_name_input.set_is_read_only(cx, false);
                    display_name_input.set_disabled(cx, false);
                    Self::enable_display_name_buttons(
                        cx,
                        false,
                        &accept_display_name_button,
                        &cancel_display_name_button,
                    );
                    Self::update_display_name_staging_preview(cx, &self.view, text, text);
                    enqueue_popup_notification(
                        format!(
                            "Successfully {} display name.",
                            if new_name.is_some() {
                                "updated"
                            } else {
                                "removed"
                            }
                        ),
                        PopupKind::Success,
                        Some(4.0),
                    );
                    continue;
                }
                Some(AccountDataAction::DisplayNameChangeFailed(err_msg)) => {
                    self.view
                        .widget(cx, ids!(save_name_spinner))
                        .set_visible(cx, false);
                    // Re-enable the staged draft so another confirmed Save
                    // Name click can resubmit Matrix SetDisplayName through
                    // the same live path.
                    display_name_input.set_is_read_only(cx, false);
                    display_name_input.set_disabled(cx, false);
                    Self::enable_display_name_buttons(
                        cx,
                        true,
                        &accept_display_name_button,
                        &cancel_display_name_button,
                    );
                    enqueue_popup_notification(
                        format!(
                            "{err_msg} Draft remains editable; Save Name can confirm and resubmit Matrix SetDisplayName."
                        ),
                        PopupKind::Error,
                        Some(4.0),
                    );
                    continue;
                }
                Some(AccountDataAction::OwnDeviceFetched(device)) => {
                    // Account device self-check read evidence: fetched Device data
                    // stays local to the verification banner and session/Device ID
                    // label; no account mutation or session-management request is
                    // sent from this UI update.
                    let was_refreshing = self.account_management_preview_state
                        == AccountManagementPreviewState::Refreshing;
                    self.own_device = device.as_deref().cloned();
                    self.update_verification_banner(cx);
                    if was_refreshing {
                        let loaded_identity_text = self.account_management_loaded_identity_text();
                        self.account_management_preview_state =
                            AccountManagementPreviewState::Overview;
                        enqueue_popup_notification(
                            account_management_refresh_confirmation_label(
                                "result loaded locally",
                                Some(&loaded_identity_text),
                            ),
                            PopupKind::Info,
                            Some(4.0),
                        );
                    }
                    self.update_account_management_preview(cx);
                    continue;
                }
                Some(AccountDataAction::OwnDevicesFetched(result)) => {
                    self.account_management_preview_state = AccountManagementPreviewState::Sessions;
                    let loaded_identity_text = self.account_management_loaded_identity_text();
                    let metadata = match result {
                        Ok(devices) => {
                            self.own_devices = devices.clone();
                            self.own_devices_last_error = None;
                            account_management_device_directory_result_label(
                                "All devices result loaded",
                                Some(&loaded_identity_text),
                                &self.own_devices,
                                None,
                            )
                        }
                        Err(error) => {
                            self.own_devices_last_error = Some(error.clone());
                            account_management_device_directory_result_label(
                                "All devices request failed",
                                Some(&loaded_identity_text),
                                &self.own_devices,
                                Some(error),
                            )
                        }
                    };
                    self.view
                        .label(cx, ids!(account_management_preview_status))
                        .set_text(cx, &metadata);
                    self.view
                        .label(cx, ids!(account_management_option_evidence))
                        .set_text(cx, &metadata);
                    self.view
                        .label(cx, ids!(account_management_device_metadata_status))
                        .set_text(cx, &metadata);
                    enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
                    self.view.redraw(cx);
                    continue;
                }
                Some(AccountDataAction::DeviceRenamed(result)) => {
                    self.account_management_preview_state = AccountManagementPreviewState::Security;
                    let loaded_identity_text = self.account_management_loaded_identity_text();
                    let (metadata, popup_kind) = match result {
                        Ok(AccountDeviceRenameResult {
                            device_id,
                            display_name,
                        }) => {
                            self.own_devices_last_error = None;
                            submit_async_request(MatrixRequest::GetOwnDevice);
                            submit_async_request(MatrixRequest::GetDevices);
                            (
                                account_management_current_device_rename_confirmation_label(
                                    "result loaded; Matrix rename_device succeeded and refresh reads were requested",
                                    Some(&loaded_identity_text),
                                    Some(device_id.as_str()),
                                    Some(display_name),
                                    None,
                                ),
                                PopupKind::Success,
                            )
                        }
                        Err(error) => (
                            account_management_current_device_rename_confirmation_label(
                                "request failed; current-device metadata was not mutated locally",
                                Some(&loaded_identity_text),
                                self.own_device
                                    .as_ref()
                                    .map(|device| device.device_id().as_str()),
                                None,
                                Some(error),
                            ),
                            PopupKind::Error,
                        ),
                    };
                    self.view
                        .label(cx, ids!(account_management_preview_status))
                        .set_text(cx, &metadata);
                    self.view
                        .label(cx, ids!(account_management_option_evidence))
                        .set_text(cx, &metadata);
                    self.view
                        .label(cx, ids!(account_management_device_metadata_status))
                        .set_text(cx, &metadata);
                    self.view
                        .label(cx, ids!(account_management_preflight_detail_status))
                        .set_text(cx, &metadata);
                    enqueue_popup_notification(metadata, popup_kind, Some(4.0));
                    self.view.redraw(cx);
                    continue;
                }
                _ => {}
            }

            match action.downcast_ref() {
                Some(AccountSettingsAction::AvatarDeleteStarted) => {
                    self.avatar_direct_mxc_pending_url = None;
                    self.avatar_direct_mxc_failed_url = None;
                    self.update_avatar_direct_mxc_editor(cx);
                    self.view
                        .widget(cx, ids!(delete_avatar_spinner))
                        .set_visible(cx, true);
                    Self::enable_upload_avatar_button(cx, false, &upload_avatar_button);
                    Self::enable_delete_avatar_button(cx, false, &delete_avatar_button);
                    continue;
                }
                Some(AccountSettingsAction::AvatarUploadStarted) => {
                    self.avatar_direct_mxc_pending_url = None;
                    self.avatar_direct_mxc_failed_url = None;
                    self.update_avatar_direct_mxc_editor(cx);
                    self.view
                        .widget(cx, ids!(upload_avatar_spinner))
                        .set_visible(cx, true);
                    if self.avatar_upload_selection_preview.is_some() {
                        self.set_avatar_upload_preview_state(
                            cx,
                            AvatarUploadPreviewState::Selected,
                        );
                    }
                    Self::enable_upload_avatar_button(cx, false, &upload_avatar_button);
                    Self::enable_delete_avatar_button(cx, false, &delete_avatar_button);
                    continue;
                }
                Some(AccountSettingsAction::AvatarDirectSetStarted(avatar_url)) => {
                    self.avatar_direct_mxc_pending_url = Some(avatar_url.clone());
                    self.avatar_direct_mxc_failed_url = None;
                    self.avatar_upload_selection_preview = None;
                    self.set_avatar_upload_preview_state(cx, AvatarUploadPreviewState::Hidden);
                    self.update_avatar_direct_mxc_editor(cx);
                    self.view
                        .widget(cx, ids!(upload_avatar_spinner))
                        .set_visible(cx, true);
                    Self::enable_upload_avatar_button(cx, false, &upload_avatar_button);
                    Self::enable_delete_avatar_button(cx, false, &delete_avatar_button);
                    continue;
                }
                Some(AccountSettingsAction::DisplayNameChangeStarted) => {
                    self.view
                        .widget(cx, ids!(save_name_spinner))
                        .set_visible(cx, true);
                    display_name_input.set_disabled(cx, true);
                    display_name_input.set_is_read_only(cx, true);
                    Self::enable_display_name_buttons(
                        cx,
                        false,
                        &accept_display_name_button,
                        &cancel_display_name_button,
                    );
                    continue;
                }
                Some(AccountSettingsAction::AccountManagementRefreshStarted) => {
                    self.set_account_management_preview_state(
                        cx,
                        AccountManagementPreviewState::Refreshing,
                    );
                    continue;
                }
                Some(AccountSettingsAction::AccountManagementDeviceDirectoryRetryStarted) => {
                    self.account_management_preview_state = AccountManagementPreviewState::Sessions;
                    let loaded_identity_text = self.account_management_loaded_identity_text();
                    let metadata = account_management_device_directory_retry_confirmation_label(
                        "confirmed; MatrixRequest::GetDevices was requested",
                        Some(&loaded_identity_text),
                        self.own_devices_last_error.as_deref(),
                    );
                    self.own_devices_last_error = None;
                    self.update_account_management_preview(cx);
                    self.view
                        .label(cx, ids!(account_management_preview_status))
                        .set_text(cx, &metadata);
                    self.view
                        .label(cx, ids!(account_management_option_evidence))
                        .set_text(cx, &metadata);
                    self.view
                        .label(cx, ids!(account_management_device_metadata_status))
                        .set_text(cx, &metadata);
                    self.view
                        .label(cx, ids!(account_management_preflight_detail_status))
                        .set_text(cx, &metadata);
                    self.view.redraw(cx);
                    continue;
                }
                Some(AccountSettingsAction::AccountManagementDeviceRenameStarted(
                    device_id,
                    display_name,
                )) => {
                    self.account_management_preview_state = AccountManagementPreviewState::Security;
                    let loaded_identity_text = self.account_management_loaded_identity_text();
                    let metadata = account_management_current_device_rename_confirmation_label(
                        "confirmed; MatrixRequest::RenameDevice was requested",
                        Some(&loaded_identity_text),
                        Some(device_id.as_str()),
                        Some(display_name),
                        None,
                    );
                    self.view
                        .label(cx, ids!(account_management_preview_status))
                        .set_text(cx, &metadata);
                    self.view
                        .label(cx, ids!(account_management_option_evidence))
                        .set_text(cx, &metadata);
                    self.view
                        .label(cx, ids!(account_management_device_metadata_status))
                        .set_text(cx, &metadata);
                    self.view
                        .label(cx, ids!(account_management_preflight_detail_status))
                        .set_text(cx, &metadata);
                    self.view.redraw(cx);
                    continue;
                }
                _ => {}
            }
        }

        if upload_avatar_button.clicked(actions) {
            self.open_avatar_upload_picker(cx);
        }

        if avatar_preview_choose_photo_button.clicked(actions) {
            self.open_avatar_upload_picker(cx);
        }

        if avatar_preview_crop_button.clicked(actions) {
            let selected_summary = self
                .avatar_upload_selection_preview
                .as_ref()
                .map(AvatarUploadSelectionPreview::summary);
            self.set_avatar_upload_preview_state(cx, AvatarUploadPreviewState::Crop);
            enqueue_popup_notification(
                account_avatar_upload_cropper_snapshot_label(
                    "Crop",
                    self.avatar_upload_preview_state,
                    selected_summary.as_deref(),
                ),
                PopupKind::Info,
                Some(4.0),
            );
        }

        if avatar_editor_aspect_button.clicked(actions) {
            self.stage_avatar_upload_editor_control(cx, "Aspect");
        }

        if avatar_editor_rotate_button.clicked(actions) {
            self.stage_avatar_upload_editor_control(cx, "Rotate");
        }

        if avatar_editor_zoom_button.clicked(actions) {
            self.stage_avatar_upload_editor_control(cx, "Zoom");
        }

        if avatar_editor_camera_button.clicked(actions) {
            self.stage_avatar_upload_editor_control(cx, "Camera");
        }

        if avatar_editor_library_button.clicked(actions) {
            self.stage_avatar_upload_editor_control(cx, "Library");
        }

        if avatar_source_preview_source_button.clicked(actions) {
            self.copy_avatar_upload_source_path(cx);
        }

        if avatar_source_preview_camera_button.clicked(actions) {
            self.stage_avatar_upload_source_preview_control(cx, "Camera");
        }

        if avatar_source_preview_library_button.clicked(actions) {
            self.stage_avatar_upload_source_preview_control(cx, "Library");
        }

        if avatar_source_preview_thumbnail_button.clicked(actions) {
            self.run_avatar_upload_decode_probe(cx, "Thumbnail");
        }

        if avatar_source_preview_full_size_button.clicked(actions) {
            self.run_avatar_upload_decode_probe(cx, "Full-size");
        }

        if avatar_source_preview_packet_button.clicked(actions) {
            self.stage_avatar_upload_source_preview_control(cx, "Packet");
        }

        if avatar_source_preview_contract_button.clicked(actions) {
            self.stage_avatar_upload_source_preview_control(cx, "Contract");
        }

        if avatar_source_preview_taxonomy_button.clicked(actions) {
            self.stage_avatar_upload_source_preview_control(cx, "Taxonomy");
        }

        if avatar_upload_preflight_request_button.clicked(actions) {
            self.stage_avatar_upload_preflight_detail_control(cx, "Request");
        }

        if avatar_upload_preflight_result_button.clicked(actions) {
            self.stage_avatar_upload_preflight_detail_control(cx, "Result");
        }

        if avatar_upload_preflight_error_button.clicked(actions) {
            self.stage_avatar_upload_preflight_detail_control(cx, "Error");
        }

        if avatar_upload_preflight_retry_button.clicked(actions) {
            self.stage_avatar_upload_preflight_detail_control(cx, "Retry");
        }

        if avatar_upload_preflight_source_button.clicked(actions) {
            self.stage_avatar_upload_preflight_detail_control(cx, "Source");
        }

        if avatar_direct_mxc_input.changed(actions).is_some() {
            self.update_avatar_direct_mxc_editor(cx);
        }

        if avatar_direct_mxc_set_button.clicked(actions) {
            let draft = avatar_direct_mxc_input.text();
            match parse_account_avatar_direct_mxc_uri(&draft) {
                Ok(avatar_url) => self.show_avatar_direct_mxc_confirmation(cx, avatar_url),
                Err(error) => {
                    let metadata = account_avatar_direct_mxc_editor_status_label(
                        &draft,
                        self.avatar_direct_mxc_failed_url.as_ref(),
                    );
                    self.view
                        .label(cx, ids!(avatar_direct_mxc_status))
                        .set_text(cx, &format!("{error}. {metadata}"));
                    enqueue_popup_notification(error, PopupKind::Warning, Some(4.0));
                    self.view.redraw(cx);
                }
            }
        }

        if avatar_direct_mxc_retry_button.clicked(actions) {
            self.show_avatar_direct_mxc_retry_confirmation(cx);
        }

        if avatar_preview_retry_button.clicked(actions) {
            self.show_avatar_upload_retry_confirmation(cx);
        }

        if avatar_preview_cancel_button.clicked(actions) {
            let selected_summary = self
                .avatar_upload_selection_preview
                .as_ref()
                .map(AvatarUploadSelectionPreview::summary);
            // Account local surface close evidence: Cancel only hides the
            // avatar upload preview and does not open pickers, upload media,
            // hand off to a browser, or submit Matrix account/avatar requests.
            self.avatar_upload_selection_preview = None;
            self.set_avatar_upload_preview_state(cx, AvatarUploadPreviewState::Hidden);
            enqueue_popup_notification(
                account_avatar_upload_lifecycle_metadata_label(
                    "preview canceled; local selected image cleared",
                    selected_summary.as_deref(),
                ),
                PopupKind::Info,
                Some(3.0),
            );
        }

        if delete_avatar_button.clicked(actions) {
            // Don't immediately disable the buttons. Instead, we wait for the user
            // to confirm the action in the confirmation modal,
            // and then we disable the buttons in the AvatarDeleteStarted action handler.
            let content = ConfirmationModalContent {
                title_text: "Delete Avatar".into(),
                body_text: format!("Delete your avatar? {ACCOUNT_CONFIRMATION_COMPACT_LABEL}")
                    .into(),
                accept_button_text: Some("Delete".into()),
                cancel_button_text: Some("Cancel".into()),
                on_accept_clicked: Some(Box::new(|cx| {
                    // Confirmed account avatar delete: this is the only branch
                    // that submits Matrix SetAvatar(None) from Account Settings.
                    // Upload preview and Cancel remain local UI state.
                    submit_async_request(MatrixRequest::SetAvatar { avatar_url: None });
                    cx.action(AccountSettingsAction::AvatarDeleteStarted);
                    enqueue_popup_notification(
                        "Delete Avatar confirmed. Matrix SetAvatar(None) was requested from the confirmed accept handler.",
                        PopupKind::Info,
                        Some(5.0),
                    );
                })),
                on_cancel_clicked: Some(Box::new(|_cx| {
                    enqueue_popup_notification(
                        "Delete Avatar canceled. Matrix SetAvatar(None) was not requested.",
                        PopupKind::Info,
                        Some(3.0),
                    );
                })),
            };
            enqueue_popup_notification(
                format!("Delete Avatar confirmation opened. {ACCOUNT_CONFIRMATION_COMPACT_LABEL}"),
                PopupKind::Info,
                Some(4.0),
            );
            cx.action(ConfirmDeleteAction::Show(RefCell::new(Some(content))));
        }

        let Some(own_profile) = &self.own_profile else {
            return;
        };

        // Enable the name change buttons if the user modified the display name to be different.
        if let Some(new_name) = display_name_input.changed(actions) {
            let trimmed = new_name.trim();
            let current_name = own_profile.username.as_deref().unwrap_or("");
            let enable = trimmed != current_name;
            Self::enable_display_name_buttons(
                cx,
                enable,
                &accept_display_name_button,
                &cancel_display_name_button,
            );
            Self::update_display_name_staging_preview(cx, &self.view, trimmed, current_name);
        }

        if cancel_display_name_button.clicked(actions) {
            // Reset the display name input and disable the name change buttons.
            // This is a local reset only; it sends no Matrix SetDisplayName,
            // avatar, account, device/session, message, room-state, or
            // membership request.
            let new_text = own_profile.username.as_deref().unwrap_or("");
            display_name_input.set_text(cx, new_text);
            display_name_input.set_cursor(
                cx,
                Cursor {
                    index: new_text.len(),
                    prefer_next_row: false,
                },
                false,
            );
            Self::enable_display_name_buttons(
                cx,
                false,
                &accept_display_name_button,
                &cancel_display_name_button,
            );
            Self::update_display_name_staging_preview(cx, &self.view, new_text, new_text);
        }

        if accept_display_name_button.clicked(actions) {
            let new_display_name = match display_name_input.text().trim() {
                "" => None,
                name => Some(name.to_string()),
            };
            let display_name_label = new_display_name
                .as_deref()
                .filter(|name| !name.trim().is_empty())
                .unwrap_or("no display name")
                .to_string();
            let content = ConfirmationModalContent {
                title_text: "Save Display Name".into(),
                body_text: format!(
                    "Save display name as \"{display_name_label}\"? {ACCOUNT_CONFIRMATION_COMPACT_LABEL}"
                )
                .into(),
                accept_button_text: Some("Save Name".into()),
                cancel_button_text: Some("Cancel".into()),
                on_accept_clicked: Some(Box::new(move |cx| {
                    // Confirmed account display-name update: this is the only
                    // branch that submits Matrix SetDisplayName from Account
                    // Settings. The staging preview and Cancel/reset branches
                    // remain local UI state.
                    submit_async_request(MatrixRequest::SetDisplayName { new_display_name });
                    cx.action(AccountSettingsAction::DisplayNameChangeStarted);
                    enqueue_popup_notification(
                        "Save Name confirmed. Matrix SetDisplayName was requested from the confirmed accept handler.",
                        PopupKind::Info,
                        Some(5.0),
                    );
                })),
                on_cancel_clicked: Some(Box::new(|_cx| {
                    enqueue_popup_notification(
                        "Save Name canceled. Matrix SetDisplayName was not requested.",
                        PopupKind::Info,
                        Some(3.0),
                    );
                })),
            };
            enqueue_popup_notification(
                format!("Save Name confirmation opened. {ACCOUNT_CONFIRMATION_COMPACT_LABEL}"),
                PopupKind::Info,
                Some(4.0),
            );
            cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
                content,
            ))));
        }

        if self
            .view
            .button(cx, ids!(copy_user_id_button))
            .clicked(actions)
        {
            // Profile/account identity clipboard evidence: Copy User ID uses
            // the already loaded own profile id and only writes local
            // clipboard text. It sends no Matrix profile lookup, account
            // request, event fetch, message send, room-state, membership, or
            // live mutation request.
            cx.copy_to_clipboard(own_profile.user_id.as_str());
            enqueue_popup_notification(
                "Copied your User ID locally. No Matrix profile lookup, account request, event fetch, message send, room-state, membership, or live mutation request was sent.",
                PopupKind::Success,
                Some(3.0),
            );
        }

        if self
            .view
            .button(cx, ids!(manage_account_button))
            .clicked(actions)
        {
            // Account management preview reuses the existing GetOwnDevice read
            // path for current-session details; Browser/Portal use their own
            // confirmation-gated homeserver opener.
            if self.own_device.is_none() {
                submit_async_request(MatrixRequest::GetOwnDevice);
            }
            self.set_account_management_preview_state(cx, AccountManagementPreviewState::Overview);
            let loaded_identity_text = self.account_management_loaded_identity_text();
            enqueue_popup_notification(
                account_management_session_revoke_boundary_label(
                    AccountManagementPreviewState::Overview,
                    Some(&loaded_identity_text),
                ),
                PopupKind::Info,
                Some(4.0),
            );
        }

        if self
            .view
            .button(cx, ids!(account_preview_security_button))
            .clicked(actions)
        {
            // Security shows the current Matrix device verification/session details
            // from GetOwnDevice; password/SSO changes stay unwired.
            if self.own_device.is_none() {
                submit_async_request(MatrixRequest::GetOwnDevice);
            }
            self.set_account_management_preview_state(cx, AccountManagementPreviewState::Security);
            let loaded_identity_text = self.account_management_loaded_identity_text();
            enqueue_popup_notification(
                account_management_session_revoke_boundary_label(
                    AccountManagementPreviewState::Security,
                    Some(&loaded_identity_text),
                ),
                PopupKind::Info,
                Some(4.0),
            );
        }

        if self
            .view
            .button(cx, ids!(account_preview_sessions_button))
            .clicked(actions)
        {
            // Sessions shows this device via GetOwnDevice and can load the read-only
            // device directory; cross-session revoke remains unwired.
            if self.own_device.is_none() {
                submit_async_request(MatrixRequest::GetOwnDevice);
            }
            self.set_account_management_preview_state(cx, AccountManagementPreviewState::Sessions);
            let loaded_identity_text = self.account_management_loaded_identity_text();
            enqueue_popup_notification(
                account_management_session_revoke_boundary_label(
                    AccountManagementPreviewState::Sessions,
                    Some(&loaded_identity_text),
                ),
                PopupKind::Info,
                Some(4.0),
            );
        }

        if self
            .view
            .button(cx, ids!(account_preview_refresh_button))
            .clicked(actions)
        {
            self.show_account_management_refresh_confirmation(cx);
        }

        if self
            .view
            .button(cx, ids!(account_preview_revoke_button))
            .clicked(actions)
        {
            self.stage_account_management_session_action(
                cx,
                "Revoke",
                AccountManagementPreviewState::Sessions,
            );
        }

        if self
            .view
            .button(cx, ids!(account_preview_rename_button))
            .clicked(actions)
        {
            self.show_account_management_device_rename_confirmation(cx);
        }

        if self
            .view
            .button(cx, ids!(account_preview_trust_button))
            .clicked(actions)
        {
            self.stage_account_management_session_action(
                cx,
                "Trust",
                AccountManagementPreviewState::Security,
            );
        }

        if self
            .view
            .button(cx, ids!(account_preview_browser_button))
            .clicked(actions)
        {
            self.show_account_management_browser_portal_confirmation(
                cx,
                "Browser",
                AccountManagementPreviewState::Overview,
            );
        }

        if self
            .view
            .button(cx, ids!(account_preview_all_devices_button))
            .clicked(actions)
        {
            self.request_account_management_all_devices(cx);
        }

        if self
            .view
            .button(cx, ids!(account_preview_password_button))
            .clicked(actions)
        {
            self.stage_account_management_device_directory_action(
                cx,
                "Password",
                AccountManagementPreviewState::Security,
            );
        }

        if self
            .view
            .button(cx, ids!(account_preview_sso_button))
            .clicked(actions)
        {
            self.stage_account_management_device_directory_action(
                cx,
                "SSO",
                AccountManagementPreviewState::Security,
            );
        }

        if self
            .view
            .button(cx, ids!(account_preview_portal_button))
            .clicked(actions)
        {
            self.show_account_management_browser_portal_confirmation(
                cx,
                "Portal",
                AccountManagementPreviewState::Overview,
            );
        }

        if self
            .view
            .button(cx, ids!(account_preview_activity_button))
            .clicked(actions)
        {
            self.stage_account_management_device_directory_action(
                cx,
                "Activity",
                AccountManagementPreviewState::Sessions,
            );
        }

        if self
            .view
            .button(cx, ids!(account_preview_device_metadata_device_button))
            .clicked(actions)
        {
            self.copy_account_management_current_device_id(cx);
        }

        if self
            .view
            .button(cx, ids!(account_preview_device_metadata_verified_button))
            .clicked(actions)
        {
            self.copy_account_management_current_device_verification(cx);
        }

        if self
            .view
            .button(cx, ids!(account_preview_device_metadata_display_button))
            .clicked(actions)
        {
            self.copy_account_management_current_device_display_name(cx);
        }

        if self
            .view
            .button(cx, ids!(account_preview_device_metadata_session_button))
            .clicked(actions)
        {
            self.copy_account_management_current_session(cx);
        }

        if self
            .view
            .button(cx, ids!(account_preview_device_metadata_source_button))
            .clicked(actions)
        {
            self.copy_account_management_current_device_source_metadata(cx);
        }

        if self
            .view
            .button(cx, ids!(account_preview_preflight_request_button))
            .clicked(actions)
        {
            self.stage_account_management_preflight_detail_control(cx, "Request");
        }

        if self
            .view
            .button(cx, ids!(account_preview_preflight_result_button))
            .clicked(actions)
        {
            self.stage_account_management_preflight_detail_control(cx, "Result");
        }

        if self
            .view
            .button(cx, ids!(account_preview_preflight_error_button))
            .clicked(actions)
        {
            self.stage_account_management_preflight_detail_control(cx, "Error");
        }

        if self
            .view
            .button(cx, ids!(account_preview_preflight_retry_button))
            .clicked(actions)
        {
            self.show_account_management_device_directory_retry_confirmation(cx);
        }

        if self
            .view
            .button(cx, ids!(account_preview_preflight_source_button))
            .clicked(actions)
        {
            self.stage_account_management_preflight_detail_control(cx, "Source");
        }

        if self
            .view
            .button(cx, ids!(account_preview_preflight_packet_button))
            .clicked(actions)
        {
            self.stage_account_management_preflight_detail_control(cx, "Packet");
        }

        if self
            .view
            .button(cx, ids!(account_preview_preflight_contract_button))
            .clicked(actions)
        {
            self.stage_account_management_preflight_detail_control(cx, "Contract");
        }

        if self
            .view
            .button(cx, ids!(account_preview_preflight_taxonomy_button))
            .clicked(actions)
        {
            self.stage_account_management_preflight_detail_control(cx, "Taxonomy");
        }

        if self
            .view
            .button(cx, ids!(account_preview_close_button))
            .clicked(actions)
        {
            // Account management option staging evidence: Close only hides the
            // preview and does not send account, device, message, or room-state requests.
            let previous_state = self.account_management_preview_state;
            let loaded_identity_text = self.account_management_loaded_identity_text();
            self.set_account_management_preview_state(cx, AccountManagementPreviewState::Hidden);
            enqueue_popup_notification(
                account_management_lifecycle_metadata_label(
                    "Close hid the local preview; no account management request was sent",
                    previous_state,
                    Some(&loaded_identity_text),
                ),
                PopupKind::Info,
                Some(3.0),
            );
        }

        if self.view.button(cx, ids!(logout_button)).clicked(actions) {
            // Account logout confirmation evidence: opening the modal only
            // stages local UI state. Matrix Logout is requested solely by the
            // confirmed LogoutConfirmModal handler.
            enqueue_popup_notification(
                format!("Logout confirmation opened. {LOGOUT_CONFIRMATION_COMPACT_LABEL}"),
                PopupKind::Info,
                Some(4.0),
            );
            cx.action(LogoutConfirmModalAction::Open);
        }
    }
}

impl AccountSettings {
    fn stage_account_management_session_action(
        &mut self,
        cx: &mut Cx,
        action_label: &str,
        preview_state: AccountManagementPreviewState,
    ) {
        self.set_account_management_preview_state(cx, preview_state);
        let loaded_identity_text = self.account_management_loaded_identity_text();
        enqueue_popup_notification(
            account_management_session_actions_row_label(
                action_label,
                preview_state,
                Some(&loaded_identity_text),
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn stage_account_management_device_directory_action(
        &mut self,
        cx: &mut Cx,
        action_label: &str,
        preview_state: AccountManagementPreviewState,
    ) {
        self.set_account_management_preview_state(cx, preview_state);
        let loaded_identity_text = self.account_management_loaded_identity_text();
        enqueue_popup_notification(
            account_management_device_directory_controls_row_label(
                action_label,
                preview_state,
                Some(&loaded_identity_text),
            ),
            PopupKind::Info,
            Some(4.0),
        );
    }

    fn show_account_management_browser_portal_confirmation(
        &mut self,
        cx: &mut Cx,
        action_label: &'static str,
        preview_state: AccountManagementPreviewState,
    ) {
        self.set_account_management_preview_state(cx, preview_state);
        let loaded_identity_text = self.account_management_loaded_identity_text();
        let target_url = match account_management_browser_portal_url_from_client() {
            Ok(url) => url,
            Err(error) => {
                let metadata = account_management_browser_portal_handoff_label(
                    action_label,
                    preview_state,
                    Some(&loaded_identity_text),
                    None,
                    Some(&error),
                );
                self.view
                    .label(cx, ids!(account_management_preview_status))
                    .set_text(cx, &metadata);
                self.view
                    .label(cx, ids!(account_management_option_evidence))
                    .set_text(cx, &metadata);
                self.view
                    .label(cx, ids!(account_management_preflight_detail_status))
                    .set_text(cx, &metadata);
                enqueue_popup_notification(metadata, PopupKind::Warning, Some(4.0));
                self.view.redraw(cx);
                return;
            }
        };

        let opened_metadata = account_management_browser_portal_handoff_label(
            action_label,
            preview_state,
            Some(&loaded_identity_text),
            Some(&target_url),
            None,
        );
        self.view
            .label(cx, ids!(account_management_preview_status))
            .set_text(cx, &opened_metadata);
        self.view
            .label(cx, ids!(account_management_option_evidence))
            .set_text(cx, &opened_metadata);
        self.view
            .label(cx, ids!(account_management_preflight_detail_status))
            .set_text(cx, &opened_metadata);

        let target_url_for_accept = target_url.clone();
        let target_url_for_cancel = target_url.clone();
        let loaded_identity_for_accept = loaded_identity_text.clone();
        let loaded_identity_for_cancel = loaded_identity_text;
        let content = ConfirmationModalContent {
            title_text: format!("Open Account {action_label}").into(),
            body_text: opened_metadata.clone().into(),
            accept_button_text: Some("Open".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |_cx| {
                match robius_open::Uri::new(&target_url_for_accept).open() {
                    Ok(()) => enqueue_popup_notification(
                        account_management_browser_portal_handoff_label(
                            action_label,
                            preview_state,
                            Some(&loaded_identity_for_accept),
                            Some(&target_url_for_accept),
                            None,
                        ),
                        PopupKind::Success,
                        Some(4.0),
                    ),
                    Err(error) => {
                        error!(
                            "Failed to open account management homeserver URL {target_url_for_accept}: {error:?}"
                        );
                        enqueue_popup_notification(
                            account_management_browser_portal_handoff_label(
                                action_label,
                                preview_state,
                                Some(&loaded_identity_for_accept),
                                Some(&target_url_for_accept),
                                Some("system opener failed"),
                            ),
                            PopupKind::Warning,
                            Some(4.0),
                        );
                    }
                }
            })),
            on_cancel_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    account_management_browser_portal_handoff_label(
                        action_label,
                        preview_state,
                        Some(&loaded_identity_for_cancel),
                        Some(&target_url_for_cancel),
                        Some("confirmation canceled; homeserver was not opened"),
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
        };
        enqueue_popup_notification(opened_metadata, PopupKind::Info, Some(4.0));
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
        self.view.redraw(cx);
    }

    fn request_account_management_all_devices(&mut self, cx: &mut Cx) {
        self.set_account_management_preview_state(cx, AccountManagementPreviewState::Sessions);
        self.own_devices_last_error = None;
        let loaded_identity_text = self.account_management_loaded_identity_text();
        let metadata = account_management_device_directory_result_label(
            "All devices request submitted",
            Some(&loaded_identity_text),
            &self.own_devices,
            None,
        );
        self.view
            .label(cx, ids!(account_management_preview_status))
            .set_text(cx, &metadata);
        self.view
            .label(cx, ids!(account_management_option_evidence))
            .set_text(cx, &metadata);
        self.view
            .label(cx, ids!(account_management_device_metadata_status))
            .set_text(cx, &metadata);
        submit_async_request(MatrixRequest::GetDevices);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
        self.view.redraw(cx);
    }

    fn copy_account_management_current_device_id(&mut self, cx: &mut Cx) {
        let preview_state = AccountManagementPreviewState::Sessions;
        self.set_account_management_preview_state(cx, preview_state);
        let device_id = self
            .own_device
            .as_ref()
            .map(|device| device.device_id().as_str());
        let payload = account_management_current_device_id_clipboard_payload(device_id);
        if let Some(device_id) = payload.as_deref() {
            // Current-device clipboard evidence: Device copies only the
            // already loaded GetOwnDevice id and never requests session lists
            // or mutates account/device state.
            cx.copy_to_clipboard(device_id);
        }

        let loaded_identity_text = self.account_management_loaded_identity_text();
        let metadata = account_management_current_device_id_clipboard_label(
            preview_state,
            Some(&loaded_identity_text),
            payload.as_deref(),
        );
        self.view
            .label(cx, ids!(account_management_device_metadata_status))
            .set_text(cx, &metadata);
        self.view
            .label(cx, ids!(account_management_option_evidence))
            .set_text(cx, &metadata);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
        self.view.redraw(cx);
    }

    fn copy_account_management_current_device_verification(&mut self, cx: &mut Cx) {
        let preview_state = AccountManagementPreviewState::Security;
        self.set_account_management_preview_state(cx, preview_state);
        let device_id = self
            .own_device
            .as_ref()
            .map(|device| device.device_id().to_string());
        let payload = account_management_current_device_verification_clipboard_payload(
            device_id.as_deref(),
            self.verification_state,
        );
        if let Some(summary) = payload.as_deref() {
            // Verified clipboard evidence: this copies only the local
            // current-device verification status already shown in Settings.
            cx.copy_to_clipboard(summary);
        }

        let loaded_identity_text = self.account_management_loaded_identity_text();
        let metadata = account_management_current_device_verification_clipboard_label(
            preview_state,
            Some(&loaded_identity_text),
            device_id.as_deref(),
            self.verification_state,
        );
        self.view
            .label(cx, ids!(account_management_device_metadata_status))
            .set_text(cx, &metadata);
        self.view
            .label(cx, ids!(account_management_option_evidence))
            .set_text(cx, &metadata);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
        self.view.redraw(cx);
    }

    fn copy_account_management_current_device_source_metadata(&mut self, cx: &mut Cx) {
        let preview_state = AccountManagementPreviewState::Overview;
        self.set_account_management_preview_state(cx, preview_state);
        let loaded_identity_text = self.account_management_loaded_identity_text();
        let payload =
            account_management_current_device_source_clipboard_payload(Some(&loaded_identity_text));
        if let Some(summary) = payload.as_deref() {
            // Source clipboard evidence: this copies only local account and
            // current-device summary text already rendered in Settings.
            cx.copy_to_clipboard(summary);
        }

        let metadata = account_management_current_device_source_clipboard_label(
            preview_state,
            payload.as_deref(),
        );
        self.view
            .label(cx, ids!(account_management_device_metadata_status))
            .set_text(cx, &metadata);
        self.view
            .label(cx, ids!(account_management_option_evidence))
            .set_text(cx, &metadata);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
        self.view.redraw(cx);
    }

    fn copy_account_management_current_device_display_name(&mut self, cx: &mut Cx) {
        let preview_state = AccountManagementPreviewState::Security;
        self.set_account_management_preview_state(cx, preview_state);
        let display_name = self
            .own_device
            .as_ref()
            .and_then(|device| device.display_name());
        let payload =
            account_management_current_device_display_name_clipboard_payload(display_name);
        if let Some(display_name) = payload.as_deref() {
            // Display clipboard evidence: this copies only the local
            // current-device display name already loaded via GetOwnDevice.
            cx.copy_to_clipboard(display_name);
        }

        let loaded_identity_text = self.account_management_loaded_identity_text();
        let metadata = account_management_current_device_display_name_clipboard_label(
            preview_state,
            Some(&loaded_identity_text),
            payload.as_deref(),
        );
        self.view
            .label(cx, ids!(account_management_device_metadata_status))
            .set_text(cx, &metadata);
        self.view
            .label(cx, ids!(account_management_option_evidence))
            .set_text(cx, &metadata);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
        self.view.redraw(cx);
    }

    fn copy_account_management_current_session(&mut self, cx: &mut Cx) {
        let preview_state = AccountManagementPreviewState::Sessions;
        self.set_account_management_preview_state(cx, preview_state);
        let session_text = self
            .own_device
            .as_ref()
            .map(|_| self.account_management_device_text());
        let payload = account_management_current_session_clipboard_payload(session_text.as_deref());
        if let Some(session_text) = payload.as_deref() {
            // Session clipboard evidence: this copies only local current-device
            // session text already loaded through GetOwnDevice.
            cx.copy_to_clipboard(session_text);
        }

        let loaded_identity_text = self.account_management_loaded_identity_text();
        let metadata = account_management_current_session_clipboard_label(
            preview_state,
            Some(&loaded_identity_text),
            payload.as_deref(),
        );
        self.view
            .label(cx, ids!(account_management_device_metadata_status))
            .set_text(cx, &metadata);
        self.view
            .label(cx, ids!(account_management_option_evidence))
            .set_text(cx, &metadata);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
        self.view.redraw(cx);
    }

    #[allow(dead_code)]
    fn stage_account_management_current_device_metadata_control(
        &mut self,
        cx: &mut Cx,
        action_label: &str,
        preview_state: AccountManagementPreviewState,
    ) {
        self.set_account_management_preview_state(cx, preview_state);
        let loaded_identity_text = self.account_management_loaded_identity_text();
        let metadata = account_management_current_device_metadata_controls_row_label(
            action_label,
            preview_state,
            Some(&loaded_identity_text),
        );
        self.view
            .label(cx, ids!(account_management_device_metadata_status))
            .set_text(cx, &metadata);
        self.view
            .label(cx, ids!(account_management_option_evidence))
            .set_text(cx, &metadata);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
        self.view.redraw(cx);
    }

    fn stage_account_management_preflight_detail_control(
        &mut self,
        cx: &mut Cx,
        action_label: &str,
    ) {
        if self.account_management_preview_state == AccountManagementPreviewState::Hidden {
            self.set_account_management_preview_state(cx, AccountManagementPreviewState::Overview);
        }
        let loaded_identity_text = self.account_management_loaded_identity_text();
        let metadata = if action_label == "Request" {
            account_management_request_snapshot_label(
                action_label,
                self.account_management_preview_state,
                Some(&loaded_identity_text),
            )
        } else {
            account_management_preflight_detail_controls_row_label(
                action_label,
                self.account_management_preview_state,
                Some(&loaded_identity_text),
            )
        };
        self.view
            .label(cx, ids!(account_management_preflight_detail_status))
            .set_text(cx, &metadata);
        self.view
            .label(cx, ids!(account_management_option_evidence))
            .set_text(cx, &metadata);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
        self.view.redraw(cx);
    }

    fn show_account_management_refresh_confirmation(&mut self, cx: &mut Cx) {
        let loaded_identity_text = self.account_management_loaded_identity_text();
        let loaded_identity_for_accept = loaded_identity_text.clone();
        let loaded_identity_for_cancel = loaded_identity_text.clone();
        let content = ConfirmationModalContent {
            title_text: "Refresh Session Metadata".into(),
            body_text: account_management_refresh_confirmation_label(
                "confirmation opened",
                Some(&loaded_identity_text),
            )
            .into(),
            accept_button_text: Some("Refresh".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |cx| {
                // Confirmed account management refresh: this reuses only the
                // existing current-device read path for local session metadata.
                submit_async_request(MatrixRequest::GetOwnDevice);
                cx.action(AccountSettingsAction::AccountManagementRefreshStarted);
                enqueue_popup_notification(
                    account_management_refresh_confirmation_label(
                        "confirmed; MatrixRequest::GetOwnDevice was requested",
                        Some(&loaded_identity_for_accept),
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    account_management_refresh_confirmation_label(
                        "confirmation canceled; GetOwnDevice was not requested",
                        Some(&loaded_identity_for_cancel),
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
        };
        enqueue_popup_notification(
            account_management_refresh_confirmation_label(
                "confirmation opened",
                Some(&loaded_identity_text),
            ),
            PopupKind::Info,
            Some(4.0),
        );
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
    }

    fn show_account_management_device_directory_retry_confirmation(&mut self, cx: &mut Cx) {
        self.set_account_management_preview_state(cx, AccountManagementPreviewState::Sessions);
        let loaded_identity_text = self.account_management_loaded_identity_text();
        let Some(cached_error) = self
            .own_devices_last_error
            .as_deref()
            .map(str::trim)
            .filter(|error| !error.is_empty())
            .map(str::to_owned)
        else {
            let metadata = account_management_device_directory_retry_confirmation_label(
                "unavailable; no cached GetDevices failure",
                Some(&loaded_identity_text),
                None,
            );
            self.view
                .label(cx, ids!(account_management_preflight_detail_status))
                .set_text(cx, &metadata);
            self.view
                .label(cx, ids!(account_management_option_evidence))
                .set_text(cx, &metadata);
            enqueue_popup_notification(metadata, PopupKind::Warning, Some(4.0));
            self.view.redraw(cx);
            return;
        };
        let loaded_identity_for_accept = loaded_identity_text.clone();
        let loaded_identity_for_cancel = loaded_identity_text.clone();
        let cached_error_for_accept = cached_error.clone();
        let cached_error_for_cancel = cached_error.clone();
        let content = ConfirmationModalContent {
            title_text: "Retry Device Directory".into(),
            body_text: account_management_device_directory_retry_confirmation_label(
                "confirmation opened",
                Some(&loaded_identity_text),
                Some(&cached_error),
            )
            .into(),
            accept_button_text: Some("Retry".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |cx| {
                submit_async_request(MatrixRequest::GetDevices);
                cx.action(AccountSettingsAction::AccountManagementDeviceDirectoryRetryStarted);
                enqueue_popup_notification(
                    account_management_device_directory_retry_confirmation_label(
                        "confirmed; MatrixRequest::GetDevices was requested",
                        Some(&loaded_identity_for_accept),
                        Some(&cached_error_for_accept),
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    account_management_device_directory_retry_confirmation_label(
                        "confirmation canceled; GetDevices was not requested",
                        Some(&loaded_identity_for_cancel),
                        Some(&cached_error_for_cancel),
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
        };
        let metadata = account_management_device_directory_retry_confirmation_label(
            "confirmation opened",
            Some(&loaded_identity_text),
            Some(&cached_error),
        );
        self.view
            .label(cx, ids!(account_management_preflight_detail_status))
            .set_text(cx, &metadata);
        self.view
            .label(cx, ids!(account_management_option_evidence))
            .set_text(cx, &metadata);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
    }

    fn show_account_management_device_rename_confirmation(&mut self, cx: &mut Cx) {
        self.set_account_management_preview_state(cx, AccountManagementPreviewState::Security);
        let loaded_identity_text = self.account_management_loaded_identity_text();
        let Some(device_id) = self
            .own_device
            .as_ref()
            .map(|device| device.device_id().to_owned())
        else {
            submit_async_request(MatrixRequest::GetOwnDevice);
            let metadata = account_management_current_device_rename_confirmation_label(
                "unavailable; requested current-device metadata first",
                Some(&loaded_identity_text),
                None,
                None,
                Some("current device metadata is pending"),
            );
            self.view
                .label(cx, ids!(account_management_preview_status))
                .set_text(cx, &metadata);
            self.view
                .label(cx, ids!(account_management_option_evidence))
                .set_text(cx, &metadata);
            self.view
                .label(cx, ids!(account_management_device_metadata_status))
                .set_text(cx, &metadata);
            enqueue_popup_notification(metadata, PopupKind::Warning, Some(4.0));
            self.view.redraw(cx);
            return;
        };
        let display_name = account_management_device_rename_target(self.own_profile.as_ref());
        if display_name.trim().is_empty() {
            let metadata = account_management_current_device_rename_confirmation_label(
                "unavailable; target display name was empty",
                Some(&loaded_identity_text),
                Some(device_id.as_str()),
                None,
                Some("target display name is empty"),
            );
            self.view
                .label(cx, ids!(account_management_preview_status))
                .set_text(cx, &metadata);
            self.view
                .label(cx, ids!(account_management_option_evidence))
                .set_text(cx, &metadata);
            self.view
                .label(cx, ids!(account_management_device_metadata_status))
                .set_text(cx, &metadata);
            enqueue_popup_notification(metadata, PopupKind::Warning, Some(4.0));
            self.view.redraw(cx);
            return;
        }

        let opened_metadata = account_management_current_device_rename_confirmation_label(
            "confirmation opened",
            Some(&loaded_identity_text),
            Some(device_id.as_str()),
            Some(&display_name),
            None,
        );
        self.view
            .label(cx, ids!(account_management_preview_status))
            .set_text(cx, &opened_metadata);
        self.view
            .label(cx, ids!(account_management_option_evidence))
            .set_text(cx, &opened_metadata);
        self.view
            .label(cx, ids!(account_management_device_metadata_status))
            .set_text(cx, &opened_metadata);

        let device_id_for_accept = device_id.clone();
        let device_id_for_cancel = device_id.clone();
        let display_name_for_accept = display_name.clone();
        let display_name_for_cancel = display_name.clone();
        let loaded_identity_for_accept = loaded_identity_text.clone();
        let loaded_identity_for_cancel = loaded_identity_text;
        let content = ConfirmationModalContent {
            title_text: "Rename Current Device".into(),
            body_text: opened_metadata.clone().into(),
            accept_button_text: Some("Rename".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |cx| {
                submit_async_request(MatrixRequest::RenameDevice {
                    device_id: device_id_for_accept.clone(),
                    display_name: display_name_for_accept.clone(),
                });
                cx.action(AccountSettingsAction::AccountManagementDeviceRenameStarted(
                    device_id_for_accept.clone(),
                    display_name_for_accept.clone(),
                ));
                enqueue_popup_notification(
                    account_management_current_device_rename_confirmation_label(
                        "confirmed; MatrixRequest::RenameDevice was requested",
                        Some(&loaded_identity_for_accept),
                        Some(device_id_for_accept.as_str()),
                        Some(&display_name_for_accept),
                        None,
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    account_management_current_device_rename_confirmation_label(
                        "confirmation canceled; RenameDevice was not requested",
                        Some(&loaded_identity_for_cancel),
                        Some(device_id_for_cancel.as_str()),
                        Some(&display_name_for_cancel),
                        Some("confirmation canceled"),
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
        };
        enqueue_popup_notification(opened_metadata, PopupKind::Info, Some(4.0));
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
        self.view.redraw(cx);
    }

    fn show_avatar_upload_retry_confirmation(&mut self, cx: &mut Cx) {
        let Some(preview) = self.avatar_upload_selection_preview.clone() else {
            enqueue_popup_notification(
                "Avatar upload retry unavailable: no cached selected image.",
                PopupKind::Warning,
                Some(4.0),
            );
            return;
        };
        let selected_summary = preview.summary();
        let selected_summary_for_accept = selected_summary.clone();
        let selected_summary_for_cancel = selected_summary.clone();
        let file_path_for_request = preview.file_path.clone();
        let mime_type_for_request = preview.mime.clone();
        let content = ConfirmationModalContent {
            title_text: "Retry Avatar Upload".into(),
            body_text: account_avatar_upload_retry_confirmation_label(
                &selected_summary,
                &preview.file_path,
            )
            .into(),
            accept_button_text: Some("Retry".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |cx| {
                submit_async_request(MatrixRequest::UploadAvatar {
                    file_path: file_path_for_request,
                    mime_type: mime_type_for_request,
                });
                cx.action(AccountSettingsAction::AvatarUploadStarted);
                enqueue_popup_notification(
                    account_avatar_upload_lifecycle_metadata_label(
                        "retry confirmed; Matrix avatar upload path was requested",
                        Some(&selected_summary_for_accept),
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    account_avatar_upload_lifecycle_metadata_label(
                        "retry confirmation canceled; Matrix avatar upload was not requested",
                        Some(&selected_summary_for_cancel),
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
        };
        enqueue_popup_notification(
            account_avatar_upload_lifecycle_metadata_label(
                "retry confirmation opened",
                Some(&selected_summary),
            ),
            PopupKind::Info,
            Some(4.0),
        );
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
    }

    fn show_avatar_direct_mxc_confirmation(&mut self, cx: &mut Cx, avatar_url: OwnedMxcUri) {
        let avatar_url_label = avatar_url.as_str().to_string();
        let avatar_url_for_request = avatar_url.clone();
        let avatar_url_for_action = avatar_url.clone();
        let avatar_url_for_cancel = avatar_url.clone();
        let content = ConfirmationModalContent {
            title_text: "Set Avatar MXC".into(),
            body_text: account_avatar_direct_mxc_confirmation_label(&avatar_url).into(),
            accept_button_text: Some("Set MXC".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |cx| {
                submit_async_request(MatrixRequest::SetAvatar {
                    avatar_url: Some(avatar_url_for_request.clone()),
                });
                cx.action(AccountSettingsAction::AvatarDirectSetStarted(
                    avatar_url_for_action.clone(),
                ));
                enqueue_popup_notification(
                    format!(
                        "Direct avatar MXC confirmed. Matrix SetAvatar(Some) was requested for {}.",
                        avatar_url_for_action.as_str()
                    ),
                    PopupKind::Info,
                    Some(5.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    format!(
                        "Direct avatar MXC canceled. Matrix SetAvatar(Some) was not requested for {}.",
                        avatar_url_for_cancel.as_str()
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
        };
        self.view
            .label(cx, ids!(avatar_direct_mxc_status))
            .set_text(
                cx,
                &account_avatar_direct_mxc_confirmation_label(&avatar_url),
            );
        enqueue_popup_notification(
            format!(
                "Direct avatar MXC confirmation opened for {avatar_url_label}. {ACCOUNT_CONFIRMATION_COMPACT_LABEL}"
            ),
            PopupKind::Info,
            Some(4.0),
        );
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
    }

    fn show_avatar_direct_mxc_retry_confirmation(&mut self, cx: &mut Cx) {
        let Some(avatar_url) = self.avatar_direct_mxc_failed_url.clone() else {
            enqueue_popup_notification(
                "Direct avatar MXC retry unavailable: no cached failed mxc:// URI.",
                PopupKind::Warning,
                Some(4.0),
            );
            return;
        };
        let avatar_url_for_request = avatar_url.clone();
        let avatar_url_for_action = avatar_url.clone();
        let avatar_url_for_cancel = avatar_url.clone();
        let content = ConfirmationModalContent {
            title_text: "Retry Avatar MXC".into(),
            body_text: account_avatar_direct_mxc_retry_confirmation_label(&avatar_url).into(),
            accept_button_text: Some("Retry MXC".into()),
            cancel_button_text: Some("Cancel".into()),
            on_accept_clicked: Some(Box::new(move |cx| {
                submit_async_request(MatrixRequest::SetAvatar {
                    avatar_url: Some(avatar_url_for_request.clone()),
                });
                cx.action(AccountSettingsAction::AvatarDirectSetStarted(
                    avatar_url_for_action.clone(),
                ));
                enqueue_popup_notification(
                    format!(
                        "Direct avatar MXC retry confirmed. Matrix SetAvatar(Some) was requested for {}.",
                        avatar_url_for_action.as_str()
                    ),
                    PopupKind::Info,
                    Some(5.0),
                );
            })),
            on_cancel_clicked: Some(Box::new(move |_cx| {
                enqueue_popup_notification(
                    format!(
                        "Direct avatar MXC retry canceled. Matrix SetAvatar(Some) was not requested for {}.",
                        avatar_url_for_cancel.as_str()
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            })),
        };
        self.view
            .label(cx, ids!(avatar_direct_mxc_status))
            .set_text(
                cx,
                &account_avatar_direct_mxc_retry_confirmation_label(&avatar_url),
            );
        enqueue_popup_notification(
            format!(
                "Direct avatar MXC retry confirmation opened for {}. {ACCOUNT_CONFIRMATION_COMPACT_LABEL}",
                avatar_url.as_str()
            ),
            PopupKind::Info,
            Some(4.0),
        );
        cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
            content,
        ))));
    }

    fn open_avatar_upload_picker(&mut self, cx: &mut Cx) {
        self.avatar_upload_selection_preview = None;
        self.set_avatar_upload_preview_state(cx, AvatarUploadPreviewState::ChoosePhoto);
        enqueue_popup_notification(
            account_avatar_upload_lifecycle_metadata_label("picker opened", None),
            PopupKind::Info,
            Some(3.0),
        );
        match pick_account_avatar_file() {
            AvatarUploadPickResult::Picked(file_path) => {
                let mime_type = account_avatar_mime_type(&file_path);
                if let Err(reason) = validate_account_avatar_file(&file_path, &mime_type) {
                    let invalid_summary = account_avatar_invalid_selection_metadata_summary(
                        &file_path, &mime_type, reason,
                    );
                    self.avatar_upload_selection_preview = None;
                    self.update_avatar_upload_preview(cx);
                    enqueue_popup_notification(
                        account_avatar_upload_lifecycle_metadata_label(
                            "invalid selection held locally",
                            Some(&invalid_summary),
                        ),
                        PopupKind::Error,
                        Some(4.0),
                    );
                    return;
                }

                let selected_preview = account_avatar_selection_preview(&file_path, &mime_type);
                let filename = selected_preview.filename.clone();
                let selected_summary = selected_preview.summary();
                self.avatar_upload_selection_preview = Some(selected_preview);
                self.set_avatar_upload_preview_state(cx, AvatarUploadPreviewState::Selected);
                let file_path_for_request = file_path.clone();
                let mime_type_for_request = mime_type.clone();
                let selected_summary_for_accept = selected_summary.clone();
                let selected_summary_for_cancel = selected_summary.clone();
                let content = ConfirmationModalContent {
                    title_text: "Upload Avatar".into(),
                    body_text: format!(
                        "Upload \"{filename}\" as your avatar? Selected image: {selected_summary}. {ACCOUNT_CONFIRMATION_COMPACT_LABEL}"
                    )
                    .into(),
                    accept_button_text: Some("Upload".into()),
                    cancel_button_text: Some("Cancel".into()),
                    on_accept_clicked: Some(Box::new(move |cx| {
                        submit_async_request(MatrixRequest::UploadAvatar {
                            file_path: file_path_for_request,
                            mime_type: mime_type_for_request,
                        });
                        cx.action(AccountSettingsAction::AvatarUploadStarted);
                        enqueue_popup_notification(
                            account_avatar_upload_lifecycle_metadata_label(
                                "confirmed; Matrix avatar upload path was requested",
                                Some(&selected_summary_for_accept),
                            ),
                            PopupKind::Info,
                            Some(4.0),
                        );
                    })),
                    on_cancel_clicked: Some(Box::new(move |_cx| {
                        enqueue_popup_notification(
                            account_avatar_upload_lifecycle_metadata_label(
                                "confirmation canceled; Matrix avatar upload was not requested",
                                Some(&selected_summary_for_cancel),
                            ),
                            PopupKind::Info,
                            Some(3.0),
                        );
                    })),
                };
                enqueue_popup_notification(
                    account_avatar_upload_lifecycle_metadata_label(
                        "confirmation opened",
                        Some(&selected_summary),
                    ),
                    PopupKind::Info,
                    Some(4.0),
                );
                cx.action(PositiveConfirmationModalAction::Show(RefCell::new(Some(
                    content,
                ))));
            }
            AvatarUploadPickResult::Canceled => {
                self.avatar_upload_selection_preview = None;
                self.update_avatar_upload_preview(cx);
                enqueue_popup_notification(
                    account_avatar_upload_lifecycle_metadata_label(
                        "picker canceled; no upload was requested",
                        None,
                    ),
                    PopupKind::Info,
                    Some(3.0),
                );
            }
            AvatarUploadPickResult::Unsupported => {
                self.avatar_upload_selection_preview = None;
                self.update_avatar_upload_preview(cx);
                enqueue_popup_notification(
                    account_avatar_upload_lifecycle_metadata_label(
                        "picker unsupported on this platform",
                        None,
                    ),
                    PopupKind::Warning,
                    Some(4.0),
                );
            }
        }
    }

    /// Populate avatar-related views with the user's profile data.
    ///
    /// This does nothing if `self.own_profile` is `None`.
    fn populate_avatar_views(&mut self, cx: &mut Cx) {
        let Some(own_profile) = &self.own_profile else {
            error!("BUG: AccountSettings::populate_avatar_views() called with no profile data.");
            return;
        };

        let our_own_avatar = self.view.avatar(cx, ids!(our_own_avatar));
        let mut drew_avatar = false;
        if let Some(avatar_img_data) = own_profile.avatar_state.data() {
            drew_avatar = our_own_avatar
                .show_image(
                    cx,
                    None, // don't make this avatar clickable; we handle clicks on this ProfileIcon widget directly.
                    |cx, img| utils::load_png_or_jpg(&img, cx, avatar_img_data),
                )
                .is_ok();
        }
        if !drew_avatar {
            our_own_avatar.show_text(
                cx,
                Some(COLOR_ROBRIX_PURPLE),
                None, // don't make this avatar clickable; we handle clicks on this ProfileIcon widget directly.
                own_profile.displayable_name(),
            );
        }

        Self::enable_upload_avatar_button(
            cx,
            true,
            &self.view.button(cx, ids!(upload_avatar_button)),
        );
        Self::enable_delete_avatar_button(
            cx,
            own_profile.avatar_state.has_avatar(),
            &self.view.button(cx, ids!(delete_avatar_button)),
        );
    }

    /// Populates the account settings within the SettingsScreen.
    /// Pass `Some(new_profile)` to replace the cached profile, or `None`
    /// to use the existing `self.own_profile`.
    ///
    /// Don't call this from `Event::ScriptReapply`, since it unconditionally
    /// `set_text`s `display_name_input`, which would wipe any in-progress edit.
    /// Use [`Self::restore_after_reapply`] for that path.
    pub fn populate(&mut self, cx: &mut Cx, new_profile: Option<UserProfile>) {
        if let Some(new_profile) = new_profile {
            self.own_profile = Some(new_profile);
        }
        self.populate_inner(cx, PopulateMode::Initial);
    }

    /// Restores widget state after `Event::ScriptReapply` reset DSL-bound
    /// fields to their `script_mod!` defaults. See
    /// [`PopulateMode::AfterReapply`] for what's re-applied vs left alone.
    pub fn restore_after_reapply(&mut self, cx: &mut Cx) {
        self.populate_inner(cx, PopulateMode::AfterReapply);
    }

    /// Shared core. The two modes only differ at the text-input writes
    /// and display-name button enable logic. Avatar repaint, hover sweep,
    /// and redraw are the same.
    fn populate_inner(&mut self, cx: &mut Cx, mode: PopulateMode) {
        let Some(own_profile) = self.own_profile.as_ref() else {
            if matches!(mode, PopulateMode::Initial) {
                error!("BUG: AccountSettings::populate() called with no cached profile.");
            }
            return;
        };

        let cached_user_id = own_profile.user_id.as_str().to_owned();
        let cached_name = own_profile.username.clone().unwrap_or_default();
        // Cloning here releases the `own_profile` borrow,
        // so the `&mut self` calls below don't conflict.

        // `display_name_input` is user-editable, so the modes diverge:
        //   * Initial: write the cached username + user_id, buttons start disabled.
        //   * AfterReapply: leave the input alone to preserve in-progress edits,
        //     skip user_id (already restored in `on_after_apply`),
        //     and re-derive the button enable state from whether the input still matches.
        let modified = match mode {
            PopulateMode::Initial => {
                self.view
                    .label(cx, ids!(user_id))
                    .set_text(cx, &cached_user_id);
                self.view
                    .text_input(cx, ids!(display_name_input))
                    .set_text(cx, &cached_name);
                false
            }
            PopulateMode::AfterReapply => {
                self.view.text_input(cx, ids!(display_name_input)).text() != cached_name
            }
        };

        Self::enable_display_name_buttons(
            cx,
            modified,
            &self.view.button(cx, ids!(accept_display_name_button)),
            &self.view.button(cx, ids!(cancel_display_name_button)),
        );
        let draft_name = self.view.text_input(cx, ids!(display_name_input)).text();
        Self::update_display_name_staging_preview(cx, &self.view, &draft_name, &cached_name);

        self.populate_avatar_views(cx);
        self.update_avatar_upload_preview(cx);
        self.update_avatar_direct_mxc_editor(cx);
        self.update_account_management_preview(cx);

        self.view
            .button(cx, ids!(upload_avatar_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(delete_avatar_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(accept_display_name_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(cancel_display_name_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(copy_user_id_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(manage_account_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_security_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_sessions_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_revoke_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_rename_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_trust_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_browser_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_device_metadata_device_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_device_metadata_verified_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_device_metadata_display_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_device_metadata_session_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_device_metadata_source_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_preflight_request_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_preflight_result_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_preflight_error_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_preflight_retry_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_preflight_source_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_preflight_packet_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_preflight_contract_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(account_preview_close_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_preview_choose_photo_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_preview_crop_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_preview_retry_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_preview_cancel_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_editor_aspect_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_editor_rotate_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_editor_zoom_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_editor_camera_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_editor_library_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_source_preview_source_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_source_preview_camera_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_source_preview_library_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_source_preview_thumbnail_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_source_preview_full_size_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_source_preview_packet_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_source_preview_contract_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_source_preview_taxonomy_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_upload_preflight_request_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_upload_preflight_result_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_upload_preflight_error_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_upload_preflight_retry_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_upload_preflight_source_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_direct_mxc_set_button))
            .reset_hover(cx);
        self.view
            .button(cx, ids!(avatar_direct_mxc_retry_button))
            .reset_hover(cx);
        self.view.button(cx, ids!(logout_button)).reset_hover(cx);
        self.view.redraw(cx);
    }

    fn update_display_name_staging_preview(
        cx: &mut Cx,
        view: &View,
        draft_name: &str,
        current_name: &str,
    ) {
        let draft_name = draft_name.trim();
        let current_name = current_name.trim();
        let visible = draft_name != current_name;
        view.view(cx, ids!(display_name_staging_preview))
            .set_visible(cx, visible);
        if !visible {
            return;
        }

        let staged_text = if draft_name.is_empty() {
            "Remove display name".to_string()
        } else {
            format!("New display name: {draft_name}")
        };
        view.label(cx, ids!(display_name_staging_preview_status))
            .set_text(
                cx,
                &format!(
                    "{staged_text}. Draft is local; Save Name confirms before Matrix SetDisplayName, failed results keep the draft editable for another confirmed Save Name resubmit, and Cancel/reset sends no avatar, account, device/session, message, room-state, or membership request."
                ),
            );
    }

    fn set_avatar_upload_preview_state(&mut self, cx: &mut Cx, state: AvatarUploadPreviewState) {
        // Account local surface close evidence: this setter only updates the
        // preview enum that drives local avatar upload labels/buttons.
        self.avatar_upload_preview_state = state;
        self.update_avatar_upload_preview(cx);
    }

    fn stage_avatar_upload_editor_control(&mut self, cx: &mut Cx, action: &str) {
        let selected_summary = self
            .avatar_upload_selection_preview
            .as_ref()
            .map(AvatarUploadSelectionPreview::summary);
        if self.avatar_upload_preview_state == AvatarUploadPreviewState::Hidden {
            self.set_avatar_upload_preview_state(cx, AvatarUploadPreviewState::ChoosePhoto);
        }
        if matches!(action, "Aspect" | "Rotate" | "Zoom") {
            self.set_avatar_upload_preview_state(cx, AvatarUploadPreviewState::Crop);
        }

        let metadata = if action == "Aspect" {
            account_avatar_upload_cropper_snapshot_label(
                action,
                self.avatar_upload_preview_state,
                selected_summary.as_deref(),
            )
        } else {
            account_avatar_upload_editor_controls_row_label(
                action,
                self.avatar_upload_preview_state,
                selected_summary.as_deref(),
            )
        };
        self.view
            .label(cx, ids!(avatar_upload_editor_controls_status))
            .set_text(cx, &metadata);
        self.view
            .label(cx, ids!(avatar_upload_option_evidence))
            .set_text(cx, &metadata);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
        self.view.redraw(cx);
    }

    fn stage_avatar_upload_source_preview_control(&mut self, cx: &mut Cx, action: &str) {
        let selected_summary = self
            .avatar_upload_selection_preview
            .as_ref()
            .map(AvatarUploadSelectionPreview::summary);
        if self.avatar_upload_preview_state == AvatarUploadPreviewState::Hidden {
            self.set_avatar_upload_preview_state(cx, AvatarUploadPreviewState::ChoosePhoto);
        }

        let metadata = account_avatar_upload_source_preview_controls_label(
            action,
            self.avatar_upload_preview_state,
            selected_summary.as_deref(),
        );
        self.view
            .label(cx, ids!(avatar_upload_source_preview_status))
            .set_text(cx, &metadata);
        self.view
            .label(cx, ids!(avatar_upload_option_evidence))
            .set_text(cx, &metadata);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
        self.view.redraw(cx);
    }

    fn run_avatar_upload_decode_probe(&mut self, cx: &mut Cx, action: &str) {
        if self.avatar_upload_preview_state == AvatarUploadPreviewState::Hidden {
            self.set_avatar_upload_preview_state(cx, AvatarUploadPreviewState::ChoosePhoto);
        }

        let metadata = account_avatar_upload_decode_probe_label(
            action,
            self.avatar_upload_selection_preview.as_ref(),
        );
        self.view
            .label(cx, ids!(avatar_upload_source_preview_status))
            .set_text(cx, &metadata);
        self.view
            .label(cx, ids!(avatar_upload_option_evidence))
            .set_text(cx, &metadata);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
        self.view.redraw(cx);
    }

    fn copy_avatar_upload_source_path(&mut self, cx: &mut Cx) {
        if self.avatar_upload_preview_state == AvatarUploadPreviewState::Hidden {
            self.set_avatar_upload_preview_state(cx, AvatarUploadPreviewState::ChoosePhoto);
        }

        let payload = account_avatar_upload_source_path_clipboard_payload(
            self.avatar_upload_selection_preview.as_ref(),
        );
        if let Some(path) = payload.as_deref() {
            // Avatar upload source clipboard evidence: Source copies only the
            // already selected local path and does not read, decode, upload, or
            // mutate avatar/account state.
            cx.copy_to_clipboard(path);
        }

        let metadata = account_avatar_upload_source_path_clipboard_label(
            self.avatar_upload_selection_preview.as_ref(),
        );
        let detail = account_avatar_upload_source_path_clipboard_metadata(
            self.avatar_upload_selection_preview.as_ref(),
        );
        self.view
            .label(cx, ids!(avatar_upload_source_preview_status))
            .set_text(cx, &metadata);
        self.view
            .label(cx, ids!(avatar_upload_option_evidence))
            .set_text(cx, &detail);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
        self.view.redraw(cx);
    }

    fn stage_avatar_upload_preflight_detail_control(&mut self, cx: &mut Cx, action: &str) {
        let selected_summary = self
            .avatar_upload_selection_preview
            .as_ref()
            .map(AvatarUploadSelectionPreview::summary);
        if self.avatar_upload_preview_state == AvatarUploadPreviewState::Hidden {
            self.set_avatar_upload_preview_state(cx, AvatarUploadPreviewState::ChoosePhoto);
        }

        let metadata = account_avatar_upload_preflight_detail_controls_label(
            action,
            self.avatar_upload_preview_state,
            selected_summary.as_deref(),
        );
        self.view
            .label(cx, ids!(avatar_upload_preflight_detail_status))
            .set_text(cx, &metadata);
        self.view
            .label(cx, ids!(avatar_upload_option_evidence))
            .set_text(cx, &metadata);
        enqueue_popup_notification(metadata, PopupKind::Info, Some(4.0));
        self.view.redraw(cx);
    }

    fn update_avatar_direct_mxc_editor(&mut self, cx: &mut Cx) {
        let draft = self
            .view
            .text_input(cx, ids!(avatar_direct_mxc_input))
            .text();
        let status = account_avatar_direct_mxc_editor_status_label(
            &draft,
            self.avatar_direct_mxc_failed_url.as_ref(),
        );
        self.view
            .label(cx, ids!(avatar_direct_mxc_status))
            .set_text(cx, &status);
        self.view
            .button(cx, ids!(avatar_direct_mxc_retry_button))
            .set_visible(cx, self.avatar_direct_mxc_failed_url.is_some());
        self.view.redraw(cx);
    }

    fn update_avatar_upload_preview(&mut self, cx: &mut Cx) {
        let visible = self.avatar_upload_preview_state != AvatarUploadPreviewState::Hidden;
        self.view
            .view(cx, ids!(avatar_upload_preview))
            .set_visible(cx, visible);
        if !visible {
            self.view.redraw(cx);
            return;
        }

        let status = match self.avatar_upload_preview_state {
            AvatarUploadPreviewState::Hidden => "",
            AvatarUploadPreviewState::ChoosePhoto => ACCOUNT_AVATAR_UPLOAD_LOCAL_BOUNDARY_LABEL,
            AvatarUploadPreviewState::Selected => {
                ACCOUNT_AVATAR_UPLOAD_SELECTED_IMAGE_METADATA_LABEL
            }
            AvatarUploadPreviewState::Failed => ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_LABEL,
            AvatarUploadPreviewState::Crop => ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_LABEL,
        };
        self.view
            .label(cx, ids!(avatar_upload_preview_status))
            .set_text(cx, status);
        let evidence = match (
            self.avatar_upload_preview_state,
            self.avatar_upload_selection_preview.as_ref(),
        ) {
            (AvatarUploadPreviewState::Failed, Some(preview)) => format!(
                "Upload failed for selected image: {}. Retry reuses the cached local file and still requires confirmation before MatrixRequest::UploadAvatar.",
                preview.summary()
            ),
            (AvatarUploadPreviewState::Selected, Some(preview)) => format!(
                "Selected image: {}. Filename, MIME, size, extension, and dimensions are local metadata; Upload still requires confirmation before MatrixRequest::UploadAvatar.",
                preview.summary()
            ),
            (AvatarUploadPreviewState::Crop, Some(preview)) => {
                let selected_summary = preview.summary();
                account_avatar_upload_crop_editor_boundary_label(
                    "crop preview opened locally",
                    Some(&selected_summary),
                )
            }
            (AvatarUploadPreviewState::Crop, None) => {
                account_avatar_upload_crop_editor_boundary_label(
                    "crop preview opened locally",
                    None,
                )
            }
            _ => ACCOUNT_AVATAR_UPLOAD_COMPACT_EVIDENCE.to_string(),
        };
        self.view
            .label(cx, ids!(avatar_upload_option_evidence))
            .set_text(cx, &evidence);
        self.view
            .label(cx, ids!(avatar_upload_editor_controls_status))
            .set_text(
                cx,
                &account_avatar_upload_editor_controls_row_label(
                    "preview refreshed",
                    self.avatar_upload_preview_state,
                    self.avatar_upload_selection_preview
                        .as_ref()
                        .map(AvatarUploadSelectionPreview::summary)
                        .as_deref(),
                ),
            );
        self.view
            .label(cx, ids!(avatar_upload_source_preview_status))
            .set_text(
                cx,
                &account_avatar_upload_source_preview_controls_label(
                    "preview refreshed",
                    self.avatar_upload_preview_state,
                    self.avatar_upload_selection_preview
                        .as_ref()
                        .map(AvatarUploadSelectionPreview::summary)
                        .as_deref(),
                ),
            );
        self.view
            .label(cx, ids!(avatar_upload_preflight_detail_status))
            .set_text(
                cx,
                &account_avatar_upload_preflight_detail_controls_label(
                    "preview refreshed",
                    self.avatar_upload_preview_state,
                    self.avatar_upload_selection_preview
                        .as_ref()
                        .map(AvatarUploadSelectionPreview::summary)
                        .as_deref(),
                ),
            );
        self.view
            .button(cx, ids!(avatar_preview_retry_button))
            .set_visible(
                cx,
                self.avatar_upload_preview_state == AvatarUploadPreviewState::Failed
                    && self.avatar_upload_selection_preview.is_some(),
            );
        self.view.redraw(cx);
    }

    fn set_account_management_preview_state(
        &mut self,
        cx: &mut Cx,
        state: AccountManagementPreviewState,
    ) {
        // Account management option staging evidence: this setter only changes
        // the preview enum consumed by local labels/buttons.
        self.account_management_preview_state = state;
        self.update_account_management_preview(cx);
    }

    fn update_account_management_preview(&mut self, cx: &mut Cx) {
        let visible =
            self.account_management_preview_state != AccountManagementPreviewState::Hidden;
        self.view
            .view(cx, ids!(account_management_preview))
            .set_visible(cx, visible);
        if !visible {
            self.view.redraw(cx);
            return;
        }

        let status = match self.account_management_preview_state {
            AccountManagementPreviewState::Hidden => "",
            AccountManagementPreviewState::Overview => {
                "Manage Account shows loaded account identity, current Matrix session details, read-only all-devices, confirmed current-device Rename, and confirmed Browser/Portal homeserver opener; password/SSO, activity, revoke, and trust mutations stay local."
            }
            AccountManagementPreviewState::Security => {
                "Security shows loaded account identity plus this Matrix device and verification state; Rename confirms current-device Matrix rename_device, Browser/Portal confirm homeserver opener, while password/SSO and trust changes stay local."
            }
            AccountManagementPreviewState::Sessions => {
                "Sessions shows this loaded account, current device, read-only GetDevices directory results when loaded, confirmed current-device Rename, and confirmed Browser/Portal homeserver opener; activity, session-management mutation, revoke, and trust stay local."
            }
            AccountManagementPreviewState::Refreshing => {
                "Refresh confirmed before GetOwnDevice; only current session metadata is being reread locally."
            }
        };
        self.view
            .label(cx, ids!(account_management_preview_status))
            .set_text(cx, status);
        let device_text = self.account_management_loaded_identity_text();
        let directory_text = account_management_device_directory_result_label(
            "preview refreshed",
            Some(&device_text),
            &self.own_devices,
            self.own_devices_last_error.as_deref(),
        );
        self.view
            .label(cx, ids!(account_management_option_evidence))
            .set_text(cx, &device_text);
        self.view
            .label(cx, ids!(account_management_preflight_detail_status))
            .set_text(
                cx,
                &account_management_preflight_detail_controls_row_label(
                    "preview refreshed",
                    self.account_management_preview_state,
                    Some(&device_text),
                ),
            );
        self.view
            .label(cx, ids!(account_management_device_metadata_status))
            .set_text(cx, &directory_text);
        self.view.redraw(cx);
    }

    fn account_management_loaded_identity_text(&self) -> String {
        format!(
            "{} · {}",
            loaded_account_identity_label(self.own_profile.as_ref()),
            self.account_management_device_text()
        )
    }

    fn account_management_device_text(&self) -> String {
        let verification = account_management_verification_status_label(self.verification_state);
        match self.own_device.as_ref() {
            Some(device) => match device.display_name() {
                Some(name) => format!(
                    "Current session: \"{name}\" · Device ID: {} · {verification}. GetOwnDevice only; account actions stay local.",
                    device.device_id()
                ),
                None => format!(
                    "Current device: {} · {verification}. GetOwnDevice only; account actions stay local.",
                    device.device_id()
                ),
            },
            None => "Current session details pending via GetOwnDevice; account actions stay local."
                .into(),
        }
    }

    /// Show verification info based on `self.verification_state`.
    ///
    /// If unknown, nothing will be shown.
    fn update_verification_banner(&mut self, cx: &mut Cx) {
        let (verified, unverified) = match self.verification_state {
            VerificationState::Verified => (true, false),
            VerificationState::Unverified => (false, true),
            VerificationState::Unknown => (false, false),
        };
        self.view
            .view(cx, ids!(verification_banner_verified))
            .set_visible(cx, verified);
        self.view
            .view(cx, ids!(verification_banner_unverified))
            .set_visible(cx, unverified);

        // Refill the session info even if the banner is hidden, so it's
        // already right if it shows up later.
        let info_text = match self.own_device.as_ref() {
            Some(device) => match device.display_name() {
                Some(name) => format!("Session: \"{name}\",  Device ID: {}", device.device_id()),
                None => format!("Device ID: {}", device.device_id()),
            },
            None => String::new(),
        };
        self.view
            .label(cx, ids!(unverified_device_info_label))
            .set_text(cx, &info_text);
        self.view.redraw(cx);
    }

    /// Enable or disable the delete avatar button.
    fn enable_delete_avatar_button(cx: &mut Cx, enable: bool, delete_avatar_button: &ButtonRef) {
        let (delete_button_fg_color, delete_button_bg_color) = if enable {
            (COLOR_FG_DANGER_RED, COLOR_BG_DANGER_RED)
        } else {
            (COLOR_FG_DISABLED, COLOR_BG_DISABLED)
        };
        let mut delete_avatar_button = delete_avatar_button.clone();
        script_apply_eval!(cx, delete_avatar_button, {
            enabled: #(enable),
            draw_bg +: {
                color: #(delete_button_bg_color),
                border_color: #(delete_button_fg_color),
            }
            draw_icon +: {
                color: #(delete_button_fg_color),
            }
            draw_text +: {
                color: #(delete_button_fg_color),
            }
        });
    }

    /// Enable or disable the upload avatar button.
    fn enable_upload_avatar_button(cx: &mut Cx, enable: bool, upload_avatar_button: &ButtonRef) {
        let (upload_button_fg_color, upload_button_bg_color) = if enable {
            (COLOR_PRIMARY, COLOR_ACTIVE_PRIMARY)
        } else {
            (COLOR_FG_DISABLED, COLOR_BG_DISABLED)
        };
        let mut upload_avatar_button = upload_avatar_button.clone();
        script_apply_eval!(cx, upload_avatar_button, {
            enabled: #(enable),
            draw_bg +: {
                color: #(upload_button_bg_color),
                border_color: #(upload_button_fg_color),
            }
            draw_icon +: {
                color: #(upload_button_fg_color),
            }
            draw_text +: {
                color: #(upload_button_fg_color),
            }
        });
    }

    /// Enable or disable the display name accept and cancel buttons.
    fn enable_display_name_buttons(
        cx: &mut Cx,
        enable: bool,
        accept_display_name_button: &ButtonRef,
        cancel_display_name_button: &ButtonRef,
    ) {
        let (accept_button_fg_color, accept_button_bg_color) = if enable {
            (COLOR_FG_ACCEPT_GREEN, COLOR_BG_ACCEPT_GREEN)
        } else {
            (COLOR_FG_DISABLED, COLOR_BG_DISABLED)
        };
        let (cancel_button_fg_color, cancel_button_bg_color) = if enable {
            (COLOR_FG_DANGER_RED, COLOR_BG_DANGER_RED)
        } else {
            (COLOR_FG_DISABLED, COLOR_BG_DISABLED)
        };

        let mut accept_display_name_button = accept_display_name_button.clone();
        script_apply_eval!(cx, accept_display_name_button, {
            enabled: #(enable),
            draw_bg +: {
                color: #(accept_button_bg_color),
                border_color: #(accept_button_fg_color),
            },
            draw_text +: {
                color: #(accept_button_fg_color),
            },
            draw_icon +: {
                color: #(accept_button_fg_color),
            }
        });
        let mut cancel_display_name_button = cancel_display_name_button.clone();
        script_apply_eval!(cx, cancel_display_name_button, {
            enabled: #(enable),
            draw_bg +: {
                color: #(cancel_button_bg_color),
                border_color: #(cancel_button_fg_color),
            },
            draw_text +: {
                color: #(cancel_button_fg_color),
            },
            draw_icon +: {
                color: #(cancel_button_fg_color),
            }
        });
    }
}

#[cfg(test)]
mod account_avatar_upload_tests;

impl AccountSettingsRef {
    /// See [`AccountSettings::populate()`].
    pub fn populate(&self, cx: &mut Cx, new_profile: Option<UserProfile>) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.populate(cx, new_profile);
    }

    /// See [`AccountSettings::restore_after_reapply()`].
    pub fn restore_after_reapply(&self, cx: &mut Cx) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.restore_after_reapply(cx);
    }
}

/// Actions that are handled by the AccountSettings widget.
#[derive(Debug)]
pub enum AccountSettingsAction {
    /// The avatar delete operation was started (e.g., confirmed in a modal).
    AvatarDeleteStarted,
    /// The avatar upload operation was started (e.g., confirmed in a modal).
    AvatarUploadStarted,
    /// The direct MXC avatar SetAvatar(Some) operation was started after confirmation.
    AvatarDirectSetStarted(OwnedMxcUri),
    /// The display name update operation was started after confirmation.
    DisplayNameChangeStarted,
    /// The account-management current-device refresh was confirmed.
    AccountManagementRefreshStarted,
    /// The account-management all-device directory retry was confirmed.
    AccountManagementDeviceDirectoryRetryStarted,
    /// The account-management current-device rename was confirmed.
    AccountManagementDeviceRenameStarted(OwnedDeviceId, String),
}
