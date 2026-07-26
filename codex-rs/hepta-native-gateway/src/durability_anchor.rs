use std::env;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;

use anyhow::Context;
use anyhow::Result;
use hepta_memory::DurableMonotonicState;
use hepta_memory::RuntimeStateMonotonicState;
use hmac::Hmac;
use hmac::Mac;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
use zeroize::Zeroizing;

use crate::operator_mutation_journal::OperatorMutationMonotonicState;
use crate::secure_key_file::read_private_key;
use crate::telegram_authority::TelegramAuthorityMonotonicState;
use crate::telegram_durable_files::update_private_state_atomically;

pub(crate) const MONOTONIC_ANCHOR_FILE_ENV: &str = "HEPTA_MONOTONIC_ANCHOR_FILE";
pub(crate) const MONOTONIC_ANCHOR_KEY_FILE_ENV: &str = "HEPTA_MONOTONIC_ANCHOR_KEY_FILE";
const ANCHOR_SCHEMA: &str = "hepta.external-monotonic-anchor.v2";
const ANCHOR_MAC_DOMAIN: &[u8] = b"hepta.external-monotonic-anchor.hmac-sha256.v2";
const ANCHOR_ENTRY_HASH_DOMAIN: &[u8] = b"hepta.external-monotonic-anchor.entry-sha256.v2";
const ANCHOR_ROTATION_HISTORY_DOMAIN: &[u8] =
    b"hepta.external-monotonic-anchor.rotation-history-sha256.v1";
const EFFECT_LEASE_SOURCE_HASH_DOMAIN: &[u8] =
    b"hepta.external-monotonic-anchor.effect-lease-source-sha256.v1";
const TELEGRAM_STATE_HASH_DOMAIN: &[u8] =
    b"hepta.external-monotonic-anchor.telegram-state-sha256.v1";
const GENESIS_PREVIOUS_HASH: &str = "sha256:anchor-genesis";
const ROTATION_GENESIS_HASH: &str = "sha256:anchor-rotation-genesis";
const MAX_ANCHOR_BYTES: u64 = 16 * 1024 * 1024;
const MAX_ANCHOR_RECORDS: usize = 4096;
const MAX_ANCHOR_LINE_BYTES: usize = 4096;
const ANCHOR_STAGING_PREFIX: &str = ".hepta-external-monotonic-anchor";
type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub(crate) struct ExternalMonotonicAnchorConfig {
    path: PathBuf,
    key_file: PathBuf,
}

impl ExternalMonotonicAnchorConfig {
    pub(crate) fn from_env() -> Result<Option<Self>> {
        Self::from_lookup(|name| env::var_os(name))
    }

    fn from_lookup(mut lookup: impl FnMut(&str) -> Option<OsString>) -> Result<Option<Self>> {
        let path = lookup(MONOTONIC_ANCHOR_FILE_ENV).filter(|value| !value.is_empty());
        let key_file = lookup(MONOTONIC_ANCHOR_KEY_FILE_ENV).filter(|value| !value.is_empty());
        match (path, key_file) {
            (None, None) => Ok(None),
            (Some(path), Some(key_file)) => {
                let path = PathBuf::from(path);
                let key_file = PathBuf::from(key_file);
                if !path.is_absolute() || !key_file.is_absolute() {
                    anyhow::bail!("external monotonic anchor paths must be absolute");
                }
                if path == key_file {
                    anyhow::bail!("external monotonic anchor and key paths must be distinct");
                }
                Ok(Some(Self { path, key_file }))
            }
            _ => anyhow::bail!(
                "{MONOTONIC_ANCHOR_FILE_ENV} and {MONOTONIC_ANCHOR_KEY_FILE_ENV} must be configured together"
            ),
        }
    }

    #[cfg(all(test, unix))]
    pub(crate) fn for_runtime_test(root: &Path) -> Result<Self> {
        use std::os::unix::fs::PermissionsExt;

        let key_file = root.join("anchor.key");
        fs::write(
            &key_file,
            b"606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f",
        )?;
        fs::set_permissions(&key_file, fs::Permissions::from_mode(0o600))?;
        Ok(Self {
            path: root.join("monotonic.anchor"),
            key_file,
        })
    }
}

pub(crate) struct ExternalMonotonicAnchor {
    path: PathBuf,
    key: Zeroizing<[u8; 32]>,
    operation: Mutex<AnchorOperationState>,
    operation_idle: Condvar,
}

#[derive(Debug, Clone)]
pub(crate) struct DurableAnchorStateSnapshot {
    pub(crate) outcome: DurableMonotonicState,
    pub(crate) preference: DurableMonotonicState,
    pub(crate) telegram: Option<TelegramAuthorityMonotonicState>,
    pub(crate) operator: Option<OperatorMutationMonotonicState>,
    pub(crate) runtime_state: Option<RuntimeStateMonotonicState>,
}

#[derive(Debug, Clone)]
struct AnchorSnapshot {
    entries: Vec<AnchorEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum EffectLeaseRecordState {
    Pending,
    Finalized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AnchorEntry {
    schema: String,
    sequence: u64,
    previous_entry_hash: String,
    outcome_generation: u64,
    outcome_state_hash: String,
    preference_generation: u64,
    preference_state_hash: String,
    telegram_generation: Option<u64>,
    telegram_state_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    operator_generation: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    operator_state_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    runtime_state_generation: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    runtime_state_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    effect_lease_state: Option<EffectLeaseRecordState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    effect_lease_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    effect_lease_source_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    rotation_compacted_records: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    rotation_history_hash: Option<String>,
    mac: String,
}

#[derive(Debug, Default)]
struct AnchorOperationState {
    fault: Option<String>,
    effect_lease_active: bool,
}

#[derive(Debug)]
pub(crate) struct ExternalMonotonicAnchorBusy;

impl std::fmt::Display for ExternalMonotonicAnchorBusy {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("external monotonic anchor effect lease is active in another process")
    }
}

impl std::error::Error for ExternalMonotonicAnchorBusy {}

pub(crate) struct ExternalMonotonicAnchorEffectLease {
    anchor: Arc<ExternalMonotonicAnchor>,
    effect_lock: Option<File>,
    lease_id: String,
    source_hash: String,
    active: bool,
}

impl ExternalMonotonicAnchor {
    pub(crate) fn open(config: ExternalMonotonicAnchorConfig) -> Result<Self> {
        validate_anchor_parent(&config.path)?;
        let key = read_private_key(
            &config.key_file,
            MONOTONIC_ANCHOR_KEY_FILE_ENV,
            "external monotonic anchor",
        )?;
        Ok(Self {
            path: config.path,
            key,
            operation: Mutex::new(AnchorOperationState::default()),
            operation_idle: Condvar::new(),
        })
    }

