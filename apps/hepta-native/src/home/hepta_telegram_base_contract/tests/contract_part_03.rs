#[test]
fn hepta_telegram_base_composer_attachment_placeholder_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_PLACEHOLDER_MARKER,
        "hepta_telegram_composer_attachment_placeholder_local_only"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_upload_composer"));
}

#[test]
fn hepta_telegram_base_composer_attachment_surface_has_send_handoff_gap() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_LOCAL_SURFACE_MARKER,
        "hepta_telegram_composer_attachment_local_surface_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_picker_surface"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "file upload send"
            && capability.notes.contains("desktop rfd picker")
            && capability.notes.contains("local selected-file preview")
            && capability.notes.contains("local pre-send review")
            && capability
                .notes
                .contains("full cross-platform attachment UX")
            && capability.notes.contains("remain TODO")
    }));
}

#[test]
fn hepta_telegram_base_composer_attachment_option_staging_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_OPTION_STAGING_LOCAL_MARKER,
        "hepta_telegram_composer_attachment_option_staging_local_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_option_staging_local_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment option staging local evidence"
            && capability
                .notes
                .contains("Photo and File only open the confirmation guard")
            && capability.notes.contains("Camera, Contact, and Close")
            && capability.notes.contains("no native picker, upload")
            && capability.notes.contains("before confirmation")
    }));
}

#[test]
fn hepta_telegram_base_attachment_camera_contact_boundary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_CAMERA_CONTACT_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_composer_attachment_camera_contact_local_boundary_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_CAMERA_CONTACT_LOCAL_BOUNDARY_EVIDENCE
            .contains("local-only placeholders")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_CAMERA_CONTACT_LOCAL_BOUNDARY_EVIDENCE
            .contains("does not request camera or photo-library permission")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_CAMERA_CONTACT_LOCAL_BOUNDARY_EVIDENCE
            .contains("does not request contacts permission")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_CAMERA_CONTACT_LOCAL_BOUNDARY_EVIDENCE
            .contains("vCard/contact payloads")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_CAMERA_CONTACT_LOCAL_BOUNDARY_LABEL
            .contains("no permissions, capture, contacts read")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_camera_contact_local_boundary_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment camera/contact local boundary evidence"
            && capability.base_module.contains("Camera/Contact")
            && capability.notes.contains("file_upload_send")
            && capability.notes.contains("cross-platform capture")
            && capability.notes.contains("does not request camera")
            && capability
                .notes
                .contains("does not request contacts permission")
            && capability.notes.contains("vCard/contact payloads")
            && capability.notes.contains("SendAttachment")
            && capability.notes.contains("SendMessage")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_attachment_handoff_confirmation_is_picker_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_HANDOFF_CONFIRMATION_MARKER,
        "hepta_telegram_composer_attachment_handoff_confirmation_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_handoff_confirmation_guard")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment handoff confirmation evidence"
            && capability
                .notes
                .contains("before the native desktop picker")
            && capability.notes.contains("opening and canceling")
            && capability.notes.contains("picker cancel")
            && capability.notes.contains("local pre-send review")
            && capability.notes.contains("send no upload")
    }));
}

#[test]
fn hepta_telegram_base_composer_attachment_send_handoff_uses_matrix_send_queue() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_SEND_HANDOFF_MARKER,
        "hepta_telegram_composer_attachment_send_handoff_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_send_handoff_evidence"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment send handoff"
            && capability
                .base_module
                .contains("MatrixRequest::SendAttachment")
            && capability.notes.contains("desktop rfd file")
            && capability.notes.contains("review row Send action")
            && capability
                .notes
                .contains("Timeline::send_attachment().use_send_queue()")
            && capability.notes.contains("unsupported platforms")
            && capability.notes.contains("gateway/runtime/auth")
    }));
}

#[test]
fn hepta_telegram_base_composer_attachment_pre_send_review_is_local_until_send() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_PRE_SEND_REVIEW_MARKER,
        "hepta_telegram_composer_attachment_pre_send_review_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_PRE_SEND_REVIEW_EVIDENCE
            .contains("local pending review state")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_PRE_SEND_REVIEW_EVIDENCE
            .contains("until the user clicks Send")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_PRE_SEND_REVIEW_EVIDENCE
            .contains("Discard and Close clear the pending attachment locally")
    );
    assert!(crate::room::room_input_bar::ATTACHMENT_PRE_SEND_REVIEW_EVIDENCE.contains("MIME type"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_pre_send_review_local_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment pre-send review local evidence"
            && capability
                .base_module
                .contains("telegram_pending_attachment_send")
            && capability.notes.contains("filename, MIME type")
            && capability.notes.contains("local file size")
            && capability.notes.contains("Only the review row Send button")
            && capability.notes.contains("Discard and Close clear pending")
            && capability.notes.contains("no Matrix media upload")
            && capability.notes.contains("gateway/runtime/auth")
    }));
}

#[test]
fn hepta_telegram_base_composer_attachment_selected_file_preview_is_local_metadata() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_SELECTED_FILE_PREVIEW_MARKER,
        "hepta_telegram_composer_attachment_selected_file_preview_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SELECTED_FILE_PREVIEW_EVIDENCE
            .contains("filename, MIME type, file extension")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SELECTED_FILE_PREVIEW_EVIDENCE
            .contains("local file size")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SELECTED_FILE_PREVIEW_EVIDENCE
            .contains("no upload, media decode, thumbnail generation")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SELECTED_FILE_PREVIEW_EVIDENCE
            .contains("final caption")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_selected_file_preview_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_selected_image_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment selected-file preview local evidence"
            && capability.base_module.contains("attachment_review_preview")
            && capability.notes.contains("filename, MIME type, extension")
            && capability.notes.contains("local size")
            && capability
                .notes
                .contains("no upload, media decode, thumbnail generation")
            && capability.notes.contains("review Send is clicked")
    }));
}

#[test]
fn hepta_telegram_base_composer_attachment_selected_image_metadata_is_header_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_SELECTED_IMAGE_METADATA_MARKER,
        "hepta_telegram_composer_attachment_selected_image_metadata_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_selected_image_metadata_preview")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SELECTED_IMAGE_METADATA_EVIDENCE
            .contains("filename, MIME type, extension, local file size")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SELECTED_IMAGE_METADATA_EVIDENCE
            .contains("image dimensions status")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SELECTED_IMAGE_METADATA_EVIDENCE
            .contains("PNG, JPEG, GIF, BMP, or WebP header")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SELECTED_IMAGE_METADATA_EVIDENCE
            .contains("no thumbnail decode, full image decode")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SELECTED_IMAGE_METADATA_LABEL
            .contains("dimensions status")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment selected image metadata preview"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.base_module.contains("Photo picker")
            && capability.notes.contains("filename, MIME type, extension")
            && capability.notes.contains("dimensions status")
            && capability
                .notes
                .contains("PNG, JPEG, GIF, BMP, or WebP header")
            && capability.notes.contains("no thumbnail decode")
            && capability.notes.contains("full image decode")
            && capability.notes.contains("before review Send")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_attachment_main_send_guard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_MAIN_SEND_GUARD_MARKER,
        "hepta_telegram_composer_attachment_main_send_guard_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MAIN_SEND_GUARD_EVIDENCE
            .contains("main composer Send button and Enter")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MAIN_SEND_GUARD_EVIDENCE
            .contains("does not send the caption as a plain text SendMessage")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MAIN_SEND_GUARD_EVIDENCE
            .contains("does not submit SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MAIN_SEND_GUARD_EVIDENCE
            .contains("does not clear the pending attachment")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_main_send_guard_local_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment main send guard local evidence"
            && capability.base_module.contains("send_message_button")
            && capability.notes.contains("main composer Send button")
            && capability
                .notes
                .contains("do not send caption text as SendMessage")
            && capability.notes.contains("submit SendAttachment")
            && capability.notes.contains("clear the pending attachment")
            && capability.notes.contains("gateway/runtime/auth")
    }));
}

#[test]
fn hepta_telegram_base_composer_attachment_selection_replacement_preserves_pending() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_SELECTION_REPLACEMENT_PRESERVE_MARKER,
        "hepta_telegram_composer_attachment_selection_replacement_preserve_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SELECTION_REPLACEMENT_PRESERVE_EVIDENCE
            .contains("local replacement of pending review state only")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SELECTION_REPLACEMENT_PRESERVE_EVIDENCE
            .contains("picker cancel as preservation")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SELECTION_REPLACEMENT_PRESERVE_EVIDENCE
            .contains("does not upload or send the previous file")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SELECTION_REPLACEMENT_PRESERVE_EVIDENCE
            .contains("does not clear caption/reply context")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_selection_replacement_preserve_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment selection replacement preserve evidence"
            && capability
                .base_module
                .contains("telegram_pending_attachment_send")
            && capability
                .notes
                .contains("replaces only local pending review state")
            && capability
                .notes
                .contains("preserves the existing pending attachment")
            && capability
                .notes
                .contains("do not upload or send the previous file")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live state")
    }));
}

#[test]
fn hepta_telegram_base_composer_attachment_review_lifecycle_metadata_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_REVIEW_LIFECYCLE_METADATA_MARKER,
        "hepta_telegram_composer_attachment_review_lifecycle_metadata_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_LIFECYCLE_METADATA_EVIDENCE
            .contains("Select, Replace, picker cancel, Close, and Discard")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_LIFECYCLE_METADATA_EVIDENCE
            .contains("filename, MIME type, local size")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_LIFECYCLE_METADATA_EVIDENCE
            .contains("previous pending filename")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_LIFECYCLE_METADATA_EVIDENCE
            .contains("MatrixRequest::SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_LIFECYCLE_METADATA_LABEL
            .contains("local pending/replaced/closed/discarded")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_review_lifecycle_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment review lifecycle metadata preview"
            && capability
                .base_module
                .contains("telegram_pending_attachment_send")
            && capability.notes.contains("caption preview")
            && capability.notes.contains("validation warning")
            && capability.notes.contains("previous pending filename")
            && capability.notes.contains("MatrixRequest::SendAttachment")
            && capability.notes.contains("SDK send-queue")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_attachment_review_send_is_single_submit_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_REVIEW_SEND_SINGLE_SUBMIT_MARKER,
        "hepta_telegram_composer_attachment_review_send_single_submit_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_SEND_SINGLE_SUBMIT_EVIDENCE
            .contains("Option::take() before review-row Send")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_SEND_SINGLE_SUBMIT_EVIDENCE
            .contains("second click or empty review Send")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_SEND_SINGLE_SUBMIT_EVIDENCE
            .contains("does not submit duplicate SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_SEND_SINGLE_SUBMIT_EVIDENCE
            .contains("does not send the caption as SendMessage")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_review_send_single_submit_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment review send single-submit evidence"
            && capability
                .base_module
                .contains("telegram_pending_attachment_send.take")
            && capability.notes.contains("duplicate clicks")
            && capability
                .notes
                .contains("does not submit duplicate SendAttachment")
            && capability.notes.contains("send the caption as SendMessage")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live state")
    }));
}

#[test]
fn hepta_telegram_base_composer_attachment_review_discard_close_is_idempotent() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_REVIEW_DISCARD_CLOSE_IDEMPOTENT_MARKER,
        "hepta_telegram_composer_attachment_review_discard_close_idempotent_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_DISCARD_CLOSE_IDEMPOTENT_EVIDENCE
            .contains("Discard and Close consume telegram_pending_attachment_send")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_DISCARD_CLOSE_IDEMPOTENT_EVIDENCE
            .contains("review-row Send after Discard/Close")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_DISCARD_CLOSE_IDEMPOTENT_EVIDENCE
            .contains("Repeated Discard, empty Discard, empty Close")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_DISCARD_CLOSE_IDEMPOTENT_EVIDENCE
            .contains("no caption-only SendMessage")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_review_discard_close_idempotent_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment review discard close idempotent evidence"
            && capability
                .base_module
                .contains("discard_selected_attachment_button")
            && capability
                .base_module
                .contains("close_attachment_picker_button")
            && capability.notes.contains("Repeated Discard")
            && capability
                .notes
                .contains("review-row Send after Discard/Close")
            && capability
                .notes
                .contains("clear composer caption/reply text")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live state")
    }));
}

