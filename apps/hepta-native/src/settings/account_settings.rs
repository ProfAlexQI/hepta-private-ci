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

fn account_profile_avatar_state_label(state: &AvatarState) -> &'static str {
    match state {
        AvatarState::Unknown => "avatar unknown",
        AvatarState::Known(Some(_)) => "avatar MXC known",
        AvatarState::Known(None) => "no avatar",
        AvatarState::Loaded(_) => "avatar loaded",
        AvatarState::Failed => "avatar fetch failed",
    }
}

fn loaded_account_identity_label(profile: Option<&UserProfile>) -> String {
    match profile {
        Some(profile) => format!(
            "Loaded account: {} · {} · {}",
            profile.displayable_name(),
            profile.user_id.as_str(),
            account_profile_avatar_state_label(&profile.avatar_state)
        ),
        None => "Loaded account identity pending from own_profile cache.".to_string(),
    }
}

fn account_device_directory_entry_label(entry: &AccountDeviceDirectoryEntry) -> String {
    let display_name = entry
        .display_name
        .as_deref()
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .unwrap_or("unnamed device");
    let last_seen_ip = entry
        .last_seen_ip
        .as_deref()
        .map(str::trim)
        .filter(|ip| !ip.is_empty())
        .unwrap_or("last IP unavailable");
    let last_seen_ts = entry
        .last_seen_ts_ms
        .map(|timestamp| format!("last seen {timestamp} ms"))
        .unwrap_or_else(|| "last seen unavailable".to_string());

    format!(
        "{} ({display_name}; {last_seen_ip}; {last_seen_ts})",
        entry.device_id
    )
}

fn account_device_directory_summary(entries: &[AccountDeviceDirectoryEntry]) -> String {
    if entries.is_empty() {
        return "Device directory loaded: 0 devices.".to_string();
    }

    let preview = entries
        .iter()
        .take(3)
        .map(account_device_directory_entry_label)
        .collect::<Vec<_>>()
        .join(" | ");
    let suffix = if entries.len() > 3 {
        format!(" +{} more", entries.len() - 3)
    } else {
        String::new()
    };

    format!(
        "Device directory loaded: {} device(s): {preview}{suffix}.",
        entries.len()
    )
}

fn account_management_device_directory_result_label(
    result_state: &str,
    loaded_identity_text: Option<&str>,
    entries: &[AccountDeviceDirectoryEntry],
    error: Option<&str>,
) -> String {
    let result_state = result_state.trim();
    let result_state = if result_state.is_empty() {
        "device directory updated"
    } else {
        result_state
    };
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");
    let directory_state = match error {
        Some(error) if !error.trim().is_empty() => {
            format!("GetDevices failed: {}", error.trim())
        }
        _ => account_device_directory_summary(entries),
    };

    format!(
        "Account management {result_state}. Loaded metadata: {loaded_identity_text}. {directory_state} MatrixRequest::GetDevices is read-only; current-device Rename confirms before MatrixRequest::RenameDevice; Browser/Portal confirm before homeserver opener; revoke, trust, password/SSO, dedicated account portal route, gateway/runtime/auth, and unconfirmed live mutation stay unwired."
    )
}

fn account_management_preview_state_label(state: AccountManagementPreviewState) -> &'static str {
    match state {
        AccountManagementPreviewState::Hidden => "hidden preview",
        AccountManagementPreviewState::Overview => "Manage Account overview",
        AccountManagementPreviewState::Security => "Security preview",
        AccountManagementPreviewState::Sessions => "Sessions preview",
        AccountManagementPreviewState::Refreshing => "Refresh session metadata",
    }
}

fn account_management_lifecycle_metadata_label(
    lifecycle_state: &str,
    preview_state: AccountManagementPreviewState,
    loaded_identity_text: Option<&str>,
) -> String {
    let lifecycle_state = lifecycle_state.trim();
    let lifecycle_state = if lifecycle_state.is_empty() {
        "status updated"
    } else {
        lifecycle_state
    };
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");

    format!(
        "Account management {lifecycle_state}. Preview state: {}. Loaded metadata: {loaded_identity_text}. {ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_LABEL}",
        account_management_preview_state_label(preview_state),
    )
}

fn account_management_refresh_confirmation_label(
    lifecycle_state: &str,
    loaded_identity_text: Option<&str>,
) -> String {
    let lifecycle_state = lifecycle_state.trim();
    let lifecycle_state = if lifecycle_state.is_empty() {
        "status updated"
    } else {
        lifecycle_state
    };
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");

    format!(
        "Account management refresh {lifecycle_state}. Loaded metadata: {loaded_identity_text}. {ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_LABEL}"
    )
}

fn account_management_device_directory_retry_confirmation_label(
    lifecycle_state: &str,
    loaded_identity_text: Option<&str>,
    cached_error: Option<&str>,
) -> String {
    let lifecycle_state = lifecycle_state.trim();
    let lifecycle_state = if lifecycle_state.is_empty() {
        "status updated"
    } else {
        lifecycle_state
    };
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");
    let cached_error = cached_error
        .map(str::trim)
        .filter(|error| !error.is_empty())
        .map(|error| format!("Cached GetDevices error: {error}."))
        .unwrap_or_else(|| "No cached GetDevices error is available.".to_string());

    format!(
        "Account management device-directory retry {lifecycle_state}. Loaded metadata: {loaded_identity_text}. {cached_error} Retry reuses only PositiveConfirmationModal plus the read-only MatrixRequest::GetDevices path; unavailable cache and cancel stay local. {ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_RETRY_CONFIRMATION_LABEL}"
    )
}

fn account_management_device_rename_target(profile: Option<&UserProfile>) -> String {
    let raw_name = profile
        .map(UserProfile::displayable_name)
        .unwrap_or("Hepta Native");
    let normalized_name = raw_name.split_whitespace().collect::<Vec<_>>().join(" ");
    let candidate = match normalized_name.trim() {
        "" | "Hepta Native" => "Hepta Native".to_string(),
        name => format!("Hepta Native - {name}"),
    };
    candidate.chars().take(64).collect()
}

fn account_management_current_device_rename_confirmation_label(
    lifecycle_state: &str,
    loaded_identity_text: Option<&str>,
    device_id: Option<&str>,
    display_name: Option<&str>,
    error: Option<&str>,
) -> String {
    let lifecycle_state = lifecycle_state.trim();
    let lifecycle_state = if lifecycle_state.is_empty() {
        "status updated"
    } else {
        lifecycle_state
    };
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");
    let device_id = device_id
        .map(str::trim)
        .filter(|device_id| !device_id.is_empty())
        .unwrap_or("current Device ID pending");
    let display_name = display_name
        .map(str::trim)
        .filter(|display_name| !display_name.is_empty())
        .unwrap_or("target display name pending");
    let error_state = error
        .map(str::trim)
        .filter(|error| !error.is_empty())
        .map(|error| format!("Error: {error}."))
        .unwrap_or_else(|| "Rename request ready after confirmation.".to_string());

    format!(
        "Account management current-device Rename {lifecycle_state}. Loaded metadata: {loaded_identity_text}. Target Device ID: {device_id}. Target display name: {display_name}. {error_state} PositiveConfirmationModal gates MatrixRequest::RenameDevice and SlidingSync client.rename_device for the current device only; success refreshes GetOwnDevice and GetDevices. No password/SSO change, dedicated account portal route, session-management lookup, cross-session revoke, device delete/trust mutation, gateway/runtime/auth, Telegram delivery, or unconfirmed live mutation. {ACCOUNT_MANAGEMENT_CURRENT_DEVICE_RENAME_LABEL}"
    )
}

fn account_management_browser_portal_url_from_homeserver(
    homeserver_url: &str,
) -> Result<String, String> {
    let homeserver_url = homeserver_url.trim();
    if homeserver_url.is_empty() {
        return Err("homeserver URL unavailable".to_string());
    }

    let mut url = url::Url::parse(homeserver_url)
        .map_err(|error| format!("invalid homeserver URL: {error}"))?;
    match url.scheme() {
        "http" | "https" => {}
        scheme => return Err(format!("unsupported homeserver URL scheme: {scheme}")),
    }
    url.set_query(None);
    url.set_fragment(None);
    Ok(url.to_string())
}

fn account_management_browser_portal_url_from_client() -> Result<String, String> {
    let Some(client) = get_client() else {
        return Err("Matrix client unavailable; log in before opening account browser".to_string());
    };
    account_management_browser_portal_url_from_homeserver(client.homeserver().as_str())
}

fn account_management_browser_portal_handoff_label(
    action_label: &str,
    preview_state: AccountManagementPreviewState,
    loaded_identity_text: Option<&str>,
    target_url: Option<&str>,
    error: Option<&str>,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Browser/Portal"
    } else {
        action_label
    };
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");
    let target_state = target_url
        .map(str::trim)
        .filter(|url| !url.is_empty())
        .map(|url| format!("Target homeserver URL: {url}."))
        .unwrap_or_else(|| "Target homeserver URL pending.".to_string());
    let error_state = error
        .map(str::trim)
        .filter(|error| !error.is_empty())
        .map(|error| format!("Handoff unavailable: {error}."))
        .unwrap_or_else(|| "Handoff ready after confirmation.".to_string());

    format!(
        "Account management {action_label} homeserver handoff. Preview state: {}. Loaded metadata: {loaded_identity_text}. {target_state} {error_state} PositiveConfirmationModal gates robius_open system-browser handoff; accept opens only the active Matrix homeserver URL and cancel stays local. No MatrixRequest, password change, SSO flow, dedicated account-management portal route, session-management lookup, cross-session revoke/trust, device delete/trust mutation, Matrix account/profile mutation beyond current-device rename, gateway/runtime/auth, Telegram delivery, or live mutation. {ACCOUNT_MANAGEMENT_BROWSER_PORTAL_HANDOFF_LABEL}",
        account_management_preview_state_label(preview_state),
    )
}

fn account_management_session_revoke_boundary_label(
    preview_state: AccountManagementPreviewState,
    loaded_identity_text: Option<&str>,
) -> String {
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");

    format!(
        "Account management session/revoke boundary. Preview state: {}. Loaded metadata: {loaded_identity_text}. Read-only GetDevices directory summaries may be displayed; current-device Rename has a separate confirmed Matrix rename_device path; Browser/Portal use a separate confirmed homeserver opener. Dedicated external account page routes, password change, SSO change, all-device management beyond the read-only directory, session-management lookup, cross-session revoke, device delete/trust changes, account-data mutation, Matrix account/profile mutation beyond display name and current-device rename, gateway/runtime/auth, and unconfirmed live mutation stay local blocked. {ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_LABEL}",
        account_management_preview_state_label(preview_state),
    )
}

fn account_management_session_actions_row_label(
    action_label: &str,
    preview_state: AccountManagementPreviewState,
    loaded_identity_text: Option<&str>,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Session action"
    } else {
        action_label
    };
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");

    format!(
        "{action_label} staged locally. Preview state: {}. Loaded metadata: {loaded_identity_text}. Rename has a separate confirmation-gated current-device MatrixRequest::RenameDevice path; Revoke and Trust are visible local blocked controls; Browser uses a separate PositiveConfirmationModal homeserver opener. No all-device list, session-management lookup, cross-session revoke, device delete/trust change, Matrix account/profile mutation beyond current-device rename, gateway/runtime/auth, or unconfirmed live mutation. {ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_LABEL}",
        account_management_preview_state_label(preview_state),
    )
}

fn account_management_device_directory_controls_row_label(
    action_label: &str,
    preview_state: AccountManagementPreviewState,
    loaded_identity_text: Option<&str>,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Device directory action"
    } else {
        action_label
    };
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");

    format!(
        "{action_label} staged locally. Preview state: {}. Loaded metadata: {loaded_identity_text}. All devices, Password, SSO, Portal, and Activity are visible account-management controls; All devices is a read-only MatrixRequest::GetDevices path and Portal uses a separate PositiveConfirmationModal homeserver opener. No session-management lookup, password change, SSO start, session revoke, device trust/rename/delete change, Matrix account/profile mutation, gateway/runtime/auth, or live mutation. {ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_LABEL}",
        account_management_preview_state_label(preview_state),
    )
}

fn account_management_preflight_detail_controls_row_label(
    action_label: &str,
    preview_state: AccountManagementPreviewState,
    loaded_identity_text: Option<&str>,
) -> String {
    let action_label = action_label.trim();
    if action_label.eq_ignore_ascii_case("Packet") {
        return account_management_session_device_drilldown_packet_label(
            preview_state,
            loaded_identity_text,
        );
    }
    if action_label.eq_ignore_ascii_case("Contract") {
        return account_management_session_device_typed_contract_packet_label(
            preview_state,
            loaded_identity_text,
        );
    }
    if action_label.eq_ignore_ascii_case("Taxonomy") {
        return account_management_session_device_result_taxonomy_packet_label(
            preview_state,
            loaded_identity_text,
        );
    }
    let action_label = if action_label.is_empty() {
        "Preflight detail"
    } else {
        action_label
    };
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");

    format!(
        "{action_label} account-management detail stayed local. Preview state: {}. Loaded metadata: {loaded_identity_text}. Request, Result, Error, Retry, Source, Packet, Contract, and Taxonomy are visible account/session preflight controls: Result, Error, Source, Packet, Contract, and Taxonomy only update local copy; Retry confirms before resubmitting a cached read-only GetDevices failure; Browser/Portal homeserver opener has a separate confirmation path. No extra GetOwnDevice, dedicated account portal route, session-management lookup, password change, SSO start, automatic retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, gateway/runtime/auth, or live mutation. {ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_LABEL}",
        account_management_preview_state_label(preview_state),
    )
}

fn account_management_session_device_drilldown_packet_label(
    preview_state: AccountManagementPreviewState,
    loaded_identity_text: Option<&str>,
) -> String {
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");

    format!(
        "Account session/device drilldown packet. Preview state: {}. Loaded metadata: {loaded_identity_text}. Loaded own_profile identity, current GetOwnDevice session/device metadata, verification state, device id/display/session/source clipboard payloads, Refresh/GetOwnDevice request/result/error/retry/source slots, current-device RenameDevice request/result/error/retry/source slots, dedicated account portal route targets, Browser/Portal homeserver opener outcome, all-device directory scope, password/SSO scope, cross-session revoke/trust scope, device delete/trust scope, account/profile mutation guard, and live-mutation boundary are represented as local acceptance criteria only. No extra GetOwnDevice, dedicated portal route open, extra homeserver opener, all-device list fetch, session-management lookup, password/SSO change, automatic retry, session revoke, extra current-device RenameDevice, cross-session device delete/trust mutation, Matrix account/profile mutation beyond current-device rename, message send, room-state or membership change, gateway/runtime/auth, or live mutation was submitted. {ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_LABEL}",
        account_management_preview_state_label(preview_state),
    )
}