    pub(crate) fn verify_and_advance_with(
        &self,
        state_provider: impl FnOnce() -> Result<DurableAnchorStateSnapshot>,
    ) -> Result<()> {
        self.run_serialized_operation(|| {
            let effect_lock = open_effect_lock_file(&effect_lock_path(&self.path)?)?;
            try_lock_effect(&effect_lock)?;
            let states = state_provider()?;
            self.verify_and_advance_inner(
                &states.outcome,
                &states.preference,
                states.telegram.as_ref(),
                states.operator.as_ref(),
                states.runtime_state.as_ref(),
            )
        })
    }

    pub(crate) fn begin_effect_lease_with(
        self: &Arc<Self>,
        state_provider: impl FnOnce() -> Result<DurableAnchorStateSnapshot>,
    ) -> Result<ExternalMonotonicAnchorEffectLease> {
        let mut state = match self.operation.lock() {
            Ok(state) => state,
            Err(poisoned) => {
                let mut state = poisoned.into_inner();
                state.fault =
                    Some("external monotonic anchor operation mutex poisoned".to_string());
                state.effect_lease_active = false;
                self.operation_idle.notify_all();
                return Err(anyhow::anyhow!(
                    "external monotonic anchor operation mutex poisoned"
                ));
            }
        };
        ensure_operation_healthy(&state)?;
        if state.effect_lease_active {
            return Err(ExternalMonotonicAnchorBusy.into());
        }
        let effect_lock = open_effect_lock_file(&effect_lock_path(&self.path)?)?;
        try_lock_effect(&effect_lock)?;
        let states = state_provider()?;
        if let Err(error) = self.verify_and_advance_inner(
            &states.outcome,
            &states.preference,
            states.telegram.as_ref(),
            states.operator.as_ref(),
            states.runtime_state.as_ref(),
        ) {
            state.fault = Some(format!("{error:#}"));
            return Err(error);
        }
        let lease_id = random_effect_lease_id()?;
        let source_hash = match self.append_pending_effect_lease(
            &lease_id,
            &states.outcome,
            &states.preference,
            states.telegram.as_ref(),
            states.operator.as_ref(),
            states.runtime_state.as_ref(),
        ) {
            Ok(source_hash) => source_hash,
            Err(error) => {
                state.fault = Some(format!("{error:#}"));
                return Err(error);
            }
        };
        state.effect_lease_active = true;
        drop(state);
        Ok(ExternalMonotonicAnchorEffectLease {
            anchor: Arc::clone(self),
            effect_lock: Some(effect_lock),
            lease_id,
            source_hash,
            active: true,
        })
    }

    fn run_serialized_operation<T>(&self, operation: impl FnOnce() -> Result<T>) -> Result<T> {
        let mut state = self
            .operation
            .lock()
            .map_err(|_| anyhow::anyhow!("external monotonic anchor operation mutex poisoned"))?;
        while state.effect_lease_active {
            state = self.operation_idle.wait(state).map_err(|_| {
                anyhow::anyhow!("external monotonic anchor operation mutex poisoned")
            })?;
        }
        ensure_operation_healthy(&state)?;
        let result = operation();
        if let Err(error) = &result
            && error
                .downcast_ref::<ExternalMonotonicAnchorBusy>()
                .is_none()
        {
            state.fault = Some(format!("{error:#}"));
        }
        result
    }

    fn verify_and_advance_inner(
        &self,
        outcome: &DurableMonotonicState,
        preference: &DurableMonotonicState,
        telegram: Option<&TelegramAuthorityMonotonicState>,
        operator: Option<&OperatorMutationMonotonicState>,
        runtime_state: Option<&RuntimeStateMonotonicState>,
    ) -> Result<()> {
        self.with_locked_anchor(|file, parent, snapshot| {
            if let Some(last) = snapshot.entries.last() {
                if last.effect_lease_state == Some(EffectLeaseRecordState::Pending) {
                    anyhow::bail!(
                        "external monotonic anchor contains an in-doubt effect lease; automatic re-entry is forbidden"
                    );
                }
                compare_anchor_entry_state(
                    last,
                    outcome,
                    preference,
                    telegram,
                    operator,
                    runtime_state,
                )?;
                if last.outcome_generation == outcome.generation()
                    && last.outcome_state_hash == outcome.state_hash()
                    && last.preference_generation == preference.generation()
                    && last.preference_state_hash == preference.state_hash()
                    && telegram_state_matches(last, telegram)
                    && operator_state_matches(last, operator)
                    && runtime_state_matches(last, runtime_state)
                {
                    return Ok(());
                }
            }
            if snapshot.physical_record_count() >= MAX_ANCHOR_RECORDS {
                anyhow::bail!(
                    "external monotonic anchor reached its {MAX_ANCHOR_RECORDS}-record limit"
                );
            }
            let sequence = snapshot.next_sequence();
            let previous_entry_hash = snapshot.previous_entry_hash();
            append_anchor_entry(
                file,
                parent,
                &self.key,
                sequence,
                previous_entry_hash,
                outcome,
                preference,
                telegram,
                operator,
                runtime_state,
                None,
            )
        })
    }