#[test]
fn hepta_telegram_base_composer_attachment_caption_reply_context_boundary_is_explicit() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_CAPTION_REPLY_CONTEXT_MARKER,
        "hepta_telegram_composer_attachment_caption_reply_context_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_CAPTION_REPLY_CONTEXT_BOUNDARY_EVIDENCE
            .contains("Caption preview live-updates from the composer text")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_CAPTION_REPLY_CONTEXT_BOUNDARY_EVIDENCE
            .contains("preserving composer caption/reply text")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_CAPTION_REPLY_CONTEXT_BOUNDARY_EVIDENCE
            .contains("only attachment path that consumes the current composer caption")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_CAPTION_REPLY_CONTEXT_BOUNDARY_EVIDENCE
            .contains("captured reply/thread event id")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_caption_reply_context_boundary_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment caption reply context boundary evidence"
            && capability
                .base_module
                .contains("MentionableTextInput caption preview")
            && capability.notes.contains("caption preview live-updates")
            && capability
                .notes
                .contains("preserve composer caption/reply text")
            && capability
                .notes
                .contains("only attachment path that consumes")
            && capability.notes.contains("captured reply/thread event id")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_attachment_file_validation_is_local_error_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_FILE_VALIDATION_LOCAL_ERROR_MARKER,
        "hepta_telegram_composer_attachment_file_validation_local_error_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_FILE_VALIDATION_LOCAL_ERROR_EVIDENCE
            .contains("final local attachment file validation")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_FILE_VALIDATION_LOCAL_ERROR_EVIDENCE
            .contains("unreadable, not a regular file, or an empty file")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_FILE_VALIDATION_LOCAL_ERROR_EVIDENCE
            .contains("Attachment validation held locally")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_FILE_VALIDATION_LOCAL_ERROR_EVIDENCE
            .contains("MIME fallback to application/octet-stream")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_file_validation_local_error_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment file validation local error evidence"
            && capability
                .base_module
                .contains("validate_telegram_attachment_file_for_review_send")
            && capability.notes.contains("Unreadable paths")
            && capability.notes.contains("empty files")
            && capability
                .notes
                .contains("Attachment validation held locally")
            && capability.notes.contains("application/octet-stream")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_attachment_validation_error_recovery_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_VALIDATION_ERROR_RECOVERY_MARKER,
        "hepta_telegram_composer_attachment_validation_error_recovery_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_VALIDATION_ERROR_RECOVERY_EVIDENCE
            .contains("recoverable local review state")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_VALIDATION_ERROR_RECOVERY_EVIDENCE
            .contains("choosing Photo/File again replaces the local pending review")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_VALIDATION_ERROR_RECOVERY_EVIDENCE
            .contains("Discard and Close clear the pending review plus warning locally")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_VALIDATION_ERROR_RECOVERY_EVIDENCE
            .contains("Retry/Cancel controls remain local evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_validation_error_recovery_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment validation error recovery evidence"
            && capability.base_module.contains("validation_error")
            && capability
                .notes
                .contains("pending attachment stays visible")
            && capability.notes.contains("clears the validation warning")
            && capability.notes.contains("Retry/Cancel controls")
            && capability.notes.contains("SendAttachment")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation state")
    }));
}

