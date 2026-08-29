#!/usr/bin/env python3
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def write(path: str, content: str) -> None:
    (ROOT / path).write_text(content, encoding="utf-8")


def replace_exact(
    source: str,
    before: str,
    after: str,
    expected: int,
    label: str,
) -> tuple[str, int]:
    count = source.count(before)
    if count != expected:
        raise SystemExit(f"{label}: expected {expected} anchors, found {count}")
    return source.replace(before, after), count


model_path = "codex-rs/hepta-authbus-qualification/src/model.rs"
model = read(model_path)
model_repairs = 0
for before, after, expected, label in (
    (
        "fencing_token_sha256: Sha256Digest::for_bytes(request.fencing_token.as_bytes()),",
        "fencing_token_sha256: request.fencing_token.clone(),",
        2,
        "preserve the already-digested fence token",
    ),
    (
        'effect_key: format!("provider-effect:v1:{operation_key_sha256}"),',
        'effect_key: format!("provider-effect:v1:{}", operation_key_sha256.as_str()),',
        1,
        "render operation-key digest explicitly",
    ),
    (
        "matches!(Self::Completed | Self::Rejected | Self::Quarantined, self)",
        "matches!(self, Self::Completed | Self::Rejected | Self::Quarantined)",
        1,
        "terminal-state matches order",
    ),
    (
        "matches!(\n            Self::AttemptStarted | Self::Accepted | Self::Unknown | Self::Indeterminate,\n            self\n        )",
        "matches!(\n            self,\n            Self::AttemptStarted | Self::Accepted | Self::Unknown | Self::Indeterminate\n        )",
        1,
        "lookup-only-state matches order",
    ),
):
    model, count = replace_exact(model, before, after, expected, label)
    model_repairs += count
write(model_path, model)

store_path = "codex-rs/hepta-authbus-qualification/src/store.rs"
store = read(store_path)
store_repairs = 0
for unused_import in (
    "use crate::VerifiedNoEffectTerminal;\n",
    "use crate::digest_length_delimited;\n",
):
    store, count = replace_exact(
        store,
        unused_import,
        "",
        1,
        f"remove unused import {unused_import.strip()}",
    )
    store_repairs += count

# Sha256Digest deliberately has no Display implementation. Persist its stable
# lowercase text through as_str() rather than relying on ToString or formatting.
patterns = (
    r"(?P<expr>\b(?:[A-Za-z_]\w*\.)*[A-Za-z_]\w*_sha256)\.to_string\(\)",
    r"(?P<expr>\b(?:[A-Za-z_]\w*\.)*[A-Za-z_]\w*_sha256\(\)\?)\.to_string\(\)",
    r"(?P<expr>\b(?:[A-Za-z_]\w*\.)*[A-Za-z_]\w*\.digest\(\)\?)\.to_string\(\)",
)
for pattern in patterns:
    store, count = re.subn(pattern, r"\g<expr>.as_str().to_owned()", store)
    store_repairs += count

store, count = replace_exact(
    store,
    "Some(ack_sha256.as_str().to_owned().as_str())",
    "Some(ack_sha256.as_str())",
    1,
    "avoid a temporary allocation for ACK replay comparison",
)
store_repairs += count
store, count = replace_exact(
    store,
    'let outbox_id = format!("authbus-outbox:v1:{payload_sha256}");',
    'let outbox_id = format!("authbus-outbox:v1:{}", payload_sha256.as_str());',
    1,
    "render outbox payload digest explicitly",
)
store_repairs += count

