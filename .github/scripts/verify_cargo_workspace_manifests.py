#!/usr/bin/env python3

"""Verify that codex-rs Cargo manifests follow workspace manifest policy.

Workspace crate feature and optional-dependency exceptions are exact, temporary,
and fail closed.  Broad path-prefix exceptions are intentionally unsupported.
"""

from __future__ import annotations

import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CARGO_RS_ROOT = ROOT / "codex-rs"
WORKSPACE_PACKAGE_FIELDS = ("version", "edition", "license")
TOP_LEVEL_NAME_EXCEPTIONS = {"windows-sandbox-rs": "codex-windows-sandbox"}
UTILITY_NAME_EXCEPTIONS = {"path-utils": "codex-utils-path"}
VENDORED_PACKAGE_METADATA_EXCEPTIONS = {
    "codex-rs/third_party/matrix-sdk-sqlite-0.18.0/Cargo.toml",
}
MANIFEST_FEATURE_EXCEPTIONS: dict[str, dict[str, tuple[str, ...]]] = {
    "codex-rs/hepta-agentd/Cargo.toml": {
        "default": (),
        "qualification-cognitive-write": (),
    },
    "codex-rs/hepta-automation/Cargo.toml": {
        "default": (),
        "taskflow-structural-qualification": (),
    },
    "codex-rs/hepta-contracts/Cargo.toml": {
        "default": (),
        "authbus-local-qualification": ("dep:zeroize",),
    },
    "codex-rs/hepta-matrix-sdk/Cargo.toml": {
        "default": (),
        "qualification-failpoints": (),
    },
    "codex-rs/hepta-matrixd/Cargo.toml": {
        "default": (),
        "real-synapse-e2e": (
            "codex-hepta-matrix-sdk/qualification-failpoints",
        ),
    },
    "codex-rs/hepta-supervisor/Cargo.toml": {
        "default": (),
        "production-authority": (),
    },
    "codex-rs/third_party/matrix-sdk-sqlite-0.18.0/Cargo.toml": {
        "bundled": ("rusqlite/bundled",),
        "crypto-store": ("dep:matrix-sdk-base", "dep:matrix-sdk-crypto"),
        "default": ("state-store", "event-cache"),
        "event-cache": ("dep:matrix-sdk-base",),
        "experimental-encrypted-state-events": (
            "matrix-sdk-crypto?/experimental-encrypted-state-events",
        ),
        "experimental-push-secrets": (
            "matrix-sdk-crypto?/experimental-push-secrets",
        ),
        "state-store": ("dep:matrix-sdk-base",),
        "testing": ("matrix-sdk-crypto?/testing",),
    },
    "codex-rs/v8-poc/Cargo.toml": {
        "sandbox": ("v8/v8_enable_sandbox",),
    },
}
OPTIONAL_DEPENDENCY_EXCEPTIONS = {
    ("codex-rs/hepta-contracts/Cargo.toml", "dependencies", "zeroize"),
    (
        "codex-rs/third_party/matrix-sdk-sqlite-0.18.0/Cargo.toml",
        "dependencies",
        "matrix-sdk-base",
    ),
    (
        "codex-rs/third_party/matrix-sdk-sqlite-0.18.0/Cargo.toml",
        "dependencies",
        "matrix-sdk-crypto",
    ),
}
INTERNAL_DEPENDENCY_FEATURE_EXCEPTIONS: dict[
    tuple[str, str, str], tuple[str, ...]
] = {}


def main() -> int:
    internal_package_names = workspace_package_names()
    used_features: set[str] = set()
    used_optional: set[tuple[str, str, str]] = set()
    used_internal_features: set[tuple[str, str, str]] = set()
    failures: dict[str, list[str]] = {}

    for path in manifests_to_verify():
        errors = manifest_errors(
            path,
            internal_package_names,
            used_features,
            used_optional,
            used_internal_features,
        )
        if errors:
            failures[manifest_key(path)] = errors

    add_unused_exception_errors(
        failures,
        used_features,
        used_optional,
        used_internal_features,
    )
    for path_key in sorted(
        VENDORED_PACKAGE_METADATA_EXCEPTIONS
        - {manifest_key(path) for path in cargo_manifests()}
    ):
        add_failure(failures, path_key, "remove stale vendored metadata exception")

    if not failures:
        return 0

    print(
        "Cargo manifests under codex-rs must inherit workspace package metadata, "
        "opt into workspace lints, and avoid unreviewed crate features."
    )
    print(
        "Feature and optional-dependency exceptions are exact temporary contracts; "
        "update them only with a reviewed migration and remove stale entries."
    )
    print()
    for path in sorted(failures):
        print(f"{path}:")
        for error in failures[path]:
            print(f"  - {error}")
    return 1


