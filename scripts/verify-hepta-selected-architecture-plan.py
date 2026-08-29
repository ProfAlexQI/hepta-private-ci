#!/usr/bin/env python3
"""Run the verifier selected by HEPTA_CURRENT_PLAN.json and fail closed."""

from __future__ import annotations

import json
import pathlib
import subprocess
import sys
from typing import Any, NoReturn

ROOT = pathlib.Path(__file__).resolve().parents[1]
POINTER = ROOT / "docs/architecture/HEPTA_CURRENT_PLAN.json"
VERIFIERS = {
    "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V4.md": ROOT
    / "scripts/verify-hepta-architecture-plan-v4.py",
    "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V5.md": ROOT
    / "scripts/verify-hepta-architecture-plan-v5.py",
}


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_SELECTED_ARCHITECTURE_PLAN: {message}")


def duplicate_safe_object(path: pathlib.Path) -> dict[str, Any]:
    def hook(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key, value in pairs:
            if key in result:
                fail(f"duplicate JSON key {key!r} in {path.relative_to(ROOT)}")
            result[key] = value
        return result

    try:
        value = json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=hook)
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path.relative_to(ROOT)}: {error}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain one JSON object")
    return value


def main() -> int:
    if sys.argv[1:]:
        fail("this verifier router accepts no arguments")
    pointer = duplicate_safe_object(POINTER)
    if pointer.get("schema") != "hepta.current-plan.v1" or pointer.get("schemaVersion") != 1:
        fail("current-plan pointer schema drifted")
    selected = pointer.get("currentPlan")
    if not isinstance(selected, str):
        fail("currentPlan must be a string")
    verifier = VERIFIERS.get(selected)
    if verifier is None:
        fail(f"no allowlisted verifier for selected plan {selected!r}")
    if not verifier.is_file():
        fail(f"selected verifier is missing: {verifier.relative_to(ROOT)}")
    print(f"HEPTA_SELECTED_PLAN={selected}")
    print(f"HEPTA_SELECTED_VERIFIER={verifier.relative_to(ROOT)}")
    return subprocess.run([sys.executable, str(verifier)], cwd=ROOT, check=False).returncode


if __name__ == "__main__":
    raise SystemExit(main())
