use std::{mem::ManuallyDrop, path::PathBuf, ptr};

use matrix_sdk::{authentication::matrix::MatrixSession, ruma::OwnedUserId, sliding_sync};
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// The in-memory data needed to re-build a Matrix client.
///
/// The database passphrase deliberately does not implement `Serialize`: only
/// [`SessionSecretsPersisted`] can serialize it. That payload is encrypted into
/// the authenticated filesystem envelope; the operating-system credential
/// store contains only the small generation-specific master key.
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

impl Drop for ClientSessionPersisted {
    fn drop(&mut self) {
        self.passphrase.zeroize();
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

impl SessionMaterial {
    pub(crate) fn take_sync_token(&mut self) -> Option<String> {
        self.sync_token.take()
    }
}

impl Drop for SessionMaterial {
    fn drop(&mut self) {
        wipe_session_material(self);
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SessionMetadataPersisted {
    pub(super) version: u8,
    pub(super) user_id: OwnedUserId,
    pub(super) client_session: ClientSessionMetadataPersisted,
    pub(super) credential_account: String,
    /// A random identifier shared with the credential payload. It makes each
    /// save use a fresh keyring account, so a crash can leave at worst an
    /// unreachable old/new credential rather than a mismatched live pair.
    pub(super) binding_nonce: String,
    /// Keyed BLAKE3 authentication tag for all non-secret metadata plus the
    /// authoritative Matrix user ID. The authentication key lives only in the
    /// OS credential store.
    pub(super) binding_tag: String,
    pub(super) encryption_nonce: String,
    /// AEAD ciphertext containing the Matrix tokens, database passphrase, and
    /// sync token. No plaintext credential material is stored on disk.
    pub(super) encrypted_secrets: String,
    pub(super) sliding_sync_version: SlidingSyncVersion,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ClientSessionMetadataPersisted {
    pub(super) homeserver: String,
    pub(super) db_path: PathBuf,
}

/// This is the only serialized representation containing Matrix credentials.
/// Its JSON value is stored directly in the OS credential store and is never
/// passed to the filesystem writer.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SessionSecretsPersisted {
    pub(super) version: u8,
    pub(super) database_passphrase: String,
    pub(super) user_session: MatrixSession,
    pub(super) sync_token: Option<String>,
}

impl SessionSecretsPersisted {
    pub(super) fn wipe(&mut self) {
        self.database_passphrase.zeroize();
        wipe_session_tokens(&mut self.user_session);
        wipe_sync_token(&mut self.sync_token);
    }

    pub(super) fn into_material(
        self,
        homeserver: String,
        db_path: PathBuf,
        sliding_sync_version: SlidingSyncVersion,
    ) -> SessionMaterial {
        let this = ManuallyDrop::new(self);
        // SAFETY: `this` will not be dropped, every secret-bearing field is
        // moved exactly once into a `SessionMaterial` that zeroizes on drop.
        unsafe {
            SessionMaterial {
                client_session: ClientSessionPersisted {
                    homeserver,
                    db_path,
                    passphrase: ptr::read(&this.database_passphrase),
                },
                user_session: ptr::read(&this.user_session),
                sync_token: ptr::read(&this.sync_token),
                sliding_sync_version,
            }
        }
    }
}

impl Drop for SessionSecretsPersisted {
    fn drop(&mut self) {
        self.wipe();
    }
}

/// Small generation-specific keyring payload. Its serialized size is fixed and
/// remains far below the Windows Credential Manager blob limit, regardless of
/// Matrix token length.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SessionCredentialKeyPersisted {
    pub(super) version: u8,
    pub(super) master_key: Vec<u8>,
    pub(super) binding_nonce: String,
}

impl SessionCredentialKeyPersisted {
    pub(super) fn wipe(&mut self) {
        self.master_key.zeroize();
    }
}

impl Drop for SessionCredentialKeyPersisted {
    fn drop(&mut self) {
        self.wipe();
    }
}

/// Read-only compatibility shape for the first split metadata/keyring format.
/// Version 2 metadata was not authenticated and is therefore never restored;
/// it is used only to derive a same-user credential account during logout.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)] // Compatibility-only fields are decoded but never trusted for restore.
pub(super) struct LegacySecureMetadataV2 {
    pub(super) version: u8,
    pub(super) client_session: ClientSessionMetadataPersisted,
    pub(super) credential_account: String,
    pub(super) sliding_sync_version: SlidingSyncVersion,
}

/// Read-only compatibility shape for the plaintext session format used before
/// metadata schema v2. It is never used by the save path.
#[derive(Deserialize)]
pub(super) struct LegacyFullSessionPersisted {
    pub(super) client_session: LegacyClientSessionPersisted,
    pub(super) user_session: MatrixSession,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) sync_token: Option<String>,
    #[serde(default)]
    pub(super) sliding_sync_version: SlidingSyncVersion,
}

#[derive(Deserialize)]
pub(super) struct LegacyClientSessionPersisted {
    pub(super) homeserver: String,
    pub(super) db_path: PathBuf,
    pub(super) passphrase: String,
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

pub(super) fn wipe_session_material(material: &mut SessionMaterial) {
    wipe_client_passphrase(&mut material.client_session);
    wipe_session_tokens(&mut material.user_session);
    wipe_sync_token(&mut material.sync_token);
}
