#!/usr/bin/env python3
"""Assemble a strict immutable Servo worker receipt-graph manifest.

The assembler is standard-library only and performs no fetch, build, network,
process launch, or worker execution. It accepts one canonical packet root plus
explicit source/build/artifact/reproducibility/worker relative paths. It checks
critical SHA and source-pin relations before emitting the existing strict graph
manifest. The output never authorizes launch and must still be independently
verified by `hepta-servo-worker-receipt-graph-strict.py`.
"""
from __future__ import annotations

import argparse
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
SOURCE_SCHEMA = "hepta.servo.source_receipt.v1"
BUILD_SCHEMA = "hepta.servo.worker_build_manifest.v1"
ARTIFACT_SCHEMA = "hepta.servo.worker_artifact_receipt.v1"
REPRODUCIBILITY_SCHEMA = "hepta.servo.worker_reproducibility_receipt.v1"
SHA64 = re.compile(r"^[0-9a-f]{64}$")
MAX_FILE_BYTES = 4 * 1024 * 1024 * 1024

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


class AssemblyError(RuntimeError):
    """Fail-closed graph-manifest assembly error."""


def fail(message: str) -> None:
    raise AssemblyError(message)


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
        fail(f"{label} escaped the packet root: {relative}")
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
                if size > MAX_FILE_BYTES:
                    fail(f"packet file exceeds byte bound: {path.name}")
    except OSError as error:
        fail(f"cannot hash packet file {path.name}: {error}")
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


def require_schema(value: dict[str, Any], schema: str, label: str) -> None:
    if value.get("schema") != schema:
        fail(f"{label} schema drifted: expected {schema!r}")
    reject_positive_posture(value)


def require_sha(value: Any, label: str) -> str:
    if not isinstance(value, str) or not SHA64.fullmatch(value):
        fail(f"{label} must be lowercase SHA-256")
    return value


def pointer(value: Any, *tokens: str, label: str) -> Any:
    current = value
    traversed: list[str] = []
    for token in tokens:
        traversed.append(token)
        if isinstance(current, dict) and token in current:
            current = current[token]
        else:
            fail(f"{label} is missing /{'/'.join(traversed)}")
    return current


def inspect_packet(
    root: pathlib.Path,
    source_relative: str,
    build_relative: str,
    artifact_relative: str,
    reproducibility_relative: str,
    worker_relative: str,
) -> dict[str, Any]:
    root = require_root(root)
    relatives = {
        "source": validate_relative_path(source_relative, "source path"),
        "build": validate_relative_path(build_relative, "build path"),
        "artifact": validate_relative_path(artifact_relative, "artifact path"),
        "reproducibility": validate_relative_path(
            reproducibility_relative, "reproducibility path"
        ),
        "worker": validate_relative_path(worker_relative, "worker path"),
    }
    if len(set(relatives.values())) != len(relatives):
        fail("source/build/artifact/reproducibility/worker paths must be unique")

    paths = {
        key: require_file(root, relative, f"{key} node")
        for key, relative in relatives.items()
    }
    source, source_raw = load_json(paths["source"], "source receipt")
    build, build_raw = load_json(paths["build"], "worker build manifest")
    artifact, _ = load_json(paths["artifact"], "worker artifact receipt")
    reproducibility, _ = load_json(
        paths["reproducibility"], "worker reproducibility receipt"
    )
    require_schema(source, SOURCE_SCHEMA, "source receipt")
    require_schema(build, BUILD_SCHEMA, "worker build manifest")
    require_schema(artifact, ARTIFACT_SCHEMA, "worker artifact receipt")
    require_schema(
        reproducibility,
        REPRODUCIBILITY_SCHEMA,
        "worker reproducibility receipt",
    )

    if pointer(source, "source", "commit", label="source receipt") != SERVO_COMMIT:
        fail("source receipt Servo commit drifted")
    if pointer(source, "source", "tree", label="source receipt") != SERVO_TREE:
        fail("source receipt Servo tree drifted")

    source_sha = hashlib.sha256(source_raw).hexdigest()
    build_sha = hashlib.sha256(build_raw).hexdigest()
    worker_sha, worker_bytes = sha256_file(paths["worker"])
    if worker_bytes <= 0:
        fail("worker binary is empty")

    if require_sha(
        pointer(artifact, "source_receipt_sha256", label="artifact receipt"),
        "artifact source receipt SHA-256",
    ) != source_sha:
        fail("artifact receipt does not bind the source receipt bytes")
    if require_sha(
        pointer(artifact, "build_manifest_sha256", label="artifact receipt"),
        "artifact build manifest SHA-256",
    ) != build_sha:
        fail("artifact receipt does not bind the build manifest bytes")
    if require_sha(
        pointer(artifact, "artifact", "sha256", label="artifact receipt"),
        "artifact worker SHA-256",
    ) != worker_sha:
        fail("artifact receipt does not bind the worker bytes")

    source_binding = pointer(
        reproducibility,
        "source_binding",
        label="reproducibility receipt",
    )
    if not isinstance(source_binding, dict):
        fail("reproducibility source_binding must be an object")
    if source_binding.get("servo_commit") != SERVO_COMMIT:
        fail("reproducibility receipt Servo commit drifted")
    if source_binding.get("servo_tree") != SERVO_TREE:
        fail("reproducibility receipt Servo tree drifted")
    if require_sha(
        source_binding.get("build_manifest_sha256"),
        "reproducibility build manifest SHA-256",
    ) != build_sha:
        fail("reproducibility receipt does not bind the build manifest bytes")

    outputs = reproducibility.get("outputs")
    if not isinstance(outputs, list):
        fail("reproducibility outputs must be an array")
    worker_matches = [
        item
        for item in outputs
        if isinstance(item, dict)
        and item.get("path") == worker_relative
        and item.get("type") == "binary"
        and item.get("sha256") == worker_sha
        and item.get("bytes") == worker_bytes
    ]
    if len(worker_matches) != 1:
        fail("reproducibility receipt must bind exactly one matching worker output")

    return {
        "root": root,
        "relatives": relatives,
        "source_sha256": source_sha,
        "build_sha256": build_sha,
        "worker_sha256": worker_sha,
        "worker_bytes": worker_bytes,
    }


