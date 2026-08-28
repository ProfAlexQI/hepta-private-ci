#!/usr/bin/env python3
"""Apply the bounded P0.3.2 compile/reopen repair on the exact development branch."""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def read(relative: str) -> str:
    return (ROOT / relative).read_text(encoding="utf-8")


def write(relative: str, text: str) -> None:
    (ROOT / relative).write_text(text, encoding="utf-8")


def replace_once(relative: str, old: str, new: str) -> None:
    text = read(relative)
    count = text.count(old)
    if count != 1:
        raise SystemExit(
            f"{relative}: expected exactly one replacement target, observed {count}: {old[:120]!r}"
        )
    write(relative, text.replace(old, new, 1))


def patch_visibility() -> None:
    prepare = "codex-rs/hepta-memory/src/fact_grounding/durable/grounding/prepare.rs"
    text = read(prepare)
    for name in (
        "validate_source_binding",
        "require_groundable_revision",
        "bind_exact_citation",
        "prepare",
        "validate_canonical_identity_binding",
    ):
        old = f"pub(super) fn {name}("
        new = f"pub(in super::super) fn {name}("
        count = text.count(old)
        if count != 1:
            raise SystemExit(
                f"{prepare}: expected one visibility target for {name}, observed {count}"
            )
        text = text.replace(old, new, 1)
    write(prepare, text)

    replace_once(
        "codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger/insert.rs",
        "pub(super) async fn insert_tx(",
        "pub(in super::super::super) async fn insert_tx(",
    )
    replace_once(
        "codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger/verify.rs",
        "pub(super) async fn verify_receipts(",
        "pub(in super::super::super) async fn verify_receipts(",
    )


def patch_frame_helper_and_imports() -> None:
    support = (
        "codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger/support.rs"
    )
    text = read(support)
    count = text.count("super::super::frame_part")
    if count < 10:
        raise SystemExit(
            f"{support}: expected durable receipt frame calls, observed only {count}"
        )
    write(
        support,
        text.replace("super::super::frame_part", "crate::framing::frame_part"),
    )

    replace_once(
        "codex-rs/hepta-memory/src/fact_grounding/durable/schema.rs",
        "use sqlx::Executor;\n",
        "",
    )


