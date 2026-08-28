PRAGMA foreign_keys = ON;

CREATE TABLE authbus_p1_2_meta (
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
    listener_enabled INTEGER NOT NULL CHECK (listener_enabled = 0),
    provider_call_enabled INTEGER NOT NULL CHECK (provider_call_enabled = 0),
    openbao_enabled INTEGER NOT NULL CHECK (openbao_enabled = 0),
    private_key_storage INTEGER NOT NULL CHECK (private_key_storage = 0),
    raw_signature_storage INTEGER NOT NULL CHECK (raw_signature_storage = 0),
    secret_storage INTEGER NOT NULL CHECK (secret_storage = 0),
    parent_workspace_wired INTEGER NOT NULL CHECK (parent_workspace_wired = 0),
    writer_boot_id TEXT NOT NULL,
    writer_generation INTEGER NOT NULL CHECK (writer_generation > 0),
    policy_json TEXT NOT NULL,
    policy_sha256 TEXT NOT NULL,
    created_at_unix_seconds INTEGER NOT NULL CHECK (created_at_unix_seconds > 0),
    updated_at_unix_seconds INTEGER NOT NULL CHECK (updated_at_unix_seconds > 0)
) STRICT;

CREATE TABLE p12_key_registrations (
    issuer_id TEXT NOT NULL,
    key_id TEXT NOT NULL,
    key_epoch INTEGER NOT NULL CHECK (key_epoch > 0),
    purpose TEXT NOT NULL CHECK (purpose IN (
        'IDENTITY_ISSUER',
        'PROVIDER_STATUS_ISSUER',
        'OPERATOR_EVIDENCE_ISSUER'
    )),
    record_json TEXT NOT NULL,
    record_sha256 TEXT NOT NULL,
    revoked_at_unix_seconds INTEGER,
    created_at_unix_seconds INTEGER NOT NULL CHECK (created_at_unix_seconds > 0),
    updated_at_unix_seconds INTEGER NOT NULL CHECK (updated_at_unix_seconds > 0),
    row_sha256 TEXT NOT NULL,
    PRIMARY KEY (issuer_id, key_id, key_epoch)
) STRICT;

CREATE INDEX p12_key_registrations_purpose_idx
    ON p12_key_registrations(issuer_id, purpose, key_epoch);

CREATE TABLE p12_key_heads (
    issuer_id TEXT NOT NULL,
    purpose TEXT NOT NULL CHECK (purpose IN (
        'IDENTITY_ISSUER',
        'PROVIDER_STATUS_ISSUER',
        'OPERATOR_EVIDENCE_ISSUER'
    )),
    current_key_id TEXT NOT NULL,
    current_key_epoch INTEGER NOT NULL CHECK (current_key_epoch > 0),
    updated_at_unix_seconds INTEGER NOT NULL CHECK (updated_at_unix_seconds > 0),
    row_sha256 TEXT NOT NULL,
    PRIMARY KEY (issuer_id, purpose),
    FOREIGN KEY (issuer_id, current_key_id, current_key_epoch)
        REFERENCES p12_key_registrations(issuer_id, key_id, key_epoch)
        ON DELETE RESTRICT
) STRICT;

CREATE TABLE p12_nonce_claims (
    nonce_key_sha256 TEXT PRIMARY KEY,
    claim_json TEXT NOT NULL,
    claim_sha256 TEXT NOT NULL,
    evidence_sha256 TEXT NOT NULL,
    binding_sha256 TEXT NOT NULL,
    subject_sha256 TEXT NOT NULL,
    nonce_sha256 TEXT NOT NULL,
    launch_nonce_sha256 TEXT NOT NULL,
    expires_at_unix_seconds INTEGER NOT NULL CHECK (expires_at_unix_seconds > 0),
    claimed_at_unix_seconds INTEGER NOT NULL CHECK (claimed_at_unix_seconds > 0),
    row_sha256 TEXT NOT NULL
) STRICT;

CREATE INDEX p12_nonce_claims_expiry_idx
    ON p12_nonce_claims(expires_at_unix_seconds);

CREATE TABLE p12_operations (
    operation_id TEXT PRIMARY KEY,
    binding_json TEXT NOT NULL,
    binding_sha256 TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN (
        'PENDING',
        'UNKNOWN',
        'INDETERMINATE',
        'LOOKUP_ONLY',
        'MANUAL_REQUIRED',
        'COMPLETED',
        'NO_EFFECT',
        'QUARANTINED'
    )),
    last_status_revision INTEGER,
    last_manual_revision INTEGER,
    last_status_sha256 TEXT,
    last_manual_sha256 TEXT,
    last_observed_at_unix_seconds INTEGER,
    record_revision INTEGER NOT NULL CHECK (record_revision > 0),
    created_at_unix_seconds INTEGER NOT NULL CHECK (created_at_unix_seconds > 0),
    updated_at_unix_seconds INTEGER NOT NULL CHECK (updated_at_unix_seconds > 0),
    row_sha256 TEXT NOT NULL
) STRICT;

CREATE INDEX p12_operations_state_idx
    ON p12_operations(state, updated_at_unix_seconds);

