use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
#[cfg(unix)]
use std::os::fd::AsRawFd;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::path::PathBuf;

const JOURNAL_VERSION: u32 = 1;
const MAX_RECORDS: usize = 4096;
#[cfg(unix)]
const LOCK_EXCLUSIVE: std::ffi::c_int = 2;
#[cfg(unix)]
const LOCK_RELEASE: std::ffi::c_int = 8;

#[cfg(unix)]
unsafe extern "C" {
    fn flock(file_descriptor: std::ffi::c_int, operation: std::ffi::c_int) -> std::ffi::c_int;
}

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct PluginMutationJournalError {
    message: String,
}

impl PluginMutationJournalError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginMutationEnvelope {
    pub request_binding: String,
    pub operation: String,
    pub target_binding: String,
    pub payload_digest: String,
    pub idempotency_binding: String,
    pub effect_plan_hash: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PluginMutationBegin {
    Planned,
    ReplayedSuccess(Value),
    ReplayedFailure(String),
    InDoubt,
}

#[derive(Debug, Clone)]
pub struct PluginMutationJournal {
    path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PluginMutationStatus {
    Planned,
    Committing,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PluginMutationRecord {
    envelope: PluginMutationEnvelope,
    status: PluginMutationStatus,
    provider_ack_hash: Option<String>,
    terminal_receipt_hash: Option<String>,
    response: Option<Value>,
    error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PluginMutationState {
    version: u32,
    generation: u64,
    records: Vec<PluginMutationRecord>,
    state_hash: String,
}

impl PluginMutationState {
    fn empty() -> Result<Self, PluginMutationJournalError> {
        let mut state = Self {
            version: JOURNAL_VERSION,
            generation: 0,
            records: Vec::new(),
            state_hash: String::new(),
        };
        state.refresh_hash()?;
        Ok(state)
    }

    fn refresh_hash(&mut self) -> Result<(), PluginMutationJournalError> {
        self.state_hash.clear();
        let encoded = serde_json::to_vec(self)
            .map_err(|error| PluginMutationJournalError::new(format!("encode journal: {error}")))?;
        self.state_hash = content_hash("hepta-plugin-mutation-journal", &[&encoded]);
        Ok(())
    }

    fn verify(&self) -> Result<(), PluginMutationJournalError> {
        if self.version != JOURNAL_VERSION {
            return Err(PluginMutationJournalError::new(format!(
                "unsupported plugin mutation journal version {}",
                self.version
            )));
        }
        let mut candidate = self.clone();
        candidate.refresh_hash()?;
        if candidate.state_hash != self.state_hash {
            return Err(PluginMutationJournalError::new(
                "plugin mutation journal integrity check failed",
            ));
        }
        Ok(())
    }
}

impl PluginMutationJournal {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn begin(
        &self,
        envelope: PluginMutationEnvelope,
    ) -> Result<PluginMutationBegin, PluginMutationJournalError> {
        validate_envelope(&envelope)?;
        self.with_locked_state(|state| {
            if let Some(record) = state
                .records
                .iter()
                .find(|record| record.envelope.request_binding == envelope.request_binding)
            {
                if record.envelope != envelope {
                    return Err(PluginMutationJournalError::new(
                        "plugin mutation request binding was reused with different authority",
                    ));
                }
                return Ok(match record.status {
                    PluginMutationStatus::Planned | PluginMutationStatus::Committing => {
                        PluginMutationBegin::InDoubt
                    }
                    PluginMutationStatus::Succeeded => PluginMutationBegin::ReplayedSuccess(
                        record.response.clone().ok_or_else(|| {
                            PluginMutationJournalError::new(
                                "succeeded plugin mutation is missing its response",
                            )
                        })?,
                    ),
                    PluginMutationStatus::Failed => PluginMutationBegin::ReplayedFailure(
                        record.error.clone().ok_or_else(|| {
                            PluginMutationJournalError::new(
                                "failed plugin mutation is missing its terminal error",
                            )
                        })?,
                    ),
                });
            }
            if state.records.len() >= MAX_RECORDS {
                compact_terminal_records(state);
            }
            if state.records.len() >= MAX_RECORDS {
                return Err(PluginMutationJournalError::new(
                    "plugin mutation journal is full with non-terminal records",
                ));
            }
            state.records.push(PluginMutationRecord {
                envelope,
                status: PluginMutationStatus::Planned,
                provider_ack_hash: None,
                terminal_receipt_hash: None,
                response: None,
                error: None,
            });
            state.generation = state.generation.saturating_add(1);
            Ok(PluginMutationBegin::Planned)
        })
    }

    pub fn mark_committing(&self, request_binding: &str) -> Result<(), PluginMutationJournalError> {
        self.update_record(request_binding, |record| {
            if record.status != PluginMutationStatus::Planned {
                return Err(PluginMutationJournalError::new(
                    "plugin mutation must be planned before committing",
                ));
            }
            record.status = PluginMutationStatus::Committing;
            Ok(())
        })
    }

    pub fn succeed(
        &self,
        request_binding: &str,
        provider_ack_hash: String,
        terminal_receipt_hash: String,
        response: Value,
    ) -> Result<(), PluginMutationJournalError> {
        require_content_hash(&provider_ack_hash, "provider ACK hash")?;
        require_content_hash(&terminal_receipt_hash, "terminal receipt hash")?;
        self.update_record(request_binding, |record| {
            if record.status != PluginMutationStatus::Committing {
                return Err(PluginMutationJournalError::new(
                    "plugin mutation must be committing before success",
                ));
            }
            record.status = PluginMutationStatus::Succeeded;
            record.provider_ack_hash = Some(provider_ack_hash);
            record.terminal_receipt_hash = Some(terminal_receipt_hash);
            record.response = Some(response);
            Ok(())
        })
    }

    pub fn fail(
        &self,
        request_binding: &str,
        provider_ack_hash: String,
        terminal_receipt_hash: String,
        error: String,
    ) -> Result<(), PluginMutationJournalError> {
        require_content_hash(&provider_ack_hash, "provider ACK hash")?;
        require_content_hash(&terminal_receipt_hash, "terminal receipt hash")?;
        if error.is_empty() {
            return Err(PluginMutationJournalError::new(
                "plugin mutation terminal error cannot be empty",
            ));
        }
        self.update_record(request_binding, |record| {
            if record.status != PluginMutationStatus::Committing {
                return Err(PluginMutationJournalError::new(
                    "plugin mutation must be committing before failure",
                ));
            }
            record.status = PluginMutationStatus::Failed;
            record.provider_ack_hash = Some(provider_ack_hash);
            record.terminal_receipt_hash = Some(terminal_receipt_hash);
            record.error = Some(error);
            Ok(())
        })
    }

    fn update_record(
        &self,
        request_binding: &str,
        update: impl FnOnce(&mut PluginMutationRecord) -> Result<(), PluginMutationJournalError>,
    ) -> Result<(), PluginMutationJournalError> {
        self.with_locked_state(|state| {
            let record = state
                .records
                .iter_mut()
                .find(|record| record.envelope.request_binding == request_binding)
                .ok_or_else(|| {
                    PluginMutationJournalError::new("plugin mutation record is missing")
                })?;
            update(record)?;
            state.generation = state.generation.saturating_add(1);
            Ok(())
        })
    }

    fn with_locked_state<T>(
        &self,
        mutate: impl FnOnce(&mut PluginMutationState) -> Result<T, PluginMutationJournalError>,
    ) -> Result<T, PluginMutationJournalError> {
        let parent = self.path.parent().ok_or_else(|| {
            PluginMutationJournalError::new("plugin mutation journal has no parent directory")
        })?;
        std::fs::create_dir_all(parent).map_err(|error| {
            PluginMutationJournalError::new(format!("create journal directory: {error}"))
        })?;
        let lock_path = self.path.with_extension("lock");
        let lock = open_private(&lock_path)?;
        lock_exclusive(&lock)?;
        let mut state = read_state(&self.path)?;
        let result = mutate(&mut state);
        if result.is_ok() {
            state.refresh_hash()?;
            publish_state(&self.path, &state)?;
        }
        unlock(&lock);
        result
    }
}

fn read_state(path: &Path) -> Result<PluginMutationState, PluginMutationJournalError> {
    let mut file = match OpenOptions::new().read(true).open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == ErrorKind::NotFound => return PluginMutationState::empty(),
        Err(error) => {
            return Err(PluginMutationJournalError::new(format!(
                "open plugin mutation journal: {error}"
            )));
        }
    };
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(|error| {
        PluginMutationJournalError::new(format!("read plugin mutation journal: {error}"))
    })?;
    let state: PluginMutationState = serde_json::from_slice(&bytes).map_err(|error| {
        PluginMutationJournalError::new(format!("decode plugin mutation journal: {error}"))
    })?;
    state.verify()?;
    Ok(state)
}

fn publish_state(
    path: &Path,
    state: &PluginMutationState,
) -> Result<(), PluginMutationJournalError> {
    let bytes = serde_json::to_vec(state).map_err(|error| {
        PluginMutationJournalError::new(format!("encode plugin mutation journal: {error}"))
    })?;
    let staging = path.with_extension(format!("tmp-{}", std::process::id()));
    let mut options = OpenOptions::new();
    options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    options.mode(0o600);
    let mut file = options.open(&staging).map_err(|error| {
        PluginMutationJournalError::new(format!("open plugin mutation staging file: {error}"))
    })?;
    file.write_all(&bytes).map_err(|error| {
        PluginMutationJournalError::new(format!("write plugin mutation staging file: {error}"))
    })?;
    file.sync_all().map_err(|error| {
        PluginMutationJournalError::new(format!("sync plugin mutation staging file: {error}"))
    })?;
    std::fs::rename(&staging, path).map_err(|error| {
        PluginMutationJournalError::new(format!("publish plugin mutation journal: {error}"))
    })?;
    if let Some(parent) = path.parent() {
        File::open(parent)
            .and_then(|directory| directory.sync_all())
            .map_err(|error| {
                PluginMutationJournalError::new(format!(
                    "sync plugin mutation journal directory: {error}"
                ))
            })?;
    }
    Ok(())
}

fn open_private(path: &Path) -> Result<File, PluginMutationJournalError> {
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true);
    #[cfg(unix)]
    options.mode(0o600);
    options.open(path).map_err(|error| {
        PluginMutationJournalError::new(format!("open plugin mutation lock: {error}"))
    })
}

#[cfg(unix)]
fn lock_exclusive(file: &File) -> Result<(), PluginMutationJournalError> {
    if unsafe { flock(file.as_raw_fd(), LOCK_EXCLUSIVE) } != 0 {
        return Err(PluginMutationJournalError::new(format!(
            "lock plugin mutation journal: {}",
            std::io::Error::last_os_error()
        )));
    }
    Ok(())
}

#[cfg(not(unix))]
fn lock_exclusive(_file: &File) -> Result<(), PluginMutationJournalError> {
    Ok(())
}

#[cfg(unix)]
fn unlock(file: &File) {
    let _ = unsafe { flock(file.as_raw_fd(), LOCK_RELEASE) };
}

#[cfg(not(unix))]
fn unlock(_file: &File) {}

fn compact_terminal_records(state: &mut PluginMutationState) {
    let terminal_count = state
        .records
        .iter()
        .filter(|record| {
            matches!(
                record.status,
                PluginMutationStatus::Succeeded | PluginMutationStatus::Failed
            )
        })
        .count();
    let remove_count = terminal_count.min(MAX_RECORDS / 2);
    let mut remaining = remove_count;
    state.records.retain(|record| {
        if remaining > 0
            && matches!(
                record.status,
                PluginMutationStatus::Succeeded | PluginMutationStatus::Failed
            )
        {
            remaining -= 1;
            false
        } else {
            true
        }
    });
}

fn validate_envelope(envelope: &PluginMutationEnvelope) -> Result<(), PluginMutationJournalError> {
    require_sha256_hex(&envelope.request_binding, "request binding")?;
    require_label(&envelope.operation, "operation")?;
    require_label(&envelope.target_binding, "target binding")?;
    require_content_hash(&envelope.payload_digest, "payload digest")?;
    require_sha256_hex(&envelope.idempotency_binding, "idempotency binding")?;
    require_content_hash(&envelope.effect_plan_hash, "effect plan hash")
}

fn require_label(value: &str, name: &str) -> Result<(), PluginMutationJournalError> {
    if value.is_empty() || value.len() > 1024 || value.chars().any(char::is_control) {
        return Err(PluginMutationJournalError::new(format!(
            "{name} must be a bounded printable value"
        )));
    }
    Ok(())
}

fn require_sha256_hex(value: &str, name: &str) -> Result<(), PluginMutationJournalError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(PluginMutationJournalError::new(format!(
            "{name} must be lowercase SHA-256 hex"
        )));
    }
    Ok(())
}

