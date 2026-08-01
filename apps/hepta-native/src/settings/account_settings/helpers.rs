use super::*;

pub(super) fn account_profile_avatar_state_label(state: &AvatarState) -> &'static str {
    match state {
        AvatarState::Unknown => "avatar unknown",
        AvatarState::Known(Some(_)) => "avatar MXC known",
        AvatarState::Known(None) => "no avatar",
        AvatarState::Loaded(_) => "avatar loaded",
        AvatarState::Failed => "avatar fetch failed",
    }
}

pub(super) fn loaded_account_identity_label(profile: Option<&UserProfile>) -> String {
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

pub(super) fn account_device_directory_entry_label(entry: &AccountDeviceDirectoryEntry) -> String {
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

pub(super) fn account_device_directory_summary(entries: &[AccountDeviceDirectoryEntry]) -> String {
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

pub(super) fn account_management_device_directory_result_label(
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

pub(super) fn account_management_preview_state_label(
    state: AccountManagementPreviewState,
) -> &'static str {
    match state {
        AccountManagementPreviewState::Hidden => "hidden preview",
        AccountManagementPreviewState::Overview => "Manage Account overview",
        AccountManagementPreviewState::Security => "Security preview",
        AccountManagementPreviewState::Sessions => "Sessions preview",
        AccountManagementPreviewState::Refreshing => "Refresh session metadata",
    }
}

pub(super) fn account_management_lifecycle_metadata_label(
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

pub(super) fn account_management_refresh_confirmation_label(
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

pub(super) fn account_management_device_directory_retry_confirmation_label(
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

pub(super) fn account_management_device_rename_target(profile: Option<&UserProfile>) -> String {
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

pub(super) fn account_management_current_device_rename_confirmation_label(
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

pub(super) fn account_management_browser_portal_url_from_homeserver(
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

pub(super) fn account_management_browser_portal_url_from_client() -> Result<String, String> {
    let Some(client) = get_client() else {
        return Err("Matrix client unavailable; log in before opening account browser".to_string());
    };
    account_management_browser_portal_url_from_homeserver(client.homeserver().as_str())
}

pub(super) fn account_management_browser_portal_handoff_label(
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

pub(super) fn account_management_session_revoke_boundary_label(
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

pub(super) fn account_management_session_actions_row_label(
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

pub(super) fn account_management_device_directory_controls_row_label(
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

pub(super) fn account_management_preflight_detail_controls_row_label(
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

pub(super) fn account_management_session_device_drilldown_packet_label(
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

pub(super) fn account_management_session_device_typed_contract_packet_label(
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

pub(super) fn account_management_session_device_result_taxonomy_packet_label(
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

pub(super) fn account_management_request_snapshot_label(
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

pub(super) fn account_management_current_device_metadata_controls_row_label(
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

pub(super) fn account_management_verification_status_label(
    verification_state: VerificationState,
) -> &'static str {
    match verification_state {
        VerificationState::Verified => "verified",
        VerificationState::Unverified => "unverified",
        VerificationState::Unknown => "unknown verification",
    }
}

pub(super) fn account_management_current_device_verification_clipboard_payload(
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

pub(super) fn account_management_current_device_verification_clipboard_label(
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

pub(super) fn account_management_current_device_id_clipboard_payload(
    device_id: Option<&str>,
) -> Option<String> {
    device_id
        .map(str::trim)
        .filter(|device_id| !device_id.is_empty())
        .map(ToOwned::to_owned)
}

pub(super) fn account_management_current_device_id_clipboard_label(
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

pub(super) fn account_management_current_device_display_name_clipboard_payload(
    display_name: Option<&str>,
) -> Option<String> {
    display_name
        .map(str::trim)
        .filter(|display_name| !display_name.is_empty())
        .map(ToOwned::to_owned)
}

pub(super) fn account_management_current_device_display_name_clipboard_label(
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

pub(super) fn account_management_current_session_clipboard_payload(
    session_text: Option<&str>,
) -> Option<String> {
    session_text
        .map(str::trim)
        .filter(|session_text| !session_text.is_empty())
        .map(ToOwned::to_owned)
}

pub(super) fn account_management_current_session_clipboard_label(
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

pub(super) fn account_management_current_device_source_clipboard_payload(
    loaded_identity_text: Option<&str>,
) -> Option<String> {
    loaded_identity_text
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(ToOwned::to_owned)
}

pub(super) fn account_management_current_device_source_clipboard_label(
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
pub(super) enum AvatarUploadPickResult {
    Picked(PathBuf),
    Canceled,
    Unsupported,
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
pub(super) fn pick_account_avatar_file() -> AvatarUploadPickResult {
    rfd::FileDialog::new()
        .add_filter("Images", &["png", "jpg", "jpeg", "gif", "bmp", "webp"])
        .pick_file()
        .map(AvatarUploadPickResult::Picked)
        .unwrap_or(AvatarUploadPickResult::Canceled)
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub(super) fn pick_account_avatar_file() -> AvatarUploadPickResult {
    AvatarUploadPickResult::Unsupported
}

pub(super) fn account_avatar_mime_type(path: &Path) -> mime::Mime {
    mime_guess::from_path(path).first_or_octet_stream()
}

pub(super) fn validate_account_avatar_file(
    path: &Path,
    mime_type: &mime::Mime,
) -> Result<(), &'static str> {
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
pub(super) struct AvatarUploadSelectionPreview {
    pub(super) file_path: PathBuf,
    pub(super) mime: mime::Mime,
    pub(super) filename: String,
    pub(super) extension: String,
    pub(super) mime_type: String,
    pub(super) size_label: String,
    pub(super) dimensions_label: String,
}

impl AvatarUploadSelectionPreview {
    pub(super) fn summary(&self) -> String {
        format!(
            "{} · {} · {} · {} · {}",
            self.filename, self.mime_type, self.size_label, self.extension, self.dimensions_label
        )
    }
}

pub(super) fn account_avatar_upload_lifecycle_metadata_label(
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

pub(super) fn account_avatar_invalid_selection_metadata_summary(
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

pub(super) fn is_account_avatar_header_dimension_image_file(
    path: &Path,
    mime_type: &mime::Mime,
) -> bool {
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

pub(super) fn read_account_avatar_image_header_bytes(path: &Path) -> Option<Vec<u8>> {
    let mut file = fs::File::open(path).ok()?;
    let mut bytes = Vec::new();
    file.by_ref()
        .take(512 * 1024)
        .read_to_end(&mut bytes)
        .ok()?;
    Some(bytes)
}

pub(super) fn parse_account_avatar_png_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 24 || &bytes[0..8] != b"\x89PNG\r\n\x1a\n" || &bytes[12..16] != b"IHDR" {
        return None;
    }
    Some((
        u32::from_be_bytes(bytes[16..20].try_into().ok()?),
        u32::from_be_bytes(bytes[20..24].try_into().ok()?),
    ))
}

pub(super) fn parse_account_avatar_gif_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 10 || !matches!(&bytes[0..6], b"GIF87a" | b"GIF89a") {
        return None;
    }
    Some((
        u16::from_le_bytes(bytes[6..8].try_into().ok()?) as u32,
        u16::from_le_bytes(bytes[8..10].try_into().ok()?) as u32,
    ))
}

pub(super) fn parse_account_avatar_bmp_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 26 || &bytes[0..2] != b"BM" {
        return None;
    }
    let width = i32::from_le_bytes(bytes[18..22].try_into().ok()?);
    let height = i32::from_le_bytes(bytes[22..26].try_into().ok()?);
    Some((width.unsigned_abs(), height.unsigned_abs()))
}

pub(super) fn parse_account_avatar_jpeg_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
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

pub(super) fn parse_account_avatar_webp_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
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

pub(super) fn account_avatar_image_dimensions_from_header(
    bytes: &[u8],
) -> Option<(u32, u32, &'static str)> {
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

pub(super) fn account_avatar_image_dimensions_label(path: &Path, mime_type: &mime::Mime) -> String {
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
pub(super) fn account_avatar_thumbnail_target_dimensions(
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

pub(super) fn account_avatar_normalized_decode_action(action: &str) -> &'static str {
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

pub(super) fn read_account_avatar_image_decode_bytes(path: &Path) -> Result<Vec<u8>, String> {
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

pub(super) fn account_avatar_upload_decode_probe_label(
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

pub(super) fn account_avatar_selection_preview(
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

pub(super) fn account_avatar_upload_retry_confirmation_label(
    selected_summary: &str,
    file_path: &Path,
) -> String {
    let file_label = file_path.display();
    format!(
        "Retry avatar upload for {file_label}? Selected image: {selected_summary}. {ACCOUNT_AVATAR_UPLOAD_RETRY_CONFIRMATION_LABEL} No new file picker, cropper/editor, thumbnail decode, camera/photo-library capture, browser handoff, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

pub(super) fn parse_account_avatar_direct_mxc_uri(input: &str) -> Result<OwnedMxcUri, String> {
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

pub(super) fn account_avatar_direct_mxc_editor_status_label(
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

pub(super) fn account_avatar_direct_mxc_confirmation_label(avatar_url: &OwnedMxcUri) -> String {
    format!(
        "Set avatar directly to {}? {ACCOUNT_AVATAR_DIRECT_SET_CONFIRMATION_LABEL} This uses MatrixRequest::SetAvatar(Some) and client.account().set_avatar_url(Some) after confirmation only.",
        avatar_url.as_str()
    )
}

pub(super) fn account_avatar_direct_mxc_retry_confirmation_label(
    avatar_url: &OwnedMxcUri,
) -> String {
    format!(
        "Retry direct avatar SetAvatar(Some) for {}? {ACCOUNT_AVATAR_DIRECT_SET_CONFIRMATION_LABEL} Retry reuses only the cached mxc:// URI and confirms before MatrixRequest::SetAvatar(Some).",
        avatar_url.as_str()
    )
}

pub(super) fn account_avatar_upload_crop_editor_boundary_label(
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

pub(super) fn account_avatar_upload_cropper_snapshot_label(
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

pub(super) fn avatar_upload_preview_state_label(state: AvatarUploadPreviewState) -> &'static str {
    match state {
        AvatarUploadPreviewState::Hidden => "hidden",
        AvatarUploadPreviewState::ChoosePhoto => "choose photo preview",
        AvatarUploadPreviewState::Selected => "selected image preview",
        AvatarUploadPreviewState::Failed => "failed upload retry preview",
        AvatarUploadPreviewState::Crop => "crop/editor preview",
    }
}

pub(super) fn account_avatar_upload_editor_controls_row_label(
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

pub(super) fn account_avatar_upload_source_preview_controls_label(
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

pub(super) fn account_avatar_upload_source_editor_drilldown_packet_label(
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

pub(super) fn account_avatar_upload_source_editor_typed_contract_packet_label(
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

pub(super) fn account_avatar_upload_source_editor_result_taxonomy_packet_label(
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

pub(super) fn account_avatar_upload_source_path_clipboard_label(
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

pub(super) fn account_avatar_upload_source_path_clipboard_payload(
    selected_preview: Option<&AvatarUploadSelectionPreview>,
) -> Option<String> {
    selected_preview.map(|preview| preview.file_path.to_string_lossy().into_owned())
}

pub(super) fn account_avatar_upload_source_path_clipboard_metadata(
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

pub(super) fn account_avatar_upload_preflight_detail_controls_label(
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
