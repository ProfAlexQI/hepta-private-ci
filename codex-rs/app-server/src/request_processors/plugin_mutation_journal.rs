use hepta_authority::AuthenticatedJournalStore;
use hepta_authority::AuthenticationFraming;
use hepta_authority::LegacyJournalMigrationStore;
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
const RETIRED_LEGACY_JOURNAL_VERSION: u32 = 3;
const ANCHOR_VERSION: u32 = 1;
const MIGRATION_MARKER_VERSION: u32 = 1;
const MIGRATION_LAYOUT_VERSION: u32 = 1;
const MAX_RECORDS: usize = PLUGIN_MUTATION_JOURNAL_POLICY.max_active_records;
const RETAIN_TERMINAL_RECORDS: usize = 512;
const MAX_CHECKPOINTED_TERMINALS: usize =
    PLUGIN_MUTATION_JOURNAL_POLICY.max_checkpointed_authorities;
const MAX_JOURNAL_BYTES: u64 = PLUGIN_MUTATION_JOURNAL_POLICY.max_journal_bytes;
const MAX_ANCHOR_BYTES: u64 = 64 * 1024;
const MAX_KEY_BYTES: u64 = 1024;
const MAX_MIGRATION_MARKER_BYTES: u64 = 64 * 1024;
const MAX_TERMINAL_RESPONSE_BYTES: usize = PLUGIN_MUTATION_JOURNAL_POLICY.max_record_bytes;
const MAX_TERMINAL_ERROR_BYTES: usize = 64 * 1024;
const JOURNAL_HASH_DOMAIN: &str = "hepta-plugin-mutation-journal-v2";
const JOURNAL_MAC_DOMAIN: &[u8] = b"hepta.plugin-mutation-journal.hmac-sha256.v2";
const ANCHOR_MAC_DOMAIN: &[u8] = b"hepta.plugin-mutation-anchor.hmac-sha256.v1";
const MIGRATION_MARKER_MAC_DOMAIN: &[u8] = b"hepta.plugin-mutation-layout-marker.hmac-sha256.v1";
const MIGRATION_TOMBSTONE_MAC_DOMAIN: &[u8] =
    b"hepta.plugin-mutation-layout-tombstone.hmac-sha256.v1";
const ACTIVE_JOURNAL_RELATIVE_PATH: &str = ".hepta-authority/plugin-mutation/journal.json";
const CHECKPOINT_HASH_DOMAIN: &str = "hepta-plugin-mutation-checkpoint-v1";
const CHECKPOINT_GENESIS_HASH: &str =
    "sha256:0000000000000000000000000000000000000000000000000000000000000000";
include!("plugin_mutation_journal/state_and_migration.rs");

fn mutate_current_state<T>(
    journal_store: &AuthenticatedJournalStore,
    anchor_store: &AuthenticatedJournalStore,
    key: &[u8; 32],
    mutate: impl FnOnce(&mut PluginMutationState) -> Result<T, PluginMutationJournalError>,
) -> Result<T, PluginMutationJournalError> {
    let mut loaded = read_state(journal_store, key)?;
    if loaded.origin != StateOrigin::Current {
        return Err(PluginMutationJournalError::new(
            "secure plugin mutation layout did not contain a current journal",
        ));
    }
    verify_or_initialize_anchor(anchor_store, &loaded, key)?;
    apply_state_mutation(journal_store, anchor_store, key, &mut loaded.state, mutate)
}

fn apply_state_mutation<T>(
    journal_store: &AuthenticatedJournalStore,
    anchor_store: &AuthenticatedJournalStore,
    key: &[u8; 32],
    state: &mut PluginMutationState,
    mutate: impl FnOnce(&mut PluginMutationState) -> Result<T, PluginMutationJournalError>,
) -> Result<T, PluginMutationJournalError> {
    let mutation_result = mutate(state);
    if mutation_result.is_ok() {
        validate_state(state)?;
        state.refresh_integrity(key)?;
        publish_state(journal_store, state)?;
        publish_anchor(anchor_store, state, key)?;
    }
    mutation_result
}

