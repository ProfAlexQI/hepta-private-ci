#!/usr/bin/env python3
"""Verify and record the legacy production authority adapter materialization."""

from __future__ import annotations

import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
RUNTIME_LIB = ROOT / "codex-rs/hepta-memory-runtime/src/lib.rs"
ADAPTER = ROOT / "codex-rs/hepta-memory-runtime/src/legacy_authority.rs"
AGENTD_ADAPTER = ROOT / "codex-rs/hepta-agentd/src/production_authority_adapter.rs"
WRITER_HOST = ROOT / "codex-rs/hepta-agentd/src/production_writer_host.rs"
STATUS = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_P0_2_STATUS.json"


def verify_source() -> None:
    runtime = RUNTIME_LIB.read_text(encoding="utf-8")
    adapter = ADAPTER.read_text(encoding="utf-8")
    agentd = AGENTD_ADAPTER.read_text(encoding="utf-8")
    host = WRITER_HOST.read_text(encoding="utf-8")

    for marker in (
        "mod legacy_authority;",
        "pub use legacy_authority::ProductionCognitiveWriteAuthorization;",
    ):
        if marker not in runtime:
            raise SystemExit(f"Memory runtime is missing {marker!r}")
    for marker in (
        "pub struct ProductionCognitiveWriteAuthorization",
        "authorize_verified_capability::<CognitiveWriteCapability",
        "ProductionAuthorityVerifier",
        "AuthorityLeaseBinding::new(",
    ):
        if marker not in adapter:
            raise SystemExit(f"legacy authority adapter is missing {marker!r}")
    for forbidden in (
        "ModelInvocationCapability",
        "ProviderDispatchCapability",
        "ExternalEffectCapability",
        "ReleasePromotionCapability",
    ):
        if forbidden in adapter:
            raise SystemExit(f"legacy authority adapter escaped into {forbidden}")
    if "ProductionCognitiveWriteAuthorization" not in agentd:
        raise SystemExit("Agentd production authority adapter is not bound to Memory Runtime")
    if host.count("ProductionCognitiveWriteAuthorization::verify(") < 2:
        raise SystemExit("all production writer open paths must pass through typed verification")


def update_status() -> None:
    value = json.loads(STATUS.read_text(encoding="utf-8"))
    implemented = value.get("implemented")
    remaining = value.get("remaining")
    if not isinstance(implemented, dict) or not isinstance(remaining, dict):
        raise SystemExit("architecture status implementation boundary is missing")
    implemented["legacyProductionLeaseTypedAdapter"] = True
    implemented["legacyProductionWriterHostCallerMigrated"] = True
    remaining["legacyProductionLeaseToTypedWitnessAdapter"] = False
    remaining["allExistingCrossOwnerCallersMigrated"] = False
    remaining.pop("legacyProductionWriterHostCallerMigrated", None)
    STATUS.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


def main() -> int:
    verify_source()
    update_status()
    print("LEGACY_PRODUCTION_AUTHORITY_ADAPTER_P0_1_MATERIALIZED")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
