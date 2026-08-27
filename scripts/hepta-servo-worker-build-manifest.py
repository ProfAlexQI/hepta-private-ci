#!/usr/bin/env python3
"""Create and verify a fail-closed Servo worker build-input packet and manifest.

The tool performs no network access and never builds or executes Servo. It binds
one exact, already-sealed source bundle to a reproducible build command,
allowlisted environment, toolchain identities, patch/license inputs, and an
SPDX-2.3 JSON SBOM. Output remains DEVELOPMENT / build-input-only evidence.
"""
from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
import pathlib
import re
import stat
import sys
from typing import Any, Iterable

SOURCE_SCHEMA = "hepta.servo.source_receipt.v1"
BUNDLE_SCHEMA = "hepta.servo.source_bundle_verification.v1"
PATCH_SCHEMA = "hepta.servo.patch_inventory.v1"
LICENSE_SCHEMA = "hepta.servo.license_packet.v1"
COMMAND_SCHEMA = "hepta.servo.worker_build_command.v1"
ENVIRONMENT_SCHEMA = "hepta.servo.worker_build_environment.v1"
PACKET_SCHEMA = "hepta.servo.worker_build_input_packet.v1"
MANIFEST_SCHEMA = "hepta.servo.worker_build_manifest.v1"

REPOSITORY = "https://github.com/servo/servo"
COMMIT = "0a48e298482659817eb50097df23841f2b8e3044"
TREE = "b04d2f75b3217374d079d579c270177b57fa1389"

PACKET_ID_DOMAIN = b"hepta.servo.worker-build-input-packet.v1"
SOURCE_ID_DOMAIN = b"hepta.servo.source-receipt.v1"
BUNDLE_ID_DOMAIN = b"hepta.servo.source-bundle-verification.v1"

SHA64 = re.compile(r"^[0-9a-f]{64}$")
UTC_SECONDS = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$")
SAFE_TEXT = re.compile(r"^[A-Za-z0-9._:+/@=-]+$")
SAFE_ENV_KEY = re.compile(r"^[A-Z][A-Z0-9_]{0,127}$")
TARGETS = {
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
    "x86_64-pc-windows-msvc",
    "aarch64-pc-windows-msvc",
}
ENVIRONMENT_ALLOWLIST = {
    "AR",
    "CARGO_HOME",
    "CARGO_NET_OFFLINE",
    "CARGO_TARGET_DIR",
    "CC",
    "CFLAGS",
    "CXX",
    "CXXFLAGS",
    "LANG",
    "LC_ALL",
    "LDFLAGS",
    "PATH",
    "PKG_CONFIG_PATH",
    "RANLIB",
    "RUSTFLAGS",
    "RUSTUP_HOME",
    "SOURCE_DATE_EPOCH",
    "TZ",
}

AUTHORITY = {
    "machine_authority": False,
    "runtime_authority": False,
    "production_caller": False,
    "production_writer": False,
    "effect_authority": False,
    "external_effect": False,
    "external_network_allowed": False,
    "credential_export_allowed": False,
    "operator_acceptance": False,
    "promotion": False,
    "release_qualified": False,
}

MANIFEST_NEGATIVE_FLAGS = {
    "network_access_during_build": False,
    "worker_tcp_listener": False,
    "worker_http_surface": False,
    "worker_external_network": False,
    "worker_credential_export": False,
    "worker_production_authority": False,
    "worker_effect_authority": False,
}


class BuildManifestError(RuntimeError):
    """A fail-closed build-input binding error."""


def fail(message: str) -> None:
    raise BuildManifestError(message)


