//! Secure persistence for Matrix session credentials and non-secret metadata.

mod credential;
mod model;
mod private_file;
#[cfg(test)]
mod tests;

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context, Result, anyhow, bail};
use chacha20poly1305::{
    ChaCha20Poly1305, Nonce,
    aead::{Aead, KeyInit, Payload},
};
use codex_keyring_store::KeyringStore;
use matrix_sdk::ruma::UserId;
use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize, de::IgnoredAny};
use url::Url;
use zeroize::{Zeroize, Zeroizing};
use credential::{
    credential_account, default_keyring_store, ensure_system_credential_store_supported,
    keyring_delete, keyring_load, keyring_save, legacy_credential_account,
};
use model::{
    ClientSessionMetadataPersisted, LegacyFullSessionPersisted, LegacySecureMetadataV2,
    SessionCredentialKeyPersisted, SessionMetadataPersisted, SessionSecretsPersisted,
    wipe_session_material,
};
use private_file::{harden_existing_private_file, remove_file_if_exists, write_private_file};
pub use model::{ClientSessionPersisted, SlidingSyncVersion};
pub(crate) use model::{SessionMaterial, wipe_client_passphrase, wipe_session_tokens, wipe_sync_token};

const SESSION_METADATA_VERSION: u8 = 3;
const LEGACY_SECURE_METADATA_VERSION: u8 = 2;
const SESSION_CREDENTIAL_VERSION: u8 = 2;
const SESSION_SECRET_PAYLOAD_VERSION: u8 = 1;
const METADATA_AUTHENTICATION_KEY_LEN: usize = 32;
const BINDING_NONCE_LEN: usize = 16;
const ENCRYPTION_NONCE_LEN: usize = 12;
const MAX_CREDENTIAL_ALLOCATION_ATTEMPTS: usize = 8;
static MATRIX_SESSION_STORE_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

enum DecodedSessionFile {
    Secure(SessionMetadataPersisted),
    LegacySecure(LegacySecureMetadataV2),
    LegacyPlaintext(SessionMaterial),
}

enum CredentialCleanupAuthorization {
    Owned(Option<String>),
    DifferentUser,
}

#[derive(Serialize)]
struct SessionBindingClaims<'a> {
    domain: &'static str,
    metadata_version: u8,
    credential_version: u8,
    user_id: &'a str,
    homeserver: &'a str,
    db_path: &'a str,
    credential_account: &'a str,
    binding_nonce: &'a str,
    encryption_nonce: &'a str,
    sliding_sync_version: SlidingSyncVersion,
}

