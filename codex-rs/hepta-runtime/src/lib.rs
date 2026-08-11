//! Minimal live-runtime composition for the vNext integration trunk.
//!
//! The compatibility adapter opens the existing Hepta schema-v5 stores in
//! read-only mode. It deliberately does not copy Memory S1/S2 logic: a future
//! integrator can provide a [`RuntimeStateAdapter`] backed by the process-owned
//! `StateDbHandle` without changing the gateway contract.

#![forbid(unsafe_code)]

use std::fmt;
use std::fs::File;
use std::fs::Metadata;
use std::fs::OpenOptions;
use std::io::Read;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::sync::Arc;

use anyhow::Context;
use anyhow::Result;
use codex_hepta_paths::HeptaStateLayout;
use codex_hepta_paths::HeptaStateRoot;
use serde::Deserialize;
use serde::Serialize;
use sqlx::SqlitePool;

pub const EXISTING_SCHEMA_VERSION: i64 = 5;
pub const RUNTIME_SNAPSHOT_VERSION: u64 = 1;

/// Read-only state information exposed to the loopback gateway.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuntimeStateStatus {
    pub adapter: &'static str,
    pub schema_version: i64,
    pub outcome_generation: i64,
    pub preference_generation: i64,
    pub runtime_snapshot_version: u64,
    pub runtime_snapshot_generation: u64,
    pub integrity_binding_present: bool,
    pub integrity_verification: &'static str,
    pub open_mode: &'static str,
}

/// Seam for the parallel memory port. Implementations must already be open and
/// must not create or migrate state as a side effect of status inspection.
pub trait RuntimeStateAdapter: fmt::Debug + Send + Sync {
    fn status(&self) -> RuntimeStateStatus;
}

/// Minimal runtime held by the native loopback gateway.
#[derive(Debug, Clone)]
pub struct HeptaRuntime {
    state_root: HeptaStateRoot,
    state: Arc<dyn RuntimeStateAdapter>,
}

impl HeptaRuntime {
    pub async fn open_existing(state_root: HeptaStateRoot) -> Result<Self> {
        let layout = state_root.layout();
        let adapter = SchemaV5OpenExistingAdapter::open(&layout).await?;
        Ok(Self::from_adapter(state_root, Arc::new(adapter)))
    }

    pub fn from_adapter(state_root: HeptaStateRoot, state: Arc<dyn RuntimeStateAdapter>) -> Self {
        Self { state_root, state }
    }

    pub fn status(&self) -> RuntimeStatus {
        RuntimeStatus {
            schema: "hepta_vnext_live_runtime_status_v1",
            product: "hepta",
            status: "ready",
            state_root: self.state_root.to_string(),
            state: self.state.status(),
            authority: RuntimeAuthorityStatus::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuntimeStatus {
    pub schema: &'static str,
    pub product: &'static str,
    pub status: &'static str,
    pub state_root: String,
    pub state: RuntimeStateStatus,
    pub authority: RuntimeAuthorityStatus,
}

/// Explicitly closed effects for the internal-test live shell.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct RuntimeAuthorityStatus {
    pub telegram: bool,
    pub outbound: bool,
    pub model_invocation: bool,
    pub operator_mutation: bool,
    pub enforce: bool,
    pub promotion: bool,
    pub retirement: bool,
    pub automatic_transition: bool,
}

struct SchemaV5OpenExistingAdapter {
    _outcomes: SqlitePool,
    _preferences: SqlitePool,
    status: RuntimeStateStatus,
}

impl fmt::Debug for SchemaV5OpenExistingAdapter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SchemaV5OpenExistingAdapter")
            .field("status", &self.status)
            .finish_non_exhaustive()
    }
}

impl RuntimeStateAdapter for SchemaV5OpenExistingAdapter {
    fn status(&self) -> RuntimeStateStatus {
        self.status.clone()
    }
}

