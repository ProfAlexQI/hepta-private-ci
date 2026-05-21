use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::Value;

use hepta_runtime::{
    native_telegram_normalize_binding_id, parse_native_telegram_env_truthy_value,
    parse_native_telegram_env_u64_value,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramConfigStatus {
    pub config_path: Option<String>,
    pub config_found: bool,
    pub enabled: bool,
    pub dm_policy: String,
    pub group_policy: String,
    pub allow_from_count: usize,
    pub group_count: usize,
    pub token_source: &'static str,
    pub token_secret_ref_present: bool,
    pub token_secret_provider: Option<String>,
    pub token_secret_id_present: bool,
    pub token_file_present: bool,
    pub token_file_mode_0600: bool,
    pub token_shape_ok: bool,
    pub raw_token_exposed: bool,
    pub binding_ready: bool,
    pub error: Option<String>,
}

impl NativeTelegramConfigStatus {
    pub fn disabled() -> Self {
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

    pub fn missing(error: String) -> Self {
        Self {
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
            error: Some(error),
        }
    }

    pub fn error(config_path: Option<String>, config_found: bool, error: String) -> Self {
        Self {
            config_path,
            config_found,
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
            error: Some(error),
        }
    }

    pub fn config_ready(&self) -> bool {
        self.enabled && self.token_shape_ok && self.binding_ready
    }
}

#[derive(Debug, Clone)]
pub struct NativeTelegramConfigStatusInput {
    pub config_path: Option<String>,
    pub config_found: bool,
    pub enabled: bool,
    pub dm_policy: String,
    pub group_policy: String,
    pub allow_from_count: usize,
    pub group_count: usize,
    pub token_source: &'static str,
    pub token_secret_ref_present: bool,
    pub token_secret_provider: Option<String>,
    pub token_secret_id_present: bool,
    pub token_file_present: bool,
    pub token_file_mode_0600: bool,
    pub token_shape_ok: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeTelegramTokenObservationInput {
    pub env_token_present: bool,
    pub env_token_shape_ok: bool,
    pub file_token_present: bool,
    pub file_token_shape_ok: bool,
    pub inline_token_present: bool,
    pub inline_token_shape_ok: bool,
    pub token_secret_ref_present: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeTelegramTokenObservation {
    pub token_source: &'static str,
    pub token_shape_ok: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeTelegramConfigMetadata {
    pub enabled: bool,
    pub dm_policy: String,
    pub group_policy: String,
    pub allow_from_count: usize,
    pub group_count: usize,
    pub token_secret_ref_present: bool,
    pub token_secret_provider: Option<String>,
    pub token_secret_id_present: bool,
    pub token_secret_path: Option<PathBuf>,
    pub inline_token_present: bool,
}

pub fn resolve_native_telegram_token_observation(
    input: NativeTelegramTokenObservationInput,
) -> NativeTelegramTokenObservation {
    if input.env_token_present {
        return NativeTelegramTokenObservation {
            token_source: "env",
            token_shape_ok: input.env_token_shape_ok,
        };
    }
    if input.file_token_present {
        return NativeTelegramTokenObservation {
            token_source: "secret_file",
            token_shape_ok: input.file_token_shape_ok,
        };
    }
    if input.inline_token_present {
        return NativeTelegramTokenObservation {
            token_source: "inline_config",
            token_shape_ok: input.inline_token_shape_ok,
        };
    }
    if input.token_secret_ref_present {
        return NativeTelegramTokenObservation {
            token_source: "secret_file_missing",
            token_shape_ok: false,
        };
    }
    NativeTelegramTokenObservation {
        token_source: "missing",
        token_shape_ok: false,
    }
}

pub fn build_native_telegram_config_status(
    input: NativeTelegramConfigStatusInput,
) -> NativeTelegramConfigStatus {
    let dm_policy = input.dm_policy.trim().to_ascii_lowercase();
    let group_policy = input.group_policy.trim().to_ascii_lowercase();
    let binding_ready = input.enabled
        && input.token_shape_ok
        && (input.allow_from_count > 0
            || input.group_count > 0
            || matches!(dm_policy.as_str(), "allow" | "trusted" | "all"));

    NativeTelegramConfigStatus {
        config_path: input.config_path,
        config_found: input.config_found,
        enabled: input.enabled,
        dm_policy,
        group_policy,
        allow_from_count: input.allow_from_count,
        group_count: input.group_count,
        token_source: input.token_source,
        token_secret_ref_present: input.token_secret_ref_present,
        token_secret_provider: input.token_secret_provider,
        token_secret_id_present: input.token_secret_id_present,
        token_file_present: input.token_file_present,
        token_file_mode_0600: input.token_file_mode_0600,
        token_shape_ok: input.token_shape_ok,
        raw_token_exposed: false,
        binding_ready,
        error: input.error,
    }
}

pub fn normalize_telegram_binding_id(raw: &str) -> String {
    native_telegram_normalize_binding_id(raw)
}

pub fn extract_native_telegram_config_metadata(
    config_path: &Path,
    config: &Value,
) -> Result<NativeTelegramConfigMetadata, String> {
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
    let allow_from_count = telegram
        .get("allowFrom")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(normalize_telegram_binding_id)
                .filter(|item| !item.is_empty())
                .count()
        })
        .unwrap_or(0);
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
    let token_secret_path = token_secret_provider
        .as_deref()
        .and_then(|provider| resolve_telegram_secret_provider_path(config_path, config, provider));
    let inline_token_present = bot_token_ref
        .and_then(Value::as_str)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false);

    Ok(NativeTelegramConfigMetadata {
        enabled,
        dm_policy,
        group_policy,
        allow_from_count,
        group_count,
        token_secret_ref_present,
        token_secret_provider,
        token_secret_id_present,
        token_secret_path,
        inline_token_present,
    })
}

pub fn parse_telegram_env_truthy_value(raw: &str) -> bool {
    parse_native_telegram_env_truthy_value(raw)
}

pub fn parse_telegram_env_u64_value(raw: &str) -> Option<u64> {
    parse_native_telegram_env_u64_value(raw)
}

pub fn resolve_telegram_secret_provider_path(
    config_path: &Path,
    config: &Value,
    provider: &str,
) -> Option<PathBuf> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_status_builder_derives_binding_without_exposing_tokens() {
        let status = build_native_telegram_config_status(NativeTelegramConfigStatusInput {
            config_path: Some("private/config/openclaw.json".to_string()),
            config_found: true,
            enabled: true,
            dm_policy: " Trusted ".to_string(),
            group_policy: "Deny".to_string(),
            allow_from_count: 1,
            group_count: 0,
            token_source: "secret_file",
            token_secret_ref_present: true,
            token_secret_provider: Some("telegram_bot".to_string()),
            token_secret_id_present: true,
            token_file_present: true,
            token_file_mode_0600: true,
            token_shape_ok: true,
            error: None,
        });

        assert!(status.binding_ready);
        assert!(status.config_ready());
        assert_eq!(status.dm_policy, "trusted");
        assert_eq!(status.group_policy, "deny");
        assert!(!status.raw_token_exposed);
    }

    #[test]
    fn config_status_builder_requires_binding_scope() {
        let status = build_native_telegram_config_status(NativeTelegramConfigStatusInput {
            config_path: Some("private/config/openclaw.json".to_string()),
            config_found: true,
            enabled: true,
            dm_policy: "deny".to_string(),
            group_policy: "deny".to_string(),
            allow_from_count: 0,
            group_count: 0,
            token_source: "env",
            token_secret_ref_present: false,
            token_secret_provider: None,
            token_secret_id_present: false,
            token_file_present: false,
            token_file_mode_0600: false,
            token_shape_ok: true,
            error: None,
        });

        assert!(!status.binding_ready);
        assert!(!status.config_ready());
    }

    #[test]
    fn normalize_telegram_binding_id_strips_known_prefixes() {
        assert_eq!(
            normalize_telegram_binding_id(" telegram:6476198178 "),
            "6476198178"
        );
        assert_eq!(normalize_telegram_binding_id("tg:6476198178"), "6476198178");
        assert_eq!(normalize_telegram_binding_id("6476198178"), "6476198178");
    }

    #[test]
    fn token_observation_prefers_env_then_file_then_inline_without_exposing_token() {
        let env = resolve_native_telegram_token_observation(NativeTelegramTokenObservationInput {
            env_token_present: true,
            env_token_shape_ok: true,
            file_token_present: true,
            file_token_shape_ok: true,
            inline_token_present: true,
            inline_token_shape_ok: true,
            token_secret_ref_present: true,
        });
        assert_eq!(env.token_source, "env");
        assert!(env.token_shape_ok);

        let file = resolve_native_telegram_token_observation(NativeTelegramTokenObservationInput {
            env_token_present: false,
            env_token_shape_ok: false,
            file_token_present: true,
            file_token_shape_ok: false,
            inline_token_present: true,
            inline_token_shape_ok: true,
            token_secret_ref_present: true,
        });
        assert_eq!(file.token_source, "secret_file");
        assert!(!file.token_shape_ok);

        let inline =
            resolve_native_telegram_token_observation(NativeTelegramTokenObservationInput {
                env_token_present: false,
                env_token_shape_ok: false,
                file_token_present: false,
                file_token_shape_ok: false,
                inline_token_present: true,
                inline_token_shape_ok: true,
                token_secret_ref_present: false,
            });
        assert_eq!(inline.token_source, "inline_config");
        assert!(inline.token_shape_ok);
    }

    #[test]
    fn token_observation_distinguishes_secret_file_missing_from_missing() {
        let secret_missing =
            resolve_native_telegram_token_observation(NativeTelegramTokenObservationInput {
                env_token_present: false,
                env_token_shape_ok: false,
                file_token_present: false,
                file_token_shape_ok: false,
                inline_token_present: false,
                inline_token_shape_ok: false,
                token_secret_ref_present: true,
            });
        assert_eq!(secret_missing.token_source, "secret_file_missing");
        assert!(!secret_missing.token_shape_ok);

        let missing =
            resolve_native_telegram_token_observation(NativeTelegramTokenObservationInput {
                env_token_present: false,
                env_token_shape_ok: false,
                file_token_present: false,
                file_token_shape_ok: false,
                inline_token_present: false,
                inline_token_shape_ok: false,
                token_secret_ref_present: false,
            });
        assert_eq!(missing.token_source, "missing");
        assert!(!missing.token_shape_ok);
    }

    #[test]
    fn telegram_env_value_parsers_are_trimmed_and_bounded() {
        assert!(parse_telegram_env_truthy_value(" YES "));
        assert!(parse_telegram_env_truthy_value("on"));
        assert!(!parse_telegram_env_truthy_value("off"));
        assert_eq!(parse_telegram_env_u64_value(" 42 "), Some(42));
        assert_eq!(parse_telegram_env_u64_value("not-a-number"), None);
    }

    #[test]
    fn config_metadata_extracts_non_secret_telegram_fields() {
        let config = serde_json::json!({
            "secrets": {
                "providers": {
                    "telegram_bot": {
                        "path": "../secrets/telegram-token"
                    }
                }
            },
            "channels": {
                "telegram": {
                    "enabled": true,
                    "dmPolicy": " Trusted ",
                    "groupPolicy": "Mention",
                    "allowFrom": ["telegram:6476198178", " tg:42 ", ""],
                    "groups": {
                        "ops": { "id": "-1001" },
                        "dev": { "id": "-1002" }
                    },
                    "botToken": {
                        "source": "file",
                        "provider": "telegram_bot",
                        "id": " bot-token "
                    }
                }
            }
        });

        let metadata = extract_native_telegram_config_metadata(
            Path::new("/tmp/hepta/private/config/openclaw.json"),
            &config,
        )
        .expect("metadata");

        assert!(metadata.enabled);
        assert_eq!(metadata.dm_policy, "trusted");
        assert_eq!(metadata.group_policy, "mention");
        assert_eq!(metadata.allow_from_count, 2);
        assert_eq!(metadata.group_count, 2);
        assert!(metadata.token_secret_ref_present);
        assert_eq!(
            metadata.token_secret_provider.as_deref(),
            Some("telegram_bot")
        );
        assert!(metadata.token_secret_id_present);
        assert_eq!(
            metadata.token_secret_path,
            Some(PathBuf::from(
                "/tmp/hepta/private/config/../secrets/telegram-token"
            ))
        );
        assert!(!metadata.inline_token_present);
    }

    #[test]
    fn secret_provider_path_resolves_relative_to_config_parent() {
        let config = serde_json::json!({
            "secrets": {
                "providers": {
                    "telegram_bot": {
                        "path": "../secrets/telegram-token"
                    }
                }
            }
        });
        let path = resolve_telegram_secret_provider_path(
            Path::new("/tmp/hepta/private/config/openclaw.json"),
            &config,
            "telegram_bot",
        )
        .expect("secret provider path");

        assert_eq!(
            path,
            PathBuf::from("/tmp/hepta/private/config/../secrets/telegram-token")
        );
        assert!(
            resolve_telegram_secret_provider_path(
                Path::new("/tmp/hepta/private/config/openclaw.json"),
                &config,
                "missing"
            )
            .is_none()
        );
    }
}