    fn append_pending_effect_lease(
        &self,
        lease_id: &str,
        outcome: &DurableMonotonicState,
        preference: &DurableMonotonicState,
        telegram: Option<&TelegramAuthorityMonotonicState>,
        operator: Option<&OperatorMutationMonotonicState>,
        runtime_state: Option<&RuntimeStateMonotonicState>,
    ) -> Result<String> {
        self.with_locked_anchor(|file, parent, snapshot| {
            let last = snapshot
                .entries
                .last()
                .context("effect lease source state was not anchored")?;
            if last.effect_lease_state == Some(EffectLeaseRecordState::Pending) {
                anyhow::bail!(
                    "external monotonic anchor already contains an in-doubt effect lease"
                );
            }
            require_exact_anchor_state(
                last,
                outcome,
                preference,
                telegram,
                operator,
                runtime_state,
            )?;
            if snapshot.physical_record_count() > MAX_ANCHOR_RECORDS.saturating_sub(2) {
                anyhow::bail!(
                    "external monotonic anchor lacks two reserved records for an effect lease"
                );
            }
            let source_hash = effect_lease_source_hash(lease_id, &entry_hash(last), last);
            append_anchor_entry(
                file,
                parent,
                &self.key,
                last.sequence + 1,
                entry_hash(last),
                outcome,
                preference,
                telegram,
                operator,
                runtime_state,
                Some(EffectLeaseBinding {
                    state: EffectLeaseRecordState::Pending,
                    lease_id,
                    source_hash: &source_hash,
                }),
            )?;
            Ok(source_hash)
        })
    }

    fn finalize_effect_lease_inner(
        &self,
        lease_id: &str,
        source_hash: &str,
        states: &DurableAnchorStateSnapshot,
    ) -> Result<()> {
        self.with_locked_anchor(|file, parent, snapshot| {
            let pending = snapshot
                .entries
                .last()
                .context("external monotonic anchor effect lease marker disappeared")?;
            if pending.effect_lease_state != Some(EffectLeaseRecordState::Pending)
                || pending.effect_lease_id.as_deref() != Some(lease_id)
                || pending.effect_lease_source_hash.as_deref() != Some(source_hash)
            {
                anyhow::bail!("external monotonic anchor effect lease marker does not match");
            }
            compare_anchor_entry_state(
                pending,
                &states.outcome,
                &states.preference,
                states.telegram.as_ref(),
                states.operator.as_ref(),
                states.runtime_state.as_ref(),
            )?;
            if snapshot.physical_record_count() >= MAX_ANCHOR_RECORDS {
                anyhow::bail!(
                    "external monotonic anchor reached its {MAX_ANCHOR_RECORDS}-record limit"
                );
            }
            append_anchor_entry(
                file,
                parent,
                &self.key,
                pending.sequence + 1,
                entry_hash(pending),
                &states.outcome,
                &states.preference,
                states.telegram.as_ref(),
                states.operator.as_ref(),
                states.runtime_state.as_ref(),
                Some(EffectLeaseBinding {
                    state: EffectLeaseRecordState::Finalized,
                    lease_id,
                    source_hash,
                }),
            )
        })
    }

    fn with_locked_anchor<T>(
        &self,
        operation: impl FnOnce(&mut File, &Path, &AnchorSnapshot) -> Result<T>,
    ) -> Result<T> {
        let parent = self
            .path
            .parent()
            .context("external monotonic anchor has no parent directory")?;
        let mut file = open_anchor_file(&self.path)?;
        lock_anchor(&file)?;
        let result = (|| {
            validate_private_file(&file, "external monotonic anchor", MAX_ANCHOR_BYTES)?;
            let mut snapshot = read_anchor_snapshot(&mut file, &self.key)?;
            if snapshot.should_rotate() {
                rotate_anchor_atomically(&self.path, &snapshot, &self.key)?;
                unlock_anchor(&file);
                file = open_anchor_file(&self.path)?;
                lock_anchor(&file)?;
                validate_private_file(&file, "external monotonic anchor", MAX_ANCHOR_BYTES)?;
                snapshot = read_anchor_snapshot(&mut file, &self.key)?;
            }
            operation(&mut file, parent, &snapshot)
        })();
        unlock_anchor(&file);
        result
    }
}

#[cfg(test)]
impl ExternalMonotonicAnchor {
    fn verify_and_advance(
        &self,
        outcome: &DurableMonotonicState,
        preference: &DurableMonotonicState,
        telegram: Option<&TelegramAuthorityMonotonicState>,
        operator: Option<&OperatorMutationMonotonicState>,
    ) -> Result<()> {
        let states = DurableAnchorStateSnapshot {
            outcome: outcome.clone(),
            preference: preference.clone(),
            telegram: telegram.cloned(),
            operator: operator.cloned(),
            runtime_state: None,
        };
        self.verify_and_advance_with(|| Ok(states))
    }

    fn begin_effect_lease(
        self: &Arc<Self>,
        outcome: &DurableMonotonicState,
        preference: &DurableMonotonicState,
        telegram: Option<&TelegramAuthorityMonotonicState>,
        operator: Option<&OperatorMutationMonotonicState>,
    ) -> Result<ExternalMonotonicAnchorEffectLease> {
        let states = DurableAnchorStateSnapshot {
            outcome: outcome.clone(),
            preference: preference.clone(),
            telegram: telegram.cloned(),
            operator: operator.cloned(),
            runtime_state: None,
        };
        self.begin_effect_lease_with(|| Ok(states))
    }
}

impl ExternalMonotonicAnchorEffectLease {
    pub(crate) fn finalize_with(
        mut self,
        state_provider: impl FnOnce() -> Result<DurableAnchorStateSnapshot>,
    ) -> Result<()> {
        if self.effect_lock.is_none() {
            anyhow::bail!("external monotonic anchor effect lock was lost");
        }
        let result =
            self.anchor
                .finish_effect_lease(&self.lease_id, &self.source_hash, state_provider);
        self.active = false;
        result
    }
}

#[cfg(test)]
impl ExternalMonotonicAnchorEffectLease {
    fn finalize(
        self,
        outcome: &DurableMonotonicState,
        preference: &DurableMonotonicState,
        telegram: Option<&TelegramAuthorityMonotonicState>,
        operator: Option<&OperatorMutationMonotonicState>,
    ) -> Result<()> {
        let states = DurableAnchorStateSnapshot {
            outcome: outcome.clone(),
            preference: preference.clone(),
            telegram: telegram.cloned(),
            operator: operator.cloned(),
            runtime_state: None,
        };
        self.finalize_with(|| Ok(states))
    }
}

impl AnchorSnapshot {
    fn physical_record_count(&self) -> usize {
        self.entries.len()
    }

