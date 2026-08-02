use super::*;

/// Draws the Html or plaintext body of the given Text or Notice message into the `message_content_widget`.
/// Also populates link previews if a link_preview_ref is provided.
///
/// Returns whether the text items were fully drawn.
pub(super) fn populate_text_message_content(
    cx: &mut Cx,
    message_content_widget: &HtmlOrPlaintextRef,
    body: &str,
    formatted_body: Option<&FormattedBody>,
    room_mention_room_id: Option<&OwnedRoomId>,
    link_preview_ref: Option<&mut LinkPreviewRef>,
    media_cache: Option<&mut MediaCache>,
    link_preview_cache: Option<&mut LinkPreviewCache>,
) -> bool {
    /// If this is a room mention, replace `@room` text in `html` with a pill
    /// link to the room so it renders as a red room pill with the room's avatar.
    fn apply_room_mention<'a>(html: Cow<'a, str>, room_id: Option<&OwnedRoomId>) -> Cow<'a, str> {
        if let Some(room_id) = room_id {
            // Only replace @room if it's NOT already inside an <a> tag
            // (some clients pre-link @room in the formatted_body).
            if html.contains("@room") && !html.contains("\">@room</a>") {
                return Cow::Owned(html.replace(
                    "@room",
                    &format!("<a href=\"https://matrix.to/#/{room_id}\">@room</a>"),
                ));
            }
        }
        html
    }

    // The message was HTML-formatted rich text.
    let mut links = Vec::new();
    if let Some(fb) = formatted_body.as_ref()
        .and_then(|fb| (fb.format == MessageFormat::Html).then_some(fb))
    {
        let linkified_html = utils::linkify_get_urls(
            utils::trim_start_html_whitespace(&fb.body),
            true,
            Some(&mut links),
        );
        let html = apply_room_mention(linkified_html, room_mention_room_id);
        message_content_widget.show_html(cx, html);
    }
    // The message was non-HTML plaintext.
    else {
        let linkified_html = utils::linkify_get_urls(body, false, Some(&mut links));
        let html = apply_room_mention(linkified_html, room_mention_room_id);
        match html {
            Cow::Owned(linkified_html) => message_content_widget.show_html(cx, &linkified_html),
            Cow::Borrowed(plaintext) => message_content_widget.show_plaintext(cx, plaintext),
        }
    };

    // Populate link previews if all required parameters are provided
    if let (Some(link_preview_ref), Some(media_cache), Some(link_preview_cache)) =
        (link_preview_ref, media_cache, link_preview_cache)
    {
        link_preview_ref.populate_below_message(
            cx,
            &links,
            media_cache,
            link_preview_cache,
            &populate_image_message_content,
        )
    } else {
        true
    }
}


/// Populates the caption (and makes its view visible) for the given message `item`.
///
/// Prefers the formatted caption (HTML), with an optional plaintext caption as fallback.
pub(super) fn populate_media_caption(
    cx: &mut Cx,
    item: &WidgetRef,
    formatted_caption: Option<&FormattedBody>,
    backup_caption: Option<&str>,
) {
    let caption_view = item.view(cx, ids!(content.message.caption_view));
    let caption_ref = item.html_or_plaintext(cx, ids!(content.message.caption_view.caption));
    let should_show = if let Some(fb) = formatted_caption
        .filter(|fb| fb.format == MessageFormat::Html && !fb.body.trim().is_empty())
    {
        caption_ref.show_html(cx, &fb.body);
        true
    } else if let Some(text) = backup_caption.filter(|c| !c.trim().is_empty()) {
        caption_ref.show_plaintext(cx, text);
        true
    } else {
        false
    };
    caption_view.set_visible(cx, should_show);
}

/// Like `populate_image_message_content`, but also returns metadata
/// about how to download the image if we were unable to show a preview of it.
pub(super) fn populate_image_message_content_with_fallback(
    cx: &mut Cx,
    text_or_image_ref: &TextOrImageRef,
    image_info_source: Option<&ImageInfo>,
    original_source: MediaSource,
    body: &str,
    media_cache: &mut MediaCache,
    filename: &str,
    size: Option<u64>,
    kind: DownloadKind,
) -> (bool, Option<DownloadableAttachment>) {
    let fully_drawn = populate_image_message_content(
        cx,
        text_or_image_ref,
        image_info_source,
        original_source.clone(),
        body,
        media_cache,
    );
    let fallback = text_or_image_ref.status().is_text().then(|| DownloadableAttachment {
        media_source: original_source,
        filename: filename.to_owned(),
        size,
        kind,
    });
    (fully_drawn, fallback)
}

