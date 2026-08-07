use sqlx::Row;
use sqlx::SqlitePool;

use crate::EvidenceError;
use crate::store::classify_sqlx_error;

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
        let normalized_sql = sql.to_ascii_lowercase();
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
    Ok(())
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
