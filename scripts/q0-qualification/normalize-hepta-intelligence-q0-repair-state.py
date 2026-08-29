#!/usr/bin/env python3
"""Normalize semantically absorbed Q0 repairs before replaying the frozen tail.

The frozen v7 manual-repair tail is intentionally literal. Later hardening
absorbed several of the same safety properties with equivalent formatting or
wording, so an old/new string matcher cannot prove idempotence. This adapter
is deliberately narrow: every supported repair has an explicit path and a
finite set of exact states. Equivalent safe states are canonicalized to the
frozen tail's exact output; partial, duplicated, or unknown states fail closed.
"""

from __future__ import annotations

from pathlib import Path

LEASE_TARGET = Path("codex-rs/hepta-memory/src/local_lease_outbox.rs")
LEASE_FUNCTION_START = "pub(crate) async fn load_lease_chain("
LEASE_FUNCTION_END = "/// Return every fence tuple granted anywhere in the lease history."

LEASE_OLD_UNSAFE = (
    "        if index > 0 {\n"
    "            let prior = latest.as_ref().expect(\"lease prior row\");"
)
LEASE_CANONICAL_SAFE = (
    "        if index > 0 {\n"
    "            let Some(prior) = latest.as_ref() else {\n"
    "                return Err(corrupt(\"lease prior row is missing\"));\n"
    "            };"
)
LEASE_ABSORBED_SAFE = (
    "        if index > 0 {\n"
    "            let Some(prior) = latest.as_ref() else {\n"
    "                return Err(corrupt(\"lease journal is missing its prior row\"));\n"
    "            };"
)

ASSERTION_BLOCKS: dict[str, tuple[str, ...]] = {
    "codex-rs/hepta-memory/src/local_atomic_witness_tests.rs": (
        "!LOCAL_ATOMIC_WITNESS_EXTERNAL_EFFECTS",
        "!LOCAL_ATOMIC_WITNESS_KG_WRITE_AUTHORITY",
        "!LOCAL_ATOMIC_WITNESS_LIFECYCLE_REGISTERED",
        "LOCAL_ATOMIC_WITNESS_LEASE_EPOCH_BOUND",
        "LOCAL_ATOMIC_WITNESS_LEASE_EXPIRY_BOUND",
    ),
    "codex-rs/hepta-memory/src/compact_persistence_tests.rs": (
        "!COMPACT_PERSISTENCE_EXTERNAL_EFFECTS",
        "!COMPACT_PERSISTENCE_KG_WRITE_AUTHORITY",
    ),
    "codex-rs/hepta-memory/src/local_compact_executor_tests.rs": (
        "!LOCAL_COMPACT_EXECUTOR_EXTERNAL_EFFECTS",
        "!LOCAL_COMPACT_EXECUTOR_KG_WRITE_AUTHORITY",
    ),
    "codex-rs/hepta-memory/src/local_lease_outbox_tests.rs": (
        "!LOCAL_LEASE_OUTBOX_EXTERNAL_EFFECTS",
        "!LOCAL_LEASE_OUTBOX_KG_WRITE_AUTHORITY",
        "!LOCAL_LEASE_OUTBOX_PRODUCTION_CALLER",
    ),
    "codex-rs/hepta-memory/src/logical_turn_registry_tests.rs": (
        "!crate::LOGICAL_TURN_REGISTRY_EXTERNAL_EFFECTS",
        "!crate::LOGICAL_TURN_REGISTRY_KG_WRITE_AUTHORITY",
        "!crate::LOGICAL_TURN_REGISTRY_PRODUCTION_CALLER",
    ),
    "codex-rs/hepta-memory/src/h7_feedback_tests.rs": (
        "!H7_FEEDBACK_EXTERNAL_EFFECTS",
        "!H7_FEEDBACK_KG_WRITE_AUTHORITY",
        "!H7_FEEDBACK_PRODUCTION_CALLER",
        "H7_FEEDBACK_REPLAY_ONLY",
    ),
}


def exact_state(counts: dict[str, int], label: str) -> str:
    present = [name for name, count in counts.items() if count]
    if any(count not in {0, 1} for count in counts.values()) or len(present) != 1:
        raise SystemExit(f"{label} is partial or ambiguous: {counts}")
    return present[0]


