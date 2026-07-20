use std::path::PathBuf;

use matrix_sdk::authentication::matrix::MatrixSession;
use matrix_sdk::sliding_sync;
use serde::Deserialize;
use serde::Serialize;
use zeroize::Zeroize;

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
pub(super) struct SessionMetadataPersisted {
    pub(super) version: u8,
    pub(super) client_session: ClientSessionMetadataPersisted,
    pub(super) credential_account: String,
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
