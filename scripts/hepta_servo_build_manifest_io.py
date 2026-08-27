"""Fail-closed primitives for the Hepta Servo worker build-input manifest."""
from __future__ import annotations

import hashlib
import json
import os
import pathlib
import re
import stat
from typing import Any

SCHEMA = "hepta.servo.worker_build_manifest.v1"
SOURCE_SCHEMA = "hepta.servo.source_receipt.v1"
PATCH_SCHEMA = "hepta.servo.patch_inventory.v1"
LICENSE_SCHEMA = "hepta.servo.license_packet.v1"
REPOSITORY = "https://github.com/servo/servo"
COMMIT = "0a48e298482659817eb50097df23841f2b8e3044"
TREE = "b04d2f75b3217374d079d579c270177b57fa1389"
SOURCE_ID_DOMAIN = b"hepta.servo.source-receipt.v1"
SBOM_FORMAT = "SPDX-2.3-json"
SHA256 = re.compile(r"^[0-9a-f]{64}$")
SAFE = re.compile(r"^[A-Za-z0-9._:+/@=-]{1,256}$")
FEATURE = re.compile(r"^[A-Za-z0-9._:+/@=-]{1,128}$")
TARGETS = {
    "x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu",
    "x86_64-apple-darwin", "aarch64-apple-darwin",
    "x86_64-pc-windows-msvc", "aarch64-pc-windows-msvc",
}
ALLOWED_ENV = {
    "AR", "CARGO_BUILD_JOBS", "CARGO_INCREMENTAL", "CC", "CFLAGS", "CXX",
    "CXXFLAGS", "LANG", "LC_ALL", "LINKER", "LDFLAGS", "PATH_DIGEST_SHA256",
    "PKG_CONFIG_ALLOW_CROSS", "RUSTFLAGS", "SOURCE_DATE_EPOCH", "TZ",
}
SECRET_MARKERS = (
    "TOKEN", "SECRET", "PASSWORD", "PASSWD", "CREDENTIAL", "AUTH", "COOKIE",
    "KEY", "PROXY", "HOME", "SSH", "AWS", "AZURE", "GITHUB", "OPENAI",
)
MAX_JSON = 4 * 1024 * 1024
MAX_TEXT = 64 * 1024
BUILD_KEYS = {
    "schema", "source_receipt_id", "source_receipt_sha256", "target_triple",
    "build_profile", "rustc_verbose_sha256", "cargo_version", "linker_id",
    "features", "build_command_sha256", "environment_allowlist_sha256",
    "patch_inventory_sha256", "license_packet_sha256", "sbom_sha256",
    "sbom_format", "network_access_during_build", "worker_tcp_listener",
    "worker_http_surface", "worker_external_network", "worker_credential_export",
    "worker_production_authority", "worker_effect_authority",
}


class BuildManifestError(RuntimeError):
    pass


def fail(message: str) -> None:
    raise BuildManifestError(message)