#[derive(Deserialize)]
struct VersionOnly {
    version: u8,
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

/// Retire a legacy-location session only after a canonical collision-resistant
/// copy has committed. Unlike logout cleanup, this deliberately leaves the
/// latest-user pointer intact.
pub(crate) async fn retire_session_material(session_path: &Path, user_id: &UserId) -> Result<()> {
    let _guard = MATRIX_SESSION_STORE_LOCK.lock().await;
    retire_session_material_with_store(session_path, user_id, default_keyring_store()).await
}

/// Authenticates a saved session before allowing its database path to influence
/// orphan pruning. The caller-owned buffer is zeroized on every path, including
/// malformed legacy plaintext.
pub(crate) async fn referenced_db_path_from_slice(bytes: &mut [u8]) -> Result<PathBuf> {
    ensure_system_credential_store_supported()?;
    let decoded = decode_session_file_and_zeroize(bytes);
    let metadata = match decoded? {
        DecodedSessionFile::Secure(metadata) => metadata,
        DecodedSessionFile::LegacySecure(_) => {
            bail!("unauthenticated v2 Matrix metadata cannot authorize database pruning")
        }
        DecodedSessionFile::LegacyPlaintext(_) => {
            bail!("legacy plaintext Matrix state cannot authorize database pruning")
        }
    };
    let expected_user_id = metadata.user_id.clone();
    let material =
        load_secure_session(metadata, &expected_user_id, default_keyring_store()).await?;
    let db_path = normalize_db_path_for_metadata(&material.client_session.db_path);
    drop(material);
    db_path
}

async fn persist_secure_session_with_store(
    session_path: &Path,
    material: &mut SessionMaterial,
    store: Arc<dyn KeyringStore>,
) -> Result<()> {
    persist_secure_session_with_store_impl(session_path, material, store, false).await
}

#[cfg(test)]
async fn persist_secure_session_with_forced_write_failure(
    session_path: &Path,
    material: &mut SessionMaterial,
    store: Arc<dyn KeyringStore>,
) -> Result<()> {
    persist_secure_session_with_store_impl(session_path, material, store, true).await
}

async fn persist_secure_session_with_store_impl(
    session_path: &Path,
    material: &mut SessionMaterial,
    store: Arc<dyn KeyringStore>,
    force_envelope_write_failure: bool,
) -> Result<()> {
    let homeserver = validate_homeserver_metadata(&material.client_session.homeserver)?;
    let db_path = normalize_db_path_for_metadata(&material.client_session.db_path)?;
    let user_id = material.user_session.meta.user_id.clone();
    // A pre-existing file at the collision-resistant path must authenticate as
    // this exact full Matrix user ID before it may be replaced. Corrupt data or
    // even a theoretical digest collision fails closed and is never destroyed.
    let previous_account =
        authenticated_existing_account(session_path, &user_id, store.clone()).await?;

    let (account, binding_nonce) = allocate_credential_account(&user_id, store.clone()).await?;
    let mut master_key = vec![0_u8; METADATA_AUTHENTICATION_KEY_LEN];
    OsRng.fill_bytes(&mut master_key);
    let mut encryption_nonce_bytes = [0_u8; ENCRYPTION_NONCE_LEN];
    OsRng.fill_bytes(&mut encryption_nonce_bytes);

    let mut metadata = SessionMetadataPersisted {
        version: SESSION_METADATA_VERSION,
        user_id: user_id.clone(),
        client_session: ClientSessionMetadataPersisted {
            homeserver,
            db_path,
        },
        credential_account: account.clone(),
        binding_nonce: binding_nonce.clone(),
        binding_tag: String::new(),
        encryption_nonce: encode_hex(&encryption_nonce_bytes),
        encrypted_secrets: String::new(),
        sliding_sync_version: material.sliding_sync_version,
    };

    let mut secrets = SessionSecretsPersisted {
        version: SESSION_SECRET_PAYLOAD_VERSION,
        database_passphrase: material.client_session.passphrase.clone(),
        user_session: material.user_session.clone(),
        sync_token: material.sync_token.clone(),
    };
    let serialized_result = serde_json::to_vec(&secrets)
        .context("failed to serialize Matrix credentials for encrypted private storage");
    secrets.wipe();
    let mut serialized_secrets = serialized_result?;
    let binding_claims = encode_binding_claims(&metadata)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&master_key)
        .map_err(|_| anyhow!("failed to initialize Matrix session encryption"))?;
    let encrypted = cipher
        .encrypt(
            Nonce::from_slice(&encryption_nonce_bytes),
            Payload {
                msg: &serialized_secrets,
                aad: &binding_claims,
            },
        )
        .map_err(|_| anyhow!("failed to encrypt Matrix session credentials"));
    serialized_secrets.zeroize();
    encryption_nonce_bytes.zeroize();
    metadata.encrypted_secrets = encode_hex(&encrypted?);
    metadata.binding_tag = compute_binding_tag(&metadata, &master_key)?;

    let mut credential_key = SessionCredentialKeyPersisted {
        version: SESSION_CREDENTIAL_VERSION,
        master_key,
        binding_nonce,
    };
    let serialized_key_result = serde_json::to_string(&credential_key)
        .context("failed to serialize Matrix session key for the OS credential store");
    credential_key.wipe();
    let serialized_key = Zeroizing::new(serialized_key_result?);
    if serialized_key.len() > 512 {
        bail!("Matrix OS credential payload exceeded its fixed safety bound");
    }

    let metadata_bytes = serde_json::to_vec(&metadata)
        .context("failed to serialize encrypted Matrix session envelope")?;

    keyring_save(store.clone(), account.clone(), serialized_key).await?;
    let envelope_write = if force_envelope_write_failure {
        Err(anyhow!(
            "injected secure Matrix session envelope write failure"
        ))
    } else {
        write_private_file(session_path.to_path_buf(), metadata_bytes).await
    };
    if let Err(write_error) = envelope_write {
        let rollback_result = keyring_delete(store.clone(), account.clone())
            .await
            .map(|_| ());
        if let Err(rollback_error) = rollback_result {
            return Err(write_error).context(format!(
                "failed to write secure Matrix metadata and failed to remove its uncommitted credential: {rollback_error:#}"
            ));
        }
        return Err(write_error).context("failed to write secure Matrix session metadata");
    }