impl SchemaV5OpenExistingAdapter {
    async fn open(layout: &HeptaStateLayout) -> Result<Self> {
        validate_private_directory(layout.state_root().as_path(), "state root")?;
        validate_private_directory(layout.runtime_root(), "runtime root")?;
        for (path, description) in [
            (layout.runtime_integrity_key(), "runtime integrity key"),
            (
                layout.preference_integrity_key(),
                "preference integrity key",
            ),
            (layout.preference_ingress_key(), "preference ingress key"),
        ] {
            validate_private_regular_file(&path, description)?;
        }

        let outcome_database = layout.outcomes_database();
        let preference_database = layout.preferences_database();
        let (outcomes, outcome_metadata) =
            open_schema_v5_database(&outcome_database, "outcome database").await?;
        let (preferences, preference_metadata) =
            open_schema_v5_database(&preference_database, "preference database").await?;
        let runtime_state = read_runtime_state(&layout.runtime_state())?;

        Ok(Self {
            _outcomes: outcomes,
            _preferences: preferences,
            status: RuntimeStateStatus {
                adapter: "schema-v5-open-existing",
                schema_version: EXISTING_SCHEMA_VERSION,
                outcome_generation: outcome_metadata.generation,
                preference_generation: preference_metadata.generation,
                runtime_snapshot_version: runtime_state.payload.version,
                runtime_snapshot_generation: runtime_state.payload.generation,
                integrity_binding_present: outcome_metadata.integrity_binding_present
                    && preference_metadata.integrity_binding_present
                    && runtime_state.integrity_tag.starts_with("hmac-sha256:"),
                integrity_verification: "delegated-to-memory-adapter",
                open_mode: "read-only-open-existing",
            },
        })
    }
}

#[derive(Debug)]
struct DatabaseMetadata {
    generation: i64,
    integrity_binding_present: bool,
}

async fn open_schema_v5_database(
    path: &Path,
    description: &'static str,
) -> Result<(SqlitePool, DatabaseMetadata)> {
    let identity = validate_private_regular_file(path, description)?;
    let pool = sqlite_config_for(path)?
        .open_read_only_pool(path)
        .await
        .with_context(|| format!("open existing {description} at {}", path.display()))?;
    identity
        .revalidate(path)
        .with_context(|| format!("revalidate opened {description}"))?;

    let schema_version =
        sqlx::query_scalar::<_, i64>("SELECT version FROM hepta_v2_schema WHERE singleton = 1")
            .fetch_optional(&pool)
            .await
            .with_context(|| format!("read schema version from {description}"))?
            .context("schema version row is missing")?;
    if schema_version != EXISTING_SCHEMA_VERSION {
        anyhow::bail!(
            "{description} schema version is {schema_version}, expected {EXISTING_SCHEMA_VERSION}"
        );
    }

    let generation = sqlx::query_scalar::<_, i64>(
        "SELECT generation FROM hepta_v2_write_lock WHERE singleton = 1",
    )
    .fetch_optional(&pool)
    .await
    .with_context(|| format!("read write-lock generation from {description}"))?
    .context("write-lock generation row is missing")?;
    if generation < 0 {
        anyhow::bail!("{description} write-lock generation must not be negative");
    }

    let integrity = sqlx::query_as::<_, (String, String)>(
        "SELECT algorithm, key_id FROM hepta_v2_integrity WHERE singleton = 1",
    )
    .fetch_optional(&pool)
    .await
    .with_context(|| format!("read integrity binding from {description}"))?
    .context("integrity binding row is missing")?;
    let integrity_binding_present = integrity.0 == "hmac-sha256" && !integrity.1.trim().is_empty();
    if !integrity_binding_present {
        anyhow::bail!("{description} has an unsupported integrity binding");
    }

    Ok((
        pool,
        DatabaseMetadata {
            generation,
            integrity_binding_present,
        },
    ))
}

fn sqlite_config_for(path: &Path) -> Result<codex_state::SqliteConfig> {
    let parent = path.parent().context("SQLite database has no parent")?;
    let parent = codex_utils_absolute_path::AbsolutePathBuf::from_absolute_path_checked(parent)
        .context("SQLite parent path must be absolute")?;
    Ok(codex_state::SqliteConfig::from_sqlite_home(parent))
}