def assemble_manifest(packet: dict[str, Any]) -> dict[str, Any]:
    paths = packet["relatives"]
    return {
        "schema": MANIFEST_SCHEMA,
        "schema_version": 1,
        "phase": "DEVELOPMENT",
        "servo_commit": SERVO_COMMIT,
        "servo_tree": SERVO_TREE,
        "nodes": [
            {
                "id": "artifact",
                "kind": "canonical_json",
                "path": paths["artifact"],
                "expected_schema": ARTIFACT_SCHEMA,
                "negative_posture_scan": True,
            },
            {
                "id": "build",
                "kind": "canonical_json",
                "path": paths["build"],
                "expected_schema": BUILD_SCHEMA,
                "negative_posture_scan": True,
            },
            {
                "id": "reproducibility",
                "kind": "canonical_json",
                "path": paths["reproducibility"],
                "expected_schema": REPRODUCIBILITY_SCHEMA,
                "negative_posture_scan": True,
            },
            {
                "id": "source",
                "kind": "canonical_json",
                "path": paths["source"],
                "expected_schema": SOURCE_SCHEMA,
                "negative_posture_scan": True,
            },
            {
                "id": "worker",
                "kind": "binary",
                "path": paths["worker"],
                "expected_schema": None,
                "negative_posture_scan": False,
            },
        ],
        "edges": [
            {
                "id": "artifact-build",
                "type": "pointer_equals_file_sha256",
                "left_node": "artifact",
                "left_pointer": "/build_manifest_sha256",
                "right_node": "build",
            },
            {
                "id": "artifact-source",
                "type": "pointer_equals_file_sha256",
                "left_node": "artifact",
                "left_pointer": "/source_receipt_sha256",
                "right_node": "source",
            },
            {
                "id": "artifact-worker",
                "type": "pointer_equals_file_sha256",
                "left_node": "artifact",
                "left_pointer": "/artifact/sha256",
                "right_node": "worker",
            },
            {
                "id": "repro-build",
                "type": "pointer_equals_file_sha256",
                "left_node": "reproducibility",
                "left_pointer": "/source_binding/build_manifest_sha256",
                "right_node": "build",
            },
            {
                "id": "repro-worker",
                "type": "pointer_equals_file_sha256",
                "left_node": "reproducibility",
                "left_pointer": f"/outputs/{find_worker_index(packet)}/sha256",
                "right_node": "worker",
            },
            {
                "id": "source-commit",
                "type": "pointer_equals_literal",
                "left_node": "source",
                "left_pointer": "/source/commit",
                "literal": SERVO_COMMIT,
            },
            {
                "id": "source-tree",
                "type": "pointer_equals_literal",
                "left_node": "source",
                "left_pointer": "/source/tree",
                "literal": SERVO_TREE,
            },
        ],
        "worker_node": "worker",
        "policy": {
            "require_all_edges": True,
            "launch_authorized": False,
            "runtime_qualified": False,
            "allow_unknown_nodes": False,
            "allow_unknown_edges": False,
        },
        "authority": AUTHORITY,
    }


