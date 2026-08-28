#!/usr/bin/env python3
"""Apply the exact AuthBus P0.2 compile fixes found by hosted Rust 1.95."""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MODEL = ROOT / "codex-rs/hepta-authbus-qualification/src/model.rs"
STORE = ROOT / "codex-rs/hepta-authbus-qualification/src/store.rs"


def replace_exact(text: str, old: str, new: str, expected: int = 1) -> str:
    count = text.count(old)
    if count != expected:
        raise SystemExit(
            f"expected {expected} occurrence(s) of {old!r}, found {count}"
        )
    return text.replace(old, new)


def patch_model() -> None:
    text = MODEL.read_text(encoding="utf-8")
    text = replace_exact(
        text,
        "fencing_token_sha256: Sha256Digest::for_bytes(request.fencing_token.as_bytes()),",
        "fencing_token_sha256: request.fencing_token.clone(),",
        expected=2,
    )
    text = replace_exact(
        text,
        'effect_key: format!("provider-effect:v1:{operation_key_sha256}"),',
        'effect_key: format!("provider-effect:v1:{}", operation_key_sha256.as_str()),',
    )
    text = replace_exact(
        text,
        "matches!(Self::Completed | Self::Rejected | Self::Quarantined, self)",
        "matches!(self, Self::Completed | Self::Rejected | Self::Quarantined)",
    )
    text = replace_exact(
        text,
        "matches!(\n            Self::AttemptStarted | Self::Accepted | Self::Unknown | Self::Indeterminate,\n            self\n        )",
        "matches!(\n            self,\n            Self::AttemptStarted | Self::Accepted | Self::Unknown | Self::Indeterminate\n        )",
    )
    MODEL.write_text(text, encoding="utf-8")


def patch_store() -> None:
    text = STORE.read_text(encoding="utf-8")
    text = replace_exact(text, "use crate::digest_length_delimited;\n", "")

    replacements: list[tuple[str, str, int]] = [
        (
            "existing.intent_sha256 == admission_sha256.to_string()",
            "existing.intent_sha256 == admission_sha256.as_str()",
            1,
        ),
        (
            "let claim_key = claim_sha256.to_string();",
            "let claim_key = claim_sha256.as_str().to_owned();",
            1,
        ),
        (
            "intent_sha256: admission_sha256.to_string(),",
            "intent_sha256: admission_sha256.as_str().to_owned(),",
            1,
        ),
        (
            "existing == marker_sha256.to_string()",
            "existing == marker_sha256.as_str()",
            1,
        ),
        (
            "existing_sha256 == observation_sha256.to_string()",
            "existing_sha256 == observation_sha256.as_str()",
            1,
        ),
        (
            "current.ack_sha256.as_deref() == Some(ack_sha256.to_string().as_str())",
            "current.ack_sha256.as_deref() == Some(ack_sha256.as_str())",
            1,
        ),
        (
            ".bind(ack_sha256.to_string())",
            ".bind(ack_sha256.as_str())",
            1,
        ),
        (
            "sqlx::query_scalar(&statement)",
            "sqlx::query_scalar(sqlx::AssertSqlSafe(statement.as_str()))",
            1,
        ),
        (
            "sqlx::query(&statement)",
            "sqlx::query(sqlx::AssertSqlSafe(statement.as_str()))",
            1,
        ),
        (
            "observation.digest()?.to_string() != stored",
            "observation.digest()?.as_str() != stored",
            1,
        ),
        (
            "self.row_sha256 = self.digest()?.to_string();",
            "self.row_sha256 = self.digest()?.as_str().to_owned();",
            3,
        ),
        (
            "self.marker_sha256 = Some(marker_sha256.to_string());",
            "self.marker_sha256 = Some(marker_sha256.as_str().to_owned());",
            1,
        ),
        (
            "self.ack_sha256 = Some(ack_sha256.to_string());",
            "self.ack_sha256 = Some(ack_sha256.as_str().to_owned());",
            1,
        ),
        (
            ".bind(row.fence.fencing_token_sha256.to_string())",
            ".bind(row.fence.fencing_token_sha256.as_str())",
            2,
        ),
        (
            ".bind(fence.fencing_token_sha256.to_string())",
            ".bind(fence.fencing_token_sha256.as_str())",
            1,
        ),
        (
            "resource_sha256: admission.permit.resource_sha256.to_string(),",
            "resource_sha256: admission.permit.resource_sha256.as_str().to_owned(),",
            1,
        ),
        (
            "let digest = row.digest()?.to_string();",
            "let digest = row.digest()?.as_str().to_owned();",
            2,
        ),
        (
            "let digest = next.digest()?.to_string();",
            "let digest = next.digest()?.as_str().to_owned();",
            1,
        ),
        (
            ".bind(observation.binding_sha256.to_string())",
            ".bind(observation.binding_sha256.as_str())",
            1,
        ),
        (
            ".bind(observation_sha256.to_string())",
            ".bind(observation_sha256.as_str())",
            1,
        ),
        (
            'let outbox_id = format!("authbus-outbox:v1:{payload_sha256}");',
            'let outbox_id = format!("authbus-outbox:v1:{}", payload_sha256.as_str());',
            1,
        ),
        (
            ".bind(payload_sha256.to_string())",
            ".bind(payload_sha256.as_str())",
            2,
        ),
        (
            "payload_sha256: payload_sha256.to_string(),",
            "payload_sha256: payload_sha256.as_str().to_owned(),",
            2,
        ),
        (
            ".bind(witness_sha256.to_string())",
            ".bind(witness_sha256.as_str())",
            1,
        ),
        (
            "row.digest()?.to_string() != row.row_sha256",
            "row.digest()?.as_str() != row.row_sha256",
            4,
        ),
        (
            "admission.intent_sha256()?.to_string() != row.intent_sha256",
            "admission.intent_sha256()?.as_str() != row.intent_sha256",
            1,
        ),
        (
            "row.digest()?.to_string() != row.witness_sha256",
            "row.digest()?.as_str() != row.witness_sha256",
            1,
        ),
        (
            "async fn count_query(pool: &SqlitePool, statement: &str) -> QualificationResult<u64> {",
            "async fn count_query(\n    pool: &SqlitePool,\n    statement: &'static str,\n) -> QualificationResult<u64> {",
            1,
        ),
    ]

    for old, new, expected in replacements:
        text = replace_exact(text, old, new, expected)

    STORE.write_text(text, encoding="utf-8")


def main() -> None:
    patch_model()
    patch_store()


if __name__ == "__main__":
    main()
