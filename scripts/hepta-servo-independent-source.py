#!/usr/bin/env python3
"""Acquire and seal two independent copies of the pinned Servo source.

This tool is source-only. It does not build or execute Servo and it never grants
browser, network, production, effect, operator, promotion, or release authority.
"""

from __future__ import annotations

import argparse
import gzip
import hashlib
import json
import os
import posixpath
import shutil
import stat
import subprocess
import sys
import tarfile
from pathlib import Path, PurePosixPath
from typing import Any, BinaryIO, Iterable

ROOT = Path(__file__).resolve().parents[1]
BUNDLE = ROOT / "docs/hepta-vnext/browser"
DEFAULT_PIN = BUNDLE / "SERVO_UPSTREAM_PIN.json"
DEFAULT_PATCH_INVENTORY = ROOT / "third_party/servo-patches/PATCH_INVENTORY.json"
FETCH_SCHEMA = "hepta.browser.servo_independent_fetch.v1"
BUNDLE_SCHEMA = "hepta.browser.servo_independent_source_bundle.v1"
LICENSE_PACKET_SCHEMA = "hepta.browser.servo_license_packet.v1"
EXPECTED_REPOSITORY = "servo/servo"
EXPECTED_REPOSITORY_URL = "https://github.com/servo/servo"
EXPECTED_LICENSE = "MPL-2.0"
MAX_GIT_ERROR_BYTES = 4096
MAX_ARCHIVE_BYTES = 8 * 1024 * 1024 * 1024
AUTHORITY = {
    "runtime_authority": False,
    "effect_authority": False,
    "production_caller": False,
    "production_writer": False,
    "runtime_external_network": False,
    "raw_cookie_export": False,
    "credential_export": False,
    "operator_acceptance": False,
    "promotion": False,
    "release_qualified": False,
}


class SourcePipelineError(RuntimeError):
    """Raised when source acquisition or sealing cannot be proven safe."""


def fail(message: str) -> None:
    raise SourcePipelineError(message)


