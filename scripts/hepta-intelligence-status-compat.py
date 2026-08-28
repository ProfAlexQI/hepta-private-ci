#!/usr/bin/env python3
"""Materialize immutable tranche status snapshots for frozen qualification gates.

The legacy P0 source verifiers predate the canonical master plan and all read the
same mutable EXECUTION_STATUS_V2 path.  A stacked branch cannot make that one
path describe P0.2, P0.3, P0.4a, P0.4b, and P0.4c simultaneously.  This helper
validates a registered immutable snapshot and copies it into the legacy path in
the ephemeral CI worktree.  It never edits Git history and never grants current
plan or runtime authority to the snapshot.
"""

from __future__ import annotations

import argparse
import json
import shutil
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
PLAN_DIR = ROOT / "plans" / "hepta-intelligence"
LEGACY_STATUS = PLAN_DIR / "HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json"
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


def load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise SystemExit(f"snapshot must be an object: {path}")
    return value


def validate(tranche: str, snapshot: dict[str, Any]) -> None:
    if snapshot.get("schema") != "hepta_intelligence_tranche_status_snapshot_v1":
        raise SystemExit(f"unsupported snapshot schema for {tranche}")
    if snapshot.get("snapshot_id") != tranche:
        raise SystemExit(f"snapshot identity mismatch for {tranche}")
    if snapshot.get("classification") != "IMMUTABLE_QUALIFICATION_COMPATIBILITY_SNAPSHOT":
        raise SystemExit(f"snapshot classification mismatch for {tranche}")
    if snapshot.get("current_authority") is not False:
        raise SystemExit(f"snapshot unexpectedly gained current authority for {tranche}")
    current = snapshot.get("current_tranche")
    if not isinstance(current, dict) or current.get("id") != tranche:
        raise SystemExit(f"snapshot current_tranche mismatch for {tranche}")
    if current.get("qualified") is not False:
        raise SystemExit(f"snapshot must remain unqualified for {tranche}")
    authority = snapshot.get("authority")
    if not isinstance(authority, dict) or not authority:
        raise SystemExit(f"snapshot authority object missing for {tranche}")
    if any(value is not False for value in authority.values()):
        raise SystemExit(f"snapshot authority must remain false for {tranche}")
    source_ref = snapshot.get("source_ref")
    source_blob = snapshot.get("source_status_blob_sha")
    if not isinstance(source_ref, str) or len(source_ref) != 40:
        raise SystemExit(f"snapshot source_ref is invalid for {tranche}")
    if not isinstance(source_blob, str) or len(source_blob) != 40:
        raise SystemExit(f"snapshot source blob is invalid for {tranche}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("tranche", choices=sorted(SNAPSHOTS))
    parser.add_argument(
        "--check-only",
        action="store_true",
        help="validate without materializing the legacy path",
    )
    args = parser.parse_args()

    path = SNAPSHOTS[args.tranche]
    if not path.is_file():
        raise SystemExit(f"missing snapshot: {path.relative_to(ROOT)}")
    snapshot = load(path)
    validate(args.tranche, snapshot)
    if not args.check_only:
        shutil.copyfile(path, LEGACY_STATUS)
    print(
        json.dumps(
            {
                "status": "PASS_TRANCHE_STATUS_SNAPSHOT",
                "tranche": args.tranche,
                "snapshot": str(path.relative_to(ROOT)),
                "materialized": not args.check_only,
                "current_authority": False,
                "runtime_authority": False,
            },
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
