-- Hepta Intelligence P0.3.4 legacy grounding governance component migration 0012.
--
-- This schema is opt-in and qualification-only. It appends inventory,
-- backfill, and quarantine evidence without deleting or updating immutable
-- memory/KG facts and without changing a projection or recall pointer.

CREATE TABLE cognitive_legacy_grounding_migrations (
    version INTEGER PRIMARY KEY CHECK (version = 12),
    description TEXT NOT NULL CHECK (
        description = 'legacy grounding governance ledger'
    ),
    checksum_sha256 TEXT NOT NULL CHECK (
        length(checksum_sha256) = 64 AND
        checksum_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    applied_at_unix_seconds INTEGER NOT NULL
) STRICT;

CREATE TRIGGER cognitive_legacy_grounding_migrations_no_update
BEFORE UPDATE ON cognitive_legacy_grounding_migrations BEGIN
    SELECT RAISE(ABORT, 'legacy-grounding migration ledger is immutable');
END;

CREATE TRIGGER cognitive_legacy_grounding_migrations_no_delete
BEFORE DELETE ON cognitive_legacy_grounding_migrations BEGIN
    SELECT RAISE(ABORT, 'legacy-grounding migration ledger is immutable');
END;

CREATE TABLE kg_revision_fact_backfill_receipts (
    memory_id TEXT NOT NULL,
    memory_revision INTEGER NOT NULL CHECK (memory_revision > 0),
    fact_set_sha256 TEXT NOT NULL CHECK (
        length(fact_set_sha256) = 64 AND
        fact_set_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    grounding_receipt_sha256 TEXT NOT NULL CHECK (
        length(grounding_receipt_sha256) = 64 AND
        grounding_receipt_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    operator_receipt_sha256 TEXT NOT NULL CHECK (
        length(operator_receipt_sha256) = 64 AND
        operator_receipt_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    receipt_sha256 TEXT NOT NULL CHECK (
        length(receipt_sha256) = 64 AND
        receipt_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    recorded_at_unix_seconds INTEGER NOT NULL,
    PRIMARY KEY (memory_id, memory_revision),
    UNIQUE (memory_id, memory_revision, receipt_sha256),
    FOREIGN KEY (memory_id, memory_revision, fact_set_sha256)
        REFERENCES kg_revision_fact_sets(
            memory_id, memory_revision, fact_set_sha256
        ) ON DELETE RESTRICT,
    FOREIGN KEY (memory_id, memory_revision, grounding_receipt_sha256)
        REFERENCES kg_revision_fact_grounding_receipts(
            memory_id, memory_revision, receipt_sha256
        ) ON DELETE RESTRICT
) STRICT;

CREATE TRIGGER kg_revision_fact_backfill_receipts_guard
BEFORE INSERT ON kg_revision_fact_backfill_receipts
WHEN EXISTS (
    SELECT 1 FROM kg_revision_fact_quarantine_receipts AS q
    WHERE q.memory_id = NEW.memory_id
      AND q.memory_revision = NEW.memory_revision
) OR NOT EXISTS (
    SELECT 1 FROM kg_revision_fact_sets AS f
    WHERE f.memory_id = NEW.memory_id
      AND f.memory_revision = NEW.memory_revision
      AND f.fact_set_sha256 = NEW.fact_set_sha256
      AND (f.entity_count + f.relation_count) > 0
) BEGIN
    SELECT RAISE(ABORT, 'legacy backfill receipt binding is invalid');
END;

CREATE TRIGGER kg_revision_fact_backfill_receipts_no_update
BEFORE UPDATE ON kg_revision_fact_backfill_receipts BEGIN
    SELECT RAISE(ABORT, 'legacy backfill receipts are immutable');
END;

CREATE TRIGGER kg_revision_fact_backfill_receipts_no_delete
BEFORE DELETE ON kg_revision_fact_backfill_receipts BEGIN
    SELECT RAISE(ABORT, 'legacy backfill receipts are immutable');
END;

CREATE INDEX kg_revision_fact_backfill_receipts_digest_lookup
ON kg_revision_fact_backfill_receipts(
    receipt_sha256, grounding_receipt_sha256, memory_id, memory_revision
);

CREATE TABLE kg_revision_fact_quarantine_receipts (
    memory_id TEXT NOT NULL,
    memory_revision INTEGER NOT NULL CHECK (memory_revision > 0),
    fact_set_sha256 TEXT NOT NULL CHECK (
        length(fact_set_sha256) = 64 AND
        fact_set_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    reason_code TEXT NOT NULL CHECK (
        reason_code IN (
            'no_evidence',
            'ambiguous_evidence',
            'conflicting_evidence',
            'unsupported_fact',
            'operator_rejected'
        )
    ),
    operator_receipt_sha256 TEXT NOT NULL CHECK (
        length(operator_receipt_sha256) = 64 AND
        operator_receipt_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    receipt_sha256 TEXT NOT NULL CHECK (
        length(receipt_sha256) = 64 AND
        receipt_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    recorded_at_unix_seconds INTEGER NOT NULL,
    PRIMARY KEY (memory_id, memory_revision),
    UNIQUE (memory_id, memory_revision, receipt_sha256),
    FOREIGN KEY (memory_id, memory_revision, fact_set_sha256)
        REFERENCES kg_revision_fact_sets(
            memory_id, memory_revision, fact_set_sha256
        ) ON DELETE RESTRICT
) STRICT;

CREATE TRIGGER kg_revision_fact_quarantine_receipts_guard
BEFORE INSERT ON kg_revision_fact_quarantine_receipts
WHEN EXISTS (
    SELECT 1 FROM kg_revision_fact_grounding_receipts AS g
    WHERE g.memory_id = NEW.memory_id
      AND g.memory_revision = NEW.memory_revision
) OR EXISTS (
    SELECT 1 FROM kg_revision_fact_backfill_receipts AS b
    WHERE b.memory_id = NEW.memory_id
      AND b.memory_revision = NEW.memory_revision
) OR NOT EXISTS (
    SELECT 1 FROM kg_revision_fact_sets AS f
    WHERE f.memory_id = NEW.memory_id
      AND f.memory_revision = NEW.memory_revision
      AND f.fact_set_sha256 = NEW.fact_set_sha256
      AND (f.entity_count + f.relation_count) > 0
) BEGIN
    SELECT RAISE(ABORT, 'legacy quarantine receipt binding is invalid');
END;

CREATE TRIGGER kg_revision_fact_quarantine_receipts_no_update
BEFORE UPDATE ON kg_revision_fact_quarantine_receipts BEGIN
    SELECT RAISE(ABORT, 'legacy quarantine receipts are immutable');
END;

CREATE TRIGGER kg_revision_fact_quarantine_receipts_no_delete
BEFORE DELETE ON kg_revision_fact_quarantine_receipts BEGIN
    SELECT RAISE(ABORT, 'legacy quarantine receipts are immutable');
END;

CREATE INDEX kg_revision_fact_quarantine_receipts_digest_lookup
ON kg_revision_fact_quarantine_receipts(
    receipt_sha256, reason_code, memory_id, memory_revision
);

CREATE TABLE kg_legacy_grounding_inventory_receipts (
    inventory_receipt_sha256 TEXT PRIMARY KEY CHECK (
        length(inventory_receipt_sha256) = 64 AND
        inventory_receipt_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    owner_agent_id TEXT NOT NULL,
    projection_scope TEXT NOT NULL,
    item_count INTEGER NOT NULL CHECK (item_count BETWEEN 0 AND 10000),
    grounded_v1_count INTEGER NOT NULL CHECK (grounded_v1_count >= 0),
    backfilled_grounded_v1_count INTEGER NOT NULL CHECK (
        backfilled_grounded_v1_count >= 0
    ),
    legacy_unreviewed_count INTEGER NOT NULL CHECK (
        legacy_unreviewed_count >= 0
    ),
    quarantined_count INTEGER NOT NULL CHECK (quarantined_count >= 0),
    zero_fact_count INTEGER NOT NULL CHECK (zero_fact_count >= 0),
    entity_count INTEGER NOT NULL CHECK (entity_count >= 0),
    relation_count INTEGER NOT NULL CHECK (relation_count >= 0),
    oldest_observed_at_unix_seconds INTEGER,
    newest_observed_at_unix_seconds INTEGER,
    recorded_at_unix_seconds INTEGER NOT NULL,
    CHECK (
        item_count = grounded_v1_count + backfilled_grounded_v1_count +
                     legacy_unreviewed_count + quarantined_count + zero_fact_count
    ),
    CHECK (
        oldest_observed_at_unix_seconds IS NULL OR
        newest_observed_at_unix_seconds IS NULL OR
        oldest_observed_at_unix_seconds <= newest_observed_at_unix_seconds
    )
) STRICT;

CREATE TRIGGER kg_legacy_grounding_inventory_receipts_no_update
BEFORE UPDATE ON kg_legacy_grounding_inventory_receipts BEGIN
    SELECT RAISE(ABORT, 'legacy inventory receipts are immutable');
END;

CREATE TRIGGER kg_legacy_grounding_inventory_receipts_no_delete
BEFORE DELETE ON kg_legacy_grounding_inventory_receipts BEGIN
    SELECT RAISE(ABORT, 'legacy inventory receipts are immutable');
END;

CREATE TABLE kg_legacy_grounding_inventory_items (
    inventory_receipt_sha256 TEXT NOT NULL,
    item_ordinal INTEGER NOT NULL CHECK (item_ordinal >= 0),
    memory_id TEXT NOT NULL,
    memory_revision INTEGER NOT NULL CHECK (memory_revision > 0),
    projection_scope TEXT NOT NULL,
    fact_set_sha256 TEXT NOT NULL CHECK (
        length(fact_set_sha256) = 64 AND
        fact_set_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    grounding_status TEXT NOT NULL CHECK (
        grounding_status IN (
            'grounded_v1',
            'backfilled_grounded_v1',
            'legacy_unreviewed',
            'quarantined',
            'zero_fact'
        )
    ),
    entity_count INTEGER NOT NULL CHECK (entity_count >= 0),
    relation_count INTEGER NOT NULL CHECK (relation_count >= 0),
    source_kind TEXT NOT NULL,
    language_bucket TEXT NOT NULL CHECK (
        language_bucket IN ('ascii', 'cjk', 'other_unicode', 'mixed')
    ),
    observed_at_unix_seconds INTEGER NOT NULL,
    grounding_receipt_sha256 TEXT,
    backfill_receipt_sha256 TEXT,
    quarantine_receipt_sha256 TEXT,
    PRIMARY KEY (inventory_receipt_sha256, item_ordinal),
    UNIQUE (inventory_receipt_sha256, memory_id, memory_revision),
    FOREIGN KEY (inventory_receipt_sha256)
        REFERENCES kg_legacy_grounding_inventory_receipts(
            inventory_receipt_sha256
        ) ON DELETE RESTRICT,
    FOREIGN KEY (memory_id, memory_revision, fact_set_sha256)
        REFERENCES kg_revision_fact_sets(
            memory_id, memory_revision, fact_set_sha256
        ) ON DELETE RESTRICT,
    CHECK (
        grounding_receipt_sha256 IS NULL OR
        (length(grounding_receipt_sha256) = 64 AND
         grounding_receipt_sha256 NOT GLOB '*[^0-9a-f]*')
    ),
    CHECK (
        backfill_receipt_sha256 IS NULL OR
        (length(backfill_receipt_sha256) = 64 AND
         backfill_receipt_sha256 NOT GLOB '*[^0-9a-f]*')
    ),
    CHECK (
        quarantine_receipt_sha256 IS NULL OR
        (length(quarantine_receipt_sha256) = 64 AND
         quarantine_receipt_sha256 NOT GLOB '*[^0-9a-f]*')
    )
) STRICT;

CREATE TRIGGER kg_legacy_grounding_inventory_items_ordinal_guard
BEFORE INSERT ON kg_legacy_grounding_inventory_items
WHEN NEW.item_ordinal != (
    SELECT COUNT(*)
    FROM kg_legacy_grounding_inventory_items AS i
    WHERE i.inventory_receipt_sha256 = NEW.inventory_receipt_sha256
) BEGIN
    SELECT RAISE(ABORT, 'legacy inventory item ordinals must be contiguous');
END;

CREATE TRIGGER kg_legacy_grounding_inventory_items_no_update
BEFORE UPDATE ON kg_legacy_grounding_inventory_items BEGIN
    SELECT RAISE(ABORT, 'legacy inventory items are immutable');
END;

CREATE TRIGGER kg_legacy_grounding_inventory_items_no_delete
BEFORE DELETE ON kg_legacy_grounding_inventory_items BEGIN
    SELECT RAISE(ABORT, 'legacy inventory items are immutable');
END;

CREATE INDEX kg_legacy_grounding_inventory_items_status_lookup
ON kg_legacy_grounding_inventory_items(
    grounding_status, projection_scope, memory_id, memory_revision
);
