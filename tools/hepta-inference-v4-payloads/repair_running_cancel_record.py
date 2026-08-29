#!/usr/bin/env python3
"""Repair the S2 request-record field after all V4 payloads are materialized.

S2 adds running-cancel transition methods. Historical source variants name the
private record `Record` or `RequestRecord`, and an older payload emitted member
accesses named `running_cancel` without adding the matching field. This repair is
semantic and fail-closed: it locates the one record containing an
`InferenceRequest` and lifecycle state, canonicalizes the field name, adds the
field and every matching initializer, then proves no unfenced spelling remains.
"""

from __future__ import annotations

from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[2]
CONTROLLER = ROOT / "codex-rs/hepta-infer-core/src/controller.rs"
CANONICAL_FIELD = "running_cancel_phase"


def fail(message: str) -> "NoReturn":
    raise SystemExit(message)


def matching_brace(source: str, opening: int) -> int:
    depth = 0
    in_string = False
    in_char = False
    escaped = False
    line_comment = False
    block_comment_depth = 0
    index = opening
    while index < len(source):
        char = source[index]
        next_char = source[index + 1] if index + 1 < len(source) else ""

        if line_comment:
            if char == "\n":
                line_comment = False
            index += 1
            continue
        if block_comment_depth:
            if char == "/" and next_char == "*":
                block_comment_depth += 1
                index += 2
                continue
            if char == "*" and next_char == "/":
                block_comment_depth -= 1
                index += 2
                continue
            index += 1
            continue
        if in_string:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                in_string = False
            index += 1
            continue
        if in_char:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == "'":
                in_char = False
            index += 1
            continue

        if char == "/" and next_char == "/":
            line_comment = True
            index += 2
            continue
        if char == "/" and next_char == "*":
            block_comment_depth = 1
            index += 2
            continue
        if char == '"':
            in_string = True
            index += 1
            continue
        if char == "'":
            # Rust lifetimes are not character literals. Treat a quote followed by
            # an identifier as a lifetime unless a closing quote is immediately
            # present after one scalar.
            closing = source.find("'", index + 1, min(index + 8, len(source)))
            if closing >= 0:
                in_char = True
            index += 1
            continue
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return index
            if depth < 0:
                fail("brace parser underflow")
        index += 1
    fail("unterminated Rust brace block")


def insert_after_state(block: str, line: str) -> str:
    state = re.search(r"(?m)^(?P<indent>[ \t]+)state\s*:[^\n]+\n", block)
    if state is None:
        fail("record block has no state field or initializer")
    indent = state.group("indent")
    return block[: state.end()] + f"{indent}{line}\n" + block[state.end() :]


def main() -> None:
    source = CONTROLLER.read_text(encoding="utf-8")

    # Canonicalize only member/field spellings. Method names such as
    # `request_running_cancel` remain unchanged.
    source = re.sub(r"\.running_cancel\b", f".{CANONICAL_FIELD}", source)
    source = re.sub(
        r"(?m)^(?P<indent>[ \t]+)running_cancel\s*:",
        rf"\g<indent>{CANONICAL_FIELD}:",
        source,
    )

    candidates: list[tuple[str, int, int]] = []
    for match in re.finditer(r"(?m)^struct\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{", source):
        opening = source.index("{", match.start())
        closing = matching_brace(source, opening)
        block = source[match.start() : closing + 1]
        if "request: InferenceRequest" in block and "state: LifecycleState" in block:
            candidates.append((match.group(1), match.start(), closing + 1))
    if len(candidates) != 1:
        fail(f"expected one inference request record, found {len(candidates)}")

    record_name, record_start, record_end = candidates[0]
    record_block = source[record_start:record_end]
    field_pattern = rf"(?m)^[ \t]+{CANONICAL_FIELD}\s*:"
    field_count = len(re.findall(field_pattern, record_block))
    if field_count == 0:
        record_block = insert_after_state(
            record_block,
            f"{CANONICAL_FIELD}: Option<RunningCancelPhase>,",
        )
        source = source[:record_start] + record_block + source[record_end:]
    elif field_count != 1:
        fail(f"unexpected {CANONICAL_FIELD} field count: {field_count}")

    # Re-scan after the struct insertion because all following offsets changed.
    initializers: list[tuple[int, int]] = []
    pattern = re.compile(rf"\b{re.escape(record_name)}\s*\{{")
    for match in pattern.finditer(source):
        prefix = source[max(0, match.start() - 16) : match.start()]
        if re.search(r"struct\s+$", prefix):
            continue
        opening = source.index("{", match.start())
        closing = matching_brace(source, opening)
        block = source[match.start() : closing + 1]
        if (
            re.search(r"(?m)^[ \t]+state\s*:", block)
            and "terminal_receipt" in block
            and (re.search(r"(?m)^[ \t]+request\s*,", block) or "request:" in block)
        ):
            initializers.append((match.start(), closing + 1))
    if not initializers:
        fail(f"found no {record_name} request-record initializer")

    for start, end in reversed(initializers):
        block = source[start:end]
        count = len(re.findall(field_pattern, block))
        if count == 0:
            block = insert_after_state(block, f"{CANONICAL_FIELD}: None,")
            source = source[:start] + block + source[end:]
        elif count != 1:
            fail(f"initializer has unexpected {CANONICAL_FIELD} count: {count}")

    if re.search(r"\.running_cancel\b", source):
        fail("noncanonical running_cancel member access remains")

    # Prove the final record and all discovered initializers are complete.
    record_match = re.search(
        rf"(?ms)^struct\s+{re.escape(record_name)}\s*\{{.*?^\}}",
        source,
    )
    if record_match is None or len(re.findall(field_pattern, record_match.group(0))) != 1:
        fail("canonical running-cancel field was not materialized exactly once")
    if source.count(f".{CANONICAL_FIELD}") < 3:
        fail("running-cancel lifecycle methods are not wired to the record field")
    if source.count(f"{CANONICAL_FIELD}: None,") < 1:
        fail("running-cancel field is not initialized")

    CONTROLLER.write_text(source, encoding="utf-8")
    print(
        "PASS_HEPTA_INFERENCE_V4_RUNNING_CANCEL_RECORD_REPAIR "
        f"record={record_name} initializers={len(initializers)}"
    )


if __name__ == "__main__":
    main()
