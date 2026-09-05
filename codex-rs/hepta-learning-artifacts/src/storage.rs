//! Create-only snapshots and candidate payloads over host-authorized files.
//! The host authenticates files, receipts, current revocations and selection.

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::fs::TryLockError;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use codex_hepta_types::Digest32;
use codex_hepta_types::Generation;
use codex_hepta_types::StableId;

use crate::ArtifactEvent;
use crate::ArtifactKind;
use crate::ArtifactManifest;
use crate::ArtifactRegistry;
use crate::RegistryAppendDisposition;
use crate::StateChange;

const MAX_SNAPSHOT: usize = 8 * 1024 * 1024;
const MAX_PAYLOAD: usize = 64 * 1024 * 1024;
const MAX_RECORDS: usize = 4096;
const MAGIC: &str = "HEPTAR01";

/// Exact bytes and history witness. This is not a signature or acceptance.
/// Retain and authenticate it outside the suspect file; never derive an expected
/// receipt from the file being checked. The host enforces latest revocation state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RegistrySnapshotReceipt {
    pub binding: Digest32,
    pub head_digest: Digest32,
    pub file_digest: Digest32,
    pub records: usize,
    pub encoded_bytes: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactStorageError {
    InvalidBinding,
    InvalidReceipt,
    Busy,
    NotRegular,
    AlreadyExists,
    Capacity,
    Corrupt,
    Semantic,
    Unavailable,
    PayloadMismatch,
    Indeterminate,
    Io(io::ErrorKind),
}

impl fmt::Display for ArtifactStorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl Error for ArtifactStorageError {}
impl From<io::Error> for ArtifactStorageError {
    fn from(value: io::Error) -> Self {
        Self::Io(value.kind())
    }
}

/// Write a new immutable snapshot; an existing file is never overwritten.
/// Directory durability, witness publication, retention and selection are host work.
pub fn write_registry_snapshot(
    file: File,
    registry: &ArtifactRegistry,
    binding: Digest32,
) -> Result<RegistrySnapshotReceipt, ArtifactStorageError> {
    if binding.is_zero() {
        return Err(ArtifactStorageError::InvalidBinding);
    }
    let bytes = encode_snapshot(registry, binding)?;
    let receipt = RegistrySnapshotReceipt {
        binding,
        head_digest: registry.snapshot().head_digest,
        file_digest: Digest32::of_bytes(&bytes),
        records: registry.records().len(),
        encoded_bytes: bytes.len(),
    };
    write_new(file, &bytes)?;
    Ok(receipt)
}

/// Rebuild the same canonical registry and revocation lineage from exact bytes.
/// There is no repair, initialization, old-snapshot fallback or selected pointer.
pub fn read_registry_snapshot(
    file: File,
    expected: RegistrySnapshotReceipt,
) -> Result<ArtifactRegistry, ArtifactStorageError> {
    if expected.binding.is_zero()
        || expected.file_digest.is_zero()
        || expected.records > MAX_RECORDS
        || expected.encoded_bytes > MAX_SNAPSHOT
        || expected.encoded_bytes == 0
        || (expected.records == 0) != expected.head_digest.is_zero()
    {
        return Err(ArtifactStorageError::InvalidReceipt);
    }
    let bytes = read_bounded(file, MAX_SNAPSHOT)?;
    if bytes.len() != expected.encoded_bytes || Digest32::of_bytes(&bytes) != expected.file_digest {
        return Err(ArtifactStorageError::Corrupt);
    }
    let text = std::str::from_utf8(&bytes).map_err(|_| ArtifactStorageError::Corrupt)?;
    let mut lines = text.lines();
    if lines.next() != Some(MAGIC)
        || lines.next() != Some(expected.binding.to_string().as_str())
        || lines.next() != Some(expected.records.to_string().as_str())
    {
        return Err(ArtifactStorageError::Corrupt);
    }
    let mut registry = ArtifactRegistry::new();
    for line in lines {
        if registry.records().len() >= expected.records || line.len() > 2048 {
            return Err(ArtifactStorageError::Corrupt);
        }
        let receipt = registry
            .append(decode_event(line)?)
            .map_err(|_| ArtifactStorageError::Semantic)?;
        if receipt.disposition != RegistryAppendDisposition::Appended {
            return Err(ArtifactStorageError::Corrupt);
        }
    }
    if registry.records().len() != expected.records
        || registry.snapshot().head_digest != expected.head_digest
        || encode_snapshot(&registry, expected.binding)? != bytes
    {
        return Err(ArtifactStorageError::Corrupt);
    }
    Ok(registry)
}

