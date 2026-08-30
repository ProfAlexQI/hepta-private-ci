#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path
from typing import Any

SHA1_RE = re.compile(r"^[0-9a-f]{40}$")
MANIFEST_PATH = (
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
)
INTEGRATION_BRANCH_PREFIX = "integration/hepta-intelligence-a0-q0-"


class AdmissionError(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AdmissionError(message)


def require_sha(value: str, label: str) -> str:
    require(bool(SHA1_RE.fullmatch(value)), f"{label} must be a lowercase SHA-1")
    return value


def run_git(root: Path, *args: str) -> str:
    completed = subprocess.run(
        ["git", *args],
        cwd=root,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    if completed.returncode != 0:
        raise AdmissionError(
            f"git {' '.join(args)} failed ({completed.returncode}): "
            f"{completed.stderr.strip()}"
        )
    return completed.stdout


def reject_duplicate_keys(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    value: dict[str, Any] = {}
    for key, item in pairs:
        if key in value:
            raise AdmissionError(f"duplicate JSON key in A0 manifest: {key}")
        value[key] = item
    return value


def read_json_at(root: Path, commit: str, path: str) -> dict[str, Any]:
    raw = run_git(root, "show", f"{commit}:{path}")
    try:
        value = json.loads(raw, object_pairs_hook=reject_duplicate_keys)
    except json.JSONDecodeError as error:
        raise AdmissionError(f"invalid JSON at {commit}:{path}: {error}") from error
    require(isinstance(value, dict), f"{commit}:{path} must contain a JSON object")
    return value


def commit_parents(root: Path, commit: str) -> list[str]:
    parents = run_git(root, "show", "-s", "--format=%P", commit).strip().split()
    for index, parent in enumerate(parents):
        require_sha(parent, f"parent[{index}]")
    return parents


def tree_sha(root: Path, commit: str) -> str:
    return require_sha(
        run_git(root, "rev-parse", f"{commit}^{{tree}}").strip(),
        f"tree for {commit}",
    )


def changed_paths(root: Path, base: str, head: str) -> list[str]:
    raw = subprocess.run(
        ["git", "diff", "--name-only", "--no-renames", "-z", base, head],
        cwd=root,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if raw.returncode != 0:
        raise AdmissionError(
            f"git diff {base} {head} failed ({raw.returncode}): "
            f"{raw.stderr.decode('utf-8', errors='replace').strip()}"
        )
    try:
        paths = [item.decode("utf-8") for item in raw.stdout.split(b"\0") if item]
    except UnicodeDecodeError as error:
        raise AdmissionError("changed path is not valid UTF-8") from error
    require(paths == sorted(paths), "git changed-path output is not sorted")
    require(len(paths) == len(set(paths)), "git changed-path output contains duplicates")
    return paths


def tree_entry(root: Path, commit: str, path: str) -> tuple[str, str, str]:
    raw = subprocess.run(
        ["git", "ls-tree", "-z", commit, "--", path],
        cwd=root,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if raw.returncode != 0:
        raise AdmissionError(
            f"git ls-tree {commit} -- {path} failed ({raw.returncode}): "
            f"{raw.stderr.decode('utf-8', errors='replace').strip()}"
        )
    entries = [item for item in raw.stdout.split(b"\0") if item]
    require(len(entries) == 1, f"expected exactly one tree entry for {commit}:{path}")
    try:
        metadata, encoded_path = entries[0].split(b"\t", 1)
        mode, object_type, sha = metadata.decode("ascii").split(" ", 2)
        observed_path = encoded_path.decode("utf-8")
    except (ValueError, UnicodeDecodeError) as error:
        raise AdmissionError(f"malformed tree entry for {commit}:{path}") from error
    require(observed_path == path, f"tree entry path drifted for {commit}:{path}")
    require(object_type == "blob", f"A0 path is not a blob: {path}")
    require_sha(sha, f"blob for {commit}:{path}")
    return mode, object_type, sha


def digest_lines(lines: list[str]) -> str:
    payload = "".join(f"{line}\n" for line in lines).encode("utf-8")
    return hashlib.sha256(payload).hexdigest()


def fetch_commit_metadata(repository: str, head: str, token: str) -> dict[str, Any]:
    owner, separator, name = repository.partition("/")
    require(separator == "/" and owner and name and "/" not in name, "invalid repository")
    url = (
        "https://api.github.com/repos/"
        f"{urllib.parse.quote(owner, safe='')}/"
        f"{urllib.parse.quote(name, safe='')}/commits/{head}"
    )
    request = urllib.request.Request(
        url,
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {token}",
            "User-Agent": "hepta-intelligence-integration-admission-v1",
            "X-GitHub-Api-Version": "2022-11-28",
        },
        method="GET",
    )
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            require(response.status == 200, f"GitHub commit API returned {response.status}")
            payload = response.read(2 * 1024 * 1024 + 1)
    except (urllib.error.HTTPError, urllib.error.URLError, TimeoutError) as error:
        raise AdmissionError(f"GitHub commit API request failed: {error}") from error
    require(len(payload) <= 2 * 1024 * 1024, "GitHub commit API payload exceeds 2 MiB")
    try:
        value = json.loads(payload)
    except json.JSONDecodeError as error:
        raise AdmissionError(f"GitHub commit API returned invalid JSON: {error}") from error
    require(isinstance(value, dict), "GitHub commit API payload must be an object")
    return value


def verify(
    *,
    root: Path,
    repository: str,
    expected_head: str,
    expected_base: str,
    expected_base_branch: str,
    expected_head_branch: str,
    token: str,
) -> dict[str, Any]:
    expected_head = require_sha(expected_head, "expected head")
    expected_base = require_sha(expected_base, "expected base")
    require(
        expected_head_branch.startswith(INTEGRATION_BRANCH_PREFIX),
        "integration head branch is outside the governed prefix",
    )
    require(expected_base_branch.strip() != "", "expected base branch is empty")

    actual_head = require_sha(run_git(root, "rev-parse", "HEAD").strip(), "actual head")
    require(actual_head == expected_head, "checked-out head does not match the PR head")
    actual_tree = tree_sha(root, actual_head)
    parents = commit_parents(root, actual_head)
    require(len(parents) == 2, "integration admission head must have exactly two parents")
    first_parent, second_parent = parents
    require(first_parent == expected_base, "A0 must be the first integration parent")
    require(first_parent != second_parent, "integration parents must be distinct")

    manifest = read_json_at(root, first_parent, MANIFEST_PATH)
    require(
        manifest.get("branch") == expected_base_branch,
        "PR base branch does not match the canonical A0 manifest branch",
    )
    require(
        manifest.get("classification") == "SOURCE_ONLY_GOVERNANCE_CANDIDATE",
        "first parent is not the canonical source-only A0 candidate",
    )
    policy = manifest.get("candidate_provenance_policy")
    require(isinstance(policy, dict), "A0 candidate provenance policy is missing")
    require(
        policy.get("commit_must_have_exactly_one_parent") is True,
        "A0 manifest no longer requires a sole parent",
    )
    require(
        policy.get("candidate_workflow_may_write_source") is False,
        "A0 manifest permits candidate workflow source writeback",
    )

    a0_parent = require_sha(str(manifest.get("expected_parent", "")), "A0 expected parent")
    first_parents = commit_parents(root, first_parent)
    require(first_parents == [a0_parent], "canonical A0 parent topology drifted")
    second_parents = commit_parents(root, second_parent)
    require(len(second_parents) == 1, "selected Q0 source parent must not be a merge commit")

    allowlist_value = manifest.get("allowed_changed_paths")
    require(isinstance(allowlist_value, list), "A0 allowed_changed_paths is missing")
    require(
        all(isinstance(path, str) and path for path in allowlist_value),
        "A0 allowed_changed_paths contains an invalid path",
    )
    allowlist = sorted(allowlist_value)
    require(allowlist == allowlist_value, "A0 allowed_changed_paths is not sorted")
    require(len(allowlist) == len(set(allowlist)), "A0 allowlist contains duplicates")
    require(
        manifest.get("expected_changed_path_count") == len(allowlist),
        "A0 expected changed-path count does not match its allowlist",
    )
    require(
        changed_paths(root, a0_parent, first_parent) == allowlist,
        "canonical A0 first-parent delta no longer equals its registered allowlist",
    )

    authority = manifest.get("authority")
    require(isinstance(authority, dict) and authority, "A0 authority map is missing")
    escaped = sorted(key for key, value in authority.items() if value is not False)
    require(not escaped, f"positive or non-boolean A0 authority escaped: {escaped}")

    preserved_entries: list[str] = []
    for path in allowlist:
        first_entry = tree_entry(root, first_parent, path)
        head_entry = tree_entry(root, actual_head, path)
        require(head_entry == first_entry, f"integration head changed canonical A0 path: {path}")
        preserved_entries.append("\0".join((path, *first_entry)))

    q0_to_integration = changed_paths(root, second_parent, actual_head)
    require(
        q0_to_integration == allowlist,
        "integration tree is not an exact canonical-A0 overlay on the selected Q0 parent",
    )
    a0_to_integration = changed_paths(root, first_parent, actual_head)
    leaked_a0_paths = sorted(set(a0_to_integration).intersection(allowlist))
    require(not leaked_a0_paths, f"A0 paths changed relative to first parent: {leaked_a0_paths}")

    tracked_status = run_git(root, "status", "--porcelain", "--untracked-files=no")
    require(tracked_status == "", "tracked worktree is not clean during identity verification")

    api = fetch_commit_metadata(repository, actual_head, token)
    require(api.get("sha") == actual_head, "GitHub API head identity drifted")
    api_commit = api.get("commit")
    require(isinstance(api_commit, dict), "GitHub API commit object is missing")
    api_tree = api_commit.get("tree")
    require(isinstance(api_tree, dict), "GitHub API tree object is missing")
    require(api_tree.get("sha") == actual_tree, "GitHub API tree differs from local Git")
    api_parents = api.get("parents")
    require(isinstance(api_parents, list), "GitHub API parents are missing")
    api_parent_shas = [item.get("sha") for item in api_parents if isinstance(item, dict)]
    require(api_parent_shas == parents, "GitHub API parent order differs from local Git")

    verification = api_commit.get("verification")
    require(isinstance(verification, dict), "GitHub signature verification is missing")
    require(verification.get("verified") is True, "integration commit signature is not verified")
    require(verification.get("reason") == "valid", "integration signature reason is not valid")
    signature = verification.get("signature")
    payload = verification.get("payload")
    require(isinstance(signature, str) and signature, "verified signature bytes are missing")
    require(isinstance(payload, str) and payload, "verified signature payload is missing")

    return {
        "schema": "hepta_intelligence_integration_admission_identity_v1",
        "status": "PASS_HEPTA_INTELLIGENCE_INTEGRATION_ADMISSION_IDENTITY_ONLY",
        "repository": repository,
        "head_branch": expected_head_branch,
        "base_branch": expected_base_branch,
        "head": actual_head,
        "tree": actual_tree,
        "first_parent_a0": first_parent,
        "first_parent_tree": tree_sha(root, first_parent),
        "a0_parent": a0_parent,
        "second_parent_q0": second_parent,
        "second_parent_tree": tree_sha(root, second_parent),
        "second_parent_sole_parent": second_parents[0],
        "a0_overlay_path_count": len(allowlist),
        "a0_overlay_paths_sha256": digest_lines(allowlist),
        "a0_overlay_entries_sha256": digest_lines(preserved_entries),
        "q0_delta_path_count": len(a0_to_integration),
        "q0_delta_paths_sha256": digest_lines(a0_to_integration),
        "github_signature": {
            "verified": True,
            "reason": "valid",
            "verified_at": verification.get("verified_at"),
            "signature_sha256": hashlib.sha256(signature.encode("utf-8")).hexdigest(),
            "payload_sha256": hashlib.sha256(payload.encode("utf-8")).hexdigest(),
        },
        "source_writeback": False,
        "a0_candidate_qualified": False,
        "selected": False,
        "full_repository_merge_green": False,
        "runtime_wired": False,
        "production_authority": False,
        "operator_acceptance": False,
        "promotion": False,
        "release_authority": False,
        "callers_ratchet": False,
    }


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Verify the signed A0-first Hepta Intelligence admission composition."
    )
    parser.add_argument("--repository", required=True)
    parser.add_argument("--expected-head", required=True)
    parser.add_argument("--expected-base", required=True)
    parser.add_argument("--expected-base-branch", required=True)
    parser.add_argument("--expected-head-branch", required=True)
    parser.add_argument("--token-env", default="GITHUB_TOKEN")
    parser.add_argument("--output", required=True)
    parser.add_argument("--root", default=".")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        token = os.environ.get(args.token_env, "")
        require(token != "", f"GitHub token environment variable is empty: {args.token_env}")
        receipt = verify(
            root=Path(args.root).resolve(),
            repository=args.repository,
            expected_head=args.expected_head,
            expected_base=args.expected_base,
            expected_base_branch=args.expected_base_branch,
            expected_head_branch=args.expected_head_branch,
            token=token,
        )
        output = Path(args.output)
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(
            json.dumps(receipt, indent=2, sort_keys=True) + "\n",
            encoding="utf-8",
        )
        print(receipt["status"])
        return 0
    except AdmissionError as error:
        print(
            f"FAIL_HEPTA_INTELLIGENCE_INTEGRATION_ADMISSION_IDENTITY: {error}",
            file=sys.stderr,
        )
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
