#!/usr/bin/env python3
"""Wire the verified legacy production authority adapter into Memory Runtime."""

from __future__ import annotations

import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
RUNTIME_LIB = ROOT / "codex-rs/hepta-memory-runtime/src/lib.rs"
STATUS = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_P0_2_STATUS.json"

MODULE_DECLARATION = "mod legacy_authority;\n\n"
EXPORT_BLOCK = """pub use legacy_authority::LegacyAuthorityBridgeError;
pub use legacy_authority::LegacyProductionAuthorityVerifier;
pub use legacy_authority::LegacyProductionLeaseEvidence;
pub use legacy_authority::VerifiedProductionCognitiveWrite;
pub use legacy_authority::adopt_verified_legacy_cognitive_write;

"""


def migrate_runtime_lib() -> None:
    source = RUNTIME_LIB.read_text(encoding="utf-8")
    marker = "#![forbid(unsafe_code)]\n\n"
    if MODULE_DECLARATION not in source:
        if source.count(marker) != 1:
            raise SystemExit("Memory runtime module insertion anchor drifted")
        source = source.replace(marker, marker + MODULE_DECLARATION, 1)
    if EXPORT_BLOCK not in source:
        import_marker = "use std::fmt;\n\n"
        if source.count(import_marker) != 1:
            raise SystemExit("Memory runtime export insertion anchor drifted")
        source = source.replace(import_marker, import_marker + EXPORT_BLOCK, 1)
    RUNTIME_LIB.write_text(source, encoding="utf-8")


def migrate_status() -> None:
    value = json.loads(STATUS.read_text(encoding="utf-8"))
    implemented = value.get("implemented")
    remaining = value.get("remaining")
    if not isinstance(implemented, dict) or not isinstance(remaining, dict):
        raise SystemExit("architecture status implementation boundary is missing")
    implemented["legacyProductionLeaseTypedAdapter"] = True
    remaining["legacyProductionLeaseToTypedWitnessAdapter"] = False
    remaining["legacyProductionWriterHostCallerMigrated"] = True
    STATUS.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


def main() -> int:
    migrate_runtime_lib()
    migrate_status()
    print("LEGACY_PRODUCTION_AUTHORITY_ADAPTER_P0_1_APPLIED")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
