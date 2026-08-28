#!/usr/bin/env python3
"""Fail-closed source gate for AuthBus P1.1 qualification."""

from __future__ import annotations

import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
CRATE = ROOT / "codex-rs" / "hepta-authbus-p1-qualification"
REQUIRED = [
    CRATE / "Cargo.toml",
    CRATE / "README.md",
    CRATE / "src" / "lib.rs",
    CRATE / "src" / "model.rs",
    CRATE / "src" / "verifier.rs",
    CRATE / "tests" / "p1_1.rs",
    ROOT / "docs" / "hepta-vnext" / "authbus" / "AUTHBUS_P1_1_DEVELOPMENT_PLAN_2026-08-28.md",
    ROOT / ".github" / "workflows" / "authbus-p1-1-qualification.yml",
]

errors: list[str] = []
for path in REQUIRED:
    if not path.is_file() or path.stat().st_size == 0:
        errors.append(f"missing or empty required file: {path.relative_to(ROOT)}")

manifest = (CRATE / "Cargo.toml").read_text(encoding="utf-8")
lib = (CRATE / "src" / "lib.rs").read_text(encoding="utf-8")
model = (CRATE / "src" / "model.rs").read_text(encoding="utf-8")
verifier = (CRATE / "src" / "verifier.rs").read_text(encoding="utf-8")
parent_manifest = (ROOT / "codex-rs" / "Cargo.toml").read_text(encoding="utf-8")

required_manifest_fragments = [
    'rust-version = "1.95"',
    'resolver = "3"',
    'default = []',
    'p1-qualification = [',
    'ed25519-dalek = { version = "=2.2.0"',
]
for fragment in required_manifest_fragments:
    if fragment not in manifest:
        errors.append(f"manifest missing required fragment: {fragment}")

for constant in [
    "AUTHBUS_P1_1_AUTHORITY",
    "AUTHBUS_P1_1_EFFECT_AUTHORITY",
    "AUTHBUS_P1_1_PRODUCTION_CALLER",
    "AUTHBUS_P1_1_PRODUCTION_WRITER",
    "AUTHBUS_P1_1_OPERATOR_ACCEPTANCE",
    "AUTHBUS_P1_1_PROMOTION",
    "AUTHBUS_P1_1_G5_ALLOWED",
    "AUTHBUS_P1_1_EXECUTE_ALLOWED",
    "AUTHBUS_P1_1_LISTENER_ENABLED",
    "AUTHBUS_P1_1_PROVIDER_CALL_ENABLED",
    "AUTHBUS_P1_1_OPENBAO_ENABLED",
    "AUTHBUS_P1_1_PRIVATE_KEY_STORAGE",
]:
    if f"pub const {constant}: bool = false;" not in lib:
        errors.append(f"negative-authority constant is absent or not false: {constant}")

for required_source in [
    "verify_strict",
    "P11NonceReplayCache",
    "P11SignedProviderStatusEvidence",
    "P11SignedManualEvidence",
    "ManualEvidenceRequired",
    "StaleKeyEpoch",
    "KeyRevoked",
]:
    if required_source not in model + verifier:
        errors.append(f"source missing required P1.1 construct: {required_source}")

for forbidden in [
    "TcpListener",
    "UnixListener",
    "reqwest::",
    "openbao::",
    "SigningKey",
    "SecretString",
]:
    if forbidden in lib + model + verifier:
        errors.append(f"qualification source contains forbidden runtime/private-key construct: {forbidden}")

if "hepta-authbus-p1-qualification" in parent_manifest:
    errors.append("P1.1 qualification crate is wired into the parent product workspace")

decision = {
    "schema": "hepta.authbus.p1.1.source-gate.v1",
    "decision": "PASS_AUTHBUS_P1_1_SOURCE_ONLY" if not errors else "FAIL_AUTHBUS_P1_1_SOURCE",
    "source_present": not errors,
    "executed": True,
    "qualification_only": True,
    "rust_qualified": False,
    "authority": False,
    "effect_authority": False,
    "production_caller": False,
    "production_writer": False,
    "operator_acceptance": False,
    "promotion": False,
    "g5_allowed": False,
    "execute_allowed": False,
    "errors": errors,
}
print(json.dumps(decision, sort_keys=True, separators=(",", ":")))
sys.exit(1 if errors else 0)
