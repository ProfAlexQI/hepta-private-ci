use hepta_core::HeptaError;
use serde::Deserialize;
use serde::Serialize;

const MAX_PREVIEW_BYTES: usize = 192;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperatorPolicyDecision {
    Allow,
    RequireApproval,
    Deny,
}

impl OperatorPolicyDecision {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::RequireApproval => "require_approval",
            Self::Deny => "deny",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorPolicyInput {
    pub channel_id: String,
    pub sender_id: String,
    #[serde(default)]
    pub sender_is_owner: bool,
    pub tool_name: String,
    pub tool_action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_session_id: Option<String>,
    #[serde(default)]
    pub message_cross_context_allowed: bool,
    #[serde(default)]
    pub message_action_allowed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_auth_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pairing_request_kind: Option<String>,
    #[serde(default)]
    pub pairing_provenance_verified: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sandbox_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_mount_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload_preview: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal_output_preview: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OperatorPolicyEvaluationReport {
    pub decision: OperatorPolicyDecision,
    pub decision_label: &'static str,
    pub allowed: bool,
    pub requires_approval: bool,
    pub denied_reasons: Vec<String>,
    pub policy_scope: &'static str,
    pub canonical_sender_key: String,
    pub channel_scoped_sender_key: bool,
    pub raw_sender_redacted: bool,
    pub tool_name: String,
    pub tool_action: String,
    pub message_cross_context_blocked: bool,
    pub message_action_allow_required: bool,
    pub message_action_blocked: bool,
    pub secret_ref_required: bool,
    pub secret_ref_only_auth_ok: bool,
    pub env_marker_auth_blocked: bool,
    pub provider_secret_read: bool,
    pub persisted_secret_payload_redacted: bool,
    pub redacted_payload_preview: Option<String>,
    pub pairing_approval_required: bool,
    pub pairing_provenance_ok: bool,
    pub pairing_command_started: bool,
    pub blocked_credential_home_root: bool,
    pub blocked_home_root_kind: Option<&'static str>,
    pub osc_terminal_escape_stripped: bool,
    pub sanitized_terminal_output_preview: Option<String>,
    pub read_only_sandbox_workspace_read_allowed: bool,
    pub read_only_sandbox_write_blocked: bool,
    pub tool_invocation_performed: bool,
    pub external_send_performed: bool,
    pub provider_invoked: bool,
}

pub fn evaluate_operator_policy(
    input: OperatorPolicyInput,
) -> Result<OperatorPolicyEvaluationReport, HeptaError> {
    let channel_id = normalize_non_empty(&input.channel_id, "channel id")?;
    let sender_id = normalize_non_empty(&input.sender_id, "sender id")?;
    let tool_name = normalize_non_empty(&input.tool_name, "tool name")?;
    let tool_action = normalize_action(&input.tool_action)?;
    let canonical_sender_key = canonical_sender_key(&channel_id, &sender_id);
    let mut denied_reasons = Vec::new();

    let message_tool = is_message_tool(&tool_name);
    let message_cross_context_blocked = message_tool
        && !input.message_cross_context_allowed
        && crosses_session_context(
            input.current_session_id.as_deref(),
            input.target_session_id.as_deref(),
        );
    if message_cross_context_blocked {
        denied_reasons.push("message-cross-context-blocked".into());
    }

    let message_action_allow_required = message_tool && is_message_mutating_action(&tool_action);
    let message_action_blocked = message_action_allow_required && !input.message_action_allowed;
    if message_action_blocked {
        denied_reasons.push("message-action-not-allowed-for-sender".into());
    }

    let secret_ref_required = input.provider_auth_ref.is_some();
    let (secret_ref_only_auth_ok, env_marker_auth_blocked) =
        match input.provider_auth_ref.as_deref() {
            Some(auth_ref) => {
                let normalized = auth_ref.trim().to_ascii_lowercase();
                let secret_ref = normalized.starts_with("secretref:")
                    || normalized.starts_with("secret-ref:")
                    || normalized.starts_with("secret://");
                let env_marker = normalized.starts_with("env:")
                    || normalized.contains("__env__:")
                    || normalized.contains("apikey=")
                    || normalized.contains("api_key=")
                    || normalized.contains("token=");
                (secret_ref && !env_marker, !secret_ref || env_marker)
            }
            None => (false, false),
        };
    if env_marker_auth_blocked {
        denied_reasons.push("provider-auth-must-use-secretref".into());
    }

    let pairing_approval_required = input.pairing_request_kind.is_some();
    let pairing_provenance_ok = !pairing_approval_required || input.pairing_provenance_verified;
    if pairing_approval_required && !input.sender_is_owner {
        denied_reasons.push("pairing-requires-owner-sender".into());
    }
    if pairing_approval_required && !pairing_provenance_ok {
        denied_reasons.push("pairing-provenance-not-verified".into());
    }

    let (blocked_credential_home_root, blocked_home_root_kind) =
        blocked_credential_home_root(input.target_path.as_deref());
    if blocked_credential_home_root {
        denied_reasons.push("credential-home-root-blocked".into());
    }

    let sandbox_mode = input
        .sandbox_mode
        .as_deref()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    let target_under_workspace_mount = path_under_mount(
        input.target_path.as_deref(),
        input.workspace_mount_path.as_deref(),
    );
    let read_only_sandbox = matches!(sandbox_mode.as_str(), "read-only" | "readonly" | "ro");
    let read_only_sandbox_workspace_read_allowed =
        read_only_sandbox && target_under_workspace_mount && is_read_action(&tool_action);
    let read_only_sandbox_write_blocked =
        read_only_sandbox && target_under_workspace_mount && is_write_action(&tool_action);
    if read_only_sandbox_write_blocked {
        denied_reasons.push("read-only-sandbox-write-blocked".into());
    }

    let redacted_payload_preview = input
        .payload_preview
        .as_deref()
        .map(redact_persisted_secret_payload);
    let persisted_secret_payload_redacted = redacted_payload_preview
        .as_deref()
        .is_some_and(|preview| preview == "<redacted-secret-payload>");

    let sanitized_terminal_output_preview = input
        .terminal_output_preview
        .as_deref()
        .map(strip_terminal_osc);
    let osc_terminal_escape_stripped = input
        .terminal_output_preview
        .as_deref()
        .zip(sanitized_terminal_output_preview.as_deref())
        .is_some_and(|(raw, sanitized)| raw != sanitized);

    let mut requires_approval = pairing_approval_required;
    if !input.sender_is_owner && is_mutating_action(&tool_action) {
        requires_approval = true;
    }
    let decision = if denied_reasons.is_empty() {
        if requires_approval {
            OperatorPolicyDecision::RequireApproval
        } else {
            OperatorPolicyDecision::Allow
        }
    } else {
        OperatorPolicyDecision::Deny
    };

    Ok(OperatorPolicyEvaluationReport {
        decision,
        decision_label: decision.label(),
        allowed: decision != OperatorPolicyDecision::Deny,
        requires_approval: decision == OperatorPolicyDecision::RequireApproval,
        denied_reasons,
        policy_scope: "channel-scoped-sender-tool-policy",
        canonical_sender_key,
        channel_scoped_sender_key: true,
        raw_sender_redacted: true,
        tool_name,
        tool_action,
        message_cross_context_blocked,
        message_action_allow_required,
        message_action_blocked,
        secret_ref_required,
        secret_ref_only_auth_ok,
        env_marker_auth_blocked,
        provider_secret_read: false,
        persisted_secret_payload_redacted,
        redacted_payload_preview,
        pairing_approval_required,
        pairing_provenance_ok,
        pairing_command_started: false,
        blocked_credential_home_root,
        blocked_home_root_kind,
        osc_terminal_escape_stripped,
        sanitized_terminal_output_preview,
        read_only_sandbox_workspace_read_allowed,
        read_only_sandbox_write_blocked,
        tool_invocation_performed: false,
        external_send_performed: false,
        provider_invoked: false,
    })
}

fn canonical_sender_key(channel_id: &str, sender_id: &str) -> String {
    format!(
        "sender-key:v1:channel-{:016x}:sender-{:016x}",
        stable_fingerprint64(channel_id),
        stable_fingerprint64(sender_id)
    )
}

fn stable_fingerprint64(value: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn normalize_non_empty(value: &str, label: &str) -> Result<String, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(HeptaError(format!("{label} must not be empty")));
    }
    if trimmed.contains('\n') || trimmed.contains('\r') {
        return Err(HeptaError(format!("{label} must be a single line")));
    }
    Ok(trimmed.to_string())
}

fn normalize_action(value: &str) -> Result<String, HeptaError> {
    Ok(normalize_non_empty(value, "tool action")?.to_ascii_lowercase())
}

fn is_message_tool(tool_name: &str) -> bool {
    let normalized = tool_name.trim().to_ascii_lowercase();
    matches!(
        normalized.as_str(),
        "message" | "tools.message" | "openclaw_message" | "hepta_message"
    ) || normalized.ends_with("message")
}

fn crosses_session_context(current: Option<&str>, target: Option<&str>) -> bool {
    match (current, target) {
        (Some(current), Some(target)) => current.trim() != target.trim(),
        _ => false,
    }
}

fn is_message_mutating_action(action: &str) -> bool {
    matches!(
        action,
        "send"
            | "sendwitheffect"
            | "reply"
            | "poll"
            | "react"
            | "edit"
            | "delete"
            | "upload-file"
            | "channel-create"
            | "channel-edit"
            | "channel-delete"
            | "thread-create"
            | "thread-reply"
            | "pin"
            | "unpin"
    )
}

fn is_mutating_action(action: &str) -> bool {
    is_message_mutating_action(action)
        || matches!(
            action,
            "write"
                | "edit"
                | "delete"
                | "apply_patch"
                | "patch"
                | "exec"
                | "run"
                | "spawn"
                | "approve"
                | "pair"
                | "install"
                | "update"
                | "restart"
        )
}

fn is_read_action(action: &str) -> bool {
    matches!(action, "read" | "list" | "get" | "search" | "status")
}

fn is_write_action(action: &str) -> bool {
    matches!(
        action,
        "write" | "edit" | "delete" | "apply_patch" | "patch" | "install" | "update"
    )
}

fn blocked_credential_home_root(path: Option<&str>) -> (bool, Option<&'static str>) {
    let Some(path) = path.map(str::trim).filter(|path| !path.is_empty()) else {
        return (false, None);
    };
    let lower = path.to_ascii_lowercase();
    let credential_like = lower.contains(".ssh")
        || lower.contains(".aws")
        || lower.contains(".config")
        || lower.contains(".openclaw")
        || lower.contains(".codex")
        || lower.contains("auth")
        || lower.contains("token")
        || lower.contains("secret")
        || lower.contains("credential");
    if !credential_like {
        return (false, None);
    }
    if lower.starts_with("%userprofile%") || lower.starts_with("$userprofile") {
        return (true, Some("windows-userprofile"));
    }
    if lower.starts_with("c:\\users\\") || lower.starts_with("c:/users/") {
        return (true, Some("windows-userprofile"));
    }
    if lower.starts_with("~/") {
        return (true, Some("shell-home"));
    }
    if lower.starts_with("/users/") || lower.starts_with("/home/") {
        return (true, Some("unix-home"));
    }
    (false, None)
}

fn path_under_mount(path: Option<&str>, mount: Option<&str>) -> bool {
    let Some(path) = path.map(str::trim).filter(|path| !path.is_empty()) else {
        return false;
    };
    let Some(mount) = mount.map(str::trim).filter(|mount| !mount.is_empty()) else {
        return false;
    };
    path == mount || path.starts_with(&format!("{}/", mount.trim_end_matches('/')))
}

fn redact_persisted_secret_payload(value: &str) -> String {
    let sanitized = strip_terminal_osc(value);
    let lower = sanitized.to_ascii_lowercase();
    if lower.contains("api_key")
        || lower.contains("apikey")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("token")
        || lower.contains("bearer ")
        || lower.contains("__env__:")
    {
        "<redacted-secret-payload>".into()
    } else {
        truncate_preview(&sanitized)
    }
}

fn strip_terminal_osc(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch != '\u{1b}' {
            output.push(ch);
            continue;
        }
        match chars.peek().copied() {
            Some(']') => {
                chars.next();
                strip_until_osc_end(&mut chars);
            }
            Some('[') => {
                chars.next();
                strip_until_csi_end(&mut chars);
            }
            _ => {}
        }
    }
    truncate_preview(&output)
}

