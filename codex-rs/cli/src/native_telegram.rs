use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::Value;

const LEGACY_RUNTIME_SLUG: &str = "openclaw";
const LEGACY_CONFIG_FILE_NAME: &str = "openclaw.json";
const LOCAL_IMPORT_CONFIG_PATH: &str = ".hepta/local-import/private/config/openclaw.json";
const LOCAL_IMPORT_MANIFEST_PATH: &str = ".hepta/local-import/manifest.json";
const TELEGRAM_ALLOWED_UPDATES: &str =
    "[\"message\",\"edited_message\",\"callback_query\",\"message_reaction\"]";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramPluginStatus {
    pub(crate) product: &'static str,
    pub(crate) runtime: &'static str,
    pub(crate) requested: bool,
    pub(crate) status: &'static str,
    pub(crate) in_process_supervisor_ready: bool,
    pub(crate) in_process_reply_loop_ready: bool,
    pub(crate) model_turn_bridge_ready: bool,
    pub(crate) bot_api_poll_ready: bool,
    pub(crate) bot_api_send_ready: bool,
    pub(crate) openclaw_gateway_runtime_dependency: bool,
    pub(crate) external_network_read: bool,
    pub(crate) external_send: bool,
    pub(crate) poll_ms: u64,
    pub(crate) allowed_updates: &'static str,
    pub(crate) config: NativeTelegramConfigStatus,
    pub(crate) transport_plan: NativeTelegramTransportPlan,
    pub(crate) ingress_parser: NativeTelegramIngressInspection,
    pub(crate) migration_blocker: Option<&'static str>,
    pub(crate) next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramConfigStatus {
    pub(crate) config_path: Option<String>,
    pub(crate) config_found: bool,
    pub(crate) enabled: bool,
    pub(crate) dm_policy: String,
    pub(crate) group_policy: String,
    pub(crate) allow_from_count: usize,
    pub(crate) group_count: usize,
    pub(crate) token_source: &'static str,
    pub(crate) token_secret_ref_present: bool,
    pub(crate) token_secret_provider: Option<String>,
    pub(crate) token_secret_id_present: bool,
    pub(crate) token_file_present: bool,
    pub(crate) token_file_mode_0600: bool,
    pub(crate) token_shape_ok: bool,
    pub(crate) raw_token_exposed: bool,
    pub(crate) binding_ready: bool,
    pub(crate) error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramTransportPlan {
    pub(crate) bot_api_transport_plan_ready: bool,
    pub(crate) endpoint_template: &'static str,
    pub(crate) get_updates_method: &'static str,
    pub(crate) send_message_method: &'static str,
    pub(crate) send_chat_action_method: &'static str,
    pub(crate) allowed_updates: &'static str,
    pub(crate) offset_commit_strategy: &'static str,
    pub(crate) send_delivery_gate: &'static str,
    pub(crate) typing_keepalive_plan: &'static str,
    pub(crate) raw_token_exposed: bool,
    pub(crate) external_network_performed_by_status: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NativeTelegramIngressInspection {
    pub(crate) parser_ready: bool,
    pub(crate) update_count: usize,
    pub(crate) allowed_update_count: usize,
    pub(crate) latest_observed_update_id: Option<i64>,
    pub(crate) latest_allowed_update_id: Option<i64>,
    pub(crate) latest_allowed_text_present: bool,
    pub(crate) message_count: usize,
    pub(crate) edited_message_count: usize,
    pub(crate) callback_query_count: usize,
    pub(crate) reaction_count: usize,
    pub(crate) raw_message_text_exposed: bool,
    pub(crate) raw_chat_id_exposed: bool,
    pub(crate) raw_sender_id_exposed: bool,
}

pub(crate) fn telegram_plugin_status(requested: bool, poll_ms: u64) -> NativeTelegramPluginStatus {
    if !requested {
        return NativeTelegramPluginStatus {
            product: "Hepta",
            runtime: "hepta-codex",
            requested,
            status: "disabled",
            in_process_supervisor_ready: false,
            in_process_reply_loop_ready: false,
            model_turn_bridge_ready: false,
            bot_api_poll_ready: false,
            bot_api_send_ready: false,
            openclaw_gateway_runtime_dependency: false,
            external_network_read: false,
            external_send: false,
            poll_ms,
            allowed_updates: TELEGRAM_ALLOWED_UPDATES,
            config: NativeTelegramConfigStatus::disabled(),
            transport_plan: NativeTelegramTransportPlan::disabled(),
            ingress_parser: inspect_telegram_updates(&[]),
            migration_blocker: None,
            next_migration_slice: "enable --with-telegram-plugin, then wire Bot API polling and model-turn delivery",
        };
    }

    let config = load_telegram_config_status();
    let supervisor_ready = config.error.is_none();
    let config_ready = config.enabled && config.token_shape_ok && config.binding_ready;
    let status = if supervisor_ready && config_ready {
        "native_supervisor_ready"
    } else {
        "attention"
    };

    NativeTelegramPluginStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested,
        status,
        in_process_supervisor_ready: supervisor_ready,
        in_process_reply_loop_ready: false,
        model_turn_bridge_ready: false,
        bot_api_poll_ready: false,
        bot_api_send_ready: false,
        openclaw_gateway_runtime_dependency: false,
        external_network_read: false,
        external_send: false,
        poll_ms,
        allowed_updates: TELEGRAM_ALLOWED_UPDATES,
        transport_plan: NativeTelegramTransportPlan::for_config(&config),
        config,
        ingress_parser: inspect_telegram_updates(&[]),
        migration_blocker: Some(
            "Bot API polling/send and Codex model-turn bridge are not enabled in hepta-codex yet",
        ),
        next_migration_slice: "wire native Bot API getUpdates/sendMessage loop behind explicit delivery gates",
    }
}

fn load_telegram_config_status() -> NativeTelegramConfigStatus {
    let Some(config_path) = resolve_private_hepta_runtime_config_path() else {
        return NativeTelegramConfigStatus {
            config_path: None,
            config_found: false,
            enabled: false,
            dm_policy: String::new(),
            group_policy: String::new(),
            allow_from_count: 0,
            group_count: 0,
            token_source: "missing",
            token_secret_ref_present: false,
            token_secret_provider: None,
            token_secret_id_present: false,
            token_file_present: false,
            token_file_mode_0600: false,
            token_shape_ok: false,
            raw_token_exposed: false,
            binding_ready: false,
            error: Some("Hepta private Telegram config not found".to_string()),
        };
    };

    match load_telegram_config_status_from_path(&config_path) {
        Ok(status) => status,
        Err(error) => NativeTelegramConfigStatus {
            config_path: Some(config_path.display().to_string()),
            config_found: config_path.is_file(),
            enabled: false,
            dm_policy: String::new(),
            group_policy: String::new(),
            allow_from_count: 0,
            group_count: 0,
            token_source: "error",
            token_secret_ref_present: false,
            token_secret_provider: None,
            token_secret_id_present: false,
            token_file_present: false,
            token_file_mode_0600: false,
            token_shape_ok: false,
            raw_token_exposed: false,
            binding_ready: false,
            error: Some(redact_token_like_text(&error)),
        },
    }
}

fn load_telegram_config_status_from_path(
    path: &Path,
) -> Result<NativeTelegramConfigStatus, String> {
    let raw = fs::read_to_string(path)
        .map_err(|error| format!("failed to read Hepta private Telegram config: {error}"))?;
    let config: Value = serde_json::from_str(&raw)
        .map_err(|error| format!("failed to parse Hepta private Telegram config: {error}"))?;
    let telegram = config
        .pointer("/channels/telegram")
        .ok_or_else(|| "channels.telegram config is missing".to_string())?;

    let enabled = telegram
        .get("enabled")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let dm_policy = telegram
        .get("dmPolicy")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    let group_policy = telegram
        .get("groupPolicy")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    let allow_from = telegram
        .get("allowFrom")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(normalize_telegram_id)
                .filter(|item| !item.is_empty())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let group_count = telegram
        .get("groups")
        .and_then(Value::as_array)
        .map(Vec::len)
        .or_else(|| {
            telegram
                .get("groups")
                .and_then(Value::as_object)
                .map(|groups| groups.len())
        })
        .unwrap_or(0);

    let bot_token_ref = telegram.get("botToken");
    let token_secret_ref_present = bot_token_ref
        .and_then(|value| value.get("source"))
        .and_then(Value::as_str)
        == Some("file");
    let token_secret_provider = bot_token_ref
        .and_then(|value| value.get("provider"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);
    let token_secret_id_present = bot_token_ref
        .and_then(|value| value.get("id"))
        .and_then(Value::as_str)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false);
    let token_path = token_secret_provider
        .as_deref()
        .and_then(|provider| secret_provider_path(path, &config, provider));
    let token_file_present = token_path
        .as_ref()
        .map(|path| path.is_file())
        .unwrap_or(false);
    let token_file_mode_0600 = token_path.as_ref().map(file_mode_is_0600).unwrap_or(false);
    let inline_token = bot_token_ref
        .and_then(Value::as_str)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let env_token = env::var("HEPTA_TELEGRAM_BOT_TOKEN")
        .ok()
        .or_else(|| env::var("TELEGRAM_BOT_TOKEN").ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let file_token = token_path
        .as_ref()
        .and_then(|path| fs::read_to_string(path).ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let (token_source, token) = if let Some(token) = env_token {
        ("env", Some(token))
    } else if let Some(token) = file_token {
        ("secret_file", Some(token))
    } else if let Some(token) = inline_token {
        ("inline_config", Some(token))
    } else if token_secret_ref_present {
        ("secret_file_missing", None)
    } else {
        ("missing", None)
    };
    let token_shape_ok = token.as_deref().map(token_shape_ok).unwrap_or(false);
    let binding_ready = enabled
        && token_shape_ok
        && (!allow_from.is_empty()
            || group_count > 0
            || matches!(dm_policy.as_str(), "allow" | "trusted" | "all"));

    Ok(NativeTelegramConfigStatus {
        config_path: Some(path.display().to_string()),
        config_found: true,
        enabled,
        dm_policy,
        group_policy,
        allow_from_count: allow_from.len(),
        group_count,
        token_source,
        token_secret_ref_present,
        token_secret_provider,
        token_secret_id_present,
        token_file_present,
        token_file_mode_0600,
        token_shape_ok,
        raw_token_exposed: false,
        binding_ready,
        error: None,
    })
}

fn resolve_private_hepta_runtime_config_path() -> Option<PathBuf> {
    if let Ok(path) = env::var("HEPTA_CONFIG_PATH") {
        let path = PathBuf::from(path);
        if path.is_file() {
            return Some(path);
        }
    }

    let relative = PathBuf::from(LOCAL_IMPORT_CONFIG_PATH);
    if relative.is_file() {
        return Some(relative);
    }

    let manifest = PathBuf::from(LOCAL_IMPORT_MANIFEST_PATH);
    if let Ok(raw) = fs::read_to_string(&manifest) {
        if let Ok(value) = serde_json::from_str::<Value>(&raw) {
            if let Some(import_root) = value.get("import_root").and_then(Value::as_str) {
                let candidate = PathBuf::from(import_root)
                    .join("private/config")
                    .join(LEGACY_CONFIG_FILE_NAME);
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }
    }

    let home_config = env::var_os("HOME").map(|home| {
        PathBuf::from(home)
            .join(format!(".{LEGACY_RUNTIME_SLUG}"))
            .join(LEGACY_CONFIG_FILE_NAME)
    });
    home_config.filter(|path| path.is_file())
}

fn secret_provider_path(config_path: &Path, config: &Value, provider: &str) -> Option<PathBuf> {
    let raw = config
        .get("secrets")?
        .get("providers")?
        .get(provider)?
        .get("path")?
        .as_str()?;
    let path = PathBuf::from(raw);
    if path.is_absolute() {
        Some(path)
    } else {
        config_path.parent().map(|parent| parent.join(path))
    }
}

#[cfg(unix)]
fn file_mode_is_0600(path: &PathBuf) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o777 == 0o600)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn file_mode_is_0600(path: &PathBuf) -> bool {
    path.is_file()
}

fn token_shape_ok(token: &str) -> bool {
    let Some((bot_id, secret)) = token.split_once(':') else {
        return false;
    };
    !bot_id.is_empty()
        && bot_id.chars().all(|ch| ch.is_ascii_digit())
        && secret.len() >= 20
        && secret
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
}

fn normalize_telegram_id(raw: &str) -> String {
    let trimmed = raw.trim();
    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("telegram:") {
        return trimmed["telegram:".len()..].trim().to_string();
    }
    if lower.starts_with("tg:") {
        return trimmed["tg:".len()..].trim().to_string();
    }
    trimmed.to_string()
}

fn redact_token_like_text(text: &str) -> String {
    text.split_whitespace()
        .map(|part| {
            if token_shape_ok(part.trim_matches(|ch: char| {
                !ch.is_ascii_alphanumeric() && ch != ':' && ch != '_' && ch != '-'
            })) {
                "[redacted-telegram-token]".to_string()
            } else {
                part.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn inspect_telegram_updates(updates: &[Value]) -> NativeTelegramIngressInspection {
    let mut inspection = NativeTelegramIngressInspection {
        parser_ready: true,
        update_count: updates.len(),
        allowed_update_count: 0,
        latest_observed_update_id: None,
        latest_allowed_update_id: None,
        latest_allowed_text_present: false,
        message_count: 0,
        edited_message_count: 0,
        callback_query_count: 0,
        reaction_count: 0,
        raw_message_text_exposed: false,
        raw_chat_id_exposed: false,
        raw_sender_id_exposed: false,
    };

    for update in updates {
        let update_id = update.get("update_id").and_then(Value::as_i64);
        if let Some(update_id) = update_id {
            inspection.latest_observed_update_id = Some(
                inspection
                    .latest_observed_update_id
                    .map(|current| current.max(update_id))
                    .unwrap_or(update_id),
            );
        }

        let (allowed, text_present) = if let Some(message) = update.get("message") {
            inspection.message_count = inspection.message_count.saturating_add(1);
            (
                telegram_message_is_reply_candidate(message),
                telegram_message_text_present(message),
            )
        } else if let Some(message) = update.get("edited_message") {
            inspection.edited_message_count = inspection.edited_message_count.saturating_add(1);
            (
                telegram_message_is_reply_candidate(message),
                telegram_message_text_present(message),
            )
        } else if update.get("callback_query").is_some() {
            inspection.callback_query_count = inspection.callback_query_count.saturating_add(1);
            (true, false)
        } else if update.get("message_reaction").is_some() {
            inspection.reaction_count = inspection.reaction_count.saturating_add(1);
            (true, false)
        } else {
            (false, false)
        };

        if allowed {
            inspection.allowed_update_count = inspection.allowed_update_count.saturating_add(1);
            if let Some(update_id) = update_id {
                inspection.latest_allowed_update_id = Some(
                    inspection
                        .latest_allowed_update_id
                        .map(|current| current.max(update_id))
                        .unwrap_or(update_id),
                );
            }
            inspection.latest_allowed_text_present |= text_present;
        }
    }

    inspection
}

fn telegram_message_is_reply_candidate(message: &Value) -> bool {
    message
        .get("chat")
        .and_then(|chat| chat.get("id"))
        .is_some()
        && message.get("message_id").is_some()
        && telegram_message_text_present(message)
}

fn telegram_message_text_present(message: &Value) -> bool {
    message
        .get("text")
        .and_then(Value::as_str)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
        || message
            .get("caption")
            .and_then(Value::as_str)
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
}

impl NativeTelegramConfigStatus {
    fn disabled() -> Self {
        Self {
            config_path: None,
            config_found: false,
            enabled: false,
            dm_policy: String::new(),
            group_policy: String::new(),
            allow_from_count: 0,
            group_count: 0,
            token_source: "disabled",
            token_secret_ref_present: false,
            token_secret_provider: None,
            token_secret_id_present: false,
            token_file_present: false,
            token_file_mode_0600: false,
            token_shape_ok: false,
            raw_token_exposed: false,
            binding_ready: false,
            error: None,
        }
    }
}

impl NativeTelegramTransportPlan {
    fn disabled() -> Self {
        Self {
            bot_api_transport_plan_ready: false,
            endpoint_template: "https://api.telegram.org/bot<redacted-token>/{method}",
            get_updates_method: "getUpdates",
            send_message_method: "sendMessage",
            send_chat_action_method: "sendChatAction",
            allowed_updates: TELEGRAM_ALLOWED_UPDATES,
            offset_commit_strategy: "disabled",
            send_delivery_gate: "disabled",
            typing_keepalive_plan: "disabled",
            raw_token_exposed: false,
            external_network_performed_by_status: false,
        }
    }

    fn for_config(config: &NativeTelegramConfigStatus) -> Self {
        let ready = config.enabled && config.token_shape_ok && config.binding_ready;
        Self {
            bot_api_transport_plan_ready: ready,
            endpoint_template: "https://api.telegram.org/bot<redacted-token>/{method}",
            get_updates_method: "getUpdates",
            send_message_method: "sendMessage",
            send_chat_action_method: "sendChatAction",
            allowed_updates: TELEGRAM_ALLOWED_UPDATES,
            offset_commit_strategy: "commit getUpdates offset only after delivery succeeds or duplicate suppression is recorded",
            send_delivery_gate: "sendMessage requires a successful model-turn or command dispatch plus explicit confirm-send runtime gate",
            typing_keepalive_plan: "sendChatAction typing keepalive is planned while the model turn is running, with bounded TTL",
            raw_token_exposed: false,
            external_network_performed_by_status: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telegram_config_status_reads_secret_file_without_exposing_token() {
        let temp = tempfile::tempdir().expect("tempdir");
        let secret_path = temp.path().join("telegram-token.txt");
        fs::write(&secret_path, "123456789:abcdefghijklmnopqrstuvwxyz").expect("write token");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&secret_path, fs::Permissions::from_mode(0o600)).expect("set mode");
        }
        let config_path = temp.path().join("openclaw.json");
        fs::write(
            &config_path,
            format!(
                r#"{{
                    "secrets": {{ "providers": {{ "telegram_bot": {{ "path": "{}" }} }} }},
                    "channels": {{
                        "telegram": {{
                            "enabled": true,
                            "dmPolicy": "allow",
                            "groupPolicy": "mention",
                            "allowFrom": ["telegram:6476198178"],
                            "botToken": {{
                                "source": "file",
                                "provider": "telegram_bot",
                                "id": "bot-token"
                            }}
                        }}
                    }}
                }}"#,
                secret_path.display()
            ),
        )
        .expect("write config");

        let status = load_telegram_config_status_from_path(&config_path).expect("load config");
        assert!(status.enabled);
        assert_eq!(status.token_source, "secret_file");
        assert!(status.token_shape_ok);
        assert!(status.binding_ready);
        assert!(!status.raw_token_exposed);

        let serialized = serde_json::to_string(&status).expect("serialize");
        assert!(!serialized.contains("abcdefghijklmnopqrstuvwxyz"));
        assert!(serialized.contains("\"raw_token_exposed\":false"));
    }

    #[test]
    fn requested_plugin_reports_native_supervisor_without_reply_loop_claim() {
        let temp = tempfile::tempdir().expect("tempdir");
        let config_path = temp.path().join("openclaw.json");
        fs::write(
            &config_path,
            r#"{
                "channels": {
                    "telegram": {
                        "enabled": true,
                        "dmPolicy": "allow",
                        "allowFrom": ["6476198178"],
                        "botToken": "123456789:abcdefghijklmnopqrstuvwxyz"
                    }
                }
            }"#,
        )
        .expect("write config");

        let config = load_telegram_config_status_from_path(&config_path).expect("load config");
        let plugin = NativeTelegramPluginStatus {
            product: "Hepta",
            runtime: "hepta-codex",
            requested: true,
            status: "native_supervisor_ready",
            in_process_supervisor_ready: true,
            in_process_reply_loop_ready: false,
            model_turn_bridge_ready: false,
            bot_api_poll_ready: false,
            bot_api_send_ready: false,
            openclaw_gateway_runtime_dependency: false,
            external_network_read: false,
            external_send: false,
            poll_ms: 1500,
            allowed_updates: TELEGRAM_ALLOWED_UPDATES,
            transport_plan: NativeTelegramTransportPlan::for_config(&config),
            config,
            ingress_parser: inspect_telegram_updates(&[]),
            migration_blocker: Some(
                "Bot API polling/send and Codex model-turn bridge are not enabled in hepta-codex yet",
            ),
            next_migration_slice: "wire native Bot API getUpdates/sendMessage loop behind explicit delivery gates",
        };

        assert_eq!(plugin.status, "native_supervisor_ready");
        assert!(plugin.in_process_supervisor_ready);
        assert!(!plugin.in_process_reply_loop_ready);
        assert!(!plugin.external_send);
        assert!(plugin.transport_plan.bot_api_transport_plan_ready);
        assert!(!plugin.transport_plan.external_network_performed_by_status);
        assert!(!plugin.transport_plan.raw_token_exposed);
        assert!(plugin.ingress_parser.parser_ready);
        assert!(!plugin.ingress_parser.raw_message_text_exposed);
    }

    #[test]
    fn ingress_parser_counts_allowed_updates_without_exposing_private_fields() {
        let update = serde_json::json!({
            "update_id": 42,
            "message": {
                "message_id": 7,
                "text": "private prompt text",
                "chat": { "id": 6476198178_i64, "type": "private" },
                "from": { "id": 6476198178_i64, "username": "private_user" }
            }
        });

        let inspection = inspect_telegram_updates(&[update]);
        assert!(inspection.parser_ready);
        assert_eq!(inspection.update_count, 1);
        assert_eq!(inspection.allowed_update_count, 1);
        assert_eq!(inspection.latest_observed_update_id, Some(42));
        assert_eq!(inspection.latest_allowed_update_id, Some(42));
        assert!(inspection.latest_allowed_text_present);

        let serialized = serde_json::to_string(&inspection).expect("serialize");
        assert!(!serialized.contains("private prompt text"));
        assert!(!serialized.contains("6476198178"));
        assert!(!inspection.raw_message_text_exposed);
        assert!(!inspection.raw_chat_id_exposed);
        assert!(!inspection.raw_sender_id_exposed);
    }
}