    fn previous_entry_hash(&self) -> String {
        self.entries
            .last()
            .map(entry_hash)
            .unwrap_or_else(|| GENESIS_PREVIOUS_HASH.to_owned())
    }

    fn next_sequence(&self) -> u64 {
        self.entries.last().map_or(1, |entry| entry.sequence + 1)
    }

    fn should_rotate(&self) -> bool {
        self.entries.len() >= MAX_ANCHOR_RECORDS.saturating_sub(2)
            && self.entries.last().is_some_and(|entry| {
                entry.effect_lease_state != Some(EffectLeaseRecordState::Pending)
            })
    }
}

impl Drop for ExternalMonotonicAnchorEffectLease {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        self.anchor.abandon_effect_lease();
        self.active = false;
    }
}

impl ExternalMonotonicAnchor {
    fn finish_effect_lease(
        &self,
        lease_id: &str,
        source_hash: &str,
        state_provider: impl FnOnce() -> Result<DurableAnchorStateSnapshot>,
    ) -> Result<()> {
        let mut state = match self.operation.lock() {
            Ok(state) => state,
            Err(poisoned) => {
                let mut state = poisoned.into_inner();
                state.fault =
                    Some("external monotonic anchor operation mutex poisoned".to_string());
                state.effect_lease_active = false;
                self.operation_idle.notify_all();
                return Err(anyhow::anyhow!(
                    "external monotonic anchor operation mutex poisoned"
                ));
            }
        };
        if !state.effect_lease_active {
            state.fault = Some("effect lease finalization lost its active marker".to_string());
            self.operation_idle.notify_all();
            anyhow::bail!("external monotonic anchor effect lease is not active");
        }
        let result = state_provider()
            .and_then(|states| self.finalize_effect_lease_inner(lease_id, source_hash, &states));
        if let Err(error) = &result {
            state.fault = Some(format!("{error:#}"));
        }
        state.effect_lease_active = false;
        self.operation_idle.notify_all();
        result
    }

    fn abandon_effect_lease(&self) {
        let mut state = match self.operation.lock() {
            Ok(state) => state,
            Err(poisoned) => poisoned.into_inner(),
        };
        if state.effect_lease_active {
            state.fault =
                Some("external monotonic anchor effect lease dropped before finalization".into());
            state.effect_lease_active = false;
        }
        self.operation_idle.notify_all();
    }
}

fn ensure_operation_healthy(state: &AnchorOperationState) -> Result<()> {
    if let Some(fault) = state.fault.as_ref() {
        anyhow::bail!("external monotonic anchor is faulted: {fault}");
    }
    Ok(())
}

struct EffectLeaseBinding<'a> {
    state: EffectLeaseRecordState,
    lease_id: &'a str,
    source_hash: &'a str,
}

#[allow(clippy::too_many_arguments)]
fn append_anchor_entry(
    file: &mut File,
    parent: &Path,
    key: &[u8; 32],
    sequence: u64,
    previous_entry_hash: String,
    outcome: &DurableMonotonicState,
    preference: &DurableMonotonicState,
    telegram: Option<&TelegramAuthorityMonotonicState>,
    operator: Option<&OperatorMutationMonotonicState>,
    runtime_state: Option<&RuntimeStateMonotonicState>,
    effect_lease: Option<EffectLeaseBinding<'_>>,
) -> Result<()> {
    let mut entry = AnchorEntry {
        schema: ANCHOR_SCHEMA.into(),
        sequence,
        previous_entry_hash,
        outcome_generation: outcome.generation(),
        outcome_state_hash: outcome.state_hash().to_owned(),
        preference_generation: preference.generation(),
        preference_state_hash: preference.state_hash().to_owned(),
        telegram_generation: telegram.map(|state| state.journal_sequence),
        telegram_state_hash: telegram.map(telegram_state_hash),
        operator_generation: operator.map(|state| state.journal_revision),
        operator_state_hash: operator.map(|state| state.state_hash.clone()),
        runtime_state_generation: runtime_state.map(RuntimeStateMonotonicState::generation),
        runtime_state_hash: runtime_state.map(|state| state.state_hash().to_owned()),
        effect_lease_state: effect_lease.as_ref().map(|binding| binding.state),
        effect_lease_id: effect_lease
            .as_ref()
            .map(|binding| binding.lease_id.to_owned()),
        effect_lease_source_hash: effect_lease
            .as_ref()
            .map(|binding| binding.source_hash.to_owned()),
        rotation_compacted_records: None,
        rotation_history_hash: None,
        mac: String::new(),
    };
    entry.mac = entry_mac(&entry, key)?;
    let mut bytes = serde_json::to_vec(&entry).context("encode external monotonic anchor entry")?;
    bytes.push(b'\n');
    if bytes.len() > MAX_ANCHOR_LINE_BYTES {
        anyhow::bail!("external monotonic anchor entry exceeds bounded line size");
    }
    let current_len = file
        .metadata()
        .context("inspect external monotonic anchor before append")?
        .len();
    let reserve = u64::from(entry.effect_lease_state == Some(EffectLeaseRecordState::Pending))
        * MAX_ANCHOR_LINE_BYTES as u64;
    if current_len.saturating_add(bytes.len() as u64 + reserve) > MAX_ANCHOR_BYTES {
        anyhow::bail!("external monotonic anchor exceeds bounded file size");
    }
    file.seek(SeekFrom::End(0))
        .context("seek external monotonic anchor append position")?;
    file.write_all(&bytes)
        .context("append external monotonic anchor entry")?;
    file.sync_all()
        .context("fsync external monotonic anchor entry")?;
    fs::File::open(parent)
        .and_then(|directory| directory.sync_all())
        .context("fsync external monotonic anchor parent")
}

