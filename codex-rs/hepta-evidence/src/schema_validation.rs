use futures::TryStreamExt;
use sqlx::Row;
use sqlx::SqlitePool;

use codex_hepta_contracts::Sha256Digest;

use crate::EvidenceError;

pub(crate) async fn verify_quick_check(pool: &SqlitePool) -> Result<(), EvidenceError> {
    let results = sqlx::query_scalar::<_, String>("PRAGMA quick_check(1)")
        .fetch_all(pool)
        .await
        .map_err(classify_sqlx_error)?;
    if results.len() == 1 && results[0] == "ok" {
        Ok(())
    } else {
        Err(EvidenceError::Corrupt(
            "SQLite quick_check reported invalid evidence storage".to_string(),
        ))
    }
}

struct SchemaObjectSpec {
    name: &'static str,
    object_type: &'static str,
    table_name: &'static str,
    required_sql_fragments: &'static [&'static str],
}

struct ExactSchemaFingerprint {
    name: &'static str,
    normalized_sql_sha256: &'static str,
}

// These exact fingerprints prevent a no-op trigger from satisfying the
// descriptive substring checks by copying the expected words into comments or
// string literals. `seq` and wall-clock columns are deliberately absent from
// the trust boundary; immutable payloads and chain triggers carry identity.
const EXACT_FROZEN_ORACLE_TRIGGER_FINGERPRINTS: &[ExactSchemaFingerprint] = &[
    ExactSchemaFingerprint {
        name: "frozen_oracle_qualification_bindings_no_delete",
        normalized_sql_sha256: "a80a7ef023d170cc678851c74fe9ef34ba7ee2f9d0bafbad547602a8838095e6",
    },
    ExactSchemaFingerprint {
        name: "frozen_oracle_qualification_bindings_no_update",
        normalized_sql_sha256: "230b0c18307a1e6af5ba77646f06edcf9beafdb8dfba8e13e119fc1fab92e509",
    },
    ExactSchemaFingerprint {
        name: "frozen_oracle_qualification_heads_no_delete",
        normalized_sql_sha256: "c148f4a5fcc45ff6cd007f93cb4de9a7459f96bfcd388684c5f2050dbbe32e62",
    },
    ExactSchemaFingerprint {
        name: "frozen_oracle_qualification_observations_advance_head",
        normalized_sql_sha256: "6ea247083e48dd5a37fcf194fd6f42ed6ccef535c5f555faa506d84bc6f5450a",
    },
    ExactSchemaFingerprint {
        name: "frozen_oracle_qualification_observations_before_terminal",
        normalized_sql_sha256: "ee43398a29cf5d84a6ef4b19ba02379767f15d36fad113768b6ffc05106d9d28",
    },
    ExactSchemaFingerprint {
        name: "frozen_oracle_qualification_observations_chain_guard",
        normalized_sql_sha256: "cd26ce2ed8ebfe1b97ccf0a991e5d8223f0714221c9c63b5e8d935c548036b25",
    },
    ExactSchemaFingerprint {
        name: "frozen_oracle_qualification_observations_no_delete",
        normalized_sql_sha256: "8a578c64d09556fc0292ad1f9bc37b4c7df08f80b0eb4b18998e65dd31155841",
    },
    ExactSchemaFingerprint {
        name: "frozen_oracle_qualification_observations_no_update",
        normalized_sql_sha256: "5447f31c4537294fba4b54326cc9f5596a1c6b5701ba9e6357f312d7b38987a0",
    },
    ExactSchemaFingerprint {
        name: "frozen_oracle_qualification_terminal_state_guard",
        normalized_sql_sha256: "074c9ab0a5f487eafbbf077e0a8eb4d9e7dbf76dfba908219fd5323aa63f5686",
    },
    ExactSchemaFingerprint {
        name: "frozen_oracle_qualification_terminals_no_delete",
        normalized_sql_sha256: "f739c92b7d63304517c0dafb973ed7f264b350df0f72ff769d04c91a39cd1211",
    },
    ExactSchemaFingerprint {
        name: "frozen_oracle_qualification_terminals_no_update",
        normalized_sql_sha256: "0ef7635143b5154d81fb0cb2d54e92a2f526a9be714772ae4b40a34d92f15e36",
    },
];

