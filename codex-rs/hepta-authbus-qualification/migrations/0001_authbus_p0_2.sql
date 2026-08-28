PRAGMA foreign_keys = ON;

CREATE TABLE authbus_p0_2_meta (
    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
    schema_version INTEGER NOT NULL CHECK (schema_version = 1),
    qualification_only INTEGER NOT NULL CHECK (qualification_only = 1),
    authority INTEGER NOT NULL CHECK (authority = 0),
    effect_authority INTEGER NOT NULL CHECK (effect_authority = 0),
    production_caller INTEGER NOT NULL CHECK (production_caller = 0),
    production_writer INTEGER NOT NULL CHECK (production_writer = 0),
    operator_acceptance INTEGER NOT NULL CHECK (operator_acceptance = 0),
    promotion INTEGER NOT NULL CHECK (promotion = 0),
    g5_allowed INTEGER NOT NULL CHECK (g5_allowed = 0),
    execute_allowed INTEGER NOT NULL CHECK (execute_allowed = 0),
    writer_boot_id TEXT NOT NULL,
    writer_generation INTEGER NOT NULL CHECK (writer_generation > 0),
    writer_epoch INTEGER NOT NULL CHECK (writer_epoch > 0),
    created_at_ms INTEGER NOT NULL CHECK (created_at_ms > 0),
    updated_at_ms INTEGER NOT NULL CHECK (updated_at_ms > 0)
) STRICT;

CREATE TABLE operations (
    operation_id TEXT PRIMARY KEY,
    operation_key TEXT NOT NULL UNIQUE,
    effect_key TEXT NOT NULL UNIQUE,
    idempotency_key TEXT NOT NULL UNIQUE,
    operation_kind TEXT NOT NULL CHECK (operation_kind IN ('REFRESH', 'ROTATE')),
    provider_id TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    token_family_id TEXT NOT NULL,
    intent_json TEXT NOT NULL,
    intent_sha256 TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN (
        'INTENT_DURABLE',
        'ATTEMPT_STARTED',
        'ACCEPTED',
        'UNKNOWN',
        'INDETERMINATE',
        'COMPLETED',
        'REJECTED',
        'QUARANTINED',
        'MANUAL_REQUIRED'
    )),
    revision INTEGER NOT NULL CHECK (revision > 0),
    attempt INTEGER NOT NULL CHECK (attempt >= 0),
    last_status_revision INTEGER,
    last_observed_at_ms INTEGER,
    authority_epoch INTEGER NOT NULL CHECK (authority_epoch > 0),
    owner_epoch INTEGER NOT NULL CHECK (owner_epoch > 0),
    generation INTEGER NOT NULL CHECK (generation > 0),
    fencing_token_sha256 TEXT NOT NULL,
    writer_boot_id TEXT NOT NULL,
    writer_generation INTEGER NOT NULL CHECK (writer_generation > 0),
    created_at_ms INTEGER NOT NULL CHECK (created_at_ms > 0),
    updated_at_ms INTEGER NOT NULL CHECK (updated_at_ms > 0),
    row_sha256 TEXT NOT NULL
) STRICT;

CREATE INDEX operations_state_idx ON operations(state, updated_at_ms);
CREATE INDEX operations_family_idx
    ON operations(provider_id, profile_id, token_family_id, state);

CREATE TABLE token_family_claims (
    claim_sha256 TEXT PRIMARY KEY,
    operation_id TEXT NOT NULL UNIQUE REFERENCES operations(operation_id) ON DELETE RESTRICT,
    active INTEGER NOT NULL CHECK (active IN (0, 1)),
    authority_epoch INTEGER NOT NULL CHECK (authority_epoch > 0),
    owner_epoch INTEGER NOT NULL CHECK (owner_epoch > 0),
    generation INTEGER NOT NULL CHECK (generation > 0),
    fencing_token_sha256 TEXT NOT NULL,
    acquired_at_ms INTEGER NOT NULL CHECK (acquired_at_ms > 0),
    released_at_ms INTEGER
) STRICT;

CREATE UNIQUE INDEX token_family_claims_one_active_idx
    ON token_family_claims(claim_sha256)
    WHERE active = 1;

