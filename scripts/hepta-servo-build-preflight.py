#!/usr/bin/env python3
"""Verify every input immediately before the first bounded Servo build.

The preflight is Linux-first and does not invoke Cargo, rustc, a linker, build
scripts, or the Servo worker. It cross-binds canonical receipts, the compressed
source archive, the exact build recipe, the sealed build-input manifest, and
current toolchain binary bytes. A pass permits only a separately controlled
bounded build attempt.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import stat
import sys
from pathlib import Path, PurePosixPath
from typing import Any

EXPECTED_COMMIT = "0a48e298482659817eb50097df23841f2b8e3044"
EXPECTED_TREE = "b04d2f75b3217374d079d579c270177b57fa1389"
EXPECTED_TARGET = "x86_64-unknown-linux-gnu"
SOURCE_VERIFICATION_SCHEMA = "hepta.browser.servo_source_bundle_verification.v1"
TOOLCHAIN_SCHEMA = "hepta.browser.servo_toolchain_receipt.v1"
RECIPE_SCHEMA = "hepta.browser.servo_worker_build_recipe.v1"
BUILD_INPUT_SCHEMA = "hepta.browser.servo_build_input_manifest.v2"
OUTPUT_SCHEMA = "hepta.browser.servo_build_preflight.v1"
MAX_JSON_BYTES = 8 * 1024 * 1024
MAX_INPUT_BYTES = 8 * 1024 * 1024 * 1024
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
FROZEN_ENVIRONMENT = {
    "CARGO_NET_OFFLINE": "true",
    "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_TERMINAL_PROMPT": "0",
    "LANG": "C",
    "LC_ALL": "C",
    "SOURCE_DATE_EPOCH": "0",
    "TZ": "UTC",
}
COMMAND_PREFIX = ["cargo", "build", "--locked", "--offline", "--frozen"]


class BuildPreflightError(RuntimeError):
    pass


def fail(message: str) -> None:
    raise BuildPreflightError(message)


def canonical_bytes(value: Any) -> bytes:
    return json.dumps(
        value,
        ensure_ascii=False,
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def sha256_file(path: Path, *, maximum: int = MAX_INPUT_BYTES) -> tuple[str, int]:
    digest = hashlib.sha256()
    length = 0
    with path.open("rb") as handle:
        while True:
            block = handle.read(1024 * 1024)
            if not block:
                break
            length += len(block)
            if length > maximum:
                fail(f"input exceeds the byte bound: {path.name}")
            digest.update(block)
    if length == 0:
        fail(f"input is empty: {path.name}")
    return digest.hexdigest(), length


def canonical_absolute(value: str, label: str, *, must_exist: bool = True) -> Path:
    path = Path(value)
    if not path.is_absolute() or ".." in path.parts:
        fail(f"{label} must be an absolute path without '..'")
    try:
        parent = path.parent.resolve(strict=True)
    except OSError as error:
        fail(f"{label} parent is unavailable: {error}")
    if path != parent / path.name:
        fail(f"{label} must already be canonical and contain no symlink component")
    if must_exist:
        try:
            resolved = path.resolve(strict=True)
        except OSError as error:
            fail(f"{label} is unavailable: {error}")
        if resolved != path:
            fail(f"{label} must already be canonical and contain no symlink component")
    return path


def require_regular_file(path: Path, label: str, *, executable: bool = False) -> os.stat_result:
    try:
        metadata = path.lstat()
    except OSError as error:
        fail(f"{label} is unavailable: {error}")
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail(f"{label} must be a non-symlink regular file")
    if metadata.st_nlink != 1:
        fail(f"{label} must have exactly one hard link")
    if metadata.st_mode & 0o022:
        fail(f"{label} must not be group/world writable")
    if executable and not os.access(path, os.X_OK):
        fail(f"{label} must be executable")
    return metadata


def read_canonical_json(path: Path, label: str) -> tuple[dict[str, Any], bytes]:
    require_regular_file(path, label)
    try:
        raw = path.read_bytes()
    except OSError as error:
        fail(f"cannot read {label}: {error}")
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
    if (
        not isinstance(value, str)
        or len(value) != 64
        or any(character not in "0123456789abcdef" for character in value)
    ):
        fail(f"{label} must be lowercase SHA-256")
    return value


def require_closed_authority(value: Any, label: str) -> None:
    if not isinstance(value, dict) or set(value) != AUTHORITY_KEYS:
        fail(f"{label} authority keys differ")
    enabled = sorted(key for key, item in value.items() if item is not False)
    if enabled:
        fail(f"{label} attempted to enable authority: {enabled}")


def safe_relative(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value or "\\" in value or "\0" in value:
        fail(f"{label} must be a non-empty POSIX relative path")
    path = PurePosixPath(value)
    if path.is_absolute() or ".." in path.parts or any(part in ("", ".") for part in path.parts):
        fail(f"{label} is unsafe")
    return value


def private_empty_directory(path: Path, label: str) -> dict[str, Any]:
    try:
        metadata = path.lstat()
    except OSError as error:
        fail(f"{label} is unavailable: {error}")
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        fail(f"{label} must be a non-symlink directory")
    if hasattr(os, "getuid") and metadata.st_uid != os.getuid():
        fail(f"{label} is not owned by the current user")
    mode = stat.S_IMODE(metadata.st_mode)
    if mode & 0o077:
        fail(f"{label} must not grant group/world permissions")
    try:
        entries = list(path.iterdir())
    except OSError as error:
        fail(f"cannot inspect {label}: {error}")
    if entries:
        fail(f"{label} must be empty")
    return {
        "empty": True,
        "owned_by_current_user": True,
        "group_world_permissions": False,
        "mode_octal": format(mode, "04o"),
    }


def validate_source_verification(value: dict[str, Any]) -> None:
    if value.get("schema") != SOURCE_VERIFICATION_SCHEMA or value.get("schema_version") != 1:
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
    for key in (
        "bundle_receipt_sha256",
        "compressed_archive_sha256",
        "tar_sha256",
        "license_packet_sha256",
    ):
        require_sha256(value.get(key), key)
    verification = value.get("verification")
    if not isinstance(verification, dict):
        fail("source verification facts are missing")
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
            fail(f"source verification attempted to enable {key}")
    if value.get("machine_local_paths_included") is not False:
        fail("source verification contains machine-local paths")
    require_closed_authority(value.get("authority"), "source verification")


def toolchain_projection(value: dict[str, Any]) -> dict[str, str]:
    if value.get("schema") != TOOLCHAIN_SCHEMA or value.get("schema_version") != 1:
        fail("toolchain receipt schema is invalid")
    if value.get("target") != EXPECTED_TARGET:
        fail("toolchain receipt target is not the first Linux target")
    host = value.get("host")
    rustc = value.get("rustc")
    cargo = value.get("cargo")
    linker = value.get("linker")
    capture = value.get("capture")
    if not all(isinstance(item, dict) for item in (rustc, cargo, linker, capture)):
        fail("toolchain receipt facts are incomplete")
    if rustc.get("host") != host or cargo.get("host") != host:
        fail("toolchain receipt component hosts differ")
    for section, label in ((rustc, "rustc"), (cargo, "cargo"), (linker, "linker")):
        require_sha256(section.get("binary_sha256"), f"{label} binary digest")
        require_sha256(section.get("output_sha256"), f"{label} output digest")
        size = section.get("binary_bytes")
        if not isinstance(size, int) or isinstance(size, bool) or size <= 0:
            fail(f"{label} binary size is invalid")
    commit_hash = rustc.get("commit_hash")
    if (
        not isinstance(commit_hash, str)
        or len(commit_hash) != 40
        or any(character not in "0123456789abcdef" for character in commit_hash)
    ):
        fail("rustc commit hash is invalid")
    for key in ("version",):
        for section, label in ((rustc, "rustc"), (cargo, "cargo"), (linker, "linker")):
            text = section.get(key)
            if not isinstance(text, str) or not text or len(text) > 256 or any(ord(ch) < 32 or ord(ch) == 127 for ch in text):
                fail(f"{label} version text is invalid")
    if capture.get("network_access_used") is not False:
        fail("toolchain receipt used network access")
    if capture.get("build_run") is not False or capture.get("artifact_created") is not False:
        fail("toolchain receipt attempted to claim a build or artifact")
    if value.get("machine_local_paths_included") is not False:
        fail("toolchain receipt contains machine-local paths")
    require_closed_authority(value.get("authority"), "toolchain receipt")
    return {
        "rustc_version": rustc["version"],
        "rustc_commit_hash": rustc["commit_hash"],
        "cargo_version": cargo["version"],
        "host": host,
        "target": value["target"],
        "rustc_binary_sha256": rustc["binary_sha256"],
        "cargo_binary_sha256": cargo["binary_sha256"],
        "linker_kind": linker["kind"],
        "linker_version": linker["version"],
        "linker_binary_sha256": linker["binary_sha256"],
    }


def validate_recipe(value: dict[str, Any], projection: dict[str, str]) -> dict[str, Any]:
    if value.get("schema") != RECIPE_SCHEMA or value.get("schema_version") != 1:
        fail("build recipe schema is invalid")
    if value.get("plan_id") != "HEPTA-BROWSER-WEB-D" or value.get("stage") != "WEB_C1":
        fail("build recipe plan/stage is invalid")
    if value.get("status") != "FROZEN_INPUTS_NOT_EXECUTED":
        fail("build recipe status is invalid")
    if value.get("target") != EXPECTED_TARGET:
        fail("build recipe target is not the first Linux target")
    if value.get("profile") not in ("release", "profiling"):
        fail("build recipe profile is invalid")
    manifest_path = safe_relative(value.get("manifest_path"), "manifest_path")
    artifact_path = safe_relative(value.get("artifact_path"), "artifact_path")
    package = value.get("package")
    if not isinstance(package, str) or not package or len(package) > 128:
        fail("build recipe package is invalid")
    features = value.get("features")
    if not isinstance(features, list) or features != sorted(features) or len(features) != len(set(features)):
        fail("build recipe features must be unique and strictly sorted")
    if any(not isinstance(feature, str) or not feature for feature in features):
        fail("build recipe feature is invalid")
    if value.get("default_features") is not False:
        fail("build recipe must disable default features")
    jobs = value.get("jobs")
    if not isinstance(jobs, int) or isinstance(jobs, bool) or not (1 <= jobs <= 64):
        fail("build recipe jobs are invalid")
    if value.get("command_prefix") != COMMAND_PREFIX:
        fail("build recipe command prefix must equal the frozen direct Cargo command")
    if value.get("environment") != FROZEN_ENVIRONMENT:
        fail("build recipe environment differs from the frozen allowlist")
    if value.get("toolchain") != projection:
        fail("build recipe toolchain projection differs from the receipt")
    for key in ("build_network", "source_mutation_allowed", "runtime_external_network"):
        if value.get(key) is not False:
            fail(f"build recipe attempted to enable {key}")
    require_closed_authority(value.get("authority"), "build recipe")
    command = [
        *COMMAND_PREFIX,
        "--manifest-path",
        manifest_path,
        "--package",
        package,
        "--target",
        EXPECTED_TARGET,
        "--profile",
        value["profile"],
        "--jobs",
        str(jobs),
        "--no-default-features",
    ]
    if features:
        command.extend(["--features", ",".join(features)])
    return {
        "target": EXPECTED_TARGET,
        "profile": value["profile"],
        "manifest_path": manifest_path,
        "package": package,
        "artifact_path": artifact_path,
        "features": features,
        "default_features": False,
        "jobs": jobs,
        "canonical_command": command,
        "environment": dict(sorted(FROZEN_ENVIRONMENT.items())),
        "toolchain": projection,
        "build_network": False,
        "source_mutation_allowed": False,
        "runtime_external_network": False,
    }


def validate_manifest(
    value: dict[str, Any],
    source: dict[str, Any],
    source_raw: bytes,
    recipe_raw: bytes,
    toolchain_raw: bytes,
    normalized_recipe: dict[str, Any],
) -> None:
    if value.get("schema") != BUILD_INPUT_SCHEMA or value.get("schema_version") != 1:
        fail("build-input manifest schema is invalid or not v2")
    if value.get("plan_id") != "HEPTA-BROWSER-WEB-D" or value.get("stage") != "WEB_C1":
        fail("build-input manifest plan/stage is invalid")
    if value.get("status") != "SEALED_INPUTS_BUILD_NOT_RUN":
        fail("build-input manifest status is invalid")
    source_projection = value.get("source")
    if not isinstance(source_projection, dict):
        fail("build-input manifest source projection is missing")
    expected_source = {
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
    }
    if source_projection != expected_source:
        fail("build-input manifest source projection differs from accepted source bytes")
    if value.get("recipe_sha256") != sha256_bytes(recipe_raw):
        fail("build-input manifest recipe digest differs")
    if value.get("toolchain_receipt_sha256") != sha256_bytes(toolchain_raw):
        fail("build-input manifest toolchain receipt digest differs")
    if value.get("build") != normalized_recipe:
        fail("build-input manifest build projection differs from recipe")
    qualification = value.get("qualification")
    if not isinstance(qualification, dict):
        fail("build-input manifest qualification facts are missing")
    for key in (
        "inputs_sealed",
        "source_tree_independently_verified",
        "command_canonicalized",
        "environment_allowlisted",
        "toolchain_digests_bound",
        "toolchain_receipt_independently_captured",
        "build_network_disabled",
    ):
        if qualification.get(key) is not True:
            fail(f"build-input manifest proof is missing: {key}")
    for key in (
        "build_run",
        "artifact_created",
        "sbom_created",
        "servo_runtime_qualified",
        "operator_accepted",
        "release_qualified",
    ):
        if qualification.get(key) is not False:
            fail(f"build-input manifest attempted to enable {key}")
    if value.get("machine_local_paths_included") is not False:
        fail("build-input manifest contains machine-local paths")
    require_closed_authority(value.get("authority"), "build-input manifest")


def verify_binary(path: Path, expected_sha256: str, expected_bytes: int, label: str) -> dict[str, Any]:
    require_regular_file(path, label, executable=True)
    digest, length = sha256_file(path, maximum=1024 * 1024 * 1024)
    if digest != expected_sha256 or length != expected_bytes:
        fail(f"{label} bytes differ from the captured toolchain receipt")
    return {
        "sha256": digest,
        "bytes": length,
        "non_symlink": True,
        "single_hardlink": True,
        "group_world_writable": False,
        "executable": True,
    }


def run_preflight(
    *,
    source: dict[str, Any],
    source_raw: bytes,
    recipe: dict[str, Any],
    recipe_raw: bytes,
    toolchain: dict[str, Any],
    toolchain_raw: bytes,
    manifest: dict[str, Any],
    manifest_raw: bytes,
    source_archive: Path,
    rustc: Path,
    cargo: Path,
    linker: Path,
    source_root: Path,
    artifact_root: Path,
) -> dict[str, Any]:
    validate_source_verification(source)
    projection = toolchain_projection(toolchain)
    normalized_recipe = validate_recipe(recipe, projection)
    validate_manifest(
        manifest,
        source,
        source_raw,
        recipe_raw,
        toolchain_raw,
        normalized_recipe,
    )
    require_regular_file(source_archive, "compressed source archive")
    archive_sha256, archive_bytes = sha256_file(source_archive)
    if archive_sha256 != source["compressed_archive_sha256"]:
        fail("compressed source archive differs from source verification receipt")
    rustc_record = toolchain["rustc"]
    cargo_record = toolchain["cargo"]
    linker_record = toolchain["linker"]
    binaries = {
        "rustc": verify_binary(
            rustc,
            rustc_record["binary_sha256"],
            rustc_record["binary_bytes"],
            "rustc binary",
        ),
        "cargo": verify_binary(
            cargo,
            cargo_record["binary_sha256"],
            cargo_record["binary_bytes"],
            "cargo binary",
        ),
        "linker": verify_binary(
            linker,
            linker_record["binary_sha256"],
            linker_record["binary_bytes"],
            "linker binary",
        ),
    }
    if os.path.samefile(source_root, artifact_root):
        fail("source and artifact roots must be distinct")
    roots = {
        "source_root": private_empty_directory(source_root, "source root"),
        "artifact_root": private_empty_directory(artifact_root, "artifact root"),
    }
    return {
        "schema": OUTPUT_SCHEMA,
        "schema_version": 1,
        "plan_id": "HEPTA-BROWSER-WEB-D",
        "stage": "WEB_C1",
        "status": "READY_FOR_SEPARATE_BOUNDED_BUILD",
        "inputs": {
            "source_verification_receipt_sha256": sha256_bytes(source_raw),
            "recipe_sha256": sha256_bytes(recipe_raw),
            "toolchain_receipt_sha256": sha256_bytes(toolchain_raw),
            "build_input_manifest_sha256": sha256_bytes(manifest_raw),
            "compressed_source_archive_sha256": archive_sha256,
            "compressed_source_archive_bytes": archive_bytes,
        },
        "build": {
            "target": normalized_recipe["target"],
            "profile": normalized_recipe["profile"],
            "package": normalized_recipe["package"],
            "manifest_path": normalized_recipe["manifest_path"],
            "artifact_path": normalized_recipe["artifact_path"],
            "features": normalized_recipe["features"],
            "jobs": normalized_recipe["jobs"],
            "canonical_command": normalized_recipe["canonical_command"],
            "environment": normalized_recipe["environment"],
            "build_network": False,
            "runtime_external_network": False,
        },
        "toolchain_binaries": binaries,
        "filesystem_roots": roots,
        "preflight": {
            "source_receipt_verified": True,
            "source_archive_verified": True,
            "recipe_verified": True,
            "toolchain_receipt_verified": True,
            "build_input_manifest_verified": True,
            "toolchain_binaries_rehashed": True,
            "private_empty_roots_verified": True,
            "ready_for_separate_bounded_build": True,
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
        fail("preflight receipt output already exists")
    encoded = canonical_bytes(value)
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    with os.fdopen(descriptor, "wb") as handle:
        handle.write(encoded)
        handle.flush()
        os.fsync(handle.fileno())


def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-verification", required=True)
    parser.add_argument("--source-archive", required=True)
    parser.add_argument("--recipe", required=True)
    parser.add_argument("--toolchain-receipt", required=True)
    parser.add_argument("--build-input-manifest", required=True)
    parser.add_argument("--rustc", required=True)
    parser.add_argument("--cargo", required=True)
    parser.add_argument("--linker", required=True)
    parser.add_argument("--source-root", required=True)
    parser.add_argument("--artifact-root", required=True)
    parser.add_argument("--output", required=True)
    return parser.parse_args()


def main() -> int:
    if os.name != "posix":
        print("HEPTA_SERVO_BUILD_PREFLIGHT=FAIL: Linux/POSIX preflight only", file=sys.stderr)
        return 1
    try:
        arguments = parse_arguments()
        source_path = canonical_absolute(arguments.source_verification, "source verification")
        archive_path = canonical_absolute(arguments.source_archive, "source archive")
        recipe_path = canonical_absolute(arguments.recipe, "build recipe")
        toolchain_path = canonical_absolute(arguments.toolchain_receipt, "toolchain receipt")
        manifest_path = canonical_absolute(arguments.build_input_manifest, "build-input manifest")
        rustc = canonical_absolute(arguments.rustc, "rustc")
        cargo = canonical_absolute(arguments.cargo, "cargo")
        linker = canonical_absolute(arguments.linker, "linker")
        source_root = canonical_absolute(arguments.source_root, "source root")
        artifact_root = canonical_absolute(arguments.artifact_root, "artifact root")
        output = canonical_absolute(arguments.output, "output", must_exist=False)
        source, source_raw = read_canonical_json(source_path, "source verification receipt")
        recipe, recipe_raw = read_canonical_json(recipe_path, "build recipe")
        toolchain, toolchain_raw = read_canonical_json(toolchain_path, "toolchain receipt")
        manifest, manifest_raw = read_canonical_json(manifest_path, "build-input manifest")
        receipt = run_preflight(
            source=source,
            source_raw=source_raw,
            recipe=recipe,
            recipe_raw=recipe_raw,
            toolchain=toolchain,
            toolchain_raw=toolchain_raw,
            manifest=manifest,
            manifest_raw=manifest_raw,
            source_archive=archive_path,
            rustc=rustc,
            cargo=cargo,
            linker=linker,
            source_root=source_root,
            artifact_root=artifact_root,
        )
        write_atomic(output, receipt)
    except (BuildPreflightError, OSError, UnicodeError) as error:
        print(f"HEPTA_SERVO_BUILD_PREFLIGHT=FAIL: {error}", file=sys.stderr)
        return 1
    print(
        json.dumps(
            {
                "schema": OUTPUT_SCHEMA,
                "status": receipt["status"],
                "target": receipt["build"]["target"],
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
