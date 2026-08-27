#!/usr/bin/env python3
"""Generate a deterministic, fail-closed receipt for the pinned Servo source.

The generator performs no network access. It accepts only a canonical, clean
checkout at the exact commit and tree frozen by the Hepta browser plan. Machine-
local source paths are intentionally excluded from the receipt.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import stat
import subprocess
import sys
from pathlib import Path
from typing import Any, Iterable

ROOT = Path(__file__).resolve().parents[1]
BUNDLE = ROOT / "docs/hepta-vnext/browser"
PIN_PATH = BUNDLE / "SERVO_UPSTREAM_PIN.json"
TOPOLOGY_PATH = BUNDLE / "SERVO_SOURCE_IMPORT_TOPOLOGY.yaml"
PATCH_INVENTORY_PATH = ROOT / "third_party/servo-patches/PATCH_INVENTORY.json"
PATCH_ROOT = PATCH_INVENTORY_PATH.parent
REQUIRED_SOURCE_FILES = (
    "Cargo.toml",
    "Cargo.lock",
    "LICENSE",
    "rust-toolchain.toml",
)
AUTHORITY = {
    "runtime_authority": False,
    "production_caller": False,
    "production_writer": False,
    "external_network": False,
    "external_effect": False,
    "operator_acceptance": False,
    "promotion": False,
}


class ProvenanceError(RuntimeError):
    """Raised when the source cannot produce an authoritative dev receipt."""


def fail(message: str) -> None:
    raise ProvenanceError(message)


def load_object(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot read canonical object {path.relative_to(ROOT)}: {error}")
    if not isinstance(value, dict):
        fail(f"canonical object must contain one JSON object: {path.relative_to(ROOT)}")
    return value


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
    try:
        with path.open("rb") as handle:
            while True:
                block = handle.read(1024 * 1024)
                if not block:
                    break
                digest.update(block)
                length += len(block)
    except OSError as error:
        fail(f"cannot hash source file {path.name}: {error}")
    return digest.hexdigest(), length


def git(source: Path, *arguments: str, check: bool = True) -> subprocess.CompletedProcess[str]:
    command = ["git", "-C", os.fspath(source), *arguments]
    try:
        result = subprocess.run(
            command,
            check=False,
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="strict",
        )
    except (OSError, UnicodeError) as error:
        fail(f"cannot execute git command {arguments!r}: {error}")
    if check and result.returncode != 0:
        detail = result.stderr.strip() or result.stdout.strip() or "unknown git failure"
        fail(f"git {' '.join(arguments)} failed: {detail}")
    return result


def one_line(result: subprocess.CompletedProcess[str], label: str) -> str:
    lines = result.stdout.splitlines()
    if len(lines) != 1 or not lines[0]:
        fail(f"{label} did not produce exactly one non-empty line")
    return lines[0]


def require_canonical_source(path_value: str) -> Path:
    source = Path(path_value)
    if not source.is_absolute():
        fail("--servo-source must be an absolute path")
    try:
        canonical = source.resolve(strict=True)
        metadata = canonical.lstat()
    except OSError as error:
        fail(f"Servo source root is unavailable: {error}")
    if canonical != source:
        fail("--servo-source must already be canonical and contain no symlink or '..' components")
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        fail("Servo source root must be a non-symlink directory")
    if not (canonical / ".git").exists():
        fail("Servo source root is not a Git checkout")
    return canonical


def require_regular_source_file(source: Path, relative: str) -> Path:
    if not relative or relative.startswith(("/", "~")) or ".." in Path(relative).parts:
        fail(f"invalid source-relative path in topology: {relative!r}")
    path = source / relative
    try:
        canonical = path.resolve(strict=True)
        metadata = path.lstat()
    except OSError as error:
        fail(f"required Servo source file is unavailable ({relative}): {error}")
    if canonical != path:
        fail(f"Servo source file contains a symlink component: {relative}")
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail(f"Servo source file must be a non-symlink regular file: {relative}")
    try:
        canonical.relative_to(source)
    except ValueError:
        fail(f"Servo source file escaped its checkout: {relative}")
    return canonical


def verify_git_source(source: Path, pin: dict[str, Any]) -> dict[str, Any]:
    expected_commit = pin.get("commit")
    expected_tree = pin.get("tree")
    commit = one_line(git(source, "rev-parse", "HEAD"), "Servo HEAD")
    tree = one_line(git(source, "rev-parse", "HEAD^{tree}"), "Servo tree")
    if commit != expected_commit:
        fail(f"Servo HEAD {commit} does not match the pinned commit {expected_commit}")
    if tree != expected_tree:
        fail(f"Servo tree {tree} does not match the pinned tree {expected_tree}")

    status = git(source, "status", "--porcelain=v1", "--untracked-files=all").stdout
    if status:
        fail("Servo source checkout is dirty or contains untracked files")
    git(source, "diff", "--quiet")
    git(source, "diff", "--cached", "--quiet")

    tracked_raw = git(source, "ls-files", "-z").stdout
    tracked_files = [entry for entry in tracked_raw.split("\0") if entry]
    if not tracked_files:
        fail("Servo source checkout has no tracked files")
    if len(tracked_files) != len(set(tracked_files)):
        fail("Servo tracked file inventory contains duplicates")
    return {
        "commit": commit,
        "tree": tree,
        "clean": True,
        "tracked_file_count": len(tracked_files),
    }


def verify_reviewed_files(
    source: Path,
    topology: dict[str, Any],
) -> list[dict[str, Any]]:
    reviewed = topology.get("reviewed_upstream_files")
    if not isinstance(reviewed, list) or not reviewed:
        fail("Servo topology does not contain a reviewed upstream file inventory")
    records: list[dict[str, Any]] = []
    seen: set[str] = set()
    for item in reviewed:
        if not isinstance(item, dict):
            fail("reviewed upstream file entry must be an object")
        relative = item.get("path")
        expected_blob = item.get("blob_sha")
        role = item.get("role")
        if not isinstance(relative, str) or not isinstance(expected_blob, str):
            fail("reviewed upstream file entry is missing path or blob_sha")
        if relative in seen:
            fail(f"duplicate reviewed Servo file: {relative}")
        seen.add(relative)
        path = require_regular_source_file(source, relative)
        actual_blob = one_line(git(source, "hash-object", "--", relative), relative)
        if actual_blob != expected_blob:
            fail(
                f"reviewed Servo blob mismatch for {relative}: "
                f"expected {expected_blob}, found {actual_blob}"
            )
        digest, length = sha256_file(path)
        records.append(
            {
                "path": relative,
                "git_blob_sha1": actual_blob,
                "sha256": digest,
                "bytes": length,
                "role": role,
            }
        )
    records.sort(key=lambda item: item["path"])
    return records


def verify_required_source_files(source: Path) -> list[dict[str, Any]]:
    records: list[dict[str, Any]] = []
    for relative in REQUIRED_SOURCE_FILES:
        path = require_regular_source_file(source, relative)
        digest, length = sha256_file(path)
        records.append({"path": relative, "sha256": digest, "bytes": length})
    license_text = (source / "LICENSE").read_text(encoding="utf-8", errors="strict")
    if "Mozilla Public License Version 2.0" not in license_text:
        fail("Servo LICENSE does not contain the expected MPL-2.0 text")
    return records


def verify_patch_inventory(
    inventory: dict[str, Any],
    pin: dict[str, Any],
) -> tuple[list[dict[str, Any]], str]:
    if inventory.get("schema") != "hepta.browser.servo_patch_inventory.v1":
        fail("Servo patch inventory schema is invalid")
    if inventory.get("servo_commit") != pin.get("commit"):
        fail("Servo patch inventory commit differs from the source pin")
    if inventory.get("servo_tree") != pin.get("tree"):
        fail("Servo patch inventory tree differs from the source pin")
    patches = inventory.get("patches")
    if not isinstance(patches, list):
        fail("Servo patch inventory patches must be an array")

    files = sorted(PATCH_ROOT.glob("*.patch"), key=lambda path: path.name)
    if len(files) != len(patches):
        fail("Servo patch files and inventory entries differ")
    entries_by_file: dict[str, dict[str, Any]] = {}
    for entry in patches:
        if not isinstance(entry, dict):
            fail("Servo patch inventory entry must be an object")
        filename = entry.get("file")
        if not isinstance(filename, str) or filename in entries_by_file:
            fail("Servo patch inventory contains a missing or duplicate filename")
        entries_by_file[filename] = entry

    records: list[dict[str, Any]] = []
    for patch in files:
        if patch.name not in entries_by_file:
            fail(f"unregistered Servo patch file: {patch.name}")
        metadata = patch.lstat()
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
            fail(f"Servo patch must be a non-symlink regular file: {patch.name}")
        digest, length = sha256_file(patch)
        expected = entries_by_file[patch.name].get("patch_sha256")
        if digest != expected:
            fail(f"Servo patch SHA-256 mismatch: {patch.name}")
        records.append({"file": patch.name, "sha256": digest, "bytes": length})

    canonical_inventory = canonical_bytes(inventory)
    return records, sha256_bytes(canonical_inventory)


def inventory_digest(
    source: dict[str, Any],
    reviewed_files: Iterable[dict[str, Any]],
    required_files: Iterable[dict[str, Any]],
    patches: Iterable[dict[str, Any]],
) -> str:
    preimage = {
        "source": source,
        "reviewed_files": list(reviewed_files),
        "required_files": list(required_files),
        "patches": list(patches),
    }
    return sha256_bytes(canonical_bytes(preimage))


def build_receipt(source: Path) -> dict[str, Any]:
    pin = load_object(PIN_PATH)
    topology = load_object(TOPOLOGY_PATH)
    patch_inventory = load_object(PATCH_INVENTORY_PATH)

    if pin.get("integration_status") != "SOURCE_PIN_ONLY_NOT_IMPORTED":
        fail("Servo pin must remain source-only before this receipt is generated")
    source_binding = verify_git_source(source, pin)
    reviewed_files = verify_reviewed_files(source, topology)
    required_files = verify_required_source_files(source)
    patches, patch_inventory_sha256 = verify_patch_inventory(patch_inventory, pin)

    receipt = {
        "schema": "hepta.browser.servo_source_receipt.v1",
        "schema_version": 1,
        "source": {
            "repository": pin.get("repository"),
            "commit": source_binding["commit"],
            "tree": source_binding["tree"],
            "clean": source_binding["clean"],
            "tracked_file_count": source_binding["tracked_file_count"],
            "workspace_version": topology.get("source", {}).get("workspace_version"),
            "workspace_edition": topology.get("source", {}).get("workspace_edition"),
            "minimum_rust_version": topology.get("source", {}).get("minimum_rust_version"),
            "license": topology.get("source", {}).get("license"),
        },
        "integration": {
            "mode": topology.get("integration_topology", {}).get("mode"),
            "main_cargo_workspace_dependency": topology.get("integration_topology", {}).get(
                "main_cargo_workspace_dependency"
            ),
            "servo_types_exposed_to_hepta_callers": topology.get("integration_topology", {}).get(
                "servo_types_exposed_to_hepta_callers"
            ),
            "raw_webdriver_surface_exposed": topology.get("integration_topology", {}).get(
                "raw_webdriver_surface_exposed"
            ),
        },
        "reviewed_files": reviewed_files,
        "required_files": required_files,
        "patches": patches,
        "patch_inventory_sha256": patch_inventory_sha256,
        "inventory_sha256": inventory_digest(
            source_binding,
            reviewed_files,
            required_files,
            patches,
        ),
        "machine_local_paths_included": False,
        "network_access_used": False,
        "authority": AUTHORITY,
    }
    if receipt["source"]["license"] != "MPL-2.0":
        fail("Servo source topology license is not MPL-2.0")
    if any(value is not False for value in receipt["authority"].values()):
        fail("Servo source receipt attempted to enable authority")
    return receipt


def write_atomic(path_value: str, receipt: dict[str, Any]) -> None:
    destination = Path(path_value)
    if not destination.is_absolute():
        fail("--output must be an absolute path")
    parent = destination.parent.resolve(strict=True)
    canonical_destination = parent / destination.name
    if destination != canonical_destination:
        fail("--output must already be canonical and contain no symlink or '..' components")
    if destination.exists() and destination.is_symlink():
        fail("--output must not be a symlink")
    encoded = canonical_bytes(receipt)
    temporary = parent / f".{destination.name}.tmp-{os.getpid()}"
    try:
        flags = os.O_WRONLY | os.O_CREAT | os.O_EXCL
        descriptor = os.open(temporary, flags, 0o600)
        with os.fdopen(descriptor, "wb") as handle:
            handle.write(encoded)
            handle.flush()
            os.fsync(handle.fileno())
        os.replace(temporary, destination)
    except OSError as error:
        try:
            temporary.unlink(missing_ok=True)
        except OSError:
            pass
        fail(f"cannot write Servo source receipt: {error}")


def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--servo-source", required=True, help="canonical absolute Servo checkout")
    parser.add_argument("--output", required=True, help="canonical absolute receipt output path")
    return parser.parse_args()


def main() -> int:
    try:
        arguments = parse_arguments()
        source = require_canonical_source(arguments.servo_source)
        receipt = build_receipt(source)
        write_atomic(arguments.output, receipt)
    except ProvenanceError as error:
        print(f"HEPTA_SERVO_PROVENANCE=FAIL: {error}", file=sys.stderr)
        return 1
    print(
        json.dumps(
            {
                "schema": "hepta.browser.servo_source_receipt_result.v1",
                "status": "PASS_SOURCE_BOUND_QUALIFICATION_ONLY",
                "commit": receipt["source"]["commit"],
                "tree": receipt["source"]["tree"],
                "inventory_sha256": receipt["inventory_sha256"],
                "authority": "all_false",
            },
            sort_keys=True,
            separators=(",", ":"),
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
