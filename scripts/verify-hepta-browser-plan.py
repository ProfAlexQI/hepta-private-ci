#!/usr/bin/env python3
"""Canonical entrypoint for the repository-native Hepta browser verifier."""

from __future__ import annotations

from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
VERIFIER_CORE = Path(__file__).with_name("verify-hepta-browser-plan-v2.py")


def load_verifier_core() -> dict[str, Any]:
    source = VERIFIER_CORE.read_text(encoding="utf-8")
    namespace: dict[str, Any] = {
        "__file__": str(VERIFIER_CORE),
        "__name__": "hepta_browser_verifier_core",
        "__package__": None,
    }
    exec(compile(source, str(VERIFIER_CORE), "exec"), namespace)
    return namespace


def install_ci_verifier(namespace: dict[str, Any]) -> None:
    fail = namespace.get("fail")
    if not callable(fail):
        raise RuntimeError("browser verifier core has no fail function")

    def verify_ci_and_ownership() -> None:
        hepta_path = ROOT / ".github/workflows/hepta-vnext-qualification.yml"
        browser_path = ROOT / ".github/workflows/hepta-browser-ci.yml"
        blocking_path = ROOT / ".github/workflows/blocking-ci.yml"
        owners_path = ROOT / ".github/CODEOWNERS"
        for path in (hepta_path, browser_path, blocking_path, owners_path):
            if not path.is_file():
                fail(f"missing CI or ownership file: {path.relative_to(ROOT)}")

        hepta = hepta_path.read_text(encoding="utf-8")
        browser = browser_path.read_text(encoding="utf-8")
        blocking = blocking_path.read_text(encoding="utf-8")
        owners = owners_path.read_text(encoding="utf-8")

        hepta_tokens = (
            "workflow_call:",
            "runner-preflight:",
            "browser-c0-c3:",
            "uses: ./.github/workflows/hepta-browser-ci.yml",
            "integration/vnext-main-20260811",
        )
        for token in hepta_tokens:
            if token not in hepta:
                fail(f"Hepta qualification workflow is missing {token}")

        browser_tokens = (
            "workflow_call:",
            "runner-preflight:",
            "browser-c0-c3-c1-protocol:",
            "scripts/verify-hepta-browser-plan.py",
            "scripts/test_generate_hepta_servo_provenance.py",
            "--test browser_worker_process",
            "--all-targets -- -D warnings",
            "servo_runtime_qualified=false",
            "external_network=false",
        )
        for token in browser_tokens:
            if token not in browser:
                fail(f"reusable Browser workflow is missing {token}")

        blocking_tokens = (
            "pull_request:",
            "integration/vnext-main-20260811",
            "runner-preflight:",
            "hepta-vnext:",
            "uses: ./.github/workflows/hepta-vnext-qualification.yml",
            "- hepta-vnext",
        )
        for token in blocking_tokens:
            if token not in blocking:
                fail(f"blocking CI is missing {token}")

        owner_patterns = (
            "/codex-rs/hepta-* @ProfAlexQI",
            "/docs/hepta-vnext/ @ProfAlexQI",
            "/third_party/servo-patches/ @ProfAlexQI",
            "/scripts/generate-hepta-servo-provenance.py @ProfAlexQI",
            "/.github/workflows/hepta-browser-ci.yml @ProfAlexQI",
            "/.github/workflows/hepta-vnext-qualification.yml @ProfAlexQI",
            "/.github/workflows/blocking-ci.yml @ProfAlexQI",
        )
        for pattern in owner_patterns:
            if pattern not in owners:
                fail(f"CODEOWNERS is missing {pattern}")

    namespace["verify_ci_and_ownership"] = verify_ci_and_ownership


def main() -> int:
    namespace = load_verifier_core()
    install_ci_verifier(namespace)
    verifier = namespace.get("main")
    if not callable(verifier):
        raise RuntimeError("canonical Hepta browser verifier has no callable main")
    result = verifier()
    if not isinstance(result, int):
        raise RuntimeError("canonical Hepta browser verifier returned a non-integer result")
    return result


if __name__ == "__main__":
    raise SystemExit(main())