def canonical_bytes(value: Any) -> bytes:
    return json.dumps(
        value,
        ensure_ascii=False,
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def sha256_file(path: Path) -> tuple[str, int]:
    digest = hashlib.sha256()
    length = 0
    with path.open("rb") as handle:
        while True:
            block = handle.read(1024 * 1024)
            if not block:
                break
            digest.update(block)
            length += len(block)
            if length > MAX_ARCHIVE_BYTES:
                fail(f"file exceeds source-pipeline byte bound: {path.name}")
    return digest.hexdigest(), length


def framed_sha256(domain: bytes, fields: Iterable[bytes]) -> str:
    digest = hashlib.sha256()
    digest.update(len(domain).to_bytes(8, "big"))
    digest.update(domain)
    for field in fields:
        digest.update(len(field).to_bytes(8, "big"))
        digest.update(field)
    return digest.hexdigest()


def load_object(path: Path, label: str) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot read {label}: {error}")
    if not isinstance(value, dict):
        fail(f"{label} must contain one JSON object")
    return value


def require_hex(value: Any, length: int, label: str) -> str:
    if not isinstance(value, str):
        fail(f"{label} must be a string")
    try:
        encoded = value.encode("ascii", "strict")
    except UnicodeEncodeError:
        fail(f"{label} must use ASCII lowercase hexadecimal")
    if len(encoded) != length or any(byte not in b"0123456789abcdef" for byte in encoded):
        fail(f"{label} must be exactly {length} lowercase hexadecimal characters")
    return value


def normalized_repository_url(value: str) -> str:
    normalized = value.strip().rstrip("/")
    if normalized.endswith(".git"):
        normalized = normalized[:-4]
    return normalized


def is_local_repository(value: str) -> bool:
    return value.startswith(("file://", "/", "./", "../")) or ":\\" in value


def safe_absolute_path(value: str, label: str, *, must_exist: bool) -> Path:
    path = Path(value)
    if not path.is_absolute() or ".." in path.parts:
        fail(f"{label} must be an absolute path without '..'")
    if must_exist:
        try:
            resolved = path.resolve(strict=True)
        except OSError as error:
            fail(f"{label} is unavailable: {error}")
        if resolved != path:
            fail(f"{label} must already be canonical and contain no symlink component")
    else:
        try:
            parent = path.parent.resolve(strict=True)
        except OSError as error:
            fail(f"{label} parent is unavailable: {error}")
        if path != parent / path.name:
            fail(f"{label} must already be canonical and contain no symlink component")
    return path


def regular_non_symlink(path: Path, label: str) -> None:
    try:
        metadata = path.lstat()
    except OSError as error:
        fail(f"{label} is unavailable: {error}")
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail(f"{label} must be a non-symlink regular file")


def git_environment() -> dict[str, str]:
    environment = os.environ.copy()
    environment.update(
        {
            "GIT_CONFIG_NOSYSTEM": "1",
            "GIT_TERMINAL_PROMPT": "0",
            "GIT_OPTIONAL_LOCKS": "0",
            "LC_ALL": "C",
            "LANG": "C",
        }
    )
    return environment


def run_git(
    source: Path | None,
    *arguments: str,
    stdout: BinaryIO | int | None = subprocess.PIPE,
) -> bytes:
    command = ["git"]
    if source is not None:
        command.extend(["-C", os.fspath(source)])
    command.extend(arguments)
    try:
        result = subprocess.run(
            command,
            check=False,
            stdin=subprocess.DEVNULL,
            stdout=stdout,
            stderr=subprocess.PIPE,
            env=git_environment(),
        )
    except OSError as error:
        fail(f"cannot execute git: {error}")
    if result.returncode != 0:
        detail = (result.stderr or b"")[:MAX_GIT_ERROR_BYTES].decode("utf-8", "replace").strip()
        fail(f"git {' '.join(arguments)} failed: {detail or 'unknown Git error'}")
    if stdout == subprocess.PIPE:
        return result.stdout or b""
    return b""


def one_line(value: bytes, label: str) -> str:
    try:
        text = value.decode("utf-8", "strict")
    except UnicodeError as error:
        fail(f"{label} is not UTF-8: {error}")
    lines = text.splitlines()
    if len(lines) != 1 or not lines[0]:
        fail(f"{label} did not produce exactly one non-empty line")
    return lines[0]


def load_pin(path: Path) -> dict[str, Any]:
    pin = load_object(path, "Servo source pin")
    if pin.get("schema") != "hepta.browser.servo_upstream_pin.v1":
        fail("Servo source pin schema is invalid")
    if pin.get("repository") != EXPECTED_REPOSITORY:
        fail("Servo source pin repository is invalid")
    if normalized_repository_url(str(pin.get("repository_url", ""))) != EXPECTED_REPOSITORY_URL:
        fail("Servo source pin URL is invalid")
    if pin.get("license") != EXPECTED_LICENSE:
        fail("Servo source pin license is invalid")
    if pin.get("integration_status") != "SOURCE_PIN_ONLY_NOT_IMPORTED":
        fail("Servo source pin must remain source-only before this pipeline runs")
    require_hex(pin.get("commit"), 40, "Servo commit")
    require_hex(pin.get("tree"), 40, "Servo tree")
    authority = pin.get("authority")
    if not isinstance(authority, dict) or any(value is not False for value in authority.values()):
        fail("Servo source pin attempted to enable authority")
    return pin


def load_patch_inventory(path: Path, pin: dict[str, Any]) -> dict[str, Any]:
    inventory = load_object(path, "Servo patch inventory")
    if inventory.get("schema") != "hepta.browser.servo_patch_inventory.v1":
        fail("Servo patch inventory schema is invalid")
    if inventory.get("servo_commit") != pin["commit"] or inventory.get("servo_tree") != pin["tree"]:
        fail("Servo patch inventory differs from the pinned source")
    patches = inventory.get("patches")
    if not isinstance(patches, list):
        fail("Servo patch inventory patches must be an array")
    seen: set[str] = set()
    previous = ""
    for patch in patches:
        if not isinstance(patch, dict):
            fail("Servo patch entry must be an object")
        patch_id = patch.get("id")
        if not isinstance(patch_id, str) or not patch_id or patch_id in seen:
            fail("Servo patch entry has a missing or duplicate id")
        if patch_id <= previous:
            fail("Servo patch inventory must be strictly sorted by id")
        previous = patch_id
        seen.add(patch_id)
    return inventory


def ensure_empty_destination(path: Path) -> None:
    if path.exists():
        fail(f"independent fetch destination already exists: {path.name}")
    path.mkdir(mode=0o700)


def parse_tree_inventory(raw: bytes) -> dict[str, Any]:
    entries = [entry for entry in raw.split(b"\0") if entry]
    digest_fields: list[bytes] = []
    path_count = 0
    blob_count = 0
    symlink_count = 0
    submodule_count = 0
    previous_path: bytes | None = None
    for entry in entries:
        metadata, separator, path_bytes = entry.partition(b"\t")
        if separator != b"\t":
            fail("Servo ls-tree entry has no path separator")
        parts = metadata.split(b" ")
        if len(parts) != 3:
            fail("Servo ls-tree entry has invalid metadata")
        mode, object_type, object_id = parts
        if len(object_id) != 40 or any(byte not in b"0123456789abcdef" for byte in object_id):
            fail("Servo ls-tree entry has an invalid object id")
        try:
            path_text = path_bytes.decode("utf-8", "strict")
        except UnicodeError as error:
            fail(f"Servo source path is not UTF-8: {error}")
        path = PurePosixPath(path_text)
        if not path_text or path.is_absolute() or ".." in path.parts or "\\" in path_text:
            fail(f"Servo source tree contains an unsafe path: {path_text!r}")
        if previous_path is not None and path_bytes <= previous_path:
            fail("Servo source tree paths are not strictly sorted")
        previous_path = path_bytes
        path_count += 1
        blob_count += object_type == b"blob"
        symlink_count += mode == b"120000"
        submodule_count += mode == b"160000"
        digest_fields.extend((mode, object_type, object_id, path_bytes))
    if path_count == 0 or blob_count == 0:
        fail("Servo source tree inventory is empty")
    return {
        "path_count": path_count,
        "blob_count": blob_count,
        "symlink_count": symlink_count,
        "submodule_count": submodule_count,
        "tree_manifest_sha256": framed_sha256(
            b"hepta.servo.git-tree-manifest:v1",
            digest_fields,
        ),
    }


def assert_clean_checkout(source: Path, pin: dict[str, Any]) -> dict[str, Any]:
    if not (source / ".git").is_dir():
        fail("independent Servo source is not a standalone Git checkout")
    if (source / ".git/objects/info/alternates").exists():
        fail("independent Servo source uses an alternate object database")
    commit = one_line(run_git(source, "rev-parse", "HEAD"), "Servo HEAD")
    tree = one_line(run_git(source, "rev-parse", "HEAD^{tree}"), "Servo tree")
    if commit != pin["commit"] or tree != pin["tree"]:
        fail("independent Servo fetch does not match the pinned commit and tree")
    if run_git(source, "status", "--porcelain=v1", "-z", "--untracked-files=all"):
        fail("independent Servo checkout is dirty or contains untracked files")
    object_format = one_line(run_git(source, "rev-parse", "--show-object-format"), "Servo object format")
    if object_format != "sha1":
        fail(f"unsupported Servo Git object format: {object_format}")
    inventory = parse_tree_inventory(run_git(source, "ls-tree", "-rz", "-r", "--full-tree", "HEAD"))
    license_path = source / "LICENSE"
    regular_non_symlink(license_path, "Servo LICENSE")
    license_text = license_path.read_text(encoding="utf-8", errors="strict")
    if "Mozilla Public License Version 2.0" not in license_text:
        fail("Servo LICENSE does not contain the MPL-2.0 text")
    license_sha256, license_bytes = sha256_file(license_path)
    return {
        "repository": pin["repository"],
        "commit": commit,
        "tree": tree,
        "object_format": object_format,
        **inventory,
        "license": EXPECTED_LICENSE,
        "license_sha256": license_sha256,
        "license_bytes": license_bytes,
    }


def source_projection(receipt: dict[str, Any]) -> dict[str, Any]:
    fields = (
        "repository",
        "commit",
        "tree",
        "object_format",
        "path_count",
        "blob_count",
        "symlink_count",
        "submodule_count",
        "tree_manifest_sha256",
        "license",
        "license_sha256",
        "license_bytes",
    )
    source = receipt.get("source")
    if not isinstance(source, dict):
        fail("independent fetch receipt has no source projection")
    return {field: source.get(field) for field in fields}


def acquire_source(
    repository_url: str,
    destination: Path,
    pin: dict[str, Any],
    *,
    allow_local_test_origin: bool,
) -> dict[str, Any]:
    local_origin = is_local_repository(repository_url)
    if local_origin and not allow_local_test_origin:
        fail("canonical source acquisition forbids local or file origins")
    if not local_origin and normalized_repository_url(repository_url) != EXPECTED_REPOSITORY_URL:
        fail("canonical source acquisition requires the pinned HTTPS Servo origin")
    ensure_empty_destination(destination)
    run_git(destination, "init", "--quiet")
    run_git(destination, "remote", "add", "origin", repository_url)
    run_git(
        destination,
        "-c",
        "protocol.version=2",
        "fetch",
        "--quiet",
        "--no-tags",
        "--depth=1",
        "--filter=blob:none",
        "origin",
        pin["commit"],
    )
    run_git(destination, "checkout", "--quiet", "--detach", "--force", "FETCH_HEAD")
    remote = one_line(run_git(destination, "remote", "get-url", "origin"), "Servo origin")
    if normalized_repository_url(remote) != normalized_repository_url(repository_url):
        fail("independent Servo fetch origin changed unexpectedly")
    source = assert_clean_checkout(destination, pin)
    nonce = os.urandom(32)
    receipt = {
        "schema": FETCH_SCHEMA,
        "schema_version": 1,
        "source": source,
        "acquisition": {
            "method": "git_fetch_depth_1_filter_blob_none",
            "origin_kind": "test_local" if local_origin else "pinned_https",
            "network_access_used": not local_origin,
            "standalone_object_store": True,
            "alternate_object_database": False,
            "acquisition_nonce_sha256": sha256_bytes(nonce),
        },
        "machine_local_paths_included": False,
        "canonical": not local_origin,
        "authority": AUTHORITY,
    }
    nonce = b"\0" * len(nonce)
    return receipt


def require_distinct_fetches(
    left_root: Path,
    right_root: Path,
    left: dict[str, Any],
    right: dict[str, Any],
) -> dict[str, Any]:
    if left_root == right_root or os.path.samefile(left_root, right_root):
        fail("independent Servo fetch roots are not distinct")
    if os.path.samefile(left_root / ".git/objects", right_root / ".git/objects"):
        fail("independent Servo fetches share one Git object store")
    left_nonce = left.get("acquisition", {}).get("acquisition_nonce_sha256")
    right_nonce = right.get("acquisition", {}).get("acquisition_nonce_sha256")
    if not isinstance(left_nonce, str) or left_nonce == right_nonce:
        fail("independent Servo fetch acquisition nonces are not distinct")
    left_projection = source_projection(left)
    if left_projection != source_projection(right):
        fail("independent Servo fetches produced different source projections")
    return {
        "fetch_count": 2,
        "roots_distinct": True,
        "object_stores_distinct": True,
        "source_projections_identical": True,
        "source_projection_sha256": sha256_bytes(canonical_bytes(left_projection)),
    }


def safe_archive_member(prefix: str, member: tarfile.TarInfo) -> None:
    name = member.name
    if not name.startswith(prefix) or "\\" in name or "\0" in name:
        fail(f"source archive contains an invalid path: {name!r}")
    path = PurePosixPath(name)
    if path.is_absolute() or ".." in path.parts:
        fail(f"source archive contains path traversal: {name!r}")
    if member.islnk():
        fail(f"source archive contains a hard link: {name!r}")
    if not (member.isfile() or member.isdir() or member.issym()):
        fail(f"source archive contains an unsupported entry type: {name!r}")
    if member.issym():
        target = member.linkname
        if not target or target.startswith(("/", "\\")) or "\\" in target or "\0" in target:
            fail(f"source archive contains an unsafe symlink target: {name!r}")
        resolved = posixpath.normpath(posixpath.join(posixpath.dirname(name), target))
        if not (resolved == prefix.rstrip("/") or resolved.startswith(prefix)):
            fail(f"source archive symlink escapes the archive root: {name!r}")


def inspect_tar(path: Path, prefix: str) -> dict[str, Any]:
    names: set[str] = set()
    file_count = 0
    directory_count = 0
    symlink_count = 0
    with tarfile.open(path, mode="r:") as archive:
        for member in archive:
            safe_archive_member(prefix, member)
            if member.name in names:
                fail(f"source archive contains a duplicate path: {member.name!r}")
            names.add(member.name)
            file_count += member.isfile()
            directory_count += member.isdir()
            symlink_count += member.issym()
    if not names or file_count == 0:
        fail("source archive is empty")
    return {
        "entry_count": len(names),
        "file_count": file_count,
        "directory_count": directory_count,
        "symlink_count": symlink_count,
        "hardlink_count": 0,
        "unsafe_path_count": 0,
    }


def git_archive(source: Path, commit: str, destination: Path) -> dict[str, Any]:
    if destination.exists():
        fail(f"source archive output already exists: {destination.name}")
    prefix = f"servo-{commit}/"
    temporary = destination.parent / f".{destination.name}.tmp-{os.getpid()}"
    try:
        descriptor = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        with os.fdopen(descriptor, "wb") as handle:
            run_git(source, "archive", "--format=tar", f"--prefix={prefix}", commit, stdout=handle)
            handle.flush()
            os.fsync(handle.fileno())
        os.replace(temporary, destination)
    except OSError as error:
        temporary.unlink(missing_ok=True)
        fail(f"cannot write deterministic Servo archive: {error}")
    archive_sha256, archive_bytes = sha256_file(destination)
    return {
        "format": "git_archive_tar",
        "prefix": prefix,
        "sha256": archive_sha256,
        "bytes": archive_bytes,
        **inspect_tar(destination, prefix),
    }


def deterministic_gzip(source: Path, destination: Path) -> dict[str, Any]:
    if destination.exists():
        fail(f"compressed source archive already exists: {destination.name}")
    temporary = destination.parent / f".{destination.name}.tmp-{os.getpid()}"
    try:
        descriptor = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        with source.open("rb") as input_handle, os.fdopen(descriptor, "wb") as raw_output:
            with gzip.GzipFile(
                filename="",
                mode="wb",
                compresslevel=9,
                fileobj=raw_output,
                mtime=0,
            ) as compressed:
                shutil.copyfileobj(input_handle, compressed, length=1024 * 1024)
            raw_output.flush()
            os.fsync(raw_output.fileno())
        os.replace(temporary, destination)
    except OSError as error:
        temporary.unlink(missing_ok=True)
        fail(f"cannot write deterministic gzip archive: {error}")
    digest, length = sha256_file(destination)
    return {
        "format": "gzip_mtime_0_level_9_no_filename",
        "sha256": digest,
        "bytes": length,
    }


def archive_source(source: Path, pin: dict[str, Any], tar_path: Path, gzip_path: Path) -> dict[str, Any]:
    assert_clean_checkout(source, pin)
    return {
        "tar": git_archive(source, pin["commit"], tar_path),
        "gzip": deterministic_gzip(tar_path, gzip_path),
    }


def build_license_packet(
    source: Path,
    pin: dict[str, Any],
    patch_inventory: dict[str, Any],
) -> dict[str, Any]:
    license_path = source / "LICENSE"
    regular_non_symlink(license_path, "Servo LICENSE")
    license_sha256, license_bytes = sha256_file(license_path)
    return {
        "schema": LICENSE_PACKET_SCHEMA,
        "schema_version": 1,
        "source": {
            "repository": pin["repository"],
            "commit": pin["commit"],
            "tree": pin["tree"],
        },
        "license": {
            "spdx": EXPECTED_LICENSE,
            "path_in_source_archive": "LICENSE",
            "sha256": license_sha256,
            "bytes": license_bytes,
            "source_distribution_required": True,
        },
        "patch_inventory_sha256": sha256_bytes(canonical_bytes(patch_inventory)),
        "patch_count": len(patch_inventory["patches"]),
        "machine_local_paths_included": False,
        "authority": AUTHORITY,
    }


def write_atomic_json(path: Path, value: dict[str, Any]) -> tuple[str, int]:
    if path.exists() and path.is_symlink():
        fail(f"JSON output cannot be a symlink: {path.name}")
    encoded = canonical_bytes(value)
    temporary = path.parent / f".{path.name}.tmp-{os.getpid()}"
    try:
        descriptor = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        with os.fdopen(descriptor, "wb") as handle:
            handle.write(encoded)
            handle.flush()
            os.fsync(handle.fileno())
        os.replace(temporary, path)
    except OSError as error:
        temporary.unlink(missing_ok=True)
        fail(f"cannot write {path.name}: {error}")
    return sha256_bytes(encoded), len(encoded)


def execute_pipeline(
    *,
    repository_url: str,
    output_dir: Path,
    pin_path: Path,
    patch_inventory_path: Path,
    keep_checkouts: bool,
    allow_local_test_origin: bool,
) -> dict[str, Any]:
    pin = load_pin(pin_path)
    patch_inventory = load_patch_inventory(patch_inventory_path, pin)
    if output_dir.exists():
        fail("source-pipeline output directory must not already exist")
    output_dir.mkdir(mode=0o700)
    left_root = output_dir / "fetch-a"
    right_root = output_dir / "fetch-b"
    left = acquire_source(
        repository_url,
        left_root,
        pin,
        allow_local_test_origin=allow_local_test_origin,
    )
    right = acquire_source(
        repository_url,
        right_root,
        pin,
        allow_local_test_origin=allow_local_test_origin,
    )
    comparison = require_distinct_fetches(left_root, right_root, left, right)

    left_receipt_path = output_dir / "fetch-a.receipt.json"
    right_receipt_path = output_dir / "fetch-b.receipt.json"
    left_receipt_sha256, left_receipt_bytes = write_atomic_json(left_receipt_path, left)
    right_receipt_sha256, right_receipt_bytes = write_atomic_json(right_receipt_path, right)

    left_tar = output_dir / "servo-source-a.tar"
    left_gzip = output_dir / "servo-source-a.tar.gz"
    right_tar = output_dir / "servo-source-b.tar"
    right_gzip = output_dir / "servo-source-b.tar.gz"
    left_archive = archive_source(left_root, pin, left_tar, left_gzip)
    right_archive = archive_source(right_root, pin, right_tar, right_gzip)
    if left_archive != right_archive:
        fail("independent Servo fetches did not produce byte-identical tar and gzip archives")

    license_packet = build_license_packet(left_root, pin, patch_inventory)
    license_packet_path = output_dir / "license-packet.json"
    license_packet_sha256, license_packet_bytes = write_atomic_json(
        license_packet_path,
        license_packet,
    )

    canonical = not allow_local_test_origin and not is_local_repository(repository_url)
    bundle = {
        "schema": BUNDLE_SCHEMA,
        "schema_version": 1,
        "source": source_projection(left),
        "independence": comparison,
        "fetch_receipts": [
            {
                "slot": "a",
                "sha256": left_receipt_sha256,
                "bytes": left_receipt_bytes,
                "canonical": left["canonical"],
            },
            {
                "slot": "b",
                "sha256": right_receipt_sha256,
                "bytes": right_receipt_bytes,
                "canonical": right["canonical"],
            },
        ],
        "archive": left_archive,
        "license_packet": {
            "sha256": license_packet_sha256,
            "bytes": license_packet_bytes,
            "patch_inventory_sha256": license_packet["patch_inventory_sha256"],
            "patch_count": license_packet["patch_count"],
        },
        "outputs": {
            "source_archive": left_tar.name,
            "compressed_source_archive": left_gzip.name,
            "fetch_receipts": [left_receipt_path.name, right_receipt_path.name],
            "license_packet": license_packet_path.name,
        },
        "qualification": {
            "canonical_source_acquired": canonical,
            "independent_fetches_compared": True,
            "deterministic_tar_verified": True,
            "deterministic_gzip_verified": True,
            "source_distribution_packet_bound": True,
            "servo_built": False,
            "servo_runtime_qualified": False,
            "operator_accepted": False,
            "release_qualified": False,
        },
        "acquisition_network_used": canonical,
        "runtime_external_network": False,
        "machine_local_paths_included": False,
        "canonical": canonical,
        "authority": AUTHORITY,
    }
    write_atomic_json(output_dir / "independent-source-bundle.receipt.json", bundle)

    right_tar.unlink()
    right_gzip.unlink()
    if not keep_checkouts:
        shutil.rmtree(left_root)
        shutil.rmtree(right_root)
    return bundle


def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", required=True)
    parser.add_argument("--repository-url", default=EXPECTED_REPOSITORY_URL)
    parser.add_argument("--pin", default=os.fspath(DEFAULT_PIN))
    parser.add_argument("--patch-inventory", default=os.fspath(DEFAULT_PATCH_INVENTORY))
    parser.add_argument("--keep-checkouts", action="store_true")
    parser.add_argument("--test-only-allow-local-origin", action="store_true", help=argparse.SUPPRESS)
    return parser.parse_args()


def main() -> int:
    try:
        arguments = parse_arguments()
        output_dir = safe_absolute_path(arguments.output_dir, "--output-dir", must_exist=False)
        pin_path = safe_absolute_path(arguments.pin, "--pin", must_exist=True)
        patch_inventory_path = safe_absolute_path(
            arguments.patch_inventory,
            "--patch-inventory",
            must_exist=True,
        )
        bundle = execute_pipeline(
            repository_url=arguments.repository_url,
            output_dir=output_dir,
            pin_path=pin_path,
            patch_inventory_path=patch_inventory_path,
            keep_checkouts=arguments.keep_checkouts,
            allow_local_test_origin=arguments.test_only_allow_local_origin,
        )
    except (SourcePipelineError, OSError, UnicodeError, tarfile.TarError) as error:
        print(f"HEPTA_SERVO_INDEPENDENT_SOURCE=FAIL: {error}", file=sys.stderr)
        return 1
    print(
        canonical_bytes(
            {
                "schema": BUNDLE_SCHEMA,
                "status": "PASS_SOURCE_ONLY" if bundle["canonical"] else "PASS_TEST_FIXTURE_ONLY",
                "commit": bundle["source"]["commit"],
                "tree": bundle["source"]["tree"],
                "archive_sha256": bundle["archive"]["gzip"]["sha256"],
                "servo_built": False,
                "servo_runtime_qualified": False,
                "authority": "all_false",
            }
        ).decode("utf-8")
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