def find_worker_index(packet: dict[str, Any]) -> int:
    reproducibility_path = packet["root"] / packet["relatives"]["reproducibility"]
    reproducibility, _ = load_json(
        reproducibility_path,
        "worker reproducibility receipt",
    )
    outputs = reproducibility["outputs"]
    matches = [
        index
        for index, item in enumerate(outputs)
        if isinstance(item, dict)
        and item.get("path") == packet["relatives"]["worker"]
        and item.get("type") == "binary"
        and item.get("sha256") == packet["worker_sha256"]
        and item.get("bytes") == packet["worker_bytes"]
    ]
    if len(matches) != 1:
        fail("cannot locate one exact worker output index")
    return matches[0]


def create_manifest(
    root: pathlib.Path,
    source: str,
    build: str,
    artifact: str,
    reproducibility: str,
    worker: str,
) -> dict[str, Any]:
    return assemble_manifest(
        inspect_packet(root, source, build, artifact, reproducibility, worker)
    )


def verify_manifest(root: pathlib.Path, manifest_path: pathlib.Path) -> dict[str, Any]:
    manifest, raw = load_json(manifest_path, "assembled graph manifest")
    if manifest.get("schema") != MANIFEST_SCHEMA:
        fail("assembled graph manifest schema drifted")
    nodes = manifest.get("nodes")
    if not isinstance(nodes, list):
        fail("assembled graph manifest nodes must be an array")
    by_id = {
        item.get("id"): item
        for item in nodes
        if isinstance(item, dict) and isinstance(item.get("id"), str)
    }
    if set(by_id) != {"artifact", "build", "reproducibility", "source", "worker"}:
        fail("assembled graph manifest node set drifted")
    recomputed = create_manifest(
        root,
        by_id["source"].get("path"),
        by_id["build"].get("path"),
        by_id["artifact"].get("path"),
        by_id["reproducibility"].get("path"),
        by_id["worker"].get("path"),
    )
    if canonical(recomputed) != raw:
        fail("assembled graph manifest differs from exact recomputation")
    return manifest


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
        fail("output path already exists; graph manifests are create-only")
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
            fail(f"graph manifest assembler contains forbidden surface: {forbidden}")
    if any(AUTHORITY.values()):
        fail("graph manifest assembler authority posture is open")
    return {
        "schema": "hepta.servo.worker_receipt_graph_manifest_assembler_contract.v1",
        "status": "PASS_FIXTURE_CONTRACT_ONLY",
        "servo_commit": SERVO_COMMIT,
        "servo_tree": SERVO_TREE,
        "launch_authorized": False,
        "worker_executed": False,
        "real_packet_assembled": False,
        "authority": AUTHORITY,
    }


def fixture(root: pathlib.Path) -> dict[str, str]:
    worker_relative = "bin/worker"
    (root / "bin").mkdir()
    worker = root / worker_relative
    worker.write_bytes(b"fixture-worker\x00")
    worker_sha = hashlib.sha256(worker.read_bytes()).hexdigest()
    worker_bytes = worker.stat().st_size

    source = {
        "schema": SOURCE_SCHEMA,
        "source": {"commit": SERVO_COMMIT, "tree": SERVO_TREE},
        "authority": AUTHORITY,
    }
    source_path = root / "source.json"
    source_path.write_bytes(canonical(source))
    source_sha = hashlib.sha256(source_path.read_bytes()).hexdigest()

    build = {
        "schema": BUILD_SCHEMA,
        "network_access_during_build": False,
        "worker_production_authority": False,
        "worker_effect_authority": False,
    }
    build_path = root / "build.json"
    build_path.write_bytes(canonical(build))
    build_sha = hashlib.sha256(build_path.read_bytes()).hexdigest()

    artifact = {
        "schema": ARTIFACT_SCHEMA,
        "source_receipt_sha256": source_sha,
        "build_manifest_sha256": build_sha,
        "artifact": {"sha256": worker_sha},
        "runtime": {"runtime_qualified": False},
        "authority": AUTHORITY,
    }
    artifact_path = root / "artifact.json"
    artifact_path.write_bytes(canonical(artifact))

    reproducibility = {
        "schema": REPRODUCIBILITY_SCHEMA,
        "source_binding": {
            "servo_commit": SERVO_COMMIT,
            "servo_tree": SERVO_TREE,
            "build_manifest_sha256": build_sha,
        },
        "outputs": [
            {
                "path": worker_relative,
                "type": "binary",
                "sha256": worker_sha,
                "bytes": worker_bytes,
            }
        ],
        "runtime": {
            "worker_executed": False,
            "servo_runtime_qualified": False,
        },
        "authority": AUTHORITY,
    }
    reproducibility_path = root / "reproducibility.json"
    reproducibility_path.write_bytes(canonical(reproducibility))
    return {
        "source": "source.json",
        "build": "build.json",
        "artifact": "artifact.json",
        "reproducibility": "reproducibility.json",
        "worker": worker_relative,
    }


