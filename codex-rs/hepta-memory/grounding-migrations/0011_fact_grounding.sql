-- Hepta Intelligence P0.2 durable fact-grounding component migration 0011.
--
-- This migration is intentionally applied by CognitiveStore's
-- open_with_durable_fact_grounding/ensure_durable_fact_grounding_schema path.
-- It does not activate the production projection gate or mutate legacy facts.

CREATE TABLE cognitive_fact_grounding_migrations (
    version INTEGER PRIMARY KEY CHECK (version = 11),
    description TEXT NOT NULL CHECK (
        description = 'durable fact grounding ledger'
    ),
    checksum_sha256 TEXT NOT NULL CHECK (
        length(checksum_sha256) = 64 AND
        checksum_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    applied_at_unix_seconds INTEGER NOT NULL
) STRICT;

CREATE TRIGGER cognitive_fact_grounding_migrations_no_update
BEFORE UPDATE ON cognitive_fact_grounding_migrations BEGIN
    SELECT RAISE(ABORT, 'fact-grounding migration ledger is immutable');
END;

CREATE TRIGGER cognitive_fact_grounding_migrations_no_delete
BEFORE DELETE ON cognitive_fact_grounding_migrations BEGIN
    SELECT RAISE(ABORT, 'fact-grounding migration ledger is immutable');
END;

CREATE TABLE kg_revision_fact_grounding_receipts (
    memory_id TEXT NOT NULL,
    memory_revision INTEGER NOT NULL CHECK (memory_revision > 0),
    grounding_contract TEXT NOT NULL CHECK (
        grounding_contract = 'source_span_grounding_v1'
    ),
    source_id TEXT NOT NULL,
    source_revision INTEGER NOT NULL CHECK (source_revision > 0),
    source_content_sha256 TEXT NOT NULL CHECK (
        length(source_content_sha256) = 64 AND
        source_content_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    fact_set_sha256 TEXT NOT NULL CHECK (
        length(fact_set_sha256) = 64 AND
        fact_set_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    fact_identity_sha256 TEXT NOT NULL CHECK (
        length(fact_identity_sha256) = 64 AND
        fact_identity_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    evidence_count INTEGER NOT NULL CHECK (evidence_count BETWEEN 1 AND 768),
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
    FOREIGN KEY (memory_id, memory_revision, source_id, source_revision)
        REFERENCES memory_citations(
            memory_id, memory_revision, source_id, source_revision
        ) ON DELETE RESTRICT
) STRICT;

CREATE TRIGGER kg_revision_fact_grounding_receipts_binding_guard
BEFORE INSERT ON kg_revision_fact_grounding_receipts
WHEN NOT EXISTS (
    SELECT 1
    FROM memory_revisions AS m
    JOIN source_ledger AS s
      ON s.source_id = NEW.source_id
     AND s.source_revision = NEW.source_revision
    JOIN kg_revision_fact_sets AS f
      ON f.memory_id = NEW.memory_id
     AND f.memory_revision = NEW.memory_revision
     AND f.fact_set_sha256 = NEW.fact_set_sha256
    WHERE m.memory_id = NEW.memory_id
      AND m.revision = NEW.memory_revision
      AND m.verification = 'verified'
      AND m.lifecycle = 'active'
      AND m.content_sha256 = NEW.source_content_sha256
      AND s.content_sha256 = NEW.source_content_sha256
      AND f.source_id = NEW.source_id
      AND f.source_revision = NEW.source_revision
      AND (f.entity_count + f.relation_count) > 0
      AND NEW.evidence_count >= (f.entity_count + f.relation_count)
      AND NEW.evidence_count <= 4 * (f.entity_count + f.relation_count)
) BEGIN
    SELECT RAISE(ABORT, 'fact-grounding receipt binding is invalid');
END;

CREATE TRIGGER kg_revision_fact_grounding_receipts_no_update
BEFORE UPDATE ON kg_revision_fact_grounding_receipts BEGIN
    SELECT RAISE(ABORT, 'fact-grounding receipts are immutable');
END;

CREATE TRIGGER kg_revision_fact_grounding_receipts_no_delete
BEFORE DELETE ON kg_revision_fact_grounding_receipts BEGIN
    SELECT RAISE(ABORT, 'fact-grounding receipts are immutable');
END;

CREATE INDEX kg_revision_fact_grounding_receipts_source_lookup
ON kg_revision_fact_grounding_receipts(
    source_id, source_revision, memory_id, memory_revision
);

CREATE INDEX kg_revision_fact_grounding_receipts_digest_lookup
ON kg_revision_fact_grounding_receipts(
    receipt_sha256, memory_id, memory_revision
);

CREATE TABLE kg_revision_fact_grounding_spans (
    memory_id TEXT NOT NULL,
    memory_revision INTEGER NOT NULL CHECK (memory_revision > 0),
    fact_kind TEXT NOT NULL CHECK (fact_kind IN ('entity', 'relation')),
    fact_key TEXT NOT NULL CHECK (
        length(trim(fact_key)) BETWEEN 1 AND 256 AND
        instr(fact_key, char(0)) = 0
    ),
    evidence_ordinal INTEGER NOT NULL CHECK (evidence_ordinal BETWEEN 0 AND 3),
    start_byte INTEGER NOT NULL CHECK (start_byte >= 0),
    end_byte INTEGER NOT NULL CHECK (end_byte > start_byte),
    evidence_sha256 TEXT NOT NULL CHECK (
        length(evidence_sha256) = 64 AND
        evidence_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    PRIMARY KEY (
        memory_id, memory_revision, fact_kind, fact_key, evidence_ordinal
    ),
    UNIQUE (
        memory_id, memory_revision, fact_kind, fact_key,
        start_byte, end_byte, evidence_sha256
    ),
    FOREIGN KEY (memory_id, memory_revision)
        REFERENCES kg_revision_fact_grounding_receipts(
            memory_id, memory_revision
        ) ON DELETE RESTRICT
) STRICT;

CREATE TRIGGER kg_revision_fact_grounding_spans_fact_guard
BEFORE INSERT ON kg_revision_fact_grounding_spans
WHEN (
    NEW.fact_kind = 'entity' AND NOT EXISTS (
        SELECT 1 FROM kg_revision_entities AS e
        WHERE e.memory_id = NEW.memory_id
          AND e.memory_revision = NEW.memory_revision
          AND e.entity_key = NEW.fact_key
    )
) OR (
    NEW.fact_kind = 'relation' AND NOT EXISTS (
        SELECT 1 FROM kg_revision_relations AS r
        WHERE r.memory_id = NEW.memory_id
          AND r.memory_revision = NEW.memory_revision
          AND r.relation_key = NEW.fact_key
    )
) BEGIN
    SELECT RAISE(ABORT, 'fact-grounding span references an unknown fact');
END;

CREATE TRIGGER kg_revision_fact_grounding_spans_range_guard
BEFORE INSERT ON kg_revision_fact_grounding_spans
WHEN NOT EXISTS (
    SELECT 1
    FROM kg_revision_fact_grounding_receipts AS g
    JOIN source_ledger AS s
      ON s.source_id = g.source_id
     AND s.source_revision = g.source_revision
    WHERE g.memory_id = NEW.memory_id
      AND g.memory_revision = NEW.memory_revision
      AND NEW.end_byte <= length(s.content)
) BEGIN
    SELECT RAISE(ABORT, 'fact-grounding span exceeds source bytes');
END;

CREATE TRIGGER kg_revision_fact_grounding_spans_ordinal_guard
BEFORE INSERT ON kg_revision_fact_grounding_spans
WHEN NEW.evidence_ordinal != (
    SELECT COUNT(*)
    FROM kg_revision_fact_grounding_spans AS p
    WHERE p.memory_id = NEW.memory_id
      AND p.memory_revision = NEW.memory_revision
      AND p.fact_kind = NEW.fact_kind
      AND p.fact_key = NEW.fact_key
) BEGIN
    SELECT RAISE(ABORT, 'fact-grounding span ordinals must be contiguous');
END;

CREATE TRIGGER kg_revision_fact_grounding_spans_total_guard
BEFORE INSERT ON kg_revision_fact_grounding_spans
WHEN (
    SELECT COUNT(*)
    FROM kg_revision_fact_grounding_spans AS p
    WHERE p.memory_id = NEW.memory_id
      AND p.memory_revision = NEW.memory_revision
) >= (
    SELECT evidence_count
    FROM kg_revision_fact_grounding_receipts AS g
    WHERE g.memory_id = NEW.memory_id
      AND g.memory_revision = NEW.memory_revision
) BEGIN
    SELECT RAISE(ABORT, 'fact-grounding spans exceed the receipt count');
END;

CREATE TRIGGER kg_revision_fact_grounding_spans_no_update
BEFORE UPDATE ON kg_revision_fact_grounding_spans BEGIN
    SELECT RAISE(ABORT, 'fact-grounding spans are immutable');
END;

CREATE TRIGGER kg_revision_fact_grounding_spans_no_delete
BEFORE DELETE ON kg_revision_fact_grounding_spans BEGIN
    SELECT RAISE(ABORT, 'fact-grounding spans are immutable');
END;

CREATE INDEX kg_revision_fact_grounding_spans_fact_lookup
ON kg_revision_fact_grounding_spans(
    memory_id, memory_revision, fact_kind, fact_key, evidence_ordinal
);

CREATE INDEX kg_revision_fact_grounding_spans_digest_lookup
ON kg_revision_fact_grounding_spans(
    evidence_sha256, memory_id, memory_revision
);
