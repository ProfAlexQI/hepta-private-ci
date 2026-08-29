#!/usr/bin/env python3
"""Publish an already paired-qualified P0.3.4 bundle without rewriting history."""
from __future__ import annotations

import hashlib
import json
import os
import subprocess
import sys
import urllib.parse
import urllib.request
from pathlib import Path
from typing import Sequence

P033_HEAD = "eddcb59ca43a76ac83b64507983bd908f406ff48"
BASE_BRANCH = "codex/hepta-intelligence-evidence-resolver-v4-20260828"
TARGET_BRANCH = "codex/hepta-intelligence-legacy-governance-v3-20260829"
BUNDLE_REF = "refs/heads/p034-candidate-bundle"


def run(args: Sequence[str], cwd: Path, check: bool = True) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(list(args), cwd=cwd, text=True, stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT, check=False)
    if result.stdout:
        print(result.stdout, end="" if result.stdout.endswith("\n") else "\n")
    if check and result.returncode != 0:
        raise RuntimeError(f"command failed ({result.returncode}): {' '.join(args)}")
    return result


def api(method: str, path: str, payload: object | None = None) -> object:
    base = os.environ["GITHUB_API_URL"].rstrip("/")
    token = os.environ["GH_TOKEN"]
    data = None if payload is None else json.dumps(payload).encode()
    request = urllib.request.Request(base + path, data=data, method=method)
    request.add_header("Accept", "application/vnd.github+json")
    request.add_header("Authorization", f"Bearer {token}")
    request.add_header("X-GitHub-Api-Version", "2022-11-28")
    if data is not None:
        request.add_header("Content-Type", "application/json")
    with urllib.request.urlopen(request, timeout=60) as response:
        return json.loads(response.read().decode())


def ls_remote(cwd: Path, branch: str) -> str | None:
    result = run(("git", "ls-remote", "--heads", "origin", f"refs/heads/{branch}"), cwd)
    fields = result.stdout.strip().split()
    return fields[0] if fields else None


