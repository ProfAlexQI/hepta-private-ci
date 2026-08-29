#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
STORE_PATH = ROOT / "codex-rs/hepta-authbus-qualification/src/store.rs"


def replace_exact(source: str, before: str, after: str, expected: int, label: str) -> tuple[str, int]:
    count = source.count(before)
    if count != expected:
        raise SystemExit(f"{label}: expected {expected} anchors, found {count}")
    return source.replace(before, after), count


store = STORE_PATH.read_text(encoding="utf-8")
repairs = 0

for before, after, expected, label in (
    (
        "existing.intent_sha256 == admission_sha256.as_str().to_owned()",
        "existing.intent_sha256 == admission_sha256.as_str()",
        1,
        "borrow admission digest for comparison",
    ),
    (
        "existing == marker_sha256.as_str().to_owned()",
        "existing == marker_sha256.as_str()",
        1,
        "borrow marker digest for comparison",
    ),
    (
        "existing_sha256 == observation_sha256.as_str().to_owned()",
        "existing_sha256 == observation_sha256.as_str()",
        1,
        "borrow observation digest for comparison",
    ),
    (
        "observation.digest()?.as_str().to_owned() != stored",
        "observation.digest()?.as_str() != stored",
        1,
        "borrow observation row digest for comparison",
    ),
    (
        "row.digest()?.as_str().to_owned() != row.row_sha256",
        "row.digest()?.as_str() != row.row_sha256",
        4,
        "borrow durable row digest for comparison",
    ),
    (
        "admission.intent_sha256()?.as_str().to_owned() != row.intent_sha256",
        "admission.intent_sha256()?.as_str() != row.intent_sha256",
        1,
        "borrow admission intent digest for comparison",
    ),
    (
        "row.digest()?.as_str().to_owned() != row.witness_sha256",
        "row.digest()?.as_str() != row.witness_sha256",
        1,
        "borrow witness digest for comparison",
    ),
):
    store, count = replace_exact(store, before, after, expected, label)
    repairs += count

old_claim_guard = """        if let Some((existing_operation, active)) = load_claim(&mut transaction, &claim_key).await?
        {
            if active && existing_operation != admission.intent.operation_id {
                return Err(QualificationError::ActiveClaim);
            }
        }
"""
new_claim_guard = """        if let Some((existing_operation, active)) =
            load_claim(&mut transaction, &claim_key).await?
            && active
            && existing_operation != admission.intent.operation_id
        {
            return Err(QualificationError::ActiveClaim);
        }
"""
store, count = replace_exact(
    store,
    old_claim_guard,
    new_claim_guard,
    1,
    "collapse active-claim guard",
)
repairs += count

remaining = []
for line_number, line in enumerate(store.splitlines(), start=1):
    if ".as_str().to_owned()" in line and ("==" in line or "!=" in line):
        remaining.append(f"{line_number}:{line.strip()}")
if remaining:
    raise SystemExit(
        "owned digest comparisons remained after strict Clippy completion:\n"
        + "\n".join(remaining)
    )

STORE_PATH.write_text(store, encoding="utf-8")
print(f"applied_authbus_p1_3_strict_clippy_repairs={repairs}")
print("closed_authbus_p1_3_strict_clippy_gap=1")
