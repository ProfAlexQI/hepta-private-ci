#!/usr/bin/env python3
# Seed revision 3: close Rust 1.95 strict-Clippy gaps without lint suppression.
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
STORE_PATH = ROOT / "codex-rs/hepta-authbus-qualification/src/store.rs"
P03_LIB_PATH = ROOT / "codex-rs/hepta-authbus-p0-3-qualification/src/lib.rs"
P03_TEST_PATH = ROOT / "codex-rs/hepta-authbus-p0-3-qualification/tests/p0_3.rs"


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

inline_assertions = """        assert!(AUTHBUS_B4_P0_3_QUALIFICATION_ONLY);
        assert!(!AUTHBUS_B4_P0_3_AUTHORITY);
        assert!(!AUTHBUS_B4_P0_3_EFFECT_AUTHORITY);
        assert!(!AUTHBUS_B4_P0_3_PRODUCTION_CALLER);
        assert!(!AUTHBUS_B4_P0_3_PRODUCTION_WRITER);
        assert!(!AUTHBUS_B4_P0_3_OPERATOR_ACCEPTANCE);
        assert!(!AUTHBUS_B4_P0_3_PROMOTION);
        assert!(!AUTHBUS_B4_P0_3_G5_ALLOWED);
        assert!(!AUTHBUS_B4_P0_3_EXECUTE_ALLOWED);
"""
external_assertions = inline_assertions.replace("        ", "    ")

p03_lib = P03_LIB_PATH.read_text(encoding="utf-8")
old_inline_test = f"""    #[test]
    fn default_posture_never_grants_authority() {{
{inline_assertions}    }}
"""
new_inline_test = f"""    #[test]
    fn default_posture_never_grants_authority() {{
        const {{
{inline_assertions}        }}
    }}
"""
p03_lib, inline_constant_assertion_repairs = replace_exact(
    p03_lib,
    old_inline_test,
    new_inline_test,
    1,
    "move inline static authority assertions into const block",
)

p03_tests = P03_TEST_PATH.read_text(encoding="utf-8")
old_authority_test = f"""#[test]
fn authority_posture_is_statically_closed() {{
{external_assertions}}}
"""
new_authority_test = f"""#[test]
fn authority_posture_is_statically_closed() {{
    const {{
{external_assertions}    }}
}}
"""
p03_tests, external_constant_assertion_repairs = replace_exact(
    p03_tests,
    old_authority_test,
    new_authority_test,
    1,
    "move external static authority assertions into const block",
)

STORE_PATH.write_text(store, encoding="utf-8")
P03_LIB_PATH.write_text(p03_lib, encoding="utf-8")
P03_TEST_PATH.write_text(p03_tests, encoding="utf-8")
print(f"applied_authbus_p1_3_strict_clippy_repairs={repairs}")
print(
    "applied_authbus_p0_3_inline_constant_assertion_repairs="
    f"{inline_constant_assertion_repairs}"
)
print(
    "applied_authbus_p0_3_external_constant_assertion_repairs="
    f"{external_constant_assertion_repairs}"
)
print("applied_authbus_p0_3_constant_assertion_repairs=2")
print("closed_authbus_p0_3_constant_assertions_clippy_gap=1")
print("closed_authbus_p1_3_strict_clippy_gap=1")
