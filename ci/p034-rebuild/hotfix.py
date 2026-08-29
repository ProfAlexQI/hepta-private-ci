#!/usr/bin/env python3
"""Apply exact plan and bounded predecessor-admission repairs to P0.3.4 payload."""

from __future__ import annotations

import ast
import hashlib
import json
import sys
from pathlib import Path

PLAN_BEFORE_SHA256 = "89516216e15fde9a573300aab53dace604114308bf71a6fce6d62c360b56fe66"
PLAN_AFTER_SHA256 = "8ec425fec8c97f36eccff50d6e24f2438987339fb40bfe2734511a0b1b20304e"
PREDECESSOR_SCRIPT = "scripts/verify-hepta-intelligence-grounding-ledger.py"

OLD_PLAN = """## Reconstruction decision

The historical payload was truncated: the archived gzip stream had no EOF and
contained only the first 6,024 bytes of a declared 79,891-byte Rust module.
That artifact is not source evidence. P0.3.4 is therefore independently
reconstructed from the recovered written contract, the complete migration, and
the qualified P0.2/P0.3.3 implementation. The new candidate must not reuse the
historical payload digest or claim byte identity with the lost source.
"""

NEW_PLAN = """## Reconstruction decision

historical payload was truncated: the archived gzip stream had no EOF and contained only the first 6,024 bytes of a declared 79,891-byte Rust module. This candidate is independently reconstructed from the recovered written contract, the complete migration, and the qualified P0.2/P0.3.3 implementation. It must not reuse the historical payload digest or claim byte identity with the lost source.
"""

COMPAT_BODY = '''_subprocess = __import__("subprocess")
_sys = __import__("sys")
_json = __import__("json")
predecessor = _subprocess.run(
    [_sys.executable, "scripts/verify-hepta-intelligence-grounding-ledger.py"],
    cwd={root_name},
    text=True,
    stdout=_subprocess.PIPE,
    stderr=_subprocess.STDOUT,
    check=False,
)
output = predecessor.stdout or ""
if output:
    print(output, end="" if output.endswith("\\n") else "\\n", flush=True)
if predecessor.returncode == 0:
    return

start = output.find("{")
if start < 0:
    raise RuntimeError(
        "P0.2 predecessor source gate failed without a machine-readable receipt"
    )
try:
    receipt, consumed = _json.JSONDecoder().raw_decode(output[start:])
except _json.JSONDecodeError as error:
    raise RuntimeError(
        "P0.2 predecessor source gate emitted an invalid receipt"
    ) from error
if output[start + consumed :].strip():
    raise RuntimeError(
        "P0.2 predecessor source gate emitted unbound trailing output"
    )

allowed = {{
    "check_hepta_intelligence_grounding_v2_present",
    "transitive_gate_format_clean",
}}
checks = receipt.get("checks")
failures = receipt.get("failures")
if not isinstance(checks, dict) or not isinstance(failures, list):
    raise RuntimeError("P0.2 predecessor source receipt structure drifted")
observed_false = {{
    str(name) for name, passed in checks.items() if passed is not True
}}
if (
    receipt.get("status")
    != "PASS_P0_2_SOURCE_CONTRACT_WITH_TRANSITIVE_FORMAT_GAP"
    or set(map(str, failures)) != allowed
    or observed_false != allowed
):
    raise RuntimeError(
        "P0.2 predecessor source gate has a non-compatible failure: "
        f"status={{receipt.get('status')!r}} failures={{failures!r}} "
        f"false_checks={{sorted(observed_false)!r}}"
    )

formatting = _subprocess.run(
    [
        "rustfmt",
        "--edition",
        "2024",
        "--config",
        "skip_children=true",
        "--check",
        *P02_RUST_FILES,
    ],
    cwd={root_name} / "codex-rs",
    text=True,
    stdout=_subprocess.PIPE,
    stderr=_subprocess.STDOUT,
    check=False,
)
if formatting.stdout:
    print(
        formatting.stdout,
        end="" if formatting.stdout.endswith("\\n") else "\\n",
        flush=True,
    )
if formatting.returncode != 0:
    raise RuntimeError(
        "P0.2 governed Rust formatting failed independent successor admission"
    )
print(
    "P0.2 predecessor compatibility admission PASS: only the retired "
    "transitive checker and workspace-wide formatter were absent; all "
    "semantic checks and governed-file rustfmt passed",
    flush=True,
)
'''


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def patch_plan(root: Path) -> None:
    plan = root / "P034_PLAN.md"
    actual_before = digest(plan)
    if actual_before != PLAN_BEFORE_SHA256:
        raise SystemExit(
            f"P0.3.4 plan input drifted: expected {PLAN_BEFORE_SHA256}, "
            f"got {actual_before}"
        )
    text = plan.read_text(encoding="utf-8")
    if text.count(OLD_PLAN) != 1:
        raise SystemExit("P0.3.4 reconstruction-decision block drifted")
    plan.write_text(text.replace(OLD_PLAN, NEW_PLAN, 1), encoding="utf-8")
    actual_after = digest(plan)
    if actual_after != PLAN_AFTER_SHA256:
        raise SystemExit(
            f"P0.3.4 repaired plan digest mismatch: expected {PLAN_AFTER_SHA256}, "
            f"got {actual_after}"
        )
    print(f"P0.3.4 exact plan repair PASS sha256={actual_after}")


def source_segment(text: str, node: ast.AST) -> str:
    segment = ast.get_source_segment(text, node)
    return segment if isinstance(segment, str) else ""


