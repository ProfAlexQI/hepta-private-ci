#!/usr/bin/env python3
"""Fail closed unless the Memory model bounded context is physically extracted."""

from __future__ import annotations

import pathlib
import sys
import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]
CODEX = ROOT / "codex-rs"
WORKSPACE = CODEX / "Cargo.toml"
LOCK = CODEX / "Cargo.lock"
MODEL_MANIFEST = CODEX / "hepta-memory-model/Cargo.toml"
MODEL_LIB = CODEX / "hepta-memory-model/src/lib.rs"
LEGACY_MANIFEST = CODEX / "hepta-memory/Cargo.toml"
LEGACY_LIB = CODEX / "hepta-memory/src/lib.rs"
LEGACY_MODEL = CODEX / "hepta-memory/src/cognitive_model.rs"


def fail(message: str) -> None:
    raise SystemExit(f"FAIL_HEPTA_MEMORY_MODEL_EXTRACTION_P1: {message}")


def read(path: pathlib.Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError as error:
        fail(f"cannot read {path.relative_to(ROOT)}: {error}")


def load_toml(path: pathlib.Path) -> dict:
    try:
        return tomllib.loads(read(path))
    except tomllib.TOMLDecodeError as error:
        fail(f"invalid TOML in {path.relative_to(ROOT)}: {error}")


def main() -> int:
    for path in (WORKSPACE, LOCK, MODEL_MANIFEST, MODEL_LIB, LEGACY_MANIFEST, LEGACY_LIB):
        if not path.is_file():
            fail(f"required file is missing: {path.relative_to(ROOT)}")
    if LEGACY_MODEL.exists():
        fail("legacy cognitive_model.rs still owns the extracted model source")

    workspace = load_toml(WORKSPACE)
    members = workspace.get("workspace", {}).get("members", [])
    if members.count("hepta-memory-model") != 1:
        fail("hepta-memory-model must be exactly one workspace member")
    dependencies = workspace.get("workspace", {}).get("dependencies", {})
    if dependencies.get("codex-hepta-memory-model", {}).get("path") != "hepta-memory-model":
        fail("workspace dependency for hepta-memory-model is missing")

    model_manifest = load_toml(MODEL_MANIFEST)
    if model_manifest.get("package", {}).get("name") != "codex-hepta-memory-model":
        fail("model crate package identity drifted")
    model_dependencies = model_manifest.get("dependencies", {})
    if "codex-hepta-memory" in model_dependencies:
        fail("model crate must not depend back on the legacy Memory crate")
    if set(model_dependencies) != {"codex-hepta-contracts", "serde", "sha2"}:
        fail(f"unexpected model dependency surface: {sorted(model_dependencies)}")

    legacy_dependencies = load_toml(LEGACY_MANIFEST).get("dependencies", {})
    if "codex-hepta-memory-model" not in legacy_dependencies:
        fail("legacy Memory crate does not consume the physical model crate")

    model_source = read(MODEL_LIB)
    for marker in (
        "pub struct StableMemoryId",
        "pub struct CognitiveAccess",
        "pub struct KgFactSetDraft",
        "pub const MAX_MEMORY_BYTES",
        "fn frame_part(",
        "b\"hepta:cognitive:memory:v2\"",
    ):
        if marker not in model_source:
            fail(f"model source is missing {marker!r}")
    if "use crate::framing::frame_part" in model_source:
        fail("model crate still reaches into the legacy crate")

    legacy_source = read(LEGACY_LIB)
    for marker in (
        "pub mod cognitive_model",
        "pub use codex_hepta_memory_model::*;",
    ):
        if marker not in legacy_source:
            fail(f"legacy compatibility surface is missing {marker!r}")

    lock = load_toml(LOCK)
    packages = {package.get("name"): package for package in lock.get("package", [])}
    if "codex-hepta-memory-model" not in packages:
        fail("model crate is missing from Cargo.lock")
    legacy_package = packages.get("codex-hepta-memory")
    if not isinstance(legacy_package, dict):
        fail("legacy Memory package is missing from Cargo.lock")
    if "codex-hepta-memory-model" not in legacy_package.get("dependencies", []):
        fail("legacy Memory lock entry does not bind the model crate")

    print("PASS_HEPTA_MEMORY_MODEL_EXTRACTION_P1")
    return 0


if __name__ == "__main__":
    sys.exit(main())
