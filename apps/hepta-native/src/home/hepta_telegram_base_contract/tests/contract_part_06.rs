#[test]
fn hepta_telegram_base_account_avatar_upload_option_staging_has_picker_boundary() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_OPTION_STAGING_LOCAL_MARKER,
        "hepta_telegram_account_avatar_upload_option_staging_local_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_avatar_upload"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_option_staging_local_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar upload"
            && capability.notes.contains("desktop image picker")
            && capability.notes.contains("selected-file metadata preview")
            && capability.notes.contains("MatrixRequest::UploadAvatar")
            && capability.notes.contains("image editing")
            && capability
                .notes
                .contains("persistent thumbnail file handoff")
            && capability.notes.contains("browser handoff")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_avatar_upload_selected_file_preview_is_local_until_confirmed() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SELECTED_FILE_MARKER,
        "hepta_telegram_account_avatar_upload_selected_file_ready"
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SELECTED_FILE_EVIDENCE
            .contains("filename")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SELECTED_FILE_EVIDENCE
            .contains("MIME type")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SELECTED_FILE_EVIDENCE
            .contains("local file size")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SELECTED_FILE_EVIDENCE
            .contains("extension")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SELECTED_FILE_EVIDENCE
            .contains("dimensions status")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SELECTED_FILE_EVIDENCE
            .contains("before confirmation")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SELECTED_FILE_EVIDENCE
            .contains("UploadAvatar")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SELECTED_FILE_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SELECTED_FILE_EVIDENCE
            .contains("live mutation")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_selected_file_preview")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar selected file preview"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("avatar_upload_selection_preview")
            && capability.notes.contains("filename")
            && capability.notes.contains("MIME type")
            && capability.notes.contains("local file size")
            && capability.notes.contains("extension")
            && capability.notes.contains("dimensions status")
            && capability.notes.contains("confirmation open/cancel")
            && capability.notes.contains("MatrixRequest::UploadAvatar")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_avatar_upload_selected_image_metadata_is_header_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SELECTED_IMAGE_METADATA_MARKER,
        "hepta_telegram_account_avatar_upload_selected_image_metadata_ready"
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SELECTED_IMAGE_METADATA_EVIDENCE
            .contains("already selected local file path")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SELECTED_IMAGE_METADATA_EVIDENCE
            .contains("dimensions status")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SELECTED_IMAGE_METADATA_EVIDENCE
            .contains("PNG, JPEG, GIF, BMP, or WebP header dimensions")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SELECTED_IMAGE_METADATA_EVIDENCE
            .contains("no thumbnail decode")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SELECTED_IMAGE_METADATA_EVIDENCE
            .contains("full image decode")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SELECTED_IMAGE_METADATA_EVIDENCE
            .contains("SetAvatar(Some)")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SELECTED_IMAGE_METADATA_EVIDENCE
            .contains("MatrixRequest::UploadAvatar")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_selected_image_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar selected image metadata preview"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.base_module.contains("header metadata")
            && capability
                .notes
                .contains("already selected local file header")
            && capability.notes.contains("PNG, JPEG, GIF, BMP, or WebP")
            && capability.notes.contains("thumbnail decode")
            && capability.notes.contains("full image decode")
            && capability.notes.contains("cropper/editor")
            && capability.notes.contains("upload")
            && capability.notes.contains("SetAvatar(Some)")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_avatar_upload_decode_probe_is_bounded_pixel_decode() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_MARKER,
        "hepta_telegram_account_avatar_upload_decode_probe_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_PIXEL_DECODE_LIVE_MARKER,
        "hepta_telegram_account_avatar_upload_pixel_decode_live_ready"
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_EVIDENCE
            .contains("Thumbnail and Full-size controls")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_EVIDENCE
            .contains("already selected local image file")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_EVIDENCE
            .contains("bounded local pixel decode")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_EVIDENCE
            .contains("byte and pixel budgets")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_EVIDENCE
            .contains("in-memory 128px RGBA thumbnail buffer")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_EVIDENCE
            .contains("creates no thumbnail file")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_EVIDENCE
            .contains("Full-size decodes the original RGBA pixel buffer")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_EVIDENCE
            .contains("UploadAvatar")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_avatar_upload_decode_probe"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_avatar_upload_pixel_decode_live")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar thumbnail/full-size pixel decode"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("account_avatar_upload_decode_probe_label")
            && capability
                .notes
                .contains("PNG/JPEG/GIF/BMP/WebP header parser")
            && capability
                .notes
                .contains("in-memory 128px RGBA thumbnail buffer")
            && capability.notes.contains("original RGBA pixel buffer")
            && capability.notes.contains("creates no thumbnail file")
            && capability.notes.contains("UploadAvatar")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_avatar_upload_boundary_keeps_remaining_local_gaps() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_MARKER,
        "hepta_telegram_account_avatar_upload_lifecycle_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_account_avatar_upload_local_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_avatar_upload"));
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_LOCAL_BOUNDARY_EVIDENCE
            .contains("desktop image picker plus confirmation")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_LOCAL_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::UploadAvatar")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_LOCAL_BOUNDARY_EVIDENCE
            .contains("Account::set_avatar_url(Some(mxc))")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_LOCAL_BOUNDARY_EVIDENCE
            .contains("Crop, Cancel, picker cancel")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_LOCAL_BOUNDARY_EVIDENCE
            .contains("mobile camera/photo-library capture")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_LOCAL_BOUNDARY_EVIDENCE
            .contains("direct MXC editor")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_LOCAL_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::SetAvatar(Some")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_LOCAL_BOUNDARY_LABEL
            .contains("direct MXC SetAvatar(Some)")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_EVIDENCE
            .contains("picker opened")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_EVIDENCE
            .contains("confirmation canceled")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_EVIDENCE
            .contains("confirmed upload handoff")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_EVIDENCE
            .contains(
            "MatrixRequest::UploadAvatar is still submitted only from the confirmed accept handler"
        )
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_LABEL
            .contains("UploadAvatar waits for confirmation")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_local_boundary_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_lifecycle_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar upload local boundary evidence"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.base_module.contains("avatar_upload_preview")
            && capability.notes.contains("MatrixRequest::UploadAvatar")
            && capability
                .notes
                .contains("Account::set_avatar_url(Some(mxc))")
            && capability.notes.contains("desktop image picker")
            && capability.notes.contains("image editing")
            && capability.notes.contains("Crop")
            && capability.notes.contains("camera")
            && capability.notes.contains("photo-library")
            && capability.notes.contains("thumbnail generation")
            && capability.notes.contains("SetAvatar(Some)")
            && capability.notes.contains("message send/edit/redact")
            && capability.notes.contains("Direct MXC SetAvatar(Some)")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar upload lifecycle metadata"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.notes.contains("picker opened")
            && capability.notes.contains("confirmation canceled")
            && capability.notes.contains("confirmed upload handoff")
            && capability.notes.contains("validation reason")
            && capability
                .notes
                .contains("MatrixRequest::UploadAvatar remains gated")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_avatar_upload_retry_confirmation_is_narrow() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_MARKER,
        "hepta_telegram_account_avatar_upload_retry_confirmation_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_avatar_upload"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_retry_confirmation_guard")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_EVIDENCE
            .contains("cached local file path")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_EVIDENCE
            .contains("PositiveConfirmationModal")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_EVIDENCE
            .contains("MatrixRequest::UploadAvatar")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_EVIDENCE
            .contains("browser handoff")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_EVIDENCE
            .contains("live mutation")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar upload retry confirmation"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("show_avatar_upload_retry_confirmation")
            && capability.notes.contains("cached local file path")
            && capability.notes.contains("MIME type")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("confirmed accept handler")
            && capability.notes.contains("no new picker")
            && capability.notes.contains("camera/photo-library")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_account_avatar_crop_editor_boundary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_MARKER,
        "hepta_telegram_account_avatar_upload_crop_editor_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_avatar_upload"));
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_EVIDENCE
            .contains(
                "existing desktop picker plus confirmation-gated MatrixRequest::UploadAvatar"
            )
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_EVIDENCE
            .contains("AvatarUploadPreviewState")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_EVIDENCE
            .contains("local avatar cropper packet snapshot")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_EVIDENCE
            .contains("aspect-ratio presets")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_EVIDENCE
            .contains("mobile camera capture")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_EVIDENCE
            .contains("mobile photo-library capture")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_EVIDENCE
            .contains("direct SetAvatar")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_LABEL
            .contains("Crop/editor")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_crop_editor_boundary")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar crop editor boundary"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("account_avatar_upload_crop_editor_boundary_label")
            && capability
                .base_module
                .contains("account_avatar_upload_cropper_snapshot_label")
            && capability.notes.contains("AvatarUploadPreviewState")
            && capability
                .notes
                .contains("local avatar cropper packet snapshot")
            && capability.notes.contains("crop box")
            && capability.notes.contains("aspect-ratio presets")
            && capability.notes.contains("rotate/zoom")
            && capability.notes.contains("mobile camera capture")
            && capability.notes.contains("mobile photo-library capture")
            && capability.notes.contains("SetAvatar(Some)")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_avatar_editor_controls_row_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_MARKER,
        "hepta_telegram_account_avatar_upload_editor_controls_row_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_avatar_upload"));
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_EVIDENCE
            .contains("Aspect, Rotate, Zoom, Camera, and Library")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_EVIDENCE
            .contains("AvatarUploadPreviewState")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_EVIDENCE
            .contains("local avatar cropper packet snapshot")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_EVIDENCE
            .contains("selected image metadata")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_EVIDENCE
            .contains("no cropper/editor")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_EVIDENCE
            .contains("photo-library picker")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_EVIDENCE
            .contains("UploadAvatar")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_EDITOR_CONTROLS_ROW_LABEL
            .contains("visible local avatar controls")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_avatar_upload_editor_controls_row")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar editor controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.base_module.contains("avatar_editor_controls")
            && capability
                .notes
                .contains("Aspect, Rotate, Zoom, Camera, and Library")
            && capability
                .notes
                .contains("local avatar cropper packet snapshot")
            && capability.notes.contains("visible local controls")
            && capability.notes.contains("AvatarUploadPreviewState")
            && capability.notes.contains("no cropper/editor")
            && capability.notes.contains("thumbnail generation")
            && capability.notes.contains("photo-library picker")
            && capability.notes.contains("UploadAvatar")
            && capability.notes.contains("SetAvatar(Some)")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_avatar_source_preview_controls_row_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_MARKER,
        "hepta_telegram_account_avatar_upload_source_preview_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_avatar_upload"));
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_EVIDENCE
            .contains(
                "Source, Camera, Library, Thumbnail, Full-size, Packet, Contract, and Taxonomy"
            )
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_EVIDENCE
            .contains("AvatarUploadPreviewState")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_EVIDENCE
            .contains("selected image metadata")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_EVIDENCE
            .contains("opens no file picker")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_EVIDENCE
            .contains("bounded local pixel decode")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_EVIDENCE
            .contains("in-memory RGBA preview buffers")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_EVIDENCE
            .contains("persistent thumbnail file")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_EVIDENCE
            .contains(
                "Taxonomy records source/cropper/camera/library/thumbnail artifact result slots"
            )
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_LABEL
            .contains("local avatar source controls")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_source_preview_controls_row")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar source preview controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("avatar_upload_source_preview_controls")
            && capability
                .base_module
                .contains("stage_avatar_upload_source_preview_control")
            && capability.notes.contains(
                "Source, Camera, Library, Thumbnail, Full-size, Packet, Contract, and Taxonomy",
            )
            && capability
                .notes
                .contains("visible local source/preview controls")
            && capability.notes.contains("Contract maps that matrix")
            && capability.notes.contains("Taxonomy records")
            && capability.notes.contains("AvatarUploadPreviewState")
            && capability.notes.contains("opens no file picker")
            && capability.notes.contains("camera capture")
            && capability.notes.contains("photo-library picker")
            && capability.notes.contains("bounded local pixel decode")
            && capability.notes.contains("in-memory RGBA buffers")
            && capability.notes.contains("persistent thumbnail file")
            && capability.notes.contains("UploadAvatar")
            && capability.notes.contains("SetAvatar(Some)")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_avatar_source_editor_drilldown_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_MARKER,
        "hepta_telegram_account_avatar_upload_source_editor_drilldown_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_MARKER,
        "hepta_telegram_account_avatar_upload_source_editor_typed_contract_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_avatar_upload"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_source_editor_drilldown_packet_action")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_EVIDENCE
            .contains("visible Packet control")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_EVIDENCE
            .contains("source type, desktop file path handoff")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_EVIDENCE
            .contains("MIME/extension/size/dimensions")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_EVIDENCE
            .contains("crop box/aspect/rotate/zoom")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_EVIDENCE
            .contains("camera/photo-library permission and picker states")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_EVIDENCE
            .contains("UploadAvatar request/result/error/retry/source slots")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_EVIDENCE
            .contains("SetAvatar handoff")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_EVIDENCE
            .contains("source mutation")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_DRILLDOWN_PACKET_LABEL
            .contains("acceptance criteria stay local")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar source editor drilldown packet"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("avatar_source_preview_packet_button")
            && capability
                .base_module
                .contains("account_avatar_upload_source_editor_drilldown_packet_label")
            && capability
                .notes
                .contains("source/editor drilldown acceptance matrix")
            && capability.notes.contains("AvatarUploadPreviewState")
            && capability.notes.contains("camera/photo-library permission")
            && capability
                .notes
                .contains("thumbnail/full-size decode targets")
            && capability
                .notes
                .contains("UploadAvatar request/result/error/retry/source")
            && capability.notes.contains("SetAvatar handoff")
            && capability
                .notes
                .contains("account_avatar_upload remains a base gap")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_avatar_source_editor_typed_contract_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_MARKER,
        "hepta_telegram_account_avatar_upload_source_editor_typed_contract_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_avatar_upload"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_source_editor_typed_contract_packet_action")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("visible Contract control")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("typed cropper, camera, image-edit, thumbnail/full-size decode")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("camera/photo-library permission and picker request/result/error slots")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("UploadAvatar request/result/error/retry/source slots")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("direct SetAvatar(Some) request/result/retry mapping")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("idempotency")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_TYPED_CONTRACT_PACKET_LABEL
            .contains("direct MXC SetAvatar(Some) is live")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar source editor typed contract packet"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("avatar_source_preview_contract_button")
            && capability
                .base_module
                .contains("account_avatar_upload_source_editor_typed_contract_packet_label")
            && capability
                .notes
                .contains("typed cropper-camera contract packet")
            && capability.notes.contains("AvatarUploadPreviewState")
            && capability.notes.contains("camera/photo-library permission")
            && capability.notes.contains("thumbnail/full-size decode")
            && capability
                .notes
                .contains("UploadAvatar request/result/error/retry/source")
            && capability
                .notes
                .contains("direct SetAvatar(Some) request/result/retry mapping")
            && capability
                .notes
                .contains("account_avatar_upload remains a base gap")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "account_avatar_upload"
            && runway
                .current_path
                .contains("typed cropper-camera contract packet")
            && runway.next_ui_safe_step.contains(
                "coordinate backend avatar source/cropper/camera/editor/thumbnail contracts",
            )
    }));
}

