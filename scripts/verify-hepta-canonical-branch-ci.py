#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_BRANCH = "integration/vnext-main-20260811"
FULL_CI_PATTERN = '"integration/vnext-main-full-ci-*"'
POLICY_PATH = (
    "plans/hepta-convergence/HEPTA_CANONICAL_BRANCH_POLICY_V1.json"
)
EXPECTED_PARENT = {
    "pull_request": 155,
    "commit": "58f7df731a8c0febd8118be9a34cd69663089253",
    "tree": "528884f1d8d5bc8b32326873107adc3b49334ce0",
}
EXPECTED_AUTHORITY = {
    "a0_candidate_qualified": False,
    "selected": False,
    "full_repository_merge_green": False,
    "runtime": False,
    "production": False,
    "operator_acceptance": False,
    "promotion": False,
    "release": False,
    "callers_ratchet": False,
}
EXPECTED_TRIGGER_COVERAGE = {
    "blocking_ci_pull_request": True,
    "blocking_ci_main_push": True,
    "blocking_ci_default_branch_push": True,
    "hepta_vnext_manual_dispatch": True,
    "hepta_vnext_main_push": True,
    "hepta_vnext_default_branch_push": True,
    "hepta_vnext_legacy_branch_push": True,
    "hepta_vnext_full_ci_integration_pattern": (
        "integration/vnext-main-full-ci-*"
    ),
}


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def read(relative: str) -> str:
    path = ROOT / relative
    require(path.is_file(), f"missing canonical-branch contract path: {relative}")
    return path.read_text(encoding="utf-8")


