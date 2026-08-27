#!/usr/bin/env python3
"""Generate or verify a fail-closed source receipt for the pinned Servo checkout.

The canonical CLI accepts only the repository-native Servo pin. Tests import the helper functions
with local fixture expectations; fixture receipts are never accepted as canonical Servo evidence.
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
import pathlib
import re
import subprocess
import sys
from dataclasses import dataclass
from typing import Iterable

SCHEMA = "hepta.servo.source_receipt.v1"
EXPECTED_REPOSITORY = "https://github.com/servo/servo"
EXPECTED_COMMIT = "0a48e298482659817eb50097df23841f2b8e3044"
EXPECTED_TREE = "b04d2f75b3217374d079d579c270177b57fa1389"
EXPECTED_LICENSE = "MPL-2.0"
MANIFEST_DOMAIN = b"hepta.servo.git-tree-manifest.v1"
RECEIPT_DOMAIN = b"hepta.servo.source-receipt.v1"
SHA40 = re.compile(r"^[0-9a-f]{40}$")
SHA64 = re.compile(r"^[0-9a-f]{64}$")


class ReceiptError(RuntimeError):
    """A fail-closed source qualification error."""


@dataclass(frozen=True)
class SourceExpectation:
    repository: str
    commit: str
    tree: str
    license_id: str = EXPECTED_LICENSE

    def validate(self) -> None:
        if normalize_repository(self.repository) != EXPECTED_REPOSITORY and self.repository == EXPECTED_REPOSITORY:
            raise ReceiptError("canonical Servo repository normalization failed")
        if not SHA40.fullmatch(self.commit) or not SHA40.fullmatch(self.tree):
            raise ReceiptError("source expectation commit/tree must be lowercase SHA-1 object IDs")
        if not self.repository or not self.license_id:
            raise ReceiptError("source expectation is incomplete")


CANONICAL_EXPECTATION = SourceExpectation(
    repository=EXPECTED_REPOSITORY,
    commit=EXPECTED_COMMIT,
    tree=EXPECTED_TREE,
)


def run_git(checkout: pathlib.Path, *arguments: str, input_bytes: bytes | None = None) -> bytes:
    command = ["git", "-C", os.fspath(checkout), *arguments]
    try:
        return subprocess.run(
            command,
            input=input_bytes,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=True,
        ).stdout
    except FileNotFoundError as error:
        raise ReceiptError("git executable is unavailable") from error
    except subprocess.CalledProcessError as error:
        stderr = error.stderr.decode("utf-8", errors="replace").strip()
        raise ReceiptError(f"git command failed ({' '.join(arguments)}): {stderr}") from error


def normalize_repository(value: str) -> str:
    value = value.strip().removesuffix("/").removesuffix(".git")
    prefixes = (
        "git@github.com:",
        "ssh://git@github.com/",
        "git://github.com/",
        "https://github.com/",
        "http://github.com/",
    )
    for prefix in prefixes:
        if value.startswith(prefix):
            value = value[len(prefix) :]
            break
    value = value.strip("/")
    if value.casefold() != "servo/servo":
        raise ReceiptError(f"unexpected Servo origin repository: {value!r}")
    return EXPECTED_REPOSITORY


def canonical_bytes(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode(
        "utf-8"
    )


def framed_digest(domain: bytes, fields: Iterable[bytes]) -> str:
    digest = hashlib.sha256()
    digest.update(len(domain).to_bytes(8, "big"))
    digest.update(domain)
    for field in fields:
        digest.update(len(field).to_bytes(8, "big"))
        digest.update(field)
    return digest.hexdigest()


def parse_tree_entries(raw: bytes) -> list[dict[str, str]]:
    entries: list[dict[str, str]] = []
    for record in raw.split(b"\0"):
        if not record:
            continue
        try:
            metadata, path_bytes = record.split(b"\t", 1)
            mode, object_type, object_id = metadata.decode("ascii").split(" ", 2)
            path = path_bytes.decode("utf-8")
        except (ValueError, UnicodeDecodeError) as error:
            raise ReceiptError("git tree contains a noncanonical entry") from error
        if not re.fullmatch(r"[0-7]{6}", mode):
            raise ReceiptError(f"git tree mode is noncanonical for {path!r}")
        if object_type not in {"blob", "commit"}:
            raise ReceiptError(f"unexpected recursive tree object type {object_type!r}")
        if not SHA40.fullmatch(object_id):
            raise ReceiptError(f"git tree object ID is invalid for {path!r}")
        if not path or path.startswith("/") or "\x00" in path:
            raise ReceiptError("git tree path is invalid")
        entries.append(
            {
                "mode": mode,
                "object_type": object_type,
                "object_id": object_id,
                "path": path,
            }
        )
    if not entries:
        raise ReceiptError("git source tree is empty")
    if entries != sorted(entries, key=lambda entry: entry["path"].encode("utf-8")):
        raise ReceiptError("git ls-tree output is not path-sorted")
    paths = [entry["path"] for entry in entries]
    if len(paths) != len(set(paths)):
        raise ReceiptError("git source tree contains duplicate paths")
    return entries


def tree_manifest_digest(entries: list[dict[str, str]]) -> str:
    fields = [
        b"\0".join(
            (
                entry["mode"].encode("ascii"),
                entry["object_type"].encode("ascii"),
                entry["object_id"].encode("ascii"),
                entry["path"].encode("utf-8"),
            )
        )
        for entry in entries
    ]
    return framed_digest(MANIFEST_DOMAIN, fields)


def commit_has_embedded_signature(checkout: pathlib.Path, commit: str) -> bool:
    raw = run_git(checkout, "cat-file", "commit", commit)
    header = raw.split(b"\n\n", 1)[0]
    return b"\ngpgsig " in b"\n" + header


def license_facts(checkout: pathlib.Path, commit: str, license_id: str) -> dict[str, object]:
    try:
        license_bytes = run_git(checkout, "show", f"{commit}:LICENSE")
    except ReceiptError as error:
        raise ReceiptError("pinned Servo tree lacks LICENSE") from error
    if b"Mozilla Public License Version 2.0" not in license_bytes:
        raise ReceiptError("Servo LICENSE does not contain the expected MPL-2.0 marker")
    return {
        "spdx_id": license_id,
        "path": "LICENSE",
        "bytes": len(license_bytes),
        "sha256": hashlib.sha256(license_bytes).hexdigest(),
    }


def collect_source(
    checkout: pathlib.Path,
    expectation: SourceExpectation,
    *,
    require_clean: bool = True,
) -> dict[str, object]:
    expectation.validate()
    checkout = checkout.resolve(strict=True)
    if not checkout.is_dir():
        raise ReceiptError("Servo checkout is not a directory")
    inside = run_git(checkout, "rev-parse", "--is-inside-work-tree").decode("ascii").strip()
    if inside != "true":
        raise ReceiptError("Servo source path is not a Git worktree")

    commit = run_git(checkout, "rev-parse", "HEAD").decode("ascii").strip()
    tree = run_git(checkout, "rev-parse", "HEAD^{tree}").decode("ascii").strip()
    if commit != expectation.commit or tree != expectation.tree:
        raise ReceiptError(
            f"Servo source pin mismatch: observed {commit}/{tree}, expected "
            f"{expectation.commit}/{expectation.tree}"
        )

    status = run_git(checkout, "status", "--porcelain=v1", "--untracked-files=all")
    if require_clean and status:
        raise ReceiptError("Servo checkout is dirty or contains untracked files")

    origin = normalize_repository(
        run_git(checkout, "config", "--get", "remote.origin.url").decode("utf-8").strip()
    )
    if origin != normalize_repository(expectation.repository):
        raise ReceiptError("Servo origin does not match the expected repository")

    entries = parse_tree_entries(
        run_git(checkout, "ls-tree", "-r", "-z", "--full-tree", commit)
    )
    submodules = [entry for entry in entries if entry["object_type"] == "commit"]
    blob_entries = [entry for entry in entries if entry["object_type"] == "blob"]
    symlink_entries = [entry for entry in entries if entry["mode"] == "120000"]

    return {
        "repository": origin,
        "commit": commit,
        "tree": tree,
        "clean_worktree": not bool(status),
        "embedded_commit_signature": commit_has_embedded_signature(checkout, commit),
        "tree_manifest": {
            "algorithm": "sha256-framed-git-ls-tree-v1",
            "sha256": tree_manifest_digest(entries),
            "entry_count": len(entries),
            "blob_count": len(blob_entries),
            "submodule_count": len(submodules),
            "symlink_count": len(symlink_entries),
            "path_utf8_bytes": sum(len(entry["path"].encode("utf-8")) for entry in entries),
        },
        "submodules": submodules,
        "license": license_facts(checkout, commit, expectation.license_id),
    }


def receipt_payload(
    checkout: pathlib.Path,
    expectation: SourceExpectation,
    captured_at: str,
) -> dict[str, object]:
    source = collect_source(checkout, expectation)
    payload: dict[str, object] = {
        "schema": SCHEMA,
        "phase": "DEVELOPMENT",
        "claim_level": "SOURCE_PIN_AND_TREE_ONLY",
        "captured_at_utc": captured_at,
        "source": source,
        "artifact": {
            "source_archive_created": False,
            "source_archive_sha256": None,
            "worker_artifact_built": False,
            "worker_artifact_sha256": None,
            "sbom_created": False,
        },
        "authority": {
            "machine_authority": False,
            "runtime_authority": False,
            "production_caller": False,
            "production_writer": False,
            "effect_authority": False,
            "external_effect": False,
            "operator_acceptance": False,
            "promotion": False,
            "release_qualified": False,
        },
        "decision": "SOURCE_PIN_VERIFIED_BUILD_NOT_QUALIFIED",
    }
    receipt_id = framed_digest(RECEIPT_DOMAIN, [canonical_bytes(payload)])
    payload["receipt_id"] = f"servo-source-receipt:v1:{receipt_id}"
    return payload


def parse_canonical_receipt(path: pathlib.Path) -> dict[str, object]:
    try:
        raw = path.read_bytes()
        value = json.loads(raw)
    except (OSError, json.JSONDecodeError) as error:
        raise ReceiptError(f"cannot decode receipt: {error}") from error
    if not isinstance(value, dict):
        raise ReceiptError("source receipt must be a JSON object")
    if canonical_bytes(value) != raw:
        raise ReceiptError("source receipt is not compact canonical JSON")
    return value


def validate_receipt_shape(receipt: dict[str, object]) -> None:
    if receipt.get("schema") != SCHEMA:
        raise ReceiptError("source receipt schema is unsupported")
    if receipt.get("phase") != "DEVELOPMENT":
        raise ReceiptError("source receipt phase is not DEVELOPMENT")
    if receipt.get("claim_level") != "SOURCE_PIN_AND_TREE_ONLY":
        raise ReceiptError("source receipt claim level is invalid")
    if receipt.get("decision") != "SOURCE_PIN_VERIFIED_BUILD_NOT_QUALIFIED":
        raise ReceiptError("source receipt overclaims or has an unknown decision")
    authority = receipt.get("authority")
    if not isinstance(authority, dict) or any(value is not False for value in authority.values()):
        raise ReceiptError("source receipt contains positive or non-boolean authority")
    artifact = receipt.get("artifact")
    if not isinstance(artifact, dict):
        raise ReceiptError("source receipt lacks artifact posture")
    expected_artifact = {
        "source_archive_created": False,
        "source_archive_sha256": None,
        "worker_artifact_built": False,
        "worker_artifact_sha256": None,
        "sbom_created": False,
    }
    if artifact != expected_artifact:
        raise ReceiptError("source receipt overclaims an archive, artifact, or SBOM")
    receipt_id = receipt.get("receipt_id")
    if not isinstance(receipt_id, str) or not receipt_id.startswith("servo-source-receipt:v1:"):
        raise ReceiptError("source receipt ID is invalid")
    digest = receipt_id.removeprefix("servo-source-receipt:v1:")
    if not SHA64.fullmatch(digest):
        raise ReceiptError("source receipt ID digest is invalid")
    without_id = dict(receipt)
    without_id.pop("receipt_id")
    expected_digest = framed_digest(RECEIPT_DOMAIN, [canonical_bytes(without_id)])
    if digest != expected_digest:
        raise ReceiptError("source receipt ID does not match its canonical payload")


def verify_receipt(
    checkout: pathlib.Path,
    receipt_path: pathlib.Path,
    expectation: SourceExpectation,
) -> dict[str, object]:
    receipt = parse_canonical_receipt(receipt_path)
    validate_receipt_shape(receipt)
    observed = collect_source(checkout, expectation)
    if receipt.get("source") != observed:
        raise ReceiptError("current Servo checkout does not match the sealed source facts")
    return receipt


def utc_now() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def write_exclusive(path: pathlib.Path, content: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    flags = os.O_WRONLY | os.O_CREAT | os.O_EXCL
    descriptor = os.open(path, flags, 0o600)
    try:
        with os.fdopen(descriptor, "wb", closefd=True) as output:
            output.write(content)
            output.flush()
            os.fsync(output.fileno())
    except Exception:
        try:
            path.unlink()
        except OSError:
            pass
        raise


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    snapshot = subparsers.add_parser("snapshot", help="create a canonical source-only receipt")
    snapshot.add_argument("--checkout", type=pathlib.Path, required=True)
    snapshot.add_argument("--output", type=pathlib.Path, required=True)
    snapshot.add_argument("--captured-at", default=None)

    verify = subparsers.add_parser("verify", help="verify a receipt against the current checkout")
    verify.add_argument("--checkout", type=pathlib.Path, required=True)
    verify.add_argument("--receipt", type=pathlib.Path, required=True)
    return parser


def main(argv: list[str] | None = None) -> int:
    arguments = build_parser().parse_args(argv)
    try:
        if arguments.command == "snapshot":
            captured_at = arguments.captured_at or utc_now()
            receipt = receipt_payload(arguments.checkout, CANONICAL_EXPECTATION, captured_at)
            encoded = canonical_bytes(receipt)
            write_exclusive(arguments.output, encoded)
            result = {
                "status": "SOURCE_PIN_VERIFIED_BUILD_NOT_QUALIFIED",
                "receipt": os.fspath(arguments.output),
                "receipt_sha256": hashlib.sha256(encoded).hexdigest(),
                "receipt_id": receipt["receipt_id"],
            }
        else:
            receipt = verify_receipt(
                arguments.checkout,
                arguments.receipt,
                CANONICAL_EXPECTATION,
            )
            result = {
                "status": "SOURCE_RECEIPT_VERIFIED_BUILD_NOT_QUALIFIED",
                "receipt": os.fspath(arguments.receipt),
                "receipt_sha256": hashlib.sha256(arguments.receipt.read_bytes()).hexdigest(),
                "receipt_id": receipt["receipt_id"],
            }
    except (ReceiptError, OSError) as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1

    print(json.dumps(result, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