fn rotate_anchor_atomically(path: &Path, expected: &AnchorSnapshot, key: &[u8; 32]) -> Result<()> {
    let expected_last = expected
        .entries
        .last()
        .context("external monotonic anchor rotation has no source entry")?;
    if expected_last.effect_lease_state == Some(EffectLeaseRecordState::Pending) {
        anyhow::bail!("external monotonic anchor cannot rotate a pending effect lease");
    }
    let expected_last_hash = entry_hash(expected_last);
    update_private_state_atomically(path, MAX_ANCHOR_BYTES, ANCHOR_STAGING_PREFIX, |current| {
        let current = current.context("external monotonic anchor disappeared during rotation")?;
        let snapshot = read_anchor_snapshot_bytes(current, key)?;
        let last = snapshot
            .entries
            .last()
            .context("external monotonic anchor rotation source disappeared")?;
        if snapshot.entries.len() != expected.entries.len()
            || entry_hash(last) != expected_last_hash
        {
            anyhow::bail!("external monotonic anchor changed during rotation");
        }
        let previous_history = snapshot
            .entries
            .first()
            .and_then(|entry| entry.rotation_history_hash.as_deref())
            .unwrap_or(ROTATION_GENESIS_HASH);
        let previous_compacted = snapshot
            .entries
            .first()
            .and_then(|entry| entry.rotation_compacted_records)
            .unwrap_or(0);
        let compacted_records = previous_compacted
            .checked_add(snapshot.entries.len() as u64)
            .context("external monotonic anchor rotation count overflow")?;
        let history_hash =
            rotation_history_hash(previous_history, &expected_last_hash, compacted_records);
        let mut rotated = last.clone();
        rotated.sequence = last.sequence + 1;
        rotated.previous_entry_hash = expected_last_hash;
        rotated.effect_lease_state = None;
        rotated.effect_lease_id = None;
        rotated.effect_lease_source_hash = None;
        rotated.rotation_compacted_records = Some(compacted_records);
        rotated.rotation_history_hash = Some(history_hash);
        rotated.mac.clear();
        rotated.mac = entry_mac(&rotated, key)?;
        let mut bytes =
            serde_json::to_vec(&rotated).context("encode rotated external monotonic anchor")?;
        bytes.push(b'\n');
        if bytes.len() > MAX_ANCHOR_LINE_BYTES {
            anyhow::bail!("rotated external monotonic anchor exceeds bounded line size");
        }
        Ok((bytes, ()))
    })
}

fn rotation_history_hash(
    previous_history_hash: &str,
    previous_entry_hash: &str,
    compacted_records: u64,
) -> String {
    let mut hasher = Sha256::new();
    update_hash_frame(&mut hasher, ANCHOR_ROTATION_HISTORY_DOMAIN);
    update_hash_frame(&mut hasher, previous_history_hash.as_bytes());
    update_hash_frame(&mut hasher, previous_entry_hash.as_bytes());
    update_hash_frame(&mut hasher, &compacted_records.to_be_bytes());
    format!("sha256:{:x}", hasher.finalize())
}

fn compare_anchor_entry_state(
    anchored: &AnchorEntry,
    outcome: &DurableMonotonicState,
    preference: &DurableMonotonicState,
    telegram: Option<&TelegramAuthorityMonotonicState>,
    operator: Option<&OperatorMutationMonotonicState>,
    runtime_state: Option<&RuntimeStateMonotonicState>,
) -> Result<()> {
    compare_state(
        "outcome",
        anchored.outcome_generation,
        &anchored.outcome_state_hash,
        outcome,
    )?;
    compare_state(
        "preference",
        anchored.preference_generation,
        &anchored.preference_state_hash,
        preference,
    )?;
    compare_telegram_state(
        anchored.telegram_generation,
        anchored.telegram_state_hash.as_deref(),
        telegram,
    )?;
    compare_operator_state(
        anchored.operator_generation,
        anchored.operator_state_hash.as_deref(),
        operator,
    )?;
    compare_runtime_state(
        anchored.runtime_state_generation,
        anchored.runtime_state_hash.as_deref(),
        runtime_state,
    )
}

fn require_exact_anchor_state(
    anchored: &AnchorEntry,
    outcome: &DurableMonotonicState,
    preference: &DurableMonotonicState,
    telegram: Option<&TelegramAuthorityMonotonicState>,
    operator: Option<&OperatorMutationMonotonicState>,
    runtime_state: Option<&RuntimeStateMonotonicState>,
) -> Result<()> {
    if anchored.outcome_generation != outcome.generation()
        || anchored.outcome_state_hash != outcome.state_hash()
        || anchored.preference_generation != preference.generation()
        || anchored.preference_state_hash != preference.state_hash()
        || !telegram_state_matches(anchored, telegram)
        || !operator_state_matches(anchored, operator)
        || !runtime_state_matches(anchored, runtime_state)
    {
        anyhow::bail!("external monotonic anchor effect lease source state is not exact");
    }
    Ok(())
}

