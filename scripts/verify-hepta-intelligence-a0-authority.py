#!/usr/bin/env python3
"""Thin wrapper over the strict canonical current-truth validator."""
from __future__ import annotations
import importlib.util
from pathlib import Path
import sys


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    path = root / "scripts" / "hepta-intelligence-current-truth.py"
    spec = importlib.util.spec_from_file_location("hepta_current_truth", path)
    if spec is None or spec.loader is None:
        raise SystemExit("cannot load current-truth validator")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    truth = module.validate_and_build()

    import hashlib
    import json
    import os
    import subprocess
    integration = module.load(module.PATHS["integration"])
    head = tree = None
    if (root / ".git").exists():
        def git(*args: str) -> str:
            return subprocess.check_output(["git", *args], cwd=root, text=True).strip()
        head = git("rev-parse", "HEAD")
        tree = git("rev-parse", "HEAD^{tree}")
        parent = git("rev-parse", "HEAD^")
        module.require(parent == module.Q0_HEAD, f"exact parent mismatch: {parent}")
        changed = sorted(line for line in git("diff", "--name-only", "HEAD^", "HEAD").splitlines() if line)
        module.require(changed == module.ALLOWED_PATHS, f"changed-path surface mismatch: {changed}")
        env_sha = os.environ.get("GITHUB_SHA")
        if env_sha:
            module.require(env_sha == head, "GITHUB_SHA mismatch")
        env_repo = os.environ.get("GITHUB_REPOSITORY")
        if env_repo:
            module.require(env_repo == module.REPOSITORY, "GITHUB_REPOSITORY mismatch")
        env_branch = os.environ.get("GITHUB_HEAD_REF") or os.environ.get("GITHUB_REF_NAME")
        if env_branch:
            module.require(env_branch == module.A0_BRANCH, f"branch mismatch: {env_branch}")
    receipt = {
        "schema": "hepta_intelligence_a0_source_evidence_receipt_v2",
        "status": "PASS_HEPTA_INTELLIGENCE_A0_SOURCE_EVIDENCE",
        "repository": module.REPOSITORY,
        "candidate": {"branch": module.A0_BRANCH, "head": head, "tree": tree, "parent": module.Q0_HEAD},
        "current_truth_sha256": hashlib.sha256(module.canonical(truth)).hexdigest(),
        "source_writeback": False,
        "a0_candidate_qualified": False,
        "selected": False,
        "full_repository_merge_green": False,
        "authority": module.AUTHORITY_FALSE,
    }
    print(json.dumps(receipt, indent=2, sort_keys=True))
    return 0



if __name__ == "__main__":
    raise SystemExit(main())
