use std::env;
use std::ffi::OsString;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use codex_hepta_agentd::HEPTA_AGENT_GENERATION_ENV;
use codex_hepta_agentd::HEPTA_AGENT_ID_ENV;
use codex_hepta_contracts::AgentId;
use codex_hepta_fleet::AgentLifecycle;
use codex_hepta_fleet::FleetRegistry;
use codex_hepta_matrix_protocol::MATRIX_BINDING_SCHEMA_VERSION;
use codex_hepta_matrix_protocol::MatrixBindingV1;
use codex_hepta_matrix_protocol::MatrixDeviceId;
use codex_hepta_matrix_protocol::MatrixHomeserverUrl;
use codex_hepta_matrix_protocol::MatrixUserId;
use codex_hepta_paths::HEPTA_FLEET_ROOT_ENV;
use codex_hepta_paths::HeptaAgentLayout;
use codex_hepta_paths::HeptaFleetRoot;

pub const HEPTA_MATRIX_HOMESERVER_ENV: &str = "HEPTA_MATRIX_HOMESERVER";
pub const HEPTA_MATRIX_USER_ID_ENV: &str = "HEPTA_MATRIX_USER_ID";
pub const HEPTA_MATRIX_DEVICE_ID_ENV: &str = "HEPTA_MATRIX_DEVICE_ID";
pub const HEPTA_MATRIX_PASSWORD_ENV: &str = "HEPTA_MATRIX_PASSWORD";
pub const HEPTA_MATRIX_STORE_PASSPHRASE_ENV: &str = "HEPTA_MATRIX_STORE_PASSPHRASE";
pub const HEPTA_MATRIX_ALLOWED_ROOMS_ENV: &str = "HEPTA_MATRIX_ALLOWED_ROOMS_JSON";
pub const HEPTA_MATRIX_ALLOWED_SENDERS_ENV: &str = "HEPTA_MATRIX_ALLOWED_SENDERS_JSON";
pub const HEPTA_MATRIX_REQUIRE_EXPLICIT_MENTION_ENV: &str = "HEPTA_MATRIX_REQUIRE_EXPLICIT_MENTION";
pub const HEPTA_MATRIX_BINDING_REVISION_ENV: &str = "HEPTA_MATRIX_BINDING_REVISION";
pub const HEPTA_MATRIX_SYNC_TIMELINE_LIMIT_ENV: &str = "HEPTA_MATRIX_SYNC_TIMELINE_LIMIT";
pub const HEPTA_MATRIX_SYNC_TIMEOUT_MS_ENV: &str = "HEPTA_MATRIX_SYNC_TIMEOUT_MS";
pub const HEPTA_MATRIX_DEVICE_DISPLAY_NAME_ENV: &str = "HEPTA_MATRIX_DEVICE_DISPLAY_NAME";

const DEFAULT_SYNC_TIMELINE_LIMIT: u16 = 64;
const MAX_SYNC_TIMELINE_LIMIT: u16 = 256;
const DEFAULT_SYNC_TIMEOUT_MS: u64 = 30_000;
const MIN_SYNC_TIMEOUT_MS: u64 = 1_000;
const MAX_SYNC_TIMEOUT_MS: u64 = 30_000;

/// Process-owned secrets are deliberately non-serializable and redact Debug.
pub struct MatrixdCredentials {
    password: String,
    store_passphrase: Option<String>,
}

impl MatrixdCredentials {
    pub fn new(
        password: impl Into<String>,
        store_passphrase: Option<String>,
    ) -> Result<Self, MatrixdConfigError> {
        let password = password.into();
        if password.is_empty() || store_passphrase.as_deref() == Some("") {
            return Err(MatrixdConfigError::Invalid(
                "Matrix credentials must not be empty".to_string(),
            ));
        }
        Ok(Self {
            password,
            store_passphrase,
        })
    }

    pub(crate) fn password(&self) -> &str {
        &self.password
    }

    pub(crate) fn store_passphrase(&self) -> Option<&str> {
        self.store_passphrase.as_deref()
    }
}

impl fmt::Debug for MatrixdCredentials {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MatrixdCredentials")
            .field("password", &"<redacted>")
            .field(
                "store_passphrase",
                &self.store_passphrase.as_ref().map(|_| "<redacted>"),
            )
            .finish()
    }
}

