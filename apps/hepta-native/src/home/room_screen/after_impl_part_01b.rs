#[allow(dead_code)]
fn media_result_control_url(
    action: &str,
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
) -> String {
    media_result_control_url_with_source(
        action,
        action_label,
        filename,
        metadata,
        open_after_save,
        None,
    )
}

fn media_result_control_url_with_source(
    action: &str,
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
    source_mxc: Option<&str>,
) -> String {
    let mut serializer = url::form_urlencoded::Serializer::new(String::new());
    serializer
        .append_pair("action", action)
        .append_pair("label", action_label)
        .append_pair("name", filename)
        .append_pair("open", if open_after_save { "1" } else { "0" })
        .append_pair("kind", metadata.kind.as_str());
    if let Some(source_mxc) = source_mxc.filter(|value| !value.trim().is_empty()) {
        serializer.append_pair("mxc", source_mxc);
    }
    if let Some(mime_type) = metadata.mime_type.as_deref() {
        serializer.append_pair("mime", mime_type);
    }
    if let Some(size_label) = metadata.size_label.as_deref() {
        serializer.append_pair("size", size_label);
    }
    if let Some(duration_label) = metadata.duration_label.as_deref() {
        serializer.append_pair("duration", duration_label);
    }
    if let Some(dimensions_label) = metadata.dimensions_label.as_deref() {
        serializer.append_pair("dimensions", dimensions_label);
    }
    let query = serializer.finish();
    format!("{MEDIA_RESULT_CONTROL_URL_SCHEME}://media?{query}")
}

#[allow(dead_code)]
fn media_save_preflight_control_url(
    action: &str,
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
) -> String {
    media_save_preflight_control_url_with_source(
        action,
        action_label,
        filename,
        metadata,
        open_after_save,
        None,
    )
}

fn media_save_preflight_control_url_with_source(
    action: &str,
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
    source_mxc: Option<&str>,
) -> String {
    let mut serializer = url::form_urlencoded::Serializer::new(String::new());
    serializer
        .append_pair("action", action)
        .append_pair("label", action_label)
        .append_pair("name", filename)
        .append_pair("open", if open_after_save { "1" } else { "0" })
        .append_pair("kind", metadata.kind.as_str());
    if let Some(source_mxc) = source_mxc.filter(|value| !value.trim().is_empty()) {
        serializer.append_pair("mxc", source_mxc);
    }
    if let Some(mime_type) = metadata.mime_type.as_deref() {
        serializer.append_pair("mime", mime_type);
    }
    if let Some(size_label) = metadata.size_label.as_deref() {
        serializer.append_pair("size", size_label);
    }
    if let Some(duration_label) = metadata.duration_label.as_deref() {
        serializer.append_pair("duration", duration_label);
    }
    if let Some(dimensions_label) = metadata.dimensions_label.as_deref() {
        serializer.append_pair("dimensions", dimensions_label);
    }
    let query = serializer.finish();
    format!("{MEDIA_SAVE_PREFLIGHT_CONTROL_URL_SCHEME}://media?{query}")
}

fn media_codec_transcode_control_url(
    action: &str,
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
) -> String {
    let mut serializer = url::form_urlencoded::Serializer::new(String::new());
    serializer
        .append_pair("action", action)
        .append_pair("label", action_label)
        .append_pair("name", filename)
        .append_pair("open", if open_after_save { "1" } else { "0" })
        .append_pair("kind", metadata.kind.as_str());
    if let Some(mime_type) = metadata.mime_type.as_deref() {
        serializer.append_pair("mime", mime_type);
    }
    if let Some(size_label) = metadata.size_label.as_deref() {
        serializer.append_pair("size", size_label);
    }
    if let Some(duration_label) = metadata.duration_label.as_deref() {
        serializer.append_pair("duration", duration_label);
    }
    if let Some(dimensions_label) = metadata.dimensions_label.as_deref() {
        serializer.append_pair("dimensions", dimensions_label);
    }
    let query = serializer.finish();
    format!("{MEDIA_CODEC_TRANSCODE_CONTROL_URL_SCHEME}://media?{query}")
}

#[allow(dead_code)]
fn media_save_preflight_detail_controls_preview(
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
) -> String {
    media_save_preflight_detail_controls_preview_with_source(
        action_label,
        filename,
        metadata,
        open_after_save,
        None,
    )
}

fn media_save_preflight_detail_controls_preview_with_source(
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
    source_mxc: Option<&str>,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata.summary(filename);
    let controls = ["Request", "Result", "Error", "Retry", "Source"]
        .into_iter()
        .map(|action| {
            let url = media_save_preflight_control_url_with_source(
                action,
                action_label,
                filename,
                metadata,
                open_after_save,
                source_mxc,
            );
            format!(
                "<a href=\"{}\">{}</a>",
                htmlize::escape_attribute(&url),
                htmlize::escape_text(action),
            )
        })
        .collect::<Vec<_>>()
        .join(" · ");

    format!(
        "<br><i>SaveMedia preflight:</i> {controls}<br><i>{}. Loaded metadata: {} Retry confirms before SaveMedia when this row has a plain MXC; no unconfirmed FetchMedia, open-folder request, retry automation, queue control, decrypt retry, gateway/runtime/auth, or live mutation.</i>",
        MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_LABEL,
        htmlize::escape_text(&metadata_summary),
    )
}

fn media_codec_transcode_controls_preview(
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Play"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata.summary(filename);
    let controls = ["Codec", "Transcode", "Captions", "Quality", "Decrypt"]
        .into_iter()
        .map(|action| {
            let url = media_codec_transcode_control_url(
                action,
                action_label,
                filename,
                metadata,
                open_after_save,
            );
            format!(
                "<a href=\"{}\">{}</a>",
                htmlize::escape_attribute(&url),
                htmlize::escape_text(action),
            )
        })
        .collect::<Vec<_>>()
        .join(" · ");

    format!(
        "<br><i>Codec/transcode controls:</i> {controls}<br><i>{}. Loaded metadata: {} No FetchMedia, SaveMedia, decoder, transcoder, captions fetch, quality switch, decrypt, inline player, queue mutation, gateway/runtime/auth, or live mutation.</i>",
        MEDIA_CODEC_TRANSCODE_CONTROLS_LABEL,
        htmlize::escape_text(&metadata_summary),
    )
}

#[allow(dead_code)]
fn media_save_result_recovery_controls_preview(
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
) -> String {
    media_save_result_recovery_controls_preview_with_source(
        action_label,
        filename,
        metadata,
        open_after_save,
        None,
    )
}

fn media_save_result_recovery_controls_preview_with_source(
    action_label: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
    open_after_save: bool,
    source_mxc: Option<&str>,
) -> String {
    let action_label = action_label.trim();
    let action_label = if action_label.is_empty() {
        "Download"
    } else {
        action_label
    };
    let filename = filename.trim();
    let filename = if filename.is_empty() {
        "hepta-media-download"
    } else {
        filename
    };
    let metadata_summary = metadata.summary(filename);
    let controls = ["Open folder", "Replay", "Retry", "Queue", "Decrypt"]
        .into_iter()
        .map(|action| {
            let url = media_result_control_url_with_source(
                action,
                action_label,
                filename,
                metadata,
                open_after_save,
                source_mxc,
            );
            format!(
                "<a href=\"{}\">{}</a>",
                htmlize::escape_attribute(&url),
                htmlize::escape_text(action),
            )
        })
        .collect::<Vec<_>>()
        .join(" · ");

    format!(
        "<br><i>Save/Open recovery:</i> {controls}<br><i>{}. Loaded metadata: {} Open folder and Replay use a cached successful SaveMedia destination for this plain MXC when present; Retry confirms before SaveMedia when this row has a plain MXC; no unconfirmed FetchMedia, unconfirmed SaveMedia, queue control, decrypt retry, gateway/runtime/auth, or live mutation.</i>",
        MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_LABEL,
        htmlize::escape_text(&metadata_summary),
    )
}

fn image_info_download_metadata(image_info: &ImageInfo) -> MediaDownloadActionMetadata {
    let mut metadata = MediaDownloadActionMetadata::new("Image");
    metadata.mime_type = image_info
        .mimetype
        .as_ref()
        .filter(|mimetype| !mimetype.trim().is_empty())
        .map(ToString::to_string);
    metadata.size_label = image_info
        .size
        .map(|bytes| ByteSize::b(bytes.into()).to_string());
    metadata.dimensions_label = image_info
        .width
        .and_then(|width| image_info.height.map(|height| format!("{width}x{height}")));
    metadata
}