#[test]
fn hepta_telegram_base_attachment_send_operation_status_is_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_SEND_OPERATION_STATUS_LOCAL_MARKER,
        "hepta_telegram_attachment_send_operation_status_local_ready"
    );
    assert!(
        crate::room::room_input_bar::MESSAGE_SEND_OPERATION_STATUS_EVIDENCE
            .contains("confirmed desktop attachments")
    );
    assert!(
        crate::room::room_input_bar::MESSAGE_SEND_OPERATION_STATUS_EVIDENCE
            .contains("Attachment worker failure Retry is the one guarded exception")
    );
    assert!(
        crate::room::room_input_bar::MESSAGE_SEND_OPERATION_STATUS_EVIDENCE
            .contains("Retry never auto-runs")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_send_operation_status_local_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment send operation status local evidence"
            && capability
                .base_module
                .contains("MatrixRequest::SendAttachment")
            && capability
                .notes
                .contains("worker queued/failure handoff result")
            && capability
                .notes
                .contains("timeline local echo rows show SDK queue progress/error/sent state")
            && capability.notes.contains("Worker failure Retry confirms")
            && capability
                .notes
                .contains("Cancel does not abort SDK send-queue work")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_attachment_send_result_bridge_is_honest_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_SEND_RESULT_BRIDGE_MARKER,
        "hepta_telegram_attachment_send_result_bridge_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SEND_RESULT_BRIDGE_EVIDENCE
            .contains("Timeline::send_attachment().use_send_queue()")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SEND_RESULT_BRIDGE_EVIDENCE
            .contains("async worker success returns a queued-only result")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SEND_RESULT_BRIDGE_EVIDENCE
            .contains("worker failure returns a failure-copy result")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SEND_RESULT_BRIDGE_EVIDENCE
            .contains("does not claim delivery success")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SEND_RESULT_BRIDGE_EVIDENCE
            .contains("resubmit SendAttachment")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_send_result_bridge_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_queue_failure_recovery_copy_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_status_taxonomy_local_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment send result bridge evidence"
            && capability
                .base_module
                .contains("sliding_sync popup result path")
            && capability
                .notes
                .contains("queued-only or immediate failure result")
            && capability.notes.contains("existing popup error path")
            && capability.notes.contains("does not claim delivery success")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_attachment_queue_failure_recovery_copy_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_QUEUE_FAILURE_RECOVERY_MARKER,
        "hepta_telegram_attachment_queue_failure_recovery_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_QUEUE_FAILURE_RECOVERY_COPY_MARKER,
        "hepta_telegram_attachment_queue_failure_recovery_copy_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_QUEUE_FAILURE_RECOVERY_COPY_EVIDENCE
            .contains("queued or immediate handoff failure")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_QUEUE_FAILURE_RECOVERY_COPY_EVIDENCE.contains(
            "SDK queue progress/error/sent state is rendered on the timeline local echo row"
        )
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_QUEUE_FAILURE_RECOVERY_COPY_EVIDENCE
            .contains("Worker failure Retry reuses the cached last validated SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_QUEUE_FAILURE_RECOVERY_COPY_EVIDENCE
            .contains("Cancel does not abort, remove, or cancel SDK send-queue work")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_QUEUE_FAILURE_RECOVERY_RESULT_BRIDGE_LABEL
            .contains("worker reports queued or immediate failure")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_queue_failure_recovery_copy_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment queue failure recovery copy evidence"
            && capability.base_module.contains("result_bridge")
            && capability
                .notes
                .contains("queued-only or immediate failure handoff results")
            && capability
                .notes
                .contains("RoomScreen renders SDK queue progress/error/sent state")
            && capability.notes.contains("caption-only SendMessage")
            && capability
                .notes
                .contains("Cancel does not abort, remove, or cancel SDK send-queue work")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_attachment_send_failure_retry_confirmation_is_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_SEND_FAILURE_RETRY_CONFIRMATION_MARKER,
        "hepta_telegram_attachment_send_failure_retry_confirmation_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SEND_FAILURE_RETRY_CONFIRMATION_EVIDENCE
            .contains("last validated attachment handoff")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SEND_FAILURE_RETRY_CONFIRMATION_EVIDENCE
            .contains("PositiveConfirmationModal")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SEND_FAILURE_RETRY_CONFIRMATION_EVIDENCE
            .contains("MatrixRequest::SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SEND_FAILURE_RETRY_CONFIRMATION_LABEL
            .contains("SDK queue retry/resume/cancel stays unwired")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_send_failure_retry_confirmation_guard")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment send failure retry confirmation"
            && capability
                .base_module
                .contains("telegram_attachment_send_retry_attempt")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("MatrixRequest::SendAttachment")
            && capability.notes.contains("SDK queue retry/resume")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_attachment_true_queue_control_boundary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_TRUE_QUEUE_CONTROL_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_attachment_true_queue_control_local_boundary_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_TRUE_QUEUE_CONTROL_LOCAL_BOUNDARY_EVIDENCE
            .contains("remaining file_upload_send queue-control gap")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_TRUE_QUEUE_CONTROL_LOCAL_BOUNDARY_EVIDENCE
            .contains("Timeline::send_attachment().use_send_queue()")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_TRUE_QUEUE_CONTROL_LOCAL_BOUNDARY_EVIDENCE
            .contains("RoomScreen renders SDK queue progress/error/sent state")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_TRUE_QUEUE_CONTROL_LOCAL_BOUNDARY_EVIDENCE
            .contains("does not retry or resume accepted SDK queue uploads")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_TRUE_QUEUE_CONTROL_LOCAL_BOUNDARY_LABEL
            .contains("timeline shows SDK queue state")
    );
    assert!(
        crate::home::room_screen::ATTACHMENT_TIMELINE_SEND_STATE_COMPACT_LABEL
            .contains("SDK queue progress/error/sent")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_true_queue_control_local_boundary_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment true queue control local boundary evidence"
            && capability
                .base_module
                .contains("Timeline::send_attachment().use_send_queue")
            && capability.notes.contains("handoff is real")
            && capability
                .notes
                .contains("worker now reports queued or immediate failure")
            && capability
                .notes
                .contains("RoomScreen renders SDK queue progress/error/sent state")
            && capability
                .notes
                .contains("true file_upload_send queue control remains a base gap")
            && capability
                .notes
                .contains("does not retry or resume accepted SDK queue uploads")
            && capability.notes.contains("abort uploads")
            && capability
                .notes
                .contains("Cancel emits no SDK queue abort/remove/cancel")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_attachment_accepted_queue_actions_row_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_MARKER,
        "hepta_telegram_attachment_accepted_queue_actions_row_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(
        crate::room::room_input_bar::ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_EVIDENCE
            .contains("Pause, Resume, Reorder, Background, and Clear")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_EVIDENCE
            .contains("MatrixRequest::SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_EVIDENCE
            .contains("Timeline::send_attachment().use_send_queue()")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_EVIDENCE
            .contains("local accepted attachment queue snapshot")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_EVIDENCE
            .contains("without retrying or resuming accepted SDK queue uploads")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_ACCEPTED_QUEUE_ACTIONS_ROW_LABEL
            .contains("only failed-handoff Retry can resubmit")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_accepted_queue_actions_row")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment accepted queue actions row"
            && capability.base_module.contains("accepted_queue_actions")
            && capability.notes.contains("Pause")
            && capability.notes.contains("Resume")
            && capability.notes.contains("Reorder")
            && capability.notes.contains("Background")
            && capability.notes.contains("Clear")
            && capability
                .notes
                .contains("local accepted attachment queue snapshot")
            && capability
                .notes
                .contains("only call the local accepted-queue boundary handler")
            && capability.notes.contains("resubmit SendAttachment")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_attachment_accepted_queue_timeline_cancel_bridge_is_explicit() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_ACCEPTED_QUEUE_TIMELINE_CANCEL_BRIDGE_MARKER,
        "hepta_telegram_attachment_accepted_queue_timeline_cancel_bridge_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(
        crate::room::room_input_bar::ATTACHMENT_ACCEPTED_QUEUE_TIMELINE_CANCEL_BRIDGE_EVIDENCE
            .contains("Status, Handle, Timeline, Cancel, and Source")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_ACCEPTED_QUEUE_TIMELINE_CANCEL_BRIDGE_EVIDENCE
            .contains("local_echo_send_handle")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_ACCEPTED_QUEUE_TIMELINE_CANCEL_BRIDGE_EVIDENCE
            .contains("MatrixRequest::AbortLocalSend")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_ACCEPTED_QUEUE_TIMELINE_CANCEL_BRIDGE_EVIDENCE
            .contains("TimelineUpdate::LocalSendAbortResult")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_ACCEPTED_QUEUE_TIMELINE_CANCEL_BRIDGE_LABEL
            .contains("Timeline cancel bridge")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_accepted_queue_timeline_cancel_bridge")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment accepted queue timeline cancel bridge"
            && capability
                .base_module
                .contains("accepted_queue_timeline_cancel_bridge")
            && capability
                .base_module
                .contains("MatrixRequest::AbortLocalSend")
            && capability
                .base_module
                .contains("TimelineUpdate::LocalSendAbortResult")
            && capability.notes.contains("Status")
            && capability.notes.contains("Handle")
            && capability.notes.contains("Timeline")
            && capability.notes.contains("Cancel")
            && capability.notes.contains("Source")
            && capability.notes.contains("local_echo_send_handle")
            && capability
                .notes
                .contains("TimelineUpdate::LocalSendAbortResult")
            && capability
                .notes
                .contains("already-sent/no-longer-cancellable")
            && capability.notes.contains("holds no SendHandle")
            && capability.notes.contains("resubmits no SendAttachment")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_attachment_local_send_abort_result_bridge_is_live() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_LOCAL_SEND_ABORT_RESULT_MARKER,
        "hepta_telegram_attachment_local_send_abort_result_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(
        crate::room::room_input_bar::ATTACHMENT_LOCAL_SEND_ABORT_RESULT_EVIDENCE
            .contains("TimelineUpdate::LocalSendAbortResult")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_LOCAL_SEND_ABORT_RESULT_LABEL
            .contains("operation strip")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_local_send_abort_result_bridge")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment local send abort result bridge"
            && capability
                .base_module
                .contains("MatrixRequest::AbortLocalSend")
            && capability
                .base_module
                .contains("TimelineUpdate::LocalSendAbortResult")
            && capability
                .base_module
                .contains("handle_local_send_abort_result")
            && capability.notes.contains("SendHandle::abort")
            && capability.notes.contains("canceled")
            && capability
                .notes
                .contains("already-sent/no-longer-cancellable")
            && capability.notes.contains("failed outcomes")
            && capability.notes.contains("resubmitting SendAttachment")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_attachment_per_file_status_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_PER_FILE_STATUS_CONTROLS_MARKER,
        "hepta_telegram_attachment_per_file_status_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(
        crate::room::room_input_bar::ATTACHMENT_PER_FILE_STATUS_CONTROLS_EVIDENCE.contains(
            "Status, Progress, Pause, Resume, Cancel, Retry, Drilldown, Contract, and Taxonomy"
        )
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_PER_FILE_STATUS_CONTROLS_EVIDENCE
            .contains("typed SDK queue control/progress/result/error")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_PER_FILE_STATUS_CONTROLS_EVIDENCE
            .contains("current local pending review state")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_PER_FILE_STATUS_CONTROLS_EVIDENCE
            .contains("cached immediate handoff retry availability")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_PER_FILE_STATUS_CONTROLS_EVIDENCE
            .contains("does not inspect or mutate SDK queue entries")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_PER_FILE_STATUS_CONTROLS_EVIDENCE
            .contains("retry accepted SDK queue items")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_PER_FILE_STATUS_CONTROLS_LABEL
            .contains("failed-handoff Retry confirmation remains the only resubmit path")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_per_file_status_controls_row")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment per-file status controls row"
            && capability.base_module.contains("per_file_status_controls")
            && capability.notes.contains("Status")
            && capability.notes.contains("Progress")
            && capability.notes.contains("Pause")
            && capability.notes.contains("Resume")
            && capability.notes.contains("Cancel")
            && capability.notes.contains("Retry")
            && capability.notes.contains("Drilldown")
            && capability.notes.contains("Contract")
            && capability.notes.contains("Taxonomy")
            && capability
                .notes
                .contains("typed SDK queue control/progress/result/error")
            && capability
                .notes
                .contains("accepted queue/progress/result slots")
            && capability
                .notes
                .contains("local per-file status boundary handler")
            && capability.notes.contains("upload progress")
            && capability.notes.contains("retry accepted SDK queue items")
            && capability.notes.contains("map delivery receipts")
            && capability.notes.contains("resubmit SendAttachment")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_attachment_sdk_queue_contract_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_MARKER,
        "hepta_telegram_attachment_sdk_queue_contract_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_EVIDENCE
            .contains("typed SDK queue contract")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_EVIDENCE
            .contains("upload progress bytes/percent/speed/ETA")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_EVIDENCE
            .contains("pause/resume/cancel/retry/reorder/remove eligibility")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_EVIDENCE
            .contains("SendHandle and AbortLocalSend boundary")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_EVIDENCE
            .contains("delivery receipt mapping")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SDK_QUEUE_CONTRACT_PACKET_LABEL
            .contains("typed progress/control/result/error")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_sdk_queue_contract_packet_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment SDK queue contract packet"
            && capability
                .base_module
                .contains("contract_attachment_file_button")
            && capability
                .base_module
                .contains("attachment_sdk_queue_contract_packet_label")
            && capability.notes.contains("typed SDK queue contract")
            && capability.notes.contains("queue item/local echo identity")
            && capability
                .notes
                .contains("upload progress bytes/percent/speed/ETA")
            && capability.notes.contains("SendHandle")
            && capability.notes.contains("AbortLocalSend")
            && capability.notes.contains("delivery receipt mapping")
            && capability.notes.contains("multi-file album grouping")
            && capability.notes.contains("idempotency")
            && capability.notes.contains("no SDK queue lookup")
            && capability.notes.contains("SendAttachment resubmit")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_attachment_per_file_queue_drilldown_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_PER_FILE_QUEUE_DRILLDOWN_MARKER,
        "hepta_telegram_attachment_per_file_queue_drilldown_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(
        crate::room::room_input_bar::ATTACHMENT_PER_FILE_QUEUE_DRILLDOWN_EVIDENCE
            .contains("local accepted-send queue acceptance matrix")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_PER_FILE_QUEUE_DRILLDOWN_EVIDENCE
            .contains("queue item identity")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_PER_FILE_QUEUE_DRILLDOWN_EVIDENCE
            .contains("progress slot")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_PER_FILE_QUEUE_DRILLDOWN_EVIDENCE
            .contains("timeline local-echo cancel handle")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_PER_FILE_QUEUE_DRILLDOWN_EVIDENCE
            .contains("SDK queue lookup")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_PER_FILE_QUEUE_DRILLDOWN_EVIDENCE
            .contains("live mutation")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_per_file_queue_drilldown_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment per-file queue drilldown"
            && capability
                .base_module
                .contains("drilldown_attachment_file_button")
            && capability
                .notes
                .contains("accepted-send queue acceptance matrix")
            && capability.notes.contains("queue item identity")
            && capability.notes.contains("progress slot")
            && capability.notes.contains("pause/resume/cancel eligibility")
            && capability
                .notes
                .contains("timeline local-echo cancel handle")
            && capability.notes.contains("delivery receipt mapping")
            && capability.notes.contains("reorder/grouping")
            && capability.notes.contains("no SDK queue lookup")
            && capability.notes.contains("SendAttachment resubmit")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_attachment_send_preflight_detail_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_SEND_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_attachment_send_preflight_detail_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("Request, Result, Error, Retry, and Source")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("current local pending review state")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("cached immediate handoff failure text")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("does not submit MatrixRequest::SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("retry accepted SDK queue items")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_SEND_PREFLIGHT_DETAIL_CONTROLS_LABEL
            .contains("review Send and confirmed failed-handoff Retry")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_send_preflight_detail_controls_row")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment send preflight detail controls row"
            && capability
                .base_module
                .contains("send_preflight_detail_controls")
            && capability.notes.contains("Request")
            && capability.notes.contains("Result")
            && capability.notes.contains("Error")
            && capability.notes.contains("Retry")
            && capability.notes.contains("Source")
            && capability.notes.contains("local preflight-detail handler")
            && capability
                .notes
                .contains("cached immediate handoff failure")
            && capability.notes.contains("subscribe to upload progress")
            && capability.notes.contains("retry accepted SDK queue items")
            && capability.notes.contains("cancel SDK send-queue work")
            && capability.notes.contains("caption-only SendMessage")
            && capability.notes.contains("map delivery receipts")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_attachment_multi_file_queue_boundary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_MULTI_FILE_QUEUE_BOUNDARY_MARKER,
        "hepta_telegram_attachment_multi_file_queue_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MULTI_FILE_QUEUE_BOUNDARY_EVIDENCE
            .contains("Multiple-file selection")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MULTI_FILE_QUEUE_BOUNDARY_EVIDENCE
            .contains("album grouping")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MULTI_FILE_QUEUE_BOUNDARY_EVIDENCE
            .contains("per-file progress rows")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MULTI_FILE_QUEUE_BOUNDARY_EVIDENCE
            .contains("background upload list")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MULTI_FILE_QUEUE_BOUNDARY_EVIDENCE
            .contains("accepted SDK queue retry/resume/cancel")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MULTI_FILE_QUEUE_BOUNDARY_EVIDENCE
            .contains("delivery receipt fan-in")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MULTI_FILE_QUEUE_BOUNDARY_LABEL
            .contains("local blocked")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_multi_file_queue_boundary_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment multi file queue boundary"
            && capability.base_module.contains("multi_file_queue_boundary")
            && capability.notes.contains("local pending review state")
            && capability
                .notes
                .contains("cached immediate handoff retry state")
            && capability
                .notes
                .contains("single selected-file review SendAttachment")
            && capability.notes.contains("Multiple-file selection")
            && capability.notes.contains("album grouping")
            && capability.notes.contains("per-file progress rows")
            && capability.notes.contains("background upload list")
            && capability.notes.contains("bulk retry")
            && capability
                .notes
                .contains("accepted SDK queue retry/resume/cancel")
            && capability.notes.contains("delivery receipt fan-in")
            && capability.notes.contains("SDK queue abort/remove/cancel")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability
                .notes
                .contains("file_upload_send remains a base gap")
    }));
}

#[test]
fn hepta_telegram_base_attachment_timeline_send_state_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_TIMELINE_SEND_STATE_MARKER,
        "hepta_telegram_attachment_timeline_send_state_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(
        crate::home::room_screen::ATTACHMENT_TIMELINE_SEND_STATE_COMPACT_LABEL
            .contains("SDK queue progress/error/sent")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_timeline_send_state_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment timeline send state"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.base_module.contains("RoomScreen")
            && capability
                .base_module
                .contains("MatrixRequest::AbortLocalSend")
            && capability.notes.contains("SDK queue progress")
            && capability.notes.contains("Timeline::send_attachment")
            && capability.notes.contains("local_echo_send_handle")
            && capability
                .notes
                .contains("TimelineUpdate::LocalSendAbortResult")
            && capability.notes.contains("submits no SendAttachment")
            && capability.notes.contains("extra SendMessage")
            && capability.notes.contains("room-state")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "file_upload_send"
            && runway
                .current_path
                .contains("live single-file SendAttachment/use_send_queue")
            && runway
                .current_path
                .contains("live confirmed failed-handoff Retry resubmit")
            && runway.current_path.contains("live timeline AbortLocalSend")
            && runway
                .current_path
                .contains("LocalSendAbortResult operation-strip bridge")
            && runway
                .current_path
                .contains("local accepted queue snapshot")
            && runway
                .current_path
                .contains("per-file accepted-send queue drilldown")
            && runway
                .current_path
                .contains("typed SDK queue contract packet")
            && runway.current_path.contains("timeline-cancel")
            && runway
                .remaining_gap
                .contains("accepted SDK queue retry/resume/abort/remove/reorder")
            && runway
                .next_ui_safe_step
                .contains("progress, result, delivery, and cancel contracts")
    }));
}

#[test]
fn hepta_telegram_base_attachment_timeline_cancel_local_send_uses_sdk_handle() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_TIMELINE_CANCEL_LOCAL_SEND_MARKER,
        "hepta_telegram_attachment_timeline_cancel_local_send_ready"
    );
    assert_eq!(
        crate::home::new_message_context_menu::MESSAGE_LOCAL_SEND_CANCEL_COMPACT_LABEL,
        "Cancel uses this local echo SendHandle only."
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_timeline_cancel_local_send_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment timeline local send cancel"
            && capability
                .base_module
                .contains("MessageAction::CancelLocalSend")
            && capability
                .base_module
                .contains("MatrixRequest::AbortLocalSend")
            && capability.notes.contains("local_echo_send_handle")
            && capability.notes.contains("SendHandle::abort")
            && capability
                .notes
                .contains("TimelineUpdate::LocalSendAbortResult")
            && capability.notes.contains("CancelledLocalEvent")
            && capability
                .notes
                .contains("already-sent/no-longer-cancellable")
            && capability
                .notes
                .contains("does not resubmit SendAttachment")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_attachment_queue_progress_result_taxonomy_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_QUEUE_PROGRESS_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_attachment_queue_progress_result_taxonomy_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(
        crate::room::room_input_bar::ATTACHMENT_QUEUE_PROGRESS_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("accepted queue/progress/result taxonomy packet")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_QUEUE_PROGRESS_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("MatrixRequest::SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_QUEUE_PROGRESS_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("TimelineUpdate::LocalSendAbortResult")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_QUEUE_PROGRESS_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("delivery receipt")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_QUEUE_PROGRESS_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("stale SendHandle")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_QUEUE_PROGRESS_RESULT_TAXONOMY_PACKET_LABEL
            .contains("timeline local echo Cancel")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_queue_progress_result_taxonomy_packet_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment queue progress result taxonomy packet"
            && capability
                .base_module
                .contains("taxonomy_attachment_file_button")
            && capability
                .base_module
                .contains("attachment_queue_progress_result_taxonomy_packet_label")
            && capability
                .notes
                .contains("accepted queue/progress/result taxonomy packet")
            && capability.notes.contains("MatrixRequest::SendAttachment")
            && capability
                .notes
                .contains("TimelineUpdate::LocalSendAbortResult")
            && capability.notes.contains("delivery receipt")
            && capability.notes.contains("stale SendHandle")
            && capability.notes.contains("not_wired")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("file_upload_send")
    }));
    let runway = HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY
        .iter()
        .find(|runway| runway.gap_id == "file_upload_send")
        .expect("file_upload_send runway should exist");
    assert!(
        runway
            .current_path
            .contains("accepted queue/progress/result taxonomy packet")
    );
    assert!(
        runway
            .next_ui_safe_step
            .contains("real accepted-queue controls")
    );
}