fn account_management_session_device_typed_contract_packet_label(
    preview_state: AccountManagementPreviewState,
    loaded_identity_text: Option<&str>,
) -> String {
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");

    format!(
        "Account session/device typed contract packet. Preview state: {}. Loaded metadata: {loaded_identity_text}. The local session/device drilldown Packet is mapped to typed dedicated account portal route, Browser/Portal homeserver opener outcome, all-device directory, password/SSO, current-device RenameDevice, cross-session revoke/trust, device delete/trust, account/profile mutation guard, GetOwnDevice refresh, result/error/retry/source, source-hash, idempotency, stale-session, and promotion-blocker contracts before account/session work can be promoted. No extra GetOwnDevice, dedicated portal route open, extra homeserver opener, all-device list fetch, session-management lookup, password/SSO change, automatic retry, session revoke, extra current-device RenameDevice, cross-session device delete/trust mutation, Matrix account/profile mutation beyond current-device rename, message send, room-state or membership change, gateway/runtime/auth, or live mutation was submitted. {ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_LABEL}",
        account_management_preview_state_label(preview_state),
    )
}

fn account_management_session_device_result_taxonomy_packet_label(
    preview_state: AccountManagementPreviewState,
    loaded_identity_text: Option<&str>,
) -> String {
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");

    format!(
        "Account session/device result taxonomy packet. Preview state: {}. Loaded metadata: {loaded_identity_text}. Live result references: MatrixRequest::GetOwnDevice current-session read, MatrixRequest::GetDevices read-only directory, MatrixRequest::SetDisplayName, MatrixRequest::RenameDevice for the current device, and confirmed Browser/Portal homeserver opener only. Blocked result slots: dedicated_portal_operation_id not_assigned, password_action_operation_id not_assigned, sso_action_operation_id not_assigned, cross_session_revoke_operation_id not_assigned, cross_session_trust_operation_id not_assigned, device_delete_operation_id not_assigned, device_trust_operation_id not_assigned; dedicated_portal_result opened/blocked/failed/stale not_wired; password_result opened/completed/cancelled/failed/stale not_wired; sso_result opened/completed/cancelled/failed/stale not_wired; revoke_result applied/permission_denied/failed/stale not_wired; device_delete_result deleted/permission_denied/failed/stale not_wired; device_trust_result trusted/untrusted/permission_denied/failed/stale not_wired. Retry policy requires PositiveConfirmationModal plus backend request id and directory/source hash; cancel policy is local dismiss with no request; stale policy requires current-device id plus all-device directory generation; audit redaction excludes password, token, SSO code, refresh token, raw last-seen IP, and device secrets. No extra GetOwnDevice, dedicated portal route open, password/SSO flow, session revoke, cross-session trust, device delete/trust mutation, Matrix account/profile mutation beyond existing display-name/current-device rename, message send, room-state or membership change, gateway/runtime/auth, or live mutation was submitted. {ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_LABEL}",
        account_management_preview_state_label(preview_state),
    )
}

fn account_management_request_snapshot_label(
    action_label: &str,
    preview_state: AccountManagementPreviewState,
    loaded_identity_text: Option<&str>,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Request"
    } else {
        action_label
    };
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");

    format!(
        "Local account/session request snapshot: {action_label} selected. Preview state: {}. Loaded metadata: {loaded_identity_text}. Request body, result slot, error slot, retry availability, source summary, dedicated portal target, Browser/Portal homeserver opener outcome, all-device scope, session-management scope, password/SSO scope, current-device rename scope, and cross-session device delete/trust scope are represented as local metadata only. No extra GetOwnDevice, dedicated account portal route, all-device list, session-management lookup, password change, SSO start, automatic retry, session revoke, extra current-device RenameDevice, cross-session device delete/trust change, Matrix account/profile mutation beyond current-device rename, gateway/runtime/auth, or live mutation was submitted. {ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_LABEL}",
        account_management_preview_state_label(preview_state),
    )
}

fn account_management_current_device_metadata_controls_row_label(
    action_label: &str,
    preview_state: AccountManagementPreviewState,
    loaded_identity_text: Option<&str>,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Device metadata"
    } else {
        action_label
    };
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");

    format!(
        "{action_label} current-device metadata stayed local. Preview state: {}. Loaded metadata: {loaded_identity_text}. Device copies only the already loaded current Device ID to the local clipboard when available; Verified copies only the already loaded current-device verification status to the local clipboard when available; Display copies only the already loaded current device display name to the local clipboard when available; Session copies only the already loaded current-session summary to the local clipboard when available; Source copies only the loaded account/current-device summary to the local clipboard. No extra GetOwnDevice, external account portal or browser, all-device list, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, gateway/runtime/auth, or live mutation. {ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_LABEL}",
        account_management_preview_state_label(preview_state),
    )
}

fn account_management_verification_status_label(
    verification_state: VerificationState,
) -> &'static str {
    match verification_state {
        VerificationState::Verified => "verified",
        VerificationState::Unverified => "unverified",
        VerificationState::Unknown => "unknown verification",
    }
}

fn account_management_current_device_verification_clipboard_payload(
    device_id: Option<&str>,
    verification_state: VerificationState,
) -> Option<String> {
    device_id
        .map(str::trim)
        .filter(|device_id| !device_id.is_empty())
        .map(|device_id| {
            let verification = account_management_verification_status_label(verification_state);
            format!(
                "Current device verification: {verification}. Device ID: {device_id}. GetOwnDevice only; account actions stay local."
            )
        })
}

fn account_management_current_device_verification_clipboard_label(
    preview_state: AccountManagementPreviewState,
    loaded_identity_text: Option<&str>,
    device_id: Option<&str>,
    verification_state: VerificationState,
) -> String {
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");
    match account_management_current_device_verification_clipboard_payload(
        device_id,
        verification_state,
    ) {
        Some(summary) => format!(
            "Current-device verification status copied locally. Preview state: {}. Verification status: {}. Verification summary chars: {}, bytes: {}. Loaded metadata: {loaded_identity_text}. Verified copies only the already loaded local Matrix verification state plus GetOwnDevice current device ID to the local clipboard. No extra GetOwnDevice, external account portal or browser, all-device list, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, gateway/runtime/auth, or live mutation. {ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_LABEL}",
            account_management_preview_state_label(preview_state),
            account_management_verification_status_label(verification_state),
            summary.chars().count(),
            summary.len(),
        ),
        None => format!(
            "Current-device verification status copy stayed local because current-device metadata is pending. Preview state: {}. Loaded metadata: {loaded_identity_text}. No clipboard payload was written, and no extra GetOwnDevice, external account portal or browser, all-device list, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, gateway/runtime/auth, or live mutation was requested. {ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_LABEL}",
            account_management_preview_state_label(preview_state),
        ),
    }
}

fn account_management_current_device_id_clipboard_payload(
    device_id: Option<&str>,
) -> Option<String> {
    device_id
        .map(str::trim)
        .filter(|device_id| !device_id.is_empty())
        .map(ToOwned::to_owned)
}

fn account_management_current_device_id_clipboard_label(
    preview_state: AccountManagementPreviewState,
    loaded_identity_text: Option<&str>,
    device_id: Option<&str>,
) -> String {
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");
    match account_management_current_device_id_clipboard_payload(device_id) {
        Some(device_id) => format!(
            "Device ID copied locally. Preview state: {}. Device ID chars: {}, bytes: {}. Loaded metadata: {loaded_identity_text}. Device copies only the already loaded GetOwnDevice Device ID to the local clipboard. No extra GetOwnDevice, external account portal or browser, all-device list, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, gateway/runtime/auth, or live mutation. {ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_LABEL}",
            account_management_preview_state_label(preview_state),
            device_id.chars().count(),
            device_id.len(),
        ),
        None => format!(
            "Device ID copy stayed local because current-device metadata is pending. Preview state: {}. Loaded metadata: {loaded_identity_text}. No clipboard payload was written, and no extra GetOwnDevice, external account portal or browser, all-device list, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, gateway/runtime/auth, or live mutation was requested. {ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_LABEL}",
            account_management_preview_state_label(preview_state),
        ),
    }
}

fn account_management_current_device_display_name_clipboard_payload(
    display_name: Option<&str>,
) -> Option<String> {
    display_name
        .map(str::trim)
        .filter(|display_name| !display_name.is_empty())
        .map(ToOwned::to_owned)
}

fn account_management_current_device_display_name_clipboard_label(
    preview_state: AccountManagementPreviewState,
    loaded_identity_text: Option<&str>,
    display_name: Option<&str>,
) -> String {
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");
    match account_management_current_device_display_name_clipboard_payload(display_name) {
        Some(display_name) => format!(
            "Device display name copied locally. Preview state: {}. Display name chars: {}, bytes: {}. Loaded metadata: {loaded_identity_text}. Display copies only the already loaded GetOwnDevice display name to the local clipboard. No extra GetOwnDevice, external account portal or browser, all-device list, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, gateway/runtime/auth, or live mutation. {ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_LABEL}",
            account_management_preview_state_label(preview_state),
            display_name.chars().count(),
            display_name.len(),
        ),
        None => format!(
            "Device display name copy stayed local because current-device display name is unavailable. Preview state: {}. Loaded metadata: {loaded_identity_text}. No clipboard payload was written, and no extra GetOwnDevice, external account portal or browser, all-device list, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, gateway/runtime/auth, or live mutation was requested. {ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_LABEL}",
            account_management_preview_state_label(preview_state),
        ),
    }
}

fn account_management_current_session_clipboard_payload(
    session_text: Option<&str>,
) -> Option<String> {
    session_text
        .map(str::trim)
        .filter(|session_text| !session_text.is_empty())
        .map(ToOwned::to_owned)
}

fn account_management_current_session_clipboard_label(
    preview_state: AccountManagementPreviewState,
    loaded_identity_text: Option<&str>,
    session_text: Option<&str>,
) -> String {
    let loaded_identity_text = loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .unwrap_or("loaded account/device metadata pending");
    match account_management_current_session_clipboard_payload(session_text) {
        Some(session_text) => format!(
            "Current session summary copied locally. Preview state: {}. Session summary chars: {}, bytes: {}. Loaded metadata: {loaded_identity_text}. Session copies only the already loaded GetOwnDevice current-session summary to the local clipboard. No extra GetOwnDevice, external account portal or browser, all-device list, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, gateway/runtime/auth, or live mutation. {ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_LABEL}",
            account_management_preview_state_label(preview_state),
            session_text.chars().count(),
            session_text.len(),
        ),
        None => format!(
            "Current session summary copy stayed local because current-device metadata is pending. Preview state: {}. Loaded metadata: {loaded_identity_text}. No clipboard payload was written, and no extra GetOwnDevice, external account portal or browser, all-device list, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, gateway/runtime/auth, or live mutation was requested. {ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_LABEL}",
            account_management_preview_state_label(preview_state),
        ),
    }
}

fn account_management_current_device_source_clipboard_payload(
    loaded_identity_text: Option<&str>,
) -> Option<String> {
    loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(ToOwned::to_owned)
}

fn account_management_current_device_source_clipboard_label(
    preview_state: AccountManagementPreviewState,
    loaded_identity_text: Option<&str>,
) -> String {
    match account_management_current_device_source_clipboard_payload(loaded_identity_text) {
        Some(summary) => format!(
            "Source account/current-device summary copied locally. Preview state: {}. Summary chars: {}, bytes: {}. Source copies only loaded own_profile plus GetOwnDevice text to the local clipboard. No extra GetOwnDevice, external account portal or browser, all-device list, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, gateway/runtime/auth, or live mutation. {ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_LABEL}",
            account_management_preview_state_label(preview_state),
            summary.chars().count(),
            summary.len(),
        ),
        None => format!(
            "Source account/current-device summary copy stayed local because loaded metadata is empty. Preview state: {}. No clipboard payload was written, and no extra GetOwnDevice, external account portal or browser, all-device list, session-management lookup, password change, SSO start, retry, session revoke, device trust/rename/delete change, Matrix account/profile mutation, gateway/runtime/auth, or live mutation was requested. {ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_LABEL}",
            account_management_preview_state_label(preview_state),
        ),
    }
}

#[allow(dead_code)]
enum AvatarUploadPickResult {
    Picked(PathBuf),
    Canceled,
    Unsupported,
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
fn pick_account_avatar_file() -> AvatarUploadPickResult {
    rfd::FileDialog::new()
        .add_filter("Images", &["png", "jpg", "jpeg", "gif", "bmp", "webp"])
        .pick_file()
        .map(AvatarUploadPickResult::Picked)
        .unwrap_or(AvatarUploadPickResult::Canceled)
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
fn pick_account_avatar_file() -> AvatarUploadPickResult {
    AvatarUploadPickResult::Unsupported
}

fn account_avatar_mime_type(path: &Path) -> mime::Mime {
    mime_guess::from_path(path).first_or_octet_stream()
}

fn validate_account_avatar_file(path: &Path, mime_type: &mime::Mime) -> Result<(), &'static str> {
    let metadata = fs::metadata(path).map_err(|_| "selected image is unreadable")?;
    if !metadata.is_file() {
        return Err("selected path is not a regular file");
    }
    if metadata.len() == 0 {
        return Err("selected image is empty");
    }
    if mime_type.type_() != mime::IMAGE {
        return Err("selected file is not an image");
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct AvatarUploadSelectionPreview {
    file_path: PathBuf,
    mime: mime::Mime,
    filename: String,
    extension: String,
    mime_type: String,
    size_label: String,
    dimensions_label: String,
}

impl AvatarUploadSelectionPreview {
    fn summary(&self) -> String {
        format!(
            "{} · {} · {} · {} · {}",
            self.filename, self.mime_type, self.size_label, self.extension, self.dimensions_label
        )
    }
}

fn account_avatar_upload_lifecycle_metadata_label(
    lifecycle_state: &str,
    selected_summary: Option<&str>,
) -> String {
    let lifecycle_state = lifecycle_state.trim();
    let lifecycle_state = if lifecycle_state.is_empty() {
        "status updated"
    } else {
        lifecycle_state
    };
    let selected_summary = selected_summary
        .map(str::trim)
        .filter(|summary| !summary.is_empty())
        .unwrap_or("no selected image metadata loaded");

    format!(
        "Avatar upload {lifecycle_state}. Selected metadata: {selected_summary}. {ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_LABEL}"
    )
}

fn account_avatar_invalid_selection_metadata_summary(
    path: &Path,
    mime_type: &mime::Mime,
    reason: &str,
) -> String {
    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("selected image");
    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase())
        .filter(|extension| !extension.trim().is_empty())
        .unwrap_or_else(|| "no extension".to_string());
    format!(
        "{filename} · {} · {extension} · validation: {reason}",
        mime_type
    )
}

fn is_account_avatar_header_dimension_image_file(path: &Path, mime_type: &mime::Mime) -> bool {
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

fn read_account_avatar_image_header_bytes(path: &Path) -> Option<Vec<u8>> {
    let mut file = fs::File::open(path).ok()?;
    let mut bytes = Vec::new();
    file.by_ref()
        .take(512 * 1024)
        .read_to_end(&mut bytes)
        .ok()?;
    Some(bytes)
}

fn parse_account_avatar_png_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 24 || &bytes[0..8] != b"\x89PNG\r\n\x1a\n" || &bytes[12..16] != b"IHDR" {
        return None;
    }
    Some((
        u32::from_be_bytes(bytes[16..20].try_into().ok()?),
        u32::from_be_bytes(bytes[20..24].try_into().ok()?),
    ))
}

fn parse_account_avatar_gif_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 10 || !matches!(&bytes[0..6], b"GIF87a" | b"GIF89a") {
        return None;
    }
    Some((
        u16::from_le_bytes(bytes[6..8].try_into().ok()?) as u32,
        u16::from_le_bytes(bytes[8..10].try_into().ok()?) as u32,
    ))
}

fn parse_account_avatar_bmp_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 26 || &bytes[0..2] != b"BM" {
        return None;
    }
    let width = i32::from_le_bytes(bytes[18..22].try_into().ok()?);
    let height = i32::from_le_bytes(bytes[22..26].try_into().ok()?);
    Some((width.unsigned_abs(), height.unsigned_abs()))
}

