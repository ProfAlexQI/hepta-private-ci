#!/usr/bin/env python3
"""Canonical entrypoint for the repository-native Hepta browser verifier."""

from __future__ import annotations

import copy
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


def install_current_verifier(namespace: dict[str, Any]) -> None:
    fail = namespace.get("fail")
    original = namespace.get("verify_current")
    require_repo_file = namespace.get("require_repo_file")
    if not callable(fail) or not callable(original) or not callable(require_repo_file):
        raise RuntimeError("browser verifier core has no current-pointer verifier")

    def verify_current(current: dict[str, Any]) -> list[Path]:
        implementation = current.get("implementation")
        if not isinstance(implementation, dict):
            fail("CURRENT.yaml implementation must be an object")
        expected = {
            "servo_provenance_generator": "implemented_unqualified_no_source_receipt",
            "unix_inherited_socketpair": "implemented_qualification_only_unqualified",
            "windows_sid_named_pipe": "not_implemented",
            "servo_runtime": "not_integrated",
            "production_caller": "not_integrated",
        }
        for key, value in expected.items():
            if implementation.get(key) != value:
                fail(f"CURRENT.yaml implementation status drift for {key}")

        compatibility = copy.deepcopy(current)
        compatibility["implementation"]["unix_inherited_socketpair"] = "not_implemented"
        paths = original(compatibility)
        extra_pointers = (
            "worker_protocol_spec",
            "servo_source_receipt_schema",
            "servo_provenance_spec",
            "provenance_generator",
        )
        for key in extra_pointers:
            paths.append(require_repo_file(current.get(key), f"CURRENT.yaml#{key}"))
        return paths

    namespace["verify_current"] = verify_current


def install_worker_verifier(namespace: dict[str, Any]) -> None:
    fail = namespace.get("fail")
    original = namespace.get("verify_worker_code")
    if not callable(fail) or not callable(original):
        raise RuntimeError("browser verifier core has no worker verifier")

    unix_module_path = (
        ROOT / "codex-rs/hepta-shadow-qualification/src/browser_worker_unix.rs"
    )
    unix_test_path = (
        ROOT / "codex-rs/hepta-shadow-qualification/tests/browser_worker_unix_socketpair.rs"
    )
    binary_path = (
        ROOT
        / "codex-rs/hepta-shadow-qualification/src/bin/hepta-browser-worker-qualification.rs"
    )

    def verify_worker_code() -> None:
        original()
        for path in (unix_module_path, unix_test_path, binary_path):
            if not path.is_file():
                fail(f"missing Unix worker file: {path.relative_to(ROOT)}")
        module = unix_module_path.read_text(encoding="utf-8")
        test = unix_test_path.read_text(encoding="utf-8")
        binary = binary_path.read_text(encoding="utf-8")
        required = (
            "#![cfg(unix)]",
            "StdUnixStream::pair()",
            "UnixInheritedSocketPair",
            "OwnedFd",
            "Stdio::from(child_input_fd)",
            "Stdio::from(child_output_fd)",
            "worker_pid == expected_pid",
            "kill_on_drop(true)",
            "run_unix_qualification_browser_worker",
        )
        for token in required:
            if token not in module:
                fail(f"Unix socketpair scaffold is missing {token}")
        if "BROWSER_WORKER_UNIX_MODE_ARGUMENT" not in binary:
            fail("worker binary does not dispatch the Unix socketpair mode")
        if "fn inherited_unix_socketpair_has_no_listener_and_preserves_worker_identity" not in test:
            fail("Unix socketpair process test is missing")
        for forbidden in (
            "TcpListener",
            "TcpStream",
            "0.0.0.0",
            "127.0.0.1",
            "WebSocket",
            "unsafe {",
            ".unwrap(",
            ".expect(",
        ):
            if forbidden in module or forbidden in test or forbidden in binary:
                fail(f"Unix worker scaffold contains forbidden surface: {forbidden}")

    def verify_test_names(test_names: set[str]) -> None:
        test_paths = (
            ROOT / "codex-rs/hepta-shadow-qualification/src/browser_tests.rs",
            ROOT / "codex-rs/hepta-shadow-qualification/src/browser_worker_tests.rs",
            ROOT / "codex-rs/hepta-shadow-qualification/tests/browser_worker_process.rs",
            unix_test_path,
        )
        sources = "\n".join(path.read_text(encoding="utf-8") for path in test_paths)
        missing = sorted(name for name in test_names if f"fn {name}" not in sources)
        if missing:
            fail(f"traceability references missing tests: {missing}")

    namespace["verify_worker_code"] = verify_worker_code
    namespace["verify_test_names"] = verify_test_names


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
            "--test browser_worker_unix_socketpair",
            "--all-targets -- -D warnings",
            "servo_runtime_qualified=false",
            "unix_socketpair_fixture_implemented=true",
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
    install_current_verifier(namespace)
    install_worker_verifier(namespace)
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