/// Persist exactly the bytes of a currently eligible candidate, never select it.
pub fn write_candidate_payload(
    file: File,
    registry: &ArtifactRegistry,
    artifact: &StableId,
    bytes: &[u8],
) -> Result<Digest32, ArtifactStorageError> {
    let manifest = eligible_manifest(registry, artifact)?;
    validate_payload(manifest, bytes)?;
    write_new(file, bytes)?;
    Ok(manifest.content_digest)
}

/// Load candidate bytes using a CURRENT host-authenticated registry snapshot.
/// Successful loading is not authority to execute, install or select the bytes.
pub fn read_candidate_payload(
    file: File,
    registry: &ArtifactRegistry,
    artifact: &StableId,
) -> Result<Vec<u8>, ArtifactStorageError> {
    let manifest = eligible_manifest(registry, artifact)?;
    let bytes = read_bounded(file, MAX_PAYLOAD)?;
    validate_payload(manifest, &bytes)?;
    Ok(bytes)
}

fn eligible_manifest<'a>(
    registry: &'a ArtifactRegistry,
    artifact: &StableId,
) -> Result<&'a ArtifactManifest, ArtifactStorageError> {
    if !registry.is_eligible(artifact) {
        return Err(ArtifactStorageError::Unavailable);
    }
    registry.manifest(artifact).ok_or(ArtifactStorageError::Unavailable)
}

fn validate_payload(manifest: &ArtifactManifest, bytes: &[u8]) -> Result<(), ArtifactStorageError> {
    if bytes.is_empty() || bytes.len() > MAX_PAYLOAD {
        return Err(ArtifactStorageError::Capacity);
    }
    if bytes.len() as u64 != manifest.encoded_size_bytes
        || Digest32::of_bytes(bytes) != manifest.content_digest
    {
        return Err(ArtifactStorageError::PayloadMismatch);
    }
    Ok(())
}

fn lock(file: &File) -> Result<(), ArtifactStorageError> {
    if !file.metadata()?.is_file() {
        return Err(ArtifactStorageError::NotRegular);
    }
    match file.try_lock() {
        Ok(()) => Ok(()),
        Err(TryLockError::WouldBlock) => Err(ArtifactStorageError::Busy),
        Err(TryLockError::Error(error)) => Err(error.into()),
    }
}

fn write_new(mut file: File, bytes: &[u8]) -> Result<(), ArtifactStorageError> {
    lock(&file)?;
    if file.metadata()?.len() != 0 {
        return Err(ArtifactStorageError::AlreadyExists);
    }
    file.seek(SeekFrom::Start(0))?;
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|_| ArtifactStorageError::Indeterminate)
}

fn read_bounded(mut file: File, limit: usize) -> Result<Vec<u8>, ArtifactStorageError> {
    lock(&file)?;
    if file.metadata()?.len() > limit as u64 {
        return Err(ArtifactStorageError::Capacity);
    }
    file.seek(SeekFrom::Start(0))?;
    let mut bytes = Vec::new();
    file.take(limit as u64 + 1).read_to_end(&mut bytes)?;
    if bytes.len() > limit {
        return Err(ArtifactStorageError::Capacity);
    }
    Ok(bytes)
}

