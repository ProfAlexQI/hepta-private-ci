#!/usr/bin/env python3
"""Normalize one semantically absorbed Q0 repair before replaying the frozen tail.

The frozen v7 manual-repair tail converts an internal `expect` in
`load_lease_chain` into a fail-closed corruption error.  Later hardening
already absorbed that safety property but used a more specific error message,
so the literal old/new matcher could not prove idempotence.  This adapter is
intentionally narrow: it recognizes only the three exact states of that one
predecessor guard and canonicalizes the absorbed safe state to the frozen
repair's exact output.  Any partial or structurally different state fails
closed.
"""

from __future__ import annotations

from pathlib import Path

TARGET = Path("codex-rs/hepta-memory/src/local_lease_outbox.rs")
FUNCTION_START = "pub(crate) async fn load_lease_chain("
FUNCTION_END = "/// Return every fence tuple granted anywhere in the lease history."

OLD_UNSAFE = (
    "        if index > 0 {\n"
    "            let prior = latest.as_ref().expect(\"lease prior row\");"
)
CANONICAL_SAFE = (
    "        if index > 0 {\n"
    "            let Some(prior) = latest.as_ref() else {\n"
    "                return Err(corrupt(\"lease prior row is missing\"));\n"
    "            };"
)
ABSORBED_SAFE = (
    "        if index > 0 {\n"
    "            let Some(prior) = latest.as_ref() else {\n"
    "                return Err(corrupt(\"lease journal is missing its prior row\"));\n"
    "            };"
)


def main() -> None:
    text = TARGET.read_text(encoding="utf-8")
    if text.count(FUNCTION_START) != 1 or text.count(FUNCTION_END) != 1:
        raise SystemExit("load_lease_chain boundary is missing or ambiguous")

    start = text.index(FUNCTION_START)
    end = text.index(FUNCTION_END, start)
    if end <= start:
        raise SystemExit("load_lease_chain boundary order is invalid")
    body = text[start:end]

    counts = {
        "old_unsafe": body.count(OLD_UNSAFE),
        "canonical_safe": body.count(CANONICAL_SAFE),
        "absorbed_safe": body.count(ABSORBED_SAFE),
    }
    present = [name for name, count in counts.items() if count]
    if any(count not in {0, 1} for count in counts.values()) or len(present) != 1:
        raise SystemExit(f"lease predecessor guard is partial or ambiguous: {counts}")

    state = present[0]
    if state == "absorbed_safe":
        body = body.replace(ABSORBED_SAFE, CANONICAL_SAFE, 1)
        text = text[:start] + body + text[end:]
        TARGET.write_text(text, encoding="utf-8")
    elif state not in {"old_unsafe", "canonical_safe"}:
        raise SystemExit(f"unsupported lease predecessor guard state: {state}")

    verified = TARGET.read_text(encoding="utf-8")
    start = verified.index(FUNCTION_START)
    end = verified.index(FUNCTION_END, start)
    verified_body = verified[start:end]
    final_counts = {
        "old_unsafe": verified_body.count(OLD_UNSAFE),
        "canonical_safe": verified_body.count(CANONICAL_SAFE),
        "absorbed_safe": verified_body.count(ABSORBED_SAFE),
    }
    if final_counts not in (
        {"old_unsafe": 1, "canonical_safe": 0, "absorbed_safe": 0},
        {"old_unsafe": 0, "canonical_safe": 1, "absorbed_safe": 0},
    ):
        raise SystemExit(f"lease predecessor guard did not normalize exactly: {final_counts}")

    print(
        "PASS_Q0_LEASE_PREDECESSOR_GUARD_NORMALIZATION "
        f"input={state} output={'old_unsafe' if final_counts['old_unsafe'] else 'canonical_safe'}"
    )


if __name__ == "__main__":
    main()