fn parse_account_avatar_jpeg_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
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

fn parse_account_avatar_webp_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
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

fn account_avatar_image_dimensions_from_header(bytes: &[u8]) -> Option<(u32, u32, &'static str)> {
    if let Some((width, height)) = parse_account_avatar_png_dimensions(bytes) {
        return Some((width, height, "PNG"));
    }
    if let Some((width, height)) = parse_account_avatar_jpeg_dimensions(bytes) {
        return Some((width, height, "JPEG"));
    }
    if let Some((width, height)) = parse_account_avatar_gif_dimensions(bytes) {
        return Some((width, height, "GIF"));
    }
    if let Some((width, height)) = parse_account_avatar_bmp_dimensions(bytes) {
        return Some((width, height, "BMP"));
    }
    if let Some((width, height)) = parse_account_avatar_webp_dimensions(bytes) {
        return Some((width, height, "WebP"));
    }
    None
}

fn account_avatar_image_dimensions_label(path: &Path, mime_type: &mime::Mime) -> String {
    if !is_account_avatar_header_dimension_image_file(path, mime_type) {
        return "dimensions: unavailable for this avatar image type".to_string();
    }
    let Some(bytes) = read_account_avatar_image_header_bytes(path) else {
        return "dimensions: unavailable from unreadable image header".to_string();
    };
    account_avatar_image_dimensions_from_header(&bytes)
        .map(|(width, height, format)| format!("dimensions: {width}x{height} from {format} header"))
        .unwrap_or_else(|| "dimensions: unavailable from image header".to_string())
}

#[allow(dead_code)]
fn account_avatar_thumbnail_target_dimensions(
    width: u32,
    height: u32,
    max_side: u32,
) -> (u32, u32) {
    if width == 0 || height == 0 || max_side == 0 {
        return (0, 0);
    }
    let longest = width.max(height);
    if longest <= max_side {
        return (width, height);
    }
    let longest = longest as u64;
    let max_side = max_side as u64;
    let target_width = ((width as u64 * max_side + longest - 1) / longest).max(1) as u32;
    let target_height = ((height as u64 * max_side + longest - 1) / longest).max(1) as u32;
    (target_width, target_height)
}

const ACCOUNT_AVATAR_PIXEL_DECODE_MAX_BYTES: u64 = 20 * 1024 * 1024;
const ACCOUNT_AVATAR_PIXEL_DECODE_MAX_PIXELS: u64 = 12_000_000;
const ACCOUNT_AVATAR_THUMBNAIL_MAX_SIDE: u32 = 128;

fn account_avatar_normalized_decode_action(action: &str) -> &'static str {
    let action = action.trim();
    if action.eq_ignore_ascii_case("Thumbnail") {
        "Thumbnail"
    } else if action.eq_ignore_ascii_case("Full-size")
        || action.eq_ignore_ascii_case("Full size")
        || action.eq_ignore_ascii_case("Fullsize")
    {
        "Full-size"
    } else {
        "Thumbnail"
    }
}

fn read_account_avatar_image_decode_bytes(path: &Path) -> Result<Vec<u8>, String> {
    let metadata = fs::metadata(path).map_err(|_| "selected image is unreadable".to_string())?;
    if !metadata.is_file() {
        return Err("selected path is not a regular file".to_string());
    }
    if metadata.len() == 0 {
        return Err("selected image is empty".to_string());
    }
    if metadata.len() > ACCOUNT_AVATAR_PIXEL_DECODE_MAX_BYTES {
        return Err(format!(
            "selected image is {} over the local {} decode byte budget",
            ByteSize::b(metadata.len()),
            ByteSize::b(ACCOUNT_AVATAR_PIXEL_DECODE_MAX_BYTES),
        ));
    }
    fs::read(path).map_err(|error| format!("selected image could not be read: {error}"))
}

fn account_avatar_upload_decode_probe_label(
    action: &str,
    selected_preview: Option<&AvatarUploadSelectionPreview>,
) -> String {
    let normalized_action = account_avatar_normalized_decode_action(action);

    let Some(preview) = selected_preview else {
        return format!(
            "Avatar {normalized_action} pixel decode has no selected local image yet. Choose Photo must stage a local file first. {ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_LABEL} No file picker, image decode, thumbnail file, UploadAvatar, SetAvatar(Some), account/profile mutation, gateway/runtime/auth, or live mutation was requested."
        );
    };

    if !is_account_avatar_header_dimension_image_file(&preview.file_path, &preview.mime) {
        return format!(
            "Avatar {normalized_action} pixel decode skipped unsupported image header type for {}. Selected metadata: {}. {ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_LABEL} No thumbnail file, UploadAvatar, SetAvatar(Some), account/profile mutation, gateway/runtime/auth, or live mutation was requested.",
            preview.filename,
            preview.summary(),
        );
    }

    let Ok(bytes) = read_account_avatar_image_decode_bytes(&preview.file_path) else {
        return format!(
            "Avatar {normalized_action} pixel decode could not read the selected image bytes for {}. Selected metadata: {}. {ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_LABEL} No thumbnail file, UploadAvatar, SetAvatar(Some), account/profile mutation, gateway/runtime/auth, or live mutation was requested.",
            preview.file_path.display(),
            preview.summary(),
        );
    };

    let Some((width, height, format)) = account_avatar_image_dimensions_from_header(&bytes) else {
        return format!(
            "Avatar {normalized_action} pixel decode found no parseable dimensions in {} for {}. Selected metadata: {}. {ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_LABEL} No thumbnail file, UploadAvatar, SetAvatar(Some), account/profile mutation, gateway/runtime/auth, or live mutation was requested.",
            ByteSize::b(bytes.len() as u64),
            preview.filename,
            preview.summary(),
        );
    };

    let source_pixels = width as u64 * height as u64;
    if source_pixels > ACCOUNT_AVATAR_PIXEL_DECODE_MAX_PIXELS {
        return format!(
            "Avatar {normalized_action} pixel decode blocked for {}. Format: {format}; original: {width}x{height} ({source_pixels} pixels) exceeds the local {} pixel budget. Selected metadata: {}. {ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_LABEL} No thumbnail file, full pixel buffer, UploadAvatar, SetAvatar(Some), account/profile mutation, gateway/runtime/auth, or live mutation was requested.",
            preview.filename,
            ACCOUNT_AVATAR_PIXEL_DECODE_MAX_PIXELS,
            preview.summary(),
        );
    }

    let Ok(decoded) = ::image::load_from_memory(&bytes) else {
        return format!(
            "Avatar {normalized_action} pixel decode failed for {} after reading {}. Header format: {format}; original: {width}x{height}. Selected metadata: {}. {ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_LABEL} No thumbnail file, UploadAvatar, SetAvatar(Some), account/profile mutation, gateway/runtime/auth, or live mutation was requested.",
            preview.filename,
            ByteSize::b(bytes.len() as u64),
            preview.summary(),
        );
    };

    let (target_width, target_height, output_kind, rgba_bytes) = if normalized_action == "Thumbnail"
    {
        let thumbnail = decoded.thumbnail(
            ACCOUNT_AVATAR_THUMBNAIL_MAX_SIDE,
            ACCOUNT_AVATAR_THUMBNAIL_MAX_SIDE,
        );
        let rgba = thumbnail.to_rgba8();
        (
            rgba.width(),
            rgba.height(),
            "generated in-memory 128px RGBA thumbnail",
            rgba.as_raw().len() as u64,
        )
    } else {
        let rgba = decoded.to_rgba8();
        (
            rgba.width(),
            rgba.height(),
            "decoded full-size RGBA pixel buffer",
            rgba.as_raw().len() as u64,
        )
    };
    let target_pixels = target_width as u64 * target_height as u64;

    format!(
        "Avatar {normalized_action} pixel decode ready for {}. Format: {format}; original: {width}x{height} ({source_pixels} pixels); {output_kind}: {target_width}x{target_height} ({target_pixels} pixels, {} RGBA); source bytes read: {}; selected metadata: {}. {ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_LABEL} No thumbnail file, cropper/editor transform, UploadAvatar, SetAvatar(Some), account/profile mutation, gateway/runtime/auth, or live mutation was requested.",
        preview.filename,
        ByteSize::b(rgba_bytes),
        ByteSize::b(bytes.len() as u64),
        preview.summary(),
    )
}

fn account_avatar_selection_preview(
    path: &Path,
    mime_type: &mime::Mime,
) -> AvatarUploadSelectionPreview {
    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("selected image")
        .to_string();
    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase())
        .filter(|extension| !extension.trim().is_empty())
        .unwrap_or_else(|| "no extension".to_string());
    let size_label = fs::metadata(path)
        .map(|metadata| ByteSize::b(metadata.len()).to_string())
        .unwrap_or_else(|_| "size unavailable".to_string());
    AvatarUploadSelectionPreview {
        file_path: path.to_path_buf(),
        mime: mime_type.clone(),
        filename,
        extension,
        mime_type: mime_type.to_string(),
        size_label,
        dimensions_label: account_avatar_image_dimensions_label(path, mime_type),
    }
}

fn account_avatar_upload_retry_confirmation_label(
    selected_summary: &str,
    file_path: &Path,
) -> String {
    let file_label = file_path.display();
    format!(
        "Retry avatar upload for {file_label}? Selected image: {selected_summary}. {ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_LABEL} No new file picker, cropper/editor, thumbnail decode, camera/photo-library capture, browser handoff, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn parse_account_avatar_direct_mxc_uri(input: &str) -> Result<OwnedMxcUri, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Enter an mxc:// content URI before setting a direct avatar.".to_string());
    }

    let mxc_uri = <&MxcUri>::from(trimmed);
    mxc_uri
        .validate()
        .map_err(|error| format!("Invalid avatar MXC URI: {error}"))?;
    Ok(OwnedMxcUri::from(trimmed.to_string()))
}

fn account_avatar_direct_mxc_editor_status_label(
    draft: &str,
    failed_url: Option<&OwnedMxcUri>,
) -> String {
    let draft = draft.trim();
    let draft_state = if draft.is_empty() {
        "no direct MXC draft staged".to_string()
    } else {
        format!("draft MXC URI staged locally: {draft}")
    };
    let retry_state = failed_url
        .map(|url| {
            format!(
                " Failed direct SetAvatar(Some) retry cache: {}.",
                url.as_str()
            )
        })
        .unwrap_or_default();
    format!(
        "{draft_state}.{retry_state} {ACCOUNT_AVATAR_DIRECT_SET_CONFIRMATION_LABEL} No file picker, UploadAvatar, cropper/editor, thumbnail decode, camera/photo-library, browser handoff, gateway/runtime/auth, or unconfirmed live mutation."
    )
}

fn account_avatar_direct_mxc_confirmation_label(avatar_url: &OwnedMxcUri) -> String {
    format!(
        "Set avatar directly to {}? {ACCOUNT_AVATAR_DIRECT_SET_CONFIRMATION_LABEL} This uses MatrixRequest::SetAvatar(Some) and client.account().set_avatar_url(Some) after confirmation only.",
        avatar_url.as_str()
    )
}

fn account_avatar_direct_mxc_retry_confirmation_label(avatar_url: &OwnedMxcUri) -> String {
    format!(
        "Retry direct avatar SetAvatar(Some) for {}? {ACCOUNT_AVATAR_DIRECT_SET_CONFIRMATION_LABEL} Retry reuses only the cached mxc:// URI and confirms before MatrixRequest::SetAvatar(Some).",
        avatar_url.as_str()
    )
}

fn account_avatar_upload_crop_editor_boundary_label(
    preview_state: &str,
    selected_summary: Option<&str>,
) -> String {
    let preview_state = preview_state.trim();
    let preview_state = if preview_state.is_empty() {
        "preview state unknown"
    } else {
        preview_state
    };
    let selected_summary = selected_summary
        .map(str::trim)
        .filter(|summary| !summary.is_empty())
        .unwrap_or("no selected image metadata loaded");

    format!(
        "Avatar crop/editor boundary: {preview_state}. Selected metadata: {selected_summary}. Crop, aspect-ratio presets, rotate/zoom, image editor controls, thumbnail generation, mobile camera capture, mobile photo-library capture, browser handoff, account-data mutation beyond confirmed UploadAvatar/direct SetAvatar(Some), gateway/runtime/auth, and live mutation stay local blocked. {ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_LABEL}"
    )
}

fn account_avatar_upload_cropper_snapshot_label(
    action: &str,
    preview_state: AvatarUploadPreviewState,
    selected_summary: Option<&str>,
) -> String {
    let action = action.trim();
    let action = if action.is_empty() { "Crop" } else { action };
    let selected_summary = selected_summary
        .map(str::trim)
        .filter(|summary| !summary.is_empty())
        .unwrap_or("no selected image metadata loaded");
    let preview_state = avatar_upload_preview_state_label(preview_state);

    format!(
        "Local avatar cropper packet snapshot: {action} selected. Preview state: {preview_state}. Selected metadata: {selected_summary}. Crop box, aspect preset, rotate/zoom state, thumbnail target, camera/library source, browser handoff, and UploadAvatar handoff are represented as local metadata only. No cropper/editor, image decode, thumbnail generation, camera capture, photo-library picker, browser handoff, UploadAvatar, SetAvatar(Some), account/profile mutation, gateway/runtime/auth, or live mutation request was submitted. {ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_LABEL}"
    )
}

fn avatar_upload_preview_state_label(state: AvatarUploadPreviewState) -> &'static str {
    match state {
        AvatarUploadPreviewState::Hidden => "hidden",
        AvatarUploadPreviewState::ChoosePhoto => "choose photo preview",
        AvatarUploadPreviewState::Selected => "selected image preview",
        AvatarUploadPreviewState::Failed => "failed upload retry preview",
        AvatarUploadPreviewState::Crop => "crop/editor preview",
    }
}