#[test]
fn hepta_telegram_base_attachment_status_taxonomy_is_local_contract() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_STATUS_TAXONOMY_LOCAL_MARKER,
        "hepta_telegram_attachment_status_taxonomy_local_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_STATUS_TAXONOMY_LOCAL_EVIDENCE
            .contains("review-pending")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_STATUS_TAXONOMY_LOCAL_EVIDENCE
            .contains("validation-held")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_STATUS_TAXONOMY_LOCAL_EVIDENCE
            .contains("handoff-submitted")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_STATUS_TAXONOMY_LOCAL_EVIDENCE
            .contains("queued-only is not delivery success")
    );
    assert!(crate::room::room_input_bar::ATTACHMENT_STATUS_TAXONOMY_LABEL.contains("retry-local"));
    assert!(
        crate::room::room_input_bar::ATTACHMENT_STATUS_TAXONOMY_LABEL
            .contains("retry-confirmation-open")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_STATUS_TAXONOMY_LABEL.contains("retry-confirmed")
    );
    assert!(crate::room::room_input_bar::ATTACHMENT_STATUS_TAXONOMY_LABEL.contains("cancel-local"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_status_taxonomy_local_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment status taxonomy local evidence"
            && capability.base_module.contains("taxonomy label")
            && capability.notes.contains("review-pending")
            && capability.notes.contains("handoff-submitted")
            && capability.notes.contains("retry-confirmed")
            && capability
                .notes
                .contains("queued-only never claims delivery success")
            && capability.notes.contains("caption-only SendMessage")
            && capability.notes.contains("SDK queue abort/remove/cancel")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_attachment_review_row_compact_fit_is_local_contract() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_REVIEW_ROW_COMPACT_FIT_MARKER,
        "hepta_telegram_attachment_review_row_compact_fit_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_ROW_COMPACT_FIT_EVIDENCE
            .contains("desktop and narrow mobile")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_ROW_COMPACT_FIT_EVIDENCE
            .contains("wrapping Fill/Fit text surfaces")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_ROW_COMPACT_FIT_EVIDENCE
            .contains("Send, Discard, Retry, and Cancel")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_ROW_COMPACT_FIT_EVIDENCE
            .contains("only review-row Send with pending state")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_REVIEW_ROW_COMPACT_FIT_LABEL
            .contains("no extra send or queue-cancel request")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"attachment_review_row_compact_fit_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment review row compact fit evidence"
            && capability.base_module.contains("attachment_review_actions")
            && capability.notes.contains("wrapping Fill/Fit labels")
            && capability.notes.contains("prevents overlap")
            && capability.notes.contains("MatrixRequest::SendAttachment")
            && capability.notes.contains("caption-only SendMessage")
            && capability.notes.contains("SDK queue abort/remove/cancel")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_attachment_mobile_picker_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_ATTACHMENT_MOBILE_PICKER_CONTROLS_MARKER,
        "hepta_telegram_composer_attachment_mobile_picker_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_PICKER_CONTROLS_EVIDENCE
            .contains("Gallery, Camera, Files, Contact, Thumbnail, and Share")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_PICKER_CONTROLS_EVIDENCE
            .contains("visible local mobile picker controls")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_PICKER_CONTROLS_EVIDENCE
            .contains("does not request camera permission")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_PICKER_CONTROLS_EVIDENCE
            .contains("files provider permission")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_PICKER_CONTROLS_EVIDENCE
            .contains("generate or decode thumbnails")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_PICKER_CONTROLS_EVIDENCE
            .contains("system share sheet")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_PICKER_CONTROLS_EVIDENCE
            .contains("share payloads")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_PICKER_CONTROLS_EVIDENCE
            .contains("MatrixRequest::SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_PICKER_CONTROLS_LABEL
            .contains("no permissions")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_mobile_picker_controls_row")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment mobile picker controls row"
            && capability
                .base_module
                .contains("attachment_mobile_picker_controls")
            && capability.notes.contains("Gallery")
            && capability.notes.contains("Camera")
            && capability.notes.contains("Files")
            && capability.notes.contains("Contact")
            && capability.notes.contains("Thumbnail")
            && capability.notes.contains("Share")
            && capability
                .notes
                .contains("local mobile-picker boundary handler")
            && capability.notes.contains("opens no mobile picker")
            && capability
                .notes
                .contains("generates or decodes no thumbnails")
            && capability.notes.contains("system share sheet")
            && capability.notes.contains("submits no SendAttachment")
            && capability.notes.contains("submits no SendMessage")
            && capability.notes.contains("SDK send queue")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_attachment_mobile_share_sheet_boundary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_MARKER,
        "hepta_telegram_attachment_mobile_share_sheet_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_mobile_share_sheet_boundary")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_EVIDENCE
            .contains("Share")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_EVIDENCE
            .contains("opens no system share sheet")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_EVIDENCE
            .contains("invokes no platform share extension")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_EVIDENCE
            .contains("creates no share payload")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_EVIDENCE
            .contains("submits no MatrixRequest::SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_SHARE_SHEET_BOUNDARY_LABEL
            .contains("no system share sheet")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment mobile share sheet boundary"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("share_attachment_mobile_button")
            && capability
                .notes
                .contains("visible local share-sheet boundary")
            && capability.notes.contains("opens no system share sheet")
            && capability.notes.contains("platform share extension")
            && capability.notes.contains("shared media")
            && capability.notes.contains("MatrixRequest::SendAttachment")
            && capability.notes.contains("MatrixRequest::SendMessage")
            && capability.notes.contains("SDK send-queue")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "file_upload_send"
            && runway.current_path.contains("share-sheet controls")
            && runway
                .remaining_gap
                .contains("real camera/contact capture/share")
    }));
}

#[test]
fn hepta_telegram_base_attachment_mobile_action_density_is_local_contract() {
    assert_eq!(
        HEPTA_TELEGRAM_ATTACHMENT_MOBILE_ACTION_DENSITY_MARKER,
        "hepta_telegram_attachment_mobile_action_density_ready"
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_ACTION_DENSITY_EVIDENCE
            .contains("narrow mobile")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_ACTION_DENSITY_EVIDENCE
            .contains("36px touch-height")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_ACTION_DENSITY_EVIDENCE
            .contains("Send, Discard, Retry, Cancel")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_ACTION_DENSITY_EVIDENCE
            .contains("no hidden overflow send affordance")
    );
    assert!(
        crate::room::room_input_bar::ATTACHMENT_MOBILE_ACTION_DENSITY_LABEL
            .contains("SDK queue-cancel behavior")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"attachment_mobile_action_density_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "attachment mobile action density evidence"
            && capability
                .base_module
                .contains("TelegramAttachmentOptionButton")
            && capability.notes.contains("36px touch-height")
            && capability
                .notes
                .contains("hidden overflow send affordances")
            && capability.notes.contains("MatrixRequest::SendAttachment")
            && capability.notes.contains("caption-only SendMessage")
            && capability.notes.contains("SDK send-queue")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live state")
    }));
}

#[test]
fn hepta_telegram_base_media_message_surface_has_confirmed_save_path() {
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_MESSAGE_LOCAL_SURFACE_MARKER,
        "hepta_telegram_media_message_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_MESSAGE_BLOCKED_ACTIONS_MARKER,
        "hepta_telegram_media_message_blocked_actions_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_ENCRYPTED_METADATA_LOCAL_MARKER,
        "hepta_telegram_media_encrypted_metadata_local_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"media_download_playback"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_message_preview_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_message_blocked_actions_evidence")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_save_play_confirmation_guard"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_download_metadata_preview"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"media_save_dialog_lifecycle_metadata_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_save_destination_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_encrypted_metadata_preview"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_encrypted_image_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media download/playback"
            && capability
                .notes
                .contains("Download and Play links guarded by PositiveConfirmationModal")
            && capability.notes.contains("MatrixRequest::SaveMedia")
            && capability.notes.contains("SDK media cache path")
            && capability.notes.contains("system opener")
            && capability.notes.contains("Encrypted image")
            && capability.notes.contains("richer playback remain TODO")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media download/playback metadata preview"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.notes.contains("loaded timeline media metadata")
            && capability
                .notes
                .contains("MIME type, size, duration, and dimensions")
            && capability.notes.contains("confirmation body and popup")
            && capability.notes.contains("no extra media fetch")
            && capability.notes.contains("MatrixRequest::SaveMedia")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media save dialog lifecycle metadata"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.notes.contains("confirmation opened")
            && capability.notes.contains("save dialog canceled")
            && capability.notes.contains("selected local save path")
            && capability.notes.contains("submit no SaveMedia")
            && capability.notes.contains("extra media fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media save destination metadata"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.notes.contains("selected destination path")
            && capability.notes.contains("loaded filename/type metadata")
            && capability.notes.contains("system opener")
            && capability.notes.contains("MatrixRequest::SaveMedia")
            && capability.notes.contains("retry/cancel queue controls")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media encrypted metadata local preview"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.notes.contains("already loaded filename")
            && capability.notes.contains("MIME type")
            && capability.notes.contains("duration")
            && capability.notes.contains("dimensions")
            && capability.notes.contains("Decrypt")
            && capability.notes.contains("SaveMedia")
            && capability.notes.contains("FetchMedia")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_media_download_playback_boundary_keeps_remaining_gaps() {
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_DOWNLOAD_PLAYBACK_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_media_download_playback_local_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"media_download_playback"));
    assert!(
        crate::home::room_screen::MEDIA_DOWNLOAD_PLAYBACK_LOCAL_BOUNDARY_EVIDENCE
            .contains("media_download_playback remains a base gap")
    );
    assert!(
        crate::home::room_screen::MEDIA_DOWNLOAD_PLAYBACK_LOCAL_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::SaveMedia")
    );
    assert!(
        crate::home::room_screen::MEDIA_DOWNLOAD_PLAYBACK_LOCAL_BOUNDARY_EVIDENCE
            .contains("PositiveConfirmationModal")
    );
    assert!(
        crate::home::room_screen::MEDIA_DOWNLOAD_PLAYBACK_LOCAL_BOUNDARY_EVIDENCE
            .contains("type, filename, MIME type, size, duration, and dimensions")
    );
    assert!(
        crate::home::room_screen::MEDIA_DOWNLOAD_PLAYBACK_METADATA_EVIDENCE
            .contains("loaded file/audio/video metadata")
    );
    assert!(
        crate::home::room_screen::MEDIA_DOWNLOAD_PLAYBACK_METADATA_EVIDENCE
            .contains("confirmation body and popup copy")
    );
    assert!(
        crate::home::room_screen::MEDIA_DOWNLOAD_PLAYBACK_METADATA_EVIDENCE
            .contains("no extra media fetch")
    );
    assert!(
        crate::home::room_screen::MEDIA_DOWNLOAD_PLAYBACK_METADATA_LABEL
            .contains("SaveMedia still waits for confirmation")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_EVIDENCE
            .contains("confirmation opened")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_EVIDENCE
            .contains("save dialog accepted")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_EVIDENCE
            .contains("selected path")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_EVIDENCE
            .contains("no extra media fetch")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_DIALOG_LIFECYCLE_METADATA_LABEL
            .contains("SaveMedia waits for a picked path")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_DESTINATION_METADATA_EVIDENCE
            .contains("selected local destination path")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_DESTINATION_METADATA_EVIDENCE
            .contains("MatrixRequest::SaveMedia")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_DESTINATION_METADATA_LABEL
            .contains("Selected save destination")
    );
    assert!(
        crate::home::room_screen::MEDIA_DOWNLOAD_PLAYBACK_LOCAL_BOUNDARY_EVIDENCE
            .contains("SDK media cache path")
    );
    assert!(
        crate::home::room_screen::MEDIA_DOWNLOAD_PLAYBACK_LOCAL_BOUNDARY_EVIDENCE
            .contains("selected path")
    );
    assert!(
        crate::home::room_screen::MEDIA_DOWNLOAD_PLAYBACK_LOCAL_BOUNDARY_EVIDENCE
            .contains("codec/transcode work")
    );
    assert!(
        crate::home::room_screen::MEDIA_DOWNLOAD_PLAYBACK_LOCAL_BOUNDARY_LABEL
            .contains("requires confirmation")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"media_download_playback_local_boundary_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media download/playback local boundary evidence"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("image/file/audio/video/encrypted media preview")
            && capability
                .notes
                .contains("media_download_playback remains a base gap")
            && capability.notes.contains("MatrixRequest::SaveMedia")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("SDK media cache path")
            && capability.notes.contains("selected path")
            && capability.notes.contains("system opener")
            && capability.notes.contains("Encrypted image")
            && capability.notes.contains("Decrypt")
            && capability.notes.contains("codec/transcode")
            && capability.notes.contains("inline audio/video player")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_media_metadata_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_METADATA_CLIPBOARD_MARKER,
        "hepta_telegram_media_metadata_clipboard_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"media_download_playback"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_metadata_clipboard_action"));
    assert!(
        crate::home::room_screen::MEDIA_METADATA_CLIPBOARD_EVIDENCE.contains("local clipboard")
    );
    assert!(
        crate::home::room_screen::MEDIA_METADATA_CLIPBOARD_EVIDENCE.contains("plain and encrypted")
    );
    assert!(
        crate::home::room_screen::MEDIA_METADATA_CLIPBOARD_EVIDENCE.contains("sends no FetchMedia")
    );
    assert!(crate::home::room_screen::MEDIA_METADATA_CLIPBOARD_EVIDENCE.contains("SaveMedia"));
    assert!(crate::home::room_screen::MEDIA_METADATA_CLIPBOARD_LABEL.contains("local clipboard"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media metadata clipboard"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.base_module.contains("Copy metadata")
            && capability.notes.contains("kind, filename, MIME type")
            && capability.notes.contains("encrypted file/audio/video")
            && capability.notes.contains("submits no FetchMedia")
            && capability.notes.contains("SaveMedia")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "media_download_playback"
            && runway.current_path.contains("metadata clipboard")
            && runway.remaining_gap.contains("true inline playback")
    }));
}

