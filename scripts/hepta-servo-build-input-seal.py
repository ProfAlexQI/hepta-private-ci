#!/usr/bin/env python3
"""Seal exact source, toolchain, command, and environment inputs for Servo.

This tool does not compile or execute Servo. It accepts an independently
verified source-bundle receipt plus a strict build recipe and emits one canonical
build-input manifest. All runtime and release authority remains false.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import stat
import sys
from pathlib import Path, PurePosixPath
from typing import Any

EXPECTED_COMMIT = "0a48e298482659817eb50097df23841f2b8e3044"
EXPECTED_TREE = "b04d2f75b3217374d079d579c270177b57fa1389"
EXPECTED_SOURCE_VERIFICATION_SCHEMA = "hepta.browser.servo_source_bundle_verification.v1"
EXPECTED_RECIPE_SCHEMA = "hepta.browser.servo_worker_build_recipe.v1"
OUTPUT_SCHEMA = "hepta.browser.servo_build_input_manifest.v1"
ALLOWED_TARGETS = {
    "x86_64-unknown-linux-gnu",
    "aarch64-apple-darwin",
    "x86_64-pc-windows-msvc",
}
ALLOWED_PROFILES = {"release", "profiling"}
ALLOWED_ENVIRONMENT = {
    "CARGO_NET_OFFLINE": "true",
    "SOURCE_DATE_EPOCH": "0",
    "TZ": "UTC",
    "LC_ALL": "C",
    "LANG": "C",
    "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_TERMINAL_PROMPT": "0",
}
AUTHORITY_KEYS = {
    "runtime_authority",
    "effect_authority",
    "production_caller",
    "production_writer",
    "runtime_external_network",
    "raw_cookie_export",
    "credential_export",
    "operator_acceptance",
    "promotion",
    "release_qualified",
}
SHA256_PATTERN = re.compile(r"^[0-9a-f]{64}$")
GIT_OBJECT_PATTERN = re.compile(r"^[0-9a-f]{40}$")
VERSION_PATTERN = re.compile(r"^[A-Za-z0-9][A-Za-z0-9.+_ -]{0,127}$")
FEATURE_PATTERN = re.compile(r"^[a-z0-9][a-z0-9_-]{0,127}$")
TOKEN_PATTERN = re.compile(r"^[^\x00-\x1f\x7f]{1,512}$")
FORBIDDEN_TOKEN_FRAGMENTS = (
    "&&",
    "||",
    ";",
    "`",
    "$(",
    ">${",
    "<${",
    "http://",
    "https://",
    "git://",
    "ssh://",
)
MAX_JSON_BYTES = 4 * 1024 * 1024


class BuildInputError(RuntimeError):
    pass


def fail(message: str) -> None:
    raise BuildInputError(message)


def canonical_bytes(value: Any) -> bytes:
    return json.dumps(
        value,
        ensure_ascii=False,
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def read_canonical_json(path: Path, label: str) -> tuple[dict[str, Any], bytes]:
    try:
        metadata = path.lstat()
        raw = path.read_bytes()
    except OSError as error:
        fail(f"cannot read {label}: {error}")
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail(f"{label} must be a non-symlink regular file")
    if metadata.st_nlink != 1:
        fail(f"{label} must have exactly one hard link")
    if not raw or len(raw) > MAX_JSON_BYTES:
        fail(f"{label} is empty or exceeds the JSON byte bound")
    try:
        value = json.loads(raw.decode("utf-8", "strict"))
    except (UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot parse {label}: {error}")
    if not isinstance(value, dict):
        fail(f"{label} must contain one JSON object")
    if raw != canonical_bytes(value):
        fail(f"{label} is not compact canonical JSON")
    return value, raw


def require_exact_keys(value: dict[str, Any], expected: set[str], label: str) -> None:
    actual = set(value)
    if actual != expected:
        fail(f"{label} keys differ: {sorted(actual ^ expected)}")


def require_sha256(value: Any, label: str) -> str:
    if not isinstance(value, str) or not SHA256_PATTERN.fullmatch(value):
        fail(f"{label} must be lowercase SHA-256")
    return value


def require_git_object(value: Any, label: str) -> str:
    if not isinstance(value, str) or not GIT_OBJECT_PATTERN.fullmatch(value):
        fail(f"{label} must be a lowercase Git SHA-1 object")
    return value


def require_closed_authority(value: Any, label: str) -> None:
    if not isinstance(value, dict) or set(value) != AUTHORITY_KEYS:
        fail(f"{label} authority keys differ")
    enabled = sorted(key for key, item in value.items() if item is not False)
    if enabled:
        fail(f"{label} attempted to enable authority: {enabled}")


def safe_relative_path(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value or "\\" in value or "\0" in value:
        fail(f"{label} must be a non-empty POSIX relative path")
    path = PurePosixPath(value)
    if path.is_absolute() or ".." in path.parts or any(part in ("", ".") for part in path.parts):
        fail(f"{label} is not a safe relative path")
    return value


def safe_absolute_input(value: str, label: str) -> Path:
    path = Path(value)
    if not path.is_absolute() or ".." in path.parts:
        fail(f"{label} must be a canonical absolute path")
    try:
        resolved = path.resolve(strict=True)
    except OSError as error:
        fail(f"{label} is unavailable: {error}")
    if resolved != path:
        fail(f"{label} must contain no symlink component")
    return path


def validate_source_verification(value: dict[str, Any]) -> None:
    if value.get("schema") != EXPECTED_SOURCE_VERIFICATION_SCHEMA or value.get("schema_version") != 1:
        fail("source verification receipt schema is invalid")
    source = value.get("source")
    if not isinstance(source, dict):
        fail("source verification receipt has no source object")
    for key, expected in (
        ("repository", "servo/servo"),
        ("commit", EXPECTED_COMMIT),
        ("tree", EXPECTED_TREE),
        ("recomputed_tree", EXPECTED_TREE),
    ):
        if source.get(key) != expected:
            fail(f"source verification receipt differs for {key}")
    require_sha256(source.get("tree_manifest_sha256"), "source tree manifest")
    require_sha256(value.get("bundle_receipt_sha256"), "source bundle receipt digest")
    require_sha256(value.get("compressed_archive_sha256"), "compressed source archive digest")
    require_sha256(value.get("tar_sha256"), "source tar digest")
    require_sha256(value.get("license_packet_sha256"), "license packet digest")
    verification = value.get("verification")
    if not isinstance(verification, dict):
        fail("source verification receipt has no verification object")
    for key in (
        "canonical_json",
        "no_machine_local_paths",
        "two_distinct_acquisition_nonces",
        "gzip_single_member_mtime_zero",
        "archive_paths_safe",
        "git_tree_recomputed",
        "pinned_tree_matched",
        "license_matched",
    ):
        if verification.get(key) is not True:
            fail(f"source verification proof is missing: {key}")
    for key in ("servo_built", "servo_runtime_qualified", "release_qualified"):
        if verification.get(key) is not False:
            fail(f"source verification receipt attempted to enable {key}")
    if value.get("machine_local_paths_included") is not False:
        fail("source verification receipt contains machine-local paths")
    require_closed_authority(value.get("authority"), "source verification receipt")


def validate_toolchain(value: Any, target: str) -> dict[str, str]:
    if not isinstance(value, dict):
        fail("build recipe toolchain must be an object")
    require_exact_keys(
        value,
        {
            "rustc_version",
            "rustc_commit_hash",
            "cargo_version",
            "host",
            "target",
            "rustc_binary_sha256",
            "cargo_binary_sha256",
            "linker_kind",
            "linker_version",
            "linker_binary_sha256",
        },
        "build recipe toolchain",
    )
    for key in ("rustc_version", "cargo_version", "host", "linker_kind", "linker_version"):
        item = value.get(key)
        if not isinstance(item, str) or not VERSION_PATTERN.fullmatch(item):
            fail(f"toolchain field {key} is invalid")
    require_git_object(value.get("rustc_commit_hash"), "rustc commit hash")
    for key in ("rustc_binary_sha256", "cargo_binary_sha256", "linker_binary_sha256"):
        require_sha256(value.get(key), key)
    if value.get("target") != target:
        fail("toolchain target differs from build target")
    return {key: str(value[key]) for key in sorted(value)}


def validate_environment(value: Any) -> dict[str, str]:
    if not isinstance(value, dict) or value != ALLOWED_ENVIRONMENT:
        fail("build recipe environment must equal the frozen allowlist")
    return {key: value[key] for key in sorted(value)}


def validate_features(value: Any) -> list[str]:
    if not isinstance(value, list):
        fail("build recipe features must be an array")
    if value != sorted(value) or len(value) != len(set(value)):
        fail("build recipe features must be unique and strictly sorted")
    if len(value) > 64:
        fail("build recipe feature count exceeds the bound")
    for feature in value:
        if not isinstance(feature, str) or not FEATURE_PATTERN.fullmatch(feature):
            fail(f"build recipe feature is invalid: {feature!r}")
    return value


def validate_command_prefix(value: Any) -> list[str]:
    if not isinstance(value, list) or not (2 <= len(value) <= 16):
        fail("build recipe command_prefix must contain 2..16 tokens")
    tokens: list[str] = []
    for token in value:
        if not isinstance(token, str) or not TOKEN_PATTERN.fullmatch(token):
            fail("build recipe command token is invalid")
        if any(fragment in token for fragment in FORBIDDEN_TOKEN_FRAGMENTS):
            fail(f"build recipe command token contains a forbidden fragment: {token!r}")
        if token.startswith(("/", "~")) or "\\" in token:
            fail("build recipe command cannot contain machine-local paths")
        tokens.append(token)
    if tokens[0] != "cargo" or "build" not in tokens[1:]:
        fail("build recipe command_prefix must invoke cargo build directly")
    forbidden_options = {
        "--features",
        "--all-features",
        "--no-default-features",
        "--target",
        "--manifest-path",
        "--profile",
        "--release",
        "--jobs",
        "-j",
    }
    if any(token in forbidden_options for token in tokens):
        fail("build recipe command_prefix duplicates sealer-owned options")
    for required in ("--locked", "--offline", "--frozen"):
        if required not in tokens:
            fail(f"build recipe command_prefix is missing {required}")
    return tokens


def validate_recipe(value: dict[str, Any]) -> dict[str, Any]:
    require_exact_keys(
        value,
        {
            "schema",
            "schema_version",
            "plan_id",
            "stage",
            "status",
            "target",
            "profile",
            "manifest_path",
            "package",
            "artifact_path",
            "features",
            "default_features",
            "jobs",
            "command_prefix",
            "environment",
            "toolchain",
            "build_network",
            "source_mutation_allowed",
            "runtime_external_network",
            "authority",
        },
        "build recipe",
    )
    if value.get("schema") != EXPECTED_RECIPE_SCHEMA or value.get("schema_version") != 1:
        fail("build recipe schema is invalid")
    if value.get("plan_id") != "HEPTA-BROWSER-WEB-D" or value.get("stage") != "WEB_C1":
        fail("build recipe plan/stage binding is invalid")
    if value.get("status") != "FROZEN_INPUTS_NOT_EXECUTED":
        fail("build recipe must remain frozen-inputs/not-executed")
    target = value.get("target")
    profile = value.get("profile")
    if target not in ALLOWED_TARGETS:
        fail("build recipe target is not in the initial platform allowlist")
    if profile not in ALLOWED_PROFILES:
        fail("build recipe profile is invalid")
    manifest_path = safe_relative_path(value.get("manifest_path"), "manifest_path")
    artifact_path = safe_relative_path(value.get("artifact_path"), "artifact_path")
    package = value.get("package")
    if not isinstance(package, str) or not FEATURE_PATTERN.fullmatch(package):
        fail("build recipe package name is invalid")
    if value.get("default_features") is not False:
        fail("build recipe must disable default features")
    jobs = value.get("jobs")
    if not isinstance(jobs, int) or isinstance(jobs, bool) or not (1 <= jobs <= 64):
        fail("build recipe jobs must be an integer within 1..64")
    features = validate_features(value.get("features"))
    command_prefix = validate_command_prefix(value.get("command_prefix"))
    environment = validate_environment(value.get("environment"))
    toolchain = validate_toolchain(value.get("toolchain"), target)
    if value.get("build_network") is not False:
        fail("build recipe attempted to enable build network")
    if value.get("source_mutation_allowed") is not False:
        fail("build recipe attempted to allow source mutation")
    if value.get("runtime_external_network") is not False:
        fail("build recipe attempted to enable runtime external network")
    require_closed_authority(value.get("authority"), "build recipe")
    canonical_command = [
        *command_prefix,
        "--manifest-path",
        manifest_path,
        "--package",
        package,
        "--target",
        target,
        "--profile",
        profile,
        "--jobs",
        str(jobs),
        "--no-default-features",
    ]
    if features:
        canonical_command.extend(["--features", ",".join(features)])
    return {
        "target": target,
        "profile": profile,
        "manifest_path": manifest_path,
        "package": package,
        "artifact_path": artifact_path,
        "features": features,
        "default_features": False,
        "jobs": jobs,
        "canonical_command": canonical_command,
        "environment": environment,
        "toolchain": toolchain,
        "build_network": False,
        "source_mutation_allowed": False,
        "runtime_external_network": False,
    }


def seal(source: dict[str, Any], source_raw: bytes, recipe: dict[str, Any], recipe_raw: bytes) -> dict[str, Any]:
    validate_source_verification(source)
    normalized = validate_recipe(recipe)
    return {
        "schema": OUTPUT_SCHEMA,
        "schema_version": 1,
        "plan_id": "HEPTA-BROWSER-WEB-D",
        "stage": "WEB_C1",
        "status": "SEALED_INPUTS_BUILD_NOT_RUN",
        "source": {
            "repository": "servo/servo",
            "commit": EXPECTED_COMMIT,
            "tree": EXPECTED_TREE,
            "recomputed_tree": EXPECTED_TREE,
            "source_verification_receipt_sha256": sha256_bytes(source_raw),
            "source_bundle_receipt_sha256": source["bundle_receipt_sha256"],
            "compressed_source_archive_sha256": source["compressed_archive_sha256"],
            "source_tar_sha256": source["tar_sha256"],
            "license_packet_sha256": source["license_packet_sha256"],
            "tree_manifest_sha256": source["source"]["tree_manifest_sha256"],
        },
        "recipe_sha256": sha256_bytes(recipe_raw),
        "build": normalized,
        "qualification": {
            "inputs_sealed": True,
            "source_tree_independently_verified": True,
            "command_canonicalized": True,
            "environment_allowlisted": True,
            "toolchain_digests_bound": True,
            "build_network_disabled": True,
            "build_run": False,
            "artifact_created": False,
            "sbom_created": False,
            "servo_runtime_qualified": False,
            "operator_accepted": False,
            "release_qualified": False,
        },
        "machine_local_paths_included": False,
        "authority": {key: False for key in sorted(AUTHORITY_KEYS)},
    }


def write_atomic(path: Path, value: dict[str, Any]) -> None:
    if path.exists():
        fail("build-input manifest output already exists")
    encoded = canonical_bytes(value)
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    with os.fdopen(descriptor, "wb") as handle:
        handle.write(encoded)
        handle.flush()
        os.fsync(handle.fileno())


def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-verification", required=True)
    parser.add_argument("--recipe", required=True)
    parser.add_argument("--output", required=True)
    return parser.parse_args()


def main() -> int:
    try:
        arguments = parse_arguments()
        source_path = safe_absolute_input(arguments.source_verification, "--source-verification")
        recipe_path = safe_absolute_input(arguments.recipe, "--recipe")
        output = Path(arguments.output)
        if not output.is_absolute() or output != output.parent.resolve(strict=True) / output.name:
            fail("--output must be a canonical absolute path")
        source, source_raw = read_canonical_json(source_path, "source verification receipt")
        recipe, recipe_raw = read_canonical_json(recipe_path, "build recipe")
        manifest = seal(source, source_raw, recipe, recipe_raw)
        write_atomic(output, manifest)
    except (BuildInputError, OSError, UnicodeError) as error:
        print(f"HEPTA_SERVO_BUILD_INPUT_SEAL=FAIL: {error}", file=sys.stderr)
        return 1
    print(
        json.dumps(
            {
                "schema": OUTPUT_SCHEMA,
                "status": manifest["status"],
                "commit": EXPECTED_COMMIT,
                "tree": EXPECTED_TREE,
                "target": manifest["build"]["target"],
                "build_run": False,
                "artifact_created": False,
                "servo_runtime_qualified": False,
                "authority": "all_false",
            },
            sort_keys=True,
            separators=(",", ":"),
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
