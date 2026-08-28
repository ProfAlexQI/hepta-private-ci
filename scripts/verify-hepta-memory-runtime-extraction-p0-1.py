#!/usr/bin/env python3
"""Verify the physical Hepta Memory runtime extraction and Agentd binding."""

from __future__ import annotations

import json
import pathlib
import sys
import tomllib
from typing import NoReturn

ROOT = pathlib.Path(__file__).resolve().parents[1]
CODEX = ROOT / "codex-rs"
WORKSPACE = CODEX / "Cargo.toml"
LOCKFILE = CODEX / "Cargo.lock"
AGENTD_MANIFEST = CODEX / "hepta-agentd/Cargo.toml"
AGENTD_SOURCE = CODEX / "hepta-agentd/src"
MEMORY_SERVICE = AGENTD_SOURCE / "memory_service.rs"
RUNTIME_MANIFEST = CODEX / "hepta-memory-runtime/Cargo.toml"
RUNTIME_LIB = CODEX / "hepta-memory-runtime/src/lib.rs"
RUNTIME_ADAPTER = CODEX / "hepta-memory-runtime/src/legacy_authority.rs"
STATUS = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_P0_2_STATUS.json"


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_MEMORY_RUNTIME_EXTRACTION_P0_1: {message}")


def read_toml(path: pathlib.Path) -> dict:
    try:
        return tomllib.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, tomllib.TOMLDecodeError) as error:
        fail(f"cannot read {path.relative_to(ROOT)}: {error}")


def main() -> int:
    for path in (
        WORKSPACE,
        LOCKFILE,
        AGENTD_MANIFEST,
        MEMORY_SERVICE,
        RUNTIME_MANIFEST,
        RUNTIME_LIB,
        RUNTIME_ADAPTER,
        STATUS,
    ):
        if not path.is_file():
            fail(f"required file is missing: {path.relative_to(ROOT)}")

    workspace = read_toml(WORKSPACE)
    members = workspace.get("workspace", {}).get("members", [])
    if members.count("hepta-memory-runtime") != 1:
        fail("hepta-memory-runtime must be exactly one workspace member")
    dependencies = workspace.get("workspace", {}).get("dependencies", {})
    if dependencies.get("codex-hepta-memory-runtime", {}).get("path") != "hepta-memory-runtime":
        fail("workspace dependency codex-hepta-memory-runtime is missing or drifted")

    runtime_manifest = read_toml(RUNTIME_MANIFEST)
    if runtime_manifest.get("package", {}).get("name") != "codex-hepta-memory-runtime":
        fail("Memory runtime package identity drifted")

    agentd_manifest = read_toml(AGENTD_MANIFEST)
    if "codex-hepta-memory-runtime" not in agentd_manifest.get("dependencies", {}):
        fail("Agentd does not depend on the Memory runtime facade")

    service = MEMORY_SERVICE.read_text(encoding="utf-8")
    for marker in (
        "use codex_hepta_memory_runtime::AgentMemoryRuntime;",
        "runtime: AgentMemoryRuntime,",
        "AgentMemoryRuntime::open(",
        "runtime.with_discovered_federation(",
        "self.runtime.into_cognitive_runtime()",
    ):
        if marker not in service:
            fail(f"Agentd Memory service is missing {marker!r}")
    if "CognitiveRuntime::open_agent_owned" in service:
        fail("Agentd still opens the Memory implementation crate directly")

    direct_open_callers: list[str] = []
    for path in sorted(AGENTD_SOURCE.rglob("*.rs")):
        source = path.read_text(encoding="utf-8")
        if "CognitiveRuntime::open_agent_owned" in source:
            direct_open_callers.append(path.relative_to(ROOT).as_posix())
    if direct_open_callers:
        fail(f"direct Memory implementation open callers remain: {direct_open_callers}")

    runtime_source = RUNTIME_LIB.read_text(encoding="utf-8")
    for marker in (
        "mod legacy_authority;",
        "pub struct AgentMemoryRuntime",
        "pub use legacy_authority::ProductionCognitiveWriteAuthorization;",
    ):
        if marker not in runtime_source:
            fail(f"Memory runtime facade is missing {marker!r}")

    adapter = RUNTIME_ADAPTER.read_text(encoding="utf-8")
    for marker in (
        "pub struct ProductionCognitiveWriteAuthorization",
        "authorize_verified_capability::<CognitiveWriteCapability",
        "ProductionAuthorityVerifier",
        "AuthorityLeaseBinding::new(",
    ):
        if marker not in adapter:
            fail(f"legacy authority adapter is missing {marker!r}")

    lock = read_toml(LOCKFILE)
    packages = {package.get("name"): package for package in lock.get("package", [])}
    if packages.get("codex-hepta-memory-runtime") is None:
        fail("codex-hepta-memory-runtime is missing from Cargo.lock")
    agentd_lock = packages.get("codex-hepta-agentd")
    if agentd_lock is None:
        fail("codex-hepta-agentd is missing from Cargo.lock")
    if "codex-hepta-memory-runtime" not in agentd_lock.get("dependencies", []):
        fail("Agentd Cargo.lock entry does not bind the Memory runtime facade")

    try:
        status = json.loads(STATUS.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot read architecture status: {error}")
    implemented = status.get("implemented")
    remaining = status.get("remaining")
    if not isinstance(implemented, dict) or not isinstance(remaining, dict):
        fail("architecture status implementation boundary is missing")
    for field in (
        "memoryRuntimeFacadeCrate",
        "physicalMemoryRuntimeCrate",
        "legacyProductionLeaseTypedAdapter",
        "legacyProductionWriterHostCallerMigrated",
    ):
        if implemented.get(field) is not True:
            fail(f"implemented status field {field} must be true")
    for field in (
        "legacyProductionLeaseToTypedWitnessAdapter",
        "physicalMemoryCrateExtraction",
        "allExistingCrossOwnerCallersMigrated",
    ):
        if remaining.get(field) is not False:
            fail(f"remaining status field {field} must be false")
    if "legacyProductionWriterHostCallerMigrated" in remaining:
        fail("obsolete remaining legacyProductionWriterHostCallerMigrated key is present")

    authority = status.get("authority")
    if not isinstance(authority, dict) or any(authority.values()):
        fail("Memory extraction must not open any production authority field")

    print("PASS_HEPTA_MEMORY_RUNTIME_EXTRACTION_P0_1")
    return 0


if __name__ == "__main__":
    sys.exit(main())
