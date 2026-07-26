//! Authenticated last-committed preference attachment for restart hydration.

use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;

use anyhow::Context;
use anyhow::Result;
use hepta_contracts::PreferenceId;
use hepta_contracts::PrincipalId;
use hepta_contracts::Revision;
use hepta_contracts::RevisionStamp;
use hepta_intelligence::DurableHmacTrustedPreferenceIngress;
use hmac::Hmac;
use hmac::Mac;
use serde::Deserialize;
use serde::Serialize;
use sha2::Sha256;
use zeroize::Zeroizing;

const ATTACHMENT_SCHEMA: &str = "hepta.native-preference-session-attachment.v1";
const ATTACHMENT_MAC_DOMAIN: &[u8] = b"hepta.native-preference-session-attachment.hmac-sha256.v1";
const MAX_ATTACHMENT_BYTES: u64 = 4096;
type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub(crate) struct PreferenceAttachmentCandidate {
    pub(crate) session_binding_hash: String,
    pub(crate) subject: String,
    pub(crate) preference: String,
    pub(crate) stamp: RevisionStamp,
}

pub(crate) struct PreferenceAttachmentStore {
    path: PathBuf,
    lock_path: PathBuf,
    key: Zeroizing<[u8; 32]>,
    operation: Mutex<()>,
}

impl std::fmt::Debug for PreferenceAttachmentStore {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PreferenceAttachmentStore")
            .field("path", &self.path)
            .field("lock_path", &self.lock_path)
            .field("key", &"[REDACTED]")
            .finish()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AttachmentWire {
    schema: String,
    session_binding_hash: String,
    subject: String,
    preference: String,
    revision: u64,
    content_hash: String,
    mac: String,
}

impl PreferenceAttachmentStore {
    pub(crate) fn for_database(database: &Path, key: [u8; 32]) -> Result<Self> {
        let file_name = database
            .file_name()
            .context("preference attachment database path must name a file")?
            .to_string_lossy();
        let path = database.with_file_name(format!("{file_name}.session-attachment.json"));
        let lock_path = database.with_file_name(format!("{file_name}.session-attachment.lock"));
        Ok(Self {
            path,
            lock_path,
            key: Zeroizing::new(key),
            operation: Mutex::new(()),
        })
    }

    pub(crate) fn persist(&self, candidate: &PreferenceAttachmentCandidate) -> Result<()> {
        self.persist_with_after_verified_read(candidate, || {})
    }

    fn persist_with_after_verified_read(
        &self,
        candidate: &PreferenceAttachmentCandidate,
        after_verified_read: impl FnOnce(),
    ) -> Result<()> {
        validate_candidate(candidate)?;
        let _operation = self
            .operation
            .lock()
            .map_err(|_| anyhow::anyhow!("preference attachment operation mutex poisoned"))?;
        let _file_lock = PreferenceAttachmentFileLock::acquire(&self.lock_path)?;
        let current = self.read_verified()?;
        after_verified_read();
        if let Some(current) = current {
            if candidate.stamp.revision().get() < current.revision {
                anyhow::bail!(
                    "authenticated preference attachment revision rollback from {} to {}",
                    current.revision,
                    candidate.stamp.revision()
                );
            }
            if candidate.stamp.revision().get() == current.revision {
                if candidate.stamp.content_hash().as_str() != current.content_hash {
                    anyhow::bail!(
                        "authenticated preference attachment diverged at revision {}",
                        current.revision
                    );
                }
                if candidate.session_binding_hash != current.session_binding_hash
                    || candidate.subject != current.subject
                    || candidate.preference != current.preference
                {
                    anyhow::bail!(
                        "authenticated preference attachment bindings diverged at revision {}",
                        current.revision
                    );
                }
                return Ok(());
            }
        }
        self.publish(candidate)
    }

