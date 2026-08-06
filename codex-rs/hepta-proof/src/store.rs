use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::ProofError;
use crate::command::ProofIntent;
use crate::command::ProofInvocationId;
use crate::command::ProofReceipt;
use crate::command::ProofReceiptId;
use crate::validation::validate_intent;
use crate::validation::validate_receipt;

const MAX_PROOF_RECORD_BYTES: u64 = 256 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProofAppendDisposition {
    Inserted,
    AlreadyPresent,
}

/// Create-new local observation records with intact-root replay blocking.
///
/// The store has no anti-rollback anchor and deliberately performs no
/// automatic stale-lock recovery; operators must treat a stale lock as a
/// liveness fault, not permission to execute again.
#[derive(Clone)]
pub struct ProofStore {
    root: PathBuf,
    intents: PathBuf,
    receipts: PathBuf,
    locks: PathBuf,
}

impl ProofStore {
    pub fn open(root: impl Into<PathBuf>) -> Result<Self, ProofError> {
        let root = root.into();
        validate_directory(&root, "proof root")?;
        let intents = ensure_child_directory(&root, "intents")?;
        let receipts = ensure_child_directory(&root, "receipts")?;
        let locks = ensure_child_directory(&root, "locks")?;
        Ok(Self {
            root,
            intents,
            receipts,
            locks,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub(crate) fn claim_intent(
        &self,
        intent: &ProofIntent,
    ) -> Result<ProofAppendDisposition, ProofError> {
        validate_directory(&self.intents, "proof intents directory")?;
        validate_intent(intent)?;
        append_record(
            &self.intent_path(&intent.invocation_id),
            intent.invocation_id.as_str(),
            intent,
        )
    }

    pub(crate) fn append_receipt(
        &self,
        receipt: &ProofReceipt,
    ) -> Result<ProofAppendDisposition, ProofError> {
        validate_directory(&self.receipts, "proof receipts directory")?;
        validate_receipt(receipt)?;
        self.validate_receipt_intent_binding(receipt)?;
        append_record(
            &self.receipt_path(receipt.receipt_id()),
            receipt.receipt_id().as_str(),
            receipt,
        )
    }

    pub fn get_receipt(
        &self,
        receipt_id: &ProofReceiptId,
    ) -> Result<Option<ProofReceipt>, ProofError> {
        validate_directory(&self.receipts, "proof receipts directory")?;
        let receipt = read_record(&self.receipt_path(receipt_id), receipt_id.as_str())?;
        let Some(receipt) = receipt else {
            return Ok(None);
        };
        validate_receipt(&receipt)?;
        if receipt.receipt_id() != receipt_id {
            return Err(ProofError::Corrupt(
                "proof receipt does not match the requested identity".to_string(),
            ));
        }
        self.validate_receipt_intent_binding(&receipt)?;
        Ok(Some(receipt))
    }

    pub(crate) fn acquire_lock(
        &self,
        invocation_id: &ProofInvocationId,
    ) -> Result<ProofLock, ProofError> {
        validate_directory(&self.locks, "proof locks directory")?;
        let path = self
            .locks
            .join(format!("{}.lock", invocation_id.digest_suffix()));
        acquire_file_lock(path, invocation_id.as_str())
    }

    fn validate_receipt_intent_binding(&self, receipt: &ProofReceipt) -> Result<(), ProofError> {
        validate_directory(&self.intents, "proof intents directory")?;
        let intent = read_record::<ProofIntent>(
            &self.intent_path(receipt.invocation_id()),
            receipt.invocation_id().as_str(),
        )?
        .ok_or_else(|| {
            ProofError::Corrupt("proof receipt has no corresponding intent".to_string())
        })?;
        validate_intent(&intent)?;
        if receipt.invocation_id() != &intent.invocation_id
            || receipt.subject() != &intent.subject
            || receipt.command_binding_sha256() != &intent.command_binding_sha256
        {
            return Err(ProofError::Corrupt(
                "proof receipt does not match its intent bindings".to_string(),
            ));
        }
        Ok(())
    }

    fn intent_path(&self, invocation_id: &ProofInvocationId) -> PathBuf {
        self.intents
            .join(format!("{}.json", invocation_id.digest_suffix()))
    }

    fn receipt_path(&self, receipt_id: &ProofReceiptId) -> PathBuf {
        self.receipts
            .join(format!("{}.json", receipt_id.digest_suffix()))
    }
}

pub(crate) struct ProofLock {
    path: PathBuf,
    _file: File,
}

fn acquire_file_lock(path: PathBuf, lock_id: &str) -> Result<ProofLock, ProofError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|error| {
            ProofError::StoreUnavailable(format!("failed to acquire proof lock: {error}"))
        })?;
    let lock_record = ProofLockRecord {
        schema_version: 1,
        lock_id,
        process_id: std::process::id(),
    };
    let bytes = canonical_json(&lock_record)?;
    file.write_all(&bytes).map_err(store_error)?;
    file.sync_all().map_err(store_error)?;
    Ok(ProofLock { path, _file: file })
}

impl Drop for ProofLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

#[derive(Serialize)]
struct ProofLockRecord<'a> {
    schema_version: u32,
    lock_id: &'a str,
    process_id: u32,
}

