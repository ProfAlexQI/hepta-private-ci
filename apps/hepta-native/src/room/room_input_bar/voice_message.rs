use super::*;

pub(super) fn voice_message_lifecycle_metadata_label(
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

pub(super) fn voice_confirmation_cancel_metadata_label(
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

pub(super) fn voice_message_recorder_waveform_codec_boundary_label(
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

pub(super) fn voice_message_recorder_status_controls_label(
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

pub(super) fn voice_message_capture_lifecycle_controls_label(
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

pub(super) fn voice_message_capture_request_packet_snapshot_label(
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

pub(super) fn voice_message_recorder_lifecycle_drilldown_packet_label(
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

pub(super) fn voice_message_recorder_typed_contract_packet_label(
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

pub(super) fn voice_message_recorder_result_taxonomy_packet_label(
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

pub(super) fn voice_message_mobile_picker_controls_label(
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

pub(super) fn voice_message_review_playback_controls_label(
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

pub(super) fn open_voice_review_audio_file(path: &Path) -> Result<(), String> {
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

pub(super) fn voice_message_review_playback_open_result_label(
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

pub(super) fn voice_message_review_drop_pending_audio_label(
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

pub(super) fn voice_message_send_preflight_control_from_status(status: &str) -> &'static str {
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

pub(super) fn voice_message_send_preflight_detail_controls_label(
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
