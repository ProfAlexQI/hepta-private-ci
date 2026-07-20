//! Secure persistence for Matrix session credentials and non-secret metadata.

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use codex_keyring_store::DefaultKeyringStore;
use codex_keyring_store::KeyringStore;
use matrix_sdk::authentication::matrix::MatrixSession;
use matrix_sdk::ruma::UserId;
use matrix_sdk::sliding_sync;
use serde::Deserialize;
use serde::Serialize;
use url::Url;
use zeroize::Zeroize;
use zeroize::Zeroizing;

const SESSION_METADATA_VERSION: u8 = 2;
const SESSION_CREDENTIAL_VERSION: u8 = 1;
const MATRIX_CREDENTIAL_SERVICE: &str = "ai.hepta.native.matrix";
static MATRIX_SESSION_STORE_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

/// The in-memory data needed to re-build a Matrix client.
///
/// The database passphrase deliberately does not implement `Serialize`: only
/// [`SessionSecretsPersisted`] can serialize it, and that payload is written to
/// the operating-system credential store rather than to the filesystem.
#[derive(Clone)]
pub struct ClientSessionPersisted {
    /// The URL of the homeserver of the user.
    pub homeserver: String,

    /// The database path. New sessions store this as a relative subfolder
    /// (joined with `app_data_dir()` at restore time); legacy sessions may have
    /// an absolute path.
    pub db_path: PathBuf,

    /// The passphrase of the encrypted Matrix database.
    pub passphrase: String,
}

impl std::fmt::Debug for ClientSessionPersisted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientSessionPersisted")
            .field("homeserver", &self.homeserver)
            .field("db_path", &self.db_path)
            .field("passphrase", &"<REDACTED>")
            .finish()
    }
}

/// A serializable duplicate of [`sliding_sync::Version`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlidingSyncVersion {
    #[default]
    Native,
    None,
}

impl From<SlidingSyncVersion> for sliding_sync::Version {
    fn from(version: SlidingSyncVersion) -> Self {
        match version {
            SlidingSyncVersion::None => sliding_sync::Version::None,
            SlidingSyncVersion::Native => sliding_sync::Version::Native,
        }
    }
}

impl From<sliding_sync::Version> for SlidingSyncVersion {
    fn from(version: sliding_sync::Version) -> Self {
        match version {
            sliding_sync::Version::None => SlidingSyncVersion::None,
            sliding_sync::Version::Native => SlidingSyncVersion::Native,
        }
    }
}

pub(crate) struct SessionMaterial {
    pub client_session: ClientSessionPersisted,
    pub user_session: MatrixSession,
    pub sync_token: Option<String>,
    pub sliding_sync_version: SlidingSyncVersion,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SessionMetadataPersisted {
    version: u8,
    client_session: ClientSessionMetadataPersisted,
    credential_account: String,
    sliding_sync_version: SlidingSyncVersion,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ClientSessionMetadataPersisted {
    homeserver: String,
    db_path: PathBuf,
}

/// This is the only serialized representation containing Matrix credentials.
/// Its JSON value is stored directly in the OS credential store and is never
/// passed to the filesystem writer.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SessionSecretsPersisted {
    version: u8,
    database_passphrase: String,
    user_session: MatrixSession,
    sync_token: Option<String>,
}

/// Read-only compatibility shape for the plaintext session format used before
/// metadata schema v2. It is never used by the save path.
#[derive(Deserialize)]
struct LegacyFullSessionPersisted {
    client_session: LegacyClientSessionPersisted,
    user_session: MatrixSession,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_token: Option<String>,
    #[serde(default)]
    sliding_sync_version: SlidingSyncVersion,
}

#[derive(Deserialize)]
struct LegacyClientSessionPersisted {
    homeserver: String,
    db_path: PathBuf,
    passphrase: String,
}

enum DecodedSessionFile {
    Secure(SessionMetadataPersisted),
    Legacy(SessionMaterial),
}

fn credential_account(user_id: &UserId) -> String {
    format!("matrix-session-v1|{user_id}")
}

fn default_keyring_store() -> Arc<dyn KeyringStore> {
    Arc::new(DefaultKeyringStore)
}

fn ensure_system_credential_store_supported() -> Result<()> {
    #[cfg(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "linux",
        target_os = "macos",
        target_os = "openbsd",
        target_os = "windows"
    ))]
    {
        Ok(())
    }
    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "linux",
        target_os = "macos",
        target_os = "openbsd",
        target_os = "windows"
    )))]
    {
        bail!(
            "secure Matrix session persistence is unavailable on this platform; re-login is required"
        )
    }
}