#[test]
fn hepta_telegram_base_media_inline_playback_queue_boundary_is_metadata_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_INLINE_PLAYBACK_QUEUE_BOUNDARY_MARKER,
        "hepta_telegram_media_inline_playback_queue_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"media_download_playback"));
    assert!(
        crate::home::room_screen::MEDIA_INLINE_PLAYBACK_QUEUE_BOUNDARY_EVIDENCE
            .contains("already loaded file/audio/video action metadata")
    );
    assert!(
        crate::home::room_screen::MEDIA_INLINE_PLAYBACK_QUEUE_BOUNDARY_EVIDENCE
            .contains("Download states that it only writes a picked local file")
    );
    assert!(
        crate::home::room_screen::MEDIA_INLINE_PLAYBACK_QUEUE_BOUNDARY_EVIDENCE
            .contains("Play states that it saves first")
    );
    assert!(
        crate::home::room_screen::MEDIA_INLINE_PLAYBACK_QUEUE_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::SaveMedia")
    );
    assert!(
        crate::home::room_screen::MEDIA_INLINE_PLAYBACK_QUEUE_BOUNDARY_EVIDENCE
            .contains("system opener")
    );
    assert!(
        crate::home::room_screen::MEDIA_INLINE_PLAYBACK_QUEUE_BOUNDARY_EVIDENCE
            .contains("inline audio/video controls")
    );
    assert!(
        crate::home::room_screen::MEDIA_INLINE_PLAYBACK_QUEUE_BOUNDARY_EVIDENCE
            .contains("retry/cancel queue controls")
    );
    assert!(
        crate::home::room_screen::MEDIA_INLINE_PLAYBACK_QUEUE_BOUNDARY_LABEL
            .contains("boundary metadata")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"media_inline_playback_queue_boundary_metadata")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media inline playback queue boundary metadata"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.base_module.contains("media action confirmation")
            && capability
                .notes
                .contains("loaded file/audio/video action metadata")
            && capability.notes.contains("picked local file")
            && capability.notes.contains("MatrixRequest::SaveMedia")
            && capability.notes.contains("system opener")
            && capability.notes.contains("Inline audio/video controls")
            && capability.notes.contains("retry/cancel queue controls")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation remain unwired")
    }));
}

#[test]
fn hepta_telegram_base_media_inline_player_disabled_controls_are_visible_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_INLINE_PLAYER_DISABLED_CONTROLS_MARKER,
        "hepta_telegram_media_inline_player_disabled_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"media_download_playback"));
    assert!(
        crate::home::room_screen::MEDIA_INLINE_PLAYER_DISABLED_CONTROLS_EVIDENCE
            .contains("audio and video timeline rows")
    );
    assert!(
        crate::home::room_screen::MEDIA_INLINE_PLAYER_DISABLED_CONTROLS_EVIDENCE
            .contains("visible disabled inline-player control strip")
    );
    assert!(
        crate::home::room_screen::MEDIA_INLINE_PLAYER_DISABLED_CONTROLS_EVIDENCE
            .contains("Playhead, Seek, Queue, Decrypt, and Codec")
    );
    assert!(
        crate::home::room_screen::MEDIA_INLINE_PLAYER_DISABLED_CONTROLS_EVIDENCE
            .contains("Download/Play remain the only active links")
    );
    assert!(
        crate::home::room_screen::MEDIA_INLINE_PLAYER_DISABLED_CONTROLS_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::MEDIA_INLINE_PLAYER_DISABLED_CONTROLS_LABEL
            .contains("Download/Play still confirm before SaveMedia")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_inline_player_disabled_controls")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media inline player disabled controls"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("media_inline_player_disabled_controls_preview")
            && capability.notes.contains("visible disabled inline-player")
            && capability
                .notes
                .contains("already loaded timeline metadata")
            && capability.notes.contains("Playhead")
            && capability.notes.contains("Download/Play")
            && capability.notes.contains("FetchMedia")
            && capability.notes.contains("SaveMedia")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_media_codec_transcode_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_CODEC_TRANSCODE_CONTROLS_MARKER,
        "hepta_telegram_media_codec_transcode_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"media_download_playback"));
    assert!(
        crate::home::room_screen::MEDIA_CODEC_TRANSCODE_CONTROLS_EVIDENCE
            .contains("Codec, Transcode, Captions, Quality, and Decrypt")
    );
    assert!(
        crate::home::room_screen::MEDIA_CODEC_TRANSCODE_CONTROLS_EVIDENCE
            .contains("already loaded media metadata")
    );
    assert!(
        crate::home::room_screen::MEDIA_CODEC_TRANSCODE_CONTROLS_EVIDENCE
            .contains("does not submit FetchMedia")
    );
    assert!(
        crate::home::room_screen::MEDIA_CODEC_TRANSCODE_CONTROLS_EVIDENCE
            .contains("start a decoder")
    );
    assert!(
        crate::home::room_screen::MEDIA_CODEC_TRANSCODE_CONTROLS_EVIDENCE
            .contains("start a transcoder")
    );
    assert!(
        crate::home::room_screen::MEDIA_CODEC_TRANSCODE_CONTROLS_LABEL
            .contains("local codec/transcode controls")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_codec_transcode_controls_row"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media codec transcode controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("media_codec_transcode_controls_preview")
            && capability
                .base_module
                .contains("handle_media_codec_transcode_control_link")
            && capability.notes.contains("Codec")
            && capability.notes.contains("Transcode")
            && capability.notes.contains("Captions")
            && capability.notes.contains("Quality")
            && capability.notes.contains("Decrypt")
            && capability.notes.contains("local link query")
            && capability.notes.contains("does not submit FetchMedia")
            && capability.notes.contains("start a decoder")
            && capability.notes.contains("start a transcoder")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_media_save_result_status_boundary_is_popup_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_SAVE_RESULT_STATUS_BOUNDARY_MARKER,
        "hepta_telegram_media_save_result_status_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"media_download_playback"));
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_STATUS_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::SaveMedia completion reports saved")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_STATUS_BOUNDARY_EVIDENCE
            .contains("download failed")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_STATUS_BOUNDARY_EVIDENCE
            .contains("system opener opened")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_STATUS_BOUNDARY_EVIDENCE
            .contains("opener failed")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_STATUS_BOUNDARY_EVIDENCE
            .contains("invalid saved-path")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_STATUS_BOUNDARY_EVIDENCE
            .contains("SaveMediaOpenOutcome")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_STATUS_BOUNDARY_EVIDENCE
            .contains("TimelineUpdate::MediaSaveResult")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_STATUS_BOUNDARY_EVIDENCE
            .contains("Open folder and Replay handoffs")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_STATUS_BOUNDARY_EVIDENCE
            .contains("retry/cancel queue controls")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_STATUS_BOUNDARY_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_STATUS_BOUNDARY_LABEL
            .contains("Save/open result popup maps")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_save_result_status_boundary"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media save result/open status boundary"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("media_save_result_status_boundary_label")
            && capability
                .notes
                .contains("saved, download failed, save failed")
            && capability.notes.contains("system opener opened")
            && capability.notes.contains("opener failed")
            && capability.notes.contains("invalid saved-path")
            && capability.notes.contains("SaveMediaOpenOutcome")
            && capability.notes.contains("TimelineUpdate::MediaSaveResult")
            && capability.notes.contains("Open folder and Replay handoffs")
            && capability.notes.contains("Inline audio/video player")
            && capability.notes.contains("retry/cancel queue controls")
            && capability.notes.contains("codec/transcode fallback")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_media_save_result_recovery_controls_row_has_guarded_retry() {
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_MARKER,
        "hepta_telegram_media_save_result_recovery_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_MARKER,
        "hepta_telegram_media_save_result_recovery_controls_row_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_CACHED_SAVED_FILE_STATUS_MARKER,
        "hepta_telegram_media_cached_saved_file_status_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"media_download_playback"));
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
            .contains("Open folder")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
            .contains("live local OS handoff")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
            .contains("cached-destination Open folder")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
            .contains("Replay")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE.contains(
            "live local OS handoff from the same cached successful SaveMedia destination"
        )
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
            .contains("Retry")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
            .contains("Queue")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
            .contains("local media playback/download queue snapshot")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
            .contains("cached saved-file status")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
            .contains("Queue clears")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
            .contains("Decrypt")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
            .contains("Retry is a guarded live resubmit")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
            .contains("PositiveConfirmationModal")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
            .contains("MatrixRequest::SaveMedia")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
            .contains("no unconfirmed FetchMedia")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_RESULT_RECOVERY_CONTROLS_ROW_LABEL
            .contains("cached SaveMedia destination")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_save_result_recovery_controls_row")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_cached_saved_file_status_snapshot")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_save_retry_confirmation_guard"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media save retry confirmation"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("handle_media_result_control_link")
            && capability
                .base_module
                .contains("handle_media_save_preflight_control_link")
            && capability.notes.contains("plain MXC source")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("MatrixRequest::SaveMedia")
            && capability.notes.contains("local unavailable label")
            && capability.notes.contains("no unconfirmed FetchMedia")
            && capability.notes.contains("queue resume/cancel")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("confirmed SaveMedia retry")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media save result recovery controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("media_save_result_recovery_controls_preview")
            && capability.notes.contains("Open folder")
            && capability.notes.contains("Replay")
            && capability
                .notes
                .contains("saved file is still a regular local file")
            && capability.notes.contains("Retry")
            && capability.notes.contains("Queue")
            && capability
                .notes
                .contains("local media playback/download queue snapshot")
            && capability.notes.contains("cached saved-file status")
            && capability.notes.contains("Decrypt")
            && capability
                .notes
                .contains("handle_media_result_control_link")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("MatrixRequest::SaveMedia")
            && capability.notes.contains("no unconfirmed FetchMedia")
            && capability.notes.contains("queue retry/resume/cancel")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("confirmed SaveMedia retry")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "media_download_playback"
            && runway
                .current_path
                .contains("guarded row-scoped SaveMedia Retry resubmit")
            && runway
                .current_path
                .contains("cached successful SaveMedia destination Open folder/Replay handoffs with stale cache validation/eviction")
            && runway
                .current_path
                .contains("local playback/download queue snapshot")
            && runway
                .current_path
                .contains("Queue cached saved-file metadata snapshot")
            && runway
                .current_path
                .contains("visible save/open recovery/preflight detail controls")
            && runway.remaining_gap.contains("playback progress subscription")
            && runway.remaining_gap.contains("queue retry/resume/cancel controls")
    }));
}

