#!/usr/bin/env python3
"""Fail-closed source verifier for Hepta architecture convergence P0.5."""

from __future__ import annotations

import json
import pathlib
import subprocess
import sys
from typing import Any, NoReturn

ROOT = pathlib.Path(__file__).resolve().parents[1]
LEGACY_STATUS = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_P0_2_STATUS.json"
EXECUTION_STATUS = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_EXECUTION_STATUS_V2.json"

REQUIRED_FILES = (
    ROOT / "ARCHITECTURE.md",
    ROOT / "README.md",
    ROOT / "SECURITY.md",
    ROOT / ".github/CODEOWNERS",
    ROOT / "docs/architecture/HEPTA_ARCHITECTURE_CATALOG_V1.json",
    ROOT / "docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json",
    ROOT / "docs/architecture/HEPTA_RUNTIME_PROFILE_MATRIX_V1.json",
    ROOT / "docs/architecture/DATA_AUTHORITY_MAP.md",
    ROOT / "docs/architecture/HEPTA_DOCUMENT_AUTHORITY_INDEX_V1.json",
    ROOT / "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V2.md",
    LEGACY_STATUS,
    EXECUTION_STATUS,
    ROOT / "docs/governance/HEPTA_REPOSITORY_RULESET_REQUIRED_V1.json",
    ROOT / "codex-rs/hepta-contracts/src/authority.rs",
    ROOT / "codex-rs/hepta-contracts/src/product_graph.rs",
    ROOT / "codex-rs/hepta-contracts/src/operation.rs",
    ROOT / "codex-rs/hepta-contracts/src/provider_operation.rs",
    ROOT / "codex-rs/hepta-memory-runtime/Cargo.toml",
    ROOT / "codex-rs/hepta-memory-runtime/src/lib.rs",
    ROOT / "codex-rs/hepta-agentd/src/composition.rs",
    ROOT / "codex-rs/hepta-agentd/src/memory_service.rs",
    ROOT / "codex-rs/hepta-agentd/src/automation_service.rs",
)


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_ARCHITECTURE_CONVERGENCE_P0_5: {message}")


def load_json(path: pathlib.Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path.relative_to(ROOT)}: {error}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain one object")
    return value


def run_gate(relative: str, *args: str) -> None:
    result = subprocess.run(
        [sys.executable, str(ROOT / relative), *args],
        cwd=ROOT,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        fail(f"nested source gate failed: {relative}")


def verify_status_boundaries() -> None:
    legacy = load_json(LEGACY_STATUS)
    if legacy.get("deprecated") is not True:
        fail("legacy P0.2 status must be explicitly deprecated")
    if legacy.get("supersededBy") != (
        "docs/architecture/HEPTA_ARCHITECTURE_EXECUTION_STATUS_V2.json"
    ):
        fail("legacy status supersession drifted")
    if legacy.get("remainingBooleanSemantics") != "true_means_outstanding_not_completed":
        fail("legacy remaining booleans remain ambiguous")
    if legacy.get("candidateBinding") != "workflow_runtime_receipt_only":
        fail("legacy status may not self-bind an executable candidate")
    authority = legacy.get("authority")
    if not isinstance(authority, dict) or not authority or any(authority.values()):
        fail("legacy status widened authority")
    qualification = legacy.get("qualification")
    if not isinstance(qualification, dict) or qualification.get("qualified") is not False:
        fail("committed source status may not claim executable qualification")

    execution = load_json(EXECUTION_STATUS)
    policy = execution.get("candidateBindingPolicy")
    if (
        not isinstance(policy, dict)
        or policy.get("committedSourceFileMayClaimExecutableQualification") is not False
    ):
        fail("execution status may self-issue qualification")
    external = execution.get("externalGates")
    if (
        not isinstance(external, dict)
        or external.get("repositoryRuleset", {}).get("state") != "blocked"
    ):
        fail("live repository ruleset gap is not represented honestly")


def main() -> int:
    missing = [str(path.relative_to(ROOT)) for path in REQUIRED_FILES if not path.is_file()]
    if missing:
        fail(f"required files are absent: {missing}")
    run_gate("scripts/generate-hepta-architecture-views.py", "--check")
    run_gate("scripts/verify-hepta-architecture-catalog.py")
    run_gate("scripts/verify-hepta-repository-governance.py")
    verify_status_boundaries()
    print("PASS_ARCHITECTURE_CONVERGENCE_P0_5_CANONICAL_SOURCE_ONLY")
    return 0


if __name__ == "__main__":
    sys.exit(main())