pub(crate) async fn save_session_material(
    session_path: &Path,
    latest_user_id_path: &Path,
    material: &mut SessionMaterial,
) -> Result<()> {
    ensure_system_credential_store_supported()?;
    let _guard = MATRIX_SESSION_STORE_LOCK.lock().await;
    let store = default_keyring_store();
    persist_secure_session_with_store(session_path, material, store).await?;
    write_private_file(
        latest_user_id_path.to_path_buf(),
        material.user_session.meta.user_id.as_bytes().to_vec(),
    )
    .await
}

pub(crate) async fn load_session_material(
    session_path: &Path,
    expected_user_id: &UserId,
) -> Result<SessionMaterial> {
    ensure_system_credential_store_supported()?;
    let _guard = MATRIX_SESSION_STORE_LOCK.lock().await;
    let store = default_keyring_store();
    load_session_material_with_store(session_path, expected_user_id, store).await
}

pub(crate) async fn clear_session_material(
    session_path: &Path,
    latest_user_id_path: &Path,
    user_id: &UserId,
) -> Result<()> {
    let _guard = MATRIX_SESSION_STORE_LOCK.lock().await;
    let store = default_keyring_store();
    clear_session_material_with_store(session_path, latest_user_id_path, user_id, store).await
}

pub(crate) fn referenced_db_path_from_slice(bytes: &[u8]) -> Result<PathBuf> {
    match decode_session_file(bytes)? {
        DecodedSessionFile::Secure(metadata) => {
            normalize_db_path_for_metadata(&metadata.client_session.db_path)
        }
        DecodedSessionFile::Legacy(material) => {
            normalize_db_path_for_metadata(&material.client_session.db_path)
        }
    }
}

async fn persist_secure_session_with_store(
    session_path: &Path,
    material: &mut SessionMaterial,
    store: Arc<dyn KeyringStore>,
) -> Result<()> {
    let user_id = &material.user_session.meta.user_id;
    let account = credential_account(user_id);
    let mut secrets = SessionSecretsPersisted {
        version: SESSION_CREDENTIAL_VERSION,
        database_passphrase: material.client_session.passphrase.clone(),
        user_session: material.user_session.clone(),
        sync_token: material.sync_token.clone(),
    };
    let serialized_result = serde_json::to_string(&secrets)
        .context("failed to serialize Matrix credentials for the OS credential store");
    secrets.database_passphrase.zeroize();
    wipe_session_tokens(&mut secrets.user_session);
    wipe_sync_token(&mut secrets.sync_token);
    let serialized_secrets = Zeroizing::new(serialized_result?);

    let homeserver = validate_homeserver_metadata(&material.client_session.homeserver)?;
    let db_path = normalize_db_path_for_metadata(&material.client_session.db_path)?;

    let metadata = SessionMetadataPersisted {
        version: SESSION_METADATA_VERSION,
        client_session: ClientSessionMetadataPersisted {
            homeserver,
            db_path,
        },
        credential_account: account.clone(),
        sliding_sync_version: material.sliding_sync_version,
    };
    let metadata_bytes = serde_json::to_vec(&metadata)
        .context("failed to serialize non-secret Matrix session metadata")?;

    let mut previous = keyring_load(store.clone(), account.clone()).await?;
    keyring_save(store.clone(), account.clone(), serialized_secrets).await?;

    if let Err(write_error) = write_private_file(session_path.to_path_buf(), metadata_bytes).await {
        let rollback_result = match previous.take() {
            Some(previous_value) => {
                keyring_save(store.clone(), account.clone(), previous_value).await
            }
            None => keyring_delete(store.clone(), account.clone())
                .await
                .map(|_| ()),
        };
        if let Err(rollback_error) = rollback_result {
            return Err(write_error).context(format!(
                "failed to write secure Matrix metadata and failed to roll back its credential: {rollback_error:#}"
            ));
        }
        return Err(write_error).context("failed to write secure Matrix session metadata");
    }

    Ok(())
}

