from __future__ import annotations

from pathlib import Path
import re

ROOT = Path('.')


def load(path: str) -> tuple[Path, str]:
    file_path = ROOT / path
    return file_path, file_path.read_text(encoding='utf-8')


def save(file_path: Path, text: str) -> None:
    file_path.write_text(text, encoding='utf-8')


def replace_exact(path: str, old: str, new: str, *, expected: int = 1) -> None:
    file_path, text = load(path)
    count = text.count(old)
    if count != expected:
        raise AssertionError(f'{path}: expected {expected} copies, found {count}: {old[:100]!r}')
    save(file_path, text.replace(old, new, expected))


def expect_function(path: str, name: str, lint: str, reason: str) -> None:
    file_path, text = load(path)
    pattern = re.compile(
        r'(?m)^(?P<indent>[ \t]*)(?P<signature>'
        rf'(?:(?:pub(?:\([^\)]*\))?)[ \t]+)?'
        rf'(?:async[ \t]+)?fn[ \t]+{re.escape(name)}[ \t]*\('
    )
    matches = list(pattern.finditer(text))
    if len(matches) != 1:
        raise AssertionError(f'{path}: expected one function {name}, found {len(matches)}')
    match = matches[0]
    indent = match.group('indent')
    attribute = (
        f'{indent}#[expect(\n'
        f'{indent}    clippy::{lint},\n'
        f'{indent}    reason = "{reason}"\n'
        f'{indent})]\n'
    )
    text = text[: match.start()] + attribute + text[match.start() :]
    save(file_path, text)


TOO_MANY_ARGUMENTS = {
    'codex-rs/hepta-memory/src/h7_trajectory_store.rs': [
        'turn_start',
        'terminal',
        'verify_historical_lease',
        'insert_row',
    ],
    'codex-rs/hepta-memory/src/local_compact_executor.rs': [
        'historical_compact_lease_expiry',
        'verify_historical_compact_lease_binding',
    ],
    'codex-rs/hepta-memory/src/local_lease_outbox.rs': [
        'acquire_local_lease_after_head_bound',
        'acquire_host_bound_lease_after_head',
        'append_lease',
        'lease_digest',
        'event_digest',
        'outbox_digest',
    ],
    'codex-rs/hepta-memory/src/local_memory_saga.rs': [
        'tombstone_memory_candidate_saga',
        'replay_tombstone',
    ],
    'codex-rs/hepta-memory/src/logical_turn_registry.rs': [
        'append_attempt',
        'attempt_digest',
        'attempt_digest_without_scope',
    ],
    'codex-rs/hepta-memory/src/memory_admission.rs': [
        'verify_memory_candidate',
        'tombstone_memory_candidate',
    ],
    'codex-rs/hepta-memory/src/local_compact_executor_tests.rs': [
        'process_soak_child_command',
    ],
    'codex-rs/hepta-memory/src/local_lease_outbox_tests.rs': [
        'insert_test_transition',
        'test_event_digest',
    ],
    'codex-rs/hepta-memory/src/h7_feedback_tests.rs': [
        'record',
    ],
}

for file_name, functions in TOO_MANY_ARGUMENTS.items():
    for function in functions:
        expect_function(
            file_name,
            function,
            'too_many_arguments',
            'the frozen Q0 boundary keeps every persisted witness field explicit for auditability',
        )

# The reservation value intentionally owns complete verified attempt snapshots.
replace_exact(
    'codex-rs/hepta-memory/src/logical_turn_registry.rs',
    '#[derive(Clone, Debug, Eq, PartialEq)]\npub enum LogicalTurnReservation {',
    '#[expect(\n'
    '    clippy::large_enum_variant,\n'
    '    reason = "the frozen Q0 API owns complete verified attempt snapshots without hidden allocation"\n'
    ')]\n'
    '#[derive(Clone, Debug, Eq, PartialEq)]\n'
    'pub enum LogicalTurnReservation {',
)

# Serde JSON cannot fail for this tuple shape, but the From contract cannot
# expose a fallible result. Keep the invariant explicit without expect/unwrap.
replace_exact(
    'codex-rs/hepta-memory/src/h7_feedback.rs',
    '        serde_json::to_string(&(key.trajectory_id, key.event_seq, key.event_id))\n'
    '            .expect("feedback key tuple is serializable")',
    '        serde_json::Value::Array(vec![\n'
    '            serde_json::Value::String(key.trajectory_id),\n'
    '            serde_json::Value::Number(serde_json::Number::from(key.event_seq)),\n'
    '            serde_json::Value::String(key.event_id),\n'
    '        ])\n'
    '        .to_string()',
)