def normalize_lease_predecessor_guard() -> str:
    text = LEASE_TARGET.read_text(encoding="utf-8")
    if text.count(LEASE_FUNCTION_START) != 1 or text.count(LEASE_FUNCTION_END) != 1:
        raise SystemExit("load_lease_chain boundary is missing or ambiguous")

    start = text.index(LEASE_FUNCTION_START)
    end = text.index(LEASE_FUNCTION_END, start)
    if end <= start:
        raise SystemExit("load_lease_chain boundary order is invalid")
    body = text[start:end]
    counts = {
        "old_unsafe": body.count(LEASE_OLD_UNSAFE),
        "canonical_safe": body.count(LEASE_CANONICAL_SAFE),
        "absorbed_safe": body.count(LEASE_ABSORBED_SAFE),
    }
    state = exact_state(counts, "lease predecessor guard")
    if state == "absorbed_safe":
        body = body.replace(LEASE_ABSORBED_SAFE, LEASE_CANONICAL_SAFE, 1)
        LEASE_TARGET.write_text(text[:start] + body + text[end:], encoding="utf-8")

    verified = LEASE_TARGET.read_text(encoding="utf-8")
    start = verified.index(LEASE_FUNCTION_START)
    end = verified.index(LEASE_FUNCTION_END, start)
    verified_body = verified[start:end]
    final_counts = {
        "old_unsafe": verified_body.count(LEASE_OLD_UNSAFE),
        "canonical_safe": verified_body.count(LEASE_CANONICAL_SAFE),
        "absorbed_safe": verified_body.count(LEASE_ABSORBED_SAFE),
    }
    if final_counts not in (
        {"old_unsafe": 1, "canonical_safe": 0, "absorbed_safe": 0},
        {"old_unsafe": 0, "canonical_safe": 1, "absorbed_safe": 0},
    ):
        raise SystemExit(f"lease predecessor guard did not normalize exactly: {final_counts}")
    return state


def assertion_forms(expressions: tuple[str, ...]) -> tuple[str, str, str]:
    old = "\n".join(f"    assert!({expression});" for expression in expressions)
    canonical = (
        "    const {\n"
        + "\n".join(f"        assert!({expression});" for expression in expressions)
        + "\n    }"
    )
    absorbed = "\n".join(
        f"    const {{ assert!({expression}); }}" for expression in expressions
    )
    return old, canonical, absorbed


def normalize_assertion_block(path_text: str, expressions: tuple[str, ...]) -> str:
    path = Path(path_text)
    text = path.read_text(encoding="utf-8")
    old, canonical, absorbed = assertion_forms(expressions)
    counts = {
        "old_runtime_assertions": text.count(old),
        "canonical_const_block": text.count(canonical),
        "absorbed_individual_const_blocks": text.count(absorbed),
    }
    state = exact_state(counts, f"{path_text} authority assertion block")
    if state == "absorbed_individual_const_blocks":
        path.write_text(text.replace(absorbed, canonical, 1), encoding="utf-8")

    verified = path.read_text(encoding="utf-8")
    final_counts = {
        "old_runtime_assertions": verified.count(old),
        "canonical_const_block": verified.count(canonical),
        "absorbed_individual_const_blocks": verified.count(absorbed),
    }
    if final_counts not in (
        {
            "old_runtime_assertions": 1,
            "canonical_const_block": 0,
            "absorbed_individual_const_blocks": 0,
        },
        {
            "old_runtime_assertions": 0,
            "canonical_const_block": 1,
            "absorbed_individual_const_blocks": 0,
        },
    ):
        raise SystemExit(f"{path_text} authority block did not normalize: {final_counts}")
    return state


def main() -> None:
    lease_state = normalize_lease_predecessor_guard()
    assertion_states = {
        path: normalize_assertion_block(path, expressions)
        for path, expressions in ASSERTION_BLOCKS.items()
    }
    print(
        "PASS_Q0_ABSORBED_REPAIR_NORMALIZATION "
        f"lease={lease_state} assertions={assertion_states}"
    )


if __name__ == "__main__":
    main()
