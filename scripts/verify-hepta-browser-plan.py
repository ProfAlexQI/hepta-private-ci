#!/usr/bin/env python3
"""Verify the repository-native Hepta Browser plan without third-party packages."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs" / "hepta-vnext"
EXPECTED_STAGES = [f"WEB-C{index}" for index in range(8)]
SHA40 = re.compile(r"^[0-9a-f]{40}$")


class VerificationError(RuntimeError):
    """Raised when an active browser-plan invariant is violated."""


def require(condition: bool, message: str) -> None:
    if not condition:
        raise VerificationError(message)


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise VerificationError(f"cannot parse {path.relative_to(ROOT)}: {error}") from error
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must contain one object")
    return value


def verify_authority(authority: dict[str, Any], source: str) -> None:
    expected = {
        "qualification_only": True,
        "production_caller": False,
        "production_writer": False,
        "effect_authority": False,
        "external_effect": False,
        "operator_acceptance": False,
        "promotion": False,
        "g5_allowed": False,
        "execute_allowed": False,
    }
    require(authority == expected, f"{source} authority is not exactly fail-closed")


def verify_documents() -> tuple[dict[str, Any], dict[str, Any], dict[str, Any]]:
    current = load_json(DOCS / "HEPTA_VNEXT_CURRENT.yaml")
    matrix = load_json(DOCS / "HEPTA_BROWSER_STAGE_MATRIX_v1_4.yaml")
    trace = load_json(DOCS / "HEPTA_BROWSER_TRACEABILITY_v1.yaml")
    receipt_schema = load_json(DOCS / "hepta.browser_receipt.v2.schema.json")
    binding = load_json(DOCS / "HEPTA_BROWSER_IMPLEMENTATION_BINDING.json")

    require(current.get("schema") == "hepta.vnext.current.v2", "unexpected current schema")
    require(current.get("phase") == "DEVELOPMENT", "browser plan must remain DEVELOPMENT")
    active_plan = current.get("active_plan")
    require(isinstance(active_plan, dict), "current active_plan is missing")
    require(active_plan.get("plan_id") == "WEB-PLAN-2026-08-27E", "wrong active plan")
    require(active_plan.get("stages") == EXPECTED_STAGES, "active stages are not WEB-C0..WEB-C7")
    require(active_plan.get("unique_live_engine") == "servo", "Servo must be the unique engine")
    require(
        active_plan.get("fixture_engine") == "qualification_only",
        "fixture engine must be qualification-only",
    )
    authority = current.get("authority")
    require(isinstance(authority, dict), "current authority is missing")
    verify_authority(authority, "current")
    require(
        binding.get("schema") == "hepta.browser.implementation-binding.v1",
        "wrong implementation binding schema",
    )
    binding_authority = binding.get("authority")
    require(isinstance(binding_authority, dict), "binding authority is missing")
    verify_authority(binding_authority, "implementation binding")
    candidate_commit = binding.get("candidate_commit")
    candidate_tree = binding.get("candidate_tree")
    require(
        isinstance(candidate_commit, str) and SHA40.fullmatch(candidate_commit),
        "binding candidate_commit is not an exact SHA",
    )
    require(
        isinstance(candidate_tree, str) and SHA40.fullmatch(candidate_tree),
        "binding candidate_tree is not an exact SHA",
    )
    binding_path = "docs/hepta-vnext/HEPTA_BROWSER_IMPLEMENTATION_BINDING.json"
    require(current.get("implementation_commit") == candidate_commit, "current commit binding drifted")
    require(current.get("implementation_tree") == candidate_tree, "current tree binding drifted")
    require(current.get("implementation_binding") == binding_path, "current binding path drifted")

    require(matrix.get("schema") == "hepta.browser.stage-matrix.v1.4", "wrong matrix schema")
    matrix_authority = matrix.get("authority")
    require(isinstance(matrix_authority, dict), "matrix authority is missing")
    verify_authority(matrix_authority, "stage matrix")
    stages = matrix.get("stages")
    require(isinstance(stages, list), "stage matrix stages are missing")
    stage_ids = [stage.get("id") for stage in stages if isinstance(stage, dict)]
    require(stage_ids == EXPECTED_STAGES, "stage matrix order/set differs from WEB-C0..WEB-C7")
    for stage in stages:
        require(isinstance(stage, dict), "stage entry must be an object")
        stage_authority = stage.get("authority")
        require(isinstance(stage_authority, dict), f"{stage.get('id')} authority is missing")
        verify_authority(stage_authority, str(stage.get("id")))

    require(trace.get("schema") == "hepta.browser.traceability.v1", "wrong trace schema")
    require(trace.get("implementation_commit") == candidate_commit, "trace commit binding drifted")
    require(trace.get("implementation_tree") == candidate_tree, "trace tree binding drifted")
    require(trace.get("implementation_binding") == binding_path, "trace binding path drifted")
    requirements = trace.get("requirements")
    require(isinstance(requirements, list) and requirements, "traceability requirements are empty")
    requirement_ids = [item.get("id") for item in requirements if isinstance(item, dict)]
    require(len(requirement_ids) == len(set(requirement_ids)), "duplicate traceability requirement ID")
    require(
        {item.get("stage") for item in requirements if isinstance(item, dict)}
        <= set(EXPECTED_STAGES),
        "traceability references an inactive stage",
    )

    definitions = receipt_schema.get("$defs")
    require(isinstance(definitions, dict), "receipt schema definitions are missing")
    receipt_authority = definitions.get("authority")
    require(isinstance(receipt_authority, dict), "receipt authority schema is missing")
    properties = receipt_authority.get("properties")
    require(isinstance(properties, dict), "receipt authority properties are missing")
    for key, value in {
        "qualification_only": True,
        "production_caller": False,
        "production_writer": False,
        "effect_authority": False,
        "operator_acceptance": False,
        "promotion": False,
        "g5_allowed": False,
        "execute_allowed": False,
        "external_effect": False,
    }.items():
        require(properties.get(key) == {"const": value}, f"receipt schema widens {key}")

    return current, matrix, trace


def verify_code(trace: dict[str, Any]) -> None:
    contract_path = ROOT / "codex-rs" / "hepta-shadow-qualification" / "src" / "browser_contracts.rs"
    runtime_root = ROOT / "codex-rs" / "hepta-shadow-qualification" / "src" / "browser_runtime"
    runtime_paths = (
        runtime_root / "mod.rs",
        runtime_root / "engine.rs",
        runtime_root / "actor.rs",
        runtime_root / "actor_support.rs",
    )
    tests_path = ROOT / "codex-rs" / "hepta-shadow-qualification" / "src" / "browser_tests.rs"
    lib_path = ROOT / "codex-rs" / "hepta-shadow-qualification" / "src" / "lib.rs"
    for path in (contract_path, *runtime_paths, tests_path, lib_path):
        require(path.is_file(), f"missing implementation file {path.relative_to(ROOT)}")

    contracts = contract_path.read_text(encoding="utf-8")
    runtime = "\n".join(path.read_text(encoding="utf-8") for path in runtime_paths)
    tests = tests_path.read_text(encoding="utf-8")
    lib = lib_path.read_text(encoding="utf-8")

    for constant in (
        "BROWSER_PRODUCTION_CALLER: bool = false",
        "BROWSER_PRODUCTION_WRITER: bool = false",
        "BROWSER_EFFECT_AUTHORITY: bool = false",
        "BROWSER_OPERATOR_ACCEPTANCE: bool = false",
        "BROWSER_PROMOTION: bool = false",
        "BROWSER_G5_ALLOWED: bool = false",
        "BROWSER_EXECUTE_ALLOWED: bool = false",
        "BROWSER_EXTERNAL_EFFECT: bool = false",
    ):
        require(constant in contracts, f"missing closed authority constant {constant}")
    require("pub struct BrowserActor" in runtime, "BrowserActor implementation is missing")
    require("raw_secret_bytes_present" in runtime, "engine privacy marker is missing")
    require("RequestIdConflict" in runtime, "request conflict fence is missing")
    require("StaleSemanticRef" in runtime, "semantic ref fence is missing")
    require("pub use browser_runtime::BrowserActor" in lib, "browser API is not exported")

    requirements = trace.get("requirements", [])
    for item in requirements:
        if not isinstance(item, dict):
            continue
        for reference in item.get("code", []):
            path_text, _, symbol = reference.partition(":")
            path = ROOT / path_text
            require(path.is_file(), f"traceability code path does not exist: {path_text}")
            if symbol:
                require(
                    symbol in path.read_text(encoding="utf-8"),
                    f"traceability symbol {symbol} is absent from {path_text}",
                )
        for test_name in item.get("tests", []):
            require(f"fn {test_name}" in tests, f"traceability test is missing: {test_name}")

    cargo = (ROOT / "codex-rs" / "Cargo.toml").read_text(encoding="utf-8").lower()
    require("obscura" not in cargo, "Obscura must not enter the live dependency graph")


def verify_ci_and_ownership() -> None:
    workflow_path = ROOT / ".github" / "workflows" / "hepta-vnext-qualification.yml"
    blocking_path = ROOT / ".github" / "workflows" / "blocking-ci.yml"
    owners_path = ROOT / ".github" / "CODEOWNERS"
    for path in (workflow_path, blocking_path, owners_path):
        require(path.is_file(), f"missing governance file {path.relative_to(ROOT)}")
    workflow = workflow_path.read_text(encoding="utf-8")
    blocking = blocking_path.read_text(encoding="utf-8")
    owners = owners_path.read_text(encoding="utf-8")
    require("integration/vnext-main-20260811" in workflow, "Hepta workflow misses default branch")
    require("pull_request:" in workflow, "Hepta workflow has no pull-request trigger")
    require("browser-c0-c3:" in workflow, "focused browser qualification job is missing")
    require("verify-hepta-browser-plan.py" in workflow, "plan verifier is not run by CI")
    require("integration/vnext-main-20260811" in blocking, "blocking CI misses default branch")
    for pattern in (
        "/codex-rs/hepta-* @ProfAlexQI",
        "/docs/hepta-vnext/ @ProfAlexQI",
        "/.github/workflows/hepta-vnext-qualification.yml @ProfAlexQI",
    ):
        require(pattern in owners, f"CODEOWNERS is missing {pattern}")


def main() -> int:
    try:
        _, _, trace = verify_documents()
        verify_code(trace)
        verify_ci_and_ownership()
    except VerificationError as error:
        print(f"HEPTA_BROWSER_PLAN_VERIFY=FAIL: {error}", file=sys.stderr)
        return 1
    print("HEPTA_BROWSER_PLAN_VERIFY=PASS")
    print("active_plan=WEB-PLAN-2026-08-27E")
    print("active_stages=WEB-C0,WEB-C1,WEB-C2,WEB-C3,WEB-C4,WEB-C5,WEB-C6,WEB-C7")
    print("authority=qualification_only_no_production_no_effect_no_promotion")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
