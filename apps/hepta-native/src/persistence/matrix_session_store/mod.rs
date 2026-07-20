//! Secure persistence for Matrix session credentials and non-secret metadata.

mod credential;
mod model;
mod private_file;
#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use codex_keyring_store::KeyringStore;
use matrix_sdk::ruma::UserId;
use serde::de::IgnoredAny;
use url::Url;
use zeroize::Zeroize;
use zeroize::Zeroizing;

use credential::credential_account;
use credential::default_keyring_store;
use credential::ensure_system_credential_store_supported;
use credential::keyring_delete;
use credential::keyring_load;
use credential::keyring_save;
use model::ClientSessionMetadataPersisted;
use model::LegacyFullSessionPersisted;
use model::SessionMetadataPersisted;
use model::SessionSecretsPersisted;
use model::wipe_session_material;
use private_file::harden_existing_private_file;
use private_file::remove_file_if_exists;
use private_file::write_private_file;

pub use model::ClientSessionPersisted;
pub use model::SlidingSyncVersion;
pub(crate) use model::SessionMaterial;
pub(crate) use model::wipe_client_passphrase;
pub(crate) use model::wipe_session_tokens;
pub(crate) use model::wipe_sync_token;

const SESSION_METADATA_VERSION: u8 = 2;
const SESSION_CREDENTIAL_VERSION: u8 = 1;
static MATRIX_SESSION_STORE_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

enum DecodedSessionFile {
    Secure(SessionMetadataPersisted),
    Legacy(SessionMaterial),
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
        DecodedSessionFile::Legacy(mut material) => {
            let result = normalize_db_path_for_metadata(&material.client_session.db_path);
            wipe_session_material(&mut material);
            result
        }
    }
}

async fn persist_secure_session_with_store(
    session_path: &Path,
    material: &mut SessionMaterial,
    store: Arc<dyn KeyringStore>,
) -> Result<()> {
    let account = credential_account(&material.user_session.meta.user_id);
    let mut secrets = SessionSecretsPersisted {
        version: SESSION_CREDENTIAL_VERSION,
        database_passphrase: material.client_session.passphrase.clone(),
        user_session: material.user_session.clone(),
        sync_token: material.sync_token.clone(),
    };
    let serialized_result = serde_json::to_string(&secrets)
        .context("failed to serialize Matrix credentials for the OS credential store");
    secrets.wipe();
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
    let mut bytes = tokio::fs::read(session_path).await.with_context(|| {
        format!(
            "failed to read Matrix session file at {}",
            session_path.display()
        )
    })?;
    let decoded = decode_session_file(&bytes);
    bytes.zeroize();

    match decoded? {
        DecodedSessionFile::Secure(metadata) => {
            load_secure_session(metadata, expected_user_id, store).await
        }
        DecodedSessionFile::Legacy(mut material) => {
            let validated = validate_material_metadata(&material, expected_user_id);
            let (homeserver, db_path) = match validated {
                Ok(validated) => validated,
                Err(error) => {
                    wipe_session_material(&mut material);
                    return Err(error);
                }
            };
            let migration = persist_secure_session_with_store(session_path, &mut material, store)
                .await
                .context(
                    "legacy plaintext Matrix session was rejected because secure migration failed",
                );
            if let Err(error) = migration {
                wipe_session_material(&mut material);
                return Err(error);
            }
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
    let mut secrets = parsed?;
    let validated = (|| -> Result<(String, PathBuf)> {
        if secrets.version != SESSION_CREDENTIAL_VERSION {
            bail!(
                "unsupported Matrix credential version {}; expected {}",
                secrets.version,
                SESSION_CREDENTIAL_VERSION
            );
        }
        if secrets.user_session.meta.user_id != expected_user_id {
            bail!("persisted Matrix session user does not match the requested user");
        }
        Ok((
            validate_homeserver_metadata(&metadata.client_session.homeserver)?,
            normalize_db_path_for_metadata(&metadata.client_session.db_path)?,
        ))
    })();
    let (homeserver, db_path) = match validated {
        Ok(validated) => validated,
        Err(error) => {
            secrets.wipe();
            return Err(error);
        }
    };

    Ok(SessionMaterial {
        client_session: ClientSessionPersisted {
            homeserver,
            db_path,
            passphrase: secrets.database_passphrase,
        },
        user_session: secrets.user_session,
        sync_token: secrets.sync_token,
        sliding_sync_version: metadata.sliding_sync_version,
    })
}

fn validate_material_metadata(
    material: &SessionMaterial,
    expected_user_id: &UserId,
) -> Result<(String, PathBuf)> {
    if material.user_session.meta.user_id != expected_user_id {
        bail!("persisted Matrix session user does not match the requested user");
    }
    Ok((
        validate_homeserver_metadata(&material.client_session.homeserver)?,
        normalize_db_path_for_metadata(&material.client_session.db_path)?,
    ))
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
    let top_level: BTreeMap<String, IgnoredAny> =
        serde_json::from_slice(bytes).context("failed to decode Matrix session JSON")?;
    if top_level.contains_key("version") {
        let metadata = serde_json::from_slice::<SessionMetadataPersisted>(bytes)
            .context("failed to decode secure Matrix session metadata")?;
        return Ok(DecodedSessionFile::Secure(metadata));
    }

    let legacy = serde_json::from_slice::<LegacyFullSessionPersisted>(bytes)
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

pub(crate) async fn write_latest_user_id(path: &Path, user_id: &UserId) -> Result<()> {
    let _guard = MATRIX_SESSION_STORE_LOCK.lock().await;
    write_private_file(path.to_path_buf(), user_id.as_bytes().to_vec()).await
}