async fn load_session_material_with_store(
    session_path: &Path,
    expected_user_id: &UserId,
    store: Arc<dyn KeyringStore>,
) -> Result<SessionMaterial> {
    harden_existing_private_file(session_path)?;
    let bytes = tokio::fs::read(session_path).await.with_context(|| {
        format!(
            "failed to read Matrix session file at {}",
            session_path.display()
        )
    })?;

    match decode_session_file(&bytes)? {
        DecodedSessionFile::Secure(metadata) => {
            load_secure_session(metadata, expected_user_id, store).await
        }
        DecodedSessionFile::Legacy(mut material) => {
            validate_session_user(&material, expected_user_id)?;
            let homeserver = validate_homeserver_metadata(&material.client_session.homeserver)?;
            let db_path = normalize_db_path_for_metadata(&material.client_session.db_path)?;
            persist_secure_session_with_store(session_path, &mut material, store)
                .await
                .context(
                    "legacy plaintext Matrix session was rejected because secure migration failed",
                )?;
            material.client_session.homeserver = homeserver;
            material.client_session.db_path = db_path;
            Ok(material)
        }
    }
}

async fn load_secure_session(
    metadata: SessionMetadataPersisted,
    expected_user_id: &UserId,
    store: Arc<dyn KeyringStore>,
) -> Result<SessionMaterial> {
    if metadata.version != SESSION_METADATA_VERSION {
        bail!(
            "unsupported Matrix session metadata version {}; expected {}",
            metadata.version,
            SESSION_METADATA_VERSION
        );
    }
    let expected_account = credential_account(expected_user_id);
    if metadata.credential_account != expected_account {
        bail!("Matrix session credential reference does not match the requested user");
    }

    let serialized_secrets = keyring_load(store, expected_account.clone())
        .await?
        .ok_or_else(|| {
            anyhow!(
                "Matrix credentials are missing from the OS credential store for {expected_account}"
            )
        })?;
    let parsed = serde_json::from_str::<SessionSecretsPersisted>(&serialized_secrets)
        .context("failed to decode Matrix credentials from the OS credential store");
    let secrets = parsed?;
    if secrets.version != SESSION_CREDENTIAL_VERSION {
        bail!(
            "unsupported Matrix credential version {}; expected {}",
            secrets.version,
            SESSION_CREDENTIAL_VERSION
        );
    }

    let homeserver = validate_homeserver_metadata(&metadata.client_session.homeserver)?;
    let db_path = normalize_db_path_for_metadata(&metadata.client_session.db_path)?;
    let material = SessionMaterial {
        client_session: ClientSessionPersisted {
            homeserver,
            db_path,
            passphrase: secrets.database_passphrase,
        },
        user_session: secrets.user_session,
        sync_token: secrets.sync_token,
        sliding_sync_version: metadata.sliding_sync_version,
    };
    validate_session_user(&material, expected_user_id)?;
    Ok(material)
}

fn validate_session_user(material: &SessionMaterial, expected_user_id: &UserId) -> Result<()> {
    if material.user_session.meta.user_id != expected_user_id {
        bail!("persisted Matrix session user does not match the requested user");
    }
    Ok(())
}

