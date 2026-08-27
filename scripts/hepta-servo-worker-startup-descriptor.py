#!/usr/bin/env python3
"""Create an immutable graph-bound Servo worker startup candidate descriptor.

The descriptor compiler performs no source fetch, build, network access, process
launch, executable loading, or worker execution. It binds one existing strict
receipt-graph verification, its manifest, the selected worker bytes, a browser
session/generation/owner epoch, and a private transport class. The descriptor
explicitly keeps launch authorization and every authority field false and never
contains a startup capability or host nonce.
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
GRAPH_MANIFEST_SCHEMA = "hepta.servo.worker_receipt_graph_manifest.v1"
GRAPH_VERIFICATION_SCHEMA = "hepta.servo.worker_receipt_graph_verification.v1"
DESCRIPTOR_SCHEMA = "hepta.servo.worker_startup_descriptor.v1"
GRAPH_RECEIPT_DOMAIN = b"hepta.servo.worker-receipt-graph-verification.v1"
DESCRIPTOR_DOMAIN = b"hepta.servo.worker-startup-descriptor.v1"
SHA64 = re.compile(r"^[0-9a-f]{64}$")
SESSION64 = re.compile(r"^[0-9a-f]{64}$")
UTC_SECONDS = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$")
TRANSPORTS = {"unix_inherited_socketpair", "windows_sid_named_pipe"}
MAX_WORKER_BYTES = 4 * 1024 * 1024 * 1024

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
    "g5_allowed": False,
    "execute_allowed": False,
    "promotion": False,
    "release_qualified": False,
}

FORBIDDEN_DESCRIPTOR_KEYS = {
    "startup_capability",
    "startup_capability_sha256",
    "host_nonce",
    "raw_capability",
    "credential",
    "secret",
}


class DescriptorError(RuntimeError):
    """Fail-closed startup descriptor error."""


def fail(message: str) -> None:
    raise DescriptorError(message)


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


def load_json(path: pathlib.Path, label: str) -> tuple[dict[str, Any], bytes]:
    try:
        raw = path.read_bytes()
        value = json.loads(raw, object_pairs_hook=reject_duplicate_keys)
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot decode {label}: {error}")
    if not isinstance(value, dict):
        fail(f"{label} must contain one JSON object")
    if raw != canonical(value):
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


def require_sha(value: Any, label: str) -> str:
    if not isinstance(value, str) or not SHA64.fullmatch(value):
        fail(f"{label} must be lowercase SHA-256")
    return value


def validate_relative_path(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value or "\x00" in value or "\\" in value:
        fail(f"{label} is empty or platform-ambiguous")
    path = pathlib.PurePosixPath(value)
    if path.is_absolute() or any(part in {"", ".", ".."} for part in path.parts):
        fail(f"{label} is unsafe: {value!r}")
    if len(value.encode("utf-8")) > 1024:
        fail(f"{label} is oversized")
    return value


def require_root(root: pathlib.Path) -> pathlib.Path:
    if not root.is_absolute():
        fail("--root must be an absolute path")
    try:
        canonical_root = root.resolve(strict=True)
        metadata = root.lstat()
    except OSError as error:
        fail(f"packet root is unavailable: {error}")
    if canonical_root != root or stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        fail("packet root must be a canonical non-symlink directory")
    return canonical_root


def require_file(root: pathlib.Path, relative: str, label: str) -> pathlib.Path:
    path = root / relative
    try:
        canonical_path = path.resolve(strict=True)
        metadata = path.lstat()
    except OSError as error:
        fail(f"{label} is unavailable ({relative}): {error}")
    try:
        canonical_path.relative_to(root)
    except ValueError:
        fail(f"{label} escaped packet root: {relative}")
    if canonical_path != path:
        fail(f"{label} contains a symlink component: {relative}")
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail(f"{label} must be a non-symlink regular file: {relative}")
    if getattr(metadata, "st_nlink", 1) != 1:
        fail(f"{label} must have exactly one hard link: {relative}")
    if metadata.st_mode & (stat.S_IWGRP | stat.S_IWOTH):
        fail(f"{label} must not be group/world writable: {relative}")
    return canonical_path


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
                if size > MAX_WORKER_BYTES:
                    fail(f"worker file exceeds byte bound: {path.name}")
    except OSError as error:
        fail(f"cannot hash worker file {path.name}: {error}")
    return digest.hexdigest(), size


def reject_forbidden_descriptor_keys(value: Any, location: str = "$") -> None:
    if isinstance(value, dict):
        for key, item in value.items():
            if key in FORBIDDEN_DESCRIPTOR_KEYS:
                fail(f"descriptor contains forbidden secret-bearing key at {location}.{key}")
            reject_forbidden_descriptor_keys(item, f"{location}.{key}")
    elif isinstance(value, list):
        for index, item in enumerate(value):
            reject_forbidden_descriptor_keys(item, f"{location}[{index}]")


def verify_graph_receipt_id(receipt: dict[str, Any]) -> None:
    identifier = receipt.get("receipt_id")
    prefix = "hepta-servo-worker-receipt-graph:v1:"
    if not isinstance(identifier, str) or not identifier.startswith(prefix):
        fail("receipt graph verification ID prefix is invalid")
    digest = require_sha(identifier.removeprefix(prefix), "receipt graph verification ID")
    without_id = dict(receipt)
    without_id.pop("receipt_id")
    if digest != framed(GRAPH_RECEIPT_DOMAIN, canonical(without_id)):
        fail("receipt graph verification ID does not bind its payload")


def inspect_graph_packet(
    root: pathlib.Path,
    manifest_relative: str,
    verification_relative: str,
    worker_relative: str,
) -> dict[str, Any]:
    root = require_root(root)
    manifest_relative = validate_relative_path(manifest_relative, "graph manifest path")
    verification_relative = validate_relative_path(
        verification_relative, "graph verification path"
    )
    worker_relative = validate_relative_path(worker_relative, "worker path")
    if len({manifest_relative, verification_relative, worker_relative}) != 3:
        fail("graph manifest, verification, and worker paths must be unique")

    manifest_path = require_file(root, manifest_relative, "graph manifest")
    verification_path = require_file(root, verification_relative, "graph verification")
    worker_path = require_file(root, worker_relative, "worker")
    manifest, manifest_raw = load_json(manifest_path, "graph manifest")
    verification, verification_raw = load_json(
        verification_path,
        "graph verification",
    )
    if manifest.get("schema") != GRAPH_MANIFEST_SCHEMA:
        fail("graph manifest schema drifted")
    if verification.get("schema") != GRAPH_VERIFICATION_SCHEMA:
        fail("graph verification schema drifted")
    if manifest.get("servo_commit") != SERVO_COMMIT or manifest.get("servo_tree") != SERVO_TREE:
        fail("graph manifest Servo source pin drifted")
    if verification.get("source_pin") != {
        "servo_commit": SERVO_COMMIT,
        "servo_tree": SERVO_TREE,
    }:
        fail("graph verification Servo source pin drifted")
    if manifest.get("policy") != {
        "require_all_edges": True,
        "launch_authorized": False,
        "runtime_qualified": False,
        "allow_unknown_nodes": False,
        "allow_unknown_edges": False,
    }:
        fail("graph manifest policy is not fail-closed")
    if manifest.get("authority") != AUTHORITY:
        fail("graph manifest authority posture is open")
    if verification.get("authority") != AUTHORITY:
        fail("graph verification authority posture is open")
    if verification.get("runtime") != {
        "launch_authorized": False,
        "worker_executed": False,
        "servo_runtime_qualified": False,
        "external_network_used": False,
    }:
        fail("graph verification runtime posture is open")
    if verification.get("decision") != "RECEIPT_GRAPH_BOUND_LAUNCH_NOT_AUTHORIZED":
        fail("graph verification decision overclaims")
    if verification.get("manifest_sha256") != hashlib.sha256(manifest_raw).hexdigest():
        fail("graph verification does not bind the manifest bytes")
    verify_graph_receipt_id(verification)

    nodes = manifest.get("nodes")
    receipt_nodes = verification.get("nodes")
    if not isinstance(nodes, list) or not isinstance(receipt_nodes, list):
        fail("graph manifest/verification node inventories must be arrays")
    manifest_by_id = {
        node.get("id"): node
        for node in nodes
        if isinstance(node, dict) and isinstance(node.get("id"), str)
    }
    verification_by_id = {
        node.get("id"): node
        for node in receipt_nodes
        if isinstance(node, dict) and isinstance(node.get("id"), str)
    }
    required_ids = {"artifact", "build", "reproducibility", "source", "worker"}
    if set(manifest_by_id) != required_ids or set(verification_by_id) != required_ids:
        fail("graph manifest/verification must contain the exact five-node set")
    worker_node_id = manifest.get("worker_node")
    if worker_node_id != "worker":
        fail("graph manifest worker_node drifted")
    worker_node = manifest_by_id["worker"]
    worker_receipt_node = verification_by_id["worker"]
    if worker_node.get("kind") != "binary" or worker_node.get("path") != worker_relative:
        fail("graph manifest selected a different worker path")
    if worker_receipt_node.get("path") != worker_relative or worker_receipt_node.get("kind") != "binary":
        fail("graph verification selected a different worker path")

    worker_sha, worker_bytes = sha256_file(worker_path)
    if worker_bytes <= 0:
        fail("worker file is empty")
    if worker_receipt_node.get("sha256") != worker_sha or worker_receipt_node.get("bytes") != worker_bytes:
        fail("graph verification worker node does not bind actual worker bytes")
    if verification.get("worker") != {
        "node_id": "worker",
        "sha256": worker_sha,
        "bytes": worker_bytes,
    }:
        fail("graph verification worker summary does not bind actual worker bytes")
    graph = verification.get("graph")
    if not isinstance(graph, dict) or graph.get("all_edges_matched") is not True:
        fail("graph verification does not assert all edges matched")
    if graph.get("node_count") != 5 or not isinstance(graph.get("edge_count"), int) or graph["edge_count"] < 7:
        fail("graph verification node/edge count is incomplete")

    return {
        "root": root,
        "manifest_relative": manifest_relative,
        "verification_relative": verification_relative,
        "worker_relative": worker_relative,
        "manifest_sha256": hashlib.sha256(manifest_raw).hexdigest(),
        "verification_sha256": hashlib.sha256(verification_raw).hexdigest(),
        "verification_receipt_id": verification["receipt_id"],
        "worker_sha256": worker_sha,
        "worker_bytes": worker_bytes,
    }


def build_descriptor(
    packet: dict[str, Any],
    browser_session_id: str,
    generation: int,
    owner_epoch: int,
    transport: str,
    captured_at: str | None,
) -> dict[str, Any]:
    if not isinstance(browser_session_id, str) or not SESSION64.fullmatch(browser_session_id):
        fail("browser session ID must be 32-byte lowercase hexadecimal")
    if not isinstance(generation, int) or generation <= 0:
        fail("generation must be a positive integer")
    if not isinstance(owner_epoch, int) or owner_epoch <= 0:
        fail("owner epoch must be a positive integer")
    if transport not in TRANSPORTS:
        fail("transport must be a private inherited socketpair or SID named pipe")

    descriptor: dict[str, Any] = {
        "schema": DESCRIPTOR_SCHEMA,
        "schema_version": 1,
        "phase": "DEVELOPMENT",
        "claim_level": "IMMUTABLE_GRAPH_BOUND_STARTUP_CANDIDATE_ONLY",
        "captured_at_utc": timestamp(captured_at),
        "source_pin": {
            "servo_commit": SERVO_COMMIT,
            "servo_tree": SERVO_TREE,
        },
        "session_binding": {
            "browser_session_id": browser_session_id,
            "generation": generation,
            "owner_epoch": owner_epoch,
        },
        "transport": {
            "kind": transport,
            "network_listener": False,
            "filesystem_endpoint": False,
            "external_network": False,
        },
        "worker": {
            "path": packet["worker_relative"],
            "sha256": packet["worker_sha256"],
            "bytes": packet["worker_bytes"],
        },
        "receipt_graph": {
            "manifest_path": packet["manifest_relative"],
            "manifest_sha256": packet["manifest_sha256"],
            "verification_path": packet["verification_relative"],
            "verification_sha256": packet["verification_sha256"],
            "verification_receipt_id": packet["verification_receipt_id"],
            "verification_required_again_at_launch": True,
        },
        "runtime": {
            "launch_authorized": False,
            "worker_executed": False,
            "servo_runtime_qualified": False,
            "external_network_used": False,
        },
        "authority": AUTHORITY,
        "decision": "GRAPH_BOUND_STARTUP_CANDIDATE_LAUNCH_NOT_AUTHORIZED",
    }
    reject_forbidden_descriptor_keys(descriptor)
    descriptor["descriptor_id"] = "hepta-servo-worker-startup:v1:" + framed(
        DESCRIPTOR_DOMAIN,
        canonical(descriptor),
    )
    return descriptor


def create_descriptor(
    root: pathlib.Path,
    manifest: str,
    verification: str,
    worker: str,
    browser_session_id: str,
    generation: int,
    owner_epoch: int,
    transport: str,
    captured_at: str | None,
) -> dict[str, Any]:
    packet = inspect_graph_packet(root, manifest, verification, worker)
    return build_descriptor(
        packet,
        browser_session_id,
        generation,
        owner_epoch,
        transport,
        captured_at,
    )


def validate_descriptor(
    descriptor_path: pathlib.Path,
    root: pathlib.Path,
) -> dict[str, Any]:
    descriptor, raw = load_json(descriptor_path, "startup descriptor")
    if descriptor.get("schema") != DESCRIPTOR_SCHEMA or descriptor.get("schema_version") != 1:
        fail("startup descriptor schema/version drifted")
    if descriptor.get("phase") != "DEVELOPMENT" or descriptor.get("claim_level") != (
        "IMMUTABLE_GRAPH_BOUND_STARTUP_CANDIDATE_ONLY"
    ):
        fail("startup descriptor phase/claim drifted")
    if descriptor.get("source_pin") != {
        "servo_commit": SERVO_COMMIT,
        "servo_tree": SERVO_TREE,
    }:
        fail("startup descriptor source pin drifted")
    if descriptor.get("authority") != AUTHORITY:
        fail("startup descriptor authority posture is open")
    if descriptor.get("runtime") != {
        "launch_authorized": False,
        "worker_executed": False,
        "servo_runtime_qualified": False,
        "external_network_used": False,
    }:
        fail("startup descriptor runtime posture is open")
    if descriptor.get("decision") != "GRAPH_BOUND_STARTUP_CANDIDATE_LAUNCH_NOT_AUTHORIZED":
        fail("startup descriptor decision overclaims")
    reject_forbidden_descriptor_keys(descriptor)

    identifier = descriptor.get("descriptor_id")
    prefix = "hepta-servo-worker-startup:v1:"
    if not isinstance(identifier, str) or not identifier.startswith(prefix):
        fail("startup descriptor ID prefix is invalid")
    digest = require_sha(identifier.removeprefix(prefix), "startup descriptor ID")
    without_id = dict(descriptor)
    without_id.pop("descriptor_id")
    if digest != framed(DESCRIPTOR_DOMAIN, canonical(without_id)):
        fail("startup descriptor ID does not bind its payload")

    session = descriptor.get("session_binding")
    transport = descriptor.get("transport")
    graph = descriptor.get("receipt_graph")
    worker = descriptor.get("worker")
    if not isinstance(session, dict) or not isinstance(transport, dict) or not isinstance(graph, dict) or not isinstance(worker, dict):
        fail("startup descriptor binding sections are incomplete")
    recomputed = create_descriptor(
        root,
        graph.get("manifest_path"),
        graph.get("verification_path"),
        worker.get("path"),
        session.get("browser_session_id"),
        session.get("generation"),
        session.get("owner_epoch"),
        transport.get("kind"),
        descriptor.get("captured_at_utc"),
    )
    if canonical(recomputed) != raw:
        fail("startup descriptor differs from exact recomputation")
    return descriptor


def write_new(path: pathlib.Path, data: bytes) -> None:
    if not path.is_absolute():
        fail("output path must be absolute")
    try:
        parent = path.parent.resolve(strict=True)
    except OSError as error:
        fail(f"output parent is unavailable: {error}")
    if parent / path.name != path:
        fail("output path must be canonical")
    if path.exists():
        fail("output path already exists; startup descriptors are create-only")
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
            fail(f"startup descriptor compiler contains forbidden execution/network surface: {forbidden}")
    if any(AUTHORITY.values()):
        fail("startup descriptor compiler authority posture is open")
    return {
        "schema": "hepta.servo.worker_startup_descriptor_contract.v1",
        "status": "PASS_FIXTURE_CONTRACT_ONLY",
        "launch_authorized": False,
        "worker_executed": False,
        "real_descriptor_created": False,
        "secret_material_in_descriptor": False,
        "authority": AUTHORITY,
    }


def fixture(root: pathlib.Path) -> dict[str, str]:
    worker_relative = "bin/worker"
    (root / "bin").mkdir()
    worker_path = root / worker_relative
    worker_path.write_bytes(b"fixture-worker\x00")
    worker_sha = hashlib.sha256(worker_path.read_bytes()).hexdigest()
    worker_bytes = worker_path.stat().st_size
    manifest = {
        "schema": GRAPH_MANIFEST_SCHEMA,
        "schema_version": 1,
        "phase": "DEVELOPMENT",
        "servo_commit": SERVO_COMMIT,
        "servo_tree": SERVO_TREE,
        "nodes": [
            {"id": "artifact", "kind": "canonical_json", "path": "artifact.json", "expected_schema": "hepta.servo.worker_artifact_receipt.v1", "negative_posture_scan": True},
            {"id": "build", "kind": "canonical_json", "path": "build.json", "expected_schema": "hepta.servo.worker_build_manifest.v1", "negative_posture_scan": True},
            {"id": "reproducibility", "kind": "canonical_json", "path": "reproducibility.json", "expected_schema": "hepta.servo.worker_reproducibility_receipt.v1", "negative_posture_scan": True},
            {"id": "source", "kind": "canonical_json", "path": "source.json", "expected_schema": "hepta.servo.source_receipt.v1", "negative_posture_scan": True},
            {"id": "worker", "kind": "binary", "path": worker_relative, "expected_schema": None, "negative_posture_scan": False},
        ],
        "edges": [
            {"id": "artifact-build", "type": "pointer_equals_file_sha256", "left_node": "artifact", "left_pointer": "/build_manifest_sha256", "right_node": "build"},
            {"id": "artifact-source", "type": "pointer_equals_file_sha256", "left_node": "artifact", "left_pointer": "/source_receipt_sha256", "right_node": "source"},
            {"id": "artifact-worker", "type": "pointer_equals_file_sha256", "left_node": "artifact", "left_pointer": "/artifact/sha256", "right_node": "worker"},
            {"id": "repro-build", "type": "pointer_equals_file_sha256", "left_node": "reproducibility", "left_pointer": "/source_binding/build_manifest_sha256", "right_node": "build"},
            {"id": "repro-worker", "type": "pointer_equals_file_sha256", "left_node": "reproducibility", "left_pointer": "/outputs/0/sha256", "right_node": "worker"},
            {"id": "source-commit", "type": "pointer_equals_literal", "left_node": "source", "left_pointer": "/source/commit", "literal": SERVO_COMMIT},
            {"id": "source-tree", "type": "pointer_equals_literal", "left_node": "source", "left_pointer": "/source/tree", "literal": SERVO_TREE},
        ],
        "worker_node": "worker",
        "policy": {"require_all_edges": True, "launch_authorized": False, "runtime_qualified": False, "allow_unknown_nodes": False, "allow_unknown_edges": False},
        "authority": AUTHORITY,
    }
    manifest_path = root / "graph-manifest.json"
    manifest_path.write_bytes(canonical(manifest))
    manifest_sha = hashlib.sha256(manifest_path.read_bytes()).hexdigest()
    verification: dict[str, Any] = {
        "schema": GRAPH_VERIFICATION_SCHEMA,
        "schema_version": 1,
        "phase": "DEVELOPMENT",
        "claim_level": "IMMUTABLE_RECEIPT_GRAPH_BINDING_ONLY",
        "captured_at_utc": "2026-08-28T00:00:00Z",
        "source_pin": {"servo_commit": SERVO_COMMIT, "servo_tree": SERVO_TREE},
        "manifest_sha256": manifest_sha,
        "nodes": [
            {"id": "artifact", "kind": "canonical_json", "path": "artifact.json", "sha256": "1" * 64, "bytes": 1, "schema": "hepta.servo.worker_artifact_receipt.v1"},
            {"id": "build", "kind": "canonical_json", "path": "build.json", "sha256": "2" * 64, "bytes": 1, "schema": "hepta.servo.worker_build_manifest.v1"},
            {"id": "reproducibility", "kind": "canonical_json", "path": "reproducibility.json", "sha256": "3" * 64, "bytes": 1, "schema": "hepta.servo.worker_reproducibility_receipt.v1"},
            {"id": "source", "kind": "canonical_json", "path": "source.json", "sha256": "4" * 64, "bytes": 1, "schema": "hepta.servo.source_receipt.v1"},
            {"id": "worker", "kind": "binary", "path": worker_relative, "sha256": worker_sha, "bytes": worker_bytes, "schema": None},
        ],
        "edges": [
            {"id": f"edge-{index}", "type": "pointer_equals_file_sha256", "left_value_sha256": "5" * 64, "right_value_sha256": "5" * 64, "proof_sha256": str(index) * 64, "matched": True}
            for index in range(1, 8)
        ],
        "worker": {"node_id": "worker", "sha256": worker_sha, "bytes": worker_bytes},
        "graph": {"node_count": 5, "edge_count": 7, "total_bytes": worker_bytes + 4, "all_edges_matched": True},
        "runtime": {"launch_authorized": False, "worker_executed": False, "servo_runtime_qualified": False, "external_network_used": False},
        "authority": AUTHORITY,
        "decision": "RECEIPT_GRAPH_BOUND_LAUNCH_NOT_AUTHORIZED",
    }
    verification["receipt_id"] = "hepta-servo-worker-receipt-graph:v1:" + framed(
        GRAPH_RECEIPT_DOMAIN,
        canonical(verification),
    )
    verification_path = root / "graph-verification.json"
    verification_path.write_bytes(canonical(verification))
    return {
        "manifest": "graph-manifest.json",
        "verification": "graph-verification.json",
        "worker": worker_relative,
    }


def self_test() -> dict[str, Any]:
    import tempfile

    tests: list[str] = []
    with tempfile.TemporaryDirectory() as directory:
        root = pathlib.Path(directory)
        paths = fixture(root)
        descriptor = create_descriptor(
            root,
            paths["manifest"],
            paths["verification"],
            paths["worker"],
            "ab" * 32,
            7,
            3,
            "unix_inherited_socketpair",
            "2026-08-28T00:01:00Z",
        )
        descriptor_path = root / "startup-descriptor.json"
        write_new(descriptor_path, canonical(descriptor))
        validate_descriptor(descriptor_path, root)
        tests.append("descriptor_created_and_recomputed")

        try:
            write_new(descriptor_path, canonical(descriptor))
            fail("create-only descriptor overwrite passed")
        except DescriptorError:
            tests.append("create_only_enforced")

        worker_path = root / paths["worker"]
        worker_path.write_bytes(b"drifted-worker")
        try:
            create_descriptor(
                root,
                paths["manifest"],
                paths["verification"],
                paths["worker"],
                "ab" * 32,
                7,
                3,
                "unix_inherited_socketpair",
                "2026-08-28T00:02:00Z",
            )
            fail("worker drift passed")
        except DescriptorError:
            tests.append("worker_drift_rejected")
        worker_path.write_bytes(b"fixture-worker\x00")

        verification_path = root / paths["verification"]
        verification, _ = load_json(verification_path, "verification fixture")
        verification["runtime"]["launch_authorized"] = True
        without_id = dict(verification)
        without_id.pop("receipt_id")
        verification["receipt_id"] = "hepta-servo-worker-receipt-graph:v1:" + framed(
            GRAPH_RECEIPT_DOMAIN,
            canonical(without_id),
        )
        verification_path.write_bytes(canonical(verification))
        try:
            create_descriptor(
                root,
                paths["manifest"],
                paths["verification"],
                paths["worker"],
                "ab" * 32,
                7,
                3,
                "unix_inherited_socketpair",
                "2026-08-28T00:03:00Z",
            )
            fail("launch-authorized graph passed")
        except DescriptorError:
            tests.append("launch_authorized_graph_rejected")
        verification["runtime"]["launch_authorized"] = False
        without_id = dict(verification)
        without_id.pop("receipt_id")
        verification["receipt_id"] = "hepta-servo-worker-receipt-graph:v1:" + framed(
            GRAPH_RECEIPT_DOMAIN,
            canonical(without_id),
        )
        verification_path.write_bytes(canonical(verification))

        for bad_session in ("", "0" * 63, "G" * 64):
            try:
                create_descriptor(
                    root,
                    paths["manifest"],
                    paths["verification"],
                    paths["worker"],
                    bad_session,
                    7,
                    3,
                    "unix_inherited_socketpair",
                    "2026-08-28T00:04:00Z",
                )
                fail("invalid session binding passed")
            except DescriptorError:
                pass
        tests.append("invalid_session_binding_rejected")

        try:
            create_descriptor(
                root,
                paths["manifest"],
                paths["verification"],
                paths["worker"],
                "ab" * 32,
                7,
                3,
                "tcp_loopback",
                "2026-08-28T00:05:00Z",
            )
            fail("network transport passed")
        except DescriptorError:
            tests.append("network_transport_rejected")

        tampered = dict(descriptor)
        tampered["startup_capability"] = "secret"
        try:
            reject_forbidden_descriptor_keys(tampered)
            fail("secret-bearing descriptor key passed")
        except DescriptorError:
            tests.append("secret_bearing_key_rejected")

    if len(tests) != 7:
        fail("unexpected startup descriptor self-test count")
    return {
        "schema": "hepta.servo.worker_startup_descriptor_self_test.v1",
        "status": "PASS_LOCAL_FIXTURE_ONLY",
        "tests": tests,
        "test_count": len(tests),
        "real_descriptor_created": False,
        "launch_authorized": False,
        "worker_executed": False,
        "authority": AUTHORITY,
    }


def parser() -> argparse.ArgumentParser:
    root = argparse.ArgumentParser(description=__doc__)
    sub = root.add_subparsers(dest="command", required=True)
    create = sub.add_parser("create")
    create.add_argument("--root", type=pathlib.Path, required=True)
    create.add_argument("--manifest", required=True)
    create.add_argument("--verification", required=True)
    create.add_argument("--worker", required=True)
    create.add_argument("--browser-session-id", required=True)
    create.add_argument("--generation", type=int, required=True)
    create.add_argument("--owner-epoch", type=int, required=True)
    create.add_argument("--transport", choices=sorted(TRANSPORTS), required=True)
    create.add_argument("--captured-at")
    create.add_argument("--output", type=pathlib.Path, required=True)
    verify = sub.add_parser("verify")
    verify.add_argument("--root", type=pathlib.Path, required=True)
    verify.add_argument("--descriptor", type=pathlib.Path, required=True)
    sub.add_parser("contract")
    sub.add_parser("self-test")
    return root


def main(argv: list[str] | None = None) -> int:
    arguments = parser().parse_args(argv)
    try:
        if arguments.command == "create":
            descriptor = create_descriptor(
                arguments.root,
                arguments.manifest,
                arguments.verification,
                arguments.worker,
                arguments.browser_session_id,
                arguments.generation,
                arguments.owner_epoch,
                arguments.transport,
                arguments.captured_at,
            )
            write_new(arguments.output, canonical(descriptor))
            result = {
                "status": descriptor["decision"],
                "descriptor_id": descriptor["descriptor_id"],
                "launch_authorized": False,
            }
        elif arguments.command == "verify":
            descriptor = validate_descriptor(arguments.descriptor, arguments.root)
            result = {
                "status": "STARTUP_DESCRIPTOR_VERIFIED_LAUNCH_NOT_AUTHORIZED",
                "descriptor_id": descriptor["descriptor_id"],
                "launch_authorized": False,
            }
        elif arguments.command == "contract":
            result = contract()
        else:
            result = self_test()
    except (DescriptorError, OSError, UnicodeError) as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    sys.exit(main())