def find_gate_root(function: ast.FunctionDef | ast.AsyncFunctionDef) -> str | None:
    for call in (node for node in ast.walk(function) if isinstance(node, ast.Call)):
        if PREDECESSOR_SCRIPT not in source_segment(_APPLY_TEXT, call):
            continue
        for keyword in call.keywords:
            if keyword.arg == "cwd" and isinstance(keyword.value, ast.Name):
                return keyword.value.id
    argument_names = {
        argument.arg
        for argument in (
            list(function.args.posonlyargs)
            + list(function.args.args)
            + list(function.args.kwonlyargs)
        )
    }
    for candidate in ("root", "repo", "repository", "checkout", "target"):
        if candidate in argument_names:
            return candidate
    return None


def emit_diagnostic(
    *,
    text: str,
    module: ast.Module,
    apply_sha256: str,
    candidates: list[ast.FunctionDef | ast.AsyncFunctionDef],
) -> None:
    functions = []
    for node in ast.walk(module):
        if not isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            continue
        functions.append(
            {
                "name": node.name,
                "line": node.lineno,
                "end_line": node.end_lineno,
                "async": isinstance(node, ast.AsyncFunctionDef),
                "contains_predecessor_script": PREDECESSOR_SCRIPT
                in source_segment(text, node),
                "arguments": [
                    argument.arg
                    for argument in (
                        list(node.args.posonlyargs)
                        + list(node.args.args)
                        + list(node.args.kwonlyargs)
                    )
                ],
            }
        )
    marker_lines = [
        {"line": index, "text": line[:400]}
        for index, line in enumerate(text.splitlines(), start=1)
        if any(
            marker in line
            for marker in (
                PREDECESSOR_SCRIPT,
                "P02_RUST_FILES",
                "transitive_gate_format_clean",
                "run_predecessor",
            )
        )
    ]
    print(
        "P034_APPLY_AST_DIAGNOSTIC="
        + json.dumps(
            {
                "apply_sha256": apply_sha256,
                "candidate_count": len(candidates),
                "functions": functions,
                "marker_lines": marker_lines,
            },
            sort_keys=True,
        ),
        flush=True,
    )


def indent_body(body: str, spaces: int) -> str:
    prefix = " " * spaces
    return "".join(
        (prefix + line if line.strip() else "\n")
        for line in body.splitlines(keepends=True)
    )


_APPLY_TEXT = ""


def patch_predecessor_admission(root: Path) -> None:
    global _APPLY_TEXT
    path = root / "apply.py"
    text = path.read_text(encoding="utf-8")
    _APPLY_TEXT = text
    before = hashlib.sha256(text.encode("utf-8")).hexdigest()
    try:
        module = ast.parse(text, filename=str(path))
    except SyntaxError as error:
        raise SystemExit("P0.3.4 publisher apply.py is not valid Python") from error

    candidates = [
        node
        for node in ast.walk(module)
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef))
        and PREDECESSOR_SCRIPT in source_segment(text, node)
    ]
    emit_diagnostic(
        text=text,
        module=module,
        apply_sha256=before,
        candidates=candidates,
    )
    if len(candidates) != 1:
        raise SystemExit(
            "P0.3.4 publisher must contain exactly one function that owns the "
            "P0.2 predecessor source gate"
        )
    function = candidates[0]
    if not function.body or function.end_lineno is None:
        raise SystemExit("P0.3.4 predecessor-admission source range is unavailable")
    original = source_segment(text, function)
    if "P02_RUST_FILES" not in original or "rustfmt" not in original:
        raise SystemExit(
            "P0.3.4 predecessor-admission implementation drifted from the "
            "known source-plus-format gate"
        )
    root_name = find_gate_root(function)
    if root_name is None:
        raise SystemExit("P0.3.4 predecessor-admission checkout variable is unknown")

    assignments = {
        target.id
        for node in module.body
        if isinstance(node, (ast.Assign, ast.AnnAssign))
        for target in (
            node.targets if isinstance(node, ast.Assign) else [node.target]
        )
        if isinstance(target, ast.Name)
    }
    if "P02_RUST_FILES" not in assignments:
        raise SystemExit("P0.3.4 publisher lost the P0.2 governed-file inventory")

    first_statement = function.body[0]
    if first_statement.lineno is None:
        raise SystemExit("P0.3.4 predecessor-admission body start is unavailable")
    lines = text.splitlines(keepends=True)
    body = COMPAT_BODY.format(root_name=root_name)
    repaired = (
        "".join(lines[: first_statement.lineno - 1])
        + indent_body(body, first_statement.col_offset)
        + "".join(lines[function.end_lineno :])
    )
    try:
        compile(repaired, str(path), "exec")
    except SyntaxError as error:
        raise SystemExit("P0.3.4 repaired publisher is not valid Python") from error
    if repaired.count(PREDECESSOR_SCRIPT) != text.count(PREDECESSOR_SCRIPT):
        raise SystemExit("P0.3.4 predecessor source-gate cardinality drifted")
    path.write_text(repaired, encoding="utf-8")
    after = digest(path)
    if after == before:
        raise SystemExit("P0.3.4 predecessor-admission repair made no change")
    print(
        "P0.3.4 bounded predecessor-admission repair PASS "
        f"function={function.name} root={root_name} before={before} after={after}"
    )


def main() -> int:
    if len(sys.argv) != 2:
        raise SystemExit("usage: p034-rebuild-hotfix.py EXTRACTED_PAYLOAD_DIR")
    root = Path(sys.argv[1]).resolve()
    patch_plan(root)
    patch_predecessor_admission(root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
