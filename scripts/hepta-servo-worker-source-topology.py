#!/usr/bin/env python3
"""Verify the frozen minimal Servo worker source/API topology.

The verifier is standard-library only. In real verification mode it first reruns
the canonical offline source-bundle verifier, then independently inspects the
selected upstream files inside the deterministic archive. It never builds,
links, launches, or executes Servo.
"""
from __future__ import annotations

import argparse
import datetime as dt
import gzip
import hashlib
import importlib.util
import json
import os
import pathlib
import re
import stat
import sys
import tarfile
import tempfile
from types import ModuleType
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
BROWSER_ROOT = ROOT / "docs/hepta-vnext/browser"
DEFAULT_TOPOLOGY = BROWSER_ROOT / "SERVO_WORKER_SOURCE_TOPOLOGY_V1.json"
DEFAULT_PIN = BROWSER_ROOT / "SERVO_UPSTREAM_PIN.json"
SOURCE_VERIFY_V2 = pathlib.Path(__file__).with_name(
    "hepta-servo-source-bundle-verify-v2.py"
)

SERVO_REPOSITORY = "servo/servo"
SERVO_COMMIT = "0a48e298482659817eb50097df23841f2b8e3044"
SERVO_TREE = "b04d2f75b3217374d079d579c270177b57fa1389"
TOPOLOGY_SCHEMA = "hepta.servo.worker_source_topology.v1"
RECEIPT_SCHEMA = "hepta.servo.worker_source_topology_verification.v1"
TOPOLOGY_DOMAIN = b"hepta.servo.worker-source-topology.v1"
RECEIPT_DOMAIN = b"hepta.servo.worker-source-topology-verification.v1"
MAX_TOPOLOGY_BYTES = 2 * 1024 * 1024
MAX_REQUIRED_FILE_BYTES = 8 * 1024 * 1024
MAX_ARCHIVE_BYTES = 8 * 1024 * 1024 * 1024
SHA1 = re.compile(r"^[0-9a-f]{40}$")
SHA256 = re.compile(r"^[0-9a-f]{64}$")
UTC_SECONDS = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$")

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

EXPECTED_SELECTED_BLOBS = {
    "Cargo.toml": "de3665c8cf98337aa908ddacef906b4bea7832e5",
    "components/servo/Cargo.toml": "d9dce354a8db17b95f2ef618d73a19b1438f08fc",
    "components/servo/lib.rs": "f7a33d88ca3fdb68e0984cff95101c4df98df1da",
    "components/servo/examples/winit_minimal.rs": "9a185c102ca812477c855089fde0af92c937d838",
}
EXPECTED_REFERENCE_BLOBS = {
    "ports/servoshell/Cargo.toml": "2576ee3e26f0772c7737c42215ac266346b1c2ea",
    "ports/servoshell/lib.rs": "399e0c4fcc68a5c426ff981286ded466fcf9a03b",
    "ports/servoshell/running_app_state.rs": "4685ebd21fd93c20669f947b62a48daf7ae415a5",
    "ports/servoshell/webdriver.rs": "4665c52134a968ab3ff572ac97dab61463967de4",
    "components/webdriver_server/lib.rs": "8341df151f04a214694dcf0d9ad675e7d1e7fe98",
}
REQUIRED_FEATURES = ["background_hang_monitor", "bundled"]
CONDITIONAL_FEATURES = ["js_jit"]
FORBIDDEN_FEATURES = [
    "bluetooth",
    "clipboard",
    "default",
    "default_web_features",
    "default_without_allocator",
    "gamepad",
    "media-gstreamer",
    "native-bluetooth",
    "testbinding",
    "webgl",
    "webgpu",
    "webxr",
]


class TopologyError(RuntimeError):
    """Fail-closed source/API topology error."""


def fail(message: str) -> None:
    raise TopologyError(message)