fn effect_lease_source_hash(
    lease_id: &str,
    previous_entry_hash: &str,
    state: &AnchorEntry,
) -> String {
    let mut hasher = Sha256::new();
    update_hash_frame(&mut hasher, EFFECT_LEASE_SOURCE_HASH_DOMAIN);
    for value in [lease_id.to_owned(), previous_entry_hash.to_owned()]
        .into_iter()
        .chain(anchor_state_fields(state))
    {
        update_hash_frame(&mut hasher, value.as_bytes());
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn random_effect_lease_id() -> Result<String> {
    let mut bytes = [0_u8; 32];
    OpenOptions::new()
        .read(true)
        .open("/dev/urandom")
        .context("open OS random source for external monotonic effect lease")?
        .read_exact(&mut bytes)
        .context("read external monotonic effect lease identifier")?;
    Ok(hex_encode(&bytes))
}

fn telegram_state_matches(
    anchored: &AnchorEntry,
    current: Option<&TelegramAuthorityMonotonicState>,
) -> bool {
    anchored.telegram_generation == current.map(|state| state.journal_sequence)
        && anchored.telegram_state_hash.as_deref() == current.map(telegram_state_hash).as_deref()
}

fn operator_state_matches(
    anchored: &AnchorEntry,
    current: Option<&OperatorMutationMonotonicState>,
) -> bool {
    anchored.operator_generation == current.map(|state| state.journal_revision)
        && anchored.operator_state_hash.as_deref() == current.map(|state| state.state_hash.as_str())
}

fn runtime_state_matches(
    anchored: &AnchorEntry,
    current: Option<&RuntimeStateMonotonicState>,
) -> bool {
    anchored.runtime_state_generation == current.map(RuntimeStateMonotonicState::generation)
        && anchored.runtime_state_hash.as_deref()
            == current.map(RuntimeStateMonotonicState::state_hash)
}

fn compare_telegram_state(
    anchored_generation: Option<u64>,
    anchored_hash: Option<&str>,
    current: Option<&TelegramAuthorityMonotonicState>,
) -> Result<()> {
    match (anchored_generation, anchored_hash, current) {
        (None, None, None | Some(_)) => Ok(()),
        (Some(_), Some(_), None) => {
            anyhow::bail!("Telegram authority state disappeared after it was externally anchored")
        }
        (Some(generation), Some(hash), Some(current)) => {
            let current_hash = telegram_state_hash(current);
            if current.journal_sequence < generation {
                anyhow::bail!(
                    "Telegram authority journal rolled back from sequence {generation} to {}",
                    current.journal_sequence
                );
            }
            if current.journal_sequence == generation && current_hash != hash {
                anyhow::bail!("Telegram authority journal diverged at an anchored sequence");
            }
            Ok(())
        }
        _ => anyhow::bail!("external monotonic anchor Telegram state binding is incomplete"),
    }
}

fn telegram_state_hash(state: &TelegramAuthorityMonotonicState) -> String {
    let mut hasher = Sha256::new();
    update_hash_frame(&mut hasher, TELEGRAM_STATE_HASH_DOMAIN);
    for value in [
        state.schema,
        state.authority_owner,
        &state.journal_sequence.to_string(),
        &state.latest_event_hash,
        state.latest_event_mac.as_deref().unwrap_or("none"),
    ] {
        update_hash_frame(&mut hasher, value.as_bytes());
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn compare_operator_state(
    anchored_generation: Option<u64>,
    anchored_hash: Option<&str>,
    current: Option<&OperatorMutationMonotonicState>,
) -> Result<()> {
    match (anchored_generation, anchored_hash, current) {
        (None, None, None | Some(_)) => Ok(()),
        (Some(_), Some(_), None) => {
            anyhow::bail!("operator mutation journal disappeared after it was externally anchored")
        }
        (Some(generation), Some(hash), Some(current)) => {
            if current.journal_revision < generation {
                anyhow::bail!(
                    "operator mutation journal rolled back from revision {generation} to {}",
                    current.journal_revision
                );
            }
            if current.journal_revision == generation && current.state_hash != hash {
                anyhow::bail!("operator mutation journal diverged at an anchored revision");
            }
            Ok(())
        }
        _ => anyhow::bail!("external monotonic anchor operator state binding is incomplete"),
    }
}

fn compare_runtime_state(
    anchored_generation: Option<u64>,
    anchored_hash: Option<&str>,
    current: Option<&RuntimeStateMonotonicState>,
) -> Result<()> {
    match (anchored_generation, anchored_hash, current) {
        (None, None, None | Some(_)) => Ok(()),
        (Some(_), Some(_), None) => {
            anyhow::bail!("runtime session state disappeared after it was externally anchored")
        }
        (Some(generation), Some(hash), Some(current)) => {
            if current.generation() < generation {
                anyhow::bail!(
                    "runtime session state rolled back from generation {generation} to {}",
                    current.generation()
                );
            }
            if current.generation() == generation && current.state_hash() != hash {
                anyhow::bail!("runtime session state diverged at an anchored generation");
            }
            Ok(())
        }
        _ => anyhow::bail!("external monotonic anchor runtime state binding is incomplete"),
    }
}

fn compare_state(
    domain: &str,
    anchored_generation: u64,
    anchored_hash: &str,
    current: &DurableMonotonicState,
) -> Result<()> {
    if current.generation() < anchored_generation {
        anyhow::bail!(
            "{domain} durable state rolled back from generation {anchored_generation} to {}",
            current.generation()
        );
    }
    if current.generation() == anchored_generation && current.state_hash() != anchored_hash {
        anyhow::bail!("{domain} durable state diverged at an anchored generation");
    }
    Ok(())
}

fn read_anchor_snapshot(file: &mut File, key: &[u8; 32]) -> Result<AnchorSnapshot> {
    file.seek(SeekFrom::Start(0))
        .context("seek external monotonic anchor")?;
    let mut bytes = Vec::new();
    file.take(MAX_ANCHOR_BYTES + 1)
        .read_to_end(&mut bytes)
        .context("read external monotonic anchor")?;
    if bytes.len() as u64 > MAX_ANCHOR_BYTES {
        anyhow::bail!("external monotonic anchor exceeds bounded file size");
    }
    read_anchor_snapshot_bytes(&bytes, key)
}

fn read_anchor_snapshot_bytes(bytes: &[u8], key: &[u8; 32]) -> Result<AnchorSnapshot> {
    if bytes.len() as u64 > MAX_ANCHOR_BYTES {
        anyhow::bail!("external monotonic anchor exceeds bounded file size");
    }
    let lines = bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let mut previous_hash = GENESIS_PREVIOUS_HASH.to_owned();
    let mut expected_sequence = 1;
    let mut entries: Vec<AnchorEntry> = Vec::new();
    for line in lines {
        if line.len() > MAX_ANCHOR_LINE_BYTES || entries.len() >= MAX_ANCHOR_RECORDS {
            anyhow::bail!("external monotonic anchor exceeds bounded record limits");
        }
        let entry: AnchorEntry =
            serde_json::from_slice(line).context("decode external monotonic anchor entry")?;
        let rotation_binding_valid = match (
            entry.rotation_compacted_records,
            entry.rotation_history_hash.as_deref(),
        ) {
            (None, None) => true,
            (Some(compacted), Some(history_hash)) => {
                compacted > 0 && history_hash.strip_prefix("sha256:").is_some_and(hex64)
            }
            _ => false,
        };
        if entries.is_empty() && entry.rotation_compacted_records.is_some() {
            if entry.effect_lease_state.is_some()
                || !entry
                    .previous_entry_hash
                    .strip_prefix("sha256:")
                    .is_some_and(hex64)
            {
                anyhow::bail!("external monotonic anchor rotation binding is invalid");
            }
            expected_sequence = entry.sequence;
            previous_hash.clone_from(&entry.previous_entry_hash);
        } else if !entries.is_empty() && entry.rotation_compacted_records.is_some() {
            anyhow::bail!("external monotonic anchor rotation marker is not the root entry");
        }
        let effect_lease_binding_valid = match (
            entry.effect_lease_state,
            entry.effect_lease_id.as_deref(),
            entry.effect_lease_source_hash.as_deref(),
        ) {
            (None, None, None) => true,
            (Some(_), Some(lease_id), Some(source_hash)) => {
                hex64(lease_id) && source_hash.strip_prefix("sha256:").is_some_and(hex64)
            }
            _ => false,
        };
        if entry.schema != ANCHOR_SCHEMA
            || entry.sequence != expected_sequence
            || entry.previous_entry_hash != previous_hash
            || entry.outcome_state_hash.is_empty()
            || entry.preference_state_hash.is_empty()
            || (entry.telegram_generation.is_some() != entry.telegram_state_hash.is_some())
            || (entry.operator_generation.is_some() != entry.operator_state_hash.is_some())
            || (entry.runtime_state_generation.is_some() != entry.runtime_state_hash.is_some())
            || entry
                .telegram_state_hash
                .as_deref()
                .is_some_and(str::is_empty)
            || entry
                .operator_state_hash
                .as_deref()
                .is_some_and(str::is_empty)
            || entry
                .runtime_state_hash
                .as_deref()
                .is_some_and(str::is_empty)
            || !rotation_binding_valid
            || !effect_lease_binding_valid
            || entry.mac.len() != 64
        {
            anyhow::bail!("external monotonic anchor chain binding is invalid");
        }
        let expected_mac = entry_mac(&entry, key)?;
        if !constant_time_hex_equal(&expected_mac, &entry.mac) {
            anyhow::bail!("external monotonic anchor MAC is invalid");
        }
        if let Some(previous) = entries.last() {
            if previous.effect_lease_state == Some(EffectLeaseRecordState::Pending)
                && (entry.effect_lease_state != Some(EffectLeaseRecordState::Finalized)
                    || entry.effect_lease_id != previous.effect_lease_id
                    || entry.effect_lease_source_hash != previous.effect_lease_source_hash)
            {
                anyhow::bail!(
                    "external monotonic anchor pending effect lease has no exact finalization"
                );
            }
            if entry.effect_lease_state == Some(EffectLeaseRecordState::Finalized)
                && previous.effect_lease_state != Some(EffectLeaseRecordState::Pending)
            {
                anyhow::bail!(
                    "external monotonic anchor effect lease finalization has no pending marker"
                );
            }
        } else if entry.effect_lease_state == Some(EffectLeaseRecordState::Finalized) {
            anyhow::bail!("external monotonic anchor starts with an effect lease finalization");
        }
        if entry.effect_lease_state == Some(EffectLeaseRecordState::Pending) {
            let expected_source_hash = effect_lease_source_hash(
                entry
                    .effect_lease_id
                    .as_deref()
                    .context("pending effect lease identifier")?,
                &entry.previous_entry_hash,
                &entry,
            );
            if entry.effect_lease_source_hash.as_deref() != Some(expected_source_hash.as_str()) {
                anyhow::bail!("external monotonic anchor effect lease source binding is invalid");
            }
        }
        previous_hash = entry_hash(&entry);
        expected_sequence += 1;
        entries.push(entry);
    }
    Ok(AnchorSnapshot { entries })
}

fn entry_mac(entry: &AnchorEntry, key: &[u8; 32]) -> Result<String> {
    let mut mac = HmacSha256::new_from_slice(key).context("initialize anchor HMAC")?;
    update_mac_frame(&mut mac, ANCHOR_MAC_DOMAIN);
    for value in entry_fields(entry) {
        update_mac_frame(&mut mac, value.as_bytes());
    }
    Ok(hex_encode(&mac.finalize().into_bytes()))
}

fn entry_hash(entry: &AnchorEntry) -> String {
    let mut hasher = Sha256::new();
    update_hash_frame(&mut hasher, ANCHOR_ENTRY_HASH_DOMAIN);
    for value in entry_fields(entry) {
        update_hash_frame(&mut hasher, value.as_bytes());
    }
    update_hash_frame(&mut hasher, entry.mac.as_bytes());
    format!("sha256:{:x}", hasher.finalize())
}

fn entry_fields(entry: &AnchorEntry) -> Vec<String> {
    let mut fields = vec![
        entry.schema.clone(),
        entry.sequence.to_string(),
        entry.previous_entry_hash.clone(),
    ];
    fields.extend(anchor_state_fields(entry));
    if let (Some(state), Some(lease_id), Some(source_hash)) = (
        entry.effect_lease_state,
        entry.effect_lease_id.as_ref(),
        entry.effect_lease_source_hash.as_ref(),
    ) {
        fields.extend([
            match state {
                EffectLeaseRecordState::Pending => "pending".to_owned(),
                EffectLeaseRecordState::Finalized => "finalized".to_owned(),
            },
            lease_id.clone(),
            source_hash.clone(),
        ]);
    }
    if let (Some(compacted_records), Some(history_hash)) = (
        entry.rotation_compacted_records,
        entry.rotation_history_hash.as_ref(),
    ) {
        fields.push(compacted_records.to_string());
        fields.push(history_hash.clone());
    }
    fields
}

fn anchor_state_fields(entry: &AnchorEntry) -> Vec<String> {
    let mut fields = vec![
        entry.outcome_generation.to_string(),
        entry.outcome_state_hash.clone(),
        entry.preference_generation.to_string(),
        entry.preference_state_hash.clone(),
        entry
            .telegram_generation
            .map_or_else(|| "none".to_owned(), |value| value.to_string()),
        entry
            .telegram_state_hash
            .clone()
            .unwrap_or_else(|| "none".to_owned()),
    ];
    if let (Some(generation), Some(state_hash)) = (
        entry.operator_generation,
        entry.operator_state_hash.as_ref(),
    ) {
        fields.push(generation.to_string());
        fields.push(state_hash.clone());
    }
    if let (Some(generation), Some(state_hash)) = (
        entry.runtime_state_generation,
        entry.runtime_state_hash.as_ref(),
    ) {
        fields.push(generation.to_string());
        fields.push(state_hash.clone());
    }
    fields
}

fn update_mac_frame(mac: &mut HmacSha256, value: &[u8]) {
    mac.update(&(value.len() as u64).to_be_bytes());
    mac.update(value);
}

fn update_hash_frame(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value);
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

fn hex64(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

fn effect_lock_path(anchor_path: &Path) -> Result<PathBuf> {
    let file_name = anchor_path
        .file_name()
        .context("external monotonic anchor has no file name")?;
    let mut lock_name = file_name.to_os_string();
    lock_name.push(".effect.lock");
    Ok(anchor_path.with_file_name(lock_name))
}

#[cfg(unix)]
fn open_effect_lock_file(path: &Path) -> Result<File> {
    use std::os::unix::fs::OpenOptionsExt;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o600)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)
        .context("open private external monotonic effect lock")?;
    validate_private_file(&file, "external monotonic effect lock", 0)?;
    Ok(file)
}

#[cfg(not(unix))]
fn open_effect_lock_file(_path: &Path) -> Result<File> {
    anyhow::bail!("external monotonic effect lock requires Unix secure-file semantics")
}

#[cfg(unix)]
fn try_lock_effect(file: &File) -> Result<()> {
    use std::os::fd::AsRawFd;
    if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
        return Ok(());
    }
    let error = std::io::Error::last_os_error();
    let raw_error = error.raw_os_error();
    if raw_error == Some(libc::EWOULDBLOCK) || raw_error == Some(libc::EAGAIN) {
        return Err(ExternalMonotonicAnchorBusy.into());
    }
    Err(error).context("lock external monotonic effect lease")
}

#[cfg(not(unix))]
fn try_lock_effect(_file: &File) -> Result<()> {
    anyhow::bail!("external monotonic effect lock requires Unix secure-file semantics")
}

#[cfg(unix)]
fn open_anchor_file(path: &Path) -> Result<File> {
    use std::os::unix::fs::OpenOptionsExt;
    OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .mode(0o600)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)
        .context("open private external monotonic anchor")
}

#[cfg(not(unix))]
fn open_anchor_file(_path: &Path) -> Result<File> {
    anyhow::bail!("external monotonic anchor requires Unix secure-file semantics")
}

fn validate_private_file(file: &File, purpose: &str, max_bytes: u64) -> Result<()> {
    let metadata = file
        .metadata()
        .with_context(|| format!("inspect {purpose}"))?;
    if !metadata.file_type().is_file() || metadata.len() > max_bytes {
        anyhow::bail!("{purpose} is not a bounded regular file");
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o077 != 0 {
            anyhow::bail!("{purpose} permissions must deny group/other");
        }
        // SAFETY: `geteuid` reads process credentials and has no pointer or lifetime inputs.
        let effective_uid = unsafe { libc::geteuid() };
        if metadata.nlink() != 1 || metadata.uid() != effective_uid {
            anyhow::bail!("{purpose} owner or link count is invalid");
        }
    }
    Ok(())
}

#[cfg(unix)]
fn validate_anchor_parent(path: &Path) -> Result<()> {
    use std::os::unix::fs::MetadataExt;
    use std::os::unix::fs::PermissionsExt;
    let parent = path
        .parent()
        .context("external monotonic anchor has no parent directory")?;
    let metadata = fs::symlink_metadata(parent)
        .context("inspect external monotonic anchor parent directory")?;
    // SAFETY: `geteuid` reads process credentials and has no pointer or lifetime inputs.
    let effective_uid = unsafe { libc::geteuid() };
    if !metadata.is_dir()
        || metadata.file_type().is_symlink()
        || metadata.uid() != effective_uid
        || metadata.permissions().mode() & 0o7777 != 0o700
    {
        anyhow::bail!("external monotonic anchor parent must be an owned mode-0700 directory");
    }
    Ok(())
}

#[cfg(not(unix))]
fn validate_anchor_parent(_path: &Path) -> Result<()> {
    anyhow::bail!("external monotonic anchor requires Unix secure-file semantics")
}

#[cfg(unix)]
fn lock_anchor(file: &File) -> Result<()> {
    use std::os::fd::AsRawFd;
    if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX) } != 0 {
        return Err(std::io::Error::last_os_error()).context("lock external monotonic anchor");
    }
    Ok(())
}

#[cfg(not(unix))]
fn lock_anchor(_file: &File) -> Result<()> {
    anyhow::bail!("external monotonic anchor requires Unix secure-file semantics")
}

#[cfg(unix)]
fn unlock_anchor(file: &File) {
    use std::os::fd::AsRawFd;
    let _ = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) };
}

#[cfg(not(unix))]
fn unlock_anchor(_file: &File) {}

#[cfg(all(test, unix))]
#[path = "../tests/unit/durability_anchor.rs"]
mod tests;
