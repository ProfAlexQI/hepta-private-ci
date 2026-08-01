//! Handles app persistence by saving and restoring client session data to/from the filesystem.

use std::path::PathBuf;

use anyhow::{Context as _, anyhow, bail};
use makepad_widgets::{Cx, log};
use matrix_sdk::{
    Client,
    ruma::{OwnedUserId, UserId},
};
use zeroize::Zeroize;

use crate::{app_data_dir, cache_dir, login::login_screen::LoginAction};
use super::matrix_session_store::{
    SessionMaterial, clear_session_material, load_session_material, referenced_db_path_from_slice,
    retire_session_material, save_session_material, wipe_client_passphrase, wipe_session_tokens,
    wipe_sync_token, write_latest_user_id as write_latest_user_id_private,
};
pub use super::matrix_session_store::{ClientSessionPersisted, SlidingSyncVersion};

const MATRIX_USER_STATE_ROOT: &str = "matrix-users-v3";

fn legacy_user_id_to_file_name(user_id: &UserId) -> String {
    user_id.as_str().replace(":", "_").replace("@", "")
}

fn encoded_user_state_relative_dir(raw_user_id: &str) -> PathBuf {
    PathBuf::from(MATRIX_USER_STATE_ROOT).join(format!(
        "user-{}",
        blake3::hash(raw_user_id.as_bytes()).to_hex()
    ))
}

pub(crate) fn legacy_persistent_state_dir(user_id: &UserId) -> PathBuf {
    app_data_dir()
        .join(legacy_user_id_to_file_name(user_id))
        .join("persistent_state")
}

/// Returns the path to the persistent state directory for the given user.
pub fn persistent_state_dir(user_id: &UserId) -> PathBuf {
    app_data_dir()
        .join(encoded_user_state_relative_dir(user_id.as_str()))
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
///
/// Returns `None` if the app data directory can't be accessed,
/// which means that nothing should be considered as eligible for deletion.
async fn collect_referenced_db_paths() -> Option<std::collections::HashSet<PathBuf>> {
    use std::collections::HashSet;
    let mut paths = HashSet::new();
    let data_dir = app_data_dir();

    // Search both the collision-free v3 hierarchy and the legacy one-level
    // layout. Never follow symlinks, and abort the *whole* prune decision on
    // any enumeration/read/authentication error.
    let mut pending_dirs = vec![data_dir.to_path_buf()];
    while let Some(directory) = pending_dirs.pop() {
        let mut entries = match tokio::fs::read_dir(&directory).await {
            Ok(entries) => entries,
            Err(error) => {
                log!(
                    "collect_referenced_db_paths: refusing to prune because {} could not be read: {error}",
                    directory.display()
                );
                return None;
            }
        };
        loop {
            let entry = match entries.next_entry().await {
                Ok(Some(entry)) => entry,
                Ok(None) => break,
                Err(error) => {
                    log!(
                        "collect_referenced_db_paths: refusing to prune because enumeration of {} failed: {error}",
                        directory.display()
                    );
                    return None;
                }
            };
            let path = entry.path();
            let file_type = match entry.file_type().await {
                Ok(file_type) => file_type,
                Err(error) => {
                    log!(
                        "collect_referenced_db_paths: refusing to prune because file type for {} failed: {error}",
                        path.display()
                    );
                    return None;
                }
            };
            if file_type.is_symlink() {
                continue;
            }
            if file_type.is_dir() {
                let is_database = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with("db_"));
                if !is_database || is_legacy_user_session_dir(&path).await? {
                    pending_dirs.push(path);
                }
                continue;
            }
            let is_session_file = path.file_name().is_some_and(|name| name == "session")
                && path
                    .parent()
                    .and_then(|parent| parent.file_name())
                    .is_some_and(|name| name == "persistent_state");
            if !is_session_file {
                continue;
            }
            let mut bytes = match tokio::fs::read(&path).await {
                Ok(bytes) => bytes,
                Err(error) => {
                    log!(
                        "collect_referenced_db_paths: refusing to prune because {} could not be read: {error}",
                        path.display(),
                    );
                    return None;
                }
            };
            let db_path = match referenced_db_path_from_slice(&mut bytes).await {
                Ok(db_path) => db_path,
                Err(error) => {
                    bytes.zeroize();
                    log!(
                        "collect_referenced_db_paths: refusing to prune because {} is not authenticated session metadata: {error}",
                        path.display(),
                    );
                    return None;
                }
            };
            debug_assert!(bytes.iter().all(|byte| *byte == 0));
            paths.insert(resolve_db_path(db_path));
        }
    }

    Some(paths)
}