    fn publish(&self, candidate: &PreferenceAttachmentCandidate) -> Result<()> {
        let mut wire = AttachmentWire {
            schema: ATTACHMENT_SCHEMA.into(),
            session_binding_hash: candidate.session_binding_hash.clone(),
            subject: candidate.subject.clone(),
            preference: candidate.preference.clone(),
            revision: candidate.stamp.revision().get(),
            content_hash: candidate.stamp.content_hash().as_str().to_owned(),
            mac: String::new(),
        };
        wire.mac = self.sign(&wire)?;
        let bytes =
            serde_json::to_vec(&wire).context("encode authenticated preference attachment")?;
        if bytes.len() as u64 > MAX_ATTACHMENT_BYTES {
            anyhow::bail!("authenticated preference attachment exceeds bounded size");
        }
        let parent = self
            .path
            .parent()
            .context("preference attachment path has no parent")?;
        let mut temporary = tempfile::Builder::new()
            .prefix(".hepta-preference-attachment-")
            .tempfile_in(parent)
            .context("create private preference attachment staging file")?;
        temporary
            .as_file_mut()
            .write_all(&bytes)
            .context("write authenticated preference attachment")?;
        temporary
            .as_file_mut()
            .write_all(b"\n")
            .context("terminate authenticated preference attachment")?;
        temporary
            .as_file()
            .sync_all()
            .context("fsync authenticated preference attachment")?;
        temporary
            .persist(&self.path)
            .map_err(|error| error.error)
            .context("atomically publish authenticated preference attachment")?;
        sync_parent(parent)?;
        Ok(())
    }

    pub(crate) async fn hydrate(
        &self,
        authority: &DurableHmacTrustedPreferenceIngress,
        expected_session_binding_hash: &str,
    ) -> Result<Option<RevisionStamp>> {
        let Some(wire) = self.read_verified()? else {
            return Ok(None);
        };
        if wire.session_binding_hash != expected_session_binding_hash {
            anyhow::bail!(
                "authenticated preference attachment belongs to a different runtime session"
            );
        }
        let preference = PreferenceId::new(wire.preference);
        let subject = PrincipalId::new(wire.subject);
        let document = authority
            .read_document(&preference, &subject)
            .await
            .context("read authenticated preference attachment document")?
            .context("authenticated preference attachment document is missing")?;
        if document.state().revision() != Revision::new(wire.revision)
            || document.state().content_hash().as_str() != wire.content_hash
        {
            anyhow::bail!(
                "authenticated preference attachment differs from keyed durable preference state"
            );
        }
        Ok(Some(RevisionStamp::new(
            document.state().revision(),
            document.state().content_hash().clone(),
        )))
    }

    fn read_verified(&self) -> Result<Option<AttachmentWire>> {
        let Some(bytes) = read_bounded_private_file(&self.path)? else {
            return Ok(None);
        };
        let wire: AttachmentWire =
            serde_json::from_slice(&bytes).context("decode authenticated preference attachment")?;
        if wire.schema != ATTACHMENT_SCHEMA
            || wire.session_binding_hash.is_empty()
            || wire.subject.is_empty()
            || wire.preference.is_empty()
            || wire.content_hash.is_empty()
            || wire.mac.len() != 64
        {
            anyhow::bail!("authenticated preference attachment has invalid bindings");
        }
        let expected = self.sign(&wire)?;
        if !constant_time_hex_equal(&expected, &wire.mac) {
            anyhow::bail!("authenticated preference attachment MAC is invalid");
        }
        Ok(Some(wire))
    }

    fn sign(&self, wire: &AttachmentWire) -> Result<String> {
        let mut mac = HmacSha256::new_from_slice(self.key.as_ref())
            .context("initialize preference attachment HMAC")?;
        let revision = wire.revision.to_string();
        update_frame(&mut mac, ATTACHMENT_MAC_DOMAIN);
        for value in [
            wire.schema.as_bytes(),
            wire.session_binding_hash.as_bytes(),
            wire.subject.as_bytes(),
            wire.preference.as_bytes(),
            revision.as_bytes(),
            wire.content_hash.as_bytes(),
        ] {
            update_frame(&mut mac, value);
        }
        Ok(hex_encode(&mac.finalize().into_bytes()))
    }
}

struct PreferenceAttachmentFileLock {
    file: fs::File,
}

impl PreferenceAttachmentFileLock {
    #[cfg(unix)]
    fn acquire(path: &Path) -> Result<Self> {
        use std::os::unix::fs::OpenOptionsExt;

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o600)
            .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
            .open(path)
            .context("open private preference attachment sidecar lock")?;
        lock_attachment(&file)?;
        if let Err(error) = validate_attachment_lock_file(&file) {
            unlock_attachment(&file);
            return Err(error);
        }
        Ok(Self { file })
    }

    #[cfg(not(unix))]
    fn acquire(_path: &Path) -> Result<Self> {
        anyhow::bail!("preference attachment sidecar locking requires Unix flock semantics")
    }
}