    if let Some(previous_account) = previous_account.filter(|previous| previous != &account) {
        keyring_delete(store, previous_account)
            .await
            .context("new Matrix session was committed but old credential cleanup failed")?;
    }
    Ok(())
}

async fn allocate_credential_account(
    user_id: &UserId,
    store: Arc<dyn KeyringStore>,
) -> Result<(String, String)> {
    for _ in 0..MAX_CREDENTIAL_ALLOCATION_ATTEMPTS {
        let mut nonce = [0_u8; BINDING_NONCE_LEN];
        OsRng.fill_bytes(&mut nonce);
        let binding_nonce = encode_hex(&nonce);
        nonce.zeroize();
        let account = credential_account(user_id, &binding_nonce);
        if keyring_load(store.clone(), account.clone())
            .await?
            .is_none()
        {
            return Ok((account, binding_nonce));
        }
    }
    bail!("could not allocate a unique Matrix credential generation")
}

async fn authenticated_existing_account(
    session_path: &Path,
    expected_user_id: &UserId,
    store: Arc<dyn KeyringStore>,
) -> Result<Option<String>> {
    let mut bytes = match tokio::fs::read(session_path).await {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(error).with_context(|| {
                format!(
                    "failed to read previous Matrix metadata at {}",
                    session_path.display()
                )
            });
        }
    };
    let decoded = decode_session_file_and_zeroize(&mut bytes);
    let metadata = match decoded? {
        DecodedSessionFile::Secure(metadata) => metadata,
        DecodedSessionFile::LegacySecure(metadata) => {
            let expected_account = legacy_credential_account(expected_user_id);
            if metadata.credential_account != expected_account {
                bail!("existing legacy Matrix metadata belongs to a different user");
            }
            return Ok(Some(expected_account));
        }
        DecodedSessionFile::LegacyPlaintext(material) => {
            if material.user_session.meta.user_id != expected_user_id {
                bail!("existing legacy Matrix session belongs to a different user");
            }
            return Ok(None);
        }
    };
    let account = metadata.credential_account.clone();
    let material = load_secure_session(metadata, expected_user_id, store).await?;
    drop(material);
    Ok(Some(account))
}

fn compute_binding_tag(
    metadata: &SessionMetadataPersisted,
    authentication_key: &[u8],
) -> Result<String> {
    let key: &[u8; METADATA_AUTHENTICATION_KEY_LEN] = authentication_key
        .try_into()
        .map_err(|_| anyhow!("Matrix metadata authentication key has an invalid length"))?;
    let encoded = encode_binding_claims(metadata)?;
    let mut hasher = blake3::Hasher::new_keyed(key);
    hasher.update(&encoded);
    hasher.update(b"\0encrypted-secrets\0");
    hasher.update(metadata.encrypted_secrets.as_bytes());
    Ok(encode_hex(hasher.finalize().as_bytes()))
}

fn encode_binding_claims(metadata: &SessionMetadataPersisted) -> Result<Vec<u8>> {
    let db_path = metadata
        .client_session
        .db_path
        .to_str()
        .context("Matrix database metadata has a non-UTF-8 path")?;
    let claims = SessionBindingClaims {
        domain: "ai.hepta.native.matrix-session-binding.v1",
        metadata_version: metadata.version,
        credential_version: SESSION_CREDENTIAL_VERSION,
        user_id: metadata.user_id.as_str(),
        homeserver: &metadata.client_session.homeserver,
        db_path,
        credential_account: &metadata.credential_account,
        binding_nonce: &metadata.binding_nonce,
        encryption_nonce: &metadata.encryption_nonce,
        sliding_sync_version: metadata.sliding_sync_version,
    };
    serde_json::to_vec(&claims).context("failed to encode Matrix metadata authentication claims")
}

