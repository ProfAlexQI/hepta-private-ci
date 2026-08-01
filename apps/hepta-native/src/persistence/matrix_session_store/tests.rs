use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use anyhow::Result;
use codex_keyring_store::{CredentialStoreError, KeyringStore, tests::MockKeyringStore};
use matrix_sdk::{
    SessionMeta, SessionTokens,
    authentication::matrix::MatrixSession,
    ruma::{owned_device_id, owned_user_id},
};

use super::{
    ClientSessionPersisted, SESSION_METADATA_VERSION, SessionMaterial, SessionMetadataPersisted,
    SlidingSyncVersion, clear_session_material_with_store,
    credential::{MATRIX_CREDENTIAL_SERVICE, legacy_credential_account},
    decode_session_file_and_zeroize, load_session_material_with_store,
    persist_secure_session_with_forced_write_failure, persist_secure_session_with_store,
    retire_session_material_with_store,
    private_file::write_private_file_atomically,
};

#[derive(Debug, Default)]
struct RecordingKeyringStore {
    inner: MockKeyringStore,
    saved_accounts: Mutex<Vec<String>>,
    deleted_accounts: Mutex<Vec<String>>,
}

impl RecordingKeyringStore {
    fn saved_accounts(&self) -> Vec<String> {
        self.saved_accounts.lock().unwrap().clone()
    }

    fn deleted_accounts(&self) -> Vec<String> {
        self.deleted_accounts.lock().unwrap().clone()
    }
}

impl KeyringStore for RecordingKeyringStore {
    fn load(
        &self,
        service: &str,
        account: &str,
    ) -> std::result::Result<Option<String>, CredentialStoreError> {
        self.inner.load(service, account)
    }

    fn save(
        &self,
        service: &str,
        account: &str,
        value: &str,
    ) -> std::result::Result<(), CredentialStoreError> {
        self.saved_accounts
            .lock()
            .unwrap()
            .push(account.to_string());
        self.inner.save(service, account, value)
    }

    fn delete(
        &self,
        service: &str,
        account: &str,
    ) -> std::result::Result<bool, CredentialStoreError> {
        self.deleted_accounts
            .lock()
            .unwrap()
            .push(account.to_string());
        self.inner.delete(service, account)
    }
}

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

fn stored_metadata(session_path: &Path) -> Result<SessionMetadataPersisted> {
    Ok(serde_json::from_slice(&fs::read(session_path)?)?)
}

fn write_legacy_session(
    session_path: &Path,
    material: &SessionMaterial,
    homeserver: &str,
) -> Result<()> {
    fs::create_dir_all(session_path.parent().expect("session parent"))?;
    let legacy = serde_json::json!({
        "client_session": {
            "homeserver": homeserver,
            "db_path": material.client_session.db_path.clone(),
            "passphrase": material.client_session.passphrase.clone(),
        },
        "user_session": material.user_session.clone(),
        "sync_token": material.sync_token.clone(),
        "sliding_sync_version": material.sliding_sync_version,
    });
    fs::write(session_path, serde_json::to_vec(&legacy)?)?;
    Ok(())
}

#[test]
fn private_file_atomic_update_replaces_existing_contents() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let path = temp.path().join("session-metadata.json");
    write_private_file_atomically(&path, b"first")?;
    write_private_file_atomically(&path, b"second")?;
    assert_eq!(fs::read(path)?, b"second");
    Ok(())
}