fn strip_until_osc_end(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) {
    while let Some(ch) = chars.next() {
        if ch == '\u{7}' {
            return;
        }
        if ch == '\u{1b}' && chars.peek().copied() == Some('\\') {
            chars.next();
            return;
        }
    }
}

fn strip_until_csi_end(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) {
    for ch in chars.by_ref() {
        if ('@'..='~').contains(&ch) {
            return;
        }
    }
}

fn truncate_preview(value: &str) -> String {
    if value.len() <= MAX_PREVIEW_BYTES {
        return value.to_string();
    }
    let mut preview = String::new();
    for ch in value.chars() {
        if preview.len() + ch.len_utf8() > MAX_PREVIEW_BYTES {
            break;
        }
        preview.push(ch);
    }
    preview.push_str("...");
    preview
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_input() -> OperatorPolicyInput {
        OperatorPolicyInput {
            channel_id: "telegram:6476198178".into(),
            sender_id: "6476198178".into(),
            sender_is_owner: true,
            tool_name: "read".into(),
            tool_action: "read".into(),
            current_session_id: Some("session-a".into()),
            target_session_id: Some("session-a".into()),
            message_cross_context_allowed: false,
            message_action_allowed: false,
            provider_auth_ref: None,
            pairing_request_kind: None,
            pairing_provenance_verified: false,
            target_path: None,
            sandbox_mode: None,
            workspace_mount_path: None,
            payload_preview: None,
            terminal_output_preview: None,
        }
    }

    #[test]
    fn channel_scoped_sender_keys_redact_raw_sender() {
        let report = evaluate_operator_policy(base_input()).expect("policy should evaluate");

        assert_eq!(report.decision, OperatorPolicyDecision::Allow);
        assert!(report.channel_scoped_sender_key);
        assert!(report.raw_sender_redacted);
        assert!(report.canonical_sender_key.starts_with("sender-key:v1:"));
        assert!(!report.canonical_sender_key.contains("6476198178"));
        assert!(!report.tool_invocation_performed);
    }

    #[test]
    fn message_policy_blocks_cross_context_and_unallowed_send_action() {
        let mut input = base_input();
        input.sender_is_owner = false;
        input.tool_name = "tools.message".into();
        input.tool_action = "send".into();
        input.target_session_id = Some("session-b".into());

        let report = evaluate_operator_policy(input).expect("policy should evaluate");

        assert_eq!(report.decision, OperatorPolicyDecision::Deny);
        assert!(report.message_cross_context_blocked);
        assert!(report.message_action_allow_required);
        assert!(report.message_action_blocked);
        assert!(
            report
                .denied_reasons
                .contains(&"message-cross-context-blocked".to_string())
        );
        assert!(
            report
                .denied_reasons
                .contains(&"message-action-not-allowed-for-sender".to_string())
        );
        assert!(!report.external_send_performed);
    }

    #[test]
    fn secretref_only_auth_blocks_env_markers_and_credential_home_roots() {
        let mut env_ref = base_input();
        env_ref.provider_auth_ref = Some("__env__:OPENAI_API_KEY".into());
        env_ref.target_path = Some("%USERPROFILE%\\.codex\\auth.json".into());
        env_ref.payload_preview = Some("Authorization: Bearer sk-secret".into());

        let blocked = evaluate_operator_policy(env_ref).expect("policy should evaluate");
        assert_eq!(blocked.decision, OperatorPolicyDecision::Deny);
        assert!(blocked.secret_ref_required);
        assert!(!blocked.secret_ref_only_auth_ok);
        assert!(blocked.env_marker_auth_blocked);
        assert!(blocked.blocked_credential_home_root);
        assert_eq!(blocked.blocked_home_root_kind, Some("windows-userprofile"));
        assert!(blocked.persisted_secret_payload_redacted);
        assert_eq!(
            blocked.redacted_payload_preview.as_deref(),
            Some("<redacted-secret-payload>")
        );
        assert!(!blocked.provider_secret_read);

        let mut secret_ref = base_input();
        secret_ref.provider_auth_ref = Some("SecretRef:providers/openai/default".into());
        let allowed = evaluate_operator_policy(secret_ref).expect("policy should evaluate");
        assert!(allowed.secret_ref_only_auth_ok);
        assert!(!allowed.env_marker_auth_blocked);
    }

    #[test]
    fn pairing_provenance_requires_owner_and_no_command_start() {
        let mut input = base_input();
        input.sender_is_owner = false;
        input.tool_action = "pair".into();
        input.pairing_request_kind = Some("setup-code".into());
        input.pairing_provenance_verified = false;

        let report = evaluate_operator_policy(input).expect("policy should evaluate");

        assert_eq!(report.decision, OperatorPolicyDecision::Deny);
        assert!(report.pairing_approval_required);
        assert!(!report.pairing_provenance_ok);
        assert!(
            report
                .denied_reasons
                .contains(&"pairing-requires-owner-sender".to_string())
        );
        assert!(
            report
                .denied_reasons
                .contains(&"pairing-provenance-not-verified".to_string())
        );
        assert!(!report.pairing_command_started);
    }

    #[test]
    fn strips_terminal_osc_and_allows_read_only_workspace_reads() {
        let mut input = base_input();
        input.terminal_output_preview =
            Some("\u{1b}]0;secret title\u{7}visible\u{1b}[31m red".into());
        input.target_path = Some("/workspace/project/README.md".into());
        input.workspace_mount_path = Some("/workspace/project".into());
        input.sandbox_mode = Some("read-only".into());

        let read = evaluate_operator_policy(input).expect("policy should evaluate");
        assert_eq!(read.decision, OperatorPolicyDecision::Allow);
        assert!(read.osc_terminal_escape_stripped);
        assert_eq!(
            read.sanitized_terminal_output_preview.as_deref(),
            Some("visible red")
        );
        assert!(read.read_only_sandbox_workspace_read_allowed);

        let mut write = base_input();
        write.tool_action = "apply_patch".into();
        write.target_path = Some("/workspace/project/src/lib.rs".into());
        write.workspace_mount_path = Some("/workspace/project".into());
        write.sandbox_mode = Some("read-only".into());

        let blocked = evaluate_operator_policy(write).expect("policy should evaluate");
        assert_eq!(blocked.decision, OperatorPolicyDecision::Deny);
        assert!(blocked.read_only_sandbox_write_blocked);
    }
}