// Every promotion replay table and mutation trigger is fingerprinted exactly.
// The database remains unauthenticated local state; these fingerprints prevent
// a structurally plausible replacement DDL from being opened as the expected
// fail-closed ratchet schema.
const EXACT_PROMOTION_REPLAY_SCHEMA_FINGERPRINTS: &[ExactSchemaFingerprint] = &[
    ExactSchemaFingerprint {
        name: "promotion_receipt_consumptions",
        normalized_sql_sha256: "f7126ce5834b4343b9d997aa82adc5a439efdbd6ee92eb9cd7c14451e37f9d3e",
    },
    ExactSchemaFingerprint {
        name: "promotion_receipt_consumptions_no_delete",
        normalized_sql_sha256: "55db38de0fd5879fd6bcba627fce97e27ff8eb1effee31ef24ef6a7e2588c38d",
    },
    ExactSchemaFingerprint {
        name: "promotion_receipt_consumptions_no_update",
        normalized_sql_sha256: "7e7194a8360b15c2b6c39e783c5d9e96d03c5b121f56c1182d0169ca196bb796",
    },
    ExactSchemaFingerprint {
        name: "promotion_revoked_key_tombstones",
        normalized_sql_sha256: "f73e2a6a65bec7b549df77ae8ef3c7f99bb37625b61ba87b4e1fc927dbcb46be",
    },
    ExactSchemaFingerprint {
        name: "promotion_revoked_key_tombstones_no_delete",
        normalized_sql_sha256: "d0f4b64bff4ac4d5c8cba24cae164053ecd85c02b98e0a8dcab0528a1386aa71",
    },
    ExactSchemaFingerprint {
        name: "promotion_revoked_key_tombstones_no_update",
        normalized_sql_sha256: "205b76cbf78f2e1241d383d1b17273fbc1adbaef88ddc239e4a9915e3bdc2a1b",
    },
    ExactSchemaFingerprint {
        name: "promotion_revoked_nonce_tombstones",
        normalized_sql_sha256: "d1e8db9212dbefa351b97943521a68d92b12c2956dcde9182044ab9e3ce58634",
    },
    ExactSchemaFingerprint {
        name: "promotion_revoked_nonce_tombstones_no_delete",
        normalized_sql_sha256: "e844207009fc81ee5ec6732d6e6b87828bbfbe320ae4d6cfa6507f55df26f04c",
    },
    ExactSchemaFingerprint {
        name: "promotion_revoked_nonce_tombstones_no_update",
        normalized_sql_sha256: "101985ef8894b1c0cc292e239bfa846bbd1023f7c172955f55a70c4a1ebcb249",
    },
    ExactSchemaFingerprint {
        name: "promotion_revoked_receipt_tombstones",
        normalized_sql_sha256: "cb7da4d52fa681d796d9f7d3995d57d2ff61613bdd593ae5ade3f5576fb8f955",
    },
    ExactSchemaFingerprint {
        name: "promotion_revoked_receipt_tombstones_no_delete",
        normalized_sql_sha256: "cb6932275e696b7cdced3f48367ebde6bca1a05ce745189653c4b571552b9cb6",
    },
    ExactSchemaFingerprint {
        name: "promotion_revoked_receipt_tombstones_no_update",
        normalized_sql_sha256: "bb5b84f7ad9bb4fc6e0c0862fbcf6c3db83366697bcd84cccf73753ecdd3d803",
    },
    ExactSchemaFingerprint {
        name: "promotion_trust_watermarks",
        normalized_sql_sha256: "f76c90e1bfa2cf7b748f08498b96a3740f82a2c7fd2b63493b0fab0eccb065b7",
    },
    ExactSchemaFingerprint {
        name: "promotion_trust_watermarks_monotonic_update",
        normalized_sql_sha256: "a2d62b0e125b1b948dec03b4978da739e093c73bd8e518d3b4586fed16c44803",
    },
    ExactSchemaFingerprint {
        name: "promotion_trust_watermarks_no_delete",
        normalized_sql_sha256: "b7c5923f7ce30b6398a048df945c726d8cca4c74a4718952493d3136944c12d3",
    },
];

const EXACT_PROMOTION_REPLAY_TRIGGER_SET: &[(&str, &str)] = &[
    (
        "promotion_receipt_consumptions",
        "promotion_receipt_consumptions_no_delete",
    ),
    (
        "promotion_receipt_consumptions",
        "promotion_receipt_consumptions_no_update",
    ),
    (
        "promotion_revoked_key_tombstones",
        "promotion_revoked_key_tombstones_no_delete",
    ),
    (
        "promotion_revoked_key_tombstones",
        "promotion_revoked_key_tombstones_no_update",
    ),
    (
        "promotion_revoked_nonce_tombstones",
        "promotion_revoked_nonce_tombstones_no_delete",
    ),
    (
        "promotion_revoked_nonce_tombstones",
        "promotion_revoked_nonce_tombstones_no_update",
    ),
    (
        "promotion_revoked_receipt_tombstones",
        "promotion_revoked_receipt_tombstones_no_delete",
    ),
    (
        "promotion_revoked_receipt_tombstones",
        "promotion_revoked_receipt_tombstones_no_update",
    ),
    (
        "promotion_trust_watermarks",
        "promotion_trust_watermarks_monotonic_update",
    ),
    (
        "promotion_trust_watermarks",
        "promotion_trust_watermarks_no_delete",
    ),
];