#[test]
fn hepta_telegram_base_media_cached_saved_file_status_snapshot_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_CACHED_SAVED_FILE_STATUS_MARKER,
        "hepta_telegram_media_cached_saved_file_status_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"media_download_playback"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_cached_saved_file_status_snapshot")
    );
    assert!(
        crate::home::room_screen::MEDIA_CACHED_SAVED_FILE_STATUS_EVIDENCE
            .contains("regular-file state")
    );
    assert!(
        crate::home::room_screen::MEDIA_CACHED_SAVED_FILE_STATUS_EVIDENCE
            .contains("clear the cached MXC destination")
    );
    assert!(
        crate::home::room_screen::MEDIA_CACHED_SAVED_FILE_STATUS_EVIDENCE.contains("no FetchMedia")
    );
    assert!(
        crate::home::room_screen::MEDIA_CACHED_SAVED_FILE_STATUS_LABEL
            .contains("local metadata only")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media cached saved-file status snapshot"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("media_cached_saved_file_status_label")
            && capability.notes.contains("regular-file state")
            && capability.notes.contains("modified timestamp seconds")
            && capability
                .notes
                .contains("clear the cached MXC destination")
            && capability.notes.contains("no FetchMedia")
            && capability.notes.contains("no SaveMedia")
            && capability.notes.contains("no system opener")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_media_save_preflight_detail_controls_row_has_guarded_retry() {
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_media_save_preflight_detail_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"media_download_playback"));
    assert!(
        crate::home::room_screen::MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("Request")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("Result")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("Error")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("Retry")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("Source")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("Retry is a guarded live resubmit")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("PositiveConfirmationModal")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("MatrixRequest::SaveMedia")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("no unconfirmed FetchMedia")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::MEDIA_SAVE_PREFLIGHT_DETAIL_CONTROLS_LABEL
            .contains("local SaveMedia preflight")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_save_preflight_detail_controls_row")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media save preflight detail controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("media_save_preflight_detail_controls_preview")
            && capability
                .notes
                .contains("Request, Result, Error, Retry, and Source")
            && capability
                .notes
                .contains("handle_media_save_preflight_control_link")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("MatrixRequest::SaveMedia")
            && capability.notes.contains("no unconfirmed FetchMedia")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("confirmed SaveMedia retry")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "media_download_playback"
            && runway
                .current_path
                .contains("guarded row-scoped SaveMedia Retry resubmit")
            && runway.current_path.contains("preflight detail controls")
            && runway
                .remaining_gap
                .contains("queue retry/resume/cancel controls")
    }));
}

#[test]
fn hepta_telegram_base_media_operation_packet_drilldown_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_OPERATION_PACKET_DRILLDOWN_MARKER,
        "hepta_telegram_media_operation_packet_drilldown_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"media_download_playback"));
    assert!(
        crate::home::room_screen::MEDIA_OPERATION_PACKET_DRILLDOWN_EVIDENCE
            .contains("local media operation acceptance matrix")
    );
    assert!(
        crate::home::room_screen::MEDIA_OPERATION_PACKET_DRILLDOWN_EVIDENCE
            .contains("plain and encrypted file/audio/video")
    );
    assert!(
        crate::home::room_screen::MEDIA_OPERATION_PACKET_DRILLDOWN_EVIDENCE
            .contains("SaveMedia request/result shape")
    );
    assert!(
        crate::home::room_screen::MEDIA_OPERATION_PACKET_DRILLDOWN_EVIDENCE
            .contains("decrypt/decode slot")
    );
    assert!(
        crate::home::room_screen::MEDIA_OPERATION_PACKET_DRILLDOWN_EVIDENCE
            .contains("retry/cancel queue mutation")
    );
    assert!(
        crate::home::room_screen::MEDIA_OPERATION_PACKET_DRILLDOWN_LABEL
            .contains("acceptance criteria")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_operation_packet_drilldown_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media operation packet drilldown"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("media_operation_packet_payload")
            && capability.notes.contains("Packet")
            && capability
                .notes
                .contains("local media operation acceptance matrix")
            && capability.notes.contains("inline playback slot")
            && capability.notes.contains("decrypt/decode slot")
            && capability.notes.contains("codec/transcode slot")
            && capability.notes.contains("queue retry/resume/cancel slot")
            && capability.notes.contains("submits no FetchMedia")
            && capability.notes.contains("SaveMedia")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "media_download_playback"
            && runway
                .current_path
                .contains("media operation packet drilldown")
            && runway
                .current_path
                .contains("typed playback/media queue contract packet")
            && runway
                .current_path
                .contains("cached successful SaveMedia destination Open folder/Replay handoffs with stale cache validation/eviction")
            && runway.next_ui_safe_step.contains("typed playback progress")
    }));
}

#[test]
fn hepta_telegram_base_media_playback_queue_contract_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_MARKER,
        "hepta_telegram_media_playback_queue_contract_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"media_download_playback"));
    assert!(
        crate::home::room_screen::MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_EVIDENCE
            .contains("typed playback/media queue contract")
    );
    assert!(
        crate::home::room_screen::MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_EVIDENCE
            .contains("inline playback request/result/error/progress")
    );
    assert!(
        crate::home::room_screen::MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_EVIDENCE
            .contains("decrypt/decode request/result/error")
    );
    assert!(
        crate::home::room_screen::MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_EVIDENCE
            .contains("codec/transcode/captions/quality fallback")
    );
    assert!(
        crate::home::room_screen::MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_EVIDENCE
            .contains("queue retry/resume/cancel/background persistence")
    );
    assert!(
        crate::home::room_screen::MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_EVIDENCE
            .contains("stale local file handling")
    );
    assert!(
        crate::home::room_screen::MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_EVIDENCE
            .contains("submits no FetchMedia")
    );
    assert!(
        crate::home::room_screen::MEDIA_PLAYBACK_QUEUE_CONTRACT_PACKET_LABEL
            .contains("typed playback/decrypt/codec/opener/queue")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"media_playback_queue_contract_packet_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media playback queue contract packet"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("media_playback_queue_contract_payload")
            && capability.notes.contains("Contract")
            && capability.notes.contains("typed slots")
            && capability
                .notes
                .contains("inline playback request/result/error/progress")
            && capability
                .notes
                .contains("decrypt/decode request/result/error")
            && capability
                .notes
                .contains("cached Open folder/Replay destination result with stale cache validation and eviction")
            && capability
                .notes
                .contains("broader stale local file handling beyond cached Open folder/Replay validation")
            && capability.notes.contains("submits no FetchMedia")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "media_download_playback"
            && runway
                .current_path
                .contains("typed playback/media queue contract packet")
            && runway
                .current_path
                .contains("live system opener outcome popup mapping")
            && runway.remaining_gap.contains("true inline playback")
            && runway
                .next_ui_safe_step
                .contains("backend/media adapter exposes typed playback progress")
    }));
}

#[test]
fn hepta_telegram_base_media_playback_result_taxonomy_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_media_playback_result_taxonomy_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"media_download_playback"));
    assert!(
        crate::home::room_screen::MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("decrypt/decode/opener/queue result taxonomy")
    );
    assert!(
        crate::home::room_screen::MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("confirmed MatrixRequest::SaveMedia")
    );
    assert!(
        crate::home::room_screen::MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("cached Open folder/Replay")
    );
    assert!(
        crate::home::room_screen::MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("not-assigned/not-wired result slots")
    );
    assert!(
        crate::home::room_screen::MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("submits no FetchMedia")
    );
    assert!(
        crate::home::room_screen::MEDIA_PLAYBACK_RESULT_TAXONOMY_PACKET_LABEL
            .contains("decrypt/decode/opener/queue result slots")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"media_playback_result_taxonomy_packet_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media playback result taxonomy packet"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("media_playback_result_taxonomy_payload")
            && capability.notes.contains("Taxonomy")
            && capability.notes.contains("MatrixRequest::FetchMedia")
            && capability.notes.contains("MatrixRequest::SaveMedia")
            && capability.notes.contains("cached Open folder/Replay")
            && capability
                .notes
                .contains("not-assigned/not-wired result slots")
            && capability.notes.contains("decrypt retry")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "media_download_playback"
            && runway
                .current_path
                .contains("decrypt/decode/opener/queue result taxonomy packet")
            && runway
                .current_path
                .contains("typed playback/media queue contract packet")
            && runway.remaining_gap.contains("decrypt/decode")
            && runway.next_ui_safe_step.contains("decrypt/decode result")
    }));
}

#[test]
fn hepta_telegram_base_media_encrypted_metadata_preview_stays_local_disabled() {
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_ENCRYPTED_METADATA_LOCAL_MARKER,
        "hepta_telegram_media_encrypted_metadata_local_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"media_download_playback"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_encrypted_metadata_preview"));
    assert!(
        crate::home::room_screen::MEDIA_ENCRYPTED_METADATA_LOCAL_EVIDENCE
            .contains("encrypted File, Audio, and Video")
    );
    assert!(
        crate::home::room_screen::MEDIA_ENCRYPTED_METADATA_LOCAL_EVIDENCE
            .contains("already loaded timeline metadata")
    );
    assert!(
        crate::home::room_screen::MEDIA_ENCRYPTED_METADATA_LOCAL_EVIDENCE
            .contains("filename, MIME type, size, duration, and dimensions")
    );
    assert!(
        crate::home::room_screen::MEDIA_ENCRYPTED_METADATA_LOCAL_EVIDENCE
            .contains("does not start decrypt")
    );
    assert!(
        crate::home::room_screen::MEDIA_ENCRYPTED_METADATA_LOCAL_EVIDENCE.contains("SaveMedia")
    );
    assert!(
        crate::home::room_screen::MEDIA_ENCRYPTED_METADATA_LOCAL_EVIDENCE.contains("FetchMedia")
    );
    assert!(
        crate::home::room_screen::MEDIA_ENCRYPTED_METADATA_LOCAL_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::MEDIA_ENCRYPTED_METADATA_LOCAL_LABEL
            .contains("Save/Play stay disabled")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media encrypted metadata local preview"
            && capability
                .base_module
                .contains("encrypted file/audio/video")
            && capability.notes.contains("Download, Play, Decrypt")
            && capability.notes.contains("SaveMedia")
            && capability.notes.contains("FetchMedia")
            && capability.notes.contains("retry/cancel queue control")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_media_encrypted_image_metadata_preview_stays_local_disabled() {
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_ENCRYPTED_IMAGE_METADATA_LOCAL_MARKER,
        "hepta_telegram_media_encrypted_image_metadata_local_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"media_download_playback"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_encrypted_image_metadata_preview")
    );
    assert!(
        crate::home::room_screen::MEDIA_ENCRYPTED_IMAGE_METADATA_LOCAL_EVIDENCE
            .contains("encrypted Image message rows")
    );
    assert!(
        crate::home::room_screen::MEDIA_ENCRYPTED_IMAGE_METADATA_LOCAL_EVIDENCE
            .contains("already loaded ImageInfo metadata")
    );
    assert!(
        crate::home::room_screen::MEDIA_ENCRYPTED_IMAGE_METADATA_LOCAL_EVIDENCE
            .contains("MIME type, size, dimensions, blurhash availability")
    );
    assert!(
        crate::home::room_screen::MEDIA_ENCRYPTED_IMAGE_METADATA_LOCAL_EVIDENCE
            .contains("thumbnail-source availability")
    );
    assert!(
        crate::home::room_screen::MEDIA_ENCRYPTED_IMAGE_METADATA_LOCAL_EVIDENCE
            .contains("does not start decrypt")
    );
    assert!(
        crate::home::room_screen::MEDIA_ENCRYPTED_IMAGE_METADATA_LOCAL_EVIDENCE
            .contains("image decode")
    );
    assert!(
        crate::home::room_screen::MEDIA_ENCRYPTED_IMAGE_METADATA_LOCAL_EVIDENCE
            .contains("FetchMedia")
    );
    assert!(
        crate::home::room_screen::MEDIA_ENCRYPTED_IMAGE_METADATA_LOCAL_LABEL
            .contains("decrypt and image decode stay disabled")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media encrypted image metadata local preview"
            && capability.base_module.contains("encrypted image")
            && capability.notes.contains("already loaded ImageInfo")
            && capability.notes.contains("blurhash availability")
            && capability.notes.contains("thumbnail-source availability")
            && capability.notes.contains("Decrypt")
            && capability.notes.contains("SaveMedia")
            && capability.notes.contains("FetchMedia")
            && capability.notes.contains("media cache mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_media_fetch_cache_read_path_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MEDIA_FETCH_CACHE_READ_MARKER,
        "hepta_telegram_media_fetch_cache_read_ready"
    );
    assert!(
        crate::media_cache::MEDIA_FETCH_CACHE_READ_EVIDENCE
            .contains("Matrix FetchMedia read request only for a missing MXC")
    );
    assert!(
        crate::media_cache::MEDIA_FETCH_CACHE_READ_EVIDENCE
            .contains("TimelineUpdate::MediaFetched")
    );
    assert!(
        crate::media_cache::MEDIA_FETCH_CACHE_READ_EVIDENCE
            .contains("SignalToUI only update local media cache/redraw state")
    );
    assert!(
        crate::media_cache::MEDIA_FETCH_CACHE_READ_EVIDENCE
            .contains("no manual Download, Play, Decrypt")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"media_fetch_cache_read_evidence"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "media fetch/cache read path"
            && capability.base_module == "MediaCache + RoomScreen image preview"
            && capability
                .notes
                .contains("existing Matrix FetchMedia read/cache path")
            && capability
                .notes
                .contains("MediaCache::try_get_media_or_fetch")
            && capability.notes.contains("missing MXC thumbnail/full-file")
            && capability.notes.contains("TimelineUpdate::MediaFetched")
            && capability.notes.contains("SignalToUI")
            && capability.notes.contains("manual Download, Play, Decrypt")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
    }));
}

