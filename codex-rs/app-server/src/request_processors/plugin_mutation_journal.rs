use hepta_authority::AuthenticatedJournalStore;
use hepta_authority::AuthenticationFraming;
use hepta_authority::PLUGIN_MUTATION_EXTERNAL_ANCHOR_FILE_ENV;
use hepta_authority::PLUGIN_MUTATION_JOURNAL_ENGINE;
use hepta_authority::PLUGIN_MUTATION_JOURNAL_POLICY;
use hepta_authority::decode_sha256_hex;
use hepta_authority::hex_decode as decode_canonical_hex;
use hepta_authority::hex_encode;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;
use std::io::ErrorKind;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;

const JOURNAL_VERSION: u32 = 2;
const LEGACY_JOURNAL_VERSION: u32 = 1;
const ANCHOR_VERSION: u32 = 1;
const MAX_RECORDS: usize = PLUGIN_MUTATION_JOURNAL_POLICY.max_active_records;
const RETAIN_TERMINAL_RECORDS: usize = 512;
const MAX_CHECKPOINTED_TERMINALS: usize =
    PLUGIN_MUTATION_JOURNAL_POLICY.max_checkpointed_authorities;
const MAX_JOURNAL_BYTES: u64 = PLUGIN_MUTATION_JOURNAL_POLICY.max_journal_bytes;
const MAX_ANCHOR_BYTES: u64 = 64 * 1024;
const MAX_KEY_BYTES: u64 = 1024;
const MAX_TERMINAL_RESPONSE_BYTES: usize = PLUGIN_MUTATION_JOURNAL_POLICY.max_record_bytes;
const MAX_TERMINAL_ERROR_BYTES: usize = 64 * 1024;
const JOURNAL_HASH_DOMAIN: &str = "hepta-plugin-mutation-journal-v2";
const JOURNAL_MAC_DOMAIN: &[u8] = b"hepta.plugin-mutation-journal.hmac-sha256.v2";
const ANCHOR_MAC_DOMAIN: &[u8] = b"hepta.plugin-mutation-anchor.hmac-sha256.v1";
const CHECKPOINT_HASH_DOMAIN: &str = "hepta-plugin-mutation-checkpoint-v1";
const CHECKPOINT_GENESIS_HASH: &str =
    "sha256:0000000000000000000000000000000000000000000000000000000000000000";
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
    anchor_path: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PluginMutationStatus {
    Planned,
    Committing,
    Succeeded,
    Failed,
}

