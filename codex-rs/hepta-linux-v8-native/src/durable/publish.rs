use std::ffi::OsStr;
use std::path::Path;

use sha2::Digest as _;

use crate::DirectoryAnchorV8;
use crate::FileIdentityV8;
use crate::NativeErrorV8;
use crate::invalid;
use crate::rename_noreplace_at;

use super::incoming_name_v8;
use super::validate_leaf_name;

const MAX_DURABLE_RECORD_BYTES_V8: u64 = 64 * 1024 * 1024;

/// Opaque proof that one exact byte string was published without replacement,
/// both file and directory were fsynced, and the final name was reopened
/// through the mandatory openat2 policy with matching identity and bytes.
#[derive(Debug)]
pub struct PublishedRecordV8 {
    final_leaf: String,
    identity: FileIdentityV8,
    sha256: String,
    size: u64,
}

impl PublishedRecordV8 {
    pub fn final_leaf(&self) -> &str {
        &self.final_leaf
    }

    pub fn identity(&self) -> FileIdentityV8 {
        self.identity
    }

    pub fn sha256(&self) -> &str {
        &self.sha256
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

pub fn publish_record_noreplace_v8(
    directory: &DirectoryAnchorV8,
    final_leaf: &str,
    publication_nonce: &str,
    bytes: &[u8],
) -> Result<PublishedRecordV8, NativeErrorV8> {
    publish_record_noreplace_observed_v8(directory, final_leaf, publication_nonce, bytes, |_| {})
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum DurablePublicationCheckpointV8 {
    IncomingCreatedBeforeWrite,
    IncomingWrittenBeforeFileSync,
    IncomingFileSyncedBeforeRename,
    RenamedBeforeDirectorySync,
    DirectorySyncedBeforeFinalReopen,
    FinalReopenVerified,
}

pub(super) fn publish_record_noreplace_observed_v8<F>(
    directory: &DirectoryAnchorV8,
    final_leaf: &str,
    publication_nonce: &str,
    bytes: &[u8],
    mut observe: F,
) -> Result<PublishedRecordV8, NativeErrorV8>
where
    F: FnMut(DurablePublicationCheckpointV8),
{
    validate_leaf_name(final_leaf)?;
    if bytes.is_empty() {
        return Err(invalid("durable publication bytes must not be empty"));
    }
    let size = u64::try_from(bytes.len())
        .map_err(|_| invalid("durable publication size overflows u64"))?;
    if size > MAX_DURABLE_RECORD_BYTES_V8 {
        return Err(invalid("durable publication exceeds the size limit"));
    }

    let incoming_leaf = incoming_name_v8(final_leaf, publication_nonce)?;
    let created = directory.create_regular_leaf_exclusive(OsStr::new(&incoming_leaf))?;
    observe(DurablePublicationCheckpointV8::IncomingCreatedBeforeWrite);
    let written = created.write_all_without_sync(bytes)?;
    observe(DurablePublicationCheckpointV8::IncomingWrittenBeforeFileSync);
    let synced = written.sync_and_revalidate()?;
    observe(DurablePublicationCheckpointV8::IncomingFileSyncedBeforeRename);
    synced.revalidate()?;
    let source_identity = synced.identity();

    rename_noreplace_at(
        directory,
        OsStr::new(&incoming_leaf),
        directory,
        OsStr::new(final_leaf),
    )?;
    synced.revalidate()?;
    observe(DurablePublicationCheckpointV8::RenamedBeforeDirectorySync);
    directory.sync_directory()?;
    observe(DurablePublicationCheckpointV8::DirectorySyncedBeforeFinalReopen);

    let reopened = directory.open_regular_readonly_beneath(Path::new(final_leaf))?;
    let reopened_bytes = reopened.read_all(MAX_DURABLE_RECORD_BYTES_V8)?;
    if reopened.identity() != source_identity {
        return Err(invalid(
            "durable publication final inode differs from the fsynced source",
        ));
    }
    if reopened_bytes != bytes {
        return Err(invalid(
            "durable publication final bytes differ from the fsynced source",
        ));
    }
    observe(DurablePublicationCheckpointV8::FinalReopenVerified);

    Ok(PublishedRecordV8 {
        final_leaf: final_leaf.to_string(),
        identity: source_identity,
        sha256: format!("{:x}", sha2::Sha256::digest(bytes)),
        size,
    })
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use std::fs;

    use super::*;

    fn temporary_directory(label: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "hepta-linux-v8-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&path).unwrap();
        path
    }

    #[test]
    fn publication_is_exact_durable_and_noreplace() {
        let root = temporary_directory("publish");
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let nonce = "1".repeat(64);
        let published =
            publish_record_noreplace_v8(&anchor, "00000000000000000001.record", &nonce, b"one")
                .unwrap();
        assert_eq!(published.size(), 3);
        assert_eq!(fs::read(root.join(published.final_leaf())).unwrap(), b"one");
        assert_eq!(published.identity().mode(), 0o600);
        assert_eq!(published.identity().link_count(), 1);

        assert!(
            publish_record_noreplace_v8(
                &anchor,
                "00000000000000000001.record",
                &"2".repeat(64),
                b"replacement"
            )
            .is_err()
        );
        assert_eq!(
            fs::read(root.join("00000000000000000001.record")).unwrap(),
            b"one"
        );
        fs::remove_dir_all(root).unwrap();
    }
}
