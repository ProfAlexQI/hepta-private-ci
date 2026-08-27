#!/usr/bin/env python3
"""Bind a Servo worker artifact to exact source/build/SBOM/license inputs without executing it."""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
import pathlib
import re
import stat
import struct
import sys
from typing import Iterable

RECEIPT_SCHEMA = "hepta.servo.worker_artifact_receipt.v1"
BUILD_SCHEMA = "hepta.servo.worker_build_manifest.v1"
SOURCE_SCHEMA = "hepta.servo.source_receipt.v1"
PATCH_SCHEMA = "hepta.servo.patch_inventory.v1"
LICENSE_SCHEMA = "hepta.servo.license_packet.v1"
EXPECTED_SERVO_COMMIT = "0a48e298482659817eb50097df23841f2b8e3044"
EXPECTED_SERVO_TREE = "b04d2f75b3217374d079d579c270177b57fa1389"
EXPECTED_REPOSITORY = "https://github.com/servo/servo"
SOURCE_RECEIPT_DOMAIN = b"hepta.servo.source-receipt.v1"
ARTIFACT_RECEIPT_DOMAIN = b"hepta.servo.worker-artifact-receipt.v1"
SHA40 = re.compile(r"^[0-9a-f]{40}$")
SHA64 = re.compile(r"^[0-9a-f]{64}$")
UTC_SECONDS = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$")
SAFE_TEXT = re.compile(r"^[A-Za-z0-9._:+/@=-]+$")

NEGATIVE_AUTHORITY_KEYS = {
    "machine_authority",
    "runtime_authority",
    "production_caller",
    "production_writer",
    "effect_authority",
    "external_effect",
    "external_network_allowed",
    "credential_export_allowed",
    "operator_acceptance",
    "promotion",
    "release_qualified",
}

BUILD_KEYS = {
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
    "network_access_during_build",
    "worker_tcp_listener",
    "worker_http_surface",
    "worker_external_network",
    "worker_credential_export",
    "worker_production_authority",
    "worker_effect_authority",
}


class ArtifactError(RuntimeError):
    """A fail-closed artifact binding error."""