#[derive(Debug, Deserialize)]
struct RuntimeStateEnvelope {
    payload: RuntimeStatePayload,
    integrity_tag: String,
}

#[derive(Debug, Deserialize)]
struct RuntimeStatePayload {
    version: u64,
    generation: u64,
}

fn read_runtime_state(path: &Path) -> Result<RuntimeStateEnvelope> {
    let expected_identity = validate_private_regular_file(path, "runtime state")?;
    let file = open_no_follow(path).context("open existing runtime state")?;
    let opened_identity = FileIdentity::from_metadata(&file.metadata()?);
    if opened_identity != expected_identity {
        anyhow::bail!("runtime state changed while opening");
    }
    let mut bytes = Vec::new();
    file.take(1024 * 1024)
        .read_to_end(&mut bytes)
        .context("read existing runtime state")?;
    if bytes.is_empty() || bytes.len() >= 1024 * 1024 {
        anyhow::bail!("runtime state is empty or exceeds the one-megabyte limit");
    }
    let state: RuntimeStateEnvelope =
        serde_json::from_slice(&bytes).context("decode existing runtime state")?;
    if state.payload.version != RUNTIME_SNAPSHOT_VERSION {
        anyhow::bail!(
            "runtime snapshot version is {}, expected {RUNTIME_SNAPSHOT_VERSION}",
            state.payload.version
        );
    }
    if !state.integrity_tag.starts_with("hmac-sha256:")
        || state.integrity_tag.len() != "hmac-sha256:".len() + 64
    {
        anyhow::bail!("runtime state integrity binding is malformed");
    }
    Ok(state)
}

fn validate_private_directory(path: &Path, description: &str) -> Result<()> {
    let metadata = std::fs::symlink_metadata(path)
        .with_context(|| format!("inspect {description} at {}", path.display()))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        anyhow::bail!("{description} must be an existing non-symlink directory");
    }
    validate_private_permissions(&metadata, description)
}

fn validate_private_regular_file(path: &Path, description: &str) -> Result<FileIdentity> {
    let metadata = std::fs::symlink_metadata(path)
        .with_context(|| format!("inspect {description} at {}", path.display()))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        anyhow::bail!("{description} must be an existing non-symlink regular file");
    }
    validate_private_permissions(&metadata, description)?;
    #[cfg(unix)]
    if metadata.nlink() != 1 {
        anyhow::bail!("{description} must have exactly one hard link");
    }
    Ok(FileIdentity::from_metadata(&metadata))
}

#[cfg(unix)]
fn validate_private_permissions(metadata: &Metadata, description: &str) -> Result<()> {
    if metadata.mode() & 0o077 != 0 {
        anyhow::bail!("{description} must not grant group or world permissions");
    }
    Ok(())
}

#[cfg(not(unix))]
fn validate_private_permissions(_metadata: &Metadata, _description: &str) -> Result<()> {
    Ok(())
}