#[test]
fn hepta_telegram_base_poll_message_preview_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_POLL_MESSAGE_PREVIEW_LOCAL_MARKER,
        "hepta_telegram_poll_message_preview_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_POLL_ANSWER_PREVIEW_RESULT_PACKET_MARKER,
        "hepta_telegram_poll_answer_preview_result_packet_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"poll_answer_preview_result_packet_preview")
    );
    assert!(
        crate::home::room_screen::POLL_MESSAGE_PREVIEW_READ_EVIDENCE
            .contains("already loaded matrix-sdk-ui PollState results")
    );
    assert!(
        crate::home::room_screen::POLL_MESSAGE_PREVIEW_READ_EVIDENCE
            .contains("populate_poll_message_content")
    );
    assert!(
        crate::home::room_screen::POLL_MESSAGE_PREVIEW_READ_EVIDENCE
            .contains("no poll response, edit, redact")
    );
    assert!(
        crate::home::room_screen::POLL_MESSAGE_PREVIEW_READ_EVIDENCE.contains("timeline reload")
    );
    assert!(
        crate::home::room_screen::POLL_ANSWER_PREVIEW_RESULT_PACKET_EVIDENCE
            .contains("answer edit slot")
    );
    assert!(
        crate::home::room_screen::POLL_ANSWER_PREVIEW_RESULT_PACKET_EVIDENCE
            .contains("already loaded PollState only")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "poll message preview"
            && capability.base_module == "RoomScreen Message + matrix-sdk-ui PollState"
            && capability
                .notes
                .contains("render as first-class message rows")
            && capability
                .notes
                .contains("already loaded matrix-sdk-ui PollState results")
            && capability
                .notes
                .contains("Poll answer preview/result packet")
            && capability.notes.contains("answer_edit_slot not_built")
            && capability.notes.contains("vote_response_slot not_sent")
            && capability
                .notes
                .contains("result_mapping read_only_loaded_pollstate")
            && capability.notes.contains("populate_poll_message_content")
            && capability.notes.contains("no poll response")
            && capability.notes.contains("poll answer edit")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("timeline reload")
    }));
}