fn encode_snapshot(registry: &ArtifactRegistry, binding: Digest32) -> Result<Vec<u8>, ArtifactStorageError> {
    let count = registry.records().len();
    if count > MAX_RECORDS {
        return Err(ArtifactStorageError::Capacity);
    }
    let mut text = format!("{MAGIC}\n{binding}\n{count}\n");
    for record in registry.records() {
        match &record.event {
            ArtifactEvent::Register { event_id, manifest: m } => {
                let predecessor = m.predecessor_id.as_ref().map_or("", StableId::as_str);
                text.push_str(&format!(
                    "R|{event_id}|{}|{}|{}|{predecessor}|{}|{}|{}|{}|{}|{}\n",
                    m.artifact_id, m.kind.tag(), m.generation.get(), m.content_digest,
                    m.objective_digest, m.support_digest, m.producer_id,
                    m.compatibility_digest, m.encoded_size_bytes,
                ));
            }
            ArtifactEvent::Quarantine(change) | ArtifactEvent::Revoke(change) => {
                let tag = match &record.event {
                    ArtifactEvent::Quarantine(_) => "Q",
                    ArtifactEvent::Revoke(_) => "V",
                    ArtifactEvent::Register { .. } => return Err(ArtifactStorageError::Semantic),
                };
                text.push_str(&format!("{tag}|{}|{}|{}|{}\n", change.event_id,
                                       change.artifact_id, change.evaluator_id, change.reason_digest));
            }
        }
    }
    if text.len() > MAX_SNAPSHOT {
        return Err(ArtifactStorageError::Capacity);
    }
    Ok(text.into_bytes())
}

fn decode_event(line: &str) -> Result<ArtifactEvent, ArtifactStorageError> {
    let fields: Vec<&str> = line.split('|').collect();
    let id = |s: &str| StableId::new(s).map_err(|_| ArtifactStorageError::Corrupt);
    let digest = |s: &str| s.parse::<Digest32>().map_err(|_| ArtifactStorageError::Corrupt);
    let number = |s: &str| s.parse::<u64>().map_err(|_| ArtifactStorageError::Corrupt);
    match fields.as_slice() {
        ["R", event, artifact, kind, generation, predecessor, content, objective, support, producer, compatibility, size] => {
            let kind = match *kind {
                "0" => ArtifactKind::Prompt,
                "1" => ArtifactKind::Policy,
                "2" => ArtifactKind::Model,
                "3" => ArtifactKind::Workflow,
                "4" => ArtifactKind::Skill,
                "5" => ArtifactKind::Parameters,
                "6" => ArtifactKind::Topology,
                "7" => ArtifactKind::Code,
                "8" => ArtifactKind::ExternalAdapter,
                _ => return Err(ArtifactStorageError::Corrupt),
            };
            Ok(ArtifactEvent::Register {
                event_id: id(event)?,
                manifest: ArtifactManifest {
                    artifact_id: id(artifact)?, kind,
                    generation: Generation::new(number(generation)?).map_err(|_| ArtifactStorageError::Corrupt)?,
                    predecessor_id: if predecessor.is_empty() { None } else { Some(id(predecessor)?) },
                    content_digest: digest(content)?, objective_digest: digest(objective)?,
                    support_digest: digest(support)?, producer_id: id(producer)?,
                    compatibility_digest: digest(compatibility)?, encoded_size_bytes: number(size)?,
                },
            })
        }
        [tag @ ("Q" | "V"), event, artifact, evaluator, reason] => {
            let change = StateChange { event_id: id(event)?, artifact_id: id(artifact)?,
                                       evaluator_id: id(evaluator)?, reason_digest: digest(reason)? };
            Ok(if *tag == "Q" { ArtifactEvent::Quarantine(change) } else { ArtifactEvent::Revoke(change) })
        }
        _ => Err(ArtifactStorageError::Corrupt),
    }
}

#[cfg(test)]
#[path = "storage_tests.rs"]
mod tests;