async fn is_legacy_user_session_dir(path: &std::path::Path) -> Option<bool> {
    match tokio::fs::metadata(path.join("persistent_state").join("session")).await {
        Ok(metadata) => Some(metadata.is_file()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Some(false),
        Err(error) => {
            log!(
                "collect_referenced_db_paths: refusing to prune because legacy-session detection for {} failed: {error}",
                path.display()
            );
            None
        }
    }
}

/// Deletes old database files that start with `"db_"` within the given `dir` and its subdirectories.
///
/// Only deletes database files that are inactive, i.e., where `is_active` returns false.
async fn prune_orphan_db_dirs(dir: &std::path::Path, is_active: impl Fn(&std::ffi::OsStr) -> bool) {
    let mut entries = match tokio::fs::read_dir(dir).await {
        Ok(e) => e,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return,
        Err(e) => {
            log!(
                "prune_orphan_db_dirs: could not read {}: {e}",
                dir.display()
            );
            return;
        }
    };

    let mut deleted: usize = 0;
    let mut bytes_freed: u64 = 0;
    let mut kept: usize = 0;

    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        let Some(name) = path.file_name() else {
            continue;
        };
        let Some(name_str) = name.to_str() else {
            continue;
        };
        if !name_str.starts_with("db_") {
            continue;
        }
        // The legacy username sanitizer could produce a user directory that
        // begins with `db_` (for example `@db_alice:example.org`). Such a
        // directory is never a Matrix SDK database and must not be pruned.
        match tokio::fs::metadata(path.join("persistent_state").join("session")).await {
            Ok(metadata) if metadata.is_file() => {
                kept += 1;
                continue;
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                log!(
                    "prune_orphan_db_dirs: preserving {} because legacy-session detection failed: {error}",
                    path.display()
                );
                kept += 1;
                continue;
            }
        }
        if is_active(name) {
            kept += 1;
            continue;
        }
        let size = dir_size_bytes(&path).await.unwrap_or(0);
        match tokio::fs::remove_dir_all(&path).await {
            Ok(()) => {
                deleted += 1;
                bytes_freed += size;
                // log!(
                //     "prune_orphan_db_dirs: deleted orphaned dir ({size} bytes): {}",
                //     path.display(),
                // );
            }
            Err(e) => {
                log!(
                    "prune_orphan_db_dirs: failed to delete {}: {e}",
                    path.display()
                );
            }
        }
    }

    if deleted > 0 || kept > 0 {
        log!(
            "prune_orphan_db_dirs ({}): deleted {deleted} orphan(s), freed {bytes_freed} bytes; kept {kept} active",
            dir.display()
        );
    }
}

/// Deletes orphaned (no longer used) database and cache directories.
pub async fn cleanup_orphan_db_dirs() {
    use std::{collections::HashSet, ffi::OsString};

    // If we couldn't read the data directory, we can't know which ones are active, so skip pruning.
    let Some(active) = collect_referenced_db_paths().await else {
        return;
    };
    let active_names: HashSet<OsString> = active
        .iter()
        .filter_map(|p| p.file_name().map(ToOwned::to_owned))
        .collect();

    let data_dir = app_data_dir();
    prune_orphan_db_dirs(data_dir, |name| active.contains(&data_dir.join(name))).await;
    prune_orphan_db_dirs(cache_dir(), |name| active_names.contains(name)).await;
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
    let canonical_session_file = session_file_path(&user_id);
    let legacy_session_file = legacy_persistent_state_dir(&user_id).join("session");
    let session_file = if canonical_session_file.exists() {
        canonical_session_file.clone()
    } else if legacy_session_file.exists() {
        legacy_session_file.clone()
    } else {
        log!("Could not find previous session file for user {user_id}");
        bail!("Could not find previous session file");
    };
    let status_str = format!("Loading previous session file for {user_id}...");
    log!("{status_str}: '{}'", session_file.display());
    Cx::post_action(LoginAction::Status {
        title: "Restoring session".into(),
        status: status_str,
    });

    let mut material = load_session_material(&session_file, &user_id).await?;
    if session_file != canonical_session_file {
        save_session_material(
            &canonical_session_file,
            &app_data_dir().join(LATEST_USER_ID_FILE_NAME),
            &mut material,
        )
        .await
        .context("failed to migrate Matrix session into collision-free user storage")?;
    }
    if legacy_session_file.exists() {
        retire_session_material(&legacy_session_file, &user_id)
            .await
            .context("canonical Matrix session committed but legacy location retirement failed")?;
    }
    let status_str = format!(
        "Loaded session file for:\n{user_id}\n\nTrying to connect to homeserver...\n{}",
        material.client_session.homeserver,
    );
    log!("{status_str}");
    Cx::post_action(LoginAction::Status {
        title: "Connecting to homeserver".into(),
        status: status_str,
    });
    let original_stored = material.client_session.db_path.clone();
    let db_path = resolve_db_path(material.client_session.db_path.clone());
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
    let client_result =
        crate::sliding_sync::base_client_builder(&db_path, &material.client_session.passphrase)
            .homeserver_url(material.client_session.homeserver.clone())
            .build()
            .await;
    wipe_client_passphrase(&mut material.client_session);
    let client = client_result?;
    client.set_sliding_sync_version(material.sliding_sync_version.into());
    let status_str = format!(
        "Authenticating previous login session for {}...",
        material.user_session.meta.user_id
    );
    log!("{status_str}");
    Cx::post_action(LoginAction::Status {
        title: "Authenticating session".into(),
        status: status_str,
    });

    // Restore the Matrix user session.
    client
        .restore_session(material.user_session.clone())
        .await?;
    save_latest_user_id(&user_id).await?;

    let sync_token = material.take_sync_token();
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
    let retire_legacy_result = if save_result.is_ok() {
        let legacy_session_file = legacy_persistent_state_dir(&user_id).join("session");
        if legacy_session_file.exists() {
            retire_session_material(&legacy_session_file, &user_id)
                .await
                .context(
                    "canonical Matrix session committed but stale legacy location retirement failed",
                )
        } else {
            Ok(())
        }
    } else {
        Ok(())
    };
    wipe_client_passphrase(&mut material.client_session);
    wipe_session_tokens(&mut material.user_session);
    wipe_sync_token(&mut material.sync_token);
    save_result?;
    retire_legacy_result?;

    log!(
        "Session persisted securely for {user_id} at: {}",
        session_file.display()
    );
    Ok(())
}