#[tokio::test]
async fn metadata_write_failure_removes_uncommitted_generation() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let session_path = temp.path().join("persistent_state").join("session");
    let keyring = Arc::new(RecordingKeyringStore::default());
    let mut material = sample_material();

    let error = persist_secure_session_with_forced_write_failure(
        &session_path,
        &mut material,
        keyring.clone(),
    )
    .await
    .err()
    .expect("injected metadata write must fail after key save");
    assert!(
        format!("{error:#}").contains("secure Matrix session metadata"),
        "unexpected error chain: {error:#}"
    );
    let saved = keyring.saved_accounts();
    assert_eq!(saved.len(), 1);
    assert_eq!(keyring.deleted_accounts(), saved);
    assert_eq!(
        keyring.inner.load(MATRIX_CREDENTIAL_SERVICE, &saved[0])?,
        None
    );
    Ok(())
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
    assert_eq!(parsed.user_id, material.user_session.meta.user_id);
    assert!(!parsed.binding_tag.is_empty());
    assert_eq!(
        parsed.client_session.db_path,
        PathBuf::from("db_2026_07_21")
    );

    let account = parsed.credential_account;
    let stored = keyring
        .saved_value_for_service(MATRIX_CREDENTIAL_SERVICE, &account)
        .expect("credential payload should exist");
    assert!(stored.len() < 512);
    assert!(!stored.contains("matrix-access-secret"));
    assert!(!stored.contains("matrix-refresh-secret"));
    assert!(!stored.contains("database-secret-passphrase"));

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
    let material = sample_material();
    write_legacy_session(
        &session_path,
        &material,
        &material.client_session.homeserver,
    )?;

    let keyring = Arc::new(MockKeyringStore::default());
    let restored = load_session_material_with_store(
        &session_path,
        &material.user_session.meta.user_id,
        keyring,
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
async fn invalid_legacy_migration_fails_without_returning_plaintext_session() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let (session_path, _) = session_paths(&temp);
    let material = sample_material();
    write_legacy_session(
        &session_path,
        &material,
        "https://alice:legacy-secret@matrix.example.org/",
    )?;
    let keyring = Arc::new(MockKeyringStore::default());

    let error = load_session_material_with_store(
        &session_path,
        &material.user_session.meta.user_id,
        keyring.clone(),
    )
    .await
    .err()
    .expect("invalid legacy metadata must not return session material");
    assert!(
        format!("{error:#}").contains("credentials or URL secrets"),
        "unexpected error chain: {error:#}"
    );
    for secret in [
        "legacy-secret",
        "database-secret-passphrase",
        "matrix-access-secret",
        "matrix-refresh-secret",
        "matrix-sync-secret",
    ] {
        assert!(!error.to_string().contains(secret));
    }
    assert!(!session_path.exists());
    Ok(())
}

#[tokio::test]
async fn missing_keyring_entry_fails_closed_without_plaintext_fallback() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let (session_path, _) = session_paths(&temp);
    let keyring = Arc::new(MockKeyringStore::default());
    let mut material = sample_material();
    persist_secure_session_with_store(&session_path, &mut material, keyring.clone()).await?;
    let account = stored_metadata(&session_path)?.credential_account;
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
    let account = stored_metadata(&session_path)?.credential_account;

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
    let keyring = Arc::new(RecordingKeyringStore::default());
    let mut material = sample_material();
    material.client_session.homeserver =
        "https://alice:homeserver-secret@matrix.example.org/".to_string();
    let error = persist_secure_session_with_store(&session_path, &mut material, keyring.clone())
        .await
        .err()
        .expect("credential-bearing homeserver URL must be rejected");
    assert!(error.to_string().contains("credentials or URL secrets"));
    assert!(!session_path.exists());
    assert!(keyring.saved_accounts().is_empty());
    Ok(())
}

#[tokio::test]
async fn non_https_homeserver_is_rejected_before_keyring_mutation() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let (session_path, _) = session_paths(&temp);
    let keyring = Arc::new(RecordingKeyringStore::default());
    let mut material = sample_material();
    material.client_session.homeserver = "http://matrix.example.org".to_string();

    let error = persist_secure_session_with_store(&session_path, &mut material, keyring.clone())
        .await
        .err()
        .expect("plaintext HTTP homeserver must be rejected");
    assert!(error.to_string().contains("must use https"));
    assert!(keyring.saved_accounts().is_empty());
    assert!(!session_path.exists());
    Ok(())
}

#[tokio::test]
async fn metadata_homeserver_and_db_tampering_fail_authentication() -> Result<()> {
    for tamper_db in [false, true] {
        let temp = tempfile::tempdir()?;
        let (session_path, _) = session_paths(&temp);
        let keyring = Arc::new(MockKeyringStore::default());
        let mut material = sample_material();
        persist_secure_session_with_store(&session_path, &mut material, keyring.clone()).await?;

        let mut metadata = stored_metadata(&session_path)?;
        if tamper_db {
            metadata.client_session.db_path = PathBuf::from("db_attacker");
        } else {
            metadata.client_session.homeserver = "https://evil.example.org/".to_string();
        }
        write_private_file_atomically(&session_path, &serde_json::to_vec(&metadata)?)?;

        let error = load_session_material_with_store(
            &session_path,
            &material.user_session.meta.user_id,
            keyring,
        )
        .await
        .err()
        .expect("tampered metadata must fail closed");
        assert!(error.to_string().contains("authentication failed"));
    }
    Ok(())
}