def patch_reopen_adapter() -> None:
    reopen = "codex-rs/hepta-memory/src/cognitive_store.rs"

    replace_once(
        reopen,
        "#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]\nstruct StoredProjectionEdge {",
        """#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct StoredProjectionNode {
    node_id: String,
    canonical_entity_id: String,
    entity_type: String,
    label: String,
    valid_from: i64,
    valid_to: Option<i64>,
    memory_id: String,
    memory_revision: i64,
    source_id: String,
    source_revision: i64,
}

impl From<&ProjectionNode> for StoredProjectionNode {
    fn from(node: &ProjectionNode) -> Self {
        Self {
            node_id: node.node_id.clone(),
            canonical_entity_id: node.canonical_entity_id.clone(),
            entity_type: node.entity_type.clone(),
            label: node.label.clone(),
            valid_from: node.valid_from,
            valid_to: node.valid_to,
            memory_id: node.memory_id.clone(),
            memory_revision: node.memory_revision,
            source_id: node.source_id.clone(),
            source_revision: node.source_revision,
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct StoredProjectionEdge {""",
    )

    replace_once(
        reopen,
        """            \"SELECT r.memory_id, r.revision, r.content_sha256,
                    r.verification, r.lifecycle, s.fact_set_sha256
             FROM memory_heads h""",
        """            \"SELECT r.memory_id, r.revision, r.content_sha256,
                    r.verification, r.lifecycle, s.fact_set_sha256,
                    s.entity_count, s.relation_count
             FROM memory_heads h""",
    )
    replace_once(
        reopen,
        """                    lifecycle: row.try_get(\"lifecycle\").map_err(unavailable)?,
                    fact_set_sha256: row.try_get(\"fact_set_sha256\").map_err(unavailable)?,
                })""",
        """                    lifecycle: row.try_get(\"lifecycle\").map_err(unavailable)?,
                    fact_set_sha256: row.try_get(\"fact_set_sha256\").map_err(unavailable)?,
                    entity_count: row.try_get(\"entity_count\").map_err(unavailable)?,
                    relation_count: row.try_get(\"relation_count\").map_err(unavailable)?,
                    grounding_receipt_sha256: None,
                })""",
    )
    replace_once(
        reopen,
        """            expected_nodes.push(ProjectionNode {
                node_id: occurrence_node_id(&memory_id, memory_revision, &entity_key),
                canonical_entity_id: row.try_get(\"canonical_entity_id\").map_err(unavailable)?,
                entity_type: row.try_get(\"entity_type\").map_err(unavailable)?,""",
        """            expected_nodes.push(ProjectionNode {
                node_id: occurrence_node_id(&memory_id, memory_revision, &entity_key),
                canonical_entity_id: row.try_get(\"canonical_entity_id\").map_err(unavailable)?,
                entity_key,
                entity_type: row.try_get(\"entity_type\").map_err(unavailable)?,""",
    )
    replace_once(
        reopen,
        """            expected_edges.push(ProjectionEdge {
                edge_id: occurrence_edge_id(&memory_id, memory_revision, &relation_key),
                canonical_relation_id: row.try_get(\"canonical_relation_id\").map_err(unavailable)?,
                from_node_id: occurrence_node_id(&memory_id, memory_revision, &from_entity_key),
                to_node_id: occurrence_node_id(&memory_id, memory_revision, &to_entity_key),""",
        """            expected_edges.push(ProjectionEdge {
                edge_id: occurrence_edge_id(&memory_id, memory_revision, &relation_key),
                canonical_relation_id: row.try_get(\"canonical_relation_id\").map_err(unavailable)?,
                relation_key,
                from_entity_key: from_entity_key.clone(),
                to_entity_key: to_entity_key.clone(),
                from_node_id: occurrence_node_id(&memory_id, memory_revision, &from_entity_key),
                to_node_id: occurrence_node_id(&memory_id, memory_revision, &to_entity_key),""",
    )

    replace_once(
        reopen,
        """        let mut stored_nodes = stored_node_rows
            .into_iter()
            .map(|row| {
                Ok(ProjectionNode {
                    node_id: row.try_get(\"node_id\").map_err(unavailable)?,
                    canonical_entity_id: row.try_get(\"canonical_entity_id\").map_err(unavailable)?,
                    entity_type: row.try_get(\"entity_type\").map_err(unavailable)?,
                    label: row.try_get(\"label\").map_err(unavailable)?,
                    valid_from: row
                        .try_get(\"valid_from_unix_seconds\")
                        .map_err(unavailable)?,
                    valid_to: row.try_get(\"valid_to_unix_seconds\").map_err(unavailable)?,
                    memory_id: row.try_get(\"memory_id\").map_err(unavailable)?,
                    memory_revision: row.try_get(\"memory_revision\").map_err(unavailable)?,
                    source_id: row.try_get(\"source_id\").map_err(unavailable)?,
                    source_revision: row.try_get(\"source_revision\").map_err(unavailable)?,
                })
            })
            .collect::<Result<Vec<_>, CognitiveStoreError>>()?;
        let mut expected_nodes_by_id = expected_nodes.clone();
        expected_nodes_by_id.sort_by(|left, right| left.node_id.cmp(&right.node_id));
        stored_nodes.sort_by(|left, right| left.node_id.cmp(&right.node_id));
        if stored_nodes != expected_nodes_by_id {""",
        """        let mut stored_nodes = stored_node_rows
            .into_iter()
            .map(|row| {
                Ok(StoredProjectionNode {
                    node_id: row.try_get(\"node_id\").map_err(unavailable)?,
                    canonical_entity_id: row.try_get(\"canonical_entity_id\").map_err(unavailable)?,
                    entity_type: row.try_get(\"entity_type\").map_err(unavailable)?,
                    label: row.try_get(\"label\").map_err(unavailable)?,
                    valid_from: row
                        .try_get(\"valid_from_unix_seconds\")
                        .map_err(unavailable)?,
                    valid_to: row.try_get(\"valid_to_unix_seconds\").map_err(unavailable)?,
                    memory_id: row.try_get(\"memory_id\").map_err(unavailable)?,
                    memory_revision: row.try_get(\"memory_revision\").map_err(unavailable)?,
                    source_id: row.try_get(\"source_id\").map_err(unavailable)?,
                    source_revision: row.try_get(\"source_revision\").map_err(unavailable)?,
                })
            })
            .collect::<Result<Vec<_>, CognitiveStoreError>>()?;
        let mut expected_stored_nodes = expected_nodes
            .iter()
            .map(StoredProjectionNode::from)
            .collect::<Vec<_>>();
        stored_nodes.sort();
        expected_stored_nodes.sort();
        if stored_nodes != expected_stored_nodes {""",
    )


def main() -> None:
    patch_visibility()
    patch_frame_helper_and_imports()
    patch_reopen_adapter()


if __name__ == "__main__":
    main()
