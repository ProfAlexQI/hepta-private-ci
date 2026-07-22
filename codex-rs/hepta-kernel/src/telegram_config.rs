use serde::Serialize;
use serde_json::Value;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelTelegramConfigStatus {
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
    pub token_file_security_ready: bool,
    pub token_shape_ok: bool,
    pub raw_token_exposed: bool,
    pub binding_ready: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HeptaKernelTelegramConfigStatusInput {
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
    pub token_file_security_ready: bool,
    pub token_shape_ok: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeptaKernelTelegramTokenObservationInput {
    pub env_token_present: bool,
    pub env_token_shape_ok: bool,
    pub file_token_present: bool,
    pub file_token_shape_ok: bool,
    pub inline_token_present: bool,
    pub inline_token_shape_ok: bool,
    pub token_secret_ref_present: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeptaKernelTelegramTokenObservation {
    pub token_source: &'static str,
    pub token_shape_ok: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaKernelTelegramConfigMetadata {
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

pub fn hepta_kernel_telegram_normalize_binding_id(raw: &str) -> String {
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

pub fn hepta_kernel_telegram_env_truthy_value(raw: &str) -> bool {
    matches!(
        raw.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

pub fn hepta_kernel_telegram_env_u64_value(raw: &str) -> Option<u64> {
    raw.trim().parse::<u64>().ok()
}

pub fn hepta_kernel_telegram_token_observation(
    input: HeptaKernelTelegramTokenObservationInput,
) -> HeptaKernelTelegramTokenObservation {
    if input.env_token_present {
        return HeptaKernelTelegramTokenObservation {
            token_source: "env",
            token_shape_ok: input.env_token_shape_ok,
        };
    }
    if input.file_token_present {
        return HeptaKernelTelegramTokenObservation {
            token_source: "secret_file",
            token_shape_ok: input.file_token_shape_ok,
        };
    }
    if input.inline_token_present {
        return HeptaKernelTelegramTokenObservation {
            token_source: "inline_config",
            token_shape_ok: input.inline_token_shape_ok,
        };
    }
    if input.token_secret_ref_present {
        return HeptaKernelTelegramTokenObservation {
            token_source: "secret_file_missing",
            token_shape_ok: false,
        };
    }
    HeptaKernelTelegramTokenObservation {
        token_source: "missing",
        token_shape_ok: false,
    }
}

pub fn build_hepta_kernel_telegram_config_status(
    input: HeptaKernelTelegramConfigStatusInput,
) -> HeptaKernelTelegramConfigStatus {
    let dm_policy = input.dm_policy.trim().to_ascii_lowercase();
    let group_policy = input.group_policy.trim().to_ascii_lowercase();
    let binding_ready = input.enabled
        && input.token_shape_ok
        && (input.allow_from_count > 0
            || input.group_count > 0
            || matches!(dm_policy.as_str(), "allow" | "trusted" | "all"));

    HeptaKernelTelegramConfigStatus {
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
        token_file_security_ready: input.token_file_security_ready,
        token_shape_ok: input.token_shape_ok,
        raw_token_exposed: false,
        binding_ready,
        error: input.error,
    }
}

pub fn extract_hepta_kernel_telegram_config_metadata(
    config_path: &Path,
    config: &Value,
) -> Result<HeptaKernelTelegramConfigMetadata, String> {
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
                .map(hepta_kernel_telegram_normalize_binding_id)
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
    let token_secret_path = if token_secret_ref_present {
        token_secret_provider.as_deref().and_then(|provider| {
            resolve_hepta_kernel_telegram_secret_provider_path(config_path, config, provider)
        })
    } else {
        None
    };
    let inline_token_present = bot_token_ref
        .and_then(Value::as_str)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false);

    Ok(HeptaKernelTelegramConfigMetadata {
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

pub fn resolve_hepta_kernel_telegram_secret_provider_path(
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
