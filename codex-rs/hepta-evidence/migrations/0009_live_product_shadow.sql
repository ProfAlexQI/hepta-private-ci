-- Qualification-only durable foundation for live product-Shadow evidence.
-- The corpus pinned by the corresponding Rust module proves parser-reachable
-- semantics only. These tables do not grant authority and this migration does
-- not expose a production importer or any path that can record a clean run.

CREATE TABLE live_product_shadow_v2_runs (
    run_id TEXT PRIMARY KEY CHECK (
        length(run_id) = 64 AND run_id NOT GLOB '*[^0-9a-f]*'
    ),
    schema_version INTEGER NOT NULL CHECK (schema_version = 2),
    run_binding_sha256 TEXT NOT NULL CHECK (
        length(run_binding_sha256) = 64
        AND run_binding_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    run_nonce_sha256 TEXT NOT NULL CHECK (
        length(run_nonce_sha256) = 64
        AND run_nonce_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    oracle_commit TEXT NOT NULL CHECK (
        oracle_commit = '2f704dc7c1172cefca908852456beccf4d02a5d1'
    ),
    oracle_tree TEXT NOT NULL CHECK (
        oracle_tree = '7be9a382b2610790838eef874cb4d381b5025490'
    ),
    oracle_manifest_sha256 TEXT NOT NULL CHECK (
        oracle_manifest_sha256 = '2c82d45303e912b92a7b9ac31da4661197e59a5ca415d3c70375b49169691377'
    ),
    oracle_corpus_sha256 TEXT NOT NULL CHECK (
        oracle_corpus_sha256 = 'dfe4f04d26895a6fabfb8435b77d7e807f57379fbb8d2a96c85af747e996cda7'
    ),
    oracle_generator_sha256 TEXT NOT NULL CHECK (
        oracle_generator_sha256 = '0778717e2ef2a9adfc7eb3c6980a8c2e7433e4ffbbbc6f124fb9e4098b4d1ab9'
    ),
    oracle_profile TEXT NOT NULL CHECK (
        oracle_profile = 'live_product_parser_reachable_semantics_v2'
    ),
    source_identity_status TEXT NOT NULL CHECK (
        source_identity_status = 'identity_claim'
    ),
    exact_verified INTEGER NOT NULL CHECK (exact_verified = 0),
    oracle_live_reachable INTEGER NOT NULL CHECK (oracle_live_reachable = 0),
    actual_live_trial_closure_required INTEGER NOT NULL CHECK (
        actual_live_trial_closure_required = 1
    ),
    strict_artifact_import_required INTEGER NOT NULL CHECK (
        strict_artifact_import_required = 1
    ),
    qualification_status TEXT NOT NULL CHECK (
        qualification_status = 'pending_strict_artifact_import'
    ),
    governance_mode TEXT NOT NULL CHECK (governance_mode = 'shadow'),
    enforce_enabled INTEGER NOT NULL CHECK (enforce_enabled = 0),
    promotion_authority_granted INTEGER NOT NULL CHECK (
        promotion_authority_granted = 0
    ),
    outbound_enabled INTEGER NOT NULL CHECK (outbound_enabled = 0),
    retirement_authority_granted INTEGER NOT NULL CHECK (
        retirement_authority_granted = 0
    ),
    operator_acceptance_recorded INTEGER NOT NULL CHECK (
        operator_acceptance_recorded = 0
    ),
    started_at_ms INTEGER NOT NULL CHECK (started_at_ms > 0),
    recorded_at_ms INTEGER NOT NULL CHECK (recorded_at_ms >= started_at_ms)
);

CREATE UNIQUE INDEX live_product_shadow_v2_runs_nonce
ON live_product_shadow_v2_runs(run_nonce_sha256);

CREATE TABLE live_product_shadow_v2_segments (
    segment_id TEXT PRIMARY KEY CHECK (
        length(segment_id) = 64 AND segment_id NOT GLOB '*[^0-9a-f]*'
    ),
    run_id TEXT NOT NULL,
    schema_version INTEGER NOT NULL CHECK (schema_version = 2),
    segment_ordinal INTEGER NOT NULL CHECK (segment_ordinal IN (1, 2)),
    surface TEXT NOT NULL CHECK (
        (segment_ordinal = 1 AND surface = 'app_server')
        OR (segment_ordinal = 2 AND surface = 'mcp')
    ),
    source_database_nonce_sha256 TEXT NOT NULL CHECK (
        length(source_database_nonce_sha256) = 64
        AND source_database_nonce_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    source_database_fresh INTEGER NOT NULL CHECK (source_database_fresh = 1),
    segment_binding_sha256 TEXT NOT NULL CHECK (
        length(segment_binding_sha256) = 64
        AND segment_binding_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    opened_at_ms INTEGER NOT NULL CHECK (opened_at_ms > 0),
    recorded_at_ms INTEGER NOT NULL CHECK (recorded_at_ms >= opened_at_ms),
    FOREIGN KEY (run_id)
        REFERENCES live_product_shadow_v2_runs(run_id)
        ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE UNIQUE INDEX live_product_shadow_v2_segments_run_ordinal
ON live_product_shadow_v2_segments(run_id, segment_ordinal);

CREATE UNIQUE INDEX live_product_shadow_v2_segments_run_surface
ON live_product_shadow_v2_segments(run_id, surface);

CREATE UNIQUE INDEX live_product_shadow_v2_segments_run_segment
ON live_product_shadow_v2_segments(run_id, segment_id);

CREATE UNIQUE INDEX live_product_shadow_v2_segments_database_nonce
ON live_product_shadow_v2_segments(source_database_nonce_sha256);

CREATE TABLE live_product_shadow_v2_pre_send_intents (
    intent_id TEXT PRIMARY KEY CHECK (
        length(intent_id) = 64 AND intent_id NOT GLOB '*[^0-9a-f]*'
    ),
    run_id TEXT NOT NULL,
    segment_id TEXT NOT NULL,
    schema_version INTEGER NOT NULL CHECK (schema_version = 2),
    intent_ordinal INTEGER NOT NULL CHECK (intent_ordinal BETWEEN 1 AND 2),
    previous_intent_sha256 TEXT NOT NULL CHECK (
        length(previous_intent_sha256) = 64
        AND previous_intent_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    sample_token_sha256 TEXT NOT NULL CHECK (
        length(sample_token_sha256) = 64
        AND sample_token_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    provider_request_semantic_sha256 TEXT NOT NULL CHECK (
        length(provider_request_semantic_sha256) = 64
        AND provider_request_semantic_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    intent_sha256 TEXT NOT NULL CHECK (
        length(intent_sha256) = 64
        AND intent_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    recorded_at_ms INTEGER NOT NULL CHECK (recorded_at_ms > 0),
    FOREIGN KEY (run_id, segment_id)
        REFERENCES live_product_shadow_v2_segments(run_id, segment_id)
        ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE UNIQUE INDEX live_product_shadow_v2_intents_segment_ordinal
ON live_product_shadow_v2_pre_send_intents(segment_id, intent_ordinal);

CREATE UNIQUE INDEX live_product_shadow_v2_intents_segment_sample
ON live_product_shadow_v2_pre_send_intents(segment_id, sample_token_sha256);

CREATE UNIQUE INDEX live_product_shadow_v2_intents_run_segment_intent
ON live_product_shadow_v2_pre_send_intents(run_id, segment_id, intent_id);

CREATE TABLE live_product_shadow_v2_artifact_imports (
    import_id TEXT PRIMARY KEY CHECK (
        length(import_id) = 64 AND import_id NOT GLOB '*[^0-9a-f]*'
    ),
    run_id TEXT NOT NULL,
    segment_id TEXT NOT NULL,
    intent_id TEXT NOT NULL,
    schema_version INTEGER NOT NULL CHECK (schema_version = 2),
    importer_schema TEXT NOT NULL CHECK (
        importer_schema = 'hepta_live_product_shadow_strict_artifact_import_v2'
    ),
    import_status TEXT NOT NULL CHECK (
        import_status IN ('strict_verified', 'rejected')
    ),
    artifact_path_sha256 TEXT NOT NULL CHECK (
        length(artifact_path_sha256) = 64
        AND artifact_path_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    stable_bundle_manifest_sha256 TEXT NOT NULL CHECK (
        length(stable_bundle_manifest_sha256) = 64
        AND stable_bundle_manifest_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    verification_snapshot_sha256 TEXT NOT NULL CHECK (
        length(verification_snapshot_sha256) = 64
        AND verification_snapshot_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    transcript_sha256 TEXT NOT NULL CHECK (
        length(transcript_sha256) = 64
        AND transcript_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    normalized_receipt_sha256 TEXT CHECK (
        normalized_receipt_sha256 IS NULL
        OR (
            length(normalized_receipt_sha256) = 64
            AND normalized_receipt_sha256 NOT GLOB '*[^0-9a-f]*'
        )
    ),
    oracle_sample_id_sha256 TEXT CHECK (
        oracle_sample_id_sha256 IS NULL
        OR (
            length(oracle_sample_id_sha256) = 64
            AND oracle_sample_id_sha256 NOT GLOB '*[^0-9a-f]*'
        )
    ),
    strict_artifact_validated INTEGER NOT NULL CHECK (
        strict_artifact_validated IN (0, 1)
    ),
    canonical_oracle_matched INTEGER NOT NULL CHECK (
        canonical_oracle_matched IN (0, 1)
    ),
    qualification_authority_granted INTEGER NOT NULL CHECK (
        qualification_authority_granted = 0
    ),
    import_sha256 TEXT NOT NULL CHECK (
        length(import_sha256) = 64
        AND import_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    imported_at_ms INTEGER NOT NULL CHECK (imported_at_ms > 0),
    CHECK (
        (
            import_status = 'strict_verified'
            AND strict_artifact_validated = 1
            AND canonical_oracle_matched = 1
            AND normalized_receipt_sha256 IS NOT NULL
            AND normalized_receipt_sha256 = '8904f0cc74e8a1b465eb75c7cd0c3f6ebef916c414dc9f5b6610d5822e9f68c0'
            AND oracle_sample_id_sha256 IS NOT NULL
            AND oracle_sample_id_sha256 = '426468e3c420e5557f2edbbb0adfc845b611c00416112c1ed95d99219fa9c5ef'
        )
        OR (
            import_status = 'rejected'
            AND strict_artifact_validated = 0
            AND canonical_oracle_matched = 0
            AND normalized_receipt_sha256 IS NULL
            AND oracle_sample_id_sha256 IS NULL
        )
    ),
    FOREIGN KEY (run_id, segment_id, intent_id)
        REFERENCES live_product_shadow_v2_pre_send_intents(run_id, segment_id, intent_id)
        ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE UNIQUE INDEX live_product_shadow_v2_imports_intent
ON live_product_shadow_v2_artifact_imports(intent_id);

CREATE INDEX live_product_shadow_v2_imports_run_segment
ON live_product_shadow_v2_artifact_imports(run_id, segment_id);

CREATE TABLE live_product_shadow_v2_terminals (
    terminal_id TEXT PRIMARY KEY CHECK (
        length(terminal_id) = 64 AND terminal_id NOT GLOB '*[^0-9a-f]*'
    ),
    run_id TEXT NOT NULL,
    schema_version INTEGER NOT NULL CHECK (schema_version = 2),
    terminal_status TEXT NOT NULL CHECK (
        terminal_status IN ('failed', 'incomplete', 'strict_artifact_import_complete')
    ),
    observed_intent_count INTEGER NOT NULL CHECK (observed_intent_count >= 0),
    observed_import_count INTEGER NOT NULL CHECK (observed_import_count >= 0),
    evidence_set_sha256 TEXT NOT NULL CHECK (
        length(evidence_set_sha256) = 64
        AND evidence_set_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    strict_imports_complete INTEGER NOT NULL CHECK (
        strict_imports_complete IN (0, 1)
    ),
    canonical_oracle_all_matched INTEGER NOT NULL CHECK (
        canonical_oracle_all_matched IN (0, 1)
    ),
    clean_qualified INTEGER NOT NULL CHECK (clean_qualified = 0),
    duration_claimed INTEGER NOT NULL CHECK (duration_claimed = 0),
    exact_verified INTEGER NOT NULL CHECK (exact_verified = 0),
    promotion_authority_granted INTEGER NOT NULL CHECK (
        promotion_authority_granted = 0
    ),
    operator_acceptance_recorded INTEGER NOT NULL CHECK (
        operator_acceptance_recorded = 0
    ),
    enforce_enabled INTEGER NOT NULL CHECK (enforce_enabled = 0),
    outbound_enabled INTEGER NOT NULL CHECK (outbound_enabled = 0),
    retirement_authority_granted INTEGER NOT NULL CHECK (
        retirement_authority_granted = 0
    ),
    terminal_sha256 TEXT NOT NULL CHECK (
        length(terminal_sha256) = 64
        AND terminal_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    recorded_at_ms INTEGER NOT NULL CHECK (recorded_at_ms > 0),
    CHECK (
        (
            terminal_status = 'strict_artifact_import_complete'
            AND strict_imports_complete = 1
            AND canonical_oracle_all_matched = 1
            AND observed_intent_count = 4
            AND observed_import_count = 4
        )
        OR (
            terminal_status IN ('failed', 'incomplete')
            AND strict_imports_complete = 0
            AND canonical_oracle_all_matched = 0
        )
    ),
    FOREIGN KEY (run_id)
        REFERENCES live_product_shadow_v2_runs(run_id)
        ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE UNIQUE INDEX live_product_shadow_v2_terminals_run
ON live_product_shadow_v2_terminals(run_id);

-- SQLite REPLACE deletes conflicting rows without firing DELETE triggers when
-- recursive_triggers is disabled on the invoking connection. These guards run
-- before conflict resolution and reject every primary/unique identity clash,
-- so neither REPLACE spelling can bypass the append-only boundary.
CREATE TRIGGER live_product_shadow_v2_runs_identity_collision_guard
BEFORE INSERT ON live_product_shadow_v2_runs
WHEN EXISTS (
    SELECT 1 FROM live_product_shadow_v2_runs
    WHERE run_id = NEW.run_id OR run_nonce_sha256 = NEW.run_nonce_sha256
)
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 run identity already exists');
END;

CREATE TRIGGER live_product_shadow_v2_segments_identity_collision_guard
BEFORE INSERT ON live_product_shadow_v2_segments
WHEN EXISTS (
    SELECT 1 FROM live_product_shadow_v2_segments
    WHERE segment_id = NEW.segment_id
       OR source_database_nonce_sha256 = NEW.source_database_nonce_sha256
       OR (run_id = NEW.run_id AND segment_ordinal = NEW.segment_ordinal)
       OR (run_id = NEW.run_id AND surface = NEW.surface)
)
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 segment identity already exists');
END;

CREATE TRIGGER live_product_shadow_v2_intents_identity_collision_guard
BEFORE INSERT ON live_product_shadow_v2_pre_send_intents
WHEN EXISTS (
    SELECT 1 FROM live_product_shadow_v2_pre_send_intents
    WHERE intent_id = NEW.intent_id
       OR (segment_id = NEW.segment_id AND intent_ordinal = NEW.intent_ordinal)
       OR (segment_id = NEW.segment_id AND sample_token_sha256 = NEW.sample_token_sha256)
)
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 pre-send intent identity already exists');
END;

CREATE TRIGGER live_product_shadow_v2_imports_identity_collision_guard
BEFORE INSERT ON live_product_shadow_v2_artifact_imports
WHEN EXISTS (
    SELECT 1 FROM live_product_shadow_v2_artifact_imports
    WHERE import_id = NEW.import_id OR intent_id = NEW.intent_id
)
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 artifact import identity already exists');
END;

CREATE TRIGGER live_product_shadow_v2_terminals_identity_collision_guard
BEFORE INSERT ON live_product_shadow_v2_terminals
WHEN EXISTS (
    SELECT 1 FROM live_product_shadow_v2_terminals
    WHERE terminal_id = NEW.terminal_id OR run_id = NEW.run_id
)
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 terminal identity already exists');
END;

CREATE TRIGGER live_product_shadow_v2_runs_no_update
BEFORE UPDATE ON live_product_shadow_v2_runs
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 runs are immutable');
END;

CREATE TRIGGER live_product_shadow_v2_runs_no_delete
BEFORE DELETE ON live_product_shadow_v2_runs
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 runs cannot be deleted');
END;

CREATE TRIGGER live_product_shadow_v2_segments_no_update
BEFORE UPDATE ON live_product_shadow_v2_segments
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 segments are immutable');
END;

CREATE TRIGGER live_product_shadow_v2_segments_no_delete
BEFORE DELETE ON live_product_shadow_v2_segments
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 segments cannot be deleted');
END;

CREATE TRIGGER live_product_shadow_v2_segments_before_terminal
BEFORE INSERT ON live_product_shadow_v2_segments
WHEN EXISTS (
    SELECT 1 FROM live_product_shadow_v2_terminals
    WHERE run_id = NEW.run_id
)
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 run is already terminal');
END;

CREATE TRIGGER live_product_shadow_v2_segments_chronology_guard
BEFORE INSERT ON live_product_shadow_v2_segments
WHEN NEW.opened_at_ms < COALESCE((
    SELECT recorded_at_ms FROM live_product_shadow_v2_runs
    WHERE run_id = NEW.run_id
), 9223372036854775807)
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 segment predates its run record');
END;

CREATE TRIGGER live_product_shadow_v2_intents_no_update
BEFORE UPDATE ON live_product_shadow_v2_pre_send_intents
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 pre-send intents are immutable');
END;

CREATE TRIGGER live_product_shadow_v2_intents_no_delete
BEFORE DELETE ON live_product_shadow_v2_pre_send_intents
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 pre-send intents cannot be deleted');
END;

CREATE TRIGGER live_product_shadow_v2_intents_chain_guard
BEFORE INSERT ON live_product_shadow_v2_pre_send_intents
WHEN
    EXISTS (
        SELECT 1 FROM live_product_shadow_v2_terminals
        WHERE run_id = NEW.run_id
    )
    OR NEW.intent_ordinal <> (
        SELECT COUNT(*) + 1 FROM live_product_shadow_v2_pre_send_intents
        WHERE segment_id = NEW.segment_id
    )
    OR (
        NEW.intent_ordinal = 1
        AND NEW.previous_intent_sha256 <> '0000000000000000000000000000000000000000000000000000000000000000'
    )
    OR (
        NEW.intent_ordinal > 1
        AND NEW.previous_intent_sha256 <> COALESCE((
            SELECT intent_sha256 FROM live_product_shadow_v2_pre_send_intents
            WHERE segment_id = NEW.segment_id
              AND intent_ordinal = NEW.intent_ordinal - 1
        ), '')
    )
    OR (
        NEW.intent_ordinal > 1
        AND NEW.recorded_at_ms < COALESCE((
            SELECT recorded_at_ms FROM live_product_shadow_v2_pre_send_intents
            WHERE segment_id = NEW.segment_id
              AND intent_ordinal = NEW.intent_ordinal - 1
        ), 9223372036854775807)
    )
    OR NEW.recorded_at_ms < COALESCE((
        SELECT recorded_at_ms FROM live_product_shadow_v2_segments
        WHERE run_id = NEW.run_id AND segment_id = NEW.segment_id
    ), 9223372036854775807)
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 pre-send intent chain is invalid');
END;

CREATE TRIGGER live_product_shadow_v2_imports_no_update
BEFORE UPDATE ON live_product_shadow_v2_artifact_imports
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 artifact imports are immutable');
END;

CREATE TRIGGER live_product_shadow_v2_imports_no_delete
BEFORE DELETE ON live_product_shadow_v2_artifact_imports
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 artifact imports cannot be deleted');
END;

CREATE TRIGGER live_product_shadow_v2_imports_before_terminal
BEFORE INSERT ON live_product_shadow_v2_artifact_imports
WHEN EXISTS (
    SELECT 1 FROM live_product_shadow_v2_terminals
    WHERE run_id = NEW.run_id
)
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 run is already terminal');
END;

CREATE TRIGGER live_product_shadow_v2_imports_chronology_guard
BEFORE INSERT ON live_product_shadow_v2_artifact_imports
WHEN NEW.imported_at_ms < COALESCE((
    SELECT recorded_at_ms FROM live_product_shadow_v2_pre_send_intents
    WHERE run_id = NEW.run_id
      AND segment_id = NEW.segment_id
      AND intent_id = NEW.intent_id
), 9223372036854775807)
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 artifact import predates its intent');
END;

CREATE TRIGGER live_product_shadow_v2_terminals_no_update
BEFORE UPDATE ON live_product_shadow_v2_terminals
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 terminals are immutable');
END;

CREATE TRIGGER live_product_shadow_v2_terminals_no_delete
BEFORE DELETE ON live_product_shadow_v2_terminals
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 terminals cannot be deleted');
END;

CREATE TRIGGER live_product_shadow_v2_terminals_state_guard
BEFORE INSERT ON live_product_shadow_v2_terminals
WHEN
    NEW.observed_intent_count <> (
        SELECT COUNT(*) FROM live_product_shadow_v2_pre_send_intents
        WHERE run_id = NEW.run_id
    )
    OR NEW.observed_import_count <> (
        SELECT COUNT(*) FROM live_product_shadow_v2_artifact_imports
        WHERE run_id = NEW.run_id
    )
    OR NEW.recorded_at_ms < COALESCE((
        SELECT recorded_at_ms FROM live_product_shadow_v2_runs
        WHERE run_id = NEW.run_id
    ), 9223372036854775807)
    OR NEW.recorded_at_ms < COALESCE((
        SELECT MAX(recorded_at_ms) FROM live_product_shadow_v2_segments
        WHERE run_id = NEW.run_id
    ), 0)
    OR NEW.recorded_at_ms < COALESCE((
        SELECT MAX(recorded_at_ms) FROM live_product_shadow_v2_pre_send_intents
        WHERE run_id = NEW.run_id
    ), 0)
    OR NEW.recorded_at_ms < COALESCE((
        SELECT MAX(imported_at_ms) FROM live_product_shadow_v2_artifact_imports
        WHERE run_id = NEW.run_id
    ), 0)
    OR (
        NEW.terminal_status = 'strict_artifact_import_complete'
        AND (
            (SELECT COUNT(*) FROM live_product_shadow_v2_segments
             WHERE run_id = NEW.run_id) <> 2
            OR (SELECT COUNT(*)
                FROM live_product_shadow_v2_pre_send_intents AS intent
                JOIN live_product_shadow_v2_segments AS segment
                  ON segment.segment_id = intent.segment_id
                WHERE intent.run_id = NEW.run_id
                  AND segment.surface = 'app_server') <> 2
            OR (SELECT COUNT(*)
                FROM live_product_shadow_v2_pre_send_intents AS intent
                JOIN live_product_shadow_v2_segments AS segment
                  ON segment.segment_id = intent.segment_id
                WHERE intent.run_id = NEW.run_id
                  AND segment.surface = 'mcp') <> 2
            OR EXISTS (
                SELECT 1
                FROM live_product_shadow_v2_pre_send_intents AS intent
                LEFT JOIN live_product_shadow_v2_artifact_imports AS imported
                  ON imported.intent_id = intent.intent_id
                WHERE intent.run_id = NEW.run_id
                  AND (
                      imported.import_id IS NULL
                      OR imported.import_status <> 'strict_verified'
                      OR imported.strict_artifact_validated <> 1
                      OR imported.canonical_oracle_matched <> 1
                  )
            )
        )
    )
BEGIN
    SELECT RAISE(ABORT, 'live product-Shadow v2 terminal does not match durable state');
END;
