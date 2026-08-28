#!/usr/bin/env python3
"""Fail-closed verifier for the Hepta Browser C1 private worker protocol slice."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CRATE = ROOT / "tools" / "hepta-browser-c1-protocol"
DOCS = ROOT / "docs" / "hepta-vnext" / "browser"
WORKFLOW = ROOT / ".github" / "workflows" / "hepta-browser-c1-protocol.yml"

SERVO_COMMIT = "0a48e298482659817eb50097df23841f2b8e3044"
SERVO_TREE = "b04d2f75b3217374d079d579c270177b57fa1389"

REQUIRED_FILES = [
    CRATE / "Cargo.toml",
    CRATE / "Cargo.lock",
    CRATE / "README.md",
    CRATE / "src" / "lib.rs",
    CRATE / "src" / "protocol.rs",
    CRATE / "src" / "codec.rs",
    CRATE / "src" / "transport.rs",
    CRATE / "tests" / "protocol.rs",
    DOCS / "C1_PRIVATE_WORKER_PROTOCOL.md",
    DOCS / "C1_PROTOCOL_STATUS.json",
    DOCS / "hepta.browser.c1_protocol_qualification_receipt.v1.schema.json",
    WORKFLOW,
]

FALSE_CONSTANTS = [
    "PRODUCTION_CALLER",
    "PRODUCTION_WRITER",
    "EFFECT_AUTHORITY",
    "EXTERNAL_EFFECT",
    "OPERATOR_ACCEPTANCE",
    "PROMOTION",
    "G5_ALLOWED",
    "EXECUTE_ALLOWED",
    "EXTERNAL_NETWORK_ALLOWED",
    "CREDENTIAL_EXPORT_ALLOWED",
]

FORBIDDEN_RUST_TOKENS = [
    "TcpListener",
    "TcpStream",
    "UdpSocket",
    "0.0.0.0",
    "127.0.0.1",
    "reqwest::",
    "hyper::",
    "axum::",
    "tokio_tungstenite",
    "webdriver_server",
    "WebDriverHttpServer",
    "Authorization",
    "Set-Cookie",
    "unsafe {",
]


def fail(message: str) -> None:
    raise ValueError(message)


def load_json(path: Path) -> object:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        fail(f"cannot decode {path.relative_to(ROOT)}: {error}")


def require_files() -> None:
    missing = [str(path.relative_to(ROOT)) for path in REQUIRED_FILES if not path.is_file()]
    if missing:
        fail(f"required C1 protocol files are missing: {missing}")


def verify_zero_dependency_crate() -> None:
    cargo = (CRATE / "Cargo.toml").read_text(encoding="utf-8")
    if "[dependencies]" in cargo or "[dev-dependencies]" in cargo or "[build-dependencies]" in cargo:
        fail("C1 protocol crate must remain dependency-free")
    if 'name = "hepta-browser-worker-protocol-qualification"' not in cargo:
        fail("C1 protocol crate name changed")
    if 'edition = "2024"' not in cargo:
        fail("C1 protocol crate must use the pinned workspace-era Rust edition")

    lock = (CRATE / "Cargo.lock").read_text(encoding="utf-8")
    if lock.count("[[package]]") != 1:
        fail("C1 protocol Cargo.lock must contain exactly the local package")
    if "source =" in lock or "checksum =" in lock:
        fail("C1 protocol Cargo.lock unexpectedly contains a third-party package")


def verify_rust_boundary() -> None:
    sources = sorted((CRATE / "src").rglob("*.rs"))
    combined = "\n".join(path.read_text(encoding="utf-8") for path in sources)
    for token in FORBIDDEN_RUST_TOKENS:
        if token in combined:
            fail(f"forbidden C1 protocol implementation token present: {token}")

    if "pub const QUALIFICATION_ONLY: bool = true;" not in combined:
        fail("qualification-only posture constant is missing")
    for name in FALSE_CONSTANTS:
        expected = f"pub const {name}: bool = false;"
        if expected not in combined:
            fail(f"negative-authority constant changed or is missing: {name}")

    required_markers = [
        "MAX_FRAME_BYTES: usize = 65_536",
        'b"HEPTABR1"',
        "constant_time_eq",
        "host_handshake",
        "worker_handshake",
        "StartupCapability(<redacted>)",
        "NavigateLocal",
        "StaleFence",
    ]
    for marker in required_markers:
        if marker not in combined:
            fail(f"required C1 protocol marker is missing: {marker}")

    if re.search(r"\bextern\s+crate\b", combined):
        fail("C1 protocol crate must not import an external crate")


def verify_status_and_receipt() -> None:
    status = load_json(DOCS / "C1_PROTOCOL_STATUS.json")
    if not isinstance(status, dict):
        fail("C1 protocol status must be an object")
    if status.get("status") != "IMPLEMENTED_QUALIFICATION_ONLY_EVIDENCE_PENDING":
        fail("C1 protocol status is not the expected fail-closed candidate state")
    servo = status.get("servo_source_pin")
    if not isinstance(servo, dict):
        fail("C1 status lacks a Servo source pin")
    if servo.get("commit") != SERVO_COMMIT or servo.get("tree") != SERVO_TREE:
        fail("C1 status Servo source pin drifted")
    if servo.get("state") != "SOURCE_PIN_ONLY_NOT_IMPORTED_NOT_BUILT":
        fail("C1 status overclaims Servo import/build")
    implementation = status.get("implementation")
    if not isinstance(implementation, dict):
        fail("C1 status lacks implementation posture")
    for key in ("servo_linked", "real_webview", "external_network"):
        if implementation.get(key) is not False:
            fail(f"C1 implementation posture must keep {key}=false")
    authority = status.get("authority")
    if not isinstance(authority, dict) or any(value is not False for value in authority.values()):
        fail("C1 status contains positive or non-boolean authority")
    if status.get("merge_authorized") is not False or status.get("release_qualified") is not False:
        fail("C1 status must not authorize merge or release")

    schema = load_json(
        DOCS / "hepta.browser.c1_protocol_qualification_receipt.v1.schema.json"
    )
    if not isinstance(schema, dict):
        fail("C1 receipt schema must be an object")
    properties = schema.get("properties")
    if not isinstance(properties, dict):
        fail("C1 receipt schema lacks properties")
    authority_schema = properties.get("authority")
    if not isinstance(authority_schema, dict):
        fail("C1 receipt schema lacks authority object")
    authority_properties = authority_schema.get("properties")
    if not isinstance(authority_properties, dict):
        fail("C1 receipt schema lacks authority properties")
    for key, definition in authority_properties.items():
        if not isinstance(definition, dict) or definition.get("const") is not False:
            fail(f"receipt authority field is not const false: {key}")


def verify_plan_and_workflow() -> None:
    plan = (DOCS / "C1_PRIVATE_WORKER_PROTOCOL.md").read_text(encoding="utf-8")
    for marker in (
        "IMPLEMENTED_QUALIFICATION_ONLY",
        "SERVO_NOT_LINKED",
        SERVO_COMMIT,
        SERVO_TREE,
        "socketpair",
        "65,536 bytes",
        "NavigateLocal",
        "C1-004A",
    ):
        if marker not in plan:
            fail(f"C1 plan marker is missing: {marker}")

    for forbidden_path in ("/Users/", "/Volumes/T5/", "/home/qian", "Dropbox/OpenClaw"):
        if forbidden_path in plan:
            fail(f"C1 canonical plan contains a machine-local path: {forbidden_path}")

    workflow = WORKFLOW.read_text(encoding="utf-8")
    for marker in (
        "verify-hepta-browser-c1-protocol.py",
        "cargo fmt",
        "--manifest-path tools/hepta-browser-c1-protocol/Cargo.toml",
        "-- --check",
        "cargo test --locked",
        "cargo clippy --locked",
        "qualification_only=true",
        "servo_linked=false",
        "external_network=false",
        "production_caller=false",
        "promotion=false",
    ):
        if marker not in workflow:
            fail(f"C1 qualification workflow marker is missing: {marker}")
    if "cargo fmt --locked" in workflow:
        fail("C1 workflow uses unsupported cargo fmt --locked syntax")


def main() -> int:
    try:
        require_files()
        verify_zero_dependency_crate()
        verify_rust_boundary()
        verify_status_and_receipt()
        verify_plan_and_workflow()
    except ValueError as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1

    print(
        json.dumps(
            {
                "status": "QUALIFICATION_INPUT_VERIFIED",
                "slice": "WEB-C1.3-private-worker-protocol",
                "servo_commit": SERVO_COMMIT,
                "servo_tree": SERVO_TREE,
                "third_party_dependencies": 0,
                "runtime_authority": False,
                "external_network": False,
                "production_caller": False,
                "production_writer": False,
                "effect_authority": False,
                "operator_acceptance": False,
                "promotion": False,
            },
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