impl Drop for PreferenceAttachmentFileLock {
    fn drop(&mut self) {
        unlock_attachment(&self.file);
    }
}

fn validate_candidate(candidate: &PreferenceAttachmentCandidate) -> Result<()> {
    if candidate.session_binding_hash.is_empty()
        || candidate.subject.trim().is_empty()
        || candidate.preference.trim().is_empty()
        || candidate.stamp.content_hash().as_str().is_empty()
    {
        anyhow::bail!("preference attachment requires exact non-empty bindings");
    }
    Ok(())
}

fn update_frame(mac: &mut HmacSha256, value: &[u8]) {
    mac.update(&(value.len() as u64).to_be_bytes());
    mac.update(value);
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}

fn constant_time_hex_equal(left: &str, right: &str) -> bool {
    if left.len() != right.len() {
        return false;
    }
    left.bytes()
        .zip(right.bytes())
        .fold(0_u8, |difference, (left, right)| {
            difference | (left ^ right)
        })
        == 0
}

fn sync_parent(parent: &Path) -> Result<()> {
    fs::File::open(parent)
        .and_then(|directory| directory.sync_all())
        .context("fsync preference attachment parent directory")
}

fn read_bounded_private_file(path: &Path) -> Result<Option<Vec<u8>>> {
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW);
    }
    let mut file = match options.open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(error).context("open authenticated preference attachment without links");
        }
    };
    let metadata = file
        .metadata()
        .context("inspect opened authenticated preference attachment")?;
    if !metadata.file_type().is_file() || metadata.len() > MAX_ATTACHMENT_BYTES {
        anyhow::bail!("authenticated preference attachment is not a bounded regular file");
    }
    validate_private_file_mode(&metadata)?;
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    Read::by_ref(&mut file)
        .take(MAX_ATTACHMENT_BYTES + 1)
        .read_to_end(&mut bytes)
        .context("read authenticated preference attachment")?;
    if bytes.len() as u64 > MAX_ATTACHMENT_BYTES {
        anyhow::bail!("authenticated preference attachment exceeds bounded size");
    }
    Ok(Some(bytes))
}

#[cfg(unix)]
fn validate_private_file_mode(metadata: &fs::Metadata) -> Result<()> {
    use std::os::unix::fs::MetadataExt;
    use std::os::unix::fs::PermissionsExt;
    if metadata.permissions().mode() & 0o077 != 0 {
        anyhow::bail!("authenticated preference attachment permissions must deny group/other");
    }
    // SAFETY: `geteuid` reads process credentials and has no pointer or lifetime inputs.
    let effective_uid = unsafe { libc::geteuid() };
    if metadata.nlink() != 1 || metadata.uid() != effective_uid {
        anyhow::bail!("authenticated preference attachment owner or link count is invalid");
    }
    Ok(())
}

#[cfg(not(unix))]
fn validate_private_file_mode(_metadata: &fs::Metadata) -> Result<()> {
    Ok(())
}

fn validate_attachment_lock_file(file: &fs::File) -> Result<()> {
    let metadata = file
        .metadata()
        .context("inspect preference attachment sidecar lock")?;
    if !metadata.file_type().is_file() || metadata.len() != 0 {
        anyhow::bail!("preference attachment sidecar lock is not an empty regular file");
    }
    validate_private_file_mode(&metadata)
}

#[cfg(unix)]
fn lock_attachment(file: &fs::File) -> Result<()> {
    use std::os::fd::AsRawFd;

    // SAFETY: flock only consumes the valid descriptor and a constant operation.
    if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX) } != 0 {
        return Err(std::io::Error::last_os_error()).context("lock preference attachment sidecar");
    }
    Ok(())
}

#[cfg(not(unix))]
fn lock_attachment(_file: &fs::File) -> Result<()> {
    anyhow::bail!("preference attachment sidecar locking requires Unix flock semantics")
}

#[cfg(unix)]
fn unlock_attachment(file: &fs::File) {
    use std::os::fd::AsRawFd;

    // SAFETY: flock only consumes the valid descriptor and a constant operation.
    let _ = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) };
}

#[cfg(not(unix))]
fn unlock_attachment(_file: &fs::File) {}

#[cfg(all(test, unix))]
#[path = "../tests/unit/preference_attachment.rs"]
mod tests;
