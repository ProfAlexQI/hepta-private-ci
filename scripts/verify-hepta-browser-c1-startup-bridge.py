#!/usr/bin/env python3
"""Verify the qualification-only C1 artifact-to-browser startup bridge."""
from __future__ import annotations

import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
CRATE = ROOT / "tools/hepta-browser-c1-startup-bridge"
FILES = (
    CRATE / "Cargo.toml",
    CRATE / "Cargo.lock",
    CRATE / "src/bin/hepta-browser-c1-startup-bridge-trial.rs",
    CRATE / "tests/process_trial.rs",
    CRATE / "fixtures/qualification-build-manifest.json",
    CRATE / "fixtures/qualification-source-receipt.json",
)


class VerificationError(RuntimeError):
    pass


def fail(message: str) -> None:
    raise VerificationError(message)


def main() -> int:
    try:
        for path in FILES:
            if not path.is_file():
                fail(f"missing startup bridge file: {path.relative_to(ROOT)}")

        cargo = (CRATE / "Cargo.toml").read_text(encoding="utf-8")
        required_dependencies = (
            'hepta-browser-c1-artifact-gate-qualification = { path = "../hepta-browser-c1-artifact-gate" }',
            'hepta-browser-worker-protocol-qualification = { path = "../hepta-browser-c1-protocol" }',
        )
        for dependency in required_dependencies:
            if dependency not in cargo:
                fail(f"startup bridge is missing path dependency {dependency}")
        for forbidden in ("git =", "registry =", "version = \"*\"", "crates.io"):
            if forbidden in cargo:
                fail(f"startup bridge Cargo contract contains forbidden dependency surface: {forbidden}")
        if 'unsafe_code = "forbid"' not in cargo:
            fail("startup bridge must forbid unsafe code")

        rust_sources = "\n".join(
            path.read_text(encoding="utf-8")
            for path in FILES
            if path.suffix == ".rs"
        )
        for forbidden in (
            "unsafe {",
            ".unwrap(",
            ".expect(",
            "TcpListener",
            "TcpStream",
            "UdpSocket",
            "WebDriver",
            "DevTools",
            "CDP",
            "servo::",
            "http://",
            "https://",
            "127.0.0.1",
            "0.0.0.0",
            "CommandKind::NavigateLocal",
            "CommandKind::Click",
            "CommandKind::TypeText",
        ):
            if forbidden in rust_sources:
                fail(f"startup bridge contains forbidden surface: {forbidden}")

        required = (
            "binding_for_current_executable",
            "validate_worker_hello",
            "host_handshake",
            "worker_handshake",
            "BrowserSessionId::new",
            "SourcePin::new",
            "StartupCapability::new",
            "UnixStream::pair()",
            "set_read_timeout",
            "set_write_timeout",
            "matches!(&command.command",
            "--force-kill-trial",
            "wait_bounded",
            "guard.kill()",
            "ARTIFACT_TO_BROWSER_HANDOFF_QUALIFICATION_PASS",
            "ARTIFACT_TO_BROWSER_HANDOFF_FORCE_KILL_REAP_PASS",
            "runtime_authority",
            "production_caller",
            "servo_linked",
        )
        for token in required:
            if token not in rust_sources:
                fail(f"startup bridge is missing {token}")

        for fixture in FILES[-2:]:
            raw = fixture.read_text(encoding="utf-8")
            value = json.loads(raw)
            if not isinstance(value, dict):
                fail(f"fixture must be an object: {fixture.relative_to(ROOT)}")
            canonical = json.dumps(value, sort_keys=True, separators=(",", ":"))
            if raw != canonical:
                fail(f"fixture is not compact canonical JSON: {fixture.relative_to(ROOT)}")
            for key in (
                "authority",
                "runtime_qualified",
                "worker_artifact_built",
                "canonical_servo_checkout_verified",
                "network_access_during_build",
            ):
                if key in value and value[key] is not False:
                    fail(f"fixture attempted to enable {key}")
    except (VerificationError, OSError, UnicodeError, json.JSONDecodeError) as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1

    print(
        json.dumps(
            {
                "schema": "hepta.browser.c1_startup_bridge_contract_verification.v1",
                "status": "PASS_QUALIFICATION_CONTRACT_ONLY",
                "artifact_to_browser_handoff": True,
                "real_servo_artifact": False,
                "real_servo_runtime": False,
                "external_network": False,
                "runtime_authority": False,
                "production_caller": False,
                "promotion": False,
            },
            sort_keys=True,
            separators=(",", ":"),
        )
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
