-- Durable replay state is a fail-closed local ratchet. It validates evidence
-- identity and replay history only; possession of these rows does not record
-- operator acceptance and does not grant promotion authority.
CREATE TABLE promotion_trust_watermarks (
    trust_root_id TEXT PRIMARY KEY CHECK (
        length(CAST(trust_root_id AS BLOB)) BETWEEN 1 AND 128
        AND trust_root_id NOT GLOB '*[^A-Za-z0-9._:/-]*'
    ),
    schema_version INTEGER NOT NULL CHECK (schema_version = 1),
    checkpoint_source_json_sha256 TEXT NOT NULL CHECK (
        length(checkpoint_source_json_sha256) = 64
        AND checkpoint_source_json_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    checkpoint_sha256 TEXT NOT NULL CHECK (
        length(checkpoint_sha256) = 64
        AND checkpoint_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    genesis_trust_root_sha256 TEXT NOT NULL CHECK (
        length(genesis_trust_root_sha256) = 64
        AND genesis_trust_root_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    trust_root_revision INTEGER NOT NULL CHECK (trust_root_revision > 0),
    trust_root_sha256 TEXT NOT NULL CHECK (
        length(trust_root_sha256) = 64
        AND trust_root_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    revocation_revision INTEGER NOT NULL CHECK (revocation_revision > 0),
    revocations_sha256 TEXT NOT NULL CHECK (
        length(revocations_sha256) = 64
        AND revocations_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    history_chain_sha256 TEXT NOT NULL CHECK (
        length(history_chain_sha256) = 64
        AND history_chain_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    max_observed_time_unix_seconds INTEGER NOT NULL CHECK (
        max_observed_time_unix_seconds > 0
    )
);

CREATE TABLE promotion_revoked_key_tombstones (
    trust_root_id TEXT NOT NULL,
    revoked_key_id TEXT NOT NULL CHECK (
        length(CAST(revoked_key_id AS BLOB)) BETWEEN 1 AND 128
        AND revoked_key_id NOT GLOB '*[^A-Za-z0-9._:/-]*'
    ),
    durably_observed_revocation_revision INTEGER NOT NULL CHECK (
        durably_observed_revocation_revision > 0
    ),
    durably_observed_history_chain_sha256 TEXT NOT NULL CHECK (
        length(durably_observed_history_chain_sha256) = 64
        AND durably_observed_history_chain_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    PRIMARY KEY (trust_root_id, revoked_key_id),
    FOREIGN KEY (trust_root_id)
        REFERENCES promotion_trust_watermarks(trust_root_id)
        ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TABLE promotion_revoked_receipt_tombstones (
    trust_root_id TEXT NOT NULL,
    revoked_receipt_sha256 TEXT NOT NULL CHECK (
        length(revoked_receipt_sha256) = 64
        AND revoked_receipt_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    durably_observed_revocation_revision INTEGER NOT NULL CHECK (
        durably_observed_revocation_revision > 0
    ),
    durably_observed_history_chain_sha256 TEXT NOT NULL CHECK (
        length(durably_observed_history_chain_sha256) = 64
        AND durably_observed_history_chain_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    PRIMARY KEY (trust_root_id, revoked_receipt_sha256),
    FOREIGN KEY (trust_root_id)
        REFERENCES promotion_trust_watermarks(trust_root_id)
        ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TABLE promotion_revoked_nonce_tombstones (
    trust_root_id TEXT NOT NULL,
    revoked_nonce TEXT NOT NULL CHECK (
        length(revoked_nonce) = 64
        AND revoked_nonce NOT GLOB '*[^0-9a-f]*'
    ),
    durably_observed_revocation_revision INTEGER NOT NULL CHECK (
        durably_observed_revocation_revision > 0
    ),
    durably_observed_history_chain_sha256 TEXT NOT NULL CHECK (
        length(durably_observed_history_chain_sha256) = 64
        AND durably_observed_history_chain_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    PRIMARY KEY (trust_root_id, revoked_nonce),
    FOREIGN KEY (trust_root_id)
        REFERENCES promotion_trust_watermarks(trust_root_id)
        ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TABLE promotion_receipt_consumptions (
    trust_root_id TEXT NOT NULL,
    schema_version INTEGER NOT NULL CHECK (schema_version = 1),
    checkpoint_sha256 TEXT NOT NULL CHECK (
        length(checkpoint_sha256) = 64
        AND checkpoint_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    trust_root_revision INTEGER NOT NULL CHECK (trust_root_revision > 0),
    trust_root_sha256 TEXT NOT NULL CHECK (
        length(trust_root_sha256) = 64
        AND trust_root_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    revocation_revision INTEGER NOT NULL CHECK (revocation_revision > 0),
    revocations_sha256 TEXT NOT NULL CHECK (
        length(revocations_sha256) = 64
        AND revocations_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    history_chain_sha256 TEXT NOT NULL CHECK (
        length(history_chain_sha256) = 64
        AND history_chain_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    observed_at_unix_seconds INTEGER NOT NULL CHECK (observed_at_unix_seconds > 0),
    nonce TEXT NOT NULL CHECK (
        length(nonce) = 64
        AND nonce NOT GLOB '*[^0-9a-f]*'
    ),
    receipt_sha256 TEXT NOT NULL CHECK (
        length(receipt_sha256) = 64
        AND receipt_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    expires_at_unix_seconds INTEGER NOT NULL CHECK (
        expires_at_unix_seconds > observed_at_unix_seconds
    ),
    PRIMARY KEY (trust_root_id, receipt_sha256),
    UNIQUE (trust_root_id, nonce),
    FOREIGN KEY (trust_root_id)
        REFERENCES promotion_trust_watermarks(trust_root_id)
        ON UPDATE RESTRICT ON DELETE RESTRICT
);

-- A watermark may advance only through the application-level, verified
-- ancestry transaction. These triggers are defense in depth: SQLite cannot
-- authenticate the database or independently prove a signed history chain.
CREATE TRIGGER promotion_trust_watermarks_monotonic_update
BEFORE UPDATE ON promotion_trust_watermarks
WHEN
    NEW.trust_root_id <> OLD.trust_root_id
    OR NEW.schema_version <> OLD.schema_version
    OR NEW.genesis_trust_root_sha256 <> OLD.genesis_trust_root_sha256
    OR NEW.trust_root_revision < OLD.trust_root_revision
    OR NEW.revocation_revision < OLD.revocation_revision
    OR NEW.max_observed_time_unix_seconds < OLD.max_observed_time_unix_seconds
    OR (
        NEW.trust_root_revision = OLD.trust_root_revision
        AND NEW.trust_root_sha256 <> OLD.trust_root_sha256
    )
    OR (
        NEW.revocation_revision = OLD.revocation_revision
        AND NEW.revocations_sha256 <> OLD.revocations_sha256
    )
    OR (
        NEW.trust_root_revision = OLD.trust_root_revision
        AND NEW.revocation_revision = OLD.revocation_revision
        AND NEW.history_chain_sha256 <> OLD.history_chain_sha256
    )
BEGIN
    SELECT RAISE(ABORT, 'promotion trust watermark update is not monotonic');
END;

CREATE TRIGGER promotion_trust_watermarks_no_delete
BEFORE DELETE ON promotion_trust_watermarks
BEGIN
    SELECT RAISE(ABORT, 'promotion trust watermarks cannot be deleted');
END;

CREATE TRIGGER promotion_revoked_key_tombstones_no_update
BEFORE UPDATE ON promotion_revoked_key_tombstones
BEGIN
    SELECT RAISE(ABORT, 'promotion revoked key tombstones are immutable');
END;

CREATE TRIGGER promotion_revoked_key_tombstones_no_delete
BEFORE DELETE ON promotion_revoked_key_tombstones
BEGIN
    SELECT RAISE(ABORT, 'promotion revoked key tombstones are permanent');
END;

CREATE TRIGGER promotion_revoked_receipt_tombstones_no_update
BEFORE UPDATE ON promotion_revoked_receipt_tombstones
BEGIN
    SELECT RAISE(ABORT, 'promotion revoked receipt tombstones are immutable');
END;

CREATE TRIGGER promotion_revoked_receipt_tombstones_no_delete
BEFORE DELETE ON promotion_revoked_receipt_tombstones
BEGIN
    SELECT RAISE(ABORT, 'promotion revoked receipt tombstones are permanent');
END;

CREATE TRIGGER promotion_revoked_nonce_tombstones_no_update
BEFORE UPDATE ON promotion_revoked_nonce_tombstones
BEGIN
    SELECT RAISE(ABORT, 'promotion revoked nonce tombstones are immutable');
END;

CREATE TRIGGER promotion_revoked_nonce_tombstones_no_delete
BEFORE DELETE ON promotion_revoked_nonce_tombstones
BEGIN
    SELECT RAISE(ABORT, 'promotion revoked nonce tombstones are permanent');
END;

CREATE TRIGGER promotion_receipt_consumptions_no_update
BEFORE UPDATE ON promotion_receipt_consumptions
BEGIN
    SELECT RAISE(ABORT, 'promotion receipt consumptions are immutable');
END;

CREATE TRIGGER promotion_receipt_consumptions_no_delete
BEFORE DELETE ON promotion_receipt_consumptions
BEGIN
    SELECT RAISE(ABORT, 'promotion receipt consumptions are permanent');
END;