fn append_record<T: Serialize>(
    path: &Path,
    record_id: &str,
    value: &T,
) -> Result<ProofAppendDisposition, ProofError> {
    let bytes = canonical_json(value)?;
    match OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(mut file) => {
            file.write_all(&bytes).map_err(store_error)?;
            file.sync_all().map_err(store_error)?;
            sync_parent_directory(path)?;
            Ok(ProofAppendDisposition::Inserted)
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let existing = read_bounded(path)?;
            if existing == bytes {
                Ok(ProofAppendDisposition::AlreadyPresent)
            } else {
                Err(ProofError::EvidenceConflict {
                    record_id: record_id.to_string(),
                })
            }
        }
        Err(error) => Err(ProofError::StoreUnavailable(format!(
            "failed to create proof record: {error}"
        ))),
    }
}

fn read_record<T: DeserializeOwned + Serialize>(
    path: &Path,
    record_id: &str,
) -> Result<Option<T>, ProofError> {
    match fs::symlink_metadata(path) {
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(store_error(error)),
    }
    let bytes = read_bounded(path)?;
    let value = serde_json::from_slice::<T>(&bytes)
        .map_err(|error| ProofError::Corrupt(format!("invalid proof record JSON: {error}")))?;
    if canonical_json(&value)? != bytes {
        return Err(ProofError::Corrupt(format!(
            "proof record {record_id} is not canonical"
        )));
    }
    Ok(Some(value))
}

fn read_bounded(path: &Path) -> Result<Vec<u8>, ProofError> {
    let metadata = fs::symlink_metadata(path).map_err(store_error)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(ProofError::Corrupt(
            "proof record is not a regular file".to_string(),
        ));
    }
    if metadata.len() > MAX_PROOF_RECORD_BYTES {
        return Err(ProofError::Corrupt(
            "proof record exceeds the hard size limit".to_string(),
        ));
    }
    let file = File::open(path).map_err(store_error)?;
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    file.take(MAX_PROOF_RECORD_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(store_error)?;
    if bytes.len() as u64 > MAX_PROOF_RECORD_BYTES {
        return Err(ProofError::Corrupt(
            "proof record exceeds the hard size limit".to_string(),
        ));
    }
    Ok(bytes)
}

pub(crate) fn canonical_json<T: Serialize>(value: &T) -> Result<Vec<u8>, ProofError> {
    let mut value = serde_json::to_value(value)
        .map_err(|error| ProofError::Corrupt(format!("proof serialization failed: {error}")))?;
    sort_value(&mut value);
    serde_json::to_vec(&value)
        .map_err(|error| ProofError::Corrupt(format!("proof serialization failed: {error}")))
}

fn sort_value(value: &mut Value) {
    match value {
        Value::Array(items) => {
            for item in items {
                sort_value(item);
            }
        }
        Value::Object(map) => {
            let mut entries = std::mem::take(map).into_iter().collect::<Vec<_>>();
            entries.sort_by(|(left, _), (right, _)| left.cmp(right));
            for (_, item) in &mut entries {
                sort_value(item);
            }
            map.extend(entries);
        }
        _ => {}
    }
}

fn ensure_child_directory(root: &Path, name: &str) -> Result<PathBuf, ProofError> {
    let path = root.join(name);
    match fs::create_dir(&path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(error) => return Err(store_error(error)),
    }
    validate_directory(&path, name)?;
    Ok(path)
}

fn validate_directory(path: &Path, label: &str) -> Result<(), ProofError> {
    if !path.is_absolute() {
        return Err(ProofError::InvalidInput(format!(
            "{label} must be absolute"
        )));
    }
    let metadata = fs::symlink_metadata(path).map_err(store_error)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(ProofError::InvalidInput(format!(
            "{label} must be a non-symlink directory"
        )));
    }
    Ok(())
}

fn store_error(error: std::io::Error) -> ProofError {
    ProofError::StoreUnavailable(error.to_string())
}

#[cfg(unix)]
fn sync_parent_directory(path: &Path) -> Result<(), ProofError> {
    let parent = path.parent().ok_or_else(|| {
        ProofError::StoreUnavailable("proof record has no parent directory".to_string())
    })?;
    File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(store_error)
}

#[cfg(not(unix))]
fn sync_parent_directory(_path: &Path) -> Result<(), ProofError> {
    Ok(())
}
