#!/usr/bin/env python3
"""Fail-closed A0 authority/registry/integration candidate verifier."""

from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
from typing import Any, NoReturn

ROOT = Path(__file__).resolve().parents[1]
PLAN_DIR = ROOT / "plans" / "hepta-intelligence"
CURRENT = PLAN_DIR / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
MASTER = PLAN_DIR / "HEPTA_INTELLIGENCE_MASTER_PLAN.md"
DOCUMENT = PLAN_DIR / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json"
EVIDENCE = PLAN_DIR / "HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json"
CAPABILITIES = PLAN_DIR / "HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json"
PR_STACK = PLAN_DIR / "HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json"
INTEGRATION = PLAN_DIR / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
Q0 = PLAN_DIR / "HEPTA_INTELLIGENCE_Q0_QUALIFICATION_RECEIPT_2026-08-29.json"
TRUTH = ROOT / "scripts" / "hepta-intelligence-current-truth.py"
EXPECTED_REPOSITORY = "ProfHepta/hepta-private-ci"
EXPECTED_PARENT = "c768bcbeb4c1168088d2499828c24da521a2a73a"
EXPECTED_Q0_TREE = "ca455a9ef797cd95164c880c7b8faba80b305589"
EXPECTED_BRANCH = "codex/hepta-intelligence-a0-authority-gap-closure-20260829"
HEX40 = re.compile(r"[0-9a-f]{40}")


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_INTELLIGENCE_A0_AUTHORITY: {message}")


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def load(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except Exception as exc:
        fail(f"cannot parse {path.relative_to(ROOT)}: {exc}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain an object")
    return value


def canonical(value: Any) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def all_false(mapping: Any) -> bool:
    return isinstance(mapping, dict) and bool(mapping) and all(value is False for value in mapping.values())


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


def main() -> int:
    paths = [CURRENT, MASTER, DOCUMENT, EVIDENCE, CAPABILITIES, PR_STACK, INTEGRATION, Q0, TRUTH]
    for path in paths:
        require(path.is_file(), f"missing {path.relative_to(ROOT)}")

    current = load(CURRENT)
    document = load(DOCUMENT)
    evidence = load(EVIDENCE)
    capabilities = load(CAPABILITIES)
    pr_stack = load(PR_STACK)
    integration = load(INTEGRATION)
    q0 = load(Q0)

    require(current.get("repository") == EXPECTED_REPOSITORY, "current repository mismatch")
    require(current.get("active_phase", {}).get("id") == "A0", "A0 is not active")
    require(current.get("current_truth", {}).get("q0_executable_qualified") is True, "Q0 qualification missing")
    require(current.get("current_truth", {}).get("qualified_candidate") is True, "qualified candidate missing")
    require(current.get("current_truth", {}).get("wired") is False, "runtime wiring unexpectedly enabled")
    require(current.get("current_truth", {}).get("qualified") is False, "product capability prematurely qualified")
    require(current.get("current_truth", {}).get("full_repository_merge_green") is False, "merge-green was fabricated")
    require(all_false(current.get("authority")), "current authority must remain false")

    q0_observation = q0.get("evidence_observation", {})
    require(q0_observation.get("head") == EXPECTED_PARENT, "Q0 head mismatch")
    require(q0_observation.get("tree") == EXPECTED_Q0_TREE, "Q0 tree mismatch")
    require(q0_observation.get("run_id") == 33252922404, "Q0 run mismatch")
    jobs = q0_observation.get("jobs")
    require(isinstance(jobs, list) and len(jobs) == 3, "Q0 job surface incomplete")
    require({job.get("job_id") for job in jobs} == {99101597686, 99101597800, 99105393694}, "Q0 jobs drift")
    require(all(job.get("runner_id", 0) > 0 and job.get("steps_non_empty") is True for job in jobs), "Q0 has non-executable job")
    artifacts = q0_observation.get("artifacts")
    require(isinstance(artifacts, list) and len(artifacts) == 3, "Q0 artifact surface incomplete")
    require({item.get("artifact_id") for item in artifacts} == {9715334789, 9715221566, 9715623771}, "Q0 artifacts drift")
    require(all(re.fullmatch(r"sha256:[0-9a-f]{64}", str(item.get("digest", ""))) for item in artifacts), "Q0 artifact digest invalid")
    require(q0.get("conclusion", {}).get("q0_executable_qualified") is True, "Q0 pair conclusion missing")
    require(q0.get("conclusion", {}).get("runtime_capability_qualified") is False, "Q0 crossed runtime boundary")
    require(all_false(q0.get("authority")), "Q0 authority must remain false")
    binding = dict(q0)
    observed_binding = binding.pop("receipt_binding_sha256", None)
    require(observed_binding == hashlib.sha256(canonical(binding)).hexdigest(), "Q0 summary binding mismatch")

    require(document.get("repository") == EXPECTED_REPOSITORY, "document registry repository mismatch")
    registered_inputs = {
        item.get("path"): item.get("schema")
        for item in document.get("registered_canonical_inputs", [])
        if isinstance(item, dict)
    }
    require(len(registered_inputs) == 4, "document registry canonical inputs incomplete")
    require(all_false(document.get("authority")), "document registry authority must remain false")

    entries = capabilities.get("capabilities")
    require(isinstance(entries, list) and entries, "capability registry empty")
    by_id = {entry.get("capability_id"): entry for entry in entries if isinstance(entry, dict)}
    require(len(by_id) == len(entries), "duplicate or malformed capability")
    p0 = by_id.get("P0_GROUNDED_MUTATION_FOUNDATION", {})
    require(p0.get("candidate_qualified") is True, "P0 source candidate qualification not recorded")
    require(p0.get("wired") is False and p0.get("runtime_qualified") is False, "P0 gained runtime capability")
    a0 = by_id.get("A0_CANONICAL_CAPABILITY_AND_EVIDENCE_AUTHORITY", {})
    require(a0.get("implemented") is True, "A0 implementation not recorded")
    require(a0.get("candidate_qualified") is False, "A0 self-qualified before evidence")
    for capability_id, entry in by_id.items():
        require(all_false(entry.get("authority")), f"{capability_id} authority must remain false")
    require(all_false(capabilities.get("authority")), "capability registry authority must remain false")

    canonical_stack = pr_stack.get("canonical_stack", {})
    canonical_entries = canonical_stack.get("entries")
    require(isinstance(canonical_entries, list), "canonical stack entries missing")
    require([item.get("pr_number") for item in canonical_entries] == [7, 13, 14, 16, 21, 23, 29], "canonical stack order drift")
    require(canonical_stack.get("candidate_head") == EXPECTED_PARENT, "canonical candidate head drift")
    require(canonical_stack.get("candidate_tree") == EXPECTED_Q0_TREE, "canonical candidate tree drift")
    require(all(item.get("merged") is False and item.get("draft") is True for item in canonical_entries), "canonical PR state drift")
    require(all(stack.get("canonical") is False for stack in pr_stack.get("side_stacks", [])), "side stack gained canonical status")
    for item in pr_stack.get("external_unmerged_dependencies", []):
        require(item.get("canonical") is False and item.get("authority") is False, "external PR gained authority")
    require(all_false(pr_stack.get("authority")), "PR stack authority must remain false")

    require(integration.get("repository") == EXPECTED_REPOSITORY, "integration repository mismatch")
    require(integration.get("branch") == EXPECTED_BRANCH, "integration branch mismatch")
    require(integration.get("expected_parent") == EXPECTED_PARENT, "integration expected parent mismatch")
    allowed = integration.get("allowed_changed_paths")
    require(isinstance(allowed, list) and allowed == sorted(set(allowed)), "integration allowlist invalid")
    forbidden_prefixes = (
        "codex-rs/",
        "migrations/",
        "sdk/",
        "shell-tool-mcp/",
    )
    require(not any(path.startswith(forbidden_prefixes) for path in allowed), "runtime/product path in A0 allowlist")
    freeze = integration.get("source_freeze", {})
    require(freeze.get("rust_runtime_changes_allowed") is False, "runtime source freeze disabled")
    require(freeze.get("sql_migrations_allowed") is False, "migration freeze disabled")
    require(freeze.get("product_callers_allowed") is False, "caller freeze disabled")
    require(all_false(integration.get("authority")), "integration authority must remain false")

    require(evidence.get("q0_current_evidence", {}).get("q0_executable_qualified") is True, "evidence index lost Q0 qualification")
    require(evidence.get("q0_current_evidence", {}).get("full_repository_merge_green") is False, "evidence index fabricated merge-green")
    require(all_false(evidence.get("authority")), "evidence authority must remain false")

    first = subprocess.check_output([sys.executable, str(TRUTH), "--compact"], cwd=ROOT)
    second = subprocess.check_output([sys.executable, str(TRUTH), "--compact"], cwd=ROOT)
    require(first == second, "current-truth output is not byte deterministic")
    truth_value = json.loads(first)
    require(truth_value.get("schema") == "hepta_intelligence_current_truth_v1", "truth schema mismatch")
    require(truth_value.get("q0", {}).get("qualified_candidate") is True, "truth lost Q0 candidate qualification")
    require(truth_value.get("q0", {}).get("runtime_capability_qualified") is False, "truth crossed runtime boundary")
    require(all_false(truth_value.get("authority")), "truth authority must remain false")

    changed_files: list[str] = []
    head = None
    tree = None
    git_dir = ROOT / ".git"
    if git_dir.exists():
        head = git("rev-parse", "HEAD")
        tree = git("rev-parse", "HEAD^{tree}")
        parent = git("rev-parse", "HEAD^")
        require(HEX40.fullmatch(head) is not None, "HEAD is invalid")
        require(HEX40.fullmatch(tree) is not None, "tree is invalid")
        require(parent == EXPECTED_PARENT, f"exact parent mismatch: {parent}")
        changed_files = sorted(line for line in git("diff", "--name-only", "HEAD^", "HEAD").splitlines() if line)
        require(changed_files == allowed, f"changed-path surface mismatch: {changed_files}")
        subprocess.check_call(["git", "diff", "--check", "HEAD^", "HEAD"], cwd=ROOT)
        env_sha = os.environ.get("GITHUB_SHA")
        if env_sha:
            require(env_sha == head, "GITHUB_SHA does not match checkout")
        env_repo = os.environ.get("GITHUB_REPOSITORY")
        if env_repo:
            require(env_repo == EXPECTED_REPOSITORY, "GITHUB_REPOSITORY mismatch")
        env_ref = os.environ.get("GITHUB_HEAD_REF") or os.environ.get("GITHUB_REF_NAME")
        if env_ref and not env_ref.startswith("refs/"):
            require(env_ref == EXPECTED_BRANCH, f"workflow branch mismatch: {env_ref}")

    receipt = {
        "schema": "hepta_intelligence_a0_source_gate_receipt_v1",
        "status": "PASS_HEPTA_INTELLIGENCE_A0_SOURCE_ONLY",
        "repository": EXPECTED_REPOSITORY,
        "candidate": {
            "head": head,
            "tree": tree,
            "parent": EXPECTED_PARENT,
            "branch": EXPECTED_BRANCH,
        },
        "q0_base": {
            "head": EXPECTED_PARENT,
            "tree": EXPECTED_Q0_TREE,
            "run_id": 33252922404,
            "qualified_candidate": True,
        },
        "changed_files": changed_files or allowed,
        "changed_files_sha256": hashlib.sha256(("\n".join(changed_files or allowed) + "\n").encode()).hexdigest(),
        "current_truth_sha256": hashlib.sha256(first).hexdigest(),
        "runtime_source_changed": False,
        "sql_migration_changed": False,
        "product_caller_changed": False,
        "source_writeback": False,
        "a0_candidate_qualified": False,
        "full_repository_merge_green": False,
        "authority": current.get("authority"),
    }
    print(json.dumps(receipt, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
