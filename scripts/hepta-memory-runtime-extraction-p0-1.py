#!/usr/bin/env python3
"""Materialize the physical Hepta Memory runtime facade in the real workspace.

The script is intentionally idempotent and edits only dependency metadata.
Agentd source already consumes ``AgentMemoryRuntime`` through
``memory_service.rs``; the bootstrap workflow regenerates Cargo.lock and runs
all affected package gates before it is allowed to commit.
"""

from __future__ import annotations

import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
WORKSPACE = ROOT / "codex-rs/Cargo.toml"
AGENTD_MANIFEST = ROOT / "codex-rs/hepta-agentd/Cargo.toml"
MEMORY_SERVICE = ROOT / "codex-rs/hepta-agentd/src/memory_service.rs"
RUNTIME_MANIFEST = ROOT / "codex-rs/hepta-memory-runtime/Cargo.toml"
RUNTIME_LIB = ROOT / "codex-rs/hepta-memory-runtime/src/lib.rs"


def insert_once(source: str, old: str, new: str, label: str) -> str:
    if new in source:
        return source
    if source.count(old) != 1:
        raise SystemExit(f"{label} source anchor drifted")
    return source.replace(old, new)


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


def main() -> int:
    materialize_workspace()
    materialize_agentd_dependency()
    verify_real_source_wiring()
    print("MEMORY_RUNTIME_EXTRACTION_P0_1_MATERIALIZED")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
