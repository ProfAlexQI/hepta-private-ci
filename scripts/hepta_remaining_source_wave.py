#!/usr/bin/env python3
"""Materialize and verify all remaining bounded V8 source roots."""

from __future__ import annotations

import json
import subprocess
from pathlib import Path

from hepta_wave2_control_sources import FILES as CONTROL_FILES
from hepta_wave2_control_sources import RUST_PACKAGES as CONTROL_RUST_PACKAGES
from hepta_wave2_control_sources import SOURCE_ROOTS as CONTROL_SOURCE_ROOTS
from hepta_wave2_external_sources import FILES as EXTERNAL_FILES
from hepta_wave2_external_sources import SOURCE_ROOTS as EXTERNAL_SOURCE_ROOTS
from hepta_wave2_memory_sources import FILES as MEMORY_FILES
from hepta_wave2_memory_sources import RUST_PACKAGES as MEMORY_RUST_PACKAGES
from hepta_wave2_memory_sources import SOURCE_ROOTS as MEMORY_SOURCE_ROOTS

ROOT = Path(__file__).resolve().parents[1]
CARGO_MANIFEST = ROOT / "codex-rs" / "Cargo.toml"
CARGO_LOCK = ROOT / "codex-rs" / "Cargo.lock"
QUALIFICATION_MANIFEST = ROOT / "qualification" / "gap-closure" / "MANIFEST.json"

FILES = {
    **MEMORY_FILES,
    **CONTROL_FILES,
    **EXTERNAL_FILES,
}
RUST_PACKAGES = {
    **MEMORY_RUST_PACKAGES,
    **CONTROL_RUST_PACKAGES,
}
SOURCE_ROOTS = {
    **MEMORY_SOURCE_ROOTS,
    **CONTROL_SOURCE_ROOTS,
    **EXTERNAL_SOURCE_ROOTS,
}
SOURCE_PATHS = tuple(
    sorted({path for roots in SOURCE_ROOTS.values() for path in roots})
)


class RemainingSourceError(RuntimeError):
    """Raised when generated source closure is inconsistent."""


def normalize_files() -> bool:
    changed: list[Path] = []
    for relative, content in FILES.items():
        path = ROOT / relative
        current = path.read_text(encoding="utf-8") if path.exists() else None
        if current == content:
            continue
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")
        changed.append(path)

    manifest = json.loads(QUALIFICATION_MANIFEST.read_text(encoding="utf-8"))
    current_roots = manifest.get("source_roots")
    if not isinstance(current_roots, list) or not all(
        isinstance(item, str) for item in current_roots
    ):
        raise RemainingSourceError("qualification source_roots must be a string list")
    merged_roots = sorted(set(current_roots).union(SOURCE_PATHS))
    if current_roots != merged_roots:
        manifest["source_roots"] = merged_roots
        manifest["status"] = "all_declared_source_roots_materialized"
        QUALIFICATION_MANIFEST.write_text(
            json.dumps(manifest, indent=2, ensure_ascii=False) + "\n",
            encoding="utf-8",
        )
        changed.append(QUALIFICATION_MANIFEST)

    if changed:
        subprocess.run(
            ["git", "-C", str(ROOT), "add", "--", *map(str, changed)],
            check=True,
        )
    return bool(changed)


def format_and_stage() -> bool:
    command = [
        "cargo",
        "fmt",
        "--manifest-path",
        str(CARGO_MANIFEST),
    ]
    for package in RUST_PACKAGES.values():
        command.extend(["--package", package])
    subprocess.run(command, cwd=ROOT, check=True)

    paths = [ROOT / relative for relative in FILES]
    paths.extend([CARGO_MANIFEST, CARGO_LOCK])
    subprocess.run(
        ["git", "-C", str(ROOT), "add", "--", *map(str, paths)],
        check=True,
    )
    result = subprocess.run(
        ["git", "-C", str(ROOT), "diff", "--cached", "--quiet"],
        check=False,
    )
    return result.returncode != 0


def verify() -> list[str]:
    failures: list[str] = []
    for root_name, package_name in RUST_PACKAGES.items():
        root = ROOT / "codex-rs" / root_name
        required = (
            root / "Cargo.toml",
            root / "BUILD.bazel",
            root / "src" / "lib.rs",
            root / "src" / "lib_tests.rs",
        )
        for path in required:
            if not path.is_file():
                failures.append(f"missing remaining source file: {path.relative_to(ROOT)}")
        manifest_path = root / "Cargo.toml"
        if manifest_path.is_file():
            text = manifest_path.read_text(encoding="utf-8")
            if f'name = "{package_name}"' not in text:
                failures.append(f"remaining package identity mismatch: {root_name}")
            if "[lints]\nworkspace = true" not in text:
                failures.append(f"remaining workspace lints missing: {root_name}")
        lib_path = root / "src" / "lib.rs"
        if lib_path.is_file() and "#![forbid(unsafe_code)]" not in lib_path.read_text(
            encoding="utf-8"
        ):
            failures.append(f"remaining unsafe-code prohibition missing: {root_name}")

    for relative in EXTERNAL_FILES:
        path = ROOT / relative
        if not path.is_file():
            failures.append(f"missing external or UI source file: {relative}")

    if QUALIFICATION_MANIFEST.is_file():
        try:
            manifest = json.loads(QUALIFICATION_MANIFEST.read_text(encoding="utf-8"))
        except json.JSONDecodeError as error:
            failures.append(f"invalid qualification manifest: {error}")
        else:
            roots = manifest.get("source_roots")
            if not isinstance(roots, list):
                failures.append("qualification source_roots is not a list")
            else:
                missing = sorted(set(SOURCE_PATHS) - set(roots))
                failures.extend(
                    f"qualification source root is missing: {root}" for root in missing
                )

    servo_path = ROOT / "third_party" / "servo-patches" / "MANIFEST.json"
    if servo_path.is_file():
        servo = json.loads(servo_path.read_text(encoding="utf-8"))
        if servo.get("upstream_commit") != "84bcc9ac701874fa9819e5cdee06356b961d736c":
            failures.append("Servo source pin drifted")
        if servo.get("automatic_update") is not False:
            failures.append("Servo automatic update must remain disabled")

    bao_path = ROOT / "external" / "HeptaBao" / "EXTERNAL_SOURCE.json"
    if bao_path.is_file():
        bao = json.loads(bao_path.read_text(encoding="utf-8"))
        if bao.get("commit") != "1c69131d7251bc02ebeab726689ecd53bce89968":
            failures.append("HeptaBao source pin drifted")
        if bao.get("contains_secret_material") is not False:
            failures.append("external HeptaBao binding must contain no secret material")

    alias_path = ROOT / "codex-rs" / "codex-app-server" / "BINDING.json"
    if alias_path.is_file():
        alias = json.loads(alias_path.read_text(encoding="utf-8"))
        if alias.get("implementation_root") != "codex-rs/app-server":
            failures.append("Codex app-server alias target drifted")
        if alias.get("duplicate_cargo_package_created") is not False:
            failures.append("Codex app-server alias must not create a duplicate package")

    return failures