/// Draws an image into the given `text_or_image_ref`.
///
/// Returns whether it was fully drawn (meaning its content was fully loaded/available).
fn populate_image_message_content(
    cx: &mut Cx,
    text_or_image_ref: &TextOrImageRef,
    image_info_source: Option<&ImageInfo>,
    original_source: MediaSource,
    body: &str,
    media_cache: &mut MediaCache,
) -> bool {
    let (mimetype, _width, _height) = image_info_source
        .map(|info| (info.mimetype.as_deref(), info.width, info.height))
        .unwrap_or_default();

    // If the mimetype is known but isn't an image format makepad can decode,
    // show a message that it's unsupported.
    if let Some(mime) = mimetype.as_ref() {
        if !utils::is_supported_image_mimetype(mime) {
            text_or_image_ref.show_text(
                cx,
                format!("{}{}Unsupported type {:?}",
                    body,
                    if body.trim().is_empty() { "" } else { "\n" },
                    mime,
                ),
            );
            return true; // consider this as fully drawn
        }
    }

    let mut fully_drawn = false;

    // Fall back to fetching the full-size image instead of a failed thumbnail if it's not too big.
    const MAX_FULL_IMAGE_SIZE: u64 = 1024 * 1024; // 1MiB
    let should_fetch_full_size = image_info_source
        .and_then(|info| info.size)
        .is_none_or(|size| u64::from(size) <= MAX_FULL_IMAGE_SIZE);

    let mut fetch_and_show_media_source = |cx: &mut Cx, media_source: MediaSource, image_info: &ImageInfo| {
        match media_cache.try_get_media_or_fetch(&media_source, MEDIA_THUMBNAIL_FORMAT.into()) {
            (MediaCacheEntry::Loaded(data), media_format) => {
                // Include the file type (full or thumbnail) in the cache key to disambiguate.
                let variant = if matches!(media_format, MediaFormat::File) { "full" } else { "thumb" };
                let cache_key = format!("{}#{variant}", media_source_mxc(&media_source));
                let show_image_result = text_or_image_ref.show_image(cx, Some(media_source), |cx, img| {
                    utils::load_image_with_cache_key(&img, cx, std::path::Path::new(&cache_key), Arc::clone(&data))
                        .map(|()| img.size_in_pixels(cx).unwrap_or_default())
                });
                if let Err(e) = show_image_result {
                    let err_str = format!("{body}\n\nFailed to display image: {e:?}");
                    error!("{err_str}");
                    text_or_image_ref.show_text(cx, &err_str);
                }

                // We're done drawing the image, so mark it as fully drawn.
                fully_drawn = true;
            }
            (MediaCacheEntry::Requested, _media_format) => {
                // If the image is being fetched, we try to show its blurhash.
                if let (Some(blurhash), Some(width), Some(height)) = (image_info.blurhash.as_deref(), image_info.width, image_info.height) {
                    let show_image_result = text_or_image_ref.show_image(cx, Some(media_source), |cx, img| {
                        let (Ok(width), Ok(height)) = (width.try_into(), height.try_into()) else {
                            return Err(image_cache::ImageError::EmptyData)
                        };
                        let (width, height): (u32, u32) = (width, height);
                        if width == 0 || height == 0 {
                            warning!("Image had an invalid aspect ratio (width or height of 0).");
                            return Err(image_cache::ImageError::EmptyData);
                        }
                        let aspect_ratio: f32 = width as f32 / height as f32;
                        // Cap the blurhash to a max size of 500 pixels in each dimension
                        // because the `blurhash::decode()` function can be rather expensive.
                        let (mut capped_width, mut capped_height) = (width, height);
                        if capped_height > BLURHASH_IMAGE_MAX_SIZE {
                            capped_height = BLURHASH_IMAGE_MAX_SIZE;
                            capped_width = (capped_height as f32 * aspect_ratio).floor() as u32;
                        }
                        if capped_width > BLURHASH_IMAGE_MAX_SIZE {
                            capped_width = BLURHASH_IMAGE_MAX_SIZE;
                            capped_height = (capped_width as f32 / aspect_ratio).floor() as u32;
                        }

                        match blurhash::decode(blurhash, capped_width, capped_height, 1.0) {
                            Ok(data) => {
                                ImageBuffer::new(&data, capped_width as usize, capped_height as usize).map(|img_buff| {
                                    let texture = Some(img_buff.into_new_texture(cx));
                                    img.set_texture(cx, texture);
                                    img.size_in_pixels(cx).unwrap_or_default()
                                })
                            }
                            Err(e) => {
                                error!("Failed to decode blurhash {e:?}");
                                Err(image_cache::ImageError::EmptyData)
                            }
                        }
                    });
                    if let Err(e) = show_image_result {
                        let err_str = format!("{body}\n\nFailed to display image: {e:?}");
                        error!("{err_str}");
                        text_or_image_ref.show_text(cx, &err_str);
                    }
                }
                fully_drawn = false;
            }
            (MediaCacheEntry::Failed(status_code), MediaFormat::Thumbnail(_))
                if should_fetch_full_size && status_code != StatusCode::NOT_FOUND =>
            {
                match media_cache.try_get_media_or_fetch(&media_source, MediaFormat::File) {
                    (MediaCacheEntry::Loaded(data), _) => {
                        let cache_key = format!("{}#full", media_source_mxc(&media_source));
                        let res = text_or_image_ref.show_image(cx, Some(media_source.clone()), |cx, img| {
                            utils::load_image_with_cache_key(&img, cx, std::path::Path::new(&cache_key), Arc::clone(&data))
                                .map(|()| img.size_in_pixels(cx).unwrap_or_default())
                        });
                        if let Err(e) = res {
                            error!("Failed to display full-size image: {e:?}");
                        }
                        fully_drawn = true;
                    }
                    (MediaCacheEntry::Requested, _) => fully_drawn = false,
                    (MediaCacheEntry::Failed(_), _) => fully_drawn = true,
                }
            }
            (MediaCacheEntry::Failed(_status_code), _media_format) => {
                text_or_image_ref.show_text(
                    cx,
                    format!("{body}\n\nFailed to fetch image from {:?}", media_source_mxc(&media_source)),
                );
                // For now, we consider this as being "complete". In the future, we could support
                // retrying to fetch thumbnail of the image on a user click/tap.
                fully_drawn = true;
            }
        }
    };

    match image_info_source {
        Some(image_info) => {
            // Use the provided thumbnail URI if it exists; otherwise use the original URI.
            let media_source = image_info.thumbnail_source.clone()
                .unwrap_or(original_source);
            fetch_and_show_media_source(cx, media_source, image_info);
        }
        None => {
            text_or_image_ref.show_text(cx, format!("{body}\n\nImage message had no source URL."));
            fully_drawn = true;
        }
    }

    fully_drawn
}


