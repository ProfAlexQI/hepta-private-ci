use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use codex_keyring_store::KeyringStore;
use codex_keyring_store::tests::MockKeyringStore;
use matrix_sdk::SessionMeta;
use matrix_sdk::SessionTokens;
use matrix_sdk::authentication::matrix::MatrixSession;
use matrix_sdk::ruma::owned_device_id;
use matrix_sdk::ruma::owned_user_id;

use super::ClientSessionPersisted;
use super::SESSION_METADATA_VERSION;
use super::SessionMaterial;
use super::SessionMetadataPersisted;
use super::SlidingSyncVersion;
use super::clear_session_material_with_store;
use super::credential::MATRIX_CREDENTIAL_SERVICE;
use super::credential::credential_account;
use super::load_session_material_with_store;
use super::persist_secure_session_with_store;
use super::private_file::write_private_file_atomically;

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
    let account = credential_account(&material.user_session.meta.user_id);

    let error = load_session_material_with_store(
        &session_path,
        &material.user_session.meta.user_id,
        keyring.clone(),
    )
    .await
    .err()
    .expect("invalid legacy metadata must not return session material");
    assert!(error.to_string().contains("credentials or URL secrets"));
    for secret in [
        "legacy-secret",
        "database-secret-passphrase",
        "matrix-access-secret",
        "matrix-refresh-secret",
        "matrix-sync-secret",
    ] {
        assert!(!error.to_string().contains(secret));
    }
    assert!(!keyring.contains_for_service(MATRIX_CREDENTIAL_SERVICE, &account));
    assert!(fs::read_to_string(&session_path)?.contains("matrix-access-secret"));
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

    let error = persist_secure_session_with_store(&session_path, &mut material, keyring.clone())
        .await
        .err()
        .expect("credential-bearing homeserver URL must be rejected");
    assert!(error.to_string().contains("credentials or URL secrets"));
    assert!(!session_path.exists());
    assert!(!keyring.contains_for_service(MATRIX_CREDENTIAL_SERVICE, &account));
    Ok(())
}
