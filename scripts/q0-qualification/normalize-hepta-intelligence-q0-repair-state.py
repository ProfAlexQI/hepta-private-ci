#!/usr/bin/env python3
"""Normalize semantically absorbed Q0 repairs before frozen replay.

The Q0 reconstruct and supplemental scripts are intentionally literal so a
partial repair fails closed. Later hardening may already contain the same safe
result with equivalent wording or rustfmt shape. This adapter has no runtime
or authority role. It recognizes only finite, exact states and temporarily
restores already-applied supplemental outputs to their declared inputs; the
frozen scripts then replay them and produce one canonical candidate. Unknown,
mixed, duplicated, or partial states remain hard failures.
"""

from __future__ import annotations

import ast
import re
from pathlib import Path

LEASE_TARGET = Path("codex-rs/hepta-memory/src/local_lease_outbox.rs")
LEASE_FUNCTION_START = "pub(crate) async fn load_lease_chain("
LEASE_FUNCTION_END = (
    "/// Return every fence tuple granted anywhere in the lease history."
)
SUPPLEMENTAL = Path(".github/scripts/hepta-intelligence-q0-supplemental-repair-v1.py")

LEASE_OLD_UNSAFE = (
    "        if index > 0 {\n"
    '            let prior = latest.as_ref().expect("lease prior row");'
)
LEASE_CANONICAL_SAFE = (
    "        if index > 0 {\n"
    "            let Some(prior) = latest.as_ref() else {\n"
    '                return Err(corrupt("lease prior row is missing"));\n'
    "            };"
)
LEASE_ABSORBED_SAFE = (
    "        if index > 0 {\n"
    "            let Some(prior) = latest.as_ref() else {\n"
    '                return Err(corrupt("lease journal is missing its prior row"));\n'
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
        raise SystemExit(
            f"lease predecessor guard did not normalize exactly: {final_counts}"
        )
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
        raise SystemExit(
            f"{path_text} authority block did not normalize: {final_counts}"
        )
    return state


def literal(node: ast.AST, label: str):
    try:
        return ast.literal_eval(node)
    except (ValueError, TypeError) as error:
        raise SystemExit(
            f"supplemental {label} is not a literal: {ast.dump(node)}"
        ) from error


def top_level_repair_calls() -> list[ast.Call]:
    source = SUPPLEMENTAL.read_text(encoding="utf-8")
    tree = ast.parse(source, filename=str(SUPPLEMENTAL))
    calls: list[ast.Call] = []
    for statement in tree.body:
        if not isinstance(statement, ast.Expr) or not isinstance(
            statement.value, ast.Call
        ):
            continue
        call = statement.value
        if not isinstance(call.func, ast.Name):
            continue
        if call.func.id in {"replace_exact", "insert_expect_before_function"}:
            calls.append(call)
    if not calls:
        raise SystemExit("supplemental repair contains no top-level exact operations")
    return calls


def assertion_list_form(old: str) -> str | None:
    expressions: list[str] = []
    indent: str | None = None
    for line in old.splitlines():
        match = re.fullmatch(r"(?P<indent>[ \t]*)assert!\((?P<expression>.+)\);", line)
        if match is None:
            return None
        if indent is None:
            indent = match.group("indent")
        elif indent != match.group("indent"):
            return None
        expressions.append(match.group("expression"))
    if not expressions or indent is None:
        return None
    return "\n".join(
        f"{indent}const {{ assert!({expression}); }}" for expression in expressions
    )


def rollback_exact_output(path_text: str, old: str, new: str, expected: int) -> str:
    if expected < 1 or old == new or not old:
        raise SystemExit(f"invalid supplemental replacement contract for {path_text}")
    path = Path(path_text)
    text = path.read_text(encoding="utf-8")

    absorbed_assertions = assertion_list_form(old)
    if absorbed_assertions is not None:
        absorbed_count = text.count(absorbed_assertions)
        if absorbed_count not in {0, expected}:
            raise SystemExit(
                f"{path_text}: mixed absorbed assertion state; "
                f"expected 0 or {expected}, found {absorbed_count}"
            )
        if absorbed_count == expected:
            path.write_text(
                text.replace(absorbed_assertions, old, expected), encoding="utf-8"
            )
            return "absorbed_assertions_to_old"

    text = path.read_text(encoding="utf-8")
    old_count = text.count(old)
    new_count = text.count(new) if new else 0
    if old in new and 0 < new_count < expected:
        raise SystemExit(
            f"{path_text}: mixed old/new supplemental state; old={old_count} new={new_count}"
        )

    if new and new_count == expected:
        without_new = text.replace(new, "", expected)
        if without_new.count(old) != 0:
            raise SystemExit(
                f"{path_text}: new state coexists with an old supplemental state"
            )
        path.write_text(text.replace(new, old, expected), encoding="utf-8")
        return "canonical_new_to_old"

    if old_count == expected and new_count == 0:
        return "old_ready"

    raise SystemExit(
        f"{path_text}: supplemental replacement is partial or unknown; "
        f"old={old_count} new={new_count} expected={expected}"
    )


def rollback_inserted_expect(
    path_text: str,
    function_name: str,
    lint: str,
    reason: str,
) -> str:
    path = Path(path_text)
    text = path.read_text(encoding="utf-8")
    pattern = re.compile(
        rf"(?m)^(?P<indent>[ \t]*)(?P<signature>"
        rf"(?:(?:pub(?:\([^\)]*\))?)[ \t]+)?"
        rf"(?:async[ \t]+)?fn[ \t]+{re.escape(function_name)}"
        rf"(?:<[^\n]*?>)?[ \t]*\()"
    )
    matches = list(pattern.finditer(text))
    if len(matches) != 1:
        raise SystemExit(
            f"{path_text}: expected one function {function_name}, found {len(matches)}"
        )
    match = matches[0]
    indent = match.group("indent")
    exact = (
        f"{indent}#[expect(\n"
        f"{indent}    clippy::{lint},\n"
        f'{indent}    reason = "{reason}"\n'
        f"{indent})]\n"
    )
    one_line = f'{indent}#[expect(clippy::{lint}, reason = "{reason}")]\n'
    allow_line = f'{indent}#[allow(clippy::{lint}, reason = "{reason}")]\n'
    prefix = text[: match.start()]
    forms = [form for form in (exact, one_line, allow_line) if prefix.endswith(form)]
    if len(forms) > 1:
        raise SystemExit(f"{path_text}: ambiguous pre-existing Clippy attribute")
    if forms:
        form = forms[0]
        path.write_text(prefix[: -len(form)] + text[match.start() :], encoding="utf-8")
        return "attribute_to_unannotated"

    nearby = prefix[-2048:]
    if f"clippy::{lint}" in nearby and reason in nearby:
        raise SystemExit(
            f"{path_text}: equivalent Clippy attribute exists in an unknown shape"
        )
    return "unannotated_ready"


def precondition_supplemental_replay() -> dict[str, str]:
    states: dict[str, str] = {}
    ordinal = 0
    for call in top_level_repair_calls():
        ordinal += 1
        name = call.func.id if isinstance(call.func, ast.Name) else "unknown"
        if name == "replace_exact":
            if len(call.args) < 3:
                raise SystemExit(
                    "supplemental replace_exact call has fewer than three args"
                )
            path_text = literal(call.args[0], "path")
            old = literal(call.args[1], "old text")
            new = literal(call.args[2], "new text")
            expected = 1
            for keyword in call.keywords:
                if keyword.arg == "expected":
                    expected = literal(keyword.value, "expected count")
            if not all(isinstance(value, str) for value in (path_text, old, new)):
                raise SystemExit("supplemental replacement literals have invalid types")
            if not isinstance(expected, int):
                raise SystemExit("supplemental expected count is not an integer")
            state = rollback_exact_output(path_text, old, new, expected)
        elif name == "insert_expect_before_function":
            if len(call.args) != 4:
                raise SystemExit("supplemental insert-expect call must have four args")
            values = [
                literal(argument, "insert-expect argument") for argument in call.args
            ]
            if not all(isinstance(value, str) for value in values):
                raise SystemExit(
                    "supplemental insert-expect literals have invalid types"
                )
            state = rollback_inserted_expect(*values)
            path_text = values[0]
        else:
            raise SystemExit(f"unsupported supplemental operation {name}")
        states[f"{ordinal:02d}:{path_text}:{name}"] = state
    return states


def main() -> None:
    lease_state = normalize_lease_predecessor_guard()
    assertion_states = {
        path: normalize_assertion_block(path, expressions)
        for path, expressions in ASSERTION_BLOCKS.items()
    }
    supplemental_states = precondition_supplemental_replay()
    print(
        "PASS_Q0_FULL_REPAIR_PRECONDITION "
        f"lease={lease_state} assertions={assertion_states} "
        f"supplemental={supplemental_states}"
    )


if __name__ == "__main__":
    main()
