#!/usr/bin/env python3
"""Physically extract stable Hepta Memory model types from the legacy crate."""

from __future__ import annotations

import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
CODEX = ROOT / "codex-rs"
WORKSPACE = CODEX / "Cargo.toml"
LEGACY_MANIFEST = CODEX / "hepta-memory/Cargo.toml"
LEGACY_LIB = CODEX / "hepta-memory/src/lib.rs"
LEGACY_MODEL = CODEX / "hepta-memory/src/cognitive_model.rs"
MODEL_ROOT = CODEX / "hepta-memory-model"
MODEL_MANIFEST = MODEL_ROOT / "Cargo.toml"
MODEL_LIB = MODEL_ROOT / "src/lib.rs"


def fail(message: str) -> None:
    raise SystemExit(f"FAIL_HEPTA_MEMORY_MODEL_EXTRACTION_P1: {message}")


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        fail(f"{label}: expected one marker {old!r}, found {count}")
    return text.replace(old, new, 1)


def build_model_source(source: str) -> str:
    source = replace_once(
        source,
        "use crate::framing::frame_part;\n",
        "",
        "legacy model framing import",
    )
    source = source.replace("pub(crate)", "pub")
    helper = """
fn frame_part(hasher: &mut Sha256, part: &[u8]) {
    hasher.update((part.len() as u64).to_be_bytes());
    hasher.update(part);
}

"""
    source = replace_once(
        source,
        "fn stable_id(\n",
        helper + "fn stable_id(\n",
        "stable-id helper insertion",
    )
    return "#![forbid(unsafe_code)]\n\n" + source


def main() -> int:
    required = (WORKSPACE, LEGACY_MANIFEST, LEGACY_LIB)
    for path in required:
        if not path.is_file():
            fail(f"required file is missing: {path.relative_to(ROOT)}")

    if MODEL_MANIFEST.is_file() and MODEL_LIB.is_file() and not LEGACY_MODEL.exists():
        print("PASS_HEPTA_MEMORY_MODEL_EXTRACTION_P1_ALREADY_APPLIED")
        return 0
    if MODEL_ROOT.exists():
        fail("partial hepta-memory-model directory already exists")
    if not LEGACY_MODEL.is_file():
        fail("legacy cognitive_model.rs is missing before extraction")

    legacy_source = LEGACY_MODEL.read_text(encoding="utf-8")
    for marker in (
        "pub struct StableMemoryId",
        "pub struct CognitiveAccess",
        "pub struct KgFactSetDraft",
        "fn stable_id(",
    ):
        if marker not in legacy_source:
            fail(f"legacy cognitive model is missing {marker!r}")

    MODEL_LIB.parent.mkdir(parents=True)
    MODEL_MANIFEST.write_text(
        """[package]
name = "codex-hepta-memory-model"
version.workspace = true
edition.workspace = true
license.workspace = true

[lib]
name = "codex_hepta_memory_model"
path = "src/lib.rs"

[lints]
workspace = true

[dependencies]
codex-hepta-contracts = { workspace = true }
serde = { workspace = true }
sha2 = { workspace = true }
""",
        encoding="utf-8",
    )
    MODEL_LIB.write_text(build_model_source(legacy_source), encoding="utf-8")

    workspace = WORKSPACE.read_text(encoding="utf-8")
    workspace = replace_once(
        workspace,
        '    "hepta-memory",\n    "hepta-memory-runtime",',
        '    "hepta-memory",\n    "hepta-memory-model",\n    "hepta-memory-runtime",',
        "workspace member insertion",
    )
    workspace = replace_once(
        workspace,
        'codex-hepta-memory = { path = "hepta-memory" }\ncodex-hepta-memory-runtime',
        'codex-hepta-memory = { path = "hepta-memory" }\n'
        'codex-hepta-memory-model = { path = "hepta-memory-model" }\n'
        'codex-hepta-memory-runtime',
        "workspace dependency insertion",
    )
    WORKSPACE.write_text(workspace, encoding="utf-8")

    legacy_manifest = LEGACY_MANIFEST.read_text(encoding="utf-8")
    legacy_manifest = replace_once(
        legacy_manifest,
        "codex-hepta-contracts = { workspace = true }\n",
        "codex-hepta-contracts = { workspace = true }\n"
        "codex-hepta-memory-model = { workspace = true }\n",
        "legacy dependency insertion",
    )
    LEGACY_MANIFEST.write_text(legacy_manifest, encoding="utf-8")

    legacy_lib = LEGACY_LIB.read_text(encoding="utf-8")
    legacy_lib = replace_once(
        legacy_lib,
        "mod cognitive_model;",
        "#[doc(hidden)]\npub mod cognitive_model {\n"
        "    pub use codex_hepta_memory_model::*;\n"
        "}",
        "legacy compatibility module",
    )
    LEGACY_LIB.write_text(legacy_lib, encoding="utf-8")
    LEGACY_MODEL.unlink()

    print("PASS_HEPTA_MEMORY_MODEL_EXTRACTION_P1_SOURCE")
    return 0


if __name__ == "__main__":
    sys.exit(main())
