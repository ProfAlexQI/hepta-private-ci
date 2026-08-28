#!/usr/bin/env python3
"""Verify the version-controlled independent Hepta required-check contract."""
from __future__ import annotations

import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
POLICY = ROOT / "docs/hepta-vnext/browser/CI_REQUIRED_CONTEXTS_V1.json"
BLOCKING = ROOT / ".github/workflows/blocking-ci.yml"
BROWSER = ROOT / ".github/workflows/hepta-browser-next-required-v9.yml"
VNEXT = ROOT / ".github/workflows/hepta-vnext-qualification.yml"

EXPECTED = [
    {
        "check_name": "CI required",
        "purpose": "repository-wide Bazel, Rust, SDK, policy and artifact gates",
        "required": True,
        "workflow": ".github/workflows/blocking-ci.yml",
    },
    {
        "check_name": "Hepta Browser next required v9",
        "purpose": "canonical WEB-C1 source, review, topology, build-input and preflight graph",
        "required": True,
        "workflow": ".github/workflows/hepta-browser-next-required-v9.yml",
    },
    {
        "check_name": "Hepta vNext required",
        "purpose": "portable Hepta workspace, AuthBus, product callers, schemas and lock qualification",
        "required": True,
        "workflow": ".github/workflows/hepta-vnext-qualification.yml",
    },
]


def fail(message: str) -> None:
    raise RuntimeError(message)


def canonical(value: object) -> bytes:
    return json.dumps(
        value,
        sort_keys=True,
        separators=(",", ":"),
        ensure_ascii=False,
    ).encode("utf-8")


def require(text: str, label: str, *tokens: str) -> None:
    for token in tokens:
        if token not in text:
            fail(f"{label} is missing {token!r}")


def main() -> int:
    try:
        for path in (POLICY, BLOCKING, BROWSER, VNEXT):
            if not path.is_file():
                fail(f"missing {path.relative_to(ROOT)}")
        raw = POLICY.read_bytes()
        policy = json.loads(raw.decode("utf-8"))
        if raw != canonical(policy):
            fail("required-context policy is not compact canonical JSON")
        if policy.get("schema") != "hepta.ci.required_contexts.v1" or policy.get(
            "schema_version"
        ) != 1:
            fail("required-context policy schema/version drifted")
        if policy.get("phase") != "DEVELOPMENT":
            fail("required-context policy must remain DEVELOPMENT")
        if policy.get("contexts") != EXPECTED:
            fail("required-context set or ordering drifted")
        enforcement = policy.get("enforcement")
        if enforcement != {
            "branch_ruleset_configured": False,
            "reason": "repository rules remain externally administered; this file is the version-controlled required-context contract",
            "single_workflow_aggregation": False,
        }:
            fail("required-context enforcement posture drifted")
        authority = policy.get("authority")
        if not isinstance(authority, dict) or any(value is not False for value in authority.values()):
            fail("required-context policy attempted to enable authority")

        blocking = BLOCKING.read_text(encoding="utf-8")
        browser = BROWSER.read_text(encoding="utf-8")
        vnext = VNEXT.read_text(encoding="utf-8")
        require(
            blocking,
            "blocking CI",
            "pull_request:",
            "name: CI required",
            "verify-hepta-required-contexts.py",
        )
        if "uses: ./.github/workflows/hepta-browser-next-required-v9.yml" in blocking:
            fail("blocking CI must not nest the independent v9 required workflow")
        if "uses: ./.github/workflows/hepta-vnext-qualification.yml" in blocking:
            fail("blocking CI must not nest the independent Hepta vNext required workflow")
        require(
            browser,
            "Browser v9",
            "pull_request:",
            "name: Hepta Browser next required v9",
            '"build_authorized": False',
            '"servo_runtime_qualified": False',
        )
        require(
            vnext,
            "Hepta vNext",
            "pull_request:",
            "name: Hepta vNext required",
            "- portable-product",
            "- authbus-local-qualification",
            "- browser-c0-c3",
            "- generated-and-locks",
        )
    except (OSError, UnicodeError, json.JSONDecodeError, RuntimeError) as error:
        print(f"HEPTA_REQUIRED_CONTEXTS=FAIL: {error}", file=sys.stderr)
        return 1
    print(
        json.dumps(
            {
                "schema": "hepta.ci.required_contexts.verification.v1",
                "status": "PASS_VERSION_CONTROLLED_CONTEXT_CONTRACT",
                "required_contexts": [item["check_name"] for item in EXPECTED],
                "single_workflow_aggregation": False,
                "branch_ruleset_configured": False,
                "authority": "all_false",
            },
            sort_keys=True,
            separators=(",", ":"),
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