fn open_no_follow(path: &Path) -> Result<File> {
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    options.custom_flags(libc::O_NOFOLLOW);
    options.open(path).map_err(Into::into)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileIdentity {
    #[cfg(unix)]
    device: u64,
    #[cfg(unix)]
    inode: u64,
    #[cfg(not(unix))]
    length: u64,
}

impl FileIdentity {
    fn from_metadata(metadata: &Metadata) -> Self {
        #[cfg(unix)]
        {
            Self {
                device: metadata.dev(),
                inode: metadata.ino(),
            }
        }
        #[cfg(not(unix))]
        {
            Self {
                length: metadata.len(),
            }
        }
    }

    fn revalidate(self, path: &Path) -> Result<()> {
        let current = validate_private_regular_file(path, "opened database")?;
        if self != current {
            anyhow::bail!("database identity changed while opening");
        }
        Ok(())
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    async fn create_database(path: &Path, schema_version: i64) -> Result<()> {
        let pool = sqlite_config_for(path)?
            .open_durable_evidence_pool(path)
            .await?;
        for statement in [
            "CREATE TABLE hepta_v2_schema (singleton INTEGER PRIMARY KEY, version INTEGER NOT NULL)",
            "CREATE TABLE hepta_v2_write_lock (singleton INTEGER PRIMARY KEY, generation INTEGER NOT NULL)",
            "CREATE TABLE hepta_v2_integrity (singleton INTEGER PRIMARY KEY, algorithm TEXT NOT NULL, key_id TEXT NOT NULL)",
        ] {
            sqlx::query(statement).execute(&pool).await?;
        }
        sqlx::query("INSERT INTO hepta_v2_schema VALUES (1, ?)")
            .bind(schema_version)
            .execute(&pool)
            .await?;
        sqlx::query("INSERT INTO hepta_v2_write_lock VALUES (1, 0)")
            .execute(&pool)
            .await?;
        sqlx::query("INSERT INTO hepta_v2_integrity VALUES (1, 'hmac-sha256', 'test-key')")
            .execute(&pool)
            .await?;
        pool.close().await;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
        Ok(())
    }

    async fn fixture(schema_version: i64) -> Result<(tempfile::TempDir, HeptaStateRoot)> {
        let directory = tempfile::tempdir()?;
        let root = HeptaStateRoot::parse(directory.path())?;
        let layout = root.layout();
        std::fs::create_dir(layout.runtime_root())?;
        std::fs::create_dir(layout.runtime_root().join("keys"))?;
        std::fs::set_permissions(directory.path(), std::fs::Permissions::from_mode(0o700))?;
        std::fs::set_permissions(
            layout.runtime_root(),
            std::fs::Permissions::from_mode(0o700),
        )?;
        std::fs::set_permissions(
            layout.runtime_root().join("keys"),
            std::fs::Permissions::from_mode(0o700),
        )?;
        for key in [
            layout.runtime_integrity_key(),
            layout.preference_integrity_key(),
            layout.preference_ingress_key(),
        ] {
            std::fs::write(&key, b"fixture-key\n")?;
            std::fs::set_permissions(&key, std::fs::Permissions::from_mode(0o600))?;
        }
        create_database(&layout.outcomes_database(), schema_version).await?;
        create_database(&layout.preferences_database(), schema_version).await?;
        std::fs::write(
            layout.runtime_state(),
            format!(
                "{{\"payload\":{{\"version\":1,\"generation\":0}},\"integrity_tag\":\"hmac-sha256:{}\"}}",
                "0".repeat(64)
            ),
        )?;
        std::fs::set_permissions(
            layout.runtime_state(),
            std::fs::Permissions::from_mode(0o600),
        )?;
        Ok((directory, root))
    }

    #[tokio::test]
    async fn opens_exact_schema_v5_without_mutation_authority() -> Result<()> {
        let (_directory, root) = fixture(EXISTING_SCHEMA_VERSION).await?;
        let runtime = HeptaRuntime::open_existing(root).await?;
        let status = runtime.status();
        assert_eq!(status.status, "ready");
        assert_eq!(status.state.schema_version, 5);
        assert_eq!(status.state.open_mode, "read-only-open-existing");
        assert_eq!(status.authority, RuntimeAuthorityStatus::default());
        Ok(())
    }

    #[tokio::test]
    async fn rejects_wrong_schema_without_creating_or_migrating() -> Result<()> {
        let (_directory, root) = fixture(EXISTING_SCHEMA_VERSION - 1).await?;
        let Err(error) = HeptaRuntime::open_existing(root).await else {
            anyhow::bail!("wrong schema was accepted");
        };
        assert!(error.to_string().contains("expected 5"), "{error:#}");
        Ok(())
    }

    #[tokio::test]
    async fn rejects_exposed_state_file() -> Result<()> {
        let (_directory, root) = fixture(EXISTING_SCHEMA_VERSION).await?;
        let runtime_state = root.layout().runtime_state();
        std::fs::set_permissions(&runtime_state, std::fs::Permissions::from_mode(0o644))?;
        let Err(error) = HeptaRuntime::open_existing(root).await else {
            anyhow::bail!("exposed state was accepted");
        };
        assert!(error.to_string().contains("group or world"), "{error:#}");
        Ok(())
    }
}
