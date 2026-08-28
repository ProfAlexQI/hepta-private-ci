#!/usr/bin/env python3
"""Executable SQLite contract test for Hepta Intelligence P0.4b."""

from __future__ import annotations

import hashlib
import json
import sqlite3
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MIGRATION = (
    ROOT
    / "codex-rs/hepta-memory/mutation-migrations/0012_intelligence_mutation_journal.sql"
)
EXPECTED_OBJECTS = {
    "cognitive_intelligence_mutation_migrations",
    "cognitive_intelligence_mutation_migrations_no_update",
    "cognitive_intelligence_mutation_migrations_no_delete",
    "cognitive_intelligence_mutation_operations",
    "cognitive_intelligence_mutation_operations_no_update",
    "cognitive_intelligence_mutation_operations_no_delete",
    "cognitive_intelligence_mutation_operations_owner_lookup",
    "cognitive_intelligence_mutation_operations_binding_lookup",
    "cognitive_intelligence_mutation_transitions",
    "cognitive_intelligence_mutation_transitions_no_update",
    "cognitive_intelligence_mutation_transitions_no_delete",
    "cognitive_intelligence_mutation_transitions_chain_guard",
    "cognitive_intelligence_mutation_transitions_digest_lookup",
    "cognitive_intelligence_mutation_transitions_phase_lookup",
}


def sha(label: str) -> str:
    return hashlib.sha256(label.encode("utf-8")).hexdigest()


def expect_rejected(connection: sqlite3.Connection, statement: str, params: tuple = ()) -> None:
    try:
        connection.execute(statement, params)
    except sqlite3.DatabaseError:
        return
    raise AssertionError(f"statement unexpectedly succeeded: {statement}")


def insert_operation(connection: sqlite3.Connection, operation_id: str) -> None:
    connection.execute(
        """
        INSERT INTO cognitive_intelligence_mutation_operations (
            operation_id, owner_agent_id, lease_id, lease_epoch,
            expected_revision, starting_projection_generation,
            causal_root_sha256, binding_sha256, created_at_unix_seconds
        ) VALUES (?, ?, ?, 7, 3, 11, ?, ?, 100)
        """,
        (
            operation_id,
            "00000000-0000-0000-0000-000000000241",
            f"lease:{operation_id}",
            sha(f"root:{operation_id}"),
            sha(f"binding:{operation_id}"),
        ),
    )


def insert_transition(
    connection: sqlite3.Connection,
    operation_id: str,
    sequence: int,
    from_phase: str,
    to_phase: str,
    action: str,
    parent: str | None,
    transition: str,
    *,
    intent_appended: int = 0,
    intent_settled: int = 0,
    memory_writes: int = 0,
    projection_publishes: int = 0,
    generation: int = 11,
) -> None:
    connection.execute(
        """
        INSERT INTO cognitive_intelligence_mutation_transitions (
            operation_id, sequence, from_phase, to_phase, action,
            action_payload_json, request_sha256, causal_parent_sha256,
            transition_sha256, durable_intent_appended,
            durable_intent_settled, memory_write_count,
            projection_publish_count, last_published_generation,
            recorded_at_unix_seconds
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 100)
        """,
        (
            operation_id,
            sequence,
            from_phase,
            to_phase,
            action,
            json.dumps({"action": action}, sort_keys=True),
            sha(f"request:{operation_id}:{sequence}:{action}"),
            parent,
            transition,
            intent_appended,
            intent_settled,
            memory_writes,
            projection_publishes,
            generation,
        ),
    )


def schema_digest(connection: sqlite3.Connection) -> str:
    rows = connection.execute(
        """
        SELECT name, type, sql
        FROM sqlite_schema
        WHERE name LIKE 'cognitive_intelligence_mutation_%'
        ORDER BY name
        """
    ).fetchall()
    payload = "\n".join(f"{name}\0{kind}\0{sql}" for name, kind, sql in rows)
    return hashlib.sha256(payload.encode("utf-8")).hexdigest()


