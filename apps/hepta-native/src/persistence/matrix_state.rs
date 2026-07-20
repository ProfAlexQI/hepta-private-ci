//! Handles app persistence by saving and restoring client session data to/from the filesystem.

use anyhow::{anyhow, bail};
use makepad_widgets::{log, Cx};
use matrix_sdk::{
    ruma::{OwnedUserId, UserId},
    Client,
};
use std::path::PathBuf;

use crate::{app_data_dir, login::login_screen::LoginAction};
use super::matrix_session_store::SessionMaterial;
use super::matrix_session_store::clear_session_material;
use super::matrix_session_store::load_session_material;
use super::matrix_session_store::referenced_db_path_from_slice;
use super::matrix_session_store::save_session_material;
use super::matrix_session_store::wipe_client_passphrase;
use super::matrix_session_store::wipe_session_tokens;
use super::matrix_session_store::wipe_sync_token;
use super::matrix_session_store::write_latest_user_id as write_latest_user_id_private;
pub use super::matrix_session_store::ClientSessionPersisted;
pub use super::matrix_session_store::SlidingSyncVersion;

fn user_id_to_file_name(user_id: &UserId) -> String {
    user_id.as_str().replace(":", "_").replace("@", "")
}

/// Returns the path to the persistent state directory for the given user.
pub fn persistent_state_dir(user_id: &UserId) -> PathBuf {
    app_data_dir()
        .join(user_id_to_file_name(user_id))
        .join("persistent_state")
}

/// Returns the path to the session file for the given user.
pub fn session_file_path(user_id: &UserId) -> PathBuf {
    persistent_state_dir(user_id).join("session")
}

const LATEST_USER_ID_FILE_NAME: &str = "latest_user_id.txt";

/// Returns the user ID of the most recently-logged in user session.
pub async fn most_recent_user_id() -> Option<OwnedUserId> {
    tokio::fs::read_to_string(app_data_dir().join(LATEST_USER_ID_FILE_NAME))
        .await
        .ok()?
        .trim()
        .try_into()
        .ok()
}

/// Resolves the path that `restore_session()` would actually open.
fn resolve_db_path(stored: PathBuf) -> PathBuf {
    if !stored.is_absolute() {
        return app_data_dir().join(stored);
    }
    if stored.exists() {
        return stored;
    }
    let Some(name) = stored.file_name() else {
        return stored;
    };
    // iOS sandbox UUID changes across reinstalls; the absolute path
    // baked into the session is now stale. Use the basename instead.
    app_data_dir().join(name)
}

/// Returns the set of `db` paths referenced by any saved session file.
///
/// This basically scans every saved user session dir, not just the most recent one,
/// to help ensure that db dirs don't get orphaned on the filesystem forever.
async fn collect_referenced_db_paths() -> std::collections::HashSet<PathBuf> {
    use std::collections::HashSet;
    let mut paths = HashSet::new();
    let data_dir = app_data_dir();

    let Ok(mut entries) = tokio::fs::read_dir(data_dir).await else {
        return paths;
    };

    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if name.starts_with("db_") {
            continue;
        }
        let session_file = path.join("persistent_state").join("session");
        let Ok(bytes) = tokio::fs::read(&session_file).await else {
            continue;
        };
        let db_path = match referenced_db_path_from_slice(&bytes) {
            Ok(db_path) => db_path,
            Err(e) => {
                log!(
                    "collect_referenced_db_paths: skipping unparsable session file {}: {e}",
                    session_file.display(),
                );
                continue;
            }
        };
        paths.insert(resolve_db_path(db_path));
    }

    paths
}

/// Deletes `db_*` subdirs not referenced by any saved session. Only touches
/// entries that match the `db_*` prefix and that came from
/// `read_dir(app_data_dir())`, so it can't escape the data dir even with a
/// malicious session file.
pub async fn cleanup_orphan_db_dirs() {
    let data_dir = app_data_dir();
    let active = collect_referenced_db_paths().await;

    let mut entries = match tokio::fs::read_dir(data_dir).await {
        Ok(e) => e,
        Err(e) => {
            log!(
                "cleanup_orphan_db_dirs: could not read data dir {}: {e}",
                data_dir.display()
            );
            return;
        }
    };

    let mut deleted = 0usize;
    let mut bytes_freed = 0u64;
    let mut kept = 0usize;
    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if !name.starts_with("db_") {
            continue;
        }
        if active.contains(&path) {
            kept += 1;
            log!(
                "cleanup_orphan_db_dirs: preserving referenced db dir: {}",
                path.display()
            );
            continue;
        }
        let size = dir_size_bytes(&path).await.unwrap_or(0);
        match tokio::fs::remove_dir_all(&path).await {
            Ok(()) => {
                deleted += 1;
                bytes_freed += size;
                log!(
                    "cleanup_orphan_db_dirs: deleted orphaned db dir ({} bytes): {}",
                    size,
                    path.display(),
                );
            }
            Err(e) => {
                log!(
                    "cleanup_orphan_db_dirs: failed to delete {}: {e}",
                    path.display(),
                );
            }
        }
    }

    if deleted > 0 || kept > 0 {
        log!(
            "cleanup_orphan_db_dirs: deleted {deleted} orphan(s), freed {bytes_freed} bytes; kept {kept} active referenced",
        );
    }
}