#[test]
fn hepta_telegram_base_account_avatar_source_editor_result_taxonomy_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_account_avatar_upload_source_editor_result_taxonomy_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_avatar_upload"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_source_editor_result_taxonomy_packet_action")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("visible Taxonomy control")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("confirmed desktop UploadAvatar")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("SDK Account::set_avatar_url(Some)")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("direct MXC SetAvatar(Some)")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("SetAvatar(None) delete")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("camera permission")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("persistent thumbnail artifact")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("transformed SetAvatar mapping")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("audit redaction")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_LABEL
            .contains("results local")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar source editor result taxonomy packet"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("avatar_source_preview_taxonomy_button")
            && capability
                .base_module
                .contains("account_avatar_upload_source_editor_result_taxonomy_packet_label")
            && capability
                .notes
                .contains("account_avatar_upload remains a base gap")
            && capability.notes.contains("confirmed desktop UploadAvatar")
            && capability
                .notes
                .contains("SDK Account::set_avatar_url(Some)")
            && capability.notes.contains("direct MXC SetAvatar(Some)")
            && capability.notes.contains("SetAvatar(None) delete")
            && capability.notes.contains("camera/photo-library permission")
            && capability
                .notes
                .contains("crop box/aspect/rotate/zoom result")
            && capability
                .notes
                .contains("persistent thumbnail artifact id")
            && capability.notes.contains("transformed SetAvatar result")
            && capability.notes.contains("audit redaction")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "account_avatar_upload"
            && runway
                .current_path
                .contains("source/cropper/camera/editor artifact result taxonomy packet")
            && runway.next_ui_safe_step.contains(
                "coordinate backend avatar source/cropper/camera/editor/thumbnail contracts",
            )
    }));
}

#[test]
fn hepta_telegram_base_account_avatar_source_path_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_SOURCE_PATH_CLIPBOARD_MARKER,
        "hepta_telegram_account_avatar_source_path_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_SOURCE_PATH_CLIPBOARD_MARKER,
        "hepta_telegram_account_avatar_upload_source_path_clipboard_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_avatar_upload"));
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_PATH_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_PATH_CLIPBOARD_EVIDENCE
            .contains("AvatarUploadSelectionPreview")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_PATH_CLIPBOARD_EVIDENCE
            .contains("opens no file picker")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_PATH_CLIPBOARD_EVIDENCE
            .contains("UploadAvatar")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_SOURCE_PATH_CLIPBOARD_LABEL
            .contains("clipboard only")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_source_path_clipboard_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar source path clipboard"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("copy_avatar_upload_source_path")
            && capability.notes.contains("local avatar file path")
            && capability.notes.contains("local clipboard")
            && capability.notes.contains("AvatarUploadSelectionPreview")
            && capability.notes.contains("opens no file picker")
            && capability.notes.contains("thumbnail decode/generation")
            && capability.notes.contains("full image decode")
            && capability.notes.contains("UploadAvatar")
            && capability.notes.contains("SetAvatar(Some)")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_avatar_upload_preflight_detail_controls_row_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_account_avatar_upload_preflight_detail_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_avatar_upload"));
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("Request, Result, Error, Retry, and Source")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("AvatarUploadPreviewState")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("selected image metadata")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("opens no file picker")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("UploadAvatar")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_LABEL
            .contains("visible local UploadAvatar details")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_preflight_detail_controls_row")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar upload preflight detail controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("avatar_upload_preflight_controls")
            && capability
                .notes
                .contains("Request, Result, Error, Retry, and Source")
            && capability
                .notes
                .contains("UploadAvatar preflight detail controls")
            && capability.notes.contains("AvatarUploadPreviewState")
            && capability.notes.contains("opens no file picker")
            && capability.notes.contains("image decode")
            && capability.notes.contains("photo-library picker")
            && capability.notes.contains("UploadAvatar")
            && capability.notes.contains("SetAvatar(Some)")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_device_self_check_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_DEVICE_SELF_CHECK_MARKER,
        "hepta_telegram_account_device_self_check_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_device_self_check_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_device_self_check"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account device self-check"
            && capability
                .notes
                .contains("existing Matrix GetOwnDevice read path")
            && capability
                .notes
                .contains("only while own_device is missing")
            && capability
                .notes
                .contains("verified/unverified device evidence")
            && capability
                .notes
                .contains("session name, and Device ID locally")
            && capability.notes.contains("device-list lookup")
            && capability.notes.contains(
                "profile mutation, message, room-state, and membership requests remain unsent",
            )
    }));
}