def _pairs(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            fail(f"duplicate JSON key {key!r}")
        result[key] = value
    return result


def canonical(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def sha256(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def framed(domain: bytes, *fields: bytes) -> str:
    digest = hashlib.sha256()
    digest.update(len(domain).to_bytes(8, "big")); digest.update(domain)
    for field in fields:
        digest.update(len(field).to_bytes(8, "big")); digest.update(field)
    return digest.hexdigest()


def load(path: pathlib.Path, label: str, maximum: int = MAX_JSON) -> tuple[dict[str, Any], bytes]:
    try:
        metadata = path.lstat()
    except OSError as error:
        fail(f"cannot inspect {label}: {error}")
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail(f"{label} must be a non-symlink regular file")
    if not 1 <= metadata.st_size <= maximum:
        fail(f"{label} byte length is outside 1..={maximum}")
    try:
        raw = path.read_bytes()
        value = json.loads(raw, object_pairs_hook=_pairs)
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot decode {label}: {error}")
    if not isinstance(value, dict) or raw != canonical(value):
        fail(f"{label} must be one compact canonical JSON object")
    return value, raw


def require_sha(value: Any, label: str) -> str:
    if not isinstance(value, str) or not SHA256.fullmatch(value):
        fail(f"{label} must be lowercase SHA-256")
    return value


def require_safe(value: Any, label: str) -> str:
    if not isinstance(value, str) or not SAFE.fullmatch(value):
        fail(f"{label} is empty, oversized, or noncanonical")
    return value


def source_receipt(path: pathlib.Path) -> tuple[dict[str, Any], bytes]:
    value, raw = load(path, "Servo source receipt")
    if (value.get("schema"), value.get("phase"), value.get("claim_level"), value.get("decision")) != (
        SOURCE_SCHEMA, "DEVELOPMENT", "SOURCE_PIN_AND_TREE_ONLY",
        "SOURCE_PIN_VERIFIED_BUILD_NOT_QUALIFIED",
    ):
        fail("Servo source receipt posture is unsupported or overclaims")
    source = value.get("source")
    if not isinstance(source, dict) or (
        source.get("repository"), source.get("commit"), source.get("tree"),
        source.get("clean_worktree"),
    ) != (REPOSITORY, COMMIT, TREE, True):
        fail("Servo source receipt exact source binding drifted")
    authority = value.get("authority")
    if not isinstance(authority, dict) or not authority or any(item is not False for item in authority.values()):
        fail("Servo source receipt contains positive or missing authority")
    if value.get("artifact") != {
        "source_archive_created": False, "source_archive_sha256": None,
        "worker_artifact_built": False, "worker_artifact_sha256": None,
        "sbom_created": False,
    }:
        fail("Servo source receipt overclaims archive/build/SBOM state")
    receipt_id = value.get("receipt_id")
    if not isinstance(receipt_id, str) or not re.fullmatch(r"servo-source-receipt:v1:[0-9a-f]{64}", receipt_id):
        fail("Servo source receipt ID is invalid")
    payload = dict(value); payload.pop("receipt_id")
    if receipt_id != "servo-source-receipt:v1:" + framed(SOURCE_ID_DOMAIN, canonical(payload)):
        fail("Servo source receipt ID does not bind its payload")
    return value, raw


def patch_inventory(path: pathlib.Path) -> bytes:
    value, raw = load(path, "Servo patch inventory")
    if value.get("schema") != PATCH_SCHEMA:
        fail("Servo patch inventory schema is unsupported")
    if value.get("servo_commit") not in (None, COMMIT) or value.get("servo_tree") not in (None, TREE):
        fail("Servo patch inventory source pin drifted")
    patches = value.get("patches")
    if not isinstance(patches, list):
        fail("Servo patch inventory patches must be an array")
    identifiers: list[str] = []
    for item in patches:
        if not isinstance(item, dict):
            fail("Servo patch entry must be an object")
        identifier = item.get("patch_id", item.get("id", item.get("file")))
        identifiers.append(require_safe(identifier, "patch identifier"))
        for key, field in item.items():
            if key.endswith("sha256") and field is not None:
                require_sha(field, f"patch {identifier} {key}")
    if identifiers != sorted(set(identifiers)):
        fail("Servo patch identifiers must be strictly sorted and unique")
    return raw


def license_packet(path: pathlib.Path) -> bytes:
    value, raw = load(path, "Servo license packet")
    if value.get("schema") != LICENSE_SCHEMA or value.get("primary_license") != "MPL-2.0":
        fail("Servo license packet must bind MPL-2.0")
    if value.get("upstream_repository") not in (None, REPOSITORY) or value.get("upstream_commit") not in (None, COMMIT):
        fail("Servo license packet source binding drifted")
    if value.get("license_file_sha256") is not None:
        require_sha(value["license_file_sha256"], "license file digest")
    notices = value.get("notices")
    if not isinstance(notices, list) or not notices or notices != sorted(set(notices)):
        fail("Servo license packet notices must be sorted, unique, and non-empty")
    for notice in notices:
        require_safe(notice, "notice identifier")
    if value.get("source_offer_required") is not True:
        fail("Servo license packet must acknowledge source distribution obligations")
    return raw


def spdx_sbom(path: pathlib.Path) -> bytes:
    value, raw = load(path, "SPDX SBOM")
    if value.get("spdxVersion") != "SPDX-2.3" or value.get("dataLicense") != "CC0-1.0":
        fail("SBOM must be SPDX-2.3 JSON with CC0-1.0 data license")
    if value.get("SPDXID") != "SPDXRef-DOCUMENT":
        fail("SBOM document identity is invalid")
    packages = value.get("packages")
    if not isinstance(packages, list) or not packages:
        fail("SBOM must contain at least one package")
    return raw


def rustc_record(path: pathlib.Path) -> bytes:
    try:
        metadata = path.lstat(); raw = path.read_bytes()
    except OSError as error:
        fail(f"cannot read rustc verbose record: {error}")
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode) or not 1 <= len(raw) <= MAX_TEXT:
        fail("rustc verbose record must be a bounded regular file")
    try:
        text = raw.decode()
    except UnicodeError as error:
        fail(f"rustc verbose record is not UTF-8: {error}")
    if "\0" in text or "\r" in text:
        fail("rustc verbose record contains NUL or CR")
    for prefix in ("rustc ", "release: ", "host: ", "LLVM version: "):
        if not any(line.startswith(prefix) for line in text.splitlines()):
            fail(f"rustc verbose record lacks {prefix!r}")
    return raw