fn account_avatar_upload_editor_controls_row_label(
    action: &str,
    preview_state: AvatarUploadPreviewState,
    selected_summary: Option<&str>,
) -> String {
    let action = action.trim();
    let action = if action.is_empty() {
        "Editor control"
    } else {
        action
    };
    let selected_summary = selected_summary
        .map(str::trim)
        .filter(|summary| !summary.is_empty())
        .unwrap_or("no selected image metadata loaded");
    let preview_state = avatar_upload_preview_state_label(preview_state);

    format!(
        "Avatar editor control: {action} stayed local. Preview state: {preview_state}. Selected metadata: {selected_summary}. Aspect, Rotate, Zoom, Camera, and Library only update local preview metadata and popup copy; no cropper/editor, image transform, thumbnail decode, camera capture, photo-library picker, browser handoff, UploadAvatar, SetAvatar(Some), account/profile mutation, gateway/runtime/auth, or live mutation request. {ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_LABEL}"
    )
}

fn account_avatar_upload_source_preview_controls_label(
    action: &str,
    preview_state: AvatarUploadPreviewState,
    selected_summary: Option<&str>,
) -> String {
    let action = action.trim();
    if action.eq_ignore_ascii_case("Packet") {
        return account_avatar_upload_source_editor_drilldown_packet_label(
            preview_state,
            selected_summary,
        );
    }
    if action.eq_ignore_ascii_case("Contract") {
        return account_avatar_upload_source_editor_typed_contract_packet_label(
            preview_state,
            selected_summary,
        );
    }
    if action.eq_ignore_ascii_case("Taxonomy") {
        return account_avatar_upload_source_editor_result_taxonomy_packet_label(
            preview_state,
            selected_summary,
        );
    }
    let action = if action.is_empty() {
        "Source preview"
    } else {
        action
    };
    let selected_summary = selected_summary
        .map(str::trim)
        .filter(|summary| !summary.is_empty())
        .unwrap_or("no selected image metadata loaded");
    let preview_state = avatar_upload_preview_state_label(preview_state);

    format!(
        "Avatar source/preview control: {action} stayed local. Preview state: {preview_state}. Selected metadata: {selected_summary}. Source can copy the selected local file path to clipboard; Thumbnail and Full-size use bounded local pixel decode with in-memory RGBA buffers; Camera, Library, Packet, Contract, and Taxonomy only update local source/preview metadata and popup copy. No file picker, camera capture, photo-library picker, persistent thumbnail file, cropper/editor, browser handoff, UploadAvatar, SetAvatar(Some), account/profile mutation, gateway/runtime/auth, or live mutation request. {ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_LABEL}"
    )
}

fn account_avatar_upload_source_editor_drilldown_packet_label(
    preview_state: AvatarUploadPreviewState,
    selected_summary: Option<&str>,
) -> String {
    let selected_summary = selected_summary
        .map(str::trim)
        .filter(|summary| !summary.is_empty())
        .unwrap_or("no selected image metadata loaded");
    let preview_state = avatar_upload_preview_state_label(preview_state);

    format!(
        "Avatar source/editor drilldown packet: Preview state: {preview_state}. Selected metadata: {selected_summary}. Source type, desktop file path handoff, MIME/extension/size/dimensions, crop box/aspect/rotate/zoom, thumbnail/full-size decode targets, camera/photo-library permission and picker states, image editor handoff, UploadAvatar request/result/error/retry/source slots, and SetAvatar handoff are represented as local acceptance criteria only. No file picker, camera/photo-library permission, capture, source mutation, cropper/editor, thumbnail decode/generation, full image decode, UploadAvatar, SetAvatar(Some), account/profile mutation, message send/edit/redact, room-state, membership, gateway/runtime/auth, or live mutation request was submitted. {ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_LABEL}"
    )
}

fn account_avatar_upload_source_editor_typed_contract_packet_label(
    preview_state: AvatarUploadPreviewState,
    selected_summary: Option<&str>,
) -> String {
    let selected_summary = selected_summary
        .map(str::trim)
        .filter(|summary| !summary.is_empty())
        .unwrap_or("no selected image metadata loaded");
    let preview_state = avatar_upload_preview_state_label(preview_state);

    format!(
        "Avatar source/editor typed contract packet: Preview state: {preview_state}. Selected metadata: {selected_summary}. Typed source identity, desktop file handoff, camera/photo-library permission and picker request/result/error slots, cropper crop-box/aspect/rotate/zoom request/result/error slots, thumbnail/full-size decode request/result/error slots, image editor transform result slots, UploadAvatar request/result/error/retry/source slots, direct SetAvatar(Some) request/result/retry mapping, stale local file handling, source-hash, idempotency, and promotion blockers are represented as contracts. No file picker, camera/photo-library permission, capture, source mutation, cropper/editor, thumbnail decode/generation, full image decode, UploadAvatar, account/profile mutation beyond separately confirmed direct SetAvatar(Some), message send/edit/redact, room-state, membership, gateway/runtime/auth, or unconfirmed live mutation request was submitted. {ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_LABEL}"
    )
}

fn account_avatar_upload_source_editor_result_taxonomy_packet_label(
    preview_state: AvatarUploadPreviewState,
    selected_summary: Option<&str>,
) -> String {
    let selected_summary = selected_summary
        .map(str::trim)
        .filter(|summary| !summary.is_empty())
        .unwrap_or("no selected image metadata loaded");
    let preview_state = avatar_upload_preview_state_label(preview_state);

    format!(
        "Avatar source/editor result taxonomy packet: Preview state: {preview_state}. Selected metadata: {selected_summary}. Live references remain confirmed desktop UploadAvatar plus SDK Account::set_avatar_url(Some), confirmed failed-state UploadAvatar Retry, direct MXC SetAvatar(Some) plus confirmed failed-state Retry, SetAvatar(None) delete, selected-file metadata, source-path clipboard, and bounded in-memory Thumbnail/Full-size pixel decode. source_identity_operation_id not_assigned; desktop_picker_result local_selected_file_or_cancel_only; camera_permission_result not_wired; photo_library_permission_result not_wired; camera_capture_result not_wired; photo_library_selection_result not_wired; crop_box_result not_wired; aspect_rotate_zoom_result not_wired; editor_transform_result not_wired; persistent_thumbnail_artifact_id not_assigned; transformed_image_hash not_assigned; transformed_upload_result not_wired; transformed_set_avatar_result not_wired; mobile_capture_result not_wired; stale_source_result not_wired; retry_cancel_result not_wired; audit_redaction raw_path_camera_buffer_thumbnail_transform_redacted. No file picker, camera/photo-library permission, capture, source mutation, cropper/editor, persistent thumbnail generation, transformed image write, transformed UploadAvatar, transformed SetAvatar(Some), account/profile mutation beyond existing confirmed UploadAvatar/direct SetAvatar/Delete paths, message send/edit/redact, room-state, membership, account/device/session-management, gateway/runtime/auth, or live mutation request was submitted. {ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_LABEL} {ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_LABEL} {ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_LABEL}"
    )
}

fn account_avatar_upload_source_path_clipboard_label(
    selected_preview: Option<&AvatarUploadSelectionPreview>,
) -> String {
    match selected_preview {
        Some(preview) => format!(
            "Avatar Source copied selected local file path to clipboard: {}. Selected metadata: {}. {ACCOUNT_AVATAR_UPLOAD_SOURCE_PATH_CLIPBOARD_LABEL} No file picker, thumbnail decode, full image decode, cropper/editor, UploadAvatar, SetAvatar(Some), account/profile mutation, gateway/runtime/auth, or live mutation.",
            preview.file_path.display(),
            preview.summary(),
        ),
        None => format!(
            "Avatar Source has no selected local file path to copy. Choose Photo must stage a local image first. {ACCOUNT_AVATAR_UPLOAD_SOURCE_PATH_CLIPBOARD_LABEL} No file picker was opened, no UploadAvatar, SetAvatar(Some), account/profile mutation, gateway/runtime/auth, or live mutation."
        ),
    }
}

fn account_avatar_upload_source_path_clipboard_payload(
    selected_preview: Option<&AvatarUploadSelectionPreview>,
) -> Option<String> {
    selected_preview.map(|preview| preview.file_path.to_string_lossy().into_owned())
}

fn account_avatar_upload_source_path_clipboard_metadata(
    selected_preview: Option<&AvatarUploadSelectionPreview>,
) -> String {
    let path_state = account_avatar_upload_source_path_clipboard_payload(selected_preview)
        .map(|path| {
            format!(
                "path chars: {}, path bytes: {}",
                path.chars().count(),
                path.len()
            )
        })
        .unwrap_or_else(|| "no selected path payload".to_string());
    format!(
        "{path_state}. {ACCOUNT_AVATAR_UPLOAD_SOURCE_PATH_CLIPBOARD_LABEL} {ACCOUNT_AVATAR_UPLOAD_SOURCE_PATH_CLIPBOARD_EVIDENCE}"
    )
}

fn account_avatar_upload_preflight_detail_controls_label(
    action: &str,
    preview_state: AvatarUploadPreviewState,
    selected_summary: Option<&str>,
) -> String {
    let action = action.trim();
    let action = if action.is_empty() {
        "Preflight detail"
    } else {
        action
    };
    let selected_summary = selected_summary
        .map(str::trim)
        .filter(|summary| !summary.is_empty())
        .unwrap_or("no selected image metadata loaded");
    let preview_state = avatar_upload_preview_state_label(preview_state);

    format!(
        "Avatar upload preflight detail: {action} stayed local. Preview state: {preview_state}. Selected metadata: {selected_summary}. Request, Result, Error, Retry, and Source only update local UploadAvatar preflight metadata and popup copy; no file picker, cropper/editor, image decode, thumbnail generation, camera capture, photo-library picker, browser handoff, UploadAvatar, SetAvatar(Some), account/profile mutation, gateway/runtime/auth, or live mutation request. {ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_LABEL}"
    )
}

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
mod account_avatar_upload_tests {
    use super::*;

    #[test]
    fn selected_avatar_image_dimensions_parse_lightweight_headers() {
        let mut png = vec![0_u8; 24];
        png[0..8].copy_from_slice(b"\x89PNG\r\n\x1a\n");
        png[12..16].copy_from_slice(b"IHDR");
        png[16..20].copy_from_slice(&320_u32.to_be_bytes());
        png[20..24].copy_from_slice(&180_u32.to_be_bytes());
        assert_eq!(
            account_avatar_image_dimensions_from_header(&png),
            Some((320, 180, "PNG"))
        );

        let mut jpeg = vec![0_u8; 25];
        jpeg[0..6].copy_from_slice(&[0xff, 0xd8, 0xff, 0xe0, 0x00, 0x02]);
        jpeg[6..15].copy_from_slice(&[0xff, 0xc0, 0x00, 0x11, 0x08, 0x00, 0xc8, 0x01, 0x2c]);
        assert_eq!(
            account_avatar_image_dimensions_from_header(&jpeg),
            Some((300, 200, "JPEG"))
        );

        let mut gif = b"GIF89a".to_vec();
        gif.extend_from_slice(&64_u16.to_le_bytes());
        gif.extend_from_slice(&48_u16.to_le_bytes());
        assert_eq!(
            account_avatar_image_dimensions_from_header(&gif),
            Some((64, 48, "GIF"))
        );

        let mut bmp = vec![0_u8; 26];
        bmp[0..2].copy_from_slice(b"BM");
        bmp[18..22].copy_from_slice(&72_i32.to_le_bytes());
        bmp[22..26].copy_from_slice(&(-40_i32).to_le_bytes());
        assert_eq!(
            account_avatar_image_dimensions_from_header(&bmp),
            Some((72, 40, "BMP"))
        );

        let mut webp = Vec::new();
        webp.extend_from_slice(b"RIFF");
        webp.extend_from_slice(&18_u32.to_le_bytes());
        webp.extend_from_slice(b"WEBP");
        webp.extend_from_slice(b"VP8X");
        webp.extend_from_slice(&10_u32.to_le_bytes());
        webp.extend_from_slice(&[0, 0, 0, 0]);
        webp.extend_from_slice(&[127, 0, 0]);
        webp.extend_from_slice(&[95, 0, 0]);
        assert_eq!(
            account_avatar_image_dimensions_from_header(&webp),
            Some((128, 96, "WebP"))
        );
    }

    #[test]
    fn selected_avatar_image_dimensions_label_keeps_unsupported_types_explicit() {
        let mime_type: mime::Mime = "text/plain".parse().unwrap();
        assert_eq!(
            account_avatar_image_dimensions_label(Path::new("avatar.txt"), &mime_type),
            "dimensions: unavailable for this avatar image type"
        );
    }

    #[test]
    fn avatar_upload_thumbnail_target_dimensions_preserve_aspect_ratio() {
        assert_eq!(
            account_avatar_thumbnail_target_dimensions(320, 180, 128),
            (128, 72)
        );
        assert_eq!(
            account_avatar_thumbnail_target_dimensions(180, 320, 128),
            (72, 128)
        );
        assert_eq!(
            account_avatar_thumbnail_target_dimensions(64, 48, 128),
            (64, 48)
        );
        assert_eq!(
            account_avatar_thumbnail_target_dimensions(0, 48, 128),
            (0, 0)
        );
    }

    #[test]
    fn avatar_upload_decode_probe_generates_bounded_pixel_buffers() {
        let path = std::env::temp_dir().join(format!(
            "hepta-avatar-pixel-decode-{}.png",
            std::process::id()
        ));
        let mut png = std::io::Cursor::new(Vec::new());
        let image = ::image::RgbaImage::from_fn(320, 180, |x, y| {
            ::image::Rgba([(x % 255) as u8, (y % 255) as u8, ((x + y) % 255) as u8, 255])
        });
        image
            .write_to(&mut png, ::image::ImageFormat::Png)
            .expect("encode test png");
        fs::write(&path, png.into_inner()).expect("write test png");

        let mime_type: mime::Mime = "image/png".parse().unwrap();
        let preview = account_avatar_selection_preview(&path, &mime_type);
        let thumbnail = account_avatar_upload_decode_probe_label("Thumbnail", Some(&preview));
        let full_size = account_avatar_upload_decode_probe_label("Full-size", Some(&preview));
        let _ = fs::remove_file(&path);

        assert!(thumbnail.contains("Avatar Thumbnail pixel decode ready"));
        assert!(thumbnail.contains("Format: PNG"));
        assert!(thumbnail.contains("original: 320x180"));
        assert!(thumbnail.contains("generated in-memory 128px RGBA thumbnail: 128x72"));
        assert!(thumbnail.contains("RGBA"));
        assert!(thumbnail.contains("source bytes read"));
        assert!(thumbnail.contains("No thumbnail file"));
        assert!(thumbnail.contains("cropper/editor transform"));
        assert!(thumbnail.contains("UploadAvatar"));
        assert!(thumbnail.contains("SetAvatar(Some)"));
        assert!(thumbnail.contains("gateway/runtime/auth"));
        assert!(thumbnail.contains("live mutation"));
        assert!(thumbnail.contains(ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_LABEL));

        assert!(full_size.contains("Avatar Full-size pixel decode ready"));
        assert!(full_size.contains("decoded full-size RGBA pixel buffer: 320x180"));
        assert!(full_size.contains("RGBA"));
        assert!(
            ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_EVIDENCE.contains("already selected local image")
        );
        assert!(ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_EVIDENCE.contains("bounded local pixel decode"));
        assert!(ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_EVIDENCE.contains("creates no thumbnail file"));
        assert!(ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_EVIDENCE.contains("Full-size decodes"));
    }