CREATE TABLE quota_reservations (
    operation_id TEXT PRIMARY KEY REFERENCES operations(operation_id) ON DELETE RESTRICT,
    permit_id TEXT NOT NULL UNIQUE,
    resource_id TEXT NOT NULL,
    resource_sha256 TEXT NOT NULL,
    reserved_rpm INTEGER NOT NULL CHECK (reserved_rpm >= 0),
    reserved_tpm INTEGER NOT NULL CHECK (reserved_tpm >= 0),
    reserved_concurrency INTEGER NOT NULL CHECK (reserved_concurrency >= 0),
    reserved_day_budget INTEGER NOT NULL CHECK (reserved_day_budget >= 0),
    reserved_context INTEGER NOT NULL CHECK (reserved_context >= 0),
    used_rpm INTEGER NOT NULL CHECK (used_rpm >= 0),
    used_tpm INTEGER NOT NULL CHECK (used_tpm >= 0),
    used_concurrency INTEGER NOT NULL CHECK (used_concurrency >= 0),
    used_day_budget INTEGER NOT NULL CHECK (used_day_budget >= 0),
    used_context INTEGER NOT NULL CHECK (used_context >= 0),
    state TEXT NOT NULL CHECK (state IN ('HELD', 'COMPLETED', 'RELEASED')),
    revision INTEGER NOT NULL CHECK (revision > 0),
    updated_at_ms INTEGER NOT NULL CHECK (updated_at_ms > 0),
    row_sha256 TEXT NOT NULL
) STRICT;

CREATE TABLE dispatch_attempts (
    operation_id TEXT NOT NULL REFERENCES operations(operation_id) ON DELETE RESTRICT,
    attempt INTEGER NOT NULL CHECK (attempt > 0),
    operation_revision INTEGER NOT NULL CHECK (operation_revision > 0),
    writer_boot_id TEXT NOT NULL,
    writer_generation INTEGER NOT NULL CHECK (writer_generation > 0),
    authority_epoch INTEGER NOT NULL CHECK (authority_epoch > 0),
    owner_epoch INTEGER NOT NULL CHECK (owner_epoch > 0),
    generation INTEGER NOT NULL CHECK (generation > 0),
    fencing_token_sha256 TEXT NOT NULL,
    started_at_ms INTEGER NOT NULL CHECK (started_at_ms > 0),
    marker_kind TEXT,
    marker_json TEXT,
    marker_sha256 TEXT,
    marked_at_ms INTEGER,
    row_sha256 TEXT NOT NULL,
    PRIMARY KEY (operation_id, attempt)
) STRICT;

CREATE TABLE status_observations (
    operation_id TEXT NOT NULL REFERENCES operations(operation_id) ON DELETE RESTRICT,
    status_revision INTEGER NOT NULL CHECK (status_revision > 0),
    observed_at_ms INTEGER NOT NULL CHECK (observed_at_ms > 0),
    binding_sha256 TEXT NOT NULL,
    observation_json TEXT NOT NULL,
    observation_sha256 TEXT NOT NULL,
    writer_boot_id TEXT NOT NULL,
    writer_generation INTEGER NOT NULL CHECK (writer_generation > 0),
    created_at_ms INTEGER NOT NULL CHECK (created_at_ms > 0),
    PRIMARY KEY (operation_id, status_revision)
) STRICT;

CREATE TABLE outbox (
    sequence INTEGER PRIMARY KEY AUTOINCREMENT,
    outbox_id TEXT NOT NULL UNIQUE,
    operation_id TEXT NOT NULL REFERENCES operations(operation_id) ON DELETE RESTRICT,
    operation_revision INTEGER NOT NULL CHECK (operation_revision > 0),
    event_kind TEXT NOT NULL,
    idempotency_key TEXT NOT NULL UNIQUE,
    payload_sha256 TEXT NOT NULL,
    payload_json TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('PENDING', 'ACKED')),
    ack_sha256 TEXT,
    created_at_ms INTEGER NOT NULL CHECK (created_at_ms > 0),
    acked_at_ms INTEGER,
    row_sha256 TEXT NOT NULL
) STRICT;

CREATE INDEX outbox_pending_idx ON outbox(state, sequence);

CREATE TABLE outbox_cursor (
    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
    revision INTEGER NOT NULL CHECK (revision >= 0),
    last_sequence INTEGER NOT NULL CHECK (last_sequence >= 0),
    updated_at_ms INTEGER NOT NULL CHECK (updated_at_ms > 0)
) STRICT;

CREATE TABLE fsync_receipts (
    sequence INTEGER PRIMARY KEY AUTOINCREMENT,
    operation_id TEXT NOT NULL REFERENCES operations(operation_id) ON DELETE RESTRICT,
    phase TEXT NOT NULL CHECK (phase IN (
        'INTENT_DURABLE',
        'DISPATCH_ATTEMPT_DURABLE',
        'DISPATCH_MARKER_DURABLE',
        'STATUS_OBSERVATION_DURABLE',
        'OUTBOX_ACK_DURABLE'
    )),
    operation_revision INTEGER NOT NULL CHECK (operation_revision > 0),
    payload_sha256 TEXT NOT NULL,
    writer_boot_id TEXT NOT NULL,
    writer_generation INTEGER NOT NULL CHECK (writer_generation > 0),
    recorded_at_ms INTEGER NOT NULL CHECK (recorded_at_ms > 0),
    witness_sha256 TEXT NOT NULL,
    UNIQUE (operation_id, phase, operation_revision)
) STRICT;