def canonical_bytes(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode(
        "utf-8"
    )


def reject_duplicate_keys(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise ArtifactError(f"duplicate JSON key {key!r}")
        result[key] = value
    return result


def load_canonical_json(path: pathlib.Path, description: str) -> tuple[dict[str, object], bytes]:
    try:
        raw = path.read_bytes()
        value = json.loads(raw, object_pairs_hook=reject_duplicate_keys)
    except (OSError, json.JSONDecodeError) as error:
        raise ArtifactError(f"cannot decode {description}: {error}") from error
    if not isinstance(value, dict):
        raise ArtifactError(f"{description} must be a JSON object")
    if canonical_bytes(value) != raw:
        raise ArtifactError(f"{description} is not compact canonical JSON")
    return value, raw


def framed_digest(domain: bytes, fields: Iterable[bytes]) -> str:
    digest = hashlib.sha256()
    digest.update(len(domain).to_bytes(8, "big"))
    digest.update(domain)
    for field in fields:
        digest.update(len(field).to_bytes(8, "big"))
        digest.update(field)
    return digest.hexdigest()


def sha256_file(path: pathlib.Path) -> tuple[str, int]:
    digest = hashlib.sha256()
    length = 0
    try:
        with path.open("rb") as source:
            while True:
                chunk = source.read(1024 * 1024)
                if not chunk:
                    break
                digest.update(chunk)
                length += len(chunk)
    except OSError as error:
        raise ArtifactError(f"cannot hash {path}: {error}") from error
    return digest.hexdigest(), length


def validate_timestamp(value: object) -> str:
    if not isinstance(value, str) or not UTC_SECONDS.fullmatch(value):
        raise ArtifactError("captured_at_utc must use whole-second RFC3339 UTC")
    try:
        dt.datetime.strptime(value, "%Y-%m-%dT%H:%M:%SZ")
    except ValueError as error:
        raise ArtifactError("captured_at_utc is not a real UTC timestamp") from error
    return value


def validate_sha256(value: object, label: str) -> str:
    if not isinstance(value, str) or not SHA64.fullmatch(value):
        raise ArtifactError(f"{label} must be lowercase SHA-256")
    return value


def validate_safe_text(value: object, label: str, maximum: int = 256) -> str:
    if (
        not isinstance(value, str)
        or not (1 <= len(value) <= maximum)
        or not SAFE_TEXT.fullmatch(value)
    ):
        raise ArtifactError(f"{label} is empty, oversized, or noncanonical")
    return value


def validate_source_receipt(path: pathlib.Path) -> tuple[dict[str, object], str]:
    receipt, raw = load_canonical_json(path, "Servo source receipt")
    required = {
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
    if set(receipt) != required:
        raise ArtifactError("Servo source receipt field set is incomplete or unknown")
    if receipt.get("schema") != SOURCE_SCHEMA:
        raise ArtifactError("Servo source receipt schema is unsupported")
    if receipt.get("phase") != "DEVELOPMENT":
        raise ArtifactError("Servo source receipt phase is not DEVELOPMENT")
    if receipt.get("claim_level") != "SOURCE_PIN_AND_TREE_ONLY":
        raise ArtifactError("Servo source receipt claim level is invalid")
    validate_timestamp(receipt.get("captured_at_utc"))
    if receipt.get("decision") != "SOURCE_PIN_VERIFIED_BUILD_NOT_QUALIFIED":
        raise ArtifactError("Servo source receipt decision overclaims or is unknown")

    source = receipt.get("source")
    if not isinstance(source, dict):
        raise ArtifactError("Servo source receipt lacks source facts")
    if source.get("repository") != EXPECTED_REPOSITORY:
        raise ArtifactError("Servo source receipt repository is unexpected")
    if source.get("commit") != EXPECTED_SERVO_COMMIT or source.get("tree") != EXPECTED_SERVO_TREE:
        raise ArtifactError("Servo source receipt pin drifted")
    if source.get("clean_worktree") is not True:
        raise ArtifactError("Servo source receipt did not bind a clean checkout")

    artifact = receipt.get("artifact")
    expected_artifact = {
        "source_archive_created": False,
        "source_archive_sha256": None,
        "worker_artifact_built": False,
        "worker_artifact_sha256": None,
        "sbom_created": False,
    }
    if artifact != expected_artifact:
        raise ArtifactError("Servo source receipt overclaims an archive/build/SBOM")

    authority = receipt.get("authority")
    if not isinstance(authority, dict) or any(value is not False for value in authority.values()):
        raise ArtifactError("Servo source receipt contains positive authority")

    receipt_id = receipt.get("receipt_id")
    if not isinstance(receipt_id, str) or not receipt_id.startswith("servo-source-receipt:v1:"):
        raise ArtifactError("Servo source receipt ID is invalid")
    identifier_digest = receipt_id.removeprefix("servo-source-receipt:v1:")
    validate_sha256(identifier_digest, "Servo source receipt ID")
    without_id = dict(receipt)
    without_id.pop("receipt_id")
    expected_id = framed_digest(SOURCE_RECEIPT_DOMAIN, [canonical_bytes(without_id)])
    if identifier_digest != expected_id:
        raise ArtifactError("Servo source receipt ID does not match its payload")
    return receipt, hashlib.sha256(raw).hexdigest()


def validate_build_manifest(
    path: pathlib.Path,
    source_receipt: dict[str, object],
    source_receipt_sha256: str,
    patch_sha256: str,
    license_sha256: str,
    sbom_sha256: str,
) -> tuple[dict[str, object], str]:
    manifest, raw = load_canonical_json(path, "Servo worker build manifest")
    if set(manifest) != BUILD_KEYS:
        raise ArtifactError("build manifest field set is incomplete or unknown")
    if manifest.get("schema") != BUILD_SCHEMA:
        raise ArtifactError("build manifest schema is unsupported")
    if manifest.get("source_receipt_id") != source_receipt.get("receipt_id"):
        raise ArtifactError("build manifest source receipt ID does not match")
    if manifest.get("source_receipt_sha256") != source_receipt_sha256:
        raise ArtifactError("build manifest source receipt SHA-256 does not match")
    for key, expected in (
        ("patch_inventory_sha256", patch_sha256),
        ("license_packet_sha256", license_sha256),
        ("sbom_sha256", sbom_sha256),
    ):
        if manifest.get(key) != expected:
            raise ArtifactError(f"build manifest {key} does not match its exact input")

    for field in (
        "rustc_verbose_sha256",
        "build_command_sha256",
        "environment_allowlist_sha256",
        "patch_inventory_sha256",
        "license_packet_sha256",
        "sbom_sha256",
    ):
        validate_sha256(manifest.get(field), field)
    for field in ("target_triple", "build_profile", "cargo_version", "linker_id", "sbom_format"):
        validate_safe_text(manifest.get(field), field)

    features = manifest.get("features")
    if not isinstance(features, list) or len(features) > 128:
        raise ArtifactError("build manifest features must be a bounded array")
    if any(not isinstance(item, str) for item in features):
        raise ArtifactError("build manifest feature is not a string")
    normalized_features = [validate_safe_text(item, "feature", 128) for item in features]
    if normalized_features != sorted(set(normalized_features)):
        raise ArtifactError("build manifest features must be sorted and unique")

    negative_booleans = (
        "network_access_during_build",
        "worker_tcp_listener",
        "worker_http_surface",
        "worker_external_network",
        "worker_credential_export",
        "worker_production_authority",
        "worker_effect_authority",
    )
    for field in negative_booleans:
        if manifest.get(field) is not False:
            raise ArtifactError(f"build manifest must keep {field}=false")
    return manifest, hashlib.sha256(raw).hexdigest()


def validate_patch_inventory(path: pathlib.Path) -> tuple[dict[str, object], str]:
    inventory, raw = load_canonical_json(path, "Servo patch inventory")
    if set(inventory) != {"schema", "upstream_commit", "upstream_tree", "patches"}:
        raise ArtifactError("patch inventory field set is incomplete or unknown")
    if inventory.get("schema") != PATCH_SCHEMA:
        raise ArtifactError("patch inventory schema is unsupported")
    if inventory.get("upstream_commit") != EXPECTED_SERVO_COMMIT:
        raise ArtifactError("patch inventory upstream commit drifted")
    if inventory.get("upstream_tree") != EXPECTED_SERVO_TREE:
        raise ArtifactError("patch inventory upstream tree drifted")
    patches = inventory.get("patches")
    if not isinstance(patches, list) or len(patches) > 1024:
        raise ArtifactError("patch inventory patches must be a bounded array")
    identifiers: list[str] = []
    for patch in patches:
        if not isinstance(patch, dict):
            raise ArtifactError("patch inventory entry must be an object")
        expected_keys = {
            "id",
            "path",
            "sha256",
            "reason",
            "upstream_reference",
            "deletion_condition",
        }
        if set(patch) != expected_keys:
            raise ArtifactError("patch inventory entry field set is incomplete or unknown")
        identifier = validate_safe_text(patch.get("id"), "patch id", 128)
        identifiers.append(identifier)
        validate_safe_text(patch.get("path"), "patch path", 512)
        validate_sha256(patch.get("sha256"), "patch SHA-256")
        for field in ("reason", "upstream_reference", "deletion_condition"):
            value = patch.get(field)
            if not isinstance(value, str) or not (1 <= len(value) <= 2048) or "\x00" in value:
                raise ArtifactError(f"patch {field} is empty, oversized, or contains NUL")
    if identifiers != sorted(set(identifiers)):
        raise ArtifactError("patch inventory IDs must be sorted and unique")
    return inventory, hashlib.sha256(raw).hexdigest()


def validate_license_packet(path: pathlib.Path) -> tuple[dict[str, object], str]:
    packet, raw = load_canonical_json(path, "Servo license packet")
    expected_keys = {
        "schema",
        "upstream_repository",
        "upstream_commit",
        "primary_license",
        "license_file_sha256",
        "notices",
        "source_offer_required",
    }
    if set(packet) != expected_keys:
        raise ArtifactError("license packet field set is incomplete or unknown")
    if packet.get("schema") != LICENSE_SCHEMA:
        raise ArtifactError("license packet schema is unsupported")
    if packet.get("upstream_repository") != EXPECTED_REPOSITORY:
        raise ArtifactError("license packet repository is unexpected")
    if packet.get("upstream_commit") != EXPECTED_SERVO_COMMIT:
        raise ArtifactError("license packet commit drifted")
    if packet.get("primary_license") != "MPL-2.0":
        raise ArtifactError("license packet primary license is not MPL-2.0")
    validate_sha256(packet.get("license_file_sha256"), "license file SHA-256")
    if packet.get("source_offer_required") is not True:
        raise ArtifactError("license packet must acknowledge source-distribution obligations")
    notices = packet.get("notices")
    if not isinstance(notices, list) or not notices:
        raise ArtifactError("license packet notices must be a nonempty array")
    normalized = [validate_safe_text(item, "license notice", 512) for item in notices]
    if normalized != sorted(set(normalized)):
        raise ArtifactError("license packet notices must be sorted and unique")
    return packet, hashlib.sha256(raw).hexdigest()


def validate_sbom(path: pathlib.Path) -> tuple[dict[str, object], str, int]:
    sbom, raw = load_canonical_json(path, "Servo worker SBOM")
    if sbom.get("spdxVersion") != "SPDX-2.3":
        raise ArtifactError("SBOM must be SPDX-2.3 JSON")
    if sbom.get("dataLicense") != "CC0-1.0":
        raise ArtifactError("SBOM dataLicense must be CC0-1.0")
    packages = sbom.get("packages")
    if not isinstance(packages, list) or not packages:
        raise ArtifactError("SBOM packages must be a nonempty array")
    document_namespace = sbom.get("documentNamespace")
    if not isinstance(document_namespace, str) or not document_namespace.startswith("urn:uuid:"):
        raise ArtifactError("SBOM documentNamespace must be a UUID URN")
    return sbom, hashlib.sha256(raw).hexdigest(), len(raw)


def inspect_artifact(path: pathlib.Path, target_triple: str) -> dict[str, object]:
    try:
        metadata = path.lstat()
    except OSError as error:
        raise ArtifactError(f"cannot inspect worker artifact: {error}") from error
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        raise ArtifactError("worker artifact must be a non-symlink regular file")
    if getattr(metadata, "st_nlink", 1) != 1:
        raise ArtifactError("worker artifact must have exactly one hard link")
    mode = stat.S_IMODE(metadata.st_mode)
    if mode & 0o022:
        raise ArtifactError("worker artifact must not be group/world writable")
    digest, length = sha256_file(path)
    if length < 64:
        raise ArtifactError("worker artifact is too small to contain a supported executable header")
    with path.open("rb") as source:
        header = source.read(4096)
    binary = identify_binary(header)
    expected = expected_binary_for_target(target_triple)
    if binary["format"] != expected["format"] or binary["architecture"] != expected["architecture"]:
        raise ArtifactError(
            f"worker binary {binary['format']}/{binary['architecture']} does not match target "
            f"{target_triple} ({expected['format']}/{expected['architecture']})"
        )
    return {
        "sha256": digest,
        "bytes": length,
        "mode": f"0o{mode:04o}",
        "format": binary["format"],
        "architecture": binary["architecture"],
        "target_triple": target_triple,
    }


def identify_binary(header: bytes) -> dict[str, str]:
    if header.startswith(b"\x7fELF"):
        if len(header) < 20:
            raise ArtifactError("ELF header is truncated")
        elf_class = header[4]
        endian = header[5]
        if elf_class not in {1, 2} or endian not in {1, 2}:
            raise ArtifactError("ELF class or endianness is unsupported")
        machine = int.from_bytes(header[18:20], "little" if endian == 1 else "big")
        architecture = {62: "x86_64", 183: "aarch64"}.get(machine)
        if architecture is None:
            raise ArtifactError(f"ELF machine {machine} is unsupported")
        return {"format": "elf", "architecture": architecture}

    if len(header) >= 8:
        magic = int.from_bytes(header[0:4], "little")
        macho_little = {0xFEEDFACE, 0xFEEDFACF}
        macho_big = {0xCEFAEDFE, 0xCFFAEDFE}
        if magic in macho_little | macho_big:
            byteorder = "little" if magic in macho_little else "big"
            cpu_type = int.from_bytes(header[4:8], byteorder)
            architecture = {0x01000007: "x86_64", 0x0100000C: "aarch64"}.get(cpu_type)
            if architecture is None:
                raise ArtifactError(f"Mach-O CPU type {cpu_type:#x} is unsupported")
            return {"format": "macho", "architecture": architecture}

    if header.startswith(b"MZ"):
        if len(header) < 0x40:
            raise ArtifactError("PE DOS header is truncated")
        pe_offset = struct.unpack_from("<I", header, 0x3C)[0]
        if pe_offset + 6 > len(header) or header[pe_offset : pe_offset + 4] != b"PE\0\0":
            raise ArtifactError("PE header is missing or outside the inspection bound")
        machine = struct.unpack_from("<H", header, pe_offset + 4)[0]
        architecture = {0x8664: "x86_64", 0xAA64: "aarch64"}.get(machine)
        if architecture is None:
            raise ArtifactError(f"PE machine {machine:#x} is unsupported")
        return {"format": "pe", "architecture": architecture}

    raise ArtifactError("worker artifact format is not supported ELF, Mach-O, or PE")


def expected_binary_for_target(target: str) -> dict[str, str]:
    mappings = {
        "x86_64-unknown-linux-gnu": {"format": "elf", "architecture": "x86_64"},
        "aarch64-unknown-linux-gnu": {"format": "elf", "architecture": "aarch64"},
        "x86_64-apple-darwin": {"format": "macho", "architecture": "x86_64"},
        "aarch64-apple-darwin": {"format": "macho", "architecture": "aarch64"},
        "x86_64-pc-windows-msvc": {"format": "pe", "architecture": "x86_64"},
        "aarch64-pc-windows-msvc": {"format": "pe", "architecture": "aarch64"},
    }
    try:
        return mappings[target]
    except KeyError as error:
        raise ArtifactError(f"target triple is not in the C1 artifact allowlist: {target}") from error


def artifact_receipt(
    *,
    source_receipt_path: pathlib.Path,
    build_manifest_path: pathlib.Path,
    artifact_path: pathlib.Path,
    patch_inventory_path: pathlib.Path,
    license_packet_path: pathlib.Path,
    sbom_path: pathlib.Path,
    captured_at: str,
) -> dict[str, object]:
    validate_timestamp(captured_at)
    source_receipt, source_receipt_sha256 = validate_source_receipt(source_receipt_path)
    patch_inventory, patch_sha256 = validate_patch_inventory(patch_inventory_path)
    license_packet, license_sha256 = validate_license_packet(license_packet_path)
    sbom, sbom_sha256, sbom_bytes = validate_sbom(sbom_path)
    build_manifest, build_manifest_sha256 = validate_build_manifest(
        build_manifest_path,
        source_receipt,
        source_receipt_sha256,
        patch_sha256,
        license_sha256,
        sbom_sha256,
    )
    target = str(build_manifest["target_triple"])
    artifact = inspect_artifact(artifact_path, target)

    payload: dict[str, object] = {
        "schema": RECEIPT_SCHEMA,
        "phase": "DEVELOPMENT",
        "claim_level": "ARTIFACT_DIGEST_AND_BUILD_INPUTS_ONLY",
        "captured_at_utc": captured_at,
        "source_binding": {
            "source_receipt_id": source_receipt["receipt_id"],
            "source_receipt_sha256": source_receipt_sha256,
            "repository": EXPECTED_REPOSITORY,
            "commit": EXPECTED_SERVO_COMMIT,
            "tree": EXPECTED_SERVO_TREE,
        },
        "build_binding": {
            "build_manifest_sha256": build_manifest_sha256,
            "target_triple": target,
            "build_profile": build_manifest["build_profile"],
            "rustc_verbose_sha256": build_manifest["rustc_verbose_sha256"],
            "cargo_version": build_manifest["cargo_version"],
            "linker_id": build_manifest["linker_id"],
            "features": build_manifest["features"],
            "build_command_sha256": build_manifest["build_command_sha256"],
            "environment_allowlist_sha256": build_manifest["environment_allowlist_sha256"],
        },
        "artifact": artifact,
        "supporting_inputs": {
            "patch_inventory_sha256": patch_sha256,
            "patch_count": len(patch_inventory["patches"]),
            "license_packet_sha256": license_sha256,
            "license_notice_count": len(license_packet["notices"]),
            "sbom_sha256": sbom_sha256,
            "sbom_bytes": sbom_bytes,
            "sbom_format": build_manifest["sbom_format"],
            "sbom_package_count": len(sbom["packages"]),
        },
        "runtime_qualification": {
            "artifact_executed": False,
            "servo_webview_started": False,
            "listener_scan_passed": False,
            "egress_scan_passed": False,
            "sandbox_qualified": False,
            "platform_matrix_qualified": False,
        },
        "authority": {key: False for key in sorted(NEGATIVE_AUTHORITY_KEYS)},
        "decision": "ARTIFACT_BOUND_RUNTIME_NOT_QUALIFIED",
    }
    identifier = framed_digest(ARTIFACT_RECEIPT_DOMAIN, [canonical_bytes(payload)])
    payload["receipt_id"] = f"servo-worker-artifact-receipt:v1:{identifier}"
    return payload


def write_exclusive(path: pathlib.Path, content: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    try:
        with os.fdopen(descriptor, "wb", closefd=True) as output:
            output.write(content)
            output.flush()
            os.fsync(output.fileno())
        if hasattr(os, "O_DIRECTORY"):
            directory = os.open(path.parent, os.O_RDONLY | os.O_DIRECTORY)
            try:
                os.fsync(directory)
            finally:
                os.close(directory)
    except Exception:
        try:
            path.unlink()
        except OSError:
            pass
        raise


def utc_now() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-receipt", type=pathlib.Path, required=True)
    parser.add_argument("--build-manifest", type=pathlib.Path, required=True)
    parser.add_argument("--artifact", type=pathlib.Path, required=True)
    parser.add_argument("--patch-inventory", type=pathlib.Path, required=True)
    parser.add_argument("--license-packet", type=pathlib.Path, required=True)
    parser.add_argument("--sbom", type=pathlib.Path, required=True)
    parser.add_argument("--output", type=pathlib.Path, required=True)
    parser.add_argument("--captured-at", default=None)
    return parser


def main(argv: list[str] | None = None) -> int:
    arguments = build_parser().parse_args(argv)
    try:
        receipt = artifact_receipt(
            source_receipt_path=arguments.source_receipt,
            build_manifest_path=arguments.build_manifest,
            artifact_path=arguments.artifact,
            patch_inventory_path=arguments.patch_inventory,
            license_packet_path=arguments.license_packet,
            sbom_path=arguments.sbom,
            captured_at=arguments.captured_at or utc_now(),
        )
        encoded = canonical_bytes(receipt)
        write_exclusive(arguments.output, encoded)
    except (ArtifactError, OSError) as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1
    print(
        json.dumps(
            {
                "status": "ARTIFACT_BOUND_RUNTIME_NOT_QUALIFIED",
                "receipt": os.fspath(arguments.output),
                "receipt_id": receipt["receipt_id"],
                "receipt_sha256": hashlib.sha256(encoded).hexdigest(),
                "artifact_sha256": receipt["artifact"]["sha256"],
                "runtime_authority": False,
                "promotion": False,
            },
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