def manifest_errors(
    path: Path,
    internal_package_names: set[str],
    used_features: set[str],
    used_optional: set[tuple[str, str, str]],
    used_internal_features: set[tuple[str, str, str]],
) -> list[str]:
    manifest = load_manifest(path)
    package = manifest.get("package")
    if not isinstance(package, dict) and path != CARGO_RS_ROOT / "Cargo.toml":
        return []

    path_key = manifest_key(path)
    errors: list[str] = []
    if isinstance(package, dict) and path_key not in VENDORED_PACKAGE_METADATA_EXCEPTIONS:
        for field in WORKSPACE_PACKAGE_FIELDS:
            if not is_workspace_reference(package.get(field)):
                errors.append(f"set `{field}.workspace = true` in `[package]`")
        lints = manifest.get("lints")
        if not (isinstance(lints, dict) and lints.get("workspace") is True):
            errors.append("add `[lints]` with `workspace = true`")
        expected_name = expected_package_name(path)
        if expected_name is not None and package.get("name") != expected_name:
            errors.append(
                f"set `[package].name` to `{expected_name}` "
                f"(found `{package.get('name')}`)"
            )

    features = manifest.get("features")
    if features is not None:
        normalized = normalize_feature_mapping(features)
        expected = MANIFEST_FEATURE_EXCEPTIONS.get(path_key)
        if expected is None:
            errors.append("remove `[features]`; unreviewed workspace features are forbidden")
        else:
            used_features.add(path_key)
            if normalized != expected:
                errors.append(
                    "limit `[features]` to the exact registered contract "
                    f"(expected {render_feature_mapping(expected)})"
                )

    for section_name, dependencies in dependency_sections(manifest):
        for dependency_name, dependency in dependencies.items():
            if not isinstance(dependency, dict):
                continue
            if dependency.get("optional") is True:
                key = (path_key, section_name, dependency_name)
                if key in OPTIONAL_DEPENDENCY_EXCEPTIONS:
                    used_optional.add(key)
                else:
                    errors.append(
                        "remove `optional = true` from "
                        f"`{dependency_entry_label(section_name, dependency_name)}`; "
                        "the optional dependency is not registered"
                    )

            if not is_internal_dependency(
                path, dependency_name, dependency, internal_package_names
            ):
                continue
            dependency_features = dependency.get("features")
            if dependency_features is not None:
                normalized = normalize_string_list(dependency_features)
                key = (path_key, section_name, dependency_name)
                expected = INTERNAL_DEPENDENCY_FEATURE_EXCEPTIONS.get(key)
                if expected is None:
                    errors.append(
                        "remove `features = [...]` from workspace dependency "
                        f"`{dependency_entry_label(section_name, dependency_name)}`"
                    )
                else:
                    used_internal_features.add(key)
                    if normalized != expected:
                        errors.append(
                            "limit workspace dependency features on "
                            f"`{dependency_entry_label(section_name, dependency_name)}` "
                            f"to {render_string_list(expected)}"
                        )
            if dependency.get("default-features") is False:
                errors.append(
                    "remove `default-features = false` from workspace dependency "
                    f"`{dependency_entry_label(section_name, dependency_name)}`"
                )
    return errors


def expected_package_name(path: Path) -> str | None:
    parts = path.relative_to(CARGO_RS_ROOT).parts
    if len(parts) == 2 and parts[1] == "Cargo.toml":
        directory = parts[0]
        return TOP_LEVEL_NAME_EXCEPTIONS.get(
            directory,
            directory if directory.startswith("codex-") else f"codex-{directory}",
        )
    if len(parts) == 3 and parts[0] == "utils" and parts[2] == "Cargo.toml":
        directory = parts[1]
        return UTILITY_NAME_EXCEPTIONS.get(directory, f"codex-utils-{directory}")
    return None


def is_workspace_reference(value: object) -> bool:
    return isinstance(value, dict) and value.get("workspace") is True


def manifest_key(path: Path) -> str:
    return str(path.relative_to(ROOT))


