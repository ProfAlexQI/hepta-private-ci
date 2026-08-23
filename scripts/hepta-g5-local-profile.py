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
import platform
import subprocess
from pathlib import Path


SCHEMA = "hepta_g5_local_development_profile_v1"
PROFILE_NAME = "local_development"
EXPECTED_WORKTREE = Path("/Volumes/T5/hepta-vnext/worktrees/r2-g5-local-dev-profile-20260824")
EXPECTED_ANCESTOR = "2dae1ae2b09111dad94aebd6788df2d1234217cd"
EXPECTED_HEAD = "c3562438b94a7984ccfa9cdcd0db8abda36a9717"
EXPECTED_TREE = "3e0cdf5c47f98ce0c096e3f4286b31803092349e"
EXPECTED_PARENT = "ee9475c062af5c3dd39936add98257cd9dd1e9b3"
ALLOWED_DELTA_PATHS = frozenset(
    {
        "docs/hepta-vnext/G5_LOCAL_DEVELOPMENT_PROFILE_V1.md",
        "scripts/hepta-g5-local-profile.py",
    }
)
ARTIFACT_ROOT = Path("/Volumes/T5/hepta-vnext/artifacts")


def run_git(candidate: Path, *args: str) -> str:
    result = subprocess.run(
        ["git", "-C", str(candidate), *args],
        check=True,
        capture_output=True,
        text=True,
    )
    return result.stdout.strip()


def git_is_ancestor(candidate: Path, ancestor: str, descendant: str) -> bool:
    result = subprocess.run(
        ["git", "-C", str(candidate), "merge-base", "--is-ancestor", ancestor, descendant],
        check=False,
    )
    return result.returncode == 0


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
    if candidate != EXPECTED_WORKTREE:
        raise SystemExit(f"candidate must be the sealed local profile worktree: {EXPECTED_WORKTREE}")
    if not output.is_relative_to(ARTIFACT_ROOT):
        raise SystemExit(f"output must be under the sealed artifact root: {ARTIFACT_ROOT}")

    dirty = run_git(candidate, "status", "--porcelain")
    if dirty:
        raise SystemExit("candidate worktree must be clean")
    head = run_git(candidate, "rev-parse", "HEAD")
    tree = run_git(candidate, "rev-parse", "HEAD^{tree}")
    parent = run_git(candidate, "rev-parse", "HEAD^")
    if (head, tree, parent) != (EXPECTED_HEAD, EXPECTED_TREE, EXPECTED_PARENT):
        raise SystemExit(
            "candidate identity mismatch; expected exact head/tree/parent "
            f"{EXPECTED_HEAD}/{EXPECTED_TREE}/{EXPECTED_PARENT}"
        )
    if not git_is_ancestor(candidate, EXPECTED_ANCESTOR, head):
        raise SystemExit(f"candidate is not descended from the unified candidate: {EXPECTED_ANCESTOR}")
    delta_paths = [
        path
        for path in run_git(candidate, "diff", "--name-only", parent, head).splitlines()
        if path
    ]
    if set(delta_paths) != ALLOWED_DELTA_PATHS:
        raise SystemExit(
            "local profile commit has an unexpected delta; expected exactly: "
            + ", ".join(sorted(ALLOWED_DELTA_PATHS))
        )
    evidence = []
    for path in args.evidence:
        if path.is_symlink():
            raise SystemExit(f"evidence symlink is not accepted: {path}")
        resolved = path.resolve()
        if not resolved.is_relative_to(ARTIFACT_ROOT) or not resolved.is_file():
            raise SystemExit(f"evidence file does not exist: {resolved}")
        before = resolved.stat()
        digest = sha256_file(resolved)
        after = resolved.stat()
        if before.st_size != after.st_size or before.st_mtime_ns != after.st_mtime_ns:
            raise SystemExit(f"evidence changed while hashing: {resolved}")
        evidence.append(
            {
                "path": str(resolved),
                "sha256": digest,
                "size_bytes": after.st_size,
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
            "basis": "local_untrusted_acknowledgement",
            "identity_asserted_by_cli": False,
            "cryptographic_signature": False,
            "independent_trust_root": False,
            "scope": "development_and_sandbox_only",
        },
        "execution_scope": {
            "declaration_only": True,
            "planning_only": True,
            "production_caller": False,
            "production_writer": False,
            "provider_effects": False,
            "kg_write_authority": False,
            "governance_bypass": False,
            "required_governance_mode": "shadow",
        },
        "runner": {
            "kind": "python_stdlib",
            "python_version": platform.python_version(),
            "script": "scripts/hepta-g5-local-profile.py",
            "dependency_digest": "python-standard-library-only",
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
        "status_axes": {
            "authority_status": "not_granted",
            "operator_status": "local_ack_only",
            "promotion_status": "not_eligible",
            "production_operator_acceptance": False,
        },
        "input_scope": {
            "fixed_local_artifacts_only": True,
            "external_signer_required": False,
            "external_provider_required": False,
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