fn read_optional_store_bytes(
    store: &AuthenticatedJournalStore,
    label: &str,
) -> Result<Option<Vec<u8>>, PluginMutationJournalError> {
    store
        .read()
        .map_err(|error| store_error(&format!("read {label}"), error))
}

fn read_optional_key(
    store: &AuthenticatedJournalStore,
    label: &str,
) -> Result<Option<[u8; 32]>, PluginMutationJournalError> {
    read_optional_store_bytes(store, label)?
        .map(|bytes| decode_key(&bytes))
        .transpose()
}

fn read_optional_legacy_key(
    store: &LegacyJournalMigrationStore,
    label: &str,
) -> Result<Option<[u8; 32]>, PluginMutationJournalError> {
    store
        .read()
        .map_err(|error| store_error(&format!("read {label}"), error))?
        .map(|bytes| decode_key(&bytes))
        .transpose()
}

fn publish_key(
    store: &AuthenticatedJournalStore,
    key: &[u8; 32],
) -> Result<(), PluginMutationJournalError> {
    store
        .publish(hex_encode(key).as_bytes())
        .map_err(|error| store_error("publish active plugin mutation journal key", error))
}

fn optional_journal_version(
    bytes: Option<&[u8]>,
    label: &str,
) -> Result<Option<u32>, PluginMutationJournalError> {
    bytes
        .map(|bytes| {
            serde_json::from_slice::<VersionHeader>(bytes)
                .map(|header| header.version)
                .map_err(|error| {
                    PluginMutationJournalError::new(format!("decode {label} header: {error}"))
                })
        })
        .transpose()
}

fn decode_active_layout_state(
    bytes: Option<&[u8]>,
    key: &[u8; 32],
) -> Result<Option<PluginMutationState>, PluginMutationJournalError> {
    let Some(bytes) = bytes else {
        return Ok(None);
    };
    let version = optional_journal_version(Some(bytes), "active plugin mutation journal")?
        .ok_or_else(|| {
            PluginMutationJournalError::new("active plugin mutation journal header is missing")
        })?;
    if version != JOURNAL_VERSION {
        return Err(PluginMutationJournalError::new(format!(
            "unsupported active plugin mutation journal version {version}"
        )));
    }
    let state: PluginMutationState = serde_json::from_slice(bytes).map_err(|error| {
        PluginMutationJournalError::new(format!("decode active plugin mutation journal: {error}"))
    })?;
    state.verify(key)?;
    Ok(Some(state))
}

fn decode_legacy_layout_journal(
    bytes: Option<&[u8]>,
    key: &[u8; 32],
    layout: &PluginMutationLayoutMigration,
) -> Result<LegacyLayoutJournal, PluginMutationJournalError> {
    let Some(bytes) = bytes else {
        return Ok(LegacyLayoutJournal::Missing);
    };
    let version = optional_journal_version(Some(bytes), "legacy plugin mutation journal")?
        .ok_or_else(|| {
            PluginMutationJournalError::new("legacy plugin mutation journal header is missing")
        })?;
    match version {
        LEGACY_JOURNAL_VERSION => {
            let state: LegacyPluginMutationState =
                serde_json::from_slice(bytes).map_err(|error| {
                    PluginMutationJournalError::new(format!(
                        "decode legacy v1 plugin mutation journal: {error}"
                    ))
                })?;
            state.verify()?;
            Ok(LegacyLayoutJournal::Source(MigrationSourceState {
                source: MigrationSource::LegacyV1,
                state: state.migrate(key)?,
            }))
        }
        JOURNAL_VERSION => {
            let state: PluginMutationState = serde_json::from_slice(bytes).map_err(|error| {
                PluginMutationJournalError::new(format!(
                    "decode legacy v2 plugin mutation journal: {error}"
                ))
            })?;
            state.verify(key)?;
            Ok(LegacyLayoutJournal::Source(MigrationSourceState {
                source: MigrationSource::LegacyV2,
                state,
            }))
        }
        RETIRED_LEGACY_JOURNAL_VERSION => {
            let tombstone: RetiredLegacyPluginMutationJournal = serde_json::from_slice(bytes)
                .map_err(|error| {
                    PluginMutationJournalError::new(format!(
                        "decode retired legacy plugin mutation journal: {error}"
                    ))
                })?;
            tombstone.verify(key, &layout.legacy_identity)?;
            Ok(LegacyLayoutJournal::Retired(tombstone))
        }
        version => Err(PluginMutationJournalError::new(format!(
            "unsupported legacy plugin mutation journal version {version}"
        ))),
    }
}

