#!/usr/bin/env python3
"""Materialize the physical Hepta Memory runtime facade in the real workspace.

The script is idempotent and edits dependency metadata plus the machine status.
Agentd source must already consume ``AgentMemoryRuntime`` through
``memory_service.rs``; Cargo regenerates the lock graph and all affected gates
must pass before the finalizer may commit.
"""

from __future__ import annotations

import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
WORKSPACE = ROOT / "codex-rs/Cargo.toml"
AGENTD_MANIFEST = ROOT / "codex-rs/hepta-agentd/Cargo.toml"
MEMORY_SERVICE = ROOT / "codex-rs/hepta-agentd/src/memory_service.rs"
RUNTIME_MANIFEST = ROOT / "codex-rs/hepta-memory-runtime/Cargo.toml"
RUNTIME_LIB = ROOT / "codex-rs/hepta-memory-runtime/src/lib.rs"
STATUS = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_P0_2_STATUS.json"


def insert_once(source: str, old: str, new: str, label: str) -> str:
    if new in source:
        return source
    if source.count(old) != 1:
        raise SystemExit(f"{label} source anchor drifted")
    return source.replace(old, new, 1)


def materialize_workspace() -> None:
    source = WORKSPACE.read_text(encoding="utf-8")
    source = insert_once(
        source,
        '    "hepta-memory",\n    "hepta-matrix-protocol",',
        '    "hepta-memory",\n    "hepta-memory-runtime",\n    "hepta-matrix-protocol",',
        "workspace member",
    )
    source = insert_once(
        source,
        'codex-hepta-memory = { path = "hepta-memory" }\n'
        'codex-hepta-matrix-protocol',
        'codex-hepta-memory = { path = "hepta-memory" }\n'
        'codex-hepta-memory-runtime = { path = "hepta-memory-runtime" }\n'
        'codex-hepta-matrix-protocol',
        "workspace dependency",
    )
    WORKSPACE.write_text(source, encoding="utf-8")


def materialize_agentd_dependency() -> None:
    source = AGENTD_MANIFEST.read_text(encoding="utf-8")
    source = insert_once(
        source,
        "codex-hepta-memory-extension = { workspace = true }\n"
        "codex-hepta-paths = { workspace = true }",
        "codex-hepta-memory-extension = { workspace = true }\n"
        "codex-hepta-memory-runtime = { workspace = true }\n"
        "codex-hepta-paths = { workspace = true }",
        "Agentd runtime dependency",
    )
    AGENTD_MANIFEST.write_text(source, encoding="utf-8")


def verify_real_source_wiring() -> None:
    for path in (RUNTIME_MANIFEST, RUNTIME_LIB, MEMORY_SERVICE):
        if not path.is_file():
            raise SystemExit(f"required source is missing: {path.relative_to(ROOT)}")

    service = MEMORY_SERVICE.read_text(encoding="utf-8")
    required = (
        "use codex_hepta_memory_runtime::AgentMemoryRuntime;",
        "runtime: AgentMemoryRuntime,",
        "AgentMemoryRuntime::open(",
        "runtime.with_discovered_federation(",
        "runtime.into_cognitive_runtime()",
    )
    missing = [needle for needle in required if needle not in service]
    if missing:
        raise SystemExit(f"Agentd Memory facade wiring is incomplete: {missing}")
    if "CognitiveRuntime::open_agent_owned" in service:
        raise SystemExit("Agentd Memory service still bypasses the physical runtime facade")


def update_status() -> None:
    value = json.loads(STATUS.read_text(encoding="utf-8"))
    implemented = value.get("implemented")
    remaining = value.get("remaining")
    if not isinstance(implemented, dict) or not isinstance(remaining, dict):
        raise SystemExit("architecture status implementation boundary is missing")
    implemented["memoryRuntimeFacadeCrate"] = True
    implemented["physicalMemoryRuntimeCrate"] = True
    remaining["physicalMemoryCrateExtraction"] = False
    STATUS.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


def main() -> int:
    materialize_workspace()
    materialize_agentd_dependency()
    verify_real_source_wiring()
    update_status()
    print("MEMORY_RUNTIME_EXTRACTION_P0_1_MATERIALIZED")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