fn require_content_hash(value: &str, name: &str) -> Result<(), PluginMutationJournalError> {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return Err(PluginMutationJournalError::new(format!(
            "{name} must use the sha256 content-hash domain"
        )));
    };
    require_sha256_hex(hex, name)
}

fn content_hash(domain: &str, values: &[&[u8]]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(domain.as_bytes());
    for value in values {
        hasher.update((value.len() as u64).to_be_bytes());
        hasher.update(value);
    }
    format!("sha256:{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn envelope(request: &str) -> PluginMutationEnvelope {
        PluginMutationEnvelope {
            request_binding: request.repeat(64),
            operation: "plugin_share_save".to_string(),
            target_binding: "plugin-path".to_string(),
            payload_digest: format!("sha256:{}", "a".repeat(64)),
            idempotency_binding: "b".repeat(64),
            effect_plan_hash: format!("sha256:{}", "c".repeat(64)),
        }
    }

    #[test]
    fn replays_terminal_success_and_blocks_in_doubt() -> Result<(), PluginMutationJournalError> {
        let root = tempfile::tempdir()
            .map_err(|error| PluginMutationJournalError::new(format!("create tempdir: {error}")))?;
        let journal = PluginMutationJournal::new(root.path().join("journal.json"));
        let envelope = envelope("1");
        assert_eq!(
            journal.begin(envelope.clone())?,
            PluginMutationBegin::Planned
        );
        assert_eq!(
            journal.begin(envelope.clone())?,
            PluginMutationBegin::InDoubt
        );
        journal.mark_committing(&envelope.request_binding)?;
        journal.succeed(
            &envelope.request_binding,
            format!("sha256:{}", "d".repeat(64)),
            format!("sha256:{}", "e".repeat(64)),
            serde_json::json!({"ok": true}),
        )?;
        assert_eq!(
            journal.begin(envelope)?,
            PluginMutationBegin::ReplayedSuccess(serde_json::json!({"ok": true}))
        );
        Ok(())
    }

    #[test]
    fn rejects_request_binding_reuse_with_different_payload()
    -> Result<(), PluginMutationJournalError> {
        let root = tempfile::tempdir()
            .map_err(|error| PluginMutationJournalError::new(format!("create tempdir: {error}")))?;
        let journal = PluginMutationJournal::new(root.path().join("journal.json"));
        let first = envelope("2");
        journal.begin(first.clone())?;
        let mut conflicting = first;
        conflicting.payload_digest = format!("sha256:{}", "f".repeat(64));
        assert!(journal.begin(conflicting).is_err());
        Ok(())
    }
}
