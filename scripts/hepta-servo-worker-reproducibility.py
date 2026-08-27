#!/usr/bin/env python3
"""Compare two independently produced Servo worker build outputs byte-for-byte.

This standard-library tool performs no build, network access, or execution. It
accepts an explicit compact-canonical manifest and two canonical replica roots,
then seals a fail-closed reproducibility receipt only when every declared output
is byte-identical. A successful receipt does not qualify the Servo runtime and
never grants product, effect, network, operator, promotion, or release authority.
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
from typing import Any

SERVO_COMMIT = "0a48e298482659817eb50097df23841f2b8e3044"
SERVO_TREE = "b04d2f75b3217374d079d579c270177b57fa1389"
MANIFEST_SCHEMA = "hepta.servo.worker_reproducibility_manifest.v1"
RECEIPT_SCHEMA = "hepta.servo.worker_reproducibility_receipt.v1"
RECEIPT_DOMAIN = b"hepta.servo.worker-reproducibility-receipt.v1"
AGGREGATE_DOMAIN = b"hepta.servo.worker-reproducibility-aggregate.v1"
SHA64 = re.compile(r"^[0-9a-f]{64}$")
STABLE_ID = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._:+@=-]{0,127}$")
UTC_SECONDS = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$")
TARGET = re.compile(r"^[A-Za-z0-9_][A-Za-z0-9._+-]{0,127}$")
MAX_OUTPUTS = 256
MAX_TOTAL_BYTES = 8 * 1024 * 1024 * 1024
MAX_FILE_BYTES = 4 * 1024 * 1024 * 1024
OUTPUT_TYPES = {"binary", "canonical_json", "text"}

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

FORBIDDEN_TRUE_KEYS = {
    *AUTHORITY,
    "runtime_qualified",
    "network_access_during_build",
    "worker_tcp_listener",
    "worker_http_surface",
    "worker_external_network",
    "worker_credential_export",
    "worker_production_authority",
    "worker_effect_authority",
    "production_authority",
    "binary_distribution_authorized",
}


class ReproducibilityError(RuntimeError):
    """Fail-closed reproducibility validation error."""


def fail(message: str) -> None:
    raise ReproducibilityError(message)


def reject_duplicate_keys(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            fail(f"duplicate JSON key {key!r}")
        result[key] = value
    return result


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
    try:
        raw = path.read_bytes()
        value = json.loads(raw, object_pairs_hook=reject_duplicate_keys)
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot decode {label}: {error}")
    if not isinstance(value, dict):
        fail(f"{label} must contain one JSON object")
    if canonical_required and raw != canonical(value):
        fail(f"{label} is not compact canonical JSON")
    return value, raw


def framed(domain: bytes, *fields: bytes) -> str:
    digest = hashlib.sha256()
    digest.update(len(domain).to_bytes(8, "big"))
    digest.update(domain)
    for field in fields:
        digest.update(len(field).to_bytes(8, "big"))
        digest.update(field)
    return digest.hexdigest()


def timestamp(value: str | None) -> str:
    value = value or dt.datetime.now(dt.timezone.utc).replace(microsecond=0).strftime(
        "%Y-%m-%dT%H:%M:%SZ"
    )
    if not UTC_SECONDS.fullmatch(value):
        fail("captured_at_utc must use whole-second RFC3339 UTC")
    try:
        dt.datetime.strptime(value, "%Y-%m-%dT%H:%M:%SZ")
    except ValueError as error:
        fail(f"captured_at_utc is not a real UTC timestamp: {error}")
    return value


def sha256_file(path: pathlib.Path) -> tuple[str, int]:
    digest = hashlib.sha256()
    size = 0
    try:
        with path.open("rb") as stream:
            while True:
                block = stream.read(1024 * 1024)
                if not block:
                    break
                digest.update(block)
                size += len(block)
                if size > MAX_FILE_BYTES:
                    fail(f"output exceeds per-file bound: {path.name}")
    except OSError as error:
        fail(f"cannot hash output {path.name}: {error}")
    return digest.hexdigest(), size


def validate_sha(value: Any, label: str) -> str:
    if not isinstance(value, str) or not SHA64.fullmatch(value):
        fail(f"{label} must be lowercase SHA-256")
    return value


def validate_id(value: Any, label: str) -> str:
    if not isinstance(value, str) or not STABLE_ID.fullmatch(value):
        fail(f"{label} is not a stable identifier")
    return value


def validate_relative_path(value: Any) -> str:
    if not isinstance(value, str) or not value or "\x00" in value or "\\" in value:
        fail("output path is empty or platform-ambiguous")
    path = pathlib.PurePosixPath(value)
    if path.is_absolute() or any(part in {"", ".", ".."} for part in path.parts):
        fail(f"output path is unsafe: {value!r}")
    if len(value.encode("utf-8")) > 1024:
        fail("output path is oversized")
    return value


def require_root(value: pathlib.Path, label: str) -> pathlib.Path:
    if not value.is_absolute():
        fail(f"{label} must be an absolute path")
    try:
        canonical_root = value.resolve(strict=True)
        metadata = value.lstat()
    except OSError as error:
        fail(f"{label} is unavailable: {error}")
    if canonical_root != value or stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        fail(f"{label} must be a canonical non-symlink directory")
    return canonical_root


def require_output(root: pathlib.Path, relative: str) -> pathlib.Path:
    path = root / relative
    try:
        canonical_path = path.resolve(strict=True)
        metadata = path.lstat()
    except OSError as error:
        fail(f"declared output is unavailable ({relative}): {error}")
    try:
        canonical_path.relative_to(root)
    except ValueError:
        fail(f"declared output escaped replica root: {relative}")
    if canonical_path != path:
        fail(f"declared output contains a symlink component: {relative}")
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail(f"declared output must be a non-symlink regular file: {relative}")
    if getattr(metadata, "st_nlink", 1) != 1:
        fail(f"declared output must have exactly one hard link: {relative}")
    if metadata.st_mode & (stat.S_IWGRP | stat.S_IWOTH):
        fail(f"declared output must not be group/world writable: {relative}")
    return canonical_path


def reject_positive_posture(value: Any, location: str = "$") -> None:
    if isinstance(value, dict):
        for key, item in value.items():
            child = f"{location}.{key}"
            if key in FORBIDDEN_TRUE_KEYS and item is not False:
                fail(f"positive runtime/authority posture at {child}")
            reject_positive_posture(item, child)
    elif isinstance(value, list):
        for index, item in enumerate(value):
            reject_positive_posture(item, f"{location}[{index}]")


def validate_canonical_json_output(path: pathlib.Path, relative: str) -> None:
    value, _ = load_json(path, f"canonical JSON output {relative}")
    reject_positive_posture(value)


def validate_text_output(path: pathlib.Path, relative: str) -> None:
    try:
        raw = path.read_bytes()
        text = raw.decode("utf-8", errors="strict")
    except (OSError, UnicodeError) as error:
        fail(f"text output is not strict UTF-8 ({relative}): {error}")
    if "\x00" in text:
        fail(f"text output contains NUL: {relative}")


def validate_manifest(path: pathlib.Path) -> tuple[dict[str, Any], bytes]:
    manifest, raw = load_json(path, "reproducibility manifest")
    expected_keys = {
        "schema",
        "schema_version",
        "servo_commit",
        "servo_tree",
        "target_triple",
        "build_manifest_sha256",
        "outputs",
        "comparison_policy",
        "authority",
    }
    if set(manifest) != expected_keys:
        fail("reproducibility manifest field set is incomplete or unknown")
    if manifest.get("schema") != MANIFEST_SCHEMA or manifest.get("schema_version") != 1:
        fail("reproducibility manifest schema/version is unsupported")
    if manifest.get("servo_commit") != SERVO_COMMIT or manifest.get("servo_tree") != SERVO_TREE:
        fail("reproducibility manifest Servo source pin drifted")
    target = manifest.get("target_triple")
    if not isinstance(target, str) or not TARGET.fullmatch(target):
        fail("reproducibility manifest target triple is invalid")
    validate_sha(manifest.get("build_manifest_sha256"), "build manifest SHA-256")
    if manifest.get("comparison_policy") != {
        "require_byte_identical": True,
        "allow_missing_optional": False,
        "allow_explained_differences": False,
    }:
        fail("reproducibility comparison policy is not strict byte-identical")
    if manifest.get("authority") != AUTHORITY:
        fail("reproducibility manifest authority posture is open")
    outputs = manifest.get("outputs")
    if not isinstance(outputs, list) or not (1 <= len(outputs) <= MAX_OUTPUTS):
        fail("reproducibility manifest outputs must be a bounded nonempty array")
    normalized: list[tuple[str, str]] = []
    for item in outputs:
        if not isinstance(item, dict) or set(item) != {"path", "type"}:
            fail("reproducibility output entry must contain exactly path and type")
        relative = validate_relative_path(item.get("path"))
        output_type = item.get("type")
        if output_type not in OUTPUT_TYPES:
            fail(f"unsupported reproducibility output type: {output_type!r}")
        normalized.append((relative, output_type))
    if normalized != sorted(set(normalized)):
        fail("reproducibility outputs must be sorted and unique by path/type")
    return manifest, raw


def inspect_replica(
    root: pathlib.Path,
    outputs: list[dict[str, str]],
) -> tuple[list[dict[str, Any]], str, int]:
    records: list[dict[str, Any]] = []
    total = 0
    for item in outputs:
        relative = item["path"]
        output_type = item["type"]
        path = require_output(root, relative)
        digest, size = sha256_file(path)
        total += size
        if total > MAX_TOTAL_BYTES:
            fail("replica output inventory exceeds the total byte bound")
        if output_type == "canonical_json":
            validate_canonical_json_output(path, relative)
        elif output_type == "text":
            validate_text_output(path, relative)
        records.append(
            {
                "path": relative,
                "type": output_type,
                "sha256": digest,
                "bytes": size,
            }
        )
    aggregate = framed(AGGREGATE_DOMAIN, *(canonical(record) for record in records))
    return records, aggregate, total


def build_receipt(
    left_root: pathlib.Path,
    right_root: pathlib.Path,
    manifest_path: pathlib.Path,
    left_replica_id: str,
    right_replica_id: str,
    captured_at: str | None,
) -> dict[str, Any]:
    left_root = require_root(left_root, "left replica root")
    right_root = require_root(right_root, "right replica root")
    if left_root == right_root:
        fail("reproducibility replicas must use distinct roots")
    left_replica_id = validate_id(left_replica_id, "left replica ID")
    right_replica_id = validate_id(right_replica_id, "right replica ID")
    if left_replica_id == right_replica_id:
        fail("reproducibility replica IDs must differ")

    manifest, manifest_raw = validate_manifest(manifest_path)
    outputs = manifest["outputs"]
    left_records, left_aggregate, left_total = inspect_replica(left_root, outputs)
    right_records, right_aggregate, right_total = inspect_replica(right_root, outputs)
    if left_records != right_records or left_aggregate != right_aggregate or left_total != right_total:
        fail("independent worker build outputs are not byte-identical")

    receipt: dict[str, Any] = {
        "schema": RECEIPT_SCHEMA,
        "schema_version": 1,
        "phase": "DEVELOPMENT",
        "claim_level": "BYTE_IDENTICAL_WORKER_BUILD_REPRODUCIBILITY_ONLY",
        "captured_at_utc": timestamp(captured_at),
        "source_binding": {
            "servo_commit": SERVO_COMMIT,
            "servo_tree": SERVO_TREE,
            "target_triple": manifest["target_triple"],
            "build_manifest_sha256": manifest["build_manifest_sha256"],
            "reproducibility_manifest_sha256": hashlib.sha256(manifest_raw).hexdigest(),
        },
        "replicas": {
            "left": {
                "replica_id": left_replica_id,
                "aggregate_sha256": left_aggregate,
                "file_count": len(left_records),
                "total_bytes": left_total,
            },
            "right": {
                "replica_id": right_replica_id,
                "aggregate_sha256": right_aggregate,
                "file_count": len(right_records),
                "total_bytes": right_total,
            },
        },
        "outputs": left_records,
        "comparison": {
            "byte_identical": True,
            "aggregate_equal": True,
            "file_inventory_equal": True,
            "file_count_equal": True,
            "total_bytes_equal": True,
            "explained_differences_allowed": False,
        },
        "runtime": {
            "worker_executed": False,
            "servo_runtime_qualified": False,
            "external_network_used": False,
        },
        "authority": AUTHORITY,
        "decision": "WORKER_BUILD_BYTE_IDENTICAL_RUNTIME_NOT_QUALIFIED",
    }
    receipt["receipt_id"] = "hepta-servo-worker-reproducibility:v1:" + framed(
        RECEIPT_DOMAIN, canonical(receipt)
    )
    return receipt


def validate_receipt(
    receipt_path: pathlib.Path,
    left_root: pathlib.Path,
    right_root: pathlib.Path,
    manifest_path: pathlib.Path,
) -> dict[str, Any]:
    receipt, receipt_raw = load_json(receipt_path, "reproducibility receipt")
    expected_keys = {
        "schema",
        "schema_version",
        "phase",
        "claim_level",
        "captured_at_utc",
        "source_binding",
        "replicas",
        "outputs",
        "comparison",
        "runtime",
        "authority",
        "decision",
        "receipt_id",
    }
    if set(receipt) != expected_keys:
        fail("reproducibility receipt field set is incomplete or unknown")
    if receipt.get("schema") != RECEIPT_SCHEMA or receipt.get("schema_version") != 1:
        fail("reproducibility receipt schema/version is unsupported")
    if receipt.get("phase") != "DEVELOPMENT" or receipt.get("claim_level") != (
        "BYTE_IDENTICAL_WORKER_BUILD_REPRODUCIBILITY_ONLY"
    ):
        fail("reproducibility receipt phase/claim is invalid")
    timestamp(receipt.get("captured_at_utc"))
    if receipt.get("authority") != AUTHORITY:
        fail("reproducibility receipt authority posture is open")
    if receipt.get("runtime") != {
        "worker_executed": False,
        "servo_runtime_qualified": False,
        "external_network_used": False,
    }:
        fail("reproducibility receipt runtime posture is open")
    if receipt.get("comparison") != {
        "byte_identical": True,
        "aggregate_equal": True,
        "file_inventory_equal": True,
        "file_count_equal": True,
        "total_bytes_equal": True,
        "explained_differences_allowed": False,
    }:
        fail("reproducibility receipt comparison posture is invalid")
    if receipt.get("decision") != "WORKER_BUILD_BYTE_IDENTICAL_RUNTIME_NOT_QUALIFIED":
        fail("reproducibility receipt decision is invalid")

    identifier = receipt.get("receipt_id")
    prefix = "hepta-servo-worker-reproducibility:v1:"
    if not isinstance(identifier, str) or not identifier.startswith(prefix):
        fail("reproducibility receipt ID prefix is invalid")
    identifier_digest = identifier.removeprefix(prefix)
    validate_sha(identifier_digest, "reproducibility receipt ID")
    without_id = dict(receipt)
    without_id.pop("receipt_id")
    if identifier_digest != framed(RECEIPT_DOMAIN, canonical(without_id)):
        fail("reproducibility receipt ID does not bind its payload")

    left = receipt.get("replicas", {}).get("left", {})
    right = receipt.get("replicas", {}).get("right", {})
    recomputed = build_receipt(
        left_root,
        right_root,
        manifest_path,
        left.get("replica_id"),
        right.get("replica_id"),
        receipt["captured_at_utc"],
    )
    if canonical(recomputed) != receipt_raw:
        fail("reproducibility receipt differs from exact recomputation")
    return receipt


def write_new(path: pathlib.Path, data: bytes) -> None:
    if not path.is_absolute():
        fail("output path must be absolute")
    try:
        parent = path.parent.resolve(strict=True)
    except OSError as error:
        fail(f"output parent is unavailable: {error}")
    canonical_path = parent / path.name
    if canonical_path != path:
        fail("output path must be canonical")
    if path.exists():
        fail("output path already exists; reproducibility receipts are create-only")
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    try:
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(data)
            stream.flush()
            os.fsync(stream.fileno())
    except Exception:
        path.unlink(missing_ok=True)
        raise


def contract() -> dict[str, Any]:
    source = pathlib.Path(__file__).read_text(encoding="utf-8")
    for forbidden in (
        "import socket",
        "import urllib",
        "import requests",
        "import subprocess",
        "os.system",
    ):
        if forbidden in source:
            fail(f"reproducibility tool contains forbidden execution/network surface: {forbidden}")
    if any(AUTHORITY.values()):
        fail("reproducibility tool authority posture is open")
    return {
        "schema": "hepta.servo.worker_reproducibility_contract.v1",
        "status": "PASS_FIXTURE_CONTRACT_ONLY",
        "servo_commit": SERVO_COMMIT,
        "servo_tree": SERVO_TREE,
        "byte_identical_required": True,
        "real_worker_build_compared": False,
        "runtime_qualified": False,
        "authority": AUTHORITY,
    }


def self_test() -> dict[str, Any]:
    import tempfile

    tests: list[str] = []
    with tempfile.TemporaryDirectory() as directory:
        root = pathlib.Path(directory)
        left = root / "left"
        right = root / "right"
        left.mkdir()
        right.mkdir()
        for replica in (left, right):
            (replica / "bin").mkdir()
            (replica / "meta").mkdir()
            (replica / "bin/worker").write_bytes(b"fixture-worker\x00")
            (replica / "meta/artifact.json").write_bytes(
                canonical(
                    {
                        "schema": "fixture.artifact.v1",
                        "runtime_qualified": False,
                        "worker_external_network": False,
                        "worker_production_authority": False,
                    }
                )
            )
            (replica / "meta/build.txt").write_text("fixture-build\n", encoding="utf-8")
        manifest = {
            "schema": MANIFEST_SCHEMA,
            "schema_version": 1,
            "servo_commit": SERVO_COMMIT,
            "servo_tree": SERVO_TREE,
            "target_triple": "x86_64-unknown-linux-gnu",
            "build_manifest_sha256": "1" * 64,
            "outputs": [
                {"path": "bin/worker", "type": "binary"},
                {"path": "meta/artifact.json", "type": "canonical_json"},
                {"path": "meta/build.txt", "type": "text"},
            ],
            "comparison_policy": {
                "require_byte_identical": True,
                "allow_missing_optional": False,
                "allow_explained_differences": False,
            },
            "authority": AUTHORITY,
        }
        manifest_path = root / "manifest.json"
        manifest_path.write_bytes(canonical(manifest))
        receipt = build_receipt(
            left,
            right,
            manifest_path,
            "replica-a",
            "replica-b",
            "2026-08-28T00:00:00Z",
        )
        receipt_path = root / "receipt.json"
        write_new(receipt_path, canonical(receipt))
        validate_receipt(receipt_path, left, right, manifest_path)
        tests.append("byte_identical_receipt_recomputed")

        try:
            write_new(receipt_path, canonical(receipt))
            fail("create-only receipt overwrite passed")
        except ReproducibilityError:
            tests.append("create_only_receipt_enforced")

        (right / "bin/worker").write_bytes(b"drifted-worker")
        try:
            build_receipt(
                left,
                right,
                manifest_path,
                "replica-a",
                "replica-b",
                "2026-08-28T00:01:00Z",
            )
            fail("artifact drift passed")
        except ReproducibilityError:
            tests.append("artifact_drift_rejected")
        (right / "bin/worker").write_bytes(b"fixture-worker\x00")

        bad_value = {
            "schema": "fixture.artifact.v1",
            "runtime_qualified": True,
        }
        (right / "meta/artifact.json").write_bytes(canonical(bad_value))
        (left / "meta/artifact.json").write_bytes(canonical(bad_value))
        try:
            build_receipt(
                left,
                right,
                manifest_path,
                "replica-a",
                "replica-b",
                "2026-08-28T00:02:00Z",
            )
            fail("positive runtime posture passed")
        except ReproducibilityError:
            tests.append("positive_runtime_posture_rejected")

        bad_manifest = dict(manifest)
        bad_manifest["outputs"] = list(reversed(manifest["outputs"]))
        bad_manifest_path = root / "bad-manifest.json"
        bad_manifest_path.write_bytes(canonical(bad_manifest))
        try:
            validate_manifest(bad_manifest_path)
            fail("unsorted manifest passed")
        except ReproducibilityError:
            tests.append("unsorted_manifest_rejected")

    if len(tests) != 5:
        fail("unexpected reproducibility self-test count")
    return {
        "schema": "hepta.servo.worker_reproducibility_self_test.v1",
        "status": "PASS_LOCAL_FIXTURE_ONLY",
        "tests": tests,
        "test_count": len(tests),
        "real_worker_build_compared": False,
        "runtime_qualified": False,
        "authority": AUTHORITY,
    }


def parser() -> argparse.ArgumentParser:
    root = argparse.ArgumentParser(description=__doc__)
    sub = root.add_subparsers(dest="command", required=True)

    create = sub.add_parser("create")
    create.add_argument("--left-root", type=pathlib.Path, required=True)
    create.add_argument("--right-root", type=pathlib.Path, required=True)
    create.add_argument("--manifest", type=pathlib.Path, required=True)
    create.add_argument("--left-replica-id", required=True)
    create.add_argument("--right-replica-id", required=True)
    create.add_argument("--captured-at")
    create.add_argument("--output", type=pathlib.Path, required=True)

    verify = sub.add_parser("verify")
    verify.add_argument("--left-root", type=pathlib.Path, required=True)
    verify.add_argument("--right-root", type=pathlib.Path, required=True)
    verify.add_argument("--manifest", type=pathlib.Path, required=True)
    verify.add_argument("--receipt", type=pathlib.Path, required=True)

    sub.add_parser("contract")
    sub.add_parser("self-test")
    return root


def main(argv: list[str] | None = None) -> int:
    arguments = parser().parse_args(argv)
    try:
        if arguments.command == "create":
            receipt = build_receipt(
                arguments.left_root,
                arguments.right_root,
                arguments.manifest,
                arguments.left_replica_id,
                arguments.right_replica_id,
                arguments.captured_at,
            )
            write_new(arguments.output, canonical(receipt))
            result = {
                "status": receipt["decision"],
                "receipt_id": receipt["receipt_id"],
                "output_count": len(receipt["outputs"]),
            }
        elif arguments.command == "verify":
            receipt = validate_receipt(
                arguments.receipt,
                arguments.left_root,
                arguments.right_root,
                arguments.manifest,
            )
            result = {
                "status": "WORKER_BUILD_REPRODUCIBILITY_RECEIPT_VERIFIED_RUNTIME_NOT_QUALIFIED",
                "receipt_id": receipt["receipt_id"],
                "output_count": len(receipt["outputs"]),
            }
        elif arguments.command == "contract":
            result = contract()
        else:
            result = self_test()
    except (ReproducibilityError, OSError, UnicodeError) as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    sys.exit(main())