#[test]
fn hepta_telegram_base_account_avatar_delete_confirmation_is_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_DELETE_CONFIRMATION_MARKER,
        "hepta_telegram_account_avatar_delete_confirmation_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_avatar_delete_confirmation_guard")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_avatar_delete_live_wiring"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_avatar_delete"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar delete confirmation"
            && capability
                .notes
                .contains("Matrix SetAvatar(None) is requested from the confirmed accept handler")
            && capability.notes.contains(
                "AvatarChanged result data repaints cached profile/avatar widgets locally",
            )
            && capability.notes.contains("Cancel keeps SetAvatar(None)")
            && capability.notes.contains("upload, display-name")
            && capability.notes.contains("device/session-management")
            && capability
                .notes
                .contains("message, room-state, and membership requests unsent")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar delete live wiring"
            && capability
                .base_module
                .contains("MatrixRequest::SetAvatar(None)")
            && capability
                .base_module
                .contains("client.account().set_avatar_url(None)")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability
                .notes
                .contains("MatrixRequest::SetAvatar { avatar_url: None }")
            && capability.notes.contains("AvatarChanged(None)")
            && capability.notes.contains("SetAvatar(Some) handoff")
            && capability.notes.contains("gateway/runtime/auth")
    }));
}

#[test]
fn hepta_telegram_base_account_management_surface_uses_current_device_read() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_LOCAL_SURFACE_MARKER,
        "hepta_telegram_account_management_local_surface_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_management_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_option_staging_local_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management"
            && capability
                .notes
                .contains("Matrix GetOwnDevice current-session/device details")
            && capability
                .notes
                .contains("MatrixRequest::GetDevices read-only directory")
            && capability
                .notes
                .contains("loaded own_profile account identity")
            && capability.notes.contains("Security, Sessions")
            && capability.notes.contains("current-session/device details")
            && capability
                .notes
                .contains("update AccountManagementPreviewState")
            && capability.notes.contains("cross-session revoke")
            && capability
                .notes
                .contains("Matrix account mutations beyond display name remain TODO")
    }));
}

#[test]
fn hepta_telegram_base_account_management_live_wiring_is_partial_live() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_LIVE_WIRING_MARKER,
        "hepta_telegram_account_management_live_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_DISPLAY_NAME_LIVE_WIRING_MARKER,
        "hepta_telegram_account_management_display_name_live_wiring_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_management_live_wiring"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_display_name_live_wiring")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_device_directory_live_wiring")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_current_device_rename_live_wiring")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_management_browser_portal_handoff")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management live wiring"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("MatrixRequest::GetOwnDevice")
            && capability.base_module.contains("MatrixRequest::GetDevices")
            && capability
                .base_module
                .contains("MatrixRequest::SetDisplayName")
            && capability
                .base_module
                .contains("MatrixRequest::RenameDevice")
            && capability
                .base_module
                .contains("show_account_management_device_rename_confirmation")
            && capability.notes.contains("partial-live")
            && capability.notes.contains("loaded own_profile identity")
            && capability.notes.contains("MatrixRequest::GetOwnDevice")
            && capability.notes.contains("MatrixRequest::GetDevices")
            && capability.notes.contains("OwnDevicesFetched")
            && capability.notes.contains("PositiveConfirmationModal Retry")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("MatrixRequest::SetDisplayName")
            && capability
                .notes
                .contains("client.account().set_display_name")
            && capability.notes.contains("DisplayNameChanged")
            && capability
                .notes
                .contains("DisplayNameChangeFailed keeps the draft editable")
            && capability
                .notes
                .contains("resubmit MatrixRequest::SetDisplayName")
            && capability.notes.contains("MatrixRequest::RenameDevice")
            && capability.notes.contains("client.rename_device")
            && capability
                .notes
                .contains("AccountDataAction::DeviceRenamed")
            && capability.notes.contains("Device ID")
            && capability.notes.contains("display name")
            && capability.notes.contains("verification")
            && capability.notes.contains("session summary")
            && capability
                .base_module
                .contains("show_account_management_browser_portal_confirmation")
            && capability
                .notes
                .contains("Browser/Portal homeserver opener")
            && capability.notes.contains("get_client().homeserver")
            && capability.notes.contains("robius_open")
            && capability
                .notes
                .contains("Dedicated account-management portal routes")
            && capability.notes.contains("password/SSO")
            && capability.notes.contains("cross-session revoke")
            && capability.notes.contains("device delete/trust mutation")
            && capability
                .notes
                .contains("profile/account mutations beyond display name and current-device rename")
            && capability
                .notes
                .contains("unconfirmed write-side live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management browser portal handoff"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("show_account_management_browser_portal_confirmation")
            && capability.base_module.contains("get_client().homeserver")
            && capability.base_module.contains("PositiveConfirmationModal")
            && capability.base_module.contains("robius_open")
            && capability.notes.contains("validate http/https")
            && capability.notes.contains("strip query and fragment")
            && capability.notes.contains("only the accept branch")
            && capability.notes.contains("submits no MatrixRequest")
            && capability.notes.contains("password/SSO")
            && capability
                .notes
                .contains("dedicated account-management portal route")
            && capability.notes.contains("session-management lookup")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("Telegram delivery")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management display name live wiring"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("MatrixRequest::SetDisplayName")
            && capability
                .base_module
                .contains("client.account().set_display_name")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("MatrixRequest::SetDisplayName")
            && capability.notes.contains("DisplayNameChanged")
            && capability.notes.contains("DisplayNameChangeFailed")
            && capability.notes.contains("re-enables the staged draft")
            && capability
                .notes
                .contains("confirmed SetDisplayName resubmit")
            && capability
                .notes
                .contains("Cancel/reset keeps SetDisplayName unsent")
            && capability.notes.contains("session/device mutation")
            && capability.notes.contains("gateway/runtime/auth")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management device directory live wiring"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.base_module.contains("MatrixRequest::GetDevices")
            && capability.base_module.contains("client.devices")
            && capability
                .base_module
                .contains("AccountDataAction::OwnDevicesFetched")
            && capability.notes.contains("read-only directory")
            && capability.notes.contains("MatrixRequest::GetDevices")
            && capability.notes.contains("client.devices")
            && capability.notes.contains("AccountDeviceDirectoryEntry")
            && capability.notes.contains("last-seen IP")
            && capability.notes.contains("own_devices_last_error")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("cross-session revoke/trust")
            && capability.notes.contains("device delete/trust")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("write-side live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management current device rename live wiring"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("MatrixRequest::RenameDevice")
            && capability.base_module.contains("client.rename_device")
            && capability
                .base_module
                .contains("AccountDataAction::DeviceRenamed")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("loaded GetOwnDevice metadata")
            && capability.notes.contains("current Device ID")
            && capability.notes.contains("MatrixRequest::GetOwnDevice")
            && capability.notes.contains("MatrixRequest::GetDevices")
            && capability.notes.contains("failed rename_device")
            && capability.notes.contains("device delete/trust mutation")
            && capability.notes.contains("Telegram delivery")
            && capability
                .notes
                .contains("unconfirmed write-side live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "account_management"
            && runway
                .current_path
                .contains("live GetOwnDevice current-session read/refresh wiring")
            && runway
                .current_path
                .contains("live GetDevices read-only all-device directory wiring")
            && runway
                .current_path
                .contains("confirmed failed-state GetDevices Retry resubmit")
            && runway
                .current_path
                .contains("live confirmed Browser/Portal active homeserver")
            && runway
                .current_path
                .contains("live confirmed SetDisplayName profile display-name mutation")
            && runway
                .current_path
                .contains("failed-state confirmed Save Name resubmit")
            && runway
                .current_path
                .contains("live confirmed current-device Rename")
            && runway.current_path.contains("MatrixRequest::RenameDevice")
            && runway.current_path.contains("client.rename_device")
            && runway
                .remaining_gap
                .contains("dedicated account-management portal route")
            && runway.remaining_gap.contains("password/SSO")
            && runway.remaining_gap.contains("session revoke/trust")
            && runway
                .remaining_gap
                .contains("cross-session device management")
            && runway
                .remaining_gap
                .contains("device delete/trust mutation")
            && runway
                .remaining_gap
                .contains("account/profile mutations beyond display name and current-device rename")
    }));
}

