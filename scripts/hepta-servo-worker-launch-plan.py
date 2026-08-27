#!/usr/bin/env python3
"""Compile a graph-bound platform qualification launch plan for a Servo worker.

The compiler is standard-library only. It performs no source fetch, build,
network access, process launch, handle inheritance, sandbox installation, or
worker execution. It binds one graph-bound startup descriptor to one symbolic
platform enforcement policy and emits a create-only plan whose launch and
runtime authority remain false until separate enforcement receipts exist.
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
DESCRIPTOR_SCHEMA = "hepta.servo.worker_startup_descriptor.v1"
POLICY_SCHEMA = "hepta.servo.worker_launch_policy.v1"
PLAN_SCHEMA = "hepta.servo.worker_qualification_launch_plan.v1"
DESCRIPTOR_DOMAIN = b"hepta.servo.worker-startup-descriptor.v1"
PLAN_DOMAIN = b"hepta.servo.worker-qualification-launch-plan.v1"
SHA64 = re.compile(r"^[0-9a-f]{64}$")
UTC_SECONDS = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$")
MAX_POLICY_BYTES = 1024 * 1024

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

PLATFORMS = {
    "linux_x86_64": {
        "target_triple": "x86_64-unknown-linux-gnu",
        "transport": "unix_inherited_socketpair",
        "executable_binding": "open_readonly_fd_fstat_proc_fd_exec",
        "peer_identity": "spawned_pid_plus_peer_credentials",
        "parent_death": "pdeathsig_process_group",
        "descendant_cleanup": "process_group_kill_and_reap",
    },
    "macos_arm64": {
        "target_triple": "aarch64-apple-darwin",
        "transport": "unix_inherited_socketpair",
        "executable_binding": "open_readonly_fd_fstat_spawn_file_actions",
        "peer_identity": "spawned_pid_plus_peer_credentials",
        "parent_death": "parent_monitor_process_group",
        "descendant_cleanup": "process_group_kill_and_reap",
    },
    "windows_x86_64": {
        "target_triple": "x86_64-pc-windows-msvc",
        "transport": "windows_sid_named_pipe",
        "executable_binding": "readonly_file_handle_identity_then_create_process",
        "peer_identity": "spawned_pid_token_sid_and_named_pipe_client_pid",
        "parent_death": "job_object_kill_on_close",
        "descendant_cleanup": "job_object_process_tree_reap",
    },
}

LIMIT_BOUNDS = {
    "startup_timeout_ms": (100, 120_000),
    "command_timeout_ms": (100, 600_000),
    "teardown_timeout_ms": (100, 120_000),
    "memory_limit_bytes": (64 * 1024 * 1024, 64 * 1024 * 1024 * 1024),
    "cpu_time_limit_ms": (1_000, 24 * 60 * 60 * 1_000),
    "process_limit": (1, 128),
    "open_file_limit": (16, 65_536),
    "output_limit_bytes": (1_024, 1024 * 1024 * 1024),
}

FORBIDDEN_KEYS = {
    "startup_capability",
    "startup_capability_sha256",
    "host_nonce",
    "raw_capability",
    "credential",
    "secret",
    "environment",
    "command_line",
    "absolute_path",
}


class LaunchPlanError(RuntimeError):
    """Fail-closed launch-plan compilation error."""


def fail(message: str) -> None:
    raise LaunchPlanError(message)


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
    maximum_bytes: int | None = None,
) -> tuple[dict[str, Any], bytes]:
    try:
        raw = path.read_bytes()
    except OSError as error:
        fail(f"cannot read {label}: {error}")
    if maximum_bytes is not None and len(raw) > maximum_bytes:
        fail(f"{label} exceeds its byte bound")
    try:
        value = json.loads(raw, object_pairs_hook=reject_duplicate_keys)
    except (UnicodeError, json.JSONDecodeError) as error:
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


def reject_forbidden_keys(value: Any, location: str = "$") -> None:
    if isinstance(value, dict):
        for key, item in value.items():
            if key in FORBIDDEN_KEYS:
                fail(f"launch plan contains forbidden key at {location}.{key}")
            reject_forbidden_keys(item, f"{location}.{key}")
    elif isinstance(value, list):
        for index, item in enumerate(value):
            reject_forbidden_keys(item, f"{location}[{index}]")


def verify_descriptor_id(descriptor: dict[str, Any]) -> None:
    identifier = descriptor.get("descriptor_id")
    prefix = "hepta-servo-worker-startup:v1:"
    if not isinstance(identifier, str) or not identifier.startswith(prefix):
        fail("startup descriptor ID prefix is invalid")
    digest = require_sha(identifier.removeprefix(prefix), "startup descriptor ID")
    without_id = dict(descriptor)
    without_id.pop("descriptor_id")
    if digest != framed(DESCRIPTOR_DOMAIN, canonical(without_id)):
        fail("startup descriptor ID does not bind its payload")


def validate_descriptor(descriptor: dict[str, Any]) -> None:
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
    transport = descriptor.get("transport")
    if not isinstance(transport, dict) or transport.get("kind") not in {
        "unix_inherited_socketpair",
        "windows_sid_named_pipe",
    }:
        fail("startup descriptor transport is not private")
    if transport.get("network_listener") is not False or transport.get("filesystem_endpoint") is not False or transport.get("external_network") is not False:
        fail("startup descriptor transport posture is open")
    graph = descriptor.get("receipt_graph")
    if not isinstance(graph, dict) or graph.get("verification_required_again_at_launch") is not True:
        fail("startup descriptor does not require graph reverification at launch")
    reject_forbidden_keys(descriptor)
    verify_descriptor_id(descriptor)


def validate_policy(policy: dict[str, Any]) -> dict[str, Any]:
    expected_keys = {
        "schema",
        "schema_version",
        "phase",
        "platform",
        "target_triple",
        "transport",
        "enforcement",
        "policy_digests",
        "limits",
        "admission",
        "authority",
    }
    if set(policy) != expected_keys:
        fail("launch policy field set is incomplete or unknown")
    if policy.get("schema") != POLICY_SCHEMA or policy.get("schema_version") != 1:
        fail("launch policy schema/version drifted")
    if policy.get("phase") != "DEVELOPMENT":
        fail("launch policy phase must remain DEVELOPMENT")
    platform = policy.get("platform")
    expected = PLATFORMS.get(platform)
    if expected is None:
        fail("launch policy platform is unsupported")
    if policy.get("target_triple") != expected["target_triple"]:
        fail("launch policy target triple does not match platform")
    if policy.get("transport") != expected["transport"]:
        fail("launch policy transport does not match platform")
    enforcement = policy.get("enforcement")
    if not isinstance(enforcement, dict) or set(enforcement) != {
        "executable_binding",
        "peer_identity",
        "parent_death",
        "descendant_cleanup",
        "sandbox_required",
        "egress_denial_required",
        "network_listener",
        "filesystem_endpoint",
        "external_network",
    }:
        fail("launch policy enforcement field set drifted")
    for key in (
        "executable_binding",
        "peer_identity",
        "parent_death",
        "descendant_cleanup",
    ):
        if enforcement.get(key) != expected[key]:
            fail(f"launch policy {key} does not match platform")
    if enforcement.get("sandbox_required") is not True or enforcement.get("egress_denial_required") is not True:
        fail("launch policy must require sandbox and egress denial")
    if enforcement.get("network_listener") is not False or enforcement.get("filesystem_endpoint") is not False or enforcement.get("external_network") is not False:
        fail("launch policy enforcement enables network/filesystem endpoint")

    digests = policy.get("policy_digests")
    if not isinstance(digests, dict) or set(digests) != {
        "sandbox_policy_sha256",
        "egress_policy_sha256",
        "profile_root_policy_sha256",
        "resource_policy_sha256",
    }:
        fail("launch policy digest set drifted")
    for key, value in digests.items():
        require_sha(value, f"launch policy {key}")
    if len(set(digests.values())) != len(digests):
        fail("launch policy digests must bind distinct policy objects")

    limits = policy.get("limits")
    if not isinstance(limits, dict) or set(limits) != set(LIMIT_BOUNDS):
        fail("launch policy limit set drifted")
    for key, bounds in LIMIT_BOUNDS.items():
        value = limits.get(key)
        if not isinstance(value, int) or isinstance(value, bool) or not bounds[0] <= value <= bounds[1]:
            fail(f"launch policy limit {key} is outside bounds")
    if limits["command_timeout_ms"] < limits["startup_timeout_ms"]:
        fail("command timeout must not be shorter than startup timeout")
    if limits["teardown_timeout_ms"] > limits["command_timeout_ms"]:
        fail("teardown timeout must not exceed command timeout")

    admission = policy.get("admission")
    if admission != {
        "launch_authorized": False,
        "evidence_complete": False,
        "operator_acceptance_required": False,
        "promotion_required": False,
    }:
        fail("launch policy admission posture is open")
    if policy.get("authority") != AUTHORITY:
        fail("launch policy authority posture is open")
    reject_forbidden_keys(policy)
    return expected


def build_plan(
    descriptor: dict[str, Any],
    descriptor_relative: str,
    descriptor_sha256: str,
    policy: dict[str, Any],
    policy_relative: str,
    policy_sha256: str,
    captured_at: str | None,
) -> dict[str, Any]:
    validate_descriptor(descriptor)
    expected = validate_policy(policy)
    if descriptor["transport"]["kind"] != policy["transport"]:
        fail("startup descriptor transport does not match launch policy")

    plan: dict[str, Any] = {
        "schema": PLAN_SCHEMA,
        "schema_version": 1,
        "phase": "DEVELOPMENT",
        "claim_level": "GRAPH_BOUND_PLATFORM_QUALIFICATION_PLAN_ONLY",
        "captured_at_utc": timestamp(captured_at),
        "source_pin": {
            "servo_commit": SERVO_COMMIT,
            "servo_tree": SERVO_TREE,
        },
        "startup_descriptor": {
            "path": descriptor_relative,
            "sha256": descriptor_sha256,
            "descriptor_id": descriptor["descriptor_id"],
        },
        "session_binding": descriptor["session_binding"],
        "worker": descriptor["worker"],
        "receipt_graph": descriptor["receipt_graph"],
        "platform_policy": {
            "path": policy_relative,
            "sha256": policy_sha256,
            "platform": policy["platform"],
            "target_triple": policy["target_triple"],
            "transport": policy["transport"],
            "enforcement": policy["enforcement"],
            "policy_digests": policy["policy_digests"],
            "limits": policy["limits"],
        },
        "required_evidence": {
            "graph_reverification_at_launch": True,
            "executable_handle_binding_receipt": None,
            "peer_identity_receipt": None,
            "sandbox_enforcement_receipt": None,
            "egress_denial_receipt": None,
            "profile_root_receipt": None,
            "resource_limit_receipt": None,
            "listener_scan_receipt": None,
            "parent_death_receipt": None,
            "descendant_cleanup_receipt": None,
            "deadline_kill_reap_receipt": None,
        },
        "runtime": {
            "launch_authorized": False,
            "worker_executed": False,
            "servo_runtime_qualified": False,
            "external_network_used": False,
        },
        "authority": AUTHORITY,
        "admission_state": "BLOCKED_EVIDENCE_REQUIRED",
        "decision": "GRAPH_BOUND_PLATFORM_LAUNCH_PLAN_NOT_ADMITTED",
    }
    if expected["transport"] != plan["platform_policy"]["transport"]:
        fail("internal platform launch plan transport mismatch")
    reject_forbidden_keys(plan)
    plan["plan_id"] = "hepta-servo-worker-launch-plan:v1:" + framed(
        PLAN_DOMAIN,
        canonical(plan),
    )
    return plan


def create_plan(
    root: pathlib.Path,
    descriptor_relative: str,
    policy_relative: str,
    captured_at: str | None,
) -> dict[str, Any]:
    root = require_root(root)
    descriptor_relative = validate_relative_path(descriptor_relative, "descriptor path")
    policy_relative = validate_relative_path(policy_relative, "launch policy path")
    if descriptor_relative == policy_relative:
        fail("descriptor and launch policy paths must differ")
    descriptor_path = require_file(root, descriptor_relative, "startup descriptor")
    policy_path = require_file(root, policy_relative, "launch policy")
    descriptor, descriptor_raw = load_json(descriptor_path, "startup descriptor")
    policy, policy_raw = load_json(
        policy_path,
        "launch policy",
        maximum_bytes=MAX_POLICY_BYTES,
    )
    return build_plan(
        descriptor,
        descriptor_relative,
        hashlib.sha256(descriptor_raw).hexdigest(),
        policy,
        policy_relative,
        hashlib.sha256(policy_raw).hexdigest(),
        captured_at,
    )


def validate_plan(plan_path: pathlib.Path, root: pathlib.Path) -> dict[str, Any]:
    plan, raw = load_json(plan_path, "qualification launch plan")
    if plan.get("schema") != PLAN_SCHEMA or plan.get("schema_version") != 1:
        fail("qualification launch plan schema/version drifted")
    if plan.get("phase") != "DEVELOPMENT" or plan.get("claim_level") != (
        "GRAPH_BOUND_PLATFORM_QUALIFICATION_PLAN_ONLY"
    ):
        fail("qualification launch plan phase/claim drifted")
    if plan.get("source_pin") != {
        "servo_commit": SERVO_COMMIT,
        "servo_tree": SERVO_TREE,
    }:
        fail("qualification launch plan source pin drifted")
    if plan.get("runtime") != {
        "launch_authorized": False,
        "worker_executed": False,
        "servo_runtime_qualified": False,
        "external_network_used": False,
    }:
        fail("qualification launch plan runtime posture is open")
    if plan.get("authority") != AUTHORITY:
        fail("qualification launch plan authority posture is open")
    if plan.get("admission_state") != "BLOCKED_EVIDENCE_REQUIRED" or plan.get("decision") != (
        "GRAPH_BOUND_PLATFORM_LAUNCH_PLAN_NOT_ADMITTED"
    ):
        fail("qualification launch plan overclaims admission")
    evidence = plan.get("required_evidence")
    if not isinstance(evidence, dict) or evidence.get("graph_reverification_at_launch") is not True:
        fail("qualification launch plan does not require graph reverification")
    for key, value in evidence.items():
        if key != "graph_reverification_at_launch" and value is not None:
            fail(f"qualification launch plan unexpectedly contains enforcement evidence: {key}")
    reject_forbidden_keys(plan)

    identifier = plan.get("plan_id")
    prefix = "hepta-servo-worker-launch-plan:v1:"
    if not isinstance(identifier, str) or not identifier.startswith(prefix):
        fail("qualification launch plan ID prefix is invalid")
    digest = require_sha(identifier.removeprefix(prefix), "qualification launch plan ID")
    without_id = dict(plan)
    without_id.pop("plan_id")
    if digest != framed(PLAN_DOMAIN, canonical(without_id)):
        fail("qualification launch plan ID does not bind its payload")

    descriptor = plan.get("startup_descriptor")
    policy = plan.get("platform_policy")
    if not isinstance(descriptor, dict) or not isinstance(policy, dict):
        fail("qualification launch plan input bindings are incomplete")
    recomputed = create_plan(
        root,
        descriptor.get("path"),
        policy.get("path"),
        plan.get("captured_at_utc"),
    )
    if canonical(recomputed) != raw:
        fail("qualification launch plan differs from exact recomputation")
    return plan


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
        fail("output path already exists; qualification launch plans are create-only")
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
            fail(f"qualification launch plan compiler contains forbidden surface: {forbidden}")
    if any(AUTHORITY.values()):
        fail("qualification launch plan compiler authority posture is open")
    return {
        "schema": "hepta.servo.worker_qualification_launch_plan_contract.v1",
        "status": "PASS_FIXTURE_CONTRACT_ONLY",
        "supported_platforms": sorted(PLATFORMS),
        "launch_authorized": False,
        "worker_executed": False,
        "real_plan_created": False,
        "authority": AUTHORITY,
    }


def fixture(root: pathlib.Path) -> dict[str, str]:
    descriptor = {
        "schema": DESCRIPTOR_SCHEMA,
        "schema_version": 1,
        "phase": "DEVELOPMENT",
        "claim_level": "IMMUTABLE_GRAPH_BOUND_STARTUP_CANDIDATE_ONLY",
        "captured_at_utc": "2026-08-28T00:00:00Z",
        "source_pin": {"servo_commit": SERVO_COMMIT, "servo_tree": SERVO_TREE},
        "session_binding": {"browser_session_id": "ab" * 32, "generation": 7, "owner_epoch": 3},
        "transport": {"kind": "unix_inherited_socketpair", "network_listener": False, "filesystem_endpoint": False, "external_network": False},
        "worker": {"path": "bin/worker", "sha256": "1" * 64, "bytes": 42},
        "receipt_graph": {"manifest_path": "graph-manifest.json", "manifest_sha256": "2" * 64, "verification_path": "graph-verification.json", "verification_sha256": "3" * 64, "verification_receipt_id": "hepta-servo-worker-receipt-graph:v1:" + "4" * 64, "verification_required_again_at_launch": True},
        "runtime": {"launch_authorized": False, "worker_executed": False, "servo_runtime_qualified": False, "external_network_used": False},
        "authority": AUTHORITY,
        "decision": "GRAPH_BOUND_STARTUP_CANDIDATE_LAUNCH_NOT_AUTHORIZED",
    }
    descriptor["descriptor_id"] = "hepta-servo-worker-startup:v1:" + framed(
        DESCRIPTOR_DOMAIN,
        canonical(descriptor),
    )
    descriptor_path = root / "startup-descriptor.json"
    descriptor_path.write_bytes(canonical(descriptor))

    policy = {
        "schema": POLICY_SCHEMA,
        "schema_version": 1,
        "phase": "DEVELOPMENT",
        "platform": "linux_x86_64",
        "target_triple": PLATFORMS["linux_x86_64"]["target_triple"],
        "transport": PLATFORMS["linux_x86_64"]["transport"],
        "enforcement": {
            "executable_binding": PLATFORMS["linux_x86_64"]["executable_binding"],
            "peer_identity": PLATFORMS["linux_x86_64"]["peer_identity"],
            "parent_death": PLATFORMS["linux_x86_64"]["parent_death"],
            "descendant_cleanup": PLATFORMS["linux_x86_64"]["descendant_cleanup"],
            "sandbox_required": True,
            "egress_denial_required": True,
            "network_listener": False,
            "filesystem_endpoint": False,
            "external_network": False,
        },
        "policy_digests": {
            "sandbox_policy_sha256": "5" * 64,
            "egress_policy_sha256": "6" * 64,
            "profile_root_policy_sha256": "7" * 64,
            "resource_policy_sha256": "8" * 64,
        },
        "limits": {
            "startup_timeout_ms": 5_000,
            "command_timeout_ms": 30_000,
            "teardown_timeout_ms": 5_000,
            "memory_limit_bytes": 512 * 1024 * 1024,
            "cpu_time_limit_ms": 60_000,
            "process_limit": 16,
            "open_file_limit": 256,
            "output_limit_bytes": 16 * 1024 * 1024,
        },
        "admission": {"launch_authorized": False, "evidence_complete": False, "operator_acceptance_required": False, "promotion_required": False},
        "authority": AUTHORITY,
    }
    policy_path = root / "launch-policy.json"
    policy_path.write_bytes(canonical(policy))
    return {"descriptor": "startup-descriptor.json", "policy": "launch-policy.json"}


def self_test() -> dict[str, Any]:
    import tempfile

    tests: list[str] = []
    with tempfile.TemporaryDirectory() as directory:
        root = pathlib.Path(directory)
        paths = fixture(root)
        plan = create_plan(root, paths["descriptor"], paths["policy"], "2026-08-28T00:01:00Z")
        plan_path = root / "launch-plan.json"
        write_new(plan_path, canonical(plan))
        validate_plan(plan_path, root)
        tests.append("plan_created_and_recomputed")

        try:
            write_new(plan_path, canonical(plan))
            fail("create-only plan overwrite passed")
        except LaunchPlanError:
            tests.append("create_only_enforced")

        policy_path = root / paths["policy"]
        policy, _ = load_json(policy_path, "policy fixture")
        policy["transport"] = "windows_sid_named_pipe"
        policy_path.write_bytes(canonical(policy))
        try:
            create_plan(root, paths["descriptor"], paths["policy"], "2026-08-28T00:02:00Z")
            fail("platform transport mismatch passed")
        except LaunchPlanError:
            tests.append("platform_transport_mismatch_rejected")
        policy["transport"] = PLATFORMS["linux_x86_64"]["transport"]
        policy_path.write_bytes(canonical(policy))

        policy["limits"]["startup_timeout_ms"] = 0
        policy_path.write_bytes(canonical(policy))
        try:
            create_plan(root, paths["descriptor"], paths["policy"], "2026-08-28T00:03:00Z")
            fail("zero startup timeout passed")
        except LaunchPlanError:
            tests.append("invalid_limit_rejected")
        policy["limits"]["startup_timeout_ms"] = 5_000
        policy_path.write_bytes(canonical(policy))

        policy["admission"]["launch_authorized"] = True
        policy_path.write_bytes(canonical(policy))
        try:
            create_plan(root, paths["descriptor"], paths["policy"], "2026-08-28T00:04:00Z")
            fail("launch-authorized policy passed")
        except LaunchPlanError:
            tests.append("launch_authorized_policy_rejected")
        policy["admission"]["launch_authorized"] = False
        policy_path.write_bytes(canonical(policy))

        policy["host_nonce"] = "secret"
        policy_path.write_bytes(canonical(policy))
        try:
            create_plan(root, paths["descriptor"], paths["policy"], "2026-08-28T00:05:00Z")
            fail("secret-bearing policy passed")
        except LaunchPlanError:
            tests.append("secret_bearing_policy_rejected")
        policy.pop("host_nonce")
        policy_path.write_bytes(canonical(policy))

        policy["policy_digests"]["egress_policy_sha256"] = policy["policy_digests"]["sandbox_policy_sha256"]
        policy_path.write_bytes(canonical(policy))
        try:
            create_plan(root, paths["descriptor"], paths["policy"], "2026-08-28T00:06:00Z")
            fail("duplicate policy digest passed")
        except LaunchPlanError:
            tests.append("duplicate_policy_digest_rejected")

    if len(tests) != 7:
        fail("unexpected qualification launch plan self-test count")
    return {
        "schema": "hepta.servo.worker_qualification_launch_plan_self_test.v1",
        "status": "PASS_LOCAL_FIXTURE_ONLY",
        "tests": tests,
        "test_count": len(tests),
        "real_plan_created": False,
        "launch_authorized": False,
        "worker_executed": False,
        "authority": AUTHORITY,
    }


def parser() -> argparse.ArgumentParser:
    root = argparse.ArgumentParser(description=__doc__)
    sub = root.add_subparsers(dest="command", required=True)
    create = sub.add_parser("create")
    create.add_argument("--root", type=pathlib.Path, required=True)
    create.add_argument("--descriptor", required=True)
    create.add_argument("--policy", required=True)
    create.add_argument("--captured-at")
    create.add_argument("--output", type=pathlib.Path, required=True)
    verify = sub.add_parser("verify")
    verify.add_argument("--root", type=pathlib.Path, required=True)
    verify.add_argument("--plan", type=pathlib.Path, required=True)
    sub.add_parser("contract")
    sub.add_parser("self-test")
    return root


def main(argv: list[str] | None = None) -> int:
    arguments = parser().parse_args(argv)
    try:
        if arguments.command == "create":
            plan = create_plan(arguments.root, arguments.descriptor, arguments.policy, arguments.captured_at)
            write_new(arguments.output, canonical(plan))
            result = {"status": plan["decision"], "plan_id": plan["plan_id"], "launch_authorized": False}
        elif arguments.command == "verify":
            plan = validate_plan(arguments.plan, arguments.root)
            result = {"status": "QUALIFICATION_LAUNCH_PLAN_VERIFIED_NOT_ADMITTED", "plan_id": plan["plan_id"], "launch_authorized": False}
        elif arguments.command == "contract":
            result = contract()
        else:
            result = self_test()
    except (LaunchPlanError, OSError, UnicodeError) as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    sys.exit(main())
