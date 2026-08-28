#!/usr/bin/env python3
"""Verify Hepta Intelligence document authority and compatibility contracts."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any, NoReturn

ROOT = Path(__file__).resolve().parents[1]
PLAN_DIR = ROOT / "plans" / "hepta-intelligence"
CURRENT = PLAN_DIR / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
MASTER = PLAN_DIR / "HEPTA_INTELLIGENCE_MASTER_PLAN.md"
REGISTRY = PLAN_DIR / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json"
EVIDENCE = PLAN_DIR / "HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json"
LEGACY_V2 = PLAN_DIR / "HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json"
LEGACY_V3 = PLAN_DIR / "HEPTA_INTELLIGENCE_EXECUTION_STATUS_V3.json"
SNAPSHOTS = {
    "P0.2": PLAN_DIR
    / "status-snapshots/HEPTA_INTELLIGENCE_P0_2_STATUS_SNAPSHOT_V1.json",
    "P0.3": PLAN_DIR
    / "status-snapshots/HEPTA_INTELLIGENCE_P0_3_STATUS_SNAPSHOT_V1.json",
    "P0.4a": PLAN_DIR
    / "status-snapshots/HEPTA_INTELLIGENCE_P0_4A_STATUS_SNAPSHOT_V1.json",
    "P0.4b": PLAN_DIR
    / "status-snapshots/HEPTA_INTELLIGENCE_P0_4B_STATUS_SNAPSHOT_V1.json",
    "P0.4c": PLAN_DIR
    / "status-snapshots/HEPTA_INTELLIGENCE_P0_4C_STATUS_SNAPSHOT_V1.json",
}


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY: {message}")


def load(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except Exception as exc:
        fail(f"cannot parse {path.relative_to(ROOT)}: {exc}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain an object")
    return value


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def all_false(mapping: Any) -> bool:
    return isinstance(mapping, dict) and bool(mapping) and all(
        value is False for value in mapping.values()
    )


def main() -> int:
    for path in [CURRENT, MASTER, REGISTRY, EVIDENCE, LEGACY_V2, LEGACY_V3, *SNAPSHOTS.values()]:
        require(path.is_file(), f"missing {path.relative_to(ROOT)}")

    current = load(CURRENT)
    registry = load(REGISTRY)
    evidence = load(EVIDENCE)
    legacy_v2 = load(LEGACY_V2)
    legacy_v3 = load(LEGACY_V3)

    require(current.get("schema") == "hepta_intelligence_current_plan_v2", "current schema drift")
    require(current.get("canonical", {}).get("human_document") == "plans/hepta-intelligence/HEPTA_INTELLIGENCE_MASTER_PLAN.md", "master pointer drift")
    bootstrap = current.get("session_bootstrap", {})
    require(bootstrap.get("document_authority_registry") == "plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json", "registry pointer drift")
    require(bootstrap.get("time_bounded_evidence_index") == "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json", "evidence pointer drift")
    require(bootstrap.get("legacy_machine_contracts_are_current_authority") is False, "legacy contracts gained current authority")
    require(bootstrap.get("legacy_machine_contracts_must_remain_compatible_until_migrated") is True, "compatibility preservation disabled")
    require(all_false(current.get("authority")), "current authority must remain false")
    require(current.get("active_phase", {}).get("id") == "Q0", "Q0 must remain active")
    require(current.get("active_phase", {}).get("active_task") == "DOC-Q0.1_RESTORE_FROZEN_QUALIFICATION_COMPATIBILITY", "active compatibility task drift")

    require(registry.get("schema") == "hepta_intelligence_document_authority_registry_v1", "registry schema drift")
    rules = registry.get("rules", {})
    require(rules.get("single_current_machine_authority") is True, "single machine authority disabled")
    require(rules.get("single_current_human_authority") is True, "single human authority disabled")
    require(rules.get("compatibility_contracts_are_current_authority") is False, "compatibility contracts gained current authority")
    require(rules.get("breaking_a_registered_consumer_requires_migration_receipt") is True, "consumer migration receipt gate disabled")
    require(registry.get("current_authority", {}).get("machine", {}).get("path") == "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json", "registry machine authority drift")
    require(registry.get("current_authority", {}).get("human", {}).get("path") == "plans/hepta-intelligence/HEPTA_INTELLIGENCE_MASTER_PLAN.md", "registry human authority drift")
    require(all_false(registry.get("authority")), "registry authority must remain false")

    compatibility = registry.get("compatibility_contracts")
    require(isinstance(compatibility, list) and len(compatibility) >= 2, "compatibility contract registry incomplete")
    registered_paths = {item.get("path") for item in compatibility if isinstance(item, dict)}
    require("plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json" in registered_paths, "legacy V2 contract not registered")
    require("plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V3.json" in registered_paths, "legacy V3 contract not registered")
    for item in compatibility:
        require(isinstance(item, dict), "compatibility entry must be an object")
        require(item.get("current_authority") is False, f"compatibility entry gained authority: {item.get('path')}")
        require(item.get("schema_preserved") is True, f"compatibility schema not preserved: {item.get('path')}")
        consumers = item.get("registered_consumers")
        require(isinstance(consumers, list) and consumers, f"compatibility consumers missing: {item.get('path')}")
        for consumer in consumers:
            require((ROOT / consumer).exists(), f"registered consumer missing: {consumer}")

    require(legacy_v2.get("schema") == "hepta_intelligence_execution_status_v2", "legacy V2 schema was replaced")
    require(legacy_v3.get("schema") == "hepta_intelligence_execution_status_v3", "legacy V3 schema was replaced")
    require(all_false(legacy_v2.get("authority")), "legacy V2 authority must remain false")
    require(all_false(legacy_v3.get("authority")), "legacy V3 authority must remain false")

    require(evidence.get("schema") == "hepta_intelligence_evidence_index_v1", "evidence schema drift")
    require(evidence.get("not_current_plan_authority") is True, "evidence index gained plan authority")
    require(isinstance(evidence.get("as_of_utc"), str), "evidence index lacks as_of_utc")
    require(all_false(evidence.get("authority")), "evidence authority must remain false")

    for tranche, path in SNAPSHOTS.items():
        snapshot = load(path)
        require(snapshot.get("schema") == "hepta_intelligence_tranche_status_snapshot_v1", f"snapshot schema drift: {tranche}")
        require(snapshot.get("snapshot_id") == tranche, f"snapshot id drift: {tranche}")
        require(snapshot.get("classification") == "IMMUTABLE_QUALIFICATION_COMPATIBILITY_SNAPSHOT", f"snapshot classification drift: {tranche}")
        require(snapshot.get("current_authority") is False, f"snapshot gained authority: {tranche}")
        require(snapshot.get("current_tranche", {}).get("id") == tranche, f"snapshot current tranche drift: {tranche}")
        require(snapshot.get("current_tranche", {}).get("qualified") is False, f"snapshot unexpectedly qualified: {tranche}")
        require(all_false(snapshot.get("authority")), f"snapshot authority must remain false: {tranche}")
        require(isinstance(snapshot.get("source_ref"), str) and len(snapshot["source_ref"]) == 40, f"snapshot source ref invalid: {tranche}")
        require(isinstance(snapshot.get("source_status_blob_sha"), str) and len(snapshot["source_status_blob_sha"]) == 40, f"snapshot source blob invalid: {tranche}")

    print("PASS_HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_AND_COMPATIBILITY")
    return 0


if __name__ == "__main__":
    sys.exit(main())