#[test]
fn hepta_telegram_base_account_management_option_staging_keeps_mutations_unwired() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_OPTION_STAGING_LOCAL_MARKER,
        "hepta_telegram_account_management_option_staging_local_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_option_staging_local_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management"
            && capability
                .notes
                .contains("Manage Account, Security, Sessions, and Close")
            && capability
                .notes
                .contains("loaded own_profile account identity")
            && capability
                .notes
                .contains("Matrix GetOwnDevice current-session/device details")
            && capability
                .notes
                .contains("confirmed Browser/Portal homeserver")
            && capability
                .notes
                .contains("dedicated account-management portal routes")
            && capability.notes.contains("password/SSO change")
            && capability
                .notes
                .contains("MatrixRequest::GetDevices read-only directory")
            && capability.notes.contains("session-management mutation")
            && capability.notes.contains("Matrix account mutation")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_management_loaded_identity_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_LOADED_IDENTITY_MARKER,
        "hepta_telegram_account_management_loaded_identity_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LOADED_IDENTITY_EVIDENCE
            .contains("already loaded own_profile")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LOADED_IDENTITY_EVIDENCE
            .contains("Matrix user id")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LOADED_IDENTITY_EVIDENCE
            .contains("avatar state")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LOADED_IDENTITY_EVIDENCE
            .contains("GetOwnDevice current-session")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LOADED_IDENTITY_EVIDENCE
            .contains("no Matrix profile lookup")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LOADED_IDENTITY_EVIDENCE
            .contains("avatar fetch")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LOADED_IDENTITY_EVIDENCE
            .contains("all-device directory summary")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LOADED_IDENTITY_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_loaded_identity_preview")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management loaded identity preview"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("account_management_loaded_identity_text")
            && capability.notes.contains("already loaded own_profile")
            && capability
                .notes
                .contains("Matrix GetOwnDevice current-session")
            && capability.notes.contains("no Matrix profile lookup")
            && capability.notes.contains("avatar fetch")
            && capability
                .notes
                .contains("read-only GetDevices directory summary")
            && capability.notes.contains("cross-session revoke")
            && capability.notes.contains("Matrix account mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_management_lifecycle_metadata_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_MARKER,
        "hepta_telegram_account_management_lifecycle_metadata_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_EVIDENCE
            .contains("AccountManagementPreviewState")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_EVIDENCE
            .contains("loaded own_profile identity")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_EVIDENCE
            .contains("current Matrix device/session text")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_EVIDENCE
            .contains("Manage Account, Security, Sessions, and Close")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_EVIDENCE
            .contains("Browser/Portal open PositiveConfirmationModal")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_EVIDENCE
            .contains("All devices submits MatrixRequest::GetDevices")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_EVIDENCE
            .contains("MatrixRequest::GetOwnDevice only while current device data is missing")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_EVIDENCE
            .contains("Close only hides the local preview")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LIFECYCLE_METADATA_LABEL
            .contains("confirmed Browser/Portal homeserver opener")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_lifecycle_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management lifecycle metadata"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("account_management_lifecycle_metadata_label")
            && capability.notes.contains(
                "Manage Account, Security, Sessions, All devices, Browser/Portal, Rename, and Close",
            )
            && capability.notes.contains("AccountManagementPreviewState")
            && capability
                .notes
                .contains("account_management_loaded_identity_text")
            && capability.notes.contains("MatrixRequest::GetOwnDevice")
            && capability.notes.contains("MatrixRequest::GetDevices")
            && capability.notes.contains("only when own_device is missing")
            && capability.notes.contains("Close only hides")
            && capability
                .notes
                .contains("homeserver system opener handoff")
            && capability.notes.contains("session-management mutation")
            && capability.notes.contains("cross-session revoke")
            && capability.notes.contains("Matrix account mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_management_refresh_confirmation_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_MARKER,
        "hepta_telegram_account_management_refresh_confirmation_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE
            .contains("PositiveConfirmationModal")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE
            .contains("MatrixRequest::GetOwnDevice")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE
            .contains("Device display name")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE
            .contains("external account page")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE
            .contains("session-management lookup")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_LABEL
            .contains("GetOwnDevice")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_refresh_confirmation_guard")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management refresh confirmation"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("account_management_refresh_confirmation_label")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("MatrixRequest::GetOwnDevice")
            && capability.notes.contains("current Device display name")
            && capability.notes.contains("external account page")
            && capability.notes.contains("session-management mutation")
            && capability.notes.contains("cross-session revoke")
            && capability.notes.contains("Matrix account mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_management_boundary_keeps_remaining_gaps() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_account_management_local_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LOCAL_BOUNDARY_EVIDENCE
            .contains("Matrix GetOwnDevice")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LOCAL_BOUNDARY_EVIDENCE
            .contains("current account/session/device")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LOCAL_BOUNDARY_EVIDENCE
            .contains("confirmed Browser/Portal homeserver opener")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LOCAL_BOUNDARY_EVIDENCE
            .contains("dedicated account-management portal route")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LOCAL_BOUNDARY_EVIDENCE
            .contains("Password change")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LOCAL_BOUNDARY_EVIDENCE
            .contains("session-management lookup")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LOCAL_BOUNDARY_EVIDENCE
            .contains("cross-session revoke")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_LOCAL_BOUNDARY_LABEL
            .contains("confirmed homeserver opener")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_local_boundary_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management local boundary evidence"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("account_management_preview")
            && capability.notes.contains("GetOwnDevice")
            && capability.notes.contains("Browser/Portal can confirm")
            && capability
                .notes
                .contains("Dedicated account-management portal routes")
            && capability.notes.contains("password change")
            && capability.notes.contains("SSO change")
            && capability
                .notes
                .contains("read-only all-device directory details")
            && capability.notes.contains("session-management mutation")
            && capability.notes.contains("cross-session revoke")
            && capability.notes.contains("account-data mutation")
            && capability.notes.contains("Matrix account mutation")
            && capability.notes.contains("profile/account mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_management_session_revoke_boundary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_MARKER,
        "hepta_telegram_account_management_session_revoke_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_EVIDENCE
            .contains("AccountManagementPreviewState")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_EVIDENCE
            .contains("GetOwnDevice previews")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_EVIDENCE
            .contains("Dedicated external account page routes")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_EVIDENCE
            .contains("read-only GetDevices directory")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_EVIDENCE
            .contains("confirmed current-device Rename path")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_EVIDENCE
            .contains("cross-session revoke")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_EVIDENCE
            .contains("device delete/trust changes")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_LABEL
            .contains("all-device management beyond read-only directory")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_session_revoke_boundary")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management session revoke boundary"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("account_management_session_revoke_boundary_label")
            && capability.notes.contains("AccountManagementPreviewState")
            && capability.notes.contains("GetOwnDevice")
            && capability
                .notes
                .contains("separate confirmed current-device Rename path")
            && capability
                .notes
                .contains("separate confirmed homeserver opener")
            && capability
                .notes
                .contains("Dedicated external account page routes")
            && capability.notes.contains("password change")
            && capability.notes.contains("SSO change")
            && capability.notes.contains("read-only GetDevices directory")
            && capability
                .notes
                .contains("all-device management beyond the read-only directory")
            && capability.notes.contains("session-management lookup")
            && capability.notes.contains("cross-session revoke")
            && capability.notes.contains("device delete/trust changes")
            && capability.notes.contains("Matrix account/profile mutation")
            && capability.notes.contains("MatrixRequest::RenameDevice")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("unconfirmed live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_management_session_actions_row_is_visible_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_MARKER,
        "hepta_telegram_account_management_session_actions_row_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_management_session_actions_row")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_EVIDENCE
            .contains("Revoke, Rename, Trust, and Browser")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_EVIDENCE
            .contains("MatrixRequest::RenameDevice")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_EVIDENCE
            .contains("client.rename_device")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_EVIDENCE
            .contains("only the accept branch hands the active Matrix homeserver URL")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_EVIDENCE
            .contains("robius_open")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_EVIDENCE
            .contains("cross-session revoke")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_EVIDENCE
            .contains("device delete/trust")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_LABEL
            .contains("Browser confirms")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management session actions row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("account_management_session_actions_row_label")
            && capability
                .notes
                .contains("Revoke, Rename, Trust, and Browser")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("MatrixRequest::RenameDevice")
            && capability.notes.contains("client.rename_device")
            && capability.notes.contains("robius_open")
            && capability.notes.contains("AccountManagementPreviewState")
            && capability.notes.contains("GetOwnDevice")
            && capability.notes.contains("all-device list lookup")
            && capability.notes.contains("session-management lookup")
            && capability.notes.contains("cross-session revoke")
            && capability.notes.contains("device delete/trust change")
            && capability.notes.contains("unconfirmed live mutation")
            && capability.notes.contains("Matrix account/profile mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_management_device_directory_controls_row_is_visible_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_MARKER,
        "hepta_telegram_account_management_device_directory_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_MARKER,
        "hepta_telegram_account_management_device_directory_controls_row_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_device_directory_controls_row")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_EVIDENCE
            .contains("All devices, Password, SSO, Portal, and Activity")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_EVIDENCE
            .contains("only the accept branch hands the active Matrix homeserver URL")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_EVIDENCE
            .contains("robius_open")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_EVIDENCE
            .contains("all-device list")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_EVIDENCE
            .contains("password")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_DEVICE_DIRECTORY_CONTROLS_ROW_LABEL
            .contains("All devices")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management device directory controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("account_management_device_directory_controls_row_label")
            && capability
                .notes
                .contains("All devices, Password, SSO, Portal, and Activity")
            && capability.notes.contains("AccountManagementPreviewState")
            && capability.notes.contains("GetOwnDevice")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("robius_open")
            && capability.notes.contains("MatrixRequest::GetDevices")
            && capability.notes.contains("OwnDevicesFetched summaries")
            && capability.notes.contains("read-only")
            && capability.notes.contains("password change")
            && capability.notes.contains("SSO start")
            && capability.notes.contains("session revoke")
            && capability
                .notes
                .contains("device trust/rename/delete change")
            && capability.notes.contains("Matrix account/profile mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "account_management"
            && runway
                .current_path
                .contains("device-directory/preflight controls")
            && runway
                .current_path
                .contains("live GetDevices read-only all-device directory wiring")
    }));
}

#[test]
fn hepta_telegram_base_account_management_current_device_metadata_controls_row_is_visible_local_only()
 {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_MARKER,
        "hepta_telegram_account_management_current_device_metadata_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_MARKER,
        "hepta_telegram_account_management_current_device_metadata_controls_row_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_current_device_metadata_controls_row")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
            .contains("Device, Verified, Display, Session, and Source")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
            .contains("Source copies")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
            .contains("Verified copies")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
            .contains("Display copies")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
            .contains("Session copies")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
            .contains("visible local buttons")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
            .contains("does not request extra GetOwnDevice")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
            .contains("all-device list")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
            .contains("session-management lookup")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_LABEL
            .contains("Device")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management current device metadata controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("account_management_device_metadata_controls")
            && capability
                .base_module
                .contains("copy_account_management_current_device_id")
            && capability
                .base_module
                .contains("copy_account_management_current_device_verification")
            && capability
                .base_module
                .contains("copy_account_management_current_device_display_name")
            && capability
                .base_module
                .contains("copy_account_management_current_session")
            && capability
                .base_module
                .contains("copy_account_management_current_device_source_metadata")
            && capability
                .base_module
                .contains("stage_account_management_current_device_metadata_control")
            && capability
                .notes
                .contains("Device, Verified, Display, Session, and Source")
            && capability.notes.contains("local clipboard")
            && capability.notes.contains("Verified copies")
            && capability.notes.contains("Display copies")
            && capability.notes.contains("Session copies")
            && capability.notes.contains("Source copies")
            && capability
                .notes
                .contains("current-device metadata controls")
            && capability.notes.contains("GetOwnDevice")
            && capability.notes.contains("requests no extra GetOwnDevice")
            && capability.notes.contains("all-device list")
            && capability.notes.contains("session-management lookup")
            && capability.notes.contains("session revoke")
            && capability
                .notes
                .contains("device trust/rename/delete change")
            && capability.notes.contains("Matrix account/profile mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "account_management"
            && runway.current_path.contains(
                "current-device id/verified/display/session/source clipboard metadata controls",
            )
            && runway.remaining_gap.contains("session revoke")
    }));
}