const REQUIRED_SCHEMA_OBJECTS: &[SchemaObjectSpec] = &[
    SchemaObjectSpec {
        name: "governance_decisions",
        object_type: "table",
        table_name: "governance_decisions",
        required_sql_fragments: &["create table", "governance_decisions"],
    },
    SchemaObjectSpec {
        name: "governance_receipts",
        object_type: "table",
        table_name: "governance_receipts",
        required_sql_fragments: &["create table", "governance_receipts"],
    },
    SchemaObjectSpec {
        name: "governance_decisions_thread_seq",
        object_type: "index",
        table_name: "governance_decisions",
        required_sql_fragments: &["create index", "governance_decisions", "thread_id", "seq"],
    },
    SchemaObjectSpec {
        name: "governance_receipts_thread_seq",
        object_type: "index",
        table_name: "governance_receipts",
        required_sql_fragments: &["create index", "governance_receipts", "thread_id", "seq"],
    },
    SchemaObjectSpec {
        name: "governance_decisions_no_update",
        object_type: "trigger",
        table_name: "governance_decisions",
        required_sql_fragments: &[
            "before update",
            "on governance_decisions",
            "raise(abort",
            "governance decisions are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "governance_decisions_no_delete",
        object_type: "trigger",
        table_name: "governance_decisions",
        required_sql_fragments: &[
            "before delete",
            "on governance_decisions",
            "raise(abort",
            "governance decisions are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "governance_receipts_no_update",
        object_type: "trigger",
        table_name: "governance_receipts",
        required_sql_fragments: &[
            "before update",
            "on governance_receipts",
            "raise(abort",
            "governance receipts are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "governance_receipts_no_delete",
        object_type: "trigger",
        table_name: "governance_receipts",
        required_sql_fragments: &[
            "before delete",
            "on governance_receipts",
            "raise(abort",
            "governance receipts are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_intents",
        object_type: "table",
        table_name: "provider_invocation_intents",
        required_sql_fragments: &[
            "create table",
            "provider_invocation_intents",
            "attempt_id",
            "request_binding_id",
            "host_request_binding_id_sha256",
            "ephemeral_input_sha256",
            "length(ephemeral_input_sha256) = 64",
            "ephemeral_input_sha256 not glob '*[^0-9a-f]*'",
            "ephemeral_input_witness_sha256",
            "length(ephemeral_input_witness_sha256) = 64",
            "ephemeral_input_witness_sha256 not glob '*[^0-9a-f]*'",
            "(ephemeral_input_sha256 is null) = (ephemeral_input_witness_sha256 is null)",
            "payload_sha256",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_terminals",
        object_type: "table",
        table_name: "provider_invocation_terminals",
        required_sql_fragments: &[
            "create table",
            "provider_invocation_terminals",
            "foreign key",
            "provider_invocation_intents",
            "on delete restrict",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_intents_thread_seq",
        object_type: "index",
        table_name: "provider_invocation_intents",
        required_sql_fragments: &[
            "create index",
            "provider_invocation_intents",
            "thread_id",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_intents_binding_seq",
        object_type: "index",
        table_name: "provider_invocation_intents",
        required_sql_fragments: &[
            "create index",
            "provider_invocation_intents",
            "request_binding_id",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_terminals_thread_seq",
        object_type: "index",
        table_name: "provider_invocation_terminals",
        required_sql_fragments: &[
            "create index",
            "provider_invocation_terminals",
            "thread_id",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_intents_host_binding_seq",
        object_type: "index",
        table_name: "provider_invocation_intents",
        required_sql_fragments: &[
            "create index",
            "provider_invocation_intents",
            "host_request_binding_id_sha256",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_intents_host_binding_required",
        object_type: "trigger",
        table_name: "provider_invocation_intents",
        required_sql_fragments: &[
            "before insert",
            "on provider_invocation_intents",
            "host_request_binding_id_sha256",
            "raise(abort",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_intents_no_update",
        object_type: "trigger",
        table_name: "provider_invocation_intents",
        required_sql_fragments: &[
            "before update",
            "on provider_invocation_intents",
            "raise(abort",
            "provider invocation intents are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_intents_no_delete",
        object_type: "trigger",
        table_name: "provider_invocation_intents",
        required_sql_fragments: &[
            "before delete",
            "on provider_invocation_intents",
            "raise(abort",
            "provider invocation intents are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_terminals_no_update",
        object_type: "trigger",
        table_name: "provider_invocation_terminals",
        required_sql_fragments: &[
            "before update",
            "on provider_invocation_terminals",
            "raise(abort",
            "provider invocation terminals are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_terminals_no_delete",
        object_type: "trigger",
        table_name: "provider_invocation_terminals",
        required_sql_fragments: &[
            "before delete",
            "on provider_invocation_terminals",
            "raise(abort",
            "provider invocation terminals are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "memory_mutation_shadow_observations",
        object_type: "table",
        table_name: "memory_mutation_shadow_observations",
        required_sql_fragments: &[
            "create table",
            "memory_mutation_shadow_observations",
            "dry_run_id",
            "proposal_id",
            "projected_memory_writes between 0 and 2",
            "unique(proposal_id, snapshot_sha256)",
            "disposition = 'blocked'",
            "reason = 'ready'",
            "evidence_sha256",
        ],
    },
    SchemaObjectSpec {
        name: "memory_mutation_shadow_proposal_seq",
        object_type: "index",
        table_name: "memory_mutation_shadow_observations",
        required_sql_fragments: &[
            "create index",
            "memory_mutation_shadow_observations",
            "proposal_id",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "memory_mutation_shadow_turn_seq",
        object_type: "index",
        table_name: "memory_mutation_shadow_observations",
        required_sql_fragments: &[
            "create index",
            "memory_mutation_shadow_observations",
            "turn_sha256",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "memory_mutation_shadow_no_update",
        object_type: "trigger",
        table_name: "memory_mutation_shadow_observations",
        required_sql_fragments: &[
            "before update",
            "on memory_mutation_shadow_observations",
            "raise(abort",
            "memory mutation shadow observations are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "memory_mutation_shadow_no_delete",
        object_type: "trigger",
        table_name: "memory_mutation_shadow_observations",
        required_sql_fragments: &[
            "before delete",
            "on memory_mutation_shadow_observations",
            "raise(abort",
            "memory mutation shadow observations are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "channel_ingress_events",
        object_type: "table",
        table_name: "channel_ingress_events",
        required_sql_fragments: &[
            "create table",
            "channel_ingress_events",
            "event_id text not null unique",
            "target_thread_sha256",
            "length(target_thread_sha256) = 64",
            "unique(scope_sha256, source_event_sha256)",
            "schema_version integer not null check (schema_version = 1)",
            "length(evidence_sha256) = 64",
            "evidence_sha256",
        ],
    },
    SchemaObjectSpec {
        name: "channel_ingress_receipts",
        object_type: "table",
        table_name: "channel_ingress_receipts",
        required_sql_fragments: &[
            "create table",
            "channel_ingress_receipts",
            "receipt_id text not null unique",
            "event_id text not null unique",
            "terminal_kind in ('accepted', 'rejected', 'indeterminate')",
            "terminal_kind = 'accepted' and thread_id is not null and turn_id is not null",
            "terminal_kind in ('rejected', 'indeterminate') and thread_id is null and turn_id is null",
            "schema_version integer not null check (schema_version = 1)",
            "length(evidence_sha256) = 64",
            "foreign key(event_id)",
            "channel_ingress_events(event_id)",
            "on update restrict",
            "on delete restrict",
        ],
    },
    SchemaObjectSpec {
        name: "channel_ingress_events_scope_seq",
        object_type: "index",
        table_name: "channel_ingress_events",
        required_sql_fragments: &[
            "create index",
            "channel_ingress_events",
            "scope_sha256",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "channel_ingress_receipts_scope_seq",
        object_type: "index",
        table_name: "channel_ingress_receipts",
        required_sql_fragments: &[
            "create index",
            "channel_ingress_receipts",
            "scope_sha256",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "channel_ingress_events_no_update",
        object_type: "trigger",
        table_name: "channel_ingress_events",
        required_sql_fragments: &[
            "before update",
            "on channel_ingress_events",
            "raise(abort",
            "channel ingress events are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "channel_ingress_events_no_delete",
        object_type: "trigger",
        table_name: "channel_ingress_events",
        required_sql_fragments: &[
            "before delete",
            "on channel_ingress_events",
            "raise(abort",
            "channel ingress events are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "channel_ingress_receipts_no_update",
        object_type: "trigger",
        table_name: "channel_ingress_receipts",
        required_sql_fragments: &[
            "before update",
            "on channel_ingress_receipts",
            "raise(abort",
            "channel ingress receipts are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "channel_ingress_receipts_no_delete",
        object_type: "trigger",
        table_name: "channel_ingress_receipts",
        required_sql_fragments: &[
            "before delete",
            "on channel_ingress_receipts",
            "raise(abort",
            "channel ingress receipts are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_bindings",
        object_type: "table",
        table_name: "frozen_oracle_qualification_bindings",
        required_sql_fragments: &[
            "create table",
            "frozen_oracle_qualification_bindings",
            "candidate_commit",
            "candidate_tree",
            "frozen_oracle_commit",
            "frozen_oracle_tree",
            "frozen_oracle_manifest_sha256",
            "canonical_oracle_corpus_sha256",
            "required_sample_count = 252",
            "qualification_run_started_at_ms",
            "governance_mode = 'shadow'",
            "enforce_enabled = 0",
            "qualification_only = 1",
            "promotion_authority_granted = 0",
            "outbound_enabled = 0",
            "memory_mutation_enabled = 0",
            "proof_authority_enabled = 0",
            "retirement_authority_enabled = 0",
            "length(cast(binding_json as blob)) between 2 and 16384",
            "json_valid(binding_json)",
            "unique(qualification_run_id, binding_sha256, canonical_oracle_corpus_sha256)",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_candidate_seq",
        object_type: "index",
        table_name: "frozen_oracle_qualification_bindings",
        required_sql_fragments: &[
            "create index",
            "frozen_oracle_qualification_bindings",
            "candidate_commit",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_oracle_seq",
        object_type: "index",
        table_name: "frozen_oracle_qualification_bindings",
        required_sql_fragments: &[
            "create index",
            "frozen_oracle_qualification_bindings",
            "frozen_oracle_commit",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_heads",
        object_type: "table",
        table_name: "frozen_oracle_qualification_heads",
        required_sql_fragments: &[
            "create table",
            "qualification_run_id text primary key",
            "observation_count between 0 and 252",
            "canonical_oracle_match_count + canonical_oracle_divergence_count",
            "head_observation_sha256",
            "foreign key(qualification_run_id, binding_sha256)",
            "frozen_oracle_qualification_bindings",
            "on update restrict",
            "on delete restrict",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_observations",
        object_type: "table",
        table_name: "frozen_oracle_qualification_observations",
        required_sql_fragments: &[
            "create table",
            "frozen_oracle_qualification_observations",
            "canonical_oracle_corpus_sha256",
            "ordinal between 1 and 252",
            "unique(qualification_run_id, ordinal)",
            "unique(qualification_run_id, sample_id_sha256)",
            "candidate_output_sha256 = canonical_oracle_output_sha256",
            "candidate_output_sha256 <> canonical_oracle_output_sha256",
            "canonical_oracle_matched in (0, 1)",
            "previous_observation_sha256",
            "observation_sha256",
            "qualification_only = 1",
            "promotion_authority_granted = 0",
            "length(cast(payload_json as blob)) between 2 and 16384",
            "json_valid(payload_json)",
            "foreign key(qualification_run_id, binding_sha256, canonical_oracle_corpus_sha256)",
            "frozen_oracle_qualification_bindings",
            "on update restrict",
            "on delete restrict",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_observations_run_seq",
        object_type: "index",
        table_name: "frozen_oracle_qualification_observations",
        required_sql_fragments: &[
            "create index",
            "frozen_oracle_qualification_observations",
            "qualification_run_id",
            "ordinal",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_terminals",
        object_type: "table",
        table_name: "frozen_oracle_qualification_terminals",
        required_sql_fragments: &[
            "create table",
            "frozen_oracle_qualification_terminals",
            "qualification_run_id text not null unique",
            "canonical_oracle_corpus_sha256",
            "conformance_status in ('conformant', 'diverged')",
            "observation_count = 252",
            "canonical_oracle_match_count + canonical_oracle_divergence_count = observation_count",
            "qualification_run_finished_at_ms >= qualification_run_started_at_ms",
            "governance_mode = 'shadow'",
            "enforce_enabled = 0",
            "qualification_only = 1",
            "promotion_authority_granted = 0",
            "length(cast(payload_json as blob)) between 2 and 16384",
            "json_valid(payload_json)",
            "foreign key(qualification_run_id, binding_sha256, canonical_oracle_corpus_sha256)",
            "frozen_oracle_qualification_bindings",
            "on update restrict",
            "on delete restrict",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_bindings_no_update",
        object_type: "trigger",
        table_name: "frozen_oracle_qualification_bindings",
        required_sql_fragments: &[
            "before update",
            "on frozen_oracle_qualification_bindings",
            "raise(abort",
            "frozen-oracle qualification run bindings are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_bindings_no_delete",
        object_type: "trigger",
        table_name: "frozen_oracle_qualification_bindings",
        required_sql_fragments: &[
            "before delete",
            "on frozen_oracle_qualification_bindings",
            "raise(abort",
            "frozen-oracle qualification run bindings are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_heads_no_delete",
        object_type: "trigger",
        table_name: "frozen_oracle_qualification_heads",
        required_sql_fragments: &[
            "before delete",
            "on frozen_oracle_qualification_heads",
            "raise(abort",
            "frozen-oracle qualification append heads cannot be deleted",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_observations_no_update",
        object_type: "trigger",
        table_name: "frozen_oracle_qualification_observations",
        required_sql_fragments: &[
            "before update",
            "on frozen_oracle_qualification_observations",
            "raise(abort",
            "frozen-oracle qualification run observations are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_observations_no_delete",
        object_type: "trigger",
        table_name: "frozen_oracle_qualification_observations",
        required_sql_fragments: &[
            "before delete",
            "on frozen_oracle_qualification_observations",
            "raise(abort",
            "frozen-oracle qualification run observations are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_terminals_no_update",
        object_type: "trigger",
        table_name: "frozen_oracle_qualification_terminals",
        required_sql_fragments: &[
            "before update",
            "on frozen_oracle_qualification_terminals",
            "raise(abort",
            "frozen-oracle qualification run terminals are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_terminals_no_delete",
        object_type: "trigger",
        table_name: "frozen_oracle_qualification_terminals",
        required_sql_fragments: &[
            "before delete",
            "on frozen_oracle_qualification_terminals",
            "raise(abort",
            "frozen-oracle qualification run terminals are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_observations_before_terminal",
        object_type: "trigger",
        table_name: "frozen_oracle_qualification_observations",
        required_sql_fragments: &[
            "before insert",
            "on frozen_oracle_qualification_observations",
            "frozen_oracle_qualification_terminals",
            "raise(abort",
            "frozen-oracle qualification run is already terminal",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_observations_chain_guard",
        object_type: "trigger",
        table_name: "frozen_oracle_qualification_observations",
        required_sql_fragments: &[
            "before insert",
            "on frozen_oracle_qualification_observations",
            "frozen_oracle_qualification_heads",
            "observation_count + 1 = new.ordinal",
            "new.ordinal >",
            "required_sample_count",
            "previous_observation_sha256",
            "head_observation_sha256 = new.previous_observation_sha256",
            "binding_sha256",
            "raise(abort",
            "frozen-oracle qualification run chain is not contiguous",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_observations_advance_head",
        object_type: "trigger",
        table_name: "frozen_oracle_qualification_observations",
        required_sql_fragments: &[
            "after insert",
            "on frozen_oracle_qualification_observations",
            "update frozen_oracle_qualification_heads",
            "canonical_oracle_match_count + new.canonical_oracle_matched",
            "canonical_oracle_divergence_count + (1 - new.canonical_oracle_matched)",
            "head_observation_sha256 = new.observation_sha256",
            "changes() <> 1",
            "frozen-oracle qualification append head did not advance",
        ],
    },
    SchemaObjectSpec {
        name: "frozen_oracle_qualification_terminal_state_guard",
        object_type: "trigger",
        table_name: "frozen_oracle_qualification_terminals",
        required_sql_fragments: &[
            "before insert",
            "on frozen_oracle_qualification_terminals",
            "frozen_oracle_qualification_heads",
            "observation_count",
            "canonical_oracle_match_count",
            "canonical_oracle_divergence_count",
            "head_observation_sha256",
            "required_sample_count",
            "qualification_run_started_at_ms",
            "raise(abort",
            "frozen-oracle qualification run terminal does not anchor current chain",
        ],
    },
    SchemaObjectSpec {
        name: "promotion_trust_watermarks",
        object_type: "table",
        table_name: "promotion_trust_watermarks",
        required_sql_fragments: &[
            "create table",
            "checkpoint_source_json_sha256",
            "checkpoint_sha256",
            "genesis_trust_root_sha256",
            "trust_root_revision",
            "trust_root_sha256",
            "revocation_revision",
            "revocations_sha256",
            "history_chain_sha256",
            "max_observed_time_unix_seconds",
        ],
    },
    SchemaObjectSpec {
        name: "promotion_revoked_key_tombstones",
        object_type: "table",
        table_name: "promotion_revoked_key_tombstones",
        required_sql_fragments: &[
            "create table",
            "revoked_key_id",
            "durably_observed_revocation_revision",
            "durably_observed_history_chain_sha256",
            "foreign key",
            "on update restrict",
            "on delete restrict",
        ],
    },
    SchemaObjectSpec {
        name: "promotion_revoked_receipt_tombstones",
        object_type: "table",
        table_name: "promotion_revoked_receipt_tombstones",
        required_sql_fragments: &[
            "create table",
            "revoked_receipt_sha256",
            "durably_observed_revocation_revision",
            "durably_observed_history_chain_sha256",
            "foreign key",
            "on update restrict",
            "on delete restrict",
        ],
    },
    SchemaObjectSpec {
        name: "promotion_revoked_nonce_tombstones",
        object_type: "table",
        table_name: "promotion_revoked_nonce_tombstones",
        required_sql_fragments: &[
            "create table",
            "revoked_nonce",
            "length(revoked_nonce) = 64",
            "durably_observed_revocation_revision",
            "durably_observed_history_chain_sha256",
            "foreign key",
            "on update restrict",
            "on delete restrict",
        ],
    },
    SchemaObjectSpec {
        name: "promotion_receipt_consumptions",
        object_type: "table",
        table_name: "promotion_receipt_consumptions",
        required_sql_fragments: &[
            "create table",
            "checkpoint_sha256",
            "trust_root_revision",
            "trust_root_sha256",
            "revocation_revision",
            "revocations_sha256",
            "history_chain_sha256",
            "observed_at_unix_seconds > 0",
            "length(nonce) = 64",
            "receipt_sha256",
            "expires_at_unix_seconds > observed_at_unix_seconds",
            "primary key (trust_root_id, receipt_sha256)",
            "unique (trust_root_id, nonce)",
        ],
    },
    SchemaObjectSpec {
        name: "promotion_trust_watermarks_monotonic_update",
        object_type: "trigger",
        table_name: "promotion_trust_watermarks",
        required_sql_fragments: &[
            "before update",
            "new.genesis_trust_root_sha256 <> old.genesis_trust_root_sha256",
            "new.trust_root_revision < old.trust_root_revision",
            "new.revocation_revision < old.revocation_revision",
            "new.max_observed_time_unix_seconds < old.max_observed_time_unix_seconds",
            "new.history_chain_sha256 <> old.history_chain_sha256",
            "raise(abort",
        ],
    },
    SchemaObjectSpec {
        name: "promotion_trust_watermarks_no_delete",
        object_type: "trigger",
        table_name: "promotion_trust_watermarks",
        required_sql_fragments: &["before delete", "raise(abort"],
    },
    SchemaObjectSpec {
        name: "promotion_revoked_key_tombstones_no_update",
        object_type: "trigger",
        table_name: "promotion_revoked_key_tombstones",
        required_sql_fragments: &["before update", "raise(abort"],
    },
    SchemaObjectSpec {
        name: "promotion_revoked_key_tombstones_no_delete",
        object_type: "trigger",
        table_name: "promotion_revoked_key_tombstones",
        required_sql_fragments: &["before delete", "raise(abort"],
    },
    SchemaObjectSpec {
        name: "promotion_revoked_receipt_tombstones_no_update",
        object_type: "trigger",
        table_name: "promotion_revoked_receipt_tombstones",
        required_sql_fragments: &["before update", "raise(abort"],
    },
    SchemaObjectSpec {
        name: "promotion_revoked_receipt_tombstones_no_delete",
        object_type: "trigger",
        table_name: "promotion_revoked_receipt_tombstones",
        required_sql_fragments: &["before delete", "raise(abort"],
    },
    SchemaObjectSpec {
        name: "promotion_revoked_nonce_tombstones_no_update",
        object_type: "trigger",
        table_name: "promotion_revoked_nonce_tombstones",
        required_sql_fragments: &["before update", "raise(abort"],
    },
    SchemaObjectSpec {
        name: "promotion_revoked_nonce_tombstones_no_delete",
        object_type: "trigger",
        table_name: "promotion_revoked_nonce_tombstones",
        required_sql_fragments: &["before delete", "raise(abort"],
    },
    SchemaObjectSpec {
        name: "promotion_receipt_consumptions_no_update",
        object_type: "trigger",
        table_name: "promotion_receipt_consumptions",
        required_sql_fragments: &["before update", "raise(abort"],
    },
    SchemaObjectSpec {
        name: "promotion_receipt_consumptions_no_delete",
        object_type: "trigger",
        table_name: "promotion_receipt_consumptions",
        required_sql_fragments: &["before delete", "raise(abort"],
    },
];

pub(crate) async fn verify_provider_host_bindings(pool: &SqlitePool) -> Result<(), EvidenceError> {
    let missing: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM provider_invocation_intents
         WHERE host_request_binding_id_sha256 IS NULL",
    )
    .fetch_one(pool)
    .await
    .map_err(classify_sqlx_error)?;
    if missing == 0 {
        Ok(())
    } else {
        Err(EvidenceError::Corrupt(format!(
            "{missing} provider intent rows predate host request binding evidence; explicit migration is required"
        )))
    }
}

pub(crate) async fn verify_provider_ephemeral_input_projection(
    pool: &SqlitePool,
) -> Result<(), EvidenceError> {
    let invalid: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM provider_invocation_intents
         WHERE CASE
             WHEN json_valid(payload_json) = 0 THEN 1
             WHEN json_type(payload_json, '$.binding.ephemeral_input_sha256') IS NULL
                  AND json_type(payload_json, '$.binding.ephemeral_input_witness_sha256') IS NULL
             THEN ephemeral_input_sha256 IS NOT NULL
                  OR ephemeral_input_witness_sha256 IS NOT NULL
             WHEN json_type(payload_json, '$.binding.ephemeral_input_sha256') = 'text'
                  AND json_type(payload_json, '$.binding.ephemeral_input_witness_sha256') = 'text'
             THEN ephemeral_input_sha256 IS NOT
                      json_extract(payload_json, '$.binding.ephemeral_input_sha256')
                  OR ephemeral_input_witness_sha256 IS NOT
                      json_extract(payload_json, '$.binding.ephemeral_input_witness_sha256')
             ELSE 1
         END",
    )
    .fetch_one(pool)
    .await
    .map_err(classify_sqlx_error)?;
    if invalid == 0 {
        Ok(())
    } else {
        Err(EvidenceError::Corrupt(format!(
            "{invalid} provider intent rows have invalid ephemeral input projections"
        )))
    }
}

pub(crate) async fn verify_schema_manifest(pool: &SqlitePool) -> Result<(), EvidenceError> {
    for spec in REQUIRED_SCHEMA_OBJECTS {
        let row = sqlx::query(
            "SELECT type AS object_type, tbl_name, sql
             FROM sqlite_schema WHERE name = ?",
        )
        .bind(spec.name)
        .fetch_optional(pool)
        .await
        .map_err(classify_sqlx_error)?
        .ok_or_else(|| {
            EvidenceError::Corrupt(format!(
                "required SQLite schema object {} is missing",
                spec.name
            ))
        })?;
        let object_type: String = row.get("object_type");
        let table_name: String = row.get("tbl_name");
        let sql: Option<String> = row.get("sql");
        let Some(sql) = sql else {
            return Err(EvidenceError::Corrupt(format!(
                "required SQLite schema object {} has no definition",
                spec.name
            )));
        };
        let normalized_sql = normalize_schema_sql(&sql);
        if object_type != spec.object_type
            || table_name != spec.table_name
            || spec
                .required_sql_fragments
                .iter()
                .any(|fragment| !normalized_sql.contains(fragment))
        {
            return Err(EvidenceError::Corrupt(format!(
                "required SQLite schema object {} has an invalid definition",
                spec.name
            )));
        }
    }
    verify_exact_promotion_replay_trigger_set(pool).await?;
    for fingerprint in EXACT_FROZEN_ORACLE_TRIGGER_FINGERPRINTS
        .iter()
        .chain(EXACT_PROMOTION_REPLAY_SCHEMA_FINGERPRINTS)
    {
        let sql =
            sqlx::query_scalar::<_, Option<String>>("SELECT sql FROM sqlite_schema WHERE name = ?")
                .bind(fingerprint.name)
                .fetch_optional(pool)
                .await
                .map_err(classify_sqlx_error)?
                .flatten()
                .ok_or_else(|| {
                    EvidenceError::Corrupt(format!(
                        "fingerprinted SQLite trigger or schema object {} is missing",
                        fingerprint.name
                    ))
                })?;
        let actual = Sha256Digest::for_bytes(normalize_schema_sql(&sql).as_bytes());
        if actual.as_str() != fingerprint.normalized_sql_sha256 {
            return Err(EvidenceError::Corrupt(format!(
                "fingerprinted SQLite trigger or schema object {} has an invalid canonical definition",
                fingerprint.name
            )));
        }
    }
    Ok(())
}

async fn verify_exact_promotion_replay_trigger_set(pool: &SqlitePool) -> Result<(), EvidenceError> {
    let mut rows = sqlx::query(
        "SELECT tbl_name, name
         FROM sqlite_schema
         WHERE type = 'trigger'
           AND tbl_name IN (
               'promotion_trust_watermarks',
               'promotion_revoked_key_tombstones',
               'promotion_revoked_receipt_tombstones',
               'promotion_revoked_nonce_tombstones',
               'promotion_receipt_consumptions'
           )
         ORDER BY tbl_name ASC, name ASC",
    )
    .fetch(pool);
    let mut index = 0usize;
    while let Some(row) = rows.try_next().await.map_err(classify_sqlx_error)? {
        let table_name: String = row.get("tbl_name");
        let trigger_name: String = row.get("name");
        let Some((expected_table, expected_trigger)) =
            EXACT_PROMOTION_REPLAY_TRIGGER_SET.get(index)
        else {
            return Err(EvidenceError::Corrupt(format!(
                "unexpected promotion replay trigger {trigger_name} exists on {table_name}"
            )));
        };
        if table_name != *expected_table || trigger_name != *expected_trigger {
            return Err(EvidenceError::Corrupt(format!(
                "promotion replay trigger set differs at {table_name}.{trigger_name}"
            )));
        }
        index += 1;
    }
    if index != EXACT_PROMOTION_REPLAY_TRIGGER_SET.len() {
        return Err(EvidenceError::Corrupt(format!(
            "promotion replay trigger set is incomplete: expected {}, found {index}",
            EXACT_PROMOTION_REPLAY_TRIGGER_SET.len()
        )));
    }
    Ok(())
}

fn normalize_schema_sql(sql: &str) -> String {
    sql.split_ascii_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

pub(crate) async fn verify_foreign_keys(pool: &SqlitePool) -> Result<(), EvidenceError> {
    let violation = sqlx::query("PRAGMA foreign_key_check")
        .fetch_optional(pool)
        .await
        .map_err(classify_sqlx_error)?;
    if violation.is_some() {
        Err(EvidenceError::Corrupt(
            "SQLite foreign_key_check found invalid evidence references".to_string(),
        ))
    } else {
        Ok(())
    }
}

pub(crate) fn classify_migrate_error(error: sqlx::migrate::MigrateError) -> EvidenceError {
    let detail = error.to_string();
    match error {
        sqlx::migrate::MigrateError::Execute(error) => classify_sqlx_error(error),
        sqlx::migrate::MigrateError::ExecuteMigration(error, version) => {
            classify_migration_execution_error(error, version)
        }
        sqlx::migrate::MigrateError::VersionMissing(_)
        | sqlx::migrate::MigrateError::VersionMismatch(_)
        | sqlx::migrate::MigrateError::VersionNotPresent(_)
        | sqlx::migrate::MigrateError::Dirty(_) => EvidenceError::Corrupt(detail),
        _ => EvidenceError::Unavailable(detail),
    }
}

fn classify_migration_execution_error(error: sqlx::Error, version: i64) -> EvidenceError {
    let detail = error.to_string();
    let invalid_ephemeral_backfill = version == 6
        && (sqlite_primary_code(&error) == Some(19)
            || detail.to_ascii_lowercase().contains("malformed json"));
    if invalid_ephemeral_backfill {
        EvidenceError::Corrupt(detail)
    } else {
        classify_sqlx_error(error)
    }
}

pub(crate) fn classify_sqlx_error(error: sqlx::Error) -> EvidenceError {
    let detail = error.to_string();
    match sqlite_primary_code(&error) {
        // SQLITE_CORRUPT, SQLITE_SCHEMA, SQLITE_NOTADB. SQLx exposes the
        // extended numeric code, whose low byte is the primary result code.
        Some(11 | 17 | 26) => EvidenceError::Corrupt(detail),
        _ => EvidenceError::Unavailable(detail),
    }
}

fn sqlite_primary_code(error: &sqlx::Error) -> Option<i32> {
    error
        .as_database_error()?
        .code()?
        .parse::<i32>()
        .ok()
        .map(|code| code & 0xff)
}