pub struct MatrixdConfig {
    pub agent_id: AgentId,
    pub spawn_generation: u64,
    pub layout: HeptaAgentLayout,
    pub workspace_root: PathBuf,
    pub binding: MatrixBindingV1,
    pub sync_timeline_limit: u16,
    pub sync_timeout: Duration,
    pub device_display_name: String,
    credentials: MatrixdCredentials,
}

impl fmt::Debug for MatrixdConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MatrixdConfig")
            .field("agent_id", &self.agent_id)
            .field("spawn_generation", &self.spawn_generation)
            .field("layout", &self.layout)
            .field("workspace_root", &self.workspace_root)
            .field("binding", &self.binding)
            .field("sync_timeline_limit", &self.sync_timeline_limit)
            .field("sync_timeout", &self.sync_timeout)
            .field("device_display_name", &self.device_display_name)
            .field("credentials", &self.credentials)
            .finish()
    }
}

impl MatrixdConfig {
    pub fn from_process_environment() -> Result<Self, MatrixdConfigError> {
        let fleet_root = required_path(HEPTA_FLEET_ROOT_ENV)?;
        let agent_id = AgentId::parse(required_utf8(HEPTA_AGENT_ID_ENV)?)
            .map_err(|error| MatrixdConfigError::Invalid(error.to_string()))?;
        let spawn_generation = parse_required(HEPTA_AGENT_GENERATION_ENV)?;
        let binding = MatrixBindingV1 {
            schema_version: MATRIX_BINDING_SCHEMA_VERSION,
            agent_id: agent_id.clone(),
            revision: parse_required(HEPTA_MATRIX_BINDING_REVISION_ENV)?,
            homeserver: MatrixHomeserverUrl::parse(required_utf8(HEPTA_MATRIX_HOMESERVER_ENV)?)
                .map_err(|error| MatrixdConfigError::Invalid(error.to_string()))?,
            expected_mxid: MatrixUserId::parse(required_utf8(HEPTA_MATRIX_USER_ID_ENV)?)
                .map_err(|error| MatrixdConfigError::Invalid(error.to_string()))?,
            expected_device_id: MatrixDeviceId::parse(required_utf8(HEPTA_MATRIX_DEVICE_ID_ENV)?)
                .map_err(|error| MatrixdConfigError::Invalid(error.to_string()))?,
            allowed_rooms: parse_json_values(HEPTA_MATRIX_ALLOWED_ROOMS_ENV)?,
            allowed_senders: parse_json_values(HEPTA_MATRIX_ALLOWED_SENDERS_ENV)?,
            require_explicit_mention: parse_optional_bool(
                HEPTA_MATRIX_REQUIRE_EXPLICIT_MENTION_ENV,
                false,
            )?,
        };
        let credentials = MatrixdCredentials::new(
            required_utf8(HEPTA_MATRIX_PASSWORD_ENV)?,
            optional_utf8(HEPTA_MATRIX_STORE_PASSPHRASE_ENV)?,
        )?;
        let sync_timeline_limit = parse_optional(
            HEPTA_MATRIX_SYNC_TIMELINE_LIMIT_ENV,
            DEFAULT_SYNC_TIMELINE_LIMIT,
        )?;
        let sync_timeout_ms =
            parse_optional(HEPTA_MATRIX_SYNC_TIMEOUT_MS_ENV, DEFAULT_SYNC_TIMEOUT_MS)?;
        let device_display_name = optional_utf8(HEPTA_MATRIX_DEVICE_DISPLAY_NAME_ENV)?
            .unwrap_or_else(|| format!("Hepta {}", agent_id.as_str()));
        Self::load(
            fleet_root,
            agent_id,
            spawn_generation,
            binding,
            credentials,
            sync_timeline_limit,
            Duration::from_millis(sync_timeout_ms),
            device_display_name,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn load(
        fleet_root: PathBuf,
        agent_id: AgentId,
        spawn_generation: u64,
        binding: MatrixBindingV1,
        credentials: MatrixdCredentials,
        sync_timeline_limit: u16,
        sync_timeout: Duration,
        device_display_name: String,
    ) -> Result<Self, MatrixdConfigError> {
        if spawn_generation == 0
            || !(1..=MAX_SYNC_TIMELINE_LIMIT).contains(&sync_timeline_limit)
            || !(Duration::from_millis(MIN_SYNC_TIMEOUT_MS)
                ..=Duration::from_millis(MAX_SYNC_TIMEOUT_MS))
                .contains(&sync_timeout)
            || device_display_name.trim().is_empty()
        {
            return Err(MatrixdConfigError::Invalid(
                "Matrix runtime bounds are invalid".to_string(),
            ));
        }
        binding
            .validate()
            .map_err(|error| MatrixdConfigError::Invalid(error.to_string()))?;
        if binding.agent_id != agent_id {
            return Err(MatrixdConfigError::Invalid(
                "Matrix binding belongs to a different AgentId".to_string(),
            ));
        }

        require_canonical(&fleet_root, "fleet root")?;
        let typed_root = HeptaFleetRoot::parse(fleet_root)
            .map_err(|error| MatrixdConfigError::Invalid(error.to_string()))?;
        let registry = FleetRegistry::open_existing(typed_root)?;
        let record = registry.load()?.agent(&agent_id).cloned().ok_or_else(|| {
            MatrixdConfigError::Invalid(format!("unknown fleet agent {agent_id}"))
        })?;
        let expected_running_generation = spawn_generation.checked_add(1).ok_or_else(|| {
            MatrixdConfigError::Invalid("Matrix Agent generation overflow".to_string())
        })?;
        if record.lifecycle.lifecycle != AgentLifecycle::Running
            || record.lifecycle.generation != expected_running_generation
        {
            return Err(MatrixdConfigError::GenerationFenced(format!(
                "agent {agent_id} spawn generation {spawn_generation} cannot attach to {:?} generation {}",
                record.lifecycle.lifecycle, record.lifecycle.generation
            )));
        }

        Ok(Self {
            agent_id,
            spawn_generation,
            layout: record.layout,
            workspace_root: record.manifest.workspace.as_path().to_path_buf(),
            binding,
            sync_timeline_limit,
            sync_timeout,
            device_display_name,
            credentials,
        })
    }

    pub(crate) fn credentials(&self) -> &MatrixdCredentials {
        &self.credentials
    }
}

fn required_utf8(name: &str) -> Result<String, MatrixdConfigError> {
    optional_utf8(name)?.ok_or_else(|| MatrixdConfigError::Invalid(format!("{name} is required")))
}

fn optional_utf8(name: &str) -> Result<Option<String>, MatrixdConfigError> {
    let Some(value) = env::var_os(name).filter(|value| !value.is_empty()) else {
        return Ok(None);
    };
    value
        .into_string()
        .map(Some)
        .map_err(|_| MatrixdConfigError::Invalid(format!("{name} must be UTF-8")))
}

fn required_path(name: &str) -> Result<PathBuf, MatrixdConfigError> {
    let value: OsString = env::var_os(name)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| MatrixdConfigError::Invalid(format!("{name} is required")))?;
    Ok(PathBuf::from(value))
}

fn parse_required<T>(name: &str) -> Result<T, MatrixdConfigError>
where
    T: std::str::FromStr,
{
    required_utf8(name)?
        .parse()
        .map_err(|_| MatrixdConfigError::Invalid(format!("{name} has an invalid numeric value")))
}

fn parse_optional<T>(name: &str, default: T) -> Result<T, MatrixdConfigError>
where
    T: std::str::FromStr,
{
    match optional_utf8(name)? {
        Some(value) => value.parse().map_err(|_| {
            MatrixdConfigError::Invalid(format!("{name} has an invalid numeric value"))
        }),
        None => Ok(default),
    }
}

fn parse_optional_bool(name: &str, default: bool) -> Result<bool, MatrixdConfigError> {
    match optional_utf8(name)?.as_deref() {
        Some("true" | "1") => Ok(true),
        Some("false" | "0") => Ok(false),
        Some(_) => Err(MatrixdConfigError::Invalid(format!(
            "{name} must be true, false, 1, or 0"
        ))),
        None => Ok(default),
    }
}

fn parse_json_values<T>(name: &str) -> Result<Vec<T>, MatrixdConfigError>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_str(&required_utf8(name)?)
        .map_err(|_| MatrixdConfigError::Invalid(format!("{name} must be a JSON array")))
}

fn require_canonical(path: &Path, label: &str) -> Result<(), MatrixdConfigError> {
    let canonical = path.canonicalize()?;
    if canonical != path {
        return Err(MatrixdConfigError::Invalid(format!(
            "{label} must be canonical and symlink-free: {}",
            path.display()
        )));
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum MatrixdConfigError {
    #[error("invalid matrixd configuration: {0}")]
    Invalid(String),
    #[error("matrixd generation is fenced: {0}")]
    GenerationFenced(String),
    #[error(transparent)]
    Fleet(#[from] codex_hepta_fleet::FleetRegistryError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests;