CREATE TABLE p12_status_evidence (
    operation_id TEXT NOT NULL REFERENCES p12_operations(operation_id) ON DELETE CASCADE,
    status_revision INTEGER NOT NULL CHECK (status_revision > 0),
    observation_json TEXT NOT NULL,
    observation_sha256 TEXT NOT NULL,
    evidence_sha256 TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN (
        'UNKNOWN',
        'INDETERMINATE',
        'MANUAL_REQUIRED',
        'COMPLETED',
        'NO_EFFECT',
        'QUARANTINED'
    )),
    observed_at_unix_seconds INTEGER NOT NULL CHECK (observed_at_unix_seconds > 0),
    row_sha256 TEXT NOT NULL,
    PRIMARY KEY (operation_id, status_revision)
) STRICT;

CREATE INDEX p12_status_evidence_age_idx
    ON p12_status_evidence(observed_at_unix_seconds);

CREATE TABLE p12_status_heads (
    operation_id TEXT PRIMARY KEY REFERENCES p12_operations(operation_id) ON DELETE CASCADE,
    status_revision INTEGER NOT NULL CHECK (status_revision > 0),
    evidence_sha256 TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN (
        'UNKNOWN',
        'INDETERMINATE',
        'MANUAL_REQUIRED',
        'COMPLETED',
        'NO_EFFECT',
        'QUARANTINED'
    )),
    observed_at_unix_seconds INTEGER NOT NULL CHECK (observed_at_unix_seconds > 0),
    row_sha256 TEXT NOT NULL,
    FOREIGN KEY (operation_id, status_revision)
        REFERENCES p12_status_evidence(operation_id, status_revision)
        ON DELETE CASCADE
) STRICT;

CREATE TABLE p12_manual_evidence (
    operation_id TEXT NOT NULL REFERENCES p12_operations(operation_id) ON DELETE CASCADE,
    manual_revision INTEGER NOT NULL CHECK (manual_revision > 0),
    observation_json TEXT NOT NULL,
    observation_sha256 TEXT NOT NULL,
    evidence_sha256 TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN (
        'LOOKUP_ONLY',
        'MANUAL_REQUIRED',
        'QUARANTINED'
    )),
    observed_at_unix_seconds INTEGER NOT NULL CHECK (observed_at_unix_seconds > 0),
    row_sha256 TEXT NOT NULL,
    PRIMARY KEY (operation_id, manual_revision)
) STRICT;

CREATE INDEX p12_manual_evidence_age_idx
    ON p12_manual_evidence(observed_at_unix_seconds);

CREATE TABLE p12_manual_heads (
    operation_id TEXT PRIMARY KEY REFERENCES p12_operations(operation_id) ON DELETE CASCADE,
    manual_revision INTEGER NOT NULL CHECK (manual_revision > 0),
    evidence_sha256 TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN (
        'LOOKUP_ONLY',
        'MANUAL_REQUIRED',
        'QUARANTINED'
    )),
    observed_at_unix_seconds INTEGER NOT NULL CHECK (observed_at_unix_seconds > 0),
    row_sha256 TEXT NOT NULL,
    FOREIGN KEY (operation_id, manual_revision)
        REFERENCES p12_manual_evidence(operation_id, manual_revision)
        ON DELETE CASCADE
) STRICT;

CREATE TABLE p12_terminal_tombstones (
    operation_id TEXT PRIMARY KEY REFERENCES p12_operations(operation_id) ON DELETE CASCADE,
    source_kind TEXT NOT NULL CHECK (source_kind IN ('PROVIDER_STATUS', 'MANUAL_EVIDENCE')),
    terminal_state TEXT NOT NULL CHECK (terminal_state IN (
        'COMPLETED',
        'NO_EFFECT',
        'QUARANTINED'
    )),
    evidence_sha256 TEXT NOT NULL,
    terminal_at_unix_seconds INTEGER NOT NULL CHECK (terminal_at_unix_seconds > 0),
    retain_until_unix_seconds INTEGER NOT NULL CHECK (
        retain_until_unix_seconds >= terminal_at_unix_seconds
    ),
    row_sha256 TEXT NOT NULL
) STRICT;

CREATE INDEX p12_terminal_tombstones_retention_idx
    ON p12_terminal_tombstones(retain_until_unix_seconds);

CREATE TABLE p12_durable_receipts (
    sequence INTEGER PRIMARY KEY AUTOINCREMENT,
    event_kind TEXT NOT NULL CHECK (event_kind IN (
        'WRITER_REBOUND',
        'KEY_REGISTERED',
        'KEY_REVOKED',
        'NONCE_CLAIMED',
        'OPERATION_REGISTERED',
        'STATUS_APPENDED',
        'MANUAL_APPENDED',
        'GC_COMMITTED'
    )),
    subject_id TEXT NOT NULL,
    subject_revision INTEGER NOT NULL CHECK (subject_revision >= 0),
    payload_sha256 TEXT NOT NULL,
    writer_boot_id TEXT NOT NULL,
    writer_generation INTEGER NOT NULL CHECK (writer_generation > 0),
    recorded_at_unix_seconds INTEGER NOT NULL CHECK (recorded_at_unix_seconds > 0),
    witness_sha256 TEXT NOT NULL,
    UNIQUE (event_kind, subject_id, subject_revision, payload_sha256)
) STRICT;

CREATE INDEX p12_durable_receipts_age_idx
    ON p12_durable_receipts(recorded_at_unix_seconds, sequence);

CREATE TABLE p12_gc_cursor (
    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
    revision INTEGER NOT NULL CHECK (revision >= 0),
    last_run_at_unix_seconds INTEGER NOT NULL CHECK (last_run_at_unix_seconds > 0),
    row_sha256 TEXT NOT NULL
) STRICT;
