#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_BRANCH = "integration/vnext-main-20260811"
FULL_CI_PATTERN = '"integration/vnext-main-full-ci-*"'


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def read(relative: str) -> str:
    path = ROOT / relative
    require(path.is_file(), f"missing canonical-branch contract path: {relative}")
    return path.read_text(encoding="utf-8")


def trigger_prefix(text: str, owner: str) -> str:
    marker = "\njobs:\n"
    require(marker in text, f"{owner} lacks top-level jobs boundary")
    return text.split(marker, 1)[0]


def main() -> None:
    blocking = read(".github/workflows/blocking-ci.yml")
    hepta = read(".github/workflows/hepta-vnext-qualification.yml")
    repo_checks = read(".github/workflows/repo-checks.yml")
    policy_text = read(
        "plans/hepta-convergence/HEPTA_CANONICAL_BRANCH_POLICY_V1.json"
    )
    policy = json.loads(policy_text)

    require(policy.get("schema") == "hepta_canonical_branch_policy_v1", "policy schema drifted")
    require(policy.get("repository") == "ProfHepta/hepta-private-ci", "policy repository drifted")
    require(policy.get("default_branch") == DEFAULT_BRANCH, "policy default branch drifted")
    require(policy.get("required_merge_context") == "CI required", "required context drifted")
    require(policy.get("ruleset", {}).get("applied") is False, "source must not self-claim ruleset application")
    require(all(value is False for value in policy.get("authority", {}).values()), "authority escaped in branch policy")

    blocking_trigger = trigger_prefix(blocking, "blocking-ci")
    require("  pull_request: {}" in blocking_trigger, "blocking-ci lost pull_request coverage")
    require("  push:\n" in blocking_trigger, "blocking-ci lost push coverage")
    require(blocking_trigger.count(DEFAULT_BRANCH) == 1, "blocking-ci must bind the default branch exactly once")
    require("      - main\n" in blocking_trigger, "blocking-ci must retain main push coverage")
    require("paths:" not in blocking_trigger and "paths-ignore:" not in blocking_trigger, "blocking-ci trigger must not filter paths")
    require("name: CI required" in blocking, "blocking-ci lost stable required context")
    require("uses: ./.github/workflows/windows-gnullvm-qualification-boundary.yml" in blocking, "blocking-ci lost Windows gnullvm boundary")
    require("cancel-in-progress: true" in blocking, "blocking-ci must cancel obsolete candidate runs")

    hepta_trigger = trigger_prefix(hepta, "hepta-vnext qualification")
    require("  workflow_dispatch:\n" in hepta_trigger, "hepta qualification lost manual dispatch")
    require("  push:\n" in hepta_trigger, "hepta qualification lost push coverage")
    require(hepta_trigger.count(DEFAULT_BRANCH) == 1, "hepta qualification must bind the default branch exactly once")
    require("      - main\n" in hepta_trigger, "hepta qualification must cover main")
    require("      - vnext-main\n" in hepta_trigger, "hepta qualification lost vnext-main")
    require(FULL_CI_PATTERN in hepta_trigger, "hepta qualification lost full-CI integration pattern")
    require("  pull_request:" not in hepta_trigger, "specialized Hepta qualification must not duplicate every PR")
    require("paths:" not in hepta_trigger and "paths-ignore:" not in hepta_trigger, "hepta qualification trigger must not filter paths")

    command = "python3 scripts/verify-hepta-canonical-branch-ci.py"
    require(repo_checks.count(command) == 1, "repo-checks must execute branch verifier exactly once")
    require("Test Bazel CI wrapper policy" in repo_checks, "repo-checks lost Bazel policy tests")

    print("PASS_HEPTA_CANONICAL_BRANCH_CI_SOURCE")


if __name__ == "__main__":
    main()