#[test]
fn hepta_telegram_base_account_management_current_device_verification_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_MARKER,
        "hepta_telegram_account_management_current_device_verification_clipboard_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_current_device_verification_clipboard_action")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_EVIDENCE
            .contains("current-device verification status")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_EVIDENCE
            .contains("local Matrix verification state")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_EVIDENCE
            .contains("existing GetOwnDevice current device ID")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_EVIDENCE
            .contains("no extra GetOwnDevice")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_EVIDENCE
            .contains("all-device list")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_EVIDENCE
            .contains("session-management lookup")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_VERIFICATION_CLIPBOARD_LABEL
            .contains("Verified copies")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management current device verification clipboard"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("copy_account_management_current_device_verification")
            && capability
                .base_module
                .contains("account_management_current_device_verification_clipboard_label")
            && capability
                .notes
                .contains("current-device verification status")
            && capability.notes.contains("local Matrix verification state")
            && capability.notes.contains("GetOwnDevice current device ID")
            && capability.notes.contains("local clipboard")
            && capability.notes.contains("no extra GetOwnDevice")
            && capability.notes.contains("all-device list")
            && capability.notes.contains("session-management lookup")
            && capability.notes.contains("session revoke")
            && capability
                .notes
                .contains("device trust/rename/delete change")
            && capability.notes.contains("Matrix account/profile mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "account_management"
            && runway.current_path.contains(
                "current-device id/verified/display/session/source clipboard metadata controls",
            )
            && runway
                .current_path
                .contains("live GetDevices read-only all-device directory wiring")
    }));
}

#[test]
fn hepta_telegram_base_account_management_current_device_id_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_MARKER,
        "hepta_telegram_account_management_current_device_id_clipboard_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_current_device_id_clipboard_action")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_EVIDENCE
            .contains("current Matrix Device ID")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_EVIDENCE
            .contains("existing GetOwnDevice result")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_EVIDENCE
            .contains("no extra GetOwnDevice")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_EVIDENCE
            .contains("all-device list")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_EVIDENCE
            .contains("session-management lookup")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_LABEL
            .contains("clipboard only")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management current device id clipboard"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("copy_account_management_current_device_id")
            && capability
                .base_module
                .contains("account_management_current_device_id_clipboard_label")
            && capability.notes.contains("current Matrix Device ID")
            && capability.notes.contains("GetOwnDevice result")
            && capability.notes.contains("local clipboard")
            && capability.notes.contains("no extra GetOwnDevice")
            && capability.notes.contains("all-device list")
            && capability.notes.contains("session-management lookup")
            && capability.notes.contains("session revoke")
            && capability
                .notes
                .contains("device trust/rename/delete change")
            && capability.notes.contains("Matrix account/profile mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "account_management"
            && runway.current_path.contains(
                "current-device id/verified/display/session/source clipboard metadata controls",
            )
            && runway
                .current_path
                .contains("live GetDevices read-only all-device directory wiring")
    }));
}

#[test]
fn hepta_telegram_base_account_management_current_device_display_name_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_MARKER,
        "hepta_telegram_account_management_current_device_display_name_clipboard_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_current_device_display_name_clipboard_action")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_EVIDENCE
            .contains("current device display name")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_EVIDENCE
            .contains("existing GetOwnDevice result")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_EVIDENCE
            .contains("no extra GetOwnDevice")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_EVIDENCE
            .contains("all-device list")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_EVIDENCE
            .contains("session-management lookup")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_DISPLAY_NAME_CLIPBOARD_LABEL
            .contains("clipboard only")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management current device display name clipboard"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("copy_account_management_current_device_display_name")
            && capability
                .base_module
                .contains("account_management_current_device_display_name_clipboard_label")
            && capability.notes.contains("current device display name")
            && capability.notes.contains("GetOwnDevice result")
            && capability.notes.contains("local clipboard")
            && capability.notes.contains("no extra GetOwnDevice")
            && capability.notes.contains("all-device list")
            && capability.notes.contains("session-management lookup")
            && capability.notes.contains("session revoke")
            && capability
                .notes
                .contains("device trust/rename/delete change")
            && capability.notes.contains("Matrix account/profile mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "account_management"
            && runway.current_path.contains(
                "current-device id/verified/display/session/source clipboard metadata controls",
            )
            && runway.remaining_gap.contains("session revoke")
    }));
}

#[test]
fn hepta_telegram_base_account_management_current_session_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_MARKER,
        "hepta_telegram_account_management_current_session_clipboard_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_current_session_clipboard_action")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_EVIDENCE
            .contains("current-session summary")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_EVIDENCE
            .contains("existing GetOwnDevice result")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_EVIDENCE
            .contains("no extra GetOwnDevice")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_EVIDENCE
            .contains("all-device list")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_EVIDENCE
            .contains("session-management lookup")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_LABEL
            .contains("clipboard only")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management current session clipboard"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("copy_account_management_current_session")
            && capability
                .base_module
                .contains("account_management_current_session_clipboard_label")
            && capability.notes.contains("current-session summary")
            && capability.notes.contains("GetOwnDevice result")
            && capability.notes.contains("local clipboard")
            && capability.notes.contains("no extra GetOwnDevice")
            && capability.notes.contains("all-device list")
            && capability.notes.contains("session-management lookup")
            && capability.notes.contains("session revoke")
            && capability
                .notes
                .contains("device trust/rename/delete change")
            && capability.notes.contains("Matrix account/profile mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "account_management"
            && runway.current_path.contains(
                "current-device id/verified/display/session/source clipboard metadata controls",
            )
            && runway.remaining_gap.contains("session revoke")
    }));
}

#[test]
fn hepta_telegram_base_account_management_current_device_source_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_MARKER,
        "hepta_telegram_account_management_current_device_source_clipboard_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_current_device_source_clipboard_action")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_EVIDENCE
            .contains("own_profile")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_EVIDENCE
            .contains("existing GetOwnDevice text")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_EVIDENCE
            .contains("no extra GetOwnDevice")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_EVIDENCE
            .contains("all-device list")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_EVIDENCE
            .contains("session-management lookup")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_LABEL
            .contains("clipboard only")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management current device source clipboard"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("copy_account_management_current_device_source_metadata")
            && capability
                .base_module
                .contains("account_management_current_device_source_clipboard_label")
            && capability.notes.contains("own_profile")
            && capability.notes.contains("GetOwnDevice text")
            && capability.notes.contains("local clipboard")
            && capability.notes.contains("no extra GetOwnDevice")
            && capability.notes.contains("all-device list")
            && capability.notes.contains("session-management lookup")
            && capability.notes.contains("session revoke")
            && capability
                .notes
                .contains("device trust/rename/delete change")
            && capability.notes.contains("Matrix account/profile mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "account_management"
            && runway.current_path.contains(
                "current-device id/verified/display/session/source clipboard metadata controls",
            )
            && runway.remaining_gap.contains("session revoke")
    }));
}

#[test]
fn hepta_telegram_base_account_management_preflight_detail_controls_row_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_account_management_preflight_detail_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_MARKER,
        "hepta_telegram_account_management_preflight_detail_controls_row_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_preflight_detail_controls_row")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
            .contains("Request, Result, Error, Retry, Source, Packet, Contract, and Taxonomy")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
            .contains("local account/session request snapshot")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
            .contains("typed dedicated account portal")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
            .contains("Taxonomy records blocked")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
            .contains("Browser/Portal homeserver opener")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
            .contains("does not request GetOwnDevice")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
            .contains("MatrixRequest::GetDevices")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
            .contains("session-management lookup")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
            .contains("automatic retry")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_LABEL
            .contains("Request")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management preflight detail controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("account_management_preflight_controls")
            && capability
                .base_module
                .contains("account_management_request_snapshot_label")
            && capability
                .notes
                .contains("Request, Result, Error, Retry, Source, Packet, Contract, and Taxonomy")
            && capability
                .notes
                .contains("session/device drilldown acceptance matrix")
            && capability.notes.contains("typed dedicated account portal")
            && capability
                .notes
                .contains("Browser/Portal homeserver opener")
            && capability
                .notes
                .contains("current-device RenameDevice result")
            && capability.notes.contains("device delete/trust")
            && capability.notes.contains("result/error contracts")
            && capability.notes.contains("Taxonomy records blocked")
            && capability
                .notes
                .contains("local account/session request snapshot")
            && capability.notes.contains("AccountManagementPreviewState")
            && capability.notes.contains("GetOwnDevice")
            && capability.notes.contains("requests no extra GetOwnDevice")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("MatrixRequest::GetDevices")
            && capability.notes.contains("session-management lookup")
            && capability.notes.contains("automatic retry")
            && capability.notes.contains("session revoke")
            && capability.notes.contains("Matrix account/profile mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "account_management"
            && runway.current_path.contains("preflight controls")
            && runway
                .current_path
                .contains("local account/session request snapshot")
            && runway.remaining_gap.contains("session revoke")
    }));
}

