#!/usr/bin/env python3
"""Verify the qualification-only C1 artifact-bound process launch gate."""
from __future__ import annotations

import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
CRATE = ROOT / "tools/hepta-browser-c1-artifact-gate"
FILES = (
    CRATE / "Cargo.toml",
    CRATE / "Cargo.lock",
    CRATE / "src/lib.rs",
    CRATE / "src/bin/hepta-browser-c1-artifact-bound-trial.rs",
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
                fail(f"missing artifact gate file: {path.relative_to(ROOT)}")
        cargo = (CRATE / "Cargo.toml").read_text(encoding="utf-8")
        if "[dependencies]" in cargo or "path = \"../hepta-browser-c1-protocol\"" in cargo:
            fail("artifact gate must remain zero-dependency and separate from the browser API")
        if 'unsafe_code = "forbid"' not in cargo:
            fail("artifact gate must forbid unsafe code")

        sources = "\n".join(
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
        ):
            if forbidden in sources:
                fail(f"artifact gate contains forbidden surface: {forbidden}")
        required = (
            "binding_for_current_executable",
            "hash_file",
            "ArtifactBinding",
            "build_manifest_sha256",
            "source_receipt_sha256",
            "UnixStream::pair()",
            "set_read_timeout",
            "set_write_timeout",
            "--force-kill-trial",
            "wait_bounded",
            "child.kill()",
            "ARTIFACT_BOUND_QUALIFICATION_TRIAL_PASS",
            "ARTIFACT_BOUND_FORCE_KILL_REAP_PASS",
            "runtime_authority",
            "servo_linked",
        )
        for token in required:
            if token not in sources:
                fail(f"artifact gate is missing {token}")

        for fixture in FILES[-2:]:
            value = json.loads(fixture.read_text(encoding="utf-8"))
            if not isinstance(value, dict):
                fail(f"fixture must be an object: {fixture.relative_to(ROOT)}")
            serialized = json.dumps(value, sort_keys=True, separators=(",", ":"))
            if fixture.read_text(encoding="utf-8") != serialized:
                fail(f"fixture is not compact canonical JSON: {fixture.relative_to(ROOT)}")
            for key in ("authority", "runtime_qualified", "worker_artifact_built"):
                if key in value and value[key] is not False:
                    fail(f"fixture attempted to enable {key}")
    except (VerificationError, OSError, UnicodeError, json.JSONDecodeError) as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1

    print(
        json.dumps(
            {
                "schema": "hepta.browser.c1_artifact_gate_contract_verification.v1",
                "status": "PASS_QUALIFICATION_CONTRACT_ONLY",
                "real_servo_artifact": False,
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