fn validate_homeserver_metadata(raw: &str) -> Result<String> {
    let parsed = Url::parse(raw).context("Matrix homeserver metadata is not a valid URL")?;
    if !matches!(parsed.scheme(), "http" | "https") {
        bail!("Matrix homeserver metadata must use http or https");
    }
    if !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.query().is_some()
        || parsed.fragment().is_some()
    {
        bail!("Matrix homeserver metadata contains credentials or URL secrets");
    }
    Ok(parsed.to_string())
}

fn normalize_db_path_for_metadata(path: &Path) -> Result<PathBuf> {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .context("Matrix database path has no UTF-8 file name")?;
    if !name.starts_with("db_") || name.contains('/') || name.contains('\\') {
        bail!("Matrix database metadata must name an app-owned db_* directory");
    }
    Ok(PathBuf::from(name))
}

fn decode_session_file(bytes: &[u8]) -> Result<DecodedSessionFile> {
    let value: serde_json::Value =
        serde_json::from_slice(bytes).context("failed to decode Matrix session JSON")?;
    if value.get("version").is_some() {
        let metadata = serde_json::from_value::<SessionMetadataPersisted>(value)
            .context("failed to decode secure Matrix session metadata")?;
        return Ok(DecodedSessionFile::Secure(metadata));
    }

    let legacy = serde_json::from_value::<LegacyFullSessionPersisted>(value)
        .context("failed to decode legacy Matrix session for secure migration")?;
    Ok(DecodedSessionFile::Legacy(SessionMaterial {
        client_session: ClientSessionPersisted {
            homeserver: legacy.client_session.homeserver,
            db_path: legacy.client_session.db_path,
            passphrase: legacy.client_session.passphrase,
        },
        user_session: legacy.user_session,
        sync_token: legacy.sync_token,
        sliding_sync_version: legacy.sliding_sync_version,
    }))
}

async fn clear_session_material_with_store(
    session_path: &Path,
    latest_user_id_path: &Path,
    user_id: &UserId,
    store: Arc<dyn KeyringStore>,
) -> Result<()> {
    let account = credential_account(user_id);
    let mut failures = Vec::new();

    if let Err(error) = keyring_delete(store, account).await {
        failures.push(format!("OS credential deletion failed: {error:#}"));
    }
    if let Err(error) = remove_file_if_exists(session_path).await {
        failures.push(format!("session metadata deletion failed: {error:#}"));
    }
    if let Err(error) = remove_latest_user_id_if_matches(latest_user_id_path, user_id).await {
        failures.push(format!("latest-user pointer deletion failed: {error:#}"));
    }

    if failures.is_empty() {
        Ok(())
    } else {
        bail!(
            "local Matrix session cleanup was incomplete: {}",
            failures.join("; ")
        )
    }
}

async fn remove_latest_user_id_if_matches(path: &Path, user_id: &UserId) -> Result<()> {
    let current = match tokio::fs::read_to_string(path).await {
        Ok(current) => current,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => {
            return Err(error)
                .with_context(|| format!("failed to read latest user ID at {}", path.display()));
        }
    };
    if current.trim() != user_id.as_str() {
        return Ok(());
    }
    remove_file_if_exists(path).await
}

async fn remove_file_if_exists(path: &Path) -> Result<()> {
    match tokio::fs::remove_file(path).await {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("failed to remove {}", path.display())),
    }
}

pub(crate) async fn write_latest_user_id(path: &Path, user_id: &UserId) -> Result<()> {
    let _guard = MATRIX_SESSION_STORE_LOCK.lock().await;
    write_private_file(path.to_path_buf(), user_id.as_bytes().to_vec()).await
}