fn decode_migration_marker(
    bytes: Option<&[u8]>,
    key: &[u8; 32],
    layout: &PluginMutationLayoutMigration,
) -> Result<Option<PluginMutationMigrationMarker>, PluginMutationJournalError> {
    let Some(bytes) = bytes else {
        return Ok(None);
    };
    let marker: PluginMutationMigrationMarker = serde_json::from_slice(bytes).map_err(|error| {
        PluginMutationJournalError::new(format!(
            "decode plugin mutation layout migration marker: {error}"
        ))
    })?;
    marker.verify(key, &layout.legacy_identity)?;
    Ok(Some(marker))
}

fn publish_migration_marker(
    store: &AuthenticatedJournalStore,
    marker: &PluginMutationMigrationMarker,
) -> Result<(), PluginMutationJournalError> {
    let bytes = serde_json::to_vec(marker).map_err(|error| {
        PluginMutationJournalError::new(format!(
            "encode plugin mutation layout migration marker: {error}"
        ))
    })?;
    store
        .publish(&bytes)
        .map_err(|error| store_error("publish plugin mutation layout migration marker", error))
}

fn publish_legacy_tombstone(
    store: &LegacyJournalMigrationStore,
    claim: PluginMutationMigrationClaim,
    key: &[u8; 32],
) -> Result<(), PluginMutationJournalError> {
    let tombstone = RetiredLegacyPluginMutationJournal::new(claim, key)?;
    let bytes = serde_json::to_vec(&tombstone).map_err(|error| {
        PluginMutationJournalError::new(format!(
            "encode retired legacy plugin mutation journal: {error}"
        ))
    })?;
    store
        .publish(&bytes)
        .map_err(|error| store_error("publish retired legacy plugin mutation journal", error))
}

fn state_for_migration_claim(
    claim: &PluginMutationMigrationClaim,
    legacy: LegacyLayoutJournal,
    key: &[u8; 32],
) -> Result<PluginMutationState, PluginMutationJournalError> {
    match (claim.source, legacy) {
        (MigrationSource::Fresh, LegacyLayoutJournal::Missing) => PluginMutationState::empty(key),
        (source, LegacyLayoutJournal::Source(evidence)) if source == evidence.source => {
            Ok(evidence.state)
        }
        _ => Err(PluginMutationJournalError::new(
            "legacy plugin mutation evidence does not match the preparing migration marker",
        )),
    }
}

fn store_has_bytes(
    store: &AuthenticatedJournalStore,
    label: &str,
) -> Result<bool, PluginMutationJournalError> {
    Ok(read_optional_store_bytes(store, label)?.is_some())
}

fn preflight_migration_anchor(
    store: &AuthenticatedJournalStore,
    state: &PluginMutationState,
    key: &[u8; 32],
    source: MigrationSource,
) -> Result<(), PluginMutationJournalError> {
    match read_plugin_mutation_anchor(store, key)? {
        Some(anchor) => validate_anchor_relation(&anchor, state),
        None if source != MigrationSource::LegacyV2 => Ok(()),
        None => Err(PluginMutationJournalError::new(
            "external anchor is missing for a legacy authenticated plugin mutation journal",
        )),
    }
}