def self_test() -> dict[str, Any]:
    import tempfile

    tests: list[str] = []
    with tempfile.TemporaryDirectory() as directory:
        root = pathlib.Path(directory)
        paths = fixture(root)
        manifest = create_manifest(root, **paths)
        manifest_path = root / "graph-manifest.json"
        write_new(manifest_path, canonical(manifest))
        verify_manifest(root, manifest_path)
        tests.append("manifest_created_and_recomputed")

        try:
            write_new(manifest_path, canonical(manifest))
            fail("create-only manifest overwrite passed")
        except AssemblyError:
            tests.append("create_only_enforced")

        artifact_path = root / paths["artifact"]
        artifact, _ = load_json(artifact_path, "artifact fixture")
        artifact["artifact"]["sha256"] = "0" * 64
        artifact_path.write_bytes(canonical(artifact))
        try:
            create_manifest(root, **paths)
            fail("artifact-worker drift passed")
        except AssemblyError:
            tests.append("artifact_worker_drift_rejected")
        artifact["artifact"]["sha256"] = hashlib.sha256(
            (root / paths["worker"]).read_bytes()
        ).hexdigest()
        artifact_path.write_bytes(canonical(artifact))

        reproducibility_path = root / paths["reproducibility"]
        reproducibility, _ = load_json(
            reproducibility_path,
            "reproducibility fixture",
        )
        reproducibility["outputs"].append(dict(reproducibility["outputs"][0]))
        reproducibility_path.write_bytes(canonical(reproducibility))
        try:
            create_manifest(root, **paths)
            fail("ambiguous worker output passed")
        except AssemblyError:
            tests.append("ambiguous_worker_output_rejected")
        reproducibility["outputs"] = reproducibility["outputs"][:1]
        reproducibility_path.write_bytes(canonical(reproducibility))

        source_path = root / paths["source"]
        source, _ = load_json(source_path, "source fixture")
        source["authority"] = dict(AUTHORITY)
        source["authority"]["execute_allowed"] = True
        source_path.write_bytes(canonical(source))
        try:
            create_manifest(root, **paths)
            fail("positive authority passed")
        except AssemblyError:
            tests.append("positive_authority_rejected")
        source["authority"] = AUTHORITY
        source_path.write_bytes(canonical(source))

        source_path.write_text('{"schema":"x", "schema":"y"}', encoding="utf-8")
        try:
            create_manifest(root, **paths)
            fail("duplicate JSON keys passed")
        except AssemblyError:
            tests.append("duplicate_json_keys_rejected")

    if len(tests) != 6:
        fail("unexpected graph manifest assembler self-test count")
    return {
        "schema": "hepta.servo.worker_receipt_graph_manifest_assembler_self_test.v1",
        "status": "PASS_LOCAL_FIXTURE_ONLY",
        "tests": tests,
        "test_count": len(tests),
        "real_packet_assembled": False,
        "launch_authorized": False,
        "worker_executed": False,
        "authority": AUTHORITY,
    }


def parser() -> argparse.ArgumentParser:
    root = argparse.ArgumentParser(description=__doc__)
    sub = root.add_subparsers(dest="command", required=True)
    create = sub.add_parser("create")
    create.add_argument("--root", type=pathlib.Path, required=True)
    create.add_argument("--source", required=True)
    create.add_argument("--build", required=True)
    create.add_argument("--artifact", required=True)
    create.add_argument("--reproducibility", required=True)
    create.add_argument("--worker", required=True)
    create.add_argument("--output", type=pathlib.Path, required=True)
    verify = sub.add_parser("verify")
    verify.add_argument("--root", type=pathlib.Path, required=True)
    verify.add_argument("--manifest", type=pathlib.Path, required=True)
    sub.add_parser("contract")
    sub.add_parser("self-test")
    return root


def main(argv: list[str] | None = None) -> int:
    arguments = parser().parse_args(argv)
    try:
        if arguments.command == "create":
            manifest = create_manifest(
                arguments.root,
                arguments.source,
                arguments.build,
                arguments.artifact,
                arguments.reproducibility,
                arguments.worker,
            )
            write_new(arguments.output, canonical(manifest))
            result = {
                "status": "RECEIPT_GRAPH_MANIFEST_CREATED_LAUNCH_NOT_AUTHORIZED",
                "node_count": len(manifest["nodes"]),
                "edge_count": len(manifest["edges"]),
            }
        elif arguments.command == "verify":
            manifest = verify_manifest(arguments.root, arguments.manifest)
            result = {
                "status": "RECEIPT_GRAPH_MANIFEST_VERIFIED_LAUNCH_NOT_AUTHORIZED",
                "node_count": len(manifest["nodes"]),
                "edge_count": len(manifest["edges"]),
            }
        elif arguments.command == "contract":
            result = contract()
        else:
            result = self_test()
    except (AssemblyError, OSError, UnicodeError) as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    sys.exit(main())