async fn keyring_load(
    store: Arc<dyn KeyringStore>,
    account: String,
) -> Result<Option<Zeroizing<String>>> {
    let loaded = tokio::task::spawn_blocking(move || {
        store
            .load(MATRIX_CREDENTIAL_SERVICE, &account)
            .map_err(|error| anyhow!(error.message()))
            .with_context(|| format!("failed to load Matrix credentials for {account}"))
    })
    .await
    .context("Matrix credential-store read task failed")??;
    Ok(loaded.map(Zeroizing::new))
}

async fn keyring_save(
    store: Arc<dyn KeyringStore>,
    account: String,
    value: Zeroizing<String>,
) -> Result<()> {
    tokio::task::spawn_blocking(move || {
        store
            .save(MATRIX_CREDENTIAL_SERVICE, &account, &value)
            .map_err(|error| anyhow!(error.message()))
            .with_context(|| format!("failed to save Matrix credentials for {account}"))
    })
    .await
    .context("Matrix credential-store write task failed")?
}

async fn keyring_delete(store: Arc<dyn KeyringStore>, account: String) -> Result<bool> {
    tokio::task::spawn_blocking(move || {
        store
            .delete(MATRIX_CREDENTIAL_SERVICE, &account)
            .map_err(|error| anyhow!(error.message()))
            .with_context(|| format!("failed to delete Matrix credentials for {account}"))
    })
    .await
    .context("Matrix credential-store delete task failed")?
}

async fn write_private_file(path: PathBuf, contents: Vec<u8>) -> Result<()> {
    tokio::task::spawn_blocking(move || write_private_file_atomically(&path, &contents))
        .await
        .context("private-file writer task failed")?
}

fn harden_existing_private_file(path: &Path) -> Result<()> {
    let Some(parent) = path.parent() else {
        bail!("Matrix session path has no parent: {}", path.display());
    };
    ensure_private_directory(parent)?;
    if path.exists() {
        set_private_file_permissions(path)?;
    }
    Ok(())
}

fn write_private_file_atomically(path: &Path, contents: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .with_context(|| format!("private file path has no parent: {}", path.display()))?;
    ensure_private_directory(parent)?;

    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("private-state");
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    let tmp_path = parent.join(format!(".{file_name}.tmp-{}-{nonce}", std::process::id()));

    let write_result = (|| -> Result<()> {
        let mut options = OpenOptions::new();
        options.create_new(true).write(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        let mut tmp_file = options.open(&tmp_path).with_context(|| {
            format!("failed to create private temp file {}", tmp_path.display())
        })?;
        tmp_file
            .write_all(contents)
            .with_context(|| format!("failed to write private temp file {}", tmp_path.display()))?;
        tmp_file
            .sync_all()
            .with_context(|| format!("failed to sync private temp file {}", tmp_path.display()))?;
        drop(tmp_file);
        fs::rename(&tmp_path, path).with_context(|| {
            format!(
                "failed to atomically replace {} with {}",
                path.display(),
                tmp_path.display()
            )
        })?;
        set_private_file_permissions(path)?;
        sync_directory(parent)?;
        Ok(())
    })();

    if write_result.is_err() {
        let _ = fs::remove_file(&tmp_path);
    }
    write_result
}

fn ensure_private_directory(path: &Path) -> Result<()> {
    fs::create_dir_all(path)
        .with_context(|| format!("failed to create private directory {}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))
            .with_context(|| format!("failed to set private permissions on {}", path.display()))?;
    }
    Ok(())
}

fn set_private_file_permissions(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))
            .with_context(|| format!("failed to set private permissions on {}", path.display()))?;
    }
    Ok(())
}

fn sync_directory(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        File::open(path)
            .and_then(|directory| directory.sync_all())
            .with_context(|| format!("failed to sync private directory {}", path.display()))?;
    }
    Ok(())
}

pub(crate) fn wipe_client_passphrase(client_session: &mut ClientSessionPersisted) {
    client_session.passphrase.zeroize();
}