#[test]
fn hepta_telegram_base_account_management_session_device_drilldown_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_MARKER,
        "hepta_telegram_account_management_session_device_drilldown_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_session_device_drilldown_packet_action")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_EVIDENCE
            .contains("loaded own_profile identity")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_EVIDENCE
            .contains("current GetOwnDevice session/device metadata")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_EVIDENCE
            .contains("device id/display/session/source clipboard payloads")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_EVIDENCE
            .contains("dedicated account portal route targets")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_EVIDENCE
            .contains("Browser/Portal homeserver opener outcome")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_EVIDENCE
            .contains("all-device directory scope")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_EVIDENCE
            .contains("password/SSO scope")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_EVIDENCE
            .contains("current-device RenameDevice request/result/error/retry/source slots")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_EVIDENCE
            .contains("cross-session revoke/trust scope")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_EVIDENCE
            .contains("device delete/trust scope")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_EVIDENCE
            .contains("submits no extra GetOwnDevice")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_EVIDENCE
            .contains("touches no gateway/runtime/auth or live mutation")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_DRILLDOWN_PACKET_LABEL
            .contains("Session/device drilldown packet")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management session device drilldown packet"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("account_preview_preflight_packet_button")
            && capability
                .base_module
                .contains("account_management_session_device_drilldown_packet_label")
            && capability
                .notes
                .contains("session/device drilldown acceptance matrix")
            && capability.notes.contains("own_profile")
            && capability.notes.contains("GetOwnDevice")
            && capability.notes.contains("verification state")
            && capability
                .notes
                .contains("device id/display/session/source clipboard")
            && capability
                .notes
                .contains("dedicated account portal route targets")
            && capability
                .notes
                .contains("Browser/Portal homeserver opener outcome")
            && capability.notes.contains("all-device directory")
            && capability.notes.contains("password/SSO")
            && capability.notes.contains("current-device RenameDevice")
            && capability.notes.contains("cross-session revoke/trust")
            && capability.notes.contains("device delete/trust")
            && capability.notes.contains("account/profile mutation guard")
            && capability
                .notes
                .contains("account_management remains a base gap")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "account_management"
            && runway
                .current_path
                .contains("session/device drilldown packet")
            && runway
                .remaining_gap
                .contains("dedicated account-management portal route")
            && runway
                .current_path
                .contains("live GetDevices read-only all-device directory wiring")
            && runway.remaining_gap.contains("password/SSO")
            && runway.remaining_gap.contains("session revoke/trust")
            && runway
                .remaining_gap
                .contains("cross-session device management")
            && runway
                .remaining_gap
                .contains("device delete/trust mutation")
            && runway
                .current_path
                .contains("typed account-session contract packet")
            && runway
                .next_ui_safe_step
                .contains("coordinate backend dedicated account portal")
            && runway
                .next_ui_safe_step
                .contains("account/profile result contracts")
    }));
}

#[test]
fn hepta_telegram_base_account_management_session_device_typed_contract_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_MARKER,
        "hepta_telegram_account_management_session_device_typed_contract_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_session_device_typed_contract_packet_action")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("visible Contract control")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("typed dedicated account portal route")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("Browser/Portal homeserver opener outcome")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("all-device directory")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("password/SSO")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("current-device RenameDevice")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("cross-session revoke/trust")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("device delete/trust")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("account/profile mutation guard")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("submits no extra GetOwnDevice")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("touches no gateway/runtime/auth or live mutation")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_TYPED_CONTRACT_PACKET_LABEL
            .contains("Session/device typed contract packet")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management session device typed contract packet"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("account_preview_preflight_contract_button")
            && capability
                .base_module
                .contains("account_management_session_device_typed_contract_packet_label")
            && capability
                .notes
                .contains("typed dedicated account portal route")
            && capability
                .notes
                .contains("Browser/Portal homeserver opener outcome")
            && capability.notes.contains("all-device directory")
            && capability.notes.contains("password/SSO")
            && capability.notes.contains("current-device RenameDevice")
            && capability.notes.contains("cross-session revoke/trust")
            && capability.notes.contains("device delete/trust")
            && capability.notes.contains("account/profile mutation guard")
            && capability.notes.contains("source-hash")
            && capability.notes.contains("idempotency")
            && capability.notes.contains("stale-session")
            && capability.notes.contains("promotion-blocker contracts")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_management_session_device_result_taxonomy_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_account_management_session_device_result_taxonomy_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_management_session_device_result_taxonomy_packet_action")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("visible Taxonomy control")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("GetOwnDevice")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("GetDevices")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("SetDisplayName")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("current-device RenameDevice")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("dedicated account portal routes")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("password/SSO actions")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("cross-session revoke/trust")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("device delete/trust")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("operation_id slots as not_assigned")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("stale-session")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("audit redaction")
    );
    assert!(
        crate::settings::account_settings::ACCOUNT_MANAGEMENT_SESSION_DEVICE_RESULT_TAXONOMY_PACKET_LABEL
            .contains("Session/device result taxonomy packet")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management session device result taxonomy packet"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("account_preview_preflight_taxonomy_button")
            && capability
                .base_module
                .contains("account_management_session_device_result_taxonomy_packet_label")
            && capability.notes.contains("GetOwnDevice")
            && capability.notes.contains("GetDevices")
            && capability.notes.contains("SetDisplayName")
            && capability.notes.contains("current-device RenameDevice")
            && capability.notes.contains("dedicated account portal routes")
            && capability.notes.contains("password/SSO actions")
            && capability.notes.contains("cross-session revoke/trust")
            && capability.notes.contains("device delete/trust")
            && capability.notes.contains("operation_id not_assigned")
            && capability.notes.contains("not_wired")
            && capability.notes.contains("confirmation-gated retry")
            && capability.notes.contains("audit redaction")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "account_management"
            && runway
                .current_path
                .contains("password/SSO/revoke/trust/delete result taxonomy packet")
            && runway.remaining_gap.contains("session revoke/trust")
            && runway
                .remaining_gap
                .contains("device delete/trust mutation")
    }));
}

#[test]
fn hepta_telegram_base_account_local_surface_close_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_LOCAL_SURFACE_CLOSE_MARKER,
        "hepta_telegram_account_local_surface_close_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_local_surface_close_evidence"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_avatar_upload"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_management"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar upload"
            && capability.notes.contains("desktop image picker")
            && capability.notes.contains("selected-file metadata preview")
            && capability.notes.contains("picker cancel")
            && capability.notes.contains("MatrixRequest::UploadAvatar")
            && capability.notes.contains("browser handoff")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account management"
            && capability
                .notes
                .contains("Matrix GetOwnDevice current-session/device details")
            && capability
                .notes
                .contains("MatrixRequest::GetDevices read-only directory")
            && capability
                .notes
                .contains("confirmed Browser/Portal homeserver system-browser handoff")
            && capability
                .notes
                .contains("dedicated account-management portal routes")
            && capability.notes.contains("cross-session revoke")
    }));
}

#[test]
fn hepta_telegram_base_account_logout_confirmation_is_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_LOGOUT_CONFIRMATION_MARKER,
        "hepta_telegram_account_logout_confirmation_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_logout_confirmation_guard"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_logout"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account logout confirmation"
            && capability
                .notes
                .contains("before the existing Matrix Logout path")
            && capability
                .notes
                .contains("only from the confirmed LogoutConfirmModal handler")
            && capability.notes.contains("open, Cancel, dismiss, reset")
            && capability.notes.contains("no extra logout")
            && capability.notes.contains("account/profile")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_real_shell_widgets_are_skinned() {
    let skinned_standard_shell = [
        "MainDesktopUI",
        "HomeScreen",
        "RoomsSideBar",
        "RoomsListHeader",
        "RoomFilterInputBar",
        "RoomsList",
        "RoomScreen",
        "RoomInputBar",
        "NavigationTabBar",
    ];

    assert_eq!(
        HEPTA_TELEGRAM_REAL_CHROME_MARKER,
        "hepta_telegram_real_chrome_on_standard_shell"
    );
    assert_eq!(HEPTA_TELEGRAM_SHELL_MARKER, "hepta_telegram_shell_ready");
    assert_eq!(
        HEPTA_TELEGRAM_BOTTOM_ANCHORED_MARKER,
        "hepta_telegram_bottom_anchored_ready"
    );
    assert!(skinned_standard_shell.contains(&"RoomsList"));
    assert!(skinned_standard_shell.contains(&"RoomScreen"));
    assert!(skinned_standard_shell.contains(&"RoomInputBar"));
    assert!(skinned_standard_shell.contains(&"RoomFilterInputBar"));
    assert!(skinned_standard_shell.len() >= 9);
}

#[test]
fn hepta_telegram_base_action_chrome_keeps_real_menu_actions() {
    let real_action_surfaces = [
        "NewMessageContextMenu",
        "RoomContextMenu",
        "ReplyingPreview",
        "EditingPane",
        "TypingNotice",
        "JumpToBottomButton",
    ];

    assert_eq!(
        HEPTA_TELEGRAM_ACTION_CHROME_MARKER,
        "hepta_telegram_action_chrome_on_real_menus"
    );
    assert!(real_action_surfaces.contains(&"NewMessageContextMenu"));
    assert!(real_action_surfaces.contains(&"RoomContextMenu"));
    assert!(real_action_surfaces.contains(&"EditingPane"));
    assert!(real_action_surfaces.len() >= 6);
}

