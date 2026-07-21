pub use hepta_runtime::NativeTelegramConfigMetadata;
pub use hepta_runtime::NativeTelegramConfigStatus;
pub use hepta_runtime::NativeTelegramConfigStatusInput;
pub use hepta_runtime::NativeTelegramTokenObservation;
pub use hepta_runtime::NativeTelegramTokenObservationInput;
pub use hepta_runtime::build_native_telegram_config_status;
pub use hepta_runtime::extract_native_telegram_config_metadata;
pub use hepta_runtime::native_telegram_normalize_binding_id;
pub use hepta_runtime::parse_native_telegram_env_truthy_value;
pub use hepta_runtime::parse_native_telegram_env_u64_value;
pub use hepta_runtime::resolve_native_telegram_secret_provider_path as resolve_telegram_secret_provider_path;
pub use hepta_runtime::resolve_native_telegram_token_observation;

pub fn normalize_telegram_binding_id(raw: &str) -> String {
    native_telegram_normalize_binding_id(raw)
}

pub fn parse_telegram_env_truthy_value(raw: &str) -> bool {
    parse_native_telegram_env_truthy_value(raw)
}

pub fn parse_telegram_env_u64_value(raw: &str) -> Option<u64> {
    parse_native_telegram_env_u64_value(raw)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::path::PathBuf;

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
            token_file_security_ready: true,
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
            token_file_security_ready: false,
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
