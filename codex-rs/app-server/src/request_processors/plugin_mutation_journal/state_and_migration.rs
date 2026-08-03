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
    migration: Option<PluginMutationLayoutMigration>,
    #[cfg(test)]
    migration_fault: Option<MigrationFaultPoint>,
}

#[derive(Debug, Clone)]
struct PluginMutationLayoutMigration {
    legacy_path: PathBuf,
    marker_path: PathBuf,
    legacy_identity: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MigrationFaultPoint {
    KeyPublication,
    PreparingMarker,
    StateCopy,
    AnchorReconciliation,
    CommitMarker,
    LegacyRetirement,
}

impl MigrationFaultPoint {
    #[cfg(test)]
    const fn label(self) -> &'static str {
        match self {
            Self::KeyPublication => "key publication",
            Self::PreparingMarker => "preparing-marker publication",
            Self::StateCopy => "state publication",
            Self::AnchorReconciliation => "anchor publication",
            Self::CommitMarker => "committed-marker publication",
            Self::LegacyRetirement => "legacy-tombstone publication",
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum MigrationSource {
    Fresh,
    LegacyV1,
    LegacyV2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum MigrationPhase {
    Preparing,
    Committed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PluginMutationMigrationClaim {
    layout_version: u32,
    legacy_identity: String,
    destination: String,
    source: MigrationSource,
    generation: u64,
    state_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PluginMutationMigrationMarker {
    version: u32,
    phase: MigrationPhase,
    claim: PluginMutationMigrationClaim,
    mac: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RetiredLegacyPluginMutationJournal {
    version: u32,
    claim: PluginMutationMigrationClaim,
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

struct MigrationStores<'a> {
    active_journal: &'a AuthenticatedJournalStore,
    active_key: &'a AuthenticatedJournalStore,
    marker: &'a AuthenticatedJournalStore,
    legacy_journal: &'a LegacyJournalMigrationStore,
    legacy_key: &'a LegacyJournalMigrationStore,
    anchor: &'a AuthenticatedJournalStore,
    layout: &'a PluginMutationLayoutMigration,
}

struct MigrationSourceState {
    source: MigrationSource,
    state: PluginMutationState,
}

enum LegacyLayoutJournal {
    Missing,
    Source(MigrationSourceState),
    Retired(RetiredLegacyPluginMutationJournal),
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

impl PluginMutationMigrationClaim {
    fn for_state(
        legacy_identity: &str,
        source: MigrationSource,
        state: &PluginMutationState,
    ) -> Self {
        Self {
            layout_version: MIGRATION_LAYOUT_VERSION,
            legacy_identity: legacy_identity.to_string(),
            destination: ACTIVE_JOURNAL_RELATIVE_PATH.to_string(),
            source,
            generation: state.generation,
            state_hash: state.state_hash.clone(),
        }
    }

    fn validate(&self, expected_legacy_identity: &str) -> Result<(), PluginMutationJournalError> {
        if self.layout_version != MIGRATION_LAYOUT_VERSION
            || self.legacy_identity != expected_legacy_identity
            || self.destination != ACTIVE_JOURNAL_RELATIVE_PATH
        {
            return Err(PluginMutationJournalError::new(
                "plugin mutation layout migration claim is invalid",
            ));
        }
        require_sha256_hex(&self.legacy_identity, "legacy CODEX_HOME identity")?;
        require_content_hash(&self.state_hash, "layout migration state hash")
    }

    fn matches_state(&self, state: &PluginMutationState) -> bool {
        self.generation == state.generation && self.state_hash == state.state_hash
    }
}

impl PluginMutationMigrationMarker {
    fn new(
        phase: MigrationPhase,
        claim: PluginMutationMigrationClaim,
        key: &[u8; 32],
    ) -> Result<Self, PluginMutationJournalError> {
        let mut marker = Self {
            version: MIGRATION_MARKER_VERSION,
            phase,
            claim,
            mac: String::new(),
        };
        marker.refresh_mac(key)?;
        Ok(marker)
    }

    fn refresh_mac(&mut self, key: &[u8; 32]) -> Result<(), PluginMutationJournalError> {
        self.mac = migration_claim_mac(
            key,
            MIGRATION_MARKER_MAC_DOMAIN,
            Some(self.phase),
            &self.claim,
        )?;
        Ok(())
    }

    fn verify(
        &self,
        key: &[u8; 32],
        expected_legacy_identity: &str,
    ) -> Result<(), PluginMutationJournalError> {
        if self.version != MIGRATION_MARKER_VERSION {
            return Err(PluginMutationJournalError::new(
                "plugin mutation layout migration marker version is invalid",
            ));
        }
        self.claim.validate(expected_legacy_identity)?;
        let mut candidate = self.clone();
        candidate.refresh_mac(key)?;
        if !PLUGIN_MUTATION_JOURNAL_ENGINE.constant_time_equal(&candidate.mac, &self.mac) {
            return Err(PluginMutationJournalError::new(
                "plugin mutation layout migration marker integrity check failed",
            ));
        }
        Ok(())
    }
}

impl RetiredLegacyPluginMutationJournal {
    fn new(
        claim: PluginMutationMigrationClaim,
        key: &[u8; 32],
    ) -> Result<Self, PluginMutationJournalError> {
        let mut tombstone = Self {
            version: RETIRED_LEGACY_JOURNAL_VERSION,
            claim,
            mac: String::new(),
        };
        tombstone.refresh_mac(key)?;
        Ok(tombstone)
    }

    fn refresh_mac(&mut self, key: &[u8; 32]) -> Result<(), PluginMutationJournalError> {
        self.mac = migration_claim_mac(key, MIGRATION_TOMBSTONE_MAC_DOMAIN, None, &self.claim)?;
        Ok(())
    }

    fn verify(
        &self,
        key: &[u8; 32],
        expected_legacy_identity: &str,
    ) -> Result<(), PluginMutationJournalError> {
        if self.version != RETIRED_LEGACY_JOURNAL_VERSION {
            return Err(PluginMutationJournalError::new(
                "retired plugin mutation journal version is invalid",
            ));
        }
        self.claim.validate(expected_legacy_identity)?;
        let mut candidate = self.clone();
        candidate.refresh_mac(key)?;
        if !PLUGIN_MUTATION_JOURNAL_ENGINE.constant_time_equal(&candidate.mac, &self.mac) {
            return Err(PluginMutationJournalError::new(
                "retired plugin mutation journal integrity check failed",
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
        Self {
            path,
            anchor_path,
            migration: None,
            #[cfg(test)]
            migration_fault: None,
        }
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
        let secure_root = codex_home.join(".hepta-authority").join("plugin-mutation");
        Ok(Self {
            path: secure_root.join("journal.json"),
            anchor_path,
            migration: Some(PluginMutationLayoutMigration {
                legacy_path: codex_home.join("hepta-plugin-mutation-journal.json"),
                marker_path: secure_root.join("migration.json"),
                legacy_identity: codex_home_identity(codex_home)?,
            }),
            #[cfg(test)]
            migration_fault: None,
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

        if let Some(layout) = &self.migration {
            let legacy_journal = legacy_private_store(
                &layout.legacy_path,
                MAX_JOURNAL_BYTES,
                "hepta-plugin-mutation-legacy-journal",
            )?
            .with_lock_path(layout.legacy_path.with_extension("lock"))
            .map_err(|error| store_error("configure legacy plugin mutation journal lock", error))?;
            let legacy_key = legacy_private_store(
                &layout.legacy_path.with_extension("key"),
                MAX_KEY_BYTES,
                "hepta-plugin-mutation-legacy-key",
            )?;
            let marker_store = private_store(
                &layout.marker_path,
                MAX_MIGRATION_MARKER_BYTES,
                "hepta-plugin-mutation-layout-migration",
            )?;

            // The lock order is part of the migration protocol. Every caller
            // takes the legacy root lock before the active secure-directory
            // lock, including after migration has committed.
            let _legacy_lock = legacy_journal
                .lock()
                .map_err(|error| store_error("lock legacy plugin mutation journal", error))?;
            let _active_lock = journal_store
                .lock()
                .map_err(|error| store_error("lock plugin mutation journal", error))?;
            let stores = MigrationStores {
                active_journal: &journal_store,
                active_key: &key_store,
                marker: &marker_store,
                legacy_journal: &legacy_journal,
                legacy_key: &legacy_key,
                anchor: &anchor_store,
                layout,
            };
            let key = self.prepare_migrated_layout(&stores)?;
            return mutate_current_state(&journal_store, &anchor_store, &key, mutate);
        }

        let _lock = journal_store
            .lock()
            .map_err(|error| store_error("lock plugin mutation journal", error))?;
        let version = journal_version(&journal_store)?;
        let key = load_or_create_key(
            &key_store,
            version.is_none() || version == Some(LEGACY_JOURNAL_VERSION),
        )?;
        let mut loaded = read_state(&journal_store, &key)?;
        verify_or_initialize_anchor(&anchor_store, &loaded, &key)?;
        apply_state_mutation(
            &journal_store,
            &anchor_store,
            &key,
            &mut loaded.state,
            mutate,
        )
    }

    fn prepare_migrated_layout(
        &self,
        stores: &MigrationStores<'_>,
    ) -> Result<[u8; 32], PluginMutationJournalError> {
        let active_bytes =
            read_optional_store_bytes(stores.active_journal, "active plugin mutation journal")?;
        let marker_bytes =
            read_optional_store_bytes(stores.marker, "plugin mutation layout migration marker")?;
        let legacy_bytes = stores
            .legacy_journal
            .read()
            .map_err(|error| store_error("read legacy plugin mutation journal", error))?;
        let active_key =
            read_optional_key(stores.active_key, "active plugin mutation journal key")?;
        let legacy_key =
            read_optional_legacy_key(stores.legacy_key, "legacy plugin mutation journal key")?;
        if active_key.is_some_and(|active| legacy_key.is_some_and(|legacy| active != legacy)) {
            return Err(PluginMutationJournalError::new(
                "active and legacy plugin mutation journal keys diverged",
            ));
        }

        let legacy_version = optional_journal_version(legacy_bytes.as_deref(), "legacy journal")?;
        let key = match (active_key, legacy_key, legacy_version) {
            (Some(key), _, _) => key,
            (None, Some(key), _) => key,
            (None, None, Some(JOURNAL_VERSION)) => {
                return Err(PluginMutationJournalError::new(
                    "legacy authenticated plugin mutation journal key is missing",
                ));
            }
            (None, None, Some(RETIRED_LEGACY_JOURNAL_VERSION)) => {
                return Err(PluginMutationJournalError::new(
                    "active plugin mutation journal key is missing after layout retirement",
                ));
            }
            (None, None, _) => generate_key(),
        };
        let active_state = decode_active_layout_state(active_bytes.as_deref(), &key)?;
        let legacy = decode_legacy_layout_journal(legacy_bytes.as_deref(), &key, stores.layout)?;
        let marker = decode_migration_marker(marker_bytes.as_deref(), &key, stores.layout)?;

        if active_key.is_none() {
            if active_state.is_some()
                || marker.is_some()
                || matches!(legacy, LegacyLayoutJournal::Retired(_))
            {
                return Err(PluginMutationJournalError::new(
                    "active plugin mutation layout evidence exists without its key",
                ));
            }
            publish_key(stores.active_key, &key)?;
            self.maybe_fail_migration(MigrationFaultPoint::KeyPublication)?;
        }

        match marker {
            Some(marker) => match marker.phase {
                MigrationPhase::Preparing => {
                    self.resume_preparing_migration(stores, &key, marker, active_state, legacy)?;
                }
                MigrationPhase::Committed => {
                    self.verify_committed_migration(stores, &key, marker, active_state, legacy)?;
                }
            },
            None => {
                if active_state.is_some() || matches!(legacy, LegacyLayoutJournal::Retired(_)) {
                    return Err(PluginMutationJournalError::new(
                        "plugin mutation secure layout is missing its migration marker",
                    ));
                }
                let source = match legacy {
                    LegacyLayoutJournal::Missing => MigrationSourceState {
                        source: MigrationSource::Fresh,
                        state: PluginMutationState::empty(&key)?,
                    },
                    LegacyLayoutJournal::Source(source) => source,
                    LegacyLayoutJournal::Retired(_) => unreachable!(),
                };
                if source.source == MigrationSource::Fresh
                    && active_key.is_none()
                    && legacy_key.is_none()
                    && store_has_bytes(stores.anchor, "plugin mutation anchor")?
                {
                    return Err(PluginMutationJournalError::new(
                        "external plugin mutation anchor exists without local migration proof",
                    ));
                }
                preflight_migration_anchor(stores.anchor, &source.state, &key, source.source)?;
                let claim = PluginMutationMigrationClaim::for_state(
                    &stores.layout.legacy_identity,
                    source.source,
                    &source.state,
                );
                let preparing = PluginMutationMigrationMarker::new(
                    MigrationPhase::Preparing,
                    claim.clone(),
                    &key,
                )?;
                publish_migration_marker(stores.marker, &preparing)?;
                self.maybe_fail_migration(MigrationFaultPoint::PreparingMarker)?;
                publish_state(stores.active_journal, &source.state)?;
                self.maybe_fail_migration(MigrationFaultPoint::StateCopy)?;
                reconcile_migration_anchor(
                    stores.anchor,
                    &source.state,
                    &key,
                    source.source != MigrationSource::LegacyV2,
                )?;
                self.maybe_fail_migration(MigrationFaultPoint::AnchorReconciliation)?;
                let committed = PluginMutationMigrationMarker::new(
                    MigrationPhase::Committed,
                    claim.clone(),
                    &key,
                )?;
                publish_migration_marker(stores.marker, &committed)?;
                self.maybe_fail_migration(MigrationFaultPoint::CommitMarker)?;
                publish_legacy_tombstone(stores.legacy_journal, claim, &key)?;
                self.maybe_fail_migration(MigrationFaultPoint::LegacyRetirement)?;
            }
        }
        Ok(key)
    }

    fn resume_preparing_migration(
        &self,
        stores: &MigrationStores<'_>,
        key: &[u8; 32],
        marker: PluginMutationMigrationMarker,
        active_state: Option<PluginMutationState>,
        legacy: LegacyLayoutJournal,
    ) -> Result<(), PluginMutationJournalError> {
        let expected = state_for_migration_claim(&marker.claim, legacy, key)?;
        if !marker.claim.matches_state(&expected) {
            return Err(PluginMutationJournalError::new(
                "plugin mutation migration source diverged from its preparing marker",
            ));
        }
        if let Some(active) = active_state.as_ref()
            && (active.generation != expected.generation
                || active.state_hash != expected.state_hash)
        {
            return Err(PluginMutationJournalError::new(
                "active plugin mutation journal forked during layout migration",
            ));
        }
        preflight_migration_anchor(stores.anchor, &expected, key, marker.claim.source)?;
        if active_state.is_none() {
            publish_state(stores.active_journal, &expected)?;
            self.maybe_fail_migration(MigrationFaultPoint::StateCopy)?;
        }
        reconcile_migration_anchor(
            stores.anchor,
            &expected,
            key,
            marker.claim.source != MigrationSource::LegacyV2,
        )?;
        self.maybe_fail_migration(MigrationFaultPoint::AnchorReconciliation)?;
        let committed = PluginMutationMigrationMarker::new(
            MigrationPhase::Committed,
            marker.claim.clone(),
            key,
        )?;
        publish_migration_marker(stores.marker, &committed)?;
        self.maybe_fail_migration(MigrationFaultPoint::CommitMarker)?;
        publish_legacy_tombstone(stores.legacy_journal, marker.claim, key)?;
        self.maybe_fail_migration(MigrationFaultPoint::LegacyRetirement)
    }

    fn verify_committed_migration(
        &self,
        stores: &MigrationStores<'_>,
        key: &[u8; 32],
        marker: PluginMutationMigrationMarker,
        active_state: Option<PluginMutationState>,
        legacy: LegacyLayoutJournal,
    ) -> Result<(), PluginMutationJournalError> {
        let active = active_state.ok_or_else(|| {
            PluginMutationJournalError::new(
                "active plugin mutation journal is missing after layout migration",
            )
        })?;
        if active.generation < marker.claim.generation
            || (active.generation == marker.claim.generation
                && active.state_hash != marker.claim.state_hash)
        {
            return Err(PluginMutationJournalError::new(
                "active plugin mutation journal rolled back behind its migration marker",
            ));
        }
        reconcile_migration_anchor(stores.anchor, &active, key, false)?;
        match legacy {
            LegacyLayoutJournal::Retired(tombstone) => {
                if tombstone.claim != marker.claim {
                    return Err(PluginMutationJournalError::new(
                        "legacy plugin mutation tombstone diverged from migration marker",
                    ));
                }
            }
            LegacyLayoutJournal::Source(source) => {
                if source.source != marker.claim.source
                    || !marker.claim.matches_state(&source.state)
                {
                    return Err(PluginMutationJournalError::new(
                        "legacy plugin mutation source diverged after migration commit",
                    ));
                }
                publish_legacy_tombstone(stores.legacy_journal, marker.claim, key)?;
                self.maybe_fail_migration(MigrationFaultPoint::LegacyRetirement)?;
            }
            LegacyLayoutJournal::Missing => {
                publish_legacy_tombstone(stores.legacy_journal, marker.claim, key)?;
                self.maybe_fail_migration(MigrationFaultPoint::LegacyRetirement)?;
            }
        }
        Ok(())
    }

    #[cfg(test)]
    fn with_migration_fault(mut self, fault: MigrationFaultPoint) -> Self {
        self.migration_fault = Some(fault);
        self
    }

    fn maybe_fail_migration(
        &self,
        point: MigrationFaultPoint,
    ) -> Result<(), PluginMutationJournalError> {
        #[cfg(test)]
        if self.migration_fault == Some(point) {
            return Err(PluginMutationJournalError::new(format!(
                "injected plugin mutation layout migration crash after {}",
                point.label()
            )));
        }
        #[cfg(not(test))]
        let _ = point;
        Ok(())
    }
}