#[test]
fn hepta_telegram_base_dialog_filter_is_on_real_sidebar() {
    let dialog_filter_chain = [
        "RoomsSideBar",
        "RoomFilterInputBar",
        "MainFilterAction",
        "RoomsList",
        "RoomDisplayFilterBuilder",
    ];

    assert_eq!(
        HEPTA_TELEGRAM_DIALOG_FILTER_MARKER,
        "hepta_telegram_dialog_filter_on_real_sidebar"
    );
    assert!(dialog_filter_chain.contains(&"RoomsSideBar"));
    assert!(dialog_filter_chain.contains(&"RoomFilterInputBar"));
    assert!(dialog_filter_chain.contains(&"MainFilterAction"));
    assert!(dialog_filter_chain.contains(&"RoomsList"));
    assert!(dialog_filter_chain.len() >= 5);
}

#[test]
fn hepta_telegram_base_dialog_state_filters_reuse_room_fields() {
    let filter_tokens = [
        "is:direct",
        "is:unread",
        "is:mention",
        "is:favorite",
        "is:low_priority",
    ];

    assert_eq!(
        HEPTA_TELEGRAM_DIALOG_STATE_FILTER_MARKER,
        "hepta_telegram_dialog_state_filters_on_real_room_filter"
    );
    assert!(filter_tokens.contains(&"is:direct"));
    assert!(filter_tokens.contains(&"is:unread"));
    assert!(filter_tokens.contains(&"is:mention"));
    assert!(filter_tokens.contains(&"is:favorite"));
    assert!(filter_tokens.contains(&"is:low_priority"));
}

#[test]
fn hepta_telegram_base_dialog_filter_presets_emit_main_filter_action() {
    let preset_tokens = ["", "is:unread", "is:direct", "is:favorite"];

    assert_eq!(
        HEPTA_TELEGRAM_DIALOG_FILTER_PRESET_MARKER,
        "hepta_telegram_dialog_filter_presets_emit_main_filter_action"
    );
    assert_eq!(preset_tokens[0], "");
    assert!(preset_tokens.contains(&"is:unread"));
    assert!(preset_tokens.contains(&"is:direct"));
    assert!(preset_tokens.contains(&"is:favorite"));
}

#[test]
fn hepta_telegram_base_dialog_list_empty_state_filter_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_DIALOG_LIST_EMPTY_STATE_LOCAL_FILTER_MARKER,
        "hepta_telegram_dialog_list_empty_state_local_filter_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"dialog_list_empty_state_local_filter_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"dialog_list_empty_state_local_filter"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "dialog list empty/filter local state"
            && capability.base_module == "RoomsList + RoomDisplayFilter + RoomListService"
            && capability.notes.contains("RoomListService/SlidingSync")
            && capability.notes.contains("RoomDisplayFilter matches")
            && capability.notes.contains("cached SpaceService children")
            && capability
                .notes
                .contains("do not send Matrix search queries")
            && capability.notes.contains("JoinRoom")
            && capability.notes.contains("LeaveRoom")
            && capability.notes.contains("membership mutation")
            && capability.notes.contains("room-state mutation")
    }));
}

#[test]
fn hepta_telegram_base_rooms_list_membership_edge_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOMS_LIST_MEMBERSHIP_EDGE_LOCAL_MARKER,
        "hepta_telegram_rooms_list_membership_edge_local_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"rooms_list_membership_edge_local_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"rooms_list_membership_edge"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "rooms list membership edge local state"
            && capability.base_module == "SlidingSync + RoomsList"
            && capability.notes.contains("removes Banned rooms")
            && capability.notes.contains("skips Knocked and Left rooms")
            && capability.notes.contains("re-knock")
            && capability.notes.contains("cancel-prior-knock")
            && capability.notes.contains("JoinRoom")
            && capability.notes.contains("LeaveRoom")
            && capability.notes.contains("Knock")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership mutation")
    }));
}

#[test]
fn hepta_telegram_base_rooms_list_pagination_adapter_is_local_read_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOMS_LIST_PAGINATION_ADAPTER_LOCAL_MARKER,
        "hepta_telegram_rooms_list_pagination_adapter_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ROOMS_LIST_LOAD_MORE_PAGINATION_PACKET_MARKER,
        "hepta_telegram_rooms_list_load_more_pagination_packet_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_pagination_adapter_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_load_more_pagination_packet_preview")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"rooms_list_pagination_adapter"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "rooms list pagination adapter local state"
            && capability.base_module == "RoomsList + RoomListService + PaginateTimeline"
            && capability
                .notes
                .contains("entries_with_dynamic_adapters(usize::MAX)")
            && capability.notes.contains("no Load more rooms UI")
            && capability
                .notes
                .contains("Room-list Load More pagination packet")
            && capability.notes.contains("server_max_hint")
            && capability
                .notes
                .contains("load_more_button_slot not_rendered")
            && capability
                .notes
                .contains("explicit_cursor_slot not_exposed")
            && capability
                .notes
                .contains("latest_preview_pagination_source")
            && capability.notes.contains("no room-list pagination request")
            && capability
                .notes
                .contains("Matrix PaginateTimeline read path")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership mutation")
    }));
}