# A non-first row must have a predecessor. Turn the internal invariant into a
# corruption error instead of a process panic.
replace_exact(
    'codex-rs/hepta-memory/src/local_lease_outbox.rs',
    '        if index > 0 {\n'
    '            let prior = latest.as_ref().expect("lease prior row");',
    '        if index > 0 {\n'
    '            let Some(prior) = latest.as_ref() else {\n'
    '                return Err(corrupt("lease prior row is missing"));\n'
    '            };',
)

# These columns are consumed while verifying the chain and do not belong in
# the post-verification pairing records.
replace_exact(
    'codex-rs/hepta-memory/src/local_lease_outbox.rs',
    'struct EventRow {\n'
    '    sequence: u64,\n'
    '    event_id: String,\n'
    '    occurrence_key: String,\n'
    '    owner_agent_id: AgentId,\n'
    '    generation: u64,\n'
    '    fencing_token: String,\n'
    '    kind: String,\n'
    '    payload_json: String,\n'
    '    payload_sha256: Sha256Digest,\n'
    '    previous_sha256: Sha256Digest,\n'
    '    event_sha256: Sha256Digest,\n'
    '}',
    'struct EventRow {\n'
    '    event_id: String,\n'
    '    occurrence_key: String,\n'
    '    owner_agent_id: AgentId,\n'
    '    generation: u64,\n'
    '    fencing_token: String,\n'
    '    kind: String,\n'
    '    payload_json: String,\n'
    '    payload_sha256: Sha256Digest,\n'
    '}',
)
replace_exact(
    'codex-rs/hepta-memory/src/local_lease_outbox.rs',
    'struct OutboxRow {\n'
    '    sequence: u64,\n'
    '    outbox_id: String,\n'
    '    event_id: String,\n'
    '    occurrence_key: String,\n'
    '    owner_agent_id: AgentId,\n'
    '    generation: u64,\n'
    '    fencing_token: String,\n'
    '    topic: String,\n'
    '    payload_json: String,\n'
    '    payload_sha256: Sha256Digest,\n'
    '    previous_sha256: Sha256Digest,\n'
    '    outbox_sha256: Sha256Digest,\n'
    '}',
    'struct OutboxRow {\n'
    '    outbox_id: String,\n'
    '    event_id: String,\n'
    '    occurrence_key: String,\n'
    '    owner_agent_id: AgentId,\n'
    '    generation: u64,\n'
    '    fencing_token: String,\n'
    '    topic: String,\n'
    '    payload_json: String,\n'
    '    payload_sha256: Sha256Digest,\n'
    '}',
)
replace_exact(
    'codex-rs/hepta-memory/src/local_lease_outbox.rs',
    '        events.push(EventRow {\n'
    '            sequence,\n'
    '            event_id,\n'
    '            occurrence_key,\n'
    '            owner_agent_id: owner,\n'
    '            generation,\n'
    '            fencing_token,\n'
    '            kind,\n'
    '            payload_json,\n'
    '            payload_sha256,\n'
    '            previous_sha256,\n'
    '            event_sha256,\n'
    '        });',
    '        events.push(EventRow {\n'
    '            event_id,\n'
    '            occurrence_key,\n'
    '            owner_agent_id: owner,\n'
    '            generation,\n'
    '            fencing_token,\n'
    '            kind,\n'
    '            payload_json,\n'
    '            payload_sha256,\n'
    '        });',
)
replace_exact(
    'codex-rs/hepta-memory/src/local_lease_outbox.rs',
    '        previous = event_sha256.clone();',
    '        previous = event_sha256;',
)
replace_exact(
    'codex-rs/hepta-memory/src/local_lease_outbox.rs',
    '        previous = outbox_sha256.clone();',
    '        previous = outbox_sha256;',
)
replace_exact(
    'codex-rs/hepta-memory/src/local_lease_outbox.rs',
    '        outbox_rows.push(OutboxRow {\n'
    '            sequence,\n'
    '            outbox_id,\n'
    '            event_id,\n'
    '            occurrence_key,\n'
    '            owner_agent_id: owner,\n'
    '            generation,\n'
    '            fencing_token,\n'
    '            topic,\n'
    '            payload_json,\n'
    '            payload_sha256,\n'
    '            previous_sha256,\n'
    '            outbox_sha256,\n'
    '        });',
    '        outbox_rows.push(OutboxRow {\n'
    '            outbox_id,\n'
    '            event_id,\n'
    '            occurrence_key,\n'
    '            owner_agent_id: owner,\n'
    '            generation,\n'
    '            fencing_token,\n'
    '            topic,\n'
    '            payload_json,\n'
    '            payload_sha256,\n'
    '        });',
)