fn encrypted_image_local_metadata_preview(body: &str, image_info: &ImageInfo) -> String {
    let metadata = image_info_download_metadata(image_info);
    let blurhash_label = if image_info
        .blurhash
        .as_deref()
        .is_some_and(|blurhash| !blurhash.trim().is_empty())
    {
        "blurhash loaded"
    } else {
        "blurhash missing"
    };
    let thumbnail_label = if image_info.thumbnail_source.is_some() {
        "thumbnail source loaded"
    } else {
        "thumbnail source missing"
    };
    format!(
        "{body}\n\nLoaded encrypted image metadata: {}; {blurhash_label}; {thumbnail_label}. {MEDIA_ENCRYPTED_IMAGE_METADATA_LOCAL_LABEL} No decrypt, SaveMedia, FetchMedia, image decode, thumbnail fetch, media cache mutation, room-state, membership, gateway/runtime/auth, or live mutation.",
        metadata.summary(body),
    )
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
fn pick_media_save_path(suggested_filename: &str) -> MediaSavePathPickResult {
    rfd::FileDialog::new()
        .set_file_name(suggested_filename)
        .save_file()
        .map(MediaSavePathPickResult::Picked)
        .unwrap_or(MediaSavePathPickResult::Canceled)
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
fn pick_media_save_path(_suggested_filename: &str) -> MediaSavePathPickResult {
    MediaSavePathPickResult::Unsupported
}

fn media_download_url(
    source: &MediaSource,
    filename: &str,
    open_after_save: bool,
    metadata: &MediaDownloadActionMetadata,
) -> Option<String> {
    let MediaSource::Plain(mxc_uri) = source else {
        return None;
    };
    let mut serializer = url::form_urlencoded::Serializer::new(String::new());
    serializer
        .append_pair("mxc", mxc_uri.as_str())
        .append_pair("name", filename)
        .append_pair("open", if open_after_save { "1" } else { "0" })
        .append_pair("kind", metadata.kind.as_str());
    if let Some(mime_type) = metadata.mime_type.as_deref() {
        serializer.append_pair("mime", mime_type);
    }
    if let Some(size_label) = metadata.size_label.as_deref() {
        serializer.append_pair("size", size_label);
    }
    if let Some(duration_label) = metadata.duration_label.as_deref() {
        serializer.append_pair("duration", duration_label);
    }
    if let Some(dimensions_label) = metadata.dimensions_label.as_deref() {
        serializer.append_pair("dimensions", dimensions_label);
    }
    let query = serializer.finish();
    Some(format!("{MEDIA_DOWNLOAD_URL_SCHEME}://media?{query}"))
}

fn encrypted_media_local_metadata_preview(
    kind: &str,
    filename: &str,
    metadata: &MediaDownloadActionMetadata,
) -> String {
    let summary_text = metadata.summary(filename);
    let summary = htmlize::escape_text(&summary_text);
    let kind = htmlize::escape_text(kind);
    let copy_link = media_metadata_clipboard_link(filename, metadata);
    let packet_link = media_operation_packet_link("Encrypted preview", filename, metadata, false);
    let contract_link =
        media_playback_queue_contract_link("Encrypted preview", filename, metadata, false);
    let taxonomy_link =
        media_playback_result_taxonomy_link("Encrypted preview", filename, metadata, false);
    format!(
        "<br>{copy_link} · {packet_link} · {contract_link} · {taxonomy_link}<br><i>Encrypted {kind} metadata: {summary}. {MEDIA_ENCRYPTED_METADATA_LOCAL_LABEL} No decrypt, SaveMedia, FetchMedia, codec/transcode, inline playback, retry/cancel, room-state, membership, gateway/runtime/auth, or live mutation.</i>"
    )
}

/// Draws a file message's content into the given `message_content_widget`.
///
/// Returns whether the file message content was fully drawn.
fn populate_file_message_content(
    cx: &mut Cx,
    message_content_widget: &HtmlOrPlaintextRef,
    file_content: &FileMessageEventContent,
) -> bool {
    // Display the file name, human-readable size, caption, and a button to download it.
    let filename = htmlize::escape_text(file_content.filename());
    let metadata_size = file_content
        .info
        .as_ref()
        .and_then(|info| info.size)
        .map(|bytes| ByteSize::b(bytes.into()).to_string());
    let size = metadata_size
        .as_ref()
        .map(|size| format!("  ({size})"))
        .unwrap_or_default();
    let metadata_mime = file_content
        .info
        .as_ref()
        .and_then(|info| info.mimetype.as_deref())
        .map(str::to_string);
    let mut metadata = MediaDownloadActionMetadata::new("File");
    metadata.mime_type = metadata_mime;
    metadata.size_label = metadata_size;
    let caption = file_content
        .formatted_caption()
        .filter(|fb| fb.format == MessageFormat::Html)
        .map(|fb| format!("<br><i>{}</i>", fb.body))
        .or_else(|| {
            file_content
                .caption()
                .map(|c| format!("<br><i>{}</i>", htmlize::escape_text(c)))
        })
        .unwrap_or_default();
    let actions = media_download_url(
        &file_content.source,
        file_content.filename(),
        false,
        &metadata,
    )
    .map(|url| {
        let source_mxc = match &file_content.source {
            MediaSource::Plain(mxc_uri) => Some(mxc_uri.as_str()),
            MediaSource::Encrypted(_) => None,
        };
        let metadata_link = media_metadata_clipboard_link(file_content.filename(), &metadata);
        let packet_link =
            media_operation_packet_link("Download", file_content.filename(), &metadata, false);
        let contract_link = media_playback_queue_contract_link(
            "Download",
            file_content.filename(),
            &metadata,
            false,
        );
        let taxonomy_link = media_playback_result_taxonomy_link(
            "Download",
            file_content.filename(),
            &metadata,
            false,
        );
        let preflight_controls = media_save_preflight_detail_controls_preview_with_source(
            "Download",
            file_content.filename(),
            &metadata,
            false,
            source_mxc,
        );
        let recovery_controls = media_save_result_recovery_controls_preview_with_source(
            "Download",
            file_content.filename(),
            &metadata,
            false,
            source_mxc,
        );
        format!(
            "<br>{metadata_link} · {packet_link} · {contract_link} · {taxonomy_link} · <a href=\"{}\">Download</a>{preflight_controls}{recovery_controls}",
            htmlize::escape_attribute(&url),
        )
    })
    .unwrap_or_else(|| {
        encrypted_media_local_metadata_preview("file", file_content.filename(), &metadata)
    });

    message_content_widget.show_html(
        cx,
        format!(
            "<b>{filename}</b>{size}{caption}{actions}<br>\
            <i>{MEDIA_FILE_COMPACT_LABEL}</i>"
        ),
    );
    true
}

/// Draws an audio message's content into the given `message_content_widget`.
///
/// Returns whether the audio message content was fully drawn.
fn populate_audio_message_content(
    cx: &mut Cx,
    message_content_widget: &HtmlOrPlaintextRef,
    audio: &AudioMessageEventContent,
) -> bool {
    // Display the file name, human-readable size, caption, and a button to download it.
    let filename = htmlize::escape_text(audio.filename());
    let (metadata_duration, metadata_mime, metadata_size) = audio
        .info
        .as_ref()
        .map(|info| {
            (
                info.duration.map(|d| format!("{:.2} sec", d.as_secs_f64())),
                info.mimetype
                    .as_ref()
                    .filter(|m| !m.trim().is_empty())
                    .map(ToString::to_string),
                info.size.map(|bytes| ByteSize::b(bytes.into()).to_string()),
            )
        })
        .unwrap_or_default();
    let duration = metadata_duration
        .as_ref()
        .map(|duration| format!("  {duration},"))
        .unwrap_or_default();
    let mime = metadata_mime
        .as_ref()
        .map(|mime| format!("  {},", htmlize::escape_text(mime)))
        .unwrap_or_default();
    let size = metadata_size
        .as_ref()
        .map(|size| format!("  ({size}),"))
        .unwrap_or_default();
    let mut metadata = MediaDownloadActionMetadata::new("Audio");
    metadata.mime_type = metadata_mime;
    metadata.size_label = metadata_size;
    metadata.duration_label = metadata_duration;
    let caption = audio
        .formatted_caption()
        .filter(|fb| fb.format == MessageFormat::Html)
        .map(|fb| format!("<br><i>{}</i>", fb.body))
        .or_else(|| {
            audio
                .caption()
                .map(|c| format!("<br><i>{}</i>", htmlize::escape_text(c)))
        })
        .unwrap_or_default();
    let actions = media_download_url(&audio.source, audio.filename(), false, &metadata)
        .and_then(|download_url| {
            media_download_url(&audio.source, audio.filename(), true, &metadata)
                .map(|play_url| (download_url, play_url))
        })
        .map(|(download_url, play_url)| {
            let source_mxc = match &audio.source {
                MediaSource::Plain(mxc_uri) => Some(mxc_uri.as_str()),
                MediaSource::Encrypted(_) => None,
            };
            let metadata_link = media_metadata_clipboard_link(audio.filename(), &metadata);
            let packet_link =
                media_operation_packet_link("Save/Open", audio.filename(), &metadata, true);
            let contract_link =
                media_playback_queue_contract_link("Save/Open", audio.filename(), &metadata, true);
            let taxonomy_link =
                media_playback_result_taxonomy_link("Save/Open", audio.filename(), &metadata, true);
            let preflight_controls = media_save_preflight_detail_controls_preview_with_source(
                "Save/Open",
                audio.filename(),
                &metadata,
                true,
                source_mxc,
            );
            let recovery_controls = media_save_result_recovery_controls_preview_with_source(
                "Save/Open",
                audio.filename(),
                &metadata,
                true,
                source_mxc,
            );
            format!(
                "<br>{metadata_link} · {packet_link} · {contract_link} · {taxonomy_link} · <a href=\"{}\">Download</a> · <a href=\"{}\">Play</a>{preflight_controls}{recovery_controls}",
                htmlize::escape_attribute(&download_url),
                htmlize::escape_attribute(&play_url),
            )
        })
        .unwrap_or_else(|| {
            encrypted_media_local_metadata_preview("audio", audio.filename(), &metadata)
        });
    let inline_controls = media_inline_player_disabled_controls_preview(
        "Audio",
        audio.filename(),
        &metadata.summary(audio.filename()),
    );
    let codec_controls =
        media_codec_transcode_controls_preview("Save/Open", audio.filename(), &metadata, true);

    message_content_widget.show_html(
        cx,
        format!(
            "Audio: <b>{filename}</b>{mime}{duration}{size}{caption}{actions}{inline_controls}{codec_controls}<br>\
            <i>{MEDIA_AUDIO_COMPACT_LABEL}</i>"
        ),
    );
    true
}

/// Draws a video message's content into the given `message_content_widget`.
///
/// Returns whether the video message content was fully drawn.
fn populate_video_message_content(
    cx: &mut Cx,
    message_content_widget: &HtmlOrPlaintextRef,
    video: &VideoMessageEventContent,
) -> bool {
    // Display the file name, human-readable size, caption, and a button to download it.
    let filename = htmlize::escape_text(video.filename());
    let (metadata_duration, metadata_mime, metadata_size, metadata_dimensions) = video
        .info
        .as_ref()
        .map(|info| {
            (
                info.duration.map(|d| format!("{:.2} sec", d.as_secs_f64())),
                info.mimetype
                    .as_ref()
                    .filter(|m| !m.trim().is_empty())
                    .map(ToString::to_string),
                info.size.map(|bytes| ByteSize::b(bytes.into()).to_string()),
                info.width
                    .and_then(|width| info.height.map(|height| format!("{width}x{height}"))),
            )
        })
        .unwrap_or_default();
    let duration = metadata_duration
        .as_ref()
        .map(|duration| format!("  {duration},"))
        .unwrap_or_default();
    let mime = metadata_mime
        .as_ref()
        .map(|mime| format!("  {},", htmlize::escape_text(mime)))
        .unwrap_or_default();
    let size = metadata_size
        .as_ref()
        .map(|size| format!("  ({size}),"))
        .unwrap_or_default();
    let dimensions = metadata_dimensions
        .as_ref()
        .map(|dimensions| format!("  {dimensions},"))
        .unwrap_or_default();
    let mut metadata = MediaDownloadActionMetadata::new("Video");
    metadata.mime_type = metadata_mime;
    metadata.size_label = metadata_size;
    metadata.duration_label = metadata_duration;
    metadata.dimensions_label = metadata_dimensions;
    let caption = video
        .formatted_caption()
        .filter(|fb| fb.format == MessageFormat::Html)
        .map(|fb| format!("<br><i>{}</i>", fb.body))
        .or_else(|| {
            video
                .caption()
                .map(|c| format!("<br><i>{}</i>", htmlize::escape_text(c)))
        })
        .unwrap_or_default();
    let actions = media_download_url(&video.source, video.filename(), false, &metadata)
        .and_then(|download_url| {
            media_download_url(&video.source, video.filename(), true, &metadata)
                .map(|play_url| (download_url, play_url))
        })
        .map(|(download_url, play_url)| {
            let source_mxc = match &video.source {
                MediaSource::Plain(mxc_uri) => Some(mxc_uri.as_str()),
                MediaSource::Encrypted(_) => None,
            };
            let metadata_link = media_metadata_clipboard_link(video.filename(), &metadata);
            let packet_link =
                media_operation_packet_link("Save/Open", video.filename(), &metadata, true);
            let contract_link =
                media_playback_queue_contract_link("Save/Open", video.filename(), &metadata, true);
            let taxonomy_link =
                media_playback_result_taxonomy_link("Save/Open", video.filename(), &metadata, true);
            let preflight_controls = media_save_preflight_detail_controls_preview_with_source(
                "Save/Open",
                video.filename(),
                &metadata,
                true,
                source_mxc,
            );
            let recovery_controls = media_save_result_recovery_controls_preview_with_source(
                "Save/Open",
                video.filename(),
                &metadata,
                true,
                source_mxc,
            );
            format!(
                "<br>{metadata_link} · {packet_link} · {contract_link} · {taxonomy_link} · <a href=\"{}\">Download</a> · <a href=\"{}\">Play</a>{preflight_controls}{recovery_controls}",
                htmlize::escape_attribute(&download_url),
                htmlize::escape_attribute(&play_url),
            )
        })
        .unwrap_or_else(|| {
            encrypted_media_local_metadata_preview("video", video.filename(), &metadata)
        });
    let inline_controls = media_inline_player_disabled_controls_preview(
        "Video",
        video.filename(),
        &metadata.summary(video.filename()),
    );
    let codec_controls =
        media_codec_transcode_controls_preview("Save/Open", video.filename(), &metadata, true);

    message_content_widget.show_html(
        cx,
        format!(
            "Video: <b>{filename}</b>{mime}{duration}{size}{dimensions}{caption}{actions}{inline_controls}{codec_controls}<br>\
            <i>{MEDIA_VIDEO_COMPACT_LABEL}</i>"
        ),
    );
    true
}

#[cfg(test)]
mod media_download_tests {
    use super::*;

    #[test]
    fn encrypted_media_local_metadata_preview_includes_loaded_metadata_and_boundaries() {
        let mut metadata = MediaDownloadActionMetadata::new("Audio");
        metadata.mime_type = Some("audio/ogg".to_string());
        metadata.size_label = Some("42 KiB".to_string());
        metadata.duration_label = Some("3.50 sec".to_string());

        let preview = encrypted_media_local_metadata_preview("audio", "voice.ogg", &metadata);

        assert!(preview.contains("Encrypted audio metadata"));
        assert!(preview.contains("Audio"));
        assert!(preview.contains("voice.ogg"));
        assert!(preview.contains("audio/ogg"));
        assert!(preview.contains("42 KiB"));
        assert!(preview.contains("3.50 sec"));
        assert!(preview.contains(MEDIA_ENCRYPTED_METADATA_LOCAL_LABEL));
        assert!(preview.contains("No decrypt"));
        assert!(preview.contains("SaveMedia"));
        assert!(preview.contains("FetchMedia"));
        assert!(preview.contains("gateway/runtime/auth"));
        assert!(preview.contains("live mutation"));
    }

    #[test]
    fn encrypted_media_local_metadata_preview_escapes_loaded_filename() {
        let metadata = MediaDownloadActionMetadata::new("File");

        let preview =
            encrypted_media_local_metadata_preview("file", "invoice <draft>.pdf", &metadata);

        assert!(preview.contains("invoice &lt;draft&gt;.pdf"));
        assert!(!preview.contains("invoice <draft>.pdf"));
    }

    #[test]
    fn media_save_dialog_lifecycle_metadata_label_summarizes_loaded_action_state() {
        let label = media_save_dialog_lifecycle_metadata_label(
            "Play",
            "clip.mp4",
            "Video · clip.mp4 · video/mp4 · 12 MB · 4.20 sec · 1280x720",
            "save dialog canceled; no download request was sent",
        );

        assert!(label.contains("Media Play save dialog canceled"));
        assert!(label.contains("clip.mp4"));
        assert!(label.contains("video/mp4"));
        assert!(label.contains("12 MB"));
        assert!(label.contains("4.20 sec"));
        assert!(label.contains("1280x720"));
        assert!(label.contains(MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_LABEL));
        assert!(MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_EVIDENCE.contains("confirmation opened"));
        assert!(MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_EVIDENCE.contains("save dialog canceled"));
        assert!(MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_EVIDENCE.contains("SaveMedia is still submitted only after confirmation accepts and the local save dialog returns a selected path"));
        assert!(MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_EVIDENCE.contains("no extra media fetch"));
        assert!(MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_EVIDENCE.contains("gateway/runtime/auth"));
        assert!(MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn media_save_dialog_lifecycle_metadata_label_uses_safe_fallbacks() {
        let label = media_save_dialog_lifecycle_metadata_label("", "", "", "");

        assert!(label.contains("Media Download status updated"));
        assert!(label.contains("hepta-media-download"));
        assert!(label.contains("metadata unavailable"));
        assert!(label.contains(MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_LABEL));
    }

    #[test]
    fn media_metadata_clipboard_payload_uses_loaded_timeline_metadata_only() {
        let mut metadata = MediaDownloadActionMetadata::new("Video");
        metadata.mime_type = Some("video/mp4".to_string());
        metadata.size_label = Some("12 MB".to_string());
        metadata.duration_label = Some("4.20 sec".to_string());
        metadata.dimensions_label = Some("1280x720".to_string());

        let payload = media_metadata_clipboard_payload("clip.mp4", &metadata);

        assert!(payload.contains("Media metadata"));
        assert!(payload.contains("Type: Video"));
        assert!(payload.contains("Filename: clip.mp4"));
        assert!(payload.contains("MIME: video/mp4"));
        assert!(payload.contains("Size: 12 MB"));
        assert!(payload.contains("Duration: 4.20 sec"));
        assert!(payload.contains("Dimensions: 1280x720"));
        assert!(
            payload.contains("Summary: Video · clip.mp4 · video/mp4 · 12 MB · 4.20 sec · 1280x720")
        );
        assert!(payload.contains(MEDIA_METADATA_CLIPBOARD_LABEL));
        assert!(payload.contains("No FetchMedia"));
        assert!(payload.contains("SaveMedia"));
        assert!(payload.contains("gateway/runtime/auth"));
        assert!(payload.contains("live mutation"));
        assert!(MEDIA_METADATA_CLIPBOARD_EVIDENCE.contains("local clipboard"));
        assert!(MEDIA_METADATA_CLIPBOARD_EVIDENCE.contains("plain and encrypted"));
    }

    #[test]
    fn media_metadata_clipboard_label_reports_copied_state() {
        let copied = media_metadata_clipboard_label(
            true,
            "clip.mp4",
            "Video · clip.mp4 · video/mp4 · 12 MB",
        );
        let unavailable = media_metadata_clipboard_label(false, "", "");

        assert!(copied.contains("copied loaded media metadata to local clipboard"));
        assert!(copied.contains("clip.mp4"));
        assert!(copied.contains("video/mp4"));
        assert!(copied.contains(MEDIA_METADATA_CLIPBOARD_LABEL));
        assert!(copied.contains("No FetchMedia"));
        assert!(unavailable.contains("media metadata clipboard unavailable"));
        assert!(unavailable.contains("hepta-media-download"));
        assert!(unavailable.contains("metadata unavailable"));
    }

    #[test]
    fn media_metadata_clipboard_url_carries_metadata_without_mxc() {
        let mut metadata = MediaDownloadActionMetadata::new("Audio");
        metadata.mime_type = Some("audio/ogg".to_string());
        metadata.size_label = Some("42 KiB".to_string());
        metadata.duration_label = Some("3.50 sec".to_string());

        let url = media_metadata_clipboard_url("voice.ogg", &metadata);
        let link = media_metadata_clipboard_link("voice.ogg", &metadata);

        assert!(url.starts_with(MEDIA_METADATA_CLIPBOARD_URL_SCHEME));
        assert!(url.contains("name=voice.ogg"));
        assert!(url.contains("kind=Audio"));
        assert!(url.contains("mime=audio%2Fogg"));
        assert!(!url.contains("mxc="));
        assert!(link.contains("Copy metadata"));
        assert!(link.contains(MEDIA_METADATA_CLIPBOARD_URL_SCHEME));
    }

    #[test]
    fn media_operation_packet_payload_lists_acceptance_criteria() {
        let mut metadata = MediaDownloadActionMetadata::new("Video");
        metadata.mime_type = Some("video/mp4".to_string());
        metadata.size_label = Some("12 MB".to_string());
        metadata.duration_label = Some("4.20 sec".to_string());
        metadata.dimensions_label = Some("1280x720".to_string());

        let payload = media_operation_packet_payload("Save/Open", "clip.mp4", &metadata, true);

        assert!(payload.contains("Media operation packet"));
        assert!(payload.contains("Requested action: Save/Open"));
        assert!(payload.contains("Video"));
        assert!(payload.contains("clip.mp4"));
        assert!(payload.contains("video/mp4"));
        assert!(payload.contains("Inline playback acceptance"));
        assert!(payload.contains("Decrypt/decode acceptance"));
        assert!(payload.contains("Codec/transcode acceptance"));
        assert!(payload.contains("Queue acceptance"));
        assert!(payload.contains("system opener result slot"));
        assert!(payload.contains("Promotion criteria"));
        assert!(payload.contains("typed media operation contract"));
        assert!(payload.contains("no FetchMedia"));
        assert!(payload.contains("SaveMedia"));
        assert!(payload.contains("gateway/runtime/auth"));
        assert!(payload.contains("live mutation"));
        assert!(
            MEDIA_OPERATION_PACKET_DRILLDOWN_EVIDENCE
                .contains("local media operation acceptance matrix")
        );
        assert!(MEDIA_OPERATION_PACKET_DRILLDOWN_EVIDENCE.contains("encrypted file/audio/video"));
    }

    #[test]
    fn media_operation_packet_link_uses_loaded_metadata_without_mxc() {
        let mut metadata = MediaDownloadActionMetadata::new("Audio");
        metadata.mime_type = Some("audio/ogg".to_string());
        metadata.duration_label = Some("9.00 sec".to_string());

        let url = media_operation_packet_url("Save/Open", "voice <draft>.ogg", &metadata, true);
        let link = media_operation_packet_link("Save/Open", "voice <draft>.ogg", &metadata, true);
        let label = media_operation_packet_clipboard_label(
            true,
            "Save/Open",
            "voice <draft>.ogg",
            "Audio · voice <draft>.ogg · audio/ogg · 9.00 sec",
            true,
        );

        assert!(url.starts_with(MEDIA_OPERATION_PACKET_URL_SCHEME));
        assert!(url.contains("label=Save%2FOpen"));
        assert!(url.contains("kind=Audio"));
        assert!(url.contains("mime=audio%2Fogg"));
        assert!(url.contains("duration=9.00+sec"));
        assert!(url.contains("open=1"));
        assert!(!url.contains("mxc="));
        assert!(link.contains(">Packet</a>"));
        assert!(link.contains("voice+%3Cdraft%3E.ogg"));
        assert!(!link.contains("voice <draft>.ogg"));
        assert!(label.contains("copied local media operation packet to clipboard"));
        assert!(label.contains("Play opener result slot included"));
        assert!(label.contains(MEDIA_OPERATION_PACKET_DRILLDOWN_LABEL));
    }

    #[test]
    fn media_playback_queue_contract_payload_lists_typed_contract_slots() {
        let mut metadata = MediaDownloadActionMetadata::new("Video");
        metadata.mime_type = Some("video/mp4".to_string());
        metadata.size_label = Some("12 MB".to_string());
        metadata.duration_label = Some("4.20 sec".to_string());
        metadata.dimensions_label = Some("1280x720".to_string());

        let payload =
            media_playback_queue_contract_payload("Save/Open", "clip.mp4", &metadata, true);

        assert!(payload.contains("Media playback/queue typed contract"));
        assert!(payload.contains("Requested action: Save/Open"));
        assert!(payload.contains("Media identity"));
        assert!(payload.contains("SaveMedia contract"));
        assert!(payload.contains("Inline playback contract"));
        assert!(payload.contains("Decrypt/decode contract"));
        assert!(payload.contains("Codec/transcode/captions contract"));
        assert!(payload.contains("Queue contract"));
        assert!(payload.contains("system opener/open-folder result contract"));
        assert!(payload.contains("stale local file"));
        assert!(payload.contains("Promotion blockers"));
        assert!(payload.contains("typed playback/media queue adapter"));
        assert!(payload.contains("no FetchMedia"));
        assert!(payload.contains("SaveMedia"));
        assert!(payload.contains("gateway/runtime/auth"));
        assert!(payload.contains("live mutation"));
        assert!(payload.contains(MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_LABEL));
        assert!(
            MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_EVIDENCE
                .contains("typed playback/media queue contract")
        );
        assert!(MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_EVIDENCE.contains("stale local file"));
        assert!(MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_EVIDENCE.contains("submits no FetchMedia"));
    }

    #[test]
    fn media_playback_queue_contract_link_uses_loaded_metadata_without_mxc() {
        let mut metadata = MediaDownloadActionMetadata::new("Audio");
        metadata.mime_type = Some("audio/ogg".to_string());
        metadata.duration_label = Some("9.00 sec".to_string());

        let url =
            media_playback_queue_contract_url("Save/Open", "voice <draft>.ogg", &metadata, true);
        let link =
            media_playback_queue_contract_link("Save/Open", "voice <draft>.ogg", &metadata, true);
        let label = media_playback_queue_contract_clipboard_label(
            true,
            "Save/Open",
            "voice <draft>.ogg",
            "Audio · voice <draft>.ogg · audio/ogg · 9.00 sec",
            true,
        );

        assert!(url.starts_with(MEDIA_PLAYBACK_QUEUE_CONTRACT_URL_SCHEME));
        assert!(url.contains("label=Save%2FOpen"));
        assert!(url.contains("kind=Audio"));
        assert!(url.contains("mime=audio%2Fogg"));
        assert!(url.contains("duration=9.00+sec"));
        assert!(url.contains("open=1"));
        assert!(!url.contains("mxc="));
        assert!(link.contains(">Contract</a>"));
        assert!(link.contains("voice+%3Cdraft%3E.ogg"));
        assert!(!link.contains("voice <draft>.ogg"));
        assert!(label.contains("copied local media playback/queue contract to clipboard"));
        assert!(label.contains("Play opener/result contract included"));
        assert!(label.contains(MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_LABEL));
    }

    #[test]
    fn media_playback_result_taxonomy_payload_lists_blocked_result_slots() {
        let mut metadata = MediaDownloadActionMetadata::new("Video");
        metadata.mime_type = Some("video/mp4".to_string());
        metadata.size_label = Some("12 MB".to_string());
        metadata.duration_label = Some("4.20 sec".to_string());
        metadata.dimensions_label = Some("1280x720".to_string());

        let payload =
            media_playback_result_taxonomy_payload("Save/Open", "clip.mp4", &metadata, true);

        assert!(payload.contains("Media playback decrypt/decode result taxonomy packet"));
        assert!(payload.contains("Live result references"));
        assert!(payload.contains("MatrixRequest::FetchMedia"));
        assert!(payload.contains("MatrixRequest::SaveMedia"));
        assert!(payload.contains("Open folder/Replay"));
        assert!(payload.contains("PositiveConfirmationModal"));
        assert!(payload.contains("playback_session_id: not_assigned"));
        assert!(payload.contains("decrypt_operation_id: not_assigned"));
        assert!(payload.contains("decrypt_result: decrypted, missing_key"));
        assert!(payload.contains("decode_result: decoded_image, decoded_audio, decoded_video"));
        assert!(payload.contains("codec_fallback_result: transcoded"));
        assert!(payload.contains("background_queue_result: queued, resumed, cancelled"));
        assert!(payload.contains("delivery_receipt_result: not_wired"));
        assert!(payload.contains("cached_file_stale_result"));
        assert!(payload.contains("audit_redaction"));
        assert!(payload.contains(MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_LABEL));
        assert!(
            MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("decrypt/decode/opener/queue result taxonomy")
        );
        assert!(
            MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("not-assigned/not-wired result slots")
        );
        assert!(MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn media_playback_result_taxonomy_link_uses_loaded_metadata_without_mxc() {
        let mut metadata = MediaDownloadActionMetadata::new("Audio");
        metadata.mime_type = Some("audio/ogg".to_string());
        metadata.duration_label = Some("9.00 sec".to_string());

        let url =
            media_playback_result_taxonomy_url("Save/Open", "voice <draft>.ogg", &metadata, true);
        let link =
            media_playback_result_taxonomy_link("Save/Open", "voice <draft>.ogg", &metadata, true);
        let label = media_playback_result_taxonomy_clipboard_label(
            true,
            "Save/Open",
            "voice <draft>.ogg",
            "Audio · voice <draft>.ogg · audio/ogg · 9.00 sec",
            true,
        );

        assert!(url.starts_with(MEDIA_PLAYBACK_RESULT_TAXONOMY_URL_SCHEME));
        assert!(url.contains("label=Save%2FOpen"));
        assert!(url.contains("kind=Audio"));
        assert!(url.contains("mime=audio%2Fogg"));
        assert!(url.contains("duration=9.00+sec"));
        assert!(url.contains("open=1"));
        assert!(!url.contains("mxc="));
        assert!(link.contains(">Taxonomy</a>"));
        assert!(link.contains("voice+%3Cdraft%3E.ogg"));
        assert!(!link.contains("voice <draft>.ogg"));
        assert!(label.contains("copied local media result taxonomy to clipboard"));
        assert!(label.contains("Play opener taxonomy included"));
        assert!(label.contains(MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_LABEL));
    }

    #[test]
    fn media_save_destination_metadata_label_summarizes_picked_path() {
        let label = media_save_destination_metadata_label(
            "Play",
            "clip.mp4",
            "Video · clip.mp4 · video/mp4 · 12 MB · 4.20 sec · 1280x720",
            std::path::Path::new("/tmp/clip.mp4"),
            true,
        );

        assert!(label.contains("Media Play destination selected"));
        assert!(label.contains("clip.mp4"));
        assert!(label.contains("/tmp/clip.mp4"));
        assert!(label.contains("video/mp4"));
        assert!(label.contains("system opener"));
        assert!(label.contains(MEDIA_SAVE_DESTINATION_METADATA_LABEL));
        assert!(
            MEDIA_SAVE_DESTINATION_METADATA_EVIDENCE.contains("selected local destination path")
        );
        assert!(
            MEDIA_SAVE_DESTINATION_METADATA_EVIDENCE.contains("PositiveConfirmationModal accept")
        );
        assert!(MEDIA_SAVE_DESTINATION_METADATA_EVIDENCE.contains("MatrixRequest::SaveMedia"));
        assert!(MEDIA_SAVE_DESTINATION_METADATA_EVIDENCE.contains("retry/cancel queue controls"));
        assert!(MEDIA_SAVE_DESTINATION_METADATA_EVIDENCE.contains("gateway/runtime/auth"));
        assert!(MEDIA_SAVE_DESTINATION_METADATA_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn media_save_destination_metadata_label_uses_safe_fallbacks() {
        let label =
            media_save_destination_metadata_label("", "", "", std::path::Path::new(""), false);

        assert!(label.contains("Media Download destination selected"));
        assert!(label.contains("hepta-media-download"));
        assert!(label.contains("metadata unavailable"));
        assert!(label.contains("will only save to the selected local path"));
    }

    #[test]
    fn media_inline_playback_queue_boundary_label_summarizes_play_path() {
        let label = media_inline_playback_queue_boundary_label(
            "Play",
            "clip.mp4",
            "Video · clip.mp4 · video/mp4 · 12 MB · 4.20 sec · 1280x720",
            true,
        );

        assert!(label.contains("Video"));
        assert!(label.contains("clip.mp4"));
        assert!(label.contains("MatrixRequest::SaveMedia"));
        assert!(label.contains("system opener"));
        assert!(label.contains("inline audio/video controls"));
        assert!(label.contains("decrypt"));
        assert!(label.contains("codec/transcode"));
        assert!(label.contains("retry/cancel queue controls"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MEDIA_INLINE_PLAYBACK_QUEUE_BOUNDARY_LABEL));
    }

    #[test]
    fn media_inline_playback_queue_boundary_label_summarizes_download_path() {
        let label = media_inline_playback_queue_boundary_label("", "", "", false);

        assert!(label.contains("metadata unavailable"));
        assert!(label.contains("Download writes the picked local file only after confirmation"));
        assert!(label.contains("hepta-media-download"));
        assert!(label.contains("unwired"));
    }

    #[test]
    fn media_inline_player_disabled_controls_preview_is_visible_and_local() {
        let preview = media_inline_player_disabled_controls_preview(
            "Video",
            "clip <draft>.mp4",
            "Video · clip.mp4 · video/mp4 · 12 MB · 4.20 sec · 1280x720",
        );

        assert!(preview.contains("Video inline controls disabled"));
        assert!(preview.contains("clip &lt;draft&gt;.mp4"));
        assert!(!preview.contains("clip <draft>.mp4"));
        assert!(preview.contains("Playhead 00:00"));
        assert!(preview.contains("Seek"));
        assert!(preview.contains("Queue"));
        assert!(preview.contains("Decrypt"));
        assert!(preview.contains("Codec"));
        assert!(preview.contains(MEDIA_INLINE_PLAYER_DISABLED_CONTROLS_LABEL));
        assert!(preview.contains("No FetchMedia"));
        assert!(preview.contains("SaveMedia"));
        assert!(preview.contains("gateway/runtime/auth"));
        assert!(preview.contains("live mutation"));
        assert!(
            MEDIA_INLINE_PLAYER_DISABLED_CONTROLS_EVIDENCE
                .contains("visible disabled inline-player")
        );
        assert!(
            MEDIA_INLINE_PLAYER_DISABLED_CONTROLS_EVIDENCE
                .contains("Download/Play remain the only active links")
        );
    }

    #[test]
    fn media_inline_player_disabled_controls_preview_uses_safe_fallbacks() {
        let preview = media_inline_player_disabled_controls_preview("", "", "");

        assert!(preview.contains("Media inline controls disabled"));
        assert!(preview.contains("hepta-media-download"));
        assert!(preview.contains("metadata unavailable"));
        assert!(preview.contains(MEDIA_INLINE_PLAYER_DISABLED_CONTROLS_LABEL));
    }

    #[test]
    fn media_codec_transcode_control_label_keeps_actions_local() {
        let label = media_codec_transcode_control_label(
            "Transcode",
            "Save/Open",
            "clip.mp4",
            "Video · clip.mp4 · video/mp4 · 12 MB · 4.20 sec · 1280x720",
            true,
        );

        assert!(label.contains("Media codec/transcode Transcode stayed local"));
        assert!(label.contains("clip.mp4"));
        assert!(label.contains("video/mp4"));
        assert!(label.contains("no transcoder starts"));
        assert!(label.contains("confirmed SaveMedia"));
        assert!(label.contains("No FetchMedia"));
        assert!(label.contains("SaveMedia"));
        assert!(label.contains("decoder"));
        assert!(label.contains("transcoder"));
        assert!(label.contains("captions fetch"));
        assert!(label.contains("quality switch"));
        assert!(label.contains("decrypt"));
        assert!(label.contains("inline player startup"));
        assert!(label.contains("retry/cancel queue mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MEDIA_CODEC_TRANSCODE_CONTROLS_LABEL));
        assert!(MEDIA_CODEC_TRANSCODE_CONTROLS_EVIDENCE.contains("Codec"));
        assert!(MEDIA_CODEC_TRANSCODE_CONTROLS_EVIDENCE.contains("Transcode"));
        assert!(MEDIA_CODEC_TRANSCODE_CONTROLS_EVIDENCE.contains("Captions"));
        assert!(MEDIA_CODEC_TRANSCODE_CONTROLS_EVIDENCE.contains("Quality"));
        assert!(MEDIA_CODEC_TRANSCODE_CONTROLS_EVIDENCE.contains("Decrypt"));
        assert!(MEDIA_CODEC_TRANSCODE_CONTROLS_EVIDENCE.contains("does not submit FetchMedia"));
    }

    #[test]
    fn media_codec_transcode_controls_preview_contains_visible_links() {
        let mut metadata = MediaDownloadActionMetadata::new("Video");
        metadata.mime_type = Some("video/mp4".to_string());
        metadata.size_label = Some("12 MB".to_string());
        metadata.duration_label = Some("4.20 sec".to_string());
        metadata.dimensions_label = Some("1280x720".to_string());

        let preview = media_codec_transcode_controls_preview(
            "Save/Open",
            "clip <draft>.mp4",
            &metadata,
            true,
        );

        assert!(preview.contains("Codec/transcode controls"));
        assert!(preview.contains("Codec"));
        assert!(preview.contains("Transcode"));
        assert!(preview.contains("Captions"));
        assert!(preview.contains("Quality"));
        assert!(preview.contains("Decrypt"));
        assert!(preview.contains(MEDIA_CODEC_TRANSCODE_CONTROL_URL_SCHEME));
        assert!(preview.contains("clip+%3Cdraft%3E.mp4"));
        assert!(preview.contains("clip &lt;draft&gt;.mp4"));
        assert!(!preview.contains("clip <draft>.mp4"));
        assert!(preview.contains(MEDIA_CODEC_TRANSCODE_CONTROLS_LABEL));
        assert!(preview.contains("No FetchMedia"));
        assert!(preview.contains("queue mutation"));
    }

    #[test]
    fn media_codec_transcode_control_url_carries_metadata_without_plain_filename() {
        let mut metadata = MediaDownloadActionMetadata::new("Audio");
        metadata.mime_type = Some("audio/ogg".to_string());
        metadata.duration_label = Some("9.00 sec".to_string());

        let url = media_codec_transcode_control_url(
            "Codec",
            "Save/Open",
            "clip <draft>.ogg",
            &metadata,
            true,
        );

        assert!(url.starts_with(MEDIA_CODEC_TRANSCODE_CONTROL_URL_SCHEME));
        assert!(url.contains("action=Codec"));
        assert!(url.contains("kind=Audio"));
        assert!(url.contains("mime=audio%2Fogg"));
        assert!(url.contains("duration=9.00+sec"));
        assert!(url.contains("open=1"));
        assert!(!url.contains("clip <draft>.ogg"));
    }

    #[test]
    fn media_save_result_status_boundary_label_summarizes_play_result_path() {
        let label = media_save_result_status_boundary_label(
            "Play",
            "clip.mp4",
            "Video · clip.mp4 · video/mp4 · 12 MB · 4.20 sec · 1280x720",
            true,
        );

        assert!(label.contains("Media Play result boundary"));
        assert!(label.contains("clip.mp4"));
        assert!(label.contains("video/mp4"));
        assert!(label.contains("SaveMedia completion reports saved"));
        assert!(label.contains("download failed"));
        assert!(label.contains("save failed"));
        assert!(label.contains("system opener"));
        assert!(label.contains("Open folder"));
        assert!(label.contains("destination cache"));
        assert!(label.contains("Inline audio/video player"));
        assert!(label.contains("retry/cancel queue controls"));
        assert!(label.contains("decrypt retry"));
        assert!(label.contains("codec/transcode fallback"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MEDIA_SAVE_RESULT_STATUS_BOUNDARY_LABEL));
    }

    #[test]
    fn media_save_result_status_boundary_label_uses_safe_fallbacks() {
        let label = media_save_result_status_boundary_label("", "", "", false);

        assert!(label.contains("Media Download result boundary"));
        assert!(label.contains("hepta-media-download"));
        assert!(label.contains("metadata unavailable"));
        assert!(label.contains("no system opener request is attempted"));
        assert!(label.contains(MEDIA_SAVE_RESULT_STATUS_BOUNDARY_LABEL));
    }

    #[test]
    fn media_save_result_recovery_control_label_keeps_decrypt_local() {
        let label = media_save_result_recovery_control_label(
            "Decrypt",
            "Save/Open",
            "clip.mp4",
            "Video · clip.mp4 · video/mp4 · 12 MB · 4.20 sec · 1280x720",
            true,
        );

        assert!(label.contains("Media recovery control Decrypt stayed local"));
        assert!(label.contains("Save/Open"));
        assert!(label.contains("clip.mp4"));
        assert!(label.contains("video/mp4"));
        assert!(label.contains("SaveMedia completion"));
        assert!(label.contains("download failed"));
        assert!(label.contains("save failed"));
        assert!(label.contains("Open folder/Replay cache updates"));
        assert!(label.contains("No cached Open folder/Replay handoff for this action"));
        assert!(label.contains("SaveMedia retry"));
        assert!(label.contains("FetchMedia"));
        assert!(label.contains("queue control"));
        assert!(label.contains("decrypt retry"));
        assert!(label.contains("codec/transcode"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_LABEL));
        assert!(MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE.contains("Open folder"));
        assert!(MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE.contains("live local OS handoff"));
        assert!(
            MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
                .contains("cached-destination Open folder")
        );
        assert!(MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE.contains("Replay"));
        assert!(MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE.contains(
            "live local OS handoff from the same cached successful SaveMedia destination"
        ));
        assert!(MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE.contains("Retry"));
        assert!(MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE.contains("Queue"));
        assert!(
            MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
                .contains("local media playback/download queue snapshot")
        );
        assert!(MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE.contains("Decrypt"));
        assert!(
            MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
                .contains("Retry is a guarded live resubmit")
        );
        assert!(
            MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE.contains("MatrixRequest::SaveMedia")
        );
        assert!(
            MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE.contains("no unconfirmed FetchMedia")
        );
    }

    #[test]
    fn media_replay_labels_require_cached_successful_save() {
        let unavailable = media_replay_unavailable_label(
            "Save/Open",
            "clip.mp4",
            "Video · clip.mp4 · video/mp4",
            "no successful SaveMedia destination is cached for this row yet",
        );
        let handed_off = media_replay_result_label(
            "Save/Open",
            "clip.mp4",
            "Video · clip.mp4 · video/mp4",
            std::path::Path::new("/tmp/hepta/clip.mp4"),
        );
        let failed = media_replay_failed_label(
            "Save/Open",
            "clip.mp4",
            "Video · clip.mp4 · video/mp4",
            std::path::Path::new("/tmp/hepta/clip.mp4"),
            "stale file",
        );

        assert!(unavailable.contains("Replay unavailable"));
        assert!(unavailable.contains("cached successful SaveMedia destination"));
        assert!(unavailable.contains("validates the saved file"));
        assert!(unavailable.contains("clears stale cached destinations"));
        assert!(handed_off.contains("Replay opened cached saved file"));
        assert!(handed_off.contains("Saved file: /tmp/hepta/clip.mp4"));
        assert!(handed_off.contains("No FetchMedia"));
        assert!(handed_off.contains("no SaveMedia"));
        assert!(failed.contains("Replay request was attempted and failed"));
        assert!(failed.contains("stale file"));
    }

    #[test]
    fn media_open_folder_labels_require_cached_successful_save() {
        let unavailable = media_open_folder_unavailable_label(
            "Save/Open",
            "clip.mp4",
            "Video · clip.mp4 · video/mp4",
            "no successful SaveMedia destination is cached for this row yet",
        );
        let handed_off = media_open_folder_result_label(
            "Save/Open",
            "clip.mp4",
            "Video · clip.mp4 · video/mp4",
            std::path::Path::new("/tmp/hepta/clip.mp4"),
            std::path::Path::new("/tmp/hepta"),
        );
        let failed = media_open_folder_failed_label(
            "Save/Open",
            "clip.mp4",
            "Video · clip.mp4 · video/mp4",
            std::path::Path::new("/tmp/hepta/clip.mp4"),
            "permission denied",
        );

        assert!(unavailable.contains("Open folder unavailable"));
        assert!(unavailable.contains("cached successful SaveMedia destination"));
        assert!(unavailable.contains("clearing stale cached destinations"));
        assert!(unavailable.contains("no successful SaveMedia destination"));
        assert!(handed_off.contains("Open folder handed"));
        assert!(handed_off.contains("Saved file: /tmp/hepta/clip.mp4"));
        assert!(handed_off.contains("Opened folder: /tmp/hepta"));
        assert!(handed_off.contains("No FetchMedia"));
        assert!(handed_off.contains("no SaveMedia"));
        assert!(failed.contains("Open folder request was attempted and failed"));
        assert!(failed.contains("permission denied"));
        assert_eq!(
            media_save_destination_cache_key("  mxc://example.org/media  "),
            Some("mxc://example.org/media")
        );
        assert_eq!(media_save_destination_cache_key("   "), None);
        assert!(
            media_cached_saved_file_stale_reason(std::path::Path::new(
                "/definitely/missing/hepta.mp4"
            ))
            .is_some()
        );
    }

    #[test]
    fn media_cached_saved_file_status_label_reads_regular_file_metadata() {
        let path = std::env::temp_dir().join(format!(
            "hepta-media-status-{}-{}.bin",
            std::process::id(),
            "regular"
        ));
        fs::write(&path, b"hepta").unwrap();

        let label = media_cached_saved_file_status_label(&path).unwrap();

        assert!(label.contains("Cached saved-file status"));
        assert!(label.contains("regular file"));
        assert!(label.contains("5 B"));
        assert!(label.contains("modified"));
        assert!(label.contains(MEDIA_CACHED_SAVED_FILE_STATUS_LABEL));
        assert!(MEDIA_CACHED_SAVED_FILE_STATUS_EVIDENCE.contains("regular-file state"));
        assert!(MEDIA_CACHED_SAVED_FILE_STATUS_EVIDENCE.contains("clear the cached MXC"));
        assert!(MEDIA_CACHED_SAVED_FILE_STATUS_EVIDENCE.contains("no FetchMedia"));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn media_cached_saved_file_status_label_rejects_missing_file() {
        let missing = std::env::temp_dir().join(format!(
            "hepta-media-status-{}-missing.bin",
            std::process::id()
        ));
        let _ = fs::remove_file(&missing);

        let reason = media_cached_saved_file_status_label(&missing).unwrap_err();

        assert!(reason.contains("Cached saved-file status unavailable"));
        assert!(reason.contains("missing"));
    }

    #[test]
    fn media_playback_download_queue_snapshot_label_summarizes_loaded_queue_state() {
        let label = media_playback_download_queue_snapshot_label(
            "Queue",
            "Save/Open",
            "clip.mp4",
            "Video · clip.mp4 · video/mp4 · 12 MB · 4.20 sec · 1280x720",
            true,
            Some(
                "Cached saved-file status: regular file, 12 MB, writable, modified 42s since epoch, path /tmp/clip.mp4.",
            ),
        );

        assert!(label.contains("Local media playback/download queue snapshot"));
        assert!(label.contains("Queue selected"));
        assert!(label.contains("Save/Open"));
        assert!(label.contains("clip.mp4"));
        assert!(label.contains("video/mp4"));
        assert!(label.contains("Play mode would save first"));
        assert!(label.contains("Cached saved-file status"));
        assert!(label.contains("regular file"));
        assert!(label.contains("saved/download-failed/save-failed/opener-failed"));
        assert!(label.contains("no FetchMedia"));
        assert!(label.contains("SaveMedia"));
        assert!(label.contains("inline player startup"));
        assert!(label.contains("queue retry/resume/cancel"));
        assert!(label.contains("background download mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_LABEL));
    }

    #[test]
    fn media_save_result_recovery_controls_preview_contains_visible_links() {
        let mut metadata = MediaDownloadActionMetadata::new("Audio");
        metadata.mime_type = Some("audio/ogg".to_string());
        metadata.size_label = Some("42 KiB".to_string());
        metadata.duration_label = Some("3.50 sec".to_string());

        let preview = media_save_result_recovery_controls_preview(
            "Save/Open",
            "voice <draft>.ogg",
            &metadata,
            true,
        );

        assert!(preview.contains("Save/Open recovery"));
        assert!(preview.contains("Open folder"));
        assert!(preview.contains("Replay"));
        assert!(preview.contains("Retry"));
        assert!(preview.contains("Queue"));
        assert!(preview.contains("Decrypt"));
        assert!(preview.contains(MEDIA_RESULT_CONTROL_URL_SCHEME));
        assert!(preview.contains("voice+%3Cdraft%3E.ogg"));
        assert!(preview.contains("voice &lt;draft&gt;.ogg"));
        assert!(!preview.contains("voice <draft>.ogg"));
        assert!(preview.contains("audio/ogg"));
        assert!(preview.contains("42 KiB"));
        assert!(preview.contains("3.50 sec"));
        assert!(preview.contains(MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_LABEL));
        assert!(preview.contains("Retry confirms before SaveMedia"));
        assert!(preview.contains("no unconfirmed FetchMedia"));
        assert!(preview.contains("SaveMedia"));
        assert!(preview.contains("gateway/runtime/auth"));
        assert!(preview.contains("live mutation"));
    }

    #[test]
    fn media_save_result_recovery_controls_preview_can_carry_retry_source() {
        let metadata = MediaDownloadActionMetadata::new("File");

        let preview = media_save_result_recovery_controls_preview_with_source(
            "Download",
            "report.pdf",
            &metadata,
            false,
            Some("mxc://example.org/report"),
        );

        assert!(preview.contains("Retry confirms before SaveMedia"));
        assert!(preview.contains("mxc%3A%2F%2Fexample.org%2Freport"));
        assert!(preview.contains(MEDIA_RESULT_CONTROL_URL_SCHEME));
    }

    #[test]
    fn media_result_control_url_carries_metadata_without_plain_mxc() {
        let mut metadata = MediaDownloadActionMetadata::new("Video");
        metadata.mime_type = Some("video/mp4".to_string());
        metadata.size_label = Some("12 MB".to_string());
        metadata.duration_label = Some("4.20 sec".to_string());
        metadata.dimensions_label = Some("1280x720".to_string());

        let url = media_result_control_url("Decrypt", "Save/Open", "clip.mp4", &metadata, true);

        assert!(url.starts_with(MEDIA_RESULT_CONTROL_URL_SCHEME));
        assert!(url.contains("action=Decrypt"));
        assert!(url.contains("label=Save%2FOpen"));
        assert!(url.contains("name=clip.mp4"));
        assert!(url.contains("open=1"));
        assert!(url.contains("kind=Video"));
        assert!(url.contains("mime=video%2Fmp4"));
        assert!(url.contains("size=12+MB"));
        assert!(url.contains("duration=4.20+sec"));
        assert!(url.contains("dimensions=1280x720"));
        assert!(!url.contains("mxc="));
    }

    #[test]
    fn media_result_control_url_with_source_carries_plain_mxc_for_retry() {
        let metadata = MediaDownloadActionMetadata::new("File");

        let url = media_result_control_url_with_source(
            "Retry",
            "Download",
            "report.pdf",
            &metadata,
            false,
            Some("mxc://example.org/report"),
        );

        assert!(url.starts_with(MEDIA_RESULT_CONTROL_URL_SCHEME));
        assert!(url.contains("action=Retry"));
        assert!(url.contains("mxc=mxc%3A%2F%2Fexample.org%2Freport"));
    }

    #[test]
    fn media_save_preflight_detail_control_label_keeps_actions_local() {
        let label = media_save_preflight_detail_control_label(
            "Source",
            "Save/Open",
            "clip.mp4",
            "Video · clip.mp4 · video/mp4 · 12 MB · 4.20 sec · 1280x720",
            true,
        );

        assert!(label.contains("Media SaveMedia preflight Source"));
        assert!(label.contains("Save/Open"));
        assert!(label.contains("clip.mp4"));
        assert!(label.contains("video/mp4"));
        assert!(label.contains("source metadata is loaded timeline media metadata"));
        assert!(label.contains("system opener"));
        assert!(label.contains("No FetchMedia"));
        assert!(label.contains("no extra SaveMedia"));
        assert!(label.contains("no queue control"));
        assert!(label.contains("no decrypt retry"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_LABEL));
        assert!(MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("Request"));
        assert!(MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("Result"));
        assert!(MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("Error"));
        assert!(MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("Retry"));
        assert!(MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("Source"));
        assert!(
            MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("Retry is a guarded live resubmit")
        );
        assert!(MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("MatrixRequest::SaveMedia"));
        assert!(
            MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("no unconfirmed FetchMedia")
        );
    }

    #[test]
    fn media_save_preflight_detail_controls_preview_contains_visible_links() {
        let mut metadata = MediaDownloadActionMetadata::new("Audio");
        metadata.mime_type = Some("audio/ogg".to_string());
        metadata.size_label = Some("42 KiB".to_string());
        metadata.duration_label = Some("3.50 sec".to_string());

        let preview = media_save_preflight_detail_controls_preview(
            "Save/Open",
            "voice <draft>.ogg",
            &metadata,
            true,
        );

        assert!(preview.contains("SaveMedia preflight"));
        assert!(preview.contains("Request"));
        assert!(preview.contains("Result"));
        assert!(preview.contains("Error"));
        assert!(preview.contains("Retry"));
        assert!(preview.contains("Source"));
        assert!(preview.contains(MEDIA_SAVE_PREFLIGHT_CONTROL_URL_SCHEME));
        assert!(preview.contains("voice+%3Cdraft%3E.ogg"));
        assert!(preview.contains("voice &lt;draft&gt;.ogg"));
        assert!(!preview.contains("voice <draft>.ogg"));
        assert!(preview.contains("audio/ogg"));
        assert!(preview.contains("42 KiB"));
        assert!(preview.contains("3.50 sec"));
        assert!(preview.contains(MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_LABEL));
        assert!(preview.contains("Retry confirms before SaveMedia"));
        assert!(preview.contains("no unconfirmed FetchMedia"));
        assert!(preview.contains("SaveMedia"));
        assert!(preview.contains("gateway/runtime/auth"));
        assert!(preview.contains("live mutation"));
    }

    #[test]
    fn media_save_preflight_detail_controls_preview_can_carry_retry_source() {
        let metadata = MediaDownloadActionMetadata::new("File");

        let preview = media_save_preflight_detail_controls_preview_with_source(
            "Download",
            "report.pdf",
            &metadata,
            false,
            Some("mxc://example.org/report"),
        );

        assert!(preview.contains("Retry confirms before SaveMedia"));
        assert!(preview.contains("mxc%3A%2F%2Fexample.org%2Freport"));
        assert!(preview.contains(MEDIA_SAVE_PREFLIGHT_CONTROL_URL_SCHEME));
    }

    #[test]
    fn media_save_preflight_control_url_carries_metadata_without_plain_mxc() {
        let mut metadata = MediaDownloadActionMetadata::new("Video");
        metadata.mime_type = Some("video/mp4".to_string());
        metadata.size_label = Some("12 MB".to_string());
        metadata.duration_label = Some("4.20 sec".to_string());
        metadata.dimensions_label = Some("1280x720".to_string());

        let url =
            media_save_preflight_control_url("Source", "Save/Open", "clip.mp4", &metadata, true);

        assert!(url.starts_with(MEDIA_SAVE_PREFLIGHT_CONTROL_URL_SCHEME));
        assert!(url.contains("action=Source"));
        assert!(url.contains("label=Save%2FOpen"));
        assert!(url.contains("name=clip.mp4"));
        assert!(url.contains("open=1"));
        assert!(url.contains("kind=Video"));
        assert!(url.contains("mime=video%2Fmp4"));
        assert!(url.contains("size=12+MB"));
        assert!(url.contains("duration=4.20+sec"));
        assert!(url.contains("dimensions=1280x720"));
        assert!(!url.contains("mxc="));
    }

    #[test]
    fn media_save_preflight_control_url_with_source_carries_plain_mxc_for_retry() {
        let metadata = MediaDownloadActionMetadata::new("File");

        let url = media_save_preflight_control_url_with_source(
            "Retry",
            "Download",
            "report.pdf",
            &metadata,
            false,
            Some("mxc://example.org/report"),
        );

        assert!(url.starts_with(MEDIA_SAVE_PREFLIGHT_CONTROL_URL_SCHEME));
        assert!(url.contains("action=Retry"));
        assert!(url.contains("mxc=mxc%3A%2F%2Fexample.org%2Freport"));
    }

    #[test]
    fn media_save_retry_unavailable_label_keeps_encrypted_rows_local() {
        let label = media_save_retry_unavailable_label("Download", "encrypted.bin");

        assert!(label.contains("retry stayed local"));
        assert!(label.contains("no plain MXC source"));
        assert!(label.contains("Retry confirms before SaveMedia only for plain"));
        assert!(label.contains("queue resume/cancel"));
        assert!(label.contains("gateway/runtime/auth"));
    }

    #[test]
    fn encrypted_image_local_metadata_preview_includes_loaded_info_and_boundaries() {
        let mut image_info = ImageInfo::default();
        image_info.mimetype = Some("image/png".to_string());
        image_info.size = Some(uint!(4242));
        image_info.width = Some(uint!(640));
        image_info.height = Some(uint!(480));
        image_info.blurhash = Some("LEHV6nWB2yk8pyo0adR*.7kCMdnj".to_string());

        let preview = encrypted_image_local_metadata_preview("cover.png", &image_info);

        assert!(preview.contains("Loaded encrypted image metadata"));
        assert!(preview.contains("Image"));
        assert!(preview.contains("cover.png"));
        assert!(preview.contains("image/png"));
        assert!(
            preview.contains("4.1")
                || preview.contains("4.2")
                || preview.contains("4 KB")
                || preview.contains("4 KiB")
        );
        assert!(preview.contains("640x480"));
        assert!(preview.contains("blurhash loaded"));
        assert!(preview.contains("thumbnail source missing"));
        assert!(preview.contains(MEDIA_ENCRYPTED_IMAGE_METADATA_LOCAL_LABEL));
        assert!(preview.contains("No decrypt"));
        assert!(preview.contains("SaveMedia"));
        assert!(preview.contains("FetchMedia"));
        assert!(preview.contains("image decode"));
        assert!(preview.contains("gateway/runtime/auth"));
        assert!(preview.contains("live mutation"));
    }
}

/// Draws the given location message's content into the `message_content_widget`.
///
/// Returns whether the location message content was fully drawn.
fn populate_location_message_content(
    cx: &mut Cx,
    message_content_widget: &HtmlOrPlaintextRef,
    location: &LocationMessageEventContent,
) -> bool {
    let coords = location
        .geo_uri
        .get(utils::GEO_URI_SCHEME.len()..)
        .and_then(|s| {
            let mut iter = s.split(',');
            if let (Some(lat), Some(long)) = (iter.next(), iter.next()) {
                Some((lat, long))
            } else {
                None
            }
        });
    if let Some((lat, long)) = coords {
        let short_lat = lat
            .find('.')
            .and_then(|dot| lat.get(..dot + 7))
            .unwrap_or(lat);
        let short_long = long
            .find('.')
            .and_then(|dot| long.get(..dot + 7))
            .unwrap_or(long);
        let safe_lat = htmlize::escape_attribute(lat);
        let safe_long = htmlize::escape_attribute(long);
        let safe_geo_uri = htmlize::escape_attribute(&location.geo_uri);
        let safe_short_lat = htmlize::escape_text(short_lat);
        let safe_short_long = htmlize::escape_text(short_long);
        let html_body = format!(
            "Location: <a href=\"{}\">{safe_short_lat},{safe_short_long}</a><br>\
            <ul>\
            <li><a href=\"https://www.openstreetmap.org/?mlat={safe_lat}&amp;mlon={safe_long}#map=15/{safe_lat}/{safe_long}\">Open in OpenStreetMap</a></li>\
            <li><a href=\"https://www.google.com/maps/search/?api=1&amp;query={safe_lat},{safe_long}\">Open in Google Maps</a></li>\
            <li><a href=\"https://maps.apple.com/?ll={safe_lat},{safe_long}&amp;q={safe_lat},{safe_long}\">Open in Apple Maps</a></li>\
            </ul>",
            safe_geo_uri,
        );
        message_content_widget.show_html(cx, html_body);
    } else {
        message_content_widget.show_html(
            cx,
            format!(
                "<i>[Location invalid]</i> {}",
                htmlize::escape_text(&location.body)
            ),
        );
    }

    // Currently we do not fetch location thumbnail previews, so we consider this as fully drawn.
    // In the future, when we do support this, we'll return false until the thumbnail is fetched,
    // at which point we can return true.
    true
}

/// Draws the given redacted message's content into the `message_content_widget`.
///
/// Returns whether the redacted message content was fully drawn.
fn populate_redacted_message_content(
    cx: &mut Cx,
    message_content_widget: &HtmlOrPlaintextRef,
    event_tl_item: &EventTimelineItem,
    room_id: &OwnedRoomId,
) -> bool {
    let fully_drawn: bool;
    let mut redactor_id_and_reason = None;
    if let Some(redacted_msg) = event_tl_item.latest_json() {
        if let Ok(AnySyncTimelineEvent::MessageLike(AnySyncMessageLikeEvent::RoomMessage(
            SyncMessageLikeEvent::Redacted(redaction),
        ))) = redacted_msg.deserialize()
        {
            if let Ok(redacted_because) = redaction.unsigned.redacted_because.deserialize() {
                let reason = match &redacted_because {
                    AnyRedactionEvent::RoomRedaction(event) => event.content.reason.clone(),
                    _ => None,
                };
                redactor_id_and_reason =
                    Some((redacted_because.sender().to_owned(), reason));
            }
        }
    }

    let html = if let Some((redactor, reason)) = redactor_id_and_reason {
        if redactor == event_tl_item.sender() {
            fully_drawn = true;
            match reason {
                Some(r) => format!(
                    "⛔ <i>Deleted their own message. Reason: \"{}\".</i>",
                    htmlize::escape_text(r)
                ),
                None => String::from("⛔ <i>Deleted their own message.</i>"),
            }
        } else {
            // Try to get the displayable name of the user who redacted this message.
            let redactor_name = user_profile_cache::get_user_display_name_for_room(
                cx,
                redactor.clone(),
                Some(room_id),
                true,
            );
            fully_drawn = redactor_name.was_found();
            let redactor_name_esc =
                htmlize::escape_text(redactor_name.as_deref().unwrap_or(redactor.as_str()));
            match reason {
                Some(r) => format!(
                    "⛔ <i>{} deleted this message. Reason: \"{}\".</i>",
                    redactor_name_esc,
                    htmlize::escape_text(r),
                ),
                None => format!("⛔ <i>{} deleted this message.</i>", redactor_name_esc),
            }
        }
    } else {
        fully_drawn = true;
        String::from("⛔ <i>Message deleted.</i>")
    };
    message_content_widget.show_html(cx, html);
    fully_drawn
}

/// Draws a ReplyPreview above a message if it was in-reply to another message.
///
/// ## Arguments
/// * `replied_to_message_view`: the destination `RepliedToMessage` view that will be populated.
/// * `timeline_kind`: the [`TimelineKind`] of the timeline that is being drawn.
/// * `in_reply_to`: if `Some`, the details that will be used to populate the `replied_to_message_view`.
///   If `None`, this function will mark it as non-visible and consider it fully drawn.
/// * `message_event_id`: the [`EventId`] of the message that is the reply itself (the response).
///   This is needed to fetch the details of the replied-to message (if not yet available).
///
/// Returns whether the in-reply-to information was available and fully drawn,
/// i.e., whether it can be considered cached and not needing to be redrawn later.
fn draw_replied_to_message(
    cx: &mut Cx2d,
    replied_to_message_view: &ViewRef,
    timeline_kind: &TimelineKind,
    in_reply_to: Option<&InReplyToDetails>,
    message_event_id: Option<&EventId>,
) -> bool {
    let fully_drawn: bool;
    let show_reply: bool;

    if let Some(in_reply_to_details) = in_reply_to {
        show_reply = true;
        match &in_reply_to_details.event {
            TimelineDetails::Ready(replied_to_event) => {
                let (in_reply_to_username, is_avatar_fully_drawn) = replied_to_message_view
                    .avatar(cx, ids!(replied_to_message_content.reply_preview_avatar))
                    .set_avatar_and_get_username(
                        cx,
                        timeline_kind,
                        &replied_to_event.sender,
                        Some(&replied_to_event.sender_profile),
                        Some(in_reply_to_details.event_id.as_ref()),
                        true,
                    );

                fully_drawn = is_avatar_fully_drawn;

                replied_to_message_view
                    .label(cx, ids!(replied_to_message_content.reply_preview_username))
                    .set_text(cx, in_reply_to_username.as_str());
                let msg_body =
                    replied_to_message_view.html_or_plaintext(cx, ids!(reply_preview_body));
                populate_preview_of_timeline_item(
                    cx,
                    &msg_body,
                    &replied_to_event.content,
                    &replied_to_event.sender,
                    &in_reply_to_username,
                );
            }
            TimelineDetails::Error(_e) => {
                fully_drawn = true;
                replied_to_message_view
                    .label(cx, ids!(replied_to_message_content.reply_preview_username))
                    .set_text(cx, "[Error fetching username]");
                replied_to_message_view
                    .avatar(cx, ids!(replied_to_message_content.reply_preview_avatar))
                    .show_text(cx, None, None, "?");
                replied_to_message_view
                    .html_or_plaintext(cx, ids!(replied_to_message_content.reply_preview_body))
                    .show_plaintext(cx, "[Error fetching replied-to event from read path]");
            }
            td @ TimelineDetails::Pending | td @ TimelineDetails::Unavailable => {
                // We don't have the replied-to message yet, so we can't fully draw the preview.
                fully_drawn = false;
                replied_to_message_view
                    .label(cx, ids!(replied_to_message_content.reply_preview_username))
                    .set_text(cx, "[Loading username...]");
                replied_to_message_view
                    .avatar(cx, ids!(replied_to_message_content.reply_preview_avatar))
                    .show_text(cx, None, None, "?");
                replied_to_message_view
                    .html_or_plaintext(cx, ids!(replied_to_message_content.reply_preview_body))
                    .show_plaintext(
                        cx,
                        "[Loading replied-to message via event-details read path...]",
                    );

                // Confusingly, we need to fetch the details of the `message` (the event that is the reply),
                // not the details of the original event that this `message` is replying to.
                if matches!(td, TimelineDetails::Unavailable) {
                    if let Some(event_id) = message_event_id {
                        submit_async_request(MatrixRequest::FetchDetailsForEvent {
                            timeline_kind: timeline_kind.clone(),
                            event_id: event_id.to_owned(),
                        });
                    }
                }
            }
        }
    } else {
        // This message was not in reply to another message, so we don't need to show a reply.
        show_reply = false;
        fully_drawn = true;
    }

    replied_to_message_view.set_visible(cx, show_reply);
    fully_drawn
}

/// Draws a one-line thread summary at the bottom of a message if it is the root of a thread.
///
/// Returns whether the thread summary information was available and fully drawn,
/// i.e., whether it can be considered cached and not needing to be redrawn later.
fn populate_thread_root_summary(
    cx: &mut Cx2d,
    item: &WidgetRef,
    timeline_item_index: usize,
    timeline_kind: &TimelineKind,
    msg_like_content: &MsgLikeContent,
    event_tl_item: &EventTimelineItem,
    fetched_thread_summaries: &HashMap<OwnedEventId, FetchedThreadSummary>,
    pending_thread_summary_fetches: &mut HashSet<OwnedEventId>,
) -> bool {
    let thread_summary_view = item.view(cx, ids!(thread_root_summary));
    thread_summary_view.set_visible(cx, false); // hide by default
    let fully_drawn: bool;

    if matches!(timeline_kind, TimelineKind::Thread { .. }) {
        // If we're already drawing a message in a thread-focused timeline,
        // it doesn't make sense to show a redundant thread summary.
        fully_drawn = true;
        return fully_drawn;
    }

    let Some(thread_summary) = msg_like_content.thread_summary.as_ref() else {
        // consider this as fully drawn since there's no thread summary to show.
        fully_drawn = true;
        return fully_drawn;
    };

    // Here, we actually need to show the thread summary.
    thread_summary_view.set_visible(cx, true);
    let local_num_replies = thread_summary.num_replies;
    let thread_root_event_id = event_tl_item.event_id().map(|id| id.to_owned());
    let fetched_summary = thread_root_event_id
        .as_ref()
        .and_then(|root_id| fetched_thread_summaries.get(root_id));
    let replies_count = fetched_summary
        .map(|f| f.num_replies)
        .unwrap_or(local_num_replies);

    let latest_preview: Cow<str> = match &thread_summary.latest_event {
        TimelineDetails::Ready(embedded_event) => {
            fully_drawn = true;
            let sender_username = match &embedded_event.sender_profile {
                TimelineDetails::Ready(profile) => profile
                    .display_name
                    .as_deref()
                    .unwrap_or(embedded_event.sender.as_str()),
                _ => embedded_event.sender.as_str(),
            };
            let preview = text_preview_of_timeline_item(
                &embedded_event.content,
                &embedded_event.sender,
                sender_username,
            )
            .format_with(sender_username, true);
            match utils::replace_linebreaks_separators(&preview, true) {
                Cow::Borrowed(_) => Cow::Owned(preview),
                Cow::Owned(replaced) => Cow::Owned(replaced),
            }
        }
        td @ TimelineDetails::Pending | td @ TimelineDetails::Unavailable => {
            fully_drawn = true;
            if td.is_unavailable()
                && let Some(thread_root_event_id) = thread_root_event_id.clone()
            {
                let needs_refresh =
                    fetched_summary.is_none_or(|fs| fs.latest_reply_preview_text.is_none());
                if needs_refresh
                    && pending_thread_summary_fetches.insert(thread_root_event_id.clone())
                {
                    submit_async_request(MatrixRequest::FetchThreadSummaryDetails {
                        timeline_kind: timeline_kind.clone(),
                        thread_root_event_id,
                        timeline_item_index,
                    });
                }
            }
            fetched_summary
                .and_then(|fs| fs.latest_reply_preview_text.as_deref())
                .unwrap_or("<i>Loading latest reply...</i>")
                .into()
        }
        TimelineDetails::Error(_) => {
            fully_drawn = true; // consider this fully drawn since there's no point retrying.
            "<i>Unable to load latest reply</i>".into()
        }
    };

    let replies_count_text = match replies_count {
        1 => Cow::Borrowed("1 reply"),
        n => Cow::Owned(format!("{n} replies")),
    };
    item.label(cx, ids!(thread_summary_row.thread_summary_count))
        .set_text(cx, &replies_count_text);
    item.html(cx, ids!(thread_summary_row.thread_summary_latest))
        .set_text(cx, &latest_preview);
    fully_drawn
}

/// Generates a rich HTML text preview of the given `timeline_item_content`
/// and populates the given `widget_out` with that content.
pub fn populate_preview_of_timeline_item(
    cx: &mut Cx,
    widget_out: &HtmlOrPlaintextRef,
    timeline_item_content: &TimelineItemContent,
    sender_user_id: &UserId,
    sender_username: &str,
) {
    if let Some(m) = timeline_item_content.as_message() {
        match m.msgtype() {
            MessageType::Text(TextMessageEventContent {
                body, formatted, ..
            })
            | MessageType::Notice(NoticeMessageEventContent {
                body, formatted, ..
            }) => {
                let _ = populate_text_message_content(
                    cx,
                    widget_out,
                    body,
                    formatted.as_ref(),
                    None,
                    None,
                    None,
                    None,
                );
                return;
            }
            _ => {} // fall through to the general case for all timeline items below.
        }
    }
    let html =
        text_preview_of_timeline_item(timeline_item_content, sender_user_id, sender_username)
            .format_with(sender_username, true);
    widget_out.show_html(cx, html);
}

/// A trait for abstracting over the different types of timeline events
/// that can be displayed in a `SmallStateEvent` widget.
trait SmallStateEventContent {
    /// Populates the *content* (not the profile) of the given `item` with data from
    /// the given `event_tl_item` and `self` (the specific type of event content).
    ///
    /// ## Arguments
    /// * `item`: a `SmallStateEvent` widget that has already been added to
    ///   the given `PortalList` at the given `item_id`.
    ///   This function may either modify that item or completely replace it
    ///   with a different widget if needed.
    /// * `item_drawn_status`: the old (prior) drawn status of the item.
    /// * `new_drawn_status`: the new drawn status of the item, which may have already
    ///   been updated to reflect the item's profile having been drawn right before this function.
    ///
    /// ## Return
    /// Returns a tuple of the drawn `item` and its `new_drawn_status`.
    fn populate_item_content(
        &self,
        cx: &mut Cx,
        list: &mut PortalList,
        item_id: usize,
        item: WidgetRef,
        event_tl_item: &EventTimelineItem,
        username: &str,
        item_drawn_status: ItemDrawnStatus,
        new_drawn_status: ItemDrawnStatus,
    ) -> (WidgetRef, ItemDrawnStatus);
}

// For unable to decrypt messages.
impl SmallStateEventContent for EncryptedMessage {
    fn populate_item_content(
        &self,
        cx: &mut Cx,
        _list: &mut PortalList,
        _item_id: usize,
        item: WidgetRef,
        _event_tl_item: &EventTimelineItem,
        username: &str,
        _item_drawn_status: ItemDrawnStatus,
        mut new_drawn_status: ItemDrawnStatus,
    ) -> (WidgetRef, ItemDrawnStatus) {
        item.label(cx, ids!(content)).set_text(
            cx,
            &text_preview_of_encrypted_message(self).format_with(username, false),
        );
        new_drawn_status.content_drawn = true;
        (item, new_drawn_status)
    }
}

// For other message-like content (custom message-like events).
impl SmallStateEventContent for LiveLocationState {
    fn populate_item_content(
        &self,
        cx: &mut Cx,
        _list: &mut PortalList,
        _item_id: usize,
        item: WidgetRef,
        _event_tl_item: &EventTimelineItem,
        username: &str,
        _item_drawn_status: ItemDrawnStatus,
        mut new_drawn_status: ItemDrawnStatus,
    ) -> (WidgetRef, ItemDrawnStatus) {
        item.label(cx, ids!(content))
            .set_text(cx, &format!("{username} shared a live location."));
        new_drawn_status.content_drawn = true;
        (item, new_drawn_status)
    }
}

impl SmallStateEventContent for OtherMessageLike {
    fn populate_item_content(
        &self,
        cx: &mut Cx,
        _list: &mut PortalList,
        _item_id: usize,
        item: WidgetRef,
        _event_tl_item: &EventTimelineItem,
        username: &str,
        _item_drawn_status: ItemDrawnStatus,
        mut new_drawn_status: ItemDrawnStatus,
    ) -> (WidgetRef, ItemDrawnStatus) {
        item.label(cx, ids!(content)).set_text(
            cx,
            &text_preview_of_other_message_like(self).format_with(username, false),
        );
        new_drawn_status.content_drawn = true;
        (item, new_drawn_status)
    }
}

impl SmallStateEventContent for timeline::OtherState {
    fn populate_item_content(
        &self,
        cx: &mut Cx,
        list: &mut PortalList,
        item_id: usize,
        item: WidgetRef,
        _event_tl_item: &EventTimelineItem,
        username: &str,
        _item_drawn_status: ItemDrawnStatus,
        mut new_drawn_status: ItemDrawnStatus,
    ) -> (WidgetRef, ItemDrawnStatus) {
        let item = if let Some(text_preview) = text_preview_of_other_state(self, false) {
            item.label(cx, ids!(content))
                .set_text(cx, &text_preview.format_with(username, false));
            new_drawn_status.content_drawn = true;
            item
        } else {
            let item = list.item(cx, item_id, id!(Empty));
            new_drawn_status = ItemDrawnStatus::new();
            item
        };
        (item, new_drawn_status)
    }
}

impl SmallStateEventContent for MemberProfileChange {
    fn populate_item_content(
        &self,
        cx: &mut Cx,
        _list: &mut PortalList,
        _item_id: usize,
        item: WidgetRef,
        _event_tl_item: &EventTimelineItem,
        username: &str,
        _item_drawn_status: ItemDrawnStatus,
        mut new_drawn_status: ItemDrawnStatus,
    ) -> (WidgetRef, ItemDrawnStatus) {
        item.label(cx, ids!(content)).set_text(
            cx,
            &text_preview_of_member_profile_change(self, username, false)
                .format_with(username, false),
        );
        new_drawn_status.content_drawn = true;
        (item, new_drawn_status)
    }
}

impl SmallStateEventContent for RoomMembershipChange {
    fn populate_item_content(
        &self,
        cx: &mut Cx,
        list: &mut PortalList,
        item_id: usize,
        item: WidgetRef,
        _event_tl_item: &EventTimelineItem,
        username: &str,
        _item_drawn_status: ItemDrawnStatus,
        mut new_drawn_status: ItemDrawnStatus,
    ) -> (WidgetRef, ItemDrawnStatus) {
        let Some(preview) = text_preview_of_room_membership_change(self, false) else {
            // Don't actually display anything for nonexistent/unimportant membership changes.
            return (list.item(cx, item_id, id!(Empty)), ItemDrawnStatus::new());
        };

        item.label(cx, ids!(content))
            .set_text(cx, &preview.format_with(username, false));

        // The invite_user_button is only used for "Knocked" membership change events.
        item.button(cx, ids!(invite_user_button))
            .set_visible(cx, matches!(self.change(), Some(MembershipChange::Knocked)));

        new_drawn_status.content_drawn = true;
        (item, new_drawn_status)
    }
}

/// Creates and populates a first-class Hepta event card for custom `m.hepta.*`
/// message-like events. The Matrix SDK still owns the timeline item; this only
/// changes the visible Robrix-derived renderer.
fn populate_hepta_event_card(
    cx: &mut Cx,
    list: &mut PortalList,
    item_id: usize,
    event_tl_item: &EventTimelineItem,
    event_content: &OtherMessageLike,
    item_drawn_status: ItemDrawnStatus,
) -> (WidgetRef, ItemDrawnStatus) {
    let (item, existed) = list.item_with_existed(cx, item_id, id!(HeptaEventCard));
    if existed && item_drawn_status.content_drawn {
        return (item, item_drawn_status);
    }

    let event_type = event_content.event_type().to_string();
    let envelope = hepta_envelope_from_timeline_item(event_tl_item);
    let card = card_text_for_event(&event_type, envelope.as_ref());
    let show_approval_actions = envelope
        .as_ref()
        .is_some_and(|envelope| envelope.event_kind == "approval_request");
    let sender = get_profile_display_name(event_tl_item)
        .unwrap_or_else(|| event_tl_item.sender().to_string());

    item.label(cx, ids!(header.eyebrow))
        .set_text(cx, &card.eyebrow);
    item.label(cx, ids!(header.status))
        .set_text(cx, &card.status);
    item.label(cx, ids!(title)).set_text(cx, &card.title);
    item.label(cx, ids!(body)).set_text(cx, &card.body);
    item.label(cx, ids!(meta))
        .set_text(cx, &format!("{} · {}", sender, card.meta));
    item.label(cx, ids!(policy))
        .set_text(cx, &hepta_policy_badge_text(envelope.as_ref()));
    item.view(cx, ids!(hepta_actions)).set_visible(cx, true);
    item.button(cx, ids!(hepta_actions.approve_button))
        .set_visible(cx, show_approval_actions);
    item.button(cx, ids!(hepta_actions.reject_button))
        .set_visible(cx, show_approval_actions);

    (item, ItemDrawnStatus::both_drawn())
}

fn hepta_envelope_from_timeline_item(
    event_tl_item: &EventTimelineItem,
) -> Option<HeptaEventEnvelope> {
    event_tl_item
        .latest_json()
        .and_then(|raw| raw.get_field::<serde_json::Value>("content").ok())
        .flatten()
        .and_then(|content| HeptaEventEnvelope::from_content_value(&content).ok())
}

fn hepta_policy_badge_text(envelope: Option<&HeptaEventEnvelope>) -> String {
    let Some(envelope) = envelope else {
        return "policy: custom event · payload inspection available".to_string();
    };
    if let Some(bridge_policy) = envelope.payload.get("bridge_policy") {
        let mutation_class = bridge_policy
            .get("mutation_class")
            .and_then(|value| value.as_str())
            .unwrap_or("unknown_mutation_class");
        let disposition = bridge_policy
            .get("disposition")
            .and_then(|value| value.as_str())
            .unwrap_or("preview");
        return format!(
            "policy: {mutation_class} · {disposition} · external_mutation_enabled=false"
        );
    }
    match envelope.event_kind.as_str() {
        "approval_request" => {
            "policy: approval_request · exact confirmation required · live mutation blocked"
                .to_string()
        }
        "tool_call" | "tool_result" => {
            "policy: tool evidence · inspect payload · secrets redacted".to_string()
        }
        "task" | "agent_run" => "policy: progress event · read-only timeline evidence".to_string(),
        "runtime_event" => "policy: runtime event · read-only status".to_string(),
        _ => "policy: Hepta event · inspectable local payload".to_string(),
    }
}

/// Creates, populates, and adds a SmallStateEvent liveview widget to the given `PortalList`
/// with the given `item_id`.
///
/// The content of the returned widget is populated with data from the
/// given room membership change and its parent `EventTimelineItem`.
fn populate_small_state_event(
    cx: &mut Cx,
    list: &mut PortalList,
    item_id: usize,
    timeline_kind: &TimelineKind,
    event_tl_item: &EventTimelineItem,
    event_content: &impl SmallStateEventContent,
    item_drawn_status: ItemDrawnStatus,
) -> (WidgetRef, ItemDrawnStatus) {
    let mut new_drawn_status = item_drawn_status;
    let (item, existed) = list.item_with_existed(cx, item_id, id!(SmallStateEvent));
    // The content of a small state event view may depend on the profile info,
    // so we can only mark the content as drawn after the profile has been fully drawn and cached.
    let skip_redrawing_profile = existed && item_drawn_status.profile_drawn;
    let skip_redrawing_content = skip_redrawing_profile && item_drawn_status.content_drawn;
    populate_read_receipts(&item, cx, timeline_kind, event_tl_item);
    if skip_redrawing_content {
        return (item, new_drawn_status);
    }

    // If the profile has been drawn, we can just quickly grab the user's display name
    // instead of having to call `set_avatar_and_get_username` again.
    let username_opt = skip_redrawing_profile
        .then(|| get_profile_display_name(event_tl_item))
        .flatten();

    let username = username_opt.unwrap_or_else(|| {
        // As a fallback, call `set_avatar_and_get_username` to get the user's display name.
        let avatar_ref = item.avatar(cx, ids!(avatar));

        let (username, profile_drawn) = avatar_ref.set_avatar_and_get_username(
            cx,
            timeline_kind,
            event_tl_item.sender(),
            Some(event_tl_item.sender_profile()),
            event_tl_item.event_id(),
            true,
        );
        // Draw the timestamp as part of the profile.
        if let Some(dt) = unix_time_millis_to_datetime(event_tl_item.timestamp()) {
            item.timestamp(cx, ids!(left_container.timestamp))
                .set_date_time(cx, dt);
        }
        new_drawn_status.profile_drawn = profile_drawn;
        username
    });

    // Proceed to draw the actual event content.
    event_content.populate_item_content(
        cx,
        list,
        item_id,
        item,
        event_tl_item,
        &username,
        item_drawn_status,
        new_drawn_status,
    )
}

/// Returns the display name of the sender of the given `event_tl_item`, if available.
fn get_profile_display_name(event_tl_item: &EventTimelineItem) -> Option<String> {
    if let TimelineDetails::Ready(profile) = event_tl_item.sender_profile() {
        profile.display_name.clone()
    } else {
        None
    }
}

/// Actions related to invites within a room.
///
/// These are NOT widget actions, just regular actions.
#[derive(Debug)]
pub enum InviteAction {
    /// Show a confirmation modal for sending an invite.
    ///
    /// The content is wrapped in a `RefCell` to ensure that only one entity handles it
    /// and that that one entity can take ownership of the content object,
    /// which avoids having to clone it.
    ShowInviteConfirmationModal(RefCell<Option<ConfirmationModalContent>>),
}

/// The result of inviting a user to a room.
///
#[derive(Debug)]
pub enum InviteResultAction {
    /// The invite was sent successfully.
    ///
    /// This action is posted in response to the [`MatrixRequest::InviteUser`] request.
    Sent {
        room_id: OwnedRoomId,
        user_id: OwnedUserId,
    },
    /// The invite failed to be sent.
    ///
    /// This action is posted in response to the [`MatrixRequest::InviteUser`] request.
    Failed {
        room_id: OwnedRoomId,
        user_id: OwnedUserId,
        error: matrix_sdk::Error,
    },
}

/// Actions posted by the Matrix link Join confirmation modal.
#[derive(Clone, Debug)]
pub enum MatrixLinkJoinAction {
    Submitted {
        room_or_alias_id: OwnedRoomOrAliasId,
        target: String,
        via_count: usize,
        event_id_label: String,
    },
    Canceled {
        target: String,
    },
}

/// Actions posted by the Matrix link Knock confirmation modal.
#[derive(Clone, Debug)]
pub enum MatrixLinkKnockAction {
    Submitted {
        room_or_alias_id: OwnedRoomOrAliasId,
        target: String,
        via_count: usize,
        event_id_label: String,
    },
    Canceled {
        target: String,
    },
}

/// Actions posted by the Matrix link Invite confirmation modal.
#[derive(Clone, Debug)]
pub enum MatrixLinkInviteAction {
    Submitted {
        room_id: OwnedRoomId,
        user_id: OwnedUserId,
        target: String,
        via_count: usize,
    },
    Canceled {
        target: String,
    },
}

/// Actions posted by the Matrix link room id or alias join worker.
#[derive(Debug)]
pub enum MatrixLinkJoinResultAction {
    Joined {
        room_or_alias_id: OwnedRoomOrAliasId,
        server_names: Vec<OwnedServerName>,
        room_id: OwnedRoomId,
    },
    Failed {
        room_or_alias_id: OwnedRoomOrAliasId,
        server_names: Vec<OwnedServerName>,
        error: matrix_sdk::Error,
    },
}

/// Actions related to a specific message within a room timeline.
#[derive(Clone, Default, Debug)]
pub enum MessageAction {
    /// The user clicked the "react" button on a message
    /// and wants to send the given `reaction` to that message.
    React {
        details: MessageDetails,
        reaction: String,
    },
    /// The user clicked the "reply" button on a message.
    Reply(MessageDetails),
    /// The user clicked the "reply in thread" button on a message.
    ReplyInThread(MessageDetails),
    /// The user clicked the "edit" button on a message.
    Edit(MessageDetails),
    /// The user requested to edit their latest message in this room.
    EditLatest,
    /// The user clicked the "pin" button on a message.
    Pin(MessageDetails),
    /// The user clicked the "unpin" button on a message.
    Unpin(MessageDetails),
    /// The user clicked the "copy text" button on a message.
    CopyText(MessageDetails),
    /// The user clicked the "copy HTML" button on a message.
    CopyHtml(MessageDetails),
    /// The user clicked the "copy link" button on a message.
    CopyLink(MessageDetails),
    /// The user clicked the "view source" button on a message.
    ViewSource(MessageDetails),
    /// The user clicked the "jump to related" button on a message,
    /// indicating that they want to auto-scroll back to the related message,
    /// e.g., a replied-to message.
    JumpToRelated(MessageDetails),
    /// The user clicked the thread summary on a thread-root message.
    OpenThread(OwnedEventId),
    /// The user requested to jump to a specific event in this room.
    JumpToEvent(OwnedEventId),
    /// The user clicked the "delete" button on a message.
    #[doc(alias("delete"))]
    Redact {
        details: MessageDetails,
        reason: Option<String>,
    },
    /// The user confirmed reporting a message.
    Report {
        details: MessageDetails,
        reason: String,
    },
    /// The user confirmed retrying a previously failed report.
    RetryReport {
        event_id: OwnedEventId,
        reason: String,
    },
    /// The user confirmed canceling a pending local echo send queue item.
    CancelLocalSend(MessageDetails),
    /// The message at the given item index in the timeline should be highlighted.
    HighlightMessage(usize),
    /// The user requested that we show a context menu with actions
    /// that can be performed on a given message.
    OpenMessageContextMenu {
        details: MessageDetails,
        /// The absolute position where we should show the context menu,
        /// in which the (0,0) origin coordinate is the top left corner of the app window.
        abs_pos: DVec2,
    },
    /// The user requested opening the message action bar
    ActionBarOpen {
        /// At the given timeline item index
        item_id: usize,
        /// The message rect, so the action bar can be positioned relative to it
        message_rect: Rect,
    },
    /// The user requested closing the message action bar
    ActionBarClose,
    #[default]
    None,
}