def reject_duplicate_keys(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def load_policy() -> dict[str, Any]:
    try:
        value = json.loads(
            read(POLICY_PATH),
            object_pairs_hook=reject_duplicate_keys,
        )
    except (json.JSONDecodeError, ValueError) as error:
        fail(f"invalid canonical-branch policy JSON: {error}")
    require(isinstance(value, dict), "canonical-branch policy must be an object")
    return value


def require_exact_keys(
    value: dict[str, Any], expected: set[str], owner: str
) -> None:
    observed = set(value)
    require(
        observed == expected,
        f"{owner} keys drifted: expected {sorted(expected)!r}, "
        f"observed {sorted(observed)!r}",
    )


def trigger_prefix(text: str, owner: str) -> str:
    marker = "\njobs:\n"
    require(marker in text, f"{owner} lacks top-level jobs boundary")
    return text.split(marker, 1)[0]


def main() -> None:
    blocking = read(".github/workflows/blocking-ci.yml")
    hepta = read(".github/workflows/hepta-vnext-qualification.yml")
    repo_checks = read(".github/workflows/repo-checks.yml")
    policy = load_policy()

    require_exact_keys(
        policy,
        {
            "schema",
            "repository",
            "default_branch",
            "required_merge_context",
            "source_parent",
            "source_trigger_coverage",
            "ruleset",
            "authority",
        },
        "canonical-branch policy",
    )
    require(
        policy["schema"] == "hepta_canonical_branch_policy_v1",
        "policy schema drifted",
    )
    require(
        policy["repository"] == "ProfHepta/hepta-private-ci",
        "policy repository drifted",
    )
    require(
        policy["default_branch"] == DEFAULT_BRANCH,
        "policy default branch drifted",
    )
    require(
        policy["required_merge_context"] == "CI required",
        "required context drifted",
    )

    parent = policy["source_parent"]
    coverage = policy["source_trigger_coverage"]
    ruleset = policy["ruleset"]
    authority = policy["authority"]
    for value, owner in (
        (parent, "source_parent"),
        (coverage, "source_trigger_coverage"),
        (ruleset, "ruleset"),
        (authority, "authority"),
    ):
        require(isinstance(value, dict), f"{owner} must be an object")

    require_exact_keys(parent, set(EXPECTED_PARENT), "source_parent")
    require(parent == EXPECTED_PARENT, "source_parent exact binding drifted")
    require_exact_keys(
        coverage,
        set(EXPECTED_TRIGGER_COVERAGE),
        "source_trigger_coverage",
    )
    require(
        coverage == EXPECTED_TRIGGER_COVERAGE,
        "source trigger coverage drifted",
    )
    require_exact_keys(
        ruleset,
        {"name", "expected_enforcement", "applied", "readback_receipt"},
        "ruleset",
    )
    require(
        ruleset
        == {
            "name": "hepta-canonical-branch-protection-v1",
            "expected_enforcement": "active",
            "applied": False,
            "readback_receipt": None,
        },
        "source must not self-claim live ruleset enforcement",
    )
    require_exact_keys(authority, set(EXPECTED_AUTHORITY), "authority")
    require(authority == EXPECTED_AUTHORITY, "authority escaped in branch policy")

    blocking_trigger = trigger_prefix(blocking, "blocking-ci")
    require(
        blocking_trigger.count("  pull_request: {}") == 1,
        "blocking-ci must bind pull_request exactly once",
    )
    require(
        blocking_trigger.count("  push:\n") == 1,
        "blocking-ci must bind push exactly once",
    )
    require(
        blocking_trigger.count(DEFAULT_BRANCH) == 1,
        "blocking-ci must bind the default branch exactly once",
    )
    require(
        blocking_trigger.count("      - main\n") == 1,
        "blocking-ci must retain main push coverage exactly once",
    )
    require(
        "paths:" not in blocking_trigger and "paths-ignore:" not in blocking_trigger,
        "blocking-ci trigger must not filter paths",
    )
    require(
        blocking.count("name: CI required") == 1,
        "blocking-ci lost stable required context",
    )
    require(
        blocking.count(
            "uses: ./.github/workflows/windows-gnullvm-qualification-boundary.yml"
        )
        == 1,
        "blocking-ci lost Windows gnullvm boundary",
    )
    require(
        blocking.count("cancel-in-progress: true") == 1,
        "blocking-ci must cancel obsolete candidate runs",
    )

    hepta_trigger = trigger_prefix(hepta, "hepta-vnext qualification")
    require(
        hepta_trigger.count("  workflow_dispatch:\n") == 1,
        "hepta qualification lost manual dispatch",
    )
    require(
        hepta_trigger.count("  push:\n") == 1,
        "hepta qualification lost push coverage",
    )
    require(
        hepta_trigger.count(DEFAULT_BRANCH) == 1,
        "hepta qualification must bind the default branch exactly once",
    )
    require(
        hepta_trigger.count("      - main\n") == 1,
        "hepta qualification must cover main exactly once",
    )
    require(
        hepta_trigger.count("      - vnext-main\n") == 1,
        "hepta qualification lost legacy vnext-main coverage",
    )
    require(
        hepta_trigger.count(FULL_CI_PATTERN) == 1,
        "hepta qualification lost full-CI integration pattern",
    )
    require(
        "  pull_request:" not in hepta_trigger,
        "specialized Hepta qualification must not duplicate every PR",
    )
    require(
        "paths:" not in hepta_trigger and "paths-ignore:" not in hepta_trigger,
        "hepta qualification trigger must not filter paths",
    )
    require(
        hepta.count("permissions:\n  contents: read") == 1,
        "hepta qualification must remain explicitly read-only",
    )

    command = "python3 scripts/verify-hepta-canonical-branch-ci.py"
    require(
        repo_checks.count(command) == 1,
        "repo-checks must execute branch verifier exactly once",
    )
    require(
        repo_checks.index("Test Bazel CI wrapper policy")
        < repo_checks.index(command),
        "canonical branch verifier must follow Bazel policy tests",
    )

    print("PASS_HEPTA_CANONICAL_BRANCH_CI_SOURCE")


if __name__ == "__main__":
    main()