/// Remove the persisted Matrix credential, session metadata, and latest-user
/// pointer for a user after server-side logout has invalidated the session.
pub async fn clear_persisted_session(user_id: &UserId) -> anyhow::Result<()> {
    let canonical_result = clear_session_material(
        &session_file_path(user_id),
        &app_data_dir().join(LATEST_USER_ID_FILE_NAME),
        user_id,
    )
    .await;
    let legacy_result = retire_session_material(
        &legacy_persistent_state_dir(user_id).join("session"),
        user_id,
    )
    .await;

    let mut failures = Vec::new();
    if let Err(error) = canonical_result {
        failures.push(format!("canonical session cleanup failed: {error:#}"));
    }
    if let Err(error) = legacy_result {
        failures.push(format!("legacy-location cleanup failed: {error:#}"));
    }
    if failures.is_empty() {
        Ok(())
    } else {
        bail!(
            "Matrix persisted-session cleanup was incomplete: {}",
            failures.join("; ")
        )
    }
}

/// Remove the current user's persisted credentials, metadata, and latest-user
/// pointer after logout has invalidated the server-side session.
///
/// The current Matrix client's session identity is authoritative. The
/// latest-user pointer is deliberately ignored for credential selection; it
/// may be absent, corrupt, or attacker-controlled.
pub async fn delete_latest_user_id() -> anyhow::Result<bool> {
    let user_id = crate::sliding_sync::current_user_id().ok_or_else(|| {
        anyhow::anyhow!(
            "Cannot clear Matrix credentials without an authoritative current client identity"
        )
    })?;
    clear_persisted_session(&user_id).await?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::Result;

    use super::{encoded_user_state_relative_dir, prune_orphan_db_dirs};

    #[test]
    fn user_storage_encoding_is_collision_resistant_for_legacy_collision_shapes() {
        // The old `replace(':', '_')` mapping collapses these raw identities.
        let left = "@a_b:example.org";
        let right = "@a:b_example.org";
        assert_eq!(
            left.replace(':', "_").replace('@', ""),
            right.replace(':', "_").replace('@', "")
        );
        assert_ne!(
            encoded_user_state_relative_dir(left),
            encoded_user_state_relative_dir(right)
        );
    }

    #[test]
    fn user_storage_encoding_keeps_each_component_below_filesystem_limits() {
        let raw = format!("@{}:example.org", "a".repeat(240));
        let encoded = encoded_user_state_relative_dir(&raw);
        assert!(
            encoded
                .components()
                .all(|component| component.as_os_str().len() < 128)
        );
    }

    #[tokio::test]
    async fn legacy_db_prefixed_user_directory_is_never_pruned_as_a_database() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let legacy_user_dir = temp.path().join("db_alice_example.org");
        let persistent_state = legacy_user_dir.join("persistent_state");
        fs::create_dir_all(&persistent_state)?;
        fs::write(persistent_state.join("session"), b"legacy-session")?;

        prune_orphan_db_dirs(temp.path(), |_| false).await;

        assert!(legacy_user_dir.exists());
        assert!(persistent_state.join("session").exists());
        Ok(())
    }
}