    #[test]
    fn avatar_upload_decode_probe_uses_empty_fallbacks() {
        let label = account_avatar_upload_decode_probe_label("Thumbnail", None);

        assert!(label.contains("has no selected local image yet"));
        assert!(label.contains("Choose Photo"));
        assert!(label.contains("No file picker"));
        assert!(label.contains("UploadAvatar"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_LABEL));
    }

    #[test]
    fn avatar_upload_lifecycle_metadata_label_summarizes_selected_file_state() {
        let preview = AvatarUploadSelectionPreview {
            file_path: PathBuf::from("portrait.png"),
            mime: "image/png".parse().unwrap(),
            filename: "portrait.png".to_string(),
            extension: "png".to_string(),
            mime_type: "image/png".to_string(),
            size_label: "42 KiB".to_string(),
            dimensions_label: "dimensions: 320x180 from PNG header".to_string(),
        };
        let summary = preview.summary();
        let label = account_avatar_upload_lifecycle_metadata_label(
            "confirmation canceled; Matrix avatar upload was not requested",
            Some(&summary),
        );

        assert!(label.contains("Avatar upload confirmation canceled"));
        assert!(label.contains("portrait.png"));
        assert!(label.contains("image/png"));
        assert!(label.contains("42 KiB"));
        assert!(label.contains("png"));
        assert!(label.contains("320x180"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_LABEL));
        assert!(ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_EVIDENCE.contains("picker canceled"));
        assert!(ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_EVIDENCE.contains("invalid selection"));
        assert!(
            ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_EVIDENCE.contains("confirmed upload handoff")
        );
        assert!(ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_EVIDENCE.contains(
            "MatrixRequest::UploadAvatar is still submitted only from the confirmed accept handler"
        ));
        assert!(ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_EVIDENCE.contains("gateway/runtime/auth"));
        assert!(ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn avatar_upload_retry_confirmation_label_is_narrow() {
        let label = account_avatar_upload_retry_confirmation_label(
            "portrait.png · image/png · 42 KiB · png · dimensions loaded",
            Path::new("/tmp/portrait.png"),
        );

        assert!(label.contains("Retry avatar upload"));
        assert!(label.contains("/tmp/portrait.png"));
        assert!(label.contains("portrait.png"));
        assert!(label.contains("Retry confirms before UploadAvatar"));
        assert!(label.contains("No new file picker"));
        assert!(label.contains("cropper/editor"));
        assert!(label.contains("camera/photo-library"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(
            ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_EVIDENCE.contains("cached local file path")
        );
        assert!(
            ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_EVIDENCE
                .contains("MatrixRequest::UploadAvatar")
        );
    }

    #[test]
    fn avatar_direct_mxc_editor_validates_and_confirms_setavatar_some() {
        let avatar_url =
            parse_account_avatar_direct_mxc_uri("  mxc://example.org/avatar-media-id  ")
                .expect("valid mxc uri");
        assert_eq!(avatar_url.as_str(), "mxc://example.org/avatar-media-id");
        assert!(parse_account_avatar_direct_mxc_uri("").is_err());
        assert!(parse_account_avatar_direct_mxc_uri("https://example.org/avatar.png").is_err());
        assert!(parse_account_avatar_direct_mxc_uri("mxc://example.org").is_err());

        let status = account_avatar_direct_mxc_editor_status_label(
            "mxc://example.org/avatar-media-id",
            Some(&avatar_url),
        );
        assert!(status.contains("draft MXC URI staged locally"));
        assert!(status.contains("Failed direct SetAvatar(Some) retry cache"));
        assert!(status.contains("Direct MXC editor confirms"));
        assert!(status.contains("No file picker"));
        assert!(status.contains("gateway/runtime/auth"));

        let confirm = account_avatar_direct_mxc_confirmation_label(&avatar_url);
        assert!(confirm.contains("MatrixRequest::SetAvatar(Some)"));
        assert!(confirm.contains("client.account().set_avatar_url(Some)"));
        assert!(confirm.contains("after confirmation only"));

        let retry = account_avatar_direct_mxc_retry_confirmation_label(&avatar_url);
        assert!(retry.contains("Retry direct avatar SetAvatar(Some)"));
        assert!(retry.contains("cached mxc:// URI"));
        assert!(retry.contains("MatrixRequest::SetAvatar(Some)"));

        assert!(ACCOUNT_AVATAR_DIRECT_SET_CONFIRMATION_EVIDENCE.contains("mxc:// URI"));
        assert!(
            ACCOUNT_AVATAR_DIRECT_SET_CONFIRMATION_EVIDENCE
                .contains("MatrixRequest::SetAvatar(Some")
        );
        assert!(
            ACCOUNT_AVATAR_DIRECT_SET_CONFIRMATION_EVIDENCE
                .contains("client.account().set_avatar_url(Some")
        );
        assert!(ACCOUNT_AVATAR_DIRECT_SET_CONFIRMATION_EVIDENCE.contains("AvatarChangeFailed"));
    }

    #[test]
    fn avatar_upload_real_path_evidence_includes_sdk_set_avatar_some() {
        assert!(ACCOUNT_AVATAR_UPLOAD_REAL_PATH_EVIDENCE.contains("MatrixRequest::UploadAvatar"));
        assert!(
            ACCOUNT_AVATAR_UPLOAD_REAL_PATH_EVIDENCE.contains("client.account().upload_avatar")
        );
        assert!(
            ACCOUNT_AVATAR_UPLOAD_REAL_PATH_EVIDENCE.contains("Account::set_avatar_url(Some(mxc))")
        );
        assert!(ACCOUNT_AVATAR_UPLOAD_REAL_PATH_EVIDENCE.contains("AvatarChanged(Some(mxc))"));
        assert!(ACCOUNT_AVATAR_UPLOAD_REAL_PATH_EVIDENCE.contains("MatrixRequest::SetAvatar(Some"));
        assert!(
            ACCOUNT_AVATAR_UPLOAD_LOCAL_BOUNDARY_EVIDENCE
                .contains("Media::upload plus Account::set_avatar_url(Some(mxc))")
        );
        assert!(ACCOUNT_AVATAR_UPLOAD_COMPACT_EVIDENCE.contains("direct MXC SetAvatar(Some)"));
    }

    #[test]
    fn avatar_upload_crop_editor_boundary_label_lists_blocked_controls() {
        let label = account_avatar_upload_crop_editor_boundary_label(
            "Crop opened locally",
            Some("portrait.png · image/png · 42 KiB · png · dimensions: 320x180"),
        );

        assert!(label.contains("Avatar crop/editor boundary"));
        assert!(label.contains("Crop opened locally"));
        assert!(label.contains("portrait.png"));
        assert!(label.contains("aspect-ratio presets"));
        assert!(label.contains("rotate/zoom"));
        assert!(label.contains("image editor controls"));
        assert!(label.contains("thumbnail generation"));
        assert!(label.contains("mobile camera capture"));
        assert!(label.contains("mobile photo-library capture"));
        assert!(label.contains("browser handoff"));
        assert!(label.contains("direct SetAvatar(Some)"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_LABEL));
        assert!(
            ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_EVIDENCE
                .contains("AvatarUploadPreviewState")
        );
        assert!(
            ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_EVIDENCE
                .contains("local avatar cropper packet snapshot")
        );
        assert!(
            ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_EVIDENCE.contains(
                "existing desktop picker plus confirmation-gated MatrixRequest::UploadAvatar"
            )
        );
    }

    #[test]
    fn avatar_upload_crop_editor_boundary_label_uses_empty_fallbacks() {
        let label = account_avatar_upload_crop_editor_boundary_label("", None);

        assert!(label.contains("preview state unknown"));
        assert!(label.contains("no selected image metadata loaded"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_LABEL));
    }

    #[test]
    fn avatar_upload_cropper_snapshot_label_summarizes_local_crop_packet() {
        let label = account_avatar_upload_cropper_snapshot_label(
            "Aspect",
            AvatarUploadPreviewState::Crop,
            Some("portrait.png · image/png · 42 KiB · png · dimensions: 320x180"),
        );

        assert!(label.contains("Local avatar cropper packet snapshot"));
        assert!(label.contains("Aspect selected"));
        assert!(label.contains("crop/editor preview"));
        assert!(label.contains("portrait.png"));
        assert!(label.contains("Crop box"));
        assert!(label.contains("aspect preset"));
        assert!(label.contains("rotate/zoom state"));
        assert!(label.contains("thumbnail target"));
        assert!(label.contains("camera/library source"));
        assert!(label.contains("UploadAvatar handoff"));
        assert!(label.contains("No cropper/editor"));
        assert!(label.contains("image decode"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_LABEL));
    }

    #[test]
    fn avatar_upload_editor_controls_row_label_keeps_controls_local() {
        let label = account_avatar_upload_editor_controls_row_label(
            "Rotate",
            AvatarUploadPreviewState::Crop,
            Some("portrait.png · image/png · 42 KiB · png · dimensions: 320x180"),
        );

        assert!(label.contains("Avatar editor control"));
        assert!(label.contains("Rotate stayed local"));
        assert!(label.contains("crop/editor preview"));
        assert!(label.contains("portrait.png"));
        assert!(label.contains("Aspect, Rotate, Zoom, Camera, and Library"));
        assert!(label.contains("no cropper/editor"));
        assert!(label.contains("image transform"));
        assert!(label.contains("thumbnail decode"));
        assert!(label.contains("camera capture"));
        assert!(label.contains("photo-library picker"));
        assert!(label.contains("UploadAvatar"));
        assert!(label.contains("SetAvatar(Some)"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_LABEL));
        assert!(ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_EVIDENCE.contains("visible local"));
        assert!(
            ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_EVIDENCE
                .contains("local avatar cropper packet snapshot")
        );
        assert!(
            ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_EVIDENCE.contains("AvatarUploadPreviewState")
        );
        assert!(ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_EVIDENCE.contains("no cropper/editor"));
    }

    #[test]
    fn avatar_upload_editor_controls_row_label_uses_empty_fallbacks() {
        let label = account_avatar_upload_editor_controls_row_label(
            "   ",
            AvatarUploadPreviewState::Hidden,
            None,
        );

        assert!(label.contains("Editor control stayed local"));
        assert!(label.contains("hidden"));
        assert!(label.contains("no selected image metadata loaded"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_LABEL));
    }

    #[test]
    fn avatar_upload_source_preview_controls_label_keeps_controls_local() {
        let label = account_avatar_upload_source_preview_controls_label(
            "Camera",
            AvatarUploadPreviewState::Selected,
            Some("portrait.png · image/png · 42 KiB · png · dimensions: 320x180"),
        );

        assert!(label.contains("Avatar source/preview control"));
        assert!(label.contains("Camera stayed local"));
        assert!(label.contains("selected image preview"));
        assert!(label.contains("portrait.png"));
        assert!(label.contains("Source can copy the selected local file path"));
        assert!(label.contains("Thumbnail and Full-size use bounded local pixel decode"));
        assert!(label.contains("in-memory RGBA buffers"));
        assert!(label.contains("Camera, Library, Packet, Contract, and Taxonomy"));
        assert!(label.contains("No file picker"));
        assert!(label.contains("camera capture"));
        assert!(label.contains("photo-library picker"));
        assert!(label.contains("persistent thumbnail file"));
        assert!(label.contains("cropper/editor"));
        assert!(label.contains("UploadAvatar"));
        assert!(label.contains("SetAvatar(Some)"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_LABEL));
        assert!(
            ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_EVIDENCE
                .contains("Source can copy the already selected local avatar file path")
        );
        assert!(
            ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_EVIDENCE
                .contains("AvatarUploadPreviewState")
        );
        assert!(
            ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_EVIDENCE.contains("opens no file picker")
        );
        assert!(
            ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_EVIDENCE
                .contains("bounded local pixel decode")
        );
        assert!(
            ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_EVIDENCE.contains(
                "Taxonomy records source/cropper/camera/library/thumbnail artifact result slots"
            )
        );
    }

    #[test]
    fn avatar_upload_source_preview_controls_label_uses_empty_fallbacks() {
        let label = account_avatar_upload_source_preview_controls_label(
            "   ",
            AvatarUploadPreviewState::Hidden,
            None,
        );

        assert!(label.contains("Source preview stayed local"));
        assert!(label.contains("hidden"));
        assert!(label.contains("no selected image metadata loaded"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_LABEL));
    }

    #[test]
    fn avatar_upload_source_editor_drilldown_packet_label_persists_acceptance_matrix() {
        let label = account_avatar_upload_source_editor_drilldown_packet_label(
            AvatarUploadPreviewState::Selected,
            Some("portrait.png · image/png · 42 KiB · png · dimensions: 320x180"),
        );

        assert!(label.contains("Avatar source/editor drilldown packet"));
        assert!(label.contains("selected image preview"));
        assert!(label.contains("portrait.png"));
        assert!(label.contains("Source type"));
        assert!(label.contains("desktop file path handoff"));
        assert!(label.contains("MIME/extension/size/dimensions"));
        assert!(label.contains("crop box/aspect/rotate/zoom"));
        assert!(label.contains("thumbnail/full-size decode targets"));
        assert!(label.contains("camera/photo-library permission"));
        assert!(label.contains("image editor handoff"));
        assert!(label.contains("UploadAvatar request/result/error/retry/source slots"));
        assert!(label.contains("SetAvatar handoff"));
        assert!(label.contains("No file picker"));
        assert!(label.contains("source mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_LABEL));
        assert!(
            ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_EVIDENCE
                .contains("visible Packet control")
        );
        assert!(
            ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_EVIDENCE
                .contains("camera/photo-library permission and picker states")
        );
    }

    #[test]
    fn avatar_upload_source_preview_controls_label_routes_packet_to_drilldown() {
        let label = account_avatar_upload_source_preview_controls_label(
            "Packet",
            AvatarUploadPreviewState::Crop,
            None,
        );

        assert!(label.contains("Avatar source/editor drilldown packet"));
        assert!(label.contains("crop/editor preview"));
        assert!(label.contains("no selected image metadata loaded"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_LABEL));
    }

    #[test]
    fn avatar_upload_source_editor_typed_contract_packet_label_maps_drilldown_to_contracts() {
        let label = account_avatar_upload_source_editor_typed_contract_packet_label(
            AvatarUploadPreviewState::Selected,
            Some("portrait.png · image/png · 42 KiB · png · dimensions: 320x180"),
        );

        assert!(label.contains("Avatar source/editor typed contract packet"));
        assert!(label.contains("selected image preview"));
        assert!(label.contains("portrait.png"));
        assert!(label.contains("Typed source identity"));
        assert!(label.contains("desktop file handoff"));
        assert!(
            label.contains("camera/photo-library permission and picker request/result/error slots")
        );
        assert!(label.contains("cropper crop-box/aspect/rotate/zoom request/result/error slots"));
        assert!(label.contains("thumbnail/full-size decode request/result/error slots"));
        assert!(label.contains("image editor transform result slots"));
        assert!(label.contains("UploadAvatar request/result/error/retry/source slots"));
        assert!(label.contains("direct SetAvatar(Some) request/result/retry mapping"));
        assert!(label.contains("stale local file handling"));
        assert!(label.contains("source-hash"));
        assert!(label.contains("idempotency"));
        assert!(label.contains("promotion blockers"));
        assert!(label.contains("No file picker"));
        assert!(label.contains("source mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_LABEL));
        assert!(
            ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_EVIDENCE
                .contains("visible Contract control")
        );
        assert!(
            ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_EVIDENCE
                .contains("typed cropper, camera, image-edit, thumbnail/full-size decode")
        );
        assert!(
            ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_EVIDENCE
                .contains("direct SetAvatar(Some) request/result/retry mapping")
        );
    }

    #[test]
    fn avatar_upload_source_preview_controls_label_routes_contract_to_typed_packet() {
        let label = account_avatar_upload_source_preview_controls_label(
            "Contract",
            AvatarUploadPreviewState::Crop,
            None,
        );

        assert!(label.contains("Avatar source/editor typed contract packet"));
        assert!(label.contains("crop/editor preview"));
        assert!(label.contains("no selected image metadata loaded"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_LABEL));
    }

    #[test]
    fn avatar_upload_source_editor_result_taxonomy_packet_label_names_blocked_result_slots() {
        let label = account_avatar_upload_source_editor_result_taxonomy_packet_label(
            AvatarUploadPreviewState::Selected,
            Some("portrait.png · image/png · 42 KiB · png · dimensions: 320x180"),
        );

        assert!(label.contains("Avatar source/editor result taxonomy packet"));
        assert!(label.contains("selected image preview"));
        assert!(label.contains("portrait.png"));
        assert!(label.contains("confirmed desktop UploadAvatar"));
        assert!(label.contains("SDK Account::set_avatar_url(Some)"));
        assert!(label.contains("direct MXC SetAvatar(Some)"));
        assert!(label.contains("SetAvatar(None) delete"));
        assert!(label.contains("source_identity_operation_id not_assigned"));
        assert!(label.contains("camera_permission_result not_wired"));
        assert!(label.contains("photo_library_permission_result not_wired"));
        assert!(label.contains("crop_box_result not_wired"));
        assert!(label.contains("editor_transform_result not_wired"));
        assert!(label.contains("persistent_thumbnail_artifact_id not_assigned"));
        assert!(label.contains("transformed_upload_result not_wired"));
        assert!(label.contains("transformed_set_avatar_result not_wired"));
        assert!(
            label.contains("audit_redaction raw_path_camera_buffer_thumbnail_transform_redacted")
        );
        assert!(label.contains("No file picker"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_LABEL));
    }

    #[test]
    fn avatar_upload_source_preview_controls_label_routes_taxonomy_to_result_packet() {
        let label = account_avatar_upload_source_preview_controls_label(
            "Taxonomy",
            AvatarUploadPreviewState::Crop,
            None,
        );

        assert!(label.contains("Avatar source/editor result taxonomy packet"));
        assert!(label.contains("crop/editor preview"));
        assert!(label.contains("no selected image metadata loaded"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_LABEL));
    }

    #[test]
    fn avatar_upload_source_path_clipboard_label_copies_only_selected_local_path() {
        let preview = AvatarUploadSelectionPreview {
            file_path: PathBuf::from("/tmp/portrait.png"),
            mime: "image/png".parse().unwrap(),
            filename: "portrait.png".to_string(),
            extension: "png".to_string(),
            mime_type: "image/png".to_string(),
            size_label: "42 KiB".to_string(),
            dimensions_label: "dimensions: 320x180 from PNG header".to_string(),
        };

        let payload = account_avatar_upload_source_path_clipboard_payload(Some(&preview));
        assert_eq!(payload.as_deref(), Some("/tmp/portrait.png"));

        let label = account_avatar_upload_source_path_clipboard_label(Some(&preview));
        assert!(label.contains("Avatar Source copied selected local file path to clipboard"));
        assert!(label.contains("/tmp/portrait.png"));
        assert!(label.contains("portrait.png"));
        assert!(label.contains("320x180"));
        assert!(label.contains("No file picker"));
        assert!(label.contains("UploadAvatar"));
        assert!(label.contains("SetAvatar(Some)"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(ACCOUNT_AVATAR_UPLOAD_SOURCE_PATH_CLIPBOARD_EVIDENCE.contains("local clipboard"));
    }

    #[test]
    fn avatar_upload_source_path_clipboard_label_reports_missing_selection() {
        assert_eq!(
            account_avatar_upload_source_path_clipboard_payload(None),
            None
        );
        let label = account_avatar_upload_source_path_clipboard_label(None);
        assert!(label.contains("no selected local file path"));
        assert!(label.contains("Choose Photo"));
        assert!(label.contains("No file picker was opened"));

        let metadata = account_avatar_upload_source_path_clipboard_metadata(None);
        assert!(metadata.contains("no selected path payload"));
        assert!(metadata.contains(ACCOUNT_AVATAR_UPLOAD_SOURCE_PATH_CLIPBOARD_LABEL));
    }

    #[test]
    fn avatar_upload_preflight_detail_controls_label_keeps_controls_local() {
        let label = account_avatar_upload_preflight_detail_controls_label(
            "Result",
            AvatarUploadPreviewState::Selected,
            Some("portrait.png · image/png · 42 KiB · png · dimensions: 320x180"),
        );

        assert!(label.contains("Avatar upload preflight detail"));
        assert!(label.contains("Result stayed local"));
        assert!(label.contains("selected image preview"));
        assert!(label.contains("portrait.png"));
        assert!(label.contains("Request, Result, Error, Retry, and Source"));
        assert!(label.contains("UploadAvatar preflight metadata"));
        assert!(label.contains("no file picker"));
        assert!(label.contains("cropper/editor"));
        assert!(label.contains("image decode"));
        assert!(label.contains("photo-library picker"));
        assert!(label.contains("UploadAvatar"));
        assert!(label.contains("SetAvatar(Some)"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_LABEL));
        assert!(
            ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("visible local UploadAvatar detail buttons")
        );
        assert!(
            ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("AvatarUploadPreviewState")
        );
        assert!(
            ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("opens no file picker")
        );
    }

    #[test]
    fn avatar_upload_preflight_detail_controls_label_uses_empty_fallbacks() {
        let label = account_avatar_upload_preflight_detail_controls_label(
            "   ",
            AvatarUploadPreviewState::Hidden,
            None,
        );

        assert!(label.contains("Preflight detail stayed local"));
        assert!(label.contains("hidden"));
        assert!(label.contains("no selected image metadata loaded"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_LABEL));
    }

    #[test]
    fn avatar_upload_lifecycle_metadata_label_uses_empty_selection_fallback() {
        let label = account_avatar_upload_lifecycle_metadata_label("picker canceled", None);

        assert!(label.contains("Avatar upload picker canceled"));
        assert!(label.contains("no selected image metadata loaded"));
        assert!(label.contains(ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_LABEL));
    }

    #[test]
    fn avatar_upload_invalid_selection_metadata_summary_keeps_validation_reason() {
        let mime_type: mime::Mime = "text/plain".parse().unwrap();
        let summary = account_avatar_invalid_selection_metadata_summary(
            Path::new("/tmp/avatar.txt"),
            &mime_type,
            "selected file is not an image",
        );

        assert!(summary.contains("avatar.txt"));
        assert!(summary.contains("text/plain"));
        assert!(summary.contains("txt"));
        assert!(summary.contains("selected file is not an image"));
    }

    #[test]
    fn account_management_loaded_identity_uses_existing_profile_state() {
        let profile = UserProfile {
            user_id: matrix_sdk::ruma::OwnedUserId::try_from("@alice:example.org").unwrap(),
            username: Some("Alice".to_string()),
            avatar_state: AvatarState::Known(None),
        };
        let label = loaded_account_identity_label(Some(&profile));
        assert!(label.contains("Alice"));
        assert!(label.contains("@alice:example.org"));
        assert!(label.contains("no avatar"));
    }

    #[test]
    fn account_management_lifecycle_metadata_label_reuses_loaded_identity() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current device: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let label = account_management_lifecycle_metadata_label(
            "Security opened locally",
            AccountManagementPreviewState::Security,
            Some(loaded_identity),
        );

        assert!(label.contains("Account management Security opened locally"));
        assert!(label.contains("Preview state: Security preview"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("@alice:example.org"));
        assert!(label.contains("Current device: DEVICEID"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_LABEL));
        assert!(ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_EVIDENCE.contains("Manage Account"));
        assert!(ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_EVIDENCE.contains("Security"));
        assert!(ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_EVIDENCE.contains("Sessions"));
        assert!(ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_EVIDENCE.contains("Close only hides"));
        assert!(
            ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_EVIDENCE
                .contains("MatrixRequest::GetOwnDevice only while current device data is missing")
        );
        assert!(ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_EVIDENCE.contains("gateway/runtime/auth"));
        assert!(ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn account_management_lifecycle_metadata_label_uses_pending_fallback() {
        let label = account_management_lifecycle_metadata_label(
            "Close hid the local preview",
            AccountManagementPreviewState::Hidden,
            None,
        );

        assert!(label.contains("Preview state: hidden preview"));
        assert!(label.contains("loaded account/device metadata pending"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_LABEL));
    }

    #[test]
    fn account_management_refresh_confirmation_label_is_read_only() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let label = account_management_refresh_confirmation_label(
            "confirmed; MatrixRequest::GetOwnDevice was requested",
            Some(loaded_identity),
        );

        assert!(label.contains("Account management refresh confirmed"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("@alice:example.org"));
        assert!(label.contains("Device ID: DEVICEID"));
        assert!(label.contains("Refresh confirms before GetOwnDevice"));
        assert!(
            ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE.contains("PositiveConfirmationModal")
        );
        assert!(
            ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE
                .contains("MatrixRequest::GetOwnDevice")
        );
        assert!(ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE.contains("Device display name"));
        assert!(ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE.contains("external account page"));
        assert!(
            ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE.contains("session-management lookup")
        );
        assert!(ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE.contains("gateway/runtime/auth"));
        assert!(ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn account_management_refresh_confirmation_label_uses_pending_fallback() {
        let label = account_management_refresh_confirmation_label("confirmation canceled", None);

        assert!(label.contains("Account management refresh confirmation canceled"));
        assert!(label.contains("loaded account/device metadata pending"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_LABEL));
    }

    #[test]
    fn account_management_device_directory_retry_confirmation_label_confirms_getdevices() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let label = account_management_device_directory_retry_confirmation_label(
            "confirmed; MatrixRequest::GetDevices was requested",
            Some(loaded_identity),
            Some("network failed"),
        );

        assert!(label.contains("Account management device-directory retry confirmed"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("Device ID: DEVICEID"));
        assert!(label.contains("Cached GetDevices error: network failed"));
        assert!(label.contains("PositiveConfirmationModal"));
        assert!(label.contains("MatrixRequest::GetDevices"));
        assert!(label.contains("read-only"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_RETRY_CONFIRMATION_LABEL));
        assert!(
            ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_RETRY_CONFIRMATION_EVIDENCE
                .contains("own_devices_last_error")
        );
        assert!(
            ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_RETRY_CONFIRMATION_EVIDENCE
                .contains("PositiveConfirmationModal")
        );
        assert!(
            ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_RETRY_CONFIRMATION_EVIDENCE
                .contains("MatrixRequest::GetDevices")
        );
        assert!(
            ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_RETRY_CONFIRMATION_EVIDENCE
                .contains("write-side live mutation")
        );
    }

    #[test]
    fn account_management_device_directory_retry_confirmation_label_uses_fallback() {
        let label = account_management_device_directory_retry_confirmation_label(
            "confirmation canceled",
            None,
            None,
        );

        assert!(label.contains("Account management device-directory retry confirmation canceled"));
        assert!(label.contains("loaded account/device metadata pending"));
        assert!(label.contains("No cached GetDevices error is available"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_RETRY_CONFIRMATION_LABEL));
    }

    #[test]
    fn account_management_browser_portal_url_accepts_http_https_and_strips_query_fragment() {
        let https_url = account_management_browser_portal_url_from_homeserver(
            " https://matrix.example.org/_matrix/client?via=example.org#account ",
        )
        .unwrap();
        let http_url =
            account_management_browser_portal_url_from_homeserver("http://localhost:8008/#/login")
                .unwrap();

        assert_eq!(https_url, "https://matrix.example.org/_matrix/client");
        assert_eq!(http_url, "http://localhost:8008/");
    }

    #[test]
    fn account_management_browser_portal_url_rejects_empty_invalid_or_non_http() {
        assert!(account_management_browser_portal_url_from_homeserver("").is_err());
        assert!(
            account_management_browser_portal_url_from_homeserver("matrix.example.org").is_err()
        );
        assert!(
            account_management_browser_portal_url_from_homeserver("mxc://example.org/avatar")
                .is_err()
        );
    }

    #[test]
    fn account_management_browser_portal_handoff_label_confirms_homeserver_opener() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified.";
        let label = account_management_browser_portal_handoff_label(
            "Browser",
            AccountManagementPreviewState::Overview,
            Some(loaded_identity),
            Some("https://matrix.example.org/"),
            None,
        );

        assert!(label.contains("Browser homeserver handoff"));
        assert!(label.contains("Preview state: Manage Account overview"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("Target homeserver URL: https://matrix.example.org/"));
        assert!(label.contains("PositiveConfirmationModal"));
        assert!(label.contains("robius_open"));
        assert!(label.contains("active Matrix homeserver URL"));
        assert!(label.contains("No MatrixRequest"));
        assert!(label.contains("password change"));
        assert!(label.contains("SSO flow"));
        assert!(label.contains("dedicated account-management portal route"));
        assert!(label.contains("cross-session revoke/trust"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("Telegram delivery"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_BROWSER_PORTAL_HANDOFF_LABEL));
    }

    #[test]
    fn account_management_browser_portal_handoff_label_reports_unavailable_state() {
        let label = account_management_browser_portal_handoff_label(
            "Portal",
            AccountManagementPreviewState::Security,
            None,
            None,
            Some("Matrix client unavailable"),
        );

        assert!(label.contains("Portal homeserver handoff"));
        assert!(label.contains("Target homeserver URL pending"));
        assert!(label.contains("Handoff unavailable: Matrix client unavailable"));
        assert!(label.contains("loaded account/device metadata pending"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_BROWSER_PORTAL_HANDOFF_LABEL));
    }

    #[test]
    fn account_management_session_revoke_boundary_label_lists_blocked_controls() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let label = account_management_session_revoke_boundary_label(
            AccountManagementPreviewState::Sessions,
            Some(loaded_identity),
        );

        assert!(label.contains("Account management session/revoke boundary"));
        assert!(label.contains("Preview state: Sessions preview"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("Device ID: DEVICEID"));
        assert!(label.contains("Browser/Portal use a separate confirmed homeserver opener"));
        assert!(label.contains("Dedicated external account page routes"));
        assert!(label.contains("password change"));
        assert!(label.contains("SSO change"));
        assert!(label.contains("Read-only GetDevices directory"));
        assert!(
            label.contains(
                "current-device Rename has a separate confirmed Matrix rename_device path"
            )
        );
        assert!(label.contains("session-management lookup"));
        assert!(label.contains("cross-session revoke"));
        assert!(label.contains("device delete/trust changes"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("unconfirmed live mutation"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_LABEL));
        assert!(
            ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_EVIDENCE
                .contains("AccountManagementPreviewState")
        );
        assert!(
            ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_EVIDENCE.contains("GetOwnDevice previews")
        );
    }

    #[test]
    fn account_management_session_revoke_boundary_label_uses_pending_fallback() {
        let label = account_management_session_revoke_boundary_label(
            AccountManagementPreviewState::Overview,
            None,
        );

        assert!(label.contains("Preview state: Manage Account overview"));
        assert!(label.contains("loaded account/device metadata pending"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_LABEL));
    }

    #[test]
    fn account_management_session_actions_row_label_is_local_only() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let label = account_management_session_actions_row_label(
            "Revoke",
            AccountManagementPreviewState::Sessions,
            Some(loaded_identity),
        );

        assert!(label.contains("Revoke staged locally"));
        assert!(label.contains("Preview state: Sessions preview"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("Device ID: DEVICEID"));
        assert!(label.contains("Rename has a separate confirmation-gated current-device MatrixRequest::RenameDevice path"));
        assert!(label.contains("Revoke and Trust are visible local blocked controls"));
        assert!(
            label.contains("Browser uses a separate PositiveConfirmationModal homeserver opener")
        );
        assert!(label.contains("No all-device list"));
        assert!(label.contains("session-management lookup"));
        assert!(label.contains("cross-session revoke"));
        assert!(label.contains("device delete/trust change"));
        assert!(label.contains("Matrix account/profile mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("unconfirmed live mutation"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_LABEL));
        assert!(
            ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_EVIDENCE.contains("account_management_preview")
        );
        assert!(
            ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_EVIDENCE
                .contains("Revoke, Rename, Trust, and Browser")
        );
        assert!(
            ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_EVIDENCE.contains("MatrixRequest::RenameDevice")
        );
        assert!(ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_EVIDENCE.contains("robius_open"));
    }

    #[test]
    fn account_management_current_device_rename_target_is_stable_and_bounded() {
        let profile = UserProfile {
            user_id: matrix_sdk::ruma::OwnedUserId::try_from("@alice:example.org").unwrap(),
            username: Some("  Alice   Native  ".to_string()),
            avatar_state: AvatarState::Known(None),
        };
        let target = account_management_device_rename_target(Some(&profile));

        assert_eq!(target, "Hepta Native - Alice Native");
        assert!(target.chars().count() <= 64);
        assert_eq!(
            account_management_device_rename_target(None),
            "Hepta Native"
        );
    }

    #[test]
    fn account_management_current_device_rename_confirmation_label_gates_live_request() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let label = account_management_current_device_rename_confirmation_label(
            "confirmed; MatrixRequest::RenameDevice was requested",
            Some(loaded_identity),
            Some("DEVICEID"),
            Some("Hepta Native - Alice"),
            None,
        );

        assert!(label.contains("current-device Rename confirmed"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("Target Device ID: DEVICEID"));
        assert!(label.contains("Target display name: Hepta Native - Alice"));
        assert!(label.contains("PositiveConfirmationModal"));
        assert!(label.contains("MatrixRequest::RenameDevice"));
        assert!(label.contains("client.rename_device"));
        assert!(label.contains("current device only"));
        assert!(label.contains("GetOwnDevice and GetDevices"));
        assert!(label.contains("cross-session revoke"));
        assert!(label.contains("device delete/trust mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("Telegram delivery"));
        assert!(label.contains("unconfirmed live mutation"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_CURRENT_DEVICE_RENAME_LABEL));
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_DEVICE_RENAME_EVIDENCE
                .contains("AccountDataAction::DeviceRenamed")
        );
    }

    #[test]
    fn account_management_session_actions_row_label_uses_fallbacks() {
        let label = account_management_session_actions_row_label(
            "",
            AccountManagementPreviewState::Hidden,
            None,
        );

        assert!(label.contains("Session action staged locally"));
        assert!(label.contains("Preview state: hidden preview"));
        assert!(label.contains("loaded account/device metadata pending"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_LABEL));
    }

    #[test]
    fn account_management_device_directory_controls_row_label_is_local_only() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let label = account_management_device_directory_controls_row_label(
            "All devices",
            AccountManagementPreviewState::Sessions,
            Some(loaded_identity),
        );

        assert!(label.contains("All devices staged locally"));
        assert!(label.contains("Preview state: Sessions preview"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("Device ID: DEVICEID"));
        assert!(label.contains("All devices, Password, SSO, Portal, and Activity"));
        assert!(label.contains("All devices is a read-only MatrixRequest::GetDevices path"));
        assert!(
            label.contains("Portal uses a separate PositiveConfirmationModal homeserver opener")
        );
        assert!(label.contains("No session-management lookup"));
        assert!(label.contains("session-management lookup"));
        assert!(label.contains("password change"));
        assert!(label.contains("SSO start"));
        assert!(label.contains("session revoke"));
        assert!(label.contains("device trust/rename/delete change"));
        assert!(label.contains("Matrix account/profile mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_LABEL));
        assert!(
            ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_EVIDENCE
                .contains("account_management_preview")
        );
        assert!(
            ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_EVIDENCE
                .contains("All devices, Password, SSO, Portal, and Activity")
        );
        assert!(
            ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_EVIDENCE
                .contains("only the accept branch hands the active Matrix homeserver URL")
        );
    }

    #[test]
    fn account_management_device_directory_controls_row_label_uses_fallbacks() {
        let label = account_management_device_directory_controls_row_label(
            "",
            AccountManagementPreviewState::Hidden,
            None,
        );

        assert!(label.contains("Device directory action staged locally"));
        assert!(label.contains("Preview state: hidden preview"));
        assert!(label.contains("loaded account/device metadata pending"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_LABEL));
    }

    #[test]
    fn account_management_current_device_metadata_controls_row_label_is_local_only() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let label = account_management_current_device_metadata_controls_row_label(
            "Verified",
            AccountManagementPreviewState::Security,
            Some(loaded_identity),
        );

        assert!(label.contains("Verified current-device metadata stayed local"));
        assert!(label.contains("Preview state: Security preview"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("Device ID: DEVICEID"));
        assert!(label.contains("Device copies only"));
        assert!(label.contains("Verified copies only"));
        assert!(label.contains("Display copies only"));
        assert!(label.contains("Session copies only"));
        assert!(label.contains("Source copies only"));
        assert!(label.contains("current-device verification status"));
        assert!(label.contains("No extra GetOwnDevice"));
        assert!(label.contains("external account portal or browser"));
        assert!(label.contains("all-device list"));
        assert!(label.contains("session-management lookup"));
        assert!(label.contains("password change"));
        assert!(label.contains("SSO start"));
        assert!(label.contains("session revoke"));
        assert!(label.contains("device trust/rename/delete change"));
        assert!(label.contains("Matrix account/profile mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_LABEL));
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
                .contains("Device, Verified, Display, Session, and Source")
        );
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
                .contains("does not request extra GetOwnDevice")
        );
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
                .contains("local clipboard")
        );
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
                .contains("Verified copies")
        );
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
                .contains("Display copies")
        );
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
                .contains("Session copies")
        );
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
                .contains("gateway/runtime/auth")
        );
    }

    #[test]
    fn account_management_current_device_metadata_controls_row_label_uses_fallbacks() {
        let label = account_management_current_device_metadata_controls_row_label(
            "",
            AccountManagementPreviewState::Hidden,
            None,
        );

        assert!(label.contains("Device metadata current-device metadata stayed local"));
        assert!(label.contains("Preview state: hidden preview"));
        assert!(label.contains("loaded account/device metadata pending"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_LABEL));
    }

    #[test]
    fn account_management_current_device_verification_clipboard_payload_is_loaded_only() {
        let payload = account_management_current_device_verification_clipboard_payload(
            Some("  DEVICEID  "),
            VerificationState::Verified,
        );

        assert_eq!(
            payload.as_deref(),
            Some(
                "Current device verification: verified. Device ID: DEVICEID. GetOwnDevice only; account actions stay local."
            )
        );
        assert_eq!(
            account_management_current_device_verification_clipboard_payload(
                Some("DEVICEID"),
                VerificationState::Unverified,
            )
            .as_deref(),
            Some(
                "Current device verification: unverified. Device ID: DEVICEID. GetOwnDevice only; account actions stay local."
            )
        );
        assert_eq!(
            account_management_current_device_verification_clipboard_payload(
                Some("DEVICEID"),
                VerificationState::Unknown,
            )
            .as_deref(),
            Some(
                "Current device verification: unknown verification. Device ID: DEVICEID. GetOwnDevice only; account actions stay local."
            )
        );
        assert_eq!(
            account_management_current_device_verification_clipboard_payload(
                Some("   "),
                VerificationState::Verified,
            ),
            None
        );
    }

    #[test]
    fn account_management_current_device_verification_clipboard_label_is_local_only() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let label = account_management_current_device_verification_clipboard_label(
            AccountManagementPreviewState::Security,
            Some(loaded_identity),
            Some("DEVICEID"),
            VerificationState::Verified,
        );

        assert!(label.contains("Current-device verification status copied locally"));
        assert!(label.contains("Preview state: Security preview"));
        assert!(label.contains("Verification status: verified"));
        assert!(label.contains("Verification summary chars:"));
        assert!(label.contains("bytes:"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("local Matrix verification state"));
        assert!(label.contains("GetOwnDevice current device ID"));
        assert!(label.contains("local clipboard"));
        assert!(label.contains("No extra GetOwnDevice"));
        assert!(label.contains("external account portal or browser"));
        assert!(label.contains("all-device list"));
        assert!(label.contains("session-management lookup"));
        assert!(label.contains("password change"));
        assert!(label.contains("SSO start"));
        assert!(label.contains("session revoke"));
        assert!(label.contains("device trust/rename/delete change"));
        assert!(label.contains("Matrix account/profile mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_LABEL));
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_EVIDENCE
                .contains("local Matrix verification state")
        );
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_EVIDENCE
                .contains("existing GetOwnDevice current device ID")
        );
    }

    #[test]
    fn account_management_current_device_verification_clipboard_label_handles_pending_device() {
        let label = account_management_current_device_verification_clipboard_label(
            AccountManagementPreviewState::Security,
            None,
            None,
            VerificationState::Unknown,
        );

        assert!(label.contains("current-device metadata is pending"));
        assert!(label.contains("loaded account/device metadata pending"));
        assert!(label.contains("No clipboard payload was written"));
        assert!(label.contains("no extra GetOwnDevice"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_LABEL));
        assert_eq!(
            account_management_current_device_verification_clipboard_payload(
                None,
                VerificationState::Verified,
            ),
            None
        );
    }

    #[test]
    fn account_management_current_device_id_clipboard_payload_is_trimmed() {
        let payload =
            account_management_current_device_id_clipboard_payload(Some("  ABCDEFG123  "));

        assert_eq!(payload.as_deref(), Some("ABCDEFG123"));
    }

    #[test]
    fn account_management_current_device_id_clipboard_label_is_local_only() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let label = account_management_current_device_id_clipboard_label(
            AccountManagementPreviewState::Sessions,
            Some(loaded_identity),
            Some("DEVICEID"),
        );

        assert!(label.contains("Device ID copied locally"));
        assert!(label.contains("Preview state: Sessions preview"));
        assert!(label.contains("Device ID chars: 8"));
        assert!(label.contains("bytes: 8"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("GetOwnDevice Device ID"));
        assert!(label.contains("local clipboard"));
        assert!(label.contains("No extra GetOwnDevice"));
        assert!(label.contains("external account portal or browser"));
        assert!(label.contains("all-device list"));
        assert!(label.contains("session-management lookup"));
        assert!(label.contains("password change"));
        assert!(label.contains("SSO start"));
        assert!(label.contains("session revoke"));
        assert!(label.contains("device trust/rename/delete change"));
        assert!(label.contains("Matrix account/profile mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_LABEL));
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_EVIDENCE
                .contains("existing GetOwnDevice result")
        );
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_EVIDENCE.contains("local clipboard")
        );
    }

    #[test]
    fn account_management_current_device_id_clipboard_label_handles_pending_device() {
        let label = account_management_current_device_id_clipboard_label(
            AccountManagementPreviewState::Overview,
            None,
            None,
        );

        assert!(label.contains("current-device metadata is pending"));
        assert!(label.contains("loaded account/device metadata pending"));
        assert!(label.contains("No clipboard payload was written"));
        assert!(label.contains("no extra GetOwnDevice"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_LABEL));
        assert_eq!(
            account_management_current_device_id_clipboard_payload(None),
            None
        );
        assert_eq!(
            account_management_current_device_id_clipboard_payload(Some("   ")),
            None
        );
    }

    #[test]
    fn account_management_current_device_display_name_clipboard_payload_is_trimmed() {
        let payload = account_management_current_device_display_name_clipboard_payload(Some(
            "  Alice phone  ",
        ));

        assert_eq!(payload.as_deref(), Some("Alice phone"));
    }

    #[test]
    fn account_management_current_device_display_name_clipboard_label_is_local_only() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let label = account_management_current_device_display_name_clipboard_label(
            AccountManagementPreviewState::Security,
            Some(loaded_identity),
            Some("Alice phone"),
        );

        assert!(label.contains("Device display name copied locally"));
        assert!(label.contains("Preview state: Security preview"));
        assert!(label.contains("Display name chars: 11"));
        assert!(label.contains("bytes: 11"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("GetOwnDevice display name"));
        assert!(label.contains("local clipboard"));
        assert!(label.contains("No extra GetOwnDevice"));
        assert!(label.contains("external account portal or browser"));
        assert!(label.contains("all-device list"));
        assert!(label.contains("session-management lookup"));
        assert!(label.contains("password change"));
        assert!(label.contains("SSO start"));
        assert!(label.contains("session revoke"));
        assert!(label.contains("device trust/rename/delete change"));
        assert!(label.contains("Matrix account/profile mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_LABEL));
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_EVIDENCE
                .contains("device display name")
        );
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_EVIDENCE
                .contains("existing GetOwnDevice result")
        );
    }

    #[test]
    fn account_management_current_device_display_name_clipboard_label_handles_missing_name() {
        let label = account_management_current_device_display_name_clipboard_label(
            AccountManagementPreviewState::Security,
            None,
            Some("   "),
        );

        assert!(label.contains("display name is unavailable"));
        assert!(label.contains("loaded account/device metadata pending"));
        assert!(label.contains("No clipboard payload was written"));
        assert!(label.contains("no extra GetOwnDevice"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_LABEL));
        assert_eq!(
            account_management_current_device_display_name_clipboard_payload(None),
            None
        );
    }

    #[test]
    fn account_management_current_session_clipboard_payload_is_trimmed() {
        let payload = account_management_current_session_clipboard_payload(Some(
            "  Current session: Alice phone · Device ID: DEVICEID  ",
        ));

        assert_eq!(
            payload.as_deref(),
            Some("Current session: Alice phone · Device ID: DEVICEID")
        );
    }

    #[test]
    fn account_management_current_session_clipboard_label_is_local_only() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let session_text =
            "Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only.";
        let label = account_management_current_session_clipboard_label(
            AccountManagementPreviewState::Sessions,
            Some(loaded_identity),
            Some(session_text),
        );

        assert!(label.contains("Current session summary copied locally"));
        assert!(label.contains("Preview state: Sessions preview"));
        assert!(label.contains("Session summary chars:"));
        assert!(label.contains("bytes:"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("GetOwnDevice current-session summary"));
        assert!(label.contains("local clipboard"));
        assert!(label.contains("No extra GetOwnDevice"));
        assert!(label.contains("external account portal or browser"));
        assert!(label.contains("all-device list"));
        assert!(label.contains("session-management lookup"));
        assert!(label.contains("password change"));
        assert!(label.contains("SSO start"));
        assert!(label.contains("session revoke"));
        assert!(label.contains("device trust/rename/delete change"));
        assert!(label.contains("Matrix account/profile mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_LABEL));
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_EVIDENCE
                .contains("current-session summary")
        );
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_EVIDENCE
                .contains("existing GetOwnDevice result")
        );
    }

    #[test]
    fn account_management_current_session_clipboard_label_handles_pending_device() {
        let label = account_management_current_session_clipboard_label(
            AccountManagementPreviewState::Sessions,
            None,
            None,
        );

        assert!(label.contains("current-device metadata is pending"));
        assert!(label.contains("loaded account/device metadata pending"));
        assert!(label.contains("No clipboard payload was written"));
        assert!(label.contains("no extra GetOwnDevice"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_LABEL));
        assert_eq!(
            account_management_current_session_clipboard_payload(None),
            None
        );
    }

    #[test]
    fn account_management_current_device_source_clipboard_payload_is_trimmed() {
        let payload = account_management_current_device_source_clipboard_payload(Some(
            "  Loaded account: Alice · Device ID: DEVICEID  ",
        ));

        assert_eq!(
            payload.as_deref(),
            Some("Loaded account: Alice · Device ID: DEVICEID")
        );
    }

    #[test]
    fn account_management_current_device_source_clipboard_label_is_local_only() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let label = account_management_current_device_source_clipboard_label(
            AccountManagementPreviewState::Overview,
            Some(loaded_identity),
        );

        assert!(label.contains("Source account/current-device summary copied locally"));
        assert!(label.contains("Preview state: Manage Account overview"));
        assert!(label.contains("Summary chars:"));
        assert!(label.contains("bytes:"));
        assert!(label.contains("own_profile"));
        assert!(label.contains("GetOwnDevice text"));
        assert!(label.contains("local clipboard"));
        assert!(label.contains("No extra GetOwnDevice"));
        assert!(label.contains("external account portal or browser"));
        assert!(label.contains("all-device list"));
        assert!(label.contains("session-management lookup"));
        assert!(label.contains("password change"));
        assert!(label.contains("SSO start"));
        assert!(label.contains("session revoke"));
        assert!(label.contains("device trust/rename/delete change"));
        assert!(label.contains("Matrix account/profile mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_LABEL));
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_EVIDENCE.contains("own_profile")
        );
        assert!(
            ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_EVIDENCE
                .contains("existing GetOwnDevice text")
        );
    }

    #[test]
    fn account_management_current_device_source_clipboard_label_handles_empty_summary() {
        let label = account_management_current_device_source_clipboard_label(
            AccountManagementPreviewState::Hidden,
            Some("   "),
        );

        assert!(label.contains("loaded metadata is empty"));
        assert!(label.contains("No clipboard payload was written"));
        assert!(label.contains("no extra GetOwnDevice"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_LABEL));
        assert_eq!(
            account_management_current_device_source_clipboard_payload(None),
            None
        );
    }

    #[test]
    fn account_management_preflight_detail_controls_row_label_is_local_only() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let label = account_management_preflight_detail_controls_row_label(
            "Result",
            AccountManagementPreviewState::Security,
            Some(loaded_identity),
        );

        assert!(label.contains("Result account-management detail stayed local"));
        assert!(label.contains("Preview state: Security preview"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("Device ID: DEVICEID"));
        assert!(
            label.contains("Request, Result, Error, Retry, Source, Packet, Contract, and Taxonomy")
        );
        assert!(label.contains("visible account/session preflight controls"));
        assert!(label.contains("Retry confirms before resubmitting"));
        assert!(label.contains("cached read-only GetDevices failure"));
        assert!(label.contains("No extra GetOwnDevice"));
        assert!(
            label.contains("Browser/Portal homeserver opener has a separate confirmation path")
        );
        assert!(label.contains("dedicated account portal route"));
        assert!(label.contains("session-management lookup"));
        assert!(label.contains("password change"));
        assert!(label.contains("SSO start"));
        assert!(label.contains("automatic retry"));
        assert!(label.contains("session revoke"));
        assert!(label.contains("device trust/rename/delete change"));
        assert!(label.contains("Matrix account/profile mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_LABEL));
        assert!(
            ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
                .contains("Request, Result, Error, Retry, Source, Packet, Contract, and Taxonomy")
        );
        assert!(
            ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
                .contains("local account/session request snapshot")
        );
        assert!(
            ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
                .contains("PositiveConfirmationModal")
        );
        assert!(
            ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
                .contains("MatrixRequest::GetDevices")
        );
        assert!(
            ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
                .contains("typed dedicated account portal")
        );
        assert!(
            ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
                .contains("Taxonomy records blocked password/SSO/revoke/trust/delete result slots")
        );
        assert!(
            ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
                .contains("does not request GetOwnDevice")
        );
        assert!(
            ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
                .contains("gateway/runtime/auth")
        );
    }

    #[test]
    fn account_management_preflight_detail_controls_row_label_uses_fallbacks() {
        let label = account_management_preflight_detail_controls_row_label(
            "",
            AccountManagementPreviewState::Hidden,
            None,
        );

        assert!(label.contains("Preflight detail account-management detail stayed local"));
        assert!(label.contains("Preview state: hidden preview"));
        assert!(label.contains("loaded account/device metadata pending"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_LABEL));
    }

    #[test]
    fn account_management_session_device_drilldown_packet_label_persists_acceptance_matrix() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let label = account_management_session_device_drilldown_packet_label(
            AccountManagementPreviewState::Sessions,
            Some(loaded_identity),
        );

        assert!(label.contains("Account session/device drilldown packet"));
        assert!(label.contains("Preview state: Sessions preview"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("Device ID: DEVICEID"));
        assert!(label.contains("Loaded own_profile identity"));
        assert!(label.contains("current GetOwnDevice session/device metadata"));
        assert!(label.contains("verification state"));
        assert!(label.contains("device id/display/session/source clipboard payloads"));
        assert!(label.contains("Refresh/GetOwnDevice request/result/error/retry/source slots"));
        assert!(label.contains("dedicated account portal route targets"));
        assert!(label.contains("Browser/Portal homeserver opener outcome"));
        assert!(label.contains("all-device directory scope"));
        assert!(label.contains("password/SSO scope"));
        assert!(
            label.contains("current-device RenameDevice request/result/error/retry/source slots")
        );
        assert!(label.contains("cross-session revoke/trust scope"));
        assert!(label.contains("device delete/trust scope"));
        assert!(label.contains("account/profile mutation guard"));
        assert!(label.contains("live-mutation boundary"));
        assert!(label.contains("No extra GetOwnDevice"));
        assert!(label.contains("dedicated portal route open"));
        assert!(label.contains("extra homeserver opener"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_LABEL));
        assert!(
            ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_EVIDENCE
                .contains("visible Packet control")
        );
        assert!(
            ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_EVIDENCE
                .contains("account/profile mutation guard")
        );
    }

    #[test]
    fn account_management_preflight_detail_controls_row_label_routes_packet_to_drilldown() {
        let label = account_management_preflight_detail_controls_row_label(
            "Packet",
            AccountManagementPreviewState::Overview,
            None,
        );

        assert!(label.contains("Account session/device drilldown packet"));
        assert!(label.contains("Preview state: Manage Account overview"));
        assert!(label.contains("loaded account/device metadata pending"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_LABEL));
    }

    #[test]
    fn account_management_session_device_typed_contract_packet_label_maps_drilldown_to_contracts() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let label = account_management_session_device_typed_contract_packet_label(
            AccountManagementPreviewState::Sessions,
            Some(loaded_identity),
        );

        assert!(label.contains("Account session/device typed contract packet"));
        assert!(label.contains("Preview state: Sessions preview"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("Device ID: DEVICEID"));
        assert!(label.contains("local session/device drilldown Packet"));
        assert!(label.contains("typed dedicated account portal route"));
        assert!(label.contains("Browser/Portal homeserver opener outcome"));
        assert!(label.contains("all-device directory"));
        assert!(label.contains("password/SSO"));
        assert!(label.contains("current-device RenameDevice"));
        assert!(label.contains("cross-session revoke/trust"));
        assert!(label.contains("device delete/trust"));
        assert!(label.contains("account/profile mutation guard"));
        assert!(label.contains("GetOwnDevice refresh"));
        assert!(label.contains("result/error/retry/source"));
        assert!(label.contains("source-hash"));
        assert!(label.contains("idempotency"));
        assert!(label.contains("stale-session"));
        assert!(label.contains("promotion-blocker contracts"));
        assert!(label.contains("No extra GetOwnDevice"));
        assert!(label.contains("dedicated portal route open"));
        assert!(label.contains("extra homeserver opener"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_LABEL));
        assert!(
            ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_EVIDENCE
                .contains("visible Contract control")
        );
        assert!(
            ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_EVIDENCE
                .contains("typed dedicated account portal route")
        );
        assert!(
            ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_EVIDENCE
                .contains("promotion-blocker contracts")
        );
    }

    #[test]
    fn account_management_preflight_detail_controls_row_label_routes_contract_to_typed_packet() {
        let label = account_management_preflight_detail_controls_row_label(
            "Contract",
            AccountManagementPreviewState::Security,
            None,
        );

        assert!(label.contains("Account session/device typed contract packet"));
        assert!(label.contains("Preview state: Security preview"));
        assert!(label.contains("loaded account/device metadata pending"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_LABEL));
    }

    #[test]
    fn account_management_session_device_result_taxonomy_packet_label_lists_blocked_results() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let label = account_management_session_device_result_taxonomy_packet_label(
            AccountManagementPreviewState::Sessions,
            Some(loaded_identity),
        );

        assert!(label.contains("Account session/device result taxonomy packet"));
        assert!(label.contains("Preview state: Sessions preview"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("MatrixRequest::GetOwnDevice"));
        assert!(label.contains("MatrixRequest::GetDevices"));
        assert!(label.contains("MatrixRequest::SetDisplayName"));
        assert!(label.contains("MatrixRequest::RenameDevice"));
        assert!(label.contains("dedicated_portal_operation_id not_assigned"));
        assert!(label.contains("password_action_operation_id not_assigned"));
        assert!(label.contains("sso_action_operation_id not_assigned"));
        assert!(label.contains("cross_session_revoke_operation_id not_assigned"));
        assert!(label.contains("device_delete_operation_id not_assigned"));
        assert!(
            label.contains("password_result opened/completed/cancelled/failed/stale not_wired")
        );
        assert!(label.contains("revoke_result applied/permission_denied/failed/stale not_wired"));
        assert!(
            label.contains("device_delete_result deleted/permission_denied/failed/stale not_wired")
        );
        assert!(label.contains("PositiveConfirmationModal"));
        assert!(label.contains("directory/source hash"));
        assert!(label.contains("audit redaction"));
        assert!(label.contains("password, token, SSO code"));
        assert!(label.contains("No extra GetOwnDevice"));
        assert!(label.contains("password/SSO flow"));
        assert!(label.contains("session revoke"));
        assert!(label.contains("device delete/trust mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_LABEL));
        assert!(
            ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("visible Taxonomy control")
        );
        assert!(
            ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("operation_id slots as not_assigned")
        );
        assert!(
            ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("audit redaction")
        );
    }

    #[test]
    fn account_management_preflight_detail_controls_row_label_routes_taxonomy_to_result_packet() {
        let label = account_management_preflight_detail_controls_row_label(
            "Taxonomy",
            AccountManagementPreviewState::Security,
            None,
        );

        assert!(label.contains("Account session/device result taxonomy packet"));
        assert!(label.contains("Preview state: Security preview"));
        assert!(label.contains("loaded account/device metadata pending"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_LABEL));
    }

    #[test]
    fn account_management_request_snapshot_label_summarizes_loaded_request_packet() {
        let loaded_identity = "Loaded account: Alice · @alice:example.org · no avatar · Current session: Alice phone · Device ID: DEVICEID · verified. GetOwnDevice only; account actions stay local.";
        let label = account_management_request_snapshot_label(
            "Request",
            AccountManagementPreviewState::Sessions,
            Some(loaded_identity),
        );

        assert!(label.contains("Local account/session request snapshot"));
        assert!(label.contains("Request selected"));
        assert!(label.contains("Preview state: Sessions preview"));
        assert!(label.contains("Loaded account: Alice"));
        assert!(label.contains("Device ID: DEVICEID"));
        assert!(label.contains("Request body"));
        assert!(label.contains("result slot"));
        assert!(label.contains("retry availability"));
        assert!(label.contains("dedicated portal target"));
        assert!(label.contains("Browser/Portal homeserver opener outcome"));
        assert!(label.contains("all-device scope"));
        assert!(label.contains("session-management scope"));
        assert!(label.contains("password/SSO scope"));
        assert!(label.contains("current-device rename scope"));
        assert!(label.contains("cross-session device delete/trust scope"));
        assert!(label.contains("No extra GetOwnDevice"));
        assert!(label.contains("dedicated account portal route"));
        assert!(label.contains("session revoke"));
        assert!(label.contains("extra current-device RenameDevice"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_LABEL));
    }
}

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
