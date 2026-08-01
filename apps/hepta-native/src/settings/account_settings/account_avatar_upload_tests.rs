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
    assert!(ACCOUNT_AVATAR_UPLOAD_DECODE_PROBE_EVIDENCE.contains("already selected local image"));
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
    assert!(ACCOUNT_AVATAR_UPLOAD_LIFECYCLE_METADATA_EVIDENCE.contains("confirmed upload handoff"));
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
    assert!(ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_EVIDENCE.contains("cached local file path"));
    assert!(
        ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_EVIDENCE.contains("MatrixRequest::UploadAvatar")
    );
}

#[test]
fn avatar_direct_mxc_editor_validates_and_confirms_setavatar_some() {
    let avatar_url = parse_account_avatar_direct_mxc_uri("  mxc://example.org/avatar-media-id  ")
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
        ACCOUNT_AVATAR_DIRECT_SET_CONFIRMATION_EVIDENCE.contains("MatrixRequest::SetAvatar(Some")
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
    assert!(ACCOUNT_AVATAR_UPLOAD_REAL_PATH_EVIDENCE.contains("client.account().upload_avatar"));
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
        ACCOUNT_AVATAR_UPLOAD_CROP_EDITOR_BOUNDARY_EVIDENCE.contains("AvatarUploadPreviewState")
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
        ACCOUNT_AVATAR_UPLOAD_SOURCE_PREVIEW_CONTROLS_EVIDENCE.contains("AvatarUploadPreviewState")
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
    assert!(label.contains("audit_redaction raw_path_camera_buffer_thumbnail_transform_redacted"));
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
        ACCOUNT_AVATAR_UPLOAD_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("opens no file picker")
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
    assert!(ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE.contains("PositiveConfirmationModal"));
    assert!(
        ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE.contains("MatrixRequest::GetOwnDevice")
    );
    assert!(ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE.contains("Device display name"));
    assert!(ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE.contains("external account page"));
    assert!(ACCOUNT_MANAGEMENT_REFRESH_CONFIRMATION_EVIDENCE.contains("session-management lookup"));
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
    assert!(account_management_browser_portal_url_from_homeserver("matrix.example.org").is_err());
    assert!(
        account_management_browser_portal_url_from_homeserver("mxc://example.org/avatar").is_err()
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
        label.contains("current-device Rename has a separate confirmed Matrix rename_device path")
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
    assert!(ACCOUNT_MANAGEMENT_SESSION_REVOKE_BOUNDARY_EVIDENCE.contains("GetOwnDevice previews"));
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
    assert!(label.contains(
        "Rename has a separate confirmation-gated current-device MatrixRequest::RenameDevice path"
    ));
    assert!(label.contains("Revoke and Trust are visible local blocked controls"));
    assert!(label.contains("Browser uses a separate PositiveConfirmationModal homeserver opener"));
    assert!(label.contains("No all-device list"));
    assert!(label.contains("session-management lookup"));
    assert!(label.contains("cross-session revoke"));
    assert!(label.contains("device delete/trust change"));
    assert!(label.contains("Matrix account/profile mutation"));
    assert!(label.contains("gateway/runtime/auth"));
    assert!(label.contains("unconfirmed live mutation"));
    assert!(label.contains(ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_LABEL));
    assert!(ACCOUNT_MANAGEMENT_SESSION_ACTIONS_ROW_EVIDENCE.contains("account_management_preview"));
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
    assert!(label.contains("Portal uses a separate PositiveConfirmationModal homeserver opener"));
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
        ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE.contains("Display copies")
    );
    assert!(
        ACCOUNT_MANAGEMENT_CURRENT_DEVICE_METADATA_CONTROLS_ROW_EVIDENCE.contains("Session copies")
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
    let payload = account_management_current_device_id_clipboard_payload(Some("  ABCDEFG123  "));

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
    assert!(ACCOUNT_MANAGEMENT_CURRENT_DEVICE_ID_CLIPBOARD_EVIDENCE.contains("local clipboard"));
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
    let payload =
        account_management_current_device_display_name_clipboard_payload(Some("  Alice phone  "));

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
        ACCOUNT_MANAGEMENT_CURRENT_SESSION_CLIPBOARD_EVIDENCE.contains("current-session summary")
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
    assert!(ACCOUNT_MANAGEMENT_CURRENT_DEVICE_SOURCE_CLIPBOARD_EVIDENCE.contains("own_profile"));
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
    assert!(label.contains("Browser/Portal homeserver opener has a separate confirmation path"));
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
        ACCOUNT_MANAGEMENT_PREFLIGHT_DETAIL_CONTROLS_ROW_EVIDENCE.contains("gateway/runtime/auth")
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
    assert!(label.contains("current-device RenameDevice request/result/error/retry/source slots"));
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
    assert!(label.contains("password_result opened/completed/cancelled/failed/stale not_wired"));
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
