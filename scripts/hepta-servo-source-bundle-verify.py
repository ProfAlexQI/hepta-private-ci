#!/usr/bin/env python3
"""Independently verify a Hepta Servo source bundle without a Git checkout.

The verifier checks receipts and compressed archive bytes, reconstructs Git
blob/tree object IDs from the tar entries, and requires the recomputed root tree
to equal the pinned Servo tree. It never builds or executes Servo.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import posixpath
import re
import stat
import sys
import tarfile
import zlib
from pathlib import Path, PurePosixPath
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
BUNDLE_ROOT = ROOT / "docs/hepta-vnext/browser"
DEFAULT_PIN = BUNDLE_ROOT / "SERVO_UPSTREAM_PIN.json"
EXPECTED_BUNDLE_SCHEMA = "hepta.browser.servo_independent_source_bundle.v1"
EXPECTED_FETCH_SCHEMA = "hepta.browser.servo_independent_fetch.v1"
EXPECTED_LICENSE_SCHEMA = "hepta.browser.servo_license_packet.v1"
VERIFY_SCHEMA = "hepta.browser.servo_source_bundle_verification.v1"
MAX_ARCHIVE_BYTES = 8 * 1024 * 1024 * 1024
MAX_JSON_BYTES = 8 * 1024 * 1024
LOCAL_PATH_PATTERNS = (
    re.compile(r"(?:^|[\"'])/(?:home|Users|Volumes|tmp|private)/"),
    re.compile(r"[A-Za-z]:\\\\"),
)
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


class BundleVerificationError(RuntimeError):
    pass


def fail(message: str) -> None:
    raise BundleVerificationError(message)


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
            length += len(block)
            if length > MAX_ARCHIVE_BYTES:
                fail(f"file exceeds verification byte bound: {path.name}")
            digest.update(block)
    return digest.hexdigest(), length


def read_json(path: Path, label: str) -> tuple[dict[str, Any], bytes]:
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


def require_keys(value: dict[str, Any], expected: set[str], label: str) -> None:
    actual = set(value)
    if actual != expected:
        fail(f"{label} keys differ: {sorted(actual ^ expected)}")


def require_sha256(value: Any, label: str) -> str:
    if (
        not isinstance(value, str)
        or len(value) != 64
        or any(character not in "0123456789abcdef" for character in value)
    ):
        fail(f"{label} is not lowercase SHA-256")
    return value


def require_git_object(value: Any, label: str) -> str:
    if (
        not isinstance(value, str)
        or len(value) != 40
        or any(character not in "0123456789abcdef" for character in value)
    ):
        fail(f"{label} is not a lowercase SHA-1 Git object")
    return value


def require_closed_authority(value: Any, label: str) -> None:
    if not isinstance(value, dict) or set(value) != AUTHORITY_KEYS:
        fail(f"{label} authority keys differ")
    enabled = sorted(key for key, item in value.items() if item is not False)
    if enabled:
        fail(f"{label} attempted to enable authority: {enabled}")


def require_no_local_paths(raw: bytes, label: str) -> None:
    text = raw.decode("utf-8", "strict")
    for pattern in LOCAL_PATH_PATTERNS:
        if pattern.search(text):
            fail(f"{label} contains a machine-local path")


def safe_bundle_dir(value: str) -> Path:
    path = Path(value)
    if not path.is_absolute() or ".." in path.parts:
        fail("--bundle-dir must be a canonical absolute path")
    try:
        resolved = path.resolve(strict=True)
        metadata = path.lstat()
    except OSError as error:
        fail(f"bundle directory is unavailable: {error}")
    if resolved != path or stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        fail("bundle directory must be a non-symlink canonical directory")
    return path


def safe_regular_file(root: Path, filename: str, label: str) -> Path:
    if not filename or PurePosixPath(filename).name != filename or "/" in filename or "\\" in filename:
        fail(f"{label} filename is unsafe")
    path = root / filename
    try:
        resolved = path.resolve(strict=True)
        metadata = path.lstat()
    except OSError as error:
        fail(f"{label} is unavailable: {error}")
    if resolved != path or stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail(f"{label} must be a non-symlink regular file")
    if metadata.st_nlink != 1:
        fail(f"{label} must have exactly one hard link")
    return path


def load_pin(path: Path) -> dict[str, Any]:
    pin, raw = read_json(path, "Servo source pin")
    require_no_local_paths(raw, "Servo source pin")
    if pin.get("schema") != "hepta.browser.servo_upstream_pin.v1":
        fail("Servo source pin schema is invalid")
    if pin.get("repository") != "servo/servo" or pin.get("license") != "MPL-2.0":
        fail("Servo source pin repository or license is invalid")
    require_git_object(pin.get("commit"), "pinned Servo commit")
    require_git_object(pin.get("tree"), "pinned Servo tree")
    return pin


def git_hash(kind: bytes, content: bytes) -> bytes:
    header = kind + b" " + str(len(content)).encode("ascii") + b"\0"
    return hashlib.sha1(header + content).digest()


class TreeNode:
    def __init__(self) -> None:
        self.files: dict[bytes, tuple[bytes, bytes]] = {}
        self.directories: dict[bytes, "TreeNode"] = {}

    def directory(self, name: bytes) -> "TreeNode":
        if name in self.files:
            fail("archive path is both file and directory")
        return self.directories.setdefault(name, TreeNode())

    def add_file(self, name: bytes, mode: bytes, object_id: bytes) -> None:
        if name in self.files or name in self.directories:
            fail("archive contains a duplicate or conflicting path")
        self.files[name] = (mode, object_id)

    def object_id(self) -> bytes:
        entries: list[tuple[bytes, bytes, bytes]] = []
        for name, (mode, object_id) in self.files.items():
            entries.append((name, mode, object_id))
        for name, node in self.directories.items():
            entries.append((name, b"40000", node.object_id()))
        payload = bytearray()
        for name, mode, object_id in sorted(entries, key=lambda entry: entry[0]):
            payload.extend(mode)
            payload.extend(b" ")
            payload.extend(name)
            payload.extend(b"\0")
            payload.extend(object_id)
        return git_hash(b"tree", bytes(payload))


def safe_member_path(prefix: str, name: str) -> tuple[str, ...]:
    if not name.startswith(prefix) or "\\" in name or "\0" in name:
        fail(f"archive path is outside frozen prefix: {name!r}")
    relative = name[len(prefix) :]
    if not relative:
        return ()
    path = PurePosixPath(relative)
    if path.is_absolute() or ".." in path.parts or any(part in ("", ".") for part in path.parts):
        fail(f"archive path is unsafe: {name!r}")
    return path.parts


def safe_symlink(prefix: str, name: str, target: str) -> None:
    if not target or target.startswith(("/", "\\")) or "\\" in target or "\0" in target:
        fail(f"archive symlink target is unsafe: {name!r}")
    resolved = posixpath.normpath(posixpath.join(posixpath.dirname(name), target))
    if not (resolved == prefix.rstrip("/") or resolved.startswith(prefix)):
        fail(f"archive symlink escapes source root: {name!r}")


def add_tree_entry(root: TreeNode, parts: tuple[str, ...], mode: bytes, object_id: bytes) -> None:
    if not parts:
        fail("archive file has an empty relative path")
    node = root
    for component in parts[:-1]:
        encoded = component.encode("utf-8", "strict")
        if b"\0" in encoded or b"/" in encoded:
            fail("archive path component is invalid")
        node = node.directory(encoded)
    name = parts[-1].encode("utf-8", "strict")
    if b"\0" in name or b"/" in name:
        fail("archive filename is invalid")
    node.add_file(name, mode, object_id)


def verify_tar(tar_path: Path, prefix: str, expected_source: dict[str, Any]) -> dict[str, Any]:
    root = TreeNode()
    explicit_paths: set[str] = set()
    file_count = 0
    directory_count = 0
    symlink_count = 0
    license_sha256: str | None = None
    license_bytes: int | None = None
    with tarfile.open(tar_path, mode="r:") as archive:
        for member in archive:
            parts = safe_member_path(prefix, member.name)
            if member.name in explicit_paths:
                fail(f"archive contains duplicate path: {member.name!r}")
            explicit_paths.add(member.name)
            if member.islnk():
                fail(f"archive contains a hard link: {member.name!r}")
            if member.isdir():
                directory_count += 1
                continue
            if member.isfile():
                extracted = archive.extractfile(member)
                if extracted is None:
                    fail(f"archive file cannot be read: {member.name!r}")
                content = extracted.read()
                if len(content) != member.size:
                    fail(f"archive file length changed while reading: {member.name!r}")
                mode = b"100755" if member.mode & 0o111 else b"100644"
                if member.mode & 0o7000 or member.mode & 0o022:
                    fail(f"archive file has unsafe mode: {member.name!r}")
                add_tree_entry(root, parts, mode, git_hash(b"blob", content))
                file_count += 1
                if parts == ("LICENSE",):
                    license_sha256 = sha256_bytes(content)
                    license_bytes = len(content)
            elif member.issym():
                safe_symlink(prefix, member.name, member.linkname)
                content = member.linkname.encode("utf-8", "strict")
                add_tree_entry(root, parts, b"120000", git_hash(b"blob", content))
                symlink_count += 1
            else:
                fail(f"archive contains unsupported entry type: {member.name!r}")
    if file_count == 0 or not explicit_paths:
        fail("archive is empty")
    recomputed_tree = root.object_id().hex()
    if recomputed_tree != expected_source.get("tree"):
        fail(
            "archive bytes do not reconstruct pinned Git tree: "
            f"expected {expected_source.get('tree')}, found {recomputed_tree}"
        )
    if license_sha256 != expected_source.get("license_sha256"):
        fail("archive LICENSE digest differs from source receipt")
    if license_bytes != expected_source.get("license_bytes"):
        fail("archive LICENSE length differs from source receipt")
    return {
        "recomputed_tree": recomputed_tree,
        "entry_count": len(explicit_paths),
        "file_count": file_count,
        "directory_count": directory_count,
        "symlink_count": symlink_count,
        "hardlink_count": 0,
        "special_entry_count": 0,
        "unsafe_path_count": 0,
        "license_sha256": license_sha256,
        "license_bytes": license_bytes,
    }


def parse_gzip_header(path: Path) -> None:
    with path.open("rb") as handle:
        header = handle.read(10)
    if len(header) != 10 or header[:3] != b"\x1f\x8b\x08":
        fail("compressed source archive is not gzip/deflate")
    if header[3] != 0:
        fail("compressed source archive contains optional gzip header fields")
    if int.from_bytes(header[4:8], "little") != 0:
        fail("compressed source archive gzip mtime is not zero")


def decompress_single_gzip(source: Path, destination: Path) -> tuple[str, int]:
    parse_gzip_header(source)
    digest = hashlib.sha256()
    length = 0
    decompressor = zlib.decompressobj(16 + zlib.MAX_WBITS)
    with source.open("rb") as input_handle, destination.open("xb") as output_handle:
        os.chmod(destination, 0o600)
        while True:
            block = input_handle.read(1024 * 1024)
            if not block:
                break
            output = decompressor.decompress(block)
            if output:
                output_handle.write(output)
                digest.update(output)
                length += len(output)
                if length > MAX_ARCHIVE_BYTES:
                    fail("decompressed source archive exceeds byte bound")
            if decompressor.unused_data:
                fail("compressed source archive contains concatenated members or trailing data")
        output = decompressor.flush()
        if output:
            output_handle.write(output)
            digest.update(output)
            length += len(output)
        output_handle.flush()
        os.fsync(output_handle.fileno())
    if not decompressor.eof or decompressor.unused_data:
        fail("compressed source archive is truncated or has trailing data")
    return digest.hexdigest(), length


def verify_fetch_receipt(
    path: Path,
    expected_summary: dict[str, Any],
) -> tuple[dict[str, Any], str, int]:
    receipt, raw = read_json(path, path.name)
    require_no_local_paths(raw, path.name)
    require_keys(
        receipt,
        {
            "schema",
            "schema_version",
            "source",
            "acquisition",
            "machine_local_paths_included",
            "canonical",
            "authority",
        },
        path.name,
    )
    if receipt.get("schema") != EXPECTED_FETCH_SCHEMA or receipt.get("schema_version") != 1:
        fail(f"{path.name} schema is invalid")
    if receipt.get("canonical") is not True or receipt.get("machine_local_paths_included") is not False:
        fail(f"{path.name} is not a canonical path-free receipt")
    require_closed_authority(receipt.get("authority"), path.name)
    if receipt.get("source") != expected_summary:
        fail(f"{path.name} source projection differs from bundle receipt")
    acquisition = receipt.get("acquisition")
    if not isinstance(acquisition, dict):
        fail(f"{path.name} acquisition record is missing")
    if acquisition.get("origin_kind") != "pinned_https":
        fail(f"{path.name} was not acquired from pinned HTTPS origin")
    if acquisition.get("network_access_used") is not True:
        fail(f"{path.name} does not record acquisition network use")
    if acquisition.get("standalone_object_store") is not True:
        fail(f"{path.name} does not prove a standalone object store")
    if acquisition.get("alternate_object_database") is not False:
        fail(f"{path.name} used an alternate object database")
    require_sha256(acquisition.get("acquisition_nonce_sha256"), f"{path.name} acquisition nonce")
    return receipt, sha256_bytes(raw), len(raw)


def verify_license_packet(path: Path, bundle: dict[str, Any]) -> tuple[str, int]:
    packet, raw = read_json(path, path.name)
    require_no_local_paths(raw, path.name)
    if packet.get("schema") != EXPECTED_LICENSE_SCHEMA or packet.get("schema_version") != 1:
        fail("license packet schema is invalid")
    require_closed_authority(packet.get("authority"), "license packet")
    source = packet.get("source")
    if not isinstance(source, dict):
        fail("license packet source binding is missing")
    expected_source = bundle["source"]
    for key in ("repository", "commit", "tree"):
        if source.get(key) != expected_source.get(key):
            fail(f"license packet source field differs: {key}")
    license_record = packet.get("license")
    if not isinstance(license_record, dict):
        fail("license packet license record is missing")
    if license_record.get("spdx") != "MPL-2.0" or license_record.get("source_distribution_required") is not True:
        fail("license packet does not preserve MPL-2.0 source obligation")
    if license_record.get("sha256") != expected_source.get("license_sha256"):
        fail("license packet LICENSE digest differs")
    if license_record.get("bytes") != expected_source.get("license_bytes"):
        fail("license packet LICENSE length differs")
    expected = bundle.get("license_packet")
    if not isinstance(expected, dict):
        fail("bundle license packet summary is missing")
    digest = sha256_bytes(raw)
    if digest != expected.get("sha256") or len(raw) != expected.get("bytes"):
        fail("license packet bytes differ from bundle receipt")
    if packet.get("patch_inventory_sha256") != expected.get("patch_inventory_sha256"):
        fail("license packet patch inventory digest differs")
    if packet.get("patch_count") != expected.get("patch_count"):
        fail("license packet patch count differs")
    return digest, len(raw)


def verify_bundle(bundle_dir: Path, pin_path: Path, output: Path | None) -> dict[str, Any]:
    pin = load_pin(pin_path)
    bundle_path = safe_regular_file(
        bundle_dir,
        "independent-source-bundle.receipt.json",
        "bundle receipt",
    )
    bundle, bundle_raw = read_json(bundle_path, "bundle receipt")
    require_no_local_paths(bundle_raw, "bundle receipt")
    if bundle.get("schema") != EXPECTED_BUNDLE_SCHEMA or bundle.get("schema_version") != 1:
        fail("bundle receipt schema is invalid")
    if bundle.get("canonical") is not True or bundle.get("machine_local_paths_included") is not False:
        fail("bundle receipt is not canonical and path-free")
    if bundle.get("acquisition_network_used") is not True or bundle.get("runtime_external_network") is not False:
        fail("bundle receipt confuses acquisition network with runtime authority")
    require_closed_authority(bundle.get("authority"), "bundle receipt")
    source = bundle.get("source")
    if not isinstance(source, dict):
        fail("bundle source projection is missing")
    for key in ("repository", "commit", "tree", "license"):
        if source.get(key) != pin.get(key):
            fail(f"bundle source differs from pin: {key}")
    require_git_object(source.get("commit"), "bundle Servo commit")
    require_git_object(source.get("tree"), "bundle Servo tree")
    require_sha256(source.get("tree_manifest_sha256"), "bundle tree manifest")
    require_sha256(source.get("license_sha256"), "bundle LICENSE digest")
    if source.get("submodule_count") != 0:
        fail("source-bundle verification currently requires zero Git submodules")
    qualification = bundle.get("qualification")
    if not isinstance(qualification, dict):
        fail("bundle qualification record is missing")
    for key in (
        "canonical_source_acquired",
        "independent_fetches_compared",
        "deterministic_tar_verified",
        "deterministic_gzip_verified",
        "source_distribution_packet_bound",
    ):
        if qualification.get(key) is not True:
            fail(f"bundle qualification field is not true: {key}")
    for key in ("servo_built", "servo_runtime_qualified", "operator_accepted", "release_qualified"):
        if qualification.get(key) is not False:
            fail(f"source-only bundle attempted to enable {key}")

    fetch_summaries = bundle.get("fetch_receipts")
    if not isinstance(fetch_summaries, list) or len(fetch_summaries) != 2:
        fail("bundle must contain exactly two fetch receipt summaries")
    fetch_receipts: list[dict[str, Any]] = []
    fetch_hashes: list[str] = []
    for expected_slot, filename in (("a", "fetch-a.receipt.json"), ("b", "fetch-b.receipt.json")):
        summary = next(
            (
                item
                for item in fetch_summaries
                if isinstance(item, dict) and item.get("slot") == expected_slot
            ),
            None,
        )
        if summary is None:
            fail(f"bundle fetch receipt summary is missing slot {expected_slot}")
        path = safe_regular_file(bundle_dir, filename, f"fetch receipt {expected_slot}")
        receipt, digest, length = verify_fetch_receipt(path, source)
        if digest != summary.get("sha256") or length != summary.get("bytes") or summary.get("canonical") is not True:
            fail(f"fetch receipt {expected_slot} bytes differ from bundle summary")
        fetch_receipts.append(receipt)
        fetch_hashes.append(digest)
    if (
        fetch_receipts[0]["acquisition"]["acquisition_nonce_sha256"]
        == fetch_receipts[1]["acquisition"]["acquisition_nonce_sha256"]
    ):
        fail("independent fetch receipts reuse one acquisition nonce")
    independence = bundle.get("independence")
    if not isinstance(independence, dict) or independence.get("fetch_count") != 2:
        fail("bundle independence record is missing or not two-fetch")
    for key in ("roots_distinct", "object_stores_distinct", "source_projections_identical"):
        if independence.get(key) is not True:
            fail(f"bundle independence field is not true: {key}")
    if independence.get("source_projection_sha256") != sha256_bytes(canonical_bytes(source)):
        fail("bundle source projection digest is invalid")

    outputs = bundle.get("outputs")
    if not isinstance(outputs, dict) or outputs.get("compressed_source_archive") != "servo-source-a.tar.gz":
        fail("bundle output inventory is missing or invalid")
    gzip_path = safe_regular_file(bundle_dir, "servo-source-a.tar.gz", "compressed source archive")
    gzip_sha256, gzip_bytes = sha256_file(gzip_path)
    archive = bundle.get("archive")
    if not isinstance(archive, dict) or not isinstance(archive.get("gzip"), dict):
        fail("bundle archive summary is missing")
    if gzip_sha256 != archive["gzip"].get("sha256") or gzip_bytes != archive["gzip"].get("bytes"):
        fail("compressed source archive differs from bundle receipt")
    temporary_tar = bundle_dir / f".verify-source-{os.getpid()}.tar"
    try:
        tar_sha256, tar_bytes = decompress_single_gzip(gzip_path, temporary_tar)
        tar_summary = archive.get("tar")
        if not isinstance(tar_summary, dict):
            fail("bundle tar summary is missing")
        if tar_sha256 != tar_summary.get("sha256") or tar_bytes != tar_summary.get("bytes"):
            fail("decompressed tar differs from bundle receipt")
        prefix = tar_summary.get("prefix")
        if prefix != f"servo-{source['commit']}/":
            fail("source archive prefix differs from pinned commit")
        tar_verification = verify_tar(temporary_tar, prefix, source)
        for key in (
            "entry_count",
            "file_count",
            "directory_count",
            "symlink_count",
            "hardlink_count",
            "unsafe_path_count",
        ):
            if tar_verification.get(key) != tar_summary.get(key):
                fail(f"tar inspection differs from bundle receipt: {key}")
    finally:
        temporary_tar.unlink(missing_ok=True)

    if outputs.get("license_packet") != "license-packet.json":
        fail("bundle license packet filename is invalid")
    license_path = safe_regular_file(bundle_dir, "license-packet.json", "license packet")
    license_sha256, _license_bytes = verify_license_packet(license_path, bundle)

    verification = {
        "schema": VERIFY_SCHEMA,
        "schema_version": 1,
        "source": {
            "repository": source["repository"],
            "commit": source["commit"],
            "tree": source["tree"],
            "tree_manifest_sha256": source["tree_manifest_sha256"],
            "recomputed_tree": tar_verification["recomputed_tree"],
        },
        "bundle_receipt_sha256": sha256_bytes(bundle_raw),
        "fetch_receipt_sha256": fetch_hashes,
        "compressed_archive_sha256": gzip_sha256,
        "tar_sha256": tar_sha256,
        "license_packet_sha256": license_sha256,
        "verification": {
            "canonical_json": True,
            "no_machine_local_paths": True,
            "two_distinct_acquisition_nonces": True,
            "gzip_single_member_mtime_zero": True,
            "archive_paths_safe": True,
            "git_tree_recomputed": True,
            "pinned_tree_matched": True,
            "license_matched": True,
            "servo_built": False,
            "servo_runtime_qualified": False,
            "release_qualified": False,
        },
        "machine_local_paths_included": False,
        "authority": {key: False for key in sorted(AUTHORITY_KEYS)},
    }
    if output is not None:
        if output.exists():
            fail("verification receipt output already exists")
        encoded = canonical_bytes(verification)
        descriptor = os.open(output, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        with os.fdopen(descriptor, "wb") as handle:
            handle.write(encoded)
            handle.flush()
            os.fsync(handle.fileno())
    return verification


def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bundle-dir", required=True)
    parser.add_argument("--pin", default=os.fspath(DEFAULT_PIN))
    parser.add_argument("--output")
    return parser.parse_args()


def main() -> int:
    try:
        arguments = parse_arguments()
        bundle_dir = safe_bundle_dir(arguments.bundle_dir)
        pin_path = Path(arguments.pin).resolve(strict=True)
        output = None
        if arguments.output:
            output = Path(arguments.output)
            if not output.is_absolute() or output != output.parent.resolve(strict=True) / output.name:
                fail("--output must be a canonical absolute path")
        result = verify_bundle(bundle_dir, pin_path, output)
    except (BundleVerificationError, OSError, UnicodeError, tarfile.TarError, zlib.error) as error:
        print(f"HEPTA_SERVO_SOURCE_BUNDLE_VERIFY=FAIL: {error}", file=sys.stderr)
        return 1
    print(
        json.dumps(
            {
                "schema": VERIFY_SCHEMA,
                "status": "PASS_SOURCE_BUNDLE_ONLY",
                "commit": result["source"]["commit"],
                "tree": result["source"]["tree"],
                "recomputed_tree": result["source"]["recomputed_tree"],
                "servo_built": False,
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