#[tokio::test]
async fn ciphertext_tampering_fails_authentication() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let (session_path, _) = session_paths(&temp);
    let keyring = Arc::new(MockKeyringStore::default());
    let mut material = sample_material();
    persist_secure_session_with_store(&session_path, &mut material, keyring.clone()).await?;
    let mut metadata = stored_metadata(&session_path)?;
    let replacement = if metadata.encrypted_secrets.starts_with('0') {
        "1"
    } else {
        "0"
    };
    metadata.encrypted_secrets.replace_range(0..1, replacement);
    write_private_file_atomically(&session_path, &serde_json::to_vec(&metadata)?)?;

    let error = load_session_material_with_store(
        &session_path,
        &material.user_session.meta.user_id,
        keyring,
    )
    .await
    .err()
    .expect("tampered ciphertext must fail closed");
    assert!(error.to_string().contains("authentication failed"));
    Ok(())
}

#[tokio::test]
async fn keyring_generation_mismatch_cannot_bind_to_committed_metadata() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let (session_path, _) = session_paths(&temp);
    let keyring = Arc::new(MockKeyringStore::default());
    let mut material = sample_material();
    persist_secure_session_with_store(&session_path, &mut material, keyring.clone()).await?;
    let first = stored_metadata(&session_path)?;
    let first_key = keyring
        .saved_value_for_service(MATRIX_CREDENTIAL_SERVICE, &first.credential_account)
        .expect("first key");

    material.user_session.tokens.access_token = "new-access-token".to_string();
    persist_secure_session_with_store(&session_path, &mut material, keyring.clone()).await?;
    let second = stored_metadata(&session_path)?;
    assert_ne!(first.credential_account, second.credential_account);
    keyring.save(
        MATRIX_CREDENTIAL_SERVICE,
        &second.credential_account,
        &first_key,
    )?;

    let error = load_session_material_with_store(
        &session_path,
        &material.user_session.meta.user_id,
        keyring,
    )
    .await
    .err()
    .expect("wrong-generation key must fail closed");
    assert!(
        error.to_string().contains("generations do not match")
            || error.to_string().contains("authentication failed")
    );
    Ok(())
}

#[tokio::test]
async fn successful_update_uses_fresh_generation_and_retires_old_key() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let (session_path, _) = session_paths(&temp);
    let keyring = Arc::new(MockKeyringStore::default());
    let mut material = sample_material();
    persist_secure_session_with_store(&session_path, &mut material, keyring.clone()).await?;
    let first_account = stored_metadata(&session_path)?.credential_account;

    persist_secure_session_with_store(&session_path, &mut material, keyring.clone()).await?;
    let second_account = stored_metadata(&session_path)?.credential_account;

    assert_ne!(first_account, second_account);
    assert_eq!(
        keyring.load(MATRIX_CREDENTIAL_SERVICE, &first_account)?,
        None
    );
    assert!(
        keyring
            .load(MATRIX_CREDENTIAL_SERVICE, &second_account)?
            .is_some()
    );
    Ok(())
}

#[tokio::test]
async fn very_long_tokens_do_not_expand_os_credential_payload() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let (session_path, _) = session_paths(&temp);
    let keyring = Arc::new(MockKeyringStore::default());
    let mut material = sample_material();
    material.user_session.tokens.access_token = "a".repeat(16_384);
    material.user_session.tokens.refresh_token = Some("r".repeat(16_384));
    persist_secure_session_with_store(&session_path, &mut material, keyring.clone()).await?;
    let metadata = stored_metadata(&session_path)?;
    let key_payload = keyring
        .saved_value_for_service(MATRIX_CREDENTIAL_SERVICE, &metadata.credential_account)
        .expect("fixed key payload");
    assert!(key_payload.len() < 512);
    assert!(!key_payload.contains(&"a".repeat(128)));
    assert!(!fs::read_to_string(&session_path)?.contains(&"a".repeat(128)));

    let restored = load_session_material_with_store(
        &session_path,
        &material.user_session.meta.user_id,
        keyring,
    )
    .await?;
    assert_eq!(restored.user_session.tokens.access_token.len(), 16_384);
    Ok(())
}

