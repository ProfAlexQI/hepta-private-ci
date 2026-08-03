use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
#[cfg(not(target_os = "android"))]
use codex_keyring_store::DefaultKeyringStore;
use codex_keyring_store::KeyringStore;
use matrix_sdk::ruma::UserId;
use zeroize::Zeroizing;

#[cfg(any(target_os = "android", test))]
mod android_contract;
#[cfg(target_os = "android")]
mod android_keystore;

pub(super) const MATRIX_CREDENTIAL_SERVICE: &str = "ai.hepta.native.matrix";
pub(super) const SYSTEM_CREDENTIAL_STORE_SUPPORTED: bool = cfg!(any(
    target_os = "freebsd",
    target_os = "android",
    target_os = "ios",
    target_os = "linux",
    target_os = "macos",
    target_os = "openbsd",
    target_os = "windows"
));

pub(super) fn legacy_credential_account(user_id: &UserId) -> String {
    format!("matrix-session-v1|{user_id}")
}

pub(super) fn credential_account(user_id: &UserId, binding_nonce: &str) -> String {
    format!("matrix-session-v2|{user_id}|{binding_nonce}")
}

pub(super) fn default_keyring_store() -> Arc<dyn KeyringStore> {
    #[cfg(target_os = "android")]
    return Arc::new(android_keystore::AndroidKeystoreStore);
    #[cfg(not(target_os = "android"))]
    Arc::new(DefaultKeyringStore)
}

pub(super) fn ensure_system_credential_store_supported() -> Result<()> {
    if SYSTEM_CREDENTIAL_STORE_SUPPORTED {
        Ok(())
    } else {
        anyhow::bail!(
            "secure Matrix session persistence is unavailable on this platform; re-login is required"
        )
    }
}

pub(super) async fn keyring_load(
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

pub(super) async fn keyring_save(
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

pub(super) async fn keyring_delete(store: Arc<dyn KeyringStore>, account: String) -> Result<bool> {
    tokio::task::spawn_blocking(move || {
        store
            .delete(MATRIX_CREDENTIAL_SERVICE, &account)
            .map_err(|error| anyhow!(error.message()))
            .with_context(|| format!("failed to delete Matrix credentials for {account}"))
    })
    .await
    .context("Matrix credential-store delete task failed")?
}