fn binding_tags_equal(expected: &str, actual: &str) -> bool {
    if expected.len() != actual.len() {
        return false;
    }
    expected
        .as_bytes()
        .iter()
        .zip(actual.as_bytes())
        .fold(0_u8, |difference, (left, right)| {
            difference | (left ^ right)
        })
        == 0
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

fn decode_hex(encoded: &str) -> Result<Vec<u8>> {
    if encoded.len() % 2 != 0 {
        bail!("hex-encoded Matrix session field has an invalid length");
    }
    let mut decoded = Vec::with_capacity(encoded.len() / 2);
    for pair in encoded.as_bytes().chunks_exact(2) {
        let high = decode_hex_nibble(pair[0])?;
        let low = decode_hex_nibble(pair[1])?;
        decoded.push((high << 4) | low);
    }
    Ok(decoded)
}

fn decode_hex_nibble(value: u8) -> Result<u8> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => bail!("hex-encoded Matrix session field contains an invalid character"),
    }
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
    let delete_if_decode_fails = session_file_is_legacy_or_malformed(&bytes);
    let decoded = decode_session_file_and_zeroize(&mut bytes);

    let decoded = match decoded {
        Ok(decoded) => decoded,
        Err(error) if delete_if_decode_fails => {
            remove_file_if_exists(session_path).await.with_context(|| {
                format!(
                    "invalid plaintext/corrupt Matrix session required deletion at {}",
                    session_path.display()
                )
            })?;
            return Err(error)
                .context("invalid legacy Matrix session was deleted; secure re-login is required");
        }
        Err(error) => return Err(error),
    };

    match decoded {
        DecodedSessionFile::Secure(metadata) => {
            load_secure_session(metadata, expected_user_id, store).await
        }
        DecodedSessionFile::LegacySecure(metadata) => {
            if metadata.version != LEGACY_SECURE_METADATA_VERSION {
                bail!("unsupported legacy Matrix metadata version")
            }
            bail!(
                "unauthenticated Matrix metadata v2 cannot be restored; secure re-login is required"
            )
        }
        DecodedSessionFile::LegacyPlaintext(mut material) => {
            let validated = validate_material_metadata(&material, expected_user_id);
            let (homeserver, db_path) = match validated {
                Ok(validated) => validated,
                Err(error) => {
                    wipe_session_material(&mut material);
                    remove_file_if_exists(session_path).await.with_context(|| {
                        format!(
                            "invalid legacy plaintext Matrix session required deletion at {}",
                            session_path.display()
                        )
                    })?;
                    return Err(error).context(
                        "invalid legacy plaintext Matrix session was deleted; re-login is required",
                    );
                }
            };
            let migration = persist_secure_session_with_store(session_path, &mut material, store)
                .await
                .context(
                    "legacy plaintext Matrix session was rejected because secure migration failed",
                );
            if let Err(error) = migration {
                wipe_session_material(&mut material);
                remove_file_if_exists(session_path).await.with_context(|| {
                    format!(
                        "unmigrated plaintext Matrix session required deletion at {}",
                        session_path.display()
                    )
                })?;
                return Err(error).context(
                    "legacy plaintext Matrix session was deleted after secure migration failed",
                );
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
    if metadata.user_id != expected_user_id {
        bail!("Matrix metadata user does not match the requested user");
    }
    if metadata.binding_nonce.len() != BINDING_NONCE_LEN * 2
        || !metadata
            .binding_nonce
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        bail!("Matrix session binding nonce is invalid");
    }
    let expected_account = credential_account(expected_user_id, &metadata.binding_nonce);
    if metadata.credential_account != expected_account {
        bail!("Matrix session credential reference does not match the requested user");
    }

    let serialized_key = keyring_load(store, expected_account.clone())
        .await?
        .ok_or_else(|| {
            anyhow!(
                "Matrix credentials are missing from the OS credential store for {expected_account}"
            )
        })?;
    if serialized_key.len() > 512 {
        bail!("Matrix OS credential payload exceeds its fixed safety bound");
    }
    let parsed_key = serde_json::from_str::<SessionCredentialKeyPersisted>(&serialized_key)
        .context("failed to decode Matrix session key from the OS credential store");
    let credential_key = parsed_key?;
    let validated = (|| -> Result<(String, PathBuf, SessionMetadataPersisted)> {
        if credential_key.version != SESSION_CREDENTIAL_VERSION {
            bail!(
                "unsupported Matrix credential version {}; expected {}",
                credential_key.version,
                SESSION_CREDENTIAL_VERSION
            );
        }
        if credential_key.binding_nonce != metadata.binding_nonce {
            bail!("Matrix credential and metadata generations do not match");
        }
        if credential_key.master_key.len() != METADATA_AUTHENTICATION_KEY_LEN {
            bail!("Matrix session key has an invalid length");
        }
        let homeserver = validate_homeserver_metadata(&metadata.client_session.homeserver)?;
        let db_path = normalize_db_path_for_metadata(&metadata.client_session.db_path)?;
        let canonical_metadata = SessionMetadataPersisted {
            version: metadata.version,
            user_id: metadata.user_id.clone(),
            client_session: ClientSessionMetadataPersisted {
                homeserver: homeserver.clone(),
                db_path: db_path.clone(),
            },
            credential_account: metadata.credential_account.clone(),
            binding_nonce: metadata.binding_nonce.clone(),
            binding_tag: metadata.binding_tag.clone(),
            encryption_nonce: metadata.encryption_nonce.clone(),
            encrypted_secrets: metadata.encrypted_secrets.clone(),
            sliding_sync_version: metadata.sliding_sync_version,
        };
        let expected_tag = compute_binding_tag(&canonical_metadata, &credential_key.master_key)?;
        if !binding_tags_equal(&expected_tag, &metadata.binding_tag) {
            bail!("Matrix session metadata authentication failed");
        }
        Ok((homeserver, db_path, canonical_metadata))
    })();
    let (homeserver, db_path, canonical_metadata) = match validated {
        Ok(validated) => validated,
        Err(error) => return Err(error),
    };

    let mut encryption_nonce = decode_hex(&canonical_metadata.encryption_nonce)?;
    if encryption_nonce.len() != ENCRYPTION_NONCE_LEN {
        encryption_nonce.zeroize();
        bail!("Matrix session encryption nonce has an invalid length");
    }
    if canonical_metadata.encrypted_secrets.len() > 8 * 1024 * 1024 {
        encryption_nonce.zeroize();
        bail!("encrypted Matrix session payload exceeds the safety bound");
    }
    let mut ciphertext = decode_hex(&canonical_metadata.encrypted_secrets)?;
    let binding_claims = encode_binding_claims(&canonical_metadata)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&credential_key.master_key)
        .map_err(|_| anyhow!("failed to initialize Matrix session decryption"))?;
    let decrypted = cipher
        .decrypt(
            Nonce::from_slice(&encryption_nonce),
            Payload {
                msg: &ciphertext,
                aad: &binding_claims,
            },
        )
        .map_err(|_| anyhow!("Matrix session envelope authentication failed"));
    encryption_nonce.zeroize();
    ciphertext.zeroize();
    let mut plaintext = decrypted?;
    let parsed_secrets = serde_json::from_slice::<SessionSecretsPersisted>(&plaintext)
        .context("failed to decode decrypted Matrix session credentials");
    plaintext.zeroize();
    let secrets = parsed_secrets?;
    if secrets.version != SESSION_SECRET_PAYLOAD_VERSION {
        bail!(
            "unsupported encrypted Matrix session version {}; expected {}",
            secrets.version,
            SESSION_SECRET_PAYLOAD_VERSION
        );
    }
    if secrets.user_session.meta.user_id != expected_user_id {
        bail!("persisted Matrix session user does not match the requested user");
    }

    Ok(secrets.into_material(homeserver, db_path, metadata.sliding_sync_version))
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
    if parsed.scheme() != "https" {
        bail!("Matrix homeserver metadata must use https");
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
        let version = serde_json::from_slice::<VersionOnly>(bytes)
            .context("failed to decode Matrix session metadata version")?
            .version;
        return match version {
            SESSION_METADATA_VERSION => serde_json::from_slice::<SessionMetadataPersisted>(bytes)
                .map(DecodedSessionFile::Secure)
                .context("failed to decode secure Matrix session metadata"),
            LEGACY_SECURE_METADATA_VERSION => {
                serde_json::from_slice::<LegacySecureMetadataV2>(bytes)
                    .map(DecodedSessionFile::LegacySecure)
                    .context("failed to decode legacy secure Matrix session metadata")
            }
            _ => bail!("unsupported Matrix session metadata version {version}"),
        };
    }

    let legacy = serde_json::from_slice::<LegacyFullSessionPersisted>(bytes)
        .context("failed to decode legacy Matrix session for secure migration")?;
    Ok(DecodedSessionFile::LegacyPlaintext(SessionMaterial {
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

fn decode_session_file_and_zeroize(bytes: &mut [u8]) -> Result<DecodedSessionFile> {
    let decoded = decode_session_file(bytes);
    bytes.zeroize();
    decoded
}

fn session_file_is_legacy_or_malformed(bytes: &[u8]) -> bool {
    serde_json::from_slice::<BTreeMap<String, IgnoredAny>>(bytes)
        .map(|top_level| !top_level.contains_key("version"))
        .unwrap_or(true)
}

async fn clear_session_material_with_store(
    session_path: &Path,
    latest_user_id_path: &Path,
    user_id: &UserId,
    store: Arc<dyn KeyringStore>,
) -> Result<()> {
    let mut failures = Vec::new();
    let mut credential_cleanup_succeeded = true;
    match credential_account_authorized_for_cleanup(session_path, user_id, store.clone()).await {
        Ok(CredentialCleanupAuthorization::Owned(Some(account))) => {
            if let Err(error) = keyring_delete(store, account).await {
                failures.push(format!("OS credential deletion failed: {error:#}"));
                credential_cleanup_succeeded = false;
            }
        }
        Ok(CredentialCleanupAuthorization::Owned(None)) => {}
        Ok(CredentialCleanupAuthorization::DifferentUser) => {
            failures.push("session path belongs to a different Matrix user".to_string());
            credential_cleanup_succeeded = false;
        }
        Err(error) => {
            failures.push(format!(
                "credential deletion was refused because session binding was not authentic: {error:#}"
            ));
            credential_cleanup_succeeded = false;
        }
    }
    if credential_cleanup_succeeded {
        if let Err(error) = remove_file_if_exists(session_path).await {
            failures.push(format!("session metadata deletion failed: {error:#}"));
        }
    }
    // The pointer is navigation state, never identity authority. Remove it
    // regardless of whether it is absent, corrupt, or names a different user;
    // credentials above are always selected from `user_id` plus an
    // authenticated per-user metadata binding.
    if let Err(error) = remove_file_if_exists(latest_user_id_path).await {
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

async fn retire_session_material_with_store(
    session_path: &Path,
    user_id: &UserId,
    store: Arc<dyn KeyringStore>,
) -> Result<()> {
    match credential_account_authorized_for_cleanup(session_path, user_id, store.clone()).await? {
        CredentialCleanupAuthorization::Owned(Some(account)) => {
            keyring_delete(store, account)
                .await
                .context("failed to delete retired legacy-location Matrix credential")?;
        }
        CredentialCleanupAuthorization::Owned(None) => {}
        CredentialCleanupAuthorization::DifferentUser => return Ok(()),
    }
    remove_file_if_exists(session_path)
        .await
        .context("failed to remove retired legacy-location Matrix metadata")
}

async fn credential_account_authorized_for_cleanup(
    session_path: &Path,
    authoritative_user_id: &UserId,
    store: Arc<dyn KeyringStore>,
) -> Result<CredentialCleanupAuthorization> {
    let mut bytes = match tokio::fs::read(session_path).await {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(CredentialCleanupAuthorization::Owned(None));
        }
        Err(error) => {
            return Err(error).with_context(|| {
                format!(
                    "failed to read Matrix metadata at {}",
                    session_path.display()
                )
            });
        }
    };
    let decoded = decode_session_file_and_zeroize(&mut bytes);
    match decoded? {
        DecodedSessionFile::Secure(metadata) => {
            if metadata.user_id != authoritative_user_id {
                return Ok(CredentialCleanupAuthorization::DifferentUser);
            }
            let account = metadata.credential_account.clone();
            let material = load_secure_session(metadata, authoritative_user_id, store).await?;
            drop(material);
            Ok(CredentialCleanupAuthorization::Owned(Some(account)))
        }
        DecodedSessionFile::LegacySecure(metadata) => {
            let expected_account = legacy_credential_account(authoritative_user_id);
            if metadata.version != LEGACY_SECURE_METADATA_VERSION
                || metadata.credential_account != expected_account
            {
                return Ok(CredentialCleanupAuthorization::DifferentUser);
            }
            Ok(CredentialCleanupAuthorization::Owned(Some(
                expected_account,
            )))
        }
        DecodedSessionFile::LegacyPlaintext(material) => {
            if material.user_session.meta.user_id != authoritative_user_id {
                return Ok(CredentialCleanupAuthorization::DifferentUser);
            }
            Ok(CredentialCleanupAuthorization::Owned(None))
        }
    }
}

pub(crate) async fn write_latest_user_id(path: &Path, user_id: &UserId) -> Result<()> {
    let _guard = MATRIX_SESSION_STORE_LOCK.lock().await;
    write_private_file(path.to_path_buf(), user_id.as_bytes().to_vec()).await
}
