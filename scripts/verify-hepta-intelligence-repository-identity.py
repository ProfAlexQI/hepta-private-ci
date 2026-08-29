#!/usr/bin/env python3
"""Fail-closed verifier for the canonical repository identity receipt."""

from __future__ import annotations

import json
import os
from pathlib import Path

PATH = Path("plans/hepta-intelligence/HEPTA_INTELLIGENCE_REPOSITORY_IDENTITY_V1.json")
EXPECTED = {
    "repository": "ProfHepta/hepta-private-ci",
    "repository_id": 1320694176,
    "owner": "ProfHepta",
    "owner_id": 102159240,
}
NEGATIVE = (
    "runtime_wired",
    "external_effects",
    "production_authority",
    "operator_acceptance",
    "promotion",
    "callers_ratchet",
)


def main() -> int:
    value = json.loads(PATH.read_text(encoding="utf-8"))
    assert value.get("schema") == "hepta.intelligence.repository_identity.v1"
    assert value.get("status") == "CANONICAL_REPOSITORY_IDENTITY"
    assert value.get("canonical") == EXPECTED
    assert value.get("historical_aliases") == [
        {
            "repository": "ProfAlexQI/hepta-private-ci",
            "authority": "HISTORICAL_ALIAS_ONLY",
        }
    ]
    assert value.get("migration", {}).get("rewrites_frozen_master_plan") is False
    assert all(value.get(key) is False for key in NEGATIVE)

    repository = os.environ.get("GITHUB_REPOSITORY")
    repository_id = os.environ.get("Q0_REPOSITORY_ID")
    owner_id = os.environ.get("Q0_REPOSITORY_OWNER_ID")
    if repository is not None:
        assert repository == EXPECTED["repository"]
    if repository_id is not None:
        assert int(repository_id) == EXPECTED["repository_id"]
    if owner_id is not None:
        assert int(owner_id) == EXPECTED["owner_id"]
    print("PASS_HEPTA_INTELLIGENCE_REPOSITORY_IDENTITY_V1")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