async fn cleanup_with_untrusted_pointer(pointer: Option<&[u8]>) -> Result<()> {
    let temp = tempfile::tempdir()?;
    let (alice_path, latest_path) = session_paths(&temp);
    let mallory_path = temp
        .path()
        .join("mallory")
        .join("persistent_state")
        .join("session");
    let keyring = Arc::new(MockKeyringStore::default());
    let mut alice = sample_material();
    let mut mallory = sample_material();
    mallory.user_session.meta.user_id = owned_user_id!("@mallory:example.org");
    mallory.user_session.tokens.access_token = "mallory-access".to_string();
    persist_secure_session_with_store(&alice_path, &mut alice, keyring.clone()).await?;
    persist_secure_session_with_store(&mallory_path, &mut mallory, keyring.clone()).await?;
    let alice_account = stored_metadata(&alice_path)?.credential_account;
    let mallory_account = stored_metadata(&mallory_path)?.credential_account;
    if let Some(pointer) = pointer {
        write_private_file_atomically(&latest_path, pointer)?;
    }

    clear_session_material_with_store(
        &alice_path,
        &latest_path,
        &alice.user_session.meta.user_id,
        keyring.clone(),
    )
    .await?;

    assert!(!alice_path.exists());
    assert!(!latest_path.exists());
    assert_eq!(
        keyring.load(MATRIX_CREDENTIAL_SERVICE, &alice_account)?,
        None
    );
    assert!(
        keyring
            .load(MATRIX_CREDENTIAL_SERVICE, &mallory_account)?
            .is_some()
    );
    assert!(mallory_path.exists());
    Ok(())
}

#[tokio::test]
async fn cleanup_uses_authoritative_user_when_pointer_is_missing() -> Result<()> {
    cleanup_with_untrusted_pointer(None).await
}

#[tokio::test]
async fn cleanup_uses_authoritative_user_when_pointer_is_corrupt() -> Result<()> {
    cleanup_with_untrusted_pointer(Some(b"not-a-matrix-user")).await
}

#[tokio::test]
async fn cleanup_never_deletes_wrong_users_credential_from_pointer() -> Result<()> {
    cleanup_with_untrusted_pointer(Some(b"@mallory:example.org")).await
}

#[tokio::test]
async fn tampered_metadata_never_authorizes_credential_deletion() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let (session_path, latest_path) = session_paths(&temp);
    let keyring = Arc::new(MockKeyringStore::default());
    let mut material = sample_material();
    persist_secure_session_with_store(&session_path, &mut material, keyring.clone()).await?;
    let mut metadata = stored_metadata(&session_path)?;
    let authentic_account = metadata.credential_account.clone();
    metadata.client_session.db_path = PathBuf::from("db_tampered");
    write_private_file_atomically(&session_path, &serde_json::to_vec(&metadata)?)?;

    let error = clear_session_material_with_store(
        &session_path,
        &latest_path,
        &material.user_session.meta.user_id,
        keyring.clone(),
    )
    .await
    .err()
    .expect("tampered binding must make cleanup report incomplete");
    assert!(error.to_string().contains("was not authentic"));
    assert!(
        keyring
            .load(MATRIX_CREDENTIAL_SERVICE, &authentic_account)?
            .is_some()
    );
    // Keep the encrypted envelope/account reference so secure cleanup can be
    // retried; the untrusted latest-user pointer is still removed.
    assert!(session_path.exists());
    assert!(!latest_path.exists());
    Ok(())
}

#[tokio::test]
async fn canonical_commit_then_legacy_retirement_removes_duplicate_generation() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let legacy_path = temp
        .path()
        .join("legacy")
        .join("persistent_state")
        .join("session");
    let canonical_path = temp
        .path()
        .join("canonical")
        .join("persistent_state")
        .join("session");
    let keyring = Arc::new(MockKeyringStore::default());
    let mut material = sample_material();
    persist_secure_session_with_store(&legacy_path, &mut material, keyring.clone()).await?;
    let legacy_account = stored_metadata(&legacy_path)?.credential_account;
    persist_secure_session_with_store(&canonical_path, &mut material, keyring.clone()).await?;
    let canonical_account = stored_metadata(&canonical_path)?.credential_account;

    retire_session_material_with_store(
        &legacy_path,
        &material.user_session.meta.user_id,
        keyring.clone(),
    )
    .await?;

    assert!(!legacy_path.exists());
    assert_eq!(
        keyring.load(MATRIX_CREDENTIAL_SERVICE, &legacy_account)?,
        None
    );
    assert!(
        keyring
            .load(MATRIX_CREDENTIAL_SERVICE, &canonical_account)?
            .is_some()
    );
    load_session_material_with_store(
        &canonical_path,
        &material.user_session.meta.user_id,
        keyring,
    )
    .await?;
    Ok(())
}