def main() -> int:
    if len(sys.argv) != 6:
        raise SystemExit("usage: publish.py TARGET_CHECKOUT X64_EVIDENCE PAIRED_RECEIPT EXPECTED_HEAD OUTPUT_JSON")
    target = Path(sys.argv[1]).resolve()
    evidence = Path(sys.argv[2]).resolve()
    paired_path = Path(sys.argv[3]).resolve()
    expected_head = sys.argv[4]
    output = Path(sys.argv[5]).resolve()
    receipt: dict[str, object] = {
        "schema": "hepta_intelligence_p0_3_4_no_force_publication_v1",
        "base_branch": BASE_BRANCH,
        "base_head": P033_HEAD,
        "target_branch": TARGET_BRANCH,
        "head": expected_head,
        "force_push": False,
        "history_rewrite": False,
        "published": False,
        "draft_pull_request": None,
        "errors": [],
    }
    try:
        paired = json.loads(paired_path.read_text())
        if (
            paired.get("qualified") is not True
            or paired.get("publication_authorized") is not True
            or paired.get("head") != expected_head
            or paired.get("dependency_head") != P033_HEAD
            or paired.get("force_push") is not False
            or paired.get("history_rewrite") is not False
        ):
            raise RuntimeError("paired qualification receipt does not authorize publication")
        bundles = sorted(evidence.rglob("candidate.bundle"))
        bundle_receipts = sorted(evidence.rglob("candidate-bundle-receipt.json"))
        if len(bundles) != 1 or len(bundle_receipts) != 1:
            raise RuntimeError("expected exactly one candidate bundle and receipt")
        bundle = bundles[0]
        bundle_receipt = json.loads(bundle_receipts[0].read_text())
        observed_sha = hashlib.sha256(bundle.read_bytes()).hexdigest()
        if (
            observed_sha != paired.get("candidate_bundle_sha256")
            or observed_sha != bundle_receipt.get("bundle_sha256")
            or bundle_receipt.get("head") != expected_head
            or bundle_receipt.get("base_head") != P033_HEAD
            or bundle_receipt.get("successor_commit_count") != 1
            or bundle_receipt.get("qualified_before_publication") is not True
        ):
            raise RuntimeError("candidate bundle identity or policy mismatch")
        local_base = run(("git", "rev-parse", "HEAD"), target).stdout.strip()
        if local_base != P033_HEAD:
            raise RuntimeError(f"publication checkout is not exact P0.3.3: {local_base}")
        remote_base = ls_remote(target, BASE_BRANCH)
        if remote_base != P033_HEAD:
            raise RuntimeError(f"remote base branch drifted: {remote_base}")
        run(("git", "bundle", "verify", str(bundle)), target)
        run(("git", "fetch", str(bundle), f"{BUNDLE_REF}:refs/remotes/p034/candidate"), target)
        candidate = run(("git", "rev-parse", "refs/remotes/p034/candidate"), target).stdout.strip()
        tree = run(("git", "rev-parse", "refs/remotes/p034/candidate^{{tree}}"), target).stdout.strip()
        parent = run(("git", "rev-parse", "refs/remotes/p034/candidate^"), target).stdout.strip()
        count = run(("git", "rev-list", "--count", f"{P033_HEAD}..refs/remotes/p034/candidate"), target).stdout.strip()
        if candidate != expected_head or parent != P033_HEAD or count != "1" or tree != paired.get("tree"):
            raise RuntimeError("bundled candidate is not the paired exact one-commit successor")
        remote_target = ls_remote(target, TARGET_BRANCH)
        preexisting_exact = remote_target == expected_head
        if remote_target is None:
            run(("git", "push", "origin", f"refs/remotes/p034/candidate:refs/heads/{TARGET_BRANCH}"), target)
            remote_target = ls_remote(target, TARGET_BRANCH)
        elif not preexisting_exact:
            raise RuntimeError(f"target branch already exists at a different head: {remote_target}")
        if remote_target != expected_head:
            raise RuntimeError("ordinary push did not publish the exact paired head")
        repo = os.environ["GITHUB_REPOSITORY"]
        owner = repo.split("/", 1)[0]
        query = urllib.parse.urlencode({"state": "open", "head": f"{owner}:{TARGET_BRANCH}", "base": BASE_BRANCH})
        pulls = api("GET", f"/repos/{repo}/pulls?{query}")
        if not isinstance(pulls, list):
            raise RuntimeError("unexpected pull request query response")
        if len(pulls) > 1:
            raise RuntimeError("multiple open successor pull requests exist")
        if pulls:
            pr = pulls[0]
        else:
            pr = api("POST", f"/repos/{repo}/pulls", {
                "title": "Intelligence P0.3.4: close legacy grounding governance blockers",
                "head": TARGET_BRANCH,
                "base": BASE_BRANCH,
                "draft": True,
                "body": (
                    "Supersedes draft PR #64 without rewriting it. This branch is one exact "
                    "successor of P0.3.3 and was qualified before publication on the same head "
                    "and tree by Linux X64 and ARM64. All production-authority flags remain false."
                ),
            })
        if not isinstance(pr, dict) or pr.get("draft") is not True:
            raise RuntimeError("successor pull request is not Draft")
        if (pr.get("head") or {}).get("ref") != TARGET_BRANCH or (pr.get("base") or {}).get("ref") != BASE_BRANCH:
            raise RuntimeError("successor pull request base/head mismatch")
        api("POST", f"/repos/{repo}/statuses/{expected_head}", {
            "state": "success",
            "context": "Hepta Intelligence P0.3.4 paired qualification",
            "description": "P0.3.4 X64/ARM64 qualified before ordinary publication",
            "target_url": f"{os.environ['GITHUB_SERVER_URL']}/{repo}/actions/runs/{os.environ['GITHUB_RUN_ID']}",
        })
        receipt.update({
            "tree": tree,
            "bundle_sha256": observed_sha,
            "branch_preexisting_exact": preexisting_exact,
            "published": True,
            "draft_pull_request": {"number": pr.get("number"), "url": pr.get("html_url"), "draft": pr.get("draft")},
        })
    except Exception as error:
        receipt["errors"] = [str(error)]
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
    print(json.dumps(receipt, indent=2, sort_keys=True))
    return 0 if receipt.get("published") is True and not receipt.get("errors") else 1


if __name__ == "__main__":
    raise SystemExit(main())