/// Draws a file message's content into the given `message_content_widget`.
///
/// Returns whether the file message content was fully drawn.
pub(super) fn populate_file_message_content(
    cx: &mut Cx,
    message_content_widget: &HtmlOrPlaintextRef,
    file_content: &FileMessageEventContent,
) -> bool {
    let filename = htmlize::escape_text(file_content.filename());
    let size = file_content
        .info
        .as_ref()
        .and_then(|info| info.size)
        .map(|bytes| format!("  ({})", utils::format_decimal_file_size(bytes.into())))
        .unwrap_or_default();
    let caption = file_content.formatted_caption()
        .filter(|fb| fb.format == MessageFormat::Html)
        .map(|fb| format!("{}<br>", fb.body))
        .or_else(|| file_content.caption().map(|c| format!("{}<br>", htmlize::escape_text(c))))
        .unwrap_or_default();

    message_content_widget.show_html(
        cx,
        format!("<b>File: </b>{caption}{filename}{size}"),
    );
    true
}

/// Draws an audio message's content into the given `message_content_widget`.
///
/// Returns whether the audio message content was fully drawn.
pub(super) fn populate_audio_message_content(
    cx: &mut Cx,
    message_content_widget: &HtmlOrPlaintextRef,
    audio: &AudioMessageEventContent,
) -> bool {
    let filename = htmlize::escape_text(audio.filename());
    let (duration, mime, size) = audio
        .info
        .as_ref()
        .map(|info| (
            info.duration
                .map(|d| format!(",  {:.2} sec", d.as_secs_f64()))
                .unwrap_or_default(),
            info.mimetype
                .as_ref()
                .map(|m| format!("  {},", htmlize::escape_text(m)))
                .unwrap_or_default(),
            info.size
                .map(|bytes| format!("  ({})", utils::format_decimal_file_size(bytes.into())))
                .unwrap_or_default(),
        ))
        .unwrap_or_default();
    let caption = audio.formatted_caption()
        .filter(|fb| fb.format == MessageFormat::Html)
        .map(|fb| format!("{}<br>", fb.body))
        .or_else(|| audio.caption().map(|c| format!("{}<br>", htmlize::escape_text(c))))
        .unwrap_or_default();

    // TODO: add an audio to play the audio file

    message_content_widget.show_html(
        cx,
        format!("<b>Audio: </b>{caption}File: <i>{filename}</i>{size}{mime}{duration}<br> → <i>Video playback not yet supported.</i>"),
    );
    true
}


