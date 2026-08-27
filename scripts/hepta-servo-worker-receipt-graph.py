#!/usr/bin/env python3
"""Verify a complete Servo worker source/build/artifact/reproducibility receipt graph.

The graph verifier is standard-library only. It performs no source fetch, build,
network access, process launch, or executable inspection beyond hashing regular
files. It emits a create-only receipt only when every declared JSON pointer and
file digest edge matches. The result explicitly keeps launch authorization,
runtime qualification, and all authority false.
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
MANIFEST_SCHEMA = "hepta.servo.worker_receipt_graph_manifest.v1"
RECEIPT_SCHEMA = "hepta.servo.worker_receipt_graph_verification.v1"
RECEIPT_DOMAIN = b"hepta.servo.worker-receipt-graph-verification.v1"
EDGE_DOMAIN = b"hepta.servo.worker-receipt-graph-edge.v1"
SHA64 = re.compile(r"^[0-9a-f]{64}$")
NODE_ID = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._:+@=-]{0,127}$")
UTC_SECONDS = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$")
NODE_KINDS = {"canonical_json", "binary", "text"}
EDGE_KINDS = {"pointer_equals_literal", "pointer_equals_file_sha256", "pointers_equal"}
MAX_NODES = 64
MAX_EDGES = 256
MAX_FILE_BYTES = 4 * 1024 * 1024 * 1024
MAX_TOTAL_BYTES = 16 * 1024 * 1024 * 1024

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

FORBIDDEN_TRUE_KEYS = {
    *AUTHORITY,
    "runtime_qualified",
    "servo_runtime_qualified",
    "worker_executed",
    "launch_authorized",
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


class GraphError(RuntimeError):
    """Fail-closed receipt graph error."""


def fail(message: str) -> None:
    raise GraphError(message)


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


def validate_sha(value: Any, label: str) -> str:
    if not isinstance(value, str) or not SHA64.fullmatch(value):
        fail(f"{label} must be lowercase SHA-256")
    return value


def validate_node_id(value: Any, label: str) -> str:
    if not isinstance(value, str) or not NODE_ID.fullmatch(value):
        fail(f"{label} is not a stable node/edge identifier")
    return value


def validate_relative_path(value: Any) -> str:
    if not isinstance(value, str) or not value or "\x00" in value or "\\" in value:
        fail("graph node path is empty or platform-ambiguous")
    path = pathlib.PurePosixPath(value)
    if path.is_absolute() or any(part in {"", ".", ".."} for part in path.parts):
        fail(f"graph node path is unsafe: {value!r}")
    if len(value.encode("utf-8")) > 1024:
        fail("graph node path is oversized")
    return value


def require_root(value: pathlib.Path) -> pathlib.Path:
    if not value.is_absolute():
        fail("--root must be an absolute path")
    try:
        canonical_root = value.resolve(strict=True)
        metadata = value.lstat()
    except OSError as error:
        fail(f"graph root is unavailable: {error}")
    if canonical_root != value or stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        fail("graph root must be a canonical non-symlink directory")
    return canonical_root


def require_file(root: pathlib.Path, relative: str) -> pathlib.Path:
    path = root / relative
    try:
        canonical_path = path.resolve(strict=True)
        metadata = path.lstat()
    except OSError as error:
        fail(f"graph node file is unavailable ({relative}): {error}")
    try:
        canonical_path.relative_to(root)
    except ValueError:
        fail(f"graph node escaped its root: {relative}")
    if canonical_path != path:
        fail(f"graph node contains a symlink component: {relative}")
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail(f"graph node must be a non-symlink regular file: {relative}")
    if getattr(metadata, "st_nlink", 1) != 1:
        fail(f"graph node must have exactly one hard link: {relative}")
    if metadata.st_mode & (stat.S_IWGRP | stat.S_IWOTH):
        fail(f"graph node must not be group/world writable: {relative}")
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
                if size > MAX_FILE_BYTES:
                    fail(f"graph node exceeds per-file byte bound: {path.name}")
    except OSError as error:
        fail(f"cannot hash graph node {path.name}: {error}")
    return digest.hexdigest(), size


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


def decode_pointer(pointer: Any) -> list[str]:
    if not isinstance(pointer, str) or not pointer.startswith("/"):
        fail("JSON pointer must be a nonempty absolute RFC6901 pointer")
    tokens: list[str] = []
    for token in pointer[1:].split("/"):
        decoded = token.replace("~1", "/").replace("~0", "~")
        if "~" in decoded:
            original = token
            index = 0
            while index < len(original):
                if original[index] == "~":
                    if index + 1 >= len(original) or original[index + 1] not in "01":
                        fail("JSON pointer contains an invalid escape")
                    index += 2
                else:
                    index += 1
        tokens.append(decoded)
    return tokens


def resolve_pointer(value: Any, pointer: str) -> Any:
    current = value
    for token in decode_pointer(pointer):
        if isinstance(current, dict):
            if token not in current:
                fail(f"JSON pointer does not exist: {pointer}")
            current = current[token]
        elif isinstance(current, list):
            if token == "-" or not token.isdigit() or (len(token) > 1 and token.startswith("0")):
                fail(f"JSON pointer array token is invalid: {pointer}")
            index = int(token)
            if index >= len(current):
                fail(f"JSON pointer array index is outside bounds: {pointer}")
            current = current[index]
        else:
            fail(f"JSON pointer traverses a scalar: {pointer}")
    return current


def scalar_bytes(value: Any) -> bytes:
    if isinstance(value, (dict, list)):
        fail("receipt graph edges may compare only scalar JSON pointer values")
    return canonical(value)


def validate_manifest(path: pathlib.Path) -> tuple[dict[str, Any], bytes]:
    manifest, raw = load_json(path, "receipt graph manifest")
    expected_keys = {
        "schema",
        "schema_version",
        "phase",
        "servo_commit",
        "servo_tree",
        "nodes",
        "edges",
        "worker_node",
        "policy",
        "authority",
    }
    if set(manifest) != expected_keys:
        fail("receipt graph manifest field set is incomplete or unknown")
    if manifest.get("schema") != MANIFEST_SCHEMA or manifest.get("schema_version") != 1:
        fail("receipt graph manifest schema/version is unsupported")
    if manifest.get("phase") != "DEVELOPMENT":
        fail("receipt graph manifest phase must remain DEVELOPMENT")
    if manifest.get("servo_commit") != SERVO_COMMIT or manifest.get("servo_tree") != SERVO_TREE:
        fail("receipt graph manifest Servo source pin drifted")
    if manifest.get("authority") != AUTHORITY:
        fail("receipt graph manifest authority posture is open")
    if manifest.get("policy") != {
        "require_all_edges": True,
        "launch_authorized": False,
        "runtime_qualified": False,
        "allow_unknown_nodes": False,
        "allow_unknown_edges": False,
    }:
        fail("receipt graph manifest policy is not fail-closed")

    nodes = manifest.get("nodes")
    if not isinstance(nodes, list) or not (2 <= len(nodes) <= MAX_NODES):
        fail("receipt graph manifest nodes must be a bounded array")
    node_ids: list[str] = []
    for node in nodes:
        if not isinstance(node, dict) or set(node) != {
            "id",
            "kind",
            "path",
            "expected_schema",
            "negative_posture_scan",
        }:
            fail("receipt graph node field set is incomplete or unknown")
        identifier = validate_node_id(node.get("id"), "graph node ID")
        node_ids.append(identifier)
        kind = node.get("kind")
        if kind not in NODE_KINDS:
            fail(f"unsupported graph node kind: {kind!r}")
        validate_relative_path(node.get("path"))
        expected_schema = node.get("expected_schema")
        scan = node.get("negative_posture_scan")
        if not isinstance(scan, bool):
            fail("graph node negative_posture_scan must be a boolean")
        if kind == "canonical_json":
            if not isinstance(expected_schema, str) or not NODE_ID.fullmatch(expected_schema):
                fail("canonical JSON graph node requires an expected schema")
            if scan is not True:
                fail("canonical JSON graph node must enable negative posture scanning")
        elif expected_schema is not None or scan is not False:
            fail("binary/text graph nodes cannot declare JSON schema or posture scanning")
    if node_ids != sorted(set(node_ids)):
        fail("receipt graph nodes must be sorted and unique by ID")

    worker_node = manifest.get("worker_node")
    if worker_node not in node_ids:
        fail("receipt graph worker_node does not name a declared node")
    worker_entry = next(node for node in nodes if node["id"] == worker_node)
    if worker_entry["kind"] != "binary":
        fail("receipt graph worker_node must be binary")

    edges = manifest.get("edges")
    if not isinstance(edges, list) or not (3 <= len(edges) <= MAX_EDGES):
        fail("receipt graph manifest edges must be a bounded array")
    edge_ids: list[str] = []
    commit_literal = False
    tree_literal = False
    for edge in edges:
        if not isinstance(edge, dict):
            fail("receipt graph edge must be an object")
        identifier = validate_node_id(edge.get("id"), "graph edge ID")
        edge_ids.append(identifier)
        kind = edge.get("type")
        if kind not in EDGE_KINDS:
            fail(f"unsupported graph edge kind: {kind!r}")
        left_node = edge.get("left_node")
        if left_node not in node_ids:
            fail("graph edge left_node is unknown")
        decode_pointer(edge.get("left_pointer"))
        if kind == "pointer_equals_literal":
            if set(edge) != {"id", "type", "left_node", "left_pointer", "literal"}:
                fail("literal graph edge field set is invalid")
            literal = edge.get("literal")
            if isinstance(literal, (dict, list)):
                fail("literal graph edge must use a scalar")
            if literal == SERVO_COMMIT:
                commit_literal = True
            if literal == SERVO_TREE:
                tree_literal = True
        elif kind == "pointer_equals_file_sha256":
            if set(edge) != {"id", "type", "left_node", "left_pointer", "right_node"}:
                fail("file SHA graph edge field set is invalid")
            if edge.get("right_node") not in node_ids:
                fail("graph edge right_node is unknown")
        else:
            if set(edge) != {
                "id",
                "type",
                "left_node",
                "left_pointer",
                "right_node",
                "right_pointer",
            }:
                fail("pointer equality graph edge field set is invalid")
            if edge.get("right_node") not in node_ids:
                fail("graph edge right_node is unknown")
            decode_pointer(edge.get("right_pointer"))
    if edge_ids != sorted(set(edge_ids)):
        fail("receipt graph edges must be sorted and unique by ID")
    if not commit_literal or not tree_literal:
        fail("receipt graph must include exact Servo commit and tree literal edges")
    return manifest, raw


def load_nodes(
    root: pathlib.Path,
    node_specs: list[dict[str, Any]],
) -> tuple[dict[str, dict[str, Any]], int]:
    nodes: dict[str, dict[str, Any]] = {}
    total_bytes = 0
    for spec in node_specs:
        path = require_file(root, spec["path"])
        digest, size = sha256_file(path)
        total_bytes += size
        if total_bytes > MAX_TOTAL_BYTES:
            fail("receipt graph exceeds the total byte bound")
        record: dict[str, Any] = {
            "id": spec["id"],
            "kind": spec["kind"],
            "path": spec["path"],
            "sha256": digest,
            "bytes": size,
            "schema": None,
            "value": None,
        }
        if spec["kind"] == "canonical_json":
            value, _ = load_json(path, f"graph node {spec['id']}")
            if value.get("schema") != spec["expected_schema"]:
                fail(f"graph node schema drifted: {spec['id']}")
            reject_positive_posture(value)
            record["schema"] = spec["expected_schema"]
            record["value"] = value
        elif spec["kind"] == "text":
            try:
                text = path.read_text(encoding="utf-8", errors="strict")
            except (OSError, UnicodeError) as error:
                fail(f"text graph node is not strict UTF-8 ({spec['id']}): {error}")
            if "\x00" in text:
                fail(f"text graph node contains NUL: {spec['id']}")
        nodes[spec["id"]] = record
    return nodes, total_bytes


def evaluate_edge(edge: dict[str, Any], nodes: dict[str, dict[str, Any]]) -> dict[str, Any]:
    left_record = nodes[edge["left_node"]]
    left_value = resolve_pointer(left_record["value"], edge["left_pointer"])
    if edge["type"] == "pointer_equals_literal":
        right_value = edge["literal"]
    elif edge["type"] == "pointer_equals_file_sha256":
        right_value = nodes[edge["right_node"]]["sha256"]
    else:
        right_record = nodes[edge["right_node"]]
        right_value = resolve_pointer(right_record["value"], edge["right_pointer"])
    left_bytes = scalar_bytes(left_value)
    right_bytes = scalar_bytes(right_value)
    if left_bytes != right_bytes:
        fail(f"receipt graph edge does not match: {edge['id']}")
    return {
        "id": edge["id"],
        "type": edge["type"],
        "left_value_sha256": hashlib.sha256(left_bytes).hexdigest(),
        "right_value_sha256": hashlib.sha256(right_bytes).hexdigest(),
        "proof_sha256": framed(EDGE_DOMAIN, canonical(edge), left_bytes, right_bytes),
        "matched": True,
    }


def build_receipt(
    root: pathlib.Path,
    manifest_path: pathlib.Path,
    captured_at: str | None,
) -> dict[str, Any]:
    root = require_root(root)
    manifest, manifest_raw = validate_manifest(manifest_path)
    nodes, total_bytes = load_nodes(root, manifest["nodes"])
    edge_results = [evaluate_edge(edge, nodes) for edge in manifest["edges"]]
    node_receipts = [
        {
            "id": record["id"],
            "kind": record["kind"],
            "path": record["path"],
            "sha256": record["sha256"],
            "bytes": record["bytes"],
            "schema": record["schema"],
        }
        for record in nodes.values()
    ]
    worker = nodes[manifest["worker_node"]]
    receipt: dict[str, Any] = {
        "schema": RECEIPT_SCHEMA,
        "schema_version": 1,
        "phase": "DEVELOPMENT",
        "claim_level": "IMMUTABLE_RECEIPT_GRAPH_BINDING_ONLY",
        "captured_at_utc": timestamp(captured_at),
        "source_pin": {"servo_commit": SERVO_COMMIT, "servo_tree": SERVO_TREE},
        "manifest_sha256": hashlib.sha256(manifest_raw).hexdigest(),
        "nodes": node_receipts,
        "edges": edge_results,
        "worker": {
            "node_id": worker["id"],
            "sha256": worker["sha256"],
            "bytes": worker["bytes"],
        },
        "graph": {
            "node_count": len(node_receipts),
            "edge_count": len(edge_results),
            "total_bytes": total_bytes,
            "all_edges_matched": True,
        },
        "runtime": {
            "launch_authorized": False,
            "worker_executed": False,
            "servo_runtime_qualified": False,
            "external_network_used": False,
        },
        "authority": AUTHORITY,
        "decision": "RECEIPT_GRAPH_BOUND_LAUNCH_NOT_AUTHORIZED",
    }
    receipt["receipt_id"] = "hepta-servo-worker-receipt-graph:v1:" + framed(
        RECEIPT_DOMAIN, canonical(receipt)
    )
    return receipt


def validate_receipt(
    receipt_path: pathlib.Path,
    root: pathlib.Path,
    manifest_path: pathlib.Path,
) -> dict[str, Any]:
    receipt, raw = load_json(receipt_path, "receipt graph verification")
    if receipt.get("schema") != RECEIPT_SCHEMA or receipt.get("schema_version") != 1:
        fail("receipt graph verification schema/version is unsupported")
    if receipt.get("phase") != "DEVELOPMENT" or receipt.get("claim_level") != (
        "IMMUTABLE_RECEIPT_GRAPH_BINDING_ONLY"
    ):
        fail("receipt graph verification phase/claim is invalid")
    timestamp(receipt.get("captured_at_utc"))
    if receipt.get("source_pin") != {"servo_commit": SERVO_COMMIT, "servo_tree": SERVO_TREE}:
        fail("receipt graph verification source pin drifted")
    if receipt.get("runtime") != {
        "launch_authorized": False,
        "worker_executed": False,
        "servo_runtime_qualified": False,
        "external_network_used": False,
    }:
        fail("receipt graph verification runtime posture is open")
    if receipt.get("authority") != AUTHORITY:
        fail("receipt graph verification authority posture is open")
    if receipt.get("decision") != "RECEIPT_GRAPH_BOUND_LAUNCH_NOT_AUTHORIZED":
        fail("receipt graph verification decision overclaims")
    identifier = receipt.get("receipt_id")
    prefix = "hepta-servo-worker-receipt-graph:v1:"
    if not isinstance(identifier, str) or not identifier.startswith(prefix):
        fail("receipt graph verification ID prefix is invalid")
    digest = identifier.removeprefix(prefix)
    validate_sha(digest, "receipt graph verification ID")
    without_id = dict(receipt)
    without_id.pop("receipt_id")
    if digest != framed(RECEIPT_DOMAIN, canonical(without_id)):
        fail("receipt graph verification ID does not bind its payload")
    recomputed = build_receipt(root, manifest_path, receipt["captured_at_utc"])
    if canonical(recomputed) != raw:
        fail("receipt graph verification differs from exact recomputation")
    return receipt


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
        fail("output path already exists; graph receipts are create-only")
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
            fail(f"receipt graph tool contains forbidden execution/network surface: {forbidden}")
    if any(AUTHORITY.values()):
        fail("receipt graph authority posture is open")
    return {
        "schema": "hepta.servo.worker_receipt_graph_contract.v1",
        "status": "PASS_FIXTURE_CONTRACT_ONLY",
        "servo_commit": SERVO_COMMIT,
        "servo_tree": SERVO_TREE,
        "launch_authorized": False,
        "real_worker_graph_verified": False,
        "runtime_qualified": False,
        "authority": AUTHORITY,
    }


def self_test() -> dict[str, Any]:
    import tempfile

    tests: list[str] = []
    with tempfile.TemporaryDirectory() as directory:
        root = pathlib.Path(directory)
        worker = root / "worker.bin"
        worker.write_bytes(b"fixture-worker-binary\x00")
        worker_sha = hashlib.sha256(worker.read_bytes()).hexdigest()

        source = {
            "schema": "hepta.servo.source_receipt.v1",
            "source": {"commit": SERVO_COMMIT, "tree": SERVO_TREE},
            "authority": AUTHORITY,
        }
        source_path = root / "source.json"
        source_path.write_bytes(canonical(source))
        source_sha = hashlib.sha256(source_path.read_bytes()).hexdigest()

        build = {
            "schema": "hepta.servo.worker_build_manifest.v1",
            "source_receipt_sha256": source_sha,
            "worker_production_authority": False,
            "worker_effect_authority": False,
            "network_access_during_build": False,
        }
        build_path = root / "build.json"
        build_path.write_bytes(canonical(build))
        build_sha = hashlib.sha256(build_path.read_bytes()).hexdigest()

        artifact = {
            "schema": "hepta.servo.worker_artifact_receipt.v1",
            "source_receipt_sha256": source_sha,
            "build_manifest_sha256": build_sha,
            "artifact": {"sha256": worker_sha},
            "runtime": {"runtime_qualified": False},
            "authority": AUTHORITY,
        }
        artifact_path = root / "artifact.json"
        artifact_path.write_bytes(canonical(artifact))

        reproducibility = {
            "schema": "hepta.servo.worker_reproducibility_receipt.v1",
            "source_binding": {
                "servo_commit": SERVO_COMMIT,
                "servo_tree": SERVO_TREE,
                "build_manifest_sha256": build_sha,
            },
            "outputs": [{"path": "worker.bin", "sha256": worker_sha}],
            "runtime": {"worker_executed": False, "servo_runtime_qualified": False},
            "authority": AUTHORITY,
        }
        reproducibility_path = root / "reproducibility.json"
        reproducibility_path.write_bytes(canonical(reproducibility))

        manifest = {
            "schema": MANIFEST_SCHEMA,
            "schema_version": 1,
            "phase": "DEVELOPMENT",
            "servo_commit": SERVO_COMMIT,
            "servo_tree": SERVO_TREE,
            "nodes": [
                {"id": "artifact", "kind": "canonical_json", "path": "artifact.json", "expected_schema": "hepta.servo.worker_artifact_receipt.v1", "negative_posture_scan": True},
                {"id": "build", "kind": "canonical_json", "path": "build.json", "expected_schema": "hepta.servo.worker_build_manifest.v1", "negative_posture_scan": True},
                {"id": "reproducibility", "kind": "canonical_json", "path": "reproducibility.json", "expected_schema": "hepta.servo.worker_reproducibility_receipt.v1", "negative_posture_scan": True},
                {"id": "source", "kind": "canonical_json", "path": "source.json", "expected_schema": "hepta.servo.source_receipt.v1", "negative_posture_scan": True},
                {"id": "worker", "kind": "binary", "path": "worker.bin", "expected_schema": None, "negative_posture_scan": False},
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
        manifest_path = root / "manifest.json"
        manifest_path.write_bytes(canonical(manifest))
        receipt = build_receipt(root, manifest_path, "2026-08-28T00:00:00Z")
        receipt_path = root / "receipt.json"
        write_new(receipt_path, canonical(receipt))
        validate_receipt(receipt_path, root, manifest_path)
        tests.append("complete_graph_recomputed")

        worker.write_bytes(b"drifted-worker")
        try:
            build_receipt(root, manifest_path, "2026-08-28T00:01:00Z")
            fail("worker drift passed")
        except GraphError:
            tests.append("worker_drift_rejected")
        worker.write_bytes(b"fixture-worker-binary\x00")

        artifact["authority"] = dict(AUTHORITY)
        artifact["authority"]["runtime_authority"] = True
        artifact_path.write_bytes(canonical(artifact))
        try:
            build_receipt(root, manifest_path, "2026-08-28T00:02:00Z")
            fail("positive authority passed")
        except GraphError:
            tests.append("positive_authority_rejected")
        artifact["authority"] = AUTHORITY
        artifact_path.write_bytes(canonical(artifact))

        artifact["artifact"]["sha256"] = "0" * 64
        artifact_path.write_bytes(canonical(artifact))
        try:
            build_receipt(root, manifest_path, "2026-08-28T00:03:00Z")
            fail("edge mismatch passed")
        except GraphError:
            tests.append("edge_mismatch_rejected")
        artifact["artifact"]["sha256"] = worker_sha
        artifact_path.write_bytes(canonical(artifact))

        bad_manifest = dict(manifest)
        bad_manifest["nodes"] = list(reversed(manifest["nodes"]))
        bad_manifest_path = root / "bad-manifest.json"
        bad_manifest_path.write_bytes(canonical(bad_manifest))
        try:
            validate_manifest(bad_manifest_path)
            fail("unsorted graph manifest passed")
        except GraphError:
            tests.append("unsorted_manifest_rejected")

        duplicate_path = root / "duplicate.json"
        duplicate_path.write_text('{"a":1,"a":2}', encoding="utf-8")
        try:
            load_json(duplicate_path, "duplicate fixture")
            fail("duplicate keys passed")
        except GraphError:
            tests.append("duplicate_keys_rejected")

    if len(tests) != 6:
        fail("unexpected receipt graph self-test count")
    return {
        "schema": "hepta.servo.worker_receipt_graph_self_test.v1",
        "status": "PASS_LOCAL_FIXTURE_ONLY",
        "tests": tests,
        "test_count": len(tests),
        "real_worker_graph_verified": False,
        "launch_authorized": False,
        "runtime_qualified": False,
        "authority": AUTHORITY,
    }


def parser() -> argparse.ArgumentParser:
    root = argparse.ArgumentParser(description=__doc__)
    sub = root.add_subparsers(dest="command", required=True)
    create = sub.add_parser("create")
    create.add_argument("--root", type=pathlib.Path, required=True)
    create.add_argument("--manifest", type=pathlib.Path, required=True)
    create.add_argument("--captured-at")
    create.add_argument("--output", type=pathlib.Path, required=True)
    verify = sub.add_parser("verify")
    verify.add_argument("--root", type=pathlib.Path, required=True)
    verify.add_argument("--manifest", type=pathlib.Path, required=True)
    verify.add_argument("--receipt", type=pathlib.Path, required=True)
    sub.add_parser("contract")
    sub.add_parser("self-test")
    return root


def main(argv: list[str] | None = None) -> int:
    arguments = parser().parse_args(argv)
    try:
        if arguments.command == "create":
            receipt = build_receipt(arguments.root, arguments.manifest, arguments.captured_at)
            write_new(arguments.output, canonical(receipt))
            result = {"status": receipt["decision"], "receipt_id": receipt["receipt_id"], "node_count": receipt["graph"]["node_count"], "edge_count": receipt["graph"]["edge_count"]}
        elif arguments.command == "verify":
            receipt = validate_receipt(arguments.receipt, arguments.root, arguments.manifest)
            result = {"status": "RECEIPT_GRAPH_VERIFIED_LAUNCH_NOT_AUTHORIZED", "receipt_id": receipt["receipt_id"], "node_count": receipt["graph"]["node_count"], "edge_count": receipt["graph"]["edge_count"]}
        elif arguments.command == "contract":
            result = contract()
        else:
            result = self_test()
    except (GraphError, OSError, UnicodeError) as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    sys.exit(main())