#[test]
fn hepta_telegram_base_rooms_list_header_space_scope_is_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOMS_LIST_HEADER_SPACE_SCOPE_LOCAL_MARKER,
        "hepta_telegram_rooms_list_header_space_scope_local_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_header_space_scope_local_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"rooms_list_header_space_scope"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "rooms list header selected-space local evidence"
            && capability.base_module == "RoomsListHeader + NavigationBarAction::TabSelected"
            && capability.notes.contains("TabSelected(SelectedTab::Space)")
            && capability.notes.contains("local header title")
            && capability.notes.contains("resets the title to Chats")
            && capability.notes.contains("local evidence label")
            && capability.notes.contains("SpaceService fetch")
            && capability.notes.contains("Matrix search")
            && capability.notes.contains("room-list pagination")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state mutation")
            && capability.notes.contains("membership request")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_desktop_dock_restore_lazy_is_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_DESKTOP_DOCK_RESTORE_LAZY_LOCAL_MARKER,
        "hepta_telegram_desktop_dock_restore_lazy_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_DESKTOP_SHELL_MARKER,
        "hepta_telegram_desktop_shell_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"desktop_dock_restore_lazy_local_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"desktop_dock_restore"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "desktop dock restore lazy tab local evidence"
            && capability.base_module == "MainDesktopUI + SavedDockState + RobrixDock"
            && capability.notes.contains("SavedDockState")
            && capability.notes.contains("initializes only visible")
            && capability.notes.contains("defers hidden tab content")
            && capability.notes.contains("tab press")
            && capability.notes.contains("drop")
            && capability.notes.contains("close")
            && capability.notes.contains("no Matrix search")
            && capability.notes.contains("room-list pagination")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state mutation")
            && capability.notes.contains("membership request")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_mobile_stack_navigation_is_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_MOBILE_STACK_NAVIGATION_LOCAL_MARKER,
        "hepta_telegram_mobile_stack_navigation_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MOBILE_SHELL_MARKER,
        "hepta_telegram_mobile_shell_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MOBILE_EVIDENCE_DENSITY_GUARD_MARKER,
        "hepta_telegram_mobile_evidence_density_guard_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mobile_stack_navigation_local_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"mobile_stack_navigation"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mobile stack navigation local evidence"
            && capability.base_module == "HomeScreen + StackNavigation + RoomScreen pool"
            && capability
                .notes
                .contains("16 dedicated RoomScreen-backed room views")
            && capability.notes.contains("mobile_room_nav_stack")
            && capability.notes.contains("StackNavigation pop")
            && capability.notes.contains("no Matrix search")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state mutation")
            && capability.notes.contains("membership request")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_navigation_spaces_toggle_is_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_NAVIGATION_SPACES_TOGGLE_LOCAL_MARKER,
        "hepta_telegram_navigation_spaces_toggle_local_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"navigation_spaces_toggle_local_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"navigation_spaces_toggle"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "navigation spaces toggle local evidence"
            && capability.base_module == "NavigationTabBar + SpacesBarWrapper"
            && capability.notes.contains("ToggleSpacesBarButton")
            && capability
                .notes
                .contains("local SpacesBarWrapper show/hide state")
            && capability.notes.contains("does not select a space")
            && capability
                .notes
                .contains("does not fetch SpaceService children")
            && capability.notes.contains("no Matrix search")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state mutation")
            && capability.notes.contains("membership request")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_profile_icon_settings_navigation_is_local_cache_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_PROFILE_ICON_SETTINGS_NAVIGATION_LOCAL_MARKER,
        "hepta_telegram_profile_icon_settings_navigation_local_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"profile_icon_settings_navigation_local_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"profile_icon_settings_navigation"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "ProfileIcon settings navigation local/cache evidence"
            && capability.base_module == "NavigationTabBar ProfileIcon + HomeScreen SettingsScreen"
            && capability.notes.contains("get_own_profile")
            && capability.notes.contains("user_profile_cache")
            && capability.notes.contains("avatar_cache")
            && capability.notes.contains("OpenSettings")
            && capability.notes.contains("local Settings tab")
            && capability
                .notes
                .contains("SettingsScreen from current AppState/cache")
            && capability.notes.contains("account mutation")
            && capability.notes.contains("profile mutation")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_navigation_top_level_tab_selection_is_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_NAVIGATION_TOP_LEVEL_TAB_SELECTION_LOCAL_MARKER,
        "hepta_telegram_navigation_top_level_tab_selection_local_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"navigation_top_level_tab_selection_local_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"navigation_top_level_tab_selection"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "navigation top-level tab selection local evidence"
            && capability.base_module == "NavigationTabBar + HomeScreen PageFlip"
            && capability.notes.contains("Home and Add Room buttons")
            && capability.notes.contains("GoToHome")
            && capability.notes.contains("GoToAddRoom")
            && capability.notes.contains("previous_selection")
            && capability.notes.contains("selected_tab")
            && capability.notes.contains("TabSelected")
            && capability.notes.contains("PageFlip")
            && capability.notes.contains("AddRoom Join/Knock")
            && capability.notes.contains("confirmation guards")
            && capability.notes.contains("Matrix search")
            && capability.notes.contains("room-list pagination")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_settings_close_previous_selection_is_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_SETTINGS_CLOSE_PREVIOUS_SELECTION_LOCAL_MARKER,
        "hepta_telegram_settings_close_previous_selection_local_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"settings_close_previous_selection_local_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"settings_close_previous_selection"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Settings close previous-selection local evidence"
            && capability.base_module == "SettingsScreen + HomeScreen CloseSettings"
            && capability.notes.contains("close button")
            && capability.notes.contains("Escape")
            && capability.notes.contains("back gesture")
            && capability.notes.contains("mouse back")
            && capability.notes.contains("CloseSettings")
            && capability.notes.contains("previous_selection")
            && capability.notes.contains("local UI state")
            && capability.notes.contains("logout")
            && capability.notes.contains("account mutation")
            && capability.notes.contains("profile mutation")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_spaces_bar_entry_selection_is_local_read_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_SPACES_BAR_ENTRY_SELECTION_LOCAL_MARKER,
        "hepta_telegram_spaces_bar_entry_selection_local_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"spaces_bar_entry_selection_local_read_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"spaces_bar_entry_selection"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "spaces bar entry selection local/read evidence"
            && capability.base_module == "SpacesBar + HomeScreen + RoomsList"
            && capability.notes.contains("SpacesBarAction::ButtonClicked")
            && capability.notes.contains("selected_space locally")
            && capability.notes.contains("NavigationBarAction::GoToSpace")
            && capability.notes.contains("TabSelected(SelectedTab::Space)")
            && capability
                .notes
                .contains("cached SpaceService child/subspace maps")
            && capability
                .notes
                .contains("does not directly fetch SpaceService children")
            && capability.notes.contains("no Matrix search")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state mutation")
            && capability.notes.contains("membership request")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_spaces_bar_secondary_click_is_local_no_menu_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_SPACES_BAR_SECONDARY_CLICK_LOCAL_MARKER,
        "hepta_telegram_spaces_bar_secondary_click_local_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"spaces_bar_secondary_click_local_no_menu_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"spaces_bar_secondary_click"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "spaces bar secondary-click local no-menu evidence"
            && capability.base_module == "SpacesBarEntry + SpacesBar"
            && capability.notes.contains("right-click / long-press")
            && capability
                .notes
                .contains("SpacesBarAction::ButtonSecondaryClicked")
            && capability.notes.contains("local no-op")
            && capability.notes.contains("no context menu")
            && capability.notes.contains("SpaceService fetch")
            && capability.notes.contains("Matrix search")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state mutation")
            && capability.notes.contains("membership request")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_spaces_bar_empty_filter_is_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_SPACES_BAR_EMPTY_FILTER_LOCAL_MARKER,
        "hepta_telegram_spaces_bar_empty_filter_local_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"spaces_bar_empty_filter_local_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"spaces_bar_empty_filter"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "spaces bar empty/filter local evidence"
            && capability.base_module == "SpacesBar + RoomDisplayFilter"
            && capability.notes.contains("local all_joined_spaces")
            && capability
                .notes
                .contains("RoomDisplayFilter keyword matching")
            && capability
                .notes
                .contains("update_displayed_spaces rebuilds displayed_spaces locally")
            && capability.notes.contains("no Matrix search")
            && capability.notes.contains("direct SpaceService child fetch")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state mutation")
            && capability.notes.contains("membership request")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_rooms_list_section_unread_aggregate_is_local_zero_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOMS_LIST_SECTION_UNREAD_AGGREGATE_LOCAL_ZERO_MARKER,
        "hepta_telegram_rooms_list_section_unread_aggregate_local_zero_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_section_unread_aggregate_local_zero_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"rooms_list_section_unread_aggregate"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "rooms list section unread aggregate local zero state"
            && capability.base_module == "RoomsList + CollapsibleHeader + loaded room state"
            && capability
                .notes
                .contains("People and Rooms section headers")
            && capability.notes.contains("local zero placeholders")
            && capability.notes.contains("running section aggregate")
            && capability
                .notes
                .contains("individual room rows still use loaded per-room unread state")
            && capability
                .notes
                .contains("People/Rooms unread/mention aggregate packet")
            && capability
                .notes
                .contains("header_badge_source local_zero_placeholder")
            && capability
                .notes
                .contains("aggregate_refresh_slot not_built")
            && capability
                .notes
                .contains("parent-chain attribution partial-cache-only")
            && capability.notes.contains("aggregate scan")
            && capability.notes.contains("read receipt")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership mutation")
    }));
}

#[test]
fn hepta_telegram_base_rooms_list_all_rooms_loaded_is_local_unknown_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOMS_LIST_ALL_ROOMS_LOADED_LOCAL_UNKNOWN_MARKER,
        "hepta_telegram_rooms_list_all_rooms_loaded_local_unknown_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_all_rooms_loaded_local_unknown_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"rooms_list_all_rooms_loaded"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "rooms list all rooms loaded local unknown state"
            && capability.base_module == "RoomsList + RestoreStatusView + RoomScreen + InviteScreen"
            && capability.notes.contains("all_rooms_loaded")
            && capability.notes.contains("local unknown/false")
            && capability.notes.contains("room-list completeness")
            && capability.notes.contains("RestoreStatusView")
            && capability.notes.contains("room-list pagination request")
            && capability.notes.contains("Matrix search")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership mutation")
    }));
}

#[test]
fn hepta_telegram_base_rooms_list_space_parent_cache_is_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOMS_LIST_SPACE_PARENT_CACHE_LOCAL_MARKER,
        "hepta_telegram_rooms_list_space_parent_cache_local_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_space_parent_cache_local_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"rooms_list_space_parent_cache"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "rooms list space parent cache local state"
            && capability.base_module == "RoomsList + SpaceService cached child maps"
            && capability
                .notes
                .contains("Selected-space filtering uses cached SpaceService")
            && capability
                .notes
                .contains("direct child room and subspace maps")
            && capability.notes.contains("JoinedRoomInfo")
            && capability.notes.contains("every parent chain")
            && capability.notes.contains("Matrix search")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership mutation")
            && capability.notes.contains("SpaceService read-sync path")
    }));
}

#[test]
fn hepta_telegram_base_rooms_list_name_update_selected_state_is_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOMS_LIST_NAME_UPDATE_SELECTED_STATE_LOCAL_MARKER,
        "hepta_telegram_rooms_list_name_update_selected_state_local_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_name_update_selected_state_local_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"rooms_list_name_update_selected_state"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "rooms list name update selected state local evidence"
            && capability.base_module == "RoomsList + SelectedRoom"
            && capability.notes.contains("UpdateRoomName")
            && capability.notes.contains("loaded joined/invited room rows")
            && capability.notes.contains("SelectedRoom broadcast")
            && capability.notes.contains("Dock tab")
            && capability.notes.contains("StackNav header")
            && capability.notes.contains("Matrix room-state mutation")
            && capability.notes.contains("message")
            && capability.notes.contains("membership request")
            && capability.notes.contains("live rename request")
    }));
}

#[test]
fn hepta_telegram_base_rooms_list_removed_room_selected_state_is_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_ROOMS_LIST_REMOVED_ROOM_SELECTED_STATE_LOCAL_MARKER,
        "hepta_telegram_rooms_list_removed_room_selected_state_local_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_removed_room_selected_state_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"rooms_list_removed_room_rejoin_packet_preview")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"rooms_list_removed_room_selected_state"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "rooms list removed room selected state local evidence"
            && capability
                .base_module
                .contains("SelectedRoom stale-focus clear")
            && capability.notes.contains("RemoveRoom")
            && capability.notes.contains("left, kicked, or banned")
            && capability.notes.contains("loaded local list")
            && capability
                .notes
                .contains("clears AppState selected-room focus")
            && capability
                .notes
                .contains("selected-room removed/rejoin packet")
            && capability.notes.contains("replacement_ui_slot not_wired")
            && capability.notes.contains("rejoin_request_slot not_built")
            && capability.notes.contains("JoinRoom")
            && capability.notes.contains("LeaveRoom")
            && capability.notes.contains("Knock")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state mutation")
            && capability.notes.contains("membership request")
    }));
}

#[test]
fn hepta_telegram_base_space_unread_filter_is_local_zero_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_SPACE_UNREAD_FILTER_LOCAL_ZERO_MARKER,
        "hepta_telegram_space_unread_filter_local_zero_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"space_unread_filter_local_zero_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"space_unread_filter_aggregate_packet_preview")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"space_unread_filter"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "space unread filter local zero state"
            && capability.base_module == "RoomDisplayFilter + RoomsList + JoinedSpaceInfo"
            && capability.notes.contains("JoinedSpaceInfo")
            && capability.notes.contains("local zero placeholders")
            && capability
                .notes
                .contains("space unread/mention aggregate packet")
            && capability.notes.contains("is:unread")
            && capability.notes.contains("is:mention")
            && capability.notes.contains("room-display-filter zero source")
            && capability
                .notes
                .contains("aggregate_refresh_slot not_built")
            && capability
                .notes
                .contains("do not fetch aggregate unread counts")
            && capability.notes.contains("read receipts")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership mutation")
    }));
}