#[tokio::test]
async fn logout_style_cleanup_removes_canonical_and_legacy_generations() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let canonical_path = temp
        .path()
        .join("canonical")
        .join("persistent_state")
        .join("session");
    let legacy_path = temp
        .path()
        .join("legacy")
        .join("persistent_state")
        .join("session");
    let latest_path = temp.path().join("latest_user_id.txt");
    let keyring = Arc::new(MockKeyringStore::default());
    let mut material = sample_material();
    persist_secure_session_with_store(&canonical_path, &mut material, keyring.clone()).await?;
    persist_secure_session_with_store(&legacy_path, &mut material, keyring.clone()).await?;
    let canonical_account = stored_metadata(&canonical_path)?.credential_account;
    let legacy_account = stored_metadata(&legacy_path)?.credential_account;
    write_private_file_atomically(&latest_path, b"corrupt-pointer")?;

    clear_session_material_with_store(
        &canonical_path,
        &latest_path,
        &material.user_session.meta.user_id,
        keyring.clone(),
    )
    .await?;
    retire_session_material_with_store(
        &legacy_path,
        &material.user_session.meta.user_id,
        keyring.clone(),
    )
    .await?;

    assert!(!canonical_path.exists());
    assert!(!legacy_path.exists());
    assert!(!latest_path.exists());
    assert_eq!(
        keyring.load(MATRIX_CREDENTIAL_SERVICE, &canonical_account)?,
        None
    );
    assert_eq!(
        keyring.load(MATRIX_CREDENTIAL_SERVICE, &legacy_account)?,
        None
    );
    Ok(())
}

#[tokio::test]
async fn colliding_legacy_plaintext_for_different_user_is_preserved() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let legacy_path = temp
        .path()
        .join("collision")
        .join("persistent_state")
        .join("session");
    let mut other = sample_material();
    other.user_session.meta.user_id = owned_user_id!("@mallory:example.org");
    write_legacy_session(&legacy_path, &other, &other.client_session.homeserver)?;
    let keyring = Arc::new(MockKeyringStore::default());

    retire_session_material_with_store(
        &legacy_path,
        &owned_user_id!("@alice:example.org"),
        keyring,
    )
    .await?;

    assert!(legacy_path.exists());
    assert!(fs::read_to_string(legacy_path)?.contains("mallory"));
    Ok(())
}

#[tokio::test]
async fn v2_same_user_key_is_retired_only_after_v3_commit() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let (session_path, _) = session_paths(&temp);
    let keyring = Arc::new(MockKeyringStore::default());
    let mut material = sample_material();
    let old_account = legacy_credential_account(&material.user_session.meta.user_id);
    keyring.save(MATRIX_CREDENTIAL_SERVICE, &old_account, "old-v2-secret")?;
    let v2 = serde_json::json!({
        "version": 2,
        "client_session": {
            "homeserver": material.client_session.homeserver.clone(),
            "db_path": material.client_session.db_path.clone(),
        },
        "credential_account": old_account.clone(),
        "sliding_sync_version": material.sliding_sync_version,
    });
    write_private_file_atomically(&session_path, &serde_json::to_vec(&v2)?)?;

    persist_secure_session_with_store(&session_path, &mut material, keyring.clone()).await?;

    assert_eq!(keyring.load(MATRIX_CREDENTIAL_SERVICE, &old_account)?, None);
    assert_eq!(
        stored_metadata(&session_path)?.version,
        SESSION_METADATA_VERSION
    );
    Ok(())
}

#[test]
fn legacy_plaintext_scan_buffer_is_zeroized_on_success() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let (session_path, _) = session_paths(&temp);
    let material = sample_material();
    write_legacy_session(
        &session_path,
        &material,
        &material.client_session.homeserver,
    )?;
    let mut bytes = fs::read(&session_path)?;
    let decoded = decode_session_file_and_zeroize(&mut bytes)?;
    drop(decoded);
    assert!(bytes.iter().all(|byte| *byte == 0));
    Ok(())
}