def normalize_feature_mapping(value: object) -> dict[str, tuple[str, ...]] | None:
    if not isinstance(value, dict):
        return None
    normalized: dict[str, tuple[str, ...]] = {}
    for key, items in value.items():
        normalized_items = normalize_string_list(items)
        if not isinstance(key, str) or normalized_items is None:
            return None
        normalized[key] = normalized_items
    return normalized


def normalize_string_list(value: object) -> tuple[str, ...] | None:
    if not isinstance(value, list) or not all(isinstance(item, str) for item in value):
        return None
    return tuple(value)


def render_feature_mapping(features: dict[str, tuple[str, ...]]) -> str:
    return ", ".join(
        f"{name} = {render_string_list(items)}" for name, items in features.items()
    )


def render_string_list(items: tuple[str, ...]) -> str:
    return "[" + ", ".join(f'"{item}"' for item in items) + "]"


def dependency_sections(manifest: dict) -> list[tuple[str, dict]]:
    sections: list[tuple[str, dict]] = []
    for section_name in ("dependencies", "dev-dependencies", "build-dependencies"):
        dependencies = manifest.get(section_name)
        if isinstance(dependencies, dict):
            sections.append((section_name, dependencies))
    workspace = manifest.get("workspace")
    if isinstance(workspace, dict):
        dependencies = workspace.get("dependencies")
        if isinstance(dependencies, dict):
            sections.append(("workspace.dependencies", dependencies))
    target = manifest.get("target")
    if isinstance(target, dict):
        for target_name, tables in target.items():
            if not isinstance(tables, dict):
                continue
            for section_name in (
                "dependencies",
                "dev-dependencies",
                "build-dependencies",
            ):
                dependencies = tables.get(section_name)
                if isinstance(dependencies, dict):
                    sections.append(
                        (f"target.{target_name}.{section_name}", dependencies)
                    )
    return sections


def dependency_entry_label(section_name: str, dependency_name: str) -> str:
    return f"[{section_name}].{dependency_name}"


def is_internal_dependency(
    manifest_path: Path,
    dependency_name: str,
    dependency: dict,
    internal_package_names: set[str],
) -> bool:
    package_name = dependency.get("package", dependency_name)
    if isinstance(package_name, str) and package_name in internal_package_names:
        return True
    dependency_path = dependency.get("path")
    if not isinstance(dependency_path, str):
        return False
    resolved = (manifest_path.parent / dependency_path).resolve()
    try:
        resolved.relative_to(CARGO_RS_ROOT)
    except ValueError:
        return False
    return True


def add_unused_exception_errors(
    failures: dict[str, list[str]],
    used_features: set[str],
    used_optional: set[tuple[str, str, str]],
    used_internal_features: set[tuple[str, str, str]],
) -> None:
    for path_key in sorted(set(MANIFEST_FEATURE_EXCEPTIONS) - used_features):
        add_failure(failures, path_key, "remove stale feature exception")
    for path_key, section_name, dependency_name in sorted(
        OPTIONAL_DEPENDENCY_EXCEPTIONS - used_optional
    ):
        add_failure(
            failures,
            path_key,
            "remove stale optional-dependency exception for "
            f"`{dependency_entry_label(section_name, dependency_name)}`",
        )
    for path_key, section_name, dependency_name in sorted(
        set(INTERNAL_DEPENDENCY_FEATURE_EXCEPTIONS) - used_internal_features
    ):
        add_failure(
            failures,
            path_key,
            "remove stale internal feature exception for "
            f"`{dependency_entry_label(section_name, dependency_name)}`",
        )


def add_failure(failures: dict[str, list[str]], path_key: str, error: str) -> None:
    failures.setdefault(path_key, []).append(error)


def workspace_package_names() -> set[str]:
    names: set[str] = set()
    for path in cargo_manifests():
        package = load_manifest(path).get("package")
        if isinstance(package, dict) and isinstance(package.get("name"), str):
            names.add(package["name"])
    return names


def load_manifest(path: Path) -> dict:
    return tomllib.loads(path.read_text(encoding="utf-8"))


def cargo_manifests() -> list[Path]:
    return sorted(
        path
        for path in CARGO_RS_ROOT.rglob("Cargo.toml")
        if path != CARGO_RS_ROOT / "Cargo.toml"
    )


def manifests_to_verify() -> list[Path]:
    return [CARGO_RS_ROOT / "Cargo.toml", *cargo_manifests()]


if __name__ == "__main__":
    sys.exit(main())