impl PluginMutationStatus {
    const fn is_terminal(self) -> bool {
        matches!(self, Self::Succeeded | Self::Failed)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PluginMutationRecord {
    envelope: PluginMutationEnvelope,
    status: PluginMutationStatus,
    provider_ack_hash: Option<String>,
    terminal_receipt_hash: Option<String>,
    response: Option<Value>,
    error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PluginMutationCheckpoint {
    compacted_records: u64,
    terminal_records: Vec<PluginMutationRecord>,
    history_hash: String,
}

impl Default for PluginMutationCheckpoint {
    fn default() -> Self {
        Self {
            compacted_records: 0,
            terminal_records: Vec::new(),
            history_hash: CHECKPOINT_GENESIS_HASH.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PluginMutationState {
    version: u32,
    generation: u64,
    checkpoint: PluginMutationCheckpoint,
    records: Vec<PluginMutationRecord>,
    state_hash: String,
    mac: String,
}

#[derive(Serialize)]
struct UnsignedPluginMutationState<'a> {
    version: u32,
    generation: u64,
    checkpoint: &'a PluginMutationCheckpoint,
    records: &'a [PluginMutationRecord],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LegacyPluginMutationState {
    version: u32,
    generation: u64,
    records: Vec<PluginMutationRecord>,
    state_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PluginMutationAnchor {
    version: u32,
    generation: u64,
    state_hash: String,
    mac: String,
}

#[derive(Deserialize)]
struct VersionHeader {
    version: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StateOrigin {
    New,
    LegacyMigrated,
    Current,
}

struct LoadedState {
    state: PluginMutationState,
    origin: StateOrigin,
}

impl PluginMutationState {
    fn empty(key: &[u8; 32]) -> Result<Self, PluginMutationJournalError> {
        let mut state = Self {
            version: JOURNAL_VERSION,
            generation: 0,
            checkpoint: PluginMutationCheckpoint::default(),
            records: Vec::new(),
            state_hash: String::new(),
            mac: String::new(),
        };
        state.refresh_integrity(key)?;
        Ok(state)
    }

    fn refresh_integrity(&mut self, key: &[u8; 32]) -> Result<(), PluginMutationJournalError> {
        let unsigned = UnsignedPluginMutationState {
            version: self.version,
            generation: self.generation,
            checkpoint: &self.checkpoint,
            records: &self.records,
        };
        let encoded = serde_json::to_vec(&unsigned)
            .map_err(|error| PluginMutationJournalError::new(format!("encode journal: {error}")))?;
        self.state_hash = content_hash(JOURNAL_HASH_DOMAIN, &[&encoded]);
        self.mac = hmac_hex(
            key,
            JOURNAL_MAC_DOMAIN,
            &[self.state_hash.as_bytes(), &self.generation.to_be_bytes()],
        )?;
        Ok(())
    }

    fn verify(&self, key: &[u8; 32]) -> Result<(), PluginMutationJournalError> {
        if self.version != JOURNAL_VERSION {
            return Err(PluginMutationJournalError::new(format!(
                "unsupported plugin mutation journal version {}",
                self.version
            )));
        }
        validate_checkpoint(&self.checkpoint)?;
        for record in &self.records {
            validate_record(record)?;
        }
        validate_unique_request_bindings(
            self.records.iter().chain(&self.checkpoint.terminal_records),
        )?;
        let mut candidate = self.clone();
        candidate.refresh_integrity(key)?;
        if !PLUGIN_MUTATION_JOURNAL_ENGINE
            .constant_time_equal(&candidate.state_hash, &self.state_hash)
            || !PLUGIN_MUTATION_JOURNAL_ENGINE.constant_time_equal(&candidate.mac, &self.mac)
        {
            return Err(PluginMutationJournalError::new(
                "plugin mutation journal integrity check failed",
            ));
        }
        Ok(())
    }

    fn find_record(&self, request_binding: &str) -> Option<&PluginMutationRecord> {
        self.records
            .iter()
            .find(|record| record.envelope.request_binding == request_binding)
            .or_else(|| {
                self.checkpoint
                    .terminal_records
                    .iter()
                    .find(|record| record.envelope.request_binding == request_binding)
            })
    }
}

impl LegacyPluginMutationState {
    fn refresh_hash(&mut self) -> Result<(), PluginMutationJournalError> {
        self.state_hash.clear();
        let encoded = serde_json::to_vec(self).map_err(|error| {
            PluginMutationJournalError::new(format!("encode legacy journal: {error}"))
        })?;
        self.state_hash = content_hash("hepta-plugin-mutation-journal", &[&encoded]);
        Ok(())
    }

    fn verify(&self) -> Result<(), PluginMutationJournalError> {
        if self.version != LEGACY_JOURNAL_VERSION {
            return Err(PluginMutationJournalError::new(
                "legacy plugin mutation journal version is invalid",
            ));
        }
        for record in &self.records {
            validate_record(record)?;
        }
        validate_unique_request_bindings(self.records.iter())?;
        let mut candidate = self.clone();
        candidate.refresh_hash()?;
        if !PLUGIN_MUTATION_JOURNAL_ENGINE
            .constant_time_equal(&candidate.state_hash, &self.state_hash)
        {
            return Err(PluginMutationJournalError::new(
                "legacy plugin mutation journal integrity check failed",
            ));
        }
        Ok(())
    }

    fn migrate(self, key: &[u8; 32]) -> Result<PluginMutationState, PluginMutationJournalError> {
        let mut state = PluginMutationState {
            version: JOURNAL_VERSION,
            generation: self.generation,
            checkpoint: PluginMutationCheckpoint::default(),
            records: self.records,
            state_hash: String::new(),
            mac: String::new(),
        };
        state.refresh_integrity(key)?;
        Ok(state)
    }
}

impl PluginMutationAnchor {
    fn for_state(
        state: &PluginMutationState,
        key: &[u8; 32],
    ) -> Result<Self, PluginMutationJournalError> {
        let mut anchor = Self {
            version: ANCHOR_VERSION,
            generation: state.generation,
            state_hash: state.state_hash.clone(),
            mac: String::new(),
        };
        anchor.refresh_mac(key)?;
        Ok(anchor)
    }

    fn refresh_mac(&mut self, key: &[u8; 32]) -> Result<(), PluginMutationJournalError> {
        self.mac = hmac_hex(
            key,
            ANCHOR_MAC_DOMAIN,
            &[
                &self.version.to_be_bytes(),
                &self.generation.to_be_bytes(),
                self.state_hash.as_bytes(),
            ],
        )?;
        Ok(())
    }

    fn verify(&self, key: &[u8; 32]) -> Result<(), PluginMutationJournalError> {
        if self.version != ANCHOR_VERSION {
            return Err(PluginMutationJournalError::new(
                "plugin mutation anchor version is invalid",
            ));
        }
        require_content_hash(&self.state_hash, "plugin mutation anchor state hash")?;
        let mut candidate = self.clone();
        candidate.refresh_mac(key)?;
        if !PLUGIN_MUTATION_JOURNAL_ENGINE.constant_time_equal(&candidate.mac, &self.mac) {
            return Err(PluginMutationJournalError::new(
                "plugin mutation anchor integrity check failed",
            ));
        }
        Ok(())
    }
}

impl PluginMutationJournal {
    #[cfg(test)]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let anchor_path = path.with_extension("anchor");
        Self { path, anchor_path }
    }

    pub fn for_codex_home(codex_home: &Path) -> Result<Self, PluginMutationJournalError> {
        Self::for_codex_home_with_lookup(codex_home, |name| std::env::var_os(name))
    }

    fn for_codex_home_with_lookup(
        codex_home: &Path,
        mut lookup: impl FnMut(&str) -> Option<std::ffi::OsString>,
    ) -> Result<Self, PluginMutationJournalError> {
        let configured_anchor_path = lookup(PLUGIN_MUTATION_EXTERNAL_ANCHOR_FILE_ENV)
            .filter(|value| !value.is_empty())
            .map(PathBuf::from);
        let anchor_path = match configured_anchor_path {
            Some(anchor_path) => anchor_path,
            None => default_external_anchor_path(codex_home)?,
        };
        if !anchor_path.is_absolute() {
            return Err(PluginMutationJournalError::new(format!(
                "{PLUGIN_MUTATION_EXTERNAL_ANCHOR_FILE_ENV} must be absolute"
            )));
        }
        if anchor_path.starts_with(codex_home) {
            return Err(PluginMutationJournalError::new(
                "plugin mutation external anchor must be outside CODEX_HOME",
            ));
        }
        let parent = anchor_path.parent().ok_or_else(|| {
            PluginMutationJournalError::new("plugin mutation external anchor has no parent")
        })?;
        if !parent.exists() {
            match std::fs::create_dir(parent) {
                Ok(()) => {
                    #[cfg(unix)]
                    std::fs::set_permissions(parent, std::fs::Permissions::from_mode(0o700))
                        .map_err(|error| {
                            PluginMutationJournalError::new(format!(
                                "secure plugin mutation external anchor directory: {error}"
                            ))
                        })?;
                }
                Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
                Err(error) => {
                    return Err(PluginMutationJournalError::new(format!(
                        "create plugin mutation external anchor directory: {error}"
                    )));
                }
            }
        }
        let metadata = std::fs::symlink_metadata(parent).map_err(|error| {
            PluginMutationJournalError::new(format!(
                "inspect plugin mutation external anchor directory: {error}"
            ))
        })?;
        if !metadata.file_type().is_dir() {
            return Err(PluginMutationJournalError::new(
                "plugin mutation external anchor parent must be a directory",
            ));
        }
        #[cfg(unix)]
        if metadata.permissions().mode() & 0o077 != 0 {
            return Err(PluginMutationJournalError::new(
                "plugin mutation external anchor directory permissions must be private",
            ));
        }
        Ok(Self {
            path: codex_home.join("hepta-plugin-mutation-journal.json"),
            anchor_path,
        })
    }

    pub fn begin(
        &self,
        envelope: PluginMutationEnvelope,
    ) -> Result<PluginMutationBegin, PluginMutationJournalError> {
        validate_envelope(&envelope)?;
        self.with_locked_state(|state| {
            if let Some(record) = state.find_record(&envelope.request_binding) {
                if record.envelope != envelope {
                    return Err(PluginMutationJournalError::new(
                        "plugin mutation request binding was reused with different authority",
                    ));
                }
                return replay_record(record);
            }
            if state.records.len() >= MAX_RECORDS {
                compact_terminal_records(state, RETAIN_TERMINAL_RECORDS)?;
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
            state.generation = state.generation.checked_add(1).ok_or_else(|| {
                PluginMutationJournalError::new("plugin mutation generation exhausted")
            })?;
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
        let response_bytes = serde_json::to_vec(&response).map_err(|error| {
            PluginMutationJournalError::new(format!(
                "encode plugin mutation terminal response: {error}"
            ))
        })?;
        if response_bytes.len() > MAX_TERMINAL_RESPONSE_BYTES {
            return Err(PluginMutationJournalError::new(format!(
                "plugin mutation terminal response exceeds {MAX_TERMINAL_RESPONSE_BYTES} bytes"
            )));
        }
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
        if error.len() > MAX_TERMINAL_ERROR_BYTES {
            return Err(PluginMutationJournalError::new(format!(
                "plugin mutation terminal error exceeds {MAX_TERMINAL_ERROR_BYTES} bytes"
            )));
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
                    PluginMutationJournalError::new(
                        "plugin mutation record is missing or already checkpointed",
                    )
                })?;
            update(record)?;
            state.generation = state.generation.checked_add(1).ok_or_else(|| {
                PluginMutationJournalError::new("plugin mutation generation exhausted")
            })?;
            Ok(())
        })
    }

    fn with_locked_state<T>(
        &self,
        mutate: impl FnOnce(&mut PluginMutationState) -> Result<T, PluginMutationJournalError>,
    ) -> Result<T, PluginMutationJournalError> {
        let journal_store = private_store(
            &self.path,
            MAX_JOURNAL_BYTES,
            "hepta-plugin-mutation-journal",
        )?
        .with_lock_path(self.path.with_extension("lock"))
        .map_err(|error| store_error("configure plugin mutation journal lock", error))?;
        let anchor_store = private_store(
            &self.anchor_path,
            MAX_ANCHOR_BYTES,
            "hepta-plugin-mutation-anchor",
        )?;
        let key_store = private_store(
            &self.path.with_extension("key"),
            MAX_KEY_BYTES,
            "hepta-plugin-mutation-key",
        )?;
        let _lock = journal_store
            .lock()
            .map_err(|error| store_error("lock plugin mutation journal", error))?;
        let result = (|| {
            let version = journal_version(&journal_store)?;
            let key = load_or_create_key(
                &key_store,
                version.is_none() || version == Some(LEGACY_JOURNAL_VERSION),
            )?;
            let mut loaded = read_state(&journal_store, &key)?;
            verify_or_initialize_anchor(&anchor_store, &loaded, &key)?;
            let mutation_result = mutate(&mut loaded.state);
            if mutation_result.is_ok() {
                validate_state(&loaded.state)?;
                loaded.state.refresh_integrity(&key)?;
                publish_state(&journal_store, &loaded.state)?;
                publish_anchor(&anchor_store, &loaded.state, &key)?;
            }
            mutation_result
        })();
        result
    }
}

fn default_external_anchor_path(codex_home: &Path) -> Result<PathBuf, PluginMutationJournalError> {
    if !codex_home.is_absolute() {
        return Err(PluginMutationJournalError::new(
            "CODEX_HOME must be absolute for the external authority anchor",
        ));
    }
    let parent = codex_home.parent().ok_or_else(|| {
        PluginMutationJournalError::new(
            "CODEX_HOME has no parent for the external authority anchor",
        )
    })?;
    let identity = Sha256::digest(codex_home.as_os_str().to_string_lossy().as_bytes());
    let identity = identity
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    Ok(parent
        .join(".hepta-authority")
        .join(format!("plugin-mutation-{identity}.anchor")))
}

fn replay_record(
    record: &PluginMutationRecord,
) -> Result<PluginMutationBegin, PluginMutationJournalError> {
    Ok(match record.status {
        PluginMutationStatus::Planned | PluginMutationStatus::Committing => {
            PluginMutationBegin::InDoubt
        }
        PluginMutationStatus::Succeeded => {
            PluginMutationBegin::ReplayedSuccess(record.response.clone().ok_or_else(|| {
                PluginMutationJournalError::new("succeeded plugin mutation is missing its response")
            })?)
        }
        PluginMutationStatus::Failed => {
            PluginMutationBegin::ReplayedFailure(record.error.clone().ok_or_else(|| {
                PluginMutationJournalError::new(
                    "failed plugin mutation is missing its terminal error",
                )
            })?)
        }
    })
}

fn journal_version(
    store: &AuthenticatedJournalStore,
) -> Result<Option<u32>, PluginMutationJournalError> {
    let Some(bytes) = store
        .read()
        .map_err(|error| store_error("read plugin mutation journal header", error))?
    else {
        return Ok(None);
    };
    let header: VersionHeader = serde_json::from_slice(&bytes).map_err(|error| {
        PluginMutationJournalError::new(format!("decode plugin mutation journal header: {error}"))
    })?;
    Ok(Some(header.version))
}

fn read_state(
    store: &AuthenticatedJournalStore,
    key: &[u8; 32],
) -> Result<LoadedState, PluginMutationJournalError> {
    let Some(bytes) = store
        .read()
        .map_err(|error| store_error("read plugin mutation journal", error))?
    else {
        return Ok(LoadedState {
            state: PluginMutationState::empty(key)?,
            origin: StateOrigin::New,
        });
    };
    let header: VersionHeader = serde_json::from_slice(&bytes).map_err(|error| {
        PluginMutationJournalError::new(format!("decode plugin mutation journal header: {error}"))
    })?;
    match header.version {
        JOURNAL_VERSION => {
            let state: PluginMutationState = serde_json::from_slice(&bytes).map_err(|error| {
                PluginMutationJournalError::new(format!("decode plugin mutation journal: {error}"))
            })?;
            state.verify(key)?;
            Ok(LoadedState {
                state,
                origin: StateOrigin::Current,
            })
        }
        LEGACY_JOURNAL_VERSION => {
            let state: LegacyPluginMutationState =
                serde_json::from_slice(&bytes).map_err(|error| {
                    PluginMutationJournalError::new(format!(
                        "decode legacy plugin mutation journal: {error}"
                    ))
                })?;
            state.verify()?;
            Ok(LoadedState {
                state: state.migrate(key)?,
                origin: StateOrigin::LegacyMigrated,
            })
        }
        version => Err(PluginMutationJournalError::new(format!(
            "unsupported plugin mutation journal version {version}"
        ))),
    }
}

fn verify_or_initialize_anchor(
    store: &AuthenticatedJournalStore,
    loaded: &LoadedState,
    key: &[u8; 32],
) -> Result<(), PluginMutationJournalError> {
    let anchor = match store
        .read()
        .map_err(|error| store_error("read plugin mutation anchor", error))?
    {
        Some(bytes) => Some(
            serde_json::from_slice::<PluginMutationAnchor>(&bytes).map_err(|error| {
                PluginMutationJournalError::new(format!("decode plugin anchor: {error}"))
            })?,
        ),
        None => None,
    };
    let Some(anchor) = anchor else {
        if loaded.origin == StateOrigin::Current {
            return Err(PluginMutationJournalError::new(
                "plugin mutation anchor is missing for an authenticated journal",
            ));
        }
        return publish_anchor(store, &loaded.state, key);
    };
    anchor.verify(key)?;
    if anchor.generation > loaded.state.generation {
        return Err(PluginMutationJournalError::new(
            "plugin mutation journal rollback detected",
        ));
    }
    if anchor.generation == loaded.state.generation && anchor.state_hash != loaded.state.state_hash
    {
        return Err(PluginMutationJournalError::new(
            "plugin mutation journal same-generation fork detected",
        ));
    }
    if anchor.generation < loaded.state.generation {
        publish_anchor(store, &loaded.state, key)?;
    }
    Ok(())
}

fn publish_state(
    store: &AuthenticatedJournalStore,
    state: &PluginMutationState,
) -> Result<(), PluginMutationJournalError> {
    let bytes = serde_json::to_vec(state).map_err(|error| {
        PluginMutationJournalError::new(format!("encode plugin mutation journal: {error}"))
    })?;
    store
        .publish(&bytes)
        .map_err(|error| store_error("publish plugin mutation journal", error))
}

fn publish_anchor(
    store: &AuthenticatedJournalStore,
    state: &PluginMutationState,
    key: &[u8; 32],
) -> Result<(), PluginMutationJournalError> {
    let anchor = PluginMutationAnchor::for_state(state, key)?;
    let bytes = serde_json::to_vec(&anchor).map_err(|error| {
        PluginMutationJournalError::new(format!("encode plugin mutation anchor: {error}"))
    })?;
    store
        .publish(&bytes)
        .map_err(|error| store_error("publish plugin mutation anchor", error))
}

fn load_or_create_key(
    store: &AuthenticatedJournalStore,
    allow_create: bool,
) -> Result<[u8; 32], PluginMutationJournalError> {
    match store
        .read()
        .map_err(|error| store_error("read plugin mutation journal key", error))?
    {
        Some(bytes) => decode_key(&bytes),
        None if allow_create => {
            let key = generate_key();
            store
                .publish(hex_encode(&key).as_bytes())
                .map_err(|error| store_error("publish plugin mutation journal key", error))?;
            Ok(key)
        }
        None => Err(PluginMutationJournalError::new(
            "plugin mutation journal key is missing for an authenticated journal",
        )),
    }
}

fn generate_key() -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"hepta.plugin-mutation-journal.generated-key.v1");
    for _ in 0..4 {
        hasher.update(uuid::Uuid::now_v7().as_bytes());
    }
    hasher.finalize().into()
}

fn decode_key(bytes: &[u8]) -> Result<[u8; 32], PluginMutationJournalError> {
    let text = std::str::from_utf8(bytes)
        .map_err(|error| PluginMutationJournalError::new(format!("decode journal key: {error}")))?;
    let decoded = hex_decode(text.trim())?;
    decoded.try_into().map_err(|_| {
        PluginMutationJournalError::new("plugin mutation journal key must be 32 bytes")
    })
}

fn private_store(
    path: &Path,
    max_bytes: u64,
    staging_prefix: &str,
) -> Result<AuthenticatedJournalStore, PluginMutationJournalError> {
    AuthenticatedJournalStore::new(path, max_bytes, staging_prefix)
        .map_err(|error| store_error("configure authenticated journal store", error))
}

fn store_error(context: &str, error: anyhow::Error) -> PluginMutationJournalError {
    PluginMutationJournalError::new(format!("{context}: {error:#}"))
}

#[cfg(test)]
fn read_private_bytes(path: &Path, label: &str) -> Result<Vec<u8>, PluginMutationJournalError> {
    private_store(
        path,
        private_file_limit(label),
        "hepta-plugin-mutation-test",
    )?
    .read()
    .map_err(|error| store_error(&format!("read {label}"), error))?
    .ok_or_else(|| PluginMutationJournalError::new(format!("{label} does not exist")))
}

#[cfg(test)]
fn publish_private_bytes(
    path: &Path,
    bytes: &[u8],
    label: &str,
) -> Result<(), PluginMutationJournalError> {
    private_store(
        path,
        private_file_limit(label),
        "hepta-plugin-mutation-test",
    )?
    .publish(bytes)
    .map_err(|error| store_error(&format!("publish {label}"), error))
}

#[cfg(test)]
fn private_file_limit(label: &str) -> u64 {
    match label {
        "plugin mutation anchor" => MAX_ANCHOR_BYTES,
        "plugin mutation journal key" => MAX_KEY_BYTES,
        _ => MAX_JOURNAL_BYTES,
    }
}

fn compact_terminal_records(
    state: &mut PluginMutationState,
    retain: usize,
) -> Result<(), PluginMutationJournalError> {
    let terminal_count = state
        .records
        .iter()
        .filter(|record| record.status.is_terminal())
        .count();
    let mut remaining = terminal_count.saturating_sub(retain);
    if remaining == 0 {
        return Ok(());
    }
    if state
        .checkpoint
        .terminal_records
        .len()
        .saturating_add(remaining)
        > MAX_CHECKPOINTED_TERMINALS
    {
        return Err(PluginMutationJournalError::new(
            "plugin mutation checkpoint terminal limit reached",
        ));
    }
    let mut retained = Vec::with_capacity(state.records.len() - remaining);
    for record in state.records.drain(..) {
        if remaining > 0 && record.status.is_terminal() {
            state.checkpoint.history_hash =
                checkpoint_history_hash(&state.checkpoint.history_hash, &record)?;
            state.checkpoint.terminal_records.push(record);
            state.checkpoint.compacted_records = state
                .checkpoint
                .compacted_records
                .checked_add(1)
                .ok_or_else(|| {
                    PluginMutationJournalError::new("plugin mutation checkpoint count exhausted")
                })?;
            remaining -= 1;
        } else {
            retained.push(record);
        }
    }
    state.records = retained;
    Ok(())
}

fn checkpoint_history_hash(
    previous: &str,
    record: &PluginMutationRecord,
) -> Result<String, PluginMutationJournalError> {
    let encoded = serde_json::to_vec(record).map_err(|error| {
        PluginMutationJournalError::new(format!("encode compacted plugin mutation: {error}"))
    })?;
    Ok(content_hash(
        CHECKPOINT_HASH_DOMAIN,
        &[previous.as_bytes(), &encoded],
    ))
}

fn validate_state(state: &PluginMutationState) -> Result<(), PluginMutationJournalError> {
    if state.version != JOURNAL_VERSION
        || PLUGIN_MUTATION_JOURNAL_ENGINE
            .validate_counts(state.records.len(), state.checkpoint.terminal_records.len())
            .is_err()
    {
        return Err(PluginMutationJournalError::new(
            "plugin mutation journal bounds are invalid",
        ));
    }
    validate_checkpoint(&state.checkpoint)?;
    for record in &state.records {
        validate_record(record)?;
    }
    validate_unique_request_bindings(
        state
            .records
            .iter()
            .chain(&state.checkpoint.terminal_records),
    )
}

fn validate_checkpoint(
    checkpoint: &PluginMutationCheckpoint,
) -> Result<(), PluginMutationJournalError> {
    if checkpoint.compacted_records != checkpoint.terminal_records.len() as u64 {
        return Err(PluginMutationJournalError::new(
            "plugin mutation checkpoint count is invalid",
        ));
    }
    require_content_hash(&checkpoint.history_hash, "checkpoint history hash")?;
    for record in &checkpoint.terminal_records {
        validate_record(record)?;
        if !record.status.is_terminal() {
            return Err(PluginMutationJournalError::new(
                "plugin mutation checkpoint contains a non-terminal record",
            ));
        }
    }
    Ok(())
}

fn validate_unique_request_bindings<'a>(
    records: impl Iterator<Item = &'a PluginMutationRecord>,
) -> Result<(), PluginMutationJournalError> {
    let mut bindings = std::collections::HashSet::new();
    for record in records {
        if !bindings.insert(record.envelope.request_binding.as_str()) {
            return Err(PluginMutationJournalError::new(
                "plugin mutation journal contains duplicate request bindings",
            ));
        }
    }
    Ok(())
}

fn validate_record(record: &PluginMutationRecord) -> Result<(), PluginMutationJournalError> {
    validate_envelope(&record.envelope)?;
    match record.status {
        PluginMutationStatus::Planned | PluginMutationStatus::Committing => {
            if record.provider_ack_hash.is_some()
                || record.terminal_receipt_hash.is_some()
                || record.response.is_some()
                || record.error.is_some()
            {
                return Err(PluginMutationJournalError::new(
                    "non-terminal plugin mutation contains terminal data",
                ));
            }
        }
        PluginMutationStatus::Succeeded => {
            validate_terminal_hashes(record)?;
            if record.response.is_none() || record.error.is_some() {
                return Err(PluginMutationJournalError::new(
                    "succeeded plugin mutation terminal data is invalid",
                ));
            }
        }
        PluginMutationStatus::Failed => {
            validate_terminal_hashes(record)?;
            if record.error.as_deref().is_none_or(str::is_empty) || record.response.is_some() {
                return Err(PluginMutationJournalError::new(
                    "failed plugin mutation terminal data is invalid",
                ));
            }
        }
    }
    Ok(())
}

fn validate_terminal_hashes(
    record: &PluginMutationRecord,
) -> Result<(), PluginMutationJournalError> {
    require_content_hash(
        record.provider_ack_hash.as_deref().ok_or_else(|| {
            PluginMutationJournalError::new("terminal plugin mutation is missing provider ACK")
        })?,
        "provider ACK hash",
    )?;
    require_content_hash(
        record.terminal_receipt_hash.as_deref().ok_or_else(|| {
            PluginMutationJournalError::new("terminal plugin mutation is missing receipt hash")
        })?,
        "terminal receipt hash",
    )
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
    if decode_sha256_hex(value).is_err() {
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
    PLUGIN_MUTATION_JOURNAL_ENGINE.content_hash(
        AuthenticationFraming::RawDomain,
        domain.as_bytes(),
        values,
    )
}

fn hmac_hex(
    key: &[u8; 32],
    domain: &[u8],
    values: &[&[u8]],
) -> Result<String, PluginMutationJournalError> {
    PLUGIN_MUTATION_JOURNAL_ENGINE
        .mac_hex(key, AuthenticationFraming::RawDomain, domain, values)
        .map_err(|error| {
            PluginMutationJournalError::new(format!("initialize plugin mutation HMAC: {error}"))
        })
}

fn hex_decode(value: &str) -> Result<Vec<u8>, PluginMutationJournalError> {
    decode_canonical_hex(value)
        .map_err(|error| PluginMutationJournalError::new(format!("decode hex value: {error}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn envelope_index(index: u64) -> PluginMutationEnvelope {
        PluginMutationEnvelope {
            request_binding: format!("{index:064x}"),
            operation: "plugin_share_save".to_string(),
            target_binding: "plugin-path".to_string(),
            payload_digest: format!("sha256:{}", "a".repeat(64)),
            idempotency_binding: format!("{:064x}", index.saturating_add(10_000)),
            effect_plan_hash: format!("sha256:{}", "c".repeat(64)),
        }
    }

    fn terminal_record(index: u64, status: PluginMutationStatus) -> PluginMutationRecord {
        let (response, error) = match status {
            PluginMutationStatus::Succeeded => (Some(serde_json::json!({"index": index})), None),
            PluginMutationStatus::Failed => (None, Some(format!("failure-{index}"))),
            _ => (None, None),
        };
        PluginMutationRecord {
            envelope: envelope_index(index),
            status,
            provider_ack_hash: status
                .is_terminal()
                .then(|| format!("sha256:{}", "d".repeat(64))),
            terminal_receipt_hash: status
                .is_terminal()
                .then(|| format!("sha256:{}", "e".repeat(64))),
            response,
            error,
        }
    }

    #[test]
    fn replays_terminal_success_and_blocks_in_doubt() -> Result<(), PluginMutationJournalError> {
        let root = tempfile::tempdir()
            .map_err(|error| PluginMutationJournalError::new(format!("create tempdir: {error}")))?;
        let journal = PluginMutationJournal::new(root.path().join("journal.json"));
        let envelope = envelope_index(1);
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
        let first = envelope_index(2);
        journal.begin(first.clone())?;
        let mut conflicting = first;
        conflicting.payload_digest = format!("sha256:{}", "f".repeat(64));
        assert!(journal.begin(conflicting).is_err());
        Ok(())
    }

    #[test]
    fn checkpoint_preserves_success_and_failure_replay_after_record_limit()
    -> Result<(), PluginMutationJournalError> {
        let root = tempfile::tempdir()
            .map_err(|error| PluginMutationJournalError::new(format!("create tempdir: {error}")))?;
        let path = root.path().join("journal.json");
        let journal = PluginMutationJournal::new(&path);
        journal.begin(envelope_index(1))?;
        journal.with_locked_state(|state| {
            state.records = (1..=MAX_RECORDS as u64)
                .map(|index| {
                    terminal_record(
                        index,
                        if index == 2 {
                            PluginMutationStatus::Failed
                        } else {
                            PluginMutationStatus::Succeeded
                        },
                    )
                })
                .collect();
            state.generation = state.generation.saturating_add(1);
            Ok(())
        })?;
        assert_eq!(
            journal.begin(envelope_index(MAX_RECORDS as u64 + 1))?,
            PluginMutationBegin::Planned
        );
        assert_eq!(
            journal.begin(envelope_index(1))?,
            PluginMutationBegin::ReplayedSuccess(serde_json::json!({"index": 1}))
        );
        assert_eq!(
            journal.begin(envelope_index(2))?,
            PluginMutationBegin::ReplayedFailure("failure-2".to_string())
        );
        let state: PluginMutationState =
            serde_json::from_slice(&read_private_bytes(&path, "plugin mutation journal")?)
                .map_err(|error| {
                    PluginMutationJournalError::new(format!("decode state: {error}"))
                })?;
        assert_eq!(
            state.checkpoint.compacted_records,
            (MAX_RECORDS - RETAIN_TERMINAL_RECORDS) as u64
        );
        Ok(())
    }

    #[test]
    fn rejects_authenticated_journal_rollback() -> Result<(), PluginMutationJournalError> {
        let root = tempfile::tempdir()
            .map_err(|error| PluginMutationJournalError::new(format!("create tempdir: {error}")))?;
        let path = root.path().join("journal.json");
        let journal = PluginMutationJournal::new(&path);
        journal.begin(envelope_index(1))?;
        let old_state = read_private_bytes(&path, "plugin mutation journal")?;
        journal.begin(envelope_index(2))?;
        publish_private_bytes(&path, &old_state, "plugin mutation journal")?;
        let error = journal
            .begin(envelope_index(3))
            .expect_err("rollback must fail closed");
        assert!(error.to_string().contains("rollback detected"));
        Ok(())
    }

    #[test]
    fn migrates_verified_v1_state_and_creates_anchor() -> Result<(), PluginMutationJournalError> {
        let root = tempfile::tempdir()
            .map_err(|error| PluginMutationJournalError::new(format!("create tempdir: {error}")))?;
        let path = root.path().join("journal.json");
        let mut legacy = LegacyPluginMutationState {
            version: LEGACY_JOURNAL_VERSION,
            generation: 7,
            records: vec![terminal_record(7, PluginMutationStatus::Succeeded)],
            state_hash: String::new(),
        };
        legacy.refresh_hash()?;
        publish_private_bytes(
            &path,
            &serde_json::to_vec(&legacy).map_err(|error| {
                PluginMutationJournalError::new(format!("encode legacy state: {error}"))
            })?,
            "plugin mutation journal",
        )?;
        let journal = PluginMutationJournal::new(&path);
        assert_eq!(
            journal.begin(envelope_index(7))?,
            PluginMutationBegin::ReplayedSuccess(serde_json::json!({"index": 7}))
        );
        let state: PluginMutationState =
            serde_json::from_slice(&read_private_bytes(&path, "plugin mutation journal")?)
                .map_err(|error| {
                    PluginMutationJournalError::new(format!("decode v2 state: {error}"))
                })?;
        assert_eq!(state.version, JOURNAL_VERSION);
        assert!(path.with_extension("key").is_file());
        assert!(path.with_extension("anchor").is_file());
        Ok(())
    }

    #[test]
    fn production_journal_places_anchor_outside_codex_home()
    -> Result<(), PluginMutationJournalError> {
        let root = tempfile::tempdir()
            .map_err(|error| PluginMutationJournalError::new(format!("create tempdir: {error}")))?;
        let codex_home = root.path().join("codex-home");
        std::fs::create_dir(&codex_home).map_err(|error| {
            PluginMutationJournalError::new(format!("create CODEX_HOME: {error}"))
        })?;
        #[cfg(unix)]
        std::fs::set_permissions(&codex_home, std::fs::Permissions::from_mode(0o700)).map_err(
            |error| PluginMutationJournalError::new(format!("secure CODEX_HOME: {error}")),
        )?;
        let journal = PluginMutationJournal::for_codex_home_with_lookup(&codex_home, |_| None)?;
        assert!(!journal.anchor_path.starts_with(&codex_home));
        let second_codex_home = root.path().join("second-codex-home");
        std::fs::create_dir(&second_codex_home).map_err(|error| {
            PluginMutationJournalError::new(format!("create second CODEX_HOME: {error}"))
        })?;
        let second =
            PluginMutationJournal::for_codex_home_with_lookup(&second_codex_home, |_| None)?;
        assert_ne!(journal.anchor_path, second.anchor_path);
        journal.begin(envelope_index(1))?;
        assert!(journal.path.is_file());
        assert!(journal.anchor_path.is_file());
        Ok(())
    }

    #[test]
    fn external_anchor_rejects_whole_codex_home_rollback() -> Result<(), PluginMutationJournalError>
    {
        let root = tempfile::tempdir()
            .map_err(|error| PluginMutationJournalError::new(format!("create tempdir: {error}")))?;
        let codex_home = root.path().join("codex-home");
        std::fs::create_dir(&codex_home).map_err(|error| {
            PluginMutationJournalError::new(format!("create CODEX_HOME: {error}"))
        })?;
        #[cfg(unix)]
        std::fs::set_permissions(&codex_home, std::fs::Permissions::from_mode(0o700)).map_err(
            |error| PluginMutationJournalError::new(format!("secure CODEX_HOME: {error}")),
        )?;
        let journal = PluginMutationJournal::for_codex_home_with_lookup(&codex_home, |_| None)?;
        journal.begin(envelope_index(1))?;
        let old_state = read_private_bytes(&journal.path, "plugin mutation journal")?;
        journal.begin(envelope_index(2))?;
        publish_private_bytes(&journal.path, &old_state, "plugin mutation journal")?;
        let error = journal
            .begin(envelope_index(3))
            .expect_err("external anchor must reject CODEX_HOME rollback");
        assert!(error.to_string().contains("rollback detected"));
        Ok(())
    }

    #[test]
    fn terminal_payloads_are_bounded() -> Result<(), PluginMutationJournalError> {
        let root = tempfile::tempdir()
            .map_err(|error| PluginMutationJournalError::new(format!("create tempdir: {error}")))?;
        let journal = PluginMutationJournal::new(root.path().join("journal.json"));
        let envelope = envelope_index(1);
        journal.begin(envelope.clone())?;
        journal.mark_committing(&envelope.request_binding)?;
        let response = serde_json::json!({"payload": "x".repeat(MAX_TERMINAL_RESPONSE_BYTES)});
        let error = journal
            .succeed(
                &envelope.request_binding,
                format!("sha256:{}", "d".repeat(64)),
                format!("sha256:{}", "e".repeat(64)),
                response,
            )
            .expect_err("oversized terminal response must fail closed");
        assert!(error.to_string().contains("terminal response exceeds"));
        Ok(())
    }
}
