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

// Migration 0009 is qualification-only, but its append-only boundary is still
// exact: every explicitly created table, index, and trigger is fingerprinted.
// This prevents a plausible-looking replacement DDL from opening as the v2
// live-product foundation before a strict path-based importer exists.
const EXACT_LIVE_PRODUCT_SHADOW_V2_SCHEMA_FINGERPRINTS: &[ExactSchemaFingerprint] = &[
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_artifact_imports",
        normalized_sql_sha256: "00863bb6ed395818c010a6066bd4785b5de23cf7693c038379f754862e373ba6",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_imports_before_terminal",
        normalized_sql_sha256: "1b8c775811ad549461ec6f2906087262467fa1ed080084457a6a1fb78bfcf901",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_imports_chronology_guard",
        normalized_sql_sha256: "fd35e307cf5da27ace27d8aa27b99bbfe81830a422beae7cfbf299612a9bb0f5",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_imports_identity_collision_guard",
        normalized_sql_sha256: "88252e6db9bd92bcbf90b57d6ec206c16e1c2460df9d299adeb209c8991b7789",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_imports_intent",
        normalized_sql_sha256: "d02a3ec471b97681c7bb5f0183143bb04cf1a5ef30218ef840e477ce765ca43a",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_imports_no_delete",
        normalized_sql_sha256: "46a4faeab70e8366f6ffecd16356e2365365b8b28ab0287de7a82ab9655247f9",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_imports_no_update",
        normalized_sql_sha256: "615b6f3c6f6f6c6fb5f6785e06e9c5c093bfa15df63e753c482f2235773271a9",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_imports_run_segment",
        normalized_sql_sha256: "d0bc815c5e8500139eb498a3444f2e4b5a8ec702c1f029ea49cfc4ab736c9a4c",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_intents_chain_guard",
        normalized_sql_sha256: "71f04addad9180dad41c93e2f6155de4e59cd6bc46c9c22b242b3c97cca0b97f",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_intents_identity_collision_guard",
        normalized_sql_sha256: "1912d130b55be72b8a0a7f536fb6b0b2dd5cff93c0f0e7d8ac3da6cdd2d2c19c",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_intents_no_delete",
        normalized_sql_sha256: "132297ba8337be4ceb34326ce2f8d463f47ca7ab5f260322f81a0f2f0901e1e5",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_intents_no_update",
        normalized_sql_sha256: "264a83798b5b0eb22976e0bcc1ba4120ced1159b2bd1188ee309f4209622380c",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_intents_run_segment_intent",
        normalized_sql_sha256: "9a268515388517434d2a0ab4e6ec136552c7107c77ea97e5f61d0ce1ef988053",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_intents_segment_ordinal",
        normalized_sql_sha256: "170fe543fcf1a673790dc77fc438f5591015305c1490fd588723149e93df0391",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_intents_segment_sample",
        normalized_sql_sha256: "8e329cb0744bda237c13239b5ac9bbe3c5ffb63aa28dc69b81c3b426061df5bc",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_pre_send_intents",
        normalized_sql_sha256: "24991dede4a2649bd45b524943b7f0459d38c0ea668c10b3117568934fb1045e",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_runs",
        normalized_sql_sha256: "43adb1d6c6766d633b70b593ad7c045593f9a834c1368c549aea187a0e106570",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_runs_identity_collision_guard",
        normalized_sql_sha256: "823e78ebcd3e36c852fa15488ca6be8bd52febf486ab916a3a1a4fc2170bfad3",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_runs_no_delete",
        normalized_sql_sha256: "3f0f4d56f8cec614b70663785fb3feff95b877c7344b9e3cb3358ad1d000d63a",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_runs_no_update",
        normalized_sql_sha256: "2dc50199ba82273d769aa8aba685513ae80f7646d63ea722e4250bed571fcbd4",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_runs_nonce",
        normalized_sql_sha256: "56e5bca7c846a455b3b18981859f51da93bd6cc1417cd0f02a21ca4dce47440e",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_segments",
        normalized_sql_sha256: "125ef4fc59ba45e3205c95d43201b0b2c1c8dd1f5b53a96a0f204ff17f4165b2",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_segments_before_terminal",
        normalized_sql_sha256: "8e7ab396233b96d34434db4c96e58318e38136a7fd456abcf06d71b010bb09c5",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_segments_chronology_guard",
        normalized_sql_sha256: "c13a9bcfb76425b46f4618308bb010b9d561b75c16380e73da5536d08a20230b",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_segments_database_nonce",
        normalized_sql_sha256: "e7478de7b62634affe8bdfe2cc260c5c40ec08b7eed1f3e8659d6206a8af53e5",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_segments_identity_collision_guard",
        normalized_sql_sha256: "43f2c667355548dc23adb278dd176035381f927cc7e367ecb1f0c9ca5138804c",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_segments_no_delete",
        normalized_sql_sha256: "8c8c5774eb0f35d0c133d9f4a1fd5c91dba561c4b3b937f53394340b30e39acc",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_segments_no_update",
        normalized_sql_sha256: "a5de9b7c8aa21d35562df530530efef2cf3e04fb2ccdc79398aada8af0d925ce",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_segments_run_ordinal",
        normalized_sql_sha256: "47e47db8c817555d5636a6ef3cf30a9e201803b378ae1af661ba57bbdbde7fd3",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_segments_run_segment",
        normalized_sql_sha256: "436ec33cf771319dc71b34764663e65298adc701542f7294e60df705cbddf45d",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_segments_run_surface",
        normalized_sql_sha256: "5aa33b5bf979245cacded4456cf8d177fdb88c78497c3172214d3d3366e0f6ce",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_terminals",
        normalized_sql_sha256: "00a81a17864cc04965434b9d800782df726403f3d5c23d23a47ad1327fe5c721",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_terminals_identity_collision_guard",
        normalized_sql_sha256: "707d96e1f167eb7fd0d17f6a2a86b8b08e565140e3ce43b8ed49a6002d308c54",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_terminals_no_delete",
        normalized_sql_sha256: "33cf63ed1ad0b5e0790cfe13cbb7d010fd499e7465c6019cf2e828a3189f1992",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_terminals_no_update",
        normalized_sql_sha256: "7495fd424e7f7ad7e0600bba8d229897214d44455e1861249cf456fdea753d96",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_terminals_run",
        normalized_sql_sha256: "6e643b48f6f5fb516b10204456c6c63e35e7365084a98eb8d428d38a905393b7",
    },
    ExactSchemaFingerprint {
        name: "live_product_shadow_v2_terminals_state_guard",
        normalized_sql_sha256: "e809679dcab9d8b6c6217ac25d94bcb8eacb50a67357e39391dc43c4c88bb65c",
    },
];
const EXACT_LIVE_PRODUCT_SHADOW_V2_TRIGGER_SET: &[(&str, &str)] = &[
    (
        "live_product_shadow_v2_artifact_imports",
        "live_product_shadow_v2_imports_before_terminal",
    ),
    (
        "live_product_shadow_v2_artifact_imports",
        "live_product_shadow_v2_imports_chronology_guard",
    ),
    (
        "live_product_shadow_v2_artifact_imports",
        "live_product_shadow_v2_imports_identity_collision_guard",
    ),
    (
        "live_product_shadow_v2_artifact_imports",
        "live_product_shadow_v2_imports_no_delete",
    ),
    (
        "live_product_shadow_v2_artifact_imports",
        "live_product_shadow_v2_imports_no_update",
    ),
    (
        "live_product_shadow_v2_pre_send_intents",
        "live_product_shadow_v2_intents_chain_guard",
    ),
    (
        "live_product_shadow_v2_pre_send_intents",
        "live_product_shadow_v2_intents_identity_collision_guard",
    ),
    (
        "live_product_shadow_v2_pre_send_intents",
        "live_product_shadow_v2_intents_no_delete",
    ),
    (
        "live_product_shadow_v2_pre_send_intents",
        "live_product_shadow_v2_intents_no_update",
    ),
    (
        "live_product_shadow_v2_runs",
        "live_product_shadow_v2_runs_identity_collision_guard",
    ),
    (
        "live_product_shadow_v2_runs",
        "live_product_shadow_v2_runs_no_delete",
    ),
    (
        "live_product_shadow_v2_runs",
        "live_product_shadow_v2_runs_no_update",
    ),
    (
        "live_product_shadow_v2_segments",
        "live_product_shadow_v2_segments_before_terminal",
    ),
    (
        "live_product_shadow_v2_segments",
        "live_product_shadow_v2_segments_chronology_guard",
    ),
    (
        "live_product_shadow_v2_segments",
        "live_product_shadow_v2_segments_identity_collision_guard",
    ),
    (
        "live_product_shadow_v2_segments",
        "live_product_shadow_v2_segments_no_delete",
    ),
    (
        "live_product_shadow_v2_segments",
        "live_product_shadow_v2_segments_no_update",
    ),
    (
        "live_product_shadow_v2_terminals",
        "live_product_shadow_v2_terminals_identity_collision_guard",
    ),
    (
        "live_product_shadow_v2_terminals",
        "live_product_shadow_v2_terminals_no_delete",
    ),
    (
        "live_product_shadow_v2_terminals",
        "live_product_shadow_v2_terminals_no_update",
    ),
    (
        "live_product_shadow_v2_terminals",
        "live_product_shadow_v2_terminals_state_guard",
    ),
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
    verify_exact_live_product_shadow_v2_trigger_set(pool).await?;
    verify_exact_live_product_shadow_v2_object_set(pool).await?;
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
    for fingerprint in EXACT_LIVE_PRODUCT_SHADOW_V2_SCHEMA_FINGERPRINTS {
        let sql =
            sqlx::query_scalar::<_, Option<String>>("SELECT sql FROM sqlite_schema WHERE name = ?")
                .bind(fingerprint.name)
                .fetch_optional(pool)
                .await
                .map_err(classify_sqlx_error)?
                .flatten()
                .ok_or_else(|| {
                    EvidenceError::Corrupt(format!(
                        "fingerprinted live product-Shadow v2 schema object {} is missing",
                        fingerprint.name
                    ))
                })?;
        let actual = Sha256Digest::for_bytes(normalize_schema_sql_preserving_case(&sql).as_bytes());
        if actual.as_str() != fingerprint.normalized_sql_sha256 {
            return Err(EvidenceError::Corrupt(format!(
                "fingerprinted live product-Shadow v2 schema object {} has an invalid exact definition",
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

async fn verify_exact_live_product_shadow_v2_trigger_set(
    pool: &SqlitePool,
) -> Result<(), EvidenceError> {
    let mut rows = sqlx::query(
        "SELECT tbl_name, name
         FROM sqlite_schema
         WHERE type = 'trigger'
           AND tbl_name IN (
               'live_product_shadow_v2_runs',
               'live_product_shadow_v2_segments',
               'live_product_shadow_v2_pre_send_intents',
               'live_product_shadow_v2_artifact_imports',
               'live_product_shadow_v2_terminals'
           )
         ORDER BY tbl_name ASC, name ASC",
    )
    .fetch(pool);
    let mut index = 0usize;
    while let Some(row) = rows.try_next().await.map_err(classify_sqlx_error)? {
        let table_name: String = row.get("tbl_name");
        let trigger_name: String = row.get("name");
        let Some((expected_table, expected_trigger)) =
            EXACT_LIVE_PRODUCT_SHADOW_V2_TRIGGER_SET.get(index)
        else {
            return Err(EvidenceError::Corrupt(format!(
                "unexpected live product-Shadow v2 trigger {trigger_name} exists on {table_name}"
            )));
        };
        if table_name != *expected_table || trigger_name != *expected_trigger {
            return Err(EvidenceError::Corrupt(format!(
                "live product-Shadow v2 trigger set differs at {table_name}.{trigger_name}"
            )));
        }
        index += 1;
    }
    if index != EXACT_LIVE_PRODUCT_SHADOW_V2_TRIGGER_SET.len() {
        return Err(EvidenceError::Corrupt(format!(
            "live product-Shadow v2 trigger set is incomplete: expected {}, found {index}",
            EXACT_LIVE_PRODUCT_SHADOW_V2_TRIGGER_SET.len()
        )));
    }
    Ok(())
}

async fn verify_exact_live_product_shadow_v2_object_set(
    pool: &SqlitePool,
) -> Result<(), EvidenceError> {
    let mut rows = sqlx::query(
        "SELECT name
         FROM sqlite_schema
         WHERE name LIKE 'live_product_shadow_v2_%'
           AND name NOT LIKE 'sqlite_autoindex_%'
         ORDER BY name ASC",
    )
    .fetch(pool);
    let mut index = 0usize;
    while let Some(row) = rows.try_next().await.map_err(classify_sqlx_error)? {
        let name: String = row.get("name");
        let Some(expected) = EXACT_LIVE_PRODUCT_SHADOW_V2_SCHEMA_FINGERPRINTS.get(index) else {
            return Err(EvidenceError::Corrupt(format!(
                "unexpected live product-Shadow v2 schema object {name} exists"
            )));
        };
        if name != expected.name {
            return Err(EvidenceError::Corrupt(format!(
                "live product-Shadow v2 schema object set differs at {name}"
            )));
        }
        index += 1;
    }
    if index != EXACT_LIVE_PRODUCT_SHADOW_V2_SCHEMA_FINGERPRINTS.len() {
        return Err(EvidenceError::Corrupt(format!(
            "live product-Shadow v2 schema object set is incomplete: expected {}, found {index}",
            EXACT_LIVE_PRODUCT_SHADOW_V2_SCHEMA_FINGERPRINTS.len()
        )));
    }
    Ok(())
}

fn normalize_schema_sql(sql: &str) -> String {
    normalize_schema_sql_preserving_case(sql).to_ascii_lowercase()
}

fn normalize_schema_sql_preserving_case(sql: &str) -> String {
    sql.split_ascii_whitespace().collect::<Vec<_>>().join(" ")
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