#[test]
fn hepta_telegram_base_image_viewer_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_IMAGE_VIEWER_LOCAL_CONTROLS_MARKER,
        "hepta_telegram_image_viewer_local_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"media_download_playback"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"image_viewer_local_controls"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "image viewer local controls"
            && capability.base_module == "ImageViewer + RoomScreen image click"
            && capability.notes.contains("existing media cache/fetch path")
            && capability
                .notes
                .contains("Close, Escape, background tap, Zoom, Rotate, Reset")
            && capability.notes.contains("pan, pinch")
            && capability.notes.contains("overlay auto-hide")
            && capability.notes.contains("only update local viewer state")
            && capability.notes.contains("no additional FetchMedia")
            && capability.notes.contains("download")
            && capability.notes.contains("playback")
            && capability.notes.contains("decrypt")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_link_preview_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_LINK_PREVIEW_LOCAL_CONTROLS_MARKER,
        "hepta_telegram_link_preview_local_controls_ready"
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"get_url_preview"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"link_preview_local_controls"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "link preview local controls"
            && capability.base_module == "LinkPreview + LinkPreviewCache"
            && capability
                .notes
                .contains("existing Matrix GetUrlPreview cache/fetch path")
            && capability.notes.contains("Show more, Show fewer")
            && capability.notes.contains("title tap dispatch")
            && capability.notes.contains("matrix.to filtering")
            && capability.notes.contains("cache-hit reuse")
            && capability
                .notes
                .contains("local LinkPreview widget/cache state")
            && capability.notes.contains("no extra GetUrlPreview")
            && capability.notes.contains("Matrix alias resolution")
            && capability.notes.contains("room preview fetch")
            && capability.notes.contains("event context")
            && capability.notes.contains("external browser handoff")
            && capability.notes.contains("media download")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_link_preview_loaded_metadata_summary_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_LINK_PREVIEW_LOADED_METADATA_MARKER,
        "hepta_telegram_link_preview_loaded_metadata_ready"
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"get_url_preview"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"link_preview_loaded_metadata_summary"));
    assert!(
        crate::home::link_preview::LINK_PREVIEW_LOADED_METADATA_EVIDENCE
            .contains("already loaded GetUrlPreview metadata")
    );
    assert!(
        crate::home::link_preview::LINK_PREVIEW_LOADED_METADATA_EVIDENCE
            .contains("title, site name, description")
    );
    assert!(
        crate::home::link_preview::LINK_PREVIEW_LOADED_METADATA_EVIDENCE
            .contains("image MIME type, image dimensions, and image size")
    );
    assert!(
        crate::home::link_preview::LINK_PREVIEW_LOADED_METADATA_EVIDENCE
            .contains("passes loaded og:image width/height into ImageInfo")
    );
    assert!(
        crate::home::link_preview::LINK_PREVIEW_LOADED_METADATA_EVIDENCE
            .contains("no extra GetUrlPreview")
    );
    assert!(
        crate::home::link_preview::LINK_PREVIEW_LOADED_METADATA_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::link_preview::LINK_PREVIEW_LOADED_METADATA_LABEL
            .contains("no extra preview fetch")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "link preview loaded metadata summary"
            && capability.base_module == "LinkPreview populate_view + LinkPreviewData"
            && capability.notes.contains("LoadedLinkPreview rows")
            && capability
                .notes
                .contains("already loaded GetUrlPreview metadata")
            && capability.notes.contains("image dimensions")
            && capability.notes.contains("og:image width/height")
            && capability.notes.contains("no extra GetUrlPreview")
            && capability.notes.contains("Matrix alias resolution")
            && capability.notes.contains("room preview fetch")
            && capability.notes.contains("event context")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_url_preview_read_path_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_URL_PREVIEW_READ_MARKER,
        "hepta_telegram_url_preview_read_ready"
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"url_preview_read"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"url_preview_read_evidence"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "URL preview read/cache path"
            && capability.base_module == "LinkPreview + LinkPreviewCache"
            && capability
                .notes
                .contains("existing Matrix GetUrlPreview read request only")
            && capability
                .notes
                .contains("missing accepted URL cache entry")
            && capability.notes.contains("LoadedLinkPreview")
            && capability.notes.contains("Requested")
            && capability.notes.contains("Failed")
            && capability.notes.contains("cleanup")
            && capability.notes.contains("rate-limit retry scheduling")
            && capability.notes.contains("insert_into_cache")
            && capability
                .notes
                .contains("TimelineUpdate::LinkPreviewFetched")
            && capability.notes.contains("SignalToUI")
            && capability
                .notes
                .contains("local URL preview cache/redraw state")
            && capability.notes.contains("Matrix alias resolution")
            && capability.notes.contains("room preview fetch")
            && capability.notes.contains("event context")
            && capability.notes.contains("media download")
            && capability.notes.contains("browser handoff")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("account/profile")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_preview_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_LOCAL_PREVIEW_MARKER,
        "hepta_telegram_matrix_link_local_preview_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_LIVE_READ_WIRING_MARKER,
        "hepta_telegram_matrix_link_preview_live_read_wiring_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_LOADED_ALIAS_NAVIGATION_MARKER,
        "hepta_telegram_matrix_link_loaded_alias_navigation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_LOADED_EVENT_LOCAL_JUMP_MARKER,
        "hepta_telegram_matrix_link_loaded_event_local_jump_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_CURRENT_ROOM_EVENT_PAGINATION_LIVE_MARKER,
        "hepta_telegram_matrix_link_current_room_event_pagination_live_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_MARKER,
        "hepta_telegram_matrix_link_loaded_event_source_modal_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_UNKNOWN_TARGET_LOCAL_MARKER,
        "hepta_telegram_matrix_link_unknown_target_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_TARGET_METADATA_MARKER,
        "hepta_telegram_matrix_link_target_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_RESULT_METADATA_MARKER,
        "hepta_telegram_matrix_link_preview_result_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_FAILURE_METADATA_MARKER,
        "hepta_telegram_matrix_link_preview_failure_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_RETRY_CONFIRMATION_MARKER,
        "hepta_telegram_matrix_link_preview_retry_confirmation_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_MARKER,
        "hepta_telegram_matrix_link_server_context_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"matrix_link_resolution"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_preview_surface"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_preview_live_read_wiring"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_loaded_alias_navigation"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_loaded_event_local_jump"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"matrix_link_current_room_event_pagination_live")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_loaded_event_context_metadata")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"matrix_link_loaded_event_source_modal_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_unknown_target_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"matrix_link_unknown_target_boundary_evidence")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_target_metadata_preview"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_preview_result_metadata"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_preview_failure_metadata"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"matrix_link_preview_retry_confirmation_guard")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_server_context_boundary"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_browser_handoff_confirmation")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link preview/navigation"
            && capability
                .notes
                .contains("known room links on local room navigation")
            && capability
                .notes
                .contains("loaded room alias links on local RoomsList alias navigation")
            && capability
                .notes
                .contains("known user links on the profile pane handoff")
            && capability.notes.contains("existing profile read path")
            && capability.notes.contains("current-room event links")
            && capability.notes.contains("loaded local jump")
            && capability
                .notes
                .contains("BackwardsPaginateUntilEvent/PaginateTimeline")
            && capability.notes.contains("RoomScreen tl_state")
            && capability.notes.contains("PreviewMatrixLinkTarget")
            && capability.notes.contains("compact room preview")
            && capability.notes.contains("link parsing")
            && capability.notes.contains("preview staging")
            && capability.notes.contains("known-room navigation")
            && capability.notes.contains("loaded-alias navigation")
            && capability.notes.contains("profile-pane handoff")
            && capability.notes.contains("loaded-event local jump")
            && capability.notes.contains("current-room event pagination")
            && capability.notes.contains("get_room_preview")
            && capability.notes.contains("Room::load_or_fetch_event")
            && capability.notes.contains("external browser handoff")
            && capability
                .notes
                .contains("server-side event context window fetch")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_preview_live_read_wiring_is_partial_live() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_LIVE_READ_WIRING_MARKER,
        "hepta_telegram_matrix_link_preview_live_read_wiring_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_preview_live_read_wiring"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link compact preview live read wiring"
            && capability
                .base_module
                .contains("show_matrix_link_preview_request")
            && capability
                .base_module
                .contains("MatrixRequest::PreviewMatrixLinkTarget")
            && capability
                .base_module
                .contains("TimelineUpdate::MatrixLinkPreviewResult")
            && capability.notes.contains("fetch_room_preview_with_avatar")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("partial-live path")
            && capability
                .notes
                .contains("server-side alias route promotion")
            && capability.notes.contains("Room::load_or_fetch_event")
            && capability
                .notes
                .contains("server-side event context window fetch")
            && capability.notes.contains("join")
            && capability.notes.contains("browser handoff")
            && capability.notes.contains("gateway/runtime/auth/provider")
            && capability.notes.contains("Telegram delivery")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "matrix_link_resolution"
            && runway
                .current_path
                .contains("BackwardsPaginateUntilEvent/PaginateTimeline read wiring")
            && runway
                .current_path
                .contains("live compact PreviewMatrixLinkTarget read/result wiring")
            && runway.current_path.contains("confirmed failed-state Retry")
            && runway
                .current_path
                .contains("cached Server context refresh")
            && runway
                .remaining_gap
                .contains("full non-current-room event-context window")
            && runway.next_ui_safe_step.contains("route-result adapter")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_current_room_event_pagination_is_live() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_CURRENT_ROOM_EVENT_PAGINATION_LIVE_MARKER,
        "hepta_telegram_matrix_link_current_room_event_pagination_live_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_CURRENT_ROOM_EVENT_PAGINATION_LIVE_EVIDENCE
            .contains("BackwardsPaginateUntilEvent")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_CURRENT_ROOM_EVENT_PAGINATION_LIVE_EVIDENCE
            .contains("MatrixRequest::PaginateTimeline")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_CURRENT_ROOM_EVENT_PAGINATION_LIVE_EVIDENCE
            .contains("TargetEventFound")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"matrix_link_current_room_event_pagination_live")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link current-room event pagination"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("paginate_current_room_matrix_link_event")
            && capability
                .base_module
                .contains("BackwardsPaginateUntilEventRequest")
            && capability
                .base_module
                .contains("MatrixRequest::PaginateTimeline")
            && capability
                .notes
                .contains("Missing current-room Matrix event links")
            && capability.notes.contains("TargetEventFound")
            && capability.notes.contains("current-room-only")
            && capability.notes.contains("server-side event context fetch")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_loaded_alias_navigation_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_LOADED_ALIAS_NAVIGATION_MARKER,
        "hepta_telegram_matrix_link_loaded_alias_navigation_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_ALIAS_LOCAL_NAVIGATION_EVIDENCE
            .contains("canonical_alias")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_ALIAS_LOCAL_NAVIGATION_EVIDENCE
            .contains("alt_aliases")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_ALIAS_LOCAL_NAVIGATION_EVIDENCE
            .contains("NavigateToRoom")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_ALIAS_LOCAL_NAVIGATION_EVIDENCE
            .contains("MatrixRequest::PreviewMatrixLinkTarget")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_ALIAS_LOCAL_NAVIGATION_EVIDENCE
            .contains("server-side alias resolution")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_ALIAS_LOCAL_NAVIGATION_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_ALIAS_LOCAL_NAVIGATION_EVIDENCE
            .contains("live mutation request")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_loaded_alias_navigation"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link loaded alias navigation"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.notes.contains("canonical_alias")
            && capability.notes.contains("alt_aliases")
            && capability.notes.contains("NavigateToRoom")
            && capability.notes.contains("PreviewMatrixLinkTarget")
            && capability.notes.contains("server-side alias resolution")
            && capability.notes.contains("join")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_unknown_target_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_UNKNOWN_TARGET_LOCAL_MARKER,
        "hepta_telegram_matrix_link_unknown_target_local_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"matrix_link_resolution"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_loaded_alias_navigation"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_loaded_event_local_jump"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_unknown_target_local_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link preview/navigation"
            && capability
                .notes
                .contains("BackwardsPaginateUntilEvent/PaginateTimeline")
            && capability.notes.contains("non-current-room event links")
            && capability.notes.contains("PreviewMatrixLinkTarget")
            && capability.notes.contains("compact room preview")
            && capability.notes.contains("Room::load_or_fetch_event")
            && capability.notes.contains("event context window fetch")
            && capability.notes.contains("external browser handoff")
            && capability.notes.contains("message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_unknown_target_boundary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_UNKNOWN_TARGET_BOUNDARY_MARKER,
        "hepta_telegram_matrix_link_unknown_target_boundary_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_UNKNOWN_TARGET_LOCAL_BOUNDARY_EVIDENCE
            .contains("compact Matrix link preview read")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_UNKNOWN_TARGET_LOCAL_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::PreviewMatrixLinkTarget")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_EVENT_LOCAL_JUMP_EVIDENCE
            .contains("already loaded in RoomScreen tl_state")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_EVENT_LOCAL_JUMP_EVIDENCE
            .contains("BackwardsPaginateUntilEvent/PaginateTimeline read path")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_UNKNOWN_TARGET_LOCAL_BOUNDARY_EVIDENCE
            .contains("Server-side event context")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_UNKNOWN_TARGET_LOCAL_BOUNDARY_EVIDENCE
            .contains("Cached room id or alias targets can be refreshed")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_UNKNOWN_TARGET_LOCAL_BOUNDARY_LABEL
            .contains("confirmed room-or-alias Join/Knock")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"matrix_link_unknown_target_boundary_evidence")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_loaded_alias_navigation"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_loaded_event_local_jump"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link unknown target boundary evidence"
            && capability.base_module.contains("unknown Matrix")
            && capability.notes.contains("matrix_link_resolution")
            && capability.notes.contains("non-current-room event links")
            && capability
                .notes
                .contains("MatrixRequest::PreviewMatrixLinkTarget")
            && capability.notes.contains("room preview details")
            && capability.notes.contains("cached Server context")
            && capability.notes.contains("Server-side event context")
            && capability.notes.contains("Cached room id or alias targets")
            && capability.notes.contains("JoinRoomByIdOrAlias")
            && capability.notes.contains("MatrixRequest::Knock")
            && capability.notes.contains("MatrixRequest::InviteUser")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_loaded_event_context_metadata_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_LOADED_EVENT_CONTEXT_METADATA_MARKER,
        "hepta_telegram_matrix_link_loaded_event_context_metadata_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_EVENT_CONTEXT_METADATA_EVIDENCE
            .contains("already loaded timeline row")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_EVENT_CONTEXT_METADATA_EVIDENCE
            .contains("target event id")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_EVENT_CONTEXT_METADATA_EVIDENCE
            .contains("loaded item index")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_EVENT_CONTEXT_METADATA_EVIDENCE
            .contains("loaded plaintext snippet")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_EVENT_CONTEXT_METADATA_EVIDENCE
            .contains("preview-strip source affordance")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_EVENT_CONTEXT_METADATA_EVIDENCE
            .contains("MatrixRequest::BackwardsPaginateUntilEvent")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_EVENT_CONTEXT_METADATA_EVIDENCE
            .contains("PreviewMatrixLinkTarget follow-up")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_EVENT_CONTEXT_METADATA_LABEL
            .contains("no event-context fetch")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_loaded_event_context_metadata")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link loaded event context metadata"
            && capability
                .base_module
                .contains("jump_to_loaded_matrix_link_event")
            && capability
                .notes
                .contains("already loaded RoomScreen tl_state row")
            && capability.notes.contains("target event id")
            && capability.notes.contains("loaded item index")
            && capability.notes.contains("current-room relation")
            && capability.notes.contains("compact plaintext snippet")
            && capability
                .notes
                .contains("visible preview-strip Source affordance")
            && capability
                .notes
                .contains("MatrixRequest::BackwardsPaginateUntilEvent")
            && capability
                .notes
                .contains("PreviewMatrixLinkTarget follow-up")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("timeline pagination/reload")
            && capability.notes.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_loaded_event_source_modal_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_MARKER,
        "hepta_telegram_matrix_link_loaded_event_source_modal_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_EVIDENCE
            .contains("EventSourceModal")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_EVIDENCE
            .contains("current-room Matrix event link")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_EVIDENCE
            .contains("EventTimelineItem.latest_json")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_EVIDENCE
            .contains("Missing, failed, unresolved")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_EVIDENCE
            .contains("Room::load_or_fetch_event")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_EVIDENCE
            .contains("Source-click follow-up request")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"matrix_link_loaded_event_source_modal_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link loaded event source modal"
            && capability
                .base_module
                .contains("open_telegram_matrix_link_loaded_event_source")
            && capability.notes.contains("EventSourceModal")
            && capability.notes.contains("current-room event")
            && capability.notes.contains("RoomScreen tl_state")
            && capability.notes.contains("EventTimelineItem.latest_json")
            && capability.notes.contains("Missing, failed, unresolved")
            && capability.notes.contains("Room::load_or_fetch_event")
            && capability
                .notes
                .contains("Source click sends no follow-up Matrix request")
            && capability.notes.contains("BackwardsPaginateUntilEvent")
            && capability.notes.contains("event-context window fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_target_metadata_is_loaded_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_TARGET_METADATA_MARKER,
        "hepta_telegram_matrix_link_target_metadata_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_TARGET_METADATA_EVIDENCE.contains("clicked MatrixId")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_TARGET_METADATA_EVIDENCE.contains("via server list")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_TARGET_METADATA_EVIDENCE
            .contains("loaded RoomsList room/alias state")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_TARGET_METADATA_EVIDENCE
            .contains("already loaded timeline event ids")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_TARGET_METADATA_EVIDENCE
            .contains("compact PreviewMatrixLinkTarget room-preview read")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_TARGET_METADATA_EVIDENCE
            .contains("no extra Matrix request")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_TARGET_METADATA_EVIDENCE
            .contains("server-side alias resolution")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_TARGET_METADATA_EVIDENCE
            .contains("event context fetch")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_TARGET_METADATA_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_TARGET_METADATA_LABEL
            .contains("Target metadata is loaded locally")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_target_metadata_preview"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link target metadata preview"
            && capability
                .base_module
                .contains("matrix_link_target_metadata_label")
            && capability.notes.contains("clicked MatrixId kind")
            && capability.notes.contains("via server count")
            && capability.notes.contains("current-room relation")
            && capability
                .notes
                .contains("loaded RoomsList room/alias state")
            && capability
                .notes
                .contains("loaded RoomScreen timeline event-id state")
            && capability
                .notes
                .contains("MatrixRequest::PreviewMatrixLinkTarget")
            && capability.notes.contains("no extra Matrix request")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_preview_result_metadata_is_fetched_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_RESULT_METADATA_MARKER,
        "hepta_telegram_matrix_link_preview_result_metadata_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_RESULT_METADATA_EVIDENCE
            .contains("PreviewMatrixLinkTarget get_room_preview read")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_RESULT_METADATA_EVIDENCE
            .contains("FetchedRoomPreview")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_RESULT_METADATA_EVIDENCE
            .contains("canonical alias")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_RESULT_METADATA_EVIDENCE
            .contains("join rule")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_RESULT_METADATA_EVIDENCE
            .contains("world-readable history")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_RESULT_METADATA_EVIDENCE
            .contains("Room::load_or_fetch_event")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_RESULT_METADATA_EVIDENCE
            .contains("event context window fetch")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_RESULT_METADATA_EVIDENCE
            .contains("server-side alias resolution")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_RESULT_METADATA_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_RESULT_METADATA_LABEL
            .contains("fetched room preview plus source-only status")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_preview_result_metadata"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link preview result metadata"
            && capability
                .base_module
                .contains("matrix_link_preview_result_metadata_label")
            && capability.notes.contains("FetchedRoomPreview")
            && capability.notes.contains("canonical alias")
            && capability.notes.contains("active member counts")
            && capability.notes.contains("join rule")
            && capability.notes.contains("world-readable history")
            && capability.notes.contains("Room::load_or_fetch_event")
            && capability.notes.contains("event context window fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_preview_failure_metadata_is_error_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_FAILURE_METADATA_MARKER,
        "hepta_telegram_matrix_link_preview_failure_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_RETRY_CONFIRMATION_MARKER,
        "hepta_telegram_matrix_link_preview_retry_confirmation_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_FAILURE_METADATA_EVIDENCE
            .contains("PreviewMatrixLinkTarget get_room_preview read returns an error")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_FAILURE_METADATA_EVIDENCE
            .contains("via server count")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_FAILURE_METADATA_EVIDENCE
            .contains("requested event-id state")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_FAILURE_METADATA_EVIDENCE
            .contains("error message length")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_FAILURE_METADATA_EVIDENCE
            .contains("PositiveConfirmationModal")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_RETRY_CONFIRMATION_EVIDENCE
            .contains("cached originating TimelineKind")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_RETRY_CONFIRMATION_EVIDENCE
            .contains("unavailable cached target")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_FAILURE_METADATA_EVIDENCE
            .contains("server-side alias resolution")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_FAILURE_METADATA_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_FAILURE_METADATA_LABEL
            .contains("Retry confirms")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_preview_failure_metadata"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"matrix_link_preview_retry_confirmation_guard")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link preview failure metadata"
            && capability
                .base_module
                .contains("matrix_link_preview_failure_metadata_label")
            && capability.notes.contains("failed PreviewMatrixLinkTarget")
            && capability.notes.contains("via server count")
            && capability.notes.contains("requested event-id state")
            && capability.notes.contains("error message length")
            && capability.notes.contains("no retry without confirmation")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation request")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link preview retry confirmation"
            && capability.base_module.contains("PositiveConfirmationModal")
            && capability.notes.contains("cached originating TimelineKind")
            && capability.notes.contains("PreviewMatrixLinkTarget")
            && capability.notes.contains("no automatic retry")
            && capability.notes.contains("external browser handoff")
            && capability.notes.contains("live mutation request")
    }));
}
