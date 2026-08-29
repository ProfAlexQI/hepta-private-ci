#!/usr/bin/env python3
"""Verify the source-controlled Hepta repository-governance contract."""

from __future__ import annotations

import json
import pathlib
import sys
from typing import Any, NoReturn

ROOT = pathlib.Path(__file__).resolve().parents[1]
CONTRACT = ROOT / "docs/governance/HEPTA_REPOSITORY_RULESET_REQUIRED_V1.json"


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_REPOSITORY_GOVERNANCE_V1: {message}")


def load_json(path: pathlib.Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path.relative_to(ROOT)}: {error}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain one object")
    return value


def require(path: pathlib.Path, markers: tuple[str, ...]) -> str:
    try:
        source = path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        fail(f"cannot read {path.relative_to(ROOT)}: {error}")
    for marker in markers:
        if marker not in source:
            fail(f"{path.relative_to(ROOT)} is missing {marker!r}")
    return source


def main() -> int:
    contract = load_json(CONTRACT)
    if (
        contract.get("schema") != "hepta.repository-ruleset-required.v1"
        or contract.get("schemaVersion") != 1
    ):
        fail("ruleset contract schema drifted")
    target = contract.get("target")
    if (
        not isinstance(target, dict)
        or target.get("repository") != "ProfHepta/hepta-private-ci"
        or target.get("ref") != "refs/heads/integration/vnext-main-20260811"
    ):
        fail("ruleset target drifted")
    enforcement = contract.get("enforcement")
    if (
        not isinstance(enforcement, dict)
        or enforcement.get("required") is not True
        or enforcement.get("liveConfigurationIsSourceControlled") is not False
        or enforcement.get("mergeMustRemainBlockedUntilLiveConfigurationMatches") is not True
    ):
        fail("live ruleset enforcement boundary drifted")

    checks = contract.get("requiredStatusChecks")
    if set(checks or []) != {"CI required", "Hepta architecture convergence required"}:
        fail("required status-check set drifted")
    bypass = contract.get("bypass")
    if (
        not isinstance(bypass, dict)
        or bypass.get("repositoryAdministrators") is not False
        or bypass.get("githubActionsSourceMutation") is not False
    ):
        fail("governance contract permits a bypass")

    require(
        ROOT / ".github/CODEOWNERS",
        (
            "* @ProfHepta",
            "/docs/architecture/ @ProfHepta",
            "/docs/governance/ @ProfHepta",
            "/.github/workflows/ @ProfHepta",
            "/codex-rs/hepta-contracts/ @ProfHepta",
            "/codex-rs/hepta-agentd/ @ProfHepta",
        ),
    )
    require(
        ROOT / "README.md",
        (
            "# Hepta private CI mirror",
            "production release channel.",
            "CI is read-only.",
            "HEPTA_ARCHITECTURE_CATALOG_V1.json",
        ),
    )
    require(
        ROOT / "SECURITY.md",
        (
            "# Hepta security policy",
            "Hepta security boundary",
            "Do not include credentials",
            "Qualification success is not security approval.",
        ),
    )
    blocking = require(
        ROOT / ".github/workflows/blocking-ci.yml",
        (
            "uses: ./.github/workflows/hepta-architecture-convergence-p0-2.yml",
            "- hepta-architecture-convergence",
            "name: CI required",
        ),
    )
    if "cancel-in-progress: true" not in blocking:
        fail("blocking CI does not cancel superseded graphs")

    print("PASS_HEPTA_REPOSITORY_GOVERNANCE_SOURCE_CONTRACT_V1")
    return 0


if __name__ == "__main__":
    sys.exit(main())
