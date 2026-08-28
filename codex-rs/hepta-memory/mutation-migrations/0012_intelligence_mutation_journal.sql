-- Hepta Intelligence P0.4b qualification-only component migration.
--
-- This journal is opt-in. It does not join the canonical SQLx migration
-- lineage, change CognitiveStore::open, register a production caller, or
-- grant external-effect authority.

CREATE TABLE cognitive_intelligence_mutation_migrations (
    version INTEGER PRIMARY KEY CHECK (version = 12),
    description TEXT NOT NULL CHECK (description = 'intelligence mutation transition journal'),
    checksum_sha256 TEXT NOT NULL CHECK (
        length(checksum_sha256) = 64 AND checksum_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    applied_at_unix_seconds INTEGER NOT NULL
) STRICT;

CREATE TRIGGER cognitive_intelligence_mutation_migrations_no_update
BEFORE UPDATE ON cognitive_intelligence_mutation_migrations BEGIN
    SELECT RAISE(ABORT, 'intelligence mutation migration ledger is immutable');
END;

CREATE TRIGGER cognitive_intelligence_mutation_migrations_no_delete
BEFORE DELETE ON cognitive_intelligence_mutation_migrations BEGIN
    SELECT RAISE(ABORT, 'intelligence mutation migration ledger is immutable');
END;

CREATE TABLE cognitive_intelligence_mutation_operations (
    operation_id TEXT PRIMARY KEY CHECK (length(operation_id) BETWEEN 1 AND 256),
    owner_agent_id TEXT NOT NULL CHECK (length(owner_agent_id) = 36),
    lease_id TEXT NOT NULL CHECK (length(lease_id) BETWEEN 1 AND 256),
    lease_epoch INTEGER NOT NULL CHECK (lease_epoch > 0),
    expected_revision INTEGER CHECK (expected_revision IS NULL OR expected_revision > 0),
    starting_projection_generation INTEGER NOT NULL CHECK (starting_projection_generation >= 0),
    causal_root_sha256 TEXT NOT NULL CHECK (
        length(causal_root_sha256) = 64 AND causal_root_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    binding_sha256 TEXT NOT NULL CHECK (
        length(binding_sha256) = 64 AND binding_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    created_at_unix_seconds INTEGER NOT NULL
) STRICT;

CREATE TRIGGER cognitive_intelligence_mutation_operations_no_update
BEFORE UPDATE ON cognitive_intelligence_mutation_operations BEGIN
    SELECT RAISE(ABORT, 'intelligence mutation operations are immutable');
END;

CREATE TRIGGER cognitive_intelligence_mutation_operations_no_delete
BEFORE DELETE ON cognitive_intelligence_mutation_operations BEGIN
    SELECT RAISE(ABORT, 'intelligence mutation operations are immutable');
END;

CREATE INDEX cognitive_intelligence_mutation_operations_owner_lookup
ON cognitive_intelligence_mutation_operations(owner_agent_id, created_at_unix_seconds, operation_id);

CREATE UNIQUE INDEX cognitive_intelligence_mutation_operations_binding_lookup
ON cognitive_intelligence_mutation_operations(owner_agent_id, binding_sha256);

CREATE TABLE cognitive_intelligence_mutation_transitions (
    operation_id TEXT NOT NULL,
    sequence INTEGER NOT NULL CHECK (sequence >= 0),
    from_phase TEXT NOT NULL CHECK (from_phase IN (
        'planned', 'source_witnessed', 'grounding_validated',
        'durable_intent_appended', 'memory_facts_committed',
        'projection_published', 'outbox_settled', 'terminal',
        'indeterminate', 'reconciled_applied',
        'reconciled_not_applied', 'quarantined'
    )),
    to_phase TEXT NOT NULL CHECK (to_phase IN (
        'planned', 'source_witnessed', 'grounding_validated',
        'durable_intent_appended', 'memory_facts_committed',
        'projection_published', 'outbox_settled', 'terminal',
        'indeterminate', 'reconciled_applied',
        'reconciled_not_applied', 'quarantined'
    )),
    action TEXT NOT NULL CHECK (action IN (
        'witness_source', 'validate_grounding', 'append_durable_intent',
        'commit_memory_facts', 'publish_projection', 'settle_outbox',
        'terminalize', 'mark_indeterminate', 'reconcile_applied',
        'reconcile_not_applied', 'quarantine'
    )),
    action_payload_json TEXT NOT NULL CHECK (
        json_valid(action_payload_json) AND json_type(action_payload_json) = 'object'
    ),
    request_sha256 TEXT NOT NULL CHECK (
        length(request_sha256) = 64 AND request_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    causal_parent_sha256 TEXT CHECK (
        causal_parent_sha256 IS NULL OR
        (length(causal_parent_sha256) = 64 AND causal_parent_sha256 NOT GLOB '*[^0-9a-f]*')
    ),
    transition_sha256 TEXT NOT NULL CHECK (
        length(transition_sha256) = 64 AND transition_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    durable_intent_appended INTEGER NOT NULL CHECK (durable_intent_appended IN (0, 1)),
    durable_intent_settled INTEGER NOT NULL CHECK (durable_intent_settled IN (0, 1)),
    memory_write_count INTEGER NOT NULL CHECK (memory_write_count BETWEEN 0 AND 1),
    projection_publish_count INTEGER NOT NULL CHECK (projection_publish_count BETWEEN 0 AND 1),
    last_published_generation INTEGER NOT NULL CHECK (last_published_generation >= 0),
    recorded_at_unix_seconds INTEGER NOT NULL,
    PRIMARY KEY (operation_id, sequence),
    UNIQUE (transition_sha256),
    FOREIGN KEY (operation_id)
        REFERENCES cognitive_intelligence_mutation_operations(operation_id)
        ON DELETE RESTRICT
) STRICT;

CREATE TRIGGER cognitive_intelligence_mutation_transitions_no_update
BEFORE UPDATE ON cognitive_intelligence_mutation_transitions BEGIN
    SELECT RAISE(ABORT, 'intelligence mutation transitions are immutable');
END;

CREATE TRIGGER cognitive_intelligence_mutation_transitions_no_delete
BEFORE DELETE ON cognitive_intelligence_mutation_transitions BEGIN
    SELECT RAISE(ABORT, 'intelligence mutation transitions are immutable');
END;

CREATE TRIGGER cognitive_intelligence_mutation_transitions_chain_guard
BEFORE INSERT ON cognitive_intelligence_mutation_transitions BEGIN
    SELECT CASE
        WHEN NEW.sequence = 0 AND (
            NEW.from_phase != 'planned' OR NEW.causal_parent_sha256 IS NOT NULL
        )
        THEN RAISE(ABORT, 'genesis transition binding is invalid')
    END;
    SELECT CASE
        WHEN NEW.sequence > 0 AND NOT EXISTS (
            SELECT 1
            FROM cognitive_intelligence_mutation_transitions AS previous
            WHERE previous.operation_id = NEW.operation_id
              AND previous.sequence = NEW.sequence - 1
              AND previous.to_phase = NEW.from_phase
              AND previous.transition_sha256 = NEW.causal_parent_sha256
        )
        THEN RAISE(ABORT, 'transition sequence or causal parent is invalid')
    END;
    SELECT CASE
        WHEN EXISTS (
            SELECT 1
            FROM cognitive_intelligence_mutation_transitions AS later
            WHERE later.operation_id = NEW.operation_id
              AND later.sequence >= NEW.sequence
        )
        THEN RAISE(ABORT, 'transition sequence is not append-only')
    END;
    SELECT CASE
        WHEN NEW.durable_intent_settled > NEW.durable_intent_appended
        THEN RAISE(ABORT, 'settled intent requires an appended intent')
    END;
    SELECT CASE
        WHEN NEW.projection_publish_count > NEW.memory_write_count
        THEN RAISE(ABORT, 'projection publication requires a memory write')
    END;
    SELECT CASE
        WHEN NEW.to_phase IN ('terminal', 'reconciled_applied', 'reconciled_not_applied', 'quarantined')
             AND NEW.durable_intent_settled != 1
        THEN RAISE(ABORT, 'terminal resolution requires settled durable intent')
    END;
    SELECT CASE
        WHEN NEW.to_phase = 'terminal' AND (
            NEW.memory_write_count != 1 OR NEW.projection_publish_count != 1
        )
        THEN RAISE(ABORT, 'normal terminal state requires one write and one publication')
    END;
    SELECT CASE
        WHEN NEW.projection_publish_count = 0 AND NEW.last_published_generation != (
            SELECT starting_projection_generation
            FROM cognitive_intelligence_mutation_operations
            WHERE operation_id = NEW.operation_id
        )
        THEN RAISE(ABORT, 'generation changed without projection publication')
    END;
    SELECT CASE
        WHEN NEW.projection_publish_count = 1 AND NEW.last_published_generation != (
            SELECT starting_projection_generation + 1
            FROM cognitive_intelligence_mutation_operations
            WHERE operation_id = NEW.operation_id
        )
        THEN RAISE(ABORT, 'projection generation did not advance exactly once')
    END;
    SELECT CASE
        WHEN NEW.sequence > 0 AND EXISTS (
            SELECT 1
            FROM cognitive_intelligence_mutation_transitions AS previous
            WHERE previous.operation_id = NEW.operation_id
              AND previous.sequence = NEW.sequence - 1
              AND (
                  NEW.durable_intent_appended < previous.durable_intent_appended OR
                  NEW.durable_intent_settled < previous.durable_intent_settled OR
                  NEW.memory_write_count < previous.memory_write_count OR
                  NEW.projection_publish_count < previous.projection_publish_count OR
                  NEW.memory_write_count > previous.memory_write_count + 1 OR
                  NEW.projection_publish_count > previous.projection_publish_count + 1
              )
        )
        THEN RAISE(ABORT, 'transition counters are non-monotonic')
    END;
END;

CREATE UNIQUE INDEX cognitive_intelligence_mutation_transitions_digest_lookup
ON cognitive_intelligence_mutation_transitions(transition_sha256);

CREATE INDEX cognitive_intelligence_mutation_transitions_phase_lookup
ON cognitive_intelligence_mutation_transitions(operation_id, to_phase, sequence);