/// Recursive size sum, best-effort. Just for the cleanup log line.
async fn dir_size_bytes(path: &std::path::Path) -> Option<u64> {
    let mut total = 0u64;
    let mut entries = tokio::fs::read_dir(path).await.ok()?;
    while let Ok(Some(entry)) = entries.next_entry().await {
        let Ok(md) = entry.metadata().await else {
            continue;
        };
        if md.is_file() {
            total = total.saturating_add(md.len());
        } else if md.is_dir() {
            // matrix-sdk-sqlite doesn't nest subdirectories, but be safe.
            if let Some(sub) = Box::pin(dir_size_bytes(&entry.path())).await {
                total = total.saturating_add(sub);
            }
        }
    }
    Some(total)
}

/// Save which user was the most recently logged in.
async fn save_latest_user_id(user_id: &UserId) -> anyhow::Result<()> {
    write_latest_user_id_private(&app_data_dir().join(LATEST_USER_ID_FILE_NAME), user_id).await
}

/// Restores the given user's previous session from the filesystem.
///
/// If no User ID is specified, the ID of the most recently-logged in user
/// is retrieved from the filesystem.
pub async fn restore_session(
    user_id: Option<OwnedUserId>,
) -> anyhow::Result<(Client, Option<String>)> {
    let user_id = if let Some(user_id) = user_id {
        Some(user_id)
    } else {
        most_recent_user_id().await
    };

    let Some(user_id) = user_id else {
        log!("Could not find previous latest User ID");
        bail!("Could not find previous latest User ID");
    };
    let session_file = session_file_path(&user_id);
    if !session_file.exists() {
        log!("Could not find previous session file for user {user_id}");
        bail!("Could not find previous session file");
    }
    let status_str = format!("Loading previous session file for {user_id}...");
    log!("{status_str}: '{}'", session_file.display());
    Cx::post_action(LoginAction::Status {
        title: "Restoring session".into(),
        status: status_str,
    });

    let SessionMaterial {
        mut client_session,
        user_session,
        sync_token,
        sliding_sync_version,
    } = load_session_material(&session_file, &user_id).await?;

    let status_str = format!(
        "Loaded session file for:\n{user_id}\n\nTrying to connect to homeserver...\n{}",
        client_session.homeserver,
    );
    log!("{status_str}");
    Cx::post_action(LoginAction::Status {
        title: "Connecting to homeserver".into(),
        status: status_str,
    });
    let original_stored = client_session.db_path.clone();
    let db_path = resolve_db_path(client_session.db_path.clone());
    if db_path != original_stored {
        log!(
            "Stored db_path '{}' relocated to '{}'",
            original_stored.display(),
            db_path.display(),
        );
    }
    log!(
        "Restoring session for {user_id} with db at: {} (stored as: {})",
        db_path.display(),
        original_stored.display(),
    );
    let store_config =
        crate::sliding_sync::build_sqlite_store_config(&db_path, &client_session.passphrase);
    // Build the client with the previous settings from the session.
    let client_result = Client::builder()
        .homeserver_url(client_session.homeserver.clone())
        .sqlite_store_with_config_and_cache_path(store_config, None::<&std::path::Path>)
        .with_threading_support(matrix_sdk::ThreadingSupport::Enabled {
            with_subscriptions: true,
        })
        .handle_refresh_tokens()
        .build()
        .await;
    wipe_client_passphrase(&mut client_session);
    let client = client_result?;
    let sliding_sync_version = sliding_sync_version.into();
    client.set_sliding_sync_version(sliding_sync_version);
    let status_str = format!(
        "Authenticating previous login session for {}...",
        user_session.meta.user_id
    );
    log!("{status_str}");
    Cx::post_action(LoginAction::Status {
        title: "Authenticating session".into(),
        status: status_str,
    });

    // Restore the Matrix user session.
    client.restore_session(user_session).await?;
    save_latest_user_id(&user_id).await?;

    Ok((client, sync_token))
}

/// Persist a logged-in client session using private metadata plus the OS keyring.
pub async fn save_session(
    client: &Client,
    client_session: ClientSessionPersisted,
) -> anyhow::Result<()> {
    let user_session = client
        .matrix_auth()
        .session()
        .ok_or_else(|| anyhow!("A logged-in client should have a session"))?;

    let sliding_sync_version = client.sliding_sync_version().into();
    let session_file = session_file_path(&user_session.meta.user_id);
    let mut material = SessionMaterial {
        client_session,
        user_session,
        sync_token: None,
        sliding_sync_version,
    };
    let user_id = material.user_session.meta.user_id.clone();
    let save_result = save_session_material(
        &session_file,
        &app_data_dir().join(LATEST_USER_ID_FILE_NAME),
        &mut material,
    )
    .await;
    wipe_client_passphrase(&mut material.client_session);
    wipe_session_tokens(&mut material.user_session);
    wipe_sync_token(&mut material.sync_token);
    save_result?;

    log!(
        "Session persisted securely for {user_id} at: {}",
        session_file.display()
    );
    Ok(())
}

/// Remove the persisted Matrix credential, session metadata, and latest-user
/// pointer for a user after server-side logout has invalidated the session.
pub async fn clear_persisted_session(user_id: &UserId) -> anyhow::Result<()> {
    clear_session_material(
        &session_file_path(user_id),
        &app_data_dir().join(LATEST_USER_ID_FILE_NAME),
        user_id,
    )
    .await
}

/// Remove the LATEST_USER_ID_FILE_NAME file if it exists
///
/// Returns:
/// - Ok(true) if file was found and deleted
/// - Ok(false) if file didn't exist
/// - Err if deletion failed
pub async fn delete_latest_user_id() -> anyhow::Result<bool> {
    let last_login_path = app_data_dir().join(LATEST_USER_ID_FILE_NAME);

    if last_login_path.exists() {
        tokio::fs::remove_file(&last_login_path)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to remove latest user file: {e}"))
            .map(|_| true)
    } else {
        Ok(false)
    }
}