fn reconcile_migration_anchor(
    store: &AuthenticatedJournalStore,
    state: &PluginMutationState,
    key: &[u8; 32],
    allow_missing: bool,
) -> Result<(), PluginMutationJournalError> {
    match read_plugin_mutation_anchor(store, key)? {
        Some(anchor) => {
            validate_anchor_relation(&anchor, state)?;
            if anchor.generation < state.generation {
                publish_anchor(store, state, key)?;
            }
            Ok(())
        }
        None if allow_missing => publish_anchor(store, state, key),
        None => Err(PluginMutationJournalError::new(
            "plugin mutation anchor is missing after secure-layout publication",
        )),
    }
}

fn read_plugin_mutation_anchor(
    store: &AuthenticatedJournalStore,
    key: &[u8; 32],
) -> Result<Option<PluginMutationAnchor>, PluginMutationJournalError> {
    let Some(bytes) = read_optional_store_bytes(store, "plugin mutation anchor")? else {
        return Ok(None);
    };
    let anchor: PluginMutationAnchor = serde_json::from_slice(&bytes).map_err(|error| {
        PluginMutationJournalError::new(format!("decode plugin mutation anchor: {error}"))
    })?;
    anchor.verify(key)?;
    Ok(Some(anchor))
}

fn validate_anchor_relation(
    anchor: &PluginMutationAnchor,
    state: &PluginMutationState,
) -> Result<(), PluginMutationJournalError> {
    if anchor.generation > state.generation {
        return Err(PluginMutationJournalError::new(
            "plugin mutation journal rollback detected during layout migration",
        ));
    }
    if anchor.generation == state.generation && anchor.state_hash != state.state_hash {
        return Err(PluginMutationJournalError::new(
            "plugin mutation journal same-generation fork detected during layout migration",
        ));
    }
    Ok(())
}

fn legacy_private_store(
    path: &Path,
    max_bytes: u64,
    staging_prefix: &str,
) -> Result<LegacyJournalMigrationStore, PluginMutationJournalError> {
    LegacyJournalMigrationStore::new(path, max_bytes, staging_prefix)
        .map_err(|error| store_error("configure legacy authenticated journal store", error))
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
    let identity = codex_home_identity(codex_home)?;
    Ok(parent
        .join(".hepta-authority")
        .join(format!("plugin-mutation-{identity}.anchor")))
}

fn codex_home_identity(codex_home: &Path) -> Result<String, PluginMutationJournalError> {
    if !codex_home.is_absolute() {
        return Err(PluginMutationJournalError::new(
            "CODEX_HOME must be absolute for plugin mutation storage",
        ));
    }
    Ok(hex_encode(&Sha256::digest(
        codex_home.as_os_str().to_string_lossy().as_bytes(),
    )))
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

fn migration_claim_mac(
    key: &[u8; 32],
    domain: &[u8],
    phase: Option<MigrationPhase>,
    claim: &PluginMutationMigrationClaim,
) -> Result<String, PluginMutationJournalError> {
    let layout_version = claim.layout_version.to_be_bytes();
    let source = [match claim.source {
        MigrationSource::Fresh => 0,
        MigrationSource::LegacyV1 => 1,
        MigrationSource::LegacyV2 => 2,
    }];
    let generation = claim.generation.to_be_bytes();
    let phase = phase.map(|phase| match phase {
        MigrationPhase::Preparing => [0],
        MigrationPhase::Committed => [1],
    });
    let mut values: Vec<&[u8]> = Vec::with_capacity(7);
    values.push(&layout_version);
    values.push(claim.legacy_identity.as_bytes());
    values.push(claim.destination.as_bytes());
    values.push(&source);
    values.push(&generation);
    values.push(claim.state_hash.as_bytes());
    if let Some(phase) = phase.as_ref() {
        values.push(phase);
    }
    hmac_hex(key, domain, &values)
}

fn hex_decode(value: &str) -> Result<Vec<u8>, PluginMutationJournalError> {
    decode_canonical_hex(value)
        .map_err(|error| PluginMutationJournalError::new(format!("decode hex value: {error}")))
}

include!("plugin_mutation_journal/tests.rs");