# Preserve fields used after with_ack consumes the current row. This keeps the
# ownership boundary explicit and avoids cloning the complete payload record.
store, count = replace_exact(
    store,
    "        let next = current.with_ack(ack_sha256.clone(), acked_at_ms)?;\n",
    "        let current_sequence = current.sequence;\n"
    "        let current_operation_id = current.operation_id.clone();\n"
    "        let current_operation_revision = current.operation_revision;\n"
    "        let next = current.with_ack(ack_sha256.clone(), acked_at_ms)?;\n",
    1,
    "retain outbox identity before consuming the row",
)
store_repairs += count
for before, after, label in (
    (
        "last_sequence.max(current.sequence)",
        "last_sequence.max(current_sequence)",
        "use retained outbox sequence",
    ),
    (
        "            &current.operation_id,\n            \"OUTBOX_ACK_DURABLE\",\n            current.operation_revision,",
        "            &current_operation_id,\n            \"OUTBOX_ACK_DURABLE\",\n            current_operation_revision,",
        "use retained outbox operation binding",
    ),
):
    store, count = replace_exact(store, before, after, 1, label)
    store_repairs += count

# SQLx 0.9 intentionally rejects runtime-composed SQL. SQLite clamps a
# max_page_count request below the current page count to the current count, so
# the static assignment below genuinely caps growth without dynamic SQL.
old_page_cap = '''        let statement = format!("PRAGMA max_page_count = {page_count}");
        let applied: i64 = sqlx::query_scalar(&statement)
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx)?;
        to_u64(applied)
'''
new_page_cap = '''        let applied: i64 = sqlx::query_scalar("PRAGMA max_page_count = 1")
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx)?;
        if applied < page_count {
            return Err(QualificationError::Corrupt);
        }
        to_u64(applied)
'''
store, count = replace_exact(
    store,
    old_page_cap,
    new_page_cap,
    1,
    "static max-page qualification cap",
)
store_repairs += count

# Schema introspection and count helpers consume only closed static statements.
old_schema_loop = '''        let tables = [
            "operations",
            "token_family_claims",
            "quota_reservations",
            "dispatch_attempts",
            "status_observations",
            "outbox",
            "fsync_receipts",
        ];
        let mut columns = Vec::new();
        for table in tables {
            let statement = format!("PRAGMA table_info({table})");
            let rows = sqlx::query(&statement)
                .fetch_all(&self.pool)
                .await
                .map_err(map_sqlx)?;
            for row in rows {
                columns.push(row.try_get::<String, _>("name").map_err(map_sqlx)?);
            }
        }
'''
new_schema_loop = '''        let statements = [
            "PRAGMA table_info(operations)",
            "PRAGMA table_info(token_family_claims)",
            "PRAGMA table_info(quota_reservations)",
            "PRAGMA table_info(dispatch_attempts)",
            "PRAGMA table_info(status_observations)",
            "PRAGMA table_info(outbox)",
            "PRAGMA table_info(fsync_receipts)",
        ];
        let mut columns = Vec::new();
        for statement in statements {
            let rows = sqlx::query(statement)
                .fetch_all(&self.pool)
                .await
                .map_err(map_sqlx)?;
            for row in rows {
                columns.push(row.try_get::<String, _>("name").map_err(map_sqlx)?);
            }
        }
'''
store, count = replace_exact(
    store,
    old_schema_loop,
    new_schema_loop,
    1,
    "static schema-introspection statements",
)
store_repairs += count
store, count = replace_exact(
    store,
    "async fn count_query(pool: &SqlitePool, statement: &str) -> QualificationResult<u64> {",
    "async fn count_query(pool: &SqlitePool, statement: &'static str) -> QualificationResult<u64> {",
    1,
    "static count-query boundary",
)
store_repairs += count

remaining_digest_display = []
for line_number, line in enumerate(store.splitlines(), start=1):
    if ".to_string()" in line and ("sha256" in line or ".digest()?" in line):
        remaining_digest_display.append(f"{line_number}:{line.strip()}")
if remaining_digest_display:
    raise SystemExit(
        "unrepaired Sha256Digest text conversions:\n" + "\n".join(remaining_digest_display)
    )
if "format!(\"PRAGMA" in store or "sqlx::query(&statement)" in store:
    raise SystemExit("dynamic SQL remained after AuthBus P1.3 gap closure")
write(store_path, store)

print(f"applied_authbus_p1_3_model_compile_repairs={model_repairs}")
print(f"applied_authbus_p1_3_store_compile_repairs={store_repairs}")
print("closed_authbus_p1_3_materialized_compile_gap=1")
