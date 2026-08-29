#!/usr/bin/env python3
"""Verify Hepta Intelligence document authority and compatibility boundaries."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import sys
from typing import Any, NoReturn

ROOT = Path(__file__).resolve().parents[1]
PLAN = ROOT / "plans" / "hepta-intelligence"
CURRENT = PLAN / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
DOCUMENT = PLAN / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json"
MASTER = PLAN / "HEPTA_INTELLIGENCE_MASTER_PLAN.md"
SPEC = PLAN / "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md"
EVIDENCE = PLAN / "HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json"
CAPABILITIES = PLAN / "HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json"
PR_STACK = PLAN / "HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json"
INTEGRATION = PLAN / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
STATUS_V2 = PLAN / "HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json"
STATUS_V3 = PLAN / "HEPTA_INTELLIGENCE_EXECUTION_STATUS_V3.json"


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY: {message}")


def require(value: bool, message: str) -> None:
    if not value:
        fail(message)


def load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    require(isinstance(value, dict), f"{path.name} must contain an object")
    return value


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def all_false(value: Any) -> bool:
    return isinstance(value, dict) and bool(value) and all(item is False for item in value.values())


def main() -> int:
    for path in (
        CURRENT, DOCUMENT, MASTER, SPEC, EVIDENCE, CAPABILITIES,
        PR_STACK, INTEGRATION, STATUS_V2, STATUS_V3,
    ):
        require(path.is_file(), f"missing {path.relative_to(ROOT)}")
    current = load(CURRENT)
    document = load(DOCUMENT)
    require(document.get("schema") == "hepta_intelligence_document_authority_registry_v1", "schema")
    require(document.get("repository") == "ProfHepta/hepta-private-ci", "repository")
    registered = document.get("registered_canonical_inputs")
    require(isinstance(registered, list) and len(registered) == 4, "canonical input count")
    paths = [item.get("path") for item in registered if isinstance(item, dict)]
    require(len(paths) == len(set(paths)) == 4, "duplicate canonical input")
    expected = {
        CURRENT.relative_to(ROOT).as_posix(),
        EVIDENCE.relative_to(ROOT).as_posix(),
        CAPABILITIES.relative_to(ROOT).as_posix(),
        MASTER.relative_to(ROOT).as_posix(),
    }
    require(set(paths) == expected, "canonical input surface")
    authority = document.get("current_plan_authority", {})
    require(authority.get("machine_aggregate") == CURRENT.relative_to(ROOT).as_posix(), "machine authority")
    require(authority.get("human_plan") == MASTER.relative_to(ROOT).as_posix(), "human authority")
    require(authority.get("human_plan_version") == "4.3.0", "human plan version")
    require(authority.get("human_plan_content_sha256") == sha(MASTER), "human plan digest")
    require(authority.get("on_mismatch") == "FAIL_CLOSED", "mismatch policy")
    operational = document.get("registered_operational_documents")
    require(isinstance(operational, list) and len(operational) == 1, "operational docs")
    require(operational[0].get("path") == SPEC.relative_to(ROOT).as_posix(), "spec path")
    require(operational[0].get("content_sha256") == sha(SPEC), "spec digest")
    for flag in ("current_plan_authority", "promotion_authority", "production_authority"):
        require(operational[0].get(flag) is False, f"spec authority {flag}")
    require(current.get("session_bootstrap", {}).get("read_order") == [
        CURRENT.relative_to(ROOT).as_posix(),
        DOCUMENT.relative_to(ROOT).as_posix(),
        EVIDENCE.relative_to(ROOT).as_posix(),
        CAPABILITIES.relative_to(ROOT).as_posix(),
        PR_STACK.relative_to(ROOT).as_posix(),
        INTEGRATION.relative_to(ROOT).as_posix(),
        MASTER.relative_to(ROOT).as_posix(),
    ], "read order")
    require(
        current.get("session_bootstrap", {}).get("legacy_machine_contracts_are_current_authority")
        is False,
        "legacy authority",
    )
    require(
        current.get("session_bootstrap", {}).get(
            "legacy_machine_contracts_must_remain_compatible_until_migrated"
        )
        is True,
        "legacy compatibility",
    )
    require(all_false(current.get("authority")), "current authority")
    require(all_false(document.get("authority")), "document authority")
    print("PASS_HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_V4_3")
    return 0


if __name__ == "__main__":
    sys.exit(main())
