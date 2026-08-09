-- seq and recorded_at_ms are non-authoritative operational metadata. Canonical
-- payloads, domain-separated hashes, and explicit foreign keys carry identity.
CREATE TABLE frozen_oracle_qualification_bindings (
    seq INTEGER PRIMARY KEY AUTOINCREMENT,
    qualification_run_id TEXT NOT NULL UNIQUE CHECK (
        length(qualification_run_id) = length('frozen-oracle-qualification:v1:') + 64
        AND substr(qualification_run_id, 1, length('frozen-oracle-qualification:v1:')) =
            'frozen-oracle-qualification:v1:'
        AND substr(qualification_run_id, length('frozen-oracle-qualification:v1:') + 1)
            NOT GLOB '*[^0-9a-f]*'
    ),
    schema_version INTEGER NOT NULL CHECK (schema_version = 1),
    candidate_commit TEXT NOT NULL CHECK (
        length(candidate_commit) IN (40, 64)
        AND candidate_commit NOT GLOB '*[^0-9a-f]*'
    ),
    candidate_tree TEXT NOT NULL CHECK (
        length(candidate_tree) IN (40, 64)
        AND candidate_tree NOT GLOB '*[^0-9a-f]*'
    ),
    frozen_oracle_commit TEXT NOT NULL CHECK (
        length(frozen_oracle_commit) IN (40, 64)
        AND frozen_oracle_commit NOT GLOB '*[^0-9a-f]*'
    ),
    frozen_oracle_tree TEXT NOT NULL CHECK (
        length(frozen_oracle_tree) IN (40, 64)
        AND frozen_oracle_tree NOT GLOB '*[^0-9a-f]*'
    ),
    frozen_oracle_manifest_sha256 TEXT NOT NULL CHECK (
        length(frozen_oracle_manifest_sha256) = 64
        AND frozen_oracle_manifest_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    canonical_oracle_corpus_sha256 TEXT NOT NULL CHECK (
        length(canonical_oracle_corpus_sha256) = 64
        AND canonical_oracle_corpus_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    qualification_nonce_sha256 TEXT NOT NULL CHECK (
        length(qualification_nonce_sha256) = 64
        AND qualification_nonce_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    required_sample_count INTEGER NOT NULL CHECK (required_sample_count = 252),
    qualification_run_started_at_ms INTEGER NOT NULL CHECK (qualification_run_started_at_ms >= 0),
    governance_mode TEXT NOT NULL CHECK (governance_mode = 'shadow'),
    enforce_enabled INTEGER NOT NULL CHECK (enforce_enabled = 0),
    qualification_only INTEGER NOT NULL CHECK (qualification_only = 1),
    promotion_authority_granted INTEGER NOT NULL
        CHECK (promotion_authority_granted = 0),
    outbound_enabled INTEGER NOT NULL CHECK (outbound_enabled = 0),
    memory_mutation_enabled INTEGER NOT NULL CHECK (memory_mutation_enabled = 0),
    proof_authority_enabled INTEGER NOT NULL CHECK (proof_authority_enabled = 0),
    retirement_authority_enabled INTEGER NOT NULL
        CHECK (retirement_authority_enabled = 0),
    binding_json TEXT NOT NULL CHECK (
        length(CAST(binding_json AS BLOB)) BETWEEN 2 AND 16384
        AND json_valid(binding_json)
    ),
    payload_sha256 TEXT NOT NULL CHECK (
        length(payload_sha256) = 64
        AND payload_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    binding_sha256 TEXT NOT NULL UNIQUE CHECK (
        length(binding_sha256) = 64
        AND binding_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    recorded_at_ms INTEGER NOT NULL CHECK (recorded_at_ms >= 0),
    UNIQUE(qualification_run_id, binding_sha256),
    UNIQUE(qualification_run_id, binding_sha256, canonical_oracle_corpus_sha256)
);

CREATE INDEX frozen_oracle_qualification_candidate_seq
    ON frozen_oracle_qualification_bindings(candidate_commit, seq);

CREATE INDEX frozen_oracle_qualification_oracle_seq
    ON frozen_oracle_qualification_bindings(frozen_oracle_commit, seq);

-- Mutable derived append state. It is not evidence: store open streams the
-- immutable observation chain and verifies this cursor from scratch.
CREATE TABLE frozen_oracle_qualification_heads (
    qualification_run_id TEXT PRIMARY KEY,
    binding_sha256 TEXT NOT NULL CHECK (
        length(binding_sha256) = 64
        AND binding_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    observation_count INTEGER NOT NULL
        CHECK (observation_count BETWEEN 0 AND 252),
    canonical_oracle_match_count INTEGER NOT NULL
        CHECK (canonical_oracle_match_count >= 0),
    canonical_oracle_divergence_count INTEGER NOT NULL
        CHECK (canonical_oracle_divergence_count >= 0),
    head_observation_sha256 TEXT NOT NULL CHECK (
        length(head_observation_sha256) = 64
        AND head_observation_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    CHECK (
        canonical_oracle_match_count + canonical_oracle_divergence_count =
            observation_count
    ),
    FOREIGN KEY(qualification_run_id, binding_sha256)
        REFERENCES frozen_oracle_qualification_bindings(
            qualification_run_id,
            binding_sha256
        )
        ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TABLE frozen_oracle_qualification_observations (
    seq INTEGER PRIMARY KEY AUTOINCREMENT,
    qualification_run_id TEXT NOT NULL,
    binding_sha256 TEXT NOT NULL CHECK (
        length(binding_sha256) = 64
        AND binding_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    canonical_oracle_corpus_sha256 TEXT NOT NULL CHECK (
        length(canonical_oracle_corpus_sha256) = 64
        AND canonical_oracle_corpus_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    ordinal INTEGER NOT NULL CHECK (ordinal BETWEEN 1 AND 252),
    sample_id_sha256 TEXT NOT NULL CHECK (
        length(sample_id_sha256) = 64
        AND sample_id_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    candidate_output_sha256 TEXT NOT NULL CHECK (
        length(candidate_output_sha256) = 64
        AND candidate_output_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    canonical_oracle_output_sha256 TEXT NOT NULL CHECK (
        length(canonical_oracle_output_sha256) = 64
        AND canonical_oracle_output_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    canonical_oracle_matched INTEGER NOT NULL
        CHECK (canonical_oracle_matched IN (0, 1)),
    previous_observation_sha256 TEXT NOT NULL CHECK (
        length(previous_observation_sha256) = 64
        AND previous_observation_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    observation_sha256 TEXT NOT NULL UNIQUE CHECK (
        length(observation_sha256) = 64
        AND observation_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    schema_version INTEGER NOT NULL CHECK (schema_version = 1),
    qualification_only INTEGER NOT NULL CHECK (qualification_only = 1),
    promotion_authority_granted INTEGER NOT NULL
        CHECK (promotion_authority_granted = 0),
    payload_json TEXT NOT NULL CHECK (
        length(CAST(payload_json AS BLOB)) BETWEEN 2 AND 16384
        AND json_valid(payload_json)
    ),
    payload_sha256 TEXT NOT NULL CHECK (
        length(payload_sha256) = 64
        AND payload_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    recorded_at_ms INTEGER NOT NULL CHECK (recorded_at_ms >= 0),
    CHECK (
        (canonical_oracle_matched = 1
            AND candidate_output_sha256 = canonical_oracle_output_sha256)
        OR
        (canonical_oracle_matched = 0
            AND candidate_output_sha256 <> canonical_oracle_output_sha256)
    ),
    UNIQUE(qualification_run_id, ordinal),
    UNIQUE(qualification_run_id, sample_id_sha256),
    FOREIGN KEY(qualification_run_id, binding_sha256, canonical_oracle_corpus_sha256)
        REFERENCES frozen_oracle_qualification_bindings(
            qualification_run_id,
            binding_sha256,
            canonical_oracle_corpus_sha256
        )
        ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE INDEX frozen_oracle_qualification_observations_run_seq
    ON frozen_oracle_qualification_observations(qualification_run_id, ordinal, seq);

CREATE TABLE frozen_oracle_qualification_terminals (
    seq INTEGER PRIMARY KEY AUTOINCREMENT,
    terminal_id TEXT NOT NULL UNIQUE CHECK (
        length(terminal_id) = length('frozen-oracle-qualification-terminal:v1:') + 64
        AND substr(terminal_id, 1, length('frozen-oracle-qualification-terminal:v1:')) =
            'frozen-oracle-qualification-terminal:v1:'
        AND substr(terminal_id, length('frozen-oracle-qualification-terminal:v1:') + 1)
            NOT GLOB '*[^0-9a-f]*'
    ),
    qualification_run_id TEXT NOT NULL UNIQUE,
    binding_sha256 TEXT NOT NULL CHECK (
        length(binding_sha256) = 64
        AND binding_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    canonical_oracle_corpus_sha256 TEXT NOT NULL CHECK (
        length(canonical_oracle_corpus_sha256) = 64
        AND canonical_oracle_corpus_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    conformance_status TEXT NOT NULL CHECK (conformance_status IN ('conformant', 'diverged')),
    observation_count INTEGER NOT NULL CHECK (observation_count = 252),
    canonical_oracle_match_count INTEGER NOT NULL CHECK (canonical_oracle_match_count >= 0),
    canonical_oracle_divergence_count INTEGER NOT NULL CHECK (canonical_oracle_divergence_count >= 0),
    head_observation_sha256 TEXT NOT NULL CHECK (
        length(head_observation_sha256) = 64
        AND head_observation_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    qualification_run_started_at_ms INTEGER NOT NULL CHECK (qualification_run_started_at_ms >= 0),
    qualification_run_finished_at_ms INTEGER NOT NULL CHECK (qualification_run_finished_at_ms >= qualification_run_started_at_ms),
    schema_version INTEGER NOT NULL CHECK (schema_version = 1),
    governance_mode TEXT NOT NULL CHECK (governance_mode = 'shadow'),
    enforce_enabled INTEGER NOT NULL CHECK (enforce_enabled = 0),
    qualification_only INTEGER NOT NULL CHECK (qualification_only = 1),
    promotion_authority_granted INTEGER NOT NULL
        CHECK (promotion_authority_granted = 0),
    terminal_sha256 TEXT NOT NULL UNIQUE CHECK (
        length(terminal_sha256) = 64
        AND terminal_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    payload_json TEXT NOT NULL CHECK (
        length(CAST(payload_json AS BLOB)) BETWEEN 2 AND 16384
        AND json_valid(payload_json)
    ),
    payload_sha256 TEXT NOT NULL CHECK (
        length(payload_sha256) = 64
        AND payload_sha256 NOT GLOB '*[^0-9a-f]*'
    ),
    recorded_at_ms INTEGER NOT NULL CHECK (recorded_at_ms >= 0),
    CHECK (canonical_oracle_match_count + canonical_oracle_divergence_count = observation_count),
    CHECK (
        (conformance_status = 'conformant' AND canonical_oracle_divergence_count = 0)
        OR
        (conformance_status = 'diverged' AND canonical_oracle_divergence_count > 0)
    ),
    FOREIGN KEY(qualification_run_id, binding_sha256, canonical_oracle_corpus_sha256)
        REFERENCES frozen_oracle_qualification_bindings(
            qualification_run_id,
            binding_sha256,
            canonical_oracle_corpus_sha256
        )
        ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TRIGGER frozen_oracle_qualification_bindings_no_update
BEFORE UPDATE ON frozen_oracle_qualification_bindings
BEGIN
    SELECT RAISE(ABORT, 'frozen-oracle qualification run bindings are immutable');
END;

CREATE TRIGGER frozen_oracle_qualification_bindings_no_delete
BEFORE DELETE ON frozen_oracle_qualification_bindings
BEGIN
    SELECT RAISE(ABORT, 'frozen-oracle qualification run bindings are immutable');
END;

CREATE TRIGGER frozen_oracle_qualification_heads_no_delete
BEFORE DELETE ON frozen_oracle_qualification_heads
BEGIN
    SELECT RAISE(ABORT, 'frozen-oracle qualification append heads cannot be deleted');
END;

CREATE TRIGGER frozen_oracle_qualification_observations_no_update
BEFORE UPDATE ON frozen_oracle_qualification_observations
BEGIN
    SELECT RAISE(ABORT, 'frozen-oracle qualification run observations are immutable');
END;

CREATE TRIGGER frozen_oracle_qualification_observations_no_delete
BEFORE DELETE ON frozen_oracle_qualification_observations
BEGIN
    SELECT RAISE(ABORT, 'frozen-oracle qualification run observations are immutable');
END;

CREATE TRIGGER frozen_oracle_qualification_terminals_no_update
BEFORE UPDATE ON frozen_oracle_qualification_terminals
BEGIN
    SELECT RAISE(ABORT, 'frozen-oracle qualification run terminals are immutable');
END;

CREATE TRIGGER frozen_oracle_qualification_terminals_no_delete
BEFORE DELETE ON frozen_oracle_qualification_terminals
BEGIN
    SELECT RAISE(ABORT, 'frozen-oracle qualification run terminals are immutable');
END;

CREATE TRIGGER frozen_oracle_qualification_observations_before_terminal
BEFORE INSERT ON frozen_oracle_qualification_observations
WHEN EXISTS (
    SELECT 1 FROM frozen_oracle_qualification_terminals
    WHERE qualification_run_id = NEW.qualification_run_id
)
BEGIN
    SELECT RAISE(ABORT, 'frozen-oracle qualification run is already terminal');
END;

CREATE TRIGGER frozen_oracle_qualification_observations_chain_guard
BEFORE INSERT ON frozen_oracle_qualification_observations
WHEN NOT EXISTS (
        SELECT 1
        FROM frozen_oracle_qualification_heads
        WHERE qualification_run_id = NEW.qualification_run_id
          AND binding_sha256 = NEW.binding_sha256
          AND observation_count + 1 = NEW.ordinal
          AND head_observation_sha256 = NEW.previous_observation_sha256
    )
    OR NEW.ordinal > (
        SELECT required_sample_count
        FROM frozen_oracle_qualification_bindings
        WHERE qualification_run_id = NEW.qualification_run_id
    )
BEGIN
    SELECT RAISE(ABORT, 'frozen-oracle qualification run chain is not contiguous');
END;

CREATE TRIGGER frozen_oracle_qualification_observations_advance_head
AFTER INSERT ON frozen_oracle_qualification_observations
BEGIN
    UPDATE frozen_oracle_qualification_heads
    SET observation_count = observation_count + 1,
        canonical_oracle_match_count =
            canonical_oracle_match_count + NEW.canonical_oracle_matched,
        canonical_oracle_divergence_count =
            canonical_oracle_divergence_count + (1 - NEW.canonical_oracle_matched),
        head_observation_sha256 = NEW.observation_sha256
    WHERE qualification_run_id = NEW.qualification_run_id
      AND binding_sha256 = NEW.binding_sha256
      AND observation_count + 1 = NEW.ordinal
      AND head_observation_sha256 = NEW.previous_observation_sha256;
    SELECT CASE WHEN changes() <> 1
        THEN RAISE(ABORT, 'frozen-oracle qualification append head did not advance')
    END;
END;

CREATE TRIGGER frozen_oracle_qualification_terminal_state_guard
BEFORE INSERT ON frozen_oracle_qualification_terminals
WHEN NOT EXISTS (
        SELECT 1
        FROM frozen_oracle_qualification_heads
        WHERE qualification_run_id = NEW.qualification_run_id
          AND binding_sha256 = NEW.binding_sha256
          AND observation_count = NEW.observation_count
          AND canonical_oracle_match_count = NEW.canonical_oracle_match_count
          AND canonical_oracle_divergence_count = NEW.canonical_oracle_divergence_count
          AND head_observation_sha256 = NEW.head_observation_sha256
    )
    OR NEW.observation_count <> (
        SELECT required_sample_count
        FROM frozen_oracle_qualification_bindings
        WHERE qualification_run_id = NEW.qualification_run_id
    )
    OR NEW.qualification_run_started_at_ms <> (
        SELECT qualification_run_started_at_ms
        FROM frozen_oracle_qualification_bindings
        WHERE qualification_run_id = NEW.qualification_run_id
    )
BEGIN
    SELECT RAISE(ABORT, 'frozen-oracle qualification run terminal does not anchor current chain');
END;
