use super::*;

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
pub(super) fn pick_telegram_attachment_file(
    kind: AttachmentHandoffKind,
) -> AttachmentFilePickResult {
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
pub(super) fn pick_telegram_attachment_file(
    _kind: AttachmentHandoffKind,
) -> AttachmentFilePickResult {
    AttachmentFilePickResult::Unsupported
}

pub(super) fn telegram_attachment_mime_type(path: &Path) -> mime::Mime {
    mime_guess::from_path(path).first_or_octet_stream()
}

pub(super) fn telegram_attachment_file_size(path: &Path) -> Option<u64> {
    fs::metadata(path).ok().map(|metadata| metadata.len())
}

pub(super) fn validate_telegram_attachment_file_for_review_send(
    path: &Path,
) -> Result<(), &'static str> {
    let metadata = fs::metadata(path).map_err(|_| "selected path is unreadable")?;
    if !metadata.is_file() {
        return Err("selected path is not a regular file");
    }
    if metadata.len() == 0 {
        return Err("selected file is empty");
    }
    Ok(())
}

pub(super) fn display_attachment_filename(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("attachment")
        .to_string()
}

pub(super) fn display_attachment_extension(path: &Path) -> String {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(str::trim)
        .filter(|extension| !extension.is_empty())
        .map(|extension| extension.to_ascii_lowercase())
        .unwrap_or_else(|| "no extension".to_string())
}

pub(super) fn format_attachment_file_size(size: Option<u64>) -> String {
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

pub(super) fn is_header_dimension_image_file(path: &Path, mime_type: &mime::Mime) -> bool {
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

pub(super) fn read_image_header_bytes(path: &Path) -> Option<Vec<u8>> {
    let mut file = fs::File::open(path).ok()?;
    let mut bytes = Vec::new();
    file.by_ref()
        .take(512 * 1024)
        .read_to_end(&mut bytes)
        .ok()?;
    Some(bytes)
}

pub(super) fn parse_png_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 24 || &bytes[0..8] != b"\x89PNG\r\n\x1a\n" || &bytes[12..16] != b"IHDR" {
        return None;
    }
    Some((
        u32::from_be_bytes(bytes[16..20].try_into().ok()?),
        u32::from_be_bytes(bytes[20..24].try_into().ok()?),
    ))
}

pub(super) fn parse_gif_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 10 || !matches!(&bytes[0..6], b"GIF87a" | b"GIF89a") {
        return None;
    }
    Some((
        u16::from_le_bytes(bytes[6..8].try_into().ok()?) as u32,
        u16::from_le_bytes(bytes[8..10].try_into().ok()?) as u32,
    ))
}

pub(super) fn parse_bmp_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 26 || &bytes[0..2] != b"BM" {
        return None;
    }
    let width = i32::from_le_bytes(bytes[18..22].try_into().ok()?);
    let height = i32::from_le_bytes(bytes[22..26].try_into().ok()?);
    Some((width.unsigned_abs(), height.unsigned_abs()))
}

pub(super) fn parse_jpeg_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
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

pub(super) fn parse_webp_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
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

pub(super) fn image_dimensions_from_header(bytes: &[u8]) -> Option<(u32, u32, &'static str)> {
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

pub(super) fn selected_image_dimensions_label(path: &Path, mime_type: &mime::Mime) -> String {
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

pub(super) fn pending_attachment_image_metadata_label(
    pending: &PendingAttachmentSend,
) -> Option<String> {
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

pub(super) fn format_audio_duration_millis(duration_millis: u64) -> String {
    let total_seconds = (duration_millis + 500) / 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{minutes}:{seconds:02}")
}

pub(super) fn is_wav_audio_file(path: &Path, mime_type: &mime::Mime) -> bool {
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
pub(super) struct WavAudioHeader {
    audio_format: u16,
    channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    bits_per_sample: u16,
    data_offset: usize,
    data_size: usize,
}

pub(super) fn read_wav_probe_bytes(path: &Path, byte_limit: u64) -> Option<Vec<u8>> {
    let mut file = fs::File::open(path).ok()?;
    let mut bytes = Vec::new();
    file.by_ref()
        .take(byte_limit)
        .read_to_end(&mut bytes)
        .ok()?;
    Some(bytes)
}

pub(super) fn parse_wav_audio_header(bytes: &[u8]) -> Option<WavAudioHeader> {
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

pub(super) fn wav_duration_millis(path: &Path) -> Option<u64> {
    let bytes = read_wav_probe_bytes(path, 64 * 1024)?;
    let header = parse_wav_audio_header(&bytes)?;
    (header.byte_rate > 0).then(|| header.data_size as u64 * 1000 / header.byte_rate as u64)
}

pub(super) fn voice_audio_duration_label(path: &Path, mime_type: &mime::Mime) -> String {
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

pub(super) fn wav_codec_name(audio_format: u16) -> &'static str {
    match audio_format {
        1 => "PCM",
        3 => "IEEE float",
        6 => "A-law",
        7 => "mu-law",
        0xfffe => "WAVE extensible",
        _ => "unknown WAV codec",
    }
}

pub(super) fn pcm_sample_peak(sample: &[u8], bits_per_sample: u16) -> Option<f64> {
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

pub(super) fn wav_pcm_peak_buckets(
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

pub(super) fn format_waveform_peaks(peaks: &[u8]) -> String {
    peaks
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

pub(super) fn voice_audio_waveform_codec_label(path: &Path, mime_type: &mime::Mime) -> String {
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

pub(super) fn pending_attachment_audio_metadata_label(
    pending: &PendingAttachmentSend,
) -> Option<String> {
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

pub(super) fn summarize_attachment_caption(input: &str) -> String {
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

pub(super) fn attachment_review_lifecycle_metadata_label(
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

pub(super) fn attachment_send_failure_retry_confirmation_label(
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

pub(super) fn attachment_multi_file_queue_boundary_label(
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

pub(super) fn attachment_accepted_queue_actions_row_label(
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

pub(super) fn attachment_accepted_queue_background_snapshot_label(
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

pub(super) fn attachment_accepted_queue_timeline_cancel_bridge_label(
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

pub(super) fn attachment_local_send_abort_result_label(result: &Result<bool, String>) -> String {
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

pub(super) fn attachment_per_file_queue_drilldown_label(
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

pub(super) fn attachment_sdk_queue_contract_packet_label(
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

pub(super) fn attachment_queue_progress_result_taxonomy_packet_label(
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

pub(super) fn attachment_per_file_status_controls_label(
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

pub(super) fn attachment_mobile_picker_controls_label(
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

pub(super) fn attachment_send_preflight_control_from_status(status: &str) -> &'static str {
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

pub(super) fn attachment_send_preflight_detail_controls_label(
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
