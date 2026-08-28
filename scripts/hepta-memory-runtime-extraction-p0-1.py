#!/usr/bin/env python3
"""Migrate Agentd onto the physical Hepta Memory runtime facade crate."""

from __future__ import annotations

import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
WORKSPACE = ROOT / "codex-rs/Cargo.toml"
AGENTD_MANIFEST = ROOT / "codex-rs/hepta-agentd/Cargo.toml"
COMPOSITION = ROOT / "codex-rs/hepta-agentd/src/composition.rs"
STATUS = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_P0_2_STATUS.json"


def replace_once(source: str, old: str, new: str, label: str) -> str:
    if new in source:
        return source
    if source.count(old) != 1:
        raise SystemExit(f"{label} source anchor drifted")
    return source.replace(old, new)


def migrate_workspace() -> None:
    source = WORKSPACE.read_text(encoding="utf-8")
    source = replace_once(
        source,
        '    "hepta-memory",\n    "hepta-matrix-protocol",',
        '    "hepta-memory",\n    "hepta-memory-runtime",\n    "hepta-matrix-protocol",',
        "workspace member",
    )
    source = replace_once(
        source,
        'codex-hepta-memory = { path = "hepta-memory" }\ncodex-hepta-matrix-protocol',
        'codex-hepta-memory = { path = "hepta-memory" }\n'
        'codex-hepta-memory-runtime = { path = "hepta-memory-runtime" }\n'
        'codex-hepta-matrix-protocol',
        "workspace dependency",
    )
    WORKSPACE.write_text(source, encoding="utf-8")


def migrate_agentd_manifest() -> None:
    source = AGENTD_MANIFEST.read_text(encoding="utf-8")
    source = replace_once(
        source,
        "codex-hepta-memory = { workspace = true }\n",
        "codex-hepta-memory = { workspace = true }\n"
        "codex-hepta-memory-runtime = { workspace = true }\n",
        "Agentd memory dependency",
    )
    AGENTD_MANIFEST.write_text(source, encoding="utf-8")


def migrate_composition() -> None:
    source = COMPOSITION.read_text(encoding="utf-8")
    source = replace_once(
        source,
        "use codex_hepta_memory::CognitiveRuntime;\n",
        "use codex_hepta_memory::CognitiveRuntime;\n"
        "use codex_hepta_memory_runtime::AgentMemoryRuntime;\n",
        "Memory runtime facade import",
    )
    start_marker = "        state.refresh_generation()?;\n        let mut cognitive_runtime ="
    end_marker = "\n        let automation_layout = identity.layout.clone();"
    if "let mut memory_runtime = AgentMemoryRuntime::open(" not in source:
        start = source.find(start_marker)
        end = source.find(end_marker, start)
        if start < 0 or end < 0:
            raise SystemExit("Agentd Memory construction source anchors drifted")
        replacement = '''        state.refresh_generation()?;
        let mut memory_runtime = AgentMemoryRuntime::open(
            identity.agent_id.clone(),
            &identity.layout,
            &authority,
        )
        .await
        .map_err(|error| {
            AgentdError::Protocol(format!("open Agent memory runtime facade: {error}"))
        })?;
        state.refresh_generation()?;
        if authority.allows(AuthorityAction::WriteCognitiveState)
            && !memory_runtime
                .cognitive_write_store_available(&authority)
                .map_err(|error| {
                    AgentdError::Protocol(format!("bind cognitive write capability: {error}"))
                })?
        {
            return Err(AgentdError::QualificationCognitiveRuntimeUnavailable);
        }
        if let Some(store) = memory_runtime.cognitive_runtime().available_store() {
            state.attach_cognitive_store(Arc::clone(store))?;
        }

        if memory_runtime.cognitive_runtime().available_store().is_some()
            && !federation_owner_layouts.is_empty()
        {
            state.refresh_generation()?;
            memory_runtime = memory_runtime
                .with_discovered_federation(
                    federation_owner_layouts,
                    now_unix_seconds()?,
                    &authority,
                )
                .await
                .map_err(|error| {
                    AgentdError::Protocol(format!("discover memory federation: {error}"))
                })?;
            state.refresh_generation()?;
        }
        let cognitive_runtime = memory_runtime.into_cognitive_runtime();
'''
        source = source[:start] + replacement + source[end:]
    COMPOSITION.write_text(source, encoding="utf-8")


def migrate_status() -> None:
    value = json.loads(STATUS.read_text(encoding="utf-8"))
    implemented = value.get("implemented")
    if not isinstance(implemented, dict):
        raise SystemExit("status implemented map is missing")
    implemented["memoryRuntimeFacadeCrate"] = True
    STATUS.write_text(json.dumps(value, indent=2, sort_keys=False) + "\n", encoding="utf-8")


def main() -> int:
    migrate_workspace()
    migrate_agentd_manifest()
    migrate_composition()
    migrate_status()
    print("MEMORY_RUNTIME_EXTRACTION_P0_1_APPLIED")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