pub(crate) fn wipe_session_tokens(session: &mut MatrixSession) {
    session.tokens.access_token.zeroize();
    if let Some(refresh_token) = session.tokens.refresh_token.as_mut() {
        refresh_token.zeroize();
    }
}

pub(crate) fn wipe_sync_token(sync_token: &mut Option<String>) {
    if let Some(sync_token) = sync_token.as_mut() {
        sync_token.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_keyring_store::tests::MockKeyringStore;
    use matrix_sdk::SessionMeta;
    use matrix_sdk::SessionTokens;
    use matrix_sdk::ruma::owned_device_id;
    use matrix_sdk::ruma::owned_user_id;

    fn sample_material() -> SessionMaterial {
        SessionMaterial {
            client_session: ClientSessionPersisted {
                homeserver: "https://matrix.example.org".to_string(),
                db_path: PathBuf::from("db_2026_07_21"),
                passphrase: "database-secret-passphrase".to_string(),
            },
            user_session: MatrixSession {
                meta: SessionMeta {
                    user_id: owned_user_id!("@alice:example.org"),
                    device_id: owned_device_id!("HEPTADEVICE"),
                },
                tokens: SessionTokens {
                    access_token: "matrix-access-secret".to_string(),
                    refresh_token: Some("matrix-refresh-secret".to_string()),
                },
            },
            sync_token: Some("matrix-sync-secret".to_string()),
            sliding_sync_version: SlidingSyncVersion::Native,
        }
    }

    fn session_paths(temp: &tempfile::TempDir) -> (PathBuf, PathBuf) {
        let state_dir = temp.path().join("alice").join("persistent_state");
        (
            state_dir.join("session"),
            temp.path().join("latest_user_id.txt"),
        )
    }

    #[tokio::test]
    async fn secure_save_keeps_all_credentials_out_of_metadata() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let (session_path, _) = session_paths(&temp);
        let keyring = Arc::new(MockKeyringStore::default());
        let mut material = sample_material();
        persist_secure_session_with_store(&session_path, &mut material, keyring.clone()).await?;

        let metadata = fs::read_to_string(&session_path)?;
        for secret in [
            "database-secret-passphrase",
            "matrix-access-secret",
            "matrix-refresh-secret",
            "matrix-sync-secret",
        ] {
            assert!(!metadata.contains(secret));
        }
        let parsed: SessionMetadataPersisted = serde_json::from_str(&metadata)?;
        assert_eq!(parsed.version, SESSION_METADATA_VERSION);
        assert_eq!(
            parsed.client_session.db_path,
            PathBuf::from("db_2026_07_21")
        );

        let account = credential_account(&material.user_session.meta.user_id);
        let stored = keyring
            .saved_value_for_service(MATRIX_CREDENTIAL_SERVICE, &account)
            .expect("credential payload should exist");
        assert!(stored.contains("matrix-access-secret"));
        assert!(stored.contains("matrix-refresh-secret"));

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            assert_eq!(
                fs::metadata(&session_path)?.permissions().mode() & 0o777,
                0o600
            );
            assert_eq!(
                fs::metadata(session_path.parent().expect("session parent"))?
                    .permissions()
                    .mode()
                    & 0o777,
                0o700
            );
        }
        Ok(())
    }

    #[tokio::test]
    async fn legacy_plaintext_is_migrated_before_it_is_restored() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let (session_path, _) = session_paths(&temp);
        fs::create_dir_all(session_path.parent().expect("session parent"))?;
        let material = sample_material();
        let legacy = serde_json::json!({
            "client_session": {
                "homeserver": material.client_session.homeserver.clone(),
                "db_path": material.client_session.db_path.clone(),
                "passphrase": material.client_session.passphrase.clone(),
            },
            "user_session": material.user_session.clone(),
            "sync_token": material.sync_token.clone(),
            "sliding_sync_version": material.sliding_sync_version,
        });
        fs::write(&session_path, serde_json::to_vec(&legacy)?)?;

        let keyring = Arc::new(MockKeyringStore::default());
        let restored = load_session_material_with_store(
            &session_path,
            &material.user_session.meta.user_id,
            keyring.clone(),
        )
        .await?;
        assert_eq!(
            restored.client_session.passphrase,
            "database-secret-passphrase"
        );
        assert_eq!(restored.user_session, material.user_session);

        let migrated = fs::read_to_string(&session_path)?;
        assert!(!migrated.contains("database-secret-passphrase"));
        assert!(!migrated.contains("matrix-access-secret"));
        assert_eq!(
            serde_json::from_str::<SessionMetadataPersisted>(&migrated)?.version,
            SESSION_METADATA_VERSION
        );
        Ok(())
    }

    #[tokio::test]
    async fn missing_keyring_entry_fails_closed_without_plaintext_fallback() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let (session_path, _) = session_paths(&temp);
        let keyring = Arc::new(MockKeyringStore::default());
        let mut material = sample_material();
        persist_secure_session_with_store(&session_path, &mut material, keyring.clone()).await?;
        let account = credential_account(&material.user_session.meta.user_id);
        assert!(keyring.delete(MATRIX_CREDENTIAL_SERVICE, &account)?);

        let error = load_session_material_with_store(
            &session_path,
            &material.user_session.meta.user_id,
            keyring,
        )
        .await
        .err()
        .expect("missing OS credential must reject restore");
        assert!(
            error
                .to_string()
                .contains("missing from the OS credential store")
        );
        assert!(!fs::read_to_string(&session_path)?.contains("matrix-access-secret"));
        Ok(())
    }

    #[tokio::test]
    async fn logout_cleanup_removes_credentials_metadata_and_matching_pointer() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let (session_path, latest_path) = session_paths(&temp);
        let keyring = Arc::new(MockKeyringStore::default());
        let mut material = sample_material();
        persist_secure_session_with_store(&session_path, &mut material, keyring.clone()).await?;
        write_private_file_atomically(&latest_path, material.user_session.meta.user_id.as_bytes())?;
        let account = credential_account(&material.user_session.meta.user_id);

        clear_session_material_with_store(
            &session_path,
            &latest_path,
            &material.user_session.meta.user_id,
            keyring.clone(),
        )
        .await?;

        assert!(!session_path.exists());
        assert!(!latest_path.exists());
        assert!(!keyring.contains_for_service(MATRIX_CREDENTIAL_SERVICE, &account));
        Ok(())
    }

    #[tokio::test]
    async fn credential_for_another_user_is_rejected() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let (session_path, _) = session_paths(&temp);
        let keyring = Arc::new(MockKeyringStore::default());
        let mut material = sample_material();
        persist_secure_session_with_store(&session_path, &mut material, keyring.clone()).await?;

        let other_user = owned_user_id!("@mallory:example.org");
        let error = load_session_material_with_store(&session_path, &other_user, keyring)
            .await
            .err()
            .expect("cross-user credential reference must be rejected");
        assert!(
            error
                .to_string()
                .contains("does not match the requested user")
        );
        Ok(())
    }

    #[tokio::test]
    async fn homeserver_url_secrets_are_rejected_before_keyring_mutation() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let (session_path, _) = session_paths(&temp);
        let keyring = Arc::new(MockKeyringStore::default());
        let mut material = sample_material();
        material.client_session.homeserver =
            "https://alice:homeserver-secret@matrix.example.org/".to_string();
        let account = credential_account(&material.user_session.meta.user_id);

        let error =
            persist_secure_session_with_store(&session_path, &mut material, keyring.clone())
                .await
                .err()
                .expect("credential-bearing homeserver URL must be rejected");
        assert!(error.to_string().contains("credentials or URL secrets"));
        assert!(!session_path.exists());
        assert!(!keyring.contains_for_service(MATRIX_CREDENTIAL_SERVICE, &account));
        Ok(())
    }
}