def reject_duplicate_keys(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            fail(f"duplicate JSON key {key!r}")
        result[key] = value
    return result


def canonical(value: object) -> bytes:
    return json.dumps(
        value,
        ensure_ascii=False,
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")


def framed(domain: bytes, *fields: bytes) -> str:
    digest = hashlib.sha256()
    digest.update(len(domain).to_bytes(8, "big"))
    digest.update(domain)
    for field in fields:
        digest.update(len(field).to_bytes(8, "big"))
        digest.update(field)
    return digest.hexdigest()


def git_blob_id(content: bytes) -> str:
    header = b"blob " + str(len(content)).encode("ascii") + b"\0"
    return hashlib.sha1(header + content).hexdigest()


def sha256_file(path: pathlib.Path, maximum: int = MAX_ARCHIVE_BYTES) -> tuple[str, int]:
    digest = hashlib.sha256()
    total = 0
    try:
        with path.open("rb") as stream:
            for block in iter(lambda: stream.read(1024 * 1024), b""):
                total += len(block)
                if total > maximum:
                    fail(f"file exceeds byte bound: {path.name}")
                digest.update(block)
    except OSError as error:
        fail(f"cannot hash {path}: {error}")
    return digest.hexdigest(), total


def safe_regular_file(
    path: pathlib.Path,
    label: str,
    *,
    maximum_bytes: int | None = None,
) -> pathlib.Path:
    try:
        metadata = path.lstat()
        resolved = path.resolve(strict=True)
    except OSError as error:
        fail(f"{label} is unavailable: {error}")
    if resolved != path or stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail(f"{label} must be a canonical non-symlink regular file")
    if getattr(metadata, "st_nlink", 1) != 1:
        fail(f"{label} must have exactly one hard link")
    if metadata.st_mode & (stat.S_IWGRP | stat.S_IWOTH):
        fail(f"{label} must not be group/world writable")
    if maximum_bytes is not None and not (1 <= metadata.st_size <= maximum_bytes):
        fail(f"{label} byte length is outside 1..={maximum_bytes}")
    return path


def safe_absolute_directory(path: pathlib.Path, label: str) -> pathlib.Path:
    if not path.is_absolute() or ".." in path.parts:
        fail(f"{label} must be an absolute canonical path")
    try:
        metadata = path.lstat()
        resolved = path.resolve(strict=True)
    except OSError as error:
        fail(f"{label} is unavailable: {error}")
    if resolved != path or stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        fail(f"{label} must be a canonical non-symlink directory")
    return path


def load_json(
    path: pathlib.Path,
    label: str,
    *,
    maximum_bytes: int,
) -> tuple[dict[str, Any], bytes]:
    safe_regular_file(path, label, maximum_bytes=maximum_bytes)
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


def require_sha1(value: Any, label: str) -> str:
    if not isinstance(value, str) or not SHA1.fullmatch(value):
        fail(f"{label} must be lowercase Git SHA-1")
    return value


def require_sha256(value: Any, label: str) -> str:
    if not isinstance(value, str) or not SHA256.fullmatch(value):
        fail(f"{label} must be lowercase SHA-256")
    return value


def require_relative_path(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value or "\0" in value or "\\" in value:
        fail(f"{label} is empty or platform-ambiguous")
    path = pathlib.PurePosixPath(value)
    if path.is_absolute() or any(part in {"", ".", ".."} for part in path.parts):
        fail(f"{label} is not a normalized repository-relative path")
    if len(value.encode("utf-8")) > 1024:
        fail(f"{label} is oversized")
    return value


def require_string_list(
    value: Any,
    label: str,
    *,
    nonempty: bool = True,
    sorted_unique: bool = True,
    maximum_items: int = 256,
    maximum_bytes: int = 4096,
) -> list[str]:
    if not isinstance(value, list) or len(value) > maximum_items:
        fail(f"{label} must be a bounded array")
    if nonempty and not value:
        fail(f"{label} must not be empty")
    if any(
        not isinstance(item, str)
        or not item
        or len(item.encode("utf-8")) > maximum_bytes
        or "\0" in item
        for item in value
    ):
        fail(f"{label} contains an empty, oversized, or NUL-bearing string")
    if sorted_unique and value != sorted(set(value)):
        fail(f"{label} must be lexically sorted and unique")
    return value


def validate_file_entries(
    entries: Any,
    label: str,
    expected: dict[str, str],
) -> list[dict[str, Any]]:
    if not isinstance(entries, list) or len(entries) != len(expected):
        fail(f"{label} must contain exactly {len(expected)} entries")
    paths: list[str] = []
    validated: list[dict[str, Any]] = []
    for entry in entries:
        if not isinstance(entry, dict) or set(entry) != {
            "path",
            "git_blob_sha1",
            "max_bytes",
            "role",
            "required_anchors",
        }:
            fail(f"{label} entry fields are incomplete or unknown")
        path = require_relative_path(entry.get("path"), f"{label}.path")
        paths.append(path)
        expected_blob = expected.get(path)
        if expected_blob is None:
            fail(f"{label} contains unexpected path {path}")
        if require_sha1(entry.get("git_blob_sha1"), f"{path} blob") != expected_blob:
            fail(f"{path} blob differs from the frozen upstream object")
        maximum = entry.get("max_bytes")
        if (
            not isinstance(maximum, int)
            or isinstance(maximum, bool)
            or not (1 <= maximum <= MAX_REQUIRED_FILE_BYTES)
        ):
            fail(f"{path} max_bytes is outside the topology bound")
        role = entry.get("role")
        if not isinstance(role, str) or not role or len(role.encode("utf-8")) > 1024:
            fail(f"{path} role is invalid")
        require_string_list(
            entry.get("required_anchors"),
            f"{path} required anchors",
            maximum_items=64,
            maximum_bytes=2048,
        )
        validated.append(entry)
    if paths != sorted(expected):
        fail(f"{label} paths must equal the frozen sorted path set")
    return validated


def verify_topology_id(topology: dict[str, Any]) -> None:
    identifier = topology.get("topology_id")
    prefix = "hepta-servo-worker-source-topology:v1:"
    if not isinstance(identifier, str) or not identifier.startswith(prefix):
        fail("topology ID prefix is invalid")
    digest = require_sha256(identifier.removeprefix(prefix), "topology ID")
    payload = dict(topology)
    payload.pop("topology_id")
    if digest != framed(TOPOLOGY_DOMAIN, canonical(payload)):
        fail("topology ID does not bind the canonical payload")


def validate_topology(topology: dict[str, Any]) -> dict[str, Any]:
    expected_keys = {
        "schema",
        "schema_version",
        "phase",
        "claim_level",
        "source",
        "decision",
        "selected_files",
        "reference_only_files",
        "api_contract",
        "runtime_contract",
        "open_gates",
        "authority",
        "topology_id",
    }
    if set(topology) != expected_keys:
        fail("topology field set is incomplete or unknown")
    if topology.get("schema") != TOPOLOGY_SCHEMA or topology.get("schema_version") != 1:
        fail("topology schema/version drifted")
    if topology.get("phase") != "DEVELOPMENT":
        fail("topology phase must remain DEVELOPMENT")
    if topology.get("claim_level") != "MINIMAL_HEPTA_OWNED_EMBEDDER_TOPOLOGY_ONLY":
        fail("topology claim level overclaims or drifted")

    source = topology.get("source")
    if source != {
        "repository": SERVO_REPOSITORY,
        "commit": SERVO_COMMIT,
        "tree": SERVO_TREE,
        "license": "MPL-2.0",
    }:
        fail("topology source pin drifted")

    decision = topology.get("decision")
    expected_decision_keys = {
        "embedder_strategy",
        "worker_owner",
        "servo_crate_path",
        "servoshell_build_root",
        "servoshell_dependency",
        "webdriver_server_dependency",
        "servo_default_features",
        "required_servo_features",
        "conditionally_permitted_servo_features",
        "forbidden_servo_features",
        "patch_required_before_servoshell_build",
        "reason_codes",
    }
    if not isinstance(decision, dict) or set(decision) != expected_decision_keys:
        fail("topology decision fields are incomplete or unknown")
    if decision.get("embedder_strategy") != "out_of_tree_hepta_worker_using_public_servo_embedding_api":
        fail("topology did not select the Hepta-owned embedder strategy")
    if decision.get("worker_owner") != "hepta" or decision.get("servo_crate_path") != "components/servo":
        fail("topology worker ownership or Servo crate path drifted")
    for key in (
        "servoshell_build_root",
        "servoshell_dependency",
        "webdriver_server_dependency",
        "servo_default_features",
    ):
        if decision.get(key) is not False:
            fail(f"topology attempted to enable {key}")
    if decision.get("patch_required_before_servoshell_build") is not True:
        fail("topology must block direct servoshell builds until a governed patch exists")
    if decision.get("required_servo_features") != REQUIRED_FEATURES:
        fail("required Servo feature set drifted")
    if decision.get("conditionally_permitted_servo_features") != CONDITIONAL_FEATURES:
        fail("conditional Servo feature set drifted")
    if decision.get("forbidden_servo_features") != FORBIDDEN_FEATURES:
        fail("forbidden Servo feature set drifted")
    required = set(REQUIRED_FEATURES)
    conditional = set(CONDITIONAL_FEATURES)
    forbidden = set(FORBIDDEN_FEATURES)
    if required & conditional or required & forbidden or conditional & forbidden:
        fail("Servo feature sets overlap")
    require_string_list(
        decision.get("reason_codes"),
        "topology reason codes",
        maximum_items=32,
        maximum_bytes=256,
    )

    selected = validate_file_entries(
        topology.get("selected_files"),
        "selected_files",
        EXPECTED_SELECTED_BLOBS,
    )
    reference = validate_file_entries(
        topology.get("reference_only_files"),
        "reference_only_files",
        EXPECTED_REFERENCE_BLOBS,
    )
    if set(EXPECTED_SELECTED_BLOBS) & set(EXPECTED_REFERENCE_BLOBS):
        fail("selected and reference-only file sets overlap")

    api_contract = topology.get("api_contract")
    if not isinstance(api_contract, dict) or set(api_contract) != {
        "required_public_types",
        "required_public_methods",
        "forbidden_public_surfaces",
        "one_webview_collection_type",
    }:
        fail("API contract fields are incomplete or unknown")
    require_string_list(
        api_contract.get("required_public_types"),
        "required public types",
        maximum_items=64,
        maximum_bytes=256,
    )
    require_string_list(
        api_contract.get("required_public_methods"),
        "required public methods",
        maximum_items=64,
        maximum_bytes=256,
    )
    require_string_list(
        api_contract.get("forbidden_public_surfaces"),
        "forbidden public surfaces",
        maximum_items=64,
        maximum_bytes=256,
    )
    if api_contract.get("one_webview_collection_type") != "Option<WebView>":
        fail("topology must use Option<WebView>, not a multi-WebView collection")

    runtime = topology.get("runtime_contract")
    expected_runtime = {
        "one_process": True,
        "one_webview": True,
        "one_mutation_owner": True,
        "local_fixture_only": True,
        "external_network": False,
        "network_listener": False,
        "raw_webdriver": False,
        "arbitrary_javascript": False,
        "cookie_storage_profile_export": False,
        "download_upload": False,
        "second_window_or_webview": False,
    }
    if runtime != expected_runtime:
        fail("runtime contract drifted or widened")
    require_string_list(
        topology.get("open_gates"),
        "open gates",
        maximum_items=64,
        maximum_bytes=1024,
    )
    if topology.get("authority") != AUTHORITY:
        fail("topology authority posture is not exactly fail-closed")
    verify_topology_id(topology)
    return {
        "selected": selected,
        "reference": reference,
        "decision": decision,
        "api_contract": api_contract,
    }


def load_topology(path: pathlib.Path) -> tuple[dict[str, Any], bytes]:
    topology, raw = load_json(
        path,
        "Servo worker source topology",
        maximum_bytes=MAX_TOPOLOGY_BYTES,
    )
    validate_topology(topology)
    return topology, raw


def load_source_verifier() -> ModuleType:
    try:
        specification = importlib.util.spec_from_file_location(
            "hepta_servo_source_bundle_verify_v2_for_topology",
            SOURCE_VERIFY_V2,
        )
        if specification is None or specification.loader is None:
            fail("cannot load canonical source-bundle verifier v2")
        module = importlib.util.module_from_spec(specification)
        specification.loader.exec_module(module)
        base = module.load_base()
    except (OSError, AttributeError, RuntimeError) as error:
        fail(f"cannot initialize canonical source-bundle verifier v2: {error}")
    return base


def safe_member_relative(prefix: str, name: str) -> str | None:
    if "\\" in name or "\0" in name:
        fail(f"archive path escaped frozen source prefix: {name!r}")
    if name == prefix.rstrip("/"):
        return None
    if not name.startswith(prefix):
        fail(f"archive path escaped frozen source prefix: {name!r}")
    relative = name[len(prefix):]
    if not relative:
        return None
    path = pathlib.PurePosixPath(relative)
    if path.is_absolute() or any(part in {"", ".", ".."} for part in path.parts):
        fail(f"archive path is unsafe: {name!r}")
    return path.as_posix()


def scan_tar(
    tar_path: pathlib.Path,
    prefix: str,
    topology: dict[str, Any],
) -> dict[str, Any]:
    expected_entries = {
        entry["path"]: entry
        for category in ("selected_files", "reference_only_files")
        for entry in topology[category]
    }
    found: dict[str, dict[str, Any]] = {}
    seen_members: set[str] = set()
    try:
        with tarfile.open(tar_path, mode="r:") as archive:
            for member in archive:
                if member.name in seen_members:
                    fail(f"archive contains duplicate path: {member.name!r}")
                seen_members.add(member.name)
                relative = safe_member_relative(prefix, member.name)
                if member.islnk():
                    fail(f"archive contains a hard link: {member.name!r}")
                if relative is None or member.isdir():
                    continue
                if relative not in expected_entries:
                    if not (member.isfile() or member.issym()):
                        fail(f"archive contains unsupported entry type: {member.name!r}")
                    continue
                if not member.isfile():
                    fail(f"topology file must be a regular file: {relative}")
                entry = expected_entries[relative]
                if member.size < 1 or member.size > entry["max_bytes"]:
                    fail(f"topology file byte length is outside its bound: {relative}")
                stream = archive.extractfile(member)
                if stream is None:
                    fail(f"topology file cannot be read: {relative}")
                content = stream.read(entry["max_bytes"] + 1)
                if len(content) != member.size or len(content) > entry["max_bytes"]:
                    fail(f"topology file length changed or exceeded its bound: {relative}")
                blob = git_blob_id(content)
                if blob != entry["git_blob_sha1"]:
                    fail(
                        f"topology file Git blob drifted for {relative}: "
                        f"expected {entry['git_blob_sha1']}, found {blob}"
                    )
                for anchor in entry["required_anchors"]:
                    if anchor.encode("utf-8") not in content:
                        fail(f"topology anchor is missing from {relative}: {anchor!r}")
                found[relative] = {
                    "path": relative,
                    "git_blob_sha1": blob,
                    "sha256": hashlib.sha256(content).hexdigest(),
                    "bytes": len(content),
                    "role": entry["role"],
                    "anchor_count": len(entry["required_anchors"]),
                }
    except (OSError, tarfile.TarError, UnicodeError) as error:
        fail(f"cannot inspect deterministic source tar: {error}")
    missing = sorted(set(expected_entries) - set(found))
    if missing:
        fail(f"deterministic source archive lacks topology files: {missing}")
    ordered = [found[path] for path in sorted(found)]
    return {
        "selected_file_count": len(topology["selected_files"]),
        "reference_only_file_count": len(topology["reference_only_files"]),
        "files": ordered,
        "file_projection_sha256": hashlib.sha256(canonical(ordered)).hexdigest(),
    }


def scan_compressed_archive(
    archive_path: pathlib.Path,
    topology: dict[str, Any],
) -> dict[str, Any]:
    safe_regular_file(archive_path, "compressed source archive")
    compressed_sha256, compressed_bytes = sha256_file(archive_path)
    prefix = f"servo-{SERVO_COMMIT}/"
    with tempfile.TemporaryDirectory(prefix="hepta-servo-topology-") as temporary:
        tar_path = pathlib.Path(temporary) / "servo-source.tar"
        try:
            with archive_path.open("rb") as compressed:
                with gzip.GzipFile(filename="", mode="rb", fileobj=compressed) as stream:
                    total = 0
                    with tar_path.open("xb") as output:
                        os.chmod(tar_path, 0o600)
                        while True:
                            block = stream.read(1024 * 1024)
                            if not block:
                                break
                            total += len(block)
                            if total > MAX_ARCHIVE_BYTES:
                                fail("decompressed source archive exceeds the byte bound")
                            output.write(block)
                        output.flush()
                        os.fsync(output.fileno())
        except (OSError, EOFError) as error:
            fail(f"cannot decompress source archive: {error}")
        projection = scan_tar(tar_path, prefix, topology)
    return {
        "compressed_archive_sha256": compressed_sha256,
        "compressed_archive_bytes": compressed_bytes,
        **projection,
    }


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


def write_new(path: pathlib.Path, raw: bytes) -> None:
    if not path.is_absolute():
        fail("--output must be an absolute path")
    try:
        parent = path.parent.resolve(strict=True)
    except OSError as error:
        fail(f"output parent is unavailable: {error}")
    if path != parent / path.name or path.exists() or path.is_symlink():
        fail("output path must be canonical and create-only")
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    try:
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(raw)
            stream.flush()
            os.fsync(stream.fileno())
        if hasattr(os, "O_DIRECTORY"):
            directory = os.open(parent, os.O_RDONLY | os.O_DIRECTORY)
            try:
                os.fsync(directory)
            finally:
                os.close(directory)
    except Exception:
        path.unlink(missing_ok=True)
        raise


def verify_bundle_topology(
    bundle_dir: pathlib.Path,
    pin_path: pathlib.Path,
    topology_path: pathlib.Path,
    captured_at: str | None,
) -> dict[str, Any]:
    bundle_dir = safe_absolute_directory(bundle_dir, "bundle directory")
    topology, topology_raw = load_topology(topology_path)
    source_verifier = load_source_verifier()
    try:
        source_result = source_verifier.verify_bundle(bundle_dir, pin_path, None)
    except Exception as error:
        fail(f"canonical source-bundle verification failed: {error}")
    source = source_result.get("source")
    if not isinstance(source, dict) or (
        source.get("repository"),
        source.get("commit"),
        source.get("tree"),
        source.get("recomputed_tree"),
    ) != (SERVO_REPOSITORY, SERVO_COMMIT, SERVO_TREE, SERVO_TREE):
        fail("canonical source verification projection drifted")
    verification = source_result.get("verification")
    if not isinstance(verification, dict):
        fail("canonical source verification facts are missing")
    for key in (
        "canonical_json",
        "no_machine_local_paths",
        "two_distinct_acquisition_nonces",
        "gzip_single_member_mtime_zero",
        "archive_paths_safe",
        "git_tree_recomputed",
        "pinned_tree_matched",
        "license_matched",
    ):
        if verification.get(key) is not True:
            fail(f"canonical source verification fact is not true: {key}")
    for key in ("servo_built", "servo_runtime_qualified", "release_qualified"):
        if verification.get(key) is not False:
            fail(f"source verification attempted to enable {key}")
    source_authority = source_result.get("authority")
    if not isinstance(source_authority, dict) or any(
        item is not False for item in source_authority.values()
    ):
        fail("source verification authority posture is open")

    archive_path = bundle_dir / "servo-source-a.tar.gz"
    archive_projection = scan_compressed_archive(archive_path, topology)
    if archive_projection["compressed_archive_sha256"] != source_result.get(
        "compressed_archive_sha256"
    ):
        fail("topology scan archive digest differs from source verification receipt")

    receipt: dict[str, Any] = {
        "schema": RECEIPT_SCHEMA,
        "schema_version": 1,
        "phase": "DEVELOPMENT",
        "claim_level": "SOURCE_API_TOPOLOGY_VERIFIED_BUILD_NOT_AUTHORIZED",
        "captured_at_utc": timestamp(captured_at),
        "source": {
            "repository": SERVO_REPOSITORY,
            "commit": SERVO_COMMIT,
            "tree": SERVO_TREE,
            "recomputed_tree": SERVO_TREE,
        },
        "bindings": {
            "topology_id": topology["topology_id"],
            "topology_sha256": hashlib.sha256(topology_raw).hexdigest(),
            "source_bundle_receipt_sha256": source_result["bundle_receipt_sha256"],
            "compressed_archive_sha256": archive_projection[
                "compressed_archive_sha256"
            ],
        },
        "selection": {
            "embedder_strategy": topology["decision"]["embedder_strategy"],
            "worker_owner": "hepta",
            "servo_crate_path": "components/servo",
            "servoshell_build_root": False,
            "servoshell_dependency": False,
            "webdriver_server_dependency": False,
            "servo_default_features": False,
            "required_servo_features": REQUIRED_FEATURES,
            "conditionally_permitted_servo_features": CONDITIONAL_FEATURES,
            "forbidden_servo_features": FORBIDDEN_FEATURES,
            "selected_file_count": archive_projection["selected_file_count"],
            "reference_only_file_count": archive_projection[
                "reference_only_file_count"
            ],
            "file_projection_sha256": archive_projection[
                "file_projection_sha256"
            ],
        },
        "verification": {
            "canonical_source_bundle_reverified": True,
            "selected_git_blobs_match": True,
            "required_public_api_anchors_match": True,
            "servoshell_conflict_anchors_match": True,
            "webdriver_wildcard_listener_anchor_match": True,
            "hepta_owned_embedder_required": True,
            "build_recipe_created": False,
            "servo_built": False,
            "worker_artifact_created": False,
            "servo_runtime_qualified": False,
        },
        "authority": AUTHORITY,
        "decision": "HEPTA_OWNED_EMBEDDER_TOPOLOGY_VERIFIED_BUILD_NOT_AUTHORIZED",
    }
    receipt["receipt_id"] = (
        "hepta-servo-worker-source-topology-verification:v1:"
        + framed(RECEIPT_DOMAIN, canonical(receipt))
    )
    return receipt


def parse_arguments(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    contract = commands.add_parser("contract")
    contract.add_argument("--topology", default=os.fspath(DEFAULT_TOPOLOGY))
    verify = commands.add_parser("verify")
    verify.add_argument("--bundle-dir", required=True)
    verify.add_argument("--pin", default=os.fspath(DEFAULT_PIN))
    verify.add_argument("--topology", default=os.fspath(DEFAULT_TOPOLOGY))
    verify.add_argument("--captured-at")
    verify.add_argument("--output", required=True)
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    try:
        arguments = parse_arguments(argv)
        topology_path = pathlib.Path(arguments.topology).resolve(strict=True)
        topology, raw = load_topology(topology_path)
        if arguments.command == "contract":
            result = {
                "schema": TOPOLOGY_SCHEMA,
                "status": "PASS_TOPOLOGY_CONTRACT_ONLY",
                "topology_id": topology["topology_id"],
                "topology_sha256": hashlib.sha256(raw).hexdigest(),
                "embedder_strategy": topology["decision"]["embedder_strategy"],
                "servoshell_build_root": False,
                "webdriver_server_dependency": False,
                "servo_built": False,
                "runtime_authority": False,
            }
        else:
            receipt = verify_bundle_topology(
                pathlib.Path(arguments.bundle_dir),
                pathlib.Path(arguments.pin).resolve(strict=True),
                topology_path,
                arguments.captured_at,
            )
            output = pathlib.Path(arguments.output)
            encoded = canonical(receipt)
            write_new(output, encoded)
            result = {
                "schema": RECEIPT_SCHEMA,
                "status": "PASS_SOURCE_API_TOPOLOGY_ONLY",
                "receipt_id": receipt["receipt_id"],
                "receipt_sha256": hashlib.sha256(encoded).hexdigest(),
                "servo_built": False,
                "worker_artifact_created": False,
                "servo_runtime_qualified": False,
                "runtime_authority": False,
            }
    except (
        TopologyError,
        OSError,
        UnicodeError,
        json.JSONDecodeError,
        tarfile.TarError,
        gzip.BadGzipFile,
    ) as error:
        print(f"HEPTA_SERVO_WORKER_SOURCE_TOPOLOGY=FAIL: {error}", file=sys.stderr)
        return 1
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