# Move the in-memory schema-oracle database behind the repository's central
# codex-state SQLite shim.
replace_exact(
    'codex-rs/state/src/sqlite.rs',
    '    /// Open a writable Codex SQLite database, creating it if necessary.\n'
    '    pub async fn open_read_write_pool(&self, path: &Path) -> Result<SqlitePool, Error> {',
    '    /// Open a single-connection in-memory SQLite database for schema oracles.\n'
    '    ///\n'
    '    /// A single connection is required because separate SQLite in-memory\n'
    '    /// connections do not share one database. Callers remain responsible\n'
    '    /// for applying their own scratch migrations.\n'
    '    pub async fn open_in_memory_pool() -> Result<SqlitePool, Error> {\n'
    '        let options = SqliteConnectOptions::new()\n'
    '            .in_memory(true)\n'
    '            .foreign_keys(true)\n'
    '            .log_statements(LevelFilter::Off);\n'
    '        SqlitePoolOptions::new()\n'
    '            .max_connections(1)\n'
    '            .connect_with(options)\n'
    '            .await\n'
    '    }\n\n'
    '    /// Open a writable Codex SQLite database, creating it if necessary.\n'
    '    pub async fn open_read_write_pool(&self, path: &Path) -> Result<SqlitePool, Error> {',
)
replace_exact(
    'codex-rs/hepta-memory/src/intelligence_mutation_journal_v3/schema.rs',
    'use codex_hepta_contracts::Sha256Digest;\n',
    'use codex_hepta_contracts::Sha256Digest;\nuse codex_state::SqliteConfig;\n',
)
replace_exact(
    'codex-rs/hepta-memory/src/intelligence_mutation_journal_v3/schema.rs',
    'use sqlx::sqlite::SqlitePoolOptions;\n',
    '',
)
replace_exact(
    'codex-rs/hepta-memory/src/intelligence_mutation_journal_v3/schema.rs',
    '    let scratch = SqlitePoolOptions::new()\n'
    '        .max_connections(1)\n'
    '        .connect("sqlite::memory:")\n'
    '        .await\n'
    '        .map_err(unavailable)?;',
    '    let scratch = SqliteConfig::open_in_memory_pool()\n'
    '        .await\n'
    '        .map_err(unavailable)?;',
)