def reject_duplicate_keys(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    output: dict[str, Any] = {}
    for key, value in pairs:
        if key in output:
            fail(f"duplicate JSON key {key!r}")
        output[key] = value
    return output


def canonical(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode(
        "utf-8"
    )


def load_json(
    path: pathlib.Path,
    label: str,
    *,
    canonical_required: bool = True,
) -> tuple[dict[str, Any], bytes]:
    require_private_regular_file(path, label)
    try:
        raw = path.read_bytes()
        value = json.loads(raw, object_pairs_hook=reject_duplicate_keys)
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot decode {label}: {error}")
    if not isinstance(value, dict):
        fail(f"{label} must be one JSON object")
    if canonical_required and raw != canonical(value):
        fail(f"{label} is not compact canonical JSON")
    return value, raw


def require_private_regular_file(path: pathlib.Path, label: str) -> os.stat_result:
    try:
        metadata = path.lstat()
    except OSError as error:
        fail(f"{label} is unavailable: {error}")
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail(f"{label} must be a non-symlink regular file")
    if getattr(metadata, "st_nlink", 1) != 1:
        fail(f"{label} must have exactly one hard link")
    if metadata.st_mode & 0o022:
        fail(f"{label} must not be group/world writable")
    return metadata


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def sha256_file(path: pathlib.Path, label: str) -> tuple[str, int]:
    require_private_regular_file(path, label)
    digest = hashlib.sha256()
    size = 0
    try:
        with path.open("rb") as stream:
            for block in iter(lambda: stream.read(1024 * 1024), b""):
                digest.update(block)
                size += len(block)
    except OSError as error:
        fail(f"cannot hash {label}: {error}")
    if size == 0:
        fail(f"{label} must not be empty")
    return digest.hexdigest(), size


def framed_digest(domain: bytes, fields: Iterable[bytes]) -> str:
    digest = hashlib.sha256()
    digest.update(len(domain).to_bytes(8, "big"))
    digest.update(domain)
    for field in fields:
        digest.update(len(field).to_bytes(8, "big"))
        digest.update(field)
    return digest.hexdigest()


def validate_sha256(value: Any, label: str) -> str:
    if not isinstance(value, str) or not SHA64.fullmatch(value):
        fail(f"{label} must be lowercase SHA-256")
    return value


def validate_timestamp(value: Any) -> str:
    if not isinstance(value, str) or not UTC_SECONDS.fullmatch(value):
        fail("captured_at_utc must use whole-second RFC3339 UTC")
    try:
        dt.datetime.strptime(value, "%Y-%m-%dT%H:%M:%SZ")
    except ValueError as error:
        fail("captured_at_utc is not a real UTC timestamp")
    return value


def timestamp(value: str | None) -> str:
    if value is None:
        value = dt.datetime.now(dt.timezone.utc).replace(microsecond=0).strftime(
            "%Y-%m-%dT%H:%M:%SZ"
        )
    return validate_timestamp(value)


def validate_safe_text(value: Any, label: str, maximum: int = 256) -> str:
    if (
        not isinstance(value, str)
        or not (1 <= len(value) <= maximum)
        or not SAFE_TEXT.fullmatch(value)
    ):
        fail(f"{label} is empty, oversized, or noncanonical")
    return value


def validate_relative_path(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value or "\x00" in value or "\\" in value:
        fail(f"{label} is empty or platform-ambiguous")
    path = pathlib.PurePosixPath(value)
    if path.is_absolute() or any(part in {"", ".", ".."} for part in path.parts):
        fail(f"{label} must be a normalized repository-relative path")
    return value


def write_new(path: pathlib.Path, raw: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    try:
        descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(raw)
            stream.flush()
            os.fsync(stream.fileno())
    except OSError as error:
        try:
            path.unlink(missing_ok=True)
        except OSError:
            pass
        fail(f"cannot create {path}: {error}")


def validate_authority(value: Any, label: str) -> None:
    if value != AUTHORITY:
        fail(f"{label} authority posture is not exactly fail-closed")


def verify_self_bound_receipt(
    value: dict[str, Any],
    prefix: str,
    domain: bytes,
    label: str,
) -> None:
    identifier = value.get("receipt_id")
    if not isinstance(identifier, str) or not identifier.startswith(prefix):
        fail(f"{label} receipt ID prefix is invalid")
    identifier_digest = validate_sha256(identifier.removeprefix(prefix), f"{label} receipt ID")
    without_identifier = dict(value)
    without_identifier.pop("receipt_id")
    expected = framed_digest(domain, [canonical(without_identifier)])
    if identifier_digest != expected:
        fail(f"{label} receipt ID does not bind its payload")


def validate_source_receipt(path: pathlib.Path) -> tuple[dict[str, Any], bytes]:
    value, raw = load_json(path, "Servo source receipt")
    expected_keys = {
        "schema",
        "phase",
        "claim_level",
        "captured_at_utc",
        "source",
        "artifact",
        "authority",
        "decision",
        "receipt_id",
    }
    if set(value) != expected_keys:
        fail("Servo source receipt fields are incomplete or unknown")
    if value.get("schema") != SOURCE_SCHEMA or value.get("phase") != "DEVELOPMENT":
        fail("Servo source receipt schema/phase is invalid")
    if value.get("claim_level") != "SOURCE_PIN_AND_TREE_ONLY":
        fail("Servo source receipt claim level is invalid")
    validate_timestamp(value.get("captured_at_utc"))
    if value.get("decision") != "SOURCE_PIN_VERIFIED_BUILD_NOT_QUALIFIED":
        fail("Servo source receipt decision overclaims")
    source = value.get("source")
    if not isinstance(source, dict):
        fail("Servo source receipt lacks source facts")
    if (
        source.get("repository") != REPOSITORY
        or source.get("commit") != COMMIT
        or source.get("tree") != TREE
        or source.get("clean_worktree") is not True
    ):
        fail("Servo source receipt source binding drifted")
    artifact = value.get("artifact")
    if artifact != {
        "source_archive_created": False,
        "source_archive_sha256": None,
        "worker_artifact_built": False,
        "worker_artifact_sha256": None,
        "sbom_created": False,
    }:
        fail("Servo source receipt overclaims archive/build/SBOM")
    source_authority = value.get("authority")
    if not isinstance(source_authority, dict) or any(item is not False for item in source_authority.values()):
        fail("Servo source receipt contains positive authority")
    verify_self_bound_receipt(
        value,
        "servo-source-receipt:v1:",
        SOURCE_ID_DOMAIN,
        "Servo source",
    )
    return value, raw


def validate_source_bundle(
    path: pathlib.Path,
    source_receipt: dict[str, Any],
    source_receipt_raw: bytes,
) -> tuple[dict[str, Any], bytes]:
    value, raw = load_json(path, "Servo source bundle verification")
    expected_keys = {
        "schema",
        "phase",
        "claim_level",
        "captured_at_utc",
        "source",
        "archive",
        "distribution",
        "artifact",
        "authority",
        "decision",
        "receipt_id",
    }
    if set(value) != expected_keys:
        fail("Servo source bundle fields are incomplete or unknown")
    if value.get("schema") != BUNDLE_SCHEMA or value.get("phase") != "DEVELOPMENT":
        fail("Servo source bundle schema/phase is invalid")
    if value.get("claim_level") != "SOURCE_BUNDLE_RECOMPUTED_ONLY":
        fail("Servo source bundle claim level is invalid")
    validate_timestamp(value.get("captured_at_utc"))
    if value.get("decision") != "SOURCE_BUNDLE_RECOMPUTED_BUILD_AND_RUNTIME_NOT_QUALIFIED":
        fail("Servo source bundle decision overclaims")
    source = value.get("source")
    if not isinstance(source, dict):
        fail("Servo source bundle lacks source binding")
    if source.get("repository") != REPOSITORY or source.get("commit") != COMMIT or source.get("tree") != TREE:
        fail("Servo source bundle pin drifted")
    artifact = value.get("artifact")
    if artifact != {
        "worker_artifact_built": False,
        "sbom_created": False,
        "runtime_qualified": False,
    }:
        fail("Servo source bundle overclaims build/runtime")
    validate_authority(value.get("authority"), "Servo source bundle")
    verify_self_bound_receipt(
        value,
        "servo-source-bundle-verification:v1:",
        BUNDLE_ID_DOMAIN,
        "Servo source bundle",
    )
    if source_receipt.get("source", {}).get("commit") != source.get("commit"):
        fail("source receipt and source bundle commit differ")
    if sha256_bytes(source_receipt_raw) == "0" * 64:
        fail("source receipt digest is impossible")
    return value, raw


def validate_patch_inventory(path: pathlib.Path) -> tuple[dict[str, Any], bytes]:
    value, raw = load_json(path, "Servo patch inventory")
    if set(value) != {"schema", "upstream_commit", "upstream_tree", "patches"}:
        fail("Servo patch inventory fields are incomplete or unknown")
    if value.get("schema") != PATCH_SCHEMA:
        fail("Servo patch inventory schema is unsupported")
    if value.get("upstream_commit") != COMMIT or value.get("upstream_tree") != TREE:
        fail("Servo patch inventory pin drifted")
    patches = value.get("patches")
    if not isinstance(patches, list) or len(patches) > 1024:
        fail("Servo patch inventory must be a bounded array")
    identifiers: list[str] = []
    for item in patches:
        if not isinstance(item, dict):
            fail("Servo patch inventory entry must be an object")
        if set(item) != {
            "id",
            "path",
            "sha256",
            "reason",
            "upstream_reference",
            "deletion_condition",
        }:
            fail("Servo patch inventory entry fields are incomplete or unknown")
        identifiers.append(validate_safe_text(item.get("id"), "patch id", 128))
        validate_relative_path(item.get("path"), "patch path")
        validate_sha256(item.get("sha256"), "patch SHA-256")
        for key in ("reason", "upstream_reference", "deletion_condition"):
            text = item.get(key)
            if not isinstance(text, str) or not (1 <= len(text) <= 2048) or "\x00" in text:
                fail(f"patch {key} is empty, oversized, or contains NUL")
    if identifiers != sorted(set(identifiers)):
        fail("Servo patch inventory IDs must be sorted and unique")
    return value, raw


def validate_license_packet(path: pathlib.Path) -> tuple[dict[str, Any], bytes]:
    value, raw = load_json(path, "Servo license packet")
    expected = {
        "schema",
        "upstream_repository",
        "upstream_commit",
        "primary_license",
        "license_file_sha256",
        "notices",
        "source_offer_required_by_project_policy",
        "legal_review_required_before_binary_distribution",
        "binary_distribution_authorized",
    }
    if set(value) != expected:
        fail("Servo license packet fields are incomplete or unknown")
    if value.get("schema") != LICENSE_SCHEMA:
        fail("Servo license packet schema is unsupported")
    if value.get("upstream_repository") != REPOSITORY or value.get("upstream_commit") != COMMIT:
        fail("Servo license packet source binding drifted")
    if value.get("primary_license") != "MPL-2.0":
        fail("Servo license packet primary license is not MPL-2.0")
    validate_sha256(value.get("license_file_sha256"), "license file SHA-256")
    notices = value.get("notices")
    if not isinstance(notices, list) or not notices or notices != sorted(set(notices)):
        fail("Servo license notices must be sorted, unique, and nonempty")
    for notice in notices:
        if not isinstance(notice, str) or not notice or len(notice) > 512 or "\x00" in notice:
            fail("Servo license notice is invalid")
    if value.get("source_offer_required_by_project_policy") is not True:
        fail("Servo license packet must retain source-distribution obligations")
    if value.get("legal_review_required_before_binary_distribution") is not True:
        fail("Servo license packet must retain legal review gate")
    if value.get("binary_distribution_authorized") is not False:
        fail("Servo license packet cannot authorize binary distribution")
    return value, raw


def validate_sbom(path: pathlib.Path) -> tuple[dict[str, Any], bytes]:
    value, raw = load_json(path, "Servo worker SPDX SBOM")
    if value.get("spdxVersion") != "SPDX-2.3" or value.get("dataLicense") != "CC0-1.0":
        fail("Servo worker SBOM must be SPDX-2.3 JSON with CC0-1.0 dataLicense")
    packages = value.get("packages")
    if not isinstance(packages, list) or not packages:
        fail("Servo worker SBOM must contain at least one package")
    names: list[str] = []
    for package in packages:
        if not isinstance(package, dict):
            fail("Servo worker SBOM package must be an object")
        name = package.get("name")
        if not isinstance(name, str) or not name or len(name) > 512 or "\x00" in name:
            fail("Servo worker SBOM package name is invalid")
        names.append(name)
    if names != sorted(names):
        fail("Servo worker SBOM packages must be sorted by name")
    return value, raw


def validate_build_command(path: pathlib.Path) -> tuple[dict[str, Any], bytes]:
    value, raw = load_json(path, "Servo worker build command")
    if set(value) != {"schema", "working_directory", "argv", "network_access_during_build"}:
        fail("build command fields are incomplete or unknown")
    if value.get("schema") != COMMAND_SCHEMA:
        fail("build command schema is unsupported")
    validate_relative_path(value.get("working_directory"), "build working directory")
    argv = value.get("argv")
    if not isinstance(argv, list) or not (2 <= len(argv) <= 256):
        fail("build command argv must contain 2..256 entries")
    for index, item in enumerate(argv):
        if not isinstance(item, str) or not item or len(item) > 4096 or "\x00" in item:
            fail(f"build command argv[{index}] is invalid")
        if item.startswith(('/', '~')) or "\\" in item:
            fail("build command must not embed machine-local absolute or Windows paths")
    if value.get("network_access_during_build") is not False:
        fail("build command must keep network_access_during_build=false")
    return value, raw


def validate_environment(path: pathlib.Path) -> tuple[dict[str, Any], bytes, list[dict[str, Any]]]:
    value, raw = load_json(path, "Servo worker build environment")
    if set(value) != {"schema", "variables", "network_access_during_build"}:
        fail("build environment fields are incomplete or unknown")
    if value.get("schema") != ENVIRONMENT_SCHEMA:
        fail("build environment schema is unsupported")
    variables = value.get("variables")
    if not isinstance(variables, dict) or not variables:
        fail("build environment variables must be a nonempty object")
    if list(variables) != sorted(variables):
        fail("build environment variables must be lexically ordered")
    records: list[dict[str, Any]] = []
    for key, raw_value in variables.items():
        if not SAFE_ENV_KEY.fullmatch(key) or key not in ENVIRONMENT_ALLOWLIST:
            fail(f"build environment key {key!r} is not allowlisted")
        if not isinstance(raw_value, str) or len(raw_value) > 16_384 or "\x00" in raw_value:
            fail(f"build environment value for {key} is invalid")
        if key == "CARGO_NET_OFFLINE" and raw_value not in {"true", "1"}:
            fail("CARGO_NET_OFFLINE must be true")
        records.append(
            {
                "key": key,
                "value_sha256": sha256_bytes(raw_value.encode("utf-8")),
                "utf8_bytes": len(raw_value.encode("utf-8")),
            }
        )
    if value.get("network_access_during_build") is not False:
        fail("build environment must keep network_access_during_build=false")
    return value, raw, records


def load_tool_text(path: pathlib.Path, label: str, maximum: int = 64 * 1024) -> tuple[str, str, int]:
    digest, size = sha256_file(path, label)
    if size > maximum:
        fail(f"{label} exceeds {maximum} bytes")
    try:
        text = path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        fail(f"cannot decode {label}: {error}")
    if "\x00" in text or not text.strip():
        fail(f"{label} is empty or contains NUL")
    return text.strip(), digest, size


def create_packet_and_manifest(arguments: argparse.Namespace) -> tuple[dict[str, Any], dict[str, Any]]:
    source, source_raw = validate_source_receipt(arguments.source_receipt)
    bundle, bundle_raw = validate_source_bundle(arguments.source_bundle, source, source_raw)
    _, patch_raw = validate_patch_inventory(arguments.patch_inventory)
    _, license_raw = validate_license_packet(arguments.license_packet)
    _, sbom_raw = validate_sbom(arguments.sbom)
    command, command_raw = validate_build_command(arguments.build_command)
    _, environment_raw, environment_records = validate_environment(arguments.environment)

    rustc_text, rustc_sha, rustc_bytes = load_tool_text(arguments.rustc_verbose, "rustc verbose output")
    cargo_text, cargo_sha, cargo_bytes = load_tool_text(arguments.cargo_version, "cargo version output")
    linker_text, linker_sha, linker_bytes = load_tool_text(arguments.linker_id, "linker identity output")

    target = validate_safe_text(arguments.target_triple, "target triple")
    if target not in TARGETS:
        fail("target triple is outside the C1 build allowlist")
    profile = validate_safe_text(arguments.build_profile, "build profile")
    features = sorted(set(arguments.feature or []))
    if len(features) > 128:
        fail("build feature list exceeds 128 entries")
    for feature in features:
        validate_safe_text(feature, "build feature", 128)

    cargo_version = cargo_text.splitlines()[0].strip()
    linker_id = linker_text.splitlines()[0].strip()
    validate_safe_text(cargo_version, "cargo version", 256)
    validate_safe_text(linker_id, "linker identity", 256)

    source_sha = sha256_bytes(source_raw)
    bundle_sha = sha256_bytes(bundle_raw)
    patch_sha = sha256_bytes(patch_raw)
    license_sha = sha256_bytes(license_raw)
    sbom_sha = sha256_bytes(sbom_raw)
    command_sha = sha256_bytes(command_raw)
    environment_sha = sha256_bytes(environment_raw)

    packet: dict[str, Any] = {
        "schema": PACKET_SCHEMA,
        "phase": "DEVELOPMENT",
        "claim_level": "WORKER_BUILD_INPUTS_FROZEN_ONLY",
        "captured_at_utc": timestamp(arguments.captured_at),
        "source_binding": {
            "repository": REPOSITORY,
            "commit": COMMIT,
            "tree": TREE,
            "source_receipt_id": source["receipt_id"],
            "source_receipt_sha256": source_sha,
            "source_bundle_receipt_id": bundle["receipt_id"],
            "source_bundle_sha256": bundle_sha,
        },
        "distribution_binding": {
            "patch_inventory_sha256": patch_sha,
            "license_packet_sha256": license_sha,
            "sbom_sha256": sbom_sha,
            "sbom_format": "SPDX-2.3-json",
        },
        "toolchain_binding": {
            "rustc_verbose_sha256": rustc_sha,
            "rustc_verbose_bytes": rustc_bytes,
            "cargo_version": cargo_version,
            "cargo_version_sha256": cargo_sha,
            "cargo_version_bytes": cargo_bytes,
            "linker_id": linker_id,
            "linker_id_sha256": linker_sha,
            "linker_id_bytes": linker_bytes,
        },
        "build_binding": {
            "target_triple": target,
            "build_profile": profile,
            "features": features,
            "working_directory": command["working_directory"],
            "build_command_sha256": command_sha,
            "build_command_argc": len(command["argv"]),
            "environment_allowlist_sha256": environment_sha,
            "environment_keys": [record["key"] for record in environment_records],
            "environment_values": environment_records,
            "network_access_during_build": False,
        },
        "artifact": {
            "worker_artifact_built": False,
            "worker_artifact_sha256": None,
            "runtime_qualified": False,
            "reproducible_build_verified": False,
        },
        "authority": AUTHORITY,
        "decision": "BUILD_INPUTS_FROZEN_ARTIFACT_AND_RUNTIME_NOT_QUALIFIED",
    }
    packet["receipt_id"] = "servo-worker-build-inputs:v1:" + framed_digest(
        PACKET_ID_DOMAIN, [canonical(packet)]
    )

    manifest = {
        "schema": MANIFEST_SCHEMA,
        "source_receipt_id": source["receipt_id"],
        "source_receipt_sha256": source_sha,
        "target_triple": target,
        "build_profile": profile,
        "rustc_verbose_sha256": rustc_sha,
        "cargo_version": cargo_version,
        "linker_id": linker_id,
        "features": features,
        "build_command_sha256": command_sha,
        "environment_allowlist_sha256": environment_sha,
        "patch_inventory_sha256": patch_sha,
        "license_packet_sha256": license_sha,
        "sbom_sha256": sbom_sha,
        "sbom_format": "SPDX-2.3-json",
        **MANIFEST_NEGATIVE_FLAGS,
    }
    return packet, manifest


def validate_packet(
    packet: dict[str, Any],
    packet_raw: bytes,
    manifest: dict[str, Any],
    manifest_raw: bytes,
) -> None:
    expected_packet_keys = {
        "schema",
        "phase",
        "claim_level",
        "captured_at_utc",
        "source_binding",
        "distribution_binding",
        "toolchain_binding",
        "build_binding",
        "artifact",
        "authority",
        "decision",
        "receipt_id",
    }
    if set(packet) != expected_packet_keys:
        fail("build input packet fields are incomplete or unknown")
    if packet.get("schema") != PACKET_SCHEMA or packet.get("phase") != "DEVELOPMENT":
        fail("build input packet schema/phase is invalid")
    if packet.get("claim_level") != "WORKER_BUILD_INPUTS_FROZEN_ONLY":
        fail("build input packet claim level is invalid")
    validate_timestamp(packet.get("captured_at_utc"))
    validate_authority(packet.get("authority"), "build input packet")
    if packet.get("artifact") != {
        "worker_artifact_built": False,
        "worker_artifact_sha256": None,
        "runtime_qualified": False,
        "reproducible_build_verified": False,
    }:
        fail("build input packet overclaims artifact/runtime/reproducibility")
    if packet.get("decision") != "BUILD_INPUTS_FROZEN_ARTIFACT_AND_RUNTIME_NOT_QUALIFIED":
        fail("build input packet decision overclaims")
    verify_self_bound_receipt(
        packet,
        "servo-worker-build-inputs:v1:",
        PACKET_ID_DOMAIN,
        "build input packet",
    )

    build = packet.get("build_binding")
    source = packet.get("source_binding")
    distribution = packet.get("distribution_binding")
    toolchain = packet.get("toolchain_binding")
    if not all(isinstance(item, dict) for item in (build, source, distribution, toolchain)):
        fail("build input packet bindings are incomplete")
    if source.get("repository") != REPOSITORY or source.get("commit") != COMMIT or source.get("tree") != TREE:
        fail("build input packet source pin drifted")
    if build.get("network_access_during_build") is not False:
        fail("build input packet attempted to enable build network")
    if build.get("features") != sorted(set(build.get("features", []))):
        fail("build input packet features are not sorted and unique")
    environment_values = build.get("environment_values")
    if not isinstance(environment_values, list):
        fail("build input packet environment values are missing")
    keys = [record.get("key") for record in environment_values if isinstance(record, dict)]
    if keys != build.get("environment_keys") or keys != sorted(set(keys)):
        fail("build input packet environment records drifted")
    for record in environment_values:
        if not isinstance(record, dict) or set(record) != {"key", "value_sha256", "utf8_bytes"}:
            fail("build input packet environment record is invalid")
        if record["key"] not in ENVIRONMENT_ALLOWLIST:
            fail("build input packet environment record is not allowlisted")
        validate_sha256(record["value_sha256"], "environment value SHA-256")
        if not isinstance(record["utf8_bytes"], int) or record["utf8_bytes"] < 0:
            fail("build input packet environment value length is invalid")

    expected_manifest_keys = {
        "schema",
        "source_receipt_id",
        "source_receipt_sha256",
        "target_triple",
        "build_profile",
        "rustc_verbose_sha256",
        "cargo_version",
        "linker_id",
        "features",
        "build_command_sha256",
        "environment_allowlist_sha256",
        "patch_inventory_sha256",
        "license_packet_sha256",
        "sbom_sha256",
        "sbom_format",
        *MANIFEST_NEGATIVE_FLAGS,
    }
    if set(manifest) != expected_manifest_keys:
        fail("worker build manifest fields are incomplete or unknown")
    if manifest.get("schema") != MANIFEST_SCHEMA:
        fail("worker build manifest schema is invalid")
    for key, expected in MANIFEST_NEGATIVE_FLAGS.items():
        if manifest.get(key) is not expected:
            fail(f"worker build manifest must keep {key}=false")
    if manifest.get("features") != sorted(set(manifest.get("features", []))):
        fail("worker build manifest features are not sorted and unique")
    if manifest.get("source_receipt_id") != source.get("source_receipt_id"):
        fail("worker build manifest source receipt ID drifted")
    if manifest.get("source_receipt_sha256") != source.get("source_receipt_sha256"):
        fail("worker build manifest source receipt digest drifted")
    comparisons = {
        "target_triple": build.get("target_triple"),
        "build_profile": build.get("build_profile"),
        "rustc_verbose_sha256": toolchain.get("rustc_verbose_sha256"),
        "cargo_version": toolchain.get("cargo_version"),
        "linker_id": toolchain.get("linker_id"),
        "features": build.get("features"),
        "build_command_sha256": build.get("build_command_sha256"),
        "environment_allowlist_sha256": build.get("environment_allowlist_sha256"),
        "patch_inventory_sha256": distribution.get("patch_inventory_sha256"),
        "license_packet_sha256": distribution.get("license_packet_sha256"),
        "sbom_sha256": distribution.get("sbom_sha256"),
        "sbom_format": distribution.get("sbom_format"),
    }
    for key, expected in comparisons.items():
        if manifest.get(key) != expected:
            fail(f"worker build manifest {key} differs from build input packet")
    if sha256_bytes(packet_raw) == "0" * 64 or sha256_bytes(manifest_raw) == "0" * 64:
        fail("build input packet or manifest digest is impossible")


def create(arguments: argparse.Namespace) -> dict[str, Any]:
    packet, manifest = create_packet_and_manifest(arguments)
    packet_raw = canonical(packet)
    manifest_raw = canonical(manifest)
    write_new(arguments.output_packet, packet_raw)
    try:
        write_new(arguments.output_manifest, manifest_raw)
    except Exception:
        arguments.output_packet.unlink(missing_ok=True)
        raise
    validate_packet(packet, packet_raw, manifest, manifest_raw)
    return {
        "schema": "hepta.servo.worker_build_manifest_creation.v1",
        "status": "BUILD_INPUTS_FROZEN_ARTIFACT_AND_RUNTIME_NOT_QUALIFIED",
        "packet_receipt_id": packet["receipt_id"],
        "packet_sha256": sha256_bytes(packet_raw),
        "manifest_sha256": sha256_bytes(manifest_raw),
        "worker_artifact_built": False,
        "runtime_qualified": False,
        "authority": AUTHORITY,
    }


def verify(arguments: argparse.Namespace) -> dict[str, Any]:
    packet, packet_raw = load_json(arguments.packet, "Servo worker build input packet")
    manifest, manifest_raw = load_json(arguments.manifest, "Servo worker build manifest")
    validate_packet(packet, packet_raw, manifest, manifest_raw)

    expected_packet, expected_manifest = create_packet_and_manifest(arguments)
    expected_packet["captured_at_utc"] = packet["captured_at_utc"]
    expected_packet.pop("receipt_id")
    expected_packet["receipt_id"] = "servo-worker-build-inputs:v1:" + framed_digest(
        PACKET_ID_DOMAIN, [canonical({k: v for k, v in expected_packet.items() if k != "receipt_id"})]
    )
    if packet != expected_packet:
        fail("build input packet does not recompute from the exact supporting inputs")
    if manifest != expected_manifest:
        fail("worker build manifest does not recompute from the exact supporting inputs")
    return {
        "schema": "hepta.servo.worker_build_manifest_verification.v1",
        "status": "BUILD_INPUTS_RECOMPUTED_ARTIFACT_AND_RUNTIME_NOT_QUALIFIED",
        "packet_receipt_id": packet["receipt_id"],
        "packet_sha256": sha256_bytes(packet_raw),
        "manifest_sha256": sha256_bytes(manifest_raw),
        "worker_artifact_built": False,
        "runtime_qualified": False,
        "authority": AUTHORITY,
    }


def add_common(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--source-receipt", type=pathlib.Path, required=True)
    parser.add_argument("--source-bundle", type=pathlib.Path, required=True)
    parser.add_argument("--patch-inventory", type=pathlib.Path, required=True)
    parser.add_argument("--license-packet", type=pathlib.Path, required=True)
    parser.add_argument("--sbom", type=pathlib.Path, required=True)
    parser.add_argument("--rustc-verbose", type=pathlib.Path, required=True)
    parser.add_argument("--cargo-version", type=pathlib.Path, required=True)
    parser.add_argument("--linker-id", type=pathlib.Path, required=True)
    parser.add_argument("--build-command", type=pathlib.Path, required=True)
    parser.add_argument("--environment", type=pathlib.Path, required=True)
    parser.add_argument("--target-triple", required=True)
    parser.add_argument("--build-profile", required=True)
    parser.add_argument("--feature", action="append", default=[])


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)
    create_parser = subparsers.add_parser("create")
    add_common(create_parser)
    create_parser.add_argument("--captured-at")
    create_parser.add_argument("--output-packet", type=pathlib.Path, required=True)
    create_parser.add_argument("--output-manifest", type=pathlib.Path, required=True)

    verify_parser = subparsers.add_parser("verify")
    add_common(verify_parser)
    verify_parser.add_argument("--packet", type=pathlib.Path, required=True)
    verify_parser.add_argument("--manifest", type=pathlib.Path, required=True)
    verify_parser.set_defaults(captured_at=None)
    return parser


def main(argv: list[str] | None = None) -> int:
    arguments = build_parser().parse_args(argv)
    try:
        result = create(arguments) if arguments.command == "create" else verify(arguments)
    except (BuildManifestError, OSError) as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    sys.exit(main())