def main() -> None:
    migration = MIGRATION.read_text(encoding="utf-8")
    connection = sqlite3.connect(":memory:", isolation_level=None)
    connection.execute("PRAGMA foreign_keys = ON")
    connection.executescript(migration)
    migration_checksum = hashlib.sha256(migration.encode("utf-8")).hexdigest()
    connection.execute(
        """
        INSERT INTO cognitive_intelligence_mutation_migrations (
            version, description, checksum_sha256, applied_at_unix_seconds
        ) VALUES (12, 'intelligence mutation transition journal', ?, 100)
        """,
        (migration_checksum,),
    )

    objects = {
        row[0]
        for row in connection.execute(
            """
            SELECT name FROM sqlite_schema
            WHERE name LIKE 'cognitive_intelligence_mutation_%'
            """
        )
    }
    assert objects == EXPECTED_OBJECTS, sorted(objects ^ EXPECTED_OBJECTS)

    insert_operation(connection, "operation:normal")
    first = sha("transition:normal:0")
    insert_transition(
        connection,
        "operation:normal",
        0,
        "planned",
        "source_witnessed",
        "witness_source",
        None,
        first,
    )
    second = sha("transition:normal:1")
    insert_transition(
        connection,
        "operation:normal",
        1,
        "source_witnessed",
        "grounding_validated",
        "validate_grounding",
        first,
        second,
    )

    expect_rejected(
        connection,
        """
        INSERT INTO cognitive_intelligence_mutation_transitions (
            operation_id, sequence, from_phase, to_phase, action,
            action_payload_json, request_sha256, causal_parent_sha256,
            transition_sha256, durable_intent_appended,
            durable_intent_settled, memory_write_count,
            projection_publish_count, last_published_generation,
            recorded_at_unix_seconds
        ) VALUES ('operation:normal', 3, 'grounding_validated',
                  'durable_intent_appended', 'append_durable_intent', '{}',
                  ?, ?, ?, 1, 0, 0, 0, 11, 100)
        """,
        (sha("gap-request"), second, sha("gap-transition")),
    )
    expect_rejected(
        connection,
        "UPDATE cognitive_intelligence_mutation_transitions SET action = action",
    )
    expect_rejected(
        connection,
        "DELETE FROM cognitive_intelligence_mutation_operations WHERE operation_id = ?",
        ("operation:normal",),
    )

    connection.execute("BEGIN IMMEDIATE")
    insert_operation(connection, "operation:rollback")
    insert_transition(
        connection,
        "operation:rollback",
        0,
        "planned",
        "source_witnessed",
        "witness_source",
        None,
        sha("transition:rollback:0"),
    )
    connection.execute("ROLLBACK")
    assert connection.execute(
        "SELECT COUNT(*) FROM cognitive_intelligence_mutation_operations WHERE operation_id = ?",
        ("operation:rollback",),
    ).fetchone()[0] == 0

    connection.execute("BEGIN IMMEDIATE")
    insert_operation(connection, "operation:ack-loss")
    ack_transition = sha("transition:ack-loss:0")
    insert_transition(
        connection,
        "operation:ack-loss",
        0,
        "planned",
        "source_witnessed",
        "witness_source",
        None,
        ack_transition,
    )
    connection.execute("COMMIT")
    adopted = connection.execute(
        """
        SELECT transition_sha256
        FROM cognitive_intelligence_mutation_transitions
        WHERE operation_id = 'operation:ack-loss' AND sequence = 0
        """
    ).fetchone()[0]
    assert adopted == ack_transition

    insert_operation(connection, "operation:terminal-guard")
    expect_rejected(
        connection,
        """
        INSERT INTO cognitive_intelligence_mutation_transitions (
            operation_id, sequence, from_phase, to_phase, action,
            action_payload_json, request_sha256, causal_parent_sha256,
            transition_sha256, durable_intent_appended,
            durable_intent_settled, memory_write_count,
            projection_publish_count, last_published_generation,
            recorded_at_unix_seconds
        ) VALUES ('operation:terminal-guard', 0, 'planned', 'terminal',
                  'terminalize', '{}', ?, NULL, ?, 0, 0, 0, 0, 11, 100)
        """,
        (sha("terminal-request"), sha("terminal-transition")),
    )

    payload = {
        "schema": "hepta.intelligence.p0.4b.sqlite-selftest.v1",
        "status": "PASS_P0_4B_MUTATION_JOURNAL_SQLITE",
        "schema_object_count": len(objects),
        "migration_checksum_sha256": migration_checksum,
        "schema_oracle_sha256": schema_digest(connection),
        "normal_transition_count": connection.execute(
            "SELECT COUNT(*) FROM cognitive_intelligence_mutation_transitions WHERE operation_id = ?",
            ("operation:normal",),
        ).fetchone()[0],
        "rollback_operation_count": 0,
        "ack_loss_adopted": adopted == ack_transition,
        "runtime_wired": False,
        "default_open_wired": False,
        "production_authority": False,
        "external_effects": False,
        "operator_acceptance": False,
        "promotion": False,
    }
    print(json.dumps(payload, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