# Preserve the negative authority contract as compile-time assertions instead
# of suppressing Clippy's constant-assertion lint.
CONST_BLOCK_REPLACEMENTS = {
    'codex-rs/hepta-memory/src/local_atomic_witness_tests.rs': (
        '    assert!(!LOCAL_ATOMIC_WITNESS_EXTERNAL_EFFECTS);\n'
        '    assert!(!LOCAL_ATOMIC_WITNESS_KG_WRITE_AUTHORITY);\n'
        '    assert!(!LOCAL_ATOMIC_WITNESS_LIFECYCLE_REGISTERED);\n'
        '    assert!(LOCAL_ATOMIC_WITNESS_LEASE_EPOCH_BOUND);\n'
        '    assert!(LOCAL_ATOMIC_WITNESS_LEASE_EXPIRY_BOUND);',
        '    const {\n'
        '        assert!(!LOCAL_ATOMIC_WITNESS_EXTERNAL_EFFECTS);\n'
        '        assert!(!LOCAL_ATOMIC_WITNESS_KG_WRITE_AUTHORITY);\n'
        '        assert!(!LOCAL_ATOMIC_WITNESS_LIFECYCLE_REGISTERED);\n'
        '        assert!(LOCAL_ATOMIC_WITNESS_LEASE_EPOCH_BOUND);\n'
        '        assert!(LOCAL_ATOMIC_WITNESS_LEASE_EXPIRY_BOUND);\n'
        '    }',
    ),
    'codex-rs/hepta-memory/src/compact_persistence_tests.rs': (
        '    assert!(!COMPACT_PERSISTENCE_EXTERNAL_EFFECTS);\n'
        '    assert!(!COMPACT_PERSISTENCE_KG_WRITE_AUTHORITY);',
        '    const {\n'
        '        assert!(!COMPACT_PERSISTENCE_EXTERNAL_EFFECTS);\n'
        '        assert!(!COMPACT_PERSISTENCE_KG_WRITE_AUTHORITY);\n'
        '    }',
    ),
    'codex-rs/hepta-memory/src/local_compact_executor_tests.rs': (
        '    assert!(!LOCAL_COMPACT_EXECUTOR_EXTERNAL_EFFECTS);\n'
        '    assert!(!LOCAL_COMPACT_EXECUTOR_KG_WRITE_AUTHORITY);',
        '    const {\n'
        '        assert!(!LOCAL_COMPACT_EXECUTOR_EXTERNAL_EFFECTS);\n'
        '        assert!(!LOCAL_COMPACT_EXECUTOR_KG_WRITE_AUTHORITY);\n'
        '    }',
    ),
    'codex-rs/hepta-memory/src/local_lease_outbox_tests.rs': (
        '    assert!(!LOCAL_LEASE_OUTBOX_EXTERNAL_EFFECTS);\n'
        '    assert!(!LOCAL_LEASE_OUTBOX_KG_WRITE_AUTHORITY);\n'
        '    assert!(!LOCAL_LEASE_OUTBOX_PRODUCTION_CALLER);',
        '    const {\n'
        '        assert!(!LOCAL_LEASE_OUTBOX_EXTERNAL_EFFECTS);\n'
        '        assert!(!LOCAL_LEASE_OUTBOX_KG_WRITE_AUTHORITY);\n'
        '        assert!(!LOCAL_LEASE_OUTBOX_PRODUCTION_CALLER);\n'
        '    }',
    ),
    'codex-rs/hepta-memory/src/logical_turn_registry_tests.rs': (
        '    assert!(!crate::LOGICAL_TURN_REGISTRY_EXTERNAL_EFFECTS);\n'
        '    assert!(!crate::LOGICAL_TURN_REGISTRY_KG_WRITE_AUTHORITY);\n'
        '    assert!(!crate::LOGICAL_TURN_REGISTRY_PRODUCTION_CALLER);',
        '    const {\n'
        '        assert!(!crate::LOGICAL_TURN_REGISTRY_EXTERNAL_EFFECTS);\n'
        '        assert!(!crate::LOGICAL_TURN_REGISTRY_KG_WRITE_AUTHORITY);\n'
        '        assert!(!crate::LOGICAL_TURN_REGISTRY_PRODUCTION_CALLER);\n'
        '    }',
    ),
    'codex-rs/hepta-memory/src/h7_feedback_tests.rs': (
        '    assert!(!H7_FEEDBACK_EXTERNAL_EFFECTS);\n'
        '    assert!(!H7_FEEDBACK_KG_WRITE_AUTHORITY);\n'
        '    assert!(!H7_FEEDBACK_PRODUCTION_CALLER);\n'
        '    assert!(H7_FEEDBACK_REPLAY_ONLY);',
        '    const {\n'
        '        assert!(!H7_FEEDBACK_EXTERNAL_EFFECTS);\n'
        '        assert!(!H7_FEEDBACK_KG_WRITE_AUTHORITY);\n'
        '        assert!(!H7_FEEDBACK_PRODUCTION_CALLER);\n'
        '        assert!(H7_FEEDBACK_REPLAY_ONLY);\n'
        '    }',
    ),
}
for file_name, (old, new) in CONST_BLOCK_REPLACEMENTS.items():
    replace_exact(file_name, old, new)

# Iterate over the witness vector directly so the operation index remains an
# audit label rather than a second, unchecked indexing path.
replace_exact(
    'codex-rs/hepta-memory/src/local_compact_executor_tests.rs',
    '''    for operation_index in 0..OPERATIONS {
        let operation_id = format!("op:seeded:{operation_index:04}");
        assert_eq!(
            reopened.state(&operation_id),
            Some(CompactPersistenceState::Committed)
        );
        assert_eq!(
            reopened
                .rehydration(&operation_id)
                .expect("final rehydration witness")
                .sequence,
            witness_sequences[operation_index]
        );
    }''',
    '''    for (operation_index, witness_sequence) in
        witness_sequences.iter().enumerate().take(OPERATIONS)
    {
        let operation_id = format!("op:seeded:{operation_index:04}");
        assert_eq!(
            reopened.state(&operation_id),
            Some(CompactPersistenceState::Committed)
        );
        assert_eq!(
            reopened
                .rehydration(&operation_id)
                .expect("final rehydration witness")
                .sequence,
            *witness_sequence
        );
    }''',
)