/// Draws a video message's content into the given `message_content_widget`.
///
/// Returns whether the video message content was fully drawn.
pub(super) fn populate_video_message_content(
    cx: &mut Cx,
    message_content_widget: &HtmlOrPlaintextRef,
    video: &VideoMessageEventContent,
) -> bool {
    let filename = htmlize::escape_text(video.filename());
    let (duration, mime, size, dimensions) = video
        .info
        .as_ref()
        .map(|info| (
            info.duration
                .map(|d| format!(",  {:.2} sec", d.as_secs_f64()))
                .unwrap_or_default(),
            info.mimetype
                .as_ref()
                .map(|m| format!(",  {}", htmlize::escape_text(m)))
                .unwrap_or_default(),
            info.size
                .map(|bytes| format!("  ({})", utils::format_decimal_file_size(bytes.into())))
                .unwrap_or_default(),
            info.width.and_then(|width|
                info.height.map(|height| format!(",  {width}x{height}"))
            ).unwrap_or_default(),
        ))
        .unwrap_or_default();
    let caption = video.formatted_caption()
        .filter(|fb| fb.format == MessageFormat::Html)
        .map(|fb| format!("{}<br>", fb.body))
        .or_else(|| video.caption().map(|c| format!("{}<br>", htmlize::escape_text(c))))
        .unwrap_or_default();

    // TODO: populate a video widget here, once makepad supports that

    message_content_widget.show_html(
        cx,
        format!("<b>Video: </b>{caption}File: <i>{filename}</i>{size}{mime}{duration}{dimensions}<br> → <i>Video playback not yet supported.</i>"),
    );
    true
}



/// Draws the given location message's content into the `message_content_widget`.
///
/// Returns whether the location message content was fully drawn.
pub(super) fn populate_location_message_content(
    cx: &mut Cx,
    message_content_widget: &HtmlOrPlaintextRef,
    location: &LocationMessageEventContent,
) -> bool {
    let coords = location.geo_uri
        .get(utils::GEO_URI_SCHEME.len() ..)
        .and_then(|s| {
            let mut iter = s.split(',');
            if let (Some(lat), Some(long)) = (iter.next(), iter.next()) {
                Some((lat, long))
            } else {
                None
            }
        });
    if let Some((lat, long)) = coords {
        let short_lat = lat.find('.').and_then(|dot| lat.get(..dot + 7)).unwrap_or(lat);
        let short_long = long.find('.').and_then(|dot| long.get(..dot + 7)).unwrap_or(long);
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
            format!("<i>[Location invalid]</i> {}", htmlize::escape_text(&location.body))
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
pub(super) fn populate_redacted_message_content(
    cx: &mut Cx,
    message_content_widget: &HtmlOrPlaintextRef,
    event_tl_item: &EventTimelineItem,
    room_id: &OwnedRoomId,
) -> bool {
    let fully_drawn: bool;
    let mut redactor_id_and_reason = None;
    if let Some(redacted_msg) = event_tl_item.latest_json() {
        if let Ok(AnySyncTimelineEvent::MessageLike(
            AnySyncMessageLikeEvent::RoomMessage(
                SyncMessageLikeEvent::Redacted(redaction)
            )
        )) = redacted_msg.deserialize() {
            if let Ok(redacted_because) = redaction.unsigned.redacted_because.deserialize() {
                let reason = match &redacted_because {
                    AnyRedactionEvent::RoomRedaction(e) => e.content.reason.clone(),
                    _ => None,
                };
                redactor_id_and_reason = Some((
                    redacted_because.sender().to_owned(),
                    reason,
                ));
            }
        }
    }

    let html = if let Some((redactor, reason)) = redactor_id_and_reason {
        if redactor == event_tl_item.sender() {
            fully_drawn = true;
            match reason {
                Some(r) => format!("⛔ <i>Deleted their own message. Reason: \"{}\".</i>", htmlize::escape_text(r)),
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
            let redactor_name_esc = htmlize::escape_text(redactor_name.as_deref().unwrap_or(redactor.as_str()));
            match reason {
                Some(r) => format!("⛔ <i>{} deleted this message. Reason: \"{}\".</i>",
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
