#!/usr/bin/env python3
"""Create a development-only G5 profile without an external ceremony.

This profile deliberately does not grant production authority.  It records the
exact clean candidate, the bounded evidence that was actually supplied, and a
local operator acknowledgement so development can continue without a remote
signer or a provider-owned exactly-once contract.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
from pathlib import Path


SCHEMA = "hepta_g5_local_development_profile_v1"
PROFILE_NAME = "local_development"


def run_git(candidate: Path, *args: str) -> str:
    result = subprocess.run(
        ["git", "-C", str(candidate), *args],
        check=True,
        capture_output=True,
        text=True,
    )
    return result.stdout.strip()


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


def canonical_json(value: object) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode()


def write_json(path: Path, value: object) -> bytes:
    data = canonical_json(value)
    path.write_bytes(data)
    return data


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--candidate", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--operator", default="local-development-user")
    parser.add_argument(
        "--ack",
        action="store_true",
        help="record the current user's local development acknowledgement",
    )
    parser.add_argument("--evidence", type=Path, action="append", default=[])
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    candidate = args.candidate.resolve()
    output = args.output.resolve()
    if not args.ack:
        raise SystemExit("refusing to create a local profile without --ack")
    if not candidate.is_dir():
        raise SystemExit(f"candidate is not a directory: {candidate}")

    dirty = run_git(candidate, "status", "--porcelain")
    if dirty:
        raise SystemExit("candidate worktree must be clean")
    head = run_git(candidate, "rev-parse", "HEAD")
    tree = run_git(candidate, "rev-parse", "HEAD^{tree}")
    parent = run_git(candidate, "rev-parse", "HEAD^")
    delta_paths = [
        path
        for path in run_git(candidate, "diff", "--name-only", parent, head).splitlines()
        if path
    ]
    if any(not (path.startswith("docs/") or path.startswith("scripts/")) for path in delta_paths):
        raise SystemExit("local profile commit must be docs/scripts-only")
    evidence = []
    for path in args.evidence:
        resolved = path.resolve()
        if not resolved.is_file():
            raise SystemExit(f"evidence file does not exist: {resolved}")
        evidence.append(
            {
                "path": str(resolved),
                "sha256": sha256_file(resolved),
                "size_bytes": resolved.stat().st_size,
            }
        )

    output.mkdir(parents=True, exist_ok=False)
    profile = {
        "schema": SCHEMA,
        "schema_version": 1,
        "profile": PROFILE_NAME,
        "candidate": {
            "head": head,
            "tree": tree,
            "parent": parent,
            "worktree": str(candidate),
            "clean": True,
            "delta_paths": delta_paths,
            "product_paths_unchanged_from_parent": True,
        },
        "local_acknowledgement": {
            "operator": args.operator,
            "basis": "interactive_user_authorization",
            "cryptographic_signature": False,
            "independent_trust_root": False,
            "scope": "development_and_sandbox_only",
        },
        "provider_effect_policy": {
            "mode": "at_least_once_indeterminate_reconcile",
            "external_effects": False,
            "unknown_result": "Indeterminate",
            "blind_retry": False,
            "physical_exactly_once": False,
        },
        "authority": {
            "g5_local_complete": True,
            "local_operator_acceptance": True,
            "local_fleet_shadow_allowed": True,
            "production_activation": False,
            "promotion": False,
            "g5_allowed": False,
            "fleet_and_automation_unfrozen": False,
            "provider_physical_exactly_once": False,
        },
        "external_inputs_required_for_this_profile": [],
        "external_inputs_deferred_to_production_profile": [
            "provider-owned occurrence dedupe/status/effect acknowledgement contract",
            "independent signer/trust policy for production promotion",
        ],
        "evidence": evidence,
        "claims": [
            "local development may proceed without external ceremony",
            "provider uncertainty remains quarantined and reconciled",
            "this profile cannot authorize production effects or promotion",
        ],
    }
    profile_path = output / "G5-LOCAL-DEVELOPMENT-PROFILE.json"
    profile_bytes = write_json(profile_path, profile)
    sums = f"{sha256_bytes(profile_bytes)}  {profile_path.name}\n"
    (output / "SHA256SUMS").write_text(sums, encoding="utf-8")
    (output / "README.txt").write_text(
        "Development-only profile. It removes external ceremony from local work; "
        "it does not grant production authority or claim provider exactly-once.\n",
        encoding="utf-8",
    )
    print(json.dumps({"profile": str(profile_path), "sha256": sha256_bytes(profile_bytes)}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
