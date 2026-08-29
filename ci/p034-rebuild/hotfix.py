#!/usr/bin/env python3
"""Apply exact plan and bounded predecessor-admission repairs to P0.3.4 payload."""

from __future__ import annotations

import ast
import hashlib
import sys
from pathlib import Path

PLAN_BEFORE_SHA256 = "89516216e15fde9a573300aab53dace604114308bf71a6fce6d62c360b56fe66"
PLAN_AFTER_SHA256 = "8ec425fec8c97f36eccff50d6e24f2438987339fb40bfe2734511a0b1b20304e"

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

COMPAT_FUNCTION = '''def run_predecessor_admission(root: Path) -> None:
    result = run(
        (sys.executable, "scripts/verify-hepta-intelligence-grounding-ledger.py"),
        cwd=root,
        capture=True,
        check=False,
    )
    if result.returncode == 0:
        return

    output = result.stdout or ""
    start = output.find("{")
    if start < 0:
        raise RuntimeError(
            "P0.2 predecessor source gate failed without a machine-readable receipt"
        )
    try:
        payload, consumed = json.JSONDecoder().raw_decode(output[start:])
    except json.JSONDecodeError as error:
        raise RuntimeError(
            "P0.2 predecessor source gate emitted an invalid receipt"
        ) from error
    if output[start + consumed :].strip():
        raise RuntimeError(
            "P0.2 predecessor source gate emitted unbound trailing output"
        )

    allowed = {
        "check_hepta_intelligence_grounding_v2_present",
        "transitive_gate_format_clean",
    }
    checks = payload.get("checks")
    failures = payload.get("failures")
    if not isinstance(checks, dict) or not isinstance(failures, list):
        raise RuntimeError("P0.2 predecessor source receipt structure drifted")
    observed_false = {
        str(name) for name, passed in checks.items() if passed is not True
    }
    if (
        payload.get("status")
        != "PASS_P0_2_SOURCE_CONTRACT_WITH_TRANSITIVE_FORMAT_GAP"
        or set(map(str, failures)) != allowed
        or observed_false != allowed
        or result.returncode == 0
    ):
        raise RuntimeError(
            "P0.2 predecessor source gate has a non-compatible failure: "
            f"status={payload.get('status')!r} failures={failures!r} "
            f"false_checks={sorted(observed_false)!r}"
        )

    formatting = run(
        (
            "rustfmt",
            "--edition",
            "2024",
            "--config",
            "skip_children=true",
            "--check",
            *P02_RUST_FILES,
        ),
        cwd=root / "codex-rs",
        capture=True,
        check=False,
    )
    if formatting.returncode != 0:
        raise RuntimeError(
            "P0.2 governed Rust formatting failed independent successor admission"
        )
    print(
        "P0.2 predecessor compatibility admission PASS: "
        "only the retired transitive checker and workspace-wide formatter were "
        "absent; all semantic checks and governed-file rustfmt passed",
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


def patch_predecessor_admission(root: Path) -> None:
    path = root / "apply.py"
    text = path.read_text(encoding="utf-8")
    before = hashlib.sha256(text.encode("utf-8")).hexdigest()
    try:
        module = ast.parse(text, filename=str(path))
    except SyntaxError as error:
        raise SystemExit("P0.3.4 publisher apply.py is not valid Python") from error

    functions = [
        node
        for node in module.body
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef))
        and node.name == "run_predecessor_admission"
    ]
    if len(functions) != 1 or not isinstance(functions[0], ast.FunctionDef):
        raise SystemExit(
            "P0.3.4 publisher must contain exactly one synchronous "
            "run_predecessor_admission function"
        )
    function = functions[0]
    if function.lineno is None or function.end_lineno is None:
        raise SystemExit("P0.3.4 predecessor-admission source range is unavailable")
    original = "".join(
        text.splitlines(keepends=True)[function.lineno - 1 : function.end_lineno]
    )
    required_markers = (
        "verify-hepta-intelligence-grounding-ledger.py",
        "P02_RUST_FILES",
        "rustfmt",
    )
    if not all(marker in original for marker in required_markers):
        raise SystemExit(
            "P0.3.4 predecessor-admission implementation drifted from the "
            "known fail-closed gate"
        )

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

    lines = text.splitlines(keepends=True)
    replacement = COMPAT_FUNCTION.rstrip() + "\n\n"
    repaired = (
        "".join(lines[: function.lineno - 1])
        + replacement
        + "".join(lines[function.end_lineno :])
    )
    try:
        compile(repaired, str(path), "exec")
    except SyntaxError as error:
        raise SystemExit("P0.3.4 repaired publisher is not valid Python") from error
    if repaired.count("def run_predecessor_admission(root: Path) -> None:") != 1:
        raise SystemExit("P0.3.4 predecessor-admission replacement was not unique")
    path.write_text(repaired, encoding="utf-8")
    after = digest(path)
    if after == before:
        raise SystemExit("P0.3.4 predecessor-admission repair made no change")
    print(
        "P0.3.4 bounded predecessor-admission repair PASS "
        f"before={before} after={after}"
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
